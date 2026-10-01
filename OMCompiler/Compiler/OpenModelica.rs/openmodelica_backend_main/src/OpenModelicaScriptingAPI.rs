// Auto-generated from MetaModelica source
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
use openmodelica_ast::Absyn;
use openmodelica_frontend::FGraph;
use openmodelica_frontend_base::ValuesUtil;
use openmodelica_frontend_dump::AbsynUtil;
use openmodelica_frontend_dump::FCore;
use openmodelica_frontend_dump::ValuesDump;
use openmodelica_frontend_dump::ValuesMake;
use openmodelica_frontend_types::Values;
use openmodelica_loader::Parser;

pub(crate) static dummyMsg: Absyn::Msg = Absyn::Msg::MSG {
    info: SourceInfo {
        fileName: literal!("<interactive>"),
        isReadOnly: false,
        lineNumberStart: 1,
        columnNumberStart: 1,
        lineNumberEnd: 1,
        columnNumberEnd: 1,
        lastModification: metamodelica::OrderedFloat(0.0_f64),
    },
};

pub fn oms_getVersion() -> Result<ArcStr> {
    let mut res: ArcStr;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), literal!("oms_getVersion"), metamodelica::nil(), dummyMsg.clone())?) {
        (_, Deref @ Values::Value::STRING { string: __pa0 }) => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    res = metamodelica::Own::own(__pa0);
    Ok(res)
}

pub fn oms_terminate(mut cref: ArcStr) -> Result<i32> {
    let mut res: i32;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), literal!("oms_terminate"), list![metamodelica::Ref::new(Values::Value::STRING { string: cref })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::INTEGER { integer: __pa0 }) => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    res = metamodelica::Own::own(__pa0);
    Ok(res)
}

pub fn oms_stepUntil(mut cref: ArcStr, mut stopTime: metamodelica::Real) -> Result<i32> {
    let mut res: i32;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), literal!("oms_stepUntil"), list![metamodelica::Ref::new(Values::Value::STRING { string: cref }), metamodelica::Ref::new(Values::Value::REAL { real: stopTime })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::INTEGER { integer: __pa0 }) => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    res = metamodelica::Own::own(__pa0);
    Ok(res)
}

pub fn oms_simulate(mut cref: ArcStr) -> Result<i32> {
    let mut res: i32;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), literal!("oms_simulate"), list![metamodelica::Ref::new(Values::Value::STRING { string: cref })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::INTEGER { integer: __pa0 }) => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    res = metamodelica::Own::own(__pa0);
    Ok(res)
}

pub fn oms_setWorkingDirectory(mut newWorkingDir: ArcStr) -> Result<i32> {
    let mut res: i32;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), literal!("oms_setWorkingDirectory"), list![metamodelica::Ref::new(Values::Value::STRING { string: newWorkingDir })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::INTEGER { integer: __pa0 }) => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    res = metamodelica::Own::own(__pa0);
    Ok(res)
}

pub fn oms_setVariableStepSize(
    mut cref: ArcStr,
    mut initialStepSize: metamodelica::Real,
    mut minimumStepSize: metamodelica::Real,
    mut maximumStepSize: metamodelica::Real,
) -> Result<i32> {
    let mut res: i32;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), literal!("oms_setVariableStepSize"), list![metamodelica::Ref::new(Values::Value::STRING { string: cref }), metamodelica::Ref::new(Values::Value::REAL { real: initialStepSize }), metamodelica::Ref::new(Values::Value::REAL { real: minimumStepSize }), metamodelica::Ref::new(Values::Value::REAL { real: maximumStepSize })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::INTEGER { integer: __pa0 }) => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    res = metamodelica::Own::own(__pa0);
    Ok(res)
}

pub fn oms_setTolerance(
    mut cref: ArcStr,
    mut absoluteTolerance: metamodelica::Real,
    mut relativeTolerance: metamodelica::Real,
) -> Result<i32> {
    let mut res: i32;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), literal!("oms_setTolerance"), list![metamodelica::Ref::new(Values::Value::STRING { string: cref }), metamodelica::Ref::new(Values::Value::REAL { real: absoluteTolerance }), metamodelica::Ref::new(Values::Value::REAL { real: relativeTolerance })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::INTEGER { integer: __pa0 }) => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    res = metamodelica::Own::own(__pa0);
    Ok(res)
}

pub fn oms_setTLMSocketData(
    mut cref: ArcStr,
    mut address: ArcStr,
    mut managerPort: i32,
    mut monitorPort: i32,
) -> Result<i32> {
    let mut res: i32;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), literal!("oms_setTLMSocketData"), list![metamodelica::Ref::new(Values::Value::STRING { string: cref }), metamodelica::Ref::new(Values::Value::STRING { string: address }), metamodelica::Ref::new(Values::Value::INTEGER { integer: managerPort }), metamodelica::Ref::new(Values::Value::INTEGER { integer: monitorPort })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::INTEGER { integer: __pa0 }) => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    res = metamodelica::Own::own(__pa0);
    Ok(res)
}

pub fn oms_setTLMPositionAndOrientation(
    mut cref: ArcStr,
    mut x1: metamodelica::Real,
    mut x2: metamodelica::Real,
    mut x3: metamodelica::Real,
    mut A11: metamodelica::Real,
    mut A12: metamodelica::Real,
    mut A13: metamodelica::Real,
    mut A21: metamodelica::Real,
    mut A22: metamodelica::Real,
    mut A23: metamodelica::Real,
    mut A31: metamodelica::Real,
    mut A32: metamodelica::Real,
    mut A33: metamodelica::Real,
) -> Result<i32> {
    let mut res: i32;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), literal!("oms_setTLMPositionAndOrientation"), list![metamodelica::Ref::new(Values::Value::STRING { string: cref }), metamodelica::Ref::new(Values::Value::REAL { real: x1 }), metamodelica::Ref::new(Values::Value::REAL { real: x2 }), metamodelica::Ref::new(Values::Value::REAL { real: x3 }), metamodelica::Ref::new(Values::Value::REAL { real: A11 }), metamodelica::Ref::new(Values::Value::REAL { real: A12 }), metamodelica::Ref::new(Values::Value::REAL { real: A13 }), metamodelica::Ref::new(Values::Value::REAL { real: A21 }), metamodelica::Ref::new(Values::Value::REAL { real: A22 }), metamodelica::Ref::new(Values::Value::REAL { real: A23 }), metamodelica::Ref::new(Values::Value::REAL { real: A31 }), metamodelica::Ref::new(Values::Value::REAL { real: A32 }), metamodelica::Ref::new(Values::Value::REAL { real: A33 })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::INTEGER { integer: __pa0 }) => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    res = metamodelica::Own::own(__pa0);
    Ok(res)
}

pub fn oms_setTempDirectory(mut newTempDir: ArcStr) -> Result<i32> {
    let mut res: i32;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), literal!("oms_setTempDirectory"), list![metamodelica::Ref::new(Values::Value::STRING { string: newTempDir })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::INTEGER { integer: __pa0 }) => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    res = metamodelica::Own::own(__pa0);
    Ok(res)
}

pub fn oms_setStopTime(mut cref: ArcStr, mut stopTime: metamodelica::Real) -> Result<i32> {
    let mut res: i32;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), literal!("oms_setStopTime"), list![metamodelica::Ref::new(Values::Value::STRING { string: cref }), metamodelica::Ref::new(Values::Value::REAL { real: stopTime })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::INTEGER { integer: __pa0 }) => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    res = metamodelica::Own::own(__pa0);
    Ok(res)
}

pub fn oms_setStartTime(mut cref: ArcStr, mut startTime: metamodelica::Real) -> Result<i32> {
    let mut res: i32;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), literal!("oms_setStartTime"), list![metamodelica::Ref::new(Values::Value::STRING { string: cref }), metamodelica::Ref::new(Values::Value::REAL { real: startTime })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::INTEGER { integer: __pa0 }) => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    res = metamodelica::Own::own(__pa0);
    Ok(res)
}

pub fn oms_setSignalFilter(mut cref: ArcStr, mut regex: ArcStr) -> Result<i32> {
    let mut res: i32;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), literal!("oms_setSignalFilter"), list![metamodelica::Ref::new(Values::Value::STRING { string: cref }), metamodelica::Ref::new(Values::Value::STRING { string: regex })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::INTEGER { integer: __pa0 }) => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    res = metamodelica::Own::own(__pa0);
    Ok(res)
}

pub fn oms_setResultFile(mut cref: ArcStr, mut filename: ArcStr, mut bufferSize: i32) -> Result<i32> {
    let mut res: i32;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), literal!("oms_setResultFile"), list![metamodelica::Ref::new(Values::Value::STRING { string: cref }), metamodelica::Ref::new(Values::Value::STRING { string: filename }), metamodelica::Ref::new(Values::Value::INTEGER { integer: bufferSize })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::INTEGER { integer: __pa0 }) => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    res = metamodelica::Own::own(__pa0);
    Ok(res)
}

pub fn oms_setRealInputDerivative(mut cref: ArcStr, mut value: metamodelica::Real) -> Result<i32> {
    let mut res: i32;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), literal!("oms_setRealInputDerivative"), list![metamodelica::Ref::new(Values::Value::STRING { string: cref }), metamodelica::Ref::new(Values::Value::REAL { real: value })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::INTEGER { integer: __pa0 }) => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    res = metamodelica::Own::own(__pa0);
    Ok(res)
}

pub fn oms_setReal(mut cref: ArcStr, mut value: metamodelica::Real) -> Result<i32> {
    let mut res: i32;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), literal!("oms_setReal"), list![metamodelica::Ref::new(Values::Value::STRING { string: cref }), metamodelica::Ref::new(Values::Value::REAL { real: value })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::INTEGER { integer: __pa0 }) => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    res = metamodelica::Own::own(__pa0);
    Ok(res)
}

pub fn oms_setLoggingLevel(mut logLevel: i32) -> Result<i32> {
    let mut res: i32;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), literal!("oms_setLoggingLevel"), list![metamodelica::Ref::new(Values::Value::INTEGER { integer: logLevel })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::INTEGER { integer: __pa0 }) => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    res = metamodelica::Own::own(__pa0);
    Ok(res)
}

pub fn oms_setLoggingInterval(mut cref: ArcStr, mut loggingInterval: metamodelica::Real) -> Result<i32> {
    let mut res: i32;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), literal!("oms_setLoggingInterval"), list![metamodelica::Ref::new(Values::Value::STRING { string: cref }), metamodelica::Ref::new(Values::Value::REAL { real: loggingInterval })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::INTEGER { integer: __pa0 }) => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    res = metamodelica::Own::own(__pa0);
    Ok(res)
}

pub fn oms_setLogFile(mut filename: ArcStr) -> Result<i32> {
    let mut res: i32;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), literal!("oms_setLogFile"), list![metamodelica::Ref::new(Values::Value::STRING { string: filename })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::INTEGER { integer: __pa0 }) => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    res = metamodelica::Own::own(__pa0);
    Ok(res)
}

pub fn oms_setInteger(mut cref: ArcStr, mut value: i32) -> Result<i32> {
    let mut res: i32;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), literal!("oms_setInteger"), list![metamodelica::Ref::new(Values::Value::STRING { string: cref }), metamodelica::Ref::new(Values::Value::INTEGER { integer: value })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::INTEGER { integer: __pa0 }) => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    res = metamodelica::Own::own(__pa0);
    Ok(res)
}

pub fn oms_setFixedStepSize(mut cref: ArcStr, mut stepSize: metamodelica::Real) -> Result<i32> {
    let mut res: i32;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), literal!("oms_setFixedStepSize"), list![metamodelica::Ref::new(Values::Value::STRING { string: cref }), metamodelica::Ref::new(Values::Value::REAL { real: stepSize })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::INTEGER { integer: __pa0 }) => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    res = metamodelica::Own::own(__pa0);
    Ok(res)
}

pub fn oms_setCommandLineOption(mut cmd: ArcStr) -> Result<i32> {
    let mut res: i32;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), literal!("oms_setCommandLineOption"), list![metamodelica::Ref::new(Values::Value::STRING { string: cmd })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::INTEGER { integer: __pa0 }) => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    res = metamodelica::Own::own(__pa0);
    Ok(res)
}

pub fn oms_setBoolean(mut cref: ArcStr, mut value: bool) -> Result<i32> {
    let mut res: i32;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), literal!("oms_setBoolean"), list![metamodelica::Ref::new(Values::Value::STRING { string: cref }), metamodelica::Ref::new(Values::Value::BOOL { boolean: value })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::INTEGER { integer: __pa0 }) => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    res = metamodelica::Own::own(__pa0);
    Ok(res)
}

pub fn oms_RunFile(mut filename: ArcStr) -> Result<i32> {
    let mut res: i32;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), literal!("oms_RunFile"), list![metamodelica::Ref::new(Values::Value::STRING { string: filename })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::INTEGER { integer: __pa0 }) => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    res = metamodelica::Own::own(__pa0);
    Ok(res)
}

pub fn oms_reset(mut cref: ArcStr) -> Result<i32> {
    let mut res: i32;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), literal!("oms_reset"), list![metamodelica::Ref::new(Values::Value::STRING { string: cref })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::INTEGER { integer: __pa0 }) => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    res = metamodelica::Own::own(__pa0);
    Ok(res)
}

pub fn oms_rename(mut cref: ArcStr, mut newCref: ArcStr) -> Result<i32> {
    let mut res: i32;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), literal!("oms_rename"), list![metamodelica::Ref::new(Values::Value::STRING { string: cref }), metamodelica::Ref::new(Values::Value::STRING { string: newCref })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::INTEGER { integer: __pa0 }) => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    res = metamodelica::Own::own(__pa0);
    Ok(res)
}

pub fn oms_removeSignalsFromResults(mut cref: ArcStr, mut regex: ArcStr) -> Result<i32> {
    let mut res: i32;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), literal!("oms_removeSignalsFromResults"), list![metamodelica::Ref::new(Values::Value::STRING { string: cref }), metamodelica::Ref::new(Values::Value::STRING { string: regex })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::INTEGER { integer: __pa0 }) => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    res = metamodelica::Own::own(__pa0);
    Ok(res)
}

pub fn oms_newModel(mut cref: ArcStr) -> Result<i32> {
    let mut res: i32;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), literal!("oms_newModel"), list![metamodelica::Ref::new(Values::Value::STRING { string: cref })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::INTEGER { integer: __pa0 }) => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    res = metamodelica::Own::own(__pa0);
    Ok(res)
}

pub fn oms_loadSnapshot(mut cref: ArcStr, mut snapshot: ArcStr) -> Result<(ArcStr, i32)> {
    let mut res1: ArcStr;
    let mut res2: i32;
    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), literal!("oms_loadSnapshot"), list![metamodelica::Ref::new(Values::Value::STRING { string: cref }), metamodelica::Ref::new(Values::Value::STRING { string: snapshot })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::TUPLE { valueLst: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: __pa0 }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::INTEGER { integer: __pa1 }, tail: Deref @ metamodelica::ListNode::Nil } } }) => (__pa0.clone(), __pa1.clone()),
        _ => return Err("pattern mismatch"),
    } };
    res1 = metamodelica::Own::own(__pa0);
    res2 = metamodelica::Own::own(__pa1);
    Ok((res1, res2))
}

pub fn oms_listUnconnectedConnectors(mut cref: ArcStr) -> Result<(ArcStr, i32)> {
    let mut res1: ArcStr;
    let mut res2: i32;
    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), literal!("oms_listUnconnectedConnectors"), list![metamodelica::Ref::new(Values::Value::STRING { string: cref })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::TUPLE { valueLst: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: __pa0 }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::INTEGER { integer: __pa1 }, tail: Deref @ metamodelica::ListNode::Nil } } }) => (__pa0.clone(), __pa1.clone()),
        _ => return Err("pattern mismatch"),
    } };
    res1 = metamodelica::Own::own(__pa0);
    res2 = metamodelica::Own::own(__pa1);
    Ok((res1, res2))
}

pub fn oms_list(mut cref: ArcStr) -> Result<(ArcStr, i32)> {
    let mut res1: ArcStr;
    let mut res2: i32;
    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), literal!("oms_list"), list![metamodelica::Ref::new(Values::Value::STRING { string: cref })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::TUPLE { valueLst: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: __pa0 }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::INTEGER { integer: __pa1 }, tail: Deref @ metamodelica::ListNode::Nil } } }) => (__pa0.clone(), __pa1.clone()),
        _ => return Err("pattern mismatch"),
    } };
    res1 = metamodelica::Own::own(__pa0);
    res2 = metamodelica::Own::own(__pa1);
    Ok((res1, res2))
}

pub fn oms_instantiate(mut cref: ArcStr) -> Result<i32> {
    let mut res: i32;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), literal!("oms_instantiate"), list![metamodelica::Ref::new(Values::Value::STRING { string: cref })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::INTEGER { integer: __pa0 }) => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    res = metamodelica::Own::own(__pa0);
    Ok(res)
}

pub fn oms_initialize(mut cref: ArcStr) -> Result<i32> {
    let mut res: i32;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), literal!("oms_initialize"), list![metamodelica::Ref::new(Values::Value::STRING { string: cref })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::INTEGER { integer: __pa0 }) => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    res = metamodelica::Own::own(__pa0);
    Ok(res)
}

pub fn oms_importSnapshot(mut cref: ArcStr, mut snapshot: ArcStr) -> Result<i32> {
    let mut res: i32;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), literal!("oms_importSnapshot"), list![metamodelica::Ref::new(Values::Value::STRING { string: cref }), metamodelica::Ref::new(Values::Value::STRING { string: snapshot })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::INTEGER { integer: __pa0 }) => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    res = metamodelica::Own::own(__pa0);
    Ok(res)
}

pub fn oms_importFile(mut filename: ArcStr) -> Result<(ArcStr, i32)> {
    let mut res1: ArcStr;
    let mut res2: i32;
    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), literal!("oms_importFile"), list![metamodelica::Ref::new(Values::Value::STRING { string: filename })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::TUPLE { valueLst: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: __pa0 }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::INTEGER { integer: __pa1 }, tail: Deref @ metamodelica::ListNode::Nil } } }) => (__pa0.clone(), __pa1.clone()),
        _ => return Err("pattern mismatch"),
    } };
    res1 = metamodelica::Own::own(__pa0);
    res2 = metamodelica::Own::own(__pa1);
    Ok((res1, res2))
}

pub fn oms_getVariableStepSize(
    mut cref: ArcStr,
) -> Result<(metamodelica::Real, metamodelica::Real, metamodelica::Real, i32)> {
    let mut res1: metamodelica::Real;
    let mut res2: metamodelica::Real;
    let mut res3: metamodelica::Real;
    let mut res4: i32;
    let (__pa0, __pa1, __pa2, __pa3) = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), literal!("oms_getVariableStepSize"), list![metamodelica::Ref::new(Values::Value::STRING { string: cref })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::TUPLE { valueLst: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::REAL { real: __pa0 }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::REAL { real: __pa1 }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::REAL { real: __pa2 }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::INTEGER { integer: __pa3 }, tail: Deref @ metamodelica::ListNode::Nil } } } } }) => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone()),
        _ => return Err("pattern mismatch"),
    } };
    res1 = metamodelica::Own::own(__pa0);
    res2 = metamodelica::Own::own(__pa1);
    res3 = metamodelica::Own::own(__pa2);
    res4 = metamodelica::Own::own(__pa3);
    Ok((res1, res2, res3, res4))
}

pub fn oms_getTolerance(mut cref: ArcStr) -> Result<(metamodelica::Real, metamodelica::Real, i32)> {
    let mut res1: metamodelica::Real;
    let mut res2: metamodelica::Real;
    let mut res3: i32;
    let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), literal!("oms_getTolerance"), list![metamodelica::Ref::new(Values::Value::STRING { string: cref })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::TUPLE { valueLst: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::REAL { real: __pa0 }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::REAL { real: __pa1 }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::INTEGER { integer: __pa2 }, tail: Deref @ metamodelica::ListNode::Nil } } } }) => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
        _ => return Err("pattern mismatch"),
    } };
    res1 = metamodelica::Own::own(__pa0);
    res2 = metamodelica::Own::own(__pa1);
    res3 = metamodelica::Own::own(__pa2);
    Ok((res1, res2, res3))
}

pub fn oms_getSystemType(mut cref: ArcStr) -> Result<(i32, i32)> {
    let mut res1: i32;
    let mut res2: i32;
    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), literal!("oms_getSystemType"), list![metamodelica::Ref::new(Values::Value::STRING { string: cref })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::TUPLE { valueLst: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::INTEGER { integer: __pa0 }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::INTEGER { integer: __pa1 }, tail: Deref @ metamodelica::ListNode::Nil } } }) => (__pa0.clone(), __pa1.clone()),
        _ => return Err("pattern mismatch"),
    } };
    res1 = metamodelica::Own::own(__pa0);
    res2 = metamodelica::Own::own(__pa1);
    Ok((res1, res2))
}

pub fn oms_getSubModelPath(mut cref: ArcStr) -> Result<(ArcStr, i32)> {
    let mut res1: ArcStr;
    let mut res2: i32;
    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), literal!("oms_getSubModelPath"), list![metamodelica::Ref::new(Values::Value::STRING { string: cref })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::TUPLE { valueLst: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: __pa0 }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::INTEGER { integer: __pa1 }, tail: Deref @ metamodelica::ListNode::Nil } } }) => (__pa0.clone(), __pa1.clone()),
        _ => return Err("pattern mismatch"),
    } };
    res1 = metamodelica::Own::own(__pa0);
    res2 = metamodelica::Own::own(__pa1);
    Ok((res1, res2))
}

pub fn oms_getStopTime(mut cref: ArcStr) -> Result<(metamodelica::Real, i32)> {
    let mut res1: metamodelica::Real;
    let mut res2: i32;
    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), literal!("oms_getStopTime"), list![metamodelica::Ref::new(Values::Value::STRING { string: cref })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::TUPLE { valueLst: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::REAL { real: __pa0 }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::INTEGER { integer: __pa1 }, tail: Deref @ metamodelica::ListNode::Nil } } }) => (__pa0.clone(), __pa1.clone()),
        _ => return Err("pattern mismatch"),
    } };
    res1 = metamodelica::Own::own(__pa0);
    res2 = metamodelica::Own::own(__pa1);
    Ok((res1, res2))
}

pub fn oms_getStartTime(mut cref: ArcStr) -> Result<(metamodelica::Real, i32)> {
    let mut res1: metamodelica::Real;
    let mut res2: i32;
    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), literal!("oms_getStartTime"), list![metamodelica::Ref::new(Values::Value::STRING { string: cref })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::TUPLE { valueLst: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::REAL { real: __pa0 }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::INTEGER { integer: __pa1 }, tail: Deref @ metamodelica::ListNode::Nil } } }) => (__pa0.clone(), __pa1.clone()),
        _ => return Err("pattern mismatch"),
    } };
    res1 = metamodelica::Own::own(__pa0);
    res2 = metamodelica::Own::own(__pa1);
    Ok((res1, res2))
}

pub fn oms_getSolver(mut cref: ArcStr) -> Result<(i32, i32)> {
    let mut res1: i32;
    let mut res2: i32;
    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), literal!("oms_getSolver"), list![metamodelica::Ref::new(Values::Value::STRING { string: cref })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::TUPLE { valueLst: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::INTEGER { integer: __pa0 }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::INTEGER { integer: __pa1 }, tail: Deref @ metamodelica::ListNode::Nil } } }) => (__pa0.clone(), __pa1.clone()),
        _ => return Err("pattern mismatch"),
    } };
    res1 = metamodelica::Own::own(__pa0);
    res2 = metamodelica::Own::own(__pa1);
    Ok((res1, res2))
}

pub fn oms_getReal(mut cref: ArcStr) -> Result<(metamodelica::Real, i32)> {
    let mut res1: metamodelica::Real;
    let mut res2: i32;
    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), literal!("oms_getReal"), list![metamodelica::Ref::new(Values::Value::STRING { string: cref })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::TUPLE { valueLst: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::REAL { real: __pa0 }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::INTEGER { integer: __pa1 }, tail: Deref @ metamodelica::ListNode::Nil } } }) => (__pa0.clone(), __pa1.clone()),
        _ => return Err("pattern mismatch"),
    } };
    res1 = metamodelica::Own::own(__pa0);
    res2 = metamodelica::Own::own(__pa1);
    Ok((res1, res2))
}

pub fn oms_getModelState(mut cref: ArcStr) -> Result<(i32, i32)> {
    let mut res1: i32;
    let mut res2: i32;
    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), literal!("oms_getModelState"), list![metamodelica::Ref::new(Values::Value::STRING { string: cref })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::TUPLE { valueLst: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::INTEGER { integer: __pa0 }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::INTEGER { integer: __pa1 }, tail: Deref @ metamodelica::ListNode::Nil } } }) => (__pa0.clone(), __pa1.clone()),
        _ => return Err("pattern mismatch"),
    } };
    res1 = metamodelica::Own::own(__pa0);
    res2 = metamodelica::Own::own(__pa1);
    Ok((res1, res2))
}

pub fn oms_getInteger(mut cref: ArcStr, mut value: i32) -> Result<i32> {
    let mut res: i32;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), literal!("oms_getInteger"), list![metamodelica::Ref::new(Values::Value::STRING { string: cref }), metamodelica::Ref::new(Values::Value::INTEGER { integer: value })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::INTEGER { integer: __pa0 }) => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    res = metamodelica::Own::own(__pa0);
    Ok(res)
}

pub fn oms_getFixedStepSize(mut cref: ArcStr) -> Result<(metamodelica::Real, i32)> {
    let mut res1: metamodelica::Real;
    let mut res2: i32;
    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), literal!("oms_getFixedStepSize"), list![metamodelica::Ref::new(Values::Value::STRING { string: cref })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::TUPLE { valueLst: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::REAL { real: __pa0 }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::INTEGER { integer: __pa1 }, tail: Deref @ metamodelica::ListNode::Nil } } }) => (__pa0.clone(), __pa1.clone()),
        _ => return Err("pattern mismatch"),
    } };
    res1 = metamodelica::Own::own(__pa0);
    res2 = metamodelica::Own::own(__pa1);
    Ok((res1, res2))
}

pub fn oms_getBoolean(mut cref: ArcStr) -> Result<(bool, i32)> {
    let mut res1: bool;
    let mut res2: i32;
    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), literal!("oms_getBoolean"), list![metamodelica::Ref::new(Values::Value::STRING { string: cref })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::TUPLE { valueLst: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::BOOL { boolean: __pa0 }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::INTEGER { integer: __pa1 }, tail: Deref @ metamodelica::ListNode::Nil } } }) => (__pa0.clone(), __pa1.clone()),
        _ => return Err("pattern mismatch"),
    } };
    res1 = metamodelica::Own::own(__pa0);
    res2 = metamodelica::Own::own(__pa1);
    Ok((res1, res2))
}

pub fn oms_extractFMIKind(mut filename: ArcStr) -> Result<(i32, i32)> {
    let mut res1: i32;
    let mut res2: i32;
    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), literal!("oms_extractFMIKind"), list![metamodelica::Ref::new(Values::Value::STRING { string: filename })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::TUPLE { valueLst: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::INTEGER { integer: __pa0 }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::INTEGER { integer: __pa1 }, tail: Deref @ metamodelica::ListNode::Nil } } }) => (__pa0.clone(), __pa1.clone()),
        _ => return Err("pattern mismatch"),
    } };
    res1 = metamodelica::Own::own(__pa0);
    res2 = metamodelica::Own::own(__pa1);
    Ok((res1, res2))
}

pub fn oms_exportSnapshot(mut cref: ArcStr) -> Result<(ArcStr, i32)> {
    let mut res1: ArcStr;
    let mut res2: i32;
    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), literal!("oms_exportSnapshot"), list![metamodelica::Ref::new(Values::Value::STRING { string: cref })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::TUPLE { valueLst: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: __pa0 }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::INTEGER { integer: __pa1 }, tail: Deref @ metamodelica::ListNode::Nil } } }) => (__pa0.clone(), __pa1.clone()),
        _ => return Err("pattern mismatch"),
    } };
    res1 = metamodelica::Own::own(__pa0);
    res2 = metamodelica::Own::own(__pa1);
    Ok((res1, res2))
}

pub fn oms_exportDependencyGraphs(
    mut cref: ArcStr,
    mut initialization: ArcStr,
    mut event: ArcStr,
    mut simulation: ArcStr,
) -> Result<i32> {
    let mut res: i32;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), literal!("oms_exportDependencyGraphs"), list![metamodelica::Ref::new(Values::Value::STRING { string: cref }), metamodelica::Ref::new(Values::Value::STRING { string: initialization }), metamodelica::Ref::new(Values::Value::STRING { string: event }), metamodelica::Ref::new(Values::Value::STRING { string: simulation })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::INTEGER { integer: __pa0 }) => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    res = metamodelica::Own::own(__pa0);
    Ok(res)
}

pub fn oms_export(mut cref: ArcStr, mut filename: ArcStr) -> Result<i32> {
    let mut res: i32;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), literal!("oms_export"), list![metamodelica::Ref::new(Values::Value::STRING { string: cref }), metamodelica::Ref::new(Values::Value::STRING { string: filename })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::INTEGER { integer: __pa0 }) => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    res = metamodelica::Own::own(__pa0);
    Ok(res)
}

pub fn oms_deleteConnectorFromTLMBus(mut busCref: ArcStr, mut connectorCref: ArcStr) -> Result<i32> {
    let mut res: i32;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), literal!("oms_deleteConnectorFromTLMBus"), list![metamodelica::Ref::new(Values::Value::STRING { string: busCref }), metamodelica::Ref::new(Values::Value::STRING { string: connectorCref })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::INTEGER { integer: __pa0 }) => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    res = metamodelica::Own::own(__pa0);
    Ok(res)
}

pub fn oms_deleteConnectorFromBus(mut busCref: ArcStr, mut connectorCref: ArcStr) -> Result<i32> {
    let mut res: i32;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), literal!("oms_deleteConnectorFromBus"), list![metamodelica::Ref::new(Values::Value::STRING { string: busCref }), metamodelica::Ref::new(Values::Value::STRING { string: connectorCref })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::INTEGER { integer: __pa0 }) => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    res = metamodelica::Own::own(__pa0);
    Ok(res)
}

pub fn oms_deleteConnection(mut crefA: ArcStr, mut crefB: ArcStr) -> Result<i32> {
    let mut res: i32;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), literal!("oms_deleteConnection"), list![metamodelica::Ref::new(Values::Value::STRING { string: crefA }), metamodelica::Ref::new(Values::Value::STRING { string: crefB })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::INTEGER { integer: __pa0 }) => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    res = metamodelica::Own::own(__pa0);
    Ok(res)
}

pub fn oms_delete(mut cref: ArcStr) -> Result<i32> {
    let mut res: i32;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), literal!("oms_delete"), list![metamodelica::Ref::new(Values::Value::STRING { string: cref })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::INTEGER { integer: __pa0 }) => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    res = metamodelica::Own::own(__pa0);
    Ok(res)
}

pub fn oms_copySystem(mut source: ArcStr, mut target: ArcStr) -> Result<i32> {
    let mut res: i32;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), literal!("oms_copySystem"), list![metamodelica::Ref::new(Values::Value::STRING { string: source }), metamodelica::Ref::new(Values::Value::STRING { string: target })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::INTEGER { integer: __pa0 }) => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    res = metamodelica::Own::own(__pa0);
    Ok(res)
}

pub fn oms_compareSimulationResults(
    mut filenameA: ArcStr,
    mut filenameB: ArcStr,
    mut var: ArcStr,
    mut relTol: metamodelica::Real,
    mut absTol: metamodelica::Real,
) -> Result<i32> {
    let mut res: i32;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), literal!("oms_compareSimulationResults"), list![metamodelica::Ref::new(Values::Value::STRING { string: filenameA }), metamodelica::Ref::new(Values::Value::STRING { string: filenameB }), metamodelica::Ref::new(Values::Value::STRING { string: var }), metamodelica::Ref::new(Values::Value::REAL { real: relTol }), metamodelica::Ref::new(Values::Value::REAL { real: absTol })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::INTEGER { integer: __pa0 }) => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    res = metamodelica::Own::own(__pa0);
    Ok(res)
}

pub fn oms_addTLMConnection(
    mut crefA: ArcStr,
    mut crefB: ArcStr,
    mut delay: metamodelica::Real,
    mut alpha: metamodelica::Real,
    mut linearimpedance: metamodelica::Real,
    mut angularimpedance: metamodelica::Real,
) -> Result<i32> {
    let mut res: i32;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), literal!("oms_addTLMConnection"), list![metamodelica::Ref::new(Values::Value::STRING { string: crefA }), metamodelica::Ref::new(Values::Value::STRING { string: crefB }), metamodelica::Ref::new(Values::Value::REAL { real: delay }), metamodelica::Ref::new(Values::Value::REAL { real: alpha }), metamodelica::Ref::new(Values::Value::REAL { real: linearimpedance }), metamodelica::Ref::new(Values::Value::REAL { real: angularimpedance })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::INTEGER { integer: __pa0 }) => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    res = metamodelica::Own::own(__pa0);
    Ok(res)
}

pub fn oms_addTimeIndicator(mut signal: ArcStr) -> Result<i32> {
    let mut res: i32;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), literal!("oms_addTimeIndicator"), list![metamodelica::Ref::new(Values::Value::STRING { string: signal })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::INTEGER { integer: __pa0 }) => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    res = metamodelica::Own::own(__pa0);
    Ok(res)
}

pub fn oms_addSubModel(mut cref: ArcStr, mut fmuPath: ArcStr) -> Result<i32> {
    let mut res: i32;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), literal!("oms_addSubModel"), list![metamodelica::Ref::new(Values::Value::STRING { string: cref }), metamodelica::Ref::new(Values::Value::STRING { string: fmuPath })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::INTEGER { integer: __pa0 }) => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    res = metamodelica::Own::own(__pa0);
    Ok(res)
}

pub fn oms_addStaticValueIndicator(
    mut signal: ArcStr,
    mut lower: metamodelica::Real,
    mut upper: metamodelica::Real,
    mut stepSize: metamodelica::Real,
) -> Result<i32> {
    let mut res: i32;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), literal!("oms_addStaticValueIndicator"), list![metamodelica::Ref::new(Values::Value::STRING { string: signal }), metamodelica::Ref::new(Values::Value::REAL { real: lower }), metamodelica::Ref::new(Values::Value::REAL { real: upper }), metamodelica::Ref::new(Values::Value::REAL { real: stepSize })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::INTEGER { integer: __pa0 }) => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    res = metamodelica::Own::own(__pa0);
    Ok(res)
}

pub fn oms_addSignalsToResults(mut cref: ArcStr, mut regex: ArcStr) -> Result<i32> {
    let mut res: i32;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), literal!("oms_addSignalsToResults"), list![metamodelica::Ref::new(Values::Value::STRING { string: cref }), metamodelica::Ref::new(Values::Value::STRING { string: regex })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::INTEGER { integer: __pa0 }) => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    res = metamodelica::Own::own(__pa0);
    Ok(res)
}

pub fn oms_addExternalModel(mut cref: ArcStr, mut path: ArcStr, mut startscript: ArcStr) -> Result<i32> {
    let mut res: i32;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), literal!("oms_addExternalModel"), list![metamodelica::Ref::new(Values::Value::STRING { string: cref }), metamodelica::Ref::new(Values::Value::STRING { string: path }), metamodelica::Ref::new(Values::Value::STRING { string: startscript })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::INTEGER { integer: __pa0 }) => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    res = metamodelica::Own::own(__pa0);
    Ok(res)
}

pub fn oms_addEventIndicator(mut signal: ArcStr) -> Result<i32> {
    let mut res: i32;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), literal!("oms_addEventIndicator"), list![metamodelica::Ref::new(Values::Value::STRING { string: signal })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::INTEGER { integer: __pa0 }) => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    res = metamodelica::Own::own(__pa0);
    Ok(res)
}

pub fn oms_addDynamicValueIndicator(
    mut signal: ArcStr,
    mut lower: ArcStr,
    mut upper: ArcStr,
    mut stepSize: metamodelica::Real,
) -> Result<i32> {
    let mut res: i32;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), literal!("oms_addDynamicValueIndicator"), list![metamodelica::Ref::new(Values::Value::STRING { string: signal }), metamodelica::Ref::new(Values::Value::STRING { string: lower }), metamodelica::Ref::new(Values::Value::STRING { string: upper }), metamodelica::Ref::new(Values::Value::REAL { real: stepSize })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::INTEGER { integer: __pa0 }) => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    res = metamodelica::Own::own(__pa0);
    Ok(res)
}

pub fn oms_addConnectorToTLMBus(mut busCref: ArcStr, mut connectorCref: ArcStr, mut type_: ArcStr) -> Result<i32> {
    let mut res: i32;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), literal!("oms_addConnectorToTLMBus"), list![metamodelica::Ref::new(Values::Value::STRING { string: busCref }), metamodelica::Ref::new(Values::Value::STRING { string: connectorCref }), metamodelica::Ref::new(Values::Value::STRING { string: type_ })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::INTEGER { integer: __pa0 }) => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    res = metamodelica::Own::own(__pa0);
    Ok(res)
}

pub fn oms_addConnectorToBus(mut busCref: ArcStr, mut connectorCref: ArcStr) -> Result<i32> {
    let mut res: i32;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), literal!("oms_addConnectorToBus"), list![metamodelica::Ref::new(Values::Value::STRING { string: busCref }), metamodelica::Ref::new(Values::Value::STRING { string: connectorCref })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::INTEGER { integer: __pa0 }) => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    res = metamodelica::Own::own(__pa0);
    Ok(res)
}

pub fn oms_addConnection(mut crefA: ArcStr, mut crefB: ArcStr) -> Result<i32> {
    let mut res: i32;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), literal!("oms_addConnection"), list![metamodelica::Ref::new(Values::Value::STRING { string: crefA }), metamodelica::Ref::new(Values::Value::STRING { string: crefB })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::INTEGER { integer: __pa0 }) => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    res = metamodelica::Own::own(__pa0);
    Ok(res)
}

pub fn oms_addBus(mut cref: ArcStr) -> Result<i32> {
    let mut res: i32;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), literal!("oms_addBus"), list![metamodelica::Ref::new(Values::Value::STRING { string: cref })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::INTEGER { integer: __pa0 }) => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    res = metamodelica::Own::own(__pa0);
    Ok(res)
}

pub fn unloadOMSimulator() -> Result<i32> {
    let mut res: i32;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), literal!("unloadOMSimulator"), metamodelica::nil(), dummyMsg.clone())?) {
        (_, Deref @ Values::Value::INTEGER { integer: __pa0 }) => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    res = metamodelica::Own::own(__pa0);
    Ok(res)
}

pub fn loadOMSimulator() -> Result<i32> {
    let mut res: i32;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), literal!("loadOMSimulator"), metamodelica::nil(), dummyMsg.clone())?) {
        (_, Deref @ Values::Value::INTEGER { integer: __pa0 }) => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    res = metamodelica::Own::own(__pa0);
    Ok(res)
}

pub fn reverseLookup(
    mut name: ArcStr,
    mut scope: ArcStr,
    mut exactMatch: bool,
    mut prettyPrint: bool,
) -> Result<ArcStr> {
    let mut res: ArcStr;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), literal!("reverseLookup"), list![metamodelica::Ref::new(Values::Value::CODE { A: metamodelica::Ref::new(Absyn::CodeNode::C_TYPENAME { path: Parser::stringPath(name)? }) }), metamodelica::Ref::new(Values::Value::CODE { A: metamodelica::Ref::new(Absyn::CodeNode::C_TYPENAME { path: Parser::stringPath(scope)? }) }), metamodelica::Ref::new(Values::Value::BOOL { boolean: exactMatch }), metamodelica::Ref::new(Values::Value::BOOL { boolean: prettyPrint })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::STRING { string: __pa0 }) => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    res = metamodelica::Own::own(__pa0);
    Ok(res)
}

pub fn getClassDiagram(
    mut className: ArcStr,
    mut fileName: ArcStr,
    mut format: ArcStr,
    mut depth: i32,
    mut exclude: metamodelica::List<ArcStr>,
    mut showModifiers: bool,
) -> Result<ArcStr> {
    let mut res: ArcStr;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), literal!("getClassDiagram"), list![metamodelica::Ref::new(Values::Value::CODE { A: metamodelica::Ref::new(Absyn::CodeNode::C_TYPENAME { path: Parser::stringPath(className)? }) }), metamodelica::Ref::new(Values::Value::STRING { string: fileName }), metamodelica::Ref::new(Values::Value::STRING { string: format }), metamodelica::Ref::new(Values::Value::INTEGER { integer: depth }), ValuesMake::makeArray(({
        let mut __acc: metamodelica::List<metamodelica::Ref<Values::Value>> = metamodelica::nil();
        for mut exclude_iter in (exclude).into_iter().cloned() {
            let __x = metamodelica::Ref::new(Values::Value::STRING { string: exclude_iter.clone() });
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    })), metamodelica::Ref::new(Values::Value::BOOL { boolean: showModifiers })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::STRING { string: __pa0 }) => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    res = metamodelica::Own::own(__pa0);
    Ok(res)
}

pub fn getDefinitionAt(mut fileName: ArcStr, mut line: i32, mut column: i32, mut prettyPrint: bool) -> Result<ArcStr> {
    let mut res: ArcStr;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), literal!("getDefinitionAt"), list![metamodelica::Ref::new(Values::Value::STRING { string: fileName }), metamodelica::Ref::new(Values::Value::INTEGER { integer: line }), metamodelica::Ref::new(Values::Value::INTEGER { integer: column }), metamodelica::Ref::new(Values::Value::BOOL { boolean: prettyPrint })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::STRING { string: __pa0 }) => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    res = metamodelica::Own::own(__pa0);
    Ok(res)
}

pub fn getDependencyGraph(mut scope: ArcStr, mut fileName: ArcStr, mut prettyPrint: bool) -> Result<ArcStr> {
    let mut res: ArcStr;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), literal!("getDependencyGraph"), list![metamodelica::Ref::new(Values::Value::CODE { A: metamodelica::Ref::new(Absyn::CodeNode::C_TYPENAME { path: Parser::stringPath(scope)? }) }), metamodelica::Ref::new(Values::Value::STRING { string: fileName }), metamodelica::Ref::new(Values::Value::BOOL { boolean: prettyPrint })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::STRING { string: __pa0 }) => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    res = metamodelica::Own::own(__pa0);
    Ok(res)
}

pub fn getDefUseChains(
    mut className: ArcStr,
    mut fileName: ArcStr,
    mut scope: ArcStr,
    mut prettyPrint: bool,
) -> Result<ArcStr> {
    let mut res: ArcStr;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), literal!("getDefUseChains"), list![metamodelica::Ref::new(Values::Value::CODE { A: metamodelica::Ref::new(Absyn::CodeNode::C_TYPENAME { path: Parser::stringPath(className)? }) }), metamodelica::Ref::new(Values::Value::STRING { string: fileName }), metamodelica::Ref::new(Values::Value::CODE { A: metamodelica::Ref::new(Absyn::CodeNode::C_TYPENAME { path: Parser::stringPath(scope)? }) }), metamodelica::Ref::new(Values::Value::BOOL { boolean: prettyPrint })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::STRING { string: __pa0 }) => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    res = metamodelica::Own::own(__pa0);
    Ok(res)
}

pub fn getDefinitions(mut addFunctions: bool) -> Result<ArcStr> {
    let mut res: ArcStr;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), literal!("getDefinitions"), list![metamodelica::Ref::new(Values::Value::BOOL { boolean: addFunctions })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::STRING { string: __pa0 }) => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    res = metamodelica::Own::own(__pa0);
    Ok(res)
}

pub fn qualifyPath(mut classPath: ArcStr, mut path: ArcStr) -> Result<ArcStr> {
    let mut res: ArcStr;
    let mut res_path: metamodelica::Ref<Absyn::Path>;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), literal!("qualifyPath"), list![metamodelica::Ref::new(Values::Value::CODE { A: metamodelica::Ref::new(Absyn::CodeNode::C_TYPENAME { path: Parser::stringPath(classPath)? }) }), metamodelica::Ref::new(Values::Value::CODE { A: metamodelica::Ref::new(Absyn::CodeNode::C_TYPENAME { path: Parser::stringPath(path)? }) })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: __pa0 } }) => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    res_path = metamodelica::Own::own(__pa0);
    res = AbsynUtil::pathString(res_path, literal!("."), true, false)?;
    Ok(res)
}

pub fn restoreAST(mut id: i32) -> Result<bool> {
    let mut res: bool;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), literal!("restoreAST"), list![metamodelica::Ref::new(Values::Value::INTEGER { integer: id })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::BOOL { boolean: __pa0 }) => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    res = metamodelica::Own::own(__pa0);
    Ok(res)
}

pub fn storeAST() -> Result<i32> {
    let mut res: i32;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), literal!("storeAST"), metamodelica::nil(), dummyMsg.clone())?) {
        (_, Deref @ Values::Value::INTEGER { integer: __pa0 }) => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    res = metamodelica::Own::own(__pa0);
    Ok(res)
}

pub fn modifierToJSON(mut modifier: ArcStr, mut prettyPrint: bool) -> Result<ArcStr> {
    let mut res: ArcStr;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), literal!("modifierToJSON"), list![metamodelica::Ref::new(Values::Value::STRING { string: modifier }), metamodelica::Ref::new(Values::Value::BOOL { boolean: prettyPrint })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::STRING { string: __pa0 }) => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    res = metamodelica::Own::own(__pa0);
    Ok(res)
}

pub fn releaseModelInstanceReference(mut handle: i32) -> Result<bool> {
    let mut res: bool;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), literal!("releaseModelInstanceReference"), list![metamodelica::Ref::new(Values::Value::INTEGER { integer: handle })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::BOOL { boolean: __pa0 }) => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    res = metamodelica::Own::own(__pa0);
    Ok(res)
}

pub fn getModelInstanceAnnotationReference(
    mut className: ArcStr,
    mut filter: metamodelica::List<ArcStr>,
) -> Result<i32> {
    let mut res: i32;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), literal!("getModelInstanceAnnotationReference"), list![metamodelica::Ref::new(Values::Value::CODE { A: metamodelica::Ref::new(Absyn::CodeNode::C_TYPENAME { path: Parser::stringPath(className)? }) }), ValuesMake::makeArray(({
        let mut __acc: metamodelica::List<metamodelica::Ref<Values::Value>> = metamodelica::nil();
        for mut filter_iter in (filter).into_iter().cloned() {
            let __x = metamodelica::Ref::new(Values::Value::STRING { string: filter_iter.clone() });
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }))], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::INTEGER { integer: __pa0 }) => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    res = metamodelica::Own::own(__pa0);
    Ok(res)
}

pub fn getModelInstanceReference(mut className: ArcStr, mut context: ArcStr, mut modifier: ArcStr) -> Result<i32> {
    let mut res: i32;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), literal!("getModelInstanceReference"), list![metamodelica::Ref::new(Values::Value::CODE { A: metamodelica::Ref::new(Absyn::CodeNode::C_TYPENAME { path: Parser::stringPath(className)? }) }), metamodelica::Ref::new(Values::Value::CODE { A: metamodelica::Ref::new(Absyn::CodeNode::C_TYPENAME { path: Parser::stringPath(context)? }) }), metamodelica::Ref::new(Values::Value::STRING { string: modifier })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::INTEGER { integer: __pa0 }) => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    res = metamodelica::Own::own(__pa0);
    Ok(res)
}

pub fn getModelInstanceAnnotation(
    mut className: ArcStr,
    mut filter: metamodelica::List<ArcStr>,
    mut prettyPrint: bool,
) -> Result<ArcStr> {
    let mut res: ArcStr;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), literal!("getModelInstanceAnnotation"), list![metamodelica::Ref::new(Values::Value::CODE { A: metamodelica::Ref::new(Absyn::CodeNode::C_TYPENAME { path: Parser::stringPath(className)? }) }), ValuesMake::makeArray(({
        let mut __acc: metamodelica::List<metamodelica::Ref<Values::Value>> = metamodelica::nil();
        for mut filter_iter in (filter).into_iter().cloned() {
            let __x = metamodelica::Ref::new(Values::Value::STRING { string: filter_iter.clone() });
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    })), metamodelica::Ref::new(Values::Value::BOOL { boolean: prettyPrint })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::STRING { string: __pa0 }) => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    res = metamodelica::Own::own(__pa0);
    Ok(res)
}

pub fn getModelInstance(
    mut className: ArcStr,
    mut context: ArcStr,
    mut modifier: ArcStr,
    mut prettyPrint: bool,
) -> Result<ArcStr> {
    let mut res: ArcStr;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), literal!("getModelInstance"), list![metamodelica::Ref::new(Values::Value::CODE { A: metamodelica::Ref::new(Absyn::CodeNode::C_TYPENAME { path: Parser::stringPath(className)? }) }), metamodelica::Ref::new(Values::Value::CODE { A: metamodelica::Ref::new(Absyn::CodeNode::C_TYPENAME { path: Parser::stringPath(context)? }) }), metamodelica::Ref::new(Values::Value::STRING { string: modifier }), metamodelica::Ref::new(Values::Value::BOOL { boolean: prettyPrint })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::STRING { string: __pa0 }) => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    res = metamodelica::Own::own(__pa0);
    Ok(res)
}

pub fn convertPackageToLibrary(
    mut packageToConvert: ArcStr,
    mut library: ArcStr,
    mut libraryVersion: ArcStr,
) -> Result<bool> {
    let mut res: bool;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), literal!("convertPackageToLibrary"), list![metamodelica::Ref::new(Values::Value::CODE { A: metamodelica::Ref::new(Absyn::CodeNode::C_TYPENAME { path: Parser::stringPath(packageToConvert)? }) }), metamodelica::Ref::new(Values::Value::CODE { A: metamodelica::Ref::new(Absyn::CodeNode::C_TYPENAME { path: Parser::stringPath(library)? }) }), metamodelica::Ref::new(Values::Value::STRING { string: libraryVersion })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::BOOL { boolean: __pa0 }) => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    res = metamodelica::Own::own(__pa0);
    Ok(res)
}

pub fn runConversionScript(mut packageToConvert: ArcStr, mut scriptFile: ArcStr) -> Result<bool> {
    let mut res: bool;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), literal!("runConversionScript"), list![metamodelica::Ref::new(Values::Value::CODE { A: metamodelica::Ref::new(Absyn::CodeNode::C_TYPENAME { path: Parser::stringPath(packageToConvert)? }) }), metamodelica::Ref::new(Values::Value::STRING { string: scriptFile })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::BOOL { boolean: __pa0 }) => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    res = metamodelica::Own::own(__pa0);
    Ok(res)
}

pub fn generateScriptingAPI(mut cl: ArcStr, mut name: ArcStr) -> Result<(bool, ArcStr, ArcStr, ArcStr)> {
    let mut res1: bool;
    let mut res2: ArcStr;
    let mut res3: ArcStr;
    let mut res4: ArcStr;
    let (__pa0, __pa1, __pa2, __pa3) = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), literal!("generateScriptingAPI"), list![metamodelica::Ref::new(Values::Value::CODE { A: metamodelica::Ref::new(Absyn::CodeNode::C_TYPENAME { path: Parser::stringPath(cl)? }) }), metamodelica::Ref::new(Values::Value::STRING { string: name })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::TUPLE { valueLst: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::BOOL { boolean: __pa0 }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: __pa1 }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: __pa2 }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: __pa3 }, tail: Deref @ metamodelica::ListNode::Nil } } } } }) => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone()),
        _ => return Err("pattern mismatch"),
    } };
    res1 = metamodelica::Own::own(__pa0);
    res2 = metamodelica::Own::own(__pa1);
    res3 = metamodelica::Own::own(__pa2);
    res4 = metamodelica::Own::own(__pa3);
    Ok((res1, res2, res3, res4))
}

pub fn deleteInitialState(mut cl: ArcStr, mut state: ArcStr) -> Result<bool> {
    let mut res: bool;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), literal!("deleteInitialState"), list![metamodelica::Ref::new(Values::Value::CODE { A: metamodelica::Ref::new(Absyn::CodeNode::C_TYPENAME { path: Parser::stringPath(cl)? }) }), metamodelica::Ref::new(Values::Value::STRING { string: state })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::BOOL { boolean: __pa0 }) => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    res = metamodelica::Own::own(__pa0);
    Ok(res)
}

pub fn getInitialStates(mut cl: ArcStr) -> Result<metamodelica::List<metamodelica::List<ArcStr>>> {
    let mut res: metamodelica::List<metamodelica::List<ArcStr>>;
    let mut res_arr: metamodelica::Ref<Values::Value>;
    (_, res_arr) = CevalScript::cevalInteractiveFunctions2(
        FCore::emptyCache(),
        FGraph::empty(),
        literal!("getInitialStates"),
        list![metamodelica::Ref::new(Values::Value::CODE {
            A: metamodelica::Ref::new(Absyn::CodeNode::C_TYPENAME {
                path: Parser::stringPath(cl)?
            })
        })],
        dummyMsg.clone(),
    )?;
    res = ({
        let mut __acc: metamodelica::List<metamodelica::List<ArcStr>> = metamodelica::nil();
        for mut res_arr_iter in (ValuesUtil::arrayValues(&res_arr)?).into_iter().cloned() {
            let __x = ({
                let mut __acc: metamodelica::List<ArcStr> = metamodelica::nil();
                for mut res_arr_iter_iter in (ValuesUtil::arrayValues(&(res_arr_iter.clone()))?).into_iter().cloned() {
                    let __x = (match &*res_arr_iter_iter.clone() {
                        Values::Value::STRING {
                            string: __res_arr_iter_iter_string,
                        } => __res_arr_iter_iter_string.clone(),
                        _ => return Err("match: no arm matched"),
                    });
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            });
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    Ok(res)
}

pub fn deleteTransition(
    mut cl: ArcStr,
    mut from: ArcStr,
    mut to: ArcStr,
    mut condition: ArcStr,
    mut immediate: bool,
    mut reset: bool,
    mut synchronize: bool,
    mut priority: i32,
) -> Result<bool> {
    let mut res: bool;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), literal!("deleteTransition"), list![metamodelica::Ref::new(Values::Value::CODE { A: metamodelica::Ref::new(Absyn::CodeNode::C_TYPENAME { path: Parser::stringPath(cl)? }) }), metamodelica::Ref::new(Values::Value::STRING { string: from }), metamodelica::Ref::new(Values::Value::STRING { string: to }), metamodelica::Ref::new(Values::Value::STRING { string: condition }), metamodelica::Ref::new(Values::Value::BOOL { boolean: immediate }), metamodelica::Ref::new(Values::Value::BOOL { boolean: reset }), metamodelica::Ref::new(Values::Value::BOOL { boolean: synchronize }), metamodelica::Ref::new(Values::Value::INTEGER { integer: priority })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::BOOL { boolean: __pa0 }) => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    res = metamodelica::Own::own(__pa0);
    Ok(res)
}

pub fn getTransitions(mut cl: ArcStr) -> Result<metamodelica::List<metamodelica::List<ArcStr>>> {
    let mut res: metamodelica::List<metamodelica::List<ArcStr>>;
    let mut res_arr: metamodelica::Ref<Values::Value>;
    (_, res_arr) = CevalScript::cevalInteractiveFunctions2(
        FCore::emptyCache(),
        FGraph::empty(),
        literal!("getTransitions"),
        list![metamodelica::Ref::new(Values::Value::CODE {
            A: metamodelica::Ref::new(Absyn::CodeNode::C_TYPENAME {
                path: Parser::stringPath(cl)?
            })
        })],
        dummyMsg.clone(),
    )?;
    res = ({
        let mut __acc: metamodelica::List<metamodelica::List<ArcStr>> = metamodelica::nil();
        for mut res_arr_iter in (ValuesUtil::arrayValues(&res_arr)?).into_iter().cloned() {
            let __x = ({
                let mut __acc: metamodelica::List<ArcStr> = metamodelica::nil();
                for mut res_arr_iter_iter in (ValuesUtil::arrayValues(&(res_arr_iter.clone()))?).into_iter().cloned() {
                    let __x = (match &*res_arr_iter_iter.clone() {
                        Values::Value::STRING {
                            string: __res_arr_iter_iter_string,
                        } => __res_arr_iter_iter_string.clone(),
                        _ => return Err("match: no arm matched"),
                    });
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            });
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    Ok(res)
}

pub fn getEnumerationLiterals(mut className: ArcStr) -> Result<metamodelica::List<ArcStr>> {
    let mut res: metamodelica::List<ArcStr>;
    let mut res_arr: metamodelica::Ref<Values::Value>;
    (_, res_arr) = CevalScript::cevalInteractiveFunctions2(
        FCore::emptyCache(),
        FGraph::empty(),
        literal!("getEnumerationLiterals"),
        list![metamodelica::Ref::new(Values::Value::CODE {
            A: metamodelica::Ref::new(Absyn::CodeNode::C_TYPENAME {
                path: Parser::stringPath(className)?
            })
        })],
        dummyMsg.clone(),
    )?;
    res = ({
        let mut __acc: metamodelica::List<ArcStr> = metamodelica::nil();
        for mut res_arr_iter in (ValuesUtil::arrayValues(&res_arr)?).into_iter().cloned() {
            let __x = (match &*res_arr_iter.clone() {
                Values::Value::STRING {
                    string: __res_arr_iter_string,
                } => __res_arr_iter_string.clone(),
                _ => return Err("match: no arm matched"),
            });
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    Ok(res)
}

pub fn getDefaultComponentPrefixes(mut cl: ArcStr) -> Result<ArcStr> {
    let mut res: ArcStr;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), literal!("getDefaultComponentPrefixes"), list![metamodelica::Ref::new(Values::Value::CODE { A: metamodelica::Ref::new(Absyn::CodeNode::C_TYPENAME { path: Parser::stringPath(cl)? }) })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::STRING { string: __pa0 }) => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    res = metamodelica::Own::own(__pa0);
    Ok(res)
}

pub fn getDefaultComponentName(mut cl: ArcStr) -> Result<ArcStr> {
    let mut res: ArcStr;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), literal!("getDefaultComponentName"), list![metamodelica::Ref::new(Values::Value::CODE { A: metamodelica::Ref::new(Absyn::CodeNode::C_TYPENAME { path: Parser::stringPath(cl)? }) })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::STRING { string: __pa0 }) => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    res = metamodelica::Own::own(__pa0);
    Ok(res)
}

pub fn getClassInformation(
    mut cl: ArcStr,
) -> Result<(
    ArcStr,
    ArcStr,
    bool,
    bool,
    bool,
    ArcStr,
    bool,
    i32,
    i32,
    i32,
    i32,
    metamodelica::List<ArcStr>,
    bool,
    bool,
    ArcStr,
    ArcStr,
    bool,
    ArcStr,
    ArcStr,
    ArcStr,
    ArcStr,
    ArcStr,
)> {
    let mut res1: ArcStr;
    let mut res2: ArcStr;
    let mut res3: bool;
    let mut res4: bool;
    let mut res5: bool;
    let mut res6: ArcStr;
    let mut res7: bool;
    let mut res8: i32;
    let mut res9: i32;
    let mut res10: i32;
    let mut res11: i32;
    let mut res12: metamodelica::List<ArcStr>;
    let mut res13: bool;
    let mut res14: bool;
    let mut res15: ArcStr;
    let mut res16: ArcStr;
    let mut res17: bool;
    let mut res18: ArcStr;
    let mut res19: ArcStr;
    let mut res20: ArcStr;
    let mut res21: ArcStr;
    let mut res22: ArcStr;
    let mut res12_arr: metamodelica::Ref<Values::Value>;
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
        __pa17,
        __pa18,
        __pa19,
        __pa20,
        __pa21,
    ) = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), literal!("getClassInformation"), list![metamodelica::Ref::new(Values::Value::CODE { A: metamodelica::Ref::new(Absyn::CodeNode::C_TYPENAME { path: Parser::stringPath(cl)? }) })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::TUPLE { valueLst: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: __pa0 }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: __pa1 }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::BOOL { boolean: __pa2 }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::BOOL { boolean: __pa3 }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::BOOL { boolean: __pa4 }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: __pa5 }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::BOOL { boolean: __pa6 }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::INTEGER { integer: __pa7 }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::INTEGER { integer: __pa8 }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::INTEGER { integer: __pa9 }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::INTEGER { integer: __pa10 }, tail: Deref @ metamodelica::ListNode::Cons { head: __pa11, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::BOOL { boolean: __pa12 }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::BOOL { boolean: __pa13 }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: __pa14 }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: __pa15 }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::BOOL { boolean: __pa16 }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: __pa17 }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: __pa18 }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: __pa19 }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: __pa20 }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: __pa21 }, tail: Deref @ metamodelica::ListNode::Nil } } } } } } } } } } } } } } } } } } } } } } }) => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone(), __pa4.clone(), __pa5.clone(), __pa6.clone(), __pa7.clone(), __pa8.clone(), __pa9.clone(), __pa10.clone(), __pa11.clone(), __pa12.clone(), __pa13.clone(), __pa14.clone(), __pa15.clone(), __pa16.clone(), __pa17.clone(), __pa18.clone(), __pa19.clone(), __pa20.clone(), __pa21.clone()),
        _ => return Err("pattern mismatch"),
    } };
    res1 = metamodelica::Own::own(__pa0);
    res2 = metamodelica::Own::own(__pa1);
    res3 = metamodelica::Own::own(__pa2);
    res4 = metamodelica::Own::own(__pa3);
    res5 = metamodelica::Own::own(__pa4);
    res6 = metamodelica::Own::own(__pa5);
    res7 = metamodelica::Own::own(__pa6);
    res8 = metamodelica::Own::own(__pa7);
    res9 = metamodelica::Own::own(__pa8);
    res10 = metamodelica::Own::own(__pa9);
    res11 = metamodelica::Own::own(__pa10);
    res12_arr = metamodelica::Own::own(__pa11);
    res13 = metamodelica::Own::own(__pa12);
    res14 = metamodelica::Own::own(__pa13);
    res15 = metamodelica::Own::own(__pa14);
    res16 = metamodelica::Own::own(__pa15);
    res17 = metamodelica::Own::own(__pa16);
    res18 = metamodelica::Own::own(__pa17);
    res19 = metamodelica::Own::own(__pa18);
    res20 = metamodelica::Own::own(__pa19);
    res21 = metamodelica::Own::own(__pa20);
    res22 = metamodelica::Own::own(__pa21);
    res12 = ({
        let mut __acc: metamodelica::List<ArcStr> = metamodelica::nil();
        for mut res12_arr_iter in (ValuesUtil::arrayValues(&res12_arr)?).into_iter().cloned() {
            let __x = (match &*res12_arr_iter.clone() {
                Values::Value::STRING {
                    string: __res12_arr_iter_string,
                } => __res12_arr_iter_string.clone(),
                _ => return Err("match: no arm matched"),
            });
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    Ok((
        res1, res2, res3, res4, res5, res6, res7, res8, res9, res10, res11, res12, res13, res14, res15, res16, res17,
        res18, res19, res20, res21, res22,
    ))
}

pub fn sortStrings(mut arr: metamodelica::List<ArcStr>) -> Result<metamodelica::List<ArcStr>> {
    let mut res: metamodelica::List<ArcStr>;
    let mut res_arr: metamodelica::Ref<Values::Value>;
    (_, res_arr) = CevalScript::cevalInteractiveFunctions2(
        FCore::emptyCache(),
        FGraph::empty(),
        literal!("sortStrings"),
        list![ValuesMake::makeArray(
            ({
                let mut __acc: metamodelica::List<metamodelica::Ref<Values::Value>> = metamodelica::nil();
                for mut arr_iter in (arr).into_iter().cloned() {
                    let __x = metamodelica::Ref::new(Values::Value::STRING {
                        string: arr_iter.clone(),
                    });
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            })
        )],
        dummyMsg.clone(),
    )?;
    res = ({
        let mut __acc: metamodelica::List<ArcStr> = metamodelica::nil();
        for mut res_arr_iter in (ValuesUtil::arrayValues(&res_arr)?).into_iter().cloned() {
            let __x = (match &*res_arr_iter.clone() {
                Values::Value::STRING {
                    string: __res_arr_iter_string,
                } => __res_arr_iter_string.clone(),
                _ => return Err("match: no arm matched"),
            });
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    Ok(res)
}

pub fn checkInterfaceOfPackages(
    mut cl: ArcStr,
    mut dependencyMatrix: metamodelica::List<metamodelica::List<ArcStr>>,
) -> Result<bool> {
    let mut res: bool;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), literal!("checkInterfaceOfPackages"), list![metamodelica::Ref::new(Values::Value::CODE { A: metamodelica::Ref::new(Absyn::CodeNode::C_TYPENAME { path: Parser::stringPath(cl)? }) }), ValuesMake::makeArray(({
        let mut __acc: metamodelica::List<metamodelica::Ref<Values::Value>> = metamodelica::nil();
        for mut dependencyMatrix_iter in (dependencyMatrix).into_iter().cloned() {
            let __x = ValuesMake::makeArray(({
        let mut __acc: metamodelica::List<metamodelica::Ref<Values::Value>> = metamodelica::nil();
        for mut dependencyMatrix_iter_iter in (dependencyMatrix_iter.clone()).into_iter().cloned() {
            let __x = metamodelica::Ref::new(Values::Value::STRING { string: dependencyMatrix_iter_iter.clone() });
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }));
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }))], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::BOOL { boolean: __pa0 }) => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    res = metamodelica::Own::own(__pa0);
    Ok(res)
}

pub fn GC_set_max_heap_size(mut size: i32) -> Result<bool> {
    let mut res: bool;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), literal!("GC_set_max_heap_size"), list![metamodelica::Ref::new(Values::Value::INTEGER { integer: size })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::BOOL { boolean: __pa0 }) => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    res = metamodelica::Own::own(__pa0);
    Ok(res)
}

pub fn GC_expand_hp(mut size: i32) -> Result<bool> {
    let mut res: bool;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), literal!("GC_expand_hp"), list![metamodelica::Ref::new(Values::Value::INTEGER { integer: size })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::BOOL { boolean: __pa0 }) => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    res = metamodelica::Own::own(__pa0);
    Ok(res)
}

pub fn GC_gcollect_and_unmap() -> Result<()> {
    ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), literal!("GC_gcollect_and_unmap"), metamodelica::nil(), dummyMsg.clone())?) {
        (_, Deref @ Values::Value::NORETCALL { .. }) => (),
        _ => return Err("pattern mismatch"),
    } };
    Ok(())
}

pub fn getMemorySize() -> Result<metamodelica::Real> {
    let mut res: metamodelica::Real;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), literal!("getMemorySize"), metamodelica::nil(), dummyMsg.clone())?) {
        (_, Deref @ Values::Value::REAL { real: __pa0 }) => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    res = metamodelica::Own::own(__pa0);
    Ok(res)
}

pub fn threadWorkFailed() -> Result<()> {
    ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), literal!("threadWorkFailed"), metamodelica::nil(), dummyMsg.clone())?) {
        (_, Deref @ Values::Value::NORETCALL { .. }) => (),
        _ => return Err("pattern mismatch"),
    } };
    Ok(())
}

pub fn exit(mut status: i32) -> Result<()> {
    ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), literal!("exit"), list![metamodelica::Ref::new(Values::Value::INTEGER { integer: status })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::NORETCALL { .. }) => (),
        _ => return Err("pattern mismatch"),
    } };
    Ok(())
}

pub fn runScriptParallel(
    mut scripts: metamodelica::List<ArcStr>,
    mut numThreads: i32,
    mut useThreads: bool,
) -> Result<metamodelica::List<bool>> {
    let mut res: metamodelica::List<bool>;
    let mut res_arr: metamodelica::Ref<Values::Value>;
    (_, res_arr) = CevalScript::cevalInteractiveFunctions2(
        FCore::emptyCache(),
        FGraph::empty(),
        literal!("runScriptParallel"),
        list![
            ValuesMake::makeArray(
                ({
                    let mut __acc: metamodelica::List<metamodelica::Ref<Values::Value>> = metamodelica::nil();
                    for mut scripts_iter in (scripts).into_iter().cloned() {
                        let __x = metamodelica::Ref::new(Values::Value::STRING {
                            string: scripts_iter.clone(),
                        });
                        __acc = cons(__x, __acc);
                    }
                    __acc.reverse()
                })
            ),
            metamodelica::Ref::new(Values::Value::INTEGER { integer: numThreads }),
            metamodelica::Ref::new(Values::Value::BOOL { boolean: useThreads })
        ],
        dummyMsg.clone(),
    )?;
    res = ({
        let mut __acc: metamodelica::List<bool> = metamodelica::nil();
        for mut res_arr_iter in (ValuesUtil::arrayValues(&res_arr)?).into_iter().cloned() {
            let __x = (match &*res_arr_iter.clone() {
                Values::Value::BOOL {
                    boolean: __res_arr_iter_boolean,
                } => __res_arr_iter_boolean.clone(),
                _ => return Err("match: no arm matched"),
            });
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    Ok(res)
}

pub fn numProcessors() -> Result<i32> {
    let mut res: i32;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), literal!("numProcessors"), metamodelica::nil(), dummyMsg.clone())?) {
        (_, Deref @ Values::Value::INTEGER { integer: __pa0 }) => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    res = metamodelica::Own::own(__pa0);
    Ok(res)
}

pub fn generateEntryPoint(mut fileName: ArcStr, mut entryPoint: ArcStr, mut url: ArcStr) -> Result<()> {
    ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), literal!("generateEntryPoint"), list![metamodelica::Ref::new(Values::Value::STRING { string: fileName }), metamodelica::Ref::new(Values::Value::CODE { A: metamodelica::Ref::new(Absyn::CodeNode::C_TYPENAME { path: Parser::stringPath(entryPoint)? }) }), metamodelica::Ref::new(Values::Value::STRING { string: url })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::NORETCALL { .. }) => (),
        _ => return Err("pattern mismatch"),
    } };
    Ok(())
}

pub fn getDerivedClassModifierValue(mut className: ArcStr, mut modifierName: ArcStr) -> Result<ArcStr> {
    let mut res: ArcStr;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), literal!("getDerivedClassModifierValue"), list![metamodelica::Ref::new(Values::Value::CODE { A: metamodelica::Ref::new(Absyn::CodeNode::C_TYPENAME { path: Parser::stringPath(className)? }) }), metamodelica::Ref::new(Values::Value::CODE { A: metamodelica::Ref::new(Absyn::CodeNode::C_TYPENAME { path: Parser::stringPath(modifierName)? }) })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::STRING { string: __pa0 }) => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    res = metamodelica::Own::own(__pa0);
    Ok(res)
}

pub fn getDerivedClassModifierNames(mut className: ArcStr) -> Result<metamodelica::List<ArcStr>> {
    let mut res: metamodelica::List<ArcStr>;
    let mut res_arr: metamodelica::Ref<Values::Value>;
    (_, res_arr) = CevalScript::cevalInteractiveFunctions2(
        FCore::emptyCache(),
        FGraph::empty(),
        literal!("getDerivedClassModifierNames"),
        list![metamodelica::Ref::new(Values::Value::CODE {
            A: metamodelica::Ref::new(Absyn::CodeNode::C_TYPENAME {
                path: Parser::stringPath(className)?
            })
        })],
        dummyMsg.clone(),
    )?;
    res = ({
        let mut __acc: metamodelica::List<ArcStr> = metamodelica::nil();
        for mut res_arr_iter in (ValuesUtil::arrayValues(&res_arr)?).into_iter().cloned() {
            let __x = (match &*res_arr_iter.clone() {
                Values::Value::STRING {
                    string: __res_arr_iter_string,
                } => __res_arr_iter_string.clone(),
                _ => return Err("match: no arm matched"),
            });
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    Ok(res)
}

pub fn getConversionsFromVersions(
    mut pack: ArcStr,
) -> Result<(metamodelica::List<ArcStr>, metamodelica::List<ArcStr>)> {
    let mut res1: metamodelica::List<ArcStr>;
    let mut res2: metamodelica::List<ArcStr>;
    let mut res1_arr: metamodelica::Ref<Values::Value>;
    let mut res2_arr: metamodelica::Ref<Values::Value>;
    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), literal!("getConversionsFromVersions"), list![metamodelica::Ref::new(Values::Value::CODE { A: metamodelica::Ref::new(Absyn::CodeNode::C_TYPENAME { path: Parser::stringPath(pack)? }) })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::TUPLE { valueLst: Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: Deref @ metamodelica::ListNode::Cons { head: __pa1, tail: Deref @ metamodelica::ListNode::Nil } } }) => (__pa0.clone(), __pa1.clone()),
        _ => return Err("pattern mismatch"),
    } };
    res1_arr = metamodelica::Own::own(__pa0);
    res2_arr = metamodelica::Own::own(__pa1);
    res1 = ({
        let mut __acc: metamodelica::List<ArcStr> = metamodelica::nil();
        for mut res1_arr_iter in (ValuesUtil::arrayValues(&res1_arr)?).into_iter().cloned() {
            let __x = (match &*res1_arr_iter.clone() {
                Values::Value::STRING {
                    string: __res1_arr_iter_string,
                } => __res1_arr_iter_string.clone(),
                _ => return Err("match: no arm matched"),
            });
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    res2 = ({
        let mut __acc: metamodelica::List<ArcStr> = metamodelica::nil();
        for mut res2_arr_iter in (ValuesUtil::arrayValues(&res2_arr)?).into_iter().cloned() {
            let __x = (match &*res2_arr_iter.clone() {
                Values::Value::STRING {
                    string: __res2_arr_iter_string,
                } => __res2_arr_iter_string.clone(),
                _ => return Err("match: no arm matched"),
            });
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    Ok((res1, res2))
}

pub fn getUses(mut pack: ArcStr) -> Result<metamodelica::List<metamodelica::List<ArcStr>>> {
    let mut res: metamodelica::List<metamodelica::List<ArcStr>>;
    let mut res_arr: metamodelica::Ref<Values::Value>;
    (_, res_arr) = CevalScript::cevalInteractiveFunctions2(
        FCore::emptyCache(),
        FGraph::empty(),
        literal!("getUses"),
        list![metamodelica::Ref::new(Values::Value::CODE {
            A: metamodelica::Ref::new(Absyn::CodeNode::C_TYPENAME {
                path: Parser::stringPath(pack)?
            })
        })],
        dummyMsg.clone(),
    )?;
    res = ({
        let mut __acc: metamodelica::List<metamodelica::List<ArcStr>> = metamodelica::nil();
        for mut res_arr_iter in (ValuesUtil::arrayValues(&res_arr)?).into_iter().cloned() {
            let __x = ({
                let mut __acc: metamodelica::List<ArcStr> = metamodelica::nil();
                for mut res_arr_iter_iter in (ValuesUtil::arrayValues(&(res_arr_iter.clone()))?).into_iter().cloned() {
                    let __x = (match &*res_arr_iter_iter.clone() {
                        Values::Value::STRING {
                            string: __res_arr_iter_iter_string,
                        } => __res_arr_iter_iter_string.clone(),
                        _ => return Err("match: no arm matched"),
                    });
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            });
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    Ok(res)
}

pub fn upgradeInstalledPackages(mut installNewestVersions: bool) -> Result<bool> {
    let mut res: bool;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), literal!("upgradeInstalledPackages"), list![metamodelica::Ref::new(Values::Value::BOOL { boolean: installNewestVersions })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::BOOL { boolean: __pa0 }) => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    res = metamodelica::Own::own(__pa0);
    Ok(res)
}

pub fn getAvailablePackageConversionsFrom(mut pkg: ArcStr, mut version: ArcStr) -> Result<metamodelica::List<ArcStr>> {
    let mut res: metamodelica::List<ArcStr>;
    let mut res_arr: metamodelica::Ref<Values::Value>;
    (_, res_arr) = CevalScript::cevalInteractiveFunctions2(
        FCore::emptyCache(),
        FGraph::empty(),
        literal!("getAvailablePackageConversionsFrom"),
        list![
            metamodelica::Ref::new(Values::Value::CODE {
                A: metamodelica::Ref::new(Absyn::CodeNode::C_TYPENAME {
                    path: Parser::stringPath(pkg)?
                })
            }),
            metamodelica::Ref::new(Values::Value::STRING { string: version })
        ],
        dummyMsg.clone(),
    )?;
    res = ({
        let mut __acc: metamodelica::List<ArcStr> = metamodelica::nil();
        for mut res_arr_iter in (ValuesUtil::arrayValues(&res_arr)?).into_iter().cloned() {
            let __x = (match &*res_arr_iter.clone() {
                Values::Value::STRING {
                    string: __res_arr_iter_string,
                } => __res_arr_iter_string.clone(),
                _ => return Err("match: no arm matched"),
            });
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    Ok(res)
}

pub fn getAvailablePackageConversionsTo(mut pkg: ArcStr, mut version: ArcStr) -> Result<metamodelica::List<ArcStr>> {
    let mut res: metamodelica::List<ArcStr>;
    let mut res_arr: metamodelica::Ref<Values::Value>;
    (_, res_arr) = CevalScript::cevalInteractiveFunctions2(
        FCore::emptyCache(),
        FGraph::empty(),
        literal!("getAvailablePackageConversionsTo"),
        list![
            metamodelica::Ref::new(Values::Value::CODE {
                A: metamodelica::Ref::new(Absyn::CodeNode::C_TYPENAME {
                    path: Parser::stringPath(pkg)?
                })
            }),
            metamodelica::Ref::new(Values::Value::STRING { string: version })
        ],
        dummyMsg.clone(),
    )?;
    res = ({
        let mut __acc: metamodelica::List<ArcStr> = metamodelica::nil();
        for mut res_arr_iter in (ValuesUtil::arrayValues(&res_arr)?).into_iter().cloned() {
            let __x = (match &*res_arr_iter.clone() {
                Values::Value::STRING {
                    string: __res_arr_iter_string,
                } => __res_arr_iter_string.clone(),
                _ => return Err("match: no arm matched"),
            });
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    Ok(res)
}

pub fn getAvailablePackageVersions(mut pkg: ArcStr, mut version: ArcStr) -> Result<metamodelica::List<ArcStr>> {
    let mut res: metamodelica::List<ArcStr>;
    let mut res_arr: metamodelica::Ref<Values::Value>;
    (_, res_arr) = CevalScript::cevalInteractiveFunctions2(
        FCore::emptyCache(),
        FGraph::empty(),
        literal!("getAvailablePackageVersions"),
        list![
            metamodelica::Ref::new(Values::Value::CODE {
                A: metamodelica::Ref::new(Absyn::CodeNode::C_TYPENAME {
                    path: Parser::stringPath(pkg)?
                })
            }),
            metamodelica::Ref::new(Values::Value::STRING { string: version })
        ],
        dummyMsg.clone(),
    )?;
    res = ({
        let mut __acc: metamodelica::List<ArcStr> = metamodelica::nil();
        for mut res_arr_iter in (ValuesUtil::arrayValues(&res_arr)?).into_iter().cloned() {
            let __x = (match &*res_arr_iter.clone() {
                Values::Value::STRING {
                    string: __res_arr_iter_string,
                } => __res_arr_iter_string.clone(),
                _ => return Err("match: no arm matched"),
            });
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    Ok(res)
}

pub fn updatePackageIndex() -> Result<bool> {
    let mut res: bool;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), literal!("updatePackageIndex"), metamodelica::nil(), dummyMsg.clone())?) {
        (_, Deref @ Values::Value::BOOL { boolean: __pa0 }) => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    res = metamodelica::Own::own(__pa0);
    Ok(res)
}

pub fn installPackage(mut pkg: ArcStr, mut version: ArcStr, mut exactMatch: bool) -> Result<bool> {
    let mut res: bool;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), literal!("installPackage"), list![metamodelica::Ref::new(Values::Value::CODE { A: metamodelica::Ref::new(Absyn::CodeNode::C_TYPENAME { path: Parser::stringPath(pkg)? }) }), metamodelica::Ref::new(Values::Value::STRING { string: version }), metamodelica::Ref::new(Values::Value::BOOL { boolean: exactMatch })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::BOOL { boolean: __pa0 }) => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    res = metamodelica::Own::own(__pa0);
    Ok(res)
}

pub fn getAvailableLibraryVersions(mut libraryName: ArcStr) -> Result<metamodelica::List<ArcStr>> {
    let mut res: metamodelica::List<ArcStr>;
    let mut res_arr: metamodelica::Ref<Values::Value>;
    (_, res_arr) = CevalScript::cevalInteractiveFunctions2(
        FCore::emptyCache(),
        FGraph::empty(),
        literal!("getAvailableLibraryVersions"),
        list![metamodelica::Ref::new(Values::Value::CODE {
            A: metamodelica::Ref::new(Absyn::CodeNode::C_TYPENAME {
                path: Parser::stringPath(libraryName)?
            })
        })],
        dummyMsg.clone(),
    )?;
    res = ({
        let mut __acc: metamodelica::List<ArcStr> = metamodelica::nil();
        for mut res_arr_iter in (ValuesUtil::arrayValues(&res_arr)?).into_iter().cloned() {
            let __x = (match &*res_arr_iter.clone() {
                Values::Value::STRING {
                    string: __res_arr_iter_string,
                } => __res_arr_iter_string.clone(),
                _ => return Err("match: no arm matched"),
            });
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    Ok(res)
}

pub fn getAvailableLibraries() -> Result<metamodelica::List<ArcStr>> {
    let mut res: metamodelica::List<ArcStr>;
    let mut res_arr: metamodelica::Ref<Values::Value>;
    (_, res_arr) = CevalScript::cevalInteractiveFunctions2(
        FCore::emptyCache(),
        FGraph::empty(),
        literal!("getAvailableLibraries"),
        metamodelica::nil(),
        dummyMsg.clone(),
    )?;
    res = ({
        let mut __acc: metamodelica::List<ArcStr> = metamodelica::nil();
        for mut res_arr_iter in (ValuesUtil::arrayValues(&res_arr)?).into_iter().cloned() {
            let __x = (match &*res_arr_iter.clone() {
                Values::Value::STRING {
                    string: __res_arr_iter_string,
                } => __res_arr_iter_string.clone(),
                _ => return Err("match: no arm matched"),
            });
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    Ok(res)
}

pub fn searchClassNames(mut searchText: ArcStr, mut findInText: bool) -> Result<metamodelica::List<ArcStr>> {
    let mut res: metamodelica::List<ArcStr>;
    let mut res_arr: metamodelica::Ref<Values::Value>;
    (_, res_arr) = CevalScript::cevalInteractiveFunctions2(
        FCore::emptyCache(),
        FGraph::empty(),
        literal!("searchClassNames"),
        list![
            metamodelica::Ref::new(Values::Value::STRING { string: searchText }),
            metamodelica::Ref::new(Values::Value::BOOL { boolean: findInText })
        ],
        dummyMsg.clone(),
    )?;
    res = ({
        let mut __acc: metamodelica::List<ArcStr> = metamodelica::nil();
        for mut res_arr_iter in (ValuesUtil::arrayValues(&res_arr)?).into_iter().cloned() {
            let __x = ValuesDump::valString(&(res_arr_iter.clone()))?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    Ok(res)
}

pub fn extendsFrom(mut className: ArcStr, mut baseClassName: ArcStr) -> Result<bool> {
    let mut res: bool;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), literal!("extendsFrom"), list![metamodelica::Ref::new(Values::Value::CODE { A: metamodelica::Ref::new(Absyn::CodeNode::C_TYPENAME { path: Parser::stringPath(className)? }) }), metamodelica::Ref::new(Values::Value::CODE { A: metamodelica::Ref::new(Absyn::CodeNode::C_TYPENAME { path: Parser::stringPath(baseClassName)? }) })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::BOOL { boolean: __pa0 }) => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    res = metamodelica::Own::own(__pa0);
    Ok(res)
}

pub fn getBooleanClassAnnotation(mut className: ArcStr, mut annotationName: ArcStr) -> Result<bool> {
    let mut res: bool;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), literal!("getBooleanClassAnnotation"), list![metamodelica::Ref::new(Values::Value::CODE { A: metamodelica::Ref::new(Absyn::CodeNode::C_TYPENAME { path: Parser::stringPath(className)? }) }), metamodelica::Ref::new(Values::Value::CODE { A: metamodelica::Ref::new(Absyn::CodeNode::C_TYPENAME { path: Parser::stringPath(annotationName)? }) })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::BOOL { boolean: __pa0 }) => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    res = metamodelica::Own::own(__pa0);
    Ok(res)
}

pub fn classAnnotationExists(mut className: ArcStr, mut annotationName: ArcStr) -> Result<bool> {
    let mut res: bool;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), literal!("classAnnotationExists"), list![metamodelica::Ref::new(Values::Value::CODE { A: metamodelica::Ref::new(Absyn::CodeNode::C_TYPENAME { path: Parser::stringPath(className)? }) }), metamodelica::Ref::new(Values::Value::CODE { A: metamodelica::Ref::new(Absyn::CodeNode::C_TYPENAME { path: Parser::stringPath(annotationName)? }) })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::BOOL { boolean: __pa0 }) => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    res = metamodelica::Own::own(__pa0);
    Ok(res)
}

pub fn getAnnotationModifierValue(
    mut className: ArcStr,
    mut annotationName: ArcStr,
    mut modifierName: ArcStr,
) -> Result<ArcStr> {
    let mut res: ArcStr;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), literal!("getAnnotationModifierValue"), list![metamodelica::Ref::new(Values::Value::CODE { A: metamodelica::Ref::new(Absyn::CodeNode::C_TYPENAME { path: Parser::stringPath(className)? }) }), metamodelica::Ref::new(Values::Value::STRING { string: annotationName }), metamodelica::Ref::new(Values::Value::STRING { string: modifierName })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::STRING { string: __pa0 }) => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    res = metamodelica::Own::own(__pa0);
    Ok(res)
}

pub fn getAnnotationNamedModifiers(
    mut className: ArcStr,
    mut annotationName: ArcStr,
) -> Result<metamodelica::List<ArcStr>> {
    let mut res: metamodelica::List<ArcStr>;
    let mut res_arr: metamodelica::Ref<Values::Value>;
    (_, res_arr) = CevalScript::cevalInteractiveFunctions2(
        FCore::emptyCache(),
        FGraph::empty(),
        literal!("getAnnotationNamedModifiers"),
        list![
            metamodelica::Ref::new(Values::Value::CODE {
                A: metamodelica::Ref::new(Absyn::CodeNode::C_TYPENAME {
                    path: Parser::stringPath(className)?
                })
            }),
            metamodelica::Ref::new(Values::Value::STRING { string: annotationName })
        ],
        dummyMsg.clone(),
    )?;
    res = ({
        let mut __acc: metamodelica::List<ArcStr> = metamodelica::nil();
        for mut res_arr_iter in (ValuesUtil::arrayValues(&res_arr)?).into_iter().cloned() {
            let __x = (match &*res_arr_iter.clone() {
                Values::Value::STRING {
                    string: __res_arr_iter_string,
                } => __res_arr_iter_string.clone(),
                _ => return Err("match: no arm matched"),
            });
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    Ok(res)
}

pub fn getSimulationOptions(
    mut name: ArcStr,
    mut defaultStartTime: metamodelica::Real,
    mut defaultStopTime: metamodelica::Real,
    mut defaultTolerance: metamodelica::Real,
    mut defaultNumberOfIntervals: i32,
    mut defaultInterval: metamodelica::Real,
) -> Result<(
    metamodelica::Real,
    metamodelica::Real,
    metamodelica::Real,
    i32,
    metamodelica::Real,
)> {
    let mut res1: metamodelica::Real;
    let mut res2: metamodelica::Real;
    let mut res3: metamodelica::Real;
    let mut res4: i32;
    let mut res5: metamodelica::Real;
    let (__pa0, __pa1, __pa2, __pa3, __pa4) = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), literal!("getSimulationOptions"), list![metamodelica::Ref::new(Values::Value::CODE { A: metamodelica::Ref::new(Absyn::CodeNode::C_TYPENAME { path: Parser::stringPath(name)? }) }), metamodelica::Ref::new(Values::Value::REAL { real: defaultStartTime }), metamodelica::Ref::new(Values::Value::REAL { real: defaultStopTime }), metamodelica::Ref::new(Values::Value::REAL { real: defaultTolerance }), metamodelica::Ref::new(Values::Value::INTEGER { integer: defaultNumberOfIntervals }), metamodelica::Ref::new(Values::Value::REAL { real: defaultInterval })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::TUPLE { valueLst: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::REAL { real: __pa0 }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::REAL { real: __pa1 }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::REAL { real: __pa2 }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::INTEGER { integer: __pa3 }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::REAL { real: __pa4 }, tail: Deref @ metamodelica::ListNode::Nil } } } } } }) => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone(), __pa4.clone()),
        _ => return Err("pattern mismatch"),
    } };
    res1 = metamodelica::Own::own(__pa0);
    res2 = metamodelica::Own::own(__pa1);
    res3 = metamodelica::Own::own(__pa2);
    res4 = metamodelica::Own::own(__pa3);
    res5 = metamodelica::Own::own(__pa4);
    Ok((res1, res2, res3, res4, res5))
}

pub fn isExperiment(mut name: ArcStr) -> Result<bool> {
    let mut res: bool;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), literal!("isExperiment"), list![metamodelica::Ref::new(Values::Value::CODE { A: metamodelica::Ref::new(Absyn::CodeNode::C_TYPENAME { path: Parser::stringPath(name)? }) })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::BOOL { boolean: __pa0 }) => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    res = metamodelica::Own::own(__pa0);
    Ok(res)
}

pub fn getNthInheritedClass(mut className: ArcStr, mut n: i32) -> Result<ArcStr> {
    let mut res: ArcStr;
    let mut res_path: metamodelica::Ref<Absyn::Path>;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), literal!("getNthInheritedClass"), list![metamodelica::Ref::new(Values::Value::CODE { A: metamodelica::Ref::new(Absyn::CodeNode::C_TYPENAME { path: Parser::stringPath(className)? }) }), metamodelica::Ref::new(Values::Value::INTEGER { integer: n })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: __pa0 } }) => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    res_path = metamodelica::Own::own(__pa0);
    res = AbsynUtil::pathString(res_path, literal!("."), true, false)?;
    Ok(res)
}

pub fn getInheritedClasses(mut name: ArcStr) -> Result<metamodelica::List<ArcStr>> {
    let mut res: metamodelica::List<ArcStr>;
    let mut res_arr: metamodelica::Ref<Values::Value>;
    (_, res_arr) = CevalScript::cevalInteractiveFunctions2(
        FCore::emptyCache(),
        FGraph::empty(),
        literal!("getInheritedClasses"),
        list![metamodelica::Ref::new(Values::Value::CODE {
            A: metamodelica::Ref::new(Absyn::CodeNode::C_TYPENAME {
                path: Parser::stringPath(name)?
            })
        })],
        dummyMsg.clone(),
    )?;
    res = ({
        let mut __acc: metamodelica::List<ArcStr> = metamodelica::nil();
        for mut res_arr_iter in (ValuesUtil::arrayValues(&res_arr)?).into_iter().cloned() {
            let __x = ValuesDump::valString(&(res_arr_iter.clone()))?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    Ok(res)
}

pub fn getInheritanceCount(mut className: ArcStr) -> Result<i32> {
    let mut res: i32;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), literal!("getInheritanceCount"), list![metamodelica::Ref::new(Values::Value::CODE { A: metamodelica::Ref::new(Absyn::CodeNode::C_TYPENAME { path: Parser::stringPath(className)? }) })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::INTEGER { integer: __pa0 }) => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    res = metamodelica::Own::own(__pa0);
    Ok(res)
}

pub fn isProtected(mut componentName: ArcStr, mut className: ArcStr) -> Result<bool> {
    let mut res: bool;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), literal!("isProtected"), list![metamodelica::Ref::new(Values::Value::CODE { A: metamodelica::Ref::new(Absyn::CodeNode::C_TYPENAME { path: Parser::stringPath(componentName)? }) }), metamodelica::Ref::new(Values::Value::CODE { A: metamodelica::Ref::new(Absyn::CodeNode::C_TYPENAME { path: Parser::stringPath(className)? }) })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::BOOL { boolean: __pa0 }) => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    res = metamodelica::Own::own(__pa0);
    Ok(res)
}

pub fn isConstant(mut componentName: ArcStr, mut className: ArcStr) -> Result<bool> {
    let mut res: bool;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), literal!("isConstant"), list![metamodelica::Ref::new(Values::Value::CODE { A: metamodelica::Ref::new(Absyn::CodeNode::C_TYPENAME { path: Parser::stringPath(componentName)? }) }), metamodelica::Ref::new(Values::Value::CODE { A: metamodelica::Ref::new(Absyn::CodeNode::C_TYPENAME { path: Parser::stringPath(className)? }) })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::BOOL { boolean: __pa0 }) => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    res = metamodelica::Own::own(__pa0);
    Ok(res)
}

pub fn isParameter(mut componentName: ArcStr, mut className: ArcStr) -> Result<bool> {
    let mut res: bool;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), literal!("isParameter"), list![metamodelica::Ref::new(Values::Value::CODE { A: metamodelica::Ref::new(Absyn::CodeNode::C_TYPENAME { path: Parser::stringPath(componentName)? }) }), metamodelica::Ref::new(Values::Value::CODE { A: metamodelica::Ref::new(Absyn::CodeNode::C_TYPENAME { path: Parser::stringPath(className)? }) })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::BOOL { boolean: __pa0 }) => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    res = metamodelica::Own::own(__pa0);
    Ok(res)
}

pub fn isPrimitive(mut className: ArcStr) -> Result<bool> {
    let mut res: bool;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), literal!("isPrimitive"), list![metamodelica::Ref::new(Values::Value::CODE { A: metamodelica::Ref::new(Absyn::CodeNode::C_TYPENAME { path: Parser::stringPath(className)? }) })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::BOOL { boolean: __pa0 }) => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    res = metamodelica::Own::own(__pa0);
    Ok(res)
}

pub fn getBuiltinType(mut cl: ArcStr) -> Result<ArcStr> {
    let mut res: ArcStr;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), literal!("getBuiltinType"), list![metamodelica::Ref::new(Values::Value::CODE { A: metamodelica::Ref::new(Absyn::CodeNode::C_TYPENAME { path: Parser::stringPath(cl)? }) })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::STRING { string: __pa0 }) => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    res = metamodelica::Own::own(__pa0);
    Ok(res)
}

pub fn isProtectedClass(mut cl: ArcStr, mut c2: ArcStr) -> Result<bool> {
    let mut res: bool;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), literal!("isProtectedClass"), list![metamodelica::Ref::new(Values::Value::CODE { A: metamodelica::Ref::new(Absyn::CodeNode::C_TYPENAME { path: Parser::stringPath(cl)? }) }), metamodelica::Ref::new(Values::Value::STRING { string: c2 })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::BOOL { boolean: __pa0 }) => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    res = metamodelica::Own::own(__pa0);
    Ok(res)
}

pub fn isOperatorFunction(mut cl: ArcStr) -> Result<bool> {
    let mut res: bool;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), literal!("isOperatorFunction"), list![metamodelica::Ref::new(Values::Value::CODE { A: metamodelica::Ref::new(Absyn::CodeNode::C_TYPENAME { path: Parser::stringPath(cl)? }) })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::BOOL { boolean: __pa0 }) => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    res = metamodelica::Own::own(__pa0);
    Ok(res)
}

pub fn isOperatorRecord(mut cl: ArcStr) -> Result<bool> {
    let mut res: bool;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), literal!("isOperatorRecord"), list![metamodelica::Ref::new(Values::Value::CODE { A: metamodelica::Ref::new(Absyn::CodeNode::C_TYPENAME { path: Parser::stringPath(cl)? }) })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::BOOL { boolean: __pa0 }) => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    res = metamodelica::Own::own(__pa0);
    Ok(res)
}

pub fn isOperator(mut cl: ArcStr) -> Result<bool> {
    let mut res: bool;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), literal!("isOperator"), list![metamodelica::Ref::new(Values::Value::CODE { A: metamodelica::Ref::new(Absyn::CodeNode::C_TYPENAME { path: Parser::stringPath(cl)? }) })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::BOOL { boolean: __pa0 }) => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    res = metamodelica::Own::own(__pa0);
    Ok(res)
}

pub fn isEnumeration(mut cl: ArcStr) -> Result<bool> {
    let mut res: bool;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), literal!("isEnumeration"), list![metamodelica::Ref::new(Values::Value::CODE { A: metamodelica::Ref::new(Absyn::CodeNode::C_TYPENAME { path: Parser::stringPath(cl)? }) })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::BOOL { boolean: __pa0 }) => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    res = metamodelica::Own::own(__pa0);
    Ok(res)
}

pub fn isOptimization(mut cl: ArcStr) -> Result<bool> {
    let mut res: bool;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), literal!("isOptimization"), list![metamodelica::Ref::new(Values::Value::CODE { A: metamodelica::Ref::new(Absyn::CodeNode::C_TYPENAME { path: Parser::stringPath(cl)? }) })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::BOOL { boolean: __pa0 }) => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    res = metamodelica::Own::own(__pa0);
    Ok(res)
}

pub fn isConnector(mut cl: ArcStr) -> Result<bool> {
    let mut res: bool;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), literal!("isConnector"), list![metamodelica::Ref::new(Values::Value::CODE { A: metamodelica::Ref::new(Absyn::CodeNode::C_TYPENAME { path: Parser::stringPath(cl)? }) })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::BOOL { boolean: __pa0 }) => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    res = metamodelica::Own::own(__pa0);
    Ok(res)
}

pub fn isModel(mut cl: ArcStr) -> Result<bool> {
    let mut res: bool;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), literal!("isModel"), list![metamodelica::Ref::new(Values::Value::CODE { A: metamodelica::Ref::new(Absyn::CodeNode::C_TYPENAME { path: Parser::stringPath(cl)? }) })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::BOOL { boolean: __pa0 }) => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    res = metamodelica::Own::own(__pa0);
    Ok(res)
}

pub fn isRedeclare(mut element: ArcStr) -> Result<bool> {
    let mut res: bool;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), literal!("isRedeclare"), list![metamodelica::Ref::new(Values::Value::CODE { A: metamodelica::Ref::new(Absyn::CodeNode::C_TYPENAME { path: Parser::stringPath(element)? }) })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::BOOL { boolean: __pa0 }) => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    res = metamodelica::Own::own(__pa0);
    Ok(res)
}

pub fn isReplaceable(mut element: ArcStr) -> Result<bool> {
    let mut res: bool;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), literal!("isReplaceable"), list![metamodelica::Ref::new(Values::Value::CODE { A: metamodelica::Ref::new(Absyn::CodeNode::C_TYPENAME { path: Parser::stringPath(element)? }) })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::BOOL { boolean: __pa0 }) => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    res = metamodelica::Own::own(__pa0);
    Ok(res)
}

pub fn isPartial(mut cl: ArcStr) -> Result<bool> {
    let mut res: bool;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), literal!("isPartial"), list![metamodelica::Ref::new(Values::Value::CODE { A: metamodelica::Ref::new(Absyn::CodeNode::C_TYPENAME { path: Parser::stringPath(cl)? }) })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::BOOL { boolean: __pa0 }) => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    res = metamodelica::Own::own(__pa0);
    Ok(res)
}

pub fn isFunction(mut cl: ArcStr) -> Result<bool> {
    let mut res: bool;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), literal!("isFunction"), list![metamodelica::Ref::new(Values::Value::CODE { A: metamodelica::Ref::new(Absyn::CodeNode::C_TYPENAME { path: Parser::stringPath(cl)? }) })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::BOOL { boolean: __pa0 }) => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    res = metamodelica::Own::own(__pa0);
    Ok(res)
}

pub fn isBlock(mut cl: ArcStr) -> Result<bool> {
    let mut res: bool;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), literal!("isBlock"), list![metamodelica::Ref::new(Values::Value::CODE { A: metamodelica::Ref::new(Absyn::CodeNode::C_TYPENAME { path: Parser::stringPath(cl)? }) })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::BOOL { boolean: __pa0 }) => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    res = metamodelica::Own::own(__pa0);
    Ok(res)
}

pub fn isRecord(mut cl: ArcStr) -> Result<bool> {
    let mut res: bool;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), literal!("isRecord"), list![metamodelica::Ref::new(Values::Value::CODE { A: metamodelica::Ref::new(Absyn::CodeNode::C_TYPENAME { path: Parser::stringPath(cl)? }) })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::BOOL { boolean: __pa0 }) => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    res = metamodelica::Own::own(__pa0);
    Ok(res)
}

pub fn isClass(mut cl: ArcStr) -> Result<bool> {
    let mut res: bool;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), literal!("isClass"), list![metamodelica::Ref::new(Values::Value::CODE { A: metamodelica::Ref::new(Absyn::CodeNode::C_TYPENAME { path: Parser::stringPath(cl)? }) })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::BOOL { boolean: __pa0 }) => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    res = metamodelica::Own::own(__pa0);
    Ok(res)
}

pub fn isPackage(mut cl: ArcStr) -> Result<bool> {
    let mut res: bool;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), literal!("isPackage"), list![metamodelica::Ref::new(Values::Value::CODE { A: metamodelica::Ref::new(Absyn::CodeNode::C_TYPENAME { path: Parser::stringPath(cl)? }) })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::BOOL { boolean: __pa0 }) => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    res = metamodelica::Own::own(__pa0);
    Ok(res)
}

pub fn isType(mut cl: ArcStr) -> Result<bool> {
    let mut res: bool;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), literal!("isType"), list![metamodelica::Ref::new(Values::Value::CODE { A: metamodelica::Ref::new(Absyn::CodeNode::C_TYPENAME { path: Parser::stringPath(cl)? }) })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::BOOL { boolean: __pa0 }) => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    res = metamodelica::Own::own(__pa0);
    Ok(res)
}

pub fn getClassRestriction(mut cl: ArcStr) -> Result<ArcStr> {
    let mut res: ArcStr;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), literal!("getClassRestriction"), list![metamodelica::Ref::new(Values::Value::CODE { A: metamodelica::Ref::new(Absyn::CodeNode::C_TYPENAME { path: Parser::stringPath(cl)? }) })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::STRING { string: __pa0 }) => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    res = metamodelica::Own::own(__pa0);
    Ok(res)
}

pub fn existClass(mut cl: ArcStr) -> Result<bool> {
    let mut res: bool;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), literal!("existClass"), list![metamodelica::Ref::new(Values::Value::CODE { A: metamodelica::Ref::new(Absyn::CodeNode::C_TYPENAME { path: Parser::stringPath(cl)? }) })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::BOOL { boolean: __pa0 }) => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    res = metamodelica::Own::own(__pa0);
    Ok(res)
}

pub fn basename(mut path: ArcStr) -> Result<ArcStr> {
    let mut res: ArcStr;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), literal!("basename"), list![metamodelica::Ref::new(Values::Value::STRING { string: path })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::STRING { string: __pa0 }) => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    res = metamodelica::Own::own(__pa0);
    Ok(res)
}

pub fn dirname(mut path: ArcStr) -> Result<ArcStr> {
    let mut res: ArcStr;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), literal!("dirname"), list![metamodelica::Ref::new(Values::Value::STRING { string: path })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::STRING { string: __pa0 }) => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    res = metamodelica::Own::own(__pa0);
    Ok(res)
}

pub fn getClassComment(mut cl: ArcStr) -> Result<ArcStr> {
    let mut res: ArcStr;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), literal!("getClassComment"), list![metamodelica::Ref::new(Values::Value::CODE { A: metamodelica::Ref::new(Absyn::CodeNode::C_TYPENAME { path: Parser::stringPath(cl)? }) })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::STRING { string: __pa0 }) => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    res = metamodelica::Own::own(__pa0);
    Ok(res)
}

pub fn typeNameStrings(mut cl: ArcStr) -> Result<metamodelica::List<ArcStr>> {
    let mut res: metamodelica::List<ArcStr>;
    let mut res_arr: metamodelica::Ref<Values::Value>;
    (_, res_arr) = CevalScript::cevalInteractiveFunctions2(
        FCore::emptyCache(),
        FGraph::empty(),
        literal!("typeNameStrings"),
        list![metamodelica::Ref::new(Values::Value::CODE {
            A: metamodelica::Ref::new(Absyn::CodeNode::C_TYPENAME {
                path: Parser::stringPath(cl)?
            })
        })],
        dummyMsg.clone(),
    )?;
    res = ({
        let mut __acc: metamodelica::List<ArcStr> = metamodelica::nil();
        for mut res_arr_iter in (ValuesUtil::arrayValues(&res_arr)?).into_iter().cloned() {
            let __x = (match &*res_arr_iter.clone() {
                Values::Value::STRING {
                    string: __res_arr_iter_string,
                } => __res_arr_iter_string.clone(),
                _ => return Err("match: no arm matched"),
            });
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    Ok(res)
}

pub fn typeNameString(mut cl: ArcStr) -> Result<ArcStr> {
    let mut res: ArcStr;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), literal!("typeNameString"), list![metamodelica::Ref::new(Values::Value::CODE { A: metamodelica::Ref::new(Absyn::CodeNode::C_TYPENAME { path: Parser::stringPath(cl)? }) })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::STRING { string: __pa0 }) => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    res = metamodelica::Own::own(__pa0);
    Ok(res)
}

pub fn stringTypeName(mut r#str: ArcStr) -> Result<ArcStr> {
    let mut res: ArcStr;
    let mut res_path: metamodelica::Ref<Absyn::Path>;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), literal!("stringTypeName"), list![metamodelica::Ref::new(Values::Value::STRING { string: r#str })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::CODE { A: Deref @ Absyn::CodeNode::C_TYPENAME { path: __pa0 } }) => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    res_path = metamodelica::Own::own(__pa0);
    res = AbsynUtil::pathString(res_path, literal!("."), true, false)?;
    Ok(res)
}

pub fn getTimeStamp(mut cl: ArcStr) -> Result<(metamodelica::Real, ArcStr)> {
    let mut res1: metamodelica::Real;
    let mut res2: ArcStr;
    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), literal!("getTimeStamp"), list![metamodelica::Ref::new(Values::Value::CODE { A: metamodelica::Ref::new(Absyn::CodeNode::C_TYPENAME { path: Parser::stringPath(cl)? }) })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::TUPLE { valueLst: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::REAL { real: __pa0 }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: __pa1 }, tail: Deref @ metamodelica::ListNode::Nil } } }) => (__pa0.clone(), __pa1.clone()),
        _ => return Err("pattern mismatch"),
    } };
    res1 = metamodelica::Own::own(__pa0);
    res2 = metamodelica::Own::own(__pa1);
    Ok((res1, res2))
}

pub fn setDocumentationAnnotation(mut class_: ArcStr, mut info: ArcStr, mut revisions: ArcStr) -> Result<bool> {
    let mut res: bool;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), literal!("setDocumentationAnnotation"), list![metamodelica::Ref::new(Values::Value::CODE { A: metamodelica::Ref::new(Absyn::CodeNode::C_TYPENAME { path: Parser::stringPath(class_)? }) }), metamodelica::Ref::new(Values::Value::STRING { string: info }), metamodelica::Ref::new(Values::Value::STRING { string: revisions })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::BOOL { boolean: __pa0 }) => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    res = metamodelica::Own::own(__pa0);
    Ok(res)
}

pub fn getDocumentationAnnotation(mut cl: ArcStr) -> Result<metamodelica::List<ArcStr>> {
    let mut res: metamodelica::List<ArcStr>;
    let mut res_arr: metamodelica::Ref<Values::Value>;
    (_, res_arr) = CevalScript::cevalInteractiveFunctions2(
        FCore::emptyCache(),
        FGraph::empty(),
        literal!("getDocumentationAnnotation"),
        list![metamodelica::Ref::new(Values::Value::CODE {
            A: metamodelica::Ref::new(Absyn::CodeNode::C_TYPENAME {
                path: Parser::stringPath(cl)?
            })
        })],
        dummyMsg.clone(),
    )?;
    res = ({
        let mut __acc: metamodelica::List<ArcStr> = metamodelica::nil();
        for mut res_arr_iter in (ValuesUtil::arrayValues(&res_arr)?).into_iter().cloned() {
            let __x = (match &*res_arr_iter.clone() {
                Values::Value::STRING {
                    string: __res_arr_iter_string,
                } => __res_arr_iter_string.clone(),
                _ => return Err("match: no arm matched"),
            });
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    Ok(res)
}

pub fn iconv(mut string: ArcStr, mut from: ArcStr, mut to: ArcStr) -> Result<ArcStr> {
    let mut res: ArcStr;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), literal!("iconv"), list![metamodelica::Ref::new(Values::Value::STRING { string: string }), metamodelica::Ref::new(Values::Value::STRING { string: from }), metamodelica::Ref::new(Values::Value::STRING { string: to })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::STRING { string: __pa0 }) => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    res = metamodelica::Own::own(__pa0);
    Ok(res)
}

pub fn getNthImport(mut class_: ArcStr, mut index: i32) -> Result<metamodelica::List<ArcStr>> {
    let mut res: metamodelica::List<ArcStr>;
    let mut res_arr: metamodelica::Ref<Values::Value>;
    (_, res_arr) = CevalScript::cevalInteractiveFunctions2(
        FCore::emptyCache(),
        FGraph::empty(),
        literal!("getNthImport"),
        list![
            metamodelica::Ref::new(Values::Value::CODE {
                A: metamodelica::Ref::new(Absyn::CodeNode::C_TYPENAME {
                    path: Parser::stringPath(class_)?
                })
            }),
            metamodelica::Ref::new(Values::Value::INTEGER { integer: index })
        ],
        dummyMsg.clone(),
    )?;
    res = ({
        let mut __acc: metamodelica::List<ArcStr> = metamodelica::nil();
        for mut res_arr_iter in (ValuesUtil::arrayValues(&res_arr)?).into_iter().cloned() {
            let __x = (match &*res_arr_iter.clone() {
                Values::Value::STRING {
                    string: __res_arr_iter_string,
                } => __res_arr_iter_string.clone(),
                _ => return Err("match: no arm matched"),
            });
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    Ok(res)
}

pub fn getImportedNames(mut class_: ArcStr) -> Result<(metamodelica::List<ArcStr>, metamodelica::List<ArcStr>)> {
    let mut res1: metamodelica::List<ArcStr>;
    let mut res2: metamodelica::List<ArcStr>;
    let mut res1_arr: metamodelica::Ref<Values::Value>;
    let mut res2_arr: metamodelica::Ref<Values::Value>;
    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), literal!("getImportedNames"), list![metamodelica::Ref::new(Values::Value::CODE { A: metamodelica::Ref::new(Absyn::CodeNode::C_TYPENAME { path: Parser::stringPath(class_)? }) })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::TUPLE { valueLst: Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: Deref @ metamodelica::ListNode::Cons { head: __pa1, tail: Deref @ metamodelica::ListNode::Nil } } }) => (__pa0.clone(), __pa1.clone()),
        _ => return Err("pattern mismatch"),
    } };
    res1_arr = metamodelica::Own::own(__pa0);
    res2_arr = metamodelica::Own::own(__pa1);
    res1 = ({
        let mut __acc: metamodelica::List<ArcStr> = metamodelica::nil();
        for mut res1_arr_iter in (ValuesUtil::arrayValues(&res1_arr)?).into_iter().cloned() {
            let __x = (match &*res1_arr_iter.clone() {
                Values::Value::STRING {
                    string: __res1_arr_iter_string,
                } => __res1_arr_iter_string.clone(),
                _ => return Err("match: no arm matched"),
            });
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    res2 = ({
        let mut __acc: metamodelica::List<ArcStr> = metamodelica::nil();
        for mut res2_arr_iter in (ValuesUtil::arrayValues(&res2_arr)?).into_iter().cloned() {
            let __x = (match &*res2_arr_iter.clone() {
                Values::Value::STRING {
                    string: __res2_arr_iter_string,
                } => __res2_arr_iter_string.clone(),
                _ => return Err("match: no arm matched"),
            });
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    Ok((res1, res2))
}

pub fn getMMfileTotalDependencies(
    mut in_package_name: ArcStr,
    mut public_imports_dir: ArcStr,
) -> Result<metamodelica::List<ArcStr>> {
    let mut res: metamodelica::List<ArcStr>;
    let mut res_arr: metamodelica::Ref<Values::Value>;
    (_, res_arr) = CevalScript::cevalInteractiveFunctions2(
        FCore::emptyCache(),
        FGraph::empty(),
        literal!("getMMfileTotalDependencies"),
        list![
            metamodelica::Ref::new(Values::Value::STRING {
                string: in_package_name
            }),
            metamodelica::Ref::new(Values::Value::STRING {
                string: public_imports_dir
            })
        ],
        dummyMsg.clone(),
    )?;
    res = ({
        let mut __acc: metamodelica::List<ArcStr> = metamodelica::nil();
        for mut res_arr_iter in (ValuesUtil::arrayValues(&res_arr)?).into_iter().cloned() {
            let __x = (match &*res_arr_iter.clone() {
                Values::Value::STRING {
                    string: __res_arr_iter_string,
                } => __res_arr_iter_string.clone(),
                _ => return Err("match: no arm matched"),
            });
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    Ok(res)
}

pub fn getImportCount(mut class_: ArcStr) -> Result<i32> {
    let mut res: i32;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), literal!("getImportCount"), list![metamodelica::Ref::new(Values::Value::CODE { A: metamodelica::Ref::new(Absyn::CodeNode::C_TYPENAME { path: Parser::stringPath(class_)? }) })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::INTEGER { integer: __pa0 }) => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    res = metamodelica::Own::own(__pa0);
    Ok(res)
}

pub fn getNthAnnotationString(mut class_: ArcStr, mut index: i32) -> Result<ArcStr> {
    let mut res: ArcStr;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), literal!("getNthAnnotationString"), list![metamodelica::Ref::new(Values::Value::CODE { A: metamodelica::Ref::new(Absyn::CodeNode::C_TYPENAME { path: Parser::stringPath(class_)? }) }), metamodelica::Ref::new(Values::Value::INTEGER { integer: index })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::STRING { string: __pa0 }) => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    res = metamodelica::Own::own(__pa0);
    Ok(res)
}

pub fn getAnnotationCount(mut class_: ArcStr) -> Result<i32> {
    let mut res: i32;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), literal!("getAnnotationCount"), list![metamodelica::Ref::new(Values::Value::CODE { A: metamodelica::Ref::new(Absyn::CodeNode::C_TYPENAME { path: Parser::stringPath(class_)? }) })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::INTEGER { integer: __pa0 }) => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    res = metamodelica::Own::own(__pa0);
    Ok(res)
}

pub fn getNthInitialEquationItem(mut class_: ArcStr, mut index: i32) -> Result<ArcStr> {
    let mut res: ArcStr;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), literal!("getNthInitialEquationItem"), list![metamodelica::Ref::new(Values::Value::CODE { A: metamodelica::Ref::new(Absyn::CodeNode::C_TYPENAME { path: Parser::stringPath(class_)? }) }), metamodelica::Ref::new(Values::Value::INTEGER { integer: index })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::STRING { string: __pa0 }) => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    res = metamodelica::Own::own(__pa0);
    Ok(res)
}

pub fn getInitialEquationItemsCount(mut class_: ArcStr) -> Result<i32> {
    let mut res: i32;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), literal!("getInitialEquationItemsCount"), list![metamodelica::Ref::new(Values::Value::CODE { A: metamodelica::Ref::new(Absyn::CodeNode::C_TYPENAME { path: Parser::stringPath(class_)? }) })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::INTEGER { integer: __pa0 }) => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    res = metamodelica::Own::own(__pa0);
    Ok(res)
}

pub fn getNthEquationItem(mut class_: ArcStr, mut index: i32) -> Result<ArcStr> {
    let mut res: ArcStr;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), literal!("getNthEquationItem"), list![metamodelica::Ref::new(Values::Value::CODE { A: metamodelica::Ref::new(Absyn::CodeNode::C_TYPENAME { path: Parser::stringPath(class_)? }) }), metamodelica::Ref::new(Values::Value::INTEGER { integer: index })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::STRING { string: __pa0 }) => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    res = metamodelica::Own::own(__pa0);
    Ok(res)
}

pub fn getEquationItemsCount(mut class_: ArcStr) -> Result<i32> {
    let mut res: i32;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), literal!("getEquationItemsCount"), list![metamodelica::Ref::new(Values::Value::CODE { A: metamodelica::Ref::new(Absyn::CodeNode::C_TYPENAME { path: Parser::stringPath(class_)? }) })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::INTEGER { integer: __pa0 }) => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    res = metamodelica::Own::own(__pa0);
    Ok(res)
}

pub fn getNthInitialEquation(mut class_: ArcStr, mut index: i32) -> Result<ArcStr> {
    let mut res: ArcStr;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), literal!("getNthInitialEquation"), list![metamodelica::Ref::new(Values::Value::CODE { A: metamodelica::Ref::new(Absyn::CodeNode::C_TYPENAME { path: Parser::stringPath(class_)? }) }), metamodelica::Ref::new(Values::Value::INTEGER { integer: index })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::STRING { string: __pa0 }) => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    res = metamodelica::Own::own(__pa0);
    Ok(res)
}

pub fn getInitialEquationCount(mut class_: ArcStr) -> Result<i32> {
    let mut res: i32;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), literal!("getInitialEquationCount"), list![metamodelica::Ref::new(Values::Value::CODE { A: metamodelica::Ref::new(Absyn::CodeNode::C_TYPENAME { path: Parser::stringPath(class_)? }) })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::INTEGER { integer: __pa0 }) => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    res = metamodelica::Own::own(__pa0);
    Ok(res)
}

pub fn getNthEquation(mut class_: ArcStr, mut index: i32) -> Result<ArcStr> {
    let mut res: ArcStr;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), literal!("getNthEquation"), list![metamodelica::Ref::new(Values::Value::CODE { A: metamodelica::Ref::new(Absyn::CodeNode::C_TYPENAME { path: Parser::stringPath(class_)? }) }), metamodelica::Ref::new(Values::Value::INTEGER { integer: index })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::STRING { string: __pa0 }) => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    res = metamodelica::Own::own(__pa0);
    Ok(res)
}

pub fn getEquationCount(mut class_: ArcStr) -> Result<i32> {
    let mut res: i32;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), literal!("getEquationCount"), list![metamodelica::Ref::new(Values::Value::CODE { A: metamodelica::Ref::new(Absyn::CodeNode::C_TYPENAME { path: Parser::stringPath(class_)? }) })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::INTEGER { integer: __pa0 }) => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    res = metamodelica::Own::own(__pa0);
    Ok(res)
}

pub fn getNthInitialAlgorithmItem(mut class_: ArcStr, mut index: i32) -> Result<ArcStr> {
    let mut res: ArcStr;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), literal!("getNthInitialAlgorithmItem"), list![metamodelica::Ref::new(Values::Value::CODE { A: metamodelica::Ref::new(Absyn::CodeNode::C_TYPENAME { path: Parser::stringPath(class_)? }) }), metamodelica::Ref::new(Values::Value::INTEGER { integer: index })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::STRING { string: __pa0 }) => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    res = metamodelica::Own::own(__pa0);
    Ok(res)
}

pub fn getInitialAlgorithmItemsCount(mut class_: ArcStr) -> Result<i32> {
    let mut res: i32;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), literal!("getInitialAlgorithmItemsCount"), list![metamodelica::Ref::new(Values::Value::CODE { A: metamodelica::Ref::new(Absyn::CodeNode::C_TYPENAME { path: Parser::stringPath(class_)? }) })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::INTEGER { integer: __pa0 }) => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    res = metamodelica::Own::own(__pa0);
    Ok(res)
}

pub fn getNthAlgorithmItem(mut class_: ArcStr, mut index: i32) -> Result<ArcStr> {
    let mut res: ArcStr;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), literal!("getNthAlgorithmItem"), list![metamodelica::Ref::new(Values::Value::CODE { A: metamodelica::Ref::new(Absyn::CodeNode::C_TYPENAME { path: Parser::stringPath(class_)? }) }), metamodelica::Ref::new(Values::Value::INTEGER { integer: index })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::STRING { string: __pa0 }) => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    res = metamodelica::Own::own(__pa0);
    Ok(res)
}

pub fn getAlgorithmItemsCount(mut class_: ArcStr) -> Result<i32> {
    let mut res: i32;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), literal!("getAlgorithmItemsCount"), list![metamodelica::Ref::new(Values::Value::CODE { A: metamodelica::Ref::new(Absyn::CodeNode::C_TYPENAME { path: Parser::stringPath(class_)? }) })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::INTEGER { integer: __pa0 }) => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    res = metamodelica::Own::own(__pa0);
    Ok(res)
}

pub fn getNthInitialAlgorithm(mut class_: ArcStr, mut index: i32) -> Result<ArcStr> {
    let mut res: ArcStr;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), literal!("getNthInitialAlgorithm"), list![metamodelica::Ref::new(Values::Value::CODE { A: metamodelica::Ref::new(Absyn::CodeNode::C_TYPENAME { path: Parser::stringPath(class_)? }) }), metamodelica::Ref::new(Values::Value::INTEGER { integer: index })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::STRING { string: __pa0 }) => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    res = metamodelica::Own::own(__pa0);
    Ok(res)
}

pub fn getInitialAlgorithmCount(mut class_: ArcStr) -> Result<i32> {
    let mut res: i32;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), literal!("getInitialAlgorithmCount"), list![metamodelica::Ref::new(Values::Value::CODE { A: metamodelica::Ref::new(Absyn::CodeNode::C_TYPENAME { path: Parser::stringPath(class_)? }) })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::INTEGER { integer: __pa0 }) => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    res = metamodelica::Own::own(__pa0);
    Ok(res)
}

pub fn getNthAlgorithm(mut class_: ArcStr, mut index: i32) -> Result<ArcStr> {
    let mut res: ArcStr;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), literal!("getNthAlgorithm"), list![metamodelica::Ref::new(Values::Value::CODE { A: metamodelica::Ref::new(Absyn::CodeNode::C_TYPENAME { path: Parser::stringPath(class_)? }) }), metamodelica::Ref::new(Values::Value::INTEGER { integer: index })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::STRING { string: __pa0 }) => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    res = metamodelica::Own::own(__pa0);
    Ok(res)
}

pub fn getAlgorithmCount(mut class_: ArcStr) -> Result<i32> {
    let mut res: i32;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), literal!("getAlgorithmCount"), list![metamodelica::Ref::new(Values::Value::CODE { A: metamodelica::Ref::new(Absyn::CodeNode::C_TYPENAME { path: Parser::stringPath(class_)? }) })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::INTEGER { integer: __pa0 }) => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    res = metamodelica::Own::own(__pa0);
    Ok(res)
}

pub fn updateEquation(
    mut className: ArcStr,
    mut oldEq: ArcStr,
    mut newEq: ArcStr,
    mut matchAll: bool,
    mut matchShallow: bool,
    mut matchDescription: bool,
    mut mergeDescription: bool,
) -> Result<bool> {
    let mut res: bool;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), literal!("updateEquation"), list![metamodelica::Ref::new(Values::Value::CODE { A: metamodelica::Ref::new(Absyn::CodeNode::C_TYPENAME { path: Parser::stringPath(className)? }) }), metamodelica::Ref::new(Values::Value::STRING { string: oldEq }), metamodelica::Ref::new(Values::Value::STRING { string: newEq }), metamodelica::Ref::new(Values::Value::BOOL { boolean: matchAll }), metamodelica::Ref::new(Values::Value::BOOL { boolean: matchShallow }), metamodelica::Ref::new(Values::Value::BOOL { boolean: matchDescription }), metamodelica::Ref::new(Values::Value::BOOL { boolean: mergeDescription })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::BOOL { boolean: __pa0 }) => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    res = metamodelica::Own::own(__pa0);
    Ok(res)
}

pub fn addEquation(mut className: ArcStr, mut eq: ArcStr, mut isInitial: bool) -> Result<bool> {
    let mut res: bool;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), literal!("addEquation"), list![metamodelica::Ref::new(Values::Value::CODE { A: metamodelica::Ref::new(Absyn::CodeNode::C_TYPENAME { path: Parser::stringPath(className)? }) }), metamodelica::Ref::new(Values::Value::STRING { string: eq }), metamodelica::Ref::new(Values::Value::BOOL { boolean: isInitial })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::BOOL { boolean: __pa0 }) => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    res = metamodelica::Own::own(__pa0);
    Ok(res)
}

pub fn getConnectionList(mut className: ArcStr) -> Result<metamodelica::List<metamodelica::List<ArcStr>>> {
    let mut res: metamodelica::List<metamodelica::List<ArcStr>>;
    let mut res_arr: metamodelica::Ref<Values::Value>;
    (_, res_arr) = CevalScript::cevalInteractiveFunctions2(
        FCore::emptyCache(),
        FGraph::empty(),
        literal!("getConnectionList"),
        list![metamodelica::Ref::new(Values::Value::CODE {
            A: metamodelica::Ref::new(Absyn::CodeNode::C_TYPENAME {
                path: Parser::stringPath(className)?
            })
        })],
        dummyMsg.clone(),
    )?;
    res = ({
        let mut __acc: metamodelica::List<metamodelica::List<ArcStr>> = metamodelica::nil();
        for mut res_arr_iter in (ValuesUtil::arrayValues(&res_arr)?).into_iter().cloned() {
            let __x = ({
                let mut __acc: metamodelica::List<ArcStr> = metamodelica::nil();
                for mut res_arr_iter_iter in (ValuesUtil::arrayValues(&(res_arr_iter.clone()))?).into_iter().cloned() {
                    let __x = (match &*res_arr_iter_iter.clone() {
                        Values::Value::STRING {
                            string: __res_arr_iter_iter_string,
                        } => __res_arr_iter_iter_string.clone(),
                        _ => return Err("match: no arm matched"),
                    });
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            });
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    Ok(res)
}

pub fn getNthConnection(mut className: ArcStr, mut index: i32) -> Result<metamodelica::List<ArcStr>> {
    let mut res: metamodelica::List<ArcStr>;
    let mut res_arr: metamodelica::Ref<Values::Value>;
    (_, res_arr) = CevalScript::cevalInteractiveFunctions2(
        FCore::emptyCache(),
        FGraph::empty(),
        literal!("getNthConnection"),
        list![
            metamodelica::Ref::new(Values::Value::CODE {
                A: metamodelica::Ref::new(Absyn::CodeNode::C_TYPENAME {
                    path: Parser::stringPath(className)?
                })
            }),
            metamodelica::Ref::new(Values::Value::INTEGER { integer: index })
        ],
        dummyMsg.clone(),
    )?;
    res = ({
        let mut __acc: metamodelica::List<ArcStr> = metamodelica::nil();
        for mut res_arr_iter in (ValuesUtil::arrayValues(&res_arr)?).into_iter().cloned() {
            let __x = (match &*res_arr_iter.clone() {
                Values::Value::STRING {
                    string: __res_arr_iter_string,
                } => __res_arr_iter_string.clone(),
                _ => return Err("match: no arm matched"),
            });
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    Ok(res)
}

pub fn getConnectionCount(mut className: ArcStr) -> Result<i32> {
    let mut res: i32;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), literal!("getConnectionCount"), list![metamodelica::Ref::new(Values::Value::CODE { A: metamodelica::Ref::new(Absyn::CodeNode::C_TYPENAME { path: Parser::stringPath(className)? }) })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::INTEGER { integer: __pa0 }) => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    res = metamodelica::Own::own(__pa0);
    Ok(res)
}

pub fn updateConnectionNames(
    mut className: ArcStr,
    mut from: ArcStr,
    mut to: ArcStr,
    mut fromNew: ArcStr,
    mut toNew: ArcStr,
) -> Result<bool> {
    let mut res: bool;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), literal!("updateConnectionNames"), list![metamodelica::Ref::new(Values::Value::CODE { A: metamodelica::Ref::new(Absyn::CodeNode::C_TYPENAME { path: Parser::stringPath(className)? }) }), metamodelica::Ref::new(Values::Value::STRING { string: from }), metamodelica::Ref::new(Values::Value::STRING { string: to }), metamodelica::Ref::new(Values::Value::STRING { string: fromNew }), metamodelica::Ref::new(Values::Value::STRING { string: toNew })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::BOOL { boolean: __pa0 }) => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    res = metamodelica::Own::own(__pa0);
    Ok(res)
}

pub fn updateConnectionAnnotation(
    mut className: ArcStr,
    mut from: ArcStr,
    mut to: ArcStr,
    mut annotate: ArcStr,
) -> Result<bool> {
    let mut res: bool;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), literal!("updateConnectionAnnotation"), list![metamodelica::Ref::new(Values::Value::CODE { A: metamodelica::Ref::new(Absyn::CodeNode::C_TYPENAME { path: Parser::stringPath(className)? }) }), metamodelica::Ref::new(Values::Value::STRING { string: from }), metamodelica::Ref::new(Values::Value::STRING { string: to }), metamodelica::Ref::new(Values::Value::STRING { string: annotate })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::BOOL { boolean: __pa0 }) => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    res = metamodelica::Own::own(__pa0);
    Ok(res)
}

pub fn getConnectorCount(mut className: ArcStr) -> Result<i32> {
    let mut res: i32;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), literal!("getConnectorCount"), list![metamodelica::Ref::new(Values::Value::CODE { A: metamodelica::Ref::new(Absyn::CodeNode::C_TYPENAME { path: Parser::stringPath(className)? }) })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::INTEGER { integer: __pa0 }) => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    res = metamodelica::Own::own(__pa0);
    Ok(res)
}

pub fn setComponentProperties(
    mut className: ArcStr,
    mut componentName: ArcStr,
    mut prefixArray: metamodelica::List<bool>,
    mut variability: metamodelica::List<ArcStr>,
    mut innerOuter: metamodelica::List<bool>,
    mut direction: metamodelica::List<ArcStr>,
) -> Result<bool> {
    let mut res: bool;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), literal!("setComponentProperties"), list![metamodelica::Ref::new(Values::Value::CODE { A: metamodelica::Ref::new(Absyn::CodeNode::C_TYPENAME { path: Parser::stringPath(className)? }) }), metamodelica::Ref::new(Values::Value::CODE { A: metamodelica::Ref::new(Absyn::CodeNode::C_TYPENAME { path: Parser::stringPath(componentName)? }) }), ValuesMake::makeArray(({
        let mut __acc: metamodelica::List<metamodelica::Ref<Values::Value>> = metamodelica::nil();
        for mut prefixArray_iter in (prefixArray).into_iter().cloned() {
            let __x = metamodelica::Ref::new(Values::Value::BOOL { boolean: prefixArray_iter.clone() });
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    })), ValuesMake::makeArray(({
        let mut __acc: metamodelica::List<metamodelica::Ref<Values::Value>> = metamodelica::nil();
        for mut variability_iter in (variability).into_iter().cloned() {
            let __x = metamodelica::Ref::new(Values::Value::STRING { string: variability_iter.clone() });
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    })), ValuesMake::makeArray(({
        let mut __acc: metamodelica::List<metamodelica::Ref<Values::Value>> = metamodelica::nil();
        for mut innerOuter_iter in (innerOuter).into_iter().cloned() {
            let __x = metamodelica::Ref::new(Values::Value::BOOL { boolean: innerOuter_iter.clone() });
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    })), ValuesMake::makeArray(({
        let mut __acc: metamodelica::List<metamodelica::Ref<Values::Value>> = metamodelica::nil();
        for mut direction_iter in (direction).into_iter().cloned() {
            let __x = metamodelica::Ref::new(Values::Value::STRING { string: direction_iter.clone() });
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }))], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::BOOL { boolean: __pa0 }) => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    res = metamodelica::Own::own(__pa0);
    Ok(res)
}

pub fn setComponentComment(mut className: ArcStr, mut componentName: ArcStr, mut comment: ArcStr) -> Result<bool> {
    let mut res: bool;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), literal!("setComponentComment"), list![metamodelica::Ref::new(Values::Value::CODE { A: metamodelica::Ref::new(Absyn::CodeNode::C_TYPENAME { path: Parser::stringPath(className)? }) }), metamodelica::Ref::new(Values::Value::CODE { A: metamodelica::Ref::new(Absyn::CodeNode::C_TYPENAME { path: Parser::stringPath(componentName)? }) }), metamodelica::Ref::new(Values::Value::STRING { string: comment })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::BOOL { boolean: __pa0 }) => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    res = metamodelica::Own::own(__pa0);
    Ok(res)
}

pub fn getComponentComment(mut className: ArcStr, mut componentName: ArcStr) -> Result<ArcStr> {
    let mut res: ArcStr;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), literal!("getComponentComment"), list![metamodelica::Ref::new(Values::Value::CODE { A: metamodelica::Ref::new(Absyn::CodeNode::C_TYPENAME { path: Parser::stringPath(className)? }) }), metamodelica::Ref::new(Values::Value::CODE { A: metamodelica::Ref::new(Absyn::CodeNode::C_TYPENAME { path: Parser::stringPath(componentName)? }) })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::STRING { string: __pa0 }) => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    res = metamodelica::Own::own(__pa0);
    Ok(res)
}

pub fn removeExtendsModifiers(
    mut className: ArcStr,
    mut baseClassName: ArcStr,
    mut keepRedeclares: bool,
) -> Result<bool> {
    let mut res: bool;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), literal!("removeExtendsModifiers"), list![metamodelica::Ref::new(Values::Value::CODE { A: metamodelica::Ref::new(Absyn::CodeNode::C_TYPENAME { path: Parser::stringPath(className)? }) }), metamodelica::Ref::new(Values::Value::CODE { A: metamodelica::Ref::new(Absyn::CodeNode::C_TYPENAME { path: Parser::stringPath(baseClassName)? }) }), metamodelica::Ref::new(Values::Value::BOOL { boolean: keepRedeclares })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::BOOL { boolean: __pa0 }) => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    res = metamodelica::Own::own(__pa0);
    Ok(res)
}

pub fn getInstantiatedParametersAndValues(mut cls: ArcStr) -> Result<metamodelica::List<ArcStr>> {
    let mut res: metamodelica::List<ArcStr>;
    let mut res_arr: metamodelica::Ref<Values::Value>;
    (_, res_arr) = CevalScript::cevalInteractiveFunctions2(
        FCore::emptyCache(),
        FGraph::empty(),
        literal!("getInstantiatedParametersAndValues"),
        list![metamodelica::Ref::new(Values::Value::CODE {
            A: metamodelica::Ref::new(Absyn::CodeNode::C_TYPENAME {
                path: Parser::stringPath(cls)?
            })
        })],
        dummyMsg.clone(),
    )?;
    res = ({
        let mut __acc: metamodelica::List<ArcStr> = metamodelica::nil();
        for mut res_arr_iter in (ValuesUtil::arrayValues(&res_arr)?).into_iter().cloned() {
            let __x = (match &*res_arr_iter.clone() {
                Values::Value::STRING {
                    string: __res_arr_iter_string,
                } => __res_arr_iter_string.clone(),
                _ => return Err("match: no arm matched"),
            });
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    Ok(res)
}

pub fn getElementAnnotation(mut elementName: ArcStr) -> Result<ArcStr> {
    let mut res: ArcStr;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), literal!("getElementAnnotation"), list![metamodelica::Ref::new(Values::Value::CODE { A: metamodelica::Ref::new(Absyn::CodeNode::C_TYPENAME { path: Parser::stringPath(elementName)? }) })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::STRING { string: __pa0 }) => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    res = metamodelica::Own::own(__pa0);
    Ok(res)
}

pub fn getNthComponentCondition(mut className: ArcStr, mut n: i32) -> Result<ArcStr> {
    let mut res: ArcStr;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), literal!("getNthComponentCondition"), list![metamodelica::Ref::new(Values::Value::CODE { A: metamodelica::Ref::new(Absyn::CodeNode::C_TYPENAME { path: Parser::stringPath(className)? }) }), metamodelica::Ref::new(Values::Value::INTEGER { integer: n })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::STRING { string: __pa0 }) => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    res = metamodelica::Own::own(__pa0);
    Ok(res)
}

pub fn getComponentCount(mut classPath: ArcStr) -> Result<i32> {
    let mut res: i32;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), literal!("getComponentCount"), list![metamodelica::Ref::new(Values::Value::CODE { A: metamodelica::Ref::new(Absyn::CodeNode::C_TYPENAME { path: Parser::stringPath(classPath)? }) })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::INTEGER { integer: __pa0 }) => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    res = metamodelica::Own::own(__pa0);
    Ok(res)
}

pub fn isExtendsModifierFinal(
    mut className: ArcStr,
    mut extendsName: ArcStr,
    mut modifierName: ArcStr,
) -> Result<bool> {
    let mut res: bool;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), literal!("isExtendsModifierFinal"), list![metamodelica::Ref::new(Values::Value::CODE { A: metamodelica::Ref::new(Absyn::CodeNode::C_TYPENAME { path: Parser::stringPath(className)? }) }), metamodelica::Ref::new(Values::Value::CODE { A: metamodelica::Ref::new(Absyn::CodeNode::C_TYPENAME { path: Parser::stringPath(extendsName)? }) }), metamodelica::Ref::new(Values::Value::CODE { A: metamodelica::Ref::new(Absyn::CodeNode::C_TYPENAME { path: Parser::stringPath(modifierName)? }) })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::BOOL { boolean: __pa0 }) => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    res = metamodelica::Own::own(__pa0);
    Ok(res)
}

pub fn removeElementModifiers(
    mut className: ArcStr,
    mut componentName: ArcStr,
    mut keepRedeclares: bool,
) -> Result<bool> {
    let mut res: bool;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), literal!("removeElementModifiers"), list![metamodelica::Ref::new(Values::Value::CODE { A: metamodelica::Ref::new(Absyn::CodeNode::C_TYPENAME { path: Parser::stringPath(className)? }) }), metamodelica::Ref::new(Values::Value::STRING { string: componentName }), metamodelica::Ref::new(Values::Value::BOOL { boolean: keepRedeclares })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::BOOL { boolean: __pa0 }) => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    res = metamodelica::Own::own(__pa0);
    Ok(res)
}

pub fn getElementModifierValues(mut className: ArcStr, mut modifier: ArcStr) -> Result<ArcStr> {
    let mut res: ArcStr;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), literal!("getElementModifierValues"), list![metamodelica::Ref::new(Values::Value::CODE { A: metamodelica::Ref::new(Absyn::CodeNode::C_TYPENAME { path: Parser::stringPath(className)? }) }), metamodelica::Ref::new(Values::Value::CODE { A: metamodelica::Ref::new(Absyn::CodeNode::C_TYPENAME { path: Parser::stringPath(modifier)? }) })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::STRING { string: __pa0 }) => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    res = metamodelica::Own::own(__pa0);
    Ok(res)
}

pub fn getElementModifierValue(mut className: ArcStr, mut modifier: ArcStr) -> Result<ArcStr> {
    let mut res: ArcStr;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), literal!("getElementModifierValue"), list![metamodelica::Ref::new(Values::Value::CODE { A: metamodelica::Ref::new(Absyn::CodeNode::C_TYPENAME { path: Parser::stringPath(className)? }) }), metamodelica::Ref::new(Values::Value::CODE { A: metamodelica::Ref::new(Absyn::CodeNode::C_TYPENAME { path: Parser::stringPath(modifier)? }) })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::STRING { string: __pa0 }) => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    res = metamodelica::Own::own(__pa0);
    Ok(res)
}

pub fn getExtendsModifierNames(mut className: ArcStr, mut extendsName: ArcStr, mut useQuotes: bool) -> Result<ArcStr> {
    let mut res: ArcStr;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), literal!("getExtendsModifierNames"), list![metamodelica::Ref::new(Values::Value::CODE { A: metamodelica::Ref::new(Absyn::CodeNode::C_TYPENAME { path: Parser::stringPath(className)? }) }), metamodelica::Ref::new(Values::Value::CODE { A: metamodelica::Ref::new(Absyn::CodeNode::C_TYPENAME { path: Parser::stringPath(extendsName)? }) }), metamodelica::Ref::new(Values::Value::BOOL { boolean: useQuotes })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::STRING { string: __pa0 }) => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    res = metamodelica::Own::own(__pa0);
    Ok(res)
}

pub fn getElementModifierNames(mut className: ArcStr, mut elementName: ArcStr) -> Result<metamodelica::List<ArcStr>> {
    let mut res: metamodelica::List<ArcStr>;
    let mut res_arr: metamodelica::Ref<Values::Value>;
    (_, res_arr) = CevalScript::cevalInteractiveFunctions2(
        FCore::emptyCache(),
        FGraph::empty(),
        literal!("getElementModifierNames"),
        list![
            metamodelica::Ref::new(Values::Value::CODE {
                A: metamodelica::Ref::new(Absyn::CodeNode::C_TYPENAME {
                    path: Parser::stringPath(className)?
                })
            }),
            metamodelica::Ref::new(Values::Value::STRING { string: elementName })
        ],
        dummyMsg.clone(),
    )?;
    res = ({
        let mut __acc: metamodelica::List<ArcStr> = metamodelica::nil();
        for mut res_arr_iter in (ValuesUtil::arrayValues(&res_arr)?).into_iter().cloned() {
            let __x = (match &*res_arr_iter.clone() {
                Values::Value::STRING {
                    string: __res_arr_iter_string,
                } => __res_arr_iter_string.clone(),
                _ => return Err("match: no arm matched"),
            });
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    Ok(res)
}

pub fn removeComponentModifiers(
    mut class_: ArcStr,
    mut componentName: ArcStr,
    mut keepRedeclares: bool,
) -> Result<bool> {
    let mut res: bool;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), literal!("removeComponentModifiers"), list![metamodelica::Ref::new(Values::Value::CODE { A: metamodelica::Ref::new(Absyn::CodeNode::C_TYPENAME { path: Parser::stringPath(class_)? }) }), metamodelica::Ref::new(Values::Value::STRING { string: componentName }), metamodelica::Ref::new(Values::Value::BOOL { boolean: keepRedeclares })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::BOOL { boolean: __pa0 }) => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    res = metamodelica::Own::own(__pa0);
    Ok(res)
}

pub fn getComponentModifierValues(mut class_: ArcStr, mut modifier: ArcStr) -> Result<ArcStr> {
    let mut res: ArcStr;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), literal!("getComponentModifierValues"), list![metamodelica::Ref::new(Values::Value::CODE { A: metamodelica::Ref::new(Absyn::CodeNode::C_TYPENAME { path: Parser::stringPath(class_)? }) }), metamodelica::Ref::new(Values::Value::CODE { A: metamodelica::Ref::new(Absyn::CodeNode::C_TYPENAME { path: Parser::stringPath(modifier)? }) })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::STRING { string: __pa0 }) => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    res = metamodelica::Own::own(__pa0);
    Ok(res)
}

pub fn getComponentModifierValue(mut class_: ArcStr, mut modifier: ArcStr) -> Result<ArcStr> {
    let mut res: ArcStr;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), literal!("getComponentModifierValue"), list![metamodelica::Ref::new(Values::Value::CODE { A: metamodelica::Ref::new(Absyn::CodeNode::C_TYPENAME { path: Parser::stringPath(class_)? }) }), metamodelica::Ref::new(Values::Value::CODE { A: metamodelica::Ref::new(Absyn::CodeNode::C_TYPENAME { path: Parser::stringPath(modifier)? }) })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::STRING { string: __pa0 }) => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    res = metamodelica::Own::own(__pa0);
    Ok(res)
}

pub fn getComponentModifierNames(mut class_: ArcStr, mut componentName: ArcStr) -> Result<metamodelica::List<ArcStr>> {
    let mut res: metamodelica::List<ArcStr>;
    let mut res_arr: metamodelica::Ref<Values::Value>;
    (_, res_arr) = CevalScript::cevalInteractiveFunctions2(
        FCore::emptyCache(),
        FGraph::empty(),
        literal!("getComponentModifierNames"),
        list![
            metamodelica::Ref::new(Values::Value::CODE {
                A: metamodelica::Ref::new(Absyn::CodeNode::C_TYPENAME {
                    path: Parser::stringPath(class_)?
                })
            }),
            metamodelica::Ref::new(Values::Value::STRING { string: componentName })
        ],
        dummyMsg.clone(),
    )?;
    res = ({
        let mut __acc: metamodelica::List<ArcStr> = metamodelica::nil();
        for mut res_arr_iter in (ValuesUtil::arrayValues(&res_arr)?).into_iter().cloned() {
            let __x = (match &*res_arr_iter.clone() {
                Values::Value::STRING {
                    string: __res_arr_iter_string,
                } => __res_arr_iter_string.clone(),
                _ => return Err("match: no arm matched"),
            });
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    Ok(res)
}

pub fn getElements(mut className: ArcStr, mut useQuotes: bool) -> Result<()> {
    ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), literal!("getElements"), list![metamodelica::Ref::new(Values::Value::CODE { A: metamodelica::Ref::new(Absyn::CodeNode::C_TYPENAME { path: Parser::stringPath(className)? }) }), metamodelica::Ref::new(Values::Value::BOOL { boolean: useQuotes })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::NORETCALL { .. }) => (),
        _ => return Err("pattern mismatch"),
    } };
    Ok(())
}

pub fn getComponents(mut className: ArcStr, mut useQuotes: bool) -> Result<()> {
    ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), literal!("getComponents"), list![metamodelica::Ref::new(Values::Value::CODE { A: metamodelica::Ref::new(Absyn::CodeNode::C_TYPENAME { path: Parser::stringPath(className)? }) }), metamodelica::Ref::new(Values::Value::BOOL { boolean: useQuotes })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::NORETCALL { .. }) => (),
        _ => return Err("pattern mismatch"),
    } };
    Ok(())
}

pub fn getParameterValue(mut class_: ArcStr, mut parameterName: ArcStr) -> Result<ArcStr> {
    let mut res: ArcStr;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), literal!("getParameterValue"), list![metamodelica::Ref::new(Values::Value::CODE { A: metamodelica::Ref::new(Absyn::CodeNode::C_TYPENAME { path: Parser::stringPath(class_)? }) }), metamodelica::Ref::new(Values::Value::STRING { string: parameterName })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::STRING { string: __pa0 }) => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    res = metamodelica::Own::own(__pa0);
    Ok(res)
}

pub fn getParameterNames(mut class_: ArcStr) -> Result<metamodelica::List<ArcStr>> {
    let mut res: metamodelica::List<ArcStr>;
    let mut res_arr: metamodelica::Ref<Values::Value>;
    (_, res_arr) = CevalScript::cevalInteractiveFunctions2(
        FCore::emptyCache(),
        FGraph::empty(),
        literal!("getParameterNames"),
        list![metamodelica::Ref::new(Values::Value::CODE {
            A: metamodelica::Ref::new(Absyn::CodeNode::C_TYPENAME {
                path: Parser::stringPath(class_)?
            })
        })],
        dummyMsg.clone(),
    )?;
    res = ({
        let mut __acc: metamodelica::List<ArcStr> = metamodelica::nil();
        for mut res_arr_iter in (ValuesUtil::arrayValues(&res_arr)?).into_iter().cloned() {
            let __x = (match &*res_arr_iter.clone() {
                Values::Value::STRING {
                    string: __res_arr_iter_string,
                } => __res_arr_iter_string.clone(),
                _ => return Err("match: no arm matched"),
            });
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    Ok(res)
}

pub fn deleteComponent(mut componentName: ArcStr, mut classPath: ArcStr) -> Result<bool> {
    let mut res: bool;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), literal!("deleteComponent"), list![metamodelica::Ref::new(Values::Value::CODE { A: metamodelica::Ref::new(Absyn::CodeNode::C_TYPENAME { path: Parser::stringPath(componentName)? }) }), metamodelica::Ref::new(Values::Value::CODE { A: metamodelica::Ref::new(Absyn::CodeNode::C_TYPENAME { path: Parser::stringPath(classPath)? }) })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::BOOL { boolean: __pa0 }) => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    res = metamodelica::Own::own(__pa0);
    Ok(res)
}

pub fn closeSimulationResultFile() -> Result<bool> {
    let mut res: bool;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), literal!("closeSimulationResultFile"), metamodelica::nil(), dummyMsg.clone())?) {
        (_, Deref @ Values::Value::BOOL { boolean: __pa0 }) => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    res = metamodelica::Own::own(__pa0);
    Ok(res)
}

pub fn checkCodeGraph(mut graphfile: ArcStr, mut codefile: ArcStr) -> Result<metamodelica::List<ArcStr>> {
    let mut res: metamodelica::List<ArcStr>;
    let mut res_arr: metamodelica::Ref<Values::Value>;
    (_, res_arr) = CevalScript::cevalInteractiveFunctions2(
        FCore::emptyCache(),
        FGraph::empty(),
        literal!("checkCodeGraph"),
        list![
            metamodelica::Ref::new(Values::Value::STRING { string: graphfile }),
            metamodelica::Ref::new(Values::Value::STRING { string: codefile })
        ],
        dummyMsg.clone(),
    )?;
    res = ({
        let mut __acc: metamodelica::List<ArcStr> = metamodelica::nil();
        for mut res_arr_iter in (ValuesUtil::arrayValues(&res_arr)?).into_iter().cloned() {
            let __x = (match &*res_arr_iter.clone() {
                Values::Value::STRING {
                    string: __res_arr_iter_string,
                } => __res_arr_iter_string.clone(),
                _ => return Err("match: no arm matched"),
            });
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    Ok(res)
}

pub fn checkTaskGraph(mut filename: ArcStr, mut reffilename: ArcStr) -> Result<metamodelica::List<ArcStr>> {
    let mut res: metamodelica::List<ArcStr>;
    let mut res_arr: metamodelica::Ref<Values::Value>;
    (_, res_arr) = CevalScript::cevalInteractiveFunctions2(
        FCore::emptyCache(),
        FGraph::empty(),
        literal!("checkTaskGraph"),
        list![
            metamodelica::Ref::new(Values::Value::STRING { string: filename }),
            metamodelica::Ref::new(Values::Value::STRING { string: reffilename })
        ],
        dummyMsg.clone(),
    )?;
    res = ({
        let mut __acc: metamodelica::List<ArcStr> = metamodelica::nil();
        for mut res_arr_iter in (ValuesUtil::arrayValues(&res_arr)?).into_iter().cloned() {
            let __x = (match &*res_arr_iter.clone() {
                Values::Value::STRING {
                    string: __res_arr_iter_string,
                } => __res_arr_iter_string.clone(),
                _ => return Err("match: no arm matched"),
            });
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    Ok(res)
}

pub fn diffSimulationResultsHtml(
    mut var: ArcStr,
    mut actualFile: ArcStr,
    mut expectedFile: ArcStr,
    mut relTol: metamodelica::Real,
    mut relTolDiffMinMax: metamodelica::Real,
    mut rangeDelta: metamodelica::Real,
) -> Result<ArcStr> {
    let mut res: ArcStr;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), literal!("diffSimulationResultsHtml"), list![metamodelica::Ref::new(Values::Value::STRING { string: var }), metamodelica::Ref::new(Values::Value::STRING { string: actualFile }), metamodelica::Ref::new(Values::Value::STRING { string: expectedFile }), metamodelica::Ref::new(Values::Value::REAL { real: relTol }), metamodelica::Ref::new(Values::Value::REAL { real: relTolDiffMinMax }), metamodelica::Ref::new(Values::Value::REAL { real: rangeDelta })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::STRING { string: __pa0 }) => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    res = metamodelica::Own::own(__pa0);
    Ok(res)
}

pub fn diffSimulationResults(
    mut actualFile: ArcStr,
    mut expectedFile: ArcStr,
    mut diffPrefix: ArcStr,
    mut relTol: metamodelica::Real,
    mut relTolDiffMinMax: metamodelica::Real,
    mut rangeDelta: metamodelica::Real,
    mut vars: metamodelica::List<ArcStr>,
    mut keepEqualResults: bool,
) -> Result<(bool, metamodelica::List<ArcStr>)> {
    let mut res1: bool;
    let mut res2: metamodelica::List<ArcStr>;
    let mut res2_arr: metamodelica::Ref<Values::Value>;
    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), literal!("diffSimulationResults"), list![metamodelica::Ref::new(Values::Value::STRING { string: actualFile }), metamodelica::Ref::new(Values::Value::STRING { string: expectedFile }), metamodelica::Ref::new(Values::Value::STRING { string: diffPrefix }), metamodelica::Ref::new(Values::Value::REAL { real: relTol }), metamodelica::Ref::new(Values::Value::REAL { real: relTolDiffMinMax }), metamodelica::Ref::new(Values::Value::REAL { real: rangeDelta }), ValuesMake::makeArray(({
        let mut __acc: metamodelica::List<metamodelica::Ref<Values::Value>> = metamodelica::nil();
        for mut vars_iter in (vars).into_iter().cloned() {
            let __x = metamodelica::Ref::new(Values::Value::STRING { string: vars_iter.clone() });
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    })), metamodelica::Ref::new(Values::Value::BOOL { boolean: keepEqualResults })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::TUPLE { valueLst: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::BOOL { boolean: __pa0 }, tail: Deref @ metamodelica::ListNode::Cons { head: __pa1, tail: Deref @ metamodelica::ListNode::Nil } } }) => (__pa0.clone(), __pa1.clone()),
        _ => return Err("pattern mismatch"),
    } };
    res1 = metamodelica::Own::own(__pa0);
    res2_arr = metamodelica::Own::own(__pa1);
    res2 = ({
        let mut __acc: metamodelica::List<ArcStr> = metamodelica::nil();
        for mut res2_arr_iter in (ValuesUtil::arrayValues(&res2_arr)?).into_iter().cloned() {
            let __x = (match &*res2_arr_iter.clone() {
                Values::Value::STRING {
                    string: __res2_arr_iter_string,
                } => __res2_arr_iter_string.clone(),
                _ => return Err("match: no arm matched"),
            });
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    Ok((res1, res2))
}

pub fn deltaSimulationResults(
    mut filename: ArcStr,
    mut reffilename: ArcStr,
    mut method: ArcStr,
    mut vars: metamodelica::List<ArcStr>,
) -> Result<metamodelica::Real> {
    let mut res: metamodelica::Real;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), literal!("deltaSimulationResults"), list![metamodelica::Ref::new(Values::Value::STRING { string: filename }), metamodelica::Ref::new(Values::Value::STRING { string: reffilename }), metamodelica::Ref::new(Values::Value::STRING { string: method }), ValuesMake::makeArray(({
        let mut __acc: metamodelica::List<metamodelica::Ref<Values::Value>> = metamodelica::nil();
        for mut vars_iter in (vars).into_iter().cloned() {
            let __x = metamodelica::Ref::new(Values::Value::STRING { string: vars_iter.clone() });
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }))], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::REAL { real: __pa0 }) => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    res = metamodelica::Own::own(__pa0);
    Ok(res)
}

pub fn compareSimulationResults(
    mut filename: ArcStr,
    mut reffilename: ArcStr,
    mut logfilename: ArcStr,
    mut relTol: metamodelica::Real,
    mut absTol: metamodelica::Real,
    mut vars: metamodelica::List<ArcStr>,
) -> Result<metamodelica::List<ArcStr>> {
    let mut res: metamodelica::List<ArcStr>;
    let mut res_arr: metamodelica::Ref<Values::Value>;
    (_, res_arr) = CevalScript::cevalInteractiveFunctions2(
        FCore::emptyCache(),
        FGraph::empty(),
        literal!("compareSimulationResults"),
        list![
            metamodelica::Ref::new(Values::Value::STRING { string: filename }),
            metamodelica::Ref::new(Values::Value::STRING { string: reffilename }),
            metamodelica::Ref::new(Values::Value::STRING { string: logfilename }),
            metamodelica::Ref::new(Values::Value::REAL { real: relTol }),
            metamodelica::Ref::new(Values::Value::REAL { real: absTol }),
            ValuesMake::makeArray(
                ({
                    let mut __acc: metamodelica::List<metamodelica::Ref<Values::Value>> = metamodelica::nil();
                    for mut vars_iter in (vars).into_iter().cloned() {
                        let __x = metamodelica::Ref::new(Values::Value::STRING {
                            string: vars_iter.clone(),
                        });
                        __acc = cons(__x, __acc);
                    }
                    __acc.reverse()
                })
            )
        ],
        dummyMsg.clone(),
    )?;
    res = ({
        let mut __acc: metamodelica::List<ArcStr> = metamodelica::nil();
        for mut res_arr_iter in (ValuesUtil::arrayValues(&res_arr)?).into_iter().cloned() {
            let __x = (match &*res_arr_iter.clone() {
                Values::Value::STRING {
                    string: __res_arr_iter_string,
                } => __res_arr_iter_string.clone(),
                _ => return Err("match: no arm matched"),
            });
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    Ok(res)
}

pub fn filterSimulationResults(
    mut inFile: ArcStr,
    mut outFile: ArcStr,
    mut vars: metamodelica::List<ArcStr>,
    mut numberOfIntervals: i32,
    mut removeDescription: bool,
    mut hintReadAllVars: bool,
) -> Result<bool> {
    let mut res: bool;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), literal!("filterSimulationResults"), list![metamodelica::Ref::new(Values::Value::STRING { string: inFile }), metamodelica::Ref::new(Values::Value::STRING { string: outFile }), ValuesMake::makeArray(({
        let mut __acc: metamodelica::List<metamodelica::Ref<Values::Value>> = metamodelica::nil();
        for mut vars_iter in (vars).into_iter().cloned() {
            let __x = metamodelica::Ref::new(Values::Value::STRING { string: vars_iter.clone() });
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    })), metamodelica::Ref::new(Values::Value::INTEGER { integer: numberOfIntervals }), metamodelica::Ref::new(Values::Value::BOOL { boolean: removeDescription }), metamodelica::Ref::new(Values::Value::BOOL { boolean: hintReadAllVars })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::BOOL { boolean: __pa0 }) => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    res = metamodelica::Own::own(__pa0);
    Ok(res)
}

pub fn readSimulationResultVars(
    mut fileName: ArcStr,
    mut readParameters: bool,
    mut openmodelicaStyle: bool,
) -> Result<metamodelica::List<ArcStr>> {
    let mut res: metamodelica::List<ArcStr>;
    let mut res_arr: metamodelica::Ref<Values::Value>;
    (_, res_arr) = CevalScript::cevalInteractiveFunctions2(
        FCore::emptyCache(),
        FGraph::empty(),
        literal!("readSimulationResultVars"),
        list![
            metamodelica::Ref::new(Values::Value::STRING { string: fileName }),
            metamodelica::Ref::new(Values::Value::BOOL {
                boolean: readParameters
            }),
            metamodelica::Ref::new(Values::Value::BOOL {
                boolean: openmodelicaStyle
            })
        ],
        dummyMsg.clone(),
    )?;
    res = ({
        let mut __acc: metamodelica::List<ArcStr> = metamodelica::nil();
        for mut res_arr_iter in (ValuesUtil::arrayValues(&res_arr)?).into_iter().cloned() {
            let __x = (match &*res_arr_iter.clone() {
                Values::Value::STRING {
                    string: __res_arr_iter_string,
                } => __res_arr_iter_string.clone(),
                _ => return Err("match: no arm matched"),
            });
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    Ok(res)
}

pub fn readSimulationResultSize(mut fileName: ArcStr) -> Result<i32> {
    let mut res: i32;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), literal!("readSimulationResultSize"), list![metamodelica::Ref::new(Values::Value::STRING { string: fileName })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::INTEGER { integer: __pa0 }) => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    res = metamodelica::Own::own(__pa0);
    Ok(res)
}

pub fn plotAll(
    mut externalWindow: bool,
    mut fileName: ArcStr,
    mut title: ArcStr,
    mut grid: ArcStr,
    mut logX: bool,
    mut logY: bool,
    mut xLabel: ArcStr,
    mut yLabel: ArcStr,
    mut xRange: metamodelica::List<metamodelica::Real>,
    mut yRange: metamodelica::List<metamodelica::Real>,
    mut curveWidth: metamodelica::Real,
    mut curveStyle: i32,
    mut legendPosition: ArcStr,
    mut footer: ArcStr,
    mut autoScale: bool,
    mut forceOMPlot: bool,
    mut yAxis: ArcStr,
    mut yLabelRight: ArcStr,
    mut yRangeRight: metamodelica::List<metamodelica::Real>,
) -> Result<bool> {
    let mut res: bool;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), literal!("plotAll"), list![metamodelica::Ref::new(Values::Value::BOOL { boolean: externalWindow }), metamodelica::Ref::new(Values::Value::STRING { string: fileName }), metamodelica::Ref::new(Values::Value::STRING { string: title }), metamodelica::Ref::new(Values::Value::STRING { string: grid }), metamodelica::Ref::new(Values::Value::BOOL { boolean: logX }), metamodelica::Ref::new(Values::Value::BOOL { boolean: logY }), metamodelica::Ref::new(Values::Value::STRING { string: xLabel }), metamodelica::Ref::new(Values::Value::STRING { string: yLabel }), ValuesMake::makeArray(({
        let mut __acc: metamodelica::List<metamodelica::Ref<Values::Value>> = metamodelica::nil();
        for mut xRange_iter in (xRange).into_iter().cloned() {
            let __x = metamodelica::Ref::new(Values::Value::REAL { real: xRange_iter.clone() });
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    })), ValuesMake::makeArray(({
        let mut __acc: metamodelica::List<metamodelica::Ref<Values::Value>> = metamodelica::nil();
        for mut yRange_iter in (yRange).into_iter().cloned() {
            let __x = metamodelica::Ref::new(Values::Value::REAL { real: yRange_iter.clone() });
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    })), metamodelica::Ref::new(Values::Value::REAL { real: curveWidth }), metamodelica::Ref::new(Values::Value::INTEGER { integer: curveStyle }), metamodelica::Ref::new(Values::Value::STRING { string: legendPosition }), metamodelica::Ref::new(Values::Value::STRING { string: footer }), metamodelica::Ref::new(Values::Value::BOOL { boolean: autoScale }), metamodelica::Ref::new(Values::Value::BOOL { boolean: forceOMPlot }), metamodelica::Ref::new(Values::Value::STRING { string: yAxis }), metamodelica::Ref::new(Values::Value::STRING { string: yLabelRight }), ValuesMake::makeArray(({
        let mut __acc: metamodelica::List<metamodelica::Ref<Values::Value>> = metamodelica::nil();
        for mut yRangeRight_iter in (yRangeRight).into_iter().cloned() {
            let __x = metamodelica::Ref::new(Values::Value::REAL { real: yRangeRight_iter.clone() });
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }))], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::BOOL { boolean: __pa0 }) => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    res = metamodelica::Own::own(__pa0);
    Ok(res)
}

pub fn getReplaceableChoices(
    mut baseClass: ArcStr,
    mut parentClass: ArcStr,
    mut includePartial: bool,
    mut sort: bool,
) -> Result<metamodelica::List<metamodelica::List<ArcStr>>> {
    let mut res: metamodelica::List<metamodelica::List<ArcStr>>;
    let mut res_arr: metamodelica::Ref<Values::Value>;
    (_, res_arr) = CevalScript::cevalInteractiveFunctions2(
        FCore::emptyCache(),
        FGraph::empty(),
        literal!("getReplaceableChoices"),
        list![
            metamodelica::Ref::new(Values::Value::CODE {
                A: metamodelica::Ref::new(Absyn::CodeNode::C_TYPENAME {
                    path: Parser::stringPath(baseClass)?
                })
            }),
            metamodelica::Ref::new(Values::Value::CODE {
                A: metamodelica::Ref::new(Absyn::CodeNode::C_TYPENAME {
                    path: Parser::stringPath(parentClass)?
                })
            }),
            metamodelica::Ref::new(Values::Value::BOOL {
                boolean: includePartial
            }),
            metamodelica::Ref::new(Values::Value::BOOL { boolean: sort })
        ],
        dummyMsg.clone(),
    )?;
    res = ({
        let mut __acc: metamodelica::List<metamodelica::List<ArcStr>> = metamodelica::nil();
        for mut res_arr_iter in (ValuesUtil::arrayValues(&res_arr)?).into_iter().cloned() {
            let __x = ({
                let mut __acc: metamodelica::List<ArcStr> = metamodelica::nil();
                for mut res_arr_iter_iter in (ValuesUtil::arrayValues(&(res_arr_iter.clone()))?).into_iter().cloned() {
                    let __x = (match &*res_arr_iter_iter.clone() {
                        Values::Value::STRING {
                            string: __res_arr_iter_iter_string,
                        } => __res_arr_iter_iter_string.clone(),
                        _ => return Err("match: no arm matched"),
                    });
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            });
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    Ok(res)
}

pub fn getAllSubtypeOf(
    mut className: ArcStr,
    mut parentClass: ArcStr,
    mut qualified: bool,
    mut includePartial: bool,
    mut sort: bool,
) -> Result<metamodelica::List<ArcStr>> {
    let mut res: metamodelica::List<ArcStr>;
    let mut res_arr: metamodelica::Ref<Values::Value>;
    (_, res_arr) = CevalScript::cevalInteractiveFunctions2(
        FCore::emptyCache(),
        FGraph::empty(),
        literal!("getAllSubtypeOf"),
        list![
            metamodelica::Ref::new(Values::Value::CODE {
                A: metamodelica::Ref::new(Absyn::CodeNode::C_TYPENAME {
                    path: Parser::stringPath(className)?
                })
            }),
            metamodelica::Ref::new(Values::Value::CODE {
                A: metamodelica::Ref::new(Absyn::CodeNode::C_TYPENAME {
                    path: Parser::stringPath(parentClass)?
                })
            }),
            metamodelica::Ref::new(Values::Value::BOOL { boolean: qualified }),
            metamodelica::Ref::new(Values::Value::BOOL {
                boolean: includePartial
            }),
            metamodelica::Ref::new(Values::Value::BOOL { boolean: sort })
        ],
        dummyMsg.clone(),
    )?;
    res = ({
        let mut __acc: metamodelica::List<ArcStr> = metamodelica::nil();
        for mut res_arr_iter in (ValuesUtil::arrayValues(&res_arr)?).into_iter().cloned() {
            let __x = ValuesDump::valString(&(res_arr_iter.clone()))?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    Ok(res)
}

pub fn getPackages(mut class_: ArcStr) -> Result<metamodelica::List<ArcStr>> {
    let mut res: metamodelica::List<ArcStr>;
    let mut res_arr: metamodelica::Ref<Values::Value>;
    (_, res_arr) = CevalScript::cevalInteractiveFunctions2(
        FCore::emptyCache(),
        FGraph::empty(),
        literal!("getPackages"),
        list![metamodelica::Ref::new(Values::Value::CODE {
            A: metamodelica::Ref::new(Absyn::CodeNode::C_TYPENAME {
                path: Parser::stringPath(class_)?
            })
        })],
        dummyMsg.clone(),
    )?;
    res = ({
        let mut __acc: metamodelica::List<ArcStr> = metamodelica::nil();
        for mut res_arr_iter in (ValuesUtil::arrayValues(&res_arr)?).into_iter().cloned() {
            let __x = ValuesDump::valString(&(res_arr_iter.clone()))?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    Ok(res)
}

pub fn getUsedClassNames(mut className: ArcStr) -> Result<metamodelica::List<ArcStr>> {
    let mut res: metamodelica::List<ArcStr>;
    let mut res_arr: metamodelica::Ref<Values::Value>;
    (_, res_arr) = CevalScript::cevalInteractiveFunctions2(
        FCore::emptyCache(),
        FGraph::empty(),
        literal!("getUsedClassNames"),
        list![metamodelica::Ref::new(Values::Value::CODE {
            A: metamodelica::Ref::new(Absyn::CodeNode::C_TYPENAME {
                path: Parser::stringPath(className)?
            })
        })],
        dummyMsg.clone(),
    )?;
    res = ({
        let mut __acc: metamodelica::List<ArcStr> = metamodelica::nil();
        for mut res_arr_iter in (ValuesUtil::arrayValues(&res_arr)?).into_iter().cloned() {
            let __x = ValuesDump::valString(&(res_arr_iter.clone()))?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    Ok(res)
}

pub fn getClassNames(
    mut class_: ArcStr,
    mut recursive: bool,
    mut qualified: bool,
    mut sort: bool,
    mut builtin: bool,
    mut showProtected: bool,
    mut includeConstants: bool,
) -> Result<metamodelica::List<ArcStr>> {
    let mut res: metamodelica::List<ArcStr>;
    let mut res_arr: metamodelica::Ref<Values::Value>;
    (_, res_arr) = CevalScript::cevalInteractiveFunctions2(
        FCore::emptyCache(),
        FGraph::empty(),
        literal!("getClassNames"),
        list![
            metamodelica::Ref::new(Values::Value::CODE {
                A: metamodelica::Ref::new(Absyn::CodeNode::C_TYPENAME {
                    path: Parser::stringPath(class_)?
                })
            }),
            metamodelica::Ref::new(Values::Value::BOOL { boolean: recursive }),
            metamodelica::Ref::new(Values::Value::BOOL { boolean: qualified }),
            metamodelica::Ref::new(Values::Value::BOOL { boolean: sort }),
            metamodelica::Ref::new(Values::Value::BOOL { boolean: builtin }),
            metamodelica::Ref::new(Values::Value::BOOL { boolean: showProtected }),
            metamodelica::Ref::new(Values::Value::BOOL {
                boolean: includeConstants
            })
        ],
        dummyMsg.clone(),
    )?;
    res = ({
        let mut __acc: metamodelica::List<ArcStr> = metamodelica::nil();
        for mut res_arr_iter in (ValuesUtil::arrayValues(&res_arr)?).into_iter().cloned() {
            let __x = ValuesDump::valString(&(res_arr_iter.clone()))?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    Ok(res)
}

pub fn setClassComment(mut class_: ArcStr, mut filename: ArcStr) -> Result<bool> {
    let mut res: bool;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), literal!("setClassComment"), list![metamodelica::Ref::new(Values::Value::CODE { A: metamodelica::Ref::new(Absyn::CodeNode::C_TYPENAME { path: Parser::stringPath(class_)? }) }), metamodelica::Ref::new(Values::Value::STRING { string: filename })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::BOOL { boolean: __pa0 }) => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    res = metamodelica::Own::own(__pa0);
    Ok(res)
}

pub fn isShortDefinition(mut class_: ArcStr) -> Result<bool> {
    let mut res: bool;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), literal!("isShortDefinition"), list![metamodelica::Ref::new(Values::Value::CODE { A: metamodelica::Ref::new(Absyn::CodeNode::C_TYPENAME { path: Parser::stringPath(class_)? }) })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::BOOL { boolean: __pa0 }) => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    res = metamodelica::Own::own(__pa0);
    Ok(res)
}

pub fn setSourceFile(mut class_: ArcStr, mut filename: ArcStr) -> Result<bool> {
    let mut res: bool;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), literal!("setSourceFile"), list![metamodelica::Ref::new(Values::Value::CODE { A: metamodelica::Ref::new(Absyn::CodeNode::C_TYPENAME { path: Parser::stringPath(class_)? }) }), metamodelica::Ref::new(Values::Value::STRING { string: filename })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::BOOL { boolean: __pa0 }) => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    res = metamodelica::Own::own(__pa0);
    Ok(res)
}

pub fn getSourceFile(mut class_: ArcStr) -> Result<ArcStr> {
    let mut res: ArcStr;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), literal!("getSourceFile"), list![metamodelica::Ref::new(Values::Value::CODE { A: metamodelica::Ref::new(Absyn::CodeNode::C_TYPENAME { path: Parser::stringPath(class_)? }) })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::STRING { string: __pa0 }) => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    res = metamodelica::Own::own(__pa0);
    Ok(res)
}

pub fn refactorClass(mut className: ArcStr) -> Result<ArcStr> {
    let mut res: ArcStr;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), literal!("refactorClass"), list![metamodelica::Ref::new(Values::Value::CODE { A: metamodelica::Ref::new(Absyn::CodeNode::C_TYPENAME { path: Parser::stringPath(className)? }) })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::STRING { string: __pa0 }) => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    res = metamodelica::Own::own(__pa0);
    Ok(res)
}

pub fn deleteClass(mut className: ArcStr) -> Result<bool> {
    let mut res: bool;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), literal!("deleteClass"), list![metamodelica::Ref::new(Values::Value::CODE { A: metamodelica::Ref::new(Absyn::CodeNode::C_TYPENAME { path: Parser::stringPath(className)? }) })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::BOOL { boolean: __pa0 }) => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    res = metamodelica::Own::own(__pa0);
    Ok(res)
}

pub fn renameClass(mut oldName: ArcStr, mut newName: ArcStr) -> Result<metamodelica::List<ArcStr>> {
    let mut res: metamodelica::List<ArcStr>;
    let mut res_arr: metamodelica::Ref<Values::Value>;
    (_, res_arr) = CevalScript::cevalInteractiveFunctions2(
        FCore::emptyCache(),
        FGraph::empty(),
        literal!("renameClass"),
        list![
            metamodelica::Ref::new(Values::Value::CODE {
                A: metamodelica::Ref::new(Absyn::CodeNode::C_TYPENAME {
                    path: Parser::stringPath(oldName)?
                })
            }),
            metamodelica::Ref::new(Values::Value::CODE {
                A: metamodelica::Ref::new(Absyn::CodeNode::C_TYPENAME {
                    path: Parser::stringPath(newName)?
                })
            })
        ],
        dummyMsg.clone(),
    )?;
    res = ({
        let mut __acc: metamodelica::List<ArcStr> = metamodelica::nil();
        for mut res_arr_iter in (ValuesUtil::arrayValues(&res_arr)?).into_iter().cloned() {
            let __x = ValuesDump::valString(&(res_arr_iter.clone()))?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    Ok(res)
}

pub fn copyClass(mut className: ArcStr, mut newClassName: ArcStr, mut withIn: ArcStr) -> Result<bool> {
    let mut res: bool;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), literal!("copyClass"), list![metamodelica::Ref::new(Values::Value::CODE { A: metamodelica::Ref::new(Absyn::CodeNode::C_TYPENAME { path: Parser::stringPath(className)? }) }), metamodelica::Ref::new(Values::Value::STRING { string: newClassName }), metamodelica::Ref::new(Values::Value::CODE { A: metamodelica::Ref::new(Absyn::CodeNode::C_TYPENAME { path: Parser::stringPath(withIn)? }) })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::BOOL { boolean: __pa0 }) => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    res = metamodelica::Own::own(__pa0);
    Ok(res)
}

pub fn moveClassToBottom(mut className: ArcStr) -> Result<bool> {
    let mut res: bool;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), literal!("moveClassToBottom"), list![metamodelica::Ref::new(Values::Value::CODE { A: metamodelica::Ref::new(Absyn::CodeNode::C_TYPENAME { path: Parser::stringPath(className)? }) })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::BOOL { boolean: __pa0 }) => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    res = metamodelica::Own::own(__pa0);
    Ok(res)
}

pub fn moveClassToTop(mut className: ArcStr) -> Result<bool> {
    let mut res: bool;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), literal!("moveClassToTop"), list![metamodelica::Ref::new(Values::Value::CODE { A: metamodelica::Ref::new(Absyn::CodeNode::C_TYPENAME { path: Parser::stringPath(className)? }) })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::BOOL { boolean: __pa0 }) => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    res = metamodelica::Own::own(__pa0);
    Ok(res)
}

pub fn moveClass(mut className: ArcStr, mut offset: i32) -> Result<bool> {
    let mut res: bool;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), literal!("moveClass"), list![metamodelica::Ref::new(Values::Value::CODE { A: metamodelica::Ref::new(Absyn::CodeNode::C_TYPENAME { path: Parser::stringPath(className)? }) }), metamodelica::Ref::new(Values::Value::INTEGER { integer: offset })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::BOOL { boolean: __pa0 }) => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    res = metamodelica::Own::own(__pa0);
    Ok(res)
}

pub fn newModel(mut className: ArcStr, mut withinPath: ArcStr) -> Result<bool> {
    let mut res: bool;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), literal!("newModel"), list![metamodelica::Ref::new(Values::Value::CODE { A: metamodelica::Ref::new(Absyn::CodeNode::C_TYPENAME { path: Parser::stringPath(className)? }) }), metamodelica::Ref::new(Values::Value::CODE { A: metamodelica::Ref::new(Absyn::CodeNode::C_TYPENAME { path: Parser::stringPath(withinPath)? }) })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::BOOL { boolean: __pa0 }) => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    res = metamodelica::Own::own(__pa0);
    Ok(res)
}

pub fn createModel(mut className: ArcStr) -> Result<bool> {
    let mut res: bool;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), literal!("createModel"), list![metamodelica::Ref::new(Values::Value::CODE { A: metamodelica::Ref::new(Absyn::CodeNode::C_TYPENAME { path: Parser::stringPath(className)? }) })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::BOOL { boolean: __pa0 }) => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    res = metamodelica::Own::own(__pa0);
    Ok(res)
}

pub fn translateResidualsDAE(mut className: ArcStr, mut fileNamePrefix: ArcStr) -> Result<bool> {
    let mut res: bool;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), literal!("translateResidualsDAE"), list![metamodelica::Ref::new(Values::Value::CODE { A: metamodelica::Ref::new(Absyn::CodeNode::C_TYPENAME { path: Parser::stringPath(className)? }) }), metamodelica::Ref::new(Values::Value::STRING { string: fileNamePrefix })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::BOOL { boolean: __pa0 }) => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    res = metamodelica::Own::own(__pa0);
    Ok(res)
}

pub fn reduceTerms(
    mut className: ArcStr,
    mut startTime: metamodelica::Real,
    mut stopTime: metamodelica::Real,
    mut numberOfIntervals: i32,
    mut tolerance: metamodelica::Real,
    mut method: ArcStr,
    mut fileNamePrefix: ArcStr,
    mut options: ArcStr,
    mut outputFormat: ArcStr,
    mut variableFilter: ArcStr,
    mut cflags: ArcStr,
    mut simflags: ArcStr,
    mut labelstoCancel: ArcStr,
) -> Result<metamodelica::List<ArcStr>> {
    let mut res: metamodelica::List<ArcStr>;
    let mut res_arr: metamodelica::Ref<Values::Value>;
    (_, res_arr) = CevalScript::cevalInteractiveFunctions2(
        FCore::emptyCache(),
        FGraph::empty(),
        literal!("reduceTerms"),
        list![
            metamodelica::Ref::new(Values::Value::CODE {
                A: metamodelica::Ref::new(Absyn::CodeNode::C_TYPENAME {
                    path: Parser::stringPath(className)?
                })
            }),
            metamodelica::Ref::new(Values::Value::REAL { real: startTime }),
            metamodelica::Ref::new(Values::Value::REAL { real: stopTime }),
            metamodelica::Ref::new(Values::Value::INTEGER {
                integer: numberOfIntervals
            }),
            metamodelica::Ref::new(Values::Value::REAL { real: tolerance }),
            metamodelica::Ref::new(Values::Value::STRING { string: method }),
            metamodelica::Ref::new(Values::Value::STRING { string: fileNamePrefix }),
            metamodelica::Ref::new(Values::Value::STRING { string: options }),
            metamodelica::Ref::new(Values::Value::STRING { string: outputFormat }),
            metamodelica::Ref::new(Values::Value::STRING { string: variableFilter }),
            metamodelica::Ref::new(Values::Value::STRING { string: cflags }),
            metamodelica::Ref::new(Values::Value::STRING { string: simflags }),
            metamodelica::Ref::new(Values::Value::STRING { string: labelstoCancel })
        ],
        dummyMsg.clone(),
    )?;
    res = ({
        let mut __acc: metamodelica::List<ArcStr> = metamodelica::nil();
        for mut res_arr_iter in (ValuesUtil::arrayValues(&res_arr)?).into_iter().cloned() {
            let __x = (match &*res_arr_iter.clone() {
                Values::Value::STRING {
                    string: __res_arr_iter_string,
                } => __res_arr_iter_string.clone(),
                _ => return Err("match: no arm matched"),
            });
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    Ok(res)
}

pub fn buildLabel(
    mut className: ArcStr,
    mut startTime: metamodelica::Real,
    mut stopTime: metamodelica::Real,
    mut numberOfIntervals: i32,
    mut tolerance: metamodelica::Real,
    mut method: ArcStr,
    mut fileNamePrefix: ArcStr,
    mut options: ArcStr,
    mut outputFormat: ArcStr,
    mut variableFilter: ArcStr,
    mut cflags: ArcStr,
    mut simflags: ArcStr,
) -> Result<metamodelica::List<ArcStr>> {
    let mut res: metamodelica::List<ArcStr>;
    let mut res_arr: metamodelica::Ref<Values::Value>;
    (_, res_arr) = CevalScript::cevalInteractiveFunctions2(
        FCore::emptyCache(),
        FGraph::empty(),
        literal!("buildLabel"),
        list![
            metamodelica::Ref::new(Values::Value::CODE {
                A: metamodelica::Ref::new(Absyn::CodeNode::C_TYPENAME {
                    path: Parser::stringPath(className)?
                })
            }),
            metamodelica::Ref::new(Values::Value::REAL { real: startTime }),
            metamodelica::Ref::new(Values::Value::REAL { real: stopTime }),
            metamodelica::Ref::new(Values::Value::INTEGER {
                integer: numberOfIntervals
            }),
            metamodelica::Ref::new(Values::Value::REAL { real: tolerance }),
            metamodelica::Ref::new(Values::Value::STRING { string: method }),
            metamodelica::Ref::new(Values::Value::STRING { string: fileNamePrefix }),
            metamodelica::Ref::new(Values::Value::STRING { string: options }),
            metamodelica::Ref::new(Values::Value::STRING { string: outputFormat }),
            metamodelica::Ref::new(Values::Value::STRING { string: variableFilter }),
            metamodelica::Ref::new(Values::Value::STRING { string: cflags }),
            metamodelica::Ref::new(Values::Value::STRING { string: simflags })
        ],
        dummyMsg.clone(),
    )?;
    res = ({
        let mut __acc: metamodelica::List<ArcStr> = metamodelica::nil();
        for mut res_arr_iter in (ValuesUtil::arrayValues(&res_arr)?).into_iter().cloned() {
            let __x = (match &*res_arr_iter.clone() {
                Values::Value::STRING {
                    string: __res_arr_iter_string,
                } => __res_arr_iter_string.clone(),
                _ => return Err("match: no arm matched"),
            });
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    Ok(res)
}

pub fn buildEncryptedPackage(mut className: ArcStr, mut encrypt: bool) -> Result<bool> {
    let mut res: bool;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), literal!("buildEncryptedPackage"), list![metamodelica::Ref::new(Values::Value::CODE { A: metamodelica::Ref::new(Absyn::CodeNode::C_TYPENAME { path: Parser::stringPath(className)? }) }), metamodelica::Ref::new(Values::Value::BOOL { boolean: encrypt })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::BOOL { boolean: __pa0 }) => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    res = metamodelica::Own::own(__pa0);
    Ok(res)
}

pub fn buildModelFMU(
    mut className: ArcStr,
    mut version: ArcStr,
    mut fmuType: ArcStr,
    mut fileNamePrefix: ArcStr,
    mut platforms: metamodelica::List<ArcStr>,
    mut includeResources: bool,
    mut method: ArcStr,
) -> Result<ArcStr> {
    let mut res: ArcStr;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), literal!("buildModelFMU"), list![metamodelica::Ref::new(Values::Value::CODE { A: metamodelica::Ref::new(Absyn::CodeNode::C_TYPENAME { path: Parser::stringPath(className)? }) }), metamodelica::Ref::new(Values::Value::STRING { string: version }), metamodelica::Ref::new(Values::Value::STRING { string: fmuType }), metamodelica::Ref::new(Values::Value::STRING { string: fileNamePrefix }), ValuesMake::makeArray(({
        let mut __acc: metamodelica::List<metamodelica::Ref<Values::Value>> = metamodelica::nil();
        for mut platforms_iter in (platforms).into_iter().cloned() {
            let __x = metamodelica::Ref::new(Values::Value::STRING { string: platforms_iter.clone() });
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    })), metamodelica::Ref::new(Values::Value::BOOL { boolean: includeResources }), metamodelica::Ref::new(Values::Value::STRING { string: method })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::STRING { string: __pa0 }) => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    res = metamodelica::Own::own(__pa0);
    Ok(res)
}

pub fn translateModelFMU(
    mut className: ArcStr,
    mut version: ArcStr,
    mut fmuType: ArcStr,
    mut fileNamePrefix: ArcStr,
    mut platforms: metamodelica::List<ArcStr>,
    mut includeResources: bool,
) -> Result<bool> {
    let mut res: bool;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), literal!("translateModelFMU"), list![metamodelica::Ref::new(Values::Value::CODE { A: metamodelica::Ref::new(Absyn::CodeNode::C_TYPENAME { path: Parser::stringPath(className)? }) }), metamodelica::Ref::new(Values::Value::STRING { string: version }), metamodelica::Ref::new(Values::Value::STRING { string: fmuType }), metamodelica::Ref::new(Values::Value::STRING { string: fileNamePrefix }), ValuesMake::makeArray(({
        let mut __acc: metamodelica::List<metamodelica::Ref<Values::Value>> = metamodelica::nil();
        for mut platforms_iter in (platforms).into_iter().cloned() {
            let __x = metamodelica::Ref::new(Values::Value::STRING { string: platforms_iter.clone() });
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    })), metamodelica::Ref::new(Values::Value::BOOL { boolean: includeResources })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::BOOL { boolean: __pa0 }) => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    res = metamodelica::Own::own(__pa0);
    Ok(res)
}

pub fn importFMUModelDescription(
    mut filename: ArcStr,
    mut workdir: ArcStr,
    mut loglevel: i32,
    mut fullPath: bool,
    mut debugLogging: bool,
    mut generateInputConnectors: bool,
    mut generateOutputConnectors: bool,
) -> Result<ArcStr> {
    let mut res: ArcStr;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), literal!("importFMUModelDescription"), list![metamodelica::Ref::new(Values::Value::STRING { string: filename }), metamodelica::Ref::new(Values::Value::STRING { string: workdir }), metamodelica::Ref::new(Values::Value::INTEGER { integer: loglevel }), metamodelica::Ref::new(Values::Value::BOOL { boolean: fullPath }), metamodelica::Ref::new(Values::Value::BOOL { boolean: debugLogging }), metamodelica::Ref::new(Values::Value::BOOL { boolean: generateInputConnectors }), metamodelica::Ref::new(Values::Value::BOOL { boolean: generateOutputConnectors })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::STRING { string: __pa0 }) => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    res = metamodelica::Own::own(__pa0);
    Ok(res)
}

pub fn importFMU(
    mut filename: ArcStr,
    mut workdir: ArcStr,
    mut loglevel: i32,
    mut fullPath: bool,
    mut debugLogging: bool,
    mut generateInputConnectors: bool,
    mut generateOutputConnectors: bool,
    mut modelName: ArcStr,
) -> Result<ArcStr> {
    let mut res: ArcStr;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), literal!("importFMU"), list![metamodelica::Ref::new(Values::Value::STRING { string: filename }), metamodelica::Ref::new(Values::Value::STRING { string: workdir }), metamodelica::Ref::new(Values::Value::INTEGER { integer: loglevel }), metamodelica::Ref::new(Values::Value::BOOL { boolean: fullPath }), metamodelica::Ref::new(Values::Value::BOOL { boolean: debugLogging }), metamodelica::Ref::new(Values::Value::BOOL { boolean: generateInputConnectors }), metamodelica::Ref::new(Values::Value::BOOL { boolean: generateOutputConnectors }), metamodelica::Ref::new(Values::Value::CODE { A: metamodelica::Ref::new(Absyn::CodeNode::C_TYPENAME { path: Parser::stringPath(modelName)? }) })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::STRING { string: __pa0 }) => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    res = metamodelica::Own::own(__pa0);
    Ok(res)
}

pub fn solveLinearSystem(
    mut A: metamodelica::List<metamodelica::List<metamodelica::Real>>,
    mut B: metamodelica::List<metamodelica::Real>,
) -> Result<(metamodelica::List<metamodelica::Real>, i32)> {
    let mut res1: metamodelica::List<metamodelica::Real>;
    let mut res2: i32;
    let mut res1_arr: metamodelica::Ref<Values::Value>;
    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), literal!("solveLinearSystem"), list![ValuesMake::makeArray(({
        let mut __acc: metamodelica::List<metamodelica::Ref<Values::Value>> = metamodelica::nil();
        for mut A_iter in (A).into_iter().cloned() {
            let __x = ValuesMake::makeArray(({
        let mut __acc: metamodelica::List<metamodelica::Ref<Values::Value>> = metamodelica::nil();
        for mut A_iter_iter in (A_iter.clone()).into_iter().cloned() {
            let __x = metamodelica::Ref::new(Values::Value::REAL { real: A_iter_iter.clone() });
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }));
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    })), ValuesMake::makeArray(({
        let mut __acc: metamodelica::List<metamodelica::Ref<Values::Value>> = metamodelica::nil();
        for mut B_iter in (B).into_iter().cloned() {
            let __x = metamodelica::Ref::new(Values::Value::REAL { real: B_iter.clone() });
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }))], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::TUPLE { valueLst: Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::INTEGER { integer: __pa1 }, tail: Deref @ metamodelica::ListNode::Nil } } }) => (__pa0.clone(), __pa1.clone()),
        _ => return Err("pattern mismatch"),
    } };
    res1_arr = metamodelica::Own::own(__pa0);
    res2 = metamodelica::Own::own(__pa1);
    res1 = ({
        let mut __acc: metamodelica::List<metamodelica::Real> = metamodelica::nil();
        for mut res1_arr_iter in (ValuesUtil::arrayValues(&res1_arr)?).into_iter().cloned() {
            let __x = (match &*res1_arr_iter.clone() {
                Values::Value::REAL {
                    real: __res1_arr_iter_real,
                } => __res1_arr_iter_real.clone(),
                _ => return Err("match: no arm matched"),
            });
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    Ok((res1, res2))
}

pub fn getLoadedLibraries() -> Result<metamodelica::List<metamodelica::List<ArcStr>>> {
    let mut res: metamodelica::List<metamodelica::List<ArcStr>>;
    let mut res_arr: metamodelica::Ref<Values::Value>;
    (_, res_arr) = CevalScript::cevalInteractiveFunctions2(
        FCore::emptyCache(),
        FGraph::empty(),
        literal!("getLoadedLibraries"),
        metamodelica::nil(),
        dummyMsg.clone(),
    )?;
    res = ({
        let mut __acc: metamodelica::List<metamodelica::List<ArcStr>> = metamodelica::nil();
        for mut res_arr_iter in (ValuesUtil::arrayValues(&res_arr)?).into_iter().cloned() {
            let __x = ({
                let mut __acc: metamodelica::List<ArcStr> = metamodelica::nil();
                for mut res_arr_iter_iter in (ValuesUtil::arrayValues(&(res_arr_iter.clone()))?).into_iter().cloned() {
                    let __x = (match &*res_arr_iter_iter.clone() {
                        Values::Value::STRING {
                            string: __res_arr_iter_iter_string,
                        } => __res_arr_iter_iter_string.clone(),
                        _ => return Err("match: no arm matched"),
                    });
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            });
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    Ok(res)
}

pub fn uriToFilename(mut uri: ArcStr) -> Result<ArcStr> {
    let mut res: ArcStr;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), literal!("uriToFilename"), list![metamodelica::Ref::new(Values::Value::STRING { string: uri })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::STRING { string: __pa0 }) => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    res = metamodelica::Own::own(__pa0);
    Ok(res)
}

pub fn realpath(mut name: ArcStr) -> Result<ArcStr> {
    let mut res: ArcStr;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), literal!("realpath"), list![metamodelica::Ref::new(Values::Value::STRING { string: name })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::STRING { string: __pa0 }) => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    res = metamodelica::Own::own(__pa0);
    Ok(res)
}

pub fn rewriteBlockCall(mut className: ArcStr, mut inDefs: ArcStr) -> Result<bool> {
    let mut res: bool;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), literal!("rewriteBlockCall"), list![metamodelica::Ref::new(Values::Value::CODE { A: metamodelica::Ref::new(Absyn::CodeNode::C_TYPENAME { path: Parser::stringPath(className)? }) }), metamodelica::Ref::new(Values::Value::CODE { A: metamodelica::Ref::new(Absyn::CodeNode::C_TYPENAME { path: Parser::stringPath(inDefs)? }) })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::BOOL { boolean: __pa0 }) => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    res = metamodelica::Own::own(__pa0);
    Ok(res)
}

pub fn generateVerificationScenarios(mut path: ArcStr) -> Result<bool> {
    let mut res: bool;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), literal!("generateVerificationScenarios"), list![metamodelica::Ref::new(Values::Value::CODE { A: metamodelica::Ref::new(Absyn::CodeNode::C_TYPENAME { path: Parser::stringPath(path)? }) })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::BOOL { boolean: __pa0 }) => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    res = metamodelica::Own::own(__pa0);
    Ok(res)
}

pub fn inferBindings(mut path: ArcStr) -> Result<bool> {
    let mut res: bool;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), literal!("inferBindings"), list![metamodelica::Ref::new(Values::Value::CODE { A: metamodelica::Ref::new(Absyn::CodeNode::C_TYPENAME { path: Parser::stringPath(path)? }) })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::BOOL { boolean: __pa0 }) => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    res = metamodelica::Own::own(__pa0);
    Ok(res)
}

pub fn exportToFigaro(
    mut path: ArcStr,
    mut directory: ArcStr,
    mut database: ArcStr,
    mut mode: ArcStr,
    mut options: ArcStr,
    mut processor: ArcStr,
) -> Result<bool> {
    let mut res: bool;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), literal!("exportToFigaro"), list![metamodelica::Ref::new(Values::Value::CODE { A: metamodelica::Ref::new(Absyn::CodeNode::C_TYPENAME { path: Parser::stringPath(path)? }) }), metamodelica::Ref::new(Values::Value::STRING { string: directory }), metamodelica::Ref::new(Values::Value::STRING { string: database }), metamodelica::Ref::new(Values::Value::STRING { string: mode }), metamodelica::Ref::new(Values::Value::STRING { string: options }), metamodelica::Ref::new(Values::Value::STRING { string: processor })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::BOOL { boolean: __pa0 }) => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    res = metamodelica::Own::own(__pa0);
    Ok(res)
}

pub fn listFile(mut class_: ArcStr, mut nestedClasses: bool) -> Result<ArcStr> {
    let mut res: ArcStr;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), literal!("listFile"), list![metamodelica::Ref::new(Values::Value::CODE { A: metamodelica::Ref::new(Absyn::CodeNode::C_TYPENAME { path: Parser::stringPath(class_)? }) }), metamodelica::Ref::new(Values::Value::BOOL { boolean: nestedClasses })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::STRING { string: __pa0 }) => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    res = metamodelica::Own::own(__pa0);
    Ok(res)
}

pub fn stringReplace(mut r#str: ArcStr, mut source: ArcStr, mut target: ArcStr) -> Result<ArcStr> {
    let mut res: ArcStr;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), literal!("stringReplace"), list![metamodelica::Ref::new(Values::Value::STRING { string: r#str }), metamodelica::Ref::new(Values::Value::STRING { string: source }), metamodelica::Ref::new(Values::Value::STRING { string: target })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::STRING { string: __pa0 }) => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    res = metamodelica::Own::own(__pa0);
    Ok(res)
}

pub fn stringSplit(mut string: ArcStr, mut token: ArcStr) -> Result<metamodelica::List<ArcStr>> {
    let mut res: metamodelica::List<ArcStr>;
    let mut res_arr: metamodelica::Ref<Values::Value>;
    (_, res_arr) = CevalScript::cevalInteractiveFunctions2(
        FCore::emptyCache(),
        FGraph::empty(),
        literal!("stringSplit"),
        list![
            metamodelica::Ref::new(Values::Value::STRING { string: string }),
            metamodelica::Ref::new(Values::Value::STRING { string: token })
        ],
        dummyMsg.clone(),
    )?;
    res = ({
        let mut __acc: metamodelica::List<ArcStr> = metamodelica::nil();
        for mut res_arr_iter in (ValuesUtil::arrayValues(&res_arr)?).into_iter().cloned() {
            let __x = (match &*res_arr_iter.clone() {
                Values::Value::STRING {
                    string: __res_arr_iter_string,
                } => __res_arr_iter_string.clone(),
                _ => return Err("match: no arm matched"),
            });
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    Ok(res)
}

pub fn strtok(mut string: ArcStr, mut token: ArcStr) -> Result<metamodelica::List<ArcStr>> {
    let mut res: metamodelica::List<ArcStr>;
    let mut res_arr: metamodelica::Ref<Values::Value>;
    (_, res_arr) = CevalScript::cevalInteractiveFunctions2(
        FCore::emptyCache(),
        FGraph::empty(),
        literal!("strtok"),
        list![
            metamodelica::Ref::new(Values::Value::STRING { string: string }),
            metamodelica::Ref::new(Values::Value::STRING { string: token })
        ],
        dummyMsg.clone(),
    )?;
    res = ({
        let mut __acc: metamodelica::List<ArcStr> = metamodelica::nil();
        for mut res_arr_iter in (ValuesUtil::arrayValues(&res_arr)?).into_iter().cloned() {
            let __x = (match &*res_arr_iter.clone() {
                Values::Value::STRING {
                    string: __res_arr_iter_string,
                } => __res_arr_iter_string.clone(),
                _ => return Err("match: no arm matched"),
            });
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    Ok(res)
}

pub fn listVariables() -> Result<metamodelica::List<ArcStr>> {
    let mut res: metamodelica::List<ArcStr>;
    let mut res_arr: metamodelica::Ref<Values::Value>;
    (_, res_arr) = CevalScript::cevalInteractiveFunctions2(
        FCore::emptyCache(),
        FGraph::empty(),
        literal!("listVariables"),
        metamodelica::nil(),
        dummyMsg.clone(),
    )?;
    res = ({
        let mut __acc: metamodelica::List<ArcStr> = metamodelica::nil();
        for mut res_arr_iter in (ValuesUtil::arrayValues(&res_arr)?).into_iter().cloned() {
            let __x = ValuesDump::valString(&(res_arr_iter.clone()))?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    Ok(res)
}

pub fn getDerivedUnits(mut baseUnit: ArcStr) -> Result<metamodelica::List<ArcStr>> {
    let mut res: metamodelica::List<ArcStr>;
    let mut res_arr: metamodelica::Ref<Values::Value>;
    (_, res_arr) = CevalScript::cevalInteractiveFunctions2(
        FCore::emptyCache(),
        FGraph::empty(),
        literal!("getDerivedUnits"),
        list![metamodelica::Ref::new(Values::Value::STRING { string: baseUnit })],
        dummyMsg.clone(),
    )?;
    res = ({
        let mut __acc: metamodelica::List<ArcStr> = metamodelica::nil();
        for mut res_arr_iter in (ValuesUtil::arrayValues(&res_arr)?).into_iter().cloned() {
            let __x = (match &*res_arr_iter.clone() {
                Values::Value::STRING {
                    string: __res_arr_iter_string,
                } => __res_arr_iter_string.clone(),
                _ => return Err("match: no arm matched"),
            });
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    Ok(res)
}

pub fn convertUnits(mut s1: ArcStr, mut s2: ArcStr) -> Result<(bool, metamodelica::Real, metamodelica::Real)> {
    let mut res1: bool;
    let mut res2: metamodelica::Real;
    let mut res3: metamodelica::Real;
    let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), literal!("convertUnits"), list![metamodelica::Ref::new(Values::Value::STRING { string: s1 }), metamodelica::Ref::new(Values::Value::STRING { string: s2 })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::TUPLE { valueLst: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::BOOL { boolean: __pa0 }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::REAL { real: __pa1 }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::REAL { real: __pa2 }, tail: Deref @ metamodelica::ListNode::Nil } } } }) => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
        _ => return Err("pattern mismatch"),
    } };
    res1 = metamodelica::Own::own(__pa0);
    res2 = metamodelica::Own::own(__pa1);
    res3 = metamodelica::Own::own(__pa2);
    Ok((res1, res2, res3))
}

pub fn dumpXMLDAE(
    mut className: ArcStr,
    mut translationLevel: ArcStr,
    mut addOriginalAdjacencyMatrix: bool,
    mut addSolvingInfo: bool,
    mut addMathMLCode: bool,
    mut dumpResiduals: bool,
    mut fileNamePrefix: ArcStr,
    mut rewriteRulesFile: ArcStr,
) -> Result<(bool, ArcStr)> {
    let mut res1: bool;
    let mut res2: ArcStr;
    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), literal!("dumpXMLDAE"), list![metamodelica::Ref::new(Values::Value::CODE { A: metamodelica::Ref::new(Absyn::CodeNode::C_TYPENAME { path: Parser::stringPath(className)? }) }), metamodelica::Ref::new(Values::Value::STRING { string: translationLevel }), metamodelica::Ref::new(Values::Value::BOOL { boolean: addOriginalAdjacencyMatrix }), metamodelica::Ref::new(Values::Value::BOOL { boolean: addSolvingInfo }), metamodelica::Ref::new(Values::Value::BOOL { boolean: addMathMLCode }), metamodelica::Ref::new(Values::Value::BOOL { boolean: dumpResiduals }), metamodelica::Ref::new(Values::Value::STRING { string: fileNamePrefix }), metamodelica::Ref::new(Values::Value::STRING { string: rewriteRulesFile })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::TUPLE { valueLst: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::BOOL { boolean: __pa0 }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: __pa1 }, tail: Deref @ metamodelica::ListNode::Nil } } }) => (__pa0.clone(), __pa1.clone()),
        _ => return Err("pattern mismatch"),
    } };
    res1 = metamodelica::Own::own(__pa0);
    res2 = metamodelica::Own::own(__pa1);
    Ok((res1, res2))
}

pub fn translateGraphics(mut className: ArcStr) -> Result<ArcStr> {
    let mut res: ArcStr;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), literal!("translateGraphics"), list![metamodelica::Ref::new(Values::Value::CODE { A: metamodelica::Ref::new(Absyn::CodeNode::C_TYPENAME { path: Parser::stringPath(className)? }) })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::STRING { string: __pa0 }) => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    res = metamodelica::Own::own(__pa0);
    Ok(res)
}

pub fn save(mut className: ArcStr) -> Result<bool> {
    let mut res: bool;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), literal!("save"), list![metamodelica::Ref::new(Values::Value::CODE { A: metamodelica::Ref::new(Absyn::CodeNode::C_TYPENAME { path: Parser::stringPath(className)? }) })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::BOOL { boolean: __pa0 }) => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    res = metamodelica::Own::own(__pa0);
    Ok(res)
}

pub fn saveTotalModelDebug(
    mut filename: ArcStr,
    mut className: ArcStr,
    mut stripAnnotations: bool,
    mut stripComments: bool,
    mut obfuscate: bool,
) -> Result<bool> {
    let mut res: bool;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), literal!("saveTotalModelDebug"), list![metamodelica::Ref::new(Values::Value::STRING { string: filename }), metamodelica::Ref::new(Values::Value::CODE { A: metamodelica::Ref::new(Absyn::CodeNode::C_TYPENAME { path: Parser::stringPath(className)? }) }), metamodelica::Ref::new(Values::Value::BOOL { boolean: stripAnnotations }), metamodelica::Ref::new(Values::Value::BOOL { boolean: stripComments }), metamodelica::Ref::new(Values::Value::BOOL { boolean: obfuscate })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::BOOL { boolean: __pa0 }) => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    res = metamodelica::Own::own(__pa0);
    Ok(res)
}

pub fn getTotalModel(
    mut className: ArcStr,
    mut stripAnnotations: bool,
    mut stripComments: bool,
    mut obfuscate: bool,
) -> Result<ArcStr> {
    let mut res: ArcStr;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), literal!("getTotalModel"), list![metamodelica::Ref::new(Values::Value::CODE { A: metamodelica::Ref::new(Absyn::CodeNode::C_TYPENAME { path: Parser::stringPath(className)? }) }), metamodelica::Ref::new(Values::Value::BOOL { boolean: stripAnnotations }), metamodelica::Ref::new(Values::Value::BOOL { boolean: stripComments }), metamodelica::Ref::new(Values::Value::BOOL { boolean: obfuscate })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::STRING { string: __pa0 }) => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    res = metamodelica::Own::own(__pa0);
    Ok(res)
}

pub fn previous_saveTotalModel(
    mut fileName: ArcStr,
    mut className: ArcStr,
    mut stripAnnotations: bool,
    mut stripComments: bool,
    mut obfuscate: bool,
) -> Result<bool> {
    let mut res: bool;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), literal!("previous_saveTotalModel"), list![metamodelica::Ref::new(Values::Value::STRING { string: fileName }), metamodelica::Ref::new(Values::Value::CODE { A: metamodelica::Ref::new(Absyn::CodeNode::C_TYPENAME { path: Parser::stringPath(className)? }) }), metamodelica::Ref::new(Values::Value::BOOL { boolean: stripAnnotations }), metamodelica::Ref::new(Values::Value::BOOL { boolean: stripComments }), metamodelica::Ref::new(Values::Value::BOOL { boolean: obfuscate })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::BOOL { boolean: __pa0 }) => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    res = metamodelica::Own::own(__pa0);
    Ok(res)
}

pub fn saveTotalModel(
    mut fileName: ArcStr,
    mut className: ArcStr,
    mut stripAnnotations: bool,
    mut stripComments: bool,
    mut obfuscate: bool,
) -> Result<bool> {
    let mut res: bool;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), literal!("saveTotalModel"), list![metamodelica::Ref::new(Values::Value::STRING { string: fileName }), metamodelica::Ref::new(Values::Value::CODE { A: metamodelica::Ref::new(Absyn::CodeNode::C_TYPENAME { path: Parser::stringPath(className)? }) }), metamodelica::Ref::new(Values::Value::BOOL { boolean: stripAnnotations }), metamodelica::Ref::new(Values::Value::BOOL { boolean: stripComments }), metamodelica::Ref::new(Values::Value::BOOL { boolean: obfuscate })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::BOOL { boolean: __pa0 }) => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    res = metamodelica::Own::own(__pa0);
    Ok(res)
}

pub fn saveModel(mut fileName: ArcStr, mut className: ArcStr) -> Result<bool> {
    let mut res: bool;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), literal!("saveModel"), list![metamodelica::Ref::new(Values::Value::STRING { string: fileName }), metamodelica::Ref::new(Values::Value::CODE { A: metamodelica::Ref::new(Absyn::CodeNode::C_TYPENAME { path: Parser::stringPath(className)? }) })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::BOOL { boolean: __pa0 }) => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    res = metamodelica::Own::own(__pa0);
    Ok(res)
}

pub fn deleteFile(mut fileName: ArcStr) -> Result<bool> {
    let mut res: bool;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), literal!("deleteFile"), list![metamodelica::Ref::new(Values::Value::STRING { string: fileName })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::BOOL { boolean: __pa0 }) => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    res = metamodelica::Own::own(__pa0);
    Ok(res)
}

pub fn loadModel(
    mut className: ArcStr,
    mut priorityVersion: metamodelica::List<ArcStr>,
    mut notify: bool,
    mut languageStandard: ArcStr,
    mut requireExactVersion: bool,
) -> Result<bool> {
    let mut res: bool;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), literal!("loadModel"), list![metamodelica::Ref::new(Values::Value::CODE { A: metamodelica::Ref::new(Absyn::CodeNode::C_TYPENAME { path: Parser::stringPath(className)? }) }), ValuesMake::makeArray(({
        let mut __acc: metamodelica::List<metamodelica::Ref<Values::Value>> = metamodelica::nil();
        for mut priorityVersion_iter in (priorityVersion).into_iter().cloned() {
            let __x = metamodelica::Ref::new(Values::Value::STRING { string: priorityVersion_iter.clone() });
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    })), metamodelica::Ref::new(Values::Value::BOOL { boolean: notify }), metamodelica::Ref::new(Values::Value::STRING { string: languageStandard }), metamodelica::Ref::new(Values::Value::BOOL { boolean: requireExactVersion })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::BOOL { boolean: __pa0 }) => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    res = metamodelica::Own::own(__pa0);
    Ok(res)
}

pub fn generateCode(mut className: ArcStr) -> Result<bool> {
    let mut res: bool;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), literal!("generateCode"), list![metamodelica::Ref::new(Values::Value::CODE { A: metamodelica::Ref::new(Absyn::CodeNode::C_TYPENAME { path: Parser::stringPath(className)? }) })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::BOOL { boolean: __pa0 }) => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    res = metamodelica::Own::own(__pa0);
    Ok(res)
}

pub fn instantiateModel(mut className: ArcStr) -> Result<ArcStr> {
    let mut res: ArcStr;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), literal!("instantiateModel"), list![metamodelica::Ref::new(Values::Value::CODE { A: metamodelica::Ref::new(Absyn::CodeNode::C_TYPENAME { path: Parser::stringPath(className)? }) })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::STRING { string: __pa0 }) => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    res = metamodelica::Own::own(__pa0);
    Ok(res)
}

pub fn checkAllModelsRecursive(mut className: ArcStr, mut checkProtected: bool) -> Result<ArcStr> {
    let mut res: ArcStr;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), literal!("checkAllModelsRecursive"), list![metamodelica::Ref::new(Values::Value::CODE { A: metamodelica::Ref::new(Absyn::CodeNode::C_TYPENAME { path: Parser::stringPath(className)? }) }), metamodelica::Ref::new(Values::Value::BOOL { boolean: checkProtected })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::STRING { string: __pa0 }) => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    res = metamodelica::Own::own(__pa0);
    Ok(res)
}

pub fn checkModel(mut className: ArcStr) -> Result<ArcStr> {
    let mut res: ArcStr;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), literal!("checkModel"), list![metamodelica::Ref::new(Values::Value::CODE { A: metamodelica::Ref::new(Absyn::CodeNode::C_TYPENAME { path: Parser::stringPath(className)? }) })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::STRING { string: __pa0 }) => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    res = metamodelica::Own::own(__pa0);
    Ok(res)
}

pub fn remove(mut path: ArcStr) -> Result<bool> {
    let mut res: bool;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), literal!("remove"), list![metamodelica::Ref::new(Values::Value::STRING { string: path })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::BOOL { boolean: __pa0 }) => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    res = metamodelica::Own::own(__pa0);
    Ok(res)
}

pub fn copy(mut source: ArcStr, mut destination: ArcStr) -> Result<bool> {
    let mut res: bool;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), literal!("copy"), list![metamodelica::Ref::new(Values::Value::STRING { string: source }), metamodelica::Ref::new(Values::Value::STRING { string: destination })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::BOOL { boolean: __pa0 }) => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    res = metamodelica::Own::own(__pa0);
    Ok(res)
}

pub fn mkdir(mut newDirectory: ArcStr) -> Result<bool> {
    let mut res: bool;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), literal!("mkdir"), list![metamodelica::Ref::new(Values::Value::STRING { string: newDirectory })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::BOOL { boolean: __pa0 }) => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    res = metamodelica::Own::own(__pa0);
    Ok(res)
}

pub fn cd(mut newWorkingDirectory: ArcStr) -> Result<ArcStr> {
    let mut res: ArcStr;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), literal!("cd"), list![metamodelica::Ref::new(Values::Value::STRING { string: newWorkingDirectory })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::STRING { string: __pa0 }) => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    res = metamodelica::Own::own(__pa0);
    Ok(res)
}

pub fn getLanguageStandard() -> Result<ArcStr> {
    let mut res: ArcStr;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), literal!("getLanguageStandard"), metamodelica::nil(), dummyMsg.clone())?) {
        (_, Deref @ Values::Value::STRING { string: __pa0 }) => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    res = metamodelica::Own::own(__pa0);
    Ok(res)
}

pub fn getOrderConnections() -> Result<bool> {
    let mut res: bool;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), literal!("getOrderConnections"), metamodelica::nil(), dummyMsg.clone())?) {
        (_, Deref @ Values::Value::BOOL { boolean: __pa0 }) => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    res = metamodelica::Own::own(__pa0);
    Ok(res)
}

pub fn getShowAnnotations() -> Result<bool> {
    let mut res: bool;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), literal!("getShowAnnotations"), metamodelica::nil(), dummyMsg.clone())?) {
        (_, Deref @ Values::Value::BOOL { boolean: __pa0 }) => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    res = metamodelica::Own::own(__pa0);
    Ok(res)
}

pub fn setShowAnnotations(mut show: bool) -> Result<bool> {
    let mut res: bool;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), literal!("setShowAnnotations"), list![metamodelica::Ref::new(Values::Value::BOOL { boolean: show })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::BOOL { boolean: __pa0 }) => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    res = metamodelica::Own::own(__pa0);
    Ok(res)
}

pub fn getDefaultOpenCLDevice() -> Result<i32> {
    let mut res: i32;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), literal!("getDefaultOpenCLDevice"), metamodelica::nil(), dummyMsg.clone())?) {
        (_, Deref @ Values::Value::INTEGER { integer: __pa0 }) => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    res = metamodelica::Own::own(__pa0);
    Ok(res)
}

pub fn getVectorizationLimit() -> Result<i32> {
    let mut res: i32;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), literal!("getVectorizationLimit"), metamodelica::nil(), dummyMsg.clone())?) {
        (_, Deref @ Values::Value::INTEGER { integer: __pa0 }) => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    res = metamodelica::Own::own(__pa0);
    Ok(res)
}

pub fn setNoSimplify(mut noSimplify: bool) -> Result<bool> {
    let mut res: bool;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), literal!("setNoSimplify"), list![metamodelica::Ref::new(Values::Value::BOOL { boolean: noSimplify })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::BOOL { boolean: __pa0 }) => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    res = metamodelica::Own::own(__pa0);
    Ok(res)
}

pub fn getNoSimplify() -> Result<bool> {
    let mut res: bool;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), literal!("getNoSimplify"), metamodelica::nil(), dummyMsg.clone())?) {
        (_, Deref @ Values::Value::BOOL { boolean: __pa0 }) => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    res = metamodelica::Own::own(__pa0);
    Ok(res)
}

pub fn getAnnotationVersion() -> Result<ArcStr> {
    let mut res: ArcStr;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), literal!("getAnnotationVersion"), metamodelica::nil(), dummyMsg.clone())?) {
        (_, Deref @ Values::Value::STRING { string: __pa0 }) => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    res = metamodelica::Own::own(__pa0);
    Ok(res)
}

pub fn echo(mut setEcho: bool) -> Result<bool> {
    let mut res: bool;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), literal!("echo"), list![metamodelica::Ref::new(Values::Value::BOOL { boolean: setEcho })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::BOOL { boolean: __pa0 }) => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    res = metamodelica::Own::own(__pa0);
    Ok(res)
}

pub fn runScript(mut fileName: ArcStr) -> Result<ArcStr> {
    let mut res: ArcStr;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), literal!("runScript"), list![metamodelica::Ref::new(Values::Value::STRING { string: fileName })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::STRING { string: __pa0 }) => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    res = metamodelica::Own::own(__pa0);
    Ok(res)
}

pub fn clearMessages() -> Result<bool> {
    let mut res: bool;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), literal!("clearMessages"), metamodelica::nil(), dummyMsg.clone())?) {
        (_, Deref @ Values::Value::BOOL { boolean: __pa0 }) => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    res = metamodelica::Own::own(__pa0);
    Ok(res)
}

pub fn countMessages() -> Result<(i32, i32, i32)> {
    let mut res1: i32;
    let mut res2: i32;
    let mut res3: i32;
    let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), literal!("countMessages"), metamodelica::nil(), dummyMsg.clone())?) {
        (_, Deref @ Values::Value::TUPLE { valueLst: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::INTEGER { integer: __pa0 }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::INTEGER { integer: __pa1 }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::INTEGER { integer: __pa2 }, tail: Deref @ metamodelica::ListNode::Nil } } } }) => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
        _ => return Err("pattern mismatch"),
    } };
    res1 = metamodelica::Own::own(__pa0);
    res2 = metamodelica::Own::own(__pa1);
    res3 = metamodelica::Own::own(__pa2);
    Ok((res1, res2, res3))
}

pub fn getErrorString(mut warningsAsErrors: bool) -> Result<ArcStr> {
    let mut res: ArcStr;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), literal!("getErrorString"), list![metamodelica::Ref::new(Values::Value::BOOL { boolean: warningsAsErrors })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::STRING { string: __pa0 }) => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    res = metamodelica::Own::own(__pa0);
    Ok(res)
}

pub fn alarm(mut seconds: i32) -> Result<i32> {
    let mut res: i32;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), literal!("alarm"), list![metamodelica::Ref::new(Values::Value::INTEGER { integer: seconds })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::INTEGER { integer: __pa0 }) => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    res = metamodelica::Own::own(__pa0);
    Ok(res)
}

pub fn compareFiles(mut file1: ArcStr, mut file2: ArcStr) -> Result<bool> {
    let mut res: bool;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), literal!("compareFiles"), list![metamodelica::Ref::new(Values::Value::STRING { string: file1 }), metamodelica::Ref::new(Values::Value::STRING { string: file2 })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::BOOL { boolean: __pa0 }) => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    res = metamodelica::Own::own(__pa0);
    Ok(res)
}

pub fn compareFilesAndMove(mut newFile: ArcStr, mut oldFile: ArcStr) -> Result<bool> {
    let mut res: bool;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), literal!("compareFilesAndMove"), list![metamodelica::Ref::new(Values::Value::STRING { string: newFile }), metamodelica::Ref::new(Values::Value::STRING { string: oldFile })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::BOOL { boolean: __pa0 }) => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    res = metamodelica::Own::own(__pa0);
    Ok(res)
}

pub fn writeFile(mut fileName: ArcStr, mut data: ArcStr, mut append: bool) -> Result<bool> {
    let mut res: bool;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), literal!("writeFile"), list![metamodelica::Ref::new(Values::Value::STRING { string: fileName }), metamodelica::Ref::new(Values::Value::STRING { string: data }), metamodelica::Ref::new(Values::Value::BOOL { boolean: append })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::BOOL { boolean: __pa0 }) => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    res = metamodelica::Own::own(__pa0);
    Ok(res)
}

pub fn readFile(mut fileName: ArcStr) -> Result<ArcStr> {
    let mut res: ArcStr;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), literal!("readFile"), list![metamodelica::Ref::new(Values::Value::STRING { string: fileName })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::STRING { string: __pa0 }) => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    res = metamodelica::Own::own(__pa0);
    Ok(res)
}

pub fn stat(mut fileName: ArcStr) -> Result<(bool, metamodelica::Real, metamodelica::Real)> {
    let mut res1: bool;
    let mut res2: metamodelica::Real;
    let mut res3: metamodelica::Real;
    let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), literal!("stat"), list![metamodelica::Ref::new(Values::Value::STRING { string: fileName })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::TUPLE { valueLst: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::BOOL { boolean: __pa0 }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::REAL { real: __pa1 }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::REAL { real: __pa2 }, tail: Deref @ metamodelica::ListNode::Nil } } } }) => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
        _ => return Err("pattern mismatch"),
    } };
    res1 = metamodelica::Own::own(__pa0);
    res2 = metamodelica::Own::own(__pa1);
    res3 = metamodelica::Own::own(__pa2);
    Ok((res1, res2, res3))
}

pub fn directoryExists(mut dirName: ArcStr) -> Result<bool> {
    let mut res: bool;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), literal!("directoryExists"), list![metamodelica::Ref::new(Values::Value::STRING { string: dirName })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::BOOL { boolean: __pa0 }) => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    res = metamodelica::Own::own(__pa0);
    Ok(res)
}

pub fn regularFileExists(mut fileName: ArcStr) -> Result<bool> {
    let mut res: bool;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), literal!("regularFileExists"), list![metamodelica::Ref::new(Values::Value::STRING { string: fileName })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::BOOL { boolean: __pa0 }) => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    res = metamodelica::Own::own(__pa0);
    Ok(res)
}

pub fn getVersion(mut cl: ArcStr) -> Result<ArcStr> {
    let mut res: ArcStr;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), literal!("getVersion"), list![metamodelica::Ref::new(Values::Value::CODE { A: metamodelica::Ref::new(Absyn::CodeNode::C_TYPENAME { path: Parser::stringPath(cl)? }) })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::STRING { string: __pa0 }) => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    res = metamodelica::Own::own(__pa0);
    Ok(res)
}

pub fn clearCommandLineOptions() -> Result<bool> {
    let mut res: bool;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), literal!("clearCommandLineOptions"), metamodelica::nil(), dummyMsg.clone())?) {
        (_, Deref @ Values::Value::BOOL { boolean: __pa0 }) => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    res = metamodelica::Own::own(__pa0);
    Ok(res)
}

pub fn getConfigFlagValidOptions(
    mut flag: ArcStr,
) -> Result<(metamodelica::List<ArcStr>, ArcStr, metamodelica::List<ArcStr>)> {
    let mut res1: metamodelica::List<ArcStr>;
    let mut res2: ArcStr;
    let mut res3: metamodelica::List<ArcStr>;
    let mut res1_arr: metamodelica::Ref<Values::Value>;
    let mut res3_arr: metamodelica::Ref<Values::Value>;
    let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), literal!("getConfigFlagValidOptions"), list![metamodelica::Ref::new(Values::Value::STRING { string: flag })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::TUPLE { valueLst: Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: __pa1 }, tail: Deref @ metamodelica::ListNode::Cons { head: __pa2, tail: Deref @ metamodelica::ListNode::Nil } } } }) => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
        _ => return Err("pattern mismatch"),
    } };
    res1_arr = metamodelica::Own::own(__pa0);
    res2 = metamodelica::Own::own(__pa1);
    res3_arr = metamodelica::Own::own(__pa2);
    res1 = ({
        let mut __acc: metamodelica::List<ArcStr> = metamodelica::nil();
        for mut res1_arr_iter in (ValuesUtil::arrayValues(&res1_arr)?).into_iter().cloned() {
            let __x = (match &*res1_arr_iter.clone() {
                Values::Value::STRING {
                    string: __res1_arr_iter_string,
                } => __res1_arr_iter_string.clone(),
                _ => return Err("match: no arm matched"),
            });
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    res3 = ({
        let mut __acc: metamodelica::List<ArcStr> = metamodelica::nil();
        for mut res3_arr_iter in (ValuesUtil::arrayValues(&res3_arr)?).into_iter().cloned() {
            let __x = (match &*res3_arr_iter.clone() {
                Values::Value::STRING {
                    string: __res3_arr_iter_string,
                } => __res3_arr_iter_string.clone(),
                _ => return Err("match: no arm matched"),
            });
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    Ok((res1, res2, res3))
}

pub fn getCommandLineOptions() -> Result<metamodelica::List<ArcStr>> {
    let mut res: metamodelica::List<ArcStr>;
    let mut res_arr: metamodelica::Ref<Values::Value>;
    (_, res_arr) = CevalScript::cevalInteractiveFunctions2(
        FCore::emptyCache(),
        FGraph::empty(),
        literal!("getCommandLineOptions"),
        metamodelica::nil(),
        dummyMsg.clone(),
    )?;
    res = ({
        let mut __acc: metamodelica::List<ArcStr> = metamodelica::nil();
        for mut res_arr_iter in (ValuesUtil::arrayValues(&res_arr)?).into_iter().cloned() {
            let __x = (match &*res_arr_iter.clone() {
                Values::Value::STRING {
                    string: __res_arr_iter_string,
                } => __res_arr_iter_string.clone(),
                _ => return Err("match: no arm matched"),
            });
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    Ok(res)
}

pub fn setCommandLineOptions(mut options: ArcStr) -> Result<bool> {
    let mut res: bool;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), literal!("setCommandLineOptions"), list![metamodelica::Ref::new(Values::Value::STRING { string: options })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::BOOL { boolean: __pa0 }) => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    res = metamodelica::Own::own(__pa0);
    Ok(res)
}

pub fn getAvailableTearingMethods() -> Result<(metamodelica::List<ArcStr>, metamodelica::List<ArcStr>)> {
    let mut res1: metamodelica::List<ArcStr>;
    let mut res2: metamodelica::List<ArcStr>;
    let mut res1_arr: metamodelica::Ref<Values::Value>;
    let mut res2_arr: metamodelica::Ref<Values::Value>;
    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), literal!("getAvailableTearingMethods"), metamodelica::nil(), dummyMsg.clone())?) {
        (_, Deref @ Values::Value::TUPLE { valueLst: Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: Deref @ metamodelica::ListNode::Cons { head: __pa1, tail: Deref @ metamodelica::ListNode::Nil } } }) => (__pa0.clone(), __pa1.clone()),
        _ => return Err("pattern mismatch"),
    } };
    res1_arr = metamodelica::Own::own(__pa0);
    res2_arr = metamodelica::Own::own(__pa1);
    res1 = ({
        let mut __acc: metamodelica::List<ArcStr> = metamodelica::nil();
        for mut res1_arr_iter in (ValuesUtil::arrayValues(&res1_arr)?).into_iter().cloned() {
            let __x = (match &*res1_arr_iter.clone() {
                Values::Value::STRING {
                    string: __res1_arr_iter_string,
                } => __res1_arr_iter_string.clone(),
                _ => return Err("match: no arm matched"),
            });
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    res2 = ({
        let mut __acc: metamodelica::List<ArcStr> = metamodelica::nil();
        for mut res2_arr_iter in (ValuesUtil::arrayValues(&res2_arr)?).into_iter().cloned() {
            let __x = (match &*res2_arr_iter.clone() {
                Values::Value::STRING {
                    string: __res2_arr_iter_string,
                } => __res2_arr_iter_string.clone(),
                _ => return Err("match: no arm matched"),
            });
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    Ok((res1, res2))
}

pub fn getTearingMethod() -> Result<ArcStr> {
    let mut res: ArcStr;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), literal!("getTearingMethod"), metamodelica::nil(), dummyMsg.clone())?) {
        (_, Deref @ Values::Value::STRING { string: __pa0 }) => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    res = metamodelica::Own::own(__pa0);
    Ok(res)
}

pub fn getAvailableIndexReductionMethods() -> Result<(metamodelica::List<ArcStr>, metamodelica::List<ArcStr>)> {
    let mut res1: metamodelica::List<ArcStr>;
    let mut res2: metamodelica::List<ArcStr>;
    let mut res1_arr: metamodelica::Ref<Values::Value>;
    let mut res2_arr: metamodelica::Ref<Values::Value>;
    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), literal!("getAvailableIndexReductionMethods"), metamodelica::nil(), dummyMsg.clone())?) {
        (_, Deref @ Values::Value::TUPLE { valueLst: Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: Deref @ metamodelica::ListNode::Cons { head: __pa1, tail: Deref @ metamodelica::ListNode::Nil } } }) => (__pa0.clone(), __pa1.clone()),
        _ => return Err("pattern mismatch"),
    } };
    res1_arr = metamodelica::Own::own(__pa0);
    res2_arr = metamodelica::Own::own(__pa1);
    res1 = ({
        let mut __acc: metamodelica::List<ArcStr> = metamodelica::nil();
        for mut res1_arr_iter in (ValuesUtil::arrayValues(&res1_arr)?).into_iter().cloned() {
            let __x = (match &*res1_arr_iter.clone() {
                Values::Value::STRING {
                    string: __res1_arr_iter_string,
                } => __res1_arr_iter_string.clone(),
                _ => return Err("match: no arm matched"),
            });
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    res2 = ({
        let mut __acc: metamodelica::List<ArcStr> = metamodelica::nil();
        for mut res2_arr_iter in (ValuesUtil::arrayValues(&res2_arr)?).into_iter().cloned() {
            let __x = (match &*res2_arr_iter.clone() {
                Values::Value::STRING {
                    string: __res2_arr_iter_string,
                } => __res2_arr_iter_string.clone(),
                _ => return Err("match: no arm matched"),
            });
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    Ok((res1, res2))
}

pub fn getIndexReductionMethod() -> Result<ArcStr> {
    let mut res: ArcStr;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), literal!("getIndexReductionMethod"), metamodelica::nil(), dummyMsg.clone())?) {
        (_, Deref @ Values::Value::STRING { string: __pa0 }) => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    res = metamodelica::Own::own(__pa0);
    Ok(res)
}

pub fn getAvailableMatchingAlgorithms() -> Result<(metamodelica::List<ArcStr>, metamodelica::List<ArcStr>)> {
    let mut res1: metamodelica::List<ArcStr>;
    let mut res2: metamodelica::List<ArcStr>;
    let mut res1_arr: metamodelica::Ref<Values::Value>;
    let mut res2_arr: metamodelica::Ref<Values::Value>;
    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), literal!("getAvailableMatchingAlgorithms"), metamodelica::nil(), dummyMsg.clone())?) {
        (_, Deref @ Values::Value::TUPLE { valueLst: Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: Deref @ metamodelica::ListNode::Cons { head: __pa1, tail: Deref @ metamodelica::ListNode::Nil } } }) => (__pa0.clone(), __pa1.clone()),
        _ => return Err("pattern mismatch"),
    } };
    res1_arr = metamodelica::Own::own(__pa0);
    res2_arr = metamodelica::Own::own(__pa1);
    res1 = ({
        let mut __acc: metamodelica::List<ArcStr> = metamodelica::nil();
        for mut res1_arr_iter in (ValuesUtil::arrayValues(&res1_arr)?).into_iter().cloned() {
            let __x = (match &*res1_arr_iter.clone() {
                Values::Value::STRING {
                    string: __res1_arr_iter_string,
                } => __res1_arr_iter_string.clone(),
                _ => return Err("match: no arm matched"),
            });
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    res2 = ({
        let mut __acc: metamodelica::List<ArcStr> = metamodelica::nil();
        for mut res2_arr_iter in (ValuesUtil::arrayValues(&res2_arr)?).into_iter().cloned() {
            let __x = (match &*res2_arr_iter.clone() {
                Values::Value::STRING {
                    string: __res2_arr_iter_string,
                } => __res2_arr_iter_string.clone(),
                _ => return Err("match: no arm matched"),
            });
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    Ok((res1, res2))
}

pub fn getMatchingAlgorithm() -> Result<ArcStr> {
    let mut res: ArcStr;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), literal!("getMatchingAlgorithm"), metamodelica::nil(), dummyMsg.clone())?) {
        (_, Deref @ Values::Value::STRING { string: __pa0 }) => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    res = metamodelica::Own::own(__pa0);
    Ok(res)
}

pub fn clearDebugFlags() -> Result<bool> {
    let mut res: bool;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), literal!("clearDebugFlags"), metamodelica::nil(), dummyMsg.clone())?) {
        (_, Deref @ Values::Value::BOOL { boolean: __pa0 }) => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    res = metamodelica::Own::own(__pa0);
    Ok(res)
}

pub fn disableNewInstantiation() -> Result<bool> {
    let mut res: bool;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), literal!("disableNewInstantiation"), metamodelica::nil(), dummyMsg.clone())?) {
        (_, Deref @ Values::Value::BOOL { boolean: __pa0 }) => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    res = metamodelica::Own::own(__pa0);
    Ok(res)
}

pub fn enableNewInstantiation() -> Result<bool> {
    let mut res: bool;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), literal!("enableNewInstantiation"), metamodelica::nil(), dummyMsg.clone())?) {
        (_, Deref @ Values::Value::BOOL { boolean: __pa0 }) => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    res = metamodelica::Own::own(__pa0);
    Ok(res)
}

pub fn setCompilerFlags(mut compilerFlags: ArcStr) -> Result<bool> {
    let mut res: bool;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), literal!("setCompilerFlags"), list![metamodelica::Ref::new(Values::Value::STRING { string: compilerFlags })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::BOOL { boolean: __pa0 }) => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    res = metamodelica::Own::own(__pa0);
    Ok(res)
}

pub fn getHomeDirectoryPath() -> Result<ArcStr> {
    let mut res: ArcStr;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), literal!("getHomeDirectoryPath"), metamodelica::nil(), dummyMsg.clone())?) {
        (_, Deref @ Values::Value::STRING { string: __pa0 }) => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    res = metamodelica::Own::own(__pa0);
    Ok(res)
}

pub fn getModelicaPath() -> Result<ArcStr> {
    let mut res: ArcStr;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), literal!("getModelicaPath"), metamodelica::nil(), dummyMsg.clone())?) {
        (_, Deref @ Values::Value::STRING { string: __pa0 }) => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    res = metamodelica::Own::own(__pa0);
    Ok(res)
}

pub fn setModelicaPath(mut modelicaPath: ArcStr) -> Result<bool> {
    let mut res: bool;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), literal!("setModelicaPath"), list![metamodelica::Ref::new(Values::Value::STRING { string: modelicaPath })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::BOOL { boolean: __pa0 }) => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    res = metamodelica::Own::own(__pa0);
    Ok(res)
}

pub fn getInstallationDirectoryPath() -> Result<ArcStr> {
    let mut res: ArcStr;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), literal!("getInstallationDirectoryPath"), metamodelica::nil(), dummyMsg.clone())?) {
        (_, Deref @ Values::Value::STRING { string: __pa0 }) => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    res = metamodelica::Own::own(__pa0);
    Ok(res)
}

pub fn setInstallationDirectoryPath(mut installationDirectoryPath: ArcStr) -> Result<bool> {
    let mut res: bool;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), literal!("setInstallationDirectoryPath"), list![metamodelica::Ref::new(Values::Value::STRING { string: installationDirectoryPath })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::BOOL { boolean: __pa0 }) => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    res = metamodelica::Own::own(__pa0);
    Ok(res)
}

pub fn setEnvironmentVar(mut var: ArcStr, mut value: ArcStr) -> Result<bool> {
    let mut res: bool;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), literal!("setEnvironmentVar"), list![metamodelica::Ref::new(Values::Value::STRING { string: var }), metamodelica::Ref::new(Values::Value::STRING { string: value })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::BOOL { boolean: __pa0 }) => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    res = metamodelica::Own::own(__pa0);
    Ok(res)
}

pub fn getEnvironmentVar(mut var: ArcStr) -> Result<ArcStr> {
    let mut res: ArcStr;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), literal!("getEnvironmentVar"), list![metamodelica::Ref::new(Values::Value::STRING { string: var })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::STRING { string: __pa0 }) => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    res = metamodelica::Own::own(__pa0);
    Ok(res)
}

pub fn getTempDirectoryPath() -> Result<ArcStr> {
    let mut res: ArcStr;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), literal!("getTempDirectoryPath"), metamodelica::nil(), dummyMsg.clone())?) {
        (_, Deref @ Values::Value::STRING { string: __pa0 }) => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    res = metamodelica::Own::own(__pa0);
    Ok(res)
}

pub fn setTempDirectoryPath(mut tempDirectoryPath: ArcStr) -> Result<bool> {
    let mut res: bool;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), literal!("setTempDirectoryPath"), list![metamodelica::Ref::new(Values::Value::STRING { string: tempDirectoryPath })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::BOOL { boolean: __pa0 }) => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    res = metamodelica::Own::own(__pa0);
    Ok(res)
}

pub fn setCXXCompiler(mut compiler: ArcStr) -> Result<bool> {
    let mut res: bool;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), literal!("setCXXCompiler"), list![metamodelica::Ref::new(Values::Value::STRING { string: compiler })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::BOOL { boolean: __pa0 }) => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    res = metamodelica::Own::own(__pa0);
    Ok(res)
}

pub fn getCXXCompiler() -> Result<ArcStr> {
    let mut res: ArcStr;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), literal!("getCXXCompiler"), metamodelica::nil(), dummyMsg.clone())?) {
        (_, Deref @ Values::Value::STRING { string: __pa0 }) => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    res = metamodelica::Own::own(__pa0);
    Ok(res)
}

pub fn setCFlags(mut inString: ArcStr) -> Result<bool> {
    let mut res: bool;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), literal!("setCFlags"), list![metamodelica::Ref::new(Values::Value::STRING { string: inString })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::BOOL { boolean: __pa0 }) => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    res = metamodelica::Own::own(__pa0);
    Ok(res)
}

pub fn getCFlags() -> Result<ArcStr> {
    let mut res: ArcStr;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), literal!("getCFlags"), metamodelica::nil(), dummyMsg.clone())?) {
        (_, Deref @ Values::Value::STRING { string: __pa0 }) => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    res = metamodelica::Own::own(__pa0);
    Ok(res)
}

pub fn setCompiler(mut compiler: ArcStr) -> Result<bool> {
    let mut res: bool;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), literal!("setCompiler"), list![metamodelica::Ref::new(Values::Value::STRING { string: compiler })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::BOOL { boolean: __pa0 }) => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    res = metamodelica::Own::own(__pa0);
    Ok(res)
}

pub fn getCompiler() -> Result<ArcStr> {
    let mut res: ArcStr;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), literal!("getCompiler"), metamodelica::nil(), dummyMsg.clone())?) {
        (_, Deref @ Values::Value::STRING { string: __pa0 }) => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    res = metamodelica::Own::own(__pa0);
    Ok(res)
}

pub fn setLinkerFlags(mut linkerFlags: ArcStr) -> Result<bool> {
    let mut res: bool;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), literal!("setLinkerFlags"), list![metamodelica::Ref::new(Values::Value::STRING { string: linkerFlags })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::BOOL { boolean: __pa0 }) => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    res = metamodelica::Own::own(__pa0);
    Ok(res)
}

pub fn getLinkerFlags() -> Result<ArcStr> {
    let mut res: ArcStr;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), literal!("getLinkerFlags"), metamodelica::nil(), dummyMsg.clone())?) {
        (_, Deref @ Values::Value::STRING { string: __pa0 }) => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    res = metamodelica::Own::own(__pa0);
    Ok(res)
}

pub fn setLinker(mut linker: ArcStr) -> Result<bool> {
    let mut res: bool;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), literal!("setLinker"), list![metamodelica::Ref::new(Values::Value::STRING { string: linker })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::BOOL { boolean: __pa0 }) => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    res = metamodelica::Own::own(__pa0);
    Ok(res)
}

pub fn getLinker() -> Result<ArcStr> {
    let mut res: ArcStr;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), literal!("getLinker"), metamodelica::nil(), dummyMsg.clone())?) {
        (_, Deref @ Values::Value::STRING { string: __pa0 }) => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    res = metamodelica::Own::own(__pa0);
    Ok(res)
}

pub fn generateSeparateCodeDependenciesMakefile(
    mut filename: ArcStr,
    mut directory: ArcStr,
    mut suffix: ArcStr,
) -> Result<bool> {
    let mut res: bool;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), literal!("generateSeparateCodeDependenciesMakefile"), list![metamodelica::Ref::new(Values::Value::STRING { string: filename }), metamodelica::Ref::new(Values::Value::STRING { string: directory }), metamodelica::Ref::new(Values::Value::STRING { string: suffix })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::BOOL { boolean: __pa0 }) => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    res = metamodelica::Own::own(__pa0);
    Ok(res)
}

pub fn generateSeparateCodeDependencies(mut stampSuffix: ArcStr) -> Result<metamodelica::List<ArcStr>> {
    let mut res: metamodelica::List<ArcStr>;
    let mut res_arr: metamodelica::Ref<Values::Value>;
    (_, res_arr) = CevalScript::cevalInteractiveFunctions2(
        FCore::emptyCache(),
        FGraph::empty(),
        literal!("generateSeparateCodeDependencies"),
        list![metamodelica::Ref::new(Values::Value::STRING { string: stampSuffix })],
        dummyMsg.clone(),
    )?;
    res = ({
        let mut __acc: metamodelica::List<ArcStr> = metamodelica::nil();
        for mut res_arr_iter in (ValuesUtil::arrayValues(&res_arr)?).into_iter().cloned() {
            let __x = (match &*res_arr_iter.clone() {
                Values::Value::STRING {
                    string: __res_arr_iter_string,
                } => __res_arr_iter_string.clone(),
                _ => return Err("match: no arm matched"),
            });
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    Ok(res)
}

pub fn generateSeparateCode(mut className: ArcStr, mut cleanCache: bool) -> Result<bool> {
    let mut res: bool;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), literal!("generateSeparateCode"), list![metamodelica::Ref::new(Values::Value::CODE { A: metamodelica::Ref::new(Absyn::CodeNode::C_TYPENAME { path: Parser::stringPath(className)? }) }), metamodelica::Ref::new(Values::Value::BOOL { boolean: cleanCache })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::BOOL { boolean: __pa0 }) => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    res = metamodelica::Own::own(__pa0);
    Ok(res)
}

pub fn generateJuliaHeader(mut fileName: ArcStr) -> Result<bool> {
    let mut res: bool;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), literal!("generateJuliaHeader"), list![metamodelica::Ref::new(Values::Value::STRING { string: fileName })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::BOOL { boolean: __pa0 }) => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    res = metamodelica::Own::own(__pa0);
    Ok(res)
}

pub fn generateHeader(mut fileName: ArcStr) -> Result<bool> {
    let mut res: bool;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), literal!("generateHeader"), list![metamodelica::Ref::new(Values::Value::STRING { string: fileName })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::BOOL { boolean: __pa0 }) => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    res = metamodelica::Own::own(__pa0);
    Ok(res)
}

pub fn clearVariables() -> Result<bool> {
    let mut res: bool;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), literal!("clearVariables"), metamodelica::nil(), dummyMsg.clone())?) {
        (_, Deref @ Values::Value::BOOL { boolean: __pa0 }) => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    res = metamodelica::Own::own(__pa0);
    Ok(res)
}

pub fn clearProgram() -> Result<bool> {
    let mut res: bool;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), literal!("clearProgram"), metamodelica::nil(), dummyMsg.clone())?) {
        (_, Deref @ Values::Value::BOOL { boolean: __pa0 }) => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    res = metamodelica::Own::own(__pa0);
    Ok(res)
}

pub fn clear() -> Result<bool> {
    let mut res: bool;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), literal!("clear"), metamodelica::nil(), dummyMsg.clone())?) {
        (_, Deref @ Values::Value::BOOL { boolean: __pa0 }) => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    res = metamodelica::Own::own(__pa0);
    Ok(res)
}

pub fn help(mut topic: ArcStr) -> Result<ArcStr> {
    let mut res: ArcStr;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), literal!("help"), list![metamodelica::Ref::new(Values::Value::STRING { string: topic })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::STRING { string: __pa0 }) => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    res = metamodelica::Own::own(__pa0);
    Ok(res)
}

pub fn saveAll(mut fileName: ArcStr) -> Result<bool> {
    let mut res: bool;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), literal!("saveAll"), list![metamodelica::Ref::new(Values::Value::STRING { string: fileName })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::BOOL { boolean: __pa0 }) => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    res = metamodelica::Own::own(__pa0);
    Ok(res)
}

pub fn system_parallel(
    mut callStr: metamodelica::List<ArcStr>,
    mut numThreads: i32,
) -> Result<metamodelica::List<i32>> {
    let mut res: metamodelica::List<i32>;
    let mut res_arr: metamodelica::Ref<Values::Value>;
    (_, res_arr) = CevalScript::cevalInteractiveFunctions2(
        FCore::emptyCache(),
        FGraph::empty(),
        literal!("system_parallel"),
        list![
            ValuesMake::makeArray(
                ({
                    let mut __acc: metamodelica::List<metamodelica::Ref<Values::Value>> = metamodelica::nil();
                    for mut callStr_iter in (callStr).into_iter().cloned() {
                        let __x = metamodelica::Ref::new(Values::Value::STRING {
                            string: callStr_iter.clone(),
                        });
                        __acc = cons(__x, __acc);
                    }
                    __acc.reverse()
                })
            ),
            metamodelica::Ref::new(Values::Value::INTEGER { integer: numThreads })
        ],
        dummyMsg.clone(),
    )?;
    res = ({
        let mut __acc: metamodelica::List<i32> = metamodelica::nil();
        for mut res_arr_iter in (ValuesUtil::arrayValues(&res_arr)?).into_iter().cloned() {
            let __x = (match &*res_arr_iter.clone() {
                Values::Value::INTEGER {
                    integer: __res_arr_iter_integer,
                } => __res_arr_iter_integer.clone(),
                _ => return Err("match: no arm matched"),
            });
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    Ok(res)
}

pub fn system(mut callStr: ArcStr, mut outputFile: ArcStr) -> Result<i32> {
    let mut res: i32;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), literal!("system"), list![metamodelica::Ref::new(Values::Value::STRING { string: callStr }), metamodelica::Ref::new(Values::Value::STRING { string: outputFile })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::INTEGER { integer: __pa0 }) => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    res = metamodelica::Own::own(__pa0);
    Ok(res)
}

pub fn loadFileInteractive(
    mut filename: ArcStr,
    mut encoding: ArcStr,
    mut uses: bool,
    mut notify: bool,
    mut requireExactVersion: bool,
) -> Result<metamodelica::List<ArcStr>> {
    let mut res: metamodelica::List<ArcStr>;
    let mut res_arr: metamodelica::Ref<Values::Value>;
    (_, res_arr) = CevalScript::cevalInteractiveFunctions2(
        FCore::emptyCache(),
        FGraph::empty(),
        literal!("loadFileInteractive"),
        list![
            metamodelica::Ref::new(Values::Value::STRING { string: filename }),
            metamodelica::Ref::new(Values::Value::STRING { string: encoding }),
            metamodelica::Ref::new(Values::Value::BOOL { boolean: uses }),
            metamodelica::Ref::new(Values::Value::BOOL { boolean: notify }),
            metamodelica::Ref::new(Values::Value::BOOL {
                boolean: requireExactVersion
            })
        ],
        dummyMsg.clone(),
    )?;
    res = ({
        let mut __acc: metamodelica::List<ArcStr> = metamodelica::nil();
        for mut res_arr_iter in (ValuesUtil::arrayValues(&res_arr)?).into_iter().cloned() {
            let __x = ValuesDump::valString(&(res_arr_iter.clone()))?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    Ok(res)
}

pub fn loadFileInteractiveQualified(mut filename: ArcStr, mut encoding: ArcStr) -> Result<metamodelica::List<ArcStr>> {
    let mut res: metamodelica::List<ArcStr>;
    let mut res_arr: metamodelica::Ref<Values::Value>;
    (_, res_arr) = CevalScript::cevalInteractiveFunctions2(
        FCore::emptyCache(),
        FGraph::empty(),
        literal!("loadFileInteractiveQualified"),
        list![
            metamodelica::Ref::new(Values::Value::STRING { string: filename }),
            metamodelica::Ref::new(Values::Value::STRING { string: encoding })
        ],
        dummyMsg.clone(),
    )?;
    res = ({
        let mut __acc: metamodelica::List<ArcStr> = metamodelica::nil();
        for mut res_arr_iter in (ValuesUtil::arrayValues(&res_arr)?).into_iter().cloned() {
            let __x = ValuesDump::valString(&(res_arr_iter.clone()))?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    Ok(res)
}

pub fn parseFile(mut filename: ArcStr, mut encoding: ArcStr) -> Result<metamodelica::List<ArcStr>> {
    let mut res: metamodelica::List<ArcStr>;
    let mut res_arr: metamodelica::Ref<Values::Value>;
    (_, res_arr) = CevalScript::cevalInteractiveFunctions2(
        FCore::emptyCache(),
        FGraph::empty(),
        literal!("parseFile"),
        list![
            metamodelica::Ref::new(Values::Value::STRING { string: filename }),
            metamodelica::Ref::new(Values::Value::STRING { string: encoding })
        ],
        dummyMsg.clone(),
    )?;
    res = ({
        let mut __acc: metamodelica::List<ArcStr> = metamodelica::nil();
        for mut res_arr_iter in (ValuesUtil::arrayValues(&res_arr)?).into_iter().cloned() {
            let __x = ValuesDump::valString(&(res_arr_iter.clone()))?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    Ok(res)
}

pub fn parseString(mut data: ArcStr, mut filename: ArcStr) -> Result<metamodelica::List<ArcStr>> {
    let mut res: metamodelica::List<ArcStr>;
    let mut res_arr: metamodelica::Ref<Values::Value>;
    (_, res_arr) = CevalScript::cevalInteractiveFunctions2(
        FCore::emptyCache(),
        FGraph::empty(),
        literal!("parseString"),
        list![
            metamodelica::Ref::new(Values::Value::STRING { string: data }),
            metamodelica::Ref::new(Values::Value::STRING { string: filename })
        ],
        dummyMsg.clone(),
    )?;
    res = ({
        let mut __acc: metamodelica::List<ArcStr> = metamodelica::nil();
        for mut res_arr_iter in (ValuesUtil::arrayValues(&res_arr)?).into_iter().cloned() {
            let __x = ValuesDump::valString(&(res_arr_iter.clone()))?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    Ok(res)
}

pub fn loadClassContentString(
    mut data: ArcStr,
    mut className: ArcStr,
    mut offsetX: i32,
    mut offsetY: i32,
) -> Result<bool> {
    let mut res: bool;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), literal!("loadClassContentString"), list![metamodelica::Ref::new(Values::Value::STRING { string: data }), metamodelica::Ref::new(Values::Value::CODE { A: metamodelica::Ref::new(Absyn::CodeNode::C_TYPENAME { path: Parser::stringPath(className)? }) }), metamodelica::Ref::new(Values::Value::INTEGER { integer: offsetX }), metamodelica::Ref::new(Values::Value::INTEGER { integer: offsetY })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::BOOL { boolean: __pa0 }) => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    res = metamodelica::Own::own(__pa0);
    Ok(res)
}

pub fn loadString(
    mut data: ArcStr,
    mut filename: ArcStr,
    mut encoding: ArcStr,
    mut merge: bool,
    mut uses: bool,
    mut notify: bool,
    mut requireExactVersion: bool,
) -> Result<bool> {
    let mut res: bool;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), literal!("loadString"), list![metamodelica::Ref::new(Values::Value::STRING { string: data }), metamodelica::Ref::new(Values::Value::STRING { string: filename }), metamodelica::Ref::new(Values::Value::STRING { string: encoding }), metamodelica::Ref::new(Values::Value::BOOL { boolean: merge }), metamodelica::Ref::new(Values::Value::BOOL { boolean: uses }), metamodelica::Ref::new(Values::Value::BOOL { boolean: notify }), metamodelica::Ref::new(Values::Value::BOOL { boolean: requireExactVersion })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::BOOL { boolean: __pa0 }) => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    res = metamodelica::Own::own(__pa0);
    Ok(res)
}

pub fn reloadClass(mut name: ArcStr, mut encoding: ArcStr) -> Result<bool> {
    let mut res: bool;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), literal!("reloadClass"), list![metamodelica::Ref::new(Values::Value::CODE { A: metamodelica::Ref::new(Absyn::CodeNode::C_TYPENAME { path: Parser::stringPath(name)? }) }), metamodelica::Ref::new(Values::Value::STRING { string: encoding })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::BOOL { boolean: __pa0 }) => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    res = metamodelica::Own::own(__pa0);
    Ok(res)
}

pub fn loadEncryptedPackage(
    mut fileName: ArcStr,
    mut workdir: ArcStr,
    mut skipUnzip: bool,
    mut uses: bool,
    mut notify: bool,
    mut requireExactVersion: bool,
) -> Result<bool> {
    let mut res: bool;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), literal!("loadEncryptedPackage"), list![metamodelica::Ref::new(Values::Value::STRING { string: fileName }), metamodelica::Ref::new(Values::Value::STRING { string: workdir }), metamodelica::Ref::new(Values::Value::BOOL { boolean: skipUnzip }), metamodelica::Ref::new(Values::Value::BOOL { boolean: uses }), metamodelica::Ref::new(Values::Value::BOOL { boolean: notify }), metamodelica::Ref::new(Values::Value::BOOL { boolean: requireExactVersion })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::BOOL { boolean: __pa0 }) => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    res = metamodelica::Own::own(__pa0);
    Ok(res)
}

pub fn parseEncryptedPackage(mut fileName: ArcStr, mut workdir: ArcStr) -> Result<metamodelica::List<ArcStr>> {
    let mut res: metamodelica::List<ArcStr>;
    let mut res_arr: metamodelica::Ref<Values::Value>;
    (_, res_arr) = CevalScript::cevalInteractiveFunctions2(
        FCore::emptyCache(),
        FGraph::empty(),
        literal!("parseEncryptedPackage"),
        list![
            metamodelica::Ref::new(Values::Value::STRING { string: fileName }),
            metamodelica::Ref::new(Values::Value::STRING { string: workdir })
        ],
        dummyMsg.clone(),
    )?;
    res = ({
        let mut __acc: metamodelica::List<ArcStr> = metamodelica::nil();
        for mut res_arr_iter in (ValuesUtil::arrayValues(&res_arr)?).into_iter().cloned() {
            let __x = ValuesDump::valString(&(res_arr_iter.clone()))?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    Ok(res)
}

pub fn loadFiles(
    mut fileNames: metamodelica::List<ArcStr>,
    mut encoding: ArcStr,
    mut numThreads: i32,
    mut uses: bool,
    mut notify: bool,
    mut requireExactVersion: bool,
    mut allowWithin: bool,
) -> Result<bool> {
    let mut res: bool;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), literal!("loadFiles"), list![ValuesMake::makeArray(({
        let mut __acc: metamodelica::List<metamodelica::Ref<Values::Value>> = metamodelica::nil();
        for mut fileNames_iter in (fileNames).into_iter().cloned() {
            let __x = metamodelica::Ref::new(Values::Value::STRING { string: fileNames_iter.clone() });
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    })), metamodelica::Ref::new(Values::Value::STRING { string: encoding }), metamodelica::Ref::new(Values::Value::INTEGER { integer: numThreads }), metamodelica::Ref::new(Values::Value::BOOL { boolean: uses }), metamodelica::Ref::new(Values::Value::BOOL { boolean: notify }), metamodelica::Ref::new(Values::Value::BOOL { boolean: requireExactVersion }), metamodelica::Ref::new(Values::Value::BOOL { boolean: allowWithin })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::BOOL { boolean: __pa0 }) => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    res = metamodelica::Own::own(__pa0);
    Ok(res)
}

pub fn loadFile(
    mut fileName: ArcStr,
    mut encoding: ArcStr,
    mut uses: bool,
    mut notify: bool,
    mut requireExactVersion: bool,
    mut allowWithin: bool,
) -> Result<bool> {
    let mut res: bool;
    let __pa0 = ::match_deref::match_deref! { match &(CevalScript::cevalInteractiveFunctions2(FCore::emptyCache(), FGraph::empty(), literal!("loadFile"), list![metamodelica::Ref::new(Values::Value::STRING { string: fileName }), metamodelica::Ref::new(Values::Value::STRING { string: encoding }), metamodelica::Ref::new(Values::Value::BOOL { boolean: uses }), metamodelica::Ref::new(Values::Value::BOOL { boolean: notify }), metamodelica::Ref::new(Values::Value::BOOL { boolean: requireExactVersion }), metamodelica::Ref::new(Values::Value::BOOL { boolean: allowWithin })], dummyMsg.clone())?) {
        (_, Deref @ Values::Value::BOOL { boolean: __pa0 }) => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    res = metamodelica::Own::own(__pa0);
    Ok(res)
}
