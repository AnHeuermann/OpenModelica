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

use crate::NBAdjacency as Adjacency;
use crate::NBAlias as Alias;
use crate::NBBindings as Bindings;
use crate::NBCausalize as Causalize;
use crate::NBDAEMode as DAEMode;
use crate::NBDetectStates as DetectStates;
use crate::NBDifferentiate as Differentiate;
use crate::NBEquation as BEquation;
use crate::NBEquation::EqData;
use crate::NBEquation::Equation;
use crate::NBEquation::EquationAttributes;
use crate::NBEquation::EquationKind;
use crate::NBEquation::EquationPointer;
use crate::NBEquation::EquationPointers;
use crate::NBEquation::IfEquationBody;
use crate::NBEquation::Iterator;
use crate::NBEvaluation as Evaluation;
use crate::NBEvents as Events;
use crate::NBFunctionAlias as FunctionAlias;
use crate::NBInitialization as Initialization;
use crate::NBInline as Inline;
use crate::NBJacobian as Jacobian;
use crate::NBJacobian::JacobianType;
use crate::NBModule as Module;
use crate::NBPartition;
use crate::NBPartition::Partition;
use crate::NBPartitioning as Partitioning;
use crate::NBResizable as Resizable;
use crate::NBSolve as Solve;
use crate::NBStrongComponent as StrongComponent;
use crate::NBStrongComponent::CountCollector;
use crate::NBTearing as Tearing;
use crate::NBVariable as BVariable;
use crate::NBVariable::VarData;
use crate::NBVariable::VariablePointer;
use crate::NBVariable::VariablePointers;
use openmodelica_ast::Absyn::Path;
use openmodelica_frontend_types::DAE;
use openmodelica_nf_frontend::NFAlgorithm as Algorithm;
use openmodelica_nf_frontend::NFBackendExtension;
use openmodelica_nf_frontend::NFBackendExtension::Annotations;
use openmodelica_nf_frontend::NFBackendExtension::BackendInfo;
use openmodelica_nf_frontend::NFBackendExtension::VariableAttributes;
use openmodelica_nf_frontend::NFBackendExtension::VariableKind;
use openmodelica_nf_frontend::NFBinding as Binding;
use openmodelica_nf_frontend::NFBuiltinFuncs;
use openmodelica_nf_frontend::NFCall as Call;
use openmodelica_nf_frontend::NFClass as Class;
use openmodelica_nf_frontend::NFComplexType as ComplexType;
use openmodelica_nf_frontend::NFComponentRef as ComponentRef;
use openmodelica_nf_frontend::NFConvertDAE as ConvertDAE;
use openmodelica_nf_frontend::NFDimension as Dimension;
use openmodelica_nf_frontend::NFEquation as FEquation;
use openmodelica_nf_frontend::NFExpression as Expression;
use openmodelica_nf_frontend::NFFlatModel as FlatModel;
use openmodelica_nf_frontend::NFFunction::Function;
use openmodelica_nf_frontend::NFInstNode;
use openmodelica_nf_frontend::NFInstNode::InstNode;
use openmodelica_nf_frontend::NFPrefixes as Prefixes;
use openmodelica_nf_frontend::NFSimplifyExp as SimplifyExp;
use openmodelica_nf_frontend::NFStatement as Statement;
use openmodelica_nf_frontend::NFSubscript as Subscript;
use openmodelica_nf_frontend::NFType as Type;
use openmodelica_nf_frontend::NFVariable as Variable;
use openmodelica_util::BaseHashTable;
use openmodelica_util::ClockIndexes;
use openmodelica_util::Error;
use openmodelica_util::ExecStat;
use openmodelica_util::Flags;
use openmodelica_util::StringUtil;
use openmodelica_util::System;
use openmodelica_util::UnorderedMap;
use openmodelica_util::UnorderedSet;
use openmodelica_util::Util;
use openmodelica_util_datatypes_basic::List;
use openmodelica_util_datatypes_basic::MutableWeak;
use openmodelica_util_datatypes_basic::Pointer;
use openmodelica_util_datatypes_basic::PointerWeak;

/// file:        NBackendDAE.mo
/// package:     NBackendDAE
/// description: This file contains the main data type for the backend containing
///              all data. It further contains the lower and solve main function.
#[derive(Clone, Debug, Eq, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub enum NBackendDAE {
    MAIN {
        /// Partitions for differential-algebraic equations
        ode: metamodelica::List<metamodelica::Ref<Partition::Partition>>,
        /// Partitions for algebraic equations
        algebraic: metamodelica::List<metamodelica::Ref<Partition::Partition>>,
        /// Partitions for differential-algebraic event iteration
        ode_event: metamodelica::List<metamodelica::Ref<Partition::Partition>>,
        /// Partitions for algebraic event iteration
        alg_event: metamodelica::List<metamodelica::Ref<Partition::Partition>>,
        /// Clocked Partitions
        clocked: metamodelica::List<metamodelica::Ref<Partition::Partition>>,
        /// Partitions for initialization
        init: metamodelica::List<metamodelica::Ref<Partition::Partition>>,
        /// Partitions for initialization with lambda = 0 (homotopy)
        init_0: Option<metamodelica::List<metamodelica::Ref<Partition::Partition>>>,
        /// Partitions for dae mode
        dae: Option<metamodelica::List<metamodelica::Ref<Partition::Partition>>>,
        /// explicitly solved bindings of the primary parameters in evaluation order, computed before the initialization
        parameters: metamodelica::List<metamodelica::Ref<StrongComponent::NBStrongComponent>>,
        /// Variable data
        varData: metamodelica::Ref<VarData::VarData>,
        /// Equation data
        eqData: metamodelica::Ref<EqData::EqData>,
        /// contains time and state events
        eventInfo: metamodelica::Ref<Events::EventInfo::EventInfo>,
        /// contains information about clocked partitions
        clockedInfo: metamodelica::Ref<Partitioning::ClockedInfo::ClockedInfo>,
        /// Function bodies
        funcMap: metamodelica::Ref<
            UnorderedMap::UnorderedMap<metamodelica::Ref<Path>, metamodelica::Ref<Function::Function>>,
        >,
    },
    JACOBIAN {
        /// unique matrix name
        name: ArcStr,
        /// type of jacobian
        jacType: JacobianType,
        /// Variable data
        varData: metamodelica::Ref<VarData::VarData>,
        /// the sorted equations
        comps: metamodelica::Array<metamodelica::Ref<StrongComponent::NBStrongComponent>>,
        /// new sparsity pattern
        sparsity: metamodelica::Ref<Adjacency::Matrix::Matrix>,
        /// is this an adjoint jacobian?
        isAdjoint: bool,
    },
    HESSIAN {
        /// Variable data
        varData: metamodelica::Ref<VarData::VarData>,
        /// Equation data
        eqData: metamodelica::Ref<EqData::EqData>,
    },
}
impl metamodelica::gc::MMTrace for NBackendDAE {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        match self {
            NBackendDAE::MAIN {
                ode,
                algebraic,
                ode_event,
                alg_event,
                clocked,
                init,
                init_0,
                dae,
                parameters,
                varData,
                eqData,
                eventInfo,
                clockedInfo,
                funcMap,
            } => {
                metamodelica::gc::MMTrace::mm_accept(ode, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(algebraic, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(ode_event, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(alg_event, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(clocked, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(init, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(init_0, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(dae, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(parameters, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(varData, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(eqData, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(eventInfo, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(clockedInfo, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(funcMap, __mmv)?;
                Ok(())
            }
            NBackendDAE::JACOBIAN {
                name,
                jacType,
                varData,
                comps,
                sparsity,
                isAdjoint,
            } => {
                metamodelica::gc::MMTrace::mm_accept(name, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(jacType, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(varData, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(comps, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(sparsity, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(isAdjoint, __mmv)?;
                Ok(())
            }
            NBackendDAE::HESSIAN { varData, eqData } => {
                metamodelica::gc::MMTrace::mm_accept(varData, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(eqData, __mmv)?;
                Ok(())
            }
        }
    }
}
impl Default for NBackendDAE {
    fn default() -> Self {
        Self::HESSIAN {
            varData: Default::default(),
            eqData: Default::default(),
        }
    }
}
pub use self::NBackendDAE::{HESSIAN, JACOBIAN, MAIN};
pub fn toString(mut bdae: &metamodelica::Ref<NBackendDAE>, mut r#str: ArcStr) -> Result<ArcStr> {
    let mut r#str: ArcStr = r#str;
    r#str = ({
        let mut tmp: ArcStr = literal!("");
        (match &**bdae {
            MAIN {
                alg_event: __bdae_alg_event,
                algebraic: __bdae_algebraic,
                clocked: __bdae_clocked,
                clockedInfo: __bdae_clockedInfo,
                dae: __bdae_dae,
                eqData: __bdae_eqData,
                eventInfo: __bdae_eventInfo,
                init: __bdae_init,
                init_0: __bdae_init_0,
                ode: __bdae_ode,
                ode_event: __bdae_ode_event,
                varData: __bdae_varData,
                ..
            } => {
                if !(Flags::isSet(Flags::BLT_DUMP.clone())?)
                    || (__bdae_ode).is_empty()
                        && (__bdae_algebraic).is_empty()
                        && (__bdae_ode_event).is_empty()
                        && (__bdae_alg_event).is_empty()
                        && (__bdae_clocked).is_empty()
                        && (__bdae_init).is_empty()
                        && (__bdae_dae).is_none()
                {
                    tmp = {
                        let mut __mm_s = String::new();
                        __mm_s.push_str(&*StringUtil::headline_1(
                            &({
                                let mut __mm_s = String::new();
                                __mm_s.push_str(&*literal!("BackendDAE: "));
                                __mm_s.push_str(&*r#str);
                                ArcStr::from(__mm_s)
                            }),
                        )?);
                        __mm_s.push_str(&*literal!("\n"));
                        ArcStr::from(__mm_s)
                    };
                    tmp = {
                        let mut __mm_s = String::new();
                        __mm_s.push_str(&*tmp);
                        __mm_s.push_str(&*BVariable::VarData::toString(
                            metamodelica::AsArg::as_arg(&__bdae_varData),
                            2,
                        )?);
                        __mm_s.push_str(&*literal!("\n"));
                        __mm_s.push_str(&*BEquation::EqData::toString(
                            metamodelica::AsArg::as_arg(&__bdae_eqData),
                            1,
                            None,
                        )?);
                        ArcStr::from(__mm_s)
                    };
                } else {
                    tmp = {
                        let mut __mm_s = String::new();
                        __mm_s.push_str(&*tmp);
                        __mm_s.push_str(&*NBPartition::Partition::toStringList(
                            metamodelica::AsArg::as_arg(&__bdae_ode),
                            &({
                                let mut __mm_s = String::new();
                                __mm_s.push_str(&*literal!("[ODE] Differential-Algebraic: "));
                                __mm_s.push_str(&*r#str);
                                ArcStr::from(__mm_s)
                            }),
                        )?);
                        ArcStr::from(__mm_s)
                    };
                    tmp = {
                        let mut __mm_s = String::new();
                        __mm_s.push_str(&*tmp);
                        __mm_s.push_str(&*NBPartition::Partition::toStringList(
                            metamodelica::AsArg::as_arg(&__bdae_algebraic),
                            &({
                                let mut __mm_s = String::new();
                                __mm_s.push_str(&*literal!("[ALG] Algebraic: "));
                                __mm_s.push_str(&*r#str);
                                ArcStr::from(__mm_s)
                            }),
                        )?);
                        ArcStr::from(__mm_s)
                    };
                    tmp = {
                        let mut __mm_s = String::new();
                        __mm_s.push_str(&*tmp);
                        __mm_s.push_str(&*NBPartition::Partition::toStringList(
                            metamodelica::AsArg::as_arg(&__bdae_ode_event),
                            &({
                                let mut __mm_s = String::new();
                                __mm_s.push_str(&*literal!("[ODE_EVENT] Event Handling: "));
                                __mm_s.push_str(&*r#str);
                                ArcStr::from(__mm_s)
                            }),
                        )?);
                        ArcStr::from(__mm_s)
                    };
                    tmp = {
                        let mut __mm_s = String::new();
                        __mm_s.push_str(&*tmp);
                        __mm_s.push_str(&*NBPartition::Partition::toStringList(
                            metamodelica::AsArg::as_arg(&__bdae_alg_event),
                            &({
                                let mut __mm_s = String::new();
                                __mm_s.push_str(&*literal!("[ALG_EVENT] Event Handling: "));
                                __mm_s.push_str(&*r#str);
                                ArcStr::from(__mm_s)
                            }),
                        )?);
                        ArcStr::from(__mm_s)
                    };
                    tmp = {
                        let mut __mm_s = String::new();
                        __mm_s.push_str(&*tmp);
                        __mm_s.push_str(&*NBPartition::Partition::toStringList(
                            metamodelica::AsArg::as_arg(&__bdae_clocked),
                            &({
                                let mut __mm_s = String::new();
                                __mm_s.push_str(&*literal!("[CLOCKED] Event Handling: "));
                                __mm_s.push_str(&*r#str);
                                ArcStr::from(__mm_s)
                            }),
                        )?);
                        ArcStr::from(__mm_s)
                    };
                    tmp = {
                        let mut __mm_s = String::new();
                        __mm_s.push_str(&*tmp);
                        __mm_s.push_str(&*NBPartition::Partition::toStringList(
                            metamodelica::AsArg::as_arg(&__bdae_init),
                            &({
                                let mut __mm_s = String::new();
                                __mm_s.push_str(&*literal!("[INI] Initialization: "));
                                __mm_s.push_str(&*r#str);
                                ArcStr::from(__mm_s)
                            }),
                        )?);
                        ArcStr::from(__mm_s)
                    };
                    if (__bdae_init_0).is_some() {
                        tmp = {
                            let mut __mm_s = String::new();
                            __mm_s.push_str(&*tmp);
                            __mm_s.push_str(&*NBPartition::Partition::toStringList(
                                &(Util::getOption(__bdae_init_0.clone())?),
                                &({
                                    let mut __mm_s = String::new();
                                    __mm_s.push_str(&*literal!("[INI_0] Initialization Lambda=0: "));
                                    __mm_s.push_str(&*r#str);
                                    ArcStr::from(__mm_s)
                                }),
                            )?);
                            ArcStr::from(__mm_s)
                        };
                    }
                    if (__bdae_dae).is_some() {
                        tmp = {
                            let mut __mm_s = String::new();
                            __mm_s.push_str(&*tmp);
                            __mm_s.push_str(&*NBPartition::Partition::toStringList(
                                &(Util::getOption(__bdae_dae.clone())?),
                                &({
                                    let mut __mm_s = String::new();
                                    __mm_s.push_str(&*literal!("[DAE] DAEMode: "));
                                    __mm_s.push_str(&*r#str);
                                    ArcStr::from(__mm_s)
                                }),
                            )?);
                            ArcStr::from(__mm_s)
                        };
                    }
                }
                tmp = {
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*tmp);
                    __mm_s.push_str(&*Events::EventInfo::toString(metamodelica::AsArg::as_arg(
                        &__bdae_eventInfo,
                    ))?);
                    ArcStr::from(__mm_s)
                };
                tmp = {
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*tmp);
                    __mm_s.push_str(&*Partitioning::ClockedInfo::toString(metamodelica::AsArg::as_arg(
                        &__bdae_clockedInfo,
                    ))?);
                    ArcStr::from(__mm_s)
                };
                tmp
            }
            JACOBIAN {
                jacType: __bdae_jacType,
                name: __bdae_name,
                sparsity: __bdae_sparsity,
                varData: __bdae_varData,
                ..
            } => {
                tmp = {
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*StringUtil::headline_1(
                        &({
                            let mut __mm_s = String::new();
                            __mm_s.push_str(&*Jacobian::jacobianTypeString(__bdae_jacType.clone()));
                            __mm_s.push_str(&*literal!(" Jacobian "));
                            __mm_s.push_str(&*__bdae_name);
                            __mm_s.push_str(&*literal!(": "));
                            __mm_s.push_str(&*r#str);
                            ArcStr::from(__mm_s)
                        }),
                    )?);
                    __mm_s.push_str(&*literal!("\n"));
                    ArcStr::from(__mm_s)
                };
                tmp = {
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*tmp);
                    __mm_s.push_str(&*BVariable::VarData::toString(
                        metamodelica::AsArg::as_arg(&__bdae_varData),
                        1,
                    )?);
                    ArcStr::from(__mm_s)
                };
                for mut i in 1..=metamodelica::arrayLength(var_field!((**bdae).comps, NBackendDAE::JACOBIAN).clone()) {
                    tmp = {
                        let mut __mm_s = String::new();
                        __mm_s.push_str(&*tmp);
                        __mm_s.push_str(&*StrongComponent::toString(
                            &({
                                let __elt = (*metamodelica::index_checked(
                                    &var_field!((**bdae).comps, NBackendDAE::JACOBIAN).borrow(),
                                    i,
                                )?)
                                .clone();
                                __elt
                            }),
                            i,
                        )?);
                        __mm_s.push_str(&*literal!("\n"));
                        ArcStr::from(__mm_s)
                    };
                }
                tmp = {
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*tmp);
                    __mm_s.push_str(&*Adjacency::Matrix::toString(
                        metamodelica::AsArg::as_arg(&__bdae_sparsity),
                        literal!(""),
                    )?);
                    ArcStr::from(__mm_s)
                };
                tmp
            }
            HESSIAN {
                eqData: __bdae_eqData,
                varData: __bdae_varData,
            } => {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*StringUtil::headline_1(
                    &({
                        let mut __mm_s = String::new();
                        __mm_s.push_str(&*literal!("Hessian: "));
                        __mm_s.push_str(&*r#str);
                        ArcStr::from(__mm_s)
                    }),
                )?);
                __mm_s.push_str(&*literal!("\n"));
                __mm_s.push_str(&*BVariable::VarData::toString(
                    metamodelica::AsArg::as_arg(&__bdae_varData),
                    1,
                )?);
                __mm_s.push_str(&*literal!("\n"));
                __mm_s.push_str(&*BEquation::EqData::toString(
                    metamodelica::AsArg::as_arg(&__bdae_eqData),
                    1,
                    None,
                )?);
                ArcStr::from(__mm_s)
            }
            _ => {
                Error::addMessage(
                    Error::INTERNAL_ERROR.clone(),
                    list![{
                        let mut __mm_s = String::new();
                        __mm_s.push_str(&*literal!("NBackendDAE.toString"));
                        __mm_s.push_str(&*literal!(" failed."));
                        ArcStr::from(__mm_s)
                    }],
                )?;
                return Err("fail");
            }
        })
    });
    Ok(r#str)
}

pub(crate) fn getVarData(mut bdae: &metamodelica::Ref<NBackendDAE>) -> Result<metamodelica::Ref<VarData::VarData>> {
    let mut varData: metamodelica::Ref<VarData::VarData>;
    varData = (match &**bdae {
        MAIN {
            varData: __bdae_varData,
            ..
        } => __bdae_varData.clone(),
        JACOBIAN {
            varData: __bdae_varData,
            ..
        } => __bdae_varData.clone(),
        HESSIAN {
            varData: __bdae_varData,
            ..
        } => __bdae_varData.clone(),
        _ => {
            Error::addMessage(
                Error::INTERNAL_ERROR.clone(),
                list![{
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!("NBackendDAE.getVarData"));
                    __mm_s.push_str(&*literal!(" failed."));
                    ArcStr::from(__mm_s)
                }],
            )?;
            return Err("fail");
        }
    });
    Ok(varData)
}

pub(crate) fn setVarData(
    mut bdae: metamodelica::Ref<NBackendDAE>,
    mut varData: metamodelica::Ref<VarData::VarData>,
) -> Result<metamodelica::Ref<NBackendDAE>> {
    let mut bdae: metamodelica::Ref<NBackendDAE> = bdae;
    bdae = (match &*bdae {
        MAIN { .. } => {
            assign_variant_field!(bdae => NBackendDAE::MAIN; varData = varData);
            bdae
        }
        JACOBIAN { .. } => {
            assign_variant_field!(bdae => NBackendDAE::JACOBIAN; varData = varData);
            bdae
        }
        HESSIAN { .. } => {
            assign_variant_field!(bdae => NBackendDAE::HESSIAN; varData = varData);
            bdae
        }
        _ => {
            Error::addMessage(
                Error::INTERNAL_ERROR.clone(),
                list![{
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!("NBackendDAE.setVarData"));
                    __mm_s.push_str(&*literal!(" failed."));
                    ArcStr::from(__mm_s)
                }],
            )?;
            return Err("fail");
        }
    });
    Ok(bdae)
}

pub(crate) fn getIsAdjoint(mut bdae: &metamodelica::Ref<NBackendDAE>) -> Result<bool> {
    let mut isAdjoint: bool;
    isAdjoint = (match &**bdae {
        JACOBIAN {
            isAdjoint: __esc_isAdjoint,
            ..
        } => {
            isAdjoint = (*__esc_isAdjoint).clone();
            isAdjoint.clone()
        }
        _ => {
            Error::addMessage(
                Error::INTERNAL_ERROR.clone(),
                list![{
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!("NBackendDAE.getIsAdjoint"));
                    __mm_s.push_str(&*literal!(" failed! Only the record type JACOBIAN() has a jacobian."));
                    ArcStr::from(__mm_s)
                }],
            )?;
            return Err("fail");
        }
    });
    Ok(isAdjoint)
}

pub(crate) fn getFunctionMap(
    mut bdae: &metamodelica::Ref<NBackendDAE>,
) -> Result<metamodelica::Ref<UnorderedMap::UnorderedMap<metamodelica::Ref<Path>, metamodelica::Ref<Function::Function>>>>
{
    let mut funcMap: metamodelica::Ref<
        UnorderedMap::UnorderedMap<metamodelica::Ref<Path>, metamodelica::Ref<Function::Function>>,
    >;
    funcMap = (match &**bdae {
        MAIN {
            funcMap: __bdae_funcMap,
            ..
        } => __bdae_funcMap.clone(),
        _ => {
            Error::addMessage(
                Error::INTERNAL_ERROR.clone(),
                list![{
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!("NBackendDAE.getFunctionMap"));
                    __mm_s.push_str(&*literal!(" failed! Only the record type MAIN() has a function map."));
                    ArcStr::from(__mm_s)
                }],
            )?;
            return Err("fail");
        }
    });
    Ok(funcMap)
}

pub(crate) fn sizes(mut bdae: &metamodelica::Ref<NBackendDAE>) -> Result<((i32, i32), (i32, i32))> {
    let mut varSizes: (i32, i32);
    let mut eqnSizes: (i32, i32);
    (varSizes, eqnSizes) = (match &**bdae {
        MAIN {
            eqData: __bdae_eqData,
            varData: __bdae_varData,
            ..
        } => (
            (
                BVariable::VarData::scalarSize(metamodelica::AsArg::as_arg(&__bdae_varData), true)?,
                BVariable::VarData::size(metamodelica::AsArg::as_arg(&__bdae_varData))?,
            ),
            (
                BEquation::EqData::scalarSize(metamodelica::AsArg::as_arg(&__bdae_eqData), true)?,
                BEquation::EqData::size(metamodelica::AsArg::as_arg(&__bdae_eqData))?,
            ),
        ),
        _ => ((0, 0), (0, 0)),
    });
    Ok((varSizes, eqnSizes))
}

pub fn lower(
    mut flatModel: &metamodelica::Ref<FlatModel::NFFlatModel>,
    mut funcMap: metamodelica::Ref<
        UnorderedMap::UnorderedMap<metamodelica::Ref<Path>, metamodelica::Ref<Function::Function>>,
    >,
) -> Result<metamodelica::Ref<NBackendDAE>> {
    let mut bdae: metamodelica::Ref<NBackendDAE>;
    let mut variableData: metamodelica::Ref<VarData::VarData>;
    let mut equationData: metamodelica::Ref<EqData::EqData>;
    let mut eventInfo: metamodelica::Ref<Events::EventInfo::EventInfo> = Events::EventInfo::empty();
    let mut clockedInfo: metamodelica::Ref<Partitioning::ClockedInfo::ClockedInfo> = Partitioning::ClockedInfo::new();
    variableData = lowerVariableData(continuousImplicitDiscretes(
        flatModel.variables.clone(),
        &(listAppend(flatModel.equations.clone(), flatModel.initialEquations.clone())),
        &(listAppend(flatModel.algorithms.clone(), flatModel.initialAlgorithms.clone())),
    )?)?;
    (equationData, variableData) = lowerEquationData(
        &flatModel.equations,
        &flatModel.algorithms,
        &flatModel.initialEquations,
        &flatModel.initialAlgorithms,
        variableData,
    )?;
    bdae = metamodelica::Ref::new(NBackendDAE::MAIN {
        ode: metamodelica::nil(),
        algebraic: metamodelica::nil(),
        ode_event: metamodelica::nil(),
        alg_event: metamodelica::nil(),
        clocked: metamodelica::nil(),
        init: metamodelica::nil(),
        init_0: None,
        dae: None,
        parameters: metamodelica::nil(),
        varData: variableData,
        eqData: equationData,
        eventInfo: eventInfo,
        clockedInfo: clockedInfo,
        funcMap: lowerFunctions(funcMap)?,
    });
    Ok(bdae)
}

pub(crate) fn continuousImplicitDiscretes(
    mut variables: metamodelica::List<metamodelica::Ref<Variable::NFVariable>>,
    mut equations: &metamodelica::List<metamodelica::Ref<FEquation::NFEquation>>,
    mut algorithms: &metamodelica::List<metamodelica::Ref<Algorithm::NFAlgorithm>>,
) -> Result<metamodelica::List<metamodelica::Ref<Variable::NFVariable>>> {
    let mut variables: metamodelica::List<metamodelica::Ref<Variable::NFVariable>> = variables;
    let mut assigned: metamodelica::Ref<UnorderedSet::UnorderedSet<metamodelica::Ref<ComponentRef::NFComponentRef>>> =
        UnorderedSet::new(
            (std::sync::Arc::new(move |__a0: metamodelica::Ref<ComponentRef::NFComponentRef>| ComponentRef::hash(&__a0))
                as std::sync::Arc<
                    dyn ::std::ops::Fn(metamodelica::Ref<ComponentRef::NFComponentRef>) -> Result<i32> + 'static,
                >),
            (std::sync::Arc::new(
                move |__a0: metamodelica::Ref<ComponentRef::NFComponentRef>,
                      __a1: metamodelica::Ref<ComponentRef::NFComponentRef>| {
                    ComponentRef::isEqual(&__a0, &__a1)
                },
            )
                as std::sync::Arc<
                    dyn ::std::ops::Fn(
                            metamodelica::Ref<ComponentRef::NFComponentRef>,
                            metamodelica::Ref<ComponentRef::NFComponentRef>,
                        ) -> Result<bool>
                        + 'static,
                >),
            13,
        );
    for mut eq in &**equations {
        collectWhenAssigned(metamodelica::AsArg::as_arg(&eq), false, assigned.clone())?;
    }
    for mut alg in &**algorithms {
        collectWhenAssignedStmts(&alg.statements, false, assigned.clone())?;
    }
    variables = ({
        let mut __acc: metamodelica::List<metamodelica::Ref<Variable::NFVariable>> = metamodelica::nil();
        for mut var in (variables).into_iter().cloned() {
            let __x = continuousIfUnassigned(var.clone(), assigned.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    Ok(variables)
}

pub(crate) fn continuousIfUnassigned(
    mut var: metamodelica::Ref<Variable::NFVariable>,
    mut assigned: metamodelica::Ref<UnorderedSet::UnorderedSet<metamodelica::Ref<ComponentRef::NFComponentRef>>>,
) -> Result<metamodelica::Ref<Variable::NFVariable>> {
    let mut var: metamodelica::Ref<Variable::NFVariable> = var;
    if Variable::variability(&var) == Prefixes::Variability::IMPLICITLY_DISCRETE.clone()
        && Type::isReal(&(Type::arrayElementType(&var.ty)))?
        && !(UnorderedSet::contains(ComponentRef::stripSubscriptsAll(&var.name), assigned)?)
    {
        var = Variable::setVariability(var, Prefixes::Variability::CONTINUOUS.clone());
    }
    Ok(var)
}

pub(crate) fn collectWhenAssigned(
    mut eq: &metamodelica::Ref<FEquation::NFEquation>,
    mut inWhen: bool,
    mut assigned: metamodelica::Ref<UnorderedSet::UnorderedSet<metamodelica::Ref<ComponentRef::NFComponentRef>>>,
) -> Result<()> {
    let () = (match &**eq {
        FEquation::EQUALITY { lhs: __eq_lhs, .. } if (inWhen) => {
            for mut cref in &*UnorderedSet::toList(Expression::extractCrefs(__eq_lhs.clone())?) {
                UnorderedSet::add(
                    ComponentRef::stripSubscriptsAll(metamodelica::AsArg::as_arg(&cref)),
                    assigned.clone(),
                )?;
            }
            ()
        }
        FEquation::FOR { body: __eq_body, .. } => {
            for mut e in &*__eq_body.clone() {
                collectWhenAssigned(metamodelica::AsArg::as_arg(&e), inWhen, assigned.clone())?;
            }
            ()
        }
        FEquation::IF {
            branches: __eq_branches,
            ..
        } => {
            for mut branch in &*__eq_branches.clone() {
                collectWhenAssignedBranch(metamodelica::AsArg::as_arg(&branch), inWhen, assigned.clone())?;
            }
            ()
        }
        FEquation::WHEN {
            branches: __eq_branches,
            ..
        } => {
            for mut branch in &*__eq_branches.clone() {
                collectWhenAssignedBranch(metamodelica::AsArg::as_arg(&branch), true, assigned.clone())?;
            }
            ()
        }
        _ => (),
    });
    Ok(())
}

pub(crate) fn collectWhenAssignedBranch(
    mut branch: &metamodelica::Ref<FEquation::Branch::Branch>,
    mut inWhen: bool,
    mut assigned: metamodelica::Ref<UnorderedSet::UnorderedSet<metamodelica::Ref<ComponentRef::NFComponentRef>>>,
) -> Result<()> {
    let () = (match &**branch {
        FEquation::Branch::BRANCH {
            body: __branch_body, ..
        } => {
            for mut e in &*__branch_body.clone() {
                collectWhenAssigned(metamodelica::AsArg::as_arg(&e), inWhen, assigned.clone())?;
            }
            ()
        }
        _ => (),
    });
    Ok(())
}

pub(crate) fn collectWhenAssignedStmts(
    mut stmts: &metamodelica::List<metamodelica::Ref<Statement::NFStatement>>,
    mut inWhen: bool,
    mut assigned: metamodelica::Ref<UnorderedSet::UnorderedSet<metamodelica::Ref<ComponentRef::NFComponentRef>>>,
) -> Result<()> {
    for mut stmt in &**stmts {
        let () = (match &*stmt.clone() {
            Statement::ASSIGNMENT { lhs: __stmt_lhs, .. } if (inWhen) => {
                for mut cref in &*UnorderedSet::toList(Expression::extractCrefs(__stmt_lhs.clone())?) {
                    UnorderedSet::add(
                        ComponentRef::stripSubscriptsAll(metamodelica::AsArg::as_arg(&cref)),
                        assigned.clone(),
                    )?;
                }
                ()
            }
            Statement::FOR { body: __stmt_body, .. } => {
                collectWhenAssignedStmts(metamodelica::AsArg::as_arg(&__stmt_body), inWhen, assigned.clone())?;
                ()
            }
            Statement::WHILE { body: __stmt_body, .. } => {
                collectWhenAssignedStmts(metamodelica::AsArg::as_arg(&__stmt_body), inWhen, assigned.clone())?;
                ()
            }
            Statement::IF {
                branches: __stmt_branches,
                ..
            } => {
                for mut branch in &*__stmt_branches.clone() {
                    collectWhenAssignedStmts(&(Util::tuple22(branch.clone())), inWhen, assigned.clone())?;
                }
                ()
            }
            Statement::WHEN {
                branches: __stmt_branches,
                ..
            } => {
                for mut branch in &*__stmt_branches.clone() {
                    collectWhenAssignedStmts(&(Util::tuple22(branch.clone())), true, assigned.clone())?;
                }
                ()
            }
            _ => (),
        });
    }
    Ok(())
}

pub fn main(mut bdae: metamodelica::Ref<NBackendDAE>) -> Result<metamodelica::Ref<NBackendDAE>> {
    let mut bdae: metamodelica::Ref<NBackendDAE> = bdae;
    let mut preOptModules: metamodelica::List<(Module::wrapper, ArcStr)>;
    let mut mainModules: metamodelica::List<(Module::wrapper, ArcStr)>;
    let mut postOptModules: metamodelica::List<(Module::wrapper, ArcStr)>;
    let mut preOptClocks: metamodelica::List<(ArcStr, metamodelica::Real)>;
    let mut mainClocks: metamodelica::List<(ArcStr, metamodelica::Real)>;
    let mut postOptClocks: metamodelica::List<(ArcStr, metamodelica::Real)>;
    let mut followEquations: metamodelica::List<ArcStr> =
        Flags::getConfigStringList(Flags::DEBUG_FOLLOW_EQUATIONS.clone())?;
    let mut eq_filter_opt: Option<metamodelica::Ref<UnorderedSet::UnorderedSet<ArcStr>>>;
    let mut inline_types: metamodelica::List<DAE::InlineType> = list![
        openmodelica_frontend_types::DAE::InlineType::NORM_INLINE,
        openmodelica_frontend_types::DAE::InlineType::BUILTIN_EARLY_INLINE,
        openmodelica_frontend_types::DAE::InlineType::EARLY_INLINE,
        openmodelica_frontend_types::DAE::InlineType::DEFAULT_INLINE
    ];
    let mut kind: NBPartition::Kind;
    if (followEquations).is_empty() {
        eq_filter_opt = None;
    } else {
        metamodelica::print({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*List::toStringCustom(
                followEquations.clone(),
                &fnptr!(Util::id, _),
                literal!("[debugFilterEquations] filtering for equations: "),
                literal!("{"),
                literal!(", "),
                literal!("}"),
                true,
                0,
            )?);
            __mm_s.push_str(&*literal!("\n\n"));
            ArcStr::from(__mm_s)
        });
        eq_filter_opt = Some(UnorderedSet::fromList(
            &followEquations,
            (std::sync::Arc::new(fnptr!(stringHashDjb2, ArcStr))
                as std::sync::Arc<dyn ::std::ops::Fn(ArcStr) -> Result<i32> + 'static>),
            (std::sync::Arc::new(fnptr!(stringEqual, ArcStr, ArcStr))
                as std::sync::Arc<dyn ::std::ops::Fn(ArcStr, ArcStr) -> Result<bool> + 'static>),
        )?);
    }
    if Flags::getConfigBool(Flags::DAE_MODE.clone())? {
        mainModules = list![(
            (std::sync::Arc::new(DAEMode::main)
                as std::sync::Arc<
                    dyn ::std::ops::Fn(metamodelica::Ref<NBackendDAE>) -> Result<metamodelica::Ref<NBackendDAE>>
                        + 'static,
                >),
            literal!("DAE-Mode")
        )];
        kind = NBPartition::Kind::DAE.clone();
    } else {
        mainModules = metamodelica::nil();
        kind = NBPartition::Kind::ODE.clone();
    }
    preOptModules = list![
        (
            (std::sync::Arc::new(Bindings::main)
                as std::sync::Arc<
                    dyn ::std::ops::Fn(metamodelica::Ref<NBackendDAE>) -> Result<metamodelica::Ref<NBackendDAE>>
                        + 'static,
                >),
            literal!("Bindings")
        ),
        (
            (std::sync::Arc::new({
                let __pe_b1 = kind;
                move |__pe_a0| FunctionAlias::main(__pe_a0, __pe_b1.clone())
            })
                as std::sync::Arc<
                    dyn ::std::ops::Fn(metamodelica::Ref<NBackendDAE>) -> Result<metamodelica::Ref<NBackendDAE>>
                        + 'static,
                >),
            literal!("FunctionAlias")
        ),
        (
            (std::sync::Arc::new({
                let __pe_b1 = inline_types;
                let __pe_b2 = false;
                move |__pe_a0| Inline::main(__pe_a0, __pe_b1.clone(), __pe_b2.clone())
            })
                as std::sync::Arc<
                    dyn ::std::ops::Fn(metamodelica::Ref<NBackendDAE>) -> Result<metamodelica::Ref<NBackendDAE>>
                        + 'static,
                >),
            literal!("Early Inline")
        ),
        (
            (std::sync::Arc::new({
                let __pe_b1 = false;
                move |__pe_a0| simplify(__pe_a0, __pe_b1.clone())
            })
                as std::sync::Arc<
                    dyn ::std::ops::Fn(metamodelica::Ref<NBackendDAE>) -> Result<metamodelica::Ref<NBackendDAE>>
                        + 'static,
                >),
            literal!("Simplify 1")
        ),
        (
            (std::sync::Arc::new({
                let __pe_b1 = kind;
                move |__pe_a0| Alias::main(__pe_a0, __pe_b1.clone())
            })
                as std::sync::Arc<
                    dyn ::std::ops::Fn(metamodelica::Ref<NBackendDAE>) -> Result<metamodelica::Ref<NBackendDAE>>
                        + 'static,
                >),
            literal!("Alias")
        ),
        (
            (std::sync::Arc::new({
                let __pe_b1 = false;
                move |__pe_a0| simplify(__pe_a0, __pe_b1.clone())
            })
                as std::sync::Arc<
                    dyn ::std::ops::Fn(metamodelica::Ref<NBackendDAE>) -> Result<metamodelica::Ref<NBackendDAE>>
                        + 'static,
                >),
            literal!("Simplify 2")
        ),
        (
            (std::sync::Arc::new(removeStream)
                as std::sync::Arc<
                    dyn ::std::ops::Fn(metamodelica::Ref<NBackendDAE>) -> Result<metamodelica::Ref<NBackendDAE>>
                        + 'static,
                >),
            literal!("Remove Stream")
        ),
        (
            (std::sync::Arc::new(DetectStates::main)
                as std::sync::Arc<
                    dyn ::std::ops::Fn(metamodelica::Ref<NBackendDAE>) -> Result<metamodelica::Ref<NBackendDAE>>
                        + 'static,
                >),
            literal!("Detect States")
        ),
        (
            (std::sync::Arc::new(Events::main)
                as std::sync::Arc<
                    dyn ::std::ops::Fn(metamodelica::Ref<NBackendDAE>) -> Result<metamodelica::Ref<NBackendDAE>>
                        + 'static,
                >),
            literal!("Events")
        )
    ];
    mainModules = listAppend(
        list![
            (
                (std::sync::Arc::new({
                    let __pe_b1 = NBPartition::Kind::ODE.clone();
                    move |__pe_a0| Partitioning::main(__pe_a0, __pe_b1.clone())
                })
                    as std::sync::Arc<
                        dyn ::std::ops::Fn(metamodelica::Ref<NBackendDAE>) -> Result<metamodelica::Ref<NBackendDAE>>
                            + 'static,
                    >),
                literal!("Partitioning")
            ),
            (
                (std::sync::Arc::new({
                    let __pe_b1 = NBPartition::Kind::ODE.clone();
                    move |__pe_a0| Causalize::main(__pe_a0, __pe_b1.clone())
                })
                    as std::sync::Arc<
                        dyn ::std::ops::Fn(metamodelica::Ref<NBackendDAE>) -> Result<metamodelica::Ref<NBackendDAE>>
                            + 'static,
                    >),
                literal!("Causalize")
            ),
            (
                (std::sync::Arc::new({
                    let __pe_b1 = list![openmodelica_frontend_types::DAE::InlineType::AFTER_INDEX_RED_INLINE];
                    let __pe_b2 = false;
                    move |__pe_a0| Inline::main(__pe_a0, __pe_b1.clone(), __pe_b2.clone())
                })
                    as std::sync::Arc<
                        dyn ::std::ops::Fn(metamodelica::Ref<NBackendDAE>) -> Result<metamodelica::Ref<NBackendDAE>>
                            + 'static,
                    >),
                literal!("After Index Reduction Inline")
            ),
            (
                (std::sync::Arc::new(Initialization::main)
                    as std::sync::Arc<
                        dyn ::std::ops::Fn(metamodelica::Ref<NBackendDAE>) -> Result<metamodelica::Ref<NBackendDAE>>
                            + 'static,
                    >),
                literal!("Initialization")
            )
        ],
        mainModules,
    );
    postOptModules = list![
        (
            (std::sync::Arc::new(Evaluation::removeDummies)
                as std::sync::Arc<
                    dyn ::std::ops::Fn(metamodelica::Ref<NBackendDAE>) -> Result<metamodelica::Ref<NBackendDAE>>
                        + 'static,
                >),
            literal!("Remove Dummies")
        ),
        (
            (std::sync::Arc::new({
                let __pe_b1 = kind;
                move |__pe_a0| Tearing::main(__pe_a0, __pe_b1.clone())
            })
                as std::sync::Arc<
                    dyn ::std::ops::Fn(metamodelica::Ref<NBackendDAE>) -> Result<metamodelica::Ref<NBackendDAE>>
                        + 'static,
                >),
            literal!("Tearing")
        ),
        (
            (std::sync::Arc::new(Partitioning::categorize)
                as std::sync::Arc<
                    dyn ::std::ops::Fn(metamodelica::Ref<NBackendDAE>) -> Result<metamodelica::Ref<NBackendDAE>>
                        + 'static,
                >),
            literal!("Categorize")
        ),
        (
            (std::sync::Arc::new(Solve::main)
                as std::sync::Arc<
                    dyn ::std::ops::Fn(metamodelica::Ref<NBackendDAE>) -> Result<metamodelica::Ref<NBackendDAE>>
                        + 'static,
                >),
            literal!("Solve")
        ),
        (
            (std::sync::Arc::new({
                let __pe_b1 = kind;
                move |__pe_a0| Jacobian::main(__pe_a0, __pe_b1.clone())
            })
                as std::sync::Arc<
                    dyn ::std::ops::Fn(metamodelica::Ref<NBackendDAE>) -> Result<metamodelica::Ref<NBackendDAE>>
                        + 'static,
                >),
            literal!("Jacobian")
        ),
        (
            (std::sync::Arc::new(Initialization::minimizeHomotopySystem)
                as std::sync::Arc<
                    dyn ::std::ops::Fn(metamodelica::Ref<NBackendDAE>) -> Result<metamodelica::Ref<NBackendDAE>>
                        + 'static,
                >),
            literal!("Minimize Homotopy System")
        )
    ];
    (bdae, preOptClocks) = applyModules(
        bdae,
        &preOptModules,
        eq_filter_opt.clone(),
        ClockIndexes::RT_CLOCK_NEW_BACKEND_MODULE.clone(),
    )?;
    (bdae, mainClocks) = applyModules(
        bdae,
        &mainModules,
        eq_filter_opt.clone(),
        ClockIndexes::RT_CLOCK_NEW_BACKEND_MODULE.clone(),
    )?;
    (bdae, postOptClocks) = applyModules(
        bdae,
        &postOptModules,
        eq_filter_opt,
        ClockIndexes::RT_CLOCK_NEW_BACKEND_MODULE.clone(),
    )?;
    if Flags::isSet(Flags::DUMP_BACKEND_CLOCKS.clone())? {
        if !((preOptClocks).is_empty()) {
            metamodelica::print(StringUtil::headline_4(&(literal!("Pre-Opt Backend Clocks:")))?);
            metamodelica::print({
                let mut __mm_s = String::new();
                __mm_s.push_str(&*stringDelimitList(
                    ({
                        let mut __acc: metamodelica::List<ArcStr> = metamodelica::nil();
                        for mut clck in (preOptClocks).into_iter().cloned() {
                            let __x = Module::moduleClockString(&(clck.clone()))?;
                            __acc = cons(__x, __acc);
                        }
                        __acc.reverse()
                    }),
                    literal!("\n"),
                ));
                __mm_s.push_str(&*literal!("\n"));
                ArcStr::from(__mm_s)
            });
        }
        if !((mainClocks).is_empty()) {
            metamodelica::print(StringUtil::headline_4(&(literal!("Main Backend Clocks:")))?);
            metamodelica::print({
                let mut __mm_s = String::new();
                __mm_s.push_str(&*stringDelimitList(
                    ({
                        let mut __acc: metamodelica::List<ArcStr> = metamodelica::nil();
                        for mut clck in (mainClocks).into_iter().cloned() {
                            let __x = Module::moduleClockString(&(clck.clone()))?;
                            __acc = cons(__x, __acc);
                        }
                        __acc.reverse()
                    }),
                    literal!("\n"),
                ));
                __mm_s.push_str(&*literal!("\n"));
                ArcStr::from(__mm_s)
            });
        }
        if !((postOptClocks).is_empty()) {
            metamodelica::print(StringUtil::headline_4(&(literal!("Post-Opt Backend Clocks:")))?);
            metamodelica::print({
                let mut __mm_s = String::new();
                __mm_s.push_str(&*stringDelimitList(
                    ({
                        let mut __acc: metamodelica::List<ArcStr> = metamodelica::nil();
                        for mut clck in (postOptClocks).into_iter().cloned() {
                            let __x = Module::moduleClockString(&(clck.clone()))?;
                            __acc = cons(__x, __acc);
                        }
                        __acc.reverse()
                    }),
                    literal!("\n"),
                ));
                __mm_s.push_str(&*literal!("\n\n"));
                ArcStr::from(__mm_s)
            });
        }
    }
    backenddaeinfo(&bdae)?;
    Ok(bdae)
}

pub(crate) fn applyModules(
    mut bdae: metamodelica::Ref<NBackendDAE>,
    mut modules: &metamodelica::List<(
        Arc<dyn ::std::ops::Fn(metamodelica::Ref<NBackendDAE>) -> Result<metamodelica::Ref<NBackendDAE>> + 'static>,
        ArcStr,
    )>,
    mut eq_filter_opt: Option<metamodelica::Ref<UnorderedSet::UnorderedSet<ArcStr>>>,
    mut clock_idx: i32,
) -> Result<(
    metamodelica::Ref<NBackendDAE>,
    metamodelica::List<(ArcStr, metamodelica::Real)>,
)> {
    let mut bdae: metamodelica::Ref<NBackendDAE> = bdae;
    let mut module_clocks: metamodelica::List<(ArcStr, metamodelica::Real)> = metamodelica::nil();
    let mut func: Module::wrapper;
    let mut name: ArcStr;
    let mut debugStr: ArcStr = literal!("");
    let mut clock_time: metamodelica::Real;
    let mut varSizes: (i32, i32);
    let mut eqnSizes: (i32, i32);
    System::reportProgress(-1, 4);
    for mut module in &**modules {
        Error::checkCancel()?;
        (func, name) = module.clone();
        if Flags::isSet(Flags::FAILTRACE.clone())? {
            debugStr = {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("[failtrace] ........ ["));
                __mm_s.push_str(&*ClockIndexes::toString(clock_idx));
                __mm_s.push_str(&*literal!("] "));
                __mm_s.push_str(&*name);
                ArcStr::from(__mm_s)
            };
            debugStr = {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*debugStr);
                __mm_s.push_str(&*StringUtil::repeat(
                    literal!("."),
                    intMax(60 - ((debugStr).len() as i32), 0),
                )?);
                ArcStr::from(__mm_s)
            };
        }
        if clock_idx != -1 {
            System::realtimeClear(clock_idx)?;
            System::realtimeTick(clock_idx)?;
            if let Ok(__iflet0) = func(bdae.clone()) {
                bdae = __iflet0;
            } else {
                if Flags::isSet(Flags::FAILTRACE.clone())? {
                    debugStr = {
                        let mut __mm_s = String::new();
                        __mm_s.push_str(&*debugStr);
                        __mm_s.push_str(&*literal!(" failed\n"));
                        ArcStr::from(__mm_s)
                    };
                    metamodelica::print(debugStr.clone());
                }
                return Err("fail");
            }
            clock_time = System::realtimeTock(clock_idx)?;
            ExecStat::execStat(
                &({
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!("["));
                    __mm_s.push_str(&*ClockIndexes::toString(clock_idx));
                    __mm_s.push_str(&*literal!("] "));
                    __mm_s.push_str(&*name);
                    ArcStr::from(__mm_s)
                }),
            )?;
            module_clocks = metamodelica::cons((name.clone(), clock_time), module_clocks);
            if Flags::isSet(Flags::FAILTRACE.clone())? {
                (varSizes, eqnSizes) = sizes(&bdae)?;
                debugStr = {
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*debugStr);
                    __mm_s.push_str(&*literal!(" V("));
                    __mm_s.push_str(&*intString(Util::tuple21(varSizes)));
                    __mm_s.push_str(&*literal!("|"));
                    __mm_s.push_str(&*intString(Util::tuple22(varSizes)));
                    __mm_s.push_str(&*literal!(")"));
                    ArcStr::from(__mm_s)
                };
                debugStr = {
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*debugStr);
                    __mm_s.push_str(&*literal!(" E("));
                    __mm_s.push_str(&*intString(Util::tuple21(eqnSizes)));
                    __mm_s.push_str(&*literal!("|"));
                    __mm_s.push_str(&*intString(Util::tuple22(eqnSizes)));
                    __mm_s.push_str(&*literal!(") "));
                    ArcStr::from(__mm_s)
                };
                if Util::tuple21(varSizes) != Util::tuple21(eqnSizes) {
                    debugStr = {
                        let mut __mm_s = String::new();
                        __mm_s.push_str(&*debugStr);
                        __mm_s.push_str(&*literal!("XX "));
                        ArcStr::from(__mm_s)
                    };
                }
                debugStr = {
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*debugStr);
                    __mm_s.push_str(&*StringUtil::repeat(
                        literal!("."),
                        intMax(100 - ((debugStr).len() as i32), 0),
                    )?);
                    ArcStr::from(__mm_s)
                };
                debugStr = {
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*debugStr);
                    __mm_s.push_str(&*literal!(" "));
                    __mm_s.push_str(&*realString(clock_time));
                    __mm_s.push_str(&*literal!("s\n"));
                    ArcStr::from(__mm_s)
                };
                metamodelica::print(debugStr.clone());
                debugLowering(&bdae)?;
            }
        } else {
            bdae = func(bdae)?;
        }
        if Flags::isSet(Flags::OPT_DAE_DUMP.clone())?
            || Flags::isSet(Flags::BLT_DUMP.clone())?
                && (metamodelica::stringEq(&name, &(literal!("Causalize")))
                    || metamodelica::stringEq(&name, &(literal!("Solve"))))
        {
            metamodelica::print(toString(&bdae, {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("("));
                __mm_s.push_str(&*name);
                __mm_s.push_str(&*literal!(")"));
                ArcStr::from(__mm_s)
            })?);
        }
        if (eq_filter_opt).is_some() {
            debugFollowEquations(
                &bdae,
                eq_filter_opt.clone(),
                &({
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!("("));
                    __mm_s.push_str(&*name);
                    __mm_s.push_str(&*literal!(")"));
                    ArcStr::from(__mm_s)
                }),
            )?;
        }
    }
    module_clocks = module_clocks.reverse();
    Ok((bdae, module_clocks))
}

pub(crate) fn simplify(
    mut bdae: metamodelica::Ref<NBackendDAE>,
    mut init: bool,
) -> Result<metamodelica::Ref<NBackendDAE>> {
    let mut bdae: metamodelica::Ref<NBackendDAE> = bdae;
    let mut acc_discrete_states: Pointer::Pointer<
        metamodelica::List<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>>,
    > = Pointer::create(metamodelica::nil());
    let mut acc_previous: Pointer::Pointer<
        metamodelica::List<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>>,
    > = Pointer::create(metamodelica::nil());
    let mut func: BEquation::MapFuncEqn = (std::sync::Arc::new({
        let __pe_b1 = literal!("NBackendDAE.simplify");
        let __pe_b2 = literal!("");
        let __pe_b3 = acc_discrete_states.clone();
        let __pe_b4 = acc_previous.clone();
        let __pe_b5: Arc<
            dyn ::std::ops::Fn(
                    metamodelica::Ref<Expression::NFExpression>,
                ) -> Result<metamodelica::Ref<Expression::NFExpression>>
                + 'static,
        > = (std::sync::Arc::new({
            let __pe_b1 = true;
            let __pe_b2 = literal!("NBackendDAE.simplify");
            let __pe_b3 = literal!("");
            move |__pe_a0| SimplifyExp::simplifyDump(__pe_a0, __pe_b1.clone(), &__pe_b2, &__pe_b3)
        })
            as std::sync::Arc<
                dyn ::std::ops::Fn(
                        metamodelica::Ref<Expression::NFExpression>,
                    ) -> Result<metamodelica::Ref<Expression::NFExpression>>
                    + 'static,
            >);
        move |__pe_a0| {
            BEquation::Equation::simplify(
                __pe_a0,
                &__pe_b1,
                &__pe_b2,
                __pe_b3.clone(),
                __pe_b4.clone(),
                __pe_b5.clone(),
            )
        }
    })
        as std::sync::Arc<
            dyn ::std::ops::Fn(metamodelica::Ref<Equation::Equation>) -> Result<metamodelica::Ref<Equation::Equation>>
                + 'static,
        >);
    bdae = (::match_deref::match_deref! { match &(bdae.clone()) {
        Deref @ MAIN { eqData: eqData @ Deref @ BEquation::EqData::EQ_DATA_SIM { .. }, .. } => {
            let mut eqData = (*eqData).clone();
            if init {
                assign_variant_field!(eqData => EqData::EqData::EQ_DATA_SIM; initials = BEquation::EquationPointers::map(var_field!((*eqData).initials, EqData::EqData::EQ_DATA_SIM).clone(), &*(func.clone()))?);
            } else {
                assign_variant_field!(eqData => EqData::EqData::EQ_DATA_SIM; equations = BEquation::EquationPointers::map(var_field!((*eqData).equations, EqData::EqData::EQ_DATA_SIM).clone(), &*(func.clone()))?);
            }
            assign_variant_field!(bdae => NBackendDAE::MAIN;
                eqData = BEquation::EqData::compress(eqData.clone())?,
                varData = updateDiscreteStates(var_field!((*bdae).varData, NBackendDAE::MAIN).clone(), acc_discrete_states, acc_previous)?
            );
            bdae
        },
        _ => {
            bdae
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(bdae)
}

pub(crate) fn removeStream(mut bdae: metamodelica::Ref<NBackendDAE>) -> Result<metamodelica::Ref<NBackendDAE>> {
    let mut bdae: metamodelica::Ref<NBackendDAE> = bdae;
    bdae = ({
        let mut acc_discrete_states: Pointer::Pointer<
            metamodelica::List<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>>,
        > = Pointer::create(metamodelica::nil());
        let mut acc_previous: Pointer::Pointer<
            metamodelica::List<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>>,
        > = Pointer::create(metamodelica::nil());
        (::match_deref::match_deref! { match &(bdae.clone()) {
            Deref @ MAIN { eqData: eqData @ Deref @ BEquation::EqData::EQ_DATA_SIM { .. }, .. } => {
                let mut eqData = (*eqData).clone();
                assign_variant_field!(eqData => EqData::EqData::EQ_DATA_SIM; equations = BEquation::EquationPointers::map(var_field!((*eqData).equations, EqData::EqData::EQ_DATA_SIM).clone(), &({ let __pe_b1 = literal!("NBackendDAE.removeStream"); let __pe_b2 = literal!(""); let __pe_b3 = acc_discrete_states.clone(); let __pe_b4 = acc_previous.clone(); let __pe_b5: Arc<dyn ::std::ops::Fn(metamodelica::Ref<Expression::NFExpression>) -> Result<metamodelica::Ref<Expression::NFExpression>> + 'static> = (std::sync::Arc::new(SimplifyExp::removeStream) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<Expression::NFExpression>) -> Result<metamodelica::Ref<Expression::NFExpression>> + 'static>); move |__pe_a0| BEquation::Equation::simplify(__pe_a0, &__pe_b1, &__pe_b2, __pe_b3.clone(), __pe_b4.clone(), __pe_b5.clone()) }))?);
                assign_variant_field!(bdae => NBackendDAE::MAIN;
                    eqData = BEquation::EqData::compress(eqData.clone())?,
                    varData = updateDiscreteStates(var_field!((*bdae).varData, NBackendDAE::MAIN).clone(), acc_discrete_states, acc_previous)?
                );
                bdae
            },
            _ => {
                bdae
            },
            _ => unreachable!("match_deref! exhaustiveness placeholder"),
        } })
    });
    Ok(bdae)
}

pub(crate) fn updateDiscreteStates(
    mut varData: metamodelica::Ref<VarData::VarData>,
    mut acc_discrete_states: Pointer::Pointer<
        metamodelica::List<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>>,
    >,
    mut acc_previous: Pointer::Pointer<metamodelica::List<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>>>,
) -> Result<metamodelica::Ref<VarData::VarData>> {
    let mut varData: metamodelica::Ref<VarData::VarData> = varData;
    varData = (match &*varData {
        BVariable::VarData::VAR_DATA_SIM {
            discrete_states: __varData_discrete_states,
            discretes: __varData_discretes,
            knowns: __varData_knowns,
            parameters: __varData_parameters,
            previous: __varData_previous,
            unknowns: __varData_unknowns,
            variables: __varData_variables,
            ..
        } => {
            let mut ads_accessed: metamodelica::List<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>>;
            let mut ap_accessed: metamodelica::List<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>>;
            ads_accessed = Pointer::access(acc_discrete_states);
            ap_accessed = Pointer::access(acc_previous);
            if !((ads_accessed).is_empty() && (ap_accessed).is_empty()) {
                BVariable::VariablePointers::removeList(&ads_accessed, __varData_unknowns.clone())?;
                BVariable::VariablePointers::removeList(&ads_accessed, __varData_discretes.clone())?;
                BVariable::VariablePointers::removeList(&ads_accessed, __varData_discrete_states.clone())?;
                BVariable::VariablePointers::removeList(&ap_accessed, __varData_previous.clone())?;
                BVariable::VariablePointers::removeList(&ap_accessed, __varData_variables.clone())?;
                BVariable::VariablePointers::addList(&ads_accessed, __varData_parameters.clone())?;
                BVariable::VariablePointers::addList(&ads_accessed, __varData_knowns.clone())?;
                for mut v in &*ads_accessed {
                    BVariable::setVarKind(
                        v.clone(),
                        metamodelica::Ref::new(VariableKind::VariableKind::PARAMETER { resize_value: None }),
                    );
                    BVariable::removePartner(
                        v.clone(),
                        &fnptr!(
                            BackendInfo::setVarPre,
                            metamodelica::Ref<BackendInfo::BackendInfo>,
                            Option<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>>
                        ),
                    )?;
                }
            }
            varData
        }
        _ => varData,
    });
    Ok(varData)
}

pub(crate) fn getLoopResiduals(
    mut bdae: &metamodelica::Ref<NBackendDAE>,
) -> Result<metamodelica::Ref<VariablePointers::VariablePointers>> {
    let mut residuals: metamodelica::Ref<VariablePointers::VariablePointers>;
    residuals = ({
        let mut var_lst: metamodelica::List<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>> =
            metamodelica::nil();
        (match &**bdae {
            MAIN {
                alg_event: __bdae_alg_event,
                algebraic: __bdae_algebraic,
                init: __bdae_init,
                ode: __bdae_ode,
                ode_event: __bdae_ode_event,
                ..
            } => {
                for mut syst in &*__bdae_ode.clone() {
                    var_lst = listAppend(
                        NBPartition::Partition::getLoopResiduals(metamodelica::AsArg::as_arg(&syst))?,
                        var_lst,
                    );
                }
                for mut syst in &*__bdae_algebraic.clone() {
                    var_lst = listAppend(
                        NBPartition::Partition::getLoopResiduals(metamodelica::AsArg::as_arg(&syst))?,
                        var_lst,
                    );
                }
                for mut syst in &*__bdae_ode_event.clone() {
                    var_lst = listAppend(
                        NBPartition::Partition::getLoopResiduals(metamodelica::AsArg::as_arg(&syst))?,
                        var_lst,
                    );
                }
                for mut syst in &*__bdae_alg_event.clone() {
                    var_lst = listAppend(
                        NBPartition::Partition::getLoopResiduals(metamodelica::AsArg::as_arg(&syst))?,
                        var_lst,
                    );
                }
                for mut syst in &*__bdae_init.clone() {
                    var_lst = listAppend(
                        NBPartition::Partition::getLoopResiduals(metamodelica::AsArg::as_arg(&syst))?,
                        var_lst,
                    );
                }
                residuals = BVariable::VariablePointers::fromList(&var_lst, false)?;
                residuals
            }
            _ => BVariable::VariablePointers::empty(BaseHashTable::bigBucketSize.clone(), false),
        })
    });
    Ok(residuals)
}

fn lowerVariableData(
    mut varList: metamodelica::List<metamodelica::Ref<Variable::NFVariable>>,
) -> Result<metamodelica::Ref<VarData::VarData>> {
    let mut variableData: metamodelica::Ref<VarData::VarData>;
    let mut lowVar: metamodelica::Ref<Variable::NFVariable>;
    let mut vars: metamodelica::List<metamodelica::Ref<Variable::NFVariable>>;
    let mut lowVar_ptr: Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>;
    let mut time_ptr: Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>;
    let mut dummy_ptr: Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>;
    let mut unknowns_lst: metamodelica::List<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>> =
        metamodelica::nil();
    let mut knowns_lst: metamodelica::List<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>> =
        metamodelica::nil();
    let mut initials_lst: metamodelica::List<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>> =
        metamodelica::nil();
    let mut auxiliaries_lst: metamodelica::List<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>> =
        metamodelica::nil();
    let mut aliasVars_lst: metamodelica::List<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>> =
        metamodelica::nil();
    let mut nonTrivialAlias_lst: metamodelica::List<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>> =
        metamodelica::nil();
    let mut states_lst: metamodelica::List<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>> =
        metamodelica::nil();
    let mut derivatives_lst: metamodelica::List<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>> =
        metamodelica::nil();
    let mut algebraics_lst: metamodelica::List<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>> =
        metamodelica::nil();
    let mut discretes_lst: metamodelica::List<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>> =
        metamodelica::nil();
    let mut discrete_states_lst: metamodelica::List<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>> =
        metamodelica::nil();
    let mut clocked_states_lst: metamodelica::List<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>> =
        metamodelica::nil();
    let mut previous_lst: metamodelica::List<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>> =
        metamodelica::nil();
    let mut clocks_lst: metamodelica::List<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>> =
        metamodelica::nil();
    let mut inputs_lst: metamodelica::List<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>> =
        metamodelica::nil();
    let mut resizables_lst: metamodelica::List<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>> =
        metamodelica::nil();
    let mut parameters_lst: metamodelica::List<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>> =
        metamodelica::nil();
    let mut constants_lst: metamodelica::List<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>> =
        metamodelica::nil();
    let mut records_lst: metamodelica::List<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>> =
        metamodelica::nil();
    let mut external_objects_lst: metamodelica::List<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>> =
        metamodelica::nil();
    let mut artificials_lst: metamodelica::List<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>> =
        metamodelica::nil();
    let mut variables: metamodelica::Ref<VariablePointers::VariablePointers>;
    let mut unknowns: metamodelica::Ref<VariablePointers::VariablePointers>;
    let mut knowns: metamodelica::Ref<VariablePointers::VariablePointers>;
    let mut initials: metamodelica::Ref<VariablePointers::VariablePointers>;
    let mut auxiliaries: metamodelica::Ref<VariablePointers::VariablePointers>;
    let mut aliasVars: metamodelica::Ref<VariablePointers::VariablePointers>;
    let mut nonTrivialAlias: metamodelica::Ref<VariablePointers::VariablePointers>;
    let mut states: metamodelica::Ref<VariablePointers::VariablePointers>;
    let mut derivatives: metamodelica::Ref<VariablePointers::VariablePointers>;
    let mut algebraics: metamodelica::Ref<VariablePointers::VariablePointers>;
    let mut discretes: metamodelica::Ref<VariablePointers::VariablePointers>;
    let mut discrete_states: metamodelica::Ref<VariablePointers::VariablePointers>;
    let mut clocked_states: metamodelica::Ref<VariablePointers::VariablePointers>;
    let mut previous: metamodelica::Ref<VariablePointers::VariablePointers>;
    let mut clocks: metamodelica::Ref<VariablePointers::VariablePointers>;
    let mut inputs: metamodelica::Ref<VariablePointers::VariablePointers>;
    let mut resizables: metamodelica::Ref<VariablePointers::VariablePointers>;
    let mut parameters: metamodelica::Ref<VariablePointers::VariablePointers>;
    let mut constants: metamodelica::Ref<VariablePointers::VariablePointers>;
    let mut records: metamodelica::Ref<VariablePointers::VariablePointers>;
    let mut external_objects: metamodelica::Ref<VariablePointers::VariablePointers>;
    let mut artificials: metamodelica::Ref<VariablePointers::VariablePointers>;
    let mut binding_iter_set: metamodelica::Ref<
        UnorderedSet::UnorderedSet<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>>,
    > = UnorderedSet::new(
        (std::sync::Arc::new(BVariable::hash)
            as std::sync::Arc<
                dyn ::std::ops::Fn(Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>) -> Result<i32> + 'static,
            >),
        (std::sync::Arc::new(BVariable::equalName)
            as std::sync::Arc<
                dyn ::std::ops::Fn(
                        Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>,
                        Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>,
                    ) -> Result<bool>
                    + 'static,
            >),
        13,
    );
    let mut binding_iter_lst: metamodelica::List<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>>;
    let mut scalarized: bool = Flags::isSet(Flags::NF_SCALARIZE.clone())?;
    let mut forced_states: metamodelica::List<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>> =
        metamodelica::nil();
    vars = List::flatten(
        ({
            let mut __acc: metamodelica::List<metamodelica::List<metamodelica::Ref<Variable::NFVariable>>> =
                metamodelica::nil();
            for mut v in (varList).into_iter().cloned() {
                let __x = Variable::expandChildren(v.clone(), &(metamodelica::nil()), true)?;
                __acc = cons(__x, __acc);
            }
            __acc.reverse()
        }),
    )?;
    variables = BVariable::VariablePointers::empty(((vars).len() as i32) + 1, scalarized);
    dummy_ptr = Pointer::create(BVariable::DUMMY_VARIABLE().clone());
    time_ptr = BVariable::createTimeVar()?;
    variables = BVariable::VariablePointers::add(dummy_ptr.clone(), variables)?;
    variables = BVariable::VariablePointers::add(time_ptr.clone(), variables)?;
    artificials_lst = list![dummy_ptr, time_ptr];
    for mut var in &*vars.reverse() {
        lowVar_ptr = lowerVariable(var.clone())?;
        lowVar = Pointer::access(lowVar_ptr.clone());
        variables = BVariable::VariablePointers::add(lowVar_ptr.clone(), variables)?;
        let () = (match &*lowVar.backendinfo.varKind.clone() {
            _ if (Variable::size(&lowVar, false)? == 0) => (),
            _ if (Variable::isTopLevelInput(&lowVar)) => {
                inputs_lst = metamodelica::cons(lowVar_ptr.clone(), inputs_lst);
                knowns_lst = metamodelica::cons(lowVar_ptr, knowns_lst);
                ()
            }
            VariableKind::ALGEBRAIC => {
                algebraics_lst = metamodelica::cons(lowVar_ptr.clone(), algebraics_lst);
                unknowns_lst = metamodelica::cons(lowVar_ptr.clone(), unknowns_lst);
                initials_lst = metamodelica::cons(lowVar_ptr, initials_lst);
                ()
            }
            VariableKind::STATE { natural, .. } => {
                let mut der_ptr: Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>;
                let mut der_cref: metamodelica::Ref<ComponentRef::NFComponentRef>;
                if !(natural.clone()) {
                    (der_cref, der_ptr) = BVariable::makeDerVar(BVariable::getVarName(lowVar_ptr.clone()), false)?;
                    if BVariable::VariablePointers::containsCref(
                        ComponentRef::stripSubscriptsAll(&der_cref),
                        &variables,
                    )? {
                        der_ptr = BVariable::VariablePointers::getVarSafe(
                            &variables,
                            ComponentRef::stripSubscriptsAll(&der_cref),
                            None,
                        )?;
                        BVariable::setStateDerKind(der_ptr.clone(), lowVar_ptr.clone());
                    } else {
                        variables = BVariable::VariablePointers::add(der_ptr.clone(), variables)?;
                        unknowns_lst = metamodelica::cons(der_ptr.clone(), unknowns_lst);
                        initials_lst = metamodelica::cons(der_ptr.clone(), initials_lst);
                    }
                    BVariable::setStateDerivativeVar(lowVar_ptr.clone(), der_ptr.clone());
                    derivatives_lst = metamodelica::cons(der_ptr, derivatives_lst);
                    forced_states = metamodelica::cons(lowVar_ptr.clone(), forced_states);
                }
                states_lst = metamodelica::cons(lowVar_ptr.clone(), states_lst);
                knowns_lst = metamodelica::cons(lowVar_ptr.clone(), knowns_lst);
                initials_lst = metamodelica::cons(lowVar_ptr, initials_lst);
                ()
            }
            VariableKind::STATE_DER { .. } => {
                derivatives_lst = metamodelica::cons(lowVar_ptr.clone(), derivatives_lst);
                unknowns_lst = metamodelica::cons(lowVar_ptr.clone(), unknowns_lst);
                initials_lst = metamodelica::cons(lowVar_ptr, initials_lst);
                ()
            }
            VariableKind::DISCRETE => {
                discretes_lst = metamodelica::cons(lowVar_ptr.clone(), discretes_lst);
                unknowns_lst = metamodelica::cons(lowVar_ptr.clone(), unknowns_lst);
                initials_lst = metamodelica::cons(lowVar_ptr, initials_lst);
                ()
            }
            VariableKind::PREVIOUS => {
                previous_lst = metamodelica::cons(lowVar_ptr.clone(), previous_lst);
                knowns_lst = metamodelica::cons(lowVar_ptr.clone(), knowns_lst);
                initials_lst = metamodelica::cons(lowVar_ptr, initials_lst);
                ()
            }
            VariableKind::PARAMETER { .. } => {
                if BVariable::isResizableParameter(lowVar_ptr.clone()) {
                    resizables_lst = metamodelica::cons(lowVar_ptr.clone(), resizables_lst);
                } else {
                    parameters_lst = metamodelica::cons(lowVar_ptr.clone(), parameters_lst);
                }
                knowns_lst = metamodelica::cons(lowVar_ptr, knowns_lst);
                ()
            }
            VariableKind::CONSTANT => {
                constants_lst = metamodelica::cons(lowVar_ptr.clone(), constants_lst);
                knowns_lst = metamodelica::cons(lowVar_ptr, knowns_lst);
                ()
            }
            VariableKind::RECORD { .. } => {
                records_lst = metamodelica::cons(lowVar_ptr.clone(), records_lst);
                knowns_lst = metamodelica::cons(lowVar_ptr, knowns_lst);
                ()
            }
            VariableKind::CLOCK => {
                clocks_lst = metamodelica::cons(lowVar_ptr, clocks_lst);
                ()
            }
            VariableKind::CLOCKED => {
                algebraics_lst = metamodelica::cons(lowVar_ptr.clone(), algebraics_lst);
                unknowns_lst = metamodelica::cons(lowVar_ptr.clone(), unknowns_lst);
                initials_lst = metamodelica::cons(lowVar_ptr, initials_lst);
                ()
            }
            VariableKind::EXTOBJ { .. } => {
                lowVar_ptr = BVariable::setFixed(lowVar_ptr, true, false)?;
                external_objects_lst = metamodelica::cons(lowVar_ptr.clone(), external_objects_lst);
                knowns_lst = metamodelica::cons(lowVar_ptr, knowns_lst);
                ()
            }
            _ => {
                Error::addMessage(
                    Error::INTERNAL_ERROR.clone(),
                    list![{
                        let mut __mm_s = String::new();
                        __mm_s.push_str(&*literal!("NBackendDAE.lowerVariableData"));
                        __mm_s.push_str(&*literal!(" failed for "));
                        __mm_s.push_str(&*BVariable::toString(metamodelica::AsArg::as_arg(&var), literal!(""))?);
                        ArcStr::from(__mm_s)
                    }],
                )?;
                return Err("fail");
            }
        });
    }
    algebraics_lst = ({
        let mut __acc: metamodelica::List<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>> =
            metamodelica::nil();
        for mut p in (algebraics_lst).into_iter().cloned() {
            if !(!(BVariable::isStateDerivative(p.clone()))) {
                continue;
            }
            let __x = p.clone();
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    unknowns = BVariable::VariablePointers::fromList(&unknowns_lst, scalarized)?;
    knowns = BVariable::VariablePointers::fromList(&knowns_lst, scalarized)?;
    initials = BVariable::VariablePointers::fromList(&initials_lst, scalarized)?;
    auxiliaries = BVariable::VariablePointers::fromList(&auxiliaries_lst, scalarized)?;
    aliasVars = BVariable::VariablePointers::fromList(&aliasVars_lst, scalarized)?;
    nonTrivialAlias = BVariable::VariablePointers::fromList(&nonTrivialAlias_lst, scalarized)?;
    states = BVariable::VariablePointers::fromList(&states_lst, scalarized)?;
    derivatives = BVariable::VariablePointers::fromList(&derivatives_lst, scalarized)?;
    algebraics = BVariable::VariablePointers::fromList(&algebraics_lst, scalarized)?;
    discretes = BVariable::VariablePointers::fromList(&discretes_lst, scalarized)?;
    discrete_states = BVariable::VariablePointers::fromList(&discrete_states_lst, scalarized)?;
    clocked_states = BVariable::VariablePointers::fromList(&clocked_states_lst, scalarized)?;
    previous = BVariable::VariablePointers::fromList(&previous_lst, scalarized)?;
    clocks = BVariable::VariablePointers::fromList(&clocks_lst, scalarized)?;
    inputs = BVariable::VariablePointers::fromList(&inputs_lst, scalarized)?;
    resizables = BVariable::VariablePointers::fromList(&resizables_lst, scalarized)?;
    parameters = BVariable::VariablePointers::fromList(&parameters_lst, scalarized)?;
    constants = BVariable::VariablePointers::fromList(&constants_lst, scalarized)?;
    records = BVariable::VariablePointers::fromList(&records_lst, scalarized)?;
    external_objects = BVariable::VariablePointers::fromList(&external_objects_lst, scalarized)?;
    artificials = BVariable::VariablePointers::fromList(&artificials_lst, scalarized)?;
    variables = BVariable::VariablePointers::map(
        variables.clone(),
        &({
            let __pe_b1 = variables;
            let __pe_b2 = binding_iter_set.clone();
            move |__pe_a0| collectVariableBindingIterators(__pe_a0, &__pe_b1, __pe_b2.clone())
        }),
    )?;
    binding_iter_lst = UnorderedSet::toList(binding_iter_set);
    variables = BVariable::VariablePointers::addList(&binding_iter_lst, variables)?;
    knowns = BVariable::VariablePointers::addList(&binding_iter_lst, knowns)?;
    artificials = BVariable::VariablePointers::addList(&binding_iter_lst, artificials)?;
    variables = BVariable::VariablePointers::map(
        variables.clone(),
        &({
            let __pe_b1 = (std::sync::Arc::new({
                let __pe_b1 = variables;
                let __pe_b2 = true;
                move |__pe_a0| lowerComponentReferenceExp(__pe_a0, &__pe_b1, __pe_b2.clone())
            })
                as std::sync::Arc<
                    dyn ::std::ops::Fn(
                            metamodelica::Ref<Expression::NFExpression>,
                        ) -> Result<metamodelica::Ref<Expression::NFExpression>>
                        + 'static,
                >);
            move |__pe_a0| Variable::mapExp(__pe_a0, __pe_b1.clone())
        }),
    )?;
    variables = BVariable::VariablePointers::map(
        variables.clone(),
        &({
            let __pe_b1: Arc<
                dyn ::std::ops::Fn(metamodelica::Ref<Type::NFType>) -> Result<metamodelica::Ref<Type::NFType>>
                    + 'static,
            > = (std::sync::Arc::new({
                let __pe_b1: Arc<
                    dyn ::std::ops::Fn(
                            metamodelica::Ref<Dimension::NFDimension>,
                        ) -> Result<metamodelica::Ref<Dimension::NFDimension>>
                        + 'static,
                > = (std::sync::Arc::new({
                    let __pe_b1 = variables;
                    let __pe_b2 = true;
                    move |__pe_a0| lowerDimension(__pe_a0, &__pe_b1, __pe_b2.clone())
                })
                    as std::sync::Arc<
                        dyn ::std::ops::Fn(
                                metamodelica::Ref<Dimension::NFDimension>,
                            )
                                -> Result<metamodelica::Ref<Dimension::NFDimension>>
                            + 'static,
                    >);
                move |__pe_a0| Type::applyToDims(__pe_a0, &*__pe_b1)
            })
                as std::sync::Arc<
                    dyn ::std::ops::Fn(metamodelica::Ref<Type::NFType>) -> Result<metamodelica::Ref<Type::NFType>>
                        + 'static,
                >);
            move |__pe_a0| Variable::applyToType(__pe_a0, &*__pe_b1)
        }),
    )?;
    records = BVariable::VariablePointers::mapPtr(
        records,
        &({
            let __pe_b1 = variables.clone();
            move |__pe_a0| lowerRecordChildren(__pe_a0, &__pe_b1)
        }),
    )?;
    variableData = metamodelica::Ref::new(VarData::VarData::VAR_DATA_SIM {
        uniqueIndex: Pointer::create(0),
        variables: variables,
        unknowns: unknowns,
        knowns: knowns,
        initials: initials,
        auxiliaries: auxiliaries,
        aliasVars: aliasVars,
        nonTrivialAlias: nonTrivialAlias,
        derivatives: derivatives,
        algebraics: algebraics,
        discretes: discretes,
        discrete_states: discrete_states,
        clocked_states: clocked_states,
        previous: previous,
        clocks: clocks,
        states: states,
        top_level_inputs: inputs,
        resizables: resizables,
        parameters: parameters,
        constants: constants,
        records: records,
        external_objects: external_objects,
        artificials: artificials,
        state_order: UnorderedMap::new(
            (std::sync::Arc::new(move |__a0: metamodelica::Ref<ComponentRef::NFComponentRef>| ComponentRef::hash(&__a0))
                as std::sync::Arc<
                    dyn ::std::ops::Fn(metamodelica::Ref<ComponentRef::NFComponentRef>) -> Result<i32> + 'static,
                >),
            (std::sync::Arc::new(
                move |__a0: metamodelica::Ref<ComponentRef::NFComponentRef>,
                      __a1: metamodelica::Ref<ComponentRef::NFComponentRef>| {
                    ComponentRef::isEqual(&__a0, &__a1)
                },
            )
                as std::sync::Arc<
                    dyn ::std::ops::Fn(
                            metamodelica::Ref<ComponentRef::NFComponentRef>,
                            metamodelica::Ref<ComponentRef::NFComponentRef>,
                        ) -> Result<bool>
                        + 'static,
                >),
            1,
        ),
    });
    if Flags::isSet(Flags::DUMP_STATESELECTION_INFO.clone())? {
        metamodelica::print(StringUtil::headline_4(
            &({
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("[stateselection] ("));
                __mm_s.push_str(&*intString(((forced_states).len() as i32)));
                __mm_s.push_str(&*literal!(") Forced states by StateSelect.ALWAYS:"));
                ArcStr::from(__mm_s)
            }),
        )?);
        if (forced_states).is_empty() {
            metamodelica::print(literal!("\t<no states>\n\n"));
        } else {
            metamodelica::print({
                let mut __mm_s = String::new();
                __mm_s.push_str(&*List::toString(
                    forced_states,
                    &BVariable::pointerToString,
                    List::Style::NEWLINE_TAB.clone(),
                )?);
                __mm_s.push_str(&*literal!("\n\n"));
                ArcStr::from(__mm_s)
            });
        }
    }
    Ok(variableData)
}

fn lowerVariable(
    mut var: metamodelica::Ref<Variable::NFVariable>,
) -> Result<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>> {
    let mut var_ptr: Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>;
    let mut varKind: metamodelica::Ref<VariableKind::VariableKind>;
    let mut attributes: metamodelica::Ref<VariableAttributes::VariableAttributes>;
    let mut annotations: metamodelica::Ref<Annotations::Annotations>;
    match '__try0: {
        attributes = unwrap_break_err!(VariableAttributes::create(&var.typeAttributes, &var.ty, &var.attributes, &var.children, &var.comment), '__try0);
        annotations = Annotations::create(&var.comment, &var.attributes);
        assign_field!(
            var.backendinfo = (::match_deref::match_deref! { match &(var.backendinfo.clone()) {
                Deref @ BackendInfo::BACKEND_INFO { varKind: Deref @ VariableKind::FRONTEND_DUMMY, .. } => {
                    (varKind, attributes) = unwrap_break_err!(lowerVariableKind(&(var.clone()), attributes.clone(), var.ty.clone()), '__try0);
                    metamodelica::Ref::new(BackendInfo::BackendInfo { varKind: varKind.clone(), attributes: attributes.clone(), annotations: annotations.clone(), var_pre: None, var_seed: None, var_pder_res: None, var_pder_tmp: None, var_start: None, parent: None })
                },
                _ => BackendInfo::setAttributes(var.backendinfo.clone(), attributes.clone(), annotations.clone()),
                _ => unreachable!("match_deref! exhaustiveness placeholder"),
            } }),
            var.typeAttributes = metamodelica::nil()
        );
        (var_ptr, _) = unwrap_break_err!(BVariable::makeVarPtr(var.clone(), var.name.clone()), '__try0);
        Ok::<_, &'static str>((annotations.clone(), attributes.clone(), var.clone(), var_ptr.clone()))
    } {
        Ok((__try0_o0, __try0_o1, __try0_o2, __try0_o3)) => {
            annotations = __try0_o0;
            attributes = __try0_o1;
            var = __try0_o2;
            var_ptr = __try0_o3;
        }
        Err(__try0_err) => {
            Error::addMessage(
                Error::INTERNAL_ERROR.clone(),
                list![{
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!("NBackendDAE.lowerVariable"));
                    __mm_s.push_str(&*literal!(" failed for "));
                    __mm_s.push_str(&*Variable::toString(&var, literal!(""), false)?);
                    ArcStr::from(__mm_s)
                }],
            )?;
            return Err(__try0_err);
        }
    }
    Ok(var_ptr)
}

fn lowerVariableKind(
    mut var: &metamodelica::Ref<Variable::NFVariable>,
    mut attributes: metamodelica::Ref<VariableAttributes::VariableAttributes>,
    mut ty: metamodelica::Ref<Type::NFType>,
) -> Result<(
    metamodelica::Ref<VariableKind::VariableKind>,
    metamodelica::Ref<VariableAttributes::VariableAttributes>,
)> {
    fn lowerRecordKind(
        mut children: &metamodelica::List<metamodelica::Ref<Variable::NFVariable>>,
    ) -> (Prefixes::Variability, Prefixes::Variability) {
        let mut min_var: Prefixes::Variability = Prefixes::Variability::CONTINUOUS.clone();
        let mut max_var: Prefixes::Variability = Prefixes::Variability::CONSTANT.clone();
        let mut tmp_min_var: Prefixes::Variability;
        let mut tmp_max_var: Prefixes::Variability;
        for mut child in &**children {
            (tmp_min_var, tmp_max_var) = (::match_deref::match_deref! { match &(child.ty.clone()) {
                Deref @ Type::COMPLEX { .. } => lowerRecordKind(&child.children),
                Deref @ Type::ARRAY { elementType: Deref @ Type::COMPLEX { .. }, .. } => lowerRecordKind(&child.children),
                _ => (Variable::variability(metamodelica::AsArg::as_arg(&child)), Variable::variability(metamodelica::AsArg::as_arg(&child))),
                _ => unreachable!("match_deref! exhaustiveness placeholder"),
            } });
            min_var = if (tmp_min_var < min_var) { tmp_min_var } else { min_var };
            max_var = if (tmp_max_var > max_var) { tmp_max_var } else { max_var };
        }
        (min_var, max_var)
    }

    let mut varKind: metamodelica::Ref<VariableKind::VariableKind>;
    let mut attributes: metamodelica::Ref<VariableAttributes::VariableAttributes> = attributes;
    let mut min_var: Prefixes::Variability;
    let mut max_var: Prefixes::Variability;
    let mut variability: Prefixes::Variability = Variable::variability(var);
    varKind = ({
        let mut children: metamodelica::List<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>> =
            metamodelica::nil();
        (::match_deref::match_deref! { match &((variability, &*attributes, &*ty)) {
            (_, _, Deref @ Type::CLOCK) => {
                openmodelica_nf_frontend::NFBackendExtension::VariableKind::interned_CLOCK()
            },
            (_, _, _) if (Binding::isClockOrSampleFunction(&var.binding)?) => {
                openmodelica_nf_frontend::NFBackendExtension::VariableKind::interned_CLOCKED()
            },
            (Prefixes::Variability::CONTINUOUS, Deref @ VariableAttributes::VAR_ATTR_REAL { stateSelect: Some(NFBackendExtension::StateSelect::ALWAYS), .. }, _) if (variability == Prefixes::Variability::CONTINUOUS.clone()) => {
                metamodelica::Ref::new(VariableKind::VariableKind::STATE { index: 1, derivative: None, natural: false })
            },
            (_, _, Deref @ Type::COMPLEX { complexTy: Deref @ ComplexType::EXTERNAL_OBJECT { .. }, .. }) => {
                metamodelica::Ref::new(VariableKind::VariableKind::EXTOBJ { fullClassName: Class::constrainingClassPath(Type::complexNode(&ty)?)? })
            },
            (_, _, Deref @ Type::ARRAY { elementType: elemTy @ Deref @ Type::COMPLEX { complexTy: Deref @ ComplexType::EXTERNAL_OBJECT { .. }, .. }, .. }) => {
                metamodelica::Ref::new(VariableKind::VariableKind::EXTOBJ { fullClassName: Class::constrainingClassPath(Type::complexNode(metamodelica::AsArg::as_arg(&elemTy))?)? })
            },
            (_, _, Deref @ Type::COMPLEX { .. }) => {
                (min_var, max_var) = lowerRecordKind(&var.children);
                metamodelica::Ref::new(VariableKind::VariableKind::RECORD { children: metamodelica::nil(), min_var: min_var, max_var: max_var })
            },
            (_, _, Deref @ Type::ARRAY { elementType: Deref @ Type::COMPLEX { .. }, .. }) => {
                (min_var, max_var) = lowerRecordKind(&var.children);
                metamodelica::Ref::new(VariableKind::VariableKind::RECORD { children: metamodelica::nil(), min_var: min_var, max_var: max_var })
            },
            (Prefixes::Variability::CONTINUOUS, _, Deref @ Type::BOOLEAN) => {
                openmodelica_nf_frontend::NFBackendExtension::VariableKind::interned_DISCRETE()
            },
            (Prefixes::Variability::CONTINUOUS, _, Deref @ Type::INTEGER) => {
                openmodelica_nf_frontend::NFBackendExtension::VariableKind::interned_DISCRETE()
            },
            (Prefixes::Variability::CONTINUOUS, _, Deref @ Type::ENUMERATION { .. }) => {
                openmodelica_nf_frontend::NFBackendExtension::VariableKind::interned_DISCRETE()
            },
            (Prefixes::Variability::CONTINUOUS, _, _) => {
                openmodelica_nf_frontend::NFBackendExtension::VariableKind::interned_ALGEBRAIC()
            },
            (Prefixes::Variability::DISCRETE, _, _) => {
                openmodelica_nf_frontend::NFBackendExtension::VariableKind::interned_DISCRETE()
            },
            (Prefixes::Variability::IMPLICITLY_DISCRETE, _, _) => {
                openmodelica_nf_frontend::NFBackendExtension::VariableKind::interned_DISCRETE()
            },
            (Prefixes::Variability::PARAMETER, _, _) => {
                metamodelica::Ref::new(VariableKind::VariableKind::PARAMETER { resize_value: None })
            },
            (Prefixes::Variability::STRUCTURAL_PARAMETER, _, _) => {
                metamodelica::Ref::new(VariableKind::VariableKind::PARAMETER { resize_value: None })
            },
            (Prefixes::Variability::NON_STRUCTURAL_PARAMETER, _, _) => {
                metamodelica::Ref::new(VariableKind::VariableKind::PARAMETER { resize_value: None })
            },
            (Prefixes::Variability::CONSTANT, _, _) => {
                openmodelica_nf_frontend::NFBackendExtension::VariableKind::interned_CONSTANT()
            },
            _ => {
                Error::addMessage(Error::INTERNAL_ERROR.clone(), list![{ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBackendDAE.lowerVariableKind")); __mm_s.push_str(&*literal!(" failed.")); ArcStr::from(__mm_s) }])?;
                return Err("fail")
            },
            _ => unreachable!("match_deref! exhaustiveness placeholder"),
        } })
    });
    attributes = (match &*varKind {
        VariableKind::PARAMETER { .. } => VariableAttributes::setFixed(attributes, ty, true, false)?,
        _ => attributes,
    });
    Ok((varKind, attributes))
}

fn collectVariableBindingIterators(
    mut var: metamodelica::Ref<Variable::NFVariable>,
    mut variables: &metamodelica::Ref<VariablePointers::VariablePointers>,
    mut set: metamodelica::Ref<UnorderedSet::UnorderedSet<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>>>,
) -> Result<metamodelica::Ref<Variable::NFVariable>> {
    let mut var: metamodelica::Ref<Variable::NFVariable> = var;
    let mut exp_opt: Option<metamodelica::Ref<Expression::NFExpression>>;
    BackendInfo::map(
        var.backendinfo.clone(),
        (std::sync::Arc::new({
            let __pe_b1 = variables.clone();
            let __pe_b2 = set.clone();
            move |__pe_a0| collectIterators(__pe_a0, &__pe_b1, __pe_b2.clone())
        })
            as std::sync::Arc<
                dyn ::std::ops::Fn(
                        metamodelica::Ref<Expression::NFExpression>,
                    ) -> Result<metamodelica::Ref<Expression::NFExpression>>
                    + 'static,
            >),
    )?;
    exp_opt = Binding::typedExp(&var.binding);
    if (exp_opt).is_some() {
        Expression::map(
            Util::getOption(exp_opt)?,
            (std::sync::Arc::new({
                let __pe_b1 = variables.clone();
                let __pe_b2 = set;
                move |__pe_a0| collectIterators(__pe_a0, &__pe_b1, __pe_b2.clone())
            })
                as std::sync::Arc<
                    dyn ::std::ops::Fn(
                            metamodelica::Ref<Expression::NFExpression>,
                        ) -> Result<metamodelica::Ref<Expression::NFExpression>>
                        + 'static,
                >),
        )?;
    }
    Ok(var)
}

pub(crate) fn lowerRecordChildren(
    mut var_ptr: Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>,
    mut variables: &metamodelica::Ref<VariablePointers::VariablePointers>,
) -> Result<()> {
    let mut var: metamodelica::Ref<Variable::NFVariable> = Pointer::access(var_ptr.clone());
    var = (::match_deref::match_deref! { match &(var.clone()) {
        Deref @ Variable::VARIABLE { backendinfo: binfo @ Deref @ BackendInfo::BACKEND_INFO { varKind: varKind @ Deref @ VariableKind::RECORD { .. }, .. }, .. } => {
            let mut binfo = (*binfo).clone();
            let mut varKind = (*varKind).clone();
            assign_variant_field!(varKind => VariableKind::VariableKind::RECORD; children = ({
        let mut __acc: metamodelica::List<PointerWeak::PointerWeak<metamodelica::Ref<Variable::NFVariable>>> = metamodelica::nil();
        for mut child in (var.children.clone()).into_iter().cloned() {
            let __x = PointerWeak::downgrade(BVariable::VariablePointers::getVarSafe(variables, ComponentRef::stripSubscriptsAll(&(child.name.clone())), Some(metamodelica::sourceInfo!("NBackEnd/Classes/NBackendDAE.mo")))?);
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }));
            assign_variant_field!(varKind => VariableKind::VariableKind::RECORD; children = ({
        let mut __acc: metamodelica::List<PointerWeak::PointerWeak<metamodelica::Ref<Variable::NFVariable>>> = metamodelica::nil();
        for mut child in (var_field!((*varKind).children, VariableKind::VariableKind::RECORD).clone()).into_iter().cloned() {
            let __x = PointerWeak::downgrade(BVariable::setParent(PointerWeak::upgrade(child.clone())?, var_ptr.clone()));
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }));
            assign_field!(binfo.varKind = varKind.clone());
            assign_field!(var.backendinfo = binfo.clone());
            var
        },
        _ => {
            var
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Pointer::update(var_ptr, var);
    Ok(())
}

pub(crate) fn lowerUnkownRecordChildren(
    mut var_ptr: Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>,
    mut variables: &metamodelica::Ref<VariablePointers::VariablePointers>,
) -> Result<()> {
    if BVariable::isUnknownRecord(var_ptr.clone()) {
        lowerRecordChildren(var_ptr, variables)?;
    }
    Ok(())
}

fn lowerEquationData(
    mut eq_lst: &metamodelica::List<metamodelica::Ref<FEquation::NFEquation>>,
    mut al_lst: &metamodelica::List<metamodelica::Ref<Algorithm::NFAlgorithm>>,
    mut init_eq_lst: &metamodelica::List<metamodelica::Ref<FEquation::NFEquation>>,
    mut init_al_lst: &metamodelica::List<metamodelica::Ref<Algorithm::NFAlgorithm>>,
    mut varData: metamodelica::Ref<VarData::VarData>,
) -> Result<(metamodelica::Ref<EqData::EqData>, metamodelica::Ref<VarData::VarData>)> {
    let mut eqData: metamodelica::Ref<EqData::EqData>;
    let mut varData: metamodelica::Ref<VarData::VarData> = varData;
    let mut set: metamodelica::Ref<
        UnorderedSet::UnorderedSet<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>>,
    > = UnorderedSet::new(
        (std::sync::Arc::new(BVariable::hash)
            as std::sync::Arc<
                dyn ::std::ops::Fn(Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>) -> Result<i32> + 'static,
            >),
        (std::sync::Arc::new(BVariable::equalName)
            as std::sync::Arc<
                dyn ::std::ops::Fn(
                        Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>,
                        Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>,
                    ) -> Result<bool>
                    + 'static,
            >),
        13,
    );
    let mut equation_lst: metamodelica::List<Pointer::Pointer<metamodelica::Ref<Equation::Equation>>>;
    let mut continuous_lst: metamodelica::List<Pointer::Pointer<metamodelica::Ref<Equation::Equation>>>;
    let mut clocked_lst: metamodelica::List<Pointer::Pointer<metamodelica::Ref<Equation::Equation>>>;
    let mut discretes_lst: metamodelica::List<Pointer::Pointer<metamodelica::Ref<Equation::Equation>>>;
    let mut initials_lst: metamodelica::List<Pointer::Pointer<metamodelica::Ref<Equation::Equation>>>;
    let mut auxiliaries_lst: metamodelica::List<Pointer::Pointer<metamodelica::Ref<Equation::Equation>>>;
    let mut simulation_lst: metamodelica::List<Pointer::Pointer<metamodelica::Ref<Equation::Equation>>>;
    let mut removed_lst: metamodelica::List<Pointer::Pointer<metamodelica::Ref<Equation::Equation>>>;
    let mut equations: metamodelica::Ref<EquationPointers::EquationPointers>;
    let mut idx: Pointer::Pointer<i32> = Pointer::create(0);
    equation_lst = lowerEquationsAndAlgorithms(eq_lst, al_lst, init_eq_lst, init_al_lst)?;
    for mut eqn_ptr in &*equation_lst {
        BEquation::Equation::createName(
            eqn_ptr.clone(),
            idx.clone(),
            &(arcstr::literal!(BEquation::SIMULATION_STR)),
        )?;
        BEquation::Equation::renameIterators(eqn_ptr.clone(), &(literal!("$i")))?;
        lowerEquationIterators(
            Pointer::access(eqn_ptr.clone()),
            &(BVariable::VarData::getVariables(&varData)?),
            set.clone(),
        )?;
    }
    varData = BVariable::VarData::addTypedList(
        varData,
        &(UnorderedSet::toList(set)),
        BVariable::VarData::VarType::ITERATOR.clone(),
    )?;
    equations = BEquation::EquationPointers::fromList(&equation_lst)?;
    equations = lowerComponentReferences(equations, &(BVariable::VarData::getVariables(&varData)?))?;
    (
        simulation_lst,
        continuous_lst,
        clocked_lst,
        discretes_lst,
        initials_lst,
        auxiliaries_lst,
        removed_lst,
    ) = BEquation::typeList(&(BEquation::EquationPointers::toList(&equations)?))?;
    equations = BEquation::EquationPointers::removeList(&clocked_lst, equations)?;
    (equations, _) = Resizable::resize(equations, varData.clone())?;
    eqData = metamodelica::Ref::new(EqData::EqData::EQ_DATA_SIM {
        uniqueIndex: idx,
        equations: equations,
        simulation: BEquation::EquationPointers::fromList(&simulation_lst)?,
        continuous: BEquation::EquationPointers::fromList(&continuous_lst)?,
        clocked: BEquation::EquationPointers::fromList(&clocked_lst)?,
        discretes: BEquation::EquationPointers::fromList(&discretes_lst)?,
        initials: BEquation::EquationPointers::fromList(&initials_lst)?,
        auxiliaries: BEquation::EquationPointers::fromList(&auxiliaries_lst)?,
        removed: BEquation::EquationPointers::fromList(&removed_lst)?,
    });
    Ok((eqData, varData))
}

fn lowerEquationsAndAlgorithms(
    mut eq_lst: &metamodelica::List<metamodelica::Ref<FEquation::NFEquation>>,
    mut al_lst: &metamodelica::List<metamodelica::Ref<Algorithm::NFAlgorithm>>,
    mut init_eq_lst: &metamodelica::List<metamodelica::Ref<FEquation::NFEquation>>,
    mut init_al_lst: &metamodelica::List<metamodelica::Ref<Algorithm::NFAlgorithm>>,
) -> Result<metamodelica::List<Pointer::Pointer<metamodelica::Ref<Equation::Equation>>>> {
    let mut equations: metamodelica::List<Pointer::Pointer<metamodelica::Ref<Equation::Equation>>> =
        metamodelica::nil();
    for mut eq in &**eq_lst {
        equations = listAppend(
            lowerEquation(metamodelica::AsArg::as_arg(&eq), false, false)?,
            equations,
        );
    }
    for mut alg in &**al_lst {
        equations = metamodelica::cons(lowerAlgorithm(alg.clone(), false)?, equations);
    }
    for mut eq in &**init_eq_lst {
        equations = listAppend(lowerEquation(metamodelica::AsArg::as_arg(&eq), true, false)?, equations);
    }
    for mut alg in &**init_al_lst {
        equations = metamodelica::cons(lowerAlgorithm(alg.clone(), true)?, equations);
    }
    Ok(equations)
}

fn lowerEquation(
    mut frontend_equation: &metamodelica::Ref<FEquation::NFEquation>,
    mut init: bool,
    mut in_for: bool,
) -> Result<metamodelica::List<Pointer::Pointer<metamodelica::Ref<Equation::Equation>>>> {
    let mut backend_equations: metamodelica::List<Pointer::Pointer<metamodelica::Ref<Equation::Equation>>>;
    backend_equations = (match &**frontend_equation {
        FEquation::EQUALITY {
            lhs, rhs, ty, source, ..
        } => {
            let mut attr: metamodelica::Ref<EquationAttributes::EquationAttributes>;
            attr = lowerEquationAttributes(ty.clone(), init)?;
            backend_equations = (match &*ty.clone() {
                Type::ARRAY { .. } => list![Pointer::create(metamodelica::Ref::new(
                    Equation::Equation::ARRAY_EQUATION {
                        ty: ty.clone(),
                        lhs: lhs.clone(),
                        rhs: rhs.clone(),
                        source: source.clone(),
                        attr: attr,
                        recordSize: Type::complexSize(ty, false)?
                    }
                ))],
                Type::COMPLEX { .. } => list![Pointer::create(metamodelica::Ref::new(
                    Equation::Equation::RECORD_EQUATION {
                        ty: ty.clone(),
                        lhs: lhs.clone(),
                        rhs: rhs.clone(),
                        source: source.clone(),
                        attr: attr,
                        recordSize: Type::recordFieldCount(ty)
                    }
                ))],
                Type::TUPLE { .. } => list![Pointer::create(metamodelica::Ref::new(
                    Equation::Equation::RECORD_EQUATION {
                        ty: ty.clone(),
                        lhs: lhs.clone(),
                        rhs: rhs.clone(),
                        source: source.clone(),
                        attr: attr,
                        recordSize: Type::tupleFieldCount(ty)
                    }
                ))],
                _ => list![Pointer::create(metamodelica::Ref::new(
                    Equation::Equation::SCALAR_EQUATION {
                        ty: ty.clone(),
                        lhs: lhs.clone(),
                        rhs: rhs.clone(),
                        source: source.clone(),
                        attr: attr
                    }
                ))],
            });
            backend_equations
        }
        FEquation::FOR { .. } => lowerForEquation(frontend_equation, init)?,
        FEquation::IF { .. } => lowerIfEquation(frontend_equation, init, in_for)?,
        FEquation::WHEN { .. } => lowerWhenEquation(frontend_equation, init)?,
        FEquation::ASSERT { .. } => lowerAssert(frontend_equation, init)?,
        FEquation::NORETCALL {
            exp: __frontend_equation_exp,
            source: __frontend_equation_source,
            ..
        } => {
            let mut stmt: metamodelica::Ref<Statement::NFStatement>;
            let mut alg: metamodelica::Ref<Algorithm::NFAlgorithm>;
            stmt = metamodelica::Ref::new(Statement::NFStatement::NORETCALL {
                exp: __frontend_equation_exp.clone(),
                source: __frontend_equation_source.clone(),
            });
            alg = metamodelica::Ref::new(Algorithm::NFAlgorithm {
                statements: list![stmt],
                inputs: metamodelica::nil(),
                outputs: metamodelica::nil(),
                stmtDiffInfo: None,
                scope: NFInstNode::NO_SCOPE().clone(),
                source: __frontend_equation_source.clone(),
            });
            alg = Algorithm::setInputsOutputs(alg)?;
            list![lowerAlgorithm(alg, init)?]
        }
        FEquation::TERMINATE { .. } => {
            Error::addMessage(
                Error::INTERNAL_ERROR.clone(),
                list![{
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!("NBackendDAE.lowerEquation"));
                    __mm_s.push_str(&*literal!(" failed for TERMINATE expression without condition:\n"));
                    __mm_s.push_str(&*FEquation::toString(frontend_equation, literal!(""))?);
                    ArcStr::from(__mm_s)
                }],
            )?;
            return Err("fail");
        }
        FEquation::REINIT { .. } => {
            Error::addMessage(
                Error::INTERNAL_ERROR.clone(),
                list![{
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!("NBackendDAE.lowerEquation"));
                    __mm_s.push_str(&*literal!(" failed for REINIT expression without condition:\n"));
                    __mm_s.push_str(&*FEquation::toString(frontend_equation, literal!(""))?);
                    ArcStr::from(__mm_s)
                }],
            )?;
            return Err("fail");
        }
        _ => {
            Error::addMessage(
                Error::INTERNAL_ERROR.clone(),
                list![{
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!("NBackendDAE.lowerEquation"));
                    __mm_s.push_str(&*literal!(" failed for\n"));
                    __mm_s.push_str(&*FEquation::toString(frontend_equation, literal!(""))?);
                    ArcStr::from(__mm_s)
                }],
            )?;
            return Err("fail");
        }
    });
    Ok(backend_equations)
}

fn lowerForEquation(
    mut frontend_equation: &metamodelica::Ref<FEquation::NFEquation>,
    mut init: bool,
) -> Result<metamodelica::List<Pointer::Pointer<metamodelica::Ref<Equation::Equation>>>> {
    let mut backend_equations: metamodelica::List<Pointer::Pointer<metamodelica::Ref<Equation::Equation>>> =
        metamodelica::nil();
    let mut range: metamodelica::Ref<Expression::NFExpression>;
    let mut new_body: metamodelica::List<Pointer::Pointer<metamodelica::Ref<Equation::Equation>>> = metamodelica::nil();
    let mut body_elem: metamodelica::Ref<Equation::Equation>;
    let mut body: metamodelica::List<metamodelica::Ref<FEquation::NFEquation>> = metamodelica::nil();
    let mut bodies: metamodelica::List<metamodelica::Ref<IfEquationBody::IfEquationBody>>;
    let mut iterator: metamodelica::Ref<ComponentRef::NFComponentRef>;
    let mut isAlgorithm: bool;
    let mut alg: metamodelica::Ref<Algorithm::NFAlgorithm>;
    let mut size: i32;
    backend_equations = (::match_deref::match_deref! { match frontend_equation {
        Deref @ FEquation::FOR { range: Some(__esc_range), body: __frontend_equation_body, iterator: __frontend_equation_iterator, source: __frontend_equation_source, .. } => {
            range = (*__esc_range).clone();
            if Expression::rangeSize(range.clone(), false)? > 0 {
                iterator = ComponentRef::fromNode(__frontend_equation_iterator.clone(), openmodelica_nf_frontend::NFType::interned_INTEGER(), metamodelica::nil(), ComponentRef::Origin::ITERATOR.clone())?;
                for mut eq in &*__frontend_equation_body.clone() {
                    for mut body_elem_ptr in &*lowerEquation(metamodelica::AsArg::as_arg(&eq), init, true)? {
                        body_elem = Pointer::access(body_elem_ptr.clone());
                        new_body = (match &*body_elem {
        BEquation::Equation::IF_EQUATION { attr: __body_elem_attr, body: __body_elem_body, source: __body_elem_source, .. } => {
            bodies = BEquation::IfEquationBody::split(__body_elem_body.clone())?;
            for mut body in &*bodies {
                let mut body = body.clone();
                new_body = metamodelica::cons(Pointer::create(metamodelica::Ref::new(Equation::Equation::IF_EQUATION { size: BEquation::IfEquationBody::size(&body, false)?, body: body, source: __body_elem_source.clone(), attr: __body_elem_attr.clone() })), new_body);
            }
            new_body
        },
        _ => metamodelica::cons(body_elem_ptr.clone(), new_body),
    });
                    }
                }
                for mut body_elem_ptr in &*new_body {
                    body_elem = Pointer::access(body_elem_ptr.clone());
                    isAlgorithm = BEquation::Equation::isAlgorithm(body_elem_ptr.clone());
                    body_elem = metamodelica::Ref::new(Equation::Equation::FOR_EQUATION { size: Expression::rangeSize(range.clone(), false)? * BEquation::Equation::size(body_elem_ptr.clone(), false)?, iter: metamodelica::Ref::new(Iterator::Iterator::SINGLE { name: iterator.clone(), range: range.clone(), map: None }), body: list![body_elem.clone()], source: __frontend_equation_source.clone(), attr: BEquation::Equation::getAttributes(body_elem) });
                    (body_elem, _) = BEquation::Equation::mergeIterators(body_elem, true)?;
                    body_elem = BEquation::Equation::simplify(body_elem, &(literal!("")), &(literal!("")), Pointer::create(metamodelica::nil()), Pointer::create(metamodelica::nil()), (std::sync::Arc::new({ let __pe_b1 = true; let __pe_b2 = literal!(""); let __pe_b3 = literal!(""); move |__pe_a0| SimplifyExp::simplifyDump(__pe_a0, __pe_b1.clone(), &__pe_b2, &__pe_b3) }) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<Expression::NFExpression>) -> Result<metamodelica::Ref<Expression::NFExpression>> + 'static>))?;
                    if isAlgorithm {
                        alg = metamodelica::Ref::new(Algorithm::NFAlgorithm { statements: BEquation::Equation::toStatement(&body_elem)?, inputs: metamodelica::nil(), outputs: metamodelica::nil(), stmtDiffInfo: None, scope: NFInstNode::NO_SCOPE().clone(), source: __frontend_equation_source.clone() });
                        alg = Algorithm::setInputsOutputs(alg)?;
                        size = ({
        let mut __acc: i32 = 0;
        for mut out in (alg.outputs.clone()).into_iter().cloned() {
            let __x = ComponentRef::size(&(out.clone()), false, false)?;
            __acc += __x;
        }
        __acc
    });
                        body_elem = metamodelica::Ref::new(Equation::Equation::ALGORITHM { size: size, alg: alg.clone(), source: alg.source.clone(), expand: openmodelica_frontend_types::DAE::Expand::EXPAND, attr: BEquation::Equation::getAttributes(body_elem) });
                    }
                    Pointer::update(body_elem_ptr.clone(), body_elem);
                    backend_equations = metamodelica::cons(body_elem_ptr.clone(), backend_equations);
                }
            } else {
                if Flags::isSet(Flags::FAILTRACE.clone())? {
                    Error::addMessage(Error::COMPILER_WARNING.clone(), list![{ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBackendDAE.lowerForEquation")); __mm_s.push_str(&*literal!(": Empty for-equation got removed:\n")); __mm_s.push_str(&*FEquation::toString(frontend_equation, literal!(""))?); ArcStr::from(__mm_s) }])?;
                }
            }
            backend_equations
        },
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![{ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBackendDAE.lowerForEquation")); __mm_s.push_str(&*literal!(" failed for\n")); __mm_s.push_str(&*FEquation::toString(frontend_equation, literal!(""))?); ArcStr::from(__mm_s) }])?;
            return Err("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(backend_equations)
}

fn lowerIfEquation(
    mut frontend_equation: &metamodelica::Ref<FEquation::NFEquation>,
    mut init: bool,
    mut in_for: bool,
) -> Result<metamodelica::List<Pointer::Pointer<metamodelica::Ref<Equation::Equation>>>> {
    let mut backend_equations: metamodelica::List<Pointer::Pointer<metamodelica::Ref<Equation::Equation>>>;
    backend_equations = (match &**frontend_equation {
        FEquation::IF { branches, source, .. } => {
            let mut ifEqBody: metamodelica::Ref<IfEquationBody::IfEquationBody>;
            let mut bodies: metamodelica::List<metamodelica::Ref<IfEquationBody::IfEquationBody>>;
            if let Ok(__iflet0) =
                lowerIfEquationBody(branches, init, in_for || FEquation::sizeOf(frontend_equation) == 0)
            {
                ifEqBody = __iflet0;
            } else {
                Error::addMessage(
                    Error::INTERNAL_ERROR.clone(),
                    list![{
                        let mut __mm_s = String::new();
                        __mm_s.push_str(&*literal!("NBackendDAE.lowerIfEquation"));
                        __mm_s.push_str(&*literal!(" failed for:\n"));
                        __mm_s.push_str(&*FEquation::toString(frontend_equation, literal!(""))?);
                        ArcStr::from(__mm_s)
                    }],
                )?;
                return Err("fail");
            }
            if Expression::isEnd(&ifEqBody.condition) {
                backend_equations = ifEqBody.then_eqns.clone();
            } else {
                bodies = BEquation::IfEquationBody::split(ifEqBody)?;
                backend_equations = ({
                    let mut __acc: metamodelica::List<Pointer::Pointer<metamodelica::Ref<Equation::Equation>>> =
                        metamodelica::nil();
                    for mut body in (bodies).into_iter().cloned() {
                        let __x = BEquation::IfEquationBody::toEquation(body.clone(), source.clone(), init)?;
                        __acc = cons(__x, __acc);
                    }
                    __acc.reverse()
                });
            }
            backend_equations
        }
        _ => {
            Error::addMessage(
                Error::INTERNAL_ERROR.clone(),
                list![{
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!("NBackendDAE.lowerIfEquation"));
                    __mm_s.push_str(&*literal!(" failed for\n"));
                    __mm_s.push_str(&*FEquation::toString(frontend_equation, literal!(""))?);
                    ArcStr::from(__mm_s)
                }],
            )?;
            return Err("fail");
        }
    });
    Ok(backend_equations)
}

fn lowerIfEquationBody(
    mut branches: &metamodelica::List<metamodelica::Ref<FEquation::Branch::Branch>>,
    mut init: bool,
    mut allow_imbalance: bool,
) -> Result<metamodelica::Ref<IfEquationBody::IfEquationBody>> {
    let mut ifEq: metamodelica::Ref<IfEquationBody::IfEquationBody>;
    ifEq = (::match_deref::match_deref! { match branches {
        Deref @ metamodelica::ListNode::Cons { head: branch, tail: rest } => {
            let mut eqns: metamodelica::List<Pointer::Pointer<metamodelica::Ref<Equation::Equation>>>;
            let mut condition: metamodelica::Ref<Expression::NFExpression>;
            let mut result: metamodelica::Ref<IfEquationBody::IfEquationBody>;
            (eqns, condition) = lowerIfBranch(metamodelica::AsArg::as_arg(&branch), init)?;
            if Expression::isTrue(&condition) {
                result = metamodelica::Ref::new(IfEquationBody::IfEquationBody { condition: openmodelica_nf_frontend::NFExpression::interned_END(), then_eqns: eqns, else_if: None });
            } else if Expression::isFalse(&condition) {
                result = lowerIfEquationBody(rest, init, allow_imbalance)?;
            } else {
                if (rest).is_empty() && (init || allow_imbalance) {
                    result = metamodelica::Ref::new(IfEquationBody::IfEquationBody { condition: condition, then_eqns: eqns, else_if: None });
                } else {
                    result = metamodelica::Ref::new(IfEquationBody::IfEquationBody { condition: condition, then_eqns: eqns, else_if: Some(lowerIfEquationBody(rest, init, allow_imbalance)?) });
                }
            }
            result
        },
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![{ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBackendDAE.lowerIfEquationBody")); __mm_s.push_str(&*literal!(" failed due to invalid missing else case.")); ArcStr::from(__mm_s) }])?;
            return Err("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(ifEq)
}

fn lowerIfBranch(
    mut branch: &metamodelica::Ref<FEquation::Branch::Branch>,
    mut init: bool,
) -> Result<(
    metamodelica::List<Pointer::Pointer<metamodelica::Ref<Equation::Equation>>>,
    metamodelica::Ref<Expression::NFExpression>,
)> {
    let mut eqns: metamodelica::List<Pointer::Pointer<metamodelica::Ref<Equation::Equation>>>;
    let mut cond: metamodelica::Ref<Expression::NFExpression>;
    (eqns, cond) = (match &**branch {
        FEquation::Branch::BRANCH {
            body: __branch_body,
            condition: __branch_condition,
            ..
        } => {
            if Expression::isFalse(metamodelica::AsArg::as_arg(&__branch_condition)) {
                eqns = metamodelica::nil();
            } else {
                eqns = lowerIfBranchBody(metamodelica::AsArg::as_arg(&__branch_body), init, metamodelica::nil())?;
            }
            (eqns, __branch_condition.clone())
        }
        FEquation::Branch::INVALID_BRANCH { .. } => {
            Error::addMessage(
                Error::INTERNAL_ERROR.clone(),
                list![{
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!("NBackendDAE.lowerIfBranch"));
                    __mm_s.push_str(&*literal!(
                        " failed for invalid branch that should not exist outside of frontend."
                    ));
                    ArcStr::from(__mm_s)
                }],
            )?;
            return Err("fail");
        }
        _ => {
            Error::addMessage(
                Error::INTERNAL_ERROR.clone(),
                list![{
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!("NBackendDAE.lowerIfBranch"));
                    __mm_s.push_str(&*literal!(" failed without proper error message."));
                    ArcStr::from(__mm_s)
                }],
            )?;
            return Err("fail");
        }
    });
    Ok((eqns, cond))
}

fn lowerIfBranchBody<'__b>(
    mut body: &'__b metamodelica::List<metamodelica::Ref<FEquation::NFEquation>>,
    mut init: bool,
    mut eqns: metamodelica::List<Pointer::Pointer<metamodelica::Ref<Equation::Equation>>>,
) -> Result<metamodelica::List<Pointer::Pointer<metamodelica::Ref<Equation::Equation>>>> {
    '__tco: loop {
        ::match_deref::match_deref! { match body {
            Deref @ metamodelica::ListNode::Nil => {
                return Ok(eqns)
            },
            Deref @ metamodelica::ListNode::Cons { head: elem, tail: rest } => {
                { (body, init, eqns) = (rest, init, listAppend(lowerEquation(metamodelica::AsArg::as_arg(&elem), init, false)?, eqns)); continue '__tco; }
            },
            _ => return Err("match: no arm matched"),
        } }
    }
}

fn lowerAssert(
    mut frontend_eq: &metamodelica::Ref<FEquation::NFEquation>,
    mut init: bool,
) -> Result<metamodelica::List<Pointer::Pointer<metamodelica::Ref<Equation::Equation>>>> {
    let mut backend_equations: metamodelica::List<Pointer::Pointer<metamodelica::Ref<Equation::Equation>>>;
    backend_equations = (match &**frontend_eq {
        FEquation::ASSERT {
            condition: __frontend_eq_condition,
            level: __frontend_eq_level,
            message: __frontend_eq_message,
            scope: __frontend_eq_scope,
            source: __frontend_eq_source,
        } => {
            let mut alg: metamodelica::Ref<Algorithm::NFAlgorithm>;
            let mut cond: metamodelica::Ref<Expression::NFExpression>;
            BEquation::default(EquationKind::EMPTY.clone(), init, None, None);
            cond = if (Expression::isCall(metamodelica::AsArg::as_arg(&__frontend_eq_condition))) {
                __frontend_eq_condition.clone()
            } else {
                metamodelica::Ref::new(Expression::NFExpression::CALL {
                    call: Call::makeTypedCall(
                        NFBuiltinFuncs::NO_EVENT().clone(),
                        list![__frontend_eq_condition.clone()],
                        Expression::variability(__frontend_eq_condition.clone())?,
                        Prefixes::Purity::PURE.clone(),
                        NFBuiltinFuncs::NO_EVENT().returnType.clone(),
                    ),
                })
            };
            alg = metamodelica::Ref::new(Algorithm::NFAlgorithm {
                statements: list![metamodelica::Ref::new(Statement::NFStatement::ASSERT {
                    condition: cond,
                    message: __frontend_eq_message.clone(),
                    level: __frontend_eq_level.clone(),
                    source: __frontend_eq_source.clone()
                })],
                inputs: metamodelica::nil(),
                outputs: metamodelica::nil(),
                stmtDiffInfo: None,
                scope: __frontend_eq_scope.clone(),
                source: __frontend_eq_source.clone(),
            });
            list![lowerAlgorithm(alg, init)?]
        }
        _ => {
            Error::addMessage(
                Error::INTERNAL_ERROR.clone(),
                list![{
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!("NBackendDAE.lowerAssert"));
                    __mm_s.push_str(&*literal!(" failed for "));
                    __mm_s.push_str(&*FEquation::toString(frontend_eq, literal!(""))?);
                    ArcStr::from(__mm_s)
                }],
            )?;
            return Err("fail");
        }
    });
    Ok(backend_equations)
}

fn lowerWhenEquation(
    mut frontend_eq: &metamodelica::Ref<FEquation::NFEquation>,
    mut init: bool,
) -> Result<metamodelica::List<Pointer::Pointer<metamodelica::Ref<Equation::Equation>>>> {
    let mut backend_equations: metamodelica::List<Pointer::Pointer<metamodelica::Ref<Equation::Equation>>>;
    backend_equations = (match &**frontend_eq {
        FEquation::WHEN {
            branches: __frontend_eq_branches,
            source: __frontend_eq_source,
            ..
        } => {
            let mut whenEqBody: metamodelica::Ref<BEquation::WhenEquationBody::WhenEquationBody>;
            let mut bodies: metamodelica::List<metamodelica::Ref<BEquation::WhenEquationBody::WhenEquationBody>>;
            let __pa0 = ::match_deref::match_deref! { match &(lowerWhenEquationBody(metamodelica::AsArg::as_arg(&__frontend_eq_branches))?) {
                Some(__pa0) => __pa0.clone(),
                _ => return Err("pattern mismatch"),
            } };
            whenEqBody = metamodelica::Own::own(__pa0);
            bodies = BEquation::WhenEquationBody::split(whenEqBody)?;
            ({
                let mut __acc: metamodelica::List<Pointer::Pointer<metamodelica::Ref<Equation::Equation>>> =
                    metamodelica::nil();
                for mut b in (bodies).into_iter().cloned() {
                    let __x = Pointer::create(metamodelica::Ref::new(Equation::Equation::WHEN_EQUATION {
                        size: BEquation::WhenEquationBody::size(&(b.clone()), false)?,
                        body: b.clone(),
                        source: __frontend_eq_source.clone(),
                        attr: BEquation::default(
                            if (BEquation::WhenEquationBody::size(&(b.clone()), false)? > 0) {
                                EquationKind::DISCRETE.clone()
                            } else {
                                EquationKind::EMPTY.clone()
                            },
                            init,
                            None,
                            None,
                        ),
                    }));
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            })
        }
        _ => {
            Error::addMessage(
                Error::INTERNAL_ERROR.clone(),
                list![{
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!("NBackendDAE.lowerWhenEquation"));
                    __mm_s.push_str(&*literal!(" failed for "));
                    __mm_s.push_str(&*FEquation::toString(frontend_eq, literal!(""))?);
                    ArcStr::from(__mm_s)
                }],
            )?;
            return Err("fail");
        }
    });
    Ok(backend_equations)
}

fn lowerWhenEquationBody(
    mut branches: &metamodelica::List<metamodelica::Ref<FEquation::Branch::Branch>>,
) -> Result<Option<metamodelica::Ref<BEquation::WhenEquationBody::WhenEquationBody>>> {
    let mut whenEq: Option<metamodelica::Ref<BEquation::WhenEquationBody::WhenEquationBody>>;
    whenEq = (::match_deref::match_deref! { match branches {
        Deref @ metamodelica::ListNode::Nil => {
            None
        },
        Deref @ metamodelica::ListNode::Cons { head: branch, tail: rest } => {
            let mut stmts: metamodelica::List<metamodelica::Ref<BEquation::WhenStatement::WhenStatement>>;
            let mut condition: metamodelica::Ref<Expression::NFExpression>;
            (stmts, condition) = lowerWhenBranch(metamodelica::AsArg::as_arg(&branch))?;
            Some(metamodelica::Ref::new(BEquation::WhenEquationBody::WhenEquationBody { condition: condition, when_stmts: stmts, else_when: lowerWhenEquationBody(rest)? }))
        },
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![{ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBackendDAE.lowerWhenEquationBody")); __mm_s.push_str(&*literal!(" failed.")); ArcStr::from(__mm_s) }])?;
            return Err("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(whenEq)
}

fn lowerWhenBranch(
    mut branch: &metamodelica::Ref<FEquation::Branch::Branch>,
) -> Result<(
    metamodelica::List<metamodelica::Ref<BEquation::WhenStatement::WhenStatement>>,
    metamodelica::Ref<Expression::NFExpression>,
)> {
    let mut stmts: metamodelica::List<metamodelica::Ref<BEquation::WhenStatement::WhenStatement>>;
    let mut cond: metamodelica::Ref<Expression::NFExpression>;
    (stmts, cond) = (match &**branch {
        FEquation::Branch::BRANCH { condition, body, .. } => (
            lowerWhenBranchBody(condition, body, metamodelica::nil())?,
            condition.clone(),
        ),
        FEquation::Branch::INVALID_BRANCH { .. } => {
            Error::addMessage(
                Error::INTERNAL_ERROR.clone(),
                list![{
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!("NBackendDAE.lowerWhenBranch"));
                    __mm_s.push_str(&*literal!(
                        " failed for invalid branch that should not exist outside of frontend."
                    ));
                    ArcStr::from(__mm_s)
                }],
            )?;
            return Err("fail");
        }
        _ => {
            Error::addMessage(
                Error::INTERNAL_ERROR.clone(),
                list![{
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!("NBackendDAE.lowerWhenBranch"));
                    __mm_s.push_str(&*literal!(" failed without proper error message."));
                    ArcStr::from(__mm_s)
                }],
            )?;
            return Err("fail");
        }
    });
    Ok((stmts, cond))
}

fn lowerWhenBranchBody<'__b>(
    mut condition: &'__b metamodelica::Ref<Expression::NFExpression>,
    mut body: &'__b metamodelica::List<metamodelica::Ref<FEquation::NFEquation>>,
    mut stmts: metamodelica::List<metamodelica::Ref<BEquation::WhenStatement::WhenStatement>>,
) -> Result<metamodelica::List<metamodelica::Ref<BEquation::WhenStatement::WhenStatement>>> {
    '__tco: loop {
        ::match_deref::match_deref! { match body {
            Deref @ metamodelica::ListNode::Cons { head: elem, tail: rest } => {
                { (condition, body, stmts) = (condition, rest, lowerWhenBranchStatement(metamodelica::AsArg::as_arg(&elem), condition, stmts)?); continue '__tco; }
            },
            _ => {
                return Ok(stmts)
            },
            _ => return Err("match: no arm matched"),
        } }
    }
}

fn lowerWhenBranchStatement(
    mut eq: &metamodelica::Ref<FEquation::NFEquation>,
    mut condition: &metamodelica::Ref<Expression::NFExpression>,
    mut stmts: metamodelica::List<metamodelica::Ref<BEquation::WhenStatement::WhenStatement>>,
) -> Result<metamodelica::List<metamodelica::Ref<BEquation::WhenStatement::WhenStatement>>> {
    let mut stmts: metamodelica::List<metamodelica::Ref<BEquation::WhenStatement::WhenStatement>> = stmts;
    stmts = (::match_deref::match_deref! { match eq {
        Deref @ FEquation::TERMINATE { message: __eq_message, source: __eq_source, .. } => {
            metamodelica::cons(metamodelica::Ref::new(BEquation::WhenStatement::WhenStatement::TERMINATE { message: __eq_message.clone(), source: __eq_source.clone() }), stmts)
        },
        Deref @ FEquation::REINIT { cref: Deref @ Expression::CREF { cref, .. }, reinitExp: __eq_reinitExp, source: __eq_source, .. } => {
            metamodelica::cons(metamodelica::Ref::new(BEquation::WhenStatement::WhenStatement::REINIT { stateVar: cref.clone(), value: __eq_reinitExp.clone(), source: __eq_source.clone() }), stmts)
        },
        Deref @ FEquation::NORETCALL { exp: __eq_exp, source: __eq_source, .. } => {
            metamodelica::cons(metamodelica::Ref::new(BEquation::WhenStatement::WhenStatement::NORETCALL { exp: __eq_exp.clone(), source: __eq_source.clone() }), stmts)
        },
        Deref @ FEquation::ASSERT { condition: __eq_condition, level: __eq_level, message: __eq_message, source: __eq_source, .. } => {
            metamodelica::cons(metamodelica::Ref::new(BEquation::WhenStatement::WhenStatement::ASSERT { condition: __eq_condition.clone(), message: __eq_message.clone(), level: __eq_level.clone(), source: __eq_source.clone() }), stmts)
        },
        Deref @ FEquation::EQUALITY { lhs: __eq_lhs, rhs: __eq_rhs, source: __eq_source, .. } => {
            metamodelica::cons(metamodelica::Ref::new(BEquation::WhenStatement::WhenStatement::ASSIGN { lhs: __eq_lhs.clone(), rhs: __eq_rhs.clone(), source: __eq_source.clone() }), stmts)
        },
        Deref @ FEquation::IF { branches: __eq_branches, source: __eq_source, .. } => {
            let mut cref: metamodelica::Ref<ComponentRef::NFComponentRef>;
            let mut rhs: metamodelica::Ref<Expression::NFExpression>;
            let mut head: metamodelica::Ref<FEquation::Branch::Branch>;
            let mut tail: metamodelica::List<metamodelica::Ref<FEquation::Branch::Branch>>;
            let mut if_map: metamodelica::Ref<UnorderedMap::UnorderedMap<metamodelica::Ref<ComponentRef::NFComponentRef>, metamodelica::Ref<Expression::NFExpression>>>;
            if_map = UnorderedMap::new((std::sync::Arc::new(move |__a0: metamodelica::Ref<ComponentRef::NFComponentRef>| ComponentRef::hash(&__a0)) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<ComponentRef::NFComponentRef>) -> Result<i32> + 'static>), (std::sync::Arc::new(move |__a0: metamodelica::Ref<ComponentRef::NFComponentRef>, __a1: metamodelica::Ref<ComponentRef::NFComponentRef>| ComponentRef::isEqual(&__a0, &__a1)) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<ComponentRef::NFComponentRef>, metamodelica::Ref<ComponentRef::NFComponentRef>) -> Result<bool> + 'static>), 1);
            let (__pa0, __pa1) = ::match_deref::match_deref! { match &(__eq_branches.clone().reverse()) {
                Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
                _ => return Err("pattern mismatch"),
            } };
            head = metamodelica::Own::own(__pa0);
            tail = metamodelica::Own::own(__pa1);
            lowerWhenBranchIf(&head, if_map.clone(), true)?;
            for mut branch in &*tail {
                lowerWhenBranchIf(metamodelica::AsArg::as_arg(&branch), if_map.clone(), false)?;
            }
            for mut tpl in &*UnorderedMap::toList(if_map) {
                (cref, rhs) = tpl.clone();
                stmts = metamodelica::cons(metamodelica::Ref::new(BEquation::WhenStatement::WhenStatement::ASSIGN { lhs: Expression::fromCref(cref, false)?, rhs: rhs, source: __eq_source.clone() }), stmts);
            }
            stmts
        },
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![{ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBackendDAE.lowerWhenBranchStatement")); __mm_s.push_str(&*literal!(" failed for:\n")); __mm_s.push_str(&*FEquation::toString(eq, literal!(""))?); ArcStr::from(__mm_s) }])?;
            return Err("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(stmts)
}

fn lowerWhenBranchIf(
    mut branch: &metamodelica::Ref<FEquation::Branch::Branch>,
    mut if_map: metamodelica::Ref<
        UnorderedMap::UnorderedMap<
            metamodelica::Ref<ComponentRef::NFComponentRef>,
            metamodelica::Ref<Expression::NFExpression>,
        >,
    >,
    mut first: bool,
) -> Result<()> {
    let () = (match &**branch {
        FEquation::Branch::BRANCH {
            body: __branch_body,
            condition: __branch_condition,
            ..
        } => {
            let mut cref: metamodelica::Ref<ComponentRef::NFComponentRef>;
            let mut exp: metamodelica::Ref<Expression::NFExpression>;
            for mut eq in &*__branch_body.clone() {
                let () = (::match_deref::match_deref! { match &(eq.clone()) {
                    Deref @ FEquation::EQUALITY { lhs: Deref @ Expression::CREF { cref: __esc_cref, .. }, rhs: __eq_rhs, .. } => {
                        cref = (*__esc_cref).clone();
                        exp = (::match_deref::match_deref! { match &(UnorderedMap::get(cref.clone(), if_map.clone())?) {
                    Some(__esc_exp) if (!(first)) => {
                        exp = (*__esc_exp).clone();
                        metamodelica::Ref::new(Expression::NFExpression::IF { ty: Expression::typeOf(exp.clone()), condition: __branch_condition.clone(), trueBranch: __eq_rhs.clone(), falseBranch: exp.clone() })
                    },
                    None if (first) => __eq_rhs.clone(),
                    Some(_) => {
                        Error::addMessage(Error::INTERNAL_ERROR.clone(), list![{ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBackendDAE.lowerWhenBranchIf")); __mm_s.push_str(&*literal!(" failed because branch has multiple assignments for the same cref:\n")); __mm_s.push_str(&*FEquation::Branch::toString(branch, &(literal!("")))?); ArcStr::from(__mm_s) }])?;
                        return Err("fail")
                    },
                    _ => {
                        Error::addMessage(Error::INTERNAL_ERROR.clone(), list![{ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBackendDAE.lowerWhenBranchIf")); __mm_s.push_str(&*literal!(" failed because branch equation has an assignment that is missing in other branches:\n")); __mm_s.push_str(&*FEquation::toString(metamodelica::AsArg::as_arg(&eq), literal!(""))?); ArcStr::from(__mm_s) }])?;
                        return Err("fail")
                    },
                    _ => unreachable!("match_deref! exhaustiveness placeholder"),
                } });
                        UnorderedMap::add(cref.clone(), exp.clone(), if_map.clone())?;
                        ()
                    },
                    _ => {
                        Error::addMessage(Error::INTERNAL_ERROR.clone(), list![{ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBackendDAE.lowerWhenBranchIf")); __mm_s.push_str(&*literal!(" failed for branch equation:\n")); __mm_s.push_str(&*FEquation::toString(metamodelica::AsArg::as_arg(&eq), literal!(""))?); ArcStr::from(__mm_s) }])?;
                        return Err("fail")
                    },
                    _ => unreachable!("match_deref! exhaustiveness placeholder"),
                } });
            }
            ()
        }
        _ => {
            Error::addMessage(
                Error::INTERNAL_ERROR.clone(),
                list![{
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!("NBackendDAE.lowerWhenBranchIf"));
                    __mm_s.push_str(&*literal!(" failed for:\n"));
                    __mm_s.push_str(&*FEquation::Branch::toString(branch, &(literal!("")))?);
                    ArcStr::from(__mm_s)
                }],
            )?;
            return Err("fail");
        }
    });
    Ok(())
}

pub(crate) fn lowerAlgorithm(
    mut alg: metamodelica::Ref<Algorithm::NFAlgorithm>,
    mut init: bool,
) -> Result<Pointer::Pointer<metamodelica::Ref<Equation::Equation>>> {
    let mut eq: Pointer::Pointer<metamodelica::Ref<Equation::Equation>>;
    let mut size: i32;
    let mut outputs: metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>>;
    let mut attr: metamodelica::Ref<EquationAttributes::EquationAttributes>;
    size = ({
        let mut __acc: i32 = 0;
        for mut out in (alg.outputs.clone()).into_iter().cloned() {
            let __x = ComponentRef::size(&(out.clone()), false, false)?;
            __acc += __x;
        }
        __acc
    });
    if (alg.outputs).is_empty() {
        attr = BEquation::default(EquationKind::EMPTY.clone(), init, None, None);
    } else if Algorithm::isDiscrete(&alg)? {
        attr = BEquation::default(EquationKind::DISCRETE.clone(), init, None, None);
    } else {
        attr = BEquation::default(EquationKind::CONTINUOUS.clone(), init, None, None);
    }
    eq = Pointer::create(metamodelica::Ref::new(Equation::Equation::ALGORITHM {
        size: size,
        alg: alg.clone(),
        source: alg.source.clone(),
        expand: openmodelica_frontend_types::DAE::Expand::EXPAND,
        attr: attr,
    }));
    Ok(eq)
}

pub(crate) fn lowerEquationAttributes(
    mut ty: metamodelica::Ref<Type::NFType>,
    mut init: bool,
) -> Result<metamodelica::Ref<EquationAttributes::EquationAttributes>> {
    let mut attr: metamodelica::Ref<EquationAttributes::EquationAttributes>;
    if Type::isClock(&ty)? {
        attr = BEquation::default(EquationKind::CLOCKED.clone(), init, Some(-1), None);
    } else if Type::isDiscrete(ty)? {
        attr = BEquation::default(EquationKind::DISCRETE.clone(), init, None, None);
    } else {
        attr = BEquation::default(EquationKind::CONTINUOUS.clone(), init, None, None);
    }
    Ok(attr)
}

fn lowerComponentReferences(
    mut equations: metamodelica::Ref<EquationPointers::EquationPointers>,
    mut variables: &metamodelica::Ref<VariablePointers::VariablePointers>,
) -> Result<metamodelica::Ref<EquationPointers::EquationPointers>> {
    let mut equations: metamodelica::Ref<EquationPointers::EquationPointers> = equations;
    equations = BEquation::EquationPointers::mapExp(
        equations,
        (std::sync::Arc::new({
            let __pe_b1 = variables.clone();
            let __pe_b2 = true;
            move |__pe_a0| lowerComponentReferenceExp(__pe_a0, &__pe_b1, __pe_b2.clone())
        })
            as std::sync::Arc<
                dyn ::std::ops::Fn(
                        metamodelica::Ref<Expression::NFExpression>,
                    ) -> Result<metamodelica::Ref<Expression::NFExpression>>
                    + 'static,
            >),
        Some(
            (std::sync::Arc::new({
                let __pe_b1 = variables.clone();
                let __pe_b2 = true;
                move |__pe_a0| lowerComponentReference(__pe_a0, &__pe_b1, __pe_b2.clone())
            })
                as std::sync::Arc<
                    dyn ::std::ops::Fn(
                            metamodelica::Ref<ComponentRef::NFComponentRef>,
                        )
                            -> Result<metamodelica::Ref<ComponentRef::NFComponentRef>>
                        + 'static,
                >),
        ),
        (std::sync::Arc::new(Expression::map)
            as std::sync::Arc<
                dyn ::std::ops::Fn(
                        metamodelica::Ref<Expression::NFExpression>,
                        Arc<
                            dyn ::std::ops::Fn(
                                    metamodelica::Ref<Expression::NFExpression>,
                                )
                                    -> Result<metamodelica::Ref<Expression::NFExpression>>
                                + 'static,
                        >,
                    ) -> Result<metamodelica::Ref<Expression::NFExpression>>
                    + 'static,
            >),
    )?;
    Ok(equations)
}

pub(crate) fn lowerComponentReferenceExp(
    mut exp: metamodelica::Ref<Expression::NFExpression>,
    mut variables: &metamodelica::Ref<VariablePointers::VariablePointers>,
    mut complete: bool,
) -> Result<metamodelica::Ref<Expression::NFExpression>> {
    let mut exp: metamodelica::Ref<Expression::NFExpression> = exp;
    exp = (::match_deref::match_deref! { match &(exp.clone()) {
        Deref @ Expression::CREF { cref: __exp_cref, ty: __exp_ty } if (!(ComponentRef::isNameNode(metamodelica::AsArg::as_arg(&__exp_cref))?)) => {
            metamodelica::Ref::new(Expression::NFExpression::CREF { ty: __exp_ty.clone(), cref: lowerComponentReference(__exp_cref.clone(), variables, complete)? })
        },
        Deref @ Expression::CALL { call: call @ Deref @ Call::TYPED_ARRAY_CONSTRUCTOR { .. } } => {
            let mut call = (*call).clone();
            assign_variant_field!(call => Call::NFCall::TYPED_ARRAY_CONSTRUCTOR; iters = ({
        let mut __acc: metamodelica::List<(metamodelica::Ref<InstNode::InstNode>, metamodelica::Ref<Expression::NFExpression>)> = metamodelica::nil();
        for mut tpl in (var_field!((*call).iters, Call::NFCall::TYPED_ARRAY_CONSTRUCTOR).clone()).into_iter().cloned() {
            let __x = Util::applyTuple21(tpl.clone(), &({ let __pe_b1 = variables.clone(); let __pe_b2 = complete; move |__pe_a0| lowerInstNode(__pe_a0, &__pe_b1, __pe_b2.clone()) }))?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }));
            assign_variant_field!(exp => Expression::NFExpression::CALL; call = call.clone());
            exp
        },
        Deref @ Expression::CALL { call: call @ Deref @ Call::TYPED_REDUCTION { .. } } => {
            let mut call = (*call).clone();
            assign_variant_field!(call => Call::NFCall::TYPED_REDUCTION; iters = ({
        let mut __acc: metamodelica::List<(metamodelica::Ref<InstNode::InstNode>, metamodelica::Ref<Expression::NFExpression>)> = metamodelica::nil();
        for mut tpl in (var_field!((*call).iters, Call::NFCall::TYPED_REDUCTION).clone()).into_iter().cloned() {
            let __x = Util::applyTuple21(tpl.clone(), &({ let __pe_b1 = variables.clone(); let __pe_b2 = complete; move |__pe_a0| lowerInstNode(__pe_a0, &__pe_b1, __pe_b2.clone()) }))?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }));
            assign_variant_field!(exp => Expression::NFExpression::CALL; call = call.clone());
            exp
        },
        _ => {
            exp
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    exp = Expression::applyToType(
        exp,
        &({
            let __pe_b1: Arc<
                dyn ::std::ops::Fn(
                        metamodelica::Ref<Dimension::NFDimension>,
                    ) -> Result<metamodelica::Ref<Dimension::NFDimension>>
                    + 'static,
            > = (std::sync::Arc::new({
                let __pe_b1 = variables.clone();
                let __pe_b2 = complete;
                move |__pe_a0| lowerDimension(__pe_a0, &__pe_b1, __pe_b2.clone())
            })
                as std::sync::Arc<
                    dyn ::std::ops::Fn(
                            metamodelica::Ref<Dimension::NFDimension>,
                        ) -> Result<metamodelica::Ref<Dimension::NFDimension>>
                        + 'static,
                >);
            move |__pe_a0| Type::applyToDims(__pe_a0, &*__pe_b1)
        }),
    )?;
    Ok(exp)
}

pub(crate) fn lowerComponentReference(
    mut cref: metamodelica::Ref<ComponentRef::NFComponentRef>,
    mut variables: &metamodelica::Ref<VariablePointers::VariablePointers>,
    mut complete: bool,
) -> Result<metamodelica::Ref<ComponentRef::NFComponentRef>> {
    let mut cref: metamodelica::Ref<ComponentRef::NFComponentRef> = cref;
    let mut var: Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>;
    if '__try0: {
        if !(ComponentRef::isWild(&cref)) {
            var = unwrap_break_err!(BVariable::VariablePointers::getVarSafe(variables, ComponentRef::stripSubscriptsAll(&cref), if (complete) {Some(metamodelica::sourceInfo!("NBackEnd/Classes/NBackendDAE.mo"))} else {None}), '__try0);
            cref = unwrap_break_err!(lowerComponentReferenceInstNode(cref.clone(), var.clone()), '__try0);
            cref = unwrap_break_err!(ComponentRef::mapSubscripts(cref.clone(), &({ let __pe_b1: Arc<dyn ::std::ops::Fn(metamodelica::Ref<Expression::NFExpression>) -> Result<metamodelica::Ref<Expression::NFExpression>> + 'static> = (std::sync::Arc::new({ let __pe_b1 = variables.clone(); let __pe_b2 = complete; move |__pe_a0| lowerComponentReferenceExp(__pe_a0, &__pe_b1, __pe_b2.clone()) }) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<Expression::NFExpression>) -> Result<metamodelica::Ref<Expression::NFExpression>> + 'static>); move |__pe_a0| Subscript::mapExp(__pe_a0, __pe_b1.clone()) }), false), '__try0);
        }
        Ok::<(), &'static str>(())
    }.is_err() {
        if Flags::isSet(Flags::FAILTRACE.clone())? && complete {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![{ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBackendDAE.lowerComponentReference")); __mm_s.push_str(&*literal!(" failed for ")); __mm_s.push_str(&*ComponentRef::toString(&cref)?); ArcStr::from(__mm_s) }])?;
        }
    }
    Ok(cref)
}

fn lowerDimension(
    mut dim: metamodelica::Ref<Dimension::NFDimension>,
    mut variables: &metamodelica::Ref<VariablePointers::VariablePointers>,
    mut complete: bool,
) -> Result<metamodelica::Ref<Dimension::NFDimension>> {
    let mut dim: metamodelica::Ref<Dimension::NFDimension> = dim;
    dim = (match &*dim {
        Dimension::RESIZABLE { exp: __dim_exp, .. } => {
            assign_variant_field!(dim => Dimension::NFDimension::RESIZABLE; exp = Expression::map(__dim_exp.clone(), (std::sync::Arc::new({ let __pe_b1 = variables.clone(); let __pe_b2 = complete; move |__pe_a0| lowerComponentReferenceExp(__pe_a0, &__pe_b1, __pe_b2.clone()) }) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<Expression::NFExpression>) -> Result<metamodelica::Ref<Expression::NFExpression>> + 'static>))?);
            dim
        }
        _ => dim,
    });
    Ok(dim)
}

fn collectIterators(
    mut exp: metamodelica::Ref<Expression::NFExpression>,
    mut variables: &metamodelica::Ref<VariablePointers::VariablePointers>,
    mut set: metamodelica::Ref<UnorderedSet::UnorderedSet<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>>>,
) -> Result<metamodelica::Ref<Expression::NFExpression>> {
    let mut exp: metamodelica::Ref<Expression::NFExpression> = exp;
    match '__try0: {
        let () = (::match_deref::match_deref! { match &(&*exp) {
            Deref @ Expression::CREF { cref: __exp_cref, .. } if (!(unwrap_break_err!(BVariable::VariablePointers::containsCref(ComponentRef::stripSubscriptsAll(metamodelica::AsArg::as_arg(&__exp_cref)), variables), '__try0) || unwrap_break_err!(ComponentRef::isNameNode(metamodelica::AsArg::as_arg(&__exp_cref)), '__try0) || ComponentRef::isWild(metamodelica::AsArg::as_arg(&__exp_cref)))) => {
                unwrap_break_err!(UnorderedSet::add(unwrap_break_err!(lowerIterator(__exp_cref.clone()), '__try0), set.clone()), '__try0);
                ()
            },
            Deref @ Expression::CALL { call: call @ Deref @ Call::TYPED_ARRAY_CONSTRUCTOR { .. } } => {
                for mut tpl in &*var_field!((**call).iters, Call::NFCall::TYPED_ARRAY_CONSTRUCTOR).clone() {
                    unwrap_break_err!(collectIterator(Util::tuple21(tpl.clone()), variables, set.clone()), '__try0);
                }
                ()
            },
            Deref @ Expression::CALL { call: call @ Deref @ Call::TYPED_REDUCTION { .. } } => {
                for mut tpl in &*var_field!((**call).iters, Call::NFCall::TYPED_REDUCTION).clone() {
                    unwrap_break_err!(collectIterator(Util::tuple21(tpl.clone()), variables, set.clone()), '__try0);
                }
                ()
            },
            _ => {
                ()
            },
            _ => unreachable!("match_deref! exhaustiveness placeholder"),
        } });
        Ok::<(), &'static str>(())
    } {
        Ok(()) => {}
        Err(__try0_err) => {
            Error::addMessage(
                Error::INTERNAL_ERROR.clone(),
                list![{
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!("NBackendDAE.collectIterators"));
                    __mm_s.push_str(&*literal!(" failed for "));
                    __mm_s.push_str(&*Expression::toString(exp.clone())?);
                    ArcStr::from(__mm_s)
                }],
            )?;
            return Err(__try0_err);
        }
    }
    Ok(exp)
}

fn collectIterator(
    mut iterator: metamodelica::Ref<InstNode::InstNode>,
    mut variables: &metamodelica::Ref<VariablePointers::VariablePointers>,
    mut set: metamodelica::Ref<UnorderedSet::UnorderedSet<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>>>,
) -> Result<()> {
    let mut cref: metamodelica::Ref<ComponentRef::NFComponentRef>;
    cref = ComponentRef::fromNode(
        iterator.clone(),
        NFInstNode::InstNode::getType(iterator)?,
        metamodelica::nil(),
        ComponentRef::Origin::ITERATOR.clone(),
    )?;
    cref = ComponentRef::stripSubscriptsAll(&cref);
    if !(BVariable::VariablePointers::containsCref(cref.clone(), variables)?) {
        UnorderedSet::add(lowerIterator(cref)?, set)?;
    }
    Ok(())
}

fn lowerInstNode(
    mut node: metamodelica::Ref<InstNode::InstNode>,
    mut variables: &metamodelica::Ref<VariablePointers::VariablePointers>,
    mut complete: bool,
) -> Result<metamodelica::Ref<InstNode::InstNode>> {
    let mut node: metamodelica::Ref<InstNode::InstNode> = node;
    let mut cref: metamodelica::Ref<ComponentRef::NFComponentRef> = ComponentRef::fromNode(
        node.clone(),
        openmodelica_nf_frontend::NFType::interned_INTEGER(),
        metamodelica::nil(),
        ComponentRef::Origin::ITERATOR.clone(),
    )?;
    let mut var: Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>;
    if '__try0: {
        var = unwrap_break_err!(BVariable::VariablePointers::getVarSafe(variables, ComponentRef::stripSubscriptsAll(&cref), if (complete) {Some(metamodelica::sourceInfo!("NBackEnd/Classes/NBackendDAE.mo"))} else {None}), '__try0);
        node = metamodelica::Ref::new(InstNode::InstNode::VAR_NODE { name: unwrap_break_err!(NFInstNode::InstNode::name(&node), '__try0), varPointer: PointerWeak::downgrade(var.clone()) });
        Ok::<(), &'static str>(())
    }.is_err() {
    }
    Ok(node)
}

pub(crate) fn lowerComponentReferenceInstNode(
    mut cref: metamodelica::Ref<ComponentRef::NFComponentRef>,
    mut var: Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>,
) -> Result<metamodelica::Ref<ComponentRef::NFComponentRef>> {
    let mut cref: metamodelica::Ref<ComponentRef::NFComponentRef> = cref;
    cref = (::match_deref::match_deref! { match &(cref.clone()) {
        qual @ Deref @ ComponentRef::CREF { .. } => {
            let mut qual = (*qual).clone();
            assign_variant_field!(qual => ComponentRef::NFComponentRef::CREF; node = ComponentRef::storeNode(metamodelica::Ref::new(InstNode::InstNode::VAR_NODE { name: NFInstNode::InstNode::name(&(ComponentRef::node(metamodelica::AsArg::as_arg(&qual))?))?, varPointer: PointerWeak::downgrade(var) }), false)?);
            qual.clone()
        },
        _ => {
            cref
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(cref)
}

pub(crate) fn lowerEquationIterators(
    mut eqn: metamodelica::Ref<Equation::Equation>,
    mut variables: &metamodelica::Ref<VariablePointers::VariablePointers>,
    mut set: metamodelica::Ref<UnorderedSet::UnorderedSet<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>>>,
) -> Result<metamodelica::Ref<Equation::Equation>> {
    let mut eqn: metamodelica::Ref<Equation::Equation> = eqn;
    let mut iter: metamodelica::Ref<Iterator::Iterator> = BEquation::Equation::getForIterator(&eqn);
    let mut iterators: metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>>;
    (iterators, _, _) = BEquation::Iterator::getFrames(&iter);
    for mut iter in &*iterators {
        let mut iter = iter.clone();
        UnorderedSet::add(lowerIterator(iter)?, set.clone())?;
    }
    BEquation::Equation::map(
        eqn.clone(),
        (std::sync::Arc::new({
            let __pe_b1 = variables.clone();
            let __pe_b2 = set;
            move |__pe_a0| collectIterators(__pe_a0, &__pe_b1, __pe_b2.clone())
        })
            as std::sync::Arc<
                dyn ::std::ops::Fn(
                        metamodelica::Ref<Expression::NFExpression>,
                    ) -> Result<metamodelica::Ref<Expression::NFExpression>>
                    + 'static,
            >),
        None,
        (std::sync::Arc::new(Expression::map)
            as std::sync::Arc<
                dyn ::std::ops::Fn(
                        metamodelica::Ref<Expression::NFExpression>,
                        Arc<
                            dyn ::std::ops::Fn(
                                    metamodelica::Ref<Expression::NFExpression>,
                                )
                                    -> Result<metamodelica::Ref<Expression::NFExpression>>
                                + 'static,
                        >,
                    ) -> Result<metamodelica::Ref<Expression::NFExpression>>
                    + 'static,
            >),
    )?;
    Ok(eqn)
}

pub(crate) fn lowerIterator(
    mut iterator: metamodelica::Ref<ComponentRef::NFComponentRef>,
) -> Result<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>> {
    let mut var_ptr: Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>> =
        lowerVariable(Variable::fromCref(iterator.clone())?)?;
    Ok(var_ptr)
}

pub(crate) fn lowerIteratorCref(
    mut iterator: metamodelica::Ref<ComponentRef::NFComponentRef>,
) -> Result<metamodelica::Ref<ComponentRef::NFComponentRef>> {
    let mut iterator: metamodelica::Ref<ComponentRef::NFComponentRef> = iterator;
    iterator = BVariable::getVarName(lowerIterator(iterator)?);
    Ok(iterator)
}

pub(crate) fn lowerIteratorExp(
    mut exp: metamodelica::Ref<Expression::NFExpression>,
) -> Result<metamodelica::Ref<Expression::NFExpression>> {
    let mut exp: metamodelica::Ref<Expression::NFExpression> = exp;
    exp = (match &*exp {
        Expression::CREF { cref: __exp_cref, .. } => {
            assign_variant_field!(exp => Expression::NFExpression::CREF; cref = lowerIteratorCref(__exp_cref.clone())?);
            exp
        }
        _ => exp,
    });
    Ok(exp)
}

pub(crate) fn lowerFunctions(
    mut funcMap: metamodelica::Ref<
        UnorderedMap::UnorderedMap<metamodelica::Ref<Path>, metamodelica::Ref<Function::Function>>,
    >,
) -> Result<metamodelica::Ref<UnorderedMap::UnorderedMap<metamodelica::Ref<Path>, metamodelica::Ref<Function::Function>>>>
{
    let mut funcMap: metamodelica::Ref<
        UnorderedMap::UnorderedMap<metamodelica::Ref<Path>, metamodelica::Ref<Function::Function>>,
    > = funcMap;
    UnorderedMap::apply(
        funcMap.clone(),
        (std::sync::Arc::new({
            let __pe_b1 = funcMap.clone();
            move |__pe_a0| Differentiate::resolvePartialDerivatives(__pe_a0, __pe_b1.clone())
        })
            as std::sync::Arc<
                dyn ::std::ops::Fn(
                        metamodelica::Ref<Function::Function>,
                    ) -> Result<metamodelica::Ref<Function::Function>>
                    + 'static,
            >),
    )?;
    Ok(funcMap)
}

pub(crate) fn backenddaeinfo(mut bdae: &metamodelica::Ref<NBackendDAE>) -> Result<()> {
    if Flags::isSet(Flags::DUMP_BACKENDDAE_INFO.clone())? {
        let () = (::match_deref::match_deref! { match bdae {
            Deref @ MAIN { varData: varData @ Deref @ BVariable::VarData::VAR_DATA_SIM { .. }, eqData: Deref @ BEquation::EqData::EQ_DATA_SIM { .. }, alg_event: __bdae_alg_event, algebraic: __bdae_algebraic, init: __bdae_init, init_0: __bdae_init_0, ode: __bdae_ode, ode_event: __bdae_ode_event, .. } => {
                let mut p_ode: ArcStr;
                let mut p_alg: ArcStr;
                let mut p_ode_e: ArcStr;
                let mut p_alg_e: ArcStr;
                let mut p_clk: ArcStr;
                let mut p_ini: ArcStr;
                let mut p_ini_0: ArcStr;
                let mut states: ArcStr;
                let mut discretes: ArcStr;
                let mut discrete_states: ArcStr;
                let mut clocked_states: ArcStr;
                let mut clocks: ArcStr;
                let mut inputs: ArcStr;
                p_ode = intString(((__bdae_ode).len() as i32));
                p_alg = intString(((__bdae_algebraic).len() as i32));
                p_ode_e = intString(((__bdae_ode_event).len() as i32));
                p_alg_e = intString(((__bdae_alg_event).len() as i32));
                p_clk = literal!("0");
                p_ini = intString(((__bdae_init).len() as i32));
                p_ini_0 = if ((__bdae_init_0).is_some()) {intString((((Util::getOption(__bdae_init_0.clone())?)).len() as i32))} else {literal!("0")};
                states = { let mut __mm_s = String::new(); __mm_s.push_str(&*intString(BVariable::VariablePointers::scalarSize(var_field!((**varData).states, VarData::VarData::VAR_DATA_SIM), false)?)); __mm_s.push_str(&*literal!(" (")); __mm_s.push_str(&*intString(BVariable::VariablePointers::size(var_field!((**varData).states, VarData::VarData::VAR_DATA_SIM)))); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) };
                discretes = { let mut __mm_s = String::new(); __mm_s.push_str(&*intString(BVariable::VariablePointers::scalarSize(var_field!((**varData).discretes, VarData::VarData::VAR_DATA_SIM), false)?)); __mm_s.push_str(&*literal!(" (")); __mm_s.push_str(&*intString(BVariable::VariablePointers::size(var_field!((**varData).discretes, VarData::VarData::VAR_DATA_SIM)))); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) };
                discrete_states = { let mut __mm_s = String::new(); __mm_s.push_str(&*intString(BVariable::VariablePointers::scalarSize(var_field!((**varData).discrete_states, VarData::VarData::VAR_DATA_SIM), false)?)); __mm_s.push_str(&*literal!(" (")); __mm_s.push_str(&*intString(BVariable::VariablePointers::size(var_field!((**varData).discrete_states, VarData::VarData::VAR_DATA_SIM)))); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) };
                clocked_states = { let mut __mm_s = String::new(); __mm_s.push_str(&*intString(BVariable::VariablePointers::scalarSize(var_field!((**varData).clocked_states, VarData::VarData::VAR_DATA_SIM), false)?)); __mm_s.push_str(&*literal!(" (")); __mm_s.push_str(&*intString(BVariable::VariablePointers::size(var_field!((**varData).clocked_states, VarData::VarData::VAR_DATA_SIM)))); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) };
                clocks = { let mut __mm_s = String::new(); __mm_s.push_str(&*intString(BVariable::VariablePointers::scalarSize(var_field!((**varData).clocks, VarData::VarData::VAR_DATA_SIM), false)?)); __mm_s.push_str(&*literal!(" (")); __mm_s.push_str(&*intString(BVariable::VariablePointers::size(var_field!((**varData).clocks, VarData::VarData::VAR_DATA_SIM)))); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) };
                inputs = { let mut __mm_s = String::new(); __mm_s.push_str(&*intString(BVariable::VariablePointers::scalarSize(var_field!((**varData).top_level_inputs, VarData::VarData::VAR_DATA_SIM), false)?)); __mm_s.push_str(&*literal!(" (")); __mm_s.push_str(&*intString(BVariable::VariablePointers::size(var_field!((**varData).top_level_inputs, VarData::VarData::VAR_DATA_SIM)))); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) };
                if Flags::isSet(Flags::DUMP_STATESELECTION_INFO.clone())? {
                    states = { let mut __mm_s = String::new(); __mm_s.push_str(&*states); __mm_s.push_str(&*literal!(" ")); __mm_s.push_str(&*List::toString(BVariable::VariablePointers::toList(var_field!((**varData).states, VarData::VarData::VAR_DATA_SIM))?, &BVariable::nameString, List::Style::FLAT_CURLY.clone())?); ArcStr::from(__mm_s) };
                } else {
                    states = { let mut __mm_s = String::new(); __mm_s.push_str(&*states); __mm_s.push_str(&*literal!(" ('-d=stateselection' for the list of states)")); ArcStr::from(__mm_s) };
                }
                if Flags::isSet(Flags::DUMP_DISCRETEVARS_INFO.clone())? {
                    discretes = { let mut __mm_s = String::new(); __mm_s.push_str(&*discretes); __mm_s.push_str(&*literal!(" ")); __mm_s.push_str(&*List::toString(BVariable::VariablePointers::toList(var_field!((**varData).discretes, VarData::VarData::VAR_DATA_SIM))?, &BVariable::nameString, List::Style::FLAT_CURLY.clone())?); ArcStr::from(__mm_s) };
                    clocks = { let mut __mm_s = String::new(); __mm_s.push_str(&*clocks); __mm_s.push_str(&*literal!(" ")); __mm_s.push_str(&*List::toString(BVariable::VariablePointers::toList(var_field!((**varData).clocks, VarData::VarData::VAR_DATA_SIM))?, &BVariable::nameString, List::Style::FLAT_CURLY.clone())?); ArcStr::from(__mm_s) };
                    inputs = { let mut __mm_s = String::new(); __mm_s.push_str(&*inputs); __mm_s.push_str(&*literal!(" ")); __mm_s.push_str(&*List::toString(BVariable::VariablePointers::toList(var_field!((**varData).top_level_inputs, VarData::VarData::VAR_DATA_SIM))?, &BVariable::nameString, List::Style::FLAT_CURLY.clone())?); ArcStr::from(__mm_s) };
                } else {
                    discretes = { let mut __mm_s = String::new(); __mm_s.push_str(&*discretes); __mm_s.push_str(&*literal!(" ('-d=discreteinfo' for the list of discrete variables)")); ArcStr::from(__mm_s) };
                    clocks = { let mut __mm_s = String::new(); __mm_s.push_str(&*clocks); __mm_s.push_str(&*literal!(" ('-d=discreteinfo' for the list of clocks variables)")); ArcStr::from(__mm_s) };
                    inputs = { let mut __mm_s = String::new(); __mm_s.push_str(&*inputs); __mm_s.push_str(&*literal!(" ('-d=discreteinfo' for the list of top level inputs)")); ArcStr::from(__mm_s) };
                }
                if Flags::isSet(Flags::DUMP_STATESELECTION_INFO.clone())? || Flags::isSet(Flags::DUMP_DISCRETEVARS_INFO.clone())? {
                    discrete_states = { let mut __mm_s = String::new(); __mm_s.push_str(&*discrete_states); __mm_s.push_str(&*literal!(" ")); __mm_s.push_str(&*List::toString(BVariable::VariablePointers::toList(var_field!((**varData).discrete_states, VarData::VarData::VAR_DATA_SIM))?, &BVariable::nameString, List::Style::FLAT_CURLY.clone())?); ArcStr::from(__mm_s) };
                    clocked_states = { let mut __mm_s = String::new(); __mm_s.push_str(&*clocked_states); __mm_s.push_str(&*literal!(" ")); __mm_s.push_str(&*List::toString(BVariable::VariablePointers::toList(var_field!((**varData).clocked_states, VarData::VarData::VAR_DATA_SIM))?, &BVariable::nameString, List::Style::FLAT_CURLY.clone())?); ArcStr::from(__mm_s) };
                } else {
                    discrete_states = { let mut __mm_s = String::new(); __mm_s.push_str(&*discrete_states); __mm_s.push_str(&*literal!(" ('-d=discreteinfo' or '-d=stateselection' for the list of discrete states)")); ArcStr::from(__mm_s) };
                    clocked_states = { let mut __mm_s = String::new(); __mm_s.push_str(&*clocked_states); __mm_s.push_str(&*literal!(" ('-d=discreteinfo' or '-d=stateselection' for the list of clocked states)")); ArcStr::from(__mm_s) };
                }
                Error::addCompilerNotification({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Partition statistics after passing the back-end:\n")); __mm_s.push_str(&*literal!(" * Number of ODE partitions: ..................... ")); __mm_s.push_str(&*p_ode); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*literal!(" * Number of algebraic partitions: ............... ")); __mm_s.push_str(&*p_alg); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*literal!(" * Number of ODE event partitions: ............... ")); __mm_s.push_str(&*p_ode_e); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*literal!(" * Number of algebraic event partitions: ......... ")); __mm_s.push_str(&*p_alg_e); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*literal!(" * Number of clocked partitions: ................. ")); __mm_s.push_str(&*p_clk); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*literal!(" * Number of initial partitions: ................. ")); __mm_s.push_str(&*p_ini); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*literal!(" * Number of initial(lambda=0) partitions: ....... ")); __mm_s.push_str(&*p_ini_0); ArcStr::from(__mm_s) })?;
                Error::addCompilerNotification({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Variable statistics after passing the back-end:\n")); __mm_s.push_str(&*literal!(" * Number of states: ............................. ")); __mm_s.push_str(&*states); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*literal!(" * Number of discrete states: .................... ")); __mm_s.push_str(&*discrete_states); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*literal!(" * Number of clocked states: ..................... ")); __mm_s.push_str(&*clocked_states); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*literal!(" * Number of discrete variables: ................. ")); __mm_s.push_str(&*discretes); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*literal!(" * Number of clocks: ............................. ")); __mm_s.push_str(&*clocks); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*literal!(" * Number of top-level inputs: ................... ")); __mm_s.push_str(&*inputs); ArcStr::from(__mm_s) })?;
                strongcomponentinfo(&(literal!("Simulation")), &(list![__bdae_ode.clone(), __bdae_algebraic.clone(), __bdae_ode_event.clone(), __bdae_alg_event.clone()]))?;
                strongcomponentinfo(&(literal!("Initialization")), &(list![__bdae_init.clone()]))?;
                if (__bdae_init_0).is_some() {
                    strongcomponentinfo(&(literal!("Initialization (lambda=0)")), &(list![Util::getOption(__bdae_init_0.clone())?]))?;
                }
                ()
            },
            _ => return Err("match: no arm matched"),
        } });
    }
    Ok(())
}

pub(crate) fn strongcomponentinfo(
    mut phase: &ArcStr,
    mut systems: &metamodelica::List<metamodelica::List<metamodelica::Ref<Partition::Partition>>>,
) -> Result<()> {
    let mut c: CountCollector = CountCollector {
        single_scalar: 0,
        single_array: 0,
        single_record: 0,
        multi_algorithm: 0,
        multi_when: 0,
        multi_if: 0,
        multi_tpl: 0,
        resizable_for: 0,
        generic_for: 0,
        entwined_for: 0,
        loop_lin: 0,
        loop_nlin: 0,
    };
    let mut collector_ptr: Pointer::Pointer<CountCollector> = Pointer::create(c);
    let mut single_sc: ArcStr;
    let mut multi_sc: ArcStr;
    let mut for_sc: ArcStr;
    let mut alg_sc: ArcStr;
    for mut lst in &**systems {
        for mut system in &*lst.clone() {
            NBPartition::Partition::mapStrongComponents(
                system.clone(),
                &({
                    let __pe_b1 = collector_ptr.clone();
                    move |__pe_a0| StrongComponent::strongComponentInfo(__pe_a0, __pe_b1.clone())
                }),
            )?;
        }
    }
    c = Pointer::access(collector_ptr);
    single_sc = {
        let mut __mm_s = String::new();
        __mm_s.push_str(&*intString(
            c.single_scalar.clone() + c.single_array.clone() + c.single_record.clone(),
        ));
        __mm_s.push_str(&*literal!(" (scalar:"));
        __mm_s.push_str(&*intString(c.single_scalar.clone()));
        __mm_s.push_str(&*literal!(", array:"));
        __mm_s.push_str(&*intString(c.single_array.clone()));
        __mm_s.push_str(&*literal!(", record:"));
        __mm_s.push_str(&*intString(c.single_record.clone()));
        __mm_s.push_str(&*literal!(")"));
        ArcStr::from(__mm_s)
    };
    multi_sc = {
        let mut __mm_s = String::new();
        __mm_s.push_str(&*intString(
            c.multi_algorithm.clone() + c.multi_when.clone() + c.multi_if.clone(),
        ));
        __mm_s.push_str(&*literal!(" (algorithm:"));
        __mm_s.push_str(&*intString(c.multi_algorithm.clone()));
        __mm_s.push_str(&*literal!(", when:"));
        __mm_s.push_str(&*intString(c.multi_when.clone()));
        __mm_s.push_str(&*literal!(", if:"));
        __mm_s.push_str(&*intString(c.multi_if.clone()));
        __mm_s.push_str(&*literal!(", tuple:"));
        __mm_s.push_str(&*intString(c.multi_tpl.clone()));
        __mm_s.push_str(&*literal!(")"));
        ArcStr::from(__mm_s)
    };
    for_sc = {
        let mut __mm_s = String::new();
        __mm_s.push_str(&*intString(
            c.resizable_for.clone() + c.generic_for.clone() + c.entwined_for.clone(),
        ));
        __mm_s.push_str(&*literal!(" (resizable: "));
        __mm_s.push_str(&*intString(c.resizable_for.clone()));
        __mm_s.push_str(&*literal!(", generic: "));
        __mm_s.push_str(&*intString(c.generic_for.clone()));
        __mm_s.push_str(&*literal!(", entwined:"));
        __mm_s.push_str(&*intString(c.entwined_for.clone()));
        __mm_s.push_str(&*literal!(")"));
        ArcStr::from(__mm_s)
    };
    alg_sc = {
        let mut __mm_s = String::new();
        __mm_s.push_str(&*intString(c.loop_lin.clone() + c.loop_nlin.clone()));
        __mm_s.push_str(&*literal!(" (linear: "));
        __mm_s.push_str(&*intString(c.loop_lin.clone()));
        __mm_s.push_str(&*literal!(", nonlinear:"));
        __mm_s.push_str(&*intString(c.loop_nlin.clone()));
        __mm_s.push_str(&*literal!(")"));
        ArcStr::from(__mm_s)
    };
    Error::addCompilerNotification({
        let mut __mm_s = String::new();
        __mm_s.push_str(&*literal!("["));
        __mm_s.push_str(&*phase);
        __mm_s.push_str(&*literal!(
            "] Strong Component statistics after passing the back-end:\n"
        ));
        __mm_s.push_str(&*literal!(" * Number of single strong components: ........... "));
        __mm_s.push_str(&*single_sc);
        __mm_s.push_str(&*literal!("\n"));
        __mm_s.push_str(&*literal!(" * Number of multi strong components: ............ "));
        __mm_s.push_str(&*multi_sc);
        __mm_s.push_str(&*literal!("\n"));
        __mm_s.push_str(&*literal!(" * Number of for-loop strong components: ......... "));
        __mm_s.push_str(&*for_sc);
        __mm_s.push_str(&*literal!("\n"));
        __mm_s.push_str(&*literal!(" * Number of algebraic-loop strong components: ... "));
        __mm_s.push_str(&*alg_sc);
        ArcStr::from(__mm_s)
    })?;
    Ok(())
}

pub(crate) fn debugFollowEquations(
    mut bdae: &metamodelica::Ref<NBackendDAE>,
    mut eq_filter_opt: Option<metamodelica::Ref<UnorderedSet::UnorderedSet<ArcStr>>>,
    mut r#str: &ArcStr,
) -> Result<()> {
    let () = ({
        let mut tmp: ArcStr = literal!("");
        (match &**bdae {
            MAIN {
                eqData: __bdae_eqData, ..
            } => {
                tmp = {
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*StringUtil::headline_1(
                        &({
                            let mut __mm_s = String::new();
                            __mm_s.push_str(&*literal!("[debugFollowEquations]: "));
                            __mm_s.push_str(&*r#str);
                            ArcStr::from(__mm_s)
                        }),
                    )?);
                    __mm_s.push_str(&*literal!("\n"));
                    ArcStr::from(__mm_s)
                };
                tmp = {
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*tmp);
                    __mm_s.push_str(&*BEquation::EqData::toString(
                        metamodelica::AsArg::as_arg(&__bdae_eqData),
                        1,
                        eq_filter_opt,
                    )?);
                    ArcStr::from(__mm_s)
                };
                metamodelica::print(tmp);
                ()
            }
            _ => (),
        })
    });
    Ok(())
}

pub(crate) fn debugLowering(mut bdae: &metamodelica::Ref<NBackendDAE>) -> Result<()> {
    let () = (match &**bdae {
        MAIN {
            eqData: __bdae_eqData,
            varData: __bdae_varData,
            ..
        } => {
            BEquation::EqData::map(__bdae_eqData.clone(), &checkLoweredCrefEqn)?;
            BVariable::VariablePointers::mapPtr(
                BVariable::VarData::getVariables(metamodelica::AsArg::as_arg(&__bdae_varData))?,
                &checkLoweredCrefVar,
            )?;
            ()
        }
        _ => (),
    });
    Ok(())
}

pub(crate) fn checkLoweredCrefVar(mut var: Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>) -> Result<()> {
    let mut set: metamodelica::Ref<UnorderedSet::UnorderedSet<metamodelica::Ref<ComponentRef::NFComponentRef>>> =
        UnorderedSet::new(
            (std::sync::Arc::new(move |__a0: metamodelica::Ref<ComponentRef::NFComponentRef>| ComponentRef::hash(&__a0))
                as std::sync::Arc<
                    dyn ::std::ops::Fn(metamodelica::Ref<ComponentRef::NFComponentRef>) -> Result<i32> + 'static,
                >),
            (std::sync::Arc::new(
                move |__a0: metamodelica::Ref<ComponentRef::NFComponentRef>,
                      __a1: metamodelica::Ref<ComponentRef::NFComponentRef>| {
                    ComponentRef::isEqual(&__a0, &__a1)
                },
            )
                as std::sync::Arc<
                    dyn ::std::ops::Fn(
                            metamodelica::Ref<ComponentRef::NFComponentRef>,
                            metamodelica::Ref<ComponentRef::NFComponentRef>,
                        ) -> Result<bool>
                        + 'static,
                >),
            13,
        );
    BVariable::mapExp(
        var.clone(),
        (std::sync::Arc::new({
            let __pe_b1 = set.clone();
            move |__pe_a0| checkLoweredCrefExp(__pe_a0, __pe_b1.clone())
        })
            as std::sync::Arc<
                dyn ::std::ops::Fn(
                        metamodelica::Ref<Expression::NFExpression>,
                    ) -> Result<metamodelica::Ref<Expression::NFExpression>>
                    + 'static,
            >),
        &Expression::map,
    )?;
    if !(UnorderedSet::isEmpty(set.clone())) {
        metamodelica::print({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("[failtrace] the variable:\n"));
            __mm_s.push_str(&*BVariable::pointerToString(var)?);
            __mm_s.push_str(&*literal!("\n"));
            ArcStr::from(__mm_s)
        });
        metamodelica::print({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!(
                "[failtrace] has following non-lowered component references: "
            ));
            __mm_s.push_str(&*List::toString(
                UnorderedSet::toList(set),
                &move |__a0: metamodelica::Ref<ComponentRef::NFComponentRef>| ComponentRef::toString(&__a0),
                List::Style::FLAT_CURLY.clone(),
            )?);
            __mm_s.push_str(&*literal!("\n"));
            ArcStr::from(__mm_s)
        });
    }
    Ok(())
}

pub(crate) fn checkLoweredCrefEqn(
    mut eqn: metamodelica::Ref<Equation::Equation>,
) -> Result<metamodelica::Ref<Equation::Equation>> {
    let mut eqn: metamodelica::Ref<Equation::Equation> = eqn;
    let mut set: metamodelica::Ref<UnorderedSet::UnorderedSet<metamodelica::Ref<ComponentRef::NFComponentRef>>> =
        UnorderedSet::new(
            (std::sync::Arc::new(move |__a0: metamodelica::Ref<ComponentRef::NFComponentRef>| ComponentRef::hash(&__a0))
                as std::sync::Arc<
                    dyn ::std::ops::Fn(metamodelica::Ref<ComponentRef::NFComponentRef>) -> Result<i32> + 'static,
                >),
            (std::sync::Arc::new(
                move |__a0: metamodelica::Ref<ComponentRef::NFComponentRef>,
                      __a1: metamodelica::Ref<ComponentRef::NFComponentRef>| {
                    ComponentRef::isEqual(&__a0, &__a1)
                },
            )
                as std::sync::Arc<
                    dyn ::std::ops::Fn(
                            metamodelica::Ref<ComponentRef::NFComponentRef>,
                            metamodelica::Ref<ComponentRef::NFComponentRef>,
                        ) -> Result<bool>
                        + 'static,
                >),
            13,
        );
    BEquation::Equation::map(
        eqn.clone(),
        (std::sync::Arc::new({
            let __pe_b1 = set.clone();
            move |__pe_a0| checkLoweredCrefExp(__pe_a0, __pe_b1.clone())
        })
            as std::sync::Arc<
                dyn ::std::ops::Fn(
                        metamodelica::Ref<Expression::NFExpression>,
                    ) -> Result<metamodelica::Ref<Expression::NFExpression>>
                    + 'static,
            >),
        Some(
            (std::sync::Arc::new({
                let __pe_b1 = set.clone();
                move |__pe_a0| checkLoweredCref(__pe_a0, __pe_b1.clone())
            })
                as std::sync::Arc<
                    dyn ::std::ops::Fn(
                            metamodelica::Ref<ComponentRef::NFComponentRef>,
                        )
                            -> Result<metamodelica::Ref<ComponentRef::NFComponentRef>>
                        + 'static,
                >),
        ),
        (std::sync::Arc::new(Expression::map)
            as std::sync::Arc<
                dyn ::std::ops::Fn(
                        metamodelica::Ref<Expression::NFExpression>,
                        Arc<
                            dyn ::std::ops::Fn(
                                    metamodelica::Ref<Expression::NFExpression>,
                                )
                                    -> Result<metamodelica::Ref<Expression::NFExpression>>
                                + 'static,
                        >,
                    ) -> Result<metamodelica::Ref<Expression::NFExpression>>
                    + 'static,
            >),
    )?;
    if !(UnorderedSet::isEmpty(set.clone())) {
        metamodelica::print({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("[failtrace] the equation:\n"));
            __mm_s.push_str(&*BEquation::Equation::toString(eqn.clone(), literal!(""))?);
            __mm_s.push_str(&*literal!("\n"));
            ArcStr::from(__mm_s)
        });
        metamodelica::print({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!(
                "[failtrace] has following non-lowered component references: "
            ));
            __mm_s.push_str(&*List::toString(
                UnorderedSet::toList(set),
                &move |__a0: metamodelica::Ref<ComponentRef::NFComponentRef>| ComponentRef::toString(&__a0),
                List::Style::FLAT_CURLY.clone(),
            )?);
            __mm_s.push_str(&*literal!("\n"));
            ArcStr::from(__mm_s)
        });
    }
    Ok(eqn)
}

pub(crate) fn checkLoweredCrefExp(
    mut exp: metamodelica::Ref<Expression::NFExpression>,
    mut set: metamodelica::Ref<UnorderedSet::UnorderedSet<metamodelica::Ref<ComponentRef::NFComponentRef>>>,
) -> Result<metamodelica::Ref<Expression::NFExpression>> {
    let mut exp: metamodelica::Ref<Expression::NFExpression> = exp;
    let () = (match &*exp {
        Expression::CREF { cref: __exp_cref, .. } => {
            checkLoweredCref(__exp_cref.clone(), set)?;
            ()
        }
        _ => (),
    });
    Ok(exp)
}

pub(crate) fn checkLoweredCref(
    mut cref: metamodelica::Ref<ComponentRef::NFComponentRef>,
    mut set: metamodelica::Ref<UnorderedSet::UnorderedSet<metamodelica::Ref<ComponentRef::NFComponentRef>>>,
) -> Result<metamodelica::Ref<ComponentRef::NFComponentRef>> {
    let mut cref: metamodelica::Ref<ComponentRef::NFComponentRef> = cref;
    let () = (match &*cref {
        ComponentRef::CREF { .. } if (NFInstNode::InstNode::isVar(&(ComponentRef::node(&cref)?))) => (),
        ComponentRef::CREF { .. } if (NFInstNode::InstNode::isName(&(ComponentRef::node(&cref)?))) => (),
        ComponentRef::CREF { .. } => {
            UnorderedSet::add(cref.clone(), set)?;
            ()
        }
        _ => (),
    });
    Ok(cref)
}
