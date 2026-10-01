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
use crate::NBAdjacency::Dependency;
use crate::NBEquation as BEquation;
use crate::NBEquation::EqData;
use crate::NBEquation::Equation;
use crate::NBEquation::EquationPointer;
use crate::NBEquation::EquationPointers;
use crate::NBEquation::Iterator;
use crate::NBJacobian as Jacobian;
use crate::NBPartition as Partition;
use crate::NBVariable as BVariable;
use crate::NBVariable::VarData;
use crate::NBVariable::VariablePointers;
use crate::NBackendDAE as BackendDAE;
use crate::NSimCode as SimCode;
use crate::NSimCode::Identifier;
use crate::NSimCodeUtil as SimCodeUtil;
use crate::NSimGenericCall as SimGenericCall;
use crate::NSimStrongComponent as SimStrongComponent;
use crate::NSimVar::ConvertMemo;
use crate::NSimVar::SimVar;
use crate::NSimVar::SimVars;
use crate::NSimVar::VarType;
use openmodelica_backend_types::BackendDAE as OldBackendDAE;
use openmodelica_frontend_types::DAE;
use openmodelica_nf_frontend::NFComponentRef as ComponentRef;
use openmodelica_nf_frontend::NFDimension as Dimension;
use openmodelica_nf_frontend::NFExpression as Expression;
use openmodelica_nf_frontend::NFInstNode::InstNode;
use openmodelica_nf_frontend::NFSubscript as Subscript;
use openmodelica_nf_frontend::NFType as Type;
use openmodelica_nf_frontend::NFVariable as Variable;
use openmodelica_simcode_types::SimCode as OldSimCode;
use openmodelica_simcode_types::SimCodeVar;
use openmodelica_util::Error;
use openmodelica_util::Flags;
use openmodelica_util::StringUtil;
use openmodelica_util::UnorderedMap;
use openmodelica_util::UnorderedSet;
use openmodelica_util::Util;
use openmodelica_util_datatypes_basic::List;
use openmodelica_util_datatypes_basic::Pointer;

// NF imports
// Backend imports
// SimCode imports
// Old SimCode imports
// Util imports
pub mod SparsityRow {
    use super::*;
    #[derive(Clone, Debug, Eq, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
    pub struct SparsityRow {
        /// only for debugging
        pub equation_name: metamodelica::Ref<ComponentRef::NFComponentRef>,
        pub equation_iterators: metamodelica::List<metamodelica::Ref<SimGenericCall::SimIterator::SimIterator>>,
        pub dependencies: metamodelica::List<(
            metamodelica::Ref<ComponentRef::NFComponentRef>,
            metamodelica::Ref<Dependency::Dependency>,
            bool,
        )>,
        pub solved_crefs: metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>>,
    }

    impl metamodelica::gc::MMTrace for SparsityRow {
        fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
            metamodelica::gc::MMTrace::mm_accept(&self.equation_name, __mmv)?;
            metamodelica::gc::MMTrace::mm_accept(&self.equation_iterators, __mmv)?;
            metamodelica::gc::MMTrace::mm_accept(&self.dependencies, __mmv)?;
            metamodelica::gc::MMTrace::mm_accept(&self.solved_crefs, __mmv)?;
            Ok(())
        }
    }
    impl Default for SparsityRow {
        fn default() -> Self {
            Self {
                equation_name: Default::default(),
                equation_iterators: Default::default(),
                dependencies: Default::default(),
                solved_crefs: Default::default(),
            }
        }
    }

    pub type SPARSITY_ROW = SparsityRow;

    pub(crate) fn create(
        mut equation_name: metamodelica::Ref<ComponentRef::NFComponentRef>,
        mut equation_iterator: &metamodelica::Ref<Iterator::Iterator>,
        mut dependencies: metamodelica::Ref<
            UnorderedMap::UnorderedMap<
                metamodelica::Ref<ComponentRef::NFComponentRef>,
                metamodelica::Ref<Dependency::Dependency>,
            >,
        >,
        mut repetitions: metamodelica::Ref<UnorderedSet::UnorderedSet<metamodelica::Ref<ComponentRef::NFComponentRef>>>,
        mut solved_crefs: metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>>,
    ) -> Result<metamodelica::Ref<SparsityRow>> {
        let mut row: metamodelica::Ref<SparsityRow>;
        let mut crefs: metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>>;
        let mut deps: metamodelica::List<metamodelica::Ref<Dependency::Dependency>>;
        let mut reps: metamodelica::List<bool>;
        crefs = UnorderedMap::keyList(dependencies.clone());
        deps = UnorderedMap::valueList(dependencies);
        reps = ({
            let mut __acc: metamodelica::List<bool> = metamodelica::nil();
            for mut cref in (crefs.clone()).into_iter().cloned() {
                let __x = UnorderedSet::contains(cref.clone(), repetitions.clone())?;
                __acc = cons(__x, __acc);
            }
            __acc.reverse()
        });
        crefs = ({
            let mut __acc: metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>> = metamodelica::nil();
            for mut cref in (crefs).into_iter().cloned() {
                let __x = ComponentRef::fillSubscripts(cref.clone());
                __acc = cons(__x, __acc);
            }
            __acc.reverse()
        });
        row = metamodelica::Ref::new(SparsityRow {
            equation_name: equation_name,
            equation_iterators: SimGenericCall::SimIterator::fromIterator(equation_iterator)?,
            dependencies: ({
                let mut __acc: metamodelica::List<(
                    metamodelica::Ref<ComponentRef::NFComponentRef>,
                    metamodelica::Ref<Dependency::Dependency>,
                    bool,
                )> = metamodelica::nil();
                let __thr_src0 = crefs;
                let mut __thr_it0 = (&__thr_src0).into_iter();
                let __thr_src1 = deps;
                let mut __thr_it1 = (&__thr_src1).into_iter();
                let __thr_src2 = reps;
                let mut __thr_it2 = (&__thr_src2).into_iter();
                loop {
                    match (__thr_it0.next(), __thr_it1.next(), __thr_it2.next()) {
                        (Some(cref), Some(dep), Some(rep)) => {
                            let __x = (cref.clone(), dep.clone(), rep.clone());
                            __acc = cons(__x, __acc);
                        }
                        (None, None, None) => break,
                        _ => return Err("threaded for: ranges of unequal length"),
                    }
                }
                __acc.reverse()
            }),
            solved_crefs: ({
                let mut __acc: metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>> =
                    metamodelica::nil();
                for mut cref in (solved_crefs).into_iter().cloned() {
                    let __x = ComponentRef::fillSubscripts(cref.clone());
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            }),
        });
        Ok(row)
    }

    pub(crate) fn convert(mut row: &metamodelica::Ref<SparsityRow>) -> Result<OldSimCode::SparsityRow> {
        let mut oldrow: OldSimCode::SparsityRow;
        oldrow = OldSimCode::SparsityRow {
            equation_name: ComponentRef::toDAE(&row.equation_name)?,
            equation_iterators: ({
                let mut __acc: metamodelica::List<OldBackendDAE::SimIterator> = metamodelica::nil();
                for mut iter in (row.equation_iterators.clone()).into_iter().cloned() {
                    let __x = SimGenericCall::SimIterator::convert(&(iter.clone()))?;
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            }),
            dependencies: ({
                let mut __acc: metamodelica::List<(
                    metamodelica::Ref<DAE::ComponentRef>,
                    OldSimCode::Dependency,
                    bool,
                )> = metamodelica::nil();
                for mut tpl in (row.dependencies.clone()).into_iter().cloned() {
                    let __x = (
                        ComponentRef::toDAE(&(Util::tuple31(tpl.clone())))?,
                        Adjacency::Dependency::convert(&(Util::tuple32(tpl.clone()))),
                        Util::tuple33(tpl.clone()),
                    );
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            }),
            solved_crefs: ({
                let mut __acc: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>> = metamodelica::nil();
                for mut cref in (row.solved_crefs.clone()).into_iter().cloned() {
                    let __x = ComponentRef::toDAE(&(cref.clone()))?;
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            }),
        };
        Ok(oldrow)
    }

    pub(crate) fn toString(mut row: &metamodelica::Ref<SparsityRow>) -> Result<ArcStr> {
        pub(crate) fn dependencyString(
            mut tpl: (
                metamodelica::Ref<ComponentRef::NFComponentRef>,
                metamodelica::Ref<Dependency::Dependency>,
                bool,
            ),
        ) -> Result<ArcStr> {
            let mut r#str: ArcStr = {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("("));
                __mm_s.push_str(&*ComponentRef::toString(&(Util::tuple31(tpl.clone())))?);
                __mm_s.push_str(&*literal!(", "));
                __mm_s.push_str(&*Adjacency::Dependency::toString(&(Util::tuple32(tpl.clone())))?);
                __mm_s.push_str(&*literal!(", "));
                __mm_s.push_str(&*boolString(Util::tuple33(tpl.clone())));
                __mm_s.push_str(&*literal!(")"));
                ArcStr::from(__mm_s)
            };
            Ok(r#str)
        }

        let mut r#str: ArcStr;
        r#str = {
            let mut __mm_s = String::new();
            __mm_s.push_str(&*ComponentRef::toString(&row.equation_name)?);
            __mm_s.push_str(&*literal!(" ... "));
            __mm_s.push_str(&*List::toString(
                row.solved_crefs.clone(),
                &move |__a0: metamodelica::Ref<ComponentRef::NFComponentRef>| ComponentRef::toString(&__a0),
                List::Style::FLAT_CURLY.clone(),
            )?);
            __mm_s.push_str(&*literal!(" ... "));
            __mm_s.push_str(&*List::toString(
                row.dependencies.clone(),
                &dependencyString,
                List::Style::FLAT_CURLY.clone(),
            )?);
            ArcStr::from(__mm_s)
        };
        Ok(r#str)
    }

    pub(crate) fn mergeDuplicateRows(
        mut rows_in: metamodelica::List<metamodelica::Ref<SparsityRow>>,
        mut numberOfResultVars: i32,
    ) -> Result<metamodelica::List<metamodelica::Ref<SparsityRow>>> {
        let mut rows_out: metamodelica::List<metamodelica::Ref<SparsityRow>>;
        let mut row_map: metamodelica::Ref<UnorderedMap::UnorderedMap<ArcStr, metamodelica::Ref<SparsityRow>>>;
        let mut order: metamodelica::List<ArcStr>;
        let mut key: ArcStr;
        let mut existing: metamodelica::Ref<SparsityRow>;
        let mut merged: metamodelica::Ref<SparsityRow>;
        if ((rows_in).len() as i32) <= numberOfResultVars {
            rows_out = rows_in;
        } else {
            row_map = UnorderedMap::new(
                (std::sync::Arc::new(fnptr!(stringHashDjb2, ArcStr))
                    as std::sync::Arc<dyn ::std::ops::Fn(ArcStr) -> Result<i32> + 'static>),
                (std::sync::Arc::new(fnptr!(stringEq, ArcStr, ArcStr))
                    as std::sync::Arc<dyn ::std::ops::Fn(ArcStr, ArcStr) -> Result<bool> + 'static>),
                1,
            );
            order = metamodelica::nil();
            for mut row in &*rows_in {
                key = stringDelimitList(
                    ({
                        let mut __acc: metamodelica::List<ArcStr> = metamodelica::nil();
                        for mut c in (row.solved_crefs.clone()).into_iter().cloned() {
                            let __x = ComponentRef::toString(&(c.clone()))?;
                            __acc = cons(__x, __acc);
                        }
                        __acc.reverse()
                    }),
                    literal!(","),
                );
                if UnorderedMap::contains(key.clone(), row_map.clone())? {
                    existing = UnorderedMap::getSafe(
                        key.clone(),
                        row_map.clone(),
                        metamodelica::sourceInfo!("NSimCode/NSimJacobian.mo"),
                    )?;
                    merged = existing.clone();
                    assign_field!(
                        merged.dependencies =
                            List::unionOnTrue(&existing.dependencies, &row.dependencies, &dependencyCrefEqual)?
                    );
                    UnorderedMap::add(key, merged, row_map.clone())?;
                } else {
                    UnorderedMap::add(key.clone(), row.clone(), row_map.clone())?;
                    order = metamodelica::cons(key, order);
                }
            }
            order = order.reverse();
            rows_out = ({
                let mut __acc: metamodelica::List<metamodelica::Ref<SparsityRow>> = metamodelica::nil();
                for mut k in (order).into_iter().cloned() {
                    let __x = UnorderedMap::getSafe(
                        k.clone(),
                        row_map.clone(),
                        metamodelica::sourceInfo!("NSimCode/NSimJacobian.mo"),
                    )?;
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            });
        }
        Ok(rows_out)
    }

    pub(crate) fn sortByResultVars(
        mut rows_in: metamodelica::List<metamodelica::Ref<SparsityRow>>,
        mut resVars: &metamodelica::List<metamodelica::Ref<SimVar::SimVar>>,
    ) -> Result<metamodelica::List<metamodelica::Ref<SparsityRow>>> {
        let mut rows_out: metamodelica::List<metamodelica::Ref<SparsityRow>> = rows_in.clone();
        let mut first_index: metamodelica::Ref<
            UnorderedMap::UnorderedMap<metamodelica::Ref<ComponentRef::NFComponentRef>, i32>,
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
        let mut name: metamodelica::Ref<ComponentRef::NFComponentRef>;
        let mut keyed: metamodelica::List<(i32, i32, metamodelica::Ref<SparsityRow>)> = metamodelica::nil();
        let mut key: i32;
        let mut pos: i32 = 0;
        for mut sv in &**resVars {
            name = ComponentRef::stripSubscriptsAll(&sv.name);
            UnorderedMap::add(
                name.clone(),
                intMin(
                    sv.index.clone(),
                    UnorderedMap::getOrDefault(name, first_index.clone(), sv.index.clone())?,
                ),
                first_index.clone(),
            )?;
        }
        for mut row in &*rows_in {
            key = -1;
            for mut cref in &*row.solved_crefs.clone() {
                let () = (match UnorderedMap::get(
                    ComponentRef::stripSubscriptsAll(metamodelica::AsArg::as_arg(&cref)),
                    first_index.clone(),
                )? {
                    Some(mut idx) => {
                        key = if (key < 0) { idx } else { intMin(key, idx) };
                        ()
                    }
                    _ => (),
                });
            }
            if key < 0 {
                return Ok(rows_out);
            }
            keyed = metamodelica::cons((key, pos, row.clone()), keyed);
            pos = pos + 1;
        }
        keyed = List::sort(
            keyed,
            (std::sync::Arc::new(
                move |__a0: (i32, i32, metamodelica::Ref<SparsityRow>),
                      __a1: (i32, i32, metamodelica::Ref<SparsityRow>)|
                      -> metamodelica::Result<_> {
                    ::std::result::Result::Ok(rowKeyGreater(&__a0, &__a1))
                },
            )
                as std::sync::Arc<
                    dyn ::std::ops::Fn(
                            (i32, i32, metamodelica::Ref<SparsityRow>),
                            (i32, i32, metamodelica::Ref<SparsityRow>),
                        ) -> Result<bool>
                        + 'static,
                >),
        )?;
        rows_out = ({
            let mut __acc: metamodelica::List<metamodelica::Ref<SparsityRow>> = metamodelica::nil();
            for mut t in (keyed).into_iter().cloned() {
                let __x = Util::tuple33(t.clone());
                __acc = cons(__x, __acc);
            }
            __acc.reverse()
        });
        Ok(rows_out)
    }

    pub(crate) fn rowKeyGreater(
        mut row1: &(i32, i32, metamodelica::Ref<SparsityRow>),
        mut row2: &(i32, i32, metamodelica::Ref<SparsityRow>),
    ) -> bool {
        let mut b: bool;
        let mut key1: i32;
        let mut key2: i32;
        let mut pos1: i32;
        let mut pos2: i32;
        (key1, pos1, _) = row1.clone();
        (key2, pos2, _) = row2.clone();
        b = key1 > key2 || key1 == key2 && pos1 > pos2;
        b
    }

    pub(crate) fn dependencyCrefEqual(
        mut dep1: (
            metamodelica::Ref<ComponentRef::NFComponentRef>,
            metamodelica::Ref<Dependency::Dependency>,
            bool,
        ),
        mut dep2: (
            metamodelica::Ref<ComponentRef::NFComponentRef>,
            metamodelica::Ref<Dependency::Dependency>,
            bool,
        ),
    ) -> Result<bool> {
        let mut b: bool;
        b = ComponentRef::isEqual(&(Util::tuple31(dep1)), &(Util::tuple31(dep2)))?;
        Ok(b)
    }
}

pub mod Sparsity {
    use super::*;
    #[derive(Clone, Debug, Eq, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
    pub(crate) enum Sparsity {
        SPARSITY {
            rows: metamodelica::List<metamodelica::Ref<SparsityRow::SparsityRow>>,
        },
        EMPTY,
    }
    impl metamodelica::gc::MMTrace for Sparsity {
        fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
            match self {
                Sparsity::SPARSITY { rows } => {
                    metamodelica::gc::MMTrace::mm_accept(rows, __mmv)?;
                    Ok(())
                }
                Sparsity::EMPTY => Ok(()),
            }
        }
    }
    impl Sparsity {
        pub fn interned_EMPTY() -> metamodelica::Ref<Sparsity> {
            thread_local! {
                static INTERNED: metamodelica::Ref<Sparsity> = metamodelica::Ref::new(Sparsity::EMPTY);
            }
            INTERNED.with(|i| i.clone())
        }
    }
    pub fn interned_EMPTY() -> metamodelica::Ref<Sparsity> {
        Sparsity::interned_EMPTY()
    }
    impl Default for Sparsity {
        fn default() -> Self {
            Self::EMPTY
        }
    }
    pub(crate) use self::Sparsity::{EMPTY, SPARSITY};
    pub(crate) fn create(
        mut mat: &metamodelica::Ref<Adjacency::Matrix::Matrix>,
        mut resVars: metamodelica::List<metamodelica::Ref<SimVar::SimVar>>,
        mut isAdjoint: bool,
    ) -> Result<metamodelica::Ref<Sparsity>> {
        let mut sparsity: metamodelica::Ref<Sparsity>;
        let mut rows: metamodelica::List<metamodelica::Ref<SparsityRow::SparsityRow>>;
        sparsity = (match &**mat {
            Adjacency::Matrix::SPARSITY { .. } => {
                rows = SparsityRow::mergeDuplicateRows(
                    ({
                        let mut __acc: metamodelica::List<metamodelica::Ref<SparsityRow::SparsityRow>> =
                            metamodelica::nil();
                        let __thr_src0 =
                            var_field!((**mat).equation_names, Adjacency::Matrix::Matrix::SPARSITY).clone();
                        let __thr_borrow0 = __thr_src0.borrow();
                        let mut __thr_it0 = __thr_borrow0.iter().cloned();
                        let __thr_src1 =
                            var_field!((**mat).equation_iterators, Adjacency::Matrix::Matrix::SPARSITY).clone();
                        let __thr_borrow1 = __thr_src1.borrow();
                        let mut __thr_it1 = __thr_borrow1.iter().cloned();
                        let __thr_src2 = var_field!((**mat).dependencies, Adjacency::Matrix::Matrix::SPARSITY).clone();
                        let __thr_borrow2 = __thr_src2.borrow();
                        let mut __thr_it2 = __thr_borrow2.iter().cloned();
                        let __thr_src3 = var_field!((**mat).repetitions, Adjacency::Matrix::Matrix::SPARSITY).clone();
                        let __thr_borrow3 = __thr_src3.borrow();
                        let mut __thr_it3 = __thr_borrow3.iter().cloned();
                        let __thr_src4 = var_field!((**mat).solved_crefs, Adjacency::Matrix::Matrix::SPARSITY).clone();
                        let __thr_borrow4 = __thr_src4.borrow();
                        let mut __thr_it4 = __thr_borrow4.iter().cloned();
                        loop {
                            match (
                                __thr_it0.next(),
                                __thr_it1.next(),
                                __thr_it2.next(),
                                __thr_it3.next(),
                                __thr_it4.next(),
                            ) {
                                (Some(e), Some(i), Some(d), Some(r), Some(s)) => {
                                    let __x =
                                        SparsityRow::create(e.clone(), &(i.clone()), d.clone(), r.clone(), s.clone())?;
                                    __acc = cons(__x, __acc);
                                }
                                (None, None, None, None, None) => break,
                                _ => return Err("threaded for: ranges of unequal length"),
                            }
                        }
                        __acc.reverse()
                    }),
                    SimVars::numScalarElems(resVars.clone()),
                )?;
                if !(isAdjoint) {
                    rows = SparsityRow::sortByResultVars(rows, &resVars)?;
                }
                metamodelica::Ref::new(Sparsity::SPARSITY { rows: rows })
            }
            Adjacency::Matrix::EMPTY { .. } => crate::NSimJacobian::Sparsity::interned_EMPTY(),
            _ => {
                Error::addMessage(
                    Error::INTERNAL_ERROR.clone(),
                    list![{
                        let mut __mm_s = String::new();
                        __mm_s.push_str(&*literal!("NSimJacobian.Sparsity.create"));
                        __mm_s.push_str(&*literal!(" can only handle sparsity or empty matrices but got:\n"));
                        __mm_s.push_str(&*Adjacency::Matrix::toString(mat, literal!(""))?);
                        ArcStr::from(__mm_s)
                    }],
                )?;
                return Err("fail");
            }
        });
        Ok(sparsity)
    }

    pub(crate) fn convert(mut sparsity: &metamodelica::Ref<Sparsity>) -> Result<OldSimCode::Sparsity> {
        let mut oldsparsity: OldSimCode::Sparsity;
        oldsparsity = (match &**sparsity {
            SPARSITY { rows: __sparsity_rows } => OldSimCode::Sparsity::SPARSITY {
                rows: ({
                    let mut __acc: metamodelica::List<OldSimCode::SparsityRow> = metamodelica::nil();
                    for mut row in (__sparsity_rows.clone()).into_iter().cloned() {
                        let __x = SparsityRow::convert(&(row.clone()))?;
                        __acc = cons(__x, __acc);
                    }
                    __acc.reverse()
                }),
            },
            EMPTY { .. } => openmodelica_simcode_types::SimCode::Sparsity::EMPTY,
        });
        Ok(oldsparsity)
    }

    pub(crate) fn toString(mut sparsity: &metamodelica::Ref<Sparsity>) -> Result<ArcStr> {
        let mut r#str: ArcStr = StringUtil::headline_3(&(literal!("Resizable Sparsity Pattern")))?;
        r#str = (match &**sparsity {
            SPARSITY { rows: __sparsity_rows } => {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*r#str);
                __mm_s.push_str(&*List::toString(
                    __sparsity_rows.clone(),
                    &move |__a0: metamodelica::Ref<SparsityRow::SparsityRow>| SparsityRow::toString(&__a0),
                    List::Style::NEWLINE.clone(),
                )?);
                __mm_s.push_str(&*literal!("\n"));
                ArcStr::from(__mm_s)
            }
            _ => {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*r#str);
                __mm_s.push_str(&*literal!(" -- EMPTY -- \n"));
                ArcStr::from(__mm_s)
            }
        });
        Ok(r#str)
    }
}

pub mod SimJacobian {
    use super::*;
    #[derive(Clone, Debug, Eq, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
    pub struct SimJacobian {
        /// unique matrix name
        pub name: ArcStr,
        /// unique jacobian index
        pub jacobianIndex: i32,
        /// index of partition it belongs to
        pub partitionIndex: i32,
        /// corresponds to the number of rows
        pub numberOfResultVars: i32,
        /// column equations equals in size to column vars
        pub columnEqns: metamodelica::List<metamodelica::Ref<SimStrongComponent::Block::Block>>,
        /// List of constant equations independent of seed variables
        pub constantEqns: metamodelica::List<metamodelica::Ref<SimStrongComponent::Block::Block>>,
        /// all column vars, none results vars index -1, the other corresponding to rows index
        pub columnVars: metamodelica::List<metamodelica::Ref<SimVar::SimVar>>,
        /// corresponds to the number of columns
        pub seedVars: metamodelica::List<metamodelica::Ref<SimVar::SimVar>>,
        /// new sparsity pattern
        pub sparsityMatrix: metamodelica::Ref<Sparsity::Sparsity>,
        /// Generic for-loop and array calls
        pub generic_loop_calls: metamodelica::List<metamodelica::Ref<SimGenericCall::NSimGenericCall>>,
        /// hash table for cref -> simVar
        pub jac_map: Option<
            metamodelica::Ref<
                UnorderedMap::UnorderedMap<
                    metamodelica::Ref<ComponentRef::NFComponentRef>,
                    metamodelica::Ref<SimVar::SimVar>,
                >,
            >,
        >,
        /// indicates if this is an adjoint jacobian
        pub isAdjoint: bool,
        /// indicates if this jacobian is part of a bidirectional pair
        pub isBidirectional: bool,
        /// index of the adjoint jacobian for bidirectional (-1 if not bidirectional)
        pub adjointJacobianIndex: i32,
        /// matrix name of the adjoint jacobian for bidirectional
        pub adjointMatrixName: ArcStr,
    }

    impl metamodelica::gc::MMTrace for SimJacobian {
        fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
            metamodelica::gc::MMTrace::mm_accept(&self.name, __mmv)?;
            metamodelica::gc::MMTrace::mm_accept(&self.jacobianIndex, __mmv)?;
            metamodelica::gc::MMTrace::mm_accept(&self.partitionIndex, __mmv)?;
            metamodelica::gc::MMTrace::mm_accept(&self.numberOfResultVars, __mmv)?;
            metamodelica::gc::MMTrace::mm_accept(&self.columnEqns, __mmv)?;
            metamodelica::gc::MMTrace::mm_accept(&self.constantEqns, __mmv)?;
            metamodelica::gc::MMTrace::mm_accept(&self.columnVars, __mmv)?;
            metamodelica::gc::MMTrace::mm_accept(&self.seedVars, __mmv)?;
            metamodelica::gc::MMTrace::mm_accept(&self.sparsityMatrix, __mmv)?;
            metamodelica::gc::MMTrace::mm_accept(&self.generic_loop_calls, __mmv)?;
            metamodelica::gc::MMTrace::mm_accept(&self.jac_map, __mmv)?;
            metamodelica::gc::MMTrace::mm_accept(&self.isAdjoint, __mmv)?;
            metamodelica::gc::MMTrace::mm_accept(&self.isBidirectional, __mmv)?;
            metamodelica::gc::MMTrace::mm_accept(&self.adjointJacobianIndex, __mmv)?;
            metamodelica::gc::MMTrace::mm_accept(&self.adjointMatrixName, __mmv)?;
            Ok(())
        }
    }
    impl Default for SimJacobian {
        fn default() -> Self {
            Self {
                name: Default::default(),
                jacobianIndex: Default::default(),
                partitionIndex: Default::default(),
                numberOfResultVars: Default::default(),
                columnEqns: Default::default(),
                constantEqns: Default::default(),
                columnVars: Default::default(),
                seedVars: Default::default(),
                sparsityMatrix: Default::default(),
                generic_loop_calls: Default::default(),
                jac_map: Default::default(),
                isAdjoint: Default::default(),
                isBidirectional: Default::default(),
                adjointJacobianIndex: Default::default(),
                adjointMatrixName: Default::default(),
            }
        }
    }

    pub type SIM_JAC = SimJacobian;

    pub(crate) fn toString(mut simJac: &metamodelica::Ref<SimJacobian>) -> Result<ArcStr> {
        let mut r#str: ArcStr = literal!("");
        r#str = (match &**simJac {
            SimJacobian { .. } => {
                if isEmpty(simJac) {
                    r#str = {
                        let mut __mm_s = String::new();
                        __mm_s.push_str(&*StringUtil::headline_2(
                            &({
                                let mut __mm_s = String::new();
                                __mm_s.push_str(&*literal!("[EMPTY] SimCode Jacobian "));
                                __mm_s.push_str(&*simJac.name);
                                __mm_s.push_str(&*literal!("(idx = "));
                                __mm_s.push_str(&*intString(simJac.jacobianIndex.clone()));
                                __mm_s.push_str(&*literal!(", partition = "));
                                __mm_s.push_str(&*intString(simJac.partitionIndex.clone()));
                                __mm_s.push_str(&*literal!(")"));
                                ArcStr::from(__mm_s)
                            }),
                        )?);
                        __mm_s.push_str(&*literal!("\n"));
                        ArcStr::from(__mm_s)
                    };
                } else {
                    r#str = {
                        let mut __mm_s = String::new();
                        __mm_s.push_str(&*StringUtil::headline_2(
                            &({
                                let mut __mm_s = String::new();
                                __mm_s.push_str(&*literal!("SimCode Jacobian "));
                                __mm_s.push_str(&*simJac.name);
                                __mm_s.push_str(&*literal!("(idx = "));
                                __mm_s.push_str(&*intString(simJac.jacobianIndex.clone()));
                                __mm_s.push_str(&*literal!(", partition = "));
                                __mm_s.push_str(&*intString(simJac.jacobianIndex.clone()));
                                __mm_s.push_str(&*literal!(")"));
                                ArcStr::from(__mm_s)
                            }),
                        )?);
                        __mm_s.push_str(&*literal!("\n"));
                        ArcStr::from(__mm_s)
                    };
                    r#str = {
                        let mut __mm_s = String::new();
                        __mm_s.push_str(&*r#str);
                        __mm_s.push_str(&*StringUtil::headline_4(
                            &({
                                let mut __mm_s = String::new();
                                __mm_s.push_str(&*literal!("SeedVars (size = "));
                                __mm_s.push_str(&*intString(((simJac.seedVars).len() as i32)));
                                __mm_s.push_str(&*literal!(")"));
                                ArcStr::from(__mm_s)
                            }),
                        )?);
                        ArcStr::from(__mm_s)
                    };
                    for mut var in &*simJac.seedVars.clone() {
                        r#str = {
                            let mut __mm_s = String::new();
                            __mm_s.push_str(&*r#str);
                            __mm_s.push_str(&*SimVar::toString(metamodelica::AsArg::as_arg(&var), literal!("  "))?);
                            __mm_s.push_str(&*literal!("\n"));
                            ArcStr::from(__mm_s)
                        };
                    }
                    r#str = {
                        let mut __mm_s = String::new();
                        __mm_s.push_str(&*r#str);
                        __mm_s.push_str(&*literal!("\n"));
                        __mm_s.push_str(&*StringUtil::headline_4(
                            &({
                                let mut __mm_s = String::new();
                                __mm_s.push_str(&*literal!("TmpVars (size = "));
                                __mm_s.push_str(&*intString(((simJac.columnVars).len() as i32)));
                                __mm_s.push_str(&*literal!(")"));
                                ArcStr::from(__mm_s)
                            }),
                        )?);
                        ArcStr::from(__mm_s)
                    };
                    for mut var in &*simJac.columnVars.clone() {
                        r#str = {
                            let mut __mm_s = String::new();
                            __mm_s.push_str(&*r#str);
                            __mm_s.push_str(&*SimVar::toString(metamodelica::AsArg::as_arg(&var), literal!("  "))?);
                            __mm_s.push_str(&*literal!("\n"));
                            ArcStr::from(__mm_s)
                        };
                    }
                    r#str = {
                        let mut __mm_s = String::new();
                        __mm_s.push_str(&*r#str);
                        __mm_s.push_str(&*literal!("\n"));
                        __mm_s.push_str(&*StringUtil::headline_4(
                            &({
                                let mut __mm_s = String::new();
                                __mm_s.push_str(&*literal!("ResultVars (size = "));
                                __mm_s.push_str(&*intString(simJac.numberOfResultVars.clone()));
                                __mm_s.push_str(&*literal!(")"));
                                ArcStr::from(__mm_s)
                            }),
                        )?);
                        ArcStr::from(__mm_s)
                    };
                    r#str = {
                        let mut __mm_s = String::new();
                        __mm_s.push_str(&*r#str);
                        __mm_s.push_str(&*literal!("\n"));
                        __mm_s.push_str(&*StringUtil::headline_3(
                            &({
                                let mut __mm_s = String::new();
                                __mm_s.push_str(&*literal!("Column Equations (size = "));
                                __mm_s.push_str(&*intString(((simJac.columnEqns).len() as i32)));
                                __mm_s.push_str(&*literal!(")"));
                                ArcStr::from(__mm_s)
                            }),
                        )?);
                        ArcStr::from(__mm_s)
                    };
                    for mut eq in &*simJac.columnEqns.clone() {
                        r#str = {
                            let mut __mm_s = String::new();
                            __mm_s.push_str(&*r#str);
                            __mm_s.push_str(&*SimStrongComponent::Block::toString(
                                metamodelica::AsArg::as_arg(&eq),
                                literal!("  "),
                            )?);
                            ArcStr::from(__mm_s)
                        };
                    }
                    if !((simJac.constantEqns).is_empty()) {
                        r#str = {
                            let mut __mm_s = String::new();
                            __mm_s.push_str(&*r#str);
                            __mm_s.push_str(&*StringUtil::headline_3(&(literal!("Constant Equations")))?);
                            ArcStr::from(__mm_s)
                        };
                        for mut eq in &*simJac.constantEqns.clone() {
                            r#str = {
                                let mut __mm_s = String::new();
                                __mm_s.push_str(&*r#str);
                                __mm_s.push_str(&*SimStrongComponent::Block::toString(
                                    metamodelica::AsArg::as_arg(&eq),
                                    literal!("  "),
                                )?);
                                ArcStr::from(__mm_s)
                            };
                        }
                    }
                    r#str = {
                        let mut __mm_s = String::new();
                        __mm_s.push_str(&*r#str);
                        __mm_s.push_str(&*literal!("\n"));
                        __mm_s.push_str(&*Sparsity::toString(&simJac.sparsityMatrix)?);
                        ArcStr::from(__mm_s)
                    };
                    if !((simJac.generic_loop_calls).is_empty()) {
                        r#str = {
                            let mut __mm_s = String::new();
                            __mm_s.push_str(&*r#str);
                            __mm_s.push_str(&*StringUtil::headline_3(&(literal!("Generic Calls")))?);
                            ArcStr::from(__mm_s)
                        };
                        r#str = {
                            let mut __mm_s = String::new();
                            __mm_s.push_str(&*r#str);
                            __mm_s.push_str(&*List::toString(
                                simJac.generic_loop_calls.clone(),
                                &move |__a0: metamodelica::Ref<SimGenericCall::NSimGenericCall>| {
                                    SimGenericCall::toString(&__a0)
                                },
                                List::Style::NEWLINE_INDENT.clone(),
                            )?);
                            ArcStr::from(__mm_s)
                        };
                    }
                    r#str = {
                        let mut __mm_s = String::new();
                        __mm_s.push_str(&*r#str);
                        __mm_s.push_str(&*literal!("\n"));
                        ArcStr::from(__mm_s)
                    };
                }
                r#str
            }
            _ => {
                Error::addMessage(
                    Error::INTERNAL_ERROR.clone(),
                    list![{
                        let mut __mm_s = String::new();
                        __mm_s.push_str(&*literal!("NSimJacobian.SimJacobian.toString"));
                        __mm_s.push_str(&*literal!(" failed."));
                        ArcStr::from(__mm_s)
                    }],
                )?;
                return Err("fail");
            }
        });
        Ok(r#str)
    }

    pub(crate) fn isEmpty(mut simJac: &metamodelica::Ref<SimJacobian>) -> bool {
        let mut b: bool;
        b = (match &**simJac {
            SimJacobian { .. } => simJac.numberOfResultVars.clone() == 0,
            _ => false,
        });
        b
    }

    pub(crate) fn create(
        mut jacobian: &metamodelica::Ref<BackendDAE::NBackendDAE>,
        mut indices: SimCode::SimCodeIndices,
        mut simcode_map: metamodelica::Ref<
            UnorderedMap::UnorderedMap<
                metamodelica::Ref<ComponentRef::NFComponentRef>,
                metamodelica::Ref<SimVar::SimVar>,
            >,
        >,
    ) -> Result<(Option<metamodelica::Ref<SimJacobian>>, SimCode::SimCodeIndices)> {
        let mut simJacobian: Option<metamodelica::Ref<SimJacobian>>;
        let mut indices: SimCode::SimCodeIndices = indices;
        simJacobian = ({
            let mut dummy_sim_map: metamodelica::Ref<
                UnorderedMap::UnorderedMap<
                    metamodelica::Ref<ComponentRef::NFComponentRef>,
                    metamodelica::Ref<SimVar::SimVar>,
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
            let mut dummy_eqn_map: metamodelica::Ref<
                UnorderedMap::UnorderedMap<
                    metamodelica::Ref<ComponentRef::NFComponentRef>,
                    metamodelica::Ref<SimStrongComponent::Block::Block>,
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
            let mut columnEqns: metamodelica::List<metamodelica::Ref<SimStrongComponent::Block::Block>> =
                metamodelica::nil();
            (::match_deref::match_deref! { match jacobian {
                Deref @ BackendDAE::JACOBIAN { varData: varData @ Deref @ BVariable::VarData::VAR_DATA_JAC { .. }, isAdjoint: __jacobian_isAdjoint, jacType: __jacobian_jacType, name: __jacobian_name, sparsity: __jacobian_sparsity, .. } => {
                    let mut columnEqn: metamodelica::Ref<SimStrongComponent::Block::Block>;
                    let mut seed_lst: metamodelica::List<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>>;
                    let mut res_lst: metamodelica::List<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>>;
                    let mut tmp_lst: metamodelica::List<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>>;
                    let mut seedVars: metamodelica::List<metamodelica::Ref<SimVar::SimVar>>;
                    let mut resVars: metamodelica::List<metamodelica::Ref<SimVar::SimVar>>;
                    let mut tmpVars: metamodelica::List<metamodelica::Ref<SimVar::SimVar>>;
                    let mut jac_map: metamodelica::Ref<UnorderedMap::UnorderedMap<metamodelica::Ref<ComponentRef::NFComponentRef>, metamodelica::Ref<SimVar::SimVar>>>;
                    let mut jac: metamodelica::Ref<SimJacobian>;
                    let mut sim_map: metamodelica::Ref<UnorderedMap::UnorderedMap<metamodelica::Ref<Identifier::Identifier>, i32>>;
                    let mut generic_loop_calls: metamodelica::List<metamodelica::Ref<SimGenericCall::NSimGenericCall>>;
                    let mut min_sub_map: metamodelica::Ref<UnorderedMap::UnorderedMap<metamodelica::Ref<ComponentRef::NFComponentRef>, i32>>;
                    let mut min_sv_map: metamodelica::Ref<UnorderedMap::UnorderedMap<metamodelica::Ref<ComponentRef::NFComponentRef>, metamodelica::Ref<SimVar::SimVar>>>;
                    sim_map = indices.generic_call_map.clone();
                    indices.generic_call_map = UnorderedMap::new((std::sync::Arc::new(move |__a0: metamodelica::Ref<Identifier::Identifier>| SimCode::Identifier::hash(&__a0)) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<Identifier::Identifier>) -> Result<i32> + 'static>), (std::sync::Arc::new(move |__a0: metamodelica::Ref<Identifier::Identifier>, __a1: metamodelica::Ref<Identifier::Identifier>| SimCode::Identifier::isEqual(&__a0, &__a1)) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<Identifier::Identifier>, metamodelica::Ref<Identifier::Identifier>) -> Result<bool> + 'static>), 1);
                    for mut i in ({let __s=metamodelica::arrayLength(var_field!((**jacobian).comps, BackendDAE::NBackendDAE::JACOBIAN).clone()); let __e=1; (0i32..).map(move |__k| __s + __k * (-1)).take_while(move |&__v| __v >= __e)}) {
                        (columnEqn, indices, _) = SimStrongComponent::Block::fromStrongComponent(&({let __elt = (*metamodelica::index_checked(&var_field!((**jacobian).comps, BackendDAE::NBackendDAE::JACOBIAN).borrow(), i)?).clone(); __elt}), indices, Partition::Kind::JAC.clone(), dummy_sim_map.clone(), dummy_eqn_map.clone())?;
                        columnEqns = metamodelica::cons(columnEqn, columnEqns);
                    }
                    generic_loop_calls = ({
                let mut __acc: metamodelica::List<metamodelica::Ref<SimGenericCall::NSimGenericCall>> = metamodelica::nil();
                for mut tpl in (UnorderedMap::toList(indices.generic_call_map.clone())).into_iter().cloned() {
                    let __x = SimGenericCall::fromIdentifier(&(tpl.clone()))?;
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            });
                    indices.generic_call_map = sim_map;
                    if Flags::getConfigBool(Flags::SIM_CODE_SCALARIZE.clone())? {
                        seed_lst = BVariable::VariablePointers::toList(&(BVariable::VariablePointers::scalarize(var_field!((**varData).seedVars, VarData::VarData::VAR_DATA_JAC).clone())?))?;
                        res_lst = BVariable::VariablePointers::toList(&(BVariable::VariablePointers::scalarize(var_field!((**varData).resultVars, VarData::VarData::VAR_DATA_JAC).clone())?))?;
                        tmp_lst = BVariable::VariablePointers::toList(&(BVariable::VariablePointers::scalarize(var_field!((**varData).tmpVars, VarData::VarData::VAR_DATA_JAC).clone())?))?;
                    } else {
                        seed_lst = BVariable::VariablePointers::toList(var_field!((**varData).seedVars, VarData::VarData::VAR_DATA_JAC))?;
                        res_lst = BVariable::VariablePointers::toList(var_field!((**varData).resultVars, VarData::VarData::VAR_DATA_JAC))?;
                        tmp_lst = BVariable::VariablePointers::toList(var_field!((**varData).tmpVars, VarData::VarData::VAR_DATA_JAC))?;
                    }
                    if __jacobian_jacType.clone() == Jacobian::JacobianType::ODE.clone() && !(__jacobian_isAdjoint.clone()) {
                        seed_lst = sortByStateIndex(seed_lst, simcode_map.clone())?;
                        res_lst = sortByStateIndex(res_lst, simcode_map)?;
                    }
                    (seedVars, _) = SimVar::createList(&seed_lst, VarType::SIMULATION.clone(), SimCode::EMPTY_SIM_CODE_INDICES())?;
                    (resVars, _) = SimVar::createList(&res_lst, VarType::SIMULATION.clone(), SimCode::EMPTY_SIM_CODE_INDICES())?;
                    (tmpVars, _) = SimVar::createList(&tmp_lst, VarType::SIMULATION.clone(), SimCode::EMPTY_SIM_CODE_INDICES())?;
                    jac_map = UnorderedMap::new((std::sync::Arc::new(move |__a0: metamodelica::Ref<ComponentRef::NFComponentRef>| ComponentRef::hash(&__a0)) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<ComponentRef::NFComponentRef>) -> Result<i32> + 'static>), (std::sync::Arc::new(move |__a0: metamodelica::Ref<ComponentRef::NFComponentRef>, __a1: metamodelica::Ref<ComponentRef::NFComponentRef>| ComponentRef::isEqual(&__a0, &__a1)) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<ComponentRef::NFComponentRef>, metamodelica::Ref<ComponentRef::NFComponentRef>) -> Result<bool> + 'static>), ((seedVars).len() as i32) + ((resVars).len() as i32) + ((tmpVars).len() as i32));
                    SimCodeUtil::addListSimCodeMap(&seedVars, jac_map.clone())?;
                    SimCodeUtil::addListSimCodeMap(&resVars, jac_map.clone())?;
                    SimCodeUtil::addListSimCodeMap(&tmpVars, jac_map.clone())?;
                    min_sub_map = UnorderedMap::new((std::sync::Arc::new(move |__a0: metamodelica::Ref<ComponentRef::NFComponentRef>| ComponentRef::hash(&__a0)) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<ComponentRef::NFComponentRef>) -> Result<i32> + 'static>), (std::sync::Arc::new(move |__a0: metamodelica::Ref<ComponentRef::NFComponentRef>, __a1: metamodelica::Ref<ComponentRef::NFComponentRef>| ComponentRef::isEqual(&__a0, &__a1)) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<ComponentRef::NFComponentRef>, metamodelica::Ref<ComponentRef::NFComponentRef>) -> Result<bool> + 'static>), 1);
                    min_sv_map = UnorderedMap::new((std::sync::Arc::new(move |__a0: metamodelica::Ref<ComponentRef::NFComponentRef>| ComponentRef::hash(&__a0)) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<ComponentRef::NFComponentRef>) -> Result<i32> + 'static>), (std::sync::Arc::new(move |__a0: metamodelica::Ref<ComponentRef::NFComponentRef>, __a1: metamodelica::Ref<ComponentRef::NFComponentRef>| ComponentRef::isEqual(&__a0, &__a1)) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<ComponentRef::NFComponentRef>, metamodelica::Ref<ComponentRef::NFComponentRef>) -> Result<bool> + 'static>), 1);
                    for mut sv in &*seedVars {
                        if ComponentRef::hasSubscripts(&sv.name)? {
                            let () = (match ComponentRef::outermostIntegerSubscript(&sv.name) {
                mut sub_val if (sub_val > 1) => {
                    let mut base_cref: metamodelica::Ref<ComponentRef::NFComponentRef>;
                    let mut cur_min: i32;
                    base_cref = ComponentRef::stripSubscriptsAll(&sv.name);
                    if UnorderedMap::contains(base_cref.clone(), min_sub_map.clone())? {
                        cur_min = UnorderedMap::getSafe(base_cref.clone(), min_sub_map.clone(), metamodelica::sourceInfo!("NSimCode/NSimJacobian.mo"))?;
                        if sub_val < cur_min {
                            UnorderedMap::add(base_cref.clone(), sub_val, min_sub_map.clone())?;
                            UnorderedMap::add(base_cref.clone(), sv.clone(), min_sv_map.clone())?;
                        }
                    } else {
                        UnorderedMap::add(base_cref.clone(), sub_val, min_sub_map.clone())?;
                        UnorderedMap::add(base_cref.clone(), sv.clone(), min_sv_map.clone())?;
                    }
                    ()
                },
                _ => {
                    ()
                },
            });
                        }
                    }
                    for mut tpl in &*UnorderedMap::toList(min_sv_map) {
                        let () = (::match_deref::match_deref! { match &(tpl.clone()) {
                (base_cref, min_sv) => {
                    let mut virtual_sv: metamodelica::Ref<SimVar::SimVar>;
                    let mut cur_min: i32;
                    cur_min = UnorderedMap::getSafe(base_cref.clone(), min_sub_map.clone(), metamodelica::sourceInfo!("NSimCode/NSimJacobian.mo"))?;
                    if !(UnorderedMap::contains(base_cref.clone(), jac_map.clone())?) {
                        virtual_sv = min_sv.clone();
                        assign_field!(
                            virtual_sv.name = base_cref.clone(),
                            virtual_sv.index = min_sv.index.clone() - crefFlatOffset(&min_sv.name)?,
                            virtual_sv.arrayCref = None
                        );
                        UnorderedMap::add(base_cref.clone(), virtual_sv, jac_map.clone())?;
                    }
                    ()
                },
                _ => unreachable!("match_deref! exhaustiveness placeholder"),
            } });
                    }
                    jac = metamodelica::Ref::new(SimJacobian { name: __jacobian_name.clone(), jacobianIndex: indices.jacobianIndex.clone(), partitionIndex: 0, numberOfResultVars: SimVars::numScalarElems(resVars.clone()), columnEqns: columnEqns, constantEqns: metamodelica::nil(), columnVars: tmpVars, seedVars: seedVars, sparsityMatrix: Sparsity::create(metamodelica::AsArg::as_arg(&__jacobian_sparsity), resVars, __jacobian_isAdjoint.clone())?, generic_loop_calls: generic_loop_calls, jac_map: Some(jac_map), isAdjoint: __jacobian_isAdjoint.clone(), isBidirectional: false, adjointJacobianIndex: -1, adjointMatrixName: literal!("") });
                    indices.jacobianIndex = indices.jacobianIndex.clone() + 1;
                    simJacobian = Some(jac);
                    simJacobian
                },
                _ => {
                    Error::addMessage(Error::INTERNAL_ERROR.clone(), list![{ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NSimJacobian.SimJacobian.create")); __mm_s.push_str(&*literal!(" failed.")); ArcStr::from(__mm_s) }])?;
                    return Err("fail")
                },
                _ => unreachable!("match_deref! exhaustiveness placeholder"),
            } })
        });
        Ok((simJacobian, indices))
    }

    pub(crate) fn createSimulationJacobian(
        mut partitions: &metamodelica::List<metamodelica::Ref<Partition::Partition::Partition>>,
        mut simCodeIndices: SimCode::SimCodeIndices,
        mut simcode_map: metamodelica::Ref<
            UnorderedMap::UnorderedMap<
                metamodelica::Ref<ComponentRef::NFComponentRef>,
                metamodelica::Ref<SimVar::SimVar>,
            >,
        >,
    ) -> Result<(
        metamodelica::Ref<SimJacobian>,
        metamodelica::Ref<SimJacobian>,
        SimCode::SimCodeIndices,
    )> {
        let mut simJac: metamodelica::Ref<SimJacobian>;
        let mut simJacAdjoint: metamodelica::Ref<SimJacobian>;
        let mut simCodeIndices: SimCode::SimCodeIndices = simCodeIndices;
        let mut jacobians: metamodelica::List<metamodelica::Ref<BackendDAE::NBackendDAE>> = metamodelica::nil();
        let mut jacobiansAdjoint: metamodelica::List<metamodelica::Ref<BackendDAE::NBackendDAE>> = metamodelica::nil();
        let mut simJacobian: metamodelica::Ref<BackendDAE::NBackendDAE>;
        let mut simJacobianAdjoint: metamodelica::Ref<BackendDAE::NBackendDAE>;
        let mut simJac_opt: Option<metamodelica::Ref<SimJacobian>>;
        let mut simJacAdj_opt: Option<metamodelica::Ref<SimJacobian>>;
        let mut jacobian: Option<metamodelica::Ref<BackendDAE::NBackendDAE>>;
        let mut jacobianAdjoint: Option<metamodelica::Ref<BackendDAE::NBackendDAE>>;
        for mut partition in &**partitions {
            jacobian = Partition::Partition::getJacobian(metamodelica::AsArg::as_arg(&partition));
            if (jacobian).is_some() {
                jacobians = metamodelica::cons(Util::getOption(jacobian)?, jacobians);
            }
            jacobianAdjoint = Partition::Partition::getJacobianAdjoint(metamodelica::AsArg::as_arg(&partition));
            if (jacobianAdjoint).is_some() {
                jacobiansAdjoint = metamodelica::cons(Util::getOption(jacobianAdjoint)?, jacobiansAdjoint);
            }
        }
        if (jacobians).is_empty() {
            (simJac, simCodeIndices) = empty(literal!("A"), simCodeIndices)?;
        } else {
            simJacobian = Jacobian::combine(&jacobians, literal!("A"))?;
            (simJac_opt, simCodeIndices) = create(&simJacobian, simCodeIndices, simcode_map.clone())?;
            if (simJac_opt).is_some() {
                simJac = Util::getOption(simJac_opt)?;
            } else {
                (simJac, simCodeIndices) = empty(literal!("A"), simCodeIndices)?;
            }
        }
        if (jacobiansAdjoint).is_empty() {
            (simJacAdjoint, simCodeIndices) = empty(literal!("ADJ"), simCodeIndices)?;
        } else {
            simJacobianAdjoint = Jacobian::combine(&jacobiansAdjoint, literal!("ADJ"))?;
            (simJacAdj_opt, simCodeIndices) = create(&simJacobianAdjoint, simCodeIndices, simcode_map)?;
            if (simJacAdj_opt).is_some() {
                simJacAdjoint = Util::getOption(simJacAdj_opt)?;
            } else {
                (simJacAdjoint, simCodeIndices) = empty(literal!("ADJ"), simCodeIndices)?;
            }
        }
        if metamodelica::stringEq(
            &(Flags::getConfigString(Flags::GENERATE_DYNAMIC_JACOBIAN.clone())?),
            &(literal!("bidirectional")),
        ) {
            simJac = (match &*simJac {
                SimJacobian { .. } => {
                    assign_field!(simJac.isBidirectional = true);
                    assign_field!(
                        simJac.adjointJacobianIndex = (match &*simJacAdjoint {
                            SimJacobian { .. } => simJacAdjoint.jacobianIndex.clone(),
                            _ => -1,
                        })
                    );
                    assign_field!(
                        simJac.adjointMatrixName = (match &*simJacAdjoint {
                            SimJacobian { .. } => simJacAdjoint.name.clone(),
                            _ => literal!(""),
                        })
                    );
                    simJac
                }
                _ => simJac,
            });
        }
        Ok((simJac, simJacAdjoint, simCodeIndices))
    }

    pub(crate) fn createOptimizationJacobian(
        mut partitions: &metamodelica::List<metamodelica::Ref<Partition::Partition::Partition>>,
        mut simCodeIndices: SimCode::SimCodeIndices,
        mut simcode_map: metamodelica::Ref<
            UnorderedMap::UnorderedMap<
                metamodelica::Ref<ComponentRef::NFComponentRef>,
                metamodelica::Ref<SimVar::SimVar>,
            >,
        >,
    ) -> Result<(
        metamodelica::Ref<SimJacobian>,
        metamodelica::Ref<SimJacobian>,
        metamodelica::Ref<SimJacobian>,
        SimCode::SimCodeIndices,
    )> {
        let mut simJacLfg: metamodelica::Ref<SimJacobian>;
        let mut simJacMrf: metamodelica::Ref<SimJacobian>;
        let mut simJacR0: metamodelica::Ref<SimJacobian>;
        let mut simCodeIndices: SimCode::SimCodeIndices = simCodeIndices;
        let mut jacobiansLfg: metamodelica::List<metamodelica::Ref<BackendDAE::NBackendDAE>> = metamodelica::nil();
        let mut jacobiansMrf: metamodelica::List<metamodelica::Ref<BackendDAE::NBackendDAE>> = metamodelica::nil();
        let mut jacobiansR0: metamodelica::List<metamodelica::Ref<BackendDAE::NBackendDAE>> = metamodelica::nil();
        let mut simJacobianLfg: metamodelica::Ref<BackendDAE::NBackendDAE>;
        let mut simJacobianMrf: metamodelica::Ref<BackendDAE::NBackendDAE>;
        let mut simJacobianR0: metamodelica::Ref<BackendDAE::NBackendDAE>;
        let mut simJacLfg_opt: Option<metamodelica::Ref<SimJacobian>>;
        let mut simJacMrf_opt: Option<metamodelica::Ref<SimJacobian>>;
        let mut simJacR0_opt: Option<metamodelica::Ref<SimJacobian>>;
        let mut jacobianLfg: Option<metamodelica::Ref<BackendDAE::NBackendDAE>>;
        let mut jacobianMrf: Option<metamodelica::Ref<BackendDAE::NBackendDAE>>;
        let mut jacobianR0: Option<metamodelica::Ref<BackendDAE::NBackendDAE>>;
        for mut partition in &**partitions {
            jacobianLfg = Partition::Partition::getJacobianLfg(metamodelica::AsArg::as_arg(&partition));
            if (jacobianLfg).is_some() {
                jacobiansLfg = metamodelica::cons(Util::getOption(jacobianLfg)?, jacobiansLfg);
            }
            jacobianMrf = Partition::Partition::getJacobianMrf(metamodelica::AsArg::as_arg(&partition));
            if (jacobianMrf).is_some() {
                jacobiansMrf = metamodelica::cons(Util::getOption(jacobianMrf)?, jacobiansMrf);
            }
            jacobianR0 = Partition::Partition::getJacobianR0(metamodelica::AsArg::as_arg(&partition));
            if (jacobianR0).is_some() {
                jacobiansR0 = metamodelica::cons(Util::getOption(jacobianR0)?, jacobiansR0);
            }
        }
        if (jacobiansLfg).is_empty() {
            (simJacLfg, simCodeIndices) = empty(literal!("OPT_LFG"), simCodeIndices)?;
        } else {
            simJacobianLfg = Jacobian::combine(&jacobiansLfg, literal!("OPT_LFG"))?;
            (simJacLfg_opt, simCodeIndices) = create(&simJacobianLfg, simCodeIndices, simcode_map.clone())?;
            if (simJacLfg_opt).is_some() {
                simJacLfg = Util::getOption(simJacLfg_opt)?;
            } else {
                (simJacLfg, simCodeIndices) = empty(literal!("OPT_LFG"), simCodeIndices)?;
            }
        }
        if (jacobiansMrf).is_empty() {
            (simJacMrf, simCodeIndices) = empty(literal!("OPT_MRF"), simCodeIndices)?;
        } else {
            simJacobianMrf = Jacobian::combine(&jacobiansMrf, literal!("OPT_MRF"))?;
            (simJacMrf_opt, simCodeIndices) = create(&simJacobianMrf, simCodeIndices, simcode_map.clone())?;
            if (simJacMrf_opt).is_some() {
                simJacMrf = Util::getOption(simJacMrf_opt)?;
            } else {
                (simJacMrf, simCodeIndices) = empty(literal!("OPT_MRF"), simCodeIndices)?;
            }
        }
        if (jacobiansR0).is_empty() {
            (simJacR0, simCodeIndices) = empty(literal!("OPT_R0"), simCodeIndices)?;
        } else {
            simJacobianR0 = Jacobian::combine(&jacobiansR0, literal!("OPT_R0"))?;
            (simJacR0_opt, simCodeIndices) = create(&simJacobianR0, simCodeIndices, simcode_map)?;
            if (simJacR0_opt).is_some() {
                simJacR0 = Util::getOption(simJacR0_opt)?;
            } else {
                (simJacR0, simCodeIndices) = empty(literal!("OPT_R0"), simCodeIndices)?;
            }
        }
        Ok((simJacLfg, simJacMrf, simJacR0, simCodeIndices))
    }

    pub(crate) fn empty(
        mut name: ArcStr,
        mut indices: SimCode::SimCodeIndices,
    ) -> Result<(metamodelica::Ref<SimJacobian>, SimCode::SimCodeIndices)> {
        let mut emptyJac: metamodelica::Ref<SimJacobian> = EMPTY_SIM_JAC().clone();
        let mut indices: SimCode::SimCodeIndices = indices;
        emptyJac = (match &*emptyJac {
            SimJacobian { .. } => {
                assign_field!(
                    emptyJac.name = name,
                    emptyJac.jacobianIndex = indices.jacobianIndex.clone()
                );
                indices.jacobianIndex = indices.jacobianIndex.clone() + 1;
                emptyJac
            }
            _ => {
                Error::addMessage(
                    Error::INTERNAL_ERROR.clone(),
                    list![{
                        let mut __mm_s = String::new();
                        __mm_s.push_str(&*literal!("NSimJacobian.SimJacobian.empty"));
                        __mm_s.push_str(&*literal!(" failed."));
                        ArcStr::from(__mm_s)
                    }],
                )?;
                return Err("fail");
            }
        });
        Ok((emptyJac, indices))
    }

    pub(crate) fn getJacobianBlocks(
        mut jacobian: &metamodelica::Ref<SimJacobian>,
    ) -> Result<metamodelica::List<metamodelica::Ref<SimStrongComponent::Block::Block>>> {
        let mut blcks: metamodelica::List<metamodelica::Ref<SimStrongComponent::Block::Block>>;
        blcks = (match &**jacobian {
            SimJacobian { .. } => listAppend(jacobian.constantEqns.clone(), jacobian.columnEqns.clone()),
            _ => {
                Error::addMessage(
                    Error::INTERNAL_ERROR.clone(),
                    list![{
                        let mut __mm_s = String::new();
                        __mm_s.push_str(&*literal!("NSimJacobian.SimJacobian.getJacobianBlocks"));
                        __mm_s.push_str(&*literal!(" failed."));
                        ArcStr::from(__mm_s)
                    }],
                )?;
                return Err("fail");
            }
        });
        Ok(blcks)
    }

    pub(crate) fn getJacobiansBlocks(
        mut jacobians: &metamodelica::List<metamodelica::Ref<SimJacobian>>,
    ) -> Result<metamodelica::List<metamodelica::Ref<SimStrongComponent::Block::Block>>> {
        let mut blcks: metamodelica::List<metamodelica::Ref<SimStrongComponent::Block::Block>> = metamodelica::nil();
        for mut jacobian in &**jacobians {
            blcks = listAppend(getJacobianBlocks(metamodelica::AsArg::as_arg(&jacobian))?, blcks);
        }
        Ok(blcks)
    }

    pub(crate) fn getJacobianHT(
        mut jacobian: &metamodelica::Ref<SimJacobian>,
    ) -> Result<
        Option<
            metamodelica::Ref<
                UnorderedMap::UnorderedMap<
                    metamodelica::Ref<ComponentRef::NFComponentRef>,
                    metamodelica::Ref<SimVar::SimVar>,
                >,
            >,
        >,
    > {
        let mut jac_map: Option<
            metamodelica::Ref<
                UnorderedMap::UnorderedMap<
                    metamodelica::Ref<ComponentRef::NFComponentRef>,
                    metamodelica::Ref<SimVar::SimVar>,
                >,
            >,
        >;
        jac_map = (match &**jacobian {
            SimJacobian { .. } => jacobian.jac_map.clone(),
            _ => {
                Error::addMessage(
                    Error::INTERNAL_ERROR.clone(),
                    list![{
                        let mut __mm_s = String::new();
                        __mm_s.push_str(&*literal!("NSimJacobian.SimJacobian.getJacobianHT"));
                        __mm_s.push_str(&*literal!(" failed."));
                        ArcStr::from(__mm_s)
                    }],
                )?;
                return Err("fail");
            }
        });
        Ok(jac_map)
    }

    pub(crate) fn convert(
        mut simJac: &metamodelica::Ref<SimJacobian>,
    ) -> Result<metamodelica::Ref<OldSimCode::JacobianMatrix>> {
        let mut oldJac: metamodelica::Ref<OldSimCode::JacobianMatrix>;
        let mut oldJacCol: metamodelica::Ref<OldSimCode::JacobianColumn>;
        oldJac = (match &**simJac {
            SimJacobian { .. } => {
                let mut memo: metamodelica::Ref<
                    UnorderedMap::UnorderedMap<
                        metamodelica::Ref<ComponentRef::NFComponentRef>,
                        (metamodelica::Ref<SimVar::SimVar>, metamodelica::Ref<SimCodeVar::SimVar>),
                    >,
                >;
                memo = SimVar::newConvertMemo(((simJac.seedVars).len() as i32) + ((simJac.columnVars).len() as i32));
                oldJacCol = metamodelica::Ref::new(OldSimCode::JacobianColumn {
                    columnEqns: ({
                        let mut __acc: metamodelica::List<metamodelica::Ref<OldSimCode::SimEqSystem>> =
                            metamodelica::nil();
                        for mut blck in (simJac.columnEqns.clone()).into_iter().cloned() {
                            let __x = SimStrongComponent::Block::convert(&(blck.clone()))?;
                            __acc = cons(__x, __acc);
                        }
                        __acc.reverse()
                    }),
                    columnVars: SimVar::convertListMemo(simJac.columnVars.clone(), memo.clone())?,
                    numberOfResultVars: simJac.numberOfResultVars.clone(),
                    constantEqns: ({
                        let mut __acc: metamodelica::List<metamodelica::Ref<OldSimCode::SimEqSystem>> =
                            metamodelica::nil();
                        for mut blck in (simJac.constantEqns.clone()).into_iter().cloned() {
                            let __x = SimStrongComponent::Block::convert(&(blck.clone()))?;
                            __acc = cons(__x, __acc);
                        }
                        __acc.reverse()
                    }),
                });
                oldJac = metamodelica::Ref::new(OldSimCode::JacobianMatrix {
                    columns: list![oldJacCol],
                    seedVars: SimVar::convertListMemo(simJac.seedVars.clone(), memo.clone())?,
                    matrixName: simJac.name.clone(),
                    sparsityMatrix: Sparsity::convert(&simJac.sparsityMatrix)?,
                    sparsity: metamodelica::nil(),
                    sparsityT: metamodelica::nil(),
                    nonlinear: metamodelica::nil(),
                    nonlinearT: metamodelica::nil(),
                    coloredCols: metamodelica::nil(),
                    coloredRows: metamodelica::nil(),
                    maxColorCols: 0,
                    jacobianIndex: simJac.jacobianIndex.clone(),
                    partitionIndex: simJac.partitionIndex.clone(),
                    generic_loop_calls: ({
                        let mut __acc: metamodelica::List<OldSimCode::SimGenericCall> = metamodelica::nil();
                        for mut gc in (simJac.generic_loop_calls.clone()).into_iter().cloned() {
                            let __x = SimGenericCall::convert(&(gc.clone()))?;
                            __acc = cons(__x, __acc);
                        }
                        __acc.reverse()
                    }),
                    crefsHT: Util::applyOption(
                        simJac.jac_map.clone(),
                        &({
                            let __pe_b1 = memo;
                            move |__pe_a0| SimCodeUtil::convertSimCodeMap(__pe_a0, __pe_b1.clone())
                        }),
                    )?,
                    isAdjoint: simJac.isAdjoint.clone(),
                    isBidirectional: simJac.isBidirectional.clone(),
                    adjointJacobianIndex: simJac.adjointJacobianIndex.clone(),
                    adjointMatrixName: simJac.adjointMatrixName.clone(),
                });
                oldJac
            }
            _ => {
                Error::addMessage(
                    Error::INTERNAL_ERROR.clone(),
                    list![{
                        let mut __mm_s = String::new();
                        __mm_s.push_str(&*literal!("NSimJacobian.SimJacobian.convert"));
                        __mm_s.push_str(&*literal!(" failed."));
                        ArcStr::from(__mm_s)
                    }],
                )?;
                return Err("fail");
            }
        });
        Ok(oldJac)
    }
}

thread_local! { static __EMPTY_SIM_JAC_TLS: metamodelica::Ref<SimJacobian::SimJacobian> = metamodelica::Ref::new(SimJacobian::SimJacobian { name: literal!(""), jacobianIndex: 0, partitionIndex: 0, numberOfResultVars: 0, columnEqns: metamodelica::nil(), constantEqns: metamodelica::nil(), columnVars: metamodelica::nil(), seedVars: metamodelica::nil(), sparsityMatrix: crate::NSimJacobian::Sparsity::interned_EMPTY(), generic_loop_calls: metamodelica::nil(), jac_map: None, isAdjoint: false, isBidirectional: false, adjointJacobianIndex: -1, adjointMatrixName: literal!("") }); }
pub(crate) fn EMPTY_SIM_JAC() -> metamodelica::Ref<SimJacobian::SimJacobian> {
    __EMPTY_SIM_JAC_TLS.with(|__t| __t.clone())
}

fn collectNodeSubDimPairsOuterFirst(
    mut subs: &metamodelica::List<metamodelica::Ref<Subscript::NFSubscript>>,
    mut dims: &metamodelica::List<metamodelica::Ref<Dimension::NFDimension>>,
) -> Result<metamodelica::List<(i32, i32)>> {
    let mut pairs: metamodelica::List<(i32, i32)>;
    pairs = (::match_deref::match_deref! { match (subs, dims) {
        (Deref @ metamodelica::ListNode::Nil, _) => {
            metamodelica::nil()
        },
        (Deref @ metamodelica::ListNode::Cons { head: Deref @ Subscript::INDEX { index: Deref @ Expression::INTEGER { value: v3 } }, tail: Deref @ metamodelica::ListNode::Nil }, Deref @ metamodelica::ListNode::Nil) => {
            list![(v3.clone(), 1)]
        },
        (_, Deref @ metamodelica::ListNode::Nil) => {
            metamodelica::nil()
        },
        (Deref @ metamodelica::ListNode::Cons { head: s, tail: rest_subs }, Deref @ metamodelica::ListNode::Cons { head: d, tail: rest_dims }) => {
            let mut rest_pairs: metamodelica::List<(i32, i32)>;
            rest_pairs = collectNodeSubDimPairsOuterFirst(rest_subs, rest_dims)?;
            (::match_deref::match_deref! { match &(s.clone()) {
        Deref @ Subscript::INDEX { index: Deref @ Expression::INTEGER { value: v2 } } => {
            metamodelica::cons((v2.clone(), Dimension::size(metamodelica::AsArg::as_arg(&d), false)?), rest_pairs)
        },
        _ => {
            rest_pairs
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } })
        },
        _ => {
            metamodelica::nil()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(pairs)
}

fn crefSubDimPairsLeafToRoot(
    mut cref: &metamodelica::Ref<ComponentRef::NFComponentRef>,
) -> Result<metamodelica::List<(i32, i32)>> {
    let mut pairs: metamodelica::List<(i32, i32)>;
    pairs = (match &**cref {
        ComponentRef::CREF {
            subscripts: subs,
            restCref: rest,
            ..
        } => {
            let mut node_ty: metamodelica::Ref<Type::NFType>;
            let mut rest_pairs: metamodelica::List<(i32, i32)>;
            let mut node_pairs: metamodelica::List<(i32, i32)>;
            let mut dims: metamodelica::List<metamodelica::Ref<Dimension::NFDimension>>;
            let mut own: i32;
            rest_pairs = crefSubDimPairsLeafToRoot(rest)?;
            node_ty = InstNode::getType(ComponentRef::node(cref)?)?;
            dims = Type::arrayDims(node_ty);
            own = ((dims).len() as i32) - ((ComponentRef::subscriptsAllFlat(rest)?).len() as i32);
            if own >= ((subs).len() as i32) && own < ((dims).len() as i32) {
                dims = List::lastN(dims, own)?;
            }
            node_pairs = collectNodeSubDimPairsOuterFirst(subs, &dims)?.reverse();
            listAppend(node_pairs, rest_pairs)
        }
        _ => metamodelica::nil(),
    });
    Ok(pairs)
}

fn sortByStateIndex(
    mut vars: metamodelica::List<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>>,
    mut simcode_map: metamodelica::Ref<
        UnorderedMap::UnorderedMap<metamodelica::Ref<ComponentRef::NFComponentRef>, metamodelica::Ref<SimVar::SimVar>>,
    >,
) -> Result<metamodelica::List<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>>> {
    let mut sorted: metamodelica::List<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>> = vars.clone();
    let mut keyed: metamodelica::List<(i32, Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>)> =
        metamodelica::nil();
    let mut osv: Option<metamodelica::Ref<SimVar::SimVar>>;
    let mut sv: metamodelica::Ref<SimVar::SimVar>;
    for mut v in &*vars {
        osv = UnorderedMap::get(stripRootNode(BVariable::getVarName(v.clone())), simcode_map.clone())?;
        if (osv).is_none() {
            return Ok(sorted);
        }
        let __pa0 = ::match_deref::match_deref! { match &(osv) {
            Some(__pa0) => __pa0.clone(),
            _ => return Err("pattern mismatch"),
        } };
        sv = metamodelica::Own::own(__pa0);
        keyed = metamodelica::cons((sv.index.clone(), v.clone()), keyed);
    }
    keyed = List::sort(
        keyed,
        (std::sync::Arc::new(fnptr!(
            indexGreater,
            (i32, Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>),
            (i32, Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>)
        ))
            as std::sync::Arc<
                dyn ::std::ops::Fn(
                        (i32, Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>),
                        (i32, Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>),
                    ) -> Result<bool>
                    + 'static,
            >),
    )?;
    sorted = ({
        let mut __acc: metamodelica::List<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>> =
            metamodelica::nil();
        for mut t in (keyed).into_iter().cloned() {
            let __x = Util::tuple22(t.clone());
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    Ok(sorted)
}

fn indexGreater(
    mut t1: (i32, Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>),
    mut t2: (i32, Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>),
) -> bool {
    let mut b: bool = Util::tuple21(t1.clone()) > Util::tuple21(t2.clone());
    b
}

fn stripRootNode(
    mut cref: metamodelica::Ref<ComponentRef::NFComponentRef>,
) -> metamodelica::Ref<ComponentRef::NFComponentRef> {
    let mut stripped: metamodelica::Ref<ComponentRef::NFComponentRef>;
    stripped = (::match_deref::match_deref! { match &(cref.clone()) {
        Deref @ ComponentRef::CREF { restCref: Deref @ ComponentRef::EMPTY, .. } => openmodelica_nf_frontend::NFComponentRef::interned_EMPTY(),
        Deref @ ComponentRef::CREF { restCref: __cref_restCref, .. } => {
            assign_variant_field!(cref => ComponentRef::NFComponentRef::CREF; restCref = stripRootNode(__cref_restCref.clone()));
            cref
        },
        _ => cref,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    stripped
}

fn crefFlatOffset(mut cref: &metamodelica::Ref<ComponentRef::NFComponentRef>) -> Result<i32> {
    let mut offset: i32 = 0;
    let mut pairs: metamodelica::List<(i32, i32)>;
    let mut inner_prod: i32 = 1;
    let mut sub_val: i32;
    let mut dim_sz: i32;
    pairs = crefSubDimPairsLeafToRoot(cref)?;
    for mut pair in &*pairs {
        (sub_val, dim_sz) = pair.clone();
        offset = offset + (sub_val - 1) * inner_prod;
        inner_prod = inner_prod * dim_sz;
    }
    Ok(offset)
}
