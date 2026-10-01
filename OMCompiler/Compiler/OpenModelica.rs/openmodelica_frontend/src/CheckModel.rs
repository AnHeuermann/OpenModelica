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

use crate::HashSet;
use crate::PrefixUtil;
use openmodelica_ast::Absyn;
use openmodelica_frontend_base::ComponentReference;
use openmodelica_frontend_base::DAEDump;
use openmodelica_frontend_base::DAEUtil;
use openmodelica_frontend_base::Expression;
use openmodelica_frontend_dump::ComponentReferenceBasics;
use openmodelica_frontend_types::ClassInf;
use openmodelica_frontend_types::DAE;
use openmodelica_util::BaseHashSet;
use openmodelica_util::Debug;
use openmodelica_util::Error;
use openmodelica_util::ExecStat;
use openmodelica_util::Flags;
use openmodelica_util::Util;
use openmodelica_util_datatypes_basic::List;

pub fn checkModel(mut inDAELst: DAE::DAElist) -> Result<(i32, i32, i32)> {
    let mut varSize: i32;
    let mut eqnSize: i32;
    let mut simpleEqnSize: i32;
    let mut eqns: metamodelica::List<metamodelica::Ref<DAE::Element>>;
    let mut lst: metamodelica::List<metamodelica::Ref<DAE::Element>>;
    let mut hs: (
        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<Option<metamodelica::Ref<DAE::ComponentRef>>>,
        ),
        i32,
        i32,
        (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr),
    );
    let mut debug_dump: bool = Flags::isSet(Flags::DUMP_CHECK_MODEL.clone())?;
    let mut arg: CountVarEqnFoldArg;
    ExecStat::execStat(&(literal!("CheckModel - start counting")))?;
    let DAE::DAE { elementLst: __pa0 } = inDAELst;
    lst = metamodelica::Own::own(__pa0);
    hs = HashSet::emptyHashSet();
    (varSize, eqnSize, eqns, hs) = countVarEqnSizeList(&lst, (0, 0, metamodelica::nil(), hs), debug_dump)?;
    simpleEqnSize = countSimpleEqnSize(&eqns, 0, hs)?;
    ExecStat::execStat(&(literal!("CheckModel - end counting")))?;
    Ok((varSize, eqnSize, simpleEqnSize))
}

pub type CountVarEqnFoldArg = (
    i32,
    i32,
    metamodelica::List<metamodelica::Ref<DAE::Element>>,
    (
        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<Option<metamodelica::Ref<DAE::ComponentRef>>>,
        ),
        i32,
        i32,
        (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr),
    ),
);

/*varSize*/
/*eqnSize*/
/*eqns*/
/*vars*/
fn countVarEqnSizeList(
    mut elements: &metamodelica::List<metamodelica::Ref<DAE::Element>>,
    mut arg: CountVarEqnFoldArg,
    mut debugDump: bool,
) -> Result<CountVarEqnFoldArg> {
    let mut arg: CountVarEqnFoldArg = arg;
    for mut e in &**elements {
        arg = countVarEqnSize(e.clone(), arg, debugDump)?;
    }
    Ok(arg)
}

fn countVarEqnSize(
    mut element: metamodelica::Ref<DAE::Element>,
    mut inArg: CountVarEqnFoldArg,
    mut debugDump: bool,
) -> Result<CountVarEqnFoldArg> {
    let mut outArg: CountVarEqnFoldArg;
    outArg = (::match_deref::match_deref! { match &(element.clone()) {
        Deref @ DAE::Element::EXTOBJECTCLASS { .. } => {
            inArg
        },
        Deref @ DAE::Element::VAR { ty: Deref @ DAE::Type::T_COMPLEX { complexClassType: ClassInf::State::EXTERNAL_OBJ { .. }, .. }, .. } => {
            inArg
        },
        Deref @ DAE::Element::VAR { kind: DAE::VarKind::PARAM { .. }, .. } => {
            inArg
        },
        Deref @ DAE::Element::VAR { kind: DAE::VarKind::CONST { .. }, .. } => {
            inArg
        },
        Deref @ DAE::Element::VAR { componentRef: cr, binding: __element_binding, source: __element_source, ty: __element_ty, .. } => {
            let mut varSize: i32;
            let mut eqnSize: i32;
            let mut size: i32;
            let mut eqns: metamodelica::List<metamodelica::Ref<DAE::Element>>;
            let mut hs: (metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>, (i32, i32, metamodelica::Array<Option<metamodelica::Ref<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr));
            let mut elem: metamodelica::Ref<DAE::Element>;
            (varSize, eqnSize, eqns, hs) = inArg;
            size = Expression::sizeOf(metamodelica::AsArg::as_arg(&__element_ty));
            varSize = varSize + size;
            dumpVar(metamodelica::AsArg::as_arg(&cr), size, debugDump)?;
            if DAEUtil::isInput(&element) && DAEUtil::isPublicVar(&element) {
                eqnSize = eqnSize + size;
                dumpEqn(&element, size, debugDump);
            } else {
                if (__element_binding).is_some() {
                    eqnSize = eqnSize + size;
                    elem = metamodelica::Ref::new(DAE::Element::EQUATION { exp: Expression::crefExp(var_field!((*element).componentRef, DAE::Element::VAR).clone())?, scalar: Util::getOption(__element_binding.clone())?, source: __element_source.clone() });
                    dumpEqn(&elem, size, debugDump);
                    eqns = metamodelica::cons(elem, eqns);
                }
                hs = BaseHashSet::add(cr.clone(), &hs)?;
            }
            (varSize, eqnSize, eqns, hs)
        },
        Deref @ DAE::Element::EQUATION { exp: e, .. } => {
            let mut varSize: i32;
            let mut eqnSize: i32;
            let mut size: i32;
            let mut eqns: metamodelica::List<metamodelica::Ref<DAE::Element>>;
            let mut hs: (metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>, (i32, i32, metamodelica::Array<Option<metamodelica::Ref<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr));
            (varSize, eqnSize, eqns, hs) = inArg;
            size = Expression::sizeOf(&(Expression::r#typeof(e.clone())?));
            dumpEqn(&element, size, debugDump);
            (varSize, eqnSize + size, metamodelica::cons(element, eqns), hs)
        },
        Deref @ DAE::Element::INITIALEQUATION { .. } => {
            inArg
        },
        Deref @ DAE::Element::EQUEQUATION { cr1: cr, .. } => {
            let mut varSize: i32;
            let mut eqnSize: i32;
            let mut size: i32;
            let mut eqns: metamodelica::List<metamodelica::Ref<DAE::Element>>;
            let mut hs: (metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>, (i32, i32, metamodelica::Array<Option<metamodelica::Ref<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr));
            let mut tp: metamodelica::Ref<DAE::Type>;
            (varSize, eqnSize, eqns, hs) = inArg;
            tp = ComponentReference::crefTypeConsiderSubs(metamodelica::AsArg::as_arg(&cr))?;
            size = Expression::sizeOf(&tp);
            dumpEqn(&element, size, debugDump);
            (varSize, eqnSize + size, metamodelica::cons(element, eqns), hs)
        },
        Deref @ DAE::Element::DEFINE { componentRef: cr, .. } => {
            let mut varSize: i32;
            let mut eqnSize: i32;
            let mut size: i32;
            let mut eqns: metamodelica::List<metamodelica::Ref<DAE::Element>>;
            let mut hs: (metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>, (i32, i32, metamodelica::Array<Option<metamodelica::Ref<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr));
            let mut tp: metamodelica::Ref<DAE::Type>;
            (varSize, eqnSize, eqns, hs) = inArg;
            tp = ComponentReference::crefTypeConsiderSubs(metamodelica::AsArg::as_arg(&cr))?;
            size = Expression::sizeOf(&tp);
            dumpEqn(&element, size, debugDump);
            (varSize, eqnSize + size, metamodelica::cons(element, eqns), hs)
        },
        Deref @ DAE::Element::COMPLEX_EQUATION { lhs: e, .. } => {
            let mut varSize: i32;
            let mut eqnSize: i32;
            let mut size: i32;
            let mut eqns: metamodelica::List<metamodelica::Ref<DAE::Element>>;
            let mut hs: (metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>, (i32, i32, metamodelica::Array<Option<metamodelica::Ref<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr));
            (varSize, eqnSize, eqns, hs) = inArg;
            size = Expression::sizeOf(&(Expression::r#typeof(e.clone())?));
            dumpEqn(&element, size, debugDump);
            (varSize, eqnSize + size, metamodelica::cons(element, eqns), hs)
        },
        Deref @ DAE::Element::INITIAL_COMPLEX_EQUATION { .. } => {
            inArg
        },
        Deref @ DAE::Element::ARRAY_EQUATION { exp: __element_exp, .. } => {
            let mut varSize: i32;
            let mut eqnSize: i32;
            let mut size: i32;
            let mut eqns: metamodelica::List<metamodelica::Ref<DAE::Element>>;
            let mut hs: (metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>, (i32, i32, metamodelica::Array<Option<metamodelica::Ref<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr));
            (varSize, eqnSize, eqns, hs) = inArg;
            size = Expression::sizeOf(&(Expression::r#typeof(__element_exp.clone())?));
            dumpEqn(&element, size, debugDump);
            (varSize, eqnSize + size, metamodelica::cons(element, eqns), hs)
        },
        Deref @ DAE::Element::INITIAL_ARRAY_EQUATION { .. } => {
            inArg
        },
        Deref @ DAE::Element::WHEN_EQUATION { equations: daeElts, .. } => {
            let mut varSize: i32;
            let mut eqnSize: i32;
            let mut size: i32;
            let mut eqns: metamodelica::List<metamodelica::Ref<DAE::Element>>;
            let mut hs: (metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>, (i32, i32, metamodelica::Array<Option<metamodelica::Ref<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr));
            (varSize, eqnSize, eqns, hs) = inArg;
            (_, size, _, _) = countVarEqnSizeList(metamodelica::AsArg::as_arg(&daeElts), (0, 0, metamodelica::nil(), hs.clone()), false)?;
            dumpEqn(&element, size, debugDump);
            (varSize, eqnSize + size, eqns, hs)
        },
        Deref @ DAE::Element::INITIAL_FOR_EQUATION { equations: __element_equations, range: __element_range, .. } => {
            let mut varSize: i32;
            let mut eqnSize: i32;
            let mut size: i32;
            let mut eqns: metamodelica::List<metamodelica::Ref<DAE::Element>>;
            let mut hs: (metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>, (i32, i32, metamodelica::Array<Option<metamodelica::Ref<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr));
            (varSize, eqnSize, eqns, hs) = inArg;
            (_, size, _, _) = countVarEqnSizeList(metamodelica::AsArg::as_arg(&__element_equations), (0, 0, metamodelica::nil(), hs.clone()), false)?;
            size = size * Expression::sizeOf(&(Expression::r#typeof(__element_range.clone())?));
            dumpEqn(&element, size, debugDump);
            (varSize, eqnSize + size, eqns, hs)
        },
        Deref @ DAE::Element::FOR_EQUATION { equations: __element_equations, range: __element_range, .. } => {
            let mut varSize: i32;
            let mut eqnSize: i32;
            let mut size: i32;
            let mut eqns: metamodelica::List<metamodelica::Ref<DAE::Element>>;
            let mut hs: (metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>, (i32, i32, metamodelica::Array<Option<metamodelica::Ref<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr));
            (varSize, eqnSize, eqns, hs) = inArg;
            (_, size, _, _) = countVarEqnSizeList(metamodelica::AsArg::as_arg(&__element_equations), (0, 0, metamodelica::nil(), hs.clone()), false)?;
            size = size * Expression::sizeOf(&(Expression::r#typeof(__element_range.clone())?));
            dumpEqn(&element, size, debugDump);
            (varSize, eqnSize + size, eqns, hs)
        },
        Deref @ DAE::Element::IF_EQUATION { condition1: Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Exp::BCONST { bool: false }, tail: Deref @ metamodelica::ListNode::Nil }, equations3: Deref @ metamodelica::ListNode::Nil, .. } => {
            inArg
        },
        Deref @ DAE::Element::IF_EQUATION { equations2: Deref @ metamodelica::ListNode::Cons { head: daeElts, tail: _ }, .. } => {
            let mut varSize: i32;
            let mut eqnSize: i32;
            let mut size: i32;
            let mut eqns: metamodelica::List<metamodelica::Ref<DAE::Element>>;
            let mut hs: (metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>, (i32, i32, metamodelica::Array<Option<metamodelica::Ref<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr));
            (varSize, eqnSize, eqns, hs) = inArg;
            (_, size, _, _) = countVarEqnSizeList(metamodelica::AsArg::as_arg(&daeElts), (0, 0, metamodelica::nil(), hs.clone()), false)?;
            dumpEqn(&element, size, debugDump);
            (varSize, eqnSize + size, eqns, hs)
        },
        Deref @ DAE::Element::INITIAL_IF_EQUATION { condition1: Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Exp::BCONST { bool: false }, tail: Deref @ metamodelica::ListNode::Nil }, equations3: Deref @ metamodelica::ListNode::Nil, .. } => {
            inArg
        },
        Deref @ DAE::Element::INITIAL_IF_EQUATION { .. } => {
            inArg
        },
        Deref @ DAE::Element::ALGORITHM { algorithm_: alg, source } => {
            let mut varSize: i32;
            let mut eqnSize: i32;
            let mut size: i32;
            let mut eqns: metamodelica::List<metamodelica::Ref<DAE::Element>>;
            let mut hs: (metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>, (i32, i32, metamodelica::Array<Option<metamodelica::Ref<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr));
            let mut crlst: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
            (varSize, eqnSize, eqns, hs) = inArg;
            crlst = checkAndGetAlgorithmOutputs(metamodelica::AsArg::as_arg(&alg), metamodelica::AsArg::as_arg(&source), openmodelica_frontend_types::DAE::Expand::EXPAND)?;
            size = ((crlst).len() as i32);
            dumpEqn(&element, size, debugDump);
            (varSize, eqnSize + size, eqns, hs)
        },
        Deref @ DAE::Element::INITIALALGORITHM { .. } => {
            inArg
        },
        Deref @ DAE::Element::COMP { dAElist: daeElts, .. } => {
            countVarEqnSizeList(metamodelica::AsArg::as_arg(&daeElts), inArg, debugDump)?
        },
        Deref @ DAE::Element::REINIT { .. } => {
            inArg
        },
        Deref @ DAE::Element::ASSERT { .. } => {
            inArg
        },
        Deref @ DAE::Element::INITIAL_ASSERT { .. } => {
            inArg
        },
        Deref @ DAE::Element::TERMINATE { .. } => {
            inArg
        },
        Deref @ DAE::Element::INITIAL_TERMINATE { .. } => {
            inArg
        },
        Deref @ DAE::Element::NORETCALL { .. } => {
            inArg
        },
        Deref @ DAE::Element::INITIAL_NORETCALL { .. } => {
            inArg
        },
        Deref @ DAE::Element::CONSTRAINT { .. } => {
            inArg
        },
        Deref @ DAE::Element::FLAT_SM { dAElist: daeElts, .. } => {
            countVarEqnSizeList(metamodelica::AsArg::as_arg(&daeElts), inArg, debugDump)?
        },
        Deref @ DAE::Element::SM_COMP { dAElist: daeElts, .. } => {
            countVarEqnSizeList(metamodelica::AsArg::as_arg(&daeElts), inArg, debugDump)?
        },
        _ => {
            let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { return Err("pattern mismatch") };
            Debug::traceln({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("- CheckModel.countVarEqnSize failed on: ")); __mm_s.push_str(&*DAEDump::dumpElementsStr(&(list![element]))?); ArcStr::from(__mm_s) })?;
            return Err("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outArg)
}

fn dumpVar(mut cref: &metamodelica::Ref<DAE::ComponentRef>, mut size: i32, mut dump: bool) -> Result<()> {
    if dump {
        metamodelica::print({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("[var: "));
            __mm_s.push_str(&*ArcStr::from(::std::format!("{}", size)));
            __mm_s.push_str(&*literal!("] "));
            __mm_s.push_str(&*ComponentReferenceBasics::printComponentRefStr(cref)?);
            __mm_s.push_str(&*literal!("\n"));
            ArcStr::from(__mm_s)
        });
    }
    Ok(())
}

fn dumpEqn(mut eqn: &metamodelica::Ref<DAE::Element>, mut size: i32, mut dump: bool) -> () {
    if dump {
        metamodelica::print({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("[eqn: "));
            __mm_s.push_str(&*ArcStr::from(::std::format!("{}", size)));
            __mm_s.push_str(&*literal!("] "));
            __mm_s.push_str(&*DAEDump::dumpEquationStr(eqn));
            ArcStr::from(__mm_s)
        });
    }
    ()
}

pub fn checkAndGetAlgorithmOutputs(
    mut inAlgorithm: &metamodelica::Ref<DAE::Algorithm>,
    mut inSource: &metamodelica::Ref<DAE::ElementSource>,
    mut inCrefExpansionRule: DAE::Expand,
) -> Result<metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>> {
    let mut outCrefLst: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
    outCrefLst = 'mc: {
        let __mc_input = &**inSource;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::ElementSource { instance: Deref @ DAE::ComponentPrefix::NOCOMPPRE { .. }, .. } => {
                    Ok(algorithmOutputs(inAlgorithm, inCrefExpansionRule)?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::ElementSource { .. } => {
                    Ok(if (PrefixUtil::hasSubs(&inSource.instance)) {algorithmOutputs(inAlgorithm, openmodelica_frontend_types::DAE::Expand::NOT_EXPAND)?} else {algorithmOutputs(inAlgorithm, inCrefExpansionRule)?})
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Error::addMessage(Error::INTERNAL_ERROR.clone(), list![literal!("checkAndGetAlgorithmOutputs failed.")])?;
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(outCrefLst)
}

pub fn isCrefListAlgorithmOutput(
    mut crefList: &metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
    mut inAlgorithm: &metamodelica::Ref<DAE::Algorithm>,
    mut inSource: &metamodelica::Ref<DAE::ElementSource>,
    mut inCrefExpansionRule: DAE::Expand,
) -> Result<bool> {
    let mut outResult: bool = false;
    let mut ht: (
        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<Option<metamodelica::Ref<DAE::ComponentRef>>>,
        ),
        i32,
        i32,
        (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr),
    ) = HashSet::emptyHashSet();
    let mut algOutCrefs: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
    algOutCrefs = checkAndGetAlgorithmOutputs(inAlgorithm, inSource, inCrefExpansionRule)?;
    ht = List::fold(&algOutCrefs, &move |__a0: _, __a1: _| BaseHashSet::add(__a0, &__a1), ht)?;
    for mut cr in &**crefList {
        if !(BaseHashSet::has(cr.clone(), &ht)?) {
            return Ok(outResult);
        }
    }
    outResult = true;
    Ok(outResult)
}

fn algorithmOutputs(
    mut inAlgorithm: &metamodelica::Ref<DAE::Algorithm>,
    mut inCrefExpansion: DAE::Expand,
) -> Result<metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>> {
    let mut outCrefLst: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
    let mut stmts: metamodelica::List<metamodelica::Ref<DAE::Statement>>;
    let __arc1 = &(*inAlgorithm);
    let DAE::ALGORITHM_STMTS { statementLst: __pa0 } = &**__arc1;
    stmts = metamodelica::Own::own(__pa0);
    outCrefLst = algorithmStatementListOutputs(&stmts, inCrefExpansion)?;
    Ok(outCrefLst)
}

pub fn algorithmStatementListOutputs(
    mut inStmts: &metamodelica::List<metamodelica::Ref<DAE::Statement>>,
    mut inCrefExpansion: DAE::Expand,
) -> Result<metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>> {
    let mut outCrefLst: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
    let mut hs: (
        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<Option<metamodelica::Ref<DAE::ComponentRef>>>,
        ),
        i32,
        i32,
        (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr),
    );
    hs = HashSet::emptyHashSet();
    hs = List::fold1(
        inStmts,
        &move |__a0: metamodelica::Ref<DAE::Statement>,
               __a1: DAE::Expand,
               __a2: (
            metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
            (
                i32,
                i32,
                metamodelica::Array<Option<metamodelica::Ref<DAE::ComponentRef>>>,
            ),
            i32,
            i32,
            (
                Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>) -> Result<i32> + 'static>,
                Arc<
                    dyn ::std::ops::Fn(
                            metamodelica::Ref<DAE::ComponentRef>,
                            metamodelica::Ref<DAE::ComponentRef>,
                        ) -> Result<bool>
                        + 'static,
                >,
                Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>) -> Result<ArcStr> + 'static>,
            ),
        )| statementOutputs(&__a0, __a1, &__a2),
        inCrefExpansion,
        hs,
    )?;
    outCrefLst = BaseHashSet::hashSetList(&hs)?;
    Ok(outCrefLst)
}

fn statementOutputs(
    mut inStatement: &metamodelica::Ref<DAE::Statement>,
    mut inCrefExpansion: DAE::Expand,
    mut iht: &(
        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<Option<metamodelica::Ref<DAE::ComponentRef>>>,
        ),
        i32,
        i32,
        (
            Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>) -> Result<i32> + 'static>,
            Arc<
                dyn ::std::ops::Fn(
                        metamodelica::Ref<DAE::ComponentRef>,
                        metamodelica::Ref<DAE::ComponentRef>,
                    ) -> Result<bool>
                    + 'static,
            >,
            Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>) -> Result<ArcStr> + 'static>,
        ),
    ),
) -> Result<(
    metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
    (
        i32,
        i32,
        metamodelica::Array<Option<metamodelica::Ref<DAE::ComponentRef>>>,
    ),
    i32,
    i32,
    (
        Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>) -> Result<i32> + 'static>,
        Arc<
            dyn ::std::ops::Fn(
                    metamodelica::Ref<DAE::ComponentRef>,
                    metamodelica::Ref<DAE::ComponentRef>,
                ) -> Result<bool>
                + 'static,
        >,
        Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>) -> Result<ArcStr> + 'static>,
    ),
)> {
    let mut oht: (
        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<Option<metamodelica::Ref<DAE::ComponentRef>>>,
        ),
        i32,
        i32,
        (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr),
    );
    oht = 'mc: {
        let __mc_input = &**inStatement;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Statement::STMT_ASSIGN { exp1, .. } => {
                    let mut ht: (metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>, (i32, i32, metamodelica::Array<Option<metamodelica::Ref<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr));
                    let (_, (_, __pa0)) = Expression::traverseExpTopDown(exp1.clone(), &fnptr!(statementOutputsCrefFinder, metamodelica::Ref<DAE::Exp>, (DAE::Expand, (metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>, (i32, i32, metamodelica::Array<Option<metamodelica::Ref<DAE::ComponentRef>>>), i32, i32, (Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>, metamodelica::Ref<DAE::ComponentRef>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>) -> Result<ArcStr> + 'static>)))), (inCrefExpansion, iht.clone()))?;
                    ht = metamodelica::Own::own(__pa0);
                    Ok(ht.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Statement::STMT_TUPLE_ASSIGN { expExpLst: expl, .. } => {
                    let mut ht: (metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>, (i32, i32, metamodelica::Array<Option<metamodelica::Ref<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr));
                    let (_, (_, __pa0)) = Expression::traverseExpListTopDown(expl.clone(), &fnptr!(statementOutputsCrefFinder, metamodelica::Ref<DAE::Exp>, (DAE::Expand, (metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>, (i32, i32, metamodelica::Array<Option<metamodelica::Ref<DAE::ComponentRef>>>), i32, i32, (Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>, metamodelica::Ref<DAE::ComponentRef>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>) -> Result<ArcStr> + 'static>)))), (inCrefExpansion, iht.clone()))?;
                    ht = metamodelica::Own::own(__pa0);
                    Ok(ht.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Statement::STMT_ASSIGN_ARR { lhs: exp1, .. } => {
                    let mut ht: (metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>, (i32, i32, metamodelica::Array<Option<metamodelica::Ref<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr));
                    let mut cr: metamodelica::Ref<DAE::ComponentRef>;
                    let mut crlst: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
                    let mut subs: metamodelica::List<metamodelica::Ref<DAE::Subscript>>;
                    cr = Expression::expCref(metamodelica::AsArg::as_arg(&exp1))?;
                    subs = ComponentReference::crefLastSubs(&cr)?;
                    if !((subs).is_empty()) {
                        subs = List::fill(openmodelica_frontend_types::DAE::Subscript::interned_WHOLEDIM(), ((subs).len() as i32));
                        cr = ComponentReference::crefSetLastSubs(&cr, &subs)?;
                    }
                    crlst = ComponentReference::expandCref(&cr, true)?;
                    ht = List::fold(&crlst, &move |__a0: _, __a1: _| BaseHashSet::add(__a0, &__a1), iht.clone())?;
                    Ok(ht.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Statement::STMT_IF { statementLst: stmts, else_: elsebranch, .. } => {
                    let mut ht: (metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>, (i32, i32, metamodelica::Array<Option<metamodelica::Ref<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr));
                    ht = List::fold1(metamodelica::AsArg::as_arg(&stmts), &move |__a0: metamodelica::Ref<DAE::Statement>, __a1: DAE::Expand, __a2: (metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>, (i32, i32, metamodelica::Array<Option<metamodelica::Ref<DAE::ComponentRef>>>), i32, i32, (Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>, metamodelica::Ref<DAE::ComponentRef>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>) -> Result<ArcStr> + 'static>))| statementOutputs(&__a0, __a1, &__a2), inCrefExpansion, iht.clone())?;
                    ht = statementElseOutputs(metamodelica::AsArg::as_arg(&elsebranch), inCrefExpansion, ht.clone())?;
                    Ok(ht.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Statement::STMT_FOR { type_: tp, iter: iteratorName, range: e, statementLst: stmts, .. } => {
                    let mut ht: (metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>, (i32, i32, metamodelica::Array<Option<metamodelica::Ref<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr));
                    let mut cr: metamodelica::Ref<DAE::ComponentRef>;
                    let mut stmts = (*stmts).clone();
                    cr = ComponentReferenceBasics::makeCrefIdent(iteratorName.clone(), tp.clone(), metamodelica::nil());
                    (stmts, _) = DAEUtil::traverseDAEEquationsStmts(stmts.clone(), (std::sync::Arc::new(Expression::traverseSubexpressionsHelper) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>, _) -> Result<_> + 'static>), ((std::sync::Arc::new(Expression::replaceCref) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>, (metamodelica::Ref<DAE::ComponentRef>, metamodelica::Ref<DAE::Exp>)) -> Result<(metamodelica::Ref<DAE::Exp>, (metamodelica::Ref<DAE::ComponentRef>, metamodelica::Ref<DAE::Exp>))> + 'static>), (cr.clone(), e.clone())))?;
                    ht = List::fold1(metamodelica::AsArg::as_arg(&stmts), &move |__a0: metamodelica::Ref<DAE::Statement>, __a1: DAE::Expand, __a2: (metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>, (i32, i32, metamodelica::Array<Option<metamodelica::Ref<DAE::ComponentRef>>>), i32, i32, (Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>, metamodelica::Ref<DAE::ComponentRef>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>) -> Result<ArcStr> + 'static>))| statementOutputs(&__a0, __a1, &__a2), openmodelica_frontend_types::DAE::Expand::EXPAND, iht.clone())?;
                    Ok(ht.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Statement::STMT_PARFOR { type_: tp, iter: iteratorName, range: e, statementLst: stmts, .. } => {
                    let mut ht: (metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>, (i32, i32, metamodelica::Array<Option<metamodelica::Ref<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr));
                    let mut cr: metamodelica::Ref<DAE::ComponentRef>;
                    let mut stmts = (*stmts).clone();
                    cr = ComponentReferenceBasics::makeCrefIdent(iteratorName.clone(), tp.clone(), metamodelica::nil());
                    (stmts, _) = DAEUtil::traverseDAEEquationsStmts(stmts.clone(), (std::sync::Arc::new(Expression::traverseSubexpressionsHelper) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>, _) -> Result<_> + 'static>), ((std::sync::Arc::new(Expression::replaceCref) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>, (metamodelica::Ref<DAE::ComponentRef>, metamodelica::Ref<DAE::Exp>)) -> Result<(metamodelica::Ref<DAE::Exp>, (metamodelica::Ref<DAE::ComponentRef>, metamodelica::Ref<DAE::Exp>))> + 'static>), (cr.clone(), e.clone())))?;
                    ht = List::fold1(metamodelica::AsArg::as_arg(&stmts), &move |__a0: metamodelica::Ref<DAE::Statement>, __a1: DAE::Expand, __a2: (metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>, (i32, i32, metamodelica::Array<Option<metamodelica::Ref<DAE::ComponentRef>>>), i32, i32, (Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>, metamodelica::Ref<DAE::ComponentRef>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>) -> Result<ArcStr> + 'static>))| statementOutputs(&__a0, __a1, &__a2), openmodelica_frontend_types::DAE::Expand::EXPAND, iht.clone())?;
                    Ok(ht.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Statement::STMT_WHILE { statementLst: stmts, .. } => {
                    let mut ht: (metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>, (i32, i32, metamodelica::Array<Option<metamodelica::Ref<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr));
                    ht = List::fold1(metamodelica::AsArg::as_arg(&stmts), &move |__a0: metamodelica::Ref<DAE::Statement>, __a1: DAE::Expand, __a2: (metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>, (i32, i32, metamodelica::Array<Option<metamodelica::Ref<DAE::ComponentRef>>>), i32, i32, (Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>, metamodelica::Ref<DAE::ComponentRef>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>) -> Result<ArcStr> + 'static>))| statementOutputs(&__a0, __a1, &__a2), inCrefExpansion, iht.clone())?;
                    Ok(ht.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Statement::STMT_WHEN { statementLst: stmts, elseWhen: None, .. } => {
                    let mut ht: (metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>, (i32, i32, metamodelica::Array<Option<metamodelica::Ref<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr));
                    ht = List::fold1(metamodelica::AsArg::as_arg(&stmts), &move |__a0: metamodelica::Ref<DAE::Statement>, __a1: DAE::Expand, __a2: (metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>, (i32, i32, metamodelica::Array<Option<metamodelica::Ref<DAE::ComponentRef>>>), i32, i32, (Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>, metamodelica::Ref<DAE::ComponentRef>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>) -> Result<ArcStr> + 'static>))| statementOutputs(&__a0, __a1, &__a2), inCrefExpansion, iht.clone())?;
                    Ok(ht.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Statement::STMT_WHEN { statementLst: stmts, elseWhen: Some(stmt), .. } => {
                    let mut ht: (metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>, (i32, i32, metamodelica::Array<Option<metamodelica::Ref<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr));
                    ht = List::fold1(metamodelica::AsArg::as_arg(&stmts), &move |__a0: metamodelica::Ref<DAE::Statement>, __a1: DAE::Expand, __a2: (metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>, (i32, i32, metamodelica::Array<Option<metamodelica::Ref<DAE::ComponentRef>>>), i32, i32, (Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>, metamodelica::Ref<DAE::ComponentRef>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>) -> Result<ArcStr> + 'static>))| statementOutputs(&__a0, __a1, &__a2), inCrefExpansion, iht.clone())?;
                    ht = statementOutputs(metamodelica::AsArg::as_arg(&stmt), inCrefExpansion, &ht)?;
                    Ok(ht.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Statement::STMT_ASSERT { .. } => {
                    Ok(iht.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Statement::STMT_TERMINATE { .. } => {
                    Ok(iht.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Statement::STMT_REINIT { .. } => {
                    Ok(iht.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Statement::STMT_NORETCALL { .. } => {
                    Ok(iht.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Statement::STMT_RETURN { source: _ } => {
                    Ok(iht.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Statement::STMT_BREAK { source: _ } => {
                    Ok(iht.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Statement::STMT_CONTINUE { source: _ } => {
                    Ok(iht.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Statement::STMT_ARRAY_INIT { .. } => {
                    Ok(iht.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Statement::STMT_FAILURE { body: stmts, .. } => {
                    let mut ht: (metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>, (i32, i32, metamodelica::Array<Option<metamodelica::Ref<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr));
                    ht = List::fold1(metamodelica::AsArg::as_arg(&stmts), &move |__a0: metamodelica::Ref<DAE::Statement>, __a1: DAE::Expand, __a2: (metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>, (i32, i32, metamodelica::Array<Option<metamodelica::Ref<DAE::ComponentRef>>>), i32, i32, (Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>, metamodelica::Ref<DAE::ComponentRef>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>) -> Result<ArcStr> + 'static>))| statementOutputs(&__a0, __a1, &__a2), inCrefExpansion, iht.clone())?;
                    Ok(ht.clone())
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
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { return Err("pattern mismatch") };
                    r#str = DAEDump::ppStatementStr(inStatement.clone());
                    Debug::traceln({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("- CheckModel.statementOutputs failed for ")); __mm_s.push_str(&*r#str); ArcStr::from(__mm_s) })?;
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(oht)
}

fn statementElseOutputs<'__b>(
    mut inElseBranch: &'__b metamodelica::Ref<DAE::Else>,
    mut inCrefExpansion: DAE::Expand,
    mut iht: (
        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<Option<metamodelica::Ref<DAE::ComponentRef>>>,
        ),
        i32,
        i32,
        (
            Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>) -> Result<i32> + 'static>,
            Arc<
                dyn ::std::ops::Fn(
                        metamodelica::Ref<DAE::ComponentRef>,
                        metamodelica::Ref<DAE::ComponentRef>,
                    ) -> Result<bool>
                    + 'static,
            >,
            Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>) -> Result<ArcStr> + 'static>,
        ),
    ),
) -> Result<(
    metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
    (
        i32,
        i32,
        metamodelica::Array<Option<metamodelica::Ref<DAE::ComponentRef>>>,
    ),
    i32,
    i32,
    (
        Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>) -> Result<i32> + 'static>,
        Arc<
            dyn ::std::ops::Fn(
                    metamodelica::Ref<DAE::ComponentRef>,
                    metamodelica::Ref<DAE::ComponentRef>,
                ) -> Result<bool>
                + 'static,
        >,
        Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>) -> Result<ArcStr> + 'static>,
    ),
)> {
    '__tco: loop {
        match &**inElseBranch {
            DAE::Else::NOELSE { .. } => return Ok(iht),
            DAE::Else::ELSEIF {
                statementLst: stmts,
                else_: elseBranch,
                ..
            } => {
                let mut ht: (
                    metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
                    (
                        i32,
                        i32,
                        metamodelica::Array<Option<metamodelica::Ref<DAE::ComponentRef>>>,
                    ),
                    i32,
                    i32,
                    (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr),
                );
                ht = List::fold1(
                    stmts,
                    &move |__a0: metamodelica::Ref<DAE::Statement>,
                           __a1: DAE::Expand,
                           __a2: (
                        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
                        (
                            i32,
                            i32,
                            metamodelica::Array<Option<metamodelica::Ref<DAE::ComponentRef>>>,
                        ),
                        i32,
                        i32,
                        (
                            Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>) -> Result<i32> + 'static>,
                            Arc<
                                dyn ::std::ops::Fn(
                                        metamodelica::Ref<DAE::ComponentRef>,
                                        metamodelica::Ref<DAE::ComponentRef>,
                                    ) -> Result<bool>
                                    + 'static,
                            >,
                            Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>) -> Result<ArcStr> + 'static>,
                        ),
                    )| statementOutputs(&__a0, __a1, &__a2),
                    inCrefExpansion,
                    iht,
                )?;
                {
                    (inElseBranch, inCrefExpansion, iht) = (elseBranch, inCrefExpansion, ht);
                    continue '__tco;
                }
            }
            DAE::Else::ELSE { statementLst: stmts } => {
                let mut ht: (
                    metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
                    (
                        i32,
                        i32,
                        metamodelica::Array<Option<metamodelica::Ref<DAE::ComponentRef>>>,
                    ),
                    i32,
                    i32,
                    (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr),
                );
                return Ok(List::fold1(
                    stmts,
                    &move |__a0: metamodelica::Ref<DAE::Statement>,
                           __a1: DAE::Expand,
                           __a2: (
                        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
                        (
                            i32,
                            i32,
                            metamodelica::Array<Option<metamodelica::Ref<DAE::ComponentRef>>>,
                        ),
                        i32,
                        i32,
                        (
                            Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>) -> Result<i32> + 'static>,
                            Arc<
                                dyn ::std::ops::Fn(
                                        metamodelica::Ref<DAE::ComponentRef>,
                                        metamodelica::Ref<DAE::ComponentRef>,
                                    ) -> Result<bool>
                                    + 'static,
                            >,
                            Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>) -> Result<ArcStr> + 'static>,
                        ),
                    )| statementOutputs(&__a0, __a1, &__a2),
                    inCrefExpansion,
                    iht,
                )?);
            }
        }
    }
}

fn statementOutputsCrefFinder(
    mut inExp: metamodelica::Ref<DAE::Exp>,
    mut inTpl: (
        DAE::Expand,
        (
            metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
            (
                i32,
                i32,
                metamodelica::Array<Option<metamodelica::Ref<DAE::ComponentRef>>>,
            ),
            i32,
            i32,
            (
                Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>) -> Result<i32> + 'static>,
                Arc<
                    dyn ::std::ops::Fn(
                            metamodelica::Ref<DAE::ComponentRef>,
                            metamodelica::Ref<DAE::ComponentRef>,
                        ) -> Result<bool>
                        + 'static,
                >,
                Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>) -> Result<ArcStr> + 'static>,
            ),
        ),
    ),
) -> (
    metamodelica::Ref<DAE::Exp>,
    bool,
    (
        DAE::Expand,
        (
            metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
            (
                i32,
                i32,
                metamodelica::Array<Option<metamodelica::Ref<DAE::ComponentRef>>>,
            ),
            i32,
            i32,
            (
                Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>) -> Result<i32> + 'static>,
                Arc<
                    dyn ::std::ops::Fn(
                            metamodelica::Ref<DAE::ComponentRef>,
                            metamodelica::Ref<DAE::ComponentRef>,
                        ) -> Result<bool>
                        + 'static,
                >,
                Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>) -> Result<ArcStr> + 'static>,
            ),
        ),
    ),
) {
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    let mut cont: bool;
    let mut outTpl: (
        DAE::Expand,
        (
            metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
            (
                i32,
                i32,
                metamodelica::Array<Option<metamodelica::Ref<DAE::ComponentRef>>>,
            ),
            i32,
            i32,
            (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr),
        ),
    );
    (outExp, cont, outTpl) = 'mc: {
        let __mc_input = (inExp, &inTpl);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e @ Deref @ DAE::Exp::CREF { componentRef: Deref @ DAE::ComponentRef::WILD { .. }, .. }, _) => {
                    Ok((e.clone(), false, inTpl.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e @ Deref @ DAE::Exp::CREF { componentRef: Deref @ DAE::ComponentRef::CREF_IDENT { ident: Deref @ "time", subscriptLst: Deref @ metamodelica::ListNode::Nil, .. }, .. }, _) => {
                    Ok((e.clone(), false, inTpl.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e @ Deref @ DAE::Exp::CREF { ty: Deref @ DAE::Type::T_COMPLEX { complexClassType: ClassInf::State::EXTERNAL_OBJ { .. }, .. }, .. }, _) => {
                    Ok((e.clone(), false, inTpl.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e @ Deref @ DAE::Exp::CREF { componentRef: cr, ty: Deref @ DAE::Type::T_COMPLEX { complexClassType: ClassInf::State::RECORD { .. }, .. } }, (expand, ht)) => {
                    let mut crlst: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
                    let mut cr = (*cr).clone();
                    let mut ht = (*ht).clone();
                    cr = ComponentReference::crefStripSubs(metamodelica::AsArg::as_arg(&cr))?;
                    crlst = ComponentReference::expandCref(metamodelica::AsArg::as_arg(&cr), true)?;
                    ht = List::fold(&crlst, &move |__a0: _, __a1: _| BaseHashSet::add(__a0, &__a1), ht.clone())?;
                    Ok((e.clone(), false, (expand.clone(), ht.clone())))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e @ Deref @ DAE::Exp::CREF { componentRef: cr, .. }, (expand @ DAE::Expand::NOT_EXPAND { .. }, ht)) => {
                    let mut ht = (*ht).clone();
                    ht = List::fold(&(list![cr.clone()]), &move |__a0: _, __a1: _| BaseHashSet::add(__a0, &__a1), ht.clone())?;
                    Ok((e.clone(), false, (expand.clone(), ht.clone())))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e @ Deref @ DAE::Exp::CREF { componentRef: cr, .. }, (expand, ht)) => {
                    let mut first_cref: metamodelica::Ref<DAE::ComponentRef>;
                    let mut crlst: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
                    let mut cr = (*cr).clone();
                    let mut ht = (*ht).clone();
                    cr = ComponentReference::crefStripSubsExceptModelSubs(cr.clone());
                    first_cref = ComponentReference::crefArrayGetFirstCref(metamodelica::AsArg::as_arg(&cr))?;
                    if !(BaseHashSet::has(first_cref.clone(), &(ht.clone()))?) {
                        crlst = ComponentReference::expandCref(metamodelica::AsArg::as_arg(&cr), true)?;
                        ht = List::fold(&crlst, &move |__a0: _, __a1: _| BaseHashSet::add(__a0, &__a1), ht.clone())?;
                    }
                    Ok((e.clone(), false, (expand.clone(), ht.clone())))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e @ Deref @ DAE::Exp::ASUB { exp, .. }, _) => {
                    let mut outTpl: (DAE::Expand, (metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>, (i32, i32, metamodelica::Array<Option<metamodelica::Ref<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr)));
                    (_, outTpl) = Expression::traverseExpTopDown(exp.clone(), &fnptr!(statementOutputsCrefFinder, metamodelica::Ref<DAE::Exp>, (DAE::Expand, (metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>, (i32, i32, metamodelica::Array<Option<metamodelica::Ref<DAE::ComponentRef>>>), i32, i32, (Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>, metamodelica::Ref<DAE::ComponentRef>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>) -> Result<ArcStr> + 'static>)))), inTpl.clone())?;
                    Ok((e.clone(), false, outTpl.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e @ Deref @ DAE::Exp::TSUB { exp, .. }, _) => {
                    let mut outTpl: (DAE::Expand, (metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>, (i32, i32, metamodelica::Array<Option<metamodelica::Ref<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr)));
                    (_, outTpl) = Expression::traverseExpTopDown(exp.clone(), &fnptr!(statementOutputsCrefFinder, metamodelica::Ref<DAE::Exp>, (DAE::Expand, (metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>, (i32, i32, metamodelica::Array<Option<metamodelica::Ref<DAE::ComponentRef>>>), i32, i32, (Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>, metamodelica::Ref<DAE::ComponentRef>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>) -> Result<ArcStr> + 'static>)))), inTpl.clone())?;
                    Ok((e.clone(), false, outTpl.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e @ Deref @ DAE::Exp::RELATION { .. }, _) => {
                    Ok((e.clone(), false, inTpl.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e @ Deref @ DAE::Exp::RANGE { .. }, _) => {
                    Ok((e.clone(), false, inTpl.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e @ Deref @ DAE::Exp::IFEXP { expThen: e1, expElse: e2, .. }, _) => {
                    let mut outTpl: (DAE::Expand, (metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>, (i32, i32, metamodelica::Array<Option<metamodelica::Ref<DAE::ComponentRef>>>), i32, i32, (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr)));
                    (_, outTpl) = Expression::traverseExpTopDown(e1.clone(), &fnptr!(statementOutputsCrefFinder, metamodelica::Ref<DAE::Exp>, (DAE::Expand, (metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>, (i32, i32, metamodelica::Array<Option<metamodelica::Ref<DAE::ComponentRef>>>), i32, i32, (Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>, metamodelica::Ref<DAE::ComponentRef>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>) -> Result<ArcStr> + 'static>)))), inTpl.clone())?;
                    (_, outTpl) = Expression::traverseExpTopDown(e2.clone(), &fnptr!(statementOutputsCrefFinder, metamodelica::Ref<DAE::Exp>, (DAE::Expand, (metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>, (i32, i32, metamodelica::Array<Option<metamodelica::Ref<DAE::ComponentRef>>>), i32, i32, (Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>, metamodelica::Ref<DAE::ComponentRef>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>) -> Result<ArcStr> + 'static>)))), outTpl.clone())?;
                    Ok((e.clone(), false, outTpl.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e, _) => {
                    Ok((e.clone(), true, inTpl.clone()))
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

fn countSimpleEqnSize(
    mut inEqns: &metamodelica::List<metamodelica::Ref<DAE::Element>>,
    mut isimpleEqnSize: i32,
    mut ihs: (
        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<Option<metamodelica::Ref<DAE::ComponentRef>>>,
        ),
        i32,
        i32,
        (
            Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>) -> Result<i32> + 'static>,
            Arc<
                dyn ::std::ops::Fn(
                        metamodelica::Ref<DAE::ComponentRef>,
                        metamodelica::Ref<DAE::ComponentRef>,
                    ) -> Result<bool>
                    + 'static,
            >,
            Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>) -> Result<ArcStr> + 'static>,
        ),
    ),
) -> Result<i32> {
    let mut osimpleEqnSize: i32;
    osimpleEqnSize = List::applyAndFold1(
        inEqns,
        &fnptr!(intAdd, i32, i32),
        &move |__a0: metamodelica::Ref<DAE::Element>,
               __a1: (
            metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
            (
                i32,
                i32,
                metamodelica::Array<Option<metamodelica::Ref<DAE::ComponentRef>>>,
            ),
            i32,
            i32,
            (
                Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>) -> Result<i32> + 'static>,
                Arc<
                    dyn ::std::ops::Fn(
                            metamodelica::Ref<DAE::ComponentRef>,
                            metamodelica::Ref<DAE::ComponentRef>,
                        ) -> Result<bool>
                        + 'static,
                >,
                Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>) -> Result<ArcStr> + 'static>,
            ),
        )|
              -> metamodelica::Result<_> { ::std::result::Result::Ok(countSimpleEqnSizeWork(&__a0, __a1)) },
        ihs,
        0,
    )?;
    Ok(osimpleEqnSize)
}

fn countSimpleEqnSizeWork(
    mut inEqns: &metamodelica::Ref<DAE::Element>,
    mut ihs: (
        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<Option<metamodelica::Ref<DAE::ComponentRef>>>,
        ),
        i32,
        i32,
        (
            Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>) -> Result<i32> + 'static>,
            Arc<
                dyn ::std::ops::Fn(
                        metamodelica::Ref<DAE::ComponentRef>,
                        metamodelica::Ref<DAE::ComponentRef>,
                    ) -> Result<bool>
                    + 'static,
            >,
            Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>) -> Result<ArcStr> + 'static>,
        ),
    ),
) -> i32 {
    let mut osimpleEqnSize: i32;
    osimpleEqnSize = 'mc: {
        let __mc_input = &**inEqns;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Element::EQUATION { exp: e1, scalar: e2, .. } => {
                    Ok(simpleEquation(e1.clone(), e2.clone(), ihs.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Element::EQUEQUATION { cr1: cr, .. } => {
                    let mut tp: metamodelica::Ref<DAE::Type>;
                    tp = ComponentReference::crefTypeConsiderSubs(metamodelica::AsArg::as_arg(&cr))?;
                    Ok(Expression::sizeOf(&tp))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Element::DEFINE { componentRef: cr, exp: e2, .. } => {
                    let mut e1: metamodelica::Ref<DAE::Exp>;
                    e1 = Expression::crefExp(cr.clone())?;
                    Ok(simpleEquation(e1.clone(), e2.clone(), ihs.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Element::COMPLEX_EQUATION { lhs: e1, rhs: e2, .. } => {
                    Ok(simpleEquation(e1.clone(), e2.clone(), ihs.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Element::ARRAY_EQUATION { exp: e1, array: e2, .. } => {
                    Ok(simpleEquation(e1.clone(), e2.clone(), ihs.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(0)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    osimpleEqnSize
}

fn simpleEquation(
    mut e1: metamodelica::Ref<DAE::Exp>,
    mut e2: metamodelica::Ref<DAE::Exp>,
    mut ihs: (
        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<Option<metamodelica::Ref<DAE::ComponentRef>>>,
        ),
        i32,
        i32,
        (
            Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>) -> Result<i32> + 'static>,
            Arc<
                dyn ::std::ops::Fn(
                        metamodelica::Ref<DAE::ComponentRef>,
                        metamodelica::Ref<DAE::ComponentRef>,
                    ) -> Result<bool>
                    + 'static,
            >,
            Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>) -> Result<ArcStr> + 'static>,
        ),
    ),
) -> i32 {
    let mut osimpleEqnSize: i32;
    osimpleEqnSize = 'mc: {
        let __mc_input = (&*e1, &*e2);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CREF { .. }, Deref @ DAE::Exp::CREF { .. }) => {
                    Ok(Expression::sizeOf(&(Expression::r#typeof(e1.clone())?)))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CREF { .. }, Deref @ DAE::Exp::UNARY { operator: DAE::Operator::UMINUS { ty: _ }, exp: Deref @ DAE::Exp::CREF { .. } }) => {
                    Ok(Expression::sizeOf(&(Expression::r#typeof(e1.clone())?)))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CREF { .. }, Deref @ DAE::Exp::UNARY { operator: DAE::Operator::UMINUS_ARR { ty: _ }, exp: Deref @ DAE::Exp::CREF { .. } }) => {
                    Ok(Expression::sizeOf(&(Expression::r#typeof(e1.clone())?)))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::UNARY { operator: DAE::Operator::UMINUS { ty: _ }, exp: Deref @ DAE::Exp::CREF { .. } }, Deref @ DAE::Exp::CREF { .. }) => {
                    Ok(Expression::sizeOf(&(Expression::r#typeof(e1.clone())?)))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::UNARY { operator: DAE::Operator::UMINUS_ARR { ty: _ }, exp: Deref @ DAE::Exp::CREF { .. } }, Deref @ DAE::Exp::CREF { .. }) => {
                    Ok(Expression::sizeOf(&(Expression::r#typeof(e1.clone())?)))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::BINARY { exp1: Deref @ DAE::Exp::CREF { .. }, operator: DAE::Operator::ADD { .. }, exp2: Deref @ DAE::Exp::CREF { .. } }, _) => {
                    let true = (Expression::isZero(&e2)?) else { return Err("pattern mismatch") };
                    Ok(Expression::sizeOf(&(Expression::r#typeof(e1.clone())?)))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::BINARY { exp1: Deref @ DAE::Exp::CREF { .. }, operator: DAE::Operator::ADD_ARR { .. }, exp2: Deref @ DAE::Exp::CREF { .. } }, _) => {
                    let true = (Expression::isZero(&e2)?) else { return Err("pattern mismatch") };
                    Ok(Expression::sizeOf(&(Expression::r#typeof(e1.clone())?)))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::BINARY { exp1: Deref @ DAE::Exp::CREF { .. }, operator: DAE::Operator::SUB { .. }, exp2: Deref @ DAE::Exp::CREF { .. } }, _) => {
                    let true = (Expression::isZero(&e2)?) else { return Err("pattern mismatch") };
                    Ok(Expression::sizeOf(&(Expression::r#typeof(e1.clone())?)))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::BINARY { exp1: Deref @ DAE::Exp::CREF { .. }, operator: DAE::Operator::SUB_ARR { .. }, exp2: Deref @ DAE::Exp::CREF { .. } }, _) => {
                    let true = (Expression::isZero(&e2)?) else { return Err("pattern mismatch") };
                    Ok(Expression::sizeOf(&(Expression::r#typeof(e1.clone())?)))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::BINARY { exp1: Deref @ DAE::Exp::UNARY { operator: DAE::Operator::UMINUS { ty: _ }, exp: Deref @ DAE::Exp::CREF { .. } }, operator: DAE::Operator::ADD { .. }, exp2: Deref @ DAE::Exp::CREF { .. } }, _) => {
                    let true = (Expression::isZero(&e2)?) else { return Err("pattern mismatch") };
                    Ok(Expression::sizeOf(&(Expression::r#typeof(e1.clone())?)))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::BINARY { exp1: Deref @ DAE::Exp::UNARY { operator: DAE::Operator::UMINUS_ARR { ty: _ }, exp: Deref @ DAE::Exp::CREF { .. } }, operator: DAE::Operator::ADD_ARR { .. }, exp2: Deref @ DAE::Exp::CREF { .. } }, _) => {
                    let true = (Expression::isZero(&e2)?) else { return Err("pattern mismatch") };
                    Ok(Expression::sizeOf(&(Expression::r#typeof(e1.clone())?)))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::BINARY { exp1: Deref @ DAE::Exp::UNARY { operator: DAE::Operator::UMINUS { ty: _ }, exp: Deref @ DAE::Exp::CREF { .. } }, operator: DAE::Operator::SUB { .. }, exp2: Deref @ DAE::Exp::CREF { .. } }, _) => {
                    let true = (Expression::isZero(&e2)?) else { return Err("pattern mismatch") };
                    Ok(Expression::sizeOf(&(Expression::r#typeof(e1.clone())?)))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::BINARY { exp1: Deref @ DAE::Exp::UNARY { operator: DAE::Operator::UMINUS_ARR { ty: _ }, exp: Deref @ DAE::Exp::CREF { .. } }, operator: DAE::Operator::SUB_ARR { .. }, exp2: Deref @ DAE::Exp::CREF { .. } }, _) => {
                    let true = (Expression::isZero(&e2)?) else { return Err("pattern mismatch") };
                    Ok(Expression::sizeOf(&(Expression::r#typeof(e1.clone())?)))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Deref @ DAE::Exp::BINARY { exp1: Deref @ DAE::Exp::CREF { .. }, operator: DAE::Operator::ADD { .. }, exp2: Deref @ DAE::Exp::CREF { .. } }) => {
                    let true = (Expression::isZero(&e1)?) else { return Err("pattern mismatch") };
                    Ok(Expression::sizeOf(&(Expression::r#typeof(e1.clone())?)))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Deref @ DAE::Exp::BINARY { exp1: Deref @ DAE::Exp::CREF { .. }, operator: DAE::Operator::ADD_ARR { .. }, exp2: Deref @ DAE::Exp::CREF { .. } }) => {
                    let true = (Expression::isZero(&e1)?) else { return Err("pattern mismatch") };
                    Ok(Expression::sizeOf(&(Expression::r#typeof(e1.clone())?)))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Deref @ DAE::Exp::BINARY { exp1: Deref @ DAE::Exp::CREF { .. }, operator: DAE::Operator::SUB { .. }, exp2: Deref @ DAE::Exp::CREF { .. } }) => {
                    let true = (Expression::isZero(&e1)?) else { return Err("pattern mismatch") };
                    Ok(Expression::sizeOf(&(Expression::r#typeof(e1.clone())?)))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Deref @ DAE::Exp::BINARY { exp1: Deref @ DAE::Exp::CREF { .. }, operator: DAE::Operator::SUB_ARR { .. }, exp2: Deref @ DAE::Exp::CREF { .. } }) => {
                    let true = (Expression::isZero(&e1)?) else { return Err("pattern mismatch") };
                    Ok(Expression::sizeOf(&(Expression::r#typeof(e1.clone())?)))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Deref @ DAE::Exp::BINARY { exp1: Deref @ DAE::Exp::UNARY { operator: DAE::Operator::UMINUS { ty: _ }, exp: Deref @ DAE::Exp::CREF { .. } }, operator: DAE::Operator::ADD { .. }, exp2: Deref @ DAE::Exp::CREF { .. } }) => {
                    let true = (Expression::isZero(&e1)?) else { return Err("pattern mismatch") };
                    Ok(Expression::sizeOf(&(Expression::r#typeof(e1.clone())?)))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Deref @ DAE::Exp::BINARY { exp1: Deref @ DAE::Exp::UNARY { operator: DAE::Operator::UMINUS_ARR { ty: _ }, exp: Deref @ DAE::Exp::CREF { .. } }, operator: DAE::Operator::ADD_ARR { .. }, exp2: Deref @ DAE::Exp::CREF { .. } }) => {
                    let true = (Expression::isZero(&e1)?) else { return Err("pattern mismatch") };
                    Ok(Expression::sizeOf(&(Expression::r#typeof(e1.clone())?)))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Deref @ DAE::Exp::BINARY { exp1: Deref @ DAE::Exp::UNARY { operator: DAE::Operator::UMINUS { ty: _ }, exp: Deref @ DAE::Exp::CREF { .. } }, operator: DAE::Operator::SUB { .. }, exp2: Deref @ DAE::Exp::CREF { .. } }) => {
                    let true = (Expression::isZero(&e1)?) else { return Err("pattern mismatch") };
                    Ok(Expression::sizeOf(&(Expression::r#typeof(e1.clone())?)))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Deref @ DAE::Exp::BINARY { exp1: Deref @ DAE::Exp::UNARY { operator: DAE::Operator::UMINUS_ARR { ty: _ }, exp: Deref @ DAE::Exp::CREF { .. } }, operator: DAE::Operator::SUB_ARR { .. }, exp2: Deref @ DAE::Exp::CREF { .. } }) => {
                    let true = (Expression::isZero(&e1)?) else { return Err("pattern mismatch") };
                    Ok(Expression::sizeOf(&(Expression::r#typeof(e1.clone())?)))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CREF { .. }, Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "der" }, expLst: Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Exp::CREF { .. }, tail: Deref @ metamodelica::ListNode::Nil }, .. }) => {
                    Ok(Expression::sizeOf(&(Expression::r#typeof(e1.clone())?)))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "der" }, expLst: Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Exp::CREF { .. }, tail: Deref @ metamodelica::ListNode::Nil }, .. }, Deref @ DAE::Exp::CREF { .. }) => {
                    Ok(Expression::sizeOf(&(Expression::r#typeof(e2.clone())?)))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CREF { .. }, Deref @ DAE::Exp::UNARY { operator: DAE::Operator::UMINUS { ty: _ }, exp: Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "der" }, expLst: Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Exp::CREF { .. }, tail: Deref @ metamodelica::ListNode::Nil }, .. } }) => {
                    Ok(Expression::sizeOf(&(Expression::r#typeof(e1.clone())?)))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CREF { .. }, Deref @ DAE::Exp::UNARY { operator: DAE::Operator::UMINUS_ARR { ty: _ }, exp: Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "der" }, expLst: Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Exp::CREF { .. }, tail: Deref @ metamodelica::ListNode::Nil }, .. } }) => {
                    Ok(Expression::sizeOf(&(Expression::r#typeof(e1.clone())?)))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::UNARY { operator: DAE::Operator::UMINUS { ty: _ }, exp: Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "der" }, expLst: Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Exp::CREF { .. }, tail: Deref @ metamodelica::ListNode::Nil }, .. } }, Deref @ DAE::Exp::CREF { .. }) => {
                    Ok(Expression::sizeOf(&(Expression::r#typeof(e2.clone())?)))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::UNARY { operator: DAE::Operator::UMINUS_ARR { ty: _ }, exp: Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "der" }, expLst: Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Exp::CREF { .. }, tail: Deref @ metamodelica::ListNode::Nil }, .. } }, Deref @ DAE::Exp::CREF { .. }) => {
                    Ok(Expression::sizeOf(&(Expression::r#typeof(e2.clone())?)))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::UNARY { operator: DAE::Operator::UMINUS { ty: _ }, exp: Deref @ DAE::Exp::CREF { .. } }, Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "der" }, expLst: Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Exp::CREF { .. }, tail: Deref @ metamodelica::ListNode::Nil }, .. }) => {
                    Ok(Expression::sizeOf(&(Expression::r#typeof(e1.clone())?)))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::UNARY { operator: DAE::Operator::UMINUS_ARR { ty: _ }, exp: Deref @ DAE::Exp::CREF { .. } }, Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "der" }, expLst: Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Exp::CREF { .. }, tail: Deref @ metamodelica::ListNode::Nil }, .. }) => {
                    Ok(Expression::sizeOf(&(Expression::r#typeof(e1.clone())?)))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "der" }, expLst: Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Exp::CREF { .. }, tail: Deref @ metamodelica::ListNode::Nil }, .. }, Deref @ DAE::Exp::UNARY { operator: DAE::Operator::UMINUS { ty: _ }, exp: Deref @ DAE::Exp::CREF { .. } }) => {
                    Ok(Expression::sizeOf(&(Expression::r#typeof(e2.clone())?)))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "der" }, expLst: Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Exp::CREF { .. }, tail: Deref @ metamodelica::ListNode::Nil }, .. }, Deref @ DAE::Exp::UNARY { operator: DAE::Operator::UMINUS_ARR { ty: _ }, exp: Deref @ DAE::Exp::CREF { .. } }) => {
                    Ok(Expression::sizeOf(&(Expression::r#typeof(e2.clone())?)))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::UNARY { operator: DAE::Operator::UMINUS { ty: _ }, exp: Deref @ DAE::Exp::CREF { .. } }, Deref @ DAE::Exp::UNARY { operator: DAE::Operator::UMINUS { ty: _ }, exp: Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "der" }, expLst: Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Exp::CREF { .. }, tail: Deref @ metamodelica::ListNode::Nil }, .. } }) => {
                    Ok(Expression::sizeOf(&(Expression::r#typeof(e1.clone())?)))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::UNARY { operator: DAE::Operator::UMINUS_ARR { ty: _ }, exp: Deref @ DAE::Exp::CREF { .. } }, Deref @ DAE::Exp::UNARY { operator: DAE::Operator::UMINUS_ARR { ty: _ }, exp: Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "der" }, expLst: Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Exp::CREF { .. }, tail: Deref @ metamodelica::ListNode::Nil }, .. } }) => {
                    Ok(Expression::sizeOf(&(Expression::r#typeof(e1.clone())?)))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::UNARY { operator: DAE::Operator::UMINUS { ty: _ }, exp: Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "der" }, expLst: Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Exp::CREF { .. }, tail: Deref @ metamodelica::ListNode::Nil }, .. } }, Deref @ DAE::Exp::UNARY { operator: DAE::Operator::UMINUS { ty: _ }, exp: Deref @ DAE::Exp::CREF { .. } }) => {
                    Ok(Expression::sizeOf(&(Expression::r#typeof(e2.clone())?)))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::UNARY { operator: DAE::Operator::UMINUS_ARR { ty: _ }, exp: Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "der" }, expLst: Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Exp::CREF { .. }, tail: Deref @ metamodelica::ListNode::Nil }, .. } }, Deref @ DAE::Exp::UNARY { operator: DAE::Operator::UMINUS_ARR { ty: _ }, exp: Deref @ DAE::Exp::CREF { .. } }) => {
                    Ok(Expression::sizeOf(&(Expression::r#typeof(e2.clone())?)))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CREF { .. }, _) => {
                    let true = (Expression::isConst(e2.clone())?) else { return Err("pattern mismatch") };
                    Ok(Expression::sizeOf(&(Expression::r#typeof(e1.clone())?)))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Deref @ DAE::Exp::CREF { .. }) => {
                    let true = (Expression::isConst(e1.clone())?) else { return Err("pattern mismatch") };
                    Ok(Expression::sizeOf(&(Expression::r#typeof(e2.clone())?)))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::UNARY { operator: DAE::Operator::UMINUS { ty: _ }, exp: Deref @ DAE::Exp::CREF { .. } }, _) => {
                    let true = (Expression::isConst(e2.clone())?) else { return Err("pattern mismatch") };
                    Ok(Expression::sizeOf(&(Expression::r#typeof(e1.clone())?)))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::UNARY { operator: DAE::Operator::UMINUS_ARR { ty: _ }, exp: Deref @ DAE::Exp::CREF { .. } }, _) => {
                    let true = (Expression::isConst(e2.clone())?) else { return Err("pattern mismatch") };
                    Ok(Expression::sizeOf(&(Expression::r#typeof(e1.clone())?)))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Deref @ DAE::Exp::UNARY { operator: DAE::Operator::UMINUS { ty: _ }, exp: Deref @ DAE::Exp::CREF { .. } }) => {
                    let true = (Expression::isConst(e1.clone())?) else { return Err("pattern mismatch") };
                    Ok(Expression::sizeOf(&(Expression::r#typeof(e2.clone())?)))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Deref @ DAE::Exp::UNARY { operator: DAE::Operator::UMINUS_ARR { ty: _ }, exp: Deref @ DAE::Exp::CREF { .. } }) => {
                    let true = (Expression::isConst(e1.clone())?) else { return Err("pattern mismatch") };
                    Ok(Expression::sizeOf(&(Expression::r#typeof(e2.clone())?)))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _) => {
                    let mut ea1: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                    let mut ea2: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                    let true = (Expression::isArray(&e1) || Expression::isMatrix(&e1)) else { return Err("pattern mismatch") };
                    let true = (Expression::isArray(&e2) || Expression::isMatrix(&e2)) else { return Err("pattern mismatch") };
                    ea1 = Expression::flattenArrayExpToList(e1.clone());
                    ea2 = Expression::flattenArrayExpToList(e2.clone());
                    Ok(simpleEquations(&ea1, &ea2, 0, &ihs)?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _) => {
                    ::match_deref::match_deref! { match &(Expression::traverseExpBottomUp(Expression::expSub(e1.clone(), e2.clone())?, &fnptr!(traversingComponentRefFinder, metamodelica::Ref<DAE::Exp>, ((metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>, (i32, i32, metamodelica::Array<Option<metamodelica::Ref<DAE::ComponentRef>>>), i32, i32, (Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>, metamodelica::Ref<DAE::ComponentRef>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>) -> Result<ArcStr> + 'static>)), metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>)), (ihs.clone(), metamodelica::nil()))?) {
                        (_, (_, Deref @ metamodelica::ListNode::Cons { head: _, tail: Deref @ metamodelica::ListNode::Nil })) => (),
                        _ => return Err("pattern mismatch"),
                    } };
                    Ok(Expression::sizeOf(&(Expression::r#typeof(e1.clone())?)))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(0)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    osimpleEqnSize
}

fn traversingComponentRefFinder(
    mut inExp: metamodelica::Ref<DAE::Exp>,
    mut inTpl: (
        (
            metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
            (
                i32,
                i32,
                metamodelica::Array<Option<metamodelica::Ref<DAE::ComponentRef>>>,
            ),
            i32,
            i32,
            (
                Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>) -> Result<i32> + 'static>,
                Arc<
                    dyn ::std::ops::Fn(
                            metamodelica::Ref<DAE::ComponentRef>,
                            metamodelica::Ref<DAE::ComponentRef>,
                        ) -> Result<bool>
                        + 'static,
                >,
                Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>) -> Result<ArcStr> + 'static>,
            ),
        ),
        metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
    ),
) -> (
    metamodelica::Ref<DAE::Exp>,
    (
        (
            metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
            (
                i32,
                i32,
                metamodelica::Array<Option<metamodelica::Ref<DAE::ComponentRef>>>,
            ),
            i32,
            i32,
            (
                Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>) -> Result<i32> + 'static>,
                Arc<
                    dyn ::std::ops::Fn(
                            metamodelica::Ref<DAE::ComponentRef>,
                            metamodelica::Ref<DAE::ComponentRef>,
                        ) -> Result<bool>
                        + 'static,
                >,
                Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>) -> Result<ArcStr> + 'static>,
            ),
        ),
        metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
    ),
) {
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    let mut outTpl: (
        (
            metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
            (
                i32,
                i32,
                metamodelica::Array<Option<metamodelica::Ref<DAE::ComponentRef>>>,
            ),
            i32,
            i32,
            (HashSet::FuncHashCref, HashSet::FuncCrefEqual, HashSet::FuncCrefStr),
        ),
        metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
    );
    (outExp, outTpl) = 'mc: {
        let __mc_input = (inExp.clone(), &inTpl);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CREF { componentRef: Deref @ DAE::ComponentRef::WILD { .. }, .. }, _) => {
                    Ok((inExp.clone(), inTpl.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e @ Deref @ DAE::Exp::CREF { componentRef: cr, .. }, (hs, crefs)) => {
                    let mut crlst: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
                    let mut crefs = (*crefs).clone();
                    crlst = ComponentReference::expandCref(metamodelica::AsArg::as_arg(&cr), true)?;
                    crefs = getcr(&crlst, &(hs.clone()), crefs.clone())?;
                    Ok((e.clone(), (hs.clone(), crefs.clone())))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((inExp.clone(), inTpl.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    (outExp, outTpl)
}

fn getcr<'__b>(
    mut crefs: &'__b metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
    mut hs: &'__b (
        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<Option<metamodelica::Ref<DAE::ComponentRef>>>,
        ),
        i32,
        i32,
        (
            Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>) -> Result<i32> + 'static>,
            Arc<
                dyn ::std::ops::Fn(
                        metamodelica::Ref<DAE::ComponentRef>,
                        metamodelica::Ref<DAE::ComponentRef>,
                    ) -> Result<bool>
                    + 'static,
            >,
            Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>) -> Result<ArcStr> + 'static>,
        ),
    ),
    mut iAcc: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
) -> Result<metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>> {
    '__tco: loop {
        ::match_deref::match_deref! { match crefs {
            Deref @ metamodelica::ListNode::Nil => {
                return Ok(iAcc)
            },
            Deref @ metamodelica::ListNode::Cons { head: cr, tail: rest } if (BaseHashSet::has(cr.clone(), hs)?) => {
                let mut crlst: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
                crlst = List::unionEltOnTrue(cr.clone(), iAcc, &move |__a0: metamodelica::Ref<DAE::ComponentRef>, __a1: metamodelica::Ref<DAE::ComponentRef>| ComponentReferenceBasics::crefEqual(&__a0, &__a1))?;
                { (crefs, hs, iAcc) = (rest, hs, crlst); continue '__tco; }
            },
            Deref @ metamodelica::ListNode::Cons { head: _, tail: rest } => {
                { (crefs, hs, iAcc) = (rest, hs, iAcc); continue '__tco; }
            },
            _ => return Err("match: no arm matched"),
        } }
    }
}

fn simpleEquations<'__b>(
    mut e1lst: &'__b metamodelica::List<metamodelica::Ref<DAE::Exp>>,
    mut e2lst: &'__b metamodelica::List<metamodelica::Ref<DAE::Exp>>,
    mut isimpleEqnSize: i32,
    mut ihs: &'__b (
        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<Option<metamodelica::Ref<DAE::ComponentRef>>>,
        ),
        i32,
        i32,
        (
            Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>) -> Result<i32> + 'static>,
            Arc<
                dyn ::std::ops::Fn(
                        metamodelica::Ref<DAE::ComponentRef>,
                        metamodelica::Ref<DAE::ComponentRef>,
                    ) -> Result<bool>
                    + 'static,
            >,
            Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>) -> Result<ArcStr> + 'static>,
        ),
    ),
) -> Result<i32> {
    '__tco: loop {
        ::match_deref::match_deref! { match (e1lst, e2lst) {
            (Deref @ metamodelica::ListNode::Nil, Deref @ metamodelica::ListNode::Nil) => {
                return Ok(isimpleEqnSize)
            },
            (Deref @ metamodelica::ListNode::Cons { head: e1, tail: r1 }, Deref @ metamodelica::ListNode::Cons { head: e2, tail: r2 }) => {
                let mut size: i32;
                size = simpleEquation(e1.clone(), e2.clone(), ihs.clone());
                { (e1lst, e2lst, isimpleEqnSize, ihs) = (r1, r2, size + isimpleEqnSize, ihs); continue '__tco; }
            },
            _ => return Err("match: no arm matched"),
        } }
    }
}
