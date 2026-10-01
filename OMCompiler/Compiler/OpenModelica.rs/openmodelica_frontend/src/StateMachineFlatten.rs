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

use crate::HashTableCrToExpOption;
use openmodelica_ast::Absyn;
use openmodelica_frontend_base::ComponentReference;
use openmodelica_frontend_base::DAEDump;
use openmodelica_frontend_base::DAEUtil;
use openmodelica_frontend_base::Expression;
use openmodelica_frontend_base::Types;
use openmodelica_frontend_dump::AvlTreePathFunction;
use openmodelica_frontend_dump::ComponentReferenceBasics;
use openmodelica_frontend_dump::ExpressionBasics;
use openmodelica_frontend_dump::FCore;
use openmodelica_frontend_types::DAE;
use openmodelica_frontend_types::SCode;
use openmodelica_util::BaseHashTable;
use openmodelica_util::Error;
use openmodelica_util::Flags;
use openmodelica_util::Util;
use openmodelica_util_datatypes_basic::List;

///
/// Properties of a transition
#[derive(Clone, Debug, Eq, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub struct Transition {
    pub from: i32,
    pub to: i32,
    pub condition: metamodelica::Ref<DAE::Exp>,
    pub immediate: bool,
    pub reset: bool,
    pub synchronize: bool,
    pub priority: i32,
}

impl metamodelica::gc::MMTrace for Transition {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        metamodelica::gc::MMTrace::mm_accept(&self.from, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.to, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.condition, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.immediate, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.reset, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.synchronize, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.priority, __mmv)?;
        Ok(())
    }
}
impl Default for Transition {
    fn default() -> Self {
        Self {
            from: Default::default(),
            to: Default::default(),
            condition: Default::default(),
            immediate: Default::default(),
            reset: Default::default(),
            synchronize: Default::default(),
            priority: Default::default(),
        }
    }
}

pub type TRANSITION = Transition;

///
/// Structure that combines states of flat state machine in
/// canonical order with governing semantic equations.
#[derive(Clone, Debug, Eq, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub struct FlatSmSemantics {
    pub ident: ArcStr,
    /// First element is the initial state
    pub smComps: metamodelica::Array<metamodelica::Ref<DAE::Element>>,
    /// List/Array of transition data sorted in priority
    pub t: metamodelica::List<Transition>,
    /// Transition conditions sorted in priority
    pub c: metamodelica::List<metamodelica::Ref<DAE::Exp>>,
    /// SMS veriables
    pub vars: metamodelica::List<metamodelica::Ref<DAE::Element>>,
    /// SMS constants/parameters
    pub knowns: metamodelica::List<metamodelica::Ref<DAE::Element>>,
    /// SMS equations
    pub eqs: metamodelica::List<metamodelica::Ref<DAE::Element>>,
    /// Propagation related variables
    pub pvars: metamodelica::List<metamodelica::Ref<DAE::Element>>,
    /// Propagation equations
    pub peqs: metamodelica::List<metamodelica::Ref<DAE::Element>>,
    /// Cref to enclosing state if any
    pub enclosingState: Option<metamodelica::Ref<DAE::ComponentRef>>,
}

impl metamodelica::gc::MMTrace for FlatSmSemantics {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        metamodelica::gc::MMTrace::mm_accept(&self.ident, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.smComps, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.t, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.c, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.vars, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.knowns, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.eqs, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.pvars, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.peqs, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.enclosingState, __mmv)?;
        Ok(())
    }
}
impl Default for FlatSmSemantics {
    fn default() -> Self {
        Self {
            ident: Default::default(),
            smComps: Default::default(),
            t: Default::default(),
            c: Default::default(),
            vars: Default::default(),
            knowns: Default::default(),
            eqs: Default::default(),
            pvars: Default::default(),
            peqs: Default::default(),
            enclosingState: Default::default(),
        }
    }
}

pub type FLAT_SM_SEMANTICS = FlatSmSemantics;

pub(crate) const SMS_PRE: &'static str = "smOf";

pub fn stateMachineToDataFlow(
    mut cache: &FCore::Cache,
    mut env: &FCore::Graph,
    mut inDAElist: DAE::DAElist,
) -> Result<DAE::DAElist> {
    let mut outDAElist: DAE::DAElist;
    let mut elementLst: metamodelica::List<metamodelica::Ref<DAE::Element>>;
    let mut flatSmLst: metamodelica::List<metamodelica::Ref<DAE::Element>>;
    let mut otherLst: metamodelica::List<metamodelica::Ref<DAE::Element>>;
    let mut elementLst2: metamodelica::List<metamodelica::Ref<DAE::Element>>;
    let mut elementLst3: metamodelica::List<metamodelica::Ref<DAE::Element>>;
    let mut nOfSubstitutions: i32;
    let mut ident: ArcStr;
    let mut dAElist: metamodelica::List<metamodelica::Ref<DAE::Element>>;
    let mut source: metamodelica::Ref<DAE::ElementSource>;
    let mut comment: Option<metamodelica::Ref<SCode::Comment>>;
    let DAE::DAE { elementLst: __pa0 } = &inDAElist;
    elementLst = metamodelica::Own::own(__pa0);
    assert!(
        ((elementLst).len() as i32) == 1,
        "{}",
        &*literal!("Internal compiler error: Handling of elementLst != 1 not supported\n")
    );
    let (__pa1, __pa2, __pa3, __pa4) = ::match_deref::match_deref! { match &((elementLst).head().cloned()?) {
        Deref @ DAE::Element::COMP { ident: __pa1, dAElist: __pa2, source: __pa3, comment: __pa4 } => (__pa1.clone(), __pa2.clone(), __pa3.clone(), __pa4.clone()),
        _ => return Err("pattern mismatch"),
    } };
    ident = metamodelica::Own::own(__pa1);
    dAElist = metamodelica::Own::own(__pa2);
    source = metamodelica::Own::own(__pa3);
    comment = metamodelica::Own::own(__pa4);
    if !(List::any(
        &dAElist,
        &move |__a0: metamodelica::Ref<DAE::Element>| -> metamodelica::Result<_> {
            ::std::result::Result::Ok(isFlatSm(&__a0))
        },
    )?) {
        outDAElist = inDAElist;
        return Ok(outDAElist);
    }
    (flatSmLst, otherLst) = List::extractOnTrue(
        &dAElist,
        &move |__a0: metamodelica::Ref<DAE::Element>| -> metamodelica::Result<_> {
            ::std::result::Result::Ok(isFlatSm(&__a0))
        },
    )?;
    elementLst2 = List::fold2(
        &flatSmLst,
        &move |__a0: metamodelica::Ref<DAE::Element>,
               __a1: Option<metamodelica::Ref<DAE::ComponentRef>>,
               __a2: Option<FlatSmSemantics>,
               __a3: metamodelica::List<metamodelica::Ref<DAE::Element>>| {
            flatSmToDataFlow(&__a0, __a1, __a2, __a3)
        },
        None,
        None,
        metamodelica::nil(),
    )?;
    if Flags::getConfigBool(Flags::CT_STATE_MACHINES.clone())? {
        elementLst2 = wrapHack(cache, &elementLst2)?;
    }
    elementLst3 = listAppend(otherLst, elementLst2);
    outDAElist = DAE::DAElist {
        elementLst: list![metamodelica::Ref::new(DAE::Element::COMP {
            ident: ident,
            dAElist: elementLst3,
            source: source,
            comment: comment
        })],
    };
    let (__pa5, _, (_, __pa6)) = DAEUtil::traverseDAE(
        outDAElist,
        FCore::getFunctionTree(cache),
        (std::sync::Arc::new(Expression::traverseSubexpressionsHelper)
            as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>, _) -> Result<_> + 'static>),
        (
            (std::sync::Arc::new(traversingSubsActiveState)
                as std::sync::Arc<
                    dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>, i32) -> Result<(metamodelica::Ref<DAE::Exp>, i32)>
                        + 'static,
                >),
            0,
        ),
    )?;
    outDAElist = metamodelica::Own::own(__pa5);
    nOfSubstitutions = metamodelica::Own::own(__pa6);
    if Flags::getConfigBool(Flags::CT_STATE_MACHINES.clone())? {
        let (__pa7, _, (_, __pa8)) = DAEUtil::traverseDAE(
            outDAElist,
            FCore::getFunctionTree(cache),
            (std::sync::Arc::new(Expression::traverseSubexpressionsHelper)
                as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>, _) -> Result<_> + 'static>),
            (
                (std::sync::Arc::new(fnptr!(traversingSubsPreForPrevious, metamodelica::Ref<DAE::Exp>, i32))
                    as std::sync::Arc<
                        dyn ::std::ops::Fn(
                                metamodelica::Ref<DAE::Exp>,
                                i32,
                            ) -> Result<(metamodelica::Ref<DAE::Exp>, i32)>
                            + 'static,
                    >),
                0,
            ),
        )?;
        outDAElist = metamodelica::Own::own(__pa7);
        nOfSubstitutions = metamodelica::Own::own(__pa8);
    }
    Ok(outDAElist)
}

fn traversingSubsActiveState(
    mut inExp: metamodelica::Ref<DAE::Exp>,
    mut inHitCount: i32,
) -> Result<(metamodelica::Ref<DAE::Exp>, i32)> {
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    let mut outHitCount: i32;
    (outExp, outHitCount) = (::match_deref::match_deref! { match &(inExp.clone()) {
        Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "activeState" }, expLst: Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Exp::CREF { componentRef, .. }, tail: Deref @ metamodelica::ListNode::Nil }, .. } => {
            (metamodelica::Ref::new(DAE::Exp::CREF { componentRef: ComponentReference::crefPrependIdent(metamodelica::AsArg::as_arg(&componentRef), &(literal!("active")), &(metamodelica::nil()), &(DAE::T_BOOL_DEFAULT().clone()))?, ty: DAE::T_BOOL_DEFAULT().clone() }), inHitCount + 1)
        },
        _ => {
            (inExp, inHitCount)
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outExp, outHitCount))
}

fn flatSmToDataFlow(
    mut inFlatSm: &metamodelica::Ref<DAE::Element>,
    mut inEnclosingStateCrefOption: Option<metamodelica::Ref<DAE::ComponentRef>>,
    mut inEnclosingFlatSmSemanticsOption: Option<FlatSmSemantics>,
    mut accElems: metamodelica::List<metamodelica::Ref<DAE::Element>>,
) -> Result<metamodelica::List<metamodelica::Ref<DAE::Element>>> {
    let mut outElems: metamodelica::List<metamodelica::Ref<DAE::Element>> = accElems;
    let mut ident: ArcStr;
    let mut dAElist: metamodelica::List<metamodelica::Ref<DAE::Element>>;
    let mut smCompsLst: metamodelica::List<metamodelica::Ref<DAE::Element>>;
    let mut otherLst1: metamodelica::List<metamodelica::Ref<DAE::Element>>;
    let mut transitionLst: metamodelica::List<metamodelica::Ref<DAE::Element>>;
    let mut otherLst2: metamodelica::List<metamodelica::Ref<DAE::Element>>;
    let mut otherLst3: metamodelica::List<metamodelica::Ref<DAE::Element>>;
    let mut eqnLst: metamodelica::List<metamodelica::Ref<DAE::Element>>;
    let mut otherLst4: metamodelica::List<metamodelica::Ref<DAE::Element>>;
    let mut smCompsLst2: metamodelica::List<metamodelica::Ref<DAE::Element>>;
    let mut initialStateOp: metamodelica::Ref<DAE::Element>;
    let mut initialStateComp: metamodelica::Ref<DAE::Element>;
    let mut crefInitialState: metamodelica::Ref<DAE::ComponentRef>;
    let mut flatSmSemanticsBasics: FlatSmSemantics;
    let mut flatSmSemanticsWithPropagation: FlatSmSemantics;
    let mut flatSmSemantics: FlatSmSemantics;
    let mut vars: metamodelica::List<metamodelica::Ref<DAE::Element>>;
    let mut knowns: metamodelica::List<metamodelica::Ref<DAE::Element>>;
    let mut eqs: metamodelica::List<metamodelica::Ref<DAE::Element>>;
    let mut pvars: metamodelica::List<metamodelica::Ref<DAE::Element>>;
    let mut peqs: metamodelica::List<metamodelica::Ref<DAE::Element>>;
    let (__pa0, __pa1) = ::match_deref::match_deref! { match &((*inFlatSm)) {
        Deref @ DAE::Element::FLAT_SM { ident: __pa0, dAElist: __pa1 } => (__pa0.clone(), __pa1.clone()),
        _ => return Err("pattern mismatch"),
    } };
    ident = metamodelica::Own::own(__pa0);
    dAElist = metamodelica::Own::own(__pa1);
    (smCompsLst, otherLst1) = List::extractOnTrue(
        &dAElist,
        &move |__a0: metamodelica::Ref<DAE::Element>| -> metamodelica::Result<_> {
            ::std::result::Result::Ok(isSMComp(&__a0))
        },
    )?;
    (transitionLst, otherLst2) = List::extractOnTrue(
        &otherLst1,
        &move |__a0: metamodelica::Ref<DAE::Element>| -> metamodelica::Result<_> {
            ::std::result::Result::Ok(isTransition(&__a0))
        },
    )?;
    let (__pa2, __pa3) = ::match_deref::match_deref! { match &(List::extractOnTrue(&otherLst2, &move |__a0: metamodelica::Ref<DAE::Element>| -> metamodelica::Result<_> { ::std::result::Result::Ok(isInitialState(&__a0)) })?) {
        (Deref @ metamodelica::ListNode::Cons { head: __pa2, tail: Deref @ metamodelica::ListNode::Nil }, __pa3) => (__pa2.clone(), __pa3.clone()),
        _ => return Err("pattern mismatch"),
    } };
    initialStateOp = metamodelica::Own::own(__pa2);
    otherLst3 = metamodelica::Own::own(__pa3);
    (eqnLst, otherLst4) = List::extractOnTrue(
        &otherLst3,
        &move |__a0: metamodelica::Ref<DAE::Element>| -> metamodelica::Result<_> {
            ::std::result::Result::Ok(isEquation(&__a0))
        },
    )?;
    assert!(
        (otherLst4).is_empty(),
        "{}",
        &*literal!("Internal compiler error. Unexpected elements in flat state machine.")
    );
    let __pa5 = ::match_deref::match_deref! { match &(initialStateOp) {
        Deref @ DAE::Element::NORETCALL { exp: Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "initialState" }, expLst: Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Exp::CREF { componentRef: __pa5, .. }, tail: Deref @ metamodelica::ListNode::Nil }, .. }, .. } => __pa5.clone(),
        _ => return Err("pattern mismatch"),
    } };
    crefInitialState = metamodelica::Own::own(__pa5);
    let (__pa7, __pa8) = ::match_deref::match_deref! { match &(List::extract1OnTrue(&smCompsLst, &move |__a0: metamodelica::Ref<DAE::Element>, __a1: metamodelica::Ref<DAE::ComponentRef>| sMCompEqualsRef(&__a0, &__a1), crefInitialState)?) {
        (Deref @ metamodelica::ListNode::Cons { head: __pa7, tail: Deref @ metamodelica::ListNode::Nil }, __pa8) => (__pa7.clone(), __pa8.clone()),
        _ => return Err("pattern mismatch"),
    } };
    initialStateComp = metamodelica::Own::own(__pa7);
    smCompsLst2 = metamodelica::Own::own(__pa8);
    flatSmSemanticsBasics =
        basicFlatSmSemantics(ident, metamodelica::cons(initialStateComp, smCompsLst2), transitionLst)?;
    flatSmSemanticsWithPropagation = addPropagationEquations(
        flatSmSemanticsBasics,
        inEnclosingStateCrefOption.clone(),
        inEnclosingFlatSmSemanticsOption,
    )?;
    flatSmSemantics = elabXInStateOps(flatSmSemanticsWithPropagation, inEnclosingStateCrefOption)?;
    if Flags::getConfigBool(Flags::CT_STATE_MACHINES.clone())? {
        smCompsLst = List::map(smCompsLst, &move |__a0: metamodelica::Ref<DAE::Element>| {
            elabXInStateOps_CT(&__a0)
        })?;
    }
    let FlatSmSemantics {
        vars: __pa10,
        knowns: __pa11,
        eqs: __pa12,
        pvars: __pa13,
        peqs: __pa14,
        ..
    } = &flatSmSemantics;
    vars = metamodelica::Own::own(__pa10);
    knowns = metamodelica::Own::own(__pa11);
    eqs = metamodelica::Own::own(__pa12);
    pvars = metamodelica::Own::own(__pa13);
    peqs = metamodelica::Own::own(__pa14);
    outElems = List::flatten(list![outElems, eqnLst, vars, knowns, eqs, pvars, peqs])?;
    outElems = List::fold1(&smCompsLst, &smCompToDataFlow, flatSmSemantics, outElems)?;
    Ok(outElems)
}

fn elabXInStateOps_CT(mut inSmComp: &metamodelica::Ref<DAE::Element>) -> Result<metamodelica::Ref<DAE::Element>> {
    let mut outSmComp: metamodelica::Ref<DAE::Element>;
    let mut nOfHits: i32 = 0;
    let mut componentRef: metamodelica::Ref<DAE::ComponentRef>;
    let mut dAElist1: metamodelica::List<metamodelica::Ref<DAE::Element>>;
    let mut dAElist2: metamodelica::List<metamodelica::Ref<DAE::Element>>;
    let mut emptyTree: metamodelica::Ref<AvlTreePathFunction::Tree>;
    let (__pa0, __pa1) = ::match_deref::match_deref! { match &((*inSmComp)) {
        Deref @ DAE::Element::SM_COMP { componentRef: __pa0, dAElist: __pa1 } => (__pa0.clone(), __pa1.clone()),
        _ => return Err("pattern mismatch"),
    } };
    componentRef = metamodelica::Own::own(__pa0);
    dAElist1 = metamodelica::Own::own(__pa1);
    emptyTree = openmodelica_frontend_dump::AvlTreePathFunction::Tree::interned_EMPTY();
    let (__pa2, __pa3) = ::match_deref::match_deref! { match &(DAEUtil::traverseDAE(DAE::DAElist { elementLst: dAElist1 }, emptyTree, (std::sync::Arc::new(Expression::traverseSubexpressionsHelper) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>, _) -> Result<_> + 'static>), ((std::sync::Arc::new(traversingSubsTicksInState) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>, (metamodelica::Ref<DAE::ComponentRef>, i32)) -> Result<(metamodelica::Ref<DAE::Exp>, (metamodelica::Ref<DAE::ComponentRef>, i32))> + 'static>), (componentRef.clone(), 0)))?) {
        (DAE::DAElist { elementLst: __pa2 }, _, (_, (_, __pa3))) => (__pa2.clone(), __pa3.clone()),
        _ => unreachable!(),
    } };
    dAElist2 = metamodelica::Own::own(__pa2);
    nOfHits = metamodelica::Own::own(__pa3);
    outSmComp = metamodelica::Ref::new(DAE::Element::SM_COMP {
        componentRef: componentRef,
        dAElist: dAElist2,
    });
    Ok(outSmComp)
}

fn traversingSubsTicksInState(
    mut inExp: metamodelica::Ref<DAE::Exp>,
    mut inCref_HitCount: (metamodelica::Ref<DAE::ComponentRef>, i32),
) -> Result<(metamodelica::Ref<DAE::Exp>, (metamodelica::Ref<DAE::ComponentRef>, i32))> {
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    let mut outCref_HitCount: (metamodelica::Ref<DAE::ComponentRef>, i32);
    let mut cref: metamodelica::Ref<DAE::ComponentRef>;
    let mut hitCount: i32;
    (cref, hitCount) = inCref_HitCount.clone();
    (outExp, outCref_HitCount) = (::match_deref::match_deref! { match &(inExp.clone()) {
        Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "ticksInState" }, expLst: Deref @ metamodelica::ListNode::Nil, attr: Deref @ DAE::CallAttributes { ty, .. } } => {
            let mut crefTicksInState: metamodelica::Ref<DAE::ComponentRef>;
            crefTicksInState = ComponentReference::joinCrefs(&cref, metamodelica::Ref::new(DAE::ComponentRef::CREF_IDENT { ident: literal!("$ticksInState"), identType: ty.clone(), subscriptLst: metamodelica::nil() }))?;
            (metamodelica::Ref::new(DAE::Exp::CREF { componentRef: crefTicksInState, ty: ty.clone() }), (cref, hitCount + 1))
        },
        _ => {
            (inExp, inCref_HitCount)
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outExp, outCref_HitCount))
}

fn elabXInStateOps(
    mut inFlatSmSemantics: FlatSmSemantics,
    mut inEnclosingStateCrefOption: Option<metamodelica::Ref<DAE::ComponentRef>>,
) -> Result<FlatSmSemantics> {
    let mut outFlatSmSemantics: FlatSmSemantics;
    let mut i: i32;
    let mut found: bool;
    let mut c2: metamodelica::Ref<DAE::Exp>;
    let mut c3: metamodelica::Ref<DAE::Exp>;
    let mut c4: metamodelica::Ref<DAE::Exp>;
    let mut substTickExp: metamodelica::Ref<DAE::Exp>;
    let mut substTimeExp: metamodelica::Ref<DAE::Exp>;
    let mut stateRef: metamodelica::Ref<DAE::ComponentRef>;
    let mut t2: Transition;
    let mut tElab: metamodelica::List<Transition> = metamodelica::nil();
    let mut cElab: metamodelica::List<metamodelica::Ref<DAE::Exp>> = metamodelica::nil();
    let mut smeqsElab: metamodelica::List<metamodelica::Ref<DAE::Element>> = metamodelica::nil();
    let mut ident: ArcStr;
    let mut smComps: metamodelica::Array<metamodelica::Ref<DAE::Element>>;
    let mut t: metamodelica::List<Transition>;
    let mut c: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
    let mut smvars: metamodelica::List<metamodelica::Ref<DAE::Element>>;
    let mut smknowns: metamodelica::List<metamodelica::Ref<DAE::Element>>;
    let mut smeqs: metamodelica::List<metamodelica::Ref<DAE::Element>>;
    let mut pvars: metamodelica::List<metamodelica::Ref<DAE::Element>> = metamodelica::nil();
    let mut peqs: metamodelica::List<metamodelica::Ref<DAE::Element>> = metamodelica::nil();
    let mut enclosingStateOption: Option<metamodelica::Ref<DAE::ComponentRef>>;
    let mut from: i32;
    let mut to: i32;
    let mut condition: metamodelica::Ref<DAE::Exp>;
    let mut immediate: bool;
    let mut reset: bool;
    let mut synchronize: bool;
    let mut priority: i32;
    let FlatSmSemantics {
        ident: __pa0,
        smComps: __pa1,
        t: __pa2,
        c: __pa3,
        vars: __pa4,
        knowns: __pa5,
        eqs: __pa6,
        pvars: __pa7,
        peqs: __pa8,
        enclosingState: __pa9,
    } = inFlatSmSemantics;
    ident = metamodelica::Own::own(__pa0);
    smComps = metamodelica::Own::own(__pa1);
    t = metamodelica::Own::own(__pa2);
    c = metamodelica::Own::own(__pa3);
    smvars = metamodelica::Own::own(__pa4);
    smknowns = metamodelica::Own::own(__pa5);
    smeqs = metamodelica::Own::own(__pa6);
    pvars = metamodelica::Own::own(__pa7);
    peqs = metamodelica::Own::own(__pa8);
    enclosingStateOption = metamodelica::Own::own(__pa9);
    i = 0;
    for mut tc in &*List::zip(t.clone(), c) {
        i = i + 1;
        (t2, c2) = tc.clone();
        let Transition {
            from: __pa10,
            to: __pa11,
            condition: __pa12,
            immediate: __pa13,
            reset: __pa14,
            synchronize: __pa15,
            priority: __pa16,
        } = t2;
        from = metamodelica::Own::own(__pa10);
        to = metamodelica::Own::own(__pa11);
        condition = metamodelica::Own::own(__pa12);
        immediate = metamodelica::Own::own(__pa13);
        reset = metamodelica::Own::own(__pa14);
        synchronize = metamodelica::Own::own(__pa15);
        priority = metamodelica::Own::own(__pa16);
        let __pa17 = ::match_deref::match_deref! { match &(metamodelica::arrayGet(smComps.clone(), from)?) {
            Deref @ DAE::Element::SM_COMP { componentRef: __pa17, .. } => __pa17.clone(),
            _ => return Err("pattern mismatch"),
        } };
        stateRef = metamodelica::Own::own(__pa17);
        substTickExp = metamodelica::Ref::new(DAE::Exp::CREF {
            componentRef: qCref(
                literal!("$ticksInState"),
                DAE::T_INTEGER_DEFAULT().clone(),
                metamodelica::nil(),
                &stateRef,
            )?,
            ty: DAE::T_INTEGER_DEFAULT().clone(),
        });
        let (__pa18, (_, _, __pa19)) = Expression::traverseExpTopDown(
            c2.clone(),
            &fnptr!(
                traversingSubsXInState,
                metamodelica::Ref<DAE::Exp>,
                (ArcStr, metamodelica::Ref<DAE::Exp>, bool)
            ),
            (literal!("ticksInState"), substTickExp.clone(), false),
        )?;
        c3 = metamodelica::Own::own(__pa18);
        found = metamodelica::Own::own(__pa19);
        if found && (inEnclosingStateCrefOption).is_some() {
            Error::addCompilerError(literal!(
                "Found 'ticksInState()' within a state of an hierarchical state machine."
            ))?;
            return Err("fail");
        }
        smeqsElab = if (found) {
            List::map5(
                smeqs,
                &move |__a0: metamodelica::Ref<DAE::Element>,
                       __a1: metamodelica::Ref<DAE::Element>,
                       __a2: i32,
                       __a3: i32,
                       __a4: metamodelica::Ref<DAE::Exp>,
                       __a5: ArcStr| smeqsSubsXInState(&__a0, &__a1, __a2, __a3, __a4, __a5),
                metamodelica::arrayGet(smComps.clone(), 1)?,
                i,
                ((t).len() as i32),
                substTickExp,
                literal!("ticksInState"),
            )?
        } else {
            smeqs
        };
        smeqs = smeqsElab;
        substTimeExp = metamodelica::Ref::new(DAE::Exp::CREF {
            componentRef: qCref(
                literal!("$timeInState"),
                DAE::T_REAL_DEFAULT().clone(),
                metamodelica::nil(),
                &stateRef,
            )?,
            ty: DAE::T_REAL_DEFAULT().clone(),
        });
        let (__pa20, (_, _, __pa21)) = Expression::traverseExpTopDown(
            c2,
            &fnptr!(
                traversingSubsXInState,
                metamodelica::Ref<DAE::Exp>,
                (ArcStr, metamodelica::Ref<DAE::Exp>, bool)
            ),
            (literal!("timeInState"), substTimeExp.clone(), false),
        )?;
        c4 = metamodelica::Own::own(__pa20);
        found = metamodelica::Own::own(__pa21);
        if found && (inEnclosingStateCrefOption).is_some() {
            Error::addCompilerError(literal!(
                "Found 'timeInState()' within a state of an hierarchical state machine."
            ))?;
            return Err("fail");
        }
        smeqsElab = if (found) {
            List::map5(
                smeqs,
                &move |__a0: metamodelica::Ref<DAE::Element>,
                       __a1: metamodelica::Ref<DAE::Element>,
                       __a2: i32,
                       __a3: i32,
                       __a4: metamodelica::Ref<DAE::Exp>,
                       __a5: ArcStr| smeqsSubsXInState(&__a0, &__a1, __a2, __a3, __a4, __a5),
                metamodelica::arrayGet(smComps.clone(), 1)?,
                i,
                ((t).len() as i32),
                substTimeExp,
                literal!("timeInState"),
            )?
        } else {
            smeqs
        };
        smeqs = smeqsElab.clone();
        tElab = metamodelica::cons(
            Transition {
                from: from,
                to: to,
                condition: c4.clone(),
                immediate: immediate,
                reset: reset,
                synchronize: synchronize,
                priority: priority,
            },
            tElab,
        );
        cElab = metamodelica::cons(c4, cElab);
    }
    outFlatSmSemantics = FlatSmSemantics {
        ident: ident,
        smComps: smComps.clone(),
        t: tElab.reverse(),
        c: cElab.reverse(),
        vars: smvars,
        knowns: smknowns,
        eqs: smeqsElab,
        pvars: pvars,
        peqs: peqs,
        enclosingState: enclosingStateOption,
    };
    Ok(outFlatSmSemantics)
}

fn smeqsSubsXInState(
    mut inSmeqs: &metamodelica::Ref<DAE::Element>,
    mut initialStateComp: &metamodelica::Ref<DAE::Element>,
    mut i: i32,
    mut nTransitions: i32,
    mut substExp: metamodelica::Ref<DAE::Exp>,
    mut xInState: ArcStr,
) -> Result<metamodelica::Ref<DAE::Element>> {
    let mut outSmeqs: metamodelica::Ref<DAE::Element>;
    let mut preRef: metamodelica::Ref<DAE::ComponentRef>;
    let mut cref: metamodelica::Ref<DAE::ComponentRef>;
    let mut lhsRef: metamodelica::Ref<DAE::ComponentRef>;
    let mut crefInitialState: metamodelica::Ref<DAE::ComponentRef>;
    let mut tArrayBool: metamodelica::Ref<DAE::Type>;
    let mut elemSource: metamodelica::Ref<DAE::ElementSource>;
    let mut lhsExp: metamodelica::Ref<DAE::Exp>;
    let mut rhsExp: metamodelica::Ref<DAE::Exp>;
    let mut rhsExp2: metamodelica::Ref<DAE::Exp>;
    let mut ty: metamodelica::Ref<DAE::Type>;
    let __pa0 = ::match_deref::match_deref! { match &((*initialStateComp)) {
        Deref @ DAE::Element::SM_COMP { componentRef: __pa0, .. } => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    crefInitialState = metamodelica::Own::own(__pa0);
    preRef = ComponentReference::crefPrefixString(arcstr::literal!(SMS_PRE), crefInitialState);
    tArrayBool = metamodelica::Ref::new(DAE::Type::T_ARRAY {
        ty: DAE::T_BOOL_DEFAULT().clone(),
        dims: list![metamodelica::Ref::new(DAE::Dimension::DIM_INTEGER {
            integer: nTransitions
        })],
    });
    cref = qCref(
        literal!("cImmediate"),
        tArrayBool,
        list![metamodelica::Ref::new(DAE::Subscript::INDEX {
            exp: metamodelica::Ref::new(DAE::Exp::ICONST { integer: i })
        })],
        &preRef,
    )?;
    let (__pa1, __pa2, __pa3) = ::match_deref::match_deref! { match &((*inSmeqs)) {
        Deref @ DAE::Element::EQUATION { exp: __pa1, scalar: __pa2, source: __pa3 } => (__pa1.clone(), __pa2.clone(), __pa3.clone()),
        _ => return Err("pattern mismatch"),
    } };
    lhsExp = metamodelica::Own::own(__pa1);
    rhsExp = metamodelica::Own::own(__pa2);
    elemSource = metamodelica::Own::own(__pa3);
    let (__pa4, __pa5) = ::match_deref::match_deref! { match &(lhsExp.clone()) {
        Deref @ DAE::Exp::CREF { componentRef: __pa4, ty: __pa5 } => (__pa4.clone(), __pa5.clone()),
        _ => return Err("pattern mismatch"),
    } };
    lhsRef = metamodelica::Own::own(__pa4);
    ty = metamodelica::Own::own(__pa5);
    if ComponentReferenceBasics::crefEqual(&cref, &lhsRef)? {
        (rhsExp2, _) = Expression::traverseExpTopDown(
            rhsExp,
            &fnptr!(
                traversingSubsXInState,
                metamodelica::Ref<DAE::Exp>,
                (ArcStr, metamodelica::Ref<DAE::Exp>, bool)
            ),
            (xInState, substExp, false),
        )?;
    } else {
        rhsExp2 = rhsExp;
    }
    outSmeqs = metamodelica::Ref::new(DAE::Element::EQUATION {
        exp: lhsExp,
        scalar: rhsExp2,
        source: elemSource,
    });
    Ok(outSmeqs)
}

fn traversingSubsXInState(
    mut inExp: metamodelica::Ref<DAE::Exp>,
    mut inXSubstHit: (ArcStr, metamodelica::Ref<DAE::Exp>, bool),
) -> (
    metamodelica::Ref<DAE::Exp>,
    bool,
    (ArcStr, metamodelica::Ref<DAE::Exp>, bool),
) {
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    let mut cont: bool = true;
    let mut outXSubstHit: (ArcStr, metamodelica::Ref<DAE::Exp>, bool);
    (outExp, outXSubstHit) = (::match_deref::match_deref! { match &((inExp.clone(), inXSubstHit.clone())) {
        (Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name }, .. }, (xInState, subsExp, _)) if (metamodelica::stringEq(&name, &xInState)) => {
            (subsExp.clone(), (xInState.clone(), subsExp.clone(), true))
        },
        _ => {
            (inExp, inXSubstHit)
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    (outExp, cont, outXSubstHit)
}

fn smCompToDataFlow(
    mut inSMComp: metamodelica::Ref<DAE::Element>,
    mut inEnclosingFlatSmSemantics: FlatSmSemantics,
    mut accElems: metamodelica::List<metamodelica::Ref<DAE::Element>>,
) -> Result<metamodelica::List<metamodelica::Ref<DAE::Element>>> {
    let mut outElems: metamodelica::List<metamodelica::Ref<DAE::Element>> = accElems;
    let mut varLst1: metamodelica::List<metamodelica::Ref<DAE::Element>>;
    let mut varLst2: metamodelica::List<metamodelica::Ref<DAE::Element>>;
    let mut assignedVarLst: metamodelica::List<metamodelica::Ref<DAE::Element>>;
    let mut stateVarLst: metamodelica::List<metamodelica::Ref<DAE::Element>>;
    let mut otherLst1: metamodelica::List<metamodelica::Ref<DAE::Element>>;
    let mut equationLst1: metamodelica::List<metamodelica::Ref<DAE::Element>>;
    let mut equationLst2: metamodelica::List<metamodelica::Ref<DAE::Element>>;
    let mut otherLst2: metamodelica::List<metamodelica::Ref<DAE::Element>>;
    let mut flatSmLst: metamodelica::List<metamodelica::Ref<DAE::Element>>;
    let mut otherLst3: metamodelica::List<metamodelica::Ref<DAE::Element>>;
    let mut componentRef: metamodelica::Ref<DAE::ComponentRef>;
    let mut stateVarCrefs: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
    let mut startValuesOpt: metamodelica::List<Option<metamodelica::Ref<DAE::Exp>>>;
    let mut varCrefStartVal: metamodelica::List<(
        metamodelica::Ref<DAE::ComponentRef>,
        Option<metamodelica::Ref<DAE::Exp>>,
    )>;
    let mut dAElist: metamodelica::List<metamodelica::Ref<DAE::Element>>;
    let mut crToExpOpt: (
        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<
                Option<(
                    metamodelica::Ref<DAE::ComponentRef>,
                    Option<metamodelica::Ref<DAE::Exp>>,
                )>,
            >,
        ),
        i32,
        (
            HashTableCrToExpOption::FuncHashCref,
            HashTableCrToExpOption::FuncCrefEqual,
            HashTableCrToExpOption::FuncCrefStr,
            HashTableCrToExpOption::FuncExpStr,
        ),
    );
    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(inSMComp.clone()) {
        Deref @ DAE::Element::SM_COMP { componentRef: __pa0, dAElist: __pa1 } => (__pa0.clone(), __pa1.clone()),
        _ => return Err("pattern mismatch"),
    } };
    componentRef = metamodelica::Own::own(__pa0);
    dAElist = metamodelica::Own::own(__pa1);
    (varLst1, otherLst1) = List::extractOnTrue(
        &dAElist,
        &move |__a0: metamodelica::Ref<DAE::Element>| -> metamodelica::Result<_> {
            ::std::result::Result::Ok(isVar(&__a0))
        },
    )?;
    (equationLst1, otherLst2) = List::extractOnTrue(
        &otherLst1,
        &move |__a0: metamodelica::Ref<DAE::Element>| -> metamodelica::Result<_> {
            ::std::result::Result::Ok(isEquationOrWhenEquation(&__a0))
        },
    )?;
    assignedVarLst = List::filterOnTrue(
        varLst1.clone(),
        (std::sync::Arc::new({
            let __pe_b0 = equationLst1.clone();
            let __pe_b1: Arc<dyn ::std::ops::Fn(_, _) -> Result<bool> + 'static> = (std::sync::Arc::new(
                move |__a0: metamodelica::Ref<DAE::Element>, __a1: metamodelica::Ref<DAE::Element>| {
                    isVarAtLHS(&__a0, __a1)
                },
            )
                as std::sync::Arc<
                    dyn ::std::ops::Fn(metamodelica::Ref<DAE::Element>, metamodelica::Ref<DAE::Element>) -> Result<bool>
                        + 'static,
                >);
            move |__pe_a2| List::exist1(&__pe_b0, &*__pe_b1, __pe_a2)
        }) as std::sync::Arc<dyn ::std::ops::Fn(_) -> Result<bool> + 'static>),
    )?;
    stateVarLst = List::filterOnTrue(
        varLst1.clone(),
        (std::sync::Arc::new({
            let __pe_b0 = equationLst1.clone();
            let __pe_b1: Arc<dyn ::std::ops::Fn(_, _) -> Result<bool> + 'static> = (std::sync::Arc::new(
                move |__a0: metamodelica::Ref<DAE::Element>, __a1: metamodelica::Ref<DAE::Element>| {
                    isPreviousAppliedToVar(&__a0, __a1)
                },
            )
                as std::sync::Arc<
                    dyn ::std::ops::Fn(metamodelica::Ref<DAE::Element>, metamodelica::Ref<DAE::Element>) -> Result<bool>
                        + 'static,
                >);
            move |__pe_a2| List::exist1(&__pe_b0, &*__pe_b1, __pe_a2)
        }) as std::sync::Arc<dyn ::std::ops::Fn(_) -> Result<bool> + 'static>),
    )?;
    stateVarCrefs = List::map(stateVarLst.clone(), &move |__a0: metamodelica::Ref<DAE::Element>| {
        DAEUtil::varCref(&__a0)
    })?;
    startValuesOpt = List::map(stateVarLst, &move |__a0: metamodelica::Ref<DAE::Element>| {
        getStartAttrOption(&__a0)
    })?;
    varCrefStartVal = List::zip(stateVarCrefs, startValuesOpt);
    crToExpOpt = HashTableCrToExpOption::emptyHashTableSized(((varCrefStartVal).len() as i32) + 1);
    crToExpOpt = List::fold(&varCrefStartVal, &BaseHashTable::add, crToExpOpt)?;
    (equationLst2, varLst2) = List::fold3(
        &equationLst1,
        &move |__a0: metamodelica::Ref<DAE::Element>,
               __a1: metamodelica::Ref<DAE::Element>,
               __a2: FlatSmSemantics,
               __a3: (
            metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
            (
                i32,
                i32,
                metamodelica::Array<
                    Option<(
                        metamodelica::Ref<DAE::ComponentRef>,
                        Option<metamodelica::Ref<DAE::Exp>>,
                    )>,
                >,
            ),
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
                Arc<dyn ::std::ops::Fn(Option<metamodelica::Ref<DAE::Exp>>) -> Result<ArcStr> + 'static>,
            ),
        ),
               __a4: (
            metamodelica::List<metamodelica::Ref<DAE::Element>>,
            metamodelica::List<metamodelica::Ref<DAE::Element>>,
        )| addStateActivationAndReset(&__a0, __a1, __a2, __a3, __a4),
        inSMComp,
        inEnclosingFlatSmSemantics.clone(),
        crToExpOpt,
        (metamodelica::nil(), metamodelica::nil()),
    )?;
    (flatSmLst, otherLst3) = List::extractOnTrue(
        &otherLst2,
        &move |__a0: metamodelica::Ref<DAE::Element>| -> metamodelica::Result<_> {
            ::std::result::Result::Ok(isFlatSm(&__a0))
        },
    )?;
    outElems = List::flatten(list![outElems, varLst1, varLst2, equationLst2, otherLst3])?;
    outElems = List::fold2(
        &flatSmLst,
        &move |__a0: metamodelica::Ref<DAE::Element>,
               __a1: Option<metamodelica::Ref<DAE::ComponentRef>>,
               __a2: Option<FlatSmSemantics>,
               __a3: metamodelica::List<metamodelica::Ref<DAE::Element>>| {
            flatSmToDataFlow(&__a0, __a1, __a2, __a3)
        },
        Some(componentRef),
        Some(inEnclosingFlatSmSemantics),
        outElems,
    )?;
    Ok(outElems)
}

fn addStateActivationAndReset(
    mut inEqn: &metamodelica::Ref<DAE::Element>,
    mut inEnclosingSMComp: metamodelica::Ref<DAE::Element>,
    mut inEnclosingFlatSmSemantics: FlatSmSemantics,
    mut crToExpOpt: (
        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<
                Option<(
                    metamodelica::Ref<DAE::ComponentRef>,
                    Option<metamodelica::Ref<DAE::Exp>>,
                )>,
            >,
        ),
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
            Arc<dyn ::std::ops::Fn(Option<metamodelica::Ref<DAE::Exp>>) -> Result<ArcStr> + 'static>,
        ),
    ),
    mut accEqnsVars: (
        metamodelica::List<metamodelica::Ref<DAE::Element>>,
        metamodelica::List<metamodelica::Ref<DAE::Element>>,
    ),
) -> Result<(
    metamodelica::List<metamodelica::Ref<DAE::Element>>,
    metamodelica::List<metamodelica::Ref<DAE::Element>>,
)> {
    let mut outEqnsVars: (
        metamodelica::List<metamodelica::Ref<DAE::Element>>,
        metamodelica::List<metamodelica::Ref<DAE::Element>>,
    );
    let mut equations1: metamodelica::List<metamodelica::Ref<DAE::Element>>;
    let mut vars1: metamodelica::List<metamodelica::Ref<DAE::Element>>;
    let mut condition: metamodelica::Ref<DAE::Exp>;
    let mut equations: metamodelica::List<metamodelica::Ref<DAE::Element>>;
    let mut source: metamodelica::Ref<DAE::ElementSource>;
    outEqnsVars = (::match_deref::match_deref! { match inEqn {
        Deref @ DAE::Element::EQUATION { .. } => addStateActivationAndReset1(inEqn, &inEnclosingSMComp, inEnclosingFlatSmSemantics, &crToExpOpt, accEqnsVars)?,
        Deref @ DAE::Element::WHEN_EQUATION { condition: __esc_condition, equations: __esc_equations, elsewhen_: None, source: __esc_source } => {
            condition = (*__esc_condition).clone();
            equations = (*__esc_equations).clone();
            source = (*__esc_source).clone();
            (equations1, vars1) = List::fold3(metamodelica::AsArg::as_arg(&equations), &move |__a0: metamodelica::Ref<DAE::Element>, __a1: metamodelica::Ref<DAE::Element>, __a2: FlatSmSemantics, __a3: (metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>, (i32, i32, metamodelica::Array<Option<(metamodelica::Ref<DAE::ComponentRef>, Option<metamodelica::Ref<DAE::Exp>>)>>), i32, (Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>, metamodelica::Ref<DAE::ComponentRef>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>) -> Result<ArcStr> + 'static>, Arc<dyn ::std::ops::Fn(Option<metamodelica::Ref<DAE::Exp>>) -> Result<ArcStr> + 'static>)), __a4: (metamodelica::List<metamodelica::Ref<DAE::Element>>, metamodelica::List<metamodelica::Ref<DAE::Element>>)| addStateActivationAndReset(&__a0, __a1, __a2, __a3, __a4), inEnclosingSMComp, inEnclosingFlatSmSemantics, crToExpOpt, (metamodelica::nil(), metamodelica::nil()))?;
            (metamodelica::cons(metamodelica::Ref::new(DAE::Element::WHEN_EQUATION { condition: condition.clone(), equations: equations1, elsewhen_: None, source: source.clone() }), Util::tuple21(accEqnsVars.clone())), listAppend(vars1, Util::tuple22(accEqnsVars)))
        },
        Deref @ DAE::Element::WHEN_EQUATION { elsewhen_: Some(_), .. } => {
            Error::addCompilerError(literal!("Encountered elsewhen part in a when clause of a clocked state machine.\n"))?;
            return Err("fail")
        },
        _ => {
            Error::addCompilerError(literal!("Internal compiler error: StateMachineFlatten.addStateActivationAndReset(..) called with unexpected argument.\n"))?;
            return Err("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outEqnsVars)
}

fn addStateActivationAndReset1(
    mut inEqn: &metamodelica::Ref<DAE::Element>,
    mut inEnclosingSMComp: &metamodelica::Ref<DAE::Element>,
    mut inEnclosingFlatSmSemantics: FlatSmSemantics,
    mut crToExpOpt: &(
        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<
                Option<(
                    metamodelica::Ref<DAE::ComponentRef>,
                    Option<metamodelica::Ref<DAE::Exp>>,
                )>,
            >,
        ),
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
            Arc<dyn ::std::ops::Fn(Option<metamodelica::Ref<DAE::Exp>>) -> Result<ArcStr> + 'static>,
        ),
    ),
    mut accEqnsVars: (
        metamodelica::List<metamodelica::Ref<DAE::Element>>,
        metamodelica::List<metamodelica::Ref<DAE::Element>>,
    ),
) -> Result<(
    metamodelica::List<metamodelica::Ref<DAE::Element>>,
    metamodelica::List<metamodelica::Ref<DAE::Element>>,
)> {
    let mut outEqnsVars: (
        metamodelica::List<metamodelica::Ref<DAE::Element>>,
        metamodelica::List<metamodelica::Ref<DAE::Element>>,
    );
    let mut stateVarCrefs: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
    let mut crefLHS: metamodelica::Ref<DAE::ComponentRef>;
    let mut enclosingStateRef: metamodelica::Ref<DAE::ComponentRef>;
    let mut cref2: metamodelica::Ref<DAE::ComponentRef>;
    let mut found: bool;
    let mut tyLHS: metamodelica::Ref<DAE::Type>;
    let mut eqn: metamodelica::Ref<DAE::Element>;
    let mut eqn1: metamodelica::Ref<DAE::Element>;
    let mut eqn2: metamodelica::Ref<DAE::Element>;
    let mut var2: metamodelica::Ref<DAE::Element>;
    let mut varDecl: metamodelica::Ref<DAE::Element>;
    let mut attr: metamodelica::Ref<DAE::CallAttributes>;
    let mut dAElist: metamodelica::List<metamodelica::Ref<DAE::Element>>;
    let mut isOuterVar: bool;
    let mut exp: metamodelica::Ref<DAE::Exp>;
    let mut scalar: metamodelica::Ref<DAE::Exp>;
    let mut scalarNew: metamodelica::Ref<DAE::Exp>;
    let mut source: metamodelica::Ref<DAE::ElementSource>;
    let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &((*inEqn)) {
        Deref @ DAE::Element::EQUATION { exp: __pa0, scalar: __pa1, source: __pa2 } => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
        _ => return Err("pattern mismatch"),
    } };
    exp = metamodelica::Own::own(__pa0);
    scalar = metamodelica::Own::own(__pa1);
    source = metamodelica::Own::own(__pa2);
    let (__pa3, __pa4) = ::match_deref::match_deref! { match &((*inEnclosingSMComp)) {
        Deref @ DAE::Element::SM_COMP { componentRef: __pa3, dAElist: __pa4 } => (__pa3.clone(), __pa4.clone()),
        _ => return Err("pattern mismatch"),
    } };
    enclosingStateRef = metamodelica::Own::own(__pa3);
    dAElist = metamodelica::Own::own(__pa4);
    stateVarCrefs = BaseHashTable::hashTableKeyList(crToExpOpt)?;
    match '__try5: {
        let (__pa6, __pa7) = ::match_deref::match_deref! { match &(exp.clone()) {
            Deref @ DAE::Exp::CREF { componentRef: __pa6, ty: __pa7 } => (__pa6.clone(), __pa7.clone()),
            _ => break '__try5 Err::<_, _>("pattern mismatch"),
        } };
        crefLHS = metamodelica::Own::own(__pa6);
        tyLHS = metamodelica::Own::own(__pa7);
        let (__pa8, (_, __pa9)) = unwrap_break_err!(Expression::traverseExpTopDown(scalar.clone(), &traversingSubsPreviousCrefs, (stateVarCrefs.clone(), false)), '__try5);
        scalarNew = metamodelica::Own::own(__pa8);
        found = metamodelica::Own::own(__pa9);
        eqn = metamodelica::Ref::new(DAE::Element::EQUATION {
            exp: exp.clone(),
            scalar: scalarNew.clone(),
            source: source.clone(),
        });
        if unwrap_break_err!(List::any(&stateVarCrefs, &({ let __pe_b0 = crefLHS.clone(); move |__pe_a1| ComponentReferenceBasics::crefEqual(&__pe_b0, &__pe_a1) })), '__try5)
        {
            eqn1 = unwrap_break_err!(wrapInStateActivationConditional(&eqn, &enclosingStateRef, true), '__try5);
            var2 = createVarWithDefaults(
                unwrap_break_err!(ComponentReference::appendStringLastIdent(&(literal!("_previous")), &crefLHS), '__try5),
                openmodelica_frontend_types::DAE::VarKind::DISCRETE,
                tyLHS.clone(),
                metamodelica::nil(),
            );
            eqn2 = unwrap_break_err!(createResetEquation(crefLHS.clone(), tyLHS.clone(), enclosingStateRef.clone(), inEnclosingFlatSmSemantics.clone(), crToExpOpt), '__try5);
            outEqnsVars = (
                metamodelica::cons(
                    eqn1.clone(),
                    metamodelica::cons(eqn2.clone(), Util::tuple21(accEqnsVars.clone())),
                ),
                metamodelica::cons(var2.clone(), Util::tuple22(accEqnsVars.clone())),
            );
        } else {
            outEqnsVars = (
                metamodelica::cons(
                    unwrap_break_err!(wrapInStateActivationConditional(&eqn, &enclosingStateRef, false), '__try5),
                    Util::tuple21(accEqnsVars.clone()),
                ),
                Util::tuple22(accEqnsVars.clone()),
            );
        }
        Ok::<_, &'static str>((crefLHS.clone(), outEqnsVars.clone(), tyLHS.clone()))
    } {
        Ok((__try5_o0, __try5_o1, __try5_o2)) => {
            crefLHS = __try5_o0;
            outEqnsVars = __try5_o1;
            tyLHS = __try5_o2;
        }
        Err(_) => {
            match '__try10: {
                if unwrap_break_err!(Flags::getConfigBool(Flags::CT_STATE_MACHINES.clone()), '__try10) {
                    let (__pa11, __pa12, __pa13) = ::match_deref::match_deref! { match &(exp.clone()) {
                        Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "der" }, expLst: Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Exp::CREF { componentRef: __pa11, ty: __pa12 }, tail: Deref @ metamodelica::ListNode::Nil }, attr: __pa13 } => (__pa11.clone(), __pa12.clone(), __pa13.clone()),
                        _ => break '__try10 Err::<_, _>("pattern mismatch"),
                    } };
                    crefLHS = metamodelica::Own::own(__pa11);
                    tyLHS = metamodelica::Own::own(__pa12);
                    attr = metamodelica::Own::own(__pa13);
                    if let Ok(__iflet16) = List::find1(
                        &dAElist,
                        &move |__a0: metamodelica::Ref<DAE::Element>, __a1: metamodelica::Ref<DAE::ComponentRef>| {
                            isCrefInVar(&__a0, &__a1)
                        },
                        crefLHS.clone(),
                    ) {
                        varDecl = __iflet16;
                    } else {
                        unwrap_break_err!(Error::addCompilerError({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Couldn't find variable declaration matching to cref ")); __mm_s.push_str(&*unwrap_break_err!(ComponentReference::crefStr(&crefLHS), '__try10)); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) }), '__try10);
                        break '__try10 Err::<_, _>("fail");
                    }
                    isOuterVar = DAEUtil::isOuterVar(&varDecl);
                    if isOuterVar {
                        cref2 = unwrap_break_err!(ComponentReference::appendStringLastIdent(&(literal!("_der$")), &crefLHS), '__try10);
                        var2 = createVarWithDefaults(
                            cref2.clone(),
                            openmodelica_frontend_types::DAE::VarKind::VARIABLE,
                            tyLHS.clone(),
                            metamodelica::nil(),
                        );
                        eqn1 = metamodelica::Ref::new(DAE::Element::EQUATION {
                            exp: metamodelica::Ref::new(DAE::Exp::CREF {
                                componentRef: cref2.clone(),
                                ty: tyLHS.clone(),
                            }),
                            scalar: scalar.clone(),
                            source: source.clone(),
                        });
                        outEqnsVars = (
                            metamodelica::cons(eqn1.clone(), Util::tuple21(accEqnsVars.clone())),
                            metamodelica::cons(var2.clone(), Util::tuple22(accEqnsVars.clone())),
                        );
                    } else {
                        eqn1 =
                            unwrap_break_err!(wrapInStateActivationConditionalCT(inEqn, &enclosingStateRef), '__try10);
                        eqn2 = unwrap_break_err!(createResetEquationCT(crefLHS.clone(), &tyLHS, enclosingStateRef.clone(), inEnclosingFlatSmSemantics.clone(), crToExpOpt), '__try10);
                        outEqnsVars = (
                            metamodelica::cons(
                                eqn1.clone(),
                                metamodelica::cons(eqn2.clone(), Util::tuple21(accEqnsVars.clone())),
                            ),
                            Util::tuple22(accEqnsVars.clone()),
                        );
                    }
                } else {
                    break '__try10 Err::<_, _>("fail");
                }
                Ok::<_, &'static str>((
                    attr.clone(),
                    crefLHS.clone(),
                    eqn1.clone(),
                    isOuterVar.clone(),
                    outEqnsVars.clone(),
                    tyLHS.clone(),
                    varDecl.clone(),
                ))
            } {
                Ok((__try10_o0, __try10_o1, __try10_o2, __try10_o3, __try10_o4, __try10_o5, __try10_o6)) => {
                    attr = __try10_o0;
                    crefLHS = __try10_o1;
                    eqn1 = __try10_o2;
                    isOuterVar = __try10_o3;
                    outEqnsVars = __try10_o4;
                    tyLHS = __try10_o5;
                    varDecl = __try10_o6;
                }
                Err(__try10_err) => {
                    if Flags::getConfigBool(Flags::CT_STATE_MACHINES.clone())? {
                        Error::addCompilerError(literal!(
                            "Currently, only equations in state machines with a LHS component reference, e.g., x=.., or its derivative, e.g., der(x)=.., are supported"
                        ))?;
                    } else {
                        Error::addCompilerError(literal!(
                            "Currently, only equations in state machines with a LHS component reference, e.g., x=.., are supported"
                        ))?;
                    }
                    return Err(__try10_err);
                }
            }
        }
    }
    Ok(outEqnsVars)
}

fn isVarAtLHS(mut eqn: &metamodelica::Ref<DAE::Element>, mut var: metamodelica::Ref<DAE::Element>) -> Result<bool> {
    let mut res: bool;
    let mut cref: metamodelica::Ref<DAE::ComponentRef>;
    let mut crefLHS: metamodelica::Ref<DAE::ComponentRef>;
    let mut tyLHS: metamodelica::Ref<DAE::Type>;
    let mut exp: metamodelica::Ref<DAE::Exp>;
    let mut equations: metamodelica::List<metamodelica::Ref<DAE::Element>>;
    let mut elsewhen_: Option<metamodelica::Ref<DAE::Element>>;
    res = (::match_deref::match_deref! { match eqn {
        Deref @ DAE::Element::EQUATION { exp: __esc_exp, scalar: _, source: _ } => {
            exp = (*__esc_exp).clone();
            cref = DAEUtil::varCref(&var)?;
            match '__try0: {
                let (__pa1, __pa2) = ::match_deref::match_deref! { match &(exp.clone()) {
                    Deref @ DAE::Exp::CREF { componentRef: __pa1, ty: __pa2 } => (__pa1.clone(), __pa2.clone()),
                    _ => break '__try0 Err::<_, _>("pattern mismatch"),
                } };
                crefLHS = metamodelica::Own::own(__pa1);
                tyLHS = metamodelica::Own::own(__pa2);
                res = unwrap_break_err!(ComponentReferenceBasics::crefEqual(&crefLHS, &cref), '__try0);
                Ok::<_, &'static str>((res.clone(),))
            } {
                Ok((__try0_o0,)) => {
                    res = __try0_o0;
                }
                Err(_) => {
                    res = false;
                }
            }
            res
        },
        Deref @ DAE::Element::WHEN_EQUATION { equations: __esc_equations, elsewhen_: None, .. } => {
            equations = (*__esc_equations).clone();
            List::exist1(metamodelica::AsArg::as_arg(&equations), &move |__a0: metamodelica::Ref<DAE::Element>, __a1: metamodelica::Ref<DAE::Element>| isVarAtLHS(&__a0, __a1), var)?
        },
        Deref @ DAE::Element::WHEN_EQUATION { elsewhen_: Some(_), .. } => {
            Error::addCompilerError(literal!("Encountered elsewhen part in a when clause of a clocked state machine.\n"))?;
            return Err("fail")
        },
        _ => {
            Error::addCompilerError(literal!("Internal compiler error: StateMachineFlatten.isVarAtLHS(..) called with unexpected argument.\n"))?;
            return Err("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(res)
}

fn isPreviousAppliedToVar(
    mut eqn: &metamodelica::Ref<DAE::Element>,
    mut var: metamodelica::Ref<DAE::Element>,
) -> Result<bool> {
    let mut found: bool = false;
    let mut cref: metamodelica::Ref<DAE::ComponentRef>;
    let mut scalar: metamodelica::Ref<DAE::Exp>;
    let mut equations: metamodelica::List<metamodelica::Ref<DAE::Element>>;
    let mut elsewhen_: Option<metamodelica::Ref<DAE::Element>>;
    found = (::match_deref::match_deref! { match eqn {
        Deref @ DAE::Element::EQUATION { exp: _, scalar: __esc_scalar, source: _ } => {
            scalar = (*__esc_scalar).clone();
            cref = DAEUtil::varCref(&var)?;
            let (_, (_, __pa0)) = Expression::traverseExpTopDown(scalar.clone(), &traversingFindPreviousCref, (cref, false))?;
            found = metamodelica::Own::own(__pa0);
            found
        },
        Deref @ DAE::Element::WHEN_EQUATION { equations: __esc_equations, elsewhen_: None, .. } => {
            equations = (*__esc_equations).clone();
            List::exist1(metamodelica::AsArg::as_arg(&equations), &move |__a0: metamodelica::Ref<DAE::Element>, __a1: metamodelica::Ref<DAE::Element>| isPreviousAppliedToVar(&__a0, __a1), var)?
        },
        Deref @ DAE::Element::WHEN_EQUATION { elsewhen_: Some(_), .. } => {
            Error::addCompilerError(literal!("Encountered elsewhen part in a when clause of a clocked state machine.\n"))?;
            return Err("fail")
        },
        _ => {
            Error::addCompilerError(literal!("Internal compiler error: StateMachineFlatten.isPreviousAppliedToVar(..) called with unexpected argument.\n"))?;
            return Err("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(found)
}

fn traversingFindPreviousCref(
    mut inExp: metamodelica::Ref<DAE::Exp>,
    mut inCrefHit: (metamodelica::Ref<DAE::ComponentRef>, bool),
) -> Result<(
    metamodelica::Ref<DAE::Exp>,
    bool,
    (metamodelica::Ref<DAE::ComponentRef>, bool),
)> {
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    let mut cont: bool = true;
    let mut outCrefHit: (metamodelica::Ref<DAE::ComponentRef>, bool);
    (outExp, outCrefHit) = (::match_deref::match_deref! { match &((inExp.clone(), inCrefHit.clone())) {
        (Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "previous" }, expLst: Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Exp::CREF { componentRef: cr, ty: _ }, tail: Deref @ metamodelica::ListNode::Nil }, attr: _ }, (cref, _)) if (ComponentReferenceBasics::crefEqual(metamodelica::AsArg::as_arg(&cr), metamodelica::AsArg::as_arg(&cref))?) => {
            (inExp, (cref.clone(), true))
        },
        _ => {
            (inExp, inCrefHit)
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outExp, cont, outCrefHit))
}

fn createResetEquationCT(
    mut inLHSCref: metamodelica::Ref<DAE::ComponentRef>,
    mut inLHSty: &metamodelica::Ref<DAE::Type>,
    mut inStateCref: metamodelica::Ref<DAE::ComponentRef>,
    mut inEnclosingFlatSmSemantics: FlatSmSemantics,
    mut crToExpOpt: &(
        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<
                Option<(
                    metamodelica::Ref<DAE::ComponentRef>,
                    Option<metamodelica::Ref<DAE::Exp>>,
                )>,
            >,
        ),
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
            Arc<dyn ::std::ops::Fn(Option<metamodelica::Ref<DAE::Exp>>) -> Result<ArcStr> + 'static>,
        ),
    ),
) -> Result<metamodelica::Ref<DAE::Element>> {
    let mut outEqn: metamodelica::Ref<DAE::Element>;
    let mut activeExp: metamodelica::Ref<DAE::Exp>;
    let mut activeResetExp: metamodelica::Ref<DAE::Exp>;
    let mut activeResetStatesExp: metamodelica::Ref<DAE::Exp>;
    let mut orExp: metamodelica::Ref<DAE::Exp>;
    let mut andExp: metamodelica::Ref<DAE::Exp>;
    let mut startValueExp: metamodelica::Ref<DAE::Exp>;
    let mut reinitElem: metamodelica::Ref<DAE::Element>;
    let mut startValueOpt: Option<metamodelica::Ref<DAE::Exp>>;
    let mut initStateRef: metamodelica::Ref<DAE::ComponentRef>;
    let mut preRef: metamodelica::Ref<DAE::ComponentRef>;
    let mut i: i32;
    let mut nStates: i32;
    let mut enclosingFlatSMComps: metamodelica::Array<metamodelica::Ref<DAE::Element>>;
    let mut tArrayBool: metamodelica::Ref<DAE::Type>;
    let FlatSmSemantics { smComps: __pa0, .. } = inEnclosingFlatSmSemantics;
    enclosingFlatSMComps = metamodelica::Own::own(__pa0);
    let __pa1 = ::match_deref::match_deref! { match &(metamodelica::arrayGet(enclosingFlatSMComps.clone(), 1)?) {
        Deref @ DAE::Element::SM_COMP { componentRef: __pa1, .. } => __pa1.clone(),
        _ => return Err("pattern mismatch"),
    } };
    initStateRef = metamodelica::Own::own(__pa1);
    preRef = ComponentReference::crefPrefixString(arcstr::literal!(SMS_PRE), initStateRef);
    i = List::position1OnTrue(
        &(enclosingFlatSMComps
            .clone()
            .borrow()
            .iter()
            .cloned()
            .collect::<metamodelica::List<_>>()),
        &move |__a0: metamodelica::Ref<DAE::Element>, __a1: metamodelica::Ref<DAE::ComponentRef>| {
            sMCompEqualsRef(&__a0, &__a1)
        },
        inStateCref.clone(),
    )?;
    activeResetExp = metamodelica::Ref::new(DAE::Exp::CREF {
        componentRef: qCref(
            literal!("activeReset"),
            DAE::T_BOOL_DEFAULT().clone(),
            metamodelica::nil(),
            &preRef,
        )?,
        ty: DAE::T_BOOL_DEFAULT().clone(),
    });
    nStates = metamodelica::arrayLength(enclosingFlatSMComps.clone());
    tArrayBool = metamodelica::Ref::new(DAE::Type::T_ARRAY {
        ty: DAE::T_BOOL_DEFAULT().clone(),
        dims: list![metamodelica::Ref::new(DAE::Dimension::DIM_INTEGER { integer: nStates })],
    });
    activeResetStatesExp = metamodelica::Ref::new(DAE::Exp::CREF {
        componentRef: qCref(
            literal!("activeResetStates"),
            tArrayBool,
            list![metamodelica::Ref::new(DAE::Subscript::INDEX {
                exp: metamodelica::Ref::new(DAE::Exp::ICONST { integer: i })
            })],
            &preRef,
        )?,
        ty: DAE::T_BOOL_DEFAULT().clone(),
    });
    orExp = metamodelica::Ref::new(DAE::Exp::LBINARY {
        exp1: activeResetExp,
        operator: DAE::Operator::OR {
            ty: DAE::T_BOOL_DEFAULT().clone(),
        },
        exp2: activeResetStatesExp,
    });
    activeExp = metamodelica::Ref::new(DAE::Exp::CREF {
        componentRef: qCref(
            literal!("active"),
            DAE::T_BOOL_DEFAULT().clone(),
            metamodelica::nil(),
            &inStateCref,
        )?,
        ty: DAE::T_BOOL_DEFAULT().clone(),
    });
    andExp = metamodelica::Ref::new(DAE::Exp::LBINARY {
        exp1: activeExp,
        operator: DAE::Operator::AND {
            ty: DAE::T_BOOL_DEFAULT().clone(),
        },
        exp2: orExp,
    });
    startValueOpt = BaseHashTable::get(inLHSCref.clone(), crToExpOpt)?;
    if (startValueOpt).is_some() {
        startValueExp = Util::getOption(startValueOpt)?;
    } else {
        startValueExp = (match &**inLHSty {
            DAE::Type::T_INTEGER { .. } => {
                Error::addCompilerWarning({
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!("Variable "));
                    __mm_s.push_str(&*ComponentReference::crefStr(&inLHSCref)?);
                    __mm_s.push_str(&*literal!(" lacks start value. Defaulting to start=0.\n"));
                    ArcStr::from(__mm_s)
                })?;
                metamodelica::Ref::new(DAE::Exp::ICONST { integer: 0 })
            }
            DAE::Type::T_REAL { .. } => {
                Error::addCompilerWarning({
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!("Variable "));
                    __mm_s.push_str(&*ComponentReference::crefStr(&inLHSCref)?);
                    __mm_s.push_str(&*literal!(" lacks start value. Defaulting to start=0.\n"));
                    ArcStr::from(__mm_s)
                })?;
                metamodelica::Ref::new(DAE::Exp::RCONST {
                    real: metamodelica::OrderedFloat((0) as f64),
                })
            }
            DAE::Type::T_BOOL { .. } => {
                Error::addCompilerWarning({
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!("Variable "));
                    __mm_s.push_str(&*ComponentReference::crefStr(&inLHSCref)?);
                    __mm_s.push_str(&*literal!(" lacks start value. Defaulting to start=false.\n"));
                    ArcStr::from(__mm_s)
                })?;
                metamodelica::Ref::new(DAE::Exp::BCONST { bool: false })
            }
            DAE::Type::T_STRING { .. } => {
                Error::addCompilerWarning({
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!("Variable "));
                    __mm_s.push_str(&*ComponentReference::crefStr(&inLHSCref)?);
                    __mm_s.push_str(&*literal!(" lacks start value. Defaulting to start=\"\".\n"));
                    ArcStr::from(__mm_s)
                })?;
                metamodelica::Ref::new(DAE::Exp::SCONST { string: literal!("") })
            }
            DAE::Type::T_ENUMERATION { .. } => {
                Error::addCompilerWarning({
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!("Variable "));
                    __mm_s.push_str(&*ComponentReference::crefStr(&inLHSCref)?);
                    __mm_s.push_str(&*literal!(" lacks start value. Defaulting to start=\"\".\n"));
                    ArcStr::from(__mm_s)
                })?;
                Types::getNthEnumLiteral(inLHSty, 1)?
            }
            _ => {
                Error::addCompilerError({
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!("Variable "));
                    __mm_s.push_str(&*ComponentReference::crefStr(&inLHSCref)?);
                    __mm_s.push_str(&*literal!(" lacks start value.\n"));
                    ArcStr::from(__mm_s)
                })?;
                return Err("fail");
            }
        });
    }
    reinitElem = metamodelica::Ref::new(DAE::Element::REINIT {
        componentRef: inLHSCref,
        exp: startValueExp,
        source: DAE::emptyElementSource().clone(),
    });
    outEqn = metamodelica::Ref::new(DAE::Element::WHEN_EQUATION {
        condition: andExp,
        equations: list![reinitElem],
        elsewhen_: None,
        source: DAE::emptyElementSource().clone(),
    });
    Ok(outEqn)
}

fn isCrefInVar(
    mut inElement: &metamodelica::Ref<DAE::Element>,
    mut inCref: &metamodelica::Ref<DAE::ComponentRef>,
) -> Result<bool> {
    let mut result: bool;
    result = (match &**inElement {
        DAE::Element::VAR { componentRef: cref, .. } if (ComponentReferenceBasics::crefEqual(cref, inCref)?) => true,
        _ => false,
    });
    Ok(result)
}

fn createResetEquation(
    mut inLHSCref: metamodelica::Ref<DAE::ComponentRef>,
    mut inLHSty: metamodelica::Ref<DAE::Type>,
    mut inStateCref: metamodelica::Ref<DAE::ComponentRef>,
    mut inEnclosingFlatSmSemantics: FlatSmSemantics,
    mut crToExpOpt: &(
        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<
                Option<(
                    metamodelica::Ref<DAE::ComponentRef>,
                    Option<metamodelica::Ref<DAE::Exp>>,
                )>,
            >,
        ),
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
            Arc<dyn ::std::ops::Fn(Option<metamodelica::Ref<DAE::Exp>>) -> Result<ArcStr> + 'static>,
        ),
    ),
) -> Result<metamodelica::Ref<DAE::Element>> {
    let mut outEqn: metamodelica::Ref<DAE::Element>;
    let mut activeExp: metamodelica::Ref<DAE::Exp>;
    let mut lhsExp: metamodelica::Ref<DAE::Exp>;
    let mut activeResetExp: metamodelica::Ref<DAE::Exp>;
    let mut activeResetStatesExp: metamodelica::Ref<DAE::Exp>;
    let mut orExp: metamodelica::Ref<DAE::Exp>;
    let mut andExp: metamodelica::Ref<DAE::Exp>;
    let mut previousExp: metamodelica::Ref<DAE::Exp>;
    let mut startValueExp: metamodelica::Ref<DAE::Exp>;
    let mut ifExp: metamodelica::Ref<DAE::Exp>;
    let mut startValueOpt: Option<metamodelica::Ref<DAE::Exp>>;
    let mut initStateRef: metamodelica::Ref<DAE::ComponentRef>;
    let mut preRef: metamodelica::Ref<DAE::ComponentRef>;
    let mut i: i32;
    let mut nStates: i32;
    let mut enclosingFlatSMComps: metamodelica::Array<metamodelica::Ref<DAE::Element>>;
    let mut tArrayBool: metamodelica::Ref<DAE::Type>;
    let mut callAttributes: metamodelica::Ref<DAE::CallAttributes>;
    let FlatSmSemantics { smComps: __pa0, .. } = inEnclosingFlatSmSemantics;
    enclosingFlatSMComps = metamodelica::Own::own(__pa0);
    let __pa1 = ::match_deref::match_deref! { match &(metamodelica::arrayGet(enclosingFlatSMComps.clone(), 1)?) {
        Deref @ DAE::Element::SM_COMP { componentRef: __pa1, .. } => __pa1.clone(),
        _ => return Err("pattern mismatch"),
    } };
    initStateRef = metamodelica::Own::own(__pa1);
    preRef = ComponentReference::crefPrefixString(arcstr::literal!(SMS_PRE), initStateRef);
    i = List::position1OnTrue(
        &(enclosingFlatSMComps
            .clone()
            .borrow()
            .iter()
            .cloned()
            .collect::<metamodelica::List<_>>()),
        &move |__a0: metamodelica::Ref<DAE::Element>, __a1: metamodelica::Ref<DAE::ComponentRef>| {
            sMCompEqualsRef(&__a0, &__a1)
        },
        inStateCref.clone(),
    )?;
    activeResetExp = metamodelica::Ref::new(DAE::Exp::CREF {
        componentRef: qCref(
            literal!("activeReset"),
            DAE::T_BOOL_DEFAULT().clone(),
            metamodelica::nil(),
            &preRef,
        )?,
        ty: DAE::T_BOOL_DEFAULT().clone(),
    });
    nStates = metamodelica::arrayLength(enclosingFlatSMComps.clone());
    tArrayBool = metamodelica::Ref::new(DAE::Type::T_ARRAY {
        ty: DAE::T_BOOL_DEFAULT().clone(),
        dims: list![metamodelica::Ref::new(DAE::Dimension::DIM_INTEGER { integer: nStates })],
    });
    activeResetStatesExp = metamodelica::Ref::new(DAE::Exp::CREF {
        componentRef: qCref(
            literal!("activeResetStates"),
            tArrayBool,
            list![metamodelica::Ref::new(DAE::Subscript::INDEX {
                exp: metamodelica::Ref::new(DAE::Exp::ICONST { integer: i })
            })],
            &preRef,
        )?,
        ty: DAE::T_BOOL_DEFAULT().clone(),
    });
    orExp = metamodelica::Ref::new(DAE::Exp::LBINARY {
        exp1: activeResetExp,
        operator: DAE::Operator::OR {
            ty: DAE::T_BOOL_DEFAULT().clone(),
        },
        exp2: activeResetStatesExp,
    });
    activeExp = metamodelica::Ref::new(DAE::Exp::CREF {
        componentRef: qCref(
            literal!("active"),
            DAE::T_BOOL_DEFAULT().clone(),
            metamodelica::nil(),
            &inStateCref,
        )?,
        ty: DAE::T_BOOL_DEFAULT().clone(),
    });
    andExp = metamodelica::Ref::new(DAE::Exp::LBINARY {
        exp1: activeExp,
        operator: DAE::Operator::AND {
            ty: DAE::T_BOOL_DEFAULT().clone(),
        },
        exp2: orExp,
    });
    callAttributes = metamodelica::Ref::new(DAE::CallAttributes {
        ty: inLHSty.clone(),
        tuple_: false,
        builtin: true,
        isImpure: false,
        isFunctionPointerCall: false,
        inlineType: openmodelica_frontend_types::DAE::InlineType::NO_INLINE,
        tailCall: openmodelica_frontend_types::DAE::TailCall::NO_TAIL,
        noReturn: DAE::NoReturn::RETURNS.clone(),
    });
    previousExp = metamodelica::Ref::new(DAE::Exp::CALL {
        path: metamodelica::Ref::new(Absyn::Path::IDENT {
            name: literal!("previous"),
        }),
        expLst: list![metamodelica::Ref::new(DAE::Exp::CREF {
            componentRef: inLHSCref.clone(),
            ty: inLHSty.clone()
        })],
        attr: callAttributes,
    });
    startValueOpt = BaseHashTable::get(inLHSCref.clone(), crToExpOpt)?;
    if (startValueOpt).is_some() {
        startValueExp = Util::getOption(startValueOpt)?;
    } else {
        startValueExp = (match &*inLHSty {
            DAE::Type::T_INTEGER { .. } => {
                Error::addCompilerWarning({
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!("Variable "));
                    __mm_s.push_str(&*ComponentReference::crefStr(&inLHSCref)?);
                    __mm_s.push_str(&*literal!(" lacks start value. Defaulting to start=0.\n"));
                    ArcStr::from(__mm_s)
                })?;
                metamodelica::Ref::new(DAE::Exp::ICONST { integer: 0 })
            }
            DAE::Type::T_REAL { .. } => {
                Error::addCompilerWarning({
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!("Variable "));
                    __mm_s.push_str(&*ComponentReference::crefStr(&inLHSCref)?);
                    __mm_s.push_str(&*literal!(" lacks start value. Defaulting to start=0.\n"));
                    ArcStr::from(__mm_s)
                })?;
                metamodelica::Ref::new(DAE::Exp::RCONST {
                    real: metamodelica::OrderedFloat((0) as f64),
                })
            }
            DAE::Type::T_BOOL { .. } => {
                Error::addCompilerWarning({
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!("Variable "));
                    __mm_s.push_str(&*ComponentReference::crefStr(&inLHSCref)?);
                    __mm_s.push_str(&*literal!(" lacks start value. Defaulting to start=false.\n"));
                    ArcStr::from(__mm_s)
                })?;
                metamodelica::Ref::new(DAE::Exp::BCONST { bool: false })
            }
            DAE::Type::T_STRING { .. } => {
                Error::addCompilerWarning({
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!("Variable "));
                    __mm_s.push_str(&*ComponentReference::crefStr(&inLHSCref)?);
                    __mm_s.push_str(&*literal!(" lacks start value. Defaulting to start=\"\".\n"));
                    ArcStr::from(__mm_s)
                })?;
                metamodelica::Ref::new(DAE::Exp::SCONST { string: literal!("") })
            }
            DAE::Type::T_ENUMERATION { .. } => {
                Error::addCompilerWarning({
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!("Variable "));
                    __mm_s.push_str(&*ComponentReference::crefStr(&inLHSCref)?);
                    __mm_s.push_str(&*literal!(" lacks start value. Defaulting to start=\"\".\n"));
                    ArcStr::from(__mm_s)
                })?;
                Types::getNthEnumLiteral(&inLHSty, 1)?
            }
            _ => {
                Error::addCompilerError({
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!("Variable "));
                    __mm_s.push_str(&*ComponentReference::crefStr(&inLHSCref)?);
                    __mm_s.push_str(&*literal!(" lacks start value.\n"));
                    ArcStr::from(__mm_s)
                })?;
                return Err("fail");
            }
        });
    }
    ifExp = metamodelica::Ref::new(DAE::Exp::IFEXP {
        expCond: andExp,
        expThen: startValueExp,
        expElse: previousExp,
    });
    lhsExp = metamodelica::Ref::new(DAE::Exp::CREF {
        componentRef: ComponentReference::appendStringLastIdent(&(literal!("_previous")), &inLHSCref)?,
        ty: inLHSty,
    });
    outEqn = metamodelica::Ref::new(DAE::Element::EQUATION {
        exp: lhsExp,
        scalar: ifExp,
        source: DAE::emptyElementSource().clone(),
    });
    Ok(outEqn)
}

fn wrapInStateActivationConditional(
    mut inEqn: &metamodelica::Ref<DAE::Element>,
    mut inStateCref: &metamodelica::Ref<DAE::ComponentRef>,
    mut isResetEquation: bool,
) -> Result<metamodelica::Ref<DAE::Element>> {
    let mut outEqn: metamodelica::Ref<DAE::Element>;
    let mut exp: metamodelica::Ref<DAE::Exp>;
    let mut scalar: metamodelica::Ref<DAE::Exp>;
    let mut scalar1: metamodelica::Ref<DAE::Exp>;
    let mut activeRef: metamodelica::Ref<DAE::Exp>;
    let mut expElse: metamodelica::Ref<DAE::Exp>;
    let mut ty: metamodelica::Ref<DAE::Type>;
    let mut callAttributes: metamodelica::Ref<DAE::CallAttributes>;
    let mut source: metamodelica::Ref<DAE::ElementSource>;
    let mut cref: metamodelica::Ref<DAE::ComponentRef>;
    let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &((*inEqn)) {
        Deref @ DAE::Element::EQUATION { exp: __pa0, scalar: __pa1, source: __pa2 } => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
        _ => return Err("pattern mismatch"),
    } };
    exp = metamodelica::Own::own(__pa0);
    scalar = metamodelica::Own::own(__pa1);
    source = metamodelica::Own::own(__pa2);
    match '__try3: {
        let (__pa4, __pa5) = ::match_deref::match_deref! { match &(exp.clone()) {
            Deref @ DAE::Exp::CREF { componentRef: __pa4, ty: __pa5 } => (__pa4.clone(), __pa5.clone()),
            _ => break '__try3 Err::<_, _>("pattern mismatch"),
        } };
        cref = metamodelica::Own::own(__pa4);
        ty = metamodelica::Own::own(__pa5);
        Ok::<_, &'static str>((cref.clone(), ty.clone()))
    } {
        Ok((__try3_o0, __try3_o1)) => {
            cref = __try3_o0;
            ty = __try3_o1;
        }
        Err(__try3_err) => {
            Error::addCompilerError(literal!(
                "The LHS of equations in state machines needs to be a component reference"
            ))?;
            return Err(__try3_err);
        }
    }
    activeRef = metamodelica::Ref::new(DAE::Exp::CREF {
        componentRef: qCref(
            literal!("active"),
            DAE::T_BOOL_DEFAULT().clone(),
            metamodelica::nil(),
            inStateCref,
        )?,
        ty: DAE::T_BOOL_DEFAULT().clone(),
    });
    callAttributes = metamodelica::Ref::new(DAE::CallAttributes {
        ty: ty.clone(),
        tuple_: false,
        builtin: true,
        isImpure: false,
        isFunctionPointerCall: false,
        inlineType: openmodelica_frontend_types::DAE::InlineType::NO_INLINE,
        tailCall: openmodelica_frontend_types::DAE::TailCall::NO_TAIL,
        noReturn: DAE::NoReturn::RETURNS.clone(),
    });
    if isResetEquation {
        expElse = metamodelica::Ref::new(DAE::Exp::CREF {
            componentRef: ComponentReference::appendStringLastIdent(&(literal!("_previous")), &cref)?,
            ty: ty,
        });
    } else {
        expElse = metamodelica::Ref::new(DAE::Exp::CALL {
            path: metamodelica::Ref::new(Absyn::Path::IDENT {
                name: literal!("previous"),
            }),
            expLst: list![exp.clone()],
            attr: callAttributes,
        });
    }
    scalar1 = metamodelica::Ref::new(DAE::Exp::IFEXP {
        expCond: activeRef,
        expThen: scalar,
        expElse: expElse,
    });
    outEqn = metamodelica::Ref::new(DAE::Element::EQUATION {
        exp: exp,
        scalar: scalar1,
        source: source,
    });
    Ok(outEqn)
}

fn wrapInStateActivationConditionalCT(
    mut inEqn: &metamodelica::Ref<DAE::Element>,
    mut inStateCref: &metamodelica::Ref<DAE::ComponentRef>,
) -> Result<metamodelica::Ref<DAE::Element>> {
    let mut outEqn: metamodelica::Ref<DAE::Element>;
    let mut exp: metamodelica::Ref<DAE::Exp>;
    let mut scalar: metamodelica::Ref<DAE::Exp>;
    let mut scalar1: metamodelica::Ref<DAE::Exp>;
    let mut activeRef: metamodelica::Ref<DAE::Exp>;
    let mut expElse: metamodelica::Ref<DAE::Exp>;
    let mut ty: metamodelica::Ref<DAE::Type>;
    let mut callAttributes: metamodelica::Ref<DAE::CallAttributes>;
    let mut source: metamodelica::Ref<DAE::ElementSource>;
    let mut cref: metamodelica::Ref<DAE::ComponentRef>;
    let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &((*inEqn)) {
        Deref @ DAE::Element::EQUATION { exp: __pa0, scalar: __pa1, source: __pa2 } => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
        _ => return Err("pattern mismatch"),
    } };
    exp = metamodelica::Own::own(__pa0);
    scalar = metamodelica::Own::own(__pa1);
    source = metamodelica::Own::own(__pa2);
    match '__try3: {
        let (__pa4, __pa5) = ::match_deref::match_deref! { match &(exp.clone()) {
            Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "der" }, expLst: Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Exp::CREF { componentRef: __pa4, ty: __pa5 }, tail: Deref @ metamodelica::ListNode::Nil }, attr: _ } => (__pa4.clone(), __pa5.clone()),
            _ => break '__try3 Err::<_, _>("pattern mismatch"),
        } };
        cref = metamodelica::Own::own(__pa4);
        ty = metamodelica::Own::own(__pa5);
        Ok::<_, &'static str>((cref.clone(), ty.clone()))
    } {
        Ok((__try3_o0, __try3_o1)) => {
            cref = __try3_o0;
            ty = __try3_o1;
        }
        Err(__try3_err) => {
            Error::addCompilerError(literal!(
                "The LHS of equations in state machines needs to be a component reference, e.g., x = .., or its derivative, e.g., der(x) = .."
            ))?;
            return Err(__try3_err);
        }
    }
    activeRef = metamodelica::Ref::new(DAE::Exp::CREF {
        componentRef: qCref(
            literal!("active"),
            DAE::T_BOOL_DEFAULT().clone(),
            metamodelica::nil(),
            inStateCref,
        )?,
        ty: DAE::T_BOOL_DEFAULT().clone(),
    });
    callAttributes = metamodelica::Ref::new(DAE::CallAttributes {
        ty: ty,
        tuple_: false,
        builtin: true,
        isImpure: false,
        isFunctionPointerCall: false,
        inlineType: openmodelica_frontend_types::DAE::InlineType::NO_INLINE,
        tailCall: openmodelica_frontend_types::DAE::TailCall::NO_TAIL,
        noReturn: DAE::NoReturn::RETURNS.clone(),
    });
    expElse = metamodelica::Ref::new(DAE::Exp::RCONST {
        real: metamodelica::OrderedFloat((0) as f64),
    });
    scalar1 = metamodelica::Ref::new(DAE::Exp::IFEXP {
        expCond: activeRef,
        expThen: scalar,
        expElse: expElse,
    });
    outEqn = metamodelica::Ref::new(DAE::Element::EQUATION {
        exp: exp,
        scalar: scalar1,
        source: source,
    });
    Ok(outEqn)
}

fn traversingSubsPreviousCref(
    mut inExp: metamodelica::Ref<DAE::Exp>,
    mut inCrefHit: (metamodelica::Ref<DAE::ComponentRef>, bool),
) -> Result<(
    metamodelica::Ref<DAE::Exp>,
    bool,
    (metamodelica::Ref<DAE::ComponentRef>, bool),
)> {
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    let mut cont: bool = true;
    let mut outCrefHit: (metamodelica::Ref<DAE::ComponentRef>, bool);
    (outExp, outCrefHit) = (::match_deref::match_deref! { match &((inExp.clone(), inCrefHit.clone())) {
        (Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "previous" }, expLst: Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Exp::CREF { componentRef: cr, ty }, tail: Deref @ metamodelica::ListNode::Nil }, attr: _ }, (cref, _)) if (ComponentReferenceBasics::crefEqual(metamodelica::AsArg::as_arg(&cr), metamodelica::AsArg::as_arg(&cref))?) => {
            let mut substituteRef: metamodelica::Ref<DAE::ComponentRef>;
            metamodelica::print({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("StateMachineFlatten.traversingSubsPreviousCref: cr: ")); __mm_s.push_str(&*ComponentReference::crefStr(metamodelica::AsArg::as_arg(&cr))?); __mm_s.push_str(&*literal!(", cref: ")); __mm_s.push_str(&*ComponentReference::crefStr(metamodelica::AsArg::as_arg(&cref))?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) });
            substituteRef = ComponentReference::appendStringLastIdent(&(literal!("_previous")), metamodelica::AsArg::as_arg(&cref))?;
            (metamodelica::Ref::new(DAE::Exp::CREF { componentRef: substituteRef, ty: ty.clone() }), (cref.clone(), true))
        },
        _ => {
            (inExp, inCrefHit)
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outExp, cont, outCrefHit))
}

fn traversingSubsPreviousCrefs(
    mut inExp: metamodelica::Ref<DAE::Exp>,
    mut inCrefsHit: (metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>, bool),
) -> Result<(
    metamodelica::Ref<DAE::Exp>,
    bool,
    (metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>, bool),
)> {
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    let mut cont: bool = true;
    let mut outCrefsHit: (metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>, bool);
    (outExp, outCrefsHit) = (::match_deref::match_deref! { match &((inExp.clone(), inCrefsHit.clone())) {
        (Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "previous" }, expLst: Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Exp::CREF { componentRef: cr, ty }, tail: Deref @ metamodelica::ListNode::Nil }, attr: _ }, (crefs, _)) if (List::any(metamodelica::AsArg::as_arg(&crefs), &({ let __pe_b0 = cr.clone(); move |__pe_a1| ComponentReferenceBasics::crefEqual(&__pe_b0, &__pe_a1) }))?) => {
            let mut substituteRef: metamodelica::Ref<DAE::ComponentRef>;
            substituteRef = ComponentReference::appendStringLastIdent(&(literal!("_previous")), metamodelica::AsArg::as_arg(&cr))?;
            (metamodelica::Ref::new(DAE::Exp::CREF { componentRef: substituteRef, ty: ty.clone() }), (crefs.clone(), true))
        },
        _ => {
            (inExp, inCrefsHit)
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outExp, cont, outCrefsHit))
}

fn getStartAttrOption(mut inElt: &metamodelica::Ref<DAE::Element>) -> Result<Option<metamodelica::Ref<DAE::Exp>>> {
    let mut outExpOpt: Option<metamodelica::Ref<DAE::Exp>>;
    let mut start: metamodelica::Ref<DAE::Exp>;
    let mut ty: metamodelica::Ref<DAE::Type>;
    let mut varAttrOpt: Option<metamodelica::Ref<DAE::VariableAttributes>>;
    let (__pa0, __pa1) = ::match_deref::match_deref! { match &((*inElt)) {
        Deref @ DAE::Element::VAR { variableAttributesOption: __pa0, ty: __pa1, .. } => (__pa0.clone(), __pa1.clone()),
        _ => return Err("pattern mismatch"),
    } };
    varAttrOpt = metamodelica::Own::own(__pa0);
    ty = metamodelica::Own::own(__pa1);
    if (varAttrOpt).is_some() {
        start = DAEUtil::getStartAttr(varAttrOpt, &ty)?;
        outExpOpt = Some(start);
    } else {
        outExpOpt = None;
    }
    Ok(outExpOpt)
}

fn addPropagationEquations(
    mut inFlatSmSemantics: FlatSmSemantics,
    mut inEnclosingStateCrefOption: Option<metamodelica::Ref<DAE::ComponentRef>>,
    mut inEnclosingFlatSmSemanticsOption: Option<FlatSmSemantics>,
) -> Result<FlatSmSemantics> {
    let mut outFlatSmSemantics: FlatSmSemantics;
    let mut preRef: metamodelica::Ref<DAE::ComponentRef>;
    let mut initStateRef: metamodelica::Ref<DAE::ComponentRef>;
    let mut initRef: metamodelica::Ref<DAE::ComponentRef>;
    let mut resetRef: metamodelica::Ref<DAE::ComponentRef>;
    let mut activeRef: metamodelica::Ref<DAE::ComponentRef>;
    let mut stateRef: metamodelica::Ref<DAE::ComponentRef>;
    let mut activePlotIndicatorRef: metamodelica::Ref<DAE::ComponentRef>;
    let mut initVar: metamodelica::Ref<DAE::Element>;
    let mut activePlotIndicatorVar: metamodelica::Ref<DAE::Element>;
    let mut ticksInStateVar: metamodelica::Ref<DAE::Element>;
    let mut timeEnteredStateVar: metamodelica::Ref<DAE::Element>;
    let mut timeInStateVar: metamodelica::Ref<DAE::Element>;
    let mut activePlotIndicatorEqn: metamodelica::Ref<DAE::Element>;
    let mut ticksInStateEqn: metamodelica::Ref<DAE::Element>;
    let mut timeEnteredStateEqn: metamodelica::Ref<DAE::Element>;
    let mut timeInStateEqn: metamodelica::Ref<DAE::Element>;
    let mut rhs: metamodelica::Ref<DAE::Exp>;
    let mut andExp: metamodelica::Ref<DAE::Exp>;
    let mut eqExp: metamodelica::Ref<DAE::Exp>;
    let mut tArrayBool: metamodelica::Ref<DAE::Type>;
    let mut tArrayInteger: metamodelica::Ref<DAE::Type>;
    let mut ident: ArcStr;
    let mut smComps: metamodelica::Array<metamodelica::Ref<DAE::Element>>;
    let mut t: metamodelica::List<Transition>;
    let mut c: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
    let mut smvars: metamodelica::List<metamodelica::Ref<DAE::Element>>;
    let mut smknowns: metamodelica::List<metamodelica::Ref<DAE::Element>>;
    let mut smeqs: metamodelica::List<metamodelica::Ref<DAE::Element>>;
    let mut pvars: metamodelica::List<metamodelica::Ref<DAE::Element>> = metamodelica::nil();
    let mut peqs: metamodelica::List<metamodelica::Ref<DAE::Element>> = metamodelica::nil();
    let mut enclosingStateCref: metamodelica::Ref<DAE::ComponentRef>;
    let mut enclosingPreRef: metamodelica::Ref<DAE::ComponentRef>;
    let mut enclosingActiveResetStateRef: metamodelica::Ref<DAE::ComponentRef>;
    let mut enclosingActiveResetRef: metamodelica::Ref<DAE::ComponentRef>;
    let mut enclosingActiveStateRef: metamodelica::Ref<DAE::ComponentRef>;
    let mut enclosingFlatSMSemantics: FlatSmSemantics;
    let mut enclosingFlatSMComps: metamodelica::Array<metamodelica::Ref<DAE::Element>>;
    let mut enclosingFlatSMInitStateRef: metamodelica::Ref<DAE::ComponentRef>;
    let mut posOfEnclosingSMComp: i32;
    let mut nStates: i32;
    let FlatSmSemantics {
        ident: __pa0,
        smComps: __pa1,
        t: __pa2,
        c: __pa3,
        vars: __pa4,
        knowns: __pa5,
        eqs: __pa6,
        ..
    } = inFlatSmSemantics;
    ident = metamodelica::Own::own(__pa0);
    smComps = metamodelica::Own::own(__pa1);
    t = metamodelica::Own::own(__pa2);
    c = metamodelica::Own::own(__pa3);
    smvars = metamodelica::Own::own(__pa4);
    smknowns = metamodelica::Own::own(__pa5);
    smeqs = metamodelica::Own::own(__pa6);
    let __pa7 = ::match_deref::match_deref! { match &(metamodelica::arrayGet(smComps.clone(), 1)?) {
        Deref @ DAE::Element::SM_COMP { componentRef: __pa7, .. } => __pa7.clone(),
        _ => return Err("pattern mismatch"),
    } };
    initStateRef = metamodelica::Own::own(__pa7);
    preRef = ComponentReference::crefPrefixString(arcstr::literal!(SMS_PRE), initStateRef);
    activeRef = qCref(
        literal!("active"),
        DAE::T_BOOL_DEFAULT().clone(),
        metamodelica::nil(),
        &preRef,
    )?;
    resetRef = qCref(
        literal!("reset"),
        DAE::T_BOOL_DEFAULT().clone(),
        metamodelica::nil(),
        &preRef,
    )?;
    if (inEnclosingFlatSmSemanticsOption).is_none() {
        initRef = qCref(
            literal!("init"),
            DAE::T_BOOL_DEFAULT().clone(),
            metamodelica::nil(),
            &preRef,
        )?;
        initVar = createVarWithDefaults(
            initRef.clone(),
            openmodelica_frontend_types::DAE::VarKind::DISCRETE,
            DAE::T_BOOL_DEFAULT().clone(),
            metamodelica::nil(),
        );
        initVar = setVarFixedStartValue(initVar, metamodelica::Ref::new(DAE::Exp::BCONST { bool: true }))?;
        pvars = metamodelica::cons(initVar, pvars);
        peqs = metamodelica::cons(
            metamodelica::Ref::new(DAE::Element::EQUATION {
                exp: metamodelica::Ref::new(DAE::Exp::CREF {
                    componentRef: initRef.clone(),
                    ty: DAE::T_BOOL_DEFAULT().clone(),
                }),
                scalar: metamodelica::Ref::new(DAE::Exp::BCONST { bool: false }),
                source: DAE::emptyElementSource().clone(),
            }),
            peqs,
        );
        rhs = metamodelica::Ref::new(DAE::Exp::CALL {
            path: metamodelica::Ref::new(Absyn::Path::IDENT {
                name: literal!("previous"),
            }),
            expLst: list![metamodelica::Ref::new(DAE::Exp::CREF {
                componentRef: initRef,
                ty: DAE::T_BOOL_DEFAULT().clone()
            })],
            attr: DAE::callAttrBuiltinImpureBool().clone(),
        });
        peqs = metamodelica::cons(
            metamodelica::Ref::new(DAE::Element::EQUATION {
                exp: metamodelica::Ref::new(DAE::Exp::CREF {
                    componentRef: resetRef,
                    ty: DAE::T_BOOL_DEFAULT().clone(),
                }),
                scalar: rhs,
                source: DAE::emptyElementSource().clone(),
            }),
            peqs,
        );
        peqs = metamodelica::cons(
            metamodelica::Ref::new(DAE::Element::EQUATION {
                exp: metamodelica::Ref::new(DAE::Exp::CREF {
                    componentRef: activeRef,
                    ty: DAE::T_BOOL_DEFAULT().clone(),
                }),
                scalar: metamodelica::Ref::new(DAE::Exp::BCONST { bool: true }),
                source: DAE::emptyElementSource().clone(),
            }),
            peqs,
        );
    } else {
        enclosingStateCref = Util::getOption(inEnclosingStateCrefOption.clone())?;
        enclosingFlatSMSemantics = Util::getOption(inEnclosingFlatSmSemanticsOption)?;
        let FlatSmSemantics { smComps: __pa8, .. } = enclosingFlatSMSemantics;
        enclosingFlatSMComps = metamodelica::Own::own(__pa8);
        let __pa9 = ::match_deref::match_deref! { match &(metamodelica::arrayGet(enclosingFlatSMComps.clone(), 1)?) {
            Deref @ DAE::Element::SM_COMP { componentRef: __pa9, .. } => __pa9.clone(),
            _ => return Err("pattern mismatch"),
        } };
        enclosingFlatSMInitStateRef = metamodelica::Own::own(__pa9);
        enclosingPreRef = ComponentReference::crefPrefixString(arcstr::literal!(SMS_PRE), enclosingFlatSMInitStateRef);
        posOfEnclosingSMComp = List::position1OnTrue(
            &(enclosingFlatSMComps
                .clone()
                .borrow()
                .iter()
                .cloned()
                .collect::<metamodelica::List<_>>()),
            &move |__a0: metamodelica::Ref<DAE::Element>, __a1: metamodelica::Ref<DAE::ComponentRef>| {
                sMCompEqualsRef(&__a0, &__a1)
            },
            enclosingStateCref,
        )?;
        nStates = metamodelica::arrayLength(enclosingFlatSMComps.clone());
        tArrayBool = metamodelica::Ref::new(DAE::Type::T_ARRAY {
            ty: DAE::T_BOOL_DEFAULT().clone(),
            dims: list![metamodelica::Ref::new(DAE::Dimension::DIM_INTEGER { integer: nStates })],
        });
        tArrayInteger = metamodelica::Ref::new(DAE::Type::T_ARRAY {
            ty: DAE::T_INTEGER_DEFAULT().clone(),
            dims: list![metamodelica::Ref::new(DAE::Dimension::DIM_INTEGER { integer: nStates })],
        });
        enclosingActiveResetStateRef = qCref(
            literal!("activeResetStates"),
            tArrayBool,
            list![metamodelica::Ref::new(DAE::Subscript::INDEX {
                exp: metamodelica::Ref::new(DAE::Exp::ICONST {
                    integer: posOfEnclosingSMComp
                })
            })],
            &enclosingPreRef,
        )?;
        enclosingActiveResetRef = qCref(
            literal!("activeReset"),
            DAE::T_BOOL_DEFAULT().clone(),
            metamodelica::nil(),
            &enclosingPreRef,
        )?;
        enclosingActiveStateRef = qCref(
            literal!("activeState"),
            DAE::T_INTEGER_DEFAULT().clone(),
            metamodelica::nil(),
            &enclosingPreRef,
        )?;
        eqExp = metamodelica::Ref::new(DAE::Exp::RELATION {
            exp1: metamodelica::Ref::new(DAE::Exp::CREF {
                componentRef: enclosingActiveStateRef.clone(),
                ty: DAE::T_INTEGER_DEFAULT().clone(),
            }),
            operator: DAE::Operator::EQUAL {
                ty: DAE::T_INTEGER_DEFAULT().clone(),
            },
            exp2: metamodelica::Ref::new(DAE::Exp::ICONST {
                integer: posOfEnclosingSMComp,
            }),
            index: -1,
            optionExpisASUB: None,
        });
        andExp = metamodelica::Ref::new(DAE::Exp::LBINARY {
            exp1: metamodelica::Ref::new(DAE::Exp::CREF {
                componentRef: enclosingActiveResetRef,
                ty: DAE::T_BOOL_DEFAULT().clone(),
            }),
            operator: DAE::Operator::AND {
                ty: DAE::T_BOOL_DEFAULT().clone(),
            },
            exp2: eqExp,
        });
        rhs = metamodelica::Ref::new(DAE::Exp::LBINARY {
            exp1: metamodelica::Ref::new(DAE::Exp::CREF {
                componentRef: enclosingActiveResetStateRef,
                ty: DAE::T_BOOL_DEFAULT().clone(),
            }),
            operator: DAE::Operator::OR {
                ty: DAE::T_BOOL_DEFAULT().clone(),
            },
            exp2: andExp,
        });
        peqs = metamodelica::cons(
            metamodelica::Ref::new(DAE::Element::EQUATION {
                exp: metamodelica::Ref::new(DAE::Exp::CREF {
                    componentRef: resetRef,
                    ty: DAE::T_BOOL_DEFAULT().clone(),
                }),
                scalar: rhs,
                source: DAE::emptyElementSource().clone(),
            }),
            peqs,
        );
        rhs = metamodelica::Ref::new(DAE::Exp::RELATION {
            exp1: metamodelica::Ref::new(DAE::Exp::CREF {
                componentRef: enclosingActiveStateRef,
                ty: DAE::T_INTEGER_DEFAULT().clone(),
            }),
            operator: DAE::Operator::EQUAL {
                ty: DAE::T_INTEGER_DEFAULT().clone(),
            },
            exp2: metamodelica::Ref::new(DAE::Exp::ICONST {
                integer: posOfEnclosingSMComp,
            }),
            index: -1,
            optionExpisASUB: None,
        });
        peqs = metamodelica::cons(
            metamodelica::Ref::new(DAE::Element::EQUATION {
                exp: metamodelica::Ref::new(DAE::Exp::CREF {
                    componentRef: activeRef,
                    ty: DAE::T_BOOL_DEFAULT().clone(),
                }),
                scalar: rhs,
                source: DAE::emptyElementSource().clone(),
            }),
            peqs,
        );
    }
    for mut i in 1..=metamodelica::arrayLength(smComps.clone()) {
        let __pa10 = ::match_deref::match_deref! { match &(metamodelica::arrayGet(smComps.clone(), i)?) {
            Deref @ DAE::Element::SM_COMP { componentRef: __pa10, .. } => __pa10.clone(),
            _ => return Err("pattern mismatch"),
        } };
        stateRef = metamodelica::Own::own(__pa10);
        (activePlotIndicatorVar, activePlotIndicatorEqn) = createActiveIndicator(&stateRef, &preRef, i)?;
        pvars = metamodelica::cons(activePlotIndicatorVar.clone(), pvars);
        peqs = metamodelica::cons(activePlotIndicatorEqn, peqs);
        let __pa11 = ::match_deref::match_deref! { match &(activePlotIndicatorVar) {
            Deref @ DAE::Element::VAR { componentRef: __pa11, .. } => __pa11.clone(),
            _ => return Err("pattern mismatch"),
        } };
        activePlotIndicatorRef = metamodelica::Own::own(__pa11);
        (ticksInStateVar, ticksInStateEqn) = createTicksInStateIndicator(&stateRef, activePlotIndicatorRef.clone())?;
        pvars = metamodelica::cons(ticksInStateVar, pvars);
        peqs = metamodelica::cons(ticksInStateEqn, peqs);
        (timeEnteredStateVar, timeEnteredStateEqn) =
            createTimeEnteredStateIndicator(&stateRef, activePlotIndicatorRef.clone())?;
        (timeInStateVar, timeInStateEqn) =
            createTimeInStateIndicator(&stateRef, activePlotIndicatorRef, &timeEnteredStateVar)?;
        pvars = metamodelica::cons(timeEnteredStateVar, metamodelica::cons(timeInStateVar, pvars));
        peqs = metamodelica::cons(timeEnteredStateEqn, metamodelica::cons(timeInStateEqn, peqs));
    }
    outFlatSmSemantics = FlatSmSemantics {
        ident: ident,
        smComps: smComps.clone(),
        t: t,
        c: c,
        vars: smvars,
        knowns: smknowns,
        eqs: smeqs,
        pvars: pvars,
        peqs: peqs,
        enclosingState: inEnclosingStateCrefOption,
    };
    Ok(outFlatSmSemantics)
}

fn createTimeInStateIndicator(
    mut stateRef: &metamodelica::Ref<DAE::ComponentRef>,
    mut stateActiveRef: metamodelica::Ref<DAE::ComponentRef>,
    mut timeEnteredStateVar: &metamodelica::Ref<DAE::Element>,
) -> Result<(metamodelica::Ref<DAE::Element>, metamodelica::Ref<DAE::Element>)> {
    let mut timeInStateVar: metamodelica::Ref<DAE::Element>;
    let mut timeInStateEqn: metamodelica::Ref<DAE::Element>;
    let mut timeInStateRef: metamodelica::Ref<DAE::ComponentRef>;
    let mut timeEnteredStateRef: metamodelica::Ref<DAE::ComponentRef>;
    let mut ty: metamodelica::Ref<DAE::Type>;
    let mut timeInStateExp: metamodelica::Ref<DAE::Exp>;
    let mut timeEnteredStateExp: metamodelica::Ref<DAE::Exp>;
    let mut stateActiveExp: metamodelica::Ref<DAE::Exp>;
    let mut expCond: metamodelica::Ref<DAE::Exp>;
    let mut expSampleTime: metamodelica::Ref<DAE::Exp>;
    let mut expThen: metamodelica::Ref<DAE::Exp>;
    let mut expElse: metamodelica::Ref<DAE::Exp>;
    timeInStateRef = qCref(
        literal!("$timeInState"),
        DAE::T_REAL_DEFAULT().clone(),
        metamodelica::nil(),
        stateRef,
    )?;
    timeInStateVar = createVarWithDefaults(
        timeInStateRef.clone(),
        openmodelica_frontend_types::DAE::VarKind::DISCRETE,
        DAE::T_REAL_DEFAULT().clone(),
        metamodelica::nil(),
    );
    timeInStateVar = setVarFixedStartValue(
        timeInStateVar,
        metamodelica::Ref::new(DAE::Exp::RCONST {
            real: metamodelica::OrderedFloat((0) as f64),
        }),
    )?;
    timeInStateExp = metamodelica::Ref::new(DAE::Exp::CREF {
        componentRef: timeInStateRef,
        ty: DAE::T_REAL_DEFAULT().clone(),
    });
    let (__pa0, __pa1) = ::match_deref::match_deref! { match &((*timeEnteredStateVar)) {
        Deref @ DAE::Element::VAR { componentRef: __pa0, ty: __pa1, .. } => (__pa0.clone(), __pa1.clone()),
        _ => return Err("pattern mismatch"),
    } };
    timeEnteredStateRef = metamodelica::Own::own(__pa0);
    ty = metamodelica::Own::own(__pa1);
    timeEnteredStateExp = metamodelica::Ref::new(DAE::Exp::CREF {
        componentRef: timeEnteredStateRef,
        ty: ty,
    });
    stateActiveExp = Expression::crefExp(stateActiveRef.clone())?;
    expCond = Expression::crefExp(stateActiveRef)?;
    expSampleTime = metamodelica::Ref::new(DAE::Exp::CALL {
        path: metamodelica::Ref::new(Absyn::Path::IDENT {
            name: literal!("sample"),
        }),
        expLst: list![
            metamodelica::Ref::new(DAE::Exp::CREF {
                componentRef: metamodelica::Ref::new(DAE::ComponentRef::CREF_IDENT {
                    ident: literal!("time"),
                    identType: DAE::T_REAL_DEFAULT().clone(),
                    subscriptLst: metamodelica::nil()
                }),
                ty: DAE::T_REAL_DEFAULT().clone()
            }),
            metamodelica::Ref::new(DAE::Exp::CLKCONST {
                clk: openmodelica_frontend_types::DAE::ClockKind::interned_INFERRED_CLOCK()
            })
        ],
        attr: DAE::callAttrBuiltinImpureReal().clone(),
    });
    expThen = metamodelica::Ref::new(DAE::Exp::BINARY {
        exp1: expSampleTime,
        operator: DAE::Operator::SUB {
            ty: DAE::T_REAL_DEFAULT().clone(),
        },
        exp2: timeEnteredStateExp,
    });
    expElse = metamodelica::Ref::new(DAE::Exp::RCONST {
        real: metamodelica::OrderedFloat((0) as f64),
    });
    timeInStateEqn = metamodelica::Ref::new(DAE::Element::EQUATION {
        exp: timeInStateExp,
        scalar: metamodelica::Ref::new(DAE::Exp::IFEXP {
            expCond: expCond,
            expThen: expThen,
            expElse: expElse,
        }),
        source: DAE::emptyElementSource().clone(),
    });
    Ok((timeInStateVar, timeInStateEqn))
}

fn createTimeEnteredStateIndicator(
    mut stateRef: &metamodelica::Ref<DAE::ComponentRef>,
    mut stateActiveRef: metamodelica::Ref<DAE::ComponentRef>,
) -> Result<(metamodelica::Ref<DAE::Element>, metamodelica::Ref<DAE::Element>)> {
    let mut timeEnteredStateVar: metamodelica::Ref<DAE::Element>;
    let mut timeEnteredStateEqn: metamodelica::Ref<DAE::Element>;
    let mut timeEnteredStateRef: metamodelica::Ref<DAE::ComponentRef>;
    let mut timeEnteredStateExp: metamodelica::Ref<DAE::Exp>;
    let mut stateActiveExp: metamodelica::Ref<DAE::Exp>;
    let mut expCond: metamodelica::Ref<DAE::Exp>;
    let mut expThen: metamodelica::Ref<DAE::Exp>;
    let mut expElse: metamodelica::Ref<DAE::Exp>;
    timeEnteredStateRef = qCref(
        literal!("$timeEnteredState"),
        DAE::T_REAL_DEFAULT().clone(),
        metamodelica::nil(),
        stateRef,
    )?;
    timeEnteredStateVar = createVarWithDefaults(
        timeEnteredStateRef.clone(),
        openmodelica_frontend_types::DAE::VarKind::DISCRETE,
        DAE::T_REAL_DEFAULT().clone(),
        metamodelica::nil(),
    );
    timeEnteredStateVar = setVarFixedStartValue(
        timeEnteredStateVar,
        metamodelica::Ref::new(DAE::Exp::RCONST {
            real: metamodelica::OrderedFloat((0) as f64),
        }),
    )?;
    timeEnteredStateExp = metamodelica::Ref::new(DAE::Exp::CREF {
        componentRef: timeEnteredStateRef,
        ty: DAE::T_REAL_DEFAULT().clone(),
    });
    stateActiveExp = Expression::crefExp(stateActiveRef)?;
    expCond = metamodelica::Ref::new(DAE::Exp::LBINARY {
        exp1: metamodelica::Ref::new(DAE::Exp::RELATION {
            exp1: metamodelica::Ref::new(DAE::Exp::CALL {
                path: metamodelica::Ref::new(Absyn::Path::IDENT {
                    name: literal!("previous"),
                }),
                expLst: list![stateActiveExp.clone()],
                attr: DAE::callAttrBuiltinImpureBool().clone(),
            }),
            operator: DAE::Operator::EQUAL {
                ty: DAE::T_BOOL_DEFAULT().clone(),
            },
            exp2: metamodelica::Ref::new(DAE::Exp::BCONST { bool: false }),
            index: -1,
            optionExpisASUB: None,
        }),
        operator: DAE::Operator::AND {
            ty: DAE::T_BOOL_DEFAULT().clone(),
        },
        exp2: metamodelica::Ref::new(DAE::Exp::RELATION {
            exp1: stateActiveExp,
            operator: DAE::Operator::EQUAL {
                ty: DAE::T_BOOL_DEFAULT().clone(),
            },
            exp2: metamodelica::Ref::new(DAE::Exp::BCONST { bool: true }),
            index: -1,
            optionExpisASUB: None,
        }),
    });
    expThen = metamodelica::Ref::new(DAE::Exp::CALL {
        path: metamodelica::Ref::new(Absyn::Path::IDENT {
            name: literal!("sample"),
        }),
        expLst: list![
            metamodelica::Ref::new(DAE::Exp::CREF {
                componentRef: metamodelica::Ref::new(DAE::ComponentRef::CREF_IDENT {
                    ident: literal!("time"),
                    identType: DAE::T_REAL_DEFAULT().clone(),
                    subscriptLst: metamodelica::nil()
                }),
                ty: DAE::T_REAL_DEFAULT().clone()
            }),
            metamodelica::Ref::new(DAE::Exp::CLKCONST {
                clk: openmodelica_frontend_types::DAE::ClockKind::interned_INFERRED_CLOCK()
            })
        ],
        attr: DAE::callAttrBuiltinImpureReal().clone(),
    });
    expElse = metamodelica::Ref::new(DAE::Exp::CALL {
        path: metamodelica::Ref::new(Absyn::Path::IDENT {
            name: literal!("previous"),
        }),
        expLst: list![timeEnteredStateExp.clone()],
        attr: DAE::callAttrBuiltinImpureReal().clone(),
    });
    timeEnteredStateEqn = metamodelica::Ref::new(DAE::Element::EQUATION {
        exp: timeEnteredStateExp,
        scalar: metamodelica::Ref::new(DAE::Exp::IFEXP {
            expCond: expCond,
            expThen: expThen,
            expElse: expElse,
        }),
        source: DAE::emptyElementSource().clone(),
    });
    Ok((timeEnteredStateVar, timeEnteredStateEqn))
}

fn createTicksInStateIndicator(
    mut stateRef: &metamodelica::Ref<DAE::ComponentRef>,
    mut stateActiveRef: metamodelica::Ref<DAE::ComponentRef>,
) -> Result<(metamodelica::Ref<DAE::Element>, metamodelica::Ref<DAE::Element>)> {
    let mut ticksInStateVar: metamodelica::Ref<DAE::Element>;
    let mut ticksInStateEqn: metamodelica::Ref<DAE::Element>;
    let mut ticksInStateRef: metamodelica::Ref<DAE::ComponentRef>;
    let mut ticksInStateExp: metamodelica::Ref<DAE::Exp>;
    let mut expCond: metamodelica::Ref<DAE::Exp>;
    let mut expThen: metamodelica::Ref<DAE::Exp>;
    let mut expElse: metamodelica::Ref<DAE::Exp>;
    ticksInStateRef = qCref(
        literal!("$ticksInState"),
        DAE::T_INTEGER_DEFAULT().clone(),
        metamodelica::nil(),
        stateRef,
    )?;
    ticksInStateVar = createVarWithDefaults(
        ticksInStateRef.clone(),
        openmodelica_frontend_types::DAE::VarKind::DISCRETE,
        DAE::T_INTEGER_DEFAULT().clone(),
        metamodelica::nil(),
    );
    ticksInStateVar = setVarFixedStartValue(ticksInStateVar, metamodelica::Ref::new(DAE::Exp::ICONST { integer: 0 }))?;
    ticksInStateExp = metamodelica::Ref::new(DAE::Exp::CREF {
        componentRef: ticksInStateRef,
        ty: DAE::T_INTEGER_DEFAULT().clone(),
    });
    expCond = Expression::crefExp(stateActiveRef)?;
    expThen = metamodelica::Ref::new(DAE::Exp::BINARY {
        exp1: metamodelica::Ref::new(DAE::Exp::CALL {
            path: metamodelica::Ref::new(Absyn::Path::IDENT {
                name: literal!("previous"),
            }),
            expLst: list![ticksInStateExp.clone()],
            attr: DAE::callAttrBuiltinImpureInteger().clone(),
        }),
        operator: DAE::Operator::ADD {
            ty: DAE::T_INTEGER_DEFAULT().clone(),
        },
        exp2: metamodelica::Ref::new(DAE::Exp::ICONST { integer: 1 }),
    });
    expElse = metamodelica::Ref::new(DAE::Exp::ICONST { integer: 0 });
    ticksInStateEqn = metamodelica::Ref::new(DAE::Element::EQUATION {
        exp: ticksInStateExp,
        scalar: metamodelica::Ref::new(DAE::Exp::IFEXP {
            expCond: expCond,
            expThen: expThen,
            expElse: expElse,
        }),
        source: DAE::emptyElementSource().clone(),
    });
    Ok((ticksInStateVar, ticksInStateEqn))
}

fn createActiveIndicator(
    mut stateRef: &metamodelica::Ref<DAE::ComponentRef>,
    mut preRef: &metamodelica::Ref<DAE::ComponentRef>,
    mut i: i32,
) -> Result<(metamodelica::Ref<DAE::Element>, metamodelica::Ref<DAE::Element>)> {
    let mut activePlotIndicatorVar: metamodelica::Ref<DAE::Element>;
    let mut eqn: metamodelica::Ref<DAE::Element>;
    let mut activeRef: metamodelica::Ref<DAE::ComponentRef>;
    let mut activePlotIndicatorRef: metamodelica::Ref<DAE::ComponentRef>;
    let mut activeStateRef: metamodelica::Ref<DAE::ComponentRef>;
    let mut andExp: metamodelica::Ref<DAE::Exp>;
    let mut eqExp: metamodelica::Ref<DAE::Exp>;
    activePlotIndicatorRef = qCref(
        literal!("active"),
        DAE::T_BOOL_DEFAULT().clone(),
        metamodelica::nil(),
        stateRef,
    )?;
    activePlotIndicatorVar = createVarWithStartValue(
        activePlotIndicatorRef.clone(),
        openmodelica_frontend_types::DAE::VarKind::DISCRETE,
        DAE::T_BOOL_DEFAULT().clone(),
        metamodelica::Ref::new(DAE::Exp::BCONST { bool: false }),
        metamodelica::nil(),
    )?;
    activeRef = qCref(
        literal!("active"),
        DAE::T_BOOL_DEFAULT().clone(),
        metamodelica::nil(),
        preRef,
    )?;
    activeStateRef = qCref(
        literal!("activeState"),
        DAE::T_INTEGER_DEFAULT().clone(),
        metamodelica::nil(),
        preRef,
    )?;
    eqExp = metamodelica::Ref::new(DAE::Exp::RELATION {
        exp1: metamodelica::Ref::new(DAE::Exp::CREF {
            componentRef: activeStateRef,
            ty: DAE::T_INTEGER_DEFAULT().clone(),
        }),
        operator: DAE::Operator::EQUAL {
            ty: DAE::T_INTEGER_DEFAULT().clone(),
        },
        exp2: metamodelica::Ref::new(DAE::Exp::ICONST { integer: i }),
        index: -1,
        optionExpisASUB: None,
    });
    andExp = metamodelica::Ref::new(DAE::Exp::LBINARY {
        exp1: metamodelica::Ref::new(DAE::Exp::CREF {
            componentRef: activeRef,
            ty: DAE::T_BOOL_DEFAULT().clone(),
        }),
        operator: DAE::Operator::AND {
            ty: DAE::T_BOOL_DEFAULT().clone(),
        },
        exp2: eqExp,
    });
    eqn = metamodelica::Ref::new(DAE::Element::EQUATION {
        exp: metamodelica::Ref::new(DAE::Exp::CREF {
            componentRef: activePlotIndicatorRef,
            ty: DAE::T_BOOL_DEFAULT().clone(),
        }),
        scalar: andExp,
        source: DAE::emptyElementSource().clone(),
    });
    Ok((activePlotIndicatorVar, eqn))
}

fn setVarFixedStartValue(
    mut inVar: metamodelica::Ref<DAE::Element>,
    mut inExp: metamodelica::Ref<DAE::Exp>,
) -> Result<metamodelica::Ref<DAE::Element>> {
    let mut outVar: metamodelica::Ref<DAE::Element>;
    let mut vao: Option<metamodelica::Ref<DAE::VariableAttributes>>;
    let __pa0 = ::match_deref::match_deref! { match &(inVar.clone()) {
        Deref @ DAE::Element::VAR { variableAttributesOption: __pa0, .. } => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    vao = metamodelica::Own::own(__pa0);
    vao = DAEUtil::setStartAttrOption(vao, Some(inExp))?;
    vao = DAEUtil::setFixedAttr(vao, Some(metamodelica::Ref::new(DAE::Exp::BCONST { bool: true })))?;
    outVar = DAEUtil::setVariableAttributes(inVar, vao)?;
    Ok(outVar)
}

fn basicFlatSmSemantics(
    mut ident: ArcStr,
    mut q: metamodelica::List<metamodelica::Ref<DAE::Element>>,
    mut inTransitions: metamodelica::List<metamodelica::Ref<DAE::Element>>,
) -> Result<FlatSmSemantics> {
    let mut flatSmSemantics: FlatSmSemantics;
    let mut crefInitialState: metamodelica::Ref<DAE::ComponentRef>;
    let mut preRef: metamodelica::Ref<DAE::ComponentRef>;
    let mut defaultIntVar: metamodelica::Ref<DAE::Element>;
    let mut defaultBoolVar: metamodelica::Ref<DAE::Element>;
    let mut vars: metamodelica::List<metamodelica::Ref<DAE::Element>>;
    let mut knowns: metamodelica::List<metamodelica::Ref<DAE::Element>>;
    let mut i: i32;
    let mut preRef: metamodelica::Ref<DAE::ComponentRef>;
    let mut nStatesRef: metamodelica::Ref<DAE::ComponentRef>;
    let mut activeRef: metamodelica::Ref<DAE::ComponentRef>;
    let mut resetRef: metamodelica::Ref<DAE::ComponentRef>;
    let mut selectedStateRef: metamodelica::Ref<DAE::ComponentRef>;
    let mut selectedResetRef: metamodelica::Ref<DAE::ComponentRef>;
    let mut firedRef: metamodelica::Ref<DAE::ComponentRef>;
    let mut activeStateRef: metamodelica::Ref<DAE::ComponentRef>;
    let mut activeResetRef: metamodelica::Ref<DAE::ComponentRef>;
    let mut nextStateRef: metamodelica::Ref<DAE::ComponentRef>;
    let mut nextResetRef: metamodelica::Ref<DAE::ComponentRef>;
    let mut stateMachineInFinalStateRef: metamodelica::Ref<DAE::ComponentRef>;
    let mut nStatesVar: metamodelica::Ref<DAE::Element>;
    let mut activeVar: metamodelica::Ref<DAE::Element>;
    let mut resetVar: metamodelica::Ref<DAE::Element>;
    let mut selectedStateVar: metamodelica::Ref<DAE::Element>;
    let mut selectedResetVar: metamodelica::Ref<DAE::Element>;
    let mut firedVar: metamodelica::Ref<DAE::Element>;
    let mut activeStateVar: metamodelica::Ref<DAE::Element>;
    let mut activeResetVar: metamodelica::Ref<DAE::Element>;
    let mut nextStateVar: metamodelica::Ref<DAE::Element>;
    let mut nextResetVar: metamodelica::Ref<DAE::Element>;
    let mut stateMachineInFinalStateVar: metamodelica::Ref<DAE::Element>;
    let mut nStates: i32;
    let mut nStatesDims: metamodelica::List<metamodelica::Ref<DAE::Dimension>>;
    let mut nStatesArrayBool: metamodelica::Ref<DAE::Type>;
    let mut activeResetStatesRefs: metamodelica::Array<metamodelica::Ref<DAE::ComponentRef>>;
    let mut nextResetStatesRefs: metamodelica::Array<metamodelica::Ref<DAE::ComponentRef>>;
    let mut finalStatesRefs: metamodelica::Array<metamodelica::Ref<DAE::ComponentRef>>;
    let mut activeResetStatesVars: metamodelica::Array<metamodelica::Ref<DAE::Element>>;
    let mut nextResetStatesVars: metamodelica::Array<metamodelica::Ref<DAE::Element>>;
    let mut finalStatesVars: metamodelica::Array<metamodelica::Ref<DAE::Element>>;
    let mut t: metamodelica::List<Transition>;
    let mut nTransitions: i32;
    let mut tDims: metamodelica::List<metamodelica::Ref<DAE::Dimension>>;
    let mut tArrayInteger: metamodelica::Ref<DAE::Type>;
    let mut tArrayBool: metamodelica::Ref<DAE::Type>;
    let mut tFromRefs: metamodelica::Array<metamodelica::Ref<DAE::ComponentRef>>;
    let mut tToRefs: metamodelica::Array<metamodelica::Ref<DAE::ComponentRef>>;
    let mut tImmediateRefs: metamodelica::Array<metamodelica::Ref<DAE::ComponentRef>>;
    let mut tResetRefs: metamodelica::Array<metamodelica::Ref<DAE::ComponentRef>>;
    let mut tSynchronizeRefs: metamodelica::Array<metamodelica::Ref<DAE::ComponentRef>>;
    let mut tPriorityRefs: metamodelica::Array<metamodelica::Ref<DAE::ComponentRef>>;
    let mut tFromVars: metamodelica::Array<metamodelica::Ref<DAE::Element>>;
    let mut tToVars: metamodelica::Array<metamodelica::Ref<DAE::Element>>;
    let mut tImmediateVars: metamodelica::Array<metamodelica::Ref<DAE::Element>>;
    let mut tResetVars: metamodelica::Array<metamodelica::Ref<DAE::Element>>;
    let mut tSynchronizeVars: metamodelica::Array<metamodelica::Ref<DAE::Element>>;
    let mut tPriorityVars: metamodelica::Array<metamodelica::Ref<DAE::Element>>;
    let mut from: i32;
    let mut to: i32;
    let mut immediate: bool;
    let mut reset: bool;
    let mut synchronize: bool;
    let mut priority: i32;
    let mut cExps: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
    let mut cRefs: metamodelica::Array<metamodelica::Ref<DAE::ComponentRef>>;
    let mut cImmediateRefs: metamodelica::Array<metamodelica::Ref<DAE::ComponentRef>>;
    let mut cVars: metamodelica::Array<metamodelica::Ref<DAE::Element>>;
    let mut cImmediateVars: metamodelica::Array<metamodelica::Ref<DAE::Element>>;
    let mut eqs: metamodelica::List<metamodelica::Ref<DAE::Element>>;
    let mut selectedStateEqn: metamodelica::Ref<DAE::Element>;
    let mut selectedResetEqn: metamodelica::Ref<DAE::Element>;
    let mut firedEqn: metamodelica::Ref<DAE::Element>;
    let mut activeStateEqn: metamodelica::Ref<DAE::Element>;
    let mut activeResetEqn: metamodelica::Ref<DAE::Element>;
    let mut nextStateEqn: metamodelica::Ref<DAE::Element>;
    let mut nextResetEqn: metamodelica::Ref<DAE::Element>;
    let mut exp: metamodelica::Ref<DAE::Exp>;
    let mut rhs: metamodelica::Ref<DAE::Exp>;
    let mut expCond: metamodelica::Ref<DAE::Exp>;
    let mut expThen: metamodelica::Ref<DAE::Exp>;
    let mut expElse: metamodelica::Ref<DAE::Exp>;
    let mut exp1: metamodelica::Ref<DAE::Exp>;
    let mut exp2: metamodelica::Ref<DAE::Exp>;
    let mut expIf: metamodelica::Ref<DAE::Exp>;
    let mut expLst: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
    let mut bindExp: Option<metamodelica::Ref<DAE::Exp>>;
    let __pa0 = ::match_deref::match_deref! { match &((q).head().cloned()?) {
        Deref @ DAE::Element::SM_COMP { componentRef: __pa0, .. } => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    crefInitialState = metamodelica::Own::own(__pa0);
    preRef = ComponentReference::crefPrefixString(arcstr::literal!(SMS_PRE), crefInitialState);
    (t, cExps) = createTandC(q.clone(), inTransitions)?;
    defaultIntVar = createVarWithDefaults(
        ComponentReference::makeDummyCref(),
        openmodelica_frontend_types::DAE::VarKind::DISCRETE,
        DAE::T_INTEGER_DEFAULT().clone(),
        metamodelica::nil(),
    );
    defaultBoolVar = createVarWithDefaults(
        ComponentReference::makeDummyCref(),
        openmodelica_frontend_types::DAE::VarKind::DISCRETE,
        DAE::T_BOOL_DEFAULT().clone(),
        metamodelica::nil(),
    );
    knowns = metamodelica::nil();
    vars = metamodelica::nil();
    nStates = ((q).len() as i32);
    nStatesRef = qCref(
        literal!("nState"),
        DAE::T_INTEGER_DEFAULT().clone(),
        metamodelica::nil(),
        &preRef,
    )?;
    nStatesVar = createVarWithDefaults(
        nStatesRef,
        openmodelica_frontend_types::DAE::VarKind::PARAM,
        DAE::T_INTEGER_DEFAULT().clone(),
        metamodelica::nil(),
    );
    nStatesVar = DAEUtil::setElementVarBinding(
        nStatesVar,
        Some(metamodelica::Ref::new(DAE::Exp::ICONST { integer: nStates })),
    );
    knowns = metamodelica::cons(nStatesVar, knowns);
    nTransitions = ((t).len() as i32);
    tDims = list![metamodelica::Ref::new(DAE::Dimension::DIM_INTEGER {
        integer: nTransitions
    })];
    tArrayInteger = metamodelica::Ref::new(DAE::Type::T_ARRAY {
        ty: DAE::T_INTEGER_DEFAULT().clone(),
        dims: tDims.clone(),
    });
    tArrayBool = metamodelica::Ref::new(DAE::Type::T_ARRAY {
        ty: DAE::T_BOOL_DEFAULT().clone(),
        dims: tDims.clone(),
    });
    tFromRefs = arrayCreate(nTransitions, ComponentReference::makeDummyCref());
    tToRefs = arrayCreate(nTransitions, ComponentReference::makeDummyCref());
    tImmediateRefs = arrayCreate(nTransitions, ComponentReference::makeDummyCref());
    tResetRefs = arrayCreate(nTransitions, ComponentReference::makeDummyCref());
    tSynchronizeRefs = arrayCreate(nTransitions, ComponentReference::makeDummyCref());
    tPriorityRefs = arrayCreate(nTransitions, ComponentReference::makeDummyCref());
    tFromVars = arrayCreate(nTransitions, defaultIntVar.clone());
    tToVars = arrayCreate(nTransitions, defaultIntVar.clone());
    tImmediateVars = arrayCreate(nTransitions, defaultBoolVar.clone());
    tResetVars = arrayCreate(nTransitions, defaultBoolVar.clone());
    tSynchronizeVars = arrayCreate(nTransitions, defaultBoolVar.clone());
    tPriorityVars = arrayCreate(nTransitions, defaultIntVar);
    i = 0;
    for mut t1 in &*t {
        i = i + 1;
        let Transition {
            from: __pa1,
            to: __pa2,
            condition: _,
            immediate: __pa3,
            reset: __pa4,
            synchronize: __pa5,
            priority: __pa6,
        } = &t1;
        from = metamodelica::Own::own(__pa1);
        to = metamodelica::Own::own(__pa2);
        immediate = metamodelica::Own::own(__pa3);
        reset = metamodelica::Own::own(__pa4);
        synchronize = metamodelica::Own::own(__pa5);
        priority = metamodelica::Own::own(__pa6);
        tFromRefs = metamodelica::arrayUpdate(
            tFromRefs.clone(),
            i,
            qCref(
                literal!("tFrom"),
                tArrayInteger.clone(),
                list![metamodelica::Ref::new(DAE::Subscript::INDEX {
                    exp: metamodelica::Ref::new(DAE::Exp::ICONST { integer: i })
                })],
                &preRef,
            )?,
        )?;
        tFromVars = metamodelica::arrayUpdate(
            tFromVars.clone(),
            i,
            createVarWithDefaults(
                metamodelica::arrayGet(tFromRefs.clone(), i)?,
                openmodelica_frontend_types::DAE::VarKind::PARAM,
                DAE::T_INTEGER_DEFAULT().clone(),
                tDims.clone(),
            ),
        )?;
        tFromVars = metamodelica::arrayUpdate(
            tFromVars.clone(),
            i,
            DAEUtil::setElementVarBinding(
                metamodelica::arrayGet(tFromVars.clone(), i)?,
                Some(metamodelica::Ref::new(DAE::Exp::ICONST { integer: from })),
            ),
        )?;
        knowns = metamodelica::cons(metamodelica::arrayGet(tFromVars.clone(), i)?, knowns);
        tToRefs = metamodelica::arrayUpdate(
            tToRefs.clone(),
            i,
            qCref(
                literal!("tTo"),
                tArrayInteger.clone(),
                list![metamodelica::Ref::new(DAE::Subscript::INDEX {
                    exp: metamodelica::Ref::new(DAE::Exp::ICONST { integer: i })
                })],
                &preRef,
            )?,
        )?;
        tToVars = metamodelica::arrayUpdate(
            tToVars.clone(),
            i,
            createVarWithDefaults(
                metamodelica::arrayGet(tToRefs.clone(), i)?,
                openmodelica_frontend_types::DAE::VarKind::PARAM,
                DAE::T_INTEGER_DEFAULT().clone(),
                tDims.clone(),
            ),
        )?;
        tToVars = metamodelica::arrayUpdate(
            tToVars.clone(),
            i,
            DAEUtil::setElementVarBinding(
                metamodelica::arrayGet(tToVars.clone(), i)?,
                Some(metamodelica::Ref::new(DAE::Exp::ICONST { integer: to })),
            ),
        )?;
        knowns = metamodelica::cons(metamodelica::arrayGet(tToVars.clone(), i)?, knowns);
        tImmediateRefs = metamodelica::arrayUpdate(
            tImmediateRefs.clone(),
            i,
            qCref(
                literal!("tImmediate"),
                tArrayBool.clone(),
                list![metamodelica::Ref::new(DAE::Subscript::INDEX {
                    exp: metamodelica::Ref::new(DAE::Exp::ICONST { integer: i })
                })],
                &preRef,
            )?,
        )?;
        tImmediateVars = metamodelica::arrayUpdate(
            tImmediateVars.clone(),
            i,
            createVarWithDefaults(
                metamodelica::arrayGet(tImmediateRefs.clone(), i)?,
                openmodelica_frontend_types::DAE::VarKind::PARAM,
                DAE::T_BOOL_DEFAULT().clone(),
                tDims.clone(),
            ),
        )?;
        tImmediateVars = metamodelica::arrayUpdate(
            tImmediateVars.clone(),
            i,
            DAEUtil::setElementVarBinding(
                metamodelica::arrayGet(tImmediateVars.clone(), i)?,
                Some(metamodelica::Ref::new(DAE::Exp::BCONST { bool: immediate })),
            ),
        )?;
        knowns = metamodelica::cons(metamodelica::arrayGet(tImmediateVars.clone(), i)?, knowns);
        tResetRefs = metamodelica::arrayUpdate(
            tResetRefs.clone(),
            i,
            qCref(
                literal!("tReset"),
                tArrayBool.clone(),
                list![metamodelica::Ref::new(DAE::Subscript::INDEX {
                    exp: metamodelica::Ref::new(DAE::Exp::ICONST { integer: i })
                })],
                &preRef,
            )?,
        )?;
        tResetVars = metamodelica::arrayUpdate(
            tResetVars.clone(),
            i,
            createVarWithDefaults(
                metamodelica::arrayGet(tResetRefs.clone(), i)?,
                openmodelica_frontend_types::DAE::VarKind::PARAM,
                DAE::T_BOOL_DEFAULT().clone(),
                tDims.clone(),
            ),
        )?;
        tResetVars = metamodelica::arrayUpdate(
            tResetVars.clone(),
            i,
            DAEUtil::setElementVarBinding(
                metamodelica::arrayGet(tResetVars.clone(), i)?,
                Some(metamodelica::Ref::new(DAE::Exp::BCONST { bool: reset })),
            ),
        )?;
        knowns = metamodelica::cons(metamodelica::arrayGet(tResetVars.clone(), i)?, knowns);
        tSynchronizeRefs = metamodelica::arrayUpdate(
            tSynchronizeRefs.clone(),
            i,
            qCref(
                literal!("tSynchronize"),
                tArrayBool.clone(),
                list![metamodelica::Ref::new(DAE::Subscript::INDEX {
                    exp: metamodelica::Ref::new(DAE::Exp::ICONST { integer: i })
                })],
                &preRef,
            )?,
        )?;
        tSynchronizeVars = metamodelica::arrayUpdate(
            tSynchronizeVars.clone(),
            i,
            createVarWithDefaults(
                metamodelica::arrayGet(tSynchronizeRefs.clone(), i)?,
                openmodelica_frontend_types::DAE::VarKind::PARAM,
                DAE::T_BOOL_DEFAULT().clone(),
                tDims.clone(),
            ),
        )?;
        tSynchronizeVars = metamodelica::arrayUpdate(
            tSynchronizeVars.clone(),
            i,
            DAEUtil::setElementVarBinding(
                metamodelica::arrayGet(tSynchronizeVars.clone(), i)?,
                Some(metamodelica::Ref::new(DAE::Exp::BCONST { bool: synchronize })),
            ),
        )?;
        knowns = metamodelica::cons(metamodelica::arrayGet(tSynchronizeVars.clone(), i)?, knowns);
        tPriorityRefs = metamodelica::arrayUpdate(
            tPriorityRefs.clone(),
            i,
            qCref(
                literal!("tPriority"),
                tArrayInteger.clone(),
                list![metamodelica::Ref::new(DAE::Subscript::INDEX {
                    exp: metamodelica::Ref::new(DAE::Exp::ICONST { integer: i })
                })],
                &preRef,
            )?,
        )?;
        tPriorityVars = metamodelica::arrayUpdate(
            tPriorityVars.clone(),
            i,
            createVarWithDefaults(
                metamodelica::arrayGet(tPriorityRefs.clone(), i)?,
                openmodelica_frontend_types::DAE::VarKind::PARAM,
                DAE::T_INTEGER_DEFAULT().clone(),
                tDims.clone(),
            ),
        )?;
        tPriorityVars = metamodelica::arrayUpdate(
            tPriorityVars.clone(),
            i,
            DAEUtil::setElementVarBinding(
                metamodelica::arrayGet(tPriorityVars.clone(), i)?,
                Some(metamodelica::Ref::new(DAE::Exp::ICONST { integer: priority })),
            ),
        )?;
        knowns = metamodelica::cons(metamodelica::arrayGet(tPriorityVars.clone(), i)?, knowns);
    }
    cRefs = arrayCreate(nTransitions, ComponentReference::makeDummyCref());
    cImmediateRefs = arrayCreate(nTransitions, ComponentReference::makeDummyCref());
    cVars = arrayCreate(nTransitions, defaultBoolVar.clone());
    cImmediateVars = arrayCreate(nTransitions, defaultBoolVar.clone());
    i = 0;
    for mut exp in &*cExps {
        let mut exp = exp.clone();
        i = i + 1;
        cRefs = metamodelica::arrayUpdate(
            cRefs.clone(),
            i,
            qCref(
                literal!("c"),
                tArrayBool.clone(),
                list![metamodelica::Ref::new(DAE::Subscript::INDEX {
                    exp: metamodelica::Ref::new(DAE::Exp::ICONST { integer: i })
                })],
                &preRef,
            )?,
        )?;
        cImmediateRefs = metamodelica::arrayUpdate(
            cImmediateRefs.clone(),
            i,
            qCref(
                literal!("cImmediate"),
                tArrayBool.clone(),
                list![metamodelica::Ref::new(DAE::Subscript::INDEX {
                    exp: metamodelica::Ref::new(DAE::Exp::ICONST { integer: i })
                })],
                &preRef,
            )?,
        )?;
        cVars = metamodelica::arrayUpdate(
            cVars.clone(),
            i,
            createVarWithDefaults(
                metamodelica::arrayGet(cRefs.clone(), i)?,
                openmodelica_frontend_types::DAE::VarKind::DISCRETE,
                DAE::T_BOOL_DEFAULT().clone(),
                tDims.clone(),
            ),
        )?;
        cImmediateVars = metamodelica::arrayUpdate(
            cImmediateVars.clone(),
            i,
            createVarWithStartValue(
                metamodelica::arrayGet(cImmediateRefs.clone(), i)?,
                openmodelica_frontend_types::DAE::VarKind::DISCRETE,
                DAE::T_BOOL_DEFAULT().clone(),
                metamodelica::Ref::new(DAE::Exp::BCONST { bool: false }),
                tDims.clone(),
            )?,
        )?;
        vars = metamodelica::cons(metamodelica::arrayGet(cVars.clone(), i)?, vars);
        vars = metamodelica::cons(metamodelica::arrayGet(cImmediateVars.clone(), i)?, vars);
    }
    activeRef = qCref(
        literal!("active"),
        DAE::T_BOOL_DEFAULT().clone(),
        metamodelica::nil(),
        &preRef,
    )?;
    activeVar = createVarWithDefaults(
        activeRef.clone(),
        openmodelica_frontend_types::DAE::VarKind::DISCRETE,
        DAE::T_BOOL_DEFAULT().clone(),
        metamodelica::nil(),
    );
    vars = metamodelica::cons(activeVar, vars);
    resetRef = qCref(
        literal!("reset"),
        DAE::T_BOOL_DEFAULT().clone(),
        metamodelica::nil(),
        &preRef,
    )?;
    resetVar = createVarWithDefaults(
        resetRef.clone(),
        openmodelica_frontend_types::DAE::VarKind::DISCRETE,
        DAE::T_BOOL_DEFAULT().clone(),
        metamodelica::nil(),
    );
    vars = metamodelica::cons(resetVar, vars);
    selectedStateRef = qCref(
        literal!("selectedState"),
        DAE::T_INTEGER_DEFAULT().clone(),
        metamodelica::nil(),
        &preRef,
    )?;
    selectedStateVar = createVarWithDefaults(
        selectedStateRef.clone(),
        openmodelica_frontend_types::DAE::VarKind::DISCRETE,
        DAE::T_INTEGER_DEFAULT().clone(),
        metamodelica::nil(),
    );
    vars = metamodelica::cons(selectedStateVar, vars);
    selectedResetRef = qCref(
        literal!("selectedReset"),
        DAE::T_BOOL_DEFAULT().clone(),
        metamodelica::nil(),
        &preRef,
    )?;
    selectedResetVar = createVarWithDefaults(
        selectedResetRef.clone(),
        openmodelica_frontend_types::DAE::VarKind::DISCRETE,
        DAE::T_BOOL_DEFAULT().clone(),
        metamodelica::nil(),
    );
    vars = metamodelica::cons(selectedResetVar, vars);
    firedRef = qCref(
        literal!("fired"),
        DAE::T_INTEGER_DEFAULT().clone(),
        metamodelica::nil(),
        &preRef,
    )?;
    firedVar = createVarWithDefaults(
        firedRef.clone(),
        openmodelica_frontend_types::DAE::VarKind::DISCRETE,
        DAE::T_INTEGER_DEFAULT().clone(),
        metamodelica::nil(),
    );
    vars = metamodelica::cons(firedVar, vars);
    activeStateRef = qCref(
        literal!("activeState"),
        DAE::T_INTEGER_DEFAULT().clone(),
        metamodelica::nil(),
        &preRef,
    )?;
    activeStateVar = createVarWithDefaults(
        activeStateRef.clone(),
        openmodelica_frontend_types::DAE::VarKind::DISCRETE,
        DAE::T_INTEGER_DEFAULT().clone(),
        metamodelica::nil(),
    );
    vars = metamodelica::cons(activeStateVar, vars);
    activeResetRef = qCref(
        literal!("activeReset"),
        DAE::T_BOOL_DEFAULT().clone(),
        metamodelica::nil(),
        &preRef,
    )?;
    activeResetVar = createVarWithDefaults(
        activeResetRef.clone(),
        openmodelica_frontend_types::DAE::VarKind::DISCRETE,
        DAE::T_BOOL_DEFAULT().clone(),
        metamodelica::nil(),
    );
    vars = metamodelica::cons(activeResetVar, vars);
    nextStateRef = qCref(
        literal!("nextState"),
        DAE::T_INTEGER_DEFAULT().clone(),
        metamodelica::nil(),
        &preRef,
    )?;
    nextStateVar = createVarWithStartValue(
        nextStateRef.clone(),
        openmodelica_frontend_types::DAE::VarKind::DISCRETE,
        DAE::T_INTEGER_DEFAULT().clone(),
        metamodelica::Ref::new(DAE::Exp::ICONST { integer: 0 }),
        metamodelica::nil(),
    )?;
    vars = metamodelica::cons(nextStateVar, vars);
    nextResetRef = qCref(
        literal!("nextReset"),
        DAE::T_BOOL_DEFAULT().clone(),
        metamodelica::nil(),
        &preRef,
    )?;
    nextResetVar = createVarWithStartValue(
        nextResetRef.clone(),
        openmodelica_frontend_types::DAE::VarKind::DISCRETE,
        DAE::T_BOOL_DEFAULT().clone(),
        metamodelica::Ref::new(DAE::Exp::BCONST { bool: false }),
        metamodelica::nil(),
    )?;
    vars = metamodelica::cons(nextResetVar, vars);
    nStatesDims = list![metamodelica::Ref::new(DAE::Dimension::DIM_INTEGER { integer: nStates })];
    nStatesArrayBool = metamodelica::Ref::new(DAE::Type::T_ARRAY {
        ty: DAE::T_BOOL_DEFAULT().clone(),
        dims: nStatesDims.clone(),
    });
    activeResetStatesRefs = arrayCreate(nStates, ComponentReference::makeDummyCref());
    activeResetStatesVars = arrayCreate(nStates, defaultBoolVar.clone());
    for mut i in 1..=nStates {
        activeResetStatesRefs = metamodelica::arrayUpdate(
            activeResetStatesRefs.clone(),
            i,
            qCref(
                literal!("activeResetStates"),
                nStatesArrayBool.clone(),
                list![metamodelica::Ref::new(DAE::Subscript::INDEX {
                    exp: metamodelica::Ref::new(DAE::Exp::ICONST { integer: i })
                })],
                &preRef,
            )?,
        )?;
        activeResetStatesVars = metamodelica::arrayUpdate(
            activeResetStatesVars.clone(),
            i,
            createVarWithDefaults(
                metamodelica::arrayGet(activeResetStatesRefs.clone(), i)?,
                openmodelica_frontend_types::DAE::VarKind::DISCRETE,
                DAE::T_BOOL_DEFAULT().clone(),
                nStatesDims.clone(),
            ),
        )?;
        vars = metamodelica::cons(metamodelica::arrayGet(activeResetStatesVars.clone(), i)?, vars);
    }
    nextResetStatesRefs = arrayCreate(nStates, ComponentReference::makeDummyCref());
    nextResetStatesVars = arrayCreate(nStates, defaultBoolVar.clone());
    for mut i in 1..=nStates {
        nextResetStatesRefs = metamodelica::arrayUpdate(
            nextResetStatesRefs.clone(),
            i,
            qCref(
                literal!("nextResetStates"),
                nStatesArrayBool.clone(),
                list![metamodelica::Ref::new(DAE::Subscript::INDEX {
                    exp: metamodelica::Ref::new(DAE::Exp::ICONST { integer: i })
                })],
                &preRef,
            )?,
        )?;
        nextResetStatesVars = metamodelica::arrayUpdate(
            nextResetStatesVars.clone(),
            i,
            createVarWithStartValue(
                metamodelica::arrayGet(nextResetStatesRefs.clone(), i)?,
                openmodelica_frontend_types::DAE::VarKind::DISCRETE,
                DAE::T_BOOL_DEFAULT().clone(),
                metamodelica::Ref::new(DAE::Exp::BCONST { bool: false }),
                nStatesDims.clone(),
            )?,
        )?;
        vars = metamodelica::cons(metamodelica::arrayGet(nextResetStatesVars.clone(), i)?, vars);
    }
    finalStatesRefs = arrayCreate(nStates, ComponentReference::makeDummyCref());
    finalStatesVars = arrayCreate(nStates, defaultBoolVar);
    for mut i in 1..=nStates {
        finalStatesRefs = metamodelica::arrayUpdate(
            finalStatesRefs.clone(),
            i,
            qCref(
                literal!("finalStates"),
                nStatesArrayBool.clone(),
                list![metamodelica::Ref::new(DAE::Subscript::INDEX {
                    exp: metamodelica::Ref::new(DAE::Exp::ICONST { integer: i })
                })],
                &preRef,
            )?,
        )?;
        finalStatesVars = metamodelica::arrayUpdate(
            finalStatesVars.clone(),
            i,
            createVarWithDefaults(
                metamodelica::arrayGet(finalStatesRefs.clone(), i)?,
                openmodelica_frontend_types::DAE::VarKind::DISCRETE,
                DAE::T_BOOL_DEFAULT().clone(),
                nStatesDims.clone(),
            ),
        )?;
        vars = metamodelica::cons(metamodelica::arrayGet(finalStatesVars.clone(), i)?, vars);
    }
    stateMachineInFinalStateRef = qCref(
        literal!("stateMachineInFinalState"),
        DAE::T_BOOL_DEFAULT().clone(),
        metamodelica::nil(),
        &preRef,
    )?;
    stateMachineInFinalStateVar = createVarWithDefaults(
        stateMachineInFinalStateRef.clone(),
        openmodelica_frontend_types::DAE::VarKind::DISCRETE,
        DAE::T_BOOL_DEFAULT().clone(),
        metamodelica::nil(),
    );
    vars = metamodelica::cons(stateMachineInFinalStateVar, vars);
    eqs = metamodelica::nil();
    i = 0;
    for mut cExp in &*cExps {
        i = i + 1;
        exp = metamodelica::Ref::new(DAE::Exp::CREF {
            componentRef: metamodelica::arrayGet(cImmediateRefs.clone(), i)?,
            ty: DAE::T_BOOL_DEFAULT().clone(),
        });
        eqs = metamodelica::cons(
            metamodelica::Ref::new(DAE::Element::EQUATION {
                exp: exp.clone(),
                scalar: cExp.clone(),
                source: DAE::emptyElementSource().clone(),
            }),
            eqs,
        );
        exp1 = metamodelica::Ref::new(DAE::Exp::CREF {
            componentRef: metamodelica::arrayGet(cRefs.clone(), i)?,
            ty: DAE::T_BOOL_DEFAULT().clone(),
        });
        let __pa7 = ::match_deref::match_deref! { match &(metamodelica::arrayGet(tImmediateVars.clone(), i)?) {
            Deref @ DAE::Element::VAR { binding: __pa7, .. } => __pa7.clone(),
            _ => return Err("pattern mismatch"),
        } };
        bindExp = metamodelica::Own::own(__pa7);
        rhs = if (Util::applyOptionOrDefault(
            bindExp,
            &({
                let __pe_b0 = metamodelica::Ref::new(DAE::Exp::BCONST { bool: true });
                move |__pe_a1| ExpressionBasics::expEqual(&__pe_b0, __pe_a1)
            }),
            false,
        )?) {
            exp
        } else {
            metamodelica::Ref::new(DAE::Exp::CALL {
                path: metamodelica::Ref::new(Absyn::Path::IDENT {
                    name: literal!("previous"),
                }),
                expLst: list![exp],
                attr: DAE::callAttrBuiltinImpureBool().clone(),
            })
        };
        eqs = metamodelica::cons(
            metamodelica::Ref::new(DAE::Element::EQUATION {
                exp: exp1,
                scalar: rhs,
                source: DAE::emptyElementSource().clone(),
            }),
            eqs,
        );
    }
    exp = metamodelica::Ref::new(DAE::Exp::CREF {
        componentRef: selectedStateRef.clone(),
        ty: DAE::T_INTEGER_DEFAULT().clone(),
    });
    expCond = metamodelica::Ref::new(DAE::Exp::CREF {
        componentRef: resetRef.clone(),
        ty: DAE::T_BOOL_DEFAULT().clone(),
    });
    expThen = metamodelica::Ref::new(DAE::Exp::ICONST { integer: 1 });
    expElse = metamodelica::Ref::new(DAE::Exp::CALL {
        path: metamodelica::Ref::new(Absyn::Path::IDENT {
            name: literal!("previous"),
        }),
        expLst: list![metamodelica::Ref::new(DAE::Exp::CREF {
            componentRef: nextStateRef.clone(),
            ty: DAE::T_INTEGER_DEFAULT().clone()
        })],
        attr: DAE::callAttrBuiltinImpureInteger().clone(),
    });
    rhs = metamodelica::Ref::new(DAE::Exp::IFEXP {
        expCond: expCond,
        expThen: expThen,
        expElse: expElse,
    });
    selectedStateEqn = metamodelica::Ref::new(DAE::Element::EQUATION {
        exp: exp,
        scalar: rhs,
        source: DAE::emptyElementSource().clone(),
    });
    eqs = metamodelica::cons(selectedStateEqn, eqs);
    exp = metamodelica::Ref::new(DAE::Exp::CREF {
        componentRef: selectedResetRef.clone(),
        ty: DAE::T_BOOL_DEFAULT().clone(),
    });
    expCond = metamodelica::Ref::new(DAE::Exp::CREF {
        componentRef: resetRef.clone(),
        ty: DAE::T_BOOL_DEFAULT().clone(),
    });
    expThen = metamodelica::Ref::new(DAE::Exp::BCONST { bool: true });
    expElse = metamodelica::Ref::new(DAE::Exp::CALL {
        path: metamodelica::Ref::new(Absyn::Path::IDENT {
            name: literal!("previous"),
        }),
        expLst: list![metamodelica::Ref::new(DAE::Exp::CREF {
            componentRef: nextResetRef.clone(),
            ty: DAE::T_BOOL_DEFAULT().clone()
        })],
        attr: DAE::callAttrBuiltinImpureBool().clone(),
    });
    rhs = metamodelica::Ref::new(DAE::Exp::IFEXP {
        expCond: expCond,
        expThen: expThen,
        expElse: expElse,
    });
    selectedResetEqn = metamodelica::Ref::new(DAE::Element::EQUATION {
        exp: exp,
        scalar: rhs,
        source: DAE::emptyElementSource().clone(),
    });
    eqs = metamodelica::cons(selectedResetEqn, eqs);
    exp = metamodelica::Ref::new(DAE::Exp::CREF {
        componentRef: firedRef.clone(),
        ty: DAE::T_INTEGER_DEFAULT().clone(),
    });
    expLst = metamodelica::nil();
    for mut i in 1..=nTransitions {
        expCond = metamodelica::Ref::new(DAE::Exp::RELATION {
            exp1: metamodelica::Ref::new(DAE::Exp::CREF {
                componentRef: metamodelica::arrayGet(tFromRefs.clone(), i)?,
                ty: DAE::T_INTEGER_DEFAULT().clone(),
            }),
            operator: DAE::Operator::EQUAL {
                ty: DAE::T_INTEGER_DEFAULT().clone(),
            },
            exp2: metamodelica::Ref::new(DAE::Exp::CREF {
                componentRef: selectedStateRef.clone(),
                ty: DAE::T_INTEGER_DEFAULT().clone(),
            }),
            index: -1,
            optionExpisASUB: None,
        });
        expThen = metamodelica::Ref::new(DAE::Exp::CREF {
            componentRef: metamodelica::arrayGet(cRefs.clone(), i)?,
            ty: DAE::T_BOOL_DEFAULT().clone(),
        });
        expElse = metamodelica::Ref::new(DAE::Exp::BCONST { bool: false });
        expIf = metamodelica::Ref::new(DAE::Exp::IFEXP {
            expCond: expCond,
            expThen: expThen,
            expElse: expElse,
        });
        expLst = metamodelica::cons(
            metamodelica::Ref::new(DAE::Exp::IFEXP {
                expCond: expIf,
                expThen: metamodelica::Ref::new(DAE::Exp::ICONST { integer: i }),
                expElse: metamodelica::Ref::new(DAE::Exp::ICONST { integer: 0 }),
            }),
            expLst,
        );
    }
    rhs = if (((expLst).len() as i32) > 1) {
        metamodelica::Ref::new(DAE::Exp::CALL {
            path: metamodelica::Ref::new(Absyn::Path::IDENT { name: literal!("max") }),
            expLst: list![Expression::makeScalarArray(expLst, DAE::T_INTEGER_DEFAULT().clone())],
            attr: DAE::callAttrBuiltinInteger().clone(),
        })
    } else {
        (expLst).head().cloned()?
    };
    firedEqn = metamodelica::Ref::new(DAE::Element::EQUATION {
        exp: exp,
        scalar: rhs,
        source: DAE::emptyElementSource().clone(),
    });
    eqs = metamodelica::cons(firedEqn, eqs);
    exp = metamodelica::Ref::new(DAE::Exp::CREF {
        componentRef: activeStateRef.clone(),
        ty: DAE::T_INTEGER_DEFAULT().clone(),
    });
    expCond = metamodelica::Ref::new(DAE::Exp::CREF {
        componentRef: resetRef.clone(),
        ty: DAE::T_BOOL_DEFAULT().clone(),
    });
    expThen = metamodelica::Ref::new(DAE::Exp::ICONST { integer: 1 });
    exp1 = metamodelica::Ref::new(DAE::Exp::RELATION {
        exp1: metamodelica::Ref::new(DAE::Exp::CREF {
            componentRef: firedRef.clone(),
            ty: DAE::T_INTEGER_DEFAULT().clone(),
        }),
        operator: DAE::Operator::GREATER {
            ty: DAE::T_INTEGER_DEFAULT().clone(),
        },
        exp2: metamodelica::Ref::new(DAE::Exp::ICONST { integer: 0 }),
        index: -1,
        optionExpisASUB: None,
    });
    exp2 = metamodelica::Ref::new(DAE::Exp::CREF {
        componentRef: qCref(
            literal!("tTo"),
            tArrayInteger,
            list![metamodelica::Ref::new(DAE::Subscript::INDEX {
                exp: metamodelica::Ref::new(DAE::Exp::CREF {
                    componentRef: firedRef.clone(),
                    ty: DAE::T_INTEGER_DEFAULT().clone()
                })
            })],
            &preRef,
        )?,
        ty: DAE::T_INTEGER_DEFAULT().clone(),
    });
    expElse = metamodelica::Ref::new(DAE::Exp::IFEXP {
        expCond: exp1,
        expThen: exp2,
        expElse: metamodelica::Ref::new(DAE::Exp::CREF {
            componentRef: selectedStateRef,
            ty: DAE::T_INTEGER_DEFAULT().clone(),
        }),
    });
    rhs = metamodelica::Ref::new(DAE::Exp::IFEXP {
        expCond: expCond,
        expThen: expThen,
        expElse: expElse,
    });
    activeStateEqn = metamodelica::Ref::new(DAE::Element::EQUATION {
        exp: exp,
        scalar: rhs,
        source: DAE::emptyElementSource().clone(),
    });
    eqs = metamodelica::cons(activeStateEqn, eqs);
    exp = metamodelica::Ref::new(DAE::Exp::CREF {
        componentRef: activeResetRef,
        ty: DAE::T_BOOL_DEFAULT().clone(),
    });
    expCond = metamodelica::Ref::new(DAE::Exp::CREF {
        componentRef: resetRef.clone(),
        ty: DAE::T_BOOL_DEFAULT().clone(),
    });
    expThen = metamodelica::Ref::new(DAE::Exp::BCONST { bool: true });
    exp1 = metamodelica::Ref::new(DAE::Exp::RELATION {
        exp1: metamodelica::Ref::new(DAE::Exp::CREF {
            componentRef: firedRef.clone(),
            ty: DAE::T_INTEGER_DEFAULT().clone(),
        }),
        operator: DAE::Operator::GREATER {
            ty: DAE::T_INTEGER_DEFAULT().clone(),
        },
        exp2: metamodelica::Ref::new(DAE::Exp::ICONST { integer: 0 }),
        index: -1,
        optionExpisASUB: None,
    });
    exp2 = metamodelica::Ref::new(DAE::Exp::CREF {
        componentRef: qCref(
            literal!("tReset"),
            tArrayBool,
            list![metamodelica::Ref::new(DAE::Subscript::INDEX {
                exp: metamodelica::Ref::new(DAE::Exp::CREF {
                    componentRef: firedRef,
                    ty: DAE::T_INTEGER_DEFAULT().clone()
                })
            })],
            &preRef,
        )?,
        ty: DAE::T_INTEGER_DEFAULT().clone(),
    });
    expElse = metamodelica::Ref::new(DAE::Exp::IFEXP {
        expCond: exp1,
        expThen: exp2,
        expElse: metamodelica::Ref::new(DAE::Exp::CREF {
            componentRef: selectedResetRef,
            ty: DAE::T_BOOL_DEFAULT().clone(),
        }),
    });
    rhs = metamodelica::Ref::new(DAE::Exp::IFEXP {
        expCond: expCond,
        expThen: expThen,
        expElse: expElse,
    });
    activeResetEqn = metamodelica::Ref::new(DAE::Element::EQUATION {
        exp: exp,
        scalar: rhs,
        source: DAE::emptyElementSource().clone(),
    });
    eqs = metamodelica::cons(activeResetEqn, eqs);
    exp = metamodelica::Ref::new(DAE::Exp::CREF {
        componentRef: nextStateRef.clone(),
        ty: DAE::T_INTEGER_DEFAULT().clone(),
    });
    expCond = metamodelica::Ref::new(DAE::Exp::CREF {
        componentRef: activeRef.clone(),
        ty: DAE::T_BOOL_DEFAULT().clone(),
    });
    expThen = metamodelica::Ref::new(DAE::Exp::CREF {
        componentRef: activeStateRef.clone(),
        ty: DAE::T_INTEGER_DEFAULT().clone(),
    });
    expElse = metamodelica::Ref::new(DAE::Exp::CALL {
        path: metamodelica::Ref::new(Absyn::Path::IDENT {
            name: literal!("previous"),
        }),
        expLst: list![metamodelica::Ref::new(DAE::Exp::CREF {
            componentRef: nextStateRef,
            ty: DAE::T_INTEGER_DEFAULT().clone()
        })],
        attr: DAE::callAttrBuiltinImpureInteger().clone(),
    });
    rhs = metamodelica::Ref::new(DAE::Exp::IFEXP {
        expCond: expCond,
        expThen: expThen,
        expElse: expElse,
    });
    nextStateEqn = metamodelica::Ref::new(DAE::Element::EQUATION {
        exp: exp,
        scalar: rhs,
        source: DAE::emptyElementSource().clone(),
    });
    eqs = metamodelica::cons(nextStateEqn, eqs);
    exp = metamodelica::Ref::new(DAE::Exp::CREF {
        componentRef: nextResetRef.clone(),
        ty: DAE::T_BOOL_DEFAULT().clone(),
    });
    expCond = metamodelica::Ref::new(DAE::Exp::CREF {
        componentRef: activeRef.clone(),
        ty: DAE::T_BOOL_DEFAULT().clone(),
    });
    expThen = metamodelica::Ref::new(DAE::Exp::BCONST { bool: false });
    expElse = metamodelica::Ref::new(DAE::Exp::CALL {
        path: metamodelica::Ref::new(Absyn::Path::IDENT {
            name: literal!("previous"),
        }),
        expLst: list![metamodelica::Ref::new(DAE::Exp::CREF {
            componentRef: nextResetRef,
            ty: DAE::T_BOOL_DEFAULT().clone()
        })],
        attr: DAE::callAttrBuiltinImpureBool().clone(),
    });
    rhs = metamodelica::Ref::new(DAE::Exp::IFEXP {
        expCond: expCond,
        expThen: expThen,
        expElse: expElse,
    });
    nextResetEqn = metamodelica::Ref::new(DAE::Element::EQUATION {
        exp: exp,
        scalar: rhs,
        source: DAE::emptyElementSource().clone(),
    });
    eqs = metamodelica::cons(nextResetEqn, eqs);
    for mut i in 1..=nStates {
        exp = metamodelica::Ref::new(DAE::Exp::CREF {
            componentRef: metamodelica::arrayGet(activeResetStatesRefs.clone(), i)?,
            ty: DAE::T_BOOL_DEFAULT().clone(),
        });
        expCond = metamodelica::Ref::new(DAE::Exp::CREF {
            componentRef: resetRef.clone(),
            ty: DAE::T_BOOL_DEFAULT().clone(),
        });
        expThen = metamodelica::Ref::new(DAE::Exp::BCONST { bool: true });
        expElse = metamodelica::Ref::new(DAE::Exp::CALL {
            path: metamodelica::Ref::new(Absyn::Path::IDENT {
                name: literal!("previous"),
            }),
            expLst: list![metamodelica::Ref::new(DAE::Exp::CREF {
                componentRef: metamodelica::arrayGet(nextResetStatesRefs.clone(), i)?,
                ty: DAE::T_BOOL_DEFAULT().clone()
            })],
            attr: DAE::callAttrBuiltinImpureBool().clone(),
        });
        rhs = metamodelica::Ref::new(DAE::Exp::IFEXP {
            expCond: expCond,
            expThen: expThen,
            expElse: expElse,
        });
        eqs = metamodelica::cons(
            metamodelica::Ref::new(DAE::Element::EQUATION {
                exp: exp,
                scalar: rhs,
                source: DAE::emptyElementSource().clone(),
            }),
            eqs,
        );
    }
    for mut i in 1..=nStates {
        exp = metamodelica::Ref::new(DAE::Exp::CREF {
            componentRef: metamodelica::arrayGet(nextResetStatesRefs.clone(), i)?,
            ty: DAE::T_BOOL_DEFAULT().clone(),
        });
        expCond = metamodelica::Ref::new(DAE::Exp::CREF {
            componentRef: activeRef.clone(),
            ty: DAE::T_BOOL_DEFAULT().clone(),
        });
        exp1 = metamodelica::Ref::new(DAE::Exp::RELATION {
            exp1: metamodelica::Ref::new(DAE::Exp::CREF {
                componentRef: activeStateRef.clone(),
                ty: DAE::T_INTEGER_DEFAULT().clone(),
            }),
            operator: DAE::Operator::EQUAL {
                ty: DAE::T_INTEGER_DEFAULT().clone(),
            },
            exp2: metamodelica::Ref::new(DAE::Exp::ICONST { integer: i }),
            index: -1,
            optionExpisASUB: None,
        });
        expThen = metamodelica::Ref::new(DAE::Exp::IFEXP {
            expCond: exp1,
            expThen: metamodelica::Ref::new(DAE::Exp::BCONST { bool: false }),
            expElse: metamodelica::Ref::new(DAE::Exp::CREF {
                componentRef: metamodelica::arrayGet(activeResetStatesRefs.clone(), i)?,
                ty: DAE::T_BOOL_DEFAULT().clone(),
            }),
        });
        expElse = metamodelica::Ref::new(DAE::Exp::CALL {
            path: metamodelica::Ref::new(Absyn::Path::IDENT {
                name: literal!("previous"),
            }),
            expLst: list![metamodelica::Ref::new(DAE::Exp::CREF {
                componentRef: metamodelica::arrayGet(nextResetStatesRefs.clone(), i)?,
                ty: DAE::T_BOOL_DEFAULT().clone()
            })],
            attr: DAE::callAttrBuiltinImpureBool().clone(),
        });
        rhs = metamodelica::Ref::new(DAE::Exp::IFEXP {
            expCond: expCond,
            expThen: expThen,
            expElse: expElse,
        });
        eqs = metamodelica::cons(
            metamodelica::Ref::new(DAE::Element::EQUATION {
                exp: exp,
                scalar: rhs,
                source: DAE::emptyElementSource().clone(),
            }),
            eqs,
        );
    }
    for mut i in 1..=nStates {
        exp = metamodelica::Ref::new(DAE::Exp::CREF {
            componentRef: metamodelica::arrayGet(finalStatesRefs.clone(), i)?,
            ty: DAE::T_BOOL_DEFAULT().clone(),
        });
        expLst = metamodelica::nil();
        for mut j in 1..=nTransitions {
            expCond = metamodelica::Ref::new(DAE::Exp::RELATION {
                exp1: metamodelica::Ref::new(DAE::Exp::CREF {
                    componentRef: metamodelica::arrayGet(tFromRefs.clone(), j)?,
                    ty: DAE::T_INTEGER_DEFAULT().clone(),
                }),
                operator: DAE::Operator::EQUAL {
                    ty: DAE::T_INTEGER_DEFAULT().clone(),
                },
                exp2: metamodelica::Ref::new(DAE::Exp::ICONST { integer: i }),
                index: -1,
                optionExpisASUB: None,
            });
            expLst = metamodelica::cons(
                metamodelica::Ref::new(DAE::Exp::IFEXP {
                    expCond: expCond,
                    expThen: metamodelica::Ref::new(DAE::Exp::ICONST { integer: 1 }),
                    expElse: metamodelica::Ref::new(DAE::Exp::ICONST { integer: 0 }),
                }),
                expLst,
            );
        }
        exp1 = if (((expLst).len() as i32) > 1) {
            metamodelica::Ref::new(DAE::Exp::CALL {
                path: metamodelica::Ref::new(Absyn::Path::IDENT { name: literal!("max") }),
                expLst: list![Expression::makeScalarArray(expLst, DAE::T_INTEGER_DEFAULT().clone())],
                attr: DAE::callAttrBuiltinInteger().clone(),
            })
        } else {
            (expLst).head().cloned()?
        };
        rhs = metamodelica::Ref::new(DAE::Exp::RELATION {
            exp1: exp1,
            operator: DAE::Operator::EQUAL {
                ty: DAE::T_INTEGER_DEFAULT().clone(),
            },
            exp2: metamodelica::Ref::new(DAE::Exp::ICONST { integer: 0 }),
            index: -1,
            optionExpisASUB: None,
        });
        eqs = metamodelica::cons(
            metamodelica::Ref::new(DAE::Element::EQUATION {
                exp: exp,
                scalar: rhs,
                source: DAE::emptyElementSource().clone(),
            }),
            eqs,
        );
    }
    exp = metamodelica::Ref::new(DAE::Exp::CREF {
        componentRef: stateMachineInFinalStateRef,
        ty: DAE::T_BOOL_DEFAULT().clone(),
    });
    rhs = metamodelica::Ref::new(DAE::Exp::CREF {
        componentRef: qCref(
            literal!("finalStates"),
            nStatesArrayBool,
            list![metamodelica::Ref::new(DAE::Subscript::INDEX {
                exp: metamodelica::Ref::new(DAE::Exp::CREF {
                    componentRef: activeStateRef,
                    ty: DAE::T_INTEGER_DEFAULT().clone()
                })
            })],
            &preRef,
        )?,
        ty: DAE::T_BOOL_DEFAULT().clone(),
    });
    eqs = metamodelica::cons(
        metamodelica::Ref::new(DAE::Element::EQUATION {
            exp: exp,
            scalar: rhs,
            source: DAE::emptyElementSource().clone(),
        }),
        eqs,
    );
    flatSmSemantics = FlatSmSemantics {
        ident: ident,
        smComps: metamodelica::arrayFromVec(q.into_iter().cloned().collect()),
        t: t,
        c: cExps,
        vars: vars,
        knowns: knowns,
        eqs: eqs,
        pvars: metamodelica::nil(),
        peqs: metamodelica::nil(),
        enclosingState: None,
    };
    Ok(flatSmSemantics)
}

fn qCref(
    mut ident: ArcStr,
    mut identType: metamodelica::Ref<DAE::Type>,
    mut subscriptLst: metamodelica::List<metamodelica::Ref<DAE::Subscript>>,
    mut componentRef: &metamodelica::Ref<DAE::ComponentRef>,
) -> Result<metamodelica::Ref<DAE::ComponentRef>> {
    let mut outQual: metamodelica::Ref<DAE::ComponentRef>;
    outQual = ComponentReference::joinCrefs(
        componentRef,
        metamodelica::Ref::new(DAE::ComponentRef::CREF_IDENT {
            ident: ident,
            identType: identType,
            subscriptLst: subscriptLst,
        }),
    )?;
    Ok(outQual)
}

fn createVarWithDefaults(
    mut componentRef: metamodelica::Ref<DAE::ComponentRef>,
    mut kind: DAE::VarKind,
    mut ty: metamodelica::Ref<DAE::Type>,
    mut dims: metamodelica::List<metamodelica::Ref<DAE::Dimension>>,
) -> metamodelica::Ref<DAE::Element> {
    let mut var: metamodelica::Ref<DAE::Element>;
    var = metamodelica::Ref::new(DAE::Element::VAR {
        componentRef: componentRef,
        kind: kind,
        direction: openmodelica_frontend_types::DAE::VarDirection::BIDIR,
        parallelism: openmodelica_frontend_types::DAE::VarParallelism::NON_PARALLEL,
        protection: openmodelica_frontend_types::DAE::VarVisibility::PUBLIC,
        ty: ty,
        binding: None,
        dims: dims,
        connectorType: openmodelica_frontend_types::DAE::ConnectorType::interned_NON_CONNECTOR(),
        source: DAE::emptyElementSource().clone(),
        variableAttributesOption: None,
        comment: None,
        innerOuter: openmodelica_ast::Absyn::InnerOuter::NOT_INNER_OUTER,
        encrypted: false,
    });
    var
}

fn createVarWithStartValue(
    mut componentRef: metamodelica::Ref<DAE::ComponentRef>,
    mut kind: DAE::VarKind,
    mut ty: metamodelica::Ref<DAE::Type>,
    mut startExp: metamodelica::Ref<DAE::Exp>,
    mut dims: metamodelica::List<metamodelica::Ref<DAE::Dimension>>,
) -> Result<metamodelica::Ref<DAE::Element>> {
    let mut outVar: metamodelica::Ref<DAE::Element>;
    let mut var: metamodelica::Ref<DAE::Element>;
    var = metamodelica::Ref::new(DAE::Element::VAR {
        componentRef: componentRef,
        kind: kind,
        direction: openmodelica_frontend_types::DAE::VarDirection::BIDIR,
        parallelism: openmodelica_frontend_types::DAE::VarParallelism::NON_PARALLEL,
        protection: openmodelica_frontend_types::DAE::VarVisibility::PUBLIC,
        ty: ty,
        binding: None,
        dims: dims,
        connectorType: openmodelica_frontend_types::DAE::ConnectorType::interned_NON_CONNECTOR(),
        source: DAE::emptyElementSource().clone(),
        variableAttributesOption: None,
        comment: None,
        innerOuter: openmodelica_ast::Absyn::InnerOuter::NOT_INNER_OUTER,
        encrypted: false,
    });
    outVar = setVarFixedStartValue(var, startExp)?;
    Ok(outVar)
}

fn createTandC(
    mut inSMComps: metamodelica::List<metamodelica::Ref<DAE::Element>>,
    mut inTransitions: metamodelica::List<metamodelica::Ref<DAE::Element>>,
) -> Result<(
    metamodelica::List<Transition>,
    metamodelica::List<metamodelica::Ref<DAE::Exp>>,
)> {
    let mut t: metamodelica::List<Transition>;
    let mut c: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
    let mut transitions: metamodelica::List<Transition>;
    transitions = List::map1(
        inTransitions,
        &move |__a0: metamodelica::Ref<DAE::Element>, __a1: metamodelica::List<metamodelica::Ref<DAE::Element>>| {
            createTransition(&__a0, &__a1)
        },
        inSMComps,
    )?;
    t = List::sort(
        transitions,
        (std::sync::Arc::new(fnptr!(priorityLt, Transition, Transition))
            as std::sync::Arc<dyn ::std::ops::Fn(Transition, Transition) -> Result<bool> + 'static>),
    )?;
    c = List::map(t.clone(), &fnptr!(extractCondtionFromTransition, Transition))?;
    Ok((t, c))
}

fn extractCondtionFromTransition(mut trans: Transition) -> metamodelica::Ref<DAE::Exp> {
    let mut condition: metamodelica::Ref<DAE::Exp>;
    let Transition { condition: __pa0, .. } = trans;
    condition = metamodelica::Own::own(__pa0);
    condition
}

fn priorityLt(mut inTrans1: Transition, mut inTrans2: Transition) -> bool {
    let mut res: bool;
    let mut priority1: i32;
    let mut priority2: i32;
    let Transition { priority: __pa0, .. } = inTrans1;
    priority1 = metamodelica::Own::own(__pa0);
    let Transition { priority: __pa1, .. } = inTrans2;
    priority2 = metamodelica::Own::own(__pa1);
    res = intLt(priority1, priority2);
    res
}

fn createTransition(
    mut transitionElem: &metamodelica::Ref<DAE::Element>,
    mut states: &metamodelica::List<metamodelica::Ref<DAE::Element>>,
) -> Result<Transition> {
    let mut trans: Transition;
    let mut crefFrom: metamodelica::Ref<DAE::ComponentRef>;
    let mut crefTo: metamodelica::Ref<DAE::ComponentRef>;
    let mut from: i32;
    let mut to: i32;
    let mut condition: metamodelica::Ref<DAE::Exp>;
    let mut immediate: bool = true;
    let mut reset: bool = true;
    let mut synchronize: bool = false;
    let mut priority: i32 = 1;
    let (__pa0, __pa1, __pa2, __pa3, __pa4, __pa5, __pa6) = ::match_deref::match_deref! { match &((*transitionElem)) {
        Deref @ DAE::Element::NORETCALL { exp: Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "transition" }, expLst: Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Exp::CREF { componentRef: __pa0, .. }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Exp::CREF { componentRef: __pa1, .. }, tail: Deref @ metamodelica::ListNode::Cons { head: __pa2, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Exp::BCONST { bool: __pa3 }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Exp::BCONST { bool: __pa4 }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Exp::BCONST { bool: __pa5 }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Exp::ICONST { integer: __pa6 }, tail: Deref @ metamodelica::ListNode::Nil } } } } } } }, .. }, .. } => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone(), __pa4.clone(), __pa5.clone(), __pa6.clone()),
        _ => return Err("pattern mismatch"),
    } };
    crefFrom = metamodelica::Own::own(__pa0);
    crefTo = metamodelica::Own::own(__pa1);
    condition = metamodelica::Own::own(__pa2);
    immediate = metamodelica::Own::own(__pa3);
    reset = metamodelica::Own::own(__pa4);
    synchronize = metamodelica::Own::own(__pa5);
    priority = metamodelica::Own::own(__pa6);
    from = List::position1OnTrue(
        states,
        &move |__a0: metamodelica::Ref<DAE::Element>, __a1: metamodelica::Ref<DAE::ComponentRef>| {
            sMCompEqualsRef(&__a0, &__a1)
        },
        crefFrom,
    )?;
    to = List::position1OnTrue(
        states,
        &move |__a0: metamodelica::Ref<DAE::Element>, __a1: metamodelica::Ref<DAE::ComponentRef>| {
            sMCompEqualsRef(&__a0, &__a1)
        },
        crefTo,
    )?;
    trans = Transition {
        from: from,
        to: to,
        condition: condition,
        immediate: immediate,
        reset: reset,
        synchronize: synchronize,
        priority: priority,
    };
    Ok(trans)
}

fn isFlatSm(mut inElement: &metamodelica::Ref<DAE::Element>) -> bool {
    let mut outResult: bool;
    outResult = (match &**inElement {
        DAE::Element::FLAT_SM { .. } => true,
        _ => false,
    });
    outResult
}

fn isSMComp(mut inElement: &metamodelica::Ref<DAE::Element>) -> bool {
    let mut outResult: bool;
    outResult = (match &**inElement {
        DAE::Element::SM_COMP { .. } => true,
        _ => false,
    });
    outResult
}

fn isTransition(mut inElement: &metamodelica::Ref<DAE::Element>) -> bool {
    let mut result: bool;
    result = (::match_deref::match_deref! { match inElement {
        Deref @ DAE::Element::NORETCALL { exp: Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "transition" }, .. }, .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    result
}

fn isInitialState(mut inElement: &metamodelica::Ref<DAE::Element>) -> bool {
    let mut result: bool;
    result = (::match_deref::match_deref! { match inElement {
        Deref @ DAE::Element::NORETCALL { exp: Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "initialState" }, .. }, .. } => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    result
}

fn isEquation(mut inElement: &metamodelica::Ref<DAE::Element>) -> bool {
    let mut result: bool;
    result = (match &**inElement {
        DAE::Element::EQUATION { .. } => true,
        _ => false,
    });
    result
}

fn isEquationOrWhenEquation(mut inElement: &metamodelica::Ref<DAE::Element>) -> bool {
    let mut result: bool;
    result = (match &**inElement {
        DAE::Element::EQUATION { .. } => true,
        DAE::Element::WHEN_EQUATION { .. } => true,
        _ => false,
    });
    result
}

fn isPreOrPreviousEquation(mut inElement: &metamodelica::Ref<DAE::Element>) -> Result<bool> {
    let mut result: bool;
    result = (match &**inElement {
        DAE::Element::EQUATION { exp, scalar, source: _ } => {
            Expression::expHasPre(exp.clone())?
                || Expression::expHasPre(scalar.clone())?
                || Expression::expHasPrevious(exp.clone())?
                || Expression::expHasPrevious(scalar.clone())?
        }
        _ => false,
    });
    Ok(result)
}

fn isVar(mut inElement: &metamodelica::Ref<DAE::Element>) -> bool {
    let mut result: bool;
    result = (match &**inElement {
        DAE::Element::VAR { .. } => true,
        _ => false,
    });
    result
}

fn sMCompEqualsRef(
    mut inElement: &metamodelica::Ref<DAE::Element>,
    mut inCref: &metamodelica::Ref<DAE::ComponentRef>,
) -> Result<bool> {
    let mut result: bool;
    result = (match &**inElement {
        DAE::Element::SM_COMP { componentRef: cref, .. } if (ComponentReferenceBasics::crefEqual(cref, inCref)?) => {
            true
        }
        _ => false,
    });
    Ok(result)
}

pub(crate) fn dumpTransitionStr(mut transition: Transition) -> Result<ArcStr> {
    let mut transitionStr: ArcStr;
    let mut from: i32;
    let mut to: i32;
    let mut condition: metamodelica::Ref<DAE::Exp>;
    let mut immediate: bool;
    let mut reset: bool;
    let mut synchronize: bool;
    let mut priority: i32;
    let Transition {
        from: __pa0,
        to: __pa1,
        condition: __pa2,
        immediate: __pa3,
        reset: __pa4,
        synchronize: __pa5,
        priority: __pa6,
    } = transition;
    from = metamodelica::Own::own(__pa0);
    to = metamodelica::Own::own(__pa1);
    condition = metamodelica::Own::own(__pa2);
    immediate = metamodelica::Own::own(__pa3);
    reset = metamodelica::Own::own(__pa4);
    synchronize = metamodelica::Own::own(__pa5);
    priority = metamodelica::Own::own(__pa6);
    transitionStr = {
        let mut __mm_s = String::new();
        __mm_s.push_str(&*literal!("TRANSITION(from="));
        __mm_s.push_str(&*intString(from));
        __mm_s.push_str(&*literal!(", to="));
        __mm_s.push_str(&*intString(to));
        __mm_s.push_str(&*literal!(", condition="));
        __mm_s.push_str(&*ExpressionBasics::printExpStr(condition)?);
        __mm_s.push_str(&*literal!(", immediate="));
        __mm_s.push_str(&*boolString(immediate));
        __mm_s.push_str(&*literal!(", reset="));
        __mm_s.push_str(&*boolString(reset));
        __mm_s.push_str(&*literal!(", synchronize="));
        __mm_s.push_str(&*boolString(synchronize));
        __mm_s.push_str(&*literal!(", priority="));
        __mm_s.push_str(&*intString(priority));
        __mm_s.push_str(&*literal!(")"));
        ArcStr::from(__mm_s)
    };
    Ok(transitionStr)
}

fn wrapHack(
    mut cache: &FCore::Cache,
    mut inElementLst: &metamodelica::List<metamodelica::Ref<DAE::Element>>,
) -> Result<metamodelica::List<metamodelica::Ref<DAE::Element>>> {
    let mut outElementLst: metamodelica::List<metamodelica::Ref<DAE::Element>>;
    let mut eqnLst: metamodelica::List<metamodelica::Ref<DAE::Element>>;
    let mut otherLst: metamodelica::List<metamodelica::Ref<DAE::Element>>;
    let mut whenEq: metamodelica::Ref<DAE::Element>;
    let mut cond1: metamodelica::Ref<DAE::Exp>;
    let mut cond2: metamodelica::Ref<DAE::Exp>;
    let mut condition: metamodelica::Ref<DAE::Exp>;
    let mut condLst: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
    let mut tArrayBool: metamodelica::Ref<DAE::Type>;
    cond1 = metamodelica::Ref::new(DAE::Exp::CALL {
        path: metamodelica::Ref::new(Absyn::Path::IDENT {
            name: literal!("initial"),
        }),
        expLst: metamodelica::nil(),
        attr: DAE::callAttrBuiltinImpureBool().clone(),
    });
    cond2 = metamodelica::Ref::new(DAE::Exp::CALL {
        path: metamodelica::Ref::new(Absyn::Path::IDENT {
            name: literal!("sample"),
        }),
        expLst: list![
            metamodelica::Ref::new(DAE::Exp::RCONST {
                real: Flags::getConfigReal(Flags::DEFAULT_CLOCK_PERIOD.clone())?
            }),
            metamodelica::Ref::new(DAE::Exp::RCONST {
                real: Flags::getConfigReal(Flags::DEFAULT_CLOCK_PERIOD.clone())?
            })
        ],
        attr: DAE::callAttrBuiltinImpureBool().clone(),
    });
    tArrayBool = metamodelica::Ref::new(DAE::Type::T_ARRAY {
        ty: DAE::T_BOOL_DEFAULT().clone(),
        dims: list![metamodelica::Ref::new(DAE::Dimension::DIM_INTEGER { integer: 2 })],
    });
    if Flags::getConfigBool(Flags::CT_STATE_MACHINES.clone())? {
        condLst = List::filterMap1(
            inElementLst,
            &move |__a0: metamodelica::Ref<DAE::Element>, __a1: ArcStr| extractSmOfExps(&__a0, &__a1),
            literal!("cImmediate"),
        );
        (eqnLst, otherLst) = List::extractOnTrue(inElementLst, &move |__a0: metamodelica::Ref<DAE::Element>| {
            isPreOrPreviousEquation(&__a0)
        })?;
        condition = metamodelica::Ref::new(DAE::Exp::ARRAY {
            ty: tArrayBool,
            scalar: true,
            array: metamodelica::cons(cond1, condLst),
        });
    } else {
        (eqnLst, otherLst) = List::extractOnTrue(
            inElementLst,
            &move |__a0: metamodelica::Ref<DAE::Element>| -> metamodelica::Result<_> {
                ::std::result::Result::Ok(isEquation(&__a0))
            },
        )?;
        condition = metamodelica::Ref::new(DAE::Exp::ARRAY {
            ty: tArrayBool,
            scalar: true,
            array: list![cond1, cond2],
        });
    }
    whenEq = metamodelica::Ref::new(DAE::Element::WHEN_EQUATION {
        condition: condition,
        equations: eqnLst,
        elsewhen_: None,
        source: DAE::emptyElementSource().clone(),
    });
    outElementLst = listAppend(otherLst, list![whenEq]);
    Ok(outElementLst)
}

fn extractSmOfExps(
    mut inElem: &metamodelica::Ref<DAE::Element>,
    mut inLastIdent: &ArcStr,
) -> Result<metamodelica::Ref<DAE::Exp>> {
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    outExp = (match &**inElem {
        DAE::Element::EQUATION { exp, .. } => {
            let mut cref: metamodelica::Ref<DAE::ComponentRef>;
            let mut firstIdent: ArcStr;
            let mut lastIdent: ArcStr;
            let __pa0 = ::match_deref::match_deref! { match &(exp.clone()) {
                Deref @ DAE::Exp::CREF { componentRef: __pa0, .. } => __pa0.clone(),
                _ => return Err("pattern mismatch"),
            } };
            cref = metamodelica::Own::own(__pa0);
            firstIdent = ComponentReferenceBasics::crefFirstIdent(&cref)?;
            let true = (metamodelica::stringEq(&firstIdent, &(literal!("smOf")))) else {
                return Err("pattern mismatch");
            };
            lastIdent = ComponentReferenceBasics::crefLastIdent(&cref)?;
            let true = (metamodelica::stringEq(&lastIdent, &inLastIdent)) else {
                return Err("pattern mismatch");
            };
            exp.clone()
        }
        _ => return Err("match: no arm matched"),
    });
    Ok(outExp)
}

fn traversingSubsPreForPrevious(
    mut inExp: metamodelica::Ref<DAE::Exp>,
    mut inHitCount: i32,
) -> (metamodelica::Ref<DAE::Exp>, i32) {
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    let mut outHitCount: i32;
    (outExp, outHitCount) = (::match_deref::match_deref! { match &(inExp.clone()) {
        Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "previous" }, expLst, attr } => {
            (metamodelica::Ref::new(DAE::Exp::CALL { path: metamodelica::Ref::new(Absyn::Path::IDENT { name: literal!("pre") }), expLst: expLst.clone(), attr: attr.clone() }), inHitCount + 1)
        },
        _ => {
            (inExp, inHitCount)
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    (outExp, outHitCount)
}

fn traversingSubsXForSampleX(
    mut inExp: metamodelica::Ref<DAE::Exp>,
    mut inHitCount: i32,
) -> (metamodelica::Ref<DAE::Exp>, i32) {
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    let mut outHitCount: i32;
    (outExp, outHitCount) = (::match_deref::match_deref! { match &(inExp.clone()) {
        Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "sample" }, expLst: Deref @ metamodelica::ListNode::Cons { head: expX, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Exp::CLKCONST { clk: Deref @ DAE::ClockKind::INFERRED_CLOCK { .. } }, tail: Deref @ metamodelica::ListNode::Nil } }, attr: _ } => {
            (expX.clone(), inHitCount + 1)
        },
        _ => {
            (inExp, inHitCount)
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    (outExp, outHitCount)
}
