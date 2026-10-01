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

use crate::NFBinding as Binding;
use crate::NFCall as Call;
use crate::NFClass as Class;
use crate::NFComponent as Component;
use crate::NFComponentRef as ComponentRef;
use crate::NFDimension as Dimension;
use crate::NFExpression as Expression;
use crate::NFFunction::Function;
use crate::NFInstNode;
use crate::NFInstNode::InstNode;
use crate::NFStatement as Statement;
use crate::NFSubscript as Subscript;
use crate::NFType as Type;
use openmodelica_frontend_dump::AbsynUtil;
use openmodelica_frontend_types::DAE;
use openmodelica_frontend_types::DAE::InlineType;
use openmodelica_util::Error;
use openmodelica_util::Flags;

pub(crate) fn inlineCallExp(
    mut callExp: metamodelica::Ref<Expression::NFExpression>,
    mut forceInline: bool,
) -> Result<metamodelica::Ref<Expression::NFExpression>> {
    let mut result: metamodelica::Ref<Expression::NFExpression>;
    result = (::match_deref::match_deref! { match &(callExp.clone()) {
        Deref @ Expression::CALL { call: call @ Deref @ Call::TYPED_CALL { .. } } => {
            let mut shouldInline: bool;
            shouldInline = (match Call::inlineType(metamodelica::AsArg::as_arg(&call)) {
        DAE::InlineType::BUILTIN_EARLY_INLINE { .. } => true,
        DAE::InlineType::EARLY_INLINE { .. } if (Flags::isSet(Flags::INLINE_FUNCTIONS.clone())?) => true,
        DAE::InlineType::NORM_INLINE { .. } => forceInline || Flags::getConfigBool(Flags::FRONTEND_INLINE.clone())?,
        _ => forceInline,
    });
            if (shouldInline) {inlineCall(callExp, forceInline)?} else {callExp}
        },
        _ => {
            callExp
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(result)
}

pub(crate) fn inlineCall(
    mut callExp: metamodelica::Ref<Expression::NFExpression>,
    mut forceInline: bool,
) -> Result<metamodelica::Ref<Expression::NFExpression>> {
    let mut exp: metamodelica::Ref<Expression::NFExpression>;
    let mut call: metamodelica::Ref<Call::NFCall>;
    let mut r#fn: metamodelica::Ref<Function::Function>;
    let mut arg: metamodelica::Ref<Expression::NFExpression>;
    let mut args: metamodelica::List<metamodelica::Ref<Expression::NFExpression>>;
    let mut inputs: metamodelica::List<metamodelica::Ref<InstNode::InstNode>>;
    let mut locals: metamodelica::List<metamodelica::Ref<InstNode::InstNode>>;
    let mut outputs: metamodelica::List<metamodelica::Ref<NFInstNode::NodeHandle>>;
    let mut body: metamodelica::List<metamodelica::Ref<Statement::NFStatement>>;
    let mut stmt: metamodelica::Ref<Statement::NFStatement>;
    let mut binding: metamodelica::Ref<Binding::NFBinding>;
    let __pa0 = ::match_deref::match_deref! { match &(callExp.clone()) {
        Deref @ Expression::CALL { call: __pa0 } => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    call = metamodelica::Own::own(__pa0);
    exp = (::match_deref::match_deref! { match &(&*call) {
        Deref @ Call::TYPED_CALL { r#fn, arguments: __esc_args, .. } if (!(InstNode::isEmpty(&(InstNode::fromHandle(&r#fn.node)?))) && InstNode::isNamed(&(InstNode::parentScope(InstNode::fromHandle(&r#fn.node)?, false)?), &(literal!("'constructor'")))) => {
            args = (*__esc_args).clone();
            body = Function::getBody(metamodelica::AsArg::as_arg(&r#fn))?;
            if !((body).is_empty() && (r#fn.locals).is_empty()) {
                exp = callExp;
                return Ok(exp);
            }
            binding = Component::getBinding(&(InstNode::component(&(InstNode::fromHandle(&((r#fn.outputs).head().cloned()?))?))?));
            if Binding::hasExp(&binding) {
                exp = Binding::getExp(&binding)?;
                let true = (Expression::isRecord(&exp)) else { return Err("pattern mismatch") };
            } else {
                exp = Class::makeRecordExp(InstNode::fromHandle(&((r#fn.outputs).head().cloned()?))?, InstNode::fromHandle(&r#fn.node)?, true)?;
            }
            for mut i in &*r#fn.inputs.clone() {
                let (__pa0, __pa1) = ::match_deref::match_deref! { match &(args.clone()) {
                    Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
                    _ => return Err("pattern mismatch"),
                } };
                arg = metamodelica::Own::own(__pa0);
                args = metamodelica::Own::own(__pa1);
                arg = inlineCallExp(arg, forceInline)?;
                exp = Expression::map(exp, (std::sync::Arc::new({ let __pe_b1 = i.clone(); let __pe_b2 = arg; move |__pe_a0| replaceCrefNode(__pe_a0, &__pe_b1, __pe_b2.clone()) }) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<Expression::NFExpression>) -> Result<metamodelica::Ref<Expression::NFExpression>> + 'static>))?;
            }
            exp
        },
        Deref @ Call::TYPED_CALL { r#fn: r#fn @ Deref @ Function::FUNCTION { inputs: __esc_inputs, outputs: __esc_outputs, locals: __esc_locals, .. }, arguments: __esc_args, .. } if (Function::hasSingleOrEmptyBody(metamodelica::AsArg::as_arg(&r#fn))) => {
            inputs = (*__esc_inputs).clone();
            outputs = (*__esc_outputs).clone();
            locals = (*__esc_locals).clone();
            args = (*__esc_args).clone();
            body = Function::getBody(metamodelica::AsArg::as_arg(&r#fn))?;
            body = removeDeadCode(body)?;
            if ((body).len() as i32) > 1 || ((outputs).len() as i32) != 1 || !((locals).is_empty()) {
                exp = callExp;
                return Ok(exp);
            }
            if (body).is_empty() {
                stmt = makeOutputStatement(InstNode::fromHandle(&((outputs).head().cloned()?))?)?;
            } else {
                stmt = convertToAssignment((body).head().cloned()?)?;
            }
            if !(Statement::isAssignment(&stmt)) {
                exp = callExp;
                return Ok(exp);
            }
            Error::assertion(((inputs).len() as i32) == ((args).len() as i32), { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NFInline.inlineCall")); __mm_s.push_str(&*literal!(" got wrong number of arguments for ")); __mm_s.push_str(&*AbsynUtil::pathString(Function::name(metamodelica::AsArg::as_arg(&r#fn)), literal!("."), true, false)?); ArcStr::from(__mm_s) }, &(metamodelica::sourceInfo!("NFFrontEnd/NFInline.mo")))?;
            match '__try0: {
                for mut i in &*inputs.clone() {
                    let (__pa1, __pa2) = ::match_deref::match_deref! { match &(args.clone()) {
                        Deref @ metamodelica::ListNode::Cons { head: __pa1, tail: __pa2 } => (__pa1.clone(), __pa2.clone()),
                        _ => break '__try0 Err::<_, _>("pattern mismatch"),
                    } };
                    arg = metamodelica::Own::own(__pa1);
                    args = metamodelica::Own::own(__pa2);
                    arg = unwrap_break_err!(inlineCallExp(arg.clone(), forceInline), '__try0);
                    stmt = unwrap_break_err!(Statement::mapExp(stmt.clone(), &({ let __pe_b1: Arc<dyn ::std::ops::Fn(metamodelica::Ref<Expression::NFExpression>) -> Result<metamodelica::Ref<Expression::NFExpression>> + 'static> = (std::sync::Arc::new({ let __pe_b1 = i.clone(); let __pe_b2 = arg.clone(); move |__pe_a0| replaceCrefNode(__pe_a0, &__pe_b1, __pe_b2.clone()) }) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<Expression::NFExpression>) -> Result<metamodelica::Ref<Expression::NFExpression>> + 'static>); move |__pe_a0| Expression::map(__pe_a0, __pe_b1.clone()) })), '__try0);
                }
                exp = unwrap_break_err!(getOutputExp(&stmt, &(unwrap_break_err!(InstNode::fromHandle(&(unwrap_break_err!((outputs).head().cloned(), '__try0))), '__try0)), call.clone()), '__try0);
                exp = unwrap_break_err!(Expression::map(exp.clone(), (std::sync::Arc::new({ let __pe_b1 = forceInline; move |__pe_a0| inlineCallExp(__pe_a0, __pe_b1.clone()) }) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<Expression::NFExpression>) -> Result<metamodelica::Ref<Expression::NFExpression>> + 'static>)), '__try0);
                Ok::<_, &'static str>((exp.clone(),))
            } {
                Ok((__try0_o0,)) => {
                    exp = __try0_o0;
                }
                Err(_) => {
                    exp = callExp.clone();
                }
            }
            exp
        },
        _ => callExp,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(exp)
}

fn replaceCrefNode(
    mut exp: metamodelica::Ref<Expression::NFExpression>,
    mut node: &metamodelica::Ref<InstNode::InstNode>,
    mut value: metamodelica::Ref<Expression::NFExpression>,
) -> Result<metamodelica::Ref<Expression::NFExpression>> {
    let mut exp: metamodelica::Ref<Expression::NFExpression> = exp;
    let mut ty: metamodelica::Ref<Type::NFType>;
    let mut repl_ty: metamodelica::Ref<Type::NFType>;
    exp = (match &*exp {
        Expression::CREF { cref: __exp_cref, .. }
            if (InstNode::refEqual(
                &(ComponentRef::node(&(ComponentRef::firstNonScope(__exp_cref.clone())?))?),
                node,
            )?) =>
        {
            replaceCrefNode2(metamodelica::AsArg::as_arg(&__exp_cref), node, value.clone())?
        }
        _ => exp,
    });
    ty = Expression::typeOf(exp.clone());
    repl_ty = Type::mapDims(
        ty.clone(),
        &({
            let __pe_b1 = node.clone();
            let __pe_b2 = value;
            move |__pe_a0| replaceDimExp(__pe_a0, &__pe_b1, &__pe_b2)
        }),
    )?;
    if !(referenceEq(&*(ty), &*(&*repl_ty))) {
        exp = Expression::setType(repl_ty, exp)?;
    }
    Ok(exp)
}

fn replaceCrefNode2(
    mut cref: &metamodelica::Ref<ComponentRef::NFComponentRef>,
    mut node: &metamodelica::Ref<InstNode::InstNode>,
    mut value: metamodelica::Ref<Expression::NFExpression>,
) -> Result<metamodelica::Ref<Expression::NFExpression>> {
    let mut value: metamodelica::Ref<Expression::NFExpression> = value;
    if !(InstNode::refEqual(node, &(ComponentRef::node(cref)?))?) {
        value = replaceCrefNode2(&(ComponentRef::rest(cref)?), node, value)?;
        value = Expression::recordElement(&(ComponentRef::nodeName(cref)?), &value)?;
    }
    value = Expression::applySubscripts(&(ComponentRef::getSubscripts(cref)), value, false)?;
    Ok(value)
}

fn replaceDimExp(
    mut dim: metamodelica::Ref<Dimension::NFDimension>,
    mut node: &metamodelica::Ref<InstNode::InstNode>,
    mut value: &metamodelica::Ref<Expression::NFExpression>,
) -> Result<metamodelica::Ref<Dimension::NFDimension>> {
    let mut dim: metamodelica::Ref<Dimension::NFDimension> = dim;
    dim = (match &*dim {
        Dimension::EXP {
            exp: __dim_exp,
            var: __dim_var,
        } => {
            let mut exp: metamodelica::Ref<Expression::NFExpression>;
            exp = Expression::map(
                __dim_exp.clone(),
                (std::sync::Arc::new({
                    let __pe_b1 = node.clone();
                    let __pe_b2 = value.clone();
                    move |__pe_a0| replaceCrefNode(__pe_a0, &__pe_b1, __pe_b2.clone())
                })
                    as std::sync::Arc<
                        dyn ::std::ops::Fn(
                                metamodelica::Ref<Expression::NFExpression>,
                            )
                                -> Result<metamodelica::Ref<Expression::NFExpression>>
                            + 'static,
                    >),
            )?;
            Dimension::fromExp(exp, __dim_var.clone())?
        }
        _ => dim,
    });
    Ok(dim)
}

fn removeDeadCode(
    mut body: metamodelica::List<metamodelica::Ref<Statement::NFStatement>>,
) -> Result<metamodelica::List<metamodelica::Ref<Statement::NFStatement>>> {
    let mut body: metamodelica::List<metamodelica::Ref<Statement::NFStatement>> = body;
    if ((body).len() as i32) > 1 && Statement::isReturn(&((body).get(2)?)) {
        body = list![(body).head().cloned()?];
    }
    Ok(body)
}

fn convertToAssignment(
    mut stmt: metamodelica::Ref<Statement::NFStatement>,
) -> Result<metamodelica::Ref<Statement::NFStatement>> {
    let mut outStmt: metamodelica::Ref<Statement::NFStatement>;
    outStmt = (match &*stmt {
        Statement::IF { .. } => convertIfToAssignment(stmt)?,
        _ => stmt,
    });
    Ok(outStmt)
}

fn convertIfToAssignment(
    mut stmt: metamodelica::Ref<Statement::NFStatement>,
) -> Result<metamodelica::Ref<Statement::NFStatement>> {
    let mut stmt: metamodelica::Ref<Statement::NFStatement> = stmt;
    let mut branches: metamodelica::List<(
        metamodelica::Ref<Expression::NFExpression>,
        metamodelica::List<metamodelica::Ref<Statement::NFStatement>>,
    )>;
    let mut cond: metamodelica::Ref<Expression::NFExpression>;
    let mut if_exp: metamodelica::Ref<Expression::NFExpression>;
    let mut output_exp: metamodelica::Ref<Expression::NFExpression>;
    let mut lhs: metamodelica::Ref<Expression::NFExpression>;
    let mut rhs: metamodelica::Ref<Expression::NFExpression>;
    let mut ty: metamodelica::Ref<Type::NFType> = crate::NFType::interned_UNKNOWN();
    let mut body: metamodelica::List<metamodelica::Ref<Statement::NFStatement>>;
    let mut s: metamodelica::Ref<Statement::NFStatement>;
    let mut source: metamodelica::Ref<DAE::ElementSource>;
    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(stmt.clone()) {
        Deref @ Statement::IF { branches: __pa0, source: __pa1 } => (__pa0.clone(), __pa1.clone()),
        _ => return Err("pattern mismatch"),
    } };
    branches = metamodelica::Own::own(__pa0);
    source = metamodelica::Own::own(__pa1);
    let (__pa2, __pa3, __pa4) = ::match_deref::match_deref! { match &(branches.reverse()) {
        Deref @ metamodelica::ListNode::Cons { head: (__pa2, __pa3), tail: __pa4 } => (__pa2.clone(), __pa3.clone(), __pa4.clone()),
        _ => return Err("pattern mismatch"),
    } };
    cond = metamodelica::Own::own(__pa2);
    body = metamodelica::Own::own(__pa3);
    branches = metamodelica::Own::own(__pa4);
    if !((branches).is_empty()) && !(Expression::isTrue(&cond)) {
        return Ok(stmt);
    }
    if ((body).len() as i32) != 1 {
        return Ok(stmt);
    }
    s = convertToAssignment((body).head().cloned()?)?;
    if !(Statement::isAssignment(&s)) {
        return Ok(stmt);
    }
    let (__pa5, __pa6) = ::match_deref::match_deref! { match &(s) {
        Deref @ Statement::ASSIGNMENT { lhs: __pa5, rhs: __pa6, .. } => (__pa5.clone(), __pa6.clone()),
        _ => return Err("pattern mismatch"),
    } };
    output_exp = metamodelica::Own::own(__pa5);
    if_exp = metamodelica::Own::own(__pa6);
    for mut b in &*branches.clone() {
        let (__pa7, __pa8, __pa9) = ::match_deref::match_deref! { match &(branches) {
            Deref @ metamodelica::ListNode::Cons { head: (__pa7, __pa8), tail: __pa9 } => (__pa7.clone(), __pa8.clone(), __pa9.clone()),
            _ => return Err("pattern mismatch"),
        } };
        cond = metamodelica::Own::own(__pa7);
        body = metamodelica::Own::own(__pa8);
        branches = metamodelica::Own::own(__pa9);
        if ((body).len() as i32) != 1 {
            return Ok(stmt);
        }
        s = convertToAssignment((body).head().cloned()?)?;
        if !(Statement::isAssignment(&s)) {
            return Ok(stmt);
        }
        let (__pa10, __pa11, __pa12) = ::match_deref::match_deref! { match &(s) {
            Deref @ Statement::ASSIGNMENT { lhs: __pa10, rhs: __pa11, ty: __pa12, .. } => (__pa10.clone(), __pa11.clone(), __pa12.clone()),
            _ => return Err("pattern mismatch"),
        } };
        lhs = metamodelica::Own::own(__pa10);
        rhs = metamodelica::Own::own(__pa11);
        ty = metamodelica::Own::own(__pa12);
        if !(Expression::isEqual(lhs, output_exp.clone())?) {
            return Ok(stmt);
        }
        if_exp = metamodelica::Ref::new(Expression::NFExpression::IF {
            ty: ty.clone(),
            condition: cond,
            trueBranch: rhs,
            falseBranch: if_exp,
        });
    }
    stmt = metamodelica::Ref::new(Statement::NFStatement::ASSIGNMENT {
        lhs: output_exp,
        rhs: if_exp,
        ty: ty,
        source: source,
    });
    Ok(stmt)
}

fn makeOutputStatement(
    mut outputNode: metamodelica::Ref<InstNode::InstNode>,
) -> Result<metamodelica::Ref<Statement::NFStatement>> {
    let mut stmt: metamodelica::Ref<Statement::NFStatement>;
    let mut binding: metamodelica::Ref<Binding::NFBinding>;
    let mut cref_exp: metamodelica::Ref<Expression::NFExpression>;
    let mut binding_exp: metamodelica::Ref<Expression::NFExpression>;
    binding = Component::getImplicitBinding(
        &(InstNode::component(&outputNode)?),
        InstNode::instanceParent(outputNode.clone())?,
    )?;
    if Binding::isBound(&binding) {
        cref_exp = Expression::fromCref(
            ComponentRef::fromNode(
                outputNode,
                crate::NFType::interned_UNKNOWN(),
                metamodelica::nil(),
                ComponentRef::Origin::CREF.clone(),
            )?,
            false,
        )?;
        binding_exp = Binding::getExp(&binding)?;
        stmt = Statement::makeAssignment(
            cref_exp,
            binding_exp,
            crate::NFType::interned_UNKNOWN(),
            DAE::emptyElementSource().clone(),
        );
    } else {
        stmt = metamodelica::Ref::new(Statement::NFStatement::FAILURE {
            body: metamodelica::nil(),
            source: DAE::emptyElementSource().clone(),
        });
    }
    Ok(stmt)
}

fn getOutputExp(
    mut stmt: &metamodelica::Ref<Statement::NFStatement>,
    mut outputNode: &metamodelica::Ref<InstNode::InstNode>,
    mut call: metamodelica::Ref<Call::NFCall>,
) -> Result<metamodelica::Ref<Expression::NFExpression>> {
    let mut exp: metamodelica::Ref<Expression::NFExpression>;
    exp = (::match_deref::match_deref! { match stmt {
        Deref @ Statement::ASSIGNMENT { lhs: Deref @ Expression::CREF { cref: cr @ Deref @ ComponentRef::CREF { subscripts: Deref @ metamodelica::ListNode::Nil, restCref: rest_cr, .. }, .. }, rhs: __stmt_rhs, .. } if (InstNode::refEqual(outputNode, &(ComponentRef::node(metamodelica::AsArg::as_arg(&cr))?))? && !(ComponentRef::isFromCref(metamodelica::AsArg::as_arg(&rest_cr)))) => {
            __stmt_rhs.clone()
        },
        _ => {
            metamodelica::Ref::new(Expression::NFExpression::CALL { call: call })
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(exp)
}
