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

use crate::Flags;
use openmodelica_util_datatypes_basic::List;

#[derive(Clone, Debug, Eq, Hash, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub struct Info {
    pub fmiVersion: ArcStr,
    pub fmiType: i32,
    pub fmiModelName: ArcStr,
    pub fmiModelIdentifier: ArcStr,
    pub fmiGuid: ArcStr,
    pub fmiDescription: ArcStr,
    pub fmiGenerationTool: ArcStr,
    pub fmiGenerationDateAndTime: ArcStr,
    pub fmiVariableNamingConvention: ArcStr,
    pub fmiNumberOfContinuousStates: metamodelica::List<i32>,
    pub fmiNumberOfEventIndicators: metamodelica::List<i32>,
}

impl metamodelica::gc::MMTrace for Info {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        metamodelica::gc::MMTrace::mm_accept(&self.fmiVersion, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.fmiType, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.fmiModelName, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.fmiModelIdentifier, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.fmiGuid, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.fmiDescription, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.fmiGenerationTool, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.fmiGenerationDateAndTime, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.fmiVariableNamingConvention, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.fmiNumberOfContinuousStates, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.fmiNumberOfEventIndicators, __mmv)?;
        Ok(())
    }
}
impl Default for Info {
    fn default() -> Self {
        Self {
            fmiVersion: Default::default(),
            fmiType: Default::default(),
            fmiModelName: Default::default(),
            fmiModelIdentifier: Default::default(),
            fmiGuid: Default::default(),
            fmiDescription: Default::default(),
            fmiGenerationTool: Default::default(),
            fmiGenerationDateAndTime: Default::default(),
            fmiVariableNamingConvention: Default::default(),
            fmiNumberOfContinuousStates: Default::default(),
            fmiNumberOfEventIndicators: Default::default(),
        }
    }
}

pub type INFO = Info;

#[derive(Clone, Debug, Eq, Hash, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub struct TypeDefinitions {
    pub name: ArcStr,
    pub description: ArcStr,
    pub quantity: ArcStr,
    pub min: i32,
    pub max: i32,
    pub items: metamodelica::List<EnumerationItem>,
}

impl metamodelica::gc::MMTrace for TypeDefinitions {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        metamodelica::gc::MMTrace::mm_accept(&self.name, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.description, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.quantity, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.min, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.max, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.items, __mmv)?;
        Ok(())
    }
}
pub type ENUMERATIONTYPE = TypeDefinitions;

#[derive(Clone, Debug, Eq, Hash, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub struct EnumerationItem {
    pub name: ArcStr,
    pub description: ArcStr,
}

impl metamodelica::gc::MMTrace for EnumerationItem {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        metamodelica::gc::MMTrace::mm_accept(&self.name, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.description, __mmv)?;
        Ok(())
    }
}
pub type ENUMERATIONITEM = EnumerationItem;

#[derive(Clone, Copy, Debug, Eq, Hash, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub struct ExperimentAnnotation {
    pub fmiExperimentStartTime: metamodelica::Real,
    pub fmiExperimentStopTime: metamodelica::Real,
    pub fmiExperimentTolerance: metamodelica::Real,
}

impl metamodelica::gc::MMTrace for ExperimentAnnotation {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        metamodelica::gc::MMTrace::mm_accept(&self.fmiExperimentStartTime, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.fmiExperimentStopTime, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.fmiExperimentTolerance, __mmv)?;
        Ok(())
    }
}
impl Default for ExperimentAnnotation {
    fn default() -> Self {
        Self {
            fmiExperimentStartTime: Default::default(),
            fmiExperimentStopTime: Default::default(),
            fmiExperimentTolerance: Default::default(),
        }
    }
}

pub type EXPERIMENTANNOTATION = ExperimentAnnotation;

#[derive(Clone, Debug, Eq, Hash, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub enum ModelVariables {
    REALVARIABLE {
        instance: i32,
        name: ArcStr,
        description: ArcStr,
        baseType: ArcStr,
        variability: ArcStr,
        causality: ArcStr,
        hasStartValue: bool,
        startValue: metamodelica::Real,
        isFixed: bool,
        valueReference: metamodelica::Real,
        x1Placement: i32,
        x2Placement: i32,
        y1Placement: i32,
        y2Placement: i32,
    },
    INTEGERVARIABLE {
        instance: i32,
        name: ArcStr,
        description: ArcStr,
        baseType: ArcStr,
        variability: ArcStr,
        causality: ArcStr,
        hasStartValue: bool,
        startValue: i32,
        isFixed: bool,
        valueReference: metamodelica::Real,
        x1Placement: i32,
        x2Placement: i32,
        y1Placement: i32,
        y2Placement: i32,
    },
    BOOLEANVARIABLE {
        instance: i32,
        name: ArcStr,
        description: ArcStr,
        baseType: ArcStr,
        variability: ArcStr,
        causality: ArcStr,
        hasStartValue: bool,
        startValue: bool,
        isFixed: bool,
        valueReference: metamodelica::Real,
        x1Placement: i32,
        x2Placement: i32,
        y1Placement: i32,
        y2Placement: i32,
    },
    STRINGVARIABLE {
        instance: i32,
        name: ArcStr,
        description: ArcStr,
        baseType: ArcStr,
        variability: ArcStr,
        causality: ArcStr,
        hasStartValue: bool,
        startValue: ArcStr,
        isFixed: bool,
        valueReference: metamodelica::Real,
        x1Placement: i32,
        x2Placement: i32,
        y1Placement: i32,
        y2Placement: i32,
    },
    ENUMERATIONVARIABLE {
        instance: i32,
        name: ArcStr,
        description: ArcStr,
        baseType: ArcStr,
        variability: ArcStr,
        causality: ArcStr,
        hasStartValue: bool,
        startValue: i32,
        isFixed: bool,
        valueReference: metamodelica::Real,
        x1Placement: i32,
        x2Placement: i32,
        y1Placement: i32,
        y2Placement: i32,
    },
    /// an FMI 3.0 Float32 or Float64 variable
    FMI3REALVARIABLE {
        instance: i32,
        name: ArcStr,
        description: ArcStr,
        baseType: ArcStr,
        /// Float32 or Float64
        fmiType: ArcStr,
        variability: ArcStr,
        causality: ArcStr,
        hasStartValue: bool,
        /// one element for a scalar, one per element for an array
        startValue: metamodelica::List<metamodelica::Real>,
        isFixed: bool,
        valueReference: i32,
        /// empty for a scalar
        dimensions: metamodelica::List<i32>,
        x1Placement: i32,
        x2Placement: i32,
        y1Placement: i32,
        y2Placement: i32,
    },
    /// an FMI 3.0 Int8/16/32/64 or UInt8/16/32/64 variable
    FMI3INTEGERVARIABLE {
        instance: i32,
        name: ArcStr,
        description: ArcStr,
        baseType: ArcStr,
        /// Int8, UInt8, Int16, ... Int64, UInt64
        fmiType: ArcStr,
        variability: ArcStr,
        causality: ArcStr,
        hasStartValue: bool,
        startValue: metamodelica::List<i32>,
        isFixed: bool,
        valueReference: i32,
        dimensions: metamodelica::List<i32>,
        x1Placement: i32,
        x2Placement: i32,
        y1Placement: i32,
        y2Placement: i32,
    },
    /// an FMI 3.0 Boolean variable
    FMI3BOOLEANVARIABLE {
        instance: i32,
        name: ArcStr,
        description: ArcStr,
        baseType: ArcStr,
        fmiType: ArcStr,
        variability: ArcStr,
        causality: ArcStr,
        hasStartValue: bool,
        startValue: metamodelica::List<bool>,
        isFixed: bool,
        valueReference: i32,
        dimensions: metamodelica::List<i32>,
        x1Placement: i32,
        x2Placement: i32,
        y1Placement: i32,
        y2Placement: i32,
    },
    /// an FMI 3.0 String variable
    FMI3STRINGVARIABLE {
        instance: i32,
        name: ArcStr,
        description: ArcStr,
        baseType: ArcStr,
        fmiType: ArcStr,
        variability: ArcStr,
        causality: ArcStr,
        hasStartValue: bool,
        startValue: metamodelica::List<ArcStr>,
        isFixed: bool,
        valueReference: i32,
        dimensions: metamodelica::List<i32>,
        x1Placement: i32,
        x2Placement: i32,
        y1Placement: i32,
        y2Placement: i32,
    },
    /// an FMI 3.0 Binary variable, which Modelica has no type for
    FMI3BINARYVARIABLE {
        instance: i32,
        name: ArcStr,
        description: ArcStr,
        baseType: ArcStr,
        fmiType: ArcStr,
        variability: ArcStr,
        causality: ArcStr,
        hasStartValue: bool,
        /// the start attribute, which FMI 3.0 writes as hex
        startValue: metamodelica::List<ArcStr>,
        isFixed: bool,
        valueReference: i32,
        dimensions: metamodelica::List<i32>,
        mimeType: ArcStr,
        /// 0 when the FMU did not say
        maxSize: i32,
        x1Placement: i32,
        x2Placement: i32,
        y1Placement: i32,
        y2Placement: i32,
    },
    /// an FMI 3.0 Clock variable
    FMI3CLOCKVARIABLE {
        instance: i32,
        name: ArcStr,
        description: ArcStr,
        baseType: ArcStr,
        fmiType: ArcStr,
        variability: ArcStr,
        causality: ArcStr,
        hasStartValue: bool,
        isFixed: bool,
        valueReference: i32,
        dimensions: metamodelica::List<i32>,
        intervalVariability: ArcStr,
        /// 0.0 when the FMU did not say
        intervalDecimal: metamodelica::Real,
        hasIntervalDecimal: bool,
        x1Placement: i32,
        x2Placement: i32,
        y1Placement: i32,
        y2Placement: i32,
    },
    /// an FMI 3.0 Enumeration variable
    FMI3ENUMERATIONVARIABLE {
        instance: i32,
        name: ArcStr,
        description: ArcStr,
        baseType: ArcStr,
        fmiType: ArcStr,
        variability: ArcStr,
        causality: ArcStr,
        hasStartValue: bool,
        startValue: metamodelica::List<i32>,
        isFixed: bool,
        valueReference: i32,
        dimensions: metamodelica::List<i32>,
        declaredType: ArcStr,
        x1Placement: i32,
        x2Placement: i32,
        y1Placement: i32,
        y2Placement: i32,
    },
}
impl metamodelica::gc::MMTrace for ModelVariables {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        match self {
            ModelVariables::REALVARIABLE {
                instance,
                name,
                description,
                baseType,
                variability,
                causality,
                hasStartValue,
                startValue,
                isFixed,
                valueReference,
                x1Placement,
                x2Placement,
                y1Placement,
                y2Placement,
            } => {
                metamodelica::gc::MMTrace::mm_accept(instance, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(name, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(description, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(baseType, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(variability, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(causality, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(hasStartValue, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(startValue, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(isFixed, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(valueReference, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(x1Placement, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(x2Placement, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(y1Placement, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(y2Placement, __mmv)?;
                Ok(())
            }
            ModelVariables::INTEGERVARIABLE {
                instance,
                name,
                description,
                baseType,
                variability,
                causality,
                hasStartValue,
                startValue,
                isFixed,
                valueReference,
                x1Placement,
                x2Placement,
                y1Placement,
                y2Placement,
            } => {
                metamodelica::gc::MMTrace::mm_accept(instance, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(name, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(description, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(baseType, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(variability, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(causality, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(hasStartValue, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(startValue, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(isFixed, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(valueReference, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(x1Placement, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(x2Placement, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(y1Placement, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(y2Placement, __mmv)?;
                Ok(())
            }
            ModelVariables::BOOLEANVARIABLE {
                instance,
                name,
                description,
                baseType,
                variability,
                causality,
                hasStartValue,
                startValue,
                isFixed,
                valueReference,
                x1Placement,
                x2Placement,
                y1Placement,
                y2Placement,
            } => {
                metamodelica::gc::MMTrace::mm_accept(instance, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(name, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(description, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(baseType, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(variability, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(causality, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(hasStartValue, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(startValue, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(isFixed, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(valueReference, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(x1Placement, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(x2Placement, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(y1Placement, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(y2Placement, __mmv)?;
                Ok(())
            }
            ModelVariables::STRINGVARIABLE {
                instance,
                name,
                description,
                baseType,
                variability,
                causality,
                hasStartValue,
                startValue,
                isFixed,
                valueReference,
                x1Placement,
                x2Placement,
                y1Placement,
                y2Placement,
            } => {
                metamodelica::gc::MMTrace::mm_accept(instance, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(name, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(description, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(baseType, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(variability, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(causality, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(hasStartValue, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(startValue, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(isFixed, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(valueReference, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(x1Placement, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(x2Placement, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(y1Placement, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(y2Placement, __mmv)?;
                Ok(())
            }
            ModelVariables::ENUMERATIONVARIABLE {
                instance,
                name,
                description,
                baseType,
                variability,
                causality,
                hasStartValue,
                startValue,
                isFixed,
                valueReference,
                x1Placement,
                x2Placement,
                y1Placement,
                y2Placement,
            } => {
                metamodelica::gc::MMTrace::mm_accept(instance, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(name, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(description, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(baseType, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(variability, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(causality, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(hasStartValue, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(startValue, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(isFixed, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(valueReference, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(x1Placement, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(x2Placement, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(y1Placement, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(y2Placement, __mmv)?;
                Ok(())
            }
            ModelVariables::FMI3REALVARIABLE {
                instance,
                name,
                description,
                baseType,
                fmiType,
                variability,
                causality,
                hasStartValue,
                startValue,
                isFixed,
                valueReference,
                dimensions,
                x1Placement,
                x2Placement,
                y1Placement,
                y2Placement,
            } => {
                metamodelica::gc::MMTrace::mm_accept(instance, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(name, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(description, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(baseType, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(fmiType, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(variability, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(causality, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(hasStartValue, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(startValue, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(isFixed, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(valueReference, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(dimensions, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(x1Placement, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(x2Placement, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(y1Placement, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(y2Placement, __mmv)?;
                Ok(())
            }
            ModelVariables::FMI3INTEGERVARIABLE {
                instance,
                name,
                description,
                baseType,
                fmiType,
                variability,
                causality,
                hasStartValue,
                startValue,
                isFixed,
                valueReference,
                dimensions,
                x1Placement,
                x2Placement,
                y1Placement,
                y2Placement,
            } => {
                metamodelica::gc::MMTrace::mm_accept(instance, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(name, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(description, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(baseType, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(fmiType, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(variability, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(causality, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(hasStartValue, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(startValue, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(isFixed, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(valueReference, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(dimensions, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(x1Placement, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(x2Placement, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(y1Placement, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(y2Placement, __mmv)?;
                Ok(())
            }
            ModelVariables::FMI3BOOLEANVARIABLE {
                instance,
                name,
                description,
                baseType,
                fmiType,
                variability,
                causality,
                hasStartValue,
                startValue,
                isFixed,
                valueReference,
                dimensions,
                x1Placement,
                x2Placement,
                y1Placement,
                y2Placement,
            } => {
                metamodelica::gc::MMTrace::mm_accept(instance, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(name, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(description, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(baseType, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(fmiType, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(variability, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(causality, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(hasStartValue, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(startValue, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(isFixed, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(valueReference, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(dimensions, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(x1Placement, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(x2Placement, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(y1Placement, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(y2Placement, __mmv)?;
                Ok(())
            }
            ModelVariables::FMI3STRINGVARIABLE {
                instance,
                name,
                description,
                baseType,
                fmiType,
                variability,
                causality,
                hasStartValue,
                startValue,
                isFixed,
                valueReference,
                dimensions,
                x1Placement,
                x2Placement,
                y1Placement,
                y2Placement,
            } => {
                metamodelica::gc::MMTrace::mm_accept(instance, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(name, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(description, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(baseType, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(fmiType, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(variability, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(causality, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(hasStartValue, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(startValue, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(isFixed, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(valueReference, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(dimensions, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(x1Placement, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(x2Placement, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(y1Placement, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(y2Placement, __mmv)?;
                Ok(())
            }
            ModelVariables::FMI3BINARYVARIABLE {
                instance,
                name,
                description,
                baseType,
                fmiType,
                variability,
                causality,
                hasStartValue,
                startValue,
                isFixed,
                valueReference,
                dimensions,
                mimeType,
                maxSize,
                x1Placement,
                x2Placement,
                y1Placement,
                y2Placement,
            } => {
                metamodelica::gc::MMTrace::mm_accept(instance, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(name, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(description, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(baseType, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(fmiType, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(variability, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(causality, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(hasStartValue, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(startValue, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(isFixed, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(valueReference, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(dimensions, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(mimeType, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(maxSize, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(x1Placement, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(x2Placement, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(y1Placement, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(y2Placement, __mmv)?;
                Ok(())
            }
            ModelVariables::FMI3CLOCKVARIABLE {
                instance,
                name,
                description,
                baseType,
                fmiType,
                variability,
                causality,
                hasStartValue,
                isFixed,
                valueReference,
                dimensions,
                intervalVariability,
                intervalDecimal,
                hasIntervalDecimal,
                x1Placement,
                x2Placement,
                y1Placement,
                y2Placement,
            } => {
                metamodelica::gc::MMTrace::mm_accept(instance, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(name, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(description, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(baseType, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(fmiType, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(variability, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(causality, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(hasStartValue, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(isFixed, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(valueReference, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(dimensions, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(intervalVariability, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(intervalDecimal, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(hasIntervalDecimal, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(x1Placement, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(x2Placement, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(y1Placement, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(y2Placement, __mmv)?;
                Ok(())
            }
            ModelVariables::FMI3ENUMERATIONVARIABLE {
                instance,
                name,
                description,
                baseType,
                fmiType,
                variability,
                causality,
                hasStartValue,
                startValue,
                isFixed,
                valueReference,
                dimensions,
                declaredType,
                x1Placement,
                x2Placement,
                y1Placement,
                y2Placement,
            } => {
                metamodelica::gc::MMTrace::mm_accept(instance, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(name, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(description, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(baseType, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(fmiType, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(variability, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(causality, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(hasStartValue, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(startValue, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(isFixed, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(valueReference, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(dimensions, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(declaredType, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(x1Placement, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(x2Placement, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(y1Placement, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(y2Placement, __mmv)?;
                Ok(())
            }
        }
    }
}
pub use self::ModelVariables::{
    BOOLEANVARIABLE, ENUMERATIONVARIABLE, FMI3BINARYVARIABLE, FMI3BOOLEANVARIABLE, FMI3CLOCKVARIABLE,
    FMI3ENUMERATIONVARIABLE, FMI3INTEGERVARIABLE, FMI3REALVARIABLE, FMI3STRINGVARIABLE, INTEGERVARIABLE, REALVARIABLE,
    STRINGVARIABLE,
};

#[derive(Clone, Debug, Eq, Hash, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub struct FmiImport {
    pub platform: ArcStr,
    pub fmuFileName: ArcStr,
    pub fmuWorkingDirectory: ArcStr,
    pub fmiLogLevel: i32,
    pub fmiDebugOutput: bool,
    pub fmiContext: Option<i32>,
    pub fmiInstance: Option<i32>,
    pub fmiInfo: Info,
    pub fmiTypeDefinitionsList: metamodelica::List<TypeDefinitions>,
    pub fmiExperimentAnnotation: ExperimentAnnotation,
    pub fmiModelVariablesInstance: Option<i32>,
    pub fmiModelVariablesList: metamodelica::List<ModelVariables>,
    pub generateInputConnectors: bool,
    pub generateOutputConnectors: bool,
}

impl metamodelica::gc::MMTrace for FmiImport {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        metamodelica::gc::MMTrace::mm_accept(&self.platform, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.fmuFileName, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.fmuWorkingDirectory, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.fmiLogLevel, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.fmiDebugOutput, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.fmiContext, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.fmiInstance, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.fmiInfo, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.fmiTypeDefinitionsList, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.fmiExperimentAnnotation, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.fmiModelVariablesInstance, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.fmiModelVariablesList, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.generateInputConnectors, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.generateOutputConnectors, __mmv)?;
        Ok(())
    }
}
pub type FMIIMPORT = FmiImport;

pub fn getFMIModelIdentifier(mut inFMIInfo: &Info) -> ArcStr {
    let mut fmiModelIdentifier: ArcStr;
    fmiModelIdentifier = (match inFMIInfo.clone() {
        Info {
            fmiModelIdentifier: mut modelIdentifier,
            ..
        } => modelIdentifier.clone(),
    });
    fmiModelIdentifier
}

pub fn getFMIType(mut inFMIInfo: &Info) -> ArcStr {
    let mut fmiType: ArcStr;
    fmiType = (::match_deref::match_deref! { match &(inFMIInfo) {
        Info { fmiVersion: Deref @ "1.0", fmiType: 0, .. } => literal!("me"),
        Info { fmiVersion: Deref @ "1.0", fmiType: 1, .. } => literal!("cs_st"),
        Info { fmiVersion: Deref @ "1.0", fmiType: 2, .. } => literal!("cs_tool"),
        Info { fmiVersion: Deref @ "2.0", fmiType: 1, .. } => literal!("me"),
        Info { fmiVersion: Deref @ "2.0", fmiType: 2, .. } => literal!("cs"),
        Info { fmiVersion: Deref @ "2.0", fmiType: 3, .. } => literal!("me_cs"),
        Info { fmiVersion: Deref @ "3.0", fmiType: 2, .. } => literal!("me"),
        Info { fmiVersion: Deref @ "3.0", fmiType: 4, .. } => literal!("cs"),
        Info { fmiVersion: Deref @ "3.0", fmiType: 8, .. } => literal!("se"),
        _ => literal!(""),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    fmiType
}

pub fn getFMIVersion(mut inFMIInfo: &Info) -> ArcStr {
    let mut fmiVersion: ArcStr;
    fmiVersion = (match inFMIInfo.clone() {
        Info {
            fmiVersion: mut version,
            ..
        } => version.clone(),
    });
    fmiVersion
}

pub fn checkFMIVersion(mut inFMIVersion: &ArcStr) -> bool {
    let mut success: bool;
    success = (::match_deref::match_deref! { match &(inFMIVersion.clone()) {
        Deref @ "1.0" => true,
        Deref @ "2.0" => true,
        Deref @ "3.0" => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    success
}

pub fn isFMIVersion10(mut inFMUVersion: &ArcStr) -> bool {
    let mut success: bool;
    success = (::match_deref::match_deref! { match &(inFMUVersion.clone()) {
        Deref @ "1.0" => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    success
}

pub fn isFMIVersion20(mut inFMUVersion: &ArcStr) -> Result<bool> {
    let mut success: bool;
    success = (::match_deref::match_deref! { match &(inFMUVersion.clone()) {
        Deref @ "2.0" => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(success)
}

pub fn isFMIVersion30(mut inFMUVersion: &ArcStr) -> Result<bool> {
    let mut success: bool;
    success = (::match_deref::match_deref! { match &(inFMUVersion.clone()) {
        Deref @ "3.0" => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(success)
}

pub fn getFMIVersionString() -> Result<ArcStr> {
    let mut version: ArcStr = Flags::getConfigString(Flags::FMI_VERSION.clone())?;
    Ok(version)
}

pub fn checkFMIType(mut inFMIType: &ArcStr) -> bool {
    let mut success: bool;
    success = (::match_deref::match_deref! { match &(inFMIType.clone()) {
        Deref @ "me" => true,
        Deref @ "cs" => true,
        Deref @ "me_cs" => true,
        Deref @ "se" => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    success
}

pub fn canExportFMU(mut inFMUVersion: &ArcStr, mut inFMIType: &ArcStr) -> bool {
    let mut success: bool;
    success = (::match_deref::match_deref! { match &((inFMUVersion.clone(), inFMIType.clone())) {
        (Deref @ "1.0", Deref @ "me") => true,
        (Deref @ "2.0", Deref @ "me") => true,
        (Deref @ "2.0", Deref @ "cs") => true,
        (Deref @ "2.0", Deref @ "me_cs") => true,
        (Deref @ "3.0", Deref @ "me") => true,
        (Deref @ "3.0", Deref @ "cs") => true,
        (Deref @ "3.0", Deref @ "me_cs") => true,
        (Deref @ "3.0", Deref @ "se") => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    success
}

pub fn isFMIMEType(mut inFMIType: &ArcStr) -> bool {
    let mut success: bool;
    success = (::match_deref::match_deref! { match &(inFMIType.clone()) {
        Deref @ "me" => true,
        Deref @ "me_cs" => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    success
}

pub fn isFMICSType(mut inFMIType: &ArcStr) -> bool {
    let mut success: bool;
    success = (::match_deref::match_deref! { match &(inFMIType.clone()) {
        Deref @ "cs" => true,
        Deref @ "me_cs" => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    success
}

pub fn isFMISEType(mut inFMIType: &ArcStr) -> bool {
    let mut success: bool;
    success = (::match_deref::match_deref! { match &(inFMIType.clone()) {
        Deref @ "se" => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    success
}

pub fn getEnumerationTypeFromTypes(
    mut inTypeDefinitionsList: metamodelica::List<TypeDefinitions>,
    mut inBaseType: ArcStr,
) -> Result<ArcStr> {
    '__tco: loop {
        ::match_deref::match_deref! { match &((inTypeDefinitionsList, inBaseType)) {
            (Deref @ metamodelica::ListNode::Cons { head: TypeDefinitions { name: name_, .. }, tail: _ }, baseType) if (stringEqual(&name_, &baseType)) => {
                return Ok(name_.clone())
            },
            (Deref @ metamodelica::ListNode::Cons { head: _, tail: xs }, baseType) => {
                let mut name_: ArcStr;
                { (inTypeDefinitionsList, inBaseType) = (xs.clone(), baseType.clone()); continue '__tco; }
            },
            (Deref @ metamodelica::ListNode::Nil, _) => {
                return Ok(literal!(""))
            },
            _ => return Err("match: no arm matched"),
        } }
    }
}

pub fn filterModelVariables(
    mut inModelVariables: metamodelica::List<ModelVariables>,
    mut tipe: ArcStr,
    mut variableCausality: ArcStr,
) -> Result<metamodelica::List<ModelVariables>> {
    let mut outModelVariables: metamodelica::List<ModelVariables>;
    outModelVariables = List::filter2OnTrue(
        inModelVariables,
        (std::sync::Arc::new(
            move |__a0: ModelVariables, __a1: ArcStr, __a2: ArcStr| -> metamodelica::Result<_> {
                ::std::result::Result::Ok(filterModelVariable(&__a0, &__a1, &__a2))
            },
        ) as std::sync::Arc<dyn ::std::ops::Fn(ModelVariables, ArcStr, ArcStr) -> Result<bool> + 'static>),
        tipe,
        variableCausality,
    )?;
    Ok(outModelVariables)
}

fn filterModelVariable(mut modelVar: &ModelVariables, mut tipe: &ArcStr, mut variableCausality: &ArcStr) -> bool {
    let mut result: bool;
    result = (match modelVar.clone() {
        ModelVariables::REALVARIABLE {
            causality: mut causality,
            ..
        } if (metamodelica::stringEq(&tipe, &(literal!("real")))
            && metamodelica::stringEq(&causality, &variableCausality)) =>
        {
            true
        }
        ModelVariables::INTEGERVARIABLE {
            causality: mut causality,
            ..
        } if (metamodelica::stringEq(&tipe, &(literal!("integer")))
            && metamodelica::stringEq(&causality, &variableCausality)) =>
        {
            true
        }
        ModelVariables::BOOLEANVARIABLE {
            causality: mut causality,
            ..
        } if (metamodelica::stringEq(&tipe, &(literal!("boolean")))
            && metamodelica::stringEq(&causality, &variableCausality)) =>
        {
            true
        }
        ModelVariables::STRINGVARIABLE {
            causality: mut causality,
            ..
        } if (metamodelica::stringEq(&tipe, &(literal!("string")))
            && metamodelica::stringEq(&causality, &variableCausality)) =>
        {
            true
        }
        ModelVariables::FMI3REALVARIABLE {
            causality: mut causality,
            ..
        } if (metamodelica::stringEq(&tipe, &(literal!("real")))
            && metamodelica::stringEq(&causality, &variableCausality)) =>
        {
            true
        }
        ModelVariables::FMI3INTEGERVARIABLE {
            causality: mut causality,
            ..
        } if (metamodelica::stringEq(&tipe, &(literal!("integer")))
            && metamodelica::stringEq(&causality, &variableCausality)) =>
        {
            true
        }
        ModelVariables::FMI3BOOLEANVARIABLE {
            causality: mut causality,
            ..
        } if (metamodelica::stringEq(&tipe, &(literal!("boolean")))
            && metamodelica::stringEq(&causality, &variableCausality)) =>
        {
            true
        }
        ModelVariables::FMI3STRINGVARIABLE {
            causality: mut causality,
            ..
        } if (metamodelica::stringEq(&tipe, &(literal!("string")))
            && metamodelica::stringEq(&causality, &variableCausality)) =>
        {
            true
        }
        _ => false,
    });
    result
}
