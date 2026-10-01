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
use crate::NFBinding as Binding;
use crate::NFComponentRef as ComponentRef;
use crate::NFDimension as Dimension;
use crate::NFEquation as Equation;
use crate::NFExpandExp as ExpandExp;
use crate::NFExpression as Expression;
use crate::NFFlatModel as FlatModel;
use crate::NFInstNode::InstNode;
use crate::NFPrefixes::Variability;
use crate::NFRecord as Record;
use crate::NFStatement as Statement;
use crate::NFSubscript as Subscript;
use crate::NFType as Type;
use crate::NFVariable as Variable;
use openmodelica_error::ErrorTypes;
use openmodelica_frontend_dump::ElementSource;
use openmodelica_frontend_types::DAE;
use openmodelica_util::Error;
use openmodelica_util::ExecStat::execStat;
use openmodelica_util::UnorderedSet;
use openmodelica_util::Util;
use openmodelica_util_datatypes_basic::List;

pub struct NFVerifyModel;
pub fn verify(mut flatModel: &metamodelica::Ref<FlatModel::NFFlatModel>, mut isPartial: bool) -> Result<()> {
    for mut var in &*flatModel.variables.clone() {
        verifyVariable(metamodelica::AsArg::as_arg(&var), isPartial)?;
    }
    for mut eq in &*flatModel.equations.clone() {
        verifyEquation(metamodelica::AsArg::as_arg(&eq), isPartial)?;
    }
    for mut ieq in &*flatModel.initialEquations.clone() {
        verifyEquation(metamodelica::AsArg::as_arg(&ieq), isPartial)?;
    }
    for mut alg in &*flatModel.algorithms.clone() {
        verifyAlgorithm(metamodelica::AsArg::as_arg(&alg), isPartial)?;
    }
    for mut ialg in &*flatModel.initialAlgorithms.clone() {
        verifyAlgorithm(metamodelica::AsArg::as_arg(&ialg), isPartial)?;
    }
    if !(isPartial) {
        checkDiscreteReal(flatModel)?;
    }
    execStat(&(literal!("NFVerifyModel.verify")))?;
    Ok(())
}

fn verifyVariable(mut var: &metamodelica::Ref<Variable::NFVariable>, mut isPartial: bool) -> Result<()> {
    verifyBinding(&var.binding, isPartial)?;
    for mut attr in &*var.typeAttributes.clone() {
        verifyBinding(&(Util::tuple22(attr.clone())), isPartial)?;
    }
    for mut v in &*var.children.clone() {
        verifyVariable(metamodelica::AsArg::as_arg(&v), isPartial)?;
    }
    Ok(())
}

fn verifyBinding(mut binding: &metamodelica::Ref<Binding::NFBinding>, mut isPartial: bool) -> Result<()> {
    if Binding::isBound(binding) {
        checkSubscriptBounds(Binding::getTypedExp(binding)?, isPartial, &(Binding::getInfo(binding)))?;
    }
    Ok(())
}

fn verifyEquation(mut eq: &metamodelica::Ref<Equation::NFEquation>, mut isPartial: bool) -> Result<()> {
    let () = (match &**eq {
        Equation::WHEN {
            branches: __eq_branches,
            source: __eq_source,
            ..
        } if (!(isPartial)) => {
            verifyWhenEquation(metamodelica::AsArg::as_arg(&__eq_branches), __eq_source.clone())?;
            ()
        }
        _ => (),
    });
    Equation::applyExpShallow(
        eq,
        &({
            let __pe_b1 = isPartial;
            let __pe_b2 = Equation::info(eq);
            move |__pe_a0| checkSubscriptBounds(__pe_a0, __pe_b1.clone(), &__pe_b2)
        }),
    )?;
    Ok(())
}

fn verifyWhenEquation(
    mut branches: &metamodelica::List<metamodelica::Ref<Equation::Branch::Branch>>,
    mut source: metamodelica::Ref<DAE::ElementSource>,
) -> Result<()> {
    let mut crefs1: metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>>;
    let mut crefs2: metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>>;
    let mut rest_branches: metamodelica::List<metamodelica::Ref<Equation::Branch::Branch>>;
    let mut body: metamodelica::List<metamodelica::Ref<Equation::NFEquation>>;
    if List::hasOneElement(branches) {
        return Ok(());
    }
    let (__pa0, __pa1) = ::match_deref::match_deref! { match &((*branches)) {
        Deref @ metamodelica::ListNode::Cons { head: Deref @ Equation::Branch::BRANCH { body: __pa0, .. }, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
        _ => return Err("pattern mismatch"),
    } };
    body = metamodelica::Own::own(__pa0);
    rest_branches = metamodelica::Own::own(__pa1);
    crefs1 = whenEquationBranchCrefs(&body)?;
    for mut branch in &*rest_branches {
        let __pa2 = ::match_deref::match_deref! { match &(branch.clone()) {
            Deref @ Equation::Branch::BRANCH { body: __pa2, .. } => __pa2.clone(),
            _ => return Err("pattern mismatch"),
        } };
        body = metamodelica::Own::own(__pa2);
        crefs2 = whenEquationBranchCrefs(&body)?;
        checkCrefSetEquality(
            crefs1.clone(),
            crefs2,
            &(Error::DIFFERENT_VARIABLES_SOLVED_IN_ELSEWHEN.clone()),
            source.clone(),
        )?;
    }
    Ok(())
}

fn whenEquationBranchCrefs(
    mut eql: &metamodelica::List<metamodelica::Ref<Equation::NFEquation>>,
) -> Result<metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>>> {
    let mut crefs: metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>> = metamodelica::nil();
    for mut eq in &**eql {
        crefs = (match &*eq.clone() {
            Equation::EQUALITY { lhs: __eq_lhs, .. } => {
                whenEquationEqualityCrefs(metamodelica::AsArg::as_arg(&__eq_lhs), crefs)?
            }
            Equation::IF {
                branches: __eq_branches,
                source: __eq_source,
                ..
            } => whenEquationIfCrefs(metamodelica::AsArg::as_arg(&__eq_branches), __eq_source.clone(), crefs)?,
            _ => crefs,
        });
    }
    crefs = List::sort(
        crefs,
        (std::sync::Arc::new(
            move |__a0: metamodelica::Ref<ComponentRef::NFComponentRef>,
                  __a1: metamodelica::Ref<ComponentRef::NFComponentRef>| {
                ComponentRef::isGreater(&__a0, &__a1)
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
    crefs = List::sortedUnique(
        crefs,
        &move |__a0: metamodelica::Ref<ComponentRef::NFComponentRef>,
               __a1: metamodelica::Ref<ComponentRef::NFComponentRef>| ComponentRef::isEqual(&__a0, &__a1),
    )?;
    Ok(crefs)
}

fn whenEquationEqualityCrefs(
    mut lhsExp: &metamodelica::Ref<Expression::NFExpression>,
    mut crefs: metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>>,
) -> Result<metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>>> {
    let mut crefs: metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>> = crefs;
    crefs = (match &**lhsExp {
        Expression::CREF {
            cref: __lhsExp_cref, ..
        } => metamodelica::cons(__lhsExp_cref.clone(), crefs),
        Expression::TUPLE {
            elements: __lhsExp_elements,
            ..
        } => List::fold(
            metamodelica::AsArg::as_arg(&__lhsExp_elements),
            &move |__a0: metamodelica::Ref<Expression::NFExpression>,
                   __a1: metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>>| {
                whenEquationEqualityCrefs(&__a0, __a1)
            },
            crefs,
        )?,
        _ => return Err("match: no arm matched"),
    });
    Ok(crefs)
}

fn whenEquationIfCrefs(
    mut branches: &metamodelica::List<metamodelica::Ref<Equation::Branch::Branch>>,
    mut source: metamodelica::Ref<DAE::ElementSource>,
    mut crefs: metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>>,
) -> Result<metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>>> {
    let mut crefs: metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>> = crefs;
    let mut crefs1: metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>>;
    let mut crefs2: metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>>;
    let mut rest_branches: metamodelica::List<metamodelica::Ref<Equation::Branch::Branch>>;
    let mut body: metamodelica::List<metamodelica::Ref<Equation::NFEquation>>;
    let (__pa0, __pa1) = ::match_deref::match_deref! { match &((*branches)) {
        Deref @ metamodelica::ListNode::Cons { head: Deref @ Equation::Branch::BRANCH { body: __pa0, .. }, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
        _ => return Err("pattern mismatch"),
    } };
    body = metamodelica::Own::own(__pa0);
    rest_branches = metamodelica::Own::own(__pa1);
    crefs1 = whenEquationBranchCrefs(&body)?;
    for mut branch in &*rest_branches {
        let __pa2 = ::match_deref::match_deref! { match &(branch.clone()) {
            Deref @ Equation::Branch::BRANCH { body: __pa2, .. } => __pa2.clone(),
            _ => return Err("pattern mismatch"),
        } };
        body = metamodelica::Own::own(__pa2);
        crefs2 = whenEquationBranchCrefs(&body)?;
        checkCrefSetEquality(
            crefs1.clone(),
            crefs2,
            &(Error::WHEN_IF_VARIABLE_MISMATCH.clone()),
            source.clone(),
        )?;
    }
    crefs = listAppend(crefs1, crefs);
    Ok(crefs)
}

fn checkCrefSetEquality(
    mut crefs1: metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>>,
    mut crefs2: metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>>,
    mut errMsg: &ErrorTypes::Message,
    mut source: metamodelica::Ref<DAE::ElementSource>,
) -> Result<()> {
    if List::isEqualOnTrue(crefs1.clone(), crefs2.clone(), &move |__a0: metamodelica::Ref<
        ComponentRef::NFComponentRef,
    >,
                                                                  __a1: metamodelica::Ref<
        ComponentRef::NFComponentRef,
    >| {
        ComponentRef::isEqual(&__a0, &__a1)
    })? {
        return Ok(());
    }
    if List::isEqualOnTrue(
        expandCrefSet(&crefs1)?,
        expandCrefSet(&crefs2)?,
        &move |__a0: metamodelica::Ref<ComponentRef::NFComponentRef>,
               __a1: metamodelica::Ref<ComponentRef::NFComponentRef>| ComponentRef::isEqual(&__a0, &__a1),
    )? {
        return Ok(());
    }
    Error::addSourceMessage(errMsg, metamodelica::nil(), &(ElementSource::getInfo(source)))?;
    return Err("fail");
    Ok(())
}

fn expandCrefSet(
    mut crefs: &metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>>,
) -> Result<metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>>> {
    let mut outCrefs: metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>> = metamodelica::nil();
    let mut exp: metamodelica::Ref<Expression::NFExpression>;
    let mut expl: metamodelica::Array<metamodelica::Ref<Expression::NFExpression>>;
    for mut cref in &**crefs {
        exp = Expression::fromCref(cref.clone(), false)?;
        (exp, _) = ExpandExp::expandCref(exp, false, false)?;
        if Expression::isArray(&exp) {
            expl = Expression::arrayElements(&exp)?;
            outCrefs = listAppend(
                ({
                    let mut __acc: metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>> =
                        metamodelica::nil();
                    for mut e in (expl.clone()).borrow().iter() {
                        let __x = Expression::toCref(&(e.clone()))?;
                        __acc = cons(__x, __acc);
                    }
                    __acc.reverse()
                }),
                outCrefs,
            );
        } else {
            outCrefs = metamodelica::cons(cref.clone(), outCrefs);
        }
    }
    outCrefs = List::sort(
        outCrefs,
        (std::sync::Arc::new(
            move |__a0: metamodelica::Ref<ComponentRef::NFComponentRef>,
                  __a1: metamodelica::Ref<ComponentRef::NFComponentRef>| {
                ComponentRef::isGreater(&__a0, &__a1)
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
    outCrefs = List::sortedUnique(outCrefs, &move |__a0: metamodelica::Ref<
        ComponentRef::NFComponentRef,
    >,
                                                   __a1: metamodelica::Ref<
        ComponentRef::NFComponentRef,
    >| ComponentRef::isEqual(&__a0, &__a1))?;
    Ok(outCrefs)
}

fn verifyAlgorithm(mut alg: &metamodelica::Ref<Algorithm::NFAlgorithm>, mut isPartial: bool) -> Result<()> {
    Algorithm::apply(
        alg,
        &({
            let __pe_b1 = isPartial;
            move |__pe_a0| verifyStatement(&__pe_a0, __pe_b1.clone())
        }),
    )?;
    Ok(())
}

fn verifyStatement(mut stmt: &metamodelica::Ref<Statement::NFStatement>, mut isPartial: bool) -> Result<()> {
    Statement::applyExp(
        stmt,
        &({
            let __pe_b1 = isPartial;
            let __pe_b2 = Statement::info(stmt);
            move |__pe_a0| checkSubscriptBounds(__pe_a0, __pe_b1.clone(), &__pe_b2)
        }),
    )?;
    Ok(())
}

fn checkSubscriptBounds(
    mut exp: metamodelica::Ref<Expression::NFExpression>,
    mut isPartial: bool,
    mut info: &SourceInfo,
) -> Result<()> {
    Expression::apply(
        exp,
        &({
            let __pe_b1 = isPartial;
            let __pe_b2 = info.clone();
            move |__pe_a0| checkSubscriptBounds_traverser(&__pe_a0, __pe_b1.clone(), &__pe_b2)
        }),
    )?;
    Ok(())
}

fn checkSubscriptBounds_traverser(
    mut exp: &metamodelica::Ref<Expression::NFExpression>,
    mut isPartial: bool,
    mut info: &SourceInfo,
) -> Result<()> {
    let () = (match &**exp {
        Expression::CREF { cref: __exp_cref, .. } => {
            checkSubscriptBoundsCref(metamodelica::AsArg::as_arg(&__exp_cref), isPartial, info)?;
            ()
        }
        _ => (),
    });
    Ok(())
}

fn checkSubscriptBoundsCref(
    mut cref: &metamodelica::Ref<ComponentRef::NFComponentRef>,
    mut isPartial: bool,
    mut info: &SourceInfo,
) -> Result<()> {
    let () = (::match_deref::match_deref! { match cref {
        Deref @ ComponentRef::CREF { subscripts: subs @ Deref @ metamodelica::ListNode::Cons { head: _, tail: _ }, ty: Deref @ Type::ARRAY { dimensions: dims, .. }, restCref: __cref_restCref, .. } => {
            let mut d: metamodelica::Ref<Dimension::NFDimension>;
            let mut int_sub: i32;
            let mut index: i32;
            let mut dims = (*dims).clone();
            index = 1;
            for mut s in &*subs.clone() {
                let (__pa0, __pa1) = ::match_deref::match_deref! { match &(dims.clone()) {
                    Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
                    _ => return Err("pattern mismatch"),
                } };
                d = metamodelica::Own::own(__pa0);
                dims = metamodelica::Own::own(__pa1);
                if Subscript::isScalarLiteral(metamodelica::AsArg::as_arg(&s)) && Dimension::isKnown(&d, false) {
                    int_sub = Subscript::toInteger(metamodelica::AsArg::as_arg(&s))?;
                    if int_sub < 1 || int_sub > Dimension::size(&d, false)? {
                        Error::addSourceMessage(&(Error::ARRAY_INDEX_OUT_OF_BOUNDS.clone()), list![Subscript::toString(metamodelica::AsArg::as_arg(&s))?, ArcStr::from(::std::format!("{}", index)), Dimension::toString(&d)?, ComponentRef::firstName(cref, false)?], info)?;
                        if !(isPartial) {
                            return Err("fail");
                        }
                    }
                }
                index = index + 1;
            }
            checkSubscriptBoundsCref(metamodelica::AsArg::as_arg(&__cref_restCref), isPartial, info)?;
            ()
        },
        _ => {
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(())
}

fn checkDiscreteReal(mut flatModel: &metamodelica::Ref<FlatModel::NFFlatModel>) -> Result<()> {
    let mut discrete_reals: metamodelica::Ref<
        UnorderedSet::UnorderedSet<metamodelica::Ref<ComponentRef::NFComponentRef>>,
    >;
    let mut illegal_discrete_vars: metamodelica::List<metamodelica::Ref<Variable::NFVariable>> = metamodelica::nil();
    discrete_reals = UnorderedSet::new(
        (std::sync::Arc::new(move |__a0: metamodelica::Ref<ComponentRef::NFComponentRef>| {
            ComponentRef::hashStrip(&__a0)
        })
            as std::sync::Arc<
                dyn ::std::ops::Fn(metamodelica::Ref<ComponentRef::NFComponentRef>) -> Result<i32> + 'static,
            >),
        (std::sync::Arc::new(
            move |__a0: metamodelica::Ref<ComponentRef::NFComponentRef>,
                  __a1: metamodelica::Ref<ComponentRef::NFComponentRef>| {
                ComponentRef::isEqualStrip(&__a0, &__a1)
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
    for mut eqn in &*flatModel.equations.clone() {
        checkDiscreteRealEquation(metamodelica::AsArg::as_arg(&eqn), discrete_reals.clone(), false)?;
    }
    for mut alg in &*flatModel.algorithms.clone() {
        for mut statement in &*alg.statements.clone() {
            checkDiscreteRealStatement(statement.clone(), discrete_reals.clone(), false)?;
        }
    }
    for mut variable in &*flatModel.variables.clone() {
        if Variable::variability(metamodelica::AsArg::as_arg(&variable)) == Variability::DISCRETE.clone()
            && Type::isReal(&(Type::arrayElementType(&variable.ty)))?
            && !(UnorderedSet::contains(variable.name.clone(), discrete_reals.clone())?)
        {
            illegal_discrete_vars = metamodelica::cons(variable.clone(), illegal_discrete_vars);
        }
    }
    if !((illegal_discrete_vars).is_empty()) {
        for mut var in &*illegal_discrete_vars {
            Error::addSourceMessage(
                &(Error::DISCRETE_REAL_UNDEFINED.clone()),
                list![ComponentRef::toString(&(ComponentRef::stripSubscriptsAll(&var.name)))?],
                &var.info,
            )?;
        }
        return Err("fail");
    }
    Ok(())
}

fn checkDiscreteRealBranch(
    mut branch: &metamodelica::Ref<Equation::Branch::Branch>,
    mut discreteReals: metamodelica::Ref<UnorderedSet::UnorderedSet<metamodelica::Ref<ComponentRef::NFComponentRef>>>,
    mut when_found: bool,
) -> Result<metamodelica::Ref<UnorderedSet::UnorderedSet<metamodelica::Ref<ComponentRef::NFComponentRef>>>> {
    let mut discreteReals: metamodelica::Ref<
        UnorderedSet::UnorderedSet<metamodelica::Ref<ComponentRef::NFComponentRef>>,
    > = discreteReals;
    let () = (match &**branch {
        Equation::Branch::BRANCH {
            body: __branch_body, ..
        } if (when_found) => {
            for mut eqn in &*__branch_body.clone() {
                checkDiscreteRealEquation(metamodelica::AsArg::as_arg(&eqn), discreteReals.clone(), when_found)?;
            }
            ()
        }
        _ => (),
    });
    Ok(discreteReals)
}

fn checkDiscreteRealEquation(
    mut body_eqn: &metamodelica::Ref<Equation::NFEquation>,
    mut discreteReals: metamodelica::Ref<UnorderedSet::UnorderedSet<metamodelica::Ref<ComponentRef::NFComponentRef>>>,
    mut when_found: bool,
) -> Result<()> {
    let () = (match &**body_eqn {
        Equation::EQUALITY { lhs, .. } if (when_found) => {
            checkDiscreteRealExp(lhs, discreteReals)?;
            ()
        }
        Equation::IF { branches, .. } => {
            for mut branch in &*branches.clone() {
                checkDiscreteRealBranch(metamodelica::AsArg::as_arg(&branch), discreteReals.clone(), when_found)?;
            }
            ()
        }
        Equation::WHEN { branches, .. } => {
            for mut branch in &*branches.clone() {
                checkDiscreteRealBranch(metamodelica::AsArg::as_arg(&branch), discreteReals.clone(), true)?;
            }
            ()
        }
        Equation::FOR { body, .. } => {
            for mut eqn in &*body.clone() {
                checkDiscreteRealEquation(metamodelica::AsArg::as_arg(&eqn), discreteReals.clone(), when_found)?;
            }
            ()
        }
        _ => (),
    });
    Ok(())
}

fn checkDiscreteRealStatement(
    mut statement: metamodelica::Ref<Statement::NFStatement>,
    mut discreteReals: metamodelica::Ref<UnorderedSet::UnorderedSet<metamodelica::Ref<ComponentRef::NFComponentRef>>>,
    mut when_found: bool,
) -> Result<()> {
    let () = (match &*statement {
        Statement::WHEN { branches, .. } => {
            let mut body: metamodelica::List<metamodelica::Ref<Statement::NFStatement>>;
            for mut branch in &*branches.clone() {
                (_, body) = branch.clone();
                for mut statement in &*body {
                    let mut statement = statement.clone();
                    checkDiscreteRealStatement(statement, discreteReals.clone(), true)?;
                }
            }
            ()
        }
        Statement::ASSIGNMENT { lhs, .. } if (when_found) => {
            checkDiscreteRealExp(metamodelica::AsArg::as_arg(&lhs), discreteReals)?;
            ()
        }
        Statement::IF { branches, .. } => {
            let mut body: metamodelica::List<metamodelica::Ref<Statement::NFStatement>>;
            for mut branch in &*branches.clone() {
                (_, body) = branch.clone();
                for mut stmt in &*body {
                    checkDiscreteRealStatement(stmt.clone(), discreteReals.clone(), when_found)?;
                }
            }
            ()
        }
        Statement::FOR { body, .. } => {
            for mut statement in &*body.clone() {
                let mut statement = statement.clone();
                checkDiscreteRealStatement(statement, discreteReals.clone(), when_found)?;
            }
            ()
        }
        _ => (),
    });
    Ok(())
}

fn checkDiscreteRealExp(
    mut exp: &metamodelica::Ref<Expression::NFExpression>,
    mut discreteReals: metamodelica::Ref<UnorderedSet::UnorderedSet<metamodelica::Ref<ComponentRef::NFComponentRef>>>,
) -> Result<()> {
    let () = (::match_deref::match_deref! { match exp {
        Deref @ Expression::CREF { ty, cref } if (Type::isReal(&(Type::arrayElementType(ty)))?) => {
            UnorderedSet::add(cref.clone(), discreteReals)?;
            ()
        },
        Deref @ Expression::CREF { ty: ty @ Deref @ Type::COMPLEX { .. }, cref } if (Type::isRecord(metamodelica::AsArg::as_arg(&ty))) => {
            checkDiscreteRealRecord(cref.clone(), Type::complexNode(metamodelica::AsArg::as_arg(&ty))?, discreteReals)?;
            ()
        },
        Deref @ Expression::TUPLE { elements, .. } => {
            for mut element in &*elements.clone() {
                checkDiscreteRealExp(metamodelica::AsArg::as_arg(&element), discreteReals.clone())?;
            }
            ()
        },
        _ => {
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(())
}

fn checkDiscreteRealRecord(
    mut cref: metamodelica::Ref<ComponentRef::NFComponentRef>,
    mut cls: metamodelica::Ref<InstNode::InstNode>,
    mut discreteReals: metamodelica::Ref<UnorderedSet::UnorderedSet<metamodelica::Ref<ComponentRef::NFComponentRef>>>,
) -> Result<()> {
    let mut element: metamodelica::Ref<ComponentRef::NFComponentRef>;
    let mut inputs: metamodelica::List<metamodelica::Ref<InstNode::InstNode>>;
    UnorderedSet::add(cref.clone(), discreteReals.clone())?;
    (inputs, _, _) = Record::collectRecordParams(cls)?;
    for mut node in &*inputs {
        element = ComponentRef::prefixCref(
            node.clone(),
            InstNode::getType(node.clone())?,
            metamodelica::nil(),
            cref.clone(),
        )?;
        UnorderedSet::add(element, discreteReals.clone())?;
    }
    Ok(())
}
