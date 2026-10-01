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
use crate::NBDifferentiate as Differentiate;
use crate::NBDifferentiate::DifferentiationArguments;
use crate::NBDifferentiate::DifferentiationType;
use crate::NBEquation as BEquation;
use crate::NBEquation::Equation;
use crate::NBEquation::EquationAttributes;
use crate::NBEquation::EquationPointers;
use crate::NBEquation::IfEquationBody;
use crate::NBEquation::Iterator;
use crate::NBEquation::WhenEquationBody;
use crate::NBEquation::WhenStatement;
use crate::NBPartition as Partition;
use crate::NBSlice as Slice;
use crate::NBSolve as Solve;
use crate::NBStrongComponent as StrongComponent;
use crate::NBVariable as BVariable;
use crate::NBVariable::VarData;
use crate::NBVariable::VariablePointers;
use openmodelica_ast::Absyn::Path;
use openmodelica_nf_frontend::NFCall as Call;
use openmodelica_nf_frontend::NFComponentRef as ComponentRef;
use openmodelica_nf_frontend::NFDimension as Dimension;
use openmodelica_nf_frontend::NFExpression as Expression;
use openmodelica_nf_frontend::NFFunction::Function;
use openmodelica_nf_frontend::NFOperator as Operator;
use openmodelica_nf_frontend::NFOperator::Op;
use openmodelica_nf_frontend::NFSimplifyExp as SimplifyExp;
use openmodelica_nf_frontend::NFStatement as Statement;
use openmodelica_nf_frontend::NFSubscript as Subscript;
use openmodelica_nf_frontend::NFType as Type;
use openmodelica_nf_frontend::NFVariable as Variable;
use openmodelica_simcode_types::SimCode as OldSimCode;
use openmodelica_util::Error;
use openmodelica_util::ExpandableArray;
use openmodelica_util::Flags;
use openmodelica_util::StringUtil;
use openmodelica_util::System as BuiltinSystem;
use openmodelica_util::UnorderedMap;
use openmodelica_util::UnorderedSet;
use openmodelica_util::Util;
use openmodelica_util::Vector;
use openmodelica_util_datatypes_basic::Array;
use openmodelica_util_datatypes_basic::List;
use openmodelica_util_datatypes_basic::Mutable;
use openmodelica_util_datatypes_basic::Pointer;

// self import
// OF imports
// Old Simcode imports
// NF imports
// NB imports
// Util import
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, metamodelica::ReferenceEq)]
#[repr(i32)]
pub enum MatrixStrictness {
    LINEAR = 1,
    MATCHING = 2,
    SORTING = 3,
    FULL = 4,
}
impl PartialOrd for MatrixStrictness {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for MatrixStrictness {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (*self as i32).cmp(&(*other as i32))
    }
}
impl metamodelica::gc::MMTrace for MatrixStrictness {
    fn mm_accept(&self, _: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        Ok(())
    }
}
impl Default for MatrixStrictness {
    fn default() -> Self {
        Self::LINEAR
    }
}

pub(crate) fn strictnessString(mut s: MatrixStrictness) -> ArcStr {
    let mut r#str: ArcStr;
    r#str = (match s {
        MatrixStrictness::LINEAR { .. } => literal!("linear"),
        MatrixStrictness::MATCHING { .. } => literal!("matching"),
        MatrixStrictness::SORTING => literal!("sorting"),
        MatrixStrictness::FULL { .. } => literal!("full"),
        _ => literal!("unknown"),
    });
    r#str
}

pub mod Mapping {
    use super::*;
    #[derive(Clone, Debug, Eq, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
    pub struct Mapping {
        /// eqn: scal_idx -> arr_idx
        pub eqn_StA: metamodelica::Array<i32>,
        /// var: scal_idx -> arr_idx
        pub var_StA: metamodelica::Array<i32>,
        /// eqn: arr_idx -> start_idx/length
        pub eqn_AtS: metamodelica::Array<(i32, i32)>,
        /// var: arr_idx -> start_idx/length
        pub var_AtS: metamodelica::Array<(i32, i32)>,
    }

    impl metamodelica::gc::MMTrace for Mapping {
        fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
            metamodelica::gc::MMTrace::mm_accept(&self.eqn_StA, __mmv)?;
            metamodelica::gc::MMTrace::mm_accept(&self.var_StA, __mmv)?;
            metamodelica::gc::MMTrace::mm_accept(&self.eqn_AtS, __mmv)?;
            metamodelica::gc::MMTrace::mm_accept(&self.var_AtS, __mmv)?;
            Ok(())
        }
    }
    impl Default for Mapping {
        fn default() -> Self {
            Self {
                eqn_StA: Default::default(),
                var_StA: Default::default(),
                eqn_AtS: Default::default(),
                var_AtS: Default::default(),
            }
        }
    }

    pub type MAPPING = Mapping;

    pub(crate) fn toString(mut mapping: &metamodelica::Ref<Mapping>) -> Result<ArcStr> {
        let mut r#str: ArcStr;
        let mut start: i32;
        let mut size: i32;
        r#str = StringUtil::headline_4(&(literal!("Equation Index Mapping (ARR) -> START | SIZE")))?;
        for mut i in 1..=metamodelica::arrayLength(mapping.eqn_AtS.clone()) {
            (start, size) = ({
                let __elt = (*metamodelica::index_checked(&mapping.eqn_AtS.borrow(), i)?).clone();
                __elt
            });
            r#str = {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*r#str);
                __mm_s.push_str(&*literal!("("));
                __mm_s.push_str(&*intString(i));
                __mm_s.push_str(&*literal!(")\t"));
                __mm_s.push_str(&*intString(start));
                __mm_s.push_str(&*literal!(" | "));
                __mm_s.push_str(&*intString(size));
                __mm_s.push_str(&*literal!("\n"));
                ArcStr::from(__mm_s)
            };
        }
        r#str = {
            let mut __mm_s = String::new();
            __mm_s.push_str(&*r#str);
            __mm_s.push_str(&*StringUtil::headline_4(
                &(literal!("Variable Index Mapping (ARR) -> START | SIZE")),
            )?);
            ArcStr::from(__mm_s)
        };
        for mut i in 1..=metamodelica::arrayLength(mapping.var_AtS.clone()) {
            (start, size) = ({
                let __elt = (*metamodelica::index_checked(&mapping.var_AtS.borrow(), i)?).clone();
                __elt
            });
            r#str = {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*r#str);
                __mm_s.push_str(&*literal!("("));
                __mm_s.push_str(&*intString(i));
                __mm_s.push_str(&*literal!(")\t"));
                __mm_s.push_str(&*intString(start));
                __mm_s.push_str(&*literal!(" | "));
                __mm_s.push_str(&*intString(size));
                __mm_s.push_str(&*literal!("\n"));
                ArcStr::from(__mm_s)
            };
        }
        Ok(r#str)
    }

    pub(crate) fn empty() -> metamodelica::Ref<Mapping> {
        let mut mapping: metamodelica::Ref<Mapping> = metamodelica::Ref::new(Mapping {
            eqn_StA: arrayCreate(0, 0),
            var_StA: arrayCreate(0, 0),
            eqn_AtS: arrayCreate(0, (0, 0)),
            var_AtS: arrayCreate(0, (0, 0)),
        });
        mapping
    }

    pub(crate) fn create(
        mut eqns: &metamodelica::Ref<EquationPointers::EquationPointers>,
        mut vars: &metamodelica::Ref<VariablePointers::VariablePointers>,
    ) -> Result<metamodelica::Ref<Mapping>> {
        let mut mapping: metamodelica::Ref<Mapping>;
        let mut eqn_lst: metamodelica::List<Pointer::Pointer<metamodelica::Ref<Equation::Equation>>> =
            BEquation::EquationPointers::toList(eqns)?;
        let mut var_lst: metamodelica::List<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>> =
            BVariable::VariablePointers::toList(vars)?;
        let mut eqn_StA: metamodelica::Array<i32>;
        let mut var_StA: metamodelica::Array<i32>;
        let mut eqn_AtS: metamodelica::Array<(i32, i32)>;
        let mut var_AtS: metamodelica::Array<(i32, i32)>;
        let mut eqn_scalar_size: i32;
        let mut var_scalar_size: i32;
        let mut eqn_idx_scal: i32 = 1;
        let mut eqn_idx_arr: i32 = 1;
        let mut var_idx_scal: i32 = 1;
        let mut var_idx_arr: i32 = 1;
        eqn_scalar_size = ({
            let mut __acc: i32 = 0;
            for mut eqn in (eqn_lst.clone()).into_iter().cloned() {
                let __x = BEquation::Equation::size(eqn.clone(), true)?;
                __acc += __x;
            }
            __acc
        });
        var_scalar_size = ({
            let mut __acc: i32 = 0;
            for mut var in (var_lst.clone()).into_iter().cloned() {
                let __x = BVariable::size(var.clone(), true)?;
                __acc += __x;
            }
            __acc
        });
        eqn_StA = arrayCreate(eqn_scalar_size, -1);
        var_StA = arrayCreate(var_scalar_size, -1);
        eqn_AtS = arrayCreate(BEquation::EquationPointers::size(eqns), (-1, -1));
        var_AtS = arrayCreate(BVariable::VariablePointers::size(vars), (-1, -1));
        (eqn_StA, var_StA, eqn_AtS, var_AtS) = fill_(
            eqn_StA.clone(),
            var_StA.clone(),
            eqn_AtS.clone(),
            var_AtS.clone(),
            &eqn_lst,
            &var_lst,
            eqn_idx_scal,
            eqn_idx_arr,
            var_idx_scal,
            var_idx_arr,
        )?;
        mapping = metamodelica::Ref::new(Mapping {
            eqn_StA: eqn_StA.clone(),
            var_StA: var_StA.clone(),
            eqn_AtS: eqn_AtS.clone(),
            var_AtS: var_AtS.clone(),
        });
        Ok(mapping)
    }

    pub(crate) fn expand(
        mut mapping: metamodelica::Ref<Mapping>,
        mut eqn_lst: metamodelica::List<Pointer::Pointer<metamodelica::Ref<Equation::Equation>>>,
        mut var_lst: metamodelica::List<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>>,
    ) -> Result<metamodelica::Ref<Mapping>> {
        let mut mapping: metamodelica::Ref<Mapping> = mapping;
        let mut eqn_StA: metamodelica::Array<i32>;
        let mut var_StA: metamodelica::Array<i32>;
        let mut eqn_AtS: metamodelica::Array<(i32, i32)>;
        let mut var_AtS: metamodelica::Array<(i32, i32)>;
        let mut eqn_scalar_size: i32;
        let mut var_scalar_size: i32;
        let mut neqn_scal: i32 = ({
            let mut __acc: i32 = 0;
            for mut eqn in (eqn_lst.clone()).into_iter().cloned() {
                let __x = BEquation::Equation::size(eqn.clone(), true)?;
                __acc += __x;
            }
            __acc
        });
        let mut nvar_scal: i32 = ({
            let mut __acc: i32 = 0;
            for mut var in (var_lst.clone()).into_iter().cloned() {
                let __x = BVariable::size(var.clone(), true)?;
                __acc += __x;
            }
            __acc
        });
        let mut neqn_arr: i32 = ((eqn_lst).len() as i32);
        let mut nvar_arr: i32 = ((var_lst).len() as i32);
        let mut eqn_idx_scal: i32 = metamodelica::arrayLength(mapping.eqn_StA.clone()) + 1;
        let mut eqn_idx_arr: i32 = metamodelica::arrayLength(mapping.eqn_AtS.clone()) + 1;
        let mut var_idx_scal: i32 = metamodelica::arrayLength(mapping.var_StA.clone()) + 1;
        let mut var_idx_arr: i32 = metamodelica::arrayLength(mapping.var_AtS.clone()) + 1;
        eqn_StA = Array::expandToSize(eqn_idx_scal - 1 + neqn_scal, mapping.eqn_StA.clone(), -1)?;
        var_StA = Array::expandToSize(var_idx_scal - 1 + nvar_scal, mapping.var_StA.clone(), -1)?;
        eqn_AtS = Array::expandToSize(eqn_idx_arr - 1 + neqn_arr, mapping.eqn_AtS.clone(), (-1, -1))?;
        var_AtS = Array::expandToSize(var_idx_arr - 1 + nvar_arr, mapping.var_AtS.clone(), (-1, -1))?;
        (eqn_StA, var_StA, eqn_AtS, var_AtS) = fill_(
            eqn_StA.clone(),
            var_StA.clone(),
            eqn_AtS.clone(),
            var_AtS.clone(),
            &eqn_lst,
            &var_lst,
            eqn_idx_scal,
            eqn_idx_arr,
            var_idx_scal,
            var_idx_arr,
        )?;
        mapping = metamodelica::Ref::new(Mapping {
            eqn_StA: eqn_StA.clone(),
            var_StA: var_StA.clone(),
            eqn_AtS: eqn_AtS.clone(),
            var_AtS: var_AtS.clone(),
        });
        Ok(mapping)
    }

    pub(crate) fn getEqnScalIndices(
        mut arr_idx: i32,
        mut mapping: &metamodelica::Ref<Mapping>,
        mut reverse: bool,
    ) -> Result<metamodelica::List<i32>> {
        let mut scal_indices: metamodelica::List<i32>;
        let mut start: i32;
        let mut length: i32;
        (start, length) = ({
            let __elt = (*metamodelica::index_checked(&mapping.eqn_AtS.borrow(), arr_idx)?).clone();
            __elt
        });
        scal_indices = if (reverse) {
            List::intRange2(start + length - 1, start)
        } else {
            List::intRange2(start, start + length - 1)
        };
        Ok(scal_indices)
    }

    pub(crate) fn getVarScalIndices(
        mut arr_idx: i32,
        mut mapping: &metamodelica::Ref<Mapping>,
        mut subs: &metamodelica::List<metamodelica::Ref<Subscript::NFSubscript>>,
        mut dims: metamodelica::List<metamodelica::Ref<Dimension::NFDimension>>,
        mut reverse: bool,
    ) -> Result<metamodelica::List<i32>> {
        fn subscriptedIndices(
            mut start: i32,
            mut length: i32,
            mut slice: metamodelica::List<i32>,
        ) -> Result<metamodelica::List<i32>> {
            let mut scal_indices: metamodelica::List<i32>;
            scal_indices = List::intRange2(start, start + length - 1);
            if !((slice).is_empty()) {
                scal_indices = List::keepPositions(scal_indices, slice, false)?;
            }
            Ok(scal_indices)
        }

        let mut scal_indices: metamodelica::List<i32>;
        let mut start: i32;
        let mut length: i32;
        (start, length) = ({
            let __elt = (*metamodelica::index_checked(&mapping.var_AtS.borrow(), arr_idx)?).clone();
            __elt
        });
        scal_indices = ({
            let mut slice: metamodelica::List<i32> = metamodelica::nil();
            (::match_deref::match_deref! { match subs {
                Deref @ metamodelica::ListNode::Nil => {
                    subscriptedIndices(start, length, metamodelica::nil())?
                },
                _ if (List::all(subs, &move |__a0: metamodelica::Ref<Subscript::NFSubscript>| -> metamodelica::Result<_> { ::std::result::Result::Ok(Subscript::isWhole(&__a0)) })?) => {
                    subscriptedIndices(start, length, metamodelica::nil())?
                },
                Deref @ metamodelica::ListNode::Cons { head: sub, tail: Deref @ metamodelica::ListNode::Nil } => {
                    slice = Subscript::toIndexList(metamodelica::AsArg::as_arg(&sub), length)?;
                    subscriptedIndices(start, length, slice)?
                },
                _ => {
                    let mut subs_lst: metamodelica::List<metamodelica::List<metamodelica::Ref<Subscript::NFSubscript>>>;
                    let mut dim_sizes: metamodelica::List<i32>;
                    let mut values: metamodelica::List<i32>;
                    subs_lst = Subscript::scalarizeList(subs, dims.clone(), true)?;
                    subs_lst = List::combination(&subs_lst);
                    dim_sizes = ({
                let mut __acc: metamodelica::List<i32> = metamodelica::nil();
                for mut dim in (dims).into_iter().cloned() {
                    let __x = Dimension::size(&(dim.clone()), false)?;
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            });
                    for mut sub_lst in &*subs_lst.reverse() {
                        values = ({
                let mut __acc: metamodelica::List<i32> = metamodelica::nil();
                for mut s in (sub_lst.clone()).into_iter().cloned() {
                    let __x = Subscript::toInteger(&(s.clone()))?;
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            });
                        slice = metamodelica::cons(Slice::locationToIndex(dim_sizes.clone(), values, start)?, slice);
                    }
                    slice
                },
                _ => {
                    return Err("fail")
                },
                _ => unreachable!("match_deref! exhaustiveness placeholder"),
            } })
        });
        if reverse {
            scal_indices = scal_indices.reverse();
        }
        Ok(scal_indices)
    }

    fn fill_(
        mut eqn_StA: metamodelica::Array<i32>,
        mut var_StA: metamodelica::Array<i32>,
        mut eqn_AtS: metamodelica::Array<(i32, i32)>,
        mut var_AtS: metamodelica::Array<(i32, i32)>,
        mut eqn_lst: &metamodelica::List<Pointer::Pointer<metamodelica::Ref<Equation::Equation>>>,
        mut var_lst: &metamodelica::List<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>>,
        mut eqn_idx_scal_start: i32,
        mut eqn_idx_arr_start: i32,
        mut var_idx_scal_start: i32,
        mut var_idx_arr_start: i32,
    ) -> Result<(
        metamodelica::Array<i32>,
        metamodelica::Array<i32>,
        metamodelica::Array<(i32, i32)>,
        metamodelica::Array<(i32, i32)>,
    )> {
        let mut eqn_StA: metamodelica::Array<i32> = eqn_StA;
        let mut var_StA: metamodelica::Array<i32> = var_StA;
        let mut eqn_AtS: metamodelica::Array<(i32, i32)> = eqn_AtS;
        let mut var_AtS: metamodelica::Array<(i32, i32)> = var_AtS;
        let mut size: i32;
        let mut eqn_idx_scal: i32 = eqn_idx_scal_start;
        let mut eqn_idx_arr: i32 = eqn_idx_arr_start;
        let mut var_idx_scal: i32 = var_idx_scal_start;
        let mut var_idx_arr: i32 = var_idx_arr_start;
        for mut eqn_ptr in &**eqn_lst {
            size = BEquation::Equation::size(eqn_ptr.clone(), true)?;
            {
                let __cell0 = (eqn_idx_scal, size);
                let __idx0 = eqn_idx_arr;
                *metamodelica::index_mut_checked(&mut eqn_AtS.clone().borrow_mut(), __idx0)? = __cell0;
            }
            for mut i in eqn_idx_scal..=eqn_idx_scal + size - 1 {
                {
                    let __cell1 = eqn_idx_arr;
                    let __idx1 = i;
                    *metamodelica::index_mut_checked(&mut eqn_StA.clone().borrow_mut(), __idx1)? = __cell1;
                }
            }
            eqn_idx_scal = eqn_idx_scal + size;
            eqn_idx_arr = eqn_idx_arr + 1;
        }
        for mut var_ptr in &**var_lst {
            size = BVariable::size(var_ptr.clone(), true)?;
            {
                let __cell2 = (var_idx_scal, size);
                let __idx2 = var_idx_arr;
                *metamodelica::index_mut_checked(&mut var_AtS.clone().borrow_mut(), __idx2)? = __cell2;
            }
            for mut i in var_idx_scal..=var_idx_scal + size - 1 {
                {
                    let __cell3 = var_idx_arr;
                    let __idx3 = i;
                    *metamodelica::index_mut_checked(&mut var_StA.clone().borrow_mut(), __idx3)? = __cell3;
                }
            }
            var_idx_scal = var_idx_scal + size;
            var_idx_arr = var_idx_arr + 1;
        }
        Ok((eqn_StA, var_StA, eqn_AtS, var_AtS))
    }
}

pub mod Mode {
    use super::*;
    /// most of the time this will only have one cref. if there are multiple crefs
    ///      representing the same variable its a multi mode and the equation needs to
    ///      be split when solved for it
    #[derive(Clone, Debug, Eq, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
    pub struct Mode {
        /// the equation name
        pub eqn_name: metamodelica::Ref<ComponentRef::NFComponentRef>,
        /// the cref(s) to solve for
        pub crefs: metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>>,
        /// true if the equation needs to be scalarized to find the cref to solve for
        pub scalarize: bool,
    }

    impl metamodelica::gc::MMTrace for Mode {
        fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
            metamodelica::gc::MMTrace::mm_accept(&self.eqn_name, __mmv)?;
            metamodelica::gc::MMTrace::mm_accept(&self.crefs, __mmv)?;
            metamodelica::gc::MMTrace::mm_accept(&self.scalarize, __mmv)?;
            Ok(())
        }
    }
    impl Default for Mode {
        fn default() -> Self {
            Self {
                eqn_name: Default::default(),
                crefs: Default::default(),
                scalarize: Default::default(),
            }
        }
    }

    pub type MODE = Mode;

    pub(crate) fn toString(mut mode: &metamodelica::Ref<Mode>) -> Result<ArcStr> {
        let mut r#str: ArcStr = {
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("[eqn: "));
            __mm_s.push_str(&*ComponentRef::toString(&mode.eqn_name)?);
            __mm_s.push_str(&*literal!(", crefs: "));
            __mm_s.push_str(&*List::toString(
                mode.crefs.clone(),
                &move |__a0: metamodelica::Ref<ComponentRef::NFComponentRef>| ComponentRef::toString(&__a0),
                List::Style::FLAT_CURLY.clone(),
            )?);
            __mm_s.push_str(&*literal!(", scal: "));
            __mm_s.push_str(&*boolString(mode.scalarize.clone()));
            __mm_s.push_str(&*literal!("]"));
            ArcStr::from(__mm_s)
        };
        Ok(r#str)
    }

    pub(crate) fn hash(mut mode: &metamodelica::Ref<Mode>) -> Result<i32> {
        let mut hash: i32 = ComponentRef::hash(&mode.eqn_name)?;
        Ok(hash)
    }

    pub(crate) fn isEqual(mut mode1: &metamodelica::Ref<Mode>, mut mode2: &metamodelica::Ref<Mode>) -> Result<bool> {
        let mut b: bool = ComponentRef::isEqual(&mode1.eqn_name, &mode2.eqn_name)?
            && mode1.scalarize.clone() == mode2.scalarize.clone()
            && List::isEqualOnTrue(
                mode1.crefs.clone(),
                mode2.crefs.clone(),
                &move |__a0: metamodelica::Ref<ComponentRef::NFComponentRef>,
                       __a1: metamodelica::Ref<ComponentRef::NFComponentRef>| {
                    ComponentRef::isEqual(&__a0, &__a1)
                },
            )?;
        Ok(b)
    }

    pub(crate) fn create(
        mut eqn_name: metamodelica::Ref<ComponentRef::NFComponentRef>,
        mut crefs: metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>>,
        mut scalarize: bool,
    ) -> Result<metamodelica::Ref<Mode>> {
        let mut mode: metamodelica::Ref<Mode> = metamodelica::Ref::new(Mode {
            eqn_name: eqn_name.clone(),
            crefs: ({
                let mut __acc: metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>> =
                    metamodelica::nil();
                for mut cref in (crefs.clone()).into_iter().cloned() {
                    let __x = ComponentRef::simplifySubscripts(cref.clone(), false)?;
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            }),
            scalarize: scalarize,
        });
        Ok(mode)
    }

    pub(crate) fn merge(
        mut mode1: &metamodelica::Ref<Mode>,
        mut mode2: &metamodelica::Ref<Mode>,
    ) -> metamodelica::Ref<Mode> {
        let mut oMode: metamodelica::Ref<Mode> = metamodelica::Ref::new(Mode {
            eqn_name: mode1.eqn_name.clone(),
            crefs: listAppend(mode1.crefs.clone(), mode2.crefs.clone()),
            scalarize: mode1.scalarize.clone() || mode2.scalarize.clone(),
        });
        oMode
    }
}

/// the distinct causalization modes. a matrix entry refers to one by its index
///    in here (0 for none), kept in the matrix payload instead of in a map keyed
///    on the (equation, variable) pair.
pub type ModeTable = metamodelica::Ref<Vector::Vector<metamodelica::Ref<Mode::Mode>>>;

pub mod Modes {
    use super::*;
    pub(crate) fn new() -> metamodelica::Ref<Vector::Vector<metamodelica::Ref<Mode::Mode>>> {
        let mut modes: metamodelica::Ref<Vector::Vector<metamodelica::Ref<Mode::Mode>>> = Vector::new(0);
        modes
    }

    pub(crate) fn add(
        mut modes: metamodelica::Ref<Vector::Vector<metamodelica::Ref<Mode::Mode>>>,
        mut mode: metamodelica::Ref<Mode::Mode>,
    ) -> i32 {
        let mut id: i32;
        Vector::push(modes.clone(), mode);
        id = Vector::size(modes);
        id
    }

    pub(crate) fn get(
        mut modes: metamodelica::Ref<Vector::Vector<metamodelica::Ref<Mode::Mode>>>,
        mut m: &metamodelica::Ref<IntMatrix::IntMatrix>,
        mut data: metamodelica::Array<i32>,
        mut ids: metamodelica::Array<i32>,
        mut eqn: i32,
        mut var: i32,
    ) -> Result<Option<metamodelica::Ref<Mode::Mode>>> {
        let __ab_data = data.borrow();
        let __ab_ids = ids.borrow();
        let mut res: Option<metamodelica::Ref<Mode::Mode>> = None;
        let mut first: i32 = ({
            let __elt = (*metamodelica::index_checked(&m.start.borrow(), eqn)?).clone();
            __elt
        });
        let mut mode: metamodelica::Ref<Mode::Mode>;
        let __range0 = first
            ..=first
                + ({
                    let __elt = (*metamodelica::index_checked(&m.len.borrow(), eqn)?).clone();
                    __elt
                })
                - 1;
        for mut k in __range0 {
            if (*metamodelica::index_checked(&__ab_data, k)?).clone() == var
                && (*metamodelica::index_checked(&__ab_ids, k)?).clone() > 0
            {
                mode = Vector::getNoBounds(modes.clone(), (*metamodelica::index_checked(&__ab_ids, k)?).clone());
                res = (::match_deref::match_deref! { match &(res) {
                    Some(acc) => {
                        Some(Mode::merge(metamodelica::AsArg::as_arg(&acc), &mode))
                    },
                    _ => {
                        Some(mode)
                    },
                    _ => unreachable!("match_deref! exhaustiveness placeholder"),
                } });
            }
        }
        Ok(res)
    }
}

pub mod IntMatrix {
    use super::*;
    /// Adjacency rows stored flat: every row is a slice of one append-only integer
    ///    buffer. Nothing in here is a pointer, so the collector does not have to walk
    ///    the nonzeros. aux carries one extra integer per entry (the causalization
    ///    mode id of the normal matrix), and is empty where it is not used.
    #[derive(Clone, Debug, Eq, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
    pub struct IntMatrix {
        /// row -> 1-based index of its first entry in data
        pub start: metamodelica::Array<i32>,
        /// row -> number of entries
        pub len: metamodelica::Array<i32>,
        /// entry buffer, only appended to
        pub data: metamodelica::Ref<Vector::Vector<i32>>,
        /// payload parallel to data, empty if unused
        pub aux: metamodelica::Ref<Vector::Vector<i32>>,
    }

    impl metamodelica::gc::MMTrace for IntMatrix {
        fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
            metamodelica::gc::MMTrace::mm_accept(&self.start, __mmv)?;
            metamodelica::gc::MMTrace::mm_accept(&self.len, __mmv)?;
            metamodelica::gc::MMTrace::mm_accept(&self.data, __mmv)?;
            metamodelica::gc::MMTrace::mm_accept(&self.aux, __mmv)?;
            Ok(())
        }
    }
    impl Default for IntMatrix {
        fn default() -> Self {
            Self {
                start: Default::default(),
                len: Default::default(),
                data: Default::default(),
                aux: Default::default(),
            }
        }
    }

    pub type INT_MATRIX = IntMatrix;

    /// The matrix is filled by collecting (row, entry) pairs in any order;
    ///      fromBuilder() then groups them by row in one counting sort.
    #[derive(Clone, Debug, Eq, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
    pub struct Builder {
        pub pairs: metamodelica::Ref<Vector::Vector<i32>>,
        pub aux: metamodelica::Ref<Vector::Vector<i32>>,
        pub hasAux: bool,
    }

    impl metamodelica::gc::MMTrace for Builder {
        fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
            metamodelica::gc::MMTrace::mm_accept(&self.pairs, __mmv)?;
            metamodelica::gc::MMTrace::mm_accept(&self.aux, __mmv)?;
            metamodelica::gc::MMTrace::mm_accept(&self.hasAux, __mmv)?;
            Ok(())
        }
    }
    impl Default for Builder {
        fn default() -> Self {
            Self {
                pairs: Default::default(),
                aux: Default::default(),
                hasAux: Default::default(),
            }
        }
    }

    pub type BUILDER = Builder;

    pub(crate) fn new(mut rows: i32) -> metamodelica::Ref<IntMatrix> {
        let mut m: metamodelica::Ref<IntMatrix> = metamodelica::Ref::new(IntMatrix {
            start: arrayCreate(rows, 1),
            len: arrayCreate(rows, 0),
            data: Vector::new(0),
            aux: Vector::new(0),
        });
        m
    }

    pub(crate) fn rows(mut m: &metamodelica::Ref<IntMatrix>) -> i32 {
        let mut n: i32 = metamodelica::arrayLength(m.start.clone());
        n
    }

    pub(crate) fn nonZeroCount(mut m: &metamodelica::Ref<IntMatrix>) -> i32 {
        let mut count: i32 = ({
            let mut __acc: i32 = 0;
            for mut l in (m.len.clone()).borrow().iter() {
                let __x = l.clone();
                __acc += __x;
            }
            __acc
        });
        count
    }

    pub(crate) fn entries(mut m: &metamodelica::Ref<IntMatrix>) -> metamodelica::Array<i32> {
        let mut data: metamodelica::Array<i32> = Vector::rawArray(m.data.clone());
        data
    }

    pub(crate) fn payload(mut m: &metamodelica::Ref<IntMatrix>) -> metamodelica::Array<i32> {
        let mut aux: metamodelica::Array<i32> = Vector::rawArray(m.aux.clone());
        aux
    }

    pub(crate) fn toList(mut m: &metamodelica::Ref<IntMatrix>, mut row: i32) -> Result<metamodelica::List<i32>> {
        let mut lst: metamodelica::List<i32> = metamodelica::nil();
        let mut data: metamodelica::Array<i32> = Vector::rawArray(m.data.clone());
        let mut first: i32 = ({
            let __elt = (*metamodelica::index_checked(&m.start.borrow(), row)?).clone();
            __elt
        });
        let __range0 = ({
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
        for mut k in __range0 {
            lst = metamodelica::cons(
                ({
                    let __elt = (*metamodelica::index_checked(&data.borrow(), k)?).clone();
                    __elt
                }),
                lst,
            );
        }
        Ok(lst)
    }

    pub(crate) fn setRow(
        mut m: &metamodelica::Ref<IntMatrix>,
        mut row: i32,
        mut values: &metamodelica::List<i32>,
    ) -> Result<()> {
        metamodelica::arrayUpdate(m.start.clone(), row, Vector::size(m.data.clone()) + 1)?;
        metamodelica::arrayUpdate(m.len.clone(), row, ((values).len() as i32))?;
        for mut v in &**values {
            Vector::push(m.data.clone(), v.clone());
        }
        Ok(())
    }

    pub(crate) fn setRowFromArray(
        mut m: &metamodelica::Ref<IntMatrix>,
        mut row: i32,
        mut values: metamodelica::Array<i32>,
        mut n: i32,
    ) -> Result<()> {
        let __ab_values = values.borrow();
        metamodelica::arrayUpdate(m.start.clone(), row, Vector::size(m.data.clone()) + 1)?;
        metamodelica::arrayUpdate(m.len.clone(), row, n)?;
        for mut i in 1..=n {
            Vector::push(m.data.clone(), (*metamodelica::index_checked(&__ab_values, i)?).clone());
        }
        Ok(())
    }

    pub(crate) fn reserveData(mut m: &metamodelica::Ref<IntMatrix>, mut extra: i32, mut withAux: bool) -> () {
        Vector::reserve(m.data.clone(), Vector::size(m.data.clone()) + extra);
        if withAux {
            Vector::reserve(m.aux.clone(), Vector::size(m.aux.clone()) + extra);
        }
        ()
    }

    pub(crate) fn clearRow(mut m: &metamodelica::Ref<IntMatrix>, mut row: i32) -> Result<()> {
        metamodelica::arrayUpdate(m.len.clone(), row, 0)?;
        Ok(())
    }

    pub(crate) fn copyRow(
        mut from: &metamodelica::Ref<IntMatrix>,
        mut src: i32,
        mut from_data: metamodelica::Array<i32>,
        mut from_aux: metamodelica::Array<i32>,
        mut m: &metamodelica::Ref<IntMatrix>,
        mut dst: i32,
    ) -> Result<()> {
        let __ab_from_data = from_data.borrow();
        let mut first: i32 = ({
            let __elt = (*metamodelica::index_checked(&from.start.borrow(), src)?).clone();
            __elt
        });
        let mut n: i32 = ({
            let __elt = (*metamodelica::index_checked(&from.len.borrow(), src)?).clone();
            __elt
        });
        let mut aux: bool = metamodelica::arrayLength(from_aux.clone()) > 0;
        metamodelica::arrayUpdate(m.start.clone(), dst, Vector::size(m.data.clone()) + 1)?;
        metamodelica::arrayUpdate(m.len.clone(), dst, n)?;
        for mut k in first..=first + n - 1 {
            Vector::push(
                m.data.clone(),
                (*metamodelica::index_checked(&__ab_from_data, k)?).clone(),
            );
            if aux {
                Vector::push(
                    m.aux.clone(),
                    ({
                        let __elt = (*metamodelica::index_checked(&from_aux.borrow(), k)?).clone();
                        __elt
                    }),
                );
            }
        }
        Ok(())
    }

    pub(crate) fn expandRows(
        mut m: metamodelica::Ref<IntMatrix>,
        mut shift: i32,
    ) -> Result<metamodelica::Ref<IntMatrix>> {
        let mut m: metamodelica::Ref<IntMatrix> = m;
        if shift > 0 {
            m = metamodelica::Ref::new(IntMatrix {
                start: Array::expandToSize(metamodelica::arrayLength(m.start.clone()) + shift, m.start.clone(), 1)?,
                len: Array::expandToSize(metamodelica::arrayLength(m.len.clone()) + shift, m.len.clone(), 0)?,
                data: m.data.clone(),
                aux: m.aux.clone(),
            });
        }
        Ok(m)
    }

    pub(crate) fn transpose(
        mut m: &metamodelica::Ref<IntMatrix>,
        mut size: i32,
        mut slack: i32,
    ) -> Result<metamodelica::Ref<IntMatrix>> {
        let mut mT: metamodelica::Ref<IntMatrix>;
        let mut data: metamodelica::Array<i32> = Vector::rawArray(m.data.clone());
        let mut start: metamodelica::Array<i32>;
        let mut len: metamodelica::Array<i32>;
        let mut out: metamodelica::Array<i32>;
        let mut n: i32 = metamodelica::arrayLength(m.start.clone());
        let mut nnz: i32 = 0;
        let mut idx: i32;
        let mut pos: i32;
        let mut first: i32;
        let mut negated: bool = false;
        start = arrayCreate(size, 1);
        len = arrayCreate(size, 0);
        for mut r in 1..=n {
            first = ({
                let __elt = (*metamodelica::index_checked(&m.start.borrow(), r)?).clone();
                __elt
            });
            let __range0 = first
                ..=first
                    + ({
                        let __elt = (*metamodelica::index_checked(&m.len.borrow(), r)?).clone();
                        __elt
                    })
                    - 1;
            for mut k in __range0 {
                idx = intAbs(
                    ({
                        let __elt = (*metamodelica::index_checked(&data.borrow(), k)?).clone();
                        __elt
                    }),
                );
                if idx > 0 && idx <= size {
                    metamodelica::arrayUpdate(
                        len.clone(),
                        idx,
                        ({
                            let __elt = (*metamodelica::index_checked(&len.borrow(), idx)?).clone();
                            __elt
                        }) + 1,
                    )?;
                    nnz = nnz + 1;
                    negated = negated
                        || ({
                            let __elt = (*metamodelica::index_checked(&data.borrow(), k)?).clone();
                            __elt
                        }) < 0;
                } else {
                    Error::addMessage(
                        Error::INTERNAL_ERROR.clone(),
                        list![{
                            let mut __mm_s = String::new();
                            __mm_s.push_str(&*literal!("NBAdjacency.IntMatrix.transpose"));
                            __mm_s.push_str(&*literal!(" failed for variable index "));
                            __mm_s.push_str(&*intString(
                                ({
                                    let __elt = (*metamodelica::index_checked(&data.borrow(), k)?).clone();
                                    __elt
                                }),
                            ));
                            __mm_s.push_str(&*literal!(".\n              The variables have to be dense (without empty spaces) for this to work!"));
                            ArcStr::from(__mm_s)
                        }],
                    )?;
                }
            }
        }
        pos = 1;
        for mut c in 1..=size {
            metamodelica::arrayUpdate(start.clone(), c, pos)?;
            pos = pos
                + ({
                    let __elt = (*metamodelica::index_checked(&len.borrow(), c)?).clone();
                    __elt
                });
        }
        out = arrayCreate(nnz + slack, 0);
        for mut r in ({
            let __s = n;
            let __e = 1;
            (0i32..)
                .map(move |__k| __s + __k * (-1))
                .take_while(move |&__v| __v >= __e)
        }) {
            first = ({
                let __elt = (*metamodelica::index_checked(&m.start.borrow(), r)?).clone();
                __elt
            });
            let __range1 = first
                ..=first
                    + ({
                        let __elt = (*metamodelica::index_checked(&m.len.borrow(), r)?).clone();
                        __elt
                    })
                    - 1;
            for mut k in __range1 {
                idx = ({
                    let __elt = (*metamodelica::index_checked(&data.borrow(), k)?).clone();
                    __elt
                });
                if idx > 0 && idx <= size {
                    metamodelica::arrayUpdate(
                        out.clone(),
                        ({
                            let __elt = (*metamodelica::index_checked(&start.borrow(), idx)?).clone();
                            __elt
                        }),
                        r,
                    )?;
                    metamodelica::arrayUpdate(
                        start.clone(),
                        idx,
                        ({
                            let __elt = (*metamodelica::index_checked(&start.borrow(), idx)?).clone();
                            __elt
                        }) + 1,
                    )?;
                }
            }
        }
        if negated {
            for mut r in 1..=n {
                first = ({
                    let __elt = (*metamodelica::index_checked(&m.start.borrow(), r)?).clone();
                    __elt
                });
                let __range2 = first
                    ..=first
                        + ({
                            let __elt = (*metamodelica::index_checked(&m.len.borrow(), r)?).clone();
                            __elt
                        })
                        - 1;
                for mut k in __range2 {
                    idx = ({
                        let __elt = (*metamodelica::index_checked(&data.borrow(), k)?).clone();
                        __elt
                    });
                    if idx < 0 && -(idx) <= size {
                        metamodelica::arrayUpdate(
                            out.clone(),
                            ({
                                let __elt = (*metamodelica::index_checked(&start.borrow(), -(idx))?).clone();
                                __elt
                            }),
                            -(r),
                        )?;
                        metamodelica::arrayUpdate(
                            start.clone(),
                            -(idx),
                            ({
                                let __elt = (*metamodelica::index_checked(&start.borrow(), -(idx))?).clone();
                                __elt
                            }) + 1,
                        )?;
                    }
                }
            }
        }
        for mut c in 1..=size {
            metamodelica::arrayUpdate(
                start.clone(),
                c,
                ({
                    let __elt = (*metamodelica::index_checked(&start.borrow(), c)?).clone();
                    __elt
                }) - ({
                    let __elt = (*metamodelica::index_checked(&len.borrow(), c)?).clone();
                    __elt
                }),
            )?;
        }
        mT = metamodelica::Ref::new(IntMatrix {
            start: start.clone(),
            len: len.clone(),
            data: Vector::fromArrayNoCopy(out.clone(), nnz),
            aux: Vector::new(0),
        });
        Ok(mT)
    }

    pub(crate) fn newBuilder(mut capacity: i32, mut withAux: bool) -> Builder {
        let mut b: Builder = Builder {
            pairs: Vector::new(2 * capacity),
            aux: Vector::new(if (withAux) { capacity } else { 0 }),
            hasAux: withAux,
        };
        b
    }

    pub(crate) fn builderAdd(mut b: &Builder, mut row: i32, mut value: i32) -> () {
        Vector::push(b.pairs.clone(), row);
        Vector::push(b.pairs.clone(), value);
        ()
    }

    pub(crate) fn builderAddAux(mut b: &Builder, mut row: i32, mut value: i32, mut aux: i32) -> () {
        Vector::push(b.pairs.clone(), row);
        Vector::push(b.pairs.clone(), value);
        Vector::push(b.aux.clone(), aux);
        ()
    }

    pub(crate) fn builderAddList(
        mut b: &Builder,
        mut row: i32,
        mut values: metamodelica::List<i32>,
        mut aux: i32,
    ) -> () {
        for mut v in &*values.reverse() {
            Vector::push(b.pairs.clone(), row);
            Vector::push(b.pairs.clone(), v.clone());
            if b.hasAux.clone() {
                Vector::push(b.aux.clone(), aux);
            }
        }
        ()
    }

    pub(crate) fn toBuilder(
        mut m: &metamodelica::Ref<IntMatrix>,
        mut capacity: i32,
        mut withAux: bool,
    ) -> Result<Builder> {
        let mut b: Builder;
        let mut data: metamodelica::Array<i32> = Vector::rawArray(m.data.clone());
        let mut aux: metamodelica::Array<i32> = Vector::rawArray(m.aux.clone());
        let mut nnz: i32 = nonZeroCount(m);
        let mut first: i32;
        b = newBuilder(intMax(nnz, capacity), withAux);
        for mut r in 1..=metamodelica::arrayLength(m.start.clone()) {
            first = ({
                let __elt = (*metamodelica::index_checked(&m.start.borrow(), r)?).clone();
                __elt
            });
            let __range0 = ({
                let __s = first
                    + ({
                        let __elt = (*metamodelica::index_checked(&m.len.borrow(), r)?).clone();
                        __elt
                    })
                    - 1;
                let __e = first;
                (0i32..)
                    .map(move |__k| __s + __k * (-1))
                    .take_while(move |&__v| __v >= __e)
            });
            for mut k in __range0 {
                Vector::push(b.pairs.clone(), r);
                Vector::push(
                    b.pairs.clone(),
                    ({
                        let __elt = (*metamodelica::index_checked(&data.borrow(), k)?).clone();
                        __elt
                    }),
                );
                if withAux {
                    Vector::push(
                        b.aux.clone(),
                        ({
                            let __elt = (*metamodelica::index_checked(&aux.borrow(), k)?).clone();
                            __elt
                        }),
                    );
                }
            }
        }
        Ok(b)
    }

    pub(crate) fn fromBuilder(mut b: &Builder, mut rows: i32) -> Result<metamodelica::Ref<IntMatrix>> {
        let mut m: metamodelica::Ref<IntMatrix>;
        let mut raw: metamodelica::Array<i32> = Vector::rawArray(b.pairs.clone());
        let mut raux: metamodelica::Array<i32> = Vector::rawArray(b.aux.clone());
        let mut start: metamodelica::Array<i32>;
        let mut len: metamodelica::Array<i32>;
        let mut out: metamodelica::Array<i32>;
        let mut aout: metamodelica::Array<i32>;
        let mut n: i32 = intDiv(Vector::size(b.pairs.clone()), 2);
        let mut r: i32;
        let mut pos: i32 = 1;
        let mut p: i32;
        start = arrayCreate(rows, 1);
        len = arrayCreate(rows, 0);
        for mut i in 1..=n {
            r = ({
                let __elt = (*metamodelica::index_checked(&raw.borrow(), 2 * i - 1)?).clone();
                __elt
            });
            metamodelica::arrayUpdate(
                len.clone(),
                r,
                ({
                    let __elt = (*metamodelica::index_checked(&len.borrow(), r)?).clone();
                    __elt
                }) + 1,
            )?;
        }
        for mut i in 1..=rows {
            metamodelica::arrayUpdate(start.clone(), i, pos)?;
            pos = pos
                + ({
                    let __elt = (*metamodelica::index_checked(&len.borrow(), i)?).clone();
                    __elt
                });
        }
        out = arrayCreate(n, 0);
        aout = arrayCreate(if (b.hasAux.clone()) { n } else { 0 }, 0);
        for mut i in ({
            let __s = n;
            let __e = 1;
            (0i32..)
                .map(move |__k| __s + __k * (-1))
                .take_while(move |&__v| __v >= __e)
        }) {
            r = ({
                let __elt = (*metamodelica::index_checked(&raw.borrow(), 2 * i - 1)?).clone();
                __elt
            });
            p = ({
                let __elt = (*metamodelica::index_checked(&start.borrow(), r)?).clone();
                __elt
            });
            metamodelica::arrayUpdate(
                out.clone(),
                p,
                ({
                    let __elt = (*metamodelica::index_checked(&raw.borrow(), 2 * i)?).clone();
                    __elt
                }),
            )?;
            if b.hasAux.clone() {
                metamodelica::arrayUpdate(
                    aout.clone(),
                    p,
                    ({
                        let __elt = (*metamodelica::index_checked(&raux.borrow(), i)?).clone();
                        __elt
                    }),
                )?;
            }
            metamodelica::arrayUpdate(start.clone(), r, p + 1)?;
        }
        for mut i in 1..=rows {
            metamodelica::arrayUpdate(
                start.clone(),
                i,
                ({
                    let __elt = (*metamodelica::index_checked(&start.borrow(), i)?).clone();
                    __elt
                }) - ({
                    let __elt = (*metamodelica::index_checked(&len.borrow(), i)?).clone();
                    __elt
                }),
            )?;
        }
        m = metamodelica::Ref::new(IntMatrix {
            start: start.clone(),
            len: len.clone(),
            data: Vector::fromArrayNoCopy(out.clone(), n),
            aux: Vector::fromArrayNoCopy(aout.clone(), metamodelica::arrayLength(aout.clone())),
        });
        Ok(m)
    }

    pub(crate) fn toString(mut m: &metamodelica::Ref<IntMatrix>) -> Result<ArcStr> {
        let mut r#str: ArcStr = literal!("");
        let mut skip: i32 = ((intString(metamodelica::arrayLength(m.start.clone()))).len() as i32) + 1;
        let mut tmp: ArcStr;
        for mut row in 1..=metamodelica::arrayLength(m.start.clone()) {
            tmp = intString(row);
            r#str = {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*r#str);
                __mm_s.push_str(&*literal!("\t("));
                __mm_s.push_str(&*tmp);
                __mm_s.push_str(&*literal!(")"));
                __mm_s.push_str(&*StringUtil::repeat(literal!(" "), skip - ((tmp).len() as i32))?);
                __mm_s.push_str(&*List::toString(
                    toList(m, row)?,
                    &fnptr!(intString, i32),
                    List::Style::FLAT_CURLY.clone(),
                )?);
                __mm_s.push_str(&*literal!("\n"));
                ArcStr::from(__mm_s)
            };
        }
        Ok(r#str)
    }
}

pub mod Matrix {
    use super::*;
    /// used to store adjacency information for the bipartite graph representing the system of equations and variables
    ///    you have to create it in this specific order: EMPTY->FULL->FINAL(LINEAR)->FINAL->(MATCHING)->FINAL(SORTING)
    ///    and store the FULL for further use.
    #[derive(Clone, Debug, Eq, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
    pub enum Matrix {
        /// placeholder for empty matrices, just stores intended strictness
        EMPTY { st: MatrixStrictness },
        /// contains all information needed. create specific final matrices from this
        FULL {
            equation_names: metamodelica::Array<metamodelica::Ref<ComponentRef::NFComponentRef>>,
            occurrences: metamodelica::Array<
                metamodelica::Ref<UnorderedSet::UnorderedSet<metamodelica::Ref<ComponentRef::NFComponentRef>>>,
            >,
            dependencies: metamodelica::Array<
                metamodelica::Ref<
                    UnorderedMap::UnorderedMap<
                        metamodelica::Ref<ComponentRef::NFComponentRef>,
                        metamodelica::Ref<Dependency::Dependency>,
                    >,
                >,
            >,
            solvabilities: metamodelica::Array<
                metamodelica::Ref<
                    UnorderedMap::UnorderedMap<
                        metamodelica::Ref<ComponentRef::NFComponentRef>,
                        metamodelica::Ref<Solvability::Solvability>,
                    >,
                >,
            >,
            repetitions: metamodelica::Array<
                metamodelica::Ref<UnorderedSet::UnorderedSet<metamodelica::Ref<ComponentRef::NFComponentRef>>>,
            >,
            mapping: metamodelica::Ref<Mapping::Mapping>,
        },
        /// specific final matrix, defined by its strictness
        FINAL {
            /// eqn -> vars
            m: metamodelica::Ref<IntMatrix::IntMatrix>,
            /// var -> eqns
            mT: metamodelica::Ref<IntMatrix::IntMatrix>,
            /// index mapping scalar <-> array
            mapping: metamodelica::Ref<Mapping::Mapping>,
            /// array reconstruction information
            modes: metamodelica::Ref<Vector::Vector<metamodelica::Ref<Mode::Mode>>>,
            /// strictness with which it was created
            st: MatrixStrictness,
        },
        /// contains sparsity information to create sparsity patterns for jacobians
        SPARSITY {
            equation_names: metamodelica::Array<metamodelica::Ref<ComponentRef::NFComponentRef>>,
            equation_iterators: metamodelica::Array<metamodelica::Ref<Iterator::Iterator>>,
            dependencies: metamodelica::Array<
                metamodelica::Ref<
                    UnorderedMap::UnorderedMap<
                        metamodelica::Ref<ComponentRef::NFComponentRef>,
                        metamodelica::Ref<Dependency::Dependency>,
                    >,
                >,
            >,
            repetitions: metamodelica::Array<
                metamodelica::Ref<UnorderedSet::UnorderedSet<metamodelica::Ref<ComponentRef::NFComponentRef>>>,
            >,
            solved_crefs: metamodelica::Array<metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>>>,
        },
    }
    impl metamodelica::gc::MMTrace for Matrix {
        fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
            match self {
                Matrix::EMPTY { st } => {
                    metamodelica::gc::MMTrace::mm_accept(st, __mmv)?;
                    Ok(())
                }
                Matrix::FULL {
                    equation_names,
                    occurrences,
                    dependencies,
                    solvabilities,
                    repetitions,
                    mapping,
                } => {
                    metamodelica::gc::MMTrace::mm_accept(equation_names, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(occurrences, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(dependencies, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(solvabilities, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(repetitions, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(mapping, __mmv)?;
                    Ok(())
                }
                Matrix::FINAL {
                    m,
                    mT,
                    mapping,
                    modes,
                    st,
                } => {
                    metamodelica::gc::MMTrace::mm_accept(m, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(mT, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(mapping, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(modes, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(st, __mmv)?;
                    Ok(())
                }
                Matrix::SPARSITY {
                    equation_names,
                    equation_iterators,
                    dependencies,
                    repetitions,
                    solved_crefs,
                } => {
                    metamodelica::gc::MMTrace::mm_accept(equation_names, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(equation_iterators, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(dependencies, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(repetitions, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(solved_crefs, __mmv)?;
                    Ok(())
                }
            }
        }
    }
    impl Default for Matrix {
        fn default() -> Self {
            Self::EMPTY { st: Default::default() }
        }
    }
    pub use self::Matrix::{EMPTY, FINAL, FULL, SPARSITY};
    pub(crate) fn createFull(
        mut vars: &metamodelica::Ref<VariablePointers::VariablePointers>,
        mut eqns: &metamodelica::Ref<EquationPointers::EquationPointers>,
        mut kind: Partition::Kind,
    ) -> Result<metamodelica::Ref<Matrix>> {
        let mut adj: metamodelica::Ref<Matrix>;
        let mut index: i32;
        let mut size: i32 = BEquation::EquationPointers::size(eqns);
        let mut equation_names: metamodelica::Array<metamodelica::Ref<ComponentRef::NFComponentRef>>;
        let mut occurrences: metamodelica::Array<
            metamodelica::Ref<UnorderedSet::UnorderedSet<metamodelica::Ref<ComponentRef::NFComponentRef>>>,
        >;
        let mut dependencies: metamodelica::Array<
            metamodelica::Ref<
                UnorderedMap::UnorderedMap<
                    metamodelica::Ref<ComponentRef::NFComponentRef>,
                    metamodelica::Ref<Dependency::Dependency>,
                >,
            >,
        >;
        let mut solvabilities: metamodelica::Array<
            metamodelica::Ref<
                UnorderedMap::UnorderedMap<
                    metamodelica::Ref<ComponentRef::NFComponentRef>,
                    metamodelica::Ref<Solvability::Solvability>,
                >,
            >,
        >;
        let mut repetitions: metamodelica::Array<
            metamodelica::Ref<UnorderedSet::UnorderedSet<metamodelica::Ref<ComponentRef::NFComponentRef>>>,
        >;
        let mut occ_set: metamodelica::Ref<
            UnorderedSet::UnorderedSet<metamodelica::Ref<ComponentRef::NFComponentRef>>,
        >;
        let mut rep_set: metamodelica::Ref<
            UnorderedSet::UnorderedSet<metamodelica::Ref<ComponentRef::NFComponentRef>>,
        >;
        let mut dep_map: metamodelica::Ref<
            UnorderedMap::UnorderedMap<
                metamodelica::Ref<ComponentRef::NFComponentRef>,
                metamodelica::Ref<Dependency::Dependency>,
            >,
        >;
        let mut sol_map: metamodelica::Ref<
            UnorderedMap::UnorderedMap<
                metamodelica::Ref<ComponentRef::NFComponentRef>,
                metamodelica::Ref<Solvability::Solvability>,
            >,
        >;
        let mut mapping: metamodelica::Ref<Mapping::Mapping>;
        if ExpandableArray::getNumberOfElements(vars.varArr.clone()) > 0
            || ExpandableArray::getNumberOfElements(eqns.eqArr.clone()) > 0
        {
            equation_names = arrayCreate(size, openmodelica_nf_frontend::NFComponentRef::interned_EMPTY());
            occurrences = arrayCreate(
                size,
                UnorderedSet::new(
                    (std::sync::Arc::new(move |__a0: metamodelica::Ref<ComponentRef::NFComponentRef>| {
                        ComponentRef::hash(&__a0)
                    })
                        as std::sync::Arc<
                            dyn ::std::ops::Fn(metamodelica::Ref<ComponentRef::NFComponentRef>) -> Result<i32>
                                + 'static,
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
                ),
            );
            dependencies = arrayCreate(
                size,
                UnorderedMap::new(
                    (std::sync::Arc::new(move |__a0: metamodelica::Ref<ComponentRef::NFComponentRef>| {
                        ComponentRef::hash(&__a0)
                    })
                        as std::sync::Arc<
                            dyn ::std::ops::Fn(metamodelica::Ref<ComponentRef::NFComponentRef>) -> Result<i32>
                                + 'static,
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
            );
            solvabilities = arrayCreate(
                size,
                UnorderedMap::new(
                    (std::sync::Arc::new(move |__a0: metamodelica::Ref<ComponentRef::NFComponentRef>| {
                        ComponentRef::hash(&__a0)
                    })
                        as std::sync::Arc<
                            dyn ::std::ops::Fn(metamodelica::Ref<ComponentRef::NFComponentRef>) -> Result<i32>
                                + 'static,
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
            );
            repetitions = arrayCreate(
                size,
                UnorderedSet::new(
                    (std::sync::Arc::new(move |__a0: metamodelica::Ref<ComponentRef::NFComponentRef>| {
                        ComponentRef::hash(&__a0)
                    })
                        as std::sync::Arc<
                            dyn ::std::ops::Fn(metamodelica::Ref<ComponentRef::NFComponentRef>) -> Result<i32>
                                + 'static,
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
                ),
            );
            for mut eqn_ptr in &*BEquation::EquationPointers::toList(eqns)? {
                index = UnorderedMap::getSafe(
                    BEquation::Equation::getEqnName(eqn_ptr.clone())?,
                    eqns.map.clone(),
                    metamodelica::sourceInfo!("NBackEnd/Util/NBAdjacency.mo"),
                )?;
                dep_map = UnorderedMap::new(
                    (std::sync::Arc::new(move |__a0: metamodelica::Ref<ComponentRef::NFComponentRef>| {
                        ComponentRef::hash(&__a0)
                    })
                        as std::sync::Arc<
                            dyn ::std::ops::Fn(metamodelica::Ref<ComponentRef::NFComponentRef>) -> Result<i32>
                                + 'static,
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
                sol_map = UnorderedMap::new(
                    (std::sync::Arc::new(move |__a0: metamodelica::Ref<ComponentRef::NFComponentRef>| {
                        ComponentRef::hash(&__a0)
                    })
                        as std::sync::Arc<
                            dyn ::std::ops::Fn(metamodelica::Ref<ComponentRef::NFComponentRef>) -> Result<i32>
                                + 'static,
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
                rep_set = UnorderedSet::new(
                    (std::sync::Arc::new(move |__a0: metamodelica::Ref<ComponentRef::NFComponentRef>| {
                        ComponentRef::hash(&__a0)
                    })
                        as std::sync::Arc<
                            dyn ::std::ops::Fn(metamodelica::Ref<ComponentRef::NFComponentRef>) -> Result<i32>
                                + 'static,
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
                occ_set = collectDependenciesEquation(
                    &(Pointer::access(eqn_ptr.clone())),
                    kind,
                    vars.map.clone(),
                    dep_map.clone(),
                    sol_map.clone(),
                    rep_set.clone(),
                )?;
                addInitialStartOccurrences(occ_set.clone(), dep_map.clone(), sol_map.clone(), rep_set.clone(), kind)?;
                {
                    let __cell0 = BEquation::Equation::getEqnName(eqn_ptr.clone())?;
                    let __idx0 = index;
                    *metamodelica::index_mut_checked(&mut equation_names.clone().borrow_mut(), __idx0)? = __cell0;
                }
                {
                    let __cell1 = occ_set;
                    let __idx1 = index;
                    *metamodelica::index_mut_checked(&mut occurrences.clone().borrow_mut(), __idx1)? = __cell1;
                }
                {
                    let __cell2 = dep_map;
                    let __idx2 = index;
                    *metamodelica::index_mut_checked(&mut dependencies.clone().borrow_mut(), __idx2)? = __cell2;
                }
                {
                    let __cell3 = sol_map;
                    let __idx3 = index;
                    *metamodelica::index_mut_checked(&mut solvabilities.clone().borrow_mut(), __idx3)? = __cell3;
                }
                {
                    let __cell4 = rep_set;
                    let __idx4 = index;
                    *metamodelica::index_mut_checked(&mut repetitions.clone().borrow_mut(), __idx4)? = __cell4;
                }
            }
            mapping = Mapping::create(eqns, vars)?;
            adj = metamodelica::Ref::new(Matrix::FULL {
                equation_names: equation_names.clone(),
                occurrences: occurrences.clone(),
                dependencies: dependencies.clone(),
                solvabilities: solvabilities.clone(),
                repetitions: repetitions.clone(),
                mapping: mapping,
            });
        } else {
            adj = metamodelica::Ref::new(Matrix::EMPTY {
                st: MatrixStrictness::FULL.clone(),
            });
        }
        if Flags::isSet(Flags::BLT_MATRIX_DUMP.clone())? {
            metamodelica::print({
                let mut __mm_s = String::new();
                __mm_s.push_str(&*StringUtil::headline_1(&(literal!("Creating Adjacency Matrices")))?);
                __mm_s.push_str(&*literal!("\n"));
                ArcStr::from(__mm_s)
            });
            metamodelica::print({
                let mut __mm_s = String::new();
                __mm_s.push_str(&*BEquation::EquationPointers::toString(
                    eqns,
                    literal!(""),
                    None,
                    true,
                    None,
                )?);
                __mm_s.push_str(&*literal!("\n"));
                ArcStr::from(__mm_s)
            });
            metamodelica::print({
                let mut __mm_s = String::new();
                __mm_s.push_str(&*BVariable::VariablePointers::toString(vars, literal!(""), None, true)?);
                __mm_s.push_str(&*literal!("\n"));
                ArcStr::from(__mm_s)
            });
            metamodelica::print({
                let mut __mm_s = String::new();
                __mm_s.push_str(&*toString(&adj, literal!("Full"))?);
                __mm_s.push_str(&*literal!("\n"));
                ArcStr::from(__mm_s)
            });
            metamodelica::print({
                let mut __mm_s = String::new();
                __mm_s.push_str(&*solvabilityString(&adj, literal!("Full"))?);
                __mm_s.push_str(&*literal!("\n"));
                ArcStr::from(__mm_s)
            });
            metamodelica::print({
                let mut __mm_s = String::new();
                __mm_s.push_str(&*dependencyString(&adj, literal!("Full"))?);
                __mm_s.push_str(&*literal!("\n"));
                ArcStr::from(__mm_s)
            });
        }
        Ok(adj)
    }

    pub(crate) fn subFull(
        mut full: &metamodelica::Ref<Matrix>,
        mut eqn_indices: &metamodelica::List<i32>,
        mut eqns: &metamodelica::Ref<EquationPointers::EquationPointers>,
        mut vars: &metamodelica::Ref<VariablePointers::VariablePointers>,
    ) -> Result<metamodelica::Ref<Matrix>> {
        let mut sub: metamodelica::Ref<Matrix>;
        let mut size: i32 = ((eqn_indices).len() as i32);
        let mut j: i32 = 1;
        let mut equation_names: metamodelica::Array<metamodelica::Ref<ComponentRef::NFComponentRef>>;
        let mut occurrences: metamodelica::Array<
            metamodelica::Ref<UnorderedSet::UnorderedSet<metamodelica::Ref<ComponentRef::NFComponentRef>>>,
        >;
        let mut repetitions: metamodelica::Array<
            metamodelica::Ref<UnorderedSet::UnorderedSet<metamodelica::Ref<ComponentRef::NFComponentRef>>>,
        >;
        let mut dependencies: metamodelica::Array<
            metamodelica::Ref<
                UnorderedMap::UnorderedMap<
                    metamodelica::Ref<ComponentRef::NFComponentRef>,
                    metamodelica::Ref<Dependency::Dependency>,
                >,
            >,
        >;
        let mut solvabilities: metamodelica::Array<
            metamodelica::Ref<
                UnorderedMap::UnorderedMap<
                    metamodelica::Ref<ComponentRef::NFComponentRef>,
                    metamodelica::Ref<Solvability::Solvability>,
                >,
            >,
        >;
        sub = (match &**full {
            FULL { .. } if (size > 0) => {
                equation_names = arrayCreate(size, openmodelica_nf_frontend::NFComponentRef::interned_EMPTY());
                occurrences = arrayCreate(
                    size,
                    UnorderedSet::new(
                        (std::sync::Arc::new(move |__a0: metamodelica::Ref<ComponentRef::NFComponentRef>| {
                            ComponentRef::hash(&__a0)
                        })
                            as std::sync::Arc<
                                dyn ::std::ops::Fn(metamodelica::Ref<ComponentRef::NFComponentRef>) -> Result<i32>
                                    + 'static,
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
                    ),
                );
                dependencies = arrayCreate(
                    size,
                    UnorderedMap::new(
                        (std::sync::Arc::new(move |__a0: metamodelica::Ref<ComponentRef::NFComponentRef>| {
                            ComponentRef::hash(&__a0)
                        })
                            as std::sync::Arc<
                                dyn ::std::ops::Fn(metamodelica::Ref<ComponentRef::NFComponentRef>) -> Result<i32>
                                    + 'static,
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
                );
                solvabilities = arrayCreate(
                    size,
                    UnorderedMap::new(
                        (std::sync::Arc::new(move |__a0: metamodelica::Ref<ComponentRef::NFComponentRef>| {
                            ComponentRef::hash(&__a0)
                        })
                            as std::sync::Arc<
                                dyn ::std::ops::Fn(metamodelica::Ref<ComponentRef::NFComponentRef>) -> Result<i32>
                                    + 'static,
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
                );
                repetitions = arrayCreate(
                    size,
                    UnorderedSet::new(
                        (std::sync::Arc::new(move |__a0: metamodelica::Ref<ComponentRef::NFComponentRef>| {
                            ComponentRef::hash(&__a0)
                        })
                            as std::sync::Arc<
                                dyn ::std::ops::Fn(metamodelica::Ref<ComponentRef::NFComponentRef>) -> Result<i32>
                                    + 'static,
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
                    ),
                );
                for mut i in &**eqn_indices {
                    {
                        let __cell0 = ({
                            let __elt = (*metamodelica::index_checked(
                                &var_field!((**full).equation_names, Matrix::FULL).borrow(),
                                i.clone(),
                            )?)
                            .clone();
                            __elt
                        });
                        let __idx0 = j;
                        *metamodelica::index_mut_checked(&mut equation_names.clone().borrow_mut(), __idx0)? = __cell0;
                    }
                    {
                        let __cell1 = ({
                            let __elt = (*metamodelica::index_checked(
                                &var_field!((**full).occurrences, Matrix::FULL).borrow(),
                                i.clone(),
                            )?)
                            .clone();
                            __elt
                        });
                        let __idx1 = j;
                        *metamodelica::index_mut_checked(&mut occurrences.clone().borrow_mut(), __idx1)? = __cell1;
                    }
                    {
                        let __cell2 = ({
                            let __elt = (*metamodelica::index_checked(
                                &var_field!((**full).dependencies, Matrix::FULL).borrow(),
                                i.clone(),
                            )?)
                            .clone();
                            __elt
                        });
                        let __idx2 = j;
                        *metamodelica::index_mut_checked(&mut dependencies.clone().borrow_mut(), __idx2)? = __cell2;
                    }
                    {
                        let __cell3 = ({
                            let __elt = (*metamodelica::index_checked(
                                &var_field!((**full).solvabilities, Matrix::FULL).borrow(),
                                i.clone(),
                            )?)
                            .clone();
                            __elt
                        });
                        let __idx3 = j;
                        *metamodelica::index_mut_checked(&mut solvabilities.clone().borrow_mut(), __idx3)? = __cell3;
                    }
                    {
                        let __cell4 = ({
                            let __elt = (*metamodelica::index_checked(
                                &var_field!((**full).repetitions, Matrix::FULL).borrow(),
                                i.clone(),
                            )?)
                            .clone();
                            __elt
                        });
                        let __idx4 = j;
                        *metamodelica::index_mut_checked(&mut repetitions.clone().borrow_mut(), __idx4)? = __cell4;
                    }
                    j = j + 1;
                }
                metamodelica::Ref::new(Matrix::FULL {
                    equation_names: equation_names.clone(),
                    occurrences: occurrences.clone(),
                    dependencies: dependencies.clone(),
                    solvabilities: solvabilities.clone(),
                    repetitions: repetitions.clone(),
                    mapping: Mapping::create(eqns, vars)?,
                })
            }
            _ => metamodelica::Ref::new(Matrix::EMPTY {
                st: MatrixStrictness::FULL.clone(),
            }),
        });
        Ok(sub)
    }

    pub(crate) fn fullToFinal(
        mut full: &metamodelica::Ref<Matrix>,
        mut vars_map: metamodelica::Ref<
            UnorderedMap::UnorderedMap<metamodelica::Ref<ComponentRef::NFComponentRef>, i32>,
        >,
        mut eqns_map: metamodelica::Ref<
            UnorderedMap::UnorderedMap<metamodelica::Ref<ComponentRef::NFComponentRef>, i32>,
        >,
        mut eqns: &metamodelica::Ref<EquationPointers::EquationPointers>,
        mut st: MatrixStrictness,
        mut iter: &metamodelica::Ref<Iterator::Iterator>,
    ) -> Result<metamodelica::Ref<Matrix>> {
        let mut adj: metamodelica::Ref<Matrix> = upgrade(
            metamodelica::Ref::new(Matrix::EMPTY {
                st: MatrixStrictness::FULL.clone(),
            }),
            full,
            vars_map.clone(),
            eqns_map.clone(),
            eqns,
            st,
            iter,
        )?;
        Ok(adj)
    }

    pub(crate) fn fullToSparsity(
        mut full: &metamodelica::Ref<Matrix>,
        mut comps: &metamodelica::List<metamodelica::Ref<StrongComponent::NBStrongComponent>>,
        mut seed_set: metamodelica::Ref<UnorderedSet::UnorderedSet<metamodelica::Ref<ComponentRef::NFComponentRef>>>,
        mut pder_set: metamodelica::Ref<UnorderedSet::UnorderedSet<metamodelica::Ref<ComponentRef::NFComponentRef>>>,
        mut diff_map: metamodelica::Ref<
            UnorderedMap::UnorderedMap<
                metamodelica::Ref<ComponentRef::NFComponentRef>,
                metamodelica::Ref<ComponentRef::NFComponentRef>,
            >,
        >,
        mut isAdjoint: bool,
    ) -> Result<metamodelica::Ref<Matrix>> {
        pub(crate) type Dependencies = metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>>;

        pub(crate) fn filterSet(
            mut cref: metamodelica::Ref<ComponentRef::NFComponentRef>,
            mut set: metamodelica::Ref<UnorderedSet::UnorderedSet<metamodelica::Ref<ComponentRef::NFComponentRef>>>,
        ) -> Result<bool> {
            let mut b: bool = UnorderedSet::contains(cref.clone(), set.clone())?
                || UnorderedSet::contains(ComponentRef::stripSubscriptsAll(&cref), set.clone())?;
            Ok(b)
        }

        pub(crate) fn expandSlice(
            mut cref: metamodelica::Ref<ComponentRef::NFComponentRef>,
            mut diff_map: metamodelica::Ref<
                UnorderedMap::UnorderedMap<
                    metamodelica::Ref<ComponentRef::NFComponentRef>,
                    metamodelica::Ref<ComponentRef::NFComponentRef>,
                >,
            >,
        ) -> Result<metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>>> {
            let mut crefs: metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>>;
            let mut ty: metamodelica::Ref<Type::NFType>;
            crefs = list![cref.clone()];
            if !(UnorderedMap::contains(cref.clone(), diff_map.clone())?
                || UnorderedMap::contains(ComponentRef::stripSubscriptsAll(&cref), diff_map.clone())?)
            {
                ty = ComponentRef::getSubscriptedType(&cref, false)?;
                if Type::isArray(&ty) && Type::sizeOf(&ty, false)? <= 256 {
                    crefs = ({
                        let mut __acc: metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>> =
                            metamodelica::nil();
                        for mut c in (ComponentRef::scalarizeAll(cref.clone(), false)?).into_iter().cloned() {
                            if !(UnorderedMap::contains(c.clone(), diff_map.clone())?) {
                                continue;
                            }
                            let __x = c.clone();
                            __acc = cons(__x, __acc);
                        }
                        __acc.reverse()
                    });
                    if (crefs).is_empty() {
                        crefs = list![cref];
                    }
                }
            }
            Ok(crefs)
        }

        let mut sparsity: metamodelica::Ref<Matrix>;
        sparsity = ({
            let mut index_map: metamodelica::Ref<
                UnorderedMap::UnorderedMap<metamodelica::Ref<ComponentRef::NFComponentRef>, i32>,
            > = UnorderedMap::new(
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
            );
            let mut inner_map: metamodelica::Ref<
                UnorderedMap::UnorderedMap<
                    metamodelica::Ref<ComponentRef::NFComponentRef>,
                    metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>>,
                >,
            > = UnorderedMap::new(
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
            );
            let mut eqn_names: metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>> =
                metamodelica::nil();
            let mut eqn_iters: metamodelica::List<metamodelica::Ref<Iterator::Iterator>> = metamodelica::nil();
            let mut deps: metamodelica::List<
                metamodelica::Ref<
                    UnorderedMap::UnorderedMap<
                        metamodelica::Ref<ComponentRef::NFComponentRef>,
                        metamodelica::Ref<Dependency::Dependency>,
                    >,
                >,
            > = metamodelica::nil();
            let mut reps: metamodelica::List<
                metamodelica::Ref<UnorderedSet::UnorderedSet<metamodelica::Ref<ComponentRef::NFComponentRef>>>,
            > = metamodelica::nil();
            let mut solved_crefs: metamodelica::List<
                metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>>,
            > = metamodelica::nil();
            let mut seed_elements: metamodelica::Ref<
                UnorderedMap::UnorderedMap<
                    metamodelica::Ref<ComponentRef::NFComponentRef>,
                    metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>>,
                >,
            > = UnorderedMap::new(
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
            );
            let mut no_iters: metamodelica::Ref<
                UnorderedSet::UnorderedSet<metamodelica::Ref<ComponentRef::NFComponentRef>>,
            > = UnorderedSet::new(
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
            let mut slice_map: metamodelica::Ref<
                UnorderedMap::UnorderedMap<
                    metamodelica::Ref<ComponentRef::NFComponentRef>,
                    metamodelica::List<(i32, i32, metamodelica::Ref<ComponentRef::NFComponentRef>, i32)>,
                >,
            > = UnorderedMap::new(
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
            );
            let mut template_map: metamodelica::Ref<
                UnorderedMap::UnorderedMap<
                    metamodelica::Ref<ComponentRef::NFComponentRef>,
                    metamodelica::List<(
                        metamodelica::Ref<ComponentRef::NFComponentRef>,
                        metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>>,
                    )>,
                >,
            > = UnorderedMap::new(
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
            );
            (match &**full {
                FULL { .. } => {
                    let mut eqns: metamodelica::List<Pointer::Pointer<metamodelica::Ref<Equation::Equation>>>;
                    let mut var_crefs: metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>>;
                    let mut pder_crefs: metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>>;
                    let mut tmp_crefs: metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>>;
                    let mut eqn_name: metamodelica::Ref<ComponentRef::NFComponentRef>;
                    let mut dep_cref: metamodelica::Ref<ComponentRef::NFComponentRef> =
                        metamodelica::Ref::new(ComponentRef::EMPTY);
                    let mut seed_cref: metamodelica::Ref<ComponentRef::NFComponentRef>;
                    let mut pder_cref: metamodelica::Ref<ComponentRef::NFComponentRef>;
                    let mut oseed_cref: Option<metamodelica::Ref<ComponentRef::NFComponentRef>>;
                    let mut eqn_index: i32;
                    let mut dep: metamodelica::Ref<Dependency::Dependency>;
                    let mut local_deps: metamodelica::List<(
                        metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>>,
                        metamodelica::Ref<Dependency::Dependency>,
                        bool,
                    )>;
                    let mut repeated: bool;
                    let mut inner_deps: metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>>;
                    let mut changed: bool;
                    let mut inner_opt: Option<metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>>>;
                    let mut dep_map: metamodelica::Ref<
                        UnorderedMap::UnorderedMap<
                            metamodelica::Ref<ComponentRef::NFComponentRef>,
                            metamodelica::Ref<Dependency::Dependency>,
                        >,
                    >;
                    let mut rep_set: metamodelica::Ref<
                        UnorderedSet::UnorderedSet<metamodelica::Ref<ComponentRef::NFComponentRef>>,
                    >;
                    let mut iter_names: metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>>;
                    let mut own_iters: metamodelica::Ref<
                        UnorderedSet::UnorderedSet<metamodelica::Ref<ComponentRef::NFComponentRef>>,
                    >;
                    let mut handled: bool;
                    let mut alias_deps: metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>>;
                    let mut template_deps: metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>>;
                    for mut i in
                        1..=metamodelica::arrayLength(var_field!((**full).equation_names, Matrix::FULL).clone())
                    {
                        UnorderedMap::add(
                            ({
                                let __elt = (*metamodelica::index_checked(
                                    &var_field!((**full).equation_names, Matrix::FULL).borrow(),
                                    i,
                                )?)
                                .clone();
                                __elt
                            }),
                            i,
                            index_map.clone(),
                        )?;
                    }
                    for mut key in &*UnorderedMap::keyList(diff_map.clone()) {
                        if ComponentRef::hasSubscripts(metamodelica::AsArg::as_arg(&key))?
                            && List::all(
                                &(ComponentRef::subscriptsAllFlat(metamodelica::AsArg::as_arg(&key))?),
                                &move |__a0: metamodelica::Ref<Subscript::NFSubscript>| Subscript::isLiteral(&__a0),
                            )?
                            && !(UnorderedMap::contains(
                                ComponentRef::stripSubscriptsAll(metamodelica::AsArg::as_arg(&key)),
                                diff_map.clone(),
                            )?)
                        {
                            UnorderedMap::add(
                                ComponentRef::stripSubscriptsAll(metamodelica::AsArg::as_arg(&key)),
                                metamodelica::cons(
                                    key.clone(),
                                    UnorderedMap::getOrDefault(
                                        ComponentRef::stripSubscriptsAll(metamodelica::AsArg::as_arg(&key)),
                                        seed_elements.clone(),
                                        metamodelica::nil(),
                                    )?,
                                ),
                                seed_elements.clone(),
                            )?;
                        }
                    }
                    for mut comp in &**comps {
                        eqns = StrongComponent::getEquations(comp.clone())?;
                        var_crefs = StrongComponent::getVariableCrefs(metamodelica::AsArg::as_arg(&comp))?;
                        for mut eqn in &*eqns {
                            eqn_name = BEquation::Equation::getEqnName(eqn.clone())?;
                            eqn_index = UnorderedMap::getSafe(
                                eqn_name.clone(),
                                index_map.clone(),
                                metamodelica::sourceInfo!("NBackEnd/Util/NBAdjacency.mo"),
                            )?;
                            local_deps = metamodelica::nil();
                            changed = false;
                            let __range0 = &*UnorderedMap::toList(
                                ({
                                    let __elt = (*metamodelica::index_checked(
                                        &var_field!((**full).dependencies, Matrix::FULL).borrow(),
                                        eqn_index,
                                    )?)
                                    .clone();
                                    __elt
                                }),
                            );
                            for mut tpl in __range0 {
                                (dep_cref, dep) = tpl.clone();
                                repeated = UnorderedSet::contains(
                                    dep_cref.clone(),
                                    ({
                                        let __elt = (*metamodelica::index_checked(
                                            &var_field!((**full).repetitions, Matrix::FULL).borrow(),
                                            eqn_index,
                                        )?)
                                        .clone();
                                        __elt
                                    }),
                                )?;
                                (inner_deps, changed) = (::match_deref::match_deref! { match &(UnorderedMap::get(dep_cref.clone(), inner_map.clone())?) {
                                    Some(__esc_inner_deps) => {
                                        inner_deps = (*__esc_inner_deps).clone();
                                        (inner_deps.clone(), true)
                                    },
                                    _ => {
                                        inner_deps = list![dep_cref.clone()];
                                        alias_deps = if (filterSet(dep_cref.clone(), seed_set.clone())?) {metamodelica::nil()} else {sparsitySliceAliasDeps(&dep_cref, slice_map.clone())?};
                                        handled = false;
                                        template_deps = metamodelica::nil();
                                        if !(filterSet(dep_cref.clone(), seed_set.clone())?) {
                                            (handled, template_deps) = sparsityTemplateDeps(&dep_cref, template_map.clone())?;
                                        }
                                        if !((alias_deps).is_empty()) {
                                            inner_deps = metamodelica::cons(dep_cref, alias_deps);
                                            changed = true;
                                        } else if handled {
                                            inner_deps = metamodelica::cons(dep_cref, template_deps);
                                            changed = true;
                                        } else if !(filterSet(dep_cref.clone(), seed_set.clone())?) {
                                            inner_opt = UnorderedMap::get(ComponentRef::stripSubscriptsAll(&dep_cref), inner_map.clone())?;
                                            if (inner_opt).is_some() {
                                                inner_deps = metamodelica::cons(dep_cref, List::flatten(({
                                    let mut __acc: metamodelica::List<metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>>> = metamodelica::nil();
                                    for mut c in (Util::getOption(inner_opt)?).into_iter().cloned() {
                                        let __x = sparsityExpandForeignIterators(c.clone(), no_iters.clone(), seed_elements.clone())?;
                                        __acc = cons(__x, __acc);
                                    }
                                    __acc.reverse()
                                }))?);
                                                changed = true;
                                            }
                                        }
                                        (inner_deps, changed)
                                    },
                                    _ => unreachable!("match_deref! exhaustiveness placeholder"),
                                } });
                                local_deps = metamodelica::cons((inner_deps.clone(), dep, repeated), local_deps);
                            }
                            (pder_crefs, tmp_crefs) = List::splitOnTrue(
                                &var_crefs,
                                &({
                                    let __pe_b1 = pder_set.clone();
                                    move |__pe_a0| filterSet(__pe_a0, __pe_b1.clone())
                                }),
                            )?;
                            if !((pder_crefs).is_empty()) {
                                dep_map = UnorderedMap::new((std::sync::Arc::new(move |__a0: metamodelica::Ref<ComponentRef::NFComponentRef>| ComponentRef::hash(&__a0)) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<ComponentRef::NFComponentRef>) -> Result<i32> + 'static>), (std::sync::Arc::new(move |__a0: metamodelica::Ref<ComponentRef::NFComponentRef>, __a1: metamodelica::Ref<ComponentRef::NFComponentRef>| ComponentRef::isEqual(&__a0, &__a1)) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<ComponentRef::NFComponentRef>, metamodelica::Ref<ComponentRef::NFComponentRef>) -> Result<bool> + 'static>), 1);
                                rep_set = UnorderedSet::new((std::sync::Arc::new(move |__a0: metamodelica::Ref<ComponentRef::NFComponentRef>| ComponentRef::hash(&__a0)) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<ComponentRef::NFComponentRef>) -> Result<i32> + 'static>), (std::sync::Arc::new(move |__a0: metamodelica::Ref<ComponentRef::NFComponentRef>, __a1: metamodelica::Ref<ComponentRef::NFComponentRef>| ComponentRef::isEqual(&__a0, &__a1)) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<ComponentRef::NFComponentRef>, metamodelica::Ref<ComponentRef::NFComponentRef>) -> Result<bool> + 'static>), 13);
                                (iter_names, _, _) = BEquation::Iterator::getFrames(
                                    &(BEquation::Equation::getForIterator(&(Pointer::access(eqn.clone())))),
                                );
                                own_iters = UnorderedSet::fromList(&iter_names, (std::sync::Arc::new(move |__a0: metamodelica::Ref<ComponentRef::NFComponentRef>| ComponentRef::hash(&__a0)) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<ComponentRef::NFComponentRef>) -> Result<i32> + 'static>), (std::sync::Arc::new(move |__a0: metamodelica::Ref<ComponentRef::NFComponentRef>, __a1: metamodelica::Ref<ComponentRef::NFComponentRef>| ComponentRef::isEqual(&__a0, &__a1)) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<ComponentRef::NFComponentRef>, metamodelica::Ref<ComponentRef::NFComponentRef>) -> Result<bool> + 'static>))?;
                                for mut tpl in &*local_deps {
                                    (inner_deps, dep, repeated) = tpl.clone();
                                    inner_deps = List::flatten(
                                        ({
                                            let mut __acc: metamodelica::List<
                                                metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>>,
                                            > = metamodelica::nil();
                                            for mut c in (inner_deps).into_iter().cloned() {
                                                let __x = sparsityExpandForeignIterators(
                                                    c.clone(),
                                                    own_iters.clone(),
                                                    seed_elements.clone(),
                                                )?;
                                                __acc = cons(__x, __acc);
                                            }
                                            __acc.reverse()
                                        }),
                                    )?;
                                    for mut dep_cref in &*List::flatten(
                                        ({
                                            let mut __acc: metamodelica::List<
                                                metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>>,
                                            > = metamodelica::nil();
                                            for mut c in (inner_deps).into_iter().cloned() {
                                                let __x = expandSlice(c.clone(), diff_map.clone())?;
                                                __acc = cons(__x, __acc);
                                            }
                                            __acc.reverse()
                                        }),
                                    )? {
                                        let mut dep_cref = dep_cref.clone();
                                        if filterSet(dep_cref.clone(), seed_set.clone())? {
                                            oseed_cref = (::match_deref::match_deref! { match &(UnorderedMap::get(dep_cref.clone(), diff_map.clone())?) {
                                                Some(__esc_seed_cref) => {
                                                    seed_cref = (*__esc_seed_cref).clone();
                                                    Some(seed_cref.clone())
                                                },
                                                _ => (::match_deref::match_deref! { match &(UnorderedMap::get(ComponentRef::stripSubscriptsAll(&dep_cref), diff_map.clone())?) {
                                                Some(__esc_seed_cref) => {
                                                    seed_cref = (*__esc_seed_cref).clone();
                                                    Some(ComponentRef::copySubscripts(&dep_cref, ComponentRef::stripSubscriptsAll(metamodelica::AsArg::as_arg(&seed_cref)))?)
                                                },
                                                _ => None,
                                                _ => unreachable!("match_deref! exhaustiveness placeholder"),
                                            } }),
                                                _ => unreachable!("match_deref! exhaustiveness placeholder"),
                                            } });
                                            if (oseed_cref).is_some() {
                                                seed_cref = Util::getOption(oseed_cref)?;
                                                UnorderedMap::add(seed_cref.clone(), dep.clone(), dep_map.clone())?;
                                                if repeated {
                                                    UnorderedSet::add(seed_cref.clone(), rep_set.clone())?;
                                                }
                                            }
                                        }
                                    }
                                }
                                if changed {
                                    inner_deps = List::flatten(
                                        ({
                                            let mut __acc: metamodelica::List<
                                                metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>>,
                                            > = metamodelica::nil();
                                            for mut tpl in (local_deps.clone()).into_iter().cloned() {
                                                let __x = Util::tuple31(tpl.clone());
                                                __acc = cons(__x, __acc);
                                            }
                                            __acc.reverse()
                                        }),
                                    )?;
                                    inner_deps = UnorderedSet::unique_list(inner_deps, (std::sync::Arc::new(move |__a0: metamodelica::Ref<ComponentRef::NFComponentRef>| ComponentRef::hash(&__a0)) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<ComponentRef::NFComponentRef>) -> Result<i32> + 'static>), (std::sync::Arc::new(move |__a0: metamodelica::Ref<ComponentRef::NFComponentRef>, __a1: metamodelica::Ref<ComponentRef::NFComponentRef>| ComponentRef::isEqual(&__a0, &__a1)) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<ComponentRef::NFComponentRef>, metamodelica::Ref<ComponentRef::NFComponentRef>) -> Result<bool> + 'static>))?;
                                } else {
                                    inner_deps = UnorderedMap::keyList(
                                        ({
                                            let __elt = (*metamodelica::index_checked(
                                                &var_field!((**full).dependencies, Matrix::FULL).borrow(),
                                                eqn_index,
                                            )?)
                                            .clone();
                                            __elt
                                        }),
                                    );
                                }
                                inner_deps = List::filterOnTrue(
                                    List::flatten(
                                        ({
                                            let mut __acc: metamodelica::List<
                                                metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>>,
                                            > = metamodelica::nil();
                                            for mut c in (inner_deps).into_iter().cloned() {
                                                let __x = expandSlice(c.clone(), diff_map.clone())?;
                                                __acc = cons(__x, __acc);
                                            }
                                            __acc.reverse()
                                        }),
                                    )?,
                                    (std::sync::Arc::new({
                                        let __pe_b1 = seed_set.clone();
                                        move |__pe_a0| filterSet(__pe_a0, __pe_b1.clone())
                                    })
                                        as std::sync::Arc<
                                            dyn ::std::ops::Fn(
                                                    metamodelica::Ref<ComponentRef::NFComponentRef>,
                                                )
                                                    -> Result<bool>
                                                + 'static,
                                        >),
                                )?;
                                for mut cref in &*pder_crefs {
                                    sparsityAddInner(cref.clone(), inner_deps.clone(), inner_map.clone())?;
                                    sparsityAddTemplate(cref.clone(), inner_deps.clone(), template_map.clone())?;
                                }
                                if isAdjoint {
                                    pder_crefs = ({
                                        let mut __acc: metamodelica::List<
                                            metamodelica::Ref<ComponentRef::NFComponentRef>,
                                        > = metamodelica::nil();
                                        for mut cref in (pder_crefs).into_iter().cloned() {
                                            let __x = (::match_deref::match_deref! { match &(UnorderedMap::get(ComponentRef::stripSubscriptsAll(&(cref.clone())), diff_map.clone())?) {
                                                Some(__esc_pder_cref) => {
                                                    pder_cref = (*__esc_pder_cref).clone();
                                                    ComponentRef::copySubscripts(&(cref.clone()), pder_cref.clone())?
                                                },
                                                _ => {
                                                    Error::addMessage(Error::INTERNAL_ERROR.clone(), list![{ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBAdjacency.Matrix.fullToSparsity")); __mm_s.push_str(&*literal!(" failed because no adjoint seed was found for ")); __mm_s.push_str(&*ComponentRef::toString(&(cref.clone()))?); __mm_s.push_str(&*literal!(" in diff_map.")); ArcStr::from(__mm_s) }])?;
                                                    return Err("fail")
                                                },
                                                _ => unreachable!("match_deref! exhaustiveness placeholder"),
                                            } });
                                            __acc = cons(__x, __acc);
                                        }
                                        __acc.reverse()
                                    });
                                } else {
                                    match '__try1: {
                                        pder_crefs = ({
                                            let mut __acc: metamodelica::List<
                                                metamodelica::Ref<ComponentRef::NFComponentRef>,
                                            > = metamodelica::nil();
                                            for mut cref in (pder_crefs.clone()).into_iter().cloned() {
                                                let __x = unwrap_break_err!(BVariable::getPartnerCref(&(cref.clone()), &({ let __pe_b1 = false; move |__pe_a0| Ok(BVariable::getVarPDer(__pe_a0, __pe_b1.clone())) }), false), '__try1);
                                                __acc = cons(__x, __acc);
                                            }
                                            __acc.reverse()
                                        });
                                        Ok::<_, &'static str>((pder_crefs.clone(),))
                                    } {
                                        Ok((__try1_o0,)) => {
                                            pder_crefs = __try1_o0;
                                        }
                                        Err(__try1_err) => {
                                            Error::addMessage(
                                                Error::INTERNAL_ERROR.clone(),
                                                list![{
                                                    let mut __mm_s = String::new();
                                                    __mm_s.push_str(&*literal!("NBAdjacency.Matrix.fullToSparsity"));
                                                    __mm_s.push_str(&*literal!(" failed for "));
                                                    __mm_s.push_str(&*List::toString(
                                                        pder_crefs.clone(),
                                                        &move |__a0: metamodelica::Ref<
                                                            ComponentRef::NFComponentRef,
                                                        >| {
                                                            ComponentRef::toString(&__a0)
                                                        },
                                                        List::Style::FLAT_CURLY.clone(),
                                                    )?);
                                                    __mm_s.push_str(&*literal!(" because they were supposed to be a row vars but at least one does not have a corresponding partial derivative."));
                                                    ArcStr::from(__mm_s)
                                                }],
                                            )?;
                                            return Err(__try1_err);
                                        }
                                    }
                                }
                                eqn_names = metamodelica::cons(eqn_name, eqn_names);
                                eqn_iters = metamodelica::cons(
                                    BEquation::Equation::getForIterator(&(Pointer::access(eqn.clone()))),
                                    eqn_iters,
                                );
                                deps = metamodelica::cons(dep_map, deps);
                                reps = metamodelica::cons(rep_set, reps);
                                solved_crefs = metamodelica::cons(pder_crefs, solved_crefs);
                            }
                            if !((tmp_crefs).is_empty()) {
                                if changed {
                                    inner_deps = List::flatten(
                                        ({
                                            let mut __acc: metamodelica::List<
                                                metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>>,
                                            > = metamodelica::nil();
                                            for mut tpl in (local_deps).into_iter().cloned() {
                                                let __x = Util::tuple31(tpl.clone());
                                                __acc = cons(__x, __acc);
                                            }
                                            __acc.reverse()
                                        }),
                                    )?;
                                    inner_deps = UnorderedSet::unique_list(inner_deps, (std::sync::Arc::new(move |__a0: metamodelica::Ref<ComponentRef::NFComponentRef>| ComponentRef::hash(&__a0)) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<ComponentRef::NFComponentRef>) -> Result<i32> + 'static>), (std::sync::Arc::new(move |__a0: metamodelica::Ref<ComponentRef::NFComponentRef>, __a1: metamodelica::Ref<ComponentRef::NFComponentRef>| ComponentRef::isEqual(&__a0, &__a1)) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<ComponentRef::NFComponentRef>, metamodelica::Ref<ComponentRef::NFComponentRef>) -> Result<bool> + 'static>))?;
                                } else {
                                    inner_deps = UnorderedMap::keyList(
                                        ({
                                            let __elt = (*metamodelica::index_checked(
                                                &var_field!((**full).dependencies, Matrix::FULL).borrow(),
                                                eqn_index,
                                            )?)
                                            .clone();
                                            __elt
                                        }),
                                    );
                                }
                                inner_deps = List::filterOnTrue(
                                    List::flatten(
                                        ({
                                            let mut __acc: metamodelica::List<
                                                metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>>,
                                            > = metamodelica::nil();
                                            for mut c in (inner_deps).into_iter().cloned() {
                                                let __x = expandSlice(c.clone(), diff_map.clone())?;
                                                __acc = cons(__x, __acc);
                                            }
                                            __acc.reverse()
                                        }),
                                    )?,
                                    (std::sync::Arc::new({
                                        let __pe_b1 = seed_set.clone();
                                        move |__pe_a0| filterSet(__pe_a0, __pe_b1.clone())
                                    })
                                        as std::sync::Arc<
                                            dyn ::std::ops::Fn(
                                                    metamodelica::Ref<ComponentRef::NFComponentRef>,
                                                )
                                                    -> Result<bool>
                                                + 'static,
                                        >),
                                )?;
                                for mut cref in &*tmp_crefs {
                                    sparsityAddInner(cref.clone(), inner_deps.clone(), inner_map.clone())?;
                                    sparsityAddTemplate(cref.clone(), inner_deps.clone(), template_map.clone())?;
                                }
                                sparsityAddSliceAlias(
                                    &(Pointer::access(eqn.clone())),
                                    &tmp_crefs,
                                    seed_set.clone(),
                                    slice_map.clone(),
                                )?;
                            }
                        }
                    }
                    metamodelica::Ref::new(Matrix::SPARSITY {
                        equation_names: metamodelica::arrayFromVec(eqn_names.reverse().into_iter().cloned().collect()),
                        equation_iterators: metamodelica::arrayFromVec(
                            eqn_iters.reverse().into_iter().cloned().collect(),
                        ),
                        dependencies: metamodelica::arrayFromVec(deps.reverse().into_iter().cloned().collect()),
                        repetitions: metamodelica::arrayFromVec(reps.reverse().into_iter().cloned().collect()),
                        solved_crefs: metamodelica::arrayFromVec(solved_crefs.reverse().into_iter().cloned().collect()),
                    })
                }
                EMPTY { .. } => metamodelica::Ref::new(Matrix::SPARSITY {
                    equation_names: metamodelica::arrayFromVec(metamodelica::nil().into_iter().cloned().collect()),
                    equation_iterators: metamodelica::arrayFromVec(metamodelica::nil().into_iter().cloned().collect()),
                    dependencies: metamodelica::arrayFromVec(metamodelica::nil().into_iter().cloned().collect()),
                    repetitions: metamodelica::arrayFromVec(metamodelica::nil().into_iter().cloned().collect()),
                    solved_crefs: metamodelica::arrayFromVec(metamodelica::nil().into_iter().cloned().collect()),
                }),
                _ => {
                    Error::addMessage(
                        Error::INTERNAL_ERROR.clone(),
                        list![{
                            let mut __mm_s = String::new();
                            __mm_s.push_str(&*literal!("NBAdjacency.Matrix.fullToSparsity"));
                            __mm_s.push_str(&*literal!(
                                " failed because of wrong matrix type.\n            Expected: full, Got :"
                            ));
                            __mm_s.push_str(&*strictnessString(getStrictness(full)?));
                            __mm_s.push_str(&*literal!("."));
                            ArcStr::from(__mm_s)
                        }],
                    )?;
                    return Err("fail");
                }
            })
        });
        if Flags::isSet(Flags::BLT_MATRIX_DUMP.clone())? {
            metamodelica::print({
                let mut __mm_s = String::new();
                __mm_s.push_str(&*toString(&sparsity, literal!(""))?);
                __mm_s.push_str(&*literal!("\n"));
                ArcStr::from(__mm_s)
            });
        }
        Ok(sparsity)
    }

    /// solved cref with the subscripts of its equation, its dependencies
    pub type InnerTemplate = (
        metamodelica::Ref<ComponentRef::NFComponentRef>,
        metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>>,
    );

    pub type InnerTemplates = metamodelica::List<(
        metamodelica::Ref<ComponentRef::NFComponentRef>,
        metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>>,
    )>;

    pub(crate) fn sparsityAddTemplate(
        mut cref: metamodelica::Ref<ComponentRef::NFComponentRef>,
        mut deps: metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>>,
        mut template_map: metamodelica::Ref<
            UnorderedMap::UnorderedMap<
                metamodelica::Ref<ComponentRef::NFComponentRef>,
                metamodelica::List<(
                    metamodelica::Ref<ComponentRef::NFComponentRef>,
                    metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>>,
                )>,
            >,
        >,
    ) -> Result<()> {
        let mut stripped: metamodelica::Ref<ComponentRef::NFComponentRef> = ComponentRef::stripSubscriptsAll(&cref);
        UnorderedMap::add(
            stripped.clone(),
            metamodelica::cons(
                (cref, deps),
                UnorderedMap::getOrDefault(stripped, template_map.clone(), metamodelica::nil())?,
            ),
            template_map,
        )?;
        Ok(())
    }

    pub(crate) fn sparsityTemplateDeps(
        mut cref: &metamodelica::Ref<ComponentRef::NFComponentRef>,
        mut template_map: metamodelica::Ref<
            UnorderedMap::UnorderedMap<
                metamodelica::Ref<ComponentRef::NFComponentRef>,
                metamodelica::List<(
                    metamodelica::Ref<ComponentRef::NFComponentRef>,
                    metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>>,
                )>,
            >,
        >,
    ) -> Result<(
        bool,
        metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>>,
    )> {
        let mut handled: bool = false;
        let mut deps: metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>> = metamodelica::nil();
        let mut templates: InnerTemplates = UnorderedMap::getOrDefault(
            ComponentRef::stripSubscriptsAll(cref),
            template_map.clone(),
            metamodelica::nil(),
        )?;
        let mut subs: metamodelica::List<metamodelica::Ref<Subscript::NFSubscript>> =
            ComponentRef::subscriptsAllWithWholeFlat(cref)?;
        let mut status: i32;
        let mut bindings: metamodelica::Ref<
            UnorderedMap::UnorderedMap<
                metamodelica::Ref<ComponentRef::NFComponentRef>,
                metamodelica::Ref<Expression::NFExpression>,
            >,
        >;
        let mut key: metamodelica::Ref<ComponentRef::NFComponentRef>;
        let mut dep: metamodelica::Ref<ComponentRef::NFComponentRef>;
        let mut key_deps: metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>>;
        let mut matched: bool = false;
        for mut template in &*templates {
            (key, key_deps) = template.clone();
            (status, bindings) = sparsityUnify(ComponentRef::subscriptsAllWithWholeFlat(&key)?, subs.clone())?;
            if status == 2 {
                return Ok((handled, deps));
            } else if status == 1 {
                matched = true;
                for mut d in &*key_deps {
                    if sparsityHasUnbound(metamodelica::AsArg::as_arg(&d), bindings.clone())? {
                        deps =
                            metamodelica::cons(ComponentRef::stripSubscriptsAll(metamodelica::AsArg::as_arg(&d)), deps);
                    } else {
                        dep = ComponentRef::mapExp(
                            &(ComponentRef::mapExp(
                                metamodelica::AsArg::as_arg(&d),
                                (std::sync::Arc::new({
                                    let __pe_b1 = bindings.clone();
                                    move |__pe_a0| sparsityBind(__pe_a0, __pe_b1.clone())
                                })
                                    as std::sync::Arc<
                                        dyn ::std::ops::Fn(
                                                metamodelica::Ref<Expression::NFExpression>,
                                            )
                                                -> Result<metamodelica::Ref<Expression::NFExpression>>
                                            + 'static,
                                    >),
                            )?),
                            (std::sync::Arc::new(sparsitySimplify)
                                as std::sync::Arc<
                                    dyn ::std::ops::Fn(
                                            metamodelica::Ref<Expression::NFExpression>,
                                        )
                                            -> Result<metamodelica::Ref<Expression::NFExpression>>
                                        + 'static,
                                >),
                        )?;
                        if sparsityInBounds(&dep)? {
                            deps = metamodelica::cons(dep, deps);
                        }
                    }
                }
            }
        }
        handled = matched;
        Ok((handled, deps))
    }

    pub(crate) fn sparsityUnify(
        mut key_subs: metamodelica::List<metamodelica::Ref<Subscript::NFSubscript>>,
        mut subs: metamodelica::List<metamodelica::Ref<Subscript::NFSubscript>>,
    ) -> Result<(
        i32,
        metamodelica::Ref<
            UnorderedMap::UnorderedMap<
                metamodelica::Ref<ComponentRef::NFComponentRef>,
                metamodelica::Ref<Expression::NFExpression>,
            >,
        >,
    )> {
        let mut status: i32 = 1;
        let mut bindings: metamodelica::Ref<
            UnorderedMap::UnorderedMap<
                metamodelica::Ref<ComponentRef::NFComponentRef>,
                metamodelica::Ref<Expression::NFExpression>,
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
        let mut key_exp: metamodelica::Ref<Expression::NFExpression>;
        let mut exp: metamodelica::Ref<Expression::NFExpression>;
        let mut value: metamodelica::Ref<Expression::NFExpression>;
        let mut ok: bool;
        let mut negated: bool;
        let mut iter: metamodelica::Ref<ComponentRef::NFComponentRef>;
        let mut offset: i32;
        if ((key_subs).len() as i32) != ((subs).len() as i32) {
            status = 2;
            return Ok((status, bindings));
        }
        for mut tpl in &*List::zip(key_subs, subs) {
            let () = (::match_deref::match_deref! { match &(tpl.clone()) {
                (Deref @ Subscript::WHOLE, _) => (),
                (Deref @ Subscript::INDEX { index: key_exp }, _) if (Expression::isInteger(metamodelica::AsArg::as_arg(&key_exp))) => {
                    let () = (match &*(Util::tuple22(tpl.clone())) {
                Subscript::INDEX { index: exp } if (Expression::isInteger(metamodelica::AsArg::as_arg(&exp))) => {
                    if Expression::integerValue(exp.clone())? != Expression::integerValue(key_exp.clone())? {
                        status = 0;
                    }
                    ()
                },
                _ => (),
            });
                    ()
                },
                (Deref @ Subscript::INDEX { index: __esc_key_exp }, Deref @ Subscript::INDEX { index: __esc_exp }) => {
                    key_exp = (*__esc_key_exp).clone();
                    exp = (*__esc_exp).clone();
                    (ok, iter, offset, negated) = sparsityIteratorOffset(metamodelica::AsArg::as_arg(&key_exp));
                    if ok {
                        value = if (negated) {SimplifyExp::simplify(metamodelica::Ref::new(Expression::NFExpression::BINARY { exp1: metamodelica::Ref::new(Expression::NFExpression::INTEGER { value: offset }), operator: Operator::makeSub(openmodelica_nf_frontend::NFType::interned_INTEGER()), exp2: exp.clone() }), false)?} else {SimplifyExp::simplify(metamodelica::Ref::new(Expression::NFExpression::BINARY { exp1: exp.clone(), operator: Operator::makeSub(openmodelica_nf_frontend::NFType::interned_INTEGER()), exp2: metamodelica::Ref::new(Expression::NFExpression::INTEGER { value: offset }) }), false)?};
                        if UnorderedMap::contains(iter.clone(), bindings.clone())? && !(Expression::isEqual(value.clone(), UnorderedMap::getSafe(iter.clone(), bindings.clone(), metamodelica::sourceInfo!("NBackEnd/Util/NBAdjacency.mo"))?)?) {
                            status = 2;
                        } else {
                            UnorderedMap::add(iter, value, bindings.clone())?;
                        }
                    } else {
                        status = 2;
                    }
                    ()
                },
                _ => {
                    status = 2;
                    ()
                },
                _ => unreachable!("match_deref! exhaustiveness placeholder"),
            } });
            if status != 1 {
                return Ok((status, bindings));
            }
        }
        Ok((status, bindings))
    }

    pub(crate) fn sparsityIteratorOffset(
        mut exp: &metamodelica::Ref<Expression::NFExpression>,
    ) -> (bool, metamodelica::Ref<ComponentRef::NFComponentRef>, i32, bool) {
        let mut ok: bool = true;
        let mut iter: metamodelica::Ref<ComponentRef::NFComponentRef> =
            openmodelica_nf_frontend::NFComponentRef::interned_EMPTY();
        let mut offset: i32 = 0;
        let mut negated: bool = false;
        let () = (::match_deref::match_deref! { match exp {
            Deref @ Expression::CREF { cref: __exp_cref, .. } if (ComponentRef::isIterator(metamodelica::AsArg::as_arg(&__exp_cref))) => {
                iter = __exp_cref.clone();
                ()
            },
            Deref @ Expression::BINARY { exp1: Deref @ Expression::CREF { cref: __esc_iter, .. }, operator: Deref @ Operator::OPERATOR { op: Operator::Op::ADD, .. }, exp2: Deref @ Expression::INTEGER { value: __esc_offset } } if (ComponentRef::isIterator(metamodelica::AsArg::as_arg(&__esc_iter))) => {
                iter = (*__esc_iter).clone();
                offset = (*__esc_offset).clone();
                ()
            },
            Deref @ Expression::BINARY { exp1: Deref @ Expression::INTEGER { value: __esc_offset }, operator: Deref @ Operator::OPERATOR { op: Operator::Op::ADD, .. }, exp2: Deref @ Expression::CREF { cref: __esc_iter, .. } } if (ComponentRef::isIterator(metamodelica::AsArg::as_arg(&__esc_iter))) => {
                iter = (*__esc_iter).clone();
                offset = (*__esc_offset).clone();
                ()
            },
            Deref @ Expression::BINARY { exp1: Deref @ Expression::CREF { cref: __esc_iter, .. }, operator: Deref @ Operator::OPERATOR { op: Operator::Op::SUB, .. }, exp2: Deref @ Expression::INTEGER { value: __esc_offset } } if (ComponentRef::isIterator(metamodelica::AsArg::as_arg(&__esc_iter))) => {
                iter = (*__esc_iter).clone();
                offset = (*__esc_offset).clone();
                offset = -(offset.clone());
                ()
            },
            Deref @ Expression::BINARY { exp1: Deref @ Expression::INTEGER { value: __esc_offset }, operator: Deref @ Operator::OPERATOR { op: Operator::Op::SUB, .. }, exp2: Deref @ Expression::CREF { cref: __esc_iter, .. } } if (ComponentRef::isIterator(metamodelica::AsArg::as_arg(&__esc_iter))) => {
                iter = (*__esc_iter).clone();
                offset = (*__esc_offset).clone();
                negated = true;
                ()
            },
            Deref @ Expression::MULTARY { operator: Deref @ Operator::OPERATOR { op: Operator::Op::ADD, .. }, arguments: __exp_arguments, inv_arguments: __exp_inv_arguments } => {
                (ok, iter, offset, negated) = sparsityMultaryOffset(metamodelica::AsArg::as_arg(&__exp_arguments), metamodelica::AsArg::as_arg(&__exp_inv_arguments));
                ()
            },
            _ => {
                ok = false;
                ()
            },
            _ => unreachable!("match_deref! exhaustiveness placeholder"),
        } });
        (ok, iter, offset, negated)
    }

    pub(crate) fn sparsitySimplify(
        mut exp: metamodelica::Ref<Expression::NFExpression>,
    ) -> Result<metamodelica::Ref<Expression::NFExpression>> {
        let mut exp: metamodelica::Ref<Expression::NFExpression> = exp;
        exp = SimplifyExp::simplify(exp, false)?;
        Ok(exp)
    }

    pub(crate) fn sparsityHasUnbound(
        mut cref: &metamodelica::Ref<ComponentRef::NFComponentRef>,
        mut bindings: metamodelica::Ref<
            UnorderedMap::UnorderedMap<
                metamodelica::Ref<ComponentRef::NFComponentRef>,
                metamodelica::Ref<Expression::NFExpression>,
            >,
        >,
    ) -> Result<bool> {
        let mut b: bool = false;
        for mut sub in &*({
            let mut __acc: metamodelica::List<metamodelica::Ref<Subscript::NFSubscript>> = metamodelica::nil();
            for mut s in (ComponentRef::subscriptsAllFlat(cref)?).into_iter().cloned() {
                if !(!(Subscript::isWhole(&(s.clone())))) {
                    continue;
                }
                let __x = s.clone();
                __acc = cons(__x, __acc);
            }
            __acc.reverse()
        }) {
            for mut c in &*UnorderedSet::toList(Expression::extractCrefs(Subscript::toExp(
                metamodelica::AsArg::as_arg(&sub),
            )?)?) {
                if ComponentRef::isIterator(metamodelica::AsArg::as_arg(&c))
                    && !(UnorderedMap::contains(c.clone(), bindings.clone())?)
                {
                    b = true;
                    return Ok(b);
                }
            }
        }
        Ok(b)
    }

    pub(crate) fn sparsityMultaryOffset(
        mut arguments: &metamodelica::List<metamodelica::Ref<Expression::NFExpression>>,
        mut inv_arguments: &metamodelica::List<metamodelica::Ref<Expression::NFExpression>>,
    ) -> (bool, metamodelica::Ref<ComponentRef::NFComponentRef>, i32, bool) {
        let mut ok: bool = true;
        let mut iter: metamodelica::Ref<ComponentRef::NFComponentRef> =
            openmodelica_nf_frontend::NFComponentRef::interned_EMPTY();
        let mut offset: i32 = 0;
        let mut negated: bool = false;
        for mut e in &**arguments {
            (ok, iter, offset, negated) =
                sparsityMultaryArgument(metamodelica::AsArg::as_arg(&e), false, ok, iter, offset, negated);
        }
        for mut e in &**inv_arguments {
            (ok, iter, offset, negated) =
                sparsityMultaryArgument(metamodelica::AsArg::as_arg(&e), true, ok, iter, offset, negated);
        }
        ok = ok && !(ComponentRef::isEmpty(&iter));
        (ok, iter, offset, negated)
    }

    pub(crate) fn sparsityMultaryArgument(
        mut e: &metamodelica::Ref<Expression::NFExpression>,
        mut inv: bool,
        mut ok: bool,
        mut iter: metamodelica::Ref<ComponentRef::NFComponentRef>,
        mut offset: i32,
        mut negated: bool,
    ) -> (bool, metamodelica::Ref<ComponentRef::NFComponentRef>, i32, bool) {
        let mut ok: bool = ok;
        let mut iter: metamodelica::Ref<ComponentRef::NFComponentRef> = iter;
        let mut offset: i32 = offset;
        let mut negated: bool = negated;
        let () = (match &**e {
            Expression::INTEGER { value: __e_value } => {
                offset = if (inv) {
                    offset - __e_value.clone()
                } else {
                    offset + __e_value.clone()
                };
                ()
            }
            Expression::CREF { cref: __e_cref, .. }
                if (ComponentRef::isIterator(metamodelica::AsArg::as_arg(&__e_cref))) =>
            {
                ok = ok && ComponentRef::isEmpty(&iter);
                iter = __e_cref.clone();
                negated = inv;
                ()
            }
            _ => {
                ok = false;
                ()
            }
        });
        (ok, iter, offset, negated)
    }

    pub(crate) fn sparsityBind(
        mut exp: metamodelica::Ref<Expression::NFExpression>,
        mut bindings: metamodelica::Ref<
            UnorderedMap::UnorderedMap<
                metamodelica::Ref<ComponentRef::NFComponentRef>,
                metamodelica::Ref<Expression::NFExpression>,
            >,
        >,
    ) -> Result<metamodelica::Ref<Expression::NFExpression>> {
        let mut exp: metamodelica::Ref<Expression::NFExpression> = exp;
        exp = (match &*exp {
            Expression::CREF { cref: __exp_cref, .. }
                if (UnorderedMap::contains(__exp_cref.clone(), bindings.clone())?) =>
            {
                UnorderedMap::getSafe(
                    __exp_cref.clone(),
                    bindings.clone(),
                    metamodelica::sourceInfo!("NBackEnd/Util/NBAdjacency.mo"),
                )?
            }
            _ => exp,
        });
        Ok(exp)
    }

    pub(crate) fn sparsityInBounds(mut cref: &metamodelica::Ref<ComponentRef::NFComponentRef>) -> Result<bool> {
        let mut b: bool = true;
        b = (match &**cref {
            ComponentRef::CREF {
                restCref: __cref_restCref,
                subscripts: __cref_subscripts,
                ty: __cref_ty,
                ..
            } => {
                let mut dims: metamodelica::List<metamodelica::Ref<Dimension::NFDimension>>;
                dims = Type::arrayDims(__cref_ty.clone());
                if ((dims).len() as i32) == ((__cref_subscripts).len() as i32) {
                    for mut tpl in &*List::zip(__cref_subscripts.clone(), dims) {
                        let () = (::match_deref::match_deref! { match &(tpl.clone()) {
                            (Deref @ Subscript::INDEX { index: e }, d) if (Expression::isInteger(metamodelica::AsArg::as_arg(&e)) && Dimension::isKnown(metamodelica::AsArg::as_arg(&d), false)) => {
                                if Expression::integerValue(e.clone())? < 1 || Expression::integerValue(e.clone())? > Dimension::size(metamodelica::AsArg::as_arg(&d), false)? {
                                    b = false;
                                }
                                ()
                            },
                            _ => {
                                ()
                            },
                            _ => unreachable!("match_deref! exhaustiveness placeholder"),
                        } });
                    }
                }
                b && sparsityInBounds(metamodelica::AsArg::as_arg(&__cref_restCref))?
            }
            _ => true,
        });
        Ok(b)
    }

    /// solved start, solved stop, seed, seed start
    pub type SliceAlias = (i32, i32, metamodelica::Ref<ComponentRef::NFComponentRef>, i32);

    pub type SliceAliases = metamodelica::List<(i32, i32, metamodelica::Ref<ComponentRef::NFComponentRef>, i32)>;

    pub(crate) fn sparsityAddSliceAlias(
        mut eqn: &metamodelica::Ref<Equation::Equation>,
        mut tmp_crefs: &metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>>,
        mut seed_set: metamodelica::Ref<UnorderedSet::UnorderedSet<metamodelica::Ref<ComponentRef::NFComponentRef>>>,
        mut slice_map: metamodelica::Ref<
            UnorderedMap::UnorderedMap<
                metamodelica::Ref<ComponentRef::NFComponentRef>,
                metamodelica::List<(i32, i32, metamodelica::Ref<ComponentRef::NFComponentRef>, i32)>,
            >,
        >,
    ) -> Result<()> {
        let mut lhs: metamodelica::Ref<ComponentRef::NFComponentRef>;
        let mut rhs: metamodelica::Ref<ComponentRef::NFComponentRef>;
        let () = (::match_deref::match_deref! { match eqn {
            Deref @ BEquation::Equation::ARRAY_EQUATION { lhs: Deref @ Expression::CREF { cref: __esc_lhs, .. }, rhs: Deref @ Expression::CREF { cref: __esc_rhs, .. }, .. } => {
                lhs = (*__esc_lhs).clone();
                rhs = (*__esc_rhs).clone();
                sparsityAddSliceAlias2(metamodelica::AsArg::as_arg(&lhs), metamodelica::AsArg::as_arg(&rhs), tmp_crefs, seed_set.clone(), slice_map.clone())?;
                sparsityAddSliceAlias2(metamodelica::AsArg::as_arg(&rhs), metamodelica::AsArg::as_arg(&lhs), tmp_crefs, seed_set, slice_map)?;
                ()
            },
            _ => (),
            _ => unreachable!("match_deref! exhaustiveness placeholder"),
        } });
        Ok(())
    }

    pub(crate) fn sparsityAddSliceAlias2(
        mut solved: &metamodelica::Ref<ComponentRef::NFComponentRef>,
        mut seed: &metamodelica::Ref<ComponentRef::NFComponentRef>,
        mut tmp_crefs: &metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>>,
        mut seed_set: metamodelica::Ref<UnorderedSet::UnorderedSet<metamodelica::Ref<ComponentRef::NFComponentRef>>>,
        mut slice_map: metamodelica::Ref<
            UnorderedMap::UnorderedMap<
                metamodelica::Ref<ComponentRef::NFComponentRef>,
                metamodelica::List<(i32, i32, metamodelica::Ref<ComponentRef::NFComponentRef>, i32)>,
            >,
        >,
    ) -> Result<()> {
        let mut solved_base: metamodelica::Ref<ComponentRef::NFComponentRef> = ComponentRef::stripSubscriptsAll(solved);
        let mut seed_base: metamodelica::Ref<ComponentRef::NFComponentRef> = ComponentRef::stripSubscriptsAll(seed);
        let mut ok1: bool;
        let mut ok2: bool;
        let mut start1: i32;
        let mut stop1: i32;
        let mut start2: i32;
        let mut stop2: i32;
        if !(List::any(
            tmp_crefs,
            &({
                let __pe_b1 = solved_base.clone();
                move |__pe_a0| sparsitySameBase(&__pe_a0, &__pe_b1)
            }),
        )?) || !(UnorderedSet::contains(seed_base.clone(), seed_set)?)
        {
            return Ok(());
        }
        (ok1, start1, stop1) = sparsitySliceBounds(solved)?;
        (ok2, start2, stop2) = sparsitySliceBounds(seed)?;
        if ok1 && ok2 && stop1 - start1 == stop2 - start2 {
            UnorderedMap::add(
                solved_base.clone(),
                metamodelica::cons(
                    (start1, stop1, seed_base, start2),
                    UnorderedMap::getOrDefault(solved_base, slice_map.clone(), metamodelica::nil())?,
                ),
                slice_map,
            )?;
        }
        Ok(())
    }

    pub(crate) fn sparsitySameBase(
        mut cref: &metamodelica::Ref<ComponentRef::NFComponentRef>,
        mut base: &metamodelica::Ref<ComponentRef::NFComponentRef>,
    ) -> Result<bool> {
        let mut b: bool = ComponentRef::isEqual(&(ComponentRef::stripSubscriptsAll(cref)), base)?;
        Ok(b)
    }

    pub(crate) fn sparsitySliceBounds(
        mut cref: &metamodelica::Ref<ComponentRef::NFComponentRef>,
    ) -> Result<(bool, i32, i32)> {
        let mut ok: bool = false;
        let mut start: i32 = 0;
        let mut stop: i32 = 0;
        let mut ty: metamodelica::Ref<Type::NFType> =
            ComponentRef::getSubscriptedType(&(ComponentRef::stripSubscriptsAll(cref)), false)?;
        let mut subs: metamodelica::List<metamodelica::Ref<Subscript::NFSubscript>> =
            ComponentRef::subscriptsAllFlat(cref)?;
        let mut step: i32;
        if Type::dimensionCount(ty.clone()) != 1
            || !(Type::hasKnownSize(ty.clone())?)
            || ((subs).len() as i32) != ((ComponentRef::getSubscripts(cref)).len() as i32)
        {
            return Ok((ok, start, stop));
        }
        (ok, start, stop) = (::match_deref::match_deref! { match &(subs) {
            Deref @ metamodelica::ListNode::Nil => {
                (true, 1, Type::sizeOf(&ty, false)?)
            },
            Deref @ metamodelica::ListNode::Cons { head: Deref @ Subscript::WHOLE, tail: Deref @ metamodelica::ListNode::Nil } => {
                (true, 1, Type::sizeOf(&ty, false)?)
            },
            Deref @ metamodelica::ListNode::Cons { head: Deref @ Subscript::SLICE { slice: range @ Deref @ Expression::RANGE { .. } }, tail: Deref @ metamodelica::ListNode::Nil } if (Expression::isLiteral(metamodelica::AsArg::as_arg(&range))?) => {
                (start, step, stop) = Expression::getIntegerRange(range.clone(), false)?;
                (step == 1, start, stop)
            },
            Deref @ metamodelica::ListNode::Cons { head: Deref @ Subscript::SLICE { slice: range @ Deref @ Expression::ARRAY { .. } }, tail: Deref @ metamodelica::ListNode::Nil } => {
                sparsityConsecutive(&(Expression::arrayElementList(metamodelica::AsArg::as_arg(&range))?))?
            },
            Deref @ metamodelica::ListNode::Cons { head: Deref @ Subscript::EXPANDED_SLICE { indices }, tail: Deref @ metamodelica::ListNode::Nil } => {
                sparsityConsecutive(&(({
            let mut __acc: metamodelica::List<metamodelica::Ref<Expression::NFExpression>> = metamodelica::nil();
            for mut sub in (indices.clone()).into_iter().cloned() {
                let __x = Subscript::toExp(&(sub.clone()))?;
                __acc = cons(__x, __acc);
            }
            __acc.reverse()
        })))?
            },
            _ => {
                (false, 0, 0)
            },
            _ => unreachable!("match_deref! exhaustiveness placeholder"),
        } });
        Ok((ok, start, stop))
    }

    pub(crate) fn sparsityConsecutive(
        mut indices: &metamodelica::List<metamodelica::Ref<Expression::NFExpression>>,
    ) -> Result<(bool, i32, i32)> {
        let mut ok: bool = true;
        let mut start: i32 = 0;
        let mut stop: i32 = 0;
        for mut index in &**indices {
            if !(Expression::isInteger(metamodelica::AsArg::as_arg(&index))) {
                ok = false;
                return Ok((ok, start, stop));
            }
            if stop == 0 {
                start = Expression::integerValue(index.clone())?;
            } else if Expression::integerValue(index.clone())? != stop + 1 {
                ok = false;
                return Ok((ok, start, stop));
            }
            stop = Expression::integerValue(index.clone())?;
        }
        ok = stop > 0;
        Ok((ok, start, stop))
    }

    pub(crate) fn sparsitySliceAliasDeps(
        mut cref: &metamodelica::Ref<ComponentRef::NFComponentRef>,
        mut slice_map: metamodelica::Ref<
            UnorderedMap::UnorderedMap<
                metamodelica::Ref<ComponentRef::NFComponentRef>,
                metamodelica::List<(i32, i32, metamodelica::Ref<ComponentRef::NFComponentRef>, i32)>,
            >,
        >,
    ) -> Result<metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>>> {
        let mut deps: metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>> = metamodelica::nil();
        let mut aliases: metamodelica::List<(i32, i32, metamodelica::Ref<ComponentRef::NFComponentRef>, i32)>;
        let mut start1: i32;
        let mut stop1: i32;
        let mut start2: i32;
        let mut value: i32;
        let mut seed: metamodelica::Ref<ComponentRef::NFComponentRef>;
        let mut index: metamodelica::Ref<Expression::NFExpression>;
        aliases = UnorderedMap::getOrDefault(ComponentRef::stripSubscriptsAll(cref), slice_map, metamodelica::nil())?;
        if (aliases).is_empty()
            || ((ComponentRef::subscriptsAllFlat(cref)?).len() as i32)
                != ((ComponentRef::getSubscripts(cref)).len() as i32)
        {
            return Ok(deps);
        }
        let _ = (::match_deref::match_deref! { match &(ComponentRef::getSubscripts(cref)) {
            Deref @ metamodelica::ListNode::Cons { head: Deref @ Subscript::INDEX { index: __esc_index }, tail: Deref @ metamodelica::ListNode::Nil } => {
                index = (*__esc_index).clone();
                for mut alias in &*aliases {
                    (start1, stop1, seed, start2) = alias.clone();
                    if Expression::isInteger(metamodelica::AsArg::as_arg(&index)) {
                        value = Expression::integerValue(index.clone())?;
                        if value >= start1 && value <= stop1 {
                            deps = metamodelica::cons(ComponentRef::setSubscripts(list![metamodelica::Ref::new(Subscript::NFSubscript::INDEX { index: metamodelica::Ref::new(Expression::NFExpression::INTEGER { value: value - start1 + start2 }) })], seed)?, deps);
                        }
                    } else {
                        deps = metamodelica::cons(ComponentRef::setSubscripts(list![metamodelica::Ref::new(Subscript::NFSubscript::INDEX { index: SimplifyExp::simplify(metamodelica::Ref::new(Expression::NFExpression::BINARY { exp1: index.clone(), operator: Operator::makeAdd(openmodelica_nf_frontend::NFType::interned_INTEGER()), exp2: metamodelica::Ref::new(Expression::NFExpression::INTEGER { value: start2 - start1 }) }), false)? })], seed)?, deps);
                    }
                }
                ()
            },
            _ => (),
            _ => unreachable!("match_deref! exhaustiveness placeholder"),
        } });
        Ok(deps)
    }

    pub(crate) fn sparsityExpandForeignIterators(
        mut cref: metamodelica::Ref<ComponentRef::NFComponentRef>,
        mut own_iters: metamodelica::Ref<UnorderedSet::UnorderedSet<metamodelica::Ref<ComponentRef::NFComponentRef>>>,
        mut seed_elements: metamodelica::Ref<
            UnorderedMap::UnorderedMap<
                metamodelica::Ref<ComponentRef::NFComponentRef>,
                metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>>,
            >,
        >,
    ) -> Result<metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>>> {
        let mut crefs: metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>> = list![cref.clone()];
        let mut base: metamodelica::Ref<ComponentRef::NFComponentRef>;
        let mut ty: metamodelica::Ref<Type::NFType>;
        if !(List::all(
            &(ComponentRef::subscriptsAllFlat(&cref)?),
            &({
                let __pe_b1 = own_iters;
                move |__pe_a0| sparsityIsOwnSubscript(&__pe_a0, __pe_b1.clone())
            }),
        )?) {
            base = ComponentRef::stripSubscriptsAll(&cref);
            if UnorderedMap::contains(base.clone(), seed_elements.clone())? {
                crefs = UnorderedMap::getSafe(
                    base,
                    seed_elements,
                    metamodelica::sourceInfo!("NBackEnd/Util/NBAdjacency.mo"),
                )?;
            } else {
                ty = ComponentRef::getSubscriptedType(&base, false)?;
                crefs = if (Type::isArray(&ty) && Type::sizeOf(&ty, false)? <= 1024) {
                    ComponentRef::scalarizeAll(base, false)?
                } else {
                    list![base]
                };
            }
        }
        Ok(crefs)
    }

    pub(crate) fn sparsityIsOwnSubscript(
        mut sub: &metamodelica::Ref<Subscript::NFSubscript>,
        mut own_iters: metamodelica::Ref<UnorderedSet::UnorderedSet<metamodelica::Ref<ComponentRef::NFComponentRef>>>,
    ) -> Result<bool> {
        let mut b: bool = Subscript::isLiteral(sub)?
            || Subscript::isWhole(sub)
            || Subscript::isSliced(sub)
            || UnorderedSet::all(
                Expression::extractCrefs(Subscript::toExp(sub)?)?,
                &({
                    let __pe_b1 = own_iters.clone();
                    move |__pe_a0| UnorderedSet::contains(__pe_a0, __pe_b1.clone())
                }),
            )?;
        Ok(b)
    }

    pub(crate) fn sparsityAddInner(
        mut cref: metamodelica::Ref<ComponentRef::NFComponentRef>,
        mut deps: metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>>,
        mut inner_map: metamodelica::Ref<
            UnorderedMap::UnorderedMap<
                metamodelica::Ref<ComponentRef::NFComponentRef>,
                metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>>,
            >,
        >,
    ) -> Result<()> {
        let mut stripped: metamodelica::Ref<ComponentRef::NFComponentRef> = ComponentRef::stripSubscriptsAll(&cref);
        UnorderedMap::add(
            cref.clone(),
            UnorderedSet::unique_list(
                listAppend(
                    deps.clone(),
                    UnorderedMap::getOrDefault(cref.clone(), inner_map.clone(), metamodelica::nil())?,
                ),
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
            )?,
            inner_map.clone(),
        )?;
        if !(ComponentRef::isEqual(&stripped, &cref)?) {
            UnorderedMap::add(
                stripped.clone(),
                UnorderedSet::unique_list(
                    listAppend(
                        deps,
                        UnorderedMap::getOrDefault(stripped, inner_map.clone(), metamodelica::nil())?,
                    ),
                    (std::sync::Arc::new(move |__a0: metamodelica::Ref<ComponentRef::NFComponentRef>| {
                        ComponentRef::hash(&__a0)
                    })
                        as std::sync::Arc<
                            dyn ::std::ops::Fn(metamodelica::Ref<ComponentRef::NFComponentRef>) -> Result<i32>
                                + 'static,
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
                )?,
                inner_map,
            )?;
        }
        Ok(())
    }

    pub(crate) fn upgrade(
        mut adj: metamodelica::Ref<Matrix>,
        mut full: &metamodelica::Ref<Matrix>,
        mut vars_map: metamodelica::Ref<
            UnorderedMap::UnorderedMap<metamodelica::Ref<ComponentRef::NFComponentRef>, i32>,
        >,
        mut eqns_map: metamodelica::Ref<
            UnorderedMap::UnorderedMap<metamodelica::Ref<ComponentRef::NFComponentRef>, i32>,
        >,
        mut eqns: &metamodelica::Ref<EquationPointers::EquationPointers>,
        mut st: MatrixStrictness,
        mut iter: &metamodelica::Ref<Iterator::Iterator>,
    ) -> Result<metamodelica::Ref<Matrix>> {
        let mut adj: metamodelica::Ref<Matrix> = adj;
        if Flags::isSet(Flags::BLT_MATRIX_DUMP.clone())? {
            metamodelica::print({
                let mut __mm_s = String::new();
                __mm_s.push_str(&*StringUtil::headline_1(
                    &({
                        let mut __mm_s = String::new();
                        __mm_s.push_str(&*literal!("Upgrading from ["));
                        __mm_s.push_str(&*strictnessString(getStrictness(&adj)?));
                        __mm_s.push_str(&*literal!("] to ["));
                        __mm_s.push_str(&*strictnessString(st));
                        __mm_s.push_str(&*literal!("]"));
                        ArcStr::from(__mm_s)
                    }),
                )?);
                __mm_s.push_str(&*literal!("\n"));
                ArcStr::from(__mm_s)
            });
        }
        adj = (match &**full {
            EMPTY { .. } => metamodelica::Ref::new(Matrix::EMPTY { st: st }),
            FULL {
                mapping: __full_mapping,
                ..
            } => {
                let mut min: i32;
                let mut max: i32;
                let mut builder: IntMatrix::Builder;
                let mut indices: metamodelica::List<i32>;
                if isEmpty(&adj) {
                    min = -1;
                    adj = initialize(__full_mapping.clone(), st);
                } else {
                    min = Solvability::rank(&(Solvability::fromStrictness(getStrictness(&adj)?)))?;
                }
                max = Solvability::rank(&(Solvability::fromStrictness(st)))?;
                adj = (match &*adj {
                    FINAL { .. } => {
                        let mut result: metamodelica::Ref<Matrix>;
                        let mut filtered: metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>>;
                        let mut occ: metamodelica::Array<
                            metamodelica::Ref<
                                UnorderedSet::UnorderedSet<metamodelica::Ref<ComponentRef::NFComponentRef>>,
                            >,
                        >;
                        let mut dep: metamodelica::Array<
                            metamodelica::Ref<
                                UnorderedMap::UnorderedMap<
                                    metamodelica::Ref<ComponentRef::NFComponentRef>,
                                    metamodelica::Ref<Dependency::Dependency>,
                                >,
                            >,
                        >;
                        let mut sol: metamodelica::Array<
                            metamodelica::Ref<
                                UnorderedMap::UnorderedMap<
                                    metamodelica::Ref<ComponentRef::NFComponentRef>,
                                    metamodelica::Ref<Solvability::Solvability>,
                                >,
                            >,
                        >;
                        let mut rep: metamodelica::Array<
                            metamodelica::Ref<
                                UnorderedSet::UnorderedSet<metamodelica::Ref<ComponentRef::NFComponentRef>>,
                            >,
                        >;
                        if max == min {
                            result = adj;
                        } else if max > min {
                            (occ, dep, sol, rep) = (
                                var_field!((**full).occurrences, Matrix::FULL).clone(),
                                var_field!((**full).dependencies, Matrix::FULL).clone(),
                                var_field!((**full).solvabilities, Matrix::FULL).clone(),
                                var_field!((**full).repetitions, Matrix::FULL).clone(),
                            );
                            indices = UnorderedMap::valueList(eqns_map);
                            builder = IntMatrix::toBuilder(
                                &(var_field!((*adj).m, Matrix::FINAL).clone()),
                                estimateEntries(occ.clone(), var_field!((*adj).mapping, Matrix::FINAL), &indices)?,
                                true,
                            )?;
                            for mut index in &*indices {
                                filtered = Solvability::filter(
                                    &(UnorderedSet::toList(
                                        ({
                                            let __elt =
                                                (*metamodelica::index_checked(&occ.borrow(), index.clone())?).clone();
                                            __elt
                                        }),
                                    )),
                                    ({
                                        let __elt =
                                            (*metamodelica::index_checked(&sol.borrow(), index.clone())?).clone();
                                        __elt
                                    }),
                                    vars_map.clone(),
                                    min + 1,
                                    max,
                                )?;
                                upgradeRow(
                                    BEquation::EquationPointers::getEqnAt(eqns, index.clone())?,
                                    index.clone(),
                                    filtered,
                                    ({
                                        let __elt =
                                            (*metamodelica::index_checked(&dep.borrow(), index.clone())?).clone();
                                        __elt
                                    }),
                                    ({
                                        let __elt =
                                            (*metamodelica::index_checked(&rep.borrow(), index.clone())?).clone();
                                        __elt
                                    }),
                                    vars_map.clone(),
                                    vars_map.clone(),
                                    &builder,
                                    &(var_field!((*adj).mapping, Matrix::FINAL).clone()),
                                    var_field!((*adj).modes, Matrix::FINAL).clone(),
                                    iter,
                                )?;
                            }
                            assign_variant_field!(adj => Matrix::FINAL; m = IntMatrix::fromBuilder(&builder, IntMatrix::rows(var_field!((*adj).m, Matrix::FINAL)))?);
                            assign_variant_field!(adj => Matrix::FINAL; mT = IntMatrix::transpose(&(var_field!((*adj).m, Matrix::FINAL).clone()), metamodelica::arrayLength(var_field!((*adj).mapping, Matrix::FINAL).var_StA.clone()), 0)?);
                            result = adj;
                        } else {
                            if Flags::isSet(Flags::FAILTRACE.clone())? {
                                Error::addCompilerWarning({
                                    let mut __mm_s = String::new();
                                    __mm_s.push_str(&*literal!(
                                        "Invalid matrix upgrade request. Cannot upgrade matrix of type "
                                    ));
                                    __mm_s.push_str(&*Solvability::toString(
                                        &(Solvability::fromStrictness(getStrictness(&adj)?)),
                                    )?);
                                    __mm_s.push_str(&*literal!(" to type "));
                                    __mm_s.push_str(&*Solvability::toString(&(Solvability::fromStrictness(st)))?);
                                    __mm_s.push_str(&*literal!(". The new matrix will be\n                    created from using only the full adjacency matrix."));
                                    ArcStr::from(__mm_s)
                                })?;
                            }
                            result = fullToFinal(full, vars_map, eqns_map, eqns, st, iter)?;
                        }
                        result
                    }
                    EMPTY { .. } => adj,
                    FULL { .. } => {
                        Error::addMessage(
                            Error::INTERNAL_ERROR.clone(),
                            list![{
                                let mut __mm_s = String::new();
                                __mm_s.push_str(&*literal!("NBAdjacency.Matrix.upgrade"));
                                __mm_s.push_str(&*literal!(" failed because of wrong matrix type for the 1st input.\n                Expected: final or empty, Got :"));
                                __mm_s.push_str(&*strictnessString(getStrictness(&adj)?));
                                __mm_s.push_str(&*literal!("."));
                                ArcStr::from(__mm_s)
                            }],
                        )?;
                        return Err("fail");
                    }
                    _ => return Err("match: no arm matched"),
                });
                adj
            }
            _ => {
                Error::addMessage(
                    Error::INTERNAL_ERROR.clone(),
                    list![{
                        let mut __mm_s = String::new();
                        __mm_s.push_str(&*literal!("NBAdjacency.Matrix.upgrade"));
                        __mm_s.push_str(&*literal!(
                            " failed because of wrong matrix type for the 2nd input.\n            Expected: full, Got :"
                        ));
                        __mm_s.push_str(&*strictnessString(getStrictness(full)?));
                        __mm_s.push_str(&*literal!("."));
                        ArcStr::from(__mm_s)
                    }],
                )?;
                return Err("fail");
            }
        });
        if Flags::isSet(Flags::BLT_MATRIX_DUMP.clone())? {
            metamodelica::print({
                let mut __mm_s = String::new();
                __mm_s.push_str(&*toString(&adj, literal!("Final"))?);
                __mm_s.push_str(&*literal!("\n"));
                ArcStr::from(__mm_s)
            });
        }
        Ok(adj)
    }

    pub(crate) fn equalRows(
        mut seed_eqns: &metamodelica::Ref<EquationPointers::EquationPointers>,
        mut seed_vars: &metamodelica::Ref<VariablePointers::VariablePointers>,
        mut eqns: &metamodelica::Ref<EquationPointers::EquationPointers>,
        mut vars: &metamodelica::Ref<VariablePointers::VariablePointers>,
    ) -> Result<metamodelica::Array<i32>> {
        let mut seed_index: metamodelica::Array<i32> = arrayCreate(BEquation::EquationPointers::lastUsedIndex(eqns), 0);
        let mut n: i32 = BVariable::VariablePointers::size(vars);
        let mut i: i32;
        let mut seed_eqn: Pointer::Pointer<metamodelica::Ref<Equation::Equation>>;
        if n != BVariable::VariablePointers::size(seed_vars)
            || n != BVariable::VariablePointers::lastUsedIndex(vars)
            || n != BVariable::VariablePointers::lastUsedIndex(seed_vars)
        {
            return Ok(seed_index);
        }
        for mut j in 1..=n {
            if !(Pointer::referenceEq(
                &(BVariable::VariablePointers::getVarAt(vars, j)?),
                &(BVariable::VariablePointers::getVarAt(seed_vars, j)?),
            )) {
                return Ok(seed_index);
            }
        }
        for mut eqn in &*BEquation::EquationPointers::toList(eqns)? {
            i = BEquation::EquationPointers::getEqnIndex(seed_eqns, BEquation::Equation::getEqnName(eqn.clone())?)?;
            if i > 0 {
                seed_eqn = BEquation::EquationPointers::getEqnAt(seed_eqns, i)?;
                if BEquation::Equation::isEqual(&(Pointer::access(eqn.clone())), &(Pointer::access(seed_eqn)))? {
                    {
                        let __cell0 = i;
                        let __idx0 = BEquation::EquationPointers::getEqnIndex(
                            eqns,
                            BEquation::Equation::getEqnName(eqn.clone())?,
                        )?;
                        *metamodelica::index_mut_checked(&mut seed_index.clone().borrow_mut(), __idx0)? = __cell0;
                    }
                }
            }
        }
        Ok(seed_index)
    }

    pub(crate) fn upgradeFrom(
        mut adj: metamodelica::Ref<Matrix>,
        mut full: &metamodelica::Ref<Matrix>,
        mut vars_map: metamodelica::Ref<
            UnorderedMap::UnorderedMap<metamodelica::Ref<ComponentRef::NFComponentRef>, i32>,
        >,
        mut eqns_map: metamodelica::Ref<
            UnorderedMap::UnorderedMap<metamodelica::Ref<ComponentRef::NFComponentRef>, i32>,
        >,
        mut eqns: &metamodelica::Ref<EquationPointers::EquationPointers>,
        mut st: MatrixStrictness,
        mut seed: &metamodelica::Ref<Matrix>,
        mut seed_index: metamodelica::Array<i32>,
    ) -> Result<metamodelica::Ref<Matrix>> {
        let __ab_seed_index = seed_index.borrow();
        let mut adj: metamodelica::Ref<Matrix> = adj;
        let mut min: i32;
        let mut max: i32;
        let mut rows: i32;
        let mut eqn_start: i32;
        let mut eqn_size: i32;
        let mut seed_start: i32;
        let mut own_entries: i32 = 0;
        let mut shared_entries: i32 = 0;
        let mut fresh: bool = isEmpty(&adj);
        let mut m: metamodelica::Ref<IntMatrix::IntMatrix>;
        let mut own_m: metamodelica::Ref<IntMatrix::IntMatrix>;
        let mut builder: IntMatrix::Builder;
        let mut own: metamodelica::List<i32> = metamodelica::nil();
        let mut filtered: metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>>;
        let mut data: metamodelica::Array<i32>;
        let mut aux: metamodelica::Array<i32>;
        let mut seed_data: metamodelica::Array<i32>;
        let mut seed_aux: metamodelica::Array<i32>;
        max = Solvability::rank(&(Solvability::fromStrictness(st)))?;
        min = if (fresh) {
            -1
        } else {
            Solvability::rank(&(Solvability::fromStrictness(getStrictness(&adj)?)))?
        };
        adj = (::match_deref::match_deref! { match (full, seed) {
            (Deref @ FULL { .. }, Deref @ FINAL { .. }) if (min < max) => {
                if fresh {
                    adj = initialize(var_field!((**full).mapping, Matrix::FULL).clone(), st);
                }
                adj = (match &*adj {
            FINAL { .. } => {
                if fresh {
                    assign_variant_field!(adj => Matrix::FINAL; modes = Vector::copy(var_field!((**seed).modes, Matrix::FINAL).clone()));
                }
                rows = IntMatrix::rows(var_field!((*adj).m, Matrix::FINAL));
                for mut index in &*UnorderedMap::valueList(eqns_map.clone()) {
                    (eqn_start, eqn_size) = ({let __elt = (*metamodelica::index_checked(&var_field!((*adj).mapping, Matrix::FINAL).eqn_AtS.borrow(), index.clone())?).clone(); __elt});
                    if (*metamodelica::index_checked(&__ab_seed_index, index.clone())?).clone() > 0 {
                        (seed_start, _) = ({let __elt = (*metamodelica::index_checked(&var_field!((**seed).mapping, Matrix::FINAL).eqn_AtS.borrow(), (*metamodelica::index_checked(&__ab_seed_index, index.clone())?).clone())?).clone(); __elt});
                        for mut k in 0..=eqn_size - 1 {
                            shared_entries = shared_entries + ({let __elt = (*metamodelica::index_checked(&var_field!((**seed).m, Matrix::FINAL).len.borrow(), seed_start + k)?).clone(); __elt});
                        }
                    } else {
                        own = metamodelica::cons(index.clone(), own);
                        for mut k in 0..=eqn_size - 1 {
                            own_entries = own_entries + ({let __elt = (*metamodelica::index_checked(&var_field!((*adj).m, Matrix::FINAL).len.borrow(), eqn_start + k)?).clone(); __elt});
                        }
                        own_entries = own_entries + UnorderedSet::size(({let __elt = (*metamodelica::index_checked(&var_field!((**full).occurrences, Matrix::FULL).borrow(), index.clone())?).clone(); __elt})) * eqn_size;
                    }
                }
                own = own.reverse();
                builder = IntMatrix::newBuilder(own_entries, true);
                data = IntMatrix::entries(var_field!((*adj).m, Matrix::FINAL));
                aux = IntMatrix::payload(var_field!((*adj).m, Matrix::FINAL));
                for mut index in &*own {
                    (eqn_start, eqn_size) = ({let __elt = (*metamodelica::index_checked(&var_field!((*adj).mapping, Matrix::FINAL).eqn_AtS.borrow(), index.clone())?).clone(); __elt});
                    for mut k in ({let __s=eqn_size - 1; let __e=0; (0i32..).map(move |__k| __s + __k * (-1)).take_while(move |&__v| __v >= __e)}) {
                        let __range0 = ({let __s=({let __elt = (*metamodelica::index_checked(&var_field!((*adj).m, Matrix::FINAL).start.borrow(), eqn_start + k)?).clone(); __elt}) + ({let __elt = (*metamodelica::index_checked(&var_field!((*adj).m, Matrix::FINAL).len.borrow(), eqn_start + k)?).clone(); __elt}) - 1; let __e=({let __elt = (*metamodelica::index_checked(&var_field!((*adj).m, Matrix::FINAL).start.borrow(), eqn_start + k)?).clone(); __elt}); (0i32..).map(move |__k| __s + __k * (-1)).take_while(move |&__v| __v >= __e)});
                        for mut p in __range0 {
                            IntMatrix::builderAddAux(&builder, eqn_start + k, ({let __elt = (*metamodelica::index_checked(&data.borrow(), p)?).clone(); __elt}), if (metamodelica::arrayLength(aux.clone()) > 0) {({let __elt = (*metamodelica::index_checked(&aux.borrow(), p)?).clone(); __elt})} else {0});
                        }
                    }
                }
                for mut index in &*own {
                    filtered = Solvability::filter(&(UnorderedSet::toList(({let __elt = (*metamodelica::index_checked(&var_field!((**full).occurrences, Matrix::FULL).borrow(), index.clone())?).clone(); __elt}))), ({let __elt = (*metamodelica::index_checked(&var_field!((**full).solvabilities, Matrix::FULL).borrow(), index.clone())?).clone(); __elt}), vars_map.clone(), min + 1, max)?;
                    upgradeRow(BEquation::EquationPointers::getEqnAt(eqns, index.clone())?, index.clone(), filtered, ({let __elt = (*metamodelica::index_checked(&var_field!((**full).dependencies, Matrix::FULL).borrow(), index.clone())?).clone(); __elt}), ({let __elt = (*metamodelica::index_checked(&var_field!((**full).repetitions, Matrix::FULL).borrow(), index.clone())?).clone(); __elt}), vars_map.clone(), vars_map.clone(), &builder, &(var_field!((*adj).mapping, Matrix::FINAL).clone()), var_field!((*adj).modes, Matrix::FINAL).clone(), &(crate::NBEquation::Iterator::interned_EMPTY()))?;
                }
                own_m = IntMatrix::fromBuilder(&builder, rows)?;
                m = IntMatrix::new(rows);
                IntMatrix::reserveData(&m, shared_entries + IntMatrix::nonZeroCount(&own_m), true);
                data = IntMatrix::entries(&own_m);
                aux = IntMatrix::payload(&own_m);
                seed_data = IntMatrix::entries(var_field!((**seed).m, Matrix::FINAL));
                seed_aux = IntMatrix::payload(var_field!((**seed).m, Matrix::FINAL));
                if metamodelica::arrayLength(seed_aux.clone()) == 0 {
                    seed_aux = arrayCreate(metamodelica::arrayLength(seed_data.clone()), 0);
                }
                for mut index in &*UnorderedMap::valueList(eqns_map) {
                    (eqn_start, eqn_size) = ({let __elt = (*metamodelica::index_checked(&var_field!((*adj).mapping, Matrix::FINAL).eqn_AtS.borrow(), index.clone())?).clone(); __elt});
                    if (*metamodelica::index_checked(&__ab_seed_index, index.clone())?).clone() > 0 {
                        (seed_start, _) = ({let __elt = (*metamodelica::index_checked(&var_field!((**seed).mapping, Matrix::FINAL).eqn_AtS.borrow(), (*metamodelica::index_checked(&__ab_seed_index, index.clone())?).clone())?).clone(); __elt});
                        for mut k in 0..=eqn_size - 1 {
                            IntMatrix::copyRow(var_field!((**seed).m, Matrix::FINAL), seed_start + k, seed_data.clone(), seed_aux.clone(), &m, eqn_start + k)?;
                        }
                    } else {
                        for mut k in 0..=eqn_size - 1 {
                            IntMatrix::copyRow(&own_m, eqn_start + k, data.clone(), aux.clone(), &m, eqn_start + k)?;
                        }
                    }
                }
                assign_variant_field!(adj => Matrix::FINAL;
                    m = m.clone(),
                    mT = IntMatrix::transpose(&m, metamodelica::arrayLength(var_field!((*adj).mapping, Matrix::FINAL).var_StA.clone()), 0)?
                );
                adj
            },
            _ => adj,
        });
                adj
            },
            _ => upgrade(adj, full, vars_map, eqns_map, eqns, st, &(crate::NBEquation::Iterator::interned_EMPTY()))?,
            _ => unreachable!("match_deref! exhaustiveness placeholder"),
        } });
        Ok(adj)
    }

    pub(crate) fn expand(
        mut adj: metamodelica::Ref<Matrix>,
        mut full: metamodelica::Ref<Matrix>,
        mut vo: metamodelica::Ref<UnorderedMap::UnorderedMap<metamodelica::Ref<ComponentRef::NFComponentRef>, i32>>,
        mut vn: metamodelica::Ref<UnorderedMap::UnorderedMap<metamodelica::Ref<ComponentRef::NFComponentRef>, i32>>,
        mut eo: metamodelica::Ref<UnorderedMap::UnorderedMap<metamodelica::Ref<ComponentRef::NFComponentRef>, i32>>,
        mut en: metamodelica::Ref<UnorderedMap::UnorderedMap<metamodelica::Ref<ComponentRef::NFComponentRef>, i32>>,
        mut vars: &metamodelica::Ref<VariablePointers::VariablePointers>,
        mut eqns: &metamodelica::Ref<EquationPointers::EquationPointers>,
        mut kind: Partition::Kind,
    ) -> Result<(metamodelica::Ref<Matrix>, metamodelica::Ref<Matrix>)> {
        let mut adj: metamodelica::Ref<Matrix> = adj;
        let mut full: metamodelica::Ref<Matrix> = full;
        let mut size_vo: i32;
        let mut size_vn: i32;
        let mut size_eo: i32;
        let mut size_en: i32;
        if Flags::isSet(Flags::BLT_MATRIX_DUMP.clone())? {
            size_vo = ({
                let mut __acc: i32 = 0;
                for mut var in (UnorderedMap::keyList(vo.clone())).into_iter().cloned() {
                    let __x = ComponentRef::size(&(var.clone()), true, false)?;
                    __acc += __x;
                }
                __acc
            });
            size_vn = ({
                let mut __acc: i32 = 0;
                for mut var in (UnorderedMap::keyList(vn.clone())).into_iter().cloned() {
                    let __x = ComponentRef::size(&(var.clone()), true, false)?;
                    __acc += __x;
                }
                __acc
            }) + size_vo;
            size_eo = ({
                let mut __acc: i32 = 0;
                for mut eqn in (UnorderedMap::keyList(eo.clone())).into_iter().cloned() {
                    let __x = ComponentRef::size(&(eqn.clone()), true, false)?;
                    __acc += __x;
                }
                __acc
            });
            size_en = ({
                let mut __acc: i32 = 0;
                for mut eqn in (UnorderedMap::keyList(en.clone())).into_iter().cloned() {
                    let __x = ComponentRef::size(&(eqn.clone()), true, false)?;
                    __acc += __x;
                }
                __acc
            }) + size_eo;
            metamodelica::print({
                let mut __mm_s = String::new();
                __mm_s.push_str(&*StringUtil::headline_1(
                    &({
                        let mut __mm_s = String::new();
                        __mm_s.push_str(&*literal!("Expanding from size [vars: "));
                        __mm_s.push_str(&*intString(size_vo));
                        __mm_s.push_str(&*literal!("| eqns: "));
                        __mm_s.push_str(&*intString(size_eo));
                        __mm_s.push_str(&*literal!("] to [vars: "));
                        __mm_s.push_str(&*intString(size_vn));
                        __mm_s.push_str(&*literal!("| eqns: "));
                        __mm_s.push_str(&*intString(size_en));
                        __mm_s.push_str(&*literal!("]"));
                        ArcStr::from(__mm_s)
                    }),
                )?);
                __mm_s.push_str(&*literal!("\n"));
                ArcStr::from(__mm_s)
            });
        }
        full = (match &*full {
            FULL { .. }
                if (BEquation::EquationPointers::size(eqns)
                    > metamodelica::arrayLength(var_field!((*full).equation_names, Matrix::FULL).clone())) =>
            {
                expandFull(
                    full.clone(),
                    vo.clone(),
                    vn.clone(),
                    eo.clone(),
                    en.clone(),
                    vars,
                    eqns,
                    kind,
                )?
            }
            _ => full.clone(),
        });
        adj = ({
            let mut v: metamodelica::Ref<
                UnorderedMap::UnorderedMap<metamodelica::Ref<ComponentRef::NFComponentRef>, i32>,
            > = vo.clone();
            (::match_deref::match_deref! { match &((adj.clone(), full.clone())) {
                (Deref @ EMPTY { .. }, Deref @ FULL { .. }) => {
                    let mut new: metamodelica::Ref<Matrix>;
                    new = initialize(var_field!((*full).mapping, Matrix::FULL).clone(), var_field!((*adj).st, Matrix::EMPTY).clone());
                    if !(isEmpty(&new)) {
                        (new, _) = expand(new, full.clone(), vo, vn, eo, en, vars, eqns, kind)?;
                    }
                    new
                },
                (Deref @ FINAL { .. }, Deref @ FULL { .. }) => {
                    let mut rank: i32;
                    let mut max_index_var: i32;
                    let mut eqn_scalar_size: i32;
                    let mut filtered: metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>>;
                    let mut eo_lst: metamodelica::List<i32>;
                    let mut en_lst: metamodelica::List<i32>;
                    let mut builder: IntMatrix::Builder;
                    eqn_scalar_size = BEquation::EquationPointers::scalarSize(eqns, true)?;
                    assign_variant_field!(adj => Matrix::FINAL; mapping = var_field!((*full).mapping, Matrix::FULL).clone());
                    eo_lst = UnorderedMap::valueList(eo);
                    en_lst = UnorderedMap::valueList(en.clone());
                    builder = IntMatrix::toBuilder(&(var_field!((*adj).m, Matrix::FINAL).clone()), estimateEntries(var_field!((*full).occurrences, Matrix::FULL).clone(), var_field!((*adj).mapping, Matrix::FINAL), &eo_lst)? + estimateEntries(var_field!((*full).occurrences, Matrix::FULL).clone(), var_field!((*adj).mapping, Matrix::FINAL), &en_lst)?, true)?;
                    rank = Solvability::rank(&(Solvability::fromStrictness(getStrictness(&adj)?)))?;
                    if !(UnorderedMap::isEmpty(vn.clone())) && !(UnorderedMap::isEmpty(en.clone())) {
                        v = UnorderedMap::merge(v, vn.clone(), metamodelica::sourceInfo!("NBackEnd/Util/NBAdjacency.mo"))?;
                    }
                    if !(UnorderedMap::isEmpty(vn.clone())) {
                        for mut e in &*eo_lst {
                            filtered = Solvability::filter(&(UnorderedSet::toList(({let __elt = (*metamodelica::index_checked(&var_field!((*full).occurrences, Matrix::FULL).borrow(), e.clone())?).clone(); __elt}))), ({let __elt = (*metamodelica::index_checked(&var_field!((*full).solvabilities, Matrix::FULL).borrow(), e.clone())?).clone(); __elt}), vn.clone(), 0, rank)?;
                            upgradeRow(BEquation::EquationPointers::getEqnAt(eqns, e.clone())?, e.clone(), filtered, ({let __elt = (*metamodelica::index_checked(&var_field!((*full).dependencies, Matrix::FULL).borrow(), e.clone())?).clone(); __elt}), ({let __elt = (*metamodelica::index_checked(&var_field!((*full).repetitions, Matrix::FULL).borrow(), e.clone())?).clone(); __elt}), vn.clone(), vars.map.clone(), &builder, &(var_field!((*adj).mapping, Matrix::FINAL).clone()), var_field!((*adj).modes, Matrix::FINAL).clone(), &(crate::NBEquation::Iterator::interned_EMPTY()))?;
                        }
                    }
                    if !(UnorderedMap::isEmpty(en)) {
                        for mut e in &*en_lst {
                            filtered = Solvability::filter(&(UnorderedSet::toList(({let __elt = (*metamodelica::index_checked(&var_field!((*full).occurrences, Matrix::FULL).borrow(), e.clone())?).clone(); __elt}))), ({let __elt = (*metamodelica::index_checked(&var_field!((*full).solvabilities, Matrix::FULL).borrow(), e.clone())?).clone(); __elt}), v.clone(), 0, rank)?;
                            upgradeRow(BEquation::EquationPointers::getEqnAt(eqns, e.clone())?, e.clone(), filtered, ({let __elt = (*metamodelica::index_checked(&var_field!((*full).dependencies, Matrix::FULL).borrow(), e.clone())?).clone(); __elt}), ({let __elt = (*metamodelica::index_checked(&var_field!((*full).repetitions, Matrix::FULL).borrow(), e.clone())?).clone(); __elt}), v.clone(), vars.map.clone(), &builder, &(var_field!((*adj).mapping, Matrix::FINAL).clone()), var_field!((*adj).modes, Matrix::FINAL).clone(), &(crate::NBEquation::Iterator::interned_EMPTY()))?;
                        }
                    }
                    if UnorderedMap::isEmpty(vo.clone()) && UnorderedMap::isEmpty(vn.clone()) {
                        max_index_var = 0;
                    } else {
                        max_index_var = intMax(({
                let mut __acc: Option<i32> = None;
                for mut i in (UnorderedMap::valueList(vo)).into_iter().cloned() {
                    let __x = i.clone();
                    __acc = Some(match __acc { None => __x, Some(__cur) => if __x > __cur { __x } else { __cur } });
                }
                __acc.unwrap_or((-i32::MAX))
            }), ({
                let mut __acc: Option<i32> = None;
                for mut i in (UnorderedMap::valueList(vn)).into_iter().cloned() {
                    let __x = i.clone();
                    __acc = Some(match __acc { None => __x, Some(__cur) => if __x > __cur { __x } else { __cur } });
                }
                __acc.unwrap_or((-i32::MAX))
            }));
                    }
                    assign_variant_field!(adj => Matrix::FINAL; m = IntMatrix::fromBuilder(&builder, eqn_scalar_size)?);
                    assign_variant_field!(adj => Matrix::FINAL; mT = IntMatrix::transpose(var_field!((*adj).m, Matrix::FINAL), BVariable::VariablePointers::scalarSize(vars, true)?, 0)?);
                    adj
                },
                (Deref @ FINAL { .. }, _) => {
                    Error::addMessage(Error::INTERNAL_ERROR.clone(), list![{ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBAdjacency.Matrix.expand")); __mm_s.push_str(&*literal!(" failed because the full matrix expected to contain all information is instead of type ")); __mm_s.push_str(&*strictnessString(getStrictness(&full)?)); __mm_s.push_str(&*literal!(".")); ArcStr::from(__mm_s) }])?;
                    return Err("fail")
                },
                (_, Deref @ FULL { .. }) => {
                    Error::addMessage(Error::INTERNAL_ERROR.clone(), list![{ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBAdjacency.Matrix.expand")); __mm_s.push_str(&*literal!(" failed because the matrix to be expanded of type ")); __mm_s.push_str(&*strictnessString(getStrictness(&adj)?)); __mm_s.push_str(&*literal!(" should be of type final.")); ArcStr::from(__mm_s) }])?;
                    return Err("fail")
                },
                _ => {
                    Error::addMessage(Error::INTERNAL_ERROR.clone(), list![{ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBAdjacency.Matrix.expand")); __mm_s.push_str(&*literal!(" expected types final and full, got types ")); __mm_s.push_str(&*strictnessString(getStrictness(&adj)?)); __mm_s.push_str(&*literal!(" and ")); __mm_s.push_str(&*strictnessString(getStrictness(&full)?)); __mm_s.push_str(&*literal!(".")); ArcStr::from(__mm_s) }])?;
                    return Err("fail")
                },
                _ => unreachable!("match_deref! exhaustiveness placeholder"),
            } })
        });
        if Flags::isSet(Flags::BLT_MATRIX_DUMP.clone())? {
            metamodelica::print({
                let mut __mm_s = String::new();
                __mm_s.push_str(&*toString(&adj, literal!("Expanded Final"))?);
                __mm_s.push_str(&*literal!("\n"));
                ArcStr::from(__mm_s)
            });
        }
        Ok((adj, full))
    }

    pub(crate) fn expandFull(
        mut full: metamodelica::Ref<Matrix>,
        mut vo: metamodelica::Ref<UnorderedMap::UnorderedMap<metamodelica::Ref<ComponentRef::NFComponentRef>, i32>>,
        mut vn: metamodelica::Ref<UnorderedMap::UnorderedMap<metamodelica::Ref<ComponentRef::NFComponentRef>, i32>>,
        mut eo: metamodelica::Ref<UnorderedMap::UnorderedMap<metamodelica::Ref<ComponentRef::NFComponentRef>, i32>>,
        mut en: metamodelica::Ref<UnorderedMap::UnorderedMap<metamodelica::Ref<ComponentRef::NFComponentRef>, i32>>,
        mut vars: &metamodelica::Ref<VariablePointers::VariablePointers>,
        mut eqns: &metamodelica::Ref<EquationPointers::EquationPointers>,
        mut kind: Partition::Kind,
    ) -> Result<metamodelica::Ref<Matrix>> {
        let mut full: metamodelica::Ref<Matrix> = full;
        full = ({
            let mut new_vars: metamodelica::List<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>> = ({
                let mut __acc: metamodelica::List<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>> =
                    metamodelica::nil();
                for mut idx in (UnorderedMap::valueList(vn.clone())).into_iter().cloned() {
                    let __x = BVariable::VariablePointers::getVarAt(vars, idx.clone())?;
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            });
            let mut new_eqns: metamodelica::List<Pointer::Pointer<metamodelica::Ref<Equation::Equation>>> = ({
                let mut __acc: metamodelica::List<Pointer::Pointer<metamodelica::Ref<Equation::Equation>>> =
                    metamodelica::nil();
                for mut idx in (UnorderedMap::valueList(en.clone())).into_iter().cloned() {
                    let __x = BEquation::EquationPointers::getEqnAt(eqns, idx.clone())?;
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            });
            let mut size: i32 = BEquation::EquationPointers::size(eqns);
            (match &*full {
                FULL {
                    mapping: __full_mapping,
                    ..
                } => {
                    let mut index: i32;
                    let mut eqn_ptr: Pointer::Pointer<metamodelica::Ref<Equation::Equation>>;
                    let mut occ_set: metamodelica::Ref<
                        UnorderedSet::UnorderedSet<metamodelica::Ref<ComponentRef::NFComponentRef>>,
                    >;
                    full = metamodelica::Ref::new(Matrix::FULL { equation_names: Array::expandToSize(size, var_field!((*full).equation_names, Matrix::FULL).clone(), openmodelica_nf_frontend::NFComponentRef::interned_EMPTY())?, occurrences: Array::expandToSize(size, var_field!((*full).occurrences, Matrix::FULL).clone(), UnorderedSet::new((std::sync::Arc::new(move |__a0: metamodelica::Ref<ComponentRef::NFComponentRef>| ComponentRef::hash(&__a0)) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<ComponentRef::NFComponentRef>) -> Result<i32> + 'static>), (std::sync::Arc::new(move |__a0: metamodelica::Ref<ComponentRef::NFComponentRef>, __a1: metamodelica::Ref<ComponentRef::NFComponentRef>| ComponentRef::isEqual(&__a0, &__a1)) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<ComponentRef::NFComponentRef>, metamodelica::Ref<ComponentRef::NFComponentRef>) -> Result<bool> + 'static>), 13))?, dependencies: Array::expandToSize(size, var_field!((*full).dependencies, Matrix::FULL).clone(), UnorderedMap::new((std::sync::Arc::new(move |__a0: metamodelica::Ref<ComponentRef::NFComponentRef>| ComponentRef::hash(&__a0)) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<ComponentRef::NFComponentRef>) -> Result<i32> + 'static>), (std::sync::Arc::new(move |__a0: metamodelica::Ref<ComponentRef::NFComponentRef>, __a1: metamodelica::Ref<ComponentRef::NFComponentRef>| ComponentRef::isEqual(&__a0, &__a1)) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<ComponentRef::NFComponentRef>, metamodelica::Ref<ComponentRef::NFComponentRef>) -> Result<bool> + 'static>), 1))?, solvabilities: Array::expandToSize(size, var_field!((*full).solvabilities, Matrix::FULL).clone(), UnorderedMap::new((std::sync::Arc::new(move |__a0: metamodelica::Ref<ComponentRef::NFComponentRef>| ComponentRef::hash(&__a0)) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<ComponentRef::NFComponentRef>) -> Result<i32> + 'static>), (std::sync::Arc::new(move |__a0: metamodelica::Ref<ComponentRef::NFComponentRef>, __a1: metamodelica::Ref<ComponentRef::NFComponentRef>| ComponentRef::isEqual(&__a0, &__a1)) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<ComponentRef::NFComponentRef>, metamodelica::Ref<ComponentRef::NFComponentRef>) -> Result<bool> + 'static>), 1))?, repetitions: Array::expandToSize(size, var_field!((*full).repetitions, Matrix::FULL).clone(), UnorderedSet::new((std::sync::Arc::new(move |__a0: metamodelica::Ref<ComponentRef::NFComponentRef>| ComponentRef::hash(&__a0)) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<ComponentRef::NFComponentRef>) -> Result<i32> + 'static>), (std::sync::Arc::new(move |__a0: metamodelica::Ref<ComponentRef::NFComponentRef>, __a1: metamodelica::Ref<ComponentRef::NFComponentRef>| ComponentRef::isEqual(&__a0, &__a1)) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<ComponentRef::NFComponentRef>, metamodelica::Ref<ComponentRef::NFComponentRef>) -> Result<bool> + 'static>), 13))?, mapping: Mapping::expand(__full_mapping.clone(), new_eqns, new_vars)? });
                    if !(UnorderedMap::isEmpty(vn.clone())) {
                        for mut e in &*UnorderedMap::valueList(eo) {
                            eqn_ptr = BEquation::EquationPointers::getEqnAt(eqns, e.clone())?;
                            index = UnorderedMap::getSafe(
                                BEquation::Equation::getEqnName(eqn_ptr.clone())?,
                                eqns.map.clone(),
                                metamodelica::sourceInfo!("NBackEnd/Util/NBAdjacency.mo"),
                            )?;
                            occ_set = collectDependenciesEquation(
                                &(Pointer::access(eqn_ptr)),
                                kind,
                                vn.clone(),
                                ({
                                    let __elt = (*metamodelica::index_checked(
                                        &var_field!((*full).dependencies, Matrix::FULL).borrow(),
                                        index,
                                    )?)
                                    .clone();
                                    __elt
                                }),
                                ({
                                    let __elt = (*metamodelica::index_checked(
                                        &var_field!((*full).solvabilities, Matrix::FULL).borrow(),
                                        index,
                                    )?)
                                    .clone();
                                    __elt
                                }),
                                ({
                                    let __elt = (*metamodelica::index_checked(
                                        &var_field!((*full).repetitions, Matrix::FULL).borrow(),
                                        index,
                                    )?)
                                    .clone();
                                    __elt
                                }),
                            )?;
                            {
                                let __cell0 = UnorderedSet::union(
                                    ({
                                        let __elt = (*metamodelica::index_checked(
                                            &var_field!((*full).occurrences, Matrix::FULL).borrow(),
                                            index,
                                        )?)
                                        .clone();
                                        __elt
                                    }),
                                    occ_set,
                                )?;
                                let __idx0 = index;
                                *metamodelica::index_mut_checked(
                                    &mut var_field!((*full).occurrences, Matrix::FULL).clone().borrow_mut(),
                                    __idx0,
                                )? = __cell0;
                            }
                        }
                    }
                    if !(UnorderedMap::isEmpty(en.clone())) {
                        for mut e in &*UnorderedMap::valueList(en) {
                            eqn_ptr = BEquation::EquationPointers::getEqnAt(eqns, e.clone())?;
                            index = UnorderedMap::getSafe(
                                BEquation::Equation::getEqnName(eqn_ptr.clone())?,
                                eqns.map.clone(),
                                metamodelica::sourceInfo!("NBackEnd/Util/NBAdjacency.mo"),
                            )?;
                            occ_set = collectDependenciesEquation(
                                &(Pointer::access(eqn_ptr.clone())),
                                kind,
                                vars.map.clone(),
                                ({
                                    let __elt = (*metamodelica::index_checked(
                                        &var_field!((*full).dependencies, Matrix::FULL).borrow(),
                                        index,
                                    )?)
                                    .clone();
                                    __elt
                                }),
                                ({
                                    let __elt = (*metamodelica::index_checked(
                                        &var_field!((*full).solvabilities, Matrix::FULL).borrow(),
                                        index,
                                    )?)
                                    .clone();
                                    __elt
                                }),
                                ({
                                    let __elt = (*metamodelica::index_checked(
                                        &var_field!((*full).repetitions, Matrix::FULL).borrow(),
                                        index,
                                    )?)
                                    .clone();
                                    __elt
                                }),
                            )?;
                            {
                                let __cell1 = BEquation::Equation::getEqnName(eqn_ptr)?;
                                let __idx1 = index;
                                *metamodelica::index_mut_checked(
                                    &mut var_field!((*full).equation_names, Matrix::FULL).clone().borrow_mut(),
                                    __idx1,
                                )? = __cell1;
                            }
                            {
                                let __cell2 = occ_set;
                                let __idx2 = index;
                                *metamodelica::index_mut_checked(
                                    &mut var_field!((*full).occurrences, Matrix::FULL).clone().borrow_mut(),
                                    __idx2,
                                )? = __cell2;
                            }
                        }
                    }
                    full
                }
                _ => {
                    Error::addMessage(
                        Error::INTERNAL_ERROR.clone(),
                        list![{
                            let mut __mm_s = String::new();
                            __mm_s.push_str(&*literal!("NBAdjacency.Matrix.expandFull"));
                            __mm_s.push_str(&*literal!(" expected type full, got type "));
                            __mm_s.push_str(&*strictnessString(getStrictness(&full)?));
                            __mm_s.push_str(&*literal!("."));
                            ArcStr::from(__mm_s)
                        }],
                    )?;
                    return Err("fail");
                }
            })
        });
        if Flags::isSet(Flags::BLT_MATRIX_DUMP.clone())? {
            metamodelica::print({
                let mut __mm_s = String::new();
                __mm_s.push_str(&*toString(&full, literal!("Expanded Full"))?);
                __mm_s.push_str(&*literal!("\n"));
                ArcStr::from(__mm_s)
            });
        }
        Ok(full)
    }

    pub(crate) fn containsLoopCref(
        mut exp: metamodelica::Ref<Expression::NFExpression>,
        mut set: metamodelica::Ref<UnorderedSet::UnorderedSet<metamodelica::Ref<ComponentRef::NFComponentRef>>>,
    ) -> Result<bool> {
        let mut b: bool = Expression::fold(
            exp.clone(),
            (std::sync::Arc::new({
                let __pe_b2 = set.clone();
                move |__pe_a0, __pe_a1| isLoopCref(&__pe_a0, __pe_a1, __pe_b2.clone())
            })
                as std::sync::Arc<
                    dyn ::std::ops::Fn(metamodelica::Ref<Expression::NFExpression>, bool) -> Result<bool> + 'static,
                >),
            false,
        )?;
        Ok(b)
    }

    pub(crate) fn isLoopCref(
        mut exp: &metamodelica::Ref<Expression::NFExpression>,
        mut b: bool,
        mut set: metamodelica::Ref<UnorderedSet::UnorderedSet<metamodelica::Ref<ComponentRef::NFComponentRef>>>,
    ) -> Result<bool> {
        let mut b: bool = b;
        b = (::match_deref::match_deref! { match &((b, exp.clone())) {
            (false, Deref @ Expression::CREF { .. }) => UnorderedSet::contains(var_field!((**exp).cref, Expression::NFExpression::CREF).clone(), set.clone())? || UnorderedSet::contains(ComponentRef::stripSubscriptsAll(var_field!((**exp).cref, Expression::NFExpression::CREF)), set)?,
            _ => b,
            _ => unreachable!("match_deref! exhaustiveness placeholder"),
        } });
        Ok(b)
    }

    pub(crate) fn refine(
        mut full: metamodelica::Ref<Matrix>,
        mut funcMap: metamodelica::Ref<
            UnorderedMap::UnorderedMap<metamodelica::Ref<Path>, metamodelica::Ref<Function::Function>>,
        >,
        mut v: metamodelica::Ref<UnorderedMap::UnorderedMap<metamodelica::Ref<ComponentRef::NFComponentRef>, i32>>,
        mut e: metamodelica::Ref<UnorderedMap::UnorderedMap<metamodelica::Ref<ComponentRef::NFComponentRef>, i32>>,
        mut vars: &metamodelica::Ref<VariablePointers::VariablePointers>,
        mut eqns: &metamodelica::Ref<EquationPointers::EquationPointers>,
        mut vars_set: metamodelica::Ref<UnorderedSet::UnorderedSet<metamodelica::Ref<ComponentRef::NFComponentRef>>>,
        mut init: bool,
    ) -> Result<metamodelica::Ref<Matrix>> {
        let mut full: metamodelica::Ref<Matrix> = full;
        full = ({
            let mut diffArgs: metamodelica::Ref<DifferentiationArguments::DifferentiationArguments> =
                Differentiate::DifferentiationArguments::default(
                    Differentiate::DifferentiationType::SIMPLE.clone(),
                    funcMap,
                );
            let mut residual: metamodelica::Ref<Expression::NFExpression> =
                metamodelica::Ref::new(Expression::NFExpression::EMPTY {
                    ty: openmodelica_nf_frontend::NFType::interned_REAL(),
                });
            (match &*full {
                FULL { .. } => {
                    let mut eqn_ptr: Pointer::Pointer<metamodelica::Ref<Equation::Equation>>;
                    let mut exp: metamodelica::Ref<Expression::NFExpression>;
                    let mut status: Solve::Status;
                    let mut sol: metamodelica::Ref<Solvability::Solvability>;
                    let mut linear_set: metamodelica::Ref<
                        UnorderedSet::UnorderedSet<metamodelica::Ref<ComponentRef::NFComponentRef>>,
                    >;
                    let mut param_set: metamodelica::Ref<
                        UnorderedSet::UnorderedSet<metamodelica::Ref<ComponentRef::NFComponentRef>>,
                    >;
                    let mut var_set: metamodelica::Ref<
                        UnorderedSet::UnorderedSet<metamodelica::Ref<ComponentRef::NFComponentRef>>,
                    >;
                    let mut eqnIsDiscrete: bool;
                    let mut eqnIsIf: bool;
                    let mut eqnHasNoResidual: bool;
                    let mut diffOk: bool;
                    let mut residual_opt: Option<metamodelica::Ref<Expression::NFExpression>>;
                    let __range0 = UnorderedMap::valueArray(e).borrow().iter().cloned().collect::<Vec<_>>();
                    for mut eqn_idx in __range0 {
                        eqn_ptr = BEquation::EquationPointers::getEqnAt(eqns, eqn_idx)?;
                        eqnIsDiscrete = BEquation::Equation::isDiscrete(eqn_ptr.clone())
                            || BEquation::Equation::isWhenEquation(eqn_ptr.clone())?
                            || BEquation::Equation::isAlgorithm(eqn_ptr.clone());
                        eqnIsIf = BEquation::Equation::isIfEquation(eqn_ptr.clone());
                        eqnHasNoResidual = false;
                        if !(eqnIsDiscrete || eqnIsIf) {
                            residual_opt = BEquation::Equation::tryGetResidualExp(eqn_ptr.clone());
                            if (residual_opt).is_some() {
                                let __pa1 = ::match_deref::match_deref! { match &(residual_opt) {
                                    Some(__pa1) => __pa1.clone(),
                                    _ => return Err("pattern mismatch"),
                                } };
                                residual = metamodelica::Own::own(__pa1);
                            } else {
                                eqnHasNoResidual = true;
                            }
                        }
                        let __range2 = UnorderedSet::toArray(
                            ({
                                let __elt = (*metamodelica::index_checked(
                                    &var_field!((*full).occurrences, Matrix::FULL).borrow(),
                                    eqn_idx,
                                )?)
                                .clone();
                                __elt
                            }),
                        )
                        .borrow()
                        .iter()
                        .cloned()
                        .collect::<Vec<_>>();
                        for mut var in __range2 {
                            if UnorderedMap::contains(var.clone(), v.clone())? {
                                sol = UnorderedMap::getSafe(
                                    var.clone(),
                                    ({
                                        let __elt = (*metamodelica::index_checked(
                                            &var_field!((*full).solvabilities, Matrix::FULL).borrow(),
                                            eqn_idx,
                                        )?)
                                        .clone();
                                        __elt
                                    }),
                                    metamodelica::sourceInfo!("NBackEnd/Util/NBAdjacency.mo"),
                                )?;
                                if Solvability::rank(&sol)?
                                    < Solvability::rank(&(crate::NBAdjacency::Solvability::interned_IMPLICIT()))?
                                {
                                    if eqnIsDiscrete
                                        || !(BVariable::checkCref(
                                            &var,
                                            &({
                                                let __pe_b1 = init;
                                                move |__pe_a0| BVariable::isContinuous(__pe_a0, __pe_b1.clone())
                                            }),
                                            metamodelica::sourceInfo!("NBackEnd/Util/NBAdjacency.mo"),
                                        )?)
                                    {
                                        (_, status, _) = Solve::solveSimple(Pointer::access(eqn_ptr.clone()), &var)?;
                                        sol = if (status == Solve::Status::EXPLICIT.clone()) {
                                            metamodelica::Ref::new(Solvability::Solvability::EXPLICIT_LINEAR {
                                                pars: None,
                                                vars: None,
                                            })
                                        } else {
                                            crate::NBAdjacency::Solvability::interned_UNSOLVABLE()
                                        };
                                    } else if eqnIsIf || eqnHasNoResidual {
                                        sol = crate::NBAdjacency::Solvability::interned_IMPLICIT();
                                    } else {
                                        assign_field!(diffArgs.diffCref = var.clone());
                                        match '__try3: {
                                            (exp, diffArgs) = unwrap_break_err!(Differentiate::differentiateExpressionDump(residual.clone(), diffArgs.clone(), &(literal!("NBAdjacency.Matrix.refine")), &(literal!(""))), '__try3);
                                            exp = unwrap_break_err!(SimplifyExp::simplifyDump(exp.clone(), true, &(literal!("NBAdjacency.Matrix.refine")), &(literal!(""))), '__try3);
                                            diffOk = true;
                                            Ok::<_, &'static str>((diffOk.clone(), exp.clone()))
                                        } {
                                            Ok((__try3_o0, __try3_o1)) => {
                                                diffOk = __try3_o0;
                                                exp = __try3_o1;
                                            }
                                            Err(_) => {
                                                exp = residual.clone();
                                                diffOk = false;
                                            }
                                        }
                                        if !(diffOk) {
                                            sol = crate::NBAdjacency::Solvability::interned_IMPLICIT();
                                        } else if Expression::isZero(&exp)? {
                                            sol = crate::NBAdjacency::Solvability::interned_UNSOLVABLE();
                                        } else if containsLoopCref(exp.clone(), vars_set.clone())? {
                                            sol =
                                                metamodelica::Ref::new(Solvability::Solvability::EXPLICIT_NONLINEAR {
                                                    unique: !(Expression::containsCref(exp.clone(), &var)?),
                                                });
                                        } else {
                                            linear_set = Expression::extractCrefs(exp.clone())?;
                                            linear_set = UnorderedSet::filterOnFalse(
                                                linear_set,
                                                &({
                                                    let __pe_b1 = (std::sync::Arc::new(fnptr!(
                                                        BVariable::isConst,
                                                        Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>
                                                    ))
                                                        as std::sync::Arc<
                                                            dyn ::std::ops::Fn(
                                                                    Pointer::Pointer<
                                                                        metamodelica::Ref<Variable::NFVariable>,
                                                                    >,
                                                                )
                                                                    -> Result<bool>
                                                                + 'static,
                                                        >);
                                                    let __pe_b2 =
                                                        metamodelica::sourceInfo!("NBackEnd/Util/NBAdjacency.mo");
                                                    move |__pe_a0| {
                                                        BVariable::checkCref(&__pe_a0, &*__pe_b1, __pe_b2.clone())
                                                    }
                                                }),
                                            )?;
                                            (param_set, var_set) = UnorderedSet::splitOnTrue(
                                                linear_set,
                                                &({
                                                    let __pe_b1 = (std::sync::Arc::new(fnptr!(
                                                        BVariable::isParamOrConst,
                                                        Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>
                                                    ))
                                                        as std::sync::Arc<
                                                            dyn ::std::ops::Fn(
                                                                    Pointer::Pointer<
                                                                        metamodelica::Ref<Variable::NFVariable>,
                                                                    >,
                                                                )
                                                                    -> Result<bool>
                                                                + 'static,
                                                        >);
                                                    let __pe_b2 =
                                                        metamodelica::sourceInfo!("NBackEnd/Util/NBAdjacency.mo");
                                                    move |__pe_a0| {
                                                        BVariable::checkCref(&__pe_a0, &*__pe_b1, __pe_b2.clone())
                                                    }
                                                }),
                                            )?;
                                            sol = metamodelica::Ref::new(Solvability::Solvability::EXPLICIT_LINEAR {
                                                pars: if (UnorderedSet::isEmpty(param_set.clone())) {
                                                    None
                                                } else {
                                                    Some(param_set)
                                                },
                                                vars: if (UnorderedSet::isEmpty(var_set.clone())) {
                                                    None
                                                } else {
                                                    Some(var_set)
                                                },
                                            });
                                        }
                                    }
                                    UnorderedMap::add(
                                        var,
                                        sol,
                                        ({
                                            let __elt = (*metamodelica::index_checked(
                                                &var_field!((*full).solvabilities, Matrix::FULL).borrow(),
                                                eqn_idx,
                                            )?)
                                            .clone();
                                            __elt
                                        }),
                                    )?;
                                }
                            }
                        }
                    }
                    full
                }
                _ => {
                    Error::addMessage(
                        Error::INTERNAL_ERROR.clone(),
                        list![{
                            let mut __mm_s = String::new();
                            __mm_s.push_str(&*literal!("NBAdjacency.Matrix.refine"));
                            __mm_s.push_str(&*literal!(" expected type full, got type "));
                            __mm_s.push_str(&*strictnessString(getStrictness(&full)?));
                            __mm_s.push_str(&*literal!("."));
                            ArcStr::from(__mm_s)
                        }],
                    )?;
                    return Err("fail");
                }
            })
        });
        if Flags::isSet(Flags::BLT_MATRIX_DUMP.clone())? {
            metamodelica::print({
                let mut __mm_s = String::new();
                __mm_s.push_str(&*toString(&full, literal!("Refined Full"))?);
                __mm_s.push_str(&*literal!("\n"));
                ArcStr::from(__mm_s)
            });
        }
        Ok(full)
    }

    pub(crate) fn compress(
        mut adj: metamodelica::Ref<Matrix>,
        mut full: metamodelica::Ref<Matrix>,
        mut eqns: &metamodelica::Ref<EquationPointers::EquationPointers>,
        mut vars: &metamodelica::Ref<VariablePointers::VariablePointers>,
        mut old_map: metamodelica::Ref<
            UnorderedMap::UnorderedMap<metamodelica::Ref<ComponentRef::NFComponentRef>, i32>,
        >,
    ) -> Result<(metamodelica::Ref<Matrix>, metamodelica::Ref<Matrix>)> {
        let mut adj: metamodelica::Ref<Matrix> = adj;
        let mut full: metamodelica::Ref<Matrix> = full;
        let mut index_old: i32;
        let mut index_new: i32;
        let mut size: i32 = BEquation::EquationPointers::size(eqns);
        let mut name: metamodelica::Ref<ComponentRef::NFComponentRef>;
        let mut equation_names: metamodelica::Array<metamodelica::Ref<ComponentRef::NFComponentRef>>;
        let mut occurrences: metamodelica::Array<
            metamodelica::Ref<UnorderedSet::UnorderedSet<metamodelica::Ref<ComponentRef::NFComponentRef>>>,
        >;
        let mut dependencies: metamodelica::Array<
            metamodelica::Ref<
                UnorderedMap::UnorderedMap<
                    metamodelica::Ref<ComponentRef::NFComponentRef>,
                    metamodelica::Ref<Dependency::Dependency>,
                >,
            >,
        >;
        let mut solvabilities: metamodelica::Array<
            metamodelica::Ref<
                UnorderedMap::UnorderedMap<
                    metamodelica::Ref<ComponentRef::NFComponentRef>,
                    metamodelica::Ref<Solvability::Solvability>,
                >,
            >,
        >;
        let mut repetitions: metamodelica::Array<
            metamodelica::Ref<UnorderedSet::UnorderedSet<metamodelica::Ref<ComponentRef::NFComponentRef>>>,
        >;
        let mut mapping: metamodelica::Ref<Mapping::Mapping>;
        let mut m: metamodelica::Ref<IntMatrix::IntMatrix>;
        let mut m_data: metamodelica::Array<i32>;
        let mut m_aux: metamodelica::Array<i32>;
        let mut old_start: i32;
        let mut old_size: i32;
        let mut new_start: i32;
        let mut new_size: i32;
        (adj, full) = (::match_deref::match_deref! { match &((adj.clone(), full.clone())) {
            (Deref @ FINAL { .. }, Deref @ FULL { .. }) => {
                let mut new_adj: metamodelica::Ref<Matrix>;
                let mut new_full: metamodelica::Ref<Matrix>;
                mapping = Mapping::create(eqns, vars)?;
                m = IntMatrix::new(metamodelica::arrayLength(mapping.eqn_StA.clone()));
                m_data = IntMatrix::entries(var_field!((*adj).m, Matrix::FINAL));
                m_aux = IntMatrix::payload(var_field!((*adj).m, Matrix::FINAL));
                equation_names = arrayCreate(size, openmodelica_nf_frontend::NFComponentRef::interned_EMPTY());
                occurrences = arrayCreate(size, UnorderedSet::new((std::sync::Arc::new(move |__a0: metamodelica::Ref<ComponentRef::NFComponentRef>| ComponentRef::hash(&__a0)) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<ComponentRef::NFComponentRef>) -> Result<i32> + 'static>), (std::sync::Arc::new(move |__a0: metamodelica::Ref<ComponentRef::NFComponentRef>, __a1: metamodelica::Ref<ComponentRef::NFComponentRef>| ComponentRef::isEqual(&__a0, &__a1)) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<ComponentRef::NFComponentRef>, metamodelica::Ref<ComponentRef::NFComponentRef>) -> Result<bool> + 'static>), 13));
                dependencies = arrayCreate(size, UnorderedMap::new((std::sync::Arc::new(move |__a0: metamodelica::Ref<ComponentRef::NFComponentRef>| ComponentRef::hash(&__a0)) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<ComponentRef::NFComponentRef>) -> Result<i32> + 'static>), (std::sync::Arc::new(move |__a0: metamodelica::Ref<ComponentRef::NFComponentRef>, __a1: metamodelica::Ref<ComponentRef::NFComponentRef>| ComponentRef::isEqual(&__a0, &__a1)) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<ComponentRef::NFComponentRef>, metamodelica::Ref<ComponentRef::NFComponentRef>) -> Result<bool> + 'static>), 1));
                solvabilities = arrayCreate(size, UnorderedMap::new((std::sync::Arc::new(move |__a0: metamodelica::Ref<ComponentRef::NFComponentRef>| ComponentRef::hash(&__a0)) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<ComponentRef::NFComponentRef>) -> Result<i32> + 'static>), (std::sync::Arc::new(move |__a0: metamodelica::Ref<ComponentRef::NFComponentRef>, __a1: metamodelica::Ref<ComponentRef::NFComponentRef>| ComponentRef::isEqual(&__a0, &__a1)) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<ComponentRef::NFComponentRef>, metamodelica::Ref<ComponentRef::NFComponentRef>) -> Result<bool> + 'static>), 1));
                repetitions = arrayCreate(size, UnorderedSet::new((std::sync::Arc::new(move |__a0: metamodelica::Ref<ComponentRef::NFComponentRef>| ComponentRef::hash(&__a0)) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<ComponentRef::NFComponentRef>) -> Result<i32> + 'static>), (std::sync::Arc::new(move |__a0: metamodelica::Ref<ComponentRef::NFComponentRef>, __a1: metamodelica::Ref<ComponentRef::NFComponentRef>| ComponentRef::isEqual(&__a0, &__a1)) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<ComponentRef::NFComponentRef>, metamodelica::Ref<ComponentRef::NFComponentRef>) -> Result<bool> + 'static>), 13));
                for mut eqn_ptr in &*BEquation::EquationPointers::toList(eqns)? {
                    name = BEquation::Equation::getEqnName(eqn_ptr.clone())?;
                    index_new = UnorderedMap::getSafe(name.clone(), eqns.map.clone(), metamodelica::sourceInfo!("NBackEnd/Util/NBAdjacency.mo"))?;
                    index_old = UnorderedMap::getSafe(name.clone(), old_map.clone(), metamodelica::sourceInfo!("NBackEnd/Util/NBAdjacency.mo"))?;
                    (old_start, old_size) = ({let __elt = (*metamodelica::index_checked(&var_field!((*adj).mapping, Matrix::FINAL).eqn_AtS.borrow(), index_old)?).clone(); __elt});
                    (new_start, new_size) = ({let __elt = (*metamodelica::index_checked(&mapping.eqn_AtS.borrow(), index_new)?).clone(); __elt});
                    if old_size == new_size {
                        for mut i in 0..=old_size - 1 {
                            IntMatrix::copyRow(var_field!((*adj).m, Matrix::FINAL), old_start + i, m_data.clone(), m_aux.clone(), &m, new_start + i)?;
                        }
                    } else {
                        Error::addMessage(Error::INTERNAL_ERROR.clone(), list![{ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBAdjacency.Matrix.compress")); __mm_s.push_str(&*literal!(" sizes (old: ")); __mm_s.push_str(&*intString(old_size)); __mm_s.push_str(&*literal!(", new: ")); __mm_s.push_str(&*intString(new_size)); __mm_s.push_str(&*literal!(" do not mach for equation:\n")); __mm_s.push_str(&*BEquation::Equation::pointerToString(eqn_ptr.clone(), literal!(""))?); ArcStr::from(__mm_s) }])?;
                        return Err("fail");
                    }
                    {
                        let __cell0 = name;
                        let __idx0 = index_new;
                        *metamodelica::index_mut_checked(&mut equation_names.clone().borrow_mut(), __idx0)? = __cell0;
                    }
                    {
                        let __cell1 = ({let __elt = (*metamodelica::index_checked(&var_field!((*full).occurrences, Matrix::FULL).borrow(), index_old)?).clone(); __elt});
                        let __idx1 = index_new;
                        *metamodelica::index_mut_checked(&mut occurrences.clone().borrow_mut(), __idx1)? = __cell1;
                    }
                    {
                        let __cell2 = ({let __elt = (*metamodelica::index_checked(&var_field!((*full).dependencies, Matrix::FULL).borrow(), index_old)?).clone(); __elt});
                        let __idx2 = index_new;
                        *metamodelica::index_mut_checked(&mut dependencies.clone().borrow_mut(), __idx2)? = __cell2;
                    }
                    {
                        let __cell3 = ({let __elt = (*metamodelica::index_checked(&var_field!((*full).solvabilities, Matrix::FULL).borrow(), index_old)?).clone(); __elt});
                        let __idx3 = index_new;
                        *metamodelica::index_mut_checked(&mut solvabilities.clone().borrow_mut(), __idx3)? = __cell3;
                    }
                    {
                        let __cell4 = ({let __elt = (*metamodelica::index_checked(&var_field!((*full).repetitions, Matrix::FULL).borrow(), index_old)?).clone(); __elt});
                        let __idx4 = index_new;
                        *metamodelica::index_mut_checked(&mut repetitions.clone().borrow_mut(), __idx4)? = __cell4;
                    }
                }
                new_adj = metamodelica::Ref::new(Matrix::FINAL { m: m.clone(), mT: IntMatrix::transpose(&m, BVariable::VariablePointers::scalarSize(vars, true)?, 0)?, mapping: mapping.clone(), modes: var_field!((*adj).modes, Matrix::FINAL).clone(), st: var_field!((*adj).st, Matrix::FINAL).clone() });
                new_full = metamodelica::Ref::new(Matrix::FULL { equation_names: equation_names.clone(), occurrences: occurrences.clone(), dependencies: dependencies.clone(), solvabilities: solvabilities.clone(), repetitions: repetitions.clone(), mapping: mapping });
                (new_adj, new_full)
            },
            _ => {
                Error::addMessage(Error::INTERNAL_ERROR.clone(), list![{ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBAdjacency.Matrix.compress")); __mm_s.push_str(&*literal!(" expected types final and full, got types ")); __mm_s.push_str(&*strictnessString(getStrictness(&adj)?)); __mm_s.push_str(&*literal!(" and ")); __mm_s.push_str(&*strictnessString(getStrictness(&full)?)); __mm_s.push_str(&*literal!(".")); ArcStr::from(__mm_s) }])?;
                return Err("fail")
            },
            _ => unreachable!("match_deref! exhaustiveness placeholder"),
        } });
        if Flags::isSet(Flags::BLT_MATRIX_DUMP.clone())? {
            metamodelica::print({
                let mut __mm_s = String::new();
                __mm_s.push_str(&*toString(&adj, literal!("Compressed Final"))?);
                __mm_s.push_str(&*literal!("\n"));
                ArcStr::from(__mm_s)
            });
            metamodelica::print({
                let mut __mm_s = String::new();
                __mm_s.push_str(&*toString(&full, literal!("Compressed Full"))?);
                __mm_s.push_str(&*literal!("\n"));
                ArcStr::from(__mm_s)
            });
        }
        Ok((adj, full))
    }

    pub(crate) fn combine(
        mut matrices: metamodelica::List<metamodelica::Ref<Matrix>>,
    ) -> Result<metamodelica::Ref<Matrix>> {
        let mut result: metamodelica::Ref<Matrix>;
        let mut equation_names: metamodelica::List<
            metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>>,
        > = metamodelica::nil();
        let mut equation_iterators: metamodelica::List<metamodelica::List<metamodelica::Ref<Iterator::Iterator>>> =
            metamodelica::nil();
        let mut dependencies: metamodelica::List<
            metamodelica::List<
                metamodelica::Ref<
                    UnorderedMap::UnorderedMap<
                        metamodelica::Ref<ComponentRef::NFComponentRef>,
                        metamodelica::Ref<Dependency::Dependency>,
                    >,
                >,
            >,
        > = metamodelica::nil();
        let mut repetitions: metamodelica::List<
            metamodelica::List<
                metamodelica::Ref<UnorderedSet::UnorderedSet<metamodelica::Ref<ComponentRef::NFComponentRef>>>,
            >,
        > = metamodelica::nil();
        let mut solved_crefs: metamodelica::List<
            metamodelica::List<metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>>>,
        > = metamodelica::nil();
        for mut matrix in &*matrices.reverse() {
            let _ = (match &*matrix.clone() {
                SPARSITY { .. } => {
                    equation_names = metamodelica::cons(
                        var_field!((**matrix).equation_names, Matrix::SPARSITY)
                            .clone()
                            .borrow()
                            .iter()
                            .cloned()
                            .collect::<metamodelica::List<_>>(),
                        equation_names,
                    );
                    equation_iterators = metamodelica::cons(
                        var_field!((**matrix).equation_iterators, Matrix::SPARSITY)
                            .clone()
                            .borrow()
                            .iter()
                            .cloned()
                            .collect::<metamodelica::List<_>>(),
                        equation_iterators,
                    );
                    dependencies = metamodelica::cons(
                        var_field!((**matrix).dependencies, Matrix::SPARSITY)
                            .clone()
                            .borrow()
                            .iter()
                            .cloned()
                            .collect::<metamodelica::List<_>>(),
                        dependencies,
                    );
                    repetitions = metamodelica::cons(
                        var_field!((**matrix).repetitions, Matrix::SPARSITY)
                            .clone()
                            .borrow()
                            .iter()
                            .cloned()
                            .collect::<metamodelica::List<_>>(),
                        repetitions,
                    );
                    solved_crefs = metamodelica::cons(
                        var_field!((**matrix).solved_crefs, Matrix::SPARSITY)
                            .clone()
                            .borrow()
                            .iter()
                            .cloned()
                            .collect::<metamodelica::List<_>>(),
                        solved_crefs,
                    );
                    ()
                }
                _ => {
                    Error::addMessage(
                        Error::INTERNAL_ERROR.clone(),
                        list![{
                            let mut __mm_s = String::new();
                            __mm_s.push_str(&*literal!("NBAdjacency.Matrix.combine"));
                            __mm_s.push_str(&*literal!(" expected type sparsity, got type "));
                            __mm_s.push_str(&*strictnessString(getStrictness(metamodelica::AsArg::as_arg(&matrix))?));
                            __mm_s.push_str(&*literal!("."));
                            ArcStr::from(__mm_s)
                        }],
                    )?;
                    return Err("fail");
                }
            });
        }
        result = metamodelica::Ref::new(Matrix::SPARSITY {
            equation_names: metamodelica::arrayFromVec(List::flatten(equation_names)?.into_iter().cloned().collect()),
            equation_iterators: metamodelica::arrayFromVec(
                List::flatten(equation_iterators)?.into_iter().cloned().collect(),
            ),
            dependencies: metamodelica::arrayFromVec(List::flatten(dependencies)?.into_iter().cloned().collect()),
            repetitions: metamodelica::arrayFromVec(List::flatten(repetitions)?.into_iter().cloned().collect()),
            solved_crefs: metamodelica::arrayFromVec(List::flatten(solved_crefs)?.into_iter().cloned().collect()),
        });
        Ok(result)
    }

    pub(crate) fn toString(mut adj: &metamodelica::Ref<Matrix>, mut r#str: ArcStr) -> Result<ArcStr> {
        let mut r#str: ArcStr = r#str;
        r#str = (match &**adj {
            FULL { .. } => {
                let mut types: metamodelica::List<metamodelica::Ref<Type::NFType>>;
                let mut names: metamodelica::Array<ArcStr>;
                let mut types_str: metamodelica::Array<ArcStr>;
                let mut complex_sizes: metamodelica::Array<ArcStr>;
                let mut length0: i32;
                let mut length1: i32;
                let mut length2: i32;
                r#str = {
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*StringUtil::headline_2(
                        &({
                            let mut __mm_s = String::new();
                            __mm_s.push_str(&*r#str);
                            __mm_s.push_str(&*literal!("FULL Adjacency Matrix"));
                            ArcStr::from(__mm_s)
                        }),
                    )?);
                    __mm_s.push_str(&*literal!("\n"));
                    ArcStr::from(__mm_s)
                };
                types = ({
                    let mut __acc: metamodelica::List<metamodelica::Ref<Type::NFType>> = metamodelica::nil();
                    for mut name in (var_field!((**adj).equation_names, Matrix::FULL).clone())
                        .borrow()
                        .iter()
                    {
                        let __x = ComponentRef::getSubscriptedType(&(name.clone()), false)?;
                        __acc = cons(__x, __acc);
                    }
                    __acc.reverse()
                });
                complex_sizes = metamodelica::arrayFromVec(
                    ({
                        let mut __acc: metamodelica::List<ArcStr> = metamodelica::nil();
                        for mut ty in (types.clone()).into_iter().cloned() {
                            let __x = Util::applyOptionOrDefault(
                                Type::complexSize(&(ty.clone()), true)?,
                                &fnptr!(intString, i32),
                                literal!("0"),
                            )?;
                            __acc = cons(__x, __acc);
                        }
                        __acc.reverse()
                    })
                    .into_iter()
                    .cloned()
                    .collect(),
                );
                types_str = metamodelica::arrayFromVec(
                    ({
                        let mut __acc: metamodelica::List<ArcStr> = metamodelica::nil();
                        for mut ty in (types).into_iter().cloned() {
                            let __x = dimsString(Type::arrayDims(ty.clone()))?;
                            __acc = cons(__x, __acc);
                        }
                        __acc.reverse()
                    })
                    .into_iter()
                    .cloned()
                    .collect(),
                );
                names = metamodelica::arrayFromVec(
                    ({
                        let mut __acc: metamodelica::List<ArcStr> = metamodelica::nil();
                        for mut name in (var_field!((**adj).equation_names, Matrix::FULL).clone())
                            .borrow()
                            .iter()
                        {
                            let __x = ComponentRef::toString(&(name.clone()))?;
                            __acc = cons(__x, __acc);
                        }
                        __acc.reverse()
                    })
                    .into_iter()
                    .cloned()
                    .collect(),
                );
                length0 = ({
                    let mut __acc: Option<i32> = None;
                    for mut sz in (complex_sizes.clone()).borrow().iter() {
                        let __x = ((sz).len() as i32);
                        __acc = Some(match __acc {
                            None => __x,
                            Some(__cur) => {
                                if __x > __cur {
                                    __x
                                } else {
                                    __cur
                                }
                            }
                        });
                    }
                    __acc.unwrap_or((-i32::MAX))
                });
                length1 = ({
                    let mut __acc: Option<i32> = None;
                    for mut ty in (types_str.clone()).borrow().iter() {
                        let __x = ((ty).len() as i32);
                        __acc = Some(match __acc {
                            None => __x,
                            Some(__cur) => {
                                if __x > __cur {
                                    __x
                                } else {
                                    __cur
                                }
                            }
                        });
                    }
                    __acc.unwrap_or((-i32::MAX))
                }) + 1;
                length2 = ({
                    let mut __acc: Option<i32> = None;
                    for mut name in (names.clone()).borrow().iter() {
                        let __x = ((name).len() as i32);
                        __acc = Some(match __acc {
                            None => __x,
                            Some(__cur) => {
                                if __x > __cur {
                                    __x
                                } else {
                                    __cur
                                }
                            }
                        });
                    }
                    __acc.unwrap_or((-i32::MAX))
                }) + 3;
                for mut i in 1..=metamodelica::arrayLength(names.clone()) {
                    r#str = {
                        let mut __mm_s = String::new();
                        __mm_s.push_str(&*r#str);
                        __mm_s.push_str(&*metamodelica::arrayGet(complex_sizes.clone(), i)?);
                        __mm_s.push_str(&*literal!(" "));
                        __mm_s.push_str(&*StringUtil::repeat(
                            literal!(" "),
                            length0 - ((metamodelica::arrayGet(complex_sizes.clone(), i)?).len() as i32),
                        )?);
                        __mm_s.push_str(&*literal!(" | "));
                        __mm_s.push_str(&*metamodelica::arrayGet(types_str.clone(), i)?);
                        __mm_s.push_str(&*literal!(" "));
                        __mm_s.push_str(&*StringUtil::repeat(
                            literal!("."),
                            length1 - ((metamodelica::arrayGet(types_str.clone(), i)?).len() as i32),
                        )?);
                        __mm_s.push_str(&*metamodelica::arrayGet(names.clone(), i)?);
                        __mm_s.push_str(&*literal!(" "));
                        __mm_s.push_str(&*StringUtil::repeat(
                            literal!("."),
                            length2 - ((metamodelica::arrayGet(names.clone(), i)?).len() as i32),
                        )?);
                        __mm_s.push_str(&*literal!(" "));
                        __mm_s.push_str(&*List::toString(
                            UnorderedSet::toList(
                                ({
                                    let __elt = (*metamodelica::index_checked(
                                        &var_field!((**adj).occurrences, Matrix::FULL).borrow(),
                                        i,
                                    )?)
                                    .clone();
                                    __elt
                                }),
                            ),
                            &({
                                let __pe_b1 = ({
                                    let __elt = (*metamodelica::index_checked(
                                        &var_field!((**adj).dependencies, Matrix::FULL).borrow(),
                                        i,
                                    )?)
                                    .clone();
                                    __elt
                                });
                                let __pe_b2 = ({
                                    let __elt = (*metamodelica::index_checked(
                                        &var_field!((**adj).solvabilities, Matrix::FULL).borrow(),
                                        i,
                                    )?)
                                    .clone();
                                    __elt
                                });
                                let __pe_b3 = ({
                                    let __elt = (*metamodelica::index_checked(
                                        &var_field!((**adj).repetitions, Matrix::FULL).borrow(),
                                        i,
                                    )?)
                                    .clone();
                                    __elt
                                });
                                move |__pe_a0| fullString(__pe_a0, __pe_b1.clone(), __pe_b2.clone(), __pe_b3.clone())
                            }),
                            List::Style::FLAT_CURLY.clone(),
                        )?);
                        __mm_s.push_str(&*literal!("\n"));
                        ArcStr::from(__mm_s)
                    };
                }
                r#str
            }
            FINAL {
                m: __adj_m,
                mT: __adj_mT,
                mapping: __adj_mapping,
                ..
            } => {
                r#str = {
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*StringUtil::headline_2(
                        &({
                            let mut __mm_s = String::new();
                            __mm_s.push_str(&*r#str);
                            __mm_s.push_str(&*literal!("FINAL Adjacency Matrix"));
                            ArcStr::from(__mm_s)
                        }),
                    )?);
                    __mm_s.push_str(&*literal!("\n"));
                    ArcStr::from(__mm_s)
                };
                if IntMatrix::rows(metamodelica::AsArg::as_arg(&__adj_m)) > 0 {
                    r#str = {
                        let mut __mm_s = String::new();
                        __mm_s.push_str(&*r#str);
                        __mm_s.push_str(&*StringUtil::headline_4(
                            &(literal!("Normal Adjacency Matrix (row = equation)")),
                        )?);
                        ArcStr::from(__mm_s)
                    };
                    r#str = {
                        let mut __mm_s = String::new();
                        __mm_s.push_str(&*r#str);
                        __mm_s.push_str(&*IntMatrix::toString(metamodelica::AsArg::as_arg(&__adj_m))?);
                        ArcStr::from(__mm_s)
                    };
                }
                r#str = {
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*r#str);
                    __mm_s.push_str(&*literal!("\n"));
                    ArcStr::from(__mm_s)
                };
                if IntMatrix::rows(metamodelica::AsArg::as_arg(&__adj_mT)) > 0 {
                    r#str = {
                        let mut __mm_s = String::new();
                        __mm_s.push_str(&*r#str);
                        __mm_s.push_str(&*StringUtil::headline_4(
                            &(literal!("Transposed Adjacency Matrix (row = variable)")),
                        )?);
                        ArcStr::from(__mm_s)
                    };
                    r#str = {
                        let mut __mm_s = String::new();
                        __mm_s.push_str(&*r#str);
                        __mm_s.push_str(&*IntMatrix::toString(metamodelica::AsArg::as_arg(&__adj_mT))?);
                        ArcStr::from(__mm_s)
                    };
                }
                r#str = {
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*r#str);
                    __mm_s.push_str(&*literal!("\n"));
                    __mm_s.push_str(&*Mapping::toString(metamodelica::AsArg::as_arg(&__adj_mapping))?);
                    ArcStr::from(__mm_s)
                };
                r#str
            }
            SPARSITY { .. } => {
                let mut types: metamodelica::List<metamodelica::Ref<Type::NFType>>;
                let mut solved: metamodelica::Array<ArcStr>;
                let mut names: metamodelica::Array<ArcStr>;
                let mut types_str: metamodelica::Array<ArcStr>;
                let mut complex_sizes: metamodelica::Array<ArcStr>;
                let mut length0: i32;
                let mut length1: i32;
                let mut length2: i32;
                let mut length3: i32;
                r#str = {
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*StringUtil::headline_2(
                        &({
                            let mut __mm_s = String::new();
                            __mm_s.push_str(&*r#str);
                            __mm_s.push_str(&*literal!("SPARSITY Adjacency Matrix"));
                            ArcStr::from(__mm_s)
                        }),
                    )?);
                    __mm_s.push_str(&*literal!("\n"));
                    ArcStr::from(__mm_s)
                };
                types = ({
                    let mut __acc: metamodelica::List<metamodelica::Ref<Type::NFType>> = metamodelica::nil();
                    for mut name in (var_field!((**adj).equation_names, Matrix::SPARSITY).clone())
                        .borrow()
                        .iter()
                    {
                        let __x = ComponentRef::getSubscriptedType(&(name.clone()), false)?;
                        __acc = cons(__x, __acc);
                    }
                    __acc.reverse()
                });
                complex_sizes = metamodelica::arrayFromVec(
                    ({
                        let mut __acc: metamodelica::List<ArcStr> = metamodelica::nil();
                        for mut ty in (types.clone()).into_iter().cloned() {
                            let __x = Util::applyOptionOrDefault(
                                Type::complexSize(&(ty.clone()), true)?,
                                &fnptr!(intString, i32),
                                literal!("0"),
                            )?;
                            __acc = cons(__x, __acc);
                        }
                        __acc.reverse()
                    })
                    .into_iter()
                    .cloned()
                    .collect(),
                );
                types_str = metamodelica::arrayFromVec(
                    ({
                        let mut __acc: metamodelica::List<ArcStr> = metamodelica::nil();
                        for mut ty in (types).into_iter().cloned() {
                            let __x = dimsString(Type::arrayDims(ty.clone()))?;
                            __acc = cons(__x, __acc);
                        }
                        __acc.reverse()
                    })
                    .into_iter()
                    .cloned()
                    .collect(),
                );
                names = metamodelica::arrayFromVec(
                    ({
                        let mut __acc: metamodelica::List<ArcStr> = metamodelica::nil();
                        for mut name in (var_field!((**adj).equation_names, Matrix::SPARSITY).clone())
                            .borrow()
                            .iter()
                        {
                            let __x = ComponentRef::toString(&(name.clone()))?;
                            __acc = cons(__x, __acc);
                        }
                        __acc.reverse()
                    })
                    .into_iter()
                    .cloned()
                    .collect(),
                );
                solved = metamodelica::arrayFromVec(
                    ({
                        let mut __acc: metamodelica::List<ArcStr> = metamodelica::nil();
                        for mut var_list in (var_field!((**adj).solved_crefs, Matrix::SPARSITY).clone())
                            .borrow()
                            .iter()
                        {
                            let __x = List::toString(
                                var_list.clone(),
                                &move |__a0: metamodelica::Ref<ComponentRef::NFComponentRef>| {
                                    ComponentRef::toString(&__a0)
                                },
                                List::Style::FLAT_CURLY.clone(),
                            )?;
                            __acc = cons(__x, __acc);
                        }
                        __acc.reverse()
                    })
                    .into_iter()
                    .cloned()
                    .collect(),
                );
                length0 = ({
                    let mut __acc: Option<i32> = None;
                    for mut sz in (complex_sizes.clone()).borrow().iter() {
                        let __x = ((sz).len() as i32);
                        __acc = Some(match __acc {
                            None => __x,
                            Some(__cur) => {
                                if __x > __cur {
                                    __x
                                } else {
                                    __cur
                                }
                            }
                        });
                    }
                    __acc.unwrap_or((-i32::MAX))
                });
                length1 = ({
                    let mut __acc: Option<i32> = None;
                    for mut ty in (types_str.clone()).borrow().iter() {
                        let __x = ((ty).len() as i32);
                        __acc = Some(match __acc {
                            None => __x,
                            Some(__cur) => {
                                if __x > __cur {
                                    __x
                                } else {
                                    __cur
                                }
                            }
                        });
                    }
                    __acc.unwrap_or((-i32::MAX))
                }) + 1;
                length2 = ({
                    let mut __acc: Option<i32> = None;
                    for mut name in (names.clone()).borrow().iter() {
                        let __x = ((name).len() as i32);
                        __acc = Some(match __acc {
                            None => __x,
                            Some(__cur) => {
                                if __x > __cur {
                                    __x
                                } else {
                                    __cur
                                }
                            }
                        });
                    }
                    __acc.unwrap_or((-i32::MAX))
                }) + 3;
                length3 = ({
                    let mut __acc: Option<i32> = None;
                    for mut var in (solved.clone()).borrow().iter() {
                        let __x = ((var).len() as i32);
                        __acc = Some(match __acc {
                            None => __x,
                            Some(__cur) => {
                                if __x > __cur {
                                    __x
                                } else {
                                    __cur
                                }
                            }
                        });
                    }
                    __acc.unwrap_or((-i32::MAX))
                }) + 3;
                for mut i in 1..=metamodelica::arrayLength(names.clone()) {
                    r#str = {
                        let mut __mm_s = String::new();
                        __mm_s.push_str(&*r#str);
                        __mm_s.push_str(&*metamodelica::arrayGet(complex_sizes.clone(), i)?);
                        __mm_s.push_str(&*literal!(" "));
                        __mm_s.push_str(&*StringUtil::repeat(
                            literal!(" "),
                            length0 - ((metamodelica::arrayGet(complex_sizes.clone(), i)?).len() as i32),
                        )?);
                        __mm_s.push_str(&*literal!(" | "));
                        __mm_s.push_str(&*metamodelica::arrayGet(types_str.clone(), i)?);
                        __mm_s.push_str(&*literal!(" "));
                        __mm_s.push_str(&*StringUtil::repeat(
                            literal!("."),
                            length1 - ((metamodelica::arrayGet(types_str.clone(), i)?).len() as i32),
                        )?);
                        __mm_s.push_str(&*metamodelica::arrayGet(names.clone(), i)?);
                        __mm_s.push_str(&*literal!(" "));
                        __mm_s.push_str(&*StringUtil::repeat(
                            literal!("."),
                            length2 - ((metamodelica::arrayGet(names.clone(), i)?).len() as i32),
                        )?);
                        __mm_s.push_str(&*metamodelica::arrayGet(solved.clone(), i)?);
                        __mm_s.push_str(&*literal!(" "));
                        __mm_s.push_str(&*StringUtil::repeat(
                            literal!("."),
                            length3 - ((metamodelica::arrayGet(solved.clone(), i)?).len() as i32),
                        )?);
                        __mm_s.push_str(&*literal!(" "));
                        __mm_s.push_str(&*List::toString(
                            UnorderedMap::keyList(
                                ({
                                    let __elt = (*metamodelica::index_checked(
                                        &var_field!((**adj).dependencies, Matrix::SPARSITY).borrow(),
                                        i,
                                    )?)
                                    .clone();
                                    __elt
                                }),
                            ),
                            &({
                                let __pe_b1 = ({
                                    let __elt = (*metamodelica::index_checked(
                                        &var_field!((**adj).dependencies, Matrix::SPARSITY).borrow(),
                                        i,
                                    )?)
                                    .clone();
                                    __elt
                                });
                                let __pe_b2 = ({
                                    let __elt = (*metamodelica::index_checked(
                                        &var_field!((**adj).repetitions, Matrix::SPARSITY).borrow(),
                                        i,
                                    )?)
                                    .clone();
                                    __elt
                                });
                                move |__pe_a0| sparsityString(__pe_a0, __pe_b1.clone(), __pe_b2.clone())
                            }),
                            List::Style::FLAT_CURLY.clone(),
                        )?);
                        __mm_s.push_str(&*literal!("\n"));
                        ArcStr::from(__mm_s)
                    };
                }
                r#str
            }
            EMPTY { .. } => {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*r#str);
                __mm_s.push_str(&*StringUtil::headline_4(&(literal!("EMPTY Adjacency Matrix")))?);
                __mm_s.push_str(&*literal!("\n"));
                ArcStr::from(__mm_s)
            }
            _ => {
                Error::addMessage(
                    Error::INTERNAL_ERROR.clone(),
                    list![{
                        let mut __mm_s = String::new();
                        __mm_s.push_str(&*literal!("NBAdjacency.Matrix.toString"));
                        __mm_s.push_str(&*literal!(" failed because of unknown adjacency matrix type."));
                        ArcStr::from(__mm_s)
                    }],
                )?;
                return Err("fail");
            }
        });
        Ok(r#str)
    }

    pub(crate) fn solvabilityString(mut adj: &metamodelica::Ref<Matrix>, mut r#str: ArcStr) -> Result<ArcStr> {
        let mut r#str: ArcStr = r#str;
        r#str = ({
            let mut xx: metamodelica::List<ArcStr> = metamodelica::nil();
            let mut ii: metamodelica::List<ArcStr> = metamodelica::nil();
            let mut nm: metamodelica::List<ArcStr> = metamodelica::nil();
            let mut np: metamodelica::List<ArcStr> = metamodelica::nil();
            let mut lv: metamodelica::List<ArcStr> = metamodelica::nil();
            let mut lp: metamodelica::List<ArcStr> = metamodelica::nil();
            let mut lc: metamodelica::List<ArcStr> = metamodelica::nil();
            let mut qq: metamodelica::List<ArcStr> = metamodelica::nil();
            (match &**adj {
                FULL { .. } => {
                    let mut XX: metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>>;
                    let mut II: metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>>;
                    let mut NM: metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>>;
                    let mut NP: metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>>;
                    let mut LV: metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>>;
                    let mut LP: metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>>;
                    let mut LC: metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>>;
                    let mut QQ: metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>>;
                    let mut names: metamodelica::Array<ArcStr>;
                    let mut types: metamodelica::Array<ArcStr>;
                    let mut XX_: metamodelica::Array<ArcStr>;
                    let mut II_: metamodelica::Array<ArcStr>;
                    let mut NM_: metamodelica::Array<ArcStr>;
                    let mut NP_: metamodelica::Array<ArcStr>;
                    let mut LV_: metamodelica::Array<ArcStr>;
                    let mut LP_: metamodelica::Array<ArcStr>;
                    let mut LC_: metamodelica::Array<ArcStr>;
                    let mut QQ_: metamodelica::Array<ArcStr>;
                    let mut length1: i32;
                    let mut length2: i32;
                    let mut length_xx: i32;
                    let mut length_ii: i32;
                    let mut length_nm: i32;
                    let mut length_np: i32;
                    let mut length_lv: i32;
                    let mut length_lp: i32;
                    let mut length_lc: i32;
                    let mut length_qq: i32;
                    r#str = {
                        let mut __mm_s = String::new();
                        __mm_s.push_str(&*StringUtil::headline_2(
                            &({
                                let mut __mm_s = String::new();
                                __mm_s.push_str(&*r#str);
                                __mm_s.push_str(&*literal!(" Solvability Adjacency Matrix"));
                                ArcStr::from(__mm_s)
                            }),
                        )?);
                        __mm_s.push_str(&*literal!("\n"));
                        ArcStr::from(__mm_s)
                    };
                    types = metamodelica::arrayFromVec(
                        ({
                            let mut __acc: metamodelica::List<ArcStr> = metamodelica::nil();
                            for mut name in (var_field!((**adj).equation_names, Matrix::FULL).clone())
                                .borrow()
                                .iter()
                            {
                                let __x = intString(Type::sizeOf(
                                    &(ComponentRef::getSubscriptedType(&(name.clone()), false)?),
                                    true,
                                )?);
                                __acc = cons(__x, __acc);
                            }
                            __acc.reverse()
                        })
                        .into_iter()
                        .cloned()
                        .collect(),
                    );
                    names = metamodelica::arrayFromVec(
                        ({
                            let mut __acc: metamodelica::List<ArcStr> = metamodelica::nil();
                            for mut name in (var_field!((**adj).equation_names, Matrix::FULL).clone())
                                .borrow()
                                .iter()
                            {
                                let __x = ComponentRef::toString(&(name.clone()))?;
                                __acc = cons(__x, __acc);
                            }
                            __acc.reverse()
                        })
                        .into_iter()
                        .cloned()
                        .collect(),
                    );
                    for mut i in ({
                        let __s = metamodelica::arrayLength(names.clone());
                        let __e = 1;
                        (0i32..)
                            .map(move |__k| __s + __k * (-1))
                            .take_while(move |&__v| __v >= __e)
                    }) {
                        (XX, II, NM, NP, LV, LP, LC, QQ) = Solvability::categorize(
                            &(UnorderedSet::toList(
                                ({
                                    let __elt = (*metamodelica::index_checked(
                                        &var_field!((**adj).occurrences, Matrix::FULL).borrow(),
                                        i,
                                    )?)
                                    .clone();
                                    __elt
                                }),
                            )),
                            ({
                                let __elt = (*metamodelica::index_checked(
                                    &var_field!((**adj).solvabilities, Matrix::FULL).borrow(),
                                    i,
                                )?)
                                .clone();
                                __elt
                            }),
                        )?;
                        xx = metamodelica::cons(
                            List::toStringCustom(
                                XX,
                                &move |__a0: metamodelica::Ref<ComponentRef::NFComponentRef>| {
                                    ComponentRef::toString(&__a0)
                                },
                                literal!("XX "),
                                literal!("{"),
                                literal!(","),
                                literal!("}"),
                                false,
                                0,
                            )?,
                            xx,
                        );
                        ii = metamodelica::cons(
                            List::toStringCustom(
                                II,
                                &move |__a0: metamodelica::Ref<ComponentRef::NFComponentRef>| {
                                    ComponentRef::toString(&__a0)
                                },
                                literal!("II "),
                                literal!("{"),
                                literal!(","),
                                literal!("}"),
                                false,
                                0,
                            )?,
                            ii,
                        );
                        nm = metamodelica::cons(
                            List::toStringCustom(
                                NM,
                                &move |__a0: metamodelica::Ref<ComponentRef::NFComponentRef>| {
                                    ComponentRef::toString(&__a0)
                                },
                                literal!("N- "),
                                literal!("{"),
                                literal!(","),
                                literal!("}"),
                                false,
                                0,
                            )?,
                            nm,
                        );
                        np = metamodelica::cons(
                            List::toStringCustom(
                                NP,
                                &move |__a0: metamodelica::Ref<ComponentRef::NFComponentRef>| {
                                    ComponentRef::toString(&__a0)
                                },
                                literal!("N+ "),
                                literal!("{"),
                                literal!(","),
                                literal!("}"),
                                false,
                                0,
                            )?,
                            np,
                        );
                        lv = metamodelica::cons(
                            List::toStringCustom(
                                LV,
                                &move |__a0: metamodelica::Ref<ComponentRef::NFComponentRef>| {
                                    ComponentRef::toString(&__a0)
                                },
                                literal!("LV "),
                                literal!("{"),
                                literal!(","),
                                literal!("}"),
                                false,
                                0,
                            )?,
                            lv,
                        );
                        lp = metamodelica::cons(
                            List::toStringCustom(
                                LP,
                                &move |__a0: metamodelica::Ref<ComponentRef::NFComponentRef>| {
                                    ComponentRef::toString(&__a0)
                                },
                                literal!("LP "),
                                literal!("{"),
                                literal!(","),
                                literal!("}"),
                                false,
                                0,
                            )?,
                            lp,
                        );
                        lc = metamodelica::cons(
                            List::toStringCustom(
                                LC,
                                &move |__a0: metamodelica::Ref<ComponentRef::NFComponentRef>| {
                                    ComponentRef::toString(&__a0)
                                },
                                literal!("LC "),
                                literal!("{"),
                                literal!(","),
                                literal!("}"),
                                false,
                                0,
                            )?,
                            lc,
                        );
                        qq = metamodelica::cons(
                            List::toStringCustom(
                                QQ,
                                &move |__a0: metamodelica::Ref<ComponentRef::NFComponentRef>| {
                                    ComponentRef::toString(&__a0)
                                },
                                literal!("|| "),
                                literal!("{"),
                                literal!(","),
                                literal!("}"),
                                false,
                                0,
                            )?,
                            qq,
                        );
                    }
                    XX_ = metamodelica::arrayFromVec(xx.into_iter().cloned().collect());
                    II_ = metamodelica::arrayFromVec(ii.into_iter().cloned().collect());
                    NM_ = metamodelica::arrayFromVec(nm.into_iter().cloned().collect());
                    NP_ = metamodelica::arrayFromVec(np.into_iter().cloned().collect());
                    LV_ = metamodelica::arrayFromVec(lv.into_iter().cloned().collect());
                    LP_ = metamodelica::arrayFromVec(lp.into_iter().cloned().collect());
                    LC_ = metamodelica::arrayFromVec(lc.into_iter().cloned().collect());
                    QQ_ = metamodelica::arrayFromVec(qq.into_iter().cloned().collect());
                    length1 = ({
                        let mut __acc: Option<i32> = None;
                        for mut ty in (types.clone()).borrow().iter() {
                            let __x = ((ty).len() as i32);
                            __acc = Some(match __acc {
                                None => __x,
                                Some(__cur) => {
                                    if __x > __cur {
                                        __x
                                    } else {
                                        __cur
                                    }
                                }
                            });
                        }
                        __acc.unwrap_or((-i32::MAX))
                    }) + 1;
                    length2 = ({
                        let mut __acc: Option<i32> = None;
                        for mut name in (names.clone()).borrow().iter() {
                            let __x = ((name).len() as i32);
                            __acc = Some(match __acc {
                                None => __x,
                                Some(__cur) => {
                                    if __x > __cur {
                                        __x
                                    } else {
                                        __cur
                                    }
                                }
                            });
                        }
                        __acc.unwrap_or((-i32::MAX))
                    }) + 3;
                    length_xx = ({
                        let mut __acc: Option<i32> = None;
                        for mut s in (XX_.clone()).borrow().iter() {
                            let __x = ((s).len() as i32);
                            __acc = Some(match __acc {
                                None => __x,
                                Some(__cur) => {
                                    if __x > __cur {
                                        __x
                                    } else {
                                        __cur
                                    }
                                }
                            });
                        }
                        __acc.unwrap_or((-i32::MAX))
                    });
                    length_ii = ({
                        let mut __acc: Option<i32> = None;
                        for mut s in (II_.clone()).borrow().iter() {
                            let __x = ((s).len() as i32);
                            __acc = Some(match __acc {
                                None => __x,
                                Some(__cur) => {
                                    if __x > __cur {
                                        __x
                                    } else {
                                        __cur
                                    }
                                }
                            });
                        }
                        __acc.unwrap_or((-i32::MAX))
                    });
                    length_nm = ({
                        let mut __acc: Option<i32> = None;
                        for mut s in (NM_.clone()).borrow().iter() {
                            let __x = ((s).len() as i32);
                            __acc = Some(match __acc {
                                None => __x,
                                Some(__cur) => {
                                    if __x > __cur {
                                        __x
                                    } else {
                                        __cur
                                    }
                                }
                            });
                        }
                        __acc.unwrap_or((-i32::MAX))
                    });
                    length_np = ({
                        let mut __acc: Option<i32> = None;
                        for mut s in (NP_.clone()).borrow().iter() {
                            let __x = ((s).len() as i32);
                            __acc = Some(match __acc {
                                None => __x,
                                Some(__cur) => {
                                    if __x > __cur {
                                        __x
                                    } else {
                                        __cur
                                    }
                                }
                            });
                        }
                        __acc.unwrap_or((-i32::MAX))
                    });
                    length_lv = ({
                        let mut __acc: Option<i32> = None;
                        for mut s in (LV_.clone()).borrow().iter() {
                            let __x = ((s).len() as i32);
                            __acc = Some(match __acc {
                                None => __x,
                                Some(__cur) => {
                                    if __x > __cur {
                                        __x
                                    } else {
                                        __cur
                                    }
                                }
                            });
                        }
                        __acc.unwrap_or((-i32::MAX))
                    });
                    length_lp = ({
                        let mut __acc: Option<i32> = None;
                        for mut s in (LP_.clone()).borrow().iter() {
                            let __x = ((s).len() as i32);
                            __acc = Some(match __acc {
                                None => __x,
                                Some(__cur) => {
                                    if __x > __cur {
                                        __x
                                    } else {
                                        __cur
                                    }
                                }
                            });
                        }
                        __acc.unwrap_or((-i32::MAX))
                    });
                    length_lc = ({
                        let mut __acc: Option<i32> = None;
                        for mut s in (LC_.clone()).borrow().iter() {
                            let __x = ((s).len() as i32);
                            __acc = Some(match __acc {
                                None => __x,
                                Some(__cur) => {
                                    if __x > __cur {
                                        __x
                                    } else {
                                        __cur
                                    }
                                }
                            });
                        }
                        __acc.unwrap_or((-i32::MAX))
                    });
                    length_qq = ({
                        let mut __acc: Option<i32> = None;
                        for mut s in (QQ_.clone()).borrow().iter() {
                            let __x = ((s).len() as i32);
                            __acc = Some(match __acc {
                                None => __x,
                                Some(__cur) => {
                                    if __x > __cur {
                                        __x
                                    } else {
                                        __cur
                                    }
                                }
                            });
                        }
                        __acc.unwrap_or((-i32::MAX))
                    });
                    for mut i in 1..=metamodelica::arrayLength(names.clone()) {
                        r#str = {
                            let mut __mm_s = String::new();
                            __mm_s.push_str(&*r#str);
                            __mm_s.push_str(&*metamodelica::arrayGet(types.clone(), i)?);
                            __mm_s.push_str(&*literal!(" "));
                            __mm_s.push_str(&*StringUtil::repeat(
                                literal!("."),
                                length1 - ((metamodelica::arrayGet(types.clone(), i)?).len() as i32),
                            )?);
                            __mm_s.push_str(&*literal!(" "));
                            __mm_s.push_str(&*metamodelica::arrayGet(names.clone(), i)?);
                            __mm_s.push_str(&*literal!(" "));
                            __mm_s.push_str(&*StringUtil::repeat(
                                literal!("."),
                                length2 - ((metamodelica::arrayGet(names.clone(), i)?).len() as i32),
                            )?);
                            __mm_s.push_str(&*metamodelica::arrayGet(LC_.clone(), i)?);
                            __mm_s.push_str(&*literal!(" "));
                            __mm_s.push_str(&*StringUtil::repeat(
                                literal!("."),
                                length_lc - ((metamodelica::arrayGet(LC_.clone(), i)?).len() as i32),
                            )?);
                            __mm_s.push_str(&*metamodelica::arrayGet(LP_.clone(), i)?);
                            __mm_s.push_str(&*literal!(" "));
                            __mm_s.push_str(&*StringUtil::repeat(
                                literal!("."),
                                length_lp - ((metamodelica::arrayGet(LP_.clone(), i)?).len() as i32),
                            )?);
                            __mm_s.push_str(&*metamodelica::arrayGet(LV_.clone(), i)?);
                            __mm_s.push_str(&*literal!(" "));
                            __mm_s.push_str(&*StringUtil::repeat(
                                literal!("."),
                                length_lv - ((metamodelica::arrayGet(LV_.clone(), i)?).len() as i32),
                            )?);
                            __mm_s.push_str(&*metamodelica::arrayGet(NP_.clone(), i)?);
                            __mm_s.push_str(&*literal!(" "));
                            __mm_s.push_str(&*StringUtil::repeat(
                                literal!("."),
                                length_np - ((metamodelica::arrayGet(NP_.clone(), i)?).len() as i32),
                            )?);
                            __mm_s.push_str(&*metamodelica::arrayGet(NM_.clone(), i)?);
                            __mm_s.push_str(&*literal!(" "));
                            __mm_s.push_str(&*StringUtil::repeat(
                                literal!("."),
                                length_nm - ((metamodelica::arrayGet(NM_.clone(), i)?).len() as i32),
                            )?);
                            __mm_s.push_str(&*metamodelica::arrayGet(II_.clone(), i)?);
                            __mm_s.push_str(&*literal!(" "));
                            __mm_s.push_str(&*StringUtil::repeat(
                                literal!("."),
                                length_ii - ((metamodelica::arrayGet(II_.clone(), i)?).len() as i32),
                            )?);
                            __mm_s.push_str(&*metamodelica::arrayGet(XX_.clone(), i)?);
                            __mm_s.push_str(&*literal!(" "));
                            __mm_s.push_str(&*StringUtil::repeat(
                                literal!("."),
                                length_xx - ((metamodelica::arrayGet(XX_.clone(), i)?).len() as i32),
                            )?);
                            __mm_s.push_str(&*metamodelica::arrayGet(QQ_.clone(), i)?);
                            __mm_s.push_str(&*literal!(" "));
                            __mm_s.push_str(&*StringUtil::repeat(
                                literal!("."),
                                length_qq - ((metamodelica::arrayGet(QQ_.clone(), i)?).len() as i32),
                            )?);
                            __mm_s.push_str(&*literal!("\n"));
                            ArcStr::from(__mm_s)
                        };
                    }
                    r#str
                }
                _ => toString(adj, r#str)?,
            })
        });
        Ok(r#str)
    }

    pub(crate) fn dependencyString(mut adj: &metamodelica::Ref<Matrix>, mut r#str: ArcStr) -> Result<ArcStr> {
        let mut r#str: ArcStr = r#str;
        r#str = ({
            let mut f: metamodelica::List<ArcStr> = metamodelica::nil();
            let mut r: metamodelica::List<ArcStr> = metamodelica::nil();
            let mut e: metamodelica::List<ArcStr> = metamodelica::nil();
            let mut a: metamodelica::List<ArcStr> = metamodelica::nil();
            let mut s: metamodelica::List<ArcStr> = metamodelica::nil();
            let mut k: metamodelica::List<ArcStr> = metamodelica::nil();
            (match &**adj {
                FULL { .. } => {
                    let mut F: metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>>;
                    let mut R: metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>>;
                    let mut E: metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>>;
                    let mut A: metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>>;
                    let mut S: metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>>;
                    let mut K: metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>>;
                    let mut names: metamodelica::Array<ArcStr>;
                    let mut types: metamodelica::Array<ArcStr>;
                    let mut F_: metamodelica::Array<ArcStr>;
                    let mut R_: metamodelica::Array<ArcStr>;
                    let mut E_: metamodelica::Array<ArcStr>;
                    let mut A_: metamodelica::Array<ArcStr>;
                    let mut S_: metamodelica::Array<ArcStr>;
                    let mut K_: metamodelica::Array<ArcStr>;
                    let mut length1: i32;
                    let mut length2: i32;
                    let mut lengthf: i32;
                    let mut lengthr: i32;
                    let mut lengthe: i32;
                    let mut lengtha: i32;
                    let mut lengths: i32;
                    let mut lengthk: i32;
                    r#str = {
                        let mut __mm_s = String::new();
                        __mm_s.push_str(&*StringUtil::headline_2(
                            &({
                                let mut __mm_s = String::new();
                                __mm_s.push_str(&*r#str);
                                __mm_s.push_str(&*literal!(" Dependency Adjacency Matrix"));
                                ArcStr::from(__mm_s)
                            }),
                        )?);
                        __mm_s.push_str(&*literal!("\n"));
                        ArcStr::from(__mm_s)
                    };
                    types = metamodelica::arrayFromVec(
                        ({
                            let mut __acc: metamodelica::List<ArcStr> = metamodelica::nil();
                            for mut name in (var_field!((**adj).equation_names, Matrix::FULL).clone())
                                .borrow()
                                .iter()
                            {
                                let __x = intString(Type::sizeOf(
                                    &(ComponentRef::getSubscriptedType(&(name.clone()), false)?),
                                    true,
                                )?);
                                __acc = cons(__x, __acc);
                            }
                            __acc.reverse()
                        })
                        .into_iter()
                        .cloned()
                        .collect(),
                    );
                    names = metamodelica::arrayFromVec(
                        ({
                            let mut __acc: metamodelica::List<ArcStr> = metamodelica::nil();
                            for mut name in (var_field!((**adj).equation_names, Matrix::FULL).clone())
                                .borrow()
                                .iter()
                            {
                                let __x = ComponentRef::toString(&(name.clone()))?;
                                __acc = cons(__x, __acc);
                            }
                            __acc.reverse()
                        })
                        .into_iter()
                        .cloned()
                        .collect(),
                    );
                    for mut i in ({
                        let __s = metamodelica::arrayLength(names.clone());
                        let __e = 1;
                        (0i32..)
                            .map(move |__k| __s + __k * (-1))
                            .take_while(move |&__v| __v >= __e)
                    }) {
                        (F, R, E, A, S, K) = Dependency::categorize(
                            &(UnorderedSet::toList(
                                ({
                                    let __elt = (*metamodelica::index_checked(
                                        &var_field!((**adj).occurrences, Matrix::FULL).borrow(),
                                        i,
                                    )?)
                                    .clone();
                                    __elt
                                }),
                            )),
                            ({
                                let __elt = (*metamodelica::index_checked(
                                    &var_field!((**adj).dependencies, Matrix::FULL).borrow(),
                                    i,
                                )?)
                                .clone();
                                __elt
                            }),
                            ({
                                let __elt = (*metamodelica::index_checked(
                                    &var_field!((**adj).repetitions, Matrix::FULL).borrow(),
                                    i,
                                )?)
                                .clone();
                                __elt
                            }),
                        )?;
                        f = metamodelica::cons(
                            List::toStringCustom(
                                F,
                                &move |__a0: metamodelica::Ref<ComponentRef::NFComponentRef>| {
                                    ComponentRef::toString(&__a0)
                                },
                                literal!("[!]"),
                                literal!("{"),
                                literal!(","),
                                literal!("}"),
                                false,
                                0,
                            )?,
                            f,
                        );
                        r = metamodelica::cons(
                            List::toStringCustom(
                                R,
                                &move |__a0: metamodelica::Ref<ComponentRef::NFComponentRef>| {
                                    ComponentRef::toString(&__a0)
                                },
                                literal!("[-]"),
                                literal!("{"),
                                literal!(","),
                                literal!("}"),
                                false,
                                0,
                            )?,
                            r,
                        );
                        e = metamodelica::cons(
                            List::toStringCustom(
                                E,
                                &move |__a0: metamodelica::Ref<ComponentRef::NFComponentRef>| {
                                    ComponentRef::toString(&__a0)
                                },
                                literal!("[+]"),
                                literal!("{"),
                                literal!(","),
                                literal!("}"),
                                false,
                                0,
                            )?,
                            e,
                        );
                        a = metamodelica::cons(
                            List::toStringCustom(
                                A,
                                &move |__a0: metamodelica::Ref<ComponentRef::NFComponentRef>| {
                                    ComponentRef::toString(&__a0)
                                },
                                literal!("[:]"),
                                literal!("{"),
                                literal!(","),
                                literal!("}"),
                                false,
                                0,
                            )?,
                            a,
                        );
                        s = metamodelica::cons(
                            List::toStringCustom(
                                S,
                                &move |__a0: metamodelica::Ref<ComponentRef::NFComponentRef>| {
                                    ComponentRef::toString(&__a0)
                                },
                                literal!("[.]"),
                                literal!("{"),
                                literal!(","),
                                literal!("}"),
                                false,
                                0,
                            )?,
                            s,
                        );
                        k = metamodelica::cons(
                            List::toStringCustom(
                                K,
                                &move |__a0: metamodelica::Ref<ComponentRef::NFComponentRef>| {
                                    ComponentRef::toString(&__a0)
                                },
                                literal!("[o]"),
                                literal!("{"),
                                literal!(","),
                                literal!("}"),
                                false,
                                0,
                            )?,
                            k,
                        );
                    }
                    F_ = metamodelica::arrayFromVec(f.into_iter().cloned().collect());
                    R_ = metamodelica::arrayFromVec(r.into_iter().cloned().collect());
                    E_ = metamodelica::arrayFromVec(e.into_iter().cloned().collect());
                    A_ = metamodelica::arrayFromVec(a.into_iter().cloned().collect());
                    S_ = metamodelica::arrayFromVec(s.into_iter().cloned().collect());
                    K_ = metamodelica::arrayFromVec(k.into_iter().cloned().collect());
                    length1 = ({
                        let mut __acc: Option<i32> = None;
                        for mut ty in (types.clone()).borrow().iter() {
                            let __x = ((ty).len() as i32);
                            __acc = Some(match __acc {
                                None => __x,
                                Some(__cur) => {
                                    if __x > __cur {
                                        __x
                                    } else {
                                        __cur
                                    }
                                }
                            });
                        }
                        __acc.unwrap_or((-i32::MAX))
                    }) + 1;
                    length2 = ({
                        let mut __acc: Option<i32> = None;
                        for mut name in (names.clone()).borrow().iter() {
                            let __x = ((name).len() as i32);
                            __acc = Some(match __acc {
                                None => __x,
                                Some(__cur) => {
                                    if __x > __cur {
                                        __x
                                    } else {
                                        __cur
                                    }
                                }
                            });
                        }
                        __acc.unwrap_or((-i32::MAX))
                    }) + 3;
                    lengthf = ({
                        let mut __acc: Option<i32> = None;
                        for mut st in (F_.clone()).borrow().iter() {
                            let __x = ((st).len() as i32);
                            __acc = Some(match __acc {
                                None => __x,
                                Some(__cur) => {
                                    if __x > __cur {
                                        __x
                                    } else {
                                        __cur
                                    }
                                }
                            });
                        }
                        __acc.unwrap_or((-i32::MAX))
                    });
                    lengthr = ({
                        let mut __acc: Option<i32> = None;
                        for mut st in (R_.clone()).borrow().iter() {
                            let __x = ((st).len() as i32);
                            __acc = Some(match __acc {
                                None => __x,
                                Some(__cur) => {
                                    if __x > __cur {
                                        __x
                                    } else {
                                        __cur
                                    }
                                }
                            });
                        }
                        __acc.unwrap_or((-i32::MAX))
                    });
                    lengthe = ({
                        let mut __acc: Option<i32> = None;
                        for mut st in (E_.clone()).borrow().iter() {
                            let __x = ((st).len() as i32);
                            __acc = Some(match __acc {
                                None => __x,
                                Some(__cur) => {
                                    if __x > __cur {
                                        __x
                                    } else {
                                        __cur
                                    }
                                }
                            });
                        }
                        __acc.unwrap_or((-i32::MAX))
                    });
                    lengtha = ({
                        let mut __acc: Option<i32> = None;
                        for mut st in (A_.clone()).borrow().iter() {
                            let __x = ((st).len() as i32);
                            __acc = Some(match __acc {
                                None => __x,
                                Some(__cur) => {
                                    if __x > __cur {
                                        __x
                                    } else {
                                        __cur
                                    }
                                }
                            });
                        }
                        __acc.unwrap_or((-i32::MAX))
                    });
                    lengths = ({
                        let mut __acc: Option<i32> = None;
                        for mut st in (S_.clone()).borrow().iter() {
                            let __x = ((st).len() as i32);
                            __acc = Some(match __acc {
                                None => __x,
                                Some(__cur) => {
                                    if __x > __cur {
                                        __x
                                    } else {
                                        __cur
                                    }
                                }
                            });
                        }
                        __acc.unwrap_or((-i32::MAX))
                    });
                    lengthk = ({
                        let mut __acc: Option<i32> = None;
                        for mut st in (K_.clone()).borrow().iter() {
                            let __x = ((st).len() as i32);
                            __acc = Some(match __acc {
                                None => __x,
                                Some(__cur) => {
                                    if __x > __cur {
                                        __x
                                    } else {
                                        __cur
                                    }
                                }
                            });
                        }
                        __acc.unwrap_or((-i32::MAX))
                    });
                    for mut i in 1..=metamodelica::arrayLength(names.clone()) {
                        r#str = {
                            let mut __mm_s = String::new();
                            __mm_s.push_str(&*r#str);
                            __mm_s.push_str(&*metamodelica::arrayGet(types.clone(), i)?);
                            __mm_s.push_str(&*literal!(" "));
                            __mm_s.push_str(&*StringUtil::repeat(
                                literal!("."),
                                length1 - ((metamodelica::arrayGet(types.clone(), i)?).len() as i32),
                            )?);
                            __mm_s.push_str(&*literal!(" "));
                            __mm_s.push_str(&*metamodelica::arrayGet(names.clone(), i)?);
                            __mm_s.push_str(&*literal!(" "));
                            __mm_s.push_str(&*StringUtil::repeat(
                                literal!("."),
                                length2 - ((metamodelica::arrayGet(names.clone(), i)?).len() as i32),
                            )?);
                            __mm_s.push_str(&*metamodelica::arrayGet(K_.clone(), i)?);
                            __mm_s.push_str(&*literal!(" "));
                            __mm_s.push_str(&*StringUtil::repeat(
                                literal!("."),
                                lengthk - ((metamodelica::arrayGet(K_.clone(), i)?).len() as i32),
                            )?);
                            __mm_s.push_str(&*metamodelica::arrayGet(S_.clone(), i)?);
                            __mm_s.push_str(&*literal!(" "));
                            __mm_s.push_str(&*StringUtil::repeat(
                                literal!("."),
                                lengths - ((metamodelica::arrayGet(S_.clone(), i)?).len() as i32),
                            )?);
                            __mm_s.push_str(&*metamodelica::arrayGet(A_.clone(), i)?);
                            __mm_s.push_str(&*literal!(" "));
                            __mm_s.push_str(&*StringUtil::repeat(
                                literal!("."),
                                lengtha - ((metamodelica::arrayGet(A_.clone(), i)?).len() as i32),
                            )?);
                            __mm_s.push_str(&*metamodelica::arrayGet(E_.clone(), i)?);
                            __mm_s.push_str(&*literal!(" "));
                            __mm_s.push_str(&*StringUtil::repeat(
                                literal!("."),
                                lengthe - ((metamodelica::arrayGet(E_.clone(), i)?).len() as i32),
                            )?);
                            __mm_s.push_str(&*metamodelica::arrayGet(R_.clone(), i)?);
                            __mm_s.push_str(&*literal!(" "));
                            __mm_s.push_str(&*StringUtil::repeat(
                                literal!("."),
                                lengthr - ((metamodelica::arrayGet(R_.clone(), i)?).len() as i32),
                            )?);
                            __mm_s.push_str(&*metamodelica::arrayGet(F_.clone(), i)?);
                            __mm_s.push_str(&*literal!(" "));
                            __mm_s.push_str(&*StringUtil::repeat(
                                literal!("."),
                                lengthf - ((metamodelica::arrayGet(F_.clone(), i)?).len() as i32),
                            )?);
                            __mm_s.push_str(&*literal!("\n"));
                            ArcStr::from(__mm_s)
                        };
                    }
                    r#str
                }
                _ => toString(adj, r#str)?,
            })
        });
        Ok(r#str)
    }

    pub(crate) fn getStrictness(mut adj: &metamodelica::Ref<Matrix>) -> Result<MatrixStrictness> {
        let mut st: MatrixStrictness;
        st = (match &**adj {
            FULL { .. } => MatrixStrictness::FULL.clone(),
            FINAL { st: __adj_st, .. } => __adj_st.clone(),
            EMPTY { st: __adj_st } => __adj_st.clone(),
            _ => return Err("fail"),
        });
        Ok(st)
    }

    pub(crate) fn isEmpty(mut adj: &metamodelica::Ref<Matrix>) -> bool {
        let mut b: bool;
        b = (match &**adj {
            EMPTY { .. } => true,
            _ => false,
        });
        b
    }

    pub(crate) fn getMappingOpt(mut adj: &metamodelica::Ref<Matrix>) -> Option<metamodelica::Ref<Mapping::Mapping>> {
        let mut mapping: Option<metamodelica::Ref<Mapping::Mapping>>;
        mapping = (match &**adj {
            FULL {
                mapping: __adj_mapping, ..
            } => Some(__adj_mapping.clone()),
            FINAL {
                mapping: __adj_mapping, ..
            } => Some(__adj_mapping.clone()),
            _ => None,
        });
        mapping
    }

    pub(crate) fn nonZeroCount(mut adj: &metamodelica::Ref<Matrix>) -> Result<i32> {
        let mut count: i32;
        count = (match &**adj {
            FINAL { m: __adj_m, .. } => IntMatrix::nonZeroCount(metamodelica::AsArg::as_arg(&__adj_m)),
            EMPTY { .. } => 0,
            _ => {
                Error::addMessage(
                    Error::INTERNAL_ERROR.clone(),
                    list![{
                        let mut __mm_s = String::new();
                        __mm_s.push_str(&*literal!("NBAdjacency.Matrix.nonZeroCount"));
                        __mm_s.push_str(&*literal!(" failed because of unknown matrix type."));
                        ArcStr::from(__mm_s)
                    }],
                )?;
                return Err("fail");
            }
        });
        Ok(count)
    }

    fn fullString(
        mut cref: metamodelica::Ref<ComponentRef::NFComponentRef>,
        mut dep_map: metamodelica::Ref<
            UnorderedMap::UnorderedMap<
                metamodelica::Ref<ComponentRef::NFComponentRef>,
                metamodelica::Ref<Dependency::Dependency>,
            >,
        >,
        mut sol_map: metamodelica::Ref<
            UnorderedMap::UnorderedMap<
                metamodelica::Ref<ComponentRef::NFComponentRef>,
                metamodelica::Ref<Solvability::Solvability>,
            >,
        >,
        mut rep_set: metamodelica::Ref<UnorderedSet::UnorderedSet<metamodelica::Ref<ComponentRef::NFComponentRef>>>,
    ) -> Result<ArcStr> {
        let mut r#str: ArcStr = {
            let mut __mm_s = String::new();
            __mm_s.push_str(&*ComponentRef::toString(&cref)?);
            __mm_s.push_str(&*literal!("["));
            ArcStr::from(__mm_s)
        };
        r#str = {
            let mut __mm_s = String::new();
            __mm_s.push_str(&*r#str);
            __mm_s.push_str(&*Solvability::toString(
                &(UnorderedMap::getSafe(
                    cref.clone(),
                    sol_map,
                    metamodelica::sourceInfo!("NBackEnd/Util/NBAdjacency.mo"),
                )?),
            )?);
            __mm_s.push_str(&*literal!("|"));
            __mm_s.push_str(&*Dependency::toString(
                &(UnorderedMap::getSafe(
                    cref.clone(),
                    dep_map,
                    metamodelica::sourceInfo!("NBackEnd/Util/NBAdjacency.mo"),
                )?),
            )?);
            ArcStr::from(__mm_s)
        };
        if UnorderedSet::contains(cref, rep_set)? {
            r#str = {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*r#str);
                __mm_s.push_str(&*literal!("+"));
                ArcStr::from(__mm_s)
            };
        }
        r#str = {
            let mut __mm_s = String::new();
            __mm_s.push_str(&*r#str);
            __mm_s.push_str(&*literal!("]"));
            ArcStr::from(__mm_s)
        };
        Ok(r#str)
    }

    fn sparsityString(
        mut cref: metamodelica::Ref<ComponentRef::NFComponentRef>,
        mut dep_map: metamodelica::Ref<
            UnorderedMap::UnorderedMap<
                metamodelica::Ref<ComponentRef::NFComponentRef>,
                metamodelica::Ref<Dependency::Dependency>,
            >,
        >,
        mut rep_set: metamodelica::Ref<UnorderedSet::UnorderedSet<metamodelica::Ref<ComponentRef::NFComponentRef>>>,
    ) -> Result<ArcStr> {
        let mut r#str: ArcStr = {
            let mut __mm_s = String::new();
            __mm_s.push_str(&*ComponentRef::toString(&cref)?);
            __mm_s.push_str(&*literal!("["));
            ArcStr::from(__mm_s)
        };
        r#str = {
            let mut __mm_s = String::new();
            __mm_s.push_str(&*r#str);
            __mm_s.push_str(&*Dependency::toString(
                &(UnorderedMap::getSafe(
                    cref.clone(),
                    dep_map,
                    metamodelica::sourceInfo!("NBackEnd/Util/NBAdjacency.mo"),
                )?),
            )?);
            ArcStr::from(__mm_s)
        };
        if UnorderedSet::contains(cref, rep_set)? {
            r#str = {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*r#str);
                __mm_s.push_str(&*literal!("+"));
                ArcStr::from(__mm_s)
            };
        }
        r#str = {
            let mut __mm_s = String::new();
            __mm_s.push_str(&*r#str);
            __mm_s.push_str(&*literal!("]"));
            ArcStr::from(__mm_s)
        };
        Ok(r#str)
    }

    fn dimsString(mut dims: metamodelica::List<metamodelica::Ref<Dimension::NFDimension>>) -> Result<ArcStr> {
        let mut r#str: ArcStr;
        r#str = (::match_deref::match_deref! { match &(dims.clone()) {
            Deref @ metamodelica::ListNode::Nil => literal!("{1}"),
            _ => List::toString(({
            let mut __acc: metamodelica::List<i32> = metamodelica::nil();
            for mut d in (dims).into_iter().cloned() {
                let __x = Dimension::size(&(d.clone()), true)?;
                __acc = cons(__x, __acc);
            }
            __acc.reverse()
        }), &fnptr!(intString, i32), List::Style::FLAT_CURLY.clone())?,
            _ => unreachable!("match_deref! exhaustiveness placeholder"),
        } });
        Ok(r#str)
    }

    fn initialize(
        mut mapping: metamodelica::Ref<Mapping::Mapping>,
        mut st: MatrixStrictness,
    ) -> metamodelica::Ref<Matrix> {
        let mut adj: metamodelica::Ref<Matrix>;
        let mut eqn_scalar_size: i32;
        let mut var_scalar_size: i32;
        eqn_scalar_size = metamodelica::arrayLength(mapping.eqn_StA.clone());
        var_scalar_size = metamodelica::arrayLength(mapping.var_StA.clone());
        if eqn_scalar_size > 0 || var_scalar_size > 0 {
            adj = metamodelica::Ref::new(Matrix::FINAL {
                m: IntMatrix::new(eqn_scalar_size),
                mT: IntMatrix::new(var_scalar_size),
                mapping: mapping,
                modes: Modes::new(),
                st: st,
            });
        } else {
            adj = metamodelica::Ref::new(Matrix::EMPTY { st: st });
        }
        adj
    }

    fn estimateEntries(
        mut occ: metamodelica::Array<
            metamodelica::Ref<UnorderedSet::UnorderedSet<metamodelica::Ref<ComponentRef::NFComponentRef>>>,
        >,
        mut mapping: &metamodelica::Ref<Mapping::Mapping>,
        mut indices: &metamodelica::List<i32>,
    ) -> Result<i32> {
        let __ab_occ = occ.borrow();
        let mut n: i32 = 0;
        let mut sz: i32;
        for mut i in &**indices {
            (_, sz) = ({
                let __elt = (*metamodelica::index_checked(&mapping.eqn_AtS.borrow(), i.clone())?).clone();
                __elt
            });
            n = n + UnorderedSet::size((*metamodelica::index_checked(&__ab_occ, i.clone())?).clone()) * sz;
        }
        Ok(n)
    }

    fn upgradeRow(
        mut eqn_ptr: Pointer::Pointer<metamodelica::Ref<Equation::Equation>>,
        mut eqn_arr_idx: i32,
        mut dependencies: metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>>,
        mut dep: metamodelica::Ref<
            UnorderedMap::UnorderedMap<
                metamodelica::Ref<ComponentRef::NFComponentRef>,
                metamodelica::Ref<Dependency::Dependency>,
            >,
        >,
        mut rep: metamodelica::Ref<UnorderedSet::UnorderedSet<metamodelica::Ref<ComponentRef::NFComponentRef>>>,
        mut map: metamodelica::Ref<UnorderedMap::UnorderedMap<metamodelica::Ref<ComponentRef::NFComponentRef>, i32>>,
        mut fullmap: metamodelica::Ref<
            UnorderedMap::UnorderedMap<metamodelica::Ref<ComponentRef::NFComponentRef>, i32>,
        >,
        mut m: &IntMatrix::Builder,
        mut mapping: &metamodelica::Ref<Mapping::Mapping>,
        mut modes: metamodelica::Ref<Vector::Vector<metamodelica::Ref<Mode::Mode>>>,
        mut iter_: &metamodelica::Ref<Iterator::Iterator>,
    ) -> Result<()> {
        let mut eqn_scal_idx: i32;
        let mut eqn_size: i32;
        let mut row: metamodelica::List<i32>;
        let mut eqn: metamodelica::Ref<Equation::Equation> = Pointer::access(eqn_ptr.clone());
        let mut iter: metamodelica::Ref<Iterator::Iterator> = BEquation::Equation::getForIterator(&eqn);
        let mut ty: metamodelica::Ref<Type::NFType> = BEquation::Equation::getType(&eqn, true)?;
        let mut names: metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>>;
        let mut ranges: metamodelica::List<metamodelica::Ref<Expression::NFExpression>>;
        let mut maps: metamodelica::List<Option<metamodelica::Ref<Iterator::Iterator>>>;
        match '__try0: {
            if BEquation::Equation::isAlgorithm(eqn_ptr.clone()) || BEquation::Equation::isIfEquation(eqn_ptr.clone()) {
                (eqn_scal_idx, eqn_size) = ({
                    let __elt = (*unwrap_break_err!(metamodelica::index_checked(&mapping.eqn_AtS.borrow(), eqn_arr_idx), '__try0)).clone();
                    __elt
                });
                row = unwrap_break_err!(Slice::upgradeRowFull(dependencies.clone(), map.clone(), mapping), '__try0);
                for mut i in 0..=eqn_size - 1 {
                    IntMatrix::builderAddList(m, eqn_scal_idx + i, row.clone(), 0);
                }
            } else {
                if !(BEquation::Iterator::isEmpty(iter_)) {
                    (names, ranges, maps) = BEquation::Iterator::getFrames(iter_);
                    iter = BEquation::Iterator::addFrames(
                        iter.clone(),
                        List::zip3(names.clone(), ranges.clone(), maps.clone()),
                    );
                }
                unwrap_break_err!(Slice::upgradeRow(unwrap_break_err!(BEquation::Equation::getEqnName(eqn_ptr.clone()), '__try0), eqn_arr_idx, &iter, ty.clone(), &dependencies, dep.clone(), rep.clone(), map.clone(), fullmap.clone(), m, mapping, modes.clone()), '__try0);
            }
            Ok::<(), &'static str>(())
        } {
            Ok(()) => {}
            Err(__try0_err) => {
                Error::addMessage(
                    Error::INTERNAL_ERROR.clone(),
                    list![{
                        let mut __mm_s = String::new();
                        __mm_s.push_str(&*literal!("NBAdjacency.Matrix.upgradeRow"));
                        __mm_s.push_str(&*literal!(" failed for:\n"));
                        __mm_s.push_str(&*BEquation::Equation::pointerToString(eqn_ptr.clone(), literal!(""))?);
                        ArcStr::from(__mm_s)
                    }],
                )?;
                return Err(__try0_err);
            }
        }
        Ok(())
    }
}

pub mod Dependency {
    use super::*;
    /// the dependency kind to show how a component reference occurs in an equation.
    ///    for each dimension there has to be one dependency kind.
    #[derive(Clone, Debug, Eq, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
    pub struct Dependency {
        pub skips: metamodelica::Array<metamodelica::List<i32>>,
        pub kinds: metamodelica::List<Kind>,
    }

    impl metamodelica::gc::MMTrace for Dependency {
        fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
            metamodelica::gc::MMTrace::mm_accept(&self.skips, __mmv)?;
            metamodelica::gc::MMTrace::mm_accept(&self.kinds, __mmv)?;
            Ok(())
        }
    }
    impl Default for Dependency {
        fn default() -> Self {
            Self {
                skips: Default::default(),
                kinds: Default::default(),
            }
        }
    }

    pub type DEPENDENCY = Dependency;

    #[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, metamodelica::ReferenceEq)]
    #[repr(i32)]
    pub(crate) enum Kind {
        REGULAR = 1,
        REDUCTION = 2,
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
            Self::REGULAR
        }
    }

    pub(crate) fn toString(mut dep: &metamodelica::Ref<Dependency>) -> Result<ArcStr> {
        fn kindString(mut kind: Kind) -> ArcStr {
            let mut r#str: ArcStr;
            r#str = (match kind {
                Kind::REGULAR => literal!(":"),
                _ => literal!("-"),
            });
            r#str
        }

        let mut r#str: ArcStr;
        let mut str1: ArcStr;
        let mut str2: ArcStr;
        str1 = Array::toString(
            dep.skips.clone(),
            &({
                let __pe_b1: Arc<dyn ::std::ops::Fn(_) -> Result<ArcStr> + 'static> =
                    (std::sync::Arc::new(fnptr!(intString, i32))
                        as std::sync::Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>);
                let __pe_b2 = List::Style::FLAT_CURLY.clone();
                move |__pe_a0| List::toString(__pe_a0, &*__pe_b1, __pe_b2.clone())
            }),
            literal!(""),
            literal!(""),
            literal!(", "),
            literal!(""),
            true,
            0,
        )?;
        str2 = List::toString(dep.kinds.clone(), &fnptr!(kindString, Kind), List::Style::FLAT.clone())?;
        r#str = if (metamodelica::stringEq(&str1, &(literal!(""))) || metamodelica::stringEq(&str2, &(literal!("")))) {
            {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*str1);
                __mm_s.push_str(&*str2);
                ArcStr::from(__mm_s)
            }
        } else {
            {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*str1);
                __mm_s.push_str(&*literal!(", "));
                __mm_s.push_str(&*str2);
                ArcStr::from(__mm_s)
            }
        };
        r#str = {
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("{"));
            __mm_s.push_str(&*r#str);
            __mm_s.push_str(&*literal!("}"));
            ArcStr::from(__mm_s)
        };
        Ok(r#str)
    }

    pub(crate) fn toBoolean(mut dep: &metamodelica::Ref<Dependency>) -> metamodelica::List<bool> {
        let mut b: metamodelica::List<bool> = ({
            let mut __acc: metamodelica::List<bool> = metamodelica::nil();
            for mut k in (dep.kinds.clone()).into_iter().cloned() {
                let __x = !(isReductionKind(k.clone()));
                __acc = cons(__x, __acc);
            }
            __acc.reverse()
        });
        b
    }

    pub(crate) fn create(
        mut sub_ty: metamodelica::Ref<Type::NFType>,
        mut depth: i32,
    ) -> Result<metamodelica::Ref<Dependency>> {
        let mut dep: metamodelica::Ref<Dependency>;
        dep = metamodelica::Ref::new(Dependency {
            skips: arrayCreate(depth, metamodelica::nil()),
            kinds: ({
                let mut __acc: metamodelica::List<Kind> = metamodelica::nil();
                for mut dim in (Type::arrayDims(sub_ty)).into_iter().cloned() {
                    if !(!(Dimension::isOne(&(dim.clone()))?)) {
                        continue;
                    }
                    let __x = Kind::REGULAR.clone();
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            }),
        });
        Ok(dep)
    }

    pub(crate) fn update(
        mut cref: metamodelica::Ref<ComponentRef::NFComponentRef>,
        mut num: i32,
        mut reverse: bool,
        mut map: metamodelica::Ref<
            UnorderedMap::UnorderedMap<metamodelica::Ref<ComponentRef::NFComponentRef>, metamodelica::Ref<Dependency>>,
        >,
    ) -> Result<()> {
        fn makeNewKinds(mut kinds: metamodelica::List<Kind>, mut num: i32) -> (metamodelica::List<Kind>, i32) {
            let mut kinds: metamodelica::List<Kind> = kinds;
            let mut num: i32 = num;
            (kinds, num) = (::match_deref::match_deref! { match &((kinds.clone(), num)) {
                (_, 0) => {
                    (kinds, num)
                },
                (Deref @ metamodelica::ListNode::Cons { head: _, tail: rest }, _) => {
                    (kinds, num) = makeNewKinds(rest.clone(), num - 1);
                    (metamodelica::cons(Kind::REDUCTION.clone(), kinds), num)
                },
                _ => {
                    (kinds, num)
                },
                _ => unreachable!("match_deref! exhaustiveness placeholder"),
            } });
            (kinds, num)
        }

        let mut opt_dep: Option<metamodelica::Ref<Dependency>> = UnorderedMap::get(cref.clone(), map.clone())?;
        let mut dep: metamodelica::Ref<Dependency>;
        let mut kinds: metamodelica::List<Kind>;
        let mut res: i32;
        if (opt_dep).is_some() {
            let __pa0 = ::match_deref::match_deref! { match &(opt_dep) {
                Some(__pa0) => __pa0.clone(),
                _ => return Err("pattern mismatch"),
            } };
            dep = metamodelica::Own::own(__pa0);
            if reverse {
                (kinds, res) = makeNewKinds(dep.kinds.clone().reverse(), num);
                assign_field!(dep.kinds = kinds.reverse());
            } else {
                (kinds, res) = makeNewKinds(dep.kinds.clone(), num);
                assign_field!(dep.kinds = kinds);
            }
            if res > 0 {
                removeSkips(cref.clone(), map.clone(), res, reverse)?;
            }
            UnorderedMap::add(cref, dep, map)?;
        } else {
            Error::addMessage(
                Error::INTERNAL_ERROR.clone(),
                list![{
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!("NBAdjacency.Dependency.update"));
                    __mm_s.push_str(&*literal!(" failed because cref "));
                    __mm_s.push_str(&*ComponentRef::toString(&cref)?);
                    __mm_s.push_str(&*literal!(" was not found in the map."));
                    ArcStr::from(__mm_s)
                }],
            )?;
            return Err("fail");
        }
        Ok(())
    }

    pub(crate) fn skip(
        mut cref: metamodelica::Ref<ComponentRef::NFComponentRef>,
        mut depth: i32,
        mut sk: i32,
        mut map: metamodelica::Ref<
            UnorderedMap::UnorderedMap<metamodelica::Ref<ComponentRef::NFComponentRef>, metamodelica::Ref<Dependency>>,
        >,
    ) -> Result<()> {
        let mut opt_dep: Option<metamodelica::Ref<Dependency>> = UnorderedMap::get(cref.clone(), map.clone())?;
        let mut dep: metamodelica::Ref<Dependency>;
        if (opt_dep).is_some() {
            let __pa0 = ::match_deref::match_deref! { match &(opt_dep) {
                Some(__pa0) => __pa0.clone(),
                _ => return Err("pattern mismatch"),
            } };
            dep = metamodelica::Own::own(__pa0);
            if metamodelica::arrayLength(dep.skips.clone()) >= depth {
                metamodelica::arrayUpdate(
                    dep.skips.clone(),
                    depth,
                    UnorderedSet::unique_list(
                        metamodelica::cons(
                            sk,
                            ({
                                let __elt = (*metamodelica::index_checked(&dep.skips.borrow(), depth)?).clone();
                                __elt
                            }),
                        ),
                        std::sync::Arc::new(fnptr!(Util::id, _)),
                        (std::sync::Arc::new(fnptr!(intEq, i32, i32))
                            as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>),
                    )?,
                )?;
            } else {
                if Flags::isSet(Flags::FAILTRACE.clone())? {
                    Error::addCompilerWarning({
                        let mut __mm_s = String::new();
                        __mm_s.push_str(&*literal!("NBAdjacency.Dependency.skip"));
                        __mm_s.push_str(&*literal!(": Cref "));
                        __mm_s.push_str(&*ComponentRef::toString(&cref)?);
                        __mm_s.push_str(&*literal!(" was saved with depth "));
                        __mm_s.push_str(&*intString(metamodelica::arrayLength(dep.skips.clone())));
                        __mm_s.push_str(&*literal!(" but depth "));
                        __mm_s.push_str(&*intString(depth));
                        __mm_s.push_str(&*literal!(" was requested."));
                        ArcStr::from(__mm_s)
                    })?;
                }
            }
            UnorderedMap::add(cref, dep, map)?;
        } else {
            Error::addMessage(
                Error::INTERNAL_ERROR.clone(),
                list![{
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!("NBAdjacency.Dependency.skip"));
                    __mm_s.push_str(&*literal!(" failed because cref "));
                    __mm_s.push_str(&*ComponentRef::toString(&cref)?);
                    __mm_s.push_str(&*literal!(" was not found in the map."));
                    ArcStr::from(__mm_s)
                }],
            )?;
            return Err("fail");
        }
        Ok(())
    }

    pub(crate) fn removeSkips(
        mut cref: metamodelica::Ref<ComponentRef::NFComponentRef>,
        mut map: metamodelica::Ref<
            UnorderedMap::UnorderedMap<metamodelica::Ref<ComponentRef::NFComponentRef>, metamodelica::Ref<Dependency>>,
        >,
        mut num: i32,
        mut reverse: bool,
    ) -> Result<()> {
        let mut opt_dep: Option<metamodelica::Ref<Dependency>> = UnorderedMap::get(cref.clone(), map.clone())?;
        let mut dep: metamodelica::Ref<Dependency>;
        let mut rest: i32 = num;
        let mut i: i32 = 0;
        let mut len: i32;
        if (opt_dep).is_some() {
            let __pa0 = ::match_deref::match_deref! { match &(opt_dep) {
                Some(__pa0) => __pa0.clone(),
                _ => return Err("pattern mismatch"),
            } };
            dep = metamodelica::Own::own(__pa0);
            if num < 0 {
                for mut i in 1..=metamodelica::arrayLength(dep.skips.clone()) {
                    metamodelica::arrayUpdate(dep.skips.clone(), i, metamodelica::nil())?;
                }
            } else {
                i = if (reverse) {
                    metamodelica::arrayLength(dep.skips.clone())
                } else {
                    1
                };
                while rest > 0 && i > 0 && i < metamodelica::arrayLength(dep.skips.clone()) + 1 {
                    len = (({
                        let __elt = (*metamodelica::index_checked(&dep.skips.borrow(), i)?).clone();
                        __elt
                    })
                    .len() as i32);
                    if len <= rest {
                        metamodelica::arrayUpdate(dep.skips.clone(), i, metamodelica::nil())?;
                    } else if len > 0 {
                        if reverse {
                            metamodelica::arrayUpdate(
                                dep.skips.clone(),
                                i,
                                List::firstN(
                                    ({
                                        let __elt = (*metamodelica::index_checked(&dep.skips.borrow(), i)?).clone();
                                        __elt
                                    }),
                                    len - rest,
                                )?,
                            )?;
                        } else {
                            metamodelica::arrayUpdate(
                                dep.skips.clone(),
                                i,
                                List::lastN(
                                    ({
                                        let __elt = (*metamodelica::index_checked(&dep.skips.borrow(), i)?).clone();
                                        __elt
                                    }),
                                    len - rest,
                                )?,
                            )?;
                        }
                    }
                    rest = rest - len;
                    i = if (reverse) { i - 1 } else { i + 1 };
                }
            }
            UnorderedMap::add(cref, dep, map)?;
        } else {
            Error::addMessage(
                Error::INTERNAL_ERROR.clone(),
                list![{
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!("NBAdjacency.Dependency.removeSkips"));
                    __mm_s.push_str(&*literal!(" failed because cref "));
                    __mm_s.push_str(&*ComponentRef::toString(&cref)?);
                    __mm_s.push_str(&*literal!(" was not found in the map."));
                    ArcStr::from(__mm_s)
                }],
            )?;
            return Err("fail");
        }
        Ok(())
    }

    pub(crate) fn updateList(
        mut lst: &metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>>,
        mut num: i32,
        mut reverse: bool,
        mut map: metamodelica::Ref<
            UnorderedMap::UnorderedMap<metamodelica::Ref<ComponentRef::NFComponentRef>, metamodelica::Ref<Dependency>>,
        >,
    ) -> Result<()> {
        for mut cref in &**lst {
            update(cref.clone(), num, reverse, map.clone())?;
        }
        Ok(())
    }

    pub(crate) fn skipList(
        mut lst: &metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>>,
        mut depth: i32,
        mut sk: i32,
        mut map: metamodelica::Ref<
            UnorderedMap::UnorderedMap<metamodelica::Ref<ComponentRef::NFComponentRef>, metamodelica::Ref<Dependency>>,
        >,
    ) -> Result<()> {
        for mut cref in &**lst {
            skip(cref.clone(), depth, sk, map.clone())?;
        }
        Ok(())
    }

    pub(crate) fn removeSkipsList(
        mut lst: &metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>>,
        mut map: metamodelica::Ref<
            UnorderedMap::UnorderedMap<metamodelica::Ref<ComponentRef::NFComponentRef>, metamodelica::Ref<Dependency>>,
        >,
    ) -> Result<()> {
        for mut cref in &**lst {
            removeSkips(cref.clone(), map.clone(), -1, false)?;
        }
        Ok(())
    }

    pub(crate) fn addListFull(
        mut lst: &metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>>,
        mut depth: i32,
        mut map: metamodelica::Ref<
            UnorderedMap::UnorderedMap<metamodelica::Ref<ComponentRef::NFComponentRef>, metamodelica::Ref<Dependency>>,
        >,
        mut rep: metamodelica::Ref<UnorderedSet::UnorderedSet<metamodelica::Ref<ComponentRef::NFComponentRef>>>,
    ) -> Result<()> {
        let mut dep: metamodelica::Ref<Dependency>;
        for mut cref in &**lst {
            UnorderedMap::add(
                cref.clone(),
                create(
                    ComponentRef::getSubscriptedType(metamodelica::AsArg::as_arg(&cref), false)?,
                    depth,
                )?,
                map.clone(),
            )?;
            UnorderedSet::add(cref.clone(), rep.clone())?;
        }
        updateList(lst, -1, false, map)?;
        Ok(())
    }

    pub(crate) fn isReductionKind(mut kind: Kind) -> bool {
        let mut b: bool = kind == Kind::REDUCTION.clone();
        b
    }

    pub(crate) fn categorize(
        mut crefs: &metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>>,
        mut map: metamodelica::Ref<
            UnorderedMap::UnorderedMap<metamodelica::Ref<ComponentRef::NFComponentRef>, metamodelica::Ref<Dependency>>,
        >,
        mut rep_set: metamodelica::Ref<UnorderedSet::UnorderedSet<metamodelica::Ref<ComponentRef::NFComponentRef>>>,
    ) -> Result<(
        metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>>,
        metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>>,
        metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>>,
        metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>>,
        metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>>,
        metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>>,
    )> {
        let mut F: metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>> = metamodelica::nil();
        let mut R: metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>> = metamodelica::nil();
        let mut E: metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>> = metamodelica::nil();
        let mut A: metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>> = metamodelica::nil();
        let mut S: metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>> = metamodelica::nil();
        let mut K: metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>> = metamodelica::nil();
        let mut repeats: bool;
        for mut cref in &**crefs {
            repeats = UnorderedSet::contains(cref.clone(), rep_set.clone())?;
            let _ = (::match_deref::match_deref! { match &(UnorderedMap::getSafe(cref.clone(), map.clone(), metamodelica::sourceInfo!("NBackEnd/Util/NBAdjacency.mo"))?) {
                Deref @ Dependency { skips, .. } if (!(Array::all(skips.clone(), &fnptr!(listEmpty, _))?)) => {
                    K = metamodelica::cons(cref.clone(), K);
                    ()
                },
                Deref @ Dependency { kinds: Deref @ metamodelica::ListNode::Nil, .. } if (repeats) => {
                    E = metamodelica::cons(cref.clone(), E);
                    ()
                },
                Deref @ Dependency { kinds: Deref @ metamodelica::ListNode::Nil, .. } => {
                    S = metamodelica::cons(cref.clone(), S);
                    ()
                },
                Deref @ Dependency { kinds, .. } => {
                    if List::any(metamodelica::AsArg::as_arg(&kinds), &fnptr!(isReductionKind, Kind))? {
                        if repeats {
                            F = metamodelica::cons(cref.clone(), F);
                        } else {
                            R = metamodelica::cons(cref.clone(), R);
                        }
                    } else {
                        A = metamodelica::cons(cref.clone(), A);
                    }
                    ()
                },
                _ => {
                    ()
                },
                _ => unreachable!("match_deref! exhaustiveness placeholder"),
            } });
        }
        Ok((F, R, E, A, S, K))
    }

    pub(crate) fn convert(mut dep: &metamodelica::Ref<Dependency>) -> OldSimCode::Dependency {
        fn convertKind(mut kind: Kind) -> bool {
            let mut okind: bool = kind == Kind::REDUCTION.clone();
            okind
        }

        let mut odep: OldSimCode::Dependency;
        odep = OldSimCode::Dependency {
            skips: dep.skips.clone(),
            kinds: ({
                let mut __acc: metamodelica::List<bool> = metamodelica::nil();
                for mut kind in (dep.kinds.clone()).into_iter().cloned() {
                    let __x = convertKind(kind.clone());
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            }),
        };
        odep
    }
}

pub mod Solvability {
    use super::*;
    #[derive(Clone, Debug, Eq, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
    pub enum Solvability {
        UNKNOWN,
        UNSOLVABLE,
        IMPLICIT,
        EXPLICIT_NONLINEAR {
            /// true if it has a unique solution when solved
            unique: bool,
        },
        EXPLICIT_LINEAR {
            /// parameters we need to divide by to solve
            pars:
                Option<metamodelica::Ref<UnorderedSet::UnorderedSet<metamodelica::Ref<ComponentRef::NFComponentRef>>>>,
            /// variables we need to divide by to solve
            vars:
                Option<metamodelica::Ref<UnorderedSet::UnorderedSet<metamodelica::Ref<ComponentRef::NFComponentRef>>>>,
        },
    }
    impl metamodelica::gc::MMTrace for Solvability {
        fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
            match self {
                Solvability::UNKNOWN => Ok(()),
                Solvability::UNSOLVABLE => Ok(()),
                Solvability::IMPLICIT => Ok(()),
                Solvability::EXPLICIT_NONLINEAR { unique } => {
                    metamodelica::gc::MMTrace::mm_accept(unique, __mmv)?;
                    Ok(())
                }
                Solvability::EXPLICIT_LINEAR { pars, vars } => {
                    metamodelica::gc::MMTrace::mm_accept(pars, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(vars, __mmv)?;
                    Ok(())
                }
            }
        }
    }
    impl Solvability {
        pub fn interned_UNKNOWN() -> metamodelica::Ref<Solvability> {
            thread_local! {
                static INTERNED: metamodelica::Ref<Solvability> = metamodelica::Ref::new(Solvability::UNKNOWN);
            }
            INTERNED.with(|i| i.clone())
        }
        pub fn interned_UNSOLVABLE() -> metamodelica::Ref<Solvability> {
            thread_local! {
                static INTERNED: metamodelica::Ref<Solvability> = metamodelica::Ref::new(Solvability::UNSOLVABLE);
            }
            INTERNED.with(|i| i.clone())
        }
        pub fn interned_IMPLICIT() -> metamodelica::Ref<Solvability> {
            thread_local! {
                static INTERNED: metamodelica::Ref<Solvability> = metamodelica::Ref::new(Solvability::IMPLICIT);
            }
            INTERNED.with(|i| i.clone())
        }
    }
    pub fn interned_UNKNOWN() -> metamodelica::Ref<Solvability> {
        Solvability::interned_UNKNOWN()
    }
    pub fn interned_UNSOLVABLE() -> metamodelica::Ref<Solvability> {
        Solvability::interned_UNSOLVABLE()
    }
    pub fn interned_IMPLICIT() -> metamodelica::Ref<Solvability> {
        Solvability::interned_IMPLICIT()
    }
    impl Default for Solvability {
        fn default() -> Self {
            Self::UNKNOWN
        }
    }
    pub use self::Solvability::{EXPLICIT_LINEAR, EXPLICIT_NONLINEAR, IMPLICIT, UNKNOWN, UNSOLVABLE};
    pub(crate) fn toString(mut sol: &metamodelica::Ref<Solvability>) -> Result<ArcStr> {
        let mut r#str: ArcStr;
        r#str = (match &**sol {
            UNSOLVABLE { .. } => literal!("XX"),
            IMPLICIT { .. } => literal!("II"),
            EXPLICIT_NONLINEAR { unique: __sol_unique } => {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("N"));
                __mm_s.push_str(&*if (__sol_unique.clone()) {
                    literal!("+")
                } else {
                    literal!("-")
                });
                ArcStr::from(__mm_s)
            }
            EXPLICIT_LINEAR {
                pars: __sol_pars,
                vars: __sol_vars,
            } => {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("L"));
                __mm_s.push_str(&*if ((__sol_vars).is_some()) {
                    literal!("V")
                } else if ((__sol_pars).is_some()) {
                    literal!("P")
                } else {
                    literal!("C")
                });
                ArcStr::from(__mm_s)
            }
            UNKNOWN { .. } => literal!("||"),
            _ => {
                Error::addMessage(
                    Error::INTERNAL_ERROR.clone(),
                    list![{
                        let mut __mm_s = String::new();
                        __mm_s.push_str(&*literal!("NBAdjacency.Solvability.toString"));
                        __mm_s.push_str(&*literal!(" failed because of unknown solvability kind."));
                        ArcStr::from(__mm_s)
                    }],
                )?;
                return Err("fail");
            }
        });
        Ok(r#str)
    }

    pub(crate) fn rank(mut sol: &metamodelica::Ref<Solvability>) -> Result<i32> {
        let mut r: i32;
        r = (::match_deref::match_deref! { match sol {
            Deref @ UNSOLVABLE { .. } => 7,
            Deref @ IMPLICIT { .. } => 6,
            Deref @ EXPLICIT_NONLINEAR { unique: false } => 5,
            Deref @ EXPLICIT_NONLINEAR { .. } => 4,
            Deref @ EXPLICIT_LINEAR { vars: Some(_), .. } => 3,
            Deref @ EXPLICIT_LINEAR { pars: Some(_), .. } => 2,
            Deref @ EXPLICIT_LINEAR { .. } => 1,
            Deref @ UNKNOWN { .. } => 0,
            _ => {
                Error::addMessage(Error::INTERNAL_ERROR.clone(), list![{ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBAdjacency.Solvability.rank")); __mm_s.push_str(&*literal!(" failed because of unknown solvability kind.")); ArcStr::from(__mm_s) }])?;
                return Err("fail")
            },
            _ => unreachable!("match_deref! exhaustiveness placeholder"),
        } });
        Ok(r)
    }

    pub(crate) fn update(
        mut cref: metamodelica::Ref<ComponentRef::NFComponentRef>,
        mut sol: metamodelica::Ref<Solvability>,
        mut map: metamodelica::Ref<
            UnorderedMap::UnorderedMap<metamodelica::Ref<ComponentRef::NFComponentRef>, metamodelica::Ref<Solvability>>,
        >,
    ) -> Result<()> {
        if rank(&sol)?
            > rank(
                &(Util::getOptionOrDefault(
                    UnorderedMap::get(cref.clone(), map.clone())?,
                    crate::NBAdjacency::Solvability::interned_UNKNOWN(),
                )),
            )?
        {
            UnorderedMap::add(cref, sol, map)?;
        }
        Ok(())
    }

    pub(crate) fn updateList(
        mut lst: &metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>>,
        mut sol: metamodelica::Ref<Solvability>,
        mut map: metamodelica::Ref<
            UnorderedMap::UnorderedMap<metamodelica::Ref<ComponentRef::NFComponentRef>, metamodelica::Ref<Solvability>>,
        >,
    ) -> Result<()> {
        for mut cref in &**lst {
            update(cref.clone(), sol.clone(), map.clone())?;
        }
        Ok(())
    }

    pub(crate) fn categorize(
        mut crefs: &metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>>,
        mut map: metamodelica::Ref<
            UnorderedMap::UnorderedMap<metamodelica::Ref<ComponentRef::NFComponentRef>, metamodelica::Ref<Solvability>>,
        >,
    ) -> Result<(
        metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>>,
        metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>>,
        metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>>,
        metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>>,
        metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>>,
        metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>>,
        metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>>,
        metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>>,
    )> {
        let mut XX: metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>> = metamodelica::nil();
        let mut II: metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>> = metamodelica::nil();
        let mut NM: metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>> = metamodelica::nil();
        let mut NP: metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>> = metamodelica::nil();
        let mut LV: metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>> = metamodelica::nil();
        let mut LP: metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>> = metamodelica::nil();
        let mut LC: metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>> = metamodelica::nil();
        let mut QQ: metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>> = metamodelica::nil();
        for mut cref in &**crefs {
            let _ = (::match_deref::match_deref! { match &(UnorderedMap::getSafe(cref.clone(), map.clone(), metamodelica::sourceInfo!("NBackEnd/Util/NBAdjacency.mo"))?) {
                Deref @ UNSOLVABLE { .. } => {
                    XX = metamodelica::cons(cref.clone(), XX);
                    ()
                },
                Deref @ IMPLICIT { .. } => {
                    II = metamodelica::cons(cref.clone(), II);
                    ()
                },
                Deref @ EXPLICIT_NONLINEAR { unique: false } => {
                    NM = metamodelica::cons(cref.clone(), NM);
                    ()
                },
                Deref @ EXPLICIT_NONLINEAR { .. } => {
                    NP = metamodelica::cons(cref.clone(), NP);
                    ()
                },
                Deref @ EXPLICIT_LINEAR { vars: Some(_), .. } => {
                    LV = metamodelica::cons(cref.clone(), LV);
                    ()
                },
                Deref @ EXPLICIT_LINEAR { pars: Some(_), .. } => {
                    LP = metamodelica::cons(cref.clone(), LP);
                    ()
                },
                Deref @ EXPLICIT_LINEAR { .. } => {
                    LC = metamodelica::cons(cref.clone(), LC);
                    ()
                },
                _ => {
                    QQ = metamodelica::cons(cref.clone(), QQ);
                    ()
                },
                _ => unreachable!("match_deref! exhaustiveness placeholder"),
            } });
        }
        Ok((XX, II, NM, NP, LV, LP, LC, QQ))
    }

    pub(crate) fn filter(
        mut all_occ: &metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>>,
        mut map: metamodelica::Ref<
            UnorderedMap::UnorderedMap<metamodelica::Ref<ComponentRef::NFComponentRef>, metamodelica::Ref<Solvability>>,
        >,
        mut rel: metamodelica::Ref<UnorderedMap::UnorderedMap<metamodelica::Ref<ComponentRef::NFComponentRef>, i32>>,
        mut min: i32,
        mut max: i32,
    ) -> Result<metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>>> {
        let mut occ: metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>> = metamodelica::nil();
        let mut r: i32;
        for mut cref in &**all_occ {
            if UnorderedMap::contains(cref.clone(), rel.clone())? {
                r = rank(
                    &(UnorderedMap::getSafe(
                        cref.clone(),
                        map.clone(),
                        metamodelica::sourceInfo!("NBackEnd/Util/NBAdjacency.mo"),
                    )?),
                )?;
                if r >= min && r <= max {
                    occ = metamodelica::cons(cref.clone(), occ);
                }
            }
        }
        Ok(occ)
    }

    pub(crate) fn fromStrictness(mut st: MatrixStrictness) -> metamodelica::Ref<Solvability> {
        let mut sol: metamodelica::Ref<Solvability>;
        sol = (match st {
            MatrixStrictness::LINEAR { .. } => metamodelica::Ref::new(Solvability::EXPLICIT_LINEAR {
                pars: None,
                vars: Some(UnorderedSet::new(
                    (std::sync::Arc::new(move |__a0: metamodelica::Ref<ComponentRef::NFComponentRef>| {
                        ComponentRef::hash(&__a0)
                    })
                        as std::sync::Arc<
                            dyn ::std::ops::Fn(metamodelica::Ref<ComponentRef::NFComponentRef>) -> Result<i32>
                                + 'static,
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
                )),
            }),
            MatrixStrictness::MATCHING { .. } => crate::NBAdjacency::Solvability::interned_IMPLICIT(),
            MatrixStrictness::SORTING => crate::NBAdjacency::Solvability::interned_UNSOLVABLE(),
            _ => crate::NBAdjacency::Solvability::interned_UNKNOWN(),
        });
        sol
    }

    pub(crate) fn isNonlinearOrImplicit(mut sol: &metamodelica::Ref<Solvability>) -> bool {
        let mut b: bool;
        b = (match &**sol {
            EXPLICIT_NONLINEAR { .. } => true,
            IMPLICIT { .. } => true,
            _ => false,
        });
        b
    }
}

pub(crate) fn collectDependenciesEquation(
    mut eqn: &metamodelica::Ref<Equation::Equation>,
    mut kind: Partition::Kind,
    mut map: metamodelica::Ref<UnorderedMap::UnorderedMap<metamodelica::Ref<ComponentRef::NFComponentRef>, i32>>,
    mut dep_map: metamodelica::Ref<
        UnorderedMap::UnorderedMap<
            metamodelica::Ref<ComponentRef::NFComponentRef>,
            metamodelica::Ref<Dependency::Dependency>,
        >,
    >,
    mut sol_map: metamodelica::Ref<
        UnorderedMap::UnorderedMap<
            metamodelica::Ref<ComponentRef::NFComponentRef>,
            metamodelica::Ref<Solvability::Solvability>,
        >,
    >,
    mut rep_set: metamodelica::Ref<UnorderedSet::UnorderedSet<metamodelica::Ref<ComponentRef::NFComponentRef>>>,
) -> Result<metamodelica::Ref<UnorderedSet::UnorderedSet<metamodelica::Ref<ComponentRef::NFComponentRef>>>> {
    let mut occurrences: metamodelica::Ref<
        UnorderedSet::UnorderedSet<metamodelica::Ref<ComponentRef::NFComponentRef>>,
    >;
    let mut inputs: metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>>;
    let mut outputs: metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>>;
    occurrences = (::match_deref::match_deref! { match eqn {
        Deref @ BEquation::Equation::SCALAR_EQUATION { lhs: __eqn_lhs, rhs: __eqn_rhs, .. } => {
            let mut occ1: metamodelica::Ref<UnorderedSet::UnorderedSet<metamodelica::Ref<ComponentRef::NFComponentRef>>>;
            let mut occ2: metamodelica::Ref<UnorderedSet::UnorderedSet<metamodelica::Ref<ComponentRef::NFComponentRef>>>;
            occ1 = collectDependencies(__eqn_lhs.clone(), 0, map.clone(), dep_map.clone(), sol_map.clone(), rep_set.clone())?;
            occ2 = collectDependencies(__eqn_rhs.clone(), 0, map, dep_map, sol_map, rep_set)?;
            UnorderedSet::union(occ1, occ2)?
        },
        Deref @ BEquation::Equation::ARRAY_EQUATION { lhs: __eqn_lhs, rhs: __eqn_rhs, .. } => {
            let mut occ1: metamodelica::Ref<UnorderedSet::UnorderedSet<metamodelica::Ref<ComponentRef::NFComponentRef>>>;
            let mut occ2: metamodelica::Ref<UnorderedSet::UnorderedSet<metamodelica::Ref<ComponentRef::NFComponentRef>>>;
            occ1 = collectDependencies(__eqn_lhs.clone(), 0, map.clone(), dep_map.clone(), sol_map.clone(), rep_set.clone())?;
            occ2 = collectDependencies(__eqn_rhs.clone(), 0, map, dep_map, sol_map, rep_set)?;
            UnorderedSet::union(occ1, occ2)?
        },
        Deref @ BEquation::Equation::RECORD_EQUATION { lhs: __eqn_lhs, rhs: __eqn_rhs, .. } => {
            let mut occ1: metamodelica::Ref<UnorderedSet::UnorderedSet<metamodelica::Ref<ComponentRef::NFComponentRef>>>;
            let mut occ2: metamodelica::Ref<UnorderedSet::UnorderedSet<metamodelica::Ref<ComponentRef::NFComponentRef>>>;
            occ1 = collectDependencies(__eqn_lhs.clone(), 0, map.clone(), dep_map.clone(), sol_map.clone(), rep_set.clone())?;
            occ2 = collectDependencies(__eqn_rhs.clone(), 0, map, dep_map, sol_map, rep_set)?;
            UnorderedSet::union(occ1, occ2)?
        },
        Deref @ BEquation::Equation::ALGORITHM { alg: __eqn_alg, .. } => {
            inputs = collectDependenciesAlgorithmInputs(&__eqn_alg.statements, __eqn_alg.inputs.clone())?;
            inputs = List::flatten(({
        let mut __acc: metamodelica::List<metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>>> = metamodelica::nil();
        for mut c in (inputs).into_iter().cloned() {
            let __x = collectDependenciesCref(c.clone(), 0, map.clone(), dep_map.clone(), sol_map.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }))?;
            outputs = List::flatten(({
        let mut __acc: metamodelica::List<metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>>> = metamodelica::nil();
        for mut c in (__eqn_alg.outputs.clone()).into_iter().cloned() {
            let __x = collectDependenciesCref(c.clone(), 0, map.clone(), dep_map.clone(), sol_map.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }))?;
            Dependency::addListFull(&inputs, 0, dep_map.clone(), rep_set.clone())?;
            Dependency::addListFull(&outputs, 0, dep_map, rep_set)?;
            Solvability::updateList(&inputs, crate::NBAdjacency::Solvability::interned_IMPLICIT(), sol_map.clone())?;
            Solvability::updateList(&outputs, metamodelica::Ref::new(Solvability::Solvability::EXPLICIT_LINEAR { pars: None, vars: None }), sol_map)?;
            UnorderedSet::fromList(&(listAppend(inputs, outputs)), (std::sync::Arc::new(move |__a0: metamodelica::Ref<ComponentRef::NFComponentRef>| ComponentRef::hash(&__a0)) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<ComponentRef::NFComponentRef>) -> Result<i32> + 'static>), (std::sync::Arc::new(move |__a0: metamodelica::Ref<ComponentRef::NFComponentRef>, __a1: metamodelica::Ref<ComponentRef::NFComponentRef>| ComponentRef::isEqual(&__a0, &__a1)) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<ComponentRef::NFComponentRef>, metamodelica::Ref<ComponentRef::NFComponentRef>) -> Result<bool> + 'static>))?
        },
        Deref @ BEquation::Equation::FOR_EQUATION { body: Deref @ metamodelica::ListNode::Cons { head: body, tail: Deref @ metamodelica::ListNode::Nil }, iter: __eqn_iter, .. } => {
            let mut occ1: metamodelica::Ref<UnorderedSet::UnorderedSet<metamodelica::Ref<ComponentRef::NFComponentRef>>>;
            let mut occ2: metamodelica::Ref<UnorderedSet::UnorderedSet<metamodelica::Ref<ComponentRef::NFComponentRef>>>;
            let mut filter: Slice::filterCref;
            occ1 = collectDependenciesEquation(metamodelica::AsArg::as_arg(&body), kind, map.clone(), dep_map, sol_map.clone(), rep_set)?;
            occ2 = UnorderedSet::new((std::sync::Arc::new(move |__a0: metamodelica::Ref<ComponentRef::NFComponentRef>| ComponentRef::hash(&__a0)) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<ComponentRef::NFComponentRef>) -> Result<i32> + 'static>), (std::sync::Arc::new(move |__a0: metamodelica::Ref<ComponentRef::NFComponentRef>, __a1: metamodelica::Ref<ComponentRef::NFComponentRef>| ComponentRef::isEqual(&__a0, &__a1)) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<ComponentRef::NFComponentRef>, metamodelica::Ref<ComponentRef::NFComponentRef>) -> Result<bool> + 'static>), 13);
            filter = (std::sync::Arc::new({ let __pe_b2 = map; let __pe_b3 = true; move |__pe_a0, __pe_a1| Slice::getDependentCref(__pe_a0, __pe_a1, __pe_b2.clone(), __pe_b3.clone()) }) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<ComponentRef::NFComponentRef>, metamodelica::Ref<UnorderedSet::UnorderedSet<metamodelica::Ref<ComponentRef::NFComponentRef>>>) -> Result<metamodelica::Ref<ComponentRef::NFComponentRef>> + 'static>);
            let _ = BEquation::Iterator::map(__eqn_iter.clone(), (std::sync::Arc::new({ let __pe_b1 = filter.clone(); let __pe_b2 = occ2.clone(); move |__pe_a0| Slice::filterExp(__pe_a0, __pe_b1.clone(), __pe_b2.clone()) }) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<Expression::NFExpression>) -> Result<metamodelica::Ref<Expression::NFExpression>> + 'static>), Some((std::sync::Arc::new({ let __pe_b1 = occ2.clone(); move |__pe_a0| filter(__pe_a0, __pe_b1.clone()) }) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<ComponentRef::NFComponentRef>) -> Result<metamodelica::Ref<ComponentRef::NFComponentRef>> + 'static>)), &Expression::mapShallow)?;
            Solvability::updateList(&(UnorderedSet::toList(occ2.clone())), crate::NBAdjacency::Solvability::interned_UNSOLVABLE(), sol_map)?;
            UnorderedSet::union(occ1, occ2)?
        },
        Deref @ BEquation::Equation::IF_EQUATION { body: __eqn_body, .. } => {
            collectDependenciesIf(metamodelica::AsArg::as_arg(&__eqn_body), kind, map, dep_map, sol_map, rep_set)?
        },
        Deref @ BEquation::Equation::WHEN_EQUATION { body: __eqn_body, .. } => {
            collectDependenciesWhen(metamodelica::AsArg::as_arg(&__eqn_body), kind, map, dep_map, sol_map, rep_set)?
        },
        _ => {
            UnorderedSet::new((std::sync::Arc::new(move |__a0: metamodelica::Ref<ComponentRef::NFComponentRef>| ComponentRef::hash(&__a0)) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<ComponentRef::NFComponentRef>) -> Result<i32> + 'static>), (std::sync::Arc::new(move |__a0: metamodelica::Ref<ComponentRef::NFComponentRef>, __a1: metamodelica::Ref<ComponentRef::NFComponentRef>| ComponentRef::isEqual(&__a0, &__a1)) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<ComponentRef::NFComponentRef>, metamodelica::Ref<ComponentRef::NFComponentRef>) -> Result<bool> + 'static>), 13)
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(occurrences)
}

pub(crate) fn collectDependencies(
    mut exp: metamodelica::Ref<Expression::NFExpression>,
    mut depth: i32,
    mut map: metamodelica::Ref<UnorderedMap::UnorderedMap<metamodelica::Ref<ComponentRef::NFComponentRef>, i32>>,
    mut dep_map: metamodelica::Ref<
        UnorderedMap::UnorderedMap<
            metamodelica::Ref<ComponentRef::NFComponentRef>,
            metamodelica::Ref<Dependency::Dependency>,
        >,
    >,
    mut sol_map: metamodelica::Ref<
        UnorderedMap::UnorderedMap<
            metamodelica::Ref<ComponentRef::NFComponentRef>,
            metamodelica::Ref<Solvability::Solvability>,
        >,
    >,
    mut rep_set: metamodelica::Ref<UnorderedSet::UnorderedSet<metamodelica::Ref<ComponentRef::NFComponentRef>>>,
) -> Result<metamodelica::Ref<UnorderedSet::UnorderedSet<metamodelica::Ref<ComponentRef::NFComponentRef>>>> {
    let mut set: metamodelica::Ref<UnorderedSet::UnorderedSet<metamodelica::Ref<ComponentRef::NFComponentRef>>>;
    set = ({
        let mut sets: metamodelica::List<
            metamodelica::Ref<UnorderedSet::UnorderedSet<metamodelica::Ref<ComponentRef::NFComponentRef>>>,
        > = metamodelica::nil();
        (::match_deref::match_deref! { match &(&*exp) {
            Deref @ Expression::CREF { cref: __exp_cref, .. } => {
                UnorderedSet::fromList(&(collectDependenciesCref(__exp_cref.clone(), depth, map, dep_map, sol_map)?), (std::sync::Arc::new(move |__a0: metamodelica::Ref<ComponentRef::NFComponentRef>| ComponentRef::hash(&__a0)) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<ComponentRef::NFComponentRef>) -> Result<i32> + 'static>), (std::sync::Arc::new(move |__a0: metamodelica::Ref<ComponentRef::NFComponentRef>, __a1: metamodelica::Ref<ComponentRef::NFComponentRef>| ComponentRef::isEqual(&__a0, &__a1)) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<ComponentRef::NFComponentRef>, metamodelica::Ref<ComponentRef::NFComponentRef>) -> Result<bool> + 'static>))?
            },
            Deref @ Expression::ARRAY { literal: false, .. } => {
                let mut set1: metamodelica::Ref<UnorderedSet::UnorderedSet<metamodelica::Ref<ComponentRef::NFComponentRef>>>;
                for mut i in 1..=metamodelica::arrayLength(var_field!((*exp).elements, Expression::NFExpression::ARRAY).clone()) {
                    set1 = collectDependencies(({let __elt = (*metamodelica::index_checked(&var_field!((*exp).elements, Expression::NFExpression::ARRAY).borrow(), i)?).clone(); __elt}), depth + 1, map.clone(), dep_map.clone(), sol_map.clone(), rep_set.clone())?;
                    Dependency::skipList(&(UnorderedSet::toList(set1.clone())), depth + 1, i, dep_map.clone())?;
                    sets = metamodelica::cons(set1, sets);
                }
                UnorderedSet::union_list(&sets, (std::sync::Arc::new(move |__a0: metamodelica::Ref<ComponentRef::NFComponentRef>| ComponentRef::hash(&__a0)) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<ComponentRef::NFComponentRef>) -> Result<i32> + 'static>), (std::sync::Arc::new(move |__a0: metamodelica::Ref<ComponentRef::NFComponentRef>, __a1: metamodelica::Ref<ComponentRef::NFComponentRef>| ComponentRef::isEqual(&__a0, &__a1)) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<ComponentRef::NFComponentRef>, metamodelica::Ref<ComponentRef::NFComponentRef>) -> Result<bool> + 'static>))?
            },
            Deref @ Expression::TUPLE { elements: __exp_elements, .. } => {
                let mut set1: metamodelica::Ref<UnorderedSet::UnorderedSet<metamodelica::Ref<ComponentRef::NFComponentRef>>>;
                let mut ind: i32;
                ind = 1;
                for mut elem in &*__exp_elements.clone() {
                    set1 = collectDependencies(elem.clone(), depth + 1, map.clone(), dep_map.clone(), sol_map.clone(), rep_set.clone())?;
                    Dependency::skipList(&(UnorderedSet::toList(set1.clone())), depth + 1, ind, dep_map.clone())?;
                    sets = metamodelica::cons(set1, sets);
                    ind = ind + 1;
                }
                set = UnorderedSet::union_list(&sets, (std::sync::Arc::new(move |__a0: metamodelica::Ref<ComponentRef::NFComponentRef>| ComponentRef::hash(&__a0)) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<ComponentRef::NFComponentRef>) -> Result<i32> + 'static>), (std::sync::Arc::new(move |__a0: metamodelica::Ref<ComponentRef::NFComponentRef>, __a1: metamodelica::Ref<ComponentRef::NFComponentRef>| ComponentRef::isEqual(&__a0, &__a1)) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<ComponentRef::NFComponentRef>, metamodelica::Ref<ComponentRef::NFComponentRef>) -> Result<bool> + 'static>))?;
                set
            },
            Deref @ Expression::SUBSCRIPTED_EXP { exp: __exp_exp, subscripts: __exp_subscripts, .. } => {
                let mut set2: metamodelica::Ref<UnorderedSet::UnorderedSet<metamodelica::Ref<ComponentRef::NFComponentRef>>>;
                set = collectDependencies(__exp_exp.clone(), depth, map.clone(), dep_map.clone(), sol_map.clone(), rep_set.clone())?;
                Dependency::updateList(&(UnorderedSet::toList(set.clone())), ((__exp_subscripts).len() as i32), true, dep_map.clone())?;
                Dependency::removeSkipsList(&(UnorderedSet::toList(set.clone())), dep_map.clone())?;
                for mut sub in &*__exp_subscripts.clone() {
                    set2 = (match &*sub.clone() {
            Subscript::INDEX { index: __sub_index } => collectDependencies(__sub_index.clone(), 0, map.clone(), dep_map.clone(), sol_map.clone(), rep_set.clone())?,
            Subscript::SLICE { slice: __sub_slice } => collectDependencies(__sub_slice.clone(), 0, map.clone(), dep_map.clone(), sol_map.clone(), rep_set.clone())?,
            _ => UnorderedSet::new((std::sync::Arc::new(move |__a0: metamodelica::Ref<ComponentRef::NFComponentRef>| ComponentRef::hash(&__a0)) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<ComponentRef::NFComponentRef>) -> Result<i32> + 'static>), (std::sync::Arc::new(move |__a0: metamodelica::Ref<ComponentRef::NFComponentRef>, __a1: metamodelica::Ref<ComponentRef::NFComponentRef>| ComponentRef::isEqual(&__a0, &__a1)) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<ComponentRef::NFComponentRef>, metamodelica::Ref<ComponentRef::NFComponentRef>) -> Result<bool> + 'static>), 13),
        });
                    Solvability::updateList(&(UnorderedSet::toList(set2.clone())), crate::NBAdjacency::Solvability::interned_UNSOLVABLE(), sol_map.clone())?;
                    set = UnorderedSet::union(set, set2)?;
                }
                set
            },
            Deref @ Expression::TUPLE_ELEMENT { tupleExp: __exp_tupleExp, .. } => {
                collectDependencies(__exp_tupleExp.clone(), depth, map, dep_map, sol_map, rep_set)?
            },
            Deref @ Expression::RECORD_ELEMENT { recordExp: __exp_recordExp, .. } => {
                collectDependencies(__exp_recordExp.clone(), depth, map, dep_map, sol_map, rep_set)?
            },
            Deref @ Expression::BINARY { exp1: __exp_exp1, exp2: __exp_exp2, operator: __exp_operator } => {
                let mut set1: metamodelica::Ref<UnorderedSet::UnorderedSet<metamodelica::Ref<ComponentRef::NFComponentRef>>>;
                let mut set2: metamodelica::Ref<UnorderedSet::UnorderedSet<metamodelica::Ref<ComponentRef::NFComponentRef>>>;
                let mut repeatLeft: bool;
                let mut repeatRight: bool;
                let mut reduce: bool;
                set1 = collectDependencies(__exp_exp1.clone(), depth, map.clone(), dep_map.clone(), sol_map.clone(), rep_set.clone())?;
                set2 = collectDependencies(__exp_exp2.clone(), depth, map, dep_map.clone(), sol_map, rep_set.clone())?;
                (repeatLeft, repeatRight) = Operator::repetition(metamodelica::AsArg::as_arg(&__exp_operator));
                if repeatLeft {
                    addRepetitions(set1.clone(), rep_set.clone())?;
                }
                if repeatRight {
                    addRepetitions(set2.clone(), rep_set)?;
                }
                reduce = Operator::reduction(metamodelica::AsArg::as_arg(&__exp_operator));
                if reduce {
                    Dependency::updateList(&(UnorderedSet::toList(set1.clone())), 1, true, dep_map.clone())?;
                    Dependency::updateList(&(UnorderedSet::toList(set2.clone())), 1, false, dep_map)?;
                }
                UnorderedSet::union(set1, set2)?
            },
            Deref @ Expression::MULTARY { arguments: __exp_arguments, inv_arguments: __exp_inv_arguments, operator: __exp_operator } => {
                let mut set1: metamodelica::Ref<UnorderedSet::UnorderedSet<metamodelica::Ref<ComponentRef::NFComponentRef>>>;
                let mut set2: metamodelica::Ref<UnorderedSet::UnorderedSet<metamodelica::Ref<ComponentRef::NFComponentRef>>>;
                let mut repeatLeft: bool;
                let mut repeatRight: bool;
                (repeatLeft, repeatRight) = Operator::repetition(metamodelica::AsArg::as_arg(&__exp_operator));
                repeatLeft = repeatLeft || repeatRight;
                for mut arg in &*__exp_arguments.clone() {
                    set1 = collectDependencies(arg.clone(), depth, map.clone(), dep_map.clone(), sol_map.clone(), rep_set.clone())?;
                    addRepetitionsCond(set1.clone(), arg.clone(), repeatLeft, rep_set.clone())?;
                    sets = metamodelica::cons(set1, sets);
                }
                for mut arg in &*__exp_inv_arguments.clone() {
                    set2 = collectDependencies(arg.clone(), depth, map.clone(), dep_map.clone(), sol_map.clone(), rep_set.clone())?;
                    addRepetitionsCond(set2.clone(), arg.clone(), repeatLeft, rep_set.clone())?;
                    sets = metamodelica::cons(set2, sets);
                }
                set = UnorderedSet::union_list(&sets, (std::sync::Arc::new(move |__a0: metamodelica::Ref<ComponentRef::NFComponentRef>| ComponentRef::hash(&__a0)) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<ComponentRef::NFComponentRef>) -> Result<i32> + 'static>), (std::sync::Arc::new(move |__a0: metamodelica::Ref<ComponentRef::NFComponentRef>, __a1: metamodelica::Ref<ComponentRef::NFComponentRef>| ComponentRef::isEqual(&__a0, &__a1)) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<ComponentRef::NFComponentRef>, metamodelica::Ref<ComponentRef::NFComponentRef>) -> Result<bool> + 'static>))?;
                set
            },
            Deref @ Expression::LBINARY { exp1: __exp_exp1, exp2: __exp_exp2, .. } => {
                let mut set1: metamodelica::Ref<UnorderedSet::UnorderedSet<metamodelica::Ref<ComponentRef::NFComponentRef>>>;
                let mut set2: metamodelica::Ref<UnorderedSet::UnorderedSet<metamodelica::Ref<ComponentRef::NFComponentRef>>>;
                set1 = collectDependencies(__exp_exp1.clone(), depth, map.clone(), dep_map.clone(), sol_map.clone(), rep_set.clone())?;
                set2 = collectDependencies(__exp_exp2.clone(), depth, map, dep_map, sol_map.clone(), rep_set)?;
                set = UnorderedSet::union(set1, set2)?;
                Solvability::updateList(&(UnorderedSet::toList(set.clone())), crate::NBAdjacency::Solvability::interned_UNSOLVABLE(), sol_map)?;
                set
            },
            Deref @ Expression::RELATION { exp1: __exp_exp1, exp2: __exp_exp2, .. } => {
                let mut set1: metamodelica::Ref<UnorderedSet::UnorderedSet<metamodelica::Ref<ComponentRef::NFComponentRef>>>;
                let mut set2: metamodelica::Ref<UnorderedSet::UnorderedSet<metamodelica::Ref<ComponentRef::NFComponentRef>>>;
                set1 = collectDependencies(__exp_exp1.clone(), depth, map.clone(), dep_map.clone(), sol_map.clone(), rep_set.clone())?;
                set2 = collectDependencies(__exp_exp2.clone(), depth, map, dep_map, sol_map.clone(), rep_set)?;
                set = UnorderedSet::union(set1, set2)?;
                Solvability::updateList(&(UnorderedSet::toList(set.clone())), crate::NBAdjacency::Solvability::interned_UNSOLVABLE(), sol_map)?;
                set
            },
            Deref @ Expression::CAST { exp: __exp_exp, .. } => {
                collectDependencies(__exp_exp.clone(), depth, map, dep_map, sol_map, rep_set)?
            },
            Deref @ Expression::BOX { exp: __exp_exp } => {
                collectDependencies(__exp_exp.clone(), depth, map, dep_map, sol_map, rep_set)?
            },
            Deref @ Expression::UNBOX { exp: __exp_exp, .. } => {
                collectDependencies(__exp_exp.clone(), depth, map, dep_map, sol_map, rep_set)?
            },
            Deref @ Expression::UNARY { exp: __exp_exp, .. } => {
                collectDependencies(__exp_exp.clone(), depth, map, dep_map, sol_map, rep_set)?
            },
            Deref @ Expression::LUNARY { exp: __exp_exp, .. } => {
                collectDependencies(__exp_exp.clone(), depth, map, dep_map, sol_map, rep_set)?
            },
            Deref @ Expression::MUTABLE { exp: __exp_exp } => {
                collectDependencies(Mutable::access(__exp_exp.clone()), depth, map, dep_map, sol_map, rep_set)?
            },
            Deref @ Expression::SIZE { dimIndex: __exp_dimIndex, exp: __exp_exp } => {
                let mut set2: metamodelica::Ref<UnorderedSet::UnorderedSet<metamodelica::Ref<ComponentRef::NFComponentRef>>>;
                set = collectDependencies(__exp_exp.clone(), depth, map.clone(), dep_map.clone(), sol_map.clone(), rep_set.clone())?;
                if (__exp_dimIndex).is_some() {
                    set2 = collectDependencies(Util::getOption(__exp_dimIndex.clone())?, depth, map, dep_map, sol_map.clone(), rep_set)?;
                    set = UnorderedSet::union(set, set2)?;
                }
                Solvability::updateList(&(UnorderedSet::toList(set.clone())), crate::NBAdjacency::Solvability::interned_UNSOLVABLE(), sol_map)?;
                set
            },
            Deref @ Expression::IF { condition: __exp_condition, falseBranch: __exp_falseBranch, trueBranch: __exp_trueBranch, .. } => {
                let mut set1: metamodelica::Ref<UnorderedSet::UnorderedSet<metamodelica::Ref<ComponentRef::NFComponentRef>>>;
                let mut set2: metamodelica::Ref<UnorderedSet::UnorderedSet<metamodelica::Ref<ComponentRef::NFComponentRef>>>;
                let mut diff: metamodelica::Ref<UnorderedSet::UnorderedSet<metamodelica::Ref<ComponentRef::NFComponentRef>>>;
                set1 = collectDependencies(__exp_trueBranch.clone(), depth, map.clone(), dep_map.clone(), sol_map.clone(), rep_set.clone())?;
                set2 = collectDependencies(__exp_falseBranch.clone(), depth, map.clone(), dep_map.clone(), sol_map.clone(), rep_set.clone())?;
                diff = UnorderedSet::sym_difference(set1.clone(), set2.clone())?;
                Solvability::updateList(&(UnorderedSet::toList(diff)), crate::NBAdjacency::Solvability::interned_IMPLICIT(), sol_map.clone())?;
                set = collectDependencies(__exp_condition.clone(), depth, map, dep_map.clone(), sol_map.clone(), rep_set.clone())?;
                addRepetitions(set.clone(), rep_set)?;
                updateConditionCrefs(&(UnorderedSet::toList(set.clone())), dep_map, sol_map)?;
                UnorderedSet::union_list(&(list![set, set1, set2]), (std::sync::Arc::new(move |__a0: metamodelica::Ref<ComponentRef::NFComponentRef>| ComponentRef::hash(&__a0)) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<ComponentRef::NFComponentRef>) -> Result<i32> + 'static>), (std::sync::Arc::new(move |__a0: metamodelica::Ref<ComponentRef::NFComponentRef>, __a1: metamodelica::Ref<ComponentRef::NFComponentRef>| ComponentRef::isEqual(&__a0, &__a1)) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<ComponentRef::NFComponentRef>, metamodelica::Ref<ComponentRef::NFComponentRef>) -> Result<bool> + 'static>))?
            },
            Deref @ Expression::CALL { call: call @ Deref @ Call::TYPED_ARRAY_CONSTRUCTOR { exp: call_exp, .. } } => {
                let mut call_exp = (*call_exp).clone();
                for mut iter in &*var_field!((**call).iters, Call::NFCall::TYPED_ARRAY_CONSTRUCTOR).clone() {
                    call_exp = Expression::replaceIterator(call_exp.clone(), &(Util::tuple21(iter.clone())), &(Util::tuple22(iter.clone())))?;
                }
                set = collectDependencies(call_exp.clone(), depth, map, dep_map, sol_map.clone(), rep_set)?;
                Solvability::updateList(&(UnorderedSet::toList(set.clone())), crate::NBAdjacency::Solvability::interned_IMPLICIT(), sol_map)?;
                set
            },
            Deref @ Expression::CALL { call: call @ Deref @ Call::TYPED_REDUCTION { exp: call_exp, .. } } => {
                let mut call_exp = (*call_exp).clone();
                for mut iter in &*var_field!((**call).iters, Call::NFCall::TYPED_REDUCTION).clone() {
                    call_exp = Expression::replaceIterator(call_exp.clone(), &(Util::tuple21(iter.clone())), &(Util::tuple22(iter.clone())))?;
                }
                set = collectDependencies(call_exp.clone(), depth, map, dep_map.clone(), sol_map, rep_set)?;
                Dependency::updateList(&(UnorderedSet::toList(set.clone())), -1, false, dep_map)?;
                set
            },
            Deref @ Expression::CALL { call: call @ Deref @ Call::TYPED_CALL { .. } } => {
                let mut new_depth: i32;
                let mut isTuple: bool;
                isTuple = Type::isTuple(var_field!((**call).ty, Call::NFCall::TYPED_CALL));
                new_depth = if (isTuple) {depth + 1} else {depth};
                for mut arg in &*var_field!((**call).arguments, Call::NFCall::TYPED_CALL).clone() {
                    sets = metamodelica::cons(collectDependencies(arg.clone(), new_depth, map.clone(), dep_map.clone(), sol_map.clone(), rep_set.clone())?, sets);
                }
                set = UnorderedSet::union_list(&sets, (std::sync::Arc::new(move |__a0: metamodelica::Ref<ComponentRef::NFComponentRef>| ComponentRef::hash(&__a0)) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<ComponentRef::NFComponentRef>) -> Result<i32> + 'static>), (std::sync::Arc::new(move |__a0: metamodelica::Ref<ComponentRef::NFComponentRef>, __a1: metamodelica::Ref<ComponentRef::NFComponentRef>| ComponentRef::isEqual(&__a0, &__a1)) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<ComponentRef::NFComponentRef>, metamodelica::Ref<ComponentRef::NFComponentRef>) -> Result<bool> + 'static>))?;
                Dependency::updateList(&(UnorderedSet::toList(set.clone())), -1, false, dep_map.clone())?;
                for mut cref in &*UnorderedSet::toList(set.clone()) {
                    Solvability::update(cref.clone(), if (BVariable::checkCref(metamodelica::AsArg::as_arg(&cref), &({ let __pe_b1 = true; move |__pe_a0| BVariable::isContinuous(__pe_a0, __pe_b1.clone()) }), metamodelica::sourceInfo!("NBackEnd/Util/NBAdjacency.mo"))?) {crate::NBAdjacency::Solvability::interned_IMPLICIT()} else {crate::NBAdjacency::Solvability::interned_UNSOLVABLE()}, sol_map.clone())?;
                }
                addRepetitions(set.clone(), rep_set)?;
                if isTuple {
                    Dependency::skipList(&(UnorderedSet::toList(set.clone())), depth + 1, 0, dep_map)?;
                }
                set
            },
            Deref @ Expression::RECORD { elements: __exp_elements, .. } => {
                for mut arg in &*__exp_elements.clone() {
                    sets = metamodelica::cons(collectDependencies(arg.clone(), depth, map.clone(), dep_map.clone(), sol_map.clone(), rep_set.clone())?, sets);
                }
                set = UnorderedSet::union_list(&sets, (std::sync::Arc::new(move |__a0: metamodelica::Ref<ComponentRef::NFComponentRef>| ComponentRef::hash(&__a0)) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<ComponentRef::NFComponentRef>) -> Result<i32> + 'static>), (std::sync::Arc::new(move |__a0: metamodelica::Ref<ComponentRef::NFComponentRef>, __a1: metamodelica::Ref<ComponentRef::NFComponentRef>| ComponentRef::isEqual(&__a0, &__a1)) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<ComponentRef::NFComponentRef>, metamodelica::Ref<ComponentRef::NFComponentRef>) -> Result<bool> + 'static>))?;
                Dependency::updateList(&(UnorderedSet::toList(set.clone())), -1, false, dep_map)?;
                Solvability::updateList(&(UnorderedSet::toList(set.clone())), crate::NBAdjacency::Solvability::interned_IMPLICIT(), sol_map)?;
                addRepetitions(set.clone(), rep_set)?;
                set
            },
            Deref @ Expression::RANGE { start: __exp_start, step: __exp_step, stop: __exp_stop, .. } => {
                sets = metamodelica::cons(collectDependencies(__exp_start.clone(), depth, map.clone(), dep_map.clone(), sol_map.clone(), rep_set.clone())?, sets);
                if (__exp_step).is_some() {
                    sets = metamodelica::cons(collectDependencies(Util::getOption(__exp_step.clone())?, depth, map.clone(), dep_map.clone(), sol_map.clone(), rep_set.clone())?, sets);
                }
                sets = metamodelica::cons(collectDependencies(__exp_stop.clone(), depth, map, dep_map, sol_map.clone(), rep_set)?, sets);
                set = UnorderedSet::union_list(&sets, (std::sync::Arc::new(move |__a0: metamodelica::Ref<ComponentRef::NFComponentRef>| ComponentRef::hash(&__a0)) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<ComponentRef::NFComponentRef>) -> Result<i32> + 'static>), (std::sync::Arc::new(move |__a0: metamodelica::Ref<ComponentRef::NFComponentRef>, __a1: metamodelica::Ref<ComponentRef::NFComponentRef>| ComponentRef::isEqual(&__a0, &__a1)) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<ComponentRef::NFComponentRef>, metamodelica::Ref<ComponentRef::NFComponentRef>) -> Result<bool> + 'static>))?;
                Solvability::updateList(&(UnorderedSet::toList(set.clone())), crate::NBAdjacency::Solvability::interned_UNSOLVABLE(), sol_map)?;
                set
            },
            _ => {
                UnorderedSet::new((std::sync::Arc::new(move |__a0: metamodelica::Ref<ComponentRef::NFComponentRef>| ComponentRef::hash(&__a0)) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<ComponentRef::NFComponentRef>) -> Result<i32> + 'static>), (std::sync::Arc::new(move |__a0: metamodelica::Ref<ComponentRef::NFComponentRef>, __a1: metamodelica::Ref<ComponentRef::NFComponentRef>| ComponentRef::isEqual(&__a0, &__a1)) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<ComponentRef::NFComponentRef>, metamodelica::Ref<ComponentRef::NFComponentRef>) -> Result<bool> + 'static>), 13)
            },
            _ => unreachable!("match_deref! exhaustiveness placeholder"),
        } })
    });
    Ok(set)
}

pub(crate) fn collectDependenciesCref(
    mut cref: metamodelica::Ref<ComponentRef::NFComponentRef>,
    mut depth: i32,
    mut map: metamodelica::Ref<UnorderedMap::UnorderedMap<metamodelica::Ref<ComponentRef::NFComponentRef>, i32>>,
    mut dep_map: metamodelica::Ref<
        UnorderedMap::UnorderedMap<
            metamodelica::Ref<ComponentRef::NFComponentRef>,
            metamodelica::Ref<Dependency::Dependency>,
        >,
    >,
    mut sol_map: metamodelica::Ref<
        UnorderedMap::UnorderedMap<
            metamodelica::Ref<ComponentRef::NFComponentRef>,
            metamodelica::Ref<Solvability::Solvability>,
        >,
    >,
) -> Result<metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>>> {
    let mut crefs: metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>>;
    let mut var: Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>;
    let mut sk: i32 = 1;
    let mut subs: metamodelica::List<metamodelica::Ref<Subscript::NFSubscript>>;
    let mut scalar_matches: metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>>;
    let mut hasSetSub: bool = false;
    for mut s in &*ComponentRef::subscriptsAllFlat(&cref)? {
        if !(Subscript::isScalar(metamodelica::AsArg::as_arg(&s))?)
            && !(Subscript::isSliced(metamodelica::AsArg::as_arg(&s)))
        {
            hasSetSub = true;
        }
    }
    if !(hasSetSub) && UnorderedMap::contains(cref.clone(), map.clone())? {
        if !(UnorderedMap::contains(cref.clone(), dep_map.clone())?) {
            UnorderedMap::add(
                cref.clone(),
                Dependency::create(ComponentRef::getSubscriptedType(&cref, false)?, depth)?,
                dep_map,
            )?;
        }
        Solvability::update(
            cref.clone(),
            metamodelica::Ref::new(Solvability::Solvability::EXPLICIT_LINEAR { pars: None, vars: None }),
            sol_map,
        )?;
        crefs = list![cref];
        return Ok(crefs);
    }
    if !(hasSetSub)
        && Type::isArray(&(ComponentRef::getSubscriptedType(&cref, false)?))
        && Type::sizeOf(&(ComponentRef::getSubscriptedType(&cref, false)?), false)? <= 256
    {
        hasSetSub = true;
    }
    if hasSetSub {
        scalar_matches = ({
            let mut __acc: metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>> = metamodelica::nil();
            for mut c in (ComponentRef::scalarize(cref.clone(), false)?).into_iter().cloned() {
                if !(UnorderedMap::contains(c.clone(), map.clone())?) {
                    continue;
                }
                let __x = c.clone();
                __acc = cons(__x, __acc);
            }
            __acc.reverse()
        });
        if !((scalar_matches).is_empty()) {
            for mut c in &*scalar_matches {
                if !(UnorderedMap::contains(c.clone(), dep_map.clone())?) {
                    UnorderedMap::add(
                        c.clone(),
                        Dependency::create(
                            ComponentRef::getSubscriptedType(metamodelica::AsArg::as_arg(&c), false)?,
                            depth,
                        )?,
                        dep_map.clone(),
                    )?;
                }
                Solvability::update(
                    c.clone(),
                    metamodelica::Ref::new(Solvability::Solvability::EXPLICIT_LINEAR { pars: None, vars: None }),
                    sol_map.clone(),
                )?;
            }
            crefs = scalar_matches;
            return Ok(crefs);
        }
    }
    var = BVariable::getVarPointer(&cref, metamodelica::sourceInfo!("NBackEnd/Util/NBAdjacency.mo"))?;
    if BVariable::isRecord(var.clone()) {
        subs = ComponentRef::subscriptsAllFlat(&cref)?;
        crefs = ({
            let mut __acc: metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>> = metamodelica::nil();
            for mut child in (BVariable::getRecordChildren(var)?).into_iter().cloned() {
                let __x = BVariable::getVarName(child.clone());
                __acc = cons(__x, __acc);
            }
            __acc.reverse()
        });
        crefs = ({
            let mut __acc: metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>> = metamodelica::nil();
            for mut child in (crefs).into_iter().cloned() {
                if !(UnorderedMap::contains(child.clone(), map.clone())?) {
                    continue;
                }
                let __x = child.clone();
                __acc = cons(__x, __acc);
            }
            __acc.reverse()
        });
        crefs = ({
            let mut __acc: metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>> = metamodelica::nil();
            for mut child in (crefs).into_iter().cloned() {
                let __x = ComponentRef::mergeSubscripts(subs.clone(), child.clone(), false, false, false)?;
                __acc = cons(__x, __acc);
            }
            __acc.reverse()
        });
        crefs = List::flatten(
            ({
                let mut __acc: metamodelica::List<metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>>> =
                    metamodelica::nil();
                for mut child in (crefs).into_iter().cloned() {
                    let __x = collectDependenciesCref(
                        child.clone(),
                        depth + 1,
                        map.clone(),
                        dep_map.clone(),
                        sol_map.clone(),
                    )?;
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            }),
        )?;
        for mut cref in &*crefs {
            let mut cref = cref.clone();
            Dependency::skip(cref, depth + 1, sk, dep_map.clone())?;
            sk = sk + 1;
        }
    } else {
        crefs = metamodelica::nil();
    }
    Ok(crefs)
}

pub(crate) fn addRepetitionsCond(
    mut occ: metamodelica::Ref<UnorderedSet::UnorderedSet<metamodelica::Ref<ComponentRef::NFComponentRef>>>,
    mut exp: metamodelica::Ref<Expression::NFExpression>,
    mut isRep: bool,
    mut rep_set: metamodelica::Ref<UnorderedSet::UnorderedSet<metamodelica::Ref<ComponentRef::NFComponentRef>>>,
) -> Result<()> {
    if isRep && Type::sizeOf(&(Expression::typeOf(exp)), false)? == 1 {
        addRepetitions(occ, rep_set)?;
    }
    Ok(())
}

pub(crate) fn addRepetitions(
    mut occ: metamodelica::Ref<UnorderedSet::UnorderedSet<metamodelica::Ref<ComponentRef::NFComponentRef>>>,
    mut rep_set: metamodelica::Ref<UnorderedSet::UnorderedSet<metamodelica::Ref<ComponentRef::NFComponentRef>>>,
) -> Result<()> {
    for mut cref in &*UnorderedSet::toList(occ) {
        UnorderedSet::add(cref.clone(), rep_set.clone())?;
    }
    Ok(())
}

pub(crate) fn collectDependenciesIf(
    mut body: &metamodelica::Ref<IfEquationBody::IfEquationBody>,
    mut kind: Partition::Kind,
    mut map: metamodelica::Ref<UnorderedMap::UnorderedMap<metamodelica::Ref<ComponentRef::NFComponentRef>, i32>>,
    mut dep_map: metamodelica::Ref<
        UnorderedMap::UnorderedMap<
            metamodelica::Ref<ComponentRef::NFComponentRef>,
            metamodelica::Ref<Dependency::Dependency>,
        >,
    >,
    mut sol_map: metamodelica::Ref<
        UnorderedMap::UnorderedMap<
            metamodelica::Ref<ComponentRef::NFComponentRef>,
            metamodelica::Ref<Solvability::Solvability>,
        >,
    >,
    mut rep_set: metamodelica::Ref<UnorderedSet::UnorderedSet<metamodelica::Ref<ComponentRef::NFComponentRef>>>,
) -> Result<metamodelica::Ref<UnorderedSet::UnorderedSet<metamodelica::Ref<ComponentRef::NFComponentRef>>>> {
    let mut set: metamodelica::Ref<UnorderedSet::UnorderedSet<metamodelica::Ref<ComponentRef::NFComponentRef>>>;
    let mut sets1: metamodelica::List<
        metamodelica::Ref<UnorderedSet::UnorderedSet<metamodelica::Ref<ComponentRef::NFComponentRef>>>,
    > = metamodelica::nil();
    let mut set1: metamodelica::Ref<UnorderedSet::UnorderedSet<metamodelica::Ref<ComponentRef::NFComponentRef>>>;
    let mut set2: metamodelica::Ref<UnorderedSet::UnorderedSet<metamodelica::Ref<ComponentRef::NFComponentRef>>>;
    let mut diff: metamodelica::Ref<UnorderedSet::UnorderedSet<metamodelica::Ref<ComponentRef::NFComponentRef>>>;
    set = collectDependencies(
        body.condition.clone(),
        0,
        map.clone(),
        dep_map.clone(),
        sol_map.clone(),
        rep_set.clone(),
    )?;
    addRepetitions(set.clone(), rep_set.clone())?;
    updateConditionCrefs(&(UnorderedSet::toList(set.clone())), dep_map.clone(), sol_map.clone())?;
    for mut eqn in &*body.then_eqns.clone() {
        sets1 = metamodelica::cons(
            collectDependenciesEquation(
                &(Pointer::access(eqn.clone())),
                kind,
                map.clone(),
                dep_map.clone(),
                sol_map.clone(),
                rep_set.clone(),
            )?,
            sets1,
        );
    }
    if (body.else_if).is_some() {
        set1 = UnorderedSet::union_list(
            &sets1,
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
        )?;
        set2 = collectDependenciesIf(
            &(Util::getOption(body.else_if.clone())?),
            kind,
            map,
            dep_map,
            sol_map.clone(),
            rep_set,
        )?;
        diff = UnorderedSet::sym_difference(set1.clone(), set2.clone())?;
        Solvability::updateList(
            &(UnorderedSet::toList(diff)),
            crate::NBAdjacency::Solvability::interned_IMPLICIT(),
            sol_map,
        )?;
        set = UnorderedSet::union_list(
            &(list![set, set1, set2]),
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
        )?;
    } else {
        set = UnorderedSet::union_list(
            &(metamodelica::cons(set, sets1)),
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
        )?;
    }
    Ok(set)
}

pub(crate) fn collectDependenciesWhen(
    mut body: &metamodelica::Ref<WhenEquationBody::WhenEquationBody>,
    mut kind: Partition::Kind,
    mut map: metamodelica::Ref<UnorderedMap::UnorderedMap<metamodelica::Ref<ComponentRef::NFComponentRef>, i32>>,
    mut dep_map: metamodelica::Ref<
        UnorderedMap::UnorderedMap<
            metamodelica::Ref<ComponentRef::NFComponentRef>,
            metamodelica::Ref<Dependency::Dependency>,
        >,
    >,
    mut sol_map: metamodelica::Ref<
        UnorderedMap::UnorderedMap<
            metamodelica::Ref<ComponentRef::NFComponentRef>,
            metamodelica::Ref<Solvability::Solvability>,
        >,
    >,
    mut rep_set: metamodelica::Ref<UnorderedSet::UnorderedSet<metamodelica::Ref<ComponentRef::NFComponentRef>>>,
) -> Result<metamodelica::Ref<UnorderedSet::UnorderedSet<metamodelica::Ref<ComponentRef::NFComponentRef>>>> {
    let mut set: metamodelica::Ref<UnorderedSet::UnorderedSet<metamodelica::Ref<ComponentRef::NFComponentRef>>>;
    let mut set1: metamodelica::Ref<UnorderedSet::UnorderedSet<metamodelica::Ref<ComponentRef::NFComponentRef>>>;
    let mut set2: metamodelica::Ref<UnorderedSet::UnorderedSet<metamodelica::Ref<ComponentRef::NFComponentRef>>>;
    let mut diff: metamodelica::Ref<UnorderedSet::UnorderedSet<metamodelica::Ref<ComponentRef::NFComponentRef>>>;
    let mut lst: metamodelica::List<
        metamodelica::Ref<UnorderedSet::UnorderedSet<metamodelica::Ref<ComponentRef::NFComponentRef>>>,
    > = metamodelica::nil();
    let mut lst1: metamodelica::List<
        metamodelica::Ref<UnorderedSet::UnorderedSet<metamodelica::Ref<ComponentRef::NFComponentRef>>>,
    >;
    let mut lst2: metamodelica::List<
        metamodelica::Ref<UnorderedSet::UnorderedSet<metamodelica::Ref<ComponentRef::NFComponentRef>>>,
    >;
    let mut tpl_lst: metamodelica::List<(
        metamodelica::Ref<UnorderedSet::UnorderedSet<metamodelica::Ref<ComponentRef::NFComponentRef>>>,
        metamodelica::Ref<UnorderedSet::UnorderedSet<metamodelica::Ref<ComponentRef::NFComponentRef>>>,
    )> = metamodelica::nil();
    set = collectDependencies(
        body.condition.clone(),
        0,
        map.clone(),
        dep_map.clone(),
        sol_map.clone(),
        rep_set.clone(),
    )?;
    updateConditionCrefs(&(UnorderedSet::toList(set.clone())), dep_map.clone(), sol_map.clone())?;
    if ({
        let mut __acc: i32 = 0;
        for mut stmt in (body.when_stmts.clone()).into_iter().cloned() {
            let __x = BEquation::WhenStatement::size(&(stmt.clone()), true)?;
            __acc += __x;
        }
        __acc
    }) > 1
    {
        addRepetitions(set.clone(), rep_set.clone())?;
    }
    for mut stmt in &*body.when_stmts.clone() {
        tpl_lst = metamodelica::cons(
            collectDependenciesStmt(
                metamodelica::AsArg::as_arg(&stmt),
                map.clone(),
                dep_map.clone(),
                sol_map.clone(),
                rep_set.clone(),
            )?,
            tpl_lst,
        );
    }
    (lst1, lst2) = List::unzip(&tpl_lst);
    set1 = UnorderedSet::union_list(
        &lst1,
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
    )?;
    set2 = UnorderedSet::union_list(
        &lst2,
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
    )?;
    diff = UnorderedSet::difference(set2.clone(), set1.clone())?;
    Solvability::updateList(
        &(UnorderedSet::toList(diff)),
        crate::NBAdjacency::Solvability::interned_UNSOLVABLE(),
        sol_map.clone(),
    )?;
    if (body.else_when).is_some() {
        lst = metamodelica::cons(
            collectDependenciesWhen(
                &(Util::getOption(body.else_when.clone())?),
                kind,
                map,
                dep_map,
                sol_map,
                rep_set,
            )?,
            lst,
        );
    }
    set = UnorderedSet::union_list(
        &(metamodelica::cons(set, metamodelica::cons(set1, metamodelica::cons(set2, lst)))),
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
    )?;
    Ok(set)
}

pub(crate) fn collectDependenciesStmt(
    mut stmt: &metamodelica::Ref<WhenStatement::WhenStatement>,
    mut map: metamodelica::Ref<UnorderedMap::UnorderedMap<metamodelica::Ref<ComponentRef::NFComponentRef>, i32>>,
    mut dep_map: metamodelica::Ref<
        UnorderedMap::UnorderedMap<
            metamodelica::Ref<ComponentRef::NFComponentRef>,
            metamodelica::Ref<Dependency::Dependency>,
        >,
    >,
    mut sol_map: metamodelica::Ref<
        UnorderedMap::UnorderedMap<
            metamodelica::Ref<ComponentRef::NFComponentRef>,
            metamodelica::Ref<Solvability::Solvability>,
        >,
    >,
    mut rep_set: metamodelica::Ref<UnorderedSet::UnorderedSet<metamodelica::Ref<ComponentRef::NFComponentRef>>>,
) -> Result<(
    metamodelica::Ref<UnorderedSet::UnorderedSet<metamodelica::Ref<ComponentRef::NFComponentRef>>>,
    metamodelica::Ref<UnorderedSet::UnorderedSet<metamodelica::Ref<ComponentRef::NFComponentRef>>>,
)> {
    let mut set_tpl: (
        metamodelica::Ref<UnorderedSet::UnorderedSet<metamodelica::Ref<ComponentRef::NFComponentRef>>>,
        metamodelica::Ref<UnorderedSet::UnorderedSet<metamodelica::Ref<ComponentRef::NFComponentRef>>>,
    );
    let mut set1: metamodelica::Ref<UnorderedSet::UnorderedSet<metamodelica::Ref<ComponentRef::NFComponentRef>>>;
    let mut set2: metamodelica::Ref<UnorderedSet::UnorderedSet<metamodelica::Ref<ComponentRef::NFComponentRef>>>;
    set_tpl = (match &**stmt {
        BEquation::WhenStatement::ASSIGN {
            lhs: __stmt_lhs,
            rhs: __stmt_rhs,
            ..
        } => {
            set1 = collectDependencies(
                __stmt_lhs.clone(),
                0,
                map.clone(),
                dep_map.clone(),
                sol_map.clone(),
                rep_set.clone(),
            )?;
            set2 = collectDependencies(__stmt_rhs.clone(), 0, map, dep_map, sol_map, rep_set)?;
            (set1, set2)
        }
        BEquation::WhenStatement::REINIT {
            value: __stmt_value, ..
        } => {
            set1 = UnorderedSet::new(
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
            set2 = collectDependencies(__stmt_value.clone(), 0, map, dep_map, sol_map, rep_set)?;
            (set1, set2)
        }
        BEquation::WhenStatement::ASSERT {
            condition: __stmt_condition,
            ..
        } => {
            set1 = UnorderedSet::new(
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
            set2 = collectDependencies(
                __stmt_condition.clone(),
                0,
                map,
                dep_map.clone(),
                sol_map.clone(),
                rep_set,
            )?;
            updateConditionCrefs(&(UnorderedSet::toList(set2.clone())), dep_map, sol_map)?;
            (set1, set2)
        }
        _ => {
            set1 = UnorderedSet::new(
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
            set2 = UnorderedSet::new(
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
            (set1, set2)
        }
    });
    Ok(set_tpl)
}

pub(crate) fn collectDependenciesAlgorithmInputs(
    mut stmts: &metamodelica::List<metamodelica::Ref<Statement::NFStatement>>,
    mut inputs: metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>>,
) -> Result<metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>>> {
    let mut inputs: metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>> = inputs;
    let mut candidates: metamodelica::Ref<UnorderedSet::UnorderedSet<metamodelica::Ref<ComponentRef::NFComponentRef>>> =
        UnorderedSet::fromList(
            &inputs,
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
        )?;
    let mut result: metamodelica::Ref<UnorderedSet::UnorderedSet<metamodelica::Ref<ComponentRef::NFComponentRef>>> =
        UnorderedSet::fromList(
            &inputs,
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
        )?;
    for mut stmt in &**stmts {
        collectDependenciesAlgorithmStatement(metamodelica::AsArg::as_arg(&stmt), candidates.clone(), result.clone())?;
    }
    inputs = UnorderedSet::toList(result);
    Ok(inputs)
}

pub(crate) fn collectDependenciesAlgorithmStatement(
    mut stmt: &metamodelica::Ref<Statement::NFStatement>,
    mut candidates: metamodelica::Ref<UnorderedSet::UnorderedSet<metamodelica::Ref<ComponentRef::NFComponentRef>>>,
    mut result: metamodelica::Ref<UnorderedSet::UnorderedSet<metamodelica::Ref<ComponentRef::NFComponentRef>>>,
) -> Result<()> {
    let _ = (match &**stmt {
        Statement::ASSIGNMENT {
            lhs: __stmt_lhs,
            rhs: __stmt_rhs,
            ..
        } => {
            Slice::filterExp(
                __stmt_lhs.clone(),
                (std::sync::Arc::new({
                    let __pe_b2 = candidates.clone();
                    move |__pe_a0, __pe_a1| BEquation::Equation::collectFromSet(__pe_a0, __pe_a1, __pe_b2.clone())
                })
                    as std::sync::Arc<
                        dyn ::std::ops::Fn(
                                metamodelica::Ref<ComponentRef::NFComponentRef>,
                                metamodelica::Ref<
                                    UnorderedSet::UnorderedSet<metamodelica::Ref<ComponentRef::NFComponentRef>>,
                                >,
                            )
                                -> Result<metamodelica::Ref<ComponentRef::NFComponentRef>>
                            + 'static,
                    >),
                result.clone(),
            )?;
            Slice::filterExp(
                __stmt_rhs.clone(),
                (std::sync::Arc::new({
                    let __pe_b2 = candidates;
                    move |__pe_a0, __pe_a1| BEquation::Equation::collectFromSet(__pe_a0, __pe_a1, __pe_b2.clone())
                })
                    as std::sync::Arc<
                        dyn ::std::ops::Fn(
                                metamodelica::Ref<ComponentRef::NFComponentRef>,
                                metamodelica::Ref<
                                    UnorderedSet::UnorderedSet<metamodelica::Ref<ComponentRef::NFComponentRef>>,
                                >,
                            )
                                -> Result<metamodelica::Ref<ComponentRef::NFComponentRef>>
                            + 'static,
                    >),
                result,
            )?;
            ()
        }
        Statement::FOR { body: __stmt_body, .. } => {
            for mut s in &*__stmt_body.clone() {
                collectDependenciesAlgorithmStatement(
                    metamodelica::AsArg::as_arg(&s),
                    candidates.clone(),
                    result.clone(),
                )?;
            }
            ()
        }
        Statement::WHILE { body: __stmt_body, .. } => {
            for mut s in &*__stmt_body.clone() {
                collectDependenciesAlgorithmStatement(
                    metamodelica::AsArg::as_arg(&s),
                    candidates.clone(),
                    result.clone(),
                )?;
            }
            ()
        }
        Statement::IF {
            branches: __stmt_branches,
            ..
        } => {
            for mut branch in &*__stmt_branches.clone() {
                for mut s in &*Util::tuple22(branch.clone()) {
                    collectDependenciesAlgorithmStatement(
                        metamodelica::AsArg::as_arg(&s),
                        candidates.clone(),
                        result.clone(),
                    )?;
                }
            }
            ()
        }
        Statement::WHEN {
            branches: __stmt_branches,
            ..
        } => {
            for mut branch in &*__stmt_branches.clone() {
                for mut s in &*Util::tuple22(branch.clone()) {
                    collectDependenciesAlgorithmStatement(
                        metamodelica::AsArg::as_arg(&s),
                        candidates.clone(),
                        result.clone(),
                    )?;
                }
            }
            ()
        }
        _ => (),
    });
    Ok(())
}

pub(crate) fn updateConditionCrefs(
    mut crefs: &metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>>,
    mut dep_map: metamodelica::Ref<
        UnorderedMap::UnorderedMap<
            metamodelica::Ref<ComponentRef::NFComponentRef>,
            metamodelica::Ref<Dependency::Dependency>,
        >,
    >,
    mut sol_map: metamodelica::Ref<
        UnorderedMap::UnorderedMap<
            metamodelica::Ref<ComponentRef::NFComponentRef>,
            metamodelica::Ref<Solvability::Solvability>,
        >,
    >,
) -> Result<()> {
    Dependency::removeSkipsList(crefs, dep_map.clone())?;
    Dependency::updateList(crefs, -1, false, dep_map)?;
    Solvability::updateList(crefs, crate::NBAdjacency::Solvability::interned_UNSOLVABLE(), sol_map)?;
    Ok(())
}

pub(crate) fn addInitialStartOccurrences(
    mut occs: metamodelica::Ref<UnorderedSet::UnorderedSet<metamodelica::Ref<ComponentRef::NFComponentRef>>>,
    mut dep_map: metamodelica::Ref<
        UnorderedMap::UnorderedMap<
            metamodelica::Ref<ComponentRef::NFComponentRef>,
            metamodelica::Ref<Dependency::Dependency>,
        >,
    >,
    mut sol_map: metamodelica::Ref<
        UnorderedMap::UnorderedMap<
            metamodelica::Ref<ComponentRef::NFComponentRef>,
            metamodelica::Ref<Solvability::Solvability>,
        >,
    >,
    mut rep_set: metamodelica::Ref<UnorderedSet::UnorderedSet<metamodelica::Ref<ComponentRef::NFComponentRef>>>,
    mut kind: Partition::Kind,
) -> Result<()> {
    if Partition::kindIsInitial(kind) {
        for mut cref in &*UnorderedSet::toList(occs.clone()) {
            let () = (match (BVariable::getVarStart(BVariable::getVarPointer(
                metamodelica::AsArg::as_arg(&cref),
                metamodelica::sourceInfo!("NBackEnd/Util/NBAdjacency.mo"),
            )?))
            .0
            {
                Some(mut start) if (BVariable::isStart(start.clone())) => {
                    let mut start_cref: metamodelica::Ref<ComponentRef::NFComponentRef>;
                    start_cref = ComponentRef::copySubscripts(
                        metamodelica::AsArg::as_arg(&cref),
                        BVariable::getVarName(start.clone()),
                    )?;
                    UnorderedSet::add(start_cref.clone(), occs.clone())?;
                    UnorderedMap::add(
                        start_cref.clone(),
                        UnorderedMap::getSafe(
                            cref.clone(),
                            dep_map.clone(),
                            metamodelica::sourceInfo!("NBackEnd/Util/NBAdjacency.mo"),
                        )?,
                        dep_map.clone(),
                    )?;
                    UnorderedMap::add(
                        start_cref.clone(),
                        crate::NBAdjacency::Solvability::interned_UNSOLVABLE(),
                        sol_map.clone(),
                    )?;
                    if UnorderedSet::contains(cref.clone(), rep_set.clone())? {
                        UnorderedSet::add(start_cref, rep_set.clone())?;
                    }
                    ()
                }
                _ => (),
            });
        }
    }
    Ok(())
}
