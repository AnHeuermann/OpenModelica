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

use crate::CevalScript;
use crate::CevalScriptBackend;
use openmodelica_ast::Absyn;
use openmodelica_error::ErrorExt;
use openmodelica_frontend::Ceval;
use openmodelica_frontend::InteractiveTypes;
use openmodelica_frontend::Static;
use openmodelica_frontend_base::ComponentReference;
use openmodelica_frontend_base::Expression;
use openmodelica_frontend_base::ExpressionSimplify;
use openmodelica_frontend_dump::AbsynUtil;
use openmodelica_frontend_dump::FCore;
use openmodelica_frontend_types::ClassInf;
use openmodelica_frontend_types::DAE;
use openmodelica_frontend_types::Values;
use openmodelica_util::Error;
use openmodelica_util::Flags;

pub type Ident = ArcStr;

fn calculateSimulationTimes(
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
    mut inAbsynExpLst: &metamodelica::List<metamodelica::Ref<Absyn::Exp>>,
    mut inAbsynNamedArgLst: metamodelica::List<metamodelica::Ref<Absyn::NamedArg>>,
    mut inImplInst: bool,
    mut inPrefix: DAE::Prefix,
    mut inInfo: SourceInfo,
    mut inSimOpt: &InteractiveTypes::SimulationOptions,
) -> Result<(
    FCore::Cache,
    metamodelica::Ref<DAE::Exp>,
    metamodelica::Ref<DAE::Exp>,
    metamodelica::Ref<DAE::Exp>,
)> {
    let mut outCache: FCore::Cache;
    let mut startTime: metamodelica::Ref<DAE::Exp> =
        <metamodelica::Ref<DAE::Exp> as ::std::default::Default>::default();
    let mut stopTime: metamodelica::Ref<DAE::Exp> = <metamodelica::Ref<DAE::Exp> as ::std::default::Default>::default();
    let mut numberOfIntervals: metamodelica::Ref<DAE::Exp> =
        <metamodelica::Ref<DAE::Exp> as ::std::default::Default>::default();
    (outCache, startTime, stopTime, numberOfIntervals) = 'mc: {
        let __mc_input = (
            inCache,
            inEnv,
            &**inAbsynExpLst,
            inAbsynNamedArgLst,
            inImplInst,
            inPrefix,
            inInfo,
        );
        if let Ok((__v, __wb0, __wb1, __wb2)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, Deref @ metamodelica::ListNode::Cons { head: Deref @ Absyn::Exp::CREF { .. }, tail: Deref @ metamodelica::ListNode::Nil }, args, r#impl, pre, info) => {
                    let mut intervals: i32;
                    let mut rstepTime: metamodelica::Real;
                    let mut rstopTime: metamodelica::Real;
                    let mut rstartTime: metamodelica::Real;
                    let mut cache = (*cache).clone();
                    let mut numberOfIntervals: metamodelica::Ref<DAE::Exp> = numberOfIntervals.clone();
                    let mut startTime: metamodelica::Ref<DAE::Exp> = startTime.clone();
                    let mut stopTime: metamodelica::Ref<DAE::Exp> = stopTime.clone();
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(Static::getOptionalNamedArg(cache.clone(), env.clone(), r#impl.clone(), &(literal!("stepSize")), DAE::T_REAL_DEFAULT().clone(), metamodelica::AsArg::as_arg(&args), metamodelica::Ref::new(DAE::Exp::ICONST { integer: 0 }), pre.clone(), info.clone())) {
                        (__pa0, Deref @ DAE::Exp::RCONST { real: __pa1 }) => (__pa0.clone(), __pa1.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    cache = metamodelica::Own::own(__pa0);
                    rstepTime = metamodelica::Own::own(__pa1);
                    let (__pa3, __pa5, __pa4) = ::match_deref::match_deref! { match &(Static::getOptionalNamedArg(cache.clone(), env.clone(), r#impl.clone(), &(literal!("startTime")), DAE::T_REAL_DEFAULT().clone(), metamodelica::AsArg::as_arg(&args), CevalScriptBackend::getSimulationOption(inSimOpt, literal!("startTime"))?, pre.clone(), info.clone())) {
                        (__pa3, __pa5 @ Deref @ DAE::Exp::RCONST { real: __pa4 }) => (__pa3.clone(), __pa5.clone(), __pa4.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    cache = metamodelica::Own::own(__pa3);
                    rstartTime = metamodelica::Own::own(__pa4);
                    startTime = metamodelica::Own::own(__pa5);
                    let (__pa7, __pa9, __pa8) = ::match_deref::match_deref! { match &(Static::getOptionalNamedArg(cache.clone(), env.clone(), r#impl.clone(), &(literal!("stopTime")), DAE::T_REAL_DEFAULT().clone(), metamodelica::AsArg::as_arg(&args), CevalScriptBackend::getSimulationOption(inSimOpt, literal!("stopTime"))?, pre.clone(), info.clone())) {
                        (__pa7, __pa9 @ Deref @ DAE::Exp::RCONST { real: __pa8 }) => (__pa7.clone(), __pa9.clone(), __pa8.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    cache = metamodelica::Own::own(__pa7);
                    rstopTime = metamodelica::Own::own(__pa8);
                    stopTime = metamodelica::Own::own(__pa9);
                    intervals = ((metamodelica::real_div_checked((rstopTime - rstartTime), rstepTime)?).0.floor() as i32);
                    numberOfIntervals = metamodelica::Ref::new(DAE::Exp::ICONST { integer: intervals });
                    Ok(((cache.clone(), startTime.clone(), stopTime.clone(), numberOfIntervals.clone()), numberOfIntervals.clone(), startTime.clone(), stopTime.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            numberOfIntervals = __wb0;
            startTime = __wb1;
            stopTime = __wb2;
            break 'mc __v;
        }
        if let Ok((__v, __wb0, __wb1, __wb2)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, Deref @ metamodelica::ListNode::Cons { head: Deref @ Absyn::Exp::CREF { .. }, tail: Deref @ metamodelica::ListNode::Nil }, args, r#impl, pre, info) => {
                    let mut cache = (*cache).clone();
                    let mut numberOfIntervals: metamodelica::Ref<DAE::Exp> = numberOfIntervals.clone();
                    let mut startTime: metamodelica::Ref<DAE::Exp> = startTime.clone();
                    let mut stopTime: metamodelica::Ref<DAE::Exp> = stopTime.clone();
                    (cache, startTime) = Static::getOptionalNamedArg(cache.clone(), env.clone(), r#impl.clone(), &(literal!("startTime")), DAE::T_REAL_DEFAULT().clone(), metamodelica::AsArg::as_arg(&args), CevalScriptBackend::getSimulationOption(inSimOpt, literal!("startTime"))?, pre.clone(), info.clone());
                    (cache, stopTime) = Static::getOptionalNamedArg(cache.clone(), env.clone(), r#impl.clone(), &(literal!("stopTime")), DAE::T_REAL_DEFAULT().clone(), metamodelica::AsArg::as_arg(&args), CevalScriptBackend::getSimulationOption(inSimOpt, literal!("stopTime"))?, pre.clone(), info.clone());
                    (cache, numberOfIntervals) = Static::getOptionalNamedArg(cache.clone(), env.clone(), r#impl.clone(), &(literal!("numberOfIntervals")), DAE::T_INTEGER_DEFAULT().clone(), metamodelica::AsArg::as_arg(&args), CevalScriptBackend::getSimulationOption(inSimOpt, literal!("numberOfIntervals"))?, pre.clone(), info.clone());
                    Ok(((cache.clone(), startTime.clone(), stopTime.clone(), numberOfIntervals.clone()), numberOfIntervals.clone(), startTime.clone(), stopTime.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            numberOfIntervals = __wb0;
            startTime = __wb1;
            stopTime = __wb2;
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((outCache, startTime, stopTime, numberOfIntervals))
}

pub(crate) fn getSimulationArguments(
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
    mut inAbsynExpLst: &metamodelica::List<metamodelica::Ref<Absyn::Exp>>,
    mut inAbsynNamedArgLst: metamodelica::List<metamodelica::Ref<Absyn::NamedArg>>,
    mut inImplInst: bool,
    mut inPrefix: DAE::Prefix,
    mut callName: ArcStr,
    mut inInfo: SourceInfo,
    mut defaultOption: Option<InteractiveTypes::SimulationOptions>,
) -> Result<(FCore::Cache, metamodelica::List<metamodelica::Ref<DAE::Exp>>)> {
    let mut outCache: FCore::Cache;
    let mut outSimulationArguments: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
    (outCache, outSimulationArguments) = (::match_deref::match_deref! { match inAbsynExpLst {
        Deref @ metamodelica::ListNode::Cons { head: crexp, tail: Deref @ metamodelica::ListNode::Nil } => {
            let mut cache = inCache.clone();
            let mut env = inEnv.clone();
            let mut args = inAbsynNamedArgLst.clone();
            let mut r#impl = inImplInst;
            let mut pre = inPrefix.clone();
            let mut info = inInfo.clone();
            let mut cname_str: ArcStr;
            let mut className: metamodelica::Ref<Absyn::Path>;
            let mut exp: metamodelica::Ref<DAE::Exp>;
            let mut startTime: metamodelica::Ref<DAE::Exp>;
            let mut stopTime: metamodelica::Ref<DAE::Exp>;
            let mut numberOfIntervals: metamodelica::Ref<DAE::Exp>;
            let mut tolerance: metamodelica::Ref<DAE::Exp>;
            let mut method: metamodelica::Ref<DAE::Exp>;
            let mut cflags: metamodelica::Ref<DAE::Exp>;
            let mut simflags: metamodelica::Ref<DAE::Exp>;
            let mut resimulateExecutable: metamodelica::Ref<DAE::Exp>;
            let mut fileNamePrefix: metamodelica::Ref<DAE::Exp>;
            let mut options: metamodelica::Ref<DAE::Exp>;
            let mut outputFormat: metamodelica::Ref<DAE::Exp>;
            let mut variableFilter: metamodelica::Ref<DAE::Exp>;
            let mut defaulSimOpt: InteractiveTypes::SimulationOptions;
            let mut v: metamodelica::Ref<Values::Value>;
            checkSimulationArguments(&args, callName.clone(), &info)?;
            exp = Static::elabCodeExp(metamodelica::AsArg::as_arg(&crexp), &cache, &env, openmodelica_frontend_types::DAE::CodeType::C_TYPENAME, &info)?;
            (cache, v) = Ceval::ceval(cache, env.clone(), exp, true, Absyn::Msg::MSG { info: info.clone() }, 0)?;
            let __pa0 = ::match_deref::match_deref! { match &(CevalScript::evalCodeTypeName(v, env.clone())) {
                Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: __pa0 } } => __pa0.clone(),
                _ => return Err("pattern mismatch"),
            } };
            className = metamodelica::Own::own(__pa0);
            cname_str = AbsynUtil::pathString(AbsynUtil::unqotePathIdents(&className)?, literal!("."), true, false)?;
            defaulSimOpt = CevalScriptBackend::buildSimulationOptionsFromModelExperimentAnnotation(className.clone(), cname_str, defaultOption)?;
            (cache, startTime, stopTime, numberOfIntervals) = calculateSimulationTimes(inCache, inEnv, inAbsynExpLst, inAbsynNamedArgLst, r#impl, inPrefix, inInfo, &defaulSimOpt)?;
            (cache, tolerance) = Static::getOptionalNamedArg(cache, env.clone(), r#impl, &(literal!("tolerance")), DAE::T_REAL_DEFAULT().clone(), &args, CevalScriptBackend::getSimulationOption(&defaulSimOpt, literal!("tolerance"))?, pre.clone(), info.clone());
            (cache, method) = Static::getOptionalNamedArg(cache, env.clone(), r#impl, &(literal!("method")), DAE::T_STRING_DEFAULT().clone(), &args, CevalScriptBackend::getSimulationOption(&defaulSimOpt, literal!("method"))?, pre.clone(), info.clone());
            (cache, fileNamePrefix) = Static::getOptionalNamedArg(cache, env.clone(), r#impl, &(literal!("fileNamePrefix")), DAE::T_STRING_DEFAULT().clone(), &args, CevalScriptBackend::getSimulationOption(&defaulSimOpt, literal!("fileNamePrefix"))?, pre.clone(), info.clone());
            (cache, options) = Static::getOptionalNamedArg(cache, env.clone(), r#impl, &(literal!("options")), DAE::T_STRING_DEFAULT().clone(), &args, CevalScriptBackend::getSimulationOption(&defaulSimOpt, literal!("options"))?, pre.clone(), info.clone());
            (cache, outputFormat) = Static::getOptionalNamedArg(cache, env.clone(), r#impl, &(literal!("outputFormat")), DAE::T_STRING_DEFAULT().clone(), &args, CevalScriptBackend::getSimulationOption(&defaulSimOpt, literal!("outputFormat"))?, pre.clone(), info.clone());
            (cache, variableFilter) = Static::getOptionalNamedArg(cache, env.clone(), r#impl, &(literal!("variableFilter")), DAE::T_STRING_DEFAULT().clone(), &args, CevalScriptBackend::getSimulationOption(&defaulSimOpt, literal!("variableFilter"))?, pre.clone(), info.clone());
            (cache, cflags) = Static::getOptionalNamedArg(cache, env.clone(), r#impl, &(literal!("cflags")), DAE::T_STRING_DEFAULT().clone(), &args, CevalScriptBackend::getSimulationOption(&defaulSimOpt, literal!("cflags"))?, pre.clone(), info.clone());
            (cache, simflags) = Static::getOptionalNamedArg(cache, env.clone(), r#impl, &(literal!("simflags")), DAE::T_STRING_DEFAULT().clone(), &args, CevalScriptBackend::getSimulationOption(&defaulSimOpt, literal!("simflags"))?, pre.clone(), info.clone());
            (cache, resimulateExecutable) = Static::getOptionalNamedArg(cache, env, r#impl, &(literal!("resimulateExecutable")), DAE::T_STRING_DEFAULT().clone(), &args, metamodelica::Ref::new(DAE::Exp::SCONST { string: literal!("") }), pre, info);
            (cache, listAppend(list![metamodelica::Ref::new(DAE::Exp::CODE { code: metamodelica::Ref::new(Absyn::CodeNode::C_TYPENAME { path: className }), ty: DAE::T_UNKNOWN_DEFAULT().clone() }), startTime, stopTime, numberOfIntervals, tolerance, method, fileNamePrefix, options, outputFormat, variableFilter, cflags, simflags], if (metamodelica::stringEq(&callName, &(literal!("simulate")))) {list![resimulateExecutable]} else {metamodelica::nil()}))
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok((outCache, outSimulationArguments))
}

pub(crate) static VALID_SIMULATE_ARGS: std::sync::LazyLock<metamodelica::List<ArcStr>> =
    std::sync::LazyLock::new(|| {
        list![
            literal!("startTime"),
            literal!("stopTime"),
            literal!("numberOfIntervals"),
            literal!("stepSize"),
            literal!("tolerance"),
            literal!("method"),
            literal!("fileNamePrefix"),
            literal!("options"),
            literal!("outputFormat"),
            literal!("variableFilter"),
            literal!("cflags"),
            literal!("simflags"),
            literal!("resimulateExecutable")
        ]
    });

pub(crate) fn checkSimulationArguments(
    mut args: &metamodelica::List<metamodelica::Ref<Absyn::NamedArg>>,
    mut callName: ArcStr,
    mut info: &SourceInfo,
) -> Result<()> {
    for mut arg in &**args {
        if !(listMember(arg.argName.clone(), VALID_SIMULATE_ARGS.clone())) {
            Error::addSourceMessage(
                &(Error::NO_SUCH_PARAMETER.clone()),
                list![callName.clone(), arg.argName.clone()],
                info,
            )?;
            return Err("fail");
        }
    }
    Ok(())
}

pub(crate) fn elabCallInteractive(
    mut cache: FCore::Cache,
    mut env: FCore::Graph,
    mut r#fn: metamodelica::Ref<Absyn::ComponentRef>,
    mut args: metamodelica::List<metamodelica::Ref<Absyn::Exp>>,
    mut nargs: metamodelica::List<metamodelica::Ref<Absyn::NamedArg>>,
    mut r#impl: bool,
    mut pre: DAE::Prefix,
    mut info: SourceInfo,
) -> Result<(FCore::Cache, metamodelica::Ref<DAE::Exp>, DAE::Properties)> {
    let mut cache: FCore::Cache = cache;
    let mut e: metamodelica::Ref<DAE::Exp>;
    let mut prop: DAE::Properties;
    let mut handles: metamodelica::List<i32>;
    if Flags::getConfigBool(Flags::BUILDING_MODEL.clone())? {
        ErrorExt::delCheckpoint(literal!("elabCall_InteractiveFunction"));
        return Err("fail");
    }
    handles = ErrorExt::popCheckPoint(literal!("elabCall_InteractiveFunction"));
    match '__try0: {
        ErrorExt::setCheckpoint(literal!("elabCall_InteractiveFunction1"));
        (cache, e, prop) = unwrap_break_err!(elabCallInteractive_work(cache.clone(), env.clone(), r#fn.clone(), args.clone(), nargs.clone(), r#impl, pre.clone(), info.clone()), '__try0);
        ErrorExt::delCheckpoint(literal!("elabCall_InteractiveFunction1"));
        Ok::<_, &'static str>((cache.clone(), e.clone(), prop.clone()))
    } {
        Ok((__try0_o0, __try0_o1, __try0_o2)) => {
            cache = __try0_o0;
            e = __try0_o1;
            prop = __try0_o2;
        }
        Err(__try0_err) => {
            ErrorExt::rollBack(literal!("elabCall_InteractiveFunction1"));
            ErrorExt::pushMessages(handles.clone());
            return Err(__try0_err);
        }
    }
    ErrorExt::freeMessages(handles);
    Ok((cache, e, prop))
}

fn elabCallInteractive_work(
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
    mut inComponentRef: metamodelica::Ref<Absyn::ComponentRef>,
    mut inExps: metamodelica::List<metamodelica::Ref<Absyn::Exp>>,
    mut inNamedArgs: metamodelica::List<metamodelica::Ref<Absyn::NamedArg>>,
    mut inImplInst: bool,
    mut inPrefix: DAE::Prefix,
    mut info: SourceInfo,
) -> Result<(FCore::Cache, metamodelica::Ref<DAE::Exp>, DAE::Properties)> {
    let mut outCache: FCore::Cache;
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    let mut outProperties: DAE::Properties;
    (outCache, outExp, outProperties) = 'mc: {
        let __mc_input = (
            inCache,
            inEnv,
            inComponentRef,
            &*inExps,
            inNamedArgs.clone(),
            inImplInst,
            inPrefix.clone(),
        );
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, cr2 @ Deref @ Absyn::ComponentRef::CREF_IDENT { .. }, _, _, r#impl, _) => {
                    let mut cr: metamodelica::Ref<Absyn::ComponentRef>;
                    let mut exp_1: metamodelica::Ref<DAE::Exp>;
                    let mut prop: DAE::Properties;
                    let mut cache = (*cache).clone();
                    ErrorExt::setCheckpoint(literal!("Scripting"));
                    cr = AbsynUtil::joinCrefs(&(metamodelica::Ref::new(Absyn::ComponentRef::CREF_QUAL { name: literal!("OpenModelica"), subscripts: metamodelica::nil(), componentRef: metamodelica::Ref::new(Absyn::ComponentRef::CREF_IDENT { name: literal!("Scripting"), subscripts: metamodelica::nil() }) })), cr2.clone())?;
                    (cache, exp_1, prop) = Static::elabExp(cache.clone(), env.clone(), metamodelica::Ref::new(Absyn::Exp::CALL { function_: cr.clone(), functionArgs: metamodelica::Ref::new(Absyn::FunctionArgs::FUNCTIONARGS { args: inExps.clone(), argNames: inNamedArgs.clone() }), typeVars: metamodelica::nil() }), r#impl.clone(), false, inPrefix.clone(), info.clone())?;
                    ErrorExt::delCheckpoint(literal!("Scripting"));
                    Ok((cache.clone(), exp_1.clone(), prop.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, Deref @ Absyn::ComponentRef::CREF_IDENT { .. }, _, _, _, _) => {
                    ErrorExt::rollBack(literal!("Scripting"));
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, Deref @ Absyn::ComponentRef::CREF_IDENT { name: Deref @ "translateModel", .. }, Deref @ metamodelica::ListNode::Cons { head: Deref @ Absyn::Exp::CREF { .. }, tail: Deref @ metamodelica::ListNode::Nil }, args, _, _) => {
                    let mut simulationArgs: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                    let mut cache = (*cache).clone();
                    (cache, simulationArgs) = getSimulationArguments(cache.clone(), env.clone(), &inExps, args.clone(), inImplInst, inPrefix.clone(), literal!("translateModel"), info.clone(), None)?;
                    Ok((cache.clone(), Expression::makePureBuiltinCall(literal!("translateModel"), simulationArgs.clone(), DAE::T_STRING_DEFAULT().clone()), DAE::Properties::PROP { type_: DAE::T_STRING_DEFAULT().clone(), constFlag: openmodelica_frontend_types::DAE::Const::C_VAR }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, Deref @ Absyn::ComponentRef::CREF_IDENT { name: Deref @ "modelEquationsUC", .. }, Deref @ metamodelica::ListNode::Cons { head: Deref @ Absyn::Exp::CREF { componentRef: cr }, tail: Deref @ metamodelica::ListNode::Nil }, args, r#impl, pre) => {
                    let mut cr_1: metamodelica::Ref<DAE::ComponentRef>;
                    let mut outputFile: metamodelica::Ref<DAE::Exp>;
                    let mut dumpExtractionSteps: metamodelica::Ref<DAE::Exp>;
                    let mut className: metamodelica::Ref<Absyn::Path>;
                    let mut cache = (*cache).clone();
                    (cache, cr_1) = Static::elabUntypedCref(metamodelica::AsArg::as_arg(&cache), metamodelica::AsArg::as_arg(&env), metamodelica::AsArg::as_arg(&cr), r#impl.clone(), metamodelica::AsArg::as_arg(&pre), &info)?;
                    className = ComponentReference::crefToPathIgnoreSubs(&cr_1)?;
                    (cache, outputFile) = Static::getOptionalNamedArg(cache.clone(), env.clone(), r#impl.clone(), &(literal!("outputFile")), DAE::T_STRING_DEFAULT().clone(), metamodelica::AsArg::as_arg(&args), metamodelica::Ref::new(DAE::Exp::SCONST { string: literal!("") }), pre.clone(), info.clone());
                    (cache, dumpExtractionSteps) = Static::getOptionalNamedArg(cache.clone(), env.clone(), r#impl.clone(), &(literal!("dumpSteps")), DAE::T_BOOL_DEFAULT().clone(), metamodelica::AsArg::as_arg(&args), metamodelica::Ref::new(DAE::Exp::BCONST { bool: false }), pre.clone(), info.clone());
                    Ok((cache.clone(), Expression::makePureBuiltinCall(literal!("modelEquationsUC"), list![metamodelica::Ref::new(DAE::Exp::CODE { code: metamodelica::Ref::new(Absyn::CodeNode::C_TYPENAME { path: className.clone() }), ty: DAE::T_UNKNOWN_DEFAULT().clone() }), outputFile.clone(), dumpExtractionSteps.clone()], DAE::T_STRING_DEFAULT().clone()), DAE::Properties::PROP { type_: DAE::T_STRING_DEFAULT().clone(), constFlag: openmodelica_frontend_types::DAE::Const::C_VAR }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, Deref @ Absyn::ComponentRef::CREF_IDENT { name: Deref @ "translateModelCPP", .. }, Deref @ metamodelica::ListNode::Cons { head: Deref @ Absyn::Exp::CREF { componentRef: cr }, tail: Deref @ metamodelica::ListNode::Nil }, args, r#impl, pre) => {
                    let mut cname_str: Ident;
                    let mut filenameprefix: metamodelica::Ref<DAE::Exp>;
                    let mut recordtype: metamodelica::Ref<DAE::Type>;
                    let mut className: metamodelica::Ref<Absyn::Path>;
                    let mut cache = (*cache).clone();
                    className = AbsynUtil::crefToPath(metamodelica::AsArg::as_arg(&cr))?;
                    cname_str = AbsynUtil::pathString(className.clone(), literal!("."), true, false)?;
                    (cache, filenameprefix) = Static::getOptionalNamedArg(cache.clone(), env.clone(), r#impl.clone(), &(literal!("fileNamePrefix")), DAE::T_STRING_DEFAULT().clone(), metamodelica::AsArg::as_arg(&args), metamodelica::Ref::new(DAE::Exp::SCONST { string: cname_str.clone() }), pre.clone(), info.clone());
                    recordtype = metamodelica::Ref::new(DAE::Type::T_COMPLEX { complexClassType: ClassInf::State::RECORD { path: metamodelica::Ref::new(Absyn::Path::IDENT { name: literal!("SimulationObject") }) }, varLst: list![metamodelica::Ref::new(DAE::Var { name: literal!("flatClass"), attributes: DAE::dummyAttrVar().clone(), ty: DAE::T_STRING_DEFAULT().clone(), binding: openmodelica_frontend_types::DAE::Binding::interned_UNBOUND(), bind_from_outside: false, constOfForIteratorRange: None }), metamodelica::Ref::new(DAE::Var { name: literal!("exeFile"), attributes: DAE::dummyAttrVar().clone(), ty: DAE::T_STRING_DEFAULT().clone(), binding: openmodelica_frontend_types::DAE::Binding::interned_UNBOUND(), bind_from_outside: false, constOfForIteratorRange: None })], equalityConstraint: None, usedExternally: false });
                    Ok((cache.clone(), Expression::makePureBuiltinCall(literal!("translateModelCPP"), list![metamodelica::Ref::new(DAE::Exp::CODE { code: metamodelica::Ref::new(Absyn::CodeNode::C_TYPENAME { path: className.clone() }), ty: DAE::T_UNKNOWN_DEFAULT().clone() }), filenameprefix.clone()], DAE::T_STRING_DEFAULT().clone()), DAE::Properties::PROP { type_: recordtype.clone(), constFlag: openmodelica_frontend_types::DAE::Const::C_VAR }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, Deref @ Absyn::ComponentRef::CREF_IDENT { name: Deref @ "translateModelXML", .. }, Deref @ metamodelica::ListNode::Cons { head: Deref @ Absyn::Exp::CREF { componentRef: cr }, tail: Deref @ metamodelica::ListNode::Nil }, args, r#impl, pre) => {
                    let mut cname_str: Ident;
                    let mut filenameprefix: metamodelica::Ref<DAE::Exp>;
                    let mut recordtype: metamodelica::Ref<DAE::Type>;
                    let mut className: metamodelica::Ref<Absyn::Path>;
                    let mut cache = (*cache).clone();
                    className = AbsynUtil::crefToPath(metamodelica::AsArg::as_arg(&cr))?;
                    cname_str = AbsynUtil::pathString(className.clone(), literal!("."), true, false)?;
                    (cache, filenameprefix) = Static::getOptionalNamedArg(cache.clone(), env.clone(), r#impl.clone(), &(literal!("fileNamePrefix")), DAE::T_STRING_DEFAULT().clone(), metamodelica::AsArg::as_arg(&args), metamodelica::Ref::new(DAE::Exp::SCONST { string: cname_str.clone() }), pre.clone(), info.clone());
                    recordtype = metamodelica::Ref::new(DAE::Type::T_COMPLEX { complexClassType: ClassInf::State::RECORD { path: metamodelica::Ref::new(Absyn::Path::IDENT { name: literal!("SimulationObject") }) }, varLst: list![metamodelica::Ref::new(DAE::Var { name: literal!("flatClass"), attributes: DAE::dummyAttrVar().clone(), ty: DAE::T_STRING_DEFAULT().clone(), binding: openmodelica_frontend_types::DAE::Binding::interned_UNBOUND(), bind_from_outside: false, constOfForIteratorRange: None }), metamodelica::Ref::new(DAE::Var { name: literal!("exeFile"), attributes: DAE::dummyAttrVar().clone(), ty: DAE::T_STRING_DEFAULT().clone(), binding: openmodelica_frontend_types::DAE::Binding::interned_UNBOUND(), bind_from_outside: false, constOfForIteratorRange: None })], equalityConstraint: None, usedExternally: false });
                    Ok((cache.clone(), Expression::makePureBuiltinCall(literal!("translateModelXML"), list![metamodelica::Ref::new(DAE::Exp::CODE { code: metamodelica::Ref::new(Absyn::CodeNode::C_TYPENAME { path: className.clone() }), ty: DAE::T_UNKNOWN_DEFAULT().clone() }), filenameprefix.clone()], DAE::T_STRING_DEFAULT().clone()), DAE::Properties::PROP { type_: recordtype.clone(), constFlag: openmodelica_frontend_types::DAE::Const::C_VAR }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, Deref @ Absyn::ComponentRef::CREF_IDENT { name: Deref @ "exportDAEtoMatlab", .. }, Deref @ metamodelica::ListNode::Cons { head: Deref @ Absyn::Exp::CREF { componentRef: cr }, tail: Deref @ metamodelica::ListNode::Nil }, args, r#impl, pre) => {
                    let mut cname_str: Ident;
                    let mut filenameprefix: metamodelica::Ref<DAE::Exp>;
                    let mut recordtype: metamodelica::Ref<DAE::Type>;
                    let mut className: metamodelica::Ref<Absyn::Path>;
                    let mut cache = (*cache).clone();
                    className = AbsynUtil::crefToPath(metamodelica::AsArg::as_arg(&cr))?;
                    cname_str = AbsynUtil::pathString(className.clone(), literal!("."), true, false)?;
                    (cache, filenameprefix) = Static::getOptionalNamedArg(cache.clone(), env.clone(), r#impl.clone(), &(literal!("fileNamePrefix")), DAE::T_STRING_DEFAULT().clone(), metamodelica::AsArg::as_arg(&args), metamodelica::Ref::new(DAE::Exp::SCONST { string: cname_str.clone() }), pre.clone(), info.clone());
                    recordtype = metamodelica::Ref::new(DAE::Type::T_COMPLEX { complexClassType: ClassInf::State::RECORD { path: metamodelica::Ref::new(Absyn::Path::IDENT { name: literal!("SimulationObject") }) }, varLst: list![metamodelica::Ref::new(DAE::Var { name: literal!("flatClass"), attributes: DAE::dummyAttrVar().clone(), ty: DAE::T_STRING_DEFAULT().clone(), binding: openmodelica_frontend_types::DAE::Binding::interned_UNBOUND(), bind_from_outside: false, constOfForIteratorRange: None }), metamodelica::Ref::new(DAE::Var { name: literal!("exeFile"), attributes: DAE::dummyAttrVar().clone(), ty: DAE::T_STRING_DEFAULT().clone(), binding: openmodelica_frontend_types::DAE::Binding::interned_UNBOUND(), bind_from_outside: false, constOfForIteratorRange: None })], equalityConstraint: None, usedExternally: false });
                    Ok((cache.clone(), Expression::makePureBuiltinCall(literal!("exportDAEtoMatlab"), list![metamodelica::Ref::new(DAE::Exp::CODE { code: metamodelica::Ref::new(Absyn::CodeNode::C_TYPENAME { path: className.clone() }), ty: DAE::T_UNKNOWN_DEFAULT().clone() }), filenameprefix.clone()], DAE::T_STRING_DEFAULT().clone()), DAE::Properties::PROP { type_: recordtype.clone(), constFlag: openmodelica_frontend_types::DAE::Const::C_VAR }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, Deref @ Absyn::ComponentRef::CREF_IDENT { name: Deref @ "buildModel", .. }, Deref @ metamodelica::ListNode::Cons { head: Deref @ Absyn::Exp::CREF { .. }, tail: Deref @ metamodelica::ListNode::Nil }, args, _, _) => {
                    let mut simulationArgs: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                    let mut cache = (*cache).clone();
                    (cache, simulationArgs) = getSimulationArguments(cache.clone(), env.clone(), &inExps, args.clone(), inImplInst, inPrefix.clone(), literal!("buildModel"), info.clone(), None)?;
                    Ok((cache.clone(), Expression::makePureBuiltinCall(literal!("buildModel"), simulationArgs.clone(), DAE::T_UNKNOWN_DEFAULT().clone()), DAE::Properties::PROP { type_: metamodelica::Ref::new(DAE::Type::T_ARRAY { ty: DAE::T_STRING_DEFAULT().clone(), dims: list![metamodelica::Ref::new(DAE::Dimension::DIM_INTEGER { integer: 2 })] }), constFlag: openmodelica_frontend_types::DAE::Const::C_VAR }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, Deref @ Absyn::ComponentRef::CREF_IDENT { name: Deref @ "buildModelBeast", .. }, Deref @ metamodelica::ListNode::Cons { head: Deref @ Absyn::Exp::CREF { .. }, tail: Deref @ metamodelica::ListNode::Nil }, args, _, _) => {
                    let mut simulationArgs: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                    let mut cache = (*cache).clone();
                    (cache, simulationArgs) = getSimulationArguments(cache.clone(), env.clone(), &inExps, args.clone(), inImplInst, inPrefix.clone(), literal!("buildModelBeast"), info.clone(), None)?;
                    Ok((cache.clone(), Expression::makePureBuiltinCall(literal!("buildModelBeast"), simulationArgs.clone(), DAE::T_UNKNOWN_DEFAULT().clone()), DAE::Properties::PROP { type_: metamodelica::Ref::new(DAE::Type::T_ARRAY { ty: DAE::T_STRING_DEFAULT().clone(), dims: list![metamodelica::Ref::new(DAE::Dimension::DIM_INTEGER { integer: 2 })] }), constFlag: openmodelica_frontend_types::DAE::Const::C_VAR }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, Deref @ Absyn::ComponentRef::CREF_IDENT { name: Deref @ "simulate", .. }, Deref @ metamodelica::ListNode::Cons { head: Deref @ Absyn::Exp::CREF { .. }, tail: Deref @ metamodelica::ListNode::Nil }, args, _, _) => {
                    let mut recordtype: metamodelica::Ref<DAE::Type>;
                    let mut simulationArgs: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                    let mut cache = (*cache).clone();
                    (cache, simulationArgs) = getSimulationArguments(cache.clone(), env.clone(), &inExps, args.clone(), inImplInst, inPrefix.clone(), literal!("simulate"), info.clone(), None)?;
                    recordtype = CevalScriptBackend::getSimulationResultType()?;
                    Ok((cache.clone(), Expression::makePureBuiltinCall(literal!("simulate"), simulationArgs.clone(), DAE::T_UNKNOWN_DEFAULT().clone()), DAE::Properties::PROP { type_: recordtype.clone(), constFlag: openmodelica_frontend_types::DAE::Const::C_VAR }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, Deref @ Absyn::ComponentRef::CREF_IDENT { name: Deref @ "simulation", .. }, Deref @ metamodelica::ListNode::Cons { head: Deref @ Absyn::Exp::CREF { .. }, tail: Deref @ metamodelica::ListNode::Nil }, args, _, _) => {
                    let mut recordtype: metamodelica::Ref<DAE::Type>;
                    let mut simulationArgs: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                    let mut cache = (*cache).clone();
                    (cache, simulationArgs) = getSimulationArguments(cache.clone(), env.clone(), &inExps, args.clone(), inImplInst, inPrefix.clone(), literal!("simulation"), info.clone(), None)?;
                    recordtype = CevalScriptBackend::getDrModelicaSimulationResultType()?;
                    Ok((cache.clone(), Expression::makePureBuiltinCall(literal!("simulation"), simulationArgs.clone(), DAE::T_UNKNOWN_DEFAULT().clone()), DAE::Properties::PROP { type_: recordtype.clone(), constFlag: openmodelica_frontend_types::DAE::Const::C_VAR }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, Deref @ Absyn::ComponentRef::CREF_IDENT { name: Deref @ "linearize", .. }, Deref @ metamodelica::ListNode::Cons { head: Deref @ Absyn::Exp::CREF { .. }, tail: Deref @ metamodelica::ListNode::Nil }, args, _, _) => {
                    let mut recordtype: metamodelica::Ref<DAE::Type>;
                    let mut simulationArgs: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                    let mut cache = (*cache).clone();
                    (cache, simulationArgs) = getSimulationArguments(cache.clone(), env.clone(), &inExps, args.clone(), inImplInst, inPrefix.clone(), literal!("linearize"), info.clone(), None)?;
                    recordtype = CevalScriptBackend::getSimulationResultType()?;
                    Ok((cache.clone(), Expression::makePureBuiltinCall(literal!("linearize"), simulationArgs.clone(), DAE::T_UNKNOWN_DEFAULT().clone()), DAE::Properties::PROP { type_: recordtype.clone(), constFlag: openmodelica_frontend_types::DAE::Const::C_VAR }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, Deref @ Absyn::ComponentRef::CREF_IDENT { name: Deref @ "optimize", .. }, Deref @ metamodelica::ListNode::Cons { head: Deref @ Absyn::Exp::CREF { .. }, tail: Deref @ metamodelica::ListNode::Nil }, args, _, _) => {
                    let mut recordtype: metamodelica::Ref<DAE::Type>;
                    let mut simulationArgs: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                    let mut cache = (*cache).clone();
                    (cache, simulationArgs) = getSimulationArguments(cache.clone(), env.clone(), &inExps, args.clone(), inImplInst, inPrefix.clone(), literal!("optimize"), info.clone(), None)?;
                    recordtype = CevalScriptBackend::getSimulationResultType()?;
                    Ok((cache.clone(), Expression::makePureBuiltinCall(literal!("optimize"), simulationArgs.clone(), DAE::T_UNKNOWN_DEFAULT().clone()), DAE::Properties::PROP { type_: recordtype.clone(), constFlag: openmodelica_frontend_types::DAE::Const::C_VAR }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, Deref @ Absyn::ComponentRef::CREF_IDENT { name: Deref @ "moo", .. }, Deref @ metamodelica::ListNode::Cons { head: Deref @ Absyn::Exp::CREF { .. }, tail: Deref @ metamodelica::ListNode::Nil }, args, _, _) => {
                    let mut recordtype: metamodelica::Ref<DAE::Type>;
                    let mut simulationArgs: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                    let mut cache = (*cache).clone();
                    (cache, simulationArgs) = getSimulationArguments(cache.clone(), env.clone(), &inExps, args.clone(), inImplInst, inPrefix.clone(), literal!("moo"), info.clone(), None)?;
                    recordtype = CevalScriptBackend::getSimulationResultType()?;
                    Ok((cache.clone(), Expression::makePureBuiltinCall(literal!("moo"), simulationArgs.clone(), DAE::T_UNKNOWN_DEFAULT().clone()), DAE::Properties::PROP { type_: recordtype.clone(), constFlag: openmodelica_frontend_types::DAE::Const::C_VAR }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, Deref @ Absyn::ComponentRef::CREF_IDENT { name: Deref @ "jacobian", .. }, Deref @ metamodelica::ListNode::Cons { head: Deref @ Absyn::Exp::CREF { componentRef: cr }, tail: Deref @ metamodelica::ListNode::Nil }, _, r#impl, pre) => {
                    let mut cr_1: metamodelica::Ref<DAE::ComponentRef>;
                    let mut crefExp: metamodelica::Ref<DAE::Exp>;
                    let mut cache = (*cache).clone();
                    (cache, cr_1) = Static::elabUntypedCref(metamodelica::AsArg::as_arg(&cache), metamodelica::AsArg::as_arg(&env), metamodelica::AsArg::as_arg(&cr), r#impl.clone(), metamodelica::AsArg::as_arg(&pre), &info)?;
                    crefExp = Expression::crefExp(cr_1.clone())?;
                    Ok((cache.clone(), Expression::makePureBuiltinCall(literal!("jacobian"), list![crefExp.clone()], DAE::T_STRING_DEFAULT().clone()), DAE::Properties::PROP { type_: DAE::T_STRING_DEFAULT().clone(), constFlag: openmodelica_frontend_types::DAE::Const::C_VAR }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, Deref @ Absyn::ComponentRef::CREF_IDENT { name: Deref @ "timing", .. }, Deref @ metamodelica::ListNode::Cons { head: exp, tail: Deref @ metamodelica::ListNode::Nil }, Deref @ metamodelica::ListNode::Nil, r#impl, pre) => {
                    let mut exp_1: metamodelica::Ref<DAE::Exp>;
                    let mut cache = (*cache).clone();
                    (cache, exp_1, _) = elabExp(cache.clone(), env.clone(), exp.clone(), r#impl.clone(), true, pre.clone(), info.clone())?;
                    Ok((cache.clone(), Expression::makePureBuiltinCall(literal!("timing"), list![exp_1.clone()], DAE::T_REAL_DEFAULT().clone()), DAE::Properties::PROP { type_: DAE::T_REAL_DEFAULT().clone(), constFlag: openmodelica_frontend_types::DAE::Const::C_VAR }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, _, Deref @ Absyn::ComponentRef::CREF_IDENT { name: Deref @ "checkExamplePackages", .. }, Deref @ metamodelica::ListNode::Nil, args, _, _) => {
                    let mut excludeList: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                    let mut excludeListSize: i32;
                    excludeList = Static::getOptionalNamedArgExpList(&(literal!("exclude")), metamodelica::AsArg::as_arg(&args));
                    excludeListSize = ((excludeList).len() as i32);
                    Ok((cache.clone(), Expression::makePureBuiltinCall(literal!("checkExamplePackages"), list![metamodelica::Ref::new(DAE::Exp::ARRAY { ty: metamodelica::Ref::new(DAE::Type::T_ARRAY { ty: DAE::T_UNKNOWN_DEFAULT().clone(), dims: list![metamodelica::Ref::new(DAE::Dimension::DIM_INTEGER { integer: excludeListSize })] }), scalar: false, array: excludeList.clone() })], DAE::T_STRING_DEFAULT().clone()), DAE::Properties::PROP { type_: DAE::T_BOOL_DEFAULT().clone(), constFlag: openmodelica_frontend_types::DAE::Const::C_CONST }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, _, Deref @ Absyn::ComponentRef::CREF_IDENT { name: Deref @ "checkExamplePackages", .. }, Deref @ metamodelica::ListNode::Cons { head: Deref @ Absyn::Exp::STRING { value: r#str }, tail: Deref @ metamodelica::ListNode::Nil }, args, _, _) => {
                    let mut excludeList: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                    let mut excludeListSize: i32;
                    excludeList = Static::getOptionalNamedArgExpList(&(literal!("exclude")), metamodelica::AsArg::as_arg(&args));
                    excludeListSize = ((excludeList).len() as i32);
                    Ok((cache.clone(), Expression::makePureBuiltinCall(literal!("checkExamplePackages"), list![metamodelica::Ref::new(DAE::Exp::ARRAY { ty: metamodelica::Ref::new(DAE::Type::T_ARRAY { ty: DAE::T_UNKNOWN_DEFAULT().clone(), dims: list![metamodelica::Ref::new(DAE::Dimension::DIM_INTEGER { integer: excludeListSize })] }), scalar: false, array: excludeList.clone() }), metamodelica::Ref::new(DAE::Exp::SCONST { string: r#str.clone() })], DAE::T_STRING_DEFAULT().clone()), DAE::Properties::PROP { type_: DAE::T_BOOL_DEFAULT().clone(), constFlag: openmodelica_frontend_types::DAE::Const::C_CONST }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, _, Deref @ Absyn::ComponentRef::CREF_IDENT { name: Deref @ "checkExamplePackages", .. }, Deref @ metamodelica::ListNode::Cons { head: Deref @ Absyn::Exp::CREF { componentRef: cr }, tail: Deref @ metamodelica::ListNode::Nil }, args, _, _) => {
                    let mut excludeList: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                    let mut excludeListSize: i32;
                    let mut className: metamodelica::Ref<Absyn::Path>;
                    className = AbsynUtil::crefToPath(metamodelica::AsArg::as_arg(&cr))?;
                    excludeList = Static::getOptionalNamedArgExpList(&(literal!("exclude")), metamodelica::AsArg::as_arg(&args));
                    excludeListSize = ((excludeList).len() as i32);
                    Ok((cache.clone(), Expression::makePureBuiltinCall(literal!("checkExamplePackages"), list![metamodelica::Ref::new(DAE::Exp::ARRAY { ty: metamodelica::Ref::new(DAE::Type::T_ARRAY { ty: DAE::T_UNKNOWN_DEFAULT().clone(), dims: list![metamodelica::Ref::new(DAE::Dimension::DIM_INTEGER { integer: excludeListSize })] }), scalar: false, array: excludeList.clone() }), metamodelica::Ref::new(DAE::Exp::CODE { code: metamodelica::Ref::new(Absyn::CodeNode::C_TYPENAME { path: className.clone() }), ty: DAE::T_UNKNOWN_DEFAULT().clone() })], DAE::T_STRING_DEFAULT().clone()), DAE::Properties::PROP { type_: DAE::T_BOOL_DEFAULT().clone(), constFlag: openmodelica_frontend_types::DAE::Const::C_CONST }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, _, Deref @ Absyn::ComponentRef::CREF_IDENT { name: Deref @ "checkExamplePackages", .. }, Deref @ metamodelica::ListNode::Cons { head: Deref @ Absyn::Exp::CREF { componentRef: cr }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Absyn::Exp::STRING { value: r#str }, tail: Deref @ metamodelica::ListNode::Nil } }, args, _, _) => {
                    let mut excludeList: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                    let mut excludeListSize: i32;
                    let mut className: metamodelica::Ref<Absyn::Path>;
                    className = AbsynUtil::crefToPath(metamodelica::AsArg::as_arg(&cr))?;
                    excludeList = Static::getOptionalNamedArgExpList(&(literal!("exclude")), metamodelica::AsArg::as_arg(&args));
                    excludeListSize = ((excludeList).len() as i32);
                    Ok((cache.clone(), Expression::makePureBuiltinCall(literal!("checkExamplePackages"), list![metamodelica::Ref::new(DAE::Exp::ARRAY { ty: metamodelica::Ref::new(DAE::Type::T_ARRAY { ty: DAE::T_UNKNOWN_DEFAULT().clone(), dims: list![metamodelica::Ref::new(DAE::Dimension::DIM_INTEGER { integer: excludeListSize })] }), scalar: false, array: excludeList.clone() }), metamodelica::Ref::new(DAE::Exp::CODE { code: metamodelica::Ref::new(Absyn::CodeNode::C_TYPENAME { path: className.clone() }), ty: DAE::T_UNKNOWN_DEFAULT().clone() }), metamodelica::Ref::new(DAE::Exp::SCONST { string: r#str.clone() })], DAE::T_STRING_DEFAULT().clone()), DAE::Properties::PROP { type_: DAE::T_BOOL_DEFAULT().clone(), constFlag: openmodelica_frontend_types::DAE::Const::C_CONST }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((outCache, outExp, outProperties))
}

pub(crate) fn elabExp(
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
    mut inExp: metamodelica::Ref<Absyn::Exp>,
    mut inImplicit: bool,
    mut performVectorization: bool,
    mut inPrefix: DAE::Prefix,
    mut info: SourceInfo,
) -> Result<(FCore::Cache, metamodelica::Ref<DAE::Exp>, DAE::Properties)> {
    let mut outCache: FCore::Cache;
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    let mut outProperties: DAE::Properties;
    (outCache, outExp, outProperties) = elabExp2(
        inCache,
        inEnv,
        inExp,
        inImplicit,
        performVectorization,
        inPrefix,
        info,
        Error::getNumErrorMessages(),
    )?;
    Ok((outCache, outExp, outProperties))
}

fn elabExp2(
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
    mut inExp: metamodelica::Ref<Absyn::Exp>,
    mut inImplicit: bool,
    mut performVectorization: bool,
    mut inPrefix: DAE::Prefix,
    mut info: SourceInfo,
    mut numErrorMessages: i32,
) -> Result<(FCore::Cache, metamodelica::Ref<DAE::Exp>, DAE::Properties)> {
    let mut outCache: FCore::Cache;
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    let mut outProperties: DAE::Properties;
    (outCache, outExp, outProperties) = 'mc: {
        let __mc_input = (inCache, inEnv, inExp, inImplicit, performVectorization, inPrefix);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, Deref @ Absyn::Exp::CALL { function_: r#fn, functionArgs: Deref @ Absyn::FunctionArgs::FUNCTIONARGS { args, argNames: nargs }, .. }, r#impl, _, pre) => {
                    let mut e_1: metamodelica::Ref<DAE::Exp>;
                    let mut prop: DAE::Properties;
                    let mut cache = (*cache).clone();
                    (cache, e_1, prop) = elabCall(cache.clone(), env.clone(), r#fn.clone(), args.clone(), nargs.clone(), r#impl.clone(), pre.clone(), info.clone(), Error::getNumErrorMessages())?;
                    (e_1, _) = ExpressionSimplify::simplify1(e_1.clone())?;
                    Ok((cache.clone(), e_1.clone(), prop.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, exp, r#impl, doVect, pre) => {
                    let mut e_1: metamodelica::Ref<DAE::Exp>;
                    let mut prop: DAE::Properties;
                    let mut cache = (*cache).clone();
                    (cache, e_1, prop) = Static::elabExp(cache.clone(), env.clone(), exp.clone(), r#impl.clone(), doVect.clone(), pre.clone(), info.clone())?;
                    Ok((cache.clone(), e_1.clone(), prop.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((outCache, outExp, outProperties))
}

fn elabCall(
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
    mut inComponentRef: metamodelica::Ref<Absyn::ComponentRef>,
    mut inAbsynExpLst: metamodelica::List<metamodelica::Ref<Absyn::Exp>>,
    mut inAbsynNamedArgLst: metamodelica::List<metamodelica::Ref<Absyn::NamedArg>>,
    mut inImplInst: bool,
    mut inPrefix: DAE::Prefix,
    mut info: SourceInfo,
    mut numErrorMessages: i32,
) -> Result<(FCore::Cache, metamodelica::Ref<DAE::Exp>, DAE::Properties)> {
    let mut outCache: FCore::Cache;
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    let mut outProperties: DAE::Properties;
    (outCache, outExp, outProperties) = (::match_deref::match_deref! { match &((inCache, inEnv, inComponentRef, inAbsynExpLst, inAbsynNamedArgLst, inImplInst, inPrefix)) {
        (cache, env, r#fn, args, nargs, r#impl, pre) => {
            let mut e: metamodelica::Ref<DAE::Exp>;
            let mut prop: DAE::Properties;
            let mut cache = (*cache).clone();
            (cache, e, prop) = elabCallInteractive_work(cache.clone(), env.clone(), r#fn.clone(), args.clone(), nargs.clone(), r#impl.clone(), pre.clone(), info)?;
            (cache.clone(), e, prop)
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outCache, outExp, outProperties))
}

pub(crate) fn elabGraphicsExp(
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
    mut inExp: metamodelica::Ref<Absyn::Exp>,
    mut inImplInst: bool,
    mut inPrefix: DAE::Prefix,
    mut info: SourceInfo,
) -> Result<(FCore::Cache, metamodelica::Ref<DAE::Exp>, DAE::Properties)> {
    let mut outCache: FCore::Cache;
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    let mut outProperties: DAE::Properties;
    (outCache, outExp, outProperties) = 'mc: {
        let __mc_input = (inCache, inEnv, inExp, inImplInst, inPrefix);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, Deref @ Absyn::Exp::CALL { function_: r#fn, functionArgs: Deref @ Absyn::FunctionArgs::FUNCTIONARGS { args, argNames: nargs }, .. }, _, pre) => {
                    let mut e_1: metamodelica::Ref<DAE::Exp>;
                    let mut prop: DAE::Properties;
                    let mut cache = (*cache).clone();
                    (cache, e_1, prop) = elabCall(cache.clone(), env.clone(), r#fn.clone(), args.clone(), nargs.clone(), true, pre.clone(), info.clone(), Error::getNumErrorMessages())?;
                    Ok((cache.clone(), e_1.clone(), prop.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, e, r#impl, pre) => {
                    let mut e_1: metamodelica::Ref<DAE::Exp>;
                    let mut prop: DAE::Properties;
                    let mut cache = (*cache).clone();
                    (cache, e_1, prop) = Static::elabGraphicsExp(cache.clone(), env.clone(), e.clone(), r#impl.clone(), pre.clone(), &info)?;
                    Ok((cache.clone(), e_1.clone(), prop.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((outCache, outExp, outProperties))
}
