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

use openmodelica_frontend_types::Values;
use openmodelica_util::OMSimulatorExt;

pub fn ceval(
    mut inFunctionName: ArcStr,
    mut inVals: metamodelica::List<metamodelica::Ref<Values::Value>>,
) -> Result<metamodelica::Ref<Values::Value>> {
    let mut outValue: metamodelica::Ref<Values::Value>;
    outValue = (::match_deref::match_deref! { match &((inFunctionName, inVals)) {
        (Deref @ "loadOMSimulator", Deref @ metamodelica::ListNode::Nil) => {
            let mut status: i32;
            status = OMSimulatorExt::loadOMSimulator();
            metamodelica::Ref::new(Values::Value::INTEGER { integer: status })
        },
        (Deref @ "unloadOMSimulator", Deref @ metamodelica::ListNode::Nil) => {
            let mut status: i32;
            status = OMSimulatorExt::unloadOMSimulator();
            metamodelica::Ref::new(Values::Value::INTEGER { integer: status })
        },
        (Deref @ "oms_addBus", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: cref }, tail: Deref @ metamodelica::ListNode::Nil }) => {
            let mut status: i32;
            status = OMSimulatorExt::oms_addBus(cref.clone());
            metamodelica::Ref::new(Values::Value::INTEGER { integer: status })
        },
        (Deref @ "oms_addConnection", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: crefA }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: crefB }, tail: Deref @ metamodelica::ListNode::Nil } }) => {
            let mut status: i32;
            status = OMSimulatorExt::oms_addConnection(crefA.clone(), crefB.clone());
            metamodelica::Ref::new(Values::Value::INTEGER { integer: status })
        },
        (Deref @ "oms_addConnector", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: cref }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::ENUM_LITERAL { index: causality, .. }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::ENUM_LITERAL { index: type_, .. }, tail: Deref @ metamodelica::ListNode::Nil } } }) => {
            let mut status: i32;
            status = OMSimulatorExt::oms_addConnector(cref.clone(), causality.clone() - 1, type_.clone() - 1);
            metamodelica::Ref::new(Values::Value::INTEGER { integer: status })
        },
        (Deref @ "oms_addConnectorToBus", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: busCref }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: connectorCref }, tail: Deref @ metamodelica::ListNode::Nil } }) => {
            let mut status: i32;
            status = OMSimulatorExt::oms_addConnectorToBus(busCref.clone(), connectorCref.clone());
            metamodelica::Ref::new(Values::Value::INTEGER { integer: status })
        },
        (Deref @ "oms_addConnectorToTLMBus", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: busCref }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: connectorCref }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: stype_ }, tail: Deref @ metamodelica::ListNode::Nil } } }) => {
            let mut status: i32;
            status = OMSimulatorExt::oms_addConnectorToTLMBus(busCref.clone(), connectorCref.clone(), stype_.clone());
            metamodelica::Ref::new(Values::Value::INTEGER { integer: status })
        },
        (Deref @ "oms_addDynamicValueIndicator", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: signal }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: s_lower }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: s_upper }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::REAL { real: stepSize }, tail: Deref @ metamodelica::ListNode::Nil } } } }) => {
            let mut status: i32;
            status = OMSimulatorExt::oms_addDynamicValueIndicator(signal.clone(), s_lower.clone(), s_upper.clone(), stepSize.clone());
            metamodelica::Ref::new(Values::Value::INTEGER { integer: status })
        },
        (Deref @ "oms_addEventIndicator", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: signal }, tail: Deref @ metamodelica::ListNode::Nil }) => {
            let mut status: i32;
            status = OMSimulatorExt::oms_addEventIndicator(signal.clone());
            metamodelica::Ref::new(Values::Value::INTEGER { integer: status })
        },
        (Deref @ "oms_addExternalModel", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: cref }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: path }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: startscript }, tail: Deref @ metamodelica::ListNode::Nil } } }) => {
            let mut status: i32;
            status = OMSimulatorExt::oms_addExternalModel(cref.clone(), path.clone(), startscript.clone());
            metamodelica::Ref::new(Values::Value::INTEGER { integer: status })
        },
        (Deref @ "oms_addSignalsToResults", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: cref }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: regex }, tail: Deref @ metamodelica::ListNode::Nil } }) => {
            let mut status: i32;
            status = OMSimulatorExt::oms_addSignalsToResults(cref.clone(), regex.clone());
            metamodelica::Ref::new(Values::Value::INTEGER { integer: status })
        },
        (Deref @ "oms_addStaticValueIndicator", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: signal }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::REAL { real: lower }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::REAL { real: upper }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::REAL { real: stepSize }, tail: Deref @ metamodelica::ListNode::Nil } } } }) => {
            let mut status: i32;
            status = OMSimulatorExt::oms_addStaticValueIndicator(signal.clone(), lower.clone(), upper.clone(), stepSize.clone());
            metamodelica::Ref::new(Values::Value::INTEGER { integer: status })
        },
        (Deref @ "oms_addSubModel", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: cref }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: fmuPath }, tail: Deref @ metamodelica::ListNode::Nil } }) => {
            let mut status: i32;
            status = OMSimulatorExt::oms_addSubModel(cref.clone(), fmuPath.clone());
            metamodelica::Ref::new(Values::Value::INTEGER { integer: status })
        },
        (Deref @ "oms_addSystem", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: cref }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::ENUM_LITERAL { index: type_, .. }, tail: Deref @ metamodelica::ListNode::Nil } }) => {
            let mut status: i32;
            status = OMSimulatorExt::oms_addSystem(cref.clone(), type_.clone() - 1);
            metamodelica::Ref::new(Values::Value::INTEGER { integer: status })
        },
        (Deref @ "oms_addTimeIndicator", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: signal }, tail: Deref @ metamodelica::ListNode::Nil }) => {
            let mut status: i32;
            status = OMSimulatorExt::oms_addTimeIndicator(signal.clone());
            metamodelica::Ref::new(Values::Value::INTEGER { integer: status })
        },
        (Deref @ "oms_addTLMBus", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: cref }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::ENUM_LITERAL { index: domain, .. }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::INTEGER { integer: dimensions }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::ENUM_LITERAL { index: interpolation, .. }, tail: Deref @ metamodelica::ListNode::Nil } } } }) => {
            let mut status: i32;
            status = OMSimulatorExt::oms_addTLMBus(cref.clone(), domain.clone() - 1, dimensions.clone(), interpolation.clone() - 1);
            metamodelica::Ref::new(Values::Value::INTEGER { integer: status })
        },
        (Deref @ "oms_addTLMConnection", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: crefA }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: crefB }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::REAL { real: delay }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::REAL { real: alpha }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::REAL { real: linearimpedance }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::REAL { real: angularimpedance }, tail: Deref @ metamodelica::ListNode::Nil } } } } } }) => {
            let mut status: i32;
            status = OMSimulatorExt::oms_addTLMConnection(crefA.clone(), crefB.clone(), delay.clone(), alpha.clone(), linearimpedance.clone(), angularimpedance.clone());
            metamodelica::Ref::new(Values::Value::INTEGER { integer: status })
        },
        (Deref @ "oms_compareSimulationResults", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: filenameA }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: filenameB }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: var }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::REAL { real: relTol }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::REAL { real: absTol }, tail: Deref @ metamodelica::ListNode::Nil } } } } }) => {
            let mut status: i32;
            status = OMSimulatorExt::oms_compareSimulationResults(filenameA.clone(), filenameB.clone(), var.clone(), relTol.clone(), absTol.clone());
            metamodelica::Ref::new(Values::Value::INTEGER { integer: status })
        },
        (Deref @ "oms_copySystem", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: source }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: target }, tail: Deref @ metamodelica::ListNode::Nil } }) => {
            let mut status: i32;
            status = OMSimulatorExt::oms_copySystem(source.clone(), target.clone());
            metamodelica::Ref::new(Values::Value::INTEGER { integer: status })
        },
        (Deref @ "oms_delete", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: cref }, tail: Deref @ metamodelica::ListNode::Nil }) => {
            let mut status: i32;
            status = OMSimulatorExt::oms_delete(cref.clone());
            metamodelica::Ref::new(Values::Value::INTEGER { integer: status })
        },
        (Deref @ "oms_deleteConnection", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: crefA }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: crefB }, tail: Deref @ metamodelica::ListNode::Nil } }) => {
            let mut status: i32;
            status = OMSimulatorExt::oms_deleteConnection(crefA.clone(), crefB.clone());
            metamodelica::Ref::new(Values::Value::INTEGER { integer: status })
        },
        (Deref @ "oms_deleteConnectorFromBus", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: busCref }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: connectorCref }, tail: Deref @ metamodelica::ListNode::Nil } }) => {
            let mut status: i32;
            status = OMSimulatorExt::oms_deleteConnectorFromBus(busCref.clone(), connectorCref.clone());
            metamodelica::Ref::new(Values::Value::INTEGER { integer: status })
        },
        (Deref @ "oms_deleteConnectorFromTLMBus", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: busCref }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: connectorCref }, tail: Deref @ metamodelica::ListNode::Nil } }) => {
            let mut status: i32;
            status = OMSimulatorExt::oms_deleteConnectorFromTLMBus(busCref.clone(), connectorCref.clone());
            metamodelica::Ref::new(Values::Value::INTEGER { integer: status })
        },
        (Deref @ "oms_export", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: cref }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: filename }, tail: Deref @ metamodelica::ListNode::Nil } }) => {
            let mut status: i32;
            status = OMSimulatorExt::oms_export(cref.clone(), filename.clone());
            metamodelica::Ref::new(Values::Value::INTEGER { integer: status })
        },
        (Deref @ "oms_exportDependencyGraphs", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: cref }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: initialization }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: event }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: simulation }, tail: Deref @ metamodelica::ListNode::Nil } } } }) => {
            let mut status: i32;
            status = OMSimulatorExt::oms_exportDependencyGraphs(cref.clone(), initialization.clone(), event.clone(), simulation.clone());
            metamodelica::Ref::new(Values::Value::INTEGER { integer: status })
        },
        (Deref @ "oms_exportSnapshot", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: cref }, tail: Deref @ metamodelica::ListNode::Nil }) => {
            let mut contents: ArcStr;
            let mut status: i32;
            (contents, status) = OMSimulatorExt::oms_exportSnapshot(cref.clone());
            metamodelica::Ref::new(Values::Value::TUPLE { valueLst: list![metamodelica::Ref::new(Values::Value::STRING { string: contents }), metamodelica::Ref::new(Values::Value::INTEGER { integer: status })] })
        },
        (Deref @ "oms_extractFMIKind", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: filename }, tail: Deref @ metamodelica::ListNode::Nil }) => {
            let mut status: i32;
            let mut kind: i32;
            (kind, status) = OMSimulatorExt::oms_extractFMIKind(filename.clone());
            metamodelica::Ref::new(Values::Value::TUPLE { valueLst: list![metamodelica::Ref::new(Values::Value::INTEGER { integer: kind }), metamodelica::Ref::new(Values::Value::INTEGER { integer: status })] })
        },
        (Deref @ "oms_getBoolean", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: cref }, tail: Deref @ metamodelica::ListNode::Nil }) => {
            let mut status: i32;
            let mut b: bool;
            (b, status) = OMSimulatorExt::oms_getBoolean(cref.clone());
            metamodelica::Ref::new(Values::Value::TUPLE { valueLst: list![metamodelica::Ref::new(Values::Value::BOOL { boolean: b }), metamodelica::Ref::new(Values::Value::INTEGER { integer: status })] })
        },
        (Deref @ "oms_getFixedStepSize", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: cref }, tail: Deref @ metamodelica::ListNode::Nil }) => {
            let mut rvalue: metamodelica::Real;
            let mut status: i32;
            (rvalue, status) = OMSimulatorExt::oms_getFixedStepSize(cref.clone());
            metamodelica::Ref::new(Values::Value::TUPLE { valueLst: list![metamodelica::Ref::new(Values::Value::REAL { real: rvalue }), metamodelica::Ref::new(Values::Value::INTEGER { integer: status })] })
        },
        (Deref @ "oms_getInteger", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: cref }, tail: Deref @ metamodelica::ListNode::Nil }) => {
            let mut status: i32;
            let mut ivalue: i32;
            (ivalue, status) = OMSimulatorExt::oms_getInteger(cref.clone());
            metamodelica::Ref::new(Values::Value::TUPLE { valueLst: list![metamodelica::Ref::new(Values::Value::INTEGER { integer: ivalue }), metamodelica::Ref::new(Values::Value::INTEGER { integer: status })] })
        },
        (Deref @ "oms_getModelState", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: cref }, tail: Deref @ metamodelica::ListNode::Nil }) => {
            let mut status: i32;
            let mut ivalue: i32;
            (ivalue, status) = OMSimulatorExt::oms_getModelState(cref.clone());
            metamodelica::Ref::new(Values::Value::TUPLE { valueLst: list![metamodelica::Ref::new(Values::Value::INTEGER { integer: ivalue }), metamodelica::Ref::new(Values::Value::INTEGER { integer: status })] })
        },
        (Deref @ "oms_getReal", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: cref }, tail: Deref @ metamodelica::ListNode::Nil }) => {
            let mut rvalue: metamodelica::Real;
            let mut status: i32;
            (rvalue, status) = OMSimulatorExt::oms_getReal(cref.clone());
            metamodelica::Ref::new(Values::Value::TUPLE { valueLst: list![metamodelica::Ref::new(Values::Value::REAL { real: rvalue }), metamodelica::Ref::new(Values::Value::INTEGER { integer: status })] })
        },
        (Deref @ "oms_getSolver", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: cref }, tail: Deref @ metamodelica::ListNode::Nil }) => {
            let mut status: i32;
            let mut ivalue: i32;
            (ivalue, status) = OMSimulatorExt::oms_getSolver(cref.clone());
            metamodelica::Ref::new(Values::Value::TUPLE { valueLst: list![metamodelica::Ref::new(Values::Value::INTEGER { integer: ivalue }), metamodelica::Ref::new(Values::Value::INTEGER { integer: status })] })
        },
        (Deref @ "oms_getStartTime", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: cref }, tail: Deref @ metamodelica::ListNode::Nil }) => {
            let mut rvalue: metamodelica::Real;
            let mut status: i32;
            (rvalue, status) = OMSimulatorExt::oms_getStartTime(cref.clone());
            metamodelica::Ref::new(Values::Value::TUPLE { valueLst: list![metamodelica::Ref::new(Values::Value::REAL { real: rvalue }), metamodelica::Ref::new(Values::Value::INTEGER { integer: status })] })
        },
        (Deref @ "oms_getStopTime", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: cref }, tail: Deref @ metamodelica::ListNode::Nil }) => {
            let mut rvalue: metamodelica::Real;
            let mut status: i32;
            (rvalue, status) = OMSimulatorExt::oms_getStopTime(cref.clone());
            metamodelica::Ref::new(Values::Value::TUPLE { valueLst: list![metamodelica::Ref::new(Values::Value::REAL { real: rvalue }), metamodelica::Ref::new(Values::Value::INTEGER { integer: status })] })
        },
        (Deref @ "oms_getSubModelPath", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: cref }, tail: Deref @ metamodelica::ListNode::Nil }) => {
            let mut path: ArcStr;
            let mut status: i32;
            (path, status) = OMSimulatorExt::oms_getSubModelPath(cref.clone());
            metamodelica::Ref::new(Values::Value::TUPLE { valueLst: list![metamodelica::Ref::new(Values::Value::STRING { string: path }), metamodelica::Ref::new(Values::Value::INTEGER { integer: status })] })
        },
        (Deref @ "oms_getSystemType", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: cref }, tail: Deref @ metamodelica::ListNode::Nil }) => {
            let mut status: i32;
            let mut ivalue: i32;
            (ivalue, status) = OMSimulatorExt::oms_getSystemType(cref.clone());
            metamodelica::Ref::new(Values::Value::TUPLE { valueLst: list![metamodelica::Ref::new(Values::Value::INTEGER { integer: ivalue }), metamodelica::Ref::new(Values::Value::INTEGER { integer: status })] })
        },
        (Deref @ "oms_getTolerance", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: cref }, tail: Deref @ metamodelica::ListNode::Nil }) => {
            let mut absoluteTolerance: metamodelica::Real;
            let mut relativeTolerance: metamodelica::Real;
            let mut status: i32;
            (absoluteTolerance, relativeTolerance, status) = OMSimulatorExt::oms_getTolerance(cref.clone());
            metamodelica::Ref::new(Values::Value::TUPLE { valueLst: list![metamodelica::Ref::new(Values::Value::REAL { real: absoluteTolerance }), metamodelica::Ref::new(Values::Value::REAL { real: relativeTolerance }), metamodelica::Ref::new(Values::Value::INTEGER { integer: status })] })
        },
        (Deref @ "oms_getVariableStepSize", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: cref }, tail: Deref @ metamodelica::ListNode::Nil }) => {
            let mut initialStepSize: metamodelica::Real;
            let mut minimumStepSize: metamodelica::Real;
            let mut maximumStepSize: metamodelica::Real;
            let mut status: i32;
            (initialStepSize, minimumStepSize, maximumStepSize, status) = OMSimulatorExt::oms_getVariableStepSize(cref.clone());
            metamodelica::Ref::new(Values::Value::TUPLE { valueLst: list![metamodelica::Ref::new(Values::Value::REAL { real: initialStepSize }), metamodelica::Ref::new(Values::Value::REAL { real: minimumStepSize }), metamodelica::Ref::new(Values::Value::REAL { real: maximumStepSize }), metamodelica::Ref::new(Values::Value::INTEGER { integer: status })] })
        },
        (Deref @ "oms_faultInjection", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: signal }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::ENUM_LITERAL { index: faultType, .. }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::REAL { real: faultValue }, tail: Deref @ metamodelica::ListNode::Nil } } }) => {
            let mut status: i32;
            status = OMSimulatorExt::oms_faultInjection(signal.clone(), faultType.clone() - 1, faultValue.clone());
            metamodelica::Ref::new(Values::Value::INTEGER { integer: status })
        },
        (Deref @ "oms_importFile", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: filename }, tail: Deref @ metamodelica::ListNode::Nil }) => {
            let mut cref: ArcStr;
            let mut status: i32;
            (cref, status) = OMSimulatorExt::oms_importFile(filename.clone());
            metamodelica::Ref::new(Values::Value::TUPLE { valueLst: list![metamodelica::Ref::new(Values::Value::STRING { string: cref }), metamodelica::Ref::new(Values::Value::INTEGER { integer: status })] })
        },
        (Deref @ "oms_importSnapshot", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: cref }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: snapshot }, tail: Deref @ metamodelica::ListNode::Nil } }) => {
            let mut status: i32;
            status = OMSimulatorExt::oms_importSnapshot(cref.clone(), snapshot.clone());
            metamodelica::Ref::new(Values::Value::INTEGER { integer: status })
        },
        (Deref @ "oms_initialize", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: cref }, tail: Deref @ metamodelica::ListNode::Nil }) => {
            let mut status: i32;
            status = OMSimulatorExt::oms_initialize(cref.clone());
            metamodelica::Ref::new(Values::Value::INTEGER { integer: status })
        },
        (Deref @ "oms_instantiate", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: cref }, tail: Deref @ metamodelica::ListNode::Nil }) => {
            let mut status: i32;
            status = OMSimulatorExt::oms_instantiate(cref.clone());
            metamodelica::Ref::new(Values::Value::INTEGER { integer: status })
        },
        (Deref @ "oms_list", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: cref }, tail: Deref @ metamodelica::ListNode::Nil }) => {
            let mut contents: ArcStr;
            let mut status: i32;
            (contents, status) = OMSimulatorExt::oms_list(cref.clone());
            metamodelica::Ref::new(Values::Value::TUPLE { valueLst: list![metamodelica::Ref::new(Values::Value::STRING { string: contents }), metamodelica::Ref::new(Values::Value::INTEGER { integer: status })] })
        },
        (Deref @ "oms_listUnconnectedConnectors", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: cref }, tail: Deref @ metamodelica::ListNode::Nil }) => {
            let mut contents: ArcStr;
            let mut status: i32;
            (contents, status) = OMSimulatorExt::oms_listUnconnectedConnectors(cref.clone());
            metamodelica::Ref::new(Values::Value::TUPLE { valueLst: list![metamodelica::Ref::new(Values::Value::STRING { string: contents }), metamodelica::Ref::new(Values::Value::INTEGER { integer: status })] })
        },
        (Deref @ "oms_loadSnapshot", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: cref }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: snapshot }, tail: Deref @ metamodelica::ListNode::Nil } }) => {
            let mut newCref: ArcStr;
            let mut status: i32;
            (newCref, status) = OMSimulatorExt::oms_loadSnapshot(cref.clone(), snapshot.clone());
            metamodelica::Ref::new(Values::Value::TUPLE { valueLst: list![metamodelica::Ref::new(Values::Value::STRING { string: newCref }), metamodelica::Ref::new(Values::Value::INTEGER { integer: status })] })
        },
        (Deref @ "oms_newModel", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: cref }, tail: Deref @ metamodelica::ListNode::Nil }) => {
            let mut status: i32;
            status = OMSimulatorExt::oms_newModel(cref.clone());
            metamodelica::Ref::new(Values::Value::INTEGER { integer: status })
        },
        (Deref @ "oms_removeSignalsFromResults", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: cref }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: regex }, tail: Deref @ metamodelica::ListNode::Nil } }) => {
            let mut status: i32;
            status = OMSimulatorExt::oms_removeSignalsFromResults(cref.clone(), regex.clone());
            metamodelica::Ref::new(Values::Value::INTEGER { integer: status })
        },
        (Deref @ "oms_rename", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: cref }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: newCref }, tail: Deref @ metamodelica::ListNode::Nil } }) => {
            let mut status: i32;
            status = OMSimulatorExt::oms_rename(cref.clone(), newCref.clone());
            metamodelica::Ref::new(Values::Value::INTEGER { integer: status })
        },
        (Deref @ "oms_reset", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: cref }, tail: Deref @ metamodelica::ListNode::Nil }) => {
            let mut status: i32;
            status = OMSimulatorExt::oms_reset(cref.clone());
            metamodelica::Ref::new(Values::Value::INTEGER { integer: status })
        },
        (Deref @ "oms_RunFile", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: filename }, tail: Deref @ metamodelica::ListNode::Nil }) => {
            let mut status: i32;
            status = OMSimulatorExt::oms_RunFile(filename.clone());
            metamodelica::Ref::new(Values::Value::INTEGER { integer: status })
        },
        (Deref @ "oms_setBoolean", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: cref }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::BOOL { boolean: b }, tail: Deref @ metamodelica::ListNode::Nil } }) => {
            let mut status: i32;
            status = OMSimulatorExt::oms_setBoolean(cref.clone(), b.clone());
            metamodelica::Ref::new(Values::Value::INTEGER { integer: status })
        },
        (Deref @ "oms_setCommandLineOption", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: cmd }, tail: Deref @ metamodelica::ListNode::Nil }) => {
            let mut status: i32;
            status = OMSimulatorExt::oms_setCommandLineOption(cmd.clone());
            metamodelica::Ref::new(Values::Value::INTEGER { integer: status })
        },
        (Deref @ "oms_setFixedStepSize", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: cref }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::REAL { real: stepSize }, tail: Deref @ metamodelica::ListNode::Nil } }) => {
            let mut status: i32;
            status = OMSimulatorExt::oms_setFixedStepSize(cref.clone(), stepSize.clone());
            metamodelica::Ref::new(Values::Value::INTEGER { integer: status })
        },
        (Deref @ "oms_setInteger", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: cref }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::INTEGER { integer: ivalue }, tail: Deref @ metamodelica::ListNode::Nil } }) => {
            let mut status: i32;
            status = OMSimulatorExt::oms_setInteger(cref.clone(), ivalue.clone());
            metamodelica::Ref::new(Values::Value::INTEGER { integer: status })
        },
        (Deref @ "oms_setLogFile", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: filename }, tail: Deref @ metamodelica::ListNode::Nil }) => {
            let mut status: i32;
            status = OMSimulatorExt::oms_setLogFile(filename.clone());
            metamodelica::Ref::new(Values::Value::INTEGER { integer: status })
        },
        (Deref @ "oms_setLoggingInterval", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: cref }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::REAL { real: loggingInterval }, tail: Deref @ metamodelica::ListNode::Nil } }) => {
            let mut status: i32;
            status = OMSimulatorExt::oms_setLoggingInterval(cref.clone(), loggingInterval.clone());
            metamodelica::Ref::new(Values::Value::INTEGER { integer: status })
        },
        (Deref @ "oms_setLoggingLevel", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::INTEGER { integer: logLevel }, tail: Deref @ metamodelica::ListNode::Nil }) => {
            let mut status: i32;
            status = OMSimulatorExt::oms_setLoggingLevel(logLevel.clone());
            metamodelica::Ref::new(Values::Value::INTEGER { integer: status })
        },
        (Deref @ "oms_setReal", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: cref }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::REAL { real: rvalue }, tail: Deref @ metamodelica::ListNode::Nil } }) => {
            let mut status: i32;
            status = OMSimulatorExt::oms_setReal(cref.clone(), rvalue.clone());
            metamodelica::Ref::new(Values::Value::INTEGER { integer: status })
        },
        (Deref @ "oms_setRealInputDerivative", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: cref }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::REAL { real: rvalue }, tail: Deref @ metamodelica::ListNode::Nil } }) => {
            let mut status: i32;
            status = OMSimulatorExt::oms_setRealInputDerivative(cref.clone(), rvalue.clone());
            metamodelica::Ref::new(Values::Value::INTEGER { integer: status })
        },
        (Deref @ "oms_setResultFile", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: cref }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: filename }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::INTEGER { integer: bufferSize }, tail: Deref @ metamodelica::ListNode::Nil } } }) => {
            let mut status: i32;
            status = OMSimulatorExt::oms_setResultFile(cref.clone(), filename.clone(), bufferSize.clone());
            metamodelica::Ref::new(Values::Value::INTEGER { integer: status })
        },
        (Deref @ "oms_setSignalFilter", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: cref }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: regex }, tail: Deref @ metamodelica::ListNode::Nil } }) => {
            let mut status: i32;
            status = OMSimulatorExt::oms_setSignalFilter(cref.clone(), regex.clone());
            metamodelica::Ref::new(Values::Value::INTEGER { integer: status })
        },
        (Deref @ "oms_setSolver", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: cref }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::ENUM_LITERAL { index: solver, .. }, tail: Deref @ metamodelica::ListNode::Nil } }) => {
            let mut status: i32;
            status = OMSimulatorExt::oms_setSolver(cref.clone(), solver.clone() - 1);
            metamodelica::Ref::new(Values::Value::INTEGER { integer: status })
        },
        (Deref @ "oms_setStartTime", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: cref }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::REAL { real: startTime }, tail: Deref @ metamodelica::ListNode::Nil } }) => {
            let mut status: i32;
            status = OMSimulatorExt::oms_setStartTime(cref.clone(), startTime.clone());
            metamodelica::Ref::new(Values::Value::INTEGER { integer: status })
        },
        (Deref @ "oms_setStopTime", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: cref }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::REAL { real: stopTime }, tail: Deref @ metamodelica::ListNode::Nil } }) => {
            let mut status: i32;
            status = OMSimulatorExt::oms_setStopTime(cref.clone(), stopTime.clone());
            metamodelica::Ref::new(Values::Value::INTEGER { integer: status })
        },
        (Deref @ "oms_setTempDirectory", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: newTempDir }, tail: Deref @ metamodelica::ListNode::Nil }) => {
            let mut status: i32;
            status = OMSimulatorExt::oms_setTempDirectory(newTempDir.clone());
            metamodelica::Ref::new(Values::Value::INTEGER { integer: status })
        },
        (Deref @ "oms_setTLMPositionAndOrientation", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: cref }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::REAL { real: x1 }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::REAL { real: x2 }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::REAL { real: x3 }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::REAL { real: A11 }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::REAL { real: A12 }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::REAL { real: A13 }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::REAL { real: A21 }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::REAL { real: A22 }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::REAL { real: A23 }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::REAL { real: A31 }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::REAL { real: A32 }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::REAL { real: A33 }, tail: Deref @ metamodelica::ListNode::Nil } } } } } } } } } } } } }) => {
            let mut status: i32;
            status = OMSimulatorExt::oms_setTLMPositionAndOrientation(cref.clone(), x1.clone(), x2.clone(), x3.clone(), A11.clone(), A12.clone(), A13.clone(), A21.clone(), A22.clone(), A23.clone(), A31.clone(), A32.clone(), A33.clone());
            metamodelica::Ref::new(Values::Value::INTEGER { integer: status })
        },
        (Deref @ "oms_setTLMSocketData", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: cref }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: address }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::INTEGER { integer: managerPort }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::INTEGER { integer: monitorPort }, tail: Deref @ metamodelica::ListNode::Nil } } } }) => {
            let mut status: i32;
            status = OMSimulatorExt::oms_setTLMSocketData(cref.clone(), address.clone(), managerPort.clone(), monitorPort.clone());
            metamodelica::Ref::new(Values::Value::INTEGER { integer: status })
        },
        (Deref @ "oms_setTolerance", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: cref }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::REAL { real: absoluteTolerance }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::REAL { real: relativeTolerance }, tail: Deref @ metamodelica::ListNode::Nil } } }) => {
            let mut status: i32;
            status = OMSimulatorExt::oms_setTolerance(cref.clone(), absoluteTolerance.clone(), relativeTolerance.clone());
            metamodelica::Ref::new(Values::Value::INTEGER { integer: status })
        },
        (Deref @ "oms_setVariableStepSize", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: cref }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::REAL { real: initialStepSize }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::REAL { real: minimumStepSize }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::REAL { real: maximumStepSize }, tail: Deref @ metamodelica::ListNode::Nil } } } }) => {
            let mut status: i32;
            status = OMSimulatorExt::oms_setVariableStepSize(cref.clone(), initialStepSize.clone(), minimumStepSize.clone(), maximumStepSize.clone());
            metamodelica::Ref::new(Values::Value::INTEGER { integer: status })
        },
        (Deref @ "oms_setWorkingDirectory", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: newWorkingDir }, tail: Deref @ metamodelica::ListNode::Nil }) => {
            let mut status: i32;
            status = OMSimulatorExt::oms_setWorkingDirectory(newWorkingDir.clone());
            metamodelica::Ref::new(Values::Value::INTEGER { integer: status })
        },
        (Deref @ "oms_simulate", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: cref }, tail: Deref @ metamodelica::ListNode::Nil }) => {
            let mut status: i32;
            status = OMSimulatorExt::oms_simulate(cref.clone());
            metamodelica::Ref::new(Values::Value::INTEGER { integer: status })
        },
        (Deref @ "oms_stepUntil", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: cref }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::REAL { real: stopTime }, tail: Deref @ metamodelica::ListNode::Nil } }) => {
            let mut status: i32;
            status = OMSimulatorExt::oms_stepUntil(cref.clone(), stopTime.clone());
            metamodelica::Ref::new(Values::Value::INTEGER { integer: status })
        },
        (Deref @ "oms_terminate", Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::STRING { string: cref }, tail: Deref @ metamodelica::ListNode::Nil }) => {
            let mut status: i32;
            status = OMSimulatorExt::oms_terminate(cref.clone());
            metamodelica::Ref::new(Values::Value::INTEGER { integer: status })
        },
        (Deref @ "oms_getVersion", Deref @ metamodelica::ListNode::Nil) => {
            let mut version: ArcStr;
            version = OMSimulatorExt::oms_getVersion();
            metamodelica::Ref::new(Values::Value::STRING { string: version })
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok(outValue)
}
