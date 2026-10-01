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

use crate::BaseModelica;
use crate::NFComponentRef as ComponentRef;
use crate::NFExpression as Expression;
use crate::NFFlatModelicaUtil as FlatModelicaUtil;
use crate::NFInstNode::InstNode;
use crate::NFType as Type;
use openmodelica_frontend_dump::ElementSource;
use openmodelica_frontend_types::DAE;
use openmodelica_util::IOStream;
use openmodelica_util::Util;
use openmodelica_util_datatypes_basic::List;

#[derive(Clone, Debug, Eq, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub enum NFStatement {
    ASSIGNMENT {
        /// The asignee
        lhs: metamodelica::Ref<Expression::NFExpression>,
        /// The expression
        rhs: metamodelica::Ref<Expression::NFExpression>,
        ty: metamodelica::Ref<Type::NFType>,
        source: metamodelica::Ref<DAE::ElementSource>,
    },
    /// Used to mark in which order local array variables in functions should be initialized
    FUNCTION_ARRAY_INIT {
        name: ArcStr,
        ty: metamodelica::Ref<Type::NFType>,
        source: metamodelica::Ref<DAE::ElementSource>,
    },
    FOR {
        iterator: metamodelica::Ref<InstNode::InstNode>,
        range: Option<metamodelica::Ref<Expression::NFExpression>>,
        /// The body of the for loop.
        body: metamodelica::List<metamodelica::Ref<NFStatement>>,
        forType: ForType,
        source: metamodelica::Ref<DAE::ElementSource>,
        /// sub-iterators for ARRAY iterator case (NBackEnd only)
        sub_iters: metamodelica::List<(
            metamodelica::Ref<ComponentRef::NFComponentRef>,
            metamodelica::Array<metamodelica::Ref<Expression::NFExpression>>,
        )>,
    },
    IF {
        /// List of branches, where each branch is a tuple of a condition and a body.
        branches: metamodelica::List<(
            metamodelica::Ref<Expression::NFExpression>,
            metamodelica::List<metamodelica::Ref<NFStatement>>,
        )>,
        source: metamodelica::Ref<DAE::ElementSource>,
    },
    WHEN {
        /// List of branches, where each branch is a tuple of a condition and a body.
        branches: metamodelica::List<(
            metamodelica::Ref<Expression::NFExpression>,
            metamodelica::List<metamodelica::Ref<NFStatement>>,
        )>,
        source: metamodelica::Ref<DAE::ElementSource>,
    },
    ASSERT {
        /// The assert condition.
        condition: metamodelica::Ref<Expression::NFExpression>,
        /// The message to display if the assert fails.
        message: metamodelica::Ref<Expression::NFExpression>,
        level: metamodelica::Ref<Expression::NFExpression>,
        source: metamodelica::Ref<DAE::ElementSource>,
    },
    TERMINATE {
        /// The message to display if the terminate triggers.
        message: metamodelica::Ref<Expression::NFExpression>,
        source: metamodelica::Ref<DAE::ElementSource>,
    },
    REINIT {
        cref: metamodelica::Ref<Expression::NFExpression>,
        reinitExp: metamodelica::Ref<Expression::NFExpression>,
        source: metamodelica::Ref<DAE::ElementSource>,
    },
    NORETCALL {
        exp: metamodelica::Ref<Expression::NFExpression>,
        source: metamodelica::Ref<DAE::ElementSource>,
    },
    WHILE {
        condition: metamodelica::Ref<Expression::NFExpression>,
        body: metamodelica::List<metamodelica::Ref<NFStatement>>,
        source: metamodelica::Ref<DAE::ElementSource>,
    },
    RETURN {
        source: metamodelica::Ref<DAE::ElementSource>,
    },
    BREAK {
        source: metamodelica::Ref<DAE::ElementSource>,
    },
    FAILURE {
        body: metamodelica::List<metamodelica::Ref<NFStatement>>,
        source: metamodelica::Ref<DAE::ElementSource>,
    },
}
impl metamodelica::gc::MMTrace for NFStatement {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        match self {
            NFStatement::ASSIGNMENT { lhs, rhs, ty, source } => {
                metamodelica::gc::MMTrace::mm_accept(lhs, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(rhs, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(ty, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(source, __mmv)?;
                Ok(())
            }
            NFStatement::FUNCTION_ARRAY_INIT { name, ty, source } => {
                metamodelica::gc::MMTrace::mm_accept(name, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(ty, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(source, __mmv)?;
                Ok(())
            }
            NFStatement::FOR {
                iterator,
                range,
                body,
                forType,
                source,
                sub_iters,
            } => {
                metamodelica::gc::MMTrace::mm_accept(iterator, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(range, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(body, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(forType, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(source, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(sub_iters, __mmv)?;
                Ok(())
            }
            NFStatement::IF { branches, source } => {
                metamodelica::gc::MMTrace::mm_accept(branches, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(source, __mmv)?;
                Ok(())
            }
            NFStatement::WHEN { branches, source } => {
                metamodelica::gc::MMTrace::mm_accept(branches, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(source, __mmv)?;
                Ok(())
            }
            NFStatement::ASSERT {
                condition,
                message,
                level,
                source,
            } => {
                metamodelica::gc::MMTrace::mm_accept(condition, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(message, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(level, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(source, __mmv)?;
                Ok(())
            }
            NFStatement::TERMINATE { message, source } => {
                metamodelica::gc::MMTrace::mm_accept(message, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(source, __mmv)?;
                Ok(())
            }
            NFStatement::REINIT {
                cref,
                reinitExp,
                source,
            } => {
                metamodelica::gc::MMTrace::mm_accept(cref, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(reinitExp, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(source, __mmv)?;
                Ok(())
            }
            NFStatement::NORETCALL { exp, source } => {
                metamodelica::gc::MMTrace::mm_accept(exp, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(source, __mmv)?;
                Ok(())
            }
            NFStatement::WHILE {
                condition,
                body,
                source,
            } => {
                metamodelica::gc::MMTrace::mm_accept(condition, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(body, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(source, __mmv)?;
                Ok(())
            }
            NFStatement::RETURN { source } => {
                metamodelica::gc::MMTrace::mm_accept(source, __mmv)?;
                Ok(())
            }
            NFStatement::BREAK { source } => {
                metamodelica::gc::MMTrace::mm_accept(source, __mmv)?;
                Ok(())
            }
            NFStatement::FAILURE { body, source } => {
                metamodelica::gc::MMTrace::mm_accept(body, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(source, __mmv)?;
                Ok(())
            }
        }
    }
}
impl Default for NFStatement {
    fn default() -> Self {
        Self::RETURN {
            source: Default::default(),
        }
    }
}
pub use self::NFStatement::{
    ASSERT, ASSIGNMENT, BREAK, FAILURE, FOR, FUNCTION_ARRAY_INIT, IF, NORETCALL, REINIT, RETURN, TERMINATE, WHEN, WHILE,
};
#[derive(Clone, Debug, Eq, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub enum ForType {
    NORMAL,
    PARALLEL {
        vars: metamodelica::List<(metamodelica::Ref<ComponentRef::NFComponentRef>, SourceInfo)>,
    },
}
impl metamodelica::gc::MMTrace for ForType {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        match self {
            ForType::NORMAL => Ok(()),
            ForType::PARALLEL { vars } => {
                metamodelica::gc::MMTrace::mm_accept(vars, __mmv)?;
                Ok(())
            }
        }
    }
}
impl Default for ForType {
    fn default() -> Self {
        Self::NORMAL
    }
}
pub use self::ForType::{NORMAL, PARALLEL};

pub(crate) fn isDiscrete(mut stmt: &metamodelica::Ref<NFStatement>) -> Result<bool> {
    let mut b: bool = false;
    b = (match &**stmt {
        ASSIGNMENT { ty: __stmt_ty, .. } => Type::isDiscrete(__stmt_ty.clone())?,
        FUNCTION_ARRAY_INIT { ty: __stmt_ty, .. } => Type::isDiscrete(__stmt_ty.clone())?,
        FOR { body: __stmt_body, .. } => List::any(
            metamodelica::AsArg::as_arg(&__stmt_body),
            &move |__a0: metamodelica::Ref<NFStatement>| isDiscrete(&__a0),
        )?,
        IF {
            branches: __stmt_branches,
            ..
        } => {
            for mut branch in &*__stmt_branches.clone() {
                b = List::any(&(Util::tuple22(branch.clone())), &move |__a0: metamodelica::Ref<
                    NFStatement,
                >| isDiscrete(&__a0))?;
                if b {
                    break;
                }
            }
            b
        }
        WHEN { .. } => true,
        WHILE { body: __stmt_body, .. } => List::any(
            metamodelica::AsArg::as_arg(&__stmt_body),
            &move |__a0: metamodelica::Ref<NFStatement>| isDiscrete(&__a0),
        )?,
        _ => false,
    });
    Ok(b)
}

pub fn filterDiscrete(
    mut stmts: metamodelica::List<metamodelica::Ref<NFStatement>>,
    mut out_stmts: metamodelica::List<metamodelica::Ref<NFStatement>>,
) -> Result<metamodelica::List<metamodelica::Ref<NFStatement>>> {
    '__tco: loop {
        let mut stmt: metamodelica::Ref<NFStatement>;
        let mut rest: metamodelica::List<metamodelica::Ref<NFStatement>>;
        ::match_deref::match_deref! { match &(stmts) {
            Deref @ metamodelica::ListNode::Cons { head: __esc_stmt @ Deref @ FOR { .. }, tail: __esc_rest } => {
                stmt = (*__esc_stmt).clone();
                rest = (*__esc_rest).clone();
                assign_variant_field!(stmt => NFStatement::FOR; body = filterDiscrete(var_field!((*stmt).body, NFStatement::FOR).clone(), metamodelica::nil())?);
                out_stmts = if (((var_field!((*stmt).body, NFStatement::FOR)).len() as i32) == 0) {out_stmts} else {metamodelica::cons(stmt.clone(), out_stmts)};
                { (stmts, out_stmts) = (rest.clone(), out_stmts); continue '__tco; }
            },
            Deref @ metamodelica::ListNode::Cons { head: __esc_stmt @ Deref @ IF { .. }, tail: __esc_rest } => {
                stmt = (*__esc_stmt).clone();
                rest = (*__esc_rest).clone();
                assign_variant_field!(stmt => NFStatement::IF; branches = ({
            let mut __acc: metamodelica::List<(metamodelica::Ref<Expression::NFExpression>, metamodelica::List<metamodelica::Ref<NFStatement>>)> = metamodelica::nil();
            for mut tpl in (var_field!((*stmt).branches, NFStatement::IF).clone()).into_iter().cloned() {
                let __x = (Util::tuple21(tpl.clone()), filterDiscrete(Util::tuple22(tpl.clone()), metamodelica::nil())?);
                __acc = cons(__x, __acc);
            }
            __acc.reverse()
        }));
                { (stmts, out_stmts) = (rest.clone(), metamodelica::cons(stmt.clone(), out_stmts)); continue '__tco; }
            },
            Deref @ metamodelica::ListNode::Cons { head: stmt, tail: __esc_rest } if (isDiscrete(metamodelica::AsArg::as_arg(&stmt))?) => {
                rest = (*__esc_rest).clone();
                { (stmts, out_stmts) = (rest.clone(), out_stmts); continue '__tco; }
            },
            Deref @ metamodelica::ListNode::Cons { head: __esc_stmt, tail: __esc_rest } => {
                stmt = (*__esc_stmt).clone();
                rest = (*__esc_rest).clone();
                { (stmts, out_stmts) = (rest.clone(), metamodelica::cons(stmt.clone(), out_stmts)); continue '__tco; }
            },
            _ => return Ok(out_stmts.reverse()),
            _ => return Err("match: no arm matched"),
        } }
    }
}

pub fn hash(mut stmt: &metamodelica::Ref<NFStatement>) -> Result<i32> {
    let mut hash: i32 = stringHashDjb2(&(toString(stmt, literal!(""))?));
    Ok(hash)
}

pub fn isEqual(mut stmt1: &metamodelica::Ref<NFStatement>, mut stmt2: &metamodelica::Ref<NFStatement>) -> Result<bool> {
    fn branchEqual(
        mut branch1: &(
            metamodelica::Ref<Expression::NFExpression>,
            metamodelica::List<metamodelica::Ref<NFStatement>>,
        ),
        mut branch2: &(
            metamodelica::Ref<Expression::NFExpression>,
            metamodelica::List<metamodelica::Ref<NFStatement>>,
        ),
    ) -> Result<bool> {
        let mut b: bool;
        let mut e1: metamodelica::Ref<Expression::NFExpression>;
        let mut e2: metamodelica::Ref<Expression::NFExpression>;
        let mut b1: metamodelica::List<metamodelica::Ref<NFStatement>>;
        let mut b2: metamodelica::List<metamodelica::Ref<NFStatement>>;
        (e1, b1) = branch1.clone();
        (e2, b2) = branch2.clone();
        b = Expression::isEqual(e1, e2)?
            && List::isEqualOnTrue(
                b1,
                b2,
                &move |__a0: metamodelica::Ref<NFStatement>, __a1: metamodelica::Ref<NFStatement>| {
                    isEqual(&__a0, &__a1)
                },
            )?;
        Ok(b)
    }

    let mut b: bool;
    b = (::match_deref::match_deref! { match (stmt1, stmt2) {
        (Deref @ ASSIGNMENT { .. }, Deref @ ASSIGNMENT { .. }) => Expression::isEqual(var_field!((**stmt1).lhs, NFStatement::ASSIGNMENT).clone(), var_field!((**stmt2).lhs, NFStatement::ASSIGNMENT).clone())? && Expression::isEqual(var_field!((**stmt1).rhs, NFStatement::ASSIGNMENT).clone(), var_field!((**stmt2).rhs, NFStatement::ASSIGNMENT).clone())?,
        (Deref @ FUNCTION_ARRAY_INIT { .. }, Deref @ FUNCTION_ARRAY_INIT { .. }) => stringEqual(&var_field!((**stmt1).name, NFStatement::FUNCTION_ARRAY_INIT), &var_field!((**stmt2).name, NFStatement::FUNCTION_ARRAY_INIT)),
        (Deref @ FOR { .. }, Deref @ FOR { .. }) => InstNode::nameEqual(var_field!((**stmt1).iterator, NFStatement::FOR), var_field!((**stmt2).iterator, NFStatement::FOR))? && Util::optionEqual(var_field!((**stmt1).range, NFStatement::FOR).clone(), var_field!((**stmt2).range, NFStatement::FOR).clone(), &Expression::isEqual)? && List::isEqualOnTrue(var_field!((**stmt1).body, NFStatement::FOR).clone(), var_field!((**stmt2).body, NFStatement::FOR).clone(), &move |__a0: metamodelica::Ref<NFStatement>, __a1: metamodelica::Ref<NFStatement>| isEqual(&__a0, &__a1))?,
        (Deref @ IF { .. }, Deref @ IF { .. }) => List::isEqualOnTrue(var_field!((**stmt1).branches, NFStatement::IF).clone(), var_field!((**stmt2).branches, NFStatement::IF).clone(), &move |__a0: (metamodelica::Ref<Expression::NFExpression>, metamodelica::List<metamodelica::Ref<NFStatement>>), __a1: (metamodelica::Ref<Expression::NFExpression>, metamodelica::List<metamodelica::Ref<NFStatement>>)| branchEqual(&__a0, &__a1))?,
        (Deref @ WHEN { .. }, Deref @ WHEN { .. }) => List::isEqualOnTrue(var_field!((**stmt1).branches, NFStatement::WHEN).clone(), var_field!((**stmt2).branches, NFStatement::WHEN).clone(), &move |__a0: (metamodelica::Ref<Expression::NFExpression>, metamodelica::List<metamodelica::Ref<NFStatement>>), __a1: (metamodelica::Ref<Expression::NFExpression>, metamodelica::List<metamodelica::Ref<NFStatement>>)| branchEqual(&__a0, &__a1))?,
        (Deref @ ASSERT { .. }, Deref @ ASSERT { .. }) => Expression::isEqual(var_field!((**stmt1).condition, NFStatement::ASSERT).clone(), var_field!((**stmt2).condition, NFStatement::ASSERT).clone())? && Expression::isEqual(var_field!((**stmt1).message, NFStatement::ASSERT).clone(), var_field!((**stmt2).message, NFStatement::ASSERT).clone())? && Expression::isEqual(var_field!((**stmt1).level, NFStatement::ASSERT).clone(), var_field!((**stmt2).level, NFStatement::ASSERT).clone())?,
        (Deref @ TERMINATE { .. }, Deref @ TERMINATE { .. }) => Expression::isEqual(var_field!((**stmt1).message, NFStatement::TERMINATE).clone(), var_field!((**stmt2).message, NFStatement::TERMINATE).clone())?,
        (Deref @ REINIT { .. }, Deref @ REINIT { .. }) => Expression::isEqual(var_field!((**stmt1).cref, NFStatement::REINIT).clone(), var_field!((**stmt2).cref, NFStatement::REINIT).clone())? && Expression::isEqual(var_field!((**stmt1).reinitExp, NFStatement::REINIT).clone(), var_field!((**stmt2).reinitExp, NFStatement::REINIT).clone())?,
        (Deref @ NORETCALL { .. }, Deref @ NORETCALL { .. }) => Expression::isEqual(var_field!((**stmt1).exp, NFStatement::NORETCALL).clone(), var_field!((**stmt2).exp, NFStatement::NORETCALL).clone())?,
        (Deref @ WHILE { .. }, Deref @ WHILE { .. }) => Expression::isEqual(var_field!((**stmt1).condition, NFStatement::WHILE).clone(), var_field!((**stmt2).condition, NFStatement::WHILE).clone())? && List::isEqualOnTrue(var_field!((**stmt1).body, NFStatement::WHILE).clone(), var_field!((**stmt2).body, NFStatement::WHILE).clone(), &move |__a0: metamodelica::Ref<NFStatement>, __a1: metamodelica::Ref<NFStatement>| isEqual(&__a0, &__a1))?,
        (Deref @ RETURN { .. }, Deref @ RETURN { .. }) => true,
        (Deref @ BREAK { .. }, Deref @ BREAK { .. }) => true,
        (Deref @ FAILURE { .. }, Deref @ FAILURE { .. }) => List::isEqualOnTrue(var_field!((**stmt1).body, NFStatement::FAILURE).clone(), var_field!((**stmt2).body, NFStatement::FAILURE).clone(), &move |__a0: metamodelica::Ref<NFStatement>, __a1: metamodelica::Ref<NFStatement>| isEqual(&__a0, &__a1))?,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(b)
}

pub fn makeAssignment(
    mut lhs: metamodelica::Ref<Expression::NFExpression>,
    mut rhs: metamodelica::Ref<Expression::NFExpression>,
    mut ty: metamodelica::Ref<Type::NFType>,
    mut src: metamodelica::Ref<DAE::ElementSource>,
) -> metamodelica::Ref<NFStatement> {
    let mut stmt: metamodelica::Ref<NFStatement>;
    stmt = metamodelica::Ref::new(NFStatement::ASSIGNMENT {
        lhs: lhs,
        rhs: rhs,
        ty: ty,
        source: src,
    });
    stmt
}

pub(crate) fn isAssignment(mut stmt: &metamodelica::Ref<NFStatement>) -> bool {
    let mut res: bool;
    res = (match &**stmt {
        ASSIGNMENT { .. } => true,
        _ => false,
    });
    res
}

pub(crate) fn isFor(mut stmt: &metamodelica::Ref<NFStatement>) -> bool {
    let mut res: bool;
    res = (match &**stmt {
        FOR { .. } => true,
        _ => false,
    });
    res
}

pub(crate) fn isReturn(mut stmt: &metamodelica::Ref<NFStatement>) -> bool {
    let mut res: bool;
    res = (match &**stmt {
        RETURN { .. } => true,
        _ => false,
    });
    res
}

pub(crate) fn makeIf(
    mut branches: metamodelica::List<(
        metamodelica::Ref<Expression::NFExpression>,
        metamodelica::List<metamodelica::Ref<NFStatement>>,
    )>,
    mut src: metamodelica::Ref<DAE::ElementSource>,
) -> metamodelica::Ref<NFStatement> {
    let mut stmt: metamodelica::Ref<NFStatement>;
    stmt = metamodelica::Ref::new(NFStatement::IF {
        branches: branches,
        source: src,
    });
    stmt
}

pub fn source(mut stmt: &metamodelica::Ref<NFStatement>) -> metamodelica::Ref<DAE::ElementSource> {
    let mut source: metamodelica::Ref<DAE::ElementSource>;
    source = (match &**stmt {
        ASSIGNMENT {
            source: __stmt_source, ..
        } => __stmt_source.clone(),
        FUNCTION_ARRAY_INIT {
            source: __stmt_source, ..
        } => __stmt_source.clone(),
        FOR {
            source: __stmt_source, ..
        } => __stmt_source.clone(),
        IF {
            source: __stmt_source, ..
        } => __stmt_source.clone(),
        WHEN {
            source: __stmt_source, ..
        } => __stmt_source.clone(),
        ASSERT {
            source: __stmt_source, ..
        } => __stmt_source.clone(),
        TERMINATE {
            source: __stmt_source, ..
        } => __stmt_source.clone(),
        REINIT {
            source: __stmt_source, ..
        } => __stmt_source.clone(),
        NORETCALL {
            source: __stmt_source, ..
        } => __stmt_source.clone(),
        WHILE {
            source: __stmt_source, ..
        } => __stmt_source.clone(),
        RETURN { source: __stmt_source } => __stmt_source.clone(),
        BREAK { source: __stmt_source } => __stmt_source.clone(),
        FAILURE {
            source: __stmt_source, ..
        } => __stmt_source.clone(),
    });
    source
}

pub(crate) fn setSource(
    mut source: metamodelica::Ref<DAE::ElementSource>,
    mut stmt: metamodelica::Ref<NFStatement>,
) -> Result<metamodelica::Ref<NFStatement>> {
    let mut stmt: metamodelica::Ref<NFStatement> = stmt;
    let () = (match &*stmt {
        ASSIGNMENT { .. } => {
            assign_variant_field!(stmt => NFStatement::ASSIGNMENT; source = source);
            ()
        }
        FUNCTION_ARRAY_INIT { .. } => {
            assign_variant_field!(stmt => NFStatement::FUNCTION_ARRAY_INIT; source = source);
            ()
        }
        FOR { .. } => {
            assign_variant_field!(stmt => NFStatement::FOR; source = source);
            ()
        }
        IF { .. } => {
            assign_variant_field!(stmt => NFStatement::IF; source = source);
            ()
        }
        WHEN { .. } => {
            assign_variant_field!(stmt => NFStatement::WHEN; source = source);
            ()
        }
        ASSERT { .. } => {
            assign_variant_field!(stmt => NFStatement::ASSERT; source = source);
            ()
        }
        TERMINATE { .. } => {
            assign_variant_field!(stmt => NFStatement::TERMINATE; source = source);
            ()
        }
        NORETCALL { .. } => {
            assign_variant_field!(stmt => NFStatement::NORETCALL; source = source);
            ()
        }
        WHILE { .. } => {
            assign_variant_field!(stmt => NFStatement::WHILE; source = source);
            ()
        }
        RETURN { .. } => {
            assign_variant_field!(stmt => NFStatement::RETURN; source = source);
            ()
        }
        BREAK { .. } => {
            assign_variant_field!(stmt => NFStatement::BREAK; source = source);
            ()
        }
        FAILURE { .. } => {
            assign_variant_field!(stmt => NFStatement::FAILURE; source = source);
            ()
        }
        _ => return Err("match: no arm matched"),
    });
    Ok(stmt)
}

pub(crate) fn info(mut stmt: &metamodelica::Ref<NFStatement>) -> SourceInfo {
    let mut info: SourceInfo = ElementSource::getInfo(source(stmt));
    info
}

pub type ApplyFn = std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<NFStatement>) -> Result<()> + 'static>;

pub(crate) fn apply(
    mut stmt: metamodelica::Ref<NFStatement>,
    mut func: &dyn ::std::ops::Fn(metamodelica::Ref<NFStatement>) -> Result<()>,
) -> Result<()> {
    let () = (match &*stmt {
        FOR { body: __stmt_body, .. } => {
            for mut e in &*__stmt_body.clone() {
                apply(e.clone(), func)?;
            }
            ()
        }
        IF {
            branches: __stmt_branches,
            ..
        } => {
            for mut b in &*__stmt_branches.clone() {
                for mut e in &*Util::tuple22(b.clone()) {
                    apply(e.clone(), func)?;
                }
            }
            ()
        }
        WHEN {
            branches: __stmt_branches,
            ..
        } => {
            for mut b in &*__stmt_branches.clone() {
                for mut e in &*Util::tuple22(b.clone()) {
                    apply(e.clone(), func)?;
                }
            }
            ()
        }
        WHILE { body: __stmt_body, .. } => {
            for mut e in &*__stmt_body.clone() {
                apply(e.clone(), func)?;
            }
            ()
        }
        FAILURE { body: __stmt_body, .. } => {
            for mut e in &*__stmt_body.clone() {
                apply(e.clone(), func)?;
            }
            ()
        }
        _ => (),
    });
    func(stmt)?;
    Ok(())
}

pub(crate) fn map(
    mut stmt: metamodelica::Ref<NFStatement>,
    mut func: &dyn ::std::ops::Fn(metamodelica::Ref<NFStatement>) -> Result<metamodelica::Ref<NFStatement>>,
) -> Result<metamodelica::Ref<NFStatement>> {
    pub type MapFn = std::sync::Arc<
        dyn ::std::ops::Fn(metamodelica::Ref<NFStatement>) -> Result<metamodelica::Ref<NFStatement>> + 'static,
    >;

    let mut stmt: metamodelica::Ref<NFStatement> = stmt;
    let () = (match &*stmt {
        FOR { body: __stmt_body, .. } => {
            assign_variant_field!(stmt => NFStatement::FOR; body = ({
                let mut __acc: metamodelica::List<metamodelica::Ref<NFStatement>> = metamodelica::nil();
                for mut s in (__stmt_body.clone()).into_iter().cloned() {
                    let __x = map(s.clone(), func)?;
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            }));
            ()
        }
        IF {
            branches: __stmt_branches,
            ..
        } => {
            assign_variant_field!(stmt => NFStatement::IF; branches = ({
                let mut __acc: metamodelica::List<(metamodelica::Ref<Expression::NFExpression>, metamodelica::List<metamodelica::Ref<NFStatement>>)> = metamodelica::nil();
                for mut b in (__stmt_branches.clone()).into_iter().cloned() {
                    let __x = (Util::tuple21(b.clone()), ({
                let mut __acc: metamodelica::List<metamodelica::Ref<NFStatement>> = metamodelica::nil();
                for mut s in (Util::tuple22(b.clone())).into_iter().cloned() {
                    let __x = map(s.clone(), func)?;
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            }));
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            }));
            ()
        }
        WHEN {
            branches: __stmt_branches,
            ..
        } => {
            assign_variant_field!(stmt => NFStatement::WHEN; branches = ({
                let mut __acc: metamodelica::List<(metamodelica::Ref<Expression::NFExpression>, metamodelica::List<metamodelica::Ref<NFStatement>>)> = metamodelica::nil();
                for mut b in (__stmt_branches.clone()).into_iter().cloned() {
                    let __x = (Util::tuple21(b.clone()), ({
                let mut __acc: metamodelica::List<metamodelica::Ref<NFStatement>> = metamodelica::nil();
                for mut s in (Util::tuple22(b.clone())).into_iter().cloned() {
                    let __x = map(s.clone(), func)?;
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            }));
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            }));
            ()
        }
        WHILE { body: __stmt_body, .. } => {
            assign_variant_field!(stmt => NFStatement::WHILE; body = ({
                let mut __acc: metamodelica::List<metamodelica::Ref<NFStatement>> = metamodelica::nil();
                for mut s in (__stmt_body.clone()).into_iter().cloned() {
                    let __x = map(s.clone(), func)?;
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            }));
            ()
        }
        _ => (),
    });
    stmt = func(stmt)?;
    Ok(stmt)
}

pub(crate) fn fold<ArgT: Clone + 'static + metamodelica::gc::MMTrace>(
    mut stmt: metamodelica::Ref<NFStatement>,
    mut func: &dyn ::std::ops::Fn(metamodelica::Ref<NFStatement>, ArgT) -> Result<ArgT>,
    mut arg: ArgT,
) -> Result<ArgT> {
    pub type MapFn<ArgT: Clone + 'static> =
        std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<NFStatement>, ArgT) -> Result<ArgT> + 'static>;

    let mut arg: ArgT = arg;
    let () = (match &*stmt {
        FOR { body: __stmt_body, .. } => {
            for mut s in &*__stmt_body.clone() {
                arg = fold(s.clone(), func, arg)?;
            }
            ()
        }
        IF {
            branches: __stmt_branches,
            ..
        } => {
            for mut b in &*__stmt_branches.clone() {
                for mut s in &*Util::tuple22(b.clone()) {
                    arg = fold(s.clone(), func, arg)?;
                }
            }
            ()
        }
        WHEN {
            branches: __stmt_branches,
            ..
        } => {
            for mut b in &*__stmt_branches.clone() {
                for mut s in &*Util::tuple22(b.clone()) {
                    arg = fold(s.clone(), func, arg)?;
                }
            }
            ()
        }
        WHILE { body: __stmt_body, .. } => {
            for mut s in &*__stmt_body.clone() {
                arg = fold(s.clone(), func, arg)?;
            }
            ()
        }
        _ => (),
    });
    arg = func(stmt, arg)?;
    Ok(arg)
}

pub fn applyExpList(
    mut stmt: &metamodelica::List<metamodelica::Ref<NFStatement>>,
    mut func: &dyn ::std::ops::Fn(metamodelica::Ref<Expression::NFExpression>) -> Result<()>,
) -> Result<()> {
    pub type FoldFunc =
        std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<Expression::NFExpression>) -> Result<()> + 'static>;

    for mut s in &**stmt {
        applyExp(metamodelica::AsArg::as_arg(&s), func)?;
    }
    Ok(())
}

pub(crate) fn applyExp(
    mut stmt: &metamodelica::Ref<NFStatement>,
    mut func: &dyn ::std::ops::Fn(metamodelica::Ref<Expression::NFExpression>) -> Result<()>,
) -> Result<()> {
    pub type ApplyFunc =
        std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<Expression::NFExpression>) -> Result<()> + 'static>;

    let () = (match &**stmt {
        ASSIGNMENT {
            lhs: __stmt_lhs,
            rhs: __stmt_rhs,
            ..
        } => {
            func(__stmt_lhs.clone())?;
            func(__stmt_rhs.clone())?;
            ()
        }
        FOR {
            body: __stmt_body,
            range: __stmt_range,
            ..
        } => {
            applyExpList(metamodelica::AsArg::as_arg(&__stmt_body), func)?;
            if (__stmt_range).is_some() {
                func(Util::getOption(__stmt_range.clone())?)?;
            }
            ()
        }
        IF {
            branches: __stmt_branches,
            ..
        } => {
            for mut b in &*__stmt_branches.clone() {
                func(Util::tuple21(b.clone()))?;
                applyExpList(&(Util::tuple22(b.clone())), func)?;
            }
            ()
        }
        WHEN {
            branches: __stmt_branches,
            ..
        } => {
            for mut b in &*__stmt_branches.clone() {
                func(Util::tuple21(b.clone()))?;
                applyExpList(&(Util::tuple22(b.clone())), func)?;
            }
            ()
        }
        ASSERT {
            condition: __stmt_condition,
            level: __stmt_level,
            message: __stmt_message,
            ..
        } => {
            func(__stmt_condition.clone())?;
            func(__stmt_message.clone())?;
            func(__stmt_level.clone())?;
            ()
        }
        TERMINATE {
            message: __stmt_message,
            ..
        } => {
            func(__stmt_message.clone())?;
            ()
        }
        REINIT {
            cref: __stmt_cref,
            reinitExp: __stmt_reinitExp,
            ..
        } => {
            func(__stmt_cref.clone())?;
            func(__stmt_reinitExp.clone())?;
            ()
        }
        NORETCALL { exp: __stmt_exp, .. } => {
            func(__stmt_exp.clone())?;
            ()
        }
        WHILE {
            body: __stmt_body,
            condition: __stmt_condition,
            ..
        } => {
            func(__stmt_condition.clone())?;
            applyExpList(metamodelica::AsArg::as_arg(&__stmt_body), func)?;
            ()
        }
        _ => (),
    });
    Ok(())
}

pub(crate) fn mapExpList(
    mut stmtl: metamodelica::List<metamodelica::Ref<NFStatement>>,
    mut func: &dyn ::std::ops::Fn(
        metamodelica::Ref<Expression::NFExpression>,
    ) -> Result<metamodelica::Ref<Expression::NFExpression>>,
) -> Result<metamodelica::List<metamodelica::Ref<NFStatement>>> {
    pub type MapFunc = std::sync::Arc<
        dyn ::std::ops::Fn(
                metamodelica::Ref<Expression::NFExpression>,
            ) -> Result<metamodelica::Ref<Expression::NFExpression>>
            + 'static,
    >;

    let mut stmtl: metamodelica::List<metamodelica::Ref<NFStatement>> = stmtl;
    stmtl = ({
        let mut __acc: metamodelica::List<metamodelica::Ref<NFStatement>> = metamodelica::nil();
        for mut s in (stmtl).into_iter().cloned() {
            let __x = mapExp(s.clone(), func)?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    Ok(stmtl)
}

pub fn mapExp(
    mut stmt: metamodelica::Ref<NFStatement>,
    mut func: &dyn ::std::ops::Fn(
        metamodelica::Ref<Expression::NFExpression>,
    ) -> Result<metamodelica::Ref<Expression::NFExpression>>,
) -> Result<metamodelica::Ref<NFStatement>> {
    pub type MapFunc = std::sync::Arc<
        dyn ::std::ops::Fn(
                metamodelica::Ref<Expression::NFExpression>,
            ) -> Result<metamodelica::Ref<Expression::NFExpression>>
            + 'static,
    >;

    let mut stmt: metamodelica::Ref<NFStatement> = stmt;
    stmt = (match &*stmt {
        ASSIGNMENT {
            lhs: __stmt_lhs,
            rhs: __stmt_rhs,
            source: __stmt_source,
            ty: __stmt_ty,
        } => {
            let mut e1: metamodelica::Ref<Expression::NFExpression>;
            let mut e2: metamodelica::Ref<Expression::NFExpression>;
            e1 = func(__stmt_lhs.clone())?;
            e2 = func(__stmt_rhs.clone())?;
            if (referenceEq(&*(&*e1), &*(__stmt_lhs.clone())) && referenceEq(&*(&*e2), &*(__stmt_rhs.clone()))) {
                stmt
            } else {
                metamodelica::Ref::new(NFStatement::ASSIGNMENT {
                    lhs: e1,
                    rhs: e2,
                    ty: __stmt_ty.clone(),
                    source: __stmt_source.clone(),
                })
            }
        }
        FOR { body: __stmt_body, .. } => {
            assign_variant_field!(stmt => NFStatement::FOR;
                body = mapExpList(__stmt_body.clone(), func)?,
                range = Util::applyOption(var_field!((*stmt).range, NFStatement::FOR).clone(), func)?
            );
            stmt
        }
        IF {
            branches: __stmt_branches,
            ..
        } => {
            assign_variant_field!(stmt => NFStatement::IF; branches = ({
                let mut __acc: metamodelica::List<(metamodelica::Ref<Expression::NFExpression>, metamodelica::List<metamodelica::Ref<NFStatement>>)> = metamodelica::nil();
                for mut b in (__stmt_branches.clone()).into_iter().cloned() {
                    let __x = (func(Util::tuple21(b.clone()))?, mapExpList(Util::tuple22(b.clone()), func)?);
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            }));
            stmt
        }
        WHEN {
            branches: __stmt_branches,
            ..
        } => {
            assign_variant_field!(stmt => NFStatement::WHEN; branches = ({
                let mut __acc: metamodelica::List<(metamodelica::Ref<Expression::NFExpression>, metamodelica::List<metamodelica::Ref<NFStatement>>)> = metamodelica::nil();
                for mut b in (__stmt_branches.clone()).into_iter().cloned() {
                    let __x = (func(Util::tuple21(b.clone()))?, mapExpList(Util::tuple22(b.clone()), func)?);
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            }));
            stmt
        }
        ASSERT {
            condition: __stmt_condition,
            level: __stmt_level,
            message: __stmt_message,
            source: __stmt_source,
        } => {
            let mut e1: metamodelica::Ref<Expression::NFExpression>;
            let mut e2: metamodelica::Ref<Expression::NFExpression>;
            let mut e3: metamodelica::Ref<Expression::NFExpression>;
            e1 = func(__stmt_condition.clone())?;
            e2 = func(__stmt_message.clone())?;
            e3 = func(__stmt_level.clone())?;
            if (referenceEq(&*(&*e1), &*(__stmt_condition.clone()))
                && referenceEq(&*(&*e2), &*(__stmt_message.clone()))
                && referenceEq(&*(&*e3), &*(__stmt_level.clone())))
            {
                stmt
            } else {
                metamodelica::Ref::new(NFStatement::ASSERT {
                    condition: e1,
                    message: e2,
                    level: e3,
                    source: __stmt_source.clone(),
                })
            }
        }
        TERMINATE {
            message: __stmt_message,
            source: __stmt_source,
        } => {
            let mut e1: metamodelica::Ref<Expression::NFExpression>;
            e1 = func(__stmt_message.clone())?;
            if (referenceEq(&*(&*e1), &*(__stmt_message.clone()))) {
                stmt
            } else {
                metamodelica::Ref::new(NFStatement::TERMINATE {
                    message: e1,
                    source: __stmt_source.clone(),
                })
            }
        }
        REINIT {
            cref: __stmt_cref,
            reinitExp: __stmt_reinitExp,
            source: __stmt_source,
        } => {
            let mut e1: metamodelica::Ref<Expression::NFExpression>;
            let mut e2: metamodelica::Ref<Expression::NFExpression>;
            e1 = func(__stmt_cref.clone())?;
            e2 = func(__stmt_reinitExp.clone())?;
            if (referenceEq(&*(&*e1), &*(__stmt_cref.clone())) && referenceEq(&*(&*e2), &*(__stmt_reinitExp.clone()))) {
                stmt
            } else {
                metamodelica::Ref::new(NFStatement::REINIT {
                    cref: e1,
                    reinitExp: e2,
                    source: __stmt_source.clone(),
                })
            }
        }
        NORETCALL {
            exp: __stmt_exp,
            source: __stmt_source,
        } => {
            let mut e1: metamodelica::Ref<Expression::NFExpression>;
            e1 = func(__stmt_exp.clone())?;
            if (referenceEq(&*(&*e1), &*(__stmt_exp.clone()))) {
                stmt
            } else {
                metamodelica::Ref::new(NFStatement::NORETCALL {
                    exp: e1,
                    source: __stmt_source.clone(),
                })
            }
        }
        WHILE {
            body: __stmt_body,
            condition: __stmt_condition,
            source: __stmt_source,
        } => metamodelica::Ref::new(NFStatement::WHILE {
            condition: func(__stmt_condition.clone())?,
            body: mapExpList(__stmt_body.clone(), func)?,
            source: __stmt_source.clone(),
        }),
        _ => stmt,
    });
    Ok(stmt)
}

pub(crate) fn mapExpShallow(
    mut stmt: metamodelica::Ref<NFStatement>,
    mut func: &dyn ::std::ops::Fn(
        metamodelica::Ref<Expression::NFExpression>,
    ) -> Result<metamodelica::Ref<Expression::NFExpression>>,
) -> Result<metamodelica::Ref<NFStatement>> {
    pub type MapFunc = std::sync::Arc<
        dyn ::std::ops::Fn(
                metamodelica::Ref<Expression::NFExpression>,
            ) -> Result<metamodelica::Ref<Expression::NFExpression>>
            + 'static,
    >;

    let mut stmt: metamodelica::Ref<NFStatement> = stmt;
    stmt = (match &*stmt {
        ASSIGNMENT {
            lhs: __stmt_lhs,
            rhs: __stmt_rhs,
            source: __stmt_source,
            ty: __stmt_ty,
        } => {
            let mut e1: metamodelica::Ref<Expression::NFExpression>;
            let mut e2: metamodelica::Ref<Expression::NFExpression>;
            e1 = func(__stmt_lhs.clone())?;
            e2 = func(__stmt_rhs.clone())?;
            if (referenceEq(&*(&*e1), &*(__stmt_lhs.clone())) && referenceEq(&*(&*e2), &*(__stmt_rhs.clone()))) {
                stmt
            } else {
                metamodelica::Ref::new(NFStatement::ASSIGNMENT {
                    lhs: e1,
                    rhs: e2,
                    ty: __stmt_ty.clone(),
                    source: __stmt_source.clone(),
                })
            }
        }
        FOR {
            range: __stmt_range, ..
        } => {
            assign_variant_field!(stmt => NFStatement::FOR; range = Util::applyOption(__stmt_range.clone(), func)?);
            stmt
        }
        IF {
            branches: __stmt_branches,
            ..
        } => {
            assign_variant_field!(stmt => NFStatement::IF; branches = ({
                let mut __acc: metamodelica::List<(metamodelica::Ref<Expression::NFExpression>, metamodelica::List<metamodelica::Ref<NFStatement>>)> = metamodelica::nil();
                for mut b in (__stmt_branches.clone()).into_iter().cloned() {
                    let __x = (func(Util::tuple21(b.clone()))?, Util::tuple22(b.clone()));
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            }));
            stmt
        }
        WHEN {
            branches: __stmt_branches,
            ..
        } => {
            assign_variant_field!(stmt => NFStatement::WHEN; branches = ({
                let mut __acc: metamodelica::List<(metamodelica::Ref<Expression::NFExpression>, metamodelica::List<metamodelica::Ref<NFStatement>>)> = metamodelica::nil();
                for mut b in (__stmt_branches.clone()).into_iter().cloned() {
                    let __x = (func(Util::tuple21(b.clone()))?, Util::tuple22(b.clone()));
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            }));
            stmt
        }
        ASSERT {
            condition: __stmt_condition,
            level: __stmt_level,
            message: __stmt_message,
            source: __stmt_source,
        } => {
            let mut e1: metamodelica::Ref<Expression::NFExpression>;
            let mut e2: metamodelica::Ref<Expression::NFExpression>;
            let mut e3: metamodelica::Ref<Expression::NFExpression>;
            e1 = func(__stmt_condition.clone())?;
            e2 = func(__stmt_message.clone())?;
            e3 = func(__stmt_level.clone())?;
            if (referenceEq(&*(&*e1), &*(__stmt_condition.clone()))
                && referenceEq(&*(&*e2), &*(__stmt_message.clone()))
                && referenceEq(&*(&*e3), &*(__stmt_level.clone())))
            {
                stmt
            } else {
                metamodelica::Ref::new(NFStatement::ASSERT {
                    condition: e1,
                    message: e2,
                    level: e3,
                    source: __stmt_source.clone(),
                })
            }
        }
        TERMINATE {
            message: __stmt_message,
            source: __stmt_source,
        } => {
            let mut e1: metamodelica::Ref<Expression::NFExpression>;
            e1 = func(__stmt_message.clone())?;
            if (referenceEq(&*(&*e1), &*(__stmt_message.clone()))) {
                stmt
            } else {
                metamodelica::Ref::new(NFStatement::TERMINATE {
                    message: e1,
                    source: __stmt_source.clone(),
                })
            }
        }
        REINIT {
            cref: __stmt_cref,
            reinitExp: __stmt_reinitExp,
            source: __stmt_source,
        } => {
            let mut e1: metamodelica::Ref<Expression::NFExpression>;
            let mut e2: metamodelica::Ref<Expression::NFExpression>;
            e1 = func(__stmt_cref.clone())?;
            e2 = func(__stmt_reinitExp.clone())?;
            if (referenceEq(&*(&*e1), &*(__stmt_cref.clone())) && referenceEq(&*(&*e2), &*(__stmt_reinitExp.clone()))) {
                stmt
            } else {
                metamodelica::Ref::new(NFStatement::REINIT {
                    cref: e1,
                    reinitExp: e2,
                    source: __stmt_source.clone(),
                })
            }
        }
        NORETCALL {
            exp: __stmt_exp,
            source: __stmt_source,
        } => {
            let mut e1: metamodelica::Ref<Expression::NFExpression>;
            e1 = func(__stmt_exp.clone())?;
            if (referenceEq(&*(&*e1), &*(__stmt_exp.clone()))) {
                stmt
            } else {
                metamodelica::Ref::new(NFStatement::NORETCALL {
                    exp: e1,
                    source: __stmt_source.clone(),
                })
            }
        }
        WHILE {
            body: __stmt_body,
            condition: __stmt_condition,
            source: __stmt_source,
        } => metamodelica::Ref::new(NFStatement::WHILE {
            condition: func(__stmt_condition.clone())?,
            body: __stmt_body.clone(),
            source: __stmt_source.clone(),
        }),
        _ => stmt,
    });
    Ok(stmt)
}

pub(crate) fn foldExpList<ArgT: Clone + 'static + metamodelica::gc::MMTrace>(
    mut stmt: &metamodelica::List<metamodelica::Ref<NFStatement>>,
    mut func: &dyn ::std::ops::Fn(metamodelica::Ref<Expression::NFExpression>, ArgT) -> Result<ArgT>,
    mut arg: ArgT,
) -> Result<ArgT> {
    pub type FoldFunc<ArgT: Clone + 'static> =
        std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<Expression::NFExpression>, ArgT) -> Result<ArgT> + 'static>;

    let mut arg: ArgT = arg;
    for mut s in &**stmt {
        arg = foldExp(metamodelica::AsArg::as_arg(&s), func, arg)?;
    }
    Ok(arg)
}

pub(crate) fn foldExp<ArgT: Clone + 'static + metamodelica::gc::MMTrace>(
    mut stmt: &metamodelica::Ref<NFStatement>,
    mut func: &dyn ::std::ops::Fn(metamodelica::Ref<Expression::NFExpression>, ArgT) -> Result<ArgT>,
    mut arg: ArgT,
) -> Result<ArgT> {
    pub type FoldFunc<ArgT: Clone + 'static> =
        std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<Expression::NFExpression>, ArgT) -> Result<ArgT> + 'static>;

    let mut arg: ArgT = arg;
    let () = (match &**stmt {
        ASSIGNMENT {
            lhs: __stmt_lhs,
            rhs: __stmt_rhs,
            ..
        } => {
            arg = func(__stmt_lhs.clone(), arg)?;
            arg = func(__stmt_rhs.clone(), arg)?;
            ()
        }
        FOR {
            body: __stmt_body,
            range: __stmt_range,
            ..
        } => {
            arg = foldExpList(metamodelica::AsArg::as_arg(&__stmt_body), func, arg)?;
            if (__stmt_range).is_some() {
                arg = func(Util::getOption(__stmt_range.clone())?, arg)?;
            }
            ()
        }
        IF {
            branches: __stmt_branches,
            ..
        } => {
            for mut b in &*__stmt_branches.clone() {
                arg = func(Util::tuple21(b.clone()), arg)?;
                arg = foldExpList(&(Util::tuple22(b.clone())), func, arg)?;
            }
            ()
        }
        WHEN {
            branches: __stmt_branches,
            ..
        } => {
            for mut b in &*__stmt_branches.clone() {
                arg = func(Util::tuple21(b.clone()), arg)?;
                arg = foldExpList(&(Util::tuple22(b.clone())), func, arg)?;
            }
            ()
        }
        ASSERT {
            condition: __stmt_condition,
            level: __stmt_level,
            message: __stmt_message,
            ..
        } => {
            arg = func(__stmt_condition.clone(), arg)?;
            arg = func(__stmt_message.clone(), arg)?;
            arg = func(__stmt_level.clone(), arg)?;
            ()
        }
        TERMINATE {
            message: __stmt_message,
            ..
        } => {
            arg = func(__stmt_message.clone(), arg)?;
            ()
        }
        REINIT {
            cref: __stmt_cref,
            reinitExp: __stmt_reinitExp,
            ..
        } => {
            arg = func(__stmt_cref.clone(), arg)?;
            arg = func(__stmt_reinitExp.clone(), arg)?;
            ()
        }
        NORETCALL { exp: __stmt_exp, .. } => {
            arg = func(__stmt_exp.clone(), arg)?;
            ()
        }
        WHILE {
            body: __stmt_body,
            condition: __stmt_condition,
            ..
        } => {
            arg = func(__stmt_condition.clone(), arg)?;
            arg = foldExpList(metamodelica::AsArg::as_arg(&__stmt_body), func, arg)?;
            ()
        }
        _ => (),
    });
    Ok(arg)
}

pub fn contains(
    mut stmt: metamodelica::Ref<NFStatement>,
    mut r#fn: &dyn ::std::ops::Fn(metamodelica::Ref<NFStatement>) -> Result<bool>,
) -> Result<bool> {
    pub type PredFn = std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<NFStatement>) -> Result<bool> + 'static>;

    let mut res: bool;
    if r#fn(stmt.clone())? {
        res = true;
        return Ok(res);
    }
    res = (match &*stmt {
        FOR { body: __stmt_body, .. } => containsList(metamodelica::AsArg::as_arg(&__stmt_body), r#fn)?,
        IF {
            branches: __stmt_branches,
            ..
        } => {
            for mut b in &*__stmt_branches.clone() {
                if containsList(&(Util::tuple22(b.clone())), r#fn)? {
                    res = true;
                    return Ok(res);
                }
            }
            false
        }
        WHEN {
            branches: __stmt_branches,
            ..
        } => {
            for mut b in &*__stmt_branches.clone() {
                if containsList(&(Util::tuple22(b.clone())), r#fn)? {
                    res = true;
                    return Ok(res);
                }
            }
            false
        }
        WHILE { body: __stmt_body, .. } => containsList(metamodelica::AsArg::as_arg(&__stmt_body), r#fn)?,
        _ => false,
    });
    Ok(res)
}

pub(crate) fn containsList(
    mut eql: &metamodelica::List<metamodelica::Ref<NFStatement>>,
    mut func: &dyn ::std::ops::Fn(metamodelica::Ref<NFStatement>) -> Result<bool>,
) -> Result<bool> {
    pub type PredFn = std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<NFStatement>) -> Result<bool> + 'static>;

    let mut res: bool;
    for mut eq in &**eql {
        if contains(eq.clone(), func)? {
            res = true;
            return Ok(res);
        }
    }
    res = false;
    Ok(res)
}

pub(crate) fn replaceIteratorList(
    mut stmtl: metamodelica::List<metamodelica::Ref<NFStatement>>,
    mut iterator: &metamodelica::Ref<InstNode::InstNode>,
    mut value: &metamodelica::Ref<Expression::NFExpression>,
) -> Result<metamodelica::List<metamodelica::Ref<NFStatement>>> {
    let mut stmtl: metamodelica::List<metamodelica::Ref<NFStatement>> = stmtl;
    stmtl = mapExpList(
        stmtl,
        &({
            let __pe_b1 = iterator.clone();
            let __pe_b2 = value.clone();
            move |__pe_a0| Expression::replaceIterator(__pe_a0, &__pe_b1, &__pe_b2)
        }),
    )?;
    Ok(stmtl)
}

pub fn toString(mut stmt: &metamodelica::Ref<NFStatement>, mut indent: ArcStr) -> Result<ArcStr> {
    let mut r#str: ArcStr;
    let mut s: IOStream::IOStream;
    s = IOStream::create(
        literal!("NFStatement.toString"),
        openmodelica_util::IOStream::IOStreamType::LIST,
    )?;
    s = toStream(stmt, indent, s)?;
    r#str = IOStream::string(&s)?;
    IOStream::delete(&s)?;
    Ok(r#str)
}

pub fn toStringList(
    mut stmtl: &metamodelica::List<metamodelica::Ref<NFStatement>>,
    mut indent: ArcStr,
) -> Result<ArcStr> {
    let mut r#str: ArcStr;
    let mut s: IOStream::IOStream;
    s = IOStream::create(
        literal!("NFStatement.toStringList"),
        openmodelica_util::IOStream::IOStreamType::LIST,
    )?;
    s = toStreamList(stmtl, indent, s)?;
    r#str = IOStream::string(&s)?;
    IOStream::delete(&s)?;
    Ok(r#str)
}

pub(crate) fn toStream(
    mut stmt: &metamodelica::Ref<NFStatement>,
    mut indent: ArcStr,
    mut s: IOStream::IOStream,
) -> Result<IOStream::IOStream> {
    let mut s: IOStream::IOStream = s;
    let mut branches: metamodelica::List<(
        metamodelica::Ref<Expression::NFExpression>,
        metamodelica::List<metamodelica::Ref<NFStatement>>,
    )>;
    let mut cond: metamodelica::Ref<Expression::NFExpression>;
    let mut body: metamodelica::List<metamodelica::Ref<NFStatement>>;
    let mut first: bool;
    s = IOStream::append(s, indent.clone())?;
    s = (match &**stmt {
        ASSIGNMENT {
            lhs: __stmt_lhs,
            rhs: __stmt_rhs,
            ..
        } => {
            s = IOStream::append(s, Expression::toString(__stmt_lhs.clone())?)?;
            s = IOStream::append(s, literal!(" := "))?;
            s = IOStream::append(s, Expression::toString(__stmt_rhs.clone())?)?;
            s
        }
        FUNCTION_ARRAY_INIT { name: __stmt_name, .. } => {
            s = IOStream::append(s, literal!("array init"))?;
            s = IOStream::append(s, __stmt_name.clone())?;
            s
        }
        FOR {
            body: __stmt_body,
            iterator: __stmt_iterator,
            range: __stmt_range,
            ..
        } => {
            s = IOStream::append(s, literal!("for "))?;
            s = IOStream::append(s, InstNode::name(metamodelica::AsArg::as_arg(&__stmt_iterator))?)?;
            if (__stmt_range).is_some() {
                s = IOStream::append(s, literal!(" in "))?;
                s = IOStream::append(s, Expression::toString(Util::getOption(__stmt_range.clone())?)?)?;
            }
            s = IOStream::append(s, literal!(" loop\n"))?;
            s = toStreamList(
                metamodelica::AsArg::as_arg(&__stmt_body),
                {
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*indent);
                    __mm_s.push_str(&*literal!("  "));
                    ArcStr::from(__mm_s)
                },
                s,
            )?;
            s = IOStream::append(s, indent)?;
            s = IOStream::append(s, literal!("end for"))?;
            s
        }
        IF {
            branches: __stmt_branches,
            ..
        } => {
            first = true;
            branches = __stmt_branches.clone();
            while !((branches).is_empty()) {
                let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(branches) {
                    Deref @ metamodelica::ListNode::Cons { head: (__pa0, __pa1), tail: __pa2 } => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
                    _ => return Err("pattern mismatch"),
                } };
                cond = metamodelica::Own::own(__pa0);
                body = metamodelica::Own::own(__pa1);
                branches = metamodelica::Own::own(__pa2);
                if !(first) && (branches).is_empty() && Expression::isTrue(&cond) {
                    s = IOStream::append(s, literal!("else\n"))?;
                } else {
                    s = IOStream::append(s, if (first) { literal!("if ") } else { literal!("elseif ") })?;
                    s = IOStream::append(s, Expression::toString(cond)?)?;
                    s = IOStream::append(s, literal!(" then\n"))?;
                }
                s = toStreamList(
                    &body,
                    {
                        let mut __mm_s = String::new();
                        __mm_s.push_str(&*indent);
                        __mm_s.push_str(&*literal!("  "));
                        ArcStr::from(__mm_s)
                    },
                    s,
                )?;
                s = IOStream::append(s, indent.clone())?;
                first = false;
            }
            s = IOStream::append(s, literal!("end if"))?;
            s
        }
        WHEN {
            branches: __stmt_branches,
            ..
        } => {
            first = true;
            for mut b in &*__stmt_branches.clone() {
                (cond, body) = b.clone();
                s = IOStream::append(
                    s,
                    if (first) {
                        literal!("when ")
                    } else {
                        literal!("elsewhen ")
                    },
                )?;
                s = IOStream::append(s, Expression::toString(cond)?)?;
                s = IOStream::append(s, literal!(" then\n"))?;
                s = toStreamList(
                    &body,
                    {
                        let mut __mm_s = String::new();
                        __mm_s.push_str(&*indent);
                        __mm_s.push_str(&*literal!("  "));
                        ArcStr::from(__mm_s)
                    },
                    s,
                )?;
                s = IOStream::append(s, indent.clone())?;
                first = false;
            }
            s = IOStream::append(s, literal!("end when"))?;
            s
        }
        ASSERT {
            condition: __stmt_condition,
            level: __stmt_level,
            message: __stmt_message,
            ..
        } => {
            s = IOStream::append(s, literal!("assert("))?;
            s = IOStream::append(s, Expression::toString(__stmt_condition.clone())?)?;
            s = IOStream::append(s, literal!(", "))?;
            s = IOStream::append(s, Expression::toString(__stmt_message.clone())?)?;
            s = IOStream::append(s, literal!(", "))?;
            s = IOStream::append(s, Expression::toString(__stmt_level.clone())?)?;
            s = IOStream::append(s, literal!(")"))?;
            s
        }
        TERMINATE {
            message: __stmt_message,
            ..
        } => {
            s = IOStream::append(s, literal!("terminate("))?;
            s = IOStream::append(s, Expression::toString(__stmt_message.clone())?)?;
            s = IOStream::append(s, literal!(")"))?;
            s
        }
        REINIT {
            cref: __stmt_cref,
            reinitExp: __stmt_reinitExp,
            ..
        } => {
            s = IOStream::append(s, literal!("reinit("))?;
            s = IOStream::append(s, Expression::toString(__stmt_cref.clone())?)?;
            s = IOStream::append(s, literal!(", "))?;
            s = IOStream::append(s, Expression::toString(__stmt_reinitExp.clone())?)?;
            s = IOStream::append(s, literal!(")"))?;
            s
        }
        NORETCALL { exp: __stmt_exp, .. } => IOStream::append(s, Expression::toString(__stmt_exp.clone())?)?,
        WHILE {
            body: __stmt_body,
            condition: __stmt_condition,
            ..
        } => {
            s = IOStream::append(s, literal!("while "))?;
            s = IOStream::append(s, Expression::toString(__stmt_condition.clone())?)?;
            s = IOStream::append(s, literal!(" then\n"))?;
            s = toStreamList(
                metamodelica::AsArg::as_arg(&__stmt_body),
                {
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*indent);
                    __mm_s.push_str(&*literal!("  "));
                    ArcStr::from(__mm_s)
                },
                s,
            )?;
            s = IOStream::append(s, indent)?;
            s = IOStream::append(s, literal!("end while"))?;
            s
        }
        RETURN { .. } => IOStream::append(s, literal!("return"))?,
        RETURN { .. } => IOStream::append(s, literal!("break"))?,
        _ => IOStream::append(s, literal!("#UNKNOWN STATEMENT#"))?,
    });
    Ok(s)
}

pub(crate) fn toStreamList(
    mut stmtl: &metamodelica::List<metamodelica::Ref<NFStatement>>,
    mut indent: ArcStr,
    mut s: IOStream::IOStream,
) -> Result<IOStream::IOStream> {
    let mut s: IOStream::IOStream = s;
    let mut prev_multi_line: bool = false;
    let mut multi_line: bool;
    let mut first: bool = true;
    for mut stmt in &**stmtl {
        multi_line = isMultiLine(metamodelica::AsArg::as_arg(&stmt));
        if first {
            first = false;
        } else if prev_multi_line || multi_line {
            s = IOStream::append(s, literal!("\n"))?;
        }
        prev_multi_line = multi_line;
        s = toStream(metamodelica::AsArg::as_arg(&stmt), indent.clone(), s)?;
        s = IOStream::append(s, literal!(";\n"))?;
    }
    Ok(s)
}

pub(crate) fn toFlatStream(
    mut stmt: &metamodelica::Ref<NFStatement>,
    mut format: BaseModelica::OutputFormat,
    mut indent: ArcStr,
    mut s: IOStream::IOStream,
) -> Result<IOStream::IOStream> {
    let mut s: IOStream::IOStream = s;
    let mut branches: metamodelica::List<(
        metamodelica::Ref<Expression::NFExpression>,
        metamodelica::List<metamodelica::Ref<NFStatement>>,
    )>;
    let mut cond: metamodelica::Ref<Expression::NFExpression>;
    let mut body: metamodelica::List<metamodelica::Ref<NFStatement>>;
    let mut first: bool;
    s = IOStream::append(s, indent.clone())?;
    s = (match &**stmt {
        ASSIGNMENT {
            lhs: __stmt_lhs,
            rhs: __stmt_rhs,
            ..
        } => {
            s = IOStream::append(s, Expression::toFlatString(__stmt_lhs.clone(), format)?)?;
            s = IOStream::append(s, literal!(" := "))?;
            s = IOStream::append(s, Expression::toFlatString(__stmt_rhs.clone(), format)?)?;
            s
        }
        FUNCTION_ARRAY_INIT { name: __stmt_name, .. } => {
            s = IOStream::append(s, literal!("array init"))?;
            s = IOStream::append(s, __stmt_name.clone())?;
            s
        }
        FOR {
            body: __stmt_body,
            iterator: __stmt_iterator,
            range: __stmt_range,
            ..
        } => {
            s = IOStream::append(s, literal!("for "))?;
            s = IOStream::append(
                s,
                Util::makeQuotedIdentifier(InstNode::name(metamodelica::AsArg::as_arg(&__stmt_iterator))?)?,
            )?;
            if (__stmt_range).is_some() {
                s = IOStream::append(s, literal!(" in "))?;
                s = IOStream::append(
                    s,
                    Expression::toFlatString(Util::getOption(__stmt_range.clone())?, format)?,
                )?;
            }
            s = IOStream::append(s, literal!(" loop\n"))?;
            s = toFlatStreamList(
                metamodelica::AsArg::as_arg(&__stmt_body),
                format,
                {
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*indent);
                    __mm_s.push_str(&*literal!("  "));
                    ArcStr::from(__mm_s)
                },
                s,
            )?;
            s = IOStream::append(s, indent)?;
            s = IOStream::append(s, literal!("end for"))?;
            s
        }
        IF {
            branches: __stmt_branches,
            ..
        } => {
            first = true;
            branches = __stmt_branches.clone();
            while !((branches).is_empty()) {
                let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(branches) {
                    Deref @ metamodelica::ListNode::Cons { head: (__pa0, __pa1), tail: __pa2 } => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
                    _ => return Err("pattern mismatch"),
                } };
                cond = metamodelica::Own::own(__pa0);
                body = metamodelica::Own::own(__pa1);
                branches = metamodelica::Own::own(__pa2);
                if !(first) && (branches).is_empty() && Expression::isTrue(&cond) {
                    s = IOStream::append(s, literal!("else\n"))?;
                } else {
                    s = IOStream::append(s, if (first) { literal!("if ") } else { literal!("elseif ") })?;
                    s = IOStream::append(s, Expression::toFlatString(cond, format)?)?;
                    s = IOStream::append(s, literal!(" then\n"))?;
                }
                s = toFlatStreamList(
                    &body,
                    format,
                    {
                        let mut __mm_s = String::new();
                        __mm_s.push_str(&*indent);
                        __mm_s.push_str(&*literal!("  "));
                        ArcStr::from(__mm_s)
                    },
                    s,
                )?;
                s = IOStream::append(s, indent.clone())?;
                first = false;
            }
            s = IOStream::append(s, literal!("end if"))?;
            s
        }
        WHEN {
            branches: __stmt_branches,
            ..
        } => {
            first = true;
            for mut b in &*__stmt_branches.clone() {
                (cond, body) = b.clone();
                s = IOStream::append(
                    s,
                    if (first) {
                        literal!("when ")
                    } else {
                        literal!("elsewhen ")
                    },
                )?;
                s = IOStream::append(s, Expression::toFlatString(cond, format)?)?;
                s = IOStream::append(s, literal!(" then\n"))?;
                s = toFlatStreamList(
                    &body,
                    format,
                    {
                        let mut __mm_s = String::new();
                        __mm_s.push_str(&*indent);
                        __mm_s.push_str(&*literal!("  "));
                        ArcStr::from(__mm_s)
                    },
                    s,
                )?;
                s = IOStream::append(s, indent.clone())?;
                first = false;
            }
            s = IOStream::append(s, literal!("end when"))?;
            s
        }
        ASSERT {
            condition: __stmt_condition,
            level: __stmt_level,
            message: __stmt_message,
            ..
        } => {
            s = IOStream::append(s, literal!("assert("))?;
            s = IOStream::append(s, Expression::toFlatString(__stmt_condition.clone(), format)?)?;
            s = IOStream::append(s, literal!(", "))?;
            s = IOStream::append(s, Expression::toFlatString(__stmt_message.clone(), format)?)?;
            s = IOStream::append(s, literal!(", "))?;
            s = IOStream::append(s, Expression::toFlatString(__stmt_level.clone(), format)?)?;
            s = IOStream::append(s, literal!(")"))?;
            s
        }
        TERMINATE {
            message: __stmt_message,
            ..
        } => {
            s = IOStream::append(s, literal!("terminate("))?;
            s = IOStream::append(s, Expression::toFlatString(__stmt_message.clone(), format)?)?;
            s = IOStream::append(s, literal!(")"))?;
            s
        }
        REINIT {
            cref: __stmt_cref,
            reinitExp: __stmt_reinitExp,
            ..
        } => {
            s = IOStream::append(s, literal!("reinit("))?;
            s = IOStream::append(s, Expression::toFlatString(__stmt_cref.clone(), format)?)?;
            s = IOStream::append(s, literal!(", "))?;
            s = IOStream::append(s, Expression::toFlatString(__stmt_reinitExp.clone(), format)?)?;
            s = IOStream::append(s, literal!(")"))?;
            s
        }
        NORETCALL { exp: __stmt_exp, .. } => {
            IOStream::append(s, Expression::toFlatString(__stmt_exp.clone(), format)?)?
        }
        WHILE {
            body: __stmt_body,
            condition: __stmt_condition,
            ..
        } => {
            s = IOStream::append(s, literal!("while "))?;
            s = IOStream::append(s, Expression::toFlatString(__stmt_condition.clone(), format)?)?;
            s = IOStream::append(s, literal!(" loop\n"))?;
            s = toFlatStreamList(
                metamodelica::AsArg::as_arg(&__stmt_body),
                format,
                {
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*indent);
                    __mm_s.push_str(&*literal!("  "));
                    ArcStr::from(__mm_s)
                },
                s,
            )?;
            s = IOStream::append(s, indent)?;
            s = IOStream::append(s, literal!("end while"))?;
            s
        }
        RETURN { .. } => IOStream::append(s, literal!("return"))?,
        BREAK { .. } => IOStream::append(s, literal!("break"))?,
        _ => IOStream::append(s, literal!("#UNKNOWN STATEMENT#"))?,
    });
    s = FlatModelicaUtil::appendElementSourceComment(
        &(source(stmt)),
        FlatModelicaUtil::ElementType::ALGORITHM.clone(),
        s,
    )?;
    Ok(s)
}

pub(crate) fn toFlatStreamList(
    mut stmtl: &metamodelica::List<metamodelica::Ref<NFStatement>>,
    mut format: BaseModelica::OutputFormat,
    mut indent: ArcStr,
    mut s: IOStream::IOStream,
) -> Result<IOStream::IOStream> {
    let mut s: IOStream::IOStream = s;
    let mut prev_multi_line: bool = false;
    let mut multi_line: bool;
    let mut first: bool = true;
    for mut stmt in &**stmtl {
        multi_line = isMultiLine(metamodelica::AsArg::as_arg(&stmt));
        if first {
            first = false;
        } else if prev_multi_line || multi_line {
            s = IOStream::append(s, literal!("\n"))?;
        }
        prev_multi_line = multi_line;
        s = toFlatStream(metamodelica::AsArg::as_arg(&stmt), format, indent.clone(), s)?;
        s = IOStream::append(s, literal!(";\n"))?;
    }
    Ok(s)
}

pub(crate) fn isMultiLine(mut stmt: &metamodelica::Ref<NFStatement>) -> bool {
    let mut multiLine: bool;
    multiLine = (match &**stmt {
        FOR { .. } => true,
        IF { .. } => true,
        WHEN { .. } => true,
        WHILE { .. } => true,
        _ => false,
    });
    multiLine
}
