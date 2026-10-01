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
use crate::Expression;
use crate::Types;
use openmodelica_ast::Absyn;
use openmodelica_frontend_dump::ComponentReferenceBasics;
use openmodelica_frontend_dump::ElementSource;
use openmodelica_frontend_dump::ExpressionBasics;
use openmodelica_frontend_dump::SCodeDump;
use openmodelica_frontend_dump::SCodeUtil;
use openmodelica_frontend_dump::TypesDump;
use openmodelica_frontend_types::DAE;
use openmodelica_frontend_types::SCode;
use openmodelica_util::Debug;
use openmodelica_util::Error;
use openmodelica_util::Flags;
use openmodelica_util::Util;
use openmodelica_util_datatypes_basic::List;

pub(crate) fn algorithmEmpty(mut alg: &metamodelica::Ref<DAE::Algorithm>) -> bool {
    let mut empty: bool;
    empty = (::match_deref::match_deref! { match alg {
        Deref @ DAE::Algorithm { statementLst: Deref @ metamodelica::ListNode::Nil } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    empty
}

pub(crate) fn isReinitStatement(mut stmt: &metamodelica::Ref<DAE::Statement>) -> bool {
    let mut res: bool;
    res = (match &**stmt {
        DAE::Statement::STMT_REINIT { .. } => true,
        _ => false,
    });
    res
}

pub(crate) fn isNotAssertStatement(mut stmt: &metamodelica::Ref<DAE::Statement>) -> bool {
    let mut res: bool;
    res = (match &**stmt {
        DAE::Statement::STMT_ASSERT { .. } => false,
        _ => true,
    });
    res
}

pub fn makeAssignmentNoTypeCheck(
    mut ty: metamodelica::Ref<DAE::Type>,
    mut lhs: metamodelica::Ref<DAE::Exp>,
    mut rhs: metamodelica::Ref<DAE::Exp>,
    mut source: metamodelica::Ref<DAE::ElementSource>,
) -> metamodelica::Ref<DAE::Statement> {
    let mut outStatement: metamodelica::Ref<DAE::Statement>;
    outStatement = (::match_deref::match_deref! { match &(lhs.clone()) {
        Deref @ DAE::Exp::CREF { componentRef: Deref @ DAE::ComponentRef::WILD { .. }, .. } => metamodelica::Ref::new(DAE::Statement::STMT_NORETCALL { exp: rhs, source: source }),
        Deref @ DAE::Exp::PATTERN { pattern: Deref @ DAE::Pattern::PAT_WILD { .. } } => metamodelica::Ref::new(DAE::Statement::STMT_NORETCALL { exp: rhs, source: source }),
        _ => metamodelica::Ref::new(DAE::Statement::STMT_ASSIGN { type_: ty, exp1: lhs, exp: rhs, source: source }),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outStatement
}

pub fn makeArrayAssignmentNoTypeCheck(
    mut ty: metamodelica::Ref<DAE::Type>,
    mut lhs: metamodelica::Ref<DAE::Exp>,
    mut rhs: metamodelica::Ref<DAE::Exp>,
    mut source: metamodelica::Ref<DAE::ElementSource>,
) -> metamodelica::Ref<DAE::Statement> {
    let mut outStatement: metamodelica::Ref<DAE::Statement>;
    outStatement = (::match_deref::match_deref! { match &(lhs.clone()) {
        Deref @ DAE::Exp::CREF { componentRef: Deref @ DAE::ComponentRef::WILD { .. }, .. } => metamodelica::Ref::new(DAE::Statement::STMT_NORETCALL { exp: rhs, source: source }),
        _ => metamodelica::Ref::new(DAE::Statement::STMT_ASSIGN_ARR { type_: ty, lhs: lhs, exp: rhs, source: source }),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outStatement
}

pub fn makeTupleAssignmentNoTypeCheck(
    mut ty: metamodelica::Ref<DAE::Type>,
    mut lhs: metamodelica::List<metamodelica::Ref<DAE::Exp>>,
    mut rhs: metamodelica::Ref<DAE::Exp>,
    mut source: metamodelica::Ref<DAE::ElementSource>,
) -> Result<metamodelica::Ref<DAE::Statement>> {
    let mut outStatement: metamodelica::Ref<DAE::Statement>;
    let mut b1: bool;
    let mut b2: bool;
    b1 = List::all(
        &lhs,
        &move |__a0: metamodelica::Ref<DAE::Exp>| -> metamodelica::Result<_> {
            ::std::result::Result::Ok(Expression::isWild(&__a0))
        },
    )?;
    b2 = List::all(&(List::restOrEmpty(lhs.clone())?), &move |__a0: metamodelica::Ref<
        DAE::Exp,
    >|
          -> metamodelica::Result<
        _,
    > {
        ::std::result::Result::Ok(Expression::isWild(&__a0))
    })?;
    outStatement = makeTupleAssignmentNoTypeCheck2(b1, b2, ty, lhs, rhs, source);
    Ok(outStatement)
}

fn makeTupleAssignmentNoTypeCheck2(
    mut allWild: bool,
    mut singleAssign: bool,
    mut ty: metamodelica::Ref<DAE::Type>,
    mut lhs: metamodelica::List<metamodelica::Ref<DAE::Exp>>,
    mut rhs: metamodelica::Ref<DAE::Exp>,
    mut source: metamodelica::Ref<DAE::ElementSource>,
) -> metamodelica::Ref<DAE::Statement> {
    let mut outStatement: metamodelica::Ref<DAE::Statement>;
    outStatement = (::match_deref::match_deref! { match &((allWild, singleAssign, ty.clone(), lhs.clone())) {
        (true, _, _, _) => {
            metamodelica::Ref::new(DAE::Statement::STMT_NORETCALL { exp: rhs, source: source })
        },
        (_, true, Deref @ DAE::Type::T_TUPLE { types: Deref @ metamodelica::ListNode::Cons { head: ty1 @ Deref @ DAE::Type::T_ARRAY { .. }, tail: _ }, .. }, Deref @ metamodelica::ListNode::Cons { head: lhs1, tail: _ }) => {
            metamodelica::Ref::new(DAE::Statement::STMT_ASSIGN_ARR { type_: ty1.clone(), lhs: lhs1.clone(), exp: metamodelica::Ref::new(DAE::Exp::TSUB { exp: rhs, ix: 1, ty: ty1.clone() }), source: source })
        },
        (_, true, Deref @ DAE::Type::T_TUPLE { types: Deref @ metamodelica::ListNode::Cons { head: ty1, tail: _ }, .. }, Deref @ metamodelica::ListNode::Cons { head: lhs1, tail: _ }) => {
            metamodelica::Ref::new(DAE::Statement::STMT_ASSIGN { type_: ty1.clone(), exp1: lhs1.clone(), exp: metamodelica::Ref::new(DAE::Exp::TSUB { exp: rhs, ix: 1, ty: ty1.clone() }), source: source })
        },
        _ => {
            metamodelica::Ref::new(DAE::Statement::STMT_TUPLE_ASSIGN { type_: ty, expExpLst: lhs, exp: rhs, source: source })
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outStatement
}

pub fn makeAssignment(
    mut inExp1: metamodelica::Ref<DAE::Exp>,
    mut inProperties2: DAE::Properties,
    mut inExp3: metamodelica::Ref<DAE::Exp>,
    mut inProperties4: DAE::Properties,
    mut inAttributes: &metamodelica::Ref<DAE::Attributes>,
    mut initial_: SCode::Initial,
    mut source: metamodelica::Ref<DAE::ElementSource>,
) -> Result<metamodelica::Ref<DAE::Statement>> {
    let mut outStatement: metamodelica::Ref<DAE::Statement> =
        <metamodelica::Ref<DAE::Statement> as ::std::default::Default>::default();
    outStatement = 'mc: {
        let __mc_input = (inExp1, inProperties2, inExp3, inProperties4, &**inAttributes, initial_);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CREF { componentRef: Deref @ DAE::ComponentRef::WILD { .. }, .. }, _, rhs, _, _, _) => {
                    Ok(metamodelica::Ref::new(DAE::Statement::STMT_NORETCALL { exp: rhs.clone(), source: source.clone() }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (lhs @ Deref @ DAE::Exp::CREF { componentRef: cr, .. }, lhprop, rhs, rhprop, _, SCode::Initial::NON_INITIAL { .. }) => {
                    let mut outStatement: metamodelica::Ref<DAE::Statement> = outStatement.clone();
                    let DAE::C_PARAM { .. } = (Types::propAnyConst(lhprop.clone())?) else { return Err("pattern mismatch") };
                    let true = (ComponentReference::isRecord(metamodelica::AsArg::as_arg(&cr))) else { return Err("pattern mismatch") };
                    outStatement = makeAssignment2(lhs.clone(), metamodelica::AsArg::as_arg(&lhprop), rhs.clone(), metamodelica::AsArg::as_arg(&rhprop), source.clone())?;
                    Ok((outStatement.clone(), outStatement.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            outStatement = __wb0;
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (lhs, lprop, rhs, _, _, SCode::Initial::NON_INITIAL { .. }) => {
                    let mut lhs_str: ArcStr;
                    let mut rhs_str: ArcStr;
                    let DAE::C_PARAM { .. } = (Types::propAnyConst(lprop.clone())?) else { return Err("pattern mismatch") };
                    lhs_str = ExpressionBasics::printExpStr(lhs.clone())?;
                    rhs_str = ExpressionBasics::printExpStr(rhs.clone())?;
                    Error::addSourceMessage(&(Error::ASSIGN_PARAM_ERROR.clone()), list![lhs_str.clone(), rhs_str.clone()], &(ElementSource::getElementSourceFileInfo(source.clone())))?;
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (lhs, _, _, _, Deref @ DAE::Attributes { variability: SCode::Variability::CONST { .. }, .. }, _) => {
                    let mut lhs_str: ArcStr;
                    lhs_str = ExpressionBasics::printExpStr(lhs.clone())?;
                    Error::addSourceMessage(&(Error::ASSIGN_READONLY_ERROR.clone()), list![literal!("constant"), lhs_str.clone()], &(ElementSource::getElementSourceFileInfo(source.clone())))?;
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (lhs, lhprop, rhs, rhprop, _, SCode::Initial::INITIAL { .. }) => {
                    let mut outStatement: metamodelica::Ref<DAE::Statement> = outStatement.clone();
                    let DAE::C_PARAM { .. } = (Types::propAnyConst(lhprop.clone())?) else { return Err("pattern mismatch") };
                    outStatement = makeAssignment2(lhs.clone(), metamodelica::AsArg::as_arg(&lhprop), rhs.clone(), metamodelica::AsArg::as_arg(&rhprop), source.clone())?;
                    Ok((outStatement.clone(), outStatement.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            outStatement = __wb0;
            break 'mc __v;
        }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (lhs, lhprop, rhs, rhprop, Deref @ DAE::Attributes { .. }, _) => {
                    let mut outStatement: metamodelica::Ref<DAE::Statement> = outStatement.clone();
                    let DAE::C_VAR { .. } = (Types::propAnyConst(lhprop.clone())?) else { return Err("pattern mismatch") };
                    outStatement = makeAssignment2(lhs.clone(), metamodelica::AsArg::as_arg(&lhprop), rhs.clone(), metamodelica::AsArg::as_arg(&rhprop), source.clone())?;
                    Ok((outStatement.clone(), outStatement.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            outStatement = __wb0;
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (lhs, lprop, rhs, rprop, _, _) => {
                    let mut lhs_str: ArcStr;
                    let mut rhs_str: ArcStr;
                    let mut lt_str: ArcStr;
                    let mut rt_str: ArcStr;
                    let mut lt: metamodelica::Ref<DAE::Type>;
                    let mut rt: metamodelica::Ref<DAE::Type>;
                    let mut info: SourceInfo;
                    lt = Types::getPropType(metamodelica::AsArg::as_arg(&lprop));
                    rt = Types::getPropType(metamodelica::AsArg::as_arg(&rprop));
                    let false = (Types::equivtypes(lt.clone(), rt.clone())) else { return Err("pattern mismatch") };
                    lhs_str = ExpressionBasics::printExpStr(lhs.clone())?;
                    rhs_str = ExpressionBasics::printExpStr(rhs.clone())?;
                    lt_str = TypesDump::unparseTypeNoAttr(&lt)?;
                    rt_str = TypesDump::unparseTypeNoAttr(&rt)?;
                    info = ElementSource::getElementSourceFileInfo(source.clone());
                    Types::typeErrorSanityCheck(lt_str.clone(), &rt_str, &info)?;
                    Error::addSourceMessage(&(Error::ASSIGN_TYPE_MISMATCH_ERROR.clone()), list![lhs_str.clone(), rhs_str.clone(), lt_str.clone(), rt_str.clone()], &info)?;
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (lhs, _, rhs, _, _, _) => {
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { return Err("pattern mismatch") };
                    Debug::traceln(literal!("- Algorithm.makeAssignment failed"))?;
                    Debug::trace(literal!("    "))?;
                    Debug::trace(ExpressionBasics::printExpStr(lhs.clone())?)?;
                    Debug::trace(literal!(" := "))?;
                    Debug::traceln(ExpressionBasics::printExpStr(rhs.clone())?)?;
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(outStatement)
}

fn makeAssignment2(
    mut lhs: metamodelica::Ref<DAE::Exp>,
    mut lhprop: &DAE::Properties,
    mut rhs: metamodelica::Ref<DAE::Exp>,
    mut rhprop: &DAE::Properties,
    mut source: metamodelica::Ref<DAE::ElementSource>,
) -> Result<metamodelica::Ref<DAE::Statement>> {
    let mut outStatement: metamodelica::Ref<DAE::Statement>;
    outStatement = (::match_deref::match_deref! { match &(lhs.clone()) {
        Deref @ DAE::Exp::CREF { .. } if (!(Types::isPropArray(lhprop))) => {
            let mut rhs_1: metamodelica::Ref<DAE::Exp>;
            let mut e1: metamodelica::Ref<DAE::Exp>;
            let mut t: metamodelica::Ref<DAE::Type>;
            (rhs_1, _) = Types::matchProp(rhs, rhprop, lhprop, true)?;
            t = getPropExpType(lhprop)?;
            let () = (::match_deref::match_deref! { match &(&*rhs_1) {
        Deref @ DAE::Exp::CALL { attr: Deref @ DAE::CallAttributes { builtin: true, .. }, path: Deref @ Absyn::Path::IDENT { name: Deref @ "listAppend" }, expLst: Deref @ metamodelica::ListNode::Cons { head: e1 @ Deref @ DAE::Exp::CREF { .. }, tail: _ } } if (ExpressionBasics::expEqual(&lhs, e1.clone())?) => {
            if Flags::isSet(Flags::LIST_REVERSE_WRONG_ORDER.clone())? && !(({
        let mut __acc: Option<bool> = None;
        for mut comment in (ElementSource::getComments(&source)).into_iter().cloned() {
            let __x = SCodeUtil::commentHasBooleanNamedAnnotation(&(comment.clone()), &(literal!("__OpenModelica_DisableListAppendWarning")));
            __acc = Some(match __acc { None => __x, Some(__cur) => if __x > __cur { __x } else { __cur } });
        }
        __acc.unwrap_or(false)
    })) {
                Error::addSourceMessage(&(Error::LIST_REVERSE_WRONG_ORDER.clone()), list![ExpressionBasics::printExpStr(e1.clone())?], &(ElementSource::getElementSourceFileInfo(source.clone())))?;
                return Err("fail");
            }
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
            metamodelica::Ref::new(DAE::Statement::STMT_ASSIGN { type_: t, exp1: lhs, exp: rhs_1, source: source })
        },
        Deref @ DAE::Exp::CREF { .. } => {
            let mut rhs_1: metamodelica::Ref<DAE::Exp>;
            let mut t: metamodelica::Ref<DAE::Type>;
            let mut ty: metamodelica::Ref<DAE::Type>;
            (rhs_1, _) = Types::matchProp(rhs, rhprop, lhprop, false)?;
            ty = Types::getPropType(lhprop);
            t = Types::simplifyType(ty)?;
            metamodelica::Ref::new(DAE::Statement::STMT_ASSIGN_ARR { type_: t, lhs: lhs, exp: rhs_1, source: source })
        },
        e3 @ Deref @ DAE::Exp::ASUB { exp: _, sub: _ } => {
            let mut rhs_1: metamodelica::Ref<DAE::Exp>;
            let mut t: metamodelica::Ref<DAE::Type>;
            (rhs_1, _) = Types::matchProp(rhs, rhprop, lhprop, true)?;
            t = getPropExpType(lhprop)?;
            metamodelica::Ref::new(DAE::Statement::STMT_ASSIGN { type_: t, exp1: e3.clone(), exp: rhs_1, source: source })
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok(outStatement)
}

pub(crate) fn makeSimpleAssignment(
    mut inTpl: &(metamodelica::Ref<DAE::Exp>, metamodelica::Ref<DAE::Exp>),
    mut source: metamodelica::Ref<DAE::ElementSource>,
) -> Result<metamodelica::Ref<DAE::Statement>> {
    let mut outStmt: metamodelica::Ref<DAE::Statement>;
    let mut e1: metamodelica::Ref<DAE::Exp>;
    let mut e2: metamodelica::Ref<DAE::Exp>;
    let mut tp: metamodelica::Ref<DAE::Type>;
    let (__pa1, __pa0, __pa2) = ::match_deref::match_deref! { match inTpl {
        (__pa1 @ Deref @ DAE::Exp::CREF { ty: __pa0, .. }, __pa2) => (__pa1.clone(), __pa0.clone(), __pa2.clone()),
        _ => return Err("pattern mismatch"),
    } };
    tp = metamodelica::Own::own(__pa0);
    e1 = metamodelica::Own::own(__pa1);
    e2 = metamodelica::Own::own(__pa2);
    outStmt = metamodelica::Ref::new(DAE::Statement::STMT_ASSIGN {
        type_: tp,
        exp1: e1,
        exp: e2,
        source: source,
    });
    Ok(outStmt)
}

pub fn makeAssignmentsList<'__b>(
    mut lhsExps: &'__b metamodelica::List<metamodelica::Ref<DAE::Exp>>,
    mut lhsProps: &'__b metamodelica::List<DAE::Properties>,
    mut rhsExps: &'__b metamodelica::List<metamodelica::Ref<DAE::Exp>>,
    mut rhsProps: &'__b metamodelica::List<DAE::Properties>,
    mut attributes: &'__b metamodelica::Ref<DAE::Attributes>,
    mut initial_: SCode::Initial,
    mut source: &'__b metamodelica::Ref<DAE::ElementSource>,
) -> Result<metamodelica::List<metamodelica::Ref<DAE::Statement>>> {
    '__tco: loop {
        ::match_deref::match_deref! { match (lhsExps, lhsProps, rhsExps, rhsProps) {
            (Deref @ metamodelica::ListNode::Nil, Deref @ metamodelica::ListNode::Nil, _, _) => {
                return Ok(metamodelica::nil())
            },
            (Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Exp::CREF { componentRef: Deref @ DAE::ComponentRef::WILD { .. }, .. }, tail: rest_lhs }, Deref @ metamodelica::ListNode::Cons { head: _, tail: rest_lhs_prop }, Deref @ metamodelica::ListNode::Cons { head: _, tail: rest_rhs }, Deref @ metamodelica::ListNode::Cons { head: _, tail: rest_rhs_prop }) => {
                { (lhsExps, lhsProps, rhsExps, rhsProps, attributes, initial_, source) = (rest_lhs, rest_lhs_prop, rest_rhs, rest_rhs_prop, attributes, initial_, source); continue '__tco; }
            },
            (Deref @ metamodelica::ListNode::Cons { head: lhs, tail: rest_lhs }, Deref @ metamodelica::ListNode::Cons { head: lhs_prop, tail: rest_lhs_prop }, Deref @ metamodelica::ListNode::Cons { head: rhs, tail: rest_rhs }, Deref @ metamodelica::ListNode::Cons { head: rhs_prop, tail: rest_rhs_prop }) => {
                let mut ass: metamodelica::Ref<DAE::Statement>;
                let mut rest_ass: metamodelica::List<metamodelica::Ref<DAE::Statement>>;
                ass = makeAssignment(lhs.clone(), lhs_prop.clone(), rhs.clone(), rhs_prop.clone(), attributes, initial_, source.clone())?;
                rest_ass = makeAssignmentsList(rest_lhs, rest_lhs_prop, rest_rhs, rest_rhs_prop, attributes, initial_, source)?;
                return Ok(metamodelica::cons(ass, rest_ass))
            },
            _ => return Err("match: no arm matched"),
        } }
    }
}

pub(crate) fn checkLHSWritable(
    mut lhs: metamodelica::List<metamodelica::Ref<DAE::Exp>>,
    mut props: &metamodelica::List<DAE::Properties>,
    mut rhs: metamodelica::Ref<DAE::Exp>,
    mut source: metamodelica::Ref<DAE::ElementSource>,
) -> Result<()> {
    let mut ty: metamodelica::Ref<DAE::Type>;
    let mut i: i32 = 1;
    let mut c: ArcStr;
    let mut l: ArcStr;
    let mut r: ArcStr;
    for mut p in &**props {
        let () = (match p.clone() {
            DAE::Properties::PROP {
                constFlag: DAE::Const::C_VAR { .. },
                ..
            } => (),
            DAE::Properties::PROP {
                type_: _,
                constFlag: DAE::Const::C_CONST { .. },
            } => {
                l = stringAppendList(list![
                    literal!("("),
                    stringDelimitList(List::map(lhs.clone(), &ExpressionBasics::printExpStr)?, literal!(", ")),
                    literal!(")")
                ]);
                r = ExpressionBasics::printExpStr(rhs.clone())?;
                Error::addSourceMessage(
                    &(Error::ASSIGN_CONSTANT_ERROR.clone()),
                    list![l, r],
                    &(ElementSource::getElementSourceFileInfo(source.clone())),
                )?;
                return Err("fail");
                ()
            }
            DAE::Properties::PROP {
                type_: ref __esc_ty,
                constFlag: DAE::Const::C_PARAM { .. },
            } => {
                ty = __esc_ty.clone();
                if Types::getFixedVarAttributeParameterOrConstant(metamodelica::AsArg::as_arg(&ty)) {
                    l = stringAppendList(list![
                        literal!("("),
                        stringDelimitList(List::map(lhs.clone(), &ExpressionBasics::printExpStr)?, literal!(", ")),
                        literal!(")")
                    ]);
                    r = ExpressionBasics::printExpStr(rhs.clone())?;
                    c = ExpressionBasics::printExpStr((lhs).get(i)?)?;
                    Error::addSourceMessage(
                        &(Error::ASSIGN_PARAM_FIXED_ERROR.clone()),
                        list![c, l, r],
                        &(ElementSource::getElementSourceFileInfo(source.clone())),
                    )?;
                    return Err("fail");
                }
                ()
            }
            DAE::Properties::PROP_TUPLE {
                type_: _,
                tupleConst: _,
            } => (),
            _ => return Err("match: no arm matched"),
        });
        i = i + 1;
    }
    Ok(())
}

pub fn makeTupleAssignment(
    mut inExpExpLst: metamodelica::List<metamodelica::Ref<DAE::Exp>>,
    mut inTypesPropertiesLst: metamodelica::List<DAE::Properties>,
    mut inExp: metamodelica::Ref<DAE::Exp>,
    mut inProperties: DAE::Properties,
    mut initial_: SCode::Initial,
    mut source: metamodelica::Ref<DAE::ElementSource>,
) -> Result<metamodelica::Ref<DAE::Statement>> {
    let mut outStatement: metamodelica::Ref<DAE::Statement>;
    outStatement = 'mc: {
        let __mc_input = (inExpExpLst, inTypesPropertiesLst, inExp, inProperties, initial_);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (lhs, lprop, rhs, _, _) => {
                    let mut bvals: metamodelica::List<DAE::Const>;
                    let mut sl: metamodelica::List<ArcStr>;
                    let mut s: ArcStr;
                    let mut lhs_str: ArcStr;
                    let mut rhs_str: ArcStr;
                    bvals = List::map(lprop.clone(), &Types::propAnyConst)?;
                    let DAE::C_CONST { .. } = (List::reduce(&bvals, &fnptr!(Types::constOr, DAE::Const, DAE::Const))?) else { return Err("pattern mismatch") };
                    sl = List::map(lhs.clone(), &ExpressionBasics::printExpStr)?;
                    s = stringDelimitList(sl.clone(), literal!(", "));
                    lhs_str = stringAppendList(list![literal!("("), s.clone(), literal!(")")]);
                    rhs_str = ExpressionBasics::printExpStr(rhs.clone())?;
                    Error::addSourceMessage(&(Error::ASSIGN_CONSTANT_ERROR.clone()), list![lhs_str.clone(), rhs_str.clone()], &(ElementSource::getElementSourceFileInfo(source.clone())))?;
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (lhs, lprop, rhs, _, SCode::Initial::NON_INITIAL { .. }) => {
                    let mut bvals: metamodelica::List<DAE::Const>;
                    let mut sl: metamodelica::List<ArcStr>;
                    let mut s: ArcStr;
                    let mut lhs_str: ArcStr;
                    let mut rhs_str: ArcStr;
                    bvals = List::map(lprop.clone(), &Types::propAnyConst)?;
                    let DAE::C_PARAM { .. } = (List::reduce(&bvals, &fnptr!(Types::constOr, DAE::Const, DAE::Const))?) else { return Err("pattern mismatch") };
                    sl = List::map(lhs.clone(), &ExpressionBasics::printExpStr)?;
                    s = stringDelimitList(sl.clone(), literal!(", "));
                    lhs_str = stringAppendList(list![literal!("("), s.clone(), literal!(")")]);
                    rhs_str = ExpressionBasics::printExpStr(rhs.clone())?;
                    Error::addSourceMessage(&(Error::ASSIGN_PARAM_ERROR.clone()), list![lhs_str.clone(), rhs_str.clone()], &(ElementSource::getElementSourceFileInfo(source.clone())))?;
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (expl, lhprops, rhs, DAE::Properties::PROP { type_: ty @ Deref @ DAE::Type::T_TUPLE { types: tpl, .. }, .. }, _) => {
                    let mut lhrtypes: metamodelica::List<metamodelica::Ref<DAE::Type>>;
                    checkLHSWritable(expl.clone(), metamodelica::AsArg::as_arg(&lhprops), rhs.clone(), source.clone())?;
                    lhrtypes = List::map(lhprops.clone(), &move |__a0: DAE::Properties| -> metamodelica::Result<_> { ::std::result::Result::Ok(Types::getPropType(&__a0)) })?;
                    Types::matchTypeTupleCall(rhs.clone(), metamodelica::AsArg::as_arg(&tpl), &lhrtypes)?;
                    Ok(makeTupleAssignmentNoTypeCheck(ty.clone(), expl.clone(), rhs.clone(), source.clone())?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (expl, lhprops, rhs, DAE::Properties::PROP_TUPLE { type_: ty @ Deref @ DAE::Type::T_TUPLE { types: tpl, .. }, tupleConst: Deref @ DAE::TupleConst::TUPLE_CONST { .. } }, _) => {
                    let mut lhrtypes: metamodelica::List<metamodelica::Ref<DAE::Type>>;
                    checkLHSWritable(expl.clone(), metamodelica::AsArg::as_arg(&lhprops), rhs.clone(), source.clone())?;
                    lhrtypes = List::map(lhprops.clone(), &move |__a0: DAE::Properties| -> metamodelica::Result<_> { ::std::result::Result::Ok(Types::getPropType(&__a0)) })?;
                    Types::matchTypeTupleCall(rhs.clone(), metamodelica::AsArg::as_arg(&tpl), &lhrtypes)?;
                    Ok(makeTupleAssignmentNoTypeCheck(ty.clone(), expl.clone(), rhs.clone(), source.clone())?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (lhs, lprop, rhs, rprop, _) => {
                    let mut sl: metamodelica::List<ArcStr>;
                    let mut s: ArcStr;
                    let mut lhs_str: ArcStr;
                    let mut rhs_str: ArcStr;
                    let mut str1: ArcStr;
                    let mut str2: ArcStr;
                    let mut strInitial: ArcStr;
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { return Err("pattern mismatch") };
                    sl = List::map(lhs.clone(), &ExpressionBasics::printExpStr)?;
                    s = stringDelimitList(sl.clone(), literal!(", "));
                    lhs_str = stringAppendList(list![literal!("("), s.clone(), literal!(")")]);
                    rhs_str = ExpressionBasics::printExpStr(rhs.clone())?;
                    str1 = stringDelimitList(List::map(lprop.clone(), &move |__a0: DAE::Properties| Types::printPropStr(&__a0))?, literal!(", "));
                    str2 = Types::printPropStr(metamodelica::AsArg::as_arg(&rprop))?;
                    strInitial = SCodeDump::printInitialStr(initial_);
                    Debug::traceln({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("- Algorithm.makeTupleAssignment failed on: \n\t")); __mm_s.push_str(&*lhs_str); __mm_s.push_str(&*literal!(" = ")); __mm_s.push_str(&*rhs_str); __mm_s.push_str(&*literal!("\n\tprops lhs: (")); __mm_s.push_str(&*str1); __mm_s.push_str(&*literal!(") =  props rhs: ")); __mm_s.push_str(&*str2); __mm_s.push_str(&*literal!("\n\tin ")); __mm_s.push_str(&*strInitial); __mm_s.push_str(&*literal!(" section")); ArcStr::from(__mm_s) })?;
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(outStatement)
}

fn getPropExpType(mut p: &DAE::Properties) -> Result<metamodelica::Ref<DAE::Type>> {
    let mut t: metamodelica::Ref<DAE::Type>;
    let mut ty: metamodelica::Ref<DAE::Type>;
    ty = Types::getPropType(p);
    t = Types::simplifyType(ty)?;
    Ok(t)
}

pub fn makeIf(
    mut inExp: metamodelica::Ref<DAE::Exp>,
    mut inProperties: &DAE::Properties,
    mut inTrueBranch: metamodelica::List<metamodelica::Ref<DAE::Statement>>,
    mut inElseIfBranches: metamodelica::List<(
        metamodelica::Ref<DAE::Exp>,
        DAE::Properties,
        metamodelica::List<metamodelica::Ref<DAE::Statement>>,
    )>,
    mut inElseBranch: metamodelica::List<metamodelica::Ref<DAE::Statement>>,
    mut source: &metamodelica::Ref<DAE::ElementSource>,
) -> Result<metamodelica::List<metamodelica::Ref<DAE::Statement>>> {
    let mut outStatements: metamodelica::List<metamodelica::Ref<DAE::Statement>>;
    outStatements = 'mc: {
        let __mc_input = (inExp, inProperties, inTrueBranch, inElseIfBranches, inElseBranch);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::BCONST { bool: true }, _, tb, _, _) => {
                    Ok(tb.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::BCONST { bool: false }, _, _, Deref @ metamodelica::ListNode::Nil, fb) => {
                    Ok(fb.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::BCONST { bool: false }, _, _, Deref @ metamodelica::ListNode::Cons { head: (e, prop, tb), tail: eib }, fb) => {
                    Ok(makeIf(e.clone(), metamodelica::AsArg::as_arg(&prop), tb.clone(), eib.clone(), fb.clone(), source)?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e, DAE::Properties::PROP { type_: t, .. }, tb, eib, fb) => {
                    let mut else_: metamodelica::Ref<DAE::Else>;
                    let mut e = (*e).clone();
                    (e, _) = Types::matchType(e.clone(), t.clone(), DAE::T_BOOL_DEFAULT().clone(), true)?;
                    else_ = makeElse(metamodelica::AsArg::as_arg(&eib), fb.clone(), source)?;
                    Ok(list![metamodelica::Ref::new(DAE::Statement::STMT_IF { exp: e.clone(), statementLst: tb.clone(), else_: else_.clone(), source: source.clone() })])
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e, DAE::Properties::PROP { type_: t, .. }, _, _, _) => {
                    let mut e_str: ArcStr;
                    let mut t_str: ArcStr;
                    e_str = ExpressionBasics::printExpStr(e.clone())?;
                    t_str = TypesDump::unparseTypeNoAttr(metamodelica::AsArg::as_arg(&t))?;
                    Error::addSourceMessage(&(Error::IF_CONDITION_TYPE_ERROR.clone()), list![e_str.clone(), t_str.clone()], &(ElementSource::getElementSourceFileInfo(source.clone())))?;
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(outStatements)
}

pub(crate) fn makeIfFromBranches(
    mut branches: &metamodelica::List<(
        metamodelica::Ref<DAE::Exp>,
        metamodelica::List<metamodelica::Ref<DAE::Statement>>,
    )>,
    mut source: metamodelica::Ref<DAE::ElementSource>,
) -> metamodelica::List<metamodelica::Ref<DAE::Statement>> {
    let mut outStatements: metamodelica::List<metamodelica::Ref<DAE::Statement>>;
    outStatements = (::match_deref::match_deref! { match branches {
        Deref @ metamodelica::ListNode::Nil => {
            metamodelica::nil()
        },
        Deref @ metamodelica::ListNode::Cons { head: (e, br), tail: rest } => {
            let mut else_: metamodelica::Ref<DAE::Else>;
            else_ = makeElseFromBranches(rest);
            list![metamodelica::Ref::new(DAE::Statement::STMT_IF { exp: e.clone(), statementLst: br.clone(), else_: else_, source: source })]
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outStatements
}

fn makeElseFromBranches(
    mut inTpl: &metamodelica::List<(
        metamodelica::Ref<DAE::Exp>,
        metamodelica::List<metamodelica::Ref<DAE::Statement>>,
    )>,
) -> metamodelica::Ref<DAE::Else> {
    let mut outElse: metamodelica::Ref<DAE::Else>;
    outElse = (::match_deref::match_deref! { match inTpl {
        Deref @ metamodelica::ListNode::Nil => {
            openmodelica_frontend_types::DAE::Else::interned_NOELSE()
        },
        Deref @ metamodelica::ListNode::Cons { head: (Deref @ DAE::Exp::BCONST { bool: true }, b), tail: Deref @ metamodelica::ListNode::Nil } => {
            metamodelica::Ref::new(DAE::Else::ELSE { statementLst: b.clone() })
        },
        Deref @ metamodelica::ListNode::Cons { head: (e, b), tail: xs } => {
            let mut else_: metamodelica::Ref<DAE::Else>;
            else_ = makeElseFromBranches(xs);
            metamodelica::Ref::new(DAE::Else::ELSEIF { exp: e.clone(), statementLst: b.clone(), else_: else_ })
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outElse
}

pub(crate) fn optimizeIf(
    mut icond: &metamodelica::Ref<DAE::Exp>,
    mut istmts: metamodelica::List<metamodelica::Ref<DAE::Statement>>,
    mut iels: &metamodelica::Ref<DAE::Else>,
    mut isource: metamodelica::Ref<DAE::ElementSource>,
) -> (metamodelica::List<metamodelica::Ref<DAE::Statement>>, bool) {
    let mut ostmts: metamodelica::List<metamodelica::Ref<DAE::Statement>>;
    let mut changed: bool;
    (ostmts, changed) = (::match_deref::match_deref! { match (icond, iels) {
        (Deref @ DAE::Exp::BCONST { bool: true }, _) => {
            let mut stmts = istmts.clone();
            (stmts, true)
        },
        (Deref @ DAE::Exp::BCONST { bool: false }, Deref @ DAE::Else::NOELSE { .. }) => {
            (metamodelica::nil(), true)
        },
        (Deref @ DAE::Exp::BCONST { bool: false }, Deref @ DAE::Else::ELSE { statementLst: stmts }) => {
            (stmts.clone(), true)
        },
        (Deref @ DAE::Exp::BCONST { bool: false }, Deref @ DAE::Else::ELSEIF { exp: cond, statementLst: stmts, else_: els }) => {
            let mut source = isource.clone();
            (ostmts, _) = optimizeIf(cond, stmts.clone(), els, source);
            (ostmts, true)
        },
        _ => {
            (metamodelica::cons(metamodelica::Ref::new(DAE::Statement::STMT_IF { exp: icond.clone(), statementLst: istmts, else_: iels.clone(), source: isource }), metamodelica::nil()), false)
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    (ostmts, changed)
}

pub(crate) fn optimizeElseIf(
    mut cond: metamodelica::Ref<DAE::Exp>,
    mut stmts: metamodelica::List<metamodelica::Ref<DAE::Statement>>,
    mut els: metamodelica::Ref<DAE::Else>,
) -> metamodelica::Ref<DAE::Else> {
    let mut oelse: metamodelica::Ref<DAE::Else>;
    oelse = (match &*cond {
        DAE::Exp::BCONST { bool: true } => metamodelica::Ref::new(DAE::Else::ELSE { statementLst: stmts }),
        DAE::Exp::BCONST { bool: false } => els,
        _ => metamodelica::Ref::new(DAE::Else::ELSEIF {
            exp: cond,
            statementLst: stmts,
            else_: els,
        }),
    });
    oelse
}

fn makeElse(
    mut inTuple: &metamodelica::List<(
        metamodelica::Ref<DAE::Exp>,
        DAE::Properties,
        metamodelica::List<metamodelica::Ref<DAE::Statement>>,
    )>,
    mut inStatementLst: metamodelica::List<metamodelica::Ref<DAE::Statement>>,
    mut inSource: &metamodelica::Ref<DAE::ElementSource>,
) -> Result<metamodelica::Ref<DAE::Else>> {
    let mut outElse: metamodelica::Ref<DAE::Else>;
    outElse = 'mc: {
        let __mc_input = (&**inTuple, inStatementLst);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Nil, Deref @ metamodelica::ListNode::Nil) => {
                    Ok(openmodelica_frontend_types::DAE::Else::interned_NOELSE())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Nil, fb) => {
                    Ok(metamodelica::Ref::new(DAE::Else::ELSE { statementLst: fb.clone() }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: (Deref @ DAE::Exp::BCONST { bool: true }, DAE::Properties::PROP { .. }, b), tail: _ }, _) => {
                    Ok(metamodelica::Ref::new(DAE::Else::ELSE { statementLst: b.clone() }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: (Deref @ DAE::Exp::BCONST { bool: false }, DAE::Properties::PROP { .. }, _), tail: xs }, fb) => {
                    Ok(makeElse(metamodelica::AsArg::as_arg(&xs), fb.clone(), inSource)?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: (e, DAE::Properties::PROP { type_: t, .. }, b), tail: xs }, fb) => {
                    let mut else_: metamodelica::Ref<DAE::Else>;
                    let mut e = (*e).clone();
                    (e, _) = Types::matchType(e.clone(), t.clone(), DAE::T_BOOL_DEFAULT().clone(), true)?;
                    else_ = makeElse(metamodelica::AsArg::as_arg(&xs), fb.clone(), inSource)?;
                    Ok(metamodelica::Ref::new(DAE::Else::ELSEIF { exp: e.clone(), statementLst: b.clone(), else_: else_.clone() }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: (e, DAE::Properties::PROP { type_: t, .. }, _), tail: _ }, _) => {
                    let mut e_str: ArcStr;
                    let mut t_str: ArcStr;
                    let mut info: SourceInfo;
                    e_str = ExpressionBasics::printExpStr(e.clone())?;
                    t_str = TypesDump::unparseTypeNoAttr(metamodelica::AsArg::as_arg(&t))?;
                    info = ElementSource::getElementSourceFileInfo(inSource.clone());
                    Error::addSourceMessage(&(Error::IF_CONDITION_TYPE_ERROR.clone()), list![e_str.clone(), t_str.clone()], &info)?;
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(outElse)
}

pub fn makeFor(
    mut inIdent: ArcStr,
    mut inExp: metamodelica::Ref<DAE::Exp>,
    mut inProperties: &DAE::Properties,
    mut inStatementLst: metamodelica::List<metamodelica::Ref<DAE::Statement>>,
    mut source: metamodelica::Ref<DAE::ElementSource>,
) -> Result<metamodelica::Ref<DAE::Statement>> {
    let mut outStatement: metamodelica::Ref<DAE::Statement>;
    outStatement = 'mc: {
        let __mc_input = (inIdent, inExp, inProperties, inStatementLst);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (i, e, DAE::Properties::PROP { type_: Deref @ DAE::Type::T_ARRAY { ty: t, dims }, .. }, stmts) => {
                    let mut isArray: bool;
                    isArray = Types::isNonscalarArray(metamodelica::AsArg::as_arg(&t), metamodelica::AsArg::as_arg(&dims));
                    Ok(metamodelica::Ref::new(DAE::Statement::STMT_FOR { type_: t.clone(), iterIsArray: isArray, iter: i.clone(), range: e.clone(), statementLst: stmts.clone(), source: source.clone(), sub_iters: metamodelica::nil() }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (i, e, DAE::Properties::PROP { type_: Deref @ DAE::Type::T_METALIST { ty: t }, .. }, stmts) => {
                    let mut t = (*t).clone();
                    t = Types::simplifyType(t.clone())?;
                    Ok(metamodelica::Ref::new(DAE::Statement::STMT_FOR { type_: t.clone(), iterIsArray: false, iter: i.clone(), range: e.clone(), statementLst: stmts.clone(), source: source.clone(), sub_iters: metamodelica::nil() }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (i, e, DAE::Properties::PROP { type_: Deref @ DAE::Type::T_METAARRAY { ty: t }, .. }, stmts) => {
                    let mut t = (*t).clone();
                    t = Types::simplifyType(t.clone())?;
                    Ok(metamodelica::Ref::new(DAE::Statement::STMT_FOR { type_: t.clone(), iterIsArray: false, iter: i.clone(), range: e.clone(), statementLst: stmts.clone(), source: source.clone(), sub_iters: metamodelica::nil() }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, e, DAE::Properties::PROP { type_: t, .. }, _) => {
                    let mut e_str: ArcStr;
                    let mut t_str: ArcStr;
                    e_str = ExpressionBasics::printExpStr(e.clone())?;
                    t_str = TypesDump::unparseTypeNoAttr(metamodelica::AsArg::as_arg(&t))?;
                    Error::addSourceMessage(&(Error::FOR_EXPRESSION_TYPE_ERROR.clone()), list![e_str.clone(), t_str.clone()], &(ElementSource::getElementSourceFileInfo(source.clone())))?;
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(outStatement)
}

pub fn makeParFor(
    mut inIdent: ArcStr,
    mut inExp: metamodelica::Ref<DAE::Exp>,
    mut inProperties: &DAE::Properties,
    mut inStatementLst: metamodelica::List<metamodelica::Ref<DAE::Statement>>,
    mut inLoopPrlVars: metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, SourceInfo)>,
    mut source: metamodelica::Ref<DAE::ElementSource>,
) -> Result<metamodelica::Ref<DAE::Statement>> {
    let mut outStatement: metamodelica::Ref<DAE::Statement>;
    outStatement = (::match_deref::match_deref! { match &(inProperties) {
        DAE::Properties::PROP { type_: Deref @ DAE::Type::T_ARRAY { ty: t, dims }, .. } => {
            let mut i = inIdent;
            let mut e = inExp;
            let mut stmts = inStatementLst;
            let mut isArray: bool;
            isArray = Types::isNonscalarArray(metamodelica::AsArg::as_arg(&t), metamodelica::AsArg::as_arg(&dims));
            metamodelica::Ref::new(DAE::Statement::STMT_PARFOR { type_: t.clone(), iterIsArray: isArray, iter: i, range: e, statementLst: stmts, loopPrlVars: inLoopPrlVars, source: source })
        },
        DAE::Properties::PROP { type_: t, .. } => {
            let mut e = inExp;
            let mut e_str: ArcStr;
            let mut t_str: ArcStr;
            e_str = ExpressionBasics::printExpStr(e)?;
            t_str = TypesDump::unparseTypeNoAttr(metamodelica::AsArg::as_arg(&t))?;
            Error::addSourceMessage(&(Error::FOR_EXPRESSION_TYPE_ERROR.clone()), list![e_str, t_str], &(ElementSource::getElementSourceFileInfo(source)))?;
            return Err("fail")
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok(outStatement)
}

pub fn makeWhile(
    mut inExp: metamodelica::Ref<DAE::Exp>,
    mut inProperties: &DAE::Properties,
    mut inStatementLst: metamodelica::List<metamodelica::Ref<DAE::Statement>>,
    mut source: metamodelica::Ref<DAE::ElementSource>,
) -> Result<metamodelica::Ref<DAE::Statement>> {
    let mut outStatement: metamodelica::Ref<DAE::Statement>;
    outStatement = (::match_deref::match_deref! { match &(inProperties) {
        DAE::Properties::PROP { type_: Deref @ DAE::Type::T_BOOL { .. }, .. } => {
            let mut e = inExp;
            let mut stmts = inStatementLst;
            metamodelica::Ref::new(DAE::Statement::STMT_WHILE { exp: e, statementLst: stmts, source: source })
        },
        DAE::Properties::PROP { type_: t, .. } => {
            let mut e = inExp;
            let mut e_str: ArcStr;
            let mut t_str: ArcStr;
            e_str = ExpressionBasics::printExpStr(e)?;
            t_str = TypesDump::unparseTypeNoAttr(metamodelica::AsArg::as_arg(&t))?;
            Error::addSourceMessage(&(Error::WHILE_CONDITION_TYPE_ERROR.clone()), list![e_str, t_str], &(ElementSource::getElementSourceFileInfo(source)))?;
            return Err("fail")
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok(outStatement)
}

pub fn makeWhenA(
    mut inExp: metamodelica::Ref<DAE::Exp>,
    mut inProperties: &DAE::Properties,
    mut inStatementLst: metamodelica::List<metamodelica::Ref<DAE::Statement>>,
    mut elseWhenStmt: Option<metamodelica::Ref<DAE::Statement>>,
    mut source: metamodelica::Ref<DAE::ElementSource>,
) -> Result<metamodelica::Ref<DAE::Statement>> {
    let mut outStatement: metamodelica::Ref<DAE::Statement>;
    outStatement = (::match_deref::match_deref! { match &(inProperties) {
        DAE::Properties::PROP { type_: Deref @ DAE::Type::T_BOOL { .. }, .. } => {
            let mut e = inExp;
            let mut stmts = inStatementLst;
            let mut elsew = elseWhenStmt;
            metamodelica::Ref::new(DAE::Statement::STMT_WHEN { exp: e, conditions: metamodelica::nil(), initialCall: false, statementLst: stmts, elseWhen: elsew, source: source })
        },
        DAE::Properties::PROP { type_: Deref @ DAE::Type::T_ARRAY { ty: Deref @ DAE::Type::T_BOOL { .. }, .. }, .. } => {
            let mut e = inExp;
            let mut stmts = inStatementLst;
            let mut elsew = elseWhenStmt;
            metamodelica::Ref::new(DAE::Statement::STMT_WHEN { exp: e, conditions: metamodelica::nil(), initialCall: false, statementLst: stmts, elseWhen: elsew, source: source })
        },
        DAE::Properties::PROP { type_: t, .. } => {
            let mut e = inExp;
            let mut e_str: ArcStr;
            let mut t_str: ArcStr;
            e_str = ExpressionBasics::printExpStr(e)?;
            t_str = TypesDump::unparseTypeNoAttr(metamodelica::AsArg::as_arg(&t))?;
            Error::addSourceMessage(&(Error::WHEN_CONDITION_TYPE_ERROR.clone()), list![e_str, t_str], &(ElementSource::getElementSourceFileInfo(source)))?;
            return Err("fail")
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok(outStatement)
}

pub fn makeReinit(
    mut inExp1: metamodelica::Ref<DAE::Exp>,
    mut inExp2: metamodelica::Ref<DAE::Exp>,
    mut inProperties3: &DAE::Properties,
    mut inProperties4: &DAE::Properties,
    mut source: metamodelica::Ref<DAE::ElementSource>,
) -> Result<metamodelica::List<metamodelica::Ref<DAE::Statement>>> {
    let mut outStatement: metamodelica::List<metamodelica::Ref<DAE::Statement>>;
    outStatement = 'mc: {
        let __mc_input = (inExp1, inExp2, inProperties3, inProperties4);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (var @ Deref @ DAE::Exp::CREF { .. }, val, DAE::Properties::PROP { type_: tp1, constFlag: _ }, DAE::Properties::PROP { type_: tp2, constFlag: _ }) => {
                    let mut var_1: metamodelica::Ref<DAE::Exp>;
                    let mut val_1: metamodelica::Ref<DAE::Exp>;
                    (val_1, _) = Types::matchType(val.clone(), tp2.clone(), DAE::T_REAL_DEFAULT().clone(), true)?;
                    (var_1, _) = Types::matchType(var.clone(), tp1.clone(), DAE::T_REAL_DEFAULT().clone(), true)?;
                    Ok(list![metamodelica::Ref::new(DAE::Statement::STMT_REINIT { var: var_1.clone(), value: val_1.clone(), source: source.clone() })])
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Error::addSourceMessage(&(Error::INTERNAL_ERROR.clone()), list![literal!("reinit called with wrong args")], &(ElementSource::getElementSourceFileInfo(source.clone())))?;
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(outStatement)
}

pub fn makeAssert(
    mut cond: metamodelica::Ref<DAE::Exp>,
    mut msg: metamodelica::Ref<DAE::Exp>,
    mut level: metamodelica::Ref<DAE::Exp>,
    mut inProperties3: &DAE::Properties,
    mut inProperties4: &DAE::Properties,
    mut inProperties5: &DAE::Properties,
    mut source: metamodelica::Ref<DAE::ElementSource>,
) -> Result<metamodelica::List<metamodelica::Ref<DAE::Statement>>> {
    let mut outStatement: metamodelica::List<metamodelica::Ref<DAE::Statement>>;
    outStatement = (::match_deref::match_deref! { match &((cond.clone(), inProperties3.clone(), inProperties4.clone(), inProperties5.clone())) {
        (Deref @ DAE::Exp::BCONST { bool: true }, DAE::Properties::PROP { type_: Deref @ DAE::Type::T_BOOL { .. }, .. }, DAE::Properties::PROP { type_: Deref @ DAE::Type::T_STRING { .. }, .. }, DAE::Properties::PROP { type_: Deref @ DAE::Type::T_ENUMERATION { path: Deref @ Absyn::Path::FULLYQUALIFIED { path: Deref @ Absyn::Path::IDENT { name: Deref @ "AssertionLevel" } }, .. }, .. }) => {
            metamodelica::nil()
        },
        (_, DAE::Properties::PROP { type_: Deref @ DAE::Type::T_BOOL { .. }, .. }, DAE::Properties::PROP { type_: Deref @ DAE::Type::T_STRING { .. }, .. }, DAE::Properties::PROP { type_: Deref @ DAE::Type::T_ENUMERATION { path: Deref @ Absyn::Path::FULLYQUALIFIED { path: Deref @ Absyn::Path::IDENT { name: Deref @ "AssertionLevel" } }, .. }, .. }) => {
            list![metamodelica::Ref::new(DAE::Statement::STMT_ASSERT { cond: cond, msg: msg, level: level, source: source })]
        },
        (_, DAE::Properties::PROP { type_: t1, .. }, DAE::Properties::PROP { type_: t2, .. }, DAE::Properties::PROP { type_: t3, .. }) => {
            let mut info: SourceInfo;
            let mut strTy: ArcStr;
            let mut strExp: ArcStr;
            info = ElementSource::getElementSourceFileInfo(source);
            strExp = ExpressionBasics::printExpStr(cond)?;
            strTy = TypesDump::unparseType(t1.clone())?;
            Error::assertionOrAddSourceMessage(Types::isBooleanOrSubTypeBoolean(t1.clone()), &(Error::EXP_TYPE_MISMATCH.clone()), list![strExp, literal!("Boolean"), strTy], &info)?;
            strExp = ExpressionBasics::printExpStr(msg)?;
            strTy = TypesDump::unparseType(t2.clone())?;
            Error::assertionOrAddSourceMessage(Types::isString(metamodelica::AsArg::as_arg(&t2)), &(Error::EXP_TYPE_MISMATCH.clone()), list![strExp, literal!("String"), strTy], &info)?;
            if '__try0: {
                ::match_deref::match_deref! { match &(t3.clone()) {
                    Deref @ DAE::Type::T_ENUMERATION { path: Deref @ Absyn::Path::IDENT { name: Deref @ "AssertionLevel" }, .. } => (),
                    _ => break '__try0 Err::<_, _>("pattern mismatch"),
                } };
                Ok::<(), &'static str>(())
            }.is_ok() { return Err("failure(): body succeeded") }
            strExp = ExpressionBasics::printExpStr(level)?;
            strTy = TypesDump::unparseType(t3.clone())?;
            Error::assertionOrAddSourceMessage(Types::isString(metamodelica::AsArg::as_arg(&t3)), &(Error::EXP_TYPE_MISMATCH.clone()), list![strExp, literal!("AssertionLevel"), strTy], &info)?;
            return Err("fail")
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok(outStatement)
}

pub fn makeTerminate(
    mut msg: metamodelica::Ref<DAE::Exp>,
    mut props: &DAE::Properties,
    mut source: metamodelica::Ref<DAE::ElementSource>,
) -> Result<metamodelica::List<metamodelica::Ref<DAE::Statement>>> {
    let mut outStatement: metamodelica::List<metamodelica::Ref<DAE::Statement>>;
    outStatement = (::match_deref::match_deref! { match &(props) {
        DAE::Properties::PROP { type_: Deref @ DAE::Type::T_STRING { .. }, .. } => list![metamodelica::Ref::new(DAE::Statement::STMT_TERMINATE { msg: msg, source: source })],
        _ => return Err("match: no arm matched"),
    } });
    Ok(outStatement)
}

pub(crate) fn getCrefFromAlg(
    mut alg: &metamodelica::Ref<DAE::Algorithm>,
) -> Result<metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>> {
    let mut crs: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
    crs = List::unionOnTrueList(
        &(List::map(getAllExps(alg)?, &Expression::extractCrefsFromExp)?),
        (std::sync::Arc::new(
            move |__a0: metamodelica::Ref<DAE::ComponentRef>, __a1: metamodelica::Ref<DAE::ComponentRef>| {
                ComponentReferenceBasics::crefEqual(&__a0, &__a1)
            },
        )
            as std::sync::Arc<
                dyn ::std::ops::Fn(
                        metamodelica::Ref<DAE::ComponentRef>,
                        metamodelica::Ref<DAE::ComponentRef>,
                    ) -> Result<bool>
                    + 'static,
            >),
    )?;
    Ok(crs)
}

pub fn getAllExps(
    mut inAlgorithm: &metamodelica::Ref<DAE::Algorithm>,
) -> Result<metamodelica::List<metamodelica::Ref<DAE::Exp>>> {
    let mut outExpExpLst: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
    outExpExpLst = (match &**inAlgorithm {
        DAE::Algorithm { statementLst: stmts } => {
            let mut exps: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
            exps = getAllExpsStmts(stmts.clone())?;
            exps
        }
    });
    Ok(outExpExpLst)
}

pub(crate) fn getAllExpsStmts(
    mut stmts: metamodelica::List<metamodelica::Ref<DAE::Statement>>,
) -> Result<metamodelica::List<metamodelica::Ref<DAE::Exp>>> {
    let mut exps: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
    let (_, (_, __pa0)) = DAEUtil::traverseDAEEquationsStmts(
        stmts,
        (std::sync::Arc::new(Expression::traverseSubexpressionsHelper)
            as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>, _) -> Result<_> + 'static>),
        (
            (std::sync::Arc::new(fnptr!(
                Expression::expressionCollector,
                metamodelica::Ref<DAE::Exp>,
                metamodelica::List<metamodelica::Ref<DAE::Exp>>
            ))
                as std::sync::Arc<
                    dyn ::std::ops::Fn(
                            metamodelica::Ref<DAE::Exp>,
                            metamodelica::List<metamodelica::Ref<DAE::Exp>>,
                        ) -> Result<(
                            metamodelica::Ref<DAE::Exp>,
                            metamodelica::List<metamodelica::Ref<DAE::Exp>>,
                        )> + 'static,
                >),
            metamodelica::nil(),
        ),
    )?;
    exps = metamodelica::Own::own(__pa0);
    Ok(exps)
}

pub fn getStatementSource(
    mut stmt: &metamodelica::Ref<DAE::Statement>,
) -> Result<metamodelica::Ref<DAE::ElementSource>> {
    let mut source: metamodelica::Ref<DAE::ElementSource>;
    source = (match &**stmt {
        DAE::Statement::STMT_ASSIGN {
            source: __esc_source, ..
        } => {
            source = (*__esc_source).clone();
            source.clone()
        }
        DAE::Statement::STMT_TUPLE_ASSIGN {
            source: __esc_source, ..
        } => {
            source = (*__esc_source).clone();
            source.clone()
        }
        DAE::Statement::STMT_ASSIGN_ARR {
            source: __esc_source, ..
        } => {
            source = (*__esc_source).clone();
            source.clone()
        }
        DAE::Statement::STMT_IF {
            source: __esc_source, ..
        } => {
            source = (*__esc_source).clone();
            source.clone()
        }
        DAE::Statement::STMT_FOR {
            source: __esc_source, ..
        } => {
            source = (*__esc_source).clone();
            source.clone()
        }
        DAE::Statement::STMT_PARFOR {
            source: __esc_source, ..
        } => {
            source = (*__esc_source).clone();
            source.clone()
        }
        DAE::Statement::STMT_WHILE {
            source: __esc_source, ..
        } => {
            source = (*__esc_source).clone();
            source.clone()
        }
        DAE::Statement::STMT_WHEN {
            source: __esc_source, ..
        } => {
            source = (*__esc_source).clone();
            source.clone()
        }
        DAE::Statement::STMT_ASSERT {
            source: __esc_source, ..
        } => {
            source = (*__esc_source).clone();
            source.clone()
        }
        DAE::Statement::STMT_TERMINATE {
            source: __esc_source, ..
        } => {
            source = (*__esc_source).clone();
            source.clone()
        }
        DAE::Statement::STMT_REINIT {
            source: __esc_source, ..
        } => {
            source = (*__esc_source).clone();
            source.clone()
        }
        DAE::Statement::STMT_NORETCALL {
            source: __esc_source, ..
        } => {
            source = (*__esc_source).clone();
            source.clone()
        }
        DAE::Statement::STMT_RETURN { source: __esc_source } => {
            source = (*__esc_source).clone();
            source.clone()
        }
        DAE::Statement::STMT_BREAK { source: __esc_source } => {
            source = (*__esc_source).clone();
            source.clone()
        }
        DAE::Statement::STMT_CONTINUE { source: __esc_source } => {
            source = (*__esc_source).clone();
            source.clone()
        }
        DAE::Statement::STMT_FAILURE {
            source: __esc_source, ..
        } => {
            source = (*__esc_source).clone();
            source.clone()
        }
        _ => {
            Error::addMessage(
                Error::INTERNAL_ERROR.clone(),
                list![literal!("Algorithm.getStatementSource")],
            )?;
            return Err("fail");
        }
    });
    Ok(source)
}

pub fn isNotDummyStatement(mut stmt: &metamodelica::Ref<DAE::Statement>) -> Result<bool> {
    let mut b: bool;
    b = (match &**stmt {
        DAE::Statement::STMT_NORETCALL { exp, .. } => {
            (_, b) = Expression::traverseExpBottomUp(
                exp.clone(),
                &fnptr!(Expression::hasNoSideEffects, metamodelica::Ref<DAE::Exp>, bool),
                true,
            )?;
            !(b)
        }
        _ => true,
    });
    Ok(b)
}
