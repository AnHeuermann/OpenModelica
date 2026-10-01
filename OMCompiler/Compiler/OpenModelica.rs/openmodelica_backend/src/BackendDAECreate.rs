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

use crate::BackendDAEUtil;
use crate::BackendDump;
use crate::BackendEquation;
use crate::BackendVarTransform;
use crate::BackendVariable;
use crate::ExpressionSolve;
use crate::Vectorization;
use openmodelica_ast::Absyn;
use openmodelica_backend_types::BackendDAE;
use openmodelica_backend_types::ZeroCrossings;
use openmodelica_error::ErrorExt;
use openmodelica_frontend::CheckModel;
use openmodelica_frontend::ConnectUtil;
use openmodelica_frontend::HashTableCrToExpSourceTpl;
use openmodelica_frontend::HashTableExpToExp;
use openmodelica_frontend::HashTableExpToIndex;
use openmodelica_frontend_base::ComponentReference;
use openmodelica_frontend_base::DAEDump;
use openmodelica_frontend_base::DAEUtil;
use openmodelica_frontend_base::Expression;
use openmodelica_frontend_base::ExpressionDump;
use openmodelica_frontend_base::ExpressionSimplify;
use openmodelica_frontend_base::Inline;
use openmodelica_frontend_base::Types;
use openmodelica_frontend_base::VarTransform;
use openmodelica_frontend_dump::AbsynUtil;
use openmodelica_frontend_dump::AvlTreePathFunction;
use openmodelica_frontend_dump::ComponentReferenceBasics;
use openmodelica_frontend_dump::ElementSource;
use openmodelica_frontend_dump::ExpressionBasics;
use openmodelica_frontend_dump::FCore;
use openmodelica_frontend_dump::HashTable;
use openmodelica_frontend_dump::TypesDump;
use openmodelica_frontend_types::ClassInf;
use openmodelica_frontend_types::DAE;
use openmodelica_frontend_types::SCode;
use openmodelica_util::BaseHashTable;
use openmodelica_util::Config;
use openmodelica_util::Debug;
use openmodelica_util::Error;
use openmodelica_util::ExecStat::execStat;
use openmodelica_util::ExpandableArray;
use openmodelica_util::Flags;
use openmodelica_util::Global;
use openmodelica_util::StackOverflow;
use openmodelica_util::System;
use openmodelica_util::UnorderedMap;
use openmodelica_util::Util;
use openmodelica_util_datatypes_basic::List;

pub fn lower(
    mut lst: DAE::DAElist,
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
    mut inExtraInfo: BackendDAE::ExtraInfo,
) -> Result<metamodelica::Ref<BackendDAE::BackendDAE>> {
    let mut outBackendDAE: metamodelica::Ref<BackendDAE::BackendDAE> =
        <metamodelica::Ref<BackendDAE::BackendDAE> as ::std::default::Default>::default();
    let mut varlst: metamodelica::List<metamodelica::Ref<BackendDAE::Var>> = metamodelica::nil();
    let mut globalKnownVarLst: metamodelica::List<metamodelica::Ref<BackendDAE::Var>> = metamodelica::nil();
    let mut extvarlst: metamodelica::List<metamodelica::Ref<BackendDAE::Var>> = metamodelica::nil();
    let mut vars: BackendDAE::Variables = <BackendDAE::Variables as ::std::default::Default>::default();
    let mut globalKnownVars: BackendDAE::Variables = <BackendDAE::Variables as ::std::default::Default>::default();
    let mut localKnownVars: BackendDAE::Variables = <BackendDAE::Variables as ::std::default::Default>::default();
    let mut vars_1: BackendDAE::Variables = <BackendDAE::Variables as ::std::default::Default>::default();
    let mut extVars: BackendDAE::Variables = <BackendDAE::Variables as ::std::default::Default>::default();
    let mut aliasVars: BackendDAE::Variables = <BackendDAE::Variables as ::std::default::Default>::default();
    let mut extAliasVars: BackendDAE::Variables = <BackendDAE::Variables as ::std::default::Default>::default();
    let mut eqns: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>> = metamodelica::nil();
    let mut reqns: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>> = metamodelica::nil();
    let mut ieqns: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>> = metamodelica::nil();
    let mut constrs: metamodelica::List<metamodelica::Ref<DAE::Constraint>> = metamodelica::nil();
    let mut clsAttrs: metamodelica::List<metamodelica::Ref<DAE::ClassAttributes>> = metamodelica::nil();
    let mut eqnarr: metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>> = <metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>> as ::std::default::Default>::default();
    let mut reqnarr: metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>> = <metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>> as ::std::default::Default>::default();
    let mut ieqnarr: metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>> = <metamodelica::Ref<ExpandableArray::ExpandableArray<metamodelica::Ref<BackendDAE::Equation>>> as ::std::default::Default>::default();
    let mut extObjCls: metamodelica::List<BackendDAE::ExternalObjectClass> = metamodelica::nil();
    let mut symjacs: metamodelica::List<(
        Option<(
            metamodelica::Ref<BackendDAE::BackendDAE>,
            ArcStr,
            metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
            metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
            metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
            metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
        )>,
        (
            metamodelica::List<(
                metamodelica::Ref<DAE::ComponentRef>,
                metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
            )>,
            metamodelica::List<(
                metamodelica::Ref<DAE::ComponentRef>,
                metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
            )>,
            (
                metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
                metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
            ),
            i32,
        ),
        metamodelica::List<metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>>,
        (
            metamodelica::List<(
                metamodelica::Ref<DAE::ComponentRef>,
                metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
            )>,
            metamodelica::List<(
                metamodelica::Ref<DAE::ComponentRef>,
                metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
            )>,
            (
                metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
                metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
            ),
            i32,
        ),
    )> = metamodelica::nil();
    let mut einfo: BackendDAE::EventInfo = <BackendDAE::EventInfo as ::std::default::Default>::default();
    let mut elems: metamodelica::List<metamodelica::Ref<DAE::Element>> = metamodelica::nil();
    let mut aliaseqns: metamodelica::List<metamodelica::Ref<DAE::Element>> = metamodelica::nil();
    let mut functionTree: metamodelica::Ref<AvlTreePathFunction::Tree> =
        metamodelica::Ref::new(AvlTreePathFunction::Tree::EMPTY);
    let mut timeEvents: metamodelica::List<BackendDAE::TimeEvent> = metamodelica::nil();
    let mut numCheckpoints: i32;
    let mut syst: metamodelica::Ref<BackendDAE::EqSystem> =
        <metamodelica::Ref<BackendDAE::EqSystem> as ::std::default::Default>::default();
    numCheckpoints = ErrorExt::getNumCheckpoints();
    let __cp0 = metamodelica::heap_limit::catch(|| -> Result<bool> {
        StackOverflow::clearStacktraceMessages();
        System::tmpTickResetIndex(0, Global::backendDAE_fileSequence.clone());
        System::tmpTickResetIndex(1, Global::backendDAE_cseIndex.clone());
        System::tmpTickResetIndex(0, Global::strongComponent_index.clone());
        functionTree = FCore::getFunctionTree(&inCache);
        functionTree = lowerFunctions(functionTree.clone())?;
        let (DAE::DAE { elementLst: __pa1 }, __pa2, __pa3) =
            processBuiltinExpressions(lst.clone(), functionTree.clone())?;
        elems = metamodelica::Own::own(__pa1);
        functionTree = metamodelica::Own::own(__pa2);
        timeEvents = metamodelica::Own::own(__pa3);
        (
            varlst,
            globalKnownVarLst,
            extvarlst,
            eqns,
            reqns,
            ieqns,
            constrs,
            clsAttrs,
            extObjCls,
            aliaseqns,
            _,
        ) = lower2(
            &(elems.clone().reverse()),
            &functionTree,
            HashTableExpToExp::emptyHashTable(),
            metamodelica::nil(),
            metamodelica::nil(),
            metamodelica::nil(),
            metamodelica::nil(),
            metamodelica::nil(),
            metamodelica::nil(),
            metamodelica::nil(),
            metamodelica::nil(),
            metamodelica::nil(),
            metamodelica::nil(),
        )?;
        globalKnownVars = BackendVariable::listVar(globalKnownVarLst.clone())?;
        localKnownVars = BackendVariable::emptyVars(BaseHashTable::bigBucketSize.clone());
        extVars = BackendVariable::listVar(extvarlst.clone())?;
        aliasVars = BackendVariable::emptyVars(BaseHashTable::bigBucketSize.clone());
        if Flags::isSet(Flags::VECTORIZE.clone())? {
            (varlst, eqns) = Vectorization::collectForLoops(&varlst, &eqns)?;
        }
        vars = BackendVariable::listVar(varlst.clone())?;
        (vars, globalKnownVars, extVars, aliasVars, eqns, reqns, ieqns) = handleAliasEquations(
            &aliaseqns,
            vars.clone(),
            globalKnownVars.clone(),
            extVars.clone(),
            aliasVars.clone(),
            eqns.clone(),
            reqns.clone(),
            ieqns.clone(),
        )?;
        (ieqns, eqns, reqns, extAliasVars, globalKnownVars, extVars) =
            getExternalObjectAlias(&ieqns, &eqns, &reqns, globalKnownVars.clone(), extVars.clone())?;
        aliasVars = BackendVariable::addVariables(extAliasVars.clone(), aliasVars.clone())?;
        (globalKnownVarLst, eqns, reqns, ieqns) = patchRecordBindings(
            &varlst,
            &extvarlst,
            globalKnownVarLst.clone(),
            eqns.clone(),
            reqns.clone(),
            ieqns.clone(),
        )?;
        vars_1 = detectImplicitDiscrete(vars.clone(), globalKnownVars.clone(), &eqns)?;
        eqnarr = BackendEquation::listEquation(&eqns)?;
        reqnarr = BackendEquation::listEquation(&reqns)?;
        ieqnarr = BackendEquation::listEquation(&ieqns)?;
        einfo = BackendDAE::EventInfo {
            timeEvents: timeEvents.clone(),
            zeroCrossings: ZeroCrossings::new()?,
            relations: ZeroCrossings::new()?,
            samples: ZeroCrossings::new()?,
            numberMathEvents: 0,
        };
        symjacs = list![
            (
                None,
                (
                    metamodelica::nil(),
                    metamodelica::nil(),
                    (metamodelica::nil(), metamodelica::nil()),
                    -1
                ),
                metamodelica::nil(),
                (
                    metamodelica::nil(),
                    metamodelica::nil(),
                    (metamodelica::nil(), metamodelica::nil()),
                    -1
                )
            ),
            (
                None,
                (
                    metamodelica::nil(),
                    metamodelica::nil(),
                    (metamodelica::nil(), metamodelica::nil()),
                    -1
                ),
                metamodelica::nil(),
                (
                    metamodelica::nil(),
                    metamodelica::nil(),
                    (metamodelica::nil(), metamodelica::nil()),
                    -1
                )
            ),
            (
                None,
                (
                    metamodelica::nil(),
                    metamodelica::nil(),
                    (metamodelica::nil(), metamodelica::nil()),
                    -1
                ),
                metamodelica::nil(),
                (
                    metamodelica::nil(),
                    metamodelica::nil(),
                    (metamodelica::nil(), metamodelica::nil()),
                    -1
                )
            ),
            (
                None,
                (
                    metamodelica::nil(),
                    metamodelica::nil(),
                    (metamodelica::nil(), metamodelica::nil()),
                    -1
                ),
                metamodelica::nil(),
                (
                    metamodelica::nil(),
                    metamodelica::nil(),
                    (metamodelica::nil(), metamodelica::nil()),
                    -1
                )
            )
        ];
        syst = BackendDAEUtil::createEqSystem(
            vars_1.clone(),
            eqnarr.clone(),
            metamodelica::nil(),
            openmodelica_backend_types::BackendDAE::BaseClockPartitionKind::UNKNOWN_PARTITION,
            reqnarr.clone(),
        );
        outBackendDAE = metamodelica::Ref::new(BackendDAE::BackendDAE {
            eqs: metamodelica::cons(syst.clone(), metamodelica::nil()),
            shared: metamodelica::Ref::new(BackendDAE::Shared {
                globalKnownVars: globalKnownVars.clone(),
                localKnownVars: localKnownVars.clone(),
                externalObjects: extVars.clone(),
                aliasVars: aliasVars.clone(),
                initialEqs: ieqnarr.clone(),
                removedEqs: BackendEquation::emptyEqns(),
                constraints: constrs.clone(),
                classAttrs: clsAttrs.clone(),
                cache: inCache.clone(),
                graph: inEnv.clone(),
                functionTree: functionTree.clone(),
                eventInfo: einfo.clone(),
                extObjClasses: extObjCls.clone(),
                backendDAEType: openmodelica_backend_types::BackendDAE::BackendDAEType::SIMULATION,
                symjacs: symjacs.clone(),
                info: inExtraInfo.clone(),
                partitionsInfo: BackendDAEUtil::emptyPartitionsInfo(),
                daeModeData: BackendDAE::emptyDAEModeData().clone(),
                dataReconciliationData: None,
                timeInterval: None,
            }),
        });
        BackendDAEUtil::checkBackendDAEWithErrorMsg(&outBackendDAE)?;
        BackendDAEUtil::checkAdjacencyMatrixSolvability(
            syst.clone(),
            functionTree.clone(),
            BackendDAEUtil::isInitializationDAE(&outBackendDAE.shared),
        )?;
        if Flags::isSet(Flags::DUMP_BACKENDDAE_INFO.clone())? {
            Error::addSourceMessage(
                &(Error::BACKENDDAEINFO_LOWER.clone()),
                list![
                    ArcStr::from(::std::format!(
                        "{}",
                        BackendEquation::equationArraySize(syst.orderedEqs.clone())?
                    )),
                    ArcStr::from(::std::format!("{}", BackendVariable::varsSize(&syst.orderedVars)))
                ],
                &(Absyn::dummyInfo.clone()),
            )?;
        }
        execStat(&(literal!("Generate backend data structure")))?;
        return Ok(true);
        Ok(false)
    });
    match __cp0 {
        Ok(__returned) => {
            if __returned? {
                return Ok(outBackendDAE.clone());
            }
        }
        Err(_) => {
            let _rearm = metamodelica::heap_limit::RearmOnDrop;
            {
                let __v = None;
                openmodelica_util::Globals::stackoverFlowIndex.with(|__root| *__root.borrow_mut() = __v)
            };
            ErrorExt::rollbackNumCheckpoints(ErrorExt::getNumCheckpoints() - numCheckpoints);
            Error::addInternalError(
                {
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!("Stack overflow in "));
                    __mm_s.push_str(&*literal!("BackendDAECreate.lower"));
                    __mm_s.push_str(&*literal!("...\n"));
                    __mm_s.push_str(&*stringDelimitList(
                        StackOverflow::readableStacktraceMessages()?,
                        literal!("\n"),
                    ));
                    ArcStr::from(__mm_s)
                },
                metamodelica::sourceInfo!("BackEnd/BackendDAECreate.mo"),
            )?;
            StackOverflow::clearStacktraceMessages();
        }
    }
    return Err("fail");
    Ok(outBackendDAE)
}

pub type Functiontuple = (
    Option<metamodelica::Ref<AvlTreePathFunction::Tree>>,
    metamodelica::List<DAE::InlineType>,
);

pub type ArrayBindingList = metamodelica::List<(metamodelica::List<i32>, metamodelica::Ref<DAE::Exp>)>;

fn printArrayBindingList(mut arrayBindingList: &ArrayBindingList) -> Result<ArcStr> {
    let mut r#str: ArcStr = literal!("");
    let mut subscriptLst: metamodelica::List<i32>;
    let mut bindingExp: metamodelica::Ref<DAE::Exp>;
    for mut tpl in &**arrayBindingList {
        (subscriptLst, bindingExp) = tpl.clone();
        r#str = {
            let mut __mm_s = String::new();
            __mm_s.push_str(&*r#str);
            __mm_s.push_str(&*literal!("["));
            ArcStr::from(__mm_s)
        };
        for mut subscript in &*subscriptLst {
            r#str = {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*r#str);
                __mm_s.push_str(&*intString(subscript.clone()));
                __mm_s.push_str(&*literal!(" "));
                ArcStr::from(__mm_s)
            };
        }
        r#str = {
            let mut __mm_s = String::new();
            __mm_s.push_str(&*r#str);
            __mm_s.push_str(&*literal!("]"));
            ArcStr::from(__mm_s)
        };
        r#str = {
            let mut __mm_s = String::new();
            __mm_s.push_str(&*r#str);
            __mm_s.push_str(&*literal!(" : "));
            __mm_s.push_str(&*ExpressionDump::dumpExpStr(bindingExp, 0)?);
            ArcStr::from(__mm_s)
        };
    }
    Ok(r#str)
}

pub(crate) fn patchRecordBindings(
    mut varlst: &metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
    mut extvarlst: &metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
    mut globalKnownVarLst: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
    mut eqns: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
    mut reqns: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
    mut ieqns: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
) -> Result<(
    metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
    metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
    metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
    metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
)> {
    let mut globalKnownVarLst: metamodelica::List<metamodelica::Ref<BackendDAE::Var>> = globalKnownVarLst;
    let mut eqns: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>> = eqns;
    let mut reqns: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>> = reqns;
    let mut ieqns: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>> = ieqns;
    let mut map: metamodelica::Ref<
        UnorderedMap::UnorderedMap<metamodelica::Ref<DAE::ComponentRef>, metamodelica::Ref<DAE::Type>>,
    >;
    let mut arrayMap: metamodelica::Ref<
        UnorderedMap::UnorderedMap<
            metamodelica::Ref<DAE::ComponentRef>,
            metamodelica::List<(metamodelica::List<i32>, metamodelica::Ref<DAE::Exp>)>,
        >,
    >;
    let mut debug: bool = false;
    map = UnorderedMap::new(
        (std::sync::Arc::new(move |__a0: metamodelica::Ref<DAE::ComponentRef>| {
            ComponentReferenceBasics::hashComponentRef(&__a0)
        })
            as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>) -> Result<i32> + 'static>),
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
        1,
    );
    collectRecordTypesVarLst(map.clone(), &globalKnownVarLst)?;
    eqns = List::map(
        eqns,
        &({
            let __pe_b1 = map.clone();
            move |__pe_a0| collectRecordTypesEqn(__pe_a0, __pe_b1.clone())
        }),
    )?;
    reqns = List::map(
        reqns,
        &({
            let __pe_b1 = map.clone();
            move |__pe_a0| collectRecordTypesEqn(__pe_a0, __pe_b1.clone())
        }),
    )?;
    ieqns = List::map(
        ieqns,
        &({
            let __pe_b1 = map.clone();
            move |__pe_a0| collectRecordTypesEqn(__pe_a0, __pe_b1.clone())
        }),
    )?;
    arrayMap = UnorderedMap::new(
        (std::sync::Arc::new(move |__a0: metamodelica::Ref<DAE::ComponentRef>| {
            ComponentReferenceBasics::hashComponentRef(&__a0)
        })
            as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>) -> Result<i32> + 'static>),
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
        1,
    );
    List::apply(
        varlst,
        &({
            let __pe_b1 = map.clone();
            let __pe_b2 = arrayMap.clone();
            move |__pe_a0| collectRecordElementBindings(&__pe_a0, __pe_b1.clone(), __pe_b2.clone())
        }),
    )?;
    List::apply(
        &globalKnownVarLst,
        &({
            let __pe_b1 = map.clone();
            let __pe_b2 = arrayMap.clone();
            move |__pe_a0| collectRecordElementBindings(&__pe_a0, __pe_b1.clone(), __pe_b2.clone())
        }),
    )?;
    List::apply(
        extvarlst,
        &({
            let __pe_b1 = map.clone();
            let __pe_b2 = arrayMap.clone();
            move |__pe_a0| collectRecordElementBindings(&__pe_a0, __pe_b1.clone(), __pe_b2.clone())
        }),
    )?;
    map = collapseArrayBindings(arrayMap.clone(), map)?;
    if debug {
        metamodelica::print(literal!("patchRecordBindings arrayMap:\n"));
        metamodelica::print({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*UnorderedMap::toString(
                arrayMap,
                &move |__a0: metamodelica::Ref<DAE::ComponentRef>| {
                    ComponentReferenceBasics::printComponentRefStr(&__a0)
                },
                &move |__a0: metamodelica::List<(metamodelica::List<i32>, metamodelica::Ref<DAE::Exp>)>| {
                    printArrayBindingList(&__a0)
                },
                literal!("\n"),
                &(literal!(", ")),
            )?);
            __mm_s.push_str(&*literal!("\n"));
            ArcStr::from(__mm_s)
        });
        metamodelica::print(literal!("\npatchRecordBindings map\n"));
        metamodelica::print({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*UnorderedMap::toString(
                map.clone(),
                &move |__a0: metamodelica::Ref<DAE::ComponentRef>| {
                    ComponentReferenceBasics::printComponentRefStr(&__a0)
                },
                &fnptr!(TypesDump::printTypeStr, metamodelica::Ref<DAE::Type>),
                literal!("\n"),
                &(literal!(", ")),
            )?);
            __mm_s.push_str(&*literal!("\n\n"));
            ArcStr::from(__mm_s)
        });
    }
    globalKnownVarLst = updateRecordTypesVarLst(map.clone(), globalKnownVarLst)?;
    eqns = List::map(
        eqns,
        &({
            let __pe_b1 = map.clone();
            move |__pe_a0| updateRecordTypesEqn(__pe_a0, __pe_b1.clone())
        }),
    )?;
    reqns = List::map(
        reqns,
        &({
            let __pe_b1 = map.clone();
            move |__pe_a0| updateRecordTypesEqn(__pe_a0, __pe_b1.clone())
        }),
    )?;
    ieqns = List::map(
        ieqns,
        &({
            let __pe_b1 = map;
            move |__pe_a0| updateRecordTypesEqn(__pe_a0, __pe_b1.clone())
        }),
    )?;
    Ok((globalKnownVarLst, eqns, reqns, ieqns))
}

fn collectRecordElementBindings(
    mut var: &metamodelica::Ref<BackendDAE::Var>,
    mut map: metamodelica::Ref<
        UnorderedMap::UnorderedMap<metamodelica::Ref<DAE::ComponentRef>, metamodelica::Ref<DAE::Type>>,
    >,
    mut arrayMap: metamodelica::Ref<
        UnorderedMap::UnorderedMap<
            metamodelica::Ref<DAE::ComponentRef>,
            metamodelica::List<(metamodelica::List<i32>, metamodelica::Ref<DAE::Exp>)>,
        >,
    >,
) -> Result<()> {
    let mut rec_cref: metamodelica::Ref<DAE::ComponentRef>;
    let mut is_rec: bool;
    (rec_cref, is_rec) = ComponentReference::crefGetFirstRec(&var.varName)?;
    let () = (::match_deref::match_deref! { match &(var.bindExp.clone()) {
        Some(binding) if (is_rec && UnorderedMap::contains(rec_cref.clone(), map.clone())? && Expression::isConst(binding.clone())?) => {
            let mut ty: metamodelica::Ref<DAE::Type>;
            let mut arrayCref: metamodelica::Ref<DAE::ComponentRef>;
            let mut arrayBindingExpList: ArrayBindingList;
            let mut subscriptLst: metamodelica::List<metamodelica::Ref<DAE::Subscript>>;
            let mut intSubLst: metamodelica::List<i32>;
            if ComponentReference::isArrayElement(&var.varName) {
                arrayCref = ComponentReference::crefStripSubsExceptModelSubs(var.varName.clone());
                arrayBindingExpList = UnorderedMap::getOrDefault(arrayCref.clone(), arrayMap.clone(), metamodelica::nil())?;
                subscriptLst = ComponentReferenceBasics::crefSubs(&var.varName)?;
                intSubLst = ({
        let mut __acc: metamodelica::List<i32> = metamodelica::nil();
        for mut subscript in (subscriptLst).into_iter().cloned() {
            let __x = (::match_deref::match_deref! { match &(subscript.clone()) {
        Deref @ DAE::Subscript::INDEX { exp: Deref @ DAE::Exp::ICONST { integer: i } } => {
            i.clone()
        },
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![{ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("BackendDAECreate.collectRecordElementBindings")); __mm_s.push_str(&*literal!(" failed because index not integer.")); ArcStr::from(__mm_s) }])?;
            return Err("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
                UnorderedMap::add(arrayCref, metamodelica::cons((intSubLst, binding.clone()), arrayBindingExpList), arrayMap)?;
            } else {
                ty = (::match_deref::match_deref! { match &(UnorderedMap::getSafe(rec_cref.clone(), map.clone(), metamodelica::sourceInfo!("BackEnd/BackendDAECreate.mo"))?) {
        __esc_ty @ Deref @ DAE::Type::T_COMPLEX { .. } => {
            ty = (*__esc_ty).clone();
            assign_variant_field!(ty => DAE::Type::T_COMPLEX; varLst = ({
        let mut __acc: metamodelica::List<metamodelica::Ref<DAE::Var>> = metamodelica::nil();
        for mut v in (var_field!((*ty).varLst, DAE::Type::T_COMPLEX).clone()).into_iter().cloned() {
            let __x = updateConstantRecordElementBinding(v.clone(), binding.clone(), &(ComponentReferenceBasics::crefLastIdent(&var.varName)?))?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }));
            ty.clone()
        },
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![{ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("BackendDAECreate.collectRecordElementBindings")); __mm_s.push_str(&*literal!(" failed because the type is not T_COMPLEX.")); ArcStr::from(__mm_s) }])?;
            return Err("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
                UnorderedMap::add(rec_cref.clone(), ty, map.clone())?;
            }
            ()
        },
        _ => {
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(())
}

fn updateConstantRecordElementBinding(
    mut var: metamodelica::Ref<DAE::Var>,
    mut binding: metamodelica::Ref<DAE::Exp>,
    mut name: &ArcStr,
) -> Result<metamodelica::Ref<DAE::Var>> {
    let mut var: metamodelica::Ref<DAE::Var> = var;
    let mut r#const: DAE::Const;
    if DAEUtil::isConstVar(&var) && metamodelica::stringEq(&var.name, &name) {
        r#const = if (Expression::isConst(binding.clone())?) {
            openmodelica_frontend_types::DAE::Const::C_CONST
        } else {
            openmodelica_frontend_types::DAE::Const::C_VAR
        };
        assign_field!(
            var.binding = metamodelica::Ref::new(DAE::Binding::EQBOUND {
                exp: binding,
                evaluatedExp: None,
                constant_: r#const,
                source: openmodelica_frontend_types::DAE::BindingSource::BINDING_FROM_DEFAULT_VALUE
            })
        );
    }
    Ok(var)
}

fn collectRecordTypesVarLst(
    mut map: metamodelica::Ref<
        UnorderedMap::UnorderedMap<metamodelica::Ref<DAE::ComponentRef>, metamodelica::Ref<DAE::Type>>,
    >,
    mut varLst: &metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
) -> Result<()> {
    for mut var in &**varLst {
        collectRecordTypesVar(map.clone(), metamodelica::AsArg::as_arg(&var))?;
    }
    Ok(())
}

fn collectRecordTypesVar(
    mut map: metamodelica::Ref<
        UnorderedMap::UnorderedMap<metamodelica::Ref<DAE::ComponentRef>, metamodelica::Ref<DAE::Type>>,
    >,
    mut var: &metamodelica::Ref<BackendDAE::Var>,
) -> Result<()> {
    let () = (::match_deref::match_deref! { match &(var.bindExp.clone()) {
        Some(exp) => {
            Expression::traverseExpTopDown(exp.clone(), &collectRecordTypesExp, map)?;
            ()
        },
        _ => {
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(())
}

fn collectRecordTypesEqn(
    mut eqn: metamodelica::Ref<BackendDAE::Equation>,
    mut map: metamodelica::Ref<
        UnorderedMap::UnorderedMap<metamodelica::Ref<DAE::ComponentRef>, metamodelica::Ref<DAE::Type>>,
    >,
) -> Result<metamodelica::Ref<BackendDAE::Equation>> {
    let mut eqn: metamodelica::Ref<BackendDAE::Equation> = eqn;
    (eqn, _) = BackendEquation::traverseExpsOfEquation(
        eqn,
        (std::sync::Arc::new({
            let __pe_b1: Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>, _) -> Result<_> + 'static> =
                (std::sync::Arc::new(collectRecordTypesExp)
                    as std::sync::Arc<
                        dyn ::std::ops::Fn(
                                metamodelica::Ref<DAE::Exp>,
                                metamodelica::Ref<
                                    UnorderedMap::UnorderedMap<
                                        metamodelica::Ref<DAE::ComponentRef>,
                                        metamodelica::Ref<DAE::Type>,
                                    >,
                                >,
                            ) -> Result<(
                                metamodelica::Ref<DAE::Exp>,
                                bool,
                                metamodelica::Ref<
                                    UnorderedMap::UnorderedMap<
                                        metamodelica::Ref<DAE::ComponentRef>,
                                        metamodelica::Ref<DAE::Type>,
                                    >,
                                >,
                            )> + 'static,
                    >);
            move |__pe_a0, __pe_a2| Expression::traverseExpTopDown(__pe_a0, &*__pe_b1, __pe_a2)
        }) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>, _) -> Result<_> + 'static>),
        map,
    )?;
    Ok(eqn)
}

fn collectRecordTypesExp(
    mut exp: metamodelica::Ref<DAE::Exp>,
    mut map: metamodelica::Ref<
        UnorderedMap::UnorderedMap<metamodelica::Ref<DAE::ComponentRef>, metamodelica::Ref<DAE::Type>>,
    >,
) -> Result<(
    metamodelica::Ref<DAE::Exp>,
    bool,
    metamodelica::Ref<UnorderedMap::UnorderedMap<metamodelica::Ref<DAE::ComponentRef>, metamodelica::Ref<DAE::Type>>>,
)> {
    let mut exp: metamodelica::Ref<DAE::Exp> = exp;
    let mut cont: bool;
    let mut map: metamodelica::Ref<
        UnorderedMap::UnorderedMap<metamodelica::Ref<DAE::ComponentRef>, metamodelica::Ref<DAE::Type>>,
    > = map;
    cont = (match &*exp {
        DAE::Exp::CREF {
            componentRef: cref,
            ty: __exp_ty,
        } if (Types::isRecord(metamodelica::AsArg::as_arg(&__exp_ty))
            && Types::recordHasConstVar(metamodelica::AsArg::as_arg(&__exp_ty))?) =>
        {
            UnorderedMap::add(cref.clone(), __exp_ty.clone(), map.clone())?;
            false
        }
        _ => true,
    });
    Ok((exp, cont, map))
}

fn updateRecordTypesEqn(
    mut eqn: metamodelica::Ref<BackendDAE::Equation>,
    mut map: metamodelica::Ref<
        UnorderedMap::UnorderedMap<metamodelica::Ref<DAE::ComponentRef>, metamodelica::Ref<DAE::Type>>,
    >,
) -> Result<metamodelica::Ref<BackendDAE::Equation>> {
    let mut eqn: metamodelica::Ref<BackendDAE::Equation> = eqn;
    (eqn, _) = BackendEquation::traverseExpsOfEquation(
        eqn,
        (std::sync::Arc::new({
            let __pe_b1: Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>, _) -> Result<_> + 'static> =
                (std::sync::Arc::new(updateRecordTypesExp)
                    as std::sync::Arc<
                        dyn ::std::ops::Fn(
                                metamodelica::Ref<DAE::Exp>,
                                metamodelica::Ref<
                                    UnorderedMap::UnorderedMap<
                                        metamodelica::Ref<DAE::ComponentRef>,
                                        metamodelica::Ref<DAE::Type>,
                                    >,
                                >,
                            ) -> Result<(
                                metamodelica::Ref<DAE::Exp>,
                                bool,
                                metamodelica::Ref<
                                    UnorderedMap::UnorderedMap<
                                        metamodelica::Ref<DAE::ComponentRef>,
                                        metamodelica::Ref<DAE::Type>,
                                    >,
                                >,
                            )> + 'static,
                    >);
            move |__pe_a0, __pe_a2| Expression::traverseExpTopDown(__pe_a0, &*__pe_b1, __pe_a2)
        }) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>, _) -> Result<_> + 'static>),
        map,
    )?;
    Ok(eqn)
}

fn updateRecordTypesExp(
    mut exp: metamodelica::Ref<DAE::Exp>,
    mut map: metamodelica::Ref<
        UnorderedMap::UnorderedMap<metamodelica::Ref<DAE::ComponentRef>, metamodelica::Ref<DAE::Type>>,
    >,
) -> Result<(
    metamodelica::Ref<DAE::Exp>,
    bool,
    metamodelica::Ref<UnorderedMap::UnorderedMap<metamodelica::Ref<DAE::ComponentRef>, metamodelica::Ref<DAE::Type>>>,
)> {
    let mut exp: metamodelica::Ref<DAE::Exp> = exp;
    let mut cont: bool;
    let mut map: metamodelica::Ref<
        UnorderedMap::UnorderedMap<metamodelica::Ref<DAE::ComponentRef>, metamodelica::Ref<DAE::Type>>,
    > = map;
    (exp, cont) = (match &*exp {
        DAE::Exp::CREF { componentRef: cref, .. } if (UnorderedMap::contains(cref.clone(), map.clone())?) => {
            assign_variant_field!(exp => DAE::Exp::CREF; ty = UnorderedMap::getSafe(cref.clone(), map.clone(), metamodelica::sourceInfo!("BackEnd/BackendDAECreate.mo"))?);
            (exp, false)
        }
        _ => (exp, true),
    });
    Ok((exp, cont, map))
}

fn compareArrayBindingExp(
    mut inElement1: &(metamodelica::List<i32>, metamodelica::Ref<DAE::Exp>),
    mut inElement2: &(metamodelica::List<i32>, metamodelica::Ref<DAE::Exp>),
) -> Result<bool> {
    let mut inRes: bool = false;
    let mut indiceLstElem1: metamodelica::List<i32>;
    let mut indiceLstElem2: metamodelica::List<i32>;
    let mut rest_e2: metamodelica::List<i32>;
    let mut e2: i32;
    (indiceLstElem1, _) = inElement1.clone();
    (indiceLstElem2, _) = inElement2.clone();
    if ((indiceLstElem1).len() as i32) != ((indiceLstElem2).len() as i32) {
        Error::addInternalError(
            {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("BackendDAECreate.compareArrayBindingExp"));
                __mm_s.push_str(&*literal!(" failed because lists have different lengths."));
                ArcStr::from(__mm_s)
            },
            metamodelica::sourceInfo!("BackEnd/BackendDAECreate.mo"),
        )?;
        return Err("fail");
    }
    rest_e2 = indiceLstElem2;
    for mut e1 in &*indiceLstElem1 {
        let (__pa0, __pa1) = ::match_deref::match_deref! { match &(rest_e2) {
            Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
            _ => return Err("pattern mismatch"),
        } };
        e2 = metamodelica::Own::own(__pa0);
        rest_e2 = metamodelica::Own::own(__pa1);
        if e1.clone() < e2 {
            inRes = true;
            return Ok(inRes);
        } else if e1.clone() > e2 {
            inRes = false;
            return Ok(inRes);
        }
    }
    inRes = true;
    return Ok(inRes);
    Ok(inRes)
}

fn collapseArrayBindings(
    mut arrayMap: metamodelica::Ref<
        UnorderedMap::UnorderedMap<
            metamodelica::Ref<DAE::ComponentRef>,
            metamodelica::List<(metamodelica::List<i32>, metamodelica::Ref<DAE::Exp>)>,
        >,
    >,
    mut map: metamodelica::Ref<
        UnorderedMap::UnorderedMap<metamodelica::Ref<DAE::ComponentRef>, metamodelica::Ref<DAE::Type>>,
    >,
) -> Result<
    metamodelica::Ref<UnorderedMap::UnorderedMap<metamodelica::Ref<DAE::ComponentRef>, metamodelica::Ref<DAE::Type>>>,
> {
    let mut map: metamodelica::Ref<
        UnorderedMap::UnorderedMap<metamodelica::Ref<DAE::ComponentRef>, metamodelica::Ref<DAE::Type>>,
    > = map;
    let mut cref: metamodelica::Ref<DAE::ComponentRef>;
    let mut rec_cref: metamodelica::Ref<DAE::ComponentRef>;
    let mut arrayBindingExpList: ArrayBindingList;
    let mut subscriptLst: metamodelica::List<i32> = metamodelica::nil();
    let mut binding: metamodelica::Ref<DAE::Exp>;
    let mut scalarBinding: metamodelica::Ref<DAE::Exp>;
    let mut expLst: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
    let mut ty: metamodelica::Ref<DAE::Type>;
    let mut dims: metamodelica::List<metamodelica::Ref<DAE::Dimension>>;
    let mut firstDim: i32;
    for mut pair in &*UnorderedMap::toList(arrayMap) {
        (cref, arrayBindingExpList) = pair.clone();
        arrayBindingExpList = List::sort(
            arrayBindingExpList,
            (std::sync::Arc::new(
                move |__a0: (metamodelica::List<i32>, metamodelica::Ref<DAE::Exp>),
                      __a1: (metamodelica::List<i32>, metamodelica::Ref<DAE::Exp>)| {
                    compareArrayBindingExp(&__a0, &__a1)
                },
            )
                as std::sync::Arc<
                    dyn ::std::ops::Fn(
                            (metamodelica::List<i32>, metamodelica::Ref<DAE::Exp>),
                            (metamodelica::List<i32>, metamodelica::Ref<DAE::Exp>),
                        ) -> Result<bool>
                        + 'static,
                >),
        )?;
        expLst = metamodelica::nil();
        for mut scalBind in &*arrayBindingExpList {
            (subscriptLst, scalarBinding) = scalBind.clone();
            expLst = metamodelica::cons(scalarBinding, expLst);
        }
        binding = (match ((subscriptLst).len() as i32) {
            1 => metamodelica::Ref::new(DAE::Exp::ARRAY {
                ty: ComponentReference::crefTypeFull(&cref)?,
                scalar: true,
                array: expLst,
            }),
            2 => {
                let mut matLst: metamodelica::List<metamodelica::List<metamodelica::Ref<DAE::Exp>>>;
                dims = TypesDump::getDimensions(&(ComponentReference::crefLastType(&cref)?));
                firstDim = (match &*((dims).head().cloned()?) {
                    DAE::Dimension::DIM_INTEGER {
                        integer: __esc_firstDim,
                    } => {
                        firstDim = (*__esc_firstDim).clone();
                        firstDim.clone()
                    }
                    _ => return Err("match: no arm matched"),
                });
                if let Ok(__iflet0) = List::splitEqualParts(expLst.clone(), firstDim) {
                    matLst = __iflet0;
                } else {
                    Error::addInternalError(
                        {
                            let mut __mm_s = String::new();
                            __mm_s.push_str(&*literal!("BackendDAECreate.collapseArrayBindings"));
                            __mm_s.push_str(&*literal!(" failed to reshape matrix."));
                            ArcStr::from(__mm_s)
                        },
                        metamodelica::sourceInfo!("BackEnd/BackendDAECreate.mo"),
                    )?;
                    return Err("fail");
                }
                metamodelica::Ref::new(DAE::Exp::MATRIX {
                    ty: ComponentReference::crefTypeFull(&cref)?,
                    integer: firstDim,
                    matrix: matLst,
                })
            }
            _ => {
                Error::addInternalError(
                    {
                        let mut __mm_s = String::new();
                        __mm_s.push_str(&*literal!("BackendDAECreate.collapseArrayBindings"));
                        __mm_s.push_str(&*literal!(
                            "failed. Array of dimension greater 2 not yet supported. Open a ticket about it."
                        ));
                        ArcStr::from(__mm_s)
                    },
                    metamodelica::sourceInfo!("BackEnd/BackendDAECreate.mo"),
                )?;
                return Err("fail");
            }
        });
        let __pa0 = ::match_deref::match_deref! { match &(ComponentReference::crefGetFirstRec(&cref)?) {
            (__pa0, true) => __pa0.clone(),
            _ => return Err("pattern mismatch"),
        } };
        rec_cref = metamodelica::Own::own(__pa0);
        ty = (::match_deref::match_deref! { match &(UnorderedMap::getSafe(rec_cref.clone(), map.clone(), metamodelica::sourceInfo!("BackEnd/BackendDAECreate.mo"))?) {
            __esc_ty @ Deref @ DAE::Type::T_COMPLEX { .. } => {
                ty = (*__esc_ty).clone();
                assign_variant_field!(ty => DAE::Type::T_COMPLEX; varLst = ({
            let mut __acc: metamodelica::List<metamodelica::Ref<DAE::Var>> = metamodelica::nil();
            for mut v in (var_field!((*ty).varLst, DAE::Type::T_COMPLEX).clone()).into_iter().cloned() {
                let __x = updateConstantRecordElementBinding(v.clone(), binding.clone(), &(ComponentReferenceBasics::crefLastIdent(&cref)?))?;
                __acc = cons(__x, __acc);
            }
            __acc.reverse()
        }));
                ty.clone()
            },
            _ => {
                Error::addMessage(Error::INTERNAL_ERROR.clone(), list![{ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("BackendDAECreate.collapseArrayBindings")); __mm_s.push_str(&*literal!(" failed because the type is not T_COMPLEX.")); ArcStr::from(__mm_s) }])?;
                return Err("fail")
            },
            _ => unreachable!("match_deref! exhaustiveness placeholder"),
        } });
        UnorderedMap::add(rec_cref, ty.clone(), map.clone())?;
    }
    Ok(map)
}

fn updateRecordTypesVarLst(
    mut map: metamodelica::Ref<
        UnorderedMap::UnorderedMap<metamodelica::Ref<DAE::ComponentRef>, metamodelica::Ref<DAE::Type>>,
    >,
    mut varLst: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
) -> Result<metamodelica::List<metamodelica::Ref<BackendDAE::Var>>> {
    let mut varLst: metamodelica::List<metamodelica::Ref<BackendDAE::Var>> = varLst;
    varLst = ({
        let mut __acc: metamodelica::List<metamodelica::Ref<BackendDAE::Var>> = metamodelica::nil();
        for mut var in (varLst).into_iter().cloned() {
            let __x = (::match_deref::match_deref! { match &(var.bindExp.clone()) {
                Some(exp) => {
                    let mut exp = (*exp).clone();
                    (exp, _) = Expression::traverseExpTopDown(exp.clone(), &updateRecordTypesExp, map.clone())?;
                    assign_field!(var.bindExp = Some(exp.clone()));
                    var.clone()
                },
                _ => {
                    var.clone()
                },
                _ => unreachable!("match_deref! exhaustiveness placeholder"),
            } });
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    Ok(varLst)
}

fn getExternalObjectAlias(
    mut inInitEqs: &metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
    mut inEqs: &metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
    mut inRemEqs: &metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
    mut globalVarsIn: BackendDAE::Variables,
    mut extVars: BackendDAE::Variables,
) -> Result<(
    metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
    metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
    metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
    BackendDAE::Variables,
    BackendDAE::Variables,
    BackendDAE::Variables,
)> {
    let mut oInitEqs: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
    let mut oEqs: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
    let mut oRemEqs: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
    let mut extAliasVars: BackendDAE::Variables;
    let mut globalVarsOut: BackendDAE::Variables;
    let mut extVarsOut: BackendDAE::Variables;
    let mut extCrefs: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
    let mut aliasEqs: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
    let mut aliasVarLst: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
    let mut repl: BackendVarTransform::VariableReplacements;
    extCrefs = BackendVariable::getAllCrefFromVariables(extVars.clone())?;
    (oEqs, aliasEqs) = List::fold1(
        inEqs,
        &move |__a0: metamodelica::Ref<BackendDAE::Equation>,
               __a1: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
               __a2: (
            metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
            metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
        )|
              -> metamodelica::Result<_> {
            ::std::result::Result::Ok(getExternalObjectAlias2(__a0, &__a1, &__a2))
        },
        extCrefs.clone(),
        (metamodelica::nil(), metamodelica::nil()),
    )?;
    (oInitEqs, aliasEqs) = List::fold1(
        inInitEqs,
        &move |__a0: metamodelica::Ref<BackendDAE::Equation>,
               __a1: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
               __a2: (
            metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
            metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
        )|
              -> metamodelica::Result<_> {
            ::std::result::Result::Ok(getExternalObjectAlias2(__a0, &__a1, &__a2))
        },
        extCrefs.clone(),
        (metamodelica::nil(), aliasEqs),
    )?;
    (oRemEqs, aliasEqs) = List::fold1(
        inRemEqs,
        &move |__a0: metamodelica::Ref<BackendDAE::Equation>,
               __a1: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
               __a2: (
            metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
            metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
        )|
              -> metamodelica::Result<_> {
            ::std::result::Result::Ok(getExternalObjectAlias2(__a0, &__a1, &__a2))
        },
        extCrefs,
        (metamodelica::nil(), aliasEqs),
    )?;
    if !((aliasEqs).is_empty()) {
        Error::addCompilerWarning({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!(
                "Alias equations of external objects are not Modelica compliant as in:\n    "
            ));
            __mm_s.push_str(&*stringDelimitList(
                List::map(aliasEqs.clone(), &move |__a0: metamodelica::Ref<
                    BackendDAE::Equation,
                >| BackendDump::equationString(&__a0))?,
                literal!("\n    "),
            ));
            __mm_s.push_str(&*literal!("\n"));
            ArcStr::from(__mm_s)
        })?;
    }
    repl = BackendVarTransform::emptyReplacements();
    (aliasVarLst, repl) = List::fold1(
        &aliasEqs,
        &move |__a0: metamodelica::Ref<BackendDAE::Equation>,
               __a1: BackendDAE::Variables,
               __a2: (
            metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
            BackendVarTransform::VariableReplacements,
        )| getExternalObjectAlias3(__a0, &__a1, __a2),
        extVars.clone(),
        (metamodelica::nil(), repl),
    )?;
    extAliasVars = BackendVariable::listVar1(&aliasVarLst)?;
    extVarsOut = BackendVariable::deleteVars(extAliasVars.clone(), extVars);
    extVarsOut = removeExtAliasBinding(&extVarsOut, &repl)?;
    (oEqs, _) = BackendVarTransform::replaceEquations(oEqs, &repl, None)?;
    (oInitEqs, _) = BackendVarTransform::replaceEquations(oInitEqs, &repl, None)?;
    (oRemEqs, _) = BackendVarTransform::replaceEquations(oRemEqs, &repl, None)?;
    (globalVarsOut, _) = BackendVariable::traverseBackendDAEVarsWithUpdate(
        globalVarsIn,
        (std::sync::Arc::new(BackendVarTransform::replaceVarTraverser)
            as std::sync::Arc<
                dyn ::std::ops::Fn(
                        metamodelica::Ref<BackendDAE::Var>,
                        BackendVarTransform::VariableReplacements,
                    ) -> Result<(
                        metamodelica::Ref<BackendDAE::Var>,
                        BackendVarTransform::VariableReplacements,
                    )> + 'static,
            >),
        repl,
    )?;
    oEqs = oEqs.reverse();
    oInitEqs = oInitEqs.reverse();
    oRemEqs = oRemEqs.reverse();
    Ok((oInitEqs, oEqs, oRemEqs, extAliasVars, globalVarsOut, extVarsOut))
}

fn removeExtAliasBinding(
    mut extVarsIn: &BackendDAE::Variables,
    mut repl: &BackendVarTransform::VariableReplacements,
) -> Result<BackendDAE::Variables> {
    let mut extVarsOut: BackendDAE::Variables;
    let mut varLst: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
    let mut extVarLst: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
    let mut cref: metamodelica::Ref<DAE::ComponentRef>;
    varLst = BackendVariable::varList(extVarsIn)?;
    extVarLst = metamodelica::nil();
    for mut var in &*varLst {
        let mut var = var.clone();
        var = (::match_deref::match_deref! { match &(var.clone()) {
            Deref @ BackendDAE::Var { bindExp: Some(Deref @ DAE::Exp::CREF { componentRef: __esc_cref, .. }), .. } => {
                cref = (*__esc_cref).clone();
                if BackendVarTransform::hasReplacement(repl, cref.clone())? {
                    assign_field!(var.bindExp = None);
                }
                var
            },
            _ => var,
            _ => unreachable!("match_deref! exhaustiveness placeholder"),
        } });
        extVarLst = metamodelica::cons(var, extVarLst);
    }
    extVarsOut = BackendVariable::listVar(extVarLst)?;
    Ok(extVarsOut)
}

fn getExternalObjectAlias3(
    mut eqIn: metamodelica::Ref<BackendDAE::Equation>,
    mut extVars: &BackendDAE::Variables,
    mut tplIn: (
        metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
        BackendVarTransform::VariableReplacements,
    ),
) -> Result<(
    metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
    BackendVarTransform::VariableReplacements,
)> {
    let mut tplOut: (
        metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
        BackendVarTransform::VariableReplacements,
    );
    let mut eq: metamodelica::Ref<BackendDAE::Equation>;
    let mut crefs_lhs: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
    let mut crefs_rhs: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
    let mut extAliasVars: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
    let mut repl: BackendVarTransform::VariableReplacements;
    (extAliasVars, repl) = tplIn.clone();
    let __pa0 = ::match_deref::match_deref! { match &(BackendVarTransform::replaceEquations(list![eqIn.clone()], &repl, None)?) {
        (Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: Deref @ metamodelica::ListNode::Nil }, _) => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    eq = metamodelica::Own::own(__pa0);
    match '__try2: {
        (crefs_lhs, crefs_rhs) = unwrap_break_err!(BackendEquation::equationCrefsSolved(&eq), '__try2);
        (extAliasVars, repl) = (::match_deref::match_deref! { match &((crefs_lhs.clone(), crefs_rhs.clone())) {
            (Deref @ metamodelica::ListNode::Cons { head: lhs, tail: Deref @ metamodelica::ListNode::Nil }, Deref @ metamodelica::ListNode::Cons { head: rhs, tail: Deref @ metamodelica::ListNode::Nil }) => {
                crefs_lhs = unwrap_break_err!(ComponentReference::expandCref(metamodelica::AsArg::as_arg(&lhs), true), '__try2);
                crefs_rhs = unwrap_break_err!(ComponentReference::expandCref(metamodelica::AsArg::as_arg(&rhs), true), '__try2);
                (extAliasVars, repl) = unwrap_break_err!(addExternalObjectReplacementRules(crefs_lhs.clone(), crefs_rhs.clone(), extVars, extAliasVars.clone(), repl.clone()), '__try2);
                (extAliasVars.clone(), repl.clone())
            },
            (Deref @ metamodelica::ListNode::Cons { head: lhs, tail: Deref @ metamodelica::ListNode::Nil }, _) => {
                crefs_lhs = unwrap_break_err!(ComponentReference::expandCref(metamodelica::AsArg::as_arg(&lhs), true), '__try2);
                (extAliasVars, repl) = unwrap_break_err!(addExternalObjectReplacementRules(crefs_lhs.clone(), crefs_rhs.clone(), extVars, extAliasVars.clone(), repl.clone()), '__try2);
                (extAliasVars.clone(), repl.clone())
            },
            (_, Deref @ metamodelica::ListNode::Cons { head: rhs, tail: Deref @ metamodelica::ListNode::Nil }) => {
                crefs_rhs = unwrap_break_err!(ComponentReference::expandCref(metamodelica::AsArg::as_arg(&rhs), true), '__try2);
                (extAliasVars, repl) = unwrap_break_err!(addExternalObjectReplacementRules(crefs_lhs.clone(), crefs_rhs.clone(), extVars, extAliasVars.clone(), repl.clone()), '__try2);
                (extAliasVars.clone(), repl.clone())
            },
            _ => {
                (extAliasVars, repl) = unwrap_break_err!(addExternalObjectReplacementRules(crefs_lhs.clone(), crefs_rhs.clone(), extVars, extAliasVars.clone(), repl.clone()), '__try2);
                (extAliasVars.clone(), repl.clone())
            },
            _ => unreachable!("match_deref! exhaustiveness placeholder"),
        } });
        tplOut = (extAliasVars.clone(), repl.clone());
        Ok::<_, &'static str>((tplOut.clone(),))
    } {
        Ok((__try2_o0,)) => {
            tplOut = __try2_o0;
        }
        Err(_) => {
            Error::addInternalError(
                {
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!("BackendDAECreate.getExternalObjectAlias3"));
                    __mm_s.push_str(&*literal!(" failed for "));
                    __mm_s.push_str(&*BackendDump::equationString(&eqIn)?);
                    ArcStr::from(__mm_s)
                },
                metamodelica::sourceInfo!("BackEnd/BackendDAECreate.mo"),
            )?;
            tplOut = tplIn.clone();
        }
    }
    Ok(tplOut)
}

fn addExternalObjectReplacementRules(
    mut crefs_lhs: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
    mut crefs_rhs: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
    mut extVars: &BackendDAE::Variables,
    mut extAliasVars: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
    mut repl: BackendVarTransform::VariableReplacements,
) -> Result<(
    metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
    BackendVarTransform::VariableReplacements,
)> {
    let mut extAliasVars: metamodelica::List<metamodelica::Ref<BackendDAE::Var>> = extAliasVars;
    let mut repl: BackendVarTransform::VariableReplacements = repl;
    let mut lhs: metamodelica::Ref<DAE::ComponentRef>;
    let mut rhs: metamodelica::Ref<DAE::ComponentRef>;
    let mut v1: metamodelica::Ref<BackendDAE::Var>;
    let mut v2: metamodelica::Ref<BackendDAE::Var>;
    let mut simVar: metamodelica::Ref<BackendDAE::Var>;
    let mut aliasVar: metamodelica::Ref<BackendDAE::Var>;
    if ((crefs_lhs).len() as i32) == ((crefs_rhs).len() as i32) {
        for mut tpl in &*List::zip(crefs_lhs, crefs_rhs) {
            (lhs, rhs) = tpl.clone();
            let __pa0 = ::match_deref::match_deref! { match &(BackendVariable::getVar(lhs, extVars)?) {
                (Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: Deref @ metamodelica::ListNode::Nil }, _) => __pa0.clone(),
                _ => return Err("pattern mismatch"),
            } };
            v1 = metamodelica::Own::own(__pa0);
            let __pa2 = ::match_deref::match_deref! { match &(BackendVariable::getVar(rhs, extVars)?) {
                (Deref @ metamodelica::ListNode::Cons { head: __pa2, tail: Deref @ metamodelica::ListNode::Nil }, _) => __pa2.clone(),
                _ => return Err("pattern mismatch"),
            } };
            v2 = metamodelica::Own::own(__pa2);
            (simVar, aliasVar) = chooseExternalAlias(v1, v2)?;
            extAliasVars = metamodelica::cons(aliasVar.clone(), extAliasVars);
            repl = BackendVarTransform::addReplacement(
                repl,
                BackendVariable::varCref(&aliasVar),
                Expression::crefExp(BackendVariable::varCref(&simVar))?,
                None,
            )?;
        }
    } else {
        return Err("fail");
    }
    Ok((extAliasVars, repl))
}

fn chooseExternalAlias(
    mut var1: metamodelica::Ref<BackendDAE::Var>,
    mut var2: metamodelica::Ref<BackendDAE::Var>,
) -> Result<(metamodelica::Ref<BackendDAE::Var>, metamodelica::Ref<BackendDAE::Var>)> {
    let mut simVar: metamodelica::Ref<BackendDAE::Var>;
    let mut aliasVar: metamodelica::Ref<BackendDAE::Var>;
    if BackendVariable::varHasBindExp(&var1) && !(BackendVariable::varHasBindExp(&var2)) {
        simVar = var1;
        aliasVar = BackendVariable::setBindExp(var2, Some(Expression::crefExp(BackendVariable::varCref(&simVar))?));
    } else if BackendVariable::varHasBindExp(&var2) && !(BackendVariable::varHasBindExp(&var1)) {
        simVar = var2;
        aliasVar = BackendVariable::setBindExp(var1, Some(Expression::crefExp(BackendVariable::varCref(&simVar))?));
    } else if BackendVariable::varHasBindExp(&var2) && BackendVariable::varHasBindExp(&var1) {
        if Expression::isCall(&(BackendVariable::varBindExp(&var1)?)) {
            simVar = var1;
            aliasVar = BackendVariable::setBindExp(var2, Some(Expression::crefExp(BackendVariable::varCref(&simVar))?));
        } else {
            simVar = var2;
            aliasVar = BackendVariable::setBindExp(var1, Some(Expression::crefExp(BackendVariable::varCref(&simVar))?));
        }
    } else {
        simVar = var1;
        aliasVar = BackendVariable::setBindExp(var2, Some(Expression::crefExp(BackendVariable::varCref(&simVar))?));
    }
    Ok((simVar, aliasVar))
}

fn getExternalObjectAlias2(
    mut eqIn: metamodelica::Ref<BackendDAE::Equation>,
    mut extCrefs: &metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
    mut eqTplIn: &(
        metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
        metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
    ),
) -> (
    metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
    metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
) {
    let mut eqTplOut: (
        metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
        metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
    );
    eqTplOut = 'mc: {
        let __mc_input = (&*eqIn, eqTplIn);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ BackendDAE::Equation::COMPLEX_EQUATION { left: Deref @ DAE::Exp::CREF { componentRef: cr1, .. }, right: Deref @ DAE::Exp::CREF { componentRef: cr2, .. }, .. }, (noAliasEqs, aliasEqs)) => {
                    let true = (List::exist1(extCrefs, &move |__a0: metamodelica::Ref<DAE::ComponentRef>, __a1: metamodelica::Ref<DAE::ComponentRef>| ComponentReferenceBasics::crefEqual(&__a0, &__a1), cr1.clone())? && List::exist1(extCrefs, &move |__a0: metamodelica::Ref<DAE::ComponentRef>, __a1: metamodelica::Ref<DAE::ComponentRef>| ComponentReferenceBasics::crefEqual(&__a0, &__a1), cr2.clone())?) else { return Err("pattern mismatch") };
                    Ok((noAliasEqs.clone(), metamodelica::cons(eqIn.clone(), aliasEqs.clone())))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ BackendDAE::Equation::EQUATION { exp: Deref @ DAE::Exp::CREF { componentRef: cr1, ty: Deref @ DAE::Type::T_COMPLEX { complexClassType: ClassInf::State::EXTERNAL_OBJ { .. }, .. } }, scalar: Deref @ DAE::Exp::CREF { componentRef: cr2, ty: Deref @ DAE::Type::T_COMPLEX { complexClassType: ClassInf::State::EXTERNAL_OBJ { .. }, .. } }, .. }, (noAliasEqs, aliasEqs)) => {
                    let true = (List::exist1(extCrefs, &move |__a0: metamodelica::Ref<DAE::ComponentRef>, __a1: metamodelica::Ref<DAE::ComponentRef>| ComponentReferenceBasics::crefEqual(&__a0, &__a1), cr1.clone())? && List::exist1(extCrefs, &move |__a0: metamodelica::Ref<DAE::ComponentRef>, __a1: metamodelica::Ref<DAE::ComponentRef>| ComponentReferenceBasics::crefEqual(&__a0, &__a1), cr2.clone())?) else { return Err("pattern mismatch") };
                    Ok((noAliasEqs.clone(), metamodelica::cons(eqIn.clone(), aliasEqs.clone())))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ BackendDAE::Equation::ARRAY_EQUATION { dimSize: _, left: Deref @ DAE::Exp::CREF { ty: Deref @ DAE::Type::T_ARRAY { ty: Deref @ DAE::Type::T_COMPLEX { complexClassType: ClassInf::State::EXTERNAL_OBJ { .. }, .. }, .. }, .. }, right: Deref @ DAE::Exp::ARRAY { .. }, .. }, (noAliasEqs, aliasEqs)) => {
                    Ok((noAliasEqs.clone(), metamodelica::cons(eqIn.clone(), aliasEqs.clone())))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ BackendDAE::Equation::ARRAY_EQUATION { dimSize: _, left: Deref @ DAE::Exp::CREF { ty: Deref @ DAE::Type::T_ARRAY { ty: Deref @ DAE::Type::T_COMPLEX { complexClassType: ClassInf::State::EXTERNAL_OBJ { .. }, .. }, .. }, .. }, right: Deref @ DAE::Exp::CREF { ty: Deref @ DAE::Type::T_ARRAY { ty: Deref @ DAE::Type::T_COMPLEX { complexClassType: ClassInf::State::EXTERNAL_OBJ { .. }, .. }, .. }, .. }, .. }, _) => {
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ BackendDAE::Equation::ARRAY_EQUATION { dimSize: _, left: Deref @ DAE::Exp::ARRAY { .. }, right: Deref @ DAE::Exp::CREF { ty: Deref @ DAE::Type::T_ARRAY { ty: Deref @ DAE::Type::T_COMPLEX { complexClassType: ClassInf::State::EXTERNAL_OBJ { .. }, .. }, .. }, .. }, .. }, _) => {
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut noAliasEqs: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
                    let mut aliasEqs: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
                    (noAliasEqs, aliasEqs) = eqTplIn.clone();
                    Ok((metamodelica::cons(eqIn.clone(), noAliasEqs.clone()), aliasEqs.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    eqTplOut
}

fn lower2(
    mut inElements: &metamodelica::List<metamodelica::Ref<DAE::Element>>,
    mut inFunctions: &metamodelica::Ref<AvlTreePathFunction::Tree>,
    mut inInlineHT: (
        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::Exp>, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<Option<(metamodelica::Ref<DAE::Exp>, metamodelica::Ref<DAE::Exp>)>>,
        ),
        i32,
        (
            Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<i32> + 'static>,
            Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>, metamodelica::Ref<DAE::Exp>) -> Result<bool> + 'static>,
            Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<ArcStr> + 'static>,
            Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<ArcStr> + 'static>,
        ),
    ),
    mut inVars: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
    mut inGlobalKnownVars: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
    mut inExVars: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
    mut inEqns: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
    mut inREqns: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
    mut inIEqns: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
    mut inConstraints: metamodelica::List<metamodelica::Ref<DAE::Constraint>>,
    mut inClassAttributes: metamodelica::List<metamodelica::Ref<DAE::ClassAttributes>>,
    mut inExtObjClasses: metamodelica::List<BackendDAE::ExternalObjectClass>,
    mut inAliasEqns: metamodelica::List<metamodelica::Ref<DAE::Element>>,
) -> Result<(
    metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
    metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
    metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
    metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
    metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
    metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
    metamodelica::List<metamodelica::Ref<DAE::Constraint>>,
    metamodelica::List<metamodelica::Ref<DAE::ClassAttributes>>,
    metamodelica::List<BackendDAE::ExternalObjectClass>,
    metamodelica::List<metamodelica::Ref<DAE::Element>>,
    (
        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::Exp>, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<Option<(metamodelica::Ref<DAE::Exp>, metamodelica::Ref<DAE::Exp>)>>,
        ),
        i32,
        (
            Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<i32> + 'static>,
            Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>, metamodelica::Ref<DAE::Exp>) -> Result<bool> + 'static>,
            Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<ArcStr> + 'static>,
            Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<ArcStr> + 'static>,
        ),
    ),
)> {
    let mut outVars: metamodelica::List<metamodelica::Ref<BackendDAE::Var>> = inVars;
    let mut outGlobalKnownVars: metamodelica::List<metamodelica::Ref<BackendDAE::Var>> = inGlobalKnownVars;
    let mut outExVars: metamodelica::List<metamodelica::Ref<BackendDAE::Var>> = inExVars;
    let mut outEqns: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>> = inEqns;
    let mut outREqns: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>> = inREqns;
    let mut outIEqns: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>> = inIEqns;
    let mut outConstraints: metamodelica::List<metamodelica::Ref<DAE::Constraint>> = inConstraints;
    let mut outClassAttributes: metamodelica::List<metamodelica::Ref<DAE::ClassAttributes>> = inClassAttributes;
    let mut outExtObjClasses: metamodelica::List<BackendDAE::ExternalObjectClass> = inExtObjClasses;
    let mut outAliasEqns: metamodelica::List<metamodelica::Ref<DAE::Element>> = inAliasEqns;
    let mut outInlineHT: (
        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::Exp>, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<Option<(metamodelica::Ref<DAE::Exp>, metamodelica::Ref<DAE::Exp>)>>,
        ),
        i32,
        (
            HashTableExpToExp::FuncHashCref,
            HashTableExpToExp::FuncCrefEqual,
            HashTableExpToExp::FuncCrefStr,
            HashTableExpToExp::FuncExpStr,
        ),
    ) = inInlineHT;
    let mut path: metamodelica::Ref<Absyn::Path>;
    let mut src: metamodelica::Ref<DAE::ElementSource>;
    let mut dae_elts: metamodelica::List<metamodelica::Ref<DAE::Element>>;
    let mut class_attrs: metamodelica::Ref<DAE::ClassAttributes>;
    let mut constraints: metamodelica::Ref<DAE::Constraint>;
    let mut el: metamodelica::Ref<DAE::Element> =
        <metamodelica::Ref<DAE::Element> as ::std::default::Default>::default();
    let mut eq_attrs: BackendDAE::EquationAttributes;
    let mut whenClkCnt: i32 = 1;
    let mut e: metamodelica::Ref<DAE::Exp>;
    let mut eqns: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
    let mut reqns: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
    for mut el in &**inElements {
        let mut el = el.clone();
        let () = (match &*el.clone() {
            DAE::Element::EXTOBJECTCLASS {
                path: __esc_path,
                source: __esc_src,
            } => {
                path = (*__esc_path).clone();
                src = (*__esc_src).clone();
                outExtObjClasses = metamodelica::cons(
                    BackendDAE::ExternalObjectClass {
                        path: path.clone(),
                        source: src.clone(),
                    },
                    outExtObjClasses,
                );
                ()
            }
            DAE::Element::VAR { .. } => {
                (outVars, outGlobalKnownVars, outExVars, outEqns, outREqns, outInlineHT) = lowerVar(
                    el,
                    inFunctions,
                    outVars,
                    outGlobalKnownVars,
                    outExVars,
                    outEqns,
                    outREqns,
                    outInlineHT,
                )?;
                ()
            }
            DAE::Element::EQUATION { .. } => {
                (outEqns, outREqns, outIEqns) = lowerEqn(el, inFunctions.clone(), outEqns, outREqns, outIEqns, false)?;
                ()
            }
            DAE::Element::INITIALEQUATION { .. } => {
                (outEqns, outREqns, outIEqns) = lowerEqn(el, inFunctions.clone(), outEqns, outREqns, outIEqns, true)?;
                ()
            }
            DAE::Element::EQUEQUATION { .. } => {
                (outEqns, outREqns, outIEqns) = lowerEqn(el, inFunctions.clone(), outEqns, outREqns, outIEqns, false)?;
                ()
            }
            DAE::Element::DEFINE { .. } => {
                (outEqns, outREqns, outIEqns) = lowerEqn(el, inFunctions.clone(), outEqns, outREqns, outIEqns, false)?;
                ()
            }
            DAE::Element::INITIALDEFINE { .. } => {
                (outEqns, outREqns, outIEqns) = lowerEqn(el, inFunctions.clone(), outEqns, outREqns, outIEqns, true)?;
                ()
            }
            DAE::Element::COMPLEX_EQUATION { .. } => {
                (outEqns, outREqns, outIEqns) = lowerEqn(el, inFunctions.clone(), outEqns, outREqns, outIEqns, false)?;
                ()
            }
            DAE::Element::INITIAL_COMPLEX_EQUATION { .. } => {
                (outEqns, outREqns, outIEqns) = lowerEqn(el, inFunctions.clone(), outEqns, outREqns, outIEqns, true)?;
                ()
            }
            DAE::Element::ARRAY_EQUATION { .. } => {
                (outEqns, outREqns, outIEqns) = lowerEqn(el, inFunctions.clone(), outEqns, outREqns, outIEqns, false)?;
                ()
            }
            DAE::Element::FOR_EQUATION { .. } => {
                (outEqns, outREqns, outIEqns) = lowerEqn(el, inFunctions.clone(), outEqns, outREqns, outIEqns, false)?;
                ()
            }
            DAE::Element::INITIAL_FOR_EQUATION { .. } => {
                (outEqns, outREqns, outIEqns) = lowerEqn(el, inFunctions.clone(), outEqns, outREqns, outIEqns, true)?;
                ()
            }
            DAE::Element::INITIAL_ARRAY_EQUATION { .. } => {
                (outEqns, outREqns, outIEqns) = lowerEqn(el, inFunctions.clone(), outEqns, outREqns, outIEqns, true)?;
                ()
            }
            DAE::Element::WHEN_EQUATION {
                condition: __esc_e,
                equations: __esc_dae_elts,
                ..
            } => {
                e = (*__esc_e).clone();
                dae_elts = (*__esc_dae_elts).clone();
                if Config::synchronousFeaturesAllowed()?
                    && Types::isClockOrSubTypeClock(Expression::r#typeof(e.clone())?)
                {
                    (outEqns, outVars, eq_attrs) = createWhenClock(whenClkCnt, e.clone(), outEqns, outVars);
                    whenClkCnt = whenClkCnt + 1;
                    (
                        outVars,
                        outGlobalKnownVars,
                        outExVars,
                        eqns,
                        reqns,
                        outIEqns,
                        outConstraints,
                        outClassAttributes,
                        outExtObjClasses,
                        outAliasEqns,
                        outInlineHT,
                    ) = lower2(
                        metamodelica::AsArg::as_arg(&dae_elts),
                        inFunctions,
                        outInlineHT,
                        outVars,
                        outGlobalKnownVars,
                        outExVars,
                        metamodelica::nil(),
                        metamodelica::nil(),
                        outIEqns,
                        outConstraints,
                        outClassAttributes,
                        outExtObjClasses,
                        outAliasEqns,
                    )?;
                    outEqns = listAppend(
                        List::map1(
                            eqns,
                            &move |__a0: metamodelica::Ref<BackendDAE::Equation>,
                                   __a1: BackendDAE::EquationAttributes| {
                                BackendEquation::setEquationAttributes(&__a0, __a1)
                            },
                            eq_attrs,
                        )?,
                        outEqns,
                    );
                    outREqns = listAppend(
                        List::map1(
                            reqns,
                            &move |__a0: metamodelica::Ref<BackendDAE::Equation>,
                                   __a1: BackendDAE::EquationAttributes| {
                                BackendEquation::setEquationAttributes(&__a0, __a1)
                            },
                            eq_attrs,
                        )?,
                        outREqns,
                    );
                } else {
                    (eqns, reqns, outVars) = lowerWhenEqn(
                        &el,
                        inFunctions,
                        &(metamodelica::nil()),
                        &(metamodelica::nil()),
                        outVars,
                    )?;
                    outEqns = listAppend(outEqns, eqns);
                    outREqns = listAppend(outREqns, reqns);
                }
                ()
            }
            DAE::Element::IF_EQUATION { .. } => {
                (outEqns, outREqns, outIEqns) = lowerEqn(el, inFunctions.clone(), outEqns, outREqns, outIEqns, false)?;
                ()
            }
            DAE::Element::INITIAL_IF_EQUATION { .. } => {
                (outEqns, outREqns, outIEqns) = lowerEqn(el, inFunctions.clone(), outEqns, outREqns, outIEqns, true)?;
                ()
            }
            DAE::Element::ALGORITHM { .. } => {
                (outEqns, outREqns, outIEqns) = lowerAlgorithm(
                    el,
                    inFunctions,
                    outEqns,
                    outREqns,
                    outIEqns,
                    openmodelica_frontend_types::DAE::Expand::EXPAND,
                    false,
                )?;
                ()
            }
            DAE::Element::INITIALALGORITHM { .. } => {
                (outEqns, outREqns, outIEqns) = lowerAlgorithm(
                    el,
                    inFunctions,
                    outEqns,
                    outREqns,
                    outIEqns,
                    openmodelica_frontend_types::DAE::Expand::EXPAND,
                    true,
                )?;
                ()
            }
            DAE::Element::COMP {
                dAElist: __esc_dae_elts,
                ..
            } => {
                dae_elts = (*__esc_dae_elts).clone();
                (
                    outVars,
                    outGlobalKnownVars,
                    outExVars,
                    outEqns,
                    outREqns,
                    outIEqns,
                    outConstraints,
                    outClassAttributes,
                    outExtObjClasses,
                    outAliasEqns,
                    outInlineHT,
                ) = lower2(
                    &(dae_elts.clone().reverse()),
                    inFunctions,
                    outInlineHT,
                    outVars,
                    outGlobalKnownVars,
                    outExVars,
                    outEqns,
                    outREqns,
                    outIEqns,
                    outConstraints,
                    outClassAttributes,
                    outExtObjClasses,
                    outAliasEqns,
                )?;
                ()
            }
            DAE::Element::ASSERT { .. } => {
                (outEqns, outREqns, outIEqns) = lowerAlgorithm(
                    el,
                    inFunctions,
                    outEqns,
                    outREqns,
                    outIEqns,
                    openmodelica_frontend_types::DAE::Expand::NOT_EXPAND,
                    false,
                )?;
                ()
            }
            DAE::Element::INITIAL_ASSERT { .. } => {
                (outEqns, outREqns, outIEqns) = lowerAlgorithm(
                    el,
                    inFunctions,
                    outEqns,
                    outREqns,
                    outIEqns,
                    openmodelica_frontend_types::DAE::Expand::NOT_EXPAND,
                    true,
                )?;
                ()
            }
            DAE::Element::TERMINATE { .. } => {
                (outEqns, outREqns, outIEqns) = lowerAlgorithm(
                    el,
                    inFunctions,
                    outEqns,
                    outREqns,
                    outIEqns,
                    openmodelica_frontend_types::DAE::Expand::NOT_EXPAND,
                    false,
                )?;
                ()
            }
            DAE::Element::INITIAL_TERMINATE { .. } => {
                (outEqns, outREqns, outIEqns) = lowerAlgorithm(
                    el,
                    inFunctions,
                    outEqns,
                    outREqns,
                    outIEqns,
                    openmodelica_frontend_types::DAE::Expand::NOT_EXPAND,
                    true,
                )?;
                ()
            }
            DAE::Element::NORETCALL { .. } => {
                (outEqns, outREqns, outIEqns) = lowerAlgorithm(
                    el,
                    inFunctions,
                    outEqns,
                    outREqns,
                    outIEqns,
                    openmodelica_frontend_types::DAE::Expand::NOT_EXPAND,
                    false,
                )?;
                ()
            }
            DAE::Element::INITIAL_NORETCALL { .. } => {
                (outEqns, outREqns, outIEqns) = lowerAlgorithm(
                    el,
                    inFunctions,
                    outEqns,
                    outREqns,
                    outIEqns,
                    openmodelica_frontend_types::DAE::Expand::NOT_EXPAND,
                    true,
                )?;
                ()
            }
            DAE::Element::CONSTRAINT {
                constraints: __esc_constraints,
                ..
            } => {
                constraints = (*__esc_constraints).clone();
                outConstraints = metamodelica::cons(constraints.clone(), outConstraints);
                ()
            }
            DAE::Element::CLASS_ATTRIBUTES {
                classAttrs: __esc_class_attrs,
            } => {
                class_attrs = (*__esc_class_attrs).clone();
                outClassAttributes = metamodelica::cons(class_attrs.clone(), outClassAttributes);
                ()
            }
            _ => {
                let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else {
                    return Err("pattern mismatch");
                };
                Debug::traceln({
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!("BackendDAECreate.lower2"));
                    __mm_s.push_str(&*literal!(" failed on: "));
                    __mm_s.push_str(&*DAEDump::dumpElementsStr(&(list![el]))?);
                    ArcStr::from(__mm_s)
                })?;
                return Err("fail");
            }
        });
    }
    Ok((
        outVars,
        outGlobalKnownVars,
        outExVars,
        outEqns,
        outREqns,
        outIEqns,
        outConstraints,
        outClassAttributes,
        outExtObjClasses,
        outAliasEqns,
        outInlineHT,
    ))
}

// =============================================================================
// section for processing builtin expressions
//
// Insert a unique index (starting with 1) before the first arguments of some
// builtin calls. Equal calls will get the same index.
//   - delay(expr, delayTime, delayMax)
//       => delay(index, expr, delayTime, delayMax)
//   - sample(start, interval)
//       => sample(index, start, interval)
// =============================================================================
fn processBuiltinExpressions(
    mut inDAE: DAE::DAElist,
    mut functionTree: metamodelica::Ref<AvlTreePathFunction::Tree>,
) -> Result<(
    DAE::DAElist,
    metamodelica::Ref<AvlTreePathFunction::Tree>,
    metamodelica::List<BackendDAE::TimeEvent>,
)> {
    let mut outDAE: DAE::DAElist;
    let mut outTree: metamodelica::Ref<AvlTreePathFunction::Tree>;
    let mut outTimeEvents: metamodelica::List<BackendDAE::TimeEvent>;
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
    ht = HashTableExpToIndex::emptyHashTable();
    let (__pa0, __pa1, (_, (_, _, _, _, __pa2))) = DAEUtil::traverseDAE(
        inDAE,
        functionTree,
        (std::sync::Arc::new(Expression::traverseSubexpressionsHelper)
            as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>, _) -> Result<_> + 'static>),
        (
            (std::sync::Arc::new(transformBuiltinExpression)
                as std::sync::Arc<
                    dyn ::std::ops::Fn(
                            metamodelica::Ref<DAE::Exp>,
                            (
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
                                                )
                                                    -> Result<bool>
                                                + 'static,
                                        >,
                                        Arc<
                                            dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<ArcStr> + 'static,
                                        >,
                                        Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>,
                                    ),
                                ),
                                i32,
                                i32,
                                i32,
                                metamodelica::List<BackendDAE::TimeEvent>,
                            ),
                        ) -> Result<(
                            metamodelica::Ref<DAE::Exp>,
                            (
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
                                                )
                                                    -> Result<bool>
                                                + 'static,
                                        >,
                                        Arc<
                                            dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<ArcStr> + 'static,
                                        >,
                                        Arc<dyn ::std::ops::Fn(i32) -> Result<ArcStr> + 'static>,
                                    ),
                                ),
                                i32,
                                i32,
                                i32,
                                metamodelica::List<BackendDAE::TimeEvent>,
                            ),
                        )> + 'static,
                >),
            (ht, 0, 0, 0, metamodelica::nil()),
        ),
    )?;
    outDAE = metamodelica::Own::own(__pa0);
    outTree = metamodelica::Own::own(__pa1);
    outTimeEvents = metamodelica::Own::own(__pa2);
    Ok((outDAE, outTree, outTimeEvents))
}

fn transformBuiltinExpression(
    mut inExp: metamodelica::Ref<DAE::Exp>,
    mut inTuple: (
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
        i32,
        i32,
        i32,
        metamodelica::List<BackendDAE::TimeEvent>,
    ),
) -> Result<(
    metamodelica::Ref<DAE::Exp>,
    (
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
        i32,
        i32,
        i32,
        metamodelica::List<BackendDAE::TimeEvent>,
    ),
)> {
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    let mut outTuple: (
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
        i32,
        i32,
        i32,
        metamodelica::List<BackendDAE::TimeEvent>,
    );
    (outExp, outTuple) = (::match_deref::match_deref! { match &((inExp.clone(), inTuple.clone())) {
        (Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "delay" }, expLst: es, attr }, (ht, _, _, _, _)) if (BaseHashTable::hasKey(inExp.clone(), &(ht.clone()))?) => {
            (metamodelica::Ref::new(DAE::Exp::CALL { path: metamodelica::Ref::new(Absyn::Path::IDENT { name: literal!("delay") }), expLst: metamodelica::cons(metamodelica::Ref::new(DAE::Exp::ICONST { integer: BaseHashTable::get(inExp.clone(), &(ht.clone()))? }), es.clone()), attr: attr.clone() }), inTuple)
        },
        (Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "delay" }, expLst: es, attr }, (ht, iDelay, iSample, iSpatial, timeEvents)) => {
            let mut ht = (*ht).clone();
            ht = BaseHashTable::add((inExp.clone(), iDelay.clone() + 1), ht.clone())?;
            (metamodelica::Ref::new(DAE::Exp::CALL { path: metamodelica::Ref::new(Absyn::Path::IDENT { name: literal!("delay") }), expLst: metamodelica::cons(metamodelica::Ref::new(DAE::Exp::ICONST { integer: iDelay.clone() }), es.clone()), attr: attr.clone() }), (ht.clone(), iDelay.clone() + 1, iSample.clone(), iSpatial.clone(), timeEvents.clone()))
        },
        (Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "spatialDistribution" }, expLst: es, attr }, (ht, _, _, _, _)) if (BaseHashTable::hasKey(inExp.clone(), &(ht.clone()))?) => {
            (metamodelica::Ref::new(DAE::Exp::CALL { path: metamodelica::Ref::new(Absyn::Path::IDENT { name: literal!("spatialDistribution") }), expLst: metamodelica::cons(metamodelica::Ref::new(DAE::Exp::ICONST { integer: BaseHashTable::get(inExp.clone(), &(ht.clone()))? }), es.clone()), attr: attr.clone() }), inTuple)
        },
        (Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "spatialDistribution" }, expLst: es, attr }, (ht, iDelay, iSample, iSpatial, timeEvents)) => {
            let mut ht = (*ht).clone();
            ht = BaseHashTable::add((inExp.clone(), iSpatial.clone() + 1), ht.clone())?;
            (metamodelica::Ref::new(DAE::Exp::CALL { path: metamodelica::Ref::new(Absyn::Path::IDENT { name: literal!("spatialDistribution") }), expLst: metamodelica::cons(metamodelica::Ref::new(DAE::Exp::ICONST { integer: iSpatial.clone() }), es.clone()), attr: attr.clone() }), (ht.clone(), iDelay.clone(), iSample.clone(), iSpatial.clone() + 1, timeEvents.clone()))
        },
        (Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "sample" }, expLst: es @ Deref @ metamodelica::ListNode::Cons { head: _, tail: Deref @ metamodelica::ListNode::Cons { head: interval, tail: Deref @ metamodelica::ListNode::Nil } }, attr }, (ht, _, _, _, _)) if (!(Types::isClockOrSubTypeClock(Expression::r#typeof(interval.clone())?)) && BaseHashTable::hasKey(inExp.clone(), &(ht.clone()))?) => {
            (metamodelica::Ref::new(DAE::Exp::CALL { path: metamodelica::Ref::new(Absyn::Path::IDENT { name: literal!("sample") }), expLst: metamodelica::cons(metamodelica::Ref::new(DAE::Exp::ICONST { integer: BaseHashTable::get(inExp.clone(), &(ht.clone()))? }), es.clone()), attr: attr.clone() }), inTuple)
        },
        (Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "sample" }, expLst: es @ Deref @ metamodelica::ListNode::Cons { head: start, tail: Deref @ metamodelica::ListNode::Cons { head: interval, tail: Deref @ metamodelica::ListNode::Nil } }, attr }, (ht, iDelay, iSample, iSpatial, timeEvents)) if (!(Types::isClockOrSubTypeClock(Expression::r#typeof(interval.clone())?))) => {
            let mut ht = (*ht).clone();
            let mut iSample = (*iSample).clone();
            let mut timeEvents = (*timeEvents).clone();
            iSample = iSample.clone() + 1;
            timeEvents = List::appendElt(BackendDAE::TimeEvent::SAMPLE_TIME_EVENT { index: iSample.clone(), startExp: start.clone(), intervalExp: interval.clone(), iter: None }, timeEvents.clone());
            ht = BaseHashTable::add((inExp.clone(), iSample.clone()), ht.clone())?;
            (metamodelica::Ref::new(DAE::Exp::CALL { path: metamodelica::Ref::new(Absyn::Path::IDENT { name: literal!("sample") }), expLst: metamodelica::cons(metamodelica::Ref::new(DAE::Exp::ICONST { integer: iSample.clone() }), es.clone()), attr: attr.clone() }), (ht.clone(), iDelay.clone(), iSample.clone(), iSpatial.clone(), timeEvents.clone()))
        },
        _ => {
            (inExp.clone(), inTuple)
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outExp, outTuple))
}

/*
 *  lower all variables
 */
pub(crate) fn lowerVars(
    mut inElements: &metamodelica::List<metamodelica::Ref<DAE::Element>>,
    mut functionTree: &metamodelica::Ref<AvlTreePathFunction::Tree>,
    mut inVars: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
    mut inGlobalKnownVars: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
    mut inExVars: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
    mut inEqns: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
    mut inREqns: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
) -> Result<(
    metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
    metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
    metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
    metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
    metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
)> {
    let mut outVars: metamodelica::List<metamodelica::Ref<BackendDAE::Var>> = inVars;
    let mut outGlobalKnownVars: metamodelica::List<metamodelica::Ref<BackendDAE::Var>> = inGlobalKnownVars;
    let mut outExVars: metamodelica::List<metamodelica::Ref<BackendDAE::Var>> = inExVars;
    let mut outEqns: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>> = inEqns;
    let mut outREqns: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>> = inREqns;
    let mut cr: metamodelica::Ref<DAE::ComponentRef>;
    let mut arr_ty: metamodelica::Ref<DAE::Type>;
    let mut crefs: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
    let mut new_vars: metamodelica::List<metamodelica::Ref<DAE::Element>>;
    let mut inline_ht: (
        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::Exp>, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<Option<(metamodelica::Ref<DAE::Exp>, metamodelica::Ref<DAE::Exp>)>>,
        ),
        i32,
        (
            HashTableExpToExp::FuncHashCref,
            HashTableExpToExp::FuncCrefEqual,
            HashTableExpToExp::FuncCrefStr,
            HashTableExpToExp::FuncExpStr,
        ),
    ) = HashTableExpToExp::emptyHashTable();
    for mut el in &**inElements {
        let mut el = el.clone();
        match '__try0: {
            let (__pa1, __pa2) = ::match_deref::match_deref! { match &(el.clone()) {
                Deref @ DAE::Element::VAR { componentRef: __pa1, ty: Deref @ DAE::Type::T_ARRAY { ty: __pa2, .. }, .. } => (__pa1.clone(), __pa2.clone()),
                _ => break '__try0 Err::<_, _>("pattern mismatch"),
            } };
            cr = metamodelica::Own::own(__pa1);
            arr_ty = metamodelica::Own::own(__pa2);
            crefs = unwrap_break_err!(ComponentReference::expandCref(&cr, false), '__try0);
            el = unwrap_break_err!(DAEUtil::replaceTypeInVar(arr_ty.clone(), &el), '__try0);
            new_vars = ({
                let mut __acc: metamodelica::List<metamodelica::Ref<DAE::Element>> = metamodelica::nil();
                for mut c in (crefs.clone()).into_iter().cloned() {
                    let __x = unwrap_break_err!(DAEUtil::replaceCrefInVar(c.clone(), &el), '__try0);
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            });
            (outVars, outGlobalKnownVars, outExVars, outEqns, outREqns) = unwrap_break_err!(lowerVars(&new_vars, functionTree, outVars.clone(), outGlobalKnownVars.clone(), outExVars.clone(), outEqns.clone(), outREqns.clone()), '__try0);
            Ok::<_, &'static str>((
                outEqns.clone(),
                outExVars.clone(),
                outGlobalKnownVars.clone(),
                outREqns.clone(),
                outVars.clone(),
            ))
        } {
            Ok((__try0_o0, __try0_o1, __try0_o2, __try0_o3, __try0_o4)) => {
                outEqns = __try0_o0;
                outExVars = __try0_o1;
                outGlobalKnownVars = __try0_o2;
                outREqns = __try0_o3;
                outVars = __try0_o4;
            }
            Err(_) => {
                (outVars, outGlobalKnownVars, outExVars, outEqns, outREqns, _) = lowerVar(
                    el.clone(),
                    functionTree,
                    outVars.clone(),
                    outGlobalKnownVars.clone(),
                    outExVars.clone(),
                    outEqns.clone(),
                    outREqns.clone(),
                    inline_ht.clone(),
                )?;
            }
        }
    }
    Ok((outVars, outGlobalKnownVars, outExVars, outEqns, outREqns))
}

fn lowerVar(
    mut inElement: metamodelica::Ref<DAE::Element>,
    mut inFunctions: &metamodelica::Ref<AvlTreePathFunction::Tree>,
    mut inVars: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
    mut inGlobalKnownVars: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
    mut inExVars: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
    mut inEqns: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
    mut inREqns: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
    mut inInlineHT: (
        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::Exp>, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<Option<(metamodelica::Ref<DAE::Exp>, metamodelica::Ref<DAE::Exp>)>>,
        ),
        i32,
        (
            Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<i32> + 'static>,
            Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>, metamodelica::Ref<DAE::Exp>) -> Result<bool> + 'static>,
            Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<ArcStr> + 'static>,
            Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<ArcStr> + 'static>,
        ),
    ),
) -> Result<(
    metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
    metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
    metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
    metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
    metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
    (
        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::Exp>, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<Option<(metamodelica::Ref<DAE::Exp>, metamodelica::Ref<DAE::Exp>)>>,
        ),
        i32,
        (
            Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<i32> + 'static>,
            Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>, metamodelica::Ref<DAE::Exp>) -> Result<bool> + 'static>,
            Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<ArcStr> + 'static>,
            Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<ArcStr> + 'static>,
        ),
    ),
)> {
    let mut outVars: metamodelica::List<metamodelica::Ref<BackendDAE::Var>> = inVars;
    let mut outGlobalKnownVars: metamodelica::List<metamodelica::Ref<BackendDAE::Var>> = inGlobalKnownVars;
    let mut outExVars: metamodelica::List<metamodelica::Ref<BackendDAE::Var>> = inExVars;
    let mut outEqns: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>> = inEqns;
    let mut outREqns: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>> = inREqns;
    let mut outInlineHT: (
        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::Exp>, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<Option<(metamodelica::Ref<DAE::Exp>, metamodelica::Ref<DAE::Exp>)>>,
        ),
        i32,
        (
            HashTableExpToExp::FuncHashCref,
            HashTableExpToExp::FuncCrefEqual,
            HashTableExpToExp::FuncCrefStr,
            HashTableExpToExp::FuncExpStr,
        ),
    ) = inInlineHT;
    let () = 'mc: {
        let __mc_input = &*inElement;
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Element::VAR { ty: Deref @ DAE::Type::T_COMPLEX { complexClassType: ClassInf::State::EXTERNAL_OBJ { .. }, .. }, .. } => {
                    let mut outExVars: metamodelica::List<metamodelica::Ref<BackendDAE::Var>> = outExVars.clone();
                    outExVars = metamodelica::cons(lowerExtObjVar(&inElement, inFunctions)?, outExVars.clone());
                    Ok(((), outExVars.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            outExVars = __wb0;
            break 'mc __v;
        }
        if let Ok((__v, __wb0, __wb1)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                        Deref @ DAE::Element::VAR { componentRef: cr, binding: Some(e2), source: src, .. } => {
                            if !((isStateOrAlgvar(&inElement))) { return Err("guard") }
                            let mut e1: metamodelica::Ref<DAE::Exp>;
                            let mut dims: metamodelica::List<metamodelica::Ref<DAE::Dimension>>;
                            let mut tp: metamodelica::Ref<DAE::Type>;
                            let mut attr: BackendDAE::EquationAttributes;
                            let mut recordSize: Option<i32>;
                            let mut e2 = (*e2).clone();
                            let mut outEqns: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>> = outEqns.clone();
                            let mut outVars: metamodelica::List<metamodelica::Ref<BackendDAE::Var>> = outVars.clone();
                            outVars = metamodelica::cons(lowerDynamicVar(&inElement, inFunctions)?, outVars.clone());
                            attr = BackendDAE::EQ_ATTR_DEFAULT_BINDING.clone();
                            (tp, dims) = ComponentReference::crefTypeFull2(metamodelica::AsArg::as_arg(&cr), metamodelica::nil())?;
                            tp = DAEUtil::expTypeElementType(&tp);
                            if DAEUtil::expTypeComplex(&tp) {
                                recordSize = Some(Expression::sizeOf(&tp));
                            } else {
                                recordSize = None;
                            }
                            e2 = (::match_deref::match_deref! { match &((Flags::isSet(Flags::NF_SCALARIZE.clone())?, (dims).is_empty(), e2.clone())) {
                (false, false, Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "fill" }, expLst: Deref @ metamodelica::ListNode::Cons { head: __esc_e1, tail: Deref @ metamodelica::ListNode::Cons { head: _, tail: Deref @ metamodelica::ListNode::Nil } }, .. }) => {
                            e1 = (*__esc_e1).clone();
                            e1.clone()
                },
                _ => e2.clone(),
                _ => unreachable!("match_deref! exhaustiveness placeholder"),
            } });
                            e1 = Expression::crefExp(cr.clone())?;
                            if (dims).is_empty() {
                                outEqns = metamodelica::cons(metamodelica::Ref::new(BackendDAE::Equation::EQUATION { exp: e1.clone(), scalar: e2.clone(), source: src.clone(), attr: attr }), outEqns.clone());
                            } else {
                                outEqns = metamodelica::cons(metamodelica::Ref::new(BackendDAE::Equation::ARRAY_EQUATION { dimSize: Expression::dimensionsSizes(dims.clone())?, left: e1.clone(), right: e2.clone(), source: src.clone(), attr: attr, recordSize: recordSize.clone() }), outEqns.clone());
                            }
                            Ok(((), outEqns.clone(), outVars.clone()))
                        }
                        _ => return Err("nomatch"),
                    }}
        })() {
            outEqns = __wb0;
            outVars = __wb1;
            break 'mc __v;
        }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Element::VAR { binding: None, .. } => {
                    if !((isStateOrAlgvar(&inElement))) { return Err("guard") }
                    let mut outVars: metamodelica::List<metamodelica::Ref<BackendDAE::Var>> = outVars.clone();
                    outVars = metamodelica::cons(lowerDynamicVar(&inElement, inFunctions)?, outVars.clone());
                    Ok(((), outVars.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            outVars = __wb0;
            break 'mc __v;
        }
        if let Ok((__v, __wb0, __wb1, __wb2)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Element::VAR { .. } => {
                    let mut var: metamodelica::Ref<BackendDAE::Var>;
                    let mut outGlobalKnownVars: metamodelica::List<metamodelica::Ref<BackendDAE::Var>> = outGlobalKnownVars.clone();
                    let mut outInlineHT: (metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::Exp>, i32)>>, (i32, i32, metamodelica::Array<Option<(metamodelica::Ref<DAE::Exp>, metamodelica::Ref<DAE::Exp>)>>), i32, (HashTableExpToExp::FuncHashCref, HashTableExpToExp::FuncCrefEqual, HashTableExpToExp::FuncCrefStr, HashTableExpToExp::FuncExpStr)) = outInlineHT.clone();
                    let mut outREqns: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>> = outREqns.clone();
                    (var, outInlineHT, outREqns) = lowerKnownVar(inElement.clone(), inFunctions, outInlineHT.clone(), outREqns.clone())?;
                    outGlobalKnownVars = metamodelica::cons(var.clone(), outGlobalKnownVars.clone());
                    Ok(((), outGlobalKnownVars.clone(), outInlineHT.clone(), outREqns.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            outGlobalKnownVars = __wb0;
            outInlineHT = __wb1;
            outREqns = __wb2;
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Error::addMessage(Error::INTERNAL_ERROR.clone(), list![{ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("BackendDAECreate.lowerVar failed for ")); __mm_s.push_str(&*DAEDump::dumpElementsStr(&(list![inElement.clone()]))?); ArcStr::from(__mm_s) }])?;
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((outVars, outGlobalKnownVars, outExVars, outEqns, outREqns, outInlineHT))
}

fn isStateOrAlgvar(mut e: &metamodelica::Ref<DAE::Element>) -> bool {
    let mut out: bool;
    out = (match &**e {
        DAE::Element::VAR {
            kind: DAE::VarKind::VARIABLE { .. },
            ..
        } => true,
        DAE::Element::VAR {
            kind: DAE::VarKind::DISCRETE { .. },
            ..
        } => true,
        _ => false,
    });
    out
}

fn lowerDynamicVar(
    mut inElement: &metamodelica::Ref<DAE::Element>,
    mut functionTree: &metamodelica::Ref<AvlTreePathFunction::Tree>,
) -> Result<metamodelica::Ref<BackendDAE::Var>> {
    let mut outVar: metamodelica::Ref<BackendDAE::Var>;
    outVar = (match &**inElement {
        DAE::Element::VAR {
            componentRef: name,
            kind,
            direction: dir,
            parallelism: prl,
            protection,
            ty: t,
            dims,
            connectorType: ct,
            source,
            variableAttributesOption: dae_var_attr,
            comment,
            innerOuter: io,
            encrypted,
            ..
        } => {
            let mut kind_1: BackendDAE::VarKind;
            let mut tp: metamodelica::Ref<DAE::Type>;
            let mut ts: Option<BackendDAE::TearingSelect>;
            let mut hideResult: Option<metamodelica::Ref<DAE::Exp>>;
            let mut b: bool;
            let mut dae_var_attr = (*dae_var_attr).clone();
            kind_1 = lowerVarkind(
                kind.clone(),
                t,
                name,
                dir.clone(),
                ct,
                dae_var_attr.clone(),
                protection.clone(),
            )?;
            tp = lowerType(t.clone())?;
            b = DAEUtil::boolVarVisibility(protection.clone())?;
            dae_var_attr = DAEUtil::setProtectedAttr(dae_var_attr.clone(), b)?;
            dae_var_attr = setMinMaxFromEnumeration(t, dae_var_attr.clone());
            if !(Flags::isSet(Flags::NF_SCALARIZE.clone())?) && !((dims).is_empty()) {
                dae_var_attr = replaceFillWithExpInAttributes(dae_var_attr.clone());
            }
            ts = BackendDAEUtil::setTearingSelectAttribute(comment.clone())?;
            hideResult = BackendDAEUtil::setHideResultAttribute(comment.clone(), name);
            metamodelica::Ref::new(BackendDAE::Var {
                varName: name.clone(),
                varKind: kind_1,
                varDirection: dir.clone(),
                varParallelism: prl.clone(),
                varType: tp,
                bindExp: None,
                tplExp: None,
                arryDim: dims.clone(),
                source: source.clone(),
                values: dae_var_attr.clone(),
                tearingSelectOption: ts,
                hideResult: hideResult,
                comment: comment.clone(),
                connectorType: ct.clone(),
                innerOuter: DAEUtil::toDAEInnerOuter(io.clone()),
                unreplaceable: false,
                initNonlinear: false,
                encrypted: encrypted.clone(),
            })
        }
        _ => return Err("match: no arm matched"),
    });
    Ok(outVar)
}

fn lowerKnownVar(
    mut inElement: metamodelica::Ref<DAE::Element>,
    mut functionTree: &metamodelica::Ref<AvlTreePathFunction::Tree>,
    mut iInlineHT: (
        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::Exp>, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<Option<(metamodelica::Ref<DAE::Exp>, metamodelica::Ref<DAE::Exp>)>>,
        ),
        i32,
        (
            Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<i32> + 'static>,
            Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>, metamodelica::Ref<DAE::Exp>) -> Result<bool> + 'static>,
            Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<ArcStr> + 'static>,
            Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<ArcStr> + 'static>,
        ),
    ),
    mut assrtEqIn: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
) -> Result<(
    metamodelica::Ref<BackendDAE::Var>,
    (
        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::Exp>, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<Option<(metamodelica::Ref<DAE::Exp>, metamodelica::Ref<DAE::Exp>)>>,
        ),
        i32,
        (
            Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<i32> + 'static>,
            Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>, metamodelica::Ref<DAE::Exp>) -> Result<bool> + 'static>,
            Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<ArcStr> + 'static>,
            Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<ArcStr> + 'static>,
        ),
    ),
    metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
)> {
    let mut outVar: metamodelica::Ref<BackendDAE::Var>;
    let mut oInlineHT: (
        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::Exp>, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<Option<(metamodelica::Ref<DAE::Exp>, metamodelica::Ref<DAE::Exp>)>>,
        ),
        i32,
        (
            HashTableExpToExp::FuncHashCref,
            HashTableExpToExp::FuncCrefEqual,
            HashTableExpToExp::FuncCrefStr,
            HashTableExpToExp::FuncExpStr,
        ),
    );
    let mut assrtEqOut: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
    (outVar, oInlineHT, assrtEqOut) = 'mc: {
        let __mc_input = &*inElement;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Element::VAR { componentRef: name, kind, direction: dir, parallelism: prl, protection, ty: t, binding: bind, dims, connectorType: ct, source, variableAttributesOption: dae_var_attr, comment, innerOuter: io, encrypted } => {
                    let mut kind_1: BackendDAE::VarKind;
                    let mut tp: metamodelica::Ref<DAE::Type>;
                    let mut ts: Option<BackendDAE::TearingSelect>;
                    let mut hideResult: Option<metamodelica::Ref<DAE::Exp>>;
                    let mut b: bool;
                    let mut eqLst: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
                    let mut bind = (*bind).clone();
                    let mut dae_var_attr = (*dae_var_attr).clone();
                    kind_1 = lowerKnownVarkind(kind.clone(), metamodelica::AsArg::as_arg(&name), dir.clone(), metamodelica::AsArg::as_arg(&ct), protection.clone())?;
                    if !(Flags::isSet(Flags::NF_SCALARIZE.clone())?) && !((dims).is_empty()) {
                        bind = replaceFillWithExp(bind.clone());
                        dae_var_attr = replaceFillWithExpInAttributes(dae_var_attr.clone());
                    }
                    tp = lowerType(t.clone())?;
                    b = DAEUtil::boolVarVisibility(protection.clone())?;
                    dae_var_attr = DAEUtil::setProtectedAttr(dae_var_attr.clone(), b)?;
                    dae_var_attr = setMinMaxFromEnumeration(metamodelica::AsArg::as_arg(&t), dae_var_attr.clone());
                    eqLst = buildAssertAlgorithms(&(metamodelica::nil()), source.clone(), assrtEqIn.clone());
                    ts = None;
                    hideResult = BackendDAEUtil::setHideResultAttribute(comment.clone(), metamodelica::AsArg::as_arg(&name));
                    Ok((metamodelica::Ref::new(BackendDAE::Var { varName: name.clone(), varKind: kind_1.clone(), varDirection: dir.clone(), varParallelism: prl.clone(), varType: tp.clone(), bindExp: bind.clone(), tplExp: None, arryDim: dims.clone(), source: source.clone(), values: dae_var_attr.clone(), tearingSelectOption: ts.clone(), hideResult: hideResult.clone(), comment: comment.clone(), connectorType: ct.clone(), innerOuter: DAEUtil::toDAEInnerOuter(io.clone()), unreplaceable: false, initNonlinear: false, encrypted: encrypted.clone() }), iInlineHT.clone(), eqLst.clone()))
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
                    r#str = { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("BackendDAECreate.lowerKnownVar failed for ")); __mm_s.push_str(&*DAEDump::dumpElementsStr(&(list![inElement.clone()]))?); ArcStr::from(__mm_s) };
                    Error::addMessage(Error::INTERNAL_ERROR.clone(), list![r#str.clone()])?;
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((outVar, oInlineHT, assrtEqOut))
}

pub(crate) fn lowerKnownVarSingle(
    mut element: metamodelica::Ref<DAE::Element>,
) -> Result<Option<metamodelica::Ref<BackendDAE::Var>>> {
    let mut var_opt: Option<metamodelica::Ref<BackendDAE::Var>>;
    var_opt = (::match_deref::match_deref! { match &(element.clone()) {
        elem @ Deref @ DAE::Element::VAR { .. } if (DAEUtil::isParamOrConstVarKind(var_field!((**elem).kind, DAE::Element::VAR).clone())) => {
            let mut visibility: bool;
            let mut var: metamodelica::Ref<BackendDAE::Var>;
            visibility = DAEUtil::boolVarVisibility(var_field!((**elem).protection, DAE::Element::VAR).clone())?;
            var = metamodelica::Ref::new(BackendDAE::Var { varName: var_field!((**elem).componentRef, DAE::Element::VAR).clone(), varKind: lowerKnownVarkind(var_field!((**elem).kind, DAE::Element::VAR).clone(), var_field!((**elem).componentRef, DAE::Element::VAR), var_field!((**elem).direction, DAE::Element::VAR).clone(), var_field!((**elem).connectorType, DAE::Element::VAR), var_field!((**elem).protection, DAE::Element::VAR).clone())?, varDirection: var_field!((**elem).direction, DAE::Element::VAR).clone(), varParallelism: var_field!((**elem).parallelism, DAE::Element::VAR).clone(), varType: lowerType(var_field!((**elem).ty, DAE::Element::VAR).clone())?, bindExp: var_field!((**elem).binding, DAE::Element::VAR).clone(), tplExp: None, arryDim: var_field!((*element).dims, DAE::Element::VAR).clone(), source: var_field!((*element).source, DAE::Element::VAR).clone(), values: setMinMaxFromEnumeration(var_field!((**elem).ty, DAE::Element::VAR), DAEUtil::setProtectedAttr(var_field!((**elem).variableAttributesOption, DAE::Element::VAR).clone(), visibility)?), tearingSelectOption: None, hideResult: BackendDAEUtil::setHideResultAttribute(var_field!((*element).comment, DAE::Element::VAR).clone(), var_field!((**elem).componentRef, DAE::Element::VAR)), comment: var_field!((*element).comment, DAE::Element::VAR).clone(), connectorType: var_field!((*element).connectorType, DAE::Element::VAR).clone(), innerOuter: DAEUtil::toDAEInnerOuter(var_field!((*element).innerOuter, DAE::Element::VAR).clone()), unreplaceable: false, initNonlinear: false, encrypted: var_field!((**elem).encrypted, DAE::Element::VAR).clone() });
            Some(var)
        },
        _ => {
            None
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(var_opt)
}

fn replaceFillWithExpInAttributes(
    mut attr: Option<metamodelica::Ref<DAE::VariableAttributes>>,
) -> Option<metamodelica::Ref<DAE::VariableAttributes>> {
    let mut attr: Option<metamodelica::Ref<DAE::VariableAttributes>> = attr;
    attr = (::match_deref::match_deref! { match &(attr.clone()) {
        Some(Deref @ DAE::VariableAttributes::VAR_ATTR_REAL { quantity: q, unit: u, displayUnit: du, min, max, start: i, fixed: f, nominal: n, stateSelectOption: ss, uncertainOption: unc, distributionOption: distOpt, equationBound: eb, isProtected: ip, finalPrefix: r#fn, startOrigin: so }) => {
            let mut q = (*q).clone();
            let mut u = (*u).clone();
            let mut du = (*du).clone();
            let mut min = (*min).clone();
            let mut max = (*max).clone();
            let mut i = (*i).clone();
            let mut f = (*f).clone();
            let mut n = (*n).clone();
            q = replaceFillWithExp(q.clone());
            u = replaceFillWithExp(u.clone());
            du = replaceFillWithExp(du.clone());
            min = replaceFillWithExp(min.clone());
            max = replaceFillWithExp(max.clone());
            i = replaceFillWithExp(i.clone());
            f = replaceFillWithExp(f.clone());
            n = replaceFillWithExp(n.clone());
            Some(metamodelica::Ref::new(DAE::VariableAttributes::VAR_ATTR_REAL { quantity: q.clone(), unit: u.clone(), displayUnit: du.clone(), min: min.clone(), max: max.clone(), start: i.clone(), fixed: f.clone(), nominal: n.clone(), stateSelectOption: ss.clone(), uncertainOption: unc.clone(), distributionOption: distOpt.clone(), equationBound: eb.clone(), isProtected: ip.clone(), finalPrefix: r#fn.clone(), startOrigin: so.clone() }))
        },
        Some(Deref @ DAE::VariableAttributes::VAR_ATTR_INT { quantity: q, min, max, start: i, fixed: f, uncertainOption: unc, distributionOption: distOpt, equationBound: eb, isProtected: ip, finalPrefix: r#fn, startOrigin: so }) => {
            let mut q = (*q).clone();
            let mut min = (*min).clone();
            let mut max = (*max).clone();
            let mut i = (*i).clone();
            let mut f = (*f).clone();
            q = replaceFillWithExp(q.clone());
            min = replaceFillWithExp(min.clone());
            max = replaceFillWithExp(max.clone());
            i = replaceFillWithExp(i.clone());
            f = replaceFillWithExp(f.clone());
            Some(metamodelica::Ref::new(DAE::VariableAttributes::VAR_ATTR_INT { quantity: q.clone(), min: min.clone(), max: max.clone(), start: i.clone(), fixed: f.clone(), uncertainOption: unc.clone(), distributionOption: distOpt.clone(), equationBound: eb.clone(), isProtected: ip.clone(), finalPrefix: r#fn.clone(), startOrigin: so.clone() }))
        },
        Some(Deref @ DAE::VariableAttributes::VAR_ATTR_BOOL { quantity: q, start: i, fixed: f, equationBound: eb, isProtected: ip, finalPrefix: r#fn, startOrigin: so }) => {
            let mut q = (*q).clone();
            let mut i = (*i).clone();
            let mut f = (*f).clone();
            q = replaceFillWithExp(q.clone());
            i = replaceFillWithExp(i.clone());
            f = replaceFillWithExp(f.clone());
            Some(metamodelica::Ref::new(DAE::VariableAttributes::VAR_ATTR_BOOL { quantity: q.clone(), start: i.clone(), fixed: f.clone(), equationBound: eb.clone(), isProtected: ip.clone(), finalPrefix: r#fn.clone(), startOrigin: so.clone() }))
        },
        Some(Deref @ DAE::VariableAttributes::VAR_ATTR_STRING { quantity: q, start: i, fixed: f, equationBound: eb, isProtected: ip, finalPrefix: r#fn, startOrigin: so }) => {
            let mut q = (*q).clone();
            let mut i = (*i).clone();
            let mut f = (*f).clone();
            q = replaceFillWithExp(q.clone());
            i = replaceFillWithExp(i.clone());
            f = replaceFillWithExp(f.clone());
            Some(metamodelica::Ref::new(DAE::VariableAttributes::VAR_ATTR_STRING { quantity: q.clone(), start: i.clone(), fixed: f.clone(), equationBound: eb.clone(), isProtected: ip.clone(), finalPrefix: r#fn.clone(), startOrigin: so.clone() }))
        },
        Some(Deref @ DAE::VariableAttributes::VAR_ATTR_ENUMERATION { quantity: q, min, max, start: u, fixed: du, equationBound: eb, isProtected: ip, finalPrefix: r#fn, startOrigin: so }) => {
            let mut q = (*q).clone();
            let mut min = (*min).clone();
            let mut max = (*max).clone();
            let mut u = (*u).clone();
            let mut du = (*du).clone();
            q = replaceFillWithExp(q.clone());
            min = replaceFillWithExp(min.clone());
            max = replaceFillWithExp(max.clone());
            u = replaceFillWithExp(u.clone());
            du = replaceFillWithExp(du.clone());
            Some(metamodelica::Ref::new(DAE::VariableAttributes::VAR_ATTR_ENUMERATION { quantity: q.clone(), min: min.clone(), max: max.clone(), start: u.clone(), fixed: du.clone(), equationBound: eb.clone(), isProtected: ip.clone(), finalPrefix: r#fn.clone(), startOrigin: so.clone() }))
        },
        _ => {
            attr
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    attr
}

fn replaceFillWithExp(mut bind: Option<metamodelica::Ref<DAE::Exp>>) -> Option<metamodelica::Ref<DAE::Exp>> {
    let mut bind: Option<metamodelica::Ref<DAE::Exp>> = bind;
    let mut e1: metamodelica::Ref<DAE::Exp>;
    bind = (::match_deref::match_deref! { match &(bind.clone()) {
        Some(Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "fill" }, expLst: Deref @ metamodelica::ListNode::Cons { head: __esc_e1, tail: Deref @ metamodelica::ListNode::Cons { head: _, tail: Deref @ metamodelica::ListNode::Nil } }, .. }) => {
            e1 = (*__esc_e1).clone();
            Some(e1.clone())
        },
        _ => bind,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    bind
}

fn buildAssertAlgorithms(
    mut assrtIn: &metamodelica::List<metamodelica::Ref<DAE::Statement>>,
    mut source: metamodelica::Ref<DAE::ElementSource>,
    mut eqIn: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
) -> metamodelica::List<metamodelica::Ref<BackendDAE::Equation>> {
    let mut eqOut: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>> = eqIn;
    for mut assrt in &**assrtIn {
        eqOut = metamodelica::cons(
            metamodelica::Ref::new(BackendDAE::Equation::ALGORITHM {
                size: 0,
                alg: metamodelica::Ref::new(DAE::Algorithm {
                    statementLst: list![assrt.clone()],
                }),
                source: source.clone(),
                expand: openmodelica_frontend_types::DAE::Expand::EXPAND,
                attr: BackendDAE::EQ_ATTR_DEFAULT_DYNAMIC.clone(),
            }),
            eqOut,
        );
    }
    eqOut
}

fn inlineExpOpt(
    mut iOptExp: Option<metamodelica::Ref<DAE::Exp>>,
    mut fnstpl: Functiontuple,
    mut iSource: metamodelica::Ref<DAE::ElementSource>,
    mut iInlineHT: (
        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::Exp>, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<Option<(metamodelica::Ref<DAE::Exp>, metamodelica::Ref<DAE::Exp>)>>,
        ),
        i32,
        (
            Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<i32> + 'static>,
            Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>, metamodelica::Ref<DAE::Exp>) -> Result<bool> + 'static>,
            Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<ArcStr> + 'static>,
            Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<ArcStr> + 'static>,
        ),
    ),
) -> (
    Option<metamodelica::Ref<DAE::Exp>>,
    metamodelica::Ref<DAE::ElementSource>,
    (
        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::Exp>, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<Option<(metamodelica::Ref<DAE::Exp>, metamodelica::Ref<DAE::Exp>)>>,
        ),
        i32,
        (
            Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<i32> + 'static>,
            Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>, metamodelica::Ref<DAE::Exp>) -> Result<bool> + 'static>,
            Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<ArcStr> + 'static>,
            Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<ArcStr> + 'static>,
        ),
    ),
    metamodelica::List<metamodelica::Ref<DAE::Statement>>,
) {
    let mut oOptExp: Option<metamodelica::Ref<DAE::Exp>>;
    let mut oSource: metamodelica::Ref<DAE::ElementSource>;
    let mut oInlineHT: (
        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::Exp>, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<Option<(metamodelica::Ref<DAE::Exp>, metamodelica::Ref<DAE::Exp>)>>,
        ),
        i32,
        (
            HashTableExpToExp::FuncHashCref,
            HashTableExpToExp::FuncCrefEqual,
            HashTableExpToExp::FuncCrefStr,
            HashTableExpToExp::FuncExpStr,
        ),
    );
    let mut assrtLstOut: metamodelica::List<metamodelica::Ref<DAE::Statement>>;
    (oOptExp, oSource, oInlineHT, assrtLstOut) = (::match_deref::match_deref! { match &(iOptExp.clone()) {
        None => {
            (iOptExp, iSource, iInlineHT, metamodelica::nil())
        },
        Some(e) => {
            let mut source: metamodelica::Ref<DAE::ElementSource>;
            let mut inlineHT: (metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::Exp>, i32)>>, (i32, i32, metamodelica::Array<Option<(metamodelica::Ref<DAE::Exp>, metamodelica::Ref<DAE::Exp>)>>), i32, (HashTableExpToExp::FuncHashCref, HashTableExpToExp::FuncCrefEqual, HashTableExpToExp::FuncCrefStr, HashTableExpToExp::FuncExpStr));
            let mut assrtLst: metamodelica::List<metamodelica::Ref<DAE::Statement>>;
            let mut e = (*e).clone();
            (e, source, inlineHT, assrtLst) = inlineExpOpt1(e.clone(), fnstpl, iSource, iInlineHT);
            (Some(e.clone()), source, inlineHT, assrtLst)
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    (oOptExp, oSource, oInlineHT, assrtLstOut)
}

fn inlineExpOpt1(
    mut iExp: metamodelica::Ref<DAE::Exp>,
    mut fnstpl: Functiontuple,
    mut iSource: metamodelica::Ref<DAE::ElementSource>,
    mut iInlineHT: (
        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::Exp>, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<Option<(metamodelica::Ref<DAE::Exp>, metamodelica::Ref<DAE::Exp>)>>,
        ),
        i32,
        (
            Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<i32> + 'static>,
            Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>, metamodelica::Ref<DAE::Exp>) -> Result<bool> + 'static>,
            Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<ArcStr> + 'static>,
            Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<ArcStr> + 'static>,
        ),
    ),
) -> (
    metamodelica::Ref<DAE::Exp>,
    metamodelica::Ref<DAE::ElementSource>,
    (
        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::Exp>, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<Option<(metamodelica::Ref<DAE::Exp>, metamodelica::Ref<DAE::Exp>)>>,
        ),
        i32,
        (
            Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<i32> + 'static>,
            Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>, metamodelica::Ref<DAE::Exp>) -> Result<bool> + 'static>,
            Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<ArcStr> + 'static>,
            Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<ArcStr> + 'static>,
        ),
    ),
    metamodelica::List<metamodelica::Ref<DAE::Statement>>,
) {
    let mut oExp: metamodelica::Ref<DAE::Exp>;
    let mut oSource: metamodelica::Ref<DAE::ElementSource>;
    let mut oInlineHT: (
        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::Exp>, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<Option<(metamodelica::Ref<DAE::Exp>, metamodelica::Ref<DAE::Exp>)>>,
        ),
        i32,
        (
            HashTableExpToExp::FuncHashCref,
            HashTableExpToExp::FuncCrefEqual,
            HashTableExpToExp::FuncCrefStr,
            HashTableExpToExp::FuncExpStr,
        ),
    );
    let mut assrtLstOut: metamodelica::List<metamodelica::Ref<DAE::Statement>>;
    (oExp, oSource, oInlineHT, assrtLstOut) = 'mc: {
        let __mc_input = &*iExp;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::CALL { .. } => {
                    let mut e1: metamodelica::Ref<DAE::Exp>;
                    let mut source: metamodelica::Ref<DAE::ElementSource>;
                    e1 = BaseHashTable::get(iExp.clone(), &iInlineHT)?;
                    source = ElementSource::addSymbolicTransformation(iSource.clone(), metamodelica::Ref::new(DAE::SymbolicOperation::OP_INLINE { before: metamodelica::Ref::new(DAE::EquationExp::PARTIAL_EQUATION { exp: iExp.clone() }), after: metamodelica::Ref::new(DAE::EquationExp::PARTIAL_EQUATION { exp: e1.clone() }) }))?;
                    Ok((e1.clone(), source.clone(), iInlineHT.clone(), metamodelica::nil()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::CALL { .. } => {
                    let mut e1: metamodelica::Ref<DAE::Exp>;
                    let mut source: metamodelica::Ref<DAE::ElementSource>;
                    let mut inlineHT: (metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::Exp>, i32)>>, (i32, i32, metamodelica::Array<Option<(metamodelica::Ref<DAE::Exp>, metamodelica::Ref<DAE::Exp>)>>), i32, (HashTableExpToExp::FuncHashCref, HashTableExpToExp::FuncCrefEqual, HashTableExpToExp::FuncCrefStr, HashTableExpToExp::FuncExpStr));
                    let mut inlined: bool;
                    (e1, source, inlined, _) = Inline::inlineExp(iExp.clone(), fnstpl.clone(), iSource.clone());
                    inlineHT = if (inlined) {BaseHashTable::add((iExp.clone(), e1.clone()), iInlineHT.clone())?} else {iInlineHT.clone()};
                    Ok((e1.clone(), source.clone(), inlineHT.clone(), metamodelica::nil()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::ASUB { exp: e, sub: elst } => {
                    let mut e1: metamodelica::Ref<DAE::Exp>;
                    let mut source: metamodelica::Ref<DAE::ElementSource>;
                    let mut e = (*e).clone();
                    e1 = BaseHashTable::get(e.clone(), &iInlineHT)?;
                    source = ElementSource::addSymbolicTransformation(iSource.clone(), metamodelica::Ref::new(DAE::SymbolicOperation::OP_INLINE { before: metamodelica::Ref::new(DAE::EquationExp::PARTIAL_EQUATION { exp: e.clone() }), after: metamodelica::Ref::new(DAE::EquationExp::PARTIAL_EQUATION { exp: e1.clone() }) }))?;
                    (e, source, _, _) = Inline::inlineExp(metamodelica::Ref::new(DAE::Exp::ASUB { exp: e1.clone(), sub: elst.clone() }), fnstpl.clone(), source.clone());
                    Ok((e.clone(), source.clone(), iInlineHT.clone(), metamodelica::nil()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::ASUB { exp: e, sub: elst } => {
                    let mut e1: metamodelica::Ref<DAE::Exp>;
                    let mut source: metamodelica::Ref<DAE::ElementSource>;
                    let mut inlineHT: (metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::Exp>, i32)>>, (i32, i32, metamodelica::Array<Option<(metamodelica::Ref<DAE::Exp>, metamodelica::Ref<DAE::Exp>)>>), i32, (HashTableExpToExp::FuncHashCref, HashTableExpToExp::FuncCrefEqual, HashTableExpToExp::FuncCrefStr, HashTableExpToExp::FuncExpStr));
                    let mut inlined: bool;
                    let mut assrtLst: metamodelica::List<metamodelica::Ref<DAE::Statement>>;
                    let mut assrtLst1: metamodelica::List<metamodelica::Ref<DAE::Statement>>;
                    let mut assrtLst2: metamodelica::List<metamodelica::Ref<DAE::Statement>>;
                    let mut e = (*e).clone();
                    (e1, _, inlined, assrtLst1) = Inline::inlineExp(e.clone(), fnstpl.clone(), iSource.clone());
                    inlineHT = if (inlined) {BaseHashTable::add((e.clone(), e1.clone()), iInlineHT.clone())?} else {iInlineHT.clone()};
                    (e, source, _, assrtLst2) = Inline::inlineExp(metamodelica::Ref::new(DAE::Exp::ASUB { exp: e1.clone(), sub: elst.clone() }), fnstpl.clone(), iSource.clone());
                    assrtLst = listAppend(assrtLst1.clone(), assrtLst2.clone());
                    Ok((e.clone(), source.clone(), inlineHT.clone(), assrtLst.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut e: metamodelica::Ref<DAE::Exp>;
                    let mut source: metamodelica::Ref<DAE::ElementSource>;
                    (e, source, _, _) = Inline::inlineExp(iExp.clone(), fnstpl.clone(), iSource.clone());
                    Ok((e.clone(), source.clone(), iInlineHT.clone(), metamodelica::nil()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    (oExp, oSource, oInlineHT, assrtLstOut)
}

fn setMinMaxFromEnumeration(
    mut inType: &metamodelica::Ref<DAE::Type>,
    mut inVarAttr: Option<metamodelica::Ref<DAE::VariableAttributes>>,
) -> Option<metamodelica::Ref<DAE::VariableAttributes>> {
    let mut outVarAttr: Option<metamodelica::Ref<DAE::VariableAttributes>>;
    outVarAttr = (match &**inType {
        DAE::Type::T_ENUMERATION { path, names, .. } => {
            let mut min: Option<metamodelica::Ref<DAE::Exp>>;
            let mut max: Option<metamodelica::Ref<DAE::Exp>>;
            (min, max) = DAEUtil::getMinMaxValues(inVarAttr.clone());
            setMinMaxFromEnumeration1(min, max, inVarAttr, path.clone(), names)
        }
        _ => inVarAttr,
    });
    outVarAttr
}

fn setMinMaxFromEnumeration1(
    mut inMin: Option<metamodelica::Ref<DAE::Exp>>,
    mut inMax: Option<metamodelica::Ref<DAE::Exp>>,
    mut inVarAttr: Option<metamodelica::Ref<DAE::VariableAttributes>>,
    mut inPath: metamodelica::Ref<Absyn::Path>,
    mut inNames: &metamodelica::List<ArcStr>,
) -> Option<metamodelica::Ref<DAE::VariableAttributes>> {
    let mut outVarAttr: Option<metamodelica::Ref<DAE::VariableAttributes>>;
    outVarAttr = 'mc: {
        let __mc_input = (&inMin, &inMax);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (None, None) => {
                    let mut i: i32;
                    let mut namee1: metamodelica::Ref<Absyn::Path>;
                    let mut nameen: metamodelica::Ref<Absyn::Path>;
                    let mut s1: ArcStr;
                    let mut sn: ArcStr;
                    i = ((inNames).len() as i32);
                    s1 = (inNames).head().cloned()?;
                    namee1 = AbsynUtil::joinPaths(inPath.clone(), metamodelica::Ref::new(Absyn::Path::IDENT { name: s1.clone() }))?;
                    sn = (inNames).get(i)?;
                    nameen = AbsynUtil::joinPaths(inPath.clone(), metamodelica::Ref::new(Absyn::Path::IDENT { name: sn.clone() }))?;
                    Ok(DAEUtil::setMinMax(inVarAttr.clone(), Some(metamodelica::Ref::new(DAE::Exp::ENUM_LITERAL { name: namee1.clone(), index: 1 })), Some(metamodelica::Ref::new(DAE::Exp::ENUM_LITERAL { name: nameen.clone(), index: i })))?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (None, Some(_)) => {
                    let mut namee1: metamodelica::Ref<Absyn::Path>;
                    let mut s1: ArcStr;
                    s1 = (inNames).head().cloned()?;
                    namee1 = AbsynUtil::joinPaths(inPath.clone(), metamodelica::Ref::new(Absyn::Path::IDENT { name: s1.clone() }))?;
                    Ok(DAEUtil::setMinMax(inVarAttr.clone(), Some(metamodelica::Ref::new(DAE::Exp::ENUM_LITERAL { name: namee1.clone(), index: 1 })), inMax.clone())?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Some(_), None) => {
                    let mut i: i32;
                    let mut nameen: metamodelica::Ref<Absyn::Path>;
                    let mut sn: ArcStr;
                    i = ((inNames).len() as i32);
                    sn = (inNames).get(i)?;
                    nameen = AbsynUtil::joinPaths(inPath.clone(), metamodelica::Ref::new(Absyn::Path::IDENT { name: sn.clone() }))?;
                    Ok(DAEUtil::setMinMax(inVarAttr.clone(), inMin.clone(), Some(metamodelica::Ref::new(DAE::Exp::ENUM_LITERAL { name: nameen.clone(), index: i })))?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(inVarAttr.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    outVarAttr
}

// protected function fixParameterStartBinding
//   input Option<DAE.Exp> bind;
//   input DAE.Type ty;
//   input Option<DAE.VariableAttributes> attr;
//   input BackendDAE.VarKind kind;
//   output Option<DAE.Exp> outBind;
// algorithm
//   outBind := matchcontinue (bind, ty, attr, kind)
//     local
//       DAE.Exp exp;
//     case (NONE(), DAE.T_REAL(source=_), _, BackendDAE.PARAM())
//       equation
//         exp = DAEUtil.getStartAttr(attr);
//       then SOME(exp);
//     else bind;
//   end matchcontinue;
// end fixParameterStartBinding;
fn lowerVarkind(
    mut inVarKind: DAE::VarKind,
    mut inType: &metamodelica::Ref<DAE::Type>,
    mut inComponentRef: &metamodelica::Ref<DAE::ComponentRef>,
    mut inVarDirection: DAE::VarDirection,
    mut inConnectorType: &metamodelica::Ref<DAE::ConnectorType>,
    mut daeAttr: Option<metamodelica::Ref<DAE::VariableAttributes>>,
    mut protection: DAE::VarVisibility,
) -> Result<BackendDAE::VarKind> {
    let mut outVarKind: BackendDAE::VarKind;
    outVarKind = (::match_deref::match_deref! { match &((inVarKind, daeAttr)) {
        (DAE::VarKind::VARIABLE { .. }, Some(Deref @ DAE::VariableAttributes::VAR_ATTR_REAL { stateSelectOption: Some(DAE::StateSelect::ALWAYS { .. }), .. })) if (!(Types::isDiscreteType(inType))) => BackendDAE::VarKind::STATE { index: 1, derName: None, natural: false },
        (DAE::VarKind::VARIABLE { .. }, Some(Deref @ DAE::VariableAttributes::VAR_ATTR_REAL { stateSelectOption: Some(DAE::StateSelect::PREFER { .. }), .. })) if (!(Types::isDiscreteType(inType))) => BackendDAE::VarKind::STATE { index: 1, derName: None, natural: false },
        _ => {
            let false = (ConnectUtil::topLevelInput(inComponentRef, inVarDirection, inConnectorType, protection)?) else { return Err("pattern mismatch") };
            (::match_deref::match_deref! { match &((inVarKind, &**inType)) {
        (DAE::VarKind::VARIABLE { .. }, Deref @ DAE::Type::T_BOOL { .. }) => openmodelica_backend_types::BackendDAE::VarKind::DISCRETE,
        (DAE::VarKind::VARIABLE { .. }, Deref @ DAE::Type::T_INTEGER { .. }) => openmodelica_backend_types::BackendDAE::VarKind::DISCRETE,
        (DAE::VarKind::VARIABLE { .. }, Deref @ DAE::Type::T_ENUMERATION { .. }) => openmodelica_backend_types::BackendDAE::VarKind::DISCRETE,
        (DAE::VarKind::VARIABLE { .. }, _) => openmodelica_backend_types::BackendDAE::VarKind::VARIABLE,
        (DAE::VarKind::DISCRETE { .. }, _) => openmodelica_backend_types::BackendDAE::VarKind::DISCRETE,
        _ => return Err("match: no arm matched"),
    } })
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outVarKind)
}

fn lowerKnownVarkind(
    mut varKind: DAE::VarKind,
    mut componentRef: &metamodelica::Ref<DAE::ComponentRef>,
    mut varDirection: DAE::VarDirection,
    mut connectorType: &metamodelica::Ref<DAE::ConnectorType>,
    mut visibility: DAE::VarVisibility,
) -> Result<BackendDAE::VarKind> {
    let mut outVarKind: BackendDAE::VarKind;
    outVarKind = 'mc: {
        let __mc_input = varKind;
        if let Ok(__v) = (|| -> Result<_> {
            let DAE::VarKind::PARAM { .. } = __mc_input.clone() else {
                return Err("nomatch");
            };
            Ok(openmodelica_backend_types::BackendDAE::VarKind::PARAM)
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let DAE::VarKind::CONST { .. } = __mc_input.clone() else {
                return Err("nomatch");
            };
            Ok(openmodelica_backend_types::BackendDAE::VarKind::CONST)
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let DAE::VarKind::VARIABLE { .. } = __mc_input.clone() else {
                return Err("nomatch");
            };
            let true = (ConnectUtil::topLevelInput(componentRef, varDirection, connectorType, visibility)?) else {
                return Err("pattern mismatch");
            };
            Ok(openmodelica_backend_types::BackendDAE::VarKind::VARIABLE)
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            Error::addInternalError(
                literal!("function lowerKnownVarkind failed"),
                metamodelica::sourceInfo!("BackEnd/BackendDAECreate.mo"),
            )?;
            Ok(return Err("fail"))
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(outVarKind)
}

fn lowerType(mut inType: metamodelica::Ref<DAE::Type>) -> Result<metamodelica::Ref<DAE::Type>> {
    let mut outType: metamodelica::Ref<DAE::Type>;
    outType = (match &*inType {
        DAE::Type::T_REAL { .. } => DAE::T_REAL_DEFAULT().clone(),
        DAE::Type::T_INTEGER { .. } => DAE::T_INTEGER_DEFAULT().clone(),
        DAE::Type::T_BOOL { .. } => DAE::T_BOOL_DEFAULT().clone(),
        DAE::Type::T_STRING { .. } => DAE::T_STRING_DEFAULT().clone(),
        DAE::Type::T_CLOCK { .. } => DAE::T_CLOCK_DEFAULT().clone(),
        DAE::Type::T_ENUMERATION { .. } => inType,
        DAE::Type::T_COMPLEX {
            complexClassType: ClassInf::State::EXTERNAL_OBJ { .. },
            ..
        } => inType,
        DAE::Type::T_COMPLEX {
            complexClassType: ClassInf::State::RECORD { .. },
            ..
        } => inType,
        DAE::Type::T_ARRAY { .. } => inType,
        DAE::Type::T_FUNCTION { .. } => inType,
        _ => {
            metamodelica::print({
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("lowerType: "));
                __mm_s.push_str(&*TypesDump::printTypeStr(inType));
                __mm_s.push_str(&*literal!(" failed\n"));
                ArcStr::from(__mm_s)
            });
            return Err("fail");
        }
    });
    Ok(outType)
}

fn lowerExtObjVar(
    mut inElement: &metamodelica::Ref<DAE::Element>,
    mut functionTree: &metamodelica::Ref<AvlTreePathFunction::Tree>,
) -> Result<metamodelica::Ref<BackendDAE::Var>> {
    let mut outVar: metamodelica::Ref<BackendDAE::Var>;
    outVar = (match &**inElement {
        DAE::Element::VAR {
            componentRef: name,
            direction: dir,
            parallelism: prl,
            ty: t,
            binding: bind,
            dims,
            connectorType: ct,
            source,
            variableAttributesOption: dae_var_attr,
            comment,
            innerOuter: io,
            encrypted,
            ..
        } => {
            let mut kind_1: BackendDAE::VarKind;
            let mut tp: metamodelica::Ref<DAE::Type>;
            let mut ts: Option<BackendDAE::TearingSelect>;
            let mut hideResult: Option<metamodelica::Ref<DAE::Exp>>;
            kind_1 = lowerExtObjVarkind(t)?;
            tp = lowerType(t.clone())?;
            ts = None;
            hideResult = None;
            metamodelica::Ref::new(BackendDAE::Var {
                varName: name.clone(),
                varKind: kind_1,
                varDirection: dir.clone(),
                varParallelism: prl.clone(),
                varType: tp,
                bindExp: bind.clone(),
                tplExp: None,
                arryDim: dims.clone(),
                source: source.clone(),
                values: dae_var_attr.clone(),
                tearingSelectOption: ts,
                hideResult: hideResult,
                comment: comment.clone(),
                connectorType: ct.clone(),
                innerOuter: DAEUtil::toDAEInnerOuter(io.clone()),
                unreplaceable: false,
                initNonlinear: false,
                encrypted: encrypted.clone(),
            })
        }
        _ => return Err("match: no arm matched"),
    });
    Ok(outVar)
}

fn lowerExtObjVarkind(mut inType: &metamodelica::Ref<DAE::Type>) -> Result<BackendDAE::VarKind> {
    let mut outVarKind: BackendDAE::VarKind;
    let mut path: metamodelica::Ref<Absyn::Path>;
    let __pa0 = ::match_deref::match_deref! { match &((*inType)) {
        Deref @ DAE::Type::T_COMPLEX { complexClassType: ClassInf::State::EXTERNAL_OBJ { path: __pa0 }, .. } => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    path = metamodelica::Own::own(__pa0);
    outVarKind = BackendDAE::VarKind::EXTOBJ { fullClassName: path };
    Ok(outVarKind)
}

/*
 *  lower all equation types
 */
fn lowerEqn(
    mut inElement: metamodelica::Ref<DAE::Element>,
    mut functionTree: metamodelica::Ref<AvlTreePathFunction::Tree>,
    mut inEquations: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
    mut inREquations: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
    mut inIEquations: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
    mut inInitialization: bool,
) -> Result<(
    metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
    metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
    metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
)> {
    let mut outEquations: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
    let mut outREquations: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
    let mut outIEquations: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
    (outEquations, outREquations, outIEquations) = (::match_deref::match_deref! { match &(inElement.clone()) {
        Deref @ DAE::Element::EQUATION { exp: Deref @ DAE::Exp::TUPLE { PR: explst }, scalar: Deref @ DAE::Exp::TUPLE { PR: explst1 }, source } => {
            let mut eqns: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
            eqns = lowerTupleAssignment(metamodelica::AsArg::as_arg(&explst), metamodelica::AsArg::as_arg(&explst1), source.clone(), &functionTree, inEquations)?;
            (eqns, inREquations, inIEquations)
        },
        Deref @ DAE::Element::INITIALEQUATION { exp1: Deref @ DAE::Exp::TUPLE { PR: explst }, exp2: Deref @ DAE::Exp::TUPLE { PR: explst1 }, source } => {
            let mut eqns: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
            eqns = lowerTupleAssignment(metamodelica::AsArg::as_arg(&explst), metamodelica::AsArg::as_arg(&explst1), source.clone(), &functionTree, inIEquations)?;
            (inEquations, inREquations, eqns)
        },
        Deref @ DAE::Element::EQUATION { exp: e1 @ Deref @ DAE::Exp::TUPLE { PR: _ }, scalar: e2 @ Deref @ DAE::Exp::CALL { .. }, source } => {
            let mut e1_1: metamodelica::Ref<DAE::Exp>;
            let mut e2_1: metamodelica::Ref<DAE::Exp>;
            let mut eqns: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
            let mut source = (*source).clone();
            let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(ExpressionSimplify::simplifyAddSymbolicOperation(metamodelica::Ref::new(DAE::EquationExp::EQUALITY_EXPS { lhs: e1.clone(), rhs: e2.clone() }), source.clone())?) {
                (Deref @ DAE::EquationExp::EQUALITY_EXPS { lhs: __pa0, rhs: __pa1 }, __pa2) => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
                _ => return Err("pattern mismatch"),
            } };
            e1_1 = metamodelica::Own::own(__pa0);
            e2_1 = metamodelica::Own::own(__pa1);
            source = metamodelica::Own::own(__pa2);
            eqns = lowerExtendedRecordEqn(e1_1, e2_1, source.clone(), BackendDAE::EQ_ATTR_DEFAULT_DYNAMIC.clone(), &functionTree, inEquations)?;
            (eqns, inREquations, inIEquations)
        },
        Deref @ DAE::Element::EQUATION { exp: e2 @ Deref @ DAE::Exp::CALL { .. }, scalar: e1 @ Deref @ DAE::Exp::TUPLE { PR: _ }, source } => {
            let mut e1_1: metamodelica::Ref<DAE::Exp>;
            let mut e2_1: metamodelica::Ref<DAE::Exp>;
            let mut eqns: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
            let mut source = (*source).clone();
            let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(ExpressionSimplify::simplifyAddSymbolicOperation(metamodelica::Ref::new(DAE::EquationExp::EQUALITY_EXPS { lhs: e1.clone(), rhs: e2.clone() }), source.clone())?) {
                (Deref @ DAE::EquationExp::EQUALITY_EXPS { lhs: __pa0, rhs: __pa1 }, __pa2) => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
                _ => return Err("pattern mismatch"),
            } };
            e1_1 = metamodelica::Own::own(__pa0);
            e2_1 = metamodelica::Own::own(__pa1);
            source = metamodelica::Own::own(__pa2);
            eqns = lowerExtendedRecordEqn(e1_1, e2_1, source.clone(), BackendDAE::EQ_ATTR_DEFAULT_DYNAMIC.clone(), &functionTree, inEquations)?;
            (eqns, inREquations, inIEquations)
        },
        Deref @ DAE::Element::INITIALEQUATION { exp1: e1 @ Deref @ DAE::Exp::TUPLE { PR: _ }, exp2: e2 @ Deref @ DAE::Exp::CALL { .. }, source } => {
            let mut e1_1: metamodelica::Ref<DAE::Exp>;
            let mut e2_1: metamodelica::Ref<DAE::Exp>;
            let mut eqns: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
            let mut source = (*source).clone();
            let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(ExpressionSimplify::simplifyAddSymbolicOperation(metamodelica::Ref::new(DAE::EquationExp::EQUALITY_EXPS { lhs: e1.clone(), rhs: e2.clone() }), source.clone())?) {
                (Deref @ DAE::EquationExp::EQUALITY_EXPS { lhs: __pa0, rhs: __pa1 }, __pa2) => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
                _ => return Err("pattern mismatch"),
            } };
            e1_1 = metamodelica::Own::own(__pa0);
            e2_1 = metamodelica::Own::own(__pa1);
            source = metamodelica::Own::own(__pa2);
            eqns = lowerExtendedRecordEqn(e1_1, e2_1, source.clone(), BackendDAE::EQ_ATTR_DEFAULT_INITIAL.clone(), &functionTree, inIEquations)?;
            (inEquations, inREquations, eqns)
        },
        Deref @ DAE::Element::INITIALEQUATION { exp1: e2 @ Deref @ DAE::Exp::CALL { .. }, exp2: e1 @ Deref @ DAE::Exp::TUPLE { PR: _ }, source } => {
            let mut e1_1: metamodelica::Ref<DAE::Exp>;
            let mut e2_1: metamodelica::Ref<DAE::Exp>;
            let mut eqns: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
            let mut source = (*source).clone();
            let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(ExpressionSimplify::simplifyAddSymbolicOperation(metamodelica::Ref::new(DAE::EquationExp::EQUALITY_EXPS { lhs: e1.clone(), rhs: e2.clone() }), source.clone())?) {
                (Deref @ DAE::EquationExp::EQUALITY_EXPS { lhs: __pa0, rhs: __pa1 }, __pa2) => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
                _ => return Err("pattern mismatch"),
            } };
            e1_1 = metamodelica::Own::own(__pa0);
            e2_1 = metamodelica::Own::own(__pa1);
            source = metamodelica::Own::own(__pa2);
            eqns = lowerExtendedRecordEqn(e1_1, e2_1, source.clone(), BackendDAE::EQ_ATTR_DEFAULT_INITIAL.clone(), &functionTree, inIEquations)?;
            (inEquations, inREquations, eqns)
        },
        Deref @ DAE::Element::EQUATION { exp: e1, scalar: e2, source } => {
            let mut e1_1: metamodelica::Ref<DAE::Exp>;
            let mut e2_1: metamodelica::Ref<DAE::Exp>;
            let mut source = (*source).clone();
            let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(ExpressionSimplify::simplifyAddSymbolicOperation(metamodelica::Ref::new(DAE::EquationExp::EQUALITY_EXPS { lhs: e1.clone(), rhs: e2.clone() }), source.clone())?) {
                (Deref @ DAE::EquationExp::EQUALITY_EXPS { lhs: __pa0, rhs: __pa1 }, __pa2) => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
                _ => return Err("pattern mismatch"),
            } };
            e1_1 = metamodelica::Own::own(__pa0);
            e2_1 = metamodelica::Own::own(__pa1);
            source = metamodelica::Own::own(__pa2);
            (metamodelica::cons(metamodelica::Ref::new(BackendDAE::Equation::EQUATION { exp: e1_1, scalar: e2_1, source: source.clone(), attr: BackendDAE::EQ_ATTR_DEFAULT_DYNAMIC.clone() }), inEquations), inREquations, inIEquations)
        },
        Deref @ DAE::Element::INITIALEQUATION { exp1: e1, exp2: e2, source } => {
            let mut e1_1: metamodelica::Ref<DAE::Exp>;
            let mut e2_1: metamodelica::Ref<DAE::Exp>;
            let mut source = (*source).clone();
            let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(ExpressionSimplify::simplifyAddSymbolicOperation(metamodelica::Ref::new(DAE::EquationExp::EQUALITY_EXPS { lhs: e1.clone(), rhs: e2.clone() }), source.clone())?) {
                (Deref @ DAE::EquationExp::EQUALITY_EXPS { lhs: __pa0, rhs: __pa1 }, __pa2) => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
                _ => return Err("pattern mismatch"),
            } };
            e1_1 = metamodelica::Own::own(__pa0);
            e2_1 = metamodelica::Own::own(__pa1);
            source = metamodelica::Own::own(__pa2);
            (inEquations, inREquations, metamodelica::cons(metamodelica::Ref::new(BackendDAE::Equation::EQUATION { exp: e1_1, scalar: e2_1, source: source.clone(), attr: BackendDAE::EQ_ATTR_DEFAULT_INITIAL.clone() }), inIEquations))
        },
        Deref @ DAE::Element::EQUEQUATION { cr1, cr2, source } => {
            let mut e1: metamodelica::Ref<DAE::Exp>;
            let mut e2: metamodelica::Ref<DAE::Exp>;
            let mut eqns: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
            if Flags::isSet(Flags::NF_SCALARIZE.clone())? {
                e1 = Expression::crefExp(cr1.clone())?;
                e2 = Expression::crefExp(cr2.clone())?;
            } else {
                e1 = Expression::crefToExp(cr1.clone())?;
                e2 = Expression::crefToExp(cr2.clone())?;
            }
            eqns = lowerExtendedRecordEqn(e1, e2, source.clone(), BackendDAE::EQ_ATTR_DEFAULT_DYNAMIC.clone(), &functionTree, inEquations)?;
            (eqns, inREquations, inIEquations)
        },
        Deref @ DAE::Element::DEFINE { componentRef: cr1, exp: e2, source } => {
            let mut e1: metamodelica::Ref<DAE::Exp>;
            let mut e1_1: metamodelica::Ref<DAE::Exp>;
            let mut e2_1: metamodelica::Ref<DAE::Exp>;
            let mut source = (*source).clone();
            e1 = Expression::crefExp(cr1.clone())?;
            let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(ExpressionSimplify::simplifyAddSymbolicOperation(metamodelica::Ref::new(DAE::EquationExp::EQUALITY_EXPS { lhs: e1, rhs: e2.clone() }), source.clone())?) {
                (Deref @ DAE::EquationExp::EQUALITY_EXPS { lhs: __pa0, rhs: __pa1 }, __pa2) => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
                _ => return Err("pattern mismatch"),
            } };
            e1_1 = metamodelica::Own::own(__pa0);
            e2_1 = metamodelica::Own::own(__pa1);
            source = metamodelica::Own::own(__pa2);
            (metamodelica::cons(metamodelica::Ref::new(BackendDAE::Equation::EQUATION { exp: e1_1, scalar: e2_1, source: source.clone(), attr: BackendDAE::EQ_ATTR_DEFAULT_DYNAMIC.clone() }), inEquations), inREquations, inIEquations)
        },
        Deref @ DAE::Element::INITIALDEFINE { componentRef: cr1, exp: e2, source } => {
            let mut e1: metamodelica::Ref<DAE::Exp>;
            let mut e1_1: metamodelica::Ref<DAE::Exp>;
            let mut e2_1: metamodelica::Ref<DAE::Exp>;
            let mut source = (*source).clone();
            e1 = Expression::crefExp(cr1.clone())?;
            let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(ExpressionSimplify::simplifyAddSymbolicOperation(metamodelica::Ref::new(DAE::EquationExp::EQUALITY_EXPS { lhs: e1, rhs: e2.clone() }), source.clone())?) {
                (Deref @ DAE::EquationExp::EQUALITY_EXPS { lhs: __pa0, rhs: __pa1 }, __pa2) => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
                _ => return Err("pattern mismatch"),
            } };
            e1_1 = metamodelica::Own::own(__pa0);
            e2_1 = metamodelica::Own::own(__pa1);
            source = metamodelica::Own::own(__pa2);
            (inEquations, inREquations, metamodelica::cons(metamodelica::Ref::new(BackendDAE::Equation::EQUATION { exp: e1_1, scalar: e2_1, source: source.clone(), attr: BackendDAE::EQ_ATTR_DEFAULT_DYNAMIC.clone() }), inIEquations))
        },
        Deref @ DAE::Element::COMPLEX_EQUATION { lhs: e1, rhs: e2, source } => {
            let mut e1_1: metamodelica::Ref<DAE::Exp>;
            let mut e2_1: metamodelica::Ref<DAE::Exp>;
            let mut eqns: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
            let mut source = (*source).clone();
            let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(Inline::simplifyAndForceInlineEquationExp(metamodelica::Ref::new(DAE::EquationExp::EQUALITY_EXPS { lhs: e1.clone(), rhs: e2.clone() }), &((Some(functionTree.clone()), list![openmodelica_frontend_types::DAE::InlineType::NORM_INLINE, openmodelica_frontend_types::DAE::InlineType::DEFAULT_INLINE])), source.clone())?) {
                (Deref @ DAE::EquationExp::EQUALITY_EXPS { lhs: __pa0, rhs: __pa1 }, __pa2) => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
                _ => return Err("pattern mismatch"),
            } };
            e1_1 = metamodelica::Own::own(__pa0);
            e2_1 = metamodelica::Own::own(__pa1);
            source = metamodelica::Own::own(__pa2);
            eqns = lowerExtendedRecordEqn(e1_1, e2_1, source.clone(), BackendDAE::EQ_ATTR_DEFAULT_DYNAMIC.clone(), &functionTree, inEquations)?;
            (eqns, inREquations, inIEquations)
        },
        Deref @ DAE::Element::INITIAL_COMPLEX_EQUATION { lhs: e1, rhs: e2, source } => {
            let mut e1_1: metamodelica::Ref<DAE::Exp>;
            let mut e2_1: metamodelica::Ref<DAE::Exp>;
            let mut eqns: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
            let mut source = (*source).clone();
            let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(Inline::simplifyAndForceInlineEquationExp(metamodelica::Ref::new(DAE::EquationExp::EQUALITY_EXPS { lhs: e1.clone(), rhs: e2.clone() }), &((Some(functionTree.clone()), list![openmodelica_frontend_types::DAE::InlineType::NORM_INLINE, openmodelica_frontend_types::DAE::InlineType::DEFAULT_INLINE])), source.clone())?) {
                (Deref @ DAE::EquationExp::EQUALITY_EXPS { lhs: __pa0, rhs: __pa1 }, __pa2) => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
                _ => return Err("pattern mismatch"),
            } };
            e1_1 = metamodelica::Own::own(__pa0);
            e2_1 = metamodelica::Own::own(__pa1);
            source = metamodelica::Own::own(__pa2);
            eqns = lowerExtendedRecordEqn(e1_1, e2_1, source.clone(), BackendDAE::EQ_ATTR_DEFAULT_DYNAMIC.clone(), &functionTree, inIEquations)?;
            (inEquations, inREquations, eqns)
        },
        Deref @ DAE::Element::ARRAY_EQUATION { dimension: dims, exp: e1 @ Deref @ DAE::Exp::ARRAY { array: Deref @ metamodelica::ListNode::Nil, .. }, array: e2 @ Deref @ DAE::Exp::CALL { path, .. }, source } => {
            let mut e1_1: metamodelica::Ref<DAE::Exp>;
            let mut e2_1: metamodelica::Ref<DAE::Exp>;
            let mut b1: bool;
            let mut eqns: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
            let mut source = (*source).clone();
            let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(ExpressionSimplify::simplifyAddSymbolicOperation(metamodelica::Ref::new(DAE::EquationExp::EQUALITY_EXPS { lhs: e1.clone(), rhs: e2.clone() }), source.clone())?) {
                (Deref @ DAE::EquationExp::EQUALITY_EXPS { lhs: __pa0, rhs: __pa1 }, __pa2) => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
                _ => return Err("pattern mismatch"),
            } };
            e1_1 = metamodelica::Own::own(__pa0);
            e2_1 = metamodelica::Own::own(__pa1);
            source = metamodelica::Own::own(__pa2);
            b1 = stringEq(&(AbsynUtil::pathLastIdent(metamodelica::AsArg::as_arg(&path))), &(literal!("equalityConstraint")));
            eqns = if (b1) {inREquations.clone()} else {inEquations.clone()};
            eqns = lowerArrayEqn(dims.clone(), e1_1, e2_1, source.clone(), BackendDAE::EQ_ATTR_DEFAULT_DYNAMIC.clone(), eqns)?;
            (eqns, _) = if (b1) {(inEquations, eqns)} else {(eqns, inREquations.clone())};
            (eqns, inREquations, inIEquations)
        },
        Deref @ DAE::Element::ARRAY_EQUATION { dimension: dims, exp: e1, array: e2, source } => {
            let mut e1_1: metamodelica::Ref<DAE::Exp>;
            let mut e2_1: metamodelica::Ref<DAE::Exp>;
            let mut eqns: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
            let mut source = (*source).clone();
            let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(ExpressionSimplify::simplifyAddSymbolicOperation(metamodelica::Ref::new(DAE::EquationExp::EQUALITY_EXPS { lhs: e1.clone(), rhs: e2.clone() }), source.clone())?) {
                (Deref @ DAE::EquationExp::EQUALITY_EXPS { lhs: __pa0, rhs: __pa1 }, __pa2) => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
                _ => return Err("pattern mismatch"),
            } };
            e1_1 = metamodelica::Own::own(__pa0);
            e2_1 = metamodelica::Own::own(__pa1);
            source = metamodelica::Own::own(__pa2);
            eqns = lowerArrayEqn(dims.clone(), e1_1, e2_1, source.clone(), BackendDAE::EQ_ATTR_DEFAULT_DYNAMIC.clone(), inEquations)?;
            (eqns, inREquations, inIEquations)
        },
        Deref @ DAE::Element::INITIAL_ARRAY_EQUATION { dimension: dims, exp: e1, array: e2, source } => {
            let mut e1_1: metamodelica::Ref<DAE::Exp>;
            let mut e2_1: metamodelica::Ref<DAE::Exp>;
            let mut eqns: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
            let mut source = (*source).clone();
            let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(ExpressionSimplify::simplifyAddSymbolicOperation(metamodelica::Ref::new(DAE::EquationExp::EQUALITY_EXPS { lhs: e1.clone(), rhs: e2.clone() }), source.clone())?) {
                (Deref @ DAE::EquationExp::EQUALITY_EXPS { lhs: __pa0, rhs: __pa1 }, __pa2) => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
                _ => return Err("pattern mismatch"),
            } };
            e1_1 = metamodelica::Own::own(__pa0);
            e2_1 = metamodelica::Own::own(__pa1);
            source = metamodelica::Own::own(__pa2);
            eqns = lowerArrayEqn(dims.clone(), e1_1, e2_1, source.clone(), BackendDAE::EQ_ATTR_DEFAULT_DYNAMIC.clone(), inIEquations)?;
            (inEquations, inREquations, eqns)
        },
        Deref @ DAE::Element::FOR_EQUATION { iter: s, range: e1, equations: eqnslst, .. } => {
            let mut eqns: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
            let mut reqns: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
            let mut ieqns: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
            (eqns, reqns, ieqns) = lowerEqns(metamodelica::AsArg::as_arg(&eqnslst), &functionTree, metamodelica::nil(), metamodelica::nil(), metamodelica::nil(), inInitialization)?;
            eqns = listAppend(List::map2(eqns, &move |__a0: metamodelica::Ref<BackendDAE::Equation>, __a1: ArcStr, __a2: metamodelica::Ref<DAE::Exp>| lowerForEquation(__a0, __a1, &__a2), s.clone(), e1.clone())?, inEquations);
            reqns = listAppend(List::map2(reqns, &move |__a0: metamodelica::Ref<BackendDAE::Equation>, __a1: ArcStr, __a2: metamodelica::Ref<DAE::Exp>| lowerForEquation(__a0, __a1, &__a2), s.clone(), e1.clone())?, inREquations);
            ieqns = listAppend(List::map2(ieqns, &move |__a0: metamodelica::Ref<BackendDAE::Equation>, __a1: ArcStr, __a2: metamodelica::Ref<DAE::Exp>| lowerForEquation(__a0, __a1, &__a2), s.clone(), e1.clone())?, inIEquations);
            (eqns, reqns, ieqns)
        },
        Deref @ DAE::Element::INITIAL_FOR_EQUATION { iter: s, range: e1, equations: eqnslst, .. } => {
            let mut eqns: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
            let mut reqns: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
            let mut ieqns: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
            (eqns, reqns, ieqns) = lowerEqns(metamodelica::AsArg::as_arg(&eqnslst), &functionTree, metamodelica::nil(), metamodelica::nil(), metamodelica::nil(), inInitialization)?;
            eqns = listAppend(List::map2(eqns, &move |__a0: metamodelica::Ref<BackendDAE::Equation>, __a1: ArcStr, __a2: metamodelica::Ref<DAE::Exp>| lowerForEquation(__a0, __a1, &__a2), s.clone(), e1.clone())?, inEquations);
            reqns = listAppend(List::map2(reqns, &move |__a0: metamodelica::Ref<BackendDAE::Equation>, __a1: ArcStr, __a2: metamodelica::Ref<DAE::Exp>| lowerForEquation(__a0, __a1, &__a2), s.clone(), e1.clone())?, inREquations);
            ieqns = listAppend(List::map2(ieqns, &move |__a0: metamodelica::Ref<BackendDAE::Equation>, __a1: ArcStr, __a2: metamodelica::Ref<DAE::Exp>| lowerForEquation(__a0, __a1, &__a2), s.clone(), e1.clone())?, inIEquations);
            (eqns, reqns, ieqns)
        },
        Deref @ DAE::Element::IF_EQUATION { condition1: Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "initial" }, .. }, tail: Deref @ metamodelica::ListNode::Nil }, equations2: Deref @ metamodelica::ListNode::Cons { head: eqnslst, tail: Deref @ metamodelica::ListNode::Nil }, equations3: Deref @ metamodelica::ListNode::Nil, .. } => {
            let mut eqns: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
            let mut reqns: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
            let mut ieqns: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
            (eqns, reqns, ieqns) = lowerEqns(metamodelica::AsArg::as_arg(&eqnslst), &functionTree, metamodelica::nil(), metamodelica::nil(), metamodelica::nil(), inInitialization)?;
            ieqns = List::flatten(list![eqns, reqns, ieqns, inIEquations])?;
            (inEquations, inREquations, ieqns)
        },
        Deref @ DAE::Element::IF_EQUATION { condition1: explst, equations2: eqnslstlst, equations3: eqnslst, source } => {
            let mut daeElts: metamodelica::List<metamodelica::Ref<DAE::Element>>;
            let mut eqns: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
            let mut reqns: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
            let mut ieqns: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
            let mut eqnslstlst = (*eqnslstlst).clone();
            let mut eqnslst = (*eqnslst).clone();
            (eqnslstlst, eqnslst, daeElts) = lowerIfEquationAsserts(metamodelica::AsArg::as_arg(&explst), metamodelica::AsArg::as_arg(&eqnslstlst), metamodelica::AsArg::as_arg(&eqnslst), metamodelica::nil(), metamodelica::nil(), metamodelica::nil())?;
            (eqns, reqns, ieqns) = lowerEqns(&daeElts, &functionTree, inEquations, inREquations, inIEquations, inInitialization)?;
            eqns = lowerIfEquation(metamodelica::AsArg::as_arg(&explst), metamodelica::AsArg::as_arg(&eqnslstlst), metamodelica::AsArg::as_arg(&eqnslst), metamodelica::nil(), metamodelica::nil(), source.clone(), &functionTree, eqns)?;
            (eqns, reqns, ieqns)
        },
        Deref @ DAE::Element::INITIAL_IF_EQUATION { condition1: explst, equations2: eqnslstlst, equations3: eqnslst, source } => {
            let mut eqns: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
            eqns = lowerIfEquation(metamodelica::AsArg::as_arg(&explst), metamodelica::AsArg::as_arg(&eqnslstlst), metamodelica::AsArg::as_arg(&eqnslst), metamodelica::nil(), metamodelica::nil(), source.clone(), &functionTree, inIEquations)?;
            (inEquations, inREquations, eqns)
        },
        Deref @ DAE::Element::ALGORITHM { .. } => {
            let mut eqns: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
            let mut reqns: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
            let mut ieqns: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
            (eqns, reqns, ieqns) = lowerAlgorithm(inElement, &functionTree, inEquations, inREquations, inIEquations, openmodelica_frontend_types::DAE::Expand::EXPAND, false)?;
            (eqns, reqns, ieqns)
        },
        Deref @ DAE::Element::INITIALALGORITHM { .. } => {
            let mut eqns: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
            let mut reqns: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
            let mut ieqns: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
            (eqns, reqns, ieqns) = lowerAlgorithm(inElement, &functionTree, inEquations, inREquations, inIEquations, openmodelica_frontend_types::DAE::Expand::EXPAND, true)?;
            (eqns, reqns, ieqns)
        },
        Deref @ DAE::Element::ASSERT { .. } => {
            let mut eqns: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
            let mut reqns: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
            let mut ieqns: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
            (eqns, reqns, ieqns) = lowerAlgorithm(inElement, &functionTree, inEquations, inREquations, inIEquations, openmodelica_frontend_types::DAE::Expand::NOT_EXPAND, inInitialization)?;
            (eqns, reqns, ieqns)
        },
        Deref @ DAE::Element::INITIAL_ASSERT { .. } => {
            let mut eqns: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
            let mut reqns: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
            let mut ieqns: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
            (eqns, reqns, ieqns) = lowerAlgorithm(inElement, &functionTree, inEquations, inREquations, inIEquations, openmodelica_frontend_types::DAE::Expand::NOT_EXPAND, inInitialization)?;
            (eqns, reqns, ieqns)
        },
        Deref @ DAE::Element::TERMINATE { message: msg, source } => {
            (inEquations, metamodelica::cons(metamodelica::Ref::new(BackendDAE::Equation::ALGORITHM { size: 0, alg: metamodelica::Ref::new(DAE::Algorithm { statementLst: list![metamodelica::Ref::new(DAE::Statement::STMT_TERMINATE { msg: msg.clone(), source: source.clone() })] }), source: source.clone(), expand: openmodelica_frontend_types::DAE::Expand::NOT_EXPAND, attr: BackendDAE::EQ_ATTR_DEFAULT_DYNAMIC.clone() }), inREquations), inIEquations)
        },
        Deref @ DAE::Element::INITIAL_TERMINATE { message: msg, source } => {
            (inEquations, inREquations, metamodelica::cons(metamodelica::Ref::new(BackendDAE::Equation::ALGORITHM { size: 0, alg: metamodelica::Ref::new(DAE::Algorithm { statementLst: list![metamodelica::Ref::new(DAE::Statement::STMT_TERMINATE { msg: msg.clone(), source: source.clone() })] }), source: source.clone(), expand: openmodelica_frontend_types::DAE::Expand::NOT_EXPAND, attr: BackendDAE::EQ_ATTR_DEFAULT_DYNAMIC.clone() }), inIEquations))
        },
        Deref @ DAE::Element::NORETCALL { .. } => {
            let mut eqns: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
            let mut reqns: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
            let mut ieqns: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
            (eqns, reqns, ieqns) = lowerAlgorithm(inElement, &functionTree, inEquations, inREquations, inIEquations, openmodelica_frontend_types::DAE::Expand::NOT_EXPAND, false)?;
            (eqns, reqns, ieqns)
        },
        Deref @ DAE::Element::INITIAL_NORETCALL { .. } => {
            let mut eqns: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
            let mut reqns: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
            let mut ieqns: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
            (eqns, reqns, ieqns) = lowerAlgorithm(inElement, &functionTree, inEquations, inREquations, inIEquations, openmodelica_frontend_types::DAE::Expand::NOT_EXPAND, true)?;
            (eqns, reqns, ieqns)
        },
        _ => {
            let mut s: ArcStr;
            s = { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("BackendDAECreate.lowerEqn failed for ")); __mm_s.push_str(&*DAEDump::dumpElementsStr(&(list![inElement.clone()]))?); ArcStr::from(__mm_s) };
            Error::addSourceMessage(&(Error::INTERNAL_ERROR.clone()), list![s], &(ElementSource::getElementSourceFileInfo(ElementSource::getElementSource(&inElement)?)))?;
            return Err("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outEquations, outREquations, outIEquations))
}

fn lowerForEquation(
    mut eq: metamodelica::Ref<BackendDAE::Equation>,
    mut iter: ArcStr,
    mut range: &metamodelica::Ref<DAE::Exp>,
) -> Result<metamodelica::Ref<BackendDAE::Equation>> {
    let mut forEq: metamodelica::Ref<BackendDAE::Equation>;
    let mut iterExp: metamodelica::Ref<DAE::Exp>;
    let mut start: metamodelica::Ref<DAE::Exp>;
    let mut stop: metamodelica::Ref<DAE::Exp>;
    let mut ty: metamodelica::Ref<DAE::Type>;
    let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &((*range)) {
        Deref @ DAE::Exp::RANGE { ty: __pa0, start: __pa1, stop: __pa2, .. } => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
        _ => return Err("pattern mismatch"),
    } };
    ty = metamodelica::Own::own(__pa0);
    start = metamodelica::Own::own(__pa1);
    stop = metamodelica::Own::own(__pa2);
    ty = Types::unliftArray(&ty)?;
    iterExp = metamodelica::Ref::new(DAE::Exp::CREF {
        componentRef: metamodelica::Ref::new(DAE::ComponentRef::CREF_IDENT {
            ident: iter,
            identType: ty.clone(),
            subscriptLst: metamodelica::nil(),
        }),
        ty: ty,
    });
    forEq = metamodelica::Ref::new(BackendDAE::Equation::FOR_EQUATION {
        iter: iterExp,
        start: start,
        stop: stop,
        body: eq.clone(),
        source: BackendEquation::equationSource(&eq)?,
        attr: BackendEquation::getEquationAttributes(&eq)?,
    });
    Ok(forEq)
}

fn lowerIfEquation(
    mut conditions: &metamodelica::List<metamodelica::Ref<DAE::Exp>>,
    mut theneqns: &metamodelica::List<metamodelica::List<metamodelica::Ref<DAE::Element>>>,
    mut elseenqs: &metamodelica::List<metamodelica::Ref<DAE::Element>>,
    mut conditions1: metamodelica::List<metamodelica::Ref<DAE::Exp>>,
    mut theneqns1: metamodelica::List<metamodelica::List<metamodelica::Ref<DAE::Element>>>,
    mut inSource: metamodelica::Ref<DAE::ElementSource>,
    mut functionTree: &metamodelica::Ref<AvlTreePathFunction::Tree>,
    mut inEquations: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
) -> Result<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>> {
    let mut outEquations: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
    outEquations = 'mc: {
        let __mc_input = (&**conditions, &**theneqns, &*conditions1, &*theneqns1);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Nil, Deref @ metamodelica::ListNode::Nil, Deref @ metamodelica::ListNode::Nil, Deref @ metamodelica::ListNode::Nil) => {
                    let mut beqns: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
                    let mut breqns: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
                    let mut bieqns: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
                    (beqns, breqns, bieqns) = lowerEqns(elseenqs, functionTree, metamodelica::nil(), metamodelica::nil(), metamodelica::nil(), false)?;
                    beqns = List::flatten(list![beqns.clone(), breqns.clone(), bieqns.clone(), inEquations.clone()])?;
                    Ok(beqns.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Nil, Deref @ metamodelica::ListNode::Nil, _, _) => {
                    let mut explst: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                    let mut beqnslst: metamodelica::List<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>>;
                    let mut beqns: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
                    let mut breqns: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
                    let mut bieqns: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
                    explst = conditions1.clone().reverse();
                    beqnslst = lowerEqnsLst(&theneqns1, functionTree, metamodelica::nil(), false)?;
                    (beqns, breqns, bieqns) = lowerEqns(elseenqs, functionTree, metamodelica::nil(), metamodelica::nil(), metamodelica::nil(), false)?;
                    beqns = List::flatten(list![beqns.clone(), breqns.clone(), bieqns.clone()])?;
                    Ok(metamodelica::cons(metamodelica::Ref::new(BackendDAE::Equation::IF_EQUATION { conditions: explst.clone(), eqnstrue: beqnslst.clone(), eqnsfalse: beqns.clone(), source: inSource.clone(), attr: BackendDAE::EQ_ATTR_DEFAULT_UNKNOWN.clone() }), inEquations.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: e, tail: explst }, Deref @ metamodelica::ListNode::Cons { head: eqns, tail: eqnslst }, _, _) => {
                    let mut source: metamodelica::Ref<DAE::ElementSource>;
                    let mut e = (*e).clone();
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(ExpressionSimplify::simplifyAddSymbolicOperation(metamodelica::Ref::new(DAE::EquationExp::PARTIAL_EQUATION { exp: e.clone() }), inSource.clone())?) {
                        (Deref @ DAE::EquationExp::PARTIAL_EQUATION { exp: __pa0 }, __pa1) => (__pa0.clone(), __pa1.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    e = metamodelica::Own::own(__pa0);
                    source = metamodelica::Own::own(__pa1);
                    Ok(lowerIfEquation1(e.clone(), metamodelica::AsArg::as_arg(&explst), eqns.clone(), metamodelica::AsArg::as_arg(&eqnslst), elseenqs, conditions1.clone(), theneqns1.clone(), source.clone(), functionTree, inEquations.clone())?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(outEquations)
}

fn lowerIfEquation1(
    mut cond: metamodelica::Ref<DAE::Exp>,
    mut conditions: &metamodelica::List<metamodelica::Ref<DAE::Exp>>,
    mut theneqn: metamodelica::List<metamodelica::Ref<DAE::Element>>,
    mut theneqns: &metamodelica::List<metamodelica::List<metamodelica::Ref<DAE::Element>>>,
    mut elseenqs: &metamodelica::List<metamodelica::Ref<DAE::Element>>,
    mut conditions1: metamodelica::List<metamodelica::Ref<DAE::Exp>>,
    mut theneqns1: metamodelica::List<metamodelica::List<metamodelica::Ref<DAE::Element>>>,
    mut source: metamodelica::Ref<DAE::ElementSource>,
    mut functionTree: &metamodelica::Ref<AvlTreePathFunction::Tree>,
    mut inEqns: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
) -> Result<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>> {
    let mut outEqns: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
    outEqns = 'mc: {
        let __mc_input = (&*cond, &*conditions1, &*theneqns1);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::BCONST { bool: true }, Deref @ metamodelica::ListNode::Nil, Deref @ metamodelica::ListNode::Nil) => {
                    let mut beqns: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
                    let mut breqns: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
                    let mut bieqns: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
                    (beqns, breqns, bieqns) = lowerEqns(&theneqn, functionTree, metamodelica::nil(), metamodelica::nil(), metamodelica::nil(), false)?;
                    beqns = List::flatten(list![beqns.clone(), breqns.clone(), bieqns.clone(), inEqns.clone()])?;
                    Ok(beqns.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::BCONST { bool: true }, Deref @ metamodelica::ListNode::Nil, Deref @ metamodelica::ListNode::Nil) => {
                    let mut explst: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                    let mut beqnslst: metamodelica::List<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>>;
                    let mut beqns: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
                    let mut breqns: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
                    let mut bieqns: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
                    explst = conditions1.clone().reverse();
                    beqnslst = lowerEqnsLst(&theneqns1, functionTree, metamodelica::nil(), false)?;
                    (beqns, breqns, bieqns) = lowerEqns(&theneqn, functionTree, metamodelica::nil(), metamodelica::nil(), metamodelica::nil(), false)?;
                    beqns = List::flatten(list![beqns.clone(), breqns.clone(), bieqns.clone()])?;
                    Ok(metamodelica::cons(metamodelica::Ref::new(BackendDAE::Equation::IF_EQUATION { conditions: explst.clone(), eqnstrue: beqnslst.clone(), eqnsfalse: beqns.clone(), source: source.clone(), attr: BackendDAE::EQ_ATTR_DEFAULT_UNKNOWN.clone() }), inEqns.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::BCONST { bool: false }, _, _) => {
                    Ok(lowerIfEquation(conditions, theneqns, elseenqs, conditions1.clone(), theneqns1.clone(), source.clone(), functionTree, inEqns.clone())?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _) => {
                    Ok(lowerIfEquation(conditions, theneqns, elseenqs, metamodelica::cons(cond.clone(), conditions1.clone()), metamodelica::cons(theneqn.clone(), theneqns1.clone()), source.clone(), functionTree, inEqns.clone())?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(outEqns)
}

fn lowerEqns<'__b>(
    mut inElements: &'__b metamodelica::List<metamodelica::Ref<DAE::Element>>,
    mut functionTree: &'__b metamodelica::Ref<AvlTreePathFunction::Tree>,
    mut inEquations: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
    mut inREquations: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
    mut inIEquations: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
    mut inInitialization: bool,
) -> Result<(
    metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
    metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
    metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
)> {
    '__tco: loop {
        ::match_deref::match_deref! { match inElements {
            Deref @ metamodelica::ListNode::Nil => {
                return Ok((inEquations, inREquations, inIEquations))
            },
            Deref @ metamodelica::ListNode::Cons { head: element, tail: elements } => {
                let mut eqns: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
                let mut reqns: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
                let mut ieqns: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
                (eqns, reqns, ieqns) = lowerEqn(element.clone(), functionTree.clone(), inEquations, inREquations, inIEquations, inInitialization)?;
                { (inElements, functionTree, inEquations, inREquations, inIEquations, inInitialization) = (elements, functionTree, eqns, reqns, ieqns, inInitialization); continue '__tco; }
            },
            _ => return Err("match: no arm matched"),
        } }
    }
}

fn lowerEqnsLst<'__b>(
    mut inElements: &'__b metamodelica::List<metamodelica::List<metamodelica::Ref<DAE::Element>>>,
    mut functionTree: &'__b metamodelica::Ref<AvlTreePathFunction::Tree>,
    mut inEquations: metamodelica::List<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>>,
    mut inInitialization: bool,
) -> Result<metamodelica::List<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>>> {
    '__tco: loop {
        ::match_deref::match_deref! { match inElements {
            Deref @ metamodelica::ListNode::Nil => {
                return Ok(inEquations)
            },
            Deref @ metamodelica::ListNode::Cons { head: element, tail: elements } => {
                let mut eqns: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
                let mut reqns: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
                let mut ieqns: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
                (eqns, reqns, ieqns) = lowerEqns(metamodelica::AsArg::as_arg(&element), functionTree, metamodelica::nil(), metamodelica::nil(), metamodelica::nil(), inInitialization)?;
                eqns = List::flatten(list![eqns, reqns, ieqns])?;
                { (inElements, functionTree, inEquations, inInitialization) = (elements, functionTree, metamodelica::cons(eqns, inEquations), inInitialization); continue '__tco; }
            },
            _ => return Err("match: no arm matched"),
        } }
    }
}

fn lowerIfEquationAsserts<'__b>(
    mut conditions: &'__b metamodelica::List<metamodelica::Ref<DAE::Exp>>,
    mut theneqns: &'__b metamodelica::List<metamodelica::List<metamodelica::Ref<DAE::Element>>>,
    mut elseenqs: &'__b metamodelica::List<metamodelica::Ref<DAE::Element>>,
    mut conditions1: metamodelica::List<metamodelica::Ref<DAE::Exp>>,
    mut theneqns1: metamodelica::List<metamodelica::List<metamodelica::Ref<DAE::Element>>>,
    mut inEqns: metamodelica::List<metamodelica::Ref<DAE::Element>>,
) -> Result<(
    metamodelica::List<metamodelica::List<metamodelica::Ref<DAE::Element>>>,
    metamodelica::List<metamodelica::Ref<DAE::Element>>,
    metamodelica::List<metamodelica::Ref<DAE::Element>>,
)> {
    '__tco: loop {
        ::match_deref::match_deref! { match (conditions, theneqns) {
            (_, Deref @ metamodelica::ListNode::Nil) => {
                let mut eqns: metamodelica::List<metamodelica::Ref<DAE::Element>>;
                let mut beqns: metamodelica::List<metamodelica::Ref<DAE::Element>>;
                (beqns, eqns) = lowerIfEquationAsserts1(elseenqs.clone(), None, &conditions1, metamodelica::nil(), inEqns)?;
                return Ok((theneqns1.reverse(), beqns, eqns))
            },
            (Deref @ metamodelica::ListNode::Cons { head: e, tail: explst }, Deref @ metamodelica::ListNode::Cons { head: eqns, tail: eqnslst }) => {
                let mut eqns1: metamodelica::List<metamodelica::Ref<DAE::Element>>;
                let mut beqns: metamodelica::List<metamodelica::Ref<DAE::Element>>;
                let mut eqnslst1: metamodelica::List<metamodelica::List<metamodelica::Ref<DAE::Element>>>;
                let mut eqns = (*eqns).clone();
                (beqns, eqns) = lowerIfEquationAsserts1(eqns.clone(), Some(e.clone()), &conditions1, metamodelica::nil(), inEqns)?;
                { (conditions, theneqns, elseenqs, conditions1, theneqns1, inEqns) = (explst, eqnslst, elseenqs, metamodelica::cons(e.clone(), conditions1), metamodelica::cons(beqns, theneqns1), eqns.clone()); continue '__tco; }
            },
            _ => return Err("match: no arm matched"),
        } }
    }
}

fn lowerIfEquationAsserts1<'__b>(
    mut brancheqns: metamodelica::List<metamodelica::Ref<DAE::Element>>,
    mut condition: Option<metamodelica::Ref<DAE::Exp>>,
    mut conditions: &'__b metamodelica::List<metamodelica::Ref<DAE::Exp>>,
    mut brancheqns1: metamodelica::List<metamodelica::Ref<DAE::Element>>,
    mut inEqns: metamodelica::List<metamodelica::Ref<DAE::Element>>,
) -> Result<(
    metamodelica::List<metamodelica::Ref<DAE::Element>>,
    metamodelica::List<metamodelica::Ref<DAE::Element>>,
)> {
    '__tco: loop {
        ::match_deref::match_deref! { match &((brancheqns, condition.clone())) {
            (Deref @ metamodelica::ListNode::Nil, _) => {
                return Ok((brancheqns1.reverse(), inEqns))
            },
            (Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Element::ASSERT { condition: cond, message: msg, level, source }, tail: eqns }, None) => {
                let mut e: metamodelica::Ref<DAE::Exp>;
                let mut beqns: metamodelica::List<metamodelica::Ref<DAE::Element>>;
                let mut eqns = (*eqns).clone();
                e = List::fold(conditions, &fnptr!(makeIfExp, metamodelica::Ref<DAE::Exp>, metamodelica::Ref<DAE::Exp>), cond.clone())?;
                { (brancheqns, condition, conditions, brancheqns1, inEqns) = (eqns.clone(), condition, conditions, brancheqns1, metamodelica::cons(metamodelica::Ref::new(DAE::Element::ASSERT { condition: e, message: msg.clone(), level: level.clone(), source: source.clone() }), inEqns)); continue '__tco; }
            },
            (Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Element::ASSERT { condition: cond, message: msg, level, source }, tail: eqns }, Some(e)) => {
                let mut beqns: metamodelica::List<metamodelica::Ref<DAE::Element>>;
                let mut eqns = (*eqns).clone();
                let mut e = (*e).clone();
                e = metamodelica::Ref::new(DAE::Exp::IFEXP { expCond: e.clone(), expThen: cond.clone(), expElse: metamodelica::Ref::new(DAE::Exp::BCONST { bool: true }) });
                e = List::fold(conditions, &fnptr!(makeIfExp, metamodelica::Ref<DAE::Exp>, metamodelica::Ref<DAE::Exp>), e.clone())?;
                { (brancheqns, condition, conditions, brancheqns1, inEqns) = (eqns.clone(), condition, conditions, brancheqns1, metamodelica::cons(metamodelica::Ref::new(DAE::Element::ASSERT { condition: e.clone(), message: msg.clone(), level: level.clone(), source: source.clone() }), inEqns)); continue '__tco; }
            },
            (Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Element::TERMINATE { message: msg, source }, tail: eqns }, None) => {
                let mut e: metamodelica::Ref<DAE::Exp>;
                let mut beqns: metamodelica::List<metamodelica::Ref<DAE::Element>>;
                let mut eqns = (*eqns).clone();
                e = List::fold(conditions, &fnptr!(makeIfExp, metamodelica::Ref<DAE::Exp>, metamodelica::Ref<DAE::Exp>), metamodelica::Ref::new(DAE::Exp::BCONST { bool: true }))?;
                { (brancheqns, condition, conditions, brancheqns1, inEqns) = (eqns.clone(), condition, conditions, brancheqns1, metamodelica::cons(metamodelica::Ref::new(DAE::Element::ALGORITHM { algorithm_: metamodelica::Ref::new(DAE::Algorithm { statementLst: list![metamodelica::Ref::new(DAE::Statement::STMT_IF { exp: e, statementLst: list![metamodelica::Ref::new(DAE::Statement::STMT_TERMINATE { msg: msg.clone(), source: source.clone() })], else_: openmodelica_frontend_types::DAE::Else::interned_NOELSE(), source: source.clone() })] }), source: source.clone() }), inEqns)); continue '__tco; }
            },
            (Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Element::TERMINATE { message: msg, source }, tail: eqns }, Some(e)) => {
                let mut beqns: metamodelica::List<metamodelica::Ref<DAE::Element>>;
                let mut eqns = (*eqns).clone();
                let mut e = (*e).clone();
                e = List::fold(conditions, &fnptr!(makeIfExp, metamodelica::Ref<DAE::Exp>, metamodelica::Ref<DAE::Exp>), e.clone())?;
                { (brancheqns, condition, conditions, brancheqns1, inEqns) = (eqns.clone(), condition, conditions, brancheqns1, metamodelica::cons(metamodelica::Ref::new(DAE::Element::ALGORITHM { algorithm_: metamodelica::Ref::new(DAE::Algorithm { statementLst: list![metamodelica::Ref::new(DAE::Statement::STMT_IF { exp: e.clone(), statementLst: list![metamodelica::Ref::new(DAE::Statement::STMT_TERMINATE { msg: msg.clone(), source: source.clone() })], else_: openmodelica_frontend_types::DAE::Else::interned_NOELSE(), source: source.clone() })] }), source: source.clone() }), inEqns)); continue '__tco; }
            },
            (Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Element::NORETCALL { exp, source }, tail: eqns }, None) => {
                let mut beqns: metamodelica::List<metamodelica::Ref<DAE::Element>>;
                let mut eqns = (*eqns).clone();
                { (brancheqns, condition, conditions, brancheqns1, inEqns) = (eqns.clone(), condition, conditions, brancheqns1, metamodelica::cons(metamodelica::Ref::new(DAE::Element::ALGORITHM { algorithm_: metamodelica::Ref::new(DAE::Algorithm { statementLst: list![metamodelica::Ref::new(DAE::Statement::STMT_IF { exp: exp.clone(), statementLst: list![metamodelica::Ref::new(DAE::Statement::STMT_NORETCALL { exp: exp.clone(), source: source.clone() })], else_: openmodelica_frontend_types::DAE::Else::interned_NOELSE(), source: source.clone() })] }), source: source.clone() }), inEqns)); continue '__tco; }
            },
            (Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Element::NORETCALL { exp, source }, tail: eqns }, Some(e)) => {
                let mut beqns: metamodelica::List<metamodelica::Ref<DAE::Element>>;
                let mut eqns = (*eqns).clone();
                let mut e = (*e).clone();
                e = List::fold(conditions, &fnptr!(makeIfExp, metamodelica::Ref<DAE::Exp>, metamodelica::Ref<DAE::Exp>), e.clone())?;
                { (brancheqns, condition, conditions, brancheqns1, inEqns) = (eqns.clone(), condition, conditions, brancheqns1, metamodelica::cons(metamodelica::Ref::new(DAE::Element::ALGORITHM { algorithm_: metamodelica::Ref::new(DAE::Algorithm { statementLst: list![metamodelica::Ref::new(DAE::Statement::STMT_IF { exp: e.clone(), statementLst: list![metamodelica::Ref::new(DAE::Statement::STMT_NORETCALL { exp: exp.clone(), source: source.clone() })], else_: openmodelica_frontend_types::DAE::Else::interned_NOELSE(), source: source.clone() })] }), source: source.clone() }), inEqns)); continue '__tco; }
            },
            (Deref @ metamodelica::ListNode::Cons { head: eqn, tail: eqns }, _) => {
                let mut beqns: metamodelica::List<metamodelica::Ref<DAE::Element>>;
                let mut eqns = (*eqns).clone();
                { (brancheqns, condition, conditions, brancheqns1, inEqns) = (eqns.clone(), condition, conditions, metamodelica::cons(eqn.clone(), brancheqns1), inEqns); continue '__tco; }
            },
            _ => return Err("match: no arm matched"),
        } }
    }
}

fn makeIfExp(
    mut cond: metamodelica::Ref<DAE::Exp>,
    mut else_: metamodelica::Ref<DAE::Exp>,
) -> metamodelica::Ref<DAE::Exp> {
    let mut oExp: metamodelica::Ref<DAE::Exp>;
    oExp = metamodelica::Ref::new(DAE::Exp::IFEXP {
        expCond: cond,
        expThen: metamodelica::Ref::new(DAE::Exp::BCONST { bool: true }),
        expElse: else_,
    });
    oExp
}

fn lowerExtendedRecordEqns<'__b>(
    mut explst1: &'__b metamodelica::List<metamodelica::Ref<DAE::Exp>>,
    mut explst2: &'__b metamodelica::List<metamodelica::Ref<DAE::Exp>>,
    mut source: &'__b metamodelica::Ref<DAE::ElementSource>,
    mut inEqAttributes: BackendDAE::EquationAttributes,
    mut functionTree: &'__b metamodelica::Ref<AvlTreePathFunction::Tree>,
    mut inEqns: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
) -> Result<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>> {
    '__tco: loop {
        ::match_deref::match_deref! { match (explst1, explst2) {
            (Deref @ metamodelica::ListNode::Nil, Deref @ metamodelica::ListNode::Nil) => {
                return Ok(inEqns)
            },
            (Deref @ metamodelica::ListNode::Cons { head: e1, tail: elst1 }, Deref @ metamodelica::ListNode::Cons { head: e2, tail: elst2 }) => {
                let mut eqns: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
                eqns = lowerExtendedRecordEqn(e1.clone(), e2.clone(), source.clone(), inEqAttributes, functionTree, inEqns)?;
                { (explst1, explst2, source, inEqAttributes, functionTree, inEqns) = (elst1, elst2, source, inEqAttributes, functionTree, eqns); continue '__tco; }
            },
            _ => return Err("match: no arm matched"),
        } }
    }
}

fn lowerExtendedRecordEqn(
    mut inExp1: metamodelica::Ref<DAE::Exp>,
    mut inExp2: metamodelica::Ref<DAE::Exp>,
    mut source: metamodelica::Ref<DAE::ElementSource>,
    mut inEqAttributes: BackendDAE::EquationAttributes,
    mut functionTree: &metamodelica::Ref<AvlTreePathFunction::Tree>,
    mut inEqns: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
) -> Result<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>> {
    let mut outEqns: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
    outEqns = 'mc: {
        let __mc_input = &*inEqns;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut explst1: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                    let mut explst2: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                    explst1 = Expression::splitRecord(&(inExp1.clone()), &(Expression::r#typeof(inExp1.clone())?))?;
                    explst2 = Expression::splitRecord(&(inExp2.clone()), &(Expression::r#typeof(inExp2.clone())?))?;
                    Ok(lowerExtendedRecordEqns(&explst1, &explst2, &source, inEqAttributes, functionTree, inEqns.clone())?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut tp: metamodelica::Ref<DAE::Type>;
                    let mut size: i32;
                    tp = Expression::r#typeof(inExp1.clone())?;
                    let true = (DAEUtil::expTypeComplex(&tp)) else { return Err("pattern mismatch") };
                    size = Expression::sizeOf(&tp);
                    Ok(metamodelica::cons(metamodelica::Ref::new(BackendDAE::Equation::COMPLEX_EQUATION { size: size, left: inExp1.clone(), right: inExp2.clone(), source: source.clone(), attr: inEqAttributes }), inEqns.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut tp: metamodelica::Ref<DAE::Type>;
                    let mut dims: metamodelica::List<metamodelica::Ref<DAE::Dimension>>;
                    tp = Expression::r#typeof(inExp1.clone())?;
                    let true = (DAEUtil::expTypeArray(&tp)) else { return Err("pattern mismatch") };
                    dims = Expression::arrayDimension(&tp);
                    Ok(lowerArrayEqn(dims.clone(), inExp1.clone(), inExp2.clone(), source.clone(), inEqAttributes, inEqns.clone())?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut tp: metamodelica::Ref<DAE::Type>;
                    let mut size: i32;
                    tp = Expression::r#typeof(inExp1.clone())?;
                    let true = (Types::isTuple(&tp)) else { return Err("pattern mismatch") };
                    size = Expression::sizeOf(&tp);
                    Ok(metamodelica::cons(metamodelica::Ref::new(BackendDAE::Equation::COMPLEX_EQUATION { size: size, left: inExp1.clone(), right: inExp2.clone(), source: source.clone(), attr: inEqAttributes }), inEqns.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut tp: metamodelica::Ref<DAE::Type>;
                    let mut b1: bool;
                    let mut b2: bool;
                    let mut b3: bool;
                    tp = Expression::r#typeof(inExp1.clone())?;
                    b1 = DAEUtil::expTypeComplex(&tp);
                    b2 = DAEUtil::expTypeArray(&tp);
                    b3 = Types::isTuple(&tp);
                    let false = (b1 || b2 || b3) else { return Err("pattern mismatch") };
                    Ok(metamodelica::cons(metamodelica::Ref::new(BackendDAE::Equation::EQUATION { exp: inExp1.clone(), scalar: inExp2.clone(), source: source.clone(), attr: inEqAttributes }), inEqns.clone()))
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
                    Debug::traceln({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("- BackendDAECreate.lowerExtendedRecordEqn failed on: ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(inExp1.clone())?); __mm_s.push_str(&*literal!(" = ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(inExp2.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) })?;
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(outEqns)
}

fn lowerArrayEqn(
    mut dims: metamodelica::List<metamodelica::Ref<DAE::Dimension>>,
    mut e1: metamodelica::Ref<DAE::Exp>,
    mut e2: metamodelica::Ref<DAE::Exp>,
    mut source: metamodelica::Ref<DAE::ElementSource>,
    mut inEqAttributes: BackendDAE::EquationAttributes,
    mut iAcc: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
) -> Result<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>> {
    let mut outEqsLst: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
    let mut dimensions: metamodelica::List<i32>;
    let mut ea1: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
    let mut ea2: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
    let mut tp: metamodelica::Ref<DAE::Type>;
    let mut recordSize: i32;
    tp = Expression::r#typeof(e1.clone())?;
    tp = DAEUtil::expTypeElementType(&tp);
    if DAEUtil::expTypeComplex(&tp) {
        recordSize = Expression::sizeOf(&tp);
        dimensions = Expression::dimensionsSizes(dims)?;
        outEqsLst = metamodelica::cons(
            metamodelica::Ref::new(BackendDAE::Equation::ARRAY_EQUATION {
                dimSize: dimensions,
                left: e1,
                right: e2,
                source: source,
                attr: inEqAttributes,
                recordSize: Some(recordSize),
            }),
            iAcc,
        );
    } else if (Expression::isArray(&e1) || Expression::isMatrix(&e1))
        && (Expression::isArray(&e2) || Expression::isMatrix(&e2))
    {
        ea1 = Expression::flattenArrayExpToList(e1);
        ea2 = Expression::flattenArrayExpToList(e2);
        outEqsLst = generateEquations(&ea1, &ea2, &source, inEqAttributes, iAcc)?;
    } else {
        dimensions = Expression::dimensionsSizes(dims)?;
        outEqsLst = metamodelica::cons(
            metamodelica::Ref::new(BackendDAE::Equation::ARRAY_EQUATION {
                dimSize: dimensions,
                left: e1,
                right: e2,
                source: source,
                attr: inEqAttributes,
                recordSize: None,
            }),
            iAcc,
        );
    }
    Ok(outEqsLst)
}

fn generateEquations<'__b>(
    mut iE1lst: &'__b metamodelica::List<metamodelica::Ref<DAE::Exp>>,
    mut iE2lst: &'__b metamodelica::List<metamodelica::Ref<DAE::Exp>>,
    mut source: &'__b metamodelica::Ref<DAE::ElementSource>,
    mut inEqAttributes: BackendDAE::EquationAttributes,
    mut iAcc: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
) -> Result<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>> {
    '__tco: loop {
        ::match_deref::match_deref! { match (iE1lst, iE2lst) {
            (Deref @ metamodelica::ListNode::Nil, Deref @ metamodelica::ListNode::Nil) => {
                return Ok(iAcc)
            },
            (Deref @ metamodelica::ListNode::Cons { head: e1, tail: e1lst }, Deref @ metamodelica::ListNode::Cons { head: e2, tail: e2lst }) => {
                { (iE1lst, iE2lst, source, inEqAttributes, iAcc) = (e1lst, e2lst, source, inEqAttributes, metamodelica::cons(metamodelica::Ref::new(BackendDAE::Equation::EQUATION { exp: e1.clone(), scalar: e2.clone(), source: source.clone(), attr: inEqAttributes }), iAcc)); continue '__tco; }
            },
            _ => return Err("match: no arm matched"),
        } }
    }
}

fn createWhenClock(
    mut whenClkCnt: i32,
    mut e: metamodelica::Ref<DAE::Exp>,
    mut inEqs: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
    mut inVars: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
) -> (
    metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
    metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
    BackendDAE::EquationAttributes,
) {
    let mut outEqs: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
    let mut outVars: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
    let mut outEqAttrs: BackendDAE::EquationAttributes;
    let mut cr: metamodelica::Ref<DAE::ComponentRef>;
    cr = metamodelica::Ref::new(DAE::ComponentRef::CREF_IDENT {
        ident: {
            let mut __mm_s = String::new();
            __mm_s.push_str(&*arcstr::literal!(BackendDAE::WHENCLK_PRREFIX));
            __mm_s.push_str(&*intString(whenClkCnt));
            ArcStr::from(__mm_s)
        },
        identType: DAE::T_CLOCK_DEFAULT().clone(),
        subscriptLst: metamodelica::nil(),
    });
    outVars = metamodelica::cons(
        metamodelica::Ref::new(BackendDAE::Var {
            varName: cr.clone(),
            varKind: openmodelica_backend_types::BackendDAE::VarKind::VARIABLE,
            varDirection: openmodelica_frontend_types::DAE::VarDirection::BIDIR,
            varParallelism: openmodelica_frontend_types::DAE::VarParallelism::NON_PARALLEL,
            varType: DAE::T_CLOCK_DEFAULT().clone(),
            bindExp: None,
            tplExp: None,
            arryDim: metamodelica::nil(),
            source: DAE::emptyElementSource().clone(),
            values: None,
            tearingSelectOption: Some(openmodelica_backend_types::BackendDAE::TearingSelect::DEFAULT),
            hideResult: None,
            comment: None,
            connectorType: openmodelica_frontend_types::DAE::ConnectorType::interned_NON_CONNECTOR(),
            innerOuter: openmodelica_frontend_types::DAE::VarInnerOuter::NOT_INNER_OUTER,
            unreplaceable: true,
            initNonlinear: false,
            encrypted: false,
        }),
        inVars,
    );
    outEqs = metamodelica::cons(
        metamodelica::Ref::new(BackendDAE::Equation::EQUATION {
            exp: metamodelica::Ref::new(DAE::Exp::CREF {
                componentRef: cr,
                ty: DAE::T_CLOCK_DEFAULT().clone(),
            }),
            scalar: e,
            source: DAE::emptyElementSource().clone(),
            attr: BackendDAE::EQ_ATTR_DEFAULT_DYNAMIC.clone(),
        }),
        inEqs,
    );
    outEqAttrs = BackendEquation::defaultClockedEqAttr(whenClkCnt);
    (outEqs, outVars, outEqAttrs)
}

fn lowerWhenEqn(
    mut inElement: &metamodelica::Ref<DAE::Element>,
    mut functionTree: &metamodelica::Ref<AvlTreePathFunction::Tree>,
    mut inEquationLst: &metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
    mut inREquationLst: &metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
    mut inVars: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
) -> Result<(
    metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
    metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
    metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
)> {
    let mut outEquationLst: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
    let mut outREquationLst: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
    let mut outVars: metamodelica::List<metamodelica::Ref<BackendDAE::Var>> = inVars;
    (outEquationLst, outREquationLst) = 'mc: {
        let __mc_input = &**inElement;
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Element::WHEN_EQUATION { condition: cond, equations: eqnl, elsewhen_: None, source } => {
                    let mut res: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
                    let mut rEqns: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
                    let mut cond = (*cond).clone();
                    let mut outVars: metamodelica::List<metamodelica::Ref<BackendDAE::Var>> = outVars.clone();
                    let __pa0 = ::match_deref::match_deref! { match &(ExpressionSimplify::simplifyAddSymbolicOperation(metamodelica::Ref::new(DAE::EquationExp::PARTIAL_EQUATION { exp: cond.clone() }), source.clone())?) {
                        (Deref @ DAE::EquationExp::PARTIAL_EQUATION { exp: __pa0 }, _) => __pa0.clone(),
                        _ => return Err("pattern mismatch"),
                    } };
                    cond = metamodelica::Own::own(__pa0);
                    (res, rEqns, outVars) = lowerWhenEqn2(&(eqnl.clone().reverse()), metamodelica::AsArg::as_arg(&cond), functionTree, &(metamodelica::nil()), &(metamodelica::nil()), outVars.clone())?;
                    res = mergeWhenEqns(inEquationLst, &res, &(metamodelica::nil()))?;
                    rEqns = mergeWhenEqns(inREquationLst, &rEqns, &(metamodelica::nil()))?;
                    Ok(((res.clone(), rEqns.clone()), outVars.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            outVars = __wb0;
            break 'mc __v;
        }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Element::WHEN_EQUATION { condition: cond, equations: eqnl, elsewhen_: Some(elsePart), source } => {
                    let mut res: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
                    let mut rEqns: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
                    let mut trueEqnLst: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
                    let mut trueREqns: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
                    let mut cond = (*cond).clone();
                    let mut outVars: metamodelica::List<metamodelica::Ref<BackendDAE::Var>> = outVars.clone();
                    let __pa0 = ::match_deref::match_deref! { match &(ExpressionSimplify::simplifyAddSymbolicOperation(metamodelica::Ref::new(DAE::EquationExp::PARTIAL_EQUATION { exp: cond.clone() }), source.clone())?) {
                        (Deref @ DAE::EquationExp::PARTIAL_EQUATION { exp: __pa0 }, _) => __pa0.clone(),
                        _ => return Err("pattern mismatch"),
                    } };
                    cond = metamodelica::Own::own(__pa0);
                    (trueEqnLst, trueREqns, outVars) = lowerWhenEqn2(&(eqnl.clone().reverse()), metamodelica::AsArg::as_arg(&cond), functionTree, &(metamodelica::nil()), &(metamodelica::nil()), outVars.clone())?;
                    res = mergeWhenEqns(inEquationLst, &trueEqnLst, &(metamodelica::nil()))?;
                    rEqns = mergeWhenEqns(inREquationLst, &trueREqns, &(metamodelica::nil()))?;
                    (res, rEqns, outVars) = lowerWhenEqn(metamodelica::AsArg::as_arg(&elsePart), functionTree, &res, &rEqns, outVars.clone())?;
                    Ok(((res.clone(), rEqns.clone()), outVars.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            outVars = __wb0;
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut r#str: ArcStr;
                    let mut source: metamodelica::Ref<DAE::ElementSource>;
                    source = ElementSource::getElementSource(inElement)?;
                    r#str = { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("BackendDAECreate.lowerWhenEqn: equation not handled:\n")); __mm_s.push_str(&*DAEDump::dumpElementsStr(&(list![inElement.clone()]))?); ArcStr::from(__mm_s) };
                    Error::addSourceMessage(&(Error::INTERNAL_ERROR.clone()), list![r#str.clone()], &(ElementSource::getElementSourceFileInfo(source.clone())))?;
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((outEquationLst, outREquationLst, outVars))
}

fn lowerWhenEqn2(
    mut inDAEElementLst: &metamodelica::List<metamodelica::Ref<DAE::Element>>,
    mut inCond: &metamodelica::Ref<DAE::Exp>,
    mut functionTree: &metamodelica::Ref<AvlTreePathFunction::Tree>,
    mut iEquationLst: &metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
    mut iREquationLst: &metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
    mut inVar_lst: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
) -> Result<(
    metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
    metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
    metamodelica::List<metamodelica::Ref<BackendDAE::Var>>,
)> {
    let mut outEquationLst: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
    let mut outREquationLst: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
    let mut outVar_lst: metamodelica::List<metamodelica::Ref<BackendDAE::Var>> = inVar_lst;
    (outEquationLst, outREquationLst) = 'mc: {
        let __mc_input = &**inDAEElementLst;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Nil => {
                    Ok((iEquationLst.clone(), iREquationLst.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Element::EQUEQUATION { cr1: cr, cr2, source }, tail: xs } => {
                    let mut eqnl: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
                    let mut reqnl: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
                    let mut e: metamodelica::Ref<DAE::Exp>;
                    let mut eq: metamodelica::Ref<BackendDAE::Equation>;
                    let mut whenEq: metamodelica::Ref<BackendDAE::WhenEquation>;
                    let mut whenOp: BackendDAE::WhenOperator;
                    let mut outVar_lst: metamodelica::List<metamodelica::Ref<BackendDAE::Var>> = outVar_lst.clone();
                    e = Expression::crefExp(cr2.clone())?;
                    whenOp = BackendDAE::WhenOperator::ASSIGN { left: Expression::crefExp(cr.clone())?, right: e.clone(), source: source.clone() };
                    whenEq = metamodelica::Ref::new(BackendDAE::WhenEquation { condition: inCond.clone(), whenStmtLst: list![whenOp.clone()], elsewhenPart: None });
                    eq = metamodelica::Ref::new(BackendDAE::Equation::WHEN_EQUATION { size: 1, whenEquation: whenEq.clone(), source: source.clone(), attr: BackendDAE::EQ_ATTR_DEFAULT_DYNAMIC.clone() });
                    (eqnl, reqnl, outVar_lst) = lowerWhenEqn2(metamodelica::AsArg::as_arg(&xs), inCond, functionTree, &(metamodelica::cons(eq.clone(), iEquationLst.clone())), iREquationLst, outVar_lst.clone())?;
                    Ok(((eqnl.clone(), reqnl.clone()), outVar_lst.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            outVar_lst = __wb0;
            break 'mc __v;
        }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Element::DEFINE { componentRef: cr, exp: e, source }, tail: xs } => {
                    let mut eqnl: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
                    let mut reqnl: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
                    let mut eq: metamodelica::Ref<BackendDAE::Equation>;
                    let mut whenEq: metamodelica::Ref<BackendDAE::WhenEquation>;
                    let mut whenOp: BackendDAE::WhenOperator;
                    let mut e = (*e).clone();
                    let mut source = (*source).clone();
                    let mut outVar_lst: metamodelica::List<metamodelica::Ref<BackendDAE::Var>> = outVar_lst.clone();
                    (e, _) = ExpressionSolve::solve(Expression::crefExp(cr.clone())?, e.clone(), Expression::crefExp(cr.clone())?, None)?;
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(ExpressionSimplify::simplifyAddSymbolicOperation(metamodelica::Ref::new(DAE::EquationExp::PARTIAL_EQUATION { exp: e.clone() }), source.clone())?) {
                        (Deref @ DAE::EquationExp::PARTIAL_EQUATION { exp: __pa0 }, __pa1) => (__pa0.clone(), __pa1.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    e = metamodelica::Own::own(__pa0);
                    source = metamodelica::Own::own(__pa1);
                    whenOp = BackendDAE::WhenOperator::ASSIGN { left: Expression::crefExp(cr.clone())?, right: e.clone(), source: source.clone() };
                    whenEq = metamodelica::Ref::new(BackendDAE::WhenEquation { condition: inCond.clone(), whenStmtLst: list![whenOp.clone()], elsewhenPart: None });
                    eq = metamodelica::Ref::new(BackendDAE::Equation::WHEN_EQUATION { size: 1, whenEquation: whenEq.clone(), source: source.clone(), attr: BackendDAE::EQ_ATTR_DEFAULT_DYNAMIC.clone() });
                    (eqnl, reqnl, outVar_lst) = lowerWhenEqn2(metamodelica::AsArg::as_arg(&xs), inCond, functionTree, &(metamodelica::cons(eq.clone(), iEquationLst.clone())), iREquationLst, outVar_lst.clone())?;
                    Ok(((eqnl.clone(), reqnl.clone()), outVar_lst.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            outVar_lst = __wb0;
            break 'mc __v;
        }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Element::EQUATION { exp: lhs @ Deref @ DAE::Exp::TUPLE { .. }, scalar: e @ Deref @ DAE::Exp::CALL { path: _, .. }, source }, tail: xs } => {
                    let mut size: i32;
                    let mut eqnl: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
                    let mut reqnl: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
                    let mut ty: metamodelica::Ref<DAE::Type>;
                    let mut eq: metamodelica::Ref<BackendDAE::Equation>;
                    let mut outVar_lst: metamodelica::List<metamodelica::Ref<BackendDAE::Var>> = outVar_lst.clone();
                    ty = Expression::r#typeof(lhs.clone())?;
                    size = Expression::sizeOf(&ty);
                    eq = metamodelica::Ref::new(BackendDAE::Equation::WHEN_EQUATION { size: size, whenEquation: metamodelica::Ref::new(BackendDAE::WhenEquation { condition: inCond.clone(), whenStmtLst: list![BackendDAE::WhenOperator::ASSIGN { left: lhs.clone(), right: e.clone(), source: source.clone() }], elsewhenPart: None }), source: source.clone(), attr: BackendDAE::EQ_ATTR_DEFAULT_DYNAMIC.clone() });
                    eqnl = metamodelica::cons(eq.clone(), iEquationLst.clone());
                    (eqnl, reqnl, outVar_lst) = lowerWhenEqn2(metamodelica::AsArg::as_arg(&xs), inCond, functionTree, &eqnl, iREquationLst, outVar_lst.clone())?;
                    Ok(((eqnl.clone(), reqnl.clone()), outVar_lst.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            outVar_lst = __wb0;
            break 'mc __v;
        }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Element::EQUATION { exp: Deref @ DAE::Exp::TUPLE { PR: expl }, scalar: e, source }, tail: xs } => {
                    let mut eqnl: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
                    let mut reqnl: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
                    let mut outVar_lst: metamodelica::List<metamodelica::Ref<BackendDAE::Var>> = outVar_lst.clone();
                    eqnl = lowerWhenTupleEqn(metamodelica::AsArg::as_arg(&expl), inCond, metamodelica::AsArg::as_arg(&e), metamodelica::AsArg::as_arg(&source), 1, iEquationLst.clone())?;
                    (eqnl, reqnl, outVar_lst) = lowerWhenEqn2(metamodelica::AsArg::as_arg(&xs), inCond, functionTree, &eqnl, iREquationLst, outVar_lst.clone())?;
                    Ok(((eqnl.clone(), reqnl.clone()), outVar_lst.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            outVar_lst = __wb0;
            break 'mc __v;
        }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: el @ Deref @ DAE::Element::EQUATION { exp: cre @ Deref @ DAE::Exp::CREF { .. }, scalar: e, source }, tail: xs } => {
                    let mut eqnl: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
                    let mut reqnl: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
                    let mut eq: metamodelica::Ref<BackendDAE::Equation>;
                    let mut whenEq: metamodelica::Ref<BackendDAE::WhenEquation>;
                    let mut whenOp: BackendDAE::WhenOperator;
                    let mut e = (*e).clone();
                    let mut source = (*source).clone();
                    let mut outVar_lst: metamodelica::List<metamodelica::Ref<BackendDAE::Var>> = outVar_lst.clone();
                    if let Ok(__iflet0) = ExpressionSolve::solve(cre.clone(), e.clone(), cre.clone(), None) {
                        e = __iflet0.0;
                    } else {
                        Error::addCompilerError({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Failed to solve ")); __mm_s.push_str(&*DAEDump::dumpElementsStr(&(list![el.clone()]))?); ArcStr::from(__mm_s) })?;
                        return Err("fail");
                    }
                    let (__pa1, __pa2) = ::match_deref::match_deref! { match &(ExpressionSimplify::simplifyAddSymbolicOperation(metamodelica::Ref::new(DAE::EquationExp::PARTIAL_EQUATION { exp: e.clone() }), source.clone())?) {
                        (Deref @ DAE::EquationExp::PARTIAL_EQUATION { exp: __pa1 }, __pa2) => (__pa1.clone(), __pa2.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    e = metamodelica::Own::own(__pa1);
                    source = metamodelica::Own::own(__pa2);
                    whenOp = BackendDAE::WhenOperator::ASSIGN { left: cre.clone(), right: e.clone(), source: source.clone() };
                    whenEq = metamodelica::Ref::new(BackendDAE::WhenEquation { condition: inCond.clone(), whenStmtLst: list![whenOp.clone()], elsewhenPart: None });
                    eq = metamodelica::Ref::new(BackendDAE::Equation::WHEN_EQUATION { size: 1, whenEquation: whenEq.clone(), source: source.clone(), attr: BackendDAE::EQ_ATTR_DEFAULT_DYNAMIC.clone() });
                    (eqnl, reqnl, outVar_lst) = lowerWhenEqn2(metamodelica::AsArg::as_arg(&xs), inCond, functionTree, &(metamodelica::cons(eq.clone(), iEquationLst.clone())), iREquationLst, outVar_lst.clone())?;
                    Ok(((eqnl.clone(), reqnl.clone()), outVar_lst.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            outVar_lst = __wb0;
            break 'mc __v;
        }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Element::COMPLEX_EQUATION { lhs: cre @ Deref @ DAE::Exp::CREF { .. }, rhs: e, source }, tail: xs } => {
                    let mut size: i32;
                    let mut eqnl: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
                    let mut reqnl: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
                    let mut eq: metamodelica::Ref<BackendDAE::Equation>;
                    let mut whenEq: metamodelica::Ref<BackendDAE::WhenEquation>;
                    let mut whenOp: BackendDAE::WhenOperator;
                    let mut e = (*e).clone();
                    let mut source = (*source).clone();
                    let mut outVar_lst: metamodelica::List<metamodelica::Ref<BackendDAE::Var>> = outVar_lst.clone();
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(ExpressionSimplify::simplifyAddSymbolicOperation(metamodelica::Ref::new(DAE::EquationExp::EQUALITY_EXPS { lhs: cre.clone(), rhs: e.clone() }), source.clone())?) {
                        (Deref @ DAE::EquationExp::EQUALITY_EXPS { lhs: _, rhs: __pa0 }, __pa1) => (__pa0.clone(), __pa1.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    e = metamodelica::Own::own(__pa0);
                    source = metamodelica::Own::own(__pa1);
                    size = Expression::sizeOf(&(Expression::r#typeof(cre.clone())?));
                    whenOp = BackendDAE::WhenOperator::ASSIGN { left: cre.clone(), right: e.clone(), source: source.clone() };
                    whenEq = metamodelica::Ref::new(BackendDAE::WhenEquation { condition: inCond.clone(), whenStmtLst: list![whenOp.clone()], elsewhenPart: None });
                    eq = metamodelica::Ref::new(BackendDAE::Equation::WHEN_EQUATION { size: size, whenEquation: whenEq.clone(), source: source.clone(), attr: BackendDAE::EQ_ATTR_DEFAULT_DYNAMIC.clone() });
                    (eqnl, reqnl, outVar_lst) = lowerWhenEqn2(metamodelica::AsArg::as_arg(&xs), inCond, functionTree, &(metamodelica::cons(eq.clone(), iEquationLst.clone())), iREquationLst, outVar_lst.clone())?;
                    Ok(((eqnl.clone(), reqnl.clone()), outVar_lst.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            outVar_lst = __wb0;
            break 'mc __v;
        }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Element::COMPLEX_EQUATION { lhs: cre @ Deref @ DAE::Exp::TUPLE { PR: expl }, rhs: e, source }, tail: xs } => {
                    let mut eqnl: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
                    let mut reqnl: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
                    let mut e = (*e).clone();
                    let mut source = (*source).clone();
                    let mut outVar_lst: metamodelica::List<metamodelica::Ref<BackendDAE::Var>> = outVar_lst.clone();
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(ExpressionSimplify::simplifyAddSymbolicOperation(metamodelica::Ref::new(DAE::EquationExp::EQUALITY_EXPS { lhs: cre.clone(), rhs: e.clone() }), source.clone())?) {
                        (Deref @ DAE::EquationExp::EQUALITY_EXPS { lhs: _, rhs: __pa0 }, __pa1) => (__pa0.clone(), __pa1.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    e = metamodelica::Own::own(__pa0);
                    source = metamodelica::Own::own(__pa1);
                    eqnl = lowerWhenTupleEqn(metamodelica::AsArg::as_arg(&expl), inCond, metamodelica::AsArg::as_arg(&e), metamodelica::AsArg::as_arg(&source), 1, iEquationLst.clone())?;
                    (eqnl, reqnl, outVar_lst) = lowerWhenEqn2(metamodelica::AsArg::as_arg(&xs), inCond, functionTree, &eqnl, iREquationLst, outVar_lst.clone())?;
                    Ok(((eqnl.clone(), reqnl.clone()), outVar_lst.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            outVar_lst = __wb0;
            break 'mc __v;
        }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Element::IF_EQUATION { condition1: expl, equations2: eqnslst, equations3: eqns, source }, tail: xs } => {
                    let mut eqnl: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
                    let mut reqnl: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
                    let mut ht: (metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>, (i32, i32, metamodelica::Array<Option<(metamodelica::Ref<DAE::ComponentRef>, (metamodelica::Ref<DAE::Exp>, metamodelica::Ref<DAE::ElementSource>))>>), i32, (HashTableCrToExpSourceTpl::FuncHashCref, HashTableCrToExpSourceTpl::FuncCrefEqual, HashTableCrToExpSourceTpl::FuncCrefStr, HashTableCrToExpSourceTpl::FuncExpStr));
                    let mut crexplst: metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, (metamodelica::Ref<DAE::Exp>, metamodelica::Ref<DAE::ElementSource>))>;
                    let mut outVar_lst: metamodelica::List<metamodelica::Ref<BackendDAE::Var>> = outVar_lst.clone();
                    ht = HashTableCrToExpSourceTpl::emptyHashTable();
                    ht = lowerWhenIfEqnsElse(eqns.clone(), functionTree, ht.clone())?;
                    ht = lowerWhenIfEqns(&(expl.clone().reverse()), &(eqnslst.clone().reverse()), functionTree, ht.clone())?;
                    crexplst = BaseHashTable::hashTableList(&ht)?;
                    eqnl = lowerWhenIfEqns2(&crexplst, inCond, metamodelica::AsArg::as_arg(&source), iEquationLst.clone())?;
                    (eqnl, reqnl, outVar_lst) = lowerWhenEqn2(metamodelica::AsArg::as_arg(&xs), inCond, functionTree, &eqnl, iREquationLst, outVar_lst.clone())?;
                    Ok(((eqnl.clone(), reqnl.clone()), outVar_lst.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            outVar_lst = __wb0;
            break 'mc __v;
        }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Element::ARRAY_EQUATION { dimension: ds, exp: cre @ Deref @ DAE::Exp::CREF { .. }, array: e, source }, tail: xs } => {
                    let mut size: i32;
                    let mut eqnl: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
                    let mut reqnl: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
                    let mut eq: metamodelica::Ref<BackendDAE::Equation>;
                    let mut whenEq: metamodelica::Ref<BackendDAE::WhenEquation>;
                    let mut whenOp: BackendDAE::WhenOperator;
                    let mut e = (*e).clone();
                    let mut source = (*source).clone();
                    let mut outVar_lst: metamodelica::List<metamodelica::Ref<BackendDAE::Var>> = outVar_lst.clone();
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(ExpressionSimplify::simplifyAddSymbolicOperation(metamodelica::Ref::new(DAE::EquationExp::EQUALITY_EXPS { lhs: cre.clone(), rhs: e.clone() }), source.clone())?) {
                        (Deref @ DAE::EquationExp::EQUALITY_EXPS { lhs: _, rhs: __pa0 }, __pa1) => (__pa0.clone(), __pa1.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    e = metamodelica::Own::own(__pa0);
                    source = metamodelica::Own::own(__pa1);
                    size = List::fold(&(Expression::dimensionsSizes(ds.clone())?), &fnptr!(intMul, i32, i32), 1)?;
                    whenOp = BackendDAE::WhenOperator::ASSIGN { left: cre.clone(), right: e.clone(), source: source.clone() };
                    whenEq = metamodelica::Ref::new(BackendDAE::WhenEquation { condition: inCond.clone(), whenStmtLst: list![whenOp.clone()], elsewhenPart: None });
                    eq = metamodelica::Ref::new(BackendDAE::Equation::WHEN_EQUATION { size: size, whenEquation: whenEq.clone(), source: source.clone(), attr: BackendDAE::EQ_ATTR_DEFAULT_DYNAMIC.clone() });
                    (eqnl, reqnl, outVar_lst) = lowerWhenEqn2(metamodelica::AsArg::as_arg(&xs), inCond, functionTree, &(metamodelica::cons(eq.clone(), iEquationLst.clone())), iREquationLst, outVar_lst.clone())?;
                    Ok(((eqnl.clone(), reqnl.clone()), outVar_lst.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            outVar_lst = __wb0;
            break 'mc __v;
        }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Element::ARRAY_EQUATION { exp: cre @ Deref @ DAE::Exp::TUPLE { PR: expl }, array: e, source, .. }, tail: xs } => {
                    let mut eqnl: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
                    let mut reqnl: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
                    let mut e = (*e).clone();
                    let mut source = (*source).clone();
                    let mut outVar_lst: metamodelica::List<metamodelica::Ref<BackendDAE::Var>> = outVar_lst.clone();
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(ExpressionSimplify::simplifyAddSymbolicOperation(metamodelica::Ref::new(DAE::EquationExp::EQUALITY_EXPS { lhs: cre.clone(), rhs: e.clone() }), source.clone())?) {
                        (Deref @ DAE::EquationExp::EQUALITY_EXPS { lhs: _, rhs: __pa0 }, __pa1) => (__pa0.clone(), __pa1.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    e = metamodelica::Own::own(__pa0);
                    source = metamodelica::Own::own(__pa1);
                    eqnl = lowerWhenTupleEqn(metamodelica::AsArg::as_arg(&expl), inCond, metamodelica::AsArg::as_arg(&e), metamodelica::AsArg::as_arg(&source), 1, iEquationLst.clone())?;
                    (eqnl, reqnl, outVar_lst) = lowerWhenEqn2(metamodelica::AsArg::as_arg(&xs), inCond, functionTree, &eqnl, iREquationLst, outVar_lst.clone())?;
                    Ok(((eqnl.clone(), reqnl.clone()), outVar_lst.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            outVar_lst = __wb0;
            break 'mc __v;
        }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Element::ASSERT { condition: cond, message: e, level, source }, tail: xs } => {
                    let mut eqnl: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
                    let mut reqnl: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
                    let mut eq: metamodelica::Ref<BackendDAE::Equation>;
                    let mut whenEq: metamodelica::Ref<BackendDAE::WhenEquation>;
                    let mut whenOp: BackendDAE::WhenOperator;
                    let mut outVar_lst: metamodelica::List<metamodelica::Ref<BackendDAE::Var>> = outVar_lst.clone();
                    whenOp = BackendDAE::WhenOperator::ASSERT { condition: cond.clone(), message: e.clone(), level: level.clone(), source: source.clone() };
                    whenEq = metamodelica::Ref::new(BackendDAE::WhenEquation { condition: inCond.clone(), whenStmtLst: list![whenOp.clone()], elsewhenPart: None });
                    eq = metamodelica::Ref::new(BackendDAE::Equation::WHEN_EQUATION { size: 0, whenEquation: whenEq.clone(), source: source.clone(), attr: BackendDAE::EQ_ATTR_DEFAULT_DYNAMIC.clone() });
                    (eqnl, reqnl, outVar_lst) = lowerWhenEqn2(metamodelica::AsArg::as_arg(&xs), inCond, functionTree, iEquationLst, &(metamodelica::cons(eq.clone(), iREquationLst.clone())), outVar_lst.clone())?;
                    Ok(((eqnl.clone(), reqnl.clone()), outVar_lst.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            outVar_lst = __wb0;
            break 'mc __v;
        }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Element::REINIT { componentRef: cr, exp: e, source }, tail: xs } => {
                    let mut eqnl: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
                    let mut reqnl: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
                    let mut eq: metamodelica::Ref<BackendDAE::Equation>;
                    let mut whenEq: metamodelica::Ref<BackendDAE::WhenEquation>;
                    let mut whenOp: BackendDAE::WhenOperator;
                    let mut var_opt: Option<metamodelica::List<metamodelica::Ref<BackendDAE::Var>>>;
                    let mut vars: BackendDAE::Variables;
                    let mut outVar_lst: metamodelica::List<metamodelica::Ref<BackendDAE::Var>> = outVar_lst.clone();
                    whenOp = BackendDAE::WhenOperator::REINIT { stateVar: cr.clone(), value: e.clone(), source: source.clone() };
                    whenEq = metamodelica::Ref::new(BackendDAE::WhenEquation { condition: inCond.clone(), whenStmtLst: list![whenOp.clone()], elsewhenPart: None });
                    eq = metamodelica::Ref::new(BackendDAE::Equation::WHEN_EQUATION { size: 0, whenEquation: whenEq.clone(), source: source.clone(), attr: BackendDAE::EQ_ATTR_DEFAULT_DYNAMIC.clone() });
                    vars = BackendVariable::listVar(outVar_lst.clone())?;
                    var_opt = BackendVariable::getVarTryHard(cr.clone(), &vars);
                    if (var_opt).is_some() {
                        for mut var in &*Util::getOption(var_opt.clone())? {
                            let mut var = var.clone();
                            var = BackendVariable::setVarStateSelect(var.clone(), openmodelica_frontend_types::DAE::StateSelect::ALWAYS)?;
                            vars = BackendVariable::addVar(var.clone(), vars.clone())?;
                        }
                    }
                    outVar_lst = BackendVariable::varList(&vars)?;
                    (eqnl, reqnl, outVar_lst) = lowerWhenEqn2(metamodelica::AsArg::as_arg(&xs), inCond, functionTree, iEquationLst, &(metamodelica::cons(eq.clone(), iREquationLst.clone())), outVar_lst.clone())?;
                    Ok(((eqnl.clone(), reqnl.clone()), outVar_lst.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            outVar_lst = __wb0;
            break 'mc __v;
        }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Element::TERMINATE { message: e, source }, tail: xs } => {
                    let mut eqnl: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
                    let mut reqnl: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
                    let mut eq: metamodelica::Ref<BackendDAE::Equation>;
                    let mut whenEq: metamodelica::Ref<BackendDAE::WhenEquation>;
                    let mut whenOp: BackendDAE::WhenOperator;
                    let mut outVar_lst: metamodelica::List<metamodelica::Ref<BackendDAE::Var>> = outVar_lst.clone();
                    whenOp = BackendDAE::WhenOperator::TERMINATE { message: e.clone(), source: source.clone() };
                    whenEq = metamodelica::Ref::new(BackendDAE::WhenEquation { condition: inCond.clone(), whenStmtLst: list![whenOp.clone()], elsewhenPart: None });
                    eq = metamodelica::Ref::new(BackendDAE::Equation::WHEN_EQUATION { size: 0, whenEquation: whenEq.clone(), source: source.clone(), attr: BackendDAE::EQ_ATTR_DEFAULT_DYNAMIC.clone() });
                    (eqnl, reqnl, outVar_lst) = lowerWhenEqn2(metamodelica::AsArg::as_arg(&xs), inCond, functionTree, iEquationLst, &(metamodelica::cons(eq.clone(), iREquationLst.clone())), outVar_lst.clone())?;
                    Ok(((eqnl.clone(), reqnl.clone()), outVar_lst.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            outVar_lst = __wb0;
            break 'mc __v;
        }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Element::NORETCALL { exp: e, source }, tail: xs } => {
                    let mut eqnl: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
                    let mut reqnl: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
                    let mut eq: metamodelica::Ref<BackendDAE::Equation>;
                    let mut whenEq: metamodelica::Ref<BackendDAE::WhenEquation>;
                    let mut whenOp: BackendDAE::WhenOperator;
                    let mut outVar_lst: metamodelica::List<metamodelica::Ref<BackendDAE::Var>> = outVar_lst.clone();
                    whenOp = BackendDAE::WhenOperator::NORETCALL { exp: e.clone(), source: source.clone() };
                    whenEq = metamodelica::Ref::new(BackendDAE::WhenEquation { condition: inCond.clone(), whenStmtLst: list![whenOp.clone()], elsewhenPart: None });
                    eq = metamodelica::Ref::new(BackendDAE::Equation::WHEN_EQUATION { size: 0, whenEquation: whenEq.clone(), source: source.clone(), attr: BackendDAE::EQ_ATTR_DEFAULT_DYNAMIC.clone() });
                    (eqnl, reqnl, outVar_lst) = lowerWhenEqn2(metamodelica::AsArg::as_arg(&xs), inCond, functionTree, iEquationLst, &(metamodelica::cons(eq.clone(), iREquationLst.clone())), outVar_lst.clone())?;
                    Ok(((eqnl.clone(), reqnl.clone()), outVar_lst.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            outVar_lst = __wb0;
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: el, tail: _ } => {
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { return Err("pattern mismatch") };
                    Debug::traceln({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("- BackendDAECreate.lowerWhenEqn2 failed on:")); __mm_s.push_str(&*DAEDump::dumpElementsStr(&(list![el.clone()]))?); ArcStr::from(__mm_s) })?;
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: _, tail: xs } => {
                    let mut eqnl: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
                    let mut reqnl: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
                    let mut outVar_lst: metamodelica::List<metamodelica::Ref<BackendDAE::Var>> = outVar_lst.clone();
                    let true = (Flags::getConfigBool(Flags::CHECK_MODEL.clone())?) else { return Err("pattern mismatch") };
                    (eqnl, reqnl, outVar_lst) = lowerWhenEqn2(metamodelica::AsArg::as_arg(&xs), inCond, functionTree, iEquationLst, iREquationLst, outVar_lst.clone())?;
                    Ok(((eqnl.clone(), reqnl.clone()), outVar_lst.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            outVar_lst = __wb0;
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((outEquationLst, outREquationLst, outVar_lst))
}

fn lowerWhenTupleEqn<'__b>(
    mut explst: &'__b metamodelica::List<metamodelica::Ref<DAE::Exp>>,
    mut inCond: &'__b metamodelica::Ref<DAE::Exp>,
    mut e: &'__b metamodelica::Ref<DAE::Exp>,
    mut source: &'__b metamodelica::Ref<DAE::ElementSource>,
    mut i: i32,
    mut iEquationLst: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
) -> Result<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>> {
    '__tco: loop {
        ::match_deref::match_deref! { match explst {
            Deref @ metamodelica::ListNode::Nil => {
                return Ok(iEquationLst)
            },
            Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Exp::CREF { componentRef: cr, ty }, tail: rest } => {
                let mut size: i32;
                let mut whenEq: metamodelica::Ref<BackendDAE::WhenEquation>;
                let mut whenOp: BackendDAE::WhenOperator;
                size = Expression::sizeOf(metamodelica::AsArg::as_arg(&ty));
                whenOp = BackendDAE::WhenOperator::ASSIGN { left: Expression::crefExp(cr.clone())?, right: metamodelica::Ref::new(DAE::Exp::TSUB { exp: e.clone(), ix: i, ty: ty.clone() }), source: source.clone() };
                whenEq = metamodelica::Ref::new(BackendDAE::WhenEquation { condition: inCond.clone(), whenStmtLst: list![whenOp], elsewhenPart: None });
                { (explst, inCond, e, source, i, iEquationLst) = (rest, inCond, e, source, i + 1, metamodelica::cons(metamodelica::Ref::new(BackendDAE::Equation::WHEN_EQUATION { size: size, whenEquation: whenEq, source: source.clone(), attr: BackendDAE::EQ_ATTR_DEFAULT_DYNAMIC.clone() }), iEquationLst)); continue '__tco; }
            },
            _ => return Err("match: no arm matched"),
        } }
    }
}

fn lowerWhenIfEqns2<'__b>(
    mut crexplst: &'__b metamodelica::List<(
        metamodelica::Ref<DAE::ComponentRef>,
        (metamodelica::Ref<DAE::Exp>, metamodelica::Ref<DAE::ElementSource>),
    )>,
    mut inCond: &'__b metamodelica::Ref<DAE::Exp>,
    mut iSource: &'__b metamodelica::Ref<DAE::ElementSource>,
    mut inEqns: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
) -> Result<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>> {
    '__tco: loop {
        ::match_deref::match_deref! { match crexplst {
            Deref @ metamodelica::ListNode::Nil => {
                return Ok(inEqns)
            },
            Deref @ metamodelica::ListNode::Cons { head: (cr, (e, source)), tail: rest } => {
                let mut size: i32;
                let mut whenEq: metamodelica::Ref<BackendDAE::WhenEquation>;
                let mut whenOp: BackendDAE::WhenOperator;
                let mut source = (*source).clone();
                source = ElementSource::mergeSources(iSource, metamodelica::AsArg::as_arg(&source))?;
                size = Expression::sizeOf(&(Expression::r#typeof(e.clone())?));
                whenOp = BackendDAE::WhenOperator::ASSIGN { left: Expression::crefExp(cr.clone())?, right: e.clone(), source: source.clone() };
                whenEq = metamodelica::Ref::new(BackendDAE::WhenEquation { condition: inCond.clone(), whenStmtLst: list![whenOp], elsewhenPart: None });
                { (crexplst, inCond, iSource, inEqns) = (rest, inCond, iSource, metamodelica::cons(metamodelica::Ref::new(BackendDAE::Equation::WHEN_EQUATION { size: size, whenEquation: whenEq, source: source.clone(), attr: BackendDAE::EQ_ATTR_DEFAULT_DYNAMIC.clone() }), inEqns)); continue '__tco; }
            },
            _ => return Err("match: no arm matched"),
        } }
    }
}

fn lowerWhenIfEqns<'__b>(
    mut conditions: &'__b metamodelica::List<metamodelica::Ref<DAE::Exp>>,
    mut theneqns: &'__b metamodelica::List<metamodelica::List<metamodelica::Ref<DAE::Element>>>,
    mut functionTree: &'__b metamodelica::Ref<AvlTreePathFunction::Tree>,
    mut iHt: (
        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<
                Option<(
                    metamodelica::Ref<DAE::ComponentRef>,
                    (metamodelica::Ref<DAE::Exp>, metamodelica::Ref<DAE::ElementSource>),
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
            Arc<
                dyn ::std::ops::Fn(
                        (metamodelica::Ref<DAE::Exp>, metamodelica::Ref<DAE::ElementSource>),
                    ) -> Result<ArcStr>
                    + 'static,
            >,
        ),
    ),
) -> Result<(
    metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
    (
        i32,
        i32,
        metamodelica::Array<
            Option<(
                metamodelica::Ref<DAE::ComponentRef>,
                (metamodelica::Ref<DAE::Exp>, metamodelica::Ref<DAE::ElementSource>),
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
        Arc<
            dyn ::std::ops::Fn((metamodelica::Ref<DAE::Exp>, metamodelica::Ref<DAE::ElementSource>)) -> Result<ArcStr>
                + 'static,
        >,
    ),
)> {
    '__tco: loop {
        ::match_deref::match_deref! { match (conditions, theneqns) {
            (Deref @ metamodelica::ListNode::Nil, Deref @ metamodelica::ListNode::Nil) => {
                return Ok(iHt)
            },
            (Deref @ metamodelica::ListNode::Cons { head: c, tail: explst }, Deref @ metamodelica::ListNode::Cons { head: eqns, tail: rest }) => {
                let mut ht: (metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>, (i32, i32, metamodelica::Array<Option<(metamodelica::Ref<DAE::ComponentRef>, (metamodelica::Ref<DAE::Exp>, metamodelica::Ref<DAE::ElementSource>))>>), i32, (HashTableCrToExpSourceTpl::FuncHashCref, HashTableCrToExpSourceTpl::FuncCrefEqual, HashTableCrToExpSourceTpl::FuncCrefStr, HashTableCrToExpSourceTpl::FuncExpStr));
                ht = lowerWhenIfEqns1(metamodelica::AsArg::as_arg(&c), metamodelica::AsArg::as_arg(&eqns), functionTree, iHt)?;
                { (conditions, theneqns, functionTree, iHt) = (explst, rest, functionTree, ht); continue '__tco; }
            },
            _ => return Err("match: no arm matched"),
        } }
    }
}

fn lowerWhenIfEqns1<'__b>(
    mut condition: &'__b metamodelica::Ref<DAE::Exp>,
    mut brancheqns: &'__b metamodelica::List<metamodelica::Ref<DAE::Element>>,
    mut functionTree: &'__b metamodelica::Ref<AvlTreePathFunction::Tree>,
    mut iHt: (
        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<
                Option<(
                    metamodelica::Ref<DAE::ComponentRef>,
                    (metamodelica::Ref<DAE::Exp>, metamodelica::Ref<DAE::ElementSource>),
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
            Arc<
                dyn ::std::ops::Fn(
                        (metamodelica::Ref<DAE::Exp>, metamodelica::Ref<DAE::ElementSource>),
                    ) -> Result<ArcStr>
                    + 'static,
            >,
        ),
    ),
) -> Result<(
    metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
    (
        i32,
        i32,
        metamodelica::Array<
            Option<(
                metamodelica::Ref<DAE::ComponentRef>,
                (metamodelica::Ref<DAE::Exp>, metamodelica::Ref<DAE::ElementSource>),
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
        Arc<
            dyn ::std::ops::Fn((metamodelica::Ref<DAE::Exp>, metamodelica::Ref<DAE::ElementSource>)) -> Result<ArcStr>
                + 'static,
        >,
    ),
)> {
    '__tco: loop {
        ::match_deref::match_deref! { match brancheqns {
            Deref @ metamodelica::ListNode::Nil => {
                return Ok(iHt)
            },
            Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Element::EQUEQUATION { cr1: cr, cr2, source }, tail: rest } => {
                let mut e: metamodelica::Ref<DAE::Exp>;
                let mut exp: metamodelica::Ref<DAE::Exp>;
                let mut source1: metamodelica::Ref<DAE::ElementSource>;
                let mut ht: (metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>, (i32, i32, metamodelica::Array<Option<(metamodelica::Ref<DAE::ComponentRef>, (metamodelica::Ref<DAE::Exp>, metamodelica::Ref<DAE::ElementSource>))>>), i32, (HashTableCrToExpSourceTpl::FuncHashCref, HashTableCrToExpSourceTpl::FuncCrefEqual, HashTableCrToExpSourceTpl::FuncCrefStr, HashTableCrToExpSourceTpl::FuncExpStr));
                let mut source = (*source).clone();
                e = Expression::crefExp(cr2.clone())?;
                let false = (Expression::expHasCrefNoPreorDer(e.clone(), cr.clone())?) else { return Err("pattern mismatch") };
                (exp, source1) = BaseHashTable::get(cr.clone(), &iHt)?;
                exp = metamodelica::Ref::new(DAE::Exp::IFEXP { expCond: condition.clone(), expThen: e, expElse: exp });
                source = ElementSource::mergeSources(metamodelica::AsArg::as_arg(&source), &source1)?;
                ht = BaseHashTable::add((cr.clone(), (exp, source.clone())), iHt)?;
                { (condition, brancheqns, functionTree, iHt) = (condition, rest, functionTree, ht); continue '__tco; }
            },
            Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Element::DEFINE { componentRef: cr, exp: e, source }, tail: rest } => {
                let mut exp: metamodelica::Ref<DAE::Exp>;
                let mut source1: metamodelica::Ref<DAE::ElementSource>;
                let mut ht: (metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>, (i32, i32, metamodelica::Array<Option<(metamodelica::Ref<DAE::ComponentRef>, (metamodelica::Ref<DAE::Exp>, metamodelica::Ref<DAE::ElementSource>))>>), i32, (HashTableCrToExpSourceTpl::FuncHashCref, HashTableCrToExpSourceTpl::FuncCrefEqual, HashTableCrToExpSourceTpl::FuncCrefStr, HashTableCrToExpSourceTpl::FuncExpStr));
                let mut source = (*source).clone();
                let false = (Expression::expHasCrefNoPreorDer(e.clone(), cr.clone())?) else { return Err("pattern mismatch") };
                (exp, source1) = BaseHashTable::get(cr.clone(), &iHt)?;
                exp = metamodelica::Ref::new(DAE::Exp::IFEXP { expCond: condition.clone(), expThen: e.clone(), expElse: exp });
                source = ElementSource::mergeSources(metamodelica::AsArg::as_arg(&source), &source1)?;
                ht = BaseHashTable::add((cr.clone(), (exp, source.clone())), iHt)?;
                { (condition, brancheqns, functionTree, iHt) = (condition, rest, functionTree, ht); continue '__tco; }
            },
            Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Element::EQUATION { exp: Deref @ DAE::Exp::CREF { componentRef: cr, .. }, scalar: e, source }, tail: rest } => {
                let mut exp: metamodelica::Ref<DAE::Exp>;
                let mut source1: metamodelica::Ref<DAE::ElementSource>;
                let mut ht: (metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>, (i32, i32, metamodelica::Array<Option<(metamodelica::Ref<DAE::ComponentRef>, (metamodelica::Ref<DAE::Exp>, metamodelica::Ref<DAE::ElementSource>))>>), i32, (HashTableCrToExpSourceTpl::FuncHashCref, HashTableCrToExpSourceTpl::FuncCrefEqual, HashTableCrToExpSourceTpl::FuncCrefStr, HashTableCrToExpSourceTpl::FuncExpStr));
                let mut source = (*source).clone();
                let false = (Expression::expHasCrefNoPreorDer(e.clone(), cr.clone())?) else { return Err("pattern mismatch") };
                (exp, source1) = BaseHashTable::get(cr.clone(), &iHt)?;
                exp = metamodelica::Ref::new(DAE::Exp::IFEXP { expCond: condition.clone(), expThen: e.clone(), expElse: exp });
                source = ElementSource::mergeSources(metamodelica::AsArg::as_arg(&source), &source1)?;
                ht = BaseHashTable::add((cr.clone(), (exp, source.clone())), iHt)?;
                { (condition, brancheqns, functionTree, iHt) = (condition, rest, functionTree, ht); continue '__tco; }
            },
            Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Element::COMPLEX_EQUATION { lhs: Deref @ DAE::Exp::CREF { componentRef: cr, .. }, rhs: e, source }, tail: rest } => {
                let mut exp: metamodelica::Ref<DAE::Exp>;
                let mut source1: metamodelica::Ref<DAE::ElementSource>;
                let mut ht: (metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>, (i32, i32, metamodelica::Array<Option<(metamodelica::Ref<DAE::ComponentRef>, (metamodelica::Ref<DAE::Exp>, metamodelica::Ref<DAE::ElementSource>))>>), i32, (HashTableCrToExpSourceTpl::FuncHashCref, HashTableCrToExpSourceTpl::FuncCrefEqual, HashTableCrToExpSourceTpl::FuncCrefStr, HashTableCrToExpSourceTpl::FuncExpStr));
                let mut source = (*source).clone();
                let false = (Expression::expHasCrefNoPreorDer(e.clone(), cr.clone())?) else { return Err("pattern mismatch") };
                (exp, source1) = BaseHashTable::get(cr.clone(), &iHt)?;
                exp = metamodelica::Ref::new(DAE::Exp::IFEXP { expCond: condition.clone(), expThen: e.clone(), expElse: exp });
                source = ElementSource::mergeSources(metamodelica::AsArg::as_arg(&source), &source1)?;
                ht = BaseHashTable::add((cr.clone(), (exp, source.clone())), iHt)?;
                { (condition, brancheqns, functionTree, iHt) = (condition, rest, functionTree, ht); continue '__tco; }
            },
            Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Element::ARRAY_EQUATION { exp: Deref @ DAE::Exp::CREF { componentRef: cr, .. }, array: e, source, .. }, tail: rest } => {
                let mut exp: metamodelica::Ref<DAE::Exp>;
                let mut source1: metamodelica::Ref<DAE::ElementSource>;
                let mut ht: (metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>, (i32, i32, metamodelica::Array<Option<(metamodelica::Ref<DAE::ComponentRef>, (metamodelica::Ref<DAE::Exp>, metamodelica::Ref<DAE::ElementSource>))>>), i32, (HashTableCrToExpSourceTpl::FuncHashCref, HashTableCrToExpSourceTpl::FuncCrefEqual, HashTableCrToExpSourceTpl::FuncCrefStr, HashTableCrToExpSourceTpl::FuncExpStr));
                let mut source = (*source).clone();
                let false = (Expression::expHasCrefNoPreorDer(e.clone(), cr.clone())?) else { return Err("pattern mismatch") };
                (exp, source1) = BaseHashTable::get(cr.clone(), &iHt)?;
                exp = metamodelica::Ref::new(DAE::Exp::IFEXP { expCond: condition.clone(), expThen: e.clone(), expElse: exp });
                source = ElementSource::mergeSources(metamodelica::AsArg::as_arg(&source), &source1)?;
                ht = BaseHashTable::add((cr.clone(), (exp, source.clone())), iHt)?;
                { (condition, brancheqns, functionTree, iHt) = (condition, rest, functionTree, ht); continue '__tco; }
            },
            Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Element::IF_EQUATION { condition1: expl, equations2: eqnslst, equations3: eqns, source }, tail: rest } => {
                let mut ht: (metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>, (i32, i32, metamodelica::Array<Option<(metamodelica::Ref<DAE::ComponentRef>, (metamodelica::Ref<DAE::Exp>, metamodelica::Ref<DAE::ElementSource>))>>), i32, (HashTableCrToExpSourceTpl::FuncHashCref, HashTableCrToExpSourceTpl::FuncCrefEqual, HashTableCrToExpSourceTpl::FuncCrefStr, HashTableCrToExpSourceTpl::FuncExpStr));
                let mut crexplst: metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, (metamodelica::Ref<DAE::Exp>, metamodelica::Ref<DAE::ElementSource>))>;
                ht = HashTableCrToExpSourceTpl::emptyHashTable();
                ht = lowerWhenIfEqnsElse(eqns.clone(), functionTree, ht)?;
                ht = lowerWhenIfEqns(&(expl.clone().reverse()), &(eqnslst.clone().reverse()), functionTree, ht)?;
                crexplst = BaseHashTable::hashTableList(&ht)?;
                ht = lowerWhenIfEqnsMergeNestedIf(&crexplst, condition, metamodelica::AsArg::as_arg(&source), iHt)?;
                { (condition, brancheqns, functionTree, iHt) = (condition, rest, functionTree, ht); continue '__tco; }
            },
            _ => return Err("match: no arm matched"),
        } }
    }
}

fn lowerWhenIfEqnsMergeNestedIf<'__b>(
    mut crexplst: &'__b metamodelica::List<(
        metamodelica::Ref<DAE::ComponentRef>,
        (metamodelica::Ref<DAE::Exp>, metamodelica::Ref<DAE::ElementSource>),
    )>,
    mut inCond: &'__b metamodelica::Ref<DAE::Exp>,
    mut iSource: &'__b metamodelica::Ref<DAE::ElementSource>,
    mut iHt: (
        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<
                Option<(
                    metamodelica::Ref<DAE::ComponentRef>,
                    (metamodelica::Ref<DAE::Exp>, metamodelica::Ref<DAE::ElementSource>),
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
            Arc<
                dyn ::std::ops::Fn(
                        (metamodelica::Ref<DAE::Exp>, metamodelica::Ref<DAE::ElementSource>),
                    ) -> Result<ArcStr>
                    + 'static,
            >,
        ),
    ),
) -> Result<(
    metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
    (
        i32,
        i32,
        metamodelica::Array<
            Option<(
                metamodelica::Ref<DAE::ComponentRef>,
                (metamodelica::Ref<DAE::Exp>, metamodelica::Ref<DAE::ElementSource>),
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
        Arc<
            dyn ::std::ops::Fn((metamodelica::Ref<DAE::Exp>, metamodelica::Ref<DAE::ElementSource>)) -> Result<ArcStr>
                + 'static,
        >,
    ),
)> {
    '__tco: loop {
        ::match_deref::match_deref! { match crexplst {
            Deref @ metamodelica::ListNode::Nil => {
                return Ok(iHt)
            },
            Deref @ metamodelica::ListNode::Cons { head: (cr, (e, source)), tail: rest } => {
                let mut exp: metamodelica::Ref<DAE::Exp>;
                let mut ht: (metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>, (i32, i32, metamodelica::Array<Option<(metamodelica::Ref<DAE::ComponentRef>, (metamodelica::Ref<DAE::Exp>, metamodelica::Ref<DAE::ElementSource>))>>), i32, (HashTableCrToExpSourceTpl::FuncHashCref, HashTableCrToExpSourceTpl::FuncCrefEqual, HashTableCrToExpSourceTpl::FuncCrefStr, HashTableCrToExpSourceTpl::FuncExpStr));
                let mut source = (*source).clone();
                (exp, _) = BaseHashTable::get(cr.clone(), &iHt)?;
                exp = metamodelica::Ref::new(DAE::Exp::IFEXP { expCond: inCond.clone(), expThen: e.clone(), expElse: exp });
                source = ElementSource::mergeSources(iSource, metamodelica::AsArg::as_arg(&source))?;
                ht = BaseHashTable::add((cr.clone(), (exp, source.clone())), iHt)?;
                { (crexplst, inCond, iSource, iHt) = (rest, inCond, iSource, ht); continue '__tco; }
            },
            _ => return Err("match: no arm matched"),
        } }
    }
}

fn lowerWhenIfEqnsElse<'__b>(
    mut elseenqs: metamodelica::List<metamodelica::Ref<DAE::Element>>,
    mut functionTree: &'__b metamodelica::Ref<AvlTreePathFunction::Tree>,
    mut iHt: (
        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<
                Option<(
                    metamodelica::Ref<DAE::ComponentRef>,
                    (metamodelica::Ref<DAE::Exp>, metamodelica::Ref<DAE::ElementSource>),
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
            Arc<
                dyn ::std::ops::Fn(
                        (metamodelica::Ref<DAE::Exp>, metamodelica::Ref<DAE::ElementSource>),
                    ) -> Result<ArcStr>
                    + 'static,
            >,
        ),
    ),
) -> Result<(
    metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
    (
        i32,
        i32,
        metamodelica::Array<
            Option<(
                metamodelica::Ref<DAE::ComponentRef>,
                (metamodelica::Ref<DAE::Exp>, metamodelica::Ref<DAE::ElementSource>),
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
        Arc<
            dyn ::std::ops::Fn((metamodelica::Ref<DAE::Exp>, metamodelica::Ref<DAE::ElementSource>)) -> Result<ArcStr>
                + 'static,
        >,
    ),
)> {
    '__tco: loop {
        ::match_deref::match_deref! { match &(elseenqs) {
            Deref @ metamodelica::ListNode::Nil => {
                return Ok(iHt.clone())
            },
            Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Element::EQUEQUATION { cr1: cr, cr2, source }, tail: rest } if (!(BaseHashTable::hasKey(cr.clone(), &iHt)?)) => {
                let mut e: metamodelica::Ref<DAE::Exp>;
                let mut ht: (metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>, (i32, i32, metamodelica::Array<Option<(metamodelica::Ref<DAE::ComponentRef>, (metamodelica::Ref<DAE::Exp>, metamodelica::Ref<DAE::ElementSource>))>>), i32, (HashTableCrToExpSourceTpl::FuncHashCref, HashTableCrToExpSourceTpl::FuncCrefEqual, HashTableCrToExpSourceTpl::FuncCrefStr, HashTableCrToExpSourceTpl::FuncExpStr));
                e = Expression::crefExp(cr2.clone())?;
                let false = (Expression::expHasCrefNoPreorDer(e.clone(), cr.clone())?) else { return Err("pattern mismatch") };
                ht = BaseHashTable::add((cr.clone(), (e, source.clone())), iHt.clone())?;
                { (elseenqs, functionTree, iHt) = (rest.clone(), functionTree, ht); continue '__tco; }
            },
            Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Element::DEFINE { componentRef: cr, exp: e, source }, tail: rest } if (!(BaseHashTable::hasKey(cr.clone(), &iHt)?)) => {
                let mut ht: (metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>, (i32, i32, metamodelica::Array<Option<(metamodelica::Ref<DAE::ComponentRef>, (metamodelica::Ref<DAE::Exp>, metamodelica::Ref<DAE::ElementSource>))>>), i32, (HashTableCrToExpSourceTpl::FuncHashCref, HashTableCrToExpSourceTpl::FuncCrefEqual, HashTableCrToExpSourceTpl::FuncCrefStr, HashTableCrToExpSourceTpl::FuncExpStr));
                let false = (Expression::expHasCrefNoPreorDer(e.clone(), cr.clone())?) else { return Err("pattern mismatch") };
                ht = BaseHashTable::add((cr.clone(), (e.clone(), source.clone())), iHt.clone())?;
                { (elseenqs, functionTree, iHt) = (rest.clone(), functionTree, ht); continue '__tco; }
            },
            Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Element::EQUATION { exp: Deref @ DAE::Exp::CREF { componentRef: cr, .. }, scalar: e, source }, tail: rest } if (!(BaseHashTable::hasKey(cr.clone(), &iHt)?)) => {
                let mut ht: (metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>, (i32, i32, metamodelica::Array<Option<(metamodelica::Ref<DAE::ComponentRef>, (metamodelica::Ref<DAE::Exp>, metamodelica::Ref<DAE::ElementSource>))>>), i32, (HashTableCrToExpSourceTpl::FuncHashCref, HashTableCrToExpSourceTpl::FuncCrefEqual, HashTableCrToExpSourceTpl::FuncCrefStr, HashTableCrToExpSourceTpl::FuncExpStr));
                let false = (Expression::expHasCrefNoPreorDer(e.clone(), cr.clone())?) else { return Err("pattern mismatch") };
                ht = BaseHashTable::add((cr.clone(), (e.clone(), source.clone())), iHt.clone())?;
                { (elseenqs, functionTree, iHt) = (rest.clone(), functionTree, ht); continue '__tco; }
            },
            Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Element::COMPLEX_EQUATION { lhs: Deref @ DAE::Exp::CREF { componentRef: cr, .. }, rhs: e, source }, tail: rest } if (!(BaseHashTable::hasKey(cr.clone(), &iHt)?)) => {
                let mut ht: (metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>, (i32, i32, metamodelica::Array<Option<(metamodelica::Ref<DAE::ComponentRef>, (metamodelica::Ref<DAE::Exp>, metamodelica::Ref<DAE::ElementSource>))>>), i32, (HashTableCrToExpSourceTpl::FuncHashCref, HashTableCrToExpSourceTpl::FuncCrefEqual, HashTableCrToExpSourceTpl::FuncCrefStr, HashTableCrToExpSourceTpl::FuncExpStr));
                let false = (Expression::expHasCrefNoPreorDer(e.clone(), cr.clone())?) else { return Err("pattern mismatch") };
                ht = BaseHashTable::add((cr.clone(), (e.clone(), source.clone())), iHt.clone())?;
                { (elseenqs, functionTree, iHt) = (rest.clone(), functionTree, ht); continue '__tco; }
            },
            Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Element::ARRAY_EQUATION { exp: Deref @ DAE::Exp::CREF { componentRef: cr, .. }, array: e, source, .. }, tail: rest } if (!(BaseHashTable::hasKey(cr.clone(), &iHt)?)) => {
                let mut ht: (metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>, (i32, i32, metamodelica::Array<Option<(metamodelica::Ref<DAE::ComponentRef>, (metamodelica::Ref<DAE::Exp>, metamodelica::Ref<DAE::ElementSource>))>>), i32, (HashTableCrToExpSourceTpl::FuncHashCref, HashTableCrToExpSourceTpl::FuncCrefEqual, HashTableCrToExpSourceTpl::FuncCrefStr, HashTableCrToExpSourceTpl::FuncExpStr));
                let false = (Expression::expHasCrefNoPreorDer(e.clone(), cr.clone())?) else { return Err("pattern mismatch") };
                ht = BaseHashTable::add((cr.clone(), (e.clone(), source.clone())), iHt.clone())?;
                { (elseenqs, functionTree, iHt) = (rest.clone(), functionTree, ht); continue '__tco; }
            },
            Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Element::IF_EQUATION { condition1: expl, equations2: eqnslst, equations3: eqns, .. }, tail: rest } => {
                let mut ht: (metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>, (i32, i32, metamodelica::Array<Option<(metamodelica::Ref<DAE::ComponentRef>, (metamodelica::Ref<DAE::Exp>, metamodelica::Ref<DAE::ElementSource>))>>), i32, (HashTableCrToExpSourceTpl::FuncHashCref, HashTableCrToExpSourceTpl::FuncCrefEqual, HashTableCrToExpSourceTpl::FuncCrefStr, HashTableCrToExpSourceTpl::FuncExpStr));
                ht = lowerWhenIfEqnsElse(eqns.clone(), functionTree, iHt.clone())?;
                ht = lowerWhenIfEqns(&(expl.clone().reverse()), &(eqnslst.clone().reverse()), functionTree, ht)?;
                { (elseenqs, functionTree, iHt) = (rest.clone(), functionTree, ht); continue '__tco; }
            },
            _ => return Err("match: no arm matched"),
        } }
    }
}

fn mergeWhenEqns(
    mut trueEqnList: &metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
    mut elseEqnList: &metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
    mut inEquationLst: &metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
) -> Result<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>> {
    let mut outEquationLst: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
    outEquationLst = 'mc: {
        let __mc_input = (&**trueEqnList, &**elseEqnList, &**inEquationLst);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Nil, Deref @ metamodelica::ListNode::Nil, _) => {
                    Ok(inEquationLst.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Deref @ metamodelica::ListNode::Nil, Deref @ metamodelica::ListNode::Nil) => {
                    Ok(trueEqnList.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Nil, _, Deref @ metamodelica::ListNode::Nil) => {
                    Ok(elseEqnList.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Nil, _, _) => {
                    Ok(listAppend(inEquationLst.clone(), elseEqnList.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Deref @ metamodelica::ListNode::Nil, _) => {
                    Ok(listAppend(inEquationLst.clone(), trueEqnList.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                        (Deref @ metamodelica::ListNode::Cons { head: inEqn @ Deref @ BackendDAE::Equation::WHEN_EQUATION { size, whenEquation: whenEq @ Deref @ BackendDAE::WhenEquation { condition: cond, whenStmtLst, elsewhenPart: whenElsePart }, source, attr }, tail: trueEqns }, _, _) => {
                            let mut res: metamodelica::Ref<BackendDAE::Equation> = metamodelica::Ref::new(BackendDAE::Equation::DUMMY_EQUATION);
                            let mut elseEqnsRest: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
                            let mut result: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
                            let mut whenEqRes: metamodelica::Ref<BackendDAE::WhenEquation> = <metamodelica::Ref<BackendDAE::WhenEquation> as ::std::default::Default>::default();
                            let mut added: bool;
                            result = inEquationLst.clone();
                            elseEqnsRest = metamodelica::nil();
                            added = false;
                            for mut eqn in &**elseEqnList {
                                let () = (::match_deref::match_deref! { match &(eqn.clone()) {
                Deref @ BackendDAE::Equation::WHEN_EQUATION { whenEquation: eq @ Deref @ BackendDAE::WhenEquation { whenStmtLst: whenStmtLst2, .. }, .. } => {
                            for mut elem in &*whenStmtLst.clone() {
                                let () = (match elem.clone() {
                BackendDAE::WhenOperator::ASSIGN { left: ref eleft, .. } => {
                            for mut stmt in &*whenStmtLst2.clone() {
                                let () = 'mc: {
                let __mc_input = stmt.clone();
                if let Ok((__v, __wb0, __wb1)) = (|| -> Result<_> {
                            let BackendDAE::WhenOperator::ASSIGN { left: ref eleft2, .. } = __mc_input.clone() else { return Err("nomatch") };
                            let mut added: bool = added.clone();
                            let mut res: metamodelica::Ref<BackendDAE::Equation>;
                            let mut result: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>> = result.clone();
                            let mut whenEqRes: metamodelica::Ref<BackendDAE::WhenEquation>;
                            let true = (ExpressionBasics::expEqual(metamodelica::AsArg::as_arg(&eleft), eleft2.clone())?) else { return Err("pattern mismatch") };
                            whenEqRes = BackendEquation::setWhenElsePart(metamodelica::AsArg::as_arg(&whenEq), metamodelica::AsArg::as_arg(&eq))?;
                            res = metamodelica::Ref::new(BackendDAE::Equation::WHEN_EQUATION { size: size.clone(), whenEquation: whenEqRes.clone(), source: source.clone(), attr: attr.clone() });
                            result = metamodelica::cons(res.clone(), result.clone());
                            added = true;
                            Ok(((), added.clone(), result.clone()))
                })() { added = __wb0; result = __wb1; break 'mc __v; }
                if let Ok((__v, __wb0)) = (|| -> Result<_> {
                            let _ = __mc_input.clone() else { return Err("nomatch") };
                            let mut elseEqnsRest: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>> = elseEqnsRest.clone();
                            elseEqnsRest = metamodelica::cons(eqn.clone(), elseEqnsRest.clone());
                            Ok(((), elseEqnsRest.clone()))
                })() { elseEqnsRest = __wb0; break 'mc __v; }
                return Err("matchcontinue: no arm matched")
            };
                            }
                            ()
                },
                BackendDAE::WhenOperator::REINIT { stateVar: ref crleft, .. } => {
                            for mut stmt in &*whenStmtLst2.clone() {
                                let () = 'mc: {
                let __mc_input = stmt.clone();
                if let Ok((__v, __wb0, __wb1)) = (|| -> Result<_> {
                            let BackendDAE::WhenOperator::REINIT { stateVar: ref crleft2, .. } = __mc_input.clone() else { return Err("nomatch") };
                            let mut added: bool = added.clone();
                            let mut res: metamodelica::Ref<BackendDAE::Equation>;
                            let mut result: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>> = result.clone();
                            let mut whenEqRes: metamodelica::Ref<BackendDAE::WhenEquation>;
                            let true = (ComponentReferenceBasics::crefEqualNoStringCompare(metamodelica::AsArg::as_arg(&crleft), &(crleft2.clone()))?) else { return Err("pattern mismatch") };
                            whenEqRes = BackendEquation::setWhenElsePart(metamodelica::AsArg::as_arg(&whenEq), metamodelica::AsArg::as_arg(&eq))?;
                            res = metamodelica::Ref::new(BackendDAE::Equation::WHEN_EQUATION { size: size.clone(), whenEquation: whenEqRes.clone(), source: source.clone(), attr: attr.clone() });
                            result = metamodelica::cons(res.clone(), result.clone());
                            added = true;
                            Ok(((), added.clone(), result.clone()))
                })() { added = __wb0; result = __wb1; break 'mc __v; }
                if let Ok((__v, __wb0)) = (|| -> Result<_> {
                            let _ = __mc_input.clone() else { return Err("nomatch") };
                            let mut elseEqnsRest: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>> = elseEqnsRest.clone();
                            elseEqnsRest = metamodelica::cons(eqn.clone(), elseEqnsRest.clone());
                            Ok(((), elseEqnsRest.clone()))
                })() { elseEqnsRest = __wb0; break 'mc __v; }
                return Err("matchcontinue: no arm matched")
            };
                            }
                            ()
                },
                _ => {
                            whenEqRes = BackendEquation::setWhenElsePart(metamodelica::AsArg::as_arg(&whenEq), metamodelica::AsArg::as_arg(&eq))?;
                            res = metamodelica::Ref::new(BackendDAE::Equation::WHEN_EQUATION { size: size.clone(), whenEquation: whenEqRes.clone(), source: source.clone(), attr: attr.clone() });
                            result = metamodelica::cons(res.clone(), result.clone());
                            added = true;
                            ()
                },
            });
                            }
                            ()
                },
                _ => {
                            res = metamodelica::Ref::new(BackendDAE::Equation::WHEN_EQUATION { size: size.clone(), whenEquation: metamodelica::Ref::new(BackendDAE::WhenEquation { condition: cond.clone(), whenStmtLst: whenStmtLst.clone(), elsewhenPart: whenElsePart.clone() }), source: source.clone(), attr: attr.clone() });
                            result = metamodelica::cons(res.clone(), result.clone());
                            ()
                },
                _ => unreachable!("match_deref! exhaustiveness placeholder"),
            } });
                            }
                            if !(added) {
                                result = metamodelica::cons(inEqn.clone(), result.clone());
                            }
                            result = mergeWhenEqns(metamodelica::AsArg::as_arg(&trueEqns), &elseEqnsRest, &result)?;
                            Ok(result.clone())
                        }
                        _ => return Err("nomatch"),
                    }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Error::addMessage(Error::INTERNAL_ERROR.clone(), list![literal!("BackendDAECreate.mergeWhenEqns: Error in mergeWhenEqns.")])?;
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(outEquationLst)
}

fn lowerTupleAssignment<'__b>(
    mut target_expl: &'__b metamodelica::List<metamodelica::Ref<DAE::Exp>>,
    mut source_expl: &'__b metamodelica::List<metamodelica::Ref<DAE::Exp>>,
    mut inEq_source: metamodelica::Ref<DAE::ElementSource>,
    mut funcs: &'__b metamodelica::Ref<AvlTreePathFunction::Tree>,
    mut iEqns: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
) -> Result<metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>> {
    '__tco: loop {
        ::match_deref::match_deref! { match (target_expl, source_expl) {
            (Deref @ metamodelica::ListNode::Nil, Deref @ metamodelica::ListNode::Nil) => {
                return Ok(iEqns)
            },
            (Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Exp::CREF { componentRef: Deref @ DAE::ComponentRef::WILD { .. }, .. }, tail: rest_targets }, Deref @ metamodelica::ListNode::Cons { head: _, tail: rest_sources }) => {
                { (target_expl, source_expl, inEq_source, funcs, iEqns) = (rest_targets, rest_sources, inEq_source, funcs, iEqns); continue '__tco; }
            },
            (Deref @ metamodelica::ListNode::Cons { head: target, tail: rest_targets }, Deref @ metamodelica::ListNode::Cons { head: source, tail: rest_sources }) => {
                let mut eqns: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
                let mut eq_source: metamodelica::Ref<DAE::ElementSource>;
                let mut target = (*target).clone();
                let mut source = (*source).clone();
                let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(ExpressionSimplify::simplifyAddSymbolicOperation(metamodelica::Ref::new(DAE::EquationExp::EQUALITY_EXPS { lhs: target.clone(), rhs: source.clone() }), inEq_source.clone())?) {
                    (Deref @ DAE::EquationExp::EQUALITY_EXPS { lhs: __pa0, rhs: __pa1 }, __pa2) => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
                    _ => return Err("pattern mismatch"),
                } };
                target = metamodelica::Own::own(__pa0);
                source = metamodelica::Own::own(__pa1);
                eq_source = metamodelica::Own::own(__pa2);
                eqns = lowerExtendedRecordEqn(target.clone(), source.clone(), inEq_source, BackendDAE::EQ_ATTR_DEFAULT_UNKNOWN.clone(), funcs, iEqns)?;
                { (target_expl, source_expl, inEq_source, funcs, iEqns) = (rest_targets, rest_sources, eq_source, funcs, eqns); continue '__tco; }
            },
            _ => return Err("match: no arm matched"),
        } }
    }
}

/*
 *   lower algorithms
 */
fn lowerAlgorithm(
    mut inElement: metamodelica::Ref<DAE::Element>,
    mut functionTree: &metamodelica::Ref<AvlTreePathFunction::Tree>,
    mut inEquations: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
    mut inREquations: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
    mut inIEquations: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
    mut inCrefExpansion: DAE::Expand,
    mut inInitialization: bool,
) -> Result<(
    metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
    metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
    metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
)> {
    let mut outEquations: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
    let mut outREquations: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
    let mut outIEquations: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
    (outEquations, outREquations, outIEquations) = ({
        let mut eqAttributes: BackendDAE::EquationAttributes = if (inInitialization) {
            BackendDAE::EQ_ATTR_DEFAULT_INITIAL.clone()
        } else {
            BackendDAE::EQ_ATTR_DEFAULT_DYNAMIC.clone()
        };
        'mc: {
            let __mc_input = &*inElement;
            if let Ok(__v) = (|| -> Result<_> {
                ::match_deref::match_deref! { match &__mc_input {
                    Deref @ DAE::Element::ALGORITHM { algorithm_: Deref @ DAE::Algorithm { statementLst: Deref @ metamodelica::ListNode::Nil }, .. } => {
                        Ok((inEquations.clone(), inREquations.clone(), inIEquations.clone()))
                    }
                    _ => return Err("nomatch"),
                }}
            })() {
                break 'mc __v;
            }
            if let Ok(__v) = (|| -> Result<_> {
                ::match_deref::match_deref! { match &__mc_input {
                    Deref @ DAE::Element::INITIALALGORITHM { algorithm_: Deref @ DAE::Algorithm { statementLst: Deref @ metamodelica::ListNode::Nil }, .. } => {
                        Ok((inEquations.clone(), inREquations.clone(), inIEquations.clone()))
                    }
                    _ => return Err("nomatch"),
                }}
            })() {
                break 'mc __v;
            }
            if let Ok(__v) = (|| -> Result<_> {
                ::match_deref::match_deref! { match &__mc_input {
                    Deref @ DAE::Element::ALGORITHM { algorithm_: alg, source } => {
                        let mut size: i32;
                        let mut crefLst: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
                        let mut eqns: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
                        let mut reqns: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
                        let mut ieqns: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
                        crefLst = CheckModel::checkAndGetAlgorithmOutputs(metamodelica::AsArg::as_arg(&alg), metamodelica::AsArg::as_arg(&source), inCrefExpansion)?;
                        size = ((crefLst).len() as i32);
                        if inInitialization {
                            ieqns = metamodelica::cons(metamodelica::Ref::new(BackendDAE::Equation::ALGORITHM { size: size, alg: alg.clone(), source: source.clone(), expand: inCrefExpansion, attr: eqAttributes }), inIEquations.clone());
                            eqns = inEquations.clone();
                            reqns = inREquations.clone();
                        } else {
                            if size > 0 {
                                eqns = metamodelica::cons(metamodelica::Ref::new(BackendDAE::Equation::ALGORITHM { size: size, alg: alg.clone(), source: source.clone(), expand: inCrefExpansion, attr: eqAttributes }), inEquations.clone());
                                reqns = inREquations.clone();
                            } else {
                                eqns = inEquations.clone();
                                reqns = metamodelica::cons(metamodelica::Ref::new(BackendDAE::Equation::ALGORITHM { size: size, alg: alg.clone(), source: source.clone(), expand: inCrefExpansion, attr: eqAttributes }), inREquations.clone());
                            }
                            ieqns = inIEquations.clone();
                        }
                        Ok((eqns.clone(), reqns.clone(), ieqns.clone()))
                    }
                    _ => return Err("nomatch"),
                }}
            })() {
                break 'mc __v;
            }
            if let Ok(__v) = (|| -> Result<_> {
                ::match_deref::match_deref! { match &__mc_input {
                    Deref @ DAE::Element::INITIALALGORITHM { algorithm_: alg, source } => {
                        let mut size: i32;
                        let mut crefLst: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
                        crefLst = CheckModel::checkAndGetAlgorithmOutputs(metamodelica::AsArg::as_arg(&alg), metamodelica::AsArg::as_arg(&source), inCrefExpansion)?;
                        size = ((crefLst).len() as i32);
                        Ok((inEquations.clone(), inREquations.clone(), metamodelica::cons(metamodelica::Ref::new(BackendDAE::Equation::ALGORITHM { size: size, alg: alg.clone(), source: source.clone(), expand: inCrefExpansion, attr: eqAttributes }), inIEquations.clone())))
                    }
                    _ => return Err("nomatch"),
                }}
            })() {
                break 'mc __v;
            }
            if let Ok(__v) = (|| -> Result<_> {
                ::match_deref::match_deref! { match &__mc_input {
                    Deref @ DAE::Element::ASSERT { condition: Deref @ DAE::Exp::BCONST { bool: true }, .. } => {
                        Ok((inEquations.clone(), inREquations.clone(), inIEquations.clone()))
                    }
                    _ => return Err("nomatch"),
                }}
            })() {
                break 'mc __v;
            }
            if let Ok(__v) = (|| -> Result<_> {
                ::match_deref::match_deref! { match &__mc_input {
                    Deref @ DAE::Element::INITIAL_ASSERT { condition: Deref @ DAE::Exp::BCONST { bool: true }, .. } => {
                        Ok((inEquations.clone(), inREquations.clone(), inIEquations.clone()))
                    }
                    _ => return Err("nomatch"),
                }}
            })() {
                break 'mc __v;
            }
            if let Ok(__v) = (|| -> Result<_> {
                ::match_deref::match_deref! { match &__mc_input {
                    Deref @ DAE::Element::ASSERT { condition: cond, message: msg, level, source } => {
                        let mut alg: metamodelica::Ref<DAE::Algorithm>;
                        let mut reqns: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
                        let mut ieqns: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
                        BackendDAEUtil::checkAssertCondition(metamodelica::AsArg::as_arg(&cond), msg.clone(), metamodelica::AsArg::as_arg(&level), &(ElementSource::getElementSourceFileInfo(source.clone())))?;
                        alg = metamodelica::Ref::new(DAE::Algorithm { statementLst: list![metamodelica::Ref::new(DAE::Statement::STMT_ASSERT { cond: cond.clone(), msg: msg.clone(), level: level.clone(), source: source.clone() })] });
                        if inInitialization {
                            reqns = inREquations.clone();
                            ieqns = metamodelica::cons(metamodelica::Ref::new(BackendDAE::Equation::ALGORITHM { size: 0, alg: alg.clone(), source: source.clone(), expand: inCrefExpansion, attr: eqAttributes }), inIEquations.clone());
                        } else {
                            reqns = metamodelica::cons(metamodelica::Ref::new(BackendDAE::Equation::ALGORITHM { size: 0, alg: alg.clone(), source: source.clone(), expand: inCrefExpansion, attr: eqAttributes }), inREquations.clone());
                            ieqns = inIEquations.clone();
                        }
                        Ok((inEquations.clone(), reqns.clone(), ieqns.clone()))
                    }
                    _ => return Err("nomatch"),
                }}
            })() {
                break 'mc __v;
            }
            if let Ok(__v) = (|| -> Result<_> {
                ::match_deref::match_deref! { match &__mc_input {
                    Deref @ DAE::Element::INITIAL_ASSERT { condition: cond, message: msg, level, source } => {
                        let mut alg: metamodelica::Ref<DAE::Algorithm>;
                        BackendDAEUtil::checkAssertCondition(metamodelica::AsArg::as_arg(&cond), msg.clone(), metamodelica::AsArg::as_arg(&level), &(ElementSource::getElementSourceFileInfo(source.clone())))?;
                        alg = metamodelica::Ref::new(DAE::Algorithm { statementLst: list![metamodelica::Ref::new(DAE::Statement::STMT_ASSERT { cond: cond.clone(), msg: msg.clone(), level: level.clone(), source: source.clone() })] });
                        Ok((inEquations.clone(), inREquations.clone(), metamodelica::cons(metamodelica::Ref::new(BackendDAE::Equation::ALGORITHM { size: 0, alg: alg.clone(), source: source.clone(), expand: inCrefExpansion, attr: BackendDAE::EQ_ATTR_DEFAULT_INITIAL.clone() }), inIEquations.clone())))
                    }
                    _ => return Err("nomatch"),
                }}
            })() {
                break 'mc __v;
            }
            if let Ok(__v) = (|| -> Result<_> {
                ::match_deref::match_deref! { match &__mc_input {
                    Deref @ DAE::Element::TERMINATE { message: msg, source } => {
                        Ok((inEquations.clone(), metamodelica::cons(metamodelica::Ref::new(BackendDAE::Equation::ALGORITHM { size: 0, alg: metamodelica::Ref::new(DAE::Algorithm { statementLst: list![metamodelica::Ref::new(DAE::Statement::STMT_TERMINATE { msg: msg.clone(), source: source.clone() })] }), source: source.clone(), expand: inCrefExpansion, attr: eqAttributes }), inREquations.clone()), inIEquations.clone()))
                    }
                    _ => return Err("nomatch"),
                }}
            })() {
                break 'mc __v;
            }
            if let Ok(__v) = (|| -> Result<_> {
                ::match_deref::match_deref! { match &__mc_input {
                    Deref @ DAE::Element::INITIAL_TERMINATE { message: msg, source } => {
                        Ok((inEquations.clone(), inREquations.clone(), metamodelica::cons(metamodelica::Ref::new(BackendDAE::Equation::ALGORITHM { size: 0, alg: metamodelica::Ref::new(DAE::Algorithm { statementLst: list![metamodelica::Ref::new(DAE::Statement::STMT_TERMINATE { msg: msg.clone(), source: source.clone() })] }), source: source.clone(), expand: inCrefExpansion, attr: eqAttributes }), inIEquations.clone())))
                    }
                    _ => return Err("nomatch"),
                }}
            })() {
                break 'mc __v;
            }
            if let Ok(__v) = (|| -> Result<_> {
                ::match_deref::match_deref! { match &__mc_input {
                    Deref @ DAE::Element::NORETCALL { exp: e, source } => {
                        let mut alg: metamodelica::Ref<DAE::Algorithm>;
                        alg = metamodelica::Ref::new(DAE::Algorithm { statementLst: list![metamodelica::Ref::new(DAE::Statement::STMT_NORETCALL { exp: e.clone(), source: source.clone() })] });
                        Ok((inEquations.clone(), metamodelica::cons(metamodelica::Ref::new(BackendDAE::Equation::ALGORITHM { size: 0, alg: alg.clone(), source: source.clone(), expand: inCrefExpansion, attr: eqAttributes }), inREquations.clone()), inIEquations.clone()))
                    }
                    _ => return Err("nomatch"),
                }}
            })() {
                break 'mc __v;
            }
            if let Ok(__v) = (|| -> Result<_> {
                ::match_deref::match_deref! { match &__mc_input {
                    Deref @ DAE::Element::INITIAL_NORETCALL { exp: e, source } => {
                        let mut alg: metamodelica::Ref<DAE::Algorithm>;
                        alg = metamodelica::Ref::new(DAE::Algorithm { statementLst: list![metamodelica::Ref::new(DAE::Statement::STMT_NORETCALL { exp: e.clone(), source: source.clone() })] });
                        Ok((inEquations.clone(), inREquations.clone(), metamodelica::cons(metamodelica::Ref::new(BackendDAE::Equation::ALGORITHM { size: 0, alg: alg.clone(), source: source.clone(), expand: inCrefExpansion, attr: eqAttributes }), inIEquations.clone())))
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
                        let 0 = (Error::getNumErrorMessages()) else { return Err("pattern mismatch") };
                        r#str = { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("BackendDAECreate.lowerAlgorithm failed for:\n")); __mm_s.push_str(&*DAEDump::dumpElementsStr(&(list![inElement.clone()]))?); ArcStr::from(__mm_s) };
                        Error::addSourceMessage(&(Error::INTERNAL_ERROR.clone()), list![r#str.clone()], &(ElementSource::getElementSourceFileInfo(ElementSource::getElementSource(&inElement)?)))?;
                        Ok(return Err("fail"))
                    }
                    _ => return Err("nomatch"),
                }}
            })() {
                break 'mc __v;
            }
            return Err("matchcontinue: no arm matched");
        }
    });
    Ok((outEquations, outREquations, outIEquations))
}

/*
 *  alias Equations
 */
fn handleAliasEquations(
    mut iAliasEqns: &metamodelica::List<metamodelica::Ref<DAE::Element>>,
    mut iVars: BackendDAE::Variables,
    mut inGlobalKnownVars: BackendDAE::Variables,
    mut iExtVars: BackendDAE::Variables,
    mut iAVars: BackendDAE::Variables,
    mut iEqns: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
    mut iREqns: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
    mut iIEqns: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
) -> Result<(
    BackendDAE::Variables,
    BackendDAE::Variables,
    BackendDAE::Variables,
    BackendDAE::Variables,
    metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
    metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
    metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
)> {
    let mut oVars: BackendDAE::Variables;
    let mut outGlobalKnownVars: BackendDAE::Variables;
    let mut oExtVars: BackendDAE::Variables;
    let mut oAVars: BackendDAE::Variables;
    let mut oEqns: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
    let mut oREqns: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
    let mut oIEqns: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
    (oVars, outGlobalKnownVars, oExtVars, oAVars, oEqns, oREqns, oIEqns) = (::match_deref::match_deref! { match iAliasEqns {
        Deref @ metamodelica::ListNode::Nil => {
            (iVars, inGlobalKnownVars, iExtVars, iAVars, iEqns, iREqns, iIEqns)
        },
        _ => {
            let mut vars: BackendDAE::Variables;
            let mut globalKnownVars: BackendDAE::Variables;
            let mut extvars: BackendDAE::Variables;
            let mut avars: BackendDAE::Variables;
            let mut eqns: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
            let mut reqns: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
            let mut ieqns: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
            (vars, globalKnownVars, extvars, avars, eqns, reqns, ieqns) = handleAliasEquations1(iAliasEqns, iVars, inGlobalKnownVars, iExtVars, iAVars, iEqns, iREqns, iIEqns)?;
            (vars, globalKnownVars, extvars, avars, eqns, reqns, ieqns)
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((oVars, outGlobalKnownVars, oExtVars, oAVars, oEqns, oREqns, oIEqns))
}

fn handleAliasEquations1(
    mut iAliasEqns: &metamodelica::List<metamodelica::Ref<DAE::Element>>,
    mut iVars: BackendDAE::Variables,
    mut inGlobalKnownVars: BackendDAE::Variables,
    mut iExtVars: BackendDAE::Variables,
    mut iAVars: BackendDAE::Variables,
    mut iEqns: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
    mut iREqns: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
    mut iIEqns: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
) -> Result<(
    BackendDAE::Variables,
    BackendDAE::Variables,
    BackendDAE::Variables,
    BackendDAE::Variables,
    metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
    metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
    metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
)> {
    let mut oVars: BackendDAE::Variables;
    let mut outGlobalKnownVars: BackendDAE::Variables;
    let mut oExtVars: BackendDAE::Variables;
    let mut oAVars: BackendDAE::Variables;
    let mut oEqns: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
    let mut oREqns: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
    let mut oIEqns: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
    let mut repl: BackendVarTransform::VariableReplacements;
    repl = BackendVarTransform::emptyReplacements();
    (oVars, outGlobalKnownVars, oExtVars, oAVars, repl, oEqns) =
        handleAliasEquations2(iAliasEqns, iVars, inGlobalKnownVars, iExtVars, iAVars, repl, iEqns)?;
    (oAVars, _) = BackendVariable::traverseBackendDAEVarsWithUpdate(
        oAVars,
        (std::sync::Arc::new(fnptr!(
            replaceAliasVarTraverser,
            metamodelica::Ref<BackendDAE::Var>,
            BackendVarTransform::VariableReplacements
        ))
            as std::sync::Arc<
                dyn ::std::ops::Fn(
                        metamodelica::Ref<BackendDAE::Var>,
                        BackendVarTransform::VariableReplacements,
                    ) -> Result<(
                        metamodelica::Ref<BackendDAE::Var>,
                        BackendVarTransform::VariableReplacements,
                    )> + 'static,
            >),
        repl.clone(),
    )?;
    oVars = BackendVariable::rehashVariables(oVars)?;
    (oEqns, _) = BackendVarTransform::replaceEquations(oEqns, &repl, None)?;
    (oREqns, _) = BackendVarTransform::replaceEquations(iREqns, &repl, None)?;
    (oIEqns, _) = BackendVarTransform::replaceEquations(iIEqns, &repl, None)?;
    Ok((oVars, outGlobalKnownVars, oExtVars, oAVars, oEqns, oREqns, oIEqns))
}

fn replaceAliasVarTraverser(
    mut inVar: metamodelica::Ref<BackendDAE::Var>,
    mut inRepl: BackendVarTransform::VariableReplacements,
) -> (
    metamodelica::Ref<BackendDAE::Var>,
    BackendVarTransform::VariableReplacements,
) {
    let mut outVar: metamodelica::Ref<BackendDAE::Var>;
    let mut repl: BackendVarTransform::VariableReplacements;
    (outVar, repl) = 'mc: {
        let __mc_input = (inVar.clone(), inRepl.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (v @ Deref @ BackendDAE::Var { bindExp: Some(e), .. }, repl) => {
                    let mut v1: metamodelica::Ref<BackendDAE::Var>;
                    let mut e1: metamodelica::Ref<DAE::Exp>;
                    let mut b: bool;
                    let __pa0 = ::match_deref::match_deref! { match &(BackendVarTransform::replaceExp(metamodelica::AsArg::as_arg(&e), metamodelica::AsArg::as_arg(&repl), None)) {
                        (__pa0, true) => __pa0.clone(),
                        _ => return Err("pattern mismatch"),
                    } };
                    e1 = metamodelica::Own::own(__pa0);
                    b = Expression::isConst(e1.clone())?;
                    v1 = if (!(b)) {BackendVariable::setBindExp(v.clone(), Some(e1.clone()))} else {v.clone()};
                    Ok((v1.clone(), repl.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((inVar.clone(), inRepl.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    (outVar, repl)
}

fn handleAliasEquations2<'__b>(
    mut iAliasEqns: &'__b metamodelica::List<metamodelica::Ref<DAE::Element>>,
    mut iVars: BackendDAE::Variables,
    mut inGlobalKnownVars: BackendDAE::Variables,
    mut iExtVars: BackendDAE::Variables,
    mut iAVars: BackendDAE::Variables,
    mut iRepl: BackendVarTransform::VariableReplacements,
    mut iEqns: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
) -> Result<(
    BackendDAE::Variables,
    BackendDAE::Variables,
    BackendDAE::Variables,
    BackendDAE::Variables,
    BackendVarTransform::VariableReplacements,
    metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
)> {
    '__tco: loop {
        ::match_deref::match_deref! { match iAliasEqns {
            Deref @ metamodelica::ListNode::Nil => {
                return Ok((iVars, inGlobalKnownVars, iExtVars, iAVars, iRepl, iEqns))
            },
            Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Element::EQUEQUATION { cr1, cr2, source }, tail: aliaseqns } => {
                let mut vars: BackendDAE::Variables;
                let mut globalKnownVars: BackendDAE::Variables;
                let mut extvars: BackendDAE::Variables;
                let mut avars: BackendDAE::Variables;
                let mut repl: BackendVarTransform::VariableReplacements;
                let mut eqns: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
                let mut ecr1: metamodelica::Ref<DAE::Exp>;
                let mut ecr2: metamodelica::Ref<DAE::Exp>;
                ecr1 = Expression::crefExp(cr1.clone())?;
                (ecr1, _) = BackendVarTransform::replaceExp(&ecr1, &iRepl, None);
                ecr2 = Expression::crefExp(cr2.clone())?;
                (ecr2, _) = BackendVarTransform::replaceExp(&ecr2, &iRepl, None);
                (vars, globalKnownVars, extvars, avars, repl, eqns) = selectAlias(ecr1, ecr2, source.clone(), iVars, inGlobalKnownVars, iExtVars, iAVars, iRepl, iEqns)?;
                { (iAliasEqns, iVars, inGlobalKnownVars, iExtVars, iAVars, iRepl, iEqns) = (aliaseqns, vars, globalKnownVars, extvars, avars, repl, eqns); continue '__tco; }
            },
            _ => return Err("match: no arm matched"),
        } }
    }
}

fn selectAlias(
    mut exp1: metamodelica::Ref<DAE::Exp>,
    mut exp2: metamodelica::Ref<DAE::Exp>,
    mut source: metamodelica::Ref<DAE::ElementSource>,
    mut iVars: BackendDAE::Variables,
    mut inGlobalKnownVars: BackendDAE::Variables,
    mut iExtVars: BackendDAE::Variables,
    mut iAVars: BackendDAE::Variables,
    mut iRepl: BackendVarTransform::VariableReplacements,
    mut iEqns: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
) -> Result<(
    BackendDAE::Variables,
    BackendDAE::Variables,
    BackendDAE::Variables,
    BackendDAE::Variables,
    BackendVarTransform::VariableReplacements,
    metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
)> {
    let mut oVars: BackendDAE::Variables;
    let mut outGlobalKnownVars: BackendDAE::Variables;
    let mut oExtVars: BackendDAE::Variables;
    let mut oAVars: BackendDAE::Variables;
    let mut oRepl: BackendVarTransform::VariableReplacements;
    let mut oEqns: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
    (oVars, outGlobalKnownVars, oExtVars, oAVars, oRepl, oEqns) = 'mc: {
        let __mc_input = (&*exp1, &*exp2);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::ARRAY { array: explst1, .. }, Deref @ DAE::Exp::ARRAY { array: explst2, .. }) => {
                    let mut vars: BackendDAE::Variables;
                    let mut globalKnownVars: BackendDAE::Variables;
                    let mut extvars: BackendDAE::Variables;
                    let mut avars: BackendDAE::Variables;
                    let mut repl: BackendVarTransform::VariableReplacements;
                    let mut eqns: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
                    (vars, globalKnownVars, extvars, avars, repl, eqns) = selectAliasLst(metamodelica::AsArg::as_arg(&explst1), metamodelica::AsArg::as_arg(&explst2), &source, iVars.clone(), inGlobalKnownVars.clone(), iExtVars.clone(), iAVars.clone(), iRepl.clone(), iEqns.clone())?;
                    Ok((vars.clone(), globalKnownVars.clone(), extvars.clone(), avars.clone(), repl.clone(), eqns.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CREF { componentRef: cr1, ty: Deref @ DAE::Type::T_ARRAY { dims: dims1, .. } }, Deref @ DAE::Exp::ARRAY { array: explst2, .. }) => {
                    let mut vars: BackendDAE::Variables;
                    let mut globalKnownVars: BackendDAE::Variables;
                    let mut extvars: BackendDAE::Variables;
                    let mut avars: BackendDAE::Variables;
                    let mut repl: BackendVarTransform::VariableReplacements;
                    let mut eqns: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
                    let mut explst1: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                    let mut crefs1: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
                    crefs1 = ComponentReference::expandArrayCref(metamodelica::AsArg::as_arg(&cr1), dims1.clone())?;
                    explst1 = List::map(crefs1.clone(), &Expression::crefExp)?;
                    (vars, globalKnownVars, extvars, avars, repl, eqns) = selectAliasLst(&explst1, metamodelica::AsArg::as_arg(&explst2), &source, iVars.clone(), inGlobalKnownVars.clone(), iExtVars.clone(), iAVars.clone(), iRepl.clone(), iEqns.clone())?;
                    Ok((vars.clone(), globalKnownVars.clone(), extvars.clone(), avars.clone(), repl.clone(), eqns.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::ARRAY { array: explst1, .. }, Deref @ DAE::Exp::CREF { componentRef: cr2, ty: Deref @ DAE::Type::T_ARRAY { dims: dims2, .. } }) => {
                    let mut vars: BackendDAE::Variables;
                    let mut globalKnownVars: BackendDAE::Variables;
                    let mut extvars: BackendDAE::Variables;
                    let mut avars: BackendDAE::Variables;
                    let mut repl: BackendVarTransform::VariableReplacements;
                    let mut eqns: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
                    let mut explst2: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                    let mut crefs2: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
                    crefs2 = ComponentReference::expandArrayCref(metamodelica::AsArg::as_arg(&cr2), dims2.clone())?;
                    explst2 = List::map(crefs2.clone(), &Expression::crefExp)?;
                    (vars, globalKnownVars, extvars, avars, repl, eqns) = selectAliasLst(metamodelica::AsArg::as_arg(&explst1), &explst2, &source, iVars.clone(), inGlobalKnownVars.clone(), iExtVars.clone(), iAVars.clone(), iRepl.clone(), iEqns.clone())?;
                    Ok((vars.clone(), globalKnownVars.clone(), extvars.clone(), avars.clone(), repl.clone(), eqns.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CREF { componentRef: cr1, ty: Deref @ DAE::Type::T_ARRAY { dims: dims1, .. } }, Deref @ DAE::Exp::CREF { componentRef: cr2, ty: Deref @ DAE::Type::T_ARRAY { dims: dims2, .. } }) => {
                    let mut vars: BackendDAE::Variables;
                    let mut globalKnownVars: BackendDAE::Variables;
                    let mut extvars: BackendDAE::Variables;
                    let mut avars: BackendDAE::Variables;
                    let mut repl: BackendVarTransform::VariableReplacements;
                    let mut eqns: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
                    let mut explst1: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                    let mut explst2: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                    let mut crefs1: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
                    let mut crefs2: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
                    crefs1 = ComponentReference::expandArrayCref(metamodelica::AsArg::as_arg(&cr1), dims1.clone())?;
                    explst1 = List::map(crefs1.clone(), &Expression::crefExp)?;
                    crefs2 = ComponentReference::expandArrayCref(metamodelica::AsArg::as_arg(&cr2), dims2.clone())?;
                    explst2 = List::map(crefs2.clone(), &Expression::crefExp)?;
                    (vars, globalKnownVars, extvars, avars, repl, eqns) = selectAliasLst(&explst1, &explst2, &source, iVars.clone(), inGlobalKnownVars.clone(), iExtVars.clone(), iAVars.clone(), iRepl.clone(), iEqns.clone())?;
                    Ok((vars.clone(), globalKnownVars.clone(), extvars.clone(), avars.clone(), repl.clone(), eqns.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::MATRIX { matrix: explstlst1, .. }, Deref @ DAE::Exp::MATRIX { matrix: explstlst2, .. }) => {
                    let mut vars: BackendDAE::Variables;
                    let mut globalKnownVars: BackendDAE::Variables;
                    let mut extvars: BackendDAE::Variables;
                    let mut avars: BackendDAE::Variables;
                    let mut repl: BackendVarTransform::VariableReplacements;
                    let mut eqns: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
                    (vars, globalKnownVars, extvars, avars, repl, eqns) = selectAliasLst(&(List::flatten(explstlst1.clone())?), &(List::flatten(explstlst2.clone())?), &source, iVars.clone(), inGlobalKnownVars.clone(), iExtVars.clone(), iAVars.clone(), iRepl.clone(), iEqns.clone())?;
                    Ok((vars.clone(), globalKnownVars.clone(), extvars.clone(), avars.clone(), repl.clone(), eqns.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CREF { componentRef: cr1, .. }, Deref @ DAE::Exp::CREF { componentRef: cr2, .. }) => {
                    let mut vars: BackendDAE::Variables;
                    let mut globalKnownVars: BackendDAE::Variables;
                    let mut extvars: BackendDAE::Variables;
                    let mut avars: BackendDAE::Variables;
                    let mut repl: BackendVarTransform::VariableReplacements;
                    let mut arrayTyp1: i32;
                    let mut arrayTyp2: i32;
                    let mut i1: i32;
                    let mut i2: i32;
                    let mut v1: metamodelica::Ref<BackendDAE::Var>;
                    let mut v2: metamodelica::Ref<BackendDAE::Var>;
                    (v1, i1, arrayTyp1) = getVar(metamodelica::AsArg::as_arg(&cr1), &iVars, &inGlobalKnownVars, &iExtVars)?;
                    (v2, i2, arrayTyp2) = getVar(metamodelica::AsArg::as_arg(&cr2), &iVars, &inGlobalKnownVars, &iExtVars)?;
                    (vars, globalKnownVars, extvars, avars, repl) = selectAliasVar(v1.clone(), i1, arrayTyp1, exp1.clone(), v2.clone(), i2, arrayTyp2, exp2.clone(), &source, iVars.clone(), inGlobalKnownVars.clone(), iExtVars.clone(), iAVars.clone(), iRepl.clone())?;
                    Ok((vars.clone(), globalKnownVars.clone(), extvars.clone(), avars.clone(), repl.clone(), iEqns.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _) => {
                    let mut vars: BackendDAE::Variables;
                    let mut globalKnownVars: BackendDAE::Variables;
                    let mut extvars: BackendDAE::Variables;
                    let mut avars: BackendDAE::Variables;
                    let mut repl: BackendVarTransform::VariableReplacements;
                    let mut eqns: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
                    let mut explst1: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                    let mut explst2: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                    explst1 = Expression::splitRecord(&(exp1.clone()), &(Expression::r#typeof(exp1.clone())?))?;
                    explst2 = Expression::splitRecord(&(exp2.clone()), &(Expression::r#typeof(exp2.clone())?))?;
                    (vars, globalKnownVars, extvars, avars, repl, eqns) = selectAliasLst(&explst1, &explst2, &source, iVars.clone(), inGlobalKnownVars.clone(), iExtVars.clone(), iAVars.clone(), iRepl.clone(), iEqns.clone())?;
                    Ok((vars.clone(), globalKnownVars.clone(), extvars.clone(), avars.clone(), repl.clone(), eqns.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _) => {
                    Ok((iVars.clone(), inGlobalKnownVars.clone(), iExtVars.clone(), iAVars.clone(), iRepl.clone(), metamodelica::cons(metamodelica::Ref::new(BackendDAE::Equation::EQUATION { exp: exp1.clone(), scalar: exp2.clone(), source: source.clone(), attr: BackendDAE::EQ_ATTR_DEFAULT_DYNAMIC.clone() }), iEqns.clone())))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((oVars, outGlobalKnownVars, oExtVars, oAVars, oRepl, oEqns))
}

fn getVar(
    mut cr: &metamodelica::Ref<DAE::ComponentRef>,
    mut iVars: &BackendDAE::Variables,
    mut inGlobalKnownVars: &BackendDAE::Variables,
    mut iExtVars: &BackendDAE::Variables,
) -> Result<(metamodelica::Ref<BackendDAE::Var>, i32, i32)> {
    let mut oVar: metamodelica::Ref<BackendDAE::Var>;
    let mut index: i32;
    let mut varrArray: i32;
    (oVar, index, varrArray) = 'mc: {
        let __mc_input = iExtVars.clone();
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            let mut v: metamodelica::Ref<BackendDAE::Var>;
            let mut i: i32;
            (v, i) = BackendVariable::getVarSingle(cr, iVars)?;
            Ok((v.clone(), i, 1))
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            let mut v: metamodelica::Ref<BackendDAE::Var>;
            let mut i: i32;
            (v, i) = BackendVariable::getVarSingle(cr, inGlobalKnownVars)?;
            Ok((v.clone(), i, 2))
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            let mut v: metamodelica::Ref<BackendDAE::Var>;
            let mut i: i32;
            (v, i) = BackendVariable::getVarSingle(cr, iExtVars)?;
            Ok((v.clone(), i, 3))
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((oVar, index, varrArray))
}

fn selectAliasLst<'__b>(
    mut iexplst1: &'__b metamodelica::List<metamodelica::Ref<DAE::Exp>>,
    mut iexplst2: &'__b metamodelica::List<metamodelica::Ref<DAE::Exp>>,
    mut source: &'__b metamodelica::Ref<DAE::ElementSource>,
    mut iVars: BackendDAE::Variables,
    mut inGlobalKnownVars: BackendDAE::Variables,
    mut iExtVars: BackendDAE::Variables,
    mut iAVars: BackendDAE::Variables,
    mut iRepl: BackendVarTransform::VariableReplacements,
    mut iEqns: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
) -> Result<(
    BackendDAE::Variables,
    BackendDAE::Variables,
    BackendDAE::Variables,
    BackendDAE::Variables,
    BackendVarTransform::VariableReplacements,
    metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
)> {
    '__tco: loop {
        ::match_deref::match_deref! { match (iexplst1, iexplst2) {
            (Deref @ metamodelica::ListNode::Nil, Deref @ metamodelica::ListNode::Nil) => {
                return Ok((iVars, inGlobalKnownVars, iExtVars, iAVars, iRepl, iEqns))
            },
            (Deref @ metamodelica::ListNode::Cons { head: e1, tail: explst1 }, Deref @ metamodelica::ListNode::Cons { head: e2, tail: explst2 }) => {
                let mut vars: BackendDAE::Variables;
                let mut globalKnownVars: BackendDAE::Variables;
                let mut extvars: BackendDAE::Variables;
                let mut avars: BackendDAE::Variables;
                let mut repl: BackendVarTransform::VariableReplacements;
                let mut eqns: metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>;
                let mut e1 = (*e1).clone();
                let mut e2 = (*e2).clone();
                (e1, _) = BackendVarTransform::replaceExp(metamodelica::AsArg::as_arg(&e1), &iRepl, None);
                (e2, _) = BackendVarTransform::replaceExp(metamodelica::AsArg::as_arg(&e2), &iRepl, None);
                (vars, globalKnownVars, extvars, avars, repl, eqns) = selectAlias(e1.clone(), e2.clone(), source.clone(), iVars, inGlobalKnownVars, iExtVars, iAVars, iRepl, iEqns)?;
                { (iexplst1, iexplst2, source, iVars, inGlobalKnownVars, iExtVars, iAVars, iRepl, iEqns) = (explst1, explst2, source, vars, globalKnownVars, extvars, avars, repl, eqns); continue '__tco; }
            },
            _ => return Err("match: no arm matched"),
        } }
    }
}

fn selectAliasVar(
    mut v1: metamodelica::Ref<BackendDAE::Var>,
    mut index1: i32,
    mut arrayIndx1: i32,
    mut e1: metamodelica::Ref<DAE::Exp>,
    mut v2: metamodelica::Ref<BackendDAE::Var>,
    mut index2: i32,
    mut arrayIndx2: i32,
    mut e2: metamodelica::Ref<DAE::Exp>,
    mut source: &metamodelica::Ref<DAE::ElementSource>,
    mut iVars: BackendDAE::Variables,
    mut inGlobalKnownVars: BackendDAE::Variables,
    mut iExtVars: BackendDAE::Variables,
    mut iAVars: BackendDAE::Variables,
    mut iRepl: BackendVarTransform::VariableReplacements,
) -> Result<(
    BackendDAE::Variables,
    BackendDAE::Variables,
    BackendDAE::Variables,
    BackendDAE::Variables,
    BackendVarTransform::VariableReplacements,
)> {
    let mut oVars: BackendDAE::Variables;
    let mut outGlobalKnownVars: BackendDAE::Variables;
    let mut oExtVars: BackendDAE::Variables;
    let mut oAVars: BackendDAE::Variables;
    let mut oRepl: BackendVarTransform::VariableReplacements;
    (oVars, outGlobalKnownVars, oExtVars, oAVars, oRepl) = (::match_deref::match_deref! { match &((v1.clone(), arrayIndx1, v2.clone(), arrayIndx2)) {
        (Deref @ BackendDAE::Var { varKind: BackendDAE::VarKind::STATE { .. }, .. }, 1, Deref @ BackendDAE::Var { varName: cr2, .. }, 1) => {
            let mut vars: BackendDAE::Variables;
            let mut avars: BackendDAE::Variables;
            let mut repl: BackendVarTransform::VariableReplacements;
            let mut ops: metamodelica::List<metamodelica::Ref<DAE::SymbolicOperation>>;
            let mut var: metamodelica::Ref<BackendDAE::Var>;
            let mut avar: metamodelica::Ref<BackendDAE::Var>;
            let false = (BackendVariable::isStateVar(&v2)) else { return Err("pattern mismatch") };
            replaceableAlias(&v2)?;
            var = BackendVariable::mergeAliasVars(v1, v2.clone(), false, inGlobalKnownVars.clone())?;
            ops = ElementSource::getSymbolicTransformations(source);
            avar = BackendVariable::mergeVariableOperations(v2, metamodelica::cons(metamodelica::Ref::new(DAE::SymbolicOperation::SOLVED { cr: cr2.clone(), exp: e1.clone() }), ops))?;
            avar = BackendVariable::setBindExp(avar, Some(e1.clone()));
            (vars, _) = BackendVariable::removeVar(index2, iVars)?;
            avars = BackendVariable::addVar(avar, iAVars)?;
            vars = BackendVariable::addVar(var, vars)?;
            repl = BackendVarTransform::addReplacement(iRepl, cr2.clone(), e1.clone(), None)?;
            if Flags::isSet(Flags::DEBUG_ALIAS.clone())? {
                BackendDump::debugStrCrefStrExpStr(&(literal!("Alias Equation ")), metamodelica::AsArg::as_arg(&cr2), &(literal!(" = ")), e1, &(literal!(" found (4).\n")))?;
            }
            (vars, inGlobalKnownVars, iExtVars, avars, repl)
        },
        (Deref @ BackendDAE::Var { varName: cr1, .. }, 1, Deref @ BackendDAE::Var { varKind: BackendDAE::VarKind::STATE { .. }, .. }, 1) => {
            let mut vars: BackendDAE::Variables;
            let mut avars: BackendDAE::Variables;
            let mut repl: BackendVarTransform::VariableReplacements;
            let mut ops: metamodelica::List<metamodelica::Ref<DAE::SymbolicOperation>>;
            let mut var: metamodelica::Ref<BackendDAE::Var>;
            let mut avar: metamodelica::Ref<BackendDAE::Var>;
            let false = (BackendVariable::isStateVar(&v1)) else { return Err("pattern mismatch") };
            replaceableAlias(&v1)?;
            var = BackendVariable::mergeAliasVars(v2, v1.clone(), false, inGlobalKnownVars.clone())?;
            ops = ElementSource::getSymbolicTransformations(source);
            avar = BackendVariable::mergeVariableOperations(v1, metamodelica::cons(metamodelica::Ref::new(DAE::SymbolicOperation::SOLVED { cr: cr1.clone(), exp: e2.clone() }), ops))?;
            avar = BackendVariable::setBindExp(avar, Some(e2.clone()));
            (vars, _) = BackendVariable::removeVar(index1, iVars)?;
            avars = BackendVariable::addVar(avar, iAVars)?;
            vars = BackendVariable::addVar(var, vars)?;
            repl = BackendVarTransform::addReplacement(iRepl, cr1.clone(), e2.clone(), None)?;
            if Flags::isSet(Flags::DEBUG_ALIAS.clone())? {
                BackendDump::debugStrCrefStrExpStr(&(literal!("Alias Equation ")), metamodelica::AsArg::as_arg(&cr1), &(literal!(" = ")), e2, &(literal!(" found (4).\n")))?;
            }
            (vars, inGlobalKnownVars, iExtVars, avars, repl)
        },
        (Deref @ BackendDAE::Var { varName: cr1, .. }, 1, Deref @ BackendDAE::Var { varName: cr2, .. }, 1) => {
            let mut vars: BackendDAE::Variables;
            let mut avars: BackendDAE::Variables;
            let mut repl: BackendVarTransform::VariableReplacements;
            let mut ops: metamodelica::List<metamodelica::Ref<DAE::SymbolicOperation>>;
            let mut var: metamodelica::Ref<BackendDAE::Var>;
            let mut avar: metamodelica::Ref<BackendDAE::Var>;
            let mut acr: metamodelica::Ref<DAE::ComponentRef>;
            let mut w1: i32;
            let mut w2: i32;
            let mut aindx: i32;
            let mut b: bool;
            let mut b1: bool;
            let mut b2: bool;
            let mut e: metamodelica::Ref<DAE::Exp>;
            b1 = BackendVariable::isStateVar(&v1);
            b2 = BackendVariable::isStateVar(&v2);
            let true = (boolEq(b1, b2)) else { return Err("pattern mismatch") };
            replaceableAlias(&v1)?;
            replaceableAlias(&v2)?;
            w1 = BackendVariable::calcAliasKey(&v1)?;
            w2 = BackendVariable::calcAliasKey(&v2)?;
            b = intGt(w2, w1);
            (acr, avar, aindx, _, _, var, e) = if (b) {(cr2.clone(), v2, index2, e2, cr1.clone(), v1, e1)} else {(cr1.clone(), v1, index1, e1, cr2.clone(), v2, e2)};
            var = BackendVariable::mergeAliasVars(var, avar.clone(), false, inGlobalKnownVars.clone())?;
            ops = ElementSource::getSymbolicTransformations(source);
            avar = BackendVariable::mergeVariableOperations(avar, metamodelica::cons(metamodelica::Ref::new(DAE::SymbolicOperation::SOLVED { cr: acr.clone(), exp: e.clone() }), ops))?;
            avar = BackendVariable::setBindExp(avar, Some(e.clone()));
            avar = if (b1) {BackendVariable::setVarKind(avar, openmodelica_backend_types::BackendDAE::VarKind::DUMMY_STATE)?} else {avar};
            (vars, _) = BackendVariable::removeVar(aindx, iVars)?;
            avars = BackendVariable::addVar(avar, iAVars)?;
            vars = BackendVariable::addVar(var, vars)?;
            repl = BackendVarTransform::addReplacement(iRepl, acr.clone(), e.clone(), None)?;
            if Flags::isSet(Flags::DEBUG_ALIAS.clone())? {
                BackendDump::debugStrCrefStrExpStr(&(literal!("Alias Equation ")), &acr, &(literal!(" = ")), e, &(literal!(" found (4).\n")))?;
            }
            (vars, inGlobalKnownVars, iExtVars, avars, repl)
        },
        (Deref @ BackendDAE::Var { varName: cr1, .. }, 1, Deref @ BackendDAE::Var { .. }, 2) => {
            let mut vars: BackendDAE::Variables;
            let mut globalKnownVars: BackendDAE::Variables;
            let mut avars: BackendDAE::Variables;
            let mut repl: BackendVarTransform::VariableReplacements;
            let mut ops: metamodelica::List<metamodelica::Ref<DAE::SymbolicOperation>>;
            let mut var: metamodelica::Ref<BackendDAE::Var>;
            let mut avar: metamodelica::Ref<BackendDAE::Var>;
            replaceableAlias(&v1)?;
            var = BackendVariable::mergeAliasVars(v2, v1.clone(), false, inGlobalKnownVars.clone())?;
            ops = ElementSource::getSymbolicTransformations(source);
            avar = BackendVariable::mergeVariableOperations(v1.clone(), metamodelica::cons(metamodelica::Ref::new(DAE::SymbolicOperation::SOLVED { cr: cr1.clone(), exp: e2.clone() }), ops))?;
            avar = BackendVariable::setBindExp(avar, Some(e2.clone()));
            avar = if (BackendVariable::isStateVar(&v1)) {BackendVariable::setVarKind(avar, openmodelica_backend_types::BackendDAE::VarKind::DUMMY_STATE)?} else {avar};
            (vars, _) = BackendVariable::removeVar(index1, iVars)?;
            avars = BackendVariable::addVar(avar, iAVars)?;
            globalKnownVars = BackendVariable::addVar(var, inGlobalKnownVars)?;
            repl = BackendVarTransform::addReplacement(iRepl, cr1.clone(), e2.clone(), None)?;
            if Flags::isSet(Flags::DEBUG_ALIAS.clone())? {
                BackendDump::debugStrCrefStrExpStr(&(literal!("Alias Equation ")), metamodelica::AsArg::as_arg(&cr1), &(literal!(" = ")), e2, &(literal!(" found (4).\n")))?;
            }
            (vars, globalKnownVars, iExtVars, avars, repl)
        },
        (Deref @ BackendDAE::Var { .. }, 2, Deref @ BackendDAE::Var { varName: cr2, .. }, 1) => {
            let mut vars: BackendDAE::Variables;
            let mut globalKnownVars: BackendDAE::Variables;
            let mut avars: BackendDAE::Variables;
            let mut repl: BackendVarTransform::VariableReplacements;
            let mut ops: metamodelica::List<metamodelica::Ref<DAE::SymbolicOperation>>;
            let mut var: metamodelica::Ref<BackendDAE::Var>;
            let mut avar: metamodelica::Ref<BackendDAE::Var>;
            replaceableAlias(&v2)?;
            var = BackendVariable::mergeAliasVars(v1, v2.clone(), false, inGlobalKnownVars.clone())?;
            ops = ElementSource::getSymbolicTransformations(source);
            avar = BackendVariable::mergeVariableOperations(v2.clone(), metamodelica::cons(metamodelica::Ref::new(DAE::SymbolicOperation::SOLVED { cr: cr2.clone(), exp: e1.clone() }), ops))?;
            avar = BackendVariable::setBindExp(avar, Some(e1.clone()));
            avar = if (BackendVariable::isStateVar(&v2)) {BackendVariable::setVarKind(avar, openmodelica_backend_types::BackendDAE::VarKind::DUMMY_STATE)?} else {avar};
            (vars, _) = BackendVariable::removeVar(index2, iVars)?;
            avars = BackendVariable::addVar(avar, iAVars)?;
            globalKnownVars = BackendVariable::addVar(var, inGlobalKnownVars)?;
            repl = BackendVarTransform::addReplacement(iRepl, cr2.clone(), e1.clone(), None)?;
            if Flags::isSet(Flags::DEBUG_ALIAS.clone())? {
                BackendDump::debugStrCrefStrExpStr(&(literal!("Alias Equation ")), metamodelica::AsArg::as_arg(&cr2), &(literal!(" = ")), e1, &(literal!(" found (4).\n")))?;
            }
            (vars, globalKnownVars, iExtVars, avars, repl)
        },
        (Deref @ BackendDAE::Var { varName: cr1, .. }, 1, Deref @ BackendDAE::Var { .. }, 3) => {
            let mut vars: BackendDAE::Variables;
            let mut extvars: BackendDAE::Variables;
            let mut avars: BackendDAE::Variables;
            let mut repl: BackendVarTransform::VariableReplacements;
            let mut ops: metamodelica::List<metamodelica::Ref<DAE::SymbolicOperation>>;
            let mut var: metamodelica::Ref<BackendDAE::Var>;
            let mut avar: metamodelica::Ref<BackendDAE::Var>;
            replaceableAlias(&v1)?;
            var = BackendVariable::mergeAliasVars(v2, v1.clone(), false, inGlobalKnownVars.clone())?;
            ops = ElementSource::getSymbolicTransformations(source);
            avar = BackendVariable::mergeVariableOperations(v1.clone(), metamodelica::cons(metamodelica::Ref::new(DAE::SymbolicOperation::SOLVED { cr: cr1.clone(), exp: e2.clone() }), ops))?;
            avar = BackendVariable::setBindExp(avar, Some(e2.clone()));
            avar = if (BackendVariable::isStateVar(&v1)) {BackendVariable::setVarKind(avar, openmodelica_backend_types::BackendDAE::VarKind::DUMMY_STATE)?} else {avar};
            (vars, _) = BackendVariable::removeVar(index1, iVars)?;
            avars = BackendVariable::addVar(avar, iAVars)?;
            extvars = BackendVariable::addVar(var, iExtVars)?;
            repl = BackendVarTransform::addReplacement(iRepl, cr1.clone(), e2.clone(), None)?;
            if Flags::isSet(Flags::DEBUG_ALIAS.clone())? {
                BackendDump::debugStrCrefStrExpStr(&(literal!("Alias Equation ")), metamodelica::AsArg::as_arg(&cr1), &(literal!(" = ")), e2, &(literal!(" found (4).\n")))?;
            }
            (vars, inGlobalKnownVars, extvars, avars, repl)
        },
        (Deref @ BackendDAE::Var { .. }, 3, Deref @ BackendDAE::Var { varName: cr2, .. }, 1) => {
            let mut vars: BackendDAE::Variables;
            let mut extvars: BackendDAE::Variables;
            let mut avars: BackendDAE::Variables;
            let mut repl: BackendVarTransform::VariableReplacements;
            let mut ops: metamodelica::List<metamodelica::Ref<DAE::SymbolicOperation>>;
            let mut var: metamodelica::Ref<BackendDAE::Var>;
            let mut avar: metamodelica::Ref<BackendDAE::Var>;
            replaceableAlias(&v2)?;
            var = BackendVariable::mergeAliasVars(v1, v2.clone(), false, inGlobalKnownVars.clone())?;
            ops = ElementSource::getSymbolicTransformations(source);
            avar = BackendVariable::mergeVariableOperations(v2.clone(), metamodelica::cons(metamodelica::Ref::new(DAE::SymbolicOperation::SOLVED { cr: cr2.clone(), exp: e1.clone() }), ops))?;
            avar = BackendVariable::setBindExp(avar, Some(e1.clone()));
            avar = if (BackendVariable::isStateVar(&v2)) {BackendVariable::setVarKind(avar, openmodelica_backend_types::BackendDAE::VarKind::DUMMY_STATE)?} else {avar};
            (vars, _) = BackendVariable::removeVar(index2, iVars)?;
            avars = BackendVariable::addVar(avar, iAVars)?;
            extvars = BackendVariable::addVar(var, iExtVars)?;
            repl = BackendVarTransform::addReplacement(iRepl, cr2.clone(), e1.clone(), None)?;
            if Flags::isSet(Flags::DEBUG_ALIAS.clone())? {
                BackendDump::debugStrCrefStrExpStr(&(literal!("Alias Equation ")), metamodelica::AsArg::as_arg(&cr2), &(literal!(" = ")), e1, &(literal!(" found (4).\n")))?;
            }
            (vars, inGlobalKnownVars, extvars, avars, repl)
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok((oVars, outGlobalKnownVars, oExtVars, oAVars, oRepl))
}

fn replaceableAlias(mut var: &metamodelica::Ref<BackendDAE::Var>) -> Result<()> {
    let () = (match &**var {
        _ => {
            let false = (BackendVariable::isVarOnTopLevelAndOutput(var)) else {
                return Err("pattern mismatch");
            };
            let false = (BackendVariable::isVarOnTopLevelAndInput(var)) else {
                return Err("pattern mismatch");
            };
            let false =
                (BackendVariable::varHasUncertainValueRefine(var) && BackendDAEUtil::isDataReconciliationEnabled()?)
            else {
                return Err("pattern mismatch");
            };
            ()
        }
    });
    Ok(())
}

/*
 *     other helping functions
 */
fn detectImplicitDiscrete(
    mut inVariables: BackendDAE::Variables,
    mut inGlobalKnownVars: BackendDAE::Variables,
    mut inEquationLst: &metamodelica::List<metamodelica::Ref<BackendDAE::Equation>>,
) -> Result<BackendDAE::Variables> {
    let mut outVariables: BackendDAE::Variables;
    outVariables = List::fold1(
        inEquationLst,
        &move |__a0: metamodelica::Ref<BackendDAE::Equation>,
               __a1: BackendDAE::Variables,
               __a2: BackendDAE::Variables|
              -> metamodelica::Result<_> {
            ::std::result::Result::Ok(detectImplicitDiscreteFold(&__a0, __a1, __a2))
        },
        inGlobalKnownVars,
        inVariables,
    )?;
    Ok(outVariables)
}

fn detectImplicitDiscreteFold(
    mut inEquation: &metamodelica::Ref<BackendDAE::Equation>,
    mut inGlobalKnownVars: BackendDAE::Variables,
    mut inVariables: BackendDAE::Variables,
) -> BackendDAE::Variables {
    let mut outVariables: BackendDAE::Variables;
    outVariables = 'mc: {
        let __mc_input = &**inEquation;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ BackendDAE::Equation::WHEN_EQUATION { whenEquation: Deref @ BackendDAE::WhenEquation { whenStmtLst: Deref @ metamodelica::ListNode::Cons { head: BackendDAE::WhenOperator::ASSIGN { left: Deref @ DAE::Exp::CREF { componentRef: cr, .. }, .. }, tail: Deref @ metamodelica::ListNode::Nil }, .. }, .. } => {
                    let mut vars: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
                    (vars, _) = BackendVariable::getVar(cr.clone(), &inVariables)?;
                    vars = List::map1(vars.clone(), &BackendVariable::setVarKind, openmodelica_backend_types::BackendDAE::VarKind::DISCRETE)?;
                    Ok(BackendVariable::addVars(&vars, inVariables.clone())?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ BackendDAE::Equation::WHEN_EQUATION { whenEquation: Deref @ BackendDAE::WhenEquation { whenStmtLst: Deref @ metamodelica::ListNode::Cons { head: BackendDAE::WhenOperator::ASSIGN { left: e, .. }, tail: Deref @ metamodelica::ListNode::Nil }, .. }, .. } => {
                    let mut crefs: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
                    let mut vars: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
                    crefs = Expression::getAllCrefs(e.clone())?;
                    crefs = List::flatten(List::map1(crefs.clone(), &move |__a0: metamodelica::Ref<DAE::ComponentRef>, __a1: bool| ComponentReference::expandCref(&__a0, __a1), true)?)?;
                    (vars, _) = BackendVariable::getVarLst(&crefs, &inVariables);
                    vars = List::map1(vars.clone(), &BackendVariable::setVarKind, openmodelica_backend_types::BackendDAE::VarKind::DISCRETE)?;
                    Ok(BackendVariable::addVars(&vars, inVariables.clone())?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ BackendDAE::Equation::ALGORITHM { alg: Deref @ DAE::Algorithm { statementLst }, .. } => {
                    Ok(detectImplicitDiscreteAlgsStatemens(inVariables.clone(), inGlobalKnownVars.clone(), metamodelica::AsArg::as_arg(&statementLst), false)?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(inVariables.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    outVariables
}

fn getVarsFromExp(
    mut inExpLst: &metamodelica::List<metamodelica::Ref<DAE::Exp>>,
    mut inVariables: BackendDAE::Variables,
) -> Result<metamodelica::List<metamodelica::Ref<BackendDAE::Var>>> {
    let mut outVarLst: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
    outVarLst = 'mc: {
        let __mc_input = (&**inExpLst, inVariables);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Nil, _) => {
                    Ok(metamodelica::nil())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Exp::CREF { componentRef: cref, .. }, tail: expLst }, variables) => {
                    let mut vars: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
                    let mut varLst: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
                    (vars, _) = BackendVariable::getVar(cref.clone(), metamodelica::AsArg::as_arg(&variables))?;
                    varLst = getVarsFromExp(metamodelica::AsArg::as_arg(&expLst), variables.clone())?;
                    Ok(listAppend(vars.clone(), varLst.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: _, tail: expLst }, variables) => {
                    let mut varLst: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
                    varLst = getVarsFromExp(metamodelica::AsArg::as_arg(&expLst), variables.clone())?;
                    Ok(varLst.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(outVarLst)
}

fn detectImplicitDiscreteAlgsStatemens(
    mut inVariables: BackendDAE::Variables,
    mut inGlobalKnownVars: BackendDAE::Variables,
    mut inStatementLst: &metamodelica::List<metamodelica::Ref<DAE::Statement>>,
    mut insideWhen: bool,
) -> Result<BackendDAE::Variables> {
    let mut outVariables: BackendDAE::Variables;
    outVariables = 'mc: {
        let __mc_input = (inVariables, inGlobalKnownVars, &**inStatementLst, insideWhen);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (v, _, Deref @ metamodelica::ListNode::Nil, _) => {
                    Ok(v.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (v, globalKnownVars, Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Statement::STMT_ASSIGN { exp1: Deref @ DAE::Exp::CREF { componentRef: cr, .. }, .. }, tail: xs }, true) => {
                    let mut v_1: BackendDAE::Variables;
                    let mut v_2: BackendDAE::Variables;
                    let mut vars: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
                    (vars, _) = BackendVariable::getVar(cr.clone(), metamodelica::AsArg::as_arg(&v))?;
                    vars = List::map(vars.clone(), &({ let __pe_b1 = openmodelica_backend_types::BackendDAE::VarKind::DISCRETE; move |__pe_a0| BackendVariable::setVarKind(__pe_a0, __pe_b1.clone()) }))?;
                    v_1 = BackendVariable::addVars(&vars, v.clone())?;
                    v_2 = detectImplicitDiscreteAlgsStatemens(v_1.clone(), globalKnownVars.clone(), metamodelica::AsArg::as_arg(&xs), true)?;
                    Ok(v_2.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (v, globalKnownVars, Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Statement::STMT_ASSIGN { exp1: Deref @ DAE::Exp::ASUB { exp: Deref @ DAE::Exp::CREF { componentRef: cr, .. }, sub: subs }, .. }, tail: xs }, true) => {
                    let mut v_1: BackendDAE::Variables;
                    let mut v_2: BackendDAE::Variables;
                    let mut vars: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
                    let mut cr = (*cr).clone();
                    cr = ComponentReference::subscriptCref(metamodelica::AsArg::as_arg(&cr), subs.clone())?;
                    (vars, _) = BackendVariable::getVar(cr.clone(), metamodelica::AsArg::as_arg(&v))?;
                    vars = List::map1(vars.clone(), &BackendVariable::setVarKind, openmodelica_backend_types::BackendDAE::VarKind::DISCRETE)?;
                    v_1 = BackendVariable::addVars(&vars, v.clone())?;
                    v_2 = detectImplicitDiscreteAlgsStatemens(v_1.clone(), globalKnownVars.clone(), metamodelica::AsArg::as_arg(&xs), true)?;
                    Ok(v_2.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (v, globalKnownVars, Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Statement::STMT_TUPLE_ASSIGN { expExpLst, .. }, tail: xs }, true) => {
                    let mut v_1: BackendDAE::Variables;
                    let mut v_2: BackendDAE::Variables;
                    let mut vars: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
                    vars = getVarsFromExp(metamodelica::AsArg::as_arg(&expExpLst), v.clone())?;
                    vars = List::map1(vars.clone(), &BackendVariable::setVarKind, openmodelica_backend_types::BackendDAE::VarKind::DISCRETE)?;
                    v_1 = BackendVariable::addVars(&vars, v.clone())?;
                    v_2 = detectImplicitDiscreteAlgsStatemens(v_1.clone(), globalKnownVars.clone(), metamodelica::AsArg::as_arg(&xs), true)?;
                    Ok(v_2.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (v, globalKnownVars, Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Statement::STMT_ASSIGN_ARR { lhs: Deref @ DAE::Exp::CREF { componentRef: cr, .. }, .. }, tail: xs }, true) => {
                    let mut v_1: BackendDAE::Variables;
                    let mut v_2: BackendDAE::Variables;
                    let mut vars: metamodelica::List<metamodelica::Ref<BackendDAE::Var>>;
                    (vars, _) = BackendVariable::getVar(cr.clone(), metamodelica::AsArg::as_arg(&v))?;
                    vars = List::map1(vars.clone(), &BackendVariable::setVarKind, openmodelica_backend_types::BackendDAE::VarKind::DISCRETE)?;
                    v_1 = BackendVariable::addVars(&vars, v.clone())?;
                    v_2 = detectImplicitDiscreteAlgsStatemens(v_1.clone(), globalKnownVars.clone(), metamodelica::AsArg::as_arg(&xs), true)?;
                    Ok(v_2.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (v, globalKnownVars, Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Statement::STMT_IF { statementLst, .. }, tail: xs }, true) => {
                    let mut v_1: BackendDAE::Variables;
                    let mut v_2: BackendDAE::Variables;
                    v_1 = detectImplicitDiscreteAlgsStatemens(v.clone(), globalKnownVars.clone(), metamodelica::AsArg::as_arg(&statementLst), true)?;
                    v_2 = detectImplicitDiscreteAlgsStatemens(v_1.clone(), globalKnownVars.clone(), metamodelica::AsArg::as_arg(&xs), true)?;
                    Ok(v_2.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (v, globalKnownVars, Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Statement::STMT_FOR { type_: tp, iter: iteratorName, range: e, statementLst, .. }, tail: xs }, true) => {
                    let mut v_1: BackendDAE::Variables;
                    let mut v_2: BackendDAE::Variables;
                    let mut cr: metamodelica::Ref<DAE::ComponentRef>;
                    let mut iteratorExp: metamodelica::Ref<DAE::Exp>;
                    let mut iteratorexps: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                    cr = ComponentReferenceBasics::makeCrefIdent(iteratorName.clone(), tp.clone(), metamodelica::nil());
                    iteratorExp = Expression::crefExp(cr.clone())?;
                    iteratorexps = BackendDAEUtil::extendRange(metamodelica::AsArg::as_arg(&e), metamodelica::AsArg::as_arg(&globalKnownVars))?;
                    v_1 = detectImplicitDiscreteAlgsStatemensFor(iteratorExp.clone(), &iteratorexps, v.clone(), globalKnownVars.clone(), statementLst.clone(), true)?;
                    v_2 = detectImplicitDiscreteAlgsStatemens(v_1.clone(), globalKnownVars.clone(), metamodelica::AsArg::as_arg(&xs), true)?;
                    Ok(v_2.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (v, globalKnownVars, Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Statement::STMT_WHEN { statementLst, elseWhen: None, .. }, tail: xs }, _) => {
                    let mut v_1: BackendDAE::Variables;
                    let mut v_2: BackendDAE::Variables;
                    v_1 = detectImplicitDiscreteAlgsStatemens(v.clone(), globalKnownVars.clone(), metamodelica::AsArg::as_arg(&statementLst), true)?;
                    v_2 = detectImplicitDiscreteAlgsStatemens(v_1.clone(), globalKnownVars.clone(), metamodelica::AsArg::as_arg(&xs), false)?;
                    Ok(v_2.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (v, globalKnownVars, Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Statement::STMT_WHEN { statementLst, elseWhen: Some(statement), .. }, tail: xs }, _) => {
                    let mut v_1: BackendDAE::Variables;
                    let mut v_2: BackendDAE::Variables;
                    let mut v_3: BackendDAE::Variables;
                    v_1 = detectImplicitDiscreteAlgsStatemens(v.clone(), globalKnownVars.clone(), metamodelica::AsArg::as_arg(&statementLst), true)?;
                    v_2 = detectImplicitDiscreteAlgsStatemens(v_1.clone(), globalKnownVars.clone(), &(list![statement.clone()]), true)?;
                    v_3 = detectImplicitDiscreteAlgsStatemens(v_2.clone(), globalKnownVars.clone(), metamodelica::AsArg::as_arg(&xs), false)?;
                    Ok(v_3.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (v, globalKnownVars, Deref @ metamodelica::ListNode::Cons { head: _, tail: xs }, b) => {
                    let mut v_1: BackendDAE::Variables;
                    v_1 = detectImplicitDiscreteAlgsStatemens(v.clone(), globalKnownVars.clone(), metamodelica::AsArg::as_arg(&xs), b.clone())?;
                    Ok(v_1.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(outVariables)
}

fn detectImplicitDiscreteAlgsStatemensFor(
    mut inIteratorExp: metamodelica::Ref<DAE::Exp>,
    mut inExplst: &metamodelica::List<metamodelica::Ref<DAE::Exp>>,
    mut inVariables: BackendDAE::Variables,
    mut inGlobalKnownVars: BackendDAE::Variables,
    mut inStatementLst: metamodelica::List<metamodelica::Ref<DAE::Statement>>,
    mut insideWhen: bool,
) -> Result<BackendDAE::Variables> {
    let mut outVariables: BackendDAE::Variables;
    outVariables = 'mc: {
        let __mc_input = (
            inIteratorExp,
            &**inExplst,
            inVariables,
            inGlobalKnownVars,
            inStatementLst.clone(),
            insideWhen,
        );
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Deref @ metamodelica::ListNode::Nil, v, globalKnownVars, _, _) => {
                    let mut v_1: BackendDAE::Variables;
                    v_1 = detectImplicitDiscreteAlgsStatemens(v.clone(), globalKnownVars.clone(), &inStatementLst, true)?;
                    Ok(v_1.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (ie, Deref @ metamodelica::ListNode::Cons { head: e, tail: Deref @ metamodelica::ListNode::Nil }, v, globalKnownVars, statementLst, _) => {
                    let mut v_1: BackendDAE::Variables;
                    let mut statementLst1: metamodelica::List<metamodelica::Ref<DAE::Statement>>;
                    (statementLst1, _) = DAEUtil::traverseDAEEquationsStmts(statementLst.clone(), (std::sync::Arc::new(Expression::replaceExpTpl) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>, (metamodelica::Ref<DAE::Exp>, metamodelica::Ref<DAE::Exp>)) -> Result<(metamodelica::Ref<DAE::Exp>, (metamodelica::Ref<DAE::Exp>, metamodelica::Ref<DAE::Exp>))> + 'static>), (ie.clone(), e.clone()))?;
                    v_1 = detectImplicitDiscreteAlgsStatemens(v.clone(), globalKnownVars.clone(), &statementLst1, true)?;
                    Ok(v_1.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (ie, Deref @ metamodelica::ListNode::Cons { head: e, tail: rest }, v, globalKnownVars, statementLst, b) => {
                    let mut v_1: BackendDAE::Variables;
                    let mut v_2: BackendDAE::Variables;
                    let mut statementLst1: metamodelica::List<metamodelica::Ref<DAE::Statement>>;
                    (statementLst1, _) = DAEUtil::traverseDAEEquationsStmts(statementLst.clone(), (std::sync::Arc::new(Expression::replaceExpTpl) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>, (metamodelica::Ref<DAE::Exp>, metamodelica::Ref<DAE::Exp>)) -> Result<(metamodelica::Ref<DAE::Exp>, (metamodelica::Ref<DAE::Exp>, metamodelica::Ref<DAE::Exp>))> + 'static>), (ie.clone(), e.clone()))?;
                    v_1 = detectImplicitDiscreteAlgsStatemens(v.clone(), globalKnownVars.clone(), &statementLst1, true)?;
                    v_2 = detectImplicitDiscreteAlgsStatemensFor(ie.clone(), metamodelica::AsArg::as_arg(&rest), v_1.clone(), globalKnownVars.clone(), statementLst.clone(), b.clone())?;
                    Ok(v_2.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (ie, Deref @ metamodelica::ListNode::Cons { head: e, tail: rest }, v, globalKnownVars, statementLst, b) => {
                    let mut v_1: BackendDAE::Variables;
                    let mut v_2: BackendDAE::Variables;
                    let mut statementLst1: metamodelica::List<metamodelica::Ref<DAE::Statement>>;
                    (statementLst1, _) = DAEUtil::traverseDAEEquationsStmts(statementLst.clone(), (std::sync::Arc::new(Expression::replaceExpTpl) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>, (metamodelica::Ref<DAE::Exp>, metamodelica::Ref<DAE::Exp>)) -> Result<(metamodelica::Ref<DAE::Exp>, (metamodelica::Ref<DAE::Exp>, metamodelica::Ref<DAE::Exp>))> + 'static>), (ie.clone(), e.clone()))?;
                    v_1 = detectImplicitDiscreteAlgsStatemens(v.clone(), globalKnownVars.clone(), &statementLst1, true)?;
                    v_2 = detectImplicitDiscreteAlgsStatemensFor(ie.clone(), metamodelica::AsArg::as_arg(&rest), v_1.clone(), globalKnownVars.clone(), statementLst.clone(), b.clone())?;
                    Ok(v_2.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _, _, _, _) => {
                    metamodelica::print(literal!("BackendDAECreate.detectImplicitDiscreteAlgsStatemensFor failed \n"));
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(outVariables)
}

fn lowerFunctions(
    mut funcTree: metamodelica::Ref<AvlTreePathFunction::Tree>,
) -> Result<metamodelica::Ref<AvlTreePathFunction::Tree>> {
    let mut funcTree: metamodelica::Ref<AvlTreePathFunction::Tree> = funcTree;
    funcTree = AvlTreePathFunction::map(
        funcTree,
        &move |__a0: metamodelica::Ref<Absyn::Path>, __a1: Option<DAE::Function>| deriveFunction(&__a0, __a1),
    )?;
    Ok(funcTree)
}

fn deriveFunction(
    mut key: &metamodelica::Ref<Absyn::Path>,
    mut value: Option<DAE::Function>,
) -> Result<Option<DAE::Function>> {
    let mut value: Option<DAE::Function> = value;
    value = (::match_deref::match_deref! { match &(value.clone()) {
        Some(r#fn @ DAE::Function::FUNCTION { functions: Deref @ metamodelica::ListNode::Cons { head: DAE::FunctionDefinition::FUNCTION_PARTIAL_DERIVATIVE { .. }, tail: _ }, .. }) => {
            Error::addSourceMessage(&(Error::UNSUPPORTED_LANGUAGE_FEATURE.clone()), list![literal!("partial derivative of function"), literal!("use --newBackend flag.")], &var_field!(r#fn.source, DAE::Function::FUNCTION).info)?;
            return Err("fail")
        },
        _ => {
            value
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(value)
}

fn renameFunctionParameter(
    mut fTreeIn: metamodelica::Ref<AvlTreePathFunction::Tree>,
) -> metamodelica::Ref<AvlTreePathFunction::Tree> {
    let mut fTreeOut: metamodelica::Ref<AvlTreePathFunction::Tree>;
    fTreeOut = 'mc: {
        let __mc_input = &*fTreeIn;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut funcLst: metamodelica::List<(metamodelica::Ref<Absyn::Path>, Option<DAE::Function>)>;
                    let mut funcs: metamodelica::Ref<AvlTreePathFunction::Tree>;
                    let true = (stringEq(&(Flags::getConfigString(Flags::SIMCODE_TARGET.clone())?), &(literal!("Cpp")))) else { return Err("pattern mismatch") };
                    funcLst = AvlTreePathFunction::toList(&fTreeIn, metamodelica::nil());
                    funcLst = List::map(funcLst.clone(), &renameFunctionParameter1)?;
                    funcs = AvlTreePathFunction::addList(AvlTreePathFunction::new(), &funcLst, &*((std::sync::Arc::new(fnptr!(AvlTreePathFunction::addConflictDefault, _, _, _)) as std::sync::Arc<dyn ::std::ops::Fn(_, _, _) -> Result<_> + 'static>)))?;
                    Ok(funcs.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(fTreeIn.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    fTreeOut
}

fn renameFunctionParameter1(
    mut funcIn: (metamodelica::Ref<Absyn::Path>, Option<DAE::Function>),
) -> Result<(metamodelica::Ref<Absyn::Path>, Option<DAE::Function>)> {
    let mut funcOut: (metamodelica::Ref<Absyn::Path>, Option<DAE::Function>);
    let mut key: metamodelica::Ref<Absyn::Path>;
    let mut value: Option<DAE::Function>;
    let mut pathName: ArcStr;
    let mut r#fn: DAE::Function;
    (key, value) = funcIn.clone();
    funcOut = (match value {
        Some(mut __esc_fn @ DAE::Function::FUNCTION { .. }) => {
            r#fn = __esc_fn.clone();
            pathName = AbsynUtil::pathString(
                var_field!(r#fn.path, DAE::Function::FUNCTION).clone(),
                literal!("."),
                true,
                false,
            )?;
            pathName = {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*Util::stringReplaceChar(pathName, literal!("."), literal!("_"))?);
                __mm_s.push_str(&*literal!("_"));
                ArcStr::from(__mm_s)
            };
            let __owned_variant_functions_0 = ({
                let mut __acc: metamodelica::List<DAE::FunctionDefinition> = metamodelica::nil();
                for mut fn_def in (var_field!(r#fn.functions, DAE::Function::FUNCTION).clone())
                    .into_iter()
                    .cloned()
                {
                    let __x = renameFunctionParameter2(fn_def.clone(), pathName.clone());
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            });
            if let DAE::Function::FUNCTION { functions, .. } = &mut r#fn {
                *functions = __owned_variant_functions_0;
            } else {
                panic!("owned-variant field-assign: value held a different variant than DAE::Function::FUNCTION");
            }
            (key, Some(r#fn))
        }
        _ => funcIn,
    });
    Ok(funcOut)
}

fn renameFunctionParameter2(mut funcIn: DAE::FunctionDefinition, mut pathName: ArcStr) -> DAE::FunctionDefinition {
    let mut funcOut: DAE::FunctionDefinition;
    funcOut = 'mc: {
        let __mc_input = funcIn.clone();
        if let Ok(__v) = (|| -> Result<_> {
            let DAE::FunctionDefinition::FUNCTION_DEF { body: mut body } = __mc_input.clone() else {
                return Err("nomatch");
            };
            let mut params: metamodelica::List<metamodelica::Ref<DAE::Element>>;
            let mut crefs: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
            let mut crefs_new: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
            let mut params_new: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
            let mut repl: VarTransform::VariableReplacements;
            let mut body = body.clone();
            params = List::filterOnTrue(
                body.clone(),
                (std::sync::Arc::new(
                    move |__a0: metamodelica::Ref<DAE::Element>| -> metamodelica::Result<_> {
                        ::std::result::Result::Ok(DAEUtil::isParameter(&__a0))
                    },
                )
                    as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Element>) -> Result<bool> + 'static>),
            )?;
            let false = ((params).is_empty()) else {
                return Err("pattern mismatch");
            };
            crefs = List::map(params.clone(), &move |__a0: metamodelica::Ref<DAE::Element>| {
                DAEUtil::varCref(&__a0)
            })?;
            crefs_new = List::map1r(
                crefs.clone(),
                &move |__a0: ArcStr, __a1: metamodelica::Ref<DAE::ComponentRef>| {
                    ComponentReference::prependStringCref(__a0, &__a1)
                },
                pathName.clone(),
            )?;
            params_new = List::map(crefs_new.clone(), &Expression::crefExp)?;
            repl = VarTransform::emptyReplacements();
            repl = VarTransform::addReplacementLst(repl.clone(), crefs.clone(), params_new.clone())?;
            (body, _) = DAEUtil::traverseDAEElementList(
                body.clone(),
                (std::sync::Arc::new(replaceParameters)
                    as std::sync::Arc<
                        dyn ::std::ops::Fn(
                                metamodelica::Ref<DAE::Exp>,
                                VarTransform::VariableReplacements,
                            )
                                -> Result<(metamodelica::Ref<DAE::Exp>, VarTransform::VariableReplacements)>
                            + 'static,
                    >),
                repl.clone(),
            )?;
            Ok(DAE::FunctionDefinition::FUNCTION_DEF { body: body.clone() })
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            Ok(funcIn.clone())
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    funcOut
}

fn replaceParameters(
    mut inExp: metamodelica::Ref<DAE::Exp>,
    mut replIn: VarTransform::VariableReplacements,
) -> Result<(metamodelica::Ref<DAE::Exp>, VarTransform::VariableReplacements)> {
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    let mut replOut: VarTransform::VariableReplacements;
    replOut = replIn.clone();
    (outExp, _) = VarTransform::replaceExp(inExp, &replIn, None)?;
    Ok((outExp, replOut))
}
