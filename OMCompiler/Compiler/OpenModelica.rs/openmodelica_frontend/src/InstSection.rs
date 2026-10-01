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

use crate::Ceval;
use crate::ConnectUtil;
use crate::ConnectionGraph;
use crate::FGraph;
use crate::InnerOuter;
use crate::Inst;
use crate::InstDAE;
use crate::InstFunction;
use crate::InstUtil;
use crate::Lookup;
use crate::Patternm;
use crate::PrefixUtil;
use crate::Static;
use openmodelica_ast::Absyn;
use openmodelica_error::ErrorExt;
use openmodelica_frontend_base::Algorithm;
use openmodelica_frontend_base::ComponentReference;
use openmodelica_frontend_base::DAEDump;
use openmodelica_frontend_base::DAEUtil;
use openmodelica_frontend_base::Expression;
use openmodelica_frontend_base::ExpressionSimplify;
use openmodelica_frontend_base::Types;
use openmodelica_frontend_base::ValuesUtil;
use openmodelica_frontend_dump::AbsynUtil;
use openmodelica_frontend_dump::ClassInfUtil;
use openmodelica_frontend_dump::ComponentReferenceBasics;
use openmodelica_frontend_dump::Dump;
use openmodelica_frontend_dump::ElementSource;
use openmodelica_frontend_dump::ExpressionBasics;
use openmodelica_frontend_dump::FCore;
use openmodelica_frontend_dump::SCodeDump;
use openmodelica_frontend_dump::SCodeUtil;
use openmodelica_frontend_dump::TypesDump;
use openmodelica_frontend_dump::ValuesDump;
use openmodelica_frontend_inst::ExpressionSimplifyTypes;
use openmodelica_frontend_types::ClassInf;
use openmodelica_frontend_types::DAE;
use openmodelica_frontend_types::DAE::Connect;
use openmodelica_frontend_types::SCode;
use openmodelica_frontend_types::Values;
use openmodelica_util::Config;
use openmodelica_util::Debug;
use openmodelica_util::Error;
use openmodelica_util::Flags;
use openmodelica_util::System;
use openmodelica_util::Util;
use openmodelica_util_datatypes_basic::List;
use openmodelica_util_datatypes_basic::Mutable;

/// an identifier
pub type Ident = ArcStr;

/// an instance hierarchy
pub type InstanceHierarchy = metamodelica::List<InnerOuter::TopInstance>;

pub(crate) const alwaysUnroll: bool = true;

pub(crate) fn instEquation(
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
    mut inIH: metamodelica::List<InnerOuter::TopInstance>,
    mut inPrefix: DAE::Prefix,
    mut inSets: DAE::Connect::Sets,
    mut inState: ClassInf::State,
    mut inEquation: metamodelica::Ref<SCode::Equation>,
    mut inImpl: bool,
    mut unrollForLoops: bool,
    mut inGraph: ConnectionGraph::ConnectionGraph,
) -> Result<(
    FCore::Cache,
    FCore::Graph,
    metamodelica::List<InnerOuter::TopInstance>,
    DAE::DAElist,
    DAE::Connect::Sets,
    ClassInf::State,
    ConnectionGraph::ConnectionGraph,
)> {
    let mut outCache: FCore::Cache;
    let mut outEnv: FCore::Graph;
    let mut outIH: metamodelica::List<InnerOuter::TopInstance>;
    let mut outDae: DAE::DAElist;
    let mut outSets: DAE::Connect::Sets;
    let mut outState: ClassInf::State;
    let mut outGraph: ConnectionGraph::ConnectionGraph;
    (outCache, outEnv, outIH, outDae, outSets, outState, outGraph) = instEquationCommon(
        inCache,
        inEnv,
        inIH,
        inPrefix,
        inSets,
        inState,
        inEquation,
        openmodelica_frontend_types::SCode::Initial::NON_INITIAL,
        inImpl,
        inGraph,
    )?;
    Ok((outCache, outEnv, outIH, outDae, outSets, outState, outGraph))
}

pub(crate) fn instInitialEquation(
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
    mut inIH: metamodelica::List<InnerOuter::TopInstance>,
    mut inPrefix: DAE::Prefix,
    mut inSets: DAE::Connect::Sets,
    mut inState: ClassInf::State,
    mut inEquation: metamodelica::Ref<SCode::Equation>,
    mut inImpl: bool,
    mut unrollForLoops: bool,
    mut inGraph: ConnectionGraph::ConnectionGraph,
) -> Result<(
    FCore::Cache,
    FCore::Graph,
    metamodelica::List<InnerOuter::TopInstance>,
    DAE::DAElist,
    DAE::Connect::Sets,
    ClassInf::State,
    ConnectionGraph::ConnectionGraph,
)> {
    let mut outCache: FCore::Cache;
    let mut outEnv: FCore::Graph;
    let mut outIH: metamodelica::List<InnerOuter::TopInstance>;
    let mut outDae: DAE::DAElist;
    let mut outSets: DAE::Connect::Sets;
    let mut outState: ClassInf::State;
    let mut outGraph: ConnectionGraph::ConnectionGraph;
    (outCache, outEnv, outIH, outDae, outSets, outState, outGraph) = instEquationCommon(
        inCache,
        inEnv,
        inIH,
        inPrefix,
        inSets,
        inState,
        inEquation,
        openmodelica_frontend_types::SCode::Initial::INITIAL,
        inImpl,
        inGraph,
    )?;
    Ok((outCache, outEnv, outIH, outDae, outSets, outState, outGraph))
}

fn instEquationCommon(
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
    mut inIH: metamodelica::List<InnerOuter::TopInstance>,
    mut inPrefix: DAE::Prefix,
    mut inSets: DAE::Connect::Sets,
    mut inState: ClassInf::State,
    mut inEquation: metamodelica::Ref<SCode::Equation>,
    mut inInitial: SCode::Initial,
    mut inImpl: bool,
    mut inGraph: ConnectionGraph::ConnectionGraph,
) -> Result<(
    FCore::Cache,
    FCore::Graph,
    metamodelica::List<InnerOuter::TopInstance>,
    DAE::DAElist,
    DAE::Connect::Sets,
    ClassInf::State,
    ConnectionGraph::ConnectionGraph,
)> {
    let mut outCache: FCore::Cache;
    let mut outEnv: FCore::Graph;
    let mut outIH: metamodelica::List<InnerOuter::TopInstance>;
    let mut outDae: DAE::DAElist;
    let mut outSets: DAE::Connect::Sets;
    let mut outState: ClassInf::State;
    let mut outGraph: ConnectionGraph::ConnectionGraph;
    let mut errorCount: i32 = Error::getNumErrorMessages();
    let mut s: ArcStr;
    let mut state: ClassInf::State;
    match '__try0: {
        state = unwrap_break_err!(ClassInfUtil::trans(inState.clone(), openmodelica_frontend_types::ClassInf::Event::FOUND_EQUATION), '__try0);
        (outCache, outEnv, outIH, outDae, outSets, outState, outGraph) = unwrap_break_err!(instEquationCommonWork(inCache.clone(), inEnv.clone(), inIH.clone(), inPrefix.clone(), inSets.clone(), state.clone(), inEquation.clone(), inInitial, inImpl, inGraph.clone(), metamodelica::Ref::new(DAE::SymbolicOperation::FLATTEN { scode: inEquation.clone(), dae: None })), '__try0);
        (outDae, _, _) = unwrap_break_err!(DAEUtil::traverseDAE(outDae.clone(), openmodelica_frontend_dump::AvlTreePathFunction::Tree::interned_EMPTY(), (std::sync::Arc::new(Expression::traverseSubexpressionsHelper) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>, _) -> Result<_> + 'static>), ((std::sync::Arc::new(ExpressionSimplify::simplifyWork) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>, ExpressionSimplifyTypes::Evaluate) -> Result<(metamodelica::Ref<DAE::Exp>, ExpressionSimplifyTypes::Evaluate)> + 'static>), ExpressionSimplifyTypes::optionSimplifyOnly.clone())), '__try0);
        Ok::<_, &'static str>((
            outCache.clone(),
            outDae.clone(),
            outEnv.clone(),
            outGraph.clone(),
            outIH.clone(),
            outSets.clone(),
            outState.clone(),
            state.clone(),
        ))
    } {
        Ok((__try0_o0, __try0_o1, __try0_o2, __try0_o3, __try0_o4, __try0_o5, __try0_o6, __try0_o7)) => {
            outCache = __try0_o0;
            outDae = __try0_o1;
            outEnv = __try0_o2;
            outGraph = __try0_o3;
            outIH = __try0_o4;
            outSets = __try0_o5;
            outState = __try0_o6;
            state = __try0_o7;
        }
        Err(__try0_err) => {
            if '__try1: {
                if '__try2: {
                    unwrap_break_err!(ClassInfUtil::trans(inState.clone(), openmodelica_frontend_types::ClassInf::Event::FOUND_EQUATION), '__try2);
                    Ok::<(), &'static str>(())
                }.is_ok() { return Err("failure(): body succeeded") }
                s = ClassInfUtil::printStateStr(&inState);
                unwrap_break_err!(Error::addSourceMessage(&(Error::EQUATION_TRANSITION_FAILURE.clone()), list![s.clone()], &(SCodeUtil::getEquationInfo(&inEquation))), '__try1);
                Ok::<(), &'static str>(())
            }.is_err() {
                if errorCount == Error::getNumErrorMessages() {
                    s = { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*SCodeDump::equationStr(inEquation.clone(), SCodeDump::defaultOptions.clone())?); ArcStr::from(__mm_s) };
                    Error::addSourceMessage(&(Error::EQUATION_GENERIC_FAILURE.clone()), list![s.clone()], &(SCodeUtil::getEquationInfo(&inEquation)))?;
                }
            }
            return Err(__try0_err);
        }
    }
    Ok((outCache, outEnv, outIH, outDae, outSets, outState, outGraph))
}

fn instEquationCommonWork(
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
    mut inIH: metamodelica::List<InnerOuter::TopInstance>,
    mut inPrefix: DAE::Prefix,
    mut inSets: DAE::Connect::Sets,
    mut inState: ClassInf::State,
    mut inEquation: metamodelica::Ref<SCode::Equation>,
    mut inInitial: SCode::Initial,
    mut inImpl: bool,
    mut inGraph: ConnectionGraph::ConnectionGraph,
    mut inFlattenOp: metamodelica::Ref<DAE::SymbolicOperation>,
) -> Result<(
    FCore::Cache,
    FCore::Graph,
    metamodelica::List<InnerOuter::TopInstance>,
    DAE::DAElist,
    DAE::Connect::Sets,
    ClassInf::State,
    ConnectionGraph::ConnectionGraph,
)> {
    let mut outCache: FCore::Cache = inCache.clone();
    let mut outEnv: FCore::Graph = inEnv.clone();
    let mut outIH: metamodelica::List<InnerOuter::TopInstance> = inIH.clone();
    let mut outDae: DAE::DAElist = <DAE::DAElist as ::std::default::Default>::default();
    let mut outSets: DAE::Connect::Sets = inSets.clone();
    let mut outState: ClassInf::State = <ClassInf::State as ::std::default::Default>::default();
    let mut outGraph: ConnectionGraph::ConnectionGraph = inGraph.clone();
    (outDae, outState) = 'mc: {
        let __mc_input = &*inEquation;
        if let Ok((__v, __wb0, __wb1, __wb2, __wb3, __wb4, __wb5, __wb6)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ SCode::Equation::EQ_CONNECT { crefLeft: lhs_acr, crefRight: rhs_acr, info, .. } => {
                    let mut outCache: FCore::Cache = outCache.clone();
                    let mut outDae: DAE::DAElist = outDae.clone();
                    let mut outEnv: FCore::Graph = outEnv.clone();
                    let mut outGraph: ConnectionGraph::ConnectionGraph = outGraph.clone();
                    let mut outIH: metamodelica::List<InnerOuter::TopInstance> = outIH.clone();
                    let mut outSets: DAE::Connect::Sets = outSets.clone();
                    let mut outState: ClassInf::State = outState.clone();
                    if SCodeUtil::isInitial(inInitial) {
                        Error::addSourceMessage(&(Error::CONNECT_IN_INITIAL_EQUATION.clone()), metamodelica::nil(), metamodelica::AsArg::as_arg(&info))?;
                        return Err("fail");
                    }
                    (outCache, outEnv, outIH, outSets, outDae, outGraph) = instConnect(outCache.clone(), outEnv.clone(), outIH.clone(), outSets.clone(), inPrefix.clone(), lhs_acr.clone(), rhs_acr.clone(), inImpl, inGraph.clone(), info.clone())?;
                    outState = instEquationCommonCiTrans(inState.clone(), inInitial)?;
                    Ok(((outDae.clone(), outState.clone()), outCache.clone(), outDae.clone(), outEnv.clone(), outGraph.clone(), outIH.clone(), outSets.clone(), outState.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            outCache = __wb0;
            outDae = __wb1;
            outEnv = __wb2;
            outGraph = __wb3;
            outIH = __wb4;
            outSets = __wb5;
            outState = __wb6;
            break 'mc __v;
        }
        if let Ok((__v, __wb0, __wb1, __wb2)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ SCode::Equation::EQ_EQUALS { expLeft: lhs_aexp, expRight: rhs_aexp, info, comment } => {
                    let mut lhs_exp: metamodelica::Ref<DAE::Exp>;
                    let mut rhs_exp: metamodelica::Ref<DAE::Exp>;
                    let mut lhs_prop: DAE::Properties;
                    let mut rhs_prop: DAE::Properties;
                    let mut source: metamodelica::Ref<DAE::ElementSource>;
                    let mut outCache: FCore::Cache = outCache.clone();
                    let mut outDae: DAE::DAElist = outDae.clone();
                    let mut outState: ClassInf::State = outState.clone();
                    checkTupleCallEquationMessage(lhs_aexp.clone(), rhs_aexp.clone(), metamodelica::AsArg::as_arg(&info))?;
                    (outCache, lhs_exp, lhs_prop) = Static::elabExpLHS(inCache.clone(), inEnv.clone(), lhs_aexp.clone(), inImpl, true, inPrefix.clone(), info.clone())?;
                    (outCache, rhs_exp, rhs_prop) = Static::elabExp(inCache.clone(), inEnv.clone(), rhs_aexp.clone(), inImpl, true, inPrefix.clone(), info.clone())?;
                    (outCache, lhs_exp, lhs_prop) = Ceval::cevalIfConstant(outCache.clone(), inEnv.clone(), lhs_exp.clone(), lhs_prop.clone(), inImpl, info.clone())?;
                    (outCache, rhs_exp, rhs_prop) = Ceval::cevalIfConstant(outCache.clone(), inEnv.clone(), rhs_exp.clone(), rhs_prop.clone(), inImpl, info.clone())?;
                    (outCache, lhs_exp, rhs_exp, lhs_prop) = condenseArrayEquation(outCache.clone(), inEnv.clone(), lhs_aexp.clone(), rhs_aexp.clone(), lhs_exp.clone(), rhs_exp.clone(), lhs_prop.clone(), rhs_prop.clone(), inImpl, inPrefix.clone(), info.clone());
                    (outCache, lhs_exp) = PrefixUtil::prefixExp(outCache.clone(), &inEnv, &inIH, lhs_exp.clone(), &inPrefix)?;
                    (outCache, rhs_exp) = PrefixUtil::prefixExp(outCache.clone(), &inEnv, &inIH, rhs_exp.clone(), &inPrefix)?;
                    source = makeEqSource(info.clone(), &inEnv, &inPrefix, inFlattenOp.clone())?;
                    source = ElementSource::addCommentToSource(source.clone(), Some(comment.clone()));
                    outDae = instEqEquation(lhs_exp.clone(), lhs_prop.clone(), rhs_exp.clone(), rhs_prop.clone(), source.clone(), inInitial, inImpl, Absyn::dummyInfo.clone())?;
                    outState = instEquationCommonCiTrans(inState.clone(), inInitial)?;
                    Ok(((outDae.clone(), outState.clone()), outCache.clone(), outDae.clone(), outState.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            outCache = __wb0;
            outDae = __wb1;
            outState = __wb2;
            break 'mc __v;
        }
        if let Ok((__v, __wb0, __wb1, __wb2, __wb3, __wb4, __wb5, __wb6)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ SCode::Equation::EQ_IF { thenBranch: branches, elseBranch: else_branch, info, .. } => {
                    let mut prop: DAE::Properties;
                    let mut source: metamodelica::Ref<DAE::ElementSource>;
                    let mut expl: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                    let mut props: metamodelica::List<DAE::Properties>;
                    let mut c: DAE::Const;
                    let mut val: metamodelica::Ref<Values::Value>;
                    let mut eql: metamodelica::List<metamodelica::Ref<SCode::Equation>>;
                    let mut rest_branches: metamodelica::List<metamodelica::List<metamodelica::Ref<SCode::Equation>>>;
                    let mut ell: metamodelica::List<metamodelica::List<metamodelica::Ref<DAE::Element>>>;
                    let mut el: metamodelica::List<metamodelica::Ref<DAE::Element>>;
                    let mut val: metamodelica::Ref<Values::Value>;
                    let mut outCache: FCore::Cache = outCache.clone();
                    let mut outDae: DAE::DAElist = outDae.clone();
                    let mut outEnv: FCore::Graph = outEnv.clone();
                    let mut outGraph: ConnectionGraph::ConnectionGraph = outGraph.clone();
                    let mut outIH: metamodelica::List<InnerOuter::TopInstance> = outIH.clone();
                    let mut outSets: DAE::Connect::Sets = outSets.clone();
                    let mut outState: ClassInf::State = outState.clone();
                    (outCache, expl, props) = Static::elabExpList(outCache.clone(), outEnv.clone(), var_field!((*inEquation).condition, SCode::Equation::EQ_IF), inImpl, true, inPrefix.clone(), info.clone(), DAE::T_UNKNOWN_DEFAULT().clone())?;
                    prop = Types::propsAnd(&props)?;
                    checkIfConditionTypes(&prop, var_field!((*inEquation).condition, SCode::Equation::EQ_IF), props.clone(), info.clone())?;
                    match '__try0: {
                        rest_branches = branches.clone();
                        eql = else_branch.clone();
                        for mut cond in &*expl {
                            let (__pa1, __pa2) = ::match_deref::match_deref! { match &(props.clone()) {
                                        Deref @ metamodelica::ListNode::Cons { head: DAE::Properties::PROP { constFlag: __pa1, .. }, tail: __pa2 } => (__pa1.clone(), __pa2.clone()),
                                        _ => break '__try0 Err::<_, _>("pattern mismatch"),
                            } };
                            c = metamodelica::Own::own(__pa1);
                            props = metamodelica::Own::own(__pa2);
                            let true = (Types::isParameterOrConstant(c)) else { break '__try0 Err::<_, _>("pattern mismatch") };
                            (outCache, val) = unwrap_break_err!(Ceval::ceval(outCache.clone(), outEnv.clone(), cond.clone(), inImpl, openmodelica_ast::Absyn::Msg::NO_MSG, 0), '__try0);
                            let true = (unwrap_break_err!(checkIfConditionBinding(val.clone(), metamodelica::AsArg::as_arg(&info)), '__try0)) else { break '__try0 Err::<_, _>("pattern mismatch") };
                            if unwrap_break_err!(ValuesUtil::valueBool(&val), '__try0) {
                                        eql = unwrap_break_err!((rest_branches).head().cloned(), '__try0);
                                        break;
                            }
                            rest_branches = unwrap_break_err!((rest_branches).rest(), '__try0);
                        }
                        outCache = unwrap_break_err!(InstUtil::popStructuralParameters(outCache.clone(), inPrefix.clone()), '__try0);
                        (outCache, outEnv, outIH, outDae, outSets, outState, outGraph) = unwrap_break_err!(Inst::instList(outCache.clone(), inEnv.clone(), inIH.clone(), inPrefix.clone(), inSets.clone(), inState.clone(), &*(if (SCodeUtil::isInitial(inInitial)) { (std::sync::Arc::new(instInitialEquation) as std::sync::Arc<dyn ::std::ops::Fn(FCore::Cache, FCore::Graph, metamodelica::List<InnerOuter::TopInstance>, DAE::Prefix, DAE::Connect::Sets, ClassInf::State, metamodelica::Ref<SCode::Equation>, bool, bool, ConnectionGraph::ConnectionGraph) -> Result<(FCore::Cache, FCore::Graph, metamodelica::List<InnerOuter::TopInstance>, DAE::DAElist, DAE::Connect::Sets, ClassInf::State, ConnectionGraph::ConnectionGraph)> + 'static>) } else { (std::sync::Arc::new(instEquation) as std::sync::Arc<dyn ::std::ops::Fn(FCore::Cache, FCore::Graph, metamodelica::List<InnerOuter::TopInstance>, DAE::Prefix, DAE::Connect::Sets, ClassInf::State, metamodelica::Ref<SCode::Equation>, bool, bool, ConnectionGraph::ConnectionGraph) -> Result<(FCore::Cache, FCore::Graph, metamodelica::List<InnerOuter::TopInstance>, DAE::DAElist, DAE::Connect::Sets, ClassInf::State, ConnectionGraph::ConnectionGraph)> + 'static>) }), &eql, inImpl, alwaysUnroll.clone(), inGraph.clone()), '__try0);
                        Ok::<_, &'static str>((outCache.clone(), outDae.clone(), outEnv.clone(), outIH.clone(), outState.clone()))
                    } {
                        Ok((__try0_o0, __try0_o1, __try0_o2, __try0_o3, __try0_o4)) => {
                            outCache = __try0_o0;
                            outDae = __try0_o1;
                            outEnv = __try0_o2;
                            outIH = __try0_o3;
                            outState = __try0_o4;
                        }
                        Err(_) => {
                            (outCache, expl) = PrefixUtil::prefixExpList(outCache.clone(), &inEnv, &inIH, &expl, &inPrefix)?;
                            source = makeEqSource(info.clone(), &inEnv, &inPrefix, inFlattenOp.clone())?;
                            if SCodeUtil::isInitial(inInitial) {
                                        (outCache, outEnv, outIH, outState, ell) = instInitialIfEqBranches(outCache.clone(), inEnv.clone(), inIH.clone(), &inPrefix, inState.clone(), branches.clone(), inImpl, metamodelica::nil())?;
                                        (outCache, outEnv, outIH, outState, el) = instInitialIfEqBranch(outCache.clone(), outEnv.clone(), outIH.clone(), inPrefix.clone(), outState.clone(), metamodelica::AsArg::as_arg(&else_branch), inImpl)?;
                                        outDae = DAE::DAElist { elementLst: list![metamodelica::Ref::new(DAE::Element::INITIAL_IF_EQUATION { condition1: expl.clone(), equations2: ell.clone(), equations3: el.clone(), source: source.clone() })] };
                            } else {
                                        (outCache, outEnv, outIH, outState, ell) = instIfEqBranches(outCache.clone(), inEnv.clone(), inIH.clone(), &inPrefix, inState.clone(), branches.clone(), inImpl, metamodelica::nil())?;
                                        (outCache, outEnv, outIH, outState, el) = instIfEqBranch(outCache.clone(), outEnv.clone(), outIH.clone(), inPrefix.clone(), outState.clone(), metamodelica::AsArg::as_arg(&else_branch), inImpl)?;
                                        outDae = DAE::DAElist { elementLst: list![metamodelica::Ref::new(DAE::Element::IF_EQUATION { condition1: expl.clone(), equations2: ell.clone(), equations3: el.clone(), source: source.clone() })] };
                            }
                        }
                    }
                    Ok(((outDae.clone(), outState.clone()), outCache.clone(), outDae.clone(), outEnv.clone(), outGraph.clone(), outIH.clone(), outSets.clone(), outState.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            outCache = __wb0;
            outDae = __wb1;
            outEnv = __wb2;
            outGraph = __wb3;
            outIH = __wb4;
            outSets = __wb5;
            outState = __wb6;
            break 'mc __v;
        }
        if let Ok((__v, __wb0, __wb1, __wb2, __wb3, __wb4, __wb5)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ SCode::Equation::EQ_WHEN { info, .. } => {
                    let mut exp: metamodelica::Ref<DAE::Exp>;
                    let mut cond_exp: metamodelica::Ref<DAE::Exp>;
                    let mut source: metamodelica::Ref<DAE::ElementSource>;
                    let mut el: metamodelica::List<metamodelica::Ref<DAE::Element>>;
                    let mut el2: metamodelica::List<metamodelica::Ref<DAE::Element>>;
                    let mut else_when: Option<metamodelica::Ref<DAE::Element>>;
                    let mut outCache: FCore::Cache = outCache.clone();
                    let mut outDae: DAE::DAElist = outDae.clone();
                    let mut outEnv: FCore::Graph = outEnv.clone();
                    let mut outGraph: ConnectionGraph::ConnectionGraph = outGraph.clone();
                    let mut outIH: metamodelica::List<InnerOuter::TopInstance> = outIH.clone();
                    let mut outState: ClassInf::State = outState.clone();
                    if SCodeUtil::isInitial(inInitial) {
                        Error::addSourceMessageAndFail(&(Error::INITIAL_WHEN.clone()), metamodelica::nil(), metamodelica::AsArg::as_arg(&info))?;
                        unreachable!("Error.addSourceMessageAndFail always fails — caller-side flow-analysis hint");
                    }
                    (outCache, outEnv, outIH, cond_exp, el, outGraph) = instWhenEqBranch(inCache.clone(), inEnv.clone(), inIH.clone(), inPrefix.clone(), inSets.clone(), inState.clone(), &((var_field!((*inEquation).condition, SCode::Equation::EQ_WHEN).clone(), var_field!((*inEquation).eEquationLst, SCode::Equation::EQ_WHEN).clone())), inImpl, alwaysUnroll.clone(), inGraph.clone(), info.clone())?;
                    source = makeEqSource(info.clone(), &inEnv, &inPrefix, inFlattenOp.clone())?;
                    else_when = None;
                    for mut branch in &*var_field!((*inEquation).elseBranches, SCode::Equation::EQ_WHEN).clone().reverse() {
                        (outCache, outEnv, outIH, exp, el2, outGraph) = instWhenEqBranch(outCache.clone(), outEnv.clone(), outIH.clone(), inPrefix.clone(), inSets.clone(), inState.clone(), &(branch.clone()), inImpl, alwaysUnroll.clone(), outGraph.clone(), info.clone())?;
                        else_when = Some(metamodelica::Ref::new(DAE::Element::WHEN_EQUATION { condition: exp.clone(), equations: el2.clone(), elsewhen_: else_when.clone(), source: source.clone() }));
                    }
                    outState = instEquationCommonCiTrans(inState.clone(), inInitial)?;
                    outDae = DAE::DAElist { elementLst: list![metamodelica::Ref::new(DAE::Element::WHEN_EQUATION { condition: cond_exp.clone(), equations: el.clone(), elsewhen_: else_when.clone(), source: source.clone() })] };
                    Ok(((outDae.clone(), outState.clone()), outCache.clone(), outDae.clone(), outEnv.clone(), outGraph.clone(), outIH.clone(), outState.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            outCache = __wb0;
            outDae = __wb1;
            outEnv = __wb2;
            outGraph = __wb3;
            outIH = __wb4;
            outState = __wb5;
            break 'mc __v;
        }
        if let Ok((__v, __wb0, __wb1, __wb2, __wb3, __wb4)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ SCode::Equation::EQ_FOR { info, .. } => {
                    let mut range_aexp: metamodelica::Ref<Absyn::Exp>;
                    let mut exp: metamodelica::Ref<DAE::Exp>;
                    let mut c: DAE::Const;
                    let mut val: metamodelica::Ref<Values::Value>;
                    let mut iter_crefs: metamodelica::List<(metamodelica::Ref<Absyn::ComponentRef>, i32)>;
                    let mut ty: metamodelica::Ref<DAE::Type>;
                    let mut env: FCore::Graph;
                    let mut val: metamodelica::Ref<Values::Value>;
                    let mut outCache: FCore::Cache = outCache.clone();
                    let mut outDae: DAE::DAElist = outDae.clone();
                    let mut outGraph: ConnectionGraph::ConnectionGraph = outGraph.clone();
                    let mut outSets: DAE::Connect::Sets = outSets.clone();
                    let mut outState: ClassInf::State = outState.clone();
                    if (var_field!((*inEquation).range, SCode::Equation::EQ_FOR)).is_some() {
                        let __pa0 = ::match_deref::match_deref! { match &(var_field!((*inEquation).range, SCode::Equation::EQ_FOR).clone()) {
                            Some(__pa0) => __pa0.clone(),
                            _ => return Err("pattern mismatch"),
                        } };
                        range_aexp = metamodelica::Own::own(__pa0);
                        let (__pa1, __pa2, __pa3, __pa4) = ::match_deref::match_deref! { match &(Static::elabExp(outCache.clone(), inEnv.clone(), range_aexp.clone(), inImpl, true, inPrefix.clone(), info.clone())?) {
                            (__pa1, __pa2, DAE::Properties::PROP { type_: Deref @ DAE::Type::T_ARRAY { ty: __pa3, .. }, constFlag: __pa4 }) => (__pa1.clone(), __pa2.clone(), __pa3.clone(), __pa4.clone()),
                            _ => return Err("pattern mismatch"),
                        } };
                        outCache = metamodelica::Own::own(__pa1);
                        exp = metamodelica::Own::own(__pa2);
                        ty = metamodelica::Own::own(__pa3);
                        c = metamodelica::Own::own(__pa4);
                    } else {
                        iter_crefs = SCodeUtil::findIteratorIndexedCrefsInEquations(&(var_field!((*inEquation).eEquationLst, SCode::Equation::EQ_FOR).clone()), var_field!((*inEquation).index, SCode::Equation::EQ_FOR).clone(), metamodelica::nil())?;
                        let (__pa6, __pa7, __pa8, __pa9) = ::match_deref::match_deref! { match &(Static::deduceIterationRange(var_field!((*inEquation).index, SCode::Equation::EQ_FOR).clone(), &iter_crefs, inEnv.clone(), outCache.clone(), metamodelica::AsArg::as_arg(&info))?) {
                            (__pa6, DAE::Properties::PROP { type_: Deref @ DAE::Type::T_ARRAY { ty: __pa7, .. }, constFlag: __pa8 }, __pa9) => (__pa6.clone(), __pa7.clone(), __pa8.clone(), __pa9.clone()),
                            _ => return Err("pattern mismatch"),
                        } };
                        exp = metamodelica::Own::own(__pa6);
                        ty = metamodelica::Own::own(__pa7);
                        c = metamodelica::Own::own(__pa8);
                        outCache = metamodelica::Own::own(__pa9);
                        range_aexp = metamodelica::Ref::new(Absyn::Exp::STRING { value: literal!("Internal error: generated implicit range could not be evaluated.") });
                    }
                    env = addForLoopScope(inEnv.clone(), var_field!((*inEquation).index, SCode::Equation::EQ_FOR).clone(), ty.clone(), openmodelica_frontend_types::SCode::Variability::VAR, Some(c))?;
                    match '__try11: {
                        (outCache, val) = unwrap_break_err!(Ceval::ceval(outCache.clone(), inEnv.clone(), exp.clone(), inImpl, openmodelica_ast::Absyn::Msg::NO_MSG, 0), '__try11);
                        Ok::<_, &'static str>((val.clone(),))
                    } {
                        Ok((__try11_o0,)) => {
                            val = __try11_o0;
                        }
                        Err(_) => {
                            if Flags::getConfigBool(Flags::CHECK_MODEL.clone())? {
                                        val = metamodelica::Ref::new(Values::Value::ARRAY { valueLst: list![metamodelica::Ref::new(Values::Value::INTEGER { integer: 1 })], dimLst: list![1] });
                            } else {
                                        Error::addSourceMessageAndFail(&(Error::NON_PARAMETER_ITERATOR_RANGE.clone()), list![Dump::printExpStr(range_aexp.clone())?], metamodelica::AsArg::as_arg(&info))?;
                                        unreachable!("Error.addSourceMessageAndFail always fails — caller-side flow-analysis hint");
                            }
                        }
                    }
                    (outCache, outDae, outSets, outGraph) = unroll(outCache.clone(), env.clone(), inIH.clone(), inPrefix.clone(), inSets.clone(), inState.clone(), var_field!((*inEquation).index, SCode::Equation::EQ_FOR).clone(), ty.clone(), &val, &(var_field!((*inEquation).eEquationLst, SCode::Equation::EQ_FOR).clone()), inInitial, inImpl, inGraph.clone())?;
                    outState = instEquationCommonCiTrans(inState.clone(), inInitial)?;
                    Ok(((outDae.clone(), outState.clone()), outCache.clone(), outDae.clone(), outGraph.clone(), outSets.clone(), outState.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            outCache = __wb0;
            outDae = __wb1;
            outGraph = __wb2;
            outSets = __wb3;
            outState = __wb4;
            break 'mc __v;
        }
        if let Ok((__v, __wb0, __wb1)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ SCode::Equation::EQ_ASSERT { info, .. } => {
                    let mut cond_exp: metamodelica::Ref<DAE::Exp>;
                    let mut msg_exp: metamodelica::Ref<DAE::Exp>;
                    let mut level_exp: metamodelica::Ref<DAE::Exp>;
                    let mut source: metamodelica::Ref<DAE::ElementSource>;
                    let mut outCache: FCore::Cache = outCache.clone();
                    let mut outDae: DAE::DAElist = outDae.clone();
                    (outCache, cond_exp) = instOperatorArg(outCache.clone(), inEnv.clone(), &inIH, inPrefix.clone(), var_field!((*inEquation).condition, SCode::Equation::EQ_ASSERT).clone(), inImpl, DAE::T_BOOL_DEFAULT().clone(), literal!("assert"), literal!("condition"), 1, info.clone())?;
                    (outCache, msg_exp) = instOperatorArg(outCache.clone(), inEnv.clone(), &inIH, inPrefix.clone(), var_field!((*inEquation).message, SCode::Equation::EQ_ASSERT).clone(), inImpl, DAE::T_STRING_DEFAULT().clone(), literal!("assert"), literal!("message"), 2, info.clone())?;
                    (outCache, level_exp) = instOperatorArg(outCache.clone(), inEnv.clone(), &inIH, inPrefix.clone(), var_field!((*inEquation).level, SCode::Equation::EQ_ASSERT).clone(), inImpl, DAE::T_ASSERTIONLEVEL().clone(), literal!("assert"), literal!("level"), 3, info.clone())?;
                    source = makeEqSource(info.clone(), &inEnv, &inPrefix, inFlattenOp.clone())?;
                    if SCodeUtil::isInitial(inInitial) {
                        outDae = DAE::DAElist { elementLst: list![metamodelica::Ref::new(DAE::Element::INITIAL_ASSERT { condition: cond_exp.clone(), message: msg_exp.clone(), level: level_exp.clone(), source: source.clone() })] };
                    } else {
                        outDae = DAE::DAElist { elementLst: list![metamodelica::Ref::new(DAE::Element::ASSERT { condition: cond_exp.clone(), message: msg_exp.clone(), level: level_exp.clone(), source: source.clone() })] };
                    }
                    Ok(((outDae.clone(), inState.clone()), outCache.clone(), outDae.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            outCache = __wb0;
            outDae = __wb1;
            break 'mc __v;
        }
        if let Ok((__v, __wb0, __wb1)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ SCode::Equation::EQ_TERMINATE { info, .. } => {
                    let mut msg_exp: metamodelica::Ref<DAE::Exp>;
                    let mut source: metamodelica::Ref<DAE::ElementSource>;
                    let mut outCache: FCore::Cache = outCache.clone();
                    let mut outDae: DAE::DAElist = outDae.clone();
                    (outCache, msg_exp) = instOperatorArg(outCache.clone(), inEnv.clone(), &inIH, inPrefix.clone(), var_field!((*inEquation).message, SCode::Equation::EQ_TERMINATE).clone(), inImpl, DAE::T_STRING_DEFAULT().clone(), literal!("terminate"), literal!("message"), 1, info.clone())?;
                    source = makeEqSource(info.clone(), &inEnv, &inPrefix, inFlattenOp.clone())?;
                    if SCodeUtil::isInitial(inInitial) {
                        outDae = DAE::DAElist { elementLst: list![metamodelica::Ref::new(DAE::Element::INITIAL_TERMINATE { message: msg_exp.clone(), source: source.clone() })] };
                    } else {
                        outDae = DAE::DAElist { elementLst: list![metamodelica::Ref::new(DAE::Element::TERMINATE { message: msg_exp.clone(), source: source.clone() })] };
                    }
                    Ok(((outDae.clone(), inState.clone()), outCache.clone(), outDae.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            outCache = __wb0;
            outDae = __wb1;
            break 'mc __v;
        }
        if let Ok((__v, __wb0, __wb1)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                        Deref @ SCode::Equation::EQ_REINIT { cref: Deref @ Absyn::Exp::CREF { componentRef: acr }, info, .. } => {
                            let mut exp: metamodelica::Ref<DAE::Exp>;
                            let mut cr_exp: metamodelica::Ref<DAE::Exp>;
                            let mut prop: DAE::Properties;
                            let mut cr_prop: DAE::Properties;
                            let mut source: metamodelica::Ref<DAE::ElementSource>;
                            let mut el: metamodelica::List<metamodelica::Ref<DAE::Element>>;
                            let mut ty: metamodelica::Ref<DAE::Type>;
                            let mut cr: metamodelica::Ref<DAE::ComponentRef>;
                            let mut outCache: FCore::Cache = outCache.clone();
                            let mut outDae: DAE::DAElist = outDae.clone();
                            let (__pa0, __pa3, __pa1, __pa2, __pa4) = ::match_deref::match_deref! { match &(Static::elabCrefNoEval(outCache.clone(), inEnv.clone(), acr.clone(), inImpl, false, inPrefix.clone(), info.clone())?) {
                                (__pa0, __pa3 @ Deref @ DAE::Exp::CREF { componentRef: __pa1, ty: __pa2 }, __pa4, _) => (__pa0.clone(), __pa3.clone(), __pa1.clone(), __pa2.clone(), __pa4.clone()),
                                _ => return Err("pattern mismatch"),
                            } };
                            outCache = metamodelica::Own::own(__pa0);
                            cr = metamodelica::Own::own(__pa1);
                            ty = metamodelica::Own::own(__pa2);
                            cr_exp = metamodelica::Own::own(__pa3);
                            cr_prop = metamodelica::Own::own(__pa4);
                            let true = (checkReinitType(&ty, &cr_prop, &cr, metamodelica::AsArg::as_arg(&info))) else { return Err("pattern mismatch") };
                            (outCache, exp, prop) = Static::elabExp(outCache.clone(), inEnv.clone(), var_field!((*inEquation).expReinit, SCode::Equation::EQ_REINIT).clone(), inImpl, true, inPrefix.clone(), info.clone())?;
                            (outCache, exp, prop) = Ceval::cevalIfConstant(outCache.clone(), inEnv.clone(), exp.clone(), prop.clone(), inImpl, info.clone())?;
                            (exp, _) = Types::matchProp(exp.clone(), &prop, &cr_prop, true)?;
                            (outCache, cr_exp, exp, cr_prop) = condenseArrayEquation(outCache.clone(), inEnv.clone(), var_field!((*inEquation).cref, SCode::Equation::EQ_REINIT).clone(), var_field!((*inEquation).expReinit, SCode::Equation::EQ_REINIT).clone(), cr_exp.clone(), exp.clone(), cr_prop.clone(), prop.clone(), inImpl, inPrefix.clone(), info.clone());
                            (outCache, cr_exp) = PrefixUtil::prefixExp(outCache.clone(), &inEnv, &inIH, cr_exp.clone(), &inPrefix)?;
                            (outCache, exp) = PrefixUtil::prefixExp(outCache.clone(), &inEnv, &inIH, exp.clone(), &inPrefix)?;
                            source = makeEqSource(info.clone(), &inEnv, &inPrefix, inFlattenOp.clone())?;
                            let DAE::DAE { elementLst: __pa6 } = instEqEquation(cr_exp.clone(), cr_prop.clone(), exp.clone(), prop.clone(), source.clone(), inInitial, inImpl, Absyn::dummyInfo.clone())?;
                            el = metamodelica::Own::own(__pa6);
                            el = ({
                let mut __acc: metamodelica::List<metamodelica::Ref<DAE::Element>> = metamodelica::nil();
                for mut e in (el.clone()).into_iter().cloned() {
                            let __x = makeDAEArrayEqToReinitForm(&(e.clone()))?;
                            __acc = cons(__x, __acc);
                }
                __acc.reverse()
            });
                            outDae = DAE::DAElist { elementLst: el.clone() };
                            Ok(((outDae.clone(), inState.clone()), outCache.clone(), outDae.clone()))
                        }
                        _ => return Err("nomatch"),
                    }}
        })() {
            outCache = __wb0;
            outDae = __wb1;
            break 'mc __v;
        }
        if let Ok((__v, __wb0, __wb1, __wb2, __wb3, __wb4, __wb5, __wb6)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ SCode::Equation::EQ_NORETCALL { info, .. } => {
                    let mut exp: metamodelica::Ref<DAE::Exp>;
                    let mut source: metamodelica::Ref<DAE::ElementSource>;
                    let mut outCache: FCore::Cache = outCache.clone();
                    let mut outDae: DAE::DAElist = outDae.clone();
                    let mut outEnv: FCore::Graph = outEnv.clone();
                    let mut outGraph: ConnectionGraph::ConnectionGraph = outGraph.clone();
                    let mut outIH: metamodelica::List<InnerOuter::TopInstance> = outIH.clone();
                    let mut outSets: DAE::Connect::Sets = outSets.clone();
                    let mut outState: ClassInf::State = outState.clone();
                    if isConnectionsOperator(var_field!((*inEquation).exp, SCode::Equation::EQ_NORETCALL)) {
                        (outCache, outEnv, outIH, outDae, outSets, outState, outGraph) = handleConnectionsOperators(inCache.clone(), inEnv.clone(), inIH.clone(), inPrefix.clone(), inSets.clone(), inState.clone(), inEquation.clone(), inInitial, inImpl, inGraph.clone(), &inFlattenOp)?;
                    } else {
                        (outCache, exp, _) = Static::elabExp(inCache.clone(), inEnv.clone(), var_field!((*inEquation).exp, SCode::Equation::EQ_NORETCALL).clone(), inImpl, false, inPrefix.clone(), info.clone())?;
                        (outCache, exp) = PrefixUtil::prefixExp(outCache.clone(), &inEnv, &inIH, exp.clone(), &inPrefix)?;
                        source = makeEqSource(info.clone(), &inEnv, &inPrefix, inFlattenOp.clone())?;
                        outDae = instEquationNoRetCallVectorization(exp.clone(), inInitial, source.clone());
                        outState = inState.clone();
                    }
                    Ok(((outDae.clone(), outState.clone()), outCache.clone(), outDae.clone(), outEnv.clone(), outGraph.clone(), outIH.clone(), outSets.clone(), outState.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            outCache = __wb0;
            outDae = __wb1;
            outEnv = __wb2;
            outGraph = __wb3;
            outIH = __wb4;
            outSets = __wb5;
            outState = __wb6;
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { return Err("pattern mismatch") };
                    Debug::trace(literal!("- InstSection.instEquationCommonWork failed for eqn: "))?;
                    Debug::traceln({ let mut __mm_s = String::new(); __mm_s.push_str(&*SCodeDump::equationStr(inEquation.clone(), SCodeDump::defaultOptions.clone())?); __mm_s.push_str(&*literal!(" in scope: ")); __mm_s.push_str(&*FGraph::getGraphNameStr(&inEnv)); ArcStr::from(__mm_s) })?;
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((outCache, outEnv, outIH, outDae, outSets, outState, outGraph))
}

fn makeEqSource(
    mut inInfo: SourceInfo,
    mut inEnv: &FCore::Graph,
    mut inPrefix: &DAE::Prefix,
    mut inFlattenOp: metamodelica::Ref<DAE::SymbolicOperation>,
) -> Result<metamodelica::Ref<DAE::ElementSource>> {
    let mut outSource: metamodelica::Ref<DAE::ElementSource>;
    outSource = ElementSource::createElementSource(
        inInfo,
        FGraph::getScopePath(inEnv)?,
        inPrefix,
        (DAE::emptyCref().clone(), DAE::emptyCref().clone()),
    );
    outSource = ElementSource::addSymbolicTransformation(outSource, inFlattenOp)?;
    Ok(outSource)
}

fn checkIfConditionTypes(
    mut inAccumProp: &DAE::Properties,
    mut inConditions: &metamodelica::List<metamodelica::Ref<Absyn::Exp>>,
    mut inProperties: metamodelica::List<DAE::Properties>,
    mut inInfo: SourceInfo,
) -> Result<()> {
    let () = (::match_deref::match_deref! { match &(inAccumProp) {
        DAE::Properties::PROP { type_: Deref @ DAE::Type::T_BOOL { .. }, .. } => {
            ()
        },
        _ => {
            let mut props: metamodelica::List<DAE::Properties>;
            let mut ty: metamodelica::Ref<DAE::Type>;
            let mut exp_str: ArcStr;
            let mut ty_str: ArcStr;
            props = inProperties;
            for mut cond in &**inConditions {
                let (__pa0, __pa1) = ::match_deref::match_deref! { match &(props) {
                    Deref @ metamodelica::ListNode::Cons { head: DAE::Properties::PROP { type_: __pa0, .. }, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
                    _ => return Err("pattern mismatch"),
                } };
                ty = metamodelica::Own::own(__pa0);
                props = metamodelica::Own::own(__pa1);
                if !(Types::isScalarBoolean(&ty)) {
                    exp_str = Dump::printExpStr(cond.clone())?;
                    ty_str = TypesDump::unparseTypeNoAttr(&ty)?;
                    Error::addSourceMessageAndFail(&(Error::IF_CONDITION_TYPE_ERROR.clone()), list![exp_str, ty_str], &inInfo)?;
                    unreachable!("Error.addSourceMessageAndFail always fails — caller-side flow-analysis hint");
                }
            }
            Error::addInternalError(literal!("InstSection.checkIfConditionTypes failed to find non-Boolean condition."), inInfo)?;
            return Err("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(())
}

fn checkIfConditionBinding(mut inValues: metamodelica::Ref<Values::Value>, mut inInfo: &SourceInfo) -> Result<bool> {
    let mut outHasBindings: bool;
    let mut empty_val: Option<metamodelica::Ref<Values::Value>>;
    let mut name: ArcStr;
    empty_val = ValuesUtil::containsEmpty(inValues);
    if (empty_val).is_some() {
        let __pa0 = ::match_deref::match_deref! { match &(empty_val) {
            Some(Deref @ Values::Value::EMPTY { name: __pa0, .. }) => __pa0.clone(),
            _ => return Err("pattern mismatch"),
        } };
        name = metamodelica::Own::own(__pa0);
        Error::addSourceMessage(&(Error::CONDITIONAL_EXP_WITHOUT_VALUE.clone()), list![name], inInfo)?;
        outHasBindings = false;
    } else {
        outHasBindings = true;
    }
    Ok(outHasBindings)
}

fn instOperatorArg(
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
    mut inIH: &metamodelica::List<InnerOuter::TopInstance>,
    mut inPrefix: DAE::Prefix,
    mut inArg: metamodelica::Ref<Absyn::Exp>,
    mut inImpl: bool,
    mut inExpectedType: metamodelica::Ref<DAE::Type>,
    mut inOperatorName: ArcStr,
    mut inArgName: ArcStr,
    mut inArgIndex: i32,
    mut inInfo: SourceInfo,
) -> Result<(FCore::Cache, metamodelica::Ref<DAE::Exp>)> {
    let mut outCache: FCore::Cache;
    let mut outArg: metamodelica::Ref<DAE::Exp>;
    let mut props: DAE::Properties;
    let mut ty: metamodelica::Ref<DAE::Type>;
    (outCache, outArg, props) = Static::elabExp(
        inCache,
        inEnv.clone(),
        inArg.clone(),
        inImpl,
        true,
        inPrefix.clone(),
        inInfo.clone(),
    )?;
    ty = Types::getPropType(&props);
    if !(Types::subtype(ty.clone(), inExpectedType.clone(), true)) {
        Error::addSourceMessageAndFail(
            &(Error::ARG_TYPE_MISMATCH.clone()),
            list![
                intString(inArgIndex),
                inOperatorName,
                inArgName,
                Dump::printExpStr(inArg)?,
                TypesDump::unparseTypeNoAttr(&ty)?,
                TypesDump::unparseType(inExpectedType)?
            ],
            &inInfo,
        )?;
        unreachable!("Error.addSourceMessageAndFail always fails — caller-side flow-analysis hint");
    }
    (outCache, outArg, _) = Ceval::cevalIfConstant(outCache, inEnv.clone(), outArg, props, inImpl, inInfo)?;
    (outCache, outArg) = PrefixUtil::prefixExp(outCache, &inEnv, inIH, outArg, &inPrefix)?;
    Ok((outCache, outArg))
}

fn isConnectionsOperator(mut inExp: &metamodelica::Ref<Absyn::Exp>) -> bool {
    let mut yes: bool;
    yes = (::match_deref::match_deref! { match inExp {
        Deref @ Absyn::Exp::CALL { function_: Deref @ Absyn::ComponentRef::CREF_QUAL { name: Deref @ "Connections", subscripts: Deref @ metamodelica::ListNode::Nil, componentRef: Deref @ Absyn::ComponentRef::CREF_IDENT { name: id, subscripts: Deref @ metamodelica::ListNode::Nil } }, .. } => {
            listMember(id.clone(), list![literal!("root"), literal!("potentialRoot"), literal!("branch"), literal!("uniqueRoot")])
        },
        _ => {
            false
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    yes
}

fn handleConnectionsOperators(
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
    mut inIH: metamodelica::List<InnerOuter::TopInstance>,
    mut inPrefix: DAE::Prefix,
    mut inSets: DAE::Connect::Sets,
    mut inState: ClassInf::State,
    mut inEquation: metamodelica::Ref<SCode::Equation>,
    mut inInitial: SCode::Initial,
    mut inImpl: bool,
    mut inGraph: ConnectionGraph::ConnectionGraph,
    mut flattenOp: &metamodelica::Ref<DAE::SymbolicOperation>,
) -> Result<(
    FCore::Cache,
    FCore::Graph,
    metamodelica::List<InnerOuter::TopInstance>,
    DAE::DAElist,
    DAE::Connect::Sets,
    ClassInf::State,
    ConnectionGraph::ConnectionGraph,
)> {
    let mut outCache: FCore::Cache;
    let mut outEnv: FCore::Graph;
    let mut outIH: metamodelica::List<InnerOuter::TopInstance>;
    let mut outDae: DAE::DAElist;
    let mut outSets: DAE::Connect::Sets;
    let mut outState: ClassInf::State;
    let mut outGraph: ConnectionGraph::ConnectionGraph;
    (outCache, outEnv, outIH, outDae, outSets, outState, outGraph) = 'mc: {
        let __mc_input = (
            inCache,
            inEnv,
            inIH,
            inPrefix,
            inSets,
            inState,
            inEquation.clone(),
            inGraph,
        );
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, ih, pre, csets, ci_state, Deref @ SCode::Equation::EQ_NORETCALL { info, exp: Deref @ Absyn::Exp::CALL { function_: Deref @ Absyn::ComponentRef::CREF_QUAL { name: Deref @ "Connections", subscripts: Deref @ metamodelica::ListNode::Nil, componentRef: Deref @ Absyn::ComponentRef::CREF_IDENT { name: Deref @ "root", subscripts: Deref @ metamodelica::ListNode::Nil } }, functionArgs: Deref @ Absyn::FunctionArgs::FUNCTIONARGS { args: Deref @ metamodelica::ListNode::Cons { head: Deref @ Absyn::Exp::CREF { componentRef: cr }, tail: Deref @ metamodelica::ListNode::Nil }, argNames: Deref @ metamodelica::ListNode::Nil }, .. }, .. }, graph) => {
                    let mut s: ArcStr;
                    let mut cache = (*cache).clone();
                    let __pa0 = ::match_deref::match_deref! { match &(Static::elabCref(cache.clone(), env.clone(), cr.clone(), false, false, pre.clone(), info.clone())?) {
                        (__pa0, Some((Deref @ DAE::Exp::ARRAY { array: Deref @ metamodelica::ListNode::Nil, .. }, _, _))) => __pa0.clone(),
                        _ => return Err("pattern mismatch"),
                    } };
                    cache = metamodelica::Own::own(__pa0);
                    s = SCodeDump::equationStr(inEquation.clone(), SCodeDump::defaultOptions.clone())?;
                    Error::addSourceMessage(&(Error::OVERCONSTRAINED_OPERATOR_SIZE_ZERO.clone()), list![s.clone()], metamodelica::AsArg::as_arg(&info))?;
                    Ok((cache.clone(), env.clone(), ih.clone(), DAE::emptyDae().clone(), csets.clone(), ci_state.clone(), graph.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, ih, pre, csets, ci_state, Deref @ SCode::Equation::EQ_NORETCALL { info, exp: Deref @ Absyn::Exp::CALL { function_: Deref @ Absyn::ComponentRef::CREF_QUAL { name: Deref @ "Connections", subscripts: Deref @ metamodelica::ListNode::Nil, componentRef: Deref @ Absyn::ComponentRef::CREF_IDENT { name: Deref @ "root", subscripts: Deref @ metamodelica::ListNode::Nil } }, functionArgs: Deref @ Absyn::FunctionArgs::FUNCTIONARGS { args: Deref @ metamodelica::ListNode::Cons { head: Deref @ Absyn::Exp::CREF { componentRef: cr }, tail: Deref @ metamodelica::ListNode::Nil }, argNames: Deref @ metamodelica::ListNode::Nil }, .. }, .. }, graph) => {
                    let mut cr_: metamodelica::Ref<DAE::ComponentRef>;
                    let mut cache = (*cache).clone();
                    let mut graph = (*graph).clone();
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(Static::elabCref(cache.clone(), env.clone(), cr.clone(), false, false, pre.clone(), info.clone())?) {
                        (__pa0, Some((Deref @ DAE::Exp::CREF { componentRef: __pa1, ty: _ }, _, _))) => (__pa0.clone(), __pa1.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    cache = metamodelica::Own::own(__pa0);
                    cr_ = metamodelica::Own::own(__pa1);
                    (cache, cr_) = PrefixUtil::prefixCref(cache.clone(), env.clone(), metamodelica::AsArg::as_arg(&ih), pre.clone(), cr_.clone())?;
                    graph = ConnectionGraph::addDefiniteRoot(metamodelica::AsArg::as_arg(&graph), cr_.clone())?;
                    Ok((cache.clone(), env.clone(), ih.clone(), DAE::emptyDae().clone(), csets.clone(), ci_state.clone(), graph.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, ih, pre, csets, ci_state, Deref @ SCode::Equation::EQ_NORETCALL { info, exp: Deref @ Absyn::Exp::CALL { function_: Deref @ Absyn::ComponentRef::CREF_QUAL { name: Deref @ "Connections", subscripts: Deref @ metamodelica::ListNode::Nil, componentRef: Deref @ Absyn::ComponentRef::CREF_IDENT { name: Deref @ "potentialRoot", subscripts: Deref @ metamodelica::ListNode::Nil } }, functionArgs, .. }, .. }, graph) => {
                    let mut cr: metamodelica::Ref<Absyn::ComponentRef>;
                    let mut s: ArcStr;
                    let mut cache = (*cache).clone();
                    (cr, _) = potentialRootArguments(metamodelica::AsArg::as_arg(&functionArgs), metamodelica::AsArg::as_arg(&info), pre.clone(), inEquation.clone())?;
                    let __pa0 = ::match_deref::match_deref! { match &(Static::elabCref(cache.clone(), env.clone(), cr.clone(), false, false, pre.clone(), info.clone())?) {
                        (__pa0, Some((Deref @ DAE::Exp::ARRAY { array: Deref @ metamodelica::ListNode::Nil, .. }, _, _))) => __pa0.clone(),
                        _ => return Err("pattern mismatch"),
                    } };
                    cache = metamodelica::Own::own(__pa0);
                    s = SCodeDump::equationStr(inEquation.clone(), SCodeDump::defaultOptions.clone())?;
                    Error::addSourceMessage(&(Error::OVERCONSTRAINED_OPERATOR_SIZE_ZERO.clone()), list![s.clone()], metamodelica::AsArg::as_arg(&info))?;
                    Ok((cache.clone(), env.clone(), ih.clone(), DAE::emptyDae().clone(), csets.clone(), ci_state.clone(), graph.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, ih, pre, csets, ci_state, Deref @ SCode::Equation::EQ_NORETCALL { info, exp: Deref @ Absyn::Exp::CALL { function_: Deref @ Absyn::ComponentRef::CREF_QUAL { name: Deref @ "Connections", subscripts: Deref @ metamodelica::ListNode::Nil, componentRef: Deref @ Absyn::ComponentRef::CREF_IDENT { name: Deref @ "potentialRoot", subscripts: Deref @ metamodelica::ListNode::Nil } }, functionArgs, .. }, .. }, graph) => {
                    let mut cr: metamodelica::Ref<Absyn::ComponentRef>;
                    let mut ipriority: i32;
                    let mut cr_: metamodelica::Ref<DAE::ComponentRef>;
                    let mut cache = (*cache).clone();
                    let mut graph = (*graph).clone();
                    (cr, ipriority) = potentialRootArguments(metamodelica::AsArg::as_arg(&functionArgs), metamodelica::AsArg::as_arg(&info), pre.clone(), inEquation.clone())?;
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(Static::elabCref(cache.clone(), env.clone(), cr.clone(), false, false, pre.clone(), info.clone())?) {
                        (__pa0, Some((Deref @ DAE::Exp::CREF { componentRef: __pa1, ty: _ }, _, _))) => (__pa0.clone(), __pa1.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    cache = metamodelica::Own::own(__pa0);
                    cr_ = metamodelica::Own::own(__pa1);
                    (cache, cr_) = PrefixUtil::prefixCref(cache.clone(), env.clone(), metamodelica::AsArg::as_arg(&ih), pre.clone(), cr_.clone())?;
                    graph = ConnectionGraph::addPotentialRoot(metamodelica::AsArg::as_arg(&graph), cr_.clone(), intReal(ipriority))?;
                    Ok((cache.clone(), env.clone(), ih.clone(), DAE::emptyDae().clone(), csets.clone(), ci_state.clone(), graph.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, ih, pre, csets, ci_state, Deref @ SCode::Equation::EQ_NORETCALL { info, exp: Deref @ Absyn::Exp::CALL { function_: Deref @ Absyn::ComponentRef::CREF_QUAL { name: Deref @ "Connections", subscripts: Deref @ metamodelica::ListNode::Nil, componentRef: Deref @ Absyn::ComponentRef::CREF_IDENT { name: Deref @ "uniqueRoot", subscripts: Deref @ metamodelica::ListNode::Nil } }, functionArgs, .. }, .. }, graph) => {
                    let mut cr: metamodelica::Ref<Absyn::ComponentRef>;
                    let mut s: ArcStr;
                    let mut cache = (*cache).clone();
                    (cr, _) = uniqueRootArguments(metamodelica::AsArg::as_arg(&functionArgs), metamodelica::AsArg::as_arg(&info), pre.clone(), inEquation.clone())?;
                    let __pa0 = ::match_deref::match_deref! { match &(Static::elabCref(cache.clone(), env.clone(), cr.clone(), false, false, pre.clone(), info.clone())?) {
                        (__pa0, Some((Deref @ DAE::Exp::ARRAY { array: Deref @ metamodelica::ListNode::Nil, .. }, _, _))) => __pa0.clone(),
                        _ => return Err("pattern mismatch"),
                    } };
                    cache = metamodelica::Own::own(__pa0);
                    s = SCodeDump::equationStr(inEquation.clone(), SCodeDump::defaultOptions.clone())?;
                    Error::addSourceMessage(&(Error::OVERCONSTRAINED_OPERATOR_SIZE_ZERO.clone()), list![s.clone()], metamodelica::AsArg::as_arg(&info))?;
                    Error::addSourceMessage(&(Error::NON_STANDARD_OPERATOR.clone()), list![literal!("Connections.uniqueRoot")], metamodelica::AsArg::as_arg(&info))?;
                    Ok((cache.clone(), env.clone(), ih.clone(), DAE::emptyDae().clone(), csets.clone(), ci_state.clone(), graph.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, ih, pre, csets, ci_state, Deref @ SCode::Equation::EQ_NORETCALL { info, exp: Deref @ Absyn::Exp::CALL { function_: Deref @ Absyn::ComponentRef::CREF_QUAL { name: Deref @ "Connections", subscripts: Deref @ metamodelica::ListNode::Nil, componentRef: Deref @ Absyn::ComponentRef::CREF_IDENT { name: Deref @ "uniqueRoot", subscripts: Deref @ metamodelica::ListNode::Nil } }, functionArgs, .. }, .. }, graph) => {
                    let mut cr: metamodelica::Ref<Absyn::ComponentRef>;
                    let mut msg: metamodelica::Ref<Absyn::Exp>;
                    let mut msg_1: metamodelica::Ref<DAE::Exp>;
                    let mut exp: metamodelica::Ref<DAE::Exp>;
                    let mut cache = (*cache).clone();
                    let mut graph = (*graph).clone();
                    (cr, msg) = uniqueRootArguments(metamodelica::AsArg::as_arg(&functionArgs), metamodelica::AsArg::as_arg(&info), pre.clone(), inEquation.clone())?;
                    (cache, exp, _) = Static::elabExp(cache.clone(), env.clone(), metamodelica::Ref::new(Absyn::Exp::CREF { componentRef: cr.clone() }), false, true, pre.clone(), info.clone())?;
                    (cache, msg_1, _) = Static::elabExp(cache.clone(), env.clone(), msg.clone(), false, false, pre.clone(), info.clone())?;
                    (cache, exp) = PrefixUtil::prefixExp(cache.clone(), metamodelica::AsArg::as_arg(&env), metamodelica::AsArg::as_arg(&ih), exp.clone(), metamodelica::AsArg::as_arg(&pre))?;
                    (cache, msg_1) = PrefixUtil::prefixExp(cache.clone(), metamodelica::AsArg::as_arg(&env), metamodelica::AsArg::as_arg(&ih), msg_1.clone(), metamodelica::AsArg::as_arg(&pre))?;
                    graph = ConnectionGraph::addUniqueRoots(graph.clone(), exp.clone(), &msg_1)?;
                    Error::addSourceMessage(&(Error::NON_STANDARD_OPERATOR.clone()), list![literal!("Connections.uniqueRoot")], metamodelica::AsArg::as_arg(&info))?;
                    Ok((cache.clone(), env.clone(), ih.clone(), DAE::emptyDae().clone(), csets.clone(), ci_state.clone(), graph.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, ih, pre, csets, ci_state, Deref @ SCode::Equation::EQ_NORETCALL { info, exp: Deref @ Absyn::Exp::CALL { function_: Deref @ Absyn::ComponentRef::CREF_QUAL { name: Deref @ "Connections", subscripts: Deref @ metamodelica::ListNode::Nil, componentRef: Deref @ Absyn::ComponentRef::CREF_IDENT { name: Deref @ "branch", subscripts: Deref @ metamodelica::ListNode::Nil } }, functionArgs: Deref @ Absyn::FunctionArgs::FUNCTIONARGS { args: Deref @ metamodelica::ListNode::Cons { head: Deref @ Absyn::Exp::CREF { componentRef: cr1 }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Absyn::Exp::CREF { componentRef: cr2 }, tail: Deref @ metamodelica::ListNode::Nil } }, argNames: Deref @ metamodelica::ListNode::Nil }, .. }, .. }, graph) => {
                    let mut b1: bool;
                    let mut b2: bool;
                    let mut s: ArcStr;
                    let mut e_1: metamodelica::Ref<DAE::Exp>;
                    let mut e_2: metamodelica::Ref<DAE::Exp>;
                    let mut cr1_: metamodelica::Ref<DAE::ComponentRef>;
                    let mut cr2_: metamodelica::Ref<DAE::ComponentRef>;
                    let mut cache = (*cache).clone();
                    let mut graph = (*graph).clone();
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(Static::elabCref(cache.clone(), env.clone(), cr1.clone(), false, false, pre.clone(), info.clone())?) {
                        (__pa0, Some((__pa1, _, _))) => (__pa0.clone(), __pa1.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    cache = metamodelica::Own::own(__pa0);
                    e_1 = metamodelica::Own::own(__pa1);
                    let (__pa2, __pa3) = ::match_deref::match_deref! { match &(Static::elabCref(cache.clone(), env.clone(), cr2.clone(), false, false, pre.clone(), info.clone())?) {
                        (__pa2, Some((__pa3, _, _))) => (__pa2.clone(), __pa3.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    cache = metamodelica::Own::own(__pa2);
                    e_2 = metamodelica::Own::own(__pa3);
                    b1 = Types::isZeroLengthArray(&(Expression::r#typeof(e_1.clone())?))?;
                    b2 = Types::isZeroLengthArray(&(Expression::r#typeof(e_2.clone())?))?;
                    if boolOr(b1, b2) {
                        s = SCodeDump::equationStr(inEquation.clone(), SCodeDump::defaultOptions.clone())?;
                        Error::addSourceMessage(&(Error::OVERCONSTRAINED_OPERATOR_SIZE_ZERO.clone()), list![s.clone()], metamodelica::AsArg::as_arg(&info))?;
                    } else {
                        let __pa4 = ::match_deref::match_deref! { match &(e_1.clone()) {
                            Deref @ DAE::Exp::CREF { componentRef: __pa4, ty: _ } => __pa4.clone(),
                            _ => return Err("pattern mismatch"),
                        } };
                        cr1_ = metamodelica::Own::own(__pa4);
                        let __pa5 = ::match_deref::match_deref! { match &(e_2.clone()) {
                            Deref @ DAE::Exp::CREF { componentRef: __pa5, ty: _ } => __pa5.clone(),
                            _ => return Err("pattern mismatch"),
                        } };
                        cr2_ = metamodelica::Own::own(__pa5);
                        (cache, cr1_) = PrefixUtil::prefixCref(cache.clone(), env.clone(), metamodelica::AsArg::as_arg(&ih), pre.clone(), cr1_.clone())?;
                        (cache, cr2_) = PrefixUtil::prefixCref(cache.clone(), env.clone(), metamodelica::AsArg::as_arg(&ih), pre.clone(), cr2_.clone())?;
                        graph = ConnectionGraph::addBranch(metamodelica::AsArg::as_arg(&graph), cr1_.clone(), cr2_.clone())?;
                    }
                    Ok((cache.clone(), env.clone(), ih.clone(), DAE::emptyDae().clone(), csets.clone(), ci_state.clone(), graph.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, env, _, _, _, _, eqn, _) => {
                    let mut s: ArcStr;
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { return Err("pattern mismatch") };
                    s = SCodeDump::equationStr(eqn.clone(), SCodeDump::defaultOptions.clone())?;
                    Debug::trace(literal!("- handleConnectionsOperators failed for eqn: "))?;
                    Debug::traceln({ let mut __mm_s = String::new(); __mm_s.push_str(&*s); __mm_s.push_str(&*literal!(" in scope:")); __mm_s.push_str(&*FGraph::getGraphNameStr(metamodelica::AsArg::as_arg(&env))); ArcStr::from(__mm_s) })?;
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((outCache, outEnv, outIH, outDae, outSets, outState, outGraph))
}

fn potentialRootArguments(
    mut inFunctionArgs: &metamodelica::Ref<Absyn::FunctionArgs>,
    mut info: &SourceInfo,
    mut inPrefix: DAE::Prefix,
    mut inEquation: metamodelica::Ref<SCode::Equation>,
) -> Result<(metamodelica::Ref<Absyn::ComponentRef>, i32)> {
    let mut outCref: metamodelica::Ref<Absyn::ComponentRef>;
    let mut outPriority: i32;
    (outCref, outPriority) = (::match_deref::match_deref! { match inFunctionArgs {
        Deref @ Absyn::FunctionArgs::FUNCTIONARGS { args: Deref @ metamodelica::ListNode::Cons { head: Deref @ Absyn::Exp::CREF { componentRef: cr }, tail: Deref @ metamodelica::ListNode::Nil }, argNames: Deref @ metamodelica::ListNode::Nil } => {
            (cr.clone(), 0)
        },
        Deref @ Absyn::FunctionArgs::FUNCTIONARGS { args: Deref @ metamodelica::ListNode::Cons { head: Deref @ Absyn::Exp::CREF { componentRef: cr }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Absyn::Exp::INTEGER { value: p }, tail: Deref @ metamodelica::ListNode::Nil } }, argNames: Deref @ metamodelica::ListNode::Nil } => {
            (cr.clone(), p.clone())
        },
        Deref @ Absyn::FunctionArgs::FUNCTIONARGS { args: Deref @ metamodelica::ListNode::Cons { head: Deref @ Absyn::Exp::CREF { componentRef: cr }, tail: Deref @ metamodelica::ListNode::Nil }, argNames: Deref @ metamodelica::ListNode::Cons { head: Deref @ Absyn::NamedArg { argName: Deref @ "priority", argValue: Deref @ Absyn::Exp::INTEGER { value: p } }, tail: Deref @ metamodelica::ListNode::Nil } } => {
            (cr.clone(), p.clone())
        },
        _ => {
            let mut s1: ArcStr;
            let mut s2: ArcStr;
            s1 = SCodeDump::equationStr(inEquation, SCodeDump::defaultOptions.clone())?;
            s2 = PrefixUtil::printPrefixStr3(inPrefix)?;
            Error::addSourceMessage(&(Error::WRONG_TYPE_OR_NO_OF_ARGS.clone()), list![s1, s2], info)?;
            return Err("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outCref, outPriority))
}

fn uniqueRootArguments(
    mut inFunctionArgs: &metamodelica::Ref<Absyn::FunctionArgs>,
    mut info: &SourceInfo,
    mut inPrefix: DAE::Prefix,
    mut inEquation: metamodelica::Ref<SCode::Equation>,
) -> Result<(metamodelica::Ref<Absyn::ComponentRef>, metamodelica::Ref<Absyn::Exp>)> {
    let mut outCref: metamodelica::Ref<Absyn::ComponentRef>;
    let mut outMessage: metamodelica::Ref<Absyn::Exp>;
    (outCref, outMessage) = (::match_deref::match_deref! { match inFunctionArgs {
        Deref @ Absyn::FunctionArgs::FUNCTIONARGS { args: Deref @ metamodelica::ListNode::Cons { head: Deref @ Absyn::Exp::CREF { componentRef: cr }, tail: Deref @ metamodelica::ListNode::Nil }, argNames: Deref @ metamodelica::ListNode::Nil } => {
            (cr.clone(), metamodelica::Ref::new(Absyn::Exp::STRING { value: literal!("") }))
        },
        Deref @ Absyn::FunctionArgs::FUNCTIONARGS { args: Deref @ metamodelica::ListNode::Cons { head: Deref @ Absyn::Exp::CREF { componentRef: cr }, tail: Deref @ metamodelica::ListNode::Cons { head: msg, tail: Deref @ metamodelica::ListNode::Nil } }, argNames: Deref @ metamodelica::ListNode::Nil } => {
            (cr.clone(), msg.clone())
        },
        Deref @ Absyn::FunctionArgs::FUNCTIONARGS { args: Deref @ metamodelica::ListNode::Cons { head: Deref @ Absyn::Exp::CREF { componentRef: cr }, tail: Deref @ metamodelica::ListNode::Nil }, argNames: Deref @ metamodelica::ListNode::Cons { head: Deref @ Absyn::NamedArg { argName: Deref @ "message", argValue: msg }, tail: Deref @ metamodelica::ListNode::Nil } } => {
            (cr.clone(), msg.clone())
        },
        _ => {
            let mut s1: ArcStr;
            let mut s2: ArcStr;
            s1 = SCodeDump::equationStr(inEquation, SCodeDump::defaultOptions.clone())?;
            s2 = PrefixUtil::printPrefixStr3(inPrefix)?;
            Error::addSourceMessage(&(Error::WRONG_TYPE_OR_NO_OF_ARGS.clone()), list![s1, s2], info)?;
            return Err("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outCref, outMessage))
}

fn checkReinitType(
    mut inType: &metamodelica::Ref<DAE::Type>,
    mut inProperties: &DAE::Properties,
    mut inCref: &metamodelica::Ref<DAE::ComponentRef>,
    mut inInfo: &SourceInfo,
) -> bool {
    let mut outSucceeded: bool;
    outSucceeded = 'mc: {
        let __mc_input = inProperties.clone();
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            let mut ty: metamodelica::Ref<DAE::Type>;
            let mut cref_str: ArcStr;
            let mut ty_str: ArcStr;
            ty = Types::arrayElementType(inType);
            let false = (Types::isReal(&ty)) else {
                return Err("pattern mismatch");
            };
            cref_str = ComponentReferenceBasics::printComponentRefStr(inCref)?;
            ty_str = TypesDump::unparseType(ty.clone())?;
            Error::addSourceMessage(
                &(Error::REINIT_MUST_BE_REAL.clone()),
                list![cref_str.clone(), ty_str.clone()],
                inInfo,
            )?;
            Ok(false)
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let DAE::Properties::PROP {
                constFlag: mut cnst, ..
            } = __mc_input.clone()
            else {
                return Err("nomatch");
            };
            let mut cref_str: ArcStr;
            let mut cnst_str: ArcStr;
            let false = (Types::isVar(cnst.clone())) else {
                return Err("pattern mismatch");
            };
            cnst_str = TypesDump::unparseConst(cnst.clone());
            cref_str = ComponentReferenceBasics::printComponentRefStr(inCref)?;
            Error::addSourceMessage(
                &(Error::REINIT_MUST_BE_VAR.clone()),
                list![cref_str.clone(), cnst_str.clone()],
                inInfo,
            )?;
            Ok(false)
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            Ok(true)
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    outSucceeded
}

fn checkTupleCallEquationMessage(
    mut left: metamodelica::Ref<Absyn::Exp>,
    mut right: metamodelica::Ref<Absyn::Exp>,
    mut info: &SourceInfo,
) -> Result<()> {
    let () = (::match_deref::match_deref! { match &((AbsynUtil::stripCommentExpressions(left.clone(), false)?, AbsynUtil::stripCommentExpressions(right.clone(), false)?)) {
        (Deref @ Absyn::Exp::TUPLE { expressions: Deref @ metamodelica::ListNode::Cons { head: _, tail: Deref @ metamodelica::ListNode::Nil } }, _) => {
            ()
        },
        (Deref @ Absyn::Exp::TUPLE { expressions: crefs }, Deref @ Absyn::Exp::CALL { .. }) => {
            let mut left_str: ArcStr;
            let mut right_str: ArcStr;
            if !(List::all(metamodelica::AsArg::as_arg(&crefs), &move |__a0: metamodelica::Ref<Absyn::Exp>| -> metamodelica::Result<_> { ::std::result::Result::Ok(AbsynUtil::isCref(&__a0)) })?) {
                left_str = Dump::printExpStr(left)?;
                right_str = Dump::printExpStr(right)?;
                Error::addSourceMessageAndFail(&(Error::TUPLE_ASSIGN_CREFS_ONLY.clone()), list![{ let mut __mm_s = String::new(); __mm_s.push_str(&*left_str); __mm_s.push_str(&*literal!(" = ")); __mm_s.push_str(&*right_str); __mm_s.push_str(&*literal!(";")); ArcStr::from(__mm_s) }], info)?;
                unreachable!("Error.addSourceMessageAndFail always fails — caller-side flow-analysis hint");
            }
            ()
        },
        (Deref @ Absyn::Exp::TUPLE { .. }, _) => {
            let mut left_str: ArcStr;
            let mut right_str: ArcStr;
            left_str = Dump::printExpStr(left)?;
            right_str = Dump::printExpStr(right)?;
            Error::addSourceMessage(&(Error::TUPLE_ASSIGN_FUNCALL_ONLY.clone()), list![{ let mut __mm_s = String::new(); __mm_s.push_str(&*left_str); __mm_s.push_str(&*literal!(" = ")); __mm_s.push_str(&*right_str); __mm_s.push_str(&*literal!(";")); ArcStr::from(__mm_s) }], info)?;
            return Err("fail")
        },
        _ => {
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(())
}

fn instEquationNoRetCallVectorization(
    mut exp: metamodelica::Ref<DAE::Exp>,
    mut initial_: SCode::Initial,
    mut source: metamodelica::Ref<DAE::ElementSource>,
) -> DAE::DAElist {
    let mut dae: DAE::DAElist;
    dae = (match initial_ {
        SCode::Initial::NON_INITIAL { .. } => DAE::DAElist {
            elementLst: list![metamodelica::Ref::new(DAE::Element::NORETCALL {
                exp: exp,
                source: source
            })],
        },
        SCode::Initial::INITIAL { .. } => DAE::DAElist {
            elementLst: list![metamodelica::Ref::new(DAE::Element::INITIAL_NORETCALL {
                exp: exp,
                source: source
            })],
        },
    });
    dae
}

fn makeDAEArrayEqToReinitForm(mut inEq: &metamodelica::Ref<DAE::Element>) -> Result<metamodelica::Ref<DAE::Element>> {
    let mut outEqn: metamodelica::Ref<DAE::Element>;
    outEqn = (::match_deref::match_deref! { match inEq {
        Deref @ DAE::Element::EQUATION { exp: Deref @ DAE::Exp::CREF { componentRef: cr1, .. }, scalar: e, source } => {
            metamodelica::Ref::new(DAE::Element::REINIT { componentRef: cr1.clone(), exp: e.clone(), source: source.clone() })
        },
        Deref @ DAE::Element::DEFINE { componentRef: cr1, exp: e, source } => {
            metamodelica::Ref::new(DAE::Element::REINIT { componentRef: cr1.clone(), exp: e.clone(), source: source.clone() })
        },
        Deref @ DAE::Element::EQUEQUATION { cr1, cr2, source } => {
            let mut e2: metamodelica::Ref<DAE::Exp>;
            let mut t: metamodelica::Ref<DAE::Type>;
            t = ComponentReference::crefLastType(cr2)?;
            e2 = Expression::makeCrefExp(cr2.clone(), t)?;
            metamodelica::Ref::new(DAE::Element::REINIT { componentRef: cr1.clone(), exp: e2, source: source.clone() })
        },
        Deref @ DAE::Element::ARRAY_EQUATION { exp: Deref @ DAE::Exp::CREF { componentRef: cr1, .. }, array: e, source, .. } => {
            metamodelica::Ref::new(DAE::Element::REINIT { componentRef: cr1.clone(), exp: e.clone(), source: source.clone() })
        },
        _ => {
            let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { return Err("pattern mismatch") };
            Debug::traceln(literal!("Failure in: makeDAEArrayEqToReinitForm"))?;
            return Err("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outEqn)
}

fn condenseArrayEquation(
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
    mut ie1: metamodelica::Ref<Absyn::Exp>,
    mut ie2: metamodelica::Ref<Absyn::Exp>,
    mut elabedE1: metamodelica::Ref<DAE::Exp>,
    mut elabedE2: metamodelica::Ref<DAE::Exp>,
    mut iprop: DAE::Properties,
    mut iprop2: DAE::Properties,
    mut r#impl: bool,
    mut inPrefix: DAE::Prefix,
    mut info: SourceInfo,
) -> (
    FCore::Cache,
    metamodelica::Ref<DAE::Exp>,
    metamodelica::Ref<DAE::Exp>,
    DAE::Properties,
) {
    let mut outCache: FCore::Cache;
    let mut outE1: metamodelica::Ref<DAE::Exp>;
    let mut outE2: metamodelica::Ref<DAE::Exp>;
    let mut oprop: DAE::Properties;
    (outCache, outE1, outE2, oprop) = 'mc: {
        let __mc_input = (inCache, inEnv, ie1, ie2, iprop, iprop2, inPrefix);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, e1, e2, prop, prop2, pre) => {
                    let mut b3: bool;
                    let mut b4: bool;
                    let mut elabedE1_2: metamodelica::Ref<DAE::Exp>;
                    let mut elabedE2_2: metamodelica::Ref<DAE::Exp>;
                    let mut prop1: DAE::Properties;
                    let mut cache = (*cache).clone();
                    let mut e1 = (*e1).clone();
                    let mut prop = (*prop).clone();
                    let mut prop2 = (*prop2).clone();
                    let true = (Flags::getConfigBool(Flags::CONDENSE_ARRAYS.clone())?) else { return Err("pattern mismatch") };
                    b3 = Types::isPropTupleArray(metamodelica::AsArg::as_arg(&prop));
                    b4 = Types::isPropTupleArray(metamodelica::AsArg::as_arg(&prop2));
                    let true = (boolOr(b3, b4)) else { return Err("pattern mismatch") };
                    let true = (Expression::containFunctioncall(elabedE2.clone())?) else { return Err("pattern mismatch") };
                    (e1, prop) = expandTupleEquationWithWild(e1.clone(), metamodelica::AsArg::as_arg(&prop2), prop.clone())?;
                    (cache, elabedE1_2, prop1) = Static::elabExpLHS(cache.clone(), env.clone(), e1.clone(), r#impl, false, pre.clone(), info.clone())?;
                    (cache, elabedE1_2, prop1) = Ceval::cevalIfConstant(cache.clone(), env.clone(), elabedE1_2.clone(), prop1.clone(), r#impl, info.clone())?;
                    (cache, elabedE2_2, prop2) = Static::elabExp(cache.clone(), env.clone(), e2.clone(), r#impl, false, pre.clone(), info.clone())?;
                    (cache, elabedE2_2, prop2) = Ceval::cevalIfConstant(cache.clone(), env.clone(), elabedE2_2.clone(), prop2.clone(), r#impl, info.clone())?;
                    Ok((cache.clone(), elabedE1_2.clone(), elabedE2_2.clone(), prop.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, _, _, _, prop, _, _) => {
                    Ok((cache.clone(), elabedE1.clone(), elabedE2.clone(), prop.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    (outCache, outE1, outE2, oprop)
}

fn expandTupleEquationWithWild(
    mut inExp: metamodelica::Ref<Absyn::Exp>,
    mut propCall: &DAE::Properties,
    mut propTuple: DAE::Properties,
) -> Result<(metamodelica::Ref<Absyn::Exp>, DAE::Properties)> {
    let mut outExp: metamodelica::Ref<Absyn::Exp>;
    let mut oprop: DAE::Properties;
    (outExp, oprop) = (::match_deref::match_deref! { match &((inExp.clone(), propCall.clone(), propTuple.clone())) {
        (Deref @ Absyn::Exp::TUPLE { expressions: aexpl }, DAE::Properties::PROP_TUPLE { type_: Deref @ DAE::Type::T_TUPLE { types: typeList, names }, .. }, DAE::Properties::PROP_TUPLE { type_: Deref @ DAE::Type::T_TUPLE { types: lst, .. }, tupleConst: Deref @ DAE::TupleConst::TUPLE_CONST { tupleConstLst: tupleConst } }) => {
            let mut aexpl2: metamodelica::List<metamodelica::Ref<Absyn::Exp>>;
            let mut fillValue: i32;
            let mut lst2: metamodelica::List<metamodelica::Ref<DAE::Type>>;
            let mut tupleConst2: metamodelica::List<metamodelica::Ref<DAE::TupleConst>>;
            fillValue = ((typeList).len() as i32) - ((aexpl).len() as i32);
            lst2 = List::fill(DAE::T_ANYTYPE_DEFAULT().clone(), fillValue);
            aexpl2 = List::fill(metamodelica::Ref::new(Absyn::Exp::CREF { componentRef: openmodelica_ast::Absyn::ComponentRef::interned_WILD() }), fillValue);
            tupleConst2 = List::fill(metamodelica::Ref::new(DAE::TupleConst::SINGLE_CONST { r#const: openmodelica_frontend_types::DAE::Const::C_VAR }), fillValue);
            aexpl2 = listAppend(aexpl.clone(), aexpl2);
            lst2 = listAppend(lst.clone(), lst2);
            tupleConst2 = listAppend(tupleConst.clone(), tupleConst2);
            (metamodelica::Ref::new(Absyn::Exp::TUPLE { expressions: aexpl2 }), DAE::Properties::PROP_TUPLE { type_: metamodelica::Ref::new(DAE::Type::T_TUPLE { types: lst2, names: names.clone() }), tupleConst: metamodelica::Ref::new(DAE::TupleConst::TUPLE_CONST { tupleConstLst: tupleConst2 }) })
        },
        (_, DAE::Properties::PROP_TUPLE { type_: Deref @ DAE::Type::T_TUPLE { types: typeList, names }, .. }, DAE::Properties::PROP { type_: propType, constFlag: tconst }) => {
            let mut aexpl: metamodelica::List<metamodelica::Ref<Absyn::Exp>>;
            let mut aexpl2: metamodelica::List<metamodelica::Ref<Absyn::Exp>>;
            let mut fillValue: i32;
            let mut lst: metamodelica::List<metamodelica::Ref<DAE::Type>>;
            let mut lst2: metamodelica::List<metamodelica::Ref<DAE::Type>>;
            let mut tupleConst: metamodelica::List<metamodelica::Ref<DAE::TupleConst>>;
            let mut tupleConst2: metamodelica::List<metamodelica::Ref<DAE::TupleConst>>;
            fillValue = ((typeList).len() as i32) - 1;
            aexpl2 = List::fill(metamodelica::Ref::new(Absyn::Exp::CREF { componentRef: openmodelica_ast::Absyn::ComponentRef::interned_WILD() }), fillValue);
            lst2 = List::fill(DAE::T_ANYTYPE_DEFAULT().clone(), fillValue);
            tupleConst2 = List::fill(metamodelica::Ref::new(DAE::TupleConst::SINGLE_CONST { r#const: openmodelica_frontend_types::DAE::Const::C_VAR }), fillValue);
            aexpl = metamodelica::cons(inExp, aexpl2);
            lst = metamodelica::cons(propType.clone(), lst2);
            tupleConst = metamodelica::cons(metamodelica::Ref::new(DAE::TupleConst::SINGLE_CONST { r#const: tconst.clone() }), tupleConst2);
            (metamodelica::Ref::new(Absyn::Exp::TUPLE { expressions: aexpl }), DAE::Properties::PROP_TUPLE { type_: metamodelica::Ref::new(DAE::Type::T_TUPLE { types: lst, names: names.clone() }), tupleConst: metamodelica::Ref::new(DAE::TupleConst::TUPLE_CONST { tupleConstLst: tupleConst }) })
        },
        (_, _, _) if (!(Types::isPropTuple(propCall))) => {
            (inExp, propTuple)
        },
        _ => {
            let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { return Err("pattern mismatch") };
            Debug::traceln(literal!("- expandTupleEquationWithWild failed"))?;
            return Err("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outExp, oprop))
}

fn instEquationCommonCiTrans(mut inState: ClassInf::State, mut inInitial: SCode::Initial) -> Result<ClassInf::State> {
    let mut outState: ClassInf::State;
    outState = (match inInitial {
        SCode::Initial::NON_INITIAL { .. } => {
            ClassInfUtil::trans(inState, openmodelica_frontend_types::ClassInf::Event::FOUND_EQUATION)?
        }
        _ => inState,
    });
    Ok(outState)
}

fn unroll(
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
    mut inIH: metamodelica::List<InnerOuter::TopInstance>,
    mut inPrefix: DAE::Prefix,
    mut inSets: DAE::Connect::Sets,
    mut inState: ClassInf::State,
    mut inIdent: Ident,
    mut inIteratorType: metamodelica::Ref<DAE::Type>,
    mut inValue: &metamodelica::Ref<Values::Value>,
    mut inEquations: &metamodelica::List<metamodelica::Ref<SCode::Equation>>,
    mut inInitial: SCode::Initial,
    mut inImplicit: bool,
    mut inGraph: ConnectionGraph::ConnectionGraph,
) -> Result<(
    FCore::Cache,
    DAE::DAElist,
    DAE::Connect::Sets,
    ConnectionGraph::ConnectionGraph,
)> {
    let mut outCache: FCore::Cache = inCache;
    let mut outDae: DAE::DAElist;
    let mut outSets: DAE::Connect::Sets = inSets;
    let mut outGraph: ConnectionGraph::ConnectionGraph = inGraph;
    let mut values: metamodelica::List<metamodelica::Ref<Values::Value>>;
    let mut env: FCore::Graph;
    let mut ci_state: ClassInf::State = inState;
    let mut daes: metamodelica::List<DAE::DAElist> = metamodelica::nil();
    let mut dae: DAE::DAElist;
    match '__try0: {
        let __pa1 = ::match_deref::match_deref! { match &((*inValue)) {
            Deref @ Values::Value::ARRAY { valueLst: __pa1, .. } => __pa1.clone(),
            _ => break '__try0 Err::<_, _>("pattern mismatch"),
        } };
        values = metamodelica::Own::own(__pa1);
        for mut val in &*values {
            env = unwrap_break_err!(FGraph::openScope(inEnv.clone(), openmodelica_frontend_types::SCode::Encapsulated::NOT_ENCAPSULATED, arcstr::literal!(FCore::forScopeName), None), '__try0);
            env = unwrap_break_err!(FGraph::addForIterator(env.clone(), inIdent.clone(), inIteratorType.clone(), metamodelica::Ref::new(DAE::Binding::VALBOUND { valBound: val.clone(), source: openmodelica_frontend_types::DAE::BindingSource::BINDING_FROM_DEFAULT_VALUE }), openmodelica_frontend_types::SCode::Variability::CONST, Some(openmodelica_frontend_types::DAE::Const::C_CONST)), '__try0);
            (outCache, _, _, dae, outSets, ci_state, outGraph) = unwrap_break_err!(Inst::instList(outCache.clone(), env.clone(), inIH.clone(), inPrefix.clone(), outSets.clone(), ci_state.clone(), &*(if (SCodeUtil::isInitial(inInitial)) { (std::sync::Arc::new(instInitialEquation) as std::sync::Arc<dyn ::std::ops::Fn(FCore::Cache, FCore::Graph, metamodelica::List<InnerOuter::TopInstance>, DAE::Prefix, DAE::Connect::Sets, ClassInf::State, metamodelica::Ref<SCode::Equation>, bool, bool, ConnectionGraph::ConnectionGraph) -> Result<(FCore::Cache, FCore::Graph, metamodelica::List<InnerOuter::TopInstance>, DAE::DAElist, DAE::Connect::Sets, ClassInf::State, ConnectionGraph::ConnectionGraph)> + 'static>) } else { (std::sync::Arc::new(instEquation) as std::sync::Arc<dyn ::std::ops::Fn(FCore::Cache, FCore::Graph, metamodelica::List<InnerOuter::TopInstance>, DAE::Prefix, DAE::Connect::Sets, ClassInf::State, metamodelica::Ref<SCode::Equation>, bool, bool, ConnectionGraph::ConnectionGraph) -> Result<(FCore::Cache, FCore::Graph, metamodelica::List<InnerOuter::TopInstance>, DAE::DAElist, DAE::Connect::Sets, ClassInf::State, ConnectionGraph::ConnectionGraph)> + 'static>) }), inEquations, inImplicit, alwaysUnroll.clone(), outGraph.clone()), '__try0);
            daes = metamodelica::cons(dae.clone(), daes.clone());
        }
        outDae = unwrap_break_err!(List::fold(&daes, &move |__a0: DAE::DAElist, __a1: DAE::DAElist| DAEUtil::joinDaes(&__a0, &__a1), DAE::emptyDae().clone()), '__try0);
        Ok::<_, &'static str>((outDae.clone(), values.clone()))
    } {
        Ok((__try0_o0, __try0_o1)) => {
            outDae = __try0_o0;
            values = __try0_o1;
        }
        Err(__try0_err) => {
            let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else {
                return Err("pattern mismatch");
            };
            Debug::traceln({
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("- InstSection.unroll failed: "));
                __mm_s.push_str(&*ValuesDump::valString(inValue)?);
                ArcStr::from(__mm_s)
            })?;
            return Err(__try0_err);
        }
    }
    Ok((outCache, outDae, outSets, outGraph))
}

fn addForLoopScope(
    mut env: FCore::Graph,
    mut iterName: Ident,
    mut iterType: metamodelica::Ref<DAE::Type>,
    mut iterVariability: SCode::Variability,
    mut constOfForIteratorRange: Option<DAE::Const>,
) -> Result<FCore::Graph> {
    let mut newEnv: FCore::Graph;
    newEnv = FGraph::openScope(
        env,
        openmodelica_frontend_types::SCode::Encapsulated::NOT_ENCAPSULATED,
        arcstr::literal!(FCore::forScopeName),
        None,
    )?;
    newEnv = FGraph::addForIterator(
        newEnv,
        iterName,
        iterType,
        openmodelica_frontend_types::DAE::Binding::interned_UNBOUND(),
        iterVariability,
        constOfForIteratorRange,
    )?;
    Ok(newEnv)
}

fn addParForLoopScope(
    mut env: FCore::Graph,
    mut iterName: Ident,
    mut iterType: metamodelica::Ref<DAE::Type>,
    mut iterVariability: SCode::Variability,
    mut constOfForIteratorRange: Option<DAE::Const>,
) -> Result<FCore::Graph> {
    let mut newEnv: FCore::Graph;
    newEnv = FGraph::openScope(
        env,
        openmodelica_frontend_types::SCode::Encapsulated::NOT_ENCAPSULATED,
        arcstr::literal!(FCore::parForScopeName),
        None,
    )?;
    newEnv = FGraph::addForIterator(
        newEnv,
        iterName,
        iterType,
        openmodelica_frontend_types::DAE::Binding::interned_UNBOUND(),
        iterVariability,
        constOfForIteratorRange,
    )?;
    Ok(newEnv)
}

pub(crate) fn instEqEquation(
    mut inExp1: metamodelica::Ref<DAE::Exp>,
    mut inProperties2: DAE::Properties,
    mut inExp3: metamodelica::Ref<DAE::Exp>,
    mut inProperties4: DAE::Properties,
    mut source: metamodelica::Ref<DAE::ElementSource>,
    mut inInitial5: SCode::Initial,
    mut inImplicit: bool,
    mut extraInfo: SourceInfo,
) -> Result<DAE::DAElist> {
    let mut outDae: DAE::DAElist;
    outDae = 'mc: {
        let __mc_input = (inExp1, inProperties2, inExp3, inProperties4.clone(), inInitial5);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e1 @ Deref @ DAE::Exp::CREF { .. }, p1 @ DAE::Properties::PROP { type_: Deref @ DAE::Type::T_COMPLEX { complexClassType: ClassInf::State::RECORD { path: _ }, .. }, .. }, e2, p2 @ DAE::Properties::PROP { constFlag: c, .. }, initial_) => {
                    let mut e2_1: metamodelica::Ref<DAE::Exp>;
                    let mut t_1: metamodelica::Ref<DAE::Type>;
                    let mut dae: DAE::DAElist;
                    let mut e1 = (*e1).clone();
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(Types::matchProp(e2.clone(), metamodelica::AsArg::as_arg(&p2), metamodelica::AsArg::as_arg(&p1), true)?) {
                        (__pa0, DAE::Properties::PROP { type_: __pa1, constFlag: _ }) => (__pa0.clone(), __pa1.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    e2_1 = metamodelica::Own::own(__pa0);
                    t_1 = metamodelica::Own::own(__pa1);
                    (e1, _) = ExpressionSimplify::simplify(e1.clone())?;
                    (e2_1, _) = ExpressionSimplify::simplify(e2_1.clone())?;
                    dae = instEqEquation2(e1.clone(), e2_1.clone(), t_1.clone(), c.clone(), &source, initial_.clone())?;
                    Ok(dae.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e1, p1 @ DAE::Properties::PROP { .. }, e2, p2 @ DAE::Properties::PROP { constFlag: c, .. }, initial_) => {
                    let mut e1_1: metamodelica::Ref<DAE::Exp>;
                    let mut t_1: metamodelica::Ref<DAE::Type>;
                    let mut dae: DAE::DAElist;
                    let mut e2 = (*e2).clone();
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(Types::matchProp(e1.clone(), metamodelica::AsArg::as_arg(&p1), metamodelica::AsArg::as_arg(&p2), false)?) {
                        (__pa0, DAE::Properties::PROP { type_: __pa1, constFlag: _ }) => (__pa0.clone(), __pa1.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    e1_1 = metamodelica::Own::own(__pa0);
                    t_1 = metamodelica::Own::own(__pa1);
                    (e1_1, _) = ExpressionSimplify::simplify(e1_1.clone())?;
                    (e2, _) = ExpressionSimplify::simplify(e2.clone())?;
                    dae = instEqEquation2(e1_1.clone(), e2.clone(), t_1.clone(), c.clone(), &source, initial_.clone())?;
                    Ok(dae.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e1, p1 @ DAE::Properties::PROP { .. }, e2, p2 @ DAE::Properties::PROP { constFlag: c, .. }, initial_) => {
                    let mut e2_1: metamodelica::Ref<DAE::Exp>;
                    let mut t_1: metamodelica::Ref<DAE::Type>;
                    let mut dae: DAE::DAElist;
                    let mut e1 = (*e1).clone();
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(Types::matchProp(e2.clone(), metamodelica::AsArg::as_arg(&p2), metamodelica::AsArg::as_arg(&p1), true)?) {
                        (__pa0, DAE::Properties::PROP { type_: __pa1, constFlag: _ }) => (__pa0.clone(), __pa1.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    e2_1 = metamodelica::Own::own(__pa0);
                    t_1 = metamodelica::Own::own(__pa1);
                    (e1, _) = ExpressionSimplify::simplify(e1.clone())?;
                    (e2_1, _) = ExpressionSimplify::simplify(e2_1.clone())?;
                    dae = instEqEquation2(e1.clone(), e2_1.clone(), t_1.clone(), c.clone(), &source, initial_.clone())?;
                    Ok(dae.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e1, p1 @ DAE::Properties::PROP_TUPLE { .. }, e2, p2 @ DAE::Properties::PROP_TUPLE { tupleConst: tp, .. }, initial_) => {
                    let mut e1_1: metamodelica::Ref<DAE::Exp>;
                    let mut t_1: metamodelica::Ref<DAE::Type>;
                    let mut dae: DAE::DAElist;
                    let mut c: DAE::Const;
                    let mut e2 = (*e2).clone();
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(Types::matchProp(e1.clone(), metamodelica::AsArg::as_arg(&p1), metamodelica::AsArg::as_arg(&p2), false)?) {
                        (__pa0, DAE::Properties::PROP_TUPLE { type_: __pa1, tupleConst: _ }) => (__pa0.clone(), __pa1.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    e1_1 = metamodelica::Own::own(__pa0);
                    t_1 = metamodelica::Own::own(__pa1);
                    (e1_1, _) = ExpressionSimplify::simplify(e1_1.clone())?;
                    (e2, _) = ExpressionSimplify::simplify(e2.clone())?;
                    c = Types::propTupleAllConst(tp.clone())?;
                    dae = instEqEquation2(e1_1.clone(), e2.clone(), t_1.clone(), c, &source, initial_.clone())?;
                    Ok(dae.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e1, p1 @ DAE::Properties::PROP_TUPLE { .. }, e2, p2 @ DAE::Properties::PROP_TUPLE { tupleConst: tp, .. }, initial_) => {
                    let mut e2_1: metamodelica::Ref<DAE::Exp>;
                    let mut t_1: metamodelica::Ref<DAE::Type>;
                    let mut dae: DAE::DAElist;
                    let mut c: DAE::Const;
                    let mut e1 = (*e1).clone();
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(Types::matchProp(e2.clone(), metamodelica::AsArg::as_arg(&p2), metamodelica::AsArg::as_arg(&p1), true)?) {
                        (__pa0, DAE::Properties::PROP_TUPLE { type_: __pa1, tupleConst: _ }) => (__pa0.clone(), __pa1.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    e2_1 = metamodelica::Own::own(__pa0);
                    t_1 = metamodelica::Own::own(__pa1);
                    (e1, _) = ExpressionSimplify::simplify(e1.clone())?;
                    (e2_1, _) = ExpressionSimplify::simplify(e2_1.clone())?;
                    c = Types::propTupleAllConst(tp.clone())?;
                    dae = instEqEquation2(e1.clone(), e2_1.clone(), t_1.clone(), c, &source, initial_.clone())?;
                    Ok(dae.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e1 @ Deref @ DAE::Exp::CREF { .. }, DAE::Properties::PROP { type_: Deref @ DAE::Type::T_ENUMERATION { .. }, .. }, e2, DAE::Properties::PROP { type_: t @ Deref @ DAE::Type::T_ENUMERATION { .. }, constFlag: c }, initial_) => {
                    let mut dae: DAE::DAElist;
                    let mut e1 = (*e1).clone();
                    let mut e2 = (*e2).clone();
                    (e1, _) = ExpressionSimplify::simplify(e1.clone())?;
                    (e2, _) = ExpressionSimplify::simplify(e2.clone())?;
                    dae = instEqEquation2(e1.clone(), e2.clone(), t.clone(), c.clone(), &source, initial_.clone())?;
                    Ok(dae.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e1, p1 @ DAE::Properties::PROP { .. }, e2, DAE::Properties::PROP_TUPLE { .. }, initial_) => {
                    let mut t_1: metamodelica::Ref<DAE::Type>;
                    let mut dae: DAE::DAElist;
                    let mut p2: DAE::Properties;
                    let mut c: DAE::Const;
                    let mut e1 = (*e1).clone();
                    let mut e2 = (*e2).clone();
                    p2 = Types::propTupleFirstProp(inProperties4.clone())?;
                    let DAE::PROP { constFlag: __pa0, .. } = (p2.clone()) else { return Err("pattern mismatch") };
                    c = metamodelica::Own::own(__pa0);
                    let (__pa1, __pa2) = ::match_deref::match_deref! { match &(Types::matchProp(e1.clone(), metamodelica::AsArg::as_arg(&p1), &p2, false)?) {
                        (__pa1, DAE::Properties::PROP { type_: __pa2, .. }) => (__pa1.clone(), __pa2.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    e1 = metamodelica::Own::own(__pa1);
                    t_1 = metamodelica::Own::own(__pa2);
                    (e1, _) = ExpressionSimplify::simplify(e1.clone())?;
                    e2 = metamodelica::Ref::new(DAE::Exp::TSUB { exp: e2.clone(), ix: 1, ty: t_1.clone() });
                    (e2, _) = ExpressionSimplify::simplify(e2.clone())?;
                    dae = instEqEquation2(e1.clone(), e2.clone(), t_1.clone(), c, &source, initial_.clone())?;
                    Ok(dae.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e1, DAE::Properties::PROP { type_: t1, .. }, e2, DAE::Properties::PROP { type_: t2, .. }, _) => {
                    let mut e1_str: ArcStr;
                    let mut t1_str: ArcStr;
                    let mut e2_str: ArcStr;
                    let mut t2_str: ArcStr;
                    let mut s1: ArcStr;
                    let mut s2: ArcStr;
                    let mut info: SourceInfo;
                    e1_str = ExpressionBasics::printExpStr(e1.clone())?;
                    t1_str = TypesDump::unparseTypeNoAttr(metamodelica::AsArg::as_arg(&t1))?;
                    e2_str = ExpressionBasics::printExpStr(e2.clone())?;
                    t2_str = TypesDump::unparseTypeNoAttr(metamodelica::AsArg::as_arg(&t2))?;
                    s1 = stringAppendList(list![e1_str.clone(), literal!("="), e2_str.clone()]);
                    s2 = stringAppendList(list![t1_str.clone(), literal!("="), t2_str.clone()]);
                    info = ElementSource::getElementSourceFileInfo(source.clone());
                    Types::typeErrorSanityCheck(t1_str.clone(), &t2_str, &info)?;
                    Error::addMultiSourceMessage(&(Error::EQUATION_TYPE_MISMATCH_ERROR.clone()), &(list![s1.clone(), s2.clone()]), &(if (extraInfo.fileName.clone() == literal!("")) {list![info.clone()]} else {list![extraInfo.clone(), info.clone()]}))?;
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(outDae)
}

fn instEqEquation2(
    mut inExp1: metamodelica::Ref<DAE::Exp>,
    mut inExp2: metamodelica::Ref<DAE::Exp>,
    mut inType3: metamodelica::Ref<DAE::Type>,
    mut inConst: DAE::Const,
    mut source: &metamodelica::Ref<DAE::ElementSource>,
    mut inInitial4: SCode::Initial,
) -> Result<DAE::DAElist> {
    let mut outDae: DAE::DAElist;
    outDae = 'mc: {
        let __mc_input = (inExp1, inExp2, inType3.clone(), inInitial4);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e1, e2, Deref @ DAE::Type::T_INTEGER { .. }, initial_) => {
                    let mut dae: DAE::DAElist;
                    dae = makeDaeEquation(e1.clone(), e2.clone(), source.clone(), initial_.clone())?;
                    Ok(dae.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e1, e2, Deref @ DAE::Type::T_REAL { .. }, initial_) => {
                    let mut dae: DAE::DAElist;
                    dae = makeDaeEquation(e1.clone(), e2.clone(), source.clone(), initial_.clone())?;
                    Ok(dae.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e1, e2, Deref @ DAE::Type::T_STRING { .. }, initial_) => {
                    let mut dae: DAE::DAElist;
                    dae = makeDaeEquation(e1.clone(), e2.clone(), source.clone(), initial_.clone())?;
                    Ok(dae.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e1, e2, Deref @ DAE::Type::T_BOOL { .. }, initial_) => {
                    let mut dae: DAE::DAElist;
                    dae = makeDaeEquation(e1.clone(), e2.clone(), source.clone(), initial_.clone())?;
                    Ok(dae.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e1, e2, Deref @ DAE::Type::T_CLOCK { .. }, initial_) => {
                    let mut dae: DAE::DAElist;
                    dae = makeDaeEquation(e1.clone(), e2.clone(), source.clone(), initial_.clone())?;
                    Ok(dae.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CREF { componentRef: cr, .. }, e2, Deref @ DAE::Type::T_ENUMERATION { .. }, initial_) => {
                    let mut dae: DAE::DAElist;
                    dae = makeDaeDefine(cr.clone(), e2.clone(), source.clone(), initial_.clone())?;
                    Ok(dae.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e1, Deref @ DAE::Exp::CREF { componentRef: cr, .. }, Deref @ DAE::Type::T_ENUMERATION { .. }, initial_) => {
                    Ok(makeDaeDefine(cr.clone(), e1.clone(), source.clone(), initial_.clone())?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e1, e2, Deref @ DAE::Type::T_ENUMERATION { .. }, initial_) => {
                    Ok(makeDaeEquation(e1.clone(), e2.clone(), source.clone(), initial_.clone())?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e1, e2, tt @ Deref @ DAE::Type::T_ARRAY { .. }, initial_) => {
                    let mut dae: DAE::DAElist;
                    dae = instArrayEquation(e1.clone(), e2.clone(), metamodelica::AsArg::as_arg(&tt), inConst, source.clone(), initial_.clone())?;
                    Ok(dae.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::TUPLE { PR: exps1 }, e2, Deref @ DAE::Type::T_TUPLE { types: Deref @ metamodelica::ListNode::Cons { head: _, tail: _ }, .. }, initial_) => {
                    let mut dae: DAE::DAElist;
                    let mut e1: metamodelica::Ref<DAE::Exp>;
                    let mut exps1 = (*exps1).clone();
                    exps1 = List::map(exps1.clone(), &Expression::emptyToWild)?;
                    checkNoDuplicateAssignments(exps1.clone(), &(ElementSource::getElementSourceFileInfo(source.clone())))?;
                    e1 = metamodelica::Ref::new(DAE::Exp::TUPLE { PR: exps1.clone() });
                    dae = makeDaeEquation(e1.clone(), e2.clone(), source.clone(), initial_.clone())?;
                    Ok(dae.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e1, e2, Deref @ DAE::Type::T_TUPLE { .. }, initial_) => {
                    if !((!(Expression::isTuple(metamodelica::AsArg::as_arg(&e1))))) { return Err("guard") }
                    let mut dae: DAE::DAElist;
                    dae = makeDaeEquation(e1.clone(), e2.clone(), source.clone(), initial_.clone())?;
                    Ok(dae.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e1, e2, Deref @ DAE::Type::T_METALIST { .. }, initial_) => {
                    let mut dae: DAE::DAElist;
                    let true = (Config::acceptMetaModelicaGrammar()?) else { return Err("pattern mismatch") };
                    dae = makeDaeEquation(e1.clone(), e2.clone(), source.clone(), initial_.clone())?;
                    Ok(dae.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e1, e2, Deref @ DAE::Type::T_METATUPLE { .. }, initial_) => {
                    let mut dae: DAE::DAElist;
                    let true = (Config::acceptMetaModelicaGrammar()?) else { return Err("pattern mismatch") };
                    dae = makeDaeEquation(e1.clone(), e2.clone(), source.clone(), initial_.clone())?;
                    Ok(dae.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e1, e2, Deref @ DAE::Type::T_METAOPTION { .. }, initial_) => {
                    let mut dae: DAE::DAElist;
                    let true = (Config::acceptMetaModelicaGrammar()?) else { return Err("pattern mismatch") };
                    dae = makeDaeEquation(e1.clone(), e2.clone(), source.clone(), initial_.clone())?;
                    Ok(dae.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e1, e2, Deref @ DAE::Type::T_METAUNIONTYPE { .. }, initial_) => {
                    let mut dae: DAE::DAElist;
                    let true = (Config::acceptMetaModelicaGrammar()?) else { return Err("pattern mismatch") };
                    dae = makeDaeEquation(e1.clone(), e2.clone(), source.clone(), initial_.clone())?;
                    Ok(dae.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e1, e2, Deref @ DAE::Type::T_SUBTYPE_BASIC { complexType: tt, .. }, initial_) => {
                    let mut dae: DAE::DAElist;
                    dae = instEqEquation2(e1.clone(), e2.clone(), tt.clone(), inConst, source, initial_.clone())?;
                    Ok(dae.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e1, e2, Deref @ DAE::Type::T_COMPLEX { varLst: vs, .. }, initial_) => {
                    let mut dae: DAE::DAElist;
                    let mut exps1: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                    let mut exps2: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                    let mut tys: metamodelica::List<metamodelica::Ref<DAE::Type>>;
                    exps1 = Expression::splitRecord(metamodelica::AsArg::as_arg(&e1), &inType3)?;
                    exps2 = Expression::splitRecord(metamodelica::AsArg::as_arg(&e2), &inType3)?;
                    tys = List::map(vs.clone(), &move |__a0: metamodelica::Ref<DAE::Var>| Types::getVarType(&__a0))?;
                    dae = instEqEquation2List(&exps1, &exps2, &tys, inConst, source, initial_.clone(), metamodelica::nil())?;
                    Ok(dae.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e1, e2, tt @ Deref @ DAE::Type::T_COMPLEX { .. }, initial_) => {
                    let mut dae: DAE::DAElist;
                    dae = instComplexEquation(e1.clone(), e2.clone(), metamodelica::AsArg::as_arg(&tt), source.clone(), initial_.clone())?;
                    Ok(dae.clone())
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
                    Debug::trace(literal!("- InstSection.instEqEquation2 failed\n"))?;
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(outDae)
}

fn instEqEquation2List<'__b>(
    mut inExps1: &'__b metamodelica::List<metamodelica::Ref<DAE::Exp>>,
    mut inExps2: &'__b metamodelica::List<metamodelica::Ref<DAE::Exp>>,
    mut inTypes3: &'__b metamodelica::List<metamodelica::Ref<DAE::Type>>,
    mut r#const: DAE::Const,
    mut source: &'__b metamodelica::Ref<DAE::ElementSource>,
    mut initial_: SCode::Initial,
    mut acc: metamodelica::List<DAE::DAElist>,
) -> Result<DAE::DAElist> {
    '__tco: loop {
        ::match_deref::match_deref! { match (inExps1, inExps2, inTypes3) {
            (Deref @ metamodelica::ListNode::Nil, Deref @ metamodelica::ListNode::Nil, Deref @ metamodelica::ListNode::Nil) => {
                return Ok(DAEUtil::joinDaeLst(&(acc.reverse()))?)
            },
            (Deref @ metamodelica::ListNode::Cons { head: exp1, tail: rest1 }, Deref @ metamodelica::ListNode::Cons { head: exp2, tail: rest2 }, Deref @ metamodelica::ListNode::Cons { head: ty, tail: rest3 }) => {
                let mut res: DAE::DAElist;
                res = instEqEquation2(exp1.clone(), exp2.clone(), ty.clone(), r#const, source, initial_)?;
                { (inExps1, inExps2, inTypes3, r#const, source, initial_, acc) = (rest1, rest2, rest3, r#const, source, initial_, metamodelica::cons(res, acc)); continue '__tco; }
            },
            _ => return Err("match: no arm matched"),
        } }
    }
}

pub(crate) fn makeDaeEquation(
    mut inExp1: metamodelica::Ref<DAE::Exp>,
    mut inExp2: metamodelica::Ref<DAE::Exp>,
    mut inSource: metamodelica::Ref<DAE::ElementSource>,
    mut inInitial3: SCode::Initial,
) -> Result<DAE::DAElist> {
    let mut outDae: DAE::DAElist;
    outDae = (match inInitial3 {
        SCode::Initial::NON_INITIAL { .. } => {
            let mut e1 = inExp1;
            let mut e2 = inExp2;
            let mut source = inSource;
            let mut elt: metamodelica::Ref<DAE::Element>;
            elt = metamodelica::Ref::new(DAE::Element::EQUATION {
                exp: e1.clone(),
                scalar: e2.clone(),
                source: source.clone(),
            });
            source = ElementSource::addSymbolicTransformationFlattenedEqs(source, elt)?;
            DAE::DAElist {
                elementLst: list![metamodelica::Ref::new(DAE::Element::EQUATION {
                    exp: e1,
                    scalar: e2,
                    source: source
                })],
            }
        }
        SCode::Initial::INITIAL { .. } => {
            let mut e1 = inExp1;
            let mut e2 = inExp2;
            let mut source = inSource;
            let mut elt: metamodelica::Ref<DAE::Element>;
            elt = metamodelica::Ref::new(DAE::Element::INITIALEQUATION {
                exp1: e1.clone(),
                exp2: e2.clone(),
                source: source.clone(),
            });
            source = ElementSource::addSymbolicTransformationFlattenedEqs(source, elt)?;
            DAE::DAElist {
                elementLst: list![metamodelica::Ref::new(DAE::Element::INITIALEQUATION {
                    exp1: e1,
                    exp2: e2,
                    source: source
                })],
            }
        }
    });
    Ok(outDae)
}

fn makeDaeDefine(
    mut inComponentRef: metamodelica::Ref<DAE::ComponentRef>,
    mut inExp: metamodelica::Ref<DAE::Exp>,
    mut source: metamodelica::Ref<DAE::ElementSource>,
    mut inInitial: SCode::Initial,
) -> Result<DAE::DAElist> {
    let mut outDae: DAE::DAElist;
    outDae = (match inInitial {
        SCode::Initial::NON_INITIAL { .. } => {
            let mut cr = inComponentRef;
            let mut e2 = inExp;
            DAE::DAElist {
                elementLst: list![metamodelica::Ref::new(DAE::Element::DEFINE {
                    componentRef: cr,
                    exp: e2,
                    source: source
                })],
            }
        }
        SCode::Initial::INITIAL { .. } => {
            let mut cr = inComponentRef;
            let mut e2 = inExp;
            DAE::DAElist {
                elementLst: list![metamodelica::Ref::new(DAE::Element::INITIALDEFINE {
                    componentRef: cr,
                    exp: e2,
                    source: source
                })],
            }
        }
    });
    Ok(outDae)
}

fn instArrayEquation(
    mut lhs: metamodelica::Ref<DAE::Exp>,
    mut rhs: metamodelica::Ref<DAE::Exp>,
    mut tp: &metamodelica::Ref<DAE::Type>,
    mut inConst: DAE::Const,
    mut inSource: metamodelica::Ref<DAE::ElementSource>,
    mut initial_: SCode::Initial,
) -> Result<DAE::DAElist> {
    let mut dae: DAE::DAElist = <DAE::DAElist as ::std::default::Default>::default();
    dae = 'mc: {
        let __mc_input = (&**tp, inSource.clone(), initial_);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, source, SCode::Initial::INITIAL { .. }) => {
                    let mut b1: bool;
                    let mut b2: bool;
                    let mut ds: metamodelica::List<metamodelica::Ref<DAE::Dimension>>;
                    let mut elt: metamodelica::Ref<DAE::Element>;
                    let mut source = (*source).clone();
                    b1 = Expression::containVectorFunctioncall(lhs.clone())?;
                    b2 = Expression::containVectorFunctioncall(rhs.clone())?;
                    let true = (boolOr(b1, b2)) else { return Err("pattern mismatch") };
                    ds = TypesDump::getDimensions(tp);
                    elt = metamodelica::Ref::new(DAE::Element::INITIAL_ARRAY_EQUATION { dimension: ds.clone(), exp: lhs.clone(), array: rhs.clone(), source: source.clone() });
                    source = ElementSource::addSymbolicTransformationFlattenedEqs(source.clone(), elt.clone())?;
                    Ok(DAE::DAElist { elementLst: list![metamodelica::Ref::new(DAE::Element::INITIAL_ARRAY_EQUATION { dimension: ds.clone(), exp: lhs.clone(), array: rhs.clone(), source: source.clone() })] })
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, source, SCode::Initial::NON_INITIAL { .. }) => {
                    let mut b1: bool;
                    let mut b2: bool;
                    let mut ds: metamodelica::List<metamodelica::Ref<DAE::Dimension>>;
                    let mut elt: metamodelica::Ref<DAE::Element>;
                    let mut source = (*source).clone();
                    b1 = Expression::containVectorFunctioncall(lhs.clone())?;
                    b2 = Expression::containVectorFunctioncall(rhs.clone())?;
                    let true = (boolOr(b1, b2)) else { return Err("pattern mismatch") };
                    ds = TypesDump::getDimensions(tp);
                    elt = metamodelica::Ref::new(DAE::Element::ARRAY_EQUATION { dimension: ds.clone(), exp: lhs.clone(), array: rhs.clone(), source: source.clone() });
                    source = ElementSource::addSymbolicTransformationFlattenedEqs(source.clone(), elt.clone())?;
                    Ok(DAE::DAElist { elementLst: list![metamodelica::Ref::new(DAE::Element::ARRAY_EQUATION { dimension: ds.clone(), exp: lhs.clone(), array: rhs.clone(), source: source.clone() })] })
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Type::T_ARRAY { ty: t, dims: Deref @ metamodelica::ListNode::Cons { head: _, tail: Deref @ metamodelica::ListNode::Nil } }, _, _) => {
                    let mut lhs_dim: metamodelica::Ref<DAE::Dimension>;
                    let mut rhs_dim: metamodelica::Ref<DAE::Dimension>;
                    let mut lhs_idxs: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                    let mut rhs_idxs: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                    let mut dae: DAE::DAElist = dae.clone();
                    let false = (Config::splitArrays()?) else { return Err("pattern mismatch") };
                    let __pa0 = ::match_deref::match_deref! { match &(Expression::r#typeof(lhs.clone())?) {
                        Deref @ DAE::Type::T_ARRAY { dims: Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: _ }, .. } => __pa0.clone(),
                        _ => return Err("pattern mismatch"),
                    } };
                    lhs_dim = metamodelica::Own::own(__pa0);
                    let __pa1 = ::match_deref::match_deref! { match &(Expression::r#typeof(rhs.clone())?) {
                        Deref @ DAE::Type::T_ARRAY { dims: Deref @ metamodelica::ListNode::Cons { head: __pa1, tail: _ }, .. } => __pa1.clone(),
                        _ => return Err("pattern mismatch"),
                    } };
                    rhs_dim = metamodelica::Own::own(__pa1);
                    lhs_idxs = expandArrayDimension(&lhs_dim, lhs.clone())?;
                    rhs_idxs = expandArrayDimension(&rhs_dim, rhs.clone())?;
                    dae = instArrayElEq(&lhs, &rhs, t.clone(), inConst, lhs_idxs.clone(), rhs_idxs.clone(), &inSource, initial_)?;
                    Ok((dae.clone(), dae.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            dae = __wb0;
            break 'mc __v;
        }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Type::T_ARRAY { ty: t, dims: Deref @ metamodelica::ListNode::Cons { head: dim, tail: Deref @ metamodelica::ListNode::Nil } }, _, _) => {
                    let mut lhs_dim: metamodelica::Ref<DAE::Dimension>;
                    let mut rhs_dim: metamodelica::Ref<DAE::Dimension>;
                    let mut lhs_idxs: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                    let mut rhs_idxs: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                    let mut dae: DAE::DAElist = dae.clone();
                    let true = (Config::splitArrays()?) else { return Err("pattern mismatch") };
                    let true = (Expression::dimensionKnown(metamodelica::AsArg::as_arg(&dim))) else { return Err("pattern mismatch") };
                    let __pa0 = ::match_deref::match_deref! { match &(Expression::r#typeof(lhs.clone())?) {
                        Deref @ DAE::Type::T_ARRAY { dims: Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: _ }, .. } => __pa0.clone(),
                        _ => return Err("pattern mismatch"),
                    } };
                    lhs_dim = metamodelica::Own::own(__pa0);
                    let __pa1 = ::match_deref::match_deref! { match &(Expression::r#typeof(rhs.clone())?) {
                        Deref @ DAE::Type::T_ARRAY { dims: Deref @ metamodelica::ListNode::Cons { head: __pa1, tail: _ }, .. } => __pa1.clone(),
                        _ => return Err("pattern mismatch"),
                    } };
                    rhs_dim = metamodelica::Own::own(__pa1);
                    lhs_idxs = expandArrayDimension(&lhs_dim, lhs.clone())?;
                    rhs_idxs = expandArrayDimension(&rhs_dim, rhs.clone())?;
                    dae = instArrayElEq(&lhs, &rhs, t.clone(), inConst, lhs_idxs.clone(), rhs_idxs.clone(), &inSource, initial_)?;
                    Ok((dae.clone(), dae.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            dae = __wb0;
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Type::T_ARRAY { dims: Deref @ metamodelica::ListNode::Cons { head: dim, tail: Deref @ metamodelica::ListNode::Nil }, .. }, source, _) => {
                    let mut b: bool;
                    let mut ds: metamodelica::List<metamodelica::Ref<DAE::Dimension>>;
                    let mut elt: metamodelica::Ref<DAE::Element>;
                    let mut source = (*source).clone();
                    let true = (Config::splitArrays()?) else { return Err("pattern mismatch") };
                    let true = (Expression::dimensionKnown(metamodelica::AsArg::as_arg(&dim))) else { return Err("pattern mismatch") };
                    let true = (Expression::isRange(&lhs) || Expression::isRange(&rhs) || Expression::isReduction(&lhs) || Expression::isReduction(&rhs)) else { return Err("pattern mismatch") };
                    ds = TypesDump::getDimensions(tp);
                    b = SCodeUtil::isInitial(initial_);
                    elt = if (b) {metamodelica::Ref::new(DAE::Element::INITIAL_ARRAY_EQUATION { dimension: ds.clone(), exp: lhs.clone(), array: rhs.clone(), source: source.clone() })} else {metamodelica::Ref::new(DAE::Element::ARRAY_EQUATION { dimension: ds.clone(), exp: lhs.clone(), array: rhs.clone(), source: source.clone() })};
                    source = ElementSource::addSymbolicTransformationFlattenedEqs(source.clone(), elt.clone())?;
                    elt = if (b) {metamodelica::Ref::new(DAE::Element::INITIAL_ARRAY_EQUATION { dimension: ds.clone(), exp: lhs.clone(), array: rhs.clone(), source: source.clone() })} else {metamodelica::Ref::new(DAE::Element::ARRAY_EQUATION { dimension: ds.clone(), exp: lhs.clone(), array: rhs.clone(), source: source.clone() })};
                    Ok(DAE::DAElist { elementLst: list![elt.clone()] })
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Type::T_ARRAY { ty: t, dims: Deref @ metamodelica::ListNode::Cons { head: dim, tail: Deref @ metamodelica::ListNode::Nil } }, _, _) => {
                    let mut lhs_dim: metamodelica::Ref<DAE::Dimension>;
                    let mut rhs_dim: metamodelica::Ref<DAE::Dimension>;
                    let mut lhs_idxs: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                    let mut rhs_idxs: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                    let mut dae: DAE::DAElist = dae.clone();
                    let true = (Config::splitArrays()?) else { return Err("pattern mismatch") };
                    let false = (Expression::dimensionKnown(metamodelica::AsArg::as_arg(&dim))) else { return Err("pattern mismatch") };
                    let true = (Flags::getConfigBool(Flags::CHECK_MODEL.clone())?) else { return Err("pattern mismatch") };
                    let __pa0 = ::match_deref::match_deref! { match &(Expression::r#typeof(lhs.clone())?) {
                        Deref @ DAE::Type::T_ARRAY { dims: Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: _ }, .. } => __pa0.clone(),
                        _ => return Err("pattern mismatch"),
                    } };
                    lhs_dim = metamodelica::Own::own(__pa0);
                    let __pa1 = ::match_deref::match_deref! { match &(Expression::r#typeof(rhs.clone())?) {
                        Deref @ DAE::Type::T_ARRAY { dims: Deref @ metamodelica::ListNode::Cons { head: __pa1, tail: _ }, .. } => __pa1.clone(),
                        _ => return Err("pattern mismatch"),
                    } };
                    rhs_dim = metamodelica::Own::own(__pa1);
                    lhs_idxs = expandArrayDimension(&lhs_dim, lhs.clone())?;
                    rhs_idxs = expandArrayDimension(&rhs_dim, rhs.clone())?;
                    dae = instArrayElEq(&lhs, &rhs, t.clone(), inConst, lhs_idxs.clone(), rhs_idxs.clone(), &inSource, initial_)?;
                    Ok((dae.clone(), dae.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            dae = __wb0;
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Type::T_ARRAY { dims: Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Dimension::DIM_UNKNOWN { .. }, tail: Deref @ metamodelica::ListNode::Nil }, .. }, source, SCode::Initial::INITIAL { .. }) => {
                    let mut elt: metamodelica::Ref<DAE::Element>;
                    let mut source = (*source).clone();
                    let true = (Config::splitArrays()?) else { return Err("pattern mismatch") };
                    let true = (Flags::getConfigBool(Flags::CHECK_MODEL.clone())?) else { return Err("pattern mismatch") };
                    elt = metamodelica::Ref::new(DAE::Element::INITIAL_ARRAY_EQUATION { dimension: list![metamodelica::Ref::new(DAE::Dimension::DIM_INTEGER { integer: 1 })], exp: lhs.clone(), array: rhs.clone(), source: source.clone() });
                    source = ElementSource::addSymbolicTransformationFlattenedEqs(source.clone(), elt.clone())?;
                    Ok(DAE::DAElist { elementLst: list![metamodelica::Ref::new(DAE::Element::INITIAL_ARRAY_EQUATION { dimension: list![metamodelica::Ref::new(DAE::Dimension::DIM_INTEGER { integer: 1 })], exp: lhs.clone(), array: rhs.clone(), source: source.clone() })] })
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Type::T_ARRAY { dims: Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Dimension::DIM_UNKNOWN { .. }, tail: Deref @ metamodelica::ListNode::Nil }, .. }, source, SCode::Initial::NON_INITIAL { .. }) => {
                    let mut elt: metamodelica::Ref<DAE::Element>;
                    let mut source = (*source).clone();
                    let true = (Config::splitArrays()?) else { return Err("pattern mismatch") };
                    let true = (Flags::getConfigBool(Flags::CHECK_MODEL.clone())?) else { return Err("pattern mismatch") };
                    elt = metamodelica::Ref::new(DAE::Element::ARRAY_EQUATION { dimension: list![metamodelica::Ref::new(DAE::Dimension::DIM_INTEGER { integer: 1 })], exp: lhs.clone(), array: rhs.clone(), source: source.clone() });
                    source = ElementSource::addSymbolicTransformationFlattenedEqs(source.clone(), elt.clone())?;
                    Ok(DAE::DAElist { elementLst: list![metamodelica::Ref::new(DAE::Element::ARRAY_EQUATION { dimension: list![metamodelica::Ref::new(DAE::Dimension::DIM_INTEGER { integer: 1 })], exp: lhs.clone(), array: rhs.clone(), source: source.clone() })] })
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Type::T_ARRAY { dims: Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Dimension::DIM_UNKNOWN { .. }, tail: Deref @ metamodelica::ListNode::Nil }, .. }, _, _) => {
                    let mut lhs_str: ArcStr;
                    let mut rhs_str: ArcStr;
                    let mut eq_str: ArcStr;
                    let true = (Config::splitArrays()?) else { return Err("pattern mismatch") };
                    let false = (Flags::getConfigBool(Flags::CHECK_MODEL.clone())?) else { return Err("pattern mismatch") };
                    lhs_str = ExpressionBasics::printExpStr(lhs.clone())?;
                    rhs_str = ExpressionBasics::printExpStr(rhs.clone())?;
                    eq_str = stringAppendList(list![lhs_str.clone(), literal!("="), rhs_str.clone()]);
                    Error::addSourceMessage(&(Error::INST_ARRAY_EQ_UNKNOWN_SIZE.clone()), list![eq_str.clone()], &(ElementSource::getElementSourceFileInfo(inSource.clone())))?;
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
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { return Err("pattern mismatch") };
                    Debug::trace(literal!("- InstSection.instArrayEquation failed\n"))?;
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(dae)
}

fn instArrayElEq(
    mut inLhsExp: &metamodelica::Ref<DAE::Exp>,
    mut inRhsExp: &metamodelica::Ref<DAE::Exp>,
    mut inType: metamodelica::Ref<DAE::Type>,
    mut inConst: DAE::Const,
    mut inLhsIndices: metamodelica::List<metamodelica::Ref<DAE::Exp>>,
    mut inRhsIndices: metamodelica::List<metamodelica::Ref<DAE::Exp>>,
    mut inSource: &metamodelica::Ref<DAE::ElementSource>,
    mut inInitial: SCode::Initial,
) -> Result<DAE::DAElist> {
    let mut outDAE: DAE::DAElist = DAE::emptyDae().clone();
    let mut rhs_idx: metamodelica::Ref<DAE::Exp>;
    let mut rhs_idxs: metamodelica::List<metamodelica::Ref<DAE::Exp>> = inRhsIndices.clone().reverse();
    let mut dae: DAE::DAElist;
    for mut lhs_idx in &*inLhsIndices.reverse() {
        let (__pa0, __pa1) = ::match_deref::match_deref! { match &(rhs_idxs) {
            Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
            _ => return Err("pattern mismatch"),
        } };
        rhs_idx = metamodelica::Own::own(__pa0);
        rhs_idxs = metamodelica::Own::own(__pa1);
        dae = instEqEquation2(lhs_idx.clone(), rhs_idx, inType.clone(), inConst, inSource, inInitial)?;
        outDAE = DAEUtil::joinDaes(&dae, &outDAE)?;
    }
    Ok(outDAE)
}

fn unrollForLoop(
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
    mut inIH: metamodelica::List<InnerOuter::TopInstance>,
    mut inPrefix: DAE::Prefix,
    mut inState: &ClassInf::State,
    mut inIterator: ArcStr,
    mut inRange: metamodelica::Ref<DAE::Exp>,
    mut inRangeProps: DAE::Properties,
    mut inBody: metamodelica::List<metamodelica::Ref<SCode::Statement>>,
    mut inStatement: metamodelica::Ref<SCode::Statement>,
    mut inInfo: SourceInfo,
    mut inSource: &metamodelica::Ref<DAE::ElementSource>,
    mut inInitial: SCode::Initial,
    mut inImpl: bool,
    mut inUnrollLoops: bool,
) -> Result<(FCore::Cache, metamodelica::List<metamodelica::Ref<DAE::Statement>>)> {
    let mut outCache: FCore::Cache;
    let mut outStatements: metamodelica::List<metamodelica::Ref<DAE::Statement>>;
    let mut ty: metamodelica::Ref<DAE::Type>;
    let mut c: DAE::Const;
    let mut env: FCore::Graph;
    let mut val: metamodelica::Ref<Values::Value>;
    match '__try0: {
        let __pa1 = ::match_deref::match_deref! { match &(Types::getPropType(&inRangeProps)) {
            Deref @ DAE::Type::T_ARRAY { ty: __pa1, .. } => __pa1.clone(),
            _ => break '__try0 Err::<_, _>("pattern mismatch"),
        } };
        ty = metamodelica::Own::own(__pa1);
        c = unwrap_break_err!(Types::getPropConst(inRangeProps.clone()), '__try0);
        let true = (Types::isParameterOrConstant(c)) else {
            break '__try0 Err::<_, _>("pattern mismatch");
        };
        env = unwrap_break_err!(addForLoopScope(inEnv.clone(), inIterator.clone(), ty.clone(), openmodelica_frontend_types::SCode::Variability::VAR, Some(c)), '__try0);
        (outCache, val) = unwrap_break_err!(Ceval::ceval(inCache.clone(), env.clone(), inRange.clone(), inImpl, Absyn::Msg::MSG { info: inInfo.clone() }, 0), '__try0);
        (outCache, outStatements) = unwrap_break_err!(loopOverRange(inCache.clone(), env.clone(), inIH.clone(), inPrefix.clone(), inState, inIterator.clone(), val.clone(), inBody.clone(), inSource, inInitial, inImpl, inUnrollLoops), '__try0);
        Ok::<_, &'static str>((
            c.clone(),
            env.clone(),
            outCache.clone(),
            outStatements.clone(),
            ty.clone(),
            val.clone(),
        ))
    } {
        Ok((__try0_o0, __try0_o1, __try0_o2, __try0_o3, __try0_o4, __try0_o5)) => {
            c = __try0_o0;
            env = __try0_o1;
            outCache = __try0_o2;
            outStatements = __try0_o3;
            ty = __try0_o4;
            val = __try0_o5;
        }
        Err(_) => {
            Error::addSourceMessageAndFail(
                &(Error::UNROLL_LOOP_CONTAINING_WHEN.clone()),
                list![SCodeDump::statementStr(
                    inStatement.clone(),
                    SCodeDump::defaultOptions.clone()
                )?],
                &inInfo,
            )?;
            unreachable!("Error.addSourceMessageAndFail always fails — caller-side flow-analysis hint");
        }
    }
    Ok((outCache, outStatements))
}

fn instForStatement(
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
    mut inIH: metamodelica::List<InnerOuter::TopInstance>,
    mut inPrefix: DAE::Prefix,
    mut inState: &ClassInf::State,
    mut inForStatement: metamodelica::Ref<SCode::Statement>,
    mut inSource: metamodelica::Ref<DAE::ElementSource>,
    mut inInitial: SCode::Initial,
    mut inImpl: bool,
    mut inUnrollLoops: bool,
) -> Result<(FCore::Cache, metamodelica::List<metamodelica::Ref<DAE::Statement>>)> {
    let mut outCache: FCore::Cache;
    let mut outStatements: metamodelica::List<metamodelica::Ref<DAE::Statement>>;
    let mut iterator: ArcStr;
    let mut oarange: Option<metamodelica::Ref<Absyn::Exp>>;
    let mut arange: metamodelica::Ref<Absyn::Exp>;
    let mut range: metamodelica::Ref<DAE::Exp>;
    let mut prop: DAE::Properties;
    let mut body: metamodelica::List<metamodelica::Ref<SCode::Statement>>;
    let mut info: SourceInfo;
    let mut iter_crefs: metamodelica::List<(metamodelica::Ref<Absyn::ComponentRef>, i32)>;
    let (__pa0, __pa1, __pa2, __pa3) = ::match_deref::match_deref! { match &(inForStatement.clone()) {
        Deref @ SCode::Statement::ALG_FOR { index: __pa0, range: __pa1, forBody: __pa2, info: __pa3, .. } => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone()),
        _ => return Err("pattern mismatch"),
    } };
    iterator = metamodelica::Own::own(__pa0);
    oarange = metamodelica::Own::own(__pa1);
    body = metamodelica::Own::own(__pa2);
    info = metamodelica::Own::own(__pa3);
    if (oarange).is_some() {
        let __pa4 = ::match_deref::match_deref! { match &(oarange) {
            Some(__pa4) => __pa4.clone(),
            _ => return Err("pattern mismatch"),
        } };
        arange = metamodelica::Own::own(__pa4);
        (outCache, range, prop) = Static::elabExp(
            inCache.clone(),
            inEnv.clone(),
            arange,
            inImpl,
            true,
            inPrefix.clone(),
            info.clone(),
        )?;
    } else {
        iter_crefs = SCodeUtil::findIteratorIndexedCrefsInStatements(&body, iterator.clone(), metamodelica::nil())?;
        (range, prop, outCache) =
            Static::deduceIterationRange(iterator.clone(), &iter_crefs, inEnv.clone(), inCache.clone(), &info)?;
    }
    if containsWhenStatements(&body)? {
        (outCache, outStatements) = unrollForLoop(
            inCache,
            inEnv,
            inIH,
            inPrefix,
            inState,
            iterator,
            range,
            prop,
            body,
            inForStatement,
            info,
            &inSource,
            inInitial,
            inImpl,
            inUnrollLoops,
        )?;
    } else {
        (outCache, outStatements) = instForStatement_dispatch(
            inCache,
            inEnv,
            inIH,
            inPrefix,
            inState,
            iterator,
            range,
            prop,
            &body,
            info,
            inSource,
            inInitial,
            inImpl,
            inUnrollLoops,
        )?;
    }
    Ok((outCache, outStatements))
}

fn instForStatement_dispatch(
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
    mut inIH: metamodelica::List<InnerOuter::TopInstance>,
    mut inPrefix: DAE::Prefix,
    mut inState: &ClassInf::State,
    mut inIterator: ArcStr,
    mut inRange: metamodelica::Ref<DAE::Exp>,
    mut inRangeProps: DAE::Properties,
    mut inBody: &metamodelica::List<metamodelica::Ref<SCode::Statement>>,
    mut inInfo: SourceInfo,
    mut inSource: metamodelica::Ref<DAE::ElementSource>,
    mut inInitial: SCode::Initial,
    mut inImpl: bool,
    mut inUnrollLoops: bool,
) -> Result<(FCore::Cache, metamodelica::List<metamodelica::Ref<DAE::Statement>>)> {
    let mut outCache: FCore::Cache = inCache;
    let mut outStatements: metamodelica::List<metamodelica::Ref<DAE::Statement>>;
    let mut ty: metamodelica::Ref<DAE::Type>;
    let mut c: DAE::Const;
    let mut env: FCore::Graph;
    let mut source: metamodelica::Ref<DAE::ElementSource>;
    let mut range: metamodelica::Ref<DAE::Exp>;
    c = Types::getPropConst(inRangeProps.clone())?;
    if Types::isParameterOrConstant(c) {
        if '__try0: {
            let __pa1 = ::match_deref::match_deref! { match &(unwrap_break_err!(Ceval::ceval(outCache.clone(), inEnv.clone(), inRange.clone(), inImpl, Absyn::Msg::MSG { info: inInfo.clone() }, 0), '__try0)) {
                (__pa1, Deref @ Values::Value::ARRAY { valueLst: Deref @ metamodelica::ListNode::Nil, .. }) => __pa1.clone(),
                _ => break '__try0 Err::<_, _>("pattern mismatch"),
            } };
            outCache = metamodelica::Own::own(__pa1);
            outStatements = metamodelica::nil();
            return Ok((outCache, outStatements));
            Ok::<(), &'static str>(())
        }.is_err() {
        }
    }
    ty = Types::getPropType(&inRangeProps);
    ty = getIteratorType(&ty, &inIterator, &inInfo)?;
    (outCache, range) = Ceval::cevalRangeIfConstant(
        outCache,
        inEnv.clone(),
        inRange,
        inRangeProps.clone(),
        inImpl,
        inInfo.clone(),
    );
    (outCache, range) = PrefixUtil::prefixExp(outCache, &inEnv, &inIH, range, &inPrefix)?;
    env = addForLoopScope(
        inEnv,
        inIterator.clone(),
        ty,
        openmodelica_frontend_types::SCode::Variability::VAR,
        Some(c),
    )?;
    (outCache, outStatements) = instStatements(
        outCache,
        env,
        inIH,
        inPrefix,
        inState,
        inBody,
        inSource.clone(),
        inInitial,
        inImpl,
        inUnrollLoops,
    )?;
    source = ElementSource::addElementSourceFileInfo(inSource, inInfo);
    outStatements = list![Algorithm::makeFor(
        inIterator,
        range,
        &inRangeProps,
        outStatements,
        source
    )?];
    Ok((outCache, outStatements))
}

fn instComplexEquation(
    mut lhs: metamodelica::Ref<DAE::Exp>,
    mut rhs: metamodelica::Ref<DAE::Exp>,
    mut tp: &metamodelica::Ref<DAE::Type>,
    mut source: metamodelica::Ref<DAE::ElementSource>,
    mut initial_: SCode::Initial,
) -> Result<DAE::DAElist> {
    let mut dae: DAE::DAElist = <DAE::DAElist as ::std::default::Default>::default();
    dae = 'mc: {
        let __mc_input = initial_;
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            let mut dae: DAE::DAElist = dae.clone();
            let true = (Types::isRecord(tp)) else {
                return Err("pattern mismatch");
            };
            dae = makeComplexDaeEquation(lhs.clone(), rhs.clone(), source.clone(), initial_);
            Ok((dae.clone(), dae.clone()))
        })() {
            dae = __wb0;
            break 'mc __v;
        }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            let mut dae: DAE::DAElist = dae.clone();
            let true = (Types::isExternalObject(tp)) else {
                return Err("pattern mismatch");
            };
            dae = makeDaeEquation(lhs.clone(), rhs.clone(), source.clone(), initial_)?;
            Ok((dae.clone(), dae.clone()))
        })() {
            dae = __wb0;
            break 'mc __v;
        }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            let mut dae: DAE::DAElist = dae.clone();
            dae = makeComplexDaeEquation(lhs.clone(), rhs.clone(), source.clone(), initial_);
            Ok((dae.clone(), dae.clone()))
        })() {
            dae = __wb0;
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            let mut s: ArcStr;
            let mut info: SourceInfo;
            let false = (Types::isRecord(tp)) else {
                return Err("pattern mismatch");
            };
            s = {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*ExpressionBasics::printExpStr(lhs.clone())?);
                __mm_s.push_str(&*literal!(" = "));
                __mm_s.push_str(&*ExpressionBasics::printExpStr(rhs.clone())?);
                ArcStr::from(__mm_s)
            };
            info = ElementSource::getElementSourceFileInfo(source.clone());
            Error::addSourceMessage(&(Error::ILLEGAL_EQUATION_TYPE.clone()), list![s.clone()], &info)?;
            Ok(return Err("fail"))
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(dae)
}

fn makeComplexDaeEquation(
    mut lhs: metamodelica::Ref<DAE::Exp>,
    mut rhs: metamodelica::Ref<DAE::Exp>,
    mut source: metamodelica::Ref<DAE::ElementSource>,
    mut initial_: SCode::Initial,
) -> DAE::DAElist {
    let mut dae: DAE::DAElist;
    dae = (match initial_ {
        SCode::Initial::NON_INITIAL { .. } => DAE::DAElist {
            elementLst: list![metamodelica::Ref::new(DAE::Element::COMPLEX_EQUATION {
                lhs: lhs,
                rhs: rhs,
                source: source
            })],
        },
        SCode::Initial::INITIAL { .. } => DAE::DAElist {
            elementLst: list![metamodelica::Ref::new(DAE::Element::INITIAL_COMPLEX_EQUATION {
                lhs: lhs,
                rhs: rhs,
                source: source
            })],
        },
    });
    dae
}

pub(crate) fn instAlgorithm(
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
    mut inIH: metamodelica::List<InnerOuter::TopInstance>,
    mut inPrefix: DAE::Prefix,
    mut inSets: DAE::Connect::Sets,
    mut inState: ClassInf::State,
    mut inAlgorithm: &metamodelica::Ref<SCode::AlgorithmSection>,
    mut inImpl: bool,
    mut unrollForLoops: bool,
    mut inGraph: ConnectionGraph::ConnectionGraph,
) -> Result<(
    FCore::Cache,
    FCore::Graph,
    metamodelica::List<InnerOuter::TopInstance>,
    DAE::DAElist,
    DAE::Connect::Sets,
    ClassInf::State,
    ConnectionGraph::ConnectionGraph,
)> {
    let mut outCache: FCore::Cache;
    let mut outEnv: FCore::Graph;
    let mut outIH: metamodelica::List<InnerOuter::TopInstance>;
    let mut outDae: DAE::DAElist;
    let mut outSets: DAE::Connect::Sets;
    let mut outState: ClassInf::State;
    let mut outGraph: ConnectionGraph::ConnectionGraph;
    (outCache, outEnv, outIH, outDae, outSets, outState, outGraph) = 'mc: {
        let __mc_input = (
            inCache,
            inEnv,
            inIH,
            inPrefix,
            inSets,
            inState,
            &**inAlgorithm,
            inImpl,
            inGraph,
        );
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, ih, pre, csets, ci_state, Deref @ SCode::AlgorithmSection { statements }, r#impl, graph) => {
                    let mut statements_1: metamodelica::List<metamodelica::Ref<DAE::Statement>>;
                    let mut source: metamodelica::Ref<DAE::ElementSource>;
                    let mut dae: DAE::DAElist;
                    let mut cache = (*cache).clone();
                    let mut ci_state = (*ci_state).clone();
                    ci_state = ClassInfUtil::trans(ci_state.clone(), openmodelica_frontend_types::ClassInf::Event::FOUND_ALGORITHM)?;
                    source = ElementSource::createElementSource(Absyn::dummyInfo.clone(), FGraph::getScopePath(metamodelica::AsArg::as_arg(&env))?, metamodelica::AsArg::as_arg(&pre), (DAE::emptyCref().clone(), DAE::emptyCref().clone()));
                    (cache, statements_1) = instStatements(cache.clone(), env.clone(), ih.clone(), pre.clone(), metamodelica::AsArg::as_arg(&ci_state), metamodelica::AsArg::as_arg(&statements), source.clone(), openmodelica_frontend_types::SCode::Initial::NON_INITIAL, r#impl.clone(), unrollForLoops)?;
                    (statements_1, _) = DAEUtil::traverseDAEEquationsStmts(statements_1.clone(), (std::sync::Arc::new(Expression::traverseSubexpressionsHelper) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>, _) -> Result<_> + 'static>), ((std::sync::Arc::new(ExpressionSimplify::simplifyWork) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>, ExpressionSimplifyTypes::Evaluate) -> Result<(metamodelica::Ref<DAE::Exp>, ExpressionSimplifyTypes::Evaluate)> + 'static>), ExpressionSimplifyTypes::optionSimplifyOnly.clone()))?;
                    dae = DAE::DAElist { elementLst: list![metamodelica::Ref::new(DAE::Element::ALGORITHM { algorithm_: metamodelica::Ref::new(DAE::Algorithm { statementLst: statements_1.clone() }), source: source.clone() })] };
                    Ok((cache.clone(), env.clone(), ih.clone(), dae.clone(), csets.clone(), ci_state.clone(), graph.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _, _, _, ci_state, Deref @ SCode::AlgorithmSection { statements: Deref @ metamodelica::ListNode::Cons { head: stmt, tail: _ } }, _, _) => {
                    let mut s: ArcStr;
                    let mut info: SourceInfo;
                    if '__try0: {
                        unwrap_break_err!(ClassInfUtil::trans(ci_state.clone(), openmodelica_frontend_types::ClassInf::Event::FOUND_ALGORITHM), '__try0);
                        Ok::<(), &'static str>(())
                    }.is_ok() { return Err("failure(): body succeeded") }
                    s = ClassInfUtil::printStateStr(metamodelica::AsArg::as_arg(&ci_state));
                    info = SCodeUtil::getStatementInfo(metamodelica::AsArg::as_arg(&stmt))?;
                    Error::addSourceMessage(&(Error::ALGORITHM_TRANSITION_FAILURE.clone()), list![s.clone()], &info)?;
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
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { return Err("pattern mismatch") };
                    Debug::traceln(literal!("- InstSection.instAlgorithm failed"))?;
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((outCache, outEnv, outIH, outDae, outSets, outState, outGraph))
}

pub(crate) fn instInitialAlgorithm(
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
    mut inIH: metamodelica::List<InnerOuter::TopInstance>,
    mut inPrefix: DAE::Prefix,
    mut inSets: DAE::Connect::Sets,
    mut inState: ClassInf::State,
    mut inAlgorithm: &metamodelica::Ref<SCode::AlgorithmSection>,
    mut inImpl: bool,
    mut unrollForLoops: bool,
    mut inGraph: ConnectionGraph::ConnectionGraph,
) -> Result<(
    FCore::Cache,
    FCore::Graph,
    metamodelica::List<InnerOuter::TopInstance>,
    DAE::DAElist,
    DAE::Connect::Sets,
    ClassInf::State,
    ConnectionGraph::ConnectionGraph,
)> {
    let mut outCache: FCore::Cache;
    let mut outEnv: FCore::Graph;
    let mut outIH: metamodelica::List<InnerOuter::TopInstance>;
    let mut outDae: DAE::DAElist;
    let mut outSets: DAE::Connect::Sets;
    let mut outState: ClassInf::State;
    let mut outGraph: ConnectionGraph::ConnectionGraph;
    (outCache, outEnv, outIH, outDae, outSets, outState, outGraph) = 'mc: {
        let __mc_input = (
            inCache,
            inEnv,
            inIH,
            inPrefix,
            inSets,
            inState,
            &**inAlgorithm,
            inImpl,
            inGraph,
        );
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, ih, pre, csets, ci_state, Deref @ SCode::AlgorithmSection { statements }, r#impl, graph) => {
                    let mut statements_1: metamodelica::List<metamodelica::Ref<DAE::Statement>>;
                    let mut source: metamodelica::Ref<DAE::ElementSource>;
                    let mut dae: DAE::DAElist;
                    let mut cache = (*cache).clone();
                    source = ElementSource::createElementSource(Absyn::dummyInfo.clone(), FGraph::getScopePath(metamodelica::AsArg::as_arg(&env))?, metamodelica::AsArg::as_arg(&pre), (DAE::emptyCref().clone(), DAE::emptyCref().clone()));
                    (cache, statements_1) = instStatements(cache.clone(), env.clone(), ih.clone(), pre.clone(), metamodelica::AsArg::as_arg(&ci_state), metamodelica::AsArg::as_arg(&statements), source.clone(), openmodelica_frontend_types::SCode::Initial::INITIAL, r#impl.clone(), unrollForLoops)?;
                    (statements_1, _) = DAEUtil::traverseDAEEquationsStmts(statements_1.clone(), (std::sync::Arc::new(Expression::traverseSubexpressionsHelper) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>, _) -> Result<_> + 'static>), ((std::sync::Arc::new(ExpressionSimplify::simplifyWork) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>, ExpressionSimplifyTypes::Evaluate) -> Result<(metamodelica::Ref<DAE::Exp>, ExpressionSimplifyTypes::Evaluate)> + 'static>), ExpressionSimplifyTypes::optionSimplifyOnly.clone()))?;
                    dae = DAE::DAElist { elementLst: list![metamodelica::Ref::new(DAE::Element::INITIALALGORITHM { algorithm_: metamodelica::Ref::new(DAE::Algorithm { statementLst: statements_1.clone() }), source: source.clone() })] };
                    Ok((cache.clone(), env.clone(), ih.clone(), dae.clone(), csets.clone(), ci_state.clone(), graph.clone()))
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
                    Debug::trace(literal!("- InstSection.instInitialAlgorithm failed\n"))?;
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((outCache, outEnv, outIH, outDae, outSets, outState, outGraph))
}

pub(crate) fn instConstraint(
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
    mut inPrefix: DAE::Prefix,
    mut inState: ClassInf::State,
    mut inConstraints: &SCode::ConstraintSection,
    mut inImpl: bool,
) -> Result<(FCore::Cache, FCore::Graph, DAE::DAElist, ClassInf::State)> {
    let mut outCache: FCore::Cache;
    let mut outEnv: FCore::Graph;
    let mut outDae: DAE::DAElist;
    let mut outState: ClassInf::State;
    (outCache, outEnv, outDae, outState) = 'mc: {
        let __mc_input = (inCache, inEnv, inPrefix, inState, inConstraints.clone(), inImpl);
        if let Ok(__v) = (|| -> Result<_> {
            let (
                mut cache,
                mut env,
                mut pre,
                mut ci_state,
                SCode::ConstraintSection {
                    constraints: mut constraints,
                },
                mut r#impl,
            ) = __mc_input.clone()
            else {
                return Err("nomatch");
            };
            let mut constraints_1: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
            let mut source: metamodelica::Ref<DAE::ElementSource>;
            let mut dae: DAE::DAElist;
            ci_state = ClassInfUtil::trans(
                ci_state.clone(),
                openmodelica_frontend_types::ClassInf::Event::FOUND_ALGORITHM,
            )?;
            source = ElementSource::createElementSource(
                Absyn::dummyInfo.clone(),
                FGraph::getScopePath(&(env.clone()))?,
                &(pre.clone()),
                (DAE::emptyCref().clone(), DAE::emptyCref().clone()),
            );
            (cache, constraints_1, _) = Static::elabExpList(
                cache.clone(),
                env.clone(),
                &(constraints.clone()),
                r#impl.clone(),
                true,
                pre.clone(),
                Absyn::dummyInfo.clone(),
                DAE::T_UNKNOWN_DEFAULT().clone(),
            )?;
            dae = DAE::DAElist {
                elementLst: list![metamodelica::Ref::new(DAE::Element::CONSTRAINT {
                    constraints: metamodelica::Ref::new(DAE::Constraint::CONSTRAINT_EXPS {
                        constraintLst: constraints_1.clone()
                    }),
                    source: source.clone()
                })],
            };
            Ok((cache.clone(), env.clone(), dae.clone(), ci_state.clone()))
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else {
                return Err("pattern mismatch");
            };
            Debug::trace(literal!("- InstSection.instConstraints failed\n"))?;
            Ok(return Err("fail"))
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((outCache, outEnv, outDae, outState))
}

pub(crate) fn instStatements(
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
    mut inIH: metamodelica::List<InnerOuter::TopInstance>,
    mut inPrefix: DAE::Prefix,
    mut inState: &ClassInf::State,
    mut inStatements: &metamodelica::List<metamodelica::Ref<SCode::Statement>>,
    mut inSource: metamodelica::Ref<DAE::ElementSource>,
    mut inInitial: SCode::Initial,
    mut inImpl: bool,
    mut unrollForLoops: bool,
) -> Result<(FCore::Cache, metamodelica::List<metamodelica::Ref<DAE::Statement>>)> {
    let mut outCache: FCore::Cache = inCache.clone();
    let mut outStatements: metamodelica::List<metamodelica::Ref<DAE::Statement>>;
    let mut stmts: metamodelica::List<metamodelica::Ref<DAE::Statement>>;
    let mut stmtsl: metamodelica::List<metamodelica::List<metamodelica::Ref<DAE::Statement>>> = metamodelica::nil();
    for mut stmt in &**inStatements {
        (outCache, stmts) = instStatement(
            inCache.clone(),
            inEnv.clone(),
            inIH.clone(),
            inPrefix.clone(),
            inState,
            stmt.clone(),
            inSource.clone(),
            inInitial,
            inImpl,
            unrollForLoops,
        )?;
        stmtsl = metamodelica::cons(stmts, stmtsl);
    }
    outStatements = List::flattenReverse(stmtsl)?;
    Ok((outCache, outStatements))
}

fn instExp(
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
    mut inIH: &metamodelica::List<InnerOuter::TopInstance>,
    mut inPrefix: DAE::Prefix,
    mut inExp: metamodelica::Ref<Absyn::Exp>,
    mut inImpl: bool,
    mut inInfo: SourceInfo,
) -> Result<(FCore::Cache, metamodelica::Ref<DAE::Exp>, DAE::Properties)> {
    let mut outCache: FCore::Cache;
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    let mut outProperties: DAE::Properties;
    (outCache, outExp, outProperties) = Static::elabExp(
        inCache,
        inEnv.clone(),
        inExp,
        inImpl,
        true,
        inPrefix.clone(),
        inInfo.clone(),
    )?;
    (outCache, outExp, outProperties) =
        Ceval::cevalIfConstant(outCache, inEnv.clone(), outExp, outProperties, inImpl, inInfo)?;
    (outCache, outExp) = PrefixUtil::prefixExp(outCache, &inEnv, inIH, outExp, &inPrefix)?;
    Ok((outCache, outExp, outProperties))
}

fn instStatement(
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
    mut inIH: metamodelica::List<InnerOuter::TopInstance>,
    mut inPrefix: DAE::Prefix,
    mut inState: &ClassInf::State,
    mut inStatement: metamodelica::Ref<SCode::Statement>,
    mut inSource: metamodelica::Ref<DAE::ElementSource>,
    mut inInitial: SCode::Initial,
    mut inImpl: bool,
    mut inUnrollLoops: bool,
) -> Result<(FCore::Cache, metamodelica::List<metamodelica::Ref<DAE::Statement>>)> {
    let mut outCache: FCore::Cache = inCache;
    let mut outStatements: metamodelica::List<metamodelica::Ref<DAE::Statement>>;
    let mut num_errors: i32 = Error::getNumErrorMessages();
    match '__try0: {
        outStatements = (match &*inStatement {
            SCode::Statement::ALG_ASSIGN { .. } => {
                (outCache, outStatements) = unwrap_break_err!(instAssignment(outCache.clone(), inEnv.clone(), &inIH, inPrefix.clone(), &inStatement, inSource.clone(), inInitial, inImpl, inUnrollLoops, num_errors), '__try0);
                outStatements.clone()
            }
            SCode::Statement::ALG_IF {
                info,
                boolExpr: __inStatement_boolExpr,
                elseBranch: __inStatement_elseBranch,
                elseIfBranch: __inStatement_elseIfBranch,
                trueBranch: __inStatement_trueBranch,
                ..
            } => {
                let mut cond_exp: metamodelica::Ref<DAE::Exp>;
                let mut exp: metamodelica::Ref<DAE::Exp>;
                let mut cond_prop: DAE::Properties;
                let mut prop: DAE::Properties;
                let mut if_branch: metamodelica::List<metamodelica::Ref<DAE::Statement>>;
                let mut else_branch: metamodelica::List<metamodelica::Ref<DAE::Statement>>;
                let mut branch: metamodelica::List<metamodelica::Ref<DAE::Statement>>;
                let mut else_if_branches: metamodelica::List<(
                    metamodelica::Ref<DAE::Exp>,
                    DAE::Properties,
                    metamodelica::List<metamodelica::Ref<DAE::Statement>>,
                )>;
                let mut aexp: metamodelica::Ref<Absyn::Exp>;
                let mut sstmts: metamodelica::List<metamodelica::Ref<SCode::Statement>>;
                let mut source: metamodelica::Ref<DAE::ElementSource>;
                (outCache, cond_exp, cond_prop) = unwrap_break_err!(instExp(outCache.clone(), inEnv.clone(), &inIH, inPrefix.clone(), __inStatement_boolExpr.clone(), inImpl, info.clone()), '__try0);
                (outCache, if_branch) = unwrap_break_err!(instStatements(outCache.clone(), inEnv.clone(), inIH.clone(), inPrefix.clone(), inState, metamodelica::AsArg::as_arg(&__inStatement_trueBranch), inSource.clone(), inInitial, inImpl, inUnrollLoops), '__try0);
                else_if_branches = metamodelica::nil();
                for mut else_if in &*__inStatement_elseIfBranch.clone() {
                    (aexp, sstmts) = else_if.clone();
                    (outCache, exp, prop) = unwrap_break_err!(instExp(outCache.clone(), inEnv.clone(), &inIH, inPrefix.clone(), aexp.clone(), inImpl, info.clone()), '__try0);
                    (outCache, branch) = unwrap_break_err!(instStatements(outCache.clone(), inEnv.clone(), inIH.clone(), inPrefix.clone(), inState, &sstmts, inSource.clone(), inInitial, inImpl, inUnrollLoops), '__try0);
                    else_if_branches =
                        metamodelica::cons((exp.clone(), prop.clone(), branch.clone()), else_if_branches.clone());
                }
                else_if_branches = else_if_branches.clone().reverse();
                (outCache, else_branch) = unwrap_break_err!(instStatements(outCache.clone(), inEnv.clone(), inIH.clone(), inPrefix.clone(), inState, metamodelica::AsArg::as_arg(&__inStatement_elseBranch), inSource.clone(), inInitial, inImpl, inUnrollLoops), '__try0);
                source = ElementSource::addElementSourceFileInfo(inSource.clone(), info.clone());
                unwrap_break_err!(Algorithm::makeIf(cond_exp.clone(), &cond_prop, if_branch.clone(), else_if_branches.clone(), else_branch.clone(), &source), '__try0)
            }
            SCode::Statement::ALG_FOR { .. } => {
                (outCache, outStatements) = unwrap_break_err!(instForStatement(outCache.clone(), inEnv.clone(), inIH.clone(), inPrefix.clone(), inState, inStatement.clone(), inSource.clone(), inInitial, inImpl, inUnrollLoops), '__try0);
                outStatements.clone()
            }
            SCode::Statement::ALG_PARFOR { .. } => {
                (outCache, outStatements) = unwrap_break_err!(instParForStatement(outCache.clone(), inEnv.clone(), inIH.clone(), inPrefix.clone(), inState, inStatement.clone(), inSource.clone(), inInitial, inImpl, inUnrollLoops), '__try0);
                outStatements.clone()
            }
            SCode::Statement::ALG_WHILE {
                info,
                boolExpr: __inStatement_boolExpr,
                whileBody: __inStatement_whileBody,
                ..
            } => {
                let mut cond_exp: metamodelica::Ref<DAE::Exp>;
                let mut cond_prop: DAE::Properties;
                let mut branch: metamodelica::List<metamodelica::Ref<DAE::Statement>>;
                let mut source: metamodelica::Ref<DAE::ElementSource>;
                (outCache, cond_exp, cond_prop) = unwrap_break_err!(instExp(outCache.clone(), inEnv.clone(), &inIH, inPrefix.clone(), __inStatement_boolExpr.clone(), inImpl, info.clone()), '__try0);
                (outCache, branch) = unwrap_break_err!(instStatements(outCache.clone(), inEnv.clone(), inIH.clone(), inPrefix.clone(), inState, metamodelica::AsArg::as_arg(&__inStatement_whileBody), inSource.clone(), inInitial, inImpl, inUnrollLoops), '__try0);
                source = ElementSource::addElementSourceFileInfo(inSource.clone(), info.clone());
                list![
                    unwrap_break_err!(Algorithm::makeWhile(cond_exp.clone(), &cond_prop, branch.clone(), source.clone()), '__try0)
                ]
            }
            SCode::Statement::ALG_WHEN_A {
                info,
                branches: __inStatement_branches,
                ..
            } => {
                let mut cond_exp: metamodelica::Ref<DAE::Exp>;
                let mut cond_prop: DAE::Properties;
                let mut branch: metamodelica::List<metamodelica::Ref<DAE::Statement>>;
                let mut aexp: metamodelica::Ref<Absyn::Exp>;
                let mut sstmts: metamodelica::List<metamodelica::Ref<SCode::Statement>>;
                let mut source: metamodelica::Ref<DAE::ElementSource>;
                let mut when_stmt_opt: Option<metamodelica::Ref<DAE::Statement>>;
                let mut when_stmt: metamodelica::Ref<DAE::Statement>;
                if ClassInfUtil::isFunction(inState) {
                    unwrap_break_err!(Error::addSourceMessageAndFail(&(Error::FUNCTION_ELEMENT_WRONG_KIND.clone()), list![literal!("when")], metamodelica::AsArg::as_arg(&info)), '__try0);
                    unreachable!("Error.addSourceMessageAndFail always fails — caller-side flow-analysis hint");
                }
                unwrap_break_err!(checkWhenAlgorithm(&inStatement), '__try0);
                source = ElementSource::addElementSourceFileInfo(inSource.clone(), info.clone());
                when_stmt_opt = None;
                for mut b in &*__inStatement_branches.clone().reverse() {
                    (aexp, sstmts) = b.clone();
                    (outCache, cond_exp, cond_prop) = unwrap_break_err!(instExp(outCache.clone(), inEnv.clone(), &inIH, inPrefix.clone(), aexp.clone(), inImpl, info.clone()), '__try0);
                    (outCache, branch) = unwrap_break_err!(instStatements(outCache.clone(), inEnv.clone(), inIH.clone(), inPrefix.clone(), inState, &sstmts, inSource.clone(), inInitial, inImpl, inUnrollLoops), '__try0);
                    when_stmt_opt = Some(
                        unwrap_break_err!(Algorithm::makeWhenA(cond_exp.clone(), &cond_prop, branch.clone(), when_stmt_opt.clone(), source.clone()), '__try0),
                    );
                }
                let __pa0 = ::match_deref::match_deref! { match &(when_stmt_opt.clone()) {
                    Some(__pa0) => __pa0.clone(),
                    _ => break '__try0 Err::<_, _>("pattern mismatch"),
                } };
                when_stmt = metamodelica::Own::own(__pa0);
                list![when_stmt.clone()]
            }
            SCode::Statement::ALG_ASSERT {
                info,
                condition: __inStatement_condition,
                level: __inStatement_level,
                message: __inStatement_message,
                ..
            } => {
                let mut cond_exp: metamodelica::Ref<DAE::Exp>;
                let mut msg_exp: metamodelica::Ref<DAE::Exp>;
                let mut level_exp: metamodelica::Ref<DAE::Exp>;
                let mut cond_prop: DAE::Properties;
                let mut msg_prop: DAE::Properties;
                let mut level_prop: DAE::Properties;
                let mut source: metamodelica::Ref<DAE::ElementSource>;
                (outCache, cond_exp, cond_prop) = unwrap_break_err!(instExp(outCache.clone(), inEnv.clone(), &inIH, inPrefix.clone(), __inStatement_condition.clone(), inImpl, info.clone()), '__try0);
                (outCache, msg_exp, msg_prop) = unwrap_break_err!(instExp(outCache.clone(), inEnv.clone(), &inIH, inPrefix.clone(), __inStatement_message.clone(), inImpl, info.clone()), '__try0);
                (outCache, level_exp, level_prop) = unwrap_break_err!(instExp(outCache.clone(), inEnv.clone(), &inIH, inPrefix.clone(), __inStatement_level.clone(), inImpl, info.clone()), '__try0);
                source = ElementSource::addElementSourceFileInfo(inSource.clone(), info.clone());
                unwrap_break_err!(Algorithm::makeAssert(cond_exp.clone(), msg_exp.clone(), level_exp.clone(), &cond_prop, &msg_prop, &level_prop, source.clone()), '__try0)
            }
            SCode::Statement::ALG_TERMINATE {
                info,
                message: __inStatement_message,
                ..
            } => {
                let mut msg_exp: metamodelica::Ref<DAE::Exp>;
                let mut msg_prop: DAE::Properties;
                let mut source: metamodelica::Ref<DAE::ElementSource>;
                (outCache, msg_exp, msg_prop) = unwrap_break_err!(instExp(outCache.clone(), inEnv.clone(), &inIH, inPrefix.clone(), __inStatement_message.clone(), inImpl, info.clone()), '__try0);
                source = ElementSource::addElementSourceFileInfo(inSource.clone(), info.clone());
                unwrap_break_err!(Algorithm::makeTerminate(msg_exp.clone(), &msg_prop, source.clone()), '__try0)
            }
            SCode::Statement::ALG_REINIT {
                info,
                cref: __inStatement_cref,
                newValue: __inStatement_newValue,
                ..
            } => {
                let mut exp: metamodelica::Ref<DAE::Exp>;
                let mut cr_exp: metamodelica::Ref<DAE::Exp>;
                let mut prop: DAE::Properties;
                let mut cr_prop: DAE::Properties;
                let mut source: metamodelica::Ref<DAE::ElementSource>;
                (outCache, cr_exp, cr_prop) = unwrap_break_err!(instExp(outCache.clone(), inEnv.clone(), &inIH, inPrefix.clone(), __inStatement_cref.clone(), inImpl, info.clone()), '__try0);
                (outCache, exp, prop) = unwrap_break_err!(instExp(outCache.clone(), inEnv.clone(), &inIH, inPrefix.clone(), __inStatement_newValue.clone(), inImpl, info.clone()), '__try0);
                source = ElementSource::addElementSourceFileInfo(inSource.clone(), info.clone());
                unwrap_break_err!(Algorithm::makeReinit(cr_exp.clone(), exp.clone(), &cr_prop, &prop, source.clone()), '__try0)
            }
            SCode::Statement::ALG_NORETCALL {
                info,
                exp: __inStatement_exp,
                ..
            } => {
                let mut exp: metamodelica::Ref<DAE::Exp>;
                let mut source: metamodelica::Ref<DAE::ElementSource>;
                (outCache, exp, _) = unwrap_break_err!(Static::elabExp(outCache.clone(), inEnv.clone(), __inStatement_exp.clone(), inImpl, true, inPrefix.clone(), info.clone()), '__try0);
                unwrap_break_err!(checkValidNoRetcall(exp.clone(), metamodelica::AsArg::as_arg(&info)), '__try0);
                (outCache, exp) = unwrap_break_err!(PrefixUtil::prefixExp(outCache.clone(), &inEnv, &inIH, exp.clone(), &inPrefix), '__try0);
                source = ElementSource::addElementSourceFileInfo(inSource.clone(), info.clone());
                if (Expression::isTuple(&exp)) {
                    metamodelica::nil()
                } else {
                    list![metamodelica::Ref::new(DAE::Statement::STMT_NORETCALL {
                        exp: exp.clone(),
                        source: source.clone()
                    })]
                }
            }
            SCode::Statement::ALG_BREAK { info, .. } => {
                let mut source: metamodelica::Ref<DAE::ElementSource>;
                source = ElementSource::addElementSourceFileInfo(inSource.clone(), info.clone());
                list![metamodelica::Ref::new(DAE::Statement::STMT_BREAK {
                    source: source.clone()
                })]
            }
            SCode::Statement::ALG_CONTINUE { info, .. } => {
                let mut source: metamodelica::Ref<DAE::ElementSource>;
                source = ElementSource::addElementSourceFileInfo(inSource.clone(), info.clone());
                list![metamodelica::Ref::new(DAE::Statement::STMT_CONTINUE {
                    source: source.clone()
                })]
            }
            SCode::Statement::ALG_RETURN { info, .. } => {
                let mut source: metamodelica::Ref<DAE::ElementSource>;
                if !(ClassInfUtil::isFunction(inState)) {
                    unwrap_break_err!(Error::addSourceMessageAndFail(&(Error::RETURN_OUTSIDE_FUNCTION.clone()), metamodelica::nil(), metamodelica::AsArg::as_arg(&info)), '__try0);
                    unreachable!("Error.addSourceMessageAndFail always fails — caller-side flow-analysis hint");
                }
                source = ElementSource::addElementSourceFileInfo(inSource.clone(), info.clone());
                list![metamodelica::Ref::new(DAE::Statement::STMT_RETURN {
                    source: source.clone()
                })]
            }
            SCode::Statement::ALG_FAILURE {
                info,
                stmts: __inStatement_stmts,
                ..
            } => {
                let mut branch: metamodelica::List<metamodelica::Ref<DAE::Statement>>;
                let mut source: metamodelica::Ref<DAE::ElementSource>;
                let true = (unwrap_break_err!(Config::acceptMetaModelicaGrammar(), '__try0)) else {
                    break '__try0 Err::<_, _>("pattern mismatch");
                };
                (outCache, branch) = unwrap_break_err!(instStatements(outCache.clone(), inEnv.clone(), inIH.clone(), inPrefix.clone(), inState, metamodelica::AsArg::as_arg(&__inStatement_stmts), inSource.clone(), inInitial, inImpl, inUnrollLoops), '__try0);
                source = ElementSource::addElementSourceFileInfo(inSource.clone(), info.clone());
                list![metamodelica::Ref::new(DAE::Statement::STMT_FAILURE {
                    body: branch.clone(),
                    source: source.clone()
                })]
            }
            SCode::Statement::ALG_TRY {
                info,
                body: __inStatement_body,
                comment: __inStatement_comment,
                elseBody: __inStatement_elseBody,
            } => {
                let mut exp: metamodelica::Ref<DAE::Exp>;
                let mut if_branch: metamodelica::List<metamodelica::Ref<DAE::Statement>>;
                let mut else_branch: metamodelica::List<metamodelica::Ref<DAE::Statement>>;
                let mut source: metamodelica::Ref<DAE::ElementSource>;
                let mut cases: metamodelica::List<metamodelica::Ref<DAE::MatchCase>>;
                let true = (unwrap_break_err!(Config::acceptMetaModelicaGrammar(), '__try0)) else {
                    break '__try0 Err::<_, _>("pattern mismatch");
                };
                (outCache, if_branch) = unwrap_break_err!(instStatements(outCache.clone(), inEnv.clone(), inIH.clone(), inPrefix.clone(), inState, metamodelica::AsArg::as_arg(&__inStatement_body), inSource.clone(), inInitial, inImpl, inUnrollLoops), '__try0);
                (outCache, else_branch) = unwrap_break_err!(instStatements(outCache.clone(), inEnv.clone(), inIH.clone(), inPrefix.clone(), inState, metamodelica::AsArg::as_arg(&__inStatement_elseBody), inSource.clone(), inInitial, inImpl, inUnrollLoops), '__try0);
                source = ElementSource::addElementSourceFileInfo(inSource.clone(), info.clone());
                cases = list![
                    metamodelica::Ref::new(DAE::MatchCase {
                        patterns: metamodelica::nil(),
                        patternGuard: None,
                        localDecls: metamodelica::nil(),
                        body: if_branch.clone(),
                        result: Some(metamodelica::Ref::new(DAE::Exp::TUPLE {
                            PR: metamodelica::nil()
                        })),
                        resultInfo: info.clone(),
                        jump: 0,
                        info: info.clone()
                    }),
                    metamodelica::Ref::new(DAE::MatchCase {
                        patterns: metamodelica::nil(),
                        patternGuard: None,
                        localDecls: metamodelica::nil(),
                        body: else_branch.clone(),
                        result: Some(metamodelica::Ref::new(DAE::Exp::TUPLE {
                            PR: metamodelica::nil()
                        })),
                        resultInfo: info.clone(),
                        jump: 0,
                        info: info.clone()
                    })
                ];
                exp = metamodelica::Ref::new(DAE::Exp::MATCHEXPRESSION {
                    matchType: if (SCodeUtil::commentHasBooleanNamedAnnotation(
                        metamodelica::AsArg::as_arg(&__inStatement_comment),
                        &(literal!("__OpenModelica_stackOverflowCheckpoint")),
                    )) {
                        openmodelica_frontend_types::DAE::MatchType::TRY_STACKOVERFLOW
                    } else {
                        openmodelica_frontend_types::DAE::MatchType::MATCHCONTINUE
                    },
                    inputs: metamodelica::nil(),
                    aliases: metamodelica::nil(),
                    localDecls: metamodelica::nil(),
                    cases: cases.clone(),
                    et: DAE::T_NORETCALL_DEFAULT().clone(),
                });
                list![metamodelica::Ref::new(DAE::Statement::STMT_NORETCALL {
                    exp: exp.clone(),
                    source: source.clone()
                })]
            }
        });
        Ok::<_, &'static str>((outStatements.clone(),))
    } {
        Ok((__try0_o0,)) => {
            outStatements = __try0_o0;
        }
        Err(_) => {
            let true = (num_errors == Error::getNumErrorMessages()) else {
                return Err("pattern mismatch");
            };
            Error::addSourceMessageAndFail(
                &(Error::STATEMENT_GENERIC_FAILURE.clone()),
                list![SCodeDump::statementStr(
                    inStatement.clone(),
                    SCodeDump::defaultOptions.clone()
                )?],
                &(SCodeUtil::getStatementInfo(&inStatement)?),
            )?;
            unreachable!("Error.addSourceMessageAndFail always fails — caller-side flow-analysis hint");
        }
    }
    Ok((outCache, outStatements))
}

fn makeAssignment(
    mut inLhs: metamodelica::Ref<DAE::Exp>,
    mut inLhsProps: DAE::Properties,
    mut inRhs: metamodelica::Ref<DAE::Exp>,
    mut inRhsProps: DAE::Properties,
    mut inAttributes: &metamodelica::Ref<DAE::Attributes>,
    mut inInitial: SCode::Initial,
    mut inSource: metamodelica::Ref<DAE::ElementSource>,
) -> Result<metamodelica::Ref<DAE::Statement>> {
    let mut outStatement: metamodelica::Ref<DAE::Statement>;
    outStatement = (::match_deref::match_deref! { match &((inLhsProps.clone(), inRhs.clone(), inRhsProps.clone())) {
        (DAE::Properties::PROP { .. }, Deref @ DAE::Exp::CALL { .. }, DAE::Properties::PROP_TUPLE { .. }) => {
            let mut wild_props: metamodelica::List<DAE::Properties>;
            let mut wild_count: i32;
            let mut wilds: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
            let mut wildCrefExp: metamodelica::Ref<DAE::Exp>;
            let __pa0 = ::match_deref::match_deref! { match &(Types::propTuplePropList(&inRhsProps)?) {
                Deref @ metamodelica::ListNode::Cons { head: _, tail: __pa0 } => __pa0.clone(),
                _ => return Err("pattern mismatch"),
            } };
            wild_props = metamodelica::Own::own(__pa0);
            wild_count = ((wild_props).len() as i32);
            wildCrefExp = Expression::makeCrefExp(openmodelica_frontend_types::DAE::ComponentRef::interned_WILD(), DAE::T_UNKNOWN_DEFAULT().clone())?;
            wilds = List::fill(wildCrefExp, wild_count);
            wild_props = List::fill(DAE::Properties::PROP { type_: DAE::T_ANYTYPE_DEFAULT().clone(), constFlag: openmodelica_frontend_types::DAE::Const::C_VAR }, wild_count);
            Algorithm::makeTupleAssignment(metamodelica::cons(inLhs, wilds), metamodelica::cons(inLhsProps, wild_props), inRhs, inRhsProps, inInitial, inSource)?
        },
        _ => {
            Algorithm::makeAssignment(inLhs, inLhsProps, inRhs, inRhsProps, inAttributes, inInitial, inSource)?
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outStatement)
}

fn containsWhenStatements(mut statementList: &metamodelica::List<metamodelica::Ref<SCode::Statement>>) -> Result<bool> {
    let mut hasWhenStatements: bool;
    hasWhenStatements = 'mc: {
        let __mc_input = &**statementList;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Nil => {
                    Ok(false)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: Deref @ SCode::Statement::ALG_WHEN_A { .. }, tail: _ } => {
                    Ok(true)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: Deref @ SCode::Statement::ALG_IF { trueBranch: tb, elseIfBranch: eib, elseBranch: eb, .. }, tail: rest } => {
                    let mut b: bool;
                    let mut b1: bool;
                    let mut b2: bool;
                    let mut b3: bool;
                    let mut b4: bool;
                    let mut blst: metamodelica::List<bool>;
                    let mut slst: metamodelica::List<metamodelica::List<metamodelica::Ref<SCode::Statement>>>;
                    b1 = containsWhenStatements(metamodelica::AsArg::as_arg(&tb))?;
                    b2 = containsWhenStatements(metamodelica::AsArg::as_arg(&eb))?;
                    slst = List::map(eib.clone(), &fnptr!(Util::tuple22, _))?;
                    blst = List::map(slst.clone(), &move |__a0: metamodelica::List<metamodelica::Ref<SCode::Statement>>| containsWhenStatements(&__a0))?;
                    b3 = List::reduce(&(metamodelica::cons(false, blst.clone())), &fnptr!(boolOr, bool, bool))?;
                    b4 = containsWhenStatements(metamodelica::AsArg::as_arg(&rest))?;
                    b = List::reduce(&(list![b1, b2, b3, b4]), &fnptr!(boolOr, bool, bool))?;
                    Ok(b)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: Deref @ SCode::Statement::ALG_FOR { forBody: lst, .. }, tail: rest } => {
                    let mut b: bool;
                    let mut b1: bool;
                    let mut b2: bool;
                    b1 = containsWhenStatements(metamodelica::AsArg::as_arg(&lst))?;
                    b2 = containsWhenStatements(metamodelica::AsArg::as_arg(&rest))?;
                    b = boolOr(b1, b2);
                    Ok(b)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: Deref @ SCode::Statement::ALG_PARFOR { parforBody: lst, .. }, tail: rest } => {
                    let mut b: bool;
                    let mut b1: bool;
                    let mut b2: bool;
                    b1 = containsWhenStatements(metamodelica::AsArg::as_arg(&lst))?;
                    b2 = containsWhenStatements(metamodelica::AsArg::as_arg(&rest))?;
                    b = boolOr(b1, b2);
                    Ok(b)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: Deref @ SCode::Statement::ALG_WHILE { whileBody: lst, .. }, tail: rest } => {
                    let mut b: bool;
                    let mut b1: bool;
                    let mut b2: bool;
                    b1 = containsWhenStatements(metamodelica::AsArg::as_arg(&lst))?;
                    b2 = containsWhenStatements(metamodelica::AsArg::as_arg(&rest))?;
                    b = boolOr(b1, b2);
                    Ok(b)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: _, tail: rest } => {
                    Ok(containsWhenStatements(metamodelica::AsArg::as_arg(&rest))?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(hasWhenStatements)
}

fn loopOverRange(
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
    mut inIH: metamodelica::List<InnerOuter::TopInstance>,
    mut inPrefix: DAE::Prefix,
    mut ci_state: &ClassInf::State,
    mut inIdent: Ident,
    mut inValue: metamodelica::Ref<Values::Value>,
    mut inAlgItmLst: metamodelica::List<metamodelica::Ref<SCode::Statement>>,
    mut source: &metamodelica::Ref<DAE::ElementSource>,
    mut inInitial: SCode::Initial,
    mut inImpl: bool,
    mut unrollForLoops: bool,
) -> Result<(FCore::Cache, metamodelica::List<metamodelica::Ref<DAE::Statement>>)> {
    let mut outCache: FCore::Cache;
    let mut outStatements: metamodelica::List<metamodelica::Ref<DAE::Statement>>;
    (outCache, outStatements) = 'mc: {
        let __mc_input = (
            inCache,
            inEnv,
            inIH,
            inPrefix,
            inIdent,
            inValue,
            inAlgItmLst,
            inInitial,
            inImpl,
        );
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, _, _, _, _, Deref @ Values::Value::ARRAY { valueLst: Deref @ metamodelica::ListNode::Nil, .. }, _, _, _) => {
                    Ok((cache.clone(), metamodelica::nil()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, ih, pre, i, Deref @ Values::Value::ARRAY { valueLst: Deref @ metamodelica::ListNode::Cons { head: fst, tail: rest }, dimLst: Deref @ metamodelica::ListNode::Cons { head: dim, tail: dims } }, algs, initial_, r#impl) => {
                    let mut env_1: FCore::Graph;
                    let mut env_2: FCore::Graph;
                    let mut stmts: metamodelica::List<metamodelica::Ref<DAE::Statement>>;
                    let mut stmts1: metamodelica::List<metamodelica::Ref<DAE::Statement>>;
                    let mut stmts2: metamodelica::List<metamodelica::Ref<DAE::Statement>>;
                    let mut cache = (*cache).clone();
                    let mut dim = (*dim).clone();
                    let mut dims = (*dims).clone();
                    dim = dim.clone() - 1;
                    dims = metamodelica::cons(dim.clone(), dims.clone());
                    env_1 = FGraph::openScope(env.clone(), openmodelica_frontend_types::SCode::Encapsulated::NOT_ENCAPSULATED, arcstr::literal!(FCore::forScopeName), None)?;
                    env_2 = FGraph::addForIterator(env_1.clone(), i.clone(), DAE::T_INTEGER_DEFAULT().clone(), metamodelica::Ref::new(DAE::Binding::VALBOUND { valBound: fst.clone(), source: openmodelica_frontend_types::DAE::BindingSource::BINDING_FROM_DEFAULT_VALUE }), openmodelica_frontend_types::SCode::Variability::CONST, Some(openmodelica_frontend_types::DAE::Const::C_CONST))?;
                    (cache, stmts1) = instStatements(cache.clone(), env_2.clone(), ih.clone(), pre.clone(), ci_state, metamodelica::AsArg::as_arg(&algs), source.clone(), initial_.clone(), r#impl.clone(), unrollForLoops)?;
                    (cache, stmts2) = loopOverRange(cache.clone(), env.clone(), ih.clone(), pre.clone(), ci_state, i.clone(), metamodelica::Ref::new(Values::Value::ARRAY { valueLst: rest.clone(), dimLst: dims.clone() }), algs.clone(), source, initial_.clone(), r#impl.clone(), unrollForLoops)?;
                    stmts = listAppend(stmts1.clone(), stmts2.clone());
                    Ok((cache.clone(), stmts.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _, _, _, v, _, _, _) => {
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { return Err("pattern mismatch") };
                    Debug::traceln({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("- InstSection.loopOverRange failed to loop over range: ")); __mm_s.push_str(&*ValuesDump::valString(metamodelica::AsArg::as_arg(&v))?); ArcStr::from(__mm_s) })?;
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((outCache, outStatements))
}

fn rangeExpression(mut inTuple: &(metamodelica::Ref<Absyn::ComponentRef>, i32)) -> metamodelica::Ref<Absyn::Exp> {
    let mut outExp: metamodelica::Ref<Absyn::Exp>;
    outExp = (::match_deref::match_deref! { match &(inTuple) {
        (acref, dimNum) => {
            let mut e: metamodelica::Ref<Absyn::Exp>;
            e = metamodelica::Ref::new(Absyn::Exp::RANGE { start: metamodelica::Ref::new(Absyn::Exp::INTEGER { value: 1 }), step: None, stop: metamodelica::Ref::new(Absyn::Exp::CALL { function_: metamodelica::Ref::new(Absyn::ComponentRef::CREF_IDENT { name: literal!("size"), subscripts: metamodelica::nil() }), functionArgs: metamodelica::Ref::new(Absyn::FunctionArgs::FUNCTIONARGS { args: list![metamodelica::Ref::new(Absyn::Exp::CREF { componentRef: acref.clone() }), metamodelica::Ref::new(Absyn::Exp::INTEGER { value: dimNum.clone() })], argNames: metamodelica::nil() }), typeVars: metamodelica::nil() }) });
            e
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outExp
}

fn instIfEqBranch(
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
    mut inIH: metamodelica::List<InnerOuter::TopInstance>,
    mut inPrefix: DAE::Prefix,
    mut inState: ClassInf::State,
    mut inEquations: &metamodelica::List<metamodelica::Ref<SCode::Equation>>,
    mut inImpl: bool,
) -> Result<(
    FCore::Cache,
    FCore::Graph,
    metamodelica::List<InnerOuter::TopInstance>,
    ClassInf::State,
    metamodelica::List<metamodelica::Ref<DAE::Element>>,
)> {
    let mut outCache: FCore::Cache;
    let mut outEnv: FCore::Graph;
    let mut outIH: metamodelica::List<InnerOuter::TopInstance>;
    let mut outState: ClassInf::State;
    let mut outEquations: metamodelica::List<metamodelica::Ref<DAE::Element>>;
    checkForConnectInIfBranch(inEquations)?;
    let (__pa0, __pa1, __pa2, DAE::DAE { elementLst: __pa3 }, _, __pa4, _) = Inst::instList(
        inCache,
        inEnv,
        inIH,
        inPrefix,
        Connect::emptySet().clone(),
        inState,
        &instEquation,
        inEquations,
        inImpl,
        alwaysUnroll.clone(),
        ConnectionGraph::EMPTY().clone(),
    )?;
    outCache = metamodelica::Own::own(__pa0);
    outEnv = metamodelica::Own::own(__pa1);
    outIH = metamodelica::Own::own(__pa2);
    outEquations = metamodelica::Own::own(__pa3);
    outState = metamodelica::Own::own(__pa4);
    Ok((outCache, outEnv, outIH, outState, outEquations))
}

fn instIfEqBranches<'__b>(
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
    mut inIH: metamodelica::List<InnerOuter::TopInstance>,
    mut inPrefix: &'__b DAE::Prefix,
    mut inState: ClassInf::State,
    mut inBranches: metamodelica::List<metamodelica::List<metamodelica::Ref<SCode::Equation>>>,
    mut inImpl: bool,
    mut inAccumEqs: metamodelica::List<metamodelica::List<metamodelica::Ref<DAE::Element>>>,
) -> Result<(
    FCore::Cache,
    FCore::Graph,
    metamodelica::List<InnerOuter::TopInstance>,
    ClassInf::State,
    metamodelica::List<metamodelica::List<metamodelica::Ref<DAE::Element>>>,
)> {
    '__tco: loop {
        ::match_deref::match_deref! { match &((inCache.clone(), inEnv.clone(), inIH.clone(), inState.clone(), inBranches)) {
            (cache, env, ih, state, Deref @ metamodelica::ListNode::Cons { head: seq, tail: rest_seq }) => {
                let mut deq: metamodelica::List<metamodelica::Ref<DAE::Element>>;
                let mut branches: metamodelica::List<metamodelica::List<metamodelica::Ref<DAE::Element>>>;
                let mut cache = (*cache).clone();
                let mut env = (*env).clone();
                let mut ih = (*ih).clone();
                let mut state = (*state).clone();
                (cache, env, ih, state, deq) = instIfEqBranch(cache.clone(), env.clone(), ih.clone(), inPrefix.clone(), state.clone(), metamodelica::AsArg::as_arg(&seq), inImpl)?;
                { (inCache, inEnv, inIH, inPrefix, inState, inBranches, inImpl, inAccumEqs) = (cache.clone(), env.clone(), ih.clone(), inPrefix, state.clone(), rest_seq.clone(), inImpl, metamodelica::cons(deq, inAccumEqs)); continue '__tco; }
            },
            (_, _, _, _, Deref @ metamodelica::ListNode::Nil) => {
                return Ok((inCache, inEnv, inIH, inState, inAccumEqs.reverse()))
            },
            _ => return Err("match: no arm matched"),
        } }
    }
}

fn instInitialIfEqBranch(
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
    mut inIH: metamodelica::List<InnerOuter::TopInstance>,
    mut inPrefix: DAE::Prefix,
    mut inState: ClassInf::State,
    mut inEquations: &metamodelica::List<metamodelica::Ref<SCode::Equation>>,
    mut inImpl: bool,
) -> Result<(
    FCore::Cache,
    FCore::Graph,
    metamodelica::List<InnerOuter::TopInstance>,
    ClassInf::State,
    metamodelica::List<metamodelica::Ref<DAE::Element>>,
)> {
    let mut outCache: FCore::Cache;
    let mut outEnv: FCore::Graph;
    let mut outIH: metamodelica::List<InnerOuter::TopInstance>;
    let mut outState: ClassInf::State;
    let mut outEquations: metamodelica::List<metamodelica::Ref<DAE::Element>>;
    checkForConnectInIfBranch(inEquations)?;
    let (__pa0, __pa1, __pa2, DAE::DAE { elementLst: __pa3 }, _, __pa4, _) = Inst::instList(
        inCache,
        inEnv,
        inIH,
        inPrefix,
        Connect::emptySet().clone(),
        inState,
        &instInitialEquation,
        inEquations,
        inImpl,
        alwaysUnroll.clone(),
        ConnectionGraph::EMPTY().clone(),
    )?;
    outCache = metamodelica::Own::own(__pa0);
    outEnv = metamodelica::Own::own(__pa1);
    outIH = metamodelica::Own::own(__pa2);
    outEquations = metamodelica::Own::own(__pa3);
    outState = metamodelica::Own::own(__pa4);
    Ok((outCache, outEnv, outIH, outState, outEquations))
}

fn instInitialIfEqBranches<'__b>(
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
    mut inIH: metamodelica::List<InnerOuter::TopInstance>,
    mut inPrefix: &'__b DAE::Prefix,
    mut inState: ClassInf::State,
    mut inBranches: metamodelica::List<metamodelica::List<metamodelica::Ref<SCode::Equation>>>,
    mut inImpl: bool,
    mut inAccumEqs: metamodelica::List<metamodelica::List<metamodelica::Ref<DAE::Element>>>,
) -> Result<(
    FCore::Cache,
    FCore::Graph,
    metamodelica::List<InnerOuter::TopInstance>,
    ClassInf::State,
    metamodelica::List<metamodelica::List<metamodelica::Ref<DAE::Element>>>,
)> {
    '__tco: loop {
        ::match_deref::match_deref! { match &((inCache.clone(), inEnv.clone(), inIH.clone(), inState.clone(), inBranches)) {
            (cache, env, ih, state, Deref @ metamodelica::ListNode::Cons { head: seq, tail: rest_seq }) => {
                let mut deq: metamodelica::List<metamodelica::Ref<DAE::Element>>;
                let mut branches: metamodelica::List<metamodelica::List<metamodelica::Ref<DAE::Element>>>;
                let mut cache = (*cache).clone();
                let mut env = (*env).clone();
                let mut ih = (*ih).clone();
                let mut state = (*state).clone();
                (cache, env, ih, state, deq) = instInitialIfEqBranch(cache.clone(), env.clone(), ih.clone(), inPrefix.clone(), state.clone(), metamodelica::AsArg::as_arg(&seq), inImpl)?;
                { (inCache, inEnv, inIH, inPrefix, inState, inBranches, inImpl, inAccumEqs) = (cache.clone(), env.clone(), ih.clone(), inPrefix, state.clone(), rest_seq.clone(), inImpl, metamodelica::cons(deq, inAccumEqs)); continue '__tco; }
            },
            (_, _, _, _, Deref @ metamodelica::ListNode::Nil) => {
                return Ok((inCache, inEnv, inIH, inState, inAccumEqs.reverse()))
            },
            _ => return Err("match: no arm matched"),
        } }
    }
}

fn checkForConnectInIfBranch(mut inEquations: &metamodelica::List<metamodelica::Ref<SCode::Equation>>) -> Result<()> {
    List::map_0(inEquations, &move |__a0: metamodelica::Ref<SCode::Equation>| {
        checkForConnectInIfBranch2(&__a0)
    })?;
    Ok(())
}

fn checkForConnectInIfBranch2(mut inEquation: &metamodelica::Ref<SCode::Equation>) -> Result<()> {
    let () = (match &**inEquation {
        SCode::Equation::EQ_CONNECT {
            crefLeft: cr1,
            crefRight: cr2,
            info,
            ..
        } => {
            Error::addSourceMessage(
                &(Error::IN_NON_EVALUABLE_IF_OR_FOR.clone()),
                list![literal!("connect")],
                info,
            )?;
            return Err("fail");
        }
        SCode::Equation::EQ_FOR { eEquationLst: eqs, .. } => {
            checkForConnectInIfBranch(eqs)?;
            ()
        }
        _ => (),
    });
    Ok(())
}

fn instElseIfs(
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
    mut inIH: metamodelica::List<InnerOuter::TopInstance>,
    mut inPre: DAE::Prefix,
    mut ci_state: &ClassInf::State,
    mut inElseIfBranches: &metamodelica::List<(
        metamodelica::Ref<Absyn::Exp>,
        metamodelica::List<metamodelica::Ref<SCode::Statement>>,
    )>,
    mut source: &metamodelica::Ref<DAE::ElementSource>,
    mut initial_: SCode::Initial,
    mut inImpl: bool,
    mut unrollForLoops: bool,
    mut info: &SourceInfo,
) -> Result<(
    FCore::Cache,
    metamodelica::List<(
        metamodelica::Ref<DAE::Exp>,
        DAE::Properties,
        metamodelica::List<metamodelica::Ref<DAE::Statement>>,
    )>,
)> {
    let mut outCache: FCore::Cache;
    let mut outElseIfBranches: metamodelica::List<(
        metamodelica::Ref<DAE::Exp>,
        DAE::Properties,
        metamodelica::List<metamodelica::Ref<DAE::Statement>>,
    )>;
    (outCache, outElseIfBranches) = 'mc: {
        let __mc_input = (inCache, inEnv, inIH, inPre, &**inElseIfBranches, inImpl);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, _, _, _, Deref @ metamodelica::ListNode::Nil, _) => {
                    Ok((cache.clone(), metamodelica::nil()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, ih, pre, Deref @ metamodelica::ListNode::Cons { head: (e, l), tail: tail }, r#impl) => {
                    let mut e_1: metamodelica::Ref<DAE::Exp>;
                    let mut e_2: metamodelica::Ref<DAE::Exp>;
                    let mut prop: DAE::Properties;
                    let mut stmts: metamodelica::List<metamodelica::Ref<DAE::Statement>>;
                    let mut tail_1: metamodelica::List<(metamodelica::Ref<DAE::Exp>, DAE::Properties, metamodelica::List<metamodelica::Ref<DAE::Statement>>)>;
                    let mut cache = (*cache).clone();
                    (cache, e_1, prop) = Static::elabExp(cache.clone(), env.clone(), e.clone(), r#impl.clone(), true, pre.clone(), info.clone())?;
                    (cache, e_1, prop) = Ceval::cevalIfConstant(cache.clone(), env.clone(), e_1.clone(), prop.clone(), r#impl.clone(), info.clone())?;
                    (cache, e_2) = PrefixUtil::prefixExp(cache.clone(), metamodelica::AsArg::as_arg(&env), metamodelica::AsArg::as_arg(&ih), e_1.clone(), metamodelica::AsArg::as_arg(&pre))?;
                    (cache, stmts) = instStatements(cache.clone(), env.clone(), ih.clone(), pre.clone(), ci_state, metamodelica::AsArg::as_arg(&l), source.clone(), initial_, r#impl.clone(), unrollForLoops)?;
                    (cache, tail_1) = instElseIfs(cache.clone(), env.clone(), ih.clone(), pre.clone(), ci_state, metamodelica::AsArg::as_arg(&tail), source, initial_, r#impl.clone(), unrollForLoops, info)?;
                    Ok((cache.clone(), metamodelica::cons((e_2.clone(), prop.clone(), stmts.clone()), tail_1.clone())))
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
                    Debug::trace(literal!("- InstSection.instElseIfs failed\n"))?;
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((outCache, outElseIfBranches))
}

fn instWhenEqBranch(
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
    mut inIH: metamodelica::List<InnerOuter::TopInstance>,
    mut inPrefix: DAE::Prefix,
    mut inSets: DAE::Connect::Sets,
    mut inState: ClassInf::State,
    mut inBranch: &(
        metamodelica::Ref<Absyn::Exp>,
        metamodelica::List<metamodelica::Ref<SCode::Equation>>,
    ),
    mut inImpl: bool,
    mut inUnrollLoops: bool,
    mut inGraph: ConnectionGraph::ConnectionGraph,
    mut info: SourceInfo,
) -> Result<(
    FCore::Cache,
    FCore::Graph,
    metamodelica::List<InnerOuter::TopInstance>,
    metamodelica::Ref<DAE::Exp>,
    metamodelica::List<metamodelica::Ref<DAE::Element>>,
    ConnectionGraph::ConnectionGraph,
)> {
    let mut outCache: FCore::Cache = inCache.clone();
    let mut outEnv: FCore::Graph;
    let mut outIH: metamodelica::List<InnerOuter::TopInstance>;
    let mut outCondition: metamodelica::Ref<DAE::Exp>;
    let mut outEquations: metamodelica::List<metamodelica::Ref<DAE::Element>>;
    let mut outGraph: ConnectionGraph::ConnectionGraph;
    let mut cond: metamodelica::Ref<Absyn::Exp>;
    let mut body: metamodelica::List<metamodelica::Ref<SCode::Equation>>;
    let mut prop: DAE::Properties;
    let mut aexps: metamodelica::List<metamodelica::Ref<Absyn::Exp>>;
    let mut dexps: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
    let mut dexp: metamodelica::Ref<DAE::Exp>;
    let mut ty: metamodelica::Ref<DAE::Type>;
    let mut isClock: bool;
    (cond, body) = inBranch.clone();
    isClock = false;
    outCondition = (match &*cond {
        Absyn::Exp::ARRAY { arrayExp: __esc_aexps } => {
            aexps = (*__esc_aexps).clone();
            dexps = metamodelica::nil();
            for mut aexp in &*aexps.clone() {
                (outCache, dexp, prop) = instExp(
                    inCache.clone(),
                    inEnv.clone(),
                    &inIH,
                    inPrefix.clone(),
                    aexp.clone(),
                    inImpl,
                    info.clone(),
                )?;
                ty = Types::getPropType(&prop);
                dexp = checkWhenCondition(dexp, ty, aexp.clone(), &info)?;
                dexps = metamodelica::cons(dexp, dexps);
            }
            Expression::makeArray(dexps.reverse(), DAE::T_BOOL_DEFAULT().clone(), true)
        }
        _ => {
            (outCache, dexp, prop) = instExp(
                inCache,
                inEnv.clone(),
                &inIH,
                inPrefix.clone(),
                cond.clone(),
                inImpl,
                info.clone(),
            )?;
            ty = Types::getPropType(&prop);
            if Types::isClockOrSubTypeClock(ty.clone()) {
                isClock = true;
            } else {
                dexp = checkWhenCondition(dexp, ty, cond, &info)?;
            }
            dexp
        }
    });
    if !(isClock) {
        List::map_0(&body, &move |__a0: metamodelica::Ref<SCode::Equation>| {
            checkForNestedWhenInEq(&__a0)
        })?;
    }
    let (__pa0, __pa1, __pa2, DAE::DAE { elementLst: __pa3 }, _, _, __pa4) = Inst::instList(
        outCache,
        inEnv,
        inIH,
        inPrefix,
        inSets,
        inState,
        &instEquation,
        &body,
        inImpl,
        alwaysUnroll.clone(),
        inGraph,
    )?;
    outCache = metamodelica::Own::own(__pa0);
    outEnv = metamodelica::Own::own(__pa1);
    outIH = metamodelica::Own::own(__pa2);
    outEquations = metamodelica::Own::own(__pa3);
    outGraph = metamodelica::Own::own(__pa4);
    Ok((outCache, outEnv, outIH, outCondition, outEquations, outGraph))
}

fn checkWhenCondition(
    mut exp: metamodelica::Ref<DAE::Exp>,
    mut ty: metamodelica::Ref<DAE::Type>,
    mut aexp: metamodelica::Ref<Absyn::Exp>,
    mut info: &SourceInfo,
) -> Result<metamodelica::Ref<DAE::Exp>> {
    let mut exp: metamodelica::Ref<DAE::Exp> = exp;
    let mut tyEl: metamodelica::Ref<DAE::Type>;
    match '__try0: {
        if Types::isArray(&ty) {
            tyEl = Types::arrayElementType(&ty);
        } else {
            tyEl = ty.clone();
        }
        (exp, _) = unwrap_break_err!(Types::matchType(exp.clone(), tyEl.clone(), DAE::T_BOOL_DEFAULT().clone(), false), '__try0);
        Ok::<_, &'static str>((exp.clone(), tyEl.clone()))
    } {
        Ok((__try0_o0, __try0_o1)) => {
            exp = __try0_o0;
            tyEl = __try0_o1;
        }
        Err(__try0_err) => {
            Error::addSourceMessage(
                &(Error::IF_CONDITION_TYPE_ERROR.clone()),
                list![Dump::printExpStr(aexp.clone())?, TypesDump::unparseType(ty.clone())?],
                info,
            )?;
            return Err(__try0_err);
        }
    }
    if Config::languageStandardAtLeast(Config::LanguageStandard::_3_2.clone())? {
        let () = (::match_deref::match_deref! { match &(&*exp) {
            Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::IDENT { name: Deref @ "initial" }, .. } => (),
            Deref @ DAE::Exp::CALL { path: Deref @ Absyn::Path::FULLYQUALIFIED { path: Deref @ Absyn::Path::IDENT { name: Deref @ "initial" } }, .. } => (),
            _ => {
                if Expression::expHasInitial(exp.clone())? {
                    Error::addSourceMessage(&(Error::INITIAL_CALL_WARNING.clone()), list![Dump::printExpStr(aexp)?], info)?;
                }
                ()
            },
            _ => unreachable!("match_deref! exhaustiveness placeholder"),
        } });
    }
    Ok(exp)
}

fn instConnect(
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
    mut inIH: metamodelica::List<InnerOuter::TopInstance>,
    mut inSets: DAE::Connect::Sets,
    mut inPrefix: DAE::Prefix,
    mut inComponentRefLeft: metamodelica::Ref<Absyn::ComponentRef>,
    mut inComponentRefRight: metamodelica::Ref<Absyn::ComponentRef>,
    mut inImplicit: bool,
    mut inGraph: ConnectionGraph::ConnectionGraph,
    mut info: SourceInfo,
) -> Result<(
    FCore::Cache,
    FCore::Graph,
    metamodelica::List<InnerOuter::TopInstance>,
    DAE::Connect::Sets,
    DAE::DAElist,
    ConnectionGraph::ConnectionGraph,
)> {
    let mut outCache: FCore::Cache;
    let mut outEnv: FCore::Graph;
    let mut outIH: metamodelica::List<InnerOuter::TopInstance>;
    let mut outSets: DAE::Connect::Sets;
    let mut outDae: DAE::DAElist;
    let mut outGraph: ConnectionGraph::ConnectionGraph;
    (outCache, outEnv, outIH, outSets, outDae, outGraph) = 'mc: {
        let __mc_input = (
            inCache,
            inEnv,
            inIH,
            inSets,
            inPrefix,
            inComponentRefLeft,
            inComponentRefRight,
            inImplicit,
            inGraph,
        );
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, ih, sets, _, c1, c2, _, graph) => {
                    let mut s1: ArcStr;
                    let mut s2: ArcStr;
                    let true = (AbsynUtil::crefEqual(metamodelica::AsArg::as_arg(&c1), metamodelica::AsArg::as_arg(&c2))?) else { return Err("pattern mismatch") };
                    s1 = Dump::printComponentRefStr(metamodelica::AsArg::as_arg(&c1))?;
                    s2 = Dump::printComponentRefStr(metamodelica::AsArg::as_arg(&c1))?;
                    Error::addSourceMessage(&(Error::SAME_CONNECT_INSTANCE.clone()), list![s1.clone(), s2.clone()], &info)?;
                    Ok((cache.clone(), env.clone(), ih.clone(), sets.clone(), DAE::emptyDae().clone(), graph.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, ih, sets, pre, c1, c2, r#impl, graph) => {
                    let mut c1_2: metamodelica::Ref<DAE::ComponentRef>;
                    let mut c2_2: metamodelica::Ref<DAE::ComponentRef>;
                    let mut attr1: metamodelica::Ref<DAE::Attributes>;
                    let mut attr2: metamodelica::Ref<DAE::Attributes>;
                    let mut ct1: metamodelica::Ref<DAE::ConnectorType>;
                    let mut ty1: metamodelica::Ref<DAE::Type>;
                    let mut ty2: metamodelica::Ref<DAE::Type>;
                    let mut f1: DAE::Connect::Face;
                    let mut f2: DAE::Connect::Face;
                    let mut dae: DAE::DAElist;
                    let mut io1: Absyn::InnerOuter;
                    let mut io2: Absyn::InnerOuter;
                    let mut vt1: SCode::Variability;
                    let mut vt2: SCode::Variability;
                    let mut del1: bool;
                    let mut del2: bool;
                    let mut cache = (*cache).clone();
                    let mut ih = (*ih).clone();
                    let mut sets = (*sets).clone();
                    let mut graph = (*graph).clone();
                    (cache, c1_2, attr1, ct1, vt1, io1, f1, ty1, del1) = instConnector(cache.clone(), env.clone(), metamodelica::AsArg::as_arg(&ih), c1.clone(), r#impl.clone(), pre.clone(), info.clone())?;
                    (cache, c2_2, attr2, _, vt2, io2, f2, ty2, del2) = instConnector(cache.clone(), env.clone(), metamodelica::AsArg::as_arg(&ih), c2.clone(), r#impl.clone(), pre.clone(), info.clone())?;
                    if del1 || del2 {
                        dae = DAE::emptyDae().clone();
                    } else if Types::isExpandableConnector(&ty1) || Types::isExpandableConnector(&ty2) {
                        return Err("fail");
                    } else {
                        checkConnectTypes(&c1_2, ty1.clone(), f1, &attr1, &c2_2, ty2.clone(), f2, &attr2, &info)?;
                        (cache, _, ih, sets, dae, graph) = connectComponents(cache.clone(), env.clone(), ih.clone(), sets.clone(), pre.clone(), c1_2.clone(), f1, ty1.clone(), vt1, c2_2.clone(), f2, ty2.clone(), vt2, ct1.clone(), io1, io2, graph.clone(), &info)?;
                        sets = ConnectUtil::increaseConnectRefCount(&c1_2, &c2_2, sets.clone())?;
                    }
                    Ok((cache.clone(), env.clone(), ih.clone(), sets.clone(), dae.clone(), graph.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, ih, sets, pre, c1, c2, r#impl, graph) => {
                    let mut dae: DAE::DAElist;
                    let mut cache = (*cache).clone();
                    let mut env = (*env).clone();
                    let mut ih = (*ih).clone();
                    let mut sets = (*sets).clone();
                    let mut graph = (*graph).clone();
                    ErrorExt::setCheckpoint(literal!("expandableConnectors"));
                    let true = (System::getHasExpandableConnectors()) else { return Err("pattern mismatch") };
                    (cache, env, ih, sets, dae, graph) = connectExpandableConnectors(cache.clone(), env.clone(), ih.clone(), sets.clone(), pre.clone(), c1.clone(), c2.clone(), r#impl.clone(), graph.clone(), &info)?;
                    ErrorExt::rollBack(literal!("expandableConnectors"));
                    Ok((cache.clone(), env.clone(), ih.clone(), sets.clone(), dae.clone(), graph.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, _, _, pre, c1, c2, _, _) => {
                    let mut subs1: metamodelica::List<metamodelica::Ref<Absyn::Subscript>>;
                    let mut subs2: metamodelica::List<metamodelica::Ref<Absyn::Subscript>>;
                    let mut crefs1: metamodelica::List<metamodelica::Ref<Absyn::ComponentRef>>;
                    let mut crefs2: metamodelica::List<metamodelica::Ref<Absyn::ComponentRef>>;
                    let mut s1: ArcStr;
                    let mut s2: ArcStr;
                    ErrorExt::rollBack(literal!("expandableConnectors"));
                    subs1 = AbsynUtil::getSubsFromCref(metamodelica::AsArg::as_arg(&c1), true, true)?;
                    crefs1 = AbsynUtil::getCrefsFromSubs(&subs1, true, true)?;
                    subs2 = AbsynUtil::getSubsFromCref(metamodelica::AsArg::as_arg(&c2), true, true)?;
                    crefs2 = AbsynUtil::getCrefsFromSubs(&subs2, true, true)?;
                    s1 = Dump::printComponentRefStr(metamodelica::AsArg::as_arg(&c1))?;
                    s2 = Dump::printComponentRefStr(metamodelica::AsArg::as_arg(&c2))?;
                    s1 = { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("connect(")); __mm_s.push_str(&*s1); __mm_s.push_str(&*literal!(", ")); __mm_s.push_str(&*s2); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) };
                    checkConstantVariability(&crefs1, metamodelica::AsArg::as_arg(&cache), metamodelica::AsArg::as_arg(&env), &s1, pre.clone(), &info)?;
                    checkConstantVariability(&crefs2, metamodelica::AsArg::as_arg(&cache), metamodelica::AsArg::as_arg(&env), &s1, pre.clone(), &info)?;
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, ih, sets, _, _, _, _, graph) => {
                    if !((Config::getGraphicsExpMode()?)) { return Err("guard") }
                    Ok((cache.clone(), env.clone(), ih.clone(), sets.clone(), DAE::emptyDae().clone(), graph.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _, _, _, c1, c2, _, _) => {
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { return Err("pattern mismatch") };
                    Debug::traceln({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("- InstSection.instConnect failed for: connect(")); __mm_s.push_str(&*Dump::printComponentRefStr(metamodelica::AsArg::as_arg(&c1))?); __mm_s.push_str(&*literal!(", ")); __mm_s.push_str(&*Dump::printComponentRefStr(metamodelica::AsArg::as_arg(&c2))?); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) })?;
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((outCache, outEnv, outIH, outSets, outDae, outGraph))
}

fn instConnector(
    mut inCache: FCore::Cache,
    mut env: FCore::Graph,
    mut ih: &metamodelica::List<InnerOuter::TopInstance>,
    mut connectorCref: metamodelica::Ref<Absyn::ComponentRef>,
    mut r#impl: bool,
    mut prefix: DAE::Prefix,
    mut info: SourceInfo,
) -> Result<(
    FCore::Cache,
    metamodelica::Ref<DAE::ComponentRef>,
    metamodelica::Ref<DAE::Attributes>,
    metamodelica::Ref<DAE::ConnectorType>,
    SCode::Variability,
    Absyn::InnerOuter,
    DAE::Connect::Face,
    metamodelica::Ref<DAE::Type>,
    bool,
)> {
    let mut outCache: FCore::Cache = inCache.clone();
    let mut outCref: metamodelica::Ref<DAE::ComponentRef>;
    let mut outAttr: metamodelica::Ref<DAE::Attributes>;
    let mut connectorType: metamodelica::Ref<DAE::ConnectorType>;
    let mut variability: SCode::Variability;
    let mut innerOuter: Absyn::InnerOuter;
    let mut face: DAE::Connect::Face;
    let mut ty: metamodelica::Ref<DAE::Type>;
    let mut deleted: bool;
    let mut status: FCore::Status;
    let mut is_expandable: bool;
    outCref = ComponentReference::toExpCref(&connectorCref)?;
    let (__t6, __pa3, __pa4, __pa5) = Lookup::lookupConnectorVar(&env, &outCref, true)?;
    let __arc7 = __t6.clone();
    let DAE::ATTR {
        connectorType: __pa0,
        variability: __pa1,
        innerOuter: __pa2,
        ..
    } = &*__arc7;
    connectorType = metamodelica::Own::own(__pa0);
    variability = metamodelica::Own::own(__pa1);
    innerOuter = metamodelica::Own::own(__pa2);
    ty = metamodelica::Own::own(__pa3);
    status = metamodelica::Own::own(__pa4);
    is_expandable = metamodelica::Own::own(__pa5);
    deleted = FCore::isDeletedComp(&status);
    if deleted || is_expandable {
        face = openmodelica_frontend_types::DAE::Connect::Face::NO_FACE;
        outAttr = DAE::dummyAttrVar().clone();
    } else {
        let (__pa8, __pa9, __pa10, __pa11) = ::match_deref::match_deref! { match &(Static::elabCrefNoEval(inCache, env.clone(), connectorCref, r#impl, false, prefix, info.clone())?) {
            (__pa8, Deref @ DAE::Exp::CREF { componentRef: __pa9, .. }, DAE::Properties::PROP { type_: __pa10, .. }, __pa11) => (__pa8.clone(), __pa9.clone(), __pa10.clone(), __pa11.clone()),
            _ => return Err("pattern mismatch"),
        } };
        outCache = metamodelica::Own::own(__pa8);
        outCref = metamodelica::Own::own(__pa9);
        ty = metamodelica::Own::own(__pa10);
        outAttr = metamodelica::Own::own(__pa11);
        (outCache, outCref) = Static::canonCref(outCache, env.clone(), outCref, r#impl)?;
        validConnector(&ty, &outCref, &info)?;
        face = ConnectUtil::componentFace(env, &outCref)?;
        ty = sortConnectorType(&ty)?;
    }
    Ok((
        outCache,
        outCref,
        outAttr,
        connectorType,
        variability,
        innerOuter,
        face,
        ty,
        deleted,
    ))
}

fn sortConnectorType(mut inType: &metamodelica::Ref<DAE::Type>) -> Result<metamodelica::Ref<DAE::Type>> {
    let mut outType: metamodelica::Ref<DAE::Type>;
    outType = (match &**inType {
        DAE::Type::T_ARRAY { ty, dims } => {
            let mut ty = (*ty).clone();
            ty = sortConnectorType(metamodelica::AsArg::as_arg(&ty))?;
            metamodelica::Ref::new(DAE::Type::T_ARRAY {
                ty: ty.clone(),
                dims: dims.clone(),
            })
        }
        DAE::Type::T_COMPLEX {
            complexClassType: ci_state,
            varLst: vars,
            equalityConstraint: ec,
            usedExternally: __inType_usedExternally,
        } => {
            let mut vars = (*vars).clone();
            vars = List::sort(
                vars.clone(),
                (std::sync::Arc::new(
                    move |__a0: metamodelica::Ref<DAE::Var>,
                          __a1: metamodelica::Ref<DAE::Var>|
                          -> metamodelica::Result<_> {
                        ::std::result::Result::Ok(connectorCompGt(&__a0, &__a1))
                    },
                )
                    as std::sync::Arc<
                        dyn ::std::ops::Fn(metamodelica::Ref<DAE::Var>, metamodelica::Ref<DAE::Var>) -> Result<bool>
                            + 'static,
                    >),
            )?;
            metamodelica::Ref::new(DAE::Type::T_COMPLEX {
                complexClassType: ci_state.clone(),
                varLst: vars.clone(),
                equalityConstraint: ec.clone(),
                usedExternally: __inType_usedExternally.clone(),
            })
        }
        _ => inType.clone(),
    });
    Ok(outType)
}

fn connectorCompGt(mut inVar1: &metamodelica::Ref<DAE::Var>, mut inVar2: &metamodelica::Ref<DAE::Var>) -> bool {
    let mut outGt: bool;
    let mut id1: ArcStr;
    let mut id2: ArcStr;
    let __arc1 = &(*inVar1);
    let DAE::TYPES_VAR { name: __pa0, .. } = &**__arc1;
    id1 = metamodelica::Own::own(__pa0);
    let __arc3 = &(*inVar2);
    let DAE::TYPES_VAR { name: __pa2, .. } = &**__arc3;
    id2 = metamodelica::Own::own(__pa2);
    outGt = 1 == stringCompare(&id1, &id2);
    outGt
}

fn checkConstantVariability(
    mut inrefs: &metamodelica::List<metamodelica::Ref<Absyn::ComponentRef>>,
    mut cache: &FCore::Cache,
    mut env: &FCore::Graph,
    mut affectedConnector: &ArcStr,
    mut inPrefix: DAE::Prefix,
    mut info: &SourceInfo,
) -> Result<()> {
    let () = 'mc: {
        let __mc_input = (&**inrefs, inPrefix);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Nil, _) => {
                    Ok(())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: cr, tail: refs }, pre) => {
                    let mut prop: DAE::Properties;
                    let mut r#const: DAE::Const;
                    let __pa0 = ::match_deref::match_deref! { match &(Static::elabCref(cache.clone(), env.clone(), cr.clone(), false, false, pre.clone(), info.clone())?) {
                        (_, Some((_, __pa0, _))) => __pa0.clone(),
                        _ => return Err("pattern mismatch"),
                    } };
                    prop = metamodelica::Own::own(__pa0);
                    r#const = Types::propertiesListToConst(&(list![prop.clone()]))?;
                    let true = (Types::isParameterOrConstant(r#const)) else { return Err("pattern mismatch") };
                    checkConstantVariability(metamodelica::AsArg::as_arg(&refs), cache, env, affectedConnector, pre.clone(), info)?;
                    Ok(())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: cr, tail: _ }, pre) => {
                    let mut prop: DAE::Properties;
                    let mut r#const: DAE::Const;
                    let mut s1: ArcStr;
                    let __pa0 = ::match_deref::match_deref! { match &(Static::elabCref(cache.clone(), env.clone(), cr.clone(), false, false, pre.clone(), info.clone())?) {
                        (_, Some((_, __pa0, _))) => __pa0.clone(),
                        _ => return Err("pattern mismatch"),
                    } };
                    prop = metamodelica::Own::own(__pa0);
                    r#const = Types::propertiesListToConst(&(list![prop.clone()]))?;
                    let false = (Types::isParameterOrConstant(r#const)) else { return Err("pattern mismatch") };
                    s1 = Dump::printComponentRefStr(metamodelica::AsArg::as_arg(&cr))?;
                    Error::addSourceMessage(&(Error::CONNECTOR_ARRAY_NONCONSTANT.clone()), list![affectedConnector.clone(), s1.clone()], info)?;
                    Ok(())
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

fn connectExpandableConnectors(
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
    mut inIH: metamodelica::List<InnerOuter::TopInstance>,
    mut inSets: DAE::Connect::Sets,
    mut inPrefix: DAE::Prefix,
    mut inComponentRefLeft: metamodelica::Ref<Absyn::ComponentRef>,
    mut inComponentRefRight: metamodelica::Ref<Absyn::ComponentRef>,
    mut inImpl: bool,
    mut inGraph: ConnectionGraph::ConnectionGraph,
    mut info: &SourceInfo,
) -> Result<(
    FCore::Cache,
    FCore::Graph,
    metamodelica::List<InnerOuter::TopInstance>,
    DAE::Connect::Sets,
    DAE::DAElist,
    ConnectionGraph::ConnectionGraph,
)> {
    let mut outCache: FCore::Cache;
    let mut outEnv: FCore::Graph;
    let mut outIH: metamodelica::List<InnerOuter::TopInstance>;
    let mut outSets: DAE::Connect::Sets;
    let mut outDae: DAE::DAElist;
    let mut outGraph: ConnectionGraph::ConnectionGraph;
    (outCache, outEnv, outIH, outSets, outDae, outGraph) = 'mc: {
        let __mc_input = (
            inCache,
            inEnv,
            inIH,
            inSets,
            inPrefix,
            inComponentRefLeft,
            inComponentRefRight,
            inImpl,
            inGraph,
        );
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, ih, sets, pre, c1, c2, r#impl, graph) => {
                    let mut c1_1: metamodelica::Ref<DAE::ComponentRef>;
                    let mut c2_1: metamodelica::Ref<DAE::ComponentRef>;
                    let mut c1_2: metamodelica::Ref<DAE::ComponentRef>;
                    let mut c2_2: metamodelica::Ref<DAE::ComponentRef>;
                    let mut attr1: metamodelica::Ref<DAE::Attributes>;
                    let mut attr2: metamodelica::Ref<DAE::Attributes>;
                    let mut ty1: metamodelica::Ref<DAE::Type>;
                    let mut ty2: metamodelica::Ref<DAE::Type>;
                    let mut dae: DAE::DAElist;
                    let mut env1: FCore::Graph;
                    let mut env2: FCore::Graph;
                    let mut variables1: metamodelica::List<ArcStr>;
                    let mut variables2: metamodelica::List<ArcStr>;
                    let mut variablesUnion: metamodelica::List<ArcStr>;
                    let mut cache = (*cache).clone();
                    let mut env = (*env).clone();
                    let mut ih = (*ih).clone();
                    let mut sets = (*sets).clone();
                    let mut graph = (*graph).clone();
                    let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(Static::elabCref(cache.clone(), env.clone(), c1.clone(), r#impl.clone(), false, pre.clone(), info.clone())?) {
                        (__pa0, Some((Deref @ DAE::Exp::CREF { componentRef: __pa1, ty: _ }, _, __pa2))) => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    cache = metamodelica::Own::own(__pa0);
                    c1_1 = metamodelica::Own::own(__pa1);
                    attr1 = metamodelica::Own::own(__pa2);
                    let (__pa4, __pa5, __pa6) = ::match_deref::match_deref! { match &(Static::elabCref(cache.clone(), env.clone(), c2.clone(), r#impl.clone(), false, pre.clone(), info.clone())?) {
                        (__pa4, Some((Deref @ DAE::Exp::CREF { componentRef: __pa5, ty: _ }, _, __pa6))) => (__pa4.clone(), __pa5.clone(), __pa6.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    cache = metamodelica::Own::own(__pa4);
                    c2_1 = metamodelica::Own::own(__pa5);
                    attr2 = metamodelica::Own::own(__pa6);
                    (cache, c1_2) = Static::canonCref(cache.clone(), env.clone(), c1_1.clone(), r#impl.clone())?;
                    (cache, c2_2) = Static::canonCref(cache.clone(), env.clone(), c2_1.clone(), r#impl.clone())?;
                    (attr1, ty1, _, _) = Lookup::lookupConnectorVar(metamodelica::AsArg::as_arg(&env), &c1_2, true)?;
                    (attr2, ty2, _, _) = Lookup::lookupConnectorVar(metamodelica::AsArg::as_arg(&env), &c2_2, true)?;
                    ::match_deref::match_deref! { match &(attr1.clone()) {
                        Deref @ DAE::Attributes { connectorType: Deref @ DAE::ConnectorType::POTENTIAL { .. }, .. } => (),
                        _ => return Err("pattern mismatch"),
                    } };
                    ::match_deref::match_deref! { match &(attr2.clone()) {
                        Deref @ DAE::Attributes { connectorType: Deref @ DAE::ConnectorType::POTENTIAL { .. }, .. } => (),
                        _ => return Err("pattern mismatch"),
                    } };
                    let true = (Types::isExpandableConnector(&ty1)) else { return Err("pattern mismatch") };
                    let true = (Types::isExpandableConnector(&ty2)) else { return Err("pattern mismatch") };
                    (_, _, _, _, _, _, _, env1, _) = Lookup::lookupVar(cache.clone(), env.clone(), c1_2.clone())?;
                    (_, _, _, _, _, _, _, env2, _) = Lookup::lookupVar(cache.clone(), env.clone(), c2_2.clone())?;
                    variables1 = FGraph::getVariablesFromGraphScope(&env1)?;
                    variables2 = FGraph::getVariablesFromGraphScope(&env2)?;
                    variablesUnion = List::union(&variables1, &variables2);
                    variablesUnion = List::sort(variablesUnion.clone(), (std::sync::Arc::new(move |__a0: ArcStr, __a1: ArcStr| -> metamodelica::Result<_> { ::std::result::Result::Ok(Util::strcmpBool(&__a0, &__a1)) }) as std::sync::Arc<dyn ::std::ops::Fn(ArcStr, ArcStr) -> Result<bool> + 'static>))?;
                    (cache, env, ih, sets, dae, graph) = connectExpandableVariables(cache.clone(), env.clone(), ih.clone(), sets.clone(), pre.clone(), c1.clone(), c2.clone(), &variablesUnion, r#impl.clone(), graph.clone(), info)?;
                    Ok((cache.clone(), env.clone(), ih.clone(), sets.clone(), dae.clone(), graph.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, ih, sets, pre, c1, c2, r#impl, graph) => {
                    let mut dae: DAE::DAElist;
                    let mut cache = (*cache).clone();
                    let mut env = (*env).clone();
                    let mut ih = (*ih).clone();
                    let mut sets = (*sets).clone();
                    let mut graph = (*graph).clone();
                    let __pa0 = ::match_deref::match_deref! { match &(Static::elabCref(cache.clone(), env.clone(), c2.clone(), r#impl.clone(), false, pre.clone(), info.clone())?) {
                        (__pa0, None) => __pa0.clone(),
                        _ => return Err("pattern mismatch"),
                    } };
                    cache = metamodelica::Own::own(__pa0);
                    let __pa1 = ::match_deref::match_deref! { match &(Static::elabCref(cache.clone(), env.clone(), c1.clone(), r#impl.clone(), false, pre.clone(), info.clone())?) {
                        (__pa1, Some((Deref @ DAE::Exp::CREF { componentRef: _, ty: _ }, _, _))) => __pa1.clone(),
                        _ => return Err("pattern mismatch"),
                    } };
                    cache = metamodelica::Own::own(__pa1);
                    (cache, env, ih, sets, dae, graph) = connectExpandableConnectors(cache.clone(), env.clone(), ih.clone(), sets.clone(), pre.clone(), c2.clone(), c1.clone(), r#impl.clone(), graph.clone(), info)?;
                    Ok((cache.clone(), env.clone(), ih.clone(), sets.clone(), dae.clone(), graph.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, _, _, pre, c1 @ Deref @ Absyn::ComponentRef::CREF_IDENT { .. }, c2, r#impl, _) => {
                    let mut cache = (*cache).clone();
                    let __pa0 = ::match_deref::match_deref! { match &(Static::elabCref(cache.clone(), env.clone(), c1.clone(), r#impl.clone(), false, pre.clone(), info.clone())?) {
                        (__pa0, None) => __pa0.clone(),
                        _ => return Err("pattern mismatch"),
                    } };
                    cache = metamodelica::Own::own(__pa0);
                    metamodelica::print({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Error: The marked virtual expandable component reference in connect([")); __mm_s.push_str(&*PrefixUtil::printPrefixStrIgnoreNoPre(pre.clone())?); __mm_s.push_str(&*literal!(".")); __mm_s.push_str(&*Dump::printComponentRefStr(metamodelica::AsArg::as_arg(&c1))?); __mm_s.push_str(&*literal!("], ")); __mm_s.push_str(&*PrefixUtil::printPrefixStrIgnoreNoPre(pre.clone())?); __mm_s.push_str(&*literal!(".")); __mm_s.push_str(&*Dump::printComponentRefStr(metamodelica::AsArg::as_arg(&c2))?); __mm_s.push_str(&*literal!("); should be qualified, i.e. expandableConnectorName.virtualName!\n")); ArcStr::from(__mm_s) });
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, ih, sets, pre, c1 @ Deref @ Absyn::ComponentRef::CREF_QUAL { .. }, c2, r#impl, graph) => {
                    let mut c1_1: metamodelica::Ref<DAE::ComponentRef>;
                    let mut c2_1: metamodelica::Ref<DAE::ComponentRef>;
                    let mut c1_2: metamodelica::Ref<DAE::ComponentRef>;
                    let mut c2_2: metamodelica::Ref<DAE::ComponentRef>;
                    let mut attr2: metamodelica::Ref<DAE::Attributes>;
                    let mut attr: metamodelica::Ref<DAE::Attributes>;
                    let mut ct2: metamodelica::Ref<DAE::ConnectorType>;
                    let mut ty1: metamodelica::Ref<DAE::Type>;
                    let mut ty2: metamodelica::Ref<DAE::Type>;
                    let mut ty: metamodelica::Ref<DAE::Type>;
                    let mut dae: DAE::DAElist;
                    let mut envExpandable: FCore::Graph;
                    let mut envComponent: FCore::Graph;
                    let mut envComponentEmpty: FCore::Graph;
                    let mut c1_prefix: metamodelica::Ref<Absyn::ComponentRef>;
                    let mut io2: Absyn::InnerOuter;
                    let mut vt2: SCode::Variability;
                    let mut prl2: SCode::Parallelism;
                    let mut componentName: ArcStr;
                    let mut binding: metamodelica::Ref<DAE::Binding>;
                    let mut cnstForRange: Option<DAE::Const>;
                    let mut variablesUnion: metamodelica::List<ArcStr>;
                    let mut vis2: SCode::Visibility;
                    let mut arrDims: metamodelica::List<metamodelica::Ref<Absyn::Subscript>>;
                    let mut daeDims: metamodelica::List<metamodelica::Ref<DAE::Dimension>>;
                    let mut cache = (*cache).clone();
                    let mut env = (*env).clone();
                    let mut ih = (*ih).clone();
                    let mut sets = (*sets).clone();
                    let mut graph = (*graph).clone();
                    let __pa0 = ::match_deref::match_deref! { match &(Static::elabCref(cache.clone(), env.clone(), c1.clone(), r#impl.clone(), false, pre.clone(), info.clone())?) {
                        (__pa0, None) => __pa0.clone(),
                        _ => return Err("pattern mismatch"),
                    } };
                    cache = metamodelica::Own::own(__pa0);
                    let (__pa1, __pa2, __pa3) = ::match_deref::match_deref! { match &(Static::elabCref(cache.clone(), env.clone(), c2.clone(), r#impl.clone(), false, pre.clone(), info.clone())?) {
                        (__pa1, Some((Deref @ DAE::Exp::CREF { componentRef: __pa2, ty: _ }, _, __pa3))) => (__pa1.clone(), __pa2.clone(), __pa3.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    cache = metamodelica::Own::own(__pa1);
                    c2_1 = metamodelica::Own::own(__pa2);
                    attr2 = metamodelica::Own::own(__pa3);
                    (cache, c2_2) = Static::canonCref(cache.clone(), env.clone(), c2_1.clone(), r#impl.clone())?;
                    (attr2, ty2, _, _) = Lookup::lookupConnectorVar(metamodelica::AsArg::as_arg(&env), &c2_2, true)?;
                    let __arc10 = attr2.clone();
                    let DAE::ATTR { connectorType: __pa5, parallelism: __pa6, variability: __pa7, direction: _, innerOuter: __pa8, visibility: __pa9 } = &*__arc10;
                    ct2 = metamodelica::Own::own(__pa5);
                    prl2 = metamodelica::Own::own(__pa6);
                    vt2 = metamodelica::Own::own(__pa7);
                    io2 = metamodelica::Own::own(__pa8);
                    vis2 = metamodelica::Own::own(__pa9);
                    c1_prefix = AbsynUtil::crefStripLast(metamodelica::AsArg::as_arg(&c1))?;
                    let (__pa11, __pa12) = ::match_deref::match_deref! { match &(Static::elabCref(cache.clone(), env.clone(), c1_prefix.clone(), r#impl.clone(), false, pre.clone(), info.clone())?) {
                        (__pa11, Some((Deref @ DAE::Exp::CREF { componentRef: __pa12, ty: _ }, _, _))) => (__pa11.clone(), __pa12.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    cache = metamodelica::Own::own(__pa11);
                    c1_1 = metamodelica::Own::own(__pa12);
                    (cache, c1_2) = Static::canonCref(cache.clone(), env.clone(), c1_1.clone(), r#impl.clone())?;
                    (_, ty1, _, _) = Lookup::lookupConnectorVar(metamodelica::AsArg::as_arg(&env), &c1_2, true)?;
                    let true = (Types::isExpandableConnector(&ty1)) else { return Err("pattern mismatch") };
                    c1_2 = ComponentReferenceBasics::crefStripLastSubs(&c1_2)?;
                    (_, attr, ty, binding, cnstForRange, _, _, envExpandable, _) = Lookup::lookupVar(cache.clone(), env.clone(), c1_2.clone())?;
                    (_, _, _, _, _, _, _, envComponent, _) = Lookup::lookupVar(cache.clone(), env.clone(), c2_2.clone())?;
                    variablesUnion = FGraph::getVariablesFromGraphScope(&envComponent)?;
                    let true = (((variablesUnion).len() as i32) > 1) else { return Err("pattern mismatch") };
                    componentName = AbsynUtil::crefGetLastIdent(metamodelica::AsArg::as_arg(&c1))?;
                    envComponentEmpty = FGraph::removeComponentsFromScope(envComponent.clone())?;
                    daeDims = TypesDump::getDimensions(&ty2);
                    arrDims = List::map(daeDims.clone(), &move |__a0: metamodelica::Ref<DAE::Dimension>| Expression::unelabDimension(&__a0))?;
                    envExpandable = FGraph::cloneLastScopeRef(envExpandable.clone())?;
                    envExpandable = FGraph::mkComponentNode(envExpandable.clone(), metamodelica::Ref::new(DAE::Var { name: componentName.clone(), attributes: metamodelica::Ref::new(DAE::Attributes { connectorType: ct2.clone(), parallelism: prl2, variability: vt2, direction: openmodelica_ast::Absyn::Direction::BIDIR, innerOuter: io2, visibility: vis2 }), ty: ty2.clone(), binding: openmodelica_frontend_types::DAE::Binding::interned_UNBOUND(), bind_from_outside: false, constOfForIteratorRange: None }), metamodelica::Ref::new(SCode::Element::COMPONENT { name: componentName.clone(), prefixes: SCode::defaultPrefixes.clone(), attributes: SCode::Attributes { arrayDims: arrDims.clone(), connectorType: openmodelica_frontend_types::SCode::ConnectorType::POTENTIAL, parallelism: openmodelica_frontend_types::SCode::Parallelism::NON_PARALLEL, variability: openmodelica_frontend_types::SCode::Variability::VAR, direction: openmodelica_ast::Absyn::Direction::BIDIR, isField: openmodelica_ast::Absyn::IsField::NONFIELD }, typeSpec: metamodelica::Ref::new(Absyn::TypeSpec::TPATH { path: metamodelica::Ref::new(Absyn::Path::IDENT { name: literal!("") }), arrayDim: None }), modifications: openmodelica_frontend_types::SCode::Mod::interned_NOMOD(), comment: SCode::noComment.clone(), condition: None, info: Absyn::dummyInfo.clone() }), openmodelica_frontend_types::DAE::Mod::interned_NOMOD(), openmodelica_frontend_dump::FCore::Status::VAR_TYPED, envComponentEmpty.clone())?;
                    env = updateEnvComponentsOnQualPath(cache.clone(), env.clone(), c1_2.clone(), attr.clone(), ty.clone(), binding.clone(), cnstForRange.clone(), envExpandable.clone())?;
                    (cache, env, ih, sets, dae, graph) = connectExpandableVariables(cache.clone(), env.clone(), ih.clone(), sets.clone(), pre.clone(), c1.clone(), c2.clone(), &variablesUnion, r#impl.clone(), graph.clone(), info)?;
                    Ok((cache.clone(), env.clone(), ih.clone(), sets.clone(), dae.clone(), graph.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, ih, sets, pre, c1 @ Deref @ Absyn::ComponentRef::CREF_QUAL { .. }, c2, r#impl, graph) => {
                    let mut c1_1: metamodelica::Ref<DAE::ComponentRef>;
                    let mut c2_1: metamodelica::Ref<DAE::ComponentRef>;
                    let mut c1_2: metamodelica::Ref<DAE::ComponentRef>;
                    let mut c2_2: metamodelica::Ref<DAE::ComponentRef>;
                    let mut c1p: metamodelica::Ref<DAE::ComponentRef>;
                    let mut c2p: metamodelica::Ref<DAE::ComponentRef>;
                    let mut attr1: metamodelica::Ref<DAE::Attributes>;
                    let mut attr2: metamodelica::Ref<DAE::Attributes>;
                    let mut attr: metamodelica::Ref<DAE::Attributes>;
                    let mut ct1: metamodelica::Ref<DAE::ConnectorType>;
                    let mut ct2: metamodelica::Ref<DAE::ConnectorType>;
                    let mut ty1: metamodelica::Ref<DAE::Type>;
                    let mut ty2: metamodelica::Ref<DAE::Type>;
                    let mut ty: metamodelica::Ref<DAE::Type>;
                    let mut dae: DAE::DAElist;
                    let mut daeExpandable: DAE::DAElist;
                    let mut envExpandable: FCore::Graph;
                    let mut envComponent: FCore::Graph;
                    let mut envComponentEmpty: FCore::Graph;
                    let mut c1_prefix: metamodelica::Ref<Absyn::ComponentRef>;
                    let mut io1: Absyn::InnerOuter;
                    let mut io2: Absyn::InnerOuter;
                    let mut vt1: SCode::Variability;
                    let mut vt2: SCode::Variability;
                    let mut prl1: SCode::Parallelism;
                    let mut prl2: SCode::Parallelism;
                    let mut componentName: ArcStr;
                    let mut binding: metamodelica::Ref<DAE::Binding>;
                    let mut cnstForRange: Option<DAE::Const>;
                    let mut state: ClassInf::State;
                    let mut variablesUnion: metamodelica::List<ArcStr>;
                    let mut source: metamodelica::Ref<DAE::ElementSource>;
                    let mut vis1: SCode::Visibility;
                    let mut vis2: SCode::Visibility;
                    let mut arrDims: metamodelica::List<metamodelica::Ref<Absyn::Subscript>>;
                    let mut daeDims: metamodelica::List<metamodelica::Ref<DAE::Dimension>>;
                    let mut cache = (*cache).clone();
                    let mut env = (*env).clone();
                    let mut ih = (*ih).clone();
                    let mut sets = (*sets).clone();
                    let mut graph = (*graph).clone();
                    let __pa0 = ::match_deref::match_deref! { match &(Static::elabCref(cache.clone(), env.clone(), c1.clone(), r#impl.clone(), false, pre.clone(), info.clone())?) {
                        (__pa0, None) => __pa0.clone(),
                        _ => return Err("pattern mismatch"),
                    } };
                    cache = metamodelica::Own::own(__pa0);
                    let (__pa1, __pa2, __pa3) = ::match_deref::match_deref! { match &(Static::elabCref(cache.clone(), env.clone(), c2.clone(), r#impl.clone(), false, pre.clone(), info.clone())?) {
                        (__pa1, Some((Deref @ DAE::Exp::CREF { componentRef: __pa2, ty: _ }, _, __pa3))) => (__pa1.clone(), __pa2.clone(), __pa3.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    cache = metamodelica::Own::own(__pa1);
                    c2_1 = metamodelica::Own::own(__pa2);
                    attr2 = metamodelica::Own::own(__pa3);
                    (cache, c2_2) = Static::canonCref(cache.clone(), env.clone(), c2_1.clone(), r#impl.clone())?;
                    (attr2, ty2, _, _) = Lookup::lookupConnectorVar(metamodelica::AsArg::as_arg(&env), &c2_2, true)?;
                    let __arc10 = attr2.clone();
                    let DAE::ATTR { connectorType: __pa5, parallelism: __pa6, variability: __pa7, direction: _, innerOuter: __pa8, visibility: __pa9 } = &*__arc10;
                    ct2 = metamodelica::Own::own(__pa5);
                    prl2 = metamodelica::Own::own(__pa6);
                    vt2 = metamodelica::Own::own(__pa7);
                    io2 = metamodelica::Own::own(__pa8);
                    vis2 = metamodelica::Own::own(__pa9);
                    c1_prefix = AbsynUtil::crefStripLast(metamodelica::AsArg::as_arg(&c1))?;
                    let (__pa11, __pa12) = ::match_deref::match_deref! { match &(Static::elabCref(cache.clone(), env.clone(), c1_prefix.clone(), r#impl.clone(), false, pre.clone(), info.clone())?) {
                        (__pa11, Some((Deref @ DAE::Exp::CREF { componentRef: __pa12, ty: _ }, _, _))) => (__pa11.clone(), __pa12.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    cache = metamodelica::Own::own(__pa11);
                    c1_1 = metamodelica::Own::own(__pa12);
                    (cache, c1_2) = Static::canonCref(cache.clone(), env.clone(), c1_1.clone(), r#impl.clone())?;
                    (attr1, ty1, _, _) = Lookup::lookupConnectorVar(metamodelica::AsArg::as_arg(&env), &c1_2, true)?;
                    let true = (Types::isExpandableConnector(&ty1)) else { return Err("pattern mismatch") };
                    c1_2 = ComponentReferenceBasics::crefStripLastSubs(&c1_2)?;
                    (_, attr, ty, binding, cnstForRange, _, _, envExpandable, _) = Lookup::lookupVar(cache.clone(), env.clone(), c1_2.clone())?;
                    (_, _, _, _, _, _, _, envComponent, _) = Lookup::lookupVar(cache.clone(), env.clone(), c2_2.clone())?;
                    variablesUnion = FGraph::getVariablesFromGraphScope(&envComponent)?;
                    let false = (((variablesUnion).len() as i32) > 1) else { return Err("pattern mismatch") };
                    componentName = AbsynUtil::crefGetLastIdent(metamodelica::AsArg::as_arg(&c1))?;
                    envComponentEmpty = FGraph::removeComponentsFromScope(envComponent.clone())?;
                    daeDims = TypesDump::getDimensions(&ty2);
                    arrDims = List::map(daeDims.clone(), &move |__a0: metamodelica::Ref<DAE::Dimension>| Expression::unelabDimension(&__a0))?;
                    envExpandable = FGraph::mkComponentNode(envExpandable.clone(), metamodelica::Ref::new(DAE::Var { name: componentName.clone(), attributes: metamodelica::Ref::new(DAE::Attributes { connectorType: ct2.clone(), parallelism: prl2, variability: vt2, direction: openmodelica_ast::Absyn::Direction::BIDIR, innerOuter: io2, visibility: vis2 }), ty: ty2.clone(), binding: openmodelica_frontend_types::DAE::Binding::interned_UNBOUND(), bind_from_outside: false, constOfForIteratorRange: None }), metamodelica::Ref::new(SCode::Element::COMPONENT { name: componentName.clone(), prefixes: SCode::defaultPrefixes.clone(), attributes: SCode::Attributes { arrayDims: arrDims.clone(), connectorType: openmodelica_frontend_types::SCode::ConnectorType::POTENTIAL, parallelism: openmodelica_frontend_types::SCode::Parallelism::NON_PARALLEL, variability: openmodelica_frontend_types::SCode::Variability::VAR, direction: openmodelica_ast::Absyn::Direction::BIDIR, isField: openmodelica_ast::Absyn::IsField::NONFIELD }, typeSpec: metamodelica::Ref::new(Absyn::TypeSpec::TPATH { path: metamodelica::Ref::new(Absyn::Path::IDENT { name: literal!("") }), arrayDim: None }), modifications: openmodelica_frontend_types::SCode::Mod::interned_NOMOD(), comment: SCode::noComment.clone(), condition: None, info: Absyn::dummyInfo.clone() }), openmodelica_frontend_types::DAE::Mod::interned_NOMOD(), openmodelica_frontend_dump::FCore::Status::VAR_TYPED, envComponentEmpty.clone())?;
                    env = updateEnvComponentsOnQualPath(cache.clone(), env.clone(), c1_2.clone(), attr.clone(), ty.clone(), binding.clone(), cnstForRange.clone(), envExpandable.clone())?;
                    let (__pa14, __pa15) = ::match_deref::match_deref! { match &(Static::elabCref(cache.clone(), env.clone(), c1.clone(), r#impl.clone(), false, pre.clone(), info.clone())?) {
                        (__pa14, Some((Deref @ DAE::Exp::CREF { componentRef: __pa15, ty: _ }, _, _))) => (__pa14.clone(), __pa15.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    cache = metamodelica::Own::own(__pa14);
                    c1_1 = metamodelica::Own::own(__pa15);
                    (cache, c1_2) = Static::canonCref(cache.clone(), env.clone(), c1_1.clone(), r#impl.clone())?;
                    (attr1, ty1, _, _) = Lookup::lookupConnectorVar(metamodelica::AsArg::as_arg(&env), &c1_2, true)?;
                    let __arc22 = attr1.clone();
                    let DAE::ATTR { connectorType: __pa17, parallelism: __pa18, variability: __pa19, direction: _, innerOuter: __pa20, visibility: __pa21 } = &*__arc22;
                    ct1 = metamodelica::Own::own(__pa17);
                    prl1 = metamodelica::Own::own(__pa18);
                    vt1 = metamodelica::Own::own(__pa19);
                    io1 = metamodelica::Own::own(__pa20);
                    vis1 = metamodelica::Own::own(__pa21);
                    (cache, env, ih, sets, dae, graph) = instConnect(cache.clone(), env.clone(), ih.clone(), sets.clone(), pre.clone(), c1.clone(), c2.clone(), r#impl.clone(), graph.clone(), info.clone())?;
                    state = ClassInf::State::CONNECTOR { path: metamodelica::Ref::new(Absyn::Path::IDENT { name: literal!("expandable connector") }), isExpandable: true };
                    (cache, c1p) = PrefixUtil::prefixCref(cache.clone(), env.clone(), metamodelica::AsArg::as_arg(&ih), pre.clone(), c1_2.clone())?;
                    (cache, c2p) = PrefixUtil::prefixCref(cache.clone(), env.clone(), metamodelica::AsArg::as_arg(&ih), pre.clone(), c2_2.clone())?;
                    source = ElementSource::createElementSource(info.clone(), FGraph::getScopePath(metamodelica::AsArg::as_arg(&env))?, metamodelica::AsArg::as_arg(&pre), (c1p.clone(), c2p.clone()));
                    (cache, c1_2) = PrefixUtil::prefixCref(cache.clone(), env.clone(), metamodelica::AsArg::as_arg(&ih), pre.clone(), c1_2.clone())?;
                    daeDims = TypesDump::getDimensions(&ty1);
                    arrDims = List::map(daeDims.clone(), &move |__a0: metamodelica::Ref<DAE::Dimension>| Expression::unelabDimension(&__a0))?;
                    daeExpandable = generateExpandableDAE(metamodelica::AsArg::as_arg(&cache), metamodelica::AsArg::as_arg(&env), &envExpandable, c1_2.clone(), state.clone(), ty1.clone(), &(SCode::Attributes { arrayDims: arrDims.clone(), connectorType: DAEUtil::toSCodeConnectorType(&ct1), parallelism: prl1, variability: vt1, direction: openmodelica_ast::Absyn::Direction::BIDIR, isField: openmodelica_ast::Absyn::IsField::NONFIELD }), vis1, io1, &source)?;
                    dae = DAEUtil::joinDaes(&dae, &daeExpandable)?;
                    Ok((cache.clone(), env.clone(), ih.clone(), sets.clone(), dae.clone(), graph.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, _, _, pre, c1, c2, r#impl, _) => {
                    let mut c1_1: metamodelica::Ref<DAE::ComponentRef>;
                    let mut c2_1: metamodelica::Ref<DAE::ComponentRef>;
                    let mut c1_2: metamodelica::Ref<DAE::ComponentRef>;
                    let mut c2_2: metamodelica::Ref<DAE::ComponentRef>;
                    let mut ty1: metamodelica::Ref<DAE::Type>;
                    let mut ty2: metamodelica::Ref<DAE::Type>;
                    let mut cache = (*cache).clone();
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(Static::elabCref(cache.clone(), env.clone(), c1.clone(), r#impl.clone(), false, pre.clone(), info.clone())?) {
                        (__pa0, Some((Deref @ DAE::Exp::CREF { componentRef: __pa1, ty: _ }, _, _))) => (__pa0.clone(), __pa1.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    cache = metamodelica::Own::own(__pa0);
                    c1_1 = metamodelica::Own::own(__pa1);
                    let (__pa3, __pa4) = ::match_deref::match_deref! { match &(Static::elabCref(cache.clone(), env.clone(), c2.clone(), r#impl.clone(), false, pre.clone(), info.clone())?) {
                        (__pa3, Some((Deref @ DAE::Exp::CREF { componentRef: __pa4, ty: _ }, _, _))) => (__pa3.clone(), __pa4.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    cache = metamodelica::Own::own(__pa3);
                    c2_1 = metamodelica::Own::own(__pa4);
                    (cache, c1_2) = Static::canonCref(cache.clone(), env.clone(), c1_1.clone(), r#impl.clone())?;
                    (cache, c2_2) = Static::canonCref(cache.clone(), env.clone(), c2_1.clone(), r#impl.clone())?;
                    (_, ty1, _, _) = Lookup::lookupConnectorVar(metamodelica::AsArg::as_arg(&env), &c1_2, true)?;
                    (_, ty2, _, _) = Lookup::lookupConnectorVar(metamodelica::AsArg::as_arg(&env), &c2_2, true)?;
                    let false = (Types::isExpandableConnector(&ty1)) else { return Err("pattern mismatch") };
                    let false = (Types::isExpandableConnector(&ty2)) else { return Err("pattern mismatch") };
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((outCache, outEnv, outIH, outSets, outDae, outGraph))
}

fn generateExpandableDAE(
    mut inCache: &FCore::Cache,
    mut inParentEnv: &FCore::Graph,
    mut inClassEnv: &FCore::Graph,
    mut cref: metamodelica::Ref<DAE::ComponentRef>,
    mut state: ClassInf::State,
    mut ty: metamodelica::Ref<DAE::Type>,
    mut attrs: &SCode::Attributes,
    mut vis: SCode::Visibility,
    mut io: Absyn::InnerOuter,
    mut source: &metamodelica::Ref<DAE::ElementSource>,
) -> Result<DAE::DAElist> {
    let mut outDAE: DAE::DAElist;
    outDAE = (match &**source {
        _ => {
            let mut daeDims: metamodelica::List<metamodelica::Ref<DAE::Dimension>>;
            let mut daeExpandable: DAE::DAElist;
            let mut crefs: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
            daeDims = TypesDump::getDimensions(&ty);
            if (daeDims).is_empty() {
                daeExpandable = InstDAE::daeDeclare(
                    inCache,
                    inParentEnv,
                    inClassEnv,
                    cref,
                    state,
                    ty,
                    attrs,
                    vis,
                    None,
                    metamodelica::nil(),
                    None,
                    None,
                    Some(metamodelica::Ref::new(SCode::Comment {
                        annotation_: None,
                        comment: Some(literal!("virtual variable in expandable connector")),
                    })),
                    io,
                    openmodelica_frontend_types::SCode::Final::NOT_FINAL,
                    source,
                    true,
                )?;
            } else {
                crefs = ComponentReference::expandCref(&cref, false)?;
                daeExpandable = daeDeclareList(
                    inCache,
                    inParentEnv,
                    inClassEnv,
                    &(crefs.reverse()),
                    &state,
                    &ty,
                    attrs,
                    vis,
                    io,
                    source,
                    DAE::emptyDae().clone(),
                )?;
            }
            daeExpandable
        }
    });
    Ok(outDAE)
}

fn daeDeclareList<'__b>(
    mut inCache: &'__b FCore::Cache,
    mut inParentEnv: &'__b FCore::Graph,
    mut inClassEnv: &'__b FCore::Graph,
    mut crefs: &'__b metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
    mut state: &'__b ClassInf::State,
    mut ty: &'__b metamodelica::Ref<DAE::Type>,
    mut attrs: &'__b SCode::Attributes,
    mut vis: SCode::Visibility,
    mut io: Absyn::InnerOuter,
    mut source: &'__b metamodelica::Ref<DAE::ElementSource>,
    mut acc: DAE::DAElist,
) -> Result<DAE::DAElist> {
    '__tco: loop {
        ::match_deref::match_deref! { match crefs {
            Deref @ metamodelica::ListNode::Nil => {
                return Ok(acc)
            },
            Deref @ metamodelica::ListNode::Cons { head: cref, tail: lst } => {
                let mut daeExpandable: DAE::DAElist;
                daeExpandable = InstDAE::daeDeclare(inCache, inParentEnv, inClassEnv, cref.clone(), state.clone(), ty.clone(), attrs, vis, None, metamodelica::nil(), None, None, Some(metamodelica::Ref::new(SCode::Comment { annotation_: None, comment: Some(literal!("virtual variable in expandable connector")) })), io, openmodelica_frontend_types::SCode::Final::NOT_FINAL, source, true)?;
                daeExpandable = DAEUtil::joinDaes(&daeExpandable, &acc)?;
                { (inCache, inParentEnv, inClassEnv, crefs, state, ty, attrs, vis, io, source, acc) = (inCache, inParentEnv, inClassEnv, lst, state, ty, attrs, vis, io, source, daeExpandable); continue '__tco; }
            },
            _ => return Err("match: no arm matched"),
        } }
    }
}

fn updateEnvComponentsOnQualPath(
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
    mut virtualExpandableCref: metamodelica::Ref<DAE::ComponentRef>,
    mut virtualExpandableAttr: metamodelica::Ref<DAE::Attributes>,
    mut virtualExpandableTy: metamodelica::Ref<DAE::Type>,
    mut virtualExpandableBinding: metamodelica::Ref<DAE::Binding>,
    mut virtualExpandableCnstForRange: Option<DAE::Const>,
    mut virtualExpandableEnv: FCore::Graph,
) -> Result<FCore::Graph> {
    '__tco: loop {
        ::match_deref::match_deref! { match &((inCache, inEnv, virtualExpandableCref, virtualExpandableAttr, virtualExpandableTy, virtualExpandableBinding, virtualExpandableCnstForRange, virtualExpandableEnv)) {
            (_, topEnv, Deref @ DAE::ComponentRef::CREF_IDENT { ident: currentName, .. }, veAttr, veTy, veBinding, veCnstForRange, veEnv) => {
                let mut updatedEnv: FCore::Graph;
                let mut realEnv: FCore::Graph;
                let mut forLoopScope: metamodelica::List<Mutable::Mutable<metamodelica::Ref<FCore::Node>>>;
                (realEnv, forLoopScope) = FGraph::splitGraphScope(metamodelica::AsArg::as_arg(&topEnv))?;
                updatedEnv = FGraph::updateComp(realEnv, metamodelica::Ref::new(DAE::Var { name: currentName.clone(), attributes: veAttr.clone(), ty: veTy.clone(), binding: veBinding.clone(), bind_from_outside: false, constOfForIteratorRange: veCnstForRange.clone() }), &(openmodelica_frontend_dump::FCore::Status::VAR_TYPED), metamodelica::AsArg::as_arg(&veEnv));
                return Ok(FGraph::pushScope(updatedEnv, forLoopScope)?)
            },
            (cache, topEnv, veCref @ Deref @ DAE::ComponentRef::CREF_QUAL { .. }, veAttr, veTy, veBinding, veCnstForRange, veEnv) => {
                let mut qualCref: metamodelica::Ref<DAE::ComponentRef>;
                let mut currentAttr: metamodelica::Ref<DAE::Attributes>;
                let mut currentTy: metamodelica::Ref<DAE::Type>;
                let mut currentBinding: metamodelica::Ref<DAE::Binding>;
                let mut currentCnstForRange: Option<DAE::Const>;
                let mut updatedEnv: FCore::Graph;
                let mut currentEnv: FCore::Graph;
                let mut realEnv: FCore::Graph;
                let mut forLoopScope: metamodelica::List<Mutable::Mutable<metamodelica::Ref<FCore::Node>>>;
                let mut currentName: ArcStr;
                currentName = ComponentReferenceBasics::crefLastIdent(metamodelica::AsArg::as_arg(&veCref))?;
                qualCref = ComponentReference::crefStripLastIdent(metamodelica::AsArg::as_arg(&veCref))?;
                qualCref = ComponentReferenceBasics::crefStripLastSubs(&qualCref)?;
                (_, currentAttr, currentTy, currentBinding, currentCnstForRange, _, _, currentEnv, _) = Lookup::lookupVar(cache.clone(), topEnv.clone(), qualCref.clone())?;
                (realEnv, forLoopScope) = FGraph::splitGraphScope(&currentEnv)?;
                currentEnv = FGraph::updateComp(realEnv, metamodelica::Ref::new(DAE::Var { name: currentName, attributes: veAttr.clone(), ty: veTy.clone(), binding: veBinding.clone(), bind_from_outside: false, constOfForIteratorRange: veCnstForRange.clone() }), &(openmodelica_frontend_dump::FCore::Status::VAR_TYPED), metamodelica::AsArg::as_arg(&veEnv));
                currentEnv = FGraph::pushScope(currentEnv, forLoopScope)?;
                { (inCache, inEnv, virtualExpandableCref, virtualExpandableAttr, virtualExpandableTy, virtualExpandableBinding, virtualExpandableCnstForRange, virtualExpandableEnv) = (cache.clone(), topEnv.clone(), qualCref, currentAttr, currentTy, currentBinding, currentCnstForRange, currentEnv); continue '__tco; }
            },
            _ => return Err("match: no arm matched"),
        } }
    }
}

fn connectExpandableVariables(
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
    mut inIH: metamodelica::List<InnerOuter::TopInstance>,
    mut inSets: DAE::Connect::Sets,
    mut inPrefix: DAE::Prefix,
    mut inComponentRefLeft: metamodelica::Ref<Absyn::ComponentRef>,
    mut inComponentRefRight: metamodelica::Ref<Absyn::ComponentRef>,
    mut inVariablesUnion: &metamodelica::List<ArcStr>,
    mut inImpl: bool,
    mut inGraph: ConnectionGraph::ConnectionGraph,
    mut info: &SourceInfo,
) -> Result<(
    FCore::Cache,
    FCore::Graph,
    metamodelica::List<InnerOuter::TopInstance>,
    DAE::Connect::Sets,
    DAE::DAElist,
    ConnectionGraph::ConnectionGraph,
)> {
    let mut outCache: FCore::Cache;
    let mut outEnv: FCore::Graph;
    let mut outIH: metamodelica::List<InnerOuter::TopInstance>;
    let mut outSets: DAE::Connect::Sets;
    let mut outDae: DAE::DAElist;
    let mut outGraph: ConnectionGraph::ConnectionGraph;
    (outCache, outEnv, outIH, outSets, outDae, outGraph) = (::match_deref::match_deref! { match inVariablesUnion {
        Deref @ metamodelica::ListNode::Nil => {
            let mut cache = inCache;
            let mut env = inEnv;
            let mut ih = inIH;
            let mut sets = inSets;
            let mut graph = inGraph;
            (cache, env, ih, sets, DAE::emptyDae().clone(), graph)
        },
        Deref @ metamodelica::ListNode::Cons { head: name, tail: names } => {
            let mut cache = inCache;
            let mut env = inEnv;
            let mut ih = inIH;
            let mut sets = inSets;
            let mut pre = inPrefix;
            let mut c1 = inComponentRefLeft;
            let mut c2 = inComponentRefRight;
            let mut r#impl = inImpl;
            let mut graph = inGraph;
            let mut dae: DAE::DAElist;
            let mut dae1: DAE::DAElist;
            let mut dae2: DAE::DAElist;
            let mut c1_full: metamodelica::Ref<Absyn::ComponentRef>;
            let mut c2_full: metamodelica::Ref<Absyn::ComponentRef>;
            c1_full = AbsynUtil::joinCrefs(&c1, metamodelica::Ref::new(Absyn::ComponentRef::CREF_IDENT { name: name.clone(), subscripts: metamodelica::nil() }))?;
            c2_full = AbsynUtil::joinCrefs(&c2, metamodelica::Ref::new(Absyn::ComponentRef::CREF_IDENT { name: name.clone(), subscripts: metamodelica::nil() }))?;
            (cache, env, ih, sets, dae1, graph) = instConnect(cache, env, ih, sets, pre.clone(), c1_full, c2_full, r#impl, graph, info.clone())?;
            (cache, env, ih, sets, dae2, graph) = connectExpandableVariables(cache, env, ih, sets, pre, c1, c2, names, r#impl, graph, info)?;
            dae = DAEUtil::joinDaes(&dae1, &dae2)?;
            (cache, env, ih, sets, dae, graph)
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outCache, outEnv, outIH, outSets, outDae, outGraph))
}

fn getStateFromType(mut ty: &metamodelica::Ref<DAE::Type>) -> Result<ClassInf::State> {
    let mut outState: ClassInf::State;
    outState = (match &**ty {
        DAE::Type::T_COMPLEX {
            complexClassType: state,
            ..
        } => state.clone(),
        DAE::Type::T_SUBTYPE_BASIC {
            complexClassType: state,
            ..
        } => state.clone(),
        _ => return Err("fail"),
    });
    Ok(outState)
}

fn isConnectorType(mut ty: &metamodelica::Ref<DAE::Type>) -> bool {
    let mut isConnector: bool;
    isConnector = (match &**ty {
        DAE::Type::T_COMPLEX {
            complexClassType:
                ClassInf::State::CONNECTOR {
                    path: _,
                    isExpandable: false,
                },
            ..
        } => true,
        DAE::Type::T_SUBTYPE_BASIC {
            complexClassType:
                ClassInf::State::CONNECTOR {
                    path: _,
                    isExpandable: false,
                },
            ..
        } => true,
        _ => false,
    });
    isConnector
}

fn flipDirection(mut inDir: Absyn::Direction) -> Result<Absyn::Direction> {
    let mut outDir: Absyn::Direction;
    outDir = (match inDir {
        Absyn::Direction::INPUT { .. } => openmodelica_ast::Absyn::Direction::OUTPUT,
        Absyn::Direction::OUTPUT { .. } => openmodelica_ast::Absyn::Direction::INPUT,
        Absyn::Direction::BIDIR { .. } => openmodelica_ast::Absyn::Direction::BIDIR,
        _ => return Err("match: no arm matched"),
    });
    Ok(outDir)
}

fn validConnector(
    mut inType: &metamodelica::Ref<DAE::Type>,
    mut inCref: &metamodelica::Ref<DAE::ComponentRef>,
    mut inInfo: &SourceInfo,
) -> Result<()> {
    let () = 'mc: {
        let __mc_input = &**inType;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Type::T_REAL { .. } => {
                    Ok(())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Type::T_INTEGER { .. } => {
                    Ok(())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Type::T_STRING { .. } => {
                    Ok(())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Type::T_BOOL { .. } => {
                    Ok(())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Type::T_ENUMERATION { .. } => {
                    Ok(())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Type::T_CLOCK { .. } => {
                    Ok(())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Type::T_COMPLEX { complexClassType: state, .. } => {
                    ClassInfUtil::valid(metamodelica::AsArg::as_arg(&state), &(SCode::Restriction::R_CONNECTOR { isExpandable: false }))?;
                    Ok(())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Type::T_COMPLEX { complexClassType: state, .. } => {
                    ClassInfUtil::valid(metamodelica::AsArg::as_arg(&state), &(SCode::Restriction::R_CONNECTOR { isExpandable: true }))?;
                    Ok(())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Type::T_SUBTYPE_BASIC { complexClassType: state, .. } => {
                    ClassInfUtil::valid(metamodelica::AsArg::as_arg(&state), &(SCode::Restriction::R_CONNECTOR { isExpandable: false }))?;
                    Ok(())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Type::T_SUBTYPE_BASIC { complexClassType: state, .. } => {
                    ClassInfUtil::valid(metamodelica::AsArg::as_arg(&state), &(SCode::Restriction::R_CONNECTOR { isExpandable: true }))?;
                    Ok(())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Type::T_ARRAY { ty: tp, .. } => {
                    validConnector(metamodelica::AsArg::as_arg(&tp), inCref, inInfo)?;
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
                    let true = (ConnectUtil::isExpandable(inCref)) else { return Err("pattern mismatch") };
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
                    let mut r#str: ArcStr;
                    r#str = ComponentReferenceBasics::printComponentRefStr(inCref)?;
                    Error::addSourceMessage(&(Error::INVALID_CONNECTOR_TYPE.clone()), list![r#str.clone()], inInfo)?;
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

fn checkConnectTypes(
    mut inLhsCref: &metamodelica::Ref<DAE::ComponentRef>,
    mut inLhsType: metamodelica::Ref<DAE::Type>,
    mut inLhsFace: DAE::Connect::Face,
    mut inLhsAttributes: &metamodelica::Ref<DAE::Attributes>,
    mut inRhsCref: &metamodelica::Ref<DAE::ComponentRef>,
    mut inRhsType: metamodelica::Ref<DAE::Type>,
    mut inRhsFace: DAE::Connect::Face,
    mut inRhsAttributes: &metamodelica::Ref<DAE::Attributes>,
    mut inInfo: &SourceInfo,
) -> Result<()> {
    let mut lhs_ct: metamodelica::Ref<DAE::ConnectorType>;
    let mut rhs_ct: metamodelica::Ref<DAE::ConnectorType>;
    let mut lhs_dir: Absyn::Direction;
    let mut rhs_dir: Absyn::Direction;
    let mut lhs_io: Absyn::InnerOuter;
    let mut rhs_io: Absyn::InnerOuter;
    let mut lhs_vis: SCode::Visibility;
    let mut rhs_vis: SCode::Visibility;
    ComponentReference::checkCrefSubscriptsBounds(inLhsCref, inInfo)?;
    ComponentReference::checkCrefSubscriptsBounds(inRhsCref, inInfo)?;
    let __arc4 = &(*inLhsAttributes);
    let DAE::ATTR {
        connectorType: __pa0,
        direction: __pa1,
        innerOuter: __pa2,
        visibility: __pa3,
        ..
    } = &**__arc4;
    lhs_ct = metamodelica::Own::own(__pa0);
    lhs_dir = metamodelica::Own::own(__pa1);
    lhs_io = metamodelica::Own::own(__pa2);
    lhs_vis = metamodelica::Own::own(__pa3);
    let __arc9 = &(*inRhsAttributes);
    let DAE::ATTR {
        connectorType: __pa5,
        direction: __pa6,
        innerOuter: __pa7,
        visibility: __pa8,
        ..
    } = &**__arc9;
    rhs_ct = metamodelica::Own::own(__pa5);
    rhs_dir = metamodelica::Own::own(__pa6);
    rhs_io = metamodelica::Own::own(__pa7);
    rhs_vis = metamodelica::Own::own(__pa8);
    checkConnectTypesType(inLhsType, inRhsType, inLhsCref, inRhsCref, inInfo)?;
    checkConnectTypesFlowStream(&lhs_ct, &rhs_ct, inLhsCref, inRhsCref, inInfo)?;
    checkConnectTypesDirection(
        lhs_dir, inLhsFace, lhs_vis, rhs_dir, inRhsFace, rhs_vis, inLhsCref, inRhsCref, inInfo,
    )?;
    checkConnectTypesInnerOuter(lhs_io, rhs_io, inLhsCref, inRhsCref, inInfo)?;
    Ok(())
}

fn checkConnectTypesType(
    mut inLhsType: metamodelica::Ref<DAE::Type>,
    mut inRhsType: metamodelica::Ref<DAE::Type>,
    mut inLhsCref: &metamodelica::Ref<DAE::ComponentRef>,
    mut inRhsCref: &metamodelica::Ref<DAE::ComponentRef>,
    mut inInfo: &SourceInfo,
) -> Result<()> {
    let () = 'mc: {
        let __mc_input = inInfo.clone();
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            let true = (Types::equivtypesOrRecordSubtypeOf(inLhsType.clone(), inRhsType.clone())) else {
                return Err("pattern mismatch");
            };
            Ok(())
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            let mut t1: metamodelica::Ref<DAE::Type>;
            let mut t2: metamodelica::Ref<DAE::Type>;
            let mut cs1: ArcStr;
            let mut cs2: ArcStr;
            let mut cref_str1: ArcStr;
            let mut cref_str2: ArcStr;
            t1 = Types::arrayElementType(&inLhsType);
            t2 = Types::arrayElementType(&inRhsType);
            let false = (Types::equivtypesOrRecordSubtypeOf(t1.clone(), t2.clone())) else {
                return Err("pattern mismatch");
            };
            (_, cs1) = TypesDump::printConnectorTypeStr(t1.clone())?;
            (_, cs2) = TypesDump::printConnectorTypeStr(t2.clone())?;
            cref_str1 = ComponentReferenceBasics::printComponentRefStr(inLhsCref)?;
            cref_str2 = ComponentReferenceBasics::printComponentRefStr(inRhsCref)?;
            Error::addSourceMessage(
                &(Error::CONNECT_INCOMPATIBLE_TYPES.clone()),
                list![
                    cref_str1.clone(),
                    cref_str2.clone(),
                    cref_str1.clone(),
                    cs1.clone(),
                    cref_str2.clone(),
                    cs2.clone()
                ],
                inInfo,
            )?;
            Ok(return Err("fail"))
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            let mut cref_str1: ArcStr;
            let mut cref_str2: ArcStr;
            let mut str1: ArcStr;
            let mut str2: ArcStr;
            let mut dims1: metamodelica::List<metamodelica::Ref<DAE::Dimension>>;
            let mut dims2: metamodelica::List<metamodelica::Ref<DAE::Dimension>>;
            dims1 = TypesDump::getDimensions(&inLhsType);
            dims2 = TypesDump::getDimensions(&inRhsType);
            let false = (List::isEqualOnTrue(dims1.clone(), dims2.clone(), &move |__a0: metamodelica::Ref<
                DAE::Dimension,
            >,
                                                                                  __a1: metamodelica::Ref<
                DAE::Dimension,
            >| {
                Expression::dimensionsEqual(&__a0, &__a1)
            })?) else {
                return Err("pattern mismatch");
            };
            let false = ((dims1).is_empty() && (dims2).is_empty()) else {
                return Err("pattern mismatch");
            };
            cref_str1 = ComponentReferenceBasics::printComponentRefStr(inLhsCref)?;
            cref_str2 = ComponentReferenceBasics::printComponentRefStr(inRhsCref)?;
            str1 = {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("["));
                __mm_s.push_str(&*ExpressionBasics::dimensionsString(dims1.clone())?);
                __mm_s.push_str(&*literal!("]"));
                ArcStr::from(__mm_s)
            };
            str2 = {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("["));
                __mm_s.push_str(&*ExpressionBasics::dimensionsString(dims2.clone())?);
                __mm_s.push_str(&*literal!("]"));
                ArcStr::from(__mm_s)
            };
            Error::addSourceMessage(
                &(Error::CONNECTOR_ARRAY_DIFFERENT.clone()),
                list![cref_str1.clone(), cref_str2.clone(), str1.clone(), str2.clone()],
                inInfo,
            )?;
            Ok(return Err("fail"))
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(())
}

fn checkConnectTypesFlowStream(
    mut inLhsConnectorType: &metamodelica::Ref<DAE::ConnectorType>,
    mut inRhsConnectorType: &metamodelica::Ref<DAE::ConnectorType>,
    mut inLhsCref: &metamodelica::Ref<DAE::ComponentRef>,
    mut inRhsCref: &metamodelica::Ref<DAE::ComponentRef>,
    mut inInfo: &SourceInfo,
) -> Result<()> {
    let () = 'mc: {
        let __mc_input = inInfo.clone();
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            let true = (DAEUtil::connectorTypeEqual(inLhsConnectorType, inRhsConnectorType)?) else {
                return Err("pattern mismatch");
            };
            Ok(())
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            let mut cref_str1: ArcStr;
            let mut cref_str2: ArcStr;
            let mut pre_str1: ArcStr;
            let mut pre_str2: ArcStr;
            let mut err_strl: metamodelica::List<ArcStr>;
            cref_str1 = ComponentReferenceBasics::printComponentRefStr(inLhsCref)?;
            cref_str2 = ComponentReferenceBasics::printComponentRefStr(inRhsCref)?;
            pre_str1 = DAEUtil::connectorTypeStr(inLhsConnectorType)?;
            pre_str2 = DAEUtil::connectorTypeStr(inRhsConnectorType)?;
            err_strl = if (DAEUtil::potentialBool(inLhsConnectorType)) {
                list![pre_str2.clone(), cref_str2.clone(), cref_str1.clone()]
            } else {
                list![pre_str1.clone(), cref_str1.clone(), cref_str2.clone()]
            };
            Error::addSourceMessage(&(Error::CONNECT_PREFIX_MISMATCH.clone()), err_strl.clone(), inInfo)?;
            Ok(return Err("fail"))
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(())
}

fn checkConnectTypesDirection(
    mut inLhsDirection: Absyn::Direction,
    mut inLhsFace: DAE::Connect::Face,
    mut inLhsVisibility: SCode::Visibility,
    mut inRhsDirection: Absyn::Direction,
    mut inRhsFace: DAE::Connect::Face,
    mut inRhsVisibility: SCode::Visibility,
    mut inLhsCref: &metamodelica::Ref<DAE::ComponentRef>,
    mut inRhsCref: &metamodelica::Ref<DAE::ComponentRef>,
    mut inInfo: &SourceInfo,
) -> Result<()> {
    if isSignalSource(inLhsDirection, inLhsFace, inLhsVisibility)
        && isSignalSource(inRhsDirection, inRhsFace, inRhsVisibility)
    {
        Error::addSourceMessage(
            &(Error::CONNECT_TWO_SOURCES.clone()),
            list![
                ComponentReferenceBasics::printComponentRefStr(inLhsCref)?,
                ComponentReferenceBasics::printComponentRefStr(inRhsCref)?
            ],
            inInfo,
        )?;
    }
    Ok(())
}

fn isSignalSource(
    mut inDirection: Absyn::Direction,
    mut inFace: DAE::Connect::Face,
    mut inVisibility: SCode::Visibility,
) -> bool {
    let mut outIsSignal: bool;
    outIsSignal = (match (inDirection, inFace, inVisibility) {
        (Absyn::Direction::OUTPUT { .. }, DAE::Connect::Face::INSIDE, _) => true,
        (Absyn::Direction::INPUT { .. }, DAE::Connect::Face::OUTSIDE, SCode::Visibility::PUBLIC { .. }) => true,
        _ => false,
    });
    outIsSignal
}

fn checkConnectTypesInnerOuter(
    mut inLhsIO: Absyn::InnerOuter,
    mut inRhsIO: Absyn::InnerOuter,
    mut inLhsCref: &metamodelica::Ref<DAE::ComponentRef>,
    mut inRhsCref: &metamodelica::Ref<DAE::ComponentRef>,
    mut inInfo: &SourceInfo,
) -> Result<()> {
    let () = (match (inLhsIO, inRhsIO) {
        (Absyn::InnerOuter::OUTER { .. }, Absyn::InnerOuter::OUTER { .. }) => {
            let mut cref_str1: ArcStr;
            let mut cref_str2: ArcStr;
            cref_str1 = ComponentReferenceBasics::printComponentRefStr(inLhsCref)?;
            cref_str2 = ComponentReferenceBasics::printComponentRefStr(inRhsCref)?;
            Error::addSourceMessage(
                &(Error::CONNECT_OUTER_OUTER.clone()),
                list![cref_str1, cref_str2],
                inInfo,
            )?;
            return Err("fail");
        }
        _ => (),
    });
    Ok(())
}

pub(crate) fn connectComponents(
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
    mut inIH: metamodelica::List<InnerOuter::TopInstance>,
    mut inSets: DAE::Connect::Sets,
    mut inPrefix3: DAE::Prefix,
    mut cr1: metamodelica::Ref<DAE::ComponentRef>,
    mut inFace5: DAE::Connect::Face,
    mut inType6: metamodelica::Ref<DAE::Type>,
    mut vt1: SCode::Variability,
    mut cr2: metamodelica::Ref<DAE::ComponentRef>,
    mut inFace8: DAE::Connect::Face,
    mut inType9: metamodelica::Ref<DAE::Type>,
    mut vt2: SCode::Variability,
    mut inConnectorType: metamodelica::Ref<DAE::ConnectorType>,
    mut io1: Absyn::InnerOuter,
    mut io2: Absyn::InnerOuter,
    mut inGraph: ConnectionGraph::ConnectionGraph,
    mut info: &SourceInfo,
) -> Result<(
    FCore::Cache,
    FCore::Graph,
    metamodelica::List<InnerOuter::TopInstance>,
    DAE::Connect::Sets,
    DAE::DAElist,
    ConnectionGraph::ConnectionGraph,
)> {
    let mut outCache: FCore::Cache;
    let mut outEnv: FCore::Graph;
    let mut outIH: metamodelica::List<InnerOuter::TopInstance>;
    let mut outSets: DAE::Connect::Sets;
    let mut outDae: DAE::DAElist;
    let mut outGraph: ConnectionGraph::ConnectionGraph;
    (outCache, outEnv, outIH, outSets, outDae, outGraph) = 'mc: {
        let __mc_input = (
            inCache,
            inEnv,
            inIH,
            inSets,
            inPrefix3,
            cr1,
            inFace5,
            inType6,
            cr2,
            inFace8,
            inType9,
            inConnectorType.clone(),
            inGraph,
        );
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, ih, sets, pre, c1, f1, _, c2, f2, _, ct, graph) => {
                    let mut c1_1: metamodelica::Ref<DAE::ComponentRef>;
                    let mut c2_1: metamodelica::Ref<DAE::ComponentRef>;
                    let mut source: metamodelica::Ref<DAE::ElementSource>;
                    let mut cache = (*cache).clone();
                    let mut sets = (*sets).clone();
                    let false = (DAEUtil::streamBool(metamodelica::AsArg::as_arg(&ct))) else { return Err("pattern mismatch") };
                    let true = (InnerOuter::outerConnection(io1, io2)) else { return Err("pattern mismatch") };
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(PrefixUtil::prefixExp(cache.clone(), metamodelica::AsArg::as_arg(&env), metamodelica::AsArg::as_arg(&ih), Expression::crefExp(c1.clone())?, metamodelica::AsArg::as_arg(&pre))?) {
                        (__pa0, Deref @ DAE::Exp::CREF { componentRef: __pa1, ty: _ }) => (__pa0.clone(), __pa1.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    cache = metamodelica::Own::own(__pa0);
                    c1_1 = metamodelica::Own::own(__pa1);
                    let (__pa3, __pa4) = ::match_deref::match_deref! { match &(PrefixUtil::prefixExp(cache.clone(), metamodelica::AsArg::as_arg(&env), metamodelica::AsArg::as_arg(&ih), Expression::crefExp(c2.clone())?, metamodelica::AsArg::as_arg(&pre))?) {
                        (__pa3, Deref @ DAE::Exp::CREF { componentRef: __pa4, ty: _ }) => (__pa3.clone(), __pa4.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    cache = metamodelica::Own::own(__pa3);
                    c2_1 = metamodelica::Own::own(__pa4);
                    source = ElementSource::createElementSource(info.clone(), FGraph::getScopePath(metamodelica::AsArg::as_arg(&env))?, metamodelica::AsArg::as_arg(&pre), (c1_1.clone(), c2_1.clone()));
                    sets = ConnectUtil::addOuterConnection(pre.clone(), sets.clone(), c1_1.clone(), c2_1.clone(), io1, io2, f1.clone(), f2.clone(), source.clone())?;
                    Ok((cache.clone(), env.clone(), ih.clone(), sets.clone(), DAE::emptyDae().clone(), graph.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, ih, sets, pre, c1, _, t1, c2, _, t2, Deref @ DAE::ConnectorType::POTENTIAL { .. }, graph) => {
                    let mut c1_1: metamodelica::Ref<DAE::ComponentRef>;
                    let mut c2_1: metamodelica::Ref<DAE::ComponentRef>;
                    let mut source: metamodelica::Ref<DAE::ElementSource>;
                    let mut crefExp1: metamodelica::Ref<DAE::Exp>;
                    let mut crefExp2: metamodelica::Ref<DAE::Exp>;
                    let mut elts: metamodelica::List<metamodelica::Ref<DAE::Element>>;
                    let mut const1: DAE::Const;
                    let mut const2: DAE::Const;
                    let mut lhsl: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                    let mut rhsl: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                    let mut cache = (*cache).clone();
                    let true = (SCodeUtil::isParameterOrConst(vt1) && SCodeUtil::isParameterOrConst(vt2)) else { return Err("pattern mismatch") };
                    let true = (Types::basicType(&(Types::arrayElementType(metamodelica::AsArg::as_arg(&t1))))) else { return Err("pattern mismatch") };
                    let true = (Types::basicType(&(Types::arrayElementType(metamodelica::AsArg::as_arg(&t2))))) else { return Err("pattern mismatch") };
                    (cache, c1_1) = PrefixUtil::prefixCref(cache.clone(), env.clone(), metamodelica::AsArg::as_arg(&ih), pre.clone(), c1.clone())?;
                    (cache, c2_1) = PrefixUtil::prefixCref(cache.clone(), env.clone(), metamodelica::AsArg::as_arg(&ih), pre.clone(), c2.clone())?;
                    source = ElementSource::createElementSource(info.clone(), FGraph::getScopePath(metamodelica::AsArg::as_arg(&env))?, metamodelica::AsArg::as_arg(&pre), (c1_1.clone(), c2_1.clone()));
                    crefExp1 = Expression::crefExp(c1_1.clone())?;
                    crefExp2 = Expression::crefExp(c2_1.clone())?;
                    const1 = Types::variabilityToConst(vt1);
                    const2 = Types::variabilityToConst(vt2);
                    (cache, crefExp1, _) = Ceval::cevalIfConstant(cache.clone(), env.clone(), crefExp1.clone(), DAE::Properties::PROP { type_: t1.clone(), constFlag: const1 }, true, info.clone())?;
                    (cache, crefExp2, _) = Ceval::cevalIfConstant(cache.clone(), env.clone(), crefExp2.clone(), DAE::Properties::PROP { type_: t2.clone(), constFlag: const2 }, true, info.clone())?;
                    lhsl = Expression::arrayElements(crefExp1.clone())?;
                    rhsl = Expression::arrayElements(crefExp2.clone())?;
                    elts = List::threadMap1(lhsl.clone(), rhsl.clone(), &generateConnectAssert, source.clone())?;
                    Ok((cache.clone(), env.clone(), ih.clone(), sets.clone(), DAE::DAElist { elementLst: elts.clone() }, graph.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, ih, sets, pre, c1, f1, t1, c2, f2, t2, _, graph) => {
                    let mut c1_1: metamodelica::Ref<DAE::ComponentRef>;
                    let mut c2_1: metamodelica::Ref<DAE::ComponentRef>;
                    let mut sets_1: DAE::Connect::Sets;
                    let mut source: metamodelica::Ref<DAE::ElementSource>;
                    let mut cache = (*cache).clone();
                    let true = (Types::basicType(metamodelica::AsArg::as_arg(&t1))) else { return Err("pattern mismatch") };
                    let true = (Types::basicType(metamodelica::AsArg::as_arg(&t2))) else { return Err("pattern mismatch") };
                    (cache, c1_1) = PrefixUtil::prefixCref(cache.clone(), env.clone(), metamodelica::AsArg::as_arg(&ih), pre.clone(), c1.clone())?;
                    (cache, c2_1) = PrefixUtil::prefixCref(cache.clone(), env.clone(), metamodelica::AsArg::as_arg(&ih), pre.clone(), c2.clone())?;
                    source = ElementSource::createElementSource(info.clone(), FGraph::getScopePath(metamodelica::AsArg::as_arg(&env))?, metamodelica::AsArg::as_arg(&pre), (c1_1.clone(), c2_1.clone()));
                    sets_1 = ConnectUtil::addConnection(sets.clone(), c1.clone(), f1.clone(), c2.clone(), f2.clone(), &inConnectorType, source.clone())?;
                    Ok((cache.clone(), env.clone(), ih.clone(), sets_1.clone(), DAE::emptyDae().clone(), graph.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, ih, sets, pre, c1, f1, Deref @ DAE::Type::T_ARRAY { dims: Deref @ metamodelica::ListNode::Cons { head: dim1, tail: Deref @ metamodelica::ListNode::Nil }, ty: t1 }, c2, f2, Deref @ DAE::Type::T_ARRAY { dims: Deref @ metamodelica::ListNode::Cons { head: dim2, tail: Deref @ metamodelica::ListNode::Nil }, ty: t2 }, ct @ Deref @ DAE::ConnectorType::POTENTIAL { .. }, graph) => {
                    let mut sets_1: DAE::Connect::Sets;
                    let mut dae: DAE::DAElist;
                    let mut crefs1: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
                    let mut crefs2: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
                    let mut cache = (*cache).clone();
                    let mut ih = (*ih).clone();
                    let mut graph = (*graph).clone();
                    ::match_deref::match_deref! { match &(Types::arrayElementType(metamodelica::AsArg::as_arg(&t1))) {
                        Deref @ DAE::Type::T_COMPLEX { .. } => (),
                        _ => return Err("pattern mismatch"),
                    } };
                    ::match_deref::match_deref! { match &(Types::arrayElementType(metamodelica::AsArg::as_arg(&t2))) {
                        Deref @ DAE::Type::T_COMPLEX { .. } => (),
                        _ => return Err("pattern mismatch"),
                    } };
                    let true = (Expression::dimensionsKnownAndEqual(metamodelica::AsArg::as_arg(&dim1), metamodelica::AsArg::as_arg(&dim2))?) else { return Err("pattern mismatch") };
                    Expression::dimensionSize(metamodelica::AsArg::as_arg(&dim1))?;
                    crefs1 = ComponentReference::expandCref(metamodelica::AsArg::as_arg(&c1), false)?;
                    crefs2 = ComponentReference::expandCref(metamodelica::AsArg::as_arg(&c2), false)?;
                    (cache, _, ih, sets_1, dae, graph) = connectArrayComponents(metamodelica::AsArg::as_arg(&cache), metamodelica::AsArg::as_arg(&env), metamodelica::AsArg::as_arg(&ih), metamodelica::AsArg::as_arg(&sets), metamodelica::AsArg::as_arg(&pre), &crefs1, f1.clone(), metamodelica::AsArg::as_arg(&t1), vt1, io1, &crefs2, f2.clone(), metamodelica::AsArg::as_arg(&t2), vt2, io2, metamodelica::AsArg::as_arg(&ct), metamodelica::AsArg::as_arg(&graph), info)?;
                    Ok((cache.clone(), env.clone(), ih.clone(), sets_1.clone(), dae.clone(), graph.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, ih, sets, pre, c1, f1, Deref @ DAE::Type::T_ARRAY { dims: Deref @ metamodelica::ListNode::Cons { head: dim1, tail: Deref @ metamodelica::ListNode::Nil }, ty: t1 }, c2, f2, Deref @ DAE::Type::T_ARRAY { dims: Deref @ metamodelica::ListNode::Cons { head: dim2, tail: Deref @ metamodelica::ListNode::Nil }, ty: t2 }, ct @ Deref @ DAE::ConnectorType::POTENTIAL { .. }, graph) => {
                    let mut sets_1: DAE::Connect::Sets;
                    let mut dae: DAE::DAElist;
                    let mut crefs1: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
                    let mut crefs2: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
                    let mut cache = (*cache).clone();
                    let mut ih = (*ih).clone();
                    let mut graph = (*graph).clone();
                    ::match_deref::match_deref! { match &(Types::arrayElementType(metamodelica::AsArg::as_arg(&t1))) {
                        Deref @ DAE::Type::T_SUBTYPE_BASIC { equalityConstraint: Some(_), .. } => (),
                        _ => return Err("pattern mismatch"),
                    } };
                    ::match_deref::match_deref! { match &(Types::arrayElementType(metamodelica::AsArg::as_arg(&t2))) {
                        Deref @ DAE::Type::T_SUBTYPE_BASIC { equalityConstraint: Some(_), .. } => (),
                        _ => return Err("pattern mismatch"),
                    } };
                    let true = (Expression::dimensionsKnownAndEqual(metamodelica::AsArg::as_arg(&dim1), metamodelica::AsArg::as_arg(&dim2))?) else { return Err("pattern mismatch") };
                    Expression::dimensionSize(metamodelica::AsArg::as_arg(&dim1))?;
                    crefs1 = ComponentReference::expandCref(metamodelica::AsArg::as_arg(&c1), false)?;
                    crefs2 = ComponentReference::expandCref(metamodelica::AsArg::as_arg(&c2), false)?;
                    (cache, _, ih, sets_1, dae, graph) = connectArrayComponents(metamodelica::AsArg::as_arg(&cache), metamodelica::AsArg::as_arg(&env), metamodelica::AsArg::as_arg(&ih), metamodelica::AsArg::as_arg(&sets), metamodelica::AsArg::as_arg(&pre), &crefs1, f1.clone(), metamodelica::AsArg::as_arg(&t1), vt1, io1, &crefs2, f2.clone(), metamodelica::AsArg::as_arg(&t2), vt2, io2, metamodelica::AsArg::as_arg(&ct), metamodelica::AsArg::as_arg(&graph), info)?;
                    Ok((cache.clone(), env.clone(), ih.clone(), sets_1.clone(), dae.clone(), graph.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, ih, sets, pre, c1, f1, t1 @ Deref @ DAE::Type::T_ARRAY { .. }, c2, f2, t2 @ Deref @ DAE::Type::T_ARRAY { .. }, ct, graph) => {
                    let mut c1p: metamodelica::Ref<DAE::ComponentRef>;
                    let mut c2p: metamodelica::Ref<DAE::ComponentRef>;
                    let mut sets_1: DAE::Connect::Sets;
                    let mut source: metamodelica::Ref<DAE::ElementSource>;
                    let mut dims: metamodelica::List<metamodelica::Ref<DAE::Dimension>>;
                    let mut dims2: metamodelica::List<metamodelica::Ref<DAE::Dimension>>;
                    let mut cache = (*cache).clone();
                    dims = TypesDump::getDimensions(metamodelica::AsArg::as_arg(&t1));
                    dims2 = TypesDump::getDimensions(metamodelica::AsArg::as_arg(&t2));
                    let true = (List::isEqualOnTrue(dims.clone(), dims2.clone(), &move |__a0: metamodelica::Ref<DAE::Dimension>, __a1: metamodelica::Ref<DAE::Dimension>| Expression::dimensionsKnownAndEqual(&__a0, &__a1))?) else { return Err("pattern mismatch") };
                    (cache, c1p) = PrefixUtil::prefixCref(cache.clone(), env.clone(), metamodelica::AsArg::as_arg(&ih), pre.clone(), c1.clone())?;
                    (cache, c2p) = PrefixUtil::prefixCref(cache.clone(), env.clone(), metamodelica::AsArg::as_arg(&ih), pre.clone(), c2.clone())?;
                    source = ElementSource::createElementSource(info.clone(), FGraph::getScopePath(metamodelica::AsArg::as_arg(&env))?, metamodelica::AsArg::as_arg(&pre), (c1p.clone(), c2p.clone()));
                    sets_1 = ConnectUtil::addArrayConnection(sets.clone(), metamodelica::AsArg::as_arg(&c1), f1.clone(), metamodelica::AsArg::as_arg(&c2), f2.clone(), source.clone(), metamodelica::AsArg::as_arg(&ct))?;
                    Ok((cache.clone(), env.clone(), ih.clone(), sets_1.clone(), DAE::emptyDae().clone(), graph.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, ih, sets, pre, c1, f1, t1 @ Deref @ DAE::Type::T_COMPLEX { equalityConstraint: Some((fpath1, idim1, inlineType1)), .. }, c2, f2, t2 @ Deref @ DAE::Type::T_COMPLEX { equalityConstraint: Some(_), .. }, ct @ Deref @ DAE::ConnectorType::POTENTIAL { .. }, graph @ ConnectionGraph::ConnectionGraph { updateGraph: true, .. }) => {
                    let mut c1_1: metamodelica::Ref<DAE::ComponentRef>;
                    let mut c2_1: metamodelica::Ref<DAE::ComponentRef>;
                    let mut sets_1: DAE::Connect::Sets;
                    let mut equalityConstraintFunctionReturnType: metamodelica::Ref<DAE::Type>;
                    let mut dae: DAE::DAElist;
                    let mut source: metamodelica::Ref<DAE::ElementSource>;
                    let mut zeroVector: metamodelica::Ref<DAE::Exp>;
                    let mut crefExp1: metamodelica::Ref<DAE::Exp>;
                    let mut crefExp2: metamodelica::Ref<DAE::Exp>;
                    let mut breakDAEElements: metamodelica::List<metamodelica::Ref<DAE::Element>>;
                    let mut equalityConstraintFunction: metamodelica::Ref<SCode::Element>;
                    let mut cache = (*cache).clone();
                    let mut env = (*env).clone();
                    let mut ih = (*ih).clone();
                    let mut fpath1 = (*fpath1).clone();
                    let mut graph = (*graph).clone();
                    (cache, c1_1) = PrefixUtil::prefixCref(cache.clone(), env.clone(), metamodelica::AsArg::as_arg(&ih), pre.clone(), c1.clone())?;
                    (cache, c2_1) = PrefixUtil::prefixCref(cache.clone(), env.clone(), metamodelica::AsArg::as_arg(&ih), pre.clone(), c2.clone())?;
                    (cache, env, ih, sets_1, dae, _) = connectComponents(cache.clone(), env.clone(), ih.clone(), sets.clone(), pre.clone(), c1.clone(), f1.clone(), t1.clone(), vt1, c2.clone(), f2.clone(), t2.clone(), vt2, ct.clone(), io1, io2, ConnectionGraph::NOUPDATE_EMPTY().clone(), info)?;
                    source = ElementSource::createElementSource(info.clone(), FGraph::getScopePath(metamodelica::AsArg::as_arg(&env))?, metamodelica::AsArg::as_arg(&pre), (c1_1.clone(), c2_1.clone()));
                    zeroVector = Expression::makeRealArrayOfZeros(idim1.clone());
                    crefExp1 = Expression::crefExp(c1_1.clone())?;
                    crefExp2 = Expression::crefExp(c2_1.clone())?;
                    equalityConstraintFunctionReturnType = metamodelica::Ref::new(DAE::Type::T_ARRAY { ty: DAE::T_REAL_DEFAULT().clone(), dims: list![metamodelica::Ref::new(DAE::Dimension::DIM_INTEGER { integer: idim1.clone() })] });
                    source = ElementSource::addAdditionalComment(&source, literal!(" equation generated by overconstrained connection graph breaking"));
                    breakDAEElements = list![metamodelica::Ref::new(DAE::Element::ARRAY_EQUATION { dimension: list![metamodelica::Ref::new(DAE::Dimension::DIM_INTEGER { integer: idim1.clone() })], exp: zeroVector.clone(), array: metamodelica::Ref::new(DAE::Exp::CALL { path: fpath1.clone(), expLst: list![crefExp1.clone(), crefExp2.clone()], attr: metamodelica::Ref::new(DAE::CallAttributes { ty: equalityConstraintFunctionReturnType.clone(), tuple_: false, builtin: false, isImpure: false, isFunctionPointerCall: false, inlineType: inlineType1.clone(), tailCall: openmodelica_frontend_types::DAE::TailCall::NO_TAIL, noReturn: DAE::NoReturn::RETURNS.clone() }) }), source: source.clone() })];
                    graph = ConnectionGraph::addConnection(metamodelica::AsArg::as_arg(&graph), c1_1.clone(), c2_1.clone(), breakDAEElements.clone())?;
                    (cache, equalityConstraintFunction, env) = Lookup::lookupClass(metamodelica::AsArg::as_arg(&cache), metamodelica::AsArg::as_arg(&env), metamodelica::AsArg::as_arg(&fpath1), None)?;
                    (cache, fpath1) = Inst::makeFullyQualified(cache.clone(), env.clone(), fpath1.clone())?;
                    cache = FCore::addCachedInstFuncGuard(cache.clone(), fpath1.clone())?;
                    (cache, env, ih) = InstFunction::implicitFunctionInstantiation(cache.clone(), env.clone(), ih.clone(), openmodelica_frontend_types::DAE::Mod::interned_NOMOD(), openmodelica_frontend_types::DAE::Prefix::NOPRE, equalityConstraintFunction.clone(), metamodelica::nil())?;
                    Ok((cache.clone(), env.clone(), ih.clone(), sets_1.clone(), dae.clone(), graph.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, ih, sets, pre, c1, f1, Deref @ DAE::Type::T_SUBTYPE_BASIC { complexType: t1, equalityConstraint: Some((fpath1, idim1, inlineType1)), .. }, c2, f2, Deref @ DAE::Type::T_SUBTYPE_BASIC { complexType: t2, equalityConstraint: Some(_), .. }, ct @ Deref @ DAE::ConnectorType::POTENTIAL { .. }, graph @ ConnectionGraph::ConnectionGraph { updateGraph: true, .. }) => {
                    let mut c1_1: metamodelica::Ref<DAE::ComponentRef>;
                    let mut c2_1: metamodelica::Ref<DAE::ComponentRef>;
                    let mut sets_1: DAE::Connect::Sets;
                    let mut equalityConstraintFunctionReturnType: metamodelica::Ref<DAE::Type>;
                    let mut dae: DAE::DAElist;
                    let mut source: metamodelica::Ref<DAE::ElementSource>;
                    let mut zeroVector: metamodelica::Ref<DAE::Exp>;
                    let mut crefExp1: metamodelica::Ref<DAE::Exp>;
                    let mut crefExp2: metamodelica::Ref<DAE::Exp>;
                    let mut breakDAEElements: metamodelica::List<metamodelica::Ref<DAE::Element>>;
                    let mut equalityConstraintFunction: metamodelica::Ref<SCode::Element>;
                    let mut cache = (*cache).clone();
                    let mut env = (*env).clone();
                    let mut ih = (*ih).clone();
                    let mut fpath1 = (*fpath1).clone();
                    let mut graph = (*graph).clone();
                    (cache, c1_1) = PrefixUtil::prefixCref(cache.clone(), env.clone(), metamodelica::AsArg::as_arg(&ih), pre.clone(), c1.clone())?;
                    (cache, c2_1) = PrefixUtil::prefixCref(cache.clone(), env.clone(), metamodelica::AsArg::as_arg(&ih), pre.clone(), c2.clone())?;
                    (cache, env, ih, sets_1, dae, _) = connectComponents(cache.clone(), env.clone(), ih.clone(), sets.clone(), pre.clone(), c1.clone(), f1.clone(), t1.clone(), vt1, c2.clone(), f2.clone(), t2.clone(), vt2, ct.clone(), io1, io2, ConnectionGraph::NOUPDATE_EMPTY().clone(), info)?;
                    source = ElementSource::createElementSource(info.clone(), FGraph::getScopePath(metamodelica::AsArg::as_arg(&env))?, metamodelica::AsArg::as_arg(&pre), (c1_1.clone(), c2_1.clone()));
                    zeroVector = Expression::makeRealArrayOfZeros(idim1.clone());
                    crefExp1 = Expression::crefExp(c1_1.clone())?;
                    crefExp2 = Expression::crefExp(c2_1.clone())?;
                    equalityConstraintFunctionReturnType = metamodelica::Ref::new(DAE::Type::T_ARRAY { ty: DAE::T_REAL_DEFAULT().clone(), dims: list![metamodelica::Ref::new(DAE::Dimension::DIM_INTEGER { integer: idim1.clone() })] });
                    source = ElementSource::addAdditionalComment(&source, literal!(" equation generated by overconstrained connection graph breaking"));
                    breakDAEElements = list![metamodelica::Ref::new(DAE::Element::ARRAY_EQUATION { dimension: list![metamodelica::Ref::new(DAE::Dimension::DIM_INTEGER { integer: idim1.clone() })], exp: zeroVector.clone(), array: metamodelica::Ref::new(DAE::Exp::CALL { path: fpath1.clone(), expLst: list![crefExp1.clone(), crefExp2.clone()], attr: metamodelica::Ref::new(DAE::CallAttributes { ty: equalityConstraintFunctionReturnType.clone(), tuple_: false, builtin: false, isImpure: false, isFunctionPointerCall: false, inlineType: inlineType1.clone(), tailCall: openmodelica_frontend_types::DAE::TailCall::NO_TAIL, noReturn: DAE::NoReturn::RETURNS.clone() }) }), source: source.clone() })];
                    graph = ConnectionGraph::addConnection(metamodelica::AsArg::as_arg(&graph), ComponentReferenceBasics::crefStripLastSubs(&c1_1)?, ComponentReferenceBasics::crefStripLastSubs(&c2_1)?, breakDAEElements.clone())?;
                    (cache, equalityConstraintFunction, env) = Lookup::lookupClass(metamodelica::AsArg::as_arg(&cache), metamodelica::AsArg::as_arg(&env), metamodelica::AsArg::as_arg(&fpath1), None)?;
                    (cache, fpath1) = Inst::makeFullyQualified(cache.clone(), env.clone(), fpath1.clone())?;
                    cache = FCore::addCachedInstFuncGuard(cache.clone(), fpath1.clone())?;
                    (cache, env, ih) = InstFunction::implicitFunctionInstantiation(cache.clone(), env.clone(), ih.clone(), openmodelica_frontend_types::DAE::Mod::interned_NOMOD(), openmodelica_frontend_types::DAE::Prefix::NOPRE, equalityConstraintFunction.clone(), metamodelica::nil())?;
                    Ok((cache.clone(), env.clone(), ih.clone(), sets_1.clone(), dae.clone(), graph.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, ih, sets, pre, c1, f1, Deref @ DAE::Type::T_SUBTYPE_BASIC { complexType: bc_tp1, .. }, c2, f2, t2, ct, graph) => {
                    let mut sets_1: DAE::Connect::Sets;
                    let mut dae: DAE::DAElist;
                    let mut cache = (*cache).clone();
                    let mut ih = (*ih).clone();
                    let mut graph = (*graph).clone();
                    (cache, _, ih, sets_1, dae, graph) = connectComponents(cache.clone(), env.clone(), ih.clone(), sets.clone(), pre.clone(), c1.clone(), f1.clone(), bc_tp1.clone(), vt1, c2.clone(), f2.clone(), t2.clone(), vt2, ct.clone(), io1, io2, graph.clone(), info)?;
                    Ok((cache.clone(), env.clone(), ih.clone(), sets_1.clone(), dae.clone(), graph.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, ih, sets, pre, c1, f1, t1, c2, f2, Deref @ DAE::Type::T_SUBTYPE_BASIC { complexType: bc_tp2, .. }, ct, graph) => {
                    let mut sets_1: DAE::Connect::Sets;
                    let mut dae: DAE::DAElist;
                    let mut cache = (*cache).clone();
                    let mut ih = (*ih).clone();
                    let mut graph = (*graph).clone();
                    (cache, _, ih, sets_1, dae, graph) = connectComponents(cache.clone(), env.clone(), ih.clone(), sets.clone(), pre.clone(), c1.clone(), f1.clone(), t1.clone(), vt1, c2.clone(), f2.clone(), bc_tp2.clone(), vt2, ct.clone(), io1, io2, graph.clone(), info)?;
                    Ok((cache.clone(), env.clone(), ih.clone(), sets_1.clone(), dae.clone(), graph.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, ih, sets, pre, c1, f1, Deref @ DAE::Type::T_COMPLEX { complexClassType: ClassInf::State::EXTERNAL_OBJ { .. }, varLst: Deref @ metamodelica::ListNode::Nil, .. }, c2, f2, Deref @ DAE::Type::T_COMPLEX { complexClassType: ClassInf::State::EXTERNAL_OBJ { .. }, varLst: Deref @ metamodelica::ListNode::Nil, .. }, _, graph) => {
                    let mut c1_1: metamodelica::Ref<DAE::ComponentRef>;
                    let mut c2_1: metamodelica::Ref<DAE::ComponentRef>;
                    let mut sets_1: DAE::Connect::Sets;
                    let mut source: metamodelica::Ref<DAE::ElementSource>;
                    let mut cache = (*cache).clone();
                    (cache, c1_1) = PrefixUtil::prefixCref(cache.clone(), env.clone(), metamodelica::AsArg::as_arg(&ih), pre.clone(), c1.clone())?;
                    (cache, c2_1) = PrefixUtil::prefixCref(cache.clone(), env.clone(), metamodelica::AsArg::as_arg(&ih), pre.clone(), c2.clone())?;
                    source = ElementSource::createElementSource(info.clone(), FGraph::getScopePath(metamodelica::AsArg::as_arg(&env))?, metamodelica::AsArg::as_arg(&pre), (c1_1.clone(), c2_1.clone()));
                    sets_1 = ConnectUtil::addConnection(sets.clone(), c1.clone(), f1.clone(), c2.clone(), f2.clone(), &inConnectorType, source.clone())?;
                    Ok((cache.clone(), env.clone(), ih.clone(), sets_1.clone(), DAE::emptyDae().clone(), graph.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, ih, sets, pre, c1, f1, Deref @ DAE::Type::T_COMPLEX { varLst: l1, .. }, c2, f2, Deref @ DAE::Type::T_COMPLEX { varLst: l2, .. }, ct, graph) => {
                    let mut sets_1: DAE::Connect::Sets;
                    let mut dae: DAE::DAElist;
                    let mut cache = (*cache).clone();
                    let mut ih = (*ih).clone();
                    let mut graph = (*graph).clone();
                    (cache, _, ih, sets_1, dae, graph) = connectVars(cache.clone(), env.clone(), ih.clone(), sets.clone(), metamodelica::AsArg::as_arg(&pre), c1.clone(), f1.clone(), metamodelica::AsArg::as_arg(&l1), vt1, c2.clone(), f2.clone(), metamodelica::AsArg::as_arg(&l2), vt2, metamodelica::AsArg::as_arg(&ct), io1, io2, graph.clone(), info)?;
                    Ok((cache.clone(), env.clone(), ih.clone(), sets_1.clone(), dae.clone(), graph.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, ih, _, pre, c1, _, t1, c2, _, t2, _, _) => {
                    let mut c1_str: ArcStr;
                    let mut t1_str: ArcStr;
                    let mut t2_str: ArcStr;
                    let mut c2_str: ArcStr;
                    let mut cache = (*cache).clone();
                    (cache, _) = PrefixUtil::prefixCref(cache.clone(), env.clone(), metamodelica::AsArg::as_arg(&ih), pre.clone(), c1.clone())?;
                    (cache, _) = PrefixUtil::prefixCref(cache.clone(), env.clone(), metamodelica::AsArg::as_arg(&ih), pre.clone(), c2.clone())?;
                    c1_str = ComponentReferenceBasics::printComponentRefStr(metamodelica::AsArg::as_arg(&c1))?;
                    t1_str = TypesDump::unparseType(t1.clone())?;
                    c2_str = ComponentReferenceBasics::printComponentRefStr(metamodelica::AsArg::as_arg(&c2))?;
                    t2_str = TypesDump::unparseType(t2.clone())?;
                    c1_str = stringAppendList(list![literal!("\n"), c1_str.clone(), literal!(" type:\n"), t1_str.clone()]);
                    c2_str = stringAppendList(list![literal!("\n"), c2_str.clone(), literal!(" type:\n"), t2_str.clone()]);
                    Error::addSourceMessage(&(Error::INVALID_CONNECTOR_VARIABLE.clone()), list![c1_str.clone(), c2_str.clone()], info)?;
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
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { return Err("pattern mismatch") };
                    Debug::trace(literal!("- InstSection.connectComponents failed\n"))?;
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((outCache, outEnv, outIH, outSets, outDae, outGraph))
}

fn generateConnectAssert(
    mut inLhsExp: metamodelica::Ref<DAE::Exp>,
    mut inRhsExp: metamodelica::Ref<DAE::Exp>,
    mut inSource: metamodelica::Ref<DAE::ElementSource>,
) -> Result<metamodelica::Ref<DAE::Element>> {
    let mut outAssert: metamodelica::Ref<DAE::Element>;
    let mut exp: metamodelica::Ref<DAE::Exp>;
    exp = metamodelica::Ref::new(DAE::Exp::RELATION {
        exp1: inLhsExp,
        operator: DAE::Operator::EQUAL {
            ty: DAE::T_BOOL_DEFAULT().clone(),
        },
        exp2: inRhsExp,
        index: -1,
        optionExpisASUB: None,
    });
    (exp, _) = ExpressionSimplify::simplify(exp)?;
    outAssert = metamodelica::Ref::new(DAE::Element::ASSERT {
        condition: exp,
        message: metamodelica::Ref::new(DAE::Exp::SCONST {
            string: literal!("automatically generated from connect"),
        }),
        level: DAE::ASSERTIONLEVEL_ERROR().clone(),
        source: inSource,
    });
    Ok(outAssert)
}

fn connectArrayComponents(
    mut inCache: &FCore::Cache,
    mut inEnv: &FCore::Graph,
    mut inIH: &metamodelica::List<InnerOuter::TopInstance>,
    mut inSets: &DAE::Connect::Sets,
    mut inPrefix: &DAE::Prefix,
    mut inLhsCrefs: &metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
    mut inLhsFace: DAE::Connect::Face,
    mut inLhsType: &metamodelica::Ref<DAE::Type>,
    mut inLhsVar: SCode::Variability,
    mut inLhsIO: Absyn::InnerOuter,
    mut inRhsCrefs: &metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
    mut inRhsFace: DAE::Connect::Face,
    mut inRhsType: &metamodelica::Ref<DAE::Type>,
    mut inRhsVar: SCode::Variability,
    mut inRhsIO: Absyn::InnerOuter,
    mut inConnectorType: &metamodelica::Ref<DAE::ConnectorType>,
    mut inGraph: &ConnectionGraph::ConnectionGraph,
    mut inInfo: &SourceInfo,
) -> Result<(
    FCore::Cache,
    FCore::Graph,
    metamodelica::List<InnerOuter::TopInstance>,
    DAE::Connect::Sets,
    DAE::DAElist,
    ConnectionGraph::ConnectionGraph,
)> {
    let mut outCache: FCore::Cache;
    let mut outEnv: FCore::Graph;
    let mut outIH: metamodelica::List<InnerOuter::TopInstance>;
    let mut outSets: DAE::Connect::Sets;
    let mut outDae: DAE::DAElist;
    let mut outGraph: ConnectionGraph::ConnectionGraph;
    (outCache, outEnv, outIH, outSets, outDae, outGraph) = (::match_deref::match_deref! { match (inLhsCrefs, inRhsCrefs) {
        (Deref @ metamodelica::ListNode::Cons { head: lhs, tail: rest_lhs }, Deref @ metamodelica::ListNode::Cons { head: rhs, tail: rest_rhs }) => {
            let mut cache: FCore::Cache;
            let mut env: FCore::Graph;
            let mut ih: InstanceHierarchy;
            let mut sets: DAE::Connect::Sets;
            let mut dae1: DAE::DAElist;
            let mut dae2: DAE::DAElist;
            let mut graph: ConnectionGraph::ConnectionGraph;
            (cache, env, ih, sets, dae1, graph) = connectComponents(inCache.clone(), inEnv.clone(), inIH.clone(), inSets.clone(), inPrefix.clone(), lhs.clone(), inLhsFace, inLhsType.clone(), inLhsVar, rhs.clone(), inRhsFace, inRhsType.clone(), inRhsVar, inConnectorType.clone(), inLhsIO, inRhsIO, inGraph.clone(), inInfo)?;
            (cache, env, ih, sets, dae2, graph) = connectArrayComponents(&cache, &env, &ih, &sets, inPrefix, rest_lhs, inLhsFace, inLhsType, inLhsVar, inLhsIO, rest_rhs, inRhsFace, inRhsType, inRhsVar, inRhsIO, inConnectorType, &graph, inInfo)?;
            dae1 = DAEUtil::joinDaes(&dae1, &dae2)?;
            (cache, env, ih, sets, dae1, graph)
        },
        _ => {
            (inCache.clone(), inEnv.clone(), inIH.clone(), inSets.clone(), DAE::emptyDae().clone(), inGraph.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outCache, outEnv, outIH, outSets, outDae, outGraph))
}

fn connectVars(
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
    mut inIH: metamodelica::List<InnerOuter::TopInstance>,
    mut inSets: DAE::Connect::Sets,
    mut inPrefix: &DAE::Prefix,
    mut inComponentRef3: metamodelica::Ref<DAE::ComponentRef>,
    mut inFace4: DAE::Connect::Face,
    mut inTypesVarLst5: &metamodelica::List<metamodelica::Ref<DAE::Var>>,
    mut vt1: SCode::Variability,
    mut inComponentRef6: metamodelica::Ref<DAE::ComponentRef>,
    mut inFace7: DAE::Connect::Face,
    mut inTypesVarLst8: &metamodelica::List<metamodelica::Ref<DAE::Var>>,
    mut vt2: SCode::Variability,
    mut inConnectorType: &metamodelica::Ref<DAE::ConnectorType>,
    mut io1: Absyn::InnerOuter,
    mut io2: Absyn::InnerOuter,
    mut inGraph: ConnectionGraph::ConnectionGraph,
    mut info: &SourceInfo,
) -> Result<(
    FCore::Cache,
    FCore::Graph,
    metamodelica::List<InnerOuter::TopInstance>,
    DAE::Connect::Sets,
    DAE::DAElist,
    ConnectionGraph::ConnectionGraph,
)> {
    let mut outCache: FCore::Cache;
    let mut outEnv: FCore::Graph;
    let mut outIH: metamodelica::List<InnerOuter::TopInstance>;
    let mut outSets: DAE::Connect::Sets;
    let mut outDae: DAE::DAElist;
    let mut outGraph: ConnectionGraph::ConnectionGraph;
    (outCache, outEnv, outIH, outSets, outDae, outGraph) = (::match_deref::match_deref! { match (inTypesVarLst5, inTypesVarLst8) {
        (Deref @ metamodelica::ListNode::Nil, Deref @ metamodelica::ListNode::Nil) => {
            let mut cache = inCache;
            let mut env = inEnv;
            let mut ih = inIH;
            let mut sets = inSets;
            let mut graph = inGraph;
            (cache, env, ih, sets, DAE::emptyDae().clone(), graph)
        },
        (Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Var { name: n, attributes: attr1 @ Deref @ DAE::Attributes { connectorType: ct, variability: vta, .. }, ty: ty1, .. }, tail: xs1 }, Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Var { attributes: attr2 @ Deref @ DAE::Attributes { variability: vtb, .. }, ty: ty2, .. }, tail: xs2 }) => {
            let mut cache = inCache;
            let mut env = inEnv;
            let mut ih = inIH;
            let mut sets = inSets;
            let mut c1 = inComponentRef3;
            let mut f1 = inFace4;
            let mut c2 = inComponentRef6;
            let mut f2 = inFace7;
            let mut graph = inGraph;
            let mut sets_1: DAE::Connect::Sets;
            let mut sets_2: DAE::Connect::Sets;
            let mut c1_1: metamodelica::Ref<DAE::ComponentRef>;
            let mut c2_1: metamodelica::Ref<DAE::ComponentRef>;
            let mut dae: DAE::DAElist;
            let mut dae2: DAE::DAElist;
            let mut dae_1: DAE::DAElist;
            let mut ty_2: metamodelica::Ref<DAE::Type>;
            let mut ct = (*ct).clone();
            ty_2 = Types::simplifyType(ty1.clone())?;
            ct = propagateConnectorType(inConnectorType.clone(), ct.clone());
            c1_1 = ComponentReference::crefPrependIdent(&c1, metamodelica::AsArg::as_arg(&n), &(metamodelica::nil()), &ty_2)?;
            c2_1 = ComponentReference::crefPrependIdent(&c2, metamodelica::AsArg::as_arg(&n), &(metamodelica::nil()), &ty_2)?;
            checkConnectTypes(&c1_1, ty1.clone(), f1, metamodelica::AsArg::as_arg(&attr1), &c2_1, ty2.clone(), f2, metamodelica::AsArg::as_arg(&attr2), info)?;
            (cache, _, ih, sets_1, dae, graph) = connectComponents(cache, env.clone(), ih, sets, inPrefix.clone(), c1_1, f1, ty1.clone(), vta.clone(), c2_1, f2, ty2.clone(), vtb.clone(), ct.clone(), io1, io2, graph, info)?;
            (cache, _, ih, sets_2, dae2, graph) = connectVars(cache, env.clone(), ih, sets_1, inPrefix, c1, f1, xs1, vt1, c2, f2, xs2, vt2, inConnectorType, io1, io2, graph, info)?;
            dae_1 = DAEUtil::joinDaes(&dae, &dae2)?;
            (cache, env, ih, sets_2, dae_1, graph)
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok((outCache, outEnv, outIH, outSets, outDae, outGraph))
}

fn propagateConnectorType(
    mut inConnectorType: metamodelica::Ref<DAE::ConnectorType>,
    mut inSubConnectorType: metamodelica::Ref<DAE::ConnectorType>,
) -> metamodelica::Ref<DAE::ConnectorType> {
    let mut outSubConnectorType: metamodelica::Ref<DAE::ConnectorType>;
    outSubConnectorType = (match &*inConnectorType {
        DAE::ConnectorType::POTENTIAL { .. } => inSubConnectorType,
        _ => inConnectorType,
    });
    outSubConnectorType
}

fn expandArrayDimension(
    mut inDim: &metamodelica::Ref<DAE::Dimension>,
    mut inArray: metamodelica::Ref<DAE::Exp>,
) -> Result<metamodelica::List<metamodelica::Ref<DAE::Exp>>> {
    let mut outExpl: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
    outExpl = 'mc: {
        let __mc_input = (&**inDim, &*inArray);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Deref @ DAE::Exp::ARRAY { array: outExpl, .. }) => {
                    Ok(outExpl.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Dimension::DIM_INTEGER { integer: 0 }, _) => {
                    Ok(metamodelica::nil())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Dimension::DIM_INTEGER { integer: sz }, _) => {
                    let mut expl: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                    let mut ints: metamodelica::List<i32>;
                    ints = List::intRange(sz.clone());
                    expl = List::map1(ints.clone(), &makeAsubIndex, inArray.clone())?;
                    Ok(expl.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Dimension::DIM_BOOLEAN { .. }, _) => {
                    let mut expl: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                    expl = list![(ExpressionSimplify::simplify1(Expression::makeASUB(inArray.clone(), list![metamodelica::Ref::new(DAE::Exp::BCONST { bool: false })])?)?).0, (ExpressionSimplify::simplify1(Expression::makeASUB(inArray.clone(), list![metamodelica::Ref::new(DAE::Exp::BCONST { bool: true })])?)?).0];
                    Ok(expl.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Dimension::DIM_ENUM { enumTypeName: name, literals: ls, .. }, _) => {
                    let mut expl: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                    expl = makeEnumLiteralIndices(metamodelica::AsArg::as_arg(&name), metamodelica::AsArg::as_arg(&ls), 1, &inArray)?;
                    Ok(expl.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Dimension::DIM_UNKNOWN { .. }, _) => {
                    let mut expl: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                    let mut ints: metamodelica::List<i32>;
                    let true = (Flags::getConfigBool(Flags::CHECK_MODEL.clone())?) else { return Err("pattern mismatch") };
                    ints = List::intRange(1);
                    expl = List::map1(ints.clone(), &makeAsubIndex, inArray.clone())?;
                    Ok(expl.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(outExpl)
}

fn makeAsubIndex(mut index: i32, mut expr: metamodelica::Ref<DAE::Exp>) -> Result<metamodelica::Ref<DAE::Exp>> {
    let mut asub: metamodelica::Ref<DAE::Exp>;
    (asub, _) = ExpressionSimplify::simplify1(Expression::makeASUB(
        expr,
        list![metamodelica::Ref::new(DAE::Exp::ICONST { integer: index })],
    )?)?;
    Ok(asub)
}

fn makeEnumLiteralIndices(
    mut enumTypeName: &metamodelica::Ref<Absyn::Path>,
    mut enumLiterals: &metamodelica::List<ArcStr>,
    mut enumIndex: i32,
    mut expr: &metamodelica::Ref<DAE::Exp>,
) -> Result<metamodelica::List<metamodelica::Ref<DAE::Exp>>> {
    let mut enumIndices: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
    enumIndices = (::match_deref::match_deref! { match enumLiterals {
        Deref @ metamodelica::ListNode::Nil => {
            metamodelica::nil()
        },
        Deref @ metamodelica::ListNode::Cons { head: l, tail: ls } => {
            let mut e: metamodelica::Ref<DAE::Exp>;
            let mut expl: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
            let mut enum_type_name: metamodelica::Ref<Absyn::Path>;
            let mut index: i32;
            enum_type_name = AbsynUtil::joinPaths(enumTypeName.clone(), metamodelica::Ref::new(Absyn::Path::IDENT { name: l.clone() }))?;
            e = metamodelica::Ref::new(DAE::Exp::ENUM_LITERAL { name: enum_type_name, index: enumIndex });
            (e, _) = ExpressionSimplify::simplify1(Expression::makeASUB(expr.clone(), list![e])?)?;
            e = if (Expression::isCref(&e)) {Expression::unliftExp(e)?} else {e};
            index = enumIndex + 1;
            expl = makeEnumLiteralIndices(enumTypeName, ls, index, expr)?;
            metamodelica::cons(e, expl)
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(enumIndices)
}

fn getVectorizedCref(mut crefOrArray: metamodelica::Ref<DAE::Exp>) -> Result<metamodelica::Ref<DAE::Exp>> {
    let mut cref: metamodelica::Ref<DAE::Exp>;
    cref = (::match_deref::match_deref! { match &(crefOrArray) {
        __esc_cref @ Deref @ DAE::Exp::CREF { componentRef: _, ty: _ } => {
            cref = (*__esc_cref).clone();
            cref.clone()
        },
        Deref @ DAE::Exp::ARRAY { ty: _, scalar: _, array: Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Exp::CREF { componentRef: cr, ty: t }, tail: _ } } => {
            let mut crefExp: metamodelica::Ref<DAE::Exp>;
            let mut cr = (*cr).clone();
            cr = ComponentReferenceBasics::crefStripLastSubs(metamodelica::AsArg::as_arg(&cr))?;
            crefExp = Expression::makeCrefExp(cr.clone(), t.clone())?;
            crefExp
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok(cref)
}

fn checkWhenAlgorithm(mut inWhenAlgorithm: &metamodelica::Ref<SCode::Statement>) -> Result<()> {
    let true = (checkForReinitInWhenInitialAlg(inWhenAlgorithm)) else {
        return Err("pattern mismatch");
    };
    checkForNestedWhenInStatements(inWhenAlgorithm)?;
    Ok(())
}

fn checkForReinitInWhenInitialAlg(mut inWhenAlgorithm: &metamodelica::Ref<SCode::Statement>) -> bool {
    let mut outOK: bool;
    outOK = 'mc: {
        let __mc_input = &**inWhenAlgorithm;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ SCode::Statement::ALG_WHEN_A { branches: Deref @ metamodelica::ListNode::Cons { head: (exp, algs), tail: _ }, info, .. } => {
                    let true = (AbsynUtil::expContainsInitial(exp.clone())) else { return Err("pattern mismatch") };
                    let true = (SCodeUtil::algorithmsContainReinit(metamodelica::AsArg::as_arg(&algs))?) else { return Err("pattern mismatch") };
                    Error::addSourceMessage(&(Error::REINIT_IN_WHEN_INITIAL.clone()), metamodelica::nil(), metamodelica::AsArg::as_arg(&info))?;
                    Ok(false)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(true)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    outOK
}

fn checkForNestedWhenInStatements(mut inWhenAlgorithm: &metamodelica::Ref<SCode::Statement>) -> Result<()> {
    let mut branches: metamodelica::List<(
        metamodelica::Ref<Absyn::Exp>,
        metamodelica::List<metamodelica::Ref<SCode::Statement>>,
    )>;
    let mut info: SourceInfo;
    let mut body: metamodelica::List<metamodelica::Ref<SCode::Statement>>;
    let (__pa0, __pa1) = ::match_deref::match_deref! { match &((*inWhenAlgorithm)) {
        Deref @ SCode::Statement::ALG_WHEN_A { branches: __pa0, info: __pa1, .. } => (__pa0.clone(), __pa1.clone()),
        _ => return Err("pattern mismatch"),
    } };
    branches = metamodelica::Own::own(__pa0);
    info = metamodelica::Own::own(__pa1);
    for mut branch in &*branches {
        (_, body) = branch.clone();
        if containsWhenStatements(&body)? {
            Error::addSourceMessageAndFail(&(Error::NESTED_WHEN.clone()), metamodelica::nil(), &info)?;
            unreachable!("Error.addSourceMessageAndFail always fails — caller-side flow-analysis hint");
        }
    }
    Ok(())
}

fn checkWhenEquation(mut inWhenEq: &metamodelica::Ref<SCode::Equation>) -> Result<()> {
    let true = (checkForReinitInWhenInitialEq(inWhenEq)) else {
        return Err("pattern mismatch");
    };
    checkForNestedWhenInEquation(inWhenEq)?;
    Ok(())
}

fn checkForReinitInWhenInitialEq(mut inWhenEq: &metamodelica::Ref<SCode::Equation>) -> bool {
    let mut outOK: bool;
    outOK = 'mc: {
        let __mc_input = &**inWhenEq;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ SCode::Equation::EQ_WHEN { condition: exp, eEquationLst: el, info, .. } => {
                    let true = (AbsynUtil::expContainsInitial(exp.clone())) else { return Err("pattern mismatch") };
                    let true = (SCodeUtil::equationsContainReinit(metamodelica::AsArg::as_arg(&el))?) else { return Err("pattern mismatch") };
                    Error::addSourceMessage(&(Error::REINIT_IN_WHEN_INITIAL.clone()), metamodelica::nil(), metamodelica::AsArg::as_arg(&info))?;
                    Ok(false)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(true)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    outOK
}

fn checkForNestedWhenInEquation(mut inWhenEq: &metamodelica::Ref<SCode::Equation>) -> Result<()> {
    let () = (match &**inWhenEq {
        SCode::Equation::EQ_WHEN {
            eEquationLst: eqs,
            elseBranches: tpl_el,
            ..
        } => {
            let mut eqs_lst: metamodelica::List<metamodelica::List<metamodelica::Ref<SCode::Equation>>>;
            checkForNestedWhenInEqList(eqs)?;
            eqs_lst = List::map(tpl_el.clone(), &fnptr!(Util::tuple22, _))?;
            List::map_0(&eqs_lst, &move |__a0: metamodelica::List<
                metamodelica::Ref<SCode::Equation>,
            >| checkForNestedWhenInEqList(&__a0))?;
            ()
        }
        _ => return Err("match: no arm matched"),
    });
    Ok(())
}

fn checkForNestedWhenInEqList(mut inEqs: &metamodelica::List<metamodelica::Ref<SCode::Equation>>) -> Result<()> {
    List::map_0(inEqs, &move |__a0: metamodelica::Ref<SCode::Equation>| {
        checkForNestedWhenInEq(&__a0)
    })?;
    Ok(())
}

fn checkForNestedWhenInEq(mut inEq: &metamodelica::Ref<SCode::Equation>) -> Result<()> {
    let () = (match &**inEq {
        SCode::Equation::EQ_WHEN { info, .. } => {
            Error::addSourceMessage(&(Error::NESTED_WHEN.clone()), metamodelica::nil(), info)?;
            return Err("fail");
        }
        SCode::Equation::EQ_IF {
            thenBranch: eqs_lst,
            elseBranch: eqs,
            ..
        } => {
            List::map_0(eqs_lst, &move |__a0: metamodelica::List<
                metamodelica::Ref<SCode::Equation>,
            >| checkForNestedWhenInEqList(&__a0))?;
            checkForNestedWhenInEqList(eqs)?;
            ()
        }
        SCode::Equation::EQ_FOR { eEquationLst: eqs, .. } => {
            checkForNestedWhenInEqList(eqs)?;
            ()
        }
        SCode::Equation::EQ_EQUALS { .. } => (),
        SCode::Equation::EQ_PDE { .. } => (),
        SCode::Equation::EQ_CONNECT {
            crefLeft: cr1,
            crefRight: cr2,
            info,
            ..
        } => {
            let mut cr1_str: ArcStr;
            let mut cr2_str: ArcStr;
            cr1_str = Dump::printComponentRefStr(cr1)?;
            cr2_str = Dump::printComponentRefStr(cr2)?;
            Error::addSourceMessage(&(Error::CONNECT_IN_WHEN.clone()), list![cr1_str, cr2_str], info)?;
            return Err("fail");
        }
        SCode::Equation::EQ_ASSERT { .. } => (),
        SCode::Equation::EQ_TERMINATE { .. } => (),
        SCode::Equation::EQ_REINIT { .. } => (),
        SCode::Equation::EQ_NORETCALL { .. } => (),
        _ => {
            let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else {
                return Err("pattern mismatch");
            };
            Debug::trace(literal!("- InstSection.checkForNestedWhenInEq failed.\n"))?;
            return Err("fail");
        }
    });
    Ok(())
}

fn instAssignment(
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
    mut ih: &metamodelica::List<InnerOuter::TopInstance>,
    mut inPre: DAE::Prefix,
    mut alg: &metamodelica::Ref<SCode::Statement>,
    mut source: metamodelica::Ref<DAE::ElementSource>,
    mut initial_: SCode::Initial,
    mut r#impl: bool,
    mut unrollForLoops: bool,
    mut numError: i32,
) -> Result<(FCore::Cache, metamodelica::List<metamodelica::Ref<DAE::Statement>>)> {
    let mut outCache: FCore::Cache;
    let mut stmts: metamodelica::List<metamodelica::Ref<DAE::Statement>> = metamodelica::nil();
    (outCache, stmts) = 'mc: {
        let __mc_input = (inCache, inEnv, inPre, &**alg);
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, pre, Deref @ SCode::Statement::ALG_ASSIGN { assignComponent: var, value, info, .. }) => {
                    let mut e_1: metamodelica::Ref<DAE::Exp>;
                    let mut eprop: DAE::Properties;
                    let mut cache = (*cache).clone();
                    let mut stmts: metamodelica::List<metamodelica::Ref<DAE::Statement>> = stmts.clone();
                    (cache, e_1, eprop) = Static::elabExp(cache.clone(), env.clone(), value.clone(), r#impl, true, pre.clone(), info.clone())?;
                    (cache, stmts) = instAssignment2(cache.clone(), metamodelica::AsArg::as_arg(&env), ih, metamodelica::AsArg::as_arg(&pre), var.clone(), value.clone(), e_1.clone(), eprop.clone(), metamodelica::AsArg::as_arg(&info), &(ElementSource::addAnnotation(source.clone(), var_field!((**alg).comment, SCode::Statement::ALG_ASSIGN).clone())), initial_, r#impl, unrollForLoops, numError)?;
                    Ok(((cache.clone(), stmts.clone()), stmts.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            stmts = __wb0;
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, pre, Deref @ SCode::Statement::ALG_ASSIGN { value, info, .. }) => {
                    let mut r#str: ArcStr;
                    let true = (numError == Error::getNumErrorMessages()) else { return Err("pattern mismatch") };
                    if '__try0: {
                        unwrap_break_err!(Static::elabExp(cache.clone(), env.clone(), value.clone(), r#impl, true, pre.clone(), info.clone()), '__try0);
                        Ok::<(), &'static str>(())
                    }.is_ok() { return Err("failure(): body succeeded") }
                    r#str = Dump::unparseAlgorithmStr(SCodeUtil::statementToAlgorithmItem(alg)?)?;
                    Error::addSourceMessage(&(Error::ASSIGN_RHS_ELABORATION.clone()), list![r#str.clone()], metamodelica::AsArg::as_arg(&info))?;
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((outCache, stmts))
}

fn instAssignment2(
    mut inCache: FCore::Cache,
    mut inEnv: &FCore::Graph,
    mut inIH: &metamodelica::List<InnerOuter::TopInstance>,
    mut inPre: &DAE::Prefix,
    mut var: metamodelica::Ref<Absyn::Exp>,
    mut inRhs: metamodelica::Ref<Absyn::Exp>,
    mut value: metamodelica::Ref<DAE::Exp>,
    mut props: DAE::Properties,
    mut info: &SourceInfo,
    mut inSource: &metamodelica::Ref<DAE::ElementSource>,
    mut initial_: SCode::Initial,
    mut inImpl: bool,
    mut unrollForLoops: bool,
    mut numError: i32,
) -> Result<(FCore::Cache, metamodelica::List<metamodelica::Ref<DAE::Statement>>)> {
    let mut outCache: FCore::Cache;
    let mut stmts: metamodelica::List<metamodelica::Ref<DAE::Statement>> = metamodelica::nil();
    let mut varNoComment: metamodelica::Ref<Absyn::Exp>;
    let mut inRhsNoComment: metamodelica::Ref<Absyn::Exp>;
    varNoComment = AbsynUtil::stripCommentExpressions(var.clone(), false)?;
    inRhsNoComment = AbsynUtil::stripCommentExpressions(inRhs.clone(), false)?;
    let () = (::match_deref::match_deref! { match &(&*varNoComment) {
        Deref @ Absyn::Exp::TUPLE { expressions: Deref @ metamodelica::ListNode::Cons { head: lhs, tail: Deref @ metamodelica::ListNode::Nil } } => {
            (outCache, stmts) = instAssignment2(inCache, inEnv, inIH, inPre, lhs.clone(), inRhsNoComment, value, props, info, inSource, initial_, inImpl, unrollForLoops, numError)?;
            return Ok((outCache, stmts));
            ()
        },
        _ => {
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    (outCache, stmts) = 'mc: {
        let __mc_input = (inCache, varNoComment.clone(), value.clone(), props);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, Deref @ Absyn::Exp::CREF { componentRef: cr }, e_1, _) => {
                    let mut lhs: metamodelica::Ref<DAE::Exp>;
                    let mut rhs: metamodelica::Ref<DAE::Exp>;
                    let mut t: metamodelica::Ref<DAE::Type>;
                    let mut attr: metamodelica::Ref<DAE::Attributes>;
                    let mut lhs_dim: metamodelica::Ref<DAE::Dimension>;
                    let mut rhs_dim: metamodelica::Ref<DAE::Dimension>;
                    let mut cache = (*cache).clone();
                    let (__pa0, __pa2, __pa1, __pa3) = ::match_deref::match_deref! { match &(Static::elabCrefNoEval(cache.clone(), inEnv.clone(), cr.clone(), inImpl, false, inPre.clone(), info.clone())?) {
                        (__pa0, __pa2 @ Deref @ DAE::Exp::CREF { componentRef: _, ty: __pa1 }, _, __pa3) => (__pa0.clone(), __pa2.clone(), __pa1.clone(), __pa3.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    cache = metamodelica::Own::own(__pa0);
                    t = metamodelica::Own::own(__pa1);
                    lhs = metamodelica::Own::own(__pa2);
                    attr = metamodelica::Own::own(__pa3);
                    ::match_deref::match_deref! { match &(t.clone()) {
                        Deref @ DAE::Type::T_ARRAY { dims: Deref @ metamodelica::ListNode::Cons { head: _, tail: Deref @ metamodelica::ListNode::Nil }, .. } => (),
                        _ => return Err("pattern mismatch"),
                    } };
                    rhs = e_1.clone();
                    Static::checkAssignmentToInput(&varNoComment, &attr, inEnv, false, info)?;
                    let __pa6 = ::match_deref::match_deref! { match &(Expression::r#typeof(lhs.clone())?) {
                        Deref @ DAE::Type::T_ARRAY { dims: Deref @ metamodelica::ListNode::Cons { head: __pa6, tail: _ }, .. } => __pa6.clone(),
                        _ => return Err("pattern mismatch"),
                    } };
                    lhs_dim = metamodelica::Own::own(__pa6);
                    let __pa7 = ::match_deref::match_deref! { match &(Expression::r#typeof(rhs.clone())?) {
                        Deref @ DAE::Type::T_ARRAY { dims: Deref @ metamodelica::ListNode::Cons { head: __pa7, tail: _ }, .. } => __pa7.clone(),
                        _ => return Err("pattern mismatch"),
                    } };
                    rhs_dim = metamodelica::Own::own(__pa7);
                    ::match_deref::match_deref! { match &(expandArrayDimension(&lhs_dim, lhs.clone())?) {
                        Deref @ metamodelica::ListNode::Nil => (),
                        _ => return Err("pattern mismatch"),
                    } };
                    ::match_deref::match_deref! { match &(expandArrayDimension(&rhs_dim, rhs.clone())?) {
                        Deref @ metamodelica::ListNode::Nil => (),
                        _ => return Err("pattern mismatch"),
                    } };
                    Ok((cache.clone(), metamodelica::nil()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, Deref @ Absyn::Exp::CREF { componentRef: cr }, e_1, eprop) => {
                    let mut ce: metamodelica::Ref<DAE::ComponentRef>;
                    let mut ce_1: metamodelica::Ref<DAE::ComponentRef>;
                    let mut cprop: DAE::Properties;
                    let mut e_2: metamodelica::Ref<DAE::Exp>;
                    let mut stmt: metamodelica::Ref<DAE::Statement>;
                    let mut lt: metamodelica::Ref<DAE::Type>;
                    let mut rt: metamodelica::Ref<DAE::Type>;
                    let mut t: metamodelica::Ref<DAE::Type>;
                    let mut attr: metamodelica::Ref<DAE::Attributes>;
                    let mut source: metamodelica::Ref<DAE::ElementSource>;
                    let mut cache = (*cache).clone();
                    let mut e_1 = (*e_1).clone();
                    let mut eprop = (*eprop).clone();
                    let (__pa0, __pa1, __pa2, __pa3, __pa4) = ::match_deref::match_deref! { match &(Static::elabCrefNoEval(cache.clone(), inEnv.clone(), cr.clone(), inImpl, false, inPre.clone(), info.clone())?) {
                        (__pa0, Deref @ DAE::Exp::CREF { componentRef: __pa1, ty: __pa2 }, __pa3, __pa4) => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone(), __pa4.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    cache = metamodelica::Own::own(__pa0);
                    ce = metamodelica::Own::own(__pa1);
                    t = metamodelica::Own::own(__pa2);
                    cprop = metamodelica::Own::own(__pa3);
                    attr = metamodelica::Own::own(__pa4);
                    Static::checkAssignmentToInput(&varNoComment, &attr, inEnv, false, info)?;
                    (cache, ce_1) = Static::canonCref(cache.clone(), inEnv.clone(), ce.clone(), inImpl)?;
                    (cache, ce_1) = PrefixUtil::prefixCrefInnerOuter(cache.clone(), inEnv, inIH.clone(), ce_1.clone(), inPre.clone())?;
                    (cache, t) = PrefixUtil::prefixExpressionsInType(cache.clone(), inEnv.clone(), inIH.clone(), inPre.clone(), t.clone())?;
                    lt = Types::getPropType(&cprop);
                    (cache, lt) = PrefixUtil::prefixExpressionsInType(cache.clone(), inEnv.clone(), inIH.clone(), inPre.clone(), lt.clone())?;
                    cprop = Types::setPropType(&cprop, lt.clone());
                    (cache, e_1, eprop) = Ceval::cevalIfConstant(cache.clone(), inEnv.clone(), e_1.clone(), eprop.clone(), inImpl, info.clone())?;
                    (cache, e_2) = PrefixUtil::prefixExp(cache.clone(), inEnv, inIH, e_1.clone(), inPre)?;
                    rt = Types::getPropType(metamodelica::AsArg::as_arg(&eprop));
                    (cache, rt) = PrefixUtil::prefixExpressionsInType(cache.clone(), inEnv.clone(), inIH.clone(), inPre.clone(), rt.clone())?;
                    eprop = Types::setPropType(metamodelica::AsArg::as_arg(&eprop), rt.clone());
                    source = ElementSource::addElementSourceFileInfo(inSource.clone(), info.clone());
                    stmt = makeAssignment(Expression::makeCrefExp(ce_1.clone(), t.clone())?, cprop.clone(), e_2.clone(), eprop.clone(), &attr, initial_, source.clone())?;
                    Ok((cache.clone(), list![stmt.clone()]))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, e2 @ Deref @ Absyn::Exp::CALL { function_: Deref @ Absyn::ComponentRef::CREF_IDENT { name: Deref @ "der", .. }, functionArgs: Deref @ Absyn::FunctionArgs::FUNCTIONARGS { args: Deref @ metamodelica::ListNode::Cons { head: Deref @ Absyn::Exp::CREF { componentRef: cr }, tail: Deref @ metamodelica::ListNode::Nil }, .. }, .. }, e_1, eprop) => {
                    let mut cprop: DAE::Properties;
                    let mut e_2: metamodelica::Ref<DAE::Exp>;
                    let mut e2_2: metamodelica::Ref<DAE::Exp>;
                    let mut e2_2_2: metamodelica::Ref<DAE::Exp>;
                    let mut stmt: metamodelica::Ref<DAE::Statement>;
                    let mut attr: metamodelica::Ref<DAE::Attributes>;
                    let mut source: metamodelica::Ref<DAE::ElementSource>;
                    let mut cache = (*cache).clone();
                    let mut e_1 = (*e_1).clone();
                    let mut eprop = (*eprop).clone();
                    (cache, _, cprop, attr) = Static::elabCrefNoEval(cache.clone(), inEnv.clone(), cr.clone(), inImpl, false, inPre.clone(), info.clone())?;
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(Static::elabExp(cache.clone(), inEnv.clone(), e2.clone(), inImpl, true, inPre.clone(), info.clone())?) {
                        (__pa0, __pa1 @ Deref @ DAE::Exp::CALL { .. }, _) => (__pa0.clone(), __pa1.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    cache = metamodelica::Own::own(__pa0);
                    e2_2 = metamodelica::Own::own(__pa1);
                    (cache, e2_2_2) = PrefixUtil::prefixExp(cache.clone(), inEnv, inIH, e2_2.clone(), inPre)?;
                    (cache, e_1, eprop) = Ceval::cevalIfConstant(cache.clone(), inEnv.clone(), e_1.clone(), eprop.clone(), inImpl, info.clone())?;
                    (cache, e_2) = PrefixUtil::prefixExp(cache.clone(), inEnv, inIH, e_1.clone(), inPre)?;
                    source = ElementSource::addElementSourceFileInfo(inSource.clone(), info.clone());
                    stmt = makeAssignment(e2_2_2.clone(), cprop.clone(), e_2.clone(), eprop.clone(), &attr, initial_, source.clone())?;
                    Ok((cache.clone(), list![stmt.clone()]))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, Deref @ Absyn::Exp::CREF { componentRef: cr }, e_1, eprop) => {
                    let mut cprop: DAE::Properties;
                    let mut e_2: metamodelica::Ref<DAE::Exp>;
                    let mut cre: metamodelica::Ref<DAE::Exp>;
                    let mut cre2: metamodelica::Ref<DAE::Exp>;
                    let mut stmt: metamodelica::Ref<DAE::Statement>;
                    let mut attr: metamodelica::Ref<DAE::Attributes>;
                    let mut source: metamodelica::Ref<DAE::ElementSource>;
                    let mut cache = (*cache).clone();
                    let mut e_1 = (*e_1).clone();
                    let mut eprop = (*eprop).clone();
                    (cache, cre, cprop, attr) = Static::elabCrefNoEval(cache.clone(), inEnv.clone(), cr.clone(), inImpl, false, inPre.clone(), info.clone())?;
                    Static::checkAssignmentToInput(&varNoComment, &attr, inEnv, false, info)?;
                    (cache, cre2) = PrefixUtil::prefixExp(cache.clone(), inEnv, inIH, cre.clone(), inPre)?;
                    (cache, e_1, eprop) = Ceval::cevalIfConstant(cache.clone(), inEnv.clone(), e_1.clone(), eprop.clone(), inImpl, info.clone())?;
                    (cache, e_2) = PrefixUtil::prefixExp(cache.clone(), inEnv, inIH, e_1.clone(), inPre)?;
                    source = ElementSource::addElementSourceFileInfo(inSource.clone(), info.clone());
                    stmt = makeAssignment(cre2.clone(), cprop.clone(), e_2.clone(), eprop.clone(), &attr, initial_, source.clone())?;
                    Ok((cache.clone(), list![stmt.clone()]))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, Deref @ Absyn::Exp::TUPLE { expressions: expl }, e_1, eprop) => {
                    let mut e_2: metamodelica::Ref<DAE::Exp>;
                    let mut stmt: metamodelica::Ref<DAE::Statement>;
                    let mut expl_1: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                    let mut expl_2: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                    let mut cprops: metamodelica::List<DAE::Properties>;
                    let mut attrs: metamodelica::List<metamodelica::Ref<DAE::Attributes>>;
                    let mut source: metamodelica::Ref<DAE::ElementSource>;
                    let mut cache = (*cache).clone();
                    let mut e_1 = (*e_1).clone();
                    let mut eprop = (*eprop).clone();
                    let true = (List::all(metamodelica::AsArg::as_arg(&expl), &move |__a0: metamodelica::Ref<Absyn::Exp>| -> metamodelica::Result<_> { ::std::result::Result::Ok(AbsynUtil::isCref(&__a0)) })?) else { return Err("pattern mismatch") };
                    let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(Ceval::cevalIfConstant(cache.clone(), inEnv.clone(), e_1.clone(), eprop.clone(), inImpl, info.clone())?) {
                        (__pa0, __pa1 @ Deref @ DAE::Exp::CALL { .. }, __pa2) => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    cache = metamodelica::Own::own(__pa0);
                    e_1 = metamodelica::Own::own(__pa1);
                    eprop = metamodelica::Own::own(__pa2);
                    (cache, e_2) = PrefixUtil::prefixExp(cache.clone(), inEnv, inIH, e_1.clone(), inPre)?;
                    (cache, expl_1, cprops, attrs) = Static::elabExpCrefNoEvalList(cache.clone(), inEnv.clone(), metamodelica::AsArg::as_arg(&expl), inImpl, false, inPre.clone(), info.clone())?;
                    Static::checkAssignmentToInputs(metamodelica::AsArg::as_arg(&expl), attrs.clone(), inEnv, info.clone())?;
                    checkNoDuplicateAssignments(expl_1.clone(), info)?;
                    (cache, expl_2) = PrefixUtil::prefixExpList(cache.clone(), inEnv, inIH, &expl_1, inPre)?;
                    source = ElementSource::addElementSourceFileInfo(inSource.clone(), info.clone());
                    stmt = Algorithm::makeTupleAssignment(expl_2.clone(), cprops.clone(), e_2.clone(), eprop.clone(), initial_, source.clone())?;
                    Ok((cache.clone(), list![stmt.clone()]))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, Deref @ Absyn::Exp::TUPLE { expressions: expl }, e_1, eprop) => {
                    let mut e_2: metamodelica::Ref<DAE::Exp>;
                    let mut stmt: metamodelica::Ref<DAE::Statement>;
                    let mut expl_1: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                    let mut expl_2: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                    let mut cprops: metamodelica::List<DAE::Properties>;
                    let mut attrs: metamodelica::List<metamodelica::Ref<DAE::Attributes>>;
                    let mut source: metamodelica::Ref<DAE::ElementSource>;
                    let mut cache = (*cache).clone();
                    let mut e_1 = (*e_1).clone();
                    let mut eprop = (*eprop).clone();
                    let true = (Config::acceptMetaModelicaGrammar()?) else { return Err("pattern mismatch") };
                    let true = (List::all(metamodelica::AsArg::as_arg(&expl), &move |__a0: metamodelica::Ref<Absyn::Exp>| -> metamodelica::Result<_> { ::std::result::Result::Ok(AbsynUtil::isCref(&__a0)) })?) else { return Err("pattern mismatch") };
                    let true = (Types::isTuple(&(Types::getPropType(metamodelica::AsArg::as_arg(&eprop))))) else { return Err("pattern mismatch") };
                    let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(Ceval::cevalIfConstant(cache.clone(), inEnv.clone(), e_1.clone(), eprop.clone(), inImpl, info.clone())?) {
                        (__pa0, __pa1 @ Deref @ DAE::Exp::MATCHEXPRESSION { .. }, __pa2) => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    cache = metamodelica::Own::own(__pa0);
                    e_1 = metamodelica::Own::own(__pa1);
                    eprop = metamodelica::Own::own(__pa2);
                    (cache, e_2) = PrefixUtil::prefixExp(cache.clone(), inEnv, inIH, e_1.clone(), inPre)?;
                    (cache, expl_1, cprops, attrs) = Static::elabExpCrefNoEvalList(cache.clone(), inEnv.clone(), metamodelica::AsArg::as_arg(&expl), inImpl, false, inPre.clone(), info.clone())?;
                    Static::checkAssignmentToInputs(metamodelica::AsArg::as_arg(&expl), attrs.clone(), inEnv, info.clone())?;
                    checkNoDuplicateAssignments(expl_1.clone(), info)?;
                    (cache, expl_2) = PrefixUtil::prefixExpList(cache.clone(), inEnv, inIH, &expl_1, inPre)?;
                    source = ElementSource::addElementSourceFileInfo(inSource.clone(), info.clone());
                    stmt = Algorithm::makeTupleAssignment(expl_2.clone(), cprops.clone(), e_2.clone(), eprop.clone(), initial_, source.clone())?;
                    Ok((cache.clone(), list![stmt.clone()]))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, left, e_1, prop) => {
                    let mut stmt: metamodelica::Ref<DAE::Statement>;
                    let mut ty: metamodelica::Ref<DAE::Type>;
                    let mut pattern: metamodelica::Ref<DAE::Pattern>;
                    let mut source: metamodelica::Ref<DAE::ElementSource>;
                    let mut cache = (*cache).clone();
                    let mut e_1 = (*e_1).clone();
                    let true = (Config::acceptMetaModelicaGrammar()?) else { return Err("pattern mismatch") };
                    ty = Types::getPropType(metamodelica::AsArg::as_arg(&prop));
                    (e_1, ty) = Types::convertTupleToMetaTuple(e_1.clone(), ty.clone())?;
                    (cache, pattern) = Patternm::elabPatternCheckDuplicateBindings(cache.clone(), inEnv, left.clone(), ty.clone(), info)?;
                    source = ElementSource::addElementSourceFileInfo(inSource.clone(), info.clone());
                    stmt = if (Types::isEmptyOrNoRetcall(&ty)) {metamodelica::Ref::new(DAE::Statement::STMT_NORETCALL { exp: e_1.clone(), source: source.clone() })} else {metamodelica::Ref::new(DAE::Statement::STMT_ASSIGN { type_: DAE::T_UNKNOWN_DEFAULT().clone(), exp1: metamodelica::Ref::new(DAE::Exp::PATTERN { pattern: pattern.clone() }), exp: e_1.clone(), source: source.clone() })};
                    Ok((cache.clone(), list![stmt.clone()]))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, Deref @ Absyn::Exp::TUPLE { expressions: expl }, e_1, eprop) => {
                    let mut expl_1: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                    let mut expl_2: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                    let mut cprops: metamodelica::List<DAE::Properties>;
                    let mut eprops: metamodelica::List<DAE::Properties>;
                    let mut attrs: metamodelica::List<metamodelica::Ref<DAE::Attributes>>;
                    let mut source: metamodelica::Ref<DAE::ElementSource>;
                    let mut cache = (*cache).clone();
                    let mut e_1 = (*e_1).clone();
                    let mut eprop = (*eprop).clone();
                    let mut stmts: metamodelica::List<metamodelica::Ref<DAE::Statement>> = stmts.clone();
                    let (__pa0, __pa2, __pa1, __pa3) = ::match_deref::match_deref! { match &(Ceval::cevalIfConstant(cache.clone(), inEnv.clone(), e_1.clone(), eprop.clone(), inImpl, info.clone())?) {
                        (__pa0, __pa2 @ Deref @ DAE::Exp::TUPLE { PR: __pa1 }, __pa3) => (__pa0.clone(), __pa2.clone(), __pa1.clone(), __pa3.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    cache = metamodelica::Own::own(__pa0);
                    expl_1 = metamodelica::Own::own(__pa1);
                    e_1 = metamodelica::Own::own(__pa2);
                    eprop = metamodelica::Own::own(__pa3);
                    (cache, expl_2, cprops, attrs) = Static::elabExpCrefNoEvalList(cache.clone(), inEnv.clone(), metamodelica::AsArg::as_arg(&expl), inImpl, false, inPre.clone(), info.clone())?;
                    Static::checkAssignmentToInputs(metamodelica::AsArg::as_arg(&expl), attrs.clone(), inEnv, info.clone())?;
                    checkNoDuplicateAssignments(expl_2.clone(), info)?;
                    (cache, expl_2) = PrefixUtil::prefixExpList(cache.clone(), inEnv, inIH, &expl_2, inPre)?;
                    eprops = Types::propTuplePropList(metamodelica::AsArg::as_arg(&eprop))?;
                    source = ElementSource::addElementSourceFileInfo(inSource.clone(), info.clone());
                    stmts = Algorithm::makeAssignmentsList(&expl_2, &cprops, &expl_1, &eprops, &(DAE::dummyAttrVar().clone()), initial_, &source)?;
                    Ok(((cache.clone(), stmts.clone()), stmts.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            stmts = __wb0;
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, e @ Deref @ Absyn::Exp::TUPLE { expressions: expl }, _, _) => {
                    let mut s: ArcStr;
                    let false = (List::all(metamodelica::AsArg::as_arg(&expl), &move |__a0: metamodelica::Ref<Absyn::Exp>| -> metamodelica::Result<_> { ::std::result::Result::Ok(AbsynUtil::isCref(&__a0)) })?) else { return Err("pattern mismatch") };
                    s = Dump::printExpStr(e.clone())?;
                    Error::addSourceMessage(&(Error::TUPLE_ASSIGN_CREFS_ONLY.clone()), list![s.clone()], info)?;
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, e1 @ Deref @ Absyn::Exp::TUPLE { expressions: expl }, _, prop2) => {
                    let mut prop1: DAE::Properties;
                    let mut e_1: metamodelica::Ref<DAE::Exp>;
                    let mut lt: metamodelica::Ref<DAE::Type>;
                    let mut rt: metamodelica::Ref<DAE::Type>;
                    let mut lhs_str: ArcStr;
                    let mut rhs_str: ArcStr;
                    let mut lt_str: ArcStr;
                    let mut rt_str: ArcStr;
                    let mut cache = (*cache).clone();
                    ::match_deref::match_deref! { match &(inRhsNoComment.clone()) {
                        Deref @ Absyn::Exp::CALL { .. } => (),
                        _ => return Err("pattern mismatch"),
                    } };
                    let true = (List::all(metamodelica::AsArg::as_arg(&expl), &move |__a0: metamodelica::Ref<Absyn::Exp>| -> metamodelica::Result<_> { ::std::result::Result::Ok(AbsynUtil::isCref(&__a0)) })?) else { return Err("pattern mismatch") };
                    (cache, e_1, prop1) = Static::elabExpLHS(cache.clone(), inEnv.clone(), e1.clone(), inImpl, false, inPre.clone(), info.clone())?;
                    lt = Types::getPropType(&prop1);
                    rt = Types::getPropType(metamodelica::AsArg::as_arg(&prop2));
                    let false = (Types::subtype(lt.clone(), rt.clone(), true)) else { return Err("pattern mismatch") };
                    lhs_str = ExpressionBasics::printExpStr(e_1.clone())?;
                    rhs_str = Dump::printExpStr(inRhs.clone())?;
                    lt_str = TypesDump::unparseTypeNoAttr(&lt)?;
                    rt_str = TypesDump::unparseTypeNoAttr(&rt)?;
                    Types::typeErrorSanityCheck(lt_str.clone(), &rt_str, info)?;
                    Error::addSourceMessage(&(Error::ASSIGN_TYPE_MISMATCH_ERROR.clone()), list![lhs_str.clone(), rhs_str.clone(), lt_str.clone(), rt_str.clone()], info)?;
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Deref @ Absyn::Exp::TUPLE { expressions: expl }, e_1, _) => {
                    let mut s: ArcStr;
                    let true = (List::all(metamodelica::AsArg::as_arg(&expl), &move |__a0: metamodelica::Ref<Absyn::Exp>| -> metamodelica::Result<_> { ::std::result::Result::Ok(AbsynUtil::isCref(&__a0)) })?) else { return Err("pattern mismatch") };
                    if '__try0: {
                        ::match_deref::match_deref! { match &(inRhsNoComment.clone()) {
                            Deref @ Absyn::Exp::CALL { .. } => (),
                            _ => break '__try0 Err::<_, _>("pattern mismatch"),
                        } };
                        Ok::<(), &'static str>(())
                    }.is_ok() { return Err("failure(): body succeeded") }
                    s = ExpressionBasics::printExpStr(e_1.clone())?;
                    Error::addSourceMessage(&(Error::TUPLE_ASSIGN_FUNCALL_ONLY.clone()), list![s.clone()], info)?;
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
                    let mut s1: ArcStr;
                    let mut s2: ArcStr;
                    let true = (numError == Error::getNumErrorMessages()) else { return Err("pattern mismatch") };
                    s1 = Dump::printExpStr(var.clone())?;
                    s2 = ExpressionBasics::printExpStr(value.clone())?;
                    Error::addSourceMessage(&(Error::ASSIGN_UNKNOWN_ERROR.clone()), list![s1.clone(), s2.clone()], info)?;
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((outCache, stmts))
}

fn checkNoDuplicateAssignments(
    mut inExps: metamodelica::List<metamodelica::Ref<DAE::Exp>>,
    mut info: &SourceInfo,
) -> Result<()> {
    let mut exp: metamodelica::Ref<DAE::Exp>;
    let mut exps: metamodelica::List<metamodelica::Ref<DAE::Exp>> = inExps;
    while !((exps).is_empty()) {
        let (__pa0, __pa1) = ::match_deref::match_deref! { match &(exps) {
            Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
            _ => return Err("pattern mismatch"),
        } };
        exp = metamodelica::Own::own(__pa0);
        exps = metamodelica::Own::own(__pa1);
        if Expression::isWild(&exp) {
            continue;
        } else if listMember(exp.clone(), exps.clone()) {
            Error::addSourceMessage(
                &(Error::DUPLICATE_DEFINITION.clone()),
                list![ExpressionBasics::printExpStr(exp)?],
                info,
            )?;
            return Err("fail");
        }
    }
    Ok(())
}

fn getIteratorType<'__b>(
    mut ty: &'__b metamodelica::Ref<DAE::Type>,
    mut id: &'__b ArcStr,
    mut info: &'__b SourceInfo,
) -> Result<metamodelica::Ref<DAE::Type>> {
    let mut oty: metamodelica::Ref<DAE::Type>;
    oty = (::match_deref::match_deref! { match ty {
        Deref @ DAE::Type::T_ARRAY { ty: Deref @ DAE::Type::T_ARRAY { .. }, .. } => {
            let mut r#str: ArcStr;
            r#str = TypesDump::unparseType(ty.clone())?;
            Error::addSourceMessage(&(Error::ITERATOR_NON_ARRAY.clone()), list![id.clone(), r#str], info)?;
            return Err("fail")
        },
        Deref @ DAE::Type::T_ARRAY { ty: __esc_oty, .. } => {
            oty = (*__esc_oty).clone();
            oty.clone()
        },
        Deref @ DAE::Type::T_METALIST { ty: __esc_oty } => {
            oty = (*__esc_oty).clone();
            Types::boxIfUnboxedType(oty.clone())
        },
        Deref @ DAE::Type::T_METAARRAY { ty: __esc_oty } => {
            oty = (*__esc_oty).clone();
            Types::boxIfUnboxedType(oty.clone())
        },
        Deref @ DAE::Type::T_METATYPE { ty: __esc_oty } => {
            oty = (*__esc_oty).clone();
            getIteratorType(var_field!((**ty).ty, DAE::Type::T_METATYPE), id, info)?
        },
        _ => {
            let mut r#str: ArcStr;
            r#str = TypesDump::unparseType(ty.clone())?;
            Error::addSourceMessage(&(Error::ITERATOR_NON_ARRAY.clone()), list![id.clone(), r#str], info)?;
            return Err("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(oty)
}

fn instParForStatement(
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
    mut inIH: metamodelica::List<InnerOuter::TopInstance>,
    mut inPrefix: DAE::Prefix,
    mut inState: &ClassInf::State,
    mut inForStatement: metamodelica::Ref<SCode::Statement>,
    mut inSource: metamodelica::Ref<DAE::ElementSource>,
    mut inInitial: SCode::Initial,
    mut inImpl: bool,
    mut inUnrollLoops: bool,
) -> Result<(FCore::Cache, metamodelica::List<metamodelica::Ref<DAE::Statement>>)> {
    let mut outCache: FCore::Cache;
    let mut outStatements: metamodelica::List<metamodelica::Ref<DAE::Statement>>;
    let mut iterator: ArcStr;
    let mut oarange: Option<metamodelica::Ref<Absyn::Exp>>;
    let mut arange: metamodelica::Ref<Absyn::Exp>;
    let mut range: metamodelica::Ref<DAE::Exp>;
    let mut prop: DAE::Properties;
    let mut body: metamodelica::List<metamodelica::Ref<SCode::Statement>>;
    let mut info: SourceInfo;
    let mut iter_crefs: metamodelica::List<(metamodelica::Ref<Absyn::ComponentRef>, i32)>;
    let (__pa0, __pa1, __pa2, __pa3) = ::match_deref::match_deref! { match &(inForStatement.clone()) {
        Deref @ SCode::Statement::ALG_PARFOR { index: __pa0, range: __pa1, parforBody: __pa2, info: __pa3, .. } => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone()),
        _ => return Err("pattern mismatch"),
    } };
    iterator = metamodelica::Own::own(__pa0);
    oarange = metamodelica::Own::own(__pa1);
    body = metamodelica::Own::own(__pa2);
    info = metamodelica::Own::own(__pa3);
    if (oarange).is_some() {
        let __pa4 = ::match_deref::match_deref! { match &(oarange) {
            Some(__pa4) => __pa4.clone(),
            _ => return Err("pattern mismatch"),
        } };
        arange = metamodelica::Own::own(__pa4);
        (outCache, range, prop) = Static::elabExp(
            inCache.clone(),
            inEnv.clone(),
            arange,
            inImpl,
            true,
            inPrefix.clone(),
            info.clone(),
        )?;
    } else {
        iter_crefs = SCodeUtil::findIteratorIndexedCrefsInStatements(&body, iterator.clone(), metamodelica::nil())?;
        (range, prop, outCache) =
            Static::deduceIterationRange(iterator.clone(), &iter_crefs, inEnv.clone(), inCache.clone(), &info)?;
    }
    if containsWhenStatements(&body)? {
        (outCache, outStatements) = unrollForLoop(
            inCache,
            inEnv,
            inIH,
            inPrefix,
            inState,
            iterator,
            range,
            prop,
            body,
            inForStatement,
            info,
            &inSource,
            inInitial,
            inImpl,
            inUnrollLoops,
        )?;
    } else {
        (outCache, outStatements) = instParForStatement_dispatch(
            inCache,
            inEnv,
            inIH,
            inPrefix,
            inState,
            iterator,
            range,
            prop,
            &body,
            info,
            inSource,
            inInitial,
            inImpl,
            inUnrollLoops,
        )?;
    }
    Ok((outCache, outStatements))
}

fn instParForStatement_dispatch(
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
    mut inIH: metamodelica::List<InnerOuter::TopInstance>,
    mut inPrefix: DAE::Prefix,
    mut inState: &ClassInf::State,
    mut inIterator: ArcStr,
    mut inRange: metamodelica::Ref<DAE::Exp>,
    mut inRangeProps: DAE::Properties,
    mut inBody: &metamodelica::List<metamodelica::Ref<SCode::Statement>>,
    mut inInfo: SourceInfo,
    mut inSource: metamodelica::Ref<DAE::ElementSource>,
    mut inInitial: SCode::Initial,
    mut inImpl: bool,
    mut inUnrollLoops: bool,
) -> Result<(FCore::Cache, metamodelica::List<metamodelica::Ref<DAE::Statement>>)> {
    let mut outCache: FCore::Cache = inCache;
    let mut outStatements: metamodelica::List<metamodelica::Ref<DAE::Statement>>;
    let mut ty: metamodelica::Ref<DAE::Type>;
    let mut c: DAE::Const;
    let mut env: FCore::Graph;
    let mut source: metamodelica::Ref<DAE::ElementSource>;
    let mut loop_prl_vars: metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, SourceInfo)>;
    let mut parfor_iter: metamodelica::Ref<DAE::ComponentRef>;
    let mut range: metamodelica::Ref<DAE::Exp>;
    c = Types::getPropConst(inRangeProps.clone())?;
    if Types::isParameterOrConstant(c) {
        if '__try0: {
            let __pa1 = ::match_deref::match_deref! { match &(unwrap_break_err!(Ceval::ceval(outCache.clone(), inEnv.clone(), inRange.clone(), inImpl, Absyn::Msg::MSG { info: inInfo.clone() }, 0), '__try0)) {
                (__pa1, Deref @ Values::Value::ARRAY { valueLst: Deref @ metamodelica::ListNode::Nil, .. }) => __pa1.clone(),
                _ => break '__try0 Err::<_, _>("pattern mismatch"),
            } };
            outCache = metamodelica::Own::own(__pa1);
            outStatements = metamodelica::nil();
            return Ok((outCache, outStatements));
            Ok::<(), &'static str>(())
        }.is_err() {
        }
    }
    ty = Types::getPropType(&inRangeProps);
    ty = getIteratorType(&ty, &inIterator, &inInfo)?;
    (outCache, range) = Ceval::cevalRangeIfConstant(
        outCache,
        inEnv.clone(),
        inRange,
        inRangeProps.clone(),
        inImpl,
        inInfo.clone(),
    );
    (outCache, range) = PrefixUtil::prefixExp(outCache, &inEnv, &inIH, range, &inPrefix)?;
    env = addParForLoopScope(
        inEnv,
        inIterator.clone(),
        ty.clone(),
        openmodelica_frontend_types::SCode::Variability::VAR,
        Some(c),
    )?;
    (outCache, outStatements) = instStatements(
        outCache,
        env.clone(),
        inIH,
        inPrefix,
        inState,
        inBody,
        inSource.clone(),
        inInitial,
        inImpl,
        inUnrollLoops,
    )?;
    loop_prl_vars = collectParallelVariables(metamodelica::nil(), &outStatements)?;
    parfor_iter = metamodelica::Ref::new(DAE::ComponentRef::CREF_IDENT {
        ident: inIterator.clone(),
        identType: ty,
        subscriptLst: metamodelica::nil(),
    });
    (loop_prl_vars, _) = List::deleteMemberOnTrue(parfor_iter, loop_prl_vars, &move |__a0: metamodelica::Ref<
        DAE::ComponentRef,
    >,
                                                                                     __a1: (
        metamodelica::Ref<DAE::ComponentRef>,
        SourceInfo,
    )|
          -> metamodelica::Result<_> {
        ::std::result::Result::Ok(crefInfoListCrefsEqual(__a0, &__a1))
    })?;
    List::map2_0(
        &loop_prl_vars,
        &move |__a0: (metamodelica::Ref<DAE::ComponentRef>, SourceInfo), __a1: FCore::Cache, __a2: FCore::Graph| {
            isCrefParGlobalOrForIterator(&__a0, __a1, __a2)
        },
        outCache.clone(),
        env,
    )?;
    source = ElementSource::addElementSourceFileInfo(inSource, inInfo);
    outStatements = list![Algorithm::makeParFor(
        inIterator,
        range,
        &inRangeProps,
        outStatements,
        loop_prl_vars,
        source
    )?];
    Ok((outCache, outStatements))
}

fn isCrefParGlobalOrForIterator(
    mut inCrefInfo: &(metamodelica::Ref<DAE::ComponentRef>, SourceInfo),
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
) -> Result<()> {
    let () = 'mc: {
        let __mc_input = inCrefInfo;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cref, _) => {
                    let mut prl: SCode::Parallelism;
                    let mut isParglobal: bool;
                    let (_, __t1, _, _, _, _, _, _, _) = Lookup::lookupVar(inCache.clone(), inEnv.clone(), cref.clone())?;
                    let __arc2 = __t1.clone();
                    let DAE::ATTR { parallelism: __pa0, .. } = &*__arc2;
                    prl = metamodelica::Own::own(__pa0);
                    isParglobal = SCodeUtil::parallelismEqual(prl, openmodelica_frontend_types::SCode::Parallelism::PARGLOBAL);
                    let true = (isParglobal) else { return Err("pattern mismatch") };
                    Ok(())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cref, info) => {
                    let mut errorString: ArcStr;
                    errorString = { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*literal!("- Component '")); __mm_s.push_str(&*AbsynUtil::pathString(ComponentReference::crefToPath(metamodelica::AsArg::as_arg(&cref))?, literal!("."), true, false)?); __mm_s.push_str(&*literal!("' is used in a parallel for loop.")); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*literal!("- Parallel for loops can only contain references to parglobal variables.")); ArcStr::from(__mm_s) };
                    Error::addSourceMessage(&(Error::PARMODELICA_ERROR.clone()), list![errorString.clone()], metamodelica::AsArg::as_arg(&info))?;
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

fn crefInfoListCrefsEqual(
    mut inFoundCref: metamodelica::Ref<DAE::ComponentRef>,
    mut inCrefInfos: &(metamodelica::Ref<DAE::ComponentRef>, SourceInfo),
) -> bool {
    let mut outBoolean: bool;
    outBoolean = (::match_deref::match_deref! { match &(inCrefInfos) {
        (cref1, _) => {
            ComponentReferenceBasics::crefEqualWithoutSubs(cref1.clone(), inFoundCref)
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outBoolean
}

fn collectParallelVariables(
    mut inCrefInfos: metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, SourceInfo)>,
    mut inStatments: &metamodelica::List<metamodelica::Ref<DAE::Statement>>,
) -> Result<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, SourceInfo)>> {
    let mut outCrefInfos: metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, SourceInfo)>;
    outCrefInfos = 'mc: {
        let __mc_input = (inCrefInfos.clone(), &**inStatments);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Deref @ metamodelica::ListNode::Nil) => {
                    Ok(inCrefInfos.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (crefInfoList, Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Statement::STMT_ASSIGN { type_: _, exp1, exp: exp2, source: Deref @ DAE::ElementSource { info, .. } }, tail: restStmts }) => {
                    let mut crefInfoList = (*crefInfoList).clone();
                    crefInfoList = collectParallelVariablesinExps(crefInfoList.clone(), &(list![exp1.clone(), exp2.clone()]), metamodelica::AsArg::as_arg(&info))?;
                    crefInfoList = collectParallelVariables(crefInfoList.clone(), metamodelica::AsArg::as_arg(&restStmts))?;
                    Ok(crefInfoList.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (crefInfoList, Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Statement::STMT_FOR { type_: iterType, iter, range: exp1, statementLst: stmtList, source: Deref @ DAE::ElementSource { info, .. }, .. }, tail: restStmts }) => {
                    let mut foundCref: metamodelica::Ref<DAE::ComponentRef>;
                    let mut crefInfoList = (*crefInfoList).clone();
                    crefInfoList = collectParallelVariablesinExps(crefInfoList.clone(), &(list![exp1.clone()]), metamodelica::AsArg::as_arg(&info))?;
                    crefInfoList = collectParallelVariables(crefInfoList.clone(), metamodelica::AsArg::as_arg(&stmtList))?;
                    foundCref = metamodelica::Ref::new(DAE::ComponentRef::CREF_IDENT { ident: iter.clone(), identType: iterType.clone(), subscriptLst: metamodelica::nil() });
                    (crefInfoList, _) = List::deleteMemberOnTrue(foundCref.clone(), crefInfoList.clone(), &move |__a0: metamodelica::Ref<DAE::ComponentRef>, __a1: (metamodelica::Ref<DAE::ComponentRef>, SourceInfo)| -> metamodelica::Result<_> { ::std::result::Result::Ok(crefInfoListCrefsEqual(__a0, &__a1)) })?;
                    crefInfoList = collectParallelVariables(crefInfoList.clone(), metamodelica::AsArg::as_arg(&restStmts))?;
                    Ok(crefInfoList.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (crefInfoList, Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Statement::STMT_IF { exp: exp1, statementLst: stmtList, else_: _, source: Deref @ DAE::ElementSource { info, .. } }, tail: restStmts }) => {
                    let mut crefInfoList = (*crefInfoList).clone();
                    crefInfoList = collectParallelVariablesinExps(crefInfoList.clone(), &(list![exp1.clone()]), metamodelica::AsArg::as_arg(&info))?;
                    crefInfoList = collectParallelVariables(crefInfoList.clone(), metamodelica::AsArg::as_arg(&stmtList))?;
                    crefInfoList = collectParallelVariables(crefInfoList.clone(), metamodelica::AsArg::as_arg(&restStmts))?;
                    Ok(crefInfoList.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (crefInfoList, Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Statement::STMT_WHILE { exp: exp1, statementLst: stmtList, source: Deref @ DAE::ElementSource { info, .. } }, tail: restStmts }) => {
                    let mut crefInfoList = (*crefInfoList).clone();
                    crefInfoList = collectParallelVariablesinExps(crefInfoList.clone(), &(list![exp1.clone()]), metamodelica::AsArg::as_arg(&info))?;
                    crefInfoList = collectParallelVariables(crefInfoList.clone(), metamodelica::AsArg::as_arg(&stmtList))?;
                    crefInfoList = collectParallelVariables(crefInfoList.clone(), metamodelica::AsArg::as_arg(&restStmts))?;
                    Ok(crefInfoList.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (crefInfoList, Deref @ metamodelica::ListNode::Cons { head: _, tail: restStmts }) => {
                    Ok(collectParallelVariables(crefInfoList.clone(), metamodelica::AsArg::as_arg(&restStmts))?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(outCrefInfos)
}

fn collectParallelVariablesinExps(
    mut inCrefInfos: metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, SourceInfo)>,
    mut inExps: &metamodelica::List<metamodelica::Ref<DAE::Exp>>,
    mut inInfo: &SourceInfo,
) -> Result<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, SourceInfo)>> {
    let mut outCrefInfos: metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, SourceInfo)>;
    outCrefInfos = 'mc: {
        let __mc_input = (inCrefInfos.clone(), &**inExps);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Deref @ metamodelica::ListNode::Nil) => {
                    Ok(inCrefInfos.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (crefInfoList, Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Exp::CREF { componentRef: foundCref, ty: _ }, tail: restExps }) => {
                    let mut subscriptLst: metamodelica::List<metamodelica::Ref<DAE::Subscript>>;
                    let mut alreadyInList: bool;
                    let mut crefInfoList = (*crefInfoList).clone();
                    alreadyInList = List::isMemberOnTrue(foundCref.clone(), metamodelica::AsArg::as_arg(&crefInfoList), &move |__a0: metamodelica::Ref<DAE::ComponentRef>, __a1: (metamodelica::Ref<DAE::ComponentRef>, SourceInfo)| -> metamodelica::Result<_> { ::std::result::Result::Ok(crefInfoListCrefsEqual(__a0, &__a1)) })?;
                    crefInfoList = if (alreadyInList) {crefInfoList.clone()} else {metamodelica::cons((foundCref.clone(), inInfo.clone()), crefInfoList.clone())};
                    let __pa0 = ::match_deref::match_deref! { match &(foundCref.clone()) {
                        Deref @ DAE::ComponentRef::CREF_IDENT { ident: _, identType: _, subscriptLst: __pa0 } => __pa0.clone(),
                        _ => return Err("pattern mismatch"),
                    } };
                    subscriptLst = metamodelica::Own::own(__pa0);
                    crefInfoList = collectParallelVariablesInSubscriptList(crefInfoList.clone(), &subscriptLst, inInfo)?;
                    crefInfoList = collectParallelVariablesinExps(crefInfoList.clone(), metamodelica::AsArg::as_arg(&restExps), inInfo)?;
                    Ok(crefInfoList.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                        (crefInfoList, Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Exp::ASUB { exp: exp1, sub: subs }, tail: restExps }) => {
                            let mut expLst1: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                            let mut crefInfoList = (*crefInfoList).clone();
                            expLst1 = ({
                let mut __acc: metamodelica::List<metamodelica::Ref<DAE::Exp>> = metamodelica::nil();
                for mut sub in (subs.clone()).into_iter().cloned() {
                            let __x = Expression::getSubscriptExp(&(sub.clone()))?;
                            __acc = cons(__x, __acc);
                }
                __acc.reverse()
            });
                            crefInfoList = collectParallelVariablesinExps(crefInfoList.clone(), &(metamodelica::cons(exp1.clone(), expLst1.clone())), inInfo)?;
                            crefInfoList = collectParallelVariablesinExps(crefInfoList.clone(), metamodelica::AsArg::as_arg(&restExps), inInfo)?;
                            Ok(crefInfoList.clone())
                        }
                        _ => return Err("nomatch"),
                    }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (crefInfoList, Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Exp::BINARY { exp1, operator: _, exp2 }, tail: restExps }) => {
                    let mut crefInfoList = (*crefInfoList).clone();
                    crefInfoList = collectParallelVariablesinExps(crefInfoList.clone(), &(list![exp1.clone(), exp2.clone()]), inInfo)?;
                    crefInfoList = collectParallelVariablesinExps(crefInfoList.clone(), metamodelica::AsArg::as_arg(&restExps), inInfo)?;
                    Ok(crefInfoList.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (crefInfoList, Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Exp::UNARY { operator: _, exp: exp1 }, tail: restExps }) => {
                    let mut crefInfoList = (*crefInfoList).clone();
                    crefInfoList = collectParallelVariablesinExps(crefInfoList.clone(), &(list![exp1.clone()]), inInfo)?;
                    crefInfoList = collectParallelVariablesinExps(crefInfoList.clone(), metamodelica::AsArg::as_arg(&restExps), inInfo)?;
                    Ok(crefInfoList.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (crefInfoList, Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Exp::LBINARY { exp1, operator: _, exp2 }, tail: restExps }) => {
                    let mut crefInfoList = (*crefInfoList).clone();
                    crefInfoList = collectParallelVariablesinExps(crefInfoList.clone(), &(list![exp1.clone(), exp2.clone()]), inInfo)?;
                    crefInfoList = collectParallelVariablesinExps(crefInfoList.clone(), metamodelica::AsArg::as_arg(&restExps), inInfo)?;
                    Ok(crefInfoList.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (crefInfoList, Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Exp::LUNARY { operator: _, exp: exp1 }, tail: restExps }) => {
                    let mut crefInfoList = (*crefInfoList).clone();
                    crefInfoList = collectParallelVariablesinExps(crefInfoList.clone(), &(list![exp1.clone()]), inInfo)?;
                    crefInfoList = collectParallelVariablesinExps(crefInfoList.clone(), metamodelica::AsArg::as_arg(&restExps), inInfo)?;
                    Ok(crefInfoList.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (crefInfoList, Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Exp::RANGE { ty: _, start: exp1, step: Some(exp2), stop: exp3 }, tail: restExps }) => {
                    let mut crefInfoList = (*crefInfoList).clone();
                    crefInfoList = collectParallelVariablesinExps(crefInfoList.clone(), &(list![exp1.clone(), exp2.clone(), exp3.clone()]), inInfo)?;
                    crefInfoList = collectParallelVariablesinExps(crefInfoList.clone(), metamodelica::AsArg::as_arg(&restExps), inInfo)?;
                    Ok(crefInfoList.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (crefInfoList, Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Exp::RANGE { ty: _, start: exp1, step: None, stop: exp3 }, tail: restExps }) => {
                    let mut crefInfoList = (*crefInfoList).clone();
                    crefInfoList = collectParallelVariablesinExps(crefInfoList.clone(), &(list![exp1.clone(), exp3.clone()]), inInfo)?;
                    crefInfoList = collectParallelVariablesinExps(crefInfoList.clone(), metamodelica::AsArg::as_arg(&restExps), inInfo)?;
                    Ok(crefInfoList.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (crefInfoList, Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Exp::CAST { ty: _, exp: exp1 }, tail: restExps }) => {
                    let mut crefInfoList = (*crefInfoList).clone();
                    crefInfoList = collectParallelVariablesinExps(crefInfoList.clone(), &(list![exp1.clone()]), inInfo)?;
                    crefInfoList = collectParallelVariablesinExps(crefInfoList.clone(), metamodelica::AsArg::as_arg(&restExps), inInfo)?;
                    Ok(crefInfoList.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (crefInfoList, Deref @ metamodelica::ListNode::Cons { head: _, tail: restExps }) => {
                    Ok(collectParallelVariablesinExps(crefInfoList.clone(), metamodelica::AsArg::as_arg(&restExps), inInfo)?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(outCrefInfos)
}

fn collectParallelVariablesInSubscriptList(
    mut inCrefInfos: metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, SourceInfo)>,
    mut inSubscriptLst: &metamodelica::List<metamodelica::Ref<DAE::Subscript>>,
    mut inInfo: &SourceInfo,
) -> Result<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, SourceInfo)>> {
    let mut outCrefInfos: metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, SourceInfo)>;
    outCrefInfos = 'mc: {
        let __mc_input = (inCrefInfos.clone(), &**inSubscriptLst);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Deref @ metamodelica::ListNode::Nil) => {
                    Ok(inCrefInfos.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (crefInfoList, Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Subscript::INDEX { exp: exp1 }, tail: restSubs }) => {
                    let mut crefInfoList = (*crefInfoList).clone();
                    crefInfoList = collectParallelVariablesinExps(crefInfoList.clone(), &(list![exp1.clone()]), inInfo)?;
                    crefInfoList = collectParallelVariablesInSubscriptList(crefInfoList.clone(), metamodelica::AsArg::as_arg(&restSubs), inInfo)?;
                    Ok(crefInfoList.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (crefInfoList, Deref @ metamodelica::ListNode::Cons { head: _, tail: restSubs }) => {
                    Ok(collectParallelVariablesInSubscriptList(crefInfoList.clone(), metamodelica::AsArg::as_arg(&restSubs), inInfo)?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(outCrefInfos)
}

fn checkValidNoRetcall(mut exp: metamodelica::Ref<DAE::Exp>, mut info: &SourceInfo) -> Result<()> {
    let () = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ DAE::Exp::CALL { .. } => {
            ()
        },
        Deref @ DAE::Exp::REDUCTION { .. } => {
            ()
        },
        Deref @ DAE::Exp::TUPLE { PR: Deref @ metamodelica::ListNode::Nil } => {
            ()
        },
        _ => {
            let mut r#str: ArcStr;
            r#str = ExpressionBasics::printExpStr(exp)?;
            Error::addSourceMessage(&(Error::NORETCALL_INVALID_EXP.clone()), list![r#str], info)?;
            return Err("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(())
}
