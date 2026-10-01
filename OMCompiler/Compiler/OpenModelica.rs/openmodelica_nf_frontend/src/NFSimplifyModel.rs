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
use crate::NFCeval as Ceval;
use crate::NFClass as Class;
use crate::NFComponentRef as ComponentRef;
use crate::NFDimension as Dimension;
use crate::NFEquation as Equation;
use crate::NFExpression as Expression;
use crate::NFFlatModel as FlatModel;
use crate::NFFlatten::FunctionTree;
use crate::NFFunction::Function;
use crate::NFInstNode;
use crate::NFInstNode::InstNode;
use crate::NFPrefixes::Variability;
use crate::NFSections as Sections;
use crate::NFSimplifyExp as SimplifyExp;
use crate::NFStatement as Statement;
use crate::NFSubscript as Subscript;
use crate::NFType as Type;
use crate::NFVariable as Variable;
use openmodelica_frontend_types::DAE;
use openmodelica_util::ExecStat::execStat;
use openmodelica_util::Flags;
use openmodelica_util::Util;
use openmodelica_util_datatypes_basic::List;
use openmodelica_util_datatypes_basic::MutableWeak;

pub fn simplify(
    mut flatModel: metamodelica::Ref<FlatModel::NFFlatModel>,
) -> Result<metamodelica::Ref<FlatModel::NFFlatModel>> {
    let mut flatModel: metamodelica::Ref<FlatModel::NFFlatModel> = flatModel;
    assign_field!(
        flatModel.variables = ({
            let mut __acc: metamodelica::List<metamodelica::Ref<Variable::NFVariable>> = metamodelica::nil();
            for mut v in (flatModel.variables.clone()).into_iter().cloned() {
                let __x = simplifyVariable(v.clone())?;
                __acc = cons(__x, __acc);
            }
            __acc.reverse()
        }),
        flatModel.equations = simplifyEquations(&flatModel.equations)?,
        flatModel.initialEquations = simplifyEquations(&flatModel.initialEquations)?,
        flatModel.algorithms = simplifyAlgorithms(&flatModel.algorithms)?,
        flatModel.initialAlgorithms = simplifyAlgorithms(&flatModel.initialAlgorithms)?
    );
    execStat(&(literal!("NFSimplifyModel.simplify")))?;
    Ok(flatModel)
}

pub(crate) fn simplifyVariable(
    mut var: metamodelica::Ref<Variable::NFVariable>,
) -> Result<metamodelica::Ref<Variable::NFVariable>> {
    let mut var: metamodelica::Ref<Variable::NFVariable> = var;
    assign_field!(
        var.binding = simplifyBinding(var.binding.clone())?,
        var.typeAttributes = ({
            let mut __acc: metamodelica::List<(ArcStr, metamodelica::Ref<Binding::NFBinding>)> = metamodelica::nil();
            for mut a in (var.typeAttributes.clone()).into_iter().cloned() {
                let __x = simplifyTypeAttribute(a.clone())?;
                __acc = cons(__x, __acc);
            }
            __acc.reverse()
        }),
        var.children = ({
            let mut __acc: metamodelica::List<metamodelica::Ref<Variable::NFVariable>> = metamodelica::nil();
            for mut v in (var.children.clone()).into_iter().cloned() {
                let __x = simplifyVariable(v.clone())?;
                __acc = cons(__x, __acc);
            }
            __acc.reverse()
        })
    );
    Ok(var)
}

pub(crate) fn simplifyBinding(
    mut binding: metamodelica::Ref<Binding::NFBinding>,
) -> Result<metamodelica::Ref<Binding::NFBinding>> {
    let mut binding: metamodelica::Ref<Binding::NFBinding> = binding;
    let mut exp: metamodelica::Ref<Expression::NFExpression>;
    let mut sexp: metamodelica::Ref<Expression::NFExpression>;
    if Binding::isBound(&binding) {
        exp = Binding::getTypedExp(&binding)?;
        sexp = SimplifyExp::simplify(exp.clone(), false)?;
        sexp = removeEmptyFunctionArguments(sexp, false)?;
        if !(referenceEq(&*(exp), &*(&*sexp))) {
            binding = Binding::setTypedExp(sexp, binding)?;
        }
    }
    Ok(binding)
}

pub(crate) fn simplifyTypeAttribute(
    mut attribute: (ArcStr, metamodelica::Ref<Binding::NFBinding>),
) -> Result<(ArcStr, metamodelica::Ref<Binding::NFBinding>)> {
    let mut attribute: (ArcStr, metamodelica::Ref<Binding::NFBinding>) = attribute;
    let mut name: ArcStr;
    let mut binding: metamodelica::Ref<Binding::NFBinding>;
    let mut sbinding: metamodelica::Ref<Binding::NFBinding>;
    (name, binding) = attribute.clone();
    sbinding = simplifyBinding(binding.clone())?;
    if !(referenceEq(&*(binding), &*(&*sbinding))) {
        attribute = (name, sbinding);
    }
    Ok(attribute)
}

pub(crate) fn simplifyDimension(
    mut dim: metamodelica::Ref<Dimension::NFDimension>,
) -> Result<metamodelica::Ref<Dimension::NFDimension>> {
    let mut outDim: metamodelica::Ref<Dimension::NFDimension>;
    outDim = (match &*dim {
        Dimension::EXP {
            exp: __dim_exp,
            var: __dim_var,
        } => {
            let mut e: metamodelica::Ref<Expression::NFExpression>;
            e = SimplifyExp::simplify(__dim_exp.clone(), false)?;
            if (referenceEq(&*(&*e), &*(__dim_exp.clone()))) {
                dim
            } else {
                Dimension::fromExp(e, __dim_var.clone())?
            }
        }
        _ => dim,
    });
    Ok(outDim)
}

pub(crate) fn simplifyEquations(
    mut eql: &metamodelica::List<metamodelica::Ref<Equation::NFEquation>>,
) -> Result<metamodelica::List<metamodelica::Ref<Equation::NFEquation>>> {
    let mut outEql: metamodelica::List<metamodelica::Ref<Equation::NFEquation>> = metamodelica::nil();
    for mut eq in &**eql {
        outEql = simplifyEquation(eq.clone(), outEql)?;
    }
    outEql = metamodelica::Dangerous::listReverseInPlace(outEql);
    Ok(outEql)
}

pub(crate) fn simplifyEquation(
    mut eq: metamodelica::Ref<Equation::NFEquation>,
    mut equations: metamodelica::List<metamodelica::Ref<Equation::NFEquation>>,
) -> Result<metamodelica::List<metamodelica::Ref<Equation::NFEquation>>> {
    let mut equations: metamodelica::List<metamodelica::Ref<Equation::NFEquation>> = equations;
    equations = (::match_deref::match_deref! { match &(eq.clone()) {
        Deref @ Equation::EQUALITY { .. } => {
            simplifyEqualityEquation(&eq, equations)?
        },
        Deref @ Equation::FOR { range: Some(e), .. } => {
            let mut body: metamodelica::List<metamodelica::Ref<Equation::NFEquation>>;
            let mut dim: metamodelica::Ref<Dimension::NFDimension>;
            let mut e = (*e).clone();
            dim = Type::nthDimension(Expression::typeOf(e.clone()), 1)?;
            if Dimension::isZero(&dim)? {
            } else if Dimension::isOne(&dim)? && Flags::getConfigBool(Flags::NEW_BACKEND.clone())? {
                e = Expression::applySubscript(&(metamodelica::Ref::new(Subscript::NFSubscript::INDEX { index: metamodelica::Ref::new(Expression::NFExpression::INTEGER { value: 1 }) })), metamodelica::AsArg::as_arg(&e), &(metamodelica::nil()), false)?;
                e = SimplifyExp::simplify(e.clone(), false)?;
                body = Equation::replaceIteratorList(var_field!((*eq).body, Equation::NFEquation::FOR).clone(), &(var_field!((*eq).iterator, Equation::NFEquation::FOR).clone()), metamodelica::AsArg::as_arg(&e))?;
                body = simplifyEquations(&body)?;
                equations = List::append_reverse(&body, equations);
            } else {
                assign_variant_field!(eq => Equation::NFEquation::FOR;
                    range = Util::applyOption(var_field!((*eq).range, Equation::NFEquation::FOR).clone(), &({ let __pe_b1 = false; move |__pe_a0| SimplifyExp::simplify(__pe_a0, __pe_b1.clone()) }))?,
                    body = simplifyEquations(var_field!((*eq).body, Equation::NFEquation::FOR))?
                );
                equations = metamodelica::cons(eq, equations);
            }
            equations
        },
        Deref @ Equation::IF { branches: __eq_branches, scope: __eq_scope, source: __eq_source } => {
            simplifyIfEqBranches(metamodelica::AsArg::as_arg(&__eq_branches), __eq_scope.clone(), __eq_source.clone(), equations)?
        },
        Deref @ Equation::WHEN { branches: __eq_branches, .. } => {
            assign_variant_field!(eq => Equation::NFEquation::WHEN; branches = ({
        let mut __acc: metamodelica::List<metamodelica::Ref<Equation::Branch::Branch>> = metamodelica::nil();
        for mut b in (__eq_branches.clone()).into_iter().cloned() {
            let __x = (match &*b.clone() {
        Equation::Branch::BRANCH { condition: __b_condition, .. } => {
            assign_variant_field!(b => Equation::Branch::Branch::BRANCH;
                condition = SimplifyExp::simplify(__b_condition.clone(), false)?,
                body = simplifyEquations(&(var_field!((*b).body, Equation::Branch::Branch::BRANCH).clone()))?
            );
            b.clone()
        },
        _ => return Err("match: no arm matched"),
    });
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }));
            metamodelica::cons(eq, equations)
        },
        Deref @ Equation::ASSERT { condition: __eq_condition, .. } => {
            assign_variant_field!(eq => Equation::NFEquation::ASSERT; condition = SimplifyExp::simplify(__eq_condition.clone(), false)?);
            if (Expression::isTrue(var_field!((*eq).condition, Equation::NFEquation::ASSERT))) {equations} else {metamodelica::cons(eq, equations)}
        },
        Deref @ Equation::REINIT { reinitExp: __eq_reinitExp, .. } => {
            assign_variant_field!(eq => Equation::NFEquation::REINIT; reinitExp = SimplifyExp::simplify(__eq_reinitExp.clone(), false)?);
            metamodelica::cons(eq, equations)
        },
        Deref @ Equation::NORETCALL { exp: __eq_exp, .. } => {
            let mut e: metamodelica::Ref<Expression::NFExpression>;
            e = SimplifyExp::simplify(__eq_exp.clone(), false)?;
            if Expression::isCall(&e) {
                assign_variant_field!(eq => Equation::NFEquation::NORETCALL; exp = removeEmptyFunctionArguments(e, false)?);
                equations = metamodelica::cons(eq, equations);
            }
            equations
        },
        _ => {
            metamodelica::cons(eq, equations)
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(equations)
}

pub(crate) fn simplifyEqualityEquation(
    mut eq: &metamodelica::Ref<Equation::NFEquation>,
    mut equations: metamodelica::List<metamodelica::Ref<Equation::NFEquation>>,
) -> Result<metamodelica::List<metamodelica::Ref<Equation::NFEquation>>> {
    let mut equations: metamodelica::List<metamodelica::Ref<Equation::NFEquation>> = equations;
    let mut lhs: metamodelica::Ref<Expression::NFExpression>;
    let mut rhs: metamodelica::Ref<Expression::NFExpression>;
    let mut ty: metamodelica::Ref<Type::NFType>;
    let mut src: metamodelica::Ref<DAE::ElementSource>;
    let mut scope: Option<MutableWeak::MutableWeak<metamodelica::Ref<InstNode::InstNode>>>;
    let mut scalarize_mode: Equation::ScalarizeMode;
    let (__pa0, __pa1, __pa2, __pa3, __pa4, __pa5) = ::match_deref::match_deref! { match &((*eq)) {
        Deref @ Equation::EQUALITY { lhs: __pa0, rhs: __pa1, ty: __pa2, scope: __pa3, source: __pa4, scalarizeMode: __pa5 } => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone(), __pa4.clone(), __pa5.clone()),
        _ => return Err("pattern mismatch"),
    } };
    lhs = metamodelica::Own::own(__pa0);
    rhs = metamodelica::Own::own(__pa1);
    ty = metamodelica::Own::own(__pa2);
    scope = metamodelica::Own::own(__pa3);
    src = metamodelica::Own::own(__pa4);
    scalarize_mode = metamodelica::Own::own(__pa5);
    ty = Type::mapDims(ty, &simplifyDimension)?;
    if Type::isEmptyArray(&ty)? {
        return Ok(equations);
    }
    lhs = SimplifyExp::simplify(lhs, false)?;
    lhs = removeEmptyTupleElements(lhs)?;
    rhs = SimplifyExp::simplify(rhs, false)?;
    rhs = removeEmptyFunctionArguments(rhs, false)?;
    equations = (::match_deref::match_deref! { match &((lhs.clone(), rhs.clone())) {
        (Deref @ Expression::TUPLE { .. }, Deref @ Expression::TUPLE { .. }) => simplifyTupleElement(var_field!((*lhs).elements, Expression::NFExpression::TUPLE), var_field!((*rhs).elements, Expression::NFExpression::TUPLE).clone(), &ty, src, &({ let __pe_b4 = NFInstNode::InstNode::fromCell(scope)?; let __pe_b5 = scalarize_mode; move |__pe_a0, __pe_a1, __pe_a2, __pe_a3| Ok(Equation::makeEquality(__pe_a0, __pe_a1, __pe_a2, __pe_a3, __pe_b4.clone(), __pe_b5.clone())) }), equations)?,
        _ => metamodelica::cons(metamodelica::Ref::new(Equation::NFEquation::EQUALITY { lhs: lhs, rhs: rhs, ty: ty, scope: scope, source: src, scalarizeMode: scalarize_mode }), equations),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(equations)
}

pub(crate) fn simplifyAlgorithms(
    mut algs: &metamodelica::List<metamodelica::Ref<Algorithm::NFAlgorithm>>,
) -> Result<metamodelica::List<metamodelica::Ref<Algorithm::NFAlgorithm>>> {
    let mut outAlgs: metamodelica::List<metamodelica::Ref<Algorithm::NFAlgorithm>> = metamodelica::nil();
    for mut alg in &**algs {
        let mut alg = alg.clone();
        alg = simplifyAlgorithm(alg)?;
        if !((alg.statements).is_empty()) {
            outAlgs = metamodelica::cons(alg, outAlgs);
        }
    }
    outAlgs = metamodelica::Dangerous::listReverseInPlace(outAlgs);
    Ok(outAlgs)
}

pub fn simplifyAlgorithm(
    mut alg: metamodelica::Ref<Algorithm::NFAlgorithm>,
) -> Result<metamodelica::Ref<Algorithm::NFAlgorithm>> {
    let mut alg: metamodelica::Ref<Algorithm::NFAlgorithm> = alg;
    assign_field!(alg.statements = simplifyStatements(&alg.statements)?);
    Ok(alg)
}

pub(crate) fn simplifyStatements(
    mut stmts: &metamodelica::List<metamodelica::Ref<Statement::NFStatement>>,
) -> Result<metamodelica::List<metamodelica::Ref<Statement::NFStatement>>> {
    let mut outStmts: metamodelica::List<metamodelica::Ref<Statement::NFStatement>> = metamodelica::nil();
    for mut s in &**stmts {
        outStmts = simplifyStatement(s.clone(), outStmts)?;
    }
    outStmts = metamodelica::Dangerous::listReverseInPlace(outStmts);
    Ok(outStmts)
}

pub(crate) fn simplifyStatement(
    mut stmt: metamodelica::Ref<Statement::NFStatement>,
    mut statements: metamodelica::List<metamodelica::Ref<Statement::NFStatement>>,
) -> Result<metamodelica::List<metamodelica::Ref<Statement::NFStatement>>> {
    let mut statements: metamodelica::List<metamodelica::Ref<Statement::NFStatement>> = statements;
    statements = (::match_deref::match_deref! { match &(stmt.clone()) {
        Deref @ Statement::ASSIGNMENT { .. } => {
            simplifyAssignment(&stmt, statements)?
        },
        Deref @ Statement::FOR { body: Deref @ metamodelica::ListNode::Nil, .. } => {
            statements
        },
        Deref @ Statement::FOR { range: Some(e), .. } => {
            let mut dim: metamodelica::Ref<Dimension::NFDimension>;
            dim = Type::nthDimension(Expression::typeOf(e.clone()), 1)?;
            if !(Dimension::isZero(&dim)?) {
                assign_variant_field!(stmt => Statement::NFStatement::FOR;
                    range = Some(SimplifyExp::simplify(e.clone(), false)?),
                    body = simplifyStatements(var_field!((*stmt).body, Statement::NFStatement::FOR))?
                );
                statements = metamodelica::cons(stmt, statements);
            }
            statements
        },
        Deref @ Statement::IF { branches: __stmt_branches, source: __stmt_source } => {
            simplifyIfStmtBranches(metamodelica::AsArg::as_arg(&__stmt_branches), __stmt_source.clone(), &fnptr!(Statement::makeIf, metamodelica::List<(metamodelica::Ref<Expression::NFExpression>, metamodelica::List<metamodelica::Ref<Statement::NFStatement>>)>, metamodelica::Ref<DAE::ElementSource>), &move |__a0: metamodelica::List<metamodelica::Ref<Statement::NFStatement>>| simplifyStatements(&__a0), statements)?
        },
        Deref @ Statement::WHEN { branches: __stmt_branches, .. } => {
            assign_variant_field!(stmt => Statement::NFStatement::WHEN; branches = simplifyWhenBranches(__stmt_branches.clone())?);
            if ((var_field!((*stmt).branches, Statement::NFStatement::WHEN)).is_empty()) {statements} else {metamodelica::cons(stmt, statements)}
        },
        Deref @ Statement::ASSERT { condition: __stmt_condition, .. } => {
            assign_variant_field!(stmt => Statement::NFStatement::ASSERT;
                condition = SimplifyExp::simplify(__stmt_condition.clone(), false)?,
                message = SimplifyExp::simplify(var_field!((*stmt).message, Statement::NFStatement::ASSERT).clone(), false)?,
                level = SimplifyExp::simplify(var_field!((*stmt).level, Statement::NFStatement::ASSERT).clone(), false)?
            );
            metamodelica::cons(stmt, statements)
        },
        Deref @ Statement::TERMINATE { message: __stmt_message, .. } => {
            assign_variant_field!(stmt => Statement::NFStatement::TERMINATE; message = SimplifyExp::simplify(__stmt_message.clone(), false)?);
            metamodelica::cons(stmt, statements)
        },
        Deref @ Statement::WHILE { condition: __stmt_condition, .. } => {
            assign_variant_field!(stmt => Statement::NFStatement::WHILE;
                condition = SimplifyExp::simplify(__stmt_condition.clone(), false)?,
                body = simplifyStatements(var_field!((*stmt).body, Statement::NFStatement::WHILE))?
            );
            metamodelica::cons(stmt, statements)
        },
        Deref @ Statement::NORETCALL { exp: __stmt_exp, .. } => {
            let mut e: metamodelica::Ref<Expression::NFExpression>;
            e = SimplifyExp::simplify(__stmt_exp.clone(), false)?;
            if Expression::isCall(&e) {
                assign_variant_field!(stmt => Statement::NFStatement::NORETCALL; exp = removeEmptyFunctionArguments(e, false)?);
                statements = metamodelica::cons(stmt, statements);
            }
            statements
        },
        _ => {
            metamodelica::cons(stmt, statements)
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(statements)
}

pub(crate) fn simplifyWhenBranches(
    mut branches: metamodelica::List<(
        metamodelica::Ref<Expression::NFExpression>,
        metamodelica::List<metamodelica::Ref<Statement::NFStatement>>,
    )>,
) -> Result<
    metamodelica::List<(
        metamodelica::Ref<Expression::NFExpression>,
        metamodelica::List<metamodelica::Ref<Statement::NFStatement>>,
    )>,
> {
    '__tco: loop {
        ::match_deref::match_deref! { match &(branches.clone()) {
            Deref @ metamodelica::ListNode::Cons { head: (condition, body), tail: tail } => {
                let mut condition = (*condition).clone();
                let mut body = (*body).clone();
                condition = SimplifyExp::simplify(condition.clone(), false)?;
                body = simplifyStatements(metamodelica::AsArg::as_arg(&body))?;
                if (Expression::isBoolean(metamodelica::AsArg::as_arg(&condition))) {{ branches = tail.clone(); continue '__tco; }} else {return Ok(metamodelica::cons((condition.clone(), body.clone()), simplifyWhenBranches(tail.clone())?))}
            },
            _ => {
                return Ok(branches)
            },
            _ => return Err("match: no arm matched"),
        } }
    }
}

pub(crate) fn simplifyAssignment(
    mut stmt: &metamodelica::Ref<Statement::NFStatement>,
    mut statements: metamodelica::List<metamodelica::Ref<Statement::NFStatement>>,
) -> Result<metamodelica::List<metamodelica::Ref<Statement::NFStatement>>> {
    let mut statements: metamodelica::List<metamodelica::Ref<Statement::NFStatement>> = statements;
    let mut lhs: metamodelica::Ref<Expression::NFExpression>;
    let mut rhs: metamodelica::Ref<Expression::NFExpression>;
    let mut ty: metamodelica::Ref<Type::NFType>;
    let mut src: metamodelica::Ref<DAE::ElementSource>;
    let (__pa0, __pa1, __pa2, __pa3) = ::match_deref::match_deref! { match &((*stmt)) {
        Deref @ Statement::ASSIGNMENT { lhs: __pa0, rhs: __pa1, ty: __pa2, source: __pa3 } => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone()),
        _ => return Err("pattern mismatch"),
    } };
    lhs = metamodelica::Own::own(__pa0);
    rhs = metamodelica::Own::own(__pa1);
    ty = metamodelica::Own::own(__pa2);
    src = metamodelica::Own::own(__pa3);
    ty = Type::mapDims(ty, &simplifyDimension)?;
    if Type::isEmptyArray(&ty)? {
        return Ok(statements);
    }
    lhs = SimplifyExp::simplify(lhs, false)?;
    lhs = removeEmptyTupleElements(lhs)?;
    rhs = SimplifyExp::simplify(rhs, false)?;
    rhs = removeEmptyFunctionArguments(rhs, false)?;
    statements = (::match_deref::match_deref! { match &((lhs.clone(), rhs.clone())) {
        (Deref @ Expression::TUPLE { .. }, Deref @ Expression::TUPLE { .. }) => simplifyTupleElement(var_field!((*lhs).elements, Expression::NFExpression::TUPLE), var_field!((*rhs).elements, Expression::NFExpression::TUPLE).clone(), &ty, src, &fnptr!(Statement::makeAssignment, metamodelica::Ref<Expression::NFExpression>, metamodelica::Ref<Expression::NFExpression>, metamodelica::Ref<Type::NFType>, metamodelica::Ref<DAE::ElementSource>), statements)?,
        _ => metamodelica::cons(metamodelica::Ref::new(Statement::NFStatement::ASSIGNMENT { lhs: lhs, rhs: rhs, ty: ty, source: src }), statements),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(statements)
}

pub(crate) fn simplifyTupleElement<ElementT: Clone + 'static + metamodelica::gc::MMTrace>(
    mut lhsTuple: &metamodelica::List<metamodelica::Ref<Expression::NFExpression>>,
    mut rhsTuple: metamodelica::List<metamodelica::Ref<Expression::NFExpression>>,
    mut ty: &metamodelica::Ref<Type::NFType>,
    mut src: metamodelica::Ref<DAE::ElementSource>,
    mut makeFn: &dyn ::std::ops::Fn(
        metamodelica::Ref<Expression::NFExpression>,
        metamodelica::Ref<Expression::NFExpression>,
        metamodelica::Ref<Type::NFType>,
        metamodelica::Ref<DAE::ElementSource>,
    ) -> Result<ElementT>,
    mut statements: metamodelica::List<ElementT>,
) -> Result<metamodelica::List<ElementT>> {
    pub type MakeElement<ElementT: Clone + 'static> = std::sync::Arc<
        dyn ::std::ops::Fn(
                metamodelica::Ref<Expression::NFExpression>,
                metamodelica::Ref<Expression::NFExpression>,
                metamodelica::Ref<Type::NFType>,
                metamodelica::Ref<DAE::ElementSource>,
            ) -> Result<ElementT>
            + 'static,
    >;

    let mut statements: metamodelica::List<ElementT> = statements;
    let mut rhs: metamodelica::Ref<Expression::NFExpression>;
    let mut rest_rhs: metamodelica::List<metamodelica::Ref<Expression::NFExpression>> = rhsTuple;
    let mut ety: metamodelica::Ref<Type::NFType>;
    let mut rest_ty: metamodelica::List<metamodelica::Ref<Type::NFType>>;
    let __pa0 = ::match_deref::match_deref! { match &((*ty)) {
        Deref @ Type::TUPLE { types: __pa0, .. } => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    rest_ty = metamodelica::Own::own(__pa0);
    for mut lhs in &**lhsTuple {
        let (__pa1, __pa2) = ::match_deref::match_deref! { match &(rest_rhs) {
            Deref @ metamodelica::ListNode::Cons { head: __pa1, tail: __pa2 } => (__pa1.clone(), __pa2.clone()),
            _ => return Err("pattern mismatch"),
        } };
        rhs = metamodelica::Own::own(__pa1);
        rest_rhs = metamodelica::Own::own(__pa2);
        let (__pa3, __pa4) = ::match_deref::match_deref! { match &(rest_ty) {
            Deref @ metamodelica::ListNode::Cons { head: __pa3, tail: __pa4 } => (__pa3.clone(), __pa4.clone()),
            _ => return Err("pattern mismatch"),
        } };
        ety = metamodelica::Own::own(__pa3);
        rest_ty = metamodelica::Own::own(__pa4);
        if !(Expression::isWildCref(metamodelica::AsArg::as_arg(&lhs))) {
            statements = metamodelica::cons(makeFn(lhs.clone(), rhs, ety, src.clone())?, statements);
        }
    }
    Ok(statements)
}

pub(crate) fn removeEmptyTupleElements(
    mut exp: metamodelica::Ref<Expression::NFExpression>,
) -> Result<metamodelica::Ref<Expression::NFExpression>> {
    let mut exp: metamodelica::Ref<Expression::NFExpression> = exp;
    let () = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ Expression::TUPLE { ty: Deref @ Type::TUPLE { types: tyl, .. }, elements: __exp_elements } => {
            assign_variant_field!(exp => Expression::NFExpression::TUPLE; elements = ({
        let mut __acc: metamodelica::List<metamodelica::Ref<Expression::NFExpression>> = metamodelica::nil();
        let __thr_src0 = __exp_elements.clone();
        let mut __thr_it0 = (&__thr_src0).into_iter();
        let __thr_src1 = tyl.clone();
        let mut __thr_it1 = (&__thr_src1).into_iter();
        loop {
            match (__thr_it0.next(), __thr_it1.next()) {
                (Some(e), Some(t)) => {
                    let __x = if (Type::isEmptyArray(&(t.clone()))?) {metamodelica::Ref::new(Expression::NFExpression::CREF { ty: t.clone(), cref: crate::NFComponentRef::interned_WILD() })} else {e.clone()};
                    __acc = cons(__x, __acc);
                }
                (None, None) => break,
                _ => return Err("threaded for: ranges of unequal length"),
            }
        }
        __acc.reverse()
    }));
            ()
        },
        _ => {
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(exp)
}

pub(crate) fn removeEmptyFunctionArguments(
    mut exp: metamodelica::Ref<Expression::NFExpression>,
    mut isArg: bool,
) -> Result<metamodelica::Ref<Expression::NFExpression>> {
    let mut outExp: metamodelica::Ref<Expression::NFExpression>;
    let mut is_arg: bool;
    if isArg {
        let () = (match &*exp {
            Expression::CREF { ty: __exp_ty, .. } if (Type::isEmptyArray(metamodelica::AsArg::as_arg(&__exp_ty))?) => {
                outExp = Expression::fillType(
                    __exp_ty.clone(),
                    metamodelica::Ref::new(Expression::NFExpression::INTEGER { value: 0 }),
                )?;
                return Ok(outExp);
                ()
            }
            _ => (),
        });
    }
    is_arg = isArg || Expression::isCall(&exp);
    outExp = Expression::mapShallow(
        exp,
        (std::sync::Arc::new({
            let __pe_b1 = is_arg;
            move |__pe_a0| removeEmptyFunctionArguments(__pe_a0, __pe_b1.clone())
        })
            as std::sync::Arc<
                dyn ::std::ops::Fn(
                        metamodelica::Ref<Expression::NFExpression>,
                    ) -> Result<metamodelica::Ref<Expression::NFExpression>>
                    + 'static,
            >),
    )?;
    Ok(outExp)
}

pub(crate) fn simplifyIfEqBranches(
    mut branches: &metamodelica::List<metamodelica::Ref<Equation::Branch::Branch>>,
    mut scope: Option<MutableWeak::MutableWeak<metamodelica::Ref<InstNode::InstNode>>>,
    mut src: metamodelica::Ref<DAE::ElementSource>,
    mut elements: metamodelica::List<metamodelica::Ref<Equation::NFEquation>>,
) -> Result<metamodelica::List<metamodelica::Ref<Equation::NFEquation>>> {
    let mut elements: metamodelica::List<metamodelica::Ref<Equation::NFEquation>> = elements;
    let mut cond: metamodelica::Ref<Expression::NFExpression>;
    let mut body: metamodelica::List<metamodelica::Ref<Equation::NFEquation>>;
    let mut var: Variability;
    let mut accum: metamodelica::List<metamodelica::Ref<Equation::Branch::Branch>> = metamodelica::nil();
    for mut branch in &**branches {
        accum = (::match_deref::match_deref! { match &(branch.clone()) {
            Deref @ Equation::Branch::BRANCH { condition: __esc_cond, conditionVar: __esc_var, body: __esc_body } => {
                cond = (*__esc_cond).clone();
                var = (*__esc_var).clone();
                body = (*__esc_body).clone();
                cond = SimplifyExp::simplify(cond.clone(), false)?;
                if Expression::isTrue(metamodelica::AsArg::as_arg(&cond)) {
                    if (accum).is_empty() {
                        for mut eq in &*body.clone() {
                            elements = simplifyEquation(eq.clone(), elements)?;
                        }
                        return Ok(elements);
                    } else {
                        accum = metamodelica::cons(Equation::makeBranch(cond.clone(), simplifyEquations(metamodelica::AsArg::as_arg(&body))?, Variability::CONTINUOUS.clone()), accum);
                        accum = List::trim(accum, &move |__a0: metamodelica::Ref<Equation::Branch::Branch>| -> metamodelica::Result<_> { ::std::result::Result::Ok(Equation::Branch::isEmpty(&__a0)) })?;
                        elements = metamodelica::cons(Equation::makeIf(metamodelica::Dangerous::listReverseInPlace(accum), NFInstNode::InstNode::fromCell(scope)?, src), elements);
                        return Ok(elements);
                    }
                } else if !(Expression::isFalse(metamodelica::AsArg::as_arg(&cond))) {
                    accum = metamodelica::cons(Equation::makeBranch(cond.clone(), simplifyEquations(metamodelica::AsArg::as_arg(&body))?, Variability::CONTINUOUS.clone()), accum);
                }
                accum
            },
            Deref @ Equation::Branch::INVALID_BRANCH { branch: Deref @ Equation::Branch::BRANCH { condition: __esc_cond, conditionVar: __esc_var, .. }, .. } => {
                cond = (*__esc_cond).clone();
                var = (*__esc_var).clone();
                if var.clone() <= Variability::STRUCTURAL_PARAMETER.clone() {
                    cond = Ceval::evalExp(cond.clone(), &(Ceval::noTarget().clone()))?;
                }
                if !(Expression::isFalse(metamodelica::AsArg::as_arg(&cond))) {
                    Equation::Branch::triggerErrors(metamodelica::AsArg::as_arg(&branch))?;
                }
                accum
            },
            _ => metamodelica::cons(branch.clone(), accum),
            _ => unreachable!("match_deref! exhaustiveness placeholder"),
        } });
    }
    accum = List::trim(
        accum,
        &move |__a0: metamodelica::Ref<Equation::Branch::Branch>| -> metamodelica::Result<_> {
            ::std::result::Result::Ok(Equation::Branch::isEmpty(&__a0))
        },
    )?;
    if !((accum).is_empty()) {
        elements = metamodelica::cons(
            Equation::makeIf(
                metamodelica::Dangerous::listReverseInPlace(accum),
                NFInstNode::InstNode::fromCell(scope)?,
                src,
            ),
            elements,
        );
    }
    Ok(elements)
}

pub(crate) fn simplifyIfStmtBranches<ElemT: Clone + 'static + metamodelica::gc::MMTrace>(
    mut branches: &metamodelica::List<(metamodelica::Ref<Expression::NFExpression>, metamodelica::List<ElemT>)>,
    mut src: metamodelica::Ref<DAE::ElementSource>,
    mut makeFunc: &dyn ::std::ops::Fn(
        metamodelica::List<(metamodelica::Ref<Expression::NFExpression>, metamodelica::List<ElemT>)>,
        metamodelica::Ref<DAE::ElementSource>,
    ) -> Result<ElemT>,
    mut simplifyFunc: &dyn ::std::ops::Fn(metamodelica::List<ElemT>) -> Result<metamodelica::List<ElemT>>,
    mut elements: metamodelica::List<ElemT>,
) -> Result<metamodelica::List<ElemT>> {
    pub type MakeFunc<ElemT: Clone + 'static> = std::sync::Arc<
        dyn ::std::ops::Fn(
                metamodelica::List<(metamodelica::Ref<Expression::NFExpression>, metamodelica::List<ElemT>)>,
                metamodelica::Ref<DAE::ElementSource>,
            ) -> Result<ElemT>
            + 'static,
    >;

    pub type SimplifyFunc<ElemT: Clone + 'static> =
        std::sync::Arc<dyn ::std::ops::Fn(metamodelica::List<ElemT>) -> Result<metamodelica::List<ElemT>> + 'static>;

    let mut elements: metamodelica::List<ElemT> = elements;
    let mut cond: metamodelica::Ref<Expression::NFExpression>;
    let mut body: metamodelica::List<ElemT>;
    let mut accum: metamodelica::List<(metamodelica::Ref<Expression::NFExpression>, metamodelica::List<ElemT>)> =
        metamodelica::nil();
    for mut branch in &**branches {
        (cond, body) = branch.clone();
        cond = SimplifyExp::simplify(cond, false)?;
        if Expression::isTrue(&cond) {
            if (accum).is_empty() {
                elements = listAppend(simplifyFunc(body)?.reverse(), elements);
                return Ok(elements);
            } else {
                accum = metamodelica::cons((cond, simplifyFunc(body)?), accum);
                break;
            }
        } else if !(Expression::isFalse(&cond)) {
            accum = metamodelica::cons((cond, simplifyFunc(body)?), accum);
        }
    }
    if !((accum).is_empty()) {
        elements = metamodelica::cons(
            makeFunc(metamodelica::Dangerous::listReverseInPlace(accum), src)?,
            elements,
        );
    }
    Ok(elements)
}

pub(crate) fn simplifyFunction(mut func: metamodelica::Ref<Function::Function>) -> Result<()> {
    let mut cls: metamodelica::Ref<Class::NFClass>;
    let mut fn_body: metamodelica::Ref<Algorithm::NFAlgorithm>;
    let mut sections: metamodelica::Ref<Sections::NFSections>;
    if !(Function::isSimplified(&func)) {
        Function::markSimplified(&func);
        Function::mapExp(
            func.clone(),
            (std::sync::Arc::new({
                let __pe_b1 = false;
                move |__pe_a0| SimplifyExp::simplify(__pe_a0, __pe_b1.clone())
            })
                as std::sync::Arc<
                    dyn ::std::ops::Fn(
                            metamodelica::Ref<Expression::NFExpression>,
                        ) -> Result<metamodelica::Ref<Expression::NFExpression>>
                        + 'static,
                >),
            (std::sync::Arc::new({
                let __pe_b1 = false;
                move |__pe_a0| SimplifyExp::simplify(__pe_a0, __pe_b1.clone())
            })
                as std::sync::Arc<
                    dyn ::std::ops::Fn(
                            metamodelica::Ref<Expression::NFExpression>,
                        ) -> Result<metamodelica::Ref<Expression::NFExpression>>
                        + 'static,
                >),
            true,
            false,
        )?;
        cls = NFInstNode::InstNode::getClass(NFInstNode::InstNode::fromHandle(&func.node)?)?;
        let () = (match &*cls {
            Class::INSTANCED_CLASS {
                sections: __esc_sections,
                ..
            } => {
                sections = (*__esc_sections).clone();
                let () = (::match_deref::match_deref! { match &(sections.clone()) {
                    Deref @ Sections::SECTIONS { algorithms: Deref @ metamodelica::ListNode::Cons { head: __esc_fn_body, tail: Deref @ metamodelica::ListNode::Nil }, .. } => {
                        fn_body = (*__esc_fn_body).clone();
                        assign_field!(fn_body.statements = simplifyStatements(&fn_body.statements)?);
                        assign_variant_field!(sections => Sections::NFSections::SECTIONS; algorithms = list![fn_body.clone()]);
                        assign_variant_field!(cls => Class::NFClass::INSTANCED_CLASS; sections = sections.clone());
                        NFInstNode::InstNode::updateClass(cls, NFInstNode::InstNode::fromHandle(&func.node)?)?;
                        ()
                    },
                    _ => (),
                    _ => unreachable!("match_deref! exhaustiveness placeholder"),
                } });
                ()
            }
            _ => (),
        });
        for mut fn_der in &*func.derivatives.clone() {
            for mut der_fn in &*Function::getCachedFuncs(NFInstNode::InstNode::borrow(fn_der.derivativeFn.clone())?)? {
                simplifyFunction(der_fn.clone())?;
            }
        }
    }
    Ok(())
}

pub(crate) fn combineBinaries(
    mut flatModel: metamodelica::Ref<FlatModel::NFFlatModel>,
) -> Result<metamodelica::Ref<FlatModel::NFFlatModel>> {
    let mut flatModel: metamodelica::Ref<FlatModel::NFFlatModel> = flatModel;
    assign_field!(
        flatModel.variables = ({
            let mut __acc: metamodelica::List<metamodelica::Ref<Variable::NFVariable>> = metamodelica::nil();
            for mut var in (flatModel.variables.clone()).into_iter().cloned() {
                let __x = Variable::mapExp(
                    var.clone(),
                    (std::sync::Arc::new(SimplifyExp::combineBinaries)
                        as std::sync::Arc<
                            dyn ::std::ops::Fn(
                                    metamodelica::Ref<Expression::NFExpression>,
                                )
                                    -> Result<metamodelica::Ref<Expression::NFExpression>>
                                + 'static,
                        >),
                )?;
                __acc = cons(__x, __acc);
            }
            __acc.reverse()
        }),
        flatModel.equations = ({
            let mut __acc: metamodelica::List<metamodelica::Ref<Equation::NFEquation>> = metamodelica::nil();
            for mut eqn in (flatModel.equations.clone()).into_iter().cloned() {
                let __x = Equation::mapExp(eqn.clone(), &SimplifyExp::combineBinaries)?;
                __acc = cons(__x, __acc);
            }
            __acc.reverse()
        }),
        flatModel.initialEquations = ({
            let mut __acc: metamodelica::List<metamodelica::Ref<Equation::NFEquation>> = metamodelica::nil();
            for mut eqn in (flatModel.initialEquations.clone()).into_iter().cloned() {
                let __x = Equation::mapExp(eqn.clone(), &SimplifyExp::combineBinaries)?;
                __acc = cons(__x, __acc);
            }
            __acc.reverse()
        }),
        flatModel.algorithms = ({
            let mut __acc: metamodelica::List<metamodelica::Ref<Algorithm::NFAlgorithm>> = metamodelica::nil();
            for mut alg in (flatModel.algorithms.clone()).into_iter().cloned() {
                let __x = Algorithm::mapExp(alg.clone(), &SimplifyExp::combineBinaries)?;
                __acc = cons(__x, __acc);
            }
            __acc.reverse()
        }),
        flatModel.initialAlgorithms = ({
            let mut __acc: metamodelica::List<metamodelica::Ref<Algorithm::NFAlgorithm>> = metamodelica::nil();
            for mut alg in (flatModel.initialAlgorithms.clone()).into_iter().cloned() {
                let __x = Algorithm::mapExp(alg.clone(), &SimplifyExp::combineBinaries)?;
                __acc = cons(__x, __acc);
            }
            __acc.reverse()
        })
    );
    Ok(flatModel)
}
