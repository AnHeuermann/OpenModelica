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

use crate::NBDifferentiate as Differentiate;
use crate::NBDifferentiate::DifferentiationArguments;
use crate::NBDifferentiate::DifferentiationType;
use crate::NBEquation;
use crate::NBEquation::Equation;
use crate::NBEquation::EquationAttributes;
use crate::NBEquation::EquationKind;
use crate::NBEquation::EquationPointer;
use crate::NBEquation::EquationPointers;
use crate::NBEquation::Iterator;
use crate::NBReplacements as Replacements;
use crate::NBSolve as Solve;
use crate::NBSolve::Status;
use crate::NBVariable as BVariable;
use crate::NBVariable::VarData;
use crate::NBVariable::VariablePointer;
use crate::NBVariable::VariablePointers;
use openmodelica_ast::Absyn;
use openmodelica_frontend_base::ExpressionDump;
use openmodelica_frontend_base::ExpressionSimplify;
use openmodelica_frontend_dump::AbsynUtil;
use openmodelica_frontend_dump::ExpressionBasics;
use openmodelica_frontend_types::DAE;
use openmodelica_nf_frontend::NFComponentRef as ComponentRef;
use openmodelica_nf_frontend::NFExpression as Expression;
use openmodelica_nf_frontend::NFFlatten::FunctionTreeImpl;
use openmodelica_nf_frontend::NFFunction::Function;
use openmodelica_nf_frontend::NFOperator as Operator;
use openmodelica_nf_frontend::NFSimplifyExp as SimplifyExp;
use openmodelica_nf_frontend::NFType as Type;
use openmodelica_util::Error;
use openmodelica_util::Flags;
use openmodelica_util::UnorderedMap;
use openmodelica_util::UnorderedSet;
use openmodelica_util::Util;
use openmodelica_util_datatypes_basic::Array;
use openmodelica_util_datatypes_basic::List;
use openmodelica_util_datatypes_basic::Pointer;

// NF imports
// Backend imports
// Util imports
pub(crate) fn main(
    mut eqns: &metamodelica::List<Pointer::Pointer<metamodelica::Ref<Equation::Equation>>>,
    mut vars: metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>>,
    mut index: Pointer::Pointer<i32>,
) -> Result<metamodelica::List<Pointer::Pointer<metamodelica::Ref<Equation::Equation>>>> {
    let mut resolved_eqns: metamodelica::List<Pointer::Pointer<metamodelica::Ref<Equation::Equation>>> =
        metamodelica::nil();
    let mut indices: metamodelica::Array<metamodelica::List<i32>>;
    let mut values: metamodelica::Array<metamodelica::List<i32>>;
    let mut num_crefs: i32;
    let mut num_eqns: i32;
    let mut num_nonzero_val: i32;
    let mut op_modes: metamodelica::Array<i32>;
    let mut op_val1: metamodelica::Array<i32>;
    let mut op_val2: metamodelica::Array<i32>;
    let mut op_val3: metamodelica::Array<i32>;
    let mut op_val4: metamodelica::Array<i32>;
    let mut num_op: i32;
    let mut count_zero_row: i32;
    let mut lhs_map: metamodelica::Ref<
        UnorderedMap::UnorderedMap<
            Pointer::Pointer<metamodelica::Ref<Equation::Equation>>,
            metamodelica::Ref<Expression::NFExpression>,
        >,
    >;
    let mut lhs_array: metamodelica::Array<metamodelica::Ref<Expression::NFExpression>>;
    let mut singular: bool;
    (indices, values, num_crefs, num_eqns, num_nonzero_val, lhs_map) = buildSparseRepresentation(eqns, vars.clone())?;
    setMatrix(num_crefs, num_eqns, num_nonzero_val, indices.clone(), values.clone());
    (indices, values) = performBareissElimination(indices.clone(), values.clone())?;
    (num_op, op_modes, op_val1, op_val2, op_val3, op_val4, lhs_array) = applyRecordedOperations(lhs_map)?;
    (singular, count_zero_row) = checkSingularity(indices.clone(), num_eqns)?;
    if singular {
        tracebackZeroRows(
            eqns,
            num_eqns,
            count_zero_row,
            num_op,
            op_modes.clone(),
            op_val1.clone(),
            op_val2.clone(),
            op_val3.clone(),
            op_val4.clone(),
        )?;
    } else {
        resolved_eqns = createEquations(
            &vars,
            index,
            indices.clone(),
            values.clone(),
            num_eqns,
            lhs_array.clone(),
        )?;
    }
    freeMatrix();
    Ok(resolved_eqns)
}

pub(crate) fn buildSparseRepresentation(
    mut eqns: &metamodelica::List<Pointer::Pointer<metamodelica::Ref<Equation::Equation>>>,
    mut vars: metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>>,
) -> Result<(
    metamodelica::Array<metamodelica::List<i32>>,
    metamodelica::Array<metamodelica::List<i32>>,
    i32,
    i32,
    i32,
    metamodelica::Ref<
        UnorderedMap::UnorderedMap<
            Pointer::Pointer<metamodelica::Ref<Equation::Equation>>,
            metamodelica::Ref<Expression::NFExpression>,
        >,
    >,
)> {
    let mut indices: metamodelica::Array<metamodelica::List<i32>>;
    let mut values: metamodelica::Array<metamodelica::List<i32>>;
    let mut num_crefs: i32;
    let mut num_eqns: i32;
    let mut num_nonzero_val: i32;
    let mut lhs_map: metamodelica::Ref<
        UnorderedMap::UnorderedMap<
            Pointer::Pointer<metamodelica::Ref<Equation::Equation>>,
            metamodelica::Ref<Expression::NFExpression>,
        >,
    > = UnorderedMap::new(
        (std::sync::Arc::new(Equation::hash)
            as std::sync::Arc<
                dyn ::std::ops::Fn(Pointer::Pointer<metamodelica::Ref<Equation::Equation>>) -> Result<i32> + 'static,
            >),
        (std::sync::Arc::new(Equation::isEqualPtr)
            as std::sync::Arc<
                dyn ::std::ops::Fn(
                        Pointer::Pointer<metamodelica::Ref<Equation::Equation>>,
                        Pointer::Pointer<metamodelica::Ref<Equation::Equation>>,
                    ) -> Result<bool>
                    + 'static,
            >),
        1,
    );
    let mut b: bool = true;
    let mut cref_lst: metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>>;
    let mut res: metamodelica::Ref<Expression::NFExpression>;
    let mut diff_res: metamodelica::Ref<Expression::NFExpression>;
    let mut expr: metamodelica::Ref<Expression::NFExpression>;
    let mut args: metamodelica::Ref<DifferentiationArguments::DifferentiationArguments>;
    let mut diff_res_int: i32;
    let mut eqn_index: i32;
    let mut var_index: i32;
    let mut var_index1: i32;
    let mut var_index2: i32;
    let mut id: metamodelica::Ref<Tuple_Id::Tuple_Id>;
    let mut diffs: metamodelica::Ref<UnorderedMap::UnorderedMap<metamodelica::Ref<Tuple_Id::Tuple_Id>, i32>> =
        UnorderedMap::new(
            (std::sync::Arc::new(move |__a0: metamodelica::Ref<Tuple_Id::Tuple_Id>| Tuple_Id::hash(&__a0))
                as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<Tuple_Id::Tuple_Id>) -> Result<i32> + 'static>),
            (std::sync::Arc::new(
                move |__a0: metamodelica::Ref<Tuple_Id::Tuple_Id>, __a1: metamodelica::Ref<Tuple_Id::Tuple_Id>| {
                    Tuple_Id::isEqual(&__a0, &__a1)
                },
            )
                as std::sync::Arc<
                    dyn ::std::ops::Fn(
                            metamodelica::Ref<Tuple_Id::Tuple_Id>,
                            metamodelica::Ref<Tuple_Id::Tuple_Id>,
                        ) -> Result<bool>
                        + 'static,
                >),
            1,
        );
    let mut int_eqns: metamodelica::Ref<
        UnorderedSet::UnorderedSet<Pointer::Pointer<metamodelica::Ref<Equation::Equation>>>,
    > = UnorderedSet::new(
        (std::sync::Arc::new(Equation::hash)
            as std::sync::Arc<
                dyn ::std::ops::Fn(Pointer::Pointer<metamodelica::Ref<Equation::Equation>>) -> Result<i32> + 'static,
            >),
        (std::sync::Arc::new(Equation::isEqualPtr)
            as std::sync::Arc<
                dyn ::std::ops::Fn(
                        Pointer::Pointer<metamodelica::Ref<Equation::Equation>>,
                        Pointer::Pointer<metamodelica::Ref<Equation::Equation>>,
                    ) -> Result<bool>
                    + 'static,
            >),
        13,
    );
    let mut int_crefs: metamodelica::Ref<UnorderedSet::UnorderedSet<metamodelica::Ref<ComponentRef::NFComponentRef>>> =
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
    let mut rows: metamodelica::Ref<
        UnorderedMap::UnorderedMap<
            Pointer::Pointer<metamodelica::Ref<Equation::Equation>>,
            metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>>,
        >,
    > = UnorderedMap::new(
        (std::sync::Arc::new(Equation::hash)
            as std::sync::Arc<
                dyn ::std::ops::Fn(Pointer::Pointer<metamodelica::Ref<Equation::Equation>>) -> Result<i32> + 'static,
            >),
        (std::sync::Arc::new(Equation::isEqualPtr)
            as std::sync::Arc<
                dyn ::std::ops::Fn(
                        Pointer::Pointer<metamodelica::Ref<Equation::Equation>>,
                        Pointer::Pointer<metamodelica::Ref<Equation::Equation>>,
                    ) -> Result<bool>
                    + 'static,
            >),
        1,
    );
    let mut replacements: metamodelica::Ref<
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
    let mut lst_enum: metamodelica::List<i32>;
    let mut lst_eqns: metamodelica::List<Pointer::Pointer<metamodelica::Ref<Equation::Equation>>>;
    let mut crefs_rows: metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>>;
    let mut enum_eqns: metamodelica::Ref<
        UnorderedMap::UnorderedMap<Pointer::Pointer<metamodelica::Ref<Equation::Equation>>, i32>,
    >;
    let mut enum_crefs: metamodelica::Ref<
        UnorderedMap::UnorderedMap<metamodelica::Ref<ComponentRef::NFComponentRef>, i32>,
    >;
    for mut eq_ptr in &**eqns {
        cref_lst = Equation::collectCrefs(
            Pointer::access(eq_ptr.clone()),
            (std::sync::Arc::new({
                let __pe_b2 = UnorderedMap::fromLists(
                    &(vars.clone()),
                    vars.clone(),
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
                )?;
                move |__pe_a0, __pe_a1| Equation::collectFromMap(__pe_a0, __pe_a1, __pe_b2.clone())
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
        res = Equation::getResidualExp(&(Pointer::access(eq_ptr.clone())), true)?;
        args = Differentiate::DifferentiationArguments::default(
            Differentiate::DifferentiationType::SIMPLE.clone(),
            UnorderedMap::new(
                (std::sync::Arc::new(move |__a0: metamodelica::Ref<Absyn::Path>| -> metamodelica::Result<_> {
                    ::std::result::Result::Ok(AbsynUtil::pathHash(&__a0))
                })
                    as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<Absyn::Path>) -> Result<i32> + 'static>),
                (std::sync::Arc::new(
                    move |__a0: metamodelica::Ref<Absyn::Path>,
                          __a1: metamodelica::Ref<Absyn::Path>|
                          -> metamodelica::Result<_> {
                        ::std::result::Result::Ok(AbsynUtil::pathEqual(&__a0, &__a1))
                    },
                )
                    as std::sync::Arc<
                        dyn ::std::ops::Fn(
                                metamodelica::Ref<Absyn::Path>,
                                metamodelica::Ref<Absyn::Path>,
                            ) -> Result<bool>
                            + 'static,
                    >),
                1,
            ),
        );
        for mut cr in &*cref_lst {
            assign_field!(args.diffCref = cr.clone());
            diff_res = SimplifyExp::simplify(
                (Differentiate::differentiateExpression(res.clone(), args.clone())?).0,
                false,
            )?;
            if Type::isReal(&(Expression::typeOf(diff_res.clone())))? {
                diff_res_int = ((Expression::realValue(&diff_res)?).0.floor() as i32);
                id = metamodelica::Ref::new(Tuple_Id::Tuple_Id {
                    eq_ptr: eq_ptr.clone(),
                    cref: cr.clone(),
                });
            } else if Type::isInteger(&(Expression::typeOf(diff_res.clone())))? {
                diff_res_int = Expression::integerValue(diff_res)?;
                id = metamodelica::Ref::new(Tuple_Id::Tuple_Id {
                    eq_ptr: eq_ptr.clone(),
                    cref: cr.clone(),
                });
                UnorderedMap::add(id, diff_res_int, diffs.clone())?;
            } else {
                b = false;
                break;
            }
        }
        if b {
            UnorderedSet::add(eq_ptr.clone(), int_eqns.clone())?;
            for mut cr in &*cref_lst {
                UnorderedSet::add(cr.clone(), int_crefs.clone())?;
                UnorderedMap::add(
                    cr.clone(),
                    Expression::makeZero(&(Equation::getType(&(Pointer::access(eq_ptr.clone())), false)?))?,
                    replacements.clone(),
                )?;
            }
            UnorderedMap::add(eq_ptr.clone(), cref_lst, rows.clone())?;
            expr = SimplifyExp::simplify(
                Expression::map(
                    res,
                    (std::sync::Arc::new({
                        let __pe_b1 = replacements.clone();
                        move |__pe_a0| Replacements::applySimpleExp(__pe_a0, __pe_b1.clone())
                    })
                        as std::sync::Arc<
                            dyn ::std::ops::Fn(
                                    metamodelica::Ref<Expression::NFExpression>,
                                )
                                    -> Result<metamodelica::Ref<Expression::NFExpression>>
                                + 'static,
                        >),
                )?,
                false,
            )?;
            UnorderedMap::add(eq_ptr.clone(), Expression::negate(expr), lhs_map.clone())?;
        }
    }
    lst_enum = List::intRange(UnorderedSet::size(int_eqns.clone()));
    enum_eqns = UnorderedMap::fromLists(
        eqns,
        lst_enum,
        (std::sync::Arc::new(Equation::hash)
            as std::sync::Arc<
                dyn ::std::ops::Fn(Pointer::Pointer<metamodelica::Ref<Equation::Equation>>) -> Result<i32> + 'static,
            >),
        (std::sync::Arc::new(Equation::isEqualPtr)
            as std::sync::Arc<
                dyn ::std::ops::Fn(
                        Pointer::Pointer<metamodelica::Ref<Equation::Equation>>,
                        Pointer::Pointer<metamodelica::Ref<Equation::Equation>>,
                    ) -> Result<bool>
                    + 'static,
            >),
    )?;
    lst_enum = List::intRange(UnorderedSet::size(int_crefs.clone()));
    enum_crefs = UnorderedMap::fromLists(
        &vars,
        lst_enum,
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
    if Flags::isSet(Flags::DUMP_ASSC.clone())? {
        metamodelica::print({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("Variable-to-column mapping:\n"));
            __mm_s.push_str(&*UnorderedMap::toString(
                enum_crefs.clone(),
                &move |__a0: metamodelica::Ref<ComponentRef::NFComponentRef>| ComponentRef::toString(&__a0),
                &fnptr!(intString, i32),
                literal!("\n"),
                &(literal!(", ")),
            )?);
            __mm_s.push_str(&*literal!("\n"));
            ArcStr::from(__mm_s)
        });
    }
    indices = arrayCreate(UnorderedSet::size(int_eqns.clone()), metamodelica::nil());
    values = arrayCreate(UnorderedSet::size(int_eqns.clone()), metamodelica::nil());
    lst_eqns = UnorderedSet::toList(int_eqns.clone());
    for mut eq_ptr in &*lst_eqns {
        crefs_rows = UnorderedMap::getSafe(
            eq_ptr.clone(),
            rows.clone(),
            metamodelica::sourceInfo!("NBackEnd/Util/NBASSC.mo"),
        )?;
        var_index1 = UnorderedMap::getSafe(
            (crefs_rows).get(1)?,
            enum_crefs.clone(),
            metamodelica::sourceInfo!("NBackEnd/Util/NBASSC.mo"),
        )?;
        var_index2 = UnorderedMap::getSafe(
            (crefs_rows).get(2)?,
            enum_crefs.clone(),
            metamodelica::sourceInfo!("NBackEnd/Util/NBASSC.mo"),
        )?;
        if var_index1 > var_index2 {
            crefs_rows = crefs_rows.reverse();
        }
        for mut cr in &*crefs_rows.reverse() {
            eqn_index = UnorderedMap::getSafe(
                eq_ptr.clone(),
                enum_eqns.clone(),
                metamodelica::sourceInfo!("NBackEnd/Util/NBASSC.mo"),
            )?;
            var_index = UnorderedMap::getSafe(
                cr.clone(),
                enum_crefs.clone(),
                metamodelica::sourceInfo!("NBackEnd/Util/NBASSC.mo"),
            )?;
            {
                let __cell0 = metamodelica::cons(
                    var_index,
                    ({
                        let __elt = (*metamodelica::index_checked(&indices.borrow(), eqn_index)?).clone();
                        __elt
                    }),
                );
                let __idx0 = eqn_index;
                *metamodelica::index_mut_checked(&mut indices.clone().borrow_mut(), __idx0)? = __cell0;
            }
            {
                let __cell1 = metamodelica::cons(
                    UnorderedMap::getSafe(
                        metamodelica::Ref::new(Tuple_Id::Tuple_Id {
                            eq_ptr: eq_ptr.clone(),
                            cref: cr.clone(),
                        }),
                        diffs.clone(),
                        metamodelica::sourceInfo!("NBackEnd/Util/NBASSC.mo"),
                    )?,
                    ({
                        let __elt = (*metamodelica::index_checked(&values.borrow(), eqn_index)?).clone();
                        __elt
                    }),
                );
                let __idx1 = eqn_index;
                *metamodelica::index_mut_checked(&mut values.clone().borrow_mut(), __idx1)? = __cell1;
            }
        }
    }
    num_crefs = UnorderedSet::size(int_crefs);
    num_eqns = UnorderedSet::size(int_eqns);
    num_nonzero_val = UnorderedMap::size(diffs);
    Ok((indices, values, num_crefs, num_eqns, num_nonzero_val, lhs_map))
}

pub(crate) fn performBareissElimination(
    mut indices: metamodelica::Array<metamodelica::List<i32>>,
    mut values: metamodelica::Array<metamodelica::List<i32>>,
) -> Result<(
    metamodelica::Array<metamodelica::List<i32>>,
    metamodelica::Array<metamodelica::List<i32>>,
)> {
    let mut indices: metamodelica::Array<metamodelica::List<i32>> = indices;
    let mut values: metamodelica::Array<metamodelica::List<i32>> = values;
    if Flags::isSet(Flags::DUMP_ASSC.clone())? {
        metamodelica::print(literal!("Sparse matrix before applying the Bareiss algorithm:\n"));
        printMatrix();
    }
    bareiss();
    if Flags::isSet(Flags::DUMP_ASSC.clone())? {
        metamodelica::print(literal!("Sparse matrix after applying the Bareiss algorithm:\n"));
        printMatrix();
        metamodelica::print(literal!("\n"));
    }
    indices = arrayCreate(metamodelica::arrayLength(indices.clone()), metamodelica::nil());
    values = arrayCreate(metamodelica::arrayLength(values.clone()), metamodelica::nil());
    getMatrix(indices.clone(), values.clone());
    if Flags::isSet(Flags::DUMP_ASSC.clone())? {
        metamodelica::print(literal!("List indices:\n"));
        for mut i in 1..=metamodelica::arrayLength(indices.clone()) {
            metamodelica::print({
                let mut __mm_s = String::new();
                __mm_s.push_str(&*List::toString(
                    ({
                        let __elt = (*metamodelica::index_checked(&indices.borrow(), i)?).clone();
                        __elt
                    }),
                    &fnptr!(intString, i32),
                    List::Style::FLAT_CURLY.clone(),
                )?);
                __mm_s.push_str(&*literal!("\n"));
                ArcStr::from(__mm_s)
            });
        }
        metamodelica::print(literal!("\nList values:"));
        for mut i in 1..=metamodelica::arrayLength(values.clone()) {
            metamodelica::print({
                let mut __mm_s = String::new();
                __mm_s.push_str(&*List::toString(
                    ({
                        let __elt = (*metamodelica::index_checked(&values.borrow(), i)?).clone();
                        __elt
                    }),
                    &fnptr!(intString, i32),
                    List::Style::FLAT_CURLY.clone(),
                )?);
                __mm_s.push_str(&*literal!("\n"));
                ArcStr::from(__mm_s)
            });
        }
        metamodelica::print(literal!("\n"));
    }
    Ok((indices, values))
}

pub(crate) fn applyRecordedOperations(
    mut lhs_map: metamodelica::Ref<
        UnorderedMap::UnorderedMap<
            Pointer::Pointer<metamodelica::Ref<Equation::Equation>>,
            metamodelica::Ref<Expression::NFExpression>,
        >,
    >,
) -> Result<(
    i32,
    metamodelica::Array<i32>,
    metamodelica::Array<i32>,
    metamodelica::Array<i32>,
    metamodelica::Array<i32>,
    metamodelica::Array<i32>,
    metamodelica::Array<metamodelica::Ref<Expression::NFExpression>>,
)> {
    let mut num_op: i32;
    let mut op_modes: metamodelica::Array<i32>;
    let mut op_val1: metamodelica::Array<i32>;
    let mut op_val2: metamodelica::Array<i32>;
    let mut op_val3: metamodelica::Array<i32>;
    let mut op_val4: metamodelica::Array<i32>;
    let mut lhs_array: metamodelica::Array<metamodelica::Ref<Expression::NFExpression>>;
    let mut nop: metamodelica::Array<i32>;
    let mut mode: i32;
    nop = arrayCreate(1, -1);
    num_op = getNumberOfOperations(nop.clone());
    if Flags::isSet(Flags::DUMP_ASSC.clone())? {
        metamodelica::print({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("Number of operations: "));
            __mm_s.push_str(&*intString(num_op));
            __mm_s.push_str(&*literal!("\n"));
            ArcStr::from(__mm_s)
        });
    }
    op_modes = arrayCreate(num_op, -1);
    op_val1 = arrayCreate(num_op, -1);
    op_val2 = arrayCreate(num_op, -1);
    op_val3 = arrayCreate(num_op, -1);
    op_val4 = arrayCreate(num_op, -1);
    getOperations(
        op_modes.clone(),
        op_val1.clone(),
        op_val2.clone(),
        op_val3.clone(),
        op_val4.clone(),
    )?;
    if Flags::isSet(Flags::DUMP_ASSC.clone())? {
        metamodelica::print(literal!("All operations:\n"));
        metamodelica::print({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*Array::toString(
                op_modes.clone(),
                &fnptr!(intString, i32),
                literal!(""),
                literal!("["),
                literal!(", "),
                literal!("]"),
                true,
                0,
            )?);
            __mm_s.push_str(&*literal!("\n"));
            ArcStr::from(__mm_s)
        });
        metamodelica::print({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*Array::toString(
                op_val1.clone(),
                &fnptr!(intString, i32),
                literal!(""),
                literal!("["),
                literal!(", "),
                literal!("]"),
                true,
                0,
            )?);
            __mm_s.push_str(&*literal!("\n"));
            ArcStr::from(__mm_s)
        });
        metamodelica::print({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*Array::toString(
                op_val2.clone(),
                &fnptr!(intString, i32),
                literal!(""),
                literal!("["),
                literal!(", "),
                literal!("]"),
                true,
                0,
            )?);
            __mm_s.push_str(&*literal!("\n"));
            ArcStr::from(__mm_s)
        });
        metamodelica::print({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*Array::toString(
                op_val3.clone(),
                &fnptr!(intString, i32),
                literal!(""),
                literal!("["),
                literal!(", "),
                literal!("]"),
                true,
                0,
            )?);
            __mm_s.push_str(&*literal!("\n"));
            ArcStr::from(__mm_s)
        });
        metamodelica::print({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*Array::toString(
                op_val4.clone(),
                &fnptr!(intString, i32),
                literal!(""),
                literal!("["),
                literal!(", "),
                literal!("]"),
                true,
                0,
            )?);
            __mm_s.push_str(&*literal!("\n"));
            ArcStr::from(__mm_s)
        });
    }
    lhs_array = metamodelica::arrayFromVec(UnorderedMap::valueList(lhs_map).into_iter().cloned().collect());
    if Flags::isSet(Flags::DUMP_ASSC.clone())? {
        metamodelica::print({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("\nlhs array: "));
            __mm_s.push_str(&*Array::toString(
                lhs_array.clone(),
                &Expression::toString,
                literal!(""),
                literal!("["),
                literal!(", "),
                literal!("]"),
                true,
                0,
            )?);
            __mm_s.push_str(&*literal!("\n"));
            ArcStr::from(__mm_s)
        });
    }
    for mut i in 1..=num_op {
        mode = ({
            let __elt = (*metamodelica::index_checked(&op_modes.borrow(), i)?).clone();
            __elt
        });
        if Flags::isSet(Flags::DUMP_ASSC.clone())? {
            metamodelica::print({
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("current op_mode: "));
                __mm_s.push_str(&*intString(mode));
                __mm_s.push_str(&*literal!("\n"));
                ArcStr::from(__mm_s)
            });
        }
        let _ = (match mode {
            0 => {
                let mut tmp_val1: metamodelica::Ref<Expression::NFExpression>;
                let mut tmp_val2: metamodelica::Ref<Expression::NFExpression>;
                tmp_val1 = metamodelica::Ref::new(Expression::NFExpression::MULTARY {
                    arguments: list![
                        Expression::makeInteger(
                            ({
                                let __elt = (*metamodelica::index_checked(&op_val2.borrow(), i)?).clone();
                                __elt
                            })
                        ),
                        ({
                            let __elt = (*metamodelica::index_checked(
                                &lhs_array.borrow(),
                                ({
                                    let __elt = (*metamodelica::index_checked(&op_val3.borrow(), i)?).clone();
                                    __elt
                                }) + 1,
                            )?)
                            .clone();
                            __elt
                        })
                    ],
                    inv_arguments: metamodelica::nil(),
                    operator: Operator::makeMul(openmodelica_nf_frontend::NFType::interned_REAL()),
                });
                tmp_val1 = SimplifyExp::simplify(tmp_val1, false)?;
                tmp_val2 = metamodelica::Ref::new(Expression::NFExpression::MULTARY {
                    arguments: list![
                        Expression::makeInteger(
                            ({
                                let __elt = (*metamodelica::index_checked(&op_val4.borrow(), i)?).clone();
                                __elt
                            })
                        ),
                        ({
                            let __elt = (*metamodelica::index_checked(
                                &lhs_array.borrow(),
                                ({
                                    let __elt = (*metamodelica::index_checked(&op_val1.borrow(), i)?).clone();
                                    __elt
                                }) + 1,
                            )?)
                            .clone();
                            __elt
                        })
                    ],
                    inv_arguments: metamodelica::nil(),
                    operator: Operator::makeMul(openmodelica_nf_frontend::NFType::interned_REAL()),
                });
                tmp_val2 = SimplifyExp::simplify(tmp_val2, false)?;
                {
                    let __cell0 = metamodelica::Ref::new(Expression::NFExpression::MULTARY {
                        arguments: list![tmp_val1],
                        inv_arguments: list![tmp_val2],
                        operator: Operator::makeAdd(openmodelica_nf_frontend::NFType::interned_REAL()),
                    });
                    let __idx0 = ({
                        let __elt = (*metamodelica::index_checked(&op_val3.borrow(), i)?).clone();
                        __elt
                    }) + 1;
                    *metamodelica::index_mut_checked(&mut lhs_array.clone().borrow_mut(), __idx0)? = __cell0;
                }
                {
                    let __cell1 = SimplifyExp::simplify(
                        ({
                            let __elt = (*metamodelica::index_checked(
                                &lhs_array.borrow(),
                                ({
                                    let __elt = (*metamodelica::index_checked(&op_val3.borrow(), i)?).clone();
                                    __elt
                                }) + 1,
                            )?)
                            .clone();
                            __elt
                        }),
                        false,
                    )?;
                    let __idx1 = ({
                        let __elt = (*metamodelica::index_checked(&op_val3.borrow(), i)?).clone();
                        __elt
                    }) + 1;
                    *metamodelica::index_mut_checked(&mut lhs_array.clone().borrow_mut(), __idx1)? = __cell1;
                }
                if Flags::isSet(Flags::DUMP_ASSC.clone())? {
                    metamodelica::print({
                        let mut __mm_s = String::new();
                        __mm_s.push_str(&*literal!("case 0, updated lhs_array: "));
                        __mm_s.push_str(&*Array::toString(
                            lhs_array.clone(),
                            &Expression::toString,
                            literal!(""),
                            literal!("["),
                            literal!(", "),
                            literal!("]"),
                            true,
                            0,
                        )?);
                        __mm_s.push_str(&*literal!("\n"));
                        ArcStr::from(__mm_s)
                    });
                }
                lhs_array.clone()
            }
            1 => {
                let mut tmp_val: metamodelica::Ref<Expression::NFExpression>;
                tmp_val = ({
                    let __elt = (*metamodelica::index_checked(
                        &lhs_array.borrow(),
                        ({
                            let __elt = (*metamodelica::index_checked(&op_val1.borrow(), i)?).clone();
                            __elt
                        }) + 1,
                    )?)
                    .clone();
                    __elt
                });
                {
                    let __cell0 = ({
                        let __elt = (*metamodelica::index_checked(
                            &lhs_array.borrow(),
                            ({
                                let __elt = (*metamodelica::index_checked(&op_val2.borrow(), i)?).clone();
                                __elt
                            }) + 1,
                        )?)
                        .clone();
                        __elt
                    });
                    let __idx0 = ({
                        let __elt = (*metamodelica::index_checked(&op_val1.borrow(), i)?).clone();
                        __elt
                    }) + 1;
                    *metamodelica::index_mut_checked(&mut lhs_array.clone().borrow_mut(), __idx0)? = __cell0;
                }
                {
                    let __cell1 = tmp_val;
                    let __idx1 = ({
                        let __elt = (*metamodelica::index_checked(&op_val2.borrow(), i)?).clone();
                        __elt
                    }) + 1;
                    *metamodelica::index_mut_checked(&mut lhs_array.clone().borrow_mut(), __idx1)? = __cell1;
                }
                if Flags::isSet(Flags::DUMP_ASSC.clone())? {
                    metamodelica::print({
                        let mut __mm_s = String::new();
                        __mm_s.push_str(&*literal!("case 1, updated lhs_array: "));
                        __mm_s.push_str(&*Array::toString(
                            lhs_array.clone(),
                            &Expression::toString,
                            literal!(""),
                            literal!("["),
                            literal!(", "),
                            literal!("]"),
                            true,
                            0,
                        )?);
                        __mm_s.push_str(&*literal!("\n"));
                        ArcStr::from(__mm_s)
                    });
                }
                lhs_array.clone()
            }
            2 => {
                let mut gcd: i32;
                gcd = ({
                    let __elt = (*metamodelica::index_checked(&op_val2.borrow(), i)?).clone();
                    __elt
                });
                {
                    let __cell0 = metamodelica::Ref::new(Expression::NFExpression::MULTARY {
                        arguments: list![
                            ({
                                let __elt = (*metamodelica::index_checked(
                                    &lhs_array.borrow(),
                                    ({
                                        let __elt = (*metamodelica::index_checked(&op_val1.borrow(), i)?).clone();
                                        __elt
                                    }) + 1,
                                )?)
                                .clone();
                                __elt
                            })
                        ],
                        inv_arguments: list![Expression::makeInteger(gcd)],
                        operator: Operator::makeMul(openmodelica_nf_frontend::NFType::interned_REAL()),
                    });
                    let __idx0 = ({
                        let __elt = (*metamodelica::index_checked(&op_val1.borrow(), i)?).clone();
                        __elt
                    }) + 1;
                    *metamodelica::index_mut_checked(&mut lhs_array.clone().borrow_mut(), __idx0)? = __cell0;
                }
                {
                    let __cell1 = SimplifyExp::simplify(
                        ({
                            let __elt = (*metamodelica::index_checked(
                                &lhs_array.borrow(),
                                ({
                                    let __elt = (*metamodelica::index_checked(&op_val1.borrow(), i)?).clone();
                                    __elt
                                }) + 1,
                            )?)
                            .clone();
                            __elt
                        }),
                        false,
                    )?;
                    let __idx1 = ({
                        let __elt = (*metamodelica::index_checked(&op_val1.borrow(), i)?).clone();
                        __elt
                    }) + 1;
                    *metamodelica::index_mut_checked(&mut lhs_array.clone().borrow_mut(), __idx1)? = __cell1;
                }
                if Flags::isSet(Flags::DUMP_ASSC.clone())? {
                    metamodelica::print({
                        let mut __mm_s = String::new();
                        __mm_s.push_str(&*literal!("case 2, updated lhs_array: "));
                        __mm_s.push_str(&*Array::toString(
                            lhs_array.clone(),
                            &Expression::toString,
                            literal!(""),
                            literal!("["),
                            literal!(", "),
                            literal!("]"),
                            true,
                            0,
                        )?);
                        __mm_s.push_str(&*literal!("\n"));
                        ArcStr::from(__mm_s)
                    });
                }
                lhs_array.clone()
            }
            _ => lhs_array.clone(),
        });
    }
    if Flags::isSet(Flags::DUMP_ASSC.clone())? {
        metamodelica::print({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("final lhs_array: "));
            __mm_s.push_str(&*Array::toString(
                lhs_array.clone(),
                &Expression::toString,
                literal!(""),
                literal!("["),
                literal!(", "),
                literal!("]"),
                true,
                0,
            )?);
            __mm_s.push_str(&*literal!("\n\n"));
            ArcStr::from(__mm_s)
        });
    }
    Ok((num_op, op_modes, op_val1, op_val2, op_val3, op_val4, lhs_array))
}

pub(crate) fn checkSingularity(
    mut indices: metamodelica::Array<metamodelica::List<i32>>,
    mut num_eqns: i32,
) -> Result<(bool, i32)> {
    let __ab_indices = indices.borrow();
    let mut singular: bool = false;
    let mut count_zero_row: i32 = 0;
    for mut i in 1..=num_eqns {
        if ((*metamodelica::index_checked(&__ab_indices, i)?).len() as i32) == 0 {
            count_zero_row = count_zero_row + 1;
        }
    }
    if count_zero_row > 0 {
        singular = true;
    }
    Ok((singular, count_zero_row))
}

pub(crate) fn tracebackZeroRows(
    mut eqns: &metamodelica::List<Pointer::Pointer<metamodelica::Ref<Equation::Equation>>>,
    mut num_eqns: i32,
    mut count_zero_row: i32,
    mut num_op: i32,
    mut op_modes: metamodelica::Array<i32>,
    mut op_val1: metamodelica::Array<i32>,
    mut op_val2: metamodelica::Array<i32>,
    mut op_val3: metamodelica::Array<i32>,
    mut op_val4: metamodelica::Array<i32>,
) -> Result<()> {
    let mut current_zero_row: i32;
    let mut exp_dae: metamodelica::Ref<DAE::Exp>;
    let mut eq_str: ArcStr = literal!("");
    let mut str_all: ArcStr = literal!("");
    if Flags::isSet(Flags::DUMP_ASSC.clone())? {
        metamodelica::print({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("Number of zero rows: "));
            __mm_s.push_str(&*intString(count_zero_row));
            __mm_s.push_str(&*literal!("\n"));
            ArcStr::from(__mm_s)
        });
    }
    for mut zero_row in 1..=count_zero_row {
        current_zero_row = num_eqns - zero_row;
        exp_dae = traceEquation(
            current_zero_row,
            num_op,
            op_modes.clone(),
            op_val1.clone(),
            op_val2.clone(),
            op_val3.clone(),
            op_val4.clone(),
        )?;
        if Flags::isSet(Flags::DUMP_ASSC.clone())? {
            metamodelica::print({
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("Reconstructed zero row: "));
                __mm_s.push_str(&*ExpressionBasics::printExpStr(exp_dae.clone())?);
                __mm_s.push_str(&*literal!("\n"));
                ArcStr::from(__mm_s)
            });
        }
        for mut eq in 1..=num_eqns {
            eq_str = {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*eq_str);
                __mm_s.push_str(&*literal!("("));
                __mm_s.push_str(&*intString(eq - 1));
                __mm_s.push_str(&*literal!("): "));
                __mm_s.push_str(&*Expression::toString(Util::getOption(Equation::getLHS(
                    Pointer::access((eqns).get(eq)?),
                )?)?)?);
                __mm_s.push_str(&*literal!(" = "));
                __mm_s.push_str(&*Expression::toString(Util::getOption(Equation::getRHS(
                    Pointer::access((eqns).get(eq)?),
                )?)?)?);
                __mm_s.push_str(&*literal!("\n"));
                ArcStr::from(__mm_s)
            };
        }
        str_all = {
            let mut __mm_s = String::new();
            __mm_s.push_str(&*str_all);
            __mm_s.push_str(&*literal!("The zero row in ("));
            __mm_s.push_str(&*intString(current_zero_row));
            __mm_s.push_str(&*literal!(") was produced by the following calculation: "));
            __mm_s.push_str(&*ExpressionBasics::printExpStr(exp_dae)?);
            __mm_s.push_str(&*literal!(" with \n"));
            __mm_s.push_str(&*eq_str);
            __mm_s.push_str(&*literal!("\n"));
            ArcStr::from(__mm_s)
        };
    }
    if Flags::isSet(Flags::DUMP_ASSC.clone())? {
        Error::addMessage(
            Error::INTERNAL_ERROR.clone(),
            list![{
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("NBASSC.tracebackZeroRows"));
                __mm_s.push_str(&*literal!(" failed because sparse matrix is singular.\n"));
                __mm_s.push_str(&*str_all);
                ArcStr::from(__mm_s)
            }],
        )?;
        return Err("fail");
    } else {
        Error::addMessage(
            Error::INTERNAL_ERROR.clone(),
            list![{
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("NBASSC.tracebackZeroRows"));
                __mm_s.push_str(&*literal!(
                    " failed because sparse matrix is singular, for more information please use -d=dumpASSC.\n"
                ));
                ArcStr::from(__mm_s)
            }],
        )?;
        return Err("fail");
    }
    Ok(())
}

pub(crate) fn traceEquation(
    mut current_row: i32,
    mut last_op: i32,
    mut op_modes: metamodelica::Array<i32>,
    mut op_val1: metamodelica::Array<i32>,
    mut op_val2: metamodelica::Array<i32>,
    mut op_val3: metamodelica::Array<i32>,
    mut op_val4: metamodelica::Array<i32>,
) -> Result<metamodelica::Ref<DAE::Exp>> {
    let mut exp_dae: metamodelica::Ref<DAE::Exp> = <metamodelica::Ref<DAE::Exp> as ::std::default::Default>::default();
    let mut expressions: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
    let mut found: bool;
    let mut pre_op: i32;
    let mut pre_mode: i32;
    let mut row1: i32;
    let mut row2: i32;
    let mut factor1: i32;
    let mut factor2: i32;
    let mut gcd: i32;
    let mut pivot_exp: metamodelica::Ref<DAE::Exp>;
    let mut update_exp: metamodelica::Ref<DAE::Exp>;
    (found, pre_op, pre_mode, row1, row2, factor1, factor2, gcd) = findLastOperation(
        current_row,
        last_op,
        op_modes.clone(),
        op_val1.clone(),
        op_val2.clone(),
        op_val3.clone(),
        op_val4.clone(),
    )?;
    if !(found) {
        exp_dae = metamodelica::Ref::new(DAE::Exp::CREF {
            componentRef: metamodelica::Ref::new(DAE::ComponentRef::CREF_IDENT {
                ident: {
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!("("));
                    __mm_s.push_str(&*intString(current_row));
                    __mm_s.push_str(&*literal!(")"));
                    ArcStr::from(__mm_s)
                },
                identType: DAE::T_REAL_DEFAULT().clone(),
                subscriptLst: metamodelica::nil(),
            }),
            ty: DAE::T_REAL_DEFAULT().clone(),
        });
        return Ok(exp_dae);
    }
    expressions = metamodelica::nil();
    let _ = (match pre_mode {
        0 => {
            pivot_exp = traceEquation(
                row1,
                pre_op,
                op_modes.clone(),
                op_val1.clone(),
                op_val2.clone(),
                op_val3.clone(),
                op_val4.clone(),
            )?;
            update_exp = traceEquation(
                row2,
                pre_op,
                op_modes.clone(),
                op_val1.clone(),
                op_val2.clone(),
                op_val3.clone(),
                op_val4.clone(),
            )?;
            exp_dae = buildExpression(factor1, factor2, pivot_exp, update_exp)?;
            return Ok(exp_dae);
            pre_mode
        }
        1 => {
            if row1 == current_row {
                exp_dae = traceEquation(
                    row2,
                    pre_op,
                    op_modes.clone(),
                    op_val1.clone(),
                    op_val2.clone(),
                    op_val3.clone(),
                    op_val4.clone(),
                )?;
            } else if row2 == current_row {
                exp_dae = traceEquation(
                    row1,
                    pre_op,
                    op_modes.clone(),
                    op_val1.clone(),
                    op_val2.clone(),
                    op_val3.clone(),
                    op_val4.clone(),
                )?;
            }
            return Ok(exp_dae);
            pre_mode
        }
        2 => {
            exp_dae = metamodelica::Ref::new(DAE::Exp::BINARY {
                exp1: traceEquation(
                    row1,
                    pre_op,
                    op_modes.clone(),
                    op_val1.clone(),
                    op_val2.clone(),
                    op_val3.clone(),
                    op_val4.clone(),
                )?,
                operator: DAE::Operator::DIV {
                    ty: DAE::T_REAL_DEFAULT().clone(),
                },
                exp2: metamodelica::Ref::new(DAE::Exp::RCONST {
                    real: metamodelica::OrderedFloat((gcd) as f64),
                }),
            });
            return Ok(exp_dae);
            pre_mode
        }
        _ => pre_mode,
    });
    return Ok(exp_dae);
    Ok(exp_dae)
}

pub(crate) fn findLastOperation(
    mut current_row: i32,
    mut last_op: i32,
    mut op_modes: metamodelica::Array<i32>,
    mut op_val1: metamodelica::Array<i32>,
    mut op_val2: metamodelica::Array<i32>,
    mut op_val3: metamodelica::Array<i32>,
    mut op_val4: metamodelica::Array<i32>,
) -> Result<(bool, i32, i32, i32, i32, i32, i32, i32)> {
    let __ab_op_modes = op_modes.borrow();
    let __ab_op_val1 = op_val1.borrow();
    let __ab_op_val2 = op_val2.borrow();
    let __ab_op_val3 = op_val3.borrow();
    let __ab_op_val4 = op_val4.borrow();
    let mut found: bool = false;
    let mut pre_op: i32;
    let mut pre_mode: i32;
    let mut row1: i32;
    let mut row2: i32;
    let mut factor1: i32;
    let mut factor2: i32;
    let mut gcd: i32;
    (pre_op, pre_mode, row1, row2, factor1, factor2, gcd) = (0, 0, 0, 0, 0, 0, 0);
    for mut op in ({
        let __s = last_op;
        let __e = 1;
        (0i32..)
            .map(move |__k| __s + __k * (-1))
            .take_while(move |&__v| __v >= __e)
    }) {
        if (*metamodelica::index_checked(&__ab_op_val3, op)?).clone() == current_row
            && (*metamodelica::index_checked(&__ab_op_modes, op)?).clone() == 0
        {
            pre_mode = 0;
            row1 = (*metamodelica::index_checked(&__ab_op_val1, op)?).clone();
            row2 = (*metamodelica::index_checked(&__ab_op_val3, op)?).clone();
            factor1 = (*metamodelica::index_checked(&__ab_op_val2, op)?).clone();
            factor2 = (*metamodelica::index_checked(&__ab_op_val4, op)?).clone();
            found = true;
            pre_op = op - 1;
            return Ok((found, pre_op, pre_mode, row1, row2, factor1, factor2, gcd));
        } else if ((*metamodelica::index_checked(&__ab_op_val1, op)?).clone() == current_row
            || (*metamodelica::index_checked(&__ab_op_val2, op)?).clone() == current_row)
            && (*metamodelica::index_checked(&__ab_op_modes, op)?).clone() == 1
        {
            pre_mode = 1;
            row1 = (*metamodelica::index_checked(&__ab_op_val1, op)?).clone();
            row2 = (*metamodelica::index_checked(&__ab_op_val2, op)?).clone();
            found = true;
            pre_op = op - 1;
            return Ok((found, pre_op, pre_mode, row1, row2, factor1, factor2, gcd));
        } else if (*metamodelica::index_checked(&__ab_op_val1, op)?).clone() == current_row
            && (*metamodelica::index_checked(&__ab_op_modes, op)?).clone() == 2
        {
            pre_mode = 2;
            row1 = (*metamodelica::index_checked(&__ab_op_val1, op)?).clone();
            gcd = (*metamodelica::index_checked(&__ab_op_val2, op)?).clone();
            found = true;
            pre_op = op - 1;
            return Ok((found, pre_op, pre_mode, row1, row2, factor1, factor2, gcd));
        }
    }
    Ok((found, pre_op, pre_mode, row1, row2, factor1, factor2, gcd))
}

pub(crate) fn buildExpression(
    mut factor_pivot: i32,
    mut factor_update: i32,
    mut pivot_exp: metamodelica::Ref<DAE::Exp>,
    mut update_exp: metamodelica::Ref<DAE::Exp>,
) -> Result<metamodelica::Ref<DAE::Exp>> {
    let mut exp_dae: metamodelica::Ref<DAE::Exp>;
    let mut exp_dae_elem: metamodelica::Ref<DAE::Exp>;
    if Flags::isSet(Flags::DUMP_ASSC.clone())? {
        metamodelica::print(literal!("Step-by-step construction of the zero row:\n"));
    }
    exp_dae = metamodelica::Ref::new(DAE::Exp::BINARY {
        exp1: metamodelica::Ref::new(DAE::Exp::BINARY {
            exp1: metamodelica::Ref::new(DAE::Exp::RCONST {
                real: metamodelica::OrderedFloat((factor_pivot) as f64),
            }),
            operator: DAE::Operator::MUL {
                ty: DAE::T_REAL_DEFAULT().clone(),
            },
            exp2: update_exp,
        }),
        operator: DAE::Operator::SUB {
            ty: DAE::T_REAL_DEFAULT().clone(),
        },
        exp2: metamodelica::Ref::new(DAE::Exp::BINARY {
            exp1: metamodelica::Ref::new(DAE::Exp::RCONST {
                real: metamodelica::OrderedFloat((factor_update) as f64),
            }),
            operator: DAE::Operator::MUL {
                ty: DAE::T_REAL_DEFAULT().clone(),
            },
            exp2: pivot_exp,
        }),
    });
    if Flags::isSet(Flags::DUMP_ASSC.clone())? {
        metamodelica::print({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*ExpressionBasics::printExpStr(exp_dae.clone())?);
            __mm_s.push_str(&*literal!("\n"));
            ArcStr::from(__mm_s)
        });
    }
    (exp_dae, _) = ExpressionSimplify::simplify(exp_dae)?;
    if Flags::isSet(Flags::DUMP_ASSC.clone())? {
        metamodelica::print({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("Simplified expression: "));
            __mm_s.push_str(&*ExpressionBasics::printExpStr(exp_dae.clone())?);
            __mm_s.push_str(&*literal!("\n"));
            ArcStr::from(__mm_s)
        });
    }
    Ok(exp_dae)
}

pub(crate) fn createEquations(
    mut vars: &metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>>,
    mut index: Pointer::Pointer<i32>,
    mut indices: metamodelica::Array<metamodelica::List<i32>>,
    mut values: metamodelica::Array<metamodelica::List<i32>>,
    mut num_eqns: i32,
    mut lhs_array: metamodelica::Array<metamodelica::Ref<Expression::NFExpression>>,
) -> Result<metamodelica::List<Pointer::Pointer<metamodelica::Ref<Equation::Equation>>>> {
    let __ab_indices = indices.borrow();
    let __ab_lhs_array = lhs_array.borrow();
    let __ab_values = values.borrow();
    let mut resolved_eqns: metamodelica::List<Pointer::Pointer<metamodelica::Ref<Equation::Equation>>> =
        metamodelica::nil();
    let mut new_eq: Pointer::Pointer<metamodelica::Ref<Equation::Equation>>;
    let mut cref_exp: metamodelica::Ref<Expression::NFExpression>;
    let mut sub_exp: metamodelica::Ref<Expression::NFExpression>;
    let mut rhs: metamodelica::Ref<Expression::NFExpression>;
    let mut lhs: metamodelica::Ref<Expression::NFExpression>;
    let mut indices_list: metamodelica::List<i32>;
    let mut values_list: metamodelica::List<i32>;
    let mut status: Status;
    let mut solved_eq: metamodelica::Ref<Equation::Equation>;
    for mut i in ({
        let __s = num_eqns;
        let __e = 1;
        (0i32..)
            .map(move |__k| __s + __k * (-1))
            .take_while(move |&__v| __v >= __e)
    }) {
        rhs = Expression::makeInteger(0);
        for mut j in 1..=((*metamodelica::index_checked(&__ab_indices, i)?).len() as i32) {
            indices_list = (*metamodelica::index_checked(&__ab_indices, i)?).clone();
            cref_exp = Expression::fromCref((vars).get((indices_list).get(j)? + 1)?, false)?;
            values_list = (*metamodelica::index_checked(&__ab_values, i)?).clone();
            sub_exp = metamodelica::Ref::new(Expression::NFExpression::MULTARY {
                arguments: list![cref_exp, Expression::makeInteger((values_list).get(j)?)],
                inv_arguments: metamodelica::nil(),
                operator: Operator::makeMul(openmodelica_nf_frontend::NFType::interned_REAL()),
            });
            sub_exp = SimplifyExp::simplify(sub_exp, false)?;
            rhs = metamodelica::Ref::new(Expression::NFExpression::MULTARY {
                arguments: list![rhs, sub_exp.clone()],
                inv_arguments: metamodelica::nil(),
                operator: Operator::makeAdd(Expression::typeOf(sub_exp)),
            });
        }
        rhs = SimplifyExp::simplify(rhs, false)?;
        if Flags::isSet(Flags::DUMP_ASSC.clone())? {
            metamodelica::print({
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("rhs: "));
                __mm_s.push_str(&*Expression::toString(rhs.clone())?);
                __mm_s.push_str(&*literal!("\n"));
                ArcStr::from(__mm_s)
            });
        }
        new_eq = Equation::makeAssignment(
            rhs,
            (*metamodelica::index_checked(&__ab_lhs_array, i)?).clone(),
            index.clone(),
            &(arcstr::literal!(NBEquation::TMP_STR)),
            crate::NBEquation::Iterator::interned_EMPTY(),
            NBEquation::default(EquationKind::UNKNOWN.clone(), false, None, None),
        )?;
        if Flags::isSet(Flags::DUMP_ASSC.clone())? {
            metamodelica::print({
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("new_eq: "));
                __mm_s.push_str(&*Equation::toString(Pointer::access(new_eq.clone()), literal!(""))?);
                __mm_s.push_str(&*literal!("\n"));
                ArcStr::from(__mm_s)
            });
        }
        (solved_eq, status, _) = Solve::solveBody(
            Pointer::access(new_eq),
            (vars).get(i)?,
            UnorderedMap::new(
                (std::sync::Arc::new(move |__a0: metamodelica::Ref<Absyn::Path>| -> metamodelica::Result<_> {
                    ::std::result::Result::Ok(AbsynUtil::pathHash(&__a0))
                })
                    as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<Absyn::Path>) -> Result<i32> + 'static>),
                (std::sync::Arc::new(
                    move |__a0: metamodelica::Ref<Absyn::Path>,
                          __a1: metamodelica::Ref<Absyn::Path>|
                          -> metamodelica::Result<_> {
                        ::std::result::Result::Ok(AbsynUtil::pathEqual(&__a0, &__a1))
                    },
                )
                    as std::sync::Arc<
                        dyn ::std::ops::Fn(
                                metamodelica::Ref<Absyn::Path>,
                                metamodelica::Ref<Absyn::Path>,
                            ) -> Result<bool>
                            + 'static,
                    >),
                1,
            ),
        )?;
        if Flags::isSet(Flags::DUMP_ASSC.clone())? {
            metamodelica::print({
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("solved_eq: "));
                __mm_s.push_str(&*Equation::toString(solved_eq.clone(), literal!(""))?);
                __mm_s.push_str(&*literal!("\n"));
                ArcStr::from(__mm_s)
            });
        }
        resolved_eqns = metamodelica::cons(Pointer::create(solved_eq), resolved_eqns);
    }
    if Flags::isSet(Flags::DUMP_ASSC.clone())? {
        metamodelica::print({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("Number of equations: "));
            __mm_s.push_str(&*intString(((resolved_eqns).len() as i32)));
            __mm_s.push_str(&*literal!("\n"));
            ArcStr::from(__mm_s)
        });
        for mut eq_ptr in &*resolved_eqns {
            metamodelica::print({
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("eq_ptr: "));
                __mm_s.push_str(&*Equation::toString(Pointer::access(eq_ptr.clone()), literal!(""))?);
                __mm_s.push_str(&*literal!("\n"));
                ArcStr::from(__mm_s)
            });
        }
    }
    Ok(resolved_eqns)
}

pub(crate) fn setMatrix(
    mut nv: i32,
    mut ne: i32,
    mut nz: i32,
    mut adj: metamodelica::Array<metamodelica::List<i32>>,
    mut val: metamodelica::Array<metamodelica::List<i32>>,
) -> () {
    crate::NBASSCExt::ASSC_setMatrix(nv.clone(), ne.clone(), nz.clone(), adj.clone(), val.clone());
    ()
}

pub(crate) fn getMatrix(
    mut adj: metamodelica::Array<metamodelica::List<i32>>,
    mut val: metamodelica::Array<metamodelica::List<i32>>,
) -> () {
    crate::NBASSCExt::ASSC_getMatrix(adj.clone(), val.clone());
    ()
}

pub(crate) fn freeMatrix() -> () {
    crate::NBASSCExt::ASSC_freeMatrix();
    ()
}

pub(crate) fn printMatrix() -> () {
    crate::NBASSCExt::ASSC_printMatrix();
    ()
}

pub(crate) fn bareiss() -> () {
    crate::NBASSCExt::ASSC_bareiss();
    ()
}

pub(crate) fn getNumberOfOperations(mut nop: metamodelica::Array<i32>) -> i32 {
    let mut num: i32 = 0;
    num = crate::NBASSCExt::ASSC_getNumberOfOperations(nop.clone());
    num
}

pub(crate) fn getOperations(
    mut op_modes: metamodelica::Array<i32>,
    mut op_val1: metamodelica::Array<i32>,
    mut op_val2: metamodelica::Array<i32>,
    mut op_val3: metamodelica::Array<i32>,
    mut op_val4: metamodelica::Array<i32>,
) -> Result<()> {
    crate::NBASSCExt::ASSC_getOperations(
        op_modes.clone(),
        op_val1.clone(),
        op_val2.clone(),
        op_val3.clone(),
        op_val4.clone(),
    )?;
    Ok(())
}

pub type CrefLst = metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>>;

pub mod Tuple_Id {
    use super::*;
    /// tuple as key for UnorderedMap
    #[derive(Clone, Debug, Eq, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
    pub struct Tuple_Id {
        pub eq_ptr: Pointer::Pointer<metamodelica::Ref<Equation::Equation>>,
        pub cref: metamodelica::Ref<ComponentRef::NFComponentRef>,
    }

    impl metamodelica::gc::MMTrace for Tuple_Id {
        fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
            metamodelica::gc::MMTrace::mm_accept(&self.eq_ptr, __mmv)?;
            metamodelica::gc::MMTrace::mm_accept(&self.cref, __mmv)?;
            Ok(())
        }
    }
    impl Default for Tuple_Id {
        fn default() -> Self {
            Self {
                eq_ptr: Default::default(),
                cref: Default::default(),
            }
        }
    }

    pub type TUPLE_ID = Tuple_Id;

    pub(crate) fn toString(mut id: &metamodelica::Ref<Tuple_Id>) -> Result<ArcStr> {
        let mut r#str: ArcStr;
        r#str = Equation::toString(Pointer::access(id.eq_ptr.clone()), literal!(""))?;
        r#str = {
            let mut __mm_s = String::new();
            __mm_s.push_str(&*BVariable::toString(
                &(BVariable::getVar(&id.cref, metamodelica::sourceInfo!("NBackEnd/Util/NBASSC.mo"))?),
                literal!(""),
            )?);
            __mm_s.push_str(&*r#str);
            ArcStr::from(__mm_s)
        };
        Ok(r#str)
    }

    pub(crate) fn hash(mut id: &metamodelica::Ref<Tuple_Id>) -> Result<i32> {
        let mut hash: i32;
        hash = stringHashDjb2(&(toString(id)?));
        Ok(hash)
    }

    pub(crate) fn isEqual(
        mut id1: &metamodelica::Ref<Tuple_Id>,
        mut id2: &metamodelica::Ref<Tuple_Id>,
    ) -> Result<bool> {
        let mut b: bool;
        b = Equation::isEqualPtr(id1.eq_ptr.clone(), id2.eq_ptr.clone())?
            && ComponentRef::isEqual(&id1.cref, &id2.cref)?;
        Ok(b)
    }
}
