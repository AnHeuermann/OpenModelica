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

use crate::ComponentReference;
use crate::DAEUtil;
use crate::ExpressionDump;
use crate::ExpressionSimplify;
use crate::Types;
use openmodelica_ast::Absyn;
use openmodelica_frontend_dump::AbsynUtil;
use openmodelica_frontend_dump::AvlTreePathFunction;
use openmodelica_frontend_dump::ComponentReferenceBasics;
use openmodelica_frontend_dump::Dump;
use openmodelica_frontend_dump::ExpressionBasics;
use openmodelica_frontend_dump::ExpressionBasics::printExpStr;
use openmodelica_frontend_dump::TypesDump;
use openmodelica_frontend_types::ClassInf;
use openmodelica_frontend_types::DAE;
use openmodelica_util::Config;
use openmodelica_util::Debug;
use openmodelica_util::Error;
use openmodelica_util::Flags;
use openmodelica_util::System;
use openmodelica_util::Util;
use openmodelica_util_datatypes_basic::Array;
use openmodelica_util_datatypes_basic::DoubleEnded;
use openmodelica_util_datatypes_basic::List;

// public imports
pub type ComponentRef = metamodelica::Ref<DAE::ComponentRef>;

pub type Exp = metamodelica::Ref<DAE::Exp>;

pub type Operator = DAE::Operator;

pub type Type = metamodelica::Ref<DAE::Type>;

pub type Subscript = metamodelica::Ref<DAE::Subscript>;

pub type Var = metamodelica::Ref<DAE::Var>;

// protected imports
// stringReal
pub(crate) const MAX_SUM_CHAIN: i32 = 32;

/* **************************************************/
/* transform to other types */
/* **************************************************/
pub fn intSubscript(mut inInteger: i32) -> metamodelica::Ref<DAE::Subscript> {
    let mut outSubscript: metamodelica::Ref<DAE::Subscript>;
    outSubscript = metamodelica::Ref::new(DAE::Subscript::INDEX {
        exp: metamodelica::Ref::new(DAE::Exp::ICONST { integer: inInteger }),
    });
    outSubscript
}

pub(crate) fn intSubscripts(
    mut inIntegers: metamodelica::List<i32>,
) -> Result<metamodelica::List<metamodelica::Ref<DAE::Subscript>>> {
    let mut outSubscripts: metamodelica::List<metamodelica::Ref<DAE::Subscript>>;
    outSubscripts = List::map(inIntegers, &fnptr!(intSubscript, i32))?;
    Ok(outSubscripts)
}

pub fn dimensionIsZero(mut inDimension: &metamodelica::Ref<DAE::Dimension>) -> Result<bool> {
    let mut outIsZero: bool;
    outIsZero = 0 == dimensionSize(inDimension)?;
    Ok(outIsZero)
}

pub fn unelabExp(mut inExp: &metamodelica::Ref<DAE::Exp>) -> Result<metamodelica::Ref<Absyn::Exp>> {
    let mut outExp: metamodelica::Ref<Absyn::Exp>;
    outExp = 'mc: {
        let __mc_input = &**inExp;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::ICONST { integer: i } => {
                    Ok(metamodelica::Ref::new(Absyn::Exp::INTEGER { value: i.clone() }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::RCONST { real: r } => {
                    let mut s: ArcStr;
                    s = realString(r.clone());
                    Ok(metamodelica::Ref::new(Absyn::Exp::REAL { value: s.clone() }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::SCONST { string: s } => {
                    Ok(metamodelica::Ref::new(Absyn::Exp::STRING { value: s.clone() }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::BCONST { bool: b } => {
                    Ok(metamodelica::Ref::new(Absyn::Exp::BOOL { value: b.clone() }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::ENUM_LITERAL { name: path, .. } => {
                    let mut cr_1: metamodelica::Ref<Absyn::ComponentRef>;
                    cr_1 = AbsynUtil::pathToCref(metamodelica::AsArg::as_arg(&path));
                    Ok(metamodelica::Ref::new(Absyn::Exp::CREF { componentRef: cr_1.clone() }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::CREF { componentRef: cr, .. } => {
                    let mut cr_1: metamodelica::Ref<Absyn::ComponentRef>;
                    cr_1 = ComponentReference::unelabCref(metamodelica::AsArg::as_arg(&cr))?;
                    Ok(metamodelica::Ref::new(Absyn::Exp::CREF { componentRef: cr_1.clone() }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::BINARY { exp1: e1, operator: op, exp2: e2 } => {
                    let mut ae1: metamodelica::Ref<Absyn::Exp>;
                    let mut ae2: metamodelica::Ref<Absyn::Exp>;
                    let mut aop: Absyn::Operator;
                    aop = unelabOperator(metamodelica::AsArg::as_arg(&op))?;
                    ae1 = unelabExp(metamodelica::AsArg::as_arg(&e1))?;
                    ae2 = unelabExp(metamodelica::AsArg::as_arg(&e2))?;
                    Ok(metamodelica::Ref::new(Absyn::Exp::BINARY { exp1: ae1.clone(), op: aop, exp2: ae2.clone() }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::UNARY { operator: op, exp: e1 } => {
                    let mut ae1: metamodelica::Ref<Absyn::Exp>;
                    let mut aop: Absyn::Operator;
                    aop = unelabOperator(metamodelica::AsArg::as_arg(&op))?;
                    ae1 = unelabExp(metamodelica::AsArg::as_arg(&e1))?;
                    Ok(metamodelica::Ref::new(Absyn::Exp::UNARY { op: aop, exp: ae1.clone() }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::LBINARY { exp1: e1, operator: op, exp2: e2 } => {
                    let mut ae1: metamodelica::Ref<Absyn::Exp>;
                    let mut ae2: metamodelica::Ref<Absyn::Exp>;
                    let mut aop: Absyn::Operator;
                    aop = unelabOperator(metamodelica::AsArg::as_arg(&op))?;
                    ae1 = unelabExp(metamodelica::AsArg::as_arg(&e1))?;
                    ae2 = unelabExp(metamodelica::AsArg::as_arg(&e2))?;
                    Ok(metamodelica::Ref::new(Absyn::Exp::LBINARY { exp1: ae1.clone(), op: aop, exp2: ae2.clone() }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::LUNARY { operator: op, exp: e1 } => {
                    let mut ae1: metamodelica::Ref<Absyn::Exp>;
                    let mut aop: Absyn::Operator;
                    aop = unelabOperator(metamodelica::AsArg::as_arg(&op))?;
                    ae1 = unelabExp(metamodelica::AsArg::as_arg(&e1))?;
                    Ok(metamodelica::Ref::new(Absyn::Exp::LUNARY { op: aop, exp: ae1.clone() }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::RELATION { exp1: e1, operator: op, exp2: e2, .. } => {
                    let mut ae1: metamodelica::Ref<Absyn::Exp>;
                    let mut ae2: metamodelica::Ref<Absyn::Exp>;
                    let mut aop: Absyn::Operator;
                    aop = unelabOperator(metamodelica::AsArg::as_arg(&op))?;
                    ae1 = unelabExp(metamodelica::AsArg::as_arg(&e1))?;
                    ae2 = unelabExp(metamodelica::AsArg::as_arg(&e2))?;
                    Ok(metamodelica::Ref::new(Absyn::Exp::RELATION { exp1: ae1.clone(), op: aop, exp2: ae2.clone() }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::IFEXP { expCond: e1, expThen: e2, expElse: e3 } => {
                    let mut ae1: metamodelica::Ref<Absyn::Exp>;
                    let mut ae2: metamodelica::Ref<Absyn::Exp>;
                    let mut ae3: metamodelica::Ref<Absyn::Exp>;
                    ae1 = unelabExp(metamodelica::AsArg::as_arg(&e1))?;
                    ae2 = unelabExp(metamodelica::AsArg::as_arg(&e2))?;
                    ae3 = unelabExp(metamodelica::AsArg::as_arg(&e3))?;
                    Ok(metamodelica::Ref::new(Absyn::Exp::IFEXP { ifExp: ae1.clone(), trueBranch: ae2.clone(), elseBranch: ae3.clone(), elseIfBranch: metamodelica::nil() }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::CALL { path, expLst: expl, attr: _ } => {
                    let mut aexpl: metamodelica::List<metamodelica::Ref<Absyn::Exp>>;
                    let mut acref: metamodelica::Ref<Absyn::ComponentRef>;
                    aexpl = List::map(expl.clone(), &move |__a0: metamodelica::Ref<DAE::Exp>| unelabExp(&__a0))?;
                    acref = AbsynUtil::pathToCref(metamodelica::AsArg::as_arg(&path));
                    Ok(metamodelica::Ref::new(Absyn::Exp::CALL { function_: acref.clone(), functionArgs: metamodelica::Ref::new(Absyn::FunctionArgs::FUNCTIONARGS { args: aexpl.clone(), argNames: metamodelica::nil() }), typeVars: metamodelica::nil() }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::RECORD { path, exps: expl, .. } => {
                    let mut aexpl: metamodelica::List<metamodelica::Ref<Absyn::Exp>>;
                    let mut acref: metamodelica::Ref<Absyn::ComponentRef>;
                    aexpl = List::map(expl.clone(), &move |__a0: metamodelica::Ref<DAE::Exp>| unelabExp(&__a0))?;
                    acref = AbsynUtil::pathToCref(metamodelica::AsArg::as_arg(&path));
                    Ok(metamodelica::Ref::new(Absyn::Exp::CALL { function_: acref.clone(), functionArgs: metamodelica::Ref::new(Absyn::FunctionArgs::FUNCTIONARGS { args: aexpl.clone(), argNames: metamodelica::nil() }), typeVars: metamodelica::nil() }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::PARTEVALFUNCTION { path, expList: expl, ty: _, origType: _ } => {
                    let mut aexpl: metamodelica::List<metamodelica::Ref<Absyn::Exp>>;
                    let mut acref: metamodelica::Ref<Absyn::ComponentRef>;
                    aexpl = List::map(expl.clone(), &move |__a0: metamodelica::Ref<DAE::Exp>| unelabExp(&__a0))?;
                    acref = AbsynUtil::pathToCref(metamodelica::AsArg::as_arg(&path));
                    Ok(metamodelica::Ref::new(Absyn::Exp::PARTEVALFUNCTION { function_: acref.clone(), functionArgs: metamodelica::Ref::new(Absyn::FunctionArgs::FUNCTIONARGS { args: aexpl.clone(), argNames: metamodelica::nil() }) }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::ARRAY { array: Deref @ metamodelica::ListNode::Nil, ty, .. } => {
                    let mut expl_1: metamodelica::List<metamodelica::Ref<Absyn::Exp>>;
                    let mut ae1: metamodelica::Ref<Absyn::Exp>;
                    let mut dims: metamodelica::List<metamodelica::Ref<DAE::Dimension>>;
                    let mut ty = (*ty).clone();
                    (ty, dims) = TypesDump::flattenArrayType(metamodelica::AsArg::as_arg(&ty));
                    ae1 = unleabZeroExpFromType(metamodelica::AsArg::as_arg(&ty))?;
                    expl_1 = List::map(dims.clone(), &move |__a0: metamodelica::Ref<DAE::Dimension>| unelabDimensionToFillExp(&__a0))?;
                    Ok(metamodelica::Ref::new(Absyn::Exp::CALL { function_: metamodelica::Ref::new(Absyn::ComponentRef::CREF_IDENT { name: literal!("fill"), subscripts: metamodelica::nil() }), functionArgs: metamodelica::Ref::new(Absyn::FunctionArgs::FUNCTIONARGS { args: metamodelica::cons(ae1.clone(), expl_1.clone()), argNames: metamodelica::nil() }), typeVars: metamodelica::nil() }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::ARRAY { array: expl, .. } => {
                    let mut expl_1: metamodelica::List<metamodelica::Ref<Absyn::Exp>>;
                    expl_1 = List::map(expl.clone(), &move |__a0: metamodelica::Ref<DAE::Exp>| unelabExp(&__a0))?;
                    Ok(metamodelica::Ref::new(Absyn::Exp::ARRAY { arrayExp: expl_1.clone() }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::MATRIX { matrix: mexpl2, .. } => {
                    let mut amexpl: metamodelica::List<metamodelica::List<metamodelica::Ref<Absyn::Exp>>>;
                    amexpl = List::mapList(mexpl2.clone(), &move |__a0: metamodelica::Ref<DAE::Exp>| unelabExp(&__a0))?;
                    Ok(metamodelica::Ref::new(Absyn::Exp::MATRIX { matrix: amexpl.clone() }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::RANGE { ty: _, start: e1, step: Some(e2), stop: e3 } => {
                    let mut ae1: metamodelica::Ref<Absyn::Exp>;
                    let mut ae2: metamodelica::Ref<Absyn::Exp>;
                    let mut ae3: metamodelica::Ref<Absyn::Exp>;
                    ae1 = unelabExp(metamodelica::AsArg::as_arg(&e1))?;
                    ae2 = unelabExp(metamodelica::AsArg::as_arg(&e2))?;
                    ae3 = unelabExp(metamodelica::AsArg::as_arg(&e3))?;
                    Ok(metamodelica::Ref::new(Absyn::Exp::RANGE { start: ae1.clone(), step: Some(ae2.clone()), stop: ae3.clone() }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::RANGE { ty: _, start: e1, step: None, stop: e3 } => {
                    let mut ae1: metamodelica::Ref<Absyn::Exp>;
                    let mut ae3: metamodelica::Ref<Absyn::Exp>;
                    ae1 = unelabExp(metamodelica::AsArg::as_arg(&e1))?;
                    ae3 = unelabExp(metamodelica::AsArg::as_arg(&e3))?;
                    Ok(metamodelica::Ref::new(Absyn::Exp::RANGE { start: ae1.clone(), step: None, stop: ae3.clone() }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::TUPLE { PR: expl } => {
                    let mut expl_1: metamodelica::List<metamodelica::Ref<Absyn::Exp>>;
                    expl_1 = List::map(expl.clone(), &move |__a0: metamodelica::Ref<DAE::Exp>| unelabExp(&__a0))?;
                    Ok(metamodelica::Ref::new(Absyn::Exp::TUPLE { expressions: expl_1.clone() }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::CAST { ty: _, exp: e1 } => {
                    let mut ae1: metamodelica::Ref<Absyn::Exp>;
                    ae1 = unelabExp(metamodelica::AsArg::as_arg(&e1))?;
                    Ok(ae1.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::ASUB { exp: _, sub: _ } => {
                    metamodelica::print(literal!("Internal Error, can not unelab ASUB\n"));
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::TSUB { exp: e1, ix: _, ty: _ } => {
                    let mut ae1: metamodelica::Ref<Absyn::Exp>;
                    ae1 = unelabExp(metamodelica::AsArg::as_arg(&e1))?;
                    Ok(ae1.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::SIZE { exp: e1, sz: Some(e2) } => {
                    let mut ae1: metamodelica::Ref<Absyn::Exp>;
                    let mut ae2: metamodelica::Ref<Absyn::Exp>;
                    ae1 = unelabExp(metamodelica::AsArg::as_arg(&e1))?;
                    ae2 = unelabExp(metamodelica::AsArg::as_arg(&e2))?;
                    Ok(metamodelica::Ref::new(Absyn::Exp::CALL { function_: metamodelica::Ref::new(Absyn::ComponentRef::CREF_IDENT { name: literal!("size"), subscripts: metamodelica::nil() }), functionArgs: metamodelica::Ref::new(Absyn::FunctionArgs::FUNCTIONARGS { args: list![ae1.clone(), ae2.clone()], argNames: metamodelica::nil() }), typeVars: metamodelica::nil() }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::CODE { code, ty: _ } => {
                    Ok(metamodelica::Ref::new(Absyn::Exp::CODE { code: code.clone() }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::REDUCTION { reductionInfo: Deref @ DAE::ReductionInfo { iterType, path, .. }, expr: e1, iterators: riters } => {
                    let mut ae1: metamodelica::Ref<Absyn::Exp>;
                    let mut acref: metamodelica::Ref<Absyn::ComponentRef>;
                    let mut aiters: metamodelica::List<metamodelica::Ref<Absyn::ForIterator>>;
                    acref = AbsynUtil::pathToCref(metamodelica::AsArg::as_arg(&path));
                    ae1 = unelabExp(metamodelica::AsArg::as_arg(&e1))?;
                    aiters = List::map(riters.clone(), &move |__a0: metamodelica::Ref<DAE::ReductionIterator>| unelabReductionIterator(&__a0))?;
                    Ok(metamodelica::Ref::new(Absyn::Exp::CALL { function_: acref.clone(), functionArgs: metamodelica::Ref::new(Absyn::FunctionArgs::FOR_ITER_FARG { exp: ae1.clone(), iterType: iterType.clone(), iterators: aiters.clone() }), typeVars: metamodelica::nil() }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { return Err("pattern mismatch") };
                    metamodelica::print({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Expression.unelabExp failed on: ")); __mm_s.push_str(&*printExpStr(inExp.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) });
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(outExp)
}

pub fn unelabDimension(mut inDim: &metamodelica::Ref<DAE::Dimension>) -> Result<metamodelica::Ref<Absyn::Subscript>> {
    let mut outDim: metamodelica::Ref<Absyn::Subscript>;
    outDim = (match &**inDim {
        DAE::Dimension::DIM_INTEGER { integer: i } => metamodelica::Ref::new(Absyn::Subscript::SUBSCRIPT {
            subscript: metamodelica::Ref::new(Absyn::Exp::INTEGER { value: i.clone() }),
        }),
        DAE::Dimension::DIM_BOOLEAN { .. } => metamodelica::Ref::new(Absyn::Subscript::SUBSCRIPT {
            subscript: metamodelica::Ref::new(Absyn::Exp::CREF {
                componentRef: metamodelica::Ref::new(Absyn::ComponentRef::CREF_IDENT {
                    name: literal!("Boolean"),
                    subscripts: metamodelica::nil(),
                }),
            }),
        }),
        DAE::Dimension::DIM_ENUM { enumTypeName: p, .. } => {
            let mut c: metamodelica::Ref<Absyn::ComponentRef>;
            c = AbsynUtil::pathToCref(p);
            metamodelica::Ref::new(Absyn::Subscript::SUBSCRIPT {
                subscript: metamodelica::Ref::new(Absyn::Exp::CREF { componentRef: c }),
            })
        }
        DAE::Dimension::DIM_EXP { exp: e } => {
            let mut ae: metamodelica::Ref<Absyn::Exp>;
            ae = unelabExp(e)?;
            metamodelica::Ref::new(Absyn::Subscript::SUBSCRIPT { subscript: ae })
        }
        DAE::Dimension::DIM_UNKNOWN { .. } => openmodelica_ast::Absyn::Subscript::interned_NOSUB(),
    });
    Ok(outDim)
}

fn unleabZeroExpFromType(mut ty: &metamodelica::Ref<DAE::Type>) -> Result<metamodelica::Ref<Absyn::Exp>> {
    let mut outExp: metamodelica::Ref<Absyn::Exp>;
    outExp = (match &**ty {
        DAE::Type::T_BOOL { .. } => metamodelica::Ref::new(Absyn::Exp::BOOL { value: false }),
        DAE::Type::T_STRING { .. } => metamodelica::Ref::new(Absyn::Exp::STRING { value: literal!("") }),
        DAE::Type::T_INTEGER { .. } => metamodelica::Ref::new(Absyn::Exp::INTEGER { value: 0 }),
        DAE::Type::T_REAL { .. } => metamodelica::Ref::new(Absyn::Exp::REAL { value: literal!("0.0") }),
        DAE::Type::T_UNKNOWN { .. } => metamodelica::Ref::new(Absyn::Exp::REAL { value: literal!("0.0") }),
        _ => return Err("match: no arm matched"),
    });
    Ok(outExp)
}

fn unelabDimensionToFillExp(mut inDim: &metamodelica::Ref<DAE::Dimension>) -> Result<metamodelica::Ref<Absyn::Exp>> {
    let mut outExp: metamodelica::Ref<Absyn::Exp>;
    outExp = (match &**inDim {
        DAE::Dimension::DIM_INTEGER { integer: i } => metamodelica::Ref::new(Absyn::Exp::INTEGER { value: i.clone() }),
        DAE::Dimension::DIM_EXP { exp: e } => unelabExp(e)?,
        _ => metamodelica::Ref::new(Absyn::Exp::INTEGER { value: 1 }),
    });
    Ok(outExp)
}

fn unelabReductionIterator(
    mut riter: &metamodelica::Ref<DAE::ReductionIterator>,
) -> Result<metamodelica::Ref<Absyn::ForIterator>> {
    let mut aiter: metamodelica::Ref<Absyn::ForIterator>;
    aiter = (match &**riter {
        DAE::ReductionIterator {
            id,
            exp,
            guardExp: gexp,
            ..
        } => {
            let mut aexp: metamodelica::Ref<Absyn::Exp>;
            let mut agexp: Option<metamodelica::Ref<Absyn::Exp>>;
            aexp = unelabExp(exp)?;
            agexp = Util::applyOption(gexp.clone(), &move |__a0: metamodelica::Ref<DAE::Exp>| unelabExp(&__a0))?;
            metamodelica::Ref::new(Absyn::ForIterator {
                name: id.clone(),
                guardExp: agexp,
                range: Some(aexp),
            })
        }
    });
    Ok(aiter)
}

fn unelabOperator(mut op: &DAE::Operator) -> Result<Absyn::Operator> {
    let mut aop: Absyn::Operator;
    aop = (match op.clone() {
        DAE::Operator::ADD { ty: _ } => openmodelica_ast::Absyn::Operator::ADD,
        DAE::Operator::SUB { ty: _ } => openmodelica_ast::Absyn::Operator::SUB,
        DAE::Operator::MUL { ty: _ } => openmodelica_ast::Absyn::Operator::MUL,
        DAE::Operator::DIV { ty: _ } => openmodelica_ast::Absyn::Operator::DIV,
        DAE::Operator::POW { ty: _ } => openmodelica_ast::Absyn::Operator::POW,
        DAE::Operator::UMINUS { ty: _ } => openmodelica_ast::Absyn::Operator::UMINUS,
        DAE::Operator::UMINUS_ARR { ty: _ } => openmodelica_ast::Absyn::Operator::UMINUS,
        DAE::Operator::ADD_ARR { ty: _ } => openmodelica_ast::Absyn::Operator::ADD,
        DAE::Operator::SUB_ARR { ty: _ } => openmodelica_ast::Absyn::Operator::SUB,
        DAE::Operator::MUL_ARR { ty: _ } => openmodelica_ast::Absyn::Operator::MUL,
        DAE::Operator::DIV_ARR { ty: _ } => openmodelica_ast::Absyn::Operator::DIV,
        DAE::Operator::MUL_ARRAY_SCALAR { ty: _ } => openmodelica_ast::Absyn::Operator::MUL,
        DAE::Operator::ADD_ARRAY_SCALAR { ty: _ } => openmodelica_ast::Absyn::Operator::ADD,
        DAE::Operator::SUB_SCALAR_ARRAY { ty: _ } => openmodelica_ast::Absyn::Operator::SUB,
        DAE::Operator::MUL_SCALAR_PRODUCT { ty: _ } => openmodelica_ast::Absyn::Operator::MUL,
        DAE::Operator::MUL_MATRIX_PRODUCT { ty: _ } => openmodelica_ast::Absyn::Operator::MUL,
        DAE::Operator::DIV_SCALAR_ARRAY { ty: _ } => openmodelica_ast::Absyn::Operator::DIV,
        DAE::Operator::DIV_ARRAY_SCALAR { ty: _ } => openmodelica_ast::Absyn::Operator::DIV,
        DAE::Operator::POW_SCALAR_ARRAY { ty: _ } => openmodelica_ast::Absyn::Operator::POW,
        DAE::Operator::POW_ARRAY_SCALAR { ty: _ } => openmodelica_ast::Absyn::Operator::POW,
        DAE::Operator::POW_ARR { ty: _ } => openmodelica_ast::Absyn::Operator::POW,
        DAE::Operator::POW_ARR2 { ty: _ } => openmodelica_ast::Absyn::Operator::POW,
        DAE::Operator::AND { ty: _ } => openmodelica_ast::Absyn::Operator::AND,
        DAE::Operator::OR { ty: _ } => openmodelica_ast::Absyn::Operator::OR,
        DAE::Operator::NOT { ty: _ } => openmodelica_ast::Absyn::Operator::NOT,
        DAE::Operator::LESS { ty: _ } => openmodelica_ast::Absyn::Operator::LESS,
        DAE::Operator::LESSEQ { ty: _ } => openmodelica_ast::Absyn::Operator::LESSEQ,
        DAE::Operator::GREATER { ty: _ } => openmodelica_ast::Absyn::Operator::GREATER,
        DAE::Operator::GREATEREQ { ty: _ } => openmodelica_ast::Absyn::Operator::GREATEREQ,
        DAE::Operator::EQUAL { ty: _ } => openmodelica_ast::Absyn::Operator::EQUAL,
        DAE::Operator::NEQUAL { ty: _ } => openmodelica_ast::Absyn::Operator::NEQUAL,
        _ => return Err("match: no arm matched"),
    });
    Ok(aop)
}

pub(crate) fn stringifyCrefs(mut inExp: metamodelica::Ref<DAE::Exp>) -> Result<metamodelica::Ref<DAE::Exp>> {
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    outExp = traverseExpDummy(
        inExp,
        (std::sync::Arc::new(traversingstringifyCrefFinder)
            as std::sync::Arc<
                dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<metamodelica::Ref<DAE::Exp>> + 'static,
            >),
    )?;
    Ok(outExp)
}

pub(crate) fn traversingstringifyCrefFinder(
    mut inExp: metamodelica::Ref<DAE::Exp>,
) -> Result<metamodelica::Ref<DAE::Exp>> {
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    outExp = (::match_deref::match_deref! { match &(inExp.clone()) {
        Deref @ DAE::Exp::CREF { ty: Deref @ DAE::Type::T_FUNCTION_REFERENCE_VAR { .. }, .. } => {
            inExp
        },
        Deref @ DAE::Exp::CREF { ty: Deref @ DAE::Type::T_FUNCTION_REFERENCE_FUNC { .. }, .. } => {
            inExp
        },
        Deref @ DAE::Exp::CREF { componentRef: cr, ty } => {
            let mut crs: ComponentRef;
            crs = ComponentReference::stringifyComponentRef(metamodelica::AsArg::as_arg(&cr))?;
            makeCrefExp(crs, ty.clone())?
        },
        _ => {
            inExp
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outExp)
}

pub(crate) fn realToIntIfPossible(mut inVal: metamodelica::Real) -> metamodelica::Ref<DAE::Exp> {
    let mut outVal: metamodelica::Ref<DAE::Exp>;
    match '__try0: {
        outVal = metamodelica::Ref::new(DAE::Exp::ICONST {
            integer: ((inVal).0.floor() as i32),
        });
        Ok::<_, &'static str>((outVal.clone(),))
    } {
        Ok((__try0_o0,)) => {
            outVal = __try0_o0;
        }
        Err(_) => {
            outVal = metamodelica::Ref::new(DAE::Exp::RCONST { real: inVal });
        }
    }
    outVal
}

pub(crate) fn liftArrayR(
    mut tp: metamodelica::Ref<DAE::Type>,
    mut n: metamodelica::Ref<DAE::Dimension>,
) -> metamodelica::Ref<DAE::Type> {
    let mut outTp: metamodelica::Ref<DAE::Type>;
    outTp = (match &*tp {
        DAE::Type::T_ARRAY { ty: elt_tp, dims } => {
            let mut dims = (*dims).clone();
            dims = metamodelica::cons(n, dims.clone());
            metamodelica::Ref::new(DAE::Type::T_ARRAY {
                ty: elt_tp.clone(),
                dims: dims.clone(),
            })
        }
        _ => metamodelica::Ref::new(DAE::Type::T_ARRAY { ty: tp, dims: list![n] }),
    });
    outTp
}

pub fn dimensionSizeConstantExp(mut dim: &metamodelica::Ref<DAE::Dimension>) -> Result<metamodelica::Ref<DAE::Exp>> {
    let mut exp: metamodelica::Ref<DAE::Exp>;
    exp = (match &**dim {
        DAE::Dimension::DIM_INTEGER { integer: i } => metamodelica::Ref::new(DAE::Exp::ICONST { integer: i.clone() }),
        DAE::Dimension::DIM_ENUM { size: i, .. } => metamodelica::Ref::new(DAE::Exp::ICONST { integer: i.clone() }),
        DAE::Dimension::DIM_BOOLEAN { .. } => metamodelica::Ref::new(DAE::Exp::ICONST { integer: 2 }),
        _ => return Err("match: no arm matched"),
    });
    Ok(exp)
}

pub fn dimensionSizeExp(mut dim: &metamodelica::Ref<DAE::Dimension>) -> Result<metamodelica::Ref<DAE::Exp>> {
    let mut exp: metamodelica::Ref<DAE::Exp>;
    exp = (match &**dim {
        DAE::Dimension::DIM_INTEGER { integer: i } => metamodelica::Ref::new(DAE::Exp::ICONST { integer: i.clone() }),
        DAE::Dimension::DIM_ENUM { size: i, .. } => metamodelica::Ref::new(DAE::Exp::ICONST { integer: i.clone() }),
        DAE::Dimension::DIM_BOOLEAN { .. } => metamodelica::Ref::new(DAE::Exp::ICONST { integer: 2 }),
        DAE::Dimension::DIM_EXP { exp: e } => e.clone(),
        _ => return Err("match: no arm matched"),
    });
    Ok(exp)
}

pub fn dimensionSizeExpHandleUnkown(
    mut dim: &metamodelica::Ref<DAE::Dimension>,
) -> Result<metamodelica::Ref<DAE::Exp>> {
    let mut exp: metamodelica::Ref<DAE::Exp>;
    exp = (match &**dim {
        DAE::Dimension::DIM_UNKNOWN { .. } => metamodelica::Ref::new(DAE::Exp::ICONST { integer: -1 }),
        _ => dimensionSizeExp(dim)?,
    });
    Ok(exp)
}

pub fn intDimension(mut value: i32) -> metamodelica::Ref<DAE::Dimension> {
    let mut dim: metamodelica::Ref<DAE::Dimension>;
    dim = metamodelica::Ref::new(DAE::Dimension::DIM_INTEGER { integer: value });
    dim
}

pub fn dimensionSubscript(mut dim: &metamodelica::Ref<DAE::Dimension>) -> Result<metamodelica::Ref<DAE::Subscript>> {
    let mut sub: metamodelica::Ref<DAE::Subscript>;
    sub = (match &**dim {
        DAE::Dimension::DIM_INTEGER { integer: i } => metamodelica::Ref::new(DAE::Subscript::INDEX {
            exp: metamodelica::Ref::new(DAE::Exp::ICONST { integer: i.clone() }),
        }),
        DAE::Dimension::DIM_ENUM { size: i, .. } => metamodelica::Ref::new(DAE::Subscript::INDEX {
            exp: metamodelica::Ref::new(DAE::Exp::ICONST { integer: i.clone() }),
        }),
        DAE::Dimension::DIM_BOOLEAN { .. } => metamodelica::Ref::new(DAE::Subscript::INDEX {
            exp: metamodelica::Ref::new(DAE::Exp::ICONST { integer: 2 }),
        }),
        DAE::Dimension::DIM_UNKNOWN { .. } => openmodelica_frontend_types::DAE::Subscript::interned_WHOLEDIM(),
        _ => return Err("match: no arm matched"),
    });
    Ok(sub)
}

/* **************************************************/
/* Change  */
/* **************************************************/
pub fn negate(mut inExp: metamodelica::Ref<DAE::Exp>) -> Result<metamodelica::Ref<DAE::Exp>> {
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    outExp = (::match_deref::match_deref! { match &(inExp) {
        Deref @ DAE::Exp::UNARY { operator: DAE::Operator::UMINUS { ty: _ }, exp: e } => {
            e.clone()
        },
        Deref @ DAE::Exp::UNARY { operator: DAE::Operator::UMINUS_ARR { ty: _ }, exp: e } => {
            e.clone()
        },
        Deref @ DAE::Exp::LUNARY { operator: DAE::Operator::NOT { ty: _ }, exp: e } => {
            e.clone()
        },
        Deref @ DAE::Exp::BINARY { exp1: e1, operator: op, exp2: e2 } if (isMulOrDiv(metamodelica::AsArg::as_arg(&op))) => {
            metamodelica::Ref::new(DAE::Exp::BINARY { exp1: negate(e1.clone())?, operator: op.clone(), exp2: e2.clone() })
        },
        Deref @ DAE::Exp::BINARY { exp1: e1, operator: op, exp2: e2 } if (isSub(metamodelica::AsArg::as_arg(&op))) => {
            metamodelica::Ref::new(DAE::Exp::BINARY { exp1: e2.clone(), operator: op.clone(), exp2: e1.clone() })
        },
        e if (isZero(metamodelica::AsArg::as_arg(&e))?) => {
            e.clone()
        },
        Deref @ DAE::Exp::ICONST { integer: i } => {
            let mut i_1: i32;
            i_1 = 0 - i.clone();
            metamodelica::Ref::new(DAE::Exp::ICONST { integer: i_1 })
        },
        Deref @ DAE::Exp::RCONST { real: r } => {
            let mut r_1: metamodelica::Real;
            r_1 = metamodelica::OrderedFloat(0.0_f64) - r.clone();
            metamodelica::Ref::new(DAE::Exp::RCONST { real: r_1 })
        },
        Deref @ DAE::Exp::BCONST { bool: b } => {
            let mut b_1: bool;
            b_1 = !(b.clone());
            metamodelica::Ref::new(DAE::Exp::BCONST { bool: b_1 })
        },
        e => {
            let mut t: Type;
            let mut op: Operator;
            let mut b: bool;
            t = r#typeof(e.clone())?;
            outExp = (match &*t {
        DAE::Type::T_BOOL { .. } => metamodelica::Ref::new(DAE::Exp::LUNARY { operator: DAE::Operator::NOT { ty: t }, exp: e.clone() }),
        _ => {
            b = DAEUtil::expTypeArray(&t);
            op = if (b) {DAE::Operator::UMINUS_ARR { ty: t }} else {DAE::Operator::UMINUS { ty: t }};
            metamodelica::Ref::new(DAE::Exp::UNARY { operator: op.clone(), exp: e.clone() })
        },
    });
            outExp
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outExp)
}

pub fn negateReal(mut inReal: metamodelica::Ref<DAE::Exp>) -> metamodelica::Ref<DAE::Exp> {
    let mut outNegatedReal: metamodelica::Ref<DAE::Exp>;
    outNegatedReal = metamodelica::Ref::new(DAE::Exp::UNARY {
        operator: DAE::Operator::UMINUS {
            ty: DAE::T_REAL_DEFAULT().clone(),
        },
        exp: inReal,
    });
    outNegatedReal
}

pub fn expand(mut e: &metamodelica::Ref<DAE::Exp>) -> Result<metamodelica::Ref<DAE::Exp>> {
    let mut outE: metamodelica::Ref<DAE::Exp>;
    outE = (::match_deref::match_deref! { match e {
        Deref @ DAE::Exp::BINARY { exp1: e1, operator: DAE::Operator::MUL { ty: tp }, exp2: e2 @ Deref @ DAE::Exp::BINARY { exp1: e21, operator: op, exp2: e22 } } if (isAddOrSub(metamodelica::AsArg::as_arg(&op))) => {
            let mut e21 = (*e21).clone();
            let mut op = (*op).clone();
            let mut e22 = (*e22).clone();
            let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(expand(metamodelica::AsArg::as_arg(&e2))?) {
                Deref @ DAE::Exp::BINARY { exp1: __pa0, operator: __pa1, exp2: __pa2 } => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
                _ => return Err("pattern mismatch"),
            } };
            e21 = metamodelica::Own::own(__pa0);
            op = metamodelica::Own::own(__pa1);
            e22 = metamodelica::Own::own(__pa2);
            metamodelica::Ref::new(DAE::Exp::BINARY { exp1: metamodelica::Ref::new(DAE::Exp::BINARY { exp1: e1.clone(), operator: DAE::Operator::MUL { ty: tp.clone() }, exp2: e21.clone() }), operator: op.clone(), exp2: metamodelica::Ref::new(DAE::Exp::BINARY { exp1: e1.clone(), operator: DAE::Operator::MUL { ty: tp.clone() }, exp2: e22.clone() }) })
        },
        _ => {
            e.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outE)
}

pub fn expDer(mut inExp: metamodelica::Ref<DAE::Exp>) -> metamodelica::Ref<DAE::Exp> {
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    outExp = metamodelica::Ref::new(DAE::Exp::CALL {
        path: metamodelica::Ref::new(Absyn::Path::IDENT { name: literal!("der") }),
        expLst: list![inExp],
        attr: DAE::callAttrBuiltinReal().clone(),
    });
    outExp
}

pub(crate) fn expAbs<'__b>(mut inExp: &'__b metamodelica::Ref<DAE::Exp>) -> metamodelica::Ref<DAE::Exp> {
    '__tco: loop {
        match &**inExp {
            DAE::Exp::ICONST { integer: i } => {
                let mut i2: i32;
                i2 = intAbs(i.clone());
                return metamodelica::Ref::new(DAE::Exp::ICONST { integer: i2 });
            }
            DAE::Exp::RCONST { real: r } => {
                let mut r2: metamodelica::Real;
                r2 = realAbs(r.clone());
                return metamodelica::Ref::new(DAE::Exp::RCONST { real: r2 });
            }
            DAE::Exp::UNARY {
                operator: DAE::Operator::UMINUS { .. },
                exp: e,
            } => {
                let mut e_1: metamodelica::Ref<DAE::Exp>;
                {
                    inExp = e;
                    continue '__tco;
                }
            }
            DAE::Exp::BINARY {
                exp1: e1,
                operator: op,
                exp2: e2,
            } => {
                let mut e1_1: metamodelica::Ref<DAE::Exp>;
                let mut e2_1: metamodelica::Ref<DAE::Exp>;
                e1_1 = expAbs(e1);
                e2_1 = expAbs(e2);
                return metamodelica::Ref::new(DAE::Exp::BINARY {
                    exp1: e1_1,
                    operator: op.clone(),
                    exp2: e2_1,
                });
            }
            _ => return inExp.clone(),
        }
    }
}

pub(crate) fn stripNoEvent(mut e: metamodelica::Ref<DAE::Exp>) -> Result<metamodelica::Ref<DAE::Exp>> {
    let mut outE: metamodelica::Ref<DAE::Exp>;
    outE = traverseExpDummy(
        e,
        (std::sync::Arc::new(fnptr!(stripNoEventExp, metamodelica::Ref<DAE::Exp>))
            as std::sync::Arc<
                dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<metamodelica::Ref<DAE::Exp>> + 'static,
            >),
    )?;
    Ok(outE)
}

fn stripNoEventExp(mut e: metamodelica::Ref<DAE::Exp>) -> metamodelica::Ref<DAE::Exp> {
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    outExp = (::match_deref::match_deref! { match &(e.clone()) {
        Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "noEvent" }, expLst: Deref @ metamodelica::ListNode::Cons { head: __esc_outExp, tail: Deref @ metamodelica::ListNode::Nil }, .. } => {
            outExp = (*__esc_outExp).clone();
            outExp.clone()
        },
        _ => e,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outExp
}

pub fn addNoEventToRelations(mut e: metamodelica::Ref<DAE::Exp>) -> Result<metamodelica::Ref<DAE::Exp>> {
    let mut outE: metamodelica::Ref<DAE::Exp>;
    outE = traverseExpDummy(
        e,
        (std::sync::Arc::new(fnptr!(addNoEventToRelationExp, metamodelica::Ref<DAE::Exp>))
            as std::sync::Arc<
                dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<metamodelica::Ref<DAE::Exp>> + 'static,
            >),
    )?;
    Ok(outE)
}

fn addNoEventToRelationExp(mut e: metamodelica::Ref<DAE::Exp>) -> metamodelica::Ref<DAE::Exp> {
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    outExp = (match &*e {
        DAE::Exp::RELATION { .. } => makeNoEvent(e),
        _ => e,
    });
    outExp
}

pub fn addNoEventToRelationsAndConds(mut e: metamodelica::Ref<DAE::Exp>) -> Result<metamodelica::Ref<DAE::Exp>> {
    let mut outE: metamodelica::Ref<DAE::Exp>;
    outE = traverseExpDummy(
        e,
        (std::sync::Arc::new(fnptr!(addNoEventToRelationandCondExp, metamodelica::Ref<DAE::Exp>))
            as std::sync::Arc<
                dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<metamodelica::Ref<DAE::Exp>> + 'static,
            >),
    )?;
    Ok(outE)
}

fn addNoEventToRelationandCondExp(mut e: metamodelica::Ref<DAE::Exp>) -> metamodelica::Ref<DAE::Exp> {
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    outExp = (match &*e {
        DAE::Exp::RELATION { .. } => makeNoEvent(e),
        DAE::Exp::IFEXP {
            expCond: e1,
            expThen: e2,
            expElse: e3,
        } => metamodelica::Ref::new(DAE::Exp::IFEXP {
            expCond: makeNoEvent(e1.clone()),
            expThen: e2.clone(),
            expElse: e3.clone(),
        }),
        _ => e,
    });
    outExp
}

pub fn addNoEventToEventTriggeringFunctions(mut e: metamodelica::Ref<DAE::Exp>) -> Result<metamodelica::Ref<DAE::Exp>> {
    let mut outE: metamodelica::Ref<DAE::Exp>;
    outE = traverseExpDummy(
        e,
        (std::sync::Arc::new(fnptr!(
            addNoEventToEventTriggeringFunctionsExp,
            metamodelica::Ref<DAE::Exp>
        ))
            as std::sync::Arc<
                dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<metamodelica::Ref<DAE::Exp>> + 'static,
            >),
    )?;
    Ok(outE)
}

fn addNoEventToEventTriggeringFunctionsExp(mut e: metamodelica::Ref<DAE::Exp>) -> metamodelica::Ref<DAE::Exp> {
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    outExp = (match &*e {
        DAE::Exp::CALL { .. } if (isEventTriggeringFunctionExp(&e)) => makeNoEvent(e.clone()),
        _ => e.clone(),
    });
    outExp
}

pub fn expStripLastSubs(mut inExp: &metamodelica::Ref<DAE::Exp>) -> Result<metamodelica::Ref<DAE::Exp>> {
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    outExp = (match &**inExp {
        DAE::Exp::CREF { componentRef: cr, .. } => {
            let mut cr_1: ComponentRef;
            let mut ty: Type;
            let mut e: metamodelica::Ref<DAE::Exp>;
            ty = ComponentReference::crefLastType(cr)?;
            cr_1 = ComponentReferenceBasics::crefStripLastSubs(cr)?;
            e = makeCrefExp(cr_1, ty)?;
            e
        }
        DAE::Exp::UNARY { exp: e, .. } => {
            let mut ty: Type;
            let mut op1: Operator;
            let mut e_1: metamodelica::Ref<DAE::Exp>;
            let mut b: bool;
            e_1 = expStripLastSubs(e)?;
            ty = r#typeof(e_1.clone())?;
            b = DAEUtil::expTypeArray(&ty);
            op1 = if (b) {
                DAE::Operator::UMINUS_ARR { ty: ty }
            } else {
                DAE::Operator::UMINUS { ty: ty }
            };
            metamodelica::Ref::new(DAE::Exp::UNARY {
                operator: op1,
                exp: e_1,
            })
        }
        _ => return Err("match: no arm matched"),
    });
    Ok(outExp)
}

pub(crate) fn expStripLastIdent(mut inExp: &metamodelica::Ref<DAE::Exp>) -> Result<metamodelica::Ref<DAE::Exp>> {
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    outExp = (match &**inExp {
        DAE::Exp::CREF { componentRef: cr, .. } => {
            let mut cr_1: ComponentRef;
            let mut ty: Type;
            let mut e: metamodelica::Ref<DAE::Exp>;
            cr_1 = ComponentReference::crefStripLastIdent(cr)?;
            ty = ComponentReference::crefLastType(&cr_1)?;
            e = makeCrefExp(cr_1, ty)?;
            e
        }
        DAE::Exp::UNARY { exp: e, .. } => {
            let mut ty: Type;
            let mut op1: Operator;
            let mut e_1: metamodelica::Ref<DAE::Exp>;
            let mut b: bool;
            e_1 = expStripLastIdent(e)?;
            ty = r#typeof(e_1.clone())?;
            b = DAEUtil::expTypeArray(&ty);
            op1 = if (b) {
                DAE::Operator::UMINUS_ARR { ty: ty }
            } else {
                DAE::Operator::UMINUS { ty: ty }
            };
            metamodelica::Ref::new(DAE::Exp::UNARY {
                operator: op1,
                exp: e_1,
            })
        }
        _ => return Err("match: no arm matched"),
    });
    Ok(outExp)
}

pub(crate) fn prependSubscriptExp(
    mut exp: &metamodelica::Ref<DAE::Exp>,
    mut subscr: metamodelica::Ref<DAE::Subscript>,
) -> Result<metamodelica::Ref<DAE::Exp>> {
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    outExp = (match &**exp {
        DAE::Exp::CREF {
            componentRef: cr,
            ty: t,
        } => {
            let mut cr1: ComponentRef;
            let mut cr2: ComponentRef;
            let mut subs: metamodelica::List<metamodelica::Ref<DAE::Subscript>>;
            let mut e: metamodelica::Ref<DAE::Exp>;
            cr1 = ComponentReferenceBasics::crefStripLastSubs(cr)?;
            subs = ComponentReference::crefLastSubs(cr)?;
            cr2 = ComponentReference::subscriptCref(&cr1, metamodelica::cons(subscr, subs))?;
            e = makeCrefExp(cr2, t.clone())?;
            e
        }
        _ => return Err("match: no arm matched"),
    });
    Ok(outExp)
}

pub fn applyExpSubscripts(
    mut exp: metamodelica::Ref<DAE::Exp>,
    mut inSubs: metamodelica::List<metamodelica::Ref<DAE::Subscript>>,
) -> Result<metamodelica::Ref<DAE::Exp>> {
    let mut exp: metamodelica::Ref<DAE::Exp> = exp;
    let mut r#str: ArcStr;
    if '__try0: {
        exp = applyExpSubscripts2(exp.clone(), inSubs.clone());
        Ok::<(), &'static str>(())
    }
    .is_err()
    {
        r#str = {
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("Expression.applyExpSubscripts failed applying subs: ["));
            __mm_s.push_str(&*ExpressionDump::printSubscriptLstStr(inSubs.clone())?);
            __mm_s.push_str(&*literal!("] on expression:"));
            __mm_s.push_str(&*printExpStr(exp.clone())?);
            __mm_s.push_str(&*literal!("\n"));
            ArcStr::from(__mm_s)
        };
        Error::addMessage(Error::INTERNAL_ERROR.clone(), list![r#str.clone()])?;
    }
    Ok(exp)
}

pub fn applyExpSubscriptsFoldCheckSimplify(
    mut exp: metamodelica::Ref<DAE::Exp>,
    mut inSubs: &metamodelica::List<metamodelica::Ref<DAE::Subscript>>,
    mut checkSimplify: bool,
) -> (metamodelica::Ref<DAE::Exp>, bool) {
    let mut exp: metamodelica::Ref<DAE::Exp> = exp;
    let mut checkSimplify: bool = checkSimplify;
    let mut b: bool;
    let mut s: metamodelica::Ref<DAE::Exp>;
    for mut sub in &**inSubs {
        if '__try0: {
            s = unwrap_break_err!(getSubscriptExp(metamodelica::AsArg::as_arg(&sub)), '__try0);
            (exp, b) = unwrap_break_err!(ExpressionSimplify::simplify(unwrap_break_err!(makeASUB(exp.clone(), list![s.clone()]), '__try0)), '__try0);
            checkSimplify = b || checkSimplify;
            Ok::<(), &'static str>(())
        }.is_err() {
        }
    }
    (exp, checkSimplify)
}

pub(crate) fn applyExpSubscripts2(
    mut inExp: metamodelica::Ref<DAE::Exp>,
    mut inSubs: metamodelica::List<metamodelica::Ref<DAE::Subscript>>,
) -> metamodelica::Ref<DAE::Exp> {
    let mut outArg: metamodelica::Ref<DAE::Exp>;
    outArg = (::match_deref::match_deref! { match &((inExp.clone(), inSubs.clone())) {
        (_, Deref @ metamodelica::ListNode::Nil) => {
            inExp
        },
        (Deref @ DAE::Exp::CREF { componentRef: cref, ty }, _) => {
            let mut exp: metamodelica::Ref<DAE::Exp>;
            let mut cref = (*cref).clone();
            let mut ty = (*ty).clone();
            match '__try0: {
                cref = unwrap_break_err!(ComponentReference::subscriptCref(metamodelica::AsArg::as_arg(&cref), inSubs.clone()), '__try0);
                ty = unwrap_break_err!(ComponentReference::crefTypeFull(metamodelica::AsArg::as_arg(&cref)), '__try0);
                exp = metamodelica::Ref::new(DAE::Exp::CREF { componentRef: cref.clone(), ty: ty.clone() });
                Ok::<_, &'static str>((exp.clone(),))
            } {
                Ok((__try0_o0,)) => {
                    exp = __try0_o0;
                }
                Err(_) => {
                    (exp, _) = applyExpSubscriptsFoldCheckSimplify(inExp.clone(), &inSubs, false);
                }
            }
            exp
        },
        _ => {
            let mut exp: metamodelica::Ref<DAE::Exp>;
            (exp, _) = applyExpSubscriptsFoldCheckSimplify(inExp, &inSubs, false);
            exp
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outArg
}

pub fn unliftArray(mut inType: &metamodelica::Ref<DAE::Type>) -> Result<metamodelica::Ref<DAE::Type>> {
    let mut outType: metamodelica::Ref<DAE::Type>;
    outType = (::match_deref::match_deref! { match inType {
        Deref @ DAE::Type::T_ARRAY { ty: tp, dims: Deref @ metamodelica::ListNode::Cons { head: _, tail: Deref @ metamodelica::ListNode::Nil } } => {
            tp.clone()
        },
        Deref @ DAE::Type::T_ARRAY { ty: tp, dims: Deref @ metamodelica::ListNode::Cons { head: _, tail: ds } } => {
            metamodelica::Ref::new(DAE::Type::T_ARRAY { ty: tp.clone(), dims: ds.clone() })
        },
        Deref @ DAE::Type::T_METATYPE { ty: tp } => {
            Types::simplifyType(unliftArray(tp)?)?
        },
        Deref @ DAE::Type::T_METAARRAY { ty: tp } => {
            tp.clone()
        },
        _ => {
            inType.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outType)
}

pub(crate) fn unliftArrayIgnoreFirst<A: Clone + 'static + metamodelica::gc::MMTrace>(
    mut a: A,
    mut inType: &metamodelica::Ref<DAE::Type>,
) -> Result<metamodelica::Ref<DAE::Type>> {
    let mut outType: metamodelica::Ref<DAE::Type>;
    outType = unliftArray(inType)?;
    Ok(outType)
}

pub fn unliftExp(mut inExp: metamodelica::Ref<DAE::Exp>) -> Result<metamodelica::Ref<DAE::Exp>> {
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    outExp = (match &*inExp {
        DAE::Exp::CREF { componentRef: cr, ty } => {
            let mut expCref: metamodelica::Ref<DAE::Exp>;
            let mut ty = (*ty).clone();
            ty = unliftArray(metamodelica::AsArg::as_arg(&ty))?;
            expCref = makeCrefExp(cr.clone(), ty.clone())?;
            expCref
        }
        DAE::Exp::ARRAY {
            ty,
            scalar: s,
            array: a,
        } => {
            let mut ty = (*ty).clone();
            ty = unliftArray(metamodelica::AsArg::as_arg(&ty))?;
            metamodelica::Ref::new(DAE::Exp::ARRAY {
                ty: ty.clone(),
                scalar: s.clone(),
                array: a.clone(),
            })
        }
        DAE::Exp::MATRIX {
            ty,
            integer: i,
            matrix: mat,
        } => {
            let mut ty = (*ty).clone();
            ty = unliftArray(metamodelica::AsArg::as_arg(&ty))?;
            metamodelica::Ref::new(DAE::Exp::MATRIX {
                ty: ty.clone(),
                integer: i.clone(),
                matrix: mat.clone(),
            })
        }
        _ => inExp,
    });
    Ok(outExp)
}

pub(crate) fn liftExp(
    mut inExp: metamodelica::Ref<DAE::Exp>,
    mut inDimension: metamodelica::Ref<DAE::Dimension>,
) -> Result<metamodelica::Ref<DAE::Exp>> {
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    outExp = metamodelica::Ref::new(DAE::Exp::ARRAY {
        ty: Types::liftArray(r#typeof(inExp.clone())?, inDimension.clone()),
        scalar: false,
        array: List::fill(inExp, dimensionSize(&inDimension)?),
    });
    Ok(outExp)
}

pub fn liftExpList(
    mut inExp: metamodelica::Ref<DAE::Exp>,
    mut inDimensions: metamodelica::List<metamodelica::Ref<DAE::Dimension>>,
) -> Result<metamodelica::Ref<DAE::Exp>> {
    let mut outExp: metamodelica::Ref<DAE::Exp> = inExp;
    for mut dim in &*inDimensions.reverse() {
        outExp = liftExp(outExp, dim.clone())?;
    }
    Ok(outExp)
}

pub fn liftArrayRight(
    mut inType: &metamodelica::Ref<DAE::Type>,
    mut inDimension: metamodelica::Ref<DAE::Dimension>,
) -> metamodelica::Ref<DAE::Type> {
    let mut outType: metamodelica::Ref<DAE::Type>;
    outType = (match &**inType {
        DAE::Type::T_ARRAY { ty, dims } => {
            let mut dim = inDimension.clone();
            let mut ty_1: Type;
            ty_1 = liftArrayRight(ty, dim);
            metamodelica::Ref::new(DAE::Type::T_ARRAY {
                ty: ty_1,
                dims: dims.clone(),
            })
        }
        _ => metamodelica::Ref::new(DAE::Type::T_ARRAY {
            ty: inType.clone(),
            dims: list![inDimension],
        }),
    });
    outType
}

pub fn liftArrayLeft(
    mut inType: metamodelica::Ref<DAE::Type>,
    mut inDimension: metamodelica::Ref<DAE::Dimension>,
) -> metamodelica::Ref<DAE::Type> {
    let mut outType: metamodelica::Ref<DAE::Type>;
    outType = (match &*inType {
        DAE::Type::T_ARRAY { ty, dims } => {
            let mut dim = inDimension.clone();
            metamodelica::Ref::new(DAE::Type::T_ARRAY {
                ty: ty.clone(),
                dims: metamodelica::cons(dim, dims.clone()),
            })
        }
        _ => metamodelica::Ref::new(DAE::Type::T_ARRAY {
            ty: inType,
            dims: list![inDimension],
        }),
    });
    outType
}

pub fn liftArrayLeftList(
    mut inType: metamodelica::Ref<DAE::Type>,
    mut inDimensions: metamodelica::List<metamodelica::Ref<DAE::Dimension>>,
) -> metamodelica::Ref<DAE::Type> {
    let mut outType: metamodelica::Ref<DAE::Type>;
    outType = (::match_deref::match_deref! { match &((inType.clone(), inDimensions.clone())) {
        (_, Deref @ metamodelica::ListNode::Nil) => {
            inType
        },
        (Deref @ DAE::Type::T_ARRAY { ty, dims }, _) => {
            let mut dims = (*dims).clone();
            dims = listAppend(inDimensions, dims.clone());
            metamodelica::Ref::new(DAE::Type::T_ARRAY { ty: ty.clone(), dims: dims.clone() })
        },
        _ => {
            metamodelica::Ref::new(DAE::Type::T_ARRAY { ty: inType, dims: inDimensions })
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outType
}

pub fn setOpType(mut inOp: DAE::Operator, mut inType: metamodelica::Ref<DAE::Type>) -> Result<DAE::Operator> {
    let mut outOp: DAE::Operator;
    outOp = (match inOp.clone() {
        DAE::Operator::ADD { .. } => DAE::Operator::ADD { ty: inType },
        DAE::Operator::SUB { .. } => DAE::Operator::SUB { ty: inType },
        DAE::Operator::MUL { .. } => DAE::Operator::MUL { ty: inType },
        DAE::Operator::DIV { .. } => DAE::Operator::DIV { ty: inType },
        DAE::Operator::POW { .. } => DAE::Operator::POW { ty: inType },
        DAE::Operator::UMINUS { .. } => DAE::Operator::UMINUS { ty: inType },
        DAE::Operator::UMINUS_ARR { .. } => DAE::Operator::UMINUS_ARR { ty: inType },
        DAE::Operator::ADD_ARR { .. } => DAE::Operator::ADD_ARR { ty: inType },
        DAE::Operator::SUB_ARR { .. } => DAE::Operator::SUB_ARR { ty: inType },
        DAE::Operator::MUL_ARR { .. } => DAE::Operator::MUL_ARR { ty: inType },
        DAE::Operator::DIV_ARR { .. } => DAE::Operator::DIV_ARR { ty: inType },
        DAE::Operator::MUL_ARRAY_SCALAR { .. } => DAE::Operator::MUL_ARRAY_SCALAR { ty: inType },
        DAE::Operator::ADD_ARRAY_SCALAR { .. } => DAE::Operator::ADD_ARRAY_SCALAR { ty: inType },
        DAE::Operator::SUB_SCALAR_ARRAY { .. } => DAE::Operator::SUB_SCALAR_ARRAY { ty: inType },
        DAE::Operator::MUL_SCALAR_PRODUCT { .. } => DAE::Operator::MUL_SCALAR_PRODUCT { ty: inType },
        DAE::Operator::MUL_MATRIX_PRODUCT { .. } => DAE::Operator::MUL_MATRIX_PRODUCT { ty: inType },
        DAE::Operator::DIV_ARRAY_SCALAR { .. } => DAE::Operator::DIV_ARRAY_SCALAR { ty: inType },
        DAE::Operator::DIV_SCALAR_ARRAY { .. } => DAE::Operator::DIV_SCALAR_ARRAY { ty: inType },
        DAE::Operator::POW_ARRAY_SCALAR { .. } => DAE::Operator::POW_ARRAY_SCALAR { ty: inType },
        DAE::Operator::POW_SCALAR_ARRAY { .. } => DAE::Operator::POW_SCALAR_ARRAY { ty: inType },
        DAE::Operator::POW_ARR { .. } => DAE::Operator::POW_ARR { ty: inType },
        DAE::Operator::POW_ARR2 { .. } => DAE::Operator::POW_ARR2 { ty: inType },
        DAE::Operator::AND { .. } => DAE::Operator::AND { ty: inType },
        DAE::Operator::OR { .. } => DAE::Operator::OR { ty: inType },
        DAE::Operator::NOT { .. } => DAE::Operator::NOT { ty: inType },
        DAE::Operator::LESS { .. } => inOp,
        DAE::Operator::LESSEQ { .. } => inOp,
        DAE::Operator::GREATER { .. } => inOp,
        DAE::Operator::GREATEREQ { .. } => inOp,
        DAE::Operator::EQUAL { .. } => inOp,
        DAE::Operator::NEQUAL { .. } => inOp,
        DAE::Operator::USERDEFINED { .. } => inOp,
        _ => {
            let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else {
                return Err("pattern mismatch");
            };
            Debug::traceln(literal!("- Expression.setOpType failed on unknown operator"))?;
            return Err("fail");
        }
    });
    Ok(outOp)
}

pub(crate) fn unliftOperator(mut inOperator: DAE::Operator) -> Result<DAE::Operator> {
    let mut outOperator: DAE::Operator;
    let mut ty: Type;
    ty = typeofOp(&inOperator);
    ty = unliftArray(&ty)?;
    outOperator = unliftOperator2(inOperator, ty)?;
    Ok(outOperator)
}

pub(crate) fn unliftOperatorX(mut inOperator: DAE::Operator, mut inX: i32) -> Result<DAE::Operator> {
    let mut outOperator: DAE::Operator;
    let mut ty: Type;
    ty = typeofOp(&inOperator);
    ty = unliftArrayX(ty, inX)?;
    outOperator = unliftOperator2(inOperator, ty)?;
    Ok(outOperator)
}

fn unliftOperator2(mut inOperator: DAE::Operator, mut inType: metamodelica::Ref<DAE::Type>) -> Result<DAE::Operator> {
    let mut outOperator: DAE::Operator;
    outOperator = (match &*inType {
        DAE::Type::T_ARRAY { .. } => setOpType(inOperator, inType)?,
        _ => makeScalarOpFromArrayOp(inOperator, inType),
    });
    Ok(outOperator)
}

fn makeScalarOpFromArrayOp(mut inOperator: DAE::Operator, mut inType: metamodelica::Ref<DAE::Type>) -> DAE::Operator {
    let mut outOperator: DAE::Operator;
    outOperator = (match inOperator.clone() {
        DAE::Operator::MUL_ARRAY_SCALAR { .. } => DAE::Operator::MUL { ty: inType },
        DAE::Operator::ADD_ARRAY_SCALAR { .. } => DAE::Operator::ADD { ty: inType },
        DAE::Operator::SUB_SCALAR_ARRAY { .. } => DAE::Operator::SUB { ty: inType },
        DAE::Operator::DIV_ARRAY_SCALAR { .. } => DAE::Operator::DIV { ty: inType },
        DAE::Operator::DIV_SCALAR_ARRAY { .. } => DAE::Operator::DIV { ty: inType },
        DAE::Operator::POW_ARRAY_SCALAR { .. } => DAE::Operator::POW { ty: inType },
        DAE::Operator::POW_SCALAR_ARRAY { .. } => DAE::Operator::POW { ty: inType },
        DAE::Operator::UMINUS_ARR { .. } => DAE::Operator::UMINUS { ty: inType },
        DAE::Operator::ADD_ARR { .. } => DAE::Operator::ADD { ty: inType },
        DAE::Operator::SUB_ARR { .. } => DAE::Operator::SUB { ty: inType },
        DAE::Operator::MUL_ARR { .. } => DAE::Operator::MUL { ty: inType },
        DAE::Operator::DIV_ARR { .. } => DAE::Operator::DIV { ty: inType },
        _ => inOperator,
    });
    outOperator
}

pub(crate) fn isScalarArrayOp(mut inOperator: &DAE::Operator) -> bool {
    let mut outIsScalarArrayOp: bool;
    outIsScalarArrayOp = (match inOperator.clone() {
        DAE::Operator::SUB_SCALAR_ARRAY { .. } => true,
        DAE::Operator::DIV_SCALAR_ARRAY { .. } => true,
        DAE::Operator::POW_SCALAR_ARRAY { .. } => true,
        _ => false,
    });
    outIsScalarArrayOp
}

pub(crate) fn isArrayScalarOp(mut inOperator: &DAE::Operator) -> bool {
    let mut outIsArrayScalarOp: bool;
    outIsArrayScalarOp = (match inOperator.clone() {
        DAE::Operator::MUL_ARRAY_SCALAR { .. } => true,
        DAE::Operator::ADD_ARRAY_SCALAR { .. } => true,
        DAE::Operator::DIV_ARRAY_SCALAR { .. } => true,
        DAE::Operator::POW_ARRAY_SCALAR { .. } => true,
        _ => false,
    });
    outIsArrayScalarOp
}

pub(crate) fn subscriptsAppend(
    mut inSubscriptLst: &metamodelica::List<metamodelica::Ref<DAE::Subscript>>,
    mut inSubscript: &metamodelica::Ref<DAE::Exp>,
) -> Result<metamodelica::List<metamodelica::Ref<DAE::Subscript>>> {
    let mut outSubscriptLst: metamodelica::List<metamodelica::Ref<DAE::Subscript>>;
    outSubscriptLst = (::match_deref::match_deref! { match inSubscriptLst {
        Deref @ metamodelica::ListNode::Nil => {
            list![metamodelica::Ref::new(DAE::Subscript::INDEX { exp: inSubscript.clone() })]
        },
        Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Subscript::WHOLEDIM { .. }, tail: ss } => {
            metamodelica::cons(metamodelica::Ref::new(DAE::Subscript::INDEX { exp: inSubscript.clone() }), ss.clone())
        },
        Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Subscript::SLICE { exp: e }, tail: Deref @ metamodelica::ListNode::Nil } => {
            let mut e_1: metamodelica::Ref<DAE::Exp>;
            (e_1, _) = ExpressionSimplify::simplify1(makeASUB(e.clone(), list![inSubscript.clone()])?)?;
            list![metamodelica::Ref::new(DAE::Subscript::INDEX { exp: e_1 })]
        },
        Deref @ metamodelica::ListNode::Cons { head: s @ Deref @ DAE::Subscript::INDEX { .. }, tail: Deref @ metamodelica::ListNode::Nil } => {
            list![s.clone(), metamodelica::Ref::new(DAE::Subscript::INDEX { exp: inSubscript.clone() })]
        },
        Deref @ metamodelica::ListNode::Cons { head: s, tail: ss } => {
            let mut ss_1: metamodelica::List<metamodelica::Ref<DAE::Subscript>>;
            ss_1 = subscriptsAppend(ss, inSubscript)?;
            metamodelica::cons(s.clone(), ss_1)
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outSubscriptLst)
}

pub(crate) fn subscriptsReplaceSlice(
    mut inSubscripts: &metamodelica::List<metamodelica::Ref<DAE::Subscript>>,
    mut inSubscript: &metamodelica::Ref<DAE::Subscript>,
) -> Result<metamodelica::List<metamodelica::Ref<DAE::Subscript>>> {
    let mut outSubscripts: metamodelica::List<metamodelica::Ref<DAE::Subscript>>;
    outSubscripts = (::match_deref::match_deref! { match inSubscripts {
        Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Subscript::WHOLEDIM { .. }, tail: rest_subs } => {
            metamodelica::cons(inSubscript.clone(), rest_subs.clone())
        },
        Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Subscript::SLICE { .. }, tail: rest_subs } => {
            metamodelica::cons(inSubscript.clone(), rest_subs.clone())
        },
        Deref @ metamodelica::ListNode::Cons { head: sub, tail: rest_subs } => {
            let mut rest_subs = (*rest_subs).clone();
            rest_subs = subscriptsReplaceSlice(metamodelica::AsArg::as_arg(&rest_subs), inSubscript)?;
            metamodelica::cons(sub.clone(), rest_subs.clone())
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok(outSubscripts)
}

pub fn unliftArrayTypeWithSubs(
    mut subs: metamodelica::List<metamodelica::Ref<DAE::Subscript>>,
    mut ity: metamodelica::Ref<DAE::Type>,
) -> Result<metamodelica::Ref<DAE::Type>> {
    '__tco: loop {
        ::match_deref::match_deref! { match &((subs, ity)) {
            (Deref @ metamodelica::ListNode::Nil, ty) => {
                return Ok(ty.clone())
            },
            (Deref @ metamodelica::ListNode::Cons { head: _, tail: rest }, ty) => {
                let mut ty = (*ty).clone();
                ty = unliftArray(metamodelica::AsArg::as_arg(&ty))?;
                { (subs, ity) = (rest.clone(), ty.clone()); continue '__tco; }
            },
            _ => return Err("match: no arm matched"),
        } }
    }
}

pub fn unliftArrayX(mut inType: metamodelica::Ref<DAE::Type>, mut x: i32) -> Result<metamodelica::Ref<DAE::Type>> {
    '__tco: loop {
        match x {
            0 => return Ok(inType),
            _ => {
                let mut ty: Type;
                ty = unliftArray(&inType)?;
                {
                    (inType, x) = (ty, x - 1);
                    continue '__tco;
                }
            }
        }
    }
}

pub fn arrayAppend(
    mut head: metamodelica::Ref<DAE::Exp>,
    mut rest: &metamodelica::Ref<DAE::Exp>,
) -> Result<metamodelica::Ref<DAE::Exp>> {
    let mut array: metamodelica::Ref<DAE::Exp>;
    array = (::match_deref::match_deref! { match rest {
        Deref @ DAE::Exp::ARRAY { ty: Deref @ DAE::Type::T_ARRAY { ty, dims: Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Dimension::DIM_INTEGER { integer: dim }, tail: dims } }, scalar, array: expl } => {
            let mut dim = (*dim).clone();
            let mut dims = (*dims).clone();
            dim = dim.clone() + 1;
            dims = metamodelica::cons(metamodelica::Ref::new(DAE::Dimension::DIM_INTEGER { integer: dim.clone() }), dims.clone());
            metamodelica::Ref::new(DAE::Exp::ARRAY { ty: metamodelica::Ref::new(DAE::Type::T_ARRAY { ty: ty.clone(), dims: dims.clone() }), scalar: scalar.clone(), array: metamodelica::cons(head, expl.clone()) })
        },
        _ => {
            let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { return Err("pattern mismatch") };
            Debug::traceln(literal!("- Expression.arrayAppend failed."))?;
            return Err("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(array)
}

pub(crate) fn arrayDimensionSetFirst(
    mut inArrayType: &metamodelica::Ref<DAE::Type>,
    mut dimension: metamodelica::Ref<DAE::Dimension>,
) -> Result<metamodelica::Ref<DAE::Type>> {
    let mut outArrayType: metamodelica::Ref<DAE::Type>;
    outArrayType = (::match_deref::match_deref! { match inArrayType {
        Deref @ DAE::Type::T_ARRAY { ty, dims: Deref @ metamodelica::ListNode::Cons { head: _, tail: rest_dims } } => {
            metamodelica::Ref::new(DAE::Type::T_ARRAY { ty: ty.clone(), dims: metamodelica::cons(dimension, rest_dims.clone()) })
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok(outArrayType)
}

/* **************************************************/
/* Getter  */
/* **************************************************/
pub fn toReal<'__b>(mut inExp: &'__b metamodelica::Ref<DAE::Exp>) -> Result<metamodelica::Real> {
    '__tco: loop {
        match &**inExp {
            DAE::Exp::RCONST { .. } => return Ok(var_field!((**inExp).real, DAE::Exp::RCONST).clone()),
            DAE::Exp::ICONST { .. } => return Ok(intReal(var_field!((**inExp).integer, DAE::Exp::ICONST).clone())),
            DAE::Exp::CAST { .. } => {
                inExp = var_field!((**inExp).exp, DAE::Exp::CAST);
                continue '__tco;
            }
            DAE::Exp::ENUM_LITERAL { .. } => {
                return Ok(intReal(var_field!((**inExp).index, DAE::Exp::ENUM_LITERAL).clone()));
            }
            _ => return Err("match: no arm matched"),
        }
    }
}

pub fn toBool(mut inExp: &metamodelica::Ref<DAE::Exp>) -> Result<bool> {
    let mut outBool: bool;
    let __pa0 = ::match_deref::match_deref! { match &((*inExp)) {
        Deref @ DAE::Exp::BCONST { bool: __pa0 } => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    outBool = metamodelica::Own::own(__pa0);
    Ok(outBool)
}

pub fn realExpIntLit(mut exp: &metamodelica::Ref<DAE::Exp>) -> Option<i32> {
    let mut oi: Option<i32>;
    oi = (match &**exp {
        DAE::Exp::RCONST { real: r } => {
            let mut i: i32;
            let mut op: Option<i32>;
            i = ((r.clone()).0.floor() as i32);
            op = if (realEq(r.clone(), intReal(i))) { Some(i) } else { None };
            op
        }
        _ => None,
    });
    oi
}

pub fn expInt(mut exp: &metamodelica::Ref<DAE::Exp>) -> Result<i32> {
    let mut i: i32;
    i = (match &**exp {
        DAE::Exp::ICONST { integer: __exp_integer } => __exp_integer.clone(),
        DAE::Exp::ENUM_LITERAL { index: __exp_index, .. } => __exp_index.clone(),
        DAE::Exp::BCONST { bool: __exp_bool } => {
            if (__exp_bool.clone()) {
                1
            } else {
                0
            }
        }
        _ => return Err("match: no arm matched"),
    });
    Ok(i)
}

pub fn getClockInterval(mut inClk: &metamodelica::Ref<DAE::ClockKind>) -> metamodelica::Ref<DAE::Exp> {
    let mut outIntvl: metamodelica::Ref<DAE::Exp>;
    let mut e: metamodelica::Ref<DAE::Exp>;
    let mut e2: metamodelica::Ref<DAE::Exp>;
    outIntvl = (match &**inClk {
        DAE::ClockKind::REAL_CLOCK { interval: __esc_e } => {
            e = (*__esc_e).clone();
            e.clone()
        }
        DAE::ClockKind::RATIONAL_CLOCK {
            intervalCounter: __esc_e,
            resolution: __esc_e2,
        } => {
            e = (*__esc_e).clone();
            e2 = (*__esc_e2).clone();
            metamodelica::Ref::new(DAE::Exp::BINARY {
                exp1: metamodelica::Ref::new(DAE::Exp::CAST {
                    ty: DAE::T_REAL_DEFAULT().clone(),
                    exp: e.clone(),
                }),
                operator: DAE::Operator::DIV {
                    ty: DAE::T_REAL_DEFAULT().clone(),
                },
                exp2: metamodelica::Ref::new(DAE::Exp::CAST {
                    ty: DAE::T_REAL_DEFAULT().clone(),
                    exp: e2.clone(),
                }),
            })
        }
        DAE::ClockKind::EVENT_CLOCK {
            condition: __esc_e,
            startInterval: __esc_e2,
        } => {
            e = (*__esc_e).clone();
            e2 = (*__esc_e2).clone();
            e2.clone()
        }
        _ => metamodelica::Ref::new(DAE::Exp::RCONST {
            real: metamodelica::OrderedFloat(0.0_f64),
        }),
    });
    outIntvl
}

pub fn sconstEnumNameString(mut exp: &metamodelica::Ref<DAE::Exp>) -> Result<ArcStr> {
    let mut r#str: ArcStr;
    r#str = (match &**exp {
        DAE::Exp::SCONST { string: s } => s.clone(),
        DAE::Exp::ENUM_LITERAL { name, .. } => AbsynUtil::pathString(name.clone(), literal!("."), true, false)?,
        _ => return Err("match: no arm matched"),
    });
    Ok(r#str)
}

pub fn varName(mut v: &metamodelica::Ref<DAE::Var>) -> ArcStr {
    let mut name: ArcStr;
    name = (match &**v {
        DAE::Var { name: __esc_name, .. } => {
            name = (*__esc_name).clone();
            name.clone()
        }
    });
    name
}

pub(crate) fn varType(mut v: &metamodelica::Ref<DAE::Var>) -> metamodelica::Ref<DAE::Type> {
    let mut tp: metamodelica::Ref<DAE::Type>;
    tp = (match &**v {
        DAE::Var { ty: __esc_tp, .. } => {
            tp = (*__esc_tp).clone();
            tp.clone()
        }
    });
    tp
}

pub fn expOrDerCref(mut inExp: &metamodelica::Ref<DAE::Exp>) -> Result<(metamodelica::Ref<DAE::ComponentRef>, bool)> {
    let mut outComponentRef: metamodelica::Ref<DAE::ComponentRef>;
    let mut isDer: bool;
    (outComponentRef, isDer) = (::match_deref::match_deref! { match inExp {
        Deref @ DAE::Exp::CREF { componentRef: cr, .. } => {
            (cr.clone(), false)
        },
        Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "der" }, expLst: Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Exp::CREF { componentRef: cr, ty: _ }, tail: Deref @ metamodelica::ListNode::Nil }, .. } => {
            (cr.clone(), true)
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok((outComponentRef, isDer))
}

pub fn expCref(mut inExp: &metamodelica::Ref<DAE::Exp>) -> Result<metamodelica::Ref<DAE::ComponentRef>> {
    let mut outComponentRef: metamodelica::Ref<DAE::ComponentRef>;
    outComponentRef = (match &**inExp {
        DAE::Exp::CREF { componentRef: cr, .. } => cr.clone(),
        _ => return Err("match: no arm matched"),
    });
    Ok(outComponentRef)
}

pub fn expCrefNegCref(mut inExp: &metamodelica::Ref<DAE::Exp>) -> Result<metamodelica::Ref<DAE::ComponentRef>> {
    let mut outComponentRef: metamodelica::Ref<DAE::ComponentRef>;
    outComponentRef = (::match_deref::match_deref! { match inExp {
        Deref @ DAE::Exp::CREF { componentRef: cr, .. } => {
            cr.clone()
        },
        Deref @ DAE::Exp::UNARY { operator: DAE::Operator::UMINUS { ty: _ }, exp: Deref @ DAE::Exp::CREF { componentRef: cr, .. } } => {
            cr.clone()
        },
        Deref @ DAE::Exp::UNARY { operator: DAE::Operator::UMINUS_ARR { ty: _ }, exp: Deref @ DAE::Exp::CREF { componentRef: cr, .. } } => {
            cr.clone()
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok(outComponentRef)
}

pub(crate) fn expCrefTuple(
    mut inTuple: &(metamodelica::Ref<DAE::Exp>, bool),
) -> Result<metamodelica::Ref<DAE::ComponentRef>> {
    let mut outComponentRef: metamodelica::Ref<DAE::ComponentRef>;
    outComponentRef = (::match_deref::match_deref! { match &(inTuple) {
        (Deref @ DAE::Exp::CREF { componentRef: cr, .. }, _) => {
            cr.clone()
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok(outComponentRef)
}

pub(crate) fn expCrefInclIfExpFactors(
    mut inExp: &metamodelica::Ref<DAE::Exp>,
) -> Result<metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>> {
    let mut outComponentRefs: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
    outComponentRefs = (match &**inExp {
        DAE::Exp::CREF { componentRef: cr, .. } => {
            list![cr.clone()]
        }
        DAE::Exp::IFEXP {
            expCond: _,
            expThen: tb,
            expElse: fb,
        } => {
            let mut f: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
            let mut crefs: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
            f = List::select(
                listAppend(factors(tb)?, factors(fb)?),
                (std::sync::Arc::new(move |__a0: metamodelica::Ref<DAE::Exp>| -> metamodelica::Result<_> {
                    ::std::result::Result::Ok(isCref(&__a0))
                })
                    as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<bool> + 'static>),
            )?;
            crefs = List::map(f, &move |__a0: metamodelica::Ref<DAE::Exp>| expCref(&__a0))?;
            crefs
        }
        _ => return Err("match: no arm matched"),
    });
    Ok(outComponentRefs)
}

pub(crate) fn getArrayContents(
    mut e: &metamodelica::Ref<DAE::Exp>,
) -> Result<metamodelica::List<metamodelica::Ref<DAE::Exp>>> {
    let mut es: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
    let __pa0 = ::match_deref::match_deref! { match &((*e)) {
        Deref @ DAE::Exp::ARRAY { array: __pa0, .. } => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    es = metamodelica::Own::own(__pa0);
    Ok(es)
}

pub fn getArrayOrMatrixContents(
    mut inExp: &metamodelica::Ref<DAE::Exp>,
) -> Result<metamodelica::List<metamodelica::Ref<DAE::Exp>>> {
    let mut outContents: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
    outContents = (::match_deref::match_deref! { match inExp {
        Deref @ DAE::Exp::ARRAY { array: expl, .. } => {
            expl.clone()
        },
        Deref @ DAE::Exp::MATRIX { ty: Deref @ DAE::Type::T_ARRAY { ty: el_ty, dims: Deref @ metamodelica::ListNode::Cons { head: _, tail: dims } }, matrix: mat, .. } => {
            let mut ty: metamodelica::Ref<DAE::Type>;
            let mut sc: bool;
            ty = metamodelica::Ref::new(DAE::Type::T_ARRAY { ty: el_ty.clone(), dims: dims.clone() });
            sc = Types::basicType(metamodelica::AsArg::as_arg(&el_ty));
            List::map2(mat.clone(), &fnptr!(makeArray, metamodelica::List<metamodelica::Ref<DAE::Exp>>, metamodelica::Ref<DAE::Type>, bool), ty, sc)?
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok(outContents)
}

pub fn expandArray(mut exp: &metamodelica::Ref<DAE::Exp>) -> Result<metamodelica::List<metamodelica::Ref<DAE::Exp>>> {
    let mut contents: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
    contents = (match &**exp {
        DAE::Exp::ARRAY { array: __exp_array, .. } => {
            ({
                let mut __acc: metamodelica::List<metamodelica::Ref<DAE::Exp>> = metamodelica::nil();
                for mut e in (__exp_array.clone().reverse()).into_iter().cloned() {
                    let __x = expandArray(&(e.clone()))?;
                    __acc = __x.append(&__acc);
                }
                __acc
            })
        }
        DAE::Exp::MATRIX { .. } => getArrayOrMatrixContents(exp)?,
        _ => list![exp.clone()],
    });
    Ok(contents)
}

fn makeASUBsForDimension(
    mut eIn: metamodelica::Ref<DAE::Exp>,
) -> Result<metamodelica::List<metamodelica::Ref<DAE::Exp>>> {
    let mut eLstOut: metamodelica::List<metamodelica::Ref<DAE::Exp>> = metamodelica::nil();
    let mut size: i32;
    let mut dims: metamodelica::List<metamodelica::Ref<DAE::Dimension>>;
    dims = expDimensions(&eIn)?;
    if '__try0: {
        let __pa1 = ::match_deref::match_deref! { match &(dims.clone()) {
            Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Dimension::DIM_INTEGER { integer: __pa1 }, tail: Deref @ metamodelica::ListNode::Nil } => __pa1.clone(),
            _ => break '__try0 Err::<_, _>("pattern mismatch"),
        } };
        size = metamodelica::Own::own(__pa1);
        for mut i in ({let __s=size; let __e=1; (0i32..).map(move |__k| __s + __k * (-1)).take_while(move |&__v| __v >= __e)}) {
            eLstOut = metamodelica::cons(unwrap_break_err!(makeASUBSingleSub(eIn.clone(), metamodelica::Ref::new(DAE::Exp::ICONST { integer: i })), '__try0), eLstOut.clone());
        }
        Ok::<(), &'static str>(())
    }.is_err() {
        eLstOut = metamodelica::nil();
    }
    Ok(eLstOut)
}

pub fn getComplexContents(mut e: &metamodelica::Ref<DAE::Exp>) -> metamodelica::List<metamodelica::Ref<DAE::Exp>> {
    let mut es: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
    es = 'mc: {
        let __mc_input = &**e;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::CREF { .. } => {
                    let mut noArr: bool;
                    let mut exp: metamodelica::Ref<DAE::Exp>;
                    let mut expLst: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                    expLst = arrayElements(e.clone())?;
                    noArr = ((expLst).len() as i32) == 1;
                    exp = (expLst).head().cloned()?;
                    noArr = noArr && ExpressionBasics::expEqual(&exp, e.clone())?;
                    expLst = if (noArr) {metamodelica::nil()} else {expLst.clone()};
                    Ok(expLst.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::BINARY { exp1, operator: DAE::Operator::ADD_ARR { .. }, exp2 } => {
                    let mut ty: metamodelica::Ref<DAE::Type>;
                    let mut expLst: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                    let mut expLst1: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                    let mut expLst2: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                    if isArray(metamodelica::AsArg::as_arg(&exp1)) {
                        expLst1 = getComplexContents(metamodelica::AsArg::as_arg(&exp1));
                    } else {
                        expLst1 = makeASUBsForDimension(exp1.clone())?;
                    }
                    if isArray(metamodelica::AsArg::as_arg(&exp2)) {
                        expLst2 = getComplexContents(metamodelica::AsArg::as_arg(&exp2));
                    } else {
                        expLst2 = makeASUBsForDimension(exp2.clone())?;
                    }
                    ty = r#typeof((expLst1).head().cloned()?)?;
                    expLst = List::threadMap(expLst1.clone(), expLst2.clone(), &({ let __pe_b1 = DAE::Operator::ADD { ty: ty.clone() }; move |__pe_a0, __pe_a2| Ok(makeBinaryExp(__pe_a0, __pe_b1.clone(), __pe_a2)) }))?;
                    Ok(expLst.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::CALL { expLst, .. } => {
                    let mut expLstLst: metamodelica::List<metamodelica::List<metamodelica::Ref<DAE::Exp>>>;
                    let mut expLst = (*expLst).clone();
                    expLstLst = List::map(expLst.clone(), &fnptr!(getComplexContentsInCall, metamodelica::Ref<DAE::Exp>))?;
                    expLst = List::flatten(expLstLst.clone())?;
                    Ok(expLst.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::RECORD { exps: expLst, .. } => {
                    let mut expLstLst: metamodelica::List<metamodelica::List<metamodelica::Ref<DAE::Exp>>>;
                    let mut expLst = (*expLst).clone();
                    expLstLst = List::map(expLst.clone(), &fnptr!(getComplexContentsInCall, metamodelica::Ref<DAE::Exp>))?;
                    expLst = List::flatten(expLstLst.clone())?;
                    Ok(expLst.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::ARRAY { .. } => {
                    let mut expLst: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                    expLst = arrayElements(e.clone())?;
                    Ok(expLst.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::MATRIX { matrix: expLstLst, .. } => {
                    let mut expLst: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                    expLst = List::flatten(expLstLst.clone())?;
                    Ok(expLst.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::TUPLE { PR: expLst } => {
                    Ok(expLst.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::CAST { exp, .. } => {
                    let mut expLst: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                    expLst = getComplexContents(metamodelica::AsArg::as_arg(&exp));
                    Ok(expLst.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::ASUB { exp, .. } => {
                    let mut expLst: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                    expLst = getComplexContents(metamodelica::AsArg::as_arg(&exp));
                    Ok(expLst.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(metamodelica::nil())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    es
}

fn getComplexContentsInCall(mut expIn: metamodelica::Ref<DAE::Exp>) -> metamodelica::List<metamodelica::Ref<DAE::Exp>> {
    let mut expsOut: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
    let mut expLst: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
    expLst = getComplexContents(&expIn);
    expsOut = if ((expLst).is_empty()) { list![expIn] } else { expLst };
    expsOut
}

pub fn getArrayOrRangeContents(
    mut e: metamodelica::Ref<DAE::Exp>,
) -> Result<metamodelica::List<metamodelica::Ref<DAE::Exp>>> {
    let mut es: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
    es = (::match_deref::match_deref! { match &(e.clone()) {
        Deref @ DAE::Exp::ARRAY { array: __esc_es, .. } => {
            es = (*__esc_es).clone();
            es.clone()
        },
        Deref @ DAE::Exp::MATRIX { matrix, ty, .. } => {
            let mut ty = (*ty).clone();
            ty = Types::unliftArray(metamodelica::AsArg::as_arg(&ty))?;
            es = List::map2(matrix.clone(), &fnptr!(makeArray, metamodelica::List<metamodelica::Ref<DAE::Exp>>, metamodelica::Ref<DAE::Type>, bool), ty.clone(), !(Types::arrayType(metamodelica::AsArg::as_arg(&ty))))?;
            es
        },
        Deref @ DAE::Exp::CREF { ty: Deref @ DAE::Type::T_ARRAY { dims: Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Dimension::DIM_INTEGER { integer: istop }, tail: _ }, .. }, .. } => {
            es = List::map(ExpressionSimplify::simplifyRange(1, 1, istop.clone())?, &fnptr!(makeIntegerExp, i32))?;
            es = List::map1r(es, &makeASUBSingleSub, e)?;
            es
        },
        Deref @ DAE::Exp::RANGE { start: Deref @ DAE::Exp::BCONST { bool: bstart }, step: None, stop: Deref @ DAE::Exp::BCONST { bool: bstop }, .. } => {
            List::map(ExpressionSimplify::simplifyRangeBool(bstart.clone(), bstop.clone()), &fnptr!(makeBoolExp, bool))?
        },
        Deref @ DAE::Exp::RANGE { start: Deref @ DAE::Exp::ICONST { integer: istart }, step: None, stop: Deref @ DAE::Exp::ICONST { integer: istop }, .. } => {
            List::map(ExpressionSimplify::simplifyRange(istart.clone(), 1, istop.clone())?, &fnptr!(makeIntegerExp, i32))?
        },
        Deref @ DAE::Exp::RANGE { start: Deref @ DAE::Exp::ICONST { integer: istart }, step: Some(Deref @ DAE::Exp::ICONST { integer: istep }), stop: Deref @ DAE::Exp::ICONST { integer: istop }, .. } => {
            List::map(ExpressionSimplify::simplifyRange(istart.clone(), istep.clone(), istop.clone())?, &fnptr!(makeIntegerExp, i32))?
        },
        Deref @ DAE::Exp::RANGE { start: Deref @ DAE::Exp::RCONST { real: rstart }, step: None, stop: Deref @ DAE::Exp::RCONST { real: rstop }, .. } => {
            List::map(ExpressionSimplify::simplifyRangeReal(rstart.clone(), metamodelica::OrderedFloat(1.0_f64), rstop.clone())?, &fnptr!(makeRealExp, metamodelica::Real))?
        },
        Deref @ DAE::Exp::RANGE { start: Deref @ DAE::Exp::RCONST { real: rstart }, step: Some(Deref @ DAE::Exp::RCONST { real: rstep }), stop: Deref @ DAE::Exp::RCONST { real: rstop }, .. } => {
            List::map(ExpressionSimplify::simplifyRangeReal(rstart.clone(), rstep.clone(), rstop.clone())?, &fnptr!(makeRealExp, metamodelica::Real))?
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok(es)
}

pub(crate) fn get2dArrayOrMatrixContent(
    mut e: &metamodelica::Ref<DAE::Exp>,
) -> Result<metamodelica::List<metamodelica::List<metamodelica::Ref<DAE::Exp>>>> {
    let mut outExps: metamodelica::List<metamodelica::List<metamodelica::Ref<DAE::Exp>>>;
    outExps = (match &**e {
        DAE::Exp::ARRAY { array: es, .. } => List::map(es.clone(), &move |__a0: metamodelica::Ref<DAE::Exp>| {
            getArrayContents(&__a0)
        })?,
        DAE::Exp::MATRIX { matrix: ess, .. } => ess.clone(),
        _ => return Err("match: no arm matched"),
    });
    Ok(outExps)
}

// stefan
pub(crate) fn unboxExpType(mut inType: metamodelica::Ref<DAE::Type>) -> metamodelica::Ref<DAE::Type> {
    let mut outType: metamodelica::Ref<DAE::Type>;
    outType = (match &*inType {
        DAE::Type::T_METABOXED { ty } => ty.clone(),
        _ => inType,
    });
    outType
}

pub fn unboxExp<'__b>(mut ie: &'__b metamodelica::Ref<DAE::Exp>) -> metamodelica::Ref<DAE::Exp> {
    '__tco: loop {
        match &**ie {
            DAE::Exp::BOX { exp: e } => {
                ie = e;
                continue '__tco;
            }
            _ => return ie.clone(),
        }
    }
}

pub(crate) fn boxExp(mut e: metamodelica::Ref<DAE::Exp>) -> metamodelica::Ref<DAE::Exp> {
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    outExp = (match &*e {
        DAE::Exp::BOX { exp: _ } => e,
        _ => metamodelica::Ref::new(DAE::Exp::BOX { exp: e }),
    });
    outExp
}

pub fn getSubscriptExp(mut inSubscript: &metamodelica::Ref<DAE::Subscript>) -> Result<metamodelica::Ref<DAE::Exp>> {
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    outExp = (match &**inSubscript {
        DAE::Subscript::SLICE { exp: e } => e.clone(),
        DAE::Subscript::INDEX { exp: e } => e.clone(),
        DAE::Subscript::WHOLE_NONEXP { exp: e } => e.clone(),
        _ => return Err("match: no arm matched"),
    });
    Ok(outExp)
}

pub(crate) fn subscriptNonExpandedExp(
    mut inSubscript: &metamodelica::Ref<DAE::Subscript>,
) -> Result<metamodelica::Ref<DAE::Exp>> {
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    outExp = (match &**inSubscript {
        DAE::Subscript::WHOLE_NONEXP { exp: e } => e.clone(),
        _ => return Err("match: no arm matched"),
    });
    Ok(outExp)
}

pub(crate) fn subscriptIsFirst(mut inSubscript: &metamodelica::Ref<DAE::Subscript>) -> Result<bool> {
    let mut outIsFirst: bool;
    outIsFirst = (::match_deref::match_deref! { match inSubscript {
        Deref @ DAE::Subscript::INDEX { exp: Deref @ DAE::Exp::ICONST { integer: 1 } } => true,
        Deref @ DAE::Subscript::INDEX { exp: Deref @ DAE::Exp::BCONST { bool: false } } => true,
        Deref @ DAE::Subscript::INDEX { exp: Deref @ DAE::Exp::ENUM_LITERAL { index: 1, .. } } => true,
        _ => return Err("match: no arm matched"),
    } });
    Ok(outIsFirst)
}

pub fn nthArrayExp(mut inExp: &metamodelica::Ref<DAE::Exp>, mut inInteger: i32) -> metamodelica::Ref<DAE::Exp> {
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    outExp = 'mc: {
        let __mc_input = &**inExp;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::BINARY { operator: op, exp1: e1, exp2: e2 } => {
                    let mut e_1: metamodelica::Ref<DAE::Exp>;
                    let mut e_2: metamodelica::Ref<DAE::Exp>;
                    let mut ty: Type;
                    ty = typeofOp(metamodelica::AsArg::as_arg(&op));
                    let true = (Types::isArray(&ty)) else { return Err("pattern mismatch") };
                    e_1 = nthArrayExp(metamodelica::AsArg::as_arg(&e1), inInteger);
                    e_2 = nthArrayExp(metamodelica::AsArg::as_arg(&e2), inInteger);
                    Ok(metamodelica::Ref::new(DAE::Exp::BINARY { exp1: e_1.clone(), operator: op.clone(), exp2: e_2.clone() }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::ARRAY { array: expl, .. } => {
                    let mut e1: metamodelica::Ref<DAE::Exp>;
                    e1 = (expl).get(inInteger)?;
                    Ok(e1.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(inExp.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    outExp
}

pub(crate) fn expLastSubs<'__b>(
    mut inExp: &'__b metamodelica::Ref<DAE::Exp>,
) -> Result<metamodelica::List<metamodelica::Ref<DAE::Subscript>>> {
    '__tco: loop {
        match &**inExp {
            DAE::Exp::CREF { componentRef: cr, .. } => {
                let mut subs: metamodelica::List<metamodelica::Ref<DAE::Subscript>>;
                return Ok(ComponentReference::crefLastSubs(cr)?);
            }
            DAE::Exp::UNARY { exp: e, .. } => {
                let mut subs: metamodelica::List<metamodelica::Ref<DAE::Subscript>>;
                {
                    inExp = e;
                    continue '__tco;
                }
            }
            _ => return Err("match: no arm matched"),
        }
    }
}

pub fn expDimensions<'__b>(
    mut inExp: &'__b metamodelica::Ref<DAE::Exp>,
) -> Result<metamodelica::List<metamodelica::Ref<DAE::Dimension>>> {
    '__tco: loop {
        ::match_deref::match_deref! { match inExp {
            Deref @ DAE::Exp::ARRAY { ty: tp, .. } => {
                return Ok(arrayDimension(tp))
            },
            Deref @ DAE::Exp::MATRIX { ty: tp, .. } => {
                return Ok(arrayDimension(tp))
            },
            Deref @ DAE::Exp::LUNARY { exp: e, .. } => {
                { inExp = e; continue '__tco; }
            },
            Deref @ DAE::Exp::LBINARY { exp1: e, .. } => {
                { inExp = e; continue '__tco; }
            },
            Deref @ DAE::Exp::CALL { attr: Deref @ DAE::CallAttributes { ty: tp, .. }, .. } => {
                return Ok(arrayDimension(metamodelica::AsArg::as_arg(&tp)))
            },
            _ => return Err("match: no arm matched"),
        } }
    }
}

pub fn arrayDimension(mut tp: &metamodelica::Ref<DAE::Type>) -> metamodelica::List<metamodelica::Ref<DAE::Dimension>> {
    let mut dims: metamodelica::List<metamodelica::Ref<DAE::Dimension>>;
    dims = (match &**tp {
        DAE::Type::T_ARRAY { dims: __esc_dims, .. } => {
            dims = (*__esc_dims).clone();
            dims.clone()
        }
        _ => metamodelica::nil(),
    });
    dims
}

pub(crate) fn arrayTypeDimensions(
    mut tp: &metamodelica::Ref<DAE::Type>,
) -> Result<metamodelica::List<metamodelica::Ref<DAE::Dimension>>> {
    let mut dims: metamodelica::List<metamodelica::Ref<DAE::Dimension>>;
    dims = (match &**tp {
        DAE::Type::T_ARRAY { dims: __esc_dims, .. } => {
            dims = (*__esc_dims).clone();
            dims.clone()
        }
        _ => return Err("match: no arm matched"),
    });
    Ok(dims)
}

pub fn subscriptDimensions(
    mut inSubscripts: metamodelica::List<metamodelica::Ref<DAE::Subscript>>,
) -> Result<metamodelica::List<metamodelica::Ref<DAE::Dimension>>> {
    let mut outDimensions: metamodelica::List<metamodelica::Ref<DAE::Dimension>>;
    outDimensions = List::map(inSubscripts, &move |__a0: metamodelica::Ref<DAE::Subscript>| {
        subscriptDimension(&__a0)
    })?;
    Ok(outDimensions)
}

pub(crate) fn subscriptDimension(
    mut inSubscript: &metamodelica::Ref<DAE::Subscript>,
) -> Result<metamodelica::Ref<DAE::Dimension>> {
    let mut outDimension: metamodelica::Ref<DAE::Dimension>;
    outDimension = (::match_deref::match_deref! { match inSubscript {
        Deref @ DAE::Subscript::INDEX { exp: Deref @ DAE::Exp::ICONST { integer: x } } => {
            metamodelica::Ref::new(DAE::Dimension::DIM_INTEGER { integer: x.clone() })
        },
        Deref @ DAE::Subscript::INDEX { exp: e } => {
            metamodelica::Ref::new(DAE::Dimension::DIM_EXP { exp: e.clone() })
        },
        Deref @ DAE::Subscript::WHOLEDIM { .. } => {
            openmodelica_frontend_types::DAE::Dimension::interned_DIM_UNKNOWN()
        },
        Deref @ DAE::Subscript::WHOLE_NONEXP { exp: Deref @ DAE::Exp::ICONST { integer: x } } if (!(Config::splitArrays()?)) => {
            metamodelica::Ref::new(DAE::Dimension::DIM_INTEGER { integer: x.clone() })
        },
        Deref @ DAE::Subscript::WHOLE_NONEXP { exp: e } if (!(Config::splitArrays()?)) => {
            metamodelica::Ref::new(DAE::Dimension::DIM_EXP { exp: e.clone() })
        },
        _ => {
            let mut sub_str: ArcStr;
            let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { return Err("pattern mismatch") };
            sub_str = ExpressionDump::subscriptString(inSubscript)?;
            Debug::traceln({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("- Expression.subscriptDimension failed on ")); __mm_s.push_str(&*sub_str); ArcStr::from(__mm_s) })?;
            return Err("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outDimension)
}

pub fn arrayEltType<'__b>(mut inType: &'__b metamodelica::Ref<DAE::Type>) -> metamodelica::Ref<DAE::Type> {
    '__tco: loop {
        match &**inType {
            DAE::Type::T_ARRAY { ty: t, .. } => {
                inType = t;
                continue '__tco;
            }
            _ => return inType.clone(),
        }
    }
}

pub fn sizeOf(mut inType: &metamodelica::Ref<DAE::Type>) -> i32 {
    let mut i: i32;
    i = 'mc: {
        let __mc_input = &**inType;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                        Deref @ DAE::Type::T_ARRAY { .. } => {
                            Ok(sizeOf(var_field!((**inType).ty, DAE::Type::T_ARRAY)) * ({
                let mut __acc: i32 = 1;
                for mut d in (var_field!((**inType).dims, DAE::Type::T_ARRAY).clone()).into_iter().cloned() {
                    let __x = dimensionSize(&(d.clone()))?;
                    __acc *= __x;
                }
                __acc
            }))
                        }
                        _ => return Err("nomatch"),
                    }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Type::T_COMPLEX { complexClassType: ClassInf::State::EXTERNAL_OBJ { .. }, .. } => {
                    Ok(0)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                        Deref @ DAE::Type::T_COMPLEX { .. } => {
                            Ok(({
                let mut __acc: i32 = 0;
                for mut v in (var_field!((**inType).varLst, DAE::Type::T_COMPLEX).clone()).into_iter().cloned() {
                    let __x = sizeOf(&(varType(&(v.clone()))));
                    __acc += __x;
                }
                __acc
            }))
                        }
                        _ => return Err("nomatch"),
                    }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                        Deref @ DAE::Type::T_TUPLE { .. } => {
                            Ok(({
                let mut __acc: i32 = 0;
                for mut ty in (var_field!((**inType).types, DAE::Type::T_TUPLE).clone()).into_iter().cloned() {
                    let __x = sizeOf(&(ty.clone()));
                    __acc += __x;
                }
                __acc
            }))
                        }
                        _ => return Err("nomatch"),
                    }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Type::T_FUNCTION { .. } => {
                    Ok(sizeOf(var_field!((**inType).funcResultType, DAE::Type::T_FUNCTION)))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Type::T_METATYPE { .. } => {
                    Ok(sizeOf(var_field!((**inType).ty, DAE::Type::T_METATYPE)))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Type::T_UNKNOWN { .. } => {
                    Ok(0)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(1)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    i
}

pub fn dimensionSize(mut dim: &metamodelica::Ref<DAE::Dimension>) -> Result<i32> {
    let mut value: i32;
    value = (::match_deref::match_deref! { match dim {
        Deref @ DAE::Dimension::DIM_INTEGER { integer: i } => {
            i.clone()
        },
        Deref @ DAE::Dimension::DIM_ENUM { size: i, .. } => {
            i.clone()
        },
        Deref @ DAE::Dimension::DIM_BOOLEAN { .. } => {
            2
        },
        Deref @ DAE::Dimension::DIM_EXP { exp: Deref @ DAE::Exp::ICONST { integer: i } } => {
            i.clone()
        },
        Deref @ DAE::Dimension::DIM_EXP { exp: Deref @ DAE::Exp::ENUM_LITERAL { index: i, .. } } => {
            i.clone()
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok(value)
}

pub(crate) fn addDimensions(
    mut dim1: &metamodelica::Ref<DAE::Dimension>,
    mut dim2: &metamodelica::Ref<DAE::Dimension>,
) -> metamodelica::Ref<DAE::Dimension> {
    let mut dim: metamodelica::Ref<DAE::Dimension>;
    dim = 'mc: {
        let __mc_input = &**dim2;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut i: i32;
                    i = dimensionSize(dim1)? + dimensionSize(dim2)?;
                    Ok(metamodelica::Ref::new(DAE::Dimension::DIM_INTEGER { integer: i }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(openmodelica_frontend_types::DAE::Dimension::interned_DIM_UNKNOWN())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    dim
}

pub(crate) fn dimensionSizeAll(mut dim: &metamodelica::Ref<DAE::Dimension>) -> Result<i32> {
    let mut value: i32;
    value = 'mc: {
        let __mc_input = &**dim;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Dimension::DIM_INTEGER { integer: i } => {
                    Ok(i.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Dimension::DIM_ENUM { size: i, .. } => {
                    Ok(i.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Dimension::DIM_BOOLEAN { .. } => {
                    Ok(2)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Dimension::DIM_EXP { exp: e } => {
                    Ok(expInt(metamodelica::AsArg::as_arg(&e))?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Dimension::DIM_EXP { .. } => {
                    let true = (Flags::getConfigBool(Flags::CHECK_MODEL.clone())?) else { return Err("pattern mismatch") };
                    Ok(0)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Dimension::DIM_UNKNOWN { .. } => {
                    let true = (Flags::getConfigBool(Flags::CHECK_MODEL.clone())?) else { return Err("pattern mismatch") };
                    Ok(0)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(value)
}

pub fn dimensionsSizes(
    mut inDims: metamodelica::List<metamodelica::Ref<DAE::Dimension>>,
) -> Result<metamodelica::List<i32>> {
    let mut outValues: metamodelica::List<i32>;
    outValues = List::map(inDims, &move |__a0: metamodelica::Ref<DAE::Dimension>| {
        dimensionSizeAll(&__a0)
    })?;
    Ok(outValues)
}

pub fn r#typeof(mut inExp: metamodelica::Ref<DAE::Exp>) -> Result<metamodelica::Ref<DAE::Type>> {
    let mut outType: metamodelica::Ref<DAE::Type>;
    outType = 'mc: {
        let __mc_input = inExp.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::ICONST { .. } => {
                    Ok(DAE::T_INTEGER_DEFAULT().clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::RCONST { .. } => {
                    Ok(DAE::T_REAL_DEFAULT().clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::SCONST { .. } => {
                    Ok(DAE::T_STRING_DEFAULT().clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::BCONST { .. } => {
                    Ok(DAE::T_BOOL_DEFAULT().clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::CLKCONST { .. } => {
                    Ok(DAE::T_CLOCK_DEFAULT().clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::ENUM_LITERAL { name: p, index: i } => {
                    Ok(metamodelica::Ref::new(DAE::Type::T_ENUMERATION { index: Some(i.clone()), path: p.clone(), names: metamodelica::nil(), literalVarLst: metamodelica::nil(), attributeLst: metamodelica::nil() }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::CREF { ty: tp, .. } => {
                    Ok(tp.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::BINARY { operator: op, .. } => {
                    Ok(typeofOp(metamodelica::AsArg::as_arg(&op)))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::UNARY { operator: op, .. } => {
                    Ok(typeofOp(metamodelica::AsArg::as_arg(&op)))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::LBINARY { operator: op, .. } => {
                    Ok(typeofOp(metamodelica::AsArg::as_arg(&op)))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::LUNARY { operator: op, .. } => {
                    Ok(typeofOp(metamodelica::AsArg::as_arg(&op)))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::RELATION { operator: op, .. } => {
                    Ok(typeofRelation(&(typeofOp(metamodelica::AsArg::as_arg(&op)))))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::IFEXP { expThen: e2, .. } => {
                    Ok(r#typeof(e2.clone())?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::CALL { attr: Deref @ DAE::CallAttributes { ty: tp, .. }, .. } => {
                    Ok(tp.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::RECORD { ty: tp, .. } => {
                    Ok(tp.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::PARTEVALFUNCTION { ty: tp, .. } => {
                    Ok(tp.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::ARRAY { ty: tp, .. } => {
                    Ok(tp.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::MATRIX { ty: tp, .. } => {
                    Ok(tp.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::RANGE { ty: tp, .. } => {
                    Ok(tp.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::CAST { ty: tp, .. } => {
                    Ok(tp.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                        Deref @ DAE::Exp::ASUB { exp: e, sub: subs } => {
                            let mut tp: Type;
                            let mut explist: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                            let mut i: i32;
                            if Config::acceptMetaModelicaGrammar()? {
                                explist = ({
                let mut __acc: metamodelica::List<metamodelica::Ref<DAE::Exp>> = metamodelica::nil();
                for mut sub in (subs.clone()).into_iter().cloned() {
                            let __x = getSubscriptExp(&(sub.clone()))?;
                            __acc = cons(__x, __acc);
                }
                __acc.reverse()
            });
                                i = ({
                let mut __acc: i32 = 0;
                for mut e in (explist.clone()).into_iter().cloned() {
                            if !(isScalar(metamodelica::AsArg::as_arg(&e))?) { continue; }
                            let __x = 1;
                            __acc += __x;
                }
                __acc
            });
                            } else {
                                i = ({
                let mut __acc: i32 = 0;
                for mut sub in (subs.clone()).into_iter().cloned() {
                            if !(isScalarSubscript(&(sub.clone()))?) { continue; }
                            let __x = 1;
                            __acc += __x;
                }
                __acc
            });
                            }
                            tp = unliftArrayX(r#typeof(e.clone())?, i)?;
                            Ok(tp.clone())
                        }
                        _ => return Err("nomatch"),
                    }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::TSUB { ty: tp, .. } => {
                    Ok(tp.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::RSUB { .. } => {
                    Ok(var_field!((*inExp).ty, DAE::Exp::RSUB).clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::CODE { ty: tp, .. } => {
                    Ok(tp.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::REDUCTION { iterators: Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::ReductionIterator { exp: iterExp, guardExp: None, .. }, tail: Deref @ metamodelica::ListNode::Nil }, expr: operExp, reductionInfo: Deref @ DAE::ReductionInfo { exprType: Deref @ DAE::Type::T_ARRAY { dims: Deref @ metamodelica::ListNode::Cons { head: dim, tail: _ }, .. }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "array" }, .. } } => {
                    let mut tp: Type;
                    let mut iterTp: metamodelica::Ref<DAE::Type>;
                    let mut operTp: metamodelica::Ref<DAE::Type>;
                    let mut iterdims: metamodelica::List<metamodelica::Ref<DAE::Dimension>>;
                    let false = (dimensionKnown(metamodelica::AsArg::as_arg(&dim))) else { return Err("pattern mismatch") };
                    iterTp = r#typeof(iterExp.clone())?;
                    operTp = r#typeof(operExp.clone())?;
                    let __pa0 = ::match_deref::match_deref! { match &(iterTp.clone()) {
                        Deref @ DAE::Type::T_ARRAY { dims: __pa0, .. } => __pa0.clone(),
                        _ => return Err("pattern mismatch"),
                    } };
                    iterdims = metamodelica::Own::own(__pa0);
                    tp = Types::liftTypeWithDims(operTp.clone(), iterdims.clone())?;
                    Ok(tp.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::REDUCTION { reductionInfo: Deref @ DAE::ReductionInfo { exprType: ty, .. }, .. } => {
                    Ok(Types::simplifyType(ty.clone())?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::SIZE { exp: _, sz: None } => {
                    Ok(metamodelica::Ref::new(DAE::Type::T_ARRAY { ty: DAE::T_INTEGER_DEFAULT().clone(), dims: list![openmodelica_frontend_types::DAE::Dimension::interned_DIM_UNKNOWN()] }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::SIZE { exp: _, sz: Some(_) } => {
                    Ok(DAE::T_INTEGER_DEFAULT().clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::LIST { .. } => {
                    Ok(metamodelica::Ref::new(DAE::Type::T_METATYPE { ty: DAE::T_METALIST_DEFAULT().clone() }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::CONS { .. } => {
                    Ok(metamodelica::Ref::new(DAE::Type::T_METATYPE { ty: DAE::T_METALIST_DEFAULT().clone() }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::META_TUPLE { listExp: exps } => {
                    let mut tys: metamodelica::List<metamodelica::Ref<DAE::Type>>;
                    tys = List::map(exps.clone(), &r#typeof)?;
                    Ok(metamodelica::Ref::new(DAE::Type::T_METATYPE { ty: metamodelica::Ref::new(DAE::Type::T_METATUPLE { types: tys.clone() }) }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::TUPLE { PR: exps } => {
                    let mut tys: metamodelica::List<metamodelica::Ref<DAE::Type>>;
                    tys = List::map(exps.clone(), &r#typeof)?;
                    Ok(metamodelica::Ref::new(DAE::Type::T_TUPLE { types: tys.clone(), names: None }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::META_OPTION { .. } => {
                    Ok(metamodelica::Ref::new(DAE::Type::T_METATYPE { ty: DAE::T_NONE_DEFAULT().clone() }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::METARECORDCALL { path: p, index: i, typeVars, .. } => {
                    Ok(metamodelica::Ref::new(DAE::Type::T_METATYPE { ty: metamodelica::Ref::new(DAE::Type::T_METARECORD { path: p.clone(), utPath: AbsynUtil::stripLast(metamodelica::AsArg::as_arg(&p))?, typeVars: typeVars.clone(), index: i.clone(), fields: metamodelica::nil(), knownSingleton: false }) }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::BOX { exp: e } => {
                    Ok(metamodelica::Ref::new(DAE::Type::T_METATYPE { ty: metamodelica::Ref::new(DAE::Type::T_METABOXED { ty: r#typeof(e.clone())? }) }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::MATCHEXPRESSION { et: tp, .. } => {
                    Ok(tp.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::UNBOX { ty: tp, .. } => {
                    Ok(tp.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::SHARED_LITERAL { exp: e, .. } => {
                    Ok(r#typeof(e.clone())?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::EMPTY { ty: tp, .. } => {
                    Ok(tp.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                e => {
                    let mut msg: ArcStr;
                    msg = { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("- Expression.typeof failed for ")); __mm_s.push_str(&*printExpStr(e.clone())?); ArcStr::from(__mm_s) };
                    Error::addMessage(Error::INTERNAL_ERROR.clone(), list![msg.clone()])?;
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(outType)
}

fn typeofRelation(mut inType: &metamodelica::Ref<DAE::Type>) -> metamodelica::Ref<DAE::Type> {
    let mut outType: metamodelica::Ref<DAE::Type>;
    outType = (match &**inType {
        DAE::Type::T_ARRAY { ty, dims } => {
            typeofRelation(ty);
            metamodelica::Ref::new(DAE::Type::T_ARRAY {
                ty: ty.clone(),
                dims: dims.clone(),
            })
        }
        _ => DAE::T_BOOL_DEFAULT().clone(),
    });
    outType
}

pub(crate) fn typeofOp(mut inOperator: &DAE::Operator) -> metamodelica::Ref<DAE::Type> {
    let mut outType: metamodelica::Ref<DAE::Type>;
    outType = (match inOperator.clone() {
        DAE::Operator::ADD { ty: ref t } => t.clone(),
        DAE::Operator::SUB { ty: ref t } => t.clone(),
        DAE::Operator::MUL { ty: ref t } => t.clone(),
        DAE::Operator::DIV { ty: ref t } => t.clone(),
        DAE::Operator::POW { ty: ref t } => t.clone(),
        DAE::Operator::UMINUS { ty: ref t } => t.clone(),
        DAE::Operator::UMINUS_ARR { ty: ref t } => t.clone(),
        DAE::Operator::ADD_ARR { ty: ref t } => t.clone(),
        DAE::Operator::SUB_ARR { ty: ref t } => t.clone(),
        DAE::Operator::MUL_ARR { ty: ref t } => t.clone(),
        DAE::Operator::DIV_ARR { ty: ref t } => t.clone(),
        DAE::Operator::MUL_ARRAY_SCALAR { ty: ref t } => t.clone(),
        DAE::Operator::ADD_ARRAY_SCALAR { ty: ref t } => t.clone(),
        DAE::Operator::SUB_SCALAR_ARRAY { ty: ref t } => t.clone(),
        DAE::Operator::MUL_SCALAR_PRODUCT { ty: ref t } => t.clone(),
        DAE::Operator::MUL_MATRIX_PRODUCT { ty: ref t } => t.clone(),
        DAE::Operator::DIV_ARRAY_SCALAR { ty: ref t } => t.clone(),
        DAE::Operator::DIV_SCALAR_ARRAY { ty: ref t } => t.clone(),
        DAE::Operator::POW_ARRAY_SCALAR { ty: ref t } => t.clone(),
        DAE::Operator::POW_SCALAR_ARRAY { ty: ref t } => t.clone(),
        DAE::Operator::POW_ARR { ty: ref t } => t.clone(),
        DAE::Operator::POW_ARR2 { ty: ref t } => t.clone(),
        DAE::Operator::AND { ty: ref t } => t.clone(),
        DAE::Operator::OR { ty: ref t } => t.clone(),
        DAE::Operator::NOT { ty: ref t } => t.clone(),
        DAE::Operator::LESS { ty: ref t } => t.clone(),
        DAE::Operator::LESSEQ { ty: ref t } => t.clone(),
        DAE::Operator::GREATER { ty: ref t } => t.clone(),
        DAE::Operator::GREATEREQ { ty: ref t } => t.clone(),
        DAE::Operator::EQUAL { ty: ref t } => t.clone(),
        DAE::Operator::NEQUAL { ty: ref t } => t.clone(),
        DAE::Operator::USERDEFINED { .. } => DAE::T_UNKNOWN_DEFAULT().clone(),
    });
    outType
}

pub(crate) fn getRelations(mut inExp: metamodelica::Ref<DAE::Exp>) -> metamodelica::List<metamodelica::Ref<DAE::Exp>> {
    '__tco: loop {
        ::match_deref::match_deref! { match &(inExp) {
            e @ Deref @ DAE::Exp::RELATION { .. } => {
                return list![e.clone()]
            },
            Deref @ DAE::Exp::LBINARY { exp1: e1, exp2: e2, .. } => {
                let mut rellst1: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                let mut rellst2: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                let mut rellst: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                rellst1 = getRelations(e1.clone());
                rellst2 = getRelations(e2.clone());
                return listAppend(rellst1, rellst2)
            },
            Deref @ DAE::Exp::LUNARY { exp: e, .. } => {
                let mut rellst: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                { inExp = e.clone(); continue '__tco; }
            },
            Deref @ DAE::Exp::BINARY { exp1: e1, exp2: e2, .. } => {
                let mut rellst1: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                let mut rellst2: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                let mut rellst: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                rellst1 = getRelations(e1.clone());
                rellst2 = getRelations(e2.clone());
                return listAppend(rellst1, rellst2)
            },
            Deref @ DAE::Exp::IFEXP { expCond: cond, expThen: tb, expElse: fb } => {
                let mut rellst1: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                let mut rellst2: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                let mut rellst: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                let mut rellst3: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                let mut rellst4: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                rellst1 = getRelations(cond.clone());
                rellst2 = getRelations(tb.clone());
                rellst3 = getRelations(fb.clone());
                rellst4 = listAppend(rellst1, rellst2);
                return listAppend(rellst3, rellst4)
            },
            Deref @ DAE::Exp::ARRAY { array: Deref @ metamodelica::ListNode::Cons { head: e, tail: Deref @ metamodelica::ListNode::Nil }, .. } => {
                let mut rellst: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                { inExp = e.clone(); continue '__tco; }
            },
            Deref @ DAE::Exp::ARRAY { ty: t, scalar: sc, array: Deref @ metamodelica::ListNode::Cons { head: e, tail: xs } } => {
                let mut rellst1: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                let mut rellst2: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                let mut rellst: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                rellst1 = getRelations(metamodelica::Ref::new(DAE::Exp::ARRAY { ty: t.clone(), scalar: sc.clone(), array: xs.clone() }));
                rellst2 = getRelations(e.clone());
                return listAppend(rellst1, rellst2)
            },
            Deref @ DAE::Exp::UNARY { exp: e, .. } => {
                let mut rellst: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                { inExp = e.clone(); continue '__tco; }
            },
            _ => {
                return metamodelica::nil()
            },
            _ => unreachable!("tail-call lowered match: no arm matched"),
        } }
    }
}

pub fn getAllCrefs(
    mut inExp: metamodelica::Ref<DAE::Exp>,
) -> Result<metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>> {
    let mut outCrefs: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
    (_, outCrefs) = traverseExpBottomUp(inExp, &getAllCrefs2, metamodelica::nil())?;
    Ok(outCrefs)
}

fn getAllCrefs2(
    mut inExp: metamodelica::Ref<DAE::Exp>,
    mut inCrefList: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
) -> Result<(
    metamodelica::Ref<DAE::Exp>,
    metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
)> {
    let mut outExp: metamodelica::Ref<DAE::Exp> = inExp.clone();
    let mut outCrefList: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>> = inCrefList.clone();
    let mut cr: metamodelica::Ref<DAE::ComponentRef>;
    if isCref(&inExp) {
        let __pa0 = ::match_deref::match_deref! { match &(inExp) {
            Deref @ DAE::Exp::CREF { componentRef: __pa0, .. } => __pa0.clone(),
            _ => return Err("pattern mismatch"),
        } };
        cr = metamodelica::Own::own(__pa0);
        if !(ComponentReferenceBasics::crefEqual(&cr, &(DAE::crefTime().clone()))?)
            && !(listMember(cr.clone(), inCrefList))
        {
            outCrefList = metamodelica::cons(cr, outCrefList);
        }
    }
    Ok((outExp, outCrefList))
}

pub fn allTerms(mut inExp: &metamodelica::Ref<DAE::Exp>) -> metamodelica::List<metamodelica::Ref<DAE::Exp>> {
    let mut outExpLst: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
    outExpLst = 'mc: {
        let __mc_input = &**inExp;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::BINARY { exp1: e1, operator: DAE::Operator::ADD { .. }, exp2: e2 } => {
                    let mut f1: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                    let mut f2: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                    let mut res: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                    f1 = allTerms(metamodelica::AsArg::as_arg(&e1));
                    f2 = allTerms(metamodelica::AsArg::as_arg(&e2));
                    res = listAppend(f1.clone(), f2.clone());
                    Ok(res.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::BINARY { exp1: e1, operator: DAE::Operator::SUB { .. }, exp2: e2 } => {
                    let mut f1: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                    let mut f2: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                    let mut res: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                    let mut f2_1: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                    f1 = allTerms(metamodelica::AsArg::as_arg(&e1));
                    f2 = allTerms(metamodelica::AsArg::as_arg(&e2));
                    f2_1 = List::map(f2.clone(), &negate)?;
                    res = listAppend(f1.clone(), f2_1.clone());
                    Ok(res.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::BINARY { exp1: e1, operator: DAE::Operator::ADD_ARR { .. }, exp2: e2 } => {
                    let mut f1: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                    let mut f2: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                    let mut res: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                    f1 = allTerms(metamodelica::AsArg::as_arg(&e1));
                    f2 = allTerms(metamodelica::AsArg::as_arg(&e2));
                    res = listAppend(f1.clone(), f2.clone());
                    Ok(res.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::BINARY { exp1: e1, operator: DAE::Operator::SUB_ARR { .. }, exp2: e2 } => {
                    let mut f1: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                    let mut f2: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                    let mut res: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                    let mut f2_1: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                    f1 = allTerms(metamodelica::AsArg::as_arg(&e1));
                    f2 = allTerms(metamodelica::AsArg::as_arg(&e2));
                    f2_1 = List::map(f2.clone(), &negate)?;
                    res = listAppend(f1.clone(), f2_1.clone());
                    Ok(res.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::BINARY { exp1: e1, operator: DAE::Operator::MUL { ty: _ }, exp2: e2 } => {
                    let mut f1: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                    let __pa0 = ::match_deref::match_deref! { match &(allTerms(metamodelica::AsArg::as_arg(&e2))) {
                        __pa0 @ Deref @ metamodelica::ListNode::Cons { head: _, tail: Deref @ metamodelica::ListNode::Cons { head: _, tail: _ } } => __pa0.clone(),
                        _ => return Err("pattern mismatch"),
                    } };
                    f1 = metamodelica::Own::own(__pa0);
                    f1 = List::map1(f1.clone(), &makeProduct, e1.clone())?;
                    f1 = List::flatten(List::map(f1.clone(), &move |__a0: metamodelica::Ref<DAE::Exp>| -> metamodelica::Result<_> { ::std::result::Result::Ok(allTerms(&__a0)) })?)?;
                    Ok(f1.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::BINARY { exp1: e1, operator: DAE::Operator::MUL_ARR { ty: _ }, exp2: e2 } => {
                    let mut f1: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                    let __pa0 = ::match_deref::match_deref! { match &(allTerms(metamodelica::AsArg::as_arg(&e2))) {
                        __pa0 @ Deref @ metamodelica::ListNode::Cons { head: _, tail: Deref @ metamodelica::ListNode::Cons { head: _, tail: _ } } => __pa0.clone(),
                        _ => return Err("pattern mismatch"),
                    } };
                    f1 = metamodelica::Own::own(__pa0);
                    f1 = List::map1(f1.clone(), &makeProduct, e1.clone())?;
                    f1 = List::flatten(List::map(f1.clone(), &move |__a0: metamodelica::Ref<DAE::Exp>| -> metamodelica::Result<_> { ::std::result::Result::Ok(allTerms(&__a0)) })?)?;
                    Ok(f1.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::BINARY { exp1: e1, operator: DAE::Operator::MUL_ARRAY_SCALAR { ty: _ }, exp2: e2 } => {
                    let mut f1: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                    let __pa0 = ::match_deref::match_deref! { match &(allTerms(metamodelica::AsArg::as_arg(&e2))) {
                        __pa0 @ Deref @ metamodelica::ListNode::Cons { head: _, tail: Deref @ metamodelica::ListNode::Cons { head: _, tail: _ } } => __pa0.clone(),
                        _ => return Err("pattern mismatch"),
                    } };
                    f1 = metamodelica::Own::own(__pa0);
                    f1 = List::map1(f1.clone(), &makeProduct, e1.clone())?;
                    f1 = List::flatten(List::map(f1.clone(), &move |__a0: metamodelica::Ref<DAE::Exp>| -> metamodelica::Result<_> { ::std::result::Result::Ok(allTerms(&__a0)) })?)?;
                    Ok(f1.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::BINARY { exp1: e1, operator: DAE::Operator::MUL { ty: _ }, exp2: e2 } => {
                    let mut f1: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                    let __pa0 = ::match_deref::match_deref! { match &(allTerms(metamodelica::AsArg::as_arg(&e1))) {
                        __pa0 @ Deref @ metamodelica::ListNode::Cons { head: _, tail: Deref @ metamodelica::ListNode::Cons { head: _, tail: _ } } => __pa0.clone(),
                        _ => return Err("pattern mismatch"),
                    } };
                    f1 = metamodelica::Own::own(__pa0);
                    f1 = List::map1(f1.clone(), &makeProduct, e2.clone())?;
                    f1 = List::flatten(List::map(f1.clone(), &move |__a0: metamodelica::Ref<DAE::Exp>| -> metamodelica::Result<_> { ::std::result::Result::Ok(allTerms(&__a0)) })?)?;
                    Ok(f1.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::BINARY { exp1: e1, operator: DAE::Operator::MUL_ARR { ty: _ }, exp2: e2 } => {
                    let mut f1: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                    let __pa0 = ::match_deref::match_deref! { match &(allTerms(metamodelica::AsArg::as_arg(&e1))) {
                        __pa0 @ Deref @ metamodelica::ListNode::Cons { head: _, tail: Deref @ metamodelica::ListNode::Cons { head: _, tail: _ } } => __pa0.clone(),
                        _ => return Err("pattern mismatch"),
                    } };
                    f1 = metamodelica::Own::own(__pa0);
                    f1 = List::map1(f1.clone(), &makeProduct, e2.clone())?;
                    f1 = List::flatten(List::map(f1.clone(), &move |__a0: metamodelica::Ref<DAE::Exp>| -> metamodelica::Result<_> { ::std::result::Result::Ok(allTerms(&__a0)) })?)?;
                    Ok(f1.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::BINARY { exp1: e1, operator: DAE::Operator::MUL_ARRAY_SCALAR { ty: _ }, exp2: e2 } => {
                    let mut f1: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                    let __pa0 = ::match_deref::match_deref! { match &(allTerms(metamodelica::AsArg::as_arg(&e1))) {
                        __pa0 @ Deref @ metamodelica::ListNode::Cons { head: _, tail: Deref @ metamodelica::ListNode::Cons { head: _, tail: _ } } => __pa0.clone(),
                        _ => return Err("pattern mismatch"),
                    } };
                    f1 = metamodelica::Own::own(__pa0);
                    f1 = List::map1(f1.clone(), &makeProduct, e2.clone())?;
                    f1 = List::flatten(List::map(f1.clone(), &move |__a0: metamodelica::Ref<DAE::Exp>| -> metamodelica::Result<_> { ::std::result::Result::Ok(allTerms(&__a0)) })?)?;
                    Ok(f1.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::BINARY { exp1: e1, operator: DAE::Operator::DIV { ty: _ }, exp2: e2 } => {
                    let mut f1: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                    let __pa0 = ::match_deref::match_deref! { match &(allTerms(metamodelica::AsArg::as_arg(&e1))) {
                        __pa0 @ Deref @ metamodelica::ListNode::Cons { head: _, tail: Deref @ metamodelica::ListNode::Cons { head: _, tail: _ } } => __pa0.clone(),
                        _ => return Err("pattern mismatch"),
                    } };
                    f1 = metamodelica::Own::own(__pa0);
                    f1 = List::map1(f1.clone(), &expDiv, e2.clone())?;
                    f1 = List::flatten(List::map(f1.clone(), &move |__a0: metamodelica::Ref<DAE::Exp>| -> metamodelica::Result<_> { ::std::result::Result::Ok(allTerms(&__a0)) })?)?;
                    Ok(f1.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::BINARY { exp1: e1, operator: DAE::Operator::DIV_ARR { ty: _ }, exp2: e2 } => {
                    let mut f1: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                    let __pa0 = ::match_deref::match_deref! { match &(allTerms(metamodelica::AsArg::as_arg(&e1))) {
                        __pa0 @ Deref @ metamodelica::ListNode::Cons { head: _, tail: Deref @ metamodelica::ListNode::Cons { head: _, tail: _ } } => __pa0.clone(),
                        _ => return Err("pattern mismatch"),
                    } };
                    f1 = metamodelica::Own::own(__pa0);
                    f1 = List::map1(f1.clone(), &expDiv, e2.clone())?;
                    f1 = List::flatten(List::map(f1.clone(), &move |__a0: metamodelica::Ref<DAE::Exp>| -> metamodelica::Result<_> { ::std::result::Result::Ok(allTerms(&__a0)) })?)?;
                    Ok(f1.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::BINARY { exp1: e1, operator: DAE::Operator::DIV_ARRAY_SCALAR { ty: _ }, exp2: e2 } => {
                    let mut f1: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                    let __pa0 = ::match_deref::match_deref! { match &(allTerms(metamodelica::AsArg::as_arg(&e1))) {
                        __pa0 @ Deref @ metamodelica::ListNode::Cons { head: _, tail: Deref @ metamodelica::ListNode::Cons { head: _, tail: _ } } => __pa0.clone(),
                        _ => return Err("pattern mismatch"),
                    } };
                    f1 = metamodelica::Own::own(__pa0);
                    f1 = List::map1(f1.clone(), &expDiv, e2.clone())?;
                    f1 = List::flatten(List::map(f1.clone(), &move |__a0: metamodelica::Ref<DAE::Exp>| -> metamodelica::Result<_> { ::std::result::Result::Ok(allTerms(&__a0)) })?)?;
                    Ok(f1.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::BINARY { exp1: e1, operator: DAE::Operator::DIV_SCALAR_ARRAY { ty: _ }, exp2: e2 } => {
                    let mut f1: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                    let __pa0 = ::match_deref::match_deref! { match &(allTerms(metamodelica::AsArg::as_arg(&e1))) {
                        __pa0 @ Deref @ metamodelica::ListNode::Cons { head: _, tail: Deref @ metamodelica::ListNode::Cons { head: _, tail: _ } } => __pa0.clone(),
                        _ => return Err("pattern mismatch"),
                    } };
                    f1 = metamodelica::Own::own(__pa0);
                    f1 = List::map1(f1.clone(), &expDiv, e2.clone())?;
                    f1 = List::flatten(List::map(f1.clone(), &move |__a0: metamodelica::Ref<DAE::Exp>| -> metamodelica::Result<_> { ::std::result::Result::Ok(allTerms(&__a0)) })?)?;
                    Ok(f1.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::UNARY { operator: DAE::Operator::UMINUS { .. }, exp: e1 } => {
                    let mut f1: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                    f1 = allTerms(metamodelica::AsArg::as_arg(&e1));
                    f1 = List::map(f1.clone(), &negate)?;
                    Ok(f1.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::UNARY { operator: DAE::Operator::UMINUS_ARR { .. }, exp: e1 } => {
                    let mut f1: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                    f1 = allTerms(metamodelica::AsArg::as_arg(&e1));
                    f1 = List::map(f1.clone(), &negate)?;
                    Ok(f1.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::LUNARY { operator: DAE::Operator::NOT { .. }, exp: e1 } => {
                    let mut f1: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                    f1 = allTerms(metamodelica::AsArg::as_arg(&e1));
                    f1 = List::map(f1.clone(), &negate)?;
                    Ok(f1.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                        Deref @ DAE::Exp::ASUB { exp: e1, sub: subs } => {
                            let mut f1: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                            let mut f2: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                            f2 = ({
                let mut __acc: metamodelica::List<metamodelica::Ref<DAE::Exp>> = metamodelica::nil();
                for mut sub in (subs.clone()).into_iter().cloned() {
                            let __x = getSubscriptExp(&(sub.clone()))?;
                            __acc = cons(__x, __acc);
                }
                __acc.reverse()
            });
                            f1 = allTerms(metamodelica::AsArg::as_arg(&e1));
                            f1 = List::map1(f1.clone(), &makeASUB, f2.clone())?;
                            Ok(f1.clone())
                        }
                        _ => return Err("nomatch"),
                    }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(list![inExp.clone()])
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    outExpLst
}

pub fn allTermsForCref(
    mut inExp: &metamodelica::Ref<DAE::Exp>,
    mut cr: &metamodelica::Ref<DAE::ComponentRef>,
    mut inFunc: &dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>, metamodelica::Ref<DAE::ComponentRef>) -> Result<bool>,
) -> Result<(
    metamodelica::List<metamodelica::Ref<DAE::Exp>>,
    metamodelica::List<metamodelica::Ref<DAE::Exp>>,
)> {
    pub type MapFunc = std::sync::Arc<
        dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>, metamodelica::Ref<DAE::ComponentRef>) -> Result<bool> + 'static,
    >;

    let mut outExpLstWithX: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
    let mut outExpLstWithoutX: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
    (outExpLstWithX, outExpLstWithoutX) = 'mc: {
        let __mc_input = &**inExp;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::BINARY { exp1: e1, operator: DAE::Operator::ADD { .. }, exp2: e2 } => {
                    let mut f1: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                    let mut f2: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                    let mut fx1: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                    let mut fx2: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                    let mut res: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                    let mut resx: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                    (fx1, f1) = allTermsForCref(metamodelica::AsArg::as_arg(&e1), cr, inFunc)?;
                    (fx2, f2) = allTermsForCref(metamodelica::AsArg::as_arg(&e2), cr, inFunc)?;
                    res = listAppend(f1.clone(), f2.clone());
                    resx = listAppend(fx1.clone(), fx2.clone());
                    Ok((resx.clone(), res.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::BINARY { exp1: e1, operator: DAE::Operator::SUB { .. }, exp2: e2 } => {
                    let mut f1: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                    let mut f2: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                    let mut fx1: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                    let mut fx2: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                    let mut res: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                    let mut resx: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                    (fx1, f1) = allTermsForCref(metamodelica::AsArg::as_arg(&e1), cr, inFunc)?;
                    (fx2, f2) = allTermsForCref(metamodelica::AsArg::as_arg(&e2), cr, inFunc)?;
                    f2 = List::map(f2.clone(), &negate)?;
                    fx2 = List::map(fx2.clone(), &negate)?;
                    res = listAppend(f1.clone(), f2.clone());
                    resx = listAppend(fx1.clone(), fx2.clone());
                    Ok((resx.clone(), res.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::BINARY { exp1: e1, operator: DAE::Operator::ADD_ARR { .. }, exp2: e2 } => {
                    let mut f1: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                    let mut f2: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                    let mut fx1: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                    let mut fx2: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                    let mut res: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                    let mut resx: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                    (fx1, f1) = allTermsForCref(metamodelica::AsArg::as_arg(&e1), cr, inFunc)?;
                    (fx2, f2) = allTermsForCref(metamodelica::AsArg::as_arg(&e2), cr, inFunc)?;
                    res = listAppend(f1.clone(), f2.clone());
                    resx = listAppend(fx1.clone(), fx2.clone());
                    Ok((resx.clone(), res.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::BINARY { exp1: e1, operator: DAE::Operator::SUB_ARR { .. }, exp2: e2 } => {
                    let mut f1: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                    let mut f2: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                    let mut fx1: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                    let mut fx2: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                    let mut res: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                    let mut resx: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                    (fx1, f1) = allTermsForCref(metamodelica::AsArg::as_arg(&e1), cr, inFunc)?;
                    (fx2, f2) = allTermsForCref(metamodelica::AsArg::as_arg(&e2), cr, inFunc)?;
                    f2 = List::map(f2.clone(), &negate)?;
                    fx2 = List::map(fx2.clone(), &negate)?;
                    res = listAppend(f1.clone(), f2.clone());
                    resx = listAppend(fx1.clone(), fx2.clone());
                    Ok((resx.clone(), res.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::BINARY { exp1: e1, operator: DAE::Operator::MUL { ty: _ }, exp2: e2 } => {
                    if !((inFunc(e2.clone(), cr.clone())?)) { return Err("guard") }
                    let mut f1: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                    let mut f2: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                    let mut fx1: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                    let mut res: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                    let mut e: metamodelica::Ref<DAE::Exp>;
                    (fx1, f1) = allTermsForCref(metamodelica::AsArg::as_arg(&e2), cr, inFunc)?;
                    (fx1, f2) = List::split1OnTrue(&fx1, inFunc, cr.clone())?;
                    res = listAppend(f1.clone(), f2.clone());
                    e = makeSum1(res.clone(), false)?;
                    e = expMul(e.clone(), e1.clone())?;
                    fx1 = List::map1(fx1.clone(), &expMul, e1.clone())?;
                    if !(isZero(&e)?) {
                        if expHasCrefNoPreOrStart(e1.clone(), cr.clone())? {
                            fx1 = metamodelica::cons(e.clone(), fx1.clone());
                            f1 = metamodelica::nil();
                        } else {
                            f1 = list![e.clone()];
                        }
                    }
                    Ok((fx1.clone(), f1.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::BINARY { exp1: e1, operator: DAE::Operator::MUL { ty: _ }, exp2: e2 } => {
                    if !((inFunc(e1.clone(), cr.clone())?)) { return Err("guard") }
                    let mut f1: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                    let mut f2: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                    let mut fx1: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                    let mut res: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                    let mut e: metamodelica::Ref<DAE::Exp>;
                    (fx1, f1) = allTermsForCref(metamodelica::AsArg::as_arg(&e1), cr, inFunc)?;
                    (fx1, f2) = List::split1OnTrue(&fx1, inFunc, cr.clone())?;
                    res = listAppend(f1.clone(), f2.clone());
                    e = makeSum1(res.clone(), false)?;
                    e = expMul(e.clone(), e2.clone())?;
                    fx1 = List::map1(fx1.clone(), &expMul, e2.clone())?;
                    if !(isZero(&e)?) {
                        if expHasCrefNoPreOrStart(e1.clone(), cr.clone())? {
                            fx1 = metamodelica::cons(e.clone(), fx1.clone());
                            f1 = metamodelica::nil();
                        } else {
                            f1 = list![e.clone()];
                        }
                    }
                    Ok((fx1.clone(), f1.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::BINARY { exp1: e1, operator: DAE::Operator::DIV { ty: _ }, exp2: e2 } => {
                    if !((inFunc(e1.clone(), cr.clone())?)) { return Err("guard") }
                    let mut f1: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                    let mut f2: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                    let mut fx1: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                    let mut res: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                    let mut e: metamodelica::Ref<DAE::Exp>;
                    (fx1, f1) = allTermsForCref(metamodelica::AsArg::as_arg(&e1), cr, inFunc)?;
                    (fx1, f2) = List::split1OnTrue(&fx1, inFunc, cr.clone())?;
                    res = listAppend(f1.clone(), f2.clone());
                    e = makeSum1(res.clone(), false)?;
                    e = makeDiv(e.clone(), e2.clone())?;
                    fx1 = List::map1(fx1.clone(), &makeDiv, e2.clone())?;
                    if !(isZero(&e)?) {
                        if expHasCrefNoPreOrStart(e1.clone(), cr.clone())? {
                            fx1 = metamodelica::cons(e.clone(), fx1.clone());
                            f1 = metamodelica::nil();
                        } else {
                            f1 = list![e.clone()];
                        }
                    }
                    Ok((fx1.clone(), f1.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::UNARY { operator: DAE::Operator::UMINUS { .. }, exp: e1 } => {
                    let mut f1: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                    let mut fx1: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                    (fx1, f1) = allTermsForCref(metamodelica::AsArg::as_arg(&e1), cr, inFunc)?;
                    f1 = List::map(f1.clone(), &negate)?;
                    fx1 = List::map(fx1.clone(), &negate)?;
                    Ok((fx1.clone(), f1.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut res: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                    let mut resx: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                    if inFunc(inExp.clone(), cr.clone())? {
                        res = metamodelica::nil();
                        resx = list![inExp.clone()];
                    } else {
                        resx = metamodelica::nil();
                        res = list![inExp.clone()];
                    }
                    Ok((resx.clone(), res.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((outExpLstWithX, outExpLstWithoutX))
}

pub(crate) fn termsExpandUnary(
    mut inExp: metamodelica::Ref<DAE::Exp>,
) -> Result<metamodelica::List<metamodelica::Ref<DAE::Exp>>> {
    let mut outExpLst: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
    outExpLst = (match &*inExp {
        DAE::Exp::UNARY {
            operator: DAE::Operator::UMINUS { .. },
            exp: e,
        } => List::map(terms(e.clone())?, &negate)?,
        _ => terms(inExp)?,
    });
    Ok(outExpLst)
}

pub fn terms(mut inExp: metamodelica::Ref<DAE::Exp>) -> Result<metamodelica::List<metamodelica::Ref<DAE::Exp>>> {
    let mut outExpLst: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
    outExpLst = terms2(inExp, metamodelica::nil(), false)?;
    Ok(outExpLst)
}

fn terms2(
    mut inExp: metamodelica::Ref<DAE::Exp>,
    mut inAcc: metamodelica::List<metamodelica::Ref<DAE::Exp>>,
    mut neg: bool,
) -> Result<metamodelica::List<metamodelica::Ref<DAE::Exp>>> {
    '__tco: loop {
        ::match_deref::match_deref! { match &((inExp, inAcc, neg)) {
            (Deref @ DAE::Exp::BINARY { exp1: e1, operator: DAE::Operator::ADD { .. }, exp2: e2 }, acc, _) => {
                let mut acc = (*acc).clone();
                acc = terms2(e2.clone(), acc.clone(), neg)?;
                { (inExp, inAcc, neg) = (e1.clone(), acc.clone(), neg); continue '__tco; }
            },
            (Deref @ DAE::Exp::BINARY { exp1: e1, operator: DAE::Operator::SUB { .. }, exp2: e2 }, acc, _) => {
                let mut acc = (*acc).clone();
                acc = terms2(e2.clone(), acc.clone(), !(neg))?;
                { (inExp, inAcc, neg) = (e1.clone(), acc.clone(), neg); continue '__tco; }
            },
            (e, acc, true) => {
                let mut e = (*e).clone();
                e = negate(e.clone())?;
                return Ok(metamodelica::cons(e.clone(), acc.clone()))
            },
            (e, acc, _) => {
                return Ok(metamodelica::cons(e.clone(), acc.clone()))
            },
            _ => return Err("match: no arm matched"),
        } }
    }
}

pub(crate) fn quotient(
    mut inExp: &metamodelica::Ref<DAE::Exp>,
) -> Result<(metamodelica::Ref<DAE::Exp>, metamodelica::Ref<DAE::Exp>)> {
    let mut num: metamodelica::Ref<DAE::Exp>;
    let mut denom: metamodelica::Ref<DAE::Exp>;
    (num, denom) = 'mc: {
        let __mc_input = &**inExp;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::BINARY { exp1: e1, operator: DAE::Operator::DIV { .. }, exp2: e2 } => {
                    Ok((e1.clone(), e2.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::BINARY { exp1: e1, operator: DAE::Operator::MUL { .. }, exp2: e2 } => {
                    let mut p: metamodelica::Ref<DAE::Exp>;
                    let mut q: metamodelica::Ref<DAE::Exp>;
                    let mut tp: Type;
                    (p, q) = quotient(metamodelica::AsArg::as_arg(&e1))?;
                    tp = r#typeof(p.clone())?;
                    Ok((metamodelica::Ref::new(DAE::Exp::BINARY { exp1: e2.clone(), operator: DAE::Operator::MUL { ty: tp.clone() }, exp2: p.clone() }), q.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::BINARY { exp1: e1, operator: DAE::Operator::MUL { .. }, exp2: e2 } => {
                    let mut p: metamodelica::Ref<DAE::Exp>;
                    let mut q: metamodelica::Ref<DAE::Exp>;
                    let mut tp: Type;
                    (p, q) = quotient(metamodelica::AsArg::as_arg(&e2))?;
                    tp = r#typeof(p.clone())?;
                    Ok((metamodelica::Ref::new(DAE::Exp::BINARY { exp1: e1.clone(), operator: DAE::Operator::MUL { ty: tp.clone() }, exp2: p.clone() }), q.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((num, denom))
}

pub fn factors(mut inExp: &metamodelica::Ref<DAE::Exp>) -> Result<metamodelica::List<metamodelica::Ref<DAE::Exp>>> {
    let mut outExpLst: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
    outExpLst = factorsWork(inExp, metamodelica::nil(), false)?.reverse();
    Ok(outExpLst)
}

fn factorsWork<'__b>(
    mut inExp: &'__b metamodelica::Ref<DAE::Exp>,
    mut acc: metamodelica::List<metamodelica::Ref<DAE::Exp>>,
    mut doInverseFactors: bool,
) -> Result<metamodelica::List<metamodelica::Ref<DAE::Exp>>> {
    '__tco: loop {
        ::match_deref::match_deref! { match inExp {
            Deref @ DAE::Exp::BINARY { exp1: e1, operator: DAE::Operator::MUL { .. }, exp2: e2 } => {
                acc = factorsWork(e1, acc, doInverseFactors)?;
                { (inExp, acc, doInverseFactors) = (e2, acc, doInverseFactors); continue '__tco; }
            },
            Deref @ DAE::Exp::BINARY { exp1: e1, operator: DAE::Operator::DIV { ty: Deref @ DAE::Type::T_REAL { .. } }, exp2: e2 } => {
                acc = factorsWork(e1, acc, doInverseFactors)?;
                { (inExp, acc, doInverseFactors) = (e2, acc, !(doInverseFactors)); continue '__tco; }
            },
            Deref @ DAE::Exp::ICONST { integer: 1 } => {
                return Ok(acc)
            },
            Deref @ DAE::Exp::RCONST { real: __rlit_0 } if __rlit_0.eq(&metamodelica::OrderedFloat((1.0) as f64)) => {
                return Ok(acc)
            },
            _ => {
                return Ok(metamodelica::cons(if (doInverseFactors) {inverseFactors(inExp.clone())?} else {inExp.clone()}, acc))
            },
            _ => return Err("match: no arm matched"),
        } }
    }
}

pub fn inverseFactors(mut inExp: metamodelica::Ref<DAE::Exp>) -> Result<metamodelica::Ref<DAE::Exp>> {
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    outExp = 'mc: {
        let __mc_input = inExp;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::BINARY { exp1: e1, operator: DAE::Operator::POW { ty: tp }, exp2: e2 } => {
                    let mut tp2: Type;
                    tp2 = r#typeof(e2.clone())?;
                    Ok(metamodelica::Ref::new(DAE::Exp::BINARY { exp1: e1.clone(), operator: DAE::Operator::POW { ty: tp.clone() }, exp2: metamodelica::Ref::new(DAE::Exp::UNARY { operator: DAE::Operator::UMINUS { ty: tp2.clone() }, exp: e2.clone() }) }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::BINARY { exp1: e1, operator: op @ DAE::Operator::DIV { .. }, exp2: e2 } => {
                    let false = (isZero(metamodelica::AsArg::as_arg(&e1))?) else { return Err("pattern mismatch") };
                    Ok(metamodelica::Ref::new(DAE::Exp::BINARY { exp1: e2.clone(), operator: op.clone(), exp2: e1.clone() }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                        e => {
                            let mut tp: Type;
                            let mut e = (*e).clone();
                            let false = (isZero(metamodelica::AsArg::as_arg(&e))?) else { return Err("pattern mismatch") };
                            tp = r#typeof(e.clone())?;
                            e = (match &*tp {
                DAE::Type::T_REAL { .. } => metamodelica::Ref::new(DAE::Exp::BINARY { exp1: metamodelica::Ref::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(1.0_f64) }), operator: DAE::Operator::DIV { ty: DAE::T_REAL_DEFAULT().clone() }, exp2: e.clone() }),
                DAE::Type::T_INTEGER { .. } => metamodelica::Ref::new(DAE::Exp::BINARY { exp1: metamodelica::Ref::new(DAE::Exp::ICONST { integer: 1 }), operator: DAE::Operator::DIV { ty: DAE::T_INTEGER_DEFAULT().clone() }, exp2: e.clone() }),
                _ => return Err("match: no arm matched"),
            });
                            Ok(e.clone())
                        }
                        _ => return Err("nomatch"),
                    }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(outExp)
}

pub fn expandFactors(
    mut inExp: &metamodelica::Ref<DAE::Exp>,
) -> Result<metamodelica::List<metamodelica::Ref<DAE::Exp>>> {
    let mut outExpLst: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
    outExpLst = expandFactorsWork(inExp, metamodelica::nil(), false)?.reverse();
    Ok(outExpLst)
}

fn expandFactorsWork(
    mut inExp: &metamodelica::Ref<DAE::Exp>,
    mut acc: metamodelica::List<metamodelica::Ref<DAE::Exp>>,
    mut doInverseFactors: bool,
) -> Result<metamodelica::List<metamodelica::Ref<DAE::Exp>>> {
    let mut acc: metamodelica::List<metamodelica::Ref<DAE::Exp>> = acc;
    acc = (::match_deref::match_deref! { match inExp {
        Deref @ DAE::Exp::BINARY { exp1: Deref @ DAE::Exp::BINARY { exp1: e1, operator: DAE::Operator::MUL { .. }, exp2: e2 }, operator: DAE::Operator::POW { .. }, exp2: e3 } => {
            let mut pow_acc: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
            let mut pow_acc2: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
            pow_acc = expandFactorsWork(metamodelica::AsArg::as_arg(&e1), metamodelica::nil(), doInverseFactors)?;
            pow_acc = expPowLst(pow_acc, e3.clone())?;
            pow_acc2 = expandFactorsWork(metamodelica::AsArg::as_arg(&e2), metamodelica::nil(), doInverseFactors)?;
            pow_acc2 = expPowLst(pow_acc2, e3.clone())?;
            acc = listAppend(pow_acc, acc);
            acc = listAppend(pow_acc2, acc);
            acc
        },
        Deref @ DAE::Exp::BINARY { exp1: Deref @ DAE::Exp::BINARY { exp1: e1, operator: DAE::Operator::DIV { .. }, exp2: e2 }, operator: DAE::Operator::POW { .. }, exp2: e3 } => {
            let mut pow_acc: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
            let mut pow_acc2: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
            pow_acc = expandFactorsWork(metamodelica::AsArg::as_arg(&e1), metamodelica::nil(), doInverseFactors)?;
            pow_acc = expPowLst(pow_acc, e3.clone())?;
            pow_acc2 = expandFactorsWork(metamodelica::AsArg::as_arg(&e2), metamodelica::nil(), doInverseFactors)?;
            pow_acc2 = expPowLst(pow_acc2, negate(e3.clone())?)?;
            acc = listAppend(pow_acc, acc);
            acc = listAppend(pow_acc2, acc);
            acc
        },
        Deref @ DAE::Exp::BINARY { exp1: Deref @ DAE::Exp::BINARY { exp1: e1, operator: DAE::Operator::POW { .. }, exp2: e2 }, operator: DAE::Operator::POW { .. }, exp2: e3 } => {
            let mut e: metamodelica::Ref<DAE::Exp>;
            let mut pow_acc: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
            e = expMul(e2.clone(), e3.clone())?;
            pow_acc = expandFactorsWork(metamodelica::AsArg::as_arg(&e1), metamodelica::nil(), doInverseFactors)?;
            pow_acc = expPowLst(pow_acc, e)?;
            acc = listAppend(pow_acc, acc);
            acc
        },
        Deref @ DAE::Exp::BINARY { exp1: e1, operator: op @ DAE::Operator::DIV { ty: tp }, exp2: e2 } if (isZero(e2)?) => {
            let mut e: metamodelica::Ref<DAE::Exp>;
            if doInverseFactors {
                e = e2.clone();
            } else {
                e = metamodelica::Ref::new(DAE::Exp::BINARY { exp1: makeConstOne(metamodelica::AsArg::as_arg(&tp)), operator: op.clone(), exp2: e2.clone() });
            }
            acc = expandFactorsWork(e1, acc, doInverseFactors)?;
            metamodelica::cons(e, acc)
        },
        Deref @ DAE::Exp::UNARY { operator: DAE::Operator::UMINUS { ty: tp }, exp: e1 } => {
            let mut e: metamodelica::Ref<DAE::Exp>;
            e = makeConstOne(metamodelica::AsArg::as_arg(&tp));
            acc = expandFactorsWork(e1, acc, doInverseFactors)?;
            e = negate(e)?;
            metamodelica::cons(e, acc)
        },
        Deref @ DAE::Exp::UNARY { operator: DAE::Operator::UMINUS_ARR { ty: tp }, exp: e1 } => {
            let mut e: metamodelica::Ref<DAE::Exp>;
            e = makeConstOne(metamodelica::AsArg::as_arg(&tp));
            acc = expandFactorsWork(e1, acc, doInverseFactors)?;
            e = negate(e)?;
            metamodelica::cons(e, acc)
        },
        _ => {
            acc = expandFactorsWork3(inExp.clone(), acc, doInverseFactors)?;
            expandFactorsWork2(&acc, doInverseFactors)?
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(acc)
}

fn expandFactorsWork3(
    mut inExp: metamodelica::Ref<DAE::Exp>,
    mut acc: metamodelica::List<metamodelica::Ref<DAE::Exp>>,
    mut doInverseFactors: bool,
) -> Result<metamodelica::List<metamodelica::Ref<DAE::Exp>>> {
    let mut acc: metamodelica::List<metamodelica::Ref<DAE::Exp>> = acc;
    let mut e1: metamodelica::Ref<DAE::Exp>;
    let mut e2: metamodelica::Ref<DAE::Exp>;
    let mut e: metamodelica::Ref<DAE::Exp>;
    let mut op: Operator;
    acc = 'mc: {
        let __mc_input = &*inExp;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(factorsWork(&inExp, acc.clone(), doInverseFactors)?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::BINARY { exp1: e1, operator: DAE::Operator::MUL { .. }, exp2: e2 } => {
                    let mut acc: metamodelica::List<metamodelica::Ref<DAE::Exp>> = acc.clone();
                    acc = expandFactorsWork(metamodelica::AsArg::as_arg(&e1), acc.clone(), doInverseFactors)?;
                    acc = expandFactorsWork(metamodelica::AsArg::as_arg(&e2), acc.clone(), doInverseFactors)?;
                    Ok((acc.clone(), acc.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            acc = __wb0;
            break 'mc __v;
        }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::BINARY { exp1: Deref @ DAE::Exp::BINARY { exp1: e, operator: op @ DAE::Operator::DIV { .. }, exp2: e1 }, operator: DAE::Operator::DIV { .. }, exp2: e2 } => {
                    let mut e = (*e).clone();
                    let mut acc: metamodelica::List<metamodelica::Ref<DAE::Exp>> = acc.clone();
                    e = metamodelica::Ref::new(DAE::Exp::BINARY { exp1: e.clone(), operator: op.clone(), exp2: expMul(e1.clone(), e2.clone())? });
                    acc = expandFactorsWork(metamodelica::AsArg::as_arg(&e), acc.clone(), doInverseFactors)?;
                    Ok((acc.clone(), acc.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            acc = __wb0;
            break 'mc __v;
        }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::BINARY { exp1: Deref @ DAE::Exp::BINARY { exp1: e, operator: DAE::Operator::MUL { .. }, exp2: e1 }, operator: op @ DAE::Operator::DIV { .. }, exp2: e2 } => {
                    let mut acc: metamodelica::List<metamodelica::Ref<DAE::Exp>> = acc.clone();
                    acc = expandFactorsWork(metamodelica::AsArg::as_arg(&e), acc.clone(), doInverseFactors)?;
                    acc = expandFactorsWork(&(metamodelica::Ref::new(DAE::Exp::BINARY { exp1: e1.clone(), operator: op.clone(), exp2: e2.clone() })), acc.clone(), doInverseFactors)?;
                    Ok((acc.clone(), acc.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            acc = __wb0;
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(metamodelica::cons(inExp.clone(), acc.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(acc)
}

fn expandFactorsWork2(
    mut inAcc: &metamodelica::List<metamodelica::Ref<DAE::Exp>>,
    mut doInverseFactors: bool,
) -> Result<metamodelica::List<metamodelica::Ref<DAE::Exp>>> {
    let mut outExpLst: metamodelica::List<metamodelica::Ref<DAE::Exp>> = metamodelica::nil();
    let mut tmpExpLst: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
    for mut elem in &**inAcc {
        tmpExpLst = (::match_deref::match_deref! { match &(elem.clone()) {
            Deref @ DAE::Exp::BINARY { exp1: Deref @ DAE::Exp::BINARY { exp1: _, operator: DAE::Operator::DIV { .. }, exp2: _ }, operator: DAE::Operator::POW { .. }, exp2: _ } => expandFactorsWork(metamodelica::AsArg::as_arg(&elem), metamodelica::nil(), doInverseFactors)?,
            Deref @ DAE::Exp::BINARY { exp1: Deref @ DAE::Exp::BINARY { exp1: _, operator: DAE::Operator::MUL { .. }, exp2: _ }, operator: DAE::Operator::POW { .. }, exp2: _ } => expandFactorsWork(metamodelica::AsArg::as_arg(&elem), metamodelica::nil(), doInverseFactors)?,
            Deref @ DAE::Exp::BINARY { exp1: Deref @ DAE::Exp::BINARY { exp1: _, operator: DAE::Operator::POW { .. }, exp2: _ }, operator: DAE::Operator::POW { .. }, exp2: _ } => expandFactorsWork(metamodelica::AsArg::as_arg(&elem), metamodelica::nil(), doInverseFactors)?,
            Deref @ DAE::Exp::UNARY { operator: DAE::Operator::UMINUS { .. }, exp: _ } => expandFactorsWork(metamodelica::AsArg::as_arg(&elem), metamodelica::nil(), doInverseFactors)?,
            Deref @ DAE::Exp::UNARY { operator: DAE::Operator::UMINUS_ARR { .. }, exp: _ } => expandFactorsWork(metamodelica::AsArg::as_arg(&elem), metamodelica::nil(), doInverseFactors)?,
            _ => list![elem.clone()],
            _ => unreachable!("match_deref! exhaustiveness placeholder"),
        } });
        outExpLst = listAppend(tmpExpLst, outExpLst);
    }
    Ok(outExpLst)
}

pub(crate) fn getTermsContainingX(
    mut inExp1: metamodelica::Ref<DAE::Exp>,
    mut inExp2: metamodelica::Ref<DAE::Exp>,
) -> Result<(metamodelica::Ref<DAE::Exp>, metamodelica::Ref<DAE::Exp>)> {
    let mut outExp1: metamodelica::Ref<DAE::Exp>;
    let mut outExp2: metamodelica::Ref<DAE::Exp>;
    (outExp1, outExp2) = 'mc: {
        let __mc_input = (inExp1, inExp2);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::BINARY { exp1: e1, operator: DAE::Operator::ADD { ty }, exp2: e2 }, cr @ Deref @ DAE::Exp::CREF { .. }) => {
                    let mut xt1: metamodelica::Ref<DAE::Exp>;
                    let mut nonxt1: metamodelica::Ref<DAE::Exp>;
                    let mut xt2: metamodelica::Ref<DAE::Exp>;
                    let mut nonxt2: metamodelica::Ref<DAE::Exp>;
                    let mut xt: metamodelica::Ref<DAE::Exp>;
                    let mut nonxt: metamodelica::Ref<DAE::Exp>;
                    (xt1, nonxt1) = getTermsContainingX(e1.clone(), cr.clone())?;
                    (xt2, nonxt2) = getTermsContainingX(e2.clone(), cr.clone())?;
                    xt = metamodelica::Ref::new(DAE::Exp::BINARY { exp1: xt1.clone(), operator: DAE::Operator::ADD { ty: ty.clone() }, exp2: xt2.clone() });
                    nonxt = metamodelica::Ref::new(DAE::Exp::BINARY { exp1: nonxt1.clone(), operator: DAE::Operator::ADD { ty: ty.clone() }, exp2: nonxt2.clone() });
                    Ok((xt.clone(), nonxt.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::BINARY { exp1: e1, operator: DAE::Operator::SUB { ty }, exp2: e2 }, cr @ Deref @ DAE::Exp::CREF { .. }) => {
                    let mut xt1: metamodelica::Ref<DAE::Exp>;
                    let mut nonxt1: metamodelica::Ref<DAE::Exp>;
                    let mut xt2: metamodelica::Ref<DAE::Exp>;
                    let mut nonxt2: metamodelica::Ref<DAE::Exp>;
                    let mut xt: metamodelica::Ref<DAE::Exp>;
                    let mut nonxt: metamodelica::Ref<DAE::Exp>;
                    (xt1, nonxt1) = getTermsContainingX(e1.clone(), cr.clone())?;
                    (xt2, nonxt2) = getTermsContainingX(e2.clone(), cr.clone())?;
                    xt = metamodelica::Ref::new(DAE::Exp::BINARY { exp1: xt1.clone(), operator: DAE::Operator::SUB { ty: ty.clone() }, exp2: xt2.clone() });
                    nonxt = metamodelica::Ref::new(DAE::Exp::BINARY { exp1: nonxt1.clone(), operator: DAE::Operator::SUB { ty: ty.clone() }, exp2: nonxt2.clone() });
                    Ok((xt.clone(), nonxt.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::UNARY { operator: DAE::Operator::UMINUS { ty }, exp: e }, cr @ Deref @ DAE::Exp::CREF { .. }) => {
                    let mut xt1: metamodelica::Ref<DAE::Exp>;
                    let mut nonxt1: metamodelica::Ref<DAE::Exp>;
                    let mut xt: metamodelica::Ref<DAE::Exp>;
                    let mut nonxt: metamodelica::Ref<DAE::Exp>;
                    (xt1, nonxt1) = getTermsContainingX(e.clone(), cr.clone())?;
                    xt = metamodelica::Ref::new(DAE::Exp::UNARY { operator: DAE::Operator::UMINUS { ty: ty.clone() }, exp: xt1.clone() });
                    nonxt = metamodelica::Ref::new(DAE::Exp::UNARY { operator: DAE::Operator::UMINUS { ty: ty.clone() }, exp: nonxt1.clone() });
                    Ok((xt.clone(), nonxt.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::BINARY { exp1: e1, operator: DAE::Operator::ADD_ARR { ty }, exp2: e2 }, cr @ Deref @ DAE::Exp::CREF { .. }) => {
                    let mut xt1: metamodelica::Ref<DAE::Exp>;
                    let mut nonxt1: metamodelica::Ref<DAE::Exp>;
                    let mut xt2: metamodelica::Ref<DAE::Exp>;
                    let mut nonxt2: metamodelica::Ref<DAE::Exp>;
                    let mut xt: metamodelica::Ref<DAE::Exp>;
                    let mut nonxt: metamodelica::Ref<DAE::Exp>;
                    (xt1, nonxt1) = getTermsContainingX(e1.clone(), cr.clone())?;
                    (xt2, nonxt2) = getTermsContainingX(e2.clone(), cr.clone())?;
                    xt = metamodelica::Ref::new(DAE::Exp::BINARY { exp1: xt1.clone(), operator: DAE::Operator::ADD_ARR { ty: ty.clone() }, exp2: xt2.clone() });
                    nonxt = metamodelica::Ref::new(DAE::Exp::BINARY { exp1: nonxt1.clone(), operator: DAE::Operator::ADD_ARR { ty: ty.clone() }, exp2: nonxt2.clone() });
                    Ok((xt.clone(), nonxt.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::BINARY { exp1: e1, operator: DAE::Operator::SUB_ARR { ty }, exp2: e2 }, cr @ Deref @ DAE::Exp::CREF { .. }) => {
                    let mut xt1: metamodelica::Ref<DAE::Exp>;
                    let mut nonxt1: metamodelica::Ref<DAE::Exp>;
                    let mut xt2: metamodelica::Ref<DAE::Exp>;
                    let mut nonxt2: metamodelica::Ref<DAE::Exp>;
                    let mut xt: metamodelica::Ref<DAE::Exp>;
                    let mut nonxt: metamodelica::Ref<DAE::Exp>;
                    (xt1, nonxt1) = getTermsContainingX(e1.clone(), cr.clone())?;
                    (xt2, nonxt2) = getTermsContainingX(e2.clone(), cr.clone())?;
                    xt = metamodelica::Ref::new(DAE::Exp::BINARY { exp1: xt1.clone(), operator: DAE::Operator::SUB_ARR { ty: ty.clone() }, exp2: xt2.clone() });
                    nonxt = metamodelica::Ref::new(DAE::Exp::BINARY { exp1: nonxt1.clone(), operator: DAE::Operator::SUB_ARR { ty: ty.clone() }, exp2: nonxt2.clone() });
                    Ok((xt.clone(), nonxt.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::UNARY { operator: DAE::Operator::UMINUS_ARR { ty }, exp: e }, cr @ Deref @ DAE::Exp::CREF { .. }) => {
                    let mut xt1: metamodelica::Ref<DAE::Exp>;
                    let mut nonxt1: metamodelica::Ref<DAE::Exp>;
                    let mut xt: metamodelica::Ref<DAE::Exp>;
                    let mut nonxt: metamodelica::Ref<DAE::Exp>;
                    (xt1, nonxt1) = getTermsContainingX(e.clone(), cr.clone())?;
                    xt = metamodelica::Ref::new(DAE::Exp::UNARY { operator: DAE::Operator::UMINUS_ARR { ty: ty.clone() }, exp: xt1.clone() });
                    nonxt = metamodelica::Ref::new(DAE::Exp::UNARY { operator: DAE::Operator::UMINUS_ARR { ty: ty.clone() }, exp: nonxt1.clone() });
                    Ok((xt.clone(), nonxt.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e, cr @ Deref @ DAE::Exp::CREF { ty, .. }) => {
                    let mut xt: metamodelica::Ref<DAE::Exp>;
                    let mut nonxt: metamodelica::Ref<DAE::Exp>;
                    let mut zero: metamodelica::Ref<DAE::Exp>;
                    let mut res: bool;
                    res = expContains(metamodelica::AsArg::as_arg(&e), metamodelica::AsArg::as_arg(&cr))?;
                    (zero, _) = makeZeroExpression(&(arrayDimension(metamodelica::AsArg::as_arg(&ty))))?;
                    xt = if (res) {e.clone()} else {zero.clone()};
                    nonxt = if (res) {zero.clone()} else {e.clone()};
                    Ok((xt.clone(), nonxt.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((outExp1, outExp2))
}

pub fn flattenArrayExpToList(mut e: metamodelica::Ref<DAE::Exp>) -> metamodelica::List<metamodelica::Ref<DAE::Exp>> {
    let mut expLst: metamodelica::List<metamodelica::Ref<DAE::Exp>> = metamodelica::nil();
    expLst = 'mc: {
        let __mc_input = &*e;
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::UNARY { operator: DAE::Operator::UMINUS_ARR { .. }, exp: Deref @ DAE::Exp::ARRAY { array: expl, .. } } => {
                    let mut expl = (*expl).clone();
                    let mut expLst: metamodelica::List<metamodelica::Ref<DAE::Exp>> = expLst.clone();
                    expl = List::flatten(List::map(expl.clone(), &fnptr!(flattenArrayExpToList, metamodelica::Ref<DAE::Exp>))?)?;
                    expLst = List::map(expl.clone(), &negate)?;
                    Ok((expLst.clone(), expLst.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            expLst = __wb0;
            break 'mc __v;
        }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::ARRAY { array: expl, .. } => {
                    let mut expLst: metamodelica::List<metamodelica::Ref<DAE::Exp>> = expLst.clone();
                    expLst = List::flatten(List::map(expl.clone(), &fnptr!(flattenArrayExpToList, metamodelica::Ref<DAE::Exp>))?)?;
                    Ok((expLst.clone(), expLst.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            expLst = __wb0;
            break 'mc __v;
        }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::UNARY { operator: DAE::Operator::UMINUS_ARR { .. }, exp: Deref @ DAE::Exp::MATRIX { matrix: mexpl, .. } } => {
                    let mut expl: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                    let mut expLst: metamodelica::List<metamodelica::Ref<DAE::Exp>> = expLst.clone();
                    expl = List::flatten(List::map(List::flatten(mexpl.clone())?, &fnptr!(flattenArrayExpToList, metamodelica::Ref<DAE::Exp>))?)?;
                    expLst = List::map(expl.clone(), &negate)?;
                    Ok((expLst.clone(), expLst.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            expLst = __wb0;
            break 'mc __v;
        }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::MATRIX { matrix: mexpl, .. } => {
                    let mut expLst: metamodelica::List<metamodelica::Ref<DAE::Exp>> = expLst.clone();
                    expLst = List::flatten(List::map(List::flatten(mexpl.clone())?, &fnptr!(flattenArrayExpToList, metamodelica::Ref<DAE::Exp>))?)?;
                    Ok((expLst.clone(), expLst.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            expLst = __wb0;
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(list![e.clone()])
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    expLst
}

/* **************************************************/
/* generate  */
/* **************************************************/
pub fn makeNoEvent(mut e1: metamodelica::Ref<DAE::Exp>) -> metamodelica::Ref<DAE::Exp> {
    let mut res: metamodelica::Ref<DAE::Exp>;
    res = makePureBuiltinCall(literal!("noEvent"), list![e1], DAE::T_BOOL_DEFAULT().clone());
    res
}

pub fn makeAbs(mut e1: metamodelica::Ref<DAE::Exp>) -> metamodelica::Ref<DAE::Exp> {
    let mut res: metamodelica::Ref<DAE::Exp>;
    res = makePureBuiltinCall(literal!("abs"), list![e1], DAE::T_REAL_DEFAULT().clone());
    res
}

pub fn makeSign(mut e1: metamodelica::Ref<DAE::Exp>) -> metamodelica::Ref<DAE::Exp> {
    let mut res: metamodelica::Ref<DAE::Exp>;
    res = makePureBuiltinCall(literal!("sign"), list![e1], DAE::T_REAL_DEFAULT().clone());
    res
}

pub fn makeNestedIf(
    mut inConds: &metamodelica::List<metamodelica::Ref<DAE::Exp>>,
    mut inTbExps: &metamodelica::List<metamodelica::Ref<DAE::Exp>>,
    mut fExp: &metamodelica::Ref<DAE::Exp>,
) -> Result<metamodelica::Ref<DAE::Exp>> {
    let mut ifExp: metamodelica::Ref<DAE::Exp>;
    ifExp = (::match_deref::match_deref! { match (inConds, inTbExps) {
        (Deref @ metamodelica::ListNode::Cons { head: c, tail: Deref @ metamodelica::ListNode::Nil }, Deref @ metamodelica::ListNode::Cons { head: tbExp, tail: Deref @ metamodelica::ListNode::Nil }) => {
            metamodelica::Ref::new(DAE::Exp::IFEXP { expCond: c.clone(), expThen: tbExp.clone(), expElse: fExp.clone() })
        },
        (Deref @ metamodelica::ListNode::Cons { head: c, tail: conds }, Deref @ metamodelica::ListNode::Cons { head: tbExp, tail: tbExps }) => {
            ifExp = makeNestedIf(conds, tbExps, fExp)?;
            metamodelica::Ref::new(DAE::Exp::IFEXP { expCond: c.clone(), expThen: tbExp.clone(), expElse: ifExp })
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok(ifExp)
}

pub fn makeCrefExp(
    mut inCref: metamodelica::Ref<DAE::ComponentRef>,
    mut inExpType: metamodelica::Ref<DAE::Type>,
) -> Result<metamodelica::Ref<DAE::Exp>> {
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    outExp = (::match_deref::match_deref! { match &((inCref, inExpType)) {
        (cref, tGiven) => {
            let mut tExisting: Type;
            if Flags::isSet(Flags::CHECK_DAE_CREF_TYPE.clone())? {
                tExisting = ComponentReference::crefLastType(metamodelica::AsArg::as_arg(&cref))?;
                if !(tGiven.clone() == tExisting.clone()) {
                    Debug::traceln({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Warning: Expression.makeCrefExp: cref ")); __mm_s.push_str(&*ComponentReferenceBasics::printComponentRefStr(metamodelica::AsArg::as_arg(&cref))?); __mm_s.push_str(&*literal!(" was given type DAE.CREF.ty: ")); __mm_s.push_str(&*TypesDump::unparseType(tGiven.clone())?); __mm_s.push_str(&*literal!(" is different from existing DAE.CREF.componentRef.ty: ")); __mm_s.push_str(&*TypesDump::unparseType(tExisting)?); ArcStr::from(__mm_s) })?;
                }
            }
            metamodelica::Ref::new(DAE::Exp::CREF { componentRef: cref.clone(), ty: tGiven.clone() })
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outExp)
}

pub fn crefToExp(mut cr: metamodelica::Ref<DAE::ComponentRef>) -> Result<metamodelica::Ref<DAE::Exp>> {
    let mut cref: metamodelica::Ref<DAE::Exp>;
    cref = metamodelica::Ref::new(DAE::Exp::CREF {
        componentRef: cr.clone(),
        ty: ComponentReference::crefTypeFull(&cr)?,
    });
    Ok(cref)
}

pub fn crefExp(mut cr: metamodelica::Ref<DAE::ComponentRef>) -> Result<metamodelica::Ref<DAE::Exp>> {
    let mut cref: metamodelica::Ref<DAE::Exp>;
    cref = (match &*cr {
        DAE::ComponentRef::WILD { .. } => metamodelica::Ref::new(DAE::Exp::CREF {
            componentRef: cr,
            ty: openmodelica_frontend_types::DAE::Type::interned_T_UNKNOWN(),
        }),
        _ => {
            let mut ty1: Type;
            let mut ty2: Type;
            let mut subs: metamodelica::List<metamodelica::Ref<DAE::Subscript>>;
            ty1 = ComponentReference::crefLastType(&cr)?;
            cref = (match &*ty1 {
                DAE::Type::T_ARRAY { .. } => {
                    subs = ComponentReference::crefLastSubs(&cr)?;
                    ty2 = unliftArrayTypeWithSubs(subs, ty1)?;
                    metamodelica::Ref::new(DAE::Exp::CREF {
                        componentRef: cr,
                        ty: ty2,
                    })
                }
                _ => metamodelica::Ref::new(DAE::Exp::CREF {
                    componentRef: cr,
                    ty: ty1,
                }),
            });
            cref
        }
    });
    Ok(cref)
}

pub fn makeASUB(
    mut inExp: metamodelica::Ref<DAE::Exp>,
    mut inSubs: metamodelica::List<metamodelica::Ref<DAE::Exp>>,
) -> Result<metamodelica::Ref<DAE::Exp>> {
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    let mut inSubs_: metamodelica::List<metamodelica::Ref<DAE::Subscript>> = ({
        let mut __acc: metamodelica::List<metamodelica::Ref<DAE::Subscript>> = metamodelica::nil();
        for mut s in (inSubs.clone()).into_iter().cloned() {
            let __x = makeIndexSubscript(s.clone());
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    outExp = (match &*inExp {
        DAE::Exp::ASUB { exp, sub: subs1 } => {
            let mut subs2 = inSubs_.clone();
            let mut subs: metamodelica::List<metamodelica::Ref<DAE::Subscript>>;
            let mut exp = (*exp).clone();
            subs = listAppend(subs1.clone(), subs2);
            exp = metamodelica::Ref::new(DAE::Exp::ASUB {
                exp: exp.clone(),
                sub: subs,
            });
            exp.clone()
        }
        _ => {
            let mut exp: metamodelica::Ref<DAE::Exp>;
            if Flags::isSet(Flags::CHECK_ASUB.clone())? {
                let () = (match &*inExp {
                    DAE::Exp::CREF { .. } => {
                        Debug::traceln({
                            let mut __mm_s = String::new();
                            __mm_s.push_str(&*literal!("Warning: makeASUB: given expression: "));
                            __mm_s.push_str(&*printExpStr(inExp.clone())?);
                            __mm_s.push_str(&*literal!(" contains a component reference!\n"));
                            __mm_s.push_str(&*literal!(" Subscripts exps: ["));
                            __mm_s.push_str(&*stringDelimitList(List::map(inSubs, &printExpStr)?, literal!(",")));
                            __mm_s.push_str(&*literal!("]\n"));
                            __mm_s.push_str(&*literal!("DAE.ASUB should not be used for component references, instead the subscripts should be added directly to the component reference!"));
                            ArcStr::from(__mm_s)
                        })?;
                        ()
                    }
                    _ => (),
                });
            }
            exp = metamodelica::Ref::new(DAE::Exp::ASUB {
                exp: inExp,
                sub: inSubs_,
            });
            exp
        }
    });
    Ok(outExp)
}

pub(crate) fn makeASUBSingleSub(
    mut exp: metamodelica::Ref<DAE::Exp>,
    mut sub: metamodelica::Ref<DAE::Exp>,
) -> Result<metamodelica::Ref<DAE::Exp>> {
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    outExp = makeASUB(exp, list![sub])?;
    Ok(outExp)
}

pub fn makeTuple(mut inExps: metamodelica::List<metamodelica::Ref<DAE::Exp>>) -> Result<metamodelica::Ref<DAE::Exp>> {
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    outExp = if (((inExps).len() as i32) > 1) {
        metamodelica::Ref::new(DAE::Exp::TUPLE { PR: inExps })
    } else {
        (inExps).head().cloned()?
    };
    Ok(outExp)
}

pub fn generateCrefsExpFromExpVar(
    mut inVar: &metamodelica::Ref<DAE::Var>,
    mut inCrefPrefix: &metamodelica::Ref<DAE::ComponentRef>,
) -> Result<metamodelica::Ref<DAE::Exp>> {
    let mut outCrefExp: metamodelica::Ref<DAE::Exp>;
    outCrefExp = (match &**inVar {
        DAE::Var { name, ty, .. } => {
            let mut cr: metamodelica::Ref<DAE::ComponentRef>;
            let mut e: metamodelica::Ref<DAE::Exp>;
            cr = ComponentReference::crefPrependIdent(inCrefPrefix, name, &(metamodelica::nil()), ty)?;
            e = makeCrefExp(cr, ty.clone())?;
            e
        }
    });
    Ok(outCrefExp)
}

pub(crate) fn generateCrefsFromExpVar(
    mut inVar: &metamodelica::Ref<DAE::Var>,
    mut inCrefPrefix: &metamodelica::Ref<DAE::ComponentRef>,
) -> Result<metamodelica::Ref<DAE::ComponentRef>> {
    let mut outCref: metamodelica::Ref<DAE::ComponentRef>;
    outCref = (match &**inVar {
        DAE::Var { name, ty, .. } => {
            let mut cr: metamodelica::Ref<DAE::ComponentRef>;
            cr = ComponentReference::crefPrependIdent(inCrefPrefix, name, &(metamodelica::nil()), ty)?;
            cr
        }
    });
    Ok(outCref)
}

pub fn generateCrefsExpFromExp(
    mut inExp: &metamodelica::Ref<DAE::Exp>,
    mut inCrefPrefix: &metamodelica::Ref<DAE::ComponentRef>,
) -> Result<metamodelica::Ref<DAE::Exp>> {
    let mut outCrefExp: metamodelica::Ref<DAE::Exp>;
    outCrefExp = (::match_deref::match_deref! { match inExp {
        Deref @ DAE::Exp::CREF { componentRef: Deref @ DAE::ComponentRef::WILD { .. }, .. } => {
            inExp.clone()
        },
        Deref @ DAE::Exp::ARRAY { ty, scalar: b, array: explst } => {
            let mut explst = (*explst).clone();
            explst = List::map1(explst.clone(), &move |__a0: metamodelica::Ref<DAE::Exp>, __a1: metamodelica::Ref<DAE::ComponentRef>| generateCrefsExpFromExp(&__a0, &__a1), inCrefPrefix.clone())?;
            metamodelica::Ref::new(DAE::Exp::ARRAY { ty: ty.clone(), scalar: b.clone(), array: explst.clone() })
        },
        Deref @ DAE::Exp::CALL { path: p1, expLst: explst, attr: attr @ Deref @ DAE::CallAttributes { ty: Deref @ DAE::Type::T_COMPLEX { complexClassType: ClassInf::State::RECORD { path: p2 }, .. }, .. } } => {
            let mut explst = (*explst).clone();
            let true = (AbsynUtil::pathEqual(p1, metamodelica::AsArg::as_arg(&p2))) else { return Err("pattern mismatch") };
            explst = List::map1(explst.clone(), &move |__a0: metamodelica::Ref<DAE::Exp>, __a1: metamodelica::Ref<DAE::ComponentRef>| generateCrefsExpFromExp(&__a0, &__a1), inCrefPrefix.clone())?;
            metamodelica::Ref::new(DAE::Exp::CALL { path: p1.clone(), expLst: explst.clone(), attr: attr.clone() })
        },
        Deref @ DAE::Exp::RECORD { path: p1, exps: explst, comp: fields, ty } => {
            let mut explst = (*explst).clone();
            explst = List::map1(explst.clone(), &move |__a0: metamodelica::Ref<DAE::Exp>, __a1: metamodelica::Ref<DAE::ComponentRef>| generateCrefsExpFromExp(&__a0, &__a1), inCrefPrefix.clone())?;
            metamodelica::Ref::new(DAE::Exp::RECORD { path: p1.clone(), exps: explst.clone(), comp: fields.clone(), ty: ty.clone() })
        },
        Deref @ DAE::Exp::CREF { componentRef: cr, ty } => {
            let mut name: ArcStr;
            let mut e: metamodelica::Ref<DAE::Exp>;
            let mut cr = (*cr).clone();
            name = ComponentReference::crefModelicaStr(metamodelica::AsArg::as_arg(&cr));
            cr = ComponentReference::crefPrependIdent(inCrefPrefix, &name, &(metamodelica::nil()), ty)?;
            e = makeCrefExp(cr.clone(), ty.clone())?;
            e
        },
        Deref @ DAE::Exp::UNARY { exp: e, .. } => {
            negate(generateCrefsExpFromExp(e, inCrefPrefix)?)?
        },
        _ => {
            metamodelica::print({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Expression.generateCrefsExpFromExp: fail for")); __mm_s.push_str(&*printExpStr(inExp.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) });
            return Err("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outCrefExp)
}

pub fn generateCrefsExpLstFromExp(
    mut inExp: metamodelica::Ref<DAE::Exp>,
    mut inCrefPrefix: Option<metamodelica::Ref<DAE::ComponentRef>>,
) -> Result<metamodelica::List<metamodelica::Ref<DAE::Exp>>> {
    '__tco: loop {
        ::match_deref::match_deref! { match &((inExp.clone(), inCrefPrefix.clone())) {
            (Deref @ DAE::Exp::TUPLE { PR: explst }, _) => {
                let mut explst = (*explst).clone();
                return Ok(List::flatten(List::map1(explst.clone(), &generateCrefsExpLstFromExp, inCrefPrefix)?)?)
            },
            (Deref @ DAE::Exp::ARRAY { array: explst, .. }, _) => {
                let mut explst = (*explst).clone();
                return Ok(List::flatten(List::map1(explst.clone(), &generateCrefsExpLstFromExp, inCrefPrefix)?)?)
            },
            (Deref @ DAE::Exp::CALL { path: p1, expLst: explst, attr: Deref @ DAE::CallAttributes { ty: Deref @ DAE::Type::T_COMPLEX { complexClassType: ClassInf::State::RECORD { path: p2 }, .. }, .. } }, _) if (AbsynUtil::pathEqual(metamodelica::AsArg::as_arg(&p1), metamodelica::AsArg::as_arg(&p2))) => {
                return Ok(List::flatten(List::map1(explst.clone(), &generateCrefsExpLstFromExp, inCrefPrefix)?)?)
            },
            (Deref @ DAE::Exp::RECORD { exps: explst, .. }, _) => {
                return Ok(List::flatten(List::map1(explst.clone(), &generateCrefsExpLstFromExp, inCrefPrefix)?)?)
            },
            (Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "der" }, expLst: Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Exp::CREF { componentRef: incref, .. }, tail: Deref @ metamodelica::ListNode::Nil }, .. }, _) => {
                let mut cr: metamodelica::Ref<DAE::ComponentRef>;
                let mut e: metamodelica::Ref<DAE::Exp>;
                cr = ComponentReference::crefPrefixDer(incref.clone());
                e = crefExp(cr)?;
                { (inExp, inCrefPrefix) = (e, inCrefPrefix); continue '__tco; }
            },
            (Deref @ DAE::Exp::CREF { componentRef: cr, ty }, Some(incref)) => {
                let mut name: ArcStr;
                let mut e: metamodelica::Ref<DAE::Exp>;
                let mut cr = (*cr).clone();
                name = ComponentReference::crefModelicaStr(metamodelica::AsArg::as_arg(&cr));
                cr = ComponentReference::crefPrependIdent(metamodelica::AsArg::as_arg(&incref), &name, &(metamodelica::nil()), metamodelica::AsArg::as_arg(&ty))?;
                e = makeCrefExp(cr.clone(), ty.clone())?;
                return Ok(list![e])
            },
            (Deref @ DAE::Exp::CREF { .. }, None) => {
                return Ok(list![inExp])
            },
            (Deref @ DAE::Exp::UNARY { exp: e, .. }, _) => {
                { (inExp, inCrefPrefix) = (e.clone(), inCrefPrefix); continue '__tco; }
            },
            _ => {
                metamodelica::print({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Expression.generateCrefsExpLstFromExp: fail for ")); __mm_s.push_str(&*printExpStr(inExp)?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) });
                return Ok(return Err("fail"))
            },
            _ => return Err("match: no arm matched"),
        } }
    }
}

pub fn makeArray(
    mut inElements: metamodelica::List<metamodelica::Ref<DAE::Exp>>,
    mut inType: metamodelica::Ref<DAE::Type>,
    mut inScalar: bool,
) -> metamodelica::Ref<DAE::Exp> {
    let mut outArray: metamodelica::Ref<DAE::Exp>;
    outArray = metamodelica::Ref::new(DAE::Exp::ARRAY {
        ty: inType,
        scalar: inScalar,
        array: inElements,
    });
    outArray
}

pub(crate) fn makeArrayFromList(
    mut inElements: metamodelica::List<metamodelica::Ref<DAE::Exp>>,
) -> Result<metamodelica::Ref<DAE::Exp>> {
    let mut outArray: metamodelica::Ref<DAE::Exp>;
    let mut ty: metamodelica::Ref<DAE::Type>;
    ty = r#typeof((inElements).head().cloned()?)?;
    outArray = metamodelica::Ref::new(DAE::Exp::ARRAY {
        ty: metamodelica::Ref::new(DAE::Type::T_ARRAY {
            ty: ty.clone(),
            dims: list![metamodelica::Ref::new(DAE::Dimension::DIM_INTEGER {
                integer: ((inElements).len() as i32)
            })],
        }),
        scalar: !(Types::isArray(&ty)),
        array: inElements,
    });
    Ok(outArray)
}

pub fn makeScalarArray(
    mut inExpLst: metamodelica::List<metamodelica::Ref<DAE::Exp>>,
    mut et: metamodelica::Ref<DAE::Type>,
) -> metamodelica::Ref<DAE::Exp> {
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    let mut i: i32;
    i = ((inExpLst).len() as i32);
    outExp = metamodelica::Ref::new(DAE::Exp::ARRAY {
        ty: metamodelica::Ref::new(DAE::Type::T_ARRAY {
            ty: et,
            dims: list![metamodelica::Ref::new(DAE::Dimension::DIM_INTEGER { integer: i })],
        }),
        scalar: true,
        array: inExpLst,
    });
    outExp
}

pub(crate) fn makeRealArray(mut expl: metamodelica::List<metamodelica::Ref<DAE::Exp>>) -> metamodelica::Ref<DAE::Exp> {
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    outExp = makeScalarArray(expl, DAE::T_REAL_DEFAULT().clone());
    outExp
}

pub fn makeRealAdd(
    mut inExp1: metamodelica::Ref<DAE::Exp>,
    mut inExp2: metamodelica::Ref<DAE::Exp>,
) -> metamodelica::Ref<DAE::Exp> {
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    outExp = metamodelica::Ref::new(DAE::Exp::BINARY {
        exp1: inExp1,
        operator: DAE::Operator::ADD {
            ty: DAE::T_REAL_DEFAULT().clone(),
        },
        exp2: inExp2,
    });
    outExp
}

pub fn expAdd(
    mut e1: metamodelica::Ref<DAE::Exp>,
    mut e2: metamodelica::Ref<DAE::Exp>,
) -> Result<metamodelica::Ref<DAE::Exp>> {
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    outExp = (::match_deref::match_deref! { match &((&*e1, &*e2)) {
        (_, _) if (isZero(&e1)?) => {
            e2.clone()
        },
        (_, _) if (isZero(&e2)?) => {
            e1.clone()
        },
        (Deref @ DAE::Exp::RCONST { real: r1 }, Deref @ DAE::Exp::RCONST { real: r2 }) => {
            metamodelica::Ref::new(DAE::Exp::RCONST { real: r1.clone() + r2.clone() })
        },
        (Deref @ DAE::Exp::ICONST { integer: i1 }, Deref @ DAE::Exp::ICONST { integer: i2 }) => {
            metamodelica::Ref::new(DAE::Exp::ICONST { integer: i1.clone() + i2.clone() })
        },
        (_, Deref @ DAE::Exp::UNARY { operator: DAE::Operator::UMINUS { .. }, exp: e }) => {
            expSub(e1.clone(), e.clone())?
        },
        (_, Deref @ DAE::Exp::UNARY { operator: DAE::Operator::UMINUS_ARR { .. }, exp: e }) => {
            expSub(e1.clone(), e.clone())?
        },
        (_, Deref @ DAE::Exp::BINARY { exp1: Deref @ DAE::Exp::UNARY { operator: DAE::Operator::UMINUS { .. }, exp: x }, operator: op @ DAE::Operator::MUL { .. }, exp2: y }) => {
            expSub(e1.clone(), metamodelica::Ref::new(DAE::Exp::BINARY { exp1: x.clone(), operator: op.clone(), exp2: y.clone() }))?
        },
        (_, Deref @ DAE::Exp::BINARY { exp1: Deref @ DAE::Exp::UNARY { operator: DAE::Operator::UMINUS_ARR { .. }, exp: x }, operator: op @ DAE::Operator::MUL_ARR { .. }, exp2: y }) => {
            expSub(e1.clone(), metamodelica::Ref::new(DAE::Exp::BINARY { exp1: x.clone(), operator: op.clone(), exp2: y.clone() }))?
        },
        (_, Deref @ DAE::Exp::BINARY { exp1: Deref @ DAE::Exp::UNARY { operator: DAE::Operator::UMINUS { .. }, exp: x }, operator: op @ DAE::Operator::DIV { .. }, exp2: y }) => {
            expSub(e1.clone(), metamodelica::Ref::new(DAE::Exp::BINARY { exp1: x.clone(), operator: op.clone(), exp2: y.clone() }))?
        },
        (_, Deref @ DAE::Exp::BINARY { exp1: Deref @ DAE::Exp::UNARY { operator: DAE::Operator::UMINUS_ARR { .. }, exp: x }, operator: op @ DAE::Operator::DIV_ARR { .. }, exp2: y }) => {
            expSub(e1.clone(), metamodelica::Ref::new(DAE::Exp::BINARY { exp1: x.clone(), operator: op.clone(), exp2: y.clone() }))?
        },
        (Deref @ DAE::Exp::UNARY { operator: DAE::Operator::UMINUS { .. }, exp: e }, _) => {
            expSub(e2.clone(), e.clone())?
        },
        (Deref @ DAE::Exp::UNARY { operator: DAE::Operator::UMINUS_ARR { .. }, exp: e }, _) => {
            expSub(e2.clone(), e.clone())?
        },
        (Deref @ DAE::Exp::BINARY { exp1: Deref @ DAE::Exp::UNARY { operator: DAE::Operator::UMINUS { .. }, exp: x }, operator: op @ DAE::Operator::MUL { .. }, exp2: y }, _) => {
            expSub(e2.clone(), metamodelica::Ref::new(DAE::Exp::BINARY { exp1: x.clone(), operator: op.clone(), exp2: y.clone() }))?
        },
        (Deref @ DAE::Exp::BINARY { exp1: Deref @ DAE::Exp::UNARY { operator: DAE::Operator::UMINUS_ARR { .. }, exp: x }, operator: op @ DAE::Operator::MUL_ARR { .. }, exp2: y }, _) => {
            expSub(e2.clone(), metamodelica::Ref::new(DAE::Exp::BINARY { exp1: x.clone(), operator: op.clone(), exp2: y.clone() }))?
        },
        (Deref @ DAE::Exp::BINARY { exp1: Deref @ DAE::Exp::UNARY { operator: DAE::Operator::UMINUS { .. }, exp: x }, operator: op @ DAE::Operator::DIV { .. }, exp2: y }, _) => {
            expSub(e2.clone(), metamodelica::Ref::new(DAE::Exp::BINARY { exp1: x.clone(), operator: op.clone(), exp2: y.clone() }))?
        },
        (Deref @ DAE::Exp::BINARY { exp1: Deref @ DAE::Exp::UNARY { operator: DAE::Operator::UMINUS_ARR { .. }, exp: x }, operator: op @ DAE::Operator::DIV_ARR { .. }, exp2: y }, _) => {
            expSub(e2.clone(), metamodelica::Ref::new(DAE::Exp::BINARY { exp1: x.clone(), operator: op.clone(), exp2: y.clone() }))?
        },
        (_, _) if (Types::isIntegerOrRealOrSubTypeOfEither(r#typeof(e1.clone())?)) => {
            let mut tp: Type;
            let mut b: bool;
            let mut op: Operator;
            tp = r#typeof(e1.clone())?;
            b = DAEUtil::expTypeArray(&tp);
            op = if (b) {DAE::Operator::ADD_ARR { ty: tp }} else {DAE::Operator::ADD { ty: tp }};
            metamodelica::Ref::new(DAE::Exp::BINARY { exp1: e1.clone(), operator: op, exp2: e2.clone() })
        },
        (_, _) if (Types::isEnumeration(&(r#typeof(e1.clone())?))) => {
            metamodelica::Ref::new(DAE::Exp::BINARY { exp1: e1.clone(), operator: DAE::Operator::ADD { ty: r#typeof(e1.clone())? }, exp2: e2.clone() })
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok(outExp)
}

pub fn expSub(
    mut e1: metamodelica::Ref<DAE::Exp>,
    mut e2: metamodelica::Ref<DAE::Exp>,
) -> Result<metamodelica::Ref<DAE::Exp>> {
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    outExp = (::match_deref::match_deref! { match &((&*e1, &*e2)) {
        (_, _) if (isZero(&e1)?) => {
            negate(e2.clone())?
        },
        (_, _) if (isZero(&e2)?) => {
            e1.clone()
        },
        (Deref @ DAE::Exp::RCONST { real: r1 }, Deref @ DAE::Exp::RCONST { real: r2 }) => {
            metamodelica::Ref::new(DAE::Exp::RCONST { real: r1.clone() - r2.clone() })
        },
        (Deref @ DAE::Exp::ICONST { integer: i1 }, Deref @ DAE::Exp::ICONST { integer: i2 }) => {
            metamodelica::Ref::new(DAE::Exp::ICONST { integer: i1.clone() - i2.clone() })
        },
        (_, Deref @ DAE::Exp::UNARY { operator: DAE::Operator::UMINUS { .. }, exp: e }) => {
            expAdd(e1.clone(), e.clone())?
        },
        (_, Deref @ DAE::Exp::UNARY { operator: DAE::Operator::UMINUS_ARR { .. }, exp: e }) => {
            expAdd(e1.clone(), e.clone())?
        },
        (_, Deref @ DAE::Exp::BINARY { exp1: Deref @ DAE::Exp::UNARY { operator: DAE::Operator::UMINUS { .. }, exp: x }, operator: op @ DAE::Operator::MUL { .. }, exp2: y }) => {
            expAdd(e1.clone(), metamodelica::Ref::new(DAE::Exp::BINARY { exp1: x.clone(), operator: op.clone(), exp2: y.clone() }))?
        },
        (_, Deref @ DAE::Exp::BINARY { exp1: Deref @ DAE::Exp::UNARY { operator: DAE::Operator::UMINUS_ARR { .. }, exp: x }, operator: op @ DAE::Operator::MUL_ARR { .. }, exp2: y }) => {
            expAdd(e1.clone(), metamodelica::Ref::new(DAE::Exp::BINARY { exp1: x.clone(), operator: op.clone(), exp2: y.clone() }))?
        },
        (_, Deref @ DAE::Exp::BINARY { exp1: Deref @ DAE::Exp::UNARY { operator: DAE::Operator::UMINUS { .. }, exp: x }, operator: op @ DAE::Operator::DIV { .. }, exp2: y }) => {
            expAdd(e1.clone(), metamodelica::Ref::new(DAE::Exp::BINARY { exp1: x.clone(), operator: op.clone(), exp2: y.clone() }))?
        },
        (_, Deref @ DAE::Exp::BINARY { exp1: Deref @ DAE::Exp::UNARY { operator: DAE::Operator::UMINUS_ARR { .. }, exp: x }, operator: op @ DAE::Operator::DIV_ARR { .. }, exp2: y }) => {
            expAdd(e1.clone(), metamodelica::Ref::new(DAE::Exp::BINARY { exp1: x.clone(), operator: op.clone(), exp2: y.clone() }))?
        },
        (Deref @ DAE::Exp::UNARY { operator: DAE::Operator::UMINUS { .. }, exp: e }, _) => {
            let mut e = (*e).clone();
            e = expAdd(e.clone(), e2.clone())?;
            negate(e.clone())?
        },
        (Deref @ DAE::Exp::UNARY { operator: DAE::Operator::UMINUS_ARR { .. }, exp: e }, _) => {
            let mut e = (*e).clone();
            e = expAdd(e.clone(), e2.clone())?;
            negate(e.clone())?
        },
        (_, _) if (Types::isIntegerOrRealOrSubTypeOfEither(r#typeof(e1.clone())?)) => {
            let mut tp: Type;
            let mut b: bool;
            let mut op: Operator;
            tp = r#typeof(e1.clone())?;
            b = DAEUtil::expTypeArray(&tp);
            op = if (b) {DAE::Operator::SUB_ARR { ty: tp }} else {DAE::Operator::SUB { ty: tp }};
            metamodelica::Ref::new(DAE::Exp::BINARY { exp1: e1.clone(), operator: op, exp2: e2.clone() })
        },
        (_, _) if (Types::isEnumeration(&(r#typeof(e1.clone())?))) => {
            metamodelica::Ref::new(DAE::Exp::BINARY { exp1: e1.clone(), operator: DAE::Operator::SUB { ty: r#typeof(e1.clone())? }, exp2: e2.clone() })
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok(outExp)
}

pub(crate) fn makeLBinary(
    mut inExpLst: &metamodelica::List<metamodelica::Ref<DAE::Exp>>,
    mut op: &DAE::Operator,
) -> Result<metamodelica::Ref<DAE::Exp>> {
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    outExp = (::match_deref::match_deref! { match &((&**inExpLst, op)) {
        (Deref @ metamodelica::ListNode::Nil, DAE::Operator::AND { ty: _ }) => {
            metamodelica::Ref::new(DAE::Exp::BCONST { bool: true })
        },
        (Deref @ metamodelica::ListNode::Nil, DAE::Operator::OR { ty: _ }) => {
            metamodelica::Ref::new(DAE::Exp::BCONST { bool: false })
        },
        (Deref @ metamodelica::ListNode::Cons { head: e1, tail: Deref @ metamodelica::ListNode::Nil }, _) => {
            e1.clone()
        },
        (Deref @ metamodelica::ListNode::Cons { head: e1, tail: Deref @ metamodelica::ListNode::Cons { head: e2, tail: Deref @ metamodelica::ListNode::Nil } }, _) => {
            metamodelica::Ref::new(DAE::Exp::LBINARY { exp1: e1.clone(), operator: op.clone(), exp2: e2.clone() })
        },
        (Deref @ metamodelica::ListNode::Cons { head: e1, tail: rest }, _) => {
            let mut res: metamodelica::Ref<DAE::Exp>;
            res = makeLBinary(metamodelica::AsArg::as_arg(&rest), op)?;
            res = metamodelica::Ref::new(DAE::Exp::LBINARY { exp1: e1.clone(), operator: op.clone(), exp2: res });
            res
        },
        _ => {
            let mut r#str: ArcStr;
            r#str = { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Expression.makeLBinary failed for operator ")); __mm_s.push_str(&*ExpressionDump::lbinopSymbol(op)?); ArcStr::from(__mm_s) };
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![r#str])?;
            return Err("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outExp)
}

pub fn makeSum1(
    mut inExpLst: metamodelica::List<metamodelica::Ref<DAE::Exp>>,
    mut simplify: bool,
) -> Result<metamodelica::Ref<DAE::Exp>> {
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    let mut e1: metamodelica::Ref<DAE::Exp>;
    let mut e2: metamodelica::Ref<DAE::Exp>;
    outExp = 'mc: {
        let __mc_input = &*inExpLst;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Nil => {
                    Ok(metamodelica::Ref::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(0.0_f64) }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: e1, tail: Deref @ metamodelica::ListNode::Nil } => {
                    Ok(e1.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: e1, tail: Deref @ metamodelica::ListNode::Cons { head: e2, tail: Deref @ metamodelica::ListNode::Nil } } => {
                    Ok(expAdd(e1.clone(), e2.clone())?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(makeSumWork(inExpLst.clone(), simplify)?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    if Flags::isSet(Flags::FAILTRACE.clone())? {
                        Debug::trace(literal!("-Expression.makeSum1 failed, DAE.Exp lst:"))?;
                        Debug::trace(ExpressionDump::printExpListStr(inExpLst.clone())?)?;
                    }
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(outExp)
}

fn makeSumWork(
    mut inExpLst: metamodelica::List<metamodelica::Ref<DAE::Exp>>,
    mut simplify: bool,
) -> Result<metamodelica::Ref<DAE::Exp>> {
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    let mut terms: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
    let mut rest: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
    terms = ({
        let mut __acc: metamodelica::List<metamodelica::Ref<DAE::Exp>> = metamodelica::nil();
        for mut e in (inExpLst.clone()).into_iter().cloned() {
            if !(!(isZero(&(e.clone()))?)) {
                continue;
            }
            let __x = e.clone();
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    if (terms).is_empty() {
        outExp = (inExpLst).head().cloned()?;
    } else if ((terms).len() as i32) > MAX_SUM_CHAIN.clone() {
        outExp = balancedSum(terms, simplify)?;
    } else {
        let (__pa0, __pa1) = ::match_deref::match_deref! { match &(terms) {
            Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
            _ => return Err("pattern mismatch"),
        } };
        outExp = metamodelica::Own::own(__pa0);
        rest = metamodelica::Own::own(__pa1);
        for mut e in &*rest {
            outExp = metamodelica::Ref::new(DAE::Exp::BINARY {
                exp1: outExp.clone(),
                operator: addOperator(outExp, e.clone())?,
                exp2: e.clone(),
            });
            if simplify {
                (outExp, _) = ExpressionSimplify::simplify1(outExp)?;
            }
        }
    }
    Ok(outExp)
}

pub fn makeSum(mut inExpLst: metamodelica::List<metamodelica::Ref<DAE::Exp>>) -> Result<metamodelica::Ref<DAE::Exp>> {
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    let mut terms: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
    terms = ({
        let mut __acc: metamodelica::List<metamodelica::Ref<DAE::Exp>> = metamodelica::nil();
        for mut e in (inExpLst.clone()).into_iter().cloned() {
            if !(!(isZero(&(e.clone()))?)) {
                continue;
            }
            let __x = e.clone();
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    if (terms).is_empty() {
        outExp = if ((inExpLst).is_empty()) {
            metamodelica::Ref::new(DAE::Exp::RCONST {
                real: metamodelica::OrderedFloat(0.0_f64),
            })
        } else {
            List::last(&inExpLst)?
        };
    } else if ((terms).len() as i32) > MAX_SUM_CHAIN.clone() {
        outExp = balancedSum(terms, false)?;
    } else {
        terms = terms.reverse();
        let (__pa0, __pa1) = ::match_deref::match_deref! { match &(terms) {
            Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
            _ => return Err("pattern mismatch"),
        } };
        outExp = metamodelica::Own::own(__pa0);
        terms = metamodelica::Own::own(__pa1);
        for mut e in &*terms {
            outExp = metamodelica::Ref::new(DAE::Exp::BINARY {
                exp1: e.clone(),
                operator: addOperator(e.clone(), outExp.clone())?,
                exp2: outExp,
            });
        }
    }
    Ok(outExp)
}

fn addOperator(mut e1: metamodelica::Ref<DAE::Exp>, mut e2: metamodelica::Ref<DAE::Exp>) -> Result<DAE::Operator> {
    let mut op: DAE::Operator;
    let mut tp: Type = r#typeof(e1.clone())?;
    if !(DAEUtil::expTypeArray(&tp)) {
        tp = r#typeof(e2)?;
    }
    op = if (DAEUtil::expTypeArray(&tp)) {
        DAE::Operator::ADD_ARR { ty: tp }
    } else {
        DAE::Operator::ADD { ty: tp }
    };
    Ok(op)
}

fn balancedSum(
    mut inExpLst: metamodelica::List<metamodelica::Ref<DAE::Exp>>,
    mut simplify: bool,
) -> Result<metamodelica::Ref<DAE::Exp>> {
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    let mut level: metamodelica::List<metamodelica::Ref<DAE::Exp>> = inExpLst;
    let mut next: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
    let mut e1: metamodelica::Ref<DAE::Exp>;
    let mut e2: metamodelica::Ref<DAE::Exp>;
    while !(((level).rest()?).is_empty()) {
        next = metamodelica::nil();
        while !((level).is_empty()) {
            let (__pa0, __pa1) = ::match_deref::match_deref! { match &(level) {
                Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
                _ => return Err("pattern mismatch"),
            } };
            e1 = metamodelica::Own::own(__pa0);
            level = metamodelica::Own::own(__pa1);
            if (level).is_empty() {
                next = metamodelica::cons(e1, next);
            } else {
                let (__pa2, __pa3) = ::match_deref::match_deref! { match &(level) {
                    Deref @ metamodelica::ListNode::Cons { head: __pa2, tail: __pa3 } => (__pa2.clone(), __pa3.clone()),
                    _ => return Err("pattern mismatch"),
                } };
                e2 = metamodelica::Own::own(__pa2);
                level = metamodelica::Own::own(__pa3);
                outExp = metamodelica::Ref::new(DAE::Exp::BINARY {
                    exp1: e1.clone(),
                    operator: addOperator(e1, e2.clone())?,
                    exp2: e2,
                });
                next = metamodelica::cons(
                    if (simplify) {
                        (ExpressionSimplify::simplify1(outExp)?).0
                    } else {
                        outExp
                    },
                    next,
                );
            }
        }
        level = metamodelica::Dangerous::listReverseInPlace(next);
    }
    outExp = (level).head().cloned()?;
    Ok(outExp)
}

pub fn expMul(
    mut e1: metamodelica::Ref<DAE::Exp>,
    mut e2: metamodelica::Ref<DAE::Exp>,
) -> Result<metamodelica::Ref<DAE::Exp>> {
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    outExp = 'mc: {
        let __mc_input = (&*e1, &*e2);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _) => {
                    let true = (isZero(&e1)?) else { return Err("pattern mismatch") };
                    Ok(e1.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _) => {
                    let true = (isZero(&e2)?) else { return Err("pattern mismatch") };
                    Ok(e2.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::RCONST { real: __rlit_1 }, _) => {
                    if !(__rlit_1.eq(&metamodelica::OrderedFloat((1.0) as f64))) { return Err("guard") }
                    Ok(e2.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Deref @ DAE::Exp::RCONST { real: __rlit_2 }) => {
                    if !(__rlit_2.eq(&metamodelica::OrderedFloat((1.0) as f64))) { return Err("guard") }
                    Ok(e1.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::ICONST { integer: 1 }, _) => {
                    Ok(e2.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Deref @ DAE::Exp::ICONST { integer: 1 }) => {
                    Ok(e1.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::RCONST { real: r1 }, Deref @ DAE::Exp::RCONST { real: r2 }) => {
                    let mut r1 = (*r1).clone();
                    r1 = (r1.clone()) * (r2.clone());
                    Ok(metamodelica::Ref::new(DAE::Exp::RCONST { real: r1.clone() }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::ICONST { integer: i1 }, Deref @ DAE::Exp::ICONST { integer: i2 }) => {
                    let mut i1 = (*i1).clone();
                    i1 = intMul(i1.clone(), i2.clone());
                    Ok(metamodelica::Ref::new(DAE::Exp::ICONST { integer: i1.clone() }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut tp: Type;
                    let mut b1: bool;
                    let mut b2: bool;
                    let mut op: Operator;
                    let mut e1_1: metamodelica::Ref<DAE::Exp>;
                    let mut e2_1: metamodelica::Ref<DAE::Exp>;
                    tp = r#typeof(e1.clone())?;
                    let true = (Types::isIntegerOrRealOrSubTypeOfEither(tp.clone())) else { return Err("pattern mismatch") };
                    b1 = DAEUtil::expTypeArray(&tp);
                    tp = r#typeof(e2.clone())?;
                    let true = (Types::isIntegerOrRealOrSubTypeOfEither(tp.clone())) else { return Err("pattern mismatch") };
                    b2 = DAEUtil::expTypeArray(&tp);
                    (e1_1, e2_1) = Util::swap(!(b1) && b2, e1.clone(), e2.clone());
                    op = if (b1 && b2) {DAE::Operator::MUL_ARR { ty: tp.clone() }} else {if (b1 == b2) {DAE::Operator::MUL { ty: tp.clone() }} else {DAE::Operator::MUL_ARRAY_SCALAR { ty: tp.clone() }}};
                    Ok(metamodelica::Ref::new(DAE::Exp::BINARY { exp1: e1_1.clone(), operator: op.clone(), exp2: e2_1.clone() }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(outExp)
}

pub fn expPow(
    mut e1: metamodelica::Ref<DAE::Exp>,
    mut e2: metamodelica::Ref<DAE::Exp>,
) -> Result<metamodelica::Ref<DAE::Exp>> {
    '__tco: loop {
        ::match_deref::match_deref! { match &((&*e1, &*e2)) {
            (_, _) if (isOne(&e2)) => {
                return Ok(e1.clone())
            },
            (_, _) if (isZero(&e2)?) => {
                return Ok(makeConstOne(&(r#typeof(e1.clone())?)))
            },
            (_, _) if (isConstOne(&e1)) => {
                return Ok(e1.clone())
            },
            (_, _) if (isZero(&e1)? && isPositive(e2.clone())?) => {
                return Ok(makeConstZero(&(r#typeof(e1.clone())?)))
            },
            (Deref @ DAE::Exp::UNARY { operator: DAE::Operator::UMINUS { .. }, exp: e }, _) if (isEven(&e2)) => {
                { (e1, e2) = (e.clone(), e2.clone()); continue '__tco; }
            },
            (Deref @ DAE::Exp::BINARY { exp1: e3, operator: DAE::Operator::DIV { .. }, exp2: e4 }, Deref @ DAE::Exp::UNARY { operator: DAE::Operator::UMINUS { .. }, exp: e5 }) => {
                let mut e: metamodelica::Ref<DAE::Exp>;
                e = makeDiv(e4.clone(), e3.clone())?;
                { (e1, e2) = (e, e5.clone()); continue '__tco; }
            },
            (Deref @ DAE::Exp::BINARY { exp1: e3, operator: DAE::Operator::DIV { .. }, exp2: e4 }, _) if (isNegativeOrZero(e2.clone())?) => {
                { (e1, e2) = (makeDiv(e4.clone(), e3.clone())?, negate(e2.clone())?); continue '__tco; }
            },
            (_, _) if (isHalf(&e2) && isPositiveOrZero(e1.clone())?) => {
                return Ok(makePureBuiltinCall(literal!("sqrt"), list![e1.clone()], DAE::T_REAL_DEFAULT().clone()))
            },
            _ => {
                let mut tp: Type;
                let mut b: bool;
                let mut op: Operator;
                tp = r#typeof(e1.clone())?;
                b = DAEUtil::expTypeArray(&tp);
                op = if (b) {DAE::Operator::POW_ARR { ty: tp }} else {DAE::Operator::POW { ty: tp }};
                return Ok(metamodelica::Ref::new(DAE::Exp::BINARY { exp1: e1.clone(), operator: op, exp2: e2.clone() }))
            },
            _ => return Err("match: no arm matched"),
        } }
    }
}

pub(crate) fn expPowLst(
    mut expLst: metamodelica::List<metamodelica::Ref<DAE::Exp>>,
    mut n: metamodelica::Ref<DAE::Exp>,
) -> Result<metamodelica::List<metamodelica::Ref<DAE::Exp>>> {
    let mut outExp: metamodelica::List<metamodelica::Ref<DAE::Exp>> = List::map1(expLst.clone(), &expPow, n.clone())?;
    Ok(outExp)
}

pub fn expMaxScalar(
    mut e1: metamodelica::Ref<DAE::Exp>,
    mut e2: metamodelica::Ref<DAE::Exp>,
) -> Result<metamodelica::Ref<DAE::Exp>> {
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    let mut tp: Type;
    tp = r#typeof(e1.clone())?;
    outExp = metamodelica::Ref::new(DAE::Exp::CALL {
        path: metamodelica::Ref::new(Absyn::Path::IDENT { name: literal!("max") }),
        expLst: list![e1, e2],
        attr: metamodelica::Ref::new(DAE::CallAttributes {
            ty: tp,
            tuple_: false,
            builtin: true,
            isImpure: false,
            isFunctionPointerCall: false,
            inlineType: openmodelica_frontend_types::DAE::InlineType::NO_INLINE,
            tailCall: openmodelica_frontend_types::DAE::TailCall::NO_TAIL,
            noReturn: DAE::NoReturn::RETURNS.clone(),
        }),
    });
    Ok(outExp)
}

pub fn expOptMaxScalar(
    mut e1: Option<metamodelica::Ref<DAE::Exp>>,
    mut e2: Option<metamodelica::Ref<DAE::Exp>>,
) -> Result<Option<metamodelica::Ref<DAE::Exp>>> {
    let mut outExp: Option<metamodelica::Ref<DAE::Exp>>;
    outExp = (::match_deref::match_deref! { match &((e1.clone(), e2.clone())) {
        (_, None) => {
            e1
        },
        (None, _) => {
            e2
        },
        (Some(e11), Some(e22)) => {
            Some(expMaxScalar(e11.clone(), e22.clone())?)
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok(outExp)
}

pub fn expMinScalar(
    mut e1: metamodelica::Ref<DAE::Exp>,
    mut e2: metamodelica::Ref<DAE::Exp>,
) -> Result<metamodelica::Ref<DAE::Exp>> {
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    let mut tp: Type;
    tp = r#typeof(e1.clone())?;
    outExp = metamodelica::Ref::new(DAE::Exp::CALL {
        path: metamodelica::Ref::new(Absyn::Path::IDENT { name: literal!("min") }),
        expLst: list![e1, e2],
        attr: metamodelica::Ref::new(DAE::CallAttributes {
            ty: tp,
            tuple_: false,
            builtin: true,
            isImpure: false,
            isFunctionPointerCall: false,
            inlineType: openmodelica_frontend_types::DAE::InlineType::NO_INLINE,
            tailCall: openmodelica_frontend_types::DAE::TailCall::NO_TAIL,
            noReturn: DAE::NoReturn::RETURNS.clone(),
        }),
    });
    Ok(outExp)
}

pub fn expOptMinScalar(
    mut e1: Option<metamodelica::Ref<DAE::Exp>>,
    mut e2: Option<metamodelica::Ref<DAE::Exp>>,
) -> Result<Option<metamodelica::Ref<DAE::Exp>>> {
    let mut outExp: Option<metamodelica::Ref<DAE::Exp>>;
    outExp = (::match_deref::match_deref! { match &((e1.clone(), e2.clone())) {
        (_, None) => {
            e1
        },
        (None, _) => {
            e2
        },
        (Some(e11), Some(e22)) => {
            Some(expMinScalar(e11.clone(), e22.clone())?)
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok(outExp)
}

pub(crate) fn makeProductVector(
    mut e1: metamodelica::Ref<DAE::Exp>,
    mut v: metamodelica::List<metamodelica::Ref<DAE::Exp>>,
) -> Result<metamodelica::List<metamodelica::Ref<DAE::Exp>>> {
    let mut res: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
    res = List::map1(v, &makeProduct, e1)?;
    Ok(res)
}

pub fn makeScalarProduct(
    mut v: metamodelica::Array<metamodelica::Ref<DAE::Exp>>,
    mut w: metamodelica::Array<metamodelica::Ref<DAE::Exp>>,
) -> Result<metamodelica::Ref<DAE::Exp>> {
    let mut s: metamodelica::Ref<DAE::Exp> = metamodelica::Ref::new(DAE::Exp::RCONST {
        real: metamodelica::OrderedFloat(0.0_f64),
    });
    let mut size1: i32 = metamodelica::arrayLength(v.clone());
    let mut size2: i32 = metamodelica::arrayLength(w.clone());
    if size1 != size2 {
        metamodelica::print(literal!("makeScalarProduct faili.\n"));
        return Ok(s);
    }
    s = makeSum1(
        ({
            let mut __acc: metamodelica::List<metamodelica::Ref<DAE::Exp>> = metamodelica::nil();
            for mut i in (1..=size1).into_iter() {
                let __x = expMul(
                    metamodelica::arrayGet(v.clone(), i.clone())?,
                    metamodelica::arrayGet(w.clone(), i.clone())?,
                )?;
                __acc = cons(__x, __acc);
            }
            __acc.reverse()
        }),
        false,
    )?;
    (s, _) = ExpressionSimplify::simplify(s)?;
    Ok(s)
}

pub fn lenVec(mut v: metamodelica::Array<metamodelica::Ref<DAE::Exp>>) -> Result<metamodelica::Ref<DAE::Exp>> {
    let mut len: metamodelica::Ref<DAE::Exp> = makeScalarProduct(v.clone(), v.clone())?;
    len = makePureBuiltinCall(literal!("sqrt"), list![len], DAE::T_REAL_DEFAULT().clone());
    Ok(len)
}

pub fn subVec(
    mut v: metamodelica::Array<metamodelica::Ref<DAE::Exp>>,
    mut w: metamodelica::Array<metamodelica::Ref<DAE::Exp>>,
) -> Result<metamodelica::Array<metamodelica::Ref<DAE::Exp>>> {
    let mut y: metamodelica::Array<metamodelica::Ref<DAE::Exp>>;
    let mut size1: i32 = metamodelica::arrayLength(v.clone());
    let mut size2: i32 = metamodelica::arrayLength(w.clone());
    if size1 != size2 {
        metamodelica::print(literal!("subVec fail.\n"));
        return Err("fail");
    }
    y = arrayCreate(
        size1,
        metamodelica::Ref::new(DAE::Exp::RCONST {
            real: metamodelica::OrderedFloat(0.0_f64),
        }),
    );
    for mut i in 1..=size1 {
        metamodelica::arrayUpdate(
            y.clone(),
            i,
            expSub(
                metamodelica::arrayGet(v.clone(), i)?,
                metamodelica::arrayGet(w.clone(), i)?,
            )?,
        )?;
    }
    Ok(y)
}

pub(crate) fn makeProduct(
    mut e1: metamodelica::Ref<DAE::Exp>,
    mut e2: metamodelica::Ref<DAE::Exp>,
) -> Result<metamodelica::Ref<DAE::Exp>> {
    let mut product: metamodelica::Ref<DAE::Exp>;
    product = makeProductLst(list![e1, e2])?;
    Ok(product)
}

pub fn makeProductLst(
    mut inExpLst: metamodelica::List<metamodelica::Ref<DAE::Exp>>,
) -> Result<metamodelica::Ref<DAE::Exp>> {
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    outExp = 'mc: {
        let __mc_input = inExpLst;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Nil => {
                    Ok(metamodelica::Ref::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(1.0_f64) }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: e1, tail: Deref @ metamodelica::ListNode::Nil } => {
                    Ok(e1.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: e, tail: es } => {
                    let mut res: metamodelica::Ref<DAE::Exp>;
                    let true = (isConstOne(metamodelica::AsArg::as_arg(&e))) else { return Err("pattern mismatch") };
                    res = makeProductLst(es.clone())?;
                    Ok(res.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Exp::BINARY { operator: DAE::Operator::DIV { .. }, exp2: e, .. }, tail: _ } => {
                    let true = (isZero(metamodelica::AsArg::as_arg(&e))?) else { return Err("pattern mismatch") };
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: _, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Exp::BINARY { operator: DAE::Operator::DIV { .. }, exp2: e, .. }, tail: Deref @ metamodelica::ListNode::Nil } } => {
                    let true = (isZero(metamodelica::AsArg::as_arg(&e))?) else { return Err("pattern mismatch") };
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: e, tail: _ } => {
                    let true = (isZero(metamodelica::AsArg::as_arg(&e))?) else { return Err("pattern mismatch") };
                    Ok(e.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Exp::BINARY { exp1: e1, operator: DAE::Operator::DIV { ty: tp }, exp2: e }, tail: Deref @ metamodelica::ListNode::Cons { head: e2, tail: Deref @ metamodelica::ListNode::Nil } } => {
                    let true = (isConstOne(metamodelica::AsArg::as_arg(&e1))) else { return Err("pattern mismatch") };
                    Ok(metamodelica::Ref::new(DAE::Exp::BINARY { exp1: e2.clone(), operator: DAE::Operator::DIV { ty: tp.clone() }, exp2: e.clone() }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: e2, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Exp::BINARY { exp1: e1, operator: DAE::Operator::DIV { ty: tp }, exp2: e }, tail: Deref @ metamodelica::ListNode::Nil } } => {
                    let true = (isConstOne(metamodelica::AsArg::as_arg(&e1))) else { return Err("pattern mismatch") };
                    Ok(metamodelica::Ref::new(DAE::Exp::BINARY { exp1: e2.clone(), operator: DAE::Operator::DIV { ty: tp.clone() }, exp2: e.clone() }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Exp::BINARY { exp1: e1, operator: DAE::Operator::DIV { ty: tp }, exp2: e }, tail: es } => {
                    let mut res: metamodelica::Ref<DAE::Exp>;
                    let mut p1: metamodelica::Ref<DAE::Exp>;
                    let mut b_isZero: bool;
                    let true = (isConstOne(metamodelica::AsArg::as_arg(&e1))) else { return Err("pattern mismatch") };
                    p1 = makeProductLst(es.clone())?;
                    res = metamodelica::Ref::new(DAE::Exp::BINARY { exp1: p1.clone(), operator: DAE::Operator::DIV { ty: tp.clone() }, exp2: e.clone() });
                    b_isZero = isZero(&p1)?;
                    res = if (b_isZero) {makeConstZero(&(r#typeof(e.clone())?))} else {res.clone()};
                    Ok(res.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: e1, tail: Deref @ metamodelica::ListNode::Cons { head: e2, tail: Deref @ metamodelica::ListNode::Nil } } => {
                    let true = (isConstOne(metamodelica::AsArg::as_arg(&e2))) else { return Err("pattern mismatch") };
                    Ok(e1.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: e1, tail: Deref @ metamodelica::ListNode::Cons { head: e2, tail: Deref @ metamodelica::ListNode::Nil } } => {
                    let mut res: metamodelica::Ref<DAE::Exp>;
                    let mut tp: Type;
                    let mut b_isZero: bool;
                    let mut b1: bool;
                    let mut b2: bool;
                    b1 = isZero(metamodelica::AsArg::as_arg(&e1))?;
                    b2 = isZero(metamodelica::AsArg::as_arg(&e2))?;
                    b_isZero = boolOr(b1, b2);
                    tp = r#typeof(e1.clone())?;
                    tp = checkIfOther(tp.clone());
                    res = metamodelica::Ref::new(DAE::Exp::BINARY { exp1: e1.clone(), operator: DAE::Operator::MUL { ty: tp.clone() }, exp2: e2.clone() });
                    res = if (b_isZero) {makeConstZero(&tp)} else {res.clone()};
                    Ok(res.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: e1, tail: rest } => {
                    let mut res: metamodelica::Ref<DAE::Exp>;
                    let mut e2: metamodelica::Ref<DAE::Exp>;
                    let mut tp: Type;
                    let mut b_isZero: bool;
                    let mut b1: bool;
                    let mut b2: bool;
                    e2 = makeProductLst(rest.clone())?;
                    tp = r#typeof(e1.clone())?;
                    tp = checkIfOther(tp.clone());
                    res = metamodelica::Ref::new(DAE::Exp::BINARY { exp1: e1.clone(), operator: DAE::Operator::MUL { ty: tp.clone() }, exp2: e2.clone() });
                    b1 = isZero(metamodelica::AsArg::as_arg(&e1))?;
                    b2 = isZero(&e2)?;
                    b_isZero = boolOr(b1, b2);
                    res = if (b_isZero) {makeConstZero(&(r#typeof(e1.clone())?))} else {res.clone()};
                    Ok(res.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                lst => {
                    let mut explst: metamodelica::List<ArcStr>;
                    let mut r#str: ArcStr;
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { return Err("pattern mismatch") };
                    Debug::trace(literal!("-Expression.makeProductLst failed, DAE.Exp lst:"))?;
                    explst = List::map(lst.clone(), &printExpStr)?;
                    r#str = stringDelimitList(explst.clone(), literal!(", "));
                    Debug::traceln(r#str.clone())?;
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(outExp)
}

fn checkIfOther(mut inTp: metamodelica::Ref<DAE::Type>) -> metamodelica::Ref<DAE::Type> {
    let mut outTp: metamodelica::Ref<DAE::Type>;
    outTp = (match &*inTp {
        DAE::Type::T_UNKNOWN { .. } => DAE::T_REAL_DEFAULT().clone(),
        _ => inTp,
    });
    outTp
}

pub fn expDiv(
    mut e1: metamodelica::Ref<DAE::Exp>,
    mut e2: metamodelica::Ref<DAE::Exp>,
) -> Result<metamodelica::Ref<DAE::Exp>> {
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    let mut tp: Type;
    let mut b: bool;
    let mut op: Operator;
    tp = r#typeof(e1.clone())?;
    let true = (Types::isIntegerOrRealOrSubTypeOfEither(tp.clone())) else {
        return Err("pattern mismatch");
    };
    b = DAEUtil::expTypeArray(&tp);
    op = if (b) {
        DAE::Operator::DIV_ARR { ty: tp }
    } else {
        DAE::Operator::DIV { ty: tp }
    };
    outExp = metamodelica::Ref::new(DAE::Exp::BINARY {
        exp1: e1,
        operator: op,
        exp2: e2,
    });
    Ok(outExp)
}

pub fn makeDiv(
    mut e1: metamodelica::Ref<DAE::Exp>,
    mut e2: metamodelica::Ref<DAE::Exp>,
) -> Result<metamodelica::Ref<DAE::Exp>> {
    let mut res: metamodelica::Ref<DAE::Exp>;
    res = (match &*e2 {
        _ if (isZero(&e1)? && !(isZero(&e2)?)) => e1.clone(),
        _ if (isOne(&e2)) => e1.clone(),
        _ => expDiv(e1.clone(), e2.clone())?,
    });
    Ok(res)
}

pub(crate) fn makeDivVector(
    mut v: metamodelica::List<metamodelica::Ref<DAE::Exp>>,
    mut e1: metamodelica::Ref<DAE::Exp>,
) -> Result<metamodelica::List<metamodelica::Ref<DAE::Exp>>> {
    let mut res: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
    res = List::map1(v, &makeDiv, e1)?;
    Ok(res)
}

pub fn makeAsubAddIndex(mut e: metamodelica::Ref<DAE::Exp>, mut indx: i32) -> Result<metamodelica::Ref<DAE::Exp>> {
    let mut outExp: metamodelica::Ref<DAE::Exp> = e.clone();
    outExp = (match &*outExp {
        DAE::Exp::ASUB { sub: __outExp_sub, .. } => {
            assign_variant_field!(outExp => DAE::Exp::ASUB; sub = listAppend(__outExp_sub.clone(), list![metamodelica::Ref::new(DAE::Subscript::INDEX { exp: metamodelica::Ref::new(DAE::Exp::ICONST { integer: indx }) })]));
            outExp
        }
        _ => makeASUB(e, list![metamodelica::Ref::new(DAE::Exp::ICONST { integer: indx })])?,
    });
    Ok(outExp)
}

pub(crate) fn makeIntegerExp(mut i: i32) -> metamodelica::Ref<DAE::Exp> {
    let mut e: metamodelica::Ref<DAE::Exp>;
    e = metamodelica::Ref::new(DAE::Exp::ICONST { integer: i });
    e
}

pub fn makeRealExp(mut r: metamodelica::Real) -> metamodelica::Ref<DAE::Exp> {
    let mut e: metamodelica::Ref<DAE::Exp>;
    e = metamodelica::Ref::new(DAE::Exp::RCONST { real: r });
    e
}

pub(crate) fn makeBoolExp(mut b: bool) -> metamodelica::Ref<DAE::Exp> {
    let mut e: metamodelica::Ref<DAE::Exp>;
    e = metamodelica::Ref::new(DAE::Exp::BCONST { bool: b });
    e
}

pub fn makeConstOne(mut inType: &metamodelica::Ref<DAE::Type>) -> metamodelica::Ref<DAE::Exp> {
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    outExp = (match &**inType {
        DAE::Type::T_INTEGER { .. } => metamodelica::Ref::new(DAE::Exp::ICONST { integer: 1 }),
        DAE::Type::T_REAL { .. } => metamodelica::Ref::new(DAE::Exp::RCONST {
            real: metamodelica::OrderedFloat(1.0_f64),
        }),
        _ => metamodelica::Ref::new(DAE::Exp::RCONST {
            real: metamodelica::OrderedFloat(1.0_f64),
        }),
    });
    outExp
}

pub fn makeConstZero(mut inType: &metamodelica::Ref<DAE::Type>) -> metamodelica::Ref<DAE::Exp> {
    let mut r#const: metamodelica::Ref<DAE::Exp>;
    r#const = (match &**inType {
        DAE::Type::T_REAL { .. } => metamodelica::Ref::new(DAE::Exp::RCONST {
            real: metamodelica::OrderedFloat(0.0_f64),
        }),
        DAE::Type::T_INTEGER { .. } => metamodelica::Ref::new(DAE::Exp::ICONST { integer: 0 }),
        _ => metamodelica::Ref::new(DAE::Exp::RCONST {
            real: metamodelica::OrderedFloat(0.0_f64),
        }),
    });
    r#const
}

pub(crate) fn makeConstNumber(mut ty: &metamodelica::Ref<DAE::Type>, mut n: i32) -> metamodelica::Ref<DAE::Exp> {
    let mut exp: metamodelica::Ref<DAE::Exp>;
    exp = (match &**ty {
        DAE::Type::T_INTEGER { .. } => metamodelica::Ref::new(DAE::Exp::ICONST { integer: n }),
        _ => metamodelica::Ref::new(DAE::Exp::RCONST {
            real: metamodelica::OrderedFloat((n) as f64),
        }),
    });
    exp
}

pub fn makeConstZeroE(mut iExp: metamodelica::Ref<DAE::Exp>) -> Result<metamodelica::Ref<DAE::Exp>> {
    let mut r#const: metamodelica::Ref<DAE::Exp>;
    let mut tp: metamodelica::Ref<DAE::Type> = r#typeof(iExp.clone())?;
    r#const = makeConstZero(&tp);
    Ok(r#const)
}

pub(crate) fn makeListOfZeros(mut inDimension: i32) -> metamodelica::List<metamodelica::Ref<DAE::Exp>> {
    let mut outList: metamodelica::List<metamodelica::Ref<DAE::Exp>> = metamodelica::nil();
    if inDimension > 0 {
        for mut i in 1..=inDimension {
            outList = metamodelica::cons(
                metamodelica::Ref::new(DAE::Exp::RCONST {
                    real: metamodelica::OrderedFloat(0.0_f64),
                }),
                outList,
            );
        }
    }
    outList
}

pub fn makeRealArrayOfZeros(mut inDimension: i32) -> metamodelica::Ref<DAE::Exp> {
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    let mut l: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
    l = makeListOfZeros(inDimension);
    outExp = makeRealArray(l);
    outExp
}

pub fn createZeroExpression(mut inType: metamodelica::Ref<DAE::Type>) -> Result<metamodelica::Ref<DAE::Exp>> {
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    outExp = (match &*inType {
        _ if (isIntegerOrReal(&inType)) => makeConstZero(&inType),
        DAE::Type::T_TUPLE { types: typeLst, .. } => {
            let mut e: metamodelica::Ref<DAE::Exp>;
            let mut expLst: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
            expLst = List::map(typeLst.clone(), &createZeroExpression)?;
            e = metamodelica::Ref::new(DAE::Exp::TUPLE { PR: expLst });
            e
        }
        DAE::Type::T_ARRAY { dims, .. } => {
            let mut e: metamodelica::Ref<DAE::Exp>;
            (e, _) = makeZeroExpression(metamodelica::AsArg::as_arg(&dims))?;
            e
        }
        DAE::Type::T_COMPLEX {
            varLst,
            complexClassType: ClassInf::State::RECORD { path },
            ..
        } => {
            let mut e: metamodelica::Ref<DAE::Exp>;
            let mut typeLst: metamodelica::List<metamodelica::Ref<DAE::Type>>;
            let mut expLst: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
            let mut varNames: metamodelica::List<ArcStr>;
            typeLst = ({
                let mut __acc: metamodelica::List<metamodelica::Ref<DAE::Type>> = metamodelica::nil();
                for mut v in (varLst.clone()).into_iter().cloned() {
                    let __x = v.ty.clone();
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            });
            expLst = List::map(typeLst, &createZeroExpression)?;
            varNames = List::map(
                varLst.clone(),
                &move |__a0: metamodelica::Ref<DAE::Var>| -> metamodelica::Result<_> {
                    ::std::result::Result::Ok(varName(&__a0))
                },
            )?;
            let true = (((varNames).len() as i32) == ((expLst).len() as i32)) else {
                return Err("pattern mismatch");
            };
            e = metamodelica::Ref::new(DAE::Exp::RECORD {
                path: path.clone(),
                exps: expLst,
                comp: varNames,
                ty: inType.clone(),
            });
            e
        }
        _ => return Err("fail"),
    });
    Ok(outExp)
}

pub fn makeZeroExpression(
    mut inDims: &metamodelica::List<metamodelica::Ref<DAE::Dimension>>,
) -> Result<(metamodelica::Ref<DAE::Exp>, metamodelica::Ref<DAE::Type>)> {
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    let mut outType: metamodelica::Ref<DAE::Type>;
    (outExp, outType) = (::match_deref::match_deref! { match inDims {
        Deref @ metamodelica::ListNode::Nil => {
            (metamodelica::Ref::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(0.0_f64) }), DAE::T_REAL_DEFAULT().clone())
        },
        Deref @ metamodelica::ListNode::Cons { head: d, tail: dims } => {
            let mut i: i32;
            let mut e: metamodelica::Ref<DAE::Exp>;
            let mut eLst: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
            let mut ty: metamodelica::Ref<DAE::Type>;
            let mut scalar: bool;
            i = dimensionSize(metamodelica::AsArg::as_arg(&d))?;
            (e, ty) = makeZeroExpression(dims)?;
            eLst = List::fill(e, i);
            scalar = (dims).is_empty();
            (metamodelica::Ref::new(DAE::Exp::ARRAY { ty: metamodelica::Ref::new(DAE::Type::T_ARRAY { ty: DAE::T_REAL_DEFAULT().clone(), dims: metamodelica::cons(d.clone(), dims.clone()) }), scalar: scalar, array: eLst }), metamodelica::Ref::new(DAE::Type::T_ARRAY { ty: ty, dims: list![d.clone()] }))
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outExp, outType))
}

pub fn makeOneExpression(
    mut inDims: &metamodelica::List<metamodelica::Ref<DAE::Dimension>>,
) -> Result<(metamodelica::Ref<DAE::Exp>, metamodelica::Ref<DAE::Type>)> {
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    let mut outType: metamodelica::Ref<DAE::Type>;
    (outExp, outType) = (::match_deref::match_deref! { match inDims {
        Deref @ metamodelica::ListNode::Nil => {
            (metamodelica::Ref::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(1.0_f64) }), DAE::T_REAL_DEFAULT().clone())
        },
        Deref @ metamodelica::ListNode::Cons { head: d, tail: dims } => {
            let mut i: i32;
            let mut e: metamodelica::Ref<DAE::Exp>;
            let mut eLst: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
            let mut ty: metamodelica::Ref<DAE::Type>;
            let mut scalar: bool;
            i = dimensionSize(metamodelica::AsArg::as_arg(&d))?;
            (e, ty) = makeOneExpression(dims)?;
            eLst = List::fill(e, i);
            scalar = (dims).is_empty();
            (metamodelica::Ref::new(DAE::Exp::ARRAY { ty: metamodelica::Ref::new(DAE::Type::T_ARRAY { ty: DAE::T_REAL_DEFAULT().clone(), dims: metamodelica::cons(d.clone(), dims.clone()) }), scalar: scalar, array: eLst }), metamodelica::Ref::new(DAE::Type::T_ARRAY { ty: ty, dims: list![d.clone()] }))
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outExp, outType))
}

pub fn listToArray(
    mut inList: &metamodelica::List<metamodelica::Ref<DAE::Exp>>,
    mut dims: &metamodelica::List<metamodelica::Ref<DAE::Dimension>>,
) -> Result<metamodelica::Ref<DAE::Exp>> {
    let mut oExp: metamodelica::Ref<DAE::Exp> = <metamodelica::Ref<DAE::Exp> as ::std::default::Default>::default();
    oExp = 'mc: {
        let __mc_input = (&**inList, &**dims);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Deref @ metamodelica::ListNode::Nil) => {
                    Error::addMessage(Error::INTERNAL_ERROR.clone(), list![literal!("Expression.listToArray called with empty dimension list.")])?;
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Nil, _) => {
                    Error::addMessage(Error::INTERNAL_ERROR.clone(), list![literal!("Expression.listToArray called with empty list.")])?;
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: exp, tail: _ }, _) => {
                    let mut ty: metamodelica::Ref<DAE::Type>;
                    let mut oExp: metamodelica::Ref<DAE::Exp> = oExp.clone();
                    ty = r#typeof(exp.clone())?;
                    oExp = listToArray2(inList, dims, &ty)?;
                    Ok((oExp.clone(), oExp.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            oExp = __wb0;
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(oExp)
}

fn listToArray2(
    mut inList: &metamodelica::List<metamodelica::Ref<DAE::Exp>>,
    mut iDims: &metamodelica::List<metamodelica::Ref<DAE::Dimension>>,
    mut inType: &metamodelica::Ref<DAE::Type>,
) -> Result<metamodelica::Ref<DAE::Exp>> {
    let mut oExp: metamodelica::Ref<DAE::Exp> = <metamodelica::Ref<DAE::Exp> as ::std::default::Default>::default();
    let () = (::match_deref::match_deref! { match iDims {
        Deref @ metamodelica::ListNode::Cons { head: d, tail: Deref @ metamodelica::ListNode::Nil } => {
            let mut i: i32;
            let mut is_scalar: bool;
            let mut ty: metamodelica::Ref<DAE::Type>;
            is_scalar = !(Types::isArray(inType));
            if dimensionKnown(metamodelica::AsArg::as_arg(&d)) {
                i = dimensionSize(metamodelica::AsArg::as_arg(&d))?;
                if i != ((inList).len() as i32) {
                    Error::addMessage(Error::INTERNAL_ERROR.clone(), list![literal!("Expression.listToArray2: Number of elements in the list does not match the dimension size.")])?;
                    return Err("fail");
                } else {
                    ty = liftArrayR(inType.clone(), metamodelica::Ref::new(DAE::Dimension::DIM_INTEGER { integer: ((inList).len() as i32) }));
                    oExp = metamodelica::Ref::new(DAE::Exp::ARRAY { ty: ty, scalar: is_scalar, array: inList.clone() });
                }
            } else {
                ty = liftArrayR(inType.clone(), d.clone());
                oExp = metamodelica::Ref::new(DAE::Exp::ARRAY { ty: ty, scalar: is_scalar, array: inList.clone() });
            }
            ()
        },
        Deref @ metamodelica::ListNode::Cons { head: _, tail: _ } => {
            let mut d: metamodelica::Ref<DAE::Dimension>;
            let mut explst: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
            let mut dims: metamodelica::List<metamodelica::Ref<DAE::Dimension>>;
            let mut ty: metamodelica::Ref<DAE::Type>;
            (d, dims) = List::splitLast(iDims.clone())?;
            explst = listToArray3(inList, d)?;
            ty = liftArrayR(inType.clone(), metamodelica::Ref::new(DAE::Dimension::DIM_INTEGER { integer: ((explst).len() as i32) }));
            oExp = listToArray2(&explst, &dims, &ty)?;
            ()
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok(oExp)
}

fn listToArray3(
    mut inList: &metamodelica::List<metamodelica::Ref<DAE::Exp>>,
    mut iDim: metamodelica::Ref<DAE::Dimension>,
) -> Result<metamodelica::List<metamodelica::Ref<DAE::Exp>>> {
    let mut oExps: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
    oExps = (::match_deref::match_deref! { match inList {
        Deref @ metamodelica::ListNode::Nil => {
            metamodelica::nil()
        },
        _ => {
            let mut d = iDim;
            let mut i: i32;
            let mut explst: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
            let mut restexps: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
            let mut restarr: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
            let mut arrexp: metamodelica::Ref<DAE::Exp>;
            i = dimensionSize(&d)?;
            if i > ((inList).len() as i32) {
                Error::addMessage(Error::INTERNAL_ERROR.clone(), list![literal!("Expression.listToArray3: Not enough elements left in list to fit dimension.")])?;
                return Err("fail");
            } else {
                (explst, restexps) = List::split(inList.clone(), i)?;
                arrexp = makeArrayFromList(explst)?;
                restarr = listToArray3(&restexps, d)?;
            }
            metamodelica::cons(arrexp, restarr)
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(oExps)
}

pub fn arrayFill(
    mut dims: metamodelica::List<metamodelica::Ref<DAE::Dimension>>,
    mut inExp: metamodelica::Ref<DAE::Exp>,
) -> Result<metamodelica::Ref<DAE::Exp>> {
    let mut oExp: metamodelica::Ref<DAE::Exp>;
    oExp = (::match_deref::match_deref! { match &(dims.clone()) {
        Deref @ metamodelica::ListNode::Nil => inExp,
        _ => {
            oExp = arrayFill2(dims, inExp)?;
            oExp
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(oExp)
}

fn arrayFill2(
    mut iDims: metamodelica::List<metamodelica::Ref<DAE::Dimension>>,
    mut inExp: metamodelica::Ref<DAE::Exp>,
) -> Result<metamodelica::Ref<DAE::Exp>> {
    '__tco: loop {
        ::match_deref::match_deref! { match &(iDims) {
            Deref @ metamodelica::ListNode::Cons { head: d, tail: Deref @ metamodelica::ListNode::Nil } => {
                let mut i: i32;
                let mut ty: Type;
                let mut expl: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                ty = r#typeof(inExp.clone())?;
                i = dimensionSize(metamodelica::AsArg::as_arg(&d))?;
                expl = List::fill(inExp, i);
                return Ok(metamodelica::Ref::new(DAE::Exp::ARRAY { ty: metamodelica::Ref::new(DAE::Type::T_ARRAY { ty: ty, dims: list![metamodelica::Ref::new(DAE::Dimension::DIM_INTEGER { integer: i })] }), scalar: true, array: expl }))
            },
            Deref @ metamodelica::ListNode::Cons { head: d, tail: dims } => {
                let mut arrexp: metamodelica::Ref<DAE::Exp>;
                arrexp = arrayFill2(list![d.clone()], inExp)?;
                { (iDims, inExp) = (dims.clone(), arrexp); continue '__tco; }
            },
            _ => return Err("match: no arm matched"),
        } }
    }
}

pub fn makeIndexSubscript(mut exp: metamodelica::Ref<DAE::Exp>) -> metamodelica::Ref<DAE::Subscript> {
    let mut subscript: metamodelica::Ref<DAE::Subscript>;
    subscript = metamodelica::Ref::new(DAE::Subscript::INDEX { exp: exp });
    subscript
}

pub fn makeVar(mut name: ArcStr, mut tp: metamodelica::Ref<DAE::Type>) -> metamodelica::Ref<DAE::Var> {
    let mut v: metamodelica::Ref<DAE::Var>;
    v = metamodelica::Ref::new(DAE::Var {
        name: name,
        attributes: DAE::dummyAttrVar().clone(),
        ty: tp,
        binding: openmodelica_frontend_types::DAE::Binding::interned_UNBOUND(),
        bind_from_outside: false,
        constOfForIteratorRange: None,
    });
    v
}

pub fn dimensionsAdd(
    mut dim1: &metamodelica::Ref<DAE::Dimension>,
    mut dim2: &metamodelica::Ref<DAE::Dimension>,
) -> metamodelica::Ref<DAE::Dimension> {
    let mut res: metamodelica::Ref<DAE::Dimension>;
    match '__try0: {
        res = intDimension(
            unwrap_break_err!(dimensionSize(dim1), '__try0) + unwrap_break_err!(dimensionSize(dim2), '__try0),
        );
        Ok::<_, &'static str>((res.clone(),))
    } {
        Ok((__try0_o0,)) => {
            res = __try0_o0;
        }
        Err(_) => {
            res = openmodelica_frontend_types::DAE::Dimension::interned_DIM_UNKNOWN();
        }
    }
    res
}

pub fn concatArrayType(
    mut arrayType1: &metamodelica::Ref<DAE::Type>,
    mut arrayType2: &metamodelica::Ref<DAE::Type>,
) -> Result<metamodelica::Ref<DAE::Type>> {
    let mut concatType: metamodelica::Ref<DAE::Type>;
    concatType = (::match_deref::match_deref! { match (arrayType1, arrayType2) {
        (Deref @ DAE::Type::T_ARRAY { ty: et, dims: Deref @ metamodelica::ListNode::Cons { head: dim1, tail: dims1 } }, Deref @ DAE::Type::T_ARRAY { dims: Deref @ metamodelica::ListNode::Cons { head: dim2, tail: _ }, .. }) => {
            let mut dim1 = (*dim1).clone();
            dim1 = dimensionsAdd(metamodelica::AsArg::as_arg(&dim1), metamodelica::AsArg::as_arg(&dim2));
            metamodelica::Ref::new(DAE::Type::T_ARRAY { ty: et.clone(), dims: metamodelica::cons(dim1.clone(), dims1.clone()) })
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok(concatType)
}

pub fn replaceExpTpl(
    mut inExp: metamodelica::Ref<DAE::Exp>,
    mut tpl: (metamodelica::Ref<DAE::Exp>, metamodelica::Ref<DAE::Exp>),
) -> Result<(
    metamodelica::Ref<DAE::Exp>,
    (metamodelica::Ref<DAE::Exp>, metamodelica::Ref<DAE::Exp>),
)> {
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    let mut outTpl: (metamodelica::Ref<DAE::Exp>, metamodelica::Ref<DAE::Exp>);
    (outExp, outTpl) = (::match_deref::match_deref! { match &(tpl.clone()) {
        (s, t) => {
            let mut e = inExp;
            let mut e1: metamodelica::Ref<DAE::Exp>;
            (e1, _) = replaceExp(e, s.clone(), t.clone())?;
            (e1, tpl)
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outExp, outTpl))
}

pub fn replaceExp(
    mut inExp: metamodelica::Ref<DAE::Exp>,
    mut inSourceExp: metamodelica::Ref<DAE::Exp>,
    mut inTargetExp: metamodelica::Ref<DAE::Exp>,
) -> Result<(metamodelica::Ref<DAE::Exp>, i32)> {
    let mut exp: metamodelica::Ref<DAE::Exp>;
    let mut i: i32;
    let (__pa0, (_, _, __pa1)) = traverseExpTopDown(inExp, &replaceExpWork, (inSourceExp, inTargetExp, 0))?;
    exp = metamodelica::Own::own(__pa0);
    i = metamodelica::Own::own(__pa1);
    Ok((exp, i))
}

fn replaceExpWork(
    mut inExp: metamodelica::Ref<DAE::Exp>,
    mut inTpl: (metamodelica::Ref<DAE::Exp>, metamodelica::Ref<DAE::Exp>, i32),
) -> Result<(
    metamodelica::Ref<DAE::Exp>,
    bool,
    (metamodelica::Ref<DAE::Exp>, metamodelica::Ref<DAE::Exp>, i32),
)> {
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    let mut cont: bool;
    let mut otpl: (metamodelica::Ref<DAE::Exp>, metamodelica::Ref<DAE::Exp>, i32);
    (outExp, cont, otpl) = (::match_deref::match_deref! { match &(inTpl.clone()) {
        (source, target, c) if (ExpressionBasics::expEqual(&inExp, source.clone())?) => {
            (target.clone(), false, (source.clone(), target.clone(), c.clone() + 1))
        },
        _ => {
            (inExp.clone(), true, inTpl)
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outExp, cont, otpl))
}

pub fn replaceExpNoEvent(
    mut inExp: metamodelica::Ref<DAE::Exp>,
    mut inSourceExp: metamodelica::Ref<DAE::Exp>,
    mut inTargetExp: metamodelica::Ref<DAE::Exp>,
) -> Result<(metamodelica::Ref<DAE::Exp>, i32)> {
    let mut exp: metamodelica::Ref<DAE::Exp>;
    let mut i: i32;
    let (__pa0, (_, _, __pa1)) = traverseExpTopDown(inExp, &replaceExpWorkNoEvent, (inSourceExp, inTargetExp, 0))?;
    exp = metamodelica::Own::own(__pa0);
    i = metamodelica::Own::own(__pa1);
    Ok((exp, i))
}

fn replaceExpWorkNoEvent(
    mut inExp: metamodelica::Ref<DAE::Exp>,
    mut inTpl: (metamodelica::Ref<DAE::Exp>, metamodelica::Ref<DAE::Exp>, i32),
) -> Result<(
    metamodelica::Ref<DAE::Exp>,
    bool,
    (metamodelica::Ref<DAE::Exp>, metamodelica::Ref<DAE::Exp>, i32),
)> {
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    let mut cont: bool;
    let mut otpl: (metamodelica::Ref<DAE::Exp>, metamodelica::Ref<DAE::Exp>, i32);
    (outExp, cont, otpl) = (::match_deref::match_deref! { match &((inExp.clone(), inTpl.clone())) {
        (_, (source, target, c)) if (ExpressionBasics::expEqual(&inExp, source.clone())?) => {
            (target.clone(), false, (source.clone(), target.clone(), c.clone() + 1))
        },
        (Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "noEvent" }, .. }, _) => {
            (inExp.clone(), false, inTpl)
        },
        (Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "smooth" }, .. }, _) => {
            (inExp.clone(), false, inTpl)
        },
        _ => {
            (inExp.clone(), true, inTpl)
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outExp, cont, otpl))
}

pub(crate) fn expressionCollector(
    mut exp: metamodelica::Ref<DAE::Exp>,
    mut acc: metamodelica::List<metamodelica::Ref<DAE::Exp>>,
) -> (
    metamodelica::Ref<DAE::Exp>,
    metamodelica::List<metamodelica::Ref<DAE::Exp>>,
) {
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    let mut outExps: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
    outExp = exp.clone();
    outExps = metamodelica::cons(exp, acc);
    (outExp, outExps)
}

pub fn replaceCrefBottomUp(
    mut inExp: metamodelica::Ref<DAE::Exp>,
    mut inSourceExp: metamodelica::Ref<DAE::ComponentRef>,
    mut inTargetExp: metamodelica::Ref<DAE::Exp>,
) -> Result<metamodelica::Ref<DAE::Exp>> {
    let mut exp: metamodelica::Ref<DAE::Exp>;
    (exp, _) = traverseExpBottomUp(inExp, &replaceCref, (inSourceExp, inTargetExp))?;
    Ok(exp)
}

pub fn replaceCref(
    mut inExp: metamodelica::Ref<DAE::Exp>,
    mut inTpl: (metamodelica::Ref<DAE::ComponentRef>, metamodelica::Ref<DAE::Exp>),
) -> Result<(
    metamodelica::Ref<DAE::Exp>,
    (metamodelica::Ref<DAE::ComponentRef>, metamodelica::Ref<DAE::Exp>),
)> {
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    let mut otpl: (metamodelica::Ref<DAE::ComponentRef>, metamodelica::Ref<DAE::Exp>);
    (outExp, otpl) = (::match_deref::match_deref! { match &((inExp.clone(), inTpl.clone())) {
        (Deref @ DAE::Exp::CREF { componentRef: cr, .. }, (cr1, target)) if (ComponentReferenceBasics::crefEqualNoStringCompare(metamodelica::AsArg::as_arg(&cr), metamodelica::AsArg::as_arg(&cr1))?) => {
            (target.clone(), inTpl)
        },
        _ => {
            (inExp, inTpl)
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outExp, otpl))
}

pub fn containsInitialCall(mut condition: &metamodelica::Ref<DAE::Exp>) -> Result<bool> {
    let mut res: bool;
    res = (::match_deref::match_deref! { match condition {
        Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "initial" }, .. } => {
            true
        },
        Deref @ DAE::Exp::ARRAY { array, .. } => {
            List::any(array, &move |__a0: metamodelica::Ref<DAE::Exp>| containsInitialCall(&__a0))?
        },
        _ => {
            false
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(res)
}

/* **************************************************/
/* traverse DAE.Exp */
/* **************************************************/
pub fn traverseExpBottomUp<T: Clone + 'static + metamodelica::gc::MMTrace>(
    mut inExp: metamodelica::Ref<DAE::Exp>,
    mut inFunc: &dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>, T) -> Result<(metamodelica::Ref<DAE::Exp>, T)>,
    mut inExtArg: T,
) -> Result<(metamodelica::Ref<DAE::Exp>, T)> {
    pub type FuncExpType<T: Clone + 'static> = std::sync::Arc<
        dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>, T) -> Result<(metamodelica::Ref<DAE::Exp>, T)> + 'static,
    >;

    let mut outExp: metamodelica::Ref<DAE::Exp>;
    let mut outExtArg: T;
    (outExp, outExtArg) = (::match_deref::match_deref! { match &(inExp.clone()) {
        Deref @ DAE::Exp::EMPTY { .. } => {
            let mut e: metamodelica::Ref<DAE::Exp>;
            let mut ext_arg: T;
            (e, ext_arg) = inFunc(inExp, inExtArg)?;
            (e, ext_arg)
        },
        Deref @ DAE::Exp::ICONST { .. } => {
            let mut e: metamodelica::Ref<DAE::Exp>;
            let mut ext_arg: T;
            (e, ext_arg) = inFunc(inExp, inExtArg)?;
            (e, ext_arg)
        },
        Deref @ DAE::Exp::RCONST { .. } => {
            let mut e: metamodelica::Ref<DAE::Exp>;
            let mut ext_arg: T;
            (e, ext_arg) = inFunc(inExp, inExtArg)?;
            (e, ext_arg)
        },
        Deref @ DAE::Exp::SCONST { .. } => {
            let mut e: metamodelica::Ref<DAE::Exp>;
            let mut ext_arg: T;
            (e, ext_arg) = inFunc(inExp, inExtArg)?;
            (e, ext_arg)
        },
        Deref @ DAE::Exp::BCONST { .. } => {
            let mut e: metamodelica::Ref<DAE::Exp>;
            let mut ext_arg: T;
            (e, ext_arg) = inFunc(inExp, inExtArg)?;
            (e, ext_arg)
        },
        Deref @ DAE::Exp::CLKCONST { clk } => {
            let mut e: metamodelica::Ref<DAE::Exp>;
            let mut ext_arg: T;
            let mut clk1: metamodelica::Ref<DAE::ClockKind>;
            (clk1, ext_arg) = traverseExpClk(clk.clone(), inFunc, inExtArg)?;
            e = if (referenceEq(&*(&*clk1),&*(clk.clone()))) {inExp} else {metamodelica::Ref::new(DAE::Exp::CLKCONST { clk: clk1 })};
            (e, ext_arg) = inFunc(e, ext_arg)?;
            (e, ext_arg)
        },
        Deref @ DAE::Exp::ENUM_LITERAL { .. } => {
            let mut e: metamodelica::Ref<DAE::Exp>;
            let mut ext_arg: T;
            (e, ext_arg) = inFunc(inExp, inExtArg)?;
            (e, ext_arg)
        },
        Deref @ DAE::Exp::CREF { componentRef: cr, ty: tp } => {
            let mut e: metamodelica::Ref<DAE::Exp>;
            let mut ext_arg: T;
            let mut cr_1: metamodelica::Ref<DAE::ComponentRef>;
            (cr_1, ext_arg) = traverseExpCref(metamodelica::AsArg::as_arg(&cr), inFunc, inExtArg)?;
            e = if (referenceEq(&*(cr.clone()),&*(&*cr_1))) {inExp} else {metamodelica::Ref::new(DAE::Exp::CREF { componentRef: cr_1, ty: tp.clone() })};
            (e, ext_arg) = inFunc(e, ext_arg)?;
            (e, ext_arg)
        },
        Deref @ DAE::Exp::UNARY { operator: op, exp: e1 } => {
            let mut e1_1: metamodelica::Ref<DAE::Exp>;
            let mut e: metamodelica::Ref<DAE::Exp>;
            let mut ext_arg: T;
            (e1_1, ext_arg) = traverseExpBottomUp(e1.clone(), inFunc, inExtArg)?;
            e = if (referenceEq(&*(e1.clone()),&*(&*e1_1))) {inExp} else {metamodelica::Ref::new(DAE::Exp::UNARY { operator: op.clone(), exp: e1_1 })};
            (e, ext_arg) = inFunc(e, ext_arg)?;
            (e, ext_arg)
        },
        Deref @ DAE::Exp::BINARY { exp1: e1, operator: op, exp2: e2 } => {
            let mut e1_1: metamodelica::Ref<DAE::Exp>;
            let mut e: metamodelica::Ref<DAE::Exp>;
            let mut e2_1: metamodelica::Ref<DAE::Exp>;
            let mut ext_arg: T;
            (e1_1, ext_arg) = traverseExpBottomUp(e1.clone(), inFunc, inExtArg)?;
            (e2_1, ext_arg) = traverseExpBottomUp(e2.clone(), inFunc, ext_arg)?;
            e = if (referenceEq(&*(e1.clone()),&*(&*e1_1)) && referenceEq(&*(e2.clone()),&*(&*e2_1))) {inExp} else {metamodelica::Ref::new(DAE::Exp::BINARY { exp1: e1_1, operator: op.clone(), exp2: e2_1 })};
            (e, ext_arg) = inFunc(e, ext_arg)?;
            (e, ext_arg)
        },
        Deref @ DAE::Exp::LUNARY { operator: op, exp: e1 } => {
            let mut e1_1: metamodelica::Ref<DAE::Exp>;
            let mut e: metamodelica::Ref<DAE::Exp>;
            let mut ext_arg: T;
            (e1_1, ext_arg) = traverseExpBottomUp(e1.clone(), inFunc, inExtArg)?;
            e = if (referenceEq(&*(e1.clone()),&*(&*e1_1))) {inExp} else {metamodelica::Ref::new(DAE::Exp::LUNARY { operator: op.clone(), exp: e1_1 })};
            (e, ext_arg) = inFunc(e, ext_arg)?;
            (e, ext_arg)
        },
        Deref @ DAE::Exp::LBINARY { exp1: e1, operator: op, exp2: e2 } => {
            let mut e1_1: metamodelica::Ref<DAE::Exp>;
            let mut e: metamodelica::Ref<DAE::Exp>;
            let mut e2_1: metamodelica::Ref<DAE::Exp>;
            let mut ext_arg: T;
            (e1_1, ext_arg) = traverseExpBottomUp(e1.clone(), inFunc, inExtArg)?;
            (e2_1, ext_arg) = traverseExpBottomUp(e2.clone(), inFunc, ext_arg)?;
            e = if (referenceEq(&*(e1.clone()),&*(&*e1_1)) && referenceEq(&*(e2.clone()),&*(&*e2_1))) {inExp} else {metamodelica::Ref::new(DAE::Exp::LBINARY { exp1: e1_1, operator: op.clone(), exp2: e2_1 })};
            (e, ext_arg) = inFunc(e, ext_arg)?;
            (e, ext_arg)
        },
        Deref @ DAE::Exp::RELATION { exp1: e1, operator: op, exp2: e2, index: index_, optionExpisASUB: isExpisASUB } => {
            let mut e1_1: metamodelica::Ref<DAE::Exp>;
            let mut e: metamodelica::Ref<DAE::Exp>;
            let mut e2_1: metamodelica::Ref<DAE::Exp>;
            let mut ext_arg: T;
            (e1_1, ext_arg) = traverseExpBottomUp(e1.clone(), inFunc, inExtArg)?;
            (e2_1, ext_arg) = traverseExpBottomUp(e2.clone(), inFunc, ext_arg)?;
            e = if (referenceEq(&*(e1.clone()),&*(&*e1_1)) && referenceEq(&*(e2.clone()),&*(&*e2_1))) {inExp} else {metamodelica::Ref::new(DAE::Exp::RELATION { exp1: e1_1, operator: op.clone(), exp2: e2_1, index: index_.clone(), optionExpisASUB: isExpisASUB.clone() })};
            (e, ext_arg) = inFunc(e, ext_arg)?;
            (e, ext_arg)
        },
        Deref @ DAE::Exp::IFEXP { expCond: e1, expThen: e2, expElse: e3 } => {
            let mut e1_1: metamodelica::Ref<DAE::Exp>;
            let mut e: metamodelica::Ref<DAE::Exp>;
            let mut e2_1: metamodelica::Ref<DAE::Exp>;
            let mut e3_1: metamodelica::Ref<DAE::Exp>;
            let mut ext_arg: T;
            (e1_1, ext_arg) = traverseExpBottomUp(e1.clone(), inFunc, inExtArg)?;
            (e2_1, ext_arg) = traverseExpBottomUp(e2.clone(), inFunc, ext_arg)?;
            (e3_1, ext_arg) = traverseExpBottomUp(e3.clone(), inFunc, ext_arg)?;
            e = if (referenceEq(&*(e1.clone()),&*(&*e1_1)) && referenceEq(&*(e2.clone()),&*(&*e2_1)) && referenceEq(&*(e3.clone()),&*(&*e3_1))) {inExp} else {metamodelica::Ref::new(DAE::Exp::IFEXP { expCond: e1_1, expThen: e2_1, expElse: e3_1 })};
            (e, ext_arg) = inFunc(e, ext_arg)?;
            (e, ext_arg)
        },
        Deref @ DAE::Exp::CALL { path: r#fn, expLst: expl, attr } => {
            let mut e: metamodelica::Ref<DAE::Exp>;
            let mut ext_arg: T;
            let mut expl_1: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
            (expl_1, ext_arg) = traverseExpList(expl.clone(), inFunc, inExtArg)?;
            e = if (metamodelica::ReferenceEq::reference_eq(&(expl.clone()), &(expl_1))) {inExp} else {metamodelica::Ref::new(DAE::Exp::CALL { path: r#fn.clone(), expLst: expl_1, attr: attr.clone() })};
            (e, ext_arg) = inFunc(e, ext_arg)?;
            (e, ext_arg)
        },
        Deref @ DAE::Exp::RECORD { path: r#fn, exps: expl, comp: fieldNames, ty: tp } => {
            let mut e: metamodelica::Ref<DAE::Exp>;
            let mut ext_arg: T;
            let mut expl_1: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
            (expl_1, ext_arg) = traverseExpList(expl.clone(), inFunc, inExtArg)?;
            e = if (metamodelica::ReferenceEq::reference_eq(&(expl.clone()), &(expl_1))) {inExp} else {metamodelica::Ref::new(DAE::Exp::RECORD { path: r#fn.clone(), exps: expl_1, comp: fieldNames.clone(), ty: tp.clone() })};
            (e, ext_arg) = inFunc(e, ext_arg)?;
            (e, ext_arg)
        },
        Deref @ DAE::Exp::PARTEVALFUNCTION { path: r#fn, expList: expl, ty: tp, origType: t } => {
            let mut e: metamodelica::Ref<DAE::Exp>;
            let mut ext_arg: T;
            let mut expl_1: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
            (expl_1, ext_arg) = traverseExpList(expl.clone(), inFunc, inExtArg)?;
            e = if (metamodelica::ReferenceEq::reference_eq(&(expl.clone()), &(expl_1))) {inExp} else {metamodelica::Ref::new(DAE::Exp::PARTEVALFUNCTION { path: r#fn.clone(), expList: expl_1, ty: tp.clone(), origType: t.clone() })};
            (e, ext_arg) = inFunc(e, ext_arg)?;
            (e, ext_arg)
        },
        Deref @ DAE::Exp::ARRAY { ty: tp, scalar, array: expl } => {
            let mut e: metamodelica::Ref<DAE::Exp>;
            let mut ext_arg: T;
            let mut expl_1: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
            (expl_1, ext_arg) = traverseExpList(expl.clone(), inFunc, inExtArg)?;
            e = if (metamodelica::ReferenceEq::reference_eq(&(expl.clone()), &(expl_1))) {inExp} else {metamodelica::Ref::new(DAE::Exp::ARRAY { ty: tp.clone(), scalar: scalar.clone(), array: expl_1 })};
            (e, ext_arg) = inFunc(e, ext_arg)?;
            (e, ext_arg)
        },
        Deref @ DAE::Exp::MATRIX { ty: tp, integer: dim, matrix: lstexpl } => {
            let mut e: metamodelica::Ref<DAE::Exp>;
            let mut ext_arg: T;
            let mut lstexpl_1: metamodelica::List<metamodelica::List<metamodelica::Ref<DAE::Exp>>>;
            (lstexpl_1, ext_arg) = traverseExpMatrix(lstexpl.clone(), inFunc, inExtArg)?;
            e = if (metamodelica::ReferenceEq::reference_eq(&(lstexpl.clone()), &(lstexpl_1))) {inExp} else {metamodelica::Ref::new(DAE::Exp::MATRIX { ty: tp.clone(), integer: dim.clone(), matrix: lstexpl_1 })};
            (e, ext_arg) = inFunc(e, ext_arg)?;
            (e, ext_arg)
        },
        Deref @ DAE::Exp::RANGE { ty: tp, start: e1, step: None, stop: e2 } => {
            let mut e1_1: metamodelica::Ref<DAE::Exp>;
            let mut e: metamodelica::Ref<DAE::Exp>;
            let mut e2_1: metamodelica::Ref<DAE::Exp>;
            let mut ext_arg: T;
            (e1_1, ext_arg) = traverseExpBottomUp(e1.clone(), inFunc, inExtArg)?;
            (e2_1, ext_arg) = traverseExpBottomUp(e2.clone(), inFunc, ext_arg)?;
            e = if (referenceEq(&*(e1.clone()),&*(&*e1_1)) && referenceEq(&*(e2.clone()),&*(&*e2_1))) {inExp} else {metamodelica::Ref::new(DAE::Exp::RANGE { ty: tp.clone(), start: e1_1, step: None, stop: e2_1 })};
            (e, ext_arg) = inFunc(e, ext_arg)?;
            (e, ext_arg)
        },
        Deref @ DAE::Exp::RANGE { ty: tp, start: e1, step: Some(e2), stop: e3 } => {
            let mut e1_1: metamodelica::Ref<DAE::Exp>;
            let mut e: metamodelica::Ref<DAE::Exp>;
            let mut e2_1: metamodelica::Ref<DAE::Exp>;
            let mut e3_1: metamodelica::Ref<DAE::Exp>;
            let mut ext_arg: T;
            (e1_1, ext_arg) = traverseExpBottomUp(e1.clone(), inFunc, inExtArg)?;
            (e2_1, ext_arg) = traverseExpBottomUp(e2.clone(), inFunc, ext_arg)?;
            (e3_1, ext_arg) = traverseExpBottomUp(e3.clone(), inFunc, ext_arg)?;
            e = if (referenceEq(&*(e1.clone()),&*(&*e1_1)) && referenceEq(&*(e2.clone()),&*(&*e2_1)) && referenceEq(&*(e3.clone()),&*(&*e3_1))) {inExp} else {metamodelica::Ref::new(DAE::Exp::RANGE { ty: tp.clone(), start: e1_1, step: Some(e2_1), stop: e3_1 })};
            (e, ext_arg) = inFunc(e, ext_arg)?;
            (e, ext_arg)
        },
        Deref @ DAE::Exp::TUPLE { PR: expl } => {
            let mut e: metamodelica::Ref<DAE::Exp>;
            let mut ext_arg: T;
            let mut expl_1: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
            (expl_1, ext_arg) = traverseExpList(expl.clone(), inFunc, inExtArg)?;
            e = if (metamodelica::ReferenceEq::reference_eq(&(expl.clone()), &(expl_1))) {inExp} else {metamodelica::Ref::new(DAE::Exp::TUPLE { PR: expl_1 })};
            (e, ext_arg) = inFunc(e, ext_arg)?;
            (e, ext_arg)
        },
        Deref @ DAE::Exp::CAST { ty: tp, exp: e1 } => {
            let mut e1_1: metamodelica::Ref<DAE::Exp>;
            let mut e: metamodelica::Ref<DAE::Exp>;
            let mut ext_arg: T;
            (e1_1, ext_arg) = traverseExpBottomUp(e1.clone(), inFunc, inExtArg)?;
            e = if (referenceEq(&*(e1.clone()),&*(&*e1_1))) {inExp} else {metamodelica::Ref::new(DAE::Exp::CAST { ty: tp.clone(), exp: e1_1 })};
            (e, ext_arg) = inFunc(e, ext_arg)?;
            (e, ext_arg)
        },
        Deref @ DAE::Exp::ASUB { exp: e1, sub: subs } => {
            let mut e1_1: metamodelica::Ref<DAE::Exp>;
            let mut e: metamodelica::Ref<DAE::Exp>;
            let mut ext_arg: T;
            let mut expl_1: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
            let mut expl: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
            expl = ({
        let mut __acc: metamodelica::List<metamodelica::Ref<DAE::Exp>> = metamodelica::nil();
        for mut sub in (subs.clone()).into_iter().cloned() {
            let __x = getSubscriptExp(&(sub.clone()))?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
            (e1_1, ext_arg) = traverseExpBottomUp(e1.clone(), inFunc, inExtArg)?;
            (expl_1, ext_arg) = traverseExpList(expl.clone(), inFunc, ext_arg)?;
            e = if (referenceEq(&*(e1.clone()),&*(&*e1_1)) && metamodelica::ReferenceEq::reference_eq(&(expl), &(expl_1))) {inExp} else {makeASUB(e1_1, expl_1)?};
            (e, ext_arg) = inFunc(e, ext_arg)?;
            (e, ext_arg)
        },
        Deref @ DAE::Exp::TSUB { exp: e1, ix: i, ty: tp } => {
            let mut e1_1: metamodelica::Ref<DAE::Exp>;
            let mut e: metamodelica::Ref<DAE::Exp>;
            let mut ext_arg: T;
            (e1_1, ext_arg) = traverseExpBottomUp(e1.clone(), inFunc, inExtArg)?;
            e = if (referenceEq(&*(e1.clone()),&*(&*e1_1))) {inExp} else {metamodelica::Ref::new(DAE::Exp::TSUB { exp: e1_1, ix: i.clone(), ty: tp.clone() })};
            (e, ext_arg) = inFunc(e, ext_arg)?;
            (e, ext_arg)
        },
        e1 @ Deref @ DAE::Exp::RSUB { .. } => {
            let mut e1_1: metamodelica::Ref<DAE::Exp>;
            let mut ext_arg: T;
            let mut e1 = (*e1).clone();
            (e1_1, ext_arg) = traverseExpBottomUp(var_field!((*e1).exp, DAE::Exp::RSUB).clone(), inFunc, inExtArg)?;
            if !(referenceEq(&*(var_field!((*e1).exp, DAE::Exp::RSUB).clone()),&*(&*e1_1))) {
                assign_variant_field!(e1 => DAE::Exp::RSUB; exp = e1_1);
            }
            (e1, ext_arg) = inFunc(e1.clone(), ext_arg)?;
            (e1.clone(), ext_arg)
        },
        Deref @ DAE::Exp::SIZE { exp: e1, sz: None } => {
            let mut e1_1: metamodelica::Ref<DAE::Exp>;
            let mut e: metamodelica::Ref<DAE::Exp>;
            let mut ext_arg: T;
            (e1_1, ext_arg) = traverseExpBottomUp(e1.clone(), inFunc, inExtArg)?;
            e = if (referenceEq(&*(e1.clone()),&*(&*e1_1))) {inExp} else {metamodelica::Ref::new(DAE::Exp::SIZE { exp: e1_1, sz: None })};
            (e, ext_arg) = inFunc(e, ext_arg)?;
            (e, ext_arg)
        },
        Deref @ DAE::Exp::SIZE { exp: e1, sz: Some(e2) } => {
            let mut e1_1: metamodelica::Ref<DAE::Exp>;
            let mut e: metamodelica::Ref<DAE::Exp>;
            let mut e2_1: metamodelica::Ref<DAE::Exp>;
            let mut ext_arg: T;
            (e1_1, ext_arg) = traverseExpBottomUp(e1.clone(), inFunc, inExtArg)?;
            (e2_1, ext_arg) = traverseExpBottomUp(e2.clone(), inFunc, ext_arg)?;
            e = if (referenceEq(&*(e1.clone()),&*(&*e1_1)) && referenceEq(&*(e2.clone()),&*(&*e2_1))) {inExp} else {metamodelica::Ref::new(DAE::Exp::SIZE { exp: e1_1, sz: Some(e2_1) })};
            (e, ext_arg) = inFunc(e, ext_arg)?;
            (e, ext_arg)
        },
        Deref @ DAE::Exp::REDUCTION { reductionInfo, expr: e1, iterators: riters } => {
            let mut e1_1: metamodelica::Ref<DAE::Exp>;
            let mut e: metamodelica::Ref<DAE::Exp>;
            let mut ext_arg: T;
            let mut riters_1: metamodelica::List<metamodelica::Ref<DAE::ReductionIterator>>;
            (e1_1, ext_arg) = traverseExpBottomUp(e1.clone(), inFunc, inExtArg)?;
            (riters_1, ext_arg) = traverseReductionIterators(riters.clone(), inFunc, ext_arg)?;
            e = if (referenceEq(&*(e1.clone()),&*(&*e1_1)) && metamodelica::ReferenceEq::reference_eq(&(riters.clone()), &(riters_1))) {inExp} else {metamodelica::Ref::new(DAE::Exp::REDUCTION { reductionInfo: reductionInfo.clone(), expr: e1_1, iterators: riters_1 })};
            (e, ext_arg) = inFunc(e, ext_arg)?;
            (e, ext_arg)
        },
        Deref @ DAE::Exp::CONS { car: e1, cdr: e2 } => {
            let mut e1_1: metamodelica::Ref<DAE::Exp>;
            let mut e: metamodelica::Ref<DAE::Exp>;
            let mut e2_1: metamodelica::Ref<DAE::Exp>;
            let mut ext_arg: T;
            (e1_1, ext_arg) = traverseExpBottomUp(e1.clone(), inFunc, inExtArg)?;
            (e2_1, ext_arg) = traverseExpBottomUp(e2.clone(), inFunc, ext_arg)?;
            e = if (referenceEq(&*(e1.clone()),&*(&*e1_1)) && referenceEq(&*(e2.clone()),&*(&*e2_1))) {inExp} else {metamodelica::Ref::new(DAE::Exp::CONS { car: e1_1, cdr: e2_1 })};
            (e, ext_arg) = inFunc(e, ext_arg)?;
            (e, ext_arg)
        },
        Deref @ DAE::Exp::LIST { valList: expl } => {
            let mut e: metamodelica::Ref<DAE::Exp>;
            let mut ext_arg: T;
            let mut expl_1: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
            (expl_1, ext_arg) = traverseExpList(expl.clone(), inFunc, inExtArg)?;
            e = if (metamodelica::ReferenceEq::reference_eq(&(expl.clone()), &(expl_1))) {inExp} else {metamodelica::Ref::new(DAE::Exp::LIST { valList: expl_1 })};
            (e, ext_arg) = inFunc(e, ext_arg)?;
            (e, ext_arg)
        },
        Deref @ DAE::Exp::META_TUPLE { listExp: expl } => {
            let mut e: metamodelica::Ref<DAE::Exp>;
            let mut ext_arg: T;
            let mut expl_1: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
            (expl_1, ext_arg) = traverseExpList(expl.clone(), inFunc, inExtArg)?;
            e = if (metamodelica::ReferenceEq::reference_eq(&(expl.clone()), &(expl_1))) {inExp} else {metamodelica::Ref::new(DAE::Exp::META_TUPLE { listExp: expl_1 })};
            (e, ext_arg) = inFunc(e, ext_arg)?;
            (e, ext_arg)
        },
        Deref @ DAE::Exp::META_OPTION { exp: None } => {
            let mut e: metamodelica::Ref<DAE::Exp>;
            let mut ext_arg: T;
            (e, ext_arg) = inFunc(inExp, inExtArg)?;
            (e, ext_arg)
        },
        Deref @ DAE::Exp::META_OPTION { exp: Some(e1) } => {
            let mut e1_1: metamodelica::Ref<DAE::Exp>;
            let mut e: metamodelica::Ref<DAE::Exp>;
            let mut ext_arg: T;
            (e1_1, ext_arg) = traverseExpBottomUp(e1.clone(), inFunc, inExtArg)?;
            e = if (referenceEq(&*(e1.clone()),&*(&*e1_1))) {inExp} else {metamodelica::Ref::new(DAE::Exp::META_OPTION { exp: Some(e1_1) })};
            (e, ext_arg) = inFunc(e, ext_arg)?;
            (e, ext_arg)
        },
        Deref @ DAE::Exp::BOX { exp: e1 } => {
            let mut e1_1: metamodelica::Ref<DAE::Exp>;
            let mut e: metamodelica::Ref<DAE::Exp>;
            let mut ext_arg: T;
            (e1_1, ext_arg) = traverseExpBottomUp(e1.clone(), inFunc, inExtArg)?;
            e = if (referenceEq(&*(e1.clone()),&*(&*e1_1))) {inExp} else {metamodelica::Ref::new(DAE::Exp::BOX { exp: e1_1 })};
            (e, ext_arg) = inFunc(e, ext_arg)?;
            (e, ext_arg)
        },
        Deref @ DAE::Exp::UNBOX { exp: e1, ty: tp } => {
            let mut e1_1: metamodelica::Ref<DAE::Exp>;
            let mut e: metamodelica::Ref<DAE::Exp>;
            let mut ext_arg: T;
            (e1_1, ext_arg) = traverseExpBottomUp(e1.clone(), inFunc, inExtArg)?;
            e = if (referenceEq(&*(e1.clone()),&*(&*e1_1))) {inExp} else {metamodelica::Ref::new(DAE::Exp::UNBOX { exp: e1_1, ty: tp.clone() })};
            (e, ext_arg) = inFunc(e, ext_arg)?;
            (e, ext_arg)
        },
        Deref @ DAE::Exp::METARECORDCALL { path: r#fn, args: expl, fieldNames, index: i, typeVars } => {
            let mut e: metamodelica::Ref<DAE::Exp>;
            let mut ext_arg: T;
            let mut expl_1: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
            (expl_1, ext_arg) = traverseExpList(expl.clone(), inFunc, inExtArg)?;
            e = if (metamodelica::ReferenceEq::reference_eq(&(expl.clone()), &(expl_1))) {inExp} else {metamodelica::Ref::new(DAE::Exp::METARECORDCALL { path: r#fn.clone(), args: expl_1, fieldNames: fieldNames.clone(), index: i.clone(), typeVars: typeVars.clone() })};
            (e, ext_arg) = inFunc(e, ext_arg)?;
            (e, ext_arg)
        },
        Deref @ DAE::Exp::MATCHEXPRESSION { matchType: matchTy, inputs: expl, aliases, localDecls, cases, et: tp } => {
            let mut e: metamodelica::Ref<DAE::Exp>;
            let mut ext_arg: T;
            let mut expl_1: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
            let mut cases_1: metamodelica::List<metamodelica::Ref<DAE::MatchCase>>;
            (expl_1, ext_arg) = traverseExpList(expl.clone(), inFunc, inExtArg)?;
            (cases_1, ext_arg) = traverseMatchCases(cases.clone(), inFunc, ext_arg);
            e = if (metamodelica::ReferenceEq::reference_eq(&(expl.clone()), &(expl_1)) && metamodelica::ReferenceEq::reference_eq(&(cases.clone()), &(cases_1))) {inExp} else {metamodelica::Ref::new(DAE::Exp::MATCHEXPRESSION { matchType: matchTy.clone(), inputs: expl_1, aliases: aliases.clone(), localDecls: localDecls.clone(), cases: cases_1, et: tp.clone() })};
            (e, ext_arg) = inFunc(e, ext_arg)?;
            (e, ext_arg)
        },
        Deref @ DAE::Exp::SHARED_LITERAL { .. } => {
            let mut e: metamodelica::Ref<DAE::Exp>;
            let mut ext_arg: T;
            (e, ext_arg) = inFunc(inExp, inExtArg)?;
            (e, ext_arg)
        },
        Deref @ DAE::Exp::PATTERN { .. } => {
            let mut e: metamodelica::Ref<DAE::Exp>;
            let mut ext_arg: T;
            (e, ext_arg) = inFunc(inExp, inExtArg)?;
            (e, ext_arg)
        },
        Deref @ DAE::Exp::CODE { .. } => {
            (inExp, inExtArg)
        },
        _ => {
            let mut r#str: ArcStr;
            r#str = printExpStr(inExp)?;
            r#str = { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Expression.traverseExpBottomUp or one of the user-defined functions using it is not implemented correctly: ")); __mm_s.push_str(&*r#str); ArcStr::from(__mm_s) };
            Error::addInternalError(r#str, metamodelica::sourceInfo!("FrontEnd/Expression.mo"))?;
            return Err("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outExp, outExtArg))
}

pub fn traverseExpDummy(
    mut inExp: metamodelica::Ref<DAE::Exp>,
    mut func: Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<metamodelica::Ref<DAE::Exp>> + 'static>,
) -> Result<metamodelica::Ref<DAE::Exp>> {
    pub type FuncExpType = std::sync::Arc<
        dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<metamodelica::Ref<DAE::Exp>> + 'static,
    >;

    let mut outExp: metamodelica::Ref<DAE::Exp>;
    (outExp, _) = traverseExpBottomUp(inExp, &traverseExpDummyHelper, func.clone())?;
    Ok(outExp)
}

pub(crate) fn traverseExpDummyHelper(
    mut inExp: metamodelica::Ref<DAE::Exp>,
    mut func: Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<metamodelica::Ref<DAE::Exp>> + 'static>,
) -> Result<(
    metamodelica::Ref<DAE::Exp>,
    Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<metamodelica::Ref<DAE::Exp>> + 'static>,
)> {
    pub type FuncExpType = std::sync::Arc<
        dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<metamodelica::Ref<DAE::Exp>> + 'static,
    >;

    let mut outExp: metamodelica::Ref<DAE::Exp>;
    let mut outFunc: Arc<
        dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<metamodelica::Ref<DAE::Exp>> + 'static,
    >;
    outExp = func(inExp)?;
    outFunc = func.clone();
    Ok((outExp, outFunc))
}

pub fn traverseSubexpressionsHelper<Type_a: Clone + 'static + metamodelica::gc::MMTrace + metamodelica::ReferenceEq>(
    mut inExp: metamodelica::Ref<DAE::Exp>,
    mut itpl: (
        Arc<
            dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>, Type_a) -> Result<(metamodelica::Ref<DAE::Exp>, Type_a)>
                + 'static,
        >,
        Type_a,
    ),
) -> Result<(
    metamodelica::Ref<DAE::Exp>,
    (
        Arc<
            dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>, Type_a) -> Result<(metamodelica::Ref<DAE::Exp>, Type_a)>
                + 'static,
        >,
        Type_a,
    ),
)> {
    pub type FuncExpType<Type_a: Clone + 'static> = std::sync::Arc<
        dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>, Type_a) -> Result<(metamodelica::Ref<DAE::Exp>, Type_a)>
            + 'static,
    >;

    let mut outExp: metamodelica::Ref<DAE::Exp>;
    let mut otpl: (
        Arc<
            dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>, Type_a) -> Result<(metamodelica::Ref<DAE::Exp>, Type_a)>
                + 'static,
        >,
        Type_a,
    );
    let mut rel: Arc<
        dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>, Type_a) -> Result<(metamodelica::Ref<DAE::Exp>, Type_a)>
            + 'static,
    >;
    let mut ext_arg: Type_a;
    let mut ext_arg2: Type_a;
    (rel, ext_arg) = itpl.clone();
    (outExp, ext_arg2) = traverseExpBottomUp(inExp, &*(rel.clone()), ext_arg.clone())?;
    otpl = if (metamodelica::ReferenceEq::reference_eq(&(ext_arg), &(ext_arg2.clone()))) {
        itpl
    } else {
        (rel.clone(), ext_arg2)
    };
    Ok((outExp, otpl))
}

pub fn traverseSubexpressions<Type_a: Clone + 'static + metamodelica::gc::MMTrace>(
    mut e: metamodelica::Ref<DAE::Exp>,
    mut arg: Type_a,
    mut func: &dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>, Type_a) -> Result<(metamodelica::Ref<DAE::Exp>, Type_a)>,
) -> Result<(metamodelica::Ref<DAE::Exp>, Type_a)> {
    pub type FuncExpType<Type_a: Clone + 'static> = std::sync::Arc<
        dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>, Type_a) -> Result<(metamodelica::Ref<DAE::Exp>, Type_a)>
            + 'static,
    >;

    let mut e: metamodelica::Ref<DAE::Exp> = e;
    let mut arg: Type_a = arg;
    (e, arg) = traverseExpBottomUp(e, func, arg)?;
    Ok((e, arg))
}

pub fn traverseSubexpressionsDummyHelper(
    mut inExp: metamodelica::Ref<DAE::Exp>,
    mut inFunc: Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<metamodelica::Ref<DAE::Exp>> + 'static>,
) -> Result<(
    metamodelica::Ref<DAE::Exp>,
    Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<metamodelica::Ref<DAE::Exp>> + 'static>,
)> {
    pub type FuncExpType = std::sync::Arc<
        dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<metamodelica::Ref<DAE::Exp>> + 'static,
    >;

    let mut outExp: metamodelica::Ref<DAE::Exp>;
    let mut outFunc: Arc<
        dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<metamodelica::Ref<DAE::Exp>> + 'static,
    >;
    (outExp, outFunc) = traverseExpBottomUp(inExp, &traverseExpDummyHelper, inFunc.clone())?;
    Ok((outExp, outFunc))
}

pub fn traverseSubexpressionsTopDownHelper<
    Type_a: Clone + 'static + metamodelica::gc::MMTrace + metamodelica::ReferenceEq,
>(
    mut inExp: metamodelica::Ref<DAE::Exp>,
    mut itpl: (
        Arc<
            dyn ::std::ops::Fn(
                    metamodelica::Ref<DAE::Exp>,
                    Type_a,
                ) -> Result<(metamodelica::Ref<DAE::Exp>, bool, Type_a)>
                + 'static,
        >,
        Type_a,
    ),
) -> Result<(
    metamodelica::Ref<DAE::Exp>,
    (
        Arc<
            dyn ::std::ops::Fn(
                    metamodelica::Ref<DAE::Exp>,
                    Type_a,
                ) -> Result<(metamodelica::Ref<DAE::Exp>, bool, Type_a)>
                + 'static,
        >,
        Type_a,
    ),
)> {
    pub type FuncExpType2<Type_a: Clone + 'static> = std::sync::Arc<
        dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>, Type_a) -> Result<(metamodelica::Ref<DAE::Exp>, bool, Type_a)>
            + 'static,
    >;

    let mut outExp: metamodelica::Ref<DAE::Exp>;
    let mut otpl: (
        Arc<
            dyn ::std::ops::Fn(
                    metamodelica::Ref<DAE::Exp>,
                    Type_a,
                ) -> Result<(metamodelica::Ref<DAE::Exp>, bool, Type_a)>
                + 'static,
        >,
        Type_a,
    );
    let mut rel: Arc<
        dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>, Type_a) -> Result<(metamodelica::Ref<DAE::Exp>, bool, Type_a)>
            + 'static,
    >;
    let mut ext_arg: Type_a;
    let mut ext_arg2: Type_a;
    (rel, ext_arg) = itpl.clone();
    (outExp, ext_arg2) = traverseExpTopDown(inExp, &*(rel.clone()), ext_arg.clone())?;
    otpl = if (metamodelica::ReferenceEq::reference_eq(&(ext_arg), &(ext_arg2.clone()))) {
        itpl
    } else {
        (rel.clone(), ext_arg2)
    };
    Ok((outExp, otpl))
}

fn traverseExpMatrix<Type_a: Clone + 'static + metamodelica::gc::MMTrace>(
    mut inMatrix: metamodelica::List<metamodelica::List<metamodelica::Ref<DAE::Exp>>>,
    mut func: &dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>, Type_a) -> Result<(metamodelica::Ref<DAE::Exp>, Type_a)>,
    mut inTypeA: Type_a,
) -> Result<(
    metamodelica::List<metamodelica::List<metamodelica::Ref<DAE::Exp>>>,
    Type_a,
)> {
    pub type FuncExpType<Type_a: Clone + 'static> = std::sync::Arc<
        dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>, Type_a) -> Result<(metamodelica::Ref<DAE::Exp>, Type_a)>
            + 'static,
    >;

    let mut outMatrix: metamodelica::List<metamodelica::List<metamodelica::Ref<DAE::Exp>>> = metamodelica::nil();
    let mut outTypeA: Type_a = inTypeA;
    let mut row_1: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
    let mut same: bool = true;
    for mut row in &*inMatrix {
        (row_1, outTypeA) = traverseExpList(row.clone(), func, outTypeA)?;
        same = if (metamodelica::ReferenceEq::reference_eq(&(row.clone()), &(row_1))) {
            same
        } else {
            false
        };
        outMatrix = metamodelica::cons(row_1, outMatrix);
    }
    if same {
        outMatrix = inMatrix;
    } else {
        outMatrix = metamodelica::Dangerous::listReverseInPlace(outMatrix);
    }
    Ok((outMatrix, outTypeA))
}

pub fn traverseExpList<ArgT: Clone + 'static + metamodelica::gc::MMTrace>(
    mut inExpl: metamodelica::List<metamodelica::Ref<DAE::Exp>>,
    mut rel: &dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>, ArgT) -> Result<(metamodelica::Ref<DAE::Exp>, ArgT)>,
    mut iext_arg: ArgT,
) -> Result<(metamodelica::List<metamodelica::Ref<DAE::Exp>>, ArgT)> {
    pub type FuncExpType<ArgT: Clone + 'static> = std::sync::Arc<
        dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>, ArgT) -> Result<(metamodelica::Ref<DAE::Exp>, ArgT)> + 'static,
    >;

    let mut expl: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
    let mut ext_arg: ArgT = iext_arg;
    let mut e1: metamodelica::Ref<DAE::Exp>;
    let mut delst: DoubleEnded::MutableList<metamodelica::Ref<DAE::Exp>>;
    let mut rest: metamodelica::List<metamodelica::Ref<DAE::Exp>> = inExpl.clone();
    let mut nEq: i32 = 0;
    expl = inExpl.clone();
    while !((rest).is_empty()) {
        (e1, ext_arg) = traverseExpBottomUp((rest).head().cloned()?, rel, ext_arg)?;
        if !(referenceEq(&*((rest).head().cloned()?), &*(&*e1))) {
            delst = DoubleEnded::empty(e1.clone());
            for mut elt in &*inExpl {
                if nEq < 1 {
                    break;
                }
                DoubleEnded::push_back(delst.clone(), elt.clone())?;
                nEq = nEq - 1;
            }
            DoubleEnded::push_back(delst.clone(), e1)?;
            for mut e in &*(rest).rest()? {
                (e1, ext_arg) = traverseExpBottomUp(e.clone(), rel, ext_arg)?;
                DoubleEnded::push_back(delst.clone(), e1)?;
            }
            expl = DoubleEnded::toListAndClear(delst, metamodelica::nil())?;
            return Ok((expl, ext_arg));
        }
        nEq = nEq + 1;
        rest = (rest).rest()?;
    }
    Ok((expl, ext_arg))
}

pub fn traverseExpTopDown<Type_a: Clone + 'static + metamodelica::gc::MMTrace>(
    mut inExp: metamodelica::Ref<DAE::Exp>,
    mut func: &dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>, Type_a) -> Result<(metamodelica::Ref<DAE::Exp>, bool, Type_a)>,
    mut ext_arg: Type_a,
) -> Result<(metamodelica::Ref<DAE::Exp>, Type_a)> {
    pub type FuncExpType<Type_a: Clone + 'static> = std::sync::Arc<
        dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>, Type_a) -> Result<(metamodelica::Ref<DAE::Exp>, bool, Type_a)>
            + 'static,
    >;

    let mut outExp: metamodelica::Ref<DAE::Exp>;
    let mut outArg: Type_a;
    let mut cont: bool;
    (outExp, cont, outArg) = func(inExp, ext_arg)?;
    (outExp, outArg) = traverseExpTopDown1(cont, outExp, func, outArg)?;
    Ok((outExp, outArg))
}

fn traverseExpClk<Type_a: Clone + 'static + metamodelica::gc::MMTrace>(
    mut inClk: metamodelica::Ref<DAE::ClockKind>,
    mut func: &dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>, Type_a) -> Result<(metamodelica::Ref<DAE::Exp>, Type_a)>,
    mut inArg: Type_a,
) -> Result<(metamodelica::Ref<DAE::ClockKind>, Type_a)> {
    pub type FuncExpType<Type_a: Clone + 'static> = std::sync::Arc<
        dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>, Type_a) -> Result<(metamodelica::Ref<DAE::Exp>, Type_a)>
            + 'static,
    >;

    let mut outClk: metamodelica::Ref<DAE::ClockKind>;
    let mut outArg: Type_a;
    (outClk, outArg) = (match &*inClk {
        DAE::ClockKind::RATIONAL_CLOCK {
            intervalCounter: e1,
            resolution: e2,
        } => {
            let mut ea: metamodelica::Ref<DAE::Exp>;
            let mut eb: metamodelica::Ref<DAE::Exp>;
            let mut arg: Type_a;
            let mut clk: metamodelica::Ref<DAE::ClockKind>;
            (ea, arg) = traverseExpBottomUp(e1.clone(), func, inArg.clone())?;
            (eb, arg) = traverseExpBottomUp(e2.clone(), func, inArg)?;
            clk = if (referenceEq(&*(&*ea), &*(e1.clone())) && referenceEq(&*(&*eb), &*(e2.clone()))) {
                inClk
            } else {
                metamodelica::Ref::new(DAE::ClockKind::RATIONAL_CLOCK {
                    intervalCounter: ea,
                    resolution: eb,
                })
            };
            (clk, arg)
        }
        DAE::ClockKind::REAL_CLOCK { interval: e } => {
            let mut e1: metamodelica::Ref<DAE::Exp>;
            let mut arg: Type_a;
            let mut clk: metamodelica::Ref<DAE::ClockKind>;
            (e1, arg) = traverseExpBottomUp(e.clone(), func, inArg)?;
            clk = if (referenceEq(&*(&*e1), &*(e.clone()))) {
                inClk
            } else {
                metamodelica::Ref::new(DAE::ClockKind::REAL_CLOCK { interval: e1 })
            };
            (clk, arg)
        }
        DAE::ClockKind::EVENT_CLOCK {
            condition: e1,
            startInterval: e2,
        } => {
            let mut ea: metamodelica::Ref<DAE::Exp>;
            let mut eb: metamodelica::Ref<DAE::Exp>;
            let mut arg: Type_a;
            let mut clk: metamodelica::Ref<DAE::ClockKind>;
            (ea, arg) = traverseExpBottomUp(e1.clone(), func, inArg.clone())?;
            (eb, arg) = traverseExpBottomUp(e2.clone(), func, inArg)?;
            clk = if (referenceEq(&*(&*ea), &*(e1.clone())) && referenceEq(&*(&*eb), &*(e2.clone()))) {
                inClk
            } else {
                metamodelica::Ref::new(DAE::ClockKind::EVENT_CLOCK {
                    condition: ea,
                    startInterval: eb,
                })
            };
            (clk, arg)
        }
        DAE::ClockKind::SOLVER_CLOCK {
            c: e1,
            solverMethod: e2,
        } => {
            let mut ea: metamodelica::Ref<DAE::Exp>;
            let mut eb: metamodelica::Ref<DAE::Exp>;
            let mut arg: Type_a;
            let mut clk: metamodelica::Ref<DAE::ClockKind>;
            (ea, arg) = traverseExpBottomUp(e1.clone(), func, inArg.clone())?;
            (eb, arg) = traverseExpBottomUp(e2.clone(), func, inArg)?;
            clk = if (referenceEq(&*(&*ea), &*(e1.clone())) && referenceEq(&*(&*eb), &*(e2.clone()))) {
                inClk
            } else {
                metamodelica::Ref::new(DAE::ClockKind::SOLVER_CLOCK {
                    c: ea,
                    solverMethod: eb,
                })
            };
            (clk, arg)
        }
        _ => (inClk, inArg),
    });
    Ok((outClk, outArg))
}

fn traverseExpTopDownClockHelper<Type_a: Clone + 'static + metamodelica::gc::MMTrace>(
    mut inClk: metamodelica::Ref<DAE::ClockKind>,
    mut func: &dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>, Type_a) -> Result<(metamodelica::Ref<DAE::Exp>, bool, Type_a)>,
    mut inArg: Type_a,
) -> Result<(metamodelica::Ref<DAE::ClockKind>, Type_a)> {
    pub type FuncExpType<Type_a: Clone + 'static> = std::sync::Arc<
        dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>, Type_a) -> Result<(metamodelica::Ref<DAE::Exp>, bool, Type_a)>
            + 'static,
    >;

    let mut outClk: metamodelica::Ref<DAE::ClockKind>;
    let mut outArg: Type_a;
    (outClk, outArg) = (match &*inClk {
        DAE::ClockKind::RATIONAL_CLOCK {
            intervalCounter: e1,
            resolution: e2,
        } => {
            let mut ea: metamodelica::Ref<DAE::Exp>;
            let mut eb: metamodelica::Ref<DAE::Exp>;
            let mut arg: Type_a;
            let mut clk: metamodelica::Ref<DAE::ClockKind>;
            (ea, arg) = traverseExpTopDown(e1.clone(), func, inArg.clone())?;
            (eb, arg) = traverseExpTopDown(e2.clone(), func, inArg)?;
            clk = if (referenceEq(&*(&*ea), &*(e1.clone())) && referenceEq(&*(&*eb), &*(e2.clone()))) {
                inClk
            } else {
                metamodelica::Ref::new(DAE::ClockKind::RATIONAL_CLOCK {
                    intervalCounter: ea,
                    resolution: eb,
                })
            };
            (clk, arg)
        }
        DAE::ClockKind::REAL_CLOCK { interval: e } => {
            let mut e1: metamodelica::Ref<DAE::Exp>;
            let mut arg: Type_a;
            let mut clk: metamodelica::Ref<DAE::ClockKind>;
            (e1, arg) = traverseExpTopDown(e.clone(), func, inArg)?;
            clk = if (referenceEq(&*(&*e1), &*(e.clone()))) {
                inClk
            } else {
                metamodelica::Ref::new(DAE::ClockKind::REAL_CLOCK { interval: e1 })
            };
            (clk, arg)
        }
        DAE::ClockKind::EVENT_CLOCK {
            condition: e1,
            startInterval: e2,
        } => {
            let mut ea: metamodelica::Ref<DAE::Exp>;
            let mut eb: metamodelica::Ref<DAE::Exp>;
            let mut arg: Type_a;
            let mut clk: metamodelica::Ref<DAE::ClockKind>;
            (ea, arg) = traverseExpTopDown(e1.clone(), func, inArg.clone())?;
            (eb, arg) = traverseExpTopDown(e2.clone(), func, inArg)?;
            clk = if (referenceEq(&*(&*ea), &*(e1.clone())) && referenceEq(&*(&*eb), &*(e2.clone()))) {
                inClk
            } else {
                metamodelica::Ref::new(DAE::ClockKind::EVENT_CLOCK {
                    condition: ea,
                    startInterval: eb,
                })
            };
            (clk, arg)
        }
        DAE::ClockKind::SOLVER_CLOCK {
            c: e1,
            solverMethod: e2,
        } => {
            let mut ea: metamodelica::Ref<DAE::Exp>;
            let mut eb: metamodelica::Ref<DAE::Exp>;
            let mut arg: Type_a;
            let mut clk: metamodelica::Ref<DAE::ClockKind>;
            (ea, arg) = traverseExpTopDown(e1.clone(), func, inArg.clone())?;
            (eb, arg) = traverseExpTopDown(e2.clone(), func, inArg)?;
            clk = if (referenceEq(&*(&*ea), &*(e1.clone())) && referenceEq(&*(&*eb), &*(e2.clone()))) {
                inClk
            } else {
                metamodelica::Ref::new(DAE::ClockKind::SOLVER_CLOCK {
                    c: ea,
                    solverMethod: eb,
                })
            };
            (clk, arg)
        }
        _ => (inClk, inArg),
    });
    Ok((outClk, outArg))
}

fn traverseExpTopDown1<Type_a: Clone + 'static + metamodelica::gc::MMTrace>(
    mut cont: bool,
    mut inExp: metamodelica::Ref<DAE::Exp>,
    mut func: &dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>, Type_a) -> Result<(metamodelica::Ref<DAE::Exp>, bool, Type_a)>,
    mut inArg: Type_a,
) -> Result<(metamodelica::Ref<DAE::Exp>, Type_a)> {
    pub type FuncExpType<Type_a: Clone + 'static> = std::sync::Arc<
        dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>, Type_a) -> Result<(metamodelica::Ref<DAE::Exp>, bool, Type_a)>
            + 'static,
    >;

    let mut outExp: metamodelica::Ref<DAE::Exp>;
    let mut outArg: Type_a;
    (outExp, outArg) = (::match_deref::match_deref! { match &((cont, inExp.clone())) {
        (false, _) => {
            (inExp, inArg)
        },
        (_, Deref @ DAE::Exp::ICONST { integer: _ }) => {
            let mut ext_arg = inArg.clone();
            (inExp, ext_arg)
        },
        (_, Deref @ DAE::Exp::RCONST { real: _ }) => {
            let mut ext_arg = inArg.clone();
            (inExp, ext_arg)
        },
        (_, Deref @ DAE::Exp::SCONST { string: _ }) => {
            let mut ext_arg = inArg.clone();
            (inExp, ext_arg)
        },
        (_, Deref @ DAE::Exp::BCONST { bool: _ }) => {
            let mut ext_arg = inArg.clone();
            (inExp, ext_arg)
        },
        (_, Deref @ DAE::Exp::CLKCONST { clk }) => {
            let mut ext_arg = inArg.clone();
            let mut e: metamodelica::Ref<DAE::Exp>;
            let mut clk1: metamodelica::Ref<DAE::ClockKind>;
            (clk1, ext_arg) = traverseExpTopDownClockHelper(clk.clone(), func, ext_arg)?;
            e = if (referenceEq(&*(&*clk1),&*(clk.clone()))) {inExp} else {metamodelica::Ref::new(DAE::Exp::CLKCONST { clk: clk1 })};
            (e, ext_arg)
        },
        (_, Deref @ DAE::Exp::ENUM_LITERAL { .. }) => {
            let mut ext_arg = inArg.clone();
            (inExp, ext_arg)
        },
        (_, Deref @ DAE::Exp::CREF { componentRef: cr, ty: tp }) => {
            let mut ext_arg = inArg.clone();
            let mut ext_arg_1: Type_a;
            let mut cr_1: ComponentRef;
            (cr_1, ext_arg_1) = traverseExpTopDownCrefHelper(metamodelica::AsArg::as_arg(&cr), func, ext_arg)?;
            (if (referenceEq(&*(cr.clone()),&*(&*cr_1))) {inExp} else {metamodelica::Ref::new(DAE::Exp::CREF { componentRef: cr_1, ty: tp.clone() })}, ext_arg_1)
        },
        (_, Deref @ DAE::Exp::UNARY { operator: op, exp: e1 }) => {
            let mut ext_arg = inArg.clone();
            let mut e1_1: metamodelica::Ref<DAE::Exp>;
            let mut ext_arg_1: Type_a;
            (e1_1, ext_arg_1) = traverseExpTopDown(e1.clone(), func, ext_arg)?;
            (if (referenceEq(&*(e1.clone()),&*(&*e1_1))) {inExp} else {metamodelica::Ref::new(DAE::Exp::UNARY { operator: op.clone(), exp: e1_1 })}, ext_arg_1)
        },
        (_, Deref @ DAE::Exp::BINARY { exp1: e1, operator: op, exp2: e2 }) => {
            let mut ext_arg = inArg.clone();
            let mut e1_1: metamodelica::Ref<DAE::Exp>;
            let mut e2_1: metamodelica::Ref<DAE::Exp>;
            let mut ext_arg_1: Type_a;
            let mut ext_arg_2: Type_a;
            (e1_1, ext_arg_1) = traverseExpTopDown(e1.clone(), func, ext_arg)?;
            (e2_1, ext_arg_2) = traverseExpTopDown(e2.clone(), func, ext_arg_1)?;
            (if (referenceEq(&*(e1.clone()),&*(&*e1_1)) && referenceEq(&*(e2.clone()),&*(&*e2_1))) {inExp} else {metamodelica::Ref::new(DAE::Exp::BINARY { exp1: e1_1, operator: op.clone(), exp2: e2_1 })}, ext_arg_2)
        },
        (_, Deref @ DAE::Exp::LUNARY { operator: op, exp: e1 }) => {
            let mut ext_arg = inArg.clone();
            let mut e1_1: metamodelica::Ref<DAE::Exp>;
            let mut ext_arg_1: Type_a;
            (e1_1, ext_arg_1) = traverseExpTopDown(e1.clone(), func, ext_arg)?;
            (if (referenceEq(&*(e1.clone()),&*(&*e1_1))) {inExp} else {metamodelica::Ref::new(DAE::Exp::LUNARY { operator: op.clone(), exp: e1_1 })}, ext_arg_1)
        },
        (_, Deref @ DAE::Exp::LBINARY { exp1: e1, operator: op, exp2: e2 }) => {
            let mut ext_arg = inArg.clone();
            let mut e1_1: metamodelica::Ref<DAE::Exp>;
            let mut e2_1: metamodelica::Ref<DAE::Exp>;
            let mut ext_arg_1: Type_a;
            let mut ext_arg_2: Type_a;
            (e1_1, ext_arg_1) = traverseExpTopDown(e1.clone(), func, ext_arg)?;
            (e2_1, ext_arg_2) = traverseExpTopDown(e2.clone(), func, ext_arg_1)?;
            (if (referenceEq(&*(e1.clone()),&*(&*e1_1)) && referenceEq(&*(e2.clone()),&*(&*e2_1))) {inExp} else {metamodelica::Ref::new(DAE::Exp::LBINARY { exp1: e1_1, operator: op.clone(), exp2: e2_1 })}, ext_arg_2)
        },
        (_, Deref @ DAE::Exp::RELATION { exp1: e1, operator: op, exp2: e2, index: index_, optionExpisASUB: isExpisASUB }) => {
            let mut ext_arg = inArg.clone();
            let mut e1_1: metamodelica::Ref<DAE::Exp>;
            let mut e2_1: metamodelica::Ref<DAE::Exp>;
            let mut ext_arg_1: Type_a;
            let mut ext_arg_2: Type_a;
            (e1_1, ext_arg_1) = traverseExpTopDown(e1.clone(), func, ext_arg)?;
            (e2_1, ext_arg_2) = traverseExpTopDown(e2.clone(), func, ext_arg_1)?;
            (if (referenceEq(&*(e1.clone()),&*(&*e1_1)) && referenceEq(&*(e2.clone()),&*(&*e2_1))) {inExp} else {metamodelica::Ref::new(DAE::Exp::RELATION { exp1: e1_1, operator: op.clone(), exp2: e2_1, index: index_.clone(), optionExpisASUB: isExpisASUB.clone() })}, ext_arg_2)
        },
        (_, Deref @ DAE::Exp::IFEXP { expCond: e1, expThen: e2, expElse: e3 }) => {
            let mut ext_arg = inArg.clone();
            let mut e1_1: metamodelica::Ref<DAE::Exp>;
            let mut e2_1: metamodelica::Ref<DAE::Exp>;
            let mut e3_1: metamodelica::Ref<DAE::Exp>;
            let mut ext_arg_1: Type_a;
            let mut ext_arg_2: Type_a;
            let mut ext_arg_3: Type_a;
            (e1_1, ext_arg_1) = traverseExpTopDown(e1.clone(), func, ext_arg)?;
            (e2_1, ext_arg_2) = traverseExpTopDown(e2.clone(), func, ext_arg_1)?;
            (e3_1, ext_arg_3) = traverseExpTopDown(e3.clone(), func, ext_arg_2)?;
            (if (referenceEq(&*(e1.clone()),&*(&*e1_1)) && referenceEq(&*(e2.clone()),&*(&*e2_1)) && referenceEq(&*(e3.clone()),&*(&*e3_1))) {inExp} else {metamodelica::Ref::new(DAE::Exp::IFEXP { expCond: e1_1, expThen: e2_1, expElse: e3_1 })}, ext_arg_3)
        },
        (_, Deref @ DAE::Exp::CALL { path: r#fn, expLst: expl, attr }) => {
            let mut ext_arg = inArg.clone();
            let mut ext_arg_1: Type_a;
            let mut expl_1: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
            (expl_1, ext_arg_1) = traverseExpListTopDown(expl.clone(), func, ext_arg)?;
            (metamodelica::Ref::new(DAE::Exp::CALL { path: r#fn.clone(), expLst: expl_1, attr: attr.clone() }), ext_arg_1)
        },
        (_, Deref @ DAE::Exp::RECORD { path: r#fn, exps: expl, comp: fieldNames, ty: tp }) => {
            let mut ext_arg = inArg.clone();
            let mut ext_arg_1: Type_a;
            let mut expl_1: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
            (expl_1, ext_arg_1) = traverseExpListTopDown(expl.clone(), func, ext_arg)?;
            (metamodelica::Ref::new(DAE::Exp::RECORD { path: r#fn.clone(), exps: expl_1, comp: fieldNames.clone(), ty: tp.clone() }), ext_arg_1)
        },
        (_, Deref @ DAE::Exp::PARTEVALFUNCTION { path: r#fn, expList: expl, ty: tp, origType: t }) => {
            let mut ext_arg = inArg.clone();
            let mut ext_arg_1: Type_a;
            let mut expl_1: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
            (expl_1, ext_arg_1) = traverseExpListTopDown(expl.clone(), func, ext_arg)?;
            (metamodelica::Ref::new(DAE::Exp::PARTEVALFUNCTION { path: r#fn.clone(), expList: expl_1, ty: tp.clone(), origType: t.clone() }), ext_arg_1)
        },
        (_, Deref @ DAE::Exp::ARRAY { ty: tp, scalar, array: expl }) => {
            let mut ext_arg = inArg.clone();
            let mut ext_arg_1: Type_a;
            let mut expl_1: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
            (expl_1, ext_arg_1) = traverseExpListTopDown(expl.clone(), func, ext_arg)?;
            (metamodelica::Ref::new(DAE::Exp::ARRAY { ty: tp.clone(), scalar: scalar.clone(), array: expl_1 }), ext_arg_1)
        },
        (_, Deref @ DAE::Exp::MATRIX { ty: tp, integer: dim, matrix: lstexpl }) => {
            let mut ext_arg = inArg.clone();
            let mut ext_arg_1: Type_a;
            let mut lstexpl_1: metamodelica::List<metamodelica::List<metamodelica::Ref<DAE::Exp>>>;
            (lstexpl_1, ext_arg_1) = traverseExpMatrixTopDown(lstexpl.clone(), func, ext_arg)?;
            (metamodelica::Ref::new(DAE::Exp::MATRIX { ty: tp.clone(), integer: dim.clone(), matrix: lstexpl_1 }), ext_arg_1)
        },
        (_, Deref @ DAE::Exp::RANGE { ty: tp, start: e1, step: None, stop: e2 }) => {
            let mut ext_arg = inArg.clone();
            let mut e1_1: metamodelica::Ref<DAE::Exp>;
            let mut e2_1: metamodelica::Ref<DAE::Exp>;
            let mut ext_arg_1: Type_a;
            let mut ext_arg_2: Type_a;
            (e1_1, ext_arg_1) = traverseExpTopDown(e1.clone(), func, ext_arg)?;
            (e2_1, ext_arg_2) = traverseExpTopDown(e2.clone(), func, ext_arg_1)?;
            (if (referenceEq(&*(e1.clone()),&*(&*e1_1)) && referenceEq(&*(e2.clone()),&*(&*e2_1))) {inExp} else {metamodelica::Ref::new(DAE::Exp::RANGE { ty: tp.clone(), start: e1_1, step: None, stop: e2_1 })}, ext_arg_2)
        },
        (_, Deref @ DAE::Exp::RANGE { ty: tp, start: e1, step: Some(e2), stop: e3 }) => {
            let mut ext_arg = inArg.clone();
            let mut e1_1: metamodelica::Ref<DAE::Exp>;
            let mut e2_1: metamodelica::Ref<DAE::Exp>;
            let mut e3_1: metamodelica::Ref<DAE::Exp>;
            let mut ext_arg_1: Type_a;
            let mut ext_arg_2: Type_a;
            let mut ext_arg_3: Type_a;
            (e1_1, ext_arg_1) = traverseExpTopDown(e1.clone(), func, ext_arg)?;
            (e2_1, ext_arg_2) = traverseExpTopDown(e2.clone(), func, ext_arg_1)?;
            (e3_1, ext_arg_3) = traverseExpTopDown(e3.clone(), func, ext_arg_2)?;
            (if (referenceEq(&*(e1.clone()),&*(&*e1_1)) && referenceEq(&*(e2.clone()),&*(&*e2_1)) && referenceEq(&*(e3.clone()),&*(&*e3_1))) {inExp} else {metamodelica::Ref::new(DAE::Exp::RANGE { ty: tp.clone(), start: e1_1, step: Some(e2_1), stop: e3_1 })}, ext_arg_3)
        },
        (_, Deref @ DAE::Exp::TUPLE { PR: expl }) => {
            let mut ext_arg = inArg.clone();
            let mut ext_arg_1: Type_a;
            let mut expl_1: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
            (expl_1, ext_arg_1) = traverseExpListTopDown(expl.clone(), func, ext_arg)?;
            (metamodelica::Ref::new(DAE::Exp::TUPLE { PR: expl_1 }), ext_arg_1)
        },
        (_, Deref @ DAE::Exp::CAST { ty: tp, exp: e1 }) => {
            let mut ext_arg = inArg.clone();
            let mut e1_1: metamodelica::Ref<DAE::Exp>;
            let mut ext_arg_1: Type_a;
            (e1_1, ext_arg_1) = traverseExpTopDown(e1.clone(), func, ext_arg)?;
            (metamodelica::Ref::new(DAE::Exp::CAST { ty: tp.clone(), exp: e1_1 }), ext_arg_1)
        },
        (_, Deref @ DAE::Exp::ASUB { exp: e1, sub: subs }) => {
            let mut ext_arg = inArg.clone();
            let mut e1_1: metamodelica::Ref<DAE::Exp>;
            let mut ext_arg_1: Type_a;
            let mut ext_arg_2: Type_a;
            let mut expl_1: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
            expl_1 = ({
        let mut __acc: metamodelica::List<metamodelica::Ref<DAE::Exp>> = metamodelica::nil();
        for mut sub in (subs.clone()).into_iter().cloned() {
            let __x = getSubscriptExp(&(sub.clone()))?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
            (e1_1, ext_arg_1) = traverseExpTopDown(e1.clone(), func, ext_arg)?;
            (expl_1, ext_arg_2) = traverseExpListTopDown(expl_1, func, ext_arg_1)?;
            (makeASUB(e1_1, expl_1)?, ext_arg_2)
        },
        (_, Deref @ DAE::Exp::TSUB { exp: e1, ix: i, ty: tp }) => {
            let mut ext_arg = inArg.clone();
            let mut e1_1: metamodelica::Ref<DAE::Exp>;
            let mut ext_arg_1: Type_a;
            (e1_1, ext_arg_1) = traverseExpTopDown(e1.clone(), func, ext_arg)?;
            (metamodelica::Ref::new(DAE::Exp::TSUB { exp: e1_1, ix: i.clone(), ty: tp.clone() }), ext_arg_1)
        },
        (_, e1 @ Deref @ DAE::Exp::RSUB { .. }) => {
            let mut ext_arg = inArg.clone();
            let mut e1_1: metamodelica::Ref<DAE::Exp>;
            let mut ext_arg_1: Type_a;
            let mut e1 = (*e1).clone();
            (e1_1, ext_arg_1) = traverseExpTopDown(var_field!((*e1).exp, DAE::Exp::RSUB).clone(), func, ext_arg)?;
            if !(referenceEq(&*(var_field!((*e1).exp, DAE::Exp::RSUB).clone()),&*(&*e1_1))) {
                assign_variant_field!(e1 => DAE::Exp::RSUB; exp = e1_1);
            }
            (e1.clone(), ext_arg_1)
        },
        (_, Deref @ DAE::Exp::SIZE { exp: e1, sz: None }) => {
            let mut ext_arg = inArg.clone();
            let mut e1_1: metamodelica::Ref<DAE::Exp>;
            let mut ext_arg_1: Type_a;
            (e1_1, ext_arg_1) = traverseExpTopDown(e1.clone(), func, ext_arg)?;
            (metamodelica::Ref::new(DAE::Exp::SIZE { exp: e1_1, sz: None }), ext_arg_1)
        },
        (_, Deref @ DAE::Exp::SIZE { exp: e1, sz: Some(e2) }) => {
            let mut ext_arg = inArg.clone();
            let mut e1_1: metamodelica::Ref<DAE::Exp>;
            let mut e2_1: metamodelica::Ref<DAE::Exp>;
            let mut ext_arg_1: Type_a;
            let mut ext_arg_2: Type_a;
            (e1_1, ext_arg_1) = traverseExpTopDown(e1.clone(), func, ext_arg)?;
            (e2_1, ext_arg_2) = traverseExpTopDown(e2.clone(), func, ext_arg_1)?;
            (if (referenceEq(&*(e1.clone()),&*(&*e1_1)) && referenceEq(&*(e2.clone()),&*(&*e2_1))) {inExp} else {metamodelica::Ref::new(DAE::Exp::SIZE { exp: e1_1, sz: Some(e2_1) })}, ext_arg_2)
        },
        (_, Deref @ DAE::Exp::CODE { .. }) => {
            let mut ext_arg = inArg.clone();
            (inExp, ext_arg)
        },
        (_, Deref @ DAE::Exp::REDUCTION { reductionInfo, expr: e1, iterators: riters }) => {
            let mut ext_arg = inArg.clone();
            let mut e1 = (*e1).clone();
            let mut riters = (*riters).clone();
            (e1, ext_arg) = traverseExpTopDown(e1.clone(), func, ext_arg)?;
            (riters, ext_arg) = traverseReductionIteratorsTopDown(metamodelica::AsArg::as_arg(&riters), func, ext_arg)?;
            (metamodelica::Ref::new(DAE::Exp::REDUCTION { reductionInfo: reductionInfo.clone(), expr: e1.clone(), iterators: riters.clone() }), ext_arg)
        },
        (_, Deref @ DAE::Exp::EMPTY { .. }) => {
            (inExp, inArg)
        },
        (_, Deref @ DAE::Exp::CONS { car: e1, cdr: e2 }) => {
            let mut ext_arg = inArg.clone();
            let mut e1_1: metamodelica::Ref<DAE::Exp>;
            let mut e2_1: metamodelica::Ref<DAE::Exp>;
            let mut ext_arg_1: Type_a;
            let mut ext_arg_2: Type_a;
            (e1_1, ext_arg_1) = traverseExpTopDown(e1.clone(), func, ext_arg)?;
            (e2_1, ext_arg_2) = traverseExpTopDown(e2.clone(), func, ext_arg_1)?;
            (if (referenceEq(&*(e1.clone()),&*(&*e1_1)) && referenceEq(&*(e2.clone()),&*(&*e2_1))) {inExp} else {metamodelica::Ref::new(DAE::Exp::CONS { car: e1_1, cdr: e2_1 })}, ext_arg_2)
        },
        (_, Deref @ DAE::Exp::LIST { valList: expl }) => {
            let mut ext_arg = inArg.clone();
            let mut ext_arg_1: Type_a;
            let mut expl_1: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
            (expl_1, ext_arg_1) = traverseExpListTopDown(expl.clone(), func, ext_arg)?;
            (metamodelica::Ref::new(DAE::Exp::LIST { valList: expl_1 }), ext_arg_1)
        },
        (_, Deref @ DAE::Exp::META_TUPLE { listExp: expl }) => {
            let mut ext_arg = inArg.clone();
            let mut ext_arg_1: Type_a;
            let mut expl_1: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
            (expl_1, ext_arg_1) = traverseExpListTopDown(expl.clone(), func, ext_arg)?;
            (metamodelica::Ref::new(DAE::Exp::META_TUPLE { listExp: expl_1 }), ext_arg_1)
        },
        (_, Deref @ DAE::Exp::META_OPTION { exp: oe1 }) => {
            let mut ext_arg = inArg.clone();
            let mut oe1 = (*oe1).clone();
            (oe1, ext_arg) = traverseExpOptTopDown(oe1.clone(), func, ext_arg)?;
            (metamodelica::Ref::new(DAE::Exp::META_OPTION { exp: oe1.clone() }), ext_arg)
        },
        (_, Deref @ DAE::Exp::MATCHEXPRESSION { matchType, inputs: expl, aliases, localDecls, cases, et }) => {
            let mut ext_arg = inArg.clone();
            let mut expl = (*expl).clone();
            let mut cases = (*cases).clone();
            (expl, ext_arg) = traverseExpListTopDown(expl.clone(), func, ext_arg)?;
            (cases, ext_arg) = traverseMatchCasesTopDown(cases.clone(), func, ext_arg);
            (metamodelica::Ref::new(DAE::Exp::MATCHEXPRESSION { matchType: matchType.clone(), inputs: expl.clone(), aliases: aliases.clone(), localDecls: localDecls.clone(), cases: cases.clone(), et: et.clone() }), ext_arg)
        },
        (_, Deref @ DAE::Exp::METARECORDCALL { path: r#fn, args: expl, fieldNames, index: i, typeVars }) => {
            let mut ext_arg = inArg.clone();
            let mut ext_arg_1: Type_a;
            let mut expl_1: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
            (expl_1, ext_arg_1) = traverseExpListTopDown(expl.clone(), func, ext_arg)?;
            (metamodelica::Ref::new(DAE::Exp::METARECORDCALL { path: r#fn.clone(), args: expl_1, fieldNames: fieldNames.clone(), index: i.clone(), typeVars: typeVars.clone() }), ext_arg_1)
        },
        (_, Deref @ DAE::Exp::UNBOX { exp: e1, ty: tp }) => {
            let mut ext_arg = inArg.clone();
            let mut e1_1: metamodelica::Ref<DAE::Exp>;
            let mut ext_arg_1: Type_a;
            (e1_1, ext_arg_1) = traverseExpTopDown(e1.clone(), func, ext_arg)?;
            (metamodelica::Ref::new(DAE::Exp::UNBOX { exp: e1_1, ty: tp.clone() }), ext_arg_1)
        },
        (_, Deref @ DAE::Exp::BOX { exp: e1 }) => {
            let mut ext_arg = inArg.clone();
            let mut e1_1: metamodelica::Ref<DAE::Exp>;
            let mut ext_arg_1: Type_a;
            (e1_1, ext_arg_1) = traverseExpTopDown(e1.clone(), func, ext_arg)?;
            (metamodelica::Ref::new(DAE::Exp::BOX { exp: e1_1 }), ext_arg_1)
        },
        (_, Deref @ DAE::Exp::PATTERN { .. }) => {
            let mut ext_arg = inArg.clone();
            (inExp, ext_arg)
        },
        (_, Deref @ DAE::Exp::SHARED_LITERAL { .. }) => {
            let mut ext_arg = inArg.clone();
            (inExp, ext_arg)
        },
        _ => {
            let mut r#str: ArcStr;
            r#str = printExpStr(inExp)?;
            r#str = { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Expression.traverseExpTopDown1")); __mm_s.push_str(&*literal!(" or ")); __mm_s.push_str(&*(System::dladdr(func)).0); __mm_s.push_str(&*literal!("not implemented correctly: ")); __mm_s.push_str(&*r#str); ArcStr::from(__mm_s) };
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![r#str])?;
            return Err("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outExp, outArg))
}

fn traverseExpMatrixTopDown<Type_a: Clone + 'static + metamodelica::gc::MMTrace>(
    mut inMatrix: metamodelica::List<metamodelica::List<metamodelica::Ref<DAE::Exp>>>,
    mut func: &dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>, Type_a) -> Result<(metamodelica::Ref<DAE::Exp>, bool, Type_a)>,
    mut inTypeA: Type_a,
) -> Result<(
    metamodelica::List<metamodelica::List<metamodelica::Ref<DAE::Exp>>>,
    Type_a,
)> {
    pub type FuncExpType<Type_a: Clone + 'static> = std::sync::Arc<
        dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>, Type_a) -> Result<(metamodelica::Ref<DAE::Exp>, bool, Type_a)>
            + 'static,
    >;

    let mut outMatrix: metamodelica::List<metamodelica::List<metamodelica::Ref<DAE::Exp>>> = metamodelica::nil();
    let mut outTypeA: Type_a = inTypeA;
    let mut row_1: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
    let mut same: bool = true;
    for mut row in &*inMatrix {
        (row_1, outTypeA) = traverseExpListTopDown(row.clone(), func, outTypeA)?;
        same = if (metamodelica::ReferenceEq::reference_eq(&(row.clone()), &(row_1))) {
            same
        } else {
            false
        };
        outMatrix = metamodelica::cons(row_1, outMatrix);
    }
    if same {
        outMatrix = inMatrix;
    } else {
        outMatrix = metamodelica::Dangerous::listReverseInPlace(outMatrix);
    }
    Ok((outMatrix, outTypeA))
}

pub fn traverseExpListTopDown<Type_a: Clone + 'static + metamodelica::gc::MMTrace>(
    mut inExpl: metamodelica::List<metamodelica::Ref<DAE::Exp>>,
    mut rel: &dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>, Type_a) -> Result<(metamodelica::Ref<DAE::Exp>, bool, Type_a)>,
    mut inExt_arg: Type_a,
) -> Result<(metamodelica::List<metamodelica::Ref<DAE::Exp>>, Type_a)> {
    pub type FuncExpType<Type_a: Clone + 'static> = std::sync::Arc<
        dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>, Type_a) -> Result<(metamodelica::Ref<DAE::Exp>, bool, Type_a)>
            + 'static,
    >;

    let mut outExpl: metamodelica::List<metamodelica::Ref<DAE::Exp>> = metamodelica::nil();
    let mut outA: Type_a = inExt_arg;
    let mut e_1: metamodelica::Ref<DAE::Exp>;
    let mut same: bool = true;
    for mut e in &*inExpl {
        (e_1, outA) = traverseExpTopDown(e.clone(), rel, outA)?;
        same = if (referenceEq(&*(e.clone()), &*(&*e_1))) {
            same
        } else {
            false
        };
        outExpl = metamodelica::cons(e_1, outExpl);
    }
    if same {
        outExpl = inExpl;
    } else {
        outExpl = metamodelica::Dangerous::listReverseInPlace(outExpl);
    }
    Ok((outExpl, outA))
}

pub fn traverseExpOpt<Type_a: Clone + 'static + metamodelica::gc::MMTrace>(
    mut inExp: Option<metamodelica::Ref<DAE::Exp>>,
    mut func: &dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>, Type_a) -> Result<(metamodelica::Ref<DAE::Exp>, Type_a)>,
    mut inTypeA: Type_a,
) -> Result<(Option<metamodelica::Ref<DAE::Exp>>, Type_a)> {
    pub type FuncExpType<Type_a: Clone + 'static> = std::sync::Arc<
        dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>, Type_a) -> Result<(metamodelica::Ref<DAE::Exp>, Type_a)>
            + 'static,
    >;

    let mut outExp: Option<metamodelica::Ref<DAE::Exp>>;
    let mut outTypeA: Type_a;
    (outExp, outTypeA) = (::match_deref::match_deref! { match &(inExp.clone()) {
        None => {
            let mut a = inTypeA;
            (inExp, a)
        },
        oe @ Some(e) => {
            let mut a = inTypeA;
            let mut e1: metamodelica::Ref<DAE::Exp>;
            let mut oe = (*oe).clone();
            (e1, a) = traverseExpBottomUp(e.clone(), func, a)?;
            oe = if (referenceEq(&*(e.clone()),&*(&*e1))) {oe.clone()} else {Some(e1)};
            (oe.clone(), a)
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok((outExp, outTypeA))
}

pub(crate) fn traverseExpOptTopDown<Type_a: Clone + 'static + metamodelica::gc::MMTrace>(
    mut inExp: Option<metamodelica::Ref<DAE::Exp>>,
    mut func: &dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>, Type_a) -> Result<(metamodelica::Ref<DAE::Exp>, bool, Type_a)>,
    mut inTypeA: Type_a,
) -> Result<(Option<metamodelica::Ref<DAE::Exp>>, Type_a)> {
    pub type FuncExpType<Type_a: Clone + 'static> = std::sync::Arc<
        dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>, Type_a) -> Result<(metamodelica::Ref<DAE::Exp>, bool, Type_a)>
            + 'static,
    >;

    let mut outExp: Option<metamodelica::Ref<DAE::Exp>>;
    let mut outA: Type_a;
    (outExp, outA) = (::match_deref::match_deref! { match &(inExp.clone()) {
        None => {
            let mut a = inTypeA;
            (None, a)
        },
        Some(e) => {
            let mut a = inTypeA;
            let mut e1: metamodelica::Ref<DAE::Exp>;
            (e1, a) = traverseExpTopDown(e.clone(), func, a)?;
            (if (referenceEq(&*(e.clone()),&*(&*e1))) {inExp} else {Some(e1)}, a)
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outExp, outA))
}

pub fn traverseExpCrefDims<ArgT: Clone + 'static + metamodelica::gc::MMTrace>(
    mut inCref: &metamodelica::Ref<DAE::ComponentRef>,
    mut inFunc: &dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>, ArgT) -> Result<(metamodelica::Ref<DAE::Exp>, ArgT)>,
    mut inArg: ArgT,
) -> Result<(metamodelica::Ref<DAE::ComponentRef>, ArgT)> {
    pub type FuncType<ArgT: Clone + 'static> = std::sync::Arc<
        dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>, ArgT) -> Result<(metamodelica::Ref<DAE::Exp>, ArgT)> + 'static,
    >;

    let mut outCref: metamodelica::Ref<DAE::ComponentRef>;
    let mut outArg: ArgT;
    (outCref, outArg) = (match &**inCref {
        DAE::ComponentRef::CREF_QUAL {
            ident: id,
            identType: ty,
            subscriptLst: subs,
            componentRef: cr,
        } => {
            let mut new_ty: metamodelica::Ref<DAE::Type>;
            let mut new_cr: metamodelica::Ref<DAE::ComponentRef>;
            let mut arg: ArgT;
            let mut cr = (*cr).clone();
            (new_cr, arg) = traverseExpCrefDims(metamodelica::AsArg::as_arg(&cr), inFunc, inArg.clone())?;
            (new_ty, arg) = traverseExpTypeDims(ty, inFunc, inArg)?;
            cr = if (referenceEq(&*(&*new_cr), &*(cr.clone())) && referenceEq(&*(&*new_ty), &*(ty.clone()))) {
                inCref.clone()
            } else {
                metamodelica::Ref::new(DAE::ComponentRef::CREF_QUAL {
                    ident: id.clone(),
                    identType: new_ty,
                    subscriptLst: subs.clone(),
                    componentRef: new_cr,
                })
            };
            (cr.clone(), arg)
        }
        DAE::ComponentRef::CREF_IDENT {
            ident: id,
            identType: ty,
            subscriptLst: subs,
        } => {
            let mut new_ty: metamodelica::Ref<DAE::Type>;
            let mut cr: metamodelica::Ref<DAE::ComponentRef>;
            let mut arg: ArgT;
            (new_ty, arg) = traverseExpTypeDims(ty, inFunc, inArg)?;
            cr = if (referenceEq(&*(&*new_ty), &*(ty.clone()))) {
                inCref.clone()
            } else {
                metamodelica::Ref::new(DAE::ComponentRef::CREF_IDENT {
                    ident: id.clone(),
                    identType: new_ty,
                    subscriptLst: subs.clone(),
                })
            };
            (cr, arg)
        }
        _ => (inCref.clone(), inArg),
    });
    Ok((outCref, outArg))
}

pub(crate) fn traverseExpTypeDims<ArgT: Clone + 'static + metamodelica::gc::MMTrace>(
    mut inType: &metamodelica::Ref<DAE::Type>,
    mut inFunc: &dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>, ArgT) -> Result<(metamodelica::Ref<DAE::Exp>, ArgT)>,
    mut inArg: ArgT,
) -> Result<(metamodelica::Ref<DAE::Type>, ArgT)> {
    pub type FuncType<ArgT: Clone + 'static> = std::sync::Arc<
        dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>, ArgT) -> Result<(metamodelica::Ref<DAE::Exp>, ArgT)> + 'static,
    >;

    let mut outType: metamodelica::Ref<DAE::Type>;
    let mut outArg: ArgT;
    (outType, outArg) = (match &**inType {
        DAE::Type::T_ARRAY { ty, dims } => {
            let mut arg: ArgT;
            let mut changed: bool;
            let mut ty = (*ty).clone();
            (_, arg, changed) = traverseExpTypeDims2(dims.clone(), inFunc, inArg)?;
            ty = if (changed) {
                metamodelica::Ref::new(DAE::Type::T_ARRAY {
                    ty: ty.clone(),
                    dims: dims.clone(),
                })
            } else {
                inType.clone()
            };
            (ty.clone(), arg)
        }
        DAE::Type::T_SUBTYPE_BASIC {
            complexClassType: state,
            varLst: vars,
            complexType: ty,
            equalityConstraint: ec,
        } => {
            let mut new_ty: metamodelica::Ref<DAE::Type>;
            let mut arg: ArgT;
            let mut ty = (*ty).clone();
            (new_ty, arg) = traverseExpTypeDims(metamodelica::AsArg::as_arg(&ty), inFunc, inArg)?;
            ty = if (referenceEq(&*(new_ty), &*(ty.clone()))) {
                inType.clone()
            } else {
                metamodelica::Ref::new(DAE::Type::T_SUBTYPE_BASIC {
                    complexClassType: state.clone(),
                    varLst: vars.clone(),
                    complexType: ty.clone(),
                    equalityConstraint: ec.clone(),
                })
            };
            (ty.clone(), arg)
        }
        _ => (inType.clone(), inArg),
    });
    Ok((outType, outArg))
}

fn traverseExpTypeDims2<ArgT: Clone + 'static + metamodelica::gc::MMTrace>(
    mut inDims: metamodelica::List<metamodelica::Ref<DAE::Dimension>>,
    mut inFunc: &dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>, ArgT) -> Result<(metamodelica::Ref<DAE::Exp>, ArgT)>,
    mut inArg: ArgT,
) -> Result<(metamodelica::List<metamodelica::Ref<DAE::Dimension>>, ArgT, bool)> {
    pub type FuncType<ArgT: Clone + 'static> = std::sync::Arc<
        dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>, ArgT) -> Result<(metamodelica::Ref<DAE::Exp>, ArgT)> + 'static,
    >;

    let mut outDims: metamodelica::List<metamodelica::Ref<DAE::Dimension>> = metamodelica::nil();
    let mut outArg: ArgT = inArg;
    let mut outChanged: bool = false;
    let mut changed: bool;
    let mut exp: metamodelica::Ref<DAE::Exp>;
    let mut new_exp: metamodelica::Ref<DAE::Exp>;
    for mut dim in &*inDims {
        let mut dim = dim.clone();
        dim = (match &*dim {
            DAE::Dimension::DIM_EXP { exp: __esc_exp } => {
                exp = (*__esc_exp).clone();
                (new_exp, outArg) = inFunc(exp.clone(), outArg)?;
                changed = !(referenceEq(&*(new_exp), &*(exp.clone())));
                outChanged = outChanged || changed;
                if (changed) {
                    metamodelica::Ref::new(DAE::Dimension::DIM_EXP { exp: exp.clone() })
                } else {
                    dim
                }
            }
            _ => dim,
        });
        outDims = metamodelica::cons(dim, outDims);
    }
    outDims = if (outChanged) { outDims.reverse() } else { inDims };
    Ok((outDims, outArg, outChanged))
}

pub(crate) fn extractUniqueCrefsFromExp(
    mut inExp: metamodelica::Ref<DAE::Exp>,
    mut expand: bool,
) -> Result<metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>> {
    let mut ocrefs: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
    ocrefs = ComponentReference::uniqueList(extractCrefsFromExp(inExp)?)?;
    if expand {
        ocrefs = List::flatten(List::map1(
            ocrefs,
            &move |__a0: metamodelica::Ref<DAE::ComponentRef>, __a1: bool| ComponentReference::expandCref(&__a0, __a1),
            true,
        )?)?;
    }
    Ok(ocrefs)
}

pub fn extractCrefsFromExp(
    mut inExp: metamodelica::Ref<DAE::Exp>,
) -> Result<metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>> {
    let mut ocrefs: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
    (_, ocrefs) = traverseExpBottomUp(inExp, &traversingComponentRefFinder, metamodelica::nil())?;
    Ok(ocrefs)
}

pub fn traversingComponentRefFinder(
    mut inExp: metamodelica::Ref<DAE::Exp>,
    mut inCrefs: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
) -> Result<(
    metamodelica::Ref<DAE::Exp>,
    metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
)> {
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    let mut crefs: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
    (outExp, crefs) = (::match_deref::match_deref! { match &((inExp.clone(), inCrefs.clone())) {
        (Deref @ DAE::Exp::CREF { componentRef: cr, .. }, __esc_crefs) => {
            crefs = (*__esc_crefs).clone();
            crefs = List::unionEltOnTrue(cr.clone(), crefs.clone(), &move |__a0: metamodelica::Ref<DAE::ComponentRef>, __a1: metamodelica::Ref<DAE::ComponentRef>| ComponentReferenceBasics::crefEqual(&__a0, &__a1))?;
            (inExp, crefs.clone())
        },
        _ => {
            (inExp, inCrefs)
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outExp, crefs))
}

pub fn extractUniqueCrefsFromExpDerPreStart(
    mut inExp: metamodelica::Ref<DAE::Exp>,
    mut expand: bool,
) -> Result<metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>> {
    let mut ocrefs: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
    ocrefs = ComponentReference::uniqueList(extractCrefsFromExpDerPreStart(inExp, expand)?)?;
    Ok(ocrefs)
}

pub fn extractCrefsFromExpDerPreStart(
    mut inExp: metamodelica::Ref<DAE::Exp>,
    mut expand: bool,
) -> Result<metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>> {
    let mut ocrefs: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
    (_, ocrefs) = traverseExpTopDown(inExp, &traversingComponentRefFinderDerPreStart, metamodelica::nil())?;
    if expand {
        ocrefs = List::flatten(List::map1(
            ocrefs,
            &move |__a0: metamodelica::Ref<DAE::ComponentRef>, __a1: bool| ComponentReference::expandCref(&__a0, __a1),
            true,
        )?)?;
    }
    Ok(ocrefs)
}

pub(crate) fn traversingComponentRefFinderDerPreStart(
    mut inExp: metamodelica::Ref<DAE::Exp>,
    mut inCrefs: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
) -> Result<(
    metamodelica::Ref<DAE::Exp>,
    bool,
    metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
)> {
    let mut e: metamodelica::Ref<DAE::Exp>;
    let mut cont: bool;
    let mut crefs: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
    (e, cont, crefs) = (::match_deref::match_deref! { match &((inExp.clone(), inCrefs.clone())) {
        (Deref @ DAE::Exp::CREF { componentRef: cr, .. }, __esc_crefs) => {
            crefs = (*__esc_crefs).clone();
            crefs = List::unionEltOnTrue(cr.clone(), inCrefs, &move |__a0: metamodelica::Ref<DAE::ComponentRef>, __a1: metamodelica::Ref<DAE::ComponentRef>| ComponentReferenceBasics::crefEqual(&__a0, &__a1))?;
            (inExp, false, crefs.clone())
        },
        (Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "der" }, expLst: Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Exp::CREF { componentRef: cr, .. }, tail: Deref @ metamodelica::ListNode::Nil }, .. }, _) => {
            let mut cr = (*cr).clone();
            cr = ComponentReference::crefPrefixDer(cr.clone());
            crefs = List::unionEltOnTrue(cr.clone(), inCrefs, &move |__a0: metamodelica::Ref<DAE::ComponentRef>, __a1: metamodelica::Ref<DAE::ComponentRef>| ComponentReferenceBasics::crefEqual(&__a0, &__a1))?;
            (inExp, false, crefs)
        },
        (Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "pre" }, expLst: Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Exp::CREF { componentRef: cr, .. }, tail: Deref @ metamodelica::ListNode::Nil }, .. }, _) => {
            let mut cr = (*cr).clone();
            cr = ComponentReference::crefPrefixPre(cr.clone());
            crefs = List::unionEltOnTrue(cr.clone(), inCrefs, &move |__a0: metamodelica::Ref<DAE::ComponentRef>, __a1: metamodelica::Ref<DAE::ComponentRef>| ComponentReferenceBasics::crefEqual(&__a0, &__a1))?;
            (inExp, false, crefs)
        },
        (Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "previous" }, expLst: Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Exp::CREF { componentRef: cr, .. }, tail: Deref @ metamodelica::ListNode::Nil }, .. }, _) => {
            let mut cr = (*cr).clone();
            cr = ComponentReference::crefPrefixPrevious(cr.clone());
            crefs = List::unionEltOnTrue(cr.clone(), inCrefs, &move |__a0: metamodelica::Ref<DAE::ComponentRef>, __a1: metamodelica::Ref<DAE::ComponentRef>| ComponentReferenceBasics::crefEqual(&__a0, &__a1))?;
            (inExp, false, crefs)
        },
        (Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "start" }, expLst: Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Exp::CREF { componentRef: cr, .. }, tail: Deref @ metamodelica::ListNode::Nil }, .. }, _) => {
            let mut cr = (*cr).clone();
            Error::addInternalError({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Expression.traversingComponentRefFinderDerPreStart")); __mm_s.push_str(&*literal!(" - Found a start call expression ")); __mm_s.push_str(&*printExpStr(inExp.clone())?); ArcStr::from(__mm_s) }, metamodelica::sourceInfo!("FrontEnd/Expression.mo"))?;
            cr = ComponentReference::crefPrefixStart(cr.clone());
            crefs = List::unionEltOnTrue(cr.clone(), inCrefs, &move |__a0: metamodelica::Ref<DAE::ComponentRef>, __a1: metamodelica::Ref<DAE::ComponentRef>| ComponentReferenceBasics::crefEqual(&__a0, &__a1))?;
            (inExp, false, crefs)
        },
        _ => {
            (inExp, true, inCrefs)
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((e, cont, crefs))
}

pub fn extractUniqueCrefsFromStatmentS(
    mut inStmts: &metamodelica::List<metamodelica::Ref<DAE::Statement>>,
) -> Result<(
    metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
    metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
)> {
    let mut olhscrefs: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
    let mut orhscrefs: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
    let mut lhscreflstlst: metamodelica::List<metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>>;
    let mut rhscreflstlst: metamodelica::List<metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>>;
    (lhscreflstlst, rhscreflstlst) = List::map_2(inStmts, &move |__a0: metamodelica::Ref<DAE::Statement>| {
        extractCrefsStatment(&__a0)
    })?;
    orhscrefs = ComponentReference::uniqueList(List::flatten(rhscreflstlst)?)?;
    olhscrefs = ComponentReference::uniqueList(List::flatten(lhscreflstlst)?)?;
    Ok((olhscrefs, orhscrefs))
}

pub fn extractCrefsStatment(
    mut inStmt: &metamodelica::Ref<DAE::Statement>,
) -> Result<(
    metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
    metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
)> {
    let mut olcrefs: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
    let mut orcrefs: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
    (olcrefs, orcrefs) = (match &**inStmt {
        DAE::Statement::STMT_ASSIGN { exp1, exp: exp2, .. } => {
            olcrefs = extractCrefsFromExpDerPreStart(exp1.clone(), false)?;
            orcrefs = extractCrefsFromExpDerPreStart(exp2.clone(), false)?;
            (olcrefs, orcrefs)
        }
        DAE::Statement::STMT_TUPLE_ASSIGN {
            expExpLst: expLst,
            exp: exp2,
            ..
        } => {
            olcrefs = List::flatten(List::map1(expLst.clone(), &extractCrefsFromExpDerPreStart, false)?)?;
            orcrefs = extractCrefsFromExpDerPreStart(exp2.clone(), false)?;
            (olcrefs, orcrefs)
        }
        DAE::Statement::STMT_ASSIGN_ARR {
            lhs: exp1, exp: exp2, ..
        } => {
            olcrefs = extractCrefsFromExpDerPreStart(exp1.clone(), false)?;
            orcrefs = extractCrefsFromExpDerPreStart(exp2.clone(), false)?;
            (olcrefs, orcrefs)
        }
        DAE::Statement::STMT_IF {
            statementLst: stmtLst, ..
        } => {
            (olcrefs, orcrefs) = extractUniqueCrefsFromStatmentS(stmtLst)?;
            (olcrefs, orcrefs)
        }
        DAE::Statement::STMT_FOR {
            statementLst: stmtLst, ..
        } => {
            (olcrefs, orcrefs) = extractUniqueCrefsFromStatmentS(stmtLst)?;
            (olcrefs, orcrefs)
        }
        DAE::Statement::STMT_WHILE {
            statementLst: stmtLst, ..
        } => {
            (olcrefs, orcrefs) = extractUniqueCrefsFromStatmentS(stmtLst)?;
            (olcrefs, orcrefs)
        }
        DAE::Statement::STMT_WHEN {
            statementLst: stmtLst, ..
        } => {
            (olcrefs, orcrefs) = extractUniqueCrefsFromStatmentS(stmtLst)?;
            (olcrefs, orcrefs)
        }
        DAE::Statement::STMT_ASSERT { cond: exp1, .. } => {
            orcrefs = extractCrefsFromExpDerPreStart(exp1.clone(), false)?;
            (metamodelica::nil(), orcrefs)
        }
        _ => (metamodelica::nil(), metamodelica::nil()),
    });
    Ok((olcrefs, orcrefs))
}

pub fn getLhsCrefsFromStatements(
    mut inStmts: metamodelica::List<metamodelica::Ref<DAE::Statement>>,
) -> Result<metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>> {
    let mut lhsCrefs: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
    let mut lhsCrefsLst: metamodelica::List<metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>>;
    lhsCrefsLst = List::map(inStmts, &move |__a0: metamodelica::Ref<DAE::Statement>| {
        getLhsCrefsFromStatement(&__a0)
    })?;
    lhsCrefs = List::flatten(lhsCrefsLst)?;
    Ok(lhsCrefs)
}

fn getLhsCrefsFromStatement(
    mut inStmt: &metamodelica::Ref<DAE::Statement>,
) -> Result<metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>> {
    let mut lhsCrefs: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
    lhsCrefs = (match &**inStmt {
        DAE::Statement::STMT_ASSIGN { exp1, .. } => {
            lhsCrefs = extractCrefsFromExpDerPreStart(exp1.clone(), false)?;
            lhsCrefs
        }
        DAE::Statement::STMT_TUPLE_ASSIGN { expExpLst: expLst, .. } => {
            lhsCrefs = List::flatten(List::map1(expLst.clone(), &extractCrefsFromExpDerPreStart, false)?)?;
            lhsCrefs
        }
        DAE::Statement::STMT_ASSIGN_ARR { lhs: exp1, .. } => {
            lhsCrefs = extractCrefsFromExpDerPreStart(exp1.clone(), false)?;
            lhsCrefs
        }
        DAE::Statement::STMT_IF {
            statementLst: stmtLst, ..
        } => {
            lhsCrefs = getLhsCrefsFromStatements(stmtLst.clone())?;
            lhsCrefs
        }
        DAE::Statement::STMT_FOR {
            statementLst: stmtLst, ..
        } => {
            lhsCrefs = getLhsCrefsFromStatements(stmtLst.clone())?;
            lhsCrefs
        }
        DAE::Statement::STMT_WHILE {
            statementLst: stmtLst, ..
        } => {
            lhsCrefs = getLhsCrefsFromStatements(stmtLst.clone())?;
            lhsCrefs
        }
        DAE::Statement::STMT_WHEN {
            statementLst: stmtLst, ..
        } => {
            lhsCrefs = getLhsCrefsFromStatements(stmtLst.clone())?;
            lhsCrefs
        }
        _ => metamodelica::nil(),
    });
    Ok(lhsCrefs)
}

pub fn expHasInitial(mut exp: metamodelica::Ref<DAE::Exp>) -> Result<bool> {
    let mut found: bool;
    (_, found) = traverseExpTopDown(
        exp,
        &fnptr!(traversingexpHasInitial, metamodelica::Ref<DAE::Exp>, bool),
        false,
    )?;
    Ok(found)
}

pub(crate) fn traversingexpHasInitial(
    mut exp: metamodelica::Ref<DAE::Exp>,
    mut found: bool,
) -> (metamodelica::Ref<DAE::Exp>, bool, bool) {
    let mut exp: metamodelica::Ref<DAE::Exp> = exp;
    let mut cont: bool;
    let mut found: bool = found;
    if found {
        cont = false;
        return (exp, cont, found);
    }
    (cont, found) = (::match_deref::match_deref! { match &(&*exp) {
        Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "initial" }, .. } => (false, true),
        _ => (true, found),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    (exp, cont, found)
}

pub fn expHasCrefs(mut inExp: metamodelica::Ref<DAE::Exp>) -> Result<bool> {
    let mut hasCrefs: bool;
    hasCrefs = (match &*inExp {
        _ => {
            let mut b: bool;
            (_, b) = traverseExpTopDown(
                inExp,
                &fnptr!(traversingComponentRefPresent, metamodelica::Ref<DAE::Exp>, bool),
                false,
            )?;
            b
        }
    });
    Ok(hasCrefs)
}

pub fn traversingComponentRefPresent(
    mut inExp: metamodelica::Ref<DAE::Exp>,
    mut found: bool,
) -> (metamodelica::Ref<DAE::Exp>, bool, bool) {
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    let mut cont: bool;
    let mut outFound: bool;
    (outExp, cont, outFound) = (::match_deref::match_deref! { match &((inExp.clone(), found)) {
        (_, true) => (inExp, false, true),
        (Deref @ DAE::Exp::CREF { .. }, _) => (inExp, false, true),
        _ => (inExp, true, false),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    (outExp, cont, outFound)
}

pub fn traversingComponentRefFinderNoPreDer(
    mut inExp: metamodelica::Ref<DAE::Exp>,
    mut inCrefs: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
) -> Result<(
    metamodelica::Ref<DAE::Exp>,
    bool,
    metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
)> {
    let mut e: metamodelica::Ref<DAE::Exp>;
    let mut cont: bool;
    let mut crefs: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
    (e, cont, crefs) = (::match_deref::match_deref! { match &((inExp.clone(), inCrefs.clone())) {
        (Deref @ DAE::Exp::CREF { componentRef: cr, .. }, __esc_crefs) => {
            crefs = (*__esc_crefs).clone();
            crefs = List::unionEltOnTrue(cr.clone(), crefs.clone(), &move |__a0: metamodelica::Ref<DAE::ComponentRef>, __a1: metamodelica::Ref<DAE::ComponentRef>| ComponentReferenceBasics::crefEqual(&__a0, &__a1))?;
            (inExp, false, crefs.clone())
        },
        (Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "der" }, .. }, _) => {
            (inExp, false, inCrefs)
        },
        (Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "pre" }, .. }, _) => {
            (inExp, false, inCrefs)
        },
        (Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "previous" }, .. }, _) => {
            (inExp, false, inCrefs)
        },
        _ => {
            (inExp, true, inCrefs)
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((e, cont, crefs))
}

pub fn expHasCref(
    mut inExp: metamodelica::Ref<DAE::Exp>,
    mut inCr: metamodelica::Ref<DAE::ComponentRef>,
) -> Result<bool> {
    let mut hasCref: bool;
    let (_, (_, __pa0)) = traverseExpTopDown(
        inExp,
        &fnptr!(
            traversingexpHasCref,
            metamodelica::Ref<DAE::Exp>,
            (metamodelica::Ref<DAE::ComponentRef>, bool)
        ),
        (inCr, false),
    )?;
    hasCref = metamodelica::Own::own(__pa0);
    Ok(hasCref)
}

pub(crate) fn traversingexpHasCref(
    mut inExp: metamodelica::Ref<DAE::Exp>,
    mut inTpl: (metamodelica::Ref<DAE::ComponentRef>, bool),
) -> (
    metamodelica::Ref<DAE::Exp>,
    bool,
    (metamodelica::Ref<DAE::ComponentRef>, bool),
) {
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    let mut cont: bool;
    let mut outTpl: (metamodelica::Ref<DAE::ComponentRef>, bool);
    (outExp, cont, outTpl) = 'mc: {
        let __mc_input = (&*inExp, &inTpl);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "pre" }, .. }, _) => {
                    Ok((inExp.clone(), false, inTpl.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CREF { componentRef: cr1, .. }, (cr, false)) => {
                    let mut b: bool;
                    b = ComponentReferenceBasics::crefEqualNoStringCompare(metamodelica::AsArg::as_arg(&cr), metamodelica::AsArg::as_arg(&cr1))?;
                    Ok((inExp.clone(), !(b), if (b) {(cr.clone(), b)} else {inTpl.clone()}))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, (_, b)) => {
                    Ok((inExp.clone(), !(b.clone()), inTpl.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    (outExp, cont, outTpl)
}

pub(crate) fn expHasCrefName(mut inExp: metamodelica::Ref<DAE::Exp>, mut name: ArcStr) -> Result<bool> {
    let mut hasCref: bool;
    let (_, (_, __pa0)) = traverseExpTopDown(inExp, &traversingexpHasName, (name, false))?;
    hasCref = metamodelica::Own::own(__pa0);
    Ok(hasCref)
}

pub fn anyExpHasCrefName(
    mut inExps: &metamodelica::List<metamodelica::Ref<DAE::Exp>>,
    mut name: ArcStr,
) -> Result<bool> {
    let mut hasCref: bool;
    hasCref = List::applyAndFold1(inExps, &fnptr!(boolOr, bool, bool), &expHasCrefName, name, false)?;
    Ok(hasCref)
}

pub(crate) fn traversingexpHasName(
    mut inExp: metamodelica::Ref<DAE::Exp>,
    mut inTpl: (ArcStr, bool),
) -> Result<(metamodelica::Ref<DAE::Exp>, bool, (ArcStr, bool))> {
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    let mut cont: bool;
    let mut outTpl: (ArcStr, bool);
    (outExp, cont, outTpl) = (::match_deref::match_deref! { match &((inExp.clone(), inTpl.clone())) {
        (Deref @ DAE::Exp::CREF { componentRef: cr, .. }, (name, false)) => {
            let mut b: bool;
            b = metamodelica::stringEq(&name, &(ComponentReferenceBasics::crefFirstIdent(metamodelica::AsArg::as_arg(&cr))?));
            (inExp, !(b), if (b) {(name.clone(), b)} else {inTpl})
        },
        (_, (_, b)) => {
            (inExp, !(b.clone()), inTpl)
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outExp, cont, outTpl))
}

pub fn expHasDerCref(
    mut inExp: metamodelica::Ref<DAE::Exp>,
    mut inCr: metamodelica::Ref<DAE::ComponentRef>,
) -> Result<bool> {
    let mut hasCref: bool;
    let (_, (_, __pa0)) = traverseExpTopDown(
        inExp,
        &fnptr!(
            traversingexpHasDerCref,
            metamodelica::Ref<DAE::Exp>,
            (metamodelica::Ref<DAE::ComponentRef>, bool)
        ),
        (inCr, false),
    )?;
    hasCref = metamodelica::Own::own(__pa0);
    Ok(hasCref)
}

pub(crate) fn traversingexpHasDerCref(
    mut inExp: metamodelica::Ref<DAE::Exp>,
    mut inTpl: (metamodelica::Ref<DAE::ComponentRef>, bool),
) -> (
    metamodelica::Ref<DAE::Exp>,
    bool,
    (metamodelica::Ref<DAE::ComponentRef>, bool),
) {
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    let mut cont: bool;
    let mut outTpl: (metamodelica::Ref<DAE::ComponentRef>, bool);
    (outExp, cont, outTpl) = 'mc: {
        let __mc_input = (&*inExp, &inTpl);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "der" }, expLst: Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Exp::CREF { componentRef: cr1, .. }, tail: Deref @ metamodelica::ListNode::Nil }, .. }, (cr, false)) => {
                    let mut b: bool;
                    b = ComponentReferenceBasics::crefEqualNoStringCompare(metamodelica::AsArg::as_arg(&cr), metamodelica::AsArg::as_arg(&cr1))?;
                    Ok((inExp.clone(), !(b), if (b) {(cr.clone(), b)} else {inTpl.clone()}))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "der" }, expLst: Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Exp::CREF { componentRef: cr1, .. }, tail: Deref @ metamodelica::ListNode::Nil }, .. }, (cr, false)) => {
                    let mut b: bool;
                    b = ComponentReferenceBasics::crefPrefixOf(metamodelica::AsArg::as_arg(&cr), metamodelica::AsArg::as_arg(&cr1))?;
                    Ok((inExp.clone(), !(b), if (b) {(cr.clone(), b)} else {inTpl.clone()}))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, (_, b)) => {
                    Ok((inExp.clone(), !(b.clone()), inTpl.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    (outExp, cont, outTpl)
}

pub fn expHasDer(mut inExp: metamodelica::Ref<DAE::Exp>) -> Result<bool> {
    let mut hasCref: bool;
    (_, hasCref) = traverseExpTopDown(inExp, &traversingexpHasDer, false)?;
    Ok(hasCref)
}

pub(crate) fn traversingexpHasDer(
    mut inExp: metamodelica::Ref<DAE::Exp>,
    mut inTpl: bool,
) -> Result<(metamodelica::Ref<DAE::Exp>, bool, bool)> {
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    let mut cont: bool;
    let mut outTpl: bool;
    (outExp, cont, outTpl) = (::match_deref::match_deref! { match &((inExp.clone(), inTpl)) {
        (Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "der" }, .. }, false) => {
            (inExp, false, true)
        },
        (Deref @ DAE::Exp::CREF { componentRef: cr, .. }, false) if (intEq(System::strncmp(ComponentReferenceBasics::crefFirstIdent(metamodelica::AsArg::as_arg(&cr))?, literal!("$DERAlias"), 9), 0)) => {
            (inExp, false, true)
        },
        (_, b) => {
            (inExp, !(b.clone()), inTpl)
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outExp, cont, outTpl))
}

pub fn expHasPre(mut inExp: metamodelica::Ref<DAE::Exp>) -> Result<bool> {
    let mut hasPre: bool;
    (_, hasPre) = traverseExpTopDown(
        inExp,
        &fnptr!(traversingexpHasPre, metamodelica::Ref<DAE::Exp>, bool),
        false,
    )?;
    Ok(hasPre)
}

fn traversingexpHasPre(
    mut inExp: metamodelica::Ref<DAE::Exp>,
    mut inHasIt: bool,
) -> (metamodelica::Ref<DAE::Exp>, bool, bool) {
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    let mut cont: bool;
    let mut outHasIt: bool;
    (outExp, cont, outHasIt) = (::match_deref::match_deref! { match &((inExp.clone(), inHasIt)) {
        (Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "pre" }, .. }, false) => {
            (inExp, false, true)
        },
        (_, b) => {
            (inExp, !(b.clone()), inHasIt)
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    (outExp, cont, outHasIt)
}

pub fn expHasPrevious(mut inExp: metamodelica::Ref<DAE::Exp>) -> Result<bool> {
    let mut hasPre: bool;
    (_, hasPre) = traverseExpTopDown(
        inExp,
        &fnptr!(traversingexpHasPrevious, metamodelica::Ref<DAE::Exp>, bool),
        false,
    )?;
    Ok(hasPre)
}

fn traversingexpHasPrevious(
    mut inExp: metamodelica::Ref<DAE::Exp>,
    mut inHasIt: bool,
) -> (metamodelica::Ref<DAE::Exp>, bool, bool) {
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    let mut cont: bool;
    let mut outHasIt: bool;
    (outExp, cont, outHasIt) = (::match_deref::match_deref! { match &((inExp.clone(), inHasIt)) {
        (Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "previous" }, .. }, false) => {
            (inExp, false, true)
        },
        (_, b) => {
            (inExp, !(b.clone()), inHasIt)
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    (outExp, cont, outHasIt)
}

pub fn expHasCrefNoPreorDer(
    mut inExp: metamodelica::Ref<DAE::Exp>,
    mut inCr: metamodelica::Ref<DAE::ComponentRef>,
) -> Result<bool> {
    let mut hasCref: bool;
    let (_, (_, __pa0)) = traverseExpTopDown(inExp, &traversingexpHasCrefNoPreorDer, (inCr, false))?;
    hasCref = metamodelica::Own::own(__pa0);
    Ok(hasCref)
}

pub(crate) fn traversingexpHasCrefNoPreorDer(
    mut inExp: metamodelica::Ref<DAE::Exp>,
    mut inTpl: (metamodelica::Ref<DAE::ComponentRef>, bool),
) -> Result<(
    metamodelica::Ref<DAE::Exp>,
    bool,
    (metamodelica::Ref<DAE::ComponentRef>, bool),
)> {
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    let mut cont: bool;
    let mut outTpl: (metamodelica::Ref<DAE::ComponentRef>, bool);
    (outExp, cont, outTpl) = (::match_deref::match_deref! { match &((inExp.clone(), inTpl.clone())) {
        (Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "pre" }, .. }, _) => {
            (inExp, false, inTpl)
        },
        (Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "previous" }, .. }, _) => {
            (inExp, false, inTpl)
        },
        (Deref @ DAE::Exp::CREF { componentRef: cr1, .. }, (cr, false)) => {
            let mut b: bool;
            b = ComponentReferenceBasics::crefEqualNoStringCompare(metamodelica::AsArg::as_arg(&cr), metamodelica::AsArg::as_arg(&cr1))?;
            (inExp, !(b), if (b) {(cr.clone(), b)} else {inTpl})
        },
        (_, (_, b)) => {
            (inExp, !(b.clone()), inTpl)
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outExp, cont, outTpl))
}

pub fn expHasCrefsNoPreOrStart(
    mut inExp: metamodelica::Ref<DAE::Exp>,
    mut inCr: &metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
) -> Result<bool> {
    let mut hasCref: bool = false;
    for mut cr in &**inCr {
        let (_, (_, __pa0)) =
            traverseExpTopDown(inExp.clone(), &traversingexpHasCrefNoPreOrStart, (cr.clone(), false))?;
        hasCref = metamodelica::Own::own(__pa0);
        if hasCref {
            break;
        }
    }
    Ok(hasCref)
}

pub fn expHasCrefNoPreOrStart(
    mut inExp: metamodelica::Ref<DAE::Exp>,
    mut inCr: metamodelica::Ref<DAE::ComponentRef>,
) -> Result<bool> {
    let mut hasCref: bool;
    let (_, (_, __pa0)) = traverseExpTopDown(inExp, &traversingexpHasCrefNoPreOrStart, (inCr, false))?;
    hasCref = metamodelica::Own::own(__pa0);
    Ok(hasCref)
}

fn traversingexpHasCrefNoPreOrStart(
    mut inExp: metamodelica::Ref<DAE::Exp>,
    mut inTpl: (metamodelica::Ref<DAE::ComponentRef>, bool),
) -> Result<(
    metamodelica::Ref<DAE::Exp>,
    bool,
    (metamodelica::Ref<DAE::ComponentRef>, bool),
)> {
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    let mut cont: bool;
    let mut outTpl: (metamodelica::Ref<DAE::ComponentRef>, bool);
    (outExp, cont, outTpl) = (::match_deref::match_deref! { match &((inExp.clone(), inTpl.clone())) {
        (Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "pre" }, .. }, _) => {
            (inExp, false, inTpl)
        },
        (Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "previous" }, .. }, _) => {
            (inExp, false, inTpl)
        },
        (Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "change" }, .. }, _) => {
            (inExp, false, inTpl)
        },
        (Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "delay" }, .. }, _) => {
            (inExp, false, inTpl)
        },
        (Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "edge" }, .. }, _) => {
            (inExp, false, inTpl)
        },
        (Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "$_round" }, .. }, _) => {
            (inExp, false, inTpl)
        },
        (Deref @ DAE::Exp::CREF { componentRef: cr1, .. }, (cr, false)) => {
            let mut b: bool;
            b = ComponentReferenceBasics::crefEqualNoStringCompare(metamodelica::AsArg::as_arg(&cr), metamodelica::AsArg::as_arg(&cr1))?;
            (inExp, !(b), if (b) {(cr.clone(), b)} else {inTpl})
        },
        (_, (_, b)) => {
            (inExp, !(b.clone()), inTpl)
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outExp, cont, outTpl))
}

pub fn expHasCrefInIf(
    mut inExp: metamodelica::Ref<DAE::Exp>,
    mut inCr: metamodelica::Ref<DAE::ComponentRef>,
) -> Result<bool> {
    let mut hasCref: bool;
    let (_, (_, __pa0)) = traverseExpTopDown(inExp, &expHasCrefInIfWork, (inCr, false))?;
    hasCref = metamodelica::Own::own(__pa0);
    Ok(hasCref)
}

pub(crate) fn expHasCrefInIfWork(
    mut inExp: metamodelica::Ref<DAE::Exp>,
    mut inTpl: (metamodelica::Ref<DAE::ComponentRef>, bool),
) -> Result<(
    metamodelica::Ref<DAE::Exp>,
    bool,
    (metamodelica::Ref<DAE::ComponentRef>, bool),
)> {
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    let mut cont: bool;
    let mut outTpl: (metamodelica::Ref<DAE::ComponentRef>, bool);
    (outExp, cont, outTpl) = (::match_deref::match_deref! { match &((inExp.clone(), inTpl.clone())) {
        (Deref @ DAE::Exp::IFEXP { expCond: e1, expThen: _, expElse: _ }, (cr, false)) if (!(isFunCall(metamodelica::AsArg::as_arg(&e1), &(literal!("noEvent"))))) => {
            let mut b: bool;
            b = expHasCref(e1.clone(), cr.clone())?;
            (e1.clone(), true, if (b) {(cr.clone(), b)} else {inTpl})
        },
        (Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "smooth" }, expLst: Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Exp::ICONST { integer: i }, tail: Deref @ metamodelica::ListNode::Cons { head: e1, tail: Deref @ metamodelica::ListNode::Nil } }, .. }, (cr, false)) if (i.clone() > 1) => {
            (e1.clone(), true, (cr.clone(), expHasCref(e1.clone(), cr.clone())?))
        },
        (Deref @ DAE::Exp::CALL { .. }, (cr, false)) if (isEventTriggeringFunctionExp(&inExp)) => {
            let mut b: bool;
            b = expHasCref(inExp.clone(), cr.clone())?;
            (inExp.clone(), true, if (b) {(cr.clone(), b)} else {inTpl})
        },
        (Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "semiLinear" }, expLst: Deref @ metamodelica::ListNode::Cons { head: e1, tail: Deref @ metamodelica::ListNode::Cons { head: _, tail: Deref @ metamodelica::ListNode::Cons { head: _, tail: Deref @ metamodelica::ListNode::Nil } } }, .. }, (cr, false)) => {
            let mut b: bool;
            b = expHasCref(e1.clone(), cr.clone())?;
            (e1.clone(), true, if (b) {(cr.clone(), b)} else {inTpl})
        },
        (Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "sign" }, expLst: Deref @ metamodelica::ListNode::Cons { head: e1, tail: Deref @ metamodelica::ListNode::Nil }, .. }, (cr, false)) => {
            let mut b: bool;
            b = expHasCref(e1.clone(), cr.clone())?;
            (e1.clone(), !(b), if (b) {(cr.clone(), b)} else {inTpl})
        },
        (_, (_, true)) => {
            (inExp.clone(), false, inTpl)
        },
        _ => {
            (inExp.clone(), true, inTpl)
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outExp, cont, outTpl))
}

pub fn expHasCrefInSmoothZero(
    mut exp: metamodelica::Ref<DAE::Exp>,
    mut cr: metamodelica::Ref<DAE::ComponentRef>,
) -> Result<bool> {
    let mut b: bool;
    let (_, (_, __pa0)) = traverseExpBottomUp(exp, &expHasCrefInSmoothZeroWork, (cr, false))?;
    b = metamodelica::Own::own(__pa0);
    Ok(b)
}

fn expHasCrefInSmoothZeroWork(
    mut exp: metamodelica::Ref<DAE::Exp>,
    mut tpl: (metamodelica::Ref<DAE::ComponentRef>, bool),
) -> Result<(
    metamodelica::Ref<DAE::Exp>,
    (metamodelica::Ref<DAE::ComponentRef>, bool),
)> {
    let mut exp: metamodelica::Ref<DAE::Exp> = exp;
    let mut tpl: (metamodelica::Ref<DAE::ComponentRef>, bool) = tpl;
    tpl = (::match_deref::match_deref! { match &((exp.clone(), tpl.clone())) {
        (Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "smooth" }, expLst: Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Exp::ICONST { integer: 0 }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Exp::CREF { componentRef: sCr, .. }, tail: Deref @ metamodelica::ListNode::Nil } }, .. }, (cr, false)) => {
            let mut b: bool;
            b = ComponentReferenceBasics::crefEqual(metamodelica::AsArg::as_arg(&sCr), metamodelica::AsArg::as_arg(&cr))?;
            (cr.clone(), b)
        },
        _ => {
            tpl
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((exp, tpl))
}

pub fn traverseCrefsFromExp<Type_a: Clone + 'static + metamodelica::gc::MMTrace + metamodelica::ReferenceEq>(
    mut inExp: metamodelica::Ref<DAE::Exp>,
    mut inFunc: Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>, Type_a) -> Result<Type_a> + 'static>,
    mut inArg: Type_a,
) -> Result<Type_a> {
    pub type FuncCrefTypeA<Type_a: Clone + 'static> =
        std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>, Type_a) -> Result<Type_a> + 'static>;

    let mut outArg: Type_a;
    outArg = (match inArg.clone() {
        _ => {
            let mut arg: Type_a;
            let (_, (_, __pa0)) = traverseExpBottomUp(inExp, &traversingCrefFinder, (inFunc.clone(), inArg))?;
            arg = metamodelica::Own::own(__pa0);
            arg
        }
    });
    Ok(outArg)
}

fn traversingCrefFinder<Type_a: Clone + 'static + metamodelica::gc::MMTrace + metamodelica::ReferenceEq>(
    mut inExp: metamodelica::Ref<DAE::Exp>,
    mut inTpl: (
        Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>, Type_a) -> Result<Type_a> + 'static>,
        Type_a,
    ),
) -> Result<(
    metamodelica::Ref<DAE::Exp>,
    (
        Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>, Type_a) -> Result<Type_a> + 'static>,
        Type_a,
    ),
)> {
    pub type FuncCrefTypeA<Type_a: Clone + 'static> =
        std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>, Type_a) -> Result<Type_a> + 'static>;

    let mut outExp: metamodelica::Ref<DAE::Exp>;
    let mut outTpl: (
        Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>, Type_a) -> Result<Type_a> + 'static>,
        Type_a,
    );
    (outExp, outTpl) = (::match_deref::match_deref! { match &((inExp.clone(), inTpl.clone())) {
        (Deref @ DAE::Exp::CREF { componentRef: cr, ty: _ }, (func, arg)) => {
            let mut arg1: Type_a;
            arg1 = func(cr.clone(), arg.clone())?;
            (inExp, if (metamodelica::ReferenceEq::reference_eq(&(arg.clone()), &(arg1.clone()))) {inTpl} else {(func.clone(), arg1)})
        },
        _ => {
            (inExp, inTpl)
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outExp, outTpl))
}

pub(crate) fn extractDivExpFromExp(
    mut inExp: metamodelica::Ref<DAE::Exp>,
) -> Result<metamodelica::List<metamodelica::Ref<DAE::Exp>>> {
    let mut outExps: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
    (_, outExps) = traverseExpBottomUp(
        inExp,
        &fnptr!(
            traversingDivExpFinder,
            metamodelica::Ref<DAE::Exp>,
            metamodelica::List<metamodelica::Ref<DAE::Exp>>
        ),
        metamodelica::nil(),
    )?;
    Ok(outExps)
}

fn traversingDivExpFinder(
    mut e: metamodelica::Ref<DAE::Exp>,
    mut exps: metamodelica::List<metamodelica::Ref<DAE::Exp>>,
) -> (
    metamodelica::Ref<DAE::Exp>,
    metamodelica::List<metamodelica::Ref<DAE::Exp>>,
) {
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    let mut acc: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
    (outExp, acc) = (match &*e.clone() {
        DAE::Exp::BINARY {
            operator: DAE::Operator::DIV { ty: _ },
            exp2: e2,
            ..
        } => (e, metamodelica::cons(e2.clone(), exps)),
        DAE::Exp::BINARY {
            operator: DAE::Operator::DIV_ARR { ty: _ },
            exp2: e2,
            ..
        } => (e, metamodelica::cons(e2.clone(), exps)),
        DAE::Exp::BINARY {
            operator: DAE::Operator::DIV_ARRAY_SCALAR { ty: _ },
            exp2: e2,
            ..
        } => (e, metamodelica::cons(e2.clone(), exps)),
        DAE::Exp::BINARY {
            operator: DAE::Operator::DIV_SCALAR_ARRAY { ty: _ },
            exp2: e2,
            ..
        } => (e, metamodelica::cons(e2.clone(), exps)),
        _ => (e, exps),
    });
    (outExp, acc)
}

pub(crate) fn traverseExpListBidir<ArgT: Clone + 'static + metamodelica::gc::MMTrace>(
    mut inExpl: metamodelica::List<metamodelica::Ref<DAE::Exp>>,
    mut inEnterFunc: Arc<
        dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>, ArgT) -> Result<(metamodelica::Ref<DAE::Exp>, ArgT)> + 'static,
    >,
    mut inExitFunc: Arc<
        dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>, ArgT) -> Result<(metamodelica::Ref<DAE::Exp>, ArgT)> + 'static,
    >,
    mut inArg: ArgT,
) -> Result<(metamodelica::List<metamodelica::Ref<DAE::Exp>>, ArgT)> {
    pub type FuncType<ArgT: Clone + 'static> = std::sync::Arc<
        dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>, ArgT) -> Result<(metamodelica::Ref<DAE::Exp>, ArgT)> + 'static,
    >;

    let mut outExpl: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
    let mut outArg: ArgT = inArg;
    let mut e1: metamodelica::Ref<DAE::Exp>;
    let mut delst: DoubleEnded::MutableList<metamodelica::Ref<DAE::Exp>>;
    let mut rest: metamodelica::List<metamodelica::Ref<DAE::Exp>> = inExpl.clone();
    let mut nEq: i32 = 0;
    outExpl = inExpl.clone();
    while !((rest).is_empty()) {
        (e1, outArg) = traverseExpBidir((rest).head().cloned()?, inEnterFunc.clone(), inExitFunc.clone(), outArg)?;
        if !(referenceEq(&*((rest).head().cloned()?), &*(&*e1))) {
            delst = DoubleEnded::empty(e1.clone());
            for mut elt in &*inExpl {
                if nEq < 1 {
                    break;
                }
                DoubleEnded::push_back(delst.clone(), elt.clone())?;
                nEq = nEq - 1;
            }
            DoubleEnded::push_back(delst.clone(), e1)?;
            for mut e in &*(rest).rest()? {
                (e1, outArg) = traverseExpBidir(e.clone(), inEnterFunc.clone(), inExitFunc.clone(), outArg)?;
                DoubleEnded::push_back(delst.clone(), e1)?;
            }
            outExpl = DoubleEnded::toListAndClear(delst, metamodelica::nil())?;
            return Ok((outExpl, outArg));
        }
        nEq = nEq + 1;
        rest = (rest).rest()?;
    }
    Ok((outExpl, outArg))
}

pub fn traverseExpBidir<ArgT: Clone + 'static + metamodelica::gc::MMTrace>(
    mut inExp: metamodelica::Ref<DAE::Exp>,
    mut inEnterFunc: Arc<
        dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>, ArgT) -> Result<(metamodelica::Ref<DAE::Exp>, ArgT)> + 'static,
    >,
    mut inExitFunc: Arc<
        dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>, ArgT) -> Result<(metamodelica::Ref<DAE::Exp>, ArgT)> + 'static,
    >,
    mut inArg: ArgT,
) -> Result<(metamodelica::Ref<DAE::Exp>, ArgT)> {
    pub type FuncType<ArgT: Clone + 'static> = std::sync::Arc<
        dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>, ArgT) -> Result<(metamodelica::Ref<DAE::Exp>, ArgT)> + 'static,
    >;

    let mut outExp: metamodelica::Ref<DAE::Exp>;
    let mut outArg: ArgT;
    (outExp, outArg) = inEnterFunc(inExp, inArg)?;
    (outExp, outArg) = traverseExpBidirSubExps(outExp, inEnterFunc.clone(), inExitFunc.clone(), outArg)?;
    (outExp, outArg) = inExitFunc(outExp, outArg)?;
    Ok((outExp, outArg))
}

pub(crate) fn traverseExpOptBidir<ArgT: Clone + 'static + metamodelica::gc::MMTrace>(
    mut inExp: Option<metamodelica::Ref<DAE::Exp>>,
    mut inEnterFunc: Arc<
        dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>, ArgT) -> Result<(metamodelica::Ref<DAE::Exp>, ArgT)> + 'static,
    >,
    mut inExitFunc: Arc<
        dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>, ArgT) -> Result<(metamodelica::Ref<DAE::Exp>, ArgT)> + 'static,
    >,
    mut inArg: ArgT,
) -> Result<(Option<metamodelica::Ref<DAE::Exp>>, ArgT)> {
    pub type FuncType<ArgT: Clone + 'static> = std::sync::Arc<dyn ::std::ops::Fn(Exp, ArgT) -> Result<Exp> + 'static>;

    let mut outExp: Option<metamodelica::Ref<DAE::Exp>>;
    let mut outArg: ArgT;
    (outExp, outArg) = (::match_deref::match_deref! { match &(inExp.clone()) {
        Some(e) => {
            let mut e1: metamodelica::Ref<DAE::Exp>;
            let mut arg: ArgT;
            (e1, arg) = traverseExpBidir(e.clone(), inEnterFunc.clone(), inExitFunc.clone(), inArg)?;
            (if (referenceEq(&*(e.clone()),&*(&*e1))) {inExp} else {Some(e1)}, arg)
        },
        _ => {
            (inExp, inArg)
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outExp, outArg))
}

fn traverseExpBidirSubExps<ArgT: Clone + 'static + metamodelica::gc::MMTrace>(
    mut inExp: metamodelica::Ref<DAE::Exp>,
    mut inEnterFunc: Arc<
        dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>, ArgT) -> Result<(metamodelica::Ref<DAE::Exp>, ArgT)> + 'static,
    >,
    mut inExitFunc: Arc<
        dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>, ArgT) -> Result<(metamodelica::Ref<DAE::Exp>, ArgT)> + 'static,
    >,
    mut inArg: ArgT,
) -> Result<(metamodelica::Ref<DAE::Exp>, ArgT)> {
    pub type FuncType<ArgT: Clone + 'static> = std::sync::Arc<
        dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>, ArgT) -> Result<(metamodelica::Ref<DAE::Exp>, ArgT)> + 'static,
    >;

    let mut outExp: metamodelica::Ref<DAE::Exp>;
    let mut outArg: ArgT;
    (outExp, outArg) = (::match_deref::match_deref! { match &(inExp.clone()) {
        Deref @ DAE::Exp::ICONST { .. } => {
            (inExp, inArg)
        },
        Deref @ DAE::Exp::RCONST { .. } => {
            (inExp, inArg)
        },
        Deref @ DAE::Exp::SCONST { .. } => {
            (inExp, inArg)
        },
        Deref @ DAE::Exp::BCONST { .. } => {
            (inExp, inArg)
        },
        Deref @ DAE::Exp::ENUM_LITERAL { .. } => {
            (inExp, inArg)
        },
        Deref @ DAE::Exp::CREF { componentRef: cref, ty } => {
            let mut cref_1: ComponentRef;
            let mut arg: ArgT;
            (cref_1, arg) = traverseExpBidirCref(metamodelica::AsArg::as_arg(&cref), inEnterFunc.clone(), inExitFunc.clone(), inArg)?;
            (if (referenceEq(&*(cref.clone()),&*(&*cref_1))) {inExp} else {metamodelica::Ref::new(DAE::Exp::CREF { componentRef: cref_1, ty: ty.clone() })}, arg)
        },
        Deref @ DAE::Exp::BINARY { exp1: e1, operator: op, exp2: e2 } => {
            let mut e1_1: metamodelica::Ref<DAE::Exp>;
            let mut e2_1: metamodelica::Ref<DAE::Exp>;
            let mut arg: ArgT;
            (e1_1, arg) = traverseExpBidir(e1.clone(), inEnterFunc.clone(), inExitFunc.clone(), inArg)?;
            (e2_1, arg) = traverseExpBidir(e2.clone(), inEnterFunc.clone(), inExitFunc.clone(), arg)?;
            (if (referenceEq(&*(e1.clone()),&*(&*e1_1)) && referenceEq(&*(e2.clone()),&*(&*e2_1))) {inExp} else {metamodelica::Ref::new(DAE::Exp::BINARY { exp1: e1_1, operator: op.clone(), exp2: e2_1 })}, arg)
        },
        Deref @ DAE::Exp::UNARY { operator: op, exp: e1 } => {
            let mut e1_1: metamodelica::Ref<DAE::Exp>;
            let mut arg: ArgT;
            (e1_1, arg) = traverseExpBidir(e1.clone(), inEnterFunc.clone(), inExitFunc.clone(), inArg)?;
            (if (referenceEq(&*(e1.clone()),&*(e1_1))) {inExp} else {metamodelica::Ref::new(DAE::Exp::UNARY { operator: op.clone(), exp: e1.clone() })}, arg)
        },
        Deref @ DAE::Exp::LBINARY { exp1: e1, operator: op, exp2: e2 } => {
            let mut e1_1: metamodelica::Ref<DAE::Exp>;
            let mut e2_1: metamodelica::Ref<DAE::Exp>;
            let mut arg: ArgT;
            (e1_1, arg) = traverseExpBidir(e1.clone(), inEnterFunc.clone(), inExitFunc.clone(), inArg)?;
            (e2_1, arg) = traverseExpBidir(e2.clone(), inEnterFunc.clone(), inExitFunc.clone(), arg)?;
            (if (referenceEq(&*(e1.clone()),&*(&*e1_1)) && referenceEq(&*(e2.clone()),&*(&*e2_1))) {inExp} else {metamodelica::Ref::new(DAE::Exp::LBINARY { exp1: e1_1, operator: op.clone(), exp2: e2_1 })}, arg)
        },
        Deref @ DAE::Exp::LUNARY { operator: op, exp: e1 } => {
            let mut e1_1: metamodelica::Ref<DAE::Exp>;
            let mut arg: ArgT;
            (e1_1, arg) = traverseExpBidir(e1.clone(), inEnterFunc.clone(), inExitFunc.clone(), inArg)?;
            (if (referenceEq(&*(e1.clone()),&*(e1_1))) {inExp} else {metamodelica::Ref::new(DAE::Exp::LUNARY { operator: op.clone(), exp: e1.clone() })}, arg)
        },
        Deref @ DAE::Exp::RELATION { exp1: e1, operator: op, exp2: e2, index, optionExpisASUB: opt_exp_asub } => {
            let mut e1_1: metamodelica::Ref<DAE::Exp>;
            let mut e2_1: metamodelica::Ref<DAE::Exp>;
            let mut arg: ArgT;
            (e1_1, arg) = traverseExpBidir(e1.clone(), inEnterFunc.clone(), inExitFunc.clone(), inArg)?;
            (e2_1, arg) = traverseExpBidir(e2.clone(), inEnterFunc.clone(), inExitFunc.clone(), arg)?;
            (if (referenceEq(&*(e1.clone()),&*(&*e1_1)) && referenceEq(&*(e2.clone()),&*(&*e2_1))) {inExp} else {metamodelica::Ref::new(DAE::Exp::RELATION { exp1: e1_1, operator: op.clone(), exp2: e2_1, index: index.clone(), optionExpisASUB: opt_exp_asub.clone() })}, arg)
        },
        Deref @ DAE::Exp::IFEXP { expCond: e1, expThen: e2, expElse: e3 } => {
            let mut e1_1: metamodelica::Ref<DAE::Exp>;
            let mut e2_1: metamodelica::Ref<DAE::Exp>;
            let mut e3_1: metamodelica::Ref<DAE::Exp>;
            let mut arg: ArgT;
            (e1_1, arg) = traverseExpBidir(e1.clone(), inEnterFunc.clone(), inExitFunc.clone(), inArg)?;
            (e2_1, arg) = traverseExpBidir(e2.clone(), inEnterFunc.clone(), inExitFunc.clone(), arg)?;
            (e3_1, arg) = traverseExpBidir(e3.clone(), inEnterFunc.clone(), inExitFunc.clone(), arg)?;
            (if (referenceEq(&*(e1.clone()),&*(&*e1_1)) && referenceEq(&*(e2.clone()),&*(&*e2_1)) && referenceEq(&*(e3.clone()),&*(&*e3_1))) {inExp} else {metamodelica::Ref::new(DAE::Exp::IFEXP { expCond: e1_1, expThen: e2_1, expElse: e3_1 })}, arg)
        },
        Deref @ DAE::Exp::CALL { path, expLst: expl, attr } => {
            let mut expl_1: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
            let mut arg: ArgT;
            (expl_1, arg) = traverseExpListBidir(expl.clone(), inEnterFunc.clone(), inExitFunc.clone(), inArg)?;
            (if (metamodelica::ReferenceEq::reference_eq(&(expl.clone()), &(expl_1))) {inExp} else {metamodelica::Ref::new(DAE::Exp::CALL { path: path.clone(), expLst: expl_1, attr: attr.clone() })}, arg)
        },
        Deref @ DAE::Exp::RECORD { path, exps: expl, comp: strl, ty } => {
            let mut expl_1: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
            let mut arg: ArgT;
            (expl_1, arg) = traverseExpListBidir(expl.clone(), inEnterFunc.clone(), inExitFunc.clone(), inArg)?;
            (if (metamodelica::ReferenceEq::reference_eq(&(expl.clone()), &(expl_1))) {inExp} else {metamodelica::Ref::new(DAE::Exp::RECORD { path: path.clone(), exps: expl_1, comp: strl.clone(), ty: ty.clone() })}, arg)
        },
        Deref @ DAE::Exp::PARTEVALFUNCTION { path, expList: expl, ty, origType: t } => {
            let mut expl_1: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
            let mut arg: ArgT;
            (expl_1, arg) = traverseExpListBidir(expl.clone(), inEnterFunc.clone(), inExitFunc.clone(), inArg)?;
            (if (metamodelica::ReferenceEq::reference_eq(&(expl.clone()), &(expl_1))) {inExp} else {metamodelica::Ref::new(DAE::Exp::PARTEVALFUNCTION { path: path.clone(), expList: expl_1, ty: ty.clone(), origType: t.clone() })}, arg)
        },
        Deref @ DAE::Exp::ARRAY { ty, scalar: b1, array: expl } => {
            let mut expl_1: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
            let mut arg: ArgT;
            (expl_1, arg) = traverseExpListBidir(expl.clone(), inEnterFunc.clone(), inExitFunc.clone(), inArg)?;
            (if (metamodelica::ReferenceEq::reference_eq(&(expl.clone()), &(expl_1))) {inExp} else {metamodelica::Ref::new(DAE::Exp::ARRAY { ty: ty.clone(), scalar: b1.clone(), array: expl_1 })}, arg)
        },
        Deref @ DAE::Exp::MATRIX { ty, integer: dim, matrix: mat_expl } => {
            let mut arg: ArgT;
            let mut mat_expl = (*mat_expl).clone();
            (mat_expl, arg) = List::map2Fold(metamodelica::AsArg::as_arg(&mat_expl), &traverseExpListBidir, inEnterFunc.clone(), inExitFunc.clone(), inArg, metamodelica::nil())?;
            (metamodelica::Ref::new(DAE::Exp::MATRIX { ty: ty.clone(), integer: dim.clone(), matrix: mat_expl.clone() }), arg)
        },
        Deref @ DAE::Exp::RANGE { ty, start: e1, step: oe1, stop: e2 } => {
            let mut e1_1: metamodelica::Ref<DAE::Exp>;
            let mut e2_1: metamodelica::Ref<DAE::Exp>;
            let mut oe1_1: Option<metamodelica::Ref<DAE::Exp>>;
            let mut arg: ArgT;
            (e1_1, arg) = traverseExpBidir(e1.clone(), inEnterFunc.clone(), inExitFunc.clone(), inArg)?;
            (oe1_1, arg) = traverseExpOptBidir(oe1.clone(), inEnterFunc.clone(), inExitFunc.clone(), arg)?;
            (e2_1, arg) = traverseExpBidir(e2.clone(), inEnterFunc.clone(), inExitFunc.clone(), arg)?;
            (if (referenceEq(&*(e1.clone()),&*(&*e1_1)) && referenceEq(&*(e2.clone()),&*(&*e2_1)) && (match (&(oe1), &(oe1_1)) { (None, None) => true, (Some(__refeq_l), Some(__refeq_r)) => referenceEq(&*(*__refeq_l),&*(*__refeq_r)), _ => false })) {inExp} else {metamodelica::Ref::new(DAE::Exp::RANGE { ty: ty.clone(), start: e1_1, step: oe1_1, stop: e2_1 })}, arg)
        },
        Deref @ DAE::Exp::TUPLE { PR: expl } => {
            let mut expl_1: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
            let mut arg: ArgT;
            (expl_1, arg) = traverseExpListBidir(expl.clone(), inEnterFunc.clone(), inExitFunc.clone(), inArg)?;
            (if (metamodelica::ReferenceEq::reference_eq(&(expl.clone()), &(expl_1))) {inExp} else {metamodelica::Ref::new(DAE::Exp::TUPLE { PR: expl_1 })}, arg)
        },
        Deref @ DAE::Exp::CAST { ty, exp: e1 } => {
            let mut e1_1: metamodelica::Ref<DAE::Exp>;
            let mut arg: ArgT;
            (e1_1, arg) = traverseExpBidir(e1.clone(), inEnterFunc.clone(), inExitFunc.clone(), inArg)?;
            (if (referenceEq(&*(e1.clone()),&*(e1_1))) {inExp} else {metamodelica::Ref::new(DAE::Exp::CAST { ty: ty.clone(), exp: e1.clone() })}, arg)
        },
        Deref @ DAE::Exp::ASUB { exp: e1, sub: subs } => {
            let mut e1_1: metamodelica::Ref<DAE::Exp>;
            let mut expl: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
            let mut expl_1: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
            let mut arg: ArgT;
            let mut subs = (*subs).clone();
            expl = ({
        let mut __acc: metamodelica::List<metamodelica::Ref<DAE::Exp>> = metamodelica::nil();
        for mut sub in (subs.clone()).into_iter().cloned() {
            let __x = getSubscriptExp(&(sub.clone()))?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
            (e1_1, arg) = traverseExpBidir(e1.clone(), inEnterFunc.clone(), inExitFunc.clone(), inArg)?;
            (expl_1, arg) = traverseExpListBidir(expl.clone(), inEnterFunc.clone(), inExitFunc.clone(), arg)?;
            subs = ({
        let mut __acc: metamodelica::List<metamodelica::Ref<DAE::Subscript>> = metamodelica::nil();
        for mut sub in (expl.clone()).into_iter().cloned() {
            let __x = makeIndexSubscript(sub.clone());
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
            (if (referenceEq(&*(e1.clone()),&*(&*e1_1)) && metamodelica::ReferenceEq::reference_eq(&(expl), &(expl_1))) {inExp} else {metamodelica::Ref::new(DAE::Exp::ASUB { exp: e1_1, sub: subs.clone() })}, arg)
        },
        e1 @ Deref @ DAE::Exp::RSUB { .. } => {
            let mut e2: metamodelica::Ref<DAE::Exp>;
            let mut arg: ArgT;
            let mut e1 = (*e1).clone();
            (e2, arg) = traverseExpBidir(var_field!((*e1).exp, DAE::Exp::RSUB).clone(), inEnterFunc.clone(), inExitFunc.clone(), inArg)?;
            if referenceEq(&*(var_field!((*e1).exp, DAE::Exp::RSUB).clone()),&*(&*e2)) {
                assign_variant_field!(e1 => DAE::Exp::RSUB; exp = e2);
            }
            (e1.clone(), arg)
        },
        Deref @ DAE::Exp::TSUB { exp: e1, ix: i, ty } => {
            let mut e1_1: metamodelica::Ref<DAE::Exp>;
            let mut arg: ArgT;
            (e1_1, arg) = traverseExpBidir(e1.clone(), inEnterFunc.clone(), inExitFunc.clone(), inArg)?;
            (if (referenceEq(&*(e1.clone()),&*(&*e1_1))) {inExp} else {metamodelica::Ref::new(DAE::Exp::TSUB { exp: e1_1, ix: i.clone(), ty: ty.clone() })}, arg)
        },
        Deref @ DAE::Exp::SIZE { exp: e1, sz: oe1 } => {
            let mut e1_1: metamodelica::Ref<DAE::Exp>;
            let mut oe1_1: Option<metamodelica::Ref<DAE::Exp>>;
            let mut arg: ArgT;
            (e1_1, arg) = traverseExpBidir(e1.clone(), inEnterFunc.clone(), inExitFunc.clone(), inArg)?;
            (oe1_1, arg) = traverseExpOptBidir(oe1.clone(), inEnterFunc.clone(), inExitFunc.clone(), arg)?;
            (if (referenceEq(&*(e1.clone()),&*(&*e1_1)) && (match (&(oe1), &(oe1_1)) { (None, None) => true, (Some(__refeq_l), Some(__refeq_r)) => referenceEq(&*(*__refeq_l),&*(*__refeq_r)), _ => false })) {inExp} else {metamodelica::Ref::new(DAE::Exp::SIZE { exp: e1_1, sz: oe1_1 })}, arg)
        },
        Deref @ DAE::Exp::CODE { .. } => {
            (inExp, inArg)
        },
        Deref @ DAE::Exp::REDUCTION { reductionInfo, expr: e1, iterators: riters } => {
            let mut e1_1: metamodelica::Ref<DAE::Exp>;
            let mut riters_1: metamodelica::List<metamodelica::Ref<DAE::ReductionIterator>>;
            let mut arg: ArgT;
            (e1_1, arg) = traverseExpBidir(e1.clone(), inEnterFunc.clone(), inExitFunc.clone(), inArg)?;
            (riters_1, arg) = List::map2Fold(metamodelica::AsArg::as_arg(&riters), &move |__a0: metamodelica::Ref<DAE::ReductionIterator>, __a1: _, __a2: _, __a3: _| traverseReductionIteratorBidir(&__a0, __a1, __a2, __a3), inEnterFunc.clone(), inExitFunc.clone(), arg, metamodelica::nil())?;
            (if (referenceEq(&*(e1.clone()),&*(e1_1)) && metamodelica::ReferenceEq::reference_eq(&(riters.clone()), &(riters_1))) {inExp} else {metamodelica::Ref::new(DAE::Exp::REDUCTION { reductionInfo: reductionInfo.clone(), expr: e1.clone(), iterators: riters.clone() })}, arg)
        },
        Deref @ DAE::Exp::LIST { valList: expl } => {
            let mut expl_1: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
            let mut arg: ArgT;
            (expl_1, arg) = traverseExpListBidir(expl.clone(), inEnterFunc.clone(), inExitFunc.clone(), inArg)?;
            (if (metamodelica::ReferenceEq::reference_eq(&(expl.clone()), &(expl_1))) {inExp} else {metamodelica::Ref::new(DAE::Exp::LIST { valList: expl_1 })}, arg)
        },
        Deref @ DAE::Exp::CONS { car: e1, cdr: e2 } => {
            let mut e1_1: metamodelica::Ref<DAE::Exp>;
            let mut e2_1: metamodelica::Ref<DAE::Exp>;
            let mut arg: ArgT;
            (e1_1, arg) = traverseExpBidir(e1.clone(), inEnterFunc.clone(), inExitFunc.clone(), inArg)?;
            (e2_1, arg) = traverseExpBidir(e2.clone(), inEnterFunc.clone(), inExitFunc.clone(), arg)?;
            (if (referenceEq(&*(e1.clone()),&*(&*e1_1)) && referenceEq(&*(e2.clone()),&*(&*e2_1))) {inExp} else {metamodelica::Ref::new(DAE::Exp::CONS { car: e1_1, cdr: e2_1 })}, arg)
        },
        Deref @ DAE::Exp::META_TUPLE { listExp: expl } => {
            let mut expl_1: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
            let mut arg: ArgT;
            (expl_1, arg) = traverseExpListBidir(expl.clone(), inEnterFunc.clone(), inExitFunc.clone(), inArg)?;
            (if (metamodelica::ReferenceEq::reference_eq(&(expl.clone()), &(expl_1))) {inExp} else {metamodelica::Ref::new(DAE::Exp::TUPLE { PR: expl_1 })}, arg)
        },
        Deref @ DAE::Exp::META_OPTION { exp: oe1 } => {
            let mut oe1_1: Option<metamodelica::Ref<DAE::Exp>>;
            let mut arg: ArgT;
            (oe1_1, arg) = traverseExpOptBidir(oe1.clone(), inEnterFunc.clone(), inExitFunc.clone(), inArg)?;
            (if ((match (&(oe1), &(oe1_1)) { (None, None) => true, (Some(__refeq_l), Some(__refeq_r)) => referenceEq(&*(*__refeq_l),&*(*__refeq_r)), _ => false })) {inExp} else {metamodelica::Ref::new(DAE::Exp::META_OPTION { exp: oe1_1 })}, arg)
        },
        Deref @ DAE::Exp::METARECORDCALL { path, args: expl, fieldNames: strl, index, typeVars } => {
            let mut expl_1: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
            let mut arg: ArgT;
            (expl_1, arg) = traverseExpListBidir(expl.clone(), inEnterFunc.clone(), inExitFunc.clone(), inArg)?;
            (if (metamodelica::ReferenceEq::reference_eq(&(expl.clone()), &(expl_1))) {inExp} else {metamodelica::Ref::new(DAE::Exp::METARECORDCALL { path: path.clone(), args: expl_1, fieldNames: strl.clone(), index: index.clone(), typeVars: typeVars.clone() })}, arg)
        },
        Deref @ DAE::Exp::MATCHEXPRESSION { matchType: match_ty, inputs: expl, aliases, localDecls: match_decls, cases: match_cases, et: ty } => {
            let mut expl_1: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
            let mut arg: ArgT;
            (expl_1, arg) = traverseExpListBidir(expl.clone(), inEnterFunc.clone(), inExitFunc.clone(), inArg)?;
            Error::addSourceMessage(&(Error::COMPILER_NOTIFICATION.clone()), list![{ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Expression.traverseExpBidirSubExps")); __mm_s.push_str(&*literal!(" not yet implemented for match expressions. Called using: ")); __mm_s.push_str(&*(System::dladdr(&inEnterFunc.clone())).0); __mm_s.push_str(&*literal!(" ")); __mm_s.push_str(&*(System::dladdr(&inExitFunc.clone())).0); ArcStr::from(__mm_s) }], &(metamodelica::sourceInfo!("FrontEnd/Expression.mo")))?;
            (if (metamodelica::ReferenceEq::reference_eq(&(expl.clone()), &(expl_1))) {inExp} else {metamodelica::Ref::new(DAE::Exp::MATCHEXPRESSION { matchType: match_ty.clone(), inputs: expl_1, aliases: aliases.clone(), localDecls: match_decls.clone(), cases: match_cases.clone(), et: ty.clone() })}, arg)
        },
        Deref @ DAE::Exp::BOX { exp: e1 } => {
            let mut e1_1: metamodelica::Ref<DAE::Exp>;
            let mut arg: ArgT;
            (e1_1, arg) = traverseExpBidir(e1.clone(), inEnterFunc.clone(), inExitFunc.clone(), inArg)?;
            (if (referenceEq(&*(e1.clone()),&*(&*e1_1))) {inExp} else {metamodelica::Ref::new(DAE::Exp::BOX { exp: e1_1 })}, arg)
        },
        Deref @ DAE::Exp::UNBOX { exp: e1, ty } => {
            let mut e1_1: metamodelica::Ref<DAE::Exp>;
            let mut arg: ArgT;
            (e1_1, arg) = traverseExpBidir(e1.clone(), inEnterFunc.clone(), inExitFunc.clone(), inArg)?;
            (if (referenceEq(&*(e1.clone()),&*(&*e1_1))) {inExp} else {metamodelica::Ref::new(DAE::Exp::UNBOX { exp: e1_1, ty: ty.clone() })}, arg)
        },
        Deref @ DAE::Exp::SHARED_LITERAL { .. } => {
            (inExp, inArg)
        },
        Deref @ DAE::Exp::PATTERN { .. } => {
            (inExp, inArg)
        },
        _ => {
            Error::addInternalError({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Expression.traverseExpBidirSubExps")); __mm_s.push_str(&*literal!(" - Unknown expression ")); __mm_s.push_str(&*printExpStr(inExp)?); __mm_s.push_str(&*literal!(". Called using: ")); __mm_s.push_str(&*(System::dladdr(&inEnterFunc.clone())).0); __mm_s.push_str(&*literal!(" ")); __mm_s.push_str(&*(System::dladdr(&inExitFunc.clone())).0); ArcStr::from(__mm_s) }, metamodelica::sourceInfo!("FrontEnd/Expression.mo"))?;
            return Err("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outExp, outArg))
}

pub(crate) fn traverseExpBidirCref<ArgT: Clone + 'static + metamodelica::gc::MMTrace>(
    mut inCref: &metamodelica::Ref<DAE::ComponentRef>,
    mut inEnterFunc: Arc<
        dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>, ArgT) -> Result<(metamodelica::Ref<DAE::Exp>, ArgT)> + 'static,
    >,
    mut inExitFunc: Arc<
        dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>, ArgT) -> Result<(metamodelica::Ref<DAE::Exp>, ArgT)> + 'static,
    >,
    mut inArg: ArgT,
) -> Result<(metamodelica::Ref<DAE::ComponentRef>, ArgT)> {
    pub type FuncType<ArgT: Clone + 'static> = std::sync::Arc<
        dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>, ArgT) -> Result<(metamodelica::Ref<DAE::Exp>, ArgT)> + 'static,
    >;

    let mut outCref: metamodelica::Ref<DAE::ComponentRef>;
    let mut outArg: ArgT;
    (outCref, outArg) = (match &**inCref {
        DAE::ComponentRef::CREF_QUAL {
            ident: name,
            identType: ty,
            subscriptLst: subs,
            componentRef: cr,
        } => {
            let mut arg: ArgT;
            let mut subs = (*subs).clone();
            let mut cr = (*cr).clone();
            (subs, arg) = List::map2Fold(
                metamodelica::AsArg::as_arg(&subs),
                &traverseExpBidirSubs,
                inEnterFunc.clone(),
                inExitFunc.clone(),
                inArg,
                metamodelica::nil(),
            )?;
            (cr, arg) = traverseExpBidirCref(
                metamodelica::AsArg::as_arg(&cr),
                inEnterFunc.clone(),
                inExitFunc.clone(),
                arg,
            )?;
            (
                metamodelica::Ref::new(DAE::ComponentRef::CREF_QUAL {
                    ident: name.clone(),
                    identType: ty.clone(),
                    subscriptLst: subs.clone(),
                    componentRef: cr.clone(),
                }),
                arg,
            )
        }
        DAE::ComponentRef::CREF_IDENT {
            ident: name,
            identType: ty,
            subscriptLst: subs,
        } => {
            let mut arg: ArgT;
            let mut subs = (*subs).clone();
            (subs, arg) = List::map2Fold(
                metamodelica::AsArg::as_arg(&subs),
                &traverseExpBidirSubs,
                inEnterFunc.clone(),
                inExitFunc.clone(),
                inArg,
                metamodelica::nil(),
            )?;
            (
                metamodelica::Ref::new(DAE::ComponentRef::CREF_IDENT {
                    ident: name.clone(),
                    identType: ty.clone(),
                    subscriptLst: subs.clone(),
                }),
                arg,
            )
        }
        _ => (inCref.clone(), inArg),
    });
    Ok((outCref, outArg))
}

pub(crate) fn traverseExpCref<Type_a: Clone + 'static + metamodelica::gc::MMTrace>(
    mut inCref: &metamodelica::Ref<DAE::ComponentRef>,
    mut rel: &dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>, Type_a) -> Result<(metamodelica::Ref<DAE::Exp>, Type_a)>,
    mut iarg: Type_a,
) -> Result<(metamodelica::Ref<DAE::ComponentRef>, Type_a)> {
    pub type FuncType<Type_a: Clone + 'static> = std::sync::Arc<
        dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>, Type_a) -> Result<(metamodelica::Ref<DAE::Exp>, Type_a)>
            + 'static,
    >;

    let mut outCref: metamodelica::Ref<DAE::ComponentRef>;
    let mut outArg: Type_a;
    (outCref, outArg) = (match &**inCref {
        DAE::ComponentRef::CREF_QUAL {
            ident: name,
            identType: ty,
            subscriptLst: subs,
            componentRef: cr,
        } => {
            let mut arg = iarg;
            let mut cr_1: ComponentRef;
            let mut subs_1: metamodelica::List<metamodelica::Ref<DAE::Subscript>>;
            let mut cr = (*cr).clone();
            (subs_1, arg) = traverseExpSubs(subs, rel, arg)?;
            (cr_1, arg) = traverseExpCref(metamodelica::AsArg::as_arg(&cr), rel, arg)?;
            cr = if (referenceEq(&*(cr.clone()), &*(&*cr_1))
                && metamodelica::ReferenceEq::reference_eq(&(subs.clone()), &(subs_1)))
            {
                inCref.clone()
            } else {
                metamodelica::Ref::new(DAE::ComponentRef::CREF_QUAL {
                    ident: name.clone(),
                    identType: ty.clone(),
                    subscriptLst: subs_1,
                    componentRef: cr_1,
                })
            };
            (cr.clone(), arg)
        }
        DAE::ComponentRef::CREF_IDENT {
            ident: name,
            identType: ty,
            subscriptLst: subs,
        } => {
            let mut arg = iarg;
            let mut cr: ComponentRef;
            let mut subs_1: metamodelica::List<metamodelica::Ref<DAE::Subscript>>;
            (subs_1, arg) = traverseExpSubs(subs, rel, arg)?;
            cr = if (metamodelica::ReferenceEq::reference_eq(&(subs.clone()), &(subs_1))) {
                inCref.clone()
            } else {
                metamodelica::Ref::new(DAE::ComponentRef::CREF_IDENT {
                    ident: name.clone(),
                    identType: ty.clone(),
                    subscriptLst: subs_1,
                })
            };
            (cr, arg)
        }
        DAE::ComponentRef::OPTIMICA_ATTR_INST_CREF {
            componentRef: cr,
            instant,
        } => {
            let mut arg = iarg;
            let mut cr_1: ComponentRef;
            let mut cr = (*cr).clone();
            (cr_1, arg) = traverseExpCref(metamodelica::AsArg::as_arg(&cr), rel, arg)?;
            cr = if (referenceEq(&*(cr.clone()), &*(&*cr_1))) {
                inCref.clone()
            } else {
                metamodelica::Ref::new(DAE::ComponentRef::OPTIMICA_ATTR_INST_CREF {
                    componentRef: cr_1,
                    instant: instant.clone(),
                })
            };
            (cr.clone(), arg)
        }
        DAE::ComponentRef::WILD { .. } => {
            let mut arg = iarg;
            (inCref.clone(), arg)
        }
        _ => {
            Error::addMessage(
                Error::INTERNAL_ERROR.clone(),
                list![literal!("Expression.traverseExpCref: Unknown cref")],
            )?;
            return Err("fail");
        }
    });
    Ok((outCref, outArg))
}

fn traverseExpSubs<Type_a: Clone + 'static + metamodelica::gc::MMTrace>(
    mut inSubscript: &metamodelica::List<metamodelica::Ref<DAE::Subscript>>,
    mut rel: &dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>, Type_a) -> Result<(metamodelica::Ref<DAE::Exp>, Type_a)>,
    mut iarg: Type_a,
) -> Result<(metamodelica::List<metamodelica::Ref<DAE::Subscript>>, Type_a)> {
    pub type FuncType<Type_a: Clone + 'static> = std::sync::Arc<
        dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>, Type_a) -> Result<(metamodelica::Ref<DAE::Exp>, Type_a)>
            + 'static,
    >;

    let mut outSubscript: metamodelica::List<metamodelica::Ref<DAE::Subscript>>;
    let mut outArg: Type_a;
    (outSubscript, outArg) = (::match_deref::match_deref! { match inSubscript {
        Deref @ metamodelica::ListNode::Nil => {
            let mut arg = iarg;
            (inSubscript.clone(), arg)
        },
        Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Subscript::WHOLEDIM { .. }, tail: rest } => {
            let mut arg = iarg;
            let mut res: metamodelica::List<metamodelica::Ref<DAE::Subscript>>;
            (res, arg) = traverseExpSubs(rest, rel, arg)?;
            res = if (metamodelica::ReferenceEq::reference_eq(&(rest.clone()), &(res))) {inSubscript.clone()} else {metamodelica::cons(openmodelica_frontend_types::DAE::Subscript::interned_WHOLEDIM(), res)};
            (res, arg)
        },
        Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Subscript::SLICE { exp: sub_exp }, tail: rest } => {
            let mut arg = iarg;
            let mut sub_exp_1: metamodelica::Ref<DAE::Exp>;
            let mut res: metamodelica::List<metamodelica::Ref<DAE::Subscript>>;
            (sub_exp_1, arg) = traverseExpBottomUp(sub_exp.clone(), rel, arg)?;
            (res, arg) = traverseExpSubs(rest, rel, arg)?;
            res = if (referenceEq(&*(sub_exp.clone()),&*(&*sub_exp_1)) && metamodelica::ReferenceEq::reference_eq(&(rest.clone()), &(res))) {inSubscript.clone()} else {metamodelica::cons(metamodelica::Ref::new(DAE::Subscript::SLICE { exp: sub_exp_1 }), res)};
            (res, arg)
        },
        Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Subscript::INDEX { exp: sub_exp }, tail: rest } => {
            let mut arg = iarg;
            let mut sub_exp_1: metamodelica::Ref<DAE::Exp>;
            let mut res: metamodelica::List<metamodelica::Ref<DAE::Subscript>>;
            (sub_exp_1, arg) = traverseExpBottomUp(sub_exp.clone(), rel, arg)?;
            (res, arg) = traverseExpSubs(rest, rel, arg)?;
            res = if (referenceEq(&*(sub_exp.clone()),&*(&*sub_exp_1)) && metamodelica::ReferenceEq::reference_eq(&(rest.clone()), &(res))) {inSubscript.clone()} else {metamodelica::cons(metamodelica::Ref::new(DAE::Subscript::INDEX { exp: sub_exp_1 }), res)};
            (res, arg)
        },
        Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Subscript::WHOLE_NONEXP { exp: sub_exp }, tail: rest } => {
            let mut arg = iarg;
            let mut sub_exp_1: metamodelica::Ref<DAE::Exp>;
            let mut res: metamodelica::List<metamodelica::Ref<DAE::Subscript>>;
            (sub_exp_1, arg) = traverseExpBottomUp(sub_exp.clone(), rel, arg)?;
            (res, arg) = traverseExpSubs(rest, rel, arg)?;
            res = if (referenceEq(&*(sub_exp.clone()),&*(&*sub_exp_1)) && metamodelica::ReferenceEq::reference_eq(&(rest.clone()), &(res))) {inSubscript.clone()} else {metamodelica::cons(metamodelica::Ref::new(DAE::Subscript::WHOLE_NONEXP { exp: sub_exp_1 }), res)};
            (res, arg)
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok((outSubscript, outArg))
}

pub fn traverseExpTopDownCrefHelper<Argument: Clone + 'static + metamodelica::gc::MMTrace>(
    mut inCref: &metamodelica::Ref<DAE::ComponentRef>,
    mut rel: &dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>, Argument) -> Result<(metamodelica::Ref<DAE::Exp>, bool, Argument)>,
    mut iarg: Argument,
) -> Result<(metamodelica::Ref<DAE::ComponentRef>, Argument)> {
    pub type FuncType<Argument: Clone + 'static> = std::sync::Arc<
        dyn ::std::ops::Fn(
                metamodelica::Ref<DAE::Exp>,
                Argument,
            ) -> Result<(metamodelica::Ref<DAE::Exp>, bool, Argument)>
            + 'static,
    >;

    let mut outCref: metamodelica::Ref<DAE::ComponentRef>;
    let mut outArg: Argument;
    (outCref, outArg) = (match &**inCref {
        DAE::ComponentRef::CREF_QUAL {
            ident: name,
            identType: ty,
            subscriptLst: subs,
            componentRef: cr,
        } => {
            let mut arg = iarg;
            let mut cr_1: ComponentRef;
            let mut subs_1: metamodelica::List<metamodelica::Ref<DAE::Subscript>>;
            (subs_1, arg) = traverseExpTopDownSubs(subs.clone(), rel, arg)?;
            (cr_1, arg) = traverseExpTopDownCrefHelper(cr, rel, arg)?;
            (
                if (metamodelica::ReferenceEq::reference_eq(&(subs.clone()), &(subs_1))
                    && referenceEq(&*(cr.clone()), &*(&*cr_1)))
                {
                    inCref.clone()
                } else {
                    metamodelica::Ref::new(DAE::ComponentRef::CREF_QUAL {
                        ident: name.clone(),
                        identType: ty.clone(),
                        subscriptLst: subs_1,
                        componentRef: cr_1,
                    })
                },
                arg,
            )
        }
        DAE::ComponentRef::CREF_IDENT {
            ident: name,
            identType: ty,
            subscriptLst: subs,
        } => {
            let mut arg = iarg;
            let mut subs_1: metamodelica::List<metamodelica::Ref<DAE::Subscript>>;
            (subs_1, arg) = traverseExpTopDownSubs(subs.clone(), rel, arg)?;
            (
                if (metamodelica::ReferenceEq::reference_eq(&(subs.clone()), &(subs_1))) {
                    inCref.clone()
                } else {
                    metamodelica::Ref::new(DAE::ComponentRef::CREF_IDENT {
                        ident: name.clone(),
                        identType: ty.clone(),
                        subscriptLst: subs_1,
                    })
                },
                arg,
            )
        }
        DAE::ComponentRef::WILD { .. } => {
            let mut arg = iarg;
            (inCref.clone(), arg)
        }
        _ => return Err("match: no arm matched"),
    });
    Ok((outCref, outArg))
}

fn traverseExpBidirSubs<ArgT: Clone + 'static + metamodelica::gc::MMTrace>(
    mut inSubscript: metamodelica::Ref<DAE::Subscript>,
    mut inEnterFunc: Arc<
        dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>, ArgT) -> Result<(metamodelica::Ref<DAE::Exp>, ArgT)> + 'static,
    >,
    mut inExitFunc: Arc<
        dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>, ArgT) -> Result<(metamodelica::Ref<DAE::Exp>, ArgT)> + 'static,
    >,
    mut inArg: ArgT,
) -> Result<(metamodelica::Ref<DAE::Subscript>, ArgT)> {
    pub type FuncType<ArgT: Clone + 'static> = std::sync::Arc<
        dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>, ArgT) -> Result<(metamodelica::Ref<DAE::Exp>, ArgT)> + 'static,
    >;

    let mut outSubscript: metamodelica::Ref<DAE::Subscript>;
    let mut outArg: ArgT;
    (outSubscript, outArg) = (match &*inSubscript {
        DAE::Subscript::WHOLEDIM { .. } => (inSubscript, inArg),
        DAE::Subscript::SLICE { exp: sub_exp } => {
            let mut arg: ArgT;
            let mut sub_exp = (*sub_exp).clone();
            (sub_exp, arg) = traverseExpBidir(sub_exp.clone(), inEnterFunc.clone(), inExitFunc.clone(), inArg)?;
            (
                metamodelica::Ref::new(DAE::Subscript::SLICE { exp: sub_exp.clone() }),
                arg,
            )
        }
        DAE::Subscript::INDEX { exp: sub_exp } => {
            let mut arg: ArgT;
            let mut sub_exp = (*sub_exp).clone();
            (sub_exp, arg) = traverseExpBidir(sub_exp.clone(), inEnterFunc.clone(), inExitFunc.clone(), inArg)?;
            (
                metamodelica::Ref::new(DAE::Subscript::INDEX { exp: sub_exp.clone() }),
                arg,
            )
        }
        DAE::Subscript::WHOLE_NONEXP { exp: sub_exp } => {
            let mut arg: ArgT;
            let mut sub_exp = (*sub_exp).clone();
            (sub_exp, arg) = traverseExpBidir(sub_exp.clone(), inEnterFunc.clone(), inExitFunc.clone(), inArg)?;
            (
                metamodelica::Ref::new(DAE::Subscript::WHOLE_NONEXP { exp: sub_exp.clone() }),
                arg,
            )
        }
    });
    Ok((outSubscript, outArg))
}

pub fn traverseExpTopDownSubs<Argument: Clone + 'static + metamodelica::gc::MMTrace>(
    mut inSubscript: metamodelica::List<metamodelica::Ref<DAE::Subscript>>,
    mut rel: &dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>, Argument) -> Result<(metamodelica::Ref<DAE::Exp>, bool, Argument)>,
    mut iarg: Argument,
) -> Result<(metamodelica::List<metamodelica::Ref<DAE::Subscript>>, Argument)> {
    pub type FuncType<Argument: Clone + 'static> = std::sync::Arc<
        dyn ::std::ops::Fn(
                metamodelica::Ref<DAE::Exp>,
                Argument,
            ) -> Result<(metamodelica::Ref<DAE::Exp>, bool, Argument)>
            + 'static,
    >;

    let mut outSubscript: metamodelica::List<metamodelica::Ref<DAE::Subscript>>;
    let mut arg: Argument = iarg;
    let mut exp: metamodelica::Ref<DAE::Exp>;
    let mut sub: metamodelica::Ref<DAE::Subscript>;
    let mut nsub: metamodelica::Ref<DAE::Subscript>;
    let mut delst: DoubleEnded::MutableList<metamodelica::Ref<DAE::Subscript>>;
    let mut rest: metamodelica::List<metamodelica::Ref<DAE::Subscript>> = inSubscript.clone();
    let mut nEq: i32 = 0;
    outSubscript = inSubscript.clone();
    while !((rest).is_empty()) {
        sub = (rest).head().cloned()?;
        nsub = (match &*sub {
            DAE::Subscript::WHOLEDIM { .. } => sub.clone(),
            DAE::Subscript::SLICE { exp: __sub_exp } => {
                (exp, arg) = traverseExpTopDown(__sub_exp.clone(), rel, arg)?;
                if (referenceEq(&*(__sub_exp.clone()), &*(&*exp))) {
                    sub.clone()
                } else {
                    metamodelica::Ref::new(DAE::Subscript::SLICE { exp: exp })
                }
            }
            DAE::Subscript::INDEX { exp: __sub_exp } => {
                (exp, arg) = traverseExpTopDown(__sub_exp.clone(), rel, arg)?;
                if (referenceEq(&*(__sub_exp.clone()), &*(&*exp))) {
                    sub.clone()
                } else {
                    metamodelica::Ref::new(DAE::Subscript::INDEX { exp: exp })
                }
            }
            DAE::Subscript::WHOLE_NONEXP { exp: __sub_exp } => {
                (exp, arg) = traverseExpTopDown(__sub_exp.clone(), rel, arg)?;
                if (referenceEq(&*(__sub_exp.clone()), &*(&*exp))) {
                    sub.clone()
                } else {
                    metamodelica::Ref::new(DAE::Subscript::WHOLE_NONEXP { exp: exp })
                }
            }
        });
        if !(referenceEq(&*(&*nsub), &*(sub))) {
            delst = DoubleEnded::empty(nsub.clone());
            for mut elt in &*inSubscript {
                if nEq < 1 {
                    break;
                }
                DoubleEnded::push_back(delst.clone(), elt.clone())?;
                nEq = nEq - 1;
            }
            DoubleEnded::push_back(delst.clone(), nsub)?;
            for mut sub2 in &*(rest).rest()? {
                sub = sub2.clone();
                nsub = (match &*sub {
                    DAE::Subscript::WHOLEDIM { .. } => sub,
                    DAE::Subscript::SLICE { exp: __sub_exp } => {
                        (exp, arg) = traverseExpTopDown(__sub_exp.clone(), rel, arg)?;
                        if (referenceEq(&*(__sub_exp.clone()), &*(&*exp))) {
                            sub
                        } else {
                            metamodelica::Ref::new(DAE::Subscript::SLICE { exp: exp })
                        }
                    }
                    DAE::Subscript::INDEX { exp: __sub_exp } => {
                        (exp, arg) = traverseExpTopDown(__sub_exp.clone(), rel, arg)?;
                        if (referenceEq(&*(__sub_exp.clone()), &*(&*exp))) {
                            sub
                        } else {
                            metamodelica::Ref::new(DAE::Subscript::INDEX { exp: exp })
                        }
                    }
                    DAE::Subscript::WHOLE_NONEXP { exp: __sub_exp } => {
                        (exp, arg) = traverseExpTopDown(__sub_exp.clone(), rel, arg)?;
                        if (referenceEq(&*(__sub_exp.clone()), &*(&*exp))) {
                            sub
                        } else {
                            metamodelica::Ref::new(DAE::Subscript::WHOLE_NONEXP { exp: exp })
                        }
                    }
                });
                DoubleEnded::push_back(delst.clone(), nsub)?;
            }
            outSubscript = DoubleEnded::toListAndClear(delst, metamodelica::nil())?;
            return Ok((outSubscript, arg));
        }
        nEq = nEq + 1;
        rest = (rest).rest()?;
    }
    Ok((outSubscript, arg))
}

/* **************************************************/
/* Compare and Check DAE.Exp */
/* **************************************************/
pub fn operatorDivOrMul(mut op: &DAE::Operator) -> bool {
    let mut res: bool;
    res = (match op.clone() {
        DAE::Operator::MUL { ty: _ } => true,
        DAE::Operator::DIV { ty: _ } => true,
        _ => false,
    });
    res
}

pub fn isRange(mut inExp: &metamodelica::Ref<DAE::Exp>) -> bool {
    let mut outBoolean: bool;
    outBoolean = (match &**inExp {
        DAE::Exp::RANGE { .. } => true,
        _ => false,
    });
    outBoolean
}

pub fn isReduction(mut inExp: &metamodelica::Ref<DAE::Exp>) -> bool {
    let mut outBoolean: bool;
    outBoolean = (match &**inExp {
        DAE::Exp::REDUCTION { .. } => true,
        _ => false,
    });
    outBoolean
}

pub fn isOne<'__b>(mut inExp: &'__b metamodelica::Ref<DAE::Exp>) -> bool {
    '__tco: loop {
        match &**inExp {
            DAE::Exp::ICONST { integer: ival } => return intEq(ival.clone(), 1),
            DAE::Exp::RCONST { real: rval } => return realEq(rval.clone(), metamodelica::OrderedFloat(1.0_f64)),
            DAE::Exp::CAST { exp: e, .. } => {
                let mut res: bool;
                {
                    inExp = e;
                    continue '__tco;
                }
            }
            _ => return false,
        }
    }
}

pub fn isZero<'__b>(mut inExp: &'__b metamodelica::Ref<DAE::Exp>) -> Result<bool> {
    '__tco: loop {
        match &**inExp {
            DAE::Exp::ICONST { integer: ival } => return Ok(intEq(ival.clone(), 0)),
            DAE::Exp::RCONST { real: rval } => return Ok(realEq(rval.clone(), metamodelica::OrderedFloat(0.0_f64))),
            DAE::Exp::CAST { exp: e, .. } => {
                inExp = e;
                continue '__tco;
            }
            DAE::Exp::UNARY {
                operator: DAE::Operator::UMINUS { ty: _ },
                exp: e,
            } => {
                inExp = e;
                continue '__tco;
            }
            DAE::Exp::ARRAY { array: ae, .. } => {
                return Ok(List::all(ae, &move |__a0: metamodelica::Ref<DAE::Exp>| isZero(&__a0))?);
            }
            DAE::Exp::MATRIX { matrix, .. } => {
                return Ok(List::all(
                    matrix,
                    &({
                        let __pe_b1: Arc<dyn ::std::ops::Fn(_) -> Result<bool> + 'static> =
                            (std::sync::Arc::new(move |__a0: metamodelica::Ref<DAE::Exp>| isZero(&__a0))
                                as std::sync::Arc<
                                    dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<bool> + 'static,
                                >);
                        move |__pe_a0| List::all(&__pe_a0, &*__pe_b1)
                    }),
                )?);
            }
            DAE::Exp::UNARY {
                operator: DAE::Operator::UMINUS_ARR { ty: _ },
                exp: e,
            } => {
                inExp = e;
                continue '__tco;
            }
            _ => return Ok(false),
        }
    }
}

pub fn isZeroOrAlmostZero<'__b>(
    mut inExp: &'__b metamodelica::Ref<DAE::Exp>,
    mut nominal: &'__b metamodelica::Ref<DAE::Exp>,
) -> Result<bool> {
    '__tco: loop {
        ::match_deref::match_deref! { match (inExp, nominal) {
            (Deref @ DAE::Exp::ICONST { integer: ival }, _) => {
                return Ok(intEq(ival.clone(), 0))
            },
            (Deref @ DAE::Exp::RCONST { real: rval }, Deref @ DAE::Exp::RCONST { real: rNom }) => {
                return Ok(realLt((rval.clone()).abs(), metamodelica::OrderedFloat(1e-6_f64) * (rNom.clone()).abs()))
            },
            (Deref @ DAE::Exp::RCONST { real: rval }, _) => {
                return Ok(realLt((rval.clone()).abs(), metamodelica::OrderedFloat(1e-6_f64)))
            },
            (Deref @ DAE::Exp::CAST { exp: e, .. }, _) => {
                { (inExp, nominal) = (e, nominal); continue '__tco; }
            },
            (Deref @ DAE::Exp::UNARY { operator: DAE::Operator::UMINUS { ty: _ }, exp: e }, _) => {
                { (inExp, nominal) = (e, nominal); continue '__tco; }
            },
            (Deref @ DAE::Exp::ARRAY { array: ae, .. }, _) => {
                return Ok(List::all(ae, &({ let __pe_b1 = nominal.clone(); move |__pe_a0| isZeroOrAlmostZero(&__pe_a0, &__pe_b1) }))?)
            },
            (Deref @ DAE::Exp::MATRIX { matrix, .. }, _) => {
                return Ok(List::all(matrix, &({ let __pe_b1: Arc<dyn ::std::ops::Fn(_) -> Result<bool> + 'static> = (std::sync::Arc::new({ let __pe_b1 = nominal.clone(); move |__pe_a0| isZeroOrAlmostZero(&__pe_a0, &__pe_b1) }) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<bool> + 'static>); move |__pe_a0| List::all(&__pe_a0, &*__pe_b1) }))?)
            },
            (Deref @ DAE::Exp::UNARY { operator: DAE::Operator::UMINUS_ARR { ty: _ }, exp: e }, _) => {
                { (inExp, nominal) = (e, nominal); continue '__tco; }
            },
            (Deref @ DAE::Exp::IFEXP { expCond: _, expThen: e, expElse: e1 }, _) => {
                return Ok(isZeroOrAlmostZero(e, nominal)? || isZeroOrAlmostZero(e1, nominal)?)
            },
            _ => {
                return Ok(false)
            },
            _ => return Err("match: no arm matched"),
        } }
    }
}

pub fn isPositiveOrZero(mut inExp: metamodelica::Ref<DAE::Exp>) -> Result<bool> {
    '__tco: loop {
        ::match_deref::match_deref! { match &(inExp.clone()) {
            Deref @ DAE::Exp::ICONST { integer: i } => {
                return Ok(i.clone() >= 0)
            },
            Deref @ DAE::Exp::RCONST { real: r } => {
                return Ok(r.clone() >= metamodelica::OrderedFloat(0.0_f64))
            },
            Deref @ DAE::Exp::BINARY { exp1: e1, operator: DAE::Operator::ADD { .. }, exp2: e2 } => {
                return Ok(isPositiveOrZero(e1.clone())? && isPositiveOrZero(e2.clone())?)
            },
            Deref @ DAE::Exp::BINARY { exp1: e1, operator: DAE::Operator::SUB { .. }, exp2: e2 } => {
                return Ok(isPositiveOrZero(e1.clone())? && isNegativeOrZero(e2.clone())?)
            },
            Deref @ DAE::Exp::BINARY { exp1: e1, operator: DAE::Operator::MUL { .. }, exp2: e2 } => {
                return Ok(isPositiveOrZero(e1.clone())? && isPositiveOrZero(e2.clone())? || isNegativeOrZero(e1.clone())? && isNegativeOrZero(e2.clone())? || ExpressionBasics::expEqual(metamodelica::AsArg::as_arg(&e1), e2.clone())?)
            },
            Deref @ DAE::Exp::BINARY { exp1: e1, operator: DAE::Operator::DIV { .. }, exp2: e2 } => {
                return Ok(isPositiveOrZero(e1.clone())? && isPositiveOrZero(e2.clone())? || isNegativeOrZero(e1.clone())? && isNegativeOrZero(e2.clone())?)
            },
            Deref @ DAE::Exp::BINARY { exp1: e1, operator: DAE::Operator::POW { .. }, exp2: _ } => {
                { inExp = e1.clone(); continue '__tco; }
            },
            Deref @ DAE::Exp::BINARY { exp1: _, operator: DAE::Operator::POW { .. }, exp2: e2 } => {
                return Ok(isEven(metamodelica::AsArg::as_arg(&e2)))
            },
            Deref @ DAE::Exp::UNARY { operator: DAE::Operator::UMINUS { .. }, exp: e1 } => {
                return Ok(isNegativeOrZero(e1.clone())?)
            },
            Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "abs" }, .. } => {
                return Ok(true)
            },
            Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "cosh" }, .. } => {
                return Ok(true)
            },
            Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "exp" }, .. } => {
                return Ok(true)
            },
            Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "sign" }, expLst: Deref @ metamodelica::ListNode::Cons { head: e1, tail: Deref @ metamodelica::ListNode::Nil }, .. } => {
                { inExp = e1.clone(); continue '__tco; }
            },
            Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "sinh" }, expLst: Deref @ metamodelica::ListNode::Cons { head: e1, tail: Deref @ metamodelica::ListNode::Nil }, .. } => {
                { inExp = e1.clone(); continue '__tco; }
            },
            Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "tanh" }, expLst: Deref @ metamodelica::ListNode::Cons { head: e1, tail: Deref @ metamodelica::ListNode::Nil }, .. } => {
                { inExp = e1.clone(); continue '__tco; }
            },
            Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "ceil" }, expLst: Deref @ metamodelica::ListNode::Cons { head: e1, tail: Deref @ metamodelica::ListNode::Nil }, .. } => {
                { inExp = e1.clone(); continue '__tco; }
            },
            Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "floor" }, expLst: Deref @ metamodelica::ListNode::Cons { head: e1, tail: Deref @ metamodelica::ListNode::Nil }, .. } => {
                { inExp = e1.clone(); continue '__tco; }
            },
            Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "integer" }, expLst: Deref @ metamodelica::ListNode::Cons { head: e1, tail: Deref @ metamodelica::ListNode::Nil }, .. } => {
                { inExp = e1.clone(); continue '__tco; }
            },
            _ => {
                return Ok(isZero(&inExp)?)
            },
            _ => return Err("match: no arm matched"),
        } }
    }
}

pub fn isNegativeOrZero(mut inExp: metamodelica::Ref<DAE::Exp>) -> Result<bool> {
    '__tco: loop {
        ::match_deref::match_deref! { match &(inExp.clone()) {
            Deref @ DAE::Exp::ICONST { integer: i } => {
                return Ok(i.clone() <= 0)
            },
            Deref @ DAE::Exp::RCONST { real: r } => {
                return Ok(r.clone() <= metamodelica::OrderedFloat(0.0_f64))
            },
            Deref @ DAE::Exp::BINARY { exp1: e1, operator: DAE::Operator::ADD { .. }, exp2: e2 } => {
                return Ok(isNegativeOrZero(e1.clone())? && isNegativeOrZero(e2.clone())?)
            },
            Deref @ DAE::Exp::BINARY { exp1: e1, operator: DAE::Operator::SUB { .. }, exp2: e2 } => {
                return Ok(isNegativeOrZero(e1.clone())? && isPositiveOrZero(e2.clone())?)
            },
            Deref @ DAE::Exp::BINARY { exp1: e1, operator: DAE::Operator::MUL { .. }, exp2: e2 } => {
                return Ok(isPositiveOrZero(e1.clone())? && isNegativeOrZero(e2.clone())? || isNegativeOrZero(e1.clone())? && isPositiveOrZero(e2.clone())?)
            },
            Deref @ DAE::Exp::BINARY { exp1: e1, operator: DAE::Operator::DIV { .. }, exp2: e2 } => {
                return Ok(isPositiveOrZero(e1.clone())? && isNegativeOrZero(e2.clone())? || isNegativeOrZero(e1.clone())? && isPositiveOrZero(e2.clone())?)
            },
            Deref @ DAE::Exp::BINARY { exp1: e1, operator: DAE::Operator::POW { .. }, exp2: e2 } => {
                return Ok(isNegativeOrZero(e1.clone())? && isOdd(metamodelica::AsArg::as_arg(&e2)))
            },
            Deref @ DAE::Exp::UNARY { operator: DAE::Operator::UMINUS { .. }, exp: e1 } => {
                return Ok(isPositiveOrZero(e1.clone())?)
            },
            Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "abs" }, expLst: Deref @ metamodelica::ListNode::Cons { head: e1, tail: Deref @ metamodelica::ListNode::Nil }, .. } => {
                return Ok(isZero(metamodelica::AsArg::as_arg(&e1))?)
            },
            Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "cosh" }, .. } => {
                return Ok(false)
            },
            Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "exp" }, .. } => {
                return Ok(false)
            },
            Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "sign" }, expLst: Deref @ metamodelica::ListNode::Cons { head: e1, tail: Deref @ metamodelica::ListNode::Nil }, .. } => {
                { inExp = e1.clone(); continue '__tco; }
            },
            Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "sinh" }, expLst: Deref @ metamodelica::ListNode::Cons { head: e1, tail: Deref @ metamodelica::ListNode::Nil }, .. } => {
                { inExp = e1.clone(); continue '__tco; }
            },
            Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "tanh" }, expLst: Deref @ metamodelica::ListNode::Cons { head: e1, tail: Deref @ metamodelica::ListNode::Nil }, .. } => {
                { inExp = e1.clone(); continue '__tco; }
            },
            Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "ceil" }, expLst: Deref @ metamodelica::ListNode::Cons { head: e1, tail: Deref @ metamodelica::ListNode::Nil }, .. } => {
                { inExp = e1.clone(); continue '__tco; }
            },
            Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "floor" }, expLst: Deref @ metamodelica::ListNode::Cons { head: e1, tail: Deref @ metamodelica::ListNode::Nil }, .. } => {
                { inExp = e1.clone(); continue '__tco; }
            },
            Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "integer" }, expLst: Deref @ metamodelica::ListNode::Cons { head: e1, tail: Deref @ metamodelica::ListNode::Nil }, .. } => {
                { inExp = e1.clone(); continue '__tco; }
            },
            _ => {
                return Ok(isZero(&inExp)?)
            },
            _ => return Err("match: no arm matched"),
        } }
    }
}

pub(crate) fn isPositive(mut inExp: metamodelica::Ref<DAE::Exp>) -> Result<bool> {
    '__tco: loop {
        ::match_deref::match_deref! { match &(inExp) {
            Deref @ DAE::Exp::ICONST { integer: i } => {
                return Ok(i.clone() > 0)
            },
            Deref @ DAE::Exp::RCONST { real: r } => {
                return Ok(r.clone() > metamodelica::OrderedFloat(0.0_f64))
            },
            Deref @ DAE::Exp::BINARY { exp1: e1, operator: DAE::Operator::ADD { .. }, exp2: e2 } => {
                return Ok(isPositive(e1.clone())? && isPositiveOrZero(e2.clone())? || isZero(metamodelica::AsArg::as_arg(&e1))? && isPositive(e2.clone())?)
            },
            Deref @ DAE::Exp::BINARY { exp1: e1, operator: DAE::Operator::SUB { .. }, exp2: e2 } => {
                return Ok(isPositive(e1.clone())? && isNegativeOrZero(e2.clone())? || isZero(metamodelica::AsArg::as_arg(&e1))? && isNegative(e2.clone())?)
            },
            Deref @ DAE::Exp::BINARY { exp1: e1, operator: DAE::Operator::MUL { .. }, exp2: e2 } => {
                return Ok(isPositive(e1.clone())? && isPositive(e2.clone())? || isNegative(e1.clone())? && isNegative(e2.clone())?)
            },
            Deref @ DAE::Exp::BINARY { exp1: e1, operator: DAE::Operator::DIV { .. }, exp2: e2 } => {
                return Ok(isPositive(e1.clone())? && isPositive(e2.clone())? || isNegative(e1.clone())? && isNegative(e2.clone())?)
            },
            Deref @ DAE::Exp::BINARY { exp1: e1, operator: DAE::Operator::POW { .. }, exp2: _ } => {
                { inExp = e1.clone(); continue '__tco; }
            },
            Deref @ DAE::Exp::UNARY { operator: DAE::Operator::UMINUS { .. }, exp: e1 } => {
                return Ok(isNegative(e1.clone())?)
            },
            Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "abs" }, expLst: Deref @ metamodelica::ListNode::Cons { head: e1, tail: Deref @ metamodelica::ListNode::Nil }, .. } => {
                return Ok(isPositive(e1.clone())? || isNegative(e1.clone())?)
            },
            Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "cosh" }, .. } => {
                return Ok(true)
            },
            Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "exp" }, .. } => {
                return Ok(true)
            },
            Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "sign" }, expLst: Deref @ metamodelica::ListNode::Cons { head: e1, tail: Deref @ metamodelica::ListNode::Nil }, .. } => {
                { inExp = e1.clone(); continue '__tco; }
            },
            Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "sinh" }, expLst: Deref @ metamodelica::ListNode::Cons { head: e1, tail: Deref @ metamodelica::ListNode::Nil }, .. } => {
                { inExp = e1.clone(); continue '__tco; }
            },
            Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "tanh" }, expLst: Deref @ metamodelica::ListNode::Cons { head: e1, tail: Deref @ metamodelica::ListNode::Nil }, .. } => {
                { inExp = e1.clone(); continue '__tco; }
            },
            Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "ceil" }, expLst: Deref @ metamodelica::ListNode::Cons { head: e1, tail: Deref @ metamodelica::ListNode::Nil }, .. } => {
                { inExp = e1.clone(); continue '__tco; }
            },
            _ => {
                return Ok(false)
            },
            _ => return Err("match: no arm matched"),
        } }
    }
}

pub(crate) fn isNegative(mut inExp: metamodelica::Ref<DAE::Exp>) -> Result<bool> {
    '__tco: loop {
        ::match_deref::match_deref! { match &(inExp) {
            Deref @ DAE::Exp::ICONST { integer: i } => {
                return Ok(i.clone() < 0)
            },
            Deref @ DAE::Exp::RCONST { real: r } => {
                return Ok(r.clone() < metamodelica::OrderedFloat(0.0_f64))
            },
            Deref @ DAE::Exp::BINARY { exp1: e1, operator: DAE::Operator::ADD { .. }, exp2: e2 } => {
                return Ok(isNegative(e1.clone())? && isNegativeOrZero(e2.clone())? || isZero(metamodelica::AsArg::as_arg(&e1))? && isNegative(e2.clone())?)
            },
            Deref @ DAE::Exp::BINARY { exp1: e1, operator: DAE::Operator::SUB { .. }, exp2: e2 } => {
                return Ok(isNegative(e1.clone())? && isPositiveOrZero(e2.clone())? || isZero(metamodelica::AsArg::as_arg(&e1))? && isPositive(e2.clone())?)
            },
            Deref @ DAE::Exp::BINARY { exp1: e1, operator: DAE::Operator::MUL { .. }, exp2: e2 } => {
                return Ok(isPositive(e1.clone())? && isNegative(e2.clone())? || isNegative(e1.clone())? && isPositive(e2.clone())?)
            },
            Deref @ DAE::Exp::BINARY { exp1: e1, operator: DAE::Operator::DIV { .. }, exp2: e2 } => {
                return Ok(isPositive(e1.clone())? && isNegative(e2.clone())? || isNegative(e1.clone())? && isPositive(e2.clone())?)
            },
            Deref @ DAE::Exp::BINARY { exp1: e1, operator: DAE::Operator::POW { .. }, exp2: e2 } => {
                return Ok(isNegative(e1.clone())? && isOdd(metamodelica::AsArg::as_arg(&e2)))
            },
            Deref @ DAE::Exp::UNARY { operator: DAE::Operator::UMINUS { .. }, exp: e1 } => {
                { inExp = e1.clone(); continue '__tco; }
            },
            Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "abs" }, .. } => {
                return Ok(false)
            },
            Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "cosh" }, .. } => {
                return Ok(false)
            },
            Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "exp" }, .. } => {
                return Ok(false)
            },
            Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "sign" }, expLst: Deref @ metamodelica::ListNode::Cons { head: e1, tail: Deref @ metamodelica::ListNode::Nil }, .. } => {
                { inExp = e1.clone(); continue '__tco; }
            },
            Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "sinh" }, expLst: Deref @ metamodelica::ListNode::Cons { head: e1, tail: Deref @ metamodelica::ListNode::Nil }, .. } => {
                { inExp = e1.clone(); continue '__tco; }
            },
            Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "tanh" }, expLst: Deref @ metamodelica::ListNode::Cons { head: e1, tail: Deref @ metamodelica::ListNode::Nil }, .. } => {
                { inExp = e1.clone(); continue '__tco; }
            },
            Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "floor" }, expLst: Deref @ metamodelica::ListNode::Cons { head: e1, tail: Deref @ metamodelica::ListNode::Nil }, .. } => {
                { inExp = e1.clone(); continue '__tco; }
            },
            _ => {
                return Ok(false)
            },
            _ => return Err("match: no arm matched"),
        } }
    }
}

pub fn isGreaterOrEqual(mut exp1: metamodelica::Ref<DAE::Exp>, mut exp2: metamodelica::Ref<DAE::Exp>) -> Result<bool> {
    let mut isGreaterOrEqual: bool =
        isPositiveOrZero((ExpressionSimplify::simplify(expSub(exp1.clone(), exp2.clone())?)?).0)?;
    Ok(isGreaterOrEqual)
}

pub fn isHalf(mut inExp: &metamodelica::Ref<DAE::Exp>) -> bool {
    let mut outBoolean: bool;
    outBoolean = (match &**inExp {
        DAE::Exp::RCONST { real: rval } => realEq(rval.clone(), metamodelica::OrderedFloat(0.5_f64)),
        _ => false,
    });
    outBoolean
}

pub fn isAtomic(mut inExp: &metamodelica::Ref<DAE::Exp>) -> bool {
    let mut outBoolean: bool;
    outBoolean = (match &**inExp {
        DAE::Exp::CREF { .. } => true,
        DAE::Exp::CALL { .. } => true,
        DAE::Exp::ICONST {
            integer: __inExp_integer,
        } => __inExp_integer.clone() >= 0,
        DAE::Exp::RCONST { real: __inExp_real } => __inExp_real.clone() > metamodelica::OrderedFloat(0.0_f64),
        _ => false,
    });
    outBoolean
}

pub fn isDeeperThan<'__b>(mut inExp: &'__b metamodelica::Ref<DAE::Exp>, mut inDepth: i32) -> bool {
    '__tco: loop {
        match &**inExp {
            _ if (inDepth <= 0) => return true,
            DAE::Exp::BINARY { .. } => {
                if (isDeeperThan(var_field!((**inExp).exp1, DAE::Exp::BINARY), inDepth - 1)) {
                    return true;
                } else {
                    {
                        (inExp, inDepth) = (var_field!((**inExp).exp2, DAE::Exp::BINARY), inDepth - 1);
                        continue '__tco;
                    }
                }
            }
            DAE::Exp::LBINARY { .. } => {
                if (isDeeperThan(var_field!((**inExp).exp1, DAE::Exp::LBINARY), inDepth - 1)) {
                    return true;
                } else {
                    {
                        (inExp, inDepth) = (var_field!((**inExp).exp2, DAE::Exp::LBINARY), inDepth - 1);
                        continue '__tco;
                    }
                }
            }
            DAE::Exp::RELATION { .. } => {
                if (isDeeperThan(var_field!((**inExp).exp1, DAE::Exp::RELATION), inDepth - 1)) {
                    return true;
                } else {
                    {
                        (inExp, inDepth) = (var_field!((**inExp).exp2, DAE::Exp::RELATION), inDepth - 1);
                        continue '__tco;
                    }
                }
            }
            DAE::Exp::IFEXP { .. } => {
                if (isDeeperThan(var_field!((**inExp).expCond, DAE::Exp::IFEXP), inDepth - 1)) {
                    return true;
                } else if (isDeeperThan(var_field!((**inExp).expThen, DAE::Exp::IFEXP), inDepth - 1)) {
                    return true;
                } else {
                    {
                        (inExp, inDepth) = (var_field!((**inExp).expElse, DAE::Exp::IFEXP), inDepth - 1);
                        continue '__tco;
                    }
                }
            }
            DAE::Exp::UNARY { .. } => {
                (inExp, inDepth) = (var_field!((**inExp).exp, DAE::Exp::UNARY), inDepth - 1);
                continue '__tco;
            }
            DAE::Exp::LUNARY { .. } => {
                (inExp, inDepth) = (var_field!((**inExp).exp, DAE::Exp::LUNARY), inDepth - 1);
                continue '__tco;
            }
            DAE::Exp::CAST { .. } => {
                (inExp, inDepth) = (var_field!((**inExp).exp, DAE::Exp::CAST), inDepth - 1);
                continue '__tco;
            }
            _ => return false,
        }
    }
}

pub fn isImpure(mut inExp: metamodelica::Ref<DAE::Exp>) -> Result<bool> {
    let mut outBoolean: bool;
    outBoolean = isConst(inExp.clone())?;
    (_, outBoolean) = traverseExpTopDown(inExp, &fnptr!(isImpureWork, metamodelica::Ref<DAE::Exp>, bool), false)?;
    Ok(outBoolean)
}

fn isImpureWork(
    mut inExp: metamodelica::Ref<DAE::Exp>,
    mut isImpure: bool,
) -> (metamodelica::Ref<DAE::Exp>, bool, bool) {
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    let mut cont: bool;
    let mut outImpure: bool;
    (outExp, cont, outImpure) = (::match_deref::match_deref! { match &((inExp.clone(), isImpure)) {
        (_, true) => (inExp, true, true),
        (Deref @ DAE::Exp::CALL { attr: Deref @ DAE::CallAttributes { isImpure: true, .. }, .. }, _) => (inExp, false, true),
        (Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "alarm" }, attr: Deref @ DAE::CallAttributes { builtin: true, .. }, .. }, _) => (inExp, false, true),
        (Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "compareFilesAndMove" }, attr: Deref @ DAE::CallAttributes { builtin: true, .. }, .. }, _) => (inExp, false, true),
        (Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "delay" }, attr: Deref @ DAE::CallAttributes { builtin: true, .. }, .. }, _) => (inExp, false, true),
        (Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "initial" }, attr: Deref @ DAE::CallAttributes { builtin: true, .. }, .. }, _) => (inExp, false, true),
        (Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "print" }, attr: Deref @ DAE::CallAttributes { builtin: true, .. }, .. }, _) => (inExp, false, true),
        (Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "readFile" }, attr: Deref @ DAE::CallAttributes { builtin: true, .. }, .. }, _) => (inExp, false, true),
        (Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "sample" }, attr: Deref @ DAE::CallAttributes { builtin: true, .. }, .. }, _) => (inExp, false, true),
        (Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "system" }, attr: Deref @ DAE::CallAttributes { builtin: true, .. }, .. }, _) => (inExp, false, true),
        (Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "system_parallel" }, attr: Deref @ DAE::CallAttributes { builtin: true, .. }, .. }, _) => (inExp, false, true),
        (Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "terminal" }, attr: Deref @ DAE::CallAttributes { builtin: true, .. }, .. }, _) => (inExp, false, true),
        (Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "writeFile" }, attr: Deref @ DAE::CallAttributes { builtin: true, .. }, .. }, _) => (inExp, false, true),
        _ => (inExp, true, false),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    (outExp, cont, outImpure)
}

pub fn containsRecordType(mut inExp: metamodelica::Ref<DAE::Exp>) -> Result<bool> {
    let mut isRec: bool;
    (_, isRec) = traverseExpTopDown(
        inExp,
        &fnptr!(containsRecordTypeWork, metamodelica::Ref<DAE::Exp>, bool),
        false,
    )?;
    Ok(isRec)
}

fn containsRecordTypeWork(
    mut inExp: metamodelica::Ref<DAE::Exp>,
    mut inRec: bool,
) -> (metamodelica::Ref<DAE::Exp>, bool, bool) {
    let mut outExp: metamodelica::Ref<DAE::Exp> = inExp.clone();
    let mut cont: bool = false;
    let mut outRec: bool = inRec;
    if !(inRec) {
        (outExp, cont, outRec) = 'mc: {
            let __mc_input = &*inExp;
            if let Ok(__v) = (|| -> Result<_> {
                ::match_deref::match_deref! { match &__mc_input {
                    Deref @ DAE::Exp::RECORD { .. } => {
                        Ok((inExp.clone(), false, true))
                    }
                    _ => return Err("nomatch"),
                }}
            })() {
                break 'mc __v;
            }
            if let Ok(__v) = (|| -> Result<_> {
                ::match_deref::match_deref! { match &__mc_input {
                    Deref @ DAE::Exp::CALL { expLst, attr: Deref @ DAE::CallAttributes { ty, .. }, .. } => {
                        let mut subRec: bool;
                        subRec = isRecordType(metamodelica::AsArg::as_arg(&ty));
                        if !(subRec) {
                            for mut exp in &*expLst.clone() {
                                subRec = containsRecordType(exp.clone())?;
                                if subRec {
                                            break;
                                }
                            }
                        }
                        Ok((inExp.clone(), !(subRec), subRec))
                    }
                    _ => return Err("nomatch"),
                }}
            })() {
                break 'mc __v;
            }
            if let Ok(__v) = (|| -> Result<_> {
                ::match_deref::match_deref! { match &__mc_input {
                    _ => {
                        Ok((inExp.clone(), true, false))
                    }
                    _ => return Err("nomatch"),
                }}
            })() {
                break 'mc __v;
            }
            panic!("matchcontinue: no arm matched")
        };
    }
    (outExp, cont, outRec)
}

pub fn isEvaluatedConst(mut inExp: &metamodelica::Ref<DAE::Exp>) -> bool {
    let mut outBoolean: bool;
    outBoolean = (match &**inExp {
        DAE::Exp::ICONST { .. } => true,
        DAE::Exp::RCONST { .. } => true,
        DAE::Exp::BCONST { .. } => true,
        DAE::Exp::SCONST { .. } => true,
        DAE::Exp::ENUM_LITERAL { .. } => true,
        _ => false,
    });
    outBoolean
}

pub fn getEvaluatedConstInteger(mut inExp: &metamodelica::Ref<DAE::Exp>) -> Result<i32> {
    let mut val: i32;
    val = (match &**inExp {
        DAE::Exp::ICONST { integer } => integer.clone(),
        DAE::Exp::RCONST { .. } => {
            let mut integer: i32;
            let __pa0 = ::match_deref::match_deref! { match &(realExpIntLit(inExp)) {
                Some(__pa0) => __pa0.clone(),
                _ => return Err("pattern mismatch"),
            } };
            integer = metamodelica::Own::own(__pa0);
            integer
        }
        _ => return Err("fail"),
    });
    Ok(val)
}

pub fn getEvaluatedConstReal(mut inExp: &metamodelica::Ref<DAE::Exp>) -> Result<metamodelica::Real> {
    let mut val: metamodelica::Real;
    val = (match &**inExp {
        DAE::Exp::RCONST { real: __inExp_real } => __inExp_real.clone(),
        DAE::Exp::ICONST {
            integer: __inExp_integer,
        } => intReal(__inExp_integer.clone()),
        _ => return Err("fail"),
    });
    Ok(val)
}

pub fn isConst(mut inExp: metamodelica::Ref<DAE::Exp>) -> Result<bool> {
    '__tco: loop {
        ::match_deref::match_deref! { match &(inExp) {
            Deref @ DAE::Exp::ICONST { .. } => {
                return Ok(true)
            },
            Deref @ DAE::Exp::RCONST { .. } => {
                return Ok(true)
            },
            Deref @ DAE::Exp::BCONST { .. } => {
                return Ok(true)
            },
            Deref @ DAE::Exp::SCONST { .. } => {
                return Ok(true)
            },
            Deref @ DAE::Exp::ENUM_LITERAL { .. } => {
                return Ok(true)
            },
            Deref @ DAE::Exp::UNARY { exp: e, .. } => {
                { inExp = e.clone(); continue '__tco; }
            },
            Deref @ DAE::Exp::CAST { exp: e, .. } => {
                { inExp = e.clone(); continue '__tco; }
            },
            Deref @ DAE::Exp::BINARY { exp1: e1, operator: _, exp2: e2 } => {
                let mut res: bool;
                res = isConst(e2.clone())?;
                if (res) {{ inExp = e1.clone(); continue '__tco; }} else {return Ok(false)}
            },
            Deref @ DAE::Exp::IFEXP { expCond: e, expThen: e1, expElse: e2 } => {
                let mut res: bool;
                res = isConst(e2.clone())?;
                if res {
                    res = isConst(e1.clone())?;
                }
                if (res) {{ inExp = e.clone(); continue '__tco; }} else {return Ok(false)}
            },
            Deref @ DAE::Exp::LBINARY { exp1: e1, exp2: e2, .. } => {
                let mut res: bool;
                res = isConst(e2.clone())?;
                if (res) {{ inExp = e1.clone(); continue '__tco; }} else {return Ok(false)}
            },
            Deref @ DAE::Exp::LUNARY { exp: e, .. } => {
                { inExp = e.clone(); continue '__tco; }
            },
            Deref @ DAE::Exp::RELATION { exp1: e1, exp2: e2, .. } => {
                let mut res: bool;
                res = isConst(e2.clone())?;
                if (res) {{ inExp = e1.clone(); continue '__tco; }} else {return Ok(false)}
            },
            Deref @ DAE::Exp::ARRAY { array: ae, .. } => {
                return Ok(isConstWorkList(ae.clone())?)
            },
            Deref @ DAE::Exp::MATRIX { matrix, .. } => {
                return Ok(isConstWorkListList(matrix.clone())?)
            },
            Deref @ DAE::Exp::RANGE { start: e1, step: None, stop: e2, .. } => {
                let mut res: bool;
                res = isConst(e2.clone())?;
                if (res) {{ inExp = e1.clone(); continue '__tco; }} else {return Ok(false)}
            },
            Deref @ DAE::Exp::RANGE { start: e, step: Some(e1), stop: e2, .. } => {
                let mut res: bool;
                res = isConst(e2.clone())?;
                if res {
                    res = isConst(e1.clone())?;
                }
                if (res) {{ inExp = e.clone(); continue '__tco; }} else {return Ok(false)}
            },
            Deref @ DAE::Exp::PARTEVALFUNCTION { expList: ae, .. } => {
                return Ok(isConstWorkList(ae.clone())?)
            },
            Deref @ DAE::Exp::TUPLE { PR: ae } => {
                return Ok(isConstWorkList(ae.clone())?)
            },
            Deref @ DAE::Exp::ASUB { exp: e, sub: subs } => {
                let mut res: bool;
                let mut ae: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                ae = ({
            let mut __acc: metamodelica::List<metamodelica::Ref<DAE::Exp>> = metamodelica::nil();
            for mut sub in (subs.clone()).into_iter().cloned() {
                let __x = getSubscriptExp(&(sub.clone()))?;
                __acc = cons(__x, __acc);
            }
            __acc.reverse()
        });
                res = isConst(e.clone())?;
                if (res) {return Ok(isConstWorkList(ae)?)} else {return Ok(false)}
            },
            Deref @ DAE::Exp::TSUB { exp: e, .. } => {
                { inExp = e.clone(); continue '__tco; }
            },
            Deref @ DAE::Exp::SIZE { exp: e, sz: None } => {
                { inExp = e.clone(); continue '__tco; }
            },
            Deref @ DAE::Exp::SIZE { exp: e1, sz: Some(e2) } => {
                let mut res: bool;
                res = isConst(e2.clone())?;
                if (res) {{ inExp = e1.clone(); continue '__tco; }} else {return Ok(false)}
            },
            Deref @ DAE::Exp::CALL { expLst: ae, attr: Deref @ DAE::CallAttributes { builtin: false, isImpure: false, .. }, .. } => {
                return Ok(isConstWorkList(ae.clone())?)
            },
            Deref @ DAE::Exp::CALL { path, expLst: ae, attr: Deref @ DAE::CallAttributes { builtin: true, .. } } => {
                if (listMember(AbsynUtil::pathFirstIdent(metamodelica::AsArg::as_arg(&path)), list![literal!("initial"), literal!("terminal"), literal!("sample")])) {return Ok(false)} else {return Ok(isConstWorkList(ae.clone())?)}
            },
            Deref @ DAE::Exp::RECORD { exps: ae, .. } => {
                return Ok(isConstWorkList(ae.clone())?)
            },
            Deref @ DAE::Exp::REDUCTION { expr: e1, iterators: Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::ReductionIterator { exp: e2, .. }, tail: Deref @ metamodelica::ListNode::Nil }, .. } => {
                let mut res: bool;
                res = isConst(e2.clone())?;
                if (res) {{ inExp = e1.clone(); continue '__tco; }} else {return Ok(false)}
            },
            Deref @ DAE::Exp::BOX { exp: e } => {
                { inExp = e.clone(); continue '__tco; }
            },
            _ => {
                return Ok(false)
            },
            _ => return Err("match: no arm matched"),
        } }
    }
}

fn isConstValueWork(mut inExp: &metamodelica::Ref<DAE::Exp>) -> Result<bool> {
    let mut outBoolean: bool;
    outBoolean = (match &**inExp {
        DAE::Exp::ICONST { .. } => true,
        DAE::Exp::RCONST { .. } => true,
        DAE::Exp::BCONST { .. } => true,
        DAE::Exp::SCONST { .. } => true,
        DAE::Exp::ENUM_LITERAL { .. } => true,
        DAE::Exp::ARRAY { array: ae, .. } => isConstValueWorkList(ae.clone())?,
        DAE::Exp::MATRIX { matrix, .. } => isConstValueWorkListList(matrix.clone())?,
        DAE::Exp::RECORD { .. } => true,
        DAE::Exp::METARECORDCALL { .. } => true,
        _ => false,
    });
    Ok(outBoolean)
}

pub fn isConstValue(mut inExp: &metamodelica::Ref<DAE::Exp>) -> Result<bool> {
    let mut outBoolean: bool;
    outBoolean = isConstValueWork(inExp)?;
    Ok(outBoolean)
}

pub fn isConstWorkList(mut inExps: metamodelica::List<metamodelica::Ref<DAE::Exp>>) -> Result<bool> {
    let mut outBoolean: bool;
    let mut e: metamodelica::Ref<DAE::Exp>;
    let mut exps: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
    let mut b: bool = true;
    exps = inExps;
    while b && !((exps).is_empty()) {
        let (__pa0, __pa1) = ::match_deref::match_deref! { match &(exps) {
            Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
            _ => return Err("pattern mismatch"),
        } };
        e = metamodelica::Own::own(__pa0);
        exps = metamodelica::Own::own(__pa1);
        b = isConst(e)?;
    }
    outBoolean = b;
    Ok(outBoolean)
}

fn isConstWorkListList(
    mut inExps: metamodelica::List<metamodelica::List<metamodelica::Ref<DAE::Exp>>>,
) -> Result<bool> {
    let mut outIsConst: bool;
    let mut e: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
    let mut exps: metamodelica::List<metamodelica::List<metamodelica::Ref<DAE::Exp>>>;
    let mut b: bool = true;
    exps = inExps;
    while b && !((exps).is_empty()) {
        let (__pa0, __pa1) = ::match_deref::match_deref! { match &(exps) {
            Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
            _ => return Err("pattern mismatch"),
        } };
        e = metamodelica::Own::own(__pa0);
        exps = metamodelica::Own::own(__pa1);
        b = isConstWorkList(e)?;
    }
    outIsConst = b;
    Ok(outIsConst)
}

fn isConstValueWorkList(mut inExps: metamodelica::List<metamodelica::Ref<DAE::Exp>>) -> Result<bool> {
    let mut outBoolean: bool;
    let mut e: metamodelica::Ref<DAE::Exp>;
    let mut exps: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
    let mut b: bool = true;
    exps = inExps;
    while b && !((exps).is_empty()) {
        let (__pa0, __pa1) = ::match_deref::match_deref! { match &(exps) {
            Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
            _ => return Err("pattern mismatch"),
        } };
        e = metamodelica::Own::own(__pa0);
        exps = metamodelica::Own::own(__pa1);
        b = isConstValueWork(&e)?;
    }
    outBoolean = b;
    Ok(outBoolean)
}

fn isConstValueWorkListList(
    mut inExps: metamodelica::List<metamodelica::List<metamodelica::Ref<DAE::Exp>>>,
) -> Result<bool> {
    let mut outIsConst: bool;
    let mut e: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
    let mut exps: metamodelica::List<metamodelica::List<metamodelica::Ref<DAE::Exp>>>;
    let mut b: bool = true;
    exps = inExps;
    while b && !((exps).is_empty()) {
        let (__pa0, __pa1) = ::match_deref::match_deref! { match &(exps) {
            Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
            _ => return Err("pattern mismatch"),
        } };
        e = metamodelica::Own::own(__pa0);
        exps = metamodelica::Own::own(__pa1);
        b = isConstValueWorkList(e)?;
    }
    outIsConst = b;
    Ok(outIsConst)
}

pub fn isNotConst(mut e: metamodelica::Ref<DAE::Exp>) -> Result<bool> {
    let mut nb: bool;
    let mut b: bool;
    b = isConst(e)?;
    nb = boolNot(b);
    Ok(nb)
}

pub fn isRelation(mut inExp: &metamodelica::Ref<DAE::Exp>) -> bool {
    let mut outBoolean: bool;
    outBoolean = (match &**inExp {
        DAE::Exp::RELATION { .. } => true,
        _ => false,
    });
    outBoolean
}

pub(crate) fn isEventTriggeringFunctionExp(mut inExp: &metamodelica::Ref<DAE::Exp>) -> bool {
    let mut outB: bool;
    outB = (::match_deref::match_deref! { match inExp {
        Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "div" }, .. } => true,
        Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "mod" }, .. } => true,
        Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "rem" }, .. } => true,
        Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "ceil" }, .. } => true,
        Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "floor" }, .. } => true,
        Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "integer" }, .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outB
}

pub fn isAddOrSub(mut op: &DAE::Operator) -> bool {
    let mut res: bool;
    res = isAdd(op) || isSub(op);
    res
}

pub fn isAdd(mut op: &DAE::Operator) -> bool {
    let mut res: bool;
    res = (match op.clone() {
        DAE::Operator::ADD { .. } => true,
        DAE::Operator::ADD_ARR { .. } => true,
        _ => false,
    });
    res
}

pub(crate) fn isSub(mut op: &DAE::Operator) -> bool {
    let mut res: bool;
    res = (match op.clone() {
        DAE::Operator::SUB { .. } => true,
        DAE::Operator::SUB_ARR { .. } => true,
        _ => false,
    });
    res
}

pub(crate) fn isAddOrSubBinary(mut iExp: &metamodelica::Ref<DAE::Exp>) -> bool {
    let mut res: bool;
    let mut op: DAE::Operator;
    res = (match &**iExp {
        DAE::Exp::BINARY {
            exp1: _,
            operator: __esc_op,
            exp2: _,
        } => {
            op = (*__esc_op).clone();
            isAddOrSub(metamodelica::AsArg::as_arg(&op))
        }
        _ => false,
    });
    res
}

pub fn isMulOrDiv(mut op: &DAE::Operator) -> bool {
    let mut res: bool = isMul(op) || isDiv(op);
    res
}

pub(crate) fn isMul(mut op: &DAE::Operator) -> bool {
    let mut res: bool;
    res = (match op.clone() {
        DAE::Operator::MUL { .. } => true,
        DAE::Operator::MUL_ARR { .. } => true,
        _ => false,
    });
    res
}

pub fn isDiv(mut op: &DAE::Operator) -> bool {
    let mut res: bool;
    res = (match op.clone() {
        DAE::Operator::DIV { .. } => true,
        DAE::Operator::DIV_ARR { .. } => true,
        _ => false,
    });
    res
}

pub fn isDivBinary(mut iExp: &metamodelica::Ref<DAE::Exp>) -> bool {
    let mut res: bool;
    let mut op: DAE::Operator;
    res = (match &**iExp {
        DAE::Exp::BINARY {
            exp1: _,
            operator: __esc_op,
            exp2: _,
        } => {
            op = (*__esc_op).clone();
            isDiv(metamodelica::AsArg::as_arg(&op))
        }
        _ => false,
    });
    res
}

pub(crate) fn isMulorDivBinary(mut iExp: &metamodelica::Ref<DAE::Exp>) -> bool {
    let mut res: bool;
    let mut op: DAE::Operator;
    res = (match &**iExp {
        DAE::Exp::BINARY {
            exp1: _,
            operator: __esc_op,
            exp2: _,
        } => {
            op = (*__esc_op).clone();
            isMulOrDiv(metamodelica::AsArg::as_arg(&op))
        }
        _ => false,
    });
    res
}

pub fn isPow(mut op: &DAE::Operator) -> bool {
    let mut res: bool;
    res = (match op.clone() {
        DAE::Operator::POW { .. } => true,
        _ => false,
    });
    res
}

pub(crate) fn isFunCall(mut iExp: &metamodelica::Ref<DAE::Exp>, mut name: &ArcStr) -> bool {
    let mut res: bool;
    res = (::match_deref::match_deref! { match iExp {
        Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: name_ }, .. } => {
            metamodelica::stringEq(&name_, &name)
        },
        _ => {
            false
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    res
}

pub fn equalTypes(mut t1: &metamodelica::Ref<DAE::Type>, mut t2: &metamodelica::Ref<DAE::Type>) -> bool {
    let mut b: bool;
    b = 'mc: {
        let __mc_input = (&**t1, &**t2);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Type::T_INTEGER { .. }, Deref @ DAE::Type::T_INTEGER { .. }) => {
                    Ok(true)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Type::T_REAL { .. }, Deref @ DAE::Type::T_REAL { .. }) => {
                    Ok(true)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Type::T_STRING { .. }, Deref @ DAE::Type::T_STRING { .. }) => {
                    Ok(true)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Type::T_BOOL { .. }, Deref @ DAE::Type::T_BOOL { .. }) => {
                    Ok(true)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Type::T_CLOCK { .. }, Deref @ DAE::Type::T_CLOCK { .. }) => {
                    Ok(true)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Type::T_COMPLEX { varLst: vars1, .. }, Deref @ DAE::Type::T_COMPLEX { varLst: vars2, .. }) => {
                    Ok(equalTypesComplexVars(metamodelica::AsArg::as_arg(&vars1), metamodelica::AsArg::as_arg(&vars2)))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Type::T_ARRAY { ty: ty1, dims: ad1 }, Deref @ DAE::Type::T_ARRAY { ty: ty2, dims: ad2 }) => {
                    let mut li1: metamodelica::List<i32>;
                    let mut li2: metamodelica::List<i32>;
                    li1 = List::map(ad1.clone(), &move |__a0: metamodelica::Ref<DAE::Dimension>| dimensionSize(&__a0))?;
                    li2 = List::map(ad2.clone(), &move |__a0: metamodelica::Ref<DAE::Dimension>| dimensionSize(&__a0))?;
                    let true = (List::isEqualOnTrue(li1.clone(), li2.clone(), &fnptr!(intEq, i32, i32))?) else { return Err("pattern mismatch") };
                    let true = (equalTypes(metamodelica::AsArg::as_arg(&ty1), metamodelica::AsArg::as_arg(&ty2))) else { return Err("pattern mismatch") };
                    Ok(true)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(false)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    b
}

fn equalTypesComplexVars(
    mut inVars1: &metamodelica::List<metamodelica::Ref<DAE::Var>>,
    mut inVars2: &metamodelica::List<metamodelica::Ref<DAE::Var>>,
) -> bool {
    let mut b: bool;
    b = 'mc: {
        let __mc_input = (&**inVars1, &**inVars2);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Nil, Deref @ metamodelica::ListNode::Nil) => {
                    Ok(true)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Var { name: s1, ty: t1, .. }, tail: vars1 }, Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Var { name: s2, ty: t2, .. }, tail: vars2 }) => {
                    let true = (stringEq(&s1, &s2)) else { return Err("pattern mismatch") };
                    let true = (equalTypes(metamodelica::AsArg::as_arg(&t1), metamodelica::AsArg::as_arg(&t2))) else { return Err("pattern mismatch") };
                    Ok(equalTypesComplexVars(metamodelica::AsArg::as_arg(&vars1), metamodelica::AsArg::as_arg(&vars2)))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(false)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    b
}

pub fn typeBuiltin(mut inType: &metamodelica::Ref<DAE::Type>) -> bool {
    let mut outBoolean: bool;
    outBoolean = (match &**inType {
        DAE::Type::T_INTEGER { .. } => true,
        DAE::Type::T_REAL { .. } => true,
        DAE::Type::T_STRING { .. } => true,
        DAE::Type::T_BOOL { .. } => true,
        DAE::Type::T_CLOCK { .. } => true,
        _ => false,
    });
    outBoolean
}

pub(crate) fn isWholeDim(mut s: &metamodelica::Ref<DAE::Subscript>) -> bool {
    let mut b: bool;
    b = (match &**s {
        DAE::Subscript::WHOLEDIM { .. } => true,
        _ => false,
    });
    b
}

pub(crate) fn isInt<'__b>(mut it: &'__b metamodelica::Ref<DAE::Type>) -> bool {
    '__tco: loop {
        match &**it {
            DAE::Type::T_INTEGER { .. } => return true,
            DAE::Type::T_ARRAY { ty: t1, .. } => {
                it = t1;
                continue '__tco;
            }
            _ => return false,
        }
    }
}

pub(crate) fn isReal<'__b>(mut it: &'__b metamodelica::Ref<DAE::Type>) -> bool {
    '__tco: loop {
        match &**it {
            DAE::Type::T_REAL { .. } => return true,
            DAE::Type::T_ARRAY { ty: t1, .. } => {
                it = t1;
                continue '__tco;
            }
            _ => return false,
        }
    }
}

pub(crate) fn isExpReal(mut e: metamodelica::Ref<DAE::Exp>) -> Result<bool> {
    let mut re: bool;
    re = isReal(&(r#typeof(e)?));
    Ok(re)
}

pub(crate) fn isConstZeroLength(mut inExp: &metamodelica::Ref<DAE::Exp>) -> bool {
    let mut outBoolean: bool;
    outBoolean = (::match_deref::match_deref! { match inExp {
        Deref @ DAE::Exp::ARRAY { array: Deref @ metamodelica::ListNode::Nil, .. } => true,
        Deref @ DAE::Exp::MATRIX { matrix: Deref @ metamodelica::ListNode::Nil, .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outBoolean
}

pub fn isConstFalse(mut inExp: &metamodelica::Ref<DAE::Exp>) -> bool {
    let mut outBoolean: bool;
    outBoolean = (match &**inExp {
        DAE::Exp::BCONST { bool: false } => true,
        _ => false,
    });
    outBoolean
}

pub fn isConstTrue(mut inExp: &metamodelica::Ref<DAE::Exp>) -> bool {
    let mut outBoolean: bool;
    outBoolean = (match &**inExp {
        DAE::Exp::BCONST { bool: true } => true,
        _ => false,
    });
    outBoolean
}

pub fn isConstOne(mut inExp: &metamodelica::Ref<DAE::Exp>) -> bool {
    let mut outBoolean: bool;
    outBoolean = (match &**inExp {
        DAE::Exp::RCONST { real: rval } => realEq(rval.clone(), metamodelica::OrderedFloat(1.0_f64)),
        DAE::Exp::ICONST { integer: ival } => intEq(ival.clone(), 1),
        _ => false,
    });
    outBoolean
}

pub fn isConstMinusOne(mut inExp: &metamodelica::Ref<DAE::Exp>) -> bool {
    let mut outBoolean: bool;
    outBoolean = (match &**inExp {
        DAE::Exp::RCONST { real: rval } => realEq(rval.clone(), metamodelica::OrderedFloat(-1.0_f64)),
        DAE::Exp::ICONST { integer: ival } => intEq(ival.clone(), -1),
        _ => false,
    });
    outBoolean
}

pub(crate) fn isGreatereqOrLesseq(mut op: &DAE::Operator) -> bool {
    let mut b: bool;
    b = (match op.clone() {
        DAE::Operator::GREATEREQ { .. } => true,
        DAE::Operator::LESSEQ { .. } => true,
        _ => false,
    });
    b
}

pub(crate) fn isLesseqOrLess(mut op: &DAE::Operator) -> bool {
    let mut b: bool;
    b = (match op.clone() {
        DAE::Operator::LESS { .. } => true,
        DAE::Operator::LESSEQ { .. } => true,
        _ => false,
    });
    b
}

pub fn containVectorFunctioncall(mut inExp: metamodelica::Ref<DAE::Exp>) -> Result<bool> {
    '__tco: loop {
        ::match_deref::match_deref! { match &(inExp) {
            Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "der" }, .. } => {
                return Ok(false)
            },
            Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "pre" }, .. } => {
                return Ok(false)
            },
            Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "previous" }, .. } => {
                return Ok(false)
            },
            Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "inStream" }, .. } => {
                return Ok(false)
            },
            Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "actualStream" }, .. } => {
                return Ok(false)
            },
            Deref @ DAE::Exp::CALL { attr: Deref @ DAE::CallAttributes { ty: Deref @ DAE::Type::T_ARRAY { .. }, .. }, .. } => {
                return Ok(true)
            },
            Deref @ DAE::Exp::CALL { .. } => {
                return Ok(false)
            },
            Deref @ DAE::Exp::PARTEVALFUNCTION { expList: elst, .. } => {
                return Ok(List::any(metamodelica::AsArg::as_arg(&elst), &containVectorFunctioncall)?)
            },
            Deref @ DAE::Exp::BINARY { exp1: e1, .. } if (containVectorFunctioncall(e1.clone())?) => {
                return Ok(true)
            },
            Deref @ DAE::Exp::BINARY { exp2: e2, .. } if (containVectorFunctioncall(e2.clone())?) => {
                return Ok(true)
            },
            Deref @ DAE::Exp::UNARY { exp: e, .. } => {
                { inExp = e.clone(); continue '__tco; }
            },
            Deref @ DAE::Exp::LBINARY { exp1: e1, .. } if (containVectorFunctioncall(e1.clone())?) => {
                return Ok(true)
            },
            Deref @ DAE::Exp::LBINARY { exp2: e2, .. } if (containVectorFunctioncall(e2.clone())?) => {
                return Ok(true)
            },
            Deref @ DAE::Exp::LUNARY { exp: e, .. } => {
                { inExp = e.clone(); continue '__tco; }
            },
            Deref @ DAE::Exp::RELATION { exp1: e1, .. } if (containVectorFunctioncall(e1.clone())?) => {
                return Ok(true)
            },
            Deref @ DAE::Exp::RELATION { exp2: e2, .. } if (containVectorFunctioncall(e2.clone())?) => {
                return Ok(true)
            },
            Deref @ DAE::Exp::IFEXP { expCond: e1, .. } if (containVectorFunctioncall(e1.clone())?) => {
                return Ok(true)
            },
            Deref @ DAE::Exp::IFEXP { expThen: e2, .. } if (containVectorFunctioncall(e2.clone())?) => {
                return Ok(true)
            },
            Deref @ DAE::Exp::IFEXP { expElse: e3, .. } if (containVectorFunctioncall(e3.clone())?) => {
                return Ok(true)
            },
            Deref @ DAE::Exp::ARRAY { array: elst, .. } => {
                return Ok(List::any(metamodelica::AsArg::as_arg(&elst), &containVectorFunctioncall)?)
            },
            Deref @ DAE::Exp::MATRIX { matrix: explst, .. } => {
                let mut res: bool;
                let mut flatexplst: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                flatexplst = List::flatten(explst.clone())?;
                return Ok(List::any(&flatexplst, &containVectorFunctioncall)?)
            },
            Deref @ DAE::Exp::RANGE { start: e1, .. } if (containVectorFunctioncall(e1.clone())?) => {
                return Ok(true)
            },
            Deref @ DAE::Exp::RANGE { stop: e2, .. } if (containVectorFunctioncall(e2.clone())?) => {
                return Ok(true)
            },
            Deref @ DAE::Exp::RANGE { step: Some(e), .. } if (containVectorFunctioncall(e.clone())?) => {
                return Ok(true)
            },
            Deref @ DAE::Exp::TUPLE { PR: elst } => {
                return Ok(List::any(metamodelica::AsArg::as_arg(&elst), &containVectorFunctioncall)?)
            },
            Deref @ DAE::Exp::CAST { exp: e, .. } => {
                { inExp = e.clone(); continue '__tco; }
            },
            Deref @ DAE::Exp::SIZE { exp: e1, .. } if (containVectorFunctioncall(e1.clone())?) => {
                return Ok(true)
            },
            Deref @ DAE::Exp::SIZE { sz: Some(e2), .. } if (containVectorFunctioncall(e2.clone())?) => {
                return Ok(true)
            },
            _ => {
                return Ok(false)
            },
            _ => return Err("match: no arm matched"),
        } }
    }
}

pub fn containFunctioncall(mut inExp: metamodelica::Ref<DAE::Exp>) -> Result<bool> {
    '__tco: loop {
        ::match_deref::match_deref! { match &(inExp) {
            Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "der" }, .. } => {
                return Ok(false)
            },
            Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "pre" }, .. } => {
                return Ok(false)
            },
            Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "previous" }, .. } => {
                return Ok(false)
            },
            Deref @ DAE::Exp::CALL { .. } => {
                return Ok(true)
            },
            Deref @ DAE::Exp::PARTEVALFUNCTION { expList: elst, .. } => {
                let mut res: bool;
                return Ok(List::any(metamodelica::AsArg::as_arg(&elst), &containFunctioncall)?)
            },
            Deref @ DAE::Exp::BINARY { exp1: e1, .. } if (containFunctioncall(e1.clone())?) => {
                return Ok(true)
            },
            Deref @ DAE::Exp::BINARY { exp2: e2, .. } if (containFunctioncall(e2.clone())?) => {
                return Ok(true)
            },
            Deref @ DAE::Exp::UNARY { exp: e, .. } => {
                { inExp = e.clone(); continue '__tco; }
            },
            Deref @ DAE::Exp::LBINARY { exp1: e1, .. } if (containFunctioncall(e1.clone())?) => {
                return Ok(true)
            },
            Deref @ DAE::Exp::LBINARY { exp2: e2, .. } if (containFunctioncall(e2.clone())?) => {
                return Ok(true)
            },
            Deref @ DAE::Exp::LUNARY { exp: e, .. } => {
                { inExp = e.clone(); continue '__tco; }
            },
            Deref @ DAE::Exp::RELATION { exp1: e1, .. } if (containFunctioncall(e1.clone())?) => {
                return Ok(true)
            },
            Deref @ DAE::Exp::RELATION { exp2: e2, .. } if (containFunctioncall(e2.clone())?) => {
                return Ok(true)
            },
            Deref @ DAE::Exp::IFEXP { expCond: e1, .. } if (containFunctioncall(e1.clone())?) => {
                return Ok(true)
            },
            Deref @ DAE::Exp::IFEXP { expThen: e2, .. } if (containFunctioncall(e2.clone())?) => {
                return Ok(true)
            },
            Deref @ DAE::Exp::IFEXP { expElse: e3, .. } if (containFunctioncall(e3.clone())?) => {
                return Ok(true)
            },
            Deref @ DAE::Exp::ARRAY { array: elst, .. } => {
                return Ok(List::any(metamodelica::AsArg::as_arg(&elst), &containFunctioncall)?)
            },
            Deref @ DAE::Exp::MATRIX { matrix: explst, .. } => {
                let mut res: bool;
                let mut flatexplst: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                flatexplst = List::flatten(explst.clone())?;
                return Ok(List::any(&flatexplst, &containFunctioncall)?)
            },
            Deref @ DAE::Exp::RANGE { start: e1, .. } if (containFunctioncall(e1.clone())?) => {
                return Ok(true)
            },
            Deref @ DAE::Exp::RANGE { stop: e2, .. } if (containFunctioncall(e2.clone())?) => {
                return Ok(true)
            },
            Deref @ DAE::Exp::RANGE { step: Some(e), .. } if (containFunctioncall(e.clone())?) => {
                return Ok(true)
            },
            Deref @ DAE::Exp::TUPLE { PR: elst } => {
                return Ok(List::any(metamodelica::AsArg::as_arg(&elst), &containVectorFunctioncall)?)
            },
            Deref @ DAE::Exp::CAST { exp: e, .. } => {
                { inExp = e.clone(); continue '__tco; }
            },
            Deref @ DAE::Exp::ASUB { exp: e, .. } => {
                { inExp = e.clone(); continue '__tco; }
            },
            Deref @ DAE::Exp::SIZE { exp: e1, .. } if (containFunctioncall(e1.clone())?) => {
                return Ok(true)
            },
            Deref @ DAE::Exp::SIZE { sz: Some(e2), .. } if (containFunctioncall(e2.clone())?) => {
                return Ok(true)
            },
            _ => {
                return Ok(false)
            },
            _ => return Err("match: no arm matched"),
        } }
    }
}

pub(crate) fn expIntOrder(
    mut expectedValue: i32,
    mut integers: metamodelica::List<metamodelica::Ref<DAE::Exp>>,
) -> bool {
    '__tco: loop {
        ::match_deref::match_deref! { match &((expectedValue, integers)) {
            (_, Deref @ metamodelica::ListNode::Nil) => {
                return true
            },
            (x1, Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Exp::ICONST { integer: x2 }, tail: expl }) if (intEq(x1.clone(), x2.clone())) => {
                { (expectedValue, integers) = (x1.clone() + 1, expl.clone()); continue '__tco; }
            },
            _ => {
                return false
            },
            _ => unreachable!("tail-call lowered match: no arm matched"),
        } }
    }
}

pub fn isArray(mut inExp: &metamodelica::Ref<DAE::Exp>) -> bool {
    let mut outB: bool;
    outB = (::match_deref::match_deref! { match inExp {
        Deref @ DAE::Exp::ARRAY { .. } => true,
        Deref @ DAE::Exp::UNARY { operator: DAE::Operator::UMINUS_ARR { .. }, exp: Deref @ DAE::Exp::ARRAY { .. } } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outB
}

pub fn isMetaArray(mut inExp: metamodelica::Ref<DAE::Exp>) -> Result<bool> {
    let mut outB: bool;
    outB = Types::isMetaArray(&(r#typeof(inExp)?));
    Ok(outB)
}

pub fn isMatrix(mut inExp: &metamodelica::Ref<DAE::Exp>) -> bool {
    let mut outB: bool;
    outB = (::match_deref::match_deref! { match inExp {
        Deref @ DAE::Exp::MATRIX { .. } => true,
        Deref @ DAE::Exp::UNARY { operator: DAE::Operator::UMINUS_ARR { .. }, exp: Deref @ DAE::Exp::MATRIX { .. } } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outB
}

pub(crate) fn isVector(mut inExp: &metamodelica::Ref<DAE::Exp>) -> bool {
    let mut outIsVector: bool;
    outIsVector = (::match_deref::match_deref! { match inExp {
        Deref @ DAE::Exp::ARRAY { ty: Deref @ DAE::Type::T_ARRAY { ty: Deref @ DAE::Type::T_ARRAY { .. }, .. }, .. } => false,
        Deref @ DAE::Exp::ARRAY { ty: Deref @ DAE::Type::T_ARRAY { dims: Deref @ metamodelica::ListNode::Cons { head: _, tail: Deref @ metamodelica::ListNode::Nil }, .. }, .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outIsVector
}

pub(crate) fn isUnary(mut inExp: &metamodelica::Ref<DAE::Exp>) -> bool {
    let mut outB: bool;
    outB = (match &**inExp {
        DAE::Exp::UNARY { .. } => true,
        _ => false,
    });
    outB
}

pub fn isBinary(mut inExp: &metamodelica::Ref<DAE::Exp>) -> bool {
    let mut outB: bool;
    outB = (match &**inExp {
        DAE::Exp::BINARY { .. } => true,
        _ => false,
    });
    outB
}

pub fn isNegativeUnary(mut inExp: &metamodelica::Ref<DAE::Exp>) -> bool {
    let mut outB: bool;
    outB = (match &**inExp {
        DAE::Exp::UNARY {
            operator: DAE::Operator::UMINUS { .. },
            ..
        } => true,
        _ => false,
    });
    outB
}

pub fn isCref(mut inExp: &metamodelica::Ref<DAE::Exp>) -> bool {
    let mut outIsCref: bool;
    outIsCref = (match &**inExp {
        DAE::Exp::CREF { .. } => true,
        _ => false,
    });
    outIsCref
}

pub fn isUnaryCref(mut inExp: &metamodelica::Ref<DAE::Exp>) -> bool {
    let mut outIsCref: bool;
    outIsCref = (::match_deref::match_deref! { match inExp {
        Deref @ DAE::Exp::UNARY { operator: DAE::Operator::UMINUS { .. }, exp: Deref @ DAE::Exp::CREF { .. } } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outIsCref
}

pub fn isCall(mut inExp: &metamodelica::Ref<DAE::Exp>) -> bool {
    let mut outIsCall: bool;
    outIsCall = (match &**inExp {
        DAE::Exp::CALL { .. } => true,
        _ => false,
    });
    outIsCall
}

pub fn isTSUB(mut inExp: &metamodelica::Ref<DAE::Exp>) -> bool {
    let mut outIsCall: bool;
    outIsCall = (match &**inExp {
        DAE::Exp::TSUB { .. } => true,
        _ => false,
    });
    outIsCall
}

pub(crate) fn isPureCall(mut inExp: metamodelica::Ref<DAE::Exp>) -> Result<bool> {
    let mut outIsPureCall: bool;
    outIsPureCall = isCall(&inExp) && !(isImpure(inExp)?);
    Ok(outIsPureCall)
}

pub fn isImpureCall(mut inExp: metamodelica::Ref<DAE::Exp>) -> Result<bool> {
    let mut outIsPureCall: bool;
    outIsPureCall = isCall(&inExp) && isImpure(inExp)?;
    Ok(outIsPureCall)
}

pub fn isRecordCall(
    mut inExp: &metamodelica::Ref<DAE::Exp>,
    mut funcsIn: &metamodelica::Ref<AvlTreePathFunction::Tree>,
) -> Result<bool> {
    let mut outIsCall: bool;
    outIsCall = (match &**inExp {
        DAE::Exp::CALL { path, .. } => {
            let mut func: DAE::Function;
            let __pa0 = ::match_deref::match_deref! { match &(AvlTreePathFunction::get(funcsIn, path.clone())?) {
                Some(__pa0) => __pa0.clone(),
                _ => return Err("pattern mismatch"),
            } };
            func = metamodelica::Own::own(__pa0);
            (DAEUtil::getFunctionElements(&func)?).is_empty()
        }
        _ => false,
    });
    Ok(outIsCall)
}

pub(crate) fn isNotCref(mut inExp: &metamodelica::Ref<DAE::Exp>) -> bool {
    let mut outIsCref: bool;
    outIsCref = (match &**inExp {
        DAE::Exp::CREF { .. } => false,
        _ => true,
    });
    outIsCref
}

pub(crate) fn isCrefArray(mut inExp: &metamodelica::Ref<DAE::Exp>) -> bool {
    let mut outIsArray: bool;
    outIsArray = (::match_deref::match_deref! { match inExp {
        Deref @ DAE::Exp::CREF { ty: Deref @ DAE::Type::T_ARRAY { .. }, .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outIsArray
}

pub(crate) fn isCrefScalar(mut inExp: &metamodelica::Ref<DAE::Exp>) -> bool {
    let mut isScalar: bool;
    isScalar = 'mc: {
        let __mc_input = &**inExp;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::CREF { ty: Deref @ DAE::Type::T_ARRAY { .. }, .. } => {
                    let mut cr: ComponentRef;
                    let mut b: bool;
                    cr = expCref(inExp)?;
                    b = ComponentReference::crefHasScalarSubscripts(&cr);
                    Ok(b)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::CREF { .. } => {
                    Ok(true)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(false)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    isScalar
}

pub fn isTuple(mut inExp: &metamodelica::Ref<DAE::Exp>) -> bool {
    let mut outIsTuple: bool;
    outIsTuple = (match &**inExp {
        DAE::Exp::TUPLE { .. } => true,
        _ => false,
    });
    outIsTuple
}

pub fn isRecord(mut inExp: &metamodelica::Ref<DAE::Exp>) -> bool {
    let mut outIsRecord: bool;
    outIsRecord = (match &**inExp {
        DAE::Exp::RECORD { .. } => true,
        _ => false,
    });
    outIsRecord
}

pub fn isScalarConst(mut inExp: &metamodelica::Ref<DAE::Exp>) -> bool {
    let mut outIsScalar: bool;
    outIsScalar = (match &**inExp {
        DAE::Exp::ICONST { .. } => true,
        DAE::Exp::RCONST { .. } => true,
        DAE::Exp::SCONST { .. } => true,
        DAE::Exp::BCONST { .. } => true,
        DAE::Exp::ENUM_LITERAL { .. } => true,
        _ => false,
    });
    outIsScalar
}

pub(crate) fn isEven<'__b>(mut e: &'__b metamodelica::Ref<DAE::Exp>) -> bool {
    '__tco: loop {
        match &**e {
            DAE::Exp::ICONST { integer: i } => return intMod(i.clone(), 2) == 0,
            DAE::Exp::RCONST { real: r } => {
                return realMod(r.clone(), metamodelica::OrderedFloat(2.0_f64)) == metamodelica::OrderedFloat(0.0_f64);
            }
            DAE::Exp::CAST { exp, .. } => {
                e = exp;
                continue '__tco;
            }
            _ => return false,
        }
    }
}

pub(crate) fn isOdd<'__b>(mut e: &'__b metamodelica::Ref<DAE::Exp>) -> bool {
    '__tco: loop {
        match &**e {
            DAE::Exp::ICONST { integer: i } => return intMod(i.clone(), 2) == 1,
            DAE::Exp::RCONST { real: r } => {
                return realMod(r.clone(), metamodelica::OrderedFloat(2.0_f64)) == metamodelica::OrderedFloat(1.0_f64);
            }
            DAE::Exp::CAST { exp, .. } => {
                e = exp;
                continue '__tco;
            }
            _ => return false,
        }
    }
}

pub(crate) fn isIntegerOrReal(mut tp: &metamodelica::Ref<DAE::Type>) -> bool {
    let mut res: bool;
    res = (match &**tp {
        DAE::Type::T_REAL { .. } => true,
        DAE::Type::T_INTEGER { .. } => true,
        _ => false,
    });
    res
}

pub(crate) fn expStructuralEqual(
    mut inExp1: metamodelica::Ref<DAE::Exp>,
    mut inExp2: metamodelica::Ref<DAE::Exp>,
) -> Result<bool> {
    '__tco: loop {
        ::match_deref::match_deref! { match &((inExp1.clone(), inExp2.clone())) {
            (Deref @ DAE::Exp::ICONST { integer: i1 }, Deref @ DAE::Exp::ICONST { integer: i2 }) => {
                return Ok(i1.clone() == i2.clone())
            },
            (Deref @ DAE::Exp::UNARY { operator: DAE::Operator::UMINUS { ty: _ }, exp: Deref @ DAE::Exp::ICONST { integer: i1 } }, Deref @ DAE::Exp::ICONST { integer: i2 }) => {
                let mut i1 = (*i1).clone();
                i1 = -(i1.clone());
                return Ok(i1.clone() == i2.clone())
            },
            (Deref @ DAE::Exp::ICONST { integer: i1 }, Deref @ DAE::Exp::UNARY { operator: DAE::Operator::UMINUS { ty: _ }, exp: Deref @ DAE::Exp::ICONST { integer: i2 } }) => {
                let mut i2 = (*i2).clone();
                i2 = -(i2.clone());
                return Ok(i1.clone() == i2.clone())
            },
            (Deref @ DAE::Exp::RCONST { real: r1 }, Deref @ DAE::Exp::RCONST { real: r2 }) => {
                return Ok(r1.clone() == r2.clone())
            },
            (Deref @ DAE::Exp::UNARY { operator: DAE::Operator::UMINUS { ty: _ }, exp: Deref @ DAE::Exp::RCONST { real: r1 } }, Deref @ DAE::Exp::RCONST { real: r2 }) => {
                let mut r1 = (*r1).clone();
                r1 = -(r1.clone());
                return Ok(r1.clone() == r2.clone())
            },
            (Deref @ DAE::Exp::RCONST { real: r1 }, Deref @ DAE::Exp::UNARY { operator: DAE::Operator::UMINUS { ty: _ }, exp: Deref @ DAE::Exp::RCONST { real: r2 } }) => {
                let mut r2 = (*r2).clone();
                r2 = -(r2.clone());
                return Ok(r1.clone() == r2.clone())
            },
            (Deref @ DAE::Exp::SCONST { string: s1 }, Deref @ DAE::Exp::SCONST { string: s2 }) => {
                return Ok(stringEq(&s1, &s2))
            },
            (Deref @ DAE::Exp::BCONST { bool: b1 }, Deref @ DAE::Exp::BCONST { bool: b2 }) => {
                return Ok(boolEq(b1.clone(), b2.clone()))
            },
            (Deref @ DAE::Exp::ENUM_LITERAL { name: enum1, .. }, Deref @ DAE::Exp::ENUM_LITERAL { name: enum2, .. }) => {
                return Ok(AbsynUtil::pathEqual(metamodelica::AsArg::as_arg(&enum1), metamodelica::AsArg::as_arg(&enum2)))
            },
            (Deref @ DAE::Exp::CREF { .. }, Deref @ DAE::Exp::CREF { .. }) => {
                return Ok(true)
            },
            (Deref @ DAE::Exp::BINARY { exp1: e11, operator: op1, exp2: e12 }, Deref @ DAE::Exp::BINARY { exp1: e21, operator: op2, exp2: e22 }) => {
                let mut b: bool;
                b = operatorEqual(metamodelica::AsArg::as_arg(&op1), metamodelica::AsArg::as_arg(&op2))?;
                b = if (b) {expStructuralEqual(e11.clone(), e21.clone())?} else {b};
                if (b) {{ (inExp1, inExp2) = (e12.clone(), e22.clone()); continue '__tco; }} else {return Ok(b)}
            },
            (Deref @ DAE::Exp::LBINARY { exp1: e11, operator: op1, exp2: e12 }, Deref @ DAE::Exp::LBINARY { exp1: e21, operator: op2, exp2: e22 }) => {
                let mut b: bool;
                b = operatorEqual(metamodelica::AsArg::as_arg(&op1), metamodelica::AsArg::as_arg(&op2))?;
                b = if (b) {expStructuralEqual(e11.clone(), e21.clone())?} else {b};
                if (b) {{ (inExp1, inExp2) = (e12.clone(), e22.clone()); continue '__tco; }} else {return Ok(b)}
            },
            (Deref @ DAE::Exp::UNARY { operator: op1, exp: e1 }, Deref @ DAE::Exp::UNARY { operator: op2, exp: e2 }) => {
                let mut b: bool;
                b = operatorEqual(metamodelica::AsArg::as_arg(&op1), metamodelica::AsArg::as_arg(&op2))?;
                if (b) {{ (inExp1, inExp2) = (e1.clone(), e2.clone()); continue '__tco; }} else {return Ok(b)}
            },
            (Deref @ DAE::Exp::LUNARY { operator: op1, exp: e1 }, Deref @ DAE::Exp::LUNARY { operator: op2, exp: e2 }) => {
                let mut b: bool;
                b = operatorEqual(metamodelica::AsArg::as_arg(&op1), metamodelica::AsArg::as_arg(&op2))?;
                if (b) {{ (inExp1, inExp2) = (e1.clone(), e2.clone()); continue '__tco; }} else {return Ok(b)}
            },
            (Deref @ DAE::Exp::RELATION { exp1: e11, operator: op1, exp2: e12, .. }, Deref @ DAE::Exp::RELATION { exp1: e21, operator: op2, exp2: e22, .. }) => {
                let mut b: bool;
                b = operatorEqual(metamodelica::AsArg::as_arg(&op1), metamodelica::AsArg::as_arg(&op2))?;
                b = if (b) {expStructuralEqual(e11.clone(), e21.clone())?} else {b};
                if (b) {{ (inExp1, inExp2) = (e12.clone(), e22.clone()); continue '__tco; }} else {return Ok(b)}
            },
            (Deref @ DAE::Exp::IFEXP { expCond: e11, expThen: e12, expElse: e13 }, Deref @ DAE::Exp::IFEXP { expCond: e21, expThen: e22, expElse: e23 }) => {
                let mut b: bool;
                b = expStructuralEqual(e11.clone(), e21.clone())?;
                b = if (b) {expStructuralEqual(e12.clone(), e22.clone())?} else {b};
                if (b) {{ (inExp1, inExp2) = (e13.clone(), e23.clone()); continue '__tco; }} else {return Ok(b)}
            },
            (Deref @ DAE::Exp::CALL { path: path1, expLst: expl1, .. }, Deref @ DAE::Exp::CALL { path: path2, expLst: expl2, .. }) => {
                let mut b: bool;
                b = AbsynUtil::pathEqual(metamodelica::AsArg::as_arg(&path1), metamodelica::AsArg::as_arg(&path2));
                if (b) {return Ok(expStructuralEqualList(metamodelica::AsArg::as_arg(&expl1), metamodelica::AsArg::as_arg(&expl2))?)} else {return Ok(b)}
            },
            (Deref @ DAE::Exp::RECORD { path: path1, exps: expl1, .. }, Deref @ DAE::Exp::RECORD { path: path2, exps: expl2, .. }) => {
                let mut b: bool;
                b = AbsynUtil::pathEqual(metamodelica::AsArg::as_arg(&path1), metamodelica::AsArg::as_arg(&path2));
                if (b) {return Ok(expStructuralEqualList(metamodelica::AsArg::as_arg(&expl1), metamodelica::AsArg::as_arg(&expl2))?)} else {return Ok(b)}
            },
            (Deref @ DAE::Exp::PARTEVALFUNCTION { path: path1, expList: expl1, .. }, Deref @ DAE::Exp::PARTEVALFUNCTION { path: path2, expList: expl2, .. }) => {
                let mut b: bool;
                b = AbsynUtil::pathEqual(metamodelica::AsArg::as_arg(&path1), metamodelica::AsArg::as_arg(&path2));
                if (b) {return Ok(expStructuralEqualList(metamodelica::AsArg::as_arg(&expl1), metamodelica::AsArg::as_arg(&expl2))?)} else {return Ok(b)}
            },
            (Deref @ DAE::Exp::ARRAY { ty: tp1, array: expl1, .. }, Deref @ DAE::Exp::ARRAY { ty: tp2, array: expl2, .. }) => {
                let mut b: bool;
                b = tp1.clone() == tp2.clone();
                if (b) {return Ok(expStructuralEqualList(metamodelica::AsArg::as_arg(&expl1), metamodelica::AsArg::as_arg(&expl2))?)} else {return Ok(b)}
            },
            (Deref @ DAE::Exp::MATRIX { matrix: explstlst1, .. }, Deref @ DAE::Exp::MATRIX { matrix: explstlst2, .. }) => {
                return Ok(expStructuralEqualListLst(metamodelica::AsArg::as_arg(&explstlst1), metamodelica::AsArg::as_arg(&explstlst2))?)
            },
            (Deref @ DAE::Exp::RANGE { start: e11, step: None, stop: e13, .. }, Deref @ DAE::Exp::RANGE { start: e21, step: None, stop: e23, .. }) => {
                let mut b: bool;
                b = expStructuralEqual(e11.clone(), e21.clone())?;
                if (b) {{ (inExp1, inExp2) = (e13.clone(), e23.clone()); continue '__tco; }} else {return Ok(b)}
            },
            (Deref @ DAE::Exp::RANGE { start: e11, step: Some(e12), stop: e13, .. }, Deref @ DAE::Exp::RANGE { start: e21, step: Some(e22), stop: e23, .. }) => {
                let mut b: bool;
                b = expStructuralEqual(e11.clone(), e21.clone())?;
                b = if (b) {expStructuralEqual(e12.clone(), e22.clone())?} else {b};
                if (b) {{ (inExp1, inExp2) = (e13.clone(), e23.clone()); continue '__tco; }} else {return Ok(b)}
            },
            (Deref @ DAE::Exp::TUPLE { PR: expl1 }, Deref @ DAE::Exp::TUPLE { PR: expl2 }) => {
                return Ok(expStructuralEqualList(metamodelica::AsArg::as_arg(&expl1), metamodelica::AsArg::as_arg(&expl2))?)
            },
            (Deref @ DAE::Exp::CAST { ty: tp1, exp: e1 }, Deref @ DAE::Exp::CAST { ty: tp2, exp: e2 }) => {
                let mut b: bool;
                b = tp1.clone() == tp2.clone();
                if (b) {{ (inExp1, inExp2) = (e1.clone(), e2.clone()); continue '__tco; }} else {return Ok(b)}
            },
            (Deref @ DAE::Exp::ASUB { exp: e1, sub: subs1 }, Deref @ DAE::Exp::ASUB { sub: subs2, .. }) => {
                let mut b: bool;
                let mut ae1: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                let mut ae2: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                ae1 = ({
            let mut __acc: metamodelica::List<metamodelica::Ref<DAE::Exp>> = metamodelica::nil();
            for mut sub in (subs1.clone()).into_iter().cloned() {
                let __x = getSubscriptExp(&(sub.clone()))?;
                __acc = cons(__x, __acc);
            }
            __acc.reverse()
        });
                ae2 = ({
            let mut __acc: metamodelica::List<metamodelica::Ref<DAE::Exp>> = metamodelica::nil();
            for mut sub in (subs2.clone()).into_iter().cloned() {
                let __x = getSubscriptExp(&(sub.clone()))?;
                __acc = cons(__x, __acc);
            }
            __acc.reverse()
        });
                b = expStructuralEqual(e1.clone(), e1.clone())?;
                if (b) {return Ok(expStructuralEqualList(&ae1, &ae2)?)} else {return Ok(b)}
            },
            (Deref @ DAE::Exp::SIZE { exp: e1, sz: None }, Deref @ DAE::Exp::SIZE { exp: e2, sz: None }) => {
                { (inExp1, inExp2) = (e1.clone(), e2.clone()); continue '__tco; }
            },
            (Deref @ DAE::Exp::SIZE { exp: e1, sz: Some(e11) }, Deref @ DAE::Exp::SIZE { exp: e2, sz: Some(e22) }) => {
                let mut b: bool;
                b = expStructuralEqual(e1.clone(), e2.clone())?;
                if (b) {{ (inExp1, inExp2) = (e11.clone(), e22.clone()); continue '__tco; }} else {return Ok(b)}
            },
            (Deref @ DAE::Exp::CODE { .. }, Deref @ DAE::Exp::CODE { .. }) => {
                Debug::trace(literal!("exp_equal on CODE not impl.\n"))?;
                return Ok(false)
            },
            (Deref @ DAE::Exp::REDUCTION { .. }, Deref @ DAE::Exp::REDUCTION { .. }) => {
                let mut res: bool;
                return Ok(inExp1 == inExp2)
            },
            (Deref @ DAE::Exp::LIST { valList: expl1 }, Deref @ DAE::Exp::LIST { valList: expl2 }) => {
                return Ok(expStructuralEqualList(metamodelica::AsArg::as_arg(&expl1), metamodelica::AsArg::as_arg(&expl2))?)
            },
            (Deref @ DAE::Exp::CONS { car: e11, cdr: e12 }, Deref @ DAE::Exp::CONS { car: e21, cdr: e22 }) => {
                let mut b: bool;
                b = expStructuralEqual(e11.clone(), e21.clone())?;
                if (b) {{ (inExp1, inExp2) = (e12.clone(), e22.clone()); continue '__tco; }} else {return Ok(b)}
            },
            (Deref @ DAE::Exp::META_TUPLE { listExp: expl1 }, Deref @ DAE::Exp::META_TUPLE { listExp: expl2 }) => {
                return Ok(expStructuralEqualList(metamodelica::AsArg::as_arg(&expl1), metamodelica::AsArg::as_arg(&expl2))?)
            },
            (Deref @ DAE::Exp::META_OPTION { exp: None }, Deref @ DAE::Exp::META_OPTION { exp: None }) => {
                return Ok(true)
            },
            (Deref @ DAE::Exp::META_OPTION { exp: Some(e1) }, Deref @ DAE::Exp::META_OPTION { exp: Some(e2) }) => {
                { (inExp1, inExp2) = (e1.clone(), e2.clone()); continue '__tco; }
            },
            (Deref @ DAE::Exp::METARECORDCALL { path: path1, args: expl1, .. }, Deref @ DAE::Exp::METARECORDCALL { path: path2, args: expl2, .. }) => {
                let mut b: bool;
                b = AbsynUtil::pathEqual(metamodelica::AsArg::as_arg(&path1), metamodelica::AsArg::as_arg(&path2));
                if (b) {return Ok(expStructuralEqualList(metamodelica::AsArg::as_arg(&expl1), metamodelica::AsArg::as_arg(&expl2))?)} else {return Ok(b)}
            },
            (e1 @ Deref @ DAE::Exp::MATCHEXPRESSION { .. }, e2 @ Deref @ DAE::Exp::MATCHEXPRESSION { .. }) => {
                return Ok(e1.clone() == e2.clone())
            },
            (Deref @ DAE::Exp::BOX { exp: e1 }, Deref @ DAE::Exp::BOX { exp: e2 }) => {
                { (inExp1, inExp2) = (e1.clone(), e2.clone()); continue '__tco; }
            },
            (Deref @ DAE::Exp::UNBOX { exp: e1, .. }, Deref @ DAE::Exp::UNBOX { exp: e2, .. }) => {
                { (inExp1, inExp2) = (e1.clone(), e2.clone()); continue '__tco; }
            },
            (Deref @ DAE::Exp::SHARED_LITERAL { index: i1, .. }, Deref @ DAE::Exp::SHARED_LITERAL { index: i2, .. }) => {
                return Ok(intEq(i1.clone(), i2.clone()))
            },
            _ => {
                return Ok(false)
            },
            _ => return Err("match: no arm matched"),
        } }
    }
}

pub(crate) fn expStructuralEqualList<'__b>(
    mut inExp1: &'__b metamodelica::List<metamodelica::Ref<DAE::Exp>>,
    mut inExp2: &'__b metamodelica::List<metamodelica::Ref<DAE::Exp>>,
) -> Result<bool> {
    '__tco: loop {
        ::match_deref::match_deref! { match (inExp1, inExp2) {
            (Deref @ metamodelica::ListNode::Nil, Deref @ metamodelica::ListNode::Nil) => {
                return Ok(true)
            },
            (Deref @ metamodelica::ListNode::Cons { head: e1, tail: es1 }, Deref @ metamodelica::ListNode::Cons { head: e2, tail: es2 }) if (expStructuralEqual(e1.clone(), e2.clone())?) => {
                { (inExp1, inExp2) = (es1, es2); continue '__tco; }
            },
            _ => {
                return Ok(false)
            },
            _ => return Err("match: no arm matched"),
        } }
    }
}

fn expStructuralEqualListLst<'__b>(
    mut inExp1: &'__b metamodelica::List<metamodelica::List<metamodelica::Ref<DAE::Exp>>>,
    mut inExp2: &'__b metamodelica::List<metamodelica::List<metamodelica::Ref<DAE::Exp>>>,
) -> Result<bool> {
    '__tco: loop {
        ::match_deref::match_deref! { match (inExp1, inExp2) {
            (Deref @ metamodelica::ListNode::Nil, Deref @ metamodelica::ListNode::Nil) => {
                return Ok(true)
            },
            (Deref @ metamodelica::ListNode::Cons { head: e1, tail: es1 }, Deref @ metamodelica::ListNode::Cons { head: e2, tail: es2 }) if (expStructuralEqualList(metamodelica::AsArg::as_arg(&e1), metamodelica::AsArg::as_arg(&e2))?) => {
                { (inExp1, inExp2) = (es1, es2); continue '__tco; }
            },
            _ => {
                return Ok(false)
            },
            _ => return Err("match: no arm matched"),
        } }
    }
}

pub(crate) fn expContainsList(
    mut expl: &metamodelica::List<metamodelica::Ref<DAE::Exp>>,
    mut exp: &metamodelica::Ref<DAE::Exp>,
) -> Result<bool> {
    let mut contains: bool = List::any(
        expl,
        &({
            let __pe_b1 = exp.clone();
            move |__pe_a0| expContains(&__pe_a0, &__pe_b1)
        }),
    )?;
    Ok(contains)
}

pub fn expContains(mut inExp1: &metamodelica::Ref<DAE::Exp>, mut inExp2: &metamodelica::Ref<DAE::Exp>) -> Result<bool> {
    let mut outBoolean: bool;
    outBoolean = 'mc: {
        let __mc_input = (&**inExp1, &**inExp2);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::ICONST { integer: i1 }, Deref @ DAE::Exp::ICONST { integer: i2 }) => {
                    Ok(i1.clone() == i2.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::ICONST { .. }, _) => {
                    Ok(false)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::RCONST { real: r1 }, Deref @ DAE::Exp::RCONST { real: r2 }) => {
                    Ok(r1.clone() == r2.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::RCONST { .. }, _) => {
                    Ok(false)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::SCONST { string: s1 }, Deref @ DAE::Exp::SCONST { string: s2 }) => {
                    Ok(metamodelica::stringEq(&s1, &s2))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::SCONST { .. }, _) => {
                    Ok(false)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::BCONST { bool: b1 }, Deref @ DAE::Exp::BCONST { bool: b2 }) => {
                    Ok(b1.clone() == b2.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::BCONST { .. }, _) => {
                    Ok(true)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::ENUM_LITERAL { .. }, _) => {
                    Ok(false)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::ARRAY { array: expLst, .. }, _) => {
                    Ok(expContainsList(metamodelica::AsArg::as_arg(&expLst), inExp2)?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::MATRIX { matrix: expl, .. }, _) => {
                    Ok(List::any(metamodelica::AsArg::as_arg(&expl), &({ let __pe_b1: Arc<dyn ::std::ops::Fn(_) -> Result<bool> + 'static> = (std::sync::Arc::new({ let __pe_b1 = inExp2.clone(); move |__pe_a0| expContains(&__pe_a0, &__pe_b1) }) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<bool> + 'static>); move |__pe_a0| List::any(&__pe_a0, &*__pe_b1) }))?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CREF { componentRef: cr1, .. }, Deref @ DAE::Exp::CREF { componentRef: cr2, .. }) => {
                    let mut res: bool;
                    let mut expLst: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                    res = ComponentReferenceBasics::crefEqual(metamodelica::AsArg::as_arg(&cr1), metamodelica::AsArg::as_arg(&cr2))?;
                    if !(res) {
                        expLst = List::map(ComponentReferenceBasics::crefSubs(metamodelica::AsArg::as_arg(&cr1))?, &move |__a0: metamodelica::Ref<DAE::Subscript>| getSubscriptExp(&__a0))?;
                        res = expContainsList(&expLst, inExp2)?;
                    }
                    Ok(res)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CREF { .. }, _) => {
                    Ok(false)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::BINARY { exp1: e1, exp2: e2, .. }, _) => {
                    Ok(expContains(metamodelica::AsArg::as_arg(&e1), inExp2)? || expContains(metamodelica::AsArg::as_arg(&e2), inExp2)?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::UNARY { exp: e, .. }, _) => {
                    Ok(expContains(metamodelica::AsArg::as_arg(&e), inExp2)?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::LBINARY { exp1: e1, exp2: e2, .. }, _) => {
                    Ok(expContains(metamodelica::AsArg::as_arg(&e1), inExp2)? || expContains(metamodelica::AsArg::as_arg(&e2), inExp2)?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::LUNARY { exp: e, .. }, _) => {
                    Ok(expContains(metamodelica::AsArg::as_arg(&e), inExp2)?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::RELATION { exp1: e1, exp2: e2, .. }, _) => {
                    Ok(expContains(metamodelica::AsArg::as_arg(&e1), inExp2)? || expContains(metamodelica::AsArg::as_arg(&e2), inExp2)?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::IFEXP { expCond: c, expThen: t, expElse: f }, _) => {
                    Ok(expContains(metamodelica::AsArg::as_arg(&c), inExp2)? || expContains(metamodelica::AsArg::as_arg(&t), inExp2)? || expContains(metamodelica::AsArg::as_arg(&f), inExp2)?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "der" }, expLst: Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Exp::CREF { componentRef: cr1, .. }, tail: Deref @ metamodelica::ListNode::Nil }, .. }, Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "der" }, expLst: Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Exp::CREF { componentRef: cr2, .. }, tail: Deref @ metamodelica::ListNode::Nil }, .. }) => {
                    let mut res: bool;
                    res = ComponentReferenceBasics::crefEqual(metamodelica::AsArg::as_arg(&cr1), metamodelica::AsArg::as_arg(&cr2))?;
                    Ok(res)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "pre" }, .. }, _) => {
                    Ok(false)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "previous" }, .. }, _) => {
                    Ok(false)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::ListNode::Nil, .. }, _) => {
                    Ok(false)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CALL { expLst, .. }, _) => {
                    Ok(expContainsList(metamodelica::AsArg::as_arg(&expLst), inExp2)?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::RECORD { exps: expLst, .. }, _) => {
                    Ok(expContainsList(metamodelica::AsArg::as_arg(&expLst), inExp2)?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::PARTEVALFUNCTION { expList: expLst, .. }, Deref @ DAE::Exp::CREF { .. }) => {
                    Ok(expContainsList(metamodelica::AsArg::as_arg(&expLst), inExp2)?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CAST { ty: Deref @ DAE::Type::T_REAL { .. }, exp: Deref @ DAE::Exp::ICONST { .. } }, _) => {
                    Ok(false)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CAST { ty: Deref @ DAE::Type::T_REAL { .. }, exp: e }, _) => {
                    Ok(expContains(metamodelica::AsArg::as_arg(&e), inExp2)?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                        (Deref @ DAE::Exp::ASUB { exp: e, sub: subs }, _) => {
                            Ok(expContainsList(&(({
                let mut __acc: metamodelica::List<metamodelica::Ref<DAE::Exp>> = metamodelica::nil();
                for mut sub in (subs.clone()).into_iter().cloned() {
                    let __x = getSubscriptExp(&(sub.clone()))?;
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            })), inExp2)? || expContains(metamodelica::AsArg::as_arg(&e), inExp2)?)
                        }
                        _ => return Err("nomatch"),
                    }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::REDUCTION { expr: e, .. }, _) => {
                    Ok(expContains(metamodelica::AsArg::as_arg(&e), inExp2)?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut r#str: ArcStr;
                    let mut s1: ArcStr;
                    let mut s2: ArcStr;
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { return Err("pattern mismatch") };
                    Debug::trace(literal!("- Expression.expContains failed\n"))?;
                    s1 = printExpStr(inExp1.clone())?;
                    s2 = printExpStr(inExp2.clone())?;
                    r#str = stringAppendList(list![literal!("exp = "), s1.clone(), literal!(" subexp = "), s2.clone()]);
                    Debug::traceln(r#str.clone())?;
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(outBoolean)
}

pub(crate) fn containsExp(
    mut inExp1: &metamodelica::Ref<DAE::Exp>,
    mut inExp2: &metamodelica::Ref<DAE::Exp>,
) -> Result<bool> {
    let mut outBoolean: bool;
    outBoolean = expContains(inExp2, inExp1)?;
    Ok(outBoolean)
}

pub fn isExpCref(mut e: &metamodelica::Ref<DAE::Exp>) -> bool {
    let mut res: bool;
    res = (match &**e {
        DAE::Exp::CREF { componentRef: _, ty: _ } => true,
        _ => false,
    });
    res
}

pub(crate) fn isExpCrefOrIfExp(mut e: &metamodelica::Ref<DAE::Exp>) -> bool {
    let mut res: bool;
    res = (match &**e {
        DAE::Exp::CREF { componentRef: _, ty: _ } => true,
        DAE::Exp::IFEXP {
            expCond: _,
            expThen: _,
            expElse: _,
        } => true,
        _ => false,
    });
    res
}

pub(crate) fn isExpIfExp(mut e: &metamodelica::Ref<DAE::Exp>) -> bool {
    let mut res: bool;
    res = (match &**e {
        DAE::Exp::IFEXP { .. } => true,
        _ => false,
    });
    res
}

pub fn operatorEqual(mut inOperator1: &DAE::Operator, mut inOperator2: &DAE::Operator) -> Result<bool> {
    let mut outBoolean: bool;
    outBoolean = 0 == ExpressionBasics::operatorCompare(inOperator1, inOperator2)?;
    Ok(outBoolean)
}

pub fn arrayContainZeroDimension<'__b>(
    mut inDimensions: &'__b metamodelica::List<metamodelica::Ref<DAE::Dimension>>,
) -> bool {
    '__tco: loop {
        ::match_deref::match_deref! { match inDimensions {
            Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Dimension::DIM_INTEGER { integer: 0 }, tail: _ } => {
                return true
            },
            Deref @ metamodelica::ListNode::Cons { head: _, tail: rest_dims } => {
                { inDimensions = rest_dims; continue '__tco; }
            },
            _ => {
                return false
            },
            _ => unreachable!("tail-call lowered match: no arm matched"),
        } }
    }
}

pub fn arrayContainWholeDimension<'__b>(
    mut inDim: &'__b metamodelica::List<metamodelica::Ref<DAE::Dimension>>,
) -> bool {
    '__tco: loop {
        ::match_deref::match_deref! { match inDim {
            Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Dimension::DIM_UNKNOWN { .. }, tail: _ } => {
                return true
            },
            Deref @ metamodelica::ListNode::Cons { head: _, tail: rest_dims } => {
                { inDim = rest_dims; continue '__tco; }
            },
            _ => {
                return false
            },
            _ => unreachable!("tail-call lowered match: no arm matched"),
        } }
    }
}

pub fn isArrayType(mut inType: &metamodelica::Ref<DAE::Type>) -> bool {
    let mut b: bool;
    b = (match &**inType {
        DAE::Type::T_ARRAY { .. } => true,
        _ => false,
    });
    b
}

pub fn isRecordType(mut inType: &metamodelica::Ref<DAE::Type>) -> bool {
    let mut b: bool;
    b = (match &**inType {
        DAE::Type::T_COMPLEX {
            complexClassType: ClassInf::State::RECORD { .. },
            ..
        } => true,
        _ => false,
    });
    b
}

pub(crate) fn isNotComplex<'__b>(mut e: &'__b metamodelica::Ref<DAE::Exp>) -> bool {
    '__tco: loop {
        match &**e {
            DAE::Exp::CALL { .. } => return false,
            DAE::Exp::RECORD { .. } => return false,
            DAE::Exp::ARRAY { .. } => return false,
            DAE::Exp::CAST { exp: e2, .. } => {
                let mut b2: bool;
                {
                    e = e2;
                    continue '__tco;
                }
            }
            _ => return true,
        }
    }
}

pub fn isRealType(mut inType: &metamodelica::Ref<DAE::Type>) -> bool {
    let mut b: bool;
    b = (match &**inType {
        DAE::Type::T_REAL { .. } => true,
        _ => false,
    });
    b
}

pub fn dimensionsEqual(
    mut dim1: &metamodelica::Ref<DAE::Dimension>,
    mut dim2: &metamodelica::Ref<DAE::Dimension>,
) -> Result<bool> {
    let mut res: bool;
    res = (::match_deref::match_deref! { match (dim1, dim2) {
        (Deref @ DAE::Dimension::DIM_UNKNOWN { .. }, _) => {
            true
        },
        (_, Deref @ DAE::Dimension::DIM_UNKNOWN { .. }) => {
            true
        },
        (Deref @ DAE::Dimension::DIM_EXP { .. }, _) => {
            true
        },
        (_, Deref @ DAE::Dimension::DIM_EXP { .. }) => {
            true
        },
        _ => {
            let mut b: bool;
            b = intEq(dimensionSize(dim1)?, dimensionSize(dim2)?);
            b
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(res)
}

pub fn dimsEqual<'__b>(
    mut dims1: &'__b metamodelica::List<metamodelica::Ref<DAE::Dimension>>,
    mut dims2: &'__b metamodelica::List<metamodelica::Ref<DAE::Dimension>>,
) -> Result<bool> {
    '__tco: loop {
        ::match_deref::match_deref! { match (dims1, dims2) {
            (Deref @ metamodelica::ListNode::Nil, Deref @ metamodelica::ListNode::Nil) => {
                return Ok(true)
            },
            (Deref @ metamodelica::ListNode::Cons { head: d1, tail: dl1 }, Deref @ metamodelica::ListNode::Cons { head: d2, tail: dl2 }) if (dimensionsEqual(metamodelica::AsArg::as_arg(&d1), metamodelica::AsArg::as_arg(&d2))?) => {
                { (dims1, dims2) = (dl1, dl2); continue '__tco; }
            },
            _ => {
                return Ok(false)
            },
            _ => return Err("match: no arm matched"),
        } }
    }
}

pub(crate) fn dimsEqualAllowZero<'__b>(
    mut dims1: &'__b metamodelica::List<metamodelica::Ref<DAE::Dimension>>,
    mut dims2: &'__b metamodelica::List<metamodelica::Ref<DAE::Dimension>>,
) -> Result<bool> {
    '__tco: loop {
        ::match_deref::match_deref! { match (dims1, dims2) {
            (Deref @ metamodelica::ListNode::Nil, Deref @ metamodelica::ListNode::Nil) => {
                return Ok(true)
            },
            (Deref @ metamodelica::ListNode::Cons { head: d1, tail: dl1 }, Deref @ metamodelica::ListNode::Cons { head: d2, tail: dl2 }) if (dimensionsEqualAllowZero(metamodelica::AsArg::as_arg(&d1), metamodelica::AsArg::as_arg(&d2))?) => {
                { (dims1, dims2) = (dl1, dl2); continue '__tco; }
            },
            _ => {
                return Ok(false)
            },
            _ => return Err("match: no arm matched"),
        } }
    }
}

pub(crate) fn dimensionsEqualAllowZero(
    mut dim1: &metamodelica::Ref<DAE::Dimension>,
    mut dim2: &metamodelica::Ref<DAE::Dimension>,
) -> Result<bool> {
    let mut res: bool;
    res = (::match_deref::match_deref! { match (dim1, dim2) {
        (Deref @ DAE::Dimension::DIM_UNKNOWN { .. }, _) => {
            true
        },
        (_, Deref @ DAE::Dimension::DIM_UNKNOWN { .. }) => {
            true
        },
        (Deref @ DAE::Dimension::DIM_EXP { .. }, _) => {
            true
        },
        (_, Deref @ DAE::Dimension::DIM_EXP { .. }) => {
            true
        },
        _ => {
            let mut b: bool;
            let mut d1: i32;
            let mut d2: i32;
            d1 = dimensionSize(dim1)?;
            d2 = dimensionSize(dim2)?;
            b = boolOr(intEq(d1, d2), boolOr(boolAnd(intEq(d1, 0), intNe(d2, 0)), boolAnd(intEq(d2, 0), intNe(d1, 0))));
            b
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(res)
}

pub fn dimensionsKnownAndEqual(
    mut dim1: &metamodelica::Ref<DAE::Dimension>,
    mut dim2: &metamodelica::Ref<DAE::Dimension>,
) -> Result<bool> {
    let mut res: bool;
    res = (::match_deref::match_deref! { match (dim1, dim2) {
        (Deref @ DAE::Dimension::DIM_UNKNOWN { .. }, _) => false,
        (_, Deref @ DAE::Dimension::DIM_UNKNOWN { .. }) => false,
        _ => ExpressionBasics::expEqual(&(dimensionSizeExp(dim1)?), dimensionSizeExp(dim2)?)?,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(res)
}

pub fn dimensionKnown(mut dim: &metamodelica::Ref<DAE::Dimension>) -> bool {
    let mut known: bool;
    known = (::match_deref::match_deref! { match dim {
        Deref @ DAE::Dimension::DIM_UNKNOWN { .. } => false,
        Deref @ DAE::Dimension::DIM_EXP { exp: Deref @ DAE::Exp::ICONST { .. } } => true,
        Deref @ DAE::Dimension::DIM_EXP { exp: Deref @ DAE::Exp::BCONST { .. } } => true,
        Deref @ DAE::Dimension::DIM_EXP { exp: Deref @ DAE::Exp::ENUM_LITERAL { .. } } => true,
        Deref @ DAE::Dimension::DIM_EXP { .. } => false,
        _ => true,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    known
}

pub(crate) fn dimensionKnownAndNonZero(mut dim: &metamodelica::Ref<DAE::Dimension>) -> bool {
    let mut known: bool;
    known = (::match_deref::match_deref! { match dim {
        Deref @ DAE::Dimension::DIM_EXP { exp: Deref @ DAE::Exp::ICONST { integer: 0 } } => false,
        Deref @ DAE::Dimension::DIM_INTEGER { integer: 0 } => false,
        _ => dimensionKnown(dim),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    known
}

pub fn dimensionsKnownAndNonZero(mut dims: &metamodelica::List<metamodelica::Ref<DAE::Dimension>>) -> Result<bool> {
    let mut allKnown: bool;
    allKnown = List::all(
        dims,
        &move |__a0: metamodelica::Ref<DAE::Dimension>| -> metamodelica::Result<_> {
            ::std::result::Result::Ok(dimensionKnownAndNonZero(&__a0))
        },
    )?;
    Ok(allKnown)
}

pub fn dimensionUnknownOrExp(mut dim: &metamodelica::Ref<DAE::Dimension>) -> bool {
    let mut known: bool;
    known = (match &**dim {
        DAE::Dimension::DIM_UNKNOWN { .. } => true,
        DAE::Dimension::DIM_EXP { .. } => true,
        _ => false,
    });
    known
}

pub(crate) fn dimensionUnknown(mut inDimension: &metamodelica::Ref<DAE::Dimension>) -> bool {
    let mut outUnknown: bool;
    outUnknown = (match &**inDimension {
        DAE::Dimension::DIM_UNKNOWN { .. } => true,
        _ => false,
    });
    outUnknown
}

pub fn hasUnknownDims(mut dims: &metamodelica::List<metamodelica::Ref<DAE::Dimension>>) -> Result<bool> {
    let mut hasUnkown: bool;
    hasUnkown = List::any(
        dims,
        &move |__a0: metamodelica::Ref<DAE::Dimension>| -> metamodelica::Result<_> {
            ::std::result::Result::Ok(dimensionUnknown(&__a0))
        },
    )?;
    Ok(hasUnkown)
}

pub(crate) fn subscriptConstant(mut sub: &metamodelica::Ref<DAE::Subscript>) -> bool {
    let mut b: bool;
    b = (::match_deref::match_deref! { match sub {
        Deref @ DAE::Subscript::INDEX { exp: Deref @ DAE::Exp::ICONST { .. } } => true,
        Deref @ DAE::Subscript::INDEX { exp: Deref @ DAE::Exp::ENUM_LITERAL { .. } } => true,
        Deref @ DAE::Subscript::INDEX { exp: Deref @ DAE::Exp::BCONST { .. } } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    b
}

pub fn subscriptConstants(mut inSubs: &metamodelica::List<metamodelica::Ref<DAE::Subscript>>) -> bool {
    let mut areConstant: bool = true;
    for mut sub in &**inSubs {
        areConstant = subscriptConstant(metamodelica::AsArg::as_arg(&sub));
        if !(areConstant) {
            return areConstant;
        }
    }
    areConstant
}

pub fn isValidSubscript(mut inSub: &metamodelica::Ref<DAE::Exp>) -> bool {
    let mut isValid: bool;
    isValid = (match &**inSub {
        DAE::Exp::ICONST { .. } => true,
        DAE::Exp::ENUM_LITERAL { .. } => true,
        DAE::Exp::BCONST { .. } => true,
        _ => false,
    });
    isValid
}

pub(crate) fn subscriptContain<'__b>(
    mut issl1: &'__b metamodelica::List<metamodelica::Ref<DAE::Subscript>>,
    mut issl2: &'__b metamodelica::List<metamodelica::Ref<DAE::Subscript>>,
) -> Result<bool> {
    '__tco: loop {
        ::match_deref::match_deref! { match (issl1, issl2) {
            (Deref @ metamodelica::ListNode::Nil, _) => {
                return Ok(true)
            },
            (Deref @ metamodelica::ListNode::Cons { head: _, tail: ssl1 }, Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Subscript::WHOLEDIM { .. }, tail: ssl2 }) => {
                let mut b: bool;
                { (issl1, issl2) = (ssl1, ssl2); continue '__tco; }
            },
            (Deref @ metamodelica::ListNode::Cons { head: _, tail: ssl1 }, Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Subscript::WHOLE_NONEXP { exp: _ }, tail: ssl2 }) => {
                let mut b: bool;
                { (issl1, issl2) = (ssl1, ssl2); continue '__tco; }
            },
            (Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Subscript::INDEX { exp: Deref @ DAE::Exp::ICONST { integer: i } }, tail: ssl1 }, Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Subscript::SLICE { exp: Deref @ DAE::Exp::ARRAY { ty: _, scalar: _, array: expl } }, tail: ssl2 }) => {
                let mut b: bool;
                let true = (subscriptContain2(i.clone(), expl.clone())) else { return Err("pattern mismatch") };
                { (issl1, issl2) = (ssl1, ssl2); continue '__tco; }
            },
            (Deref @ metamodelica::ListNode::Cons { head: ss1, tail: ssl1 }, Deref @ metamodelica::ListNode::Cons { head: ss2, tail: ssl2 }) => {
                let mut b: bool;
                let true = (ExpressionBasics::subscriptEqual(&(list![ss1.clone()]), &(list![ss2.clone()]))?) else { return Err("pattern mismatch") };
                { (issl1, issl2) = (ssl1, ssl2); continue '__tco; }
            },
            _ => {
                return Ok(false)
            },
            _ => return Err("match: no arm matched"),
        } }
    }
}

fn subscriptContain2(mut inInt: i32, mut inExp2: metamodelica::List<metamodelica::Ref<DAE::Exp>>) -> bool {
    '__tco: loop {
        ::match_deref::match_deref! { match &((inInt, inExp2)) {
            (i, Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Exp::ICONST { integer: j }, tail: _ }) if (i.clone() == j.clone()) => {
                return true
            },
            (i, Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Exp::ICONST { integer: _ }, tail: expl }) if (subscriptContain2(i.clone(), expl.clone())) => {
                return true
            },
            (i, Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Exp::ARRAY { ty: _, scalar: _, array: expl2 }, tail: expl }) => {
                let mut b: bool;
                let mut b2: bool;
                b = subscriptContain2(i.clone(), expl2.clone());
                if (b) {return true} else {{ (inInt, inExp2) = (i.clone(), expl.clone()); continue '__tco; }}
            },
            _ => {
                return false
            },
            _ => unreachable!("tail-call lowered match: no arm matched"),
        } }
    }
}

pub fn hasNoSideEffects(mut inExp: metamodelica::Ref<DAE::Exp>, mut ib: bool) -> (metamodelica::Ref<DAE::Exp>, bool) {
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    let mut ob: bool;
    (outExp, ob) = (match &*inExp {
        DAE::Exp::CALL { .. } => (inExp, false),
        DAE::Exp::MATCHEXPRESSION { .. } => (inExp, false),
        _ => (inExp, ib),
    });
    (outExp, ob)
}

pub(crate) fn isBuiltinFunctionReference(mut exp: &metamodelica::Ref<DAE::Exp>) -> bool {
    let mut b: bool;
    b = (::match_deref::match_deref! { match exp {
        Deref @ DAE::Exp::CREF { ty: Deref @ DAE::Type::T_FUNCTION_REFERENCE_FUNC { builtin: true, .. }, .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    b
}

pub(crate) fn makeCons(
    mut car: metamodelica::Ref<DAE::Exp>,
    mut cdr: metamodelica::Ref<DAE::Exp>,
) -> metamodelica::Ref<DAE::Exp> {
    let mut exp: metamodelica::Ref<DAE::Exp>;
    exp = metamodelica::Ref::new(DAE::Exp::CONS { car: car, cdr: cdr });
    exp
}

pub fn makeBuiltinCall(
    mut name: ArcStr,
    mut args: metamodelica::List<metamodelica::Ref<DAE::Exp>>,
    mut result_type: metamodelica::Ref<DAE::Type>,
    mut isImpure: bool,
) -> metamodelica::Ref<DAE::Exp> {
    let mut call: metamodelica::Ref<DAE::Exp>;
    call = metamodelica::Ref::new(DAE::Exp::CALL {
        path: metamodelica::Ref::new(Absyn::Path::IDENT { name: name }),
        expLst: args,
        attr: metamodelica::Ref::new(DAE::CallAttributes {
            ty: result_type,
            tuple_: false,
            builtin: true,
            isImpure: isImpure,
            isFunctionPointerCall: false,
            inlineType: openmodelica_frontend_types::DAE::InlineType::NO_INLINE,
            tailCall: openmodelica_frontend_types::DAE::TailCall::NO_TAIL,
            noReturn: DAE::NoReturn::RETURNS.clone(),
        }),
    });
    call
}

pub fn makePureBuiltinCall(
    mut name: ArcStr,
    mut args: metamodelica::List<metamodelica::Ref<DAE::Exp>>,
    mut result_type: metamodelica::Ref<DAE::Type>,
) -> metamodelica::Ref<DAE::Exp> {
    let mut call: metamodelica::Ref<DAE::Exp>;
    call = makeBuiltinCall(name, args, result_type, false);
    call
}

pub fn makeImpureBuiltinCall(
    mut name: ArcStr,
    mut args: metamodelica::List<metamodelica::Ref<DAE::Exp>>,
    mut result_type: metamodelica::Ref<DAE::Type>,
) -> metamodelica::Ref<DAE::Exp> {
    let mut call: metamodelica::Ref<DAE::Exp>;
    call = makeBuiltinCall(name, args, result_type, true);
    call
}

pub fn reductionIterName(mut iter: &metamodelica::Ref<DAE::ReductionIterator>) -> ArcStr {
    let mut name: ArcStr;
    let __arc1 = &(*iter);
    let DAE::REDUCTIONITER { id: __pa0, .. } = &**__arc1;
    name = metamodelica::Own::own(__pa0);
    name
}

fn traverseReductionIteratorBidir<ArgT: Clone + 'static + metamodelica::gc::MMTrace>(
    mut inIter: &metamodelica::Ref<DAE::ReductionIterator>,
    mut inEnterFunc: Arc<
        dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>, ArgT) -> Result<(metamodelica::Ref<DAE::Exp>, ArgT)> + 'static,
    >,
    mut inExitFunc: Arc<
        dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>, ArgT) -> Result<(metamodelica::Ref<DAE::Exp>, ArgT)> + 'static,
    >,
    mut inArg: ArgT,
) -> Result<(metamodelica::Ref<DAE::ReductionIterator>, ArgT)> {
    pub type FuncType<ArgT: Clone + 'static> = std::sync::Arc<
        dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>, ArgT) -> Result<(metamodelica::Ref<DAE::Exp>, ArgT)> + 'static,
    >;

    let mut outIter: metamodelica::Ref<DAE::ReductionIterator>;
    let mut outArg: ArgT;
    (outIter, outArg) = (match &**inIter {
        DAE::ReductionIterator {
            id,
            exp,
            guardExp: gexp,
            ty,
        } => {
            let mut arg: ArgT;
            let mut exp = (*exp).clone();
            let mut gexp = (*gexp).clone();
            (exp, arg) = traverseExpBidir(exp.clone(), inEnterFunc.clone(), inExitFunc.clone(), inArg)?;
            (gexp, arg) = traverseExpOptBidir(gexp.clone(), inEnterFunc.clone(), inExitFunc.clone(), arg)?;
            (
                metamodelica::Ref::new(DAE::ReductionIterator {
                    id: id.clone(),
                    exp: exp.clone(),
                    guardExp: gexp.clone(),
                    ty: ty.clone(),
                }),
                arg,
            )
        }
    });
    Ok((outIter, outArg))
}

fn traverseReductionIteratorTopDown<Type_a: Clone + 'static + metamodelica::gc::MMTrace>(
    mut iter: &metamodelica::Ref<DAE::ReductionIterator>,
    mut func: &dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>, Type_a) -> Result<(metamodelica::Ref<DAE::Exp>, bool, Type_a)>,
    mut inArg: Type_a,
) -> Result<(metamodelica::Ref<DAE::ReductionIterator>, Type_a)> {
    pub type FuncExpType<Type_a: Clone + 'static> = std::sync::Arc<
        dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>, Type_a) -> Result<(metamodelica::Ref<DAE::Exp>, bool, Type_a)>
            + 'static,
    >;

    let mut outIter: metamodelica::Ref<DAE::ReductionIterator>;
    let mut outArg: Type_a;
    (outIter, outArg) = (match &**iter {
        DAE::ReductionIterator {
            id,
            exp,
            guardExp: gexp,
            ty,
        } => {
            let mut arg = inArg;
            let mut exp = (*exp).clone();
            let mut gexp = (*gexp).clone();
            (exp, arg) = traverseExpTopDown(exp.clone(), func, arg)?;
            (gexp, arg) = traverseExpOptTopDown(gexp.clone(), func, arg)?;
            (
                metamodelica::Ref::new(DAE::ReductionIterator {
                    id: id.clone(),
                    exp: exp.clone(),
                    guardExp: gexp.clone(),
                    ty: ty.clone(),
                }),
                arg,
            )
        }
    });
    Ok((outIter, outArg))
}

fn traverseReductionIteratorsTopDown<Type_a: Clone + 'static + metamodelica::gc::MMTrace>(
    mut inIters: &metamodelica::List<metamodelica::Ref<DAE::ReductionIterator>>,
    mut func: &dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>, Type_a) -> Result<(metamodelica::Ref<DAE::Exp>, bool, Type_a)>,
    mut inArg: Type_a,
) -> Result<(metamodelica::List<metamodelica::Ref<DAE::ReductionIterator>>, Type_a)> {
    pub type FuncExpType<Type_a: Clone + 'static> = std::sync::Arc<
        dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>, Type_a) -> Result<(metamodelica::Ref<DAE::Exp>, bool, Type_a)>
            + 'static,
    >;

    let mut outIters: metamodelica::List<metamodelica::Ref<DAE::ReductionIterator>>;
    let mut outArg: Type_a;
    (outIters, outArg) = (::match_deref::match_deref! { match inIters {
        Deref @ metamodelica::ListNode::Nil => {
            let mut arg = inArg;
            (inIters.clone(), arg)
        },
        Deref @ metamodelica::ListNode::Cons { head: iter, tail: iters } => {
            let mut arg = inArg;
            let mut iter = (*iter).clone();
            let mut iters = (*iters).clone();
            (iter, arg) = traverseReductionIteratorTopDown(metamodelica::AsArg::as_arg(&iter), func, arg)?;
            (iters, arg) = traverseReductionIteratorsTopDown(metamodelica::AsArg::as_arg(&iters), func, arg)?;
            (metamodelica::cons(iter.clone(), iters.clone()), arg)
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outIters, outArg))
}

fn traverseReductionIterator<Type_a: Clone + 'static + metamodelica::gc::MMTrace>(
    mut iter: metamodelica::Ref<DAE::ReductionIterator>,
    mut func: &dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>, Type_a) -> Result<(metamodelica::Ref<DAE::Exp>, Type_a)>,
    mut iarg: Type_a,
) -> Result<(metamodelica::Ref<DAE::ReductionIterator>, Type_a)> {
    pub type FuncExpType<Type_a: Clone + 'static> = std::sync::Arc<
        dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>, Type_a) -> Result<(metamodelica::Ref<DAE::Exp>, Type_a)>
            + 'static,
    >;

    let mut outIter: metamodelica::Ref<DAE::ReductionIterator>;
    let mut outArg: Type_a;
    (outIter, outArg) = (match &*iter {
        DAE::ReductionIterator {
            id,
            exp,
            guardExp: gexp,
            ty,
        } => {
            let mut arg = iarg;
            let mut exp1: metamodelica::Ref<DAE::Exp>;
            let mut gexp1: Option<metamodelica::Ref<DAE::Exp>>;
            (exp1, arg) = traverseExpBottomUp(exp.clone(), func, arg)?;
            (gexp1, arg) = traverseExpOpt(gexp.clone(), func, arg)?;
            outIter = if (referenceEq(&*(exp.clone()), &*(&*exp1))
                && (match (&(gexp), &(gexp1)) {
                    (None, None) => true,
                    (Some(__refeq_l), Some(__refeq_r)) => referenceEq(&*(*__refeq_l), &*(*__refeq_r)),
                    _ => false,
                })) {
                iter
            } else {
                metamodelica::Ref::new(DAE::ReductionIterator {
                    id: id.clone(),
                    exp: exp1,
                    guardExp: gexp1,
                    ty: ty.clone(),
                })
            };
            (outIter, arg)
        }
    });
    Ok((outIter, outArg))
}

fn traverseReductionIterators<Type_a: Clone + 'static + metamodelica::gc::MMTrace>(
    mut iters: metamodelica::List<metamodelica::Ref<DAE::ReductionIterator>>,
    mut func: &dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>, Type_a) -> Result<(metamodelica::Ref<DAE::Exp>, Type_a)>,
    mut arg: Type_a,
) -> Result<(metamodelica::List<metamodelica::Ref<DAE::ReductionIterator>>, Type_a)> {
    pub type FuncExpType<Type_a: Clone + 'static> = std::sync::Arc<
        dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>, Type_a) -> Result<(metamodelica::Ref<DAE::Exp>, Type_a)>
            + 'static,
    >;

    let mut iters: metamodelica::List<metamodelica::Ref<DAE::ReductionIterator>> = iters;
    let mut arg: Type_a = arg;
    (iters, arg) = (::match_deref::match_deref! { match &(iters.clone()) {
        Deref @ metamodelica::ListNode::Nil => {
            (iters, arg)
        },
        Deref @ metamodelica::ListNode::Cons { head: iter, tail: rest } => {
            let mut iter1: metamodelica::Ref<DAE::ReductionIterator>;
            let mut iters1: metamodelica::List<metamodelica::Ref<DAE::ReductionIterator>>;
            (iter1, arg) = traverseReductionIterator(iter.clone(), func, arg)?;
            (iters1, arg) = traverseReductionIterators(rest.clone(), func, arg)?;
            iters = if (referenceEq(&*(iter.clone()),&*(&*iter1)) && metamodelica::ReferenceEq::reference_eq(&(rest.clone()), &(iters1))) {iters} else {metamodelica::cons(iter1, iters1)};
            (iters, arg)
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((iters, arg))
}

pub fn simpleCrefName(mut exp: &metamodelica::Ref<DAE::Exp>) -> Result<ArcStr> {
    let mut name: ArcStr;
    let __pa0 = ::match_deref::match_deref! { match &((*exp)) {
        Deref @ DAE::Exp::CREF { componentRef: Deref @ DAE::ComponentRef::CREF_IDENT { ident: __pa0, subscriptLst: Deref @ metamodelica::ListNode::Nil, .. }, .. } => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    name = metamodelica::Own::own(__pa0);
    Ok(name)
}

pub fn isTailCall(mut exp: &metamodelica::Ref<DAE::Exp>) -> bool {
    let mut isTail: bool;
    isTail = (::match_deref::match_deref! { match exp {
        Deref @ DAE::Exp::CALL { attr: Deref @ DAE::CallAttributes { tailCall: DAE::TailCall::TAIL { .. }, .. }, .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    isTail
}

pub fn complexityTraverse(
    mut exp: metamodelica::Ref<DAE::Exp>,
    mut complexity: i32,
) -> Result<(metamodelica::Ref<DAE::Exp>, i32)> {
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    let mut outComplexity: i32;
    (outExp, outComplexity) = traverseExpBottomUp(exp, &complexityTraverse2, complexity)?;
    Ok((outExp, outComplexity))
}

fn complexityTraverse2(
    mut exp: metamodelica::Ref<DAE::Exp>,
    mut complexity_: i32,
) -> Result<(metamodelica::Ref<DAE::Exp>, i32)> {
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    let mut outComplexity: i32;
    outComplexity = complexity_ + complexity(&exp)?;
    outExp = exp;
    Ok((outExp, outComplexity))
}

pub(crate) const complexityAlloc: i32 = 5;

pub(crate) const complexityVeryBig: i32 = 500000;

pub(crate) const complexityDimLarge: i32 = 1000;

pub(crate) fn complexity(mut exp: &metamodelica::Ref<DAE::Exp>) -> Result<i32> {
    let mut i: i32;
    i = (::match_deref::match_deref! { match exp {
        Deref @ DAE::Exp::ICONST { .. } => {
            0
        },
        Deref @ DAE::Exp::RCONST { .. } => {
            0
        },
        Deref @ DAE::Exp::SCONST { .. } => {
            0
        },
        Deref @ DAE::Exp::BCONST { .. } => {
            0
        },
        Deref @ DAE::Exp::SHARED_LITERAL { .. } => {
            0
        },
        Deref @ DAE::Exp::ENUM_LITERAL { .. } => {
            0
        },
        Deref @ DAE::Exp::CREF { ty: tp, .. } => {
            tpComplexity(tp)?
        },
        Deref @ DAE::Exp::BINARY { exp1: e1, operator: op, exp2: e2 } => {
            let mut c1: i32;
            let mut c2: i32;
            let mut c3: i32;
            c1 = complexity(e1)?;
            c2 = complexity(e2)?;
            c3 = opComplexity(op)?;
            c1 + c2 + c3
        },
        Deref @ DAE::Exp::UNARY { exp: e, operator: op } => {
            let mut c1: i32;
            let mut c2: i32;
            c1 = complexity(e)?;
            c2 = opComplexity(op)?;
            c1 + c2
        },
        Deref @ DAE::Exp::LBINARY { exp1: e1, exp2: e2, operator: op } => {
            let mut c1: i32;
            let mut c2: i32;
            let mut c3: i32;
            c1 = complexity(e1)?;
            c2 = complexity(e2)?;
            c3 = opComplexity(op)?;
            c1 + c2 + c3
        },
        Deref @ DAE::Exp::LUNARY { exp: e, operator: op } => {
            let mut c1: i32;
            let mut c2: i32;
            c1 = complexity(e)?;
            c2 = opComplexity(op)?;
            c1 + c2
        },
        Deref @ DAE::Exp::RELATION { exp1: e1, exp2: e2, operator: op, .. } => {
            let mut c1: i32;
            let mut c2: i32;
            let mut c3: i32;
            c1 = complexity(e1)?;
            c2 = complexity(e2)?;
            c3 = opComplexity(op)?;
            c1 + c2 + c3
        },
        Deref @ DAE::Exp::IFEXP { expCond: e1, expThen: e2, expElse: e3 } => {
            let mut c1: i32;
            let mut c2: i32;
            let mut c3: i32;
            c1 = complexity(e1)?;
            c2 = complexity(e2)?;
            c3 = complexity(e3)?;
            c1 + intMax(c2, c3)
        },
        Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name }, expLst: exps, attr: Deref @ DAE::CallAttributes { ty: tp, builtin: true, .. } } => {
            let mut c1: i32;
            let mut c2: i32;
            c1 = List::applyAndFold(exps, &fnptr!(intAdd, i32, i32), &move |__a0: metamodelica::Ref<DAE::Exp>| complexity(&__a0), 0)?;
            c2 = complexityBuiltin(metamodelica::AsArg::as_arg(&name), metamodelica::AsArg::as_arg(&tp))?;
            c1 + c2
        },
        Deref @ DAE::Exp::CALL { expLst: exps, .. } => {
            let mut c1: i32;
            let mut c2: i32;
            c1 = List::applyAndFold(exps, &fnptr!(intAdd, i32, i32), &move |__a0: metamodelica::Ref<DAE::Exp>| complexity(&__a0), 0)?;
            c2 = ((exps).len() as i32);
            c1 + c2 + 25
        },
        Deref @ DAE::Exp::RECORD { exps, .. } => {
            let mut c1: i32;
            c1 = List::applyAndFold(exps, &fnptr!(intAdd, i32, i32), &move |__a0: metamodelica::Ref<DAE::Exp>| complexity(&__a0), 1)?;
            c1
        },
        Deref @ DAE::Exp::PARTEVALFUNCTION { .. } => {
            complexityVeryBig.clone()
        },
        Deref @ DAE::Exp::ARRAY { array: exps, ty: tp, .. } => {
            let mut c1: i32;
            let mut c2: i32;
            c1 = List::applyAndFold(exps, &fnptr!(intAdd, i32, i32), &move |__a0: metamodelica::Ref<DAE::Exp>| complexity(&__a0), if (isArrayType(tp)) {0} else {complexityAlloc.clone()})?;
            c2 = ((exps).len() as i32);
            c1 + c2
        },
        Deref @ DAE::Exp::MATRIX { matrix: matrix @ Deref @ metamodelica::ListNode::Cons { head: exps, tail: _ }, .. } => {
            let mut c1: i32;
            let mut c2: i32;
            c1 = List::applyAndFold(&(List::flatten(matrix.clone())?), &fnptr!(intAdd, i32, i32), &move |__a0: metamodelica::Ref<DAE::Exp>| complexity(&__a0), complexityAlloc.clone())?;
            c2 = ((exps).len() as i32) * ((matrix).len() as i32);
            c1 + c2
        },
        Deref @ DAE::Exp::RANGE { start: e1, stop: e2, step: None, .. } => {
            complexityDimLarge.clone() + complexity(e1)? + complexity(e2)?
        },
        Deref @ DAE::Exp::RANGE { start: e1, stop: e2, step: Some(e3), .. } => {
            complexityDimLarge.clone() + complexity(e1)? + complexity(e2)? + complexity(metamodelica::AsArg::as_arg(&e3))?
        },
        Deref @ DAE::Exp::TUPLE { PR: exps } => {
            let mut c1: i32;
            let mut c2: i32;
            c1 = List::applyAndFold(exps, &fnptr!(intAdd, i32, i32), &move |__a0: metamodelica::Ref<DAE::Exp>| complexity(&__a0), complexityAlloc.clone())?;
            c2 = ((exps).len() as i32);
            c1 + c2
        },
        Deref @ DAE::Exp::CAST { exp: e, ty: tp } => {
            tpComplexity(tp)? + complexity(e)?
        },
        Deref @ DAE::Exp::ASUB { exp: e, sub: subs } => {
            let mut c1: i32;
            let mut c2: i32;
            let mut c3: i32;
            let mut exps: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
            exps = ({
        let mut __acc: metamodelica::List<metamodelica::Ref<DAE::Exp>> = metamodelica::nil();
        for mut sub in (subs.clone()).into_iter().cloned() {
            let __x = getSubscriptExp(&(sub.clone()))?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
            c1 = List::applyAndFold(&exps, &fnptr!(intAdd, i32, i32), &move |__a0: metamodelica::Ref<DAE::Exp>| complexity(&__a0), complexityAlloc.clone())?;
            c2 = ((exps).len() as i32);
            c3 = complexity(e)?;
            c1 + c2 + c3
        },
        Deref @ DAE::Exp::TSUB { exp: e, .. } => {
            complexity(e)? + 1
        },
        Deref @ DAE::Exp::SIZE { exp: e, sz: None } => {
            complexity(e)? + complexityAlloc.clone() + 10
        },
        Deref @ DAE::Exp::SIZE { exp: e1, sz: Some(e2) } => {
            complexity(e1)? + complexity(metamodelica::AsArg::as_arg(&e2))? + 1
        },
        Deref @ DAE::Exp::CODE { .. } => {
            complexityVeryBig.clone()
        },
        Deref @ DAE::Exp::EMPTY { .. } => {
            complexityVeryBig.clone()
        },
        Deref @ DAE::Exp::REDUCTION { .. } => {
            complexityVeryBig.clone()
        },
        Deref @ DAE::Exp::LIST { valList: exps } => {
            let mut c1: i32;
            let mut c2: i32;
            c1 = List::applyAndFold(exps, &fnptr!(intAdd, i32, i32), &move |__a0: metamodelica::Ref<DAE::Exp>| complexity(&__a0), complexityAlloc.clone())?;
            c2 = ((exps).len() as i32);
            c1 + c2 + complexityAlloc.clone()
        },
        Deref @ DAE::Exp::CONS { car: e1, cdr: e2 } => {
            complexityAlloc.clone() + complexity(e1)? + complexity(e2)?
        },
        Deref @ DAE::Exp::META_TUPLE { listExp: exps } => {
            let mut c1: i32;
            let mut c2: i32;
            c1 = List::applyAndFold(exps, &fnptr!(intAdd, i32, i32), &move |__a0: metamodelica::Ref<DAE::Exp>| complexity(&__a0), complexityAlloc.clone())?;
            c2 = ((exps).len() as i32);
            complexityAlloc.clone() + c1 + c2
        },
        Deref @ DAE::Exp::META_OPTION { exp: None } => {
            0
        },
        Deref @ DAE::Exp::META_OPTION { exp: Some(e) } => {
            complexity(metamodelica::AsArg::as_arg(&e))? + complexityAlloc.clone()
        },
        Deref @ DAE::Exp::METARECORDCALL { args: exps, .. } => {
            let mut c1: i32;
            let mut c2: i32;
            c1 = List::applyAndFold(exps, &fnptr!(intAdd, i32, i32), &move |__a0: metamodelica::Ref<DAE::Exp>| complexity(&__a0), complexityAlloc.clone())?;
            c2 = ((exps).len() as i32);
            c1 + c2 + complexityAlloc.clone()
        },
        Deref @ DAE::Exp::MATCHEXPRESSION { .. } => {
            complexityVeryBig.clone()
        },
        Deref @ DAE::Exp::BOX { exp: e } => {
            complexityAlloc.clone() + complexity(e)?
        },
        Deref @ DAE::Exp::UNBOX { exp: e, .. } => {
            1 + complexity(e)?
        },
        Deref @ DAE::Exp::PATTERN { .. } => {
            0
        },
        _ => {
            let mut r#str: ArcStr;
            r#str = { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Expression.complexityWork failed: ")); __mm_s.push_str(&*printExpStr(exp.clone())?); ArcStr::from(__mm_s) };
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![r#str])?;
            return Err("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(i)
}

fn complexityBuiltin(mut name: &ArcStr, mut tp: &metamodelica::Ref<DAE::Type>) -> Result<i32> {
    let mut complexity: i32;
    complexity = (::match_deref::match_deref! { match &(name.clone()) {
        Deref @ "identity" => complexityAlloc.clone() + tpComplexity(tp)?,
        Deref @ "cross" => 3 * 3,
        _ => 25,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(complexity)
}

fn tpComplexity(mut tp: &metamodelica::Ref<DAE::Type>) -> Result<i32> {
    let mut i: i32;
    i = (match &**tp {
        DAE::Type::T_ARRAY { dims, .. } => {
            i = List::applyAndFold(
                dims,
                &fnptr!(intMul, i32, i32),
                &move |__a0: metamodelica::Ref<DAE::Dimension>| -> metamodelica::Result<_> {
                    ::std::result::Result::Ok(dimComplexity(&__a0))
                },
                1,
            )?;
            i
        }
        _ => 0,
    });
    Ok(i)
}

pub(crate) fn dimComplexity(mut dim: &metamodelica::Ref<DAE::Dimension>) -> i32 {
    let mut i: i32;
    i = (match &**dim {
        DAE::Dimension::DIM_INTEGER { integer: __esc_i } => {
            i = (*__esc_i).clone();
            i.clone()
        }
        DAE::Dimension::DIM_ENUM { size: __esc_i, .. } => {
            i = (*__esc_i).clone();
            i.clone()
        }
        DAE::Dimension::DIM_BOOLEAN { .. } => 2,
        _ => complexityDimLarge.clone(),
    });
    i
}

fn opComplexity(mut op: &DAE::Operator) -> Result<i32> {
    let mut i: i32;
    i = (::match_deref::match_deref! { match &(op) {
        DAE::Operator::ADD { ty: Deref @ DAE::Type::T_STRING { .. } } => {
            100
        },
        DAE::Operator::ADD { .. } => {
            1
        },
        DAE::Operator::SUB { .. } => {
            1
        },
        DAE::Operator::MUL { .. } => {
            1
        },
        DAE::Operator::DIV { .. } => {
            1
        },
        DAE::Operator::POW { .. } => {
            30
        },
        DAE::Operator::UMINUS { .. } => {
            1
        },
        DAE::Operator::UMINUS_ARR { ty: tp } => {
            complexityAlloc.clone() + tpComplexity(metamodelica::AsArg::as_arg(&tp))?
        },
        DAE::Operator::ADD_ARR { ty: tp } => {
            complexityAlloc.clone() + tpComplexity(metamodelica::AsArg::as_arg(&tp))?
        },
        DAE::Operator::SUB_ARR { ty: tp } => {
            complexityAlloc.clone() + tpComplexity(metamodelica::AsArg::as_arg(&tp))?
        },
        DAE::Operator::MUL_ARR { ty: tp } => {
            complexityAlloc.clone() + tpComplexity(metamodelica::AsArg::as_arg(&tp))?
        },
        DAE::Operator::DIV_ARR { ty: tp } => {
            complexityAlloc.clone() + tpComplexity(metamodelica::AsArg::as_arg(&tp))?
        },
        DAE::Operator::MUL_ARRAY_SCALAR { ty: tp } => {
            complexityAlloc.clone() + tpComplexity(metamodelica::AsArg::as_arg(&tp))?
        },
        DAE::Operator::ADD_ARRAY_SCALAR { ty: tp } => {
            complexityAlloc.clone() + tpComplexity(metamodelica::AsArg::as_arg(&tp))?
        },
        DAE::Operator::SUB_SCALAR_ARRAY { ty: tp } => {
            complexityAlloc.clone() + tpComplexity(metamodelica::AsArg::as_arg(&tp))?
        },
        DAE::Operator::MUL_SCALAR_PRODUCT { ty: tp } => {
            complexityAlloc.clone() + 3 * tpComplexity(metamodelica::AsArg::as_arg(&tp))?
        },
        DAE::Operator::MUL_MATRIX_PRODUCT { ty: tp } => {
            complexityAlloc.clone() + 3 * tpComplexity(metamodelica::AsArg::as_arg(&tp))?
        },
        DAE::Operator::DIV_ARRAY_SCALAR { ty: tp } => {
            complexityAlloc.clone() + tpComplexity(metamodelica::AsArg::as_arg(&tp))?
        },
        DAE::Operator::DIV_SCALAR_ARRAY { ty: tp } => {
            complexityAlloc.clone() + tpComplexity(metamodelica::AsArg::as_arg(&tp))?
        },
        DAE::Operator::POW_ARRAY_SCALAR { ty: tp } => {
            complexityAlloc.clone() + 30 * tpComplexity(metamodelica::AsArg::as_arg(&tp))?
        },
        DAE::Operator::POW_SCALAR_ARRAY { ty: tp } => {
            complexityAlloc.clone() + 30 * tpComplexity(metamodelica::AsArg::as_arg(&tp))?
        },
        DAE::Operator::POW_ARR { ty: tp } => {
            complexityAlloc.clone() + 30 * tpComplexity(metamodelica::AsArg::as_arg(&tp))?
        },
        DAE::Operator::POW_ARR2 { ty: tp } => {
            complexityAlloc.clone() + 30 * tpComplexity(metamodelica::AsArg::as_arg(&tp))?
        },
        DAE::Operator::AND { .. } => {
            1
        },
        DAE::Operator::OR { .. } => {
            1
        },
        DAE::Operator::NOT { .. } => {
            1
        },
        DAE::Operator::LESS { .. } => {
            1
        },
        DAE::Operator::LESSEQ { .. } => {
            1
        },
        DAE::Operator::GREATER { .. } => {
            1
        },
        DAE::Operator::GREATEREQ { .. } => {
            1
        },
        DAE::Operator::EQUAL { .. } => {
            1
        },
        DAE::Operator::NEQUAL { .. } => {
            1
        },
        DAE::Operator::USERDEFINED { .. } => {
            100
        },
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![literal!("Expression.opWCET failed")])?;
            return Err("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(i)
}

pub fn makeEnumLiterals(
    mut inTypeName: metamodelica::Ref<Absyn::Path>,
    mut inLiterals: metamodelica::List<ArcStr>,
) -> Result<metamodelica::List<metamodelica::Ref<DAE::Exp>>> {
    let mut outLiterals: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
    let mut enum_lit_names: metamodelica::List<metamodelica::Ref<Absyn::Path>>;
    enum_lit_names = List::map1r(
        inLiterals,
        &move |__a0: metamodelica::Ref<Absyn::Path>, __a1: ArcStr| -> metamodelica::Result<_> {
            ::std::result::Result::Ok(AbsynUtil::suffixPath(&__a0, &__a1))
        },
        inTypeName,
    )?;
    (outLiterals, _) = List::mapFold(
        &enum_lit_names,
        &fnptr!(makeEnumLiteral, metamodelica::Ref<Absyn::Path>, i32),
        1,
    )?;
    Ok(outLiterals)
}

fn makeEnumLiteral(mut name: metamodelica::Ref<Absyn::Path>, mut index: i32) -> (metamodelica::Ref<DAE::Exp>, i32) {
    let mut enumExp: metamodelica::Ref<DAE::Exp>;
    let mut newIndex: i32;
    enumExp = metamodelica::Ref::new(DAE::Exp::ENUM_LITERAL {
        name: name,
        index: index,
    });
    newIndex = index + 1;
    (enumExp, newIndex)
}

pub fn isWild(mut exp: &metamodelica::Ref<DAE::Exp>) -> bool {
    let mut b: bool;
    b = (::match_deref::match_deref! { match exp {
        Deref @ DAE::Exp::CREF { componentRef: Deref @ DAE::ComponentRef::WILD { .. }, .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    b
}

pub fn isNotWild(mut exp: &metamodelica::Ref<DAE::Exp>) -> bool {
    let mut b: bool;
    b = (::match_deref::match_deref! { match exp {
        Deref @ DAE::Exp::CREF { componentRef: Deref @ DAE::ComponentRef::WILD { .. }, .. } => false,
        _ => true,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    b
}

pub fn dimensionsToExps(
    mut dims: &metamodelica::List<metamodelica::Ref<DAE::Dimension>>,
) -> metamodelica::List<metamodelica::Ref<DAE::Exp>> {
    let mut exps: metamodelica::List<metamodelica::Ref<DAE::Exp>> = metamodelica::nil();
    for mut d in &**dims {
        exps = (match &*d.clone() {
            DAE::Dimension::DIM_EXP { exp } => metamodelica::cons(exp.clone(), exps),
            _ => exps,
        });
    }
    exps = exps.reverse();
    exps
}

pub fn splitRecord<'__b>(
    mut inExp: &'__b metamodelica::Ref<DAE::Exp>,
    mut ty: &'__b metamodelica::Ref<DAE::Type>,
) -> Result<metamodelica::List<metamodelica::Ref<DAE::Exp>>> {
    '__tco: loop {
        ::match_deref::match_deref! { match (inExp, ty) {
            (Deref @ DAE::Exp::CAST { exp, .. }, _) => {
                { (inExp, ty) = (exp, ty); continue '__tco; }
            },
            (Deref @ DAE::Exp::CREF { .. }, Deref @ DAE::Type::T_COMPLEX { complexClassType: ClassInf::State::EXTERNAL_OBJ { .. }, varLst: Deref @ metamodelica::ListNode::Nil, .. }) => {
                return Ok(return Err("fail"))
            },
            (Deref @ DAE::Exp::CREF { componentRef: cr, .. }, Deref @ DAE::Type::T_COMPLEX { varLst: vs, .. }) => {
                return Ok(List::map1(vs.clone(), &move |__a0: metamodelica::Ref<DAE::Var>, __a1: metamodelica::Ref<DAE::ComponentRef>| splitRecord2(&__a0, &__a1), cr.clone())?)
            },
            (Deref @ DAE::Exp::CALL { path: p1, expLst: exps, attr: Deref @ DAE::CallAttributes { ty: Deref @ DAE::Type::T_COMPLEX { complexClassType: ClassInf::State::RECORD { path: p2 }, .. }, .. } }, _) => {
                let true = (AbsynUtil::pathEqual(p1, metamodelica::AsArg::as_arg(&p2))) else { return Err("pattern mismatch") };
                return Ok(exps.clone())
            },
            (Deref @ DAE::Exp::RECORD { exps, .. }, _) => {
                return Ok(exps.clone())
            },
            _ => return Err("match: no arm matched"),
        } }
    }
}

fn splitRecord2(
    mut var: &metamodelica::Ref<DAE::Var>,
    mut cr: &metamodelica::Ref<DAE::ComponentRef>,
) -> Result<metamodelica::Ref<DAE::Exp>> {
    let mut exp: metamodelica::Ref<DAE::Exp>;
    let mut n: ArcStr;
    let mut tt: metamodelica::Ref<DAE::Type>;
    let mut ty: metamodelica::Ref<DAE::Type>;
    let __arc2 = &(*var);
    let DAE::TYPES_VAR {
        name: __pa0, ty: __pa1, ..
    } = &**__arc2;
    n = metamodelica::Own::own(__pa0);
    tt = metamodelica::Own::own(__pa1);
    ty = Types::simplifyType(tt)?;
    exp = makeCrefExp(
        ComponentReference::crefPrependIdent(cr, &n, &(metamodelica::nil()), &ty)?,
        ty,
    )?;
    Ok(exp)
}

pub(crate) fn splitArray(
    mut inExp: metamodelica::Ref<DAE::Exp>,
) -> Result<(metamodelica::List<metamodelica::Ref<DAE::Exp>>, bool)> {
    let mut outExp: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
    let mut didSplit: bool;
    (outExp, didSplit) = (::match_deref::match_deref! { match &(inExp.clone()) {
        Deref @ DAE::Exp::ARRAY { array: expl, .. } => {
            (expl.clone(), true)
        },
        Deref @ DAE::Exp::MATRIX { matrix: mat, .. } => {
            (List::flatten(mat.clone())?, true)
        },
        Deref @ DAE::Exp::RANGE { start: Deref @ DAE::Exp::ICONST { integer: istart }, step, stop: Deref @ DAE::Exp::ICONST { integer: istop }, .. } => {
            let mut istep: i32;
            (({
        let mut __acc: metamodelica::List<metamodelica::Ref<DAE::Exp>> = metamodelica::nil();
        for mut i in (ExpressionSimplify::simplifyRange(istart.clone(), (::match_deref::match_deref! { match &(step.clone()) {
        None => 1,
        Some(Deref @ DAE::Exp::ICONST { integer: __esc_istep }) => {
            istep = (*__esc_istep).clone();
            istep.clone()
        },
        _ => return Err("match: no arm matched"),
    } }), istop.clone())?).into_iter().cloned() {
            let __x = metamodelica::Ref::new(DAE::Exp::ICONST { integer: i.clone() });
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }), true)
        },
        _ => {
            (list![inExp], false)
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outExp, didSplit))
}

pub(crate) fn equationExpEqual(
    mut exp1: &metamodelica::Ref<DAE::EquationExp>,
    mut exp2: &metamodelica::Ref<DAE::EquationExp>,
) -> Result<bool> {
    let mut b: bool;
    b = (::match_deref::match_deref! { match (exp1, exp2) {
        (Deref @ DAE::EquationExp::PARTIAL_EQUATION { exp: e1 }, Deref @ DAE::EquationExp::PARTIAL_EQUATION { exp: e2 }) => {
            ExpressionBasics::expEqual(e1, e2.clone())?
        },
        (Deref @ DAE::EquationExp::RESIDUAL_EXP { exp: e1 }, Deref @ DAE::EquationExp::RESIDUAL_EXP { exp: e2 }) => {
            ExpressionBasics::expEqual(e1, e2.clone())?
        },
        (Deref @ DAE::EquationExp::EQUALITY_EXPS { lhs: e1, rhs: e2 }, Deref @ DAE::EquationExp::EQUALITY_EXPS { lhs: e3, rhs: e4 }) => {
            ExpressionBasics::expEqual(e1, e3.clone())? && ExpressionBasics::expEqual(e2, e4.clone())?
        },
        _ => {
            false
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(b)
}

pub fn promoteExp(
    mut inExp: metamodelica::Ref<DAE::Exp>,
    mut inType: metamodelica::Ref<DAE::Type>,
    mut inDims: i32,
) -> (metamodelica::Ref<DAE::Exp>, metamodelica::Ref<DAE::Type>) {
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    let mut outType: metamodelica::Ref<DAE::Type>;
    (outExp, outType) = 'mc: {
        let __mc_input = inDims;
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            let mut dims_to_add: i32;
            let mut ty: metamodelica::Ref<DAE::Type>;
            let mut res_ty: metamodelica::Ref<DAE::Type>;
            let mut exp: metamodelica::Ref<DAE::Exp>;
            let mut tys: metamodelica::List<metamodelica::Ref<DAE::Type>>;
            let mut dims: metamodelica::List<metamodelica::Ref<DAE::Dimension>>;
            let mut added_dims: metamodelica::List<metamodelica::Ref<DAE::Dimension>>;
            let mut is_array_ty: bool;
            dims_to_add = inDims - Types::numberOfDimensions(&inType);
            let true = (dims_to_add > 0) else {
                return Err("pattern mismatch");
            };
            added_dims = List::fill(
                metamodelica::Ref::new(DAE::Dimension::DIM_INTEGER { integer: 1 }),
                dims_to_add,
            );
            dims = listAppend(TypesDump::getDimensions(&inType), added_dims.clone());
            ty = Types::arrayElementType(&inType);
            res_ty = Types::liftArrayListDims(ty.clone(), dims.clone());
            ty = Types::simplifyType(ty.clone())?;
            tys = makePromotedTypes(&dims, &ty, metamodelica::nil());
            is_array_ty = Types::isArray(&inType);
            exp = promoteExp2(inExp.clone(), is_array_ty, inDims, &tys)?;
            Ok((exp.clone(), res_ty.clone()))
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            Ok((inExp.clone(), inType.clone()))
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    (outExp, outType)
}

fn makePromotedTypes<'__b>(
    mut inDimensions: &'__b metamodelica::List<metamodelica::Ref<DAE::Dimension>>,
    mut inElementType: &'__b metamodelica::Ref<DAE::Type>,
    mut inAccumTypes: metamodelica::List<metamodelica::Ref<DAE::Type>>,
) -> metamodelica::List<metamodelica::Ref<DAE::Type>> {
    '__tco: loop {
        ::match_deref::match_deref! { match inDimensions {
            Deref @ metamodelica::ListNode::Cons { head: _, tail: rest_dims } => {
                let mut ty: metamodelica::Ref<DAE::Type>;
                ty = metamodelica::Ref::new(DAE::Type::T_ARRAY { ty: inElementType.clone(), dims: inDimensions.clone() });
                { (inDimensions, inElementType, inAccumTypes) = (rest_dims, inElementType, metamodelica::cons(ty, inAccumTypes)); continue '__tco; }
            },
            Deref @ metamodelica::ListNode::Nil => {
                return inAccumTypes.reverse()
            },
            _ => unreachable!("tail-call lowered match: no arm matched"),
        } }
    }
}

fn promoteExp2(
    mut inExp: metamodelica::Ref<DAE::Exp>,
    mut inIsArray: bool,
    mut inDims: i32,
    mut inTypes: &metamodelica::List<metamodelica::Ref<DAE::Type>>,
) -> Result<metamodelica::Ref<DAE::Exp>> {
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    outExp = (::match_deref::match_deref! { match &((inExp.clone(), inIsArray, inTypes.clone())) {
        (_, _, Deref @ metamodelica::ListNode::Nil) => {
            inExp
        },
        (Deref @ DAE::Exp::ARRAY { ty: _, scalar: _, array: expl }, _, Deref @ metamodelica::ListNode::Cons { head: ty, tail: rest_ty }) => {
            let mut expl = (*expl).clone();
            expl = List::map3(expl.clone(), &move |__a0: metamodelica::Ref<DAE::Exp>, __a1: bool, __a2: i32, __a3: metamodelica::List<metamodelica::Ref<DAE::Type>>| promoteExp2(__a0, __a1, __a2, &__a3), false, inDims, rest_ty.clone())?;
            metamodelica::Ref::new(DAE::Exp::ARRAY { ty: ty.clone(), scalar: false, array: expl.clone() })
        },
        (_, true, Deref @ metamodelica::ListNode::Cons { head: ty, tail: _ }) => {
            makePureBuiltinCall(literal!("promote"), list![inExp, metamodelica::Ref::new(DAE::Exp::ICONST { integer: inDims })], ty.clone())
        },
        _ => {
            promoteExp3(&inExp, inTypes)?
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outExp)
}

fn promoteExp3(
    mut inExp: &metamodelica::Ref<DAE::Exp>,
    mut inTypes: &metamodelica::List<metamodelica::Ref<DAE::Type>>,
) -> Result<metamodelica::Ref<DAE::Exp>> {
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    outExp = (::match_deref::match_deref! { match inTypes {
        Deref @ metamodelica::ListNode::Nil => {
            inExp.clone()
        },
        Deref @ metamodelica::ListNode::Cons { head: ty, tail: Deref @ metamodelica::ListNode::Nil } => {
            makeArray(list![inExp.clone()], ty.clone(), true)
        },
        Deref @ metamodelica::ListNode::Cons { head: ty, tail: rest_ty } => {
            let mut exp: metamodelica::Ref<DAE::Exp>;
            exp = promoteExp3(inExp, rest_ty)?;
            makeArray(list![exp], ty.clone(), false)
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outExp)
}

pub fn matrixToArray(mut inMatrix: metamodelica::Ref<DAE::Exp>) -> Result<metamodelica::Ref<DAE::Exp>> {
    let mut outArray: metamodelica::Ref<DAE::Exp>;
    outArray = (match &*inMatrix {
        DAE::Exp::MATRIX { ty, matrix, .. } => {
            let mut row_ty: metamodelica::Ref<DAE::Type>;
            let mut rows: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
            row_ty = unliftArray(metamodelica::AsArg::as_arg(&ty))?;
            rows = List::map2(
                matrix.clone(),
                &fnptr!(
                    makeArray,
                    metamodelica::List<metamodelica::Ref<DAE::Exp>>,
                    metamodelica::Ref<DAE::Type>,
                    bool
                ),
                row_ty,
                true,
            )?;
            metamodelica::Ref::new(DAE::Exp::ARRAY {
                ty: ty.clone(),
                scalar: false,
                array: rows,
            })
        }
        _ => inMatrix,
    });
    Ok(outArray)
}

pub(crate) fn transposeArray(mut inArray: metamodelica::Ref<DAE::Exp>) -> Result<(metamodelica::Ref<DAE::Exp>, bool)> {
    let mut outArray: metamodelica::Ref<DAE::Exp>;
    let mut outWasTransposed: bool;
    (outArray, outWasTransposed) = (::match_deref::match_deref! { match &(inArray.clone()) {
        Deref @ DAE::Exp::ARRAY { ty: Deref @ DAE::Type::T_ARRAY { ty, dims: Deref @ metamodelica::ListNode::Cons { head: dim1, tail: Deref @ metamodelica::ListNode::Cons { head: dim2, tail: rest_dims } } }, scalar: _, array: Deref @ metamodelica::ListNode::Nil } => {
            (metamodelica::Ref::new(DAE::Exp::ARRAY { ty: metamodelica::Ref::new(DAE::Type::T_ARRAY { ty: ty.clone(), dims: metamodelica::cons(dim2.clone(), metamodelica::cons(dim1.clone(), rest_dims.clone())) }), scalar: false, array: metamodelica::nil() }), true)
        },
        Deref @ DAE::Exp::ARRAY { ty: Deref @ DAE::Type::T_ARRAY { ty, dims: Deref @ metamodelica::ListNode::Cons { head: dim1, tail: Deref @ metamodelica::ListNode::Cons { head: dim2, tail: rest_dims } } }, scalar: _, array: expl } => {
            let mut row_ty: metamodelica::Ref<DAE::Type>;
            let mut matrix: metamodelica::List<metamodelica::List<metamodelica::Ref<DAE::Exp>>>;
            let mut expl = (*expl).clone();
            row_ty = metamodelica::Ref::new(DAE::Type::T_ARRAY { ty: ty.clone(), dims: metamodelica::cons(dim1.clone(), rest_dims.clone()) });
            matrix = List::map(expl.clone(), &move |__a0: metamodelica::Ref<DAE::Exp>| getArrayOrMatrixContents(&__a0))?;
            matrix = List::transposeList(matrix)?;
            expl = List::map2(matrix, &fnptr!(makeArray, metamodelica::List<metamodelica::Ref<DAE::Exp>>, metamodelica::Ref<DAE::Type>, bool), row_ty, true)?;
            (metamodelica::Ref::new(DAE::Exp::ARRAY { ty: metamodelica::Ref::new(DAE::Type::T_ARRAY { ty: ty.clone(), dims: metamodelica::cons(dim2.clone(), metamodelica::cons(dim1.clone(), rest_dims.clone())) }), scalar: false, array: expl.clone() }), true)
        },
        Deref @ DAE::Exp::MATRIX { matrix, ty: Deref @ DAE::Type::T_ARRAY { ty, dims: Deref @ metamodelica::ListNode::Cons { head: dim1, tail: Deref @ metamodelica::ListNode::Cons { head: dim2, tail: Deref @ metamodelica::ListNode::Nil } } }, .. } => {
            let mut i: i32;
            let mut matrix = (*matrix).clone();
            let mut ty = (*ty).clone();
            matrix = List::transposeList(matrix.clone())?;
            ty = metamodelica::Ref::new(DAE::Type::T_ARRAY { ty: ty.clone(), dims: list![dim2.clone(), dim1.clone()] });
            i = ((matrix).len() as i32);
            (metamodelica::Ref::new(DAE::Exp::MATRIX { ty: ty.clone(), integer: i, matrix: matrix.clone() }), true)
        },
        _ => {
            (inArray, false)
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outArray, outWasTransposed))
}

pub fn getCrefFromCrefOrAsub(mut exp: &metamodelica::Ref<DAE::Exp>) -> Result<metamodelica::Ref<DAE::ComponentRef>> {
    let mut cr: metamodelica::Ref<DAE::ComponentRef>;
    cr = (::match_deref::match_deref! { match exp {
        Deref @ DAE::Exp::CREF { componentRef: __esc_cr, .. } => {
            cr = (*__esc_cr).clone();
            cr.clone()
        },
        Deref @ DAE::Exp::ASUB { exp: Deref @ DAE::Exp::CREF { componentRef: __esc_cr, .. }, .. } => {
            cr = (*__esc_cr).clone();
            cr.clone()
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok(cr)
}

pub fn arrayElements(
    mut inExp: metamodelica::Ref<DAE::Exp>,
) -> Result<metamodelica::List<metamodelica::Ref<DAE::Exp>>> {
    let mut outExp: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
    outExp = (::match_deref::match_deref! { match &(inExp.clone()) {
        Deref @ DAE::Exp::CREF { componentRef: cr, .. } => {
            let mut expl: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
            let mut crl: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
            crl = ComponentReference::expandCref(metamodelica::AsArg::as_arg(&cr), false)?;
            expl = List::map(crl, &crefExp)?;
            expl
        },
        Deref @ DAE::Exp::ARRAY { array: expl, ty: Deref @ DAE::Type::T_ARRAY { .. }, .. } => {
            List::mapFlat(metamodelica::AsArg::as_arg(&expl), &arrayElements)?
        },
        Deref @ DAE::Exp::ARRAY { array: expl, .. } => {
            expl.clone()
        },
        Deref @ DAE::Exp::MATRIX { matrix: mat, .. } => {
            List::flatten(mat.clone())?
        },
        _ => {
            list![inExp]
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outExp)
}

pub fn arrayContent(
    mut inExp: &metamodelica::Ref<DAE::Exp>,
) -> Result<metamodelica::List<metamodelica::Ref<DAE::Exp>>> {
    let mut outContent: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
    let __pa0 = ::match_deref::match_deref! { match &((*inExp)) {
        Deref @ DAE::Exp::ARRAY { array: __pa0, .. } => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    outContent = metamodelica::Own::own(__pa0);
    Ok(outContent)
}

pub fn fromAbsynExp(mut inAExp: metamodelica::Ref<Absyn::Exp>) -> Result<metamodelica::Ref<DAE::Exp>> {
    let mut outDExp: metamodelica::Ref<DAE::Exp>;
    outDExp = (::match_deref::match_deref! { match &(inAExp.clone()) {
        Deref @ Absyn::Exp::INTEGER { value: i } => {
            metamodelica::Ref::new(DAE::Exp::ICONST { integer: i.clone() })
        },
        Deref @ Absyn::Exp::REAL { value: s } => {
            let mut r: metamodelica::Real;
            r = stringReal(s.clone())?;
            metamodelica::Ref::new(DAE::Exp::RCONST { real: r })
        },
        Deref @ Absyn::Exp::BOOL { value: b } => {
            metamodelica::Ref::new(DAE::Exp::BCONST { bool: b.clone() })
        },
        Deref @ Absyn::Exp::STRING { value: s } => {
            metamodelica::Ref::new(DAE::Exp::SCONST { string: s.clone() })
        },
        Deref @ Absyn::Exp::CREF { componentRef: acr } => {
            let mut cr: metamodelica::Ref<DAE::ComponentRef>;
            let mut e: metamodelica::Ref<DAE::Exp>;
            cr = ComponentReference::toExpCref(metamodelica::AsArg::as_arg(&acr))?;
            e = makeCrefExp(cr, DAE::T_UNKNOWN_DEFAULT().clone())?;
            e
        },
        Deref @ Absyn::Exp::BINARY { exp1: ae1, op: aop, exp2: ae2 } => {
            let mut e: metamodelica::Ref<DAE::Exp>;
            let mut e1: metamodelica::Ref<DAE::Exp>;
            let mut e2: metamodelica::Ref<DAE::Exp>;
            let mut op: DAE::Operator;
            op = fromAbsynOperator(aop.clone(), DAE::T_UNKNOWN_DEFAULT().clone())?;
            e1 = fromAbsynExp(ae1.clone())?;
            e2 = fromAbsynExp(ae2.clone())?;
            e = metamodelica::Ref::new(DAE::Exp::BINARY { exp1: e1, operator: op, exp2: e2 });
            e
        },
        Deref @ Absyn::Exp::UNARY { op: aop, exp: ae } => {
            let mut e: metamodelica::Ref<DAE::Exp>;
            let mut op: DAE::Operator;
            op = fromAbsynOperator(aop.clone(), DAE::T_UNKNOWN_DEFAULT().clone())?;
            e = fromAbsynExp(ae.clone())?;
            e = metamodelica::Ref::new(DAE::Exp::UNARY { operator: op, exp: e });
            e
        },
        Deref @ Absyn::Exp::LBINARY { exp1: ae1, op: aop, exp2: ae2 } => {
            let mut e: metamodelica::Ref<DAE::Exp>;
            let mut e1: metamodelica::Ref<DAE::Exp>;
            let mut e2: metamodelica::Ref<DAE::Exp>;
            let mut op: DAE::Operator;
            op = fromAbsynOperator(aop.clone(), DAE::T_UNKNOWN_DEFAULT().clone())?;
            e1 = fromAbsynExp(ae1.clone())?;
            e2 = fromAbsynExp(ae2.clone())?;
            e = metamodelica::Ref::new(DAE::Exp::LBINARY { exp1: e1, operator: op, exp2: e2 });
            e
        },
        Deref @ Absyn::Exp::LUNARY { op: aop, exp: ae } => {
            let mut e: metamodelica::Ref<DAE::Exp>;
            let mut op: DAE::Operator;
            op = fromAbsynOperator(aop.clone(), DAE::T_UNKNOWN_DEFAULT().clone())?;
            e = fromAbsynExp(ae.clone())?;
            e = metamodelica::Ref::new(DAE::Exp::LUNARY { operator: op, exp: e });
            e
        },
        Deref @ Absyn::Exp::RELATION { exp1: ae1, op: aop, exp2: ae2 } => {
            let mut e: metamodelica::Ref<DAE::Exp>;
            let mut e1: metamodelica::Ref<DAE::Exp>;
            let mut e2: metamodelica::Ref<DAE::Exp>;
            let mut op: DAE::Operator;
            op = fromAbsynOperator(aop.clone(), DAE::T_UNKNOWN_DEFAULT().clone())?;
            e1 = fromAbsynExp(ae1.clone())?;
            e2 = fromAbsynExp(ae2.clone())?;
            e = metamodelica::Ref::new(DAE::Exp::RELATION { exp1: e1, operator: op, exp2: e2, index: 0, optionExpisASUB: None });
            e
        },
        ae @ Deref @ Absyn::Exp::IFEXP { .. } => {
            let mut ae1: metamodelica::Ref<Absyn::Exp>;
            let mut ae2: metamodelica::Ref<Absyn::Exp>;
            let mut cond: metamodelica::Ref<Absyn::Exp>;
            let mut e: metamodelica::Ref<DAE::Exp>;
            let mut e1: metamodelica::Ref<DAE::Exp>;
            let mut e2: metamodelica::Ref<DAE::Exp>;
            let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(AbsynUtil::canonIfExp(metamodelica::AsArg::as_arg(&ae))?) {
                Deref @ Absyn::Exp::IFEXP { ifExp: __pa0, trueBranch: __pa1, elseBranch: __pa2, .. } => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
                _ => return Err("pattern mismatch"),
            } };
            cond = metamodelica::Own::own(__pa0);
            ae1 = metamodelica::Own::own(__pa1);
            ae2 = metamodelica::Own::own(__pa2);
            e = fromAbsynExp(cond)?;
            e1 = fromAbsynExp(ae1)?;
            e2 = fromAbsynExp(ae2)?;
            e = metamodelica::Ref::new(DAE::Exp::IFEXP { expCond: e, expThen: e1, expElse: e2 });
            e
        },
        Deref @ Absyn::Exp::CALL { function_: acr, functionArgs: fargs, .. } => {
            let mut p: metamodelica::Ref<Absyn::Path>;
            let mut e: metamodelica::Ref<DAE::Exp>;
            let mut exps: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
            exps = fargsToExps(metamodelica::AsArg::as_arg(&fargs))?;
            p = AbsynUtil::crefToPath(metamodelica::AsArg::as_arg(&acr))?;
            e = metamodelica::Ref::new(DAE::Exp::CALL { path: p, expLst: exps, attr: DAE::callAttrBuiltinOther().clone() });
            e
        },
        Deref @ Absyn::Exp::PARTEVALFUNCTION { function_: acr, functionArgs: fargs } => {
            let mut p: metamodelica::Ref<Absyn::Path>;
            let mut e: metamodelica::Ref<DAE::Exp>;
            let mut exps: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
            exps = fargsToExps(metamodelica::AsArg::as_arg(&fargs))?;
            p = AbsynUtil::crefToPath(metamodelica::AsArg::as_arg(&acr))?;
            e = metamodelica::Ref::new(DAE::Exp::PARTEVALFUNCTION { path: p, expList: exps, ty: DAE::T_UNKNOWN_DEFAULT().clone(), origType: DAE::T_UNKNOWN_DEFAULT().clone() });
            e
        },
        Deref @ Absyn::Exp::ARRAY { arrayExp: aexps } => {
            let mut e: metamodelica::Ref<DAE::Exp>;
            let mut exps: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
            exps = List::map(aexps.clone(), &fromAbsynExp)?;
            e = metamodelica::Ref::new(DAE::Exp::ARRAY { ty: DAE::T_UNKNOWN_DEFAULT().clone(), scalar: false, array: exps });
            e
        },
        Deref @ Absyn::Exp::MATRIX { matrix: aexpslst } => {
            let mut i: i32;
            let mut e: metamodelica::Ref<DAE::Exp>;
            let mut expslst: metamodelica::List<metamodelica::List<metamodelica::Ref<DAE::Exp>>>;
            expslst = List::mapList(aexpslst.clone(), &fromAbsynExp)?;
            i = ((((expslst).head().cloned()?)).len() as i32);
            e = metamodelica::Ref::new(DAE::Exp::MATRIX { ty: DAE::T_UNKNOWN_DEFAULT().clone(), integer: i, matrix: expslst });
            e
        },
        Deref @ Absyn::Exp::RANGE { start: ae1, step: aoe, stop: ae2 } => {
            let mut e: metamodelica::Ref<DAE::Exp>;
            let mut e1: metamodelica::Ref<DAE::Exp>;
            let mut e2: metamodelica::Ref<DAE::Exp>;
            let mut oe: Option<metamodelica::Ref<DAE::Exp>>;
            e1 = fromAbsynExp(ae1.clone())?;
            e2 = fromAbsynExp(ae2.clone())?;
            oe = fromAbsynExpOpt(aoe.clone())?;
            e = metamodelica::Ref::new(DAE::Exp::RANGE { ty: DAE::T_UNKNOWN_DEFAULT().clone(), start: e1, step: oe, stop: e2 });
            e
        },
        Deref @ Absyn::Exp::TUPLE { expressions: aexps } => {
            let mut e: metamodelica::Ref<DAE::Exp>;
            let mut exps: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
            exps = List::map(aexps.clone(), &fromAbsynExp)?;
            e = metamodelica::Ref::new(DAE::Exp::TUPLE { PR: exps });
            e
        },
        _ => {
            metamodelica::print({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Expression.fromAbsynExp: Unhandled expression: ")); __mm_s.push_str(&*Dump::printExpStr(inAExp)?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) });
            return Err("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outDExp)
}

pub(crate) fn fargsToExps(
    mut inFargs: &metamodelica::Ref<Absyn::FunctionArgs>,
) -> Result<metamodelica::List<metamodelica::Ref<DAE::Exp>>> {
    let mut outExps: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
    outExps = 'mc: {
        let __mc_input = &**inFargs;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Absyn::FunctionArgs::FUNCTIONARGS { args: aexps, argNames: Deref @ metamodelica::ListNode::Nil } => {
                    let mut exps: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                    exps = List::map(aexps.clone(), &fromAbsynExp)?;
                    Ok(exps.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Absyn::FunctionArgs::FUNCTIONARGS { args: _, argNames: _ } => {
                    metamodelica::print(literal!("Expression.fargsToExps: Named arguments are not handled!\n"));
                    Ok(metamodelica::nil())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(outExps)
}

fn fromAbsynExpOpt(mut aoe: Option<metamodelica::Ref<Absyn::Exp>>) -> Result<Option<metamodelica::Ref<DAE::Exp>>> {
    let mut oe: Option<metamodelica::Ref<DAE::Exp>>;
    oe = (::match_deref::match_deref! { match &(aoe) {
        None => {
            None
        },
        Some(ae) => {
            let mut e: metamodelica::Ref<DAE::Exp>;
            e = fromAbsynExp(ae.clone())?;
            Some(e)
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(oe)
}

fn fromAbsynOperator(mut aop: Absyn::Operator, mut ty: metamodelica::Ref<DAE::Type>) -> Result<DAE::Operator> {
    let mut op: DAE::Operator;
    op = (match aop {
        Absyn::Operator::ADD { .. } => DAE::Operator::ADD { ty: ty },
        Absyn::Operator::SUB { .. } => DAE::Operator::SUB { ty: ty },
        Absyn::Operator::MUL { .. } => DAE::Operator::MUL { ty: ty },
        Absyn::Operator::DIV { .. } => DAE::Operator::DIV { ty: ty },
        Absyn::Operator::POW { .. } => DAE::Operator::POW { ty: ty },
        Absyn::Operator::UMINUS { .. } => DAE::Operator::UMINUS { ty: ty },
        Absyn::Operator::AND { .. } => DAE::Operator::AND { ty: ty },
        Absyn::Operator::OR { .. } => DAE::Operator::OR { ty: ty },
        Absyn::Operator::NOT { .. } => DAE::Operator::NOT { ty: ty },
        Absyn::Operator::LESS { .. } => DAE::Operator::LESS { ty: ty },
        Absyn::Operator::LESSEQ { .. } => DAE::Operator::LESSEQ { ty: ty },
        Absyn::Operator::GREATER { .. } => DAE::Operator::GREATER { ty: ty },
        Absyn::Operator::GREATEREQ { .. } => DAE::Operator::GREATEREQ { ty: ty },
        Absyn::Operator::EQUAL { .. } => DAE::Operator::EQUAL { ty: ty },
        Absyn::Operator::NEQUAL { .. } => DAE::Operator::NEQUAL { ty: ty },
        _ => {
            metamodelica::print({
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("Expression.fromAbsynOperator: Unhandled operator: "));
                __mm_s.push_str(&*Dump::opSymbol(aop)?);
                __mm_s.push_str(&*literal!("\n"));
                ArcStr::from(__mm_s)
            });
            return Err("fail");
        }
    });
    Ok(op)
}

pub fn replaceDerOpInExp(mut inExp: metamodelica::Ref<DAE::Exp>) -> Result<metamodelica::Ref<DAE::Exp>> {
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    (outExp, _) = traverseExpBottomUp(
        inExp,
        &fnptr!(
            replaceDerOpInExpTraverser,
            metamodelica::Ref<DAE::Exp>,
            Option<metamodelica::Ref<DAE::ComponentRef>>
        ),
        None,
    )?;
    Ok(outExp)
}

pub fn replaceDerOpInExpCond(
    mut e: metamodelica::Ref<DAE::Exp>,
    mut cr: Option<metamodelica::Ref<DAE::ComponentRef>>,
) -> Result<(
    metamodelica::Ref<DAE::Exp>,
    Option<metamodelica::Ref<DAE::ComponentRef>>,
)> {
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    let mut outCr: Option<metamodelica::Ref<DAE::ComponentRef>>;
    (outExp, outCr) = traverseExpBottomUp(
        e,
        &fnptr!(
            replaceDerOpInExpTraverser,
            metamodelica::Ref<DAE::Exp>,
            Option<metamodelica::Ref<DAE::ComponentRef>>
        ),
        cr,
    )?;
    Ok((outExp, outCr))
}

pub(crate) fn replaceDerOpInExpTraverser(
    mut e: metamodelica::Ref<DAE::Exp>,
    mut optCr: Option<metamodelica::Ref<DAE::ComponentRef>>,
) -> (
    metamodelica::Ref<DAE::Exp>,
    Option<metamodelica::Ref<DAE::ComponentRef>>,
) {
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    let mut outCr: Option<metamodelica::Ref<DAE::ComponentRef>>;
    (outExp, outCr) = 'mc: {
        let __mc_input = (&*e, &optCr);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "der" }, expLst: Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Exp::CREF { componentRef: cr, .. }, tail: Deref @ metamodelica::ListNode::Nil }, .. }, Some(cref)) => {
                    let mut derCr: metamodelica::Ref<DAE::ComponentRef>;
                    let mut cref_exp: metamodelica::Ref<DAE::Exp>;
                    derCr = ComponentReference::crefPrefixDer(cr.clone());
                    let true = (ComponentReferenceBasics::crefEqualNoStringCompare(&derCr, metamodelica::AsArg::as_arg(&cref))?) else { return Err("pattern mismatch") };
                    cref_exp = crefExp(derCr.clone())?;
                    Ok((cref_exp.clone(), optCr.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "der" }, expLst: Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Exp::CREF { componentRef: cr, .. }, tail: Deref @ metamodelica::ListNode::Nil }, .. }, None) => {
                    let mut cref_exp: metamodelica::Ref<DAE::Exp>;
                    let mut cr = (*cr).clone();
                    cr = ComponentReference::crefPrefixDer(cr.clone());
                    cref_exp = crefExp(cr.clone())?;
                    Ok((cref_exp.clone(), None))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((e.clone(), optCr.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    (outExp, outCr)
}

pub fn makeBinaryExp(
    mut inLhs: metamodelica::Ref<DAE::Exp>,
    mut inOp: DAE::Operator,
    mut inRhs: metamodelica::Ref<DAE::Exp>,
) -> metamodelica::Ref<DAE::Exp> {
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    outExp = metamodelica::Ref::new(DAE::Exp::BINARY {
        exp1: inLhs,
        operator: inOp,
        exp2: inRhs,
    });
    outExp
}

pub(crate) fn checkExpDimensionSizes(mut dim: &metamodelica::Ref<DAE::Exp>) -> bool {
    let mut value: bool;
    value = (match &**dim {
        DAE::Exp::ICONST { integer: __dim_integer } => __dim_integer.clone() > 0,
        _ => false,
    });
    value
}

pub(crate) fn checkDimensionSizes(mut dim: &metamodelica::Ref<DAE::Dimension>) -> bool {
    let mut value: bool;
    value = (match &**dim {
        DAE::Dimension::DIM_INTEGER { .. } => true,
        DAE::Dimension::DIM_ENUM { .. } => true,
        DAE::Dimension::DIM_BOOLEAN { .. } => true,
        DAE::Dimension::DIM_EXP { .. } => true,
        DAE::Dimension::DIM_UNKNOWN { .. } => false,
    });
    value
}

pub fn dimensionsList(mut inDims: metamodelica::List<metamodelica::Ref<DAE::Dimension>>) -> metamodelica::List<i32> {
    let mut outValues: metamodelica::List<i32>;
    let mut dims: metamodelica::List<i32> = metamodelica::nil();
    outValues = 'mc: {
        let __mc_input = &*inDims;
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut dims: metamodelica::List<i32> = dims.clone();
                    let true = (List::all(&inDims, &move |__a0: metamodelica::Ref<DAE::Dimension>| -> metamodelica::Result<_> { ::std::result::Result::Ok(checkDimensionSizes(&__a0)) })?) else { return Err("pattern mismatch") };
                    dims = List::map(inDims.clone(), &move |__a0: metamodelica::Ref<DAE::Dimension>| dimensionSizeAll(&__a0))?;
                    Ok((dims.clone(), dims.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            dims = __wb0;
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(metamodelica::nil())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    outValues
}

pub fn hasZeroDimension(mut inDims: metamodelica::List<metamodelica::Ref<DAE::Dimension>>) -> bool {
    let mut hasZeroDimension: bool = false;
    let mut intDims: metamodelica::List<i32>;
    if (inDims).is_empty() {
        hasZeroDimension = true;
        return hasZeroDimension;
    }
    intDims = dimensionsList(inDims);
    for mut dim in &*intDims {
        if dim.clone() == 0 {
            hasZeroDimension = true;
            break;
        }
    }
    hasZeroDimension
}

pub fn expDimensionsList(mut inDims: metamodelica::List<metamodelica::Ref<DAE::Exp>>) -> metamodelica::List<i32> {
    let mut outValues: metamodelica::List<i32>;
    let mut dims: metamodelica::List<i32> = metamodelica::nil();
    outValues = 'mc: {
        let __mc_input = &*inDims;
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut dims: metamodelica::List<i32> = dims.clone();
                    let true = (List::all(&inDims, &move |__a0: metamodelica::Ref<DAE::Exp>| -> metamodelica::Result<_> { ::std::result::Result::Ok(checkExpDimensionSizes(&__a0)) })?) else { return Err("pattern mismatch") };
                    dims = List::map(inDims.clone(), &move |__a0: metamodelica::Ref<DAE::Exp>| expInt(&__a0))?;
                    Ok((dims.clone(), dims.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            dims = __wb0;
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(metamodelica::nil())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    outValues
}

pub fn isCrefListWithEqualIdents(mut iExpressions: metamodelica::List<metamodelica::Ref<DAE::Exp>>) -> bool {
    let mut oCrefWithEqualIdents: bool;
    let mut tmpCrefWithEqualIdents: bool = false;
    let mut crefs: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>> = metamodelica::nil();
    let mut head: metamodelica::Ref<DAE::Exp>;
    let mut headCref: metamodelica::Ref<DAE::ComponentRef> = metamodelica::Ref::new(DAE::ComponentRef::WILD);
    oCrefWithEqualIdents = 'mc: {
        let __mc_input = &*iExpressions;
        if let Ok((__v, __wb0, __wb1, __wb2)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: head, tail: _ } => {
                    let mut crefs: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>> = crefs.clone();
                    let mut headCref: metamodelica::Ref<DAE::ComponentRef> = headCref.clone();
                    let mut tmpCrefWithEqualIdents: bool = tmpCrefWithEqualIdents.clone();
                    let true = (List::all(&iExpressions, &move |__a0: metamodelica::Ref<DAE::Exp>| -> metamodelica::Result<_> { ::std::result::Result::Ok(isCref(&__a0)) })?) else { return Err("pattern mismatch") };
                    crefs = List::map(iExpressions.clone(), &move |__a0: metamodelica::Ref<DAE::Exp>| expCref(&__a0))?;
                    headCref = expCref(metamodelica::AsArg::as_arg(&head))?;
                    tmpCrefWithEqualIdents = List::all(&crefs, &({ let __pe_b1 = headCref.clone(); move |__pe_a0| ComponentReferenceBasics::crefEqualWithoutLastSubs(&__pe_a0, &__pe_b1) }))?;
                    Ok((tmpCrefWithEqualIdents, crefs.clone(), headCref.clone(), tmpCrefWithEqualIdents.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            crefs = __wb0;
            headCref = __wb1;
            tmpCrefWithEqualIdents = __wb2;
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Nil => {
                    Ok(true)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(false)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    oCrefWithEqualIdents
}

pub(crate) fn renameExpCrefIdent(
    mut inExp: metamodelica::Ref<DAE::Exp>,
    mut inTpl: (ArcStr, ArcStr),
) -> (metamodelica::Ref<DAE::Exp>, (ArcStr, ArcStr)) {
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    let mut outTpl: (ArcStr, ArcStr);
    (outExp, outTpl) = (::match_deref::match_deref! { match &((inExp.clone(), inTpl.clone())) {
        (Deref @ DAE::Exp::CREF { componentRef: Deref @ DAE::ComponentRef::CREF_IDENT { ident: name, identType: ty1, subscriptLst: Deref @ metamodelica::ListNode::Nil }, ty: ty2 }, (from, to)) => {
            let mut exp: metamodelica::Ref<DAE::Exp>;
            exp = if (stringEq(&name, &from)) {metamodelica::Ref::new(DAE::Exp::CREF { componentRef: metamodelica::Ref::new(DAE::ComponentRef::CREF_IDENT { ident: to.clone(), identType: ty1.clone(), subscriptLst: metamodelica::nil() }), ty: ty2.clone() })} else {inExp};
            (exp, inTpl)
        },
        _ => {
            (inExp, inTpl)
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    (outExp, outTpl)
}

pub fn emptyToWild(mut exp: metamodelica::Ref<DAE::Exp>) -> Result<metamodelica::Ref<DAE::Exp>> {
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    let mut ty: metamodelica::Ref<DAE::Type>;
    ty = r#typeof(exp.clone())?;
    outExp = if (Types::isZeroLengthArray(&ty)?) {
        metamodelica::Ref::new(DAE::Exp::CREF {
            componentRef: openmodelica_frontend_types::DAE::ComponentRef::interned_WILD(),
            ty: ty,
        })
    } else {
        exp
    };
    Ok(outExp)
}

pub(crate) fn makeVectorCall(
    mut exp: metamodelica::Ref<DAE::Exp>,
    mut tp: metamodelica::Ref<DAE::Type>,
) -> metamodelica::Ref<DAE::Exp> {
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    outExp = makePureBuiltinCall(literal!("vector"), list![exp], tp);
    outExp
}

pub fn expandCrefs(
    mut inExp: metamodelica::Ref<DAE::Exp>,
    mut expandRecord: bool,
    mut dummy: i32,
) -> Result<(metamodelica::Ref<DAE::Exp>, i32)> {
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    let mut dummy: i32 = dummy;
    outExp = (::match_deref::match_deref! { match &(inExp.clone()) {
        Deref @ DAE::Exp::CREF { ty: arr_ty @ Deref @ DAE::Type::T_ARRAY { .. }, componentRef: __inExp_componentRef } => {
            let mut exp_lst: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
            let mut exp: metamodelica::Ref<DAE::Exp>;
            exp_lst = ({
        let mut __acc: metamodelica::List<metamodelica::Ref<DAE::Exp>> = metamodelica::nil();
        for mut cr in (ComponentReference::expandCref(metamodelica::AsArg::as_arg(&__inExp_componentRef), expandRecord)?).into_iter().cloned() {
            let __x = makeCrefExp(cr.clone(), var_field!((**arr_ty).ty, DAE::Type::T_ARRAY).clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
            exp = listToArray(&exp_lst, var_field!((**arr_ty).dims, DAE::Type::T_ARRAY))?;
            exp
        },
        _ => {
            inExp
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outExp, dummy))
}

pub fn expandExpression(
    mut inExp: &metamodelica::Ref<DAE::Exp>,
    mut expandRecord: bool,
) -> Result<metamodelica::List<metamodelica::Ref<DAE::Exp>>> {
    let mut outExps: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
    outExps = ({
        let mut expl: metamodelica::List<metamodelica::Ref<DAE::Exp>> = metamodelica::nil();
        (match &**inExp {
            DAE::Exp::CREF {
                componentRef: cr,
                ty: _,
            } => {
                let mut crlst: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
                crlst = ComponentReference::expandCref(cr, expandRecord)?;
                outExps = List::map(crlst, &crefToExp)?;
                outExps
            }
            DAE::Exp::UNARY {
                operator: DAE::Operator::UMINUS { .. },
                exp: __inExp_exp,
            } => {
                expl = ({
                    let mut __acc: metamodelica::List<metamodelica::Ref<DAE::Exp>> = metamodelica::nil();
                    for mut exp in (expandExpression(metamodelica::AsArg::as_arg(&__inExp_exp), expandRecord)?)
                        .into_iter()
                        .cloned()
                    {
                        let __x = metamodelica::Ref::new(DAE::Exp::UNARY {
                            operator: var_field!((**inExp).operator, DAE::Exp::UNARY).clone(),
                            exp: exp.clone(),
                        });
                        __acc = cons(__x, __acc);
                    }
                    __acc.reverse()
                });
                expl
            }
            DAE::Exp::BINARY {
                exp1: __inExp_exp1,
                exp2: __inExp_exp2,
                operator: __inExp_operator,
            } => {
                let mut expl1: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                let mut expl2: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                let mut e1: metamodelica::Ref<DAE::Exp>;
                let mut e2: metamodelica::Ref<DAE::Exp>;
                let mut op: DAE::Operator;
                op = __inExp_operator.clone();
                expl1 = expandExpression(metamodelica::AsArg::as_arg(&__inExp_exp1), expandRecord)?;
                expl2 = expandExpression(metamodelica::AsArg::as_arg(&__inExp_exp2), expandRecord)?;
                if ((expl1).len() as i32) != ((expl2).len() as i32) {
                    return Err("fail");
                }
                e1 = (expl1).get(1)?;
                e2 = (expl1).get(2)?;
                for mut i in 1..=((expl1).len() as i32) {
                    e1 = (expl1).get(i)?;
                    e2 = (expl2).get(i)?;
                    expl = metamodelica::cons(
                        metamodelica::Ref::new(DAE::Exp::BINARY {
                            exp1: e1,
                            operator: op.clone(),
                            exp2: e2,
                        }),
                        expl,
                    );
                }
                expl = expl.reverse();
                expl
            }
            DAE::Exp::ARRAY {
                ty: _,
                scalar: _,
                array: expl,
            } => {
                let mut expl = (*expl).clone();
                expl = List::mapFlat(
                    metamodelica::AsArg::as_arg(&expl),
                    &({
                        let __pe_b1 = expandRecord;
                        move |__pe_a0| expandExpression(&__pe_a0, __pe_b1.clone())
                    }),
                )?;
                expl.clone()
            }
            _ => {
                let mut msg: ArcStr;
                msg = {
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!("- Expression.expandExpression failed for "));
                    __mm_s.push_str(&*printExpStr(inExp.clone())?);
                    ArcStr::from(__mm_s)
                };
                Error::addMessage(Error::INTERNAL_ERROR.clone(), list![msg])?;
                return Err("fail");
            }
        })
    });
    Ok(outExps)
}

pub fn extendArrExp(
    mut inExp: metamodelica::Ref<DAE::Exp>,
    mut inExpanded: bool,
) -> (metamodelica::Ref<DAE::Exp>, bool) {
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    let mut outExpanded: bool;
    (outExp, outExpanded) = 'mc: {
        let __mc_input = inExp.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                outExp => {
                    let mut exp: metamodelica::Ref<DAE::Exp>;
                    let mut b: bool;
                    (exp, b) = traverseExpBottomUp(inExp.clone(), &traversingextendArrExp, false)?;
                    Ok((exp.clone(), b))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((inExp.clone(), inExpanded))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    (outExp, outExpanded)
}

fn traversingextendArrExp(
    mut inExp: metamodelica::Ref<DAE::Exp>,
    mut inExpanded: bool,
) -> Result<(metamodelica::Ref<DAE::Exp>, bool)> {
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    let mut outExpanded: bool;
    (outExp, outExpanded) = (::match_deref::match_deref! { match &(inExp.clone()) {
        Deref @ DAE::Exp::CREF { ty: ty @ Deref @ DAE::Type::T_ARRAY { dims: Deref @ metamodelica::ListNode::Cons { head: id, tail: Deref @ metamodelica::ListNode::Cons { head: jd, tail: Deref @ metamodelica::ListNode::Nil } }, .. }, .. } => {
            let mut i: i32;
            let mut j: i32;
            let mut expl: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
            let mut e: metamodelica::Ref<DAE::Exp>;
            let mut mat: metamodelica::List<metamodelica::List<metamodelica::Ref<DAE::Exp>>>;
            i = dimensionSize(metamodelica::AsArg::as_arg(&id))?;
            j = dimensionSize(metamodelica::AsArg::as_arg(&jd))?;
            expl = expandExpression(&inExp, false)?;
            mat = makeMatrix(&expl, j)?;
            e = metamodelica::Ref::new(DAE::Exp::MATRIX { ty: ty.clone(), integer: i, matrix: mat });
            (e, true)
        },
        Deref @ DAE::Exp::CREF { ty: ty @ Deref @ DAE::Type::T_ARRAY { .. }, .. } => {
            let mut expl: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
            let mut e: metamodelica::Ref<DAE::Exp>;
            expl = expandExpression(&inExp, false)?;
            e = metamodelica::Ref::new(DAE::Exp::ARRAY { ty: ty.clone(), scalar: true, array: expl });
            (e, true)
        },
        Deref @ DAE::Exp::CREF { componentRef: cr, ty: ty @ Deref @ DAE::Type::T_COMPLEX { varLst, complexClassType: ClassInf::State::RECORD { path: name }, .. } } => {
            let mut expl: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
            let mut e: metamodelica::Ref<DAE::Exp>;
            let mut field_names: metamodelica::List<ArcStr>;
            expl = List::map1(varLst.clone(), &move |__a0: metamodelica::Ref<DAE::Var>, __a1: metamodelica::Ref<DAE::ComponentRef>| generateCrefsExpFromExpVar(&__a0, &__a1), cr.clone())?;
            let true = (!((expl).is_empty())) else { return Err("pattern mismatch") };
            field_names = ({
        let mut __acc: metamodelica::List<ArcStr> = metamodelica::nil();
        for mut v in (varLst.clone()).into_iter().cloned() {
            let __x = v.name.clone();
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
            e = metamodelica::Ref::new(DAE::Exp::RECORD { path: name.clone(), exps: expl, comp: field_names, ty: ty.clone() });
            (e, _) = traverseExpBottomUp(e, &traversingextendArrExp, true)?;
            (e, true)
        },
        _ => {
            (inExp, inExpanded)
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outExp, outExpanded))
}

fn makeMatrix(
    mut expl: &metamodelica::List<metamodelica::Ref<DAE::Exp>>,
    mut n: i32,
) -> Result<metamodelica::List<metamodelica::List<metamodelica::Ref<DAE::Exp>>>> {
    let mut res: metamodelica::List<metamodelica::List<metamodelica::Ref<DAE::Exp>>>;
    let mut col: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
    let mut r: i32;
    res = metamodelica::nil();
    col = metamodelica::nil();
    r = n;
    for mut e in &**expl {
        r = r - 1;
        col = metamodelica::cons(e.clone(), col);
        if r == 0 {
            res = metamodelica::cons(col.reverse(), res);
            col = metamodelica::nil();
            r = n;
        }
    }
    Error::assertionOrAddSourceMessage(
        (col).is_empty(),
        &(Error::INTERNAL_ERROR.clone()),
        list![literal!("Expression.makeMatrix failed")],
        &(metamodelica::sourceInfo!("FrontEnd/Expression.mo")),
    )?;
    res = res.reverse();
    Ok(res)
}

pub fn rangesToSubscripts(
    mut inRangelist: &metamodelica::List<metamodelica::List<metamodelica::Ref<DAE::Subscript>>>,
) -> Result<metamodelica::List<metamodelica::List<metamodelica::Ref<DAE::Subscript>>>> {
    let mut outSubslst: metamodelica::List<metamodelica::List<metamodelica::Ref<DAE::Subscript>>>;
    outSubslst = List::allCombinations(inRangelist, None, &(Absyn::dummyInfo.clone()))?;
    Ok(outSubslst)
}

pub(crate) fn expandSubscript(
    mut inSubscript: metamodelica::Ref<DAE::Subscript>,
    mut inDimension: &metamodelica::Ref<DAE::Dimension>,
) -> Result<metamodelica::List<metamodelica::Ref<DAE::Subscript>>> {
    let mut outSubscripts: metamodelica::List<metamodelica::Ref<DAE::Subscript>>;
    outSubscripts = (::match_deref::match_deref! { match &(inSubscript.clone()) {
        Deref @ DAE::Subscript::INDEX { exp: Deref @ DAE::Exp::RANGE { .. } } => {
            ({
        let mut __acc: metamodelica::List<metamodelica::Ref<DAE::Subscript>> = metamodelica::nil();
        for mut e in (expandRange(var_field!((*inSubscript).exp, DAE::Subscript::INDEX))?).into_iter().cloned() {
            let __x = metamodelica::Ref::new(DAE::Subscript::INDEX { exp: e.clone() });
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    })
        },
        Deref @ DAE::Subscript::INDEX { exp: Deref @ DAE::Exp::ARRAY { .. } } => {
            expandSliceExp(var_field!((*inSubscript).exp, DAE::Subscript::INDEX))?
        },
        Deref @ DAE::Subscript::INDEX { .. } => {
            list![inSubscript]
        },
        Deref @ DAE::Subscript::WHOLEDIM { .. } => {
            expandDimension(inDimension)?
        },
        Deref @ DAE::Subscript::SLICE { exp: __inSubscript_exp } => {
            expandSliceExp(metamodelica::AsArg::as_arg(&__inSubscript_exp))?
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok(outSubscripts)
}

pub(crate) fn expandDimension(
    mut inDimension: &metamodelica::Ref<DAE::Dimension>,
) -> Result<metamodelica::List<metamodelica::Ref<DAE::Subscript>>> {
    let mut outSubscript: metamodelica::List<metamodelica::Ref<DAE::Subscript>>;
    outSubscript = (match &**inDimension {
        DAE::Dimension::DIM_INTEGER { integer: dim_int } => dimensionSizeSubscripts(dim_int.clone()),
        DAE::Dimension::DIM_ENUM {
            enumTypeName: enum_ty,
            literals: enum_lits,
            ..
        } => {
            let mut enum_expl: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
            enum_expl = makeEnumLiterals(enum_ty.clone(), enum_lits.clone())?;
            List::map(enum_expl, &fnptr!(makeIndexSubscript, metamodelica::Ref<DAE::Exp>))?
        }
        DAE::Dimension::DIM_BOOLEAN { .. } => metamodelica::cons(
            metamodelica::Ref::new(DAE::Subscript::INDEX {
                exp: metamodelica::Ref::new(DAE::Exp::BCONST { bool: false }),
            }),
            metamodelica::cons(
                metamodelica::Ref::new(DAE::Subscript::INDEX {
                    exp: metamodelica::Ref::new(DAE::Exp::BCONST { bool: true }),
                }),
                metamodelica::nil(),
            ),
        ),
        _ => metamodelica::nil(),
    });
    Ok(outSubscript)
}

pub(crate) fn expandSliceExp(
    mut inSliceExp: &metamodelica::Ref<DAE::Exp>,
) -> Result<metamodelica::List<metamodelica::Ref<DAE::Subscript>>> {
    let mut outSubscripts: metamodelica::List<metamodelica::Ref<DAE::Subscript>>;
    outSubscripts = (match &**inSliceExp {
        DAE::Exp::ARRAY { array: expl, .. } => {
            List::map(expl.clone(), &fnptr!(makeIndexSubscript, metamodelica::Ref<DAE::Exp>))?
        }
        DAE::Exp::RANGE { .. } => List::map(
            expandRange(inSliceExp)?,
            &fnptr!(makeIndexSubscript, metamodelica::Ref<DAE::Exp>),
        )?,
        _ => return Err("match: no arm matched"),
    });
    Ok(outSubscripts)
}

pub fn dimensionSizesSubscripts(
    mut inDimSizes: metamodelica::List<i32>,
) -> Result<metamodelica::List<metamodelica::List<metamodelica::Ref<DAE::Subscript>>>> {
    let mut outSubscripts: metamodelica::List<metamodelica::List<metamodelica::Ref<DAE::Subscript>>>;
    outSubscripts = List::map(inDimSizes, &fnptr!(dimensionSizeSubscripts, i32))?;
    Ok(outSubscripts)
}

pub fn dimensionSizesSubcriptsOpt(
    mut inDimSizes: &metamodelica::List<Option<i32>>,
) -> Result<metamodelica::List<metamodelica::List<metamodelica::Ref<DAE::Subscript>>>> {
    let mut outSubscripts: metamodelica::List<metamodelica::List<metamodelica::Ref<DAE::Subscript>>>;
    outSubscripts = List::mapOption(inDimSizes, &fnptr!(dimensionSizeSubscripts, i32))?;
    Ok(outSubscripts)
}

pub fn dimensionSizeSubscripts(mut inDimSize: i32) -> metamodelica::List<metamodelica::Ref<DAE::Subscript>> {
    let mut outSubscripts: metamodelica::List<metamodelica::Ref<DAE::Subscript>>;
    outSubscripts = ({
        let mut __acc: metamodelica::List<metamodelica::Ref<DAE::Subscript>> = metamodelica::nil();
        for mut i in (1..=inDimSize).into_iter() {
            let __x = metamodelica::Ref::new(DAE::Subscript::INDEX {
                exp: metamodelica::Ref::new(DAE::Exp::ICONST { integer: i.clone() }),
            });
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    outSubscripts
}

pub fn createResidualExp(
    mut inExp1: metamodelica::Ref<DAE::Exp>,
    mut inExp2: metamodelica::Ref<DAE::Exp>,
) -> Result<metamodelica::Ref<DAE::Exp>> {
    let mut resExp: metamodelica::Ref<DAE::Exp>;
    let mut iExp1: metamodelica::Ref<DAE::Exp>;
    let mut iExp2: metamodelica::Ref<DAE::Exp>;
    (iExp1, iExp2) = createResidualExp2(inExp1, inExp2)?;
    resExp = 'mc: {
        let __mc_input = (&*iExp1, &*iExp2);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Deref @ DAE::Exp::RCONST { real: __rlit_3 }) => {
                    if !(__rlit_3.eq(&metamodelica::OrderedFloat((0.0) as f64))) { return Err("guard") }
                    Ok(iExp1.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Deref @ DAE::Exp::ICONST { integer: 0 }) => {
                    Ok(iExp1.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::RCONST { real: __rlit_4 }, _) => {
                    if !(__rlit_4.eq(&metamodelica::OrderedFloat((0.0) as f64))) { return Err("guard") }
                    Ok(iExp2.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::ICONST { integer: 0 }, _) => {
                    Ok(iExp2.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _) => {
                    let mut res: metamodelica::Ref<DAE::Exp>;
                    let mut res1: metamodelica::Ref<DAE::Exp>;
                    let mut res2: metamodelica::Ref<DAE::Exp>;
                    let mut N1: metamodelica::Ref<DAE::Exp>;
                    let mut D1: metamodelica::Ref<DAE::Exp>;
                    let mut N2: metamodelica::Ref<DAE::Exp>;
                    let mut D2: metamodelica::Ref<DAE::Exp>;
                    let mut explst: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                    let mut explst1: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                    let mut ty: metamodelica::Ref<DAE::Type>;
                    ty = r#typeof(iExp1.clone())?;
                    let true = (Types::isIntegerOrRealOrSubTypeOfEither(ty.clone())) else { return Err("pattern mismatch") };
                    (N1, D1) = makeFraction(iExp1.clone())?;
                    (N2, D2) = makeFraction(iExp2.clone())?;
                    res1 = ExpressionSimplify::simplifySumOperatorExpression(N1.clone(), DAE::Operator::MUL { ty: ty.clone() }, D2.clone())?;
                    res2 = ExpressionSimplify::simplifySumOperatorExpression(N2.clone(), DAE::Operator::MUL { ty: ty.clone() }, D1.clone())?;
                    explst = terms(iExp1.clone())?;
                    explst1 = terms(iExp2.clone())?;
                    if isConst(res1.clone())? || ((explst1).len() as i32) + 1 > ((explst).len() as i32) {
                        res = expSub(res2.clone(), res1.clone())?;
                    } else {
                        res = expSub(res1.clone(), res2.clone())?;
                    }
                    (res, _) = ExpressionSimplify::simplify(res.clone())?;
                    Ok(res.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _) => {
                    let mut res: metamodelica::Ref<DAE::Exp>;
                    let mut ty: metamodelica::Ref<DAE::Type>;
                    ty = r#typeof(iExp1.clone())?;
                    let true = (Types::isEnumeration(&ty)) else { return Err("pattern mismatch") };
                    res = expSub(iExp1.clone(), iExp2.clone())?;
                    Ok(res.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _) => {
                    let mut res: metamodelica::Ref<DAE::Exp>;
                    let mut ty: metamodelica::Ref<DAE::Type>;
                    ty = r#typeof(iExp1.clone())?;
                    let true = (Types::isBooleanOrSubTypeBoolean(ty.clone())) else { return Err("pattern mismatch") };
                    res = metamodelica::Ref::new(DAE::Exp::LUNARY { operator: DAE::Operator::NOT { ty: ty.clone() }, exp: metamodelica::Ref::new(DAE::Exp::RELATION { exp1: iExp1.clone(), operator: DAE::Operator::EQUAL { ty: ty.clone() }, exp2: iExp2.clone(), index: -1, optionExpisASUB: None }) });
                    Ok(res.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _) => {
                    let mut res: metamodelica::Ref<DAE::Exp>;
                    let mut ty: metamodelica::Ref<DAE::Type>;
                    ty = r#typeof(iExp1.clone())?;
                    let true = (Types::isStringOrSubTypeString(ty.clone())) else { return Err("pattern mismatch") };
                    res = metamodelica::Ref::new(DAE::Exp::LUNARY { operator: DAE::Operator::NOT { ty: ty.clone() }, exp: metamodelica::Ref::new(DAE::Exp::RELATION { exp1: iExp1.clone(), operator: DAE::Operator::EQUAL { ty: ty.clone() }, exp2: iExp2.clone(), index: -1, optionExpisASUB: None }) });
                    Ok(res.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut res: metamodelica::Ref<DAE::Exp>;
                    res = expSub(iExp1.clone(), iExp2.clone())?;
                    (res, _) = ExpressionSimplify::simplify(res.clone())?;
                    Ok(res.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(resExp)
}

pub fn makeFraction(
    mut iExp: metamodelica::Ref<DAE::Exp>,
) -> Result<(metamodelica::Ref<DAE::Exp>, metamodelica::Ref<DAE::Exp>)> {
    let mut n: metamodelica::Ref<DAE::Exp>;
    let mut d: metamodelica::Ref<DAE::Exp>;
    let mut N: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
    let mut D: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
    let mut T: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
    T = terms(iExp)?;
    T = ExpressionSimplify::simplifyList(T)?;
    (N, D) = moveDivToMul(T, metamodelica::nil(), metamodelica::nil())?;
    N = ExpressionSimplify::simplifyList(N)?;
    D = ExpressionSimplify::simplifyList(D)?;
    n = makeSum1(N, false)?;
    d = makeProductLst(D)?;
    (n, _) = ExpressionSimplify::simplify1(n)?;
    (d, _) = ExpressionSimplify::simplify1(d)?;
    Ok((n, d))
}

fn moveDivToMul(
    mut iExpLst: metamodelica::List<metamodelica::Ref<DAE::Exp>>,
    mut iExpLstAcc: metamodelica::List<metamodelica::Ref<DAE::Exp>>,
    mut iExpMuls: metamodelica::List<metamodelica::Ref<DAE::Exp>>,
) -> Result<(
    metamodelica::List<metamodelica::Ref<DAE::Exp>>,
    metamodelica::List<metamodelica::Ref<DAE::Exp>>,
)> {
    '__tco: loop {
        ::match_deref::match_deref! { match &(iExpLst) {
            Deref @ metamodelica::ListNode::Nil => {
                return Ok((iExpLstAcc, iExpMuls))
            },
            Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Exp::UNARY { operator: _, exp: Deref @ DAE::Exp::BINARY { exp1: e1, operator: DAE::Operator::DIV { .. }, exp2: e2 } }, tail: rest } => {
                let mut acc: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                let mut elst: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                let mut elst1: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                let mut rest = (*rest).clone();
                acc = List::map1(iExpLstAcc, &expMul, e2.clone())?;
                rest = List::map1(rest.clone(), &expMul, e2.clone())?;
                rest = ExpressionSimplify::simplifyList(rest.clone())?;
                { (iExpLst, iExpLstAcc, iExpMuls) = (rest.clone(), metamodelica::cons(negate(e1.clone())?, acc), metamodelica::cons(e2.clone(), iExpMuls)); continue '__tco; }
            },
            Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Exp::UNARY { operator: _, exp: Deref @ DAE::Exp::BINARY { exp1: e1, operator: DAE::Operator::DIV_ARRAY_SCALAR { .. }, exp2: e2 } }, tail: rest } => {
                let mut acc: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                let mut elst: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                let mut elst1: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                let mut rest = (*rest).clone();
                acc = List::map1(iExpLstAcc, &expMul, e2.clone())?;
                rest = List::map1(rest.clone(), &expMul, e2.clone())?;
                rest = ExpressionSimplify::simplifyList(rest.clone())?;
                { (iExpLst, iExpLstAcc, iExpMuls) = (rest.clone(), metamodelica::cons(negate(e1.clone())?, acc), metamodelica::cons(e2.clone(), iExpMuls)); continue '__tco; }
            },
            Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Exp::BINARY { exp1: e1, operator: DAE::Operator::DIV { .. }, exp2: e2 }, tail: rest } => {
                let mut acc: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                let mut elst: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                let mut elst1: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                let mut rest = (*rest).clone();
                acc = List::map1(iExpLstAcc, &expMul, e2.clone())?;
                rest = List::map1(rest.clone(), &expMul, e2.clone())?;
                rest = ExpressionSimplify::simplifyList(rest.clone())?;
                { (iExpLst, iExpLstAcc, iExpMuls) = (rest.clone(), metamodelica::cons(e1.clone(), acc), metamodelica::cons(e2.clone(), iExpMuls)); continue '__tco; }
            },
            Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Exp::BINARY { exp1: e1, operator: DAE::Operator::DIV_ARRAY_SCALAR { .. }, exp2: e2 }, tail: rest } => {
                let mut acc: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                let mut elst: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                let mut elst1: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                let mut rest = (*rest).clone();
                acc = List::map1(iExpLstAcc, &expMul, e2.clone())?;
                rest = List::map1(rest.clone(), &expMul, e2.clone())?;
                rest = ExpressionSimplify::simplifyList(rest.clone())?;
                { (iExpLst, iExpLstAcc, iExpMuls) = (rest.clone(), metamodelica::cons(e1.clone(), acc), metamodelica::cons(e2.clone(), iExpMuls)); continue '__tco; }
            },
            Deref @ metamodelica::ListNode::Cons { head: e, tail: rest } => {
                let mut elst: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                let mut elst1: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                { (iExpLst, iExpLstAcc, iExpMuls) = (rest.clone(), metamodelica::cons(e.clone(), iExpLstAcc), iExpMuls); continue '__tco; }
            },
            _ => return Err("match: no arm matched"),
        } }
    }
}

fn createResidualExp2(
    mut iExp1: metamodelica::Ref<DAE::Exp>,
    mut iExp2: metamodelica::Ref<DAE::Exp>,
) -> Result<(metamodelica::Ref<DAE::Exp>, metamodelica::Ref<DAE::Exp>)> {
    let mut oExp1: metamodelica::Ref<DAE::Exp> = iExp1;
    let mut oExp2: metamodelica::Ref<DAE::Exp> = iExp2;
    let mut con: bool = true;
    let mut con1: bool;
    let mut ii: i32 = 1;
    while con && ii < 15 {
        (oExp1, oExp2, con) = 'mc: {
            let __mc_input = oExp2.clone();
            if let Ok(__v) = (|| -> Result<_> {
                ::match_deref::match_deref! { match &__mc_input {
                    _ => {
                        let mut e1: metamodelica::Ref<DAE::Exp>;
                        let mut e2: metamodelica::Ref<DAE::Exp>;
                        let (__pa0, __pa1) = ::match_deref::match_deref! { match &(createResidualExp3(oExp1.clone(), oExp2.clone())) {
                            (__pa0, __pa1, true) => (__pa0.clone(), __pa1.clone()),
                            _ => return Err("pattern mismatch"),
                        } };
                        e1 = metamodelica::Own::own(__pa0);
                        e2 = metamodelica::Own::own(__pa1);
                        (e1, _) = ExpressionSimplify::simplify1(e1.clone())?;
                        (e2, _) = ExpressionSimplify::simplify1(e2.clone())?;
                        Ok((e1.clone(), e2.clone(), true))
                    }
                    _ => return Err("nomatch"),
                }}
            })() {
                break 'mc __v;
            }
            if let Ok(__v) = (|| -> Result<_> {
                ::match_deref::match_deref! { match &__mc_input {
                    _ => {
                        let mut e1: metamodelica::Ref<DAE::Exp>;
                        let mut e2: metamodelica::Ref<DAE::Exp>;
                        let (__pa0, __pa1) = ::match_deref::match_deref! { match &(createResidualExp3(oExp2.clone(), oExp1.clone())) {
                            (__pa0, __pa1, true) => (__pa0.clone(), __pa1.clone()),
                            _ => return Err("pattern mismatch"),
                        } };
                        e2 = metamodelica::Own::own(__pa0);
                        e1 = metamodelica::Own::own(__pa1);
                        (e1, _) = ExpressionSimplify::simplify1(e1.clone())?;
                        (e2, _) = ExpressionSimplify::simplify1(e2.clone())?;
                        Ok((e1.clone(), e2.clone(), true))
                    }
                    _ => return Err("nomatch"),
                }}
            })() {
                break 'mc __v;
            }
            if let Ok(__v) = (|| -> Result<_> {
                ::match_deref::match_deref! { match &__mc_input {
                    _ => {
                        Ok((oExp1.clone(), oExp2.clone(), false))
                    }
                    _ => return Err("nomatch"),
                }}
            })() {
                break 'mc __v;
            }
            return Err("matchcontinue: no arm matched");
        };
        (oExp1, oExp2, con1) = 'mc: {
            let __mc_input = oExp2.clone();
            if let Ok(__v) = (|| -> Result<_> {
                ::match_deref::match_deref! { match &__mc_input {
                    _ => {
                        let mut e1: metamodelica::Ref<DAE::Exp>;
                        let mut e2: metamodelica::Ref<DAE::Exp>;
                        let true = (isZero(&oExp1)?) else { return Err("pattern mismatch") };
                        (e1, e2) = makeFraction(oExp2.clone())?;
                        Ok((e1.clone(), oExp1.clone(), !(isOne(&e2))))
                    }
                    _ => return Err("nomatch"),
                }}
            })() {
                break 'mc __v;
            }
            if let Ok(__v) = (|| -> Result<_> {
                ::match_deref::match_deref! { match &__mc_input {
                    _ => {
                        let mut e1: metamodelica::Ref<DAE::Exp>;
                        let mut e2: metamodelica::Ref<DAE::Exp>;
                        let true = (isZero(&oExp2)?) else { return Err("pattern mismatch") };
                        (e1, e2) = makeFraction(oExp1.clone())?;
                        Ok((e1.clone(), oExp2.clone(), !(isOne(&e2))))
                    }
                    _ => return Err("nomatch"),
                }}
            })() {
                break 'mc __v;
            }
            if let Ok(__v) = (|| -> Result<_> {
                ::match_deref::match_deref! { match &__mc_input {
                    _ => {
                        let mut e1: metamodelica::Ref<DAE::Exp>;
                        let mut e2: metamodelica::Ref<DAE::Exp>;
                        let true = (isOne(&oExp1)) else { return Err("pattern mismatch") };
                        (e1, e2) = makeFraction(oExp2.clone())?;
                        Ok((e1.clone(), e2.clone(), !(isOne(&e2))))
                    }
                    _ => return Err("nomatch"),
                }}
            })() {
                break 'mc __v;
            }
            if let Ok(__v) = (|| -> Result<_> {
                ::match_deref::match_deref! { match &__mc_input {
                    _ => {
                        let mut e1: metamodelica::Ref<DAE::Exp>;
                        let mut e2: metamodelica::Ref<DAE::Exp>;
                        let true = (isOne(&oExp2)) else { return Err("pattern mismatch") };
                        (e1, e2) = makeFraction(oExp1.clone())?;
                        Ok((e1.clone(), e2.clone(), !(isOne(&e2))))
                    }
                    _ => return Err("nomatch"),
                }}
            })() {
                break 'mc __v;
            }
            if let Ok(__v) = (|| -> Result<_> {
                ::match_deref::match_deref! { match &__mc_input {
                    _ => {
                        Ok((oExp1.clone(), oExp2.clone(), false))
                    }
                    _ => return Err("nomatch"),
                }}
            })() {
                break 'mc __v;
            }
            return Err("matchcontinue: no arm matched");
        };
        con = con || con1;
        ii = ii + 1;
        if !(con) {
            (oExp1, con) = ExpressionSimplify::simplify1(oExp1)?;
            (oExp2, con1) = ExpressionSimplify::simplify1(oExp2)?;
            con = con || con1;
            ii = ii + 3;
        }
    }
    (oExp1, _) = ExpressionSimplify::simplify1(oExp1)?;
    (oExp2, _) = ExpressionSimplify::simplify1(oExp2)?;
    Ok((oExp1, oExp2))
}

pub fn createResidualExp3(
    mut iExp1: metamodelica::Ref<DAE::Exp>,
    mut iExp2: metamodelica::Ref<DAE::Exp>,
) -> (metamodelica::Ref<DAE::Exp>, metamodelica::Ref<DAE::Exp>, bool) {
    let mut oExp1: metamodelica::Ref<DAE::Exp>;
    let mut oExp2: metamodelica::Ref<DAE::Exp>;
    let mut con: bool;
    (oExp1, oExp2, con) = 'mc: {
        let __mc_input = (&*iExp1, iExp2.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: s1 }, expLst: Deref @ metamodelica::ListNode::Cons { head: e1, tail: Deref @ metamodelica::ListNode::Nil }, .. }, Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: s2 }, expLst: Deref @ metamodelica::ListNode::Cons { head: e2, tail: Deref @ metamodelica::ListNode::Nil }, .. }) => {
                    if !((metamodelica::stringEq(&s1, &s2) && createResidualExp4(metamodelica::AsArg::as_arg(&s1)))) { return Err("guard") }
                    Ok((e1.clone(), e2.clone(), true))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "sqrt" }, expLst: Deref @ metamodelica::ListNode::Cons { head: e1, tail: Deref @ metamodelica::ListNode::Nil }, .. }, Deref @ DAE::Exp::RCONST { real: __rlit_5 }) => {
                    if !(__rlit_5.eq(&metamodelica::OrderedFloat((0.0) as f64))) { return Err("guard") }
                    Ok((e1.clone(), iExp2.clone(), true))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "sqrt" }, expLst: Deref @ metamodelica::ListNode::Cons { head: e1, tail: Deref @ metamodelica::ListNode::Nil }, .. }, e2) => {
                    if !((isConst(e2.clone())?)) { return Err("guard") }
                    let mut e: metamodelica::Ref<DAE::Exp>;
                    e = expPow(iExp2.clone(), metamodelica::Ref::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(2.0_f64) }))?;
                    Ok((e1.clone(), e.clone(), true))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "log" }, expLst: Deref @ metamodelica::ListNode::Cons { head: e1, tail: Deref @ metamodelica::ListNode::Nil }, .. }, e2) => {
                    if !((isConst(e2.clone())?)) { return Err("guard") }
                    let mut e: metamodelica::Ref<DAE::Exp>;
                    let mut tp: metamodelica::Ref<DAE::Type>;
                    tp = r#typeof(iExp2.clone())?;
                    e = makePureBuiltinCall(literal!("exp"), list![iExp2.clone()], tp.clone());
                    Ok((e1.clone(), e.clone(), true))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "log10" }, expLst: Deref @ metamodelica::ListNode::Cons { head: e1, tail: Deref @ metamodelica::ListNode::Nil }, .. }, e2) => {
                    if !((isConst(e2.clone())?)) { return Err("guard") }
                    let mut e: metamodelica::Ref<DAE::Exp>;
                    e = expPow(metamodelica::Ref::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(10.0_f64) }), iExp2.clone())?;
                    Ok((e1.clone(), e.clone(), true))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "semiLinear" }, expLst: Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Exp::RCONST { real: __rlit_6 }, tail: Deref @ metamodelica::ListNode::Cons { head: e1, tail: Deref @ metamodelica::ListNode::Cons { head: e2, tail: Deref @ metamodelica::ListNode::Nil } } }, .. }, Deref @ DAE::Exp::RCONST { real: __rlit_7 }) => {
                    if !(__rlit_6.eq(&metamodelica::OrderedFloat((0.0) as f64)) && __rlit_7.eq(&metamodelica::OrderedFloat((0.0) as f64))) { return Err("guard") }
                    Ok((e1.clone(), e2.clone(), true))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::UNARY { operator: DAE::Operator::UMINUS { .. }, exp: e1 }, e2 @ Deref @ DAE::Exp::RCONST { real: __rlit_8 }) => {
                    if !(__rlit_8.eq(&metamodelica::OrderedFloat((0.0) as f64))) { return Err("guard") }
                    Ok((e1.clone(), e2.clone(), true))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::BINARY { exp1: Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: s1 }, expLst: Deref @ metamodelica::ListNode::Cons { head: e1, tail: Deref @ metamodelica::ListNode::Nil }, .. }, operator: DAE::Operator::SUB { .. }, exp2: Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: s2 }, expLst: Deref @ metamodelica::ListNode::Cons { head: e2, tail: Deref @ metamodelica::ListNode::Nil }, .. } }, Deref @ DAE::Exp::RCONST { real: __rlit_9 }) => {
                    if !(__rlit_9.eq(&metamodelica::OrderedFloat((0.0) as f64)) && (metamodelica::stringEq(&s1, &s2) && createResidualExp4(metamodelica::AsArg::as_arg(&s1)))) { return Err("guard") }
                    Ok((e1.clone(), e2.clone(), true))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((iExp1.clone(), iExp2.clone(), false))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    (oExp1, oExp2, con)
}

fn createResidualExp4(mut f: &ArcStr) -> bool {
    let mut resB: bool;
    resB = (::match_deref::match_deref! { match &(f.clone()) {
        Deref @ "sqrt" => true,
        Deref @ "exp" => true,
        Deref @ "log" => true,
        Deref @ "log10" => true,
        Deref @ "tanh" => true,
        Deref @ "sinh" => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    resB
}

pub fn isAsubExp(mut expIn: &metamodelica::Ref<DAE::Exp>) -> bool {
    let mut isAsub: bool;
    isAsub = (match &**expIn {
        DAE::Exp::ASUB { exp: _, sub: _ } => true,
        _ => false,
    });
    isAsub
}

pub(crate) fn typeCast(
    mut inExp: metamodelica::Ref<DAE::Exp>,
    mut inType: metamodelica::Ref<DAE::Type>,
) -> Result<metamodelica::Ref<DAE::Exp>> {
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    outExp = metamodelica::Ref::new(DAE::Exp::CAST { ty: inType, exp: inExp });
    (outExp, _) = ExpressionSimplify::simplify1(outExp)?;
    Ok(outExp)
}

pub(crate) fn typeCastElements(
    mut inExp: metamodelica::Ref<DAE::Exp>,
    mut inType: &metamodelica::Ref<DAE::Type>,
) -> Result<metamodelica::Ref<DAE::Exp>> {
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    let mut ty: metamodelica::Ref<DAE::Type>;
    ty = r#typeof(inExp.clone())?;
    ty = Types::setArrayElementType(&ty, inType);
    outExp = typeCast(inExp, ty)?;
    Ok(outExp)
}

pub fn expandRange(
    mut inRange: &metamodelica::Ref<DAE::Exp>,
) -> Result<metamodelica::List<metamodelica::Ref<DAE::Exp>>> {
    let mut outValues: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
    let mut start_exp: metamodelica::Ref<DAE::Exp>;
    let mut stop_exp: metamodelica::Ref<DAE::Exp>;
    let mut ostep_exp: Option<metamodelica::Ref<DAE::Exp>>;
    let mut istep: i32;
    let mut rstep: metamodelica::Real;
    let mut vals: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
    let mut enum_names: metamodelica::List<ArcStr>;
    let mut enum_type: metamodelica::Ref<Absyn::Path>;
    let mut range_ty: metamodelica::Ref<DAE::Type>;
    let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &((*inRange)) {
        Deref @ DAE::Exp::RANGE { start: __pa0, step: __pa1, stop: __pa2, .. } => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
        _ => return Err("pattern mismatch"),
    } };
    start_exp = metamodelica::Own::own(__pa0);
    ostep_exp = metamodelica::Own::own(__pa1);
    stop_exp = metamodelica::Own::own(__pa2);
    outValues = (::match_deref::match_deref! { match &((start_exp.clone(), stop_exp.clone())) {
        (Deref @ DAE::Exp::ICONST { .. }, Deref @ DAE::Exp::ICONST { .. }) => {
            let __pa0 = ::match_deref::match_deref! { match &(Util::getOptionOrDefault(ostep_exp, metamodelica::Ref::new(DAE::Exp::ICONST { integer: 1 }))) {
                Deref @ DAE::Exp::ICONST { integer: __pa0 } => __pa0.clone(),
                _ => return Err("pattern mismatch"),
            } };
            istep = metamodelica::Own::own(__pa0);
            ({
        let mut __acc: metamodelica::List<metamodelica::Ref<DAE::Exp>> = metamodelica::nil();
        for mut i in (List::intRange3(var_field!((*start_exp).integer, DAE::Exp::ICONST).clone(), istep, var_field!((*stop_exp).integer, DAE::Exp::ICONST).clone())?).into_iter().cloned() {
            let __x = metamodelica::Ref::new(DAE::Exp::ICONST { integer: i.clone() });
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    })
        },
        (Deref @ DAE::Exp::RCONST { .. }, Deref @ DAE::Exp::RCONST { .. }) => {
            let __pa0 = ::match_deref::match_deref! { match &(Util::getOptionOrDefault(ostep_exp, metamodelica::Ref::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(1.0_f64) }))) {
                Deref @ DAE::Exp::RCONST { real: __pa0 } => __pa0.clone(),
                _ => return Err("pattern mismatch"),
            } };
            rstep = metamodelica::Own::own(__pa0);
            ({
        let mut __acc: metamodelica::List<metamodelica::Ref<DAE::Exp>> = metamodelica::nil();
        for mut r in (ExpressionSimplify::simplifyRangeReal(var_field!((*start_exp).real, DAE::Exp::RCONST).clone(), rstep, var_field!((*stop_exp).real, DAE::Exp::RCONST).clone())?).into_iter().cloned() {
            let __x = metamodelica::Ref::new(DAE::Exp::RCONST { real: r.clone() });
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    })
        },
        (Deref @ DAE::Exp::BCONST { bool: false }, Deref @ DAE::Exp::BCONST { bool: true }) => list![start_exp, stop_exp],
        (Deref @ DAE::Exp::BCONST { bool: true }, Deref @ DAE::Exp::BCONST { bool: false }) => metamodelica::nil(),
        (Deref @ DAE::Exp::BCONST { .. }, Deref @ DAE::Exp::BCONST { .. }) => list![start_exp],
        (Deref @ DAE::Exp::ENUM_LITERAL { .. }, Deref @ DAE::Exp::ENUM_LITERAL { .. }) => {
            if var_field!((*start_exp).index, DAE::Exp::ENUM_LITERAL).clone() > var_field!((*stop_exp).index, DAE::Exp::ENUM_LITERAL).clone() {
                vals = metamodelica::nil();
            } else if var_field!((*start_exp).index, DAE::Exp::ENUM_LITERAL).clone() == var_field!((*stop_exp).index, DAE::Exp::ENUM_LITERAL).clone() {
                vals = list![start_exp];
            } else {
                let __pa0 = ::match_deref::match_deref! { match &((*inRange)) {
                    Deref @ DAE::Exp::RANGE { ty: __pa0, .. } => __pa0.clone(),
                    _ => return Err("pattern mismatch"),
                } };
                range_ty = metamodelica::Own::own(__pa0);
                let (__pa1, __pa2) = ::match_deref::match_deref! { match &(Types::arrayElementType(&range_ty)) {
                    Deref @ DAE::Type::T_ENUMERATION { path: __pa1, names: __pa2, .. } => (__pa1.clone(), __pa2.clone()),
                    _ => return Err("pattern mismatch"),
                } };
                enum_type = metamodelica::Own::own(__pa1);
                enum_names = metamodelica::Own::own(__pa2);
                enum_names = List::sublist(enum_names, var_field!((*start_exp).index, DAE::Exp::ENUM_LITERAL).clone(), var_field!((*stop_exp).index, DAE::Exp::ENUM_LITERAL).clone() - var_field!((*start_exp).index, DAE::Exp::ENUM_LITERAL).clone() + 1)?;
                vals = makeEnumLiterals(enum_type, enum_names)?;
            }
            vals
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok(outValues)
}

pub(crate) fn isScalarSubscript(mut sub: &metamodelica::Ref<DAE::Subscript>) -> Result<bool> {
    let mut b: bool;
    b = (match &**sub {
        DAE::Subscript::SLICE { exp: __sub_exp } => isScalar(metamodelica::AsArg::as_arg(&__sub_exp))?,
        DAE::Subscript::INDEX { exp: __sub_exp } => isScalar(metamodelica::AsArg::as_arg(&__sub_exp))?,
        DAE::Subscript::WHOLE_NONEXP { exp: __sub_exp } => isScalar(metamodelica::AsArg::as_arg(&__sub_exp))?,
        _ => false,
    });
    Ok(b)
}

pub fn isScalar<'__b>(mut inExp: &'__b metamodelica::Ref<DAE::Exp>) -> Result<bool> {
    '__tco: loop {
        match &**inExp {
            DAE::Exp::ICONST { .. } => return Ok(true),
            DAE::Exp::RCONST { .. } => return Ok(true),
            DAE::Exp::SCONST { .. } => return Ok(true),
            DAE::Exp::BCONST { .. } => return Ok(true),
            DAE::Exp::CLKCONST { .. } => return Ok(true),
            DAE::Exp::ENUM_LITERAL { .. } => return Ok(true),
            DAE::Exp::UNARY { .. } => {
                inExp = var_field!((**inExp).exp, DAE::Exp::UNARY);
                continue '__tco;
            }
            DAE::Exp::LUNARY { .. } => {
                inExp = var_field!((**inExp).exp, DAE::Exp::LUNARY);
                continue '__tco;
            }
            DAE::Exp::RELATION { .. } => return Ok(true),
            DAE::Exp::ARRAY { .. } => return Ok(false),
            DAE::Exp::MATRIX { .. } => return Ok(false),
            DAE::Exp::RANGE { .. } => return Ok(false),
            DAE::Exp::CAST { .. } => {
                inExp = var_field!((**inExp).exp, DAE::Exp::CAST);
                continue '__tco;
            }
            DAE::Exp::SIZE { .. } => return Ok((var_field!((**inExp).sz, DAE::Exp::SIZE)).is_some()),
            _ => return Ok(Types::isSimpleType(&(r#typeof(inExp.clone())?))),
        }
    }
}

pub fn containsAnyCall(mut inExp: metamodelica::Ref<DAE::Exp>) -> Result<bool> {
    let mut outContainsCall: bool;
    (_, outContainsCall) = traverseExpTopDown(
        inExp,
        &fnptr!(containsAnyCall_traverser, metamodelica::Ref<DAE::Exp>, bool),
        false,
    )?;
    Ok(outContainsCall)
}

fn containsAnyCall_traverser(
    mut inExp: metamodelica::Ref<DAE::Exp>,
    mut inContainsCall: bool,
) -> (metamodelica::Ref<DAE::Exp>, bool, bool) {
    let mut outExp: metamodelica::Ref<DAE::Exp> = inExp.clone();
    let mut outContinue: bool;
    let mut outContainsCall: bool;
    outContainsCall = (match &*inExp {
        DAE::Exp::CALL { .. } => true,
        _ => inContainsCall,
    });
    outContinue = !(outContainsCall);
    (outExp, outContinue, outContainsCall)
}

pub(crate) fn containsCallTo(
    mut inExp: metamodelica::Ref<DAE::Exp>,
    mut path: metamodelica::Ref<Absyn::Path>,
) -> Result<bool> {
    let mut outContainsCall: bool;
    let (_, (_, __pa0)) = traverseExpTopDown(
        inExp,
        &fnptr!(
            containsCallTo_traverser,
            metamodelica::Ref<DAE::Exp>,
            (metamodelica::Ref<Absyn::Path>, bool)
        ),
        (path, false),
    )?;
    outContainsCall = metamodelica::Own::own(__pa0);
    Ok(outContainsCall)
}

fn containsCallTo_traverser(
    mut inExp: metamodelica::Ref<DAE::Exp>,
    mut inTpl: (metamodelica::Ref<Absyn::Path>, bool),
) -> (
    metamodelica::Ref<DAE::Exp>,
    bool,
    (metamodelica::Ref<Absyn::Path>, bool),
) {
    let mut outExp: metamodelica::Ref<DAE::Exp> = inExp.clone();
    let mut outContinue: bool = false;
    let mut outTpl: (metamodelica::Ref<Absyn::Path>, bool) = inTpl;
    let mut containsCall: bool;
    let mut path: metamodelica::Ref<Absyn::Path>;
    (path, containsCall) = outTpl.clone();
    if containsCall {
        return (outExp, outContinue, outTpl);
    }
    outContinue = (match &*inExp {
        DAE::Exp::CALL { path: __inExp_path, .. } => {
            AbsynUtil::pathEqual(&path, metamodelica::AsArg::as_arg(&__inExp_path))
        }
        _ => true,
    });
    if !(outContinue) {
        outTpl = (path, false);
    }
    (outExp, outContinue, outTpl)
}

pub fn rangeSize(mut inRange: &metamodelica::Ref<DAE::Exp>) -> Result<i32> {
    let mut outSize: i32;
    outSize = (::match_deref::match_deref! { match inRange {
        Deref @ DAE::Exp::RANGE { ty: Deref @ DAE::Type::T_ARRAY { dims: Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Dimension::DIM_INTEGER { integer: __esc_outSize }, tail: Deref @ metamodelica::ListNode::Nil }, .. }, .. } => {
            outSize = (*__esc_outSize).clone();
            outSize.clone()
        },
        Deref @ DAE::Exp::RANGE { start: Deref @ DAE::Exp::ICONST { integer: start }, step: None, stop: Deref @ DAE::Exp::ICONST { integer: stop }, .. } => {
            std::cmp::max(stop.clone() - start.clone(), 0)
        },
        Deref @ DAE::Exp::RANGE { start: Deref @ DAE::Exp::ICONST { integer: start }, step: Some(Deref @ DAE::Exp::ICONST { integer: step }), stop: Deref @ DAE::Exp::ICONST { integer: stop }, .. } => {
            if step.clone() != 0 {
                outSize = std::cmp::max((((realDiv(metamodelica::OrderedFloat((stop.clone() - start.clone()) as f64), metamodelica::OrderedFloat((step.clone()) as f64))).floor()).0.floor() as i32) + 1, 0);
            } else {
                return Err("fail");
            }
            outSize
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok(outSize)
}

pub fn isInvariantExpNoTraverse(
    mut e: metamodelica::Ref<DAE::Exp>,
    mut b: bool,
) -> (metamodelica::Ref<DAE::Exp>, bool) {
    let mut e: metamodelica::Ref<DAE::Exp> = e;
    let mut b: bool = b;
    if !(b) {
        return (e, b);
    }
    b = (::match_deref::match_deref! { match &(&*e) {
        Deref @ DAE::Exp::ICONST { .. } => true,
        Deref @ DAE::Exp::RCONST { .. } => true,
        Deref @ DAE::Exp::SCONST { .. } => true,
        Deref @ DAE::Exp::BCONST { .. } => true,
        Deref @ DAE::Exp::ENUM_LITERAL { .. } => true,
        Deref @ DAE::Exp::BINARY { .. } => true,
        Deref @ DAE::Exp::UNARY { .. } => true,
        Deref @ DAE::Exp::LBINARY { .. } => true,
        Deref @ DAE::Exp::LUNARY { .. } => true,
        Deref @ DAE::Exp::RELATION { .. } => true,
        Deref @ DAE::Exp::IFEXP { .. } => true,
        Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::FULLYQUALIFIED { .. }, .. } => true,
        Deref @ DAE::Exp::PARTEVALFUNCTION { path: Deref @ Absyn::Path::FULLYQUALIFIED { .. }, .. } => true,
        Deref @ DAE::Exp::ARRAY { .. } => true,
        Deref @ DAE::Exp::MATRIX { .. } => true,
        Deref @ DAE::Exp::RANGE { .. } => true,
        Deref @ DAE::Exp::CONS { .. } => true,
        Deref @ DAE::Exp::LIST { .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    (e, b)
}

pub fn findCallIsInlineAfterIndexReduction(
    mut e: metamodelica::Ref<DAE::Exp>,
    mut res: bool,
) -> (metamodelica::Ref<DAE::Exp>, bool, bool) {
    let mut e: metamodelica::Ref<DAE::Exp> = e;
    let mut cont: bool;
    let mut res: bool = res;
    if !(res) {
        res = (::match_deref::match_deref! { match &(&*e) {
            Deref @ DAE::Exp::CALL { attr: Deref @ DAE::CallAttributes { inlineType: DAE::InlineType::AFTER_INDEX_RED_INLINE { .. }, .. }, .. } => true,
            _ => false,
            _ => unreachable!("match_deref! exhaustiveness placeholder"),
        } });
    }
    cont = !(res);
    (e, cont, res)
}

pub fn tupleHead(
    mut exp: metamodelica::Ref<DAE::Exp>,
    mut prop: DAE::Properties,
) -> Result<(metamodelica::Ref<DAE::Exp>, DAE::Properties)> {
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    let mut outProp: DAE::Properties;
    (outExp, outProp) = (::match_deref::match_deref! { match &((exp.clone(), prop.clone())) {
        (Deref @ DAE::Exp::TUPLE { PR: Deref @ metamodelica::ListNode::Cons { head: _, tail: _ } }, DAE::Properties::PROP_TUPLE { .. }) => {
            ((var_field!((*exp).PR, DAE::Exp::TUPLE)).head().cloned()?, Types::propTupleFirstProp(prop)?)
        },
        (_, DAE::Properties::PROP_TUPLE { type_: Deref @ DAE::Type::T_TUPLE { types: Deref @ metamodelica::ListNode::Cons { head: ty, tail: _ }, .. }, .. }) => {
            (metamodelica::Ref::new(DAE::Exp::TSUB { exp: exp, ix: 1, ty: ty.clone() }), Types::propTupleFirstProp(prop)?)
        },
        _ => {
            (exp, prop)
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outExp, outProp))
}

pub fn isSimpleLiteralValue(mut exp: &metamodelica::Ref<DAE::Exp>, mut allow_arrays: bool) -> Result<bool> {
    let mut b: bool;
    b = (match &**exp {
        DAE::Exp::SCONST { .. } => allow_arrays,
        DAE::Exp::ICONST { .. } => true,
        DAE::Exp::RCONST { .. } => true,
        DAE::Exp::BCONST { .. } => true,
        DAE::Exp::ENUM_LITERAL { .. } => true,
        DAE::Exp::ARRAY { array: __exp_array, .. } if (allow_arrays) => List::all(
            metamodelica::AsArg::as_arg(&__exp_array),
            &({
                let __pe_b1 = true;
                move |__pe_a0| isSimpleLiteralValue(&__pe_a0, __pe_b1.clone())
            }),
        )?,
        _ => false,
    });
    Ok(b)
}

pub fn consToListIgnoreSharedLiteral(mut e: metamodelica::Ref<DAE::Exp>) -> metamodelica::Ref<DAE::Exp> {
    let mut e: metamodelica::Ref<DAE::Exp> = e;
    if (match &*e {
        DAE::Exp::SHARED_LITERAL { .. } => true,
        DAE::Exp::LIST { .. } => true,
        DAE::Exp::CONS { .. } => true,
        _ => false,
    }) {
        if '__try0: {
            e = unwrap_break_err!(consToListIgnoreSharedLiteralWork(e.clone(), metamodelica::nil()), '__try0);
            Ok::<(), &'static str>(())
        }
        .is_err()
        {}
    }
    e
}

fn consToListIgnoreSharedLiteralWork(
    mut e: metamodelica::Ref<DAE::Exp>,
    mut acc: metamodelica::List<metamodelica::Ref<DAE::Exp>>,
) -> Result<metamodelica::Ref<DAE::Exp>> {
    '__tco: loop {
        ::match_deref::match_deref! { match &((e.clone(), acc.clone())) {
            (Deref @ DAE::Exp::SHARED_LITERAL { .. }, _) => { (e, acc) = (var_field!((*e).exp, DAE::Exp::SHARED_LITERAL).clone(), acc); continue '__tco; },
            (Deref @ DAE::Exp::LIST { .. }, Deref @ metamodelica::ListNode::Nil) => return Ok(e),
            (Deref @ DAE::Exp::LIST { .. }, _) => return Ok(metamodelica::Ref::new(DAE::Exp::LIST { valList: List::append_reverse(&acc, var_field!((*e).valList, DAE::Exp::LIST).clone()) })),
            (Deref @ DAE::Exp::CONS { .. }, _) => { (e, acc) = (var_field!((*e).cdr, DAE::Exp::CONS).clone(), metamodelica::cons(var_field!((*e).car, DAE::Exp::CONS).clone(), acc)); continue '__tco; },
            _ => return Err("match: no arm matched"),
        } }
    }
}

pub fn arrayFirstScalar(mut exp: metamodelica::Ref<DAE::Exp>) -> Result<metamodelica::Ref<DAE::Exp>> {
    '__tco: loop {
        match &*exp {
            DAE::Exp::ARRAY { array: __exp_array, .. } => {
                exp = (__exp_array).head().cloned()?;
                continue '__tco;
            }
            _ => return Ok(exp),
        }
    }
}

pub fn traverseCases<A: Clone + 'static + metamodelica::gc::MMTrace + metamodelica::ReferenceEq>(
    mut inCases: &metamodelica::List<metamodelica::Ref<DAE::MatchCase>>,
    mut func: Arc<
        dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>, A) -> Result<(metamodelica::Ref<DAE::Exp>, A)> + 'static,
    >,
    mut inA: A,
) -> Result<(metamodelica::List<metamodelica::Ref<DAE::MatchCase>>, A)> {
    pub type FuncExpType<A: Clone + 'static> = std::sync::Arc<
        dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>, A) -> Result<(metamodelica::Ref<DAE::Exp>, A)> + 'static,
    >;

    let mut outCases: metamodelica::List<metamodelica::Ref<DAE::MatchCase>>;
    let mut oa: A;
    (outCases, oa) = (::match_deref::match_deref! { match inCases {
        Deref @ metamodelica::ListNode::Nil => {
            let mut a = inA;
            (metamodelica::nil(), a)
        },
        Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::MatchCase { patterns, patternGuard, localDecls: decls, body, result, resultInfo, jump, info }, tail: cases } => {
            let mut a = inA;
            let mut body1: metamodelica::List<metamodelica::Ref<DAE::Statement>>;
            let mut result1: Option<metamodelica::Ref<DAE::Exp>>;
            let mut patternGuard1: Option<metamodelica::Ref<DAE::Exp>>;
            let mut cases1: metamodelica::List<metamodelica::Ref<DAE::MatchCase>>;
            let mut cases = (*cases).clone();
            let (__pa0, (_, __pa1)) = DAEUtil::traverseDAEEquationsStmts(body.clone(), (std::sync::Arc::new(traverseSubexpressionsHelper) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>, _) -> Result<_> + 'static>), (func.clone(), a))?;
            body1 = metamodelica::Own::own(__pa0);
            a = metamodelica::Own::own(__pa1);
            (patternGuard1, a) = traverseExpOpt(patternGuard.clone(), &*func, a)?;
            (result1, a) = traverseExpOpt(result.clone(), &*func, a)?;
            (cases1, a) = traverseCases(metamodelica::AsArg::as_arg(&cases), func.clone(), a)?;
            cases = if (metamodelica::ReferenceEq::reference_eq(&(cases.clone()), &(cases1)) && (match (&(patternGuard), &(patternGuard1)) { (None, None) => true, (Some(__refeq_l), Some(__refeq_r)) => referenceEq(&*(*__refeq_l),&*(*__refeq_r)), _ => false }) && (match (&(result), &(result1)) { (None, None) => true, (Some(__refeq_l), Some(__refeq_r)) => referenceEq(&*(*__refeq_l),&*(*__refeq_r)), _ => false }) && metamodelica::ReferenceEq::reference_eq(&(body.clone()), &(body1))) {inCases.clone()} else {metamodelica::cons(metamodelica::Ref::new(DAE::MatchCase { patterns: patterns.clone(), patternGuard: patternGuard1, localDecls: decls.clone(), body: body1, result: result1, resultInfo: resultInfo.clone(), jump: jump.clone(), info: info.clone() }), cases1)};
            (cases.clone(), a)
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outCases, oa))
}

fn traverseMatchCases<A: Clone + 'static + metamodelica::gc::MMTrace>(
    mut inCases: metamodelica::List<metamodelica::Ref<DAE::MatchCase>>,
    mut func: &dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>, A) -> Result<(metamodelica::Ref<DAE::Exp>, A)>,
    mut inA: A,
) -> (metamodelica::List<metamodelica::Ref<DAE::MatchCase>>, A) {
    pub type FuncExpType<A: Clone + 'static> = std::sync::Arc<
        dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>, A) -> Result<(metamodelica::Ref<DAE::Exp>, A)> + 'static,
    >;

    let mut outCases: metamodelica::List<metamodelica::Ref<DAE::MatchCase>> = inCases;
    let mut oa: A = inA;
    (outCases, oa)
}

fn traverseMatchCasesTopDown<A: Clone + 'static + metamodelica::gc::MMTrace>(
    mut inCases: metamodelica::List<metamodelica::Ref<DAE::MatchCase>>,
    mut func: &dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>, A) -> Result<(metamodelica::Ref<DAE::Exp>, bool, A)>,
    mut inA: A,
) -> (metamodelica::List<metamodelica::Ref<DAE::MatchCase>>, A) {
    pub type FuncExpType<A: Clone + 'static> = std::sync::Arc<
        dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>, A) -> Result<(metamodelica::Ref<DAE::Exp>, bool, A)> + 'static,
    >;

    let mut cases: metamodelica::List<metamodelica::Ref<DAE::MatchCase>> = inCases;
    let mut a: A = inA;
    (cases, a)
}

pub(crate) fn traverseCasesTopDown<A: Clone + 'static + metamodelica::gc::MMTrace + metamodelica::ReferenceEq>(
    mut inCases: &metamodelica::List<metamodelica::Ref<DAE::MatchCase>>,
    mut func: Arc<
        dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>, A) -> Result<(metamodelica::Ref<DAE::Exp>, bool, A)> + 'static,
    >,
    mut inA: A,
) -> Result<(metamodelica::List<metamodelica::Ref<DAE::MatchCase>>, A)> {
    pub type FuncExpType<A: Clone + 'static> = std::sync::Arc<
        dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>, A) -> Result<(metamodelica::Ref<DAE::Exp>, bool, A)> + 'static,
    >;

    let mut cases: metamodelica::List<metamodelica::Ref<DAE::MatchCase>> = metamodelica::nil();
    let mut a: A = inA;
    let mut patterns: metamodelica::List<metamodelica::Ref<DAE::Pattern>>;
    let mut decls: metamodelica::List<metamodelica::Ref<DAE::Element>>;
    let mut body: metamodelica::List<metamodelica::Ref<DAE::Statement>>;
    let mut body1: metamodelica::List<metamodelica::Ref<DAE::Statement>>;
    let mut result: Option<metamodelica::Ref<DAE::Exp>>;
    let mut result1: Option<metamodelica::Ref<DAE::Exp>>;
    let mut patternGuard: Option<metamodelica::Ref<DAE::Exp>>;
    let mut patternGuard1: Option<metamodelica::Ref<DAE::Exp>>;
    let mut jump: i32;
    let mut resultInfo: SourceInfo;
    let mut info: SourceInfo;
    let mut tpl: (
        Arc<
            dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>, A) -> Result<(metamodelica::Ref<DAE::Exp>, bool, A)>
                + 'static,
        >,
        A,
    );
    for mut c in &**inCases {
        let __arc8 = c.clone();
        let DAE::CASE {
            patterns: __pa0,
            patternGuard: __pa1,
            localDecls: __pa2,
            body: __pa3,
            result: __pa4,
            resultInfo: __pa5,
            jump: __pa6,
            info: __pa7,
        } = &*__arc8;
        patterns = metamodelica::Own::own(__pa0);
        patternGuard = metamodelica::Own::own(__pa1);
        decls = metamodelica::Own::own(__pa2);
        body = metamodelica::Own::own(__pa3);
        result = metamodelica::Own::own(__pa4);
        resultInfo = metamodelica::Own::own(__pa5);
        jump = metamodelica::Own::own(__pa6);
        info = metamodelica::Own::own(__pa7);
        tpl = (func.clone(), a);
        let (__pa9, (_, __pa10)) = DAEUtil::traverseDAEEquationsStmts(
            body,
            (std::sync::Arc::new(traverseSubexpressionsTopDownHelper)
                as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>, _) -> Result<_> + 'static>),
            tpl,
        )?;
        body1 = metamodelica::Own::own(__pa9);
        a = metamodelica::Own::own(__pa10);
        (patternGuard1, a) = traverseExpOptTopDown(patternGuard, &*func, a)?;
        (result1, a) = traverseExpOptTopDown(result, &*func, a)?;
        cases = metamodelica::cons(
            metamodelica::Ref::new(DAE::MatchCase {
                patterns: patterns,
                patternGuard: patternGuard1,
                localDecls: decls,
                body: body1,
                result: result1,
                resultInfo: resultInfo,
                jump: jump,
                info: info,
            }),
            cases,
        );
    }
    cases = cases.reverse();
    Ok((cases, a))
}

// A match expression only occurs in MetaModelica code. Its cases are not
// traversed: they would need the borrowed callback as an owned value.
