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

use openmodelica_ast::Absyn;
use openmodelica_ast_collections::HashTableStringToPath;
use openmodelica_frontend::HashTableExpToIndex;
use openmodelica_frontend::Mod;
use openmodelica_frontend::Patternm;
use openmodelica_frontend_base::ComponentReference;
use openmodelica_frontend_base::DAEDump;
use openmodelica_frontend_base::DAEUtil;
use openmodelica_frontend_base::Expression;
use openmodelica_frontend_base::ExpressionSimplify;
use openmodelica_frontend_base::Types;
use openmodelica_frontend_dump::AbsynUtil;
use openmodelica_frontend_dump::AvlTreePathFunction;
use openmodelica_frontend_dump::ComponentReferenceBasics;
use openmodelica_frontend_dump::ElementSource;
use openmodelica_frontend_dump::ExpressionBasics;
use openmodelica_frontend_dump::SCodeUtil;
use openmodelica_frontend_dump::TypesDump;
use openmodelica_frontend_types::ClassInf;
use openmodelica_frontend_types::DAE;
use openmodelica_frontend_types::SCode;
use openmodelica_program_util::ProgramUtil;
use openmodelica_simcode_types::SimCodeFunction;
use openmodelica_simcode_types::SimCodeVar;
use openmodelica_util::Autoconf;
use openmodelica_util::BaseHashTable;
use openmodelica_util::Config;
use openmodelica_util::Error;
use openmodelica_util::Flags;
use openmodelica_util::Global;
use openmodelica_util::Graph;
use openmodelica_util::Settings;
use openmodelica_util::StringUtil;
use openmodelica_util::System;
use openmodelica_util::Testsuite;
use openmodelica_util::UnorderedMap;
use openmodelica_util::Util;
use openmodelica_util_datatypes_basic::Array;
use openmodelica_util_datatypes_basic::List;

pub fn elementVars(
    mut ild: metamodelica::List<metamodelica::Ref<DAE::Element>>,
) -> Result<metamodelica::List<metamodelica::Ref<SimCodeFunction::Variable::Variable>>> {
    let mut vars: metamodelica::List<metamodelica::Ref<SimCodeFunction::Variable::Variable>>;
    let mut ld: metamodelica::List<metamodelica::Ref<DAE::Element>>;
    ld = List::filterOnTrue(
        ild,
        (std::sync::Arc::new(
            move |__a0: metamodelica::Ref<DAE::Element>| -> metamodelica::Result<_> {
                ::std::result::Result::Ok(isVarQ(&__a0))
            },
        ) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Element>) -> Result<bool> + 'static>),
    )?;
    vars = List::map(ld, &move |__a0: metamodelica::Ref<DAE::Element>| daeInOutSimVar(&__a0))?;
    Ok(vars)
}

pub fn crefSubIsScalar(mut cref: &metamodelica::Ref<DAE::ComponentRef>) -> Result<bool> {
    let mut isScalar: bool;
    let mut subs: metamodelica::List<metamodelica::Ref<DAE::Subscript>>;
    subs = ComponentReferenceBasics::crefSubs(cref)?;
    isScalar = subsToScalar(&subs)?;
    Ok(isScalar)
}

fn subsToScalar<'__b>(
    mut inExpSubscriptLst: &'__b metamodelica::List<metamodelica::Ref<DAE::Subscript>>,
) -> Result<bool> {
    '__tco: loop {
        ::match_deref::match_deref! { match inExpSubscriptLst {
            Deref @ metamodelica::ListNode::Nil => {
                return Ok(true)
            },
            Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Subscript::SLICE { .. }, tail: _ } => {
                return Ok(false)
            },
            Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Subscript::WHOLEDIM { .. }, tail: _ } => {
                return Ok(false)
            },
            Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Subscript::INDEX { .. }, tail: r } => {
                let mut b: bool;
                { inExpSubscriptLst = r; continue '__tco; }
            },
            _ => return Err("match: no arm matched"),
        } }
    }
}

pub fn crefNoSub(mut cref: &metamodelica::Ref<DAE::ComponentRef>) -> bool {
    let mut noSub: bool;
    noSub = !(ComponentReference::crefHaveSubs(cref));
    noSub
}

pub fn inFunctionContext(mut inContext: &SimCodeFunction::Context) -> bool {
    let mut outInFunction: bool;
    outInFunction = (match inContext.clone() {
        SimCodeFunction::Context::FUNCTION_CONTEXT { .. } => true,
        _ => false,
    });
    outInFunction
}

pub fn crefIsScalar(
    mut cref: &metamodelica::Ref<DAE::ComponentRef>,
    mut context: &SimCodeFunction::Context,
) -> Result<bool> {
    let mut isScalar: bool;
    if inFunctionContext(context) {
        isScalar = (ComponentReference::crefLastSubs(cref)?).is_empty();
    } else if Flags::isSet(Flags::NF_SCALARIZE.clone())? {
        isScalar = ComponentReference::crefHasScalarSubscripts(cref);
    } else {
        isScalar = !(ComponentReference::crefHaveSubs(cref));
    }
    Ok(isScalar)
}

pub(crate) fn buildCrefExpFromAsub(
    mut cref: metamodelica::Ref<DAE::Exp>,
    mut subs: metamodelica::List<metamodelica::Ref<DAE::Exp>>,
) -> Result<metamodelica::Ref<DAE::Exp>> {
    let mut cRefOut: metamodelica::Ref<DAE::Exp>;
    cRefOut = (::match_deref::match_deref! { match &((cref.clone(), subs.clone())) {
        (_, Deref @ metamodelica::ListNode::Nil) => {
            cref
        },
        (Deref @ DAE::Exp::CREF { componentRef: crNew, ty }, _) => {
            let mut crefExp: metamodelica::Ref<DAE::Exp>;
            let mut indexes: metamodelica::List<metamodelica::Ref<DAE::Subscript>>;
            let mut crNew = (*crNew).clone();
            indexes = List::map(subs, &fnptr!(Expression::makeIndexSubscript, metamodelica::Ref<DAE::Exp>))?;
            crNew = ComponentReference::subscriptCref(metamodelica::AsArg::as_arg(&crNew), indexes)?;
            crefExp = Expression::makeCrefExp(crNew.clone(), ty.clone())?;
            crefExp
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok(cRefOut)
}

pub fn buildCrefExpFromSubs(
    mut cref: metamodelica::Ref<DAE::Exp>,
    mut subs: metamodelica::List<metamodelica::Ref<DAE::Subscript>>,
) -> Result<metamodelica::Ref<DAE::Exp>> {
    let mut cRefOut: metamodelica::Ref<DAE::Exp>;
    cRefOut = (::match_deref::match_deref! { match &((cref.clone(), subs.clone())) {
        (_, Deref @ metamodelica::ListNode::Nil) => {
            cref
        },
        (Deref @ DAE::Exp::CREF { componentRef: crNew, ty }, _) => {
            let mut crefExp: metamodelica::Ref<DAE::Exp>;
            let mut crNew = (*crNew).clone();
            crNew = ComponentReference::subscriptCref(metamodelica::AsArg::as_arg(&crNew), subs)?;
            crefExp = Expression::makeCrefExp(crNew.clone(), ty.clone())?;
            crefExp
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok(cRefOut)
}

pub fn padAsubSubscripts(
    mut exp: metamodelica::Ref<DAE::Exp>,
    mut subs: metamodelica::List<metamodelica::Ref<DAE::Subscript>>,
) -> Result<metamodelica::List<metamodelica::Ref<DAE::Subscript>>> {
    let mut outSubs: metamodelica::List<metamodelica::Ref<DAE::Subscript>>;
    let mut n: i32 =
        ((Expression::arrayDimension(&(Expression::r#typeof(exp.clone())?))).len() as i32) - ((subs).len() as i32);
    outSubs = if (n > 0) {
        listAppend(
            subs,
            List::fill(openmodelica_frontend_types::DAE::Subscript::interned_WHOLEDIM(), n),
        )
    } else {
        subs
    };
    Ok(outSubs)
}

pub fn incrementInt(mut inInt: i32, mut increment: i32) -> i32 {
    let mut outInt: i32;
    outInt = inInt + increment;
    outInt
}

pub(crate) fn decrementInt(mut inInt: i32, mut decrement: i32) -> i32 {
    let mut outInt: i32;
    outInt = inInt - decrement;
    outInt
}

pub fn protectedVars(
    mut InSimVars: metamodelica::List<metamodelica::Ref<SimCodeVar::SimVar>>,
) -> Result<metamodelica::List<metamodelica::Ref<SimCodeVar::SimVar>>> {
    let mut OutSimVars: metamodelica::List<metamodelica::Ref<SimCodeVar::SimVar>>;
    OutSimVars = List::filterOnTrue(
        InSimVars,
        (std::sync::Arc::new(
            move |__a0: metamodelica::Ref<SimCodeVar::SimVar>| -> metamodelica::Result<_> {
                ::std::result::Result::Ok(isNotProtected(&__a0))
            },
        )
            as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<SimCodeVar::SimVar>) -> Result<bool> + 'static>),
    )?;
    Ok(OutSimVars)
}

fn isNotProtected(mut simVar: &metamodelica::Ref<SimCodeVar::SimVar>) -> bool {
    let mut isProtected: bool;
    let __arc1 = &(*simVar);
    let SimCodeVar::SIMVAR { isProtected: __pa0, .. } = &**__arc1;
    isProtected = metamodelica::Own::own(__pa0);
    isProtected = !(isProtected);
    isProtected
}

pub fn makeCrefRecordExp(
    mut inCRefRecord: metamodelica::Ref<DAE::ComponentRef>,
    mut inVar: &metamodelica::Ref<DAE::Var>,
) -> Result<metamodelica::Ref<DAE::Exp>> {
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    outExp = (match &**inVar {
        DAE::Var { name, ty: tp, .. } => {
            let mut cr = inCRefRecord;
            let mut cr1: metamodelica::Ref<DAE::ComponentRef>;
            cr1 = ComponentReference::crefPrependIdent(&cr, name, &(metamodelica::nil()), tp)?;
            outExp = Expression::makeCrefExp(cr1, tp.clone())?;
            outExp
        }
    });
    Ok(outExp)
}

pub fn splitRecordAssignmentToMemberAssignments(
    mut lhs_cref: metamodelica::Ref<DAE::ComponentRef>,
    mut lhs_type: metamodelica::Ref<DAE::Type>,
    mut rhs_cref_str: ArcStr,
) -> Result<metamodelica::List<metamodelica::Ref<DAE::Statement>>> {
    let mut outAssigns: metamodelica::List<metamodelica::Ref<DAE::Statement>>;
    let mut rhs_cref: metamodelica::Ref<DAE::ComponentRef>;
    outAssigns = metamodelica::nil();
    rhs_cref = metamodelica::Ref::new(DAE::ComponentRef::CREF_IDENT {
        ident: rhs_cref_str,
        identType: lhs_type.clone(),
        subscriptLst: metamodelica::nil(),
    });
    let () = (match &*lhs_type {
        DAE::Type::T_COMPLEX {
            varLst: __lhs_type_varLst,
            ..
        } => {
            let mut l_v_exp: metamodelica::Ref<DAE::Exp>;
            let mut r_v_exp: metamodelica::Ref<DAE::Exp>;
            let mut stmt: metamodelica::Ref<DAE::Statement>;
            for mut v in &*__lhs_type_varLst.clone() {
                l_v_exp = makeCrefRecordExp(lhs_cref.clone(), metamodelica::AsArg::as_arg(&v))?;
                r_v_exp = makeCrefRecordExp(rhs_cref.clone(), metamodelica::AsArg::as_arg(&v))?;
                if Types::isArray(&v.ty) {
                    stmt = metamodelica::Ref::new(DAE::Statement::STMT_ASSIGN_ARR {
                        type_: v.ty.clone(),
                        lhs: l_v_exp,
                        exp: r_v_exp,
                        source: DAE::emptyElementSource().clone(),
                    });
                } else {
                    stmt = metamodelica::Ref::new(DAE::Statement::STMT_ASSIGN {
                        type_: v.ty.clone(),
                        exp1: l_v_exp,
                        exp: r_v_exp,
                        source: DAE::emptyElementSource().clone(),
                    });
                }
                outAssigns = metamodelica::cons(stmt, outAssigns);
            }
            outAssigns = outAssigns.reverse();
            ()
        }
        _ => return Err("match: no arm matched"),
    });
    Ok(outAssigns)
}

pub(crate) fn derComponentRef(
    mut inCref: metamodelica::Ref<DAE::ComponentRef>,
) -> metamodelica::Ref<DAE::ComponentRef> {
    let mut derCref: metamodelica::Ref<DAE::ComponentRef>;
    derCref = ComponentReference::crefPrefixDer(inCref);
    derCref
}

pub(crate) fn hackArrayReverseToCref(
    mut inExp: metamodelica::Ref<DAE::Exp>,
    mut context: SimCodeFunction::Context,
) -> metamodelica::Ref<DAE::Exp> {
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    outExp = 'mc: {
        let __mc_input = &*inExp;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::ARRAY { ty: aty, scalar: true, array: Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Exp::CREF { componentRef: cr, .. }, tail: aRest } } => {
                    let mut crefExp: metamodelica::Ref<DAE::Exp>;
                    let mut cr = (*cr).clone();
                    if '__try0: {
                        let SimCodeFunction::FUNCTION_CONTEXT { .. } = (context.clone()) else { break '__try0 Err::<_, _>("pattern mismatch") };
                        Ok::<(), &'static str>(())
                    }.is_ok() { return Err("failure(): body succeeded") }
                    ::match_deref::match_deref! { match &(ComponentReference::crefLastSubs(metamodelica::AsArg::as_arg(&cr))?) {
                        Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Subscript::INDEX { exp: Deref @ DAE::Exp::ICONST { integer: 1 } }, tail: Deref @ metamodelica::ListNode::Nil } => (),
                        _ => return Err("pattern mismatch"),
                    } };
                    cr = ComponentReferenceBasics::crefStripLastSubs(metamodelica::AsArg::as_arg(&cr))?;
                    let true = (isArrayExpansion(metamodelica::AsArg::as_arg(&aRest), metamodelica::AsArg::as_arg(&cr), 2)) else { return Err("pattern mismatch") };
                    crefExp = Expression::makeCrefExp(cr.clone(), aty.clone())?;
                    Ok(crefExp.clone())
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

fn isArrayExpansion(
    mut inArrayElems: &metamodelica::List<metamodelica::Ref<DAE::Exp>>,
    mut inCref: &metamodelica::Ref<DAE::ComponentRef>,
    mut index: i32,
) -> bool {
    let mut isExpanded: bool;
    isExpanded = 'mc: {
        let __mc_input = &**inArrayElems;
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
                Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Exp::CREF { componentRef: cr, .. }, tail: aRest } => {
                    let mut i: i32;
                    let mut cr = (*cr).clone();
                    let __pa0 = ::match_deref::match_deref! { match &(ComponentReference::crefLastSubs(metamodelica::AsArg::as_arg(&cr))?) {
                        Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Subscript::INDEX { exp: Deref @ DAE::Exp::ICONST { integer: __pa0 } }, tail: Deref @ metamodelica::ListNode::Nil } => __pa0.clone(),
                        _ => return Err("pattern mismatch"),
                    } };
                    i = metamodelica::Own::own(__pa0);
                    let true = (i == index) else { return Err("pattern mismatch") };
                    cr = ComponentReferenceBasics::crefStripLastSubs(metamodelica::AsArg::as_arg(&cr))?;
                    let true = (ComponentReferenceBasics::crefEqualNoStringCompare(inCref, metamodelica::AsArg::as_arg(&cr))?) else { return Err("pattern mismatch") };
                    Ok(isArrayExpansion(metamodelica::AsArg::as_arg(&aRest), inCref, index + 1))
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
    isExpanded
}

pub(crate) fn hackMatrixReverseToCref(
    mut inExp: metamodelica::Ref<DAE::Exp>,
    mut context: SimCodeFunction::Context,
) -> metamodelica::Ref<DAE::Exp> {
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    outExp = 'mc: {
        let __mc_input = &*inExp;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::MATRIX { ty: aty, matrix: rows @ Deref @ metamodelica::ListNode::Cons { head: Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Exp::CREF { componentRef: cr, .. }, tail: _ }, tail: _ }, .. } => {
                    let mut crefExp: metamodelica::Ref<DAE::Exp>;
                    let mut cr = (*cr).clone();
                    if '__try0: {
                        let SimCodeFunction::FUNCTION_CONTEXT { .. } = (context.clone()) else { break '__try0 Err::<_, _>("pattern mismatch") };
                        Ok::<(), &'static str>(())
                    }.is_ok() { return Err("failure(): body succeeded") }
                    ::match_deref::match_deref! { match &(ComponentReference::crefLastSubs(metamodelica::AsArg::as_arg(&cr))?) {
                        Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Subscript::INDEX { exp: Deref @ DAE::Exp::ICONST { integer: 1 } }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Subscript::INDEX { exp: Deref @ DAE::Exp::ICONST { integer: 1 } }, tail: Deref @ metamodelica::ListNode::Nil } } => (),
                        _ => return Err("pattern mismatch"),
                    } };
                    cr = ComponentReferenceBasics::crefStripLastSubs(metamodelica::AsArg::as_arg(&cr))?;
                    let true = (isMatrixExpansion(metamodelica::AsArg::as_arg(&rows), metamodelica::AsArg::as_arg(&cr), 1, 1)) else { return Err("pattern mismatch") };
                    crefExp = Expression::makeCrefExp(cr.clone(), aty.clone())?;
                    Ok(crefExp.clone())
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

fn isMatrixExpansion(
    mut rows: &metamodelica::List<metamodelica::List<metamodelica::Ref<DAE::Exp>>>,
    mut inCref: &metamodelica::Ref<DAE::ComponentRef>,
    mut rowIndex: i32,
    mut colIndex: i32,
) -> bool {
    let mut isExpanded: bool;
    isExpanded = 'mc: {
        let __mc_input = &**rows;
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
                Deref @ metamodelica::ListNode::Cons { head: Deref @ metamodelica::ListNode::Nil, tail: restRows } => {
                    Ok(isMatrixExpansion(metamodelica::AsArg::as_arg(&restRows), inCref, rowIndex + 1, 1))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Exp::CREF { componentRef: cr, .. }, tail: restElems }, tail: restRows } => {
                    let mut r: i32;
                    let mut c: i32;
                    let mut cr = (*cr).clone();
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(ComponentReference::crefLastSubs(metamodelica::AsArg::as_arg(&cr))?) {
                        Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Subscript::INDEX { exp: Deref @ DAE::Exp::ICONST { integer: __pa0 } }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Subscript::INDEX { exp: Deref @ DAE::Exp::ICONST { integer: __pa1 } }, tail: Deref @ metamodelica::ListNode::Nil } } => (__pa0.clone(), __pa1.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    r = metamodelica::Own::own(__pa0);
                    c = metamodelica::Own::own(__pa1);
                    let true = (r == rowIndex && c == colIndex) else { return Err("pattern mismatch") };
                    cr = ComponentReferenceBasics::crefStripLastSubs(metamodelica::AsArg::as_arg(&cr))?;
                    let true = (ComponentReferenceBasics::crefEqualNoStringCompare(inCref, metamodelica::AsArg::as_arg(&cr))?) else { return Err("pattern mismatch") };
                    Ok(isMatrixExpansion(&(metamodelica::cons(restElems.clone(), restRows.clone())), inCref, rowIndex, colIndex + 1))
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
    isExpanded
}

pub(crate) fn hackGetFirstExternalFunctionLib(mut libs: &metamodelica::List<ArcStr>) -> ArcStr {
    let mut outFirstLib: ArcStr;
    outFirstLib = 'mc: {
        let __mc_input = &**libs;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut lib: ArcStr;
                    lib = List::last(libs)?;
                    lib = System::stringReplace(lib.clone(), literal!("-l"), literal!(""))?;
                    Ok(lib.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(literal!("NO_LIB"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    outFirstLib
}

pub fn createAssertforSqrt(mut inExp: metamodelica::Ref<DAE::Exp>) -> Result<metamodelica::Ref<DAE::Exp>> {
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    outExp = (match &*inExp {
        _ => {
            (outExp, _) = ExpressionSimplify::simplify(metamodelica::Ref::new(DAE::Exp::RELATION {
                exp1: inExp,
                operator: DAE::Operator::GREATEREQ {
                    ty: DAE::T_REAL_DEFAULT().clone(),
                },
                exp2: metamodelica::Ref::new(DAE::Exp::RCONST {
                    real: metamodelica::OrderedFloat(0.0_f64),
                }),
                index: -1,
                optionExpisASUB: None,
            }))?;
            outExp
        }
    });
    Ok(outExp)
}

pub fn createDAEString(mut inString: ArcStr) -> metamodelica::Ref<DAE::Exp> {
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    outExp = metamodelica::Ref::new(DAE::Exp::SCONST { string: inString });
    outExp
}

/* end of TypeView published functions */
// =============================================================================
// section to generate SimCode from functions
//
// Finds the called functions in BackendDAE and transforms them to a list of
// libraries and a list of SimCodeFunction.Function uniontypes.
// =============================================================================
fn orderRecordDecls(
    mut decl1: &SimCodeFunction::RecordDeclaration,
    mut decl2: &SimCodeFunction::RecordDeclaration,
) -> Result<bool> {
    let mut b: bool;
    b = (match (decl1.clone(), decl2.clone()) {
        (
            SimCodeFunction::RecordDeclaration::RECORD_DECL_DEF { path: ref path1, .. },
            SimCodeFunction::RecordDeclaration::RECORD_DECL_DEF { path: ref path2, .. },
        ) => AbsynUtil::pathGe(path1.clone(), path2.clone())?,
        _ => true,
    });
    Ok(b)
}

pub fn elaborateFunctions(
    mut program: &Absyn::Program,
    mut daeElements: metamodelica::List<DAE::Function>,
    mut metarecordTypes: &metamodelica::List<metamodelica::Ref<DAE::Type>>,
    mut literals: &metamodelica::List<metamodelica::Ref<DAE::Exp>>,
    mut includes: metamodelica::List<ArcStr>,
) -> Result<(
    metamodelica::List<metamodelica::Ref<SimCodeFunction::Function::Function>>,
    metamodelica::List<SimCodeFunction::RecordDeclaration>,
    metamodelica::List<ArcStr>,
    metamodelica::List<ArcStr>,
    metamodelica::List<ArcStr>,
    metamodelica::List<ArcStr>,
)> {
    let mut functions: metamodelica::List<metamodelica::Ref<SimCodeFunction::Function::Function>>;
    let mut recordDecls: metamodelica::List<SimCodeFunction::RecordDeclaration>;
    let mut outIncludes: metamodelica::List<ArcStr>;
    let mut includeDirs: metamodelica::List<ArcStr>;
    let mut libs: metamodelica::List<ArcStr>;
    let mut libpaths: metamodelica::List<ArcStr>;
    let mut ht: (
        metamodelica::Array<metamodelica::List<(ArcStr, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<Option<(ArcStr, metamodelica::Ref<Absyn::Path>)>>,
        ),
        i32,
        (
            HashTableStringToPath::FuncHashCref,
            HashTableStringToPath::FuncCrefEqual,
            HashTableStringToPath::FuncCrefStr,
            HashTableStringToPath::FuncExpStr,
        ),
    );
    let mut g: metamodelica::List<(
        SimCodeFunction::RecordDeclaration,
        metamodelica::List<SimCodeFunction::RecordDeclaration>,
    )>;
    let mut recDeclsMap: metamodelica::Ref<UnorderedMap::UnorderedMap<ArcStr, SimCodeFunction::RecordDeclaration>>;
    recDeclsMap = UnorderedMap::new(
        (std::sync::Arc::new(fnptr!(stringHashDjb2, ArcStr))
            as std::sync::Arc<dyn ::std::ops::Fn(ArcStr) -> Result<i32> + 'static>),
        (std::sync::Arc::new(fnptr!(stringEq, ArcStr, ArcStr))
            as std::sync::Arc<dyn ::std::ops::Fn(ArcStr, ArcStr) -> Result<bool> + 'static>),
        1,
    );
    (functions, outIncludes, includeDirs, libs, libpaths) = elaborateFunctions2(
        program,
        daeElements,
        metamodelica::nil(),
        includes,
        metamodelica::nil(),
        metamodelica::nil(),
        metamodelica::nil(),
        recDeclsMap.clone(),
    )?;
    collectRecDeclsFromMetaRecCallExps(literals, recDeclsMap.clone())?;
    collectRecDeclsFromTypes(metarecordTypes, recDeclsMap.clone())?;
    addRecordDeclsForExtraConstructors(recDeclsMap.clone())?;
    recordDecls = UnorderedMap::valueList(recDeclsMap);
    recordDecls = List::sort(
        recordDecls,
        (std::sync::Arc::new(
            move |__a0: SimCodeFunction::RecordDeclaration, __a1: SimCodeFunction::RecordDeclaration| {
                orderRecordDecls(&__a0, &__a1)
            },
        )
            as std::sync::Arc<
                dyn ::std::ops::Fn(
                        SimCodeFunction::RecordDeclaration,
                        SimCodeFunction::RecordDeclaration,
                    ) -> Result<bool>
                    + 'static,
            >),
    )?;
    ht = HashTableStringToPath::emptyHashTableSized(BaseHashTable::lowBucketSize.clone());
    (recordDecls, _) = List::mapFold(&recordDecls, &aliasRecordDeclarations, ht)?;
    g = Graph::buildGraph(
        recordDecls.clone(),
        &move |__a0: SimCodeFunction::RecordDeclaration,
               __a1: metamodelica::List<SimCodeFunction::RecordDeclaration>| {
            getRecordDependencies(&__a0, __a1)
        },
        recordDecls,
    )?;
    let __pa0 = ::match_deref::match_deref! { match &(Graph::topologicalSort(&g, (std::sync::Arc::new(move |__a0: SimCodeFunction::RecordDeclaration, __a1: SimCodeFunction::RecordDeclaration| -> metamodelica::Result<_> { ::std::result::Result::Ok(isRecordDeclEqual(&__a0, &__a1)) }) as std::sync::Arc<dyn ::std::ops::Fn(SimCodeFunction::RecordDeclaration, SimCodeFunction::RecordDeclaration) -> Result<bool> + 'static>))?) {
        (__pa0, Deref @ metamodelica::ListNode::Nil) => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    recordDecls = metamodelica::Own::own(__pa0);
    Ok((functions, recordDecls, outIncludes, includeDirs, libs, libpaths))
}

fn addRecordDeclsForExtraConstructors(
    mut recDeclsMap: metamodelica::Ref<UnorderedMap::UnorderedMap<ArcStr, SimCodeFunction::RecordDeclaration>>,
) -> Result<()> {
    for mut decl in &*UnorderedMap::valueList(recDeclsMap.clone()) {
        let () = (match decl.clone() {
            SimCodeFunction::RecordDeclaration::RECORD_DECL_ADD_CONSTRCTOR { .. }
                if (!(UnorderedMap::contains(
                    var_field!(
                        decl.name,
                        SimCodeFunction::RecordDeclaration::RECORD_DECL_ADD_CONSTRCTOR
                    )
                    .clone(),
                    recDeclsMap.clone(),
                )?)) =>
            {
                UnorderedMap::add(
                    var_field!(
                        decl.name,
                        SimCodeFunction::RecordDeclaration::RECORD_DECL_ADD_CONSTRCTOR
                    )
                    .clone(),
                    SimCodeFunction::RecordDeclaration::RECORD_DECL_FULL {
                        name: var_field!(
                            decl.name,
                            SimCodeFunction::RecordDeclaration::RECORD_DECL_ADD_CONSTRCTOR
                        )
                        .clone(),
                        aliasName: None,
                        defPath: var_field!(
                            decl.defPath,
                            SimCodeFunction::RecordDeclaration::RECORD_DECL_ADD_CONSTRCTOR
                        )
                        .clone(),
                        variables: ({
                            let mut __acc: metamodelica::List<metamodelica::Ref<SimCodeFunction::Variable::Variable>> =
                                metamodelica::nil();
                            for mut v in (var_field!(
                                decl.variables,
                                SimCodeFunction::RecordDeclaration::RECORD_DECL_ADD_CONSTRCTOR
                            )
                            .clone())
                            .into_iter()
                            .cloned()
                            {
                                let __x = variableWithoutBinding(v.clone());
                                __acc = cons(__x, __acc);
                            }
                            __acc.reverse()
                        }),
                        usedExternally: var_field!(
                            decl.usedExternally,
                            SimCodeFunction::RecordDeclaration::RECORD_DECL_ADD_CONSTRCTOR
                        )
                        .clone(),
                    },
                    recDeclsMap.clone(),
                )?;
                ()
            }
            _ => (),
        });
    }
    Ok(())
}

fn variableWithoutBinding(
    mut var: metamodelica::Ref<SimCodeFunction::Variable::Variable>,
) -> metamodelica::Ref<SimCodeFunction::Variable::Variable> {
    let mut var: metamodelica::Ref<SimCodeFunction::Variable::Variable> = var;
    let () = (match &*var {
        SimCodeFunction::Variable::VARIABLE { .. } => {
            assign_variant_field!(var => SimCodeFunction::Variable::Variable::VARIABLE;
                value = None,
                bind_from_outside = false
            );
            ()
        }
        _ => (),
    });
    var
}

fn getRecordDependencies(
    mut decl: &SimCodeFunction::RecordDeclaration,
    mut allDecls: metamodelica::List<SimCodeFunction::RecordDeclaration>,
) -> Result<metamodelica::List<SimCodeFunction::RecordDeclaration>> {
    let mut dependencies: metamodelica::List<SimCodeFunction::RecordDeclaration>;
    dependencies = (match decl.clone() {
        SimCodeFunction::RecordDeclaration::RECORD_DECL_FULL {
            aliasName: Some(mut name),
            ..
        } => List::select1(
            allDecls,
            (std::sync::Arc::new(
                move |__a0: SimCodeFunction::RecordDeclaration, __a1: ArcStr| -> metamodelica::Result<_> {
                    ::std::result::Result::Ok(recordDeclHasName(&__a0, &__a1))
                },
            )
                as std::sync::Arc<
                    dyn ::std::ops::Fn(SimCodeFunction::RecordDeclaration, ArcStr) -> Result<bool> + 'static,
                >),
            name.clone(),
        )?,
        SimCodeFunction::RecordDeclaration::RECORD_DECL_ADD_CONSTRCTOR { name: mut name, .. } => List::select1(
            allDecls,
            (std::sync::Arc::new(
                move |__a0: SimCodeFunction::RecordDeclaration, __a1: ArcStr| -> metamodelica::Result<_> {
                    ::std::result::Result::Ok(recordDeclHasName(&__a0, &__a1))
                },
            )
                as std::sync::Arc<
                    dyn ::std::ops::Fn(SimCodeFunction::RecordDeclaration, ArcStr) -> Result<bool> + 'static,
                >),
            name.clone(),
        )?,
        SimCodeFunction::RecordDeclaration::RECORD_DECL_FULL {
            variables: ref vars, ..
        } => {
            let mut tys: metamodelica::List<metamodelica::Ref<DAE::Type>>;
            let mut tyss: metamodelica::List<metamodelica::List<metamodelica::Ref<DAE::Type>>>;
            tys = ({
                let mut __acc: metamodelica::List<metamodelica::Ref<DAE::Type>> = metamodelica::nil();
                for mut v in (vars.clone()).into_iter().cloned() {
                    let __x = getVarType(&(v.clone()));
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            });
            tyss = List::map1(tys, &move |__a0: metamodelica::Ref<DAE::Type>, __a1: Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Type>) -> Result<bool> + 'static>| Types::getAllInnerTypesOfType(__a0, metamodelica::arc_ref(&__a1)), std::sync::Arc::new(fnptr!(Util::anyReturnTrue, _)))?;
            tys = List::flatten(tyss)?;
            dependencies = List::filterMap1(
                &tys,
                &move |__a0: metamodelica::Ref<DAE::Type>,
                       __a1: metamodelica::List<SimCodeFunction::RecordDeclaration>| {
                    getRecordDependenciesFromType(&__a0, &__a1)
                },
                allDecls,
            );
            List::unique(&dependencies)
        }
        _ => metamodelica::nil(),
    });
    Ok(dependencies)
}

fn getVarType(mut var: &metamodelica::Ref<SimCodeFunction::Variable::Variable>) -> metamodelica::Ref<DAE::Type> {
    let mut ty: metamodelica::Ref<DAE::Type>;
    ty = (match &**var {
        SimCodeFunction::Variable::VARIABLE { ty: __esc_ty, .. } => {
            ty = (*__esc_ty).clone();
            ty.clone()
        }
        _ => DAE::T_ANYTYPE_DEFAULT().clone(),
    });
    ty
}

fn getRecordDependenciesFromType(
    mut ty: &metamodelica::Ref<DAE::Type>,
    mut allDecls: &metamodelica::List<SimCodeFunction::RecordDeclaration>,
) -> Result<SimCodeFunction::RecordDeclaration> {
    let mut decl: SimCodeFunction::RecordDeclaration;
    let mut path: metamodelica::Ref<Absyn::Path>;
    let mut name: ArcStr;
    let __pa0 = ::match_deref::match_deref! { match &((*ty)) {
        Deref @ DAE::Type::T_COMPLEX { complexClassType: ClassInf::State::RECORD { path: __pa0 }, .. } => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    path = metamodelica::Own::own(__pa0);
    name = AbsynUtil::pathStringUnquoteReplaceDot(&path, literal!("_"))?;
    decl = List::find1(
        allDecls,
        &move |__a0: SimCodeFunction::RecordDeclaration, __a1: ArcStr| -> metamodelica::Result<_> {
            ::std::result::Result::Ok(recordDeclHasName(&__a0, &__a1))
        },
        name,
    )?;
    Ok(decl)
}

pub fn setTrivialRecords(mut recordDecls: &metamodelica::List<SimCodeFunction::RecordDeclaration>) -> Result<()> {
    let mut trivial: metamodelica::List<ArcStr> = metamodelica::nil();
    for mut decl in &**recordDecls {
        let () = (match decl.clone() {
            SimCodeFunction::RecordDeclaration::RECORD_DECL_FULL { .. }
                if (List::all(
                    var_field!(decl.variables, SimCodeFunction::RecordDeclaration::RECORD_DECL_FULL),
                    &({
                        let __pe_b1 = recordDecls.clone();
                        let __pe_b2 = trivial.clone();
                        move |__pe_a0| Ok(isTrivialRecordMember(&__pe_a0, &__pe_b1, __pe_b2.clone()))
                    }),
                )?) =>
            {
                trivial = metamodelica::cons(
                    var_field!(decl.name, SimCodeFunction::RecordDeclaration::RECORD_DECL_FULL).clone(),
                    trivial,
                );
                ()
            }
            _ => (),
        });
    }
    {
        let __v = trivial;
        openmodelica_util::Globals::trivialRecords.with(|__root| *__root.borrow_mut() = __v)
    };
    Ok(())
}

pub fn isTrivialRecord(mut name: ArcStr) -> bool {
    let mut b: bool;
    b = listMember(
        name,
        openmodelica_util::Globals::trivialRecords.with(|__root| __root.borrow().clone()),
    );
    b
}

fn isTrivialRecordMember(
    mut var: &metamodelica::Ref<SimCodeFunction::Variable::Variable>,
    mut allDecls: &metamodelica::List<SimCodeFunction::RecordDeclaration>,
    mut trivial: metamodelica::List<ArcStr>,
) -> bool {
    let mut b: bool;
    let mut name: ArcStr;
    b = (match &*(getVarType(var)) {
        DAE::Type::T_REAL { .. } => true,
        DAE::Type::T_INTEGER { .. } => true,
        DAE::Type::T_BOOL { .. } => true,
        DAE::Type::T_ENUMERATION { .. } => true,
        DAE::Type::T_COMPLEX {
            complexClassType: ClassInf::State::RECORD { .. },
            ..
        } => {
            match '__try0: {
                let SimCodeFunction::RECORD_DECL_FULL { name: __pa1, .. } =
                    (unwrap_break_err!(getRecordDependenciesFromType(&(getVarType(var)), allDecls), '__try0))
                else {
                    break '__try0 Err::<_, _>("pattern mismatch");
                };
                name = metamodelica::Own::own(__pa1);
                b = listMember(name.clone(), trivial.clone());
                Ok::<_, &'static str>((b.clone(),))
            } {
                Ok((__try0_o0,)) => {
                    b = __try0_o0;
                }
                Err(_) => {
                    b = false;
                }
            }
            b
        }
        _ => false,
    });
    b
}

fn recordDeclHasName(mut decl: &SimCodeFunction::RecordDeclaration, mut name: &ArcStr) -> bool {
    let mut b: bool;
    b = (match decl.clone() {
        SimCodeFunction::RecordDeclaration::RECORD_DECL_FULL { .. } => stringEq(
            &name,
            &var_field!(decl.name, SimCodeFunction::RecordDeclaration::RECORD_DECL_FULL),
        ),
        _ => false,
    });
    b
}

fn isRecordDeclEqual(
    mut decl1: &SimCodeFunction::RecordDeclaration,
    mut decl2: &SimCodeFunction::RecordDeclaration,
) -> bool {
    let mut b: bool;
    b = (match (decl1.clone(), decl2.clone()) {
        (
            SimCodeFunction::RecordDeclaration::RECORD_DECL_FULL { .. },
            SimCodeFunction::RecordDeclaration::RECORD_DECL_FULL { .. },
        ) => stringEq(
            &var_field!(decl1.name, SimCodeFunction::RecordDeclaration::RECORD_DECL_FULL),
            &var_field!(decl2.name, SimCodeFunction::RecordDeclaration::RECORD_DECL_FULL),
        ),
        (
            SimCodeFunction::RecordDeclaration::RECORD_DECL_DEF { .. },
            SimCodeFunction::RecordDeclaration::RECORD_DECL_DEF { .. },
        ) => AbsynUtil::pathEqual(
            var_field!(decl1.path, SimCodeFunction::RecordDeclaration::RECORD_DECL_DEF),
            var_field!(decl2.path, SimCodeFunction::RecordDeclaration::RECORD_DECL_DEF),
        ),
        _ => false,
    });
    b
}

fn elaborateFunctions2<'__b>(
    mut program: &'__b Absyn::Program,
    mut daeElements: metamodelica::List<DAE::Function>,
    mut inFunctions: metamodelica::List<metamodelica::Ref<SimCodeFunction::Function::Function>>,
    mut inIncludes: metamodelica::List<ArcStr>,
    mut inIncludeDirs: metamodelica::List<ArcStr>,
    mut inLibs: metamodelica::List<ArcStr>,
    mut inPaths: metamodelica::List<ArcStr>,
    mut recDeclsMap: metamodelica::Ref<UnorderedMap::UnorderedMap<ArcStr, SimCodeFunction::RecordDeclaration>>,
) -> Result<(
    metamodelica::List<metamodelica::Ref<SimCodeFunction::Function::Function>>,
    metamodelica::List<ArcStr>,
    metamodelica::List<ArcStr>,
    metamodelica::List<ArcStr>,
    metamodelica::List<ArcStr>,
)> {
    '__tco: loop {
        ::match_deref::match_deref! { match &((daeElements, inFunctions, inIncludes, inIncludeDirs, inLibs, inPaths)) {
            (Deref @ metamodelica::ListNode::Nil, accfns, includes, includeDirs, libs, libPaths) => {
                return Ok((accfns.clone().reverse(), includes.clone(), includeDirs.clone(), libs.clone(), libPaths.clone()))
            },
            (Deref @ metamodelica::ListNode::Cons { head: DAE::Function::FUNCTION { type_: Deref @ DAE::Type::T_FUNCTION { functionAttributes: DAE::FunctionAttributes { isBuiltin: DAE::FunctionBuiltin::FUNCTION_BUILTIN_PTR { .. }, .. }, .. }, .. }, tail: rest }, accfns, includes, includeDirs, libs, libPaths) => {
                let mut fns: metamodelica::List<metamodelica::Ref<SimCodeFunction::Function::Function>>;
                let mut includes = (*includes).clone();
                let mut includeDirs = (*includeDirs).clone();
                let mut libs = (*libs).clone();
                let mut libPaths = (*libPaths).clone();
                { (program, daeElements, inFunctions, inIncludes, inIncludeDirs, inLibs, inPaths, recDeclsMap) = (program, rest.clone(), accfns.clone(), includes.clone(), includeDirs.clone(), libs.clone(), libPaths.clone(), recDeclsMap); continue '__tco; }
            },
            (Deref @ metamodelica::ListNode::Cons { head: DAE::Function::FUNCTION { partialPrefix: true, .. }, tail: rest }, accfns, includes, includeDirs, libs, libPaths) => {
                let mut fns: metamodelica::List<metamodelica::Ref<SimCodeFunction::Function::Function>>;
                let mut includes = (*includes).clone();
                let mut includeDirs = (*includeDirs).clone();
                let mut libs = (*libs).clone();
                let mut libPaths = (*libPaths).clone();
                { (program, daeElements, inFunctions, inIncludes, inIncludeDirs, inLibs, inPaths, recDeclsMap) = (program, rest.clone(), accfns.clone(), includes.clone(), includeDirs.clone(), libs.clone(), libPaths.clone(), recDeclsMap); continue '__tco; }
            },
            (Deref @ metamodelica::ListNode::Cons { head: fel @ DAE::Function::FUNCTION { path, functions: Deref @ metamodelica::ListNode::Cons { head: DAE::FunctionDefinition::FUNCTION_EXT { externalDecl: DAE::ExternalDecl { name, language: Deref @ "builtin", .. }, .. }, tail: _ }, .. }, tail: rest }, accfns, includes, includeDirs, libs, libPaths) => {
                let mut b: bool;
                let mut fns: metamodelica::List<metamodelica::Ref<SimCodeFunction::Function::Function>>;
                let mut r#fn: metamodelica::Ref<SimCodeFunction::Function::Function>;
                let mut fname: ArcStr;
                let mut accfns = (*accfns).clone();
                let mut includes = (*includes).clone();
                let mut includeDirs = (*includeDirs).clone();
                let mut libs = (*libs).clone();
                let mut libPaths = (*libPaths).clone();
                fname = AbsynUtil::pathString(AbsynUtil::makeNotFullyQualified(path.clone()), literal!("."), true, false)?;
                b = stringEq(&fname, &name);
                if !(b) {
                    (r#fn, includes, includeDirs, libs, libPaths) = elaborateFunction(program, fel.clone(), includes.clone(), includeDirs.clone(), libs.clone(), libPaths.clone(), recDeclsMap.clone())?;
                    accfns = metamodelica::cons(r#fn, accfns.clone());
                }
                { (program, daeElements, inFunctions, inIncludes, inIncludeDirs, inLibs, inPaths, recDeclsMap) = (program, rest.clone(), accfns.clone(), includes.clone(), includeDirs.clone(), libs.clone(), libPaths.clone(), recDeclsMap); continue '__tco; }
            },
            (Deref @ metamodelica::ListNode::Cons { head: fel @ DAE::Function::FUNCTION { path, functions: Deref @ metamodelica::ListNode::Cons { head: DAE::FunctionDefinition::FUNCTION_EXT { externalDecl: DAE::ExternalDecl { name, language: Deref @ "C", .. }, .. }, tail: _ }, .. }, tail: rest }, accfns, includes, includeDirs, libs, libPaths) => {
                let mut b: bool;
                let mut fns: metamodelica::List<metamodelica::Ref<SimCodeFunction::Function::Function>>;
                let mut r#fn: metamodelica::Ref<SimCodeFunction::Function::Function>;
                let mut fname: ArcStr;
                let mut accfns = (*accfns).clone();
                let mut includes = (*includes).clone();
                let mut includeDirs = (*includeDirs).clone();
                let mut libs = (*libs).clone();
                let mut libPaths = (*libPaths).clone();
                fname = AbsynUtil::pathString(AbsynUtil::makeNotFullyQualified(path.clone()), literal!("."), true, false)?;
                b = listMember(name.clone(), SCodeUtil::knownExternalCFunctions.clone()) && stringEq(&fname, &name);
                if !(b) {
                    (r#fn, includes, includeDirs, libs, libPaths) = elaborateFunction(program, fel.clone(), includes.clone(), includeDirs.clone(), libs.clone(), libPaths.clone(), recDeclsMap.clone())?;
                    accfns = metamodelica::cons(r#fn, accfns.clone());
                }
                { (program, daeElements, inFunctions, inIncludes, inIncludeDirs, inLibs, inPaths, recDeclsMap) = (program, rest.clone(), accfns.clone(), includes.clone(), includeDirs.clone(), libs.clone(), libPaths.clone(), recDeclsMap); continue '__tco; }
            },
            (Deref @ metamodelica::ListNode::Cons { head: fel, tail: rest }, accfns, includes, includeDirs, libs, libPaths) => {
                let mut fns: metamodelica::List<metamodelica::Ref<SimCodeFunction::Function::Function>>;
                let mut r#fn: metamodelica::Ref<SimCodeFunction::Function::Function>;
                let mut includes = (*includes).clone();
                let mut includeDirs = (*includeDirs).clone();
                let mut libs = (*libs).clone();
                let mut libPaths = (*libPaths).clone();
                (r#fn, includes, includeDirs, libs, libPaths) = elaborateFunction(program, fel.clone(), includes.clone(), includeDirs.clone(), libs.clone(), libPaths.clone(), recDeclsMap.clone())?;
                { (program, daeElements, inFunctions, inIncludes, inIncludeDirs, inLibs, inPaths, recDeclsMap) = (program, rest.clone(), metamodelica::cons(r#fn, accfns.clone()), includes.clone(), includeDirs.clone(), libs.clone(), libPaths.clone(), recDeclsMap); continue '__tco; }
            },
            _ => return Err("match: no arm matched"),
        } }
    }
}

/* Does the actual work of transforming a DAE.FUNCTION to a SimCodeFunction.Function. */
fn elaborateFunction(
    mut program: &Absyn::Program,
    mut inElement: DAE::Function,
    mut inIncludes: metamodelica::List<ArcStr>,
    mut inIncludeDirs: metamodelica::List<ArcStr>,
    mut inLibs: metamodelica::List<ArcStr>,
    mut inLibPaths: metamodelica::List<ArcStr>,
    mut recDeclsMap: metamodelica::Ref<UnorderedMap::UnorderedMap<ArcStr, SimCodeFunction::RecordDeclaration>>,
) -> Result<(
    metamodelica::Ref<SimCodeFunction::Function::Function>,
    metamodelica::List<ArcStr>,
    metamodelica::List<ArcStr>,
    metamodelica::List<ArcStr>,
    metamodelica::List<ArcStr>,
)> {
    let mut outFunction: metamodelica::Ref<SimCodeFunction::Function::Function>;
    let mut outIncludes: metamodelica::List<ArcStr>;
    let mut outIncludeDirs: metamodelica::List<ArcStr>;
    let mut outLibs: metamodelica::List<ArcStr>;
    let mut outLibPaths: metamodelica::List<ArcStr>;
    (outFunction, outIncludes, outIncludeDirs, outLibs, outLibPaths) = 'mc: {
        let __mc_input = (inElement, inIncludes, inIncludeDirs, inLibs, inLibPaths);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                        (DAE::Function::FUNCTION { path: fpath, source, visibility, functions: Deref @ metamodelica::ListNode::Cons { head: DAE::FunctionDefinition::FUNCTION_DEF { body: daeElts }, tail: _ }, type_: Deref @ DAE::Type::T_FUNCTION { funcArg: args, functionAttributes: funAttrs, .. }, partialPrefix: false, .. }, includes, includeDirs, libs, libPaths) => {
                            let mut vars: metamodelica::List<metamodelica::Ref<DAE::Element>>;
                            let mut outVars: metamodelica::List<metamodelica::Ref<SimCodeFunction::Variable::Variable>>;
                            let mut funArgs: metamodelica::List<metamodelica::Ref<SimCodeFunction::Variable::Variable>>;
                            let mut varDecls: metamodelica::List<metamodelica::Ref<SimCodeFunction::Variable::Variable>>;
                            let mut bodyStmts: metamodelica::List<metamodelica::Ref<DAE::Statement>>;
                            let mut info: SourceInfo;
                            let mut daeElts = (*daeElts).clone();
                            let DAE::FUNCTION_ATTRIBUTES { functionParallelism: DAE::FP_NON_PARALLEL { .. }, .. } = (funAttrs.clone()) else { return Err("pattern mismatch") };
                            daeElts = optMRFAElems(daeElts.clone())?;
                            outVars = List::map(DAEUtil::getOutputElements(daeElts.clone())?, &move |__a0: metamodelica::Ref<DAE::Element>| daeInOutSimVar(&__a0))?;
                            funArgs = List::map1(args.clone(), &move |__a0: metamodelica::Ref<DAE::FuncArg>, __a1: Option<metamodelica::Ref<DAE::Exp>>| typesSimFunctionArg(&__a0, __a1), None)?;
                            collectRecDeclsFromElems(metamodelica::AsArg::as_arg(&daeElts), recDeclsMap.clone())?;
                            vars = List::filterOnTrue(daeElts.clone(), (std::sync::Arc::new(move |__a0: metamodelica::Ref<DAE::Element>| -> metamodelica::Result<_> { ::std::result::Result::Ok(isVarQ(&__a0)) }) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Element>) -> Result<bool> + 'static>))?;
                            varDecls = List::map(vars.clone(), &move |__a0: metamodelica::Ref<DAE::Element>| daeInOutSimVar(&__a0))?;
                            bodyStmts = ({
                let mut __acc: metamodelica::List<metamodelica::Ref<DAE::Statement>> = metamodelica::nil();
                for mut e in (daeElts.clone()).into_iter().cloned() {
                            if !(DAEUtil::isAlgorithm(&(e.clone()))) { continue; }
                            let __x = elaborateStatement(&(e.clone()))?;
                            __acc = __x.append(&__acc);
                }
                __acc
            });
                            info = ElementSource::getElementSourceFileInfo(source.clone());
                            Ok((metamodelica::Ref::new(SimCodeFunction::Function::Function::FUNCTION { name: fpath.clone(), outVars: outVars.clone(), functionArguments: funArgs.clone(), variableDeclarations: varDecls.clone(), body: bodyStmts.clone(), visibility: visibility.clone(), info: info.clone() }), includes.clone(), includeDirs.clone(), libs.clone(), libPaths.clone()))
                        }
                        _ => return Err("nomatch"),
                    }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                        (DAE::Function::FUNCTION { path: fpath, source, functions: Deref @ metamodelica::ListNode::Cons { head: DAE::FunctionDefinition::FUNCTION_DEF { body: daeElts }, tail: _ }, type_: Deref @ DAE::Type::T_FUNCTION { funcArg: args, functionAttributes: funAttrs, .. }, partialPrefix: false, .. }, includes, includeDirs, libs, libPaths) => {
                            let mut vars: metamodelica::List<metamodelica::Ref<DAE::Element>>;
                            let mut outVars: metamodelica::List<metamodelica::Ref<SimCodeFunction::Variable::Variable>>;
                            let mut funArgs: metamodelica::List<metamodelica::Ref<SimCodeFunction::Variable::Variable>>;
                            let mut varDecls: metamodelica::List<metamodelica::Ref<SimCodeFunction::Variable::Variable>>;
                            let mut bodyStmts: metamodelica::List<metamodelica::Ref<DAE::Statement>>;
                            let mut info: SourceInfo;
                            let mut daeElts = (*daeElts).clone();
                            let DAE::FUNCTION_ATTRIBUTES { functionParallelism: DAE::FP_KERNEL_FUNCTION { .. }, .. } = (funAttrs.clone()) else { return Err("pattern mismatch") };
                            daeElts = optMRFAElems(daeElts.clone())?;
                            outVars = List::map(DAEUtil::getOutputElements(daeElts.clone())?, &move |__a0: metamodelica::Ref<DAE::Element>| daeInOutSimVar(&__a0))?;
                            funArgs = List::map1(args.clone(), &move |__a0: metamodelica::Ref<DAE::FuncArg>, __a1: Option<metamodelica::Ref<DAE::Exp>>| typesSimFunctionArg(&__a0, __a1), None)?;
                            collectRecDeclsFromElems(metamodelica::AsArg::as_arg(&daeElts), recDeclsMap.clone())?;
                            vars = List::filterOnTrue(daeElts.clone(), (std::sync::Arc::new(move |__a0: metamodelica::Ref<DAE::Element>| -> metamodelica::Result<_> { ::std::result::Result::Ok(isVarNotInputNotOutput(&__a0)) }) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Element>) -> Result<bool> + 'static>))?;
                            varDecls = List::map(vars.clone(), &move |__a0: metamodelica::Ref<DAE::Element>| daeInOutSimVar(&__a0))?;
                            bodyStmts = ({
                let mut __acc: metamodelica::List<metamodelica::Ref<DAE::Statement>> = metamodelica::nil();
                for mut e in (daeElts.clone()).into_iter().cloned() {
                            if !(DAEUtil::isAlgorithm(&(e.clone()))) { continue; }
                            let __x = elaborateStatement(&(e.clone()))?;
                            __acc = __x.append(&__acc);
                }
                __acc
            });
                            info = ElementSource::getElementSourceFileInfo(source.clone());
                            Ok((metamodelica::Ref::new(SimCodeFunction::Function::Function::KERNEL_FUNCTION { name: fpath.clone(), outVars: outVars.clone(), functionArguments: funArgs.clone(), variableDeclarations: varDecls.clone(), body: bodyStmts.clone(), info: info.clone() }), includes.clone(), includeDirs.clone(), libs.clone(), libPaths.clone()))
                        }
                        _ => return Err("nomatch"),
                    }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                        (DAE::Function::FUNCTION { path: fpath, source, functions: Deref @ metamodelica::ListNode::Cons { head: DAE::FunctionDefinition::FUNCTION_DEF { body: daeElts }, tail: _ }, type_: Deref @ DAE::Type::T_FUNCTION { funcArg: args, functionAttributes: funAttrs, .. }, partialPrefix: false, .. }, includes, includeDirs, libs, libPaths) => {
                            let mut vars: metamodelica::List<metamodelica::Ref<DAE::Element>>;
                            let mut outVars: metamodelica::List<metamodelica::Ref<SimCodeFunction::Variable::Variable>>;
                            let mut funArgs: metamodelica::List<metamodelica::Ref<SimCodeFunction::Variable::Variable>>;
                            let mut varDecls: metamodelica::List<metamodelica::Ref<SimCodeFunction::Variable::Variable>>;
                            let mut bodyStmts: metamodelica::List<metamodelica::Ref<DAE::Statement>>;
                            let mut info: SourceInfo;
                            let mut daeElts = (*daeElts).clone();
                            let DAE::FUNCTION_ATTRIBUTES { functionParallelism: DAE::FP_PARALLEL_FUNCTION { .. }, .. } = (funAttrs.clone()) else { return Err("pattern mismatch") };
                            daeElts = optMRFAElems(daeElts.clone())?;
                            outVars = List::map(DAEUtil::getOutputElements(daeElts.clone())?, &move |__a0: metamodelica::Ref<DAE::Element>| daeInOutSimVar(&__a0))?;
                            funArgs = List::map1(args.clone(), &move |__a0: metamodelica::Ref<DAE::FuncArg>, __a1: Option<metamodelica::Ref<DAE::Exp>>| typesSimFunctionArg(&__a0, __a1), None)?;
                            collectRecDeclsFromElems(metamodelica::AsArg::as_arg(&daeElts), recDeclsMap.clone())?;
                            vars = List::filterOnTrue(daeElts.clone(), (std::sync::Arc::new(move |__a0: metamodelica::Ref<DAE::Element>| -> metamodelica::Result<_> { ::std::result::Result::Ok(isVarQ(&__a0)) }) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Element>) -> Result<bool> + 'static>))?;
                            varDecls = List::map(vars.clone(), &move |__a0: metamodelica::Ref<DAE::Element>| daeInOutSimVar(&__a0))?;
                            bodyStmts = ({
                let mut __acc: metamodelica::List<metamodelica::Ref<DAE::Statement>> = metamodelica::nil();
                for mut e in (daeElts.clone()).into_iter().cloned() {
                            if !(DAEUtil::isAlgorithm(&(e.clone()))) { continue; }
                            let __x = elaborateStatement(&(e.clone()))?;
                            __acc = __x.append(&__acc);
                }
                __acc
            });
                            info = ElementSource::getElementSourceFileInfo(source.clone());
                            Ok((metamodelica::Ref::new(SimCodeFunction::Function::Function::PARALLEL_FUNCTION { name: fpath.clone(), outVars: outVars.clone(), functionArguments: funArgs.clone(), variableDeclarations: varDecls.clone(), body: bodyStmts.clone(), info: info.clone() }), includes.clone(), includeDirs.clone(), libs.clone(), libPaths.clone()))
                        }
                        _ => return Err("nomatch"),
                    }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (DAE::Function::FUNCTION { path: fpath, source, visibility, functions: Deref @ metamodelica::ListNode::Cons { head: DAE::FunctionDefinition::FUNCTION_EXT { body: daeElts, externalDecl: extdecl }, tail: _ }, type_: Deref @ DAE::Type::T_FUNCTION { funcArg: args, .. }, .. }, includes, includeDirs, libs, libPaths) => {
                    let mut extfnname: ArcStr;
                    let mut lang: ArcStr;
                    let mut fn_libs: metamodelica::List<ArcStr>;
                    let mut fn_paths: metamodelica::List<ArcStr>;
                    let mut fn_includes: metamodelica::List<ArcStr>;
                    let mut fn_includeDirs: metamodelica::List<ArcStr>;
                    let mut extargs: metamodelica::List<DAE::ExtArg>;
                    let mut simextargs: metamodelica::List<metamodelica::Ref<SimCodeFunction::SimExtArg::SimExtArg>>;
                    let mut extReturn: metamodelica::Ref<SimCodeFunction::SimExtArg::SimExtArg>;
                    let mut extretarg: DAE::ExtArg;
                    let mut ann: Option<metamodelica::Ref<SCode::Annotation>>;
                    let mut outVars: metamodelica::List<metamodelica::Ref<SimCodeFunction::Variable::Variable>>;
                    let mut inVars: metamodelica::List<metamodelica::Ref<SimCodeFunction::Variable::Variable>>;
                    let mut biVars: metamodelica::List<metamodelica::Ref<SimCodeFunction::Variable::Variable>>;
                    let mut funArgs: metamodelica::List<metamodelica::Ref<SimCodeFunction::Variable::Variable>>;
                    let mut info: SourceInfo;
                    let mut dynamicLoad: bool;
                    let mut includes = (*includes).clone();
                    let mut includeDirs = (*includeDirs).clone();
                    let mut libs = (*libs).clone();
                    let mut libPaths = (*libPaths).clone();
                    let DAE::EXTERNALDECL { name: __pa0, args: __pa1, returnArg: __pa2, language: __pa3, ann: __pa4 } = &extdecl;
                    extfnname = metamodelica::Own::own(__pa0);
                    extargs = metamodelica::Own::own(__pa1);
                    extretarg = metamodelica::Own::own(__pa2);
                    lang = metamodelica::Own::own(__pa3);
                    ann = metamodelica::Own::own(__pa4);
                    funArgs = List::map1(args.clone(), &move |__a0: metamodelica::Ref<DAE::FuncArg>, __a1: Option<metamodelica::Ref<DAE::Exp>>| typesSimFunctionArg(&__a0, __a1), None)?;
                    outVars = List::map(DAEUtil::getOutputElements(daeElts.clone())?, &move |__a0: metamodelica::Ref<DAE::Element>| daeInOutSimVar(&__a0))?;
                    inVars = List::map(DAEUtil::getInputVars(daeElts.clone())?, &move |__a0: metamodelica::Ref<DAE::Element>| daeInOutSimVar(&__a0))?;
                    biVars = List::map(DAEUtil::getBidirElements(daeElts.clone())?, &move |__a0: metamodelica::Ref<DAE::Element>| daeInOutSimVar(&__a0))?;
                    collectRecDeclsFromElems(metamodelica::AsArg::as_arg(&daeElts), recDeclsMap.clone())?;
                    info = ElementSource::getElementSourceFileInfo(source.clone());
                    (fn_includes, fn_includeDirs, fn_libs, fn_paths, dynamicLoad) = generateExtFunctionIncludes(program, metamodelica::AsArg::as_arg(&fpath), ann.clone(), &info)?;
                    includes = List::union(&fn_includes, metamodelica::AsArg::as_arg(&includes));
                    includeDirs = List::union(&fn_includeDirs, metamodelica::AsArg::as_arg(&includeDirs));
                    libs = List::union(&fn_libs, metamodelica::AsArg::as_arg(&libs));
                    libPaths = List::union(&fn_paths, metamodelica::AsArg::as_arg(&libPaths));
                    simextargs = List::map(extargs.clone(), &move |__a0: DAE::ExtArg| extArgsToSimExtArgs(&__a0))?;
                    extReturn = extArgsToSimExtArgs(&extretarg)?;
                    (simextargs, extReturn) = fixOutputIndex(outVars.clone(), simextargs.clone(), extReturn.clone())?;
                    lang = System::toupper(lang.clone());
                    Ok((metamodelica::Ref::new(SimCodeFunction::Function::Function::EXTERNAL_FUNCTION { name: fpath.clone(), extName: extfnname.clone(), funArgs: funArgs.clone(), extArgs: simextargs.clone(), extReturn: extReturn.clone(), inVars: inVars.clone(), outVars: outVars.clone(), biVars: biVars.clone(), includes: fn_includes.clone(), libs: fn_libs.clone(), language: lang.clone(), visibility: visibility.clone(), info: info.clone(), dynamicLoad: dynamicLoad }), includes.clone(), includeDirs.clone(), libs.clone(), libPaths.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (DAE::Function::RECORD_CONSTRUCTOR { source, type_: Deref @ DAE::Type::T_FUNCTION { funcArg: args, funcResultType: restype @ Deref @ DAE::Type::T_COMPLEX { complexClassType: ClassInf::State::RECORD { path: name }, .. }, .. }, .. }, includes, includeDirs, libs, libPaths) => {
                    let mut funArgs: metamodelica::List<metamodelica::Ref<SimCodeFunction::Variable::Variable>>;
                    let mut varDecls: metamodelica::List<metamodelica::Ref<SimCodeFunction::Variable::Variable>>;
                    let mut info: SourceInfo;
                    let mut varlst: metamodelica::List<metamodelica::Ref<DAE::Var>>;
                    funArgs = List::map1(args.clone(), &move |__a0: metamodelica::Ref<DAE::FuncArg>, __a1: Option<metamodelica::Ref<DAE::Exp>>| typesSimFunctionArg(&__a0, __a1), None)?;
                    collectRecDeclsFromType(metamodelica::AsArg::as_arg(&restype), recDeclsMap.clone())?;
                    let __pa0 = ::match_deref::match_deref! { match &(restype.clone()) {
                        Deref @ DAE::Type::T_COMPLEX { varLst: __pa0, .. } => __pa0.clone(),
                        _ => return Err("pattern mismatch"),
                    } };
                    varlst = metamodelica::Own::own(__pa0);
                    varlst = List::filterOnFalse(varlst.clone(), &move |__a0: metamodelica::Ref<DAE::Var>| Types::isModifiableTypesVar(&__a0))?;
                    varDecls = List::map(varlst.clone(), &move |__a0: metamodelica::Ref<DAE::Var>| typesVar(&__a0))?;
                    info = ElementSource::getElementSourceFileInfo(source.clone());
                    Ok((metamodelica::Ref::new(SimCodeFunction::Function::Function::RECORD_CONSTRUCTOR { name: name.clone(), funArgs: funArgs.clone(), locals: varDecls.clone(), visibility: openmodelica_frontend_types::SCode::Visibility::PUBLIC, info: info.clone() }), includes.clone(), includeDirs.clone(), libs.clone(), libPaths.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (r#fn, _, _, _, _) => {
                    Error::addInternalError({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("function elaborateFunction failed for function:\n")); __mm_s.push_str(&*DAEDump::dumpFunctionStr(metamodelica::AsArg::as_arg(&r#fn))); ArcStr::from(__mm_s) }, metamodelica::sourceInfo!("SimCode/SimCodeFunctionUtil.mo"))?;
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((outFunction, outIncludes, outIncludeDirs, outLibs, outLibPaths))
}

fn typesSimFunctionArg(
    mut inFuncArg: &metamodelica::Ref<DAE::FuncArg>,
    mut binding: Option<metamodelica::Ref<DAE::Exp>>,
) -> Result<metamodelica::Ref<SimCodeFunction::Variable::Variable>> {
    let mut outVar: metamodelica::Ref<SimCodeFunction::Variable::Variable>;
    outVar = 'mc: {
        let __mc_input = &**inFuncArg;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::FuncArg { name, ty: Deref @ DAE::Type::T_FUNCTION { funcArg: args, funcResultType: Deref @ DAE::Type::T_TUPLE { types: tys, .. }, .. }, .. } => {
                    let mut var_args: metamodelica::List<metamodelica::Ref<SimCodeFunction::Variable::Variable>>;
                    let mut tys = (*tys).clone();
                    var_args = List::map1(args.clone(), &move |__a0: metamodelica::Ref<DAE::FuncArg>, __a1: Option<metamodelica::Ref<DAE::Exp>>| typesSimFunctionArg(&__a0, __a1), None)?;
                    tys = List::map(tys.clone(), &Types::simplifyType)?;
                    Ok(metamodelica::Ref::new(SimCodeFunction::Variable::Variable::FUNCTION_PTR { name: name.clone(), tys: tys.clone(), args: var_args.clone(), defaultValue: binding.clone() }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::FuncArg { name, ty: Deref @ DAE::Type::T_FUNCTION { funcArg: args, funcResultType: Deref @ DAE::Type::T_NORETCALL { .. }, .. }, .. } => {
                    let mut var_args: metamodelica::List<metamodelica::Ref<SimCodeFunction::Variable::Variable>>;
                    var_args = List::map1(args.clone(), &move |__a0: metamodelica::Ref<DAE::FuncArg>, __a1: Option<metamodelica::Ref<DAE::Exp>>| typesSimFunctionArg(&__a0, __a1), None)?;
                    Ok(metamodelica::Ref::new(SimCodeFunction::Variable::Variable::FUNCTION_PTR { name: name.clone(), tys: metamodelica::nil(), args: var_args.clone(), defaultValue: binding.clone() }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::FuncArg { name, ty: Deref @ DAE::Type::T_FUNCTION { funcArg: args, funcResultType: res_ty, .. }, .. } => {
                    let mut var_args: metamodelica::List<metamodelica::Ref<SimCodeFunction::Variable::Variable>>;
                    let mut res_ty = (*res_ty).clone();
                    res_ty = Types::simplifyType(res_ty.clone())?;
                    var_args = List::map1(args.clone(), &move |__a0: metamodelica::Ref<DAE::FuncArg>, __a1: Option<metamodelica::Ref<DAE::Exp>>| typesSimFunctionArg(&__a0, __a1), None)?;
                    Ok(metamodelica::Ref::new(SimCodeFunction::Variable::Variable::FUNCTION_PTR { name: name.clone(), tys: list![res_ty.clone()], args: var_args.clone(), defaultValue: binding.clone() }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::FuncArg { name, ty: tty, par: prl, r#const, .. } => {
                    let mut cref_: metamodelica::Ref<DAE::ComponentRef>;
                    let mut kind: DAE::VarKind;
                    let mut tty = (*tty).clone();
                    tty = Types::simplifyType(tty.clone())?;
                    cref_ = ComponentReferenceBasics::makeCrefIdent(name.clone(), tty.clone(), metamodelica::nil());
                    kind = DAEUtil::const2VarKind(r#const.clone())?;
                    Ok(metamodelica::Ref::new(SimCodeFunction::Variable::Variable::VARIABLE { name: cref_.clone(), ty: tty.clone(), value: binding.clone(), instDims: metamodelica::nil(), parallelism: prl.clone(), kind: kind, bind_from_outside: false }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(outVar)
}

fn daeInOutSimVar(
    mut inElement: &metamodelica::Ref<DAE::Element>,
) -> Result<metamodelica::Ref<SimCodeFunction::Variable::Variable>> {
    let mut outVar: metamodelica::Ref<SimCodeFunction::Variable::Variable>;
    outVar = 'mc: {
        let __mc_input = &**inElement;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Element::VAR { componentRef: Deref @ DAE::ComponentRef::CREF_IDENT { ident: name, .. }, ty: daeType @ Deref @ DAE::Type::T_FUNCTION { .. }, parallelism: prl, binding, .. } => {
                    let mut var: metamodelica::Ref<SimCodeFunction::Variable::Variable>;
                    var = typesSimFunctionArg(&(metamodelica::Ref::new(DAE::FuncArg { name: name.clone(), ty: daeType.clone(), r#const: openmodelica_frontend_types::DAE::Const::C_VAR, par: prl.clone(), defaultBinding: None })), binding.clone())?;
                    Ok(var.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Element::VAR { componentRef: id, parallelism: prl, ty: daeType, binding, dims: inst_dims, kind, .. } => {
                    let mut daeType = (*daeType).clone();
                    daeType = Types::simplifyType(daeType.clone())?;
                    Ok(metamodelica::Ref::new(SimCodeFunction::Variable::Variable::VARIABLE { name: id.clone(), ty: daeType.clone(), value: binding.clone(), instDims: inst_dims.clone(), parallelism: prl.clone(), kind: kind.clone(), bind_from_outside: false }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Error::addInternalError(literal!("function daeInOutSimVar failed\n"), metamodelica::sourceInfo!("SimCode/SimCodeFunctionUtil.mo"))?;
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(outVar)
}

fn extArgsToSimExtArgs(mut extArg: &DAE::ExtArg) -> Result<metamodelica::Ref<SimCodeFunction::SimExtArg::SimExtArg>> {
    let mut simExtArg: metamodelica::Ref<SimCodeFunction::SimExtArg::SimExtArg>;
    simExtArg = (match extArg.clone() {
        DAE::ExtArg::EXTARG {
            componentRef: mut componentRef,
            direction: mut dir,
            type_: mut type_,
        } => {
            let mut isInput: bool;
            let mut isOutput: bool;
            let mut isArray: bool;
            let mut outputIndex: i32;
            let mut type_ = type_.clone();
            isInput = AbsynUtil::isInput(dir.clone());
            isOutput = AbsynUtil::isOutput(dir.clone());
            outputIndex = if (isOutput) { -1 } else { 0 };
            isArray = Types::isArray(metamodelica::AsArg::as_arg(&type_));
            type_ = Types::simplifyType(type_.clone())?;
            metamodelica::Ref::new(SimCodeFunction::SimExtArg::SimExtArg::SIMEXTARG {
                cref: componentRef.clone(),
                isInput: isInput,
                outputIndex: outputIndex,
                isArray: isArray,
                hasBinding: false,
                type_: type_.clone(),
            })
        }
        DAE::ExtArg::EXTARGEXP {
            exp: ref exp_,
            type_: mut type_,
        } => {
            let mut type_ = type_.clone();
            type_ = Types::simplifyType(type_.clone())?;
            metamodelica::Ref::new(SimCodeFunction::SimExtArg::SimExtArg::SIMEXTARGEXP {
                exp: exp_.clone(),
                type_: type_.clone(),
            })
        }
        DAE::ExtArg::EXTARGSIZE {
            componentRef: mut componentRef,
            type_: mut type_,
            exp: ref exp_,
        } => {
            let mut type_ = type_.clone();
            type_ = Types::simplifyType(type_.clone())?;
            metamodelica::Ref::new(SimCodeFunction::SimExtArg::SimExtArg::SIMEXTARGSIZE {
                cref: componentRef.clone(),
                isInput: true,
                outputIndex: 0,
                type_: type_.clone(),
                exp: exp_.clone(),
            })
        }
        DAE::ExtArg::NOEXTARG { .. } => openmodelica_simcode_types::SimCodeFunction::SimExtArg::interned_SIMNOEXTARG(),
    });
    Ok(simExtArg)
}

fn fixOutputIndex(
    mut outVars: metamodelica::List<metamodelica::Ref<SimCodeFunction::Variable::Variable>>,
    mut simExtArgsIn: metamodelica::List<metamodelica::Ref<SimCodeFunction::SimExtArg::SimExtArg>>,
    mut extReturnIn: metamodelica::Ref<SimCodeFunction::SimExtArg::SimExtArg>,
) -> Result<(
    metamodelica::List<metamodelica::Ref<SimCodeFunction::SimExtArg::SimExtArg>>,
    metamodelica::Ref<SimCodeFunction::SimExtArg::SimExtArg>,
)> {
    let mut simExtArgsOut: metamodelica::List<metamodelica::Ref<SimCodeFunction::SimExtArg::SimExtArg>>;
    let mut extReturnOut: metamodelica::Ref<SimCodeFunction::SimExtArg::SimExtArg>;
    (simExtArgsOut, extReturnOut) = (match &*extReturnIn {
        _ => {
            simExtArgsOut = List::map1(
                simExtArgsIn,
                &move |__a0: metamodelica::Ref<SimCodeFunction::SimExtArg::SimExtArg>,
                       __a1: metamodelica::List<metamodelica::Ref<SimCodeFunction::Variable::Variable>>|
                      -> metamodelica::Result<_> {
                    ::std::result::Result::Ok(assignOutputIndex(__a0, &__a1))
                },
                outVars.clone(),
            )?;
            extReturnOut = assignOutputIndex(extReturnIn, &outVars);
            (simExtArgsOut, extReturnOut)
        }
    });
    Ok((simExtArgsOut, extReturnOut))
}

fn assignOutputIndex(
    mut simExtArgIn: metamodelica::Ref<SimCodeFunction::SimExtArg::SimExtArg>,
    mut outVars: &metamodelica::List<metamodelica::Ref<SimCodeFunction::Variable::Variable>>,
) -> metamodelica::Ref<SimCodeFunction::SimExtArg::SimExtArg> {
    let mut simExtArgOut: metamodelica::Ref<SimCodeFunction::SimExtArg::SimExtArg>;
    simExtArgOut = 'mc: {
        let __mc_input = &*simExtArgIn;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ SimCodeFunction::SimExtArg::SIMEXTARG { cref, isInput, outputIndex, isArray, hasBinding: _, type_ } => {
                    let mut fcref: metamodelica::Ref<DAE::ComponentRef>;
                    let mut hasBinding: bool;
                    let mut newOutputIndex: i32;
                    let true = (outputIndex.clone() == -1) else { return Err("pattern mismatch") };
                    fcref = ComponentReferenceBasics::crefFirstCref(cref.clone())?;
                    (newOutputIndex, hasBinding) = findIndexInList(&fcref, outVars, 1)?;
                    Ok(metamodelica::Ref::new(SimCodeFunction::SimExtArg::SimExtArg::SIMEXTARG { cref: cref.clone(), isInput: isInput.clone(), outputIndex: newOutputIndex, isArray: isArray.clone(), hasBinding: hasBinding, type_: type_.clone() }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ SimCodeFunction::SimExtArg::SIMEXTARGSIZE { cref, isInput, outputIndex, type_, exp } => {
                    let mut newOutputIndex: i32;
                    let true = (outputIndex.clone() == -1) else { return Err("pattern mismatch") };
                    (newOutputIndex, _) = findIndexInList(metamodelica::AsArg::as_arg(&cref), outVars, 1)?;
                    Ok(metamodelica::Ref::new(SimCodeFunction::SimExtArg::SimExtArg::SIMEXTARGSIZE { cref: cref.clone(), isInput: isInput.clone(), outputIndex: newOutputIndex, type_: type_.clone(), exp: exp.clone() }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(simExtArgIn.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    simExtArgOut
}

fn findIndexInList(
    mut cref: &metamodelica::Ref<DAE::ComponentRef>,
    mut outVars: &metamodelica::List<metamodelica::Ref<SimCodeFunction::Variable::Variable>>,
    mut inCurrentIndex: i32,
) -> Result<(i32, bool)> {
    let mut crefIndexInOutVars: i32;
    let mut hasBinding: bool = false;
    (crefIndexInOutVars, hasBinding) = 'mc: {
        let __mc_input = (&**outVars, inCurrentIndex);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Nil, _) => {
                    Ok((-1, false))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: Deref @ SimCodeFunction::Variable::VARIABLE { name, value: v, .. }, tail: _ }, currentIndex) => {
                    let true = (ComponentReferenceBasics::crefEqualNoStringCompare(cref, metamodelica::AsArg::as_arg(&name))?) else { return Err("pattern mismatch") };
                    Ok((currentIndex.clone(), (v).is_some()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: _, tail: restOutVars }, currentIndex) => {
                    let mut currentIndex = (*currentIndex).clone();
                    let mut hasBinding: bool = hasBinding.clone();
                    currentIndex = currentIndex.clone() + 1;
                    (currentIndex, hasBinding) = findIndexInList(cref, metamodelica::AsArg::as_arg(&restOutVars), currentIndex.clone())?;
                    Ok(((currentIndex.clone(), hasBinding), hasBinding.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            hasBinding = __wb0;
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((crefIndexInOutVars, hasBinding))
}

fn elaborateStatement(
    mut inElement: &metamodelica::Ref<DAE::Element>,
) -> Result<metamodelica::List<metamodelica::Ref<DAE::Statement>>> {
    let mut stmts: metamodelica::List<metamodelica::Ref<DAE::Statement>>;
    let __pa0 = ::match_deref::match_deref! { match &((*inElement)) {
        Deref @ DAE::Element::ALGORITHM { algorithm_: Deref @ DAE::Algorithm { statementLst: __pa0 }, .. } => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    stmts = metamodelica::Own::own(__pa0);
    Ok(stmts)
}

fn optMRFAElems(
    mut elems: metamodelica::List<metamodelica::Ref<DAE::Element>>,
) -> Result<metamodelica::List<metamodelica::Ref<DAE::Element>>> {
    let mut elems: metamodelica::List<metamodelica::Ref<DAE::Element>> = elems;
    let mut processed: metamodelica::List<metamodelica::Ref<DAE::Element>> = metamodelica::nil();
    let mut tempVars: metamodelica::List<metamodelica::Ref<DAE::Element>> = metamodelica::nil();
    let mut e2: metamodelica::Ref<DAE::Element>;
    for mut e in &*elems {
        (e2, tempVars) = optMRFAElem(e.clone(), tempVars)?;
        processed = metamodelica::cons(e2, processed);
    }
    elems = listAppend(tempVars.reverse(), processed.reverse());
    Ok(elems)
}

fn optMRFAElem(
    mut elem: metamodelica::Ref<DAE::Element>,
    mut tempVars: metamodelica::List<metamodelica::Ref<DAE::Element>>,
) -> Result<(
    metamodelica::Ref<DAE::Element>,
    metamodelica::List<metamodelica::Ref<DAE::Element>>,
)> {
    let mut elem: metamodelica::Ref<DAE::Element> = elem;
    let mut tempVars: metamodelica::List<metamodelica::Ref<DAE::Element>> = tempVars;
    (elem, tempVars) = (::match_deref::match_deref! { match &(elem.clone()) {
        Deref @ DAE::Element::ALGORITHM { algorithm_: Deref @ DAE::Algorithm { statementLst: stmts }, .. } => {
            let mut stmts = (*stmts).clone();
            (stmts, tempVars) = DAEUtil::optimizeMetaRecordFieldAssigns(metamodelica::AsArg::as_arg(&stmts), tempVars)?;
            assign_variant_field!(elem => DAE::Element::ALGORITHM; algorithm_ = metamodelica::Ref::new(DAE::Algorithm { statementLst: stmts.clone() }));
            (elem, tempVars)
        },
        _ => {
            (elem, tempVars)
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((elem, tempVars))
}

pub fn checkValidMainFunction(
    mut name: ArcStr,
    mut r#fn: &metamodelica::Ref<SimCodeFunction::Function::Function>,
) -> Result<()> {
    let () = 'mc: {
        let __mc_input = &**r#fn;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ SimCodeFunction::Function::FUNCTION { functionArguments: inVars, .. } => {
                    if '__try0: {
                        unwrap_break_err!(List::find(metamodelica::AsArg::as_arg(&inVars), &move |__a0: metamodelica::Ref<SimCodeFunction::Variable::Variable>| -> metamodelica::Result<_> { ::std::result::Result::Ok(isFunctionPtr(&__a0)) }), '__try0);
                        Ok::<(), &'static str>(())
                    }.is_ok() { return Err("failure(): body succeeded") }
                    Ok(())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ SimCodeFunction::Function::EXTERNAL_FUNCTION { inVars, .. } => {
                    if '__try0: {
                        unwrap_break_err!(List::find(metamodelica::AsArg::as_arg(&inVars), &move |__a0: metamodelica::Ref<SimCodeFunction::Variable::Variable>| -> metamodelica::Result<_> { ::std::result::Result::Ok(isFunctionPtr(&__a0)) }), '__try0);
                        Ok::<(), &'static str>(())
                    }.is_ok() { return Err("failure(): body succeeded") }
                    Ok(())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Error::addMessage(Error::GENERATECODE_INVARS_HAS_FUNCTION_PTR.clone(), list![name.clone()])?;
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(())
}

pub fn isBoxedFunction(mut r#fn: &metamodelica::Ref<SimCodeFunction::Function::Function>) -> bool {
    let mut b: bool;
    b = 'mc: {
        let __mc_input = &**r#fn;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ SimCodeFunction::Function::FUNCTION { functionArguments: inVars, outVars, .. } => {
                    List::map_0(metamodelica::AsArg::as_arg(&inVars), &move |__a0: metamodelica::Ref<SimCodeFunction::Variable::Variable>| isBoxedArg(&__a0))?;
                    List::map_0(metamodelica::AsArg::as_arg(&outVars), &move |__a0: metamodelica::Ref<SimCodeFunction::Variable::Variable>| isBoxedArg(&__a0))?;
                    Ok(true)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ SimCodeFunction::Function::EXTERNAL_FUNCTION { inVars, outVars, .. } => {
                    List::map_0(metamodelica::AsArg::as_arg(&inVars), &move |__a0: metamodelica::Ref<SimCodeFunction::Variable::Variable>| isBoxedArg(&__a0))?;
                    List::map_0(metamodelica::AsArg::as_arg(&outVars), &move |__a0: metamodelica::Ref<SimCodeFunction::Variable::Variable>| isBoxedArg(&__a0))?;
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

fn isFunctionPtr(mut var: &metamodelica::Ref<SimCodeFunction::Variable::Variable>) -> bool {
    let mut b: bool;
    b = (match &**var {
        SimCodeFunction::Variable::FUNCTION_PTR { .. } => true,
        _ => false,
    });
    b
}

fn isBoxedArg(mut var: &metamodelica::Ref<SimCodeFunction::Variable::Variable>) -> Result<()> {
    let () = (::match_deref::match_deref! { match var {
        Deref @ SimCodeFunction::Variable::FUNCTION_PTR { .. } => (),
        Deref @ SimCodeFunction::Variable::VARIABLE { ty: Deref @ DAE::Type::T_METABOXED { .. }, .. } => (),
        Deref @ SimCodeFunction::Variable::VARIABLE { ty: Deref @ DAE::Type::T_METATYPE { .. }, .. } => (),
        Deref @ SimCodeFunction::Variable::VARIABLE { ty: Deref @ DAE::Type::T_STRING { .. }, .. } => (),
        _ => return Err("match: no arm matched"),
    } });
    Ok(())
}

pub fn funcHasParallelInOutArrays(mut r#fn: &metamodelica::Ref<SimCodeFunction::Function::Function>) -> Result<bool> {
    let mut b: bool;
    let mut inVars: metamodelica::List<metamodelica::Ref<SimCodeFunction::Variable::Variable>>;
    let mut outVars: metamodelica::List<metamodelica::Ref<SimCodeFunction::Variable::Variable>>;
    let (__pa0, __pa1) = ::match_deref::match_deref! { match &((*r#fn)) {
        Deref @ SimCodeFunction::Function::FUNCTION { functionArguments: __pa0, outVars: __pa1, .. } => (__pa0.clone(), __pa1.clone()),
        _ => return Err("pattern mismatch"),
    } };
    inVars = metamodelica::Own::own(__pa0);
    outVars = metamodelica::Own::own(__pa1);
    for mut e in &*inVars {
        if isParallelArrayVar(metamodelica::AsArg::as_arg(&e)) {
            b = true;
            return Ok(b);
        }
    }
    for mut e in &*outVars {
        if isParallelArrayVar(metamodelica::AsArg::as_arg(&e)) {
            b = true;
            return Ok(b);
        }
    }
    b = false;
    Ok(b)
}

fn isParallelArrayVar(mut var: &metamodelica::Ref<SimCodeFunction::Variable::Variable>) -> bool {
    let mut b: bool;
    b = (::match_deref::match_deref! { match var {
        Deref @ SimCodeFunction::Variable::VARIABLE { ty: Deref @ DAE::Type::T_ARRAY { .. }, parallelism: DAE::VarParallelism::PARGLOBAL { .. }, .. } => true,
        Deref @ SimCodeFunction::Variable::VARIABLE { ty: Deref @ DAE::Type::T_ARRAY { .. }, parallelism: DAE::VarParallelism::PARLOCAL { .. }, .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    b
}

pub fn findLiterals(
    mut fns: metamodelica::List<DAE::Function>,
) -> Result<(
    metamodelica::List<DAE::Function>,
    metamodelica::List<metamodelica::Ref<DAE::Exp>>,
)> {
    let mut ofns: metamodelica::List<DAE::Function>;
    let mut literals: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
    let (__pa0, (_, _, __pa1)) = DAEUtil::traverseDAEFunctions(
        fns,
        (std::sync::Arc::new(findLiteralsHelper)
            as std::sync::Arc<
                dyn ::std::ops::Fn(
                        metamodelica::Ref<DAE::Exp>,
                        (
                            i32,
                            (
                                metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::Exp>, i32)>>,
                                (
                                    i32,
                                    i32,
                                    metamodelica::Array<Option<(metamodelica::Ref<DAE::Exp>, i32)>>,
                                ),
                                i32,
                                (
                                    Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<i32> + 'static>,
                                    Arc<
                                        dyn ::std::ops::Fn(
                                                metamodelica::Ref<DAE::Exp>,
                                                metamodelica::Ref<DAE::Exp>,
                                            ) -> Result<bool>
                                            + 'static,
                                    >,
                                    Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<ArcStr> + 'static>,
                                    Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>,
                                ),
                            ),
                            metamodelica::List<metamodelica::Ref<DAE::Exp>>,
                        ),
                    ) -> Result<(
                        metamodelica::Ref<DAE::Exp>,
                        (
                            i32,
                            (
                                metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::Exp>, i32)>>,
                                (
                                    i32,
                                    i32,
                                    metamodelica::Array<Option<(metamodelica::Ref<DAE::Exp>, i32)>>,
                                ),
                                i32,
                                (
                                    Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<i32> + 'static>,
                                    Arc<
                                        dyn ::std::ops::Fn(
                                                metamodelica::Ref<DAE::Exp>,
                                                metamodelica::Ref<DAE::Exp>,
                                            ) -> Result<bool>
                                            + 'static,
                                    >,
                                    Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<ArcStr> + 'static>,
                                    Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>,
                                ),
                            ),
                            metamodelica::List<metamodelica::Ref<DAE::Exp>>,
                        ),
                    )> + 'static,
            >),
        (
            0,
            HashTableExpToIndex::emptyHashTableSized(BaseHashTable::bigBucketSize.clone()),
            metamodelica::nil(),
        ),
    )?;
    ofns = metamodelica::Own::own(__pa0);
    literals = metamodelica::Own::own(__pa1);
    literals = literals.reverse();
    Ok((ofns, literals))
}

pub fn findLiteralsHelper(
    mut inExp: metamodelica::Ref<DAE::Exp>,
    mut inTpl: (
        i32,
        (
            metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::Exp>, i32)>>,
            (
                i32,
                i32,
                metamodelica::Array<Option<(metamodelica::Ref<DAE::Exp>, i32)>>,
            ),
            i32,
            (
                Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<i32> + 'static>,
                Arc<
                    dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>, metamodelica::Ref<DAE::Exp>) -> Result<bool>
                        + 'static,
                >,
                Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<ArcStr> + 'static>,
                Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>,
            ),
        ),
        metamodelica::List<metamodelica::Ref<DAE::Exp>>,
    ),
) -> Result<(
    metamodelica::Ref<DAE::Exp>,
    (
        i32,
        (
            metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::Exp>, i32)>>,
            (
                i32,
                i32,
                metamodelica::Array<Option<(metamodelica::Ref<DAE::Exp>, i32)>>,
            ),
            i32,
            (
                Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<i32> + 'static>,
                Arc<
                    dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>, metamodelica::Ref<DAE::Exp>) -> Result<bool>
                        + 'static,
                >,
                Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<ArcStr> + 'static>,
                Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>,
            ),
        ),
        metamodelica::List<metamodelica::Ref<DAE::Exp>>,
    ),
)> {
    let mut exp: metamodelica::Ref<DAE::Exp>;
    let mut tpl: (
        i32,
        (
            metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::Exp>, i32)>>,
            (
                i32,
                i32,
                metamodelica::Array<Option<(metamodelica::Ref<DAE::Exp>, i32)>>,
            ),
            i32,
            (
                HashTableExpToIndex::FuncHashCref,
                HashTableExpToIndex::FuncCrefEqual,
                HashTableExpToIndex::FuncCrefStr,
                HashTableExpToIndex::FuncExpStr,
            ),
        ),
        metamodelica::List<metamodelica::Ref<DAE::Exp>>,
    );
    exp = inExp;
    tpl = inTpl;
    (exp, tpl) = Expression::traverseExpBottomUp(
        exp,
        &({
            let __pe_b2: Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>, _) -> Result<_> + 'static> =
                (std::sync::Arc::new(replaceLiteralExp)
                    as std::sync::Arc<
                        dyn ::std::ops::Fn(
                                metamodelica::Ref<DAE::Exp>,
                                (
                                    i32,
                                    (
                                        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::Exp>, i32)>>,
                                        (
                                            i32,
                                            i32,
                                            metamodelica::Array<Option<(metamodelica::Ref<DAE::Exp>, i32)>>,
                                        ),
                                        i32,
                                        (
                                            Arc<
                                                dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<i32>
                                                    + 'static,
                                            >,
                                            Arc<
                                                dyn ::std::ops::Fn(
                                                        metamodelica::Ref<DAE::Exp>,
                                                        metamodelica::Ref<DAE::Exp>,
                                                    )
                                                        -> Result<bool>
                                                    + 'static,
                                            >,
                                            Arc<
                                                dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<ArcStr>
                                                    + 'static,
                                            >,
                                            Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>,
                                        ),
                                    ),
                                    metamodelica::List<metamodelica::Ref<DAE::Exp>>,
                                ),
                            ) -> Result<(
                                metamodelica::Ref<DAE::Exp>,
                                (
                                    i32,
                                    (
                                        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::Exp>, i32)>>,
                                        (
                                            i32,
                                            i32,
                                            metamodelica::Array<Option<(metamodelica::Ref<DAE::Exp>, i32)>>,
                                        ),
                                        i32,
                                        (
                                            Arc<
                                                dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<i32>
                                                    + 'static,
                                            >,
                                            Arc<
                                                dyn ::std::ops::Fn(
                                                        metamodelica::Ref<DAE::Exp>,
                                                        metamodelica::Ref<DAE::Exp>,
                                                    )
                                                        -> Result<bool>
                                                    + 'static,
                                            >,
                                            Arc<
                                                dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<ArcStr>
                                                    + 'static,
                                            >,
                                            Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>,
                                        ),
                                    ),
                                    metamodelica::List<metamodelica::Ref<DAE::Exp>>,
                                ),
                            )> + 'static,
                    >);
            move |__pe_a0, __pe_a1| Patternm::traverseConstantPatternsHelper(__pe_a0, __pe_a1, __pe_b2.clone())
        }),
        tpl,
    )?;
    (exp, tpl) = Expression::traverseExpTopDown(
        exp,
        &fnptr!(
            replaceLiteralArrayExp,
            metamodelica::Ref<DAE::Exp>,
            (
                i32,
                (
                    metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::Exp>, i32)>>,
                    (
                        i32,
                        i32,
                        metamodelica::Array<Option<(metamodelica::Ref<DAE::Exp>, i32)>>
                    ),
                    i32,
                    (
                        Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<i32> + 'static>,
                        Arc<
                            dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>, metamodelica::Ref<DAE::Exp>) -> Result<bool>
                                + 'static,
                        >,
                        Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<ArcStr> + 'static>,
                        Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>
                    )
                ),
                metamodelica::List<metamodelica::Ref<DAE::Exp>>
            )
        ),
        tpl,
    )?;
    Ok((exp, tpl))
}

pub fn findLiteralsHelperKeepSingle(
    mut inExp: metamodelica::Ref<DAE::Exp>,
    mut uses: &(
        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::Exp>, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<Option<(metamodelica::Ref<DAE::Exp>, i32)>>,
        ),
        i32,
        (
            Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<i32> + 'static>,
            Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>, metamodelica::Ref<DAE::Exp>) -> Result<bool> + 'static>,
            Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<ArcStr> + 'static>,
            Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>,
        ),
    ),
    mut inTpl: (
        i32,
        (
            metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::Exp>, i32)>>,
            (
                i32,
                i32,
                metamodelica::Array<Option<(metamodelica::Ref<DAE::Exp>, i32)>>,
            ),
            i32,
            (
                Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<i32> + 'static>,
                Arc<
                    dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>, metamodelica::Ref<DAE::Exp>) -> Result<bool>
                        + 'static,
                >,
                Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<ArcStr> + 'static>,
                Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>,
            ),
        ),
        metamodelica::List<metamodelica::Ref<DAE::Exp>>,
    ),
) -> Result<(
    metamodelica::Ref<DAE::Exp>,
    (
        i32,
        (
            metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::Exp>, i32)>>,
            (
                i32,
                i32,
                metamodelica::Array<Option<(metamodelica::Ref<DAE::Exp>, i32)>>,
            ),
            i32,
            (
                Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<i32> + 'static>,
                Arc<
                    dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>, metamodelica::Ref<DAE::Exp>) -> Result<bool>
                        + 'static,
                >,
                Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<ArcStr> + 'static>,
                Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>,
            ),
        ),
        metamodelica::List<metamodelica::Ref<DAE::Exp>>,
    ),
)> {
    let mut exp: metamodelica::Ref<DAE::Exp>;
    let mut tpl: (
        i32,
        (
            metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::Exp>, i32)>>,
            (
                i32,
                i32,
                metamodelica::Array<Option<(metamodelica::Ref<DAE::Exp>, i32)>>,
            ),
            i32,
            (
                HashTableExpToIndex::FuncHashCref,
                HashTableExpToIndex::FuncCrefEqual,
                HashTableExpToIndex::FuncCrefStr,
                HashTableExpToIndex::FuncExpStr,
            ),
        ),
        metamodelica::List<metamodelica::Ref<DAE::Exp>>,
    );
    (exp, tpl) = Expression::traverseExpBottomUp(
        inExp,
        &({
            let __pe_b2: Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>, _) -> Result<_> + 'static> =
                (std::sync::Arc::new({
                    let __pe_b1 = uses.clone();
                    move |__pe_a0, __pe_a2| replaceLiteralExpKeepSingle(__pe_a0, &__pe_b1, __pe_a2)
                })
                    as std::sync::Arc<
                        dyn ::std::ops::Fn(
                                metamodelica::Ref<DAE::Exp>,
                                (
                                    i32,
                                    (
                                        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::Exp>, i32)>>,
                                        (
                                            i32,
                                            i32,
                                            metamodelica::Array<Option<(metamodelica::Ref<DAE::Exp>, i32)>>,
                                        ),
                                        i32,
                                        (
                                            Arc<
                                                dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<i32>
                                                    + 'static,
                                            >,
                                            Arc<
                                                dyn ::std::ops::Fn(
                                                        metamodelica::Ref<DAE::Exp>,
                                                        metamodelica::Ref<DAE::Exp>,
                                                    )
                                                        -> Result<bool>
                                                    + 'static,
                                            >,
                                            Arc<
                                                dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<ArcStr>
                                                    + 'static,
                                            >,
                                            Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>,
                                        ),
                                    ),
                                    metamodelica::List<metamodelica::Ref<DAE::Exp>>,
                                ),
                            ) -> Result<(
                                metamodelica::Ref<DAE::Exp>,
                                (
                                    i32,
                                    (
                                        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::Exp>, i32)>>,
                                        (
                                            i32,
                                            i32,
                                            metamodelica::Array<Option<(metamodelica::Ref<DAE::Exp>, i32)>>,
                                        ),
                                        i32,
                                        (
                                            Arc<
                                                dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<i32>
                                                    + 'static,
                                            >,
                                            Arc<
                                                dyn ::std::ops::Fn(
                                                        metamodelica::Ref<DAE::Exp>,
                                                        metamodelica::Ref<DAE::Exp>,
                                                    )
                                                        -> Result<bool>
                                                    + 'static,
                                            >,
                                            Arc<
                                                dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<ArcStr>
                                                    + 'static,
                                            >,
                                            Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>,
                                        ),
                                    ),
                                    metamodelica::List<metamodelica::Ref<DAE::Exp>>,
                                ),
                            )> + 'static,
                    >);
            move |__pe_a0, __pe_a1| Patternm::traverseConstantPatternsHelper(__pe_a0, __pe_a1, __pe_b2.clone())
        }),
        inTpl,
    )?;
    (exp, tpl) = Expression::traverseExpTopDown(
        exp,
        &fnptr!(
            replaceLiteralArrayExp,
            metamodelica::Ref<DAE::Exp>,
            (
                i32,
                (
                    metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::Exp>, i32)>>,
                    (
                        i32,
                        i32,
                        metamodelica::Array<Option<(metamodelica::Ref<DAE::Exp>, i32)>>
                    ),
                    i32,
                    (
                        Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<i32> + 'static>,
                        Arc<
                            dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>, metamodelica::Ref<DAE::Exp>) -> Result<bool>
                                + 'static,
                        >,
                        Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<ArcStr> + 'static>,
                        Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>
                    )
                ),
                metamodelica::List<metamodelica::Ref<DAE::Exp>>
            )
        ),
        tpl,
    )?;
    Ok((exp, tpl))
}

pub fn countStringUses(
    mut inExp: metamodelica::Ref<DAE::Exp>,
    mut inUses: (
        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::Exp>, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<Option<(metamodelica::Ref<DAE::Exp>, i32)>>,
        ),
        i32,
        (
            Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<i32> + 'static>,
            Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>, metamodelica::Ref<DAE::Exp>) -> Result<bool> + 'static>,
            Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<ArcStr> + 'static>,
            Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>,
        ),
    ),
) -> Result<(
    metamodelica::Ref<DAE::Exp>,
    (
        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::Exp>, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<Option<(metamodelica::Ref<DAE::Exp>, i32)>>,
        ),
        i32,
        (
            Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<i32> + 'static>,
            Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>, metamodelica::Ref<DAE::Exp>) -> Result<bool> + 'static>,
            Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<ArcStr> + 'static>,
            Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>,
        ),
    ),
)> {
    let mut exp: metamodelica::Ref<DAE::Exp> = inExp.clone();
    let mut uses: (
        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::Exp>, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<Option<(metamodelica::Ref<DAE::Exp>, i32)>>,
        ),
        i32,
        (
            HashTableExpToIndex::FuncHashCref,
            HashTableExpToIndex::FuncCrefEqual,
            HashTableExpToIndex::FuncCrefStr,
            HashTableExpToIndex::FuncExpStr,
        ),
    );
    (_, uses) = Expression::traverseExpBottomUp(inExp, &countStringUse, inUses)?;
    Ok((exp, uses))
}

fn isSconst(mut e: &metamodelica::Ref<DAE::Exp>) -> bool {
    let mut b: bool;
    b = (match &**e {
        DAE::Exp::SCONST { .. } => true,
        _ => false,
    });
    b
}

fn countStringUse(
    mut inExp: metamodelica::Ref<DAE::Exp>,
    mut inUses: (
        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::Exp>, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<Option<(metamodelica::Ref<DAE::Exp>, i32)>>,
        ),
        i32,
        (
            Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<i32> + 'static>,
            Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>, metamodelica::Ref<DAE::Exp>) -> Result<bool> + 'static>,
            Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<ArcStr> + 'static>,
            Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>,
        ),
    ),
) -> Result<(
    metamodelica::Ref<DAE::Exp>,
    (
        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::Exp>, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<Option<(metamodelica::Ref<DAE::Exp>, i32)>>,
        ),
        i32,
        (
            Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<i32> + 'static>,
            Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>, metamodelica::Ref<DAE::Exp>) -> Result<bool> + 'static>,
            Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<ArcStr> + 'static>,
            Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>,
        ),
    ),
)> {
    let mut exp: metamodelica::Ref<DAE::Exp> = inExp.clone();
    let mut uses: (
        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::Exp>, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<Option<(metamodelica::Ref<DAE::Exp>, i32)>>,
        ),
        i32,
        (
            HashTableExpToIndex::FuncHashCref,
            HashTableExpToIndex::FuncCrefEqual,
            HashTableExpToIndex::FuncCrefStr,
            HashTableExpToIndex::FuncExpStr,
        ),
    ) = inUses;
    if isSconst(&inExp) {
        uses = addStringUse(inExp, uses)?;
    } else {
        for mut e in &*literalElements(&inExp)? {
            if isSconst(metamodelica::AsArg::as_arg(&e)) {
                uses = addStringUse(e.clone(), uses)?;
            }
        }
    }
    Ok((exp, uses))
}

fn addStringUse(
    mut e: metamodelica::Ref<DAE::Exp>,
    mut uses: (
        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::Exp>, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<Option<(metamodelica::Ref<DAE::Exp>, i32)>>,
        ),
        i32,
        (
            Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<i32> + 'static>,
            Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>, metamodelica::Ref<DAE::Exp>) -> Result<bool> + 'static>,
            Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<ArcStr> + 'static>,
            Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>,
        ),
    ),
) -> Result<(
    metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::Exp>, i32)>>,
    (
        i32,
        i32,
        metamodelica::Array<Option<(metamodelica::Ref<DAE::Exp>, i32)>>,
    ),
    i32,
    (
        Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<i32> + 'static>,
        Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>, metamodelica::Ref<DAE::Exp>) -> Result<bool> + 'static>,
        Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<ArcStr> + 'static>,
        Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>,
    ),
)> {
    let mut uses: (
        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::Exp>, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<Option<(metamodelica::Ref<DAE::Exp>, i32)>>,
        ),
        i32,
        (
            HashTableExpToIndex::FuncHashCref,
            HashTableExpToIndex::FuncCrefEqual,
            HashTableExpToIndex::FuncCrefStr,
            HashTableExpToIndex::FuncExpStr,
        ),
    ) = uses;
    uses = BaseHashTable::add(
        (
            e.clone(),
            if (BaseHashTable::hasKey(e.clone(), &uses)?) {
                BaseHashTable::get(e, &uses)? + 1
            } else {
                1
            },
        ),
        uses,
    )?;
    Ok(uses)
}

fn literalElements(mut e: &metamodelica::Ref<DAE::Exp>) -> Result<metamodelica::List<metamodelica::Ref<DAE::Exp>>> {
    let mut elts: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
    elts = (::match_deref::match_deref! { match e {
        Deref @ DAE::Exp::ARRAY { array: __e_array, .. } => {
            __e_array.clone()
        },
        Deref @ DAE::Exp::MATRIX { matrix: __e_matrix, .. } => {
            List::flatten(__e_matrix.clone())?
        },
        Deref @ DAE::Exp::BOX { exp: e1 } => {
            list![e1.clone()]
        },
        Deref @ DAE::Exp::META_OPTION { exp: Some(e1) } => {
            list![e1.clone()]
        },
        Deref @ DAE::Exp::CONS { car: e1, cdr: e2 } => {
            list![e1.clone(), e2.clone()]
        },
        Deref @ DAE::Exp::LIST { valList: __e_valList } => {
            __e_valList.clone()
        },
        Deref @ DAE::Exp::META_TUPLE { listExp: __e_listExp } => {
            __e_listExp.clone()
        },
        Deref @ DAE::Exp::METARECORDCALL { args: __e_args, .. } => {
            __e_args.clone()
        },
        Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "listArrayLiteral" }, expLst: __e_expLst, .. } => {
            __e_expLst.clone()
        },
        _ => {
            metamodelica::nil()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(elts)
}

fn replaceLiteralExpKeepSingle(
    mut inExp: metamodelica::Ref<DAE::Exp>,
    mut uses: &(
        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::Exp>, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<Option<(metamodelica::Ref<DAE::Exp>, i32)>>,
        ),
        i32,
        (
            Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<i32> + 'static>,
            Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>, metamodelica::Ref<DAE::Exp>) -> Result<bool> + 'static>,
            Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<ArcStr> + 'static>,
            Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>,
        ),
    ),
    mut inTpl: (
        i32,
        (
            metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::Exp>, i32)>>,
            (
                i32,
                i32,
                metamodelica::Array<Option<(metamodelica::Ref<DAE::Exp>, i32)>>,
            ),
            i32,
            (
                Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<i32> + 'static>,
                Arc<
                    dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>, metamodelica::Ref<DAE::Exp>) -> Result<bool>
                        + 'static,
                >,
                Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<ArcStr> + 'static>,
                Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>,
            ),
        ),
        metamodelica::List<metamodelica::Ref<DAE::Exp>>,
    ),
) -> Result<(
    metamodelica::Ref<DAE::Exp>,
    (
        i32,
        (
            metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::Exp>, i32)>>,
            (
                i32,
                i32,
                metamodelica::Array<Option<(metamodelica::Ref<DAE::Exp>, i32)>>,
            ),
            i32,
            (
                Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<i32> + 'static>,
                Arc<
                    dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>, metamodelica::Ref<DAE::Exp>) -> Result<bool>
                        + 'static,
                >,
                Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<ArcStr> + 'static>,
                Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>,
            ),
        ),
        metamodelica::List<metamodelica::Ref<DAE::Exp>>,
    ),
)> {
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    let mut outTpl: (
        i32,
        (
            metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::Exp>, i32)>>,
            (
                i32,
                i32,
                metamodelica::Array<Option<(metamodelica::Ref<DAE::Exp>, i32)>>,
            ),
            i32,
            (
                HashTableExpToIndex::FuncHashCref,
                HashTableExpToIndex::FuncCrefEqual,
                HashTableExpToIndex::FuncCrefStr,
                HashTableExpToIndex::FuncExpStr,
            ),
        ),
        metamodelica::List<metamodelica::Ref<DAE::Exp>>,
    );
    let mut ht: (
        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::Exp>, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<Option<(metamodelica::Ref<DAE::Exp>, i32)>>,
        ),
        i32,
        (
            HashTableExpToIndex::FuncHashCref,
            HashTableExpToIndex::FuncCrefEqual,
            HashTableExpToIndex::FuncCrefStr,
            HashTableExpToIndex::FuncExpStr,
        ),
    );
    (_, ht, _) = inTpl.clone();
    if isSconst(&inExp)
        && BaseHashTable::hasKey(inExp.clone(), uses)?
        && BaseHashTable::get(inExp.clone(), uses)? == 1
        && !(BaseHashTable::hasKey(inExp.clone(), &ht)?)
    {
        outExp = inExp;
        outTpl = inTpl;
    } else {
        (outExp, outTpl) = replaceLiteralExp(inExp, inTpl)?;
    }
    Ok((outExp, outTpl))
}

fn replaceLiteralArrayExp(
    mut inExp: metamodelica::Ref<DAE::Exp>,
    mut inTpl: (
        i32,
        (
            metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::Exp>, i32)>>,
            (
                i32,
                i32,
                metamodelica::Array<Option<(metamodelica::Ref<DAE::Exp>, i32)>>,
            ),
            i32,
            (
                Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<i32> + 'static>,
                Arc<
                    dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>, metamodelica::Ref<DAE::Exp>) -> Result<bool>
                        + 'static,
                >,
                Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<ArcStr> + 'static>,
                Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>,
            ),
        ),
        metamodelica::List<metamodelica::Ref<DAE::Exp>>,
    ),
) -> (
    metamodelica::Ref<DAE::Exp>,
    bool,
    (
        i32,
        (
            metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::Exp>, i32)>>,
            (
                i32,
                i32,
                metamodelica::Array<Option<(metamodelica::Ref<DAE::Exp>, i32)>>,
            ),
            i32,
            (
                Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<i32> + 'static>,
                Arc<
                    dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>, metamodelica::Ref<DAE::Exp>) -> Result<bool>
                        + 'static,
                >,
                Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<ArcStr> + 'static>,
                Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>,
            ),
        ),
        metamodelica::List<metamodelica::Ref<DAE::Exp>>,
    ),
) {
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    let mut cont: bool = true;
    let mut outTpl: (
        i32,
        (
            metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::Exp>, i32)>>,
            (
                i32,
                i32,
                metamodelica::Array<Option<(metamodelica::Ref<DAE::Exp>, i32)>>,
            ),
            i32,
            (
                HashTableExpToIndex::FuncHashCref,
                HashTableExpToIndex::FuncCrefEqual,
                HashTableExpToIndex::FuncCrefStr,
                HashTableExpToIndex::FuncExpStr,
            ),
        ),
        metamodelica::List<metamodelica::Ref<DAE::Exp>>,
    );
    (outExp, outTpl) = (match &*inExp {
        DAE::Exp::ARRAY { .. } => {
            let mut tpl = inTpl.clone();
            let mut exp2: metamodelica::Ref<DAE::Exp>;
            match '__try0: {
                unwrap_break_err!(isLiteralArrayExp(&inExp), '__try0);
                (exp2, tpl) = unwrap_break_err!(replaceLiteralExp2(inExp.clone(), tpl.clone()), '__try0);
                cont = false;
                Ok::<_, &'static str>((exp2.clone(),))
            } {
                Ok((__try0_o0,)) => {
                    exp2 = __try0_o0;
                }
                Err(_) => {
                    exp2 = inExp.clone();
                }
            }
            (exp2, tpl)
        }
        DAE::Exp::MATRIX { .. } => {
            let mut tpl = inTpl.clone();
            let mut exp2: metamodelica::Ref<DAE::Exp>;
            match '__try0: {
                unwrap_break_err!(isLiteralArrayExp(&inExp), '__try0);
                (exp2, tpl) = unwrap_break_err!(replaceLiteralExp2(inExp.clone(), tpl.clone()), '__try0);
                cont = false;
                Ok::<_, &'static str>((exp2.clone(),))
            } {
                Ok((__try0_o0,)) => {
                    exp2 = __try0_o0;
                }
                Err(_) => {
                    exp2 = inExp.clone();
                }
            }
            (exp2, tpl)
        }
        _ => (inExp, inTpl),
    });
    (outExp, cont, outTpl)
}

fn replaceLiteralExp(
    mut inExp: metamodelica::Ref<DAE::Exp>,
    mut inTpl: (
        i32,
        (
            metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::Exp>, i32)>>,
            (
                i32,
                i32,
                metamodelica::Array<Option<(metamodelica::Ref<DAE::Exp>, i32)>>,
            ),
            i32,
            (
                Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<i32> + 'static>,
                Arc<
                    dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>, metamodelica::Ref<DAE::Exp>) -> Result<bool>
                        + 'static,
                >,
                Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<ArcStr> + 'static>,
                Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>,
            ),
        ),
        metamodelica::List<metamodelica::Ref<DAE::Exp>>,
    ),
) -> Result<(
    metamodelica::Ref<DAE::Exp>,
    (
        i32,
        (
            metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::Exp>, i32)>>,
            (
                i32,
                i32,
                metamodelica::Array<Option<(metamodelica::Ref<DAE::Exp>, i32)>>,
            ),
            i32,
            (
                Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<i32> + 'static>,
                Arc<
                    dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>, metamodelica::Ref<DAE::Exp>) -> Result<bool>
                        + 'static,
                >,
                Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<ArcStr> + 'static>,
                Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>,
            ),
        ),
        metamodelica::List<metamodelica::Ref<DAE::Exp>>,
    ),
)> {
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    let mut outTpl: (
        i32,
        (
            metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::Exp>, i32)>>,
            (
                i32,
                i32,
                metamodelica::Array<Option<(metamodelica::Ref<DAE::Exp>, i32)>>,
            ),
            i32,
            (
                HashTableExpToIndex::FuncHashCref,
                HashTableExpToIndex::FuncCrefEqual,
                HashTableExpToIndex::FuncCrefStr,
                HashTableExpToIndex::FuncExpStr,
            ),
        ),
        metamodelica::List<metamodelica::Ref<DAE::Exp>>,
    );
    (outExp, outTpl) = 'mc: {
        let __mc_input = (inExp.clone(), inTpl.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (exp, t) => {
                    if '__try0: {
                        unwrap_break_err!(isLiteralExp(metamodelica::AsArg::as_arg(&exp)), '__try0);
                        Ok::<(), &'static str>(())
                    }.is_ok() { return Err("failure(): body succeeded") }
                    Ok((exp.clone(), t.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (exp, t) => {
                    isTrivialLiteralExp(metamodelica::AsArg::as_arg(&exp))?;
                    Ok((exp.clone(), t.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::LIST { valList: es }, t) => {
                    let mut exp: metamodelica::Ref<DAE::Exp>;
                    let mut t = (*t).clone();
                    let true = (((es).len() as i32) > 25) else { return Err("pattern mismatch") };
                    (exp, t) = replaceLiteralExp2(inExp.clone(), t.clone())?;
                    Ok((exp.clone(), t.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (exp, t) => {
                    let mut exp = (*exp).clone();
                    let mut t = (*t).clone();
                    exp = listToCons(metamodelica::AsArg::as_arg(&exp))?;
                    (exp, t) = Expression::traverseExpBottomUp(exp.clone(), &replaceLiteralExp, t.clone())?;
                    Ok((exp.clone(), t.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (exp, _) => {
                    let mut t: (i32, (metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::Exp>, i32)>>, (i32, i32, metamodelica::Array<Option<(metamodelica::Ref<DAE::Exp>, i32)>>), i32, (HashTableExpToIndex::FuncHashCref, HashTableExpToIndex::FuncCrefEqual, HashTableExpToIndex::FuncCrefStr, HashTableExpToIndex::FuncExpStr)), metamodelica::List<metamodelica::Ref<DAE::Exp>>);
                    let mut exp = (*exp).clone();
                    if '__try0: {
                        unwrap_break_err!(listToCons(metamodelica::AsArg::as_arg(&exp)), '__try0);
                        Ok::<(), &'static str>(())
                    }.is_ok() { return Err("failure(): body succeeded") }
                    (exp, t) = replaceLiteralExp2(exp.clone(), inTpl.clone())?;
                    Ok((exp.clone(), t.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (exp, _) => {
                    let mut msg: ArcStr;
                    msg = { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("function replaceLiteralExp failed. Falling back to not replacing ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(exp.clone())?); __mm_s.push_str(&*literal!(".")); ArcStr::from(__mm_s) };
                    Error::addInternalError(msg.clone(), metamodelica::sourceInfo!("SimCode/SimCodeFunctionUtil.mo"))?;
                    Ok((inExp.clone(), inTpl.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((outExp, outTpl))
}

fn replaceLiteralExp2(
    mut inExp: metamodelica::Ref<DAE::Exp>,
    mut inTpl: (
        i32,
        (
            metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::Exp>, i32)>>,
            (
                i32,
                i32,
                metamodelica::Array<Option<(metamodelica::Ref<DAE::Exp>, i32)>>,
            ),
            i32,
            (
                Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<i32> + 'static>,
                Arc<
                    dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>, metamodelica::Ref<DAE::Exp>) -> Result<bool>
                        + 'static,
                >,
                Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<ArcStr> + 'static>,
                Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>,
            ),
        ),
        metamodelica::List<metamodelica::Ref<DAE::Exp>>,
    ),
) -> Result<(
    metamodelica::Ref<DAE::Exp>,
    (
        i32,
        (
            metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::Exp>, i32)>>,
            (
                i32,
                i32,
                metamodelica::Array<Option<(metamodelica::Ref<DAE::Exp>, i32)>>,
            ),
            i32,
            (
                Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<i32> + 'static>,
                Arc<
                    dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>, metamodelica::Ref<DAE::Exp>) -> Result<bool>
                        + 'static,
                >,
                Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<ArcStr> + 'static>,
                Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>,
            ),
        ),
        metamodelica::List<metamodelica::Ref<DAE::Exp>>,
    ),
)> {
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    let mut outTpl: (
        i32,
        (
            metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::Exp>, i32)>>,
            (
                i32,
                i32,
                metamodelica::Array<Option<(metamodelica::Ref<DAE::Exp>, i32)>>,
            ),
            i32,
            (
                HashTableExpToIndex::FuncHashCref,
                HashTableExpToIndex::FuncCrefEqual,
                HashTableExpToIndex::FuncCrefStr,
                HashTableExpToIndex::FuncExpStr,
            ),
        ),
        metamodelica::List<metamodelica::Ref<DAE::Exp>>,
    );
    (outExp, outTpl) = 'mc: {
        let __mc_input = (inExp, &inTpl);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (exp, (_, ht, _)) => {
                    let mut nexp: metamodelica::Ref<DAE::Exp>;
                    let mut ix: i32;
                    ix = BaseHashTable::get(exp.clone(), &(ht.clone()))?;
                    nexp = metamodelica::Ref::new(DAE::Exp::SHARED_LITERAL { index: ix, exp: exp.clone() });
                    Ok((nexp.clone(), inTpl.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (exp, (i, ht, l)) => {
                    let mut nexp: metamodelica::Ref<DAE::Exp>;
                    let mut ht = (*ht).clone();
                    ht = BaseHashTable::add((exp.clone(), i.clone()), ht.clone())?;
                    nexp = metamodelica::Ref::new(DAE::Exp::SHARED_LITERAL { index: i.clone(), exp: exp.clone() });
                    Ok((nexp.clone(), (i.clone() + 1, ht.clone(), metamodelica::cons(exp.clone(), l.clone()))))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((outExp, outTpl))
}

fn listToCons(mut e: &metamodelica::Ref<DAE::Exp>) -> Result<metamodelica::Ref<DAE::Exp>> {
    let mut o: metamodelica::Ref<DAE::Exp>;
    o = (::match_deref::match_deref! { match e {
        Deref @ DAE::Exp::LIST { valList: es @ Deref @ metamodelica::ListNode::Cons { head: _, tail: _ } } => {
            listToCons2(metamodelica::AsArg::as_arg(&es))
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok(o)
}

fn listToCons2(mut ies: &metamodelica::List<metamodelica::Ref<DAE::Exp>>) -> metamodelica::Ref<DAE::Exp> {
    let mut o: metamodelica::Ref<DAE::Exp>;
    o = (::match_deref::match_deref! { match ies {
        Deref @ metamodelica::ListNode::Nil => {
            metamodelica::Ref::new(DAE::Exp::LIST { valList: metamodelica::nil() })
        },
        Deref @ metamodelica::ListNode::Cons { head: car, tail: es } => {
            let mut cdr: metamodelica::Ref<DAE::Exp>;
            cdr = listToCons2(es);
            metamodelica::Ref::new(DAE::Exp::CONS { car: car.clone(), cdr: cdr })
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    o
}

fn isTrivialLiteralExp(mut exp: &metamodelica::Ref<DAE::Exp>) -> Result<()> {
    let () = (::match_deref::match_deref! { match exp {
        Deref @ DAE::Exp::BOX { exp: Deref @ DAE::Exp::SCONST { string: _ } } => return Err("fail"),
        Deref @ DAE::Exp::BOX { exp: Deref @ DAE::Exp::RCONST { real: _ } } => return Err("fail"),
        Deref @ DAE::Exp::BOX { exp: _ } => (),
        Deref @ DAE::Exp::ICONST { integer: _ } => (),
        Deref @ DAE::Exp::BCONST { bool: _ } => (),
        Deref @ DAE::Exp::RCONST { real: _ } => (),
        Deref @ DAE::Exp::ENUM_LITERAL { .. } => (),
        Deref @ DAE::Exp::LIST { valList: Deref @ metamodelica::ListNode::Nil } => (),
        Deref @ DAE::Exp::META_OPTION { exp: None } => (),
        Deref @ DAE::Exp::SHARED_LITERAL { .. } => (),
        _ => return Err("fail"),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(())
}

fn isLiteralArrayExp(mut iexp: &metamodelica::Ref<DAE::Exp>) -> Result<()> {
    let () = (::match_deref::match_deref! { match iexp {
        Deref @ DAE::Exp::SCONST { string: _ } => {
            ()
        },
        Deref @ DAE::Exp::ICONST { integer: _ } => {
            ()
        },
        Deref @ DAE::Exp::RCONST { real: _ } => {
            ()
        },
        Deref @ DAE::Exp::BCONST { bool: _ } => {
            ()
        },
        Deref @ DAE::Exp::ARRAY { array: expl, .. } => {
            List::map_0(expl, &move |__a0: metamodelica::Ref<DAE::Exp>| isLiteralArrayExp(&__a0))?;
            ()
        },
        Deref @ DAE::Exp::MATRIX { matrix: expll, .. } => {
            List::map_0(&(List::flatten(expll.clone())?), &move |__a0: metamodelica::Ref<DAE::Exp>| isLiteralArrayExp(&__a0))?;
            ()
        },
        Deref @ DAE::Exp::ENUM_LITERAL { .. } => {
            ()
        },
        Deref @ DAE::Exp::META_OPTION { exp: None } => {
            ()
        },
        Deref @ DAE::Exp::META_OPTION { exp: Some(exp) } => {
            isLiteralArrayExp(metamodelica::AsArg::as_arg(&exp))?;
            ()
        },
        Deref @ DAE::Exp::BOX { exp } => {
            isLiteralArrayExp(exp)?;
            ()
        },
        Deref @ DAE::Exp::CONS { car: e1, cdr: e2 } => {
            isLiteralArrayExp(e1)?;
            isLiteralArrayExp(e2)?;
            ()
        },
        Deref @ DAE::Exp::LIST { valList: expl } => {
            List::map_0(expl, &move |__a0: metamodelica::Ref<DAE::Exp>| isLiteralArrayExp(&__a0))?;
            ()
        },
        Deref @ DAE::Exp::META_TUPLE { listExp: expl } => {
            List::map_0(expl, &move |__a0: metamodelica::Ref<DAE::Exp>| isLiteralArrayExp(&__a0))?;
            ()
        },
        Deref @ DAE::Exp::METARECORDCALL { args: expl, .. } => {
            List::map_0(expl, &move |__a0: metamodelica::Ref<DAE::Exp>| isLiteralArrayExp(&__a0))?;
            ()
        },
        Deref @ DAE::Exp::SHARED_LITERAL { .. } => {
            ()
        },
        _ => {
            return Err("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(())
}

fn isLiteralExp(mut iexp: &metamodelica::Ref<DAE::Exp>) -> Result<()> {
    let () = (::match_deref::match_deref! { match iexp {
        Deref @ DAE::Exp::SCONST { string: _ } => {
            ()
        },
        Deref @ DAE::Exp::ICONST { integer: _ } => {
            ()
        },
        Deref @ DAE::Exp::RCONST { real: _ } => {
            ()
        },
        Deref @ DAE::Exp::BCONST { bool: _ } => {
            ()
        },
        Deref @ DAE::Exp::ENUM_LITERAL { .. } => {
            ()
        },
        Deref @ DAE::Exp::META_OPTION { exp: None } => {
            ()
        },
        Deref @ DAE::Exp::META_OPTION { exp: Some(exp) } => {
            isLiteralExp(metamodelica::AsArg::as_arg(&exp))?;
            ()
        },
        Deref @ DAE::Exp::BOX { exp } => {
            isLiteralExp(exp)?;
            ()
        },
        Deref @ DAE::Exp::CONS { car: e1, cdr: e2 } => {
            isLiteralExp(e1)?;
            isLiteralExp(e2)?;
            ()
        },
        Deref @ DAE::Exp::LIST { valList: expl } => {
            List::map_0(expl, &move |__a0: metamodelica::Ref<DAE::Exp>| isLiteralExp(&__a0))?;
            ()
        },
        Deref @ DAE::Exp::META_TUPLE { listExp: expl } => {
            List::map_0(expl, &move |__a0: metamodelica::Ref<DAE::Exp>| isLiteralExp(&__a0))?;
            ()
        },
        Deref @ DAE::Exp::METARECORDCALL { args: expl, .. } => {
            List::map_0(expl, &move |__a0: metamodelica::Ref<DAE::Exp>| isLiteralExp(&__a0))?;
            ()
        },
        Deref @ DAE::Exp::SHARED_LITERAL { .. } => {
            ()
        },
        Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "listArrayLiteral" }, expLst: expl, .. } => {
            List::map_0(expl, &move |__a0: metamodelica::Ref<DAE::Exp>| isLiteralExp(&__a0))?;
            ()
        },
        _ => {
            return Err("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(())
}

fn collectRecDeclsFromTypes(
    mut inTypes: &metamodelica::List<metamodelica::Ref<DAE::Type>>,
    mut recDeclsMap: metamodelica::Ref<UnorderedMap::UnorderedMap<ArcStr, SimCodeFunction::RecordDeclaration>>,
) -> Result<()> {
    for mut ty in &**inTypes {
        collectRecDeclsFromType(metamodelica::AsArg::as_arg(&ty), recDeclsMap.clone())?;
    }
    Ok(())
}

fn collectRecDeclsFromElems(
    mut inElems: &metamodelica::List<metamodelica::Ref<DAE::Element>>,
    mut recDeclsMap: metamodelica::Ref<UnorderedMap::UnorderedMap<ArcStr, SimCodeFunction::RecordDeclaration>>,
) -> Result<()> {
    for mut elem in &**inElems {
        let () = (match &*elem.clone() {
            DAE::Element::VAR {
                binding: __elem_binding,
                ty: __elem_ty,
                ..
            } => {
                collectRecDeclsFromType(metamodelica::AsArg::as_arg(&__elem_ty), recDeclsMap.clone())?;
                if (__elem_binding).is_some() && Config::acceptMetaModelicaGrammar()? {
                    Expression::traverseExpBottomUp(
                        Util::getOption(__elem_binding.clone())?,
                        &collectRecDeclsFromMetaRecCallExp,
                        recDeclsMap.clone(),
                    )?;
                }
                ()
            }
            DAE::Element::ALGORITHM {
                algorithm_: __elem_algorithm_,
                ..
            } => {
                if Config::acceptMetaModelicaGrammar()? {
                    DAEUtil::traverseAlgorithmExps(
                        metamodelica::AsArg::as_arg(&__elem_algorithm_),
                        (std::sync::Arc::new(Expression::traverseSubexpressionsHelper)
                            as std::sync::Arc<
                                dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>, _) -> Result<_> + 'static,
                            >),
                        (
                            (std::sync::Arc::new(collectRecDeclsFromMetaRecCallExp)
                                as std::sync::Arc<
                                    dyn ::std::ops::Fn(
                                            metamodelica::Ref<DAE::Exp>,
                                            metamodelica::Ref<
                                                UnorderedMap::UnorderedMap<ArcStr, SimCodeFunction::RecordDeclaration>,
                                            >,
                                        ) -> Result<(
                                            metamodelica::Ref<DAE::Exp>,
                                            metamodelica::Ref<
                                                UnorderedMap::UnorderedMap<ArcStr, SimCodeFunction::RecordDeclaration>,
                                            >,
                                        )> + 'static,
                                >),
                            recDeclsMap.clone(),
                        ),
                    )?;
                }
                ()
            }
            _ => (),
        });
    }
    Ok(())
}

fn isVarQ(mut inElement: &metamodelica::Ref<DAE::Element>) -> bool {
    let mut outB: bool;
    outB = (match &**inElement {
        DAE::Element::VAR {
            kind: vk,
            direction: vd,
            ..
        } if (isVarKindVarOrParameter(vk.clone()) && isDirectionNotInput(vd.clone())) => true,
        _ => false,
    });
    outB
}

fn isVarNotInputNotOutput(mut inElement: &metamodelica::Ref<DAE::Element>) -> bool {
    let mut outB: bool;
    outB = (match &**inElement {
        DAE::Element::VAR {
            kind: vk,
            direction: vd,
            ..
        } if (isVarKindVarOrParameter(vk.clone()) && isDirectionNotInputNotOutput(vd.clone())) => true,
        _ => false,
    });
    outB
}

fn isVarKindVarOrParameter(mut inVarKind: DAE::VarKind) -> bool {
    let mut outB: bool;
    outB = (match inVarKind {
        DAE::VarKind::VARIABLE { .. } => true,
        DAE::VarKind::PARAM { .. } => true,
        DAE::VarKind::CONST { .. } => true,
        _ => false,
    });
    outB
}

fn isDirectionNotInput(mut inVarDirection: DAE::VarDirection) -> bool {
    let mut outB: bool;
    outB = (match inVarDirection {
        DAE::VarDirection::OUTPUT { .. } => true,
        DAE::VarDirection::BIDIR { .. } => true,
        _ => false,
    });
    outB
}

fn isDirectionNotInputNotOutput(mut inVarDirection: DAE::VarDirection) -> bool {
    let mut outB: bool;
    outB = (match inVarDirection {
        DAE::VarDirection::BIDIR { .. } => true,
        _ => false,
    });
    outB
}

fn filterNg(mut ng: i32) -> Result<i32> {
    let mut outInteger: i32;
    outInteger = if (useZerocrossing()?) { ng } else { 0 };
    Ok(outInteger)
}

fn useZerocrossing() -> Result<bool> {
    let mut res: bool;
    res = Flags::isSet(Flags::EVENTS.clone())?;
    Ok(res)
}

fn getCrefFromExp(mut e: &metamodelica::Ref<DAE::Exp>) -> Result<metamodelica::Ref<Absyn::ComponentRef>> {
    let mut c: metamodelica::Ref<Absyn::ComponentRef>;
    c = (match &**e {
        DAE::Exp::CREF {
            componentRef: crefe, ..
        } => {
            let mut crefa: metamodelica::Ref<Absyn::ComponentRef>;
            crefa = ComponentReference::unelabCref(crefe)?;
            crefa
        }
        _ => {
            Error::addInternalError(
                literal!("function getCrefFromExp failed: input was not of type DAE.CREF"),
                metamodelica::sourceInfo!("SimCode/SimCodeFunctionUtil.mo"),
            )?;
            return Err("fail");
        }
    });
    Ok(c)
}

fn collectRecDeclsFromType(
    mut inRecordType: &metamodelica::Ref<DAE::Type>,
    mut recDeclsMap: metamodelica::Ref<UnorderedMap::UnorderedMap<ArcStr, SimCodeFunction::RecordDeclaration>>,
) -> Result<()> {
    let () = (::match_deref::match_deref! { match inRecordType {
        Deref @ DAE::Type::T_COMPLEX { complexClassType: ClassInf::State::RECORD { path }, varLst: varlst, usedExternally, .. } => {
            let mut name: ArcStr;
            let mut sname: ArcStr;
            let mut vars: metamodelica::List<metamodelica::Ref<SimCodeFunction::Variable::Variable>>;
            let mut recDecl: SimCodeFunction::RecordDeclaration;
            let mut optRecDecl: Option<SimCodeFunction::RecordDeclaration>;
            let mut is_default: bool;
            let mut bool1: bool;
            let mut changed: bool;
            name = AbsynUtil::pathStringUnquoteReplaceDot(metamodelica::AsArg::as_arg(&path), literal!("_"))?;
            (sname, is_default) = checkBindingsandGetConstructorName(name.clone(), varlst);
            optRecDecl = UnorderedMap::get(sname.clone(), recDeclsMap.clone())?;
            if is_default {
                if (optRecDecl).is_some() {
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(optRecDecl) {
                        Some(SimCodeFunction::RecordDeclaration::RECORD_DECL_FULL { name: _, aliasName: _, defPath: _, variables: __pa0, usedExternally: __pa1 }) => (__pa0.clone(), __pa1.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    vars = metamodelica::Own::own(__pa0);
                    bool1 = metamodelica::Own::own(__pa1);
                    (vars, changed) = addMissingDefaults(&vars, varlst)?;
                    if changed || usedExternally.clone() && !(bool1) {
                        recDecl = SimCodeFunction::RecordDeclaration::RECORD_DECL_FULL { name: sname.clone(), aliasName: None, defPath: path.clone(), variables: vars, usedExternally: usedExternally.clone() || bool1 };
                        UnorderedMap::add(sname, recDecl, recDeclsMap)?;
                    }
                } else {
                    vars = List::map(varlst.clone(), &move |__a0: metamodelica::Ref<DAE::Var>| typesVar(&__a0))?;
                    recDecl = SimCodeFunction::RecordDeclaration::RECORD_DECL_FULL { name: sname.clone(), aliasName: None, defPath: path.clone(), variables: vars, usedExternally: usedExternally.clone() };
                    UnorderedMap::add(sname, recDecl, recDeclsMap.clone())?;
                    collectRecDeclsFromTypesVars(varlst, recDeclsMap)?;
                }
            } else {
                if (optRecDecl).is_none() {
                    vars = List::map(varlst.clone(), &move |__a0: metamodelica::Ref<DAE::Var>| typesVar(&__a0))?;
                    recDecl = SimCodeFunction::RecordDeclaration::RECORD_DECL_ADD_CONSTRCTOR { ctor_name: sname.clone(), name: name, variables: vars, defPath: path.clone(), usedExternally: usedExternally.clone() };
                    UnorderedMap::add(sname, recDecl, recDeclsMap.clone())?;
                    collectRecDeclsFromTypesVars(varlst, recDeclsMap)?;
                }
            }
            ()
        },
        Deref @ DAE::Type::T_COMPLEX { complexClassType: ClassInf::State::RECORD { path: _ }, .. } => {
            ()
        },
        Deref @ DAE::Type::T_METARECORD { path: Deref @ Absyn::Path::QUALIFIED { name: Deref @ "SourceInfo", .. }, .. } => {
            ()
        },
        Deref @ DAE::Type::T_METARECORD { fields: varlst, path, .. } => {
            let mut sname: ArcStr;
            let mut fieldNames: metamodelica::List<ArcStr>;
            sname = AbsynUtil::pathStringUnquoteReplaceDot(path, literal!("_"))?;
            fieldNames = List::map(varlst.clone(), &move |__a0: metamodelica::Ref<DAE::Var>| -> metamodelica::Result<_> { ::std::result::Result::Ok(generateVarName(&__a0)) })?;
            UnorderedMap::tryAdd(sname, SimCodeFunction::RecordDeclaration::RECORD_DECL_DEF { path: path.clone(), fieldNames: fieldNames }, recDeclsMap.clone())?;
            collectRecDeclsFromTypesVars(varlst, recDeclsMap)?;
            ()
        },
        _ => {
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(())
}

fn addMissingDefaults(
    mut inVars: &metamodelica::List<metamodelica::Ref<SimCodeFunction::Variable::Variable>>,
    mut typeVars: &metamodelica::List<metamodelica::Ref<DAE::Var>>,
) -> Result<(
    metamodelica::List<metamodelica::Ref<SimCodeFunction::Variable::Variable>>,
    bool,
)> {
    let mut vars: metamodelica::List<metamodelica::Ref<SimCodeFunction::Variable::Variable>> = metamodelica::nil();
    let mut changed: bool = false;
    let mut value: Option<metamodelica::Ref<DAE::Exp>>;
    let mut var: metamodelica::Ref<SimCodeFunction::Variable::Variable>;
    for mut v in &**inVars {
        var = v.clone();
        let () = (::match_deref::match_deref! { match &(var.clone()) {
            Deref @ SimCodeFunction::Variable::VARIABLE { value: None, .. } => {
                for mut tv in &**typeVars {
                    if stringEq(&tv.name, &(ComponentReferenceBasics::crefFirstIdent(var_field!((*var).name, SimCodeFunction::Variable::Variable::VARIABLE))?)) {
                        value = checkSourceAndGetBindingExp(&tv.binding);
                        if (value).is_some() {
                            assign_variant_field!(var => SimCodeFunction::Variable::Variable::VARIABLE; value = value);
                            changed = true;
                        }
                        break;
                    }
                }
                ()
            },
            _ => (),
            _ => unreachable!("match_deref! exhaustiveness placeholder"),
        } });
        vars = metamodelica::cons(var, vars);
    }
    vars = vars.reverse();
    Ok((vars, changed))
}

fn typesVarNoBinding(
    mut inTypesVar: &metamodelica::Ref<DAE::Var>,
) -> Result<metamodelica::Ref<SimCodeFunction::Variable::Variable>> {
    let mut outVar: metamodelica::Ref<SimCodeFunction::Variable::Variable>;
    outVar = (match &**inTypesVar {
        DAE::Var {
            name,
            attributes: attr,
            ty,
            ..
        } => {
            let mut cref_: metamodelica::Ref<DAE::ComponentRef>;
            let mut scPrl: SCode::Parallelism;
            let mut prl: DAE::VarParallelism;
            let mut ty = (*ty).clone();
            ty = Types::simplifyType(ty.clone())?;
            cref_ = ComponentReferenceBasics::makeCrefIdent(name.clone(), ty.clone(), metamodelica::nil());
            let __arc1 = attr.clone();
            let DAE::ATTR { parallelism: __pa0, .. } = &*__arc1;
            scPrl = metamodelica::Own::own(__pa0);
            prl = scodeParallelismToDAEParallelism(scPrl);
            metamodelica::Ref::new(SimCodeFunction::Variable::Variable::VARIABLE {
                name: cref_,
                ty: ty.clone(),
                value: None,
                instDims: metamodelica::nil(),
                parallelism: prl,
                kind: openmodelica_frontend_types::DAE::VarKind::VARIABLE,
                bind_from_outside: false,
            })
        }
    });
    Ok(outVar)
}

fn typesVar(
    mut inTypesVar: &metamodelica::Ref<DAE::Var>,
) -> Result<metamodelica::Ref<SimCodeFunction::Variable::Variable>> {
    let mut outVar: metamodelica::Ref<SimCodeFunction::Variable::Variable>;
    outVar = (match &**inTypesVar {
        DAE::Var {
            name,
            attributes: attr,
            ty,
            ..
        } => {
            let mut cref_: metamodelica::Ref<DAE::ComponentRef>;
            let mut scPrl: SCode::Parallelism;
            let mut prl: DAE::VarParallelism;
            let mut bindExp: Option<metamodelica::Ref<DAE::Exp>>;
            let mut ty = (*ty).clone();
            ty = Types::simplifyType(ty.clone())?;
            cref_ = ComponentReferenceBasics::makeCrefIdent(name.clone(), ty.clone(), metamodelica::nil());
            let __arc1 = attr.clone();
            let DAE::ATTR { parallelism: __pa0, .. } = &*__arc1;
            scPrl = metamodelica::Own::own(__pa0);
            prl = scodeParallelismToDAEParallelism(scPrl);
            bindExp = checkSourceAndGetBindingExp(&inTypesVar.binding);
            metamodelica::Ref::new(SimCodeFunction::Variable::Variable::VARIABLE {
                name: cref_,
                ty: ty.clone(),
                value: bindExp,
                instDims: metamodelica::nil(),
                parallelism: prl,
                kind: openmodelica_frontend_types::DAE::VarKind::VARIABLE,
                bind_from_outside: inTypesVar.bind_from_outside.clone(),
            })
        }
    });
    Ok(outVar)
}

fn checkBindingsandGetConstructorName(
    mut rec_name: ArcStr,
    mut vars: &metamodelica::List<metamodelica::Ref<DAE::Var>>,
) -> (ArcStr, bool) {
    let mut ctor_name: ArcStr;
    let mut is_default: bool;
    let mut varnum: i32;
    is_default = true;
    ctor_name = rec_name;
    varnum = 1;
    for mut var in &**vars {
        if var.bind_from_outside.clone() && !(isBindingFromDerivedRecordDeclaration(&var.binding)) {
            is_default = false;
            ctor_name = {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*ctor_name);
                __mm_s.push_str(&*literal!("_"));
                __mm_s.push_str(&*intString(varnum));
                ArcStr::from(__mm_s)
            };
        }
        varnum = intAdd(varnum, 1);
    }
    (ctor_name, is_default)
}

fn isBindingFromDerivedRecordDeclaration(mut bind: &metamodelica::Ref<DAE::Binding>) -> bool {
    let mut b: bool;
    b = (match &**bind {
        DAE::Binding::EQBOUND {
            source: DAE::BindingSource::BINDING_FROM_DERIVED_RECORD_DECL { .. },
            ..
        } => true,
        _ => false,
    });
    b
}

fn checkSourceAndGetBindingExp(mut inBinding: &metamodelica::Ref<DAE::Binding>) -> Option<metamodelica::Ref<DAE::Exp>> {
    let mut bindExp: Option<metamodelica::Ref<DAE::Exp>>;
    bindExp = (match &**inBinding {
        DAE::Binding::EQBOUND {
            source: DAE::BindingSource::BINDING_FROM_RECORD_SUBMODS { .. },
            ..
        } => None,
        DAE::Binding::EQBOUND {
            exp: __inBinding_exp, ..
        } => Some(__inBinding_exp.clone()),
        _ => None,
    });
    bindExp
}

fn scodeParallelismToDAEParallelism(mut inParallelism: SCode::Parallelism) -> DAE::VarParallelism {
    let mut outParallelism: DAE::VarParallelism;
    outParallelism = (match inParallelism {
        SCode::Parallelism::PARGLOBAL { .. } => openmodelica_frontend_types::DAE::VarParallelism::PARGLOBAL,
        SCode::Parallelism::PARLOCAL { .. } => openmodelica_frontend_types::DAE::VarParallelism::PARLOCAL,
        SCode::Parallelism::NON_PARALLEL { .. } => openmodelica_frontend_types::DAE::VarParallelism::NON_PARALLEL,
    });
    outParallelism
}

fn variableName(mut v: &metamodelica::Ref<SimCodeFunction::Variable::Variable>) -> Result<ArcStr> {
    let mut s: ArcStr;
    s = (::match_deref::match_deref! { match v {
        Deref @ SimCodeFunction::Variable::VARIABLE { name: Deref @ DAE::ComponentRef::CREF_IDENT { ident: __esc_s, .. }, .. } => {
            s = (*__esc_s).clone();
            s.clone()
        },
        Deref @ SimCodeFunction::Variable::FUNCTION_PTR { name: __esc_s, .. } => {
            s = (*__esc_s).clone();
            s.clone()
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok(s)
}

fn compareVariable(
    mut v1: &metamodelica::Ref<SimCodeFunction::Variable::Variable>,
    mut v2: &metamodelica::Ref<SimCodeFunction::Variable::Variable>,
) -> Result<bool> {
    let mut b: bool;
    b = stringCompare(&(variableName(v1)?), &(variableName(v2)?)) > 0;
    Ok(b)
}

fn generateVarName(mut inVar: &metamodelica::Ref<DAE::Var>) -> ArcStr {
    let mut outName: ArcStr;
    outName = (match &**inVar {
        DAE::Var { name, .. } => name.clone(),
        _ => {
            literal!("NULL")
        }
    });
    outName
}

fn collectRecDeclsFromTypesVars(
    mut inRecordTypeVars: &metamodelica::List<metamodelica::Ref<DAE::Var>>,
    mut recDeclsMap: metamodelica::Ref<UnorderedMap::UnorderedMap<ArcStr, SimCodeFunction::RecordDeclaration>>,
) -> Result<()> {
    for mut recTyVar in &**inRecordTypeVars {
        let () = (::match_deref::match_deref! { match &(Types::arrayElementType(&recTyVar.ty)) {
            ty @ Deref @ DAE::Type::T_COMPLEX { complexClassType: ClassInf::State::RECORD { path: _ }, .. } => {
                collectRecDeclsFromType(metamodelica::AsArg::as_arg(&ty), recDeclsMap.clone())?;
                ()
            },
            _ => {
                ()
            },
            _ => unreachable!("match_deref! exhaustiveness placeholder"),
        } });
    }
    Ok(())
}

fn collectRecDeclsFromMetaRecCallExps(
    mut inExpl: &metamodelica::List<metamodelica::Ref<DAE::Exp>>,
    mut recDeclsMap: metamodelica::Ref<UnorderedMap::UnorderedMap<ArcStr, SimCodeFunction::RecordDeclaration>>,
) -> Result<()> {
    for mut exp in &**inExpl {
        collectRecDeclsFromMetaRecCallExp(exp.clone(), recDeclsMap.clone())?;
    }
    Ok(())
}

fn collectRecDeclsFromMetaRecCallExp(
    mut inExp: metamodelica::Ref<DAE::Exp>,
    mut recDeclsMap: metamodelica::Ref<UnorderedMap::UnorderedMap<ArcStr, SimCodeFunction::RecordDeclaration>>,
) -> Result<(
    metamodelica::Ref<DAE::Exp>,
    metamodelica::Ref<UnorderedMap::UnorderedMap<ArcStr, SimCodeFunction::RecordDeclaration>>,
)> {
    let mut inExp: metamodelica::Ref<DAE::Exp> = inExp;
    let mut recDeclsMap: metamodelica::Ref<UnorderedMap::UnorderedMap<ArcStr, SimCodeFunction::RecordDeclaration>> =
        recDeclsMap;
    let mut name: ArcStr;
    let () = (match &*inExp {
        DAE::Exp::METARECORDCALL {
            fieldNames: __inExp_fieldNames,
            index: __inExp_index,
            path: __inExp_path,
            ..
        } => {
            if __inExp_index.clone() != -1 {
                name =
                    AbsynUtil::pathStringUnquoteReplaceDot(metamodelica::AsArg::as_arg(&__inExp_path), literal!("_"))?;
                UnorderedMap::tryAdd(
                    name,
                    SimCodeFunction::RecordDeclaration::RECORD_DECL_DEF {
                        path: __inExp_path.clone(),
                        fieldNames: __inExp_fieldNames.clone(),
                    },
                    recDeclsMap.clone(),
                )?;
            }
            ()
        }
        _ => (),
    });
    Ok((inExp, recDeclsMap))
}

fn generateExtFunctionIncludes(
    mut program: &Absyn::Program,
    mut path: &metamodelica::Ref<Absyn::Path>,
    mut inAbsynAnnotationOption: Option<metamodelica::Ref<SCode::Annotation>>,
    mut info: &SourceInfo,
) -> Result<(
    metamodelica::List<ArcStr>,
    metamodelica::List<ArcStr>,
    metamodelica::List<ArcStr>,
    metamodelica::List<ArcStr>,
    bool,
)> {
    let mut includes: metamodelica::List<ArcStr>;
    let mut includeDirs: metamodelica::List<ArcStr>;
    let mut libs: metamodelica::List<ArcStr>;
    let mut paths: metamodelica::List<ArcStr>;
    let mut dynamcLoad: bool;
    (includes, includeDirs, libs, paths, dynamcLoad) = (::match_deref::match_deref! { match &(inAbsynAnnotationOption) {
        Some(Deref @ SCode::Annotation { modification: r#mod }) => {
            let mut b: bool;
            let mut isWasm: bool;
            let mut target: ArcStr;
            let mut resources: Option<ArcStr>;
            let mut libNames: metamodelica::List<ArcStr>;
            let mut fullLibNames: metamodelica::List<ArcStr>;
            let mut dirs: metamodelica::List<ArcStr>;
            b = generateExtFunctionDynamicLoad(metamodelica::AsArg::as_arg(&r#mod));
            target = Flags::getConfigString(Flags::TARGET.clone())?;
            (libs, libNames) = generateExtFunctionIncludesLibstr(target.clone(), metamodelica::AsArg::as_arg(&r#mod))?;
            includes = generateExtFunctionIncludesIncludestr(metamodelica::AsArg::as_arg(&r#mod));
            (libs, dirs, resources) = generateExtFunctionLibraryDirectoryFlags(program, path, metamodelica::AsArg::as_arg(&r#mod), libs);
            isWasm = isWasmSimCodeTarget()?;
            paths = generateExtFunctionLibraryDirectoryPaths(program, path, metamodelica::AsArg::as_arg(&r#mod));
            if isWasm {
                dirs = List::union(&dirs, &paths);
            }
            for mut name in &*if (Flags::isSet(Flags::CHECK_EXT_LIBS.clone())?) {libNames} else {metamodelica::nil()} {
                if metamodelica::stringEq(&(getGerneralTarget(target.clone())?), &(literal!("msvc"))) || metamodelica::stringEq(&arcstr::literal!(Autoconf::os), &(literal!("Windows_NT"))) {
                    fullLibNames = list![{ let mut __mm_s = String::new(); __mm_s.push_str(&*name); __mm_s.push_str(&*arcstr::literal!(Autoconf::dllExt)); ArcStr::from(__mm_s) }, { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("lib")); __mm_s.push_str(&*name); __mm_s.push_str(&*literal!(".a")); ArcStr::from(__mm_s) }, { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("lib")); __mm_s.push_str(&*name); __mm_s.push_str(&*literal!(".lib")); ArcStr::from(__mm_s) }];
                } else {
                    fullLibNames = list![{ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("lib")); __mm_s.push_str(&*name); __mm_s.push_str(&*literal!(".a")); ArcStr::from(__mm_s) }, { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("lib")); __mm_s.push_str(&*name); __mm_s.push_str(&*arcstr::literal!(Autoconf::dllExt)); ArcStr::from(__mm_s) }];
                }
                lookForExtFunctionLibrary(fullLibNames, dirs.clone(), name.clone(), resources.clone(), path, info, false)?;
                if isWasm {
                    lookForExtFunctionLibrary(list![{ let mut __mm_s = String::new(); __mm_s.push_str(&*name); __mm_s.push_str(&*literal!(".wasm")); ArcStr::from(__mm_s) }, { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("lib")); __mm_s.push_str(&*name); __mm_s.push_str(&*literal!(".wasm")); ArcStr::from(__mm_s) }], dirs.clone(), name.clone(), resources.clone(), path, info, true)?;
                }
            }
            if isWasm {
                paths = List::union(&paths, &(({
        let mut __acc: metamodelica::List<ArcStr> = metamodelica::nil();
        for mut d in (dirs.reverse()).into_iter().cloned() {
            if !(System::directoryExists(d.clone())) { continue; }
            let __x = d.clone();
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    })));
            }
            includeDirs = generateExtFunctionIncludeDirectoryFlags(program, path, metamodelica::AsArg::as_arg(&r#mod), &includes);
            (includes, includeDirs, libs, paths, b)
        },
        None => {
            (metamodelica::nil(), metamodelica::nil(), metamodelica::nil(), metamodelica::nil(), false)
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((includes, includeDirs, libs, paths, dynamcLoad))
}

fn lookForExtFunctionLibrary(
    mut names: metamodelica::List<ArcStr>,
    mut dirs: metamodelica::List<ArcStr>,
    mut name: ArcStr,
    mut resources: Option<ArcStr>,
    mut path: &metamodelica::Ref<Absyn::Path>,
    mut info: &SourceInfo,
    mut forWasm: bool,
) -> Result<()> {
    let mut dirs2: metamodelica::List<ArcStr>;
    dirs2 = metamodelica::cons(
        {
            let mut __mm_s = String::new();
            __mm_s.push_str(&*Settings::getInstallationDirectoryPath()?);
            __mm_s.push_str(&*literal!("/lib/"));
            __mm_s.push_str(&*arcstr::literal!(Autoconf::triple));
            __mm_s.push_str(&*literal!("/omc"));
            ArcStr::from(__mm_s)
        },
        metamodelica::cons(
            {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("/usr/lib/"));
                __mm_s.push_str(&*arcstr::literal!(Autoconf::triple));
                ArcStr::from(__mm_s)
            },
            metamodelica::cons(
                {
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!("/lib/"));
                    __mm_s.push_str(&*arcstr::literal!(Autoconf::triple));
                    ArcStr::from(__mm_s)
                },
                metamodelica::cons(literal!("/usr/lib/"), metamodelica::cons(literal!("/lib/"), dirs)),
            ),
        ),
    );
    if !({
        let mut __acc: Option<bool> = None;
        for mut n in (names.clone()).into_iter().cloned() {
            let __x = ({
                let mut __acc: Option<bool> = None;
                for mut d in (dirs2.clone()).into_iter().cloned() {
                    let __x = System::regularFileExists({
                        let mut __mm_s = String::new();
                        __mm_s.push_str(&*d);
                        __mm_s.push_str(&*literal!("/"));
                        __mm_s.push_str(&*n);
                        ArcStr::from(__mm_s)
                    });
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
                __acc.unwrap_or(false)
            });
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
        __acc.unwrap_or(false)
    }) {
        let () = (match resources {
            Some(mut resourcesStr) => {
                let mut tmpdir: ArcStr;
                let mut srcdir: ArcStr;
                let mut builddir: ArcStr;
                let mut cmd: ArcStr;
                let mut pwd: ArcStr;
                let mut contents: ArcStr;
                let mut found: ArcStr;
                let mut status: i32;
                let mut didFind: bool;
                let mut isCMake: bool;
                if System::directoryExists(resourcesStr.clone())
                    && !(extLibraryBuildAttempted(&resourcesStr, &name, forWasm))
                {
                    didFind = false;
                    for mut dir in &*extLibraryBuildProjects(&resourcesStr, forWasm) {
                        srcdir = {
                            let mut __mm_s = String::new();
                            __mm_s.push_str(&*resourcesStr);
                            __mm_s.push_str(&*if (metamodelica::stringEq(&dir, &(literal!(".")))) {
                                literal!("")
                            } else {
                                {
                                    let mut __mm_s = String::new();
                                    __mm_s.push_str(&*literal!("/"));
                                    __mm_s.push_str(&*dir);
                                    ArcStr::from(__mm_s)
                                }
                            });
                            ArcStr::from(__mm_s)
                        };
                        isCMake = System::regularFileExists({
                            let mut __mm_s = String::new();
                            __mm_s.push_str(&*srcdir);
                            __mm_s.push_str(&*literal!("/CMakeLists.txt"));
                            ArcStr::from(__mm_s)
                        });
                        tmpdir = System::createTemporaryDirectory({
                            let mut __mm_s = String::new();
                            __mm_s.push_str(&*Settings::getTempDirectoryPath());
                            __mm_s.push_str(&*literal!("/omc_compile_"));
                            __mm_s.push_str(&*name);
                            __mm_s.push_str(&*literal!("_"));
                            ArcStr::from(__mm_s)
                        })?;
                        Error::addSourceMessage(
                            &(Error::COMPILER_NOTIFICATION.clone()),
                            list![{
                                let mut __mm_s = String::new();
                                __mm_s.push_str(&*literal!("Created directory "));
                                __mm_s.push_str(&*tmpdir);
                                ArcStr::from(__mm_s)
                            }],
                            info,
                        )?;
                        if isCMake {
                            builddir = tmpdir.clone();
                        } else {
                            builddir = {
                                let mut __mm_s = String::new();
                                __mm_s.push_str(&*tmpdir);
                                __mm_s.push_str(&*literal!("/"));
                                __mm_s.push_str(&*dir);
                                ArcStr::from(__mm_s)
                            };
                            cmd = {
                                let mut __mm_s = String::new();
                                __mm_s.push_str(&*literal!("cp -a \""));
                                __mm_s.push_str(&*resourcesStr);
                                __mm_s.push_str(&*literal!("\"/* \""));
                                __mm_s.push_str(&*tmpdir);
                                __mm_s.push_str(&*literal!("\""));
                                ArcStr::from(__mm_s)
                            };
                            Error::addSourceMessage(&(Error::COMPILER_NOTIFICATION.clone()), list![cmd.clone()], info)?;
                            System::systemCall(cmd, literal!(""));
                        }
                        pwd = System::pwd();
                        if 0 == System::cd(builddir.clone()) {
                            Error::addSourceMessage(
                                &(Error::COMPILER_NOTIFICATION.clone()),
                                list![{
                                    let mut __mm_s = String::new();
                                    __mm_s.push_str(&*literal!("Changed directory to "));
                                    __mm_s.push_str(&*System::pwd());
                                    ArcStr::from(__mm_s)
                                }],
                                info,
                            )?;
                            cmd = extLibraryBuildCommand(
                                path,
                                forWasm,
                                &(if (isCMake) { srcdir } else { literal!("") }),
                            )?;
                            status = System::systemCall(cmd.clone(), literal!("log"));
                            contents = System::readFile(literal!("log"))?;
                            if status != 0 {
                                Error::addSourceMessage(
                                    &(Error::COMPILER_WARNING.clone()),
                                    list![{
                                        let mut __mm_s = String::new();
                                        __mm_s.push_str(&*literal!("Failed to run "));
                                        __mm_s.push_str(&*cmd);
                                        __mm_s.push_str(&*literal!(": "));
                                        __mm_s.push_str(&*contents);
                                        ArcStr::from(__mm_s)
                                    }],
                                    info,
                                )?;
                            } else {
                                Error::addSourceMessage(
                                    &(Error::COMPILER_NOTIFICATION.clone()),
                                    list![{
                                        let mut __mm_s = String::new();
                                        __mm_s.push_str(&*literal!("Succeeded with compilation and installation of the library using:\ncommand: "));
                                        __mm_s.push_str(&*cmd);
                                        __mm_s.push_str(&*literal!("\n"));
                                        __mm_s.push_str(&*contents);
                                        ArcStr::from(__mm_s)
                                    }],
                                    info,
                                )?;
                                didFind = ({
                                    let mut __acc: Option<bool> = None;
                                    for mut n in (names.clone()).into_iter().cloned() {
                                        let __x = ({
                                            let mut __acc: Option<bool> = None;
                                            for mut d in (dirs2.clone()).into_iter().cloned() {
                                                let __x = System::regularFileExists({
                                                    let mut __mm_s = String::new();
                                                    __mm_s.push_str(&*d);
                                                    __mm_s.push_str(&*literal!("/"));
                                                    __mm_s.push_str(&*n);
                                                    ArcStr::from(__mm_s)
                                                });
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
                                            __acc.unwrap_or(false)
                                        });
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
                                    __acc.unwrap_or(false)
                                });
                                if didFind {
                                    found = ({
                                        let mut __acc: metamodelica::List<ArcStr> = metamodelica::nil();
                                        for mut x in (List::flatten(
                                            ({
                                                let mut __acc: metamodelica::List<metamodelica::List<ArcStr>> =
                                                    metamodelica::nil();
                                                for mut n in (names.clone()).into_iter().cloned() {
                                                    let __x = ({
                                                        let mut __acc: metamodelica::List<ArcStr> = metamodelica::nil();
                                                        for mut d in (dirs2.clone()).into_iter().cloned() {
                                                            let __x = {
                                                                let mut __mm_s = String::new();
                                                                __mm_s.push_str(&*d);
                                                                __mm_s.push_str(&*literal!("/"));
                                                                __mm_s.push_str(&*n);
                                                                ArcStr::from(__mm_s)
                                                            };
                                                            __acc = cons(__x, __acc);
                                                        }
                                                        __acc.reverse()
                                                    });
                                                    __acc = cons(__x, __acc);
                                                }
                                                __acc.reverse()
                                            }),
                                        )?)
                                        .into_iter()
                                        .cloned()
                                        {
                                            if !(System::regularFileExists(x.clone())) {
                                                continue;
                                            }
                                            let __x = x.clone();
                                            __acc = cons(__x, __acc);
                                        }
                                        __acc.reverse()
                                    })
                                    .head()
                                    .cloned()?;
                                    Error::addSourceMessage(
                                        &(Error::COMPILER_NOTIFICATION.clone()),
                                        list![{
                                            let mut __mm_s = String::new();
                                            __mm_s.push_str(&*literal!("Compiled "));
                                            __mm_s.push_str(&*found);
                                            __mm_s.push_str(&*literal!(" by running build project "));
                                            __mm_s.push_str(&*resourcesStr);
                                            __mm_s.push_str(&*literal!("/"));
                                            __mm_s.push_str(&*dir);
                                            ArcStr::from(__mm_s)
                                        }],
                                        info,
                                    )?;
                                } else {
                                    Error::addSourceMessage(
                                        &(Error::EXT_LIBRARY_NOT_FOUND_DESPITE_COMPILATION_SUCCESS.clone()),
                                        list![name.clone(), cmd, System::pwd()],
                                        info,
                                    )?;
                                }
                            }
                        } else {
                            Error::addSourceMessage(
                                &(Error::COMPILER_WARNING.clone()),
                                list![{
                                    let mut __mm_s = String::new();
                                    __mm_s.push_str(&*literal!("Failed to change directory to "));
                                    __mm_s.push_str(&*builddir);
                                    ArcStr::from(__mm_s)
                                }],
                                info,
                            )?;
                        }
                        System::cd(pwd);
                        System::removeDirectory(tmpdir.clone());
                        Error::addSourceMessage(
                            &(Error::COMPILER_NOTIFICATION.clone()),
                            list![{
                                let mut __mm_s = String::new();
                                __mm_s.push_str(&*literal!("Removed directory "));
                                __mm_s.push_str(&*tmpdir);
                                ArcStr::from(__mm_s)
                            }],
                            info,
                        )?;
                        if didFind {
                            break;
                        }
                    }
                }
                ()
            }
            _ => (),
        });
        if !({
            let mut __acc: Option<bool> = None;
            for mut n in (names.clone()).into_iter().cloned() {
                let __x = ({
                    let mut __acc: Option<bool> = None;
                    for mut d in (dirs2.clone()).into_iter().cloned() {
                        let __x = System::regularFileExists({
                            let mut __mm_s = String::new();
                            __mm_s.push_str(&*d);
                            __mm_s.push_str(&*literal!("/"));
                            __mm_s.push_str(&*n);
                            ArcStr::from(__mm_s)
                        });
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
                    __acc.unwrap_or(false)
                });
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
            __acc.unwrap_or(false)
        }) {
            if !(forWasm || Testsuite::isRunning()?) {
                Error::addSourceMessage(
                    &(Error::EXT_LIBRARY_NOT_FOUND.clone()),
                    list![
                        name,
                        ({
                            let mut __acc = String::new();
                            for mut n in (names).into_iter().cloned() {
                                let __x = ({
                                    let mut __acc = String::new();
                                    for mut d in (dirs2.clone()).into_iter().cloned() {
                                        let __x = {
                                            let mut __mm_s = String::new();
                                            __mm_s.push_str(&*literal!("\n  "));
                                            __mm_s.push_str(&*d);
                                            __mm_s.push_str(&*literal!("/"));
                                            __mm_s.push_str(&*n);
                                            ArcStr::from(__mm_s)
                                        };
                                        __acc.push_str(&__x);
                                    }
                                    ArcStr::from(__acc)
                                });
                                __acc.push_str(&__x);
                            }
                            ArcStr::from(__acc)
                        })
                        .clone()
                    ],
                    info,
                )?;
            }
        }
    }
    Ok(())
}

fn generateExtFunctionIncludeDirectoryFlags(
    mut program: &Absyn::Program,
    mut path: &metamodelica::Ref<Absyn::Path>,
    mut inMod: &metamodelica::Ref<SCode::Mod>,
    mut includes: &metamodelica::List<ArcStr>,
) -> metamodelica::List<ArcStr> {
    let mut outDirs: metamodelica::List<ArcStr>;
    outDirs = 'mc: {
        let __mc_input = &**includes;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Nil => {
                    Ok(metamodelica::nil())
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
                    let mut istr: ArcStr;
                    let __pa0 = ::match_deref::match_deref! { match &(Mod::getUnelabedSubMod(inMod, &(literal!("IncludeDirectory")))?) {
                        Deref @ SCode::Mod::MOD { binding: Some(Deref @ Absyn::Exp::STRING { value: __pa0 }), .. } => __pa0.clone(),
                        _ => return Err("pattern mismatch"),
                    } };
                    r#str = metamodelica::Own::own(__pa0);
                    r#str = ProgramUtil::getFullPathFromUri(program, r#str.clone(), false)?;
                    istr = { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\"-I")); __mm_s.push_str(&*r#str); __mm_s.push_str(&*literal!("\"")); ArcStr::from(__mm_s) };
                    Ok(if (System::directoryExists(r#str.clone())) {list![istr.clone()]} else {metamodelica::nil()})
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
                    let mut istr: ArcStr;
                    r#str = { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("modelica://")); __mm_s.push_str(&*AbsynUtil::pathFirstIdent(path)); __mm_s.push_str(&*literal!("/Resources/Include")); ArcStr::from(__mm_s) };
                    r#str = ProgramUtil::getFullPathFromUri(program, r#str.clone(), false)?;
                    istr = { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\"-I")); __mm_s.push_str(&*r#str); __mm_s.push_str(&*literal!("\"")); ArcStr::from(__mm_s) };
                    Ok(if (System::directoryExists(r#str.clone())) {list![istr.clone()]} else {metamodelica::nil()})
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
    outDirs
}

fn getLinkerLibraryPaths(
    mut uri: ArcStr,
    mut path: &metamodelica::Ref<Absyn::Path>,
    mut inLibs: &metamodelica::List<ArcStr>,
) -> Result<metamodelica::List<ArcStr>> {
    let mut libPaths: metamodelica::List<ArcStr> = metamodelica::nil();
    let mut installationDir: ArcStr;
    installationDir = Settings::getInstallationDirectoryPath()?;
    let () = (::match_deref::match_deref! { match inLibs {
        Deref @ metamodelica::ListNode::Cons { head: Deref @ "-lWinmm", tail: Deref @ metamodelica::ListNode::Nil } if (metamodelica::stringEq(&arcstr::literal!(Autoconf::os), &(literal!("Windows_NT")))) => {
            libPaths = list![{ let mut __mm_s = String::new(); __mm_s.push_str(&*installationDir); __mm_s.push_str(&*literal!("/lib/")); __mm_s.push_str(&*arcstr::literal!(Autoconf::triple)); __mm_s.push_str(&*literal!("/omc")); ArcStr::from(__mm_s) }];
            ()
        },
        _ => {
            libPaths = list![uri.clone(), { let mut __mm_s = String::new(); __mm_s.push_str(&*uri); __mm_s.push_str(&*literal!("/")); __mm_s.push_str(&*System::modelicaPlatform()); ArcStr::from(__mm_s) }, { let mut __mm_s = String::new(); __mm_s.push_str(&*uri); __mm_s.push_str(&*literal!("/")); __mm_s.push_str(&*System::openModelicaPlatform()); ArcStr::from(__mm_s) }, { let mut __mm_s = String::new(); __mm_s.push_str(&*uri); __mm_s.push_str(&*literal!("/")); __mm_s.push_str(&*System::openModelicaPlatformAlternative()); ArcStr::from(__mm_s) }, { let mut __mm_s = String::new(); __mm_s.push_str(&*Settings::getHomeDir(false)); __mm_s.push_str(&*literal!("/.openmodelica/binaries/")); __mm_s.push_str(&*AbsynUtil::pathFirstIdent(path)); ArcStr::from(__mm_s) }, { let mut __mm_s = String::new(); __mm_s.push_str(&*userCompiledBinariesDirectory(path)); __mm_s.push_str(&*literal!("/Library/")); __mm_s.push_str(&*System::modelicaPlatform()); ArcStr::from(__mm_s) }, { let mut __mm_s = String::new(); __mm_s.push_str(&*installationDir); __mm_s.push_str(&*literal!("/lib/")); ArcStr::from(__mm_s) }, { let mut __mm_s = String::new(); __mm_s.push_str(&*installationDir); __mm_s.push_str(&*literal!("/lib/")); __mm_s.push_str(&*arcstr::literal!(Autoconf::triple)); __mm_s.push_str(&*literal!("/omc")); ArcStr::from(__mm_s) }];
            if isWasmSimCodeTarget()? {
                libPaths = metamodelica::cons({ let mut __mm_s = String::new(); __mm_s.push_str(&*userCompiledBinariesDirectory(path)); __mm_s.push_str(&*literal!("/")); __mm_s.push_str(&*arcstr::literal!(wasmLibraryTriple)); ArcStr::from(__mm_s) }, metamodelica::cons({ let mut __mm_s = String::new(); __mm_s.push_str(&*userCompiledBinariesDirectory(path)); __mm_s.push_str(&*literal!("/")); __mm_s.push_str(&*arcstr::literal!(wasmLibraryTriple)); __mm_s.push_str(&*literal!("/Library/")); __mm_s.push_str(&*arcstr::literal!(wasmLibraryTriple)); ArcStr::from(__mm_s) }, libPaths));
            }
            if metamodelica::stringEq(&arcstr::literal!(Autoconf::os), &(literal!("Windows_NT"))) {
                libPaths = List::appendElt({ let mut __mm_s = String::new(); __mm_s.push_str(&*installationDir); __mm_s.push_str(&*literal!("/bin/")); ArcStr::from(__mm_s) }, libPaths);
            }
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(libPaths)
}

fn generateExtFunctionLibraryDirectoryFlags(
    mut program: &Absyn::Program,
    mut path: &metamodelica::Ref<Absyn::Path>,
    mut inMod: &metamodelica::Ref<SCode::Mod>,
    mut inLibs: metamodelica::List<ArcStr>,
) -> (metamodelica::List<ArcStr>, metamodelica::List<ArcStr>, Option<ArcStr>) {
    let mut outLibs: metamodelica::List<ArcStr>;
    let mut installDirs: metamodelica::List<ArcStr>;
    let mut resources: Option<ArcStr>;
    (outLibs, installDirs, resources) = 'mc: {
        let __mc_input = inLibs.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Nil => {
                    Ok((metamodelica::nil(), metamodelica::nil(), None))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                        libs => {
                            let mut r#str: ArcStr = arcstr::literal!("");
                            let mut target: ArcStr;
                            let mut resourcesStr: ArcStr;
                            let mut libs2: metamodelica::List<ArcStr>;
                            let mut isLinux: bool;
                            let mut libs = (*libs).clone();
                            r#str = 'mc: {
                let __mc_input = &**inMod;
                if let Ok(__v) = (|| -> Result<_> {
                            ::match_deref::match_deref! { match &__mc_input {
                                _ => {
                                    let mut r#str: ArcStr;
                                    let __pa0 = ::match_deref::match_deref! { match &(Mod::getUnelabedSubMod(inMod, &(literal!("LibraryDirectory")))?) {
                                                Deref @ SCode::Mod::MOD { binding: Some(Deref @ Absyn::Exp::STRING { value: __pa0 }), .. } => __pa0.clone(),
                                                _ => return Err("pattern mismatch"),
                                    } };
                                    r#str = metamodelica::Own::own(__pa0);
                                    Ok(r#str.clone())
                                }
                                _ => return Err("nomatch"),
                            }}
                })() { break 'mc __v; }
                if let Ok(__v) = (|| -> Result<_> {
                            ::match_deref::match_deref! { match &__mc_input {
                                _ => {
                                    Ok({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("modelica://")); __mm_s.push_str(&*AbsynUtil::pathFirstIdent(path)); __mm_s.push_str(&*literal!("/Resources/Library")); ArcStr::from(__mm_s) })
                                }
                                _ => return Err("nomatch"),
                            }}
                })() { break 'mc __v; }
                return Err("matchcontinue: no arm matched")
            };
                            r#str = ProgramUtil::getFullPathFromUri(program, r#str.clone(), false)?;
                            resourcesStr = ProgramUtil::getFullPathFromUri(program, { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("modelica://")); __mm_s.push_str(&*AbsynUtil::pathFirstIdent(path)); __mm_s.push_str(&*literal!("/Resources")); ArcStr::from(__mm_s) }, false)?;
                            isLinux = stringEq(&(literal!("linux")), &arcstr::literal!(Autoconf::os));
                            target = Flags::getConfigString(Flags::TARGET.clone())?;
                            libs2 = getLinkerLibraryPaths(r#str.clone(), path, &inLibs)?;
                            libs = List::fold2(&libs2, &move |__a0: ArcStr, __a1: bool, __a2: ArcStr, __a3: metamodelica::List<ArcStr>| generateExtFunctionLibraryDirectoryFlags2(&__a0, __a1, __a2, __a3), isLinux, target.clone(), libs.clone())?;
                            Ok((libs.clone(), libs2.clone().reverse(), Some(resourcesStr.clone())))
                        }
                        _ => return Err("nomatch"),
                    }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((inLibs.clone(), metamodelica::nil(), None))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    (outLibs, installDirs, resources)
}

fn generateExtFunctionLibraryDirectoryFlags2(
    mut dir: &ArcStr,
    mut isLinux: bool,
    mut target: ArcStr,
    mut inLibs: metamodelica::List<ArcStr>,
) -> Result<metamodelica::List<ArcStr>> {
    let mut libs: metamodelica::List<ArcStr>;
    if isWasmSimCodeTarget()? {
        libs = inLibs;
        return Ok(libs);
    }
    libs = if (isLinux) {
        metamodelica::cons(
            {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("-Wl,-rpath=\""));
                __mm_s.push_str(&*dir);
                __mm_s.push_str(&*literal!("\""));
                ArcStr::from(__mm_s)
            },
            inLibs,
        )
    } else {
        inLibs
    };
    libs = metamodelica::cons(
        if (metamodelica::stringEq(&(getGerneralTarget(target)?), &(literal!("msvc")))) {
            {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("/LIBPATH:\""));
                __mm_s.push_str(&*dir);
                __mm_s.push_str(&*literal!("\""));
                ArcStr::from(__mm_s)
            }
        } else {
            {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("\"-L"));
                __mm_s.push_str(&*dir);
                __mm_s.push_str(&*literal!("\""));
                ArcStr::from(__mm_s)
            }
        },
        libs,
    );
    Ok(libs)
}

fn getGerneralTarget(mut target: ArcStr) -> Result<ArcStr> {
    let mut generalTarget: ArcStr;
    generalTarget = if (System::stringFind(target.clone(), literal!("msvc"))? == 0) {
        literal!("msvc")
    } else {
        target
    };
    Ok(generalTarget)
}

fn userCompiledBinariesDirectory(mut path: &metamodelica::Ref<Absyn::Path>) -> ArcStr {
    let mut r#str: ArcStr = {
        let mut __mm_s = String::new();
        __mm_s.push_str(&*Settings::getHomeDir(false));
        __mm_s.push_str(&*literal!("/.openmodelica/binaries/"));
        __mm_s.push_str(&*AbsynUtil::pathFirstIdent(path));
        ArcStr::from(__mm_s)
    };
    r#str
}

pub(crate) const wasmLibraryTriple: &'static str = "wasm32-wasip1";

fn extLibraryBuildProjects(mut resources: &ArcStr, mut forWasm: bool) -> metamodelica::List<ArcStr> {
    let mut projects: metamodelica::List<ArcStr>;
    let mut dirs: metamodelica::List<ArcStr>;
    let mut cmakeProjects: metamodelica::List<ArcStr>;
    dirs = ({
        let mut __acc: metamodelica::List<ArcStr> = metamodelica::nil();
        for mut d in (System::subDirectories({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*resources);
            __mm_s.push_str(&*literal!("/BuildProjects"));
            ArcStr::from(__mm_s)
        }))
        .into_iter()
        .cloned()
        {
            let __x = {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("BuildProjects/"));
                __mm_s.push_str(&*d);
                ArcStr::from(__mm_s)
            };
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    if System::regularFileExists({
        let mut __mm_s = String::new();
        __mm_s.push_str(&*resources);
        __mm_s.push_str(&*literal!("/CMakeLists.txt"));
        ArcStr::from(__mm_s)
    }) {
        dirs = metamodelica::cons(literal!("."), dirs);
    }
    cmakeProjects = ({
        let mut __acc: metamodelica::List<ArcStr> = metamodelica::nil();
        for mut d in (dirs.clone()).into_iter().cloned() {
            if !(System::regularFileExists({
                let mut __mm_s = String::new();
                __mm_s.push_str(&*resources);
                __mm_s.push_str(&*literal!("/"));
                __mm_s.push_str(&*d);
                __mm_s.push_str(&*literal!("/CMakeLists.txt"));
                ArcStr::from(__mm_s)
            })) {
                continue;
            }
            let __x = d.clone();
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    projects = if (forWasm) {
        cmakeProjects
    } else {
        listAppend(
            cmakeProjects.clone(),
            ({
                let mut __acc: metamodelica::List<ArcStr> = metamodelica::nil();
                for mut d in (dirs).into_iter().cloned() {
                    if !(!(listMember(d.clone(), cmakeProjects.clone()))
                        && System::regularFileExists({
                            let mut __mm_s = String::new();
                            __mm_s.push_str(&*resources);
                            __mm_s.push_str(&*literal!("/"));
                            __mm_s.push_str(&*d);
                            __mm_s.push_str(&*literal!("/autogen.sh"));
                            ArcStr::from(__mm_s)
                        }))
                    {
                        continue;
                    }
                    let __x = d.clone();
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            }),
        )
    };
    projects
}

fn extLibraryBuildAttempted(mut resources: &ArcStr, mut name: &ArcStr, mut forWasm: bool) -> bool {
    let mut attempted: bool;
    let mut done: metamodelica::List<ArcStr>;
    let mut key: ArcStr = {
        let mut __mm_s = String::new();
        __mm_s.push_str(&*resources);
        __mm_s.push_str(&*literal!("\n"));
        __mm_s.push_str(&*name);
        __mm_s.push_str(&*literal!("\n"));
        __mm_s.push_str(&*ArcStr::from(::std::format!("{}", forWasm)));
        ArcStr::from(__mm_s)
    };
    done = openmodelica_util::Globals::extLibraryBuildIndex.with(|__root| __root.borrow().clone());
    attempted = listMember(key.clone(), done.clone());
    if !(attempted) {
        {
            let __v = metamodelica::cons(key, done);
            openmodelica_util::Globals::extLibraryBuildIndex.with(|__root| *__root.borrow_mut() = __v)
        };
    }
    attempted
}

fn extLibraryBuildCommand(
    mut path: &metamodelica::Ref<Absyn::Path>,
    mut forWasm: bool,
    mut cmakeSourceDir: &ArcStr,
) -> Result<ArcStr> {
    let mut cmd: ArcStr;
    let mut libdir: ArcStr;
    let mut extra: ArcStr;
    libdir = userCompiledBinariesDirectory(path);
    if !metamodelica::stringEq(&cmakeSourceDir, &(literal!(""))) {
        if forWasm {
            libdir = {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*libdir);
                __mm_s.push_str(&*literal!("/"));
                __mm_s.push_str(&*arcstr::literal!(wasmLibraryTriple));
                ArcStr::from(__mm_s)
            };
            extra = {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!(" -DBUILD_SHARED_LIBS=ON \"-DCMAKE_TOOLCHAIN_FILE="));
                __mm_s.push_str(&*Settings::getInstallationDirectoryPath()?);
                __mm_s.push_str(&*literal!("/share/omc/cmake/"));
                __mm_s.push_str(&*arcstr::literal!(wasmLibraryTriple));
                __mm_s.push_str(&*literal!(".cmake\""));
                ArcStr::from(__mm_s)
            };
        } else {
            extra = literal!("");
        }
        extra = {
            let mut __mm_s = String::new();
            __mm_s.push_str(&*extra);
            __mm_s.push_str(&*literal!(" \"-DMODELICA_UTILITIES_INCLUDE_DIR="));
            __mm_s.push_str(&*Settings::getInstallationDirectoryPath()?);
            __mm_s.push_str(&*literal!("/include/omc/c\""));
            ArcStr::from(__mm_s)
        };
        cmd = {
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("cmake -DCMAKE_BUILD_TYPE=Release \"-DCMAKE_INSTALL_PREFIX="));
            __mm_s.push_str(&*libdir);
            __mm_s.push_str(&*literal!("\" -DCMAKE_INSTALL_LIBDIR=."));
            __mm_s.push_str(&*extra);
            __mm_s.push_str(&*literal!(" \""));
            __mm_s.push_str(&*cmakeSourceDir);
            __mm_s.push_str(&*literal!("\" && cmake --build . && cmake --build . --target install"));
            ArcStr::from(__mm_s)
        };
    } else {
        cmd = {
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("sh ./autogen.sh && ./configure --libdir='"));
            __mm_s.push_str(&*libdir);
            __mm_s.push_str(&*literal!("' && make && make install"));
            ArcStr::from(__mm_s)
        };
    }
    Ok(cmd)
}

fn generateExtFunctionLibraryDirectoryPaths(
    mut program: &Absyn::Program,
    mut path: &metamodelica::Ref<Absyn::Path>,
    mut inMod: &metamodelica::Ref<SCode::Mod>,
) -> metamodelica::List<ArcStr> {
    let mut outLibs: metamodelica::List<ArcStr>;
    outLibs = 'mc: {
        let __mc_input = &**inMod;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut r#str: ArcStr;
                    let mut platform1: ArcStr;
                    let mut platform2: ArcStr;
                    let mut platform3: ArcStr;
                    let mut libs: metamodelica::List<ArcStr>;
                    let mut isLinux: bool;
                    let __pa0 = ::match_deref::match_deref! { match &(Mod::getUnelabedSubMod(inMod, &(literal!("LibraryDirectory")))?) {
                        Deref @ SCode::Mod::MOD { binding: Some(Deref @ Absyn::Exp::STRING { value: __pa0 }), .. } => __pa0.clone(),
                        _ => return Err("pattern mismatch"),
                    } };
                    r#str = metamodelica::Own::own(__pa0);
                    r#str = ProgramUtil::getFullPathFromUri(program, r#str.clone(), false)?;
                    platform1 = System::openModelicaPlatform();
                    platform2 = System::openModelicaPlatformAlternative();
                    platform3 = System::modelicaPlatform();
                    isLinux = stringEq(&(literal!("linux")), &arcstr::literal!(Autoconf::os));
                    libs = generateExtFunctionLibraryDirectoryPaths2(true, r#str.clone(), isLinux, metamodelica::nil());
                    libs = generateExtFunctionLibraryDirectoryPaths2(!(stringEq(&platform3, &(literal!("")))), { let mut __mm_s = String::new(); __mm_s.push_str(&*r#str); __mm_s.push_str(&*literal!("/")); __mm_s.push_str(&*platform3); ArcStr::from(__mm_s) }, isLinux, libs.clone());
                    libs = generateExtFunctionLibraryDirectoryPaths2(!(stringEq(&platform2, &(literal!("")))), { let mut __mm_s = String::new(); __mm_s.push_str(&*r#str); __mm_s.push_str(&*literal!("/")); __mm_s.push_str(&*platform2); ArcStr::from(__mm_s) }, isLinux, libs.clone());
                    libs = generateExtFunctionLibraryDirectoryPaths2(!(stringEq(&platform1, &(literal!("")))), { let mut __mm_s = String::new(); __mm_s.push_str(&*r#str); __mm_s.push_str(&*literal!("/")); __mm_s.push_str(&*platform1); ArcStr::from(__mm_s) }, isLinux, libs.clone());
                    libs = generateExtFunctionLibraryDirectoryPaths2(isWasmSimCodeTarget()?, { let mut __mm_s = String::new(); __mm_s.push_str(&*r#str); __mm_s.push_str(&*literal!("/wasm32-wasip1")); ArcStr::from(__mm_s) }, isLinux, libs.clone());
                    Ok(libs.clone())
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
                    let mut platform1: ArcStr;
                    let mut platform2: ArcStr;
                    let mut platform3: ArcStr;
                    let mut libs: metamodelica::List<ArcStr>;
                    let mut isLinux: bool;
                    r#str = { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("modelica://")); __mm_s.push_str(&*AbsynUtil::pathFirstIdent(path)); __mm_s.push_str(&*literal!("/Resources/Library")); ArcStr::from(__mm_s) };
                    r#str = ProgramUtil::getFullPathFromUri(program, r#str.clone(), false)?;
                    platform1 = System::openModelicaPlatform();
                    platform2 = System::openModelicaPlatformAlternative();
                    platform3 = System::modelicaPlatform();
                    isLinux = stringEq(&(literal!("linux")), &arcstr::literal!(Autoconf::os));
                    libs = generateExtFunctionLibraryDirectoryPaths2(true, r#str.clone(), isLinux, metamodelica::nil());
                    libs = generateExtFunctionLibraryDirectoryPaths2(!(stringEq(&platform3, &(literal!("")))), { let mut __mm_s = String::new(); __mm_s.push_str(&*r#str); __mm_s.push_str(&*literal!("/")); __mm_s.push_str(&*platform3); ArcStr::from(__mm_s) }, isLinux, libs.clone());
                    libs = generateExtFunctionLibraryDirectoryPaths2(!(stringEq(&platform2, &(literal!("")))), { let mut __mm_s = String::new(); __mm_s.push_str(&*r#str); __mm_s.push_str(&*literal!("/")); __mm_s.push_str(&*platform2); ArcStr::from(__mm_s) }, isLinux, libs.clone());
                    libs = generateExtFunctionLibraryDirectoryPaths2(!(stringEq(&platform1, &(literal!("")))), { let mut __mm_s = String::new(); __mm_s.push_str(&*r#str); __mm_s.push_str(&*literal!("/")); __mm_s.push_str(&*platform1); ArcStr::from(__mm_s) }, isLinux, libs.clone());
                    libs = generateExtFunctionLibraryDirectoryPaths2(isWasmSimCodeTarget()?, { let mut __mm_s = String::new(); __mm_s.push_str(&*r#str); __mm_s.push_str(&*literal!("/wasm32-wasip1")); ArcStr::from(__mm_s) }, isLinux, libs.clone());
                    Ok(libs.clone())
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
    outLibs
}

fn generateExtFunctionLibraryDirectoryPaths2(
    mut add: bool,
    mut dir: ArcStr,
    mut isLinux: bool,
    mut inLibs: metamodelica::List<ArcStr>,
) -> metamodelica::List<ArcStr> {
    let mut libs: metamodelica::List<ArcStr>;
    libs = (::match_deref::match_deref! { match &((add, inLibs.clone())) {
        (true, __esc_libs) => {
            libs = (*__esc_libs).clone();
            let mut b: bool;
            b = System::directoryExists(dir.clone());
            libs = List::consOnTrue(b, dir, libs.clone());
            libs.clone()
        },
        _ => {
            inLibs
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    libs
}

fn getLibraryStringInMSVCFormat(
    mut exp: &metamodelica::Ref<Absyn::Exp>,
) -> Result<(metamodelica::List<ArcStr>, metamodelica::List<ArcStr>)> {
    let mut strs: metamodelica::List<ArcStr> = metamodelica::nil();
    let mut names: metamodelica::List<ArcStr>;
    (strs, names) = 'mc: {
        let __mc_input = &**exp;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Absyn::Exp::STRING { value: r#str } => {
                    if !((metamodelica::stringEq(&r#str, &(literal!("Lapack"))) || metamodelica::stringEq(&r#str, &(literal!("lapack"))))) { return Err("guard") }
                    Ok((list![literal!("lapack_win32_MT.lib"), literal!("f2c.lib")], metamodelica::nil()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Absyn::Exp::STRING { value: Deref @ "omcruntime" } => {
                    let mut strs: metamodelica::List<ArcStr> = strs.clone();
                    let true = (metamodelica::stringEq(&(literal!("Windows_NT")), &arcstr::literal!(Autoconf::os))) else { return Err("pattern mismatch") };
                    strs = list![literal!("f2c.lib"), literal!("initialization.lib"), literal!("libexpat.lib"), literal!("math-support.lib"), literal!("meta.lib"), literal!("ModelicaExternalC.lib"), literal!("results.lib"), literal!("simulation.lib"), literal!("solver.lib"), literal!("sundials_kinsol.lib"), literal!("sundials_nvecserial.lib"), literal!("sundials_sunlinsolklu"), literal!("util.lib"), literal!("lapack_win32_MT.lib")];
                    Ok(((strs.clone(), metamodelica::nil()), strs.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            strs = __wb0;
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Absyn::Exp::STRING { value: Deref @ "fmilib" } => {
                    Ok((list![literal!("fmilib.lib"), literal!("shlwapi.lib")], metamodelica::nil()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Absyn::Exp::STRING { value: r#str } => {
                    let true = (System::regularFileExists(r#str.clone()) || metamodelica::stringEq(&(literal!("-")), &(stringGetStringChar(r#str.clone(), 1)?))) else { return Err("pattern mismatch") };
                    Ok((list![r#str.clone()], metamodelica::nil()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Absyn::Exp::STRING { value: r#str } => {
                    let mut r#str = (*r#str).clone();
                    r#str = { let mut __mm_s = String::new(); __mm_s.push_str(&*r#str); __mm_s.push_str(&*literal!(".lib")); ArcStr::from(__mm_s) };
                    Ok((list![r#str.clone()], metamodelica::nil()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Error::addInternalError(literal!("Failed to process Library annotation for external function"), metamodelica::sourceInfo!("SimCode/SimCodeFunctionUtil.mo"))?;
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((strs, names))
}

fn getLibraryStringInGccFormat(
    mut exp: &metamodelica::Ref<Absyn::Exp>,
) -> Result<(metamodelica::List<ArcStr>, metamodelica::List<ArcStr>)> {
    let mut strs: metamodelica::List<ArcStr> = metamodelica::nil();
    let mut names: metamodelica::List<ArcStr> = metamodelica::nil();
    (strs, names) = 'mc: {
        let __mc_input = &**exp;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Absyn::Exp::STRING { value: Deref @ "lapack" } => {
                    Ok((metamodelica::nil(), metamodelica::nil()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Absyn::Exp::STRING { value: Deref @ "Lapack" } => {
                    Ok((metamodelica::nil(), metamodelica::nil()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Absyn::Exp::STRING { value: Deref @ "pthread" } => {
                    if !((metamodelica::stringEq(&arcstr::literal!(Autoconf::os), &(literal!("Windows_NT"))))) { return Err("guard") }
                    Error::addCompilerNotification(literal!("pthreads library is already available. It is not linked from the external library resource directory.\n"))?;
                    Ok((metamodelica::nil(), metamodelica::nil()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Absyn::Exp::STRING { value: Deref @ "rt" } => {
                    if !((metamodelica::stringEq(&arcstr::literal!(Autoconf::os), &(literal!("Windows_NT"))))) { return Err("guard") }
                    Error::addCompilerNotification(literal!("rt library is not needed under Windows. It is not linked from the external library resource directory.\n"))?;
                    Ok((metamodelica::nil(), metamodelica::nil()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Absyn::Exp::STRING { value: Deref @ "Ws2_32" } => {
                    if !((metamodelica::stringEq(&arcstr::literal!(Autoconf::os), &(literal!("Windows_NT"))))) { return Err("guard") }
                    Error::addCompilerNotification(literal!("Ws2_32 library is not needed under Windows. It is not linked from the external library resource directory.\n"))?;
                    Ok((metamodelica::nil(), metamodelica::nil()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Absyn::Exp::STRING { value: Deref @ "User32" } => {
                    if !((metamodelica::stringEq(&arcstr::literal!(Autoconf::os), &(literal!("Windows_NT"))))) { return Err("guard") }
                    Error::addCompilerNotification(literal!("User32 library is already available. It is not linked from the external library resource directory.\n"))?;
                    Ok((metamodelica::nil(), metamodelica::nil()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Absyn::Exp::STRING { value: r#str @ Deref @ "Winmm" } => {
                    if !((metamodelica::stringEq(&arcstr::literal!(Autoconf::os), &(literal!("Windows_NT"))))) { return Err("guard") }
                    let mut r#str = (*r#str).clone();
                    r#str = { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("-l")); __mm_s.push_str(&*r#str); ArcStr::from(__mm_s) };
                    Error::addCompilerNotification(literal!("Winmm library is a windows system library. It is not linked from the external library resource directory.\n"))?;
                    Ok((list![r#str.clone()], metamodelica::nil()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Absyn::Exp::STRING { value: Deref @ "X11" } => {
                    if !((metamodelica::stringEq(&arcstr::literal!(Autoconf::os), &(literal!("Windows_NT"))))) { return Err("guard") }
                    Error::addCompilerNotification(literal!("X11 library is not needed under Windows. It is not linked from the external library resource directory.\n"))?;
                    Ok((metamodelica::nil(), metamodelica::nil()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Absyn::Exp::STRING { value: Deref @ "ModelicaMatIO" } => {
                    if !((!metamodelica::stringEq(&arcstr::literal!(Autoconf::hdf5Libs), &(literal!(""))))) { return Err("guard") }
                    Ok((list![literal!("-lModelicaMatIO"), arcstr::literal!(Autoconf::hdf5Libs)], metamodelica::nil()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Absyn::Exp::STRING { value: r#str @ Deref @ "omcruntime" } => {
                    let mut r#str = (*r#str).clone();
                    let mut strs: metamodelica::List<ArcStr> = strs.clone();
                    if metamodelica::stringEq(&(literal!("Windows_NT")), &arcstr::literal!(Autoconf::os)) {
                        r#str = { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("-l")); __mm_s.push_str(&*r#str); ArcStr::from(__mm_s) };
                        strs = metamodelica::cons(r#str.clone(), metamodelica::cons(literal!("-lintl"), metamodelica::cons(literal!("-liconv"), metamodelica::cons(literal!("-lexpat"), metamodelica::cons(literal!("-lsqlite3"), metamodelica::cons(literal!("-ltre"), metamodelica::cons(literal!("-lws2_32"), metamodelica::cons(literal!("-lRpcrt4"), metamodelica::cons(literal!("-lregex"), metamodelica::nil())))))))));
                    } else {
                        strs = Autoconf::systemLibs.clone();
                    }
                    Ok(((strs.clone(), metamodelica::nil()), strs.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            strs = __wb0;
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Absyn::Exp::STRING { value: Deref @ "fmilib" } => {
                    Ok((if (metamodelica::stringEq(&arcstr::literal!(Autoconf::os), &(literal!("Windows_NT")))) {list![literal!("-lfmilib"), literal!("-lshlwapi")]} else if (metamodelica::stringEq(&arcstr::literal!(Autoconf::fmilibLibs), &(literal!("")))) {list![literal!("-lfmilib")]} else {metamodelica::cons(literal!("-lfmilib"), Util::stringSplitAtChar(arcstr::literal!(Autoconf::fmilibLibs), literal!(" "))?)}, metamodelica::nil()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok((__v, __wb0, __wb1)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Absyn::Exp::STRING { value: r#str } => {
                    let mut strs1: metamodelica::List<ArcStr>;
                    let mut strs2: metamodelica::List<ArcStr>;
                    let mut strs3: metamodelica::List<ArcStr>;
                    let mut names1: metamodelica::List<ArcStr>;
                    let mut names2: metamodelica::List<ArcStr>;
                    let mut names3: metamodelica::List<ArcStr>;
                    let mut names: metamodelica::List<ArcStr> = names.clone();
                    let mut strs: metamodelica::List<ArcStr> = strs.clone();
                    if metamodelica::stringEq(&r#str, &(literal!("ModelicaStandardTables"))) {
                        (strs1, names1) = getLibraryStringInGccFormat(&(metamodelica::Ref::new(Absyn::Exp::STRING { value: literal!("ModelicaIO") })))?;
                        (strs2, names2) = getLibraryStringInGccFormat(&(metamodelica::Ref::new(Absyn::Exp::STRING { value: literal!("ModelicaMatIO") })))?;
                        (strs3, names3) = getLibraryStringInGccFormat(&(metamodelica::Ref::new(Absyn::Exp::STRING { value: literal!("zlib") })))?;
                        strs = listAppend(strs1.clone(), listAppend(strs2.clone(), strs3.clone()));
                        names = listAppend(names1.clone(), listAppend(names2.clone(), names3.clone()));
                    } else {
                        strs = metamodelica::nil();
                        names = metamodelica::nil();
                    }
                    if System::regularFileExists(r#str.clone()) || metamodelica::stringEq(&(literal!("-")), &(stringGetStringChar(r#str.clone(), 1)?)) {
                        strs = metamodelica::cons(r#str.clone(), strs.clone());
                    } else {
                        strs = metamodelica::cons({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("-l")); __mm_s.push_str(&*r#str); ArcStr::from(__mm_s) }, strs.clone());
                        names = metamodelica::cons(r#str.clone(), names.clone());
                    }
                    Ok(((strs.clone(), names.clone()), names.clone(), strs.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            names = __wb0;
            strs = __wb1;
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Error::addInternalError(literal!("Failed to process Library annotation for external function"), metamodelica::sourceInfo!("SimCode/SimCodeFunctionUtil.mo"))?;
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((strs, names))
}

fn isWasmSimCodeTarget() -> Result<bool> {
    let mut isWasm: bool = StringUtil::startsWith(Config::simCodeTarget()?, literal!("wasm"))
        && !(Flags::getConfigBool(Flags::BUILDING_FMU.clone())? && Testsuite::isRunning()?);
    Ok(isWasm)
}

fn stripLibraryExtension(mut r#str: ArcStr) -> Result<ArcStr> {
    let mut base: ArcStr = r#str.clone();
    for mut ext in &*list![
        literal!(".wasm"),
        literal!(".dylib"),
        literal!(".obj"),
        literal!(".dll"),
        literal!(".lib"),
        literal!(".so"),
        literal!(".a"),
        literal!(".o")
    ] {
        if StringUtil::endsWith(r#str.clone(), ext.clone()) {
            base = Util::removeLastNChar(r#str, ((ext).len() as i32))?;
            return Ok(base);
        }
    }
    Ok(base)
}

fn getLibraryStringInWasmFormat(
    mut exp: &metamodelica::Ref<Absyn::Exp>,
) -> Result<(metamodelica::List<ArcStr>, metamodelica::List<ArcStr>)> {
    let mut strs: metamodelica::List<ArcStr>;
    let mut names: metamodelica::List<ArcStr>;
    (strs, names) = (::match_deref::match_deref! { match exp {
        Deref @ Absyn::Exp::STRING { value: Deref @ "lapack" } => {
            (metamodelica::nil(), metamodelica::nil())
        },
        Deref @ Absyn::Exp::STRING { value: Deref @ "Lapack" } => {
            (metamodelica::nil(), metamodelica::nil())
        },
        Deref @ Absyn::Exp::STRING { value: Deref @ "blas" } => {
            (metamodelica::nil(), metamodelica::nil())
        },
        Deref @ Absyn::Exp::STRING { value: Deref @ "ModelicaExternalC" } => {
            (metamodelica::nil(), metamodelica::nil())
        },
        Deref @ Absyn::Exp::STRING { value: Deref @ "ModelicaStandardTables" } => {
            (metamodelica::nil(), metamodelica::nil())
        },
        Deref @ Absyn::Exp::STRING { value: Deref @ "ModelicaIO" } => {
            (metamodelica::nil(), metamodelica::nil())
        },
        Deref @ Absyn::Exp::STRING { value: Deref @ "ModelicaMatIO" } => {
            (metamodelica::nil(), metamodelica::nil())
        },
        Deref @ Absyn::Exp::STRING { value: Deref @ "zlib" } => {
            (metamodelica::nil(), metamodelica::nil())
        },
        Deref @ Absyn::Exp::STRING { value: r#str } => {
            let mut host: metamodelica::List<ArcStr>;
            if metamodelica::stringEq(&(literal!("-")), &(stringGetStringChar(r#str.clone(), 1)?)) {
                strs = metamodelica::nil();
                names = metamodelica::nil();
            } else {
                (host, names) = getLibraryStringInGccFormat(exp)?;
                strs = metamodelica::cons({ let mut __mm_s = String::new(); __mm_s.push_str(&*stripLibraryExtension(r#str.clone())?); __mm_s.push_str(&*literal!(".wasm")); ArcStr::from(__mm_s) }, host);
            }
            (strs, names)
        },
        _ => {
            Error::addInternalError(literal!("Failed to process Library annotation for external function"), metamodelica::sourceInfo!("SimCode/SimCodeFunctionUtil.mo"))?;
            return Err("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((strs, names))
}

fn generateExtFunctionIncludesLibstr(
    mut target: ArcStr,
    mut inMod: &metamodelica::Ref<SCode::Mod>,
) -> Result<(metamodelica::List<ArcStr>, metamodelica::List<ArcStr>)> {
    let mut outStringLst: metamodelica::List<ArcStr>;
    let mut names: metamodelica::List<ArcStr> = metamodelica::nil();
    (outStringLst, names) = 'mc: {
        let __mc_input = if (isWasmSimCodeTarget()?) {
            literal!("wasm")
        } else {
            getGerneralTarget(target)?
        };
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ "wasm" => {
                    let mut arr: metamodelica::List<metamodelica::Ref<Absyn::Exp>>;
                    let mut libsList: metamodelica::List<metamodelica::List<ArcStr>>;
                    let mut namesList: metamodelica::List<metamodelica::List<ArcStr>>;
                    let __pa0 = ::match_deref::match_deref! { match &(Mod::getUnelabedSubMod(inMod, &(literal!("Library")))?) {
                        Deref @ SCode::Mod::MOD { binding: Some(Deref @ Absyn::Exp::ARRAY { arrayExp: __pa0 }), .. } => __pa0.clone(),
                        _ => return Err("pattern mismatch"),
                    } };
                    arr = metamodelica::Own::own(__pa0);
                    (libsList, namesList) = List::map_2(&arr, &move |__a0: metamodelica::Ref<Absyn::Exp>| getLibraryStringInWasmFormat(&__a0))?;
                    Ok((List::flatten(libsList.clone())?, List::flatten(namesList.clone())?))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ "wasm" => {
                    let mut libs: metamodelica::List<ArcStr>;
                    let mut exp: metamodelica::Ref<Absyn::Exp>;
                    let mut names: metamodelica::List<ArcStr> = names.clone();
                    let __pa0 = ::match_deref::match_deref! { match &(Mod::getUnelabedSubMod(inMod, &(literal!("Library")))?) {
                        Deref @ SCode::Mod::MOD { binding: Some(__pa0), .. } => __pa0.clone(),
                        _ => return Err("pattern mismatch"),
                    } };
                    exp = metamodelica::Own::own(__pa0);
                    (libs, names) = getLibraryStringInWasmFormat(&exp)?;
                    Ok(((libs.clone(), names.clone()), names.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            names = __wb0;
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ "msvc" => {
                    let mut arr: metamodelica::List<metamodelica::Ref<Absyn::Exp>>;
                    let mut libsList: metamodelica::List<metamodelica::List<ArcStr>>;
                    let mut namesList: metamodelica::List<metamodelica::List<ArcStr>>;
                    let __pa0 = ::match_deref::match_deref! { match &(Mod::getUnelabedSubMod(inMod, &(literal!("Library")))?) {
                        Deref @ SCode::Mod::MOD { binding: Some(Deref @ Absyn::Exp::ARRAY { arrayExp: __pa0 }), .. } => __pa0.clone(),
                        _ => return Err("pattern mismatch"),
                    } };
                    arr = metamodelica::Own::own(__pa0);
                    (libsList, namesList) = List::map_2(&arr, &move |__a0: metamodelica::Ref<Absyn::Exp>| getLibraryStringInMSVCFormat(&__a0))?;
                    Ok((List::flatten(libsList.clone())?, List::flatten(namesList.clone())?))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ "msvc" => {
                    let mut libs: metamodelica::List<ArcStr>;
                    let mut exp: metamodelica::Ref<Absyn::Exp>;
                    let mut names: metamodelica::List<ArcStr> = names.clone();
                    let __pa0 = ::match_deref::match_deref! { match &(Mod::getUnelabedSubMod(inMod, &(literal!("Library")))?) {
                        Deref @ SCode::Mod::MOD { binding: Some(__pa0), .. } => __pa0.clone(),
                        _ => return Err("pattern mismatch"),
                    } };
                    exp = metamodelica::Own::own(__pa0);
                    (libs, names) = getLibraryStringInMSVCFormat(&exp)?;
                    Ok(((libs.clone(), names.clone()), names.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            names = __wb0;
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut arr: metamodelica::List<metamodelica::Ref<Absyn::Exp>>;
                    let mut libsList: metamodelica::List<metamodelica::List<ArcStr>>;
                    let mut namesList: metamodelica::List<metamodelica::List<ArcStr>>;
                    let __pa0 = ::match_deref::match_deref! { match &(Mod::getUnelabedSubMod(inMod, &(literal!("Library")))?) {
                        Deref @ SCode::Mod::MOD { binding: Some(Deref @ Absyn::Exp::ARRAY { arrayExp: __pa0 }), .. } => __pa0.clone(),
                        _ => return Err("pattern mismatch"),
                    } };
                    arr = metamodelica::Own::own(__pa0);
                    (libsList, namesList) = List::map_2(&arr, &move |__a0: metamodelica::Ref<Absyn::Exp>| getLibraryStringInGccFormat(&__a0))?;
                    Ok((List::flatten(libsList.clone())?, List::flatten(namesList.clone())?))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut libs: metamodelica::List<ArcStr>;
                    let mut exp: metamodelica::Ref<Absyn::Exp>;
                    let mut names: metamodelica::List<ArcStr> = names.clone();
                    let __pa0 = ::match_deref::match_deref! { match &(Mod::getUnelabedSubMod(inMod, &(literal!("Library")))?) {
                        Deref @ SCode::Mod::MOD { binding: Some(__pa0), .. } => __pa0.clone(),
                        _ => return Err("pattern mismatch"),
                    } };
                    exp = metamodelica::Own::own(__pa0);
                    (libs, names) = getLibraryStringInGccFormat(&exp)?;
                    Ok(((libs.clone(), names.clone()), names.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            names = __wb0;
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((metamodelica::nil(), metamodelica::nil()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((outStringLst, names))
}

fn generateExtFunctionIncludesIncludestr(mut inMod: &metamodelica::Ref<SCode::Mod>) -> metamodelica::List<ArcStr> {
    let mut includes: metamodelica::List<ArcStr> = metamodelica::nil();
    includes = 'mc: {
        let __mc_input = &**inMod;
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut inc: ArcStr;
                    let mut inc_1: ArcStr;
                    let mut lineNumberStart: i32;
                    let mut r#str: ArcStr;
                    let mut fileName: ArcStr;
                    let mut includes: metamodelica::List<ArcStr> = includes.clone();
                    let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(Mod::getUnelabedSubMod(inMod, &(literal!("Include")))?) {
                        Deref @ SCode::Mod::MOD { binding: Some(Deref @ Absyn::Exp::STRING { value: __pa0 }), info: SourceInfo { fileName: __pa1, lineNumberStart: __pa2, .. }, .. } => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    inc = metamodelica::Own::own(__pa0);
                    fileName = metamodelica::Own::own(__pa1);
                    lineNumberStart = metamodelica::Own::own(__pa2);
                    r#str = { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("#line ")); __mm_s.push_str(&*intString(lineNumberStart)); __mm_s.push_str(&*literal!(" \"")); __mm_s.push_str(&*fileName); __mm_s.push_str(&*literal!("\"")); ArcStr::from(__mm_s) };
                    inc_1 = System::unescapedString(inc.clone());
                    includes = if (Flags::isSet(Flags::GEN_DEBUG_SYMBOLS.clone())?) {list![r#str.clone(), inc_1.clone()]} else {list![inc_1.clone()]};
                    Ok((includes.clone(), includes.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            includes = __wb0;
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
    includes
}

fn generateExtFunctionDynamicLoad(mut inMod: &metamodelica::Ref<SCode::Mod>) -> bool {
    let mut outDynamicLoad: bool;
    outDynamicLoad = 'mc: {
        let __mc_input = &**inMod;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut b: bool;
                    let __pa0 = ::match_deref::match_deref! { match &(Mod::getUnelabedSubMod(inMod, &(literal!("DynamicLoad")))?) {
                        Deref @ SCode::Mod::MOD { binding: Some(Deref @ Absyn::Exp::BOOL { value: __pa0 }), .. } => __pa0.clone(),
                        _ => return Err("pattern mismatch"),
                    } };
                    b = metamodelica::Own::own(__pa0);
                    Ok(b)
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
    outDynamicLoad
}

pub(crate) fn getImplicitRecordConstructors(
    mut inExpLst: &metamodelica::List<metamodelica::Ref<DAE::Exp>>,
) -> metamodelica::List<metamodelica::Ref<DAE::Exp>> {
    let mut outExpLst: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
    outExpLst = 'mc: {
        let __mc_input = &**inExpLst;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Nil => {
                    Ok(metamodelica::nil())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Exp::CREF { componentRef: cref, ty: Deref @ DAE::Type::T_COMPLEX { complexClassType: ClassInf::State::RECORD { path: record_path }, .. } }, tail: rest_expr } => {
                    let mut record_cref: metamodelica::Ref<DAE::Exp>;
                    let mut cref = (*cref).clone();
                    let mut rest_expr = (*rest_expr).clone();
                    ::match_deref::match_deref! { match &(ComponentReference::crefLastSubs(metamodelica::AsArg::as_arg(&cref))?) {
                        Deref @ metamodelica::ListNode::Nil => (),
                        _ => return Err("pattern mismatch"),
                    } };
                    cref = ComponentReference::pathToCref(metamodelica::AsArg::as_arg(&record_path));
                    record_cref = Expression::crefExp(cref.clone())?;
                    rest_expr = getImplicitRecordConstructors(metamodelica::AsArg::as_arg(&rest_expr));
                    Ok(metamodelica::cons(record_cref.clone(), rest_expr.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: _, tail: rest_expr } => {
                    let mut rest_expr = (*rest_expr).clone();
                    rest_expr = getImplicitRecordConstructors(metamodelica::AsArg::as_arg(&rest_expr));
                    Ok(rest_expr.clone())
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

fn getCalledFunctionsInFunctions<'__b>(
    mut paths: metamodelica::List<metamodelica::Ref<Absyn::Path>>,
    mut inHt: (
        metamodelica::Array<metamodelica::List<(ArcStr, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<Option<(ArcStr, metamodelica::Ref<Absyn::Path>)>>,
        ),
        i32,
        (
            Arc<dyn ::std::ops::Fn(ArcStr) -> Result<i32> + 'static>,
            Arc<dyn ::std::ops::Fn(ArcStr, ArcStr) -> Result<bool> + 'static>,
            Arc<dyn ::std::ops::Fn(ArcStr) -> Result<ArcStr> + 'static>,
            Arc<dyn ::std::ops::Fn(metamodelica::Ref<Absyn::Path>) -> Result<ArcStr> + 'static>,
        ),
    ),
    mut funcs: &'__b metamodelica::Ref<AvlTreePathFunction::Tree>,
) -> Result<(
    metamodelica::Array<metamodelica::List<(ArcStr, i32)>>,
    (
        i32,
        i32,
        metamodelica::Array<Option<(ArcStr, metamodelica::Ref<Absyn::Path>)>>,
    ),
    i32,
    (
        Arc<dyn ::std::ops::Fn(ArcStr) -> Result<i32> + 'static>,
        Arc<dyn ::std::ops::Fn(ArcStr, ArcStr) -> Result<bool> + 'static>,
        Arc<dyn ::std::ops::Fn(ArcStr) -> Result<ArcStr> + 'static>,
        Arc<dyn ::std::ops::Fn(metamodelica::Ref<Absyn::Path>) -> Result<ArcStr> + 'static>,
    ),
)> {
    '__tco: loop {
        ::match_deref::match_deref! { match &((paths, inHt)) {
            (Deref @ metamodelica::ListNode::Nil, ht) => {
                return Ok(ht.clone())
            },
            (Deref @ metamodelica::ListNode::Cons { head: path, tail: rest }, ht) => {
                let mut ht = (*ht).clone();
                ht = getCalledFunctionsInFunction2(path.clone(), AbsynUtil::pathStringNoQual(path.clone(), literal!("."), false, false)?, ht.clone(), funcs)?;
                { (paths, inHt, funcs) = (rest.clone(), ht.clone(), funcs); continue '__tco; }
            },
            _ => return Err("match: no arm matched"),
        } }
    }
}

pub(crate) fn getCalledFunctionsInFunction2(
    mut inPath: metamodelica::Ref<Absyn::Path>,
    mut pathstr: ArcStr,
    mut inHt: (
        metamodelica::Array<metamodelica::List<(ArcStr, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<Option<(ArcStr, metamodelica::Ref<Absyn::Path>)>>,
        ),
        i32,
        (
            Arc<dyn ::std::ops::Fn(ArcStr) -> Result<i32> + 'static>,
            Arc<dyn ::std::ops::Fn(ArcStr, ArcStr) -> Result<bool> + 'static>,
            Arc<dyn ::std::ops::Fn(ArcStr) -> Result<ArcStr> + 'static>,
            Arc<dyn ::std::ops::Fn(metamodelica::Ref<Absyn::Path>) -> Result<ArcStr> + 'static>,
        ),
    ),
    mut funcs: &metamodelica::Ref<AvlTreePathFunction::Tree>,
) -> Result<(
    metamodelica::Array<metamodelica::List<(ArcStr, i32)>>,
    (
        i32,
        i32,
        metamodelica::Array<Option<(ArcStr, metamodelica::Ref<Absyn::Path>)>>,
    ),
    i32,
    (
        Arc<dyn ::std::ops::Fn(ArcStr) -> Result<i32> + 'static>,
        Arc<dyn ::std::ops::Fn(ArcStr, ArcStr) -> Result<bool> + 'static>,
        Arc<dyn ::std::ops::Fn(ArcStr) -> Result<ArcStr> + 'static>,
        Arc<dyn ::std::ops::Fn(metamodelica::Ref<Absyn::Path>) -> Result<ArcStr> + 'static>,
    ),
)> {
    let mut outHt: (
        metamodelica::Array<metamodelica::List<(ArcStr, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<Option<(ArcStr, metamodelica::Ref<Absyn::Path>)>>,
        ),
        i32,
        (
            HashTableStringToPath::FuncHashCref,
            HashTableStringToPath::FuncCrefEqual,
            HashTableStringToPath::FuncCrefStr,
            HashTableStringToPath::FuncExpStr,
        ),
    );
    outHt = 'mc: {
        let __mc_input = (inPath, inHt);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, ht) => {
                    if !((BaseHashTable::hasKey(pathstr.clone(), &(ht.clone()))?)) { return Err("guard") }
                    Ok(ht.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (path, ht) => {
                    let mut funcelem: DAE::Function;
                    let mut calledfuncs: metamodelica::List<metamodelica::Ref<Absyn::Path>>;
                    let mut varfuncs: metamodelica::List<metamodelica::Ref<Absyn::Path>>;
                    let mut els: metamodelica::List<metamodelica::Ref<DAE::Element>>;
                    let mut ht = (*ht).clone();
                    funcelem = DAEUtil::getNamedFunction(path.clone(), funcs)?;
                    els = DAEUtil::getFunctionElements(&funcelem)?;
                    varfuncs = List::fold(&els, &move |__a0: metamodelica::Ref<DAE::Element>, __a1: metamodelica::List<metamodelica::Ref<Absyn::Path>>| -> metamodelica::Result<_> { ::std::result::Result::Ok(DAEUtil::collectFunctionRefVarPaths(&__a0, __a1)) }, metamodelica::nil())?;
                    let (_, (_, __pa0)) = DAEUtil::traverseDAEElementList(els.clone(), (std::sync::Arc::new(Expression::traverseSubexpressionsHelper) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>, _) -> Result<_> + 'static>), ((std::sync::Arc::new(DAEUtil::collectValueblockFunctionRefVars) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>, metamodelica::List<metamodelica::Ref<Absyn::Path>>) -> Result<(metamodelica::Ref<DAE::Exp>, metamodelica::List<metamodelica::Ref<Absyn::Path>>)> + 'static>), varfuncs.clone()))?;
                    varfuncs = metamodelica::Own::own(__pa0);
                    let (_, (_, (__pa1, _))) = DAEUtil::traverseDAEElementList(els.clone(), (std::sync::Arc::new(Expression::traverseSubexpressionsHelper) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>, _) -> Result<_> + 'static>), ((std::sync::Arc::new(fnptr!(matchNonBuiltinCallsAndFnRefPaths, metamodelica::Ref<DAE::Exp>, (metamodelica::List<metamodelica::Ref<Absyn::Path>>, metamodelica::List<metamodelica::Ref<Absyn::Path>>))) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>, (metamodelica::List<metamodelica::Ref<Absyn::Path>>, metamodelica::List<metamodelica::Ref<Absyn::Path>>)) -> Result<(metamodelica::Ref<DAE::Exp>, (metamodelica::List<metamodelica::Ref<Absyn::Path>>, metamodelica::List<metamodelica::Ref<Absyn::Path>>))> + 'static>), (metamodelica::nil(), varfuncs.clone())))?;
                    calledfuncs = metamodelica::Own::own(__pa1);
                    ht = BaseHashTable::add((pathstr.clone(), path.clone()), ht.clone())?;
                    ht = addDestructor(&funcelem, ht.clone())?;
                    ht = getCalledFunctionsInFunctions(calledfuncs.clone(), ht.clone(), funcs)?;
                    Ok(ht.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (path, _) => {
                    let mut r#str: ArcStr;
                    if '__try0: {
                        unwrap_break_err!(DAEUtil::getNamedFunction(path.clone(), funcs), '__try0);
                        Ok::<(), &'static str>(())
                    }.is_ok() { return Err("failure(): body succeeded") }
                    r#str = { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("function getCalledFunctionsInFunction2: Class ")); __mm_s.push_str(&*pathstr); __mm_s.push_str(&*literal!(" not found in global scope.")); ArcStr::from(__mm_s) };
                    Error::addInternalError(r#str.clone(), metamodelica::sourceInfo!("SimCode/SimCodeFunctionUtil.mo"))?;
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(outHt)
}

pub fn getCalledFunctionsInFunction(
    mut path: metamodelica::Ref<Absyn::Path>,
    mut funcs: &metamodelica::Ref<AvlTreePathFunction::Tree>,
) -> Result<metamodelica::List<metamodelica::Ref<Absyn::Path>>> {
    let mut outPaths: metamodelica::List<metamodelica::Ref<Absyn::Path>>;
    let mut ht: (
        metamodelica::Array<metamodelica::List<(ArcStr, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<Option<(ArcStr, metamodelica::Ref<Absyn::Path>)>>,
        ),
        i32,
        (
            HashTableStringToPath::FuncHashCref,
            HashTableStringToPath::FuncCrefEqual,
            HashTableStringToPath::FuncCrefStr,
            HashTableStringToPath::FuncExpStr,
        ),
    );
    ht = HashTableStringToPath::emptyHashTable();
    ht = getCalledFunctionsInFunction2(
        path.clone(),
        AbsynUtil::pathStringNoQual(path, literal!("."), false, false)?,
        ht,
        funcs,
    )?;
    outPaths = BaseHashTable::hashTableValueList(&ht)?;
    Ok(outPaths)
}

fn addDestructor(
    mut func: &DAE::Function,
    mut inHt: (
        metamodelica::Array<metamodelica::List<(ArcStr, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<Option<(ArcStr, metamodelica::Ref<Absyn::Path>)>>,
        ),
        i32,
        (
            Arc<dyn ::std::ops::Fn(ArcStr) -> Result<i32> + 'static>,
            Arc<dyn ::std::ops::Fn(ArcStr, ArcStr) -> Result<bool> + 'static>,
            Arc<dyn ::std::ops::Fn(ArcStr) -> Result<ArcStr> + 'static>,
            Arc<dyn ::std::ops::Fn(metamodelica::Ref<Absyn::Path>) -> Result<ArcStr> + 'static>,
        ),
    ),
) -> Result<(
    metamodelica::Array<metamodelica::List<(ArcStr, i32)>>,
    (
        i32,
        i32,
        metamodelica::Array<Option<(ArcStr, metamodelica::Ref<Absyn::Path>)>>,
    ),
    i32,
    (
        Arc<dyn ::std::ops::Fn(ArcStr) -> Result<i32> + 'static>,
        Arc<dyn ::std::ops::Fn(ArcStr, ArcStr) -> Result<bool> + 'static>,
        Arc<dyn ::std::ops::Fn(ArcStr) -> Result<ArcStr> + 'static>,
        Arc<dyn ::std::ops::Fn(metamodelica::Ref<Absyn::Path>) -> Result<ArcStr> + 'static>,
    ),
)> {
    let mut outHt: (
        metamodelica::Array<metamodelica::List<(ArcStr, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<Option<(ArcStr, metamodelica::Ref<Absyn::Path>)>>,
        ),
        i32,
        (
            HashTableStringToPath::FuncHashCref,
            HashTableStringToPath::FuncCrefEqual,
            HashTableStringToPath::FuncCrefStr,
            HashTableStringToPath::FuncExpStr,
        ),
    );
    outHt = (::match_deref::match_deref! { match &(func) {
        DAE::Function::FUNCTION { type_: Deref @ DAE::Type::T_FUNCTION { funcResultType: Deref @ DAE::Type::T_COMPLEX { complexClassType: ClassInf::State::EXTERNAL_OBJ { path }, .. }, .. }, .. } => {
            let mut path = (*path).clone();
            path = AbsynUtil::joinPaths(path.clone(), metamodelica::Ref::new(Absyn::Path::IDENT { name: literal!("destructor") }))?;
            addDestructor2(path.clone(), AbsynUtil::pathStringNoQual(path.clone(), literal!("."), false, false)?, inHt)?
        },
        _ => {
            inHt
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outHt)
}

fn addDestructor2(
    mut path: metamodelica::Ref<Absyn::Path>,
    mut pathstr: ArcStr,
    mut inHt: (
        metamodelica::Array<metamodelica::List<(ArcStr, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<Option<(ArcStr, metamodelica::Ref<Absyn::Path>)>>,
        ),
        i32,
        (
            Arc<dyn ::std::ops::Fn(ArcStr) -> Result<i32> + 'static>,
            Arc<dyn ::std::ops::Fn(ArcStr, ArcStr) -> Result<bool> + 'static>,
            Arc<dyn ::std::ops::Fn(ArcStr) -> Result<ArcStr> + 'static>,
            Arc<dyn ::std::ops::Fn(metamodelica::Ref<Absyn::Path>) -> Result<ArcStr> + 'static>,
        ),
    ),
) -> Result<(
    metamodelica::Array<metamodelica::List<(ArcStr, i32)>>,
    (
        i32,
        i32,
        metamodelica::Array<Option<(ArcStr, metamodelica::Ref<Absyn::Path>)>>,
    ),
    i32,
    (
        Arc<dyn ::std::ops::Fn(ArcStr) -> Result<i32> + 'static>,
        Arc<dyn ::std::ops::Fn(ArcStr, ArcStr) -> Result<bool> + 'static>,
        Arc<dyn ::std::ops::Fn(ArcStr) -> Result<ArcStr> + 'static>,
        Arc<dyn ::std::ops::Fn(metamodelica::Ref<Absyn::Path>) -> Result<ArcStr> + 'static>,
    ),
)> {
    let mut ht: (
        metamodelica::Array<metamodelica::List<(ArcStr, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<Option<(ArcStr, metamodelica::Ref<Absyn::Path>)>>,
        ),
        i32,
        (
            HashTableStringToPath::FuncHashCref,
            HashTableStringToPath::FuncCrefEqual,
            HashTableStringToPath::FuncCrefStr,
            HashTableStringToPath::FuncExpStr,
        ),
    ) = inHt;
    if !(BaseHashTable::hasKey(pathstr.clone(), &ht)?) {
        ht = BaseHashTable::add((pathstr, path), ht)?;
    }
    Ok(ht)
}

fn matchNonBuiltinCallsAndFnRefPaths(
    mut inExp: metamodelica::Ref<DAE::Exp>,
    mut itpl: (
        metamodelica::List<metamodelica::Ref<Absyn::Path>>,
        metamodelica::List<metamodelica::Ref<Absyn::Path>>,
    ),
) -> (
    metamodelica::Ref<DAE::Exp>,
    (
        metamodelica::List<metamodelica::Ref<Absyn::Path>>,
        metamodelica::List<metamodelica::Ref<Absyn::Path>>,
    ),
) {
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    let mut otpl: (
        metamodelica::List<metamodelica::Ref<Absyn::Path>>,
        metamodelica::List<metamodelica::Ref<Absyn::Path>>,
    );
    (outExp, otpl) = 'mc: {
        let __mc_input = (&*inExp, &itpl);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CALL { path, attr: Deref @ DAE::CallAttributes { builtin: false, .. }, .. }, (acc, filter)) => {
                    let mut path = (*path).clone();
                    path = AbsynUtil::makeNotFullyQualified(path.clone());
                    let false = (List::isMemberOnTrue(path.clone(), metamodelica::AsArg::as_arg(&filter), &move |__a0: metamodelica::Ref<Absyn::Path>, __a1: metamodelica::Ref<Absyn::Path>| -> metamodelica::Result<_> { ::std::result::Result::Ok(AbsynUtil::pathEqual(&__a0, &__a1)) })?) else { return Err("pattern mismatch") };
                    Ok((inExp.clone(), (metamodelica::cons(path.clone(), acc.clone()), filter.clone())))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::REDUCTION { reductionInfo: Deref @ DAE::ReductionInfo { path, .. }, .. }, (acc, filter)) => {
                    let false = (List::isMemberOnTrue(path.clone(), &(list![metamodelica::Ref::new(Absyn::Path::IDENT { name: literal!("list") }), metamodelica::Ref::new(Absyn::Path::IDENT { name: literal!("listReverse") }), metamodelica::Ref::new(Absyn::Path::IDENT { name: literal!("array") }), metamodelica::Ref::new(Absyn::Path::IDENT { name: literal!("min") }), metamodelica::Ref::new(Absyn::Path::IDENT { name: literal!("max") }), metamodelica::Ref::new(Absyn::Path::IDENT { name: literal!("sum") }), metamodelica::Ref::new(Absyn::Path::IDENT { name: literal!("product") })]), &move |__a0: metamodelica::Ref<Absyn::Path>, __a1: metamodelica::Ref<Absyn::Path>| -> metamodelica::Result<_> { ::std::result::Result::Ok(AbsynUtil::pathEqual(&__a0, &__a1)) })?) else { return Err("pattern mismatch") };
                    let false = (List::isMemberOnTrue(path.clone(), metamodelica::AsArg::as_arg(&filter), &move |__a0: metamodelica::Ref<Absyn::Path>, __a1: metamodelica::Ref<Absyn::Path>| -> metamodelica::Result<_> { ::std::result::Result::Ok(AbsynUtil::pathEqual(&__a0, &__a1)) })?) else { return Err("pattern mismatch") };
                    Ok((inExp.clone(), (metamodelica::cons(path.clone(), acc.clone()), filter.clone())))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::PARTEVALFUNCTION { path, .. }, (acc, filter)) => {
                    let mut path = (*path).clone();
                    path = AbsynUtil::makeNotFullyQualified(path.clone());
                    let false = (List::isMemberOnTrue(path.clone(), metamodelica::AsArg::as_arg(&filter), &move |__a0: metamodelica::Ref<Absyn::Path>, __a1: metamodelica::Ref<Absyn::Path>| -> metamodelica::Result<_> { ::std::result::Result::Ok(AbsynUtil::pathEqual(&__a0, &__a1)) })?) else { return Err("pattern mismatch") };
                    Ok((inExp.clone(), (metamodelica::cons(path.clone(), acc.clone()), filter.clone())))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CREF { ty: Deref @ DAE::Type::T_FUNCTION_REFERENCE_FUNC { builtin: false, .. }, .. }, (acc, filter)) => {
                    let mut path: metamodelica::Ref<Absyn::Path>;
                    path = AbsynUtil::crefToPath(&(getCrefFromExp(&inExp)?))?;
                    let false = (List::isMemberOnTrue(path.clone(), metamodelica::AsArg::as_arg(&filter), &move |__a0: metamodelica::Ref<Absyn::Path>, __a1: metamodelica::Ref<Absyn::Path>| -> metamodelica::Result<_> { ::std::result::Result::Ok(AbsynUtil::pathEqual(&__a0, &__a1)) })?) else { return Err("pattern mismatch") };
                    Ok((inExp.clone(), (metamodelica::cons(path.clone(), acc.clone()), filter.clone())))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((inExp.clone(), itpl.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    (outExp, otpl)
}

fn aliasRecordDeclarations(
    mut inDecl: SimCodeFunction::RecordDeclaration,
    mut inHt: (
        metamodelica::Array<metamodelica::List<(ArcStr, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<Option<(ArcStr, metamodelica::Ref<Absyn::Path>)>>,
        ),
        i32,
        (
            Arc<dyn ::std::ops::Fn(ArcStr) -> Result<i32> + 'static>,
            Arc<dyn ::std::ops::Fn(ArcStr, ArcStr) -> Result<bool> + 'static>,
            Arc<dyn ::std::ops::Fn(ArcStr) -> Result<ArcStr> + 'static>,
            Arc<dyn ::std::ops::Fn(metamodelica::Ref<Absyn::Path>) -> Result<ArcStr> + 'static>,
        ),
    ),
) -> Result<(
    SimCodeFunction::RecordDeclaration,
    (
        metamodelica::Array<metamodelica::List<(ArcStr, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<Option<(ArcStr, metamodelica::Ref<Absyn::Path>)>>,
        ),
        i32,
        (
            Arc<dyn ::std::ops::Fn(ArcStr) -> Result<i32> + 'static>,
            Arc<dyn ::std::ops::Fn(ArcStr, ArcStr) -> Result<bool> + 'static>,
            Arc<dyn ::std::ops::Fn(ArcStr) -> Result<ArcStr> + 'static>,
            Arc<dyn ::std::ops::Fn(metamodelica::Ref<Absyn::Path>) -> Result<ArcStr> + 'static>,
        ),
    ),
)> {
    let mut decl: SimCodeFunction::RecordDeclaration;
    let mut ht: (
        metamodelica::Array<metamodelica::List<(ArcStr, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<Option<(ArcStr, metamodelica::Ref<Absyn::Path>)>>,
        ),
        i32,
        (
            HashTableStringToPath::FuncHashCref,
            HashTableStringToPath::FuncCrefEqual,
            HashTableStringToPath::FuncCrefStr,
            HashTableStringToPath::FuncExpStr,
        ),
    );
    (decl, ht) = (match inDecl.clone() {
        SimCodeFunction::RecordDeclaration::RECORD_DECL_FULL {
            name: mut sname,
            aliasName: _,
            defPath: ref name,
            variables: ref vars,
            usedExternally: mut extConvert,
        } => {
            let mut r#str: ArcStr;
            let mut alias: Option<ArcStr>;
            r#str = stringDelimitList(
                List::map(vars.clone(), &move |__a0: metamodelica::Ref<
                    SimCodeFunction::Variable::Variable,
                >| variableString(&__a0))?,
                literal!("\n"),
            );
            (alias, ht) = aliasRecordDeclarations2(r#str, name.clone(), inHt)?;
            (
                SimCodeFunction::RecordDeclaration::RECORD_DECL_FULL {
                    name: sname.clone(),
                    aliasName: alias,
                    defPath: name.clone(),
                    variables: vars.clone(),
                    usedExternally: extConvert.clone(),
                },
                ht,
            )
        }
        _ => (inDecl, inHt),
    });
    Ok((decl, ht))
}

fn aliasRecordDeclarations2(
    mut r#str: ArcStr,
    mut path: metamodelica::Ref<Absyn::Path>,
    mut inHt: (
        metamodelica::Array<metamodelica::List<(ArcStr, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<Option<(ArcStr, metamodelica::Ref<Absyn::Path>)>>,
        ),
        i32,
        (
            Arc<dyn ::std::ops::Fn(ArcStr) -> Result<i32> + 'static>,
            Arc<dyn ::std::ops::Fn(ArcStr, ArcStr) -> Result<bool> + 'static>,
            Arc<dyn ::std::ops::Fn(ArcStr) -> Result<ArcStr> + 'static>,
            Arc<dyn ::std::ops::Fn(metamodelica::Ref<Absyn::Path>) -> Result<ArcStr> + 'static>,
        ),
    ),
) -> Result<(
    Option<ArcStr>,
    (
        metamodelica::Array<metamodelica::List<(ArcStr, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<Option<(ArcStr, metamodelica::Ref<Absyn::Path>)>>,
        ),
        i32,
        (
            Arc<dyn ::std::ops::Fn(ArcStr) -> Result<i32> + 'static>,
            Arc<dyn ::std::ops::Fn(ArcStr, ArcStr) -> Result<bool> + 'static>,
            Arc<dyn ::std::ops::Fn(ArcStr) -> Result<ArcStr> + 'static>,
            Arc<dyn ::std::ops::Fn(metamodelica::Ref<Absyn::Path>) -> Result<ArcStr> + 'static>,
        ),
    ),
)> {
    let mut alias: Option<ArcStr>;
    let mut ht: (
        metamodelica::Array<metamodelica::List<(ArcStr, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<Option<(ArcStr, metamodelica::Ref<Absyn::Path>)>>,
        ),
        i32,
        (
            HashTableStringToPath::FuncHashCref,
            HashTableStringToPath::FuncCrefEqual,
            HashTableStringToPath::FuncCrefStr,
            HashTableStringToPath::FuncExpStr,
        ),
    );
    (alias, ht) = 'mc: {
        let __mc_input = inHt.clone();
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            let mut aliasStr: ArcStr;
            aliasStr =
                AbsynUtil::pathStringUnquoteReplaceDot(&(BaseHashTable::get(r#str.clone(), &inHt)?), literal!("_"))?;
            Ok((Some(aliasStr.clone()), inHt.clone()))
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            let mut ht: (
                metamodelica::Array<metamodelica::List<(ArcStr, i32)>>,
                (
                    i32,
                    i32,
                    metamodelica::Array<Option<(ArcStr, metamodelica::Ref<Absyn::Path>)>>,
                ),
                i32,
                (
                    HashTableStringToPath::FuncHashCref,
                    HashTableStringToPath::FuncCrefEqual,
                    HashTableStringToPath::FuncCrefStr,
                    HashTableStringToPath::FuncExpStr,
                ),
            );
            ht = BaseHashTable::add((r#str.clone(), path.clone()), inHt.clone())?;
            Ok((None, ht.clone()))
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((alias, ht))
}

fn variableString(mut var: &metamodelica::Ref<SimCodeFunction::Variable::Variable>) -> Result<ArcStr> {
    let mut r#str: ArcStr;
    r#str = (match &**var {
        SimCodeFunction::Variable::VARIABLE { name, ty, .. } => {
            let mut __mm_s = String::new();
            __mm_s.push_str(&*TypesDump::unparseType(Types::arrayElementType(ty))?);
            __mm_s.push_str(&*literal!("["));
            __mm_s.push_str(&*intString(Types::numberOfDimensions(ty)));
            __mm_s.push_str(&*literal!("] "));
            __mm_s.push_str(&*ComponentReferenceBasics::printComponentRefStr(name)?);
            ArcStr::from(__mm_s)
        }
        SimCodeFunction::Variable::FUNCTION_PTR { name: __esc_str, .. } => {
            r#str = (*__esc_str).clone();
            {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("modelica_fnptr "));
                __mm_s.push_str(&*r#str);
                ArcStr::from(__mm_s)
            }
        }
    });
    Ok(r#str)
}

pub fn createMakefileParams(
    mut includes: metamodelica::List<ArcStr>,
    mut libs: metamodelica::List<ArcStr>,
    mut libPaths: metamodelica::List<ArcStr>,
    mut isFunction: bool,
    mut isFMU: bool,
) -> Result<SimCodeFunction::MakefileParams> {
    let mut makefileParams: SimCodeFunction::MakefileParams;
    let mut omhome: ArcStr;
    let mut ccompiler: ArcStr;
    let mut cxxcompiler: ArcStr;
    let mut linker: ArcStr;
    let mut exeext: ArcStr;
    let mut dllext: ArcStr;
    let mut cflags: ArcStr;
    let mut ldflags: ArcStr;
    let mut rtlibs: ArcStr;
    let mut platform: ArcStr;
    let mut compileDir: ArcStr;
    ccompiler = if (stringEq(&(Config::simCodeTarget()?), &(literal!("JavaScript")))) {
        literal!("emcc")
    } else {
        if (Flags::isSet(Flags::HPCOM.clone())?) {
            System::getOMPCCompiler()
        } else {
            System::getCCompiler()
        }
    };
    cxxcompiler = if (stringEq(&(Config::simCodeTarget()?), &(literal!("JavaScript")))) {
        literal!("emcc")
    } else {
        System::getCXXCompiler()
    };
    linker = if (stringEq(&(Config::simCodeTarget()?), &(literal!("JavaScript")))) {
        literal!("emcc")
    } else {
        System::getLinker()
    };
    exeext = if (stringEq(&(Config::simCodeTarget()?), &(literal!("JavaScript")))) {
        literal!(".js")
    } else {
        arcstr::literal!(Autoconf::exeExt)
    };
    dllext = arcstr::literal!(Autoconf::dllExt);
    omhome = Settings::getInstallationDirectoryPath()?;
    omhome = System::trim(omhome, literal!("\""));
    cflags = {
        let mut __mm_s = String::new();
        __mm_s.push_str(&*System::getCFlags());
        __mm_s.push_str(&*literal!(" "));
        __mm_s.push_str(&*if (Flags::isSet(Flags::HPCOM.clone())?) {
            literal!("-fopenmp")
        } else {
            literal!("")
        });
        ArcStr::from(__mm_s)
    };
    cflags = if (stringEq(&(Config::simCodeTarget()?), &(literal!("JavaScript")))) {
        literal!("-Os -Wno-warn-absolute-paths")
    } else {
        cflags
    };
    ldflags = System::getLDFlags();
    if Flags::getConfigBool(Flags::PARMODAUTO.clone())? && !(Config::simCodeRustRuntime()?) {
        ldflags = {
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!(" "));
            __mm_s.push_str(&*arcstr::literal!(Autoconf::parModelicaAutoLibs));
            __mm_s.push_str(&*literal!(" "));
            __mm_s.push_str(&*ldflags);
            ArcStr::from(__mm_s)
        };
    }
    rtlibs = if (isFunction) {
        if (Config::acceptMetaModelicaGrammar()?) {
            arcstr::literal!(Autoconf::ldflags_runtime_mmc)
        } else {
            arcstr::literal!(Autoconf::ldflags_runtime)
        }
    } else if (isFMU) {
        arcstr::literal!(Autoconf::ldflags_runtime_fmu)
    } else if (Config::simCodeRustRuntime()?) {
        arcstr::literal!(Autoconf::ldflags_runtime_sim_rust)
    } else {
        arcstr::literal!(Autoconf::ldflags_runtime_sim)
    };
    platform = System::modelicaPlatform();
    compileDir = {
        let mut __mm_s = String::new();
        __mm_s.push_str(&*System::pwd());
        __mm_s.push_str(&*arcstr::literal!(Autoconf::pathDelimiter));
        ArcStr::from(__mm_s)
    };
    makefileParams = SimCodeFunction::MakefileParams {
        ccompiler: ccompiler,
        cxxcompiler: cxxcompiler,
        linker: linker,
        exeext: exeext,
        dllext: dllext,
        omhome: omhome,
        cflags: cflags,
        ldflags: ldflags,
        runtimelibs: rtlibs,
        includes: includes,
        libs: libs,
        libPaths: libPaths,
        platform: platform,
        compileDir: compileDir,
    };
    Ok(makefileParams)
}

pub fn codegenResetTryThrowIndex() -> () {
    {
        let __v = metamodelica::nil();
        openmodelica_util::Globals::codegenTryThrowIndex.with(|__root| *__root.borrow_mut() = __v)
    };
    ()
}

pub fn codegenPushTryThrowIndex(mut i: i32) -> () {
    let mut lst: metamodelica::List<i32>;
    lst = openmodelica_util::Globals::codegenTryThrowIndex.with(|__root| __root.borrow().clone());
    {
        let __v = metamodelica::cons(i, lst);
        openmodelica_util::Globals::codegenTryThrowIndex.with(|__root| *__root.borrow_mut() = __v)
    };
    ()
}

pub fn codegenPopTryThrowIndex() -> Result<()> {
    let mut lst: metamodelica::List<i32>;
    lst = openmodelica_util::Globals::codegenTryThrowIndex.with(|__root| __root.borrow().clone());
    let __pa0 = ::match_deref::match_deref! { match &(lst) {
        Deref @ metamodelica::ListNode::Cons { head: _, tail: __pa0 } => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    lst = metamodelica::Own::own(__pa0);
    {
        let __v = lst;
        openmodelica_util::Globals::codegenTryThrowIndex.with(|__root| *__root.borrow_mut() = __v)
    };
    Ok(())
}

pub fn codegenPeekTryThrowIndex() -> i32 {
    let mut i: i32;
    let mut lst: metamodelica::List<i32>;
    lst = openmodelica_util::Globals::codegenTryThrowIndex.with(|__root| __root.borrow().clone());
    i = (::match_deref::match_deref! { match &(lst) {
        Deref @ metamodelica::ListNode::Cons { head: __esc_i, tail: _ } => {
            i = (*__esc_i).clone();
            i.clone()
        },
        _ => -1,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    i
}

pub(crate) fn varIndex(mut var: &metamodelica::Ref<SimCodeVar::SimVar>) -> i32 {
    let mut index: i32;
    let __arc1 = &(*var);
    let SimCodeVar::SIMVAR { index: __pa0, .. } = &**__arc1;
    index = metamodelica::Own::own(__pa0);
    index
}

pub fn isParallelFunctionContext(mut context: &SimCodeFunction::Context) -> bool {
    let mut outBool: bool;
    outBool = (match context.clone() {
        SimCodeFunction::Context::FUNCTION_CONTEXT { .. } => {
            var_field!(context.is_parallel, SimCodeFunction::Context::FUNCTION_CONTEXT).clone()
        }
        _ => false,
    });
    outBool
}

pub fn twodigit(mut i: i32) -> ArcStr {
    let mut outS: ArcStr;
    outS = (match i {
        _ if (i < 10) => {
            let mut s: ArcStr;
            s = intString(i);
            s = stringAppend(literal!("0"), s);
            s
        }
        _ => intString(i),
    });
    outS
}

pub(crate) fn generateSubPalceholders(mut cr: &metamodelica::Ref<DAE::ComponentRef>) -> Result<ArcStr> {
    let mut outdef: ArcStr;
    let mut dims: metamodelica::List<metamodelica::Ref<DAE::Dimension>>;
    let mut nrdims: i32;
    let mut idxstrlst: metamodelica::List<ArcStr>;
    dims = ComponentReferenceBasics::crefDims(cr)?;
    nrdims = ((dims).len() as i32);
    idxstrlst = List::map(List::intRange(nrdims), &fnptr!(intString, i32))?;
    outdef = stringDelimitList(
        List::threadMap(
            List::fill(literal!("i_"), nrdims),
            idxstrlst,
            &fnptr!(stringAppend, ArcStr, ArcStr),
        )?,
        literal!(","),
    );
    Ok(outdef)
}

/* This functions are used to get/append cref prefixes in function contexts.The cref prefix is appended
to all crefs generated. We use this to generate dependent names in some cases (for example when generating
code for record constructors) so that every cref we generate while this is set has this prefix applied to it*/
pub fn getCurrentCrefPrefix(mut context: &SimCodeFunction::Context) -> Result<ArcStr> {
    let mut cref_pref: ArcStr;
    cref_pref = (match context.clone() {
        SimCodeFunction::Context::FUNCTION_CONTEXT {
            cref_prefix: mut __esc_cref_pref,
            is_parallel: _,
        } => {
            cref_pref = __esc_cref_pref.clone();
            cref_pref
        }
        _ => {
            Error::addInternalError(
                literal!(
                    "Tried to get cref prefix from a non FUNCTION_CONTEXT() context. cref_pref is only avaiable in FUNCTION_CONTEXT."
                ),
                metamodelica::sourceInfo!("SimCode/SimCodeFunctionUtil.mo"),
            )?;
            return Err("fail");
        }
    });
    Ok(cref_pref)
}

pub fn appendCurrentCrefPrefix(
    mut context: &SimCodeFunction::Context,
    mut in_cref_pref: &ArcStr,
) -> Result<SimCodeFunction::Context> {
    let mut out_context: SimCodeFunction::Context;
    let mut cref_pref: ArcStr;
    let mut prl: bool;
    out_context = (match context.clone() {
        SimCodeFunction::Context::FUNCTION_CONTEXT {
            cref_prefix: mut __esc_cref_pref,
            is_parallel: mut __esc_prl,
        } => {
            cref_pref = __esc_cref_pref.clone();
            prl = __esc_prl.clone();
            SimCodeFunction::Context::FUNCTION_CONTEXT {
                cref_prefix: {
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*cref_pref);
                    __mm_s.push_str(&*in_cref_pref);
                    ArcStr::from(__mm_s)
                },
                is_parallel: prl,
            }
        }
        _ => {
            Error::addInternalError(
                literal!(
                    "Tried to append cref prefix from a non FUNCTION_CONTEXT() context. cref_pref is only avaiable in FUNCTION_CONTEXT."
                ),
                metamodelica::sourceInfo!("SimCode/SimCodeFunctionUtil.mo"),
            )?;
            return Err("fail");
        }
    });
    Ok(out_context)
}
