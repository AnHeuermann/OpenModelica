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

use openmodelica_backend_types::BackendDAE;
use openmodelica_frontend_types::DAE;

// public imports
/// Container for metadata about variables in a Modelica model.
#[derive(Clone, Debug, Eq, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub struct SimVars {
    pub stateVars: metamodelica::List<metamodelica::Ref<SimVar>>,
    pub derivativeVars: metamodelica::List<metamodelica::Ref<SimVar>>,
    pub algVars: metamodelica::List<metamodelica::Ref<SimVar>>,
    pub discreteAlgVars: metamodelica::List<metamodelica::Ref<SimVar>>,
    pub intAlgVars: metamodelica::List<metamodelica::Ref<SimVar>>,
    pub boolAlgVars: metamodelica::List<metamodelica::Ref<SimVar>>,
    pub inputVars: metamodelica::List<metamodelica::Ref<SimVar>>,
    pub outputVars: metamodelica::List<metamodelica::Ref<SimVar>>,
    pub aliasVars: metamodelica::List<metamodelica::Ref<SimVar>>,
    pub intAliasVars: metamodelica::List<metamodelica::Ref<SimVar>>,
    pub boolAliasVars: metamodelica::List<metamodelica::Ref<SimVar>>,
    pub paramVars: metamodelica::List<metamodelica::Ref<SimVar>>,
    pub intParamVars: metamodelica::List<metamodelica::Ref<SimVar>>,
    pub boolParamVars: metamodelica::List<metamodelica::Ref<SimVar>>,
    pub stringAlgVars: metamodelica::List<metamodelica::Ref<SimVar>>,
    pub stringParamVars: metamodelica::List<metamodelica::Ref<SimVar>>,
    pub stringAliasVars: metamodelica::List<metamodelica::Ref<SimVar>>,
    pub extObjVars: metamodelica::List<metamodelica::Ref<SimVar>>,
    pub constVars: metamodelica::List<metamodelica::Ref<SimVar>>,
    pub intConstVars: metamodelica::List<metamodelica::Ref<SimVar>>,
    pub boolConstVars: metamodelica::List<metamodelica::Ref<SimVar>>,
    pub stringConstVars: metamodelica::List<metamodelica::Ref<SimVar>>,
    pub jacobianVars: metamodelica::List<metamodelica::Ref<SimVar>>,
    pub seedVars: metamodelica::List<metamodelica::Ref<SimVar>>,
    pub realOptimizeConstraintsVars: metamodelica::List<metamodelica::Ref<SimVar>>,
    pub realOptimizeFinalConstraintsVars: metamodelica::List<metamodelica::Ref<SimVar>>,
    /// variable used to calculate sensitivities for parameters nSensitivitityParameters + nRealParam*nStates
    pub sensitivityVars: metamodelica::List<metamodelica::Ref<SimVar>>,
    pub dataReconSetcVars: metamodelica::List<metamodelica::Ref<SimVar>>,
    pub dataReconinputVars: metamodelica::List<metamodelica::Ref<SimVar>>,
    pub dataReconSetBVars: metamodelica::List<metamodelica::Ref<SimVar>>,
}

impl metamodelica::gc::MMTrace for SimVars {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        metamodelica::gc::MMTrace::mm_accept(&self.stateVars, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.derivativeVars, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.algVars, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.discreteAlgVars, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.intAlgVars, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.boolAlgVars, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.inputVars, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.outputVars, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.aliasVars, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.intAliasVars, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.boolAliasVars, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.paramVars, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.intParamVars, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.boolParamVars, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.stringAlgVars, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.stringParamVars, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.stringAliasVars, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.extObjVars, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.constVars, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.intConstVars, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.boolConstVars, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.stringConstVars, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.jacobianVars, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.seedVars, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.realOptimizeConstraintsVars, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.realOptimizeFinalConstraintsVars, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.sensitivityVars, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.dataReconSetcVars, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.dataReconinputVars, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.dataReconSetBVars, __mmv)?;
        Ok(())
    }
}
impl Default for SimVars {
    fn default() -> Self {
        Self {
            stateVars: Default::default(),
            derivativeVars: Default::default(),
            algVars: Default::default(),
            discreteAlgVars: Default::default(),
            intAlgVars: Default::default(),
            boolAlgVars: Default::default(),
            inputVars: Default::default(),
            outputVars: Default::default(),
            aliasVars: Default::default(),
            intAliasVars: Default::default(),
            boolAliasVars: Default::default(),
            paramVars: Default::default(),
            intParamVars: Default::default(),
            boolParamVars: Default::default(),
            stringAlgVars: Default::default(),
            stringParamVars: Default::default(),
            stringAliasVars: Default::default(),
            extObjVars: Default::default(),
            constVars: Default::default(),
            intConstVars: Default::default(),
            boolConstVars: Default::default(),
            stringConstVars: Default::default(),
            jacobianVars: Default::default(),
            seedVars: Default::default(),
            realOptimizeConstraintsVars: Default::default(),
            realOptimizeFinalConstraintsVars: Default::default(),
            sensitivityVars: Default::default(),
            dataReconSetcVars: Default::default(),
            dataReconinputVars: Default::default(),
            dataReconSetBVars: Default::default(),
        }
    }
}

pub type SIMVARS = SimVars;

thread_local! { static __emptySimVars_TLS: SimVars = SimVars { stateVars: metamodelica::nil(), derivativeVars: metamodelica::nil(), algVars: metamodelica::nil(), discreteAlgVars: metamodelica::nil(), intAlgVars: metamodelica::nil(), boolAlgVars: metamodelica::nil(), inputVars: metamodelica::nil(), outputVars: metamodelica::nil(), aliasVars: metamodelica::nil(), intAliasVars: metamodelica::nil(), boolAliasVars: metamodelica::nil(), paramVars: metamodelica::nil(), intParamVars: metamodelica::nil(), boolParamVars: metamodelica::nil(), stringAlgVars: metamodelica::nil(), stringParamVars: metamodelica::nil(), stringAliasVars: metamodelica::nil(), extObjVars: metamodelica::nil(), constVars: metamodelica::nil(), intConstVars: metamodelica::nil(), boolConstVars: metamodelica::nil(), stringConstVars: metamodelica::nil(), jacobianVars: metamodelica::nil(), seedVars: metamodelica::nil(), realOptimizeConstraintsVars: metamodelica::nil(), realOptimizeFinalConstraintsVars: metamodelica::nil(), sensitivityVars: metamodelica::nil(), dataReconSetcVars: metamodelica::nil(), dataReconinputVars: metamodelica::nil(), dataReconSetBVars: metamodelica::nil() }; }
pub(crate) fn emptySimVars() -> SimVars {
    __emptySimVars_TLS.with(|__t| __t.clone())
}

/// Information about a variable in a Modelica model.
#[derive(Clone, Debug, Eq, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub struct SimVar {
    pub name: metamodelica::Ref<DAE::ComponentRef>,
    pub varKind: BackendDAE::VarKind,
    pub comment: ArcStr,
    pub unit: ArcStr,
    pub displayUnit: ArcStr,
    pub index: i32,
    pub minValue: Option<metamodelica::Ref<DAE::Exp>>,
    pub maxValue: Option<metamodelica::Ref<DAE::Exp>>,
    pub initialValue: Option<metamodelica::Ref<DAE::Exp>>,
    pub nominalValue: Option<metamodelica::Ref<DAE::Exp>>,
    pub isFixed: bool,
    pub type_: metamodelica::Ref<DAE::Type>,
    pub isDiscrete: bool,
    /// the name of the array if this variable is the first in that array
    pub arrayCref: Option<metamodelica::Ref<DAE::ComponentRef>>,
    pub aliasvar: AliasVariable,
    pub source: metamodelica::Ref<DAE::ElementSource>,
    pub causality: Option<Causality>,
    /// valueReference
    pub variable_index: Option<i32>,
    /// index of variable in modelDescription.xml
    pub fmi_index: Option<i32>,
    pub numArrayElement: metamodelica::List<ArcStr>,
    pub isValueChangeable: bool,
    pub isProtected: bool,
    pub hideResult: Option<bool>,
    pub isEncrypted: bool,
    pub inputIndex: Option<metamodelica::Array<i32>>,
    /// true if the variable is a nonlinear jacobian var
    pub initNonlinear: bool,
    /// if the varibale is a jacobian var, this is the corresponding matrix
    pub matrixName: Option<ArcStr>,
    /// FMI-2.0 variabilty attribute
    pub variability: Option<Variability>,
    /// FMI-2.0 initial attribute
    pub initial_: Option<Initial>,
    /// variables will only be exported to the modelDescription.xml if this attribute is SOME(cref) and this cref is only used in ModelDescription.xml for FMI-2.0 export
    pub exportVar: Option<metamodelica::Ref<DAE::ComponentRef>>,
    /// annotation(absoluteValue=false) If false, then the variable defines a relativeQuantity=true else relativeQuantity=false
    pub relativeQuantity: bool,
    /// true if the variable is a flow connector member (FMI 3.0 terminal variableKind inflow/outflow)
    pub isConnectorFlow: bool,
}

impl metamodelica::gc::MMTrace for SimVar {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        metamodelica::gc::MMTrace::mm_accept(&self.name, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.varKind, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.comment, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.unit, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.displayUnit, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.index, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.minValue, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.maxValue, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.initialValue, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.nominalValue, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.isFixed, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.type_, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.isDiscrete, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.arrayCref, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.aliasvar, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.source, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.causality, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.variable_index, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.fmi_index, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.numArrayElement, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.isValueChangeable, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.isProtected, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.hideResult, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.isEncrypted, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.inputIndex, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.initNonlinear, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.matrixName, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.variability, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.initial_, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.exportVar, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.relativeQuantity, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.isConnectorFlow, __mmv)?;
        Ok(())
    }
}
impl Default for SimVar {
    fn default() -> Self {
        Self {
            name: Default::default(),
            varKind: Default::default(),
            comment: Default::default(),
            unit: Default::default(),
            displayUnit: Default::default(),
            index: Default::default(),
            minValue: Default::default(),
            maxValue: Default::default(),
            initialValue: Default::default(),
            nominalValue: Default::default(),
            isFixed: Default::default(),
            type_: Default::default(),
            isDiscrete: Default::default(),
            arrayCref: Default::default(),
            aliasvar: Default::default(),
            source: Default::default(),
            causality: Default::default(),
            variable_index: Default::default(),
            fmi_index: Default::default(),
            numArrayElement: Default::default(),
            isValueChangeable: Default::default(),
            isProtected: Default::default(),
            hideResult: Default::default(),
            isEncrypted: Default::default(),
            inputIndex: Default::default(),
            initNonlinear: Default::default(),
            matrixName: Default::default(),
            variability: Default::default(),
            initial_: Default::default(),
            exportVar: Default::default(),
            relativeQuantity: Default::default(),
            isConnectorFlow: Default::default(),
        }
    }
}

pub type SIMVAR = SimVar;

#[derive(Clone, Debug, Eq, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub enum AliasVariable {
    NOALIAS,
    ALIAS {
        varName: metamodelica::Ref<DAE::ComponentRef>,
    },
    NEGATEDALIAS {
        varName: metamodelica::Ref<DAE::ComponentRef>,
    },
}
impl metamodelica::gc::MMTrace for AliasVariable {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        match self {
            AliasVariable::NOALIAS => Ok(()),
            AliasVariable::ALIAS { varName } => {
                metamodelica::gc::MMTrace::mm_accept(varName, __mmv)?;
                Ok(())
            }
            AliasVariable::NEGATEDALIAS { varName } => {
                metamodelica::gc::MMTrace::mm_accept(varName, __mmv)?;
                Ok(())
            }
        }
    }
}
impl Default for AliasVariable {
    fn default() -> Self {
        Self::NOALIAS
    }
}
pub use self::AliasVariable::{ALIAS, NEGATEDALIAS, NOALIAS};

#[derive(Clone, Copy, Debug, Eq, Hash, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub enum Causality {
    /// needed for FMI-1.0
    NONECAUS,
    OUTPUT,
    INPUT,
    LOCAL,
    PARAMETER,
    CALCULATED_PARAMETER,
}
impl metamodelica::gc::MMTrace for Causality {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        match self {
            Causality::NONECAUS => Ok(()),
            Causality::OUTPUT => Ok(()),
            Causality::INPUT => Ok(()),
            Causality::LOCAL => Ok(()),
            Causality::PARAMETER => Ok(()),
            Causality::CALCULATED_PARAMETER => Ok(()),
        }
    }
}
impl Default for Causality {
    fn default() -> Self {
        Self::NONECAUS
    }
}
pub use self::Causality::{CALCULATED_PARAMETER, INPUT, LOCAL, NONECAUS, OUTPUT, PARAMETER};

#[derive(Clone, Copy, Debug, Eq, Hash, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub enum Initial {
    NONE_INITIAL,
    EXACT,
    APPROX,
    CALCULATED,
}
impl metamodelica::gc::MMTrace for Initial {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        match self {
            Initial::NONE_INITIAL => Ok(()),
            Initial::EXACT => Ok(()),
            Initial::APPROX => Ok(()),
            Initial::CALCULATED => Ok(()),
        }
    }
}
impl Default for Initial {
    fn default() -> Self {
        Self::NONE_INITIAL
    }
}
pub use self::Initial::{APPROX, CALCULATED, EXACT, NONE_INITIAL};

#[derive(Clone, Copy, Debug, Eq, Hash, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub enum Variability {
    CONSTANT,
    FIXED,
    TUNABLE,
    DISCRETE,
    CONTINUOUS,
}
impl metamodelica::gc::MMTrace for Variability {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        match self {
            Variability::CONSTANT => Ok(()),
            Variability::FIXED => Ok(()),
            Variability::TUNABLE => Ok(()),
            Variability::DISCRETE => Ok(()),
            Variability::CONTINUOUS => Ok(()),
        }
    }
}
impl Default for Variability {
    fn default() -> Self {
        Self::CONSTANT
    }
}
pub use self::Variability::{CONSTANT, CONTINUOUS, DISCRETE, FIXED, TUNABLE};
