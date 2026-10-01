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

use crate::NFCeval as Ceval;
use crate::NFEvalFunction as EvalFunction;
use crate::NFEvalFunction::assignVariable;
use crate::NFExpression as Expression;
use crate::NFType as Type;
use openmodelica_util::Lapack;

pub(crate) fn Lapack_dgeev(mut args: &metamodelica::List<metamodelica::Ref<Expression::NFExpression>>) -> Result<()> {
    let mut jobvl: metamodelica::Ref<Expression::NFExpression>;
    let mut jobvr: metamodelica::Ref<Expression::NFExpression>;
    let mut n: metamodelica::Ref<Expression::NFExpression>;
    let mut a: metamodelica::Ref<Expression::NFExpression>;
    let mut lda: metamodelica::Ref<Expression::NFExpression>;
    let mut ldvl: metamodelica::Ref<Expression::NFExpression>;
    let mut ldvr: metamodelica::Ref<Expression::NFExpression>;
    let mut work: metamodelica::Ref<Expression::NFExpression>;
    let mut lwork: metamodelica::Ref<Expression::NFExpression>;
    let mut wr: metamodelica::Ref<Expression::NFExpression>;
    let mut wi: metamodelica::Ref<Expression::NFExpression>;
    let mut vl: metamodelica::Ref<Expression::NFExpression>;
    let mut vr: metamodelica::Ref<Expression::NFExpression>;
    let mut info: metamodelica::Ref<Expression::NFExpression>;
    let mut INFO: i32;
    let mut LDA: i32;
    let mut LDVL: i32;
    let mut LDVR: i32;
    let mut LWORK: i32;
    let mut N: i32;
    let mut JOBVL: ArcStr;
    let mut JOBVR: ArcStr;
    let mut A: metamodelica::List<metamodelica::List<metamodelica::Real>>;
    let mut VL: metamodelica::List<metamodelica::List<metamodelica::Real>>;
    let mut VR: metamodelica::List<metamodelica::List<metamodelica::Real>>;
    let mut WORK: metamodelica::List<metamodelica::Real>;
    let mut WR: metamodelica::List<metamodelica::Real>;
    let mut WI: metamodelica::List<metamodelica::Real>;
    let (__pa0, __pa1, __pa2, __pa3, __pa4, __pa5, __pa6, __pa7, __pa8, __pa9, __pa10, __pa11, __pa12, __pa13) = ::match_deref::match_deref! { match &((*args)) {
        Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: Deref @ metamodelica::ListNode::Cons { head: __pa1, tail: Deref @ metamodelica::ListNode::Cons { head: __pa2, tail: Deref @ metamodelica::ListNode::Cons { head: __pa3, tail: Deref @ metamodelica::ListNode::Cons { head: __pa4, tail: Deref @ metamodelica::ListNode::Cons { head: __pa5, tail: Deref @ metamodelica::ListNode::Cons { head: __pa6, tail: Deref @ metamodelica::ListNode::Cons { head: __pa7, tail: Deref @ metamodelica::ListNode::Cons { head: __pa8, tail: Deref @ metamodelica::ListNode::Cons { head: __pa9, tail: Deref @ metamodelica::ListNode::Cons { head: __pa10, tail: Deref @ metamodelica::ListNode::Cons { head: __pa11, tail: Deref @ metamodelica::ListNode::Cons { head: __pa12, tail: Deref @ metamodelica::ListNode::Cons { head: __pa13, tail: Deref @ metamodelica::ListNode::Nil } } } } } } } } } } } } } } => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone(), __pa4.clone(), __pa5.clone(), __pa6.clone(), __pa7.clone(), __pa8.clone(), __pa9.clone(), __pa10.clone(), __pa11.clone(), __pa12.clone(), __pa13.clone()),
        _ => return Err("pattern mismatch"),
    } };
    jobvl = metamodelica::Own::own(__pa0);
    jobvr = metamodelica::Own::own(__pa1);
    n = metamodelica::Own::own(__pa2);
    a = metamodelica::Own::own(__pa3);
    lda = metamodelica::Own::own(__pa4);
    wr = metamodelica::Own::own(__pa5);
    wi = metamodelica::Own::own(__pa6);
    vl = metamodelica::Own::own(__pa7);
    ldvl = metamodelica::Own::own(__pa8);
    vr = metamodelica::Own::own(__pa9);
    ldvr = metamodelica::Own::own(__pa10);
    work = metamodelica::Own::own(__pa11);
    lwork = metamodelica::Own::own(__pa12);
    info = metamodelica::Own::own(__pa13);
    JOBVL = evaluateExtStringArg(jobvl)?;
    JOBVR = evaluateExtStringArg(jobvr)?;
    N = evaluateExtIntArg(n)?;
    A = evaluateExtRealMatrixArg(a.clone())?;
    LDA = evaluateExtIntArg(lda)?;
    LDVL = evaluateExtIntArg(ldvl)?;
    LDVR = evaluateExtIntArg(ldvr)?;
    WORK = evaluateExtRealArrayArg(work.clone())?;
    LWORK = evaluateExtIntArg(lwork)?;
    (A, WR, WI, VL, VR, WORK, INFO) = Lapack::dgeev(JOBVL, JOBVR, N, A, LDA, LDVL, LDVR, WORK, LWORK);
    assignVariableExt(a, Expression::makeRealMatrix(A)?)?;
    assignVariable(&wr, &(Expression::makeRealArray(&WR)?))?;
    assignVariable(&wi, &(Expression::makeRealArray(&WI)?))?;
    assignVariableExt(vl, Expression::makeRealMatrix(VL)?)?;
    assignVariableExt(vr, Expression::makeRealMatrix(VR)?)?;
    assignVariable(&work, &(Expression::makeRealArray(&WORK)?))?;
    assignVariable(&info, &(Expression::makeInteger(INFO)))?;
    Ok(())
}

pub(crate) fn Lapack_dgegv(mut args: &metamodelica::List<metamodelica::Ref<Expression::NFExpression>>) -> Result<()> {
    let mut jobvl: metamodelica::Ref<Expression::NFExpression>;
    let mut jobvr: metamodelica::Ref<Expression::NFExpression>;
    let mut n: metamodelica::Ref<Expression::NFExpression>;
    let mut a: metamodelica::Ref<Expression::NFExpression>;
    let mut lda: metamodelica::Ref<Expression::NFExpression>;
    let mut b: metamodelica::Ref<Expression::NFExpression>;
    let mut ldb: metamodelica::Ref<Expression::NFExpression>;
    let mut alphar: metamodelica::Ref<Expression::NFExpression>;
    let mut alphai: metamodelica::Ref<Expression::NFExpression>;
    let mut beta: metamodelica::Ref<Expression::NFExpression>;
    let mut vl: metamodelica::Ref<Expression::NFExpression>;
    let mut ldvl: metamodelica::Ref<Expression::NFExpression>;
    let mut vr: metamodelica::Ref<Expression::NFExpression>;
    let mut ldvr: metamodelica::Ref<Expression::NFExpression>;
    let mut work: metamodelica::Ref<Expression::NFExpression>;
    let mut lwork: metamodelica::Ref<Expression::NFExpression>;
    let mut info: metamodelica::Ref<Expression::NFExpression>;
    let mut JOBVL: ArcStr;
    let mut JOBVR: ArcStr;
    let mut N: i32;
    let mut LDA: i32;
    let mut LDB: i32;
    let mut LDVL: i32;
    let mut LDVR: i32;
    let mut LWORK: i32;
    let mut INFO: i32;
    let mut A: metamodelica::List<metamodelica::List<metamodelica::Real>>;
    let mut B: metamodelica::List<metamodelica::List<metamodelica::Real>>;
    let mut VL: metamodelica::List<metamodelica::List<metamodelica::Real>>;
    let mut VR: metamodelica::List<metamodelica::List<metamodelica::Real>>;
    let mut WORK: metamodelica::List<metamodelica::Real>;
    let mut ALPHAR: metamodelica::List<metamodelica::Real>;
    let mut ALPHAI: metamodelica::List<metamodelica::Real>;
    let mut BETA: metamodelica::List<metamodelica::Real>;
    let (
        __pa0,
        __pa1,
        __pa2,
        __pa3,
        __pa4,
        __pa5,
        __pa6,
        __pa7,
        __pa8,
        __pa9,
        __pa10,
        __pa11,
        __pa12,
        __pa13,
        __pa14,
        __pa15,
        __pa16,
    ) = ::match_deref::match_deref! { match &((*args)) {
        Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: Deref @ metamodelica::ListNode::Cons { head: __pa1, tail: Deref @ metamodelica::ListNode::Cons { head: __pa2, tail: Deref @ metamodelica::ListNode::Cons { head: __pa3, tail: Deref @ metamodelica::ListNode::Cons { head: __pa4, tail: Deref @ metamodelica::ListNode::Cons { head: __pa5, tail: Deref @ metamodelica::ListNode::Cons { head: __pa6, tail: Deref @ metamodelica::ListNode::Cons { head: __pa7, tail: Deref @ metamodelica::ListNode::Cons { head: __pa8, tail: Deref @ metamodelica::ListNode::Cons { head: __pa9, tail: Deref @ metamodelica::ListNode::Cons { head: __pa10, tail: Deref @ metamodelica::ListNode::Cons { head: __pa11, tail: Deref @ metamodelica::ListNode::Cons { head: __pa12, tail: Deref @ metamodelica::ListNode::Cons { head: __pa13, tail: Deref @ metamodelica::ListNode::Cons { head: __pa14, tail: Deref @ metamodelica::ListNode::Cons { head: __pa15, tail: Deref @ metamodelica::ListNode::Cons { head: __pa16, tail: Deref @ metamodelica::ListNode::Nil } } } } } } } } } } } } } } } } } => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone(), __pa4.clone(), __pa5.clone(), __pa6.clone(), __pa7.clone(), __pa8.clone(), __pa9.clone(), __pa10.clone(), __pa11.clone(), __pa12.clone(), __pa13.clone(), __pa14.clone(), __pa15.clone(), __pa16.clone()),
        _ => return Err("pattern mismatch"),
    } };
    jobvl = metamodelica::Own::own(__pa0);
    jobvr = metamodelica::Own::own(__pa1);
    n = metamodelica::Own::own(__pa2);
    a = metamodelica::Own::own(__pa3);
    lda = metamodelica::Own::own(__pa4);
    b = metamodelica::Own::own(__pa5);
    ldb = metamodelica::Own::own(__pa6);
    alphar = metamodelica::Own::own(__pa7);
    alphai = metamodelica::Own::own(__pa8);
    beta = metamodelica::Own::own(__pa9);
    vl = metamodelica::Own::own(__pa10);
    ldvl = metamodelica::Own::own(__pa11);
    vr = metamodelica::Own::own(__pa12);
    ldvr = metamodelica::Own::own(__pa13);
    work = metamodelica::Own::own(__pa14);
    lwork = metamodelica::Own::own(__pa15);
    info = metamodelica::Own::own(__pa16);
    JOBVL = evaluateExtStringArg(jobvl)?;
    JOBVR = evaluateExtStringArg(jobvr)?;
    N = evaluateExtIntArg(n)?;
    A = evaluateExtRealMatrixArg(a)?;
    LDA = evaluateExtIntArg(lda)?;
    B = evaluateExtRealMatrixArg(b)?;
    LDB = evaluateExtIntArg(ldb)?;
    LDVL = evaluateExtIntArg(ldvl)?;
    LDVR = evaluateExtIntArg(ldvr)?;
    WORK = evaluateExtRealArrayArg(work.clone())?;
    LWORK = evaluateExtIntArg(lwork)?;
    (ALPHAR, ALPHAI, BETA, VL, VR, WORK, INFO) =
        Lapack::dgegv(JOBVL, JOBVR, N, A, LDA, B, LDB, LDVL, LDVR, WORK, LWORK);
    assignVariable(&alphar, &(Expression::makeRealArray(&ALPHAR)?))?;
    assignVariable(&alphai, &(Expression::makeRealArray(&ALPHAI)?))?;
    assignVariable(&beta, &(Expression::makeRealArray(&BETA)?))?;
    assignVariableExt(vl, Expression::makeRealMatrix(VL)?)?;
    assignVariableExt(vr, Expression::makeRealMatrix(VR)?)?;
    assignVariable(&work, &(Expression::makeRealArray(&WORK)?))?;
    assignVariable(&info, &(Expression::makeInteger(INFO)))?;
    Ok(())
}

pub(crate) fn Lapack_dgels(mut args: &metamodelica::List<metamodelica::Ref<Expression::NFExpression>>) -> Result<()> {
    let mut trans: metamodelica::Ref<Expression::NFExpression>;
    let mut m: metamodelica::Ref<Expression::NFExpression>;
    let mut n: metamodelica::Ref<Expression::NFExpression>;
    let mut nrhs: metamodelica::Ref<Expression::NFExpression>;
    let mut a: metamodelica::Ref<Expression::NFExpression>;
    let mut lda: metamodelica::Ref<Expression::NFExpression>;
    let mut b: metamodelica::Ref<Expression::NFExpression>;
    let mut ldb: metamodelica::Ref<Expression::NFExpression>;
    let mut work: metamodelica::Ref<Expression::NFExpression>;
    let mut lwork: metamodelica::Ref<Expression::NFExpression>;
    let mut info: metamodelica::Ref<Expression::NFExpression>;
    let mut TRANS: ArcStr;
    let mut M: i32;
    let mut N: i32;
    let mut NRHS: i32;
    let mut LDA: i32;
    let mut LDB: i32;
    let mut LWORK: i32;
    let mut INFO: i32;
    let mut A: metamodelica::List<metamodelica::List<metamodelica::Real>>;
    let mut B: metamodelica::List<metamodelica::List<metamodelica::Real>>;
    let mut WORK: metamodelica::List<metamodelica::Real>;
    let (__pa0, __pa1, __pa2, __pa3, __pa4, __pa5, __pa6, __pa7, __pa8, __pa9, __pa10) = ::match_deref::match_deref! { match &((*args)) {
        Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: Deref @ metamodelica::ListNode::Cons { head: __pa1, tail: Deref @ metamodelica::ListNode::Cons { head: __pa2, tail: Deref @ metamodelica::ListNode::Cons { head: __pa3, tail: Deref @ metamodelica::ListNode::Cons { head: __pa4, tail: Deref @ metamodelica::ListNode::Cons { head: __pa5, tail: Deref @ metamodelica::ListNode::Cons { head: __pa6, tail: Deref @ metamodelica::ListNode::Cons { head: __pa7, tail: Deref @ metamodelica::ListNode::Cons { head: __pa8, tail: Deref @ metamodelica::ListNode::Cons { head: __pa9, tail: Deref @ metamodelica::ListNode::Cons { head: __pa10, tail: Deref @ metamodelica::ListNode::Nil } } } } } } } } } } } => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone(), __pa4.clone(), __pa5.clone(), __pa6.clone(), __pa7.clone(), __pa8.clone(), __pa9.clone(), __pa10.clone()),
        _ => return Err("pattern mismatch"),
    } };
    trans = metamodelica::Own::own(__pa0);
    m = metamodelica::Own::own(__pa1);
    n = metamodelica::Own::own(__pa2);
    nrhs = metamodelica::Own::own(__pa3);
    a = metamodelica::Own::own(__pa4);
    lda = metamodelica::Own::own(__pa5);
    b = metamodelica::Own::own(__pa6);
    ldb = metamodelica::Own::own(__pa7);
    work = metamodelica::Own::own(__pa8);
    lwork = metamodelica::Own::own(__pa9);
    info = metamodelica::Own::own(__pa10);
    TRANS = evaluateExtStringArg(trans)?;
    M = evaluateExtIntArg(m)?;
    N = evaluateExtIntArg(n)?;
    NRHS = evaluateExtIntArg(nrhs)?;
    A = evaluateExtRealMatrixArg(a.clone())?;
    LDA = evaluateExtIntArg(lda)?;
    B = evaluateExtRealMatrixArg(b.clone())?;
    LDB = evaluateExtIntArg(ldb)?;
    WORK = evaluateExtRealArrayArg(work.clone())?;
    LWORK = evaluateExtIntArg(lwork)?;
    (A, B, WORK, INFO) = Lapack::dgels(TRANS, M, N, NRHS, A, LDA, B, LDB, WORK, LWORK);
    assignVariableExt(a, Expression::makeRealMatrix(A)?)?;
    assignVariableExt(b, Expression::makeRealMatrix(B)?)?;
    assignVariable(&work, &(Expression::makeRealArray(&WORK)?))?;
    assignVariable(&info, &(Expression::makeInteger(INFO)))?;
    Ok(())
}

pub(crate) fn Lapack_dgelsx(mut args: &metamodelica::List<metamodelica::Ref<Expression::NFExpression>>) -> Result<()> {
    let mut m: metamodelica::Ref<Expression::NFExpression>;
    let mut n: metamodelica::Ref<Expression::NFExpression>;
    let mut nrhs: metamodelica::Ref<Expression::NFExpression>;
    let mut a: metamodelica::Ref<Expression::NFExpression>;
    let mut lda: metamodelica::Ref<Expression::NFExpression>;
    let mut b: metamodelica::Ref<Expression::NFExpression>;
    let mut ldb: metamodelica::Ref<Expression::NFExpression>;
    let mut jpvt: metamodelica::Ref<Expression::NFExpression>;
    let mut rcond: metamodelica::Ref<Expression::NFExpression>;
    let mut rank: metamodelica::Ref<Expression::NFExpression>;
    let mut work: metamodelica::Ref<Expression::NFExpression>;
    let mut info: metamodelica::Ref<Expression::NFExpression>;
    let mut M: i32;
    let mut N: i32;
    let mut NRHS: i32;
    let mut LDA: i32;
    let mut LDB: i32;
    let mut RANK: i32;
    let mut INFO: i32;
    let mut A: metamodelica::List<metamodelica::List<metamodelica::Real>>;
    let mut B: metamodelica::List<metamodelica::List<metamodelica::Real>>;
    let mut JPVT: metamodelica::List<i32>;
    let mut RCOND: metamodelica::Real;
    let mut WORK: metamodelica::List<metamodelica::Real>;
    if ((args).len() as i32) == 12 {
        let (__pa0, __pa1, __pa2, __pa3, __pa4, __pa5, __pa6, __pa7, __pa8, __pa9, __pa10, __pa11) = ::match_deref::match_deref! { match &((*args)) {
            Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: Deref @ metamodelica::ListNode::Cons { head: __pa1, tail: Deref @ metamodelica::ListNode::Cons { head: __pa2, tail: Deref @ metamodelica::ListNode::Cons { head: __pa3, tail: Deref @ metamodelica::ListNode::Cons { head: __pa4, tail: Deref @ metamodelica::ListNode::Cons { head: __pa5, tail: Deref @ metamodelica::ListNode::Cons { head: __pa6, tail: Deref @ metamodelica::ListNode::Cons { head: __pa7, tail: Deref @ metamodelica::ListNode::Cons { head: __pa8, tail: Deref @ metamodelica::ListNode::Cons { head: __pa9, tail: Deref @ metamodelica::ListNode::Cons { head: __pa10, tail: Deref @ metamodelica::ListNode::Cons { head: __pa11, tail: Deref @ metamodelica::ListNode::Nil } } } } } } } } } } } } => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone(), __pa4.clone(), __pa5.clone(), __pa6.clone(), __pa7.clone(), __pa8.clone(), __pa9.clone(), __pa10.clone(), __pa11.clone()),
            _ => return Err("pattern mismatch"),
        } };
        m = metamodelica::Own::own(__pa0);
        n = metamodelica::Own::own(__pa1);
        nrhs = metamodelica::Own::own(__pa2);
        a = metamodelica::Own::own(__pa3);
        lda = metamodelica::Own::own(__pa4);
        b = metamodelica::Own::own(__pa5);
        ldb = metamodelica::Own::own(__pa6);
        jpvt = metamodelica::Own::own(__pa7);
        rcond = metamodelica::Own::own(__pa8);
        rank = metamodelica::Own::own(__pa9);
        work = metamodelica::Own::own(__pa10);
        info = metamodelica::Own::own(__pa11);
    } else {
        let (__pa13, __pa14, __pa15, __pa16, __pa17, __pa18, __pa19, __pa20, __pa21, __pa22, __pa23, __pa24) = ::match_deref::match_deref! { match &((*args)) {
            Deref @ metamodelica::ListNode::Cons { head: __pa13, tail: Deref @ metamodelica::ListNode::Cons { head: __pa14, tail: Deref @ metamodelica::ListNode::Cons { head: __pa15, tail: Deref @ metamodelica::ListNode::Cons { head: __pa16, tail: Deref @ metamodelica::ListNode::Cons { head: __pa17, tail: Deref @ metamodelica::ListNode::Cons { head: __pa18, tail: Deref @ metamodelica::ListNode::Cons { head: __pa19, tail: Deref @ metamodelica::ListNode::Cons { head: __pa20, tail: Deref @ metamodelica::ListNode::Cons { head: __pa21, tail: Deref @ metamodelica::ListNode::Cons { head: __pa22, tail: Deref @ metamodelica::ListNode::Cons { head: __pa23, tail: Deref @ metamodelica::ListNode::Cons { head: _, tail: Deref @ metamodelica::ListNode::Cons { head: __pa24, tail: Deref @ metamodelica::ListNode::Nil } } } } } } } } } } } } } => (__pa13.clone(), __pa14.clone(), __pa15.clone(), __pa16.clone(), __pa17.clone(), __pa18.clone(), __pa19.clone(), __pa20.clone(), __pa21.clone(), __pa22.clone(), __pa23.clone(), __pa24.clone()),
            _ => return Err("pattern mismatch"),
        } };
        m = metamodelica::Own::own(__pa13);
        n = metamodelica::Own::own(__pa14);
        nrhs = metamodelica::Own::own(__pa15);
        a = metamodelica::Own::own(__pa16);
        lda = metamodelica::Own::own(__pa17);
        b = metamodelica::Own::own(__pa18);
        ldb = metamodelica::Own::own(__pa19);
        jpvt = metamodelica::Own::own(__pa20);
        rcond = metamodelica::Own::own(__pa21);
        rank = metamodelica::Own::own(__pa22);
        work = metamodelica::Own::own(__pa23);
        info = metamodelica::Own::own(__pa24);
    }
    M = evaluateExtIntArg(m)?;
    N = evaluateExtIntArg(n)?;
    NRHS = evaluateExtIntArg(nrhs)?;
    A = evaluateExtRealMatrixArg(a.clone())?;
    LDA = evaluateExtIntArg(lda)?;
    B = evaluateExtRealMatrixArg(b.clone())?;
    LDB = evaluateExtIntArg(ldb)?;
    JPVT = evaluateExtIntArrayArg(jpvt.clone())?;
    RCOND = evaluateExtRealArg(rcond)?;
    WORK = evaluateExtRealArrayArg(work)?;
    (A, B, JPVT, RANK, INFO) = Lapack::dgelsx(M, N, NRHS, A, LDA, B, LDB, JPVT, RCOND, WORK);
    assignVariableExt(a, Expression::makeRealMatrix(A)?)?;
    assignVariableExt(b, Expression::makeRealMatrix(B)?)?;
    assignVariable(&jpvt, &(Expression::makeIntegerArray(&JPVT)?))?;
    assignVariable(&rank, &(Expression::makeInteger(RANK)))?;
    assignVariable(&info, &(Expression::makeInteger(INFO)))?;
    Ok(())
}

pub(crate) fn Lapack_dgelsy(mut args: &metamodelica::List<metamodelica::Ref<Expression::NFExpression>>) -> Result<()> {
    let mut m: metamodelica::Ref<Expression::NFExpression>;
    let mut n: metamodelica::Ref<Expression::NFExpression>;
    let mut nrhs: metamodelica::Ref<Expression::NFExpression>;
    let mut a: metamodelica::Ref<Expression::NFExpression>;
    let mut lda: metamodelica::Ref<Expression::NFExpression>;
    let mut b: metamodelica::Ref<Expression::NFExpression>;
    let mut ldb: metamodelica::Ref<Expression::NFExpression>;
    let mut jpvt: metamodelica::Ref<Expression::NFExpression>;
    let mut rcond: metamodelica::Ref<Expression::NFExpression>;
    let mut rank: metamodelica::Ref<Expression::NFExpression>;
    let mut work: metamodelica::Ref<Expression::NFExpression>;
    let mut lwork: metamodelica::Ref<Expression::NFExpression>;
    let mut info: metamodelica::Ref<Expression::NFExpression>;
    let mut M: i32;
    let mut N: i32;
    let mut NRHS: i32;
    let mut LDA: i32;
    let mut LDB: i32;
    let mut RANK: i32;
    let mut LWORK: i32;
    let mut INFO: i32;
    let mut A: metamodelica::List<metamodelica::List<metamodelica::Real>>;
    let mut B: metamodelica::List<metamodelica::List<metamodelica::Real>>;
    let mut JPVT: metamodelica::List<i32>;
    let mut RCOND: metamodelica::Real;
    let mut WORK: metamodelica::List<metamodelica::Real>;
    let (__pa0, __pa1, __pa2, __pa3, __pa4, __pa5, __pa6, __pa7, __pa8, __pa9, __pa10, __pa11, __pa12) = ::match_deref::match_deref! { match &((*args)) {
        Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: Deref @ metamodelica::ListNode::Cons { head: __pa1, tail: Deref @ metamodelica::ListNode::Cons { head: __pa2, tail: Deref @ metamodelica::ListNode::Cons { head: __pa3, tail: Deref @ metamodelica::ListNode::Cons { head: __pa4, tail: Deref @ metamodelica::ListNode::Cons { head: __pa5, tail: Deref @ metamodelica::ListNode::Cons { head: __pa6, tail: Deref @ metamodelica::ListNode::Cons { head: __pa7, tail: Deref @ metamodelica::ListNode::Cons { head: __pa8, tail: Deref @ metamodelica::ListNode::Cons { head: __pa9, tail: Deref @ metamodelica::ListNode::Cons { head: __pa10, tail: Deref @ metamodelica::ListNode::Cons { head: __pa11, tail: Deref @ metamodelica::ListNode::Cons { head: __pa12, tail: Deref @ metamodelica::ListNode::Nil } } } } } } } } } } } } } => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone(), __pa4.clone(), __pa5.clone(), __pa6.clone(), __pa7.clone(), __pa8.clone(), __pa9.clone(), __pa10.clone(), __pa11.clone(), __pa12.clone()),
        _ => return Err("pattern mismatch"),
    } };
    m = metamodelica::Own::own(__pa0);
    n = metamodelica::Own::own(__pa1);
    nrhs = metamodelica::Own::own(__pa2);
    a = metamodelica::Own::own(__pa3);
    lda = metamodelica::Own::own(__pa4);
    b = metamodelica::Own::own(__pa5);
    ldb = metamodelica::Own::own(__pa6);
    jpvt = metamodelica::Own::own(__pa7);
    rcond = metamodelica::Own::own(__pa8);
    rank = metamodelica::Own::own(__pa9);
    work = metamodelica::Own::own(__pa10);
    lwork = metamodelica::Own::own(__pa11);
    info = metamodelica::Own::own(__pa12);
    M = evaluateExtIntArg(m)?;
    N = evaluateExtIntArg(n)?;
    NRHS = evaluateExtIntArg(nrhs)?;
    A = evaluateExtRealMatrixArg(a.clone())?;
    LDA = evaluateExtIntArg(lda)?;
    B = evaluateExtRealMatrixArg(b.clone())?;
    LDB = evaluateExtIntArg(ldb)?;
    JPVT = evaluateExtIntArrayArg(jpvt.clone())?;
    RCOND = evaluateExtRealArg(rcond)?;
    WORK = evaluateExtRealArrayArg(work.clone())?;
    LWORK = evaluateExtIntArg(lwork)?;
    (A, B, JPVT, RANK, WORK, INFO) = Lapack::dgelsy(M, N, NRHS, A, LDA, B, LDB, JPVT, RCOND, WORK, LWORK);
    assignVariableExt(a, Expression::makeRealMatrix(A)?)?;
    assignVariableExt(b, Expression::makeRealMatrix(B)?)?;
    assignVariable(&jpvt, &(Expression::makeIntegerArray(&JPVT)?))?;
    assignVariable(&rank, &(Expression::makeInteger(RANK)))?;
    assignVariable(&work, &(Expression::makeRealArray(&WORK)?))?;
    assignVariable(&info, &(Expression::makeInteger(INFO)))?;
    Ok(())
}

pub(crate) fn Lapack_dgesv(mut args: &metamodelica::List<metamodelica::Ref<Expression::NFExpression>>) -> Result<()> {
    let mut n: metamodelica::Ref<Expression::NFExpression>;
    let mut nrhs: metamodelica::Ref<Expression::NFExpression>;
    let mut a: metamodelica::Ref<Expression::NFExpression>;
    let mut lda: metamodelica::Ref<Expression::NFExpression>;
    let mut ipiv: metamodelica::Ref<Expression::NFExpression>;
    let mut b: metamodelica::Ref<Expression::NFExpression>;
    let mut ldb: metamodelica::Ref<Expression::NFExpression>;
    let mut info: metamodelica::Ref<Expression::NFExpression>;
    let mut N: i32;
    let mut NRHS: i32;
    let mut LDA: i32;
    let mut LDB: i32;
    let mut INFO: i32;
    let mut A: metamodelica::List<metamodelica::List<metamodelica::Real>>;
    let mut B: metamodelica::List<metamodelica::List<metamodelica::Real>>;
    let mut IPIV: metamodelica::List<i32>;
    let (__pa0, __pa1, __pa2, __pa3, __pa4, __pa5, __pa6, __pa7) = ::match_deref::match_deref! { match &((*args)) {
        Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: Deref @ metamodelica::ListNode::Cons { head: __pa1, tail: Deref @ metamodelica::ListNode::Cons { head: __pa2, tail: Deref @ metamodelica::ListNode::Cons { head: __pa3, tail: Deref @ metamodelica::ListNode::Cons { head: __pa4, tail: Deref @ metamodelica::ListNode::Cons { head: __pa5, tail: Deref @ metamodelica::ListNode::Cons { head: __pa6, tail: Deref @ metamodelica::ListNode::Cons { head: __pa7, tail: Deref @ metamodelica::ListNode::Nil } } } } } } } } => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone(), __pa4.clone(), __pa5.clone(), __pa6.clone(), __pa7.clone()),
        _ => return Err("pattern mismatch"),
    } };
    n = metamodelica::Own::own(__pa0);
    nrhs = metamodelica::Own::own(__pa1);
    a = metamodelica::Own::own(__pa2);
    lda = metamodelica::Own::own(__pa3);
    ipiv = metamodelica::Own::own(__pa4);
    b = metamodelica::Own::own(__pa5);
    ldb = metamodelica::Own::own(__pa6);
    info = metamodelica::Own::own(__pa7);
    N = evaluateExtIntArg(n)?;
    NRHS = evaluateExtIntArg(nrhs)?;
    A = evaluateExtRealMatrixArg(a.clone())?;
    LDA = evaluateExtIntArg(lda)?;
    B = evaluateExtRealMatrixArg(b.clone())?;
    LDB = evaluateExtIntArg(ldb)?;
    (A, IPIV, B, INFO) = Lapack::dgesv(N, NRHS, A, LDA, B, LDB);
    assignVariableExt(a, Expression::makeRealMatrix(A)?)?;
    assignVariable(&ipiv, &(Expression::makeIntegerArray(&IPIV)?))?;
    assignVariableExt(b, Expression::makeRealMatrix(B)?)?;
    assignVariable(&info, &(Expression::makeInteger(INFO)))?;
    Ok(())
}

pub(crate) fn Lapack_dgglse(mut args: &metamodelica::List<metamodelica::Ref<Expression::NFExpression>>) -> Result<()> {
    let mut m: metamodelica::Ref<Expression::NFExpression>;
    let mut n: metamodelica::Ref<Expression::NFExpression>;
    let mut p: metamodelica::Ref<Expression::NFExpression>;
    let mut a: metamodelica::Ref<Expression::NFExpression>;
    let mut lda: metamodelica::Ref<Expression::NFExpression>;
    let mut b: metamodelica::Ref<Expression::NFExpression>;
    let mut ldb: metamodelica::Ref<Expression::NFExpression>;
    let mut c: metamodelica::Ref<Expression::NFExpression>;
    let mut d: metamodelica::Ref<Expression::NFExpression>;
    let mut x: metamodelica::Ref<Expression::NFExpression>;
    let mut work: metamodelica::Ref<Expression::NFExpression>;
    let mut lwork: metamodelica::Ref<Expression::NFExpression>;
    let mut info: metamodelica::Ref<Expression::NFExpression>;
    let mut M: i32;
    let mut N: i32;
    let mut P: i32;
    let mut LDA: i32;
    let mut LDB: i32;
    let mut LWORK: i32;
    let mut INFO: i32;
    let mut A: metamodelica::List<metamodelica::List<metamodelica::Real>>;
    let mut B: metamodelica::List<metamodelica::List<metamodelica::Real>>;
    let mut C: metamodelica::List<metamodelica::Real>;
    let mut D: metamodelica::List<metamodelica::Real>;
    let mut WORK: metamodelica::List<metamodelica::Real>;
    let mut X: metamodelica::List<metamodelica::Real>;
    let (__pa0, __pa1, __pa2, __pa3, __pa4, __pa5, __pa6, __pa7, __pa8, __pa9, __pa10, __pa11, __pa12) = ::match_deref::match_deref! { match &((*args)) {
        Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: Deref @ metamodelica::ListNode::Cons { head: __pa1, tail: Deref @ metamodelica::ListNode::Cons { head: __pa2, tail: Deref @ metamodelica::ListNode::Cons { head: __pa3, tail: Deref @ metamodelica::ListNode::Cons { head: __pa4, tail: Deref @ metamodelica::ListNode::Cons { head: __pa5, tail: Deref @ metamodelica::ListNode::Cons { head: __pa6, tail: Deref @ metamodelica::ListNode::Cons { head: __pa7, tail: Deref @ metamodelica::ListNode::Cons { head: __pa8, tail: Deref @ metamodelica::ListNode::Cons { head: __pa9, tail: Deref @ metamodelica::ListNode::Cons { head: __pa10, tail: Deref @ metamodelica::ListNode::Cons { head: __pa11, tail: Deref @ metamodelica::ListNode::Cons { head: __pa12, tail: Deref @ metamodelica::ListNode::Nil } } } } } } } } } } } } } => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone(), __pa4.clone(), __pa5.clone(), __pa6.clone(), __pa7.clone(), __pa8.clone(), __pa9.clone(), __pa10.clone(), __pa11.clone(), __pa12.clone()),
        _ => return Err("pattern mismatch"),
    } };
    m = metamodelica::Own::own(__pa0);
    n = metamodelica::Own::own(__pa1);
    p = metamodelica::Own::own(__pa2);
    a = metamodelica::Own::own(__pa3);
    lda = metamodelica::Own::own(__pa4);
    b = metamodelica::Own::own(__pa5);
    ldb = metamodelica::Own::own(__pa6);
    c = metamodelica::Own::own(__pa7);
    d = metamodelica::Own::own(__pa8);
    x = metamodelica::Own::own(__pa9);
    work = metamodelica::Own::own(__pa10);
    lwork = metamodelica::Own::own(__pa11);
    info = metamodelica::Own::own(__pa12);
    M = evaluateExtIntArg(m)?;
    N = evaluateExtIntArg(n)?;
    P = evaluateExtIntArg(p)?;
    A = evaluateExtRealMatrixArg(a.clone())?;
    LDA = evaluateExtIntArg(lda)?;
    B = evaluateExtRealMatrixArg(b.clone())?;
    LDB = evaluateExtIntArg(ldb)?;
    C = evaluateExtRealArrayArg(c.clone())?;
    D = evaluateExtRealArrayArg(d.clone())?;
    WORK = evaluateExtRealArrayArg(work.clone())?;
    LWORK = evaluateExtIntArg(lwork)?;
    (A, B, C, D, X, WORK, INFO) = Lapack::dgglse(M, N, P, A, LDA, B, LDB, C, D, WORK, LWORK);
    assignVariableExt(a, Expression::makeRealMatrix(A)?)?;
    assignVariableExt(b, Expression::makeRealMatrix(B)?)?;
    assignVariable(&c, &(Expression::makeRealArray(&C)?))?;
    assignVariable(&d, &(Expression::makeRealArray(&D)?))?;
    assignVariable(&x, &(Expression::makeRealArray(&X)?))?;
    assignVariable(&work, &(Expression::makeRealArray(&WORK)?))?;
    assignVariable(&info, &(Expression::makeInteger(INFO)))?;
    Ok(())
}

pub(crate) fn Lapack_dgtsv(mut args: &metamodelica::List<metamodelica::Ref<Expression::NFExpression>>) -> Result<()> {
    let mut n: metamodelica::Ref<Expression::NFExpression>;
    let mut nrhs: metamodelica::Ref<Expression::NFExpression>;
    let mut dl: metamodelica::Ref<Expression::NFExpression>;
    let mut d: metamodelica::Ref<Expression::NFExpression>;
    let mut du: metamodelica::Ref<Expression::NFExpression>;
    let mut b: metamodelica::Ref<Expression::NFExpression>;
    let mut ldb: metamodelica::Ref<Expression::NFExpression>;
    let mut info: metamodelica::Ref<Expression::NFExpression>;
    let mut N: i32;
    let mut NRHS: i32;
    let mut LDB: i32;
    let mut INFO: i32;
    let mut DL: metamodelica::List<metamodelica::Real>;
    let mut D: metamodelica::List<metamodelica::Real>;
    let mut DU: metamodelica::List<metamodelica::Real>;
    let mut B: metamodelica::List<metamodelica::List<metamodelica::Real>>;
    let (__pa0, __pa1, __pa2, __pa3, __pa4, __pa5, __pa6, __pa7) = ::match_deref::match_deref! { match &((*args)) {
        Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: Deref @ metamodelica::ListNode::Cons { head: __pa1, tail: Deref @ metamodelica::ListNode::Cons { head: __pa2, tail: Deref @ metamodelica::ListNode::Cons { head: __pa3, tail: Deref @ metamodelica::ListNode::Cons { head: __pa4, tail: Deref @ metamodelica::ListNode::Cons { head: __pa5, tail: Deref @ metamodelica::ListNode::Cons { head: __pa6, tail: Deref @ metamodelica::ListNode::Cons { head: __pa7, tail: Deref @ metamodelica::ListNode::Nil } } } } } } } } => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone(), __pa4.clone(), __pa5.clone(), __pa6.clone(), __pa7.clone()),
        _ => return Err("pattern mismatch"),
    } };
    n = metamodelica::Own::own(__pa0);
    nrhs = metamodelica::Own::own(__pa1);
    dl = metamodelica::Own::own(__pa2);
    d = metamodelica::Own::own(__pa3);
    du = metamodelica::Own::own(__pa4);
    b = metamodelica::Own::own(__pa5);
    ldb = metamodelica::Own::own(__pa6);
    info = metamodelica::Own::own(__pa7);
    N = evaluateExtIntArg(n)?;
    NRHS = evaluateExtIntArg(nrhs)?;
    DL = evaluateExtRealArrayArg(dl.clone())?;
    D = evaluateExtRealArrayArg(d.clone())?;
    DU = evaluateExtRealArrayArg(du.clone())?;
    B = evaluateExtRealMatrixArg(b.clone())?;
    LDB = evaluateExtIntArg(ldb)?;
    (DL, D, DU, B, INFO) = Lapack::dgtsv(N, NRHS, DL, D, DU, B, LDB);
    assignVariable(&dl, &(Expression::makeRealArray(&DL)?))?;
    assignVariable(&d, &(Expression::makeRealArray(&D)?))?;
    assignVariable(&du, &(Expression::makeRealArray(&DU)?))?;
    assignVariableExt(b, Expression::makeRealMatrix(B)?)?;
    assignVariable(&info, &(Expression::makeInteger(INFO)))?;
    Ok(())
}

pub(crate) fn Lapack_dgbsv(mut args: &metamodelica::List<metamodelica::Ref<Expression::NFExpression>>) -> Result<()> {
    let mut n: metamodelica::Ref<Expression::NFExpression>;
    let mut kl: metamodelica::Ref<Expression::NFExpression>;
    let mut ku: metamodelica::Ref<Expression::NFExpression>;
    let mut nrhs: metamodelica::Ref<Expression::NFExpression>;
    let mut ab: metamodelica::Ref<Expression::NFExpression>;
    let mut ldab: metamodelica::Ref<Expression::NFExpression>;
    let mut ipiv: metamodelica::Ref<Expression::NFExpression>;
    let mut b: metamodelica::Ref<Expression::NFExpression>;
    let mut ldb: metamodelica::Ref<Expression::NFExpression>;
    let mut info: metamodelica::Ref<Expression::NFExpression>;
    let mut N: i32;
    let mut KL: i32;
    let mut KU: i32;
    let mut NRHS: i32;
    let mut LDAB: i32;
    let mut LDB: i32;
    let mut INFO: i32;
    let mut AB: metamodelica::List<metamodelica::List<metamodelica::Real>>;
    let mut B: metamodelica::List<metamodelica::List<metamodelica::Real>>;
    let mut IPIV: metamodelica::List<i32>;
    let (__pa0, __pa1, __pa2, __pa3, __pa4, __pa5, __pa6, __pa7, __pa8, __pa9) = ::match_deref::match_deref! { match &((*args)) {
        Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: Deref @ metamodelica::ListNode::Cons { head: __pa1, tail: Deref @ metamodelica::ListNode::Cons { head: __pa2, tail: Deref @ metamodelica::ListNode::Cons { head: __pa3, tail: Deref @ metamodelica::ListNode::Cons { head: __pa4, tail: Deref @ metamodelica::ListNode::Cons { head: __pa5, tail: Deref @ metamodelica::ListNode::Cons { head: __pa6, tail: Deref @ metamodelica::ListNode::Cons { head: __pa7, tail: Deref @ metamodelica::ListNode::Cons { head: __pa8, tail: Deref @ metamodelica::ListNode::Cons { head: __pa9, tail: Deref @ metamodelica::ListNode::Nil } } } } } } } } } } => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone(), __pa4.clone(), __pa5.clone(), __pa6.clone(), __pa7.clone(), __pa8.clone(), __pa9.clone()),
        _ => return Err("pattern mismatch"),
    } };
    n = metamodelica::Own::own(__pa0);
    kl = metamodelica::Own::own(__pa1);
    ku = metamodelica::Own::own(__pa2);
    nrhs = metamodelica::Own::own(__pa3);
    ab = metamodelica::Own::own(__pa4);
    ldab = metamodelica::Own::own(__pa5);
    ipiv = metamodelica::Own::own(__pa6);
    b = metamodelica::Own::own(__pa7);
    ldb = metamodelica::Own::own(__pa8);
    info = metamodelica::Own::own(__pa9);
    N = evaluateExtIntArg(n)?;
    KL = evaluateExtIntArg(kl)?;
    KU = evaluateExtIntArg(ku)?;
    NRHS = evaluateExtIntArg(nrhs)?;
    AB = evaluateExtRealMatrixArg(ab.clone())?;
    LDAB = evaluateExtIntArg(ldab)?;
    B = evaluateExtRealMatrixArg(b.clone())?;
    LDB = evaluateExtIntArg(ldb)?;
    (AB, IPIV, B, INFO) = Lapack::dgbsv(N, KL, KU, NRHS, AB, LDAB, B, LDB);
    assignVariableExt(ab, Expression::makeRealMatrix(AB)?)?;
    assignVariable(&ipiv, &(Expression::makeIntegerArray(&IPIV)?))?;
    assignVariableExt(b, Expression::makeRealMatrix(B)?)?;
    assignVariable(&info, &(Expression::makeInteger(INFO)))?;
    Ok(())
}

pub(crate) fn Lapack_dgesvd(mut args: &metamodelica::List<metamodelica::Ref<Expression::NFExpression>>) -> Result<()> {
    let mut jobu: metamodelica::Ref<Expression::NFExpression>;
    let mut jobvt: metamodelica::Ref<Expression::NFExpression>;
    let mut m: metamodelica::Ref<Expression::NFExpression>;
    let mut n: metamodelica::Ref<Expression::NFExpression>;
    let mut a: metamodelica::Ref<Expression::NFExpression>;
    let mut lda: metamodelica::Ref<Expression::NFExpression>;
    let mut s: metamodelica::Ref<Expression::NFExpression>;
    let mut u: metamodelica::Ref<Expression::NFExpression>;
    let mut ldu: metamodelica::Ref<Expression::NFExpression>;
    let mut vt: metamodelica::Ref<Expression::NFExpression>;
    let mut ldvt: metamodelica::Ref<Expression::NFExpression>;
    let mut work: metamodelica::Ref<Expression::NFExpression>;
    let mut lwork: metamodelica::Ref<Expression::NFExpression>;
    let mut info: metamodelica::Ref<Expression::NFExpression>;
    let mut JOBU: ArcStr;
    let mut JOBVT: ArcStr;
    let mut M: i32;
    let mut N: i32;
    let mut LDA: i32;
    let mut LDU: i32;
    let mut LDVT: i32;
    let mut LWORK: i32;
    let mut INFO: i32;
    let mut A: metamodelica::List<metamodelica::List<metamodelica::Real>>;
    let mut U: metamodelica::List<metamodelica::List<metamodelica::Real>>;
    let mut VT: metamodelica::List<metamodelica::List<metamodelica::Real>>;
    let mut S: metamodelica::List<metamodelica::Real>;
    let mut WORK: metamodelica::List<metamodelica::Real>;
    let (__pa0, __pa1, __pa2, __pa3, __pa4, __pa5, __pa6, __pa7, __pa8, __pa9, __pa10, __pa11, __pa12, __pa13) = ::match_deref::match_deref! { match &((*args)) {
        Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: Deref @ metamodelica::ListNode::Cons { head: __pa1, tail: Deref @ metamodelica::ListNode::Cons { head: __pa2, tail: Deref @ metamodelica::ListNode::Cons { head: __pa3, tail: Deref @ metamodelica::ListNode::Cons { head: __pa4, tail: Deref @ metamodelica::ListNode::Cons { head: __pa5, tail: Deref @ metamodelica::ListNode::Cons { head: __pa6, tail: Deref @ metamodelica::ListNode::Cons { head: __pa7, tail: Deref @ metamodelica::ListNode::Cons { head: __pa8, tail: Deref @ metamodelica::ListNode::Cons { head: __pa9, tail: Deref @ metamodelica::ListNode::Cons { head: __pa10, tail: Deref @ metamodelica::ListNode::Cons { head: __pa11, tail: Deref @ metamodelica::ListNode::Cons { head: __pa12, tail: Deref @ metamodelica::ListNode::Cons { head: __pa13, tail: Deref @ metamodelica::ListNode::Nil } } } } } } } } } } } } } } => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone(), __pa4.clone(), __pa5.clone(), __pa6.clone(), __pa7.clone(), __pa8.clone(), __pa9.clone(), __pa10.clone(), __pa11.clone(), __pa12.clone(), __pa13.clone()),
        _ => return Err("pattern mismatch"),
    } };
    jobu = metamodelica::Own::own(__pa0);
    jobvt = metamodelica::Own::own(__pa1);
    m = metamodelica::Own::own(__pa2);
    n = metamodelica::Own::own(__pa3);
    a = metamodelica::Own::own(__pa4);
    lda = metamodelica::Own::own(__pa5);
    s = metamodelica::Own::own(__pa6);
    u = metamodelica::Own::own(__pa7);
    ldu = metamodelica::Own::own(__pa8);
    vt = metamodelica::Own::own(__pa9);
    ldvt = metamodelica::Own::own(__pa10);
    work = metamodelica::Own::own(__pa11);
    lwork = metamodelica::Own::own(__pa12);
    info = metamodelica::Own::own(__pa13);
    JOBU = evaluateExtStringArg(jobu)?;
    JOBVT = evaluateExtStringArg(jobvt)?;
    M = evaluateExtIntArg(m)?;
    N = evaluateExtIntArg(n)?;
    A = evaluateExtRealMatrixArg(a.clone())?;
    LDA = evaluateExtIntArg(lda)?;
    LDU = evaluateExtIntArg(ldu)?;
    LDVT = evaluateExtIntArg(ldvt)?;
    WORK = evaluateExtRealArrayArg(work.clone())?;
    LWORK = evaluateExtIntArg(lwork)?;
    (A, S, U, VT, WORK, INFO) = Lapack::dgesvd(JOBU, JOBVT, M, N, A, LDA, LDU, LDVT, WORK, LWORK);
    assignVariableExt(a, Expression::makeRealMatrix(A)?)?;
    assignVariable(&s, &(Expression::makeRealArray(&S)?))?;
    assignVariableExt(u, Expression::makeRealMatrix(U)?)?;
    assignVariableExt(vt, Expression::makeRealMatrix(VT)?)?;
    assignVariable(&work, &(Expression::makeRealArray(&WORK)?))?;
    assignVariable(&info, &(Expression::makeInteger(INFO)))?;
    Ok(())
}

pub(crate) fn Lapack_dgetrf(mut args: &metamodelica::List<metamodelica::Ref<Expression::NFExpression>>) -> Result<()> {
    let mut m: metamodelica::Ref<Expression::NFExpression>;
    let mut n: metamodelica::Ref<Expression::NFExpression>;
    let mut a: metamodelica::Ref<Expression::NFExpression>;
    let mut lda: metamodelica::Ref<Expression::NFExpression>;
    let mut ipiv: metamodelica::Ref<Expression::NFExpression>;
    let mut info: metamodelica::Ref<Expression::NFExpression>;
    let mut M: i32;
    let mut N: i32;
    let mut LDA: i32;
    let mut INFO: i32;
    let mut A: metamodelica::List<metamodelica::List<metamodelica::Real>>;
    let mut IPIV: metamodelica::List<i32>;
    let (__pa0, __pa1, __pa2, __pa3, __pa4, __pa5) = ::match_deref::match_deref! { match &((*args)) {
        Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: Deref @ metamodelica::ListNode::Cons { head: __pa1, tail: Deref @ metamodelica::ListNode::Cons { head: __pa2, tail: Deref @ metamodelica::ListNode::Cons { head: __pa3, tail: Deref @ metamodelica::ListNode::Cons { head: __pa4, tail: Deref @ metamodelica::ListNode::Cons { head: __pa5, tail: Deref @ metamodelica::ListNode::Nil } } } } } } => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone(), __pa4.clone(), __pa5.clone()),
        _ => return Err("pattern mismatch"),
    } };
    m = metamodelica::Own::own(__pa0);
    n = metamodelica::Own::own(__pa1);
    a = metamodelica::Own::own(__pa2);
    lda = metamodelica::Own::own(__pa3);
    ipiv = metamodelica::Own::own(__pa4);
    info = metamodelica::Own::own(__pa5);
    M = evaluateExtIntArg(m)?;
    N = evaluateExtIntArg(n)?;
    A = evaluateExtRealMatrixArg(a.clone())?;
    LDA = evaluateExtIntArg(lda)?;
    (A, IPIV, INFO) = Lapack::dgetrf(M, N, A, LDA);
    assignVariableExt(a, Expression::makeRealMatrix(A)?)?;
    assignVariable(&ipiv, &(Expression::makeIntegerArray(&IPIV)?))?;
    assignVariable(&info, &(Expression::makeInteger(INFO)))?;
    Ok(())
}

pub(crate) fn Lapack_dgetrs(mut args: &metamodelica::List<metamodelica::Ref<Expression::NFExpression>>) -> Result<()> {
    let mut trans: metamodelica::Ref<Expression::NFExpression>;
    let mut n: metamodelica::Ref<Expression::NFExpression>;
    let mut nrhs: metamodelica::Ref<Expression::NFExpression>;
    let mut a: metamodelica::Ref<Expression::NFExpression>;
    let mut lda: metamodelica::Ref<Expression::NFExpression>;
    let mut ipiv: metamodelica::Ref<Expression::NFExpression>;
    let mut b: metamodelica::Ref<Expression::NFExpression>;
    let mut ldb: metamodelica::Ref<Expression::NFExpression>;
    let mut info: metamodelica::Ref<Expression::NFExpression>;
    let mut TRANS: ArcStr;
    let mut N: i32;
    let mut NRHS: i32;
    let mut LDA: i32;
    let mut LDB: i32;
    let mut INFO: i32;
    let mut A: metamodelica::List<metamodelica::List<metamodelica::Real>>;
    let mut B: metamodelica::List<metamodelica::List<metamodelica::Real>>;
    let mut IPIV: metamodelica::List<i32>;
    let (__pa0, __pa1, __pa2, __pa3, __pa4, __pa5, __pa6, __pa7, __pa8) = ::match_deref::match_deref! { match &((*args)) {
        Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: Deref @ metamodelica::ListNode::Cons { head: __pa1, tail: Deref @ metamodelica::ListNode::Cons { head: __pa2, tail: Deref @ metamodelica::ListNode::Cons { head: __pa3, tail: Deref @ metamodelica::ListNode::Cons { head: __pa4, tail: Deref @ metamodelica::ListNode::Cons { head: __pa5, tail: Deref @ metamodelica::ListNode::Cons { head: __pa6, tail: Deref @ metamodelica::ListNode::Cons { head: __pa7, tail: Deref @ metamodelica::ListNode::Cons { head: __pa8, tail: Deref @ metamodelica::ListNode::Nil } } } } } } } } } => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone(), __pa4.clone(), __pa5.clone(), __pa6.clone(), __pa7.clone(), __pa8.clone()),
        _ => return Err("pattern mismatch"),
    } };
    trans = metamodelica::Own::own(__pa0);
    n = metamodelica::Own::own(__pa1);
    nrhs = metamodelica::Own::own(__pa2);
    a = metamodelica::Own::own(__pa3);
    lda = metamodelica::Own::own(__pa4);
    ipiv = metamodelica::Own::own(__pa5);
    b = metamodelica::Own::own(__pa6);
    ldb = metamodelica::Own::own(__pa7);
    info = metamodelica::Own::own(__pa8);
    TRANS = evaluateExtStringArg(trans)?;
    N = evaluateExtIntArg(n)?;
    NRHS = evaluateExtIntArg(nrhs)?;
    A = evaluateExtRealMatrixArg(a)?;
    LDA = evaluateExtIntArg(lda)?;
    IPIV = evaluateExtIntArrayArg(ipiv)?;
    B = evaluateExtRealMatrixArg(b.clone())?;
    LDB = evaluateExtIntArg(ldb)?;
    (B, INFO) = Lapack::dgetrs(TRANS, N, NRHS, A, LDA, IPIV, B, LDB);
    assignVariableExt(b, Expression::makeRealMatrix(B)?)?;
    assignVariable(&info, &(Expression::makeInteger(INFO)))?;
    Ok(())
}

pub(crate) fn Lapack_dgetri(mut args: &metamodelica::List<metamodelica::Ref<Expression::NFExpression>>) -> Result<()> {
    let mut n: metamodelica::Ref<Expression::NFExpression>;
    let mut a: metamodelica::Ref<Expression::NFExpression>;
    let mut lda: metamodelica::Ref<Expression::NFExpression>;
    let mut ipiv: metamodelica::Ref<Expression::NFExpression>;
    let mut work: metamodelica::Ref<Expression::NFExpression>;
    let mut lwork: metamodelica::Ref<Expression::NFExpression>;
    let mut info: metamodelica::Ref<Expression::NFExpression>;
    let mut N: i32;
    let mut LDA: i32;
    let mut LWORK: i32;
    let mut INFO: i32;
    let mut A: metamodelica::List<metamodelica::List<metamodelica::Real>>;
    let mut IPIV: metamodelica::List<i32>;
    let mut WORK: metamodelica::List<metamodelica::Real>;
    let (__pa0, __pa1, __pa2, __pa3, __pa4, __pa5, __pa6) = ::match_deref::match_deref! { match &((*args)) {
        Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: Deref @ metamodelica::ListNode::Cons { head: __pa1, tail: Deref @ metamodelica::ListNode::Cons { head: __pa2, tail: Deref @ metamodelica::ListNode::Cons { head: __pa3, tail: Deref @ metamodelica::ListNode::Cons { head: __pa4, tail: Deref @ metamodelica::ListNode::Cons { head: __pa5, tail: Deref @ metamodelica::ListNode::Cons { head: __pa6, tail: Deref @ metamodelica::ListNode::Nil } } } } } } } => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone(), __pa4.clone(), __pa5.clone(), __pa6.clone()),
        _ => return Err("pattern mismatch"),
    } };
    n = metamodelica::Own::own(__pa0);
    a = metamodelica::Own::own(__pa1);
    lda = metamodelica::Own::own(__pa2);
    ipiv = metamodelica::Own::own(__pa3);
    work = metamodelica::Own::own(__pa4);
    lwork = metamodelica::Own::own(__pa5);
    info = metamodelica::Own::own(__pa6);
    N = evaluateExtIntArg(n)?;
    A = evaluateExtRealMatrixArg(a.clone())?;
    LDA = evaluateExtIntArg(lda)?;
    IPIV = evaluateExtIntArrayArg(ipiv)?;
    WORK = evaluateExtRealArrayArg(work.clone())?;
    LWORK = evaluateExtIntArg(lwork)?;
    (A, WORK, INFO) = Lapack::dgetri(N, A, LDA, IPIV, WORK, LWORK);
    assignVariableExt(a, Expression::makeRealMatrix(A)?)?;
    assignVariable(&work, &(Expression::makeRealArray(&WORK)?))?;
    assignVariable(&info, &(Expression::makeInteger(INFO)))?;
    Ok(())
}

pub(crate) fn Lapack_dgeqpf(mut args: &metamodelica::List<metamodelica::Ref<Expression::NFExpression>>) -> Result<()> {
    let mut m: metamodelica::Ref<Expression::NFExpression>;
    let mut n: metamodelica::Ref<Expression::NFExpression>;
    let mut a: metamodelica::Ref<Expression::NFExpression>;
    let mut lda: metamodelica::Ref<Expression::NFExpression>;
    let mut jpvt: metamodelica::Ref<Expression::NFExpression>;
    let mut tau: metamodelica::Ref<Expression::NFExpression>;
    let mut work: metamodelica::Ref<Expression::NFExpression>;
    let mut info: metamodelica::Ref<Expression::NFExpression>;
    let mut M: i32;
    let mut N: i32;
    let mut LDA: i32;
    let mut INFO: i32;
    let mut A: metamodelica::List<metamodelica::List<metamodelica::Real>>;
    let mut JPVT: metamodelica::List<i32>;
    let mut WORK: metamodelica::List<metamodelica::Real>;
    let mut TAU: metamodelica::List<metamodelica::Real>;
    let (__pa0, __pa1, __pa2, __pa3, __pa4, __pa5, __pa6, __pa7) = ::match_deref::match_deref! { match &((*args)) {
        Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: Deref @ metamodelica::ListNode::Cons { head: __pa1, tail: Deref @ metamodelica::ListNode::Cons { head: __pa2, tail: Deref @ metamodelica::ListNode::Cons { head: __pa3, tail: Deref @ metamodelica::ListNode::Cons { head: __pa4, tail: Deref @ metamodelica::ListNode::Cons { head: __pa5, tail: Deref @ metamodelica::ListNode::Cons { head: __pa6, tail: Deref @ metamodelica::ListNode::Cons { head: __pa7, tail: Deref @ metamodelica::ListNode::Nil } } } } } } } } => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone(), __pa4.clone(), __pa5.clone(), __pa6.clone(), __pa7.clone()),
        _ => return Err("pattern mismatch"),
    } };
    m = metamodelica::Own::own(__pa0);
    n = metamodelica::Own::own(__pa1);
    a = metamodelica::Own::own(__pa2);
    lda = metamodelica::Own::own(__pa3);
    jpvt = metamodelica::Own::own(__pa4);
    tau = metamodelica::Own::own(__pa5);
    work = metamodelica::Own::own(__pa6);
    info = metamodelica::Own::own(__pa7);
    M = evaluateExtIntArg(m)?;
    N = evaluateExtIntArg(n)?;
    A = evaluateExtRealMatrixArg(a.clone())?;
    LDA = evaluateExtIntArg(lda)?;
    JPVT = evaluateExtIntArrayArg(jpvt.clone())?;
    WORK = evaluateExtRealArrayArg(work)?;
    (A, JPVT, TAU, INFO) = Lapack::dgeqpf(M, N, A, LDA, JPVT, WORK);
    assignVariableExt(a, Expression::makeRealMatrix(A)?)?;
    assignVariable(&jpvt, &(Expression::makeIntegerArray(&JPVT)?))?;
    assignVariable(&tau, &(Expression::makeRealArray(&TAU)?))?;
    assignVariable(&info, &(Expression::makeInteger(INFO)))?;
    Ok(())
}

pub(crate) fn Lapack_dorgqr(mut args: &metamodelica::List<metamodelica::Ref<Expression::NFExpression>>) -> Result<()> {
    let mut m: metamodelica::Ref<Expression::NFExpression>;
    let mut n: metamodelica::Ref<Expression::NFExpression>;
    let mut k: metamodelica::Ref<Expression::NFExpression>;
    let mut a: metamodelica::Ref<Expression::NFExpression>;
    let mut lda: metamodelica::Ref<Expression::NFExpression>;
    let mut tau: metamodelica::Ref<Expression::NFExpression>;
    let mut work: metamodelica::Ref<Expression::NFExpression>;
    let mut lwork: metamodelica::Ref<Expression::NFExpression>;
    let mut info: metamodelica::Ref<Expression::NFExpression>;
    let mut M: i32;
    let mut N: i32;
    let mut K: i32;
    let mut LDA: i32;
    let mut LWORK: i32;
    let mut INFO: i32;
    let mut A: metamodelica::List<metamodelica::List<metamodelica::Real>>;
    let mut TAU: metamodelica::List<metamodelica::Real>;
    let mut WORK: metamodelica::List<metamodelica::Real>;
    let (__pa0, __pa1, __pa2, __pa3, __pa4, __pa5, __pa6, __pa7, __pa8) = ::match_deref::match_deref! { match &((*args)) {
        Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: Deref @ metamodelica::ListNode::Cons { head: __pa1, tail: Deref @ metamodelica::ListNode::Cons { head: __pa2, tail: Deref @ metamodelica::ListNode::Cons { head: __pa3, tail: Deref @ metamodelica::ListNode::Cons { head: __pa4, tail: Deref @ metamodelica::ListNode::Cons { head: __pa5, tail: Deref @ metamodelica::ListNode::Cons { head: __pa6, tail: Deref @ metamodelica::ListNode::Cons { head: __pa7, tail: Deref @ metamodelica::ListNode::Cons { head: __pa8, tail: Deref @ metamodelica::ListNode::Nil } } } } } } } } } => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone(), __pa4.clone(), __pa5.clone(), __pa6.clone(), __pa7.clone(), __pa8.clone()),
        _ => return Err("pattern mismatch"),
    } };
    m = metamodelica::Own::own(__pa0);
    n = metamodelica::Own::own(__pa1);
    k = metamodelica::Own::own(__pa2);
    a = metamodelica::Own::own(__pa3);
    lda = metamodelica::Own::own(__pa4);
    tau = metamodelica::Own::own(__pa5);
    work = metamodelica::Own::own(__pa6);
    lwork = metamodelica::Own::own(__pa7);
    info = metamodelica::Own::own(__pa8);
    M = evaluateExtIntArg(m)?;
    N = evaluateExtIntArg(n)?;
    K = evaluateExtIntArg(k)?;
    A = evaluateExtRealMatrixArg(a.clone())?;
    LDA = evaluateExtIntArg(lda)?;
    TAU = evaluateExtRealArrayArg(tau)?;
    WORK = evaluateExtRealArrayArg(work.clone())?;
    LWORK = evaluateExtIntArg(lwork)?;
    (A, WORK, INFO) = Lapack::dorgqr(M, N, K, A, LDA, TAU, WORK, LWORK);
    assignVariableExt(a, Expression::makeRealMatrix(A)?)?;
    assignVariable(&work, &(Expression::makeRealArray(&WORK)?))?;
    assignVariable(&info, &(Expression::makeInteger(INFO)))?;
    Ok(())
}

pub(crate) fn Lapack_dhseqr(mut args: &metamodelica::List<metamodelica::Ref<Expression::NFExpression>>) -> Result<()> {
    let mut job: metamodelica::Ref<Expression::NFExpression>;
    let mut compz: metamodelica::Ref<Expression::NFExpression>;
    let mut n: metamodelica::Ref<Expression::NFExpression>;
    let mut ilo: metamodelica::Ref<Expression::NFExpression>;
    let mut ihi: metamodelica::Ref<Expression::NFExpression>;
    let mut h: metamodelica::Ref<Expression::NFExpression>;
    let mut ldh: metamodelica::Ref<Expression::NFExpression>;
    let mut wr: metamodelica::Ref<Expression::NFExpression>;
    let mut wi: metamodelica::Ref<Expression::NFExpression>;
    let mut z: metamodelica::Ref<Expression::NFExpression>;
    let mut ldz: metamodelica::Ref<Expression::NFExpression>;
    let mut work: metamodelica::Ref<Expression::NFExpression>;
    let mut lwork: metamodelica::Ref<Expression::NFExpression>;
    let mut info: metamodelica::Ref<Expression::NFExpression>;
    let mut N: i32;
    let mut ILO: i32;
    let mut IHI: i32;
    let mut LDH: i32;
    let mut LDZ: i32;
    let mut LWORK: i32;
    let mut INFO: i32;
    let mut JOB: ArcStr;
    let mut COMPZ: ArcStr;
    let mut H: metamodelica::List<metamodelica::List<metamodelica::Real>>;
    let mut Z: metamodelica::List<metamodelica::List<metamodelica::Real>>;
    let mut WR: metamodelica::List<metamodelica::Real>;
    let mut WI: metamodelica::List<metamodelica::Real>;
    let mut WORK: metamodelica::List<metamodelica::Real>;
    let (__pa0, __pa1, __pa2, __pa3, __pa4, __pa5, __pa6, __pa7, __pa8, __pa9, __pa10, __pa11, __pa12, __pa13) = ::match_deref::match_deref! { match &((*args)) {
        Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: Deref @ metamodelica::ListNode::Cons { head: __pa1, tail: Deref @ metamodelica::ListNode::Cons { head: __pa2, tail: Deref @ metamodelica::ListNode::Cons { head: __pa3, tail: Deref @ metamodelica::ListNode::Cons { head: __pa4, tail: Deref @ metamodelica::ListNode::Cons { head: __pa5, tail: Deref @ metamodelica::ListNode::Cons { head: __pa6, tail: Deref @ metamodelica::ListNode::Cons { head: __pa7, tail: Deref @ metamodelica::ListNode::Cons { head: __pa8, tail: Deref @ metamodelica::ListNode::Cons { head: __pa9, tail: Deref @ metamodelica::ListNode::Cons { head: __pa10, tail: Deref @ metamodelica::ListNode::Cons { head: __pa11, tail: Deref @ metamodelica::ListNode::Cons { head: __pa12, tail: Deref @ metamodelica::ListNode::Cons { head: __pa13, tail: Deref @ metamodelica::ListNode::Nil } } } } } } } } } } } } } } => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone(), __pa4.clone(), __pa5.clone(), __pa6.clone(), __pa7.clone(), __pa8.clone(), __pa9.clone(), __pa10.clone(), __pa11.clone(), __pa12.clone(), __pa13.clone()),
        _ => return Err("pattern mismatch"),
    } };
    job = metamodelica::Own::own(__pa0);
    compz = metamodelica::Own::own(__pa1);
    n = metamodelica::Own::own(__pa2);
    ilo = metamodelica::Own::own(__pa3);
    ihi = metamodelica::Own::own(__pa4);
    h = metamodelica::Own::own(__pa5);
    ldh = metamodelica::Own::own(__pa6);
    wr = metamodelica::Own::own(__pa7);
    wi = metamodelica::Own::own(__pa8);
    z = metamodelica::Own::own(__pa9);
    ldz = metamodelica::Own::own(__pa10);
    work = metamodelica::Own::own(__pa11);
    lwork = metamodelica::Own::own(__pa12);
    info = metamodelica::Own::own(__pa13);
    JOB = evaluateExtStringArg(job)?;
    COMPZ = evaluateExtStringArg(compz)?;
    N = evaluateExtIntArg(n)?;
    ILO = evaluateExtIntArg(ilo)?;
    IHI = evaluateExtIntArg(ihi)?;
    H = evaluateExtRealMatrixArg(h.clone())?;
    LDH = evaluateExtIntArg(ldh)?;
    Z = evaluateExtRealMatrixArg(z.clone())?;
    LDZ = evaluateExtIntArg(ldz)?;
    WORK = evaluateExtRealArrayArg(work.clone())?;
    LWORK = evaluateExtIntArg(lwork)?;
    (H, WR, WI, Z, WORK, INFO) = Lapack::dhseqr(JOB, COMPZ, N, ILO, IHI, H, LDH, Z, LDZ, WORK, LWORK);
    assignVariableExt(h, Expression::makeRealMatrix(H)?)?;
    assignVariable(&wr, &(Expression::makeRealArray(&WR)?))?;
    assignVariable(&wi, &(Expression::makeRealArray(&WI)?))?;
    assignVariableExt(z, Expression::makeRealMatrix(Z)?)?;
    assignVariable(&work, &(Expression::makeRealArray(&WORK)?))?;
    assignVariable(&info, &(Expression::makeInteger(INFO)))?;
    Ok(())
}

fn evaluateExtIntArg(mut arg: metamodelica::Ref<Expression::NFExpression>) -> Result<i32> {
    let mut value: i32 = getExtIntValue(&(Ceval::evalExp(arg.clone(), &(Ceval::noTarget().clone()))?))?;
    Ok(value)
}

fn getExtIntValue(mut exp: &metamodelica::Ref<Expression::NFExpression>) -> Result<i32> {
    let mut value: i32;
    value = (match &**exp {
        Expression::INTEGER { value: __exp_value } => __exp_value.clone(),
        Expression::EMPTY { .. } => 0,
        _ => return Err("match: no arm matched"),
    });
    Ok(value)
}

fn evaluateExtRealArg(mut arg: metamodelica::Ref<Expression::NFExpression>) -> Result<metamodelica::Real> {
    let mut value: metamodelica::Real = getExtRealValue(&(Ceval::evalExp(arg.clone(), &(Ceval::noTarget().clone()))?))?;
    Ok(value)
}

fn getExtRealValue(mut exp: &metamodelica::Ref<Expression::NFExpression>) -> Result<metamodelica::Real> {
    let mut value: metamodelica::Real;
    value = (match &**exp {
        Expression::REAL { value: __exp_value } => __exp_value.clone(),
        Expression::EMPTY { .. } => metamodelica::OrderedFloat(0.0_f64),
        _ => return Err("match: no arm matched"),
    });
    Ok(value)
}

fn evaluateExtStringArg(mut arg: metamodelica::Ref<Expression::NFExpression>) -> Result<ArcStr> {
    let mut value: ArcStr = getExtStringValue(&(Ceval::evalExp(arg.clone(), &(Ceval::noTarget().clone()))?))?;
    Ok(value)
}

fn getExtStringValue(mut exp: &metamodelica::Ref<Expression::NFExpression>) -> Result<ArcStr> {
    let mut value: ArcStr;
    value = (match &**exp {
        Expression::STRING { value: __exp_value } => __exp_value.clone(),
        Expression::EMPTY { .. } => literal!(""),
        _ => return Err("match: no arm matched"),
    });
    Ok(value)
}

fn evaluateExtIntArrayArg(mut arg: metamodelica::Ref<Expression::NFExpression>) -> Result<metamodelica::List<i32>> {
    let mut value: metamodelica::List<i32>;
    let mut expl: metamodelica::List<metamodelica::Ref<Expression::NFExpression>>;
    expl = Expression::arrayElementList(&(Ceval::evalExp(arg, &(Ceval::noTarget().clone()))?))?;
    value = ({
        let mut __acc: metamodelica::List<i32> = metamodelica::nil();
        for mut e in (expl).into_iter().cloned() {
            let __x = getExtIntValue(&(e.clone()))?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    Ok(value)
}

fn evaluateExtRealArrayArg(
    mut arg: metamodelica::Ref<Expression::NFExpression>,
) -> Result<metamodelica::List<metamodelica::Real>> {
    let mut value: metamodelica::List<metamodelica::Real>;
    let mut expl: metamodelica::List<metamodelica::Ref<Expression::NFExpression>>;
    expl = Expression::arrayElementList(&(Ceval::evalExp(arg, &(Ceval::noTarget().clone()))?))?;
    value = ({
        let mut __acc: metamodelica::List<metamodelica::Real> = metamodelica::nil();
        for mut e in (expl).into_iter().cloned() {
            let __x = getExtRealValue(&(e.clone()))?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    Ok(value)
}

fn evaluateExtRealMatrixArg(
    mut arg: metamodelica::Ref<Expression::NFExpression>,
) -> Result<metamodelica::List<metamodelica::List<metamodelica::Real>>> {
    let mut value: metamodelica::List<metamodelica::List<metamodelica::Real>>;
    let mut expl: metamodelica::Array<metamodelica::Ref<Expression::NFExpression>>;
    let mut ty: metamodelica::Ref<Type::NFType>;
    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(Ceval::evalExp(arg, &(Ceval::noTarget().clone()))?) {
        Deref @ Expression::ARRAY { ty: __pa0, elements: __pa1, .. } => (__pa0.clone(), __pa1.clone()),
        _ => return Err("pattern mismatch"),
    } };
    ty = metamodelica::Own::own(__pa0);
    expl = metamodelica::Own::own(__pa1);
    value = (match Type::dimensionCount(ty) {
        1 => {
            ({
                let mut __acc: metamodelica::List<metamodelica::List<metamodelica::Real>> = metamodelica::nil();
                for mut e in (expl.clone()).borrow().iter() {
                    let __x = list![getExtRealValue(&(e.clone()))?];
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            })
        }
        2 => {
            ({
                let mut __acc: metamodelica::List<metamodelica::List<metamodelica::Real>> = metamodelica::nil();
                for mut row in (expl.clone()).borrow().iter() {
                    let __x = ({
                        let mut __acc: metamodelica::List<metamodelica::Real> = metamodelica::nil();
                        for mut e in (Expression::arrayElements(&(row.clone()))?).borrow().iter() {
                            let __x = getExtRealValue(&(e.clone()))?;
                            __acc = cons(__x, __acc);
                        }
                        __acc.reverse()
                    });
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            })
        }
        _ => return Err("match: no arm matched"),
    });
    Ok(value)
}

fn assignVariableExt(
    mut variable: metamodelica::Ref<Expression::NFExpression>,
    mut value: metamodelica::Ref<Expression::NFExpression>,
) -> Result<()> {
    let mut exp: metamodelica::Ref<Expression::NFExpression>;
    exp = (::match_deref::match_deref! { match &((Expression::typeOf(variable.clone()), value.clone())) {
        (Deref @ Type::ARRAY { dimensions: Deref @ metamodelica::ListNode::Cons { head: _, tail: Deref @ metamodelica::ListNode::Nil }, .. }, Deref @ Expression::ARRAY { ty: Deref @ Type::ARRAY { dimensions: Deref @ metamodelica::ListNode::Cons { head: _, tail: Deref @ metamodelica::ListNode::Cons { head: _, tail: Deref @ metamodelica::ListNode::Nil } }, .. }, .. }) => Expression::makeArray(Type::unliftArray(var_field!((*value).ty, Expression::NFExpression::ARRAY).clone())?, metamodelica::arrayFromVec(({
        let mut __acc: metamodelica::List<metamodelica::Ref<Expression::NFExpression>> = metamodelica::nil();
        for mut e in (var_field!((*value).elements, Expression::NFExpression::ARRAY).clone()).borrow().iter() {
            let __x = Expression::arrayScalarElement(&(e.clone()))?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }).into_iter().cloned().collect()), true),
        _ => value,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    assignVariable(&variable, &exp)?;
    Ok(())
}
