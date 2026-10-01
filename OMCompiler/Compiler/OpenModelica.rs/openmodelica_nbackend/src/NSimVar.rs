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

use crate::NBEquation::Equation;
use crate::NBEvents::Condition;
use crate::NBEvents::EventInfo;
use crate::NBPartition::Partition;
use crate::NBSlice as Slice;
use crate::NBStrongComponent as StrongComponent;
use crate::NBVariable as BVariable;
use crate::NBVariable::VariablePointers;
use crate::NSimCode as SimCode;
use crate::NSimCode::SimCodeIndices;
use openmodelica_backend_types::BackendDAE as OldBackendDAE;
use openmodelica_frontend_types::DAE;
use openmodelica_frontend_types::SCode;
use openmodelica_nf_frontend::NFBackendExtension::BackendInfo;
use openmodelica_nf_frontend::NFBackendExtension::VariableAttributes;
use openmodelica_nf_frontend::NFBackendExtension::VariableKind;
use openmodelica_nf_frontend::NFBinding as Binding;
use openmodelica_nf_frontend::NFComponentRef as ComponentRef;
use openmodelica_nf_frontend::NFDimension as Dimension;
use openmodelica_nf_frontend::NFExpression as Expression;
use openmodelica_nf_frontend::NFInstNode::InstNode;
use openmodelica_nf_frontend::NFOperator as Operator;
use openmodelica_nf_frontend::NFPrefixes as Prefixes;
use openmodelica_nf_frontend::NFSimplifyExp as SimplifyExp;
use openmodelica_nf_frontend::NFType as Type;
use openmodelica_nf_frontend::NFVariable as Variable;
use openmodelica_simcode_types::SimCode as OldSimCode;
use openmodelica_simcode_types::SimCodeVar as OldSimCodeVar;
use openmodelica_util::Config;
use openmodelica_util::Error;
use openmodelica_util::Flags;
use openmodelica_util::StringUtil;
use openmodelica_util::UnorderedMap;
use openmodelica_util::UnorderedSet;
use openmodelica_util::Util;
use openmodelica_util_datatypes_basic::List;
use openmodelica_util_datatypes_basic::Pointer;
use openmodelica_util_datatypes_basic::PointerWeak;

// OF imports
// NF imports
// Old Backend imports
// Backend imports
// Old Simcode imports
// SimCode imports
// Util imports
pub type ConvertEntry = (
    metamodelica::Ref<SimVar::SimVar>,
    metamodelica::Ref<OldSimCodeVar::SimVar>,
);

/// SimVar -> old SimVar conversions already made
pub type ConvertMemo = metamodelica::Ref<
    UnorderedMap::UnorderedMap<
        metamodelica::Ref<ComponentRef::NFComponentRef>,
        (
            metamodelica::Ref<SimVar::SimVar>,
            metamodelica::Ref<OldSimCodeVar::SimVar>,
        ),
    >,
>;

pub mod SimVar {
    use super::*;
    /// Information about a variable in a Modelica model.
    #[derive(Clone, Debug, Eq, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
    pub struct SimVar {
        pub name: metamodelica::Ref<ComponentRef::NFComponentRef>,
        pub varKind: metamodelica::Ref<VariableKind::VariableKind>,
        pub comment: ArcStr,
        pub unit: ArcStr,
        pub displayUnit: ArcStr,
        pub index: i32,
        pub min: Option<metamodelica::Ref<Expression::NFExpression>>,
        pub max: Option<metamodelica::Ref<Expression::NFExpression>>,
        pub start: Option<metamodelica::Ref<Expression::NFExpression>>,
        pub nominal: Option<metamodelica::Ref<Expression::NFExpression>>,
        pub isFixed: bool,
        pub type_: metamodelica::Ref<Type::NFType>,
        pub isDiscrete: bool,
        /// the name of the array if this variable is the first in that array
        pub arrayCref: Option<metamodelica::Ref<ComponentRef::NFComponentRef>>,
        pub aliasvar: metamodelica::Ref<Alias::Alias>,
        pub info: SourceInfo,
        pub causality: Option<Causality>,
        /// valueReference
        pub variable_index: Option<i32>,
        /// index of variable in modelDescription.xml
        pub fmi_index: Option<i32>,
        pub numArrayElement: metamodelica::List<metamodelica::Ref<Expression::NFExpression>>,
        pub isValueChangeable: bool,
        pub isProtected: bool,
        pub hideResult: bool,
        pub isEncrypted: bool,
        pub inputIndex: Option<metamodelica::Array<i32>>,
        /// if the varibale is a jacobian var, this is the corresponding matrix
        pub matrixName: Option<ArcStr>,
        /// FMI-2.0 variabilty attribute
        pub variability: Option<Variability>,
        /// FMI-2.0 initial attribute
        pub initial_: Option<Initial>,
        /// variables will only be exported to the modelDescription.xml if this attribute is SOME(cref) and this cref is only used in ModelDescription.xml for FMI-2.0 export
        pub exportVar: Option<metamodelica::Ref<ComponentRef::NFComponentRef>>,
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
            metamodelica::gc::MMTrace::mm_accept(&self.min, __mmv)?;
            metamodelica::gc::MMTrace::mm_accept(&self.max, __mmv)?;
            metamodelica::gc::MMTrace::mm_accept(&self.start, __mmv)?;
            metamodelica::gc::MMTrace::mm_accept(&self.nominal, __mmv)?;
            metamodelica::gc::MMTrace::mm_accept(&self.isFixed, __mmv)?;
            metamodelica::gc::MMTrace::mm_accept(&self.type_, __mmv)?;
            metamodelica::gc::MMTrace::mm_accept(&self.isDiscrete, __mmv)?;
            metamodelica::gc::MMTrace::mm_accept(&self.arrayCref, __mmv)?;
            metamodelica::gc::MMTrace::mm_accept(&self.aliasvar, __mmv)?;
            metamodelica::gc::MMTrace::mm_accept(&self.info, __mmv)?;
            metamodelica::gc::MMTrace::mm_accept(&self.causality, __mmv)?;
            metamodelica::gc::MMTrace::mm_accept(&self.variable_index, __mmv)?;
            metamodelica::gc::MMTrace::mm_accept(&self.fmi_index, __mmv)?;
            metamodelica::gc::MMTrace::mm_accept(&self.numArrayElement, __mmv)?;
            metamodelica::gc::MMTrace::mm_accept(&self.isValueChangeable, __mmv)?;
            metamodelica::gc::MMTrace::mm_accept(&self.isProtected, __mmv)?;
            metamodelica::gc::MMTrace::mm_accept(&self.hideResult, __mmv)?;
            metamodelica::gc::MMTrace::mm_accept(&self.isEncrypted, __mmv)?;
            metamodelica::gc::MMTrace::mm_accept(&self.inputIndex, __mmv)?;
            metamodelica::gc::MMTrace::mm_accept(&self.matrixName, __mmv)?;
            metamodelica::gc::MMTrace::mm_accept(&self.variability, __mmv)?;
            metamodelica::gc::MMTrace::mm_accept(&self.initial_, __mmv)?;
            metamodelica::gc::MMTrace::mm_accept(&self.exportVar, __mmv)?;
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
                min: Default::default(),
                max: Default::default(),
                start: Default::default(),
                nominal: Default::default(),
                isFixed: Default::default(),
                type_: Default::default(),
                isDiscrete: Default::default(),
                arrayCref: Default::default(),
                aliasvar: Default::default(),
                info: Default::default(),
                causality: Default::default(),
                variable_index: Default::default(),
                fmi_index: Default::default(),
                numArrayElement: Default::default(),
                isValueChangeable: Default::default(),
                isProtected: Default::default(),
                hideResult: Default::default(),
                isEncrypted: Default::default(),
                inputIndex: Default::default(),
                matrixName: Default::default(),
                variability: Default::default(),
                initial_: Default::default(),
                exportVar: Default::default(),
                isConnectorFlow: Default::default(),
            }
        }
    }

    pub type SIMVAR = SimVar;

    pub(crate) fn toString(mut var: &metamodelica::Ref<SimVar>, mut r#str: ArcStr) -> Result<ArcStr> {
        let mut r#str: ArcStr = r#str;
        r#str = {
            let mut __mm_s = String::new();
            __mm_s.push_str(&*r#str);
            __mm_s.push_str(&*literal!("("));
            __mm_s.push_str(&*intString(var.index.clone()));
            __mm_s.push_str(&*literal!(")"));
            __mm_s.push_str(&*VariableKind::toString(&var.varKind));
            __mm_s.push_str(&*literal!(" ("));
            __mm_s.push_str(&*intString(size(var)?));
            __mm_s.push_str(&*literal!(") "));
            __mm_s.push_str(&*Type::toString(&var.type_)?);
            __mm_s.push_str(&*literal!(" "));
            __mm_s.push_str(&*ComponentRef::toString(&var.name)?);
            ArcStr::from(__mm_s)
        };
        if (var.start).is_some() {
            r#str = {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*r#str);
                __mm_s.push_str(&*literal!(" = "));
                __mm_s.push_str(&*Expression::toString(Util::getOption(var.start.clone())?)?);
                ArcStr::from(__mm_s)
            };
        }
        Ok(r#str)
    }

    pub(crate) fn listToString(
        mut var_lst: &metamodelica::List<metamodelica::Ref<SimVar>>,
        mut r#str: ArcStr,
        mut printAlias: bool,
    ) -> Result<ArcStr> {
        let mut r#str: ArcStr = r#str;
        if !((var_lst).is_empty()) {
            r#str = StringUtil::headline_4(
                &({
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*r#str);
                    __mm_s.push_str(&*literal!(" ("));
                    __mm_s.push_str(&*intString(((var_lst).len() as i32)));
                    __mm_s.push_str(&*literal!(")"));
                    ArcStr::from(__mm_s)
                }),
            )?;
            for mut var in &**var_lst {
                r#str = {
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*r#str);
                    __mm_s.push_str(&*toString(metamodelica::AsArg::as_arg(&var), literal!("  "))?);
                    ArcStr::from(__mm_s)
                };
                r#str = if (printAlias) {
                    {
                        let mut __mm_s = String::new();
                        __mm_s.push_str(&*r#str);
                        __mm_s.push_str(&*literal!(" "));
                        __mm_s.push_str(&*Alias::toString(&var.aliasvar)?);
                        __mm_s.push_str(&*literal!("\n"));
                        ArcStr::from(__mm_s)
                    }
                } else {
                    {
                        let mut __mm_s = String::new();
                        __mm_s.push_str(&*r#str);
                        __mm_s.push_str(&*literal!("\n"));
                        ArcStr::from(__mm_s)
                    }
                };
            }
            r#str = {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*r#str);
                __mm_s.push_str(&*literal!("\n"));
                ArcStr::from(__mm_s)
            };
        } else {
            r#str = literal!("");
        }
        Ok(r#str)
    }

    pub(crate) fn create(
        mut var: &metamodelica::Ref<Variable::NFVariable>,
        mut uniqueIndex: i32,
        mut typeIndex: i32,
        mut alias: metamodelica::Ref<Alias::Alias>,
    ) -> Result<metamodelica::Ref<SimVar>> {
        let mut simVar: metamodelica::Ref<SimVar>;
        simVar = (match &**var {
            Variable::VARIABLE { .. } => {
                let mut varKind: metamodelica::Ref<VariableKind::VariableKind>;
                let mut comment: ArcStr;
                let mut unit: ArcStr;
                let mut displayUnit: ArcStr;
                let mut min: Option<metamodelica::Ref<Expression::NFExpression>>;
                let mut max: Option<metamodelica::Ref<Expression::NFExpression>>;
                let mut start: Option<metamodelica::Ref<Expression::NFExpression>>;
                let mut nominal: Option<metamodelica::Ref<Expression::NFExpression>>;
                let mut isFixed: bool;
                let mut isDiscrete: bool;
                let mut isProtected: bool;
                let mut isValueChangeable: bool;
                let mut causality: Causality;
                let mut result: metamodelica::Ref<SimVar>;
                comment = parseComment(&var.comment);
                (
                    varKind,
                    unit,
                    displayUnit,
                    min,
                    max,
                    start,
                    nominal,
                    isFixed,
                    isDiscrete,
                    isProtected,
                ) = parseAttributes(&var.backendinfo)?;
                (start, isValueChangeable, causality) = parseBinding(start, var);
                result = metamodelica::Ref::new(SimVar {
                    name: var.name.clone(),
                    varKind: varKind,
                    comment: comment,
                    unit: unit,
                    displayUnit: displayUnit,
                    index: typeIndex,
                    min: min,
                    max: max,
                    start: start,
                    nominal: nominal,
                    isFixed: isFixed,
                    type_: var.ty.clone(),
                    isDiscrete: isDiscrete,
                    arrayCref: ComponentRef::getArrayCrefOpt(&var.name)?,
                    aliasvar: alias,
                    info: var.info.clone(),
                    causality: Some(causality),
                    variable_index: Some(uniqueIndex),
                    fmi_index: Some(typeIndex),
                    numArrayElement: ({
                        let mut __acc: metamodelica::List<metamodelica::Ref<Expression::NFExpression>> =
                            metamodelica::nil();
                        for mut dim in (Type::arrayDims(var.ty.clone())).into_iter().cloned() {
                            let __x = Dimension::sizeExp(&(dim.clone()))?;
                            __acc = cons(__x, __acc);
                        }
                        __acc.reverse()
                    }),
                    isValueChangeable: isValueChangeable,
                    isProtected: isProtected,
                    hideResult: var.backendinfo.annotations.hideResult.clone(),
                    isEncrypted: Variable::isEncrypted(var)?,
                    inputIndex: None,
                    matrixName: None,
                    variability: None,
                    initial_: None,
                    exportVar: Some(var.name.clone()),
                    isConnectorFlow: Variable::isFlow(var),
                });
                result
            }
            _ => {
                Error::addMessage(
                    Error::INTERNAL_ERROR.clone(),
                    list![{
                        let mut __mm_s = String::new();
                        __mm_s.push_str(&*literal!("NSimVar.SimVar.create"));
                        __mm_s.push_str(&*literal!(" failed for variable "));
                        __mm_s.push_str(&*ComponentRef::toString(&var.name)?);
                        __mm_s.push_str(&*literal!("."));
                        ArcStr::from(__mm_s)
                    }],
                )?;
                return Err("fail");
            }
        });
        Ok(simVar)
    }

    pub(crate) fn traverseCreate(
        mut var: metamodelica::Ref<Variable::NFVariable>,
        mut acc: Pointer::Pointer<metamodelica::List<metamodelica::Ref<SimVar>>>,
        mut indices_ptr: Pointer::Pointer<SimCodeIndices>,
        mut varType: VarType,
    ) -> Result<metamodelica::Ref<Variable::NFVariable>> {
        let mut var: metamodelica::Ref<Variable::NFVariable> = var;
        let mut simCodeIndices: SimCodeIndices = Pointer::access(indices_ptr.clone());
        let () = (match varType {
            VarType::SIMULATION => {
                Pointer::update(
                    acc.clone(),
                    metamodelica::cons(
                        create(
                            &var,
                            simCodeIndices.uniqueIndex.clone(),
                            simCodeIndices.realVarIndex.clone(),
                            crate::NSimVar::Alias::interned_NO_ALIAS(),
                        )?,
                        Pointer::access(acc),
                    ),
                );
                simCodeIndices.uniqueIndex = simCodeIndices.uniqueIndex.clone() + 1;
                simCodeIndices.realVarIndex = simCodeIndices.realVarIndex.clone() + 1;
                ()
            }
            VarType::PARAMETER { .. } => {
                Pointer::update(
                    acc.clone(),
                    metamodelica::cons(
                        create(
                            &var,
                            simCodeIndices.uniqueIndex.clone(),
                            simCodeIndices.realParamIndex.clone(),
                            crate::NSimVar::Alias::interned_NO_ALIAS(),
                        )?,
                        Pointer::access(acc),
                    ),
                );
                simCodeIndices.uniqueIndex = simCodeIndices.uniqueIndex.clone() + 1;
                simCodeIndices.realParamIndex = simCodeIndices.realParamIndex.clone() + 1;
                ()
            }
            VarType::ALIAS { .. } => {
                Pointer::update(
                    acc.clone(),
                    metamodelica::cons(
                        create(
                            &(var.clone()),
                            simCodeIndices.uniqueIndex.clone(),
                            simCodeIndices.realAliasIndex.clone(),
                            Alias::fromBinding(&var.binding)?,
                        )?,
                        Pointer::access(acc),
                    ),
                );
                simCodeIndices.uniqueIndex = simCodeIndices.uniqueIndex.clone() + 1;
                simCodeIndices.realAliasIndex = simCodeIndices.realAliasIndex.clone() + 1;
                ()
            }
            VarType::RESIDUAL { .. } => {
                Pointer::update(
                    acc.clone(),
                    metamodelica::cons(
                        create(
                            &var,
                            simCodeIndices.uniqueIndex.clone(),
                            simCodeIndices.residualIndex.clone(),
                            crate::NSimVar::Alias::interned_NO_ALIAS(),
                        )?,
                        Pointer::access(acc),
                    ),
                );
                simCodeIndices.uniqueIndex = simCodeIndices.uniqueIndex.clone() + 1;
                simCodeIndices.residualIndex = simCodeIndices.residualIndex.clone() + 1;
                ()
            }
            VarType::EXTERNAL_OBJECT { .. } => {
                Pointer::update(
                    acc.clone(),
                    metamodelica::cons(
                        create(
                            &var,
                            simCodeIndices.uniqueIndex.clone(),
                            simCodeIndices.extObjIndex.clone(),
                            crate::NSimVar::Alias::interned_NO_ALIAS(),
                        )?,
                        Pointer::access(acc),
                    ),
                );
                simCodeIndices.uniqueIndex = simCodeIndices.uniqueIndex.clone() + 1;
                simCodeIndices.extObjIndex = simCodeIndices.extObjIndex.clone() + 1;
                ()
            }
            _ => {
                Error::addMessage(
                    Error::INTERNAL_ERROR.clone(),
                    list![{
                        let mut __mm_s = String::new();
                        __mm_s.push_str(&*literal!("NSimVar.SimVar.traverseCreate"));
                        __mm_s.push_str(&*literal!(" failed for variable "));
                        __mm_s.push_str(&*ComponentRef::toString(&var.name)?);
                        __mm_s.push_str(&*literal!("."));
                        ArcStr::from(__mm_s)
                    }],
                )?;
                return Err("fail");
            }
        });
        Pointer::update(indices_ptr, simCodeIndices);
        Ok(var)
    }

    pub(crate) fn createList(
        mut vars: &metamodelica::List<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>>,
        mut varType: VarType,
        mut indices: SimCodeIndices,
    ) -> Result<(metamodelica::List<metamodelica::Ref<SimVar>>, SimCodeIndices)> {
        let mut simVars: metamodelica::List<metamodelica::Ref<SimVar>> = metamodelica::nil();
        let mut indices: SimCodeIndices = indices;
        let mut uniq: i32 = indices.uniqueIndex.clone();
        let mut idx: i32 = getTypeIndex(&indices, varType)?;
        let mut var: metamodelica::Ref<Variable::NFVariable>;
        for mut var_ptr in &**vars {
            var = Pointer::access(var_ptr.clone());
            simVars = metamodelica::cons(
                create(
                    &(var.clone()),
                    uniq,
                    idx,
                    if (varType == VarType::ALIAS.clone()) {
                        Alias::fromBinding(&var.binding)?
                    } else {
                        crate::NSimVar::Alias::interned_NO_ALIAS()
                    },
                )?,
                simVars,
            );
            uniq = uniq + 1;
            idx = idx + 1;
        }
        simVars = metamodelica::Dangerous::listReverseInPlace(simVars);
        indices.uniqueIndex = uniq;
        indices = setTypeIndex(indices, varType, idx)?;
        Ok((simVars, indices))
    }

    pub(crate) fn createListsByType(
        mut vars: &metamodelica::List<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>>,
        mut varType: VarType,
        mut indices: SimCodeIndices,
    ) -> Result<(
        metamodelica::List<metamodelica::List<metamodelica::Ref<SimVar>>>,
        SimCodeIndices,
    )> {
        let mut simVars: metamodelica::List<metamodelica::List<metamodelica::Ref<SimVar>>>;
        let mut indices: SimCodeIndices = indices;
        let mut uniq: i32 = indices.uniqueIndex.clone();
        let mut real_idx: i32;
        let mut int_idx: i32;
        let mut bool_idx: i32;
        let mut string_idx: i32;
        let mut enum_idx: i32;
        let mut real_lst: metamodelica::List<metamodelica::Ref<SimVar>> = metamodelica::nil();
        let mut int_lst: metamodelica::List<metamodelica::Ref<SimVar>> = metamodelica::nil();
        let mut bool_lst: metamodelica::List<metamodelica::Ref<SimVar>> = metamodelica::nil();
        let mut string_lst: metamodelica::List<metamodelica::Ref<SimVar>> = metamodelica::nil();
        let mut enum_lst: metamodelica::List<metamodelica::Ref<SimVar>> = metamodelica::nil();
        let mut var: metamodelica::Ref<Variable::NFVariable>;
        let mut alias: metamodelica::Ref<Alias::Alias>;
        (real_idx, int_idx, bool_idx, string_idx, enum_idx) = getTypeIndices(&indices, varType)?;
        for mut var_ptr in &**vars {
            var = Pointer::access(var_ptr.clone());
            alias = if (varType == VarType::ALIAS.clone()) {
                Alias::fromBinding(&var.binding)?
            } else {
                crate::NSimVar::Alias::interned_NO_ALIAS()
            };
            let () = (match &*(Type::arrayElementType(&var.ty)) {
                Type::REAL => {
                    real_lst = metamodelica::cons(create(&var, uniq, real_idx, alias)?, real_lst);
                    real_idx = real_idx + 1;
                    uniq = uniq + 1;
                    ()
                }
                Type::INTEGER => {
                    int_lst = metamodelica::cons(create(&var, uniq, int_idx, alias)?, int_lst);
                    int_idx = int_idx + 1;
                    uniq = uniq + 1;
                    ()
                }
                Type::BOOLEAN => {
                    bool_lst = metamodelica::cons(create(&var, uniq, bool_idx, alias)?, bool_lst);
                    bool_idx = bool_idx + 1;
                    uniq = uniq + 1;
                    ()
                }
                Type::STRING => {
                    string_lst = metamodelica::cons(create(&var, uniq, string_idx, alias)?, string_lst);
                    string_idx = string_idx + 1;
                    uniq = uniq + 1;
                    ()
                }
                Type::ENUMERATION { .. } => {
                    enum_lst = metamodelica::cons(create(&var, uniq, enum_idx, alias)?, enum_lst);
                    enum_idx = enum_idx + 1;
                    uniq = uniq + 1;
                    ()
                }
                Type::CLOCK => (),
                _ => {
                    Error::addMessage(
                        Error::INTERNAL_ERROR.clone(),
                        list![{
                            let mut __mm_s = String::new();
                            __mm_s.push_str(&*literal!("NSimVar.SimVar.createListsByType"));
                            __mm_s.push_str(&*literal!(" failed because of unhandled Variable "));
                            __mm_s.push_str(&*ComponentRef::toString(&var.name)?);
                            __mm_s.push_str(&*literal!("."));
                            ArcStr::from(__mm_s)
                        }],
                    )?;
                    return Err("fail");
                }
            });
        }
        simVars = list![
            metamodelica::Dangerous::listReverseInPlace(real_lst),
            metamodelica::Dangerous::listReverseInPlace(int_lst),
            metamodelica::Dangerous::listReverseInPlace(bool_lst),
            metamodelica::Dangerous::listReverseInPlace(string_lst),
            metamodelica::Dangerous::listReverseInPlace(enum_lst)
        ];
        indices.uniqueIndex = uniq;
        indices = setTypeIndices(indices, varType, real_idx, int_idx, bool_idx, string_idx, enum_idx)?;
        Ok((simVars, indices))
    }

    pub(crate) fn getTypeIndex(mut indices: &SimCodeIndices, mut varType: VarType) -> Result<i32> {
        let mut idx: i32;
        idx = (match varType {
            VarType::SIMULATION => indices.realVarIndex.clone(),
            VarType::PARAMETER { .. } => indices.realParamIndex.clone(),
            VarType::ALIAS { .. } => indices.realAliasIndex.clone(),
            VarType::RESIDUAL { .. } => indices.residualIndex.clone(),
            VarType::EXTERNAL_OBJECT { .. } => indices.extObjIndex.clone(),
        });
        Ok(idx)
    }

    pub(crate) fn setTypeIndex(
        mut indices: SimCodeIndices,
        mut varType: VarType,
        mut idx: i32,
    ) -> Result<SimCodeIndices> {
        let mut indices: SimCodeIndices = indices;
        let () = (match varType {
            VarType::SIMULATION => {
                indices.realVarIndex = idx;
                ()
            }
            VarType::PARAMETER { .. } => {
                indices.realParamIndex = idx;
                ()
            }
            VarType::ALIAS { .. } => {
                indices.realAliasIndex = idx;
                ()
            }
            VarType::RESIDUAL { .. } => {
                indices.residualIndex = idx;
                ()
            }
            VarType::EXTERNAL_OBJECT { .. } => {
                indices.extObjIndex = idx;
                ()
            }
        });
        Ok(indices)
    }

    pub(crate) fn getTypeIndices(
        mut indices: &SimCodeIndices,
        mut varType: VarType,
    ) -> Result<(i32, i32, i32, i32, i32)> {
        let mut real_idx: i32;
        let mut int_idx: i32;
        let mut bool_idx: i32;
        let mut string_idx: i32;
        let mut enum_idx: i32;
        (real_idx, int_idx, bool_idx, string_idx, enum_idx) = (match varType {
            VarType::SIMULATION => (
                indices.realVarIndex.clone(),
                indices.integerVarIndex.clone(),
                indices.booleanVarIndex.clone(),
                indices.stringVarIndex.clone(),
                indices.enumerationVarIndex.clone(),
            ),
            VarType::PARAMETER { .. } => (
                indices.realParamIndex.clone(),
                indices.integerParamIndex.clone(),
                indices.booleanParamIndex.clone(),
                indices.stringParamIndex.clone(),
                indices.enumerationParamIndex.clone(),
            ),
            VarType::ALIAS { .. } => (
                indices.realAliasIndex.clone(),
                indices.integerAliasIndex.clone(),
                indices.booleanAliasIndex.clone(),
                indices.stringAliasIndex.clone(),
                indices.enumerationAliasIndex.clone(),
            ),
            _ => {
                Error::addMessage(
                    Error::INTERNAL_ERROR.clone(),
                    list![{
                        let mut __mm_s = String::new();
                        __mm_s.push_str(&*literal!("NSimVar.SimVar.getTypeIndices"));
                        __mm_s.push_str(&*literal!(" failed because of unhandled VarType."));
                        ArcStr::from(__mm_s)
                    }],
                )?;
                return Err("fail");
            }
        });
        Ok((real_idx, int_idx, bool_idx, string_idx, enum_idx))
    }

    pub(crate) fn setTypeIndices(
        mut indices: SimCodeIndices,
        mut varType: VarType,
        mut real_idx: i32,
        mut int_idx: i32,
        mut bool_idx: i32,
        mut string_idx: i32,
        mut enum_idx: i32,
    ) -> Result<SimCodeIndices> {
        let mut indices: SimCodeIndices = indices;
        let () = (match varType {
            VarType::SIMULATION => {
                indices.realVarIndex = real_idx;
                indices.integerVarIndex = int_idx;
                indices.booleanVarIndex = bool_idx;
                indices.stringVarIndex = string_idx;
                indices.enumerationVarIndex = enum_idx;
                ()
            }
            VarType::PARAMETER { .. } => {
                indices.realParamIndex = real_idx;
                indices.integerParamIndex = int_idx;
                indices.booleanParamIndex = bool_idx;
                indices.stringParamIndex = string_idx;
                indices.enumerationParamIndex = enum_idx;
                ()
            }
            VarType::ALIAS { .. } => {
                indices.realAliasIndex = real_idx;
                indices.integerAliasIndex = int_idx;
                indices.booleanAliasIndex = bool_idx;
                indices.stringAliasIndex = string_idx;
                indices.enumerationAliasIndex = enum_idx;
                ()
            }
            _ => return Err("match: no arm matched"),
        });
        Ok(indices)
    }

    pub(crate) fn createFromResidualComponent(
        mut comp: metamodelica::Ref<StrongComponent::NBStrongComponent>,
        mut acc: Pointer::Pointer<metamodelica::List<metamodelica::Ref<SimVar>>>,
        mut indices_ptr: Pointer::Pointer<SimCodeIndices>,
        mut varType: VarType,
    ) -> Result<metamodelica::Ref<StrongComponent::NBStrongComponent>> {
        let mut comp: metamodelica::Ref<StrongComponent::NBStrongComponent> = comp;
        let () = (match &*comp {
            StrongComponent::SINGLE_COMPONENT { eqn: __comp_eqn, .. } if (Equation::isResidual(__comp_eqn.clone())) => {
                traverseCreate(
                    Pointer::access(Equation::getResidualVar(__comp_eqn.clone())?),
                    acc,
                    indices_ptr,
                    varType,
                )?;
                ()
            }
            _ => (),
        });
        Ok(comp)
    }

    pub(crate) fn size(mut var: &metamodelica::Ref<SimVar>) -> Result<i32> {
        let mut s: i32 = Type::sizeOf(&var.type_, false)?;
        Ok(s)
    }

    pub(crate) fn getName(mut var: &metamodelica::Ref<SimVar>) -> metamodelica::Ref<ComponentRef::NFComponentRef> {
        let mut name: metamodelica::Ref<ComponentRef::NFComponentRef> = var.name.clone();
        name
    }

    pub(crate) fn getIndex(
        mut cref: metamodelica::Ref<ComponentRef::NFComponentRef>,
        mut sim_map: metamodelica::Ref<
            UnorderedMap::UnorderedMap<metamodelica::Ref<ComponentRef::NFComponentRef>, metamodelica::Ref<SimVar>>,
        >,
    ) -> Result<i32> {
        let mut index: i32;
        let mut var: metamodelica::Ref<SimVar>;
        match '__try0: {
            var = unwrap_break_err!(UnorderedMap::getSafe(cref.clone(), sim_map.clone(), metamodelica::sourceInfo!("NSimCode/NSimVar.mo")), '__try0);
            index = var.index.clone();
            Ok::<_, &'static str>((index.clone(), var.clone()))
        } {
            Ok((__try0_o0, __try0_o1)) => {
                index = __try0_o0;
                var = __try0_o1;
            }
            Err(__try0_err) => {
                Error::addMessage(
                    Error::INTERNAL_ERROR.clone(),
                    list![{
                        let mut __mm_s = String::new();
                        __mm_s.push_str(&*literal!("NSimVar.SimVar.getIndex"));
                        __mm_s.push_str(&*literal!(" failed to get index for cref: "));
                        __mm_s.push_str(&*ComponentRef::toString(&cref)?);
                        ArcStr::from(__mm_s)
                    }],
                )?;
                return Err(__try0_err);
            }
        }
        Ok(index)
    }

    pub(crate) fn shiftIndex(mut var: metamodelica::Ref<SimVar>, mut shift: i32) -> Result<metamodelica::Ref<SimVar>> {
        let mut var: metamodelica::Ref<SimVar> = var;
        assign_field!(var.index = var.index.clone() + shift);
        if (var.fmi_index).is_some() {
            assign_field!(var.fmi_index = Some(Util::getOption(var.fmi_index.clone())? + shift));
        }
        Ok(var)
    }

    pub(crate) fn convert(mut simVar: &metamodelica::Ref<SimVar>) -> Result<metamodelica::Ref<OldSimCodeVar::SimVar>> {
        let mut oldSimVar: metamodelica::Ref<OldSimCodeVar::SimVar>;
        let mut name: metamodelica::Ref<DAE::ComponentRef> = ComponentRef::toDAE(&simVar.name)?;
        oldSimVar = metamodelica::Ref::new(OldSimCodeVar::SimVar {
            name: name.clone(),
            varKind: convertVarKind(&simVar.varKind)?,
            comment: simVar.comment.clone(),
            unit: simVar.unit.clone(),
            displayUnit: simVar.displayUnit.clone(),
            index: simVar.index.clone(),
            minValue: convertAttribute(simVar.min.clone())?,
            maxValue: convertAttribute(simVar.max.clone())?,
            initialValue: convertAttribute(simVar.start.clone())?,
            nominalValue: convertAttribute(simVar.nominal.clone())?,
            isFixed: simVar.isFixed.clone(),
            type_: Type::toDAE(&simVar.type_, true)?,
            isDiscrete: simVar.isDiscrete.clone(),
            arrayCref: Util::applyOption(simVar.arrayCref.clone(), &move |__a0: metamodelica::Ref<
                ComponentRef::NFComponentRef,
            >| ComponentRef::toDAE(&__a0))?,
            aliasvar: Alias::convert(&simVar.aliasvar)?,
            source: DAE::emptyElementSource().clone(),
            causality: Util::applyOption(simVar.causality.clone(), &convertCausality)?,
            variable_index: simVar.variable_index.clone(),
            fmi_index: simVar.fmi_index.clone(),
            numArrayElement: ({
                let mut __acc: metamodelica::List<ArcStr> = metamodelica::nil();
                for mut e in (simVar.numArrayElement.clone()).into_iter().cloned() {
                    let __x = Expression::toString(e.clone())?;
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            }),
            isValueChangeable: simVar.isValueChangeable.clone(),
            isProtected: simVar.isProtected.clone(),
            hideResult: Some(simVar.hideResult.clone()),
            isEncrypted: simVar.isEncrypted.clone(),
            inputIndex: simVar.inputIndex.clone(),
            initNonlinear: false,
            matrixName: simVar.matrixName.clone(),
            variability: Some(convertVariability(&simVar.varKind)),
            initial_: convertInitial(
                convertVariability(&simVar.varKind),
                Util::applyOption(simVar.causality.clone(), &convertCausality)?,
            ),
            exportVar: convertExportVar(simVar.exportVar.clone(), &simVar.name, name)?,
            relativeQuantity: false,
            isConnectorFlow: simVar.isConnectorFlow.clone(),
        });
        Ok(oldSimVar)
    }

    pub(crate) fn convertExportVar(
        mut exportVar: Option<metamodelica::Ref<ComponentRef::NFComponentRef>>,
        mut varName: &metamodelica::Ref<ComponentRef::NFComponentRef>,
        mut converted: metamodelica::Ref<DAE::ComponentRef>,
    ) -> Result<Option<metamodelica::Ref<DAE::ComponentRef>>> {
        let mut dcref: Option<metamodelica::Ref<DAE::ComponentRef>>;
        dcref = (::match_deref::match_deref! { match &(exportVar) {
            Some(cref) => {
                Some(if (referenceEq(&*(cref.clone()),&*(&**varName))) {converted} else {ComponentRef::toDAE(metamodelica::AsArg::as_arg(&cref))?})
            },
            _ => {
                None
            },
            _ => unreachable!("match_deref! exhaustiveness placeholder"),
        } });
        Ok(dcref)
    }

    pub(crate) fn convertAttribute(
        mut exp: Option<metamodelica::Ref<Expression::NFExpression>>,
    ) -> Result<Option<metamodelica::Ref<DAE::Exp>>> {
        let mut dexp: Option<metamodelica::Ref<DAE::Exp>>;
        dexp = (::match_deref::match_deref! { match &(exp) {
            Some(e) => {
                Some(Expression::toDAE(e.clone(), false)?)
            },
            _ => {
                None
            },
            _ => unreachable!("match_deref! exhaustiveness placeholder"),
        } });
        Ok(dexp)
    }

    pub(crate) fn convertCausality(mut c: Causality) -> Result<OldSimCodeVar::Causality> {
        let mut oc: OldSimCodeVar::Causality;
        oc = (match c {
            Causality::NONE => openmodelica_simcode_types::SimCodeVar::Causality::NONECAUS,
            Causality::OUTPUT => openmodelica_simcode_types::SimCodeVar::Causality::OUTPUT,
            Causality::INPUT => openmodelica_simcode_types::SimCodeVar::Causality::INPUT,
            Causality::LOCAL { .. } => openmodelica_simcode_types::SimCodeVar::Causality::LOCAL,
            Causality::PARAMETER { .. } => openmodelica_simcode_types::SimCodeVar::Causality::PARAMETER,
            Causality::CALCULATED_PARAMETER => openmodelica_simcode_types::SimCodeVar::Causality::CALCULATED_PARAMETER,
        });
        Ok(oc)
    }

    pub(crate) fn convertVariability(
        mut vk: &metamodelica::Ref<VariableKind::VariableKind>,
    ) -> OldSimCodeVar::Variability {
        let mut v: OldSimCodeVar::Variability;
        v = (match &**vk {
            VariableKind::CONSTANT => openmodelica_simcode_types::SimCodeVar::Variability::CONSTANT,
            VariableKind::PARAMETER { .. } => openmodelica_simcode_types::SimCodeVar::Variability::FIXED,
            VariableKind::DISCRETE => openmodelica_simcode_types::SimCodeVar::Variability::DISCRETE,
            VariableKind::DISCRETE_STATE => openmodelica_simcode_types::SimCodeVar::Variability::DISCRETE,
            VariableKind::CLOCKED => openmodelica_simcode_types::SimCodeVar::Variability::DISCRETE,
            VariableKind::PREVIOUS => openmodelica_simcode_types::SimCodeVar::Variability::DISCRETE,
            _ => openmodelica_simcode_types::SimCodeVar::Variability::CONTINUOUS,
        });
        v
    }

    pub(crate) fn convertInitial(
        mut v: OldSimCodeVar::Variability,
        mut c: Option<OldSimCodeVar::Causality>,
    ) -> Option<OldSimCodeVar::Initial> {
        let mut initial_: Option<OldSimCodeVar::Initial>;
        initial_ = (match (v, c) {
            (OldSimCodeVar::Variability::CONSTANT, _) => Some(openmodelica_simcode_types::SimCodeVar::Initial::EXACT),
            (OldSimCodeVar::Variability::FIXED, Some(OldSimCodeVar::Causality::PARAMETER)) => {
                Some(openmodelica_simcode_types::SimCodeVar::Initial::EXACT)
            }
            (OldSimCodeVar::Variability::TUNABLE, Some(OldSimCodeVar::Causality::PARAMETER)) => {
                Some(openmodelica_simcode_types::SimCodeVar::Initial::EXACT)
            }
            _ => None,
        });
        initial_
    }

    pub(crate) fn convertList(
        mut simVar_lst: metamodelica::List<metamodelica::Ref<SimVar>>,
    ) -> Result<metamodelica::List<metamodelica::Ref<OldSimCodeVar::SimVar>>> {
        let mut oldSimVar_lst: metamodelica::List<metamodelica::Ref<OldSimCodeVar::SimVar>> = ({
            let mut __acc: metamodelica::List<metamodelica::Ref<OldSimCodeVar::SimVar>> = metamodelica::nil();
            for mut simVar in (simVar_lst.clone()).into_iter().cloned() {
                let __x = convert(&(simVar.clone()))?;
                __acc = cons(__x, __acc);
            }
            __acc.reverse()
        });
        Ok(oldSimVar_lst)
    }

    pub(crate) fn newConvertMemo(
        mut size: i32,
    ) -> metamodelica::Ref<
        UnorderedMap::UnorderedMap<
            metamodelica::Ref<ComponentRef::NFComponentRef>,
            (metamodelica::Ref<SimVar>, metamodelica::Ref<OldSimCodeVar::SimVar>),
        >,
    > {
        let mut memo: metamodelica::Ref<
            UnorderedMap::UnorderedMap<
                metamodelica::Ref<ComponentRef::NFComponentRef>,
                (metamodelica::Ref<SimVar>, metamodelica::Ref<OldSimCodeVar::SimVar>),
            >,
        > = UnorderedMap::new(
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
            Util::nextPrime(size),
        );
        memo
    }

    pub(crate) fn convertMemo(
        mut simVar: metamodelica::Ref<SimVar>,
        mut memo: metamodelica::Ref<
            UnorderedMap::UnorderedMap<
                metamodelica::Ref<ComponentRef::NFComponentRef>,
                (metamodelica::Ref<SimVar>, metamodelica::Ref<OldSimCodeVar::SimVar>),
            >,
        >,
    ) -> Result<metamodelica::Ref<OldSimCodeVar::SimVar>> {
        let mut oldSimVar: metamodelica::Ref<OldSimCodeVar::SimVar> = convert(&simVar)?;
        UnorderedMap::add(simVar.name.clone(), (simVar, oldSimVar.clone()), memo)?;
        Ok(oldSimVar)
    }

    pub(crate) fn convertListMemo(
        mut simVar_lst: metamodelica::List<metamodelica::Ref<SimVar>>,
        mut memo: metamodelica::Ref<
            UnorderedMap::UnorderedMap<
                metamodelica::Ref<ComponentRef::NFComponentRef>,
                (metamodelica::Ref<SimVar>, metamodelica::Ref<OldSimCodeVar::SimVar>),
            >,
        >,
    ) -> Result<metamodelica::List<metamodelica::Ref<OldSimCodeVar::SimVar>>> {
        let mut oldSimVar_lst: metamodelica::List<metamodelica::Ref<OldSimCodeVar::SimVar>> = ({
            let mut __acc: metamodelica::List<metamodelica::Ref<OldSimCodeVar::SimVar>> = metamodelica::nil();
            for mut simVar in (simVar_lst.clone()).into_iter().cloned() {
                let __x = convertMemo(simVar.clone(), memo.clone())?;
                __acc = cons(__x, __acc);
            }
            __acc.reverse()
        });
        Ok(oldSimVar_lst)
    }

    pub(crate) fn convertMemoized(
        mut simVar: &metamodelica::Ref<SimVar>,
        mut memo: metamodelica::Ref<
            UnorderedMap::UnorderedMap<
                metamodelica::Ref<ComponentRef::NFComponentRef>,
                (metamodelica::Ref<SimVar>, metamodelica::Ref<OldSimCodeVar::SimVar>),
            >,
        >,
    ) -> Result<metamodelica::Ref<OldSimCodeVar::SimVar>> {
        let mut oldSimVar: metamodelica::Ref<OldSimCodeVar::SimVar>;
        let mut orig: metamodelica::Ref<SimVar>;
        oldSimVar = (::match_deref::match_deref! { match &(UnorderedMap::get(simVar.name.clone(), memo)?) {
            Some((orig, __esc_oldSimVar)) if (referenceEq(&*(orig.clone()),&*(&**simVar))) => {
                oldSimVar = (*__esc_oldSimVar).clone();
                oldSimVar.clone()
            },
            _ => convert(simVar)?,
            _ => unreachable!("match_deref! exhaustiveness placeholder"),
        } });
        Ok(oldSimVar)
    }

    pub(crate) fn convertTpl(
        mut tpl: &(metamodelica::Ref<SimVar>, bool),
    ) -> Result<(metamodelica::Ref<OldSimCodeVar::SimVar>, bool)> {
        let mut oldTpl: (metamodelica::Ref<OldSimCodeVar::SimVar>, bool);
        let mut var: metamodelica::Ref<SimVar>;
        let mut b: bool;
        (var, b) = tpl.clone();
        oldTpl = (convert(&var)?, b);
        Ok(oldTpl)
    }

    pub(crate) fn isOutputSimVar(mut v: &metamodelica::Ref<SimVar>) -> bool {
        let mut b: bool;
        b = (match v.causality.clone() {
            Some(Causality::OUTPUT) => true,
            _ => false,
        });
        b
    }

    fn parseAttributes(
        mut backendInfo: &metamodelica::Ref<BackendInfo::BackendInfo>,
    ) -> Result<(
        metamodelica::Ref<VariableKind::VariableKind>,
        ArcStr,
        ArcStr,
        Option<metamodelica::Ref<Expression::NFExpression>>,
        Option<metamodelica::Ref<Expression::NFExpression>>,
        Option<metamodelica::Ref<Expression::NFExpression>>,
        Option<metamodelica::Ref<Expression::NFExpression>>,
        bool,
        bool,
        bool,
    )> {
        let mut varKind: metamodelica::Ref<VariableKind::VariableKind> =
            metamodelica::Ref::new(VariableKind::ALGEBRAIC);
        let mut unit: ArcStr = literal!("");
        let mut displayUnit: ArcStr = literal!("");
        let mut min: Option<metamodelica::Ref<Expression::NFExpression>> = None;
        let mut max: Option<metamodelica::Ref<Expression::NFExpression>> = None;
        let mut start: Option<metamodelica::Ref<Expression::NFExpression>> = None;
        let mut nominal: Option<metamodelica::Ref<Expression::NFExpression>> = None;
        let mut isFixed: bool = false;
        let mut isDiscrete: bool = false;
        let mut isProtected: bool = false;
        let () = (::match_deref::match_deref! { match backendInfo {
            Deref @ BackendInfo::BACKEND_INFO { varKind: __esc_varKind, attributes: varAttr @ Deref @ VariableAttributes::VAR_ATTR_REAL { .. }, .. } => {
                varKind = (*__esc_varKind).clone();
                unit = Util::applyOptionOrDefault(Util::applyOption(var_field!((**varAttr).unit, VariableAttributes::VariableAttributes::VAR_ATTR_REAL).clone(), &move |__a0: metamodelica::Ref<Binding::NFBinding>| Binding::getTypedExp(&__a0))?, &move |__a0: metamodelica::Ref<Expression::NFExpression>| -> metamodelica::Result<_> { ::std::result::Result::Ok(Expression::stringValue(&__a0)) }, literal!(""))?;
                displayUnit = Util::applyOptionOrDefault(Util::applyOption(var_field!((**varAttr).displayUnit, VariableAttributes::VariableAttributes::VAR_ATTR_REAL).clone(), &move |__a0: metamodelica::Ref<Binding::NFBinding>| Binding::getTypedExp(&__a0))?, &move |__a0: metamodelica::Ref<Expression::NFExpression>| -> metamodelica::Result<_> { ::std::result::Result::Ok(Expression::stringValue(&__a0)) }, literal!(""))?;
                min = Util::applyOption(var_field!((**varAttr).min, VariableAttributes::VariableAttributes::VAR_ATTR_REAL).clone(), &move |__a0: metamodelica::Ref<Binding::NFBinding>| Binding::getTypedExp(&__a0))?;
                max = Util::applyOption(var_field!((**varAttr).max, VariableAttributes::VariableAttributes::VAR_ATTR_REAL).clone(), &move |__a0: metamodelica::Ref<Binding::NFBinding>| Binding::getTypedExp(&__a0))?;
                start = Util::applyOption(var_field!((**varAttr).start, VariableAttributes::VariableAttributes::VAR_ATTR_REAL).clone(), &move |__a0: metamodelica::Ref<Binding::NFBinding>| Binding::getTypedExp(&__a0))?;
                nominal = Util::applyOption(var_field!((**varAttr).nominal, VariableAttributes::VariableAttributes::VAR_ATTR_REAL).clone(), &move |__a0: metamodelica::Ref<Binding::NFBinding>| Binding::getTypedExp(&__a0))?;
                isFixed = Util::applyOptionOrDefault(Util::applyOption(var_field!((**varAttr).fixed, VariableAttributes::VariableAttributes::VAR_ATTR_REAL).clone(), &move |__a0: metamodelica::Ref<Binding::NFBinding>| Binding::getTypedExp(&__a0))?, &Expression::isAllTrue, false)?;
                isDiscrete = (match &*varKind.clone() {
            VariableKind::DISCRETE => true,
            VariableKind::DISCRETE_STATE => true,
            VariableKind::PREVIOUS => true,
            VariableKind::PARAMETER { .. } => true,
            VariableKind::CONSTANT => true,
            VariableKind::START { .. } => true,
            _ => false,
        });
                isProtected = Util::getOptionOrDefault(var_field!((**varAttr).isProtected, VariableAttributes::VariableAttributes::VAR_ATTR_REAL).clone(), false);
                ()
            },
            Deref @ BackendInfo::BACKEND_INFO { varKind: __esc_varKind, attributes: varAttr @ Deref @ VariableAttributes::VAR_ATTR_INT { .. }, .. } => {
                varKind = (*__esc_varKind).clone();
                min = Util::applyOption(var_field!((**varAttr).min, VariableAttributes::VariableAttributes::VAR_ATTR_INT).clone(), &move |__a0: metamodelica::Ref<Binding::NFBinding>| Binding::getTypedExp(&__a0))?;
                max = Util::applyOption(var_field!((**varAttr).max, VariableAttributes::VariableAttributes::VAR_ATTR_INT).clone(), &move |__a0: metamodelica::Ref<Binding::NFBinding>| Binding::getTypedExp(&__a0))?;
                start = Util::applyOption(var_field!((**varAttr).start, VariableAttributes::VariableAttributes::VAR_ATTR_INT).clone(), &move |__a0: metamodelica::Ref<Binding::NFBinding>| Binding::getTypedExp(&__a0))?;
                isFixed = Util::applyOptionOrDefault(Util::applyOption(var_field!((**varAttr).fixed, VariableAttributes::VariableAttributes::VAR_ATTR_INT).clone(), &move |__a0: metamodelica::Ref<Binding::NFBinding>| Binding::getTypedExp(&__a0))?, &Expression::isAllTrue, false)?;
                isDiscrete = true;
                isProtected = Util::getOptionOrDefault(var_field!((**varAttr).isProtected, VariableAttributes::VariableAttributes::VAR_ATTR_INT).clone(), false);
                ()
            },
            Deref @ BackendInfo::BACKEND_INFO { varKind: __esc_varKind, attributes: varAttr @ Deref @ VariableAttributes::VAR_ATTR_BOOL { .. }, .. } => {
                varKind = (*__esc_varKind).clone();
                start = Util::applyOption(var_field!((**varAttr).start, VariableAttributes::VariableAttributes::VAR_ATTR_BOOL).clone(), &move |__a0: metamodelica::Ref<Binding::NFBinding>| Binding::getTypedExp(&__a0))?;
                isFixed = Util::applyOptionOrDefault(Util::applyOption(var_field!((**varAttr).fixed, VariableAttributes::VariableAttributes::VAR_ATTR_BOOL).clone(), &move |__a0: metamodelica::Ref<Binding::NFBinding>| Binding::getTypedExp(&__a0))?, &Expression::isAllTrue, false)?;
                isDiscrete = true;
                isProtected = Util::getOptionOrDefault(var_field!((**varAttr).isProtected, VariableAttributes::VariableAttributes::VAR_ATTR_BOOL).clone(), false);
                ()
            },
            Deref @ BackendInfo::BACKEND_INFO { varKind: __esc_varKind, attributes: varAttr @ Deref @ VariableAttributes::VAR_ATTR_CLOCK { .. }, .. } => {
                varKind = (*__esc_varKind).clone();
                isDiscrete = true;
                isProtected = Util::getOptionOrDefault(var_field!((**varAttr).isProtected, VariableAttributes::VariableAttributes::VAR_ATTR_CLOCK).clone(), false);
                ()
            },
            Deref @ BackendInfo::BACKEND_INFO { varKind: __esc_varKind, attributes: varAttr @ Deref @ VariableAttributes::VAR_ATTR_STRING { .. }, .. } => {
                varKind = (*__esc_varKind).clone();
                start = Util::applyOption(var_field!((**varAttr).start, VariableAttributes::VariableAttributes::VAR_ATTR_STRING).clone(), &move |__a0: metamodelica::Ref<Binding::NFBinding>| Binding::getTypedExp(&__a0))?;
                isFixed = Util::applyOptionOrDefault(Util::applyOption(var_field!((**varAttr).fixed, VariableAttributes::VariableAttributes::VAR_ATTR_STRING).clone(), &move |__a0: metamodelica::Ref<Binding::NFBinding>| Binding::getTypedExp(&__a0))?, &Expression::isAllTrue, false)?;
                isDiscrete = true;
                isProtected = Util::getOptionOrDefault(var_field!((**varAttr).isProtected, VariableAttributes::VariableAttributes::VAR_ATTR_STRING).clone(), false);
                ()
            },
            Deref @ BackendInfo::BACKEND_INFO { varKind: __esc_varKind, attributes: varAttr @ Deref @ VariableAttributes::VAR_ATTR_ENUMERATION { .. }, .. } => {
                varKind = (*__esc_varKind).clone();
                min = Util::applyOption(var_field!((**varAttr).min, VariableAttributes::VariableAttributes::VAR_ATTR_ENUMERATION).clone(), &move |__a0: metamodelica::Ref<Binding::NFBinding>| Binding::getTypedExp(&__a0))?;
                max = Util::applyOption(var_field!((**varAttr).max, VariableAttributes::VariableAttributes::VAR_ATTR_ENUMERATION).clone(), &move |__a0: metamodelica::Ref<Binding::NFBinding>| Binding::getTypedExp(&__a0))?;
                start = Util::applyOption(var_field!((**varAttr).start, VariableAttributes::VariableAttributes::VAR_ATTR_ENUMERATION).clone(), &move |__a0: metamodelica::Ref<Binding::NFBinding>| Binding::getTypedExp(&__a0))?;
                isFixed = Util::applyOptionOrDefault(Util::applyOption(var_field!((**varAttr).fixed, VariableAttributes::VariableAttributes::VAR_ATTR_ENUMERATION).clone(), &move |__a0: metamodelica::Ref<Binding::NFBinding>| Binding::getTypedExp(&__a0))?, &Expression::isAllTrue, false)?;
                isDiscrete = true;
                isProtected = Util::getOptionOrDefault(var_field!((**varAttr).isProtected, VariableAttributes::VariableAttributes::VAR_ATTR_ENUMERATION).clone(), false);
                ()
            },
            _ => {
                Error::addMessage(Error::INTERNAL_ERROR.clone(), list![{ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NSimVar.SimVar.parseAttributes")); __mm_s.push_str(&*literal!(" failed because the BackendInfo could not be parsed:\n")); __mm_s.push_str(&*BackendInfo::toString(backendInfo)?); ArcStr::from(__mm_s) }])?;
                return Err("fail")
            },
            _ => unreachable!("match_deref! exhaustiveness placeholder"),
        } });
        Ok((
            varKind,
            unit,
            displayUnit,
            min,
            max,
            start,
            nominal,
            isFixed,
            isDiscrete,
            isProtected,
        ))
    }

    fn parseComment(mut absynComment: &metamodelica::Ref<SCode::Comment>) -> ArcStr {
        let mut commentStr: ArcStr;
        commentStr = (match &**absynComment {
            SCode::Comment {
                comment: Some(__esc_commentStr),
                ..
            } => {
                commentStr = (*__esc_commentStr).clone();
                commentStr.clone()
            }
            _ => literal!(""),
        });
        commentStr
    }

    fn parseBinding(
        mut start: Option<metamodelica::Ref<Expression::NFExpression>>,
        mut var: &metamodelica::Ref<Variable::NFVariable>,
    ) -> (Option<metamodelica::Ref<Expression::NFExpression>>, bool, Causality) {
        let mut start: Option<metamodelica::Ref<Expression::NFExpression>> = start;
        let mut isValueChangeable: bool;
        let mut causality: Causality;
        (start, isValueChangeable, causality) = (::match_deref::match_deref! { match var {
            Deref @ Variable::VARIABLE { binding: Deref @ Binding::TYPED_BINDING { variability: Prefixes::Variability::CONSTANT, bindingExp, .. }, backendinfo: Deref @ BackendInfo::BACKEND_INFO { varKind: Deref @ VariableKind::PARAMETER { .. }, .. }, .. } => {
                (Some(bindingExp.clone()), true, Causality::PARAMETER.clone())
            },
            Deref @ Variable::VARIABLE { binding: Deref @ Binding::FLAT_BINDING { variability: Prefixes::Variability::CONSTANT, bindingExp, .. }, backendinfo: Deref @ BackendInfo::BACKEND_INFO { varKind: Deref @ VariableKind::PARAMETER { .. }, .. }, .. } => {
                (Some(bindingExp.clone()), true, Causality::PARAMETER.clone())
            },
            Deref @ Variable::VARIABLE { backendinfo: Deref @ BackendInfo::BACKEND_INFO { varKind: Deref @ VariableKind::PARAMETER { .. }, .. }, .. } => {
                (start, false, Causality::CALCULATED_PARAMETER.clone())
            },
            _ if (Variable::isInput(var)) => {
                (start, true, Causality::INPUT.clone())
            },
            _ if (Variable::isOutput(var)) => {
                (start, false, Causality::OUTPUT.clone())
            },
            _ => {
                (start, false, Causality::LOCAL.clone())
            },
            _ => unreachable!("match_deref! exhaustiveness placeholder"),
        } });
        (start, isValueChangeable, causality)
    }

    fn convertVarKind(mut varKind: &metamodelica::Ref<VariableKind::VariableKind>) -> Result<OldBackendDAE::VarKind> {
        let mut oldVarKind: OldBackendDAE::VarKind;
        oldVarKind = (match &**varKind {
            VariableKind::ALGEBRAIC => openmodelica_backend_types::BackendDAE::VarKind::VARIABLE,
            VariableKind::STATE {
                derivative: __varKind_derivative,
                index: __varKind_index,
                natural: __varKind_natural,
            } => {
                let mut var: metamodelica::Ref<Variable::NFVariable>;
                let mut oldCrefOpt: Option<metamodelica::Ref<DAE::ComponentRef>>;
                if (__varKind_derivative).is_some() {
                    var = Pointer::access(PointerWeak::upgrade(Util::getOption(__varKind_derivative.clone())?)?);
                    oldCrefOpt = Some(ComponentRef::toDAE(&var.name)?);
                } else {
                    oldCrefOpt = None;
                }
                OldBackendDAE::VarKind::STATE {
                    index: __varKind_index.clone(),
                    derName: oldCrefOpt,
                    natural: __varKind_natural.clone(),
                }
            }
            VariableKind::STATE_DER { .. } => openmodelica_backend_types::BackendDAE::VarKind::STATE_DER,
            VariableKind::DUMMY_DER { .. } => openmodelica_backend_types::BackendDAE::VarKind::DUMMY_DER,
            VariableKind::DUMMY_STATE { .. } => openmodelica_backend_types::BackendDAE::VarKind::DUMMY_STATE,
            VariableKind::DISCRETE => openmodelica_backend_types::BackendDAE::VarKind::DISCRETE,
            VariableKind::DISCRETE_STATE => openmodelica_backend_types::BackendDAE::VarKind::DISCRETE,
            VariableKind::CLOCKED => openmodelica_backend_types::BackendDAE::VarKind::DISCRETE,
            VariableKind::PREVIOUS => openmodelica_backend_types::BackendDAE::VarKind::DISCRETE,
            VariableKind::PARAMETER { .. } => openmodelica_backend_types::BackendDAE::VarKind::PARAM,
            VariableKind::CONSTANT => openmodelica_backend_types::BackendDAE::VarKind::CONST,
            VariableKind::START { .. } => openmodelica_backend_types::BackendDAE::VarKind::VARIABLE,
            VariableKind::EXTOBJ {
                fullClassName: __varKind_fullClassName,
            } => OldBackendDAE::VarKind::EXTOBJ {
                fullClassName: __varKind_fullClassName.clone(),
            },
            VariableKind::JAC_VAR => openmodelica_backend_types::BackendDAE::VarKind::JAC_VAR,
            VariableKind::JAC_TMP_VAR => openmodelica_backend_types::BackendDAE::VarKind::JAC_TMP_VAR,
            VariableKind::SEED_VAR => openmodelica_backend_types::BackendDAE::VarKind::SEED_VAR,
            VariableKind::OPT_CONSTR => openmodelica_backend_types::BackendDAE::VarKind::OPT_CONSTR,
            VariableKind::OPT_FCONSTR => openmodelica_backend_types::BackendDAE::VarKind::OPT_FCONSTR,
            VariableKind::OPT_INPUT_WITH_DER => openmodelica_backend_types::BackendDAE::VarKind::OPT_INPUT_WITH_DER,
            VariableKind::OPT_INPUT_DER => openmodelica_backend_types::BackendDAE::VarKind::OPT_INPUT_DER,
            VariableKind::OPT_TGRID => openmodelica_backend_types::BackendDAE::VarKind::OPT_TGRID,
            VariableKind::OPT_LOOP_INPUT {
                replaceCref: __varKind_replaceCref,
            } => OldBackendDAE::VarKind::OPT_LOOP_INPUT {
                replaceExp: ComponentRef::toDAE(metamodelica::AsArg::as_arg(&__varKind_replaceCref))?,
            },
            VariableKind::ALG_STATE => openmodelica_backend_types::BackendDAE::VarKind::ALG_STATE,
            VariableKind::ALG_STATE_OLD => openmodelica_backend_types::BackendDAE::VarKind::ALG_STATE_OLD,
            VariableKind::RESIDUAL_VAR => openmodelica_backend_types::BackendDAE::VarKind::DAE_RESIDUAL_VAR,
            VariableKind::DAE_AUX_VAR => openmodelica_backend_types::BackendDAE::VarKind::DAE_AUX_VAR,
            VariableKind::LOOP_ITERATION => openmodelica_backend_types::BackendDAE::VarKind::LOOP_ITERATION,
            VariableKind::LOOP_SOLVED => openmodelica_backend_types::BackendDAE::VarKind::LOOP_SOLVED,
            VariableKind::FRONTEND_DUMMY => {
                Error::addMessage(
                    Error::INTERNAL_ERROR.clone(),
                    list![{
                        let mut __mm_s = String::new();
                        __mm_s.push_str(&*literal!("NSimVar.SimVar.convertVarKind"));
                        __mm_s.push_str(&*literal!(" failed because of wrong VariableKind FRONTEND_DUMMY(). This should not exist after frontend."));
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
                        __mm_s.push_str(&*literal!("NSimVar.SimVar.convertVarKind"));
                        __mm_s.push_str(&*literal!(" failed because of unhandled VariableKind "));
                        __mm_s.push_str(&*VariableKind::toString(varKind));
                        __mm_s.push_str(&*literal!("."));
                        ArcStr::from(__mm_s)
                    }],
                )?;
                return Err("fail");
            }
        });
        Ok(oldVarKind)
    }
}

pub mod Alias {
    use super::*;
    #[derive(Clone, Debug, Eq, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
    pub(crate) enum Alias {
        NO_ALIAS,
        /// General alias expression with a coefficent.
        ///      var := gain * alias + offset
        ALIAS {
            /// The name of the alias variable.
            alias: metamodelica::Ref<ComponentRef::NFComponentRef>,
            /// = 1 for regular alias.
            gain: metamodelica::Real,
            /// = 0 for regular alias.
            offset: metamodelica::Real,
        },
    }
    impl metamodelica::gc::MMTrace for Alias {
        fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
            match self {
                Alias::NO_ALIAS => Ok(()),
                Alias::ALIAS { alias, gain, offset } => {
                    metamodelica::gc::MMTrace::mm_accept(alias, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(gain, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(offset, __mmv)?;
                    Ok(())
                }
            }
        }
    }
    impl Alias {
        pub fn interned_NO_ALIAS() -> metamodelica::Ref<Alias> {
            thread_local! {
                static INTERNED: metamodelica::Ref<Alias> = metamodelica::Ref::new(Alias::NO_ALIAS);
            }
            INTERNED.with(|i| i.clone())
        }
    }
    pub fn interned_NO_ALIAS() -> metamodelica::Ref<Alias> {
        Alias::interned_NO_ALIAS()
    }
    impl Default for Alias {
        fn default() -> Self {
            Self::NO_ALIAS
        }
    }
    pub(crate) use self::Alias::{ALIAS, NO_ALIAS};
    pub(crate) fn fromBinding(mut binding: &metamodelica::Ref<Binding::NFBinding>) -> Result<metamodelica::Ref<Alias>> {
        let mut alias: metamodelica::Ref<Alias>;
        alias = (match &**binding {
            Binding::TYPED_BINDING {
                bindingExp: __binding_bindingExp,
                ..
            } => getAlias(__binding_bindingExp.clone())?,
            Binding::FLAT_BINDING {
                bindingExp: __binding_bindingExp,
                ..
            } => getAlias(__binding_bindingExp.clone())?,
            _ => crate::NSimVar::Alias::interned_NO_ALIAS(),
        });
        Ok(alias)
    }

    pub(crate) fn toString(mut alias: &metamodelica::Ref<Alias>) -> Result<ArcStr> {
        let mut r#str: ArcStr;
        r#str = (match &**alias {
            NO_ALIAS { .. } => {
                literal!("(no alias)")
            }
            ALIAS {
                alias: __alias_alias,
                gain: __alias_gain,
                offset: __alias_offset,
            } => {
                let mut gainStr: ArcStr;
                let mut offsetStr: ArcStr;
                gainStr = if (__alias_gain.clone() == metamodelica::OrderedFloat(1.0_f64)) {
                    literal!("")
                } else {
                    {
                        let mut __mm_s = String::new();
                        __mm_s.push_str(&*realString(__alias_gain.clone()));
                        __mm_s.push_str(&*literal!("*"));
                        ArcStr::from(__mm_s)
                    }
                };
                offsetStr = if (__alias_offset.clone() == metamodelica::OrderedFloat(0.0_f64)) {
                    literal!("")
                } else {
                    {
                        let mut __mm_s = String::new();
                        __mm_s.push_str(&*literal!("+"));
                        __mm_s.push_str(&*realString(__alias_offset.clone()));
                        ArcStr::from(__mm_s)
                    }
                };
                {
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!("(bound alias: "));
                    __mm_s.push_str(&*gainStr);
                    __mm_s.push_str(&*ComponentRef::toString(metamodelica::AsArg::as_arg(&__alias_alias))?);
                    __mm_s.push_str(&*offsetStr);
                    __mm_s.push_str(&*literal!(")"));
                    ArcStr::from(__mm_s)
                }
            }
        });
        Ok(r#str)
    }

    pub(crate) fn convert(mut alias: &metamodelica::Ref<Alias>) -> Result<OldSimCodeVar::AliasVariable> {
        let mut oldAlias: OldSimCodeVar::AliasVariable;
        oldAlias = (match &**alias {
            NO_ALIAS { .. } => openmodelica_simcode_types::SimCodeVar::AliasVariable::NOALIAS,
            ALIAS {
                alias: __alias_alias,
                gain: __alias_gain,
                offset: __alias_offset,
            } if (realEq(__alias_gain.clone(), metamodelica::OrderedFloat(1.0_f64))
                && realEq(__alias_offset.clone(), metamodelica::OrderedFloat(0.0_f64))) =>
            {
                OldSimCodeVar::AliasVariable::ALIAS {
                    varName: ComponentRef::toDAE(metamodelica::AsArg::as_arg(&__alias_alias))?,
                }
            }
            ALIAS {
                alias: __alias_alias,
                gain: __alias_gain,
                offset: __alias_offset,
            } if (realEq(__alias_gain.clone(), metamodelica::OrderedFloat(-1.0_f64))
                && realEq(__alias_offset.clone(), metamodelica::OrderedFloat(0.0_f64))) =>
            {
                OldSimCodeVar::AliasVariable::NEGATEDALIAS {
                    varName: ComponentRef::toDAE(metamodelica::AsArg::as_arg(&__alias_alias))?,
                }
            }
            _ => {
                Error::addMessage(
                    Error::INTERNAL_ERROR.clone(),
                    list![{
                        let mut __mm_s = String::new();
                        __mm_s.push_str(&*literal!("NSimVar.Alias.convert"));
                        __mm_s.push_str(&*literal!(" failed because of unknown Alias type."));
                        ArcStr::from(__mm_s)
                    }],
                )?;
                return Err("fail");
            }
        });
        Ok(oldAlias)
    }

    fn getAlias(mut exp: metamodelica::Ref<Expression::NFExpression>) -> Result<metamodelica::Ref<Alias>> {
        let mut alias: metamodelica::Ref<Alias>;
        alias = (::match_deref::match_deref! { match &(SimplifyExp::simplify(exp, false)?) {
            e @ Deref @ Expression::CREF { .. } => {
                metamodelica::Ref::new(Alias::ALIAS { alias: var_field!((**e).cref, Expression::NFExpression::CREF).clone(), gain: metamodelica::OrderedFloat(1.0_f64), offset: metamodelica::OrderedFloat(0.0_f64) })
            },
            Deref @ Expression::UNARY { exp: e @ Deref @ Expression::CREF { .. }, .. } => {
                metamodelica::Ref::new(Alias::ALIAS { alias: var_field!((**e).cref, Expression::NFExpression::CREF).clone(), gain: metamodelica::OrderedFloat(-1.0_f64), offset: metamodelica::OrderedFloat(0.0_f64) })
            },
            Deref @ Expression::LUNARY { exp: e @ Deref @ Expression::CREF { .. }, .. } => {
                metamodelica::Ref::new(Alias::ALIAS { alias: var_field!((**e).cref, Expression::NFExpression::CREF).clone(), gain: metamodelica::OrderedFloat(-1.0_f64), offset: metamodelica::OrderedFloat(0.0_f64) })
            },
            _ => {
                crate::NSimVar::Alias::interned_NO_ALIAS()
            },
            _ => unreachable!("match_deref! exhaustiveness placeholder"),
        } });
        Ok(alias)
    }

    fn getGainAlias(
        mut e1: metamodelica::Ref<Expression::NFExpression>,
        mut e2: metamodelica::Ref<Expression::NFExpression>,
    ) -> Result<(metamodelica::Ref<ComponentRef::NFComponentRef>, metamodelica::Real)> {
        let mut cref: metamodelica::Ref<ComponentRef::NFComponentRef>;
        let mut gain: metamodelica::Real;
        (cref, gain) = (::match_deref::match_deref! { match &((&*e1, &*e2)) {
            (Deref @ Expression::CREF { .. }, _) if (Expression::isConstNumber(&e2)) => (var_field!((*e1).cref, Expression::NFExpression::CREF).clone(), Expression::realValue(&e2)?),
            (_, Deref @ Expression::CREF { .. }) if (Expression::isConstNumber(&e1)) => (var_field!((*e2).cref, Expression::NFExpression::CREF).clone(), Expression::realValue(&e1)?),
            _ => {
                Error::addInternalError({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NSimVar.Alias.getGainAlias")); __mm_s.push_str(&*literal!(" cannot generate gain alias from Expressions: {")); __mm_s.push_str(&*Expression::toString(e1.clone())?); __mm_s.push_str(&*literal!(", ")); __mm_s.push_str(&*Expression::toString(e2.clone())?); __mm_s.push_str(&*literal!("}")); ArcStr::from(__mm_s) }, metamodelica::sourceInfo!("NSimCode/NSimVar.mo"))?;
                return Err("fail")
            },
            _ => unreachable!("match_deref! exhaustiveness placeholder"),
        } });
        Ok((cref, gain))
    }

    fn getOffsetAlias(
        mut e1: metamodelica::Ref<Expression::NFExpression>,
        mut e2: metamodelica::Ref<Expression::NFExpression>,
    ) -> Result<(
        metamodelica::Ref<ComponentRef::NFComponentRef>,
        metamodelica::Real,
        metamodelica::Real,
    )> {
        let mut cref: metamodelica::Ref<ComponentRef::NFComponentRef>;
        let mut gain: metamodelica::Real;
        let mut offset: metamodelica::Real;
        (cref, gain, offset) = (::match_deref::match_deref! { match &((&*e1, &*e2)) {
            (Deref @ Expression::CREF { .. }, _) if (Expression::isConstNumber(&e2)) => {
                (var_field!((*e1).cref, Expression::NFExpression::CREF).clone(), metamodelica::OrderedFloat(0.0_f64), Expression::realValue(&e2)?)
            },
            (_, Deref @ Expression::CREF { .. }) if (Expression::isConstNumber(&e1)) => {
                (var_field!((*e2).cref, Expression::NFExpression::CREF).clone(), metamodelica::OrderedFloat(0.0_f64), Expression::realValue(&e1)?)
            },
            (Deref @ Expression::MULTARY { arguments: Deref @ metamodelica::ListNode::Cons { head: arg1, tail: Deref @ metamodelica::ListNode::Cons { head: arg2, tail: Deref @ metamodelica::ListNode::Nil } }, inv_arguments: Deref @ metamodelica::ListNode::Nil, .. }, _) if (Operator::getMathClassification(var_field!((*e1).operator, Expression::NFExpression::MULTARY))? == Operator::MathClassification::MULTIPLICATION.clone() && Expression::isConstNumber(&e2)) => {
                (cref, gain) = getGainAlias(arg1.clone(), arg2.clone())?;
                (cref, gain, Expression::realValue(&e2)?)
            },
            (_, Deref @ Expression::MULTARY { arguments: Deref @ metamodelica::ListNode::Cons { head: arg1, tail: Deref @ metamodelica::ListNode::Cons { head: arg2, tail: Deref @ metamodelica::ListNode::Nil } }, inv_arguments: Deref @ metamodelica::ListNode::Nil, .. }) if (Operator::getMathClassification(var_field!((*e2).operator, Expression::NFExpression::MULTARY))? == Operator::MathClassification::MULTIPLICATION.clone() && Expression::isConstNumber(&e1)) => {
                (cref, gain) = getGainAlias(arg1.clone(), arg2.clone())?;
                (cref, gain, Expression::realValue(&e1)?)
            },
            _ => {
                Error::addInternalError({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NSimVar.Alias.getOffsetAlias")); __mm_s.push_str(&*literal!(" cannot generate offset alias from Expressions: {")); __mm_s.push_str(&*Expression::toString(e1.clone())?); __mm_s.push_str(&*literal!(", ")); __mm_s.push_str(&*Expression::toString(e2.clone())?); __mm_s.push_str(&*literal!("}")); ArcStr::from(__mm_s) }, metamodelica::sourceInfo!("NSimCode/NSimVar.mo"))?;
                return Err("fail")
            },
            _ => unreachable!("match_deref! exhaustiveness placeholder"),
        } });
        Ok((cref, gain, offset))
    }
}

// kabdelhak: i don't like "CALCULATED_PARAMETER", is there a better way to describe it?
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, metamodelica::ReferenceEq)]
#[repr(i32)]
pub(crate) enum Causality {
    NONE = 1,
    OUTPUT = 2,
    INPUT = 3,
    LOCAL = 4,
    PARAMETER = 5,
    CALCULATED_PARAMETER = 6,
}
impl PartialOrd for Causality {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for Causality {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (*self as i32).cmp(&(*other as i32))
    }
}
impl metamodelica::gc::MMTrace for Causality {
    fn mm_accept(&self, _: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        Ok(())
    }
}
impl Default for Causality {
    fn default() -> Self {
        Self::NONE
    }
}

// kabdelhak: where is the difference between approx and calculated?
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, metamodelica::ReferenceEq)]
#[repr(i32)]
pub(crate) enum Initial {
    NONE = 1,
    EXACT = 2,
    APPROX = 3,
    CALCULATED = 4,
}
impl PartialOrd for Initial {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for Initial {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (*self as i32).cmp(&(*other as i32))
    }
}
impl metamodelica::gc::MMTrace for Initial {
    fn mm_accept(&self, _: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        Ok(())
    }
}
impl Default for Initial {
    fn default() -> Self {
        Self::NONE
    }
}

// kabdelhak: i don't like "TUNABLE" -> just "VARIABLE"?
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, metamodelica::ReferenceEq)]
#[repr(i32)]
pub(crate) enum Variability {
    CONSTANT = 1,
    FIXED = 2,
    TUNABLE = 3,
    DISCRETE = 4,
    CONTINUOUS = 5,
}
impl PartialOrd for Variability {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for Variability {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (*self as i32).cmp(&(*other as i32))
    }
}
impl metamodelica::gc::MMTrace for Variability {
    fn mm_accept(&self, _: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        Ok(())
    }
}
impl Default for Variability {
    fn default() -> Self {
        Self::CONSTANT
    }
}

pub mod SimVars {
    use super::*;
    /// Container for metadata about variables in a Modelica model.
    #[derive(Clone, Debug, Eq, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
    pub struct SimVars {
        pub stateVars: metamodelica::List<metamodelica::Ref<SimVar::SimVar>>,
        pub derivativeVars: metamodelica::List<metamodelica::Ref<SimVar::SimVar>>,
        pub algVars: metamodelica::List<metamodelica::Ref<SimVar::SimVar>>,
        pub discreteAlgVars: metamodelica::List<metamodelica::Ref<SimVar::SimVar>>,
        pub intAlgVars: metamodelica::List<metamodelica::Ref<SimVar::SimVar>>,
        pub boolAlgVars: metamodelica::List<metamodelica::Ref<SimVar::SimVar>>,
        pub stringAlgVars: metamodelica::List<metamodelica::Ref<SimVar::SimVar>>,
        pub enumAlgVars: metamodelica::List<metamodelica::Ref<SimVar::SimVar>>,
        pub inputVars: metamodelica::List<metamodelica::Ref<SimVar::SimVar>>,
        pub outputVars: metamodelica::List<metamodelica::Ref<SimVar::SimVar>>,
        pub aliasVars: metamodelica::List<metamodelica::Ref<SimVar::SimVar>>,
        pub intAliasVars: metamodelica::List<metamodelica::Ref<SimVar::SimVar>>,
        pub boolAliasVars: metamodelica::List<metamodelica::Ref<SimVar::SimVar>>,
        pub stringAliasVars: metamodelica::List<metamodelica::Ref<SimVar::SimVar>>,
        pub enumAliasVars: metamodelica::List<metamodelica::Ref<SimVar::SimVar>>,
        pub paramVars: metamodelica::List<metamodelica::Ref<SimVar::SimVar>>,
        pub intParamVars: metamodelica::List<metamodelica::Ref<SimVar::SimVar>>,
        pub boolParamVars: metamodelica::List<metamodelica::Ref<SimVar::SimVar>>,
        pub stringParamVars: metamodelica::List<metamodelica::Ref<SimVar::SimVar>>,
        pub enumParamVars: metamodelica::List<metamodelica::Ref<SimVar::SimVar>>,
        pub extObjVars: metamodelica::List<metamodelica::Ref<SimVar::SimVar>>,
        pub constVars: metamodelica::List<metamodelica::Ref<SimVar::SimVar>>,
        pub intConstVars: metamodelica::List<metamodelica::Ref<SimVar::SimVar>>,
        pub boolConstVars: metamodelica::List<metamodelica::Ref<SimVar::SimVar>>,
        pub stringConstVars: metamodelica::List<metamodelica::Ref<SimVar::SimVar>>,
        pub enumConstVars: metamodelica::List<metamodelica::Ref<SimVar::SimVar>>,
        pub residualVars: metamodelica::List<metamodelica::Ref<SimVar::SimVar>>,
        pub jacobianVars: metamodelica::List<metamodelica::Ref<SimVar::SimVar>>,
        pub seedVars: metamodelica::List<metamodelica::Ref<SimVar::SimVar>>,
        pub realOptimizeConstraintsVars: metamodelica::List<metamodelica::Ref<SimVar::SimVar>>,
        pub realOptimizeFinalConstraintsVars: metamodelica::List<metamodelica::Ref<SimVar::SimVar>>,
        /// variable used to calculate sensitivities for parameters nSensitivitityParameters + nRealParam*nStates
        pub sensitivityVars: metamodelica::List<metamodelica::Ref<SimVar::SimVar>>,
        pub dataReconSetcVars: metamodelica::List<metamodelica::Ref<SimVar::SimVar>>,
        pub dataReconinputVars: metamodelica::List<metamodelica::Ref<SimVar::SimVar>>,
        pub dataReconSetBVars: metamodelica::List<metamodelica::Ref<SimVar::SimVar>>,
    }

    impl metamodelica::gc::MMTrace for SimVars {
        fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
            metamodelica::gc::MMTrace::mm_accept(&self.stateVars, __mmv)?;
            metamodelica::gc::MMTrace::mm_accept(&self.derivativeVars, __mmv)?;
            metamodelica::gc::MMTrace::mm_accept(&self.algVars, __mmv)?;
            metamodelica::gc::MMTrace::mm_accept(&self.discreteAlgVars, __mmv)?;
            metamodelica::gc::MMTrace::mm_accept(&self.intAlgVars, __mmv)?;
            metamodelica::gc::MMTrace::mm_accept(&self.boolAlgVars, __mmv)?;
            metamodelica::gc::MMTrace::mm_accept(&self.stringAlgVars, __mmv)?;
            metamodelica::gc::MMTrace::mm_accept(&self.enumAlgVars, __mmv)?;
            metamodelica::gc::MMTrace::mm_accept(&self.inputVars, __mmv)?;
            metamodelica::gc::MMTrace::mm_accept(&self.outputVars, __mmv)?;
            metamodelica::gc::MMTrace::mm_accept(&self.aliasVars, __mmv)?;
            metamodelica::gc::MMTrace::mm_accept(&self.intAliasVars, __mmv)?;
            metamodelica::gc::MMTrace::mm_accept(&self.boolAliasVars, __mmv)?;
            metamodelica::gc::MMTrace::mm_accept(&self.stringAliasVars, __mmv)?;
            metamodelica::gc::MMTrace::mm_accept(&self.enumAliasVars, __mmv)?;
            metamodelica::gc::MMTrace::mm_accept(&self.paramVars, __mmv)?;
            metamodelica::gc::MMTrace::mm_accept(&self.intParamVars, __mmv)?;
            metamodelica::gc::MMTrace::mm_accept(&self.boolParamVars, __mmv)?;
            metamodelica::gc::MMTrace::mm_accept(&self.stringParamVars, __mmv)?;
            metamodelica::gc::MMTrace::mm_accept(&self.enumParamVars, __mmv)?;
            metamodelica::gc::MMTrace::mm_accept(&self.extObjVars, __mmv)?;
            metamodelica::gc::MMTrace::mm_accept(&self.constVars, __mmv)?;
            metamodelica::gc::MMTrace::mm_accept(&self.intConstVars, __mmv)?;
            metamodelica::gc::MMTrace::mm_accept(&self.boolConstVars, __mmv)?;
            metamodelica::gc::MMTrace::mm_accept(&self.stringConstVars, __mmv)?;
            metamodelica::gc::MMTrace::mm_accept(&self.enumConstVars, __mmv)?;
            metamodelica::gc::MMTrace::mm_accept(&self.residualVars, __mmv)?;
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
                stringAlgVars: Default::default(),
                enumAlgVars: Default::default(),
                inputVars: Default::default(),
                outputVars: Default::default(),
                aliasVars: Default::default(),
                intAliasVars: Default::default(),
                boolAliasVars: Default::default(),
                stringAliasVars: Default::default(),
                enumAliasVars: Default::default(),
                paramVars: Default::default(),
                intParamVars: Default::default(),
                boolParamVars: Default::default(),
                stringParamVars: Default::default(),
                enumParamVars: Default::default(),
                extObjVars: Default::default(),
                constVars: Default::default(),
                intConstVars: Default::default(),
                boolConstVars: Default::default(),
                stringConstVars: Default::default(),
                enumConstVars: Default::default(),
                residualVars: Default::default(),
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

    pub(crate) fn toString(mut vars: &metamodelica::Ref<SimVars>, mut r#str: ArcStr) -> Result<ArcStr> {
        let mut r#str: ArcStr = r#str;
        r#str = StringUtil::headline_2(
            &({
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("SimVars "));
                __mm_s.push_str(&*r#str);
                ArcStr::from(__mm_s)
            }),
        )?;
        r#str = {
            let mut __mm_s = String::new();
            __mm_s.push_str(&*r#str);
            __mm_s.push_str(&*SimVar::listToString(&vars.stateVars, literal!("States"), false)?);
            ArcStr::from(__mm_s)
        };
        r#str = {
            let mut __mm_s = String::new();
            __mm_s.push_str(&*r#str);
            __mm_s.push_str(&*SimVar::listToString(
                &vars.derivativeVars,
                literal!("Derivatives"),
                false,
            )?);
            ArcStr::from(__mm_s)
        };
        r#str = {
            let mut __mm_s = String::new();
            __mm_s.push_str(&*r#str);
            __mm_s.push_str(&*SimVar::listToString(
                &vars.algVars,
                literal!("Algebraic Variables"),
                false,
            )?);
            ArcStr::from(__mm_s)
        };
        r#str = {
            let mut __mm_s = String::new();
            __mm_s.push_str(&*r#str);
            __mm_s.push_str(&*SimVar::listToString(
                &vars.discreteAlgVars,
                literal!("Discrete Algebraic Variables"),
                false,
            )?);
            ArcStr::from(__mm_s)
        };
        r#str = {
            let mut __mm_s = String::new();
            __mm_s.push_str(&*r#str);
            __mm_s.push_str(&*SimVar::listToString(
                &vars.intAlgVars,
                literal!("Integer Algebraic Variables"),
                false,
            )?);
            ArcStr::from(__mm_s)
        };
        r#str = {
            let mut __mm_s = String::new();
            __mm_s.push_str(&*r#str);
            __mm_s.push_str(&*SimVar::listToString(
                &vars.boolAlgVars,
                literal!("Boolean Algebraic Variables"),
                false,
            )?);
            ArcStr::from(__mm_s)
        };
        r#str = {
            let mut __mm_s = String::new();
            __mm_s.push_str(&*r#str);
            __mm_s.push_str(&*SimVar::listToString(
                &vars.paramVars,
                literal!("Real Parameters"),
                false,
            )?);
            ArcStr::from(__mm_s)
        };
        r#str = {
            let mut __mm_s = String::new();
            __mm_s.push_str(&*r#str);
            __mm_s.push_str(&*SimVar::listToString(
                &vars.intParamVars,
                literal!("Integer Parameters"),
                false,
            )?);
            ArcStr::from(__mm_s)
        };
        r#str = {
            let mut __mm_s = String::new();
            __mm_s.push_str(&*r#str);
            __mm_s.push_str(&*SimVar::listToString(
                &vars.boolParamVars,
                literal!("Boolean Parameters"),
                false,
            )?);
            ArcStr::from(__mm_s)
        };
        r#str = {
            let mut __mm_s = String::new();
            __mm_s.push_str(&*r#str);
            __mm_s.push_str(&*SimVar::listToString(
                &vars.residualVars,
                literal!("Residual Variables"),
                false,
            )?);
            ArcStr::from(__mm_s)
        };
        r#str = {
            let mut __mm_s = String::new();
            __mm_s.push_str(&*r#str);
            __mm_s.push_str(&*SimVar::listToString(&vars.aliasVars, literal!("Real Alias"), true)?);
            ArcStr::from(__mm_s)
        };
        Ok(r#str)
    }

    pub(crate) fn create(
        mut varData: &metamodelica::Ref<BVariable::VarData::VarData>,
        mut residual_vars: metamodelica::Ref<VariablePointers::VariablePointers>,
        mut simCodeIndices: SimCodeIndices,
    ) -> Result<(metamodelica::Ref<SimVars>, SimCodeIndices)> {
        let mut simVars: metamodelica::Ref<SimVars>;
        let mut simCodeIndices: SimCodeIndices = simCodeIndices;
        let mut stateVars: metamodelica::List<metamodelica::Ref<SimVar::SimVar>> = metamodelica::nil();
        let mut derivativeVars: metamodelica::List<metamodelica::Ref<SimVar::SimVar>> = metamodelica::nil();
        let mut algVars: metamodelica::List<metamodelica::Ref<SimVar::SimVar>> = metamodelica::nil();
        let mut nonTrivialAlias: metamodelica::List<metamodelica::Ref<SimVar::SimVar>> = metamodelica::nil();
        let mut discreteAlgVars: metamodelica::List<metamodelica::Ref<SimVar::SimVar>> = metamodelica::nil();
        let mut intAlgVars: metamodelica::List<metamodelica::Ref<SimVar::SimVar>> = metamodelica::nil();
        let mut boolAlgVars: metamodelica::List<metamodelica::Ref<SimVar::SimVar>> = metamodelica::nil();
        let mut stringAlgVars: metamodelica::List<metamodelica::Ref<SimVar::SimVar>> = metamodelica::nil();
        let mut enumAlgVars: metamodelica::List<metamodelica::Ref<SimVar::SimVar>> = metamodelica::nil();
        let mut discreteAlgVars2: metamodelica::List<metamodelica::Ref<SimVar::SimVar>> = metamodelica::nil();
        let mut intAlgVars2: metamodelica::List<metamodelica::Ref<SimVar::SimVar>> = metamodelica::nil();
        let mut boolAlgVars2: metamodelica::List<metamodelica::Ref<SimVar::SimVar>> = metamodelica::nil();
        let mut stringAlgVars2: metamodelica::List<metamodelica::Ref<SimVar::SimVar>> = metamodelica::nil();
        let mut enumAlgVars2: metamodelica::List<metamodelica::Ref<SimVar::SimVar>> = metamodelica::nil();
        let mut discreteAlgVars3: metamodelica::List<metamodelica::Ref<SimVar::SimVar>> = metamodelica::nil();
        let mut intAlgVars3: metamodelica::List<metamodelica::Ref<SimVar::SimVar>> = metamodelica::nil();
        let mut boolAlgVars3: metamodelica::List<metamodelica::Ref<SimVar::SimVar>> = metamodelica::nil();
        let mut stringAlgVars3: metamodelica::List<metamodelica::Ref<SimVar::SimVar>> = metamodelica::nil();
        let mut enumAlgVars3: metamodelica::List<metamodelica::Ref<SimVar::SimVar>> = metamodelica::nil();
        let mut inputVars: metamodelica::List<metamodelica::Ref<SimVar::SimVar>> = metamodelica::nil();
        let mut outputVars: metamodelica::List<metamodelica::Ref<SimVar::SimVar>> = metamodelica::nil();
        let mut aliasVars: metamodelica::List<metamodelica::Ref<SimVar::SimVar>> = metamodelica::nil();
        let mut intAliasVars: metamodelica::List<metamodelica::Ref<SimVar::SimVar>> = metamodelica::nil();
        let mut boolAliasVars: metamodelica::List<metamodelica::Ref<SimVar::SimVar>> = metamodelica::nil();
        let mut stringAliasVars: metamodelica::List<metamodelica::Ref<SimVar::SimVar>> = metamodelica::nil();
        let mut enumAliasVars: metamodelica::List<metamodelica::Ref<SimVar::SimVar>> = metamodelica::nil();
        let mut paramVars: metamodelica::List<metamodelica::Ref<SimVar::SimVar>> = metamodelica::nil();
        let mut intParamVars: metamodelica::List<metamodelica::Ref<SimVar::SimVar>> = metamodelica::nil();
        let mut boolParamVars: metamodelica::List<metamodelica::Ref<SimVar::SimVar>> = metamodelica::nil();
        let mut stringParamVars: metamodelica::List<metamodelica::Ref<SimVar::SimVar>> = metamodelica::nil();
        let mut enumParamVars: metamodelica::List<metamodelica::Ref<SimVar::SimVar>> = metamodelica::nil();
        let mut paramVarsR: metamodelica::List<metamodelica::Ref<SimVar::SimVar>> = metamodelica::nil();
        let mut intParamVarsR: metamodelica::List<metamodelica::Ref<SimVar::SimVar>> = metamodelica::nil();
        let mut boolParamVarsR: metamodelica::List<metamodelica::Ref<SimVar::SimVar>> = metamodelica::nil();
        let mut stringParamVarsR: metamodelica::List<metamodelica::Ref<SimVar::SimVar>> = metamodelica::nil();
        let mut enumParamVarsR: metamodelica::List<metamodelica::Ref<SimVar::SimVar>> = metamodelica::nil();
        let mut constVars: metamodelica::List<metamodelica::Ref<SimVar::SimVar>> = metamodelica::nil();
        let mut intConstVars: metamodelica::List<metamodelica::Ref<SimVar::SimVar>> = metamodelica::nil();
        let mut boolConstVars: metamodelica::List<metamodelica::Ref<SimVar::SimVar>> = metamodelica::nil();
        let mut stringConstVars: metamodelica::List<metamodelica::Ref<SimVar::SimVar>> = metamodelica::nil();
        let mut enumConstVars: metamodelica::List<metamodelica::Ref<SimVar::SimVar>> = metamodelica::nil();
        let mut extObjVars: metamodelica::List<metamodelica::Ref<SimVar::SimVar>> = metamodelica::nil();
        let mut residualVars: metamodelica::List<metamodelica::Ref<SimVar::SimVar>> = metamodelica::nil();
        let mut jacobianVars: metamodelica::List<metamodelica::Ref<SimVar::SimVar>> = metamodelica::nil();
        let mut seedVars: metamodelica::List<metamodelica::Ref<SimVar::SimVar>> = metamodelica::nil();
        let mut realOptimizeConstraintsVars: metamodelica::List<metamodelica::Ref<SimVar::SimVar>> =
            metamodelica::nil();
        let mut realOptimizeFinalConstraintsVars: metamodelica::List<metamodelica::Ref<SimVar::SimVar>> =
            metamodelica::nil();
        let mut enum_shift: i32 = 0;
        let mut sensitivityVars: metamodelica::List<metamodelica::Ref<SimVar::SimVar>> = metamodelica::nil();
        let mut dataReconSetcVars: metamodelica::List<metamodelica::Ref<SimVar::SimVar>> = metamodelica::nil();
        let mut dataReconinputVars: metamodelica::List<metamodelica::Ref<SimVar::SimVar>> = metamodelica::nil();
        let mut dataReconSetBVars: metamodelica::List<metamodelica::Ref<SimVar::SimVar>> = metamodelica::nil();
        let () = (match &**varData {
            BVariable::VarData::VAR_DATA_SIM {
                algebraics: __varData_algebraics,
                aliasVars: __varData_aliasVars,
                clocked_states: __varData_clocked_states,
                constants: __varData_constants,
                derivatives: __varData_derivatives,
                discrete_states: __varData_discrete_states,
                discretes: __varData_discretes,
                nonTrivialAlias: __varData_nonTrivialAlias,
                parameters: __varData_parameters,
                resizables: __varData_resizables,
                states: __varData_states,
                top_level_inputs: __varData_top_level_inputs,
                ..
            } => {
                let (__pa0, __pa1) = ::match_deref::match_deref! { match &(createSimVarLists(__varData_states.clone(), simCodeIndices, SplitType::NONE.clone(), VarType::SIMULATION.clone())?) {
                    (Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: Deref @ metamodelica::ListNode::Nil }, __pa1) => (__pa0.clone(), __pa1.clone()),
                    _ => return Err("pattern mismatch"),
                } };
                stateVars = metamodelica::Own::own(__pa0);
                simCodeIndices = metamodelica::Own::own(__pa1);
                let (__pa3, __pa4) = ::match_deref::match_deref! { match &(createSimVarLists(__varData_derivatives.clone(), simCodeIndices, SplitType::NONE.clone(), VarType::SIMULATION.clone())?) {
                    (Deref @ metamodelica::ListNode::Cons { head: __pa3, tail: Deref @ metamodelica::ListNode::Nil }, __pa4) => (__pa3.clone(), __pa4.clone()),
                    _ => return Err("pattern mismatch"),
                } };
                derivativeVars = metamodelica::Own::own(__pa3);
                simCodeIndices = metamodelica::Own::own(__pa4);
                let (__pa6, __pa7) = ::match_deref::match_deref! { match &(createSimVarLists(__varData_algebraics.clone(), simCodeIndices, SplitType::NONE.clone(), VarType::SIMULATION.clone())?) {
                    (Deref @ metamodelica::ListNode::Cons { head: __pa6, tail: Deref @ metamodelica::ListNode::Nil }, __pa7) => (__pa6.clone(), __pa7.clone()),
                    _ => return Err("pattern mismatch"),
                } };
                algVars = metamodelica::Own::own(__pa6);
                simCodeIndices = metamodelica::Own::own(__pa7);
                let (__pa9, __pa10) = ::match_deref::match_deref! { match &(createSimVarLists(__varData_top_level_inputs.clone(), simCodeIndices, SplitType::NONE.clone(), VarType::SIMULATION.clone())?) {
                    (Deref @ metamodelica::ListNode::Cons { head: __pa9, tail: Deref @ metamodelica::ListNode::Nil }, __pa10) => (__pa9.clone(), __pa10.clone()),
                    _ => return Err("pattern mismatch"),
                } };
                inputVars = metamodelica::Own::own(__pa9);
                simCodeIndices = metamodelica::Own::own(__pa10);
                let (__pa12, __pa13) = ::match_deref::match_deref! { match &(createSimVarLists(__varData_nonTrivialAlias.clone(), simCodeIndices, SplitType::NONE.clone(), VarType::SIMULATION.clone())?) {
                    (Deref @ metamodelica::ListNode::Cons { head: __pa12, tail: Deref @ metamodelica::ListNode::Nil }, __pa13) => (__pa12.clone(), __pa13.clone()),
                    _ => return Err("pattern mismatch"),
                } };
                nonTrivialAlias = metamodelica::Own::own(__pa12);
                simCodeIndices = metamodelica::Own::own(__pa13);
                let (__pa15, __pa16, __pa17, __pa18, __pa19, __pa20) = ::match_deref::match_deref! { match &(createSimVarLists(__varData_discretes.clone(), simCodeIndices, SplitType::TYPE.clone(), VarType::SIMULATION.clone())?) {
                    (Deref @ metamodelica::ListNode::Cons { head: __pa15, tail: Deref @ metamodelica::ListNode::Cons { head: __pa16, tail: Deref @ metamodelica::ListNode::Cons { head: __pa17, tail: Deref @ metamodelica::ListNode::Cons { head: __pa18, tail: Deref @ metamodelica::ListNode::Cons { head: __pa19, tail: Deref @ metamodelica::ListNode::Nil } } } } }, __pa20) => (__pa15.clone(), __pa16.clone(), __pa17.clone(), __pa18.clone(), __pa19.clone(), __pa20.clone()),
                    _ => return Err("pattern mismatch"),
                } };
                discreteAlgVars = metamodelica::Own::own(__pa15);
                intAlgVars = metamodelica::Own::own(__pa16);
                boolAlgVars = metamodelica::Own::own(__pa17);
                stringAlgVars = metamodelica::Own::own(__pa18);
                enumAlgVars = metamodelica::Own::own(__pa19);
                simCodeIndices = metamodelica::Own::own(__pa20);
                let (__pa22, __pa23, __pa24, __pa25, __pa26, __pa27) = ::match_deref::match_deref! { match &(createSimVarLists(__varData_discrete_states.clone(), simCodeIndices, SplitType::TYPE.clone(), VarType::SIMULATION.clone())?) {
                    (Deref @ metamodelica::ListNode::Cons { head: __pa22, tail: Deref @ metamodelica::ListNode::Cons { head: __pa23, tail: Deref @ metamodelica::ListNode::Cons { head: __pa24, tail: Deref @ metamodelica::ListNode::Cons { head: __pa25, tail: Deref @ metamodelica::ListNode::Cons { head: __pa26, tail: Deref @ metamodelica::ListNode::Nil } } } } }, __pa27) => (__pa22.clone(), __pa23.clone(), __pa24.clone(), __pa25.clone(), __pa26.clone(), __pa27.clone()),
                    _ => return Err("pattern mismatch"),
                } };
                discreteAlgVars2 = metamodelica::Own::own(__pa22);
                intAlgVars2 = metamodelica::Own::own(__pa23);
                boolAlgVars2 = metamodelica::Own::own(__pa24);
                stringAlgVars2 = metamodelica::Own::own(__pa25);
                enumAlgVars2 = metamodelica::Own::own(__pa26);
                simCodeIndices = metamodelica::Own::own(__pa27);
                let (__pa29, __pa30, __pa31, __pa32, __pa33, __pa34) = ::match_deref::match_deref! { match &(createSimVarLists(__varData_clocked_states.clone(), simCodeIndices, SplitType::TYPE.clone(), VarType::SIMULATION.clone())?) {
                    (Deref @ metamodelica::ListNode::Cons { head: __pa29, tail: Deref @ metamodelica::ListNode::Cons { head: __pa30, tail: Deref @ metamodelica::ListNode::Cons { head: __pa31, tail: Deref @ metamodelica::ListNode::Cons { head: __pa32, tail: Deref @ metamodelica::ListNode::Cons { head: __pa33, tail: Deref @ metamodelica::ListNode::Nil } } } } }, __pa34) => (__pa29.clone(), __pa30.clone(), __pa31.clone(), __pa32.clone(), __pa33.clone(), __pa34.clone()),
                    _ => return Err("pattern mismatch"),
                } };
                discreteAlgVars3 = metamodelica::Own::own(__pa29);
                intAlgVars3 = metamodelica::Own::own(__pa30);
                boolAlgVars3 = metamodelica::Own::own(__pa31);
                stringAlgVars3 = metamodelica::Own::own(__pa32);
                enumAlgVars3 = metamodelica::Own::own(__pa33);
                simCodeIndices = metamodelica::Own::own(__pa34);
                let (__pa36, __pa37, __pa38, __pa39, __pa40, __pa41) = ::match_deref::match_deref! { match &(createSimVarLists(__varData_aliasVars.clone(), simCodeIndices, SplitType::TYPE.clone(), VarType::ALIAS.clone())?) {
                    (Deref @ metamodelica::ListNode::Cons { head: __pa36, tail: Deref @ metamodelica::ListNode::Cons { head: __pa37, tail: Deref @ metamodelica::ListNode::Cons { head: __pa38, tail: Deref @ metamodelica::ListNode::Cons { head: __pa39, tail: Deref @ metamodelica::ListNode::Cons { head: __pa40, tail: Deref @ metamodelica::ListNode::Nil } } } } }, __pa41) => (__pa36.clone(), __pa37.clone(), __pa38.clone(), __pa39.clone(), __pa40.clone(), __pa41.clone()),
                    _ => return Err("pattern mismatch"),
                } };
                aliasVars = metamodelica::Own::own(__pa36);
                intAliasVars = metamodelica::Own::own(__pa37);
                boolAliasVars = metamodelica::Own::own(__pa38);
                stringAliasVars = metamodelica::Own::own(__pa39);
                enumAliasVars = metamodelica::Own::own(__pa40);
                simCodeIndices = metamodelica::Own::own(__pa41);
                enum_shift = simCodeIndices.integerVarIndex.clone();
                let (__pa43, __pa44, __pa45, __pa46, __pa47, __pa48) = ::match_deref::match_deref! { match &(createSimVarLists(__varData_parameters.clone(), simCodeIndices, SplitType::TYPE.clone(), VarType::PARAMETER.clone())?) {
                    (Deref @ metamodelica::ListNode::Cons { head: __pa43, tail: Deref @ metamodelica::ListNode::Cons { head: __pa44, tail: Deref @ metamodelica::ListNode::Cons { head: __pa45, tail: Deref @ metamodelica::ListNode::Cons { head: __pa46, tail: Deref @ metamodelica::ListNode::Cons { head: __pa47, tail: Deref @ metamodelica::ListNode::Nil } } } } }, __pa48) => (__pa43.clone(), __pa44.clone(), __pa45.clone(), __pa46.clone(), __pa47.clone(), __pa48.clone()),
                    _ => return Err("pattern mismatch"),
                } };
                paramVars = metamodelica::Own::own(__pa43);
                intParamVars = metamodelica::Own::own(__pa44);
                boolParamVars = metamodelica::Own::own(__pa45);
                stringParamVars = metamodelica::Own::own(__pa46);
                enumParamVars = metamodelica::Own::own(__pa47);
                simCodeIndices = metamodelica::Own::own(__pa48);
                let (__pa50, __pa51, __pa52, __pa53, __pa54, __pa55) = ::match_deref::match_deref! { match &(createSimVarLists(__varData_resizables.clone(), simCodeIndices, SplitType::TYPE.clone(), VarType::PARAMETER.clone())?) {
                    (Deref @ metamodelica::ListNode::Cons { head: __pa50, tail: Deref @ metamodelica::ListNode::Cons { head: __pa51, tail: Deref @ metamodelica::ListNode::Cons { head: __pa52, tail: Deref @ metamodelica::ListNode::Cons { head: __pa53, tail: Deref @ metamodelica::ListNode::Cons { head: __pa54, tail: Deref @ metamodelica::ListNode::Nil } } } } }, __pa55) => (__pa50.clone(), __pa51.clone(), __pa52.clone(), __pa53.clone(), __pa54.clone(), __pa55.clone()),
                    _ => return Err("pattern mismatch"),
                } };
                paramVarsR = metamodelica::Own::own(__pa50);
                intParamVarsR = metamodelica::Own::own(__pa51);
                boolParamVarsR = metamodelica::Own::own(__pa52);
                stringParamVarsR = metamodelica::Own::own(__pa53);
                enumParamVarsR = metamodelica::Own::own(__pa54);
                simCodeIndices = metamodelica::Own::own(__pa55);
                let (__pa57, __pa58, __pa59, __pa60, __pa61, __pa62) = ::match_deref::match_deref! { match &(createSimVarLists(__varData_constants.clone(), simCodeIndices, SplitType::TYPE.clone(), VarType::SIMULATION.clone())?) {
                    (Deref @ metamodelica::ListNode::Cons { head: __pa57, tail: Deref @ metamodelica::ListNode::Cons { head: __pa58, tail: Deref @ metamodelica::ListNode::Cons { head: __pa59, tail: Deref @ metamodelica::ListNode::Cons { head: __pa60, tail: Deref @ metamodelica::ListNode::Cons { head: __pa61, tail: Deref @ metamodelica::ListNode::Nil } } } } }, __pa62) => (__pa57.clone(), __pa58.clone(), __pa59.clone(), __pa60.clone(), __pa61.clone(), __pa62.clone()),
                    _ => return Err("pattern mismatch"),
                } };
                constVars = metamodelica::Own::own(__pa57);
                intConstVars = metamodelica::Own::own(__pa58);
                boolConstVars = metamodelica::Own::own(__pa59);
                stringConstVars = metamodelica::Own::own(__pa60);
                enumConstVars = metamodelica::Own::own(__pa61);
                simCodeIndices = metamodelica::Own::own(__pa62);
                let (__pa64, __pa65) = ::match_deref::match_deref! { match &(createSimVarLists(residual_vars, simCodeIndices, SplitType::NONE.clone(), VarType::RESIDUAL.clone())?) {
                    (Deref @ metamodelica::ListNode::Cons { head: __pa64, tail: Deref @ metamodelica::ListNode::Nil }, __pa65) => (__pa64.clone(), __pa65.clone()),
                    _ => return Err("pattern mismatch"),
                } };
                residualVars = metamodelica::Own::own(__pa64);
                simCodeIndices = metamodelica::Own::own(__pa65);
                outputVars = List::filterOnTrue(
                    List::flatten(list![
                        stateVars.clone(),
                        algVars.clone(),
                        discreteAlgVars.clone(),
                        discreteAlgVars2.clone(),
                        discreteAlgVars3.clone(),
                        intAlgVars.clone(),
                        intAlgVars2.clone(),
                        intAlgVars3.clone(),
                        boolAlgVars.clone(),
                        boolAlgVars2.clone(),
                        boolAlgVars3.clone(),
                        stringAlgVars.clone(),
                        stringAlgVars2.clone(),
                        stringAlgVars3.clone(),
                        enumAlgVars.clone(),
                        enumAlgVars2.clone(),
                        enumAlgVars3.clone()
                    ])?,
                    (std::sync::Arc::new(
                        move |__a0: metamodelica::Ref<SimVar::SimVar>| -> metamodelica::Result<_> {
                            ::std::result::Result::Ok(SimVar::isOutputSimVar(&__a0))
                        },
                    )
                        as std::sync::Arc<
                            dyn ::std::ops::Fn(metamodelica::Ref<SimVar::SimVar>) -> Result<bool> + 'static,
                        >),
                )?;
                ()
            }
            BVariable::VarData::VAR_DATA_JAC { .. } => (),
            BVariable::VarData::VAR_DATA_HES { .. } => (),
            _ => {
                Error::addMessage(
                    Error::INTERNAL_ERROR.clone(),
                    list![{
                        let mut __mm_s = String::new();
                        __mm_s.push_str(&*literal!("NSimVar.SimVars.create"));
                        __mm_s.push_str(&*literal!(" failed."));
                        ArcStr::from(__mm_s)
                    }],
                )?;
                return Err("fail");
            }
        });
        simVars = metamodelica::Ref::new(SimVars {
            stateVars: stateVars,
            derivativeVars: derivativeVars,
            algVars: List::flatten(list![algVars, inputVars.clone(), nonTrivialAlias])?,
            discreteAlgVars: List::flatten(list![discreteAlgVars, discreteAlgVars2, discreteAlgVars3])?,
            intAlgVars: List::flatten(list![intAlgVars, intAlgVars2, intAlgVars3])?,
            boolAlgVars: List::flatten(list![boolAlgVars, boolAlgVars2, boolAlgVars3])?,
            stringAlgVars: List::flatten(list![stringAlgVars, stringAlgVars2, stringAlgVars3])?,
            enumAlgVars: List::flatten(list![enumAlgVars, enumAlgVars2, enumAlgVars3])?,
            inputVars: inputVars,
            outputVars: outputVars,
            aliasVars: aliasVars,
            intAliasVars: intAliasVars,
            boolAliasVars: boolAliasVars,
            stringAliasVars: stringAliasVars,
            enumAliasVars: enumAliasVars,
            paramVars: List::flatten(list![paramVars, paramVarsR])?,
            intParamVars: List::flatten(list![intParamVars, intParamVarsR])?,
            boolParamVars: List::flatten(list![boolParamVars, boolParamVarsR])?,
            stringParamVars: List::flatten(list![stringParamVars, stringParamVarsR])?,
            enumParamVars: List::flatten(list![enumParamVars, enumParamVarsR])?,
            extObjVars: extObjVars,
            constVars: constVars,
            intConstVars: intConstVars,
            boolConstVars: boolConstVars,
            stringConstVars: stringConstVars,
            enumConstVars: enumConstVars,
            residualVars: residualVars,
            jacobianVars: jacobianVars,
            seedVars: seedVars,
            realOptimizeConstraintsVars: realOptimizeConstraintsVars,
            realOptimizeFinalConstraintsVars: realOptimizeFinalConstraintsVars,
            sensitivityVars: sensitivityVars,
            dataReconSetcVars: dataReconSetcVars,
            dataReconinputVars: dataReconinputVars,
            dataReconSetBVars: dataReconSetBVars,
        });
        assign_field!(
            simVars.intAlgVars = listAppend(
                simVars.intAlgVars.clone(),
                ({
                    let mut __acc: metamodelica::List<metamodelica::Ref<SimVar::SimVar>> = metamodelica::nil();
                    for mut v in (simVars.enumAlgVars.clone()).into_iter().cloned() {
                        let __x = SimVar::shiftIndex(v.clone(), enum_shift)?;
                        __acc = cons(__x, __acc);
                    }
                    __acc.reverse()
                })
            ),
            simVars.intAliasVars = listAppend(
                simVars.intAliasVars.clone(),
                ({
                    let mut __acc: metamodelica::List<metamodelica::Ref<SimVar::SimVar>> = metamodelica::nil();
                    for mut v in (simVars.enumAliasVars.clone()).into_iter().cloned() {
                        let __x = SimVar::shiftIndex(v.clone(), simCodeIndices.integerAliasIndex.clone())?;
                        __acc = cons(__x, __acc);
                    }
                    __acc.reverse()
                })
            ),
            simVars.intParamVars = listAppend(
                simVars.intParamVars.clone(),
                ({
                    let mut __acc: metamodelica::List<metamodelica::Ref<SimVar::SimVar>> = metamodelica::nil();
                    for mut v in (simVars.enumParamVars.clone()).into_iter().cloned() {
                        let __x = SimVar::shiftIndex(v.clone(), simCodeIndices.integerParamIndex.clone())?;
                        __acc = cons(__x, __acc);
                    }
                    __acc.reverse()
                })
            ),
            simVars.intConstVars = listAppend(
                simVars.intConstVars.clone(),
                ({
                    let mut __acc: metamodelica::List<metamodelica::Ref<SimVar::SimVar>> = metamodelica::nil();
                    for mut v in (simVars.enumConstVars.clone()).into_iter().cloned() {
                        let __x = SimVar::shiftIndex(v.clone(), simCodeIndices.integerVarIndex.clone())?;
                        __acc = cons(__x, __acc);
                    }
                    __acc.reverse()
                })
            )
        );
        Ok((simVars, simCodeIndices))
    }

    pub(crate) fn addSeedAndJacobianVars(
        mut vars: metamodelica::Ref<SimVars>,
        mut hash_tpl: &metamodelica::List<(
            metamodelica::Ref<ComponentRef::NFComponentRef>,
            metamodelica::Ref<SimVar::SimVar>,
        )>,
    ) -> Result<metamodelica::Ref<SimVars>> {
        let mut vars: metamodelica::Ref<SimVars> = vars;
        let mut cref: metamodelica::Ref<ComponentRef::NFComponentRef>;
        let mut var: metamodelica::Ref<SimVar::SimVar>;
        let mut seed_vars: metamodelica::List<metamodelica::Ref<SimVar::SimVar>> = metamodelica::nil();
        let mut jacobian_vars: metamodelica::List<metamodelica::Ref<SimVar::SimVar>> = metamodelica::nil();
        for mut tpl in &**hash_tpl {
            (cref, var) = tpl.clone();
            if BVariable::checkCref(
                &cref,
                &fnptr!(
                    BVariable::isSeed,
                    Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>
                ),
                metamodelica::sourceInfo!("NSimCode/NSimVar.mo"),
            )? {
                seed_vars = metamodelica::cons(var, seed_vars);
            } else {
                jacobian_vars = metamodelica::cons(var, jacobian_vars);
            }
        }
        assign_field!(
            vars.seedVars = listAppend(seed_vars, vars.seedVars.clone()),
            vars.jacobianVars = listAppend(jacobian_vars, vars.jacobianVars.clone())
        );
        Ok(vars)
    }

    pub(crate) fn size(mut simVars: &metamodelica::Ref<SimVars>) -> i32 {
        let mut size: i32 = ((simVars.stateVars).len() as i32)
            + ((simVars.derivativeVars).len() as i32)
            + ((simVars.algVars).len() as i32)
            + ((simVars.discreteAlgVars).len() as i32)
            + ((simVars.intAlgVars).len() as i32)
            + ((simVars.boolAlgVars).len() as i32)
            + ((simVars.inputVars).len() as i32)
            + ((simVars.outputVars).len() as i32)
            + ((simVars.aliasVars).len() as i32)
            + ((simVars.intAliasVars).len() as i32)
            + ((simVars.boolAliasVars).len() as i32)
            + ((simVars.paramVars).len() as i32)
            + ((simVars.intParamVars).len() as i32)
            + ((simVars.boolParamVars).len() as i32)
            + ((simVars.stringAlgVars).len() as i32)
            + ((simVars.stringParamVars).len() as i32)
            + ((simVars.stringAliasVars).len() as i32)
            + ((simVars.extObjVars).len() as i32)
            + ((simVars.constVars).len() as i32)
            + ((simVars.intConstVars).len() as i32)
            + ((simVars.boolConstVars).len() as i32)
            + ((simVars.stringConstVars).len() as i32)
            + ((simVars.stringAlgVars).len() as i32)
            + ((simVars.jacobianVars).len() as i32)
            + ((simVars.seedVars).len() as i32)
            + ((simVars.realOptimizeConstraintsVars).len() as i32)
            + ((simVars.realOptimizeFinalConstraintsVars).len() as i32)
            + ((simVars.sensitivityVars).len() as i32)
            + ((simVars.dataReconSetcVars).len() as i32)
            + ((simVars.dataReconinputVars).len() as i32)
            + ((simVars.dataReconSetBVars).len() as i32);
        size
    }

    pub(crate) fn convert(
        mut simVars: &metamodelica::Ref<SimVars>,
        mut memo: metamodelica::Ref<
            UnorderedMap::UnorderedMap<
                metamodelica::Ref<ComponentRef::NFComponentRef>,
                (
                    metamodelica::Ref<SimVar::SimVar>,
                    metamodelica::Ref<OldSimCodeVar::SimVar>,
                ),
            >,
        >,
    ) -> Result<OldSimCodeVar::SimVars> {
        let mut oldSimVars: OldSimCodeVar::SimVars;
        oldSimVars = OldSimCodeVar::SimVars {
            stateVars: SimVar::convertListMemo(simVars.stateVars.clone(), memo.clone())?,
            derivativeVars: SimVar::convertListMemo(simVars.derivativeVars.clone(), memo.clone())?,
            algVars: SimVar::convertListMemo(simVars.algVars.clone(), memo.clone())?,
            discreteAlgVars: SimVar::convertListMemo(simVars.discreteAlgVars.clone(), memo.clone())?,
            intAlgVars: SimVar::convertListMemo(simVars.intAlgVars.clone(), memo.clone())?,
            boolAlgVars: SimVar::convertListMemo(simVars.boolAlgVars.clone(), memo.clone())?,
            inputVars: SimVar::convertListMemo(simVars.inputVars.clone(), memo.clone())?,
            outputVars: SimVar::convertListMemo(simVars.outputVars.clone(), memo.clone())?,
            aliasVars: SimVar::convertListMemo(simVars.aliasVars.clone(), memo.clone())?,
            intAliasVars: SimVar::convertListMemo(simVars.intAliasVars.clone(), memo.clone())?,
            boolAliasVars: SimVar::convertListMemo(simVars.boolAliasVars.clone(), memo.clone())?,
            paramVars: SimVar::convertListMemo(simVars.paramVars.clone(), memo.clone())?,
            intParamVars: SimVar::convertListMemo(simVars.intParamVars.clone(), memo.clone())?,
            boolParamVars: SimVar::convertListMemo(simVars.boolParamVars.clone(), memo.clone())?,
            stringAlgVars: SimVar::convertListMemo(simVars.stringAlgVars.clone(), memo.clone())?,
            stringParamVars: SimVar::convertListMemo(simVars.stringParamVars.clone(), memo.clone())?,
            stringAliasVars: SimVar::convertListMemo(simVars.stringAliasVars.clone(), memo.clone())?,
            extObjVars: SimVar::convertListMemo(simVars.extObjVars.clone(), memo.clone())?,
            constVars: SimVar::convertListMemo(simVars.constVars.clone(), memo.clone())?,
            intConstVars: SimVar::convertListMemo(simVars.intConstVars.clone(), memo.clone())?,
            boolConstVars: SimVar::convertListMemo(simVars.boolConstVars.clone(), memo.clone())?,
            stringConstVars: SimVar::convertListMemo(simVars.stringConstVars.clone(), memo.clone())?,
            jacobianVars: SimVar::convertListMemo(simVars.jacobianVars.clone(), memo.clone())?,
            seedVars: SimVar::convertListMemo(simVars.seedVars.clone(), memo.clone())?,
            realOptimizeConstraintsVars: SimVar::convertListMemo(
                simVars.realOptimizeConstraintsVars.clone(),
                memo.clone(),
            )?,
            realOptimizeFinalConstraintsVars: SimVar::convertListMemo(
                simVars.realOptimizeFinalConstraintsVars.clone(),
                memo.clone(),
            )?,
            sensitivityVars: SimVar::convertListMemo(simVars.sensitivityVars.clone(), memo.clone())?,
            dataReconSetcVars: SimVar::convertListMemo(simVars.dataReconSetcVars.clone(), memo.clone())?,
            dataReconinputVars: SimVar::convertListMemo(simVars.dataReconinputVars.clone(), memo.clone())?,
            dataReconSetBVars: SimVar::convertListMemo(simVars.dataReconSetBVars.clone(), memo)?,
        };
        Ok(oldSimVars)
    }

    pub(crate) fn createSimVarLists(
        mut vars: metamodelica::Ref<VariablePointers::VariablePointers>,
        mut simCodeIndices: SimCodeIndices,
        mut splitType: SplitType,
        mut varType: VarType,
    ) -> Result<(
        metamodelica::List<metamodelica::List<metamodelica::Ref<SimVar::SimVar>>>,
        SimCodeIndices,
    )> {
        let mut simVars: metamodelica::List<metamodelica::List<metamodelica::Ref<SimVar::SimVar>>> =
            metamodelica::nil();
        let mut simCodeIndices: SimCodeIndices = simCodeIndices;
        let mut sim_vars: metamodelica::List<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>> =
            BVariable::VariablePointers::toList(
                &(if (Flags::getConfigBool(Flags::SIM_CODE_SCALARIZE.clone())?) {
                    BVariable::VariablePointers::scalarize(vars.clone())?
                } else {
                    vars.clone()
                }),
            )?;
        let mut lst: metamodelica::List<metamodelica::Ref<SimVar::SimVar>>;
        if splitType == SplitType::NONE.clone() {
            (lst, simCodeIndices) = SimVar::createList(&sim_vars, varType, simCodeIndices)?;
            simVars = list![lst];
        } else if splitType == SplitType::TYPE.clone() {
            (simVars, simCodeIndices) = SimVar::createListsByType(&sim_vars, varType, simCodeIndices)?;
        } else {
            Error::addMessage(
                Error::INTERNAL_ERROR.clone(),
                list![{
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!("NSimVar.SimVars.createSimVarLists"));
                    __mm_s.push_str(&*literal!(" failed because of invalid splitType."));
                    ArcStr::from(__mm_s)
                }],
            )?;
        }
        Ok((simVars, simCodeIndices))
    }

    pub(crate) fn getPartitionVars(
        mut partition: &metamodelica::Ref<Partition::Partition>,
        mut simcode_map: metamodelica::Ref<
            UnorderedMap::UnorderedMap<
                metamodelica::Ref<ComponentRef::NFComponentRef>,
                metamodelica::Ref<SimVar::SimVar>,
            >,
        >,
    ) -> Result<metamodelica::List<metamodelica::Ref<SimVar::SimVar>>> {
        let mut part_vars: metamodelica::List<metamodelica::Ref<SimVar::SimVar>>;
        part_vars = ({
            let mut result: metamodelica::List<metamodelica::List<metamodelica::Ref<SimVar::SimVar>>> =
                metamodelica::nil();
            (match partition.strongComponents.clone() {
                Some(mut comps) => {
                    for mut i in 1..=metamodelica::arrayLength(comps.clone()) {
                        result = metamodelica::cons(
                            getStrongComponentVars(
                                ({
                                    let __elt = (*metamodelica::index_checked(&comps.borrow(), i)?).clone();
                                    __elt
                                }),
                                simcode_map.clone(),
                            )?,
                            result,
                        );
                    }
                    List::flatten(result)?
                }
                _ => {
                    Error::addMessage(
                        Error::INTERNAL_ERROR.clone(),
                        list![{
                            let mut __mm_s = String::new();
                            __mm_s.push_str(&*literal!("NSimVar.SimVars.getPartitionVars"));
                            __mm_s.push_str(&*literal!(" failed for\n"));
                            __mm_s.push_str(&*Partition::toString(partition, 0)?);
                            ArcStr::from(__mm_s)
                        }],
                    )?;
                    return Err("fail");
                }
            })
        });
        Ok(part_vars)
    }

    pub(crate) fn getStrongComponentVars(
        mut comp: metamodelica::Ref<StrongComponent::NBStrongComponent>,
        mut simcode_map: metamodelica::Ref<
            UnorderedMap::UnorderedMap<
                metamodelica::Ref<ComponentRef::NFComponentRef>,
                metamodelica::Ref<SimVar::SimVar>,
            >,
        >,
    ) -> Result<metamodelica::List<metamodelica::Ref<SimVar::SimVar>>> {
        '__tco: loop {
            match &*comp {
                StrongComponent::SINGLE_COMPONENT { var: __comp_var, .. } => {
                    return Ok(getVars(__comp_var.clone(), simcode_map)?);
                }
                StrongComponent::MULTI_COMPONENT { vars: __comp_vars, .. } => {
                    return Ok(List::flatten(
                        ({
                            let mut __acc: metamodelica::List<metamodelica::List<metamodelica::Ref<SimVar::SimVar>>> =
                                metamodelica::nil();
                            for mut v in (__comp_vars.clone()).into_iter().cloned() {
                                let __x = getVars(Slice::getT(v.clone()), simcode_map.clone())?;
                                __acc = cons(__x, __acc);
                            }
                            __acc.reverse()
                        }),
                    )?);
                }
                StrongComponent::SLICED_COMPONENT { var: __comp_var, .. } => {
                    return Ok(getVars(Slice::getT(__comp_var.clone()), simcode_map)?);
                }
                StrongComponent::RESIZABLE_COMPONENT { var: __comp_var, .. } => {
                    return Ok(getVars(Slice::getT(__comp_var.clone()), simcode_map)?);
                }
                StrongComponent::GENERIC_COMPONENT {
                    var_cref: __comp_var_cref,
                    ..
                } => {
                    return Ok(getVars(
                        BVariable::getVarPointer(
                            metamodelica::AsArg::as_arg(&__comp_var_cref),
                            metamodelica::sourceInfo!("NSimCode/NSimVar.mo"),
                        )?,
                        simcode_map,
                    )?);
                }
                StrongComponent::ENTWINED_COMPONENT {
                    entwined_slices: __comp_entwined_slices,
                    ..
                } => {
                    return Ok(List::flatten(
                        ({
                            let mut __acc: metamodelica::List<metamodelica::List<metamodelica::Ref<SimVar::SimVar>>> =
                                metamodelica::nil();
                            for mut c in (__comp_entwined_slices.clone()).into_iter().cloned() {
                                let __x = getStrongComponentVars(c.clone(), simcode_map.clone())?;
                                __acc = cons(__x, __acc);
                            }
                            __acc.reverse()
                        }),
                    )?);
                }
                StrongComponent::ALGEBRAIC_LOOP {
                    strict: __comp_strict, ..
                } => {
                    return Ok(List::flatten(
                        ({
                            let mut __acc: metamodelica::List<metamodelica::List<metamodelica::Ref<SimVar::SimVar>>> =
                                metamodelica::nil();
                            for mut v in (__comp_strict.iteration_vars.clone()).into_iter().cloned() {
                                let __x = getVars(Slice::getT(v.clone()), simcode_map.clone())?;
                                __acc = cons(__x, __acc);
                            }
                            __acc.reverse()
                        }),
                    )?);
                }
                StrongComponent::ALIAS {
                    original: __comp_original,
                    ..
                } => {
                    (comp, simcode_map) = (__comp_original.clone(), simcode_map);
                    continue '__tco;
                }
                _ => {
                    Error::addMessage(
                        Error::INTERNAL_ERROR.clone(),
                        list![{
                            let mut __mm_s = String::new();
                            __mm_s.push_str(&*literal!("NSimVar.SimVars.getStrongComponentVars"));
                            __mm_s.push_str(&*literal!(" failed with unknown reason for\n"));
                            __mm_s.push_str(&*StrongComponent::toString(&comp, -1)?);
                            ArcStr::from(__mm_s)
                        }],
                    )?;
                    return Ok(return Err("fail"));
                }
            }
        }
    }

    pub(crate) fn numScalarElems(mut vars: metamodelica::List<metamodelica::Ref<SimVar::SimVar>>) -> i32 {
        let mut n: i32;
        n = ({
            let mut __acc: i32 = 0;
            for mut v in (vars).into_iter().cloned() {
                let __x = ({
                    let mut __acc: i32 = 1;
                    for mut e in (v.numArrayElement.clone()).into_iter().cloned() {
                        let __x = Expression::integerValueOrDefault(&(e.clone()), 1);
                        __acc *= __x;
                    }
                    __acc
                });
                __acc += __x;
            }
            __acc
        });
        n
    }

    fn getVars(
        mut var: Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>,
        mut simcode_map: metamodelica::Ref<
            UnorderedMap::UnorderedMap<
                metamodelica::Ref<ComponentRef::NFComponentRef>,
                metamodelica::Ref<SimVar::SimVar>,
            >,
        >,
    ) -> Result<metamodelica::List<metamodelica::Ref<SimVar::SimVar>>> {
        let mut vars: metamodelica::List<metamodelica::Ref<SimVar::SimVar>> = metamodelica::nil();
        if Flags::getConfigBool(Flags::SIM_CODE_SCALARIZE.clone())? {
            vars = ({
                let mut __acc: metamodelica::List<metamodelica::Ref<SimVar::SimVar>> = metamodelica::nil();
                for mut v in ((BVariable::VariablePointers::scalarizeList(&(list![var]))?).0)
                    .into_iter()
                    .cloned()
                {
                    let __x = UnorderedMap::getSafe(
                        BVariable::getVarName(v.clone()),
                        simcode_map.clone(),
                        metamodelica::sourceInfo!("NSimCode/NSimVar.mo"),
                    )?;
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            });
        } else {
            vars = list![UnorderedMap::getSafe(
                BVariable::getVarName(var),
                simcode_map,
                metamodelica::sourceInfo!("NSimCode/NSimVar.mo")
            )?];
        }
        Ok(vars)
    }
}

thread_local! { static __emptySimVars_TLS: metamodelica::Ref<SimVars::SimVars> = metamodelica::Ref::new(SimVars::SimVars { stateVars: metamodelica::nil(), derivativeVars: metamodelica::nil(), algVars: metamodelica::nil(), discreteAlgVars: metamodelica::nil(), intAlgVars: metamodelica::nil(), boolAlgVars: metamodelica::nil(), stringAlgVars: metamodelica::nil(), enumAlgVars: metamodelica::nil(), inputVars: metamodelica::nil(), outputVars: metamodelica::nil(), aliasVars: metamodelica::nil(), intAliasVars: metamodelica::nil(), boolAliasVars: metamodelica::nil(), stringAliasVars: metamodelica::nil(), enumAliasVars: metamodelica::nil(), paramVars: metamodelica::nil(), intParamVars: metamodelica::nil(), boolParamVars: metamodelica::nil(), stringParamVars: metamodelica::nil(), enumParamVars: metamodelica::nil(), extObjVars: metamodelica::nil(), constVars: metamodelica::nil(), intConstVars: metamodelica::nil(), boolConstVars: metamodelica::nil(), stringConstVars: metamodelica::nil(), enumConstVars: metamodelica::nil(), residualVars: metamodelica::nil(), jacobianVars: metamodelica::nil(), seedVars: metamodelica::nil(), realOptimizeConstraintsVars: metamodelica::nil(), realOptimizeFinalConstraintsVars: metamodelica::nil(), sensitivityVars: metamodelica::nil(), dataReconSetcVars: metamodelica::nil(), dataReconinputVars: metamodelica::nil(), dataReconSetBVars: metamodelica::nil() }); }
pub(crate) fn emptySimVars() -> metamodelica::Ref<SimVars::SimVars> {
    __emptySimVars_TLS.with(|__t| __t.clone())
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, metamodelica::ReferenceEq)]
#[repr(i32)]
pub(crate) enum SplitType {
    NONE = 1,
    TYPE = 2,
}
impl PartialOrd for SplitType {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for SplitType {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (*self as i32).cmp(&(*other as i32))
    }
}
impl metamodelica::gc::MMTrace for SplitType {
    fn mm_accept(&self, _: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        Ok(())
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, metamodelica::ReferenceEq)]
#[repr(i32)]
pub(crate) enum VarType {
    SIMULATION = 1,
    PARAMETER = 2,
    ALIAS = 3,
    RESIDUAL = 4,
    EXTERNAL_OBJECT = 5,
}
impl PartialOrd for VarType {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for VarType {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (*self as i32).cmp(&(*other as i32))
    }
}
impl metamodelica::gc::MMTrace for VarType {
    fn mm_accept(&self, _: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        Ok(())
    }
}

// ToDo: PRE, OLD, RELATIONS...
pub mod VarInfo {
    use super::*;
    #[derive(Clone, Debug, Eq, Hash, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
    pub struct VarInfo {
        pub numZeroCrossings: i32,
        pub numTimeEvents: i32,
        pub numRelations: i32,
        pub numMathEventFunctions: i32,
        pub numStateVars: i32,
        pub numAlgVars: i32,
        pub numDiscreteReal: i32,
        pub numIntAlgVars: i32,
        pub numBoolAlgVars: i32,
        pub numAlgAliasVars: i32,
        pub numIntAliasVars: i32,
        pub numBoolAliasVars: i32,
        pub numParams: i32,
        pub numIntParams: i32,
        pub numBoolParams: i32,
        pub numOutVars: i32,
        pub numInVars: i32,
        pub numExternalObjects: i32,
        pub numStringAlgVars: i32,
        pub numStringParamVars: i32,
        pub numStringAliasVars: i32,
        pub numEquations: i32,
        pub numLinearSystems: i32,
        pub numNonLinearSystems: i32,
        pub numMixedSystems: i32,
        pub numStateSets: i32,
        pub numJacobians: i32,
        pub numOptimizeConstraints: i32,
        pub numOptimizeFinalConstraints: i32,
        pub numSensitivityParameters: i32,
        pub numSetcVars: i32,
        pub numDataReconVars: i32,
        pub numRealIntputVars: i32,
        pub numSetbVars: i32,
        pub numRelatedBoundaryConditions: i32,
    }

    impl metamodelica::gc::MMTrace for VarInfo {
        fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
            metamodelica::gc::MMTrace::mm_accept(&self.numZeroCrossings, __mmv)?;
            metamodelica::gc::MMTrace::mm_accept(&self.numTimeEvents, __mmv)?;
            metamodelica::gc::MMTrace::mm_accept(&self.numRelations, __mmv)?;
            metamodelica::gc::MMTrace::mm_accept(&self.numMathEventFunctions, __mmv)?;
            metamodelica::gc::MMTrace::mm_accept(&self.numStateVars, __mmv)?;
            metamodelica::gc::MMTrace::mm_accept(&self.numAlgVars, __mmv)?;
            metamodelica::gc::MMTrace::mm_accept(&self.numDiscreteReal, __mmv)?;
            metamodelica::gc::MMTrace::mm_accept(&self.numIntAlgVars, __mmv)?;
            metamodelica::gc::MMTrace::mm_accept(&self.numBoolAlgVars, __mmv)?;
            metamodelica::gc::MMTrace::mm_accept(&self.numAlgAliasVars, __mmv)?;
            metamodelica::gc::MMTrace::mm_accept(&self.numIntAliasVars, __mmv)?;
            metamodelica::gc::MMTrace::mm_accept(&self.numBoolAliasVars, __mmv)?;
            metamodelica::gc::MMTrace::mm_accept(&self.numParams, __mmv)?;
            metamodelica::gc::MMTrace::mm_accept(&self.numIntParams, __mmv)?;
            metamodelica::gc::MMTrace::mm_accept(&self.numBoolParams, __mmv)?;
            metamodelica::gc::MMTrace::mm_accept(&self.numOutVars, __mmv)?;
            metamodelica::gc::MMTrace::mm_accept(&self.numInVars, __mmv)?;
            metamodelica::gc::MMTrace::mm_accept(&self.numExternalObjects, __mmv)?;
            metamodelica::gc::MMTrace::mm_accept(&self.numStringAlgVars, __mmv)?;
            metamodelica::gc::MMTrace::mm_accept(&self.numStringParamVars, __mmv)?;
            metamodelica::gc::MMTrace::mm_accept(&self.numStringAliasVars, __mmv)?;
            metamodelica::gc::MMTrace::mm_accept(&self.numEquations, __mmv)?;
            metamodelica::gc::MMTrace::mm_accept(&self.numLinearSystems, __mmv)?;
            metamodelica::gc::MMTrace::mm_accept(&self.numNonLinearSystems, __mmv)?;
            metamodelica::gc::MMTrace::mm_accept(&self.numMixedSystems, __mmv)?;
            metamodelica::gc::MMTrace::mm_accept(&self.numStateSets, __mmv)?;
            metamodelica::gc::MMTrace::mm_accept(&self.numJacobians, __mmv)?;
            metamodelica::gc::MMTrace::mm_accept(&self.numOptimizeConstraints, __mmv)?;
            metamodelica::gc::MMTrace::mm_accept(&self.numOptimizeFinalConstraints, __mmv)?;
            metamodelica::gc::MMTrace::mm_accept(&self.numSensitivityParameters, __mmv)?;
            metamodelica::gc::MMTrace::mm_accept(&self.numSetcVars, __mmv)?;
            metamodelica::gc::MMTrace::mm_accept(&self.numDataReconVars, __mmv)?;
            metamodelica::gc::MMTrace::mm_accept(&self.numRealIntputVars, __mmv)?;
            metamodelica::gc::MMTrace::mm_accept(&self.numSetbVars, __mmv)?;
            metamodelica::gc::MMTrace::mm_accept(&self.numRelatedBoundaryConditions, __mmv)?;
            Ok(())
        }
    }
    impl Default for VarInfo {
        fn default() -> Self {
            Self {
                numZeroCrossings: Default::default(),
                numTimeEvents: Default::default(),
                numRelations: Default::default(),
                numMathEventFunctions: Default::default(),
                numStateVars: Default::default(),
                numAlgVars: Default::default(),
                numDiscreteReal: Default::default(),
                numIntAlgVars: Default::default(),
                numBoolAlgVars: Default::default(),
                numAlgAliasVars: Default::default(),
                numIntAliasVars: Default::default(),
                numBoolAliasVars: Default::default(),
                numParams: Default::default(),
                numIntParams: Default::default(),
                numBoolParams: Default::default(),
                numOutVars: Default::default(),
                numInVars: Default::default(),
                numExternalObjects: Default::default(),
                numStringAlgVars: Default::default(),
                numStringParamVars: Default::default(),
                numStringAliasVars: Default::default(),
                numEquations: Default::default(),
                numLinearSystems: Default::default(),
                numNonLinearSystems: Default::default(),
                numMixedSystems: Default::default(),
                numStateSets: Default::default(),
                numJacobians: Default::default(),
                numOptimizeConstraints: Default::default(),
                numOptimizeFinalConstraints: Default::default(),
                numSensitivityParameters: Default::default(),
                numSetcVars: Default::default(),
                numDataReconVars: Default::default(),
                numRealIntputVars: Default::default(),
                numSetbVars: Default::default(),
                numRelatedBoundaryConditions: Default::default(),
            }
        }
    }

    pub type VAR_INFO = VarInfo;

    pub(crate) fn listScalarSize(mut vars: metamodelica::List<metamodelica::Ref<SimVar::SimVar>>) -> Result<i32> {
        let mut sz: i32;
        if stringEqual(&(Config::simCodeTarget()?), &(literal!("Cpp"))) {
            sz = ({
                let mut __acc: i32 = 0;
                for mut v in (vars).into_iter().cloned() {
                    let __x = ({
                        let mut __acc: i32 = 1;
                        for mut e in (v.numArrayElement.clone()).into_iter().cloned() {
                            let __x = Expression::integerValueOrDefault(&(e.clone()), 1);
                            __acc *= __x;
                        }
                        __acc
                    });
                    __acc += __x;
                }
                __acc
            });
        } else {
            sz = ((vars).len() as i32);
        }
        Ok(sz)
    }

    pub(crate) fn create(
        mut vars: &metamodelica::Ref<SimVars::SimVars>,
        mut eventInfo: &metamodelica::Ref<EventInfo::EventInfo>,
        mut simCodeIndices: &SimCodeIndices,
    ) -> Result<metamodelica::Ref<VarInfo>> {
        let mut varInfo: metamodelica::Ref<VarInfo>;
        varInfo = metamodelica::Ref::new(VarInfo {
            numZeroCrossings: ({
                let mut __acc: i32 = 0;
                for mut cond in (UnorderedMap::keyList(eventInfo.state_map.clone()))
                    .into_iter()
                    .cloned()
                {
                    let __x = Condition::size(&(cond.clone()))?;
                    __acc += __x;
                }
                __acc
            }),
            numTimeEvents: UnorderedSet::size(eventInfo.time_set.clone()),
            numRelations: ({
                let mut __acc: i32 = 0;
                for mut cond in (UnorderedMap::keyList(eventInfo.state_map.clone()))
                    .into_iter()
                    .cloned()
                {
                    let __x = Condition::size(&(cond.clone()))?;
                    __acc += __x;
                }
                __acc
            }),
            numMathEventFunctions: eventInfo.numberMathEvents.clone(),
            numStateVars: listScalarSize(vars.stateVars.clone())?,
            numAlgVars: listScalarSize(vars.algVars.clone())?,
            numDiscreteReal: listScalarSize(vars.discreteAlgVars.clone())?,
            numIntAlgVars: listScalarSize(vars.intAlgVars.clone())?,
            numBoolAlgVars: listScalarSize(vars.boolAlgVars.clone())?,
            numAlgAliasVars: listScalarSize(vars.aliasVars.clone())?,
            numIntAliasVars: listScalarSize(vars.intAliasVars.clone())?,
            numBoolAliasVars: listScalarSize(vars.boolAliasVars.clone())?,
            numParams: listScalarSize(vars.paramVars.clone())?,
            numIntParams: listScalarSize(vars.intParamVars.clone())?,
            numBoolParams: listScalarSize(vars.boolParamVars.clone())?,
            numOutVars: ((vars.outputVars).len() as i32),
            numInVars: ({
                let mut __acc: i32 = 0;
                for mut v in (vars.inputVars.clone()).into_iter().cloned() {
                    let __x = if (Type::isArray(&(v.type_.clone()))) {
                        ({
                            let mut __acc: i32 = 1;
                            for mut e in (v.numArrayElement.clone()).into_iter().cloned() {
                                let __x = Expression::integerValueOrDefault(&(e.clone()), 1);
                                __acc *= __x;
                            }
                            __acc
                        })
                    } else {
                        1
                    };
                    __acc += __x;
                }
                __acc
            }),
            numExternalObjects: ((vars.extObjVars).len() as i32),
            numStringAlgVars: listScalarSize(vars.stringAlgVars.clone())?,
            numStringParamVars: listScalarSize(vars.stringParamVars.clone())?,
            numStringAliasVars: listScalarSize(vars.stringAliasVars.clone())?,
            numEquations: simCodeIndices.equationIndex.clone(),
            numLinearSystems: simCodeIndices.linearSystemIndex.clone(),
            numNonLinearSystems: simCodeIndices.nonlinearSystemIndex.clone(),
            numMixedSystems: 0,
            numStateSets: 0,
            numJacobians: simCodeIndices.nonlinearSystemIndex.clone() + simCodeIndices.linearSystemIndex.clone() + 5,
            numOptimizeConstraints: 0,
            numOptimizeFinalConstraints: 0,
            numSensitivityParameters: 0,
            numSetcVars: 0,
            numDataReconVars: 0,
            numRealIntputVars: 0,
            numSetbVars: 0,
            numRelatedBoundaryConditions: 0,
        });
        Ok(varInfo)
    }

    pub(crate) fn convert(mut varInfo: &metamodelica::Ref<VarInfo>) -> OldSimCode::VarInfo {
        let mut oldVarInfo: OldSimCode::VarInfo;
        oldVarInfo = OldSimCode::VarInfo {
            numZeroCrossings: varInfo.numZeroCrossings.clone(),
            numTimeEvents: varInfo.numTimeEvents.clone(),
            numRelations: varInfo.numRelations.clone(),
            numMathEventFunctions: varInfo.numMathEventFunctions.clone(),
            numStateVars: varInfo.numStateVars.clone(),
            numAlgVars: varInfo.numAlgVars.clone(),
            numDiscreteReal: varInfo.numDiscreteReal.clone(),
            numIntAlgVars: varInfo.numIntAlgVars.clone(),
            numBoolAlgVars: varInfo.numBoolAlgVars.clone(),
            numAlgAliasVars: varInfo.numAlgAliasVars.clone(),
            numIntAliasVars: varInfo.numIntAliasVars.clone(),
            numBoolAliasVars: varInfo.numBoolAliasVars.clone(),
            numParams: varInfo.numParams.clone(),
            numIntParams: varInfo.numIntParams.clone(),
            numBoolParams: varInfo.numBoolParams.clone(),
            numOutVars: varInfo.numOutVars.clone(),
            numInVars: varInfo.numInVars.clone(),
            numExternalObjects: varInfo.numExternalObjects.clone(),
            numStringAlgVars: varInfo.numStringAlgVars.clone(),
            numStringParamVars: varInfo.numStringParamVars.clone(),
            numStringAliasVars: varInfo.numStringAliasVars.clone(),
            numEquations: varInfo.numEquations.clone(),
            numLinearSystems: varInfo.numLinearSystems.clone(),
            numNonLinearSystems: varInfo.numNonLinearSystems.clone(),
            numMixedSystems: varInfo.numMixedSystems.clone(),
            numStateSets: varInfo.numStateSets.clone(),
            numJacobians: varInfo.numJacobians.clone(),
            numOptimizeConstraints: varInfo.numOptimizeConstraints.clone(),
            numOptimizeFinalConstraints: varInfo.numOptimizeFinalConstraints.clone(),
            numSensitivityParameters: varInfo.numSensitivityParameters.clone(),
            numSetcVars: varInfo.numSetcVars.clone(),
            numDataReconVars: varInfo.numDataReconVars.clone(),
            numRealInputVars: varInfo.numRealIntputVars.clone(),
            numSetbVars: varInfo.numSetbVars.clone(),
            numRelatedBoundaryConditions: varInfo.numRelatedBoundaryConditions.clone(),
        };
        oldVarInfo
    }
}

pub mod ExtObjInfo {
    use super::*;
    #[derive(Clone, Debug, Eq, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
    pub struct ExtObjInfo {
        pub objects: metamodelica::List<metamodelica::Ref<SimVar::SimVar>>,
        pub aliases: metamodelica::List<(
            metamodelica::Ref<ComponentRef::NFComponentRef>,
            metamodelica::Ref<ComponentRef::NFComponentRef>,
        )>,
    }

    impl metamodelica::gc::MMTrace for ExtObjInfo {
        fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
            metamodelica::gc::MMTrace::mm_accept(&self.objects, __mmv)?;
            metamodelica::gc::MMTrace::mm_accept(&self.aliases, __mmv)?;
            Ok(())
        }
    }
    impl Default for ExtObjInfo {
        fn default() -> Self {
            Self {
                objects: Default::default(),
                aliases: Default::default(),
            }
        }
    }

    pub type EXT_OBJ_INFO = ExtObjInfo;

    pub(crate) fn toString(mut info: &metamodelica::Ref<ExtObjInfo>) -> Result<ArcStr> {
        let mut r#str: ArcStr = SimVar::listToString(&info.objects, literal!("External Objects"), false)?;
        Ok(r#str)
    }

    pub(crate) fn create(
        mut external_objects: &metamodelica::Ref<VariablePointers::VariablePointers>,
        mut vars: metamodelica::Ref<SimVars::SimVars>,
        mut simCodeIndices: SimCodeIndices,
    ) -> Result<(
        metamodelica::Ref<ExtObjInfo>,
        metamodelica::Ref<SimVars::SimVars>,
        SimCodeIndices,
    )> {
        let mut info: metamodelica::Ref<ExtObjInfo>;
        let mut vars: metamodelica::Ref<SimVars::SimVars> = vars;
        let mut simCodeIndices: SimCodeIndices = simCodeIndices;
        let mut var_lst: metamodelica::List<metamodelica::Ref<SimVar::SimVar>>;
        (var_lst, simCodeIndices) = SimVar::createList(
            &(BVariable::VariablePointers::toList(external_objects)?),
            VarType::EXTERNAL_OBJECT.clone(),
            simCodeIndices,
        )?;
        assign_field!(vars.extObjVars = var_lst.clone());
        info = metamodelica::Ref::new(ExtObjInfo {
            objects: var_lst,
            aliases: metamodelica::nil(),
        });
        Ok((info, vars, simCodeIndices))
    }

    pub(crate) fn convert(mut info: &metamodelica::Ref<ExtObjInfo>) -> Result<OldSimCode::ExtObjInfo> {
        let mut oldInfo: OldSimCode::ExtObjInfo;
        oldInfo = OldSimCode::ExtObjInfo {
            vars: SimVar::convertList(info.objects.clone())?,
            aliases: metamodelica::nil(),
        };
        Ok(oldInfo)
    }
}
