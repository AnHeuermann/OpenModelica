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

use crate::ExpressionBasics;
use openmodelica_ast::Absyn;
use openmodelica_frontend_types::DAE;
use openmodelica_frontend_types::SCode;
use openmodelica_util::Error;
use openmodelica_util::Flags;
use openmodelica_util_datatypes_basic::List;

pub fn mergeSources(
    mut src1: &metamodelica::Ref<DAE::ElementSource>,
    mut src2: &metamodelica::Ref<DAE::ElementSource>,
) -> Result<metamodelica::Ref<DAE::ElementSource>> {
    let mut mergedSrc: metamodelica::Ref<DAE::ElementSource>;
    mergedSrc = (::match_deref::match_deref! { match (src1, src2) {
        (Deref @ DAE::ElementSource { info, partOfLst: partOfLst1, instance: instanceOpt1, connectEquationOptLst: connectEquationOptLst1, typeLst: typeLst1, operations: operations1, comment: comment1 }, Deref @ DAE::ElementSource { info: _, partOfLst: partOfLst2, instance: instanceOpt2, connectEquationOptLst: connectEquationOptLst2, typeLst: typeLst2, operations: operations2, comment: comment2 }) => {
            let mut p: metamodelica::List<Absyn::Within>;
            let mut i: metamodelica::Ref<DAE::ComponentPrefix>;
            let mut c: metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, metamodelica::Ref<DAE::ComponentRef>)>;
            let mut t: metamodelica::List<metamodelica::Ref<Absyn::Path>>;
            let mut o: metamodelica::List<metamodelica::Ref<DAE::SymbolicOperation>>;
            let mut comment: metamodelica::List<metamodelica::Ref<SCode::Comment>>;
            p = List::union(partOfLst1, partOfLst2);
            i = (match &*instanceOpt1.clone() {
        DAE::ComponentPrefix::NOCOMPPRE { .. } => instanceOpt2.clone(),
        _ => instanceOpt1.clone(),
    });
            c = List::union(connectEquationOptLst1, connectEquationOptLst2);
            t = List::union(typeLst1, typeLst2);
            o = listAppend(operations1.clone(), operations2.clone());
            comment = List::union(comment1, comment2);
            metamodelica::Ref::new(DAE::ElementSource { info: info.clone(), partOfLst: p, instance: i, connectEquationOptLst: c, typeLst: t, operations: o, comment: comment })
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(mergedSrc)
}

pub fn addCommentToSource(
    mut source: metamodelica::Ref<DAE::ElementSource>,
    mut commentIn: Option<metamodelica::Ref<SCode::Comment>>,
) -> metamodelica::Ref<DAE::ElementSource> {
    let mut source: metamodelica::Ref<DAE::ElementSource> = source;
    source = (::match_deref::match_deref! { match &((source.clone(), commentIn)) {
        (Deref @ DAE::ElementSource { info: _, partOfLst: _, instance: _, connectEquationOptLst: _, typeLst: _, operations: _, comment: _ }, Some(comment)) => {
            assign_field!(source.comment = metamodelica::cons(comment.clone(), source.comment.clone()));
            source
        },
        _ => {
            source
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    source
}

pub fn createElementSource(
    mut fileInfo: SourceInfo,
    mut partOf: Option<metamodelica::Ref<Absyn::Path>>,
    mut prefix: &DAE::Prefix,
    mut connectEquation: (
        metamodelica::Ref<DAE::ComponentRef>,
        metamodelica::Ref<DAE::ComponentRef>,
    ),
) -> metamodelica::Ref<DAE::ElementSource> {
    let mut source: metamodelica::Ref<DAE::ElementSource>;
    let mut path: metamodelica::Ref<Absyn::Path>;
    source = metamodelica::Ref::new(DAE::ElementSource {
        info: fileInfo,
        partOfLst: (::match_deref::match_deref! { match &(partOf) {
            None => metamodelica::nil(),
            Some(__esc_path) => {
                path = (*__esc_path).clone();
                list![Absyn::Within::WITHIN { path: path.clone() }]
            },
            _ => unreachable!("match_deref! exhaustiveness placeholder"),
        } }),
        instance: (match prefix.clone() {
            DAE::Prefix::NOPRE { .. } => openmodelica_frontend_types::DAE::ComponentPrefix::interned_NOCOMPPRE(),
            DAE::Prefix::PREFIX { .. } => var_field!(prefix.compPre, DAE::Prefix::PREFIX).clone(),
        }),
        connectEquationOptLst: (::match_deref::match_deref! { match &(connectEquation.clone()) {
            (Deref @ DAE::ComponentRef::CREF_IDENT { ident: Deref @ "", .. }, _) => metamodelica::nil(),
            _ => list![connectEquation],
            _ => unreachable!("match_deref! exhaustiveness placeholder"),
        } }),
        typeLst: metamodelica::nil(),
        operations: metamodelica::nil(),
        comment: metamodelica::nil(),
    });
    source
}

pub fn addAdditionalComment(
    mut source: &metamodelica::Ref<DAE::ElementSource>,
    mut message: ArcStr,
) -> metamodelica::Ref<DAE::ElementSource> {
    let mut outSource: metamodelica::Ref<DAE::ElementSource>;
    outSource = (match &**source {
        DAE::ElementSource {
            info,
            partOfLst,
            instance: instanceOpt,
            connectEquationOptLst,
            typeLst,
            operations,
            comment,
        } => {
            let mut b: bool;
            let mut c: metamodelica::Ref<SCode::Comment>;
            let mut comment = (*comment).clone();
            c = metamodelica::Ref::new(SCode::Comment {
                annotation_: None,
                comment: Some(message),
            });
            b = listMember(c.clone(), comment.clone());
            comment = if (b) {
                comment.clone()
            } else {
                metamodelica::cons(c, comment.clone())
            };
            metamodelica::Ref::new(DAE::ElementSource {
                info: info.clone(),
                partOfLst: partOfLst.clone(),
                instance: instanceOpt.clone(),
                connectEquationOptLst: connectEquationOptLst.clone(),
                typeLst: typeLst.clone(),
                operations: operations.clone(),
                comment: comment.clone(),
            })
        }
    });
    outSource
}

pub fn addAnnotation(
    mut source: metamodelica::Ref<DAE::ElementSource>,
    mut comment: metamodelica::Ref<SCode::Comment>,
) -> metamodelica::Ref<DAE::ElementSource> {
    let mut outSource: metamodelica::Ref<DAE::ElementSource>;
    outSource = (::match_deref::match_deref! { match &((source.clone(), comment.clone())) {
        (Deref @ DAE::ElementSource { info, partOfLst, instance: instanceOpt, connectEquationOptLst, typeLst, operations, comment: commentLst }, Deref @ SCode::Comment { annotation_: Some(_), .. }) => {
            metamodelica::Ref::new(DAE::ElementSource { info: info.clone(), partOfLst: partOfLst.clone(), instance: instanceOpt.clone(), connectEquationOptLst: connectEquationOptLst.clone(), typeLst: typeLst.clone(), operations: operations.clone(), comment: metamodelica::cons(comment, commentLst.clone()) })
        },
        _ => {
            source
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outSource
}

pub fn getComments(
    mut source: &metamodelica::Ref<DAE::ElementSource>,
) -> metamodelica::List<metamodelica::Ref<SCode::Comment>> {
    let mut outComments: metamodelica::List<metamodelica::Ref<SCode::Comment>>;
    outComments = (match &**source {
        DAE::ElementSource { comment, .. } => comment.clone(),
    });
    outComments
}

pub fn getOptComment(
    mut source: &metamodelica::Ref<DAE::ElementSource>,
) -> Result<Option<metamodelica::Ref<SCode::Comment>>> {
    let mut outComment: Option<metamodelica::Ref<SCode::Comment>>;
    if !((source.comment).is_empty()) {
        outComment = Some(List::last(&source.comment)?);
    } else {
        outComment = None;
    }
    Ok(outComment)
}

pub fn addSymbolicTransformation(
    mut source: metamodelica::Ref<DAE::ElementSource>,
    mut op: metamodelica::Ref<DAE::SymbolicOperation>,
) -> Result<metamodelica::Ref<DAE::ElementSource>> {
    let mut source: metamodelica::Ref<DAE::ElementSource> = source;
    if !(Flags::isSet(Flags::INFO_XML_OPERATIONS.clone())?) {
        return Ok(source);
    }
    source = (::match_deref::match_deref! { match &((source, op.clone())) {
        (Deref @ DAE::ElementSource { info, partOfLst, instance: instanceOpt, connectEquationOptLst, typeLst, operations: Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::SymbolicOperation::SUBSTITUTION { substitutions: es1 @ Deref @ metamodelica::ListNode::Cons { head: h1, tail: _ }, source: t1 }, tail: operations }, comment }, Deref @ DAE::SymbolicOperation::SUBSTITUTION { substitutions: es2, source: t2 }) if (ExpressionBasics::expEqual(metamodelica::AsArg::as_arg(&t2), h1.clone())?) => {
            let mut es: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
            es = listAppend(es2.clone(), es1.clone());
            metamodelica::Ref::new(DAE::ElementSource { info: info.clone(), partOfLst: partOfLst.clone(), instance: instanceOpt.clone(), connectEquationOptLst: connectEquationOptLst.clone(), typeLst: typeLst.clone(), operations: metamodelica::cons(metamodelica::Ref::new(DAE::SymbolicOperation::SUBSTITUTION { substitutions: es, source: t1.clone() }), operations.clone()), comment: comment.clone() })
        },
        (Deref @ DAE::ElementSource { info, partOfLst, instance: instanceOpt, connectEquationOptLst, typeLst, operations, comment }, _) => {
            metamodelica::Ref::new(DAE::ElementSource { info: info.clone(), partOfLst: partOfLst.clone(), instance: instanceOpt.clone(), connectEquationOptLst: connectEquationOptLst.clone(), typeLst: typeLst.clone(), operations: metamodelica::cons(op, operations.clone()), comment: comment.clone() })
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(source)
}

pub fn condAddSymbolicTransformation(
    mut cond: bool,
    mut source: metamodelica::Ref<DAE::ElementSource>,
    mut op: metamodelica::Ref<DAE::SymbolicOperation>,
) -> Result<metamodelica::Ref<DAE::ElementSource>> {
    let mut source: metamodelica::Ref<DAE::ElementSource> = source;
    if !(cond) {
        return Ok(source);
    }
    source = addSymbolicTransformation(source, op)?;
    Ok(source)
}

pub(crate) fn addSymbolicTransformationDeriveLst<'__b>(
    mut source: metamodelica::Ref<DAE::ElementSource>,
    mut explst1: &'__b metamodelica::List<metamodelica::Ref<DAE::Exp>>,
    mut explst2: &'__b metamodelica::List<metamodelica::Ref<DAE::Exp>>,
) -> Result<metamodelica::Ref<DAE::ElementSource>> {
    let mut source: metamodelica::Ref<DAE::ElementSource> = source;
    if !(Flags::isSet(Flags::INFO_XML_OPERATIONS.clone())?) {
        return Ok(source);
    }
    source = (::match_deref::match_deref! { match (explst1, explst2) {
        (Deref @ metamodelica::ListNode::Nil, _) => {
            source
        },
        (Deref @ metamodelica::ListNode::Cons { head: exp1, tail: rexplst1 }, Deref @ metamodelica::ListNode::Cons { head: exp2, tail: rexplst2 }) => {
            let mut op: metamodelica::Ref<DAE::SymbolicOperation>;
            op = metamodelica::Ref::new(DAE::SymbolicOperation::OP_DIFFERENTIATE { cr: DAE::crefTime().clone(), before: exp1.clone(), after: exp2.clone() });
            source = addSymbolicTransformation(source, op)?;
            addSymbolicTransformationDeriveLst(source, rexplst1, rexplst2)?
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok(source)
}

pub fn addSymbolicTransformationFlattenedEqs(
    mut source: metamodelica::Ref<DAE::ElementSource>,
    mut elt: metamodelica::Ref<DAE::Element>,
) -> Result<metamodelica::Ref<DAE::ElementSource>> {
    let mut source: metamodelica::Ref<DAE::ElementSource> = source;
    if !(Flags::isSet(Flags::INFO_XML_OPERATIONS.clone())?) {
        return Ok(source);
    }
    source = (::match_deref::match_deref! { match &(source) {
        Deref @ DAE::ElementSource { info, partOfLst, instance: instanceOpt, connectEquationOptLst, typeLst, operations: Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::SymbolicOperation::FLATTEN { scode, dae: None }, tail: operations }, comment } => {
            metamodelica::Ref::new(DAE::ElementSource { info: info.clone(), partOfLst: partOfLst.clone(), instance: instanceOpt.clone(), connectEquationOptLst: connectEquationOptLst.clone(), typeLst: typeLst.clone(), operations: metamodelica::cons(metamodelica::Ref::new(DAE::SymbolicOperation::FLATTEN { scode: scode.clone(), dae: Some(elt) }), operations.clone()), comment: comment.clone() })
        },
        Deref @ DAE::ElementSource { info, .. } => {
            Error::addSourceMessage(&(Error::INTERNAL_ERROR.clone()), list![literal!("Tried to add the flattened elements to the list of operations, but did not find the SCode equation")], metamodelica::AsArg::as_arg(&info))?;
            return Err("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(source)
}

pub fn addSymbolicTransformationSubstitutionLst<'__b>(
    mut add: &'__b metamodelica::List<bool>,
    mut source: metamodelica::Ref<DAE::ElementSource>,
    mut explst1: &'__b metamodelica::List<metamodelica::Ref<DAE::Exp>>,
    mut explst2: &'__b metamodelica::List<metamodelica::Ref<DAE::Exp>>,
) -> Result<metamodelica::Ref<DAE::ElementSource>> {
    let mut source: metamodelica::Ref<DAE::ElementSource> = source;
    if !(Flags::isSet(Flags::INFO_XML_OPERATIONS.clone())?) {
        return Ok(source);
    }
    source = (::match_deref::match_deref! { match (add, explst1, explst2) {
        (Deref @ metamodelica::ListNode::Nil, _, _) => {
            source
        },
        (Deref @ metamodelica::ListNode::Cons { head: true, tail: brest }, Deref @ metamodelica::ListNode::Cons { head: exp1, tail: rexplst1 }, Deref @ metamodelica::ListNode::Cons { head: exp2, tail: rexplst2 }) => {
            source = addSymbolicTransformationSubstitution(true, source, exp1.clone(), exp2.clone())?;
            addSymbolicTransformationSubstitutionLst(brest, source, rexplst1, rexplst2)?
        },
        (Deref @ metamodelica::ListNode::Cons { head: false, tail: brest }, Deref @ metamodelica::ListNode::Cons { head: _, tail: rexplst1 }, Deref @ metamodelica::ListNode::Cons { head: _, tail: rexplst2 }) => {
            addSymbolicTransformationSubstitutionLst(brest, source, rexplst1, rexplst2)?
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok(source)
}

pub fn addSymbolicTransformationSubstitution(
    mut add: bool,
    mut source: metamodelica::Ref<DAE::ElementSource>,
    mut exp1: metamodelica::Ref<DAE::Exp>,
    mut exp2: metamodelica::Ref<DAE::Exp>,
) -> Result<metamodelica::Ref<DAE::ElementSource>> {
    let mut source: metamodelica::Ref<DAE::ElementSource> = source;
    if !(Flags::isSet(Flags::INFO_XML_OPERATIONS.clone())?) {
        return Ok(source);
    }
    source = condAddSymbolicTransformation(
        add,
        source,
        metamodelica::Ref::new(DAE::SymbolicOperation::SUBSTITUTION {
            substitutions: list![exp2],
            source: exp1,
        }),
    )?;
    Ok(source)
}

pub fn addSymbolicTransformationSimplifyLst<'__b>(
    mut add: &'__b metamodelica::List<bool>,
    mut source: metamodelica::Ref<DAE::ElementSource>,
    mut explst1: &'__b metamodelica::List<metamodelica::Ref<DAE::Exp>>,
    mut explst2: &'__b metamodelica::List<metamodelica::Ref<DAE::Exp>>,
) -> Result<metamodelica::Ref<DAE::ElementSource>> {
    let mut source: metamodelica::Ref<DAE::ElementSource> = source;
    if !(Flags::isSet(Flags::INFO_XML_OPERATIONS.clone())?) {
        return Ok(source);
    }
    source = (::match_deref::match_deref! { match (add, explst1, explst2) {
        (Deref @ metamodelica::ListNode::Nil, _, _) => {
            source
        },
        (Deref @ metamodelica::ListNode::Cons { head: true, tail: brest }, Deref @ metamodelica::ListNode::Cons { head: exp1, tail: rexplst1 }, Deref @ metamodelica::ListNode::Cons { head: exp2, tail: rexplst2 }) => {
            source = addSymbolicTransformation(source, metamodelica::Ref::new(DAE::SymbolicOperation::SIMPLIFY { before: metamodelica::Ref::new(DAE::EquationExp::PARTIAL_EQUATION { exp: exp1.clone() }), after: metamodelica::Ref::new(DAE::EquationExp::PARTIAL_EQUATION { exp: exp2.clone() }) }))?;
            addSymbolicTransformationSimplifyLst(brest, source, rexplst1, rexplst2)?
        },
        (Deref @ metamodelica::ListNode::Cons { head: false, tail: brest }, Deref @ metamodelica::ListNode::Cons { head: _, tail: rexplst1 }, Deref @ metamodelica::ListNode::Cons { head: _, tail: rexplst2 }) => {
            addSymbolicTransformationSimplifyLst(brest, source, rexplst1, rexplst2)?
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok(source)
}

pub fn addSymbolicTransformationSimplify(
    mut add: bool,
    mut source: metamodelica::Ref<DAE::ElementSource>,
    mut exp1: metamodelica::Ref<DAE::EquationExp>,
    mut exp2: metamodelica::Ref<DAE::EquationExp>,
) -> Result<metamodelica::Ref<DAE::ElementSource>> {
    let mut source: metamodelica::Ref<DAE::ElementSource> = source;
    if !(Flags::isSet(Flags::INFO_XML_OPERATIONS.clone())?) {
        return Ok(source);
    }
    source = condAddSymbolicTransformation(
        add,
        source,
        metamodelica::Ref::new(DAE::SymbolicOperation::SIMPLIFY {
            before: exp1,
            after: exp2,
        }),
    )?;
    Ok(source)
}

pub fn addSymbolicTransformationSolve(
    mut add: bool,
    mut source: metamodelica::Ref<DAE::ElementSource>,
    mut cr: metamodelica::Ref<DAE::ComponentRef>,
    mut exp1: metamodelica::Ref<DAE::Exp>,
    mut exp2: metamodelica::Ref<DAE::Exp>,
    mut exp: metamodelica::Ref<DAE::Exp>,
    mut asserts: metamodelica::List<metamodelica::Ref<DAE::Statement>>,
) -> Result<metamodelica::Ref<DAE::ElementSource>> {
    let mut source: metamodelica::Ref<DAE::ElementSource> = source;
    let mut op: metamodelica::Ref<DAE::SymbolicOperation>;
    let mut op1: metamodelica::Ref<DAE::SymbolicOperation>;
    let mut op2: metamodelica::Ref<DAE::SymbolicOperation>;
    if !(add && Flags::isSet(Flags::INFO_XML_OPERATIONS.clone())?) {
        return Ok(source);
    }
    op1 = metamodelica::Ref::new(DAE::SymbolicOperation::SOLVE {
        cr: cr.clone(),
        exp1: exp1,
        exp2: exp2.clone(),
        res: exp.clone(),
        assertConds: ({
            let mut __acc: metamodelica::List<metamodelica::Ref<DAE::Exp>> = metamodelica::nil();
            for mut ass in (asserts).into_iter().cloned() {
                let __x = getAssertCond(&(ass.clone()))?;
                __acc = cons(__x, __acc);
            }
            __acc.reverse()
        }),
    });
    op2 = metamodelica::Ref::new(DAE::SymbolicOperation::SOLVED {
        cr: cr,
        exp: exp2.clone(),
    });
    op = if (ExpressionBasics::expEqual(&exp2, exp)?) {
        op2
    } else {
        op1
    };
    source = addSymbolicTransformation(source, op)?;
    Ok(source)
}

pub(crate) fn getAssertCond(mut stmt: &metamodelica::Ref<DAE::Statement>) -> Result<metamodelica::Ref<DAE::Exp>> {
    let mut cond: metamodelica::Ref<DAE::Exp>;
    let __pa0 = ::match_deref::match_deref! { match &((*stmt)) {
        Deref @ DAE::Statement::STMT_ASSERT { cond: __pa0, .. } => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    cond = metamodelica::Own::own(__pa0);
    Ok(cond)
}

pub fn getSymbolicTransformations(
    mut source: &metamodelica::Ref<DAE::ElementSource>,
) -> metamodelica::List<metamodelica::Ref<DAE::SymbolicOperation>> {
    let mut ops: metamodelica::List<metamodelica::Ref<DAE::SymbolicOperation>>;
    ops = source.operations.clone();
    ops
}

pub fn getElementSource(
    mut element: &metamodelica::Ref<DAE::Element>,
) -> Result<metamodelica::Ref<DAE::ElementSource>> {
    let mut source: metamodelica::Ref<DAE::ElementSource>;
    source = (match &**element {
        DAE::Element::VAR {
            source: __element_source,
            ..
        } => __element_source.clone(),
        DAE::Element::DEFINE {
            source: __element_source,
            ..
        } => __element_source.clone(),
        DAE::Element::INITIALDEFINE {
            source: __element_source,
            ..
        } => __element_source.clone(),
        DAE::Element::EQUATION {
            source: __element_source,
            ..
        } => __element_source.clone(),
        DAE::Element::EQUEQUATION {
            source: __element_source,
            ..
        } => __element_source.clone(),
        DAE::Element::ARRAY_EQUATION {
            source: __element_source,
            ..
        } => __element_source.clone(),
        DAE::Element::INITIAL_ARRAY_EQUATION {
            source: __element_source,
            ..
        } => __element_source.clone(),
        DAE::Element::COMPLEX_EQUATION {
            source: __element_source,
            ..
        } => __element_source.clone(),
        DAE::Element::INITIAL_COMPLEX_EQUATION {
            source: __element_source,
            ..
        } => __element_source.clone(),
        DAE::Element::WHEN_EQUATION {
            source: __element_source,
            ..
        } => __element_source.clone(),
        DAE::Element::IF_EQUATION {
            source: __element_source,
            ..
        } => __element_source.clone(),
        DAE::Element::INITIAL_IF_EQUATION {
            source: __element_source,
            ..
        } => __element_source.clone(),
        DAE::Element::INITIALEQUATION {
            source: __element_source,
            ..
        } => __element_source.clone(),
        DAE::Element::ALGORITHM {
            source: __element_source,
            ..
        } => __element_source.clone(),
        DAE::Element::INITIALALGORITHM {
            source: __element_source,
            ..
        } => __element_source.clone(),
        DAE::Element::COMP {
            source: __element_source,
            ..
        } => __element_source.clone(),
        DAE::Element::EXTOBJECTCLASS {
            source: __element_source,
            ..
        } => __element_source.clone(),
        DAE::Element::ASSERT {
            source: __element_source,
            ..
        } => __element_source.clone(),
        DAE::Element::INITIAL_ASSERT {
            source: __element_source,
            ..
        } => __element_source.clone(),
        DAE::Element::TERMINATE {
            source: __element_source,
            ..
        } => __element_source.clone(),
        DAE::Element::INITIAL_TERMINATE {
            source: __element_source,
            ..
        } => __element_source.clone(),
        DAE::Element::REINIT {
            source: __element_source,
            ..
        } => __element_source.clone(),
        DAE::Element::NORETCALL {
            source: __element_source,
            ..
        } => __element_source.clone(),
        DAE::Element::CONSTRAINT {
            source: __element_source,
            ..
        } => __element_source.clone(),
        DAE::Element::INITIAL_NORETCALL {
            source: __element_source,
            ..
        } => __element_source.clone(),
        _ => {
            Error::addMessage(
                Error::INTERNAL_ERROR.clone(),
                list![literal!(
                    "ElementSource.getElementSource failed: Element does not have a source"
                )],
            )?;
            return Err("fail");
        }
    });
    Ok(source)
}

pub fn getStatementSource(
    mut stmt: &metamodelica::Ref<DAE::Statement>,
) -> Result<metamodelica::Ref<DAE::ElementSource>> {
    let mut source: metamodelica::Ref<DAE::ElementSource>;
    source = (match &**stmt {
        DAE::Statement::STMT_ASSIGN {
            source: __stmt_source, ..
        } => __stmt_source.clone(),
        DAE::Statement::STMT_TUPLE_ASSIGN {
            source: __stmt_source, ..
        } => __stmt_source.clone(),
        DAE::Statement::STMT_ASSIGN_ARR {
            source: __stmt_source, ..
        } => __stmt_source.clone(),
        DAE::Statement::STMT_IF {
            source: __stmt_source, ..
        } => __stmt_source.clone(),
        DAE::Statement::STMT_FOR {
            source: __stmt_source, ..
        } => __stmt_source.clone(),
        DAE::Statement::STMT_PARFOR {
            source: __stmt_source, ..
        } => __stmt_source.clone(),
        DAE::Statement::STMT_WHILE {
            source: __stmt_source, ..
        } => __stmt_source.clone(),
        DAE::Statement::STMT_WHEN {
            source: __stmt_source, ..
        } => __stmt_source.clone(),
        DAE::Statement::STMT_ASSERT {
            source: __stmt_source, ..
        } => __stmt_source.clone(),
        DAE::Statement::STMT_TERMINATE {
            source: __stmt_source, ..
        } => __stmt_source.clone(),
        DAE::Statement::STMT_REINIT {
            source: __stmt_source, ..
        } => __stmt_source.clone(),
        DAE::Statement::STMT_NORETCALL {
            source: __stmt_source, ..
        } => __stmt_source.clone(),
        DAE::Statement::STMT_RETURN { source: __stmt_source } => __stmt_source.clone(),
        DAE::Statement::STMT_BREAK { source: __stmt_source } => __stmt_source.clone(),
        DAE::Statement::STMT_ARRAY_INIT {
            source: __stmt_source, ..
        } => __stmt_source.clone(),
        DAE::Statement::STMT_FAILURE {
            source: __stmt_source, ..
        } => __stmt_source.clone(),
        _ => return Err("match: no arm matched"),
    });
    Ok(source)
}

pub use getElementSourceFileInfo as getInfo;

pub fn getElementSourceFileInfo(mut source: metamodelica::Ref<DAE::ElementSource>) -> SourceInfo {
    let mut info: SourceInfo;
    info = source.info.clone();
    info
}

pub fn getElementSourceTypes(
    mut source: &metamodelica::Ref<DAE::ElementSource>,
) -> metamodelica::List<metamodelica::Ref<Absyn::Path>> {
    let mut pathLst: metamodelica::List<metamodelica::Ref<Absyn::Path>>;
    pathLst = source.typeLst.clone();
    pathLst
}

pub(crate) fn getElementSourceInstances(
    mut source: &metamodelica::Ref<DAE::ElementSource>,
) -> metamodelica::Ref<DAE::ComponentPrefix> {
    let mut instanceOpt: metamodelica::Ref<DAE::ComponentPrefix>;
    instanceOpt = source.instance.clone();
    instanceOpt
}

pub(crate) fn getElementSourceConnects(
    mut source: &metamodelica::Ref<DAE::ElementSource>,
) -> metamodelica::List<(
    metamodelica::Ref<DAE::ComponentRef>,
    metamodelica::Ref<DAE::ComponentRef>,
)> {
    let mut connectEquationOptLst: metamodelica::List<(
        metamodelica::Ref<DAE::ComponentRef>,
        metamodelica::Ref<DAE::ComponentRef>,
    )>;
    connectEquationOptLst = source.connectEquationOptLst.clone();
    connectEquationOptLst
}

pub(crate) fn getElementSourcePartOfs(
    mut source: &metamodelica::Ref<DAE::ElementSource>,
) -> metamodelica::List<Absyn::Within> {
    let mut withinLst: metamodelica::List<Absyn::Within>;
    withinLst = source.partOfLst.clone();
    withinLst
}

pub(crate) fn addElementSourcePartOf(
    mut source: metamodelica::Ref<DAE::ElementSource>,
    mut withinPath: Absyn::Within,
) -> Result<metamodelica::Ref<DAE::ElementSource>> {
    let mut source: metamodelica::Ref<DAE::ElementSource> = source;
    if !(Flags::isSet(Flags::INFO_XML_OPERATIONS.clone())? || Flags::isSet(Flags::VISUAL_XML.clone())?) {
        return Ok(source);
    }
    assign_field!(source.partOfLst = metamodelica::cons(withinPath, source.partOfLst.clone()));
    Ok(source)
}

pub fn addElementSourcePartOfOpt(
    mut source: metamodelica::Ref<DAE::ElementSource>,
    mut classPathOpt: Option<metamodelica::Ref<Absyn::Path>>,
) -> Result<metamodelica::Ref<DAE::ElementSource>> {
    let mut source: metamodelica::Ref<DAE::ElementSource> = source;
    if !(Flags::isSet(Flags::INFO_XML_OPERATIONS.clone())? || Flags::isSet(Flags::VISUAL_XML.clone())?) {
        return Ok(source);
    }
    source = (::match_deref::match_deref! { match &(classPathOpt) {
        None => {
            source
        },
        Some(classPath) => {
            addElementSourcePartOf(source, Absyn::Within::WITHIN { path: classPath.clone() })?
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(source)
}

pub fn addElementSourceFileInfo(
    mut source: metamodelica::Ref<DAE::ElementSource>,
    mut fileInfo: SourceInfo,
) -> metamodelica::Ref<DAE::ElementSource> {
    let mut outSource: metamodelica::Ref<DAE::ElementSource> = source;
    assign_field!(outSource.info = fileInfo);
    outSource
}

pub fn addElementSourceConnect(
    mut inSource: &metamodelica::Ref<DAE::ElementSource>,
    mut connectEquationOpt: (
        metamodelica::Ref<DAE::ComponentRef>,
        metamodelica::Ref<DAE::ComponentRef>,
    ),
) -> metamodelica::Ref<DAE::ElementSource> {
    let mut outSource: metamodelica::Ref<DAE::ElementSource>;
    outSource = (match &**inSource {
        DAE::ElementSource {
            info,
            partOfLst,
            instance: instanceOpt,
            connectEquationOptLst,
            typeLst,
            operations,
            comment,
        } => metamodelica::Ref::new(DAE::ElementSource {
            info: info.clone(),
            partOfLst: partOfLst.clone(),
            instance: instanceOpt.clone(),
            connectEquationOptLst: metamodelica::cons(connectEquationOpt, connectEquationOptLst.clone()),
            typeLst: typeLst.clone(),
            operations: operations.clone(),
            comment: comment.clone(),
        }),
    });
    outSource
}

pub fn addElementSourceType(
    mut source: metamodelica::Ref<DAE::ElementSource>,
    mut classPath: metamodelica::Ref<Absyn::Path>,
) -> Result<metamodelica::Ref<DAE::ElementSource>> {
    let mut source: metamodelica::Ref<DAE::ElementSource> = source;
    if !(Flags::isSet(Flags::INFO_XML_OPERATIONS.clone())? || Flags::isSet(Flags::VISUAL_XML.clone())?) {
        return Ok(source);
    }
    source = (match &*source {
        DAE::ElementSource {
            info,
            partOfLst,
            instance: instanceOpt,
            connectEquationOptLst,
            typeLst,
            operations,
            comment,
        } => metamodelica::Ref::new(DAE::ElementSource {
            info: info.clone(),
            partOfLst: partOfLst.clone(),
            instance: instanceOpt.clone(),
            connectEquationOptLst: connectEquationOptLst.clone(),
            typeLst: metamodelica::cons(classPath, typeLst.clone()),
            operations: operations.clone(),
            comment: comment.clone(),
        }),
    });
    Ok(source)
}

pub fn addElementSourceInstanceOpt(
    mut source: metamodelica::Ref<DAE::ElementSource>,
    mut instanceOpt: metamodelica::Ref<DAE::ComponentPrefix>,
) -> Result<metamodelica::Ref<DAE::ElementSource>> {
    let mut source: metamodelica::Ref<DAE::ElementSource> = source;
    let () = (::match_deref::match_deref! { match &((source.clone(), instanceOpt.clone())) {
        (_, Deref @ DAE::ComponentPrefix::NOCOMPPRE { .. }) => (),
        (Deref @ DAE::ElementSource { .. }, _) => {
            assign_field!(source.instance = instanceOpt);
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(source)
}
