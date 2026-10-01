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

use crate::NFAlgorithm as Algorithm;
use crate::NFAttributes as Attributes;
use crate::NFBinding as Binding;
use crate::NFComponentRef as ComponentRef;
use crate::NFEquation as Equation;
use crate::NFExpandExp as ExpandExp;
use crate::NFExpression as Expression;
use crate::NFFlatModel as FlatModel;
use crate::NFPrefixes::Direction;
use crate::NFPrefixes::Variability;
use crate::NFStatement as Statement;
use crate::NFType as Type;
use crate::NFVariable as Variable;
use openmodelica_util::UnorderedSet;
use openmodelica_util::Util;
use openmodelica_util_datatypes_basic::List;

pub(crate) fn checkModel(mut flatModel: &metamodelica::Ref<FlatModel::NFFlatModel>) -> Result<(i32, i32)> {
    let mut variables: i32 = 0;
    let mut equations: i32 = 0;
    for mut v in &*flatModel.variables.clone() {
        (variables, equations) = countVariableSize(metamodelica::AsArg::as_arg(&v), variables, equations)?;
    }
    equations = equations + Equation::sizeOfList(&flatModel.equations);
    for mut a in &*flatModel.algorithms.clone() {
        equations = equations + countAlgorithmSize(metamodelica::AsArg::as_arg(&a))?;
    }
    Ok((variables, equations))
}

pub(crate) fn countVariableSize(
    mut var: &metamodelica::Ref<Variable::NFVariable>,
    mut variables: i32,
    mut equations: i32,
) -> Result<(i32, i32)> {
    let mut variables: i32 = variables;
    let mut equations: i32 = equations;
    let mut ty: metamodelica::Ref<Type::NFType>;
    let mut binding: metamodelica::Ref<Binding::NFBinding>;
    let mut attr: metamodelica::Ref<Attributes::NFAttributes>;
    let mut var_size: i32;
    let __arc3 = &(*var);
    let Variable::VARIABLE {
        ty: __pa0,
        binding: __pa1,
        attributes: __pa2,
        ..
    } = &**__arc3;
    ty = metamodelica::Own::own(__pa0);
    binding = metamodelica::Own::own(__pa1);
    attr = metamodelica::Own::own(__pa2);
    if attr.variability.clone() < Variability::DISCRETE.clone() {
        return Ok((variables, equations));
    }
    if Type::isExternalObject(&ty) {
        return Ok((variables, equations));
    }
    var_size = Type::sizeOf(&ty, false)?;
    variables = variables + var_size;
    if Variable::isTopLevelInput(var) {
        equations = equations + var_size;
    } else {
        equations = equations + Type::sizeOf(&(Binding::getType(&binding)?), false)?;
    }
    Ok((variables, equations))
}

pub(crate) fn countAlgorithmSize(mut alg: &metamodelica::Ref<Algorithm::NFAlgorithm>) -> Result<i32> {
    let mut equations: i32 = 0;
    let mut crefs: metamodelica::Ref<UnorderedSet::UnorderedSet<metamodelica::Ref<ComponentRef::NFComponentRef>>>;
    crefs = UnorderedSet::new(
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
    crefs =
        List::fold(
            &alg.statements,
            &move |__a0: metamodelica::Ref<Statement::NFStatement>,
                   __a1: metamodelica::Ref<
                UnorderedSet::UnorderedSet<metamodelica::Ref<ComponentRef::NFComponentRef>>,
            >| statementOutputs(&__a0, __a1),
            crefs,
        )?;
    equations = equations + UnorderedSet::size(crefs.clone());
    metamodelica::print({
        let mut __mm_s = String::new();
        __mm_s.push_str(&*literal!("Algorithm size: "));
        __mm_s.push_str(&*ArcStr::from(::std::format!("{}", UnorderedSet::size(crefs.clone()))));
        __mm_s.push_str(&*literal!("\n"));
        ArcStr::from(__mm_s)
    });
    for mut cr in &*UnorderedSet::toList(crefs) {
        metamodelica::print({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*ComponentRef::toString(metamodelica::AsArg::as_arg(&cr))?);
            __mm_s.push_str(&*literal!("\n"));
            ArcStr::from(__mm_s)
        });
    }
    Ok(equations)
}

fn statementOutputs(
    mut stmt: &metamodelica::Ref<Statement::NFStatement>,
    mut crefs: metamodelica::Ref<UnorderedSet::UnorderedSet<metamodelica::Ref<ComponentRef::NFComponentRef>>>,
) -> Result<metamodelica::Ref<UnorderedSet::UnorderedSet<metamodelica::Ref<ComponentRef::NFComponentRef>>>> {
    let mut crefs: metamodelica::Ref<UnorderedSet::UnorderedSet<metamodelica::Ref<ComponentRef::NFComponentRef>>> =
        crefs;
    crefs = (match &**stmt {
        Statement::ASSIGNMENT { lhs: __stmt_lhs, .. } => Expression::fold(
            __stmt_lhs.clone(),
            (std::sync::Arc::new(
                move |__a0: metamodelica::Ref<Expression::NFExpression>,
                      __a1: metamodelica::Ref<
                    UnorderedSet::UnorderedSet<metamodelica::Ref<ComponentRef::NFComponentRef>>,
                >| statementOutputCrefFinder(&__a0, __a1),
            )
                as std::sync::Arc<
                    dyn ::std::ops::Fn(
                            metamodelica::Ref<Expression::NFExpression>,
                            metamodelica::Ref<
                                UnorderedSet::UnorderedSet<metamodelica::Ref<ComponentRef::NFComponentRef>>,
                            >,
                        ) -> Result<
                            metamodelica::Ref<
                                UnorderedSet::UnorderedSet<metamodelica::Ref<ComponentRef::NFComponentRef>>,
                            >,
                        > + 'static,
                >),
            crefs,
        )?,
        Statement::FOR { body: __stmt_body, .. } => List::fold(
            metamodelica::AsArg::as_arg(&__stmt_body),
            &move |__a0: metamodelica::Ref<Statement::NFStatement>,
                   __a1: metamodelica::Ref<
                UnorderedSet::UnorderedSet<metamodelica::Ref<ComponentRef::NFComponentRef>>,
            >| statementOutputs(&__a0, __a1),
            crefs,
        )?,
        Statement::IF {
            branches: __stmt_branches,
            ..
        } => {
            for mut b in &*__stmt_branches.clone() {
                crefs = List::fold(
                    &(Util::tuple22(b.clone())),
                    &move |__a0: metamodelica::Ref<Statement::NFStatement>,
                           __a1: metamodelica::Ref<
                        UnorderedSet::UnorderedSet<metamodelica::Ref<ComponentRef::NFComponentRef>>,
                    >| statementOutputs(&__a0, __a1),
                    crefs,
                )?;
            }
            crefs
        }
        Statement::WHEN {
            branches: __stmt_branches,
            ..
        } => {
            for mut b in &*__stmt_branches.clone() {
                crefs = List::fold(
                    &(Util::tuple22(b.clone())),
                    &move |__a0: metamodelica::Ref<Statement::NFStatement>,
                           __a1: metamodelica::Ref<
                        UnorderedSet::UnorderedSet<metamodelica::Ref<ComponentRef::NFComponentRef>>,
                    >| statementOutputs(&__a0, __a1),
                    crefs,
                )?;
            }
            crefs
        }
        Statement::WHILE { body: __stmt_body, .. } => List::fold(
            metamodelica::AsArg::as_arg(&__stmt_body),
            &move |__a0: metamodelica::Ref<Statement::NFStatement>,
                   __a1: metamodelica::Ref<
                UnorderedSet::UnorderedSet<metamodelica::Ref<ComponentRef::NFComponentRef>>,
            >| statementOutputs(&__a0, __a1),
            crefs,
        )?,
        _ => crefs,
    });
    Ok(crefs)
}

fn statementOutputCrefFinder(
    mut exp: &metamodelica::Ref<Expression::NFExpression>,
    mut crefs: metamodelica::Ref<UnorderedSet::UnorderedSet<metamodelica::Ref<ComponentRef::NFComponentRef>>>,
) -> Result<metamodelica::Ref<UnorderedSet::UnorderedSet<metamodelica::Ref<ComponentRef::NFComponentRef>>>> {
    let mut crefs: metamodelica::Ref<UnorderedSet::UnorderedSet<metamodelica::Ref<ComponentRef::NFComponentRef>>> =
        crefs;
    let mut cref: metamodelica::Ref<ComponentRef::NFComponentRef>;
    crefs = (match &**exp {
        Expression::CREF { cref: __exp_cref, .. } => {
            (cref, _) = ComponentRef::stripSubscripts(__exp_cref.clone());
            Expression::fold(
                (ExpandExp::expand(Expression::fromCref(cref, false)?, false, false)?).0,
                (std::sync::Arc::new(
                    move |__a0: metamodelica::Ref<Expression::NFExpression>,
                          __a1: metamodelica::Ref<
                        UnorderedSet::UnorderedSet<metamodelica::Ref<ComponentRef::NFComponentRef>>,
                    >| statementOutputCrefFinder2(&__a0, __a1),
                )
                    as std::sync::Arc<
                        dyn ::std::ops::Fn(
                                metamodelica::Ref<Expression::NFExpression>,
                                metamodelica::Ref<
                                    UnorderedSet::UnorderedSet<metamodelica::Ref<ComponentRef::NFComponentRef>>,
                                >,
                            ) -> Result<
                                metamodelica::Ref<
                                    UnorderedSet::UnorderedSet<metamodelica::Ref<ComponentRef::NFComponentRef>>,
                                >,
                            > + 'static,
                    >),
                crefs,
            )?
        }
        _ => crefs,
    });
    Ok(crefs)
}

fn statementOutputCrefFinder2(
    mut exp: &metamodelica::Ref<Expression::NFExpression>,
    mut crefs: metamodelica::Ref<UnorderedSet::UnorderedSet<metamodelica::Ref<ComponentRef::NFComponentRef>>>,
) -> Result<metamodelica::Ref<UnorderedSet::UnorderedSet<metamodelica::Ref<ComponentRef::NFComponentRef>>>> {
    let mut crefs: metamodelica::Ref<UnorderedSet::UnorderedSet<metamodelica::Ref<ComponentRef::NFComponentRef>>> =
        crefs;
    let () = (match &**exp {
        Expression::CREF { cref: __exp_cref, .. }
            if (ComponentRef::isCref(metamodelica::AsArg::as_arg(&__exp_cref))
                && !(ComponentRef::isIterator(metamodelica::AsArg::as_arg(&__exp_cref)))) =>
        {
            UnorderedSet::add(__exp_cref.clone(), crefs.clone())?;
            ()
        }
        _ => (),
    });
    Ok(crefs)
}
