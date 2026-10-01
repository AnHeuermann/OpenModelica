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

use crate::NBASSC as ASSC;
use crate::NBAdjacency;
use crate::NBCausalize as Causalize;
use crate::NBDifferentiate as Differentiate;
use crate::NBDifferentiate::DifferentiationArguments;
use crate::NBDifferentiate::DifferentiationType;
use crate::NBEquation as BEquation;
use crate::NBEquation::EqData;
use crate::NBEquation::Equation;
use crate::NBEquation::EquationAttributes;
use crate::NBEquation::EquationKind;
use crate::NBEquation::EquationPointers;
use crate::NBEquation::Iterator;
use crate::NBModule as Module;
use crate::NBPartition as Partition;
use crate::NBReplacements as Replacements;
use crate::NBSlice as Slice;
use crate::NBSolve as Solve;
use crate::NBSolve::Status;
use crate::NBStrongComponent as StrongComponent;
use crate::NBTearing as Tearing;
use crate::NBVariable as BVariable;
use crate::NBVariable::VarData;
use crate::NBVariable::VariablePointer;
use crate::NBVariable::VariablePointers;
use crate::NBackendDAE as BackendDAE;
use openmodelica_ast::Absyn;
use openmodelica_frontend_dump::AbsynUtil;
use openmodelica_frontend_types::DAE;
use openmodelica_nf_frontend::NFBackendExtension as BackendExtension;
use openmodelica_nf_frontend::NFBackendExtension::StateSelect;
use openmodelica_nf_frontend::NFBackendExtension::TearingSelect;
use openmodelica_nf_frontend::NFBackendExtension::VariableKind;
use openmodelica_nf_frontend::NFBinding as Binding;
use openmodelica_nf_frontend::NFBuiltinFuncs;
use openmodelica_nf_frontend::NFCall as Call;
use openmodelica_nf_frontend::NFComponentRef as ComponentRef;
use openmodelica_nf_frontend::NFDimension as Dimension;
use openmodelica_nf_frontend::NFExpression as Expression;
use openmodelica_nf_frontend::NFExpressionIterator as ExpressionIterator;
use openmodelica_nf_frontend::NFFunction::Function;
use openmodelica_nf_frontend::NFInstNode::InstNode;
use openmodelica_nf_frontend::NFOperator as Operator;
use openmodelica_nf_frontend::NFPrefixes;
use openmodelica_nf_frontend::NFPrefixes::Variability;
use openmodelica_nf_frontend::NFSimplifyExp as SimplifyExp;
use openmodelica_nf_frontend::NFSubscript as Subscript;
use openmodelica_nf_frontend::NFType as Type;
use openmodelica_nf_frontend::NFVariable as Variable;
use openmodelica_util::Error;
use openmodelica_util::Flags;
use openmodelica_util::StringUtil;
use openmodelica_util::UnorderedMap;
use openmodelica_util::UnorderedSet;
use openmodelica_util::Util;
use openmodelica_util_datatypes_basic::Array;
use openmodelica_util_datatypes_basic::List;
use openmodelica_util_datatypes_basic::Pointer;

// ToDo:
// 1. simple state rules (with derivative replacement)
//    - state = state
//    - state = alg
//    - state = time
//    - state = const
// 2. write rateVar() and decide if we want an auxiliary for each set
//    - rateVar() --> mergeAttributes()
// 3. post causalize alias elimination
//    - for the ODE
//    - for jacobians/hessians (once we got hessians)
//    - for strong components in general
// 4. simplify only replaced equations and remove simplify2 module
//    - probably not that trivial
//    - Equation mapExp function that returns true if something was replaced
//    - EquationArray map function that accumulates pointers if function returns true
//    - simplify all equations in pointer list
// 5. trivial solution a = b; a = -b; (or other cyclic sets)
//    - take an equation from the set, get both crefs in it (a,b)
//    - solve for a -> set a as known
//    - solve the rest of the set with causalize
//    - replacements a -> what it solves for in eq1 and apply on all eq in set
//    - find equation that solves b, and solve for b. add to replacements
//    - apply replacements on all eq
// OF imports
// NF imports
// Backend imports
// Util imports
// ==========================================================================
//               Single Variable constants and functions
// ==========================================================================
pub(crate) const NOMINAL_THRESHOLD: metamodelica::Real = metamodelica::OrderedFloat(1000.0_f64);

pub(crate) fn main(
    mut bdae: metamodelica::Ref<BackendDAE::NBackendDAE>,
    mut kind: Partition::Kind,
) -> Result<metamodelica::Ref<BackendDAE::NBackendDAE>> {
    let mut bdae: metamodelica::Ref<BackendDAE::NBackendDAE> = bdae;
    let mut func: Module::aliasInterface;
    func = getModule()?;
    bdae = (match &*bdae {
        BackendDAE::MAIN { varData, eqData, .. } => {
            let mut varData = (*varData).clone();
            let mut eqData = (*eqData).clone();
            (varData, eqData) = func(varData.clone(), eqData.clone(), kind)?;
            (varData, eqData) = aliasClocks(varData.clone(), eqData.clone(), kind)?;
            assign_variant_field!(bdae => BackendDAE::NBackendDAE::MAIN;
                varData = varData.clone(),
                eqData = eqData.clone()
            );
            bdae
        }
        BackendDAE::HESSIAN { varData, eqData } => {
            let mut varData = (*varData).clone();
            let mut eqData = (*eqData).clone();
            (varData, eqData) = func(varData.clone(), eqData.clone(), kind)?;
            assign_variant_field!(bdae => BackendDAE::NBackendDAE::HESSIAN;
                varData = varData.clone(),
                eqData = eqData.clone()
            );
            bdae
        }
        _ => {
            Error::addMessage(
                Error::INTERNAL_ERROR.clone(),
                list![{
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!("NBAlias.main"));
                    __mm_s.push_str(&*literal!(" failed."));
                    ArcStr::from(__mm_s)
                }],
            )?;
            return Err("fail");
        }
    });
    Ok(bdae)
}

pub(crate) fn getModule() -> Result<
    Arc<
        dyn ::std::ops::Fn(
                metamodelica::Ref<VarData::VarData>,
                metamodelica::Ref<EqData::EqData>,
                Partition::Kind,
            )
                -> Result<(metamodelica::Ref<VarData::VarData>, metamodelica::Ref<EqData::EqData>)>
            + 'static,
    >,
> {
    let mut func: Module::aliasInterface;
    let mut flag: ArcStr = literal!("default");
    func = (::match_deref::match_deref! { match &(flag) {
        Deref @ "default" => (std::sync::Arc::new(aliasDefault) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<VarData::VarData>, metamodelica::Ref<EqData::EqData>, Partition::Kind) -> Result<(metamodelica::Ref<VarData::VarData>, metamodelica::Ref<EqData::EqData>)> + 'static>),
        _ => return Err("fail"),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(func)
}

pub mod AliasSet {
    use super::*;
    /// gets accumulated to find sets of alias equations and solve them
    #[derive(Clone, Debug, Eq, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
    pub struct AliasSet {
        /// list of all variables in this set
        pub simple_variables: metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>>,
        /// list of all equations in this set
        pub simple_equations: metamodelica::List<Pointer::Pointer<metamodelica::Ref<Equation::Equation>>>,
        /// optional constant binding of one variable
        pub const_opt: Option<Pointer::Pointer<metamodelica::Ref<Equation::Equation>>>,
    }

    impl metamodelica::gc::MMTrace for AliasSet {
        fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
            metamodelica::gc::MMTrace::mm_accept(&self.simple_variables, __mmv)?;
            metamodelica::gc::MMTrace::mm_accept(&self.simple_equations, __mmv)?;
            metamodelica::gc::MMTrace::mm_accept(&self.const_opt, __mmv)?;
            Ok(())
        }
    }
    impl Default for AliasSet {
        fn default() -> Self {
            Self {
                simple_variables: Default::default(),
                simple_equations: Default::default(),
                const_opt: Default::default(),
            }
        }
    }

    pub type ALIAS_SET = AliasSet;

    pub(crate) fn toString(mut set: &metamodelica::Ref<AliasSet>) -> Result<ArcStr> {
        let mut r#str: ArcStr;
        if (set.const_opt).is_some() {
            r#str = {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("\tConstant/Parameter Binding: "));
                __mm_s.push_str(&*BEquation::Equation::toString(
                    Pointer::access(Util::getOption(set.const_opt.clone())?),
                    literal!(""),
                )?);
                __mm_s.push_str(&*literal!("\n"));
                ArcStr::from(__mm_s)
            };
        } else {
            r#str = literal!("\t<No Constant/Parameter Binding>\n");
        }
        if (set.simple_equations).is_empty() {
            r#str = {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*r#str);
                __mm_s.push_str(&*literal!("\t###<No Set Equations>\n"));
                ArcStr::from(__mm_s)
            };
        } else {
            r#str = {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*r#str);
                __mm_s.push_str(&*literal!("\t### Set Equations:\n"));
                ArcStr::from(__mm_s)
            };
            for mut eq in &*set.simple_equations.clone() {
                r#str = {
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*r#str);
                    __mm_s.push_str(&*BEquation::Equation::toString(
                        Pointer::access(eq.clone()),
                        literal!("\t"),
                    )?);
                    __mm_s.push_str(&*literal!("\n"));
                    ArcStr::from(__mm_s)
                };
            }
        }
        Ok(r#str)
    }
}

thread_local! { static __EMPTY_ALIAS_SET_TLS: metamodelica::Ref<AliasSet::AliasSet> = metamodelica::Ref::new(AliasSet::AliasSet { simple_variables: metamodelica::nil(), simple_equations: metamodelica::nil(), const_opt: None }); }
pub(crate) fn EMPTY_ALIAS_SET() -> metamodelica::Ref<AliasSet::AliasSet> {
    __EMPTY_ALIAS_SET_TLS.with(|__t| __t.clone())
}

// needed for unordered map
pub type SetPtr = Pointer::Pointer<metamodelica::Ref<AliasSet::AliasSet>>;

/// used for findCrefs()
#[derive(Clone, Debug, Eq, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub struct CrefTpl {
    /// false if search already resulted in non simple structure
    pub cont: bool,
    /// variable count
    pub varCount: i32,
    /// parameter/constant count
    pub paramCount: i32,
    /// list of found variables for replacement
    pub cr_lst: metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>>,
}

impl metamodelica::gc::MMTrace for CrefTpl {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        metamodelica::gc::MMTrace::mm_accept(&self.cont, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.varCount, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.paramCount, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.cr_lst, __mmv)?;
        Ok(())
    }
}
impl Default for CrefTpl {
    fn default() -> Self {
        Self {
            cont: Default::default(),
            varCount: Default::default(),
            paramCount: Default::default(),
            cr_lst: Default::default(),
        }
    }
}

pub type CREF_TPL = CrefTpl;

thread_local! { static __EMPTY_CREF_TPL_TLS: CrefTpl = CrefTpl { cont: true, varCount: 0, paramCount: 0, cr_lst: metamodelica::nil() }; }
pub(crate) fn EMPTY_CREF_TPL() -> CrefTpl {
    __EMPTY_CREF_TPL_TLS.with(|__t| __t.clone())
}

thread_local! { static __FAILED_CREF_TPL_TLS: CrefTpl = CrefTpl { cont: false, varCount: 0, paramCount: 0, cr_lst: metamodelica::nil() }; }
pub(crate) fn FAILED_CREF_TPL() -> CrefTpl {
    __FAILED_CREF_TPL_TLS.with(|__t| __t.clone())
}

fn aliasDefault(
    mut varData: metamodelica::Ref<VarData::VarData>,
    mut eqData: metamodelica::Ref<EqData::EqData>,
    mut kind: Partition::Kind,
) -> Result<(metamodelica::Ref<VarData::VarData>, metamodelica::Ref<EqData::EqData>)> {
    let mut varData: metamodelica::Ref<VarData::VarData> = varData;
    let mut eqData: metamodelica::Ref<EqData::EqData> = eqData;
    (varData, eqData) = ({
        let mut new_iters: metamodelica::Ref<
            UnorderedSet::UnorderedSet<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>>,
        > = UnorderedSet::new(
            (std::sync::Arc::new(BVariable::hash)
                as std::sync::Arc<
                    dyn ::std::ops::Fn(Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>) -> Result<i32>
                        + 'static,
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
        (::match_deref::match_deref! { match &((varData.clone(), eqData.clone())) {
            (Deref @ BVariable::VarData::VAR_DATA_SIM { .. }, Deref @ BEquation::EqData::EQ_DATA_SIM { .. }) => {
                let mut replacements: metamodelica::Ref<UnorderedMap::UnorderedMap<metamodelica::Ref<ComponentRef::NFComponentRef>, metamodelica::Ref<Expression::NFExpression>>>;
                let mut newEquations: metamodelica::Ref<EquationPointers::EquationPointers>;
                let mut alias_vars: metamodelica::List<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>>;
                let mut const_vars: metamodelica::List<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>>;
                let mut non_trivial_alias: metamodelica::List<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>>;
                let mut non_trivial_eqs: metamodelica::List<Pointer::Pointer<metamodelica::Ref<Equation::Equation>>>;
                let mut auxEquations: metamodelica::List<Pointer::Pointer<metamodelica::Ref<Equation::Equation>>>;
                (replacements, newEquations) = aliasCausalize(var_field!((*varData).unknowns, VarData::VarData::VAR_DATA_SIM), var_field!((*eqData).simulation, EqData::EqData::EQ_DATA_SIM).clone(), kind, var_field!((*eqData).uniqueIndex, EqData::EqData::EQ_DATA_SIM).clone(), &(literal!("Simulation")))?;
                (replacements, auxEquations) = checkReplacements(replacements, eqData.clone())?;
                (eqData, varData) = Replacements::applySimple(eqData, varData, replacements.clone())?;
                alias_vars = ({
            let mut __acc: metamodelica::List<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>> = metamodelica::nil();
            for mut cref in (UnorderedMap::keyList(replacements)).into_iter().cloned() {
                let __x = BVariable::getVarPointer(&(cref.clone()), metamodelica::sourceInfo!("NBackEnd/Modules/2_Pre/NBAlias.mo"))?;
                __acc = cons(__x, __acc);
            }
            __acc.reverse()
        });
                for mut var in &*alias_vars {
                    BVariable::setRecordVariability(var.clone(), Variability::PARAMETER.clone());
                }
                alias_vars = List::flatten(({
            let mut __acc: metamodelica::List<metamodelica::List<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>>> = metamodelica::nil();
            for mut var in (alias_vars).into_iter().cloned() {
                let __x = BVariable::getRecordChildrenOrSelf(var.clone())?;
                __acc = cons(__x, __acc);
            }
            __acc.reverse()
        }))?;
                assign_variant_field!(eqData => EqData::EqData::EQ_DATA_SIM;
                    simulation = BEquation::EquationPointers::compress(newEquations)?,
                    equations = BEquation::EquationPointers::compress(var_field!((*eqData).equations, EqData::EqData::EQ_DATA_SIM).clone())?,
                    continuous = BEquation::EquationPointers::compress(var_field!((*eqData).continuous, EqData::EqData::EQ_DATA_SIM).clone())?,
                    discretes = BEquation::EquationPointers::compress(var_field!((*eqData).discretes, EqData::EqData::EQ_DATA_SIM).clone())?
                );
                assign_variant_field!(varData => VarData::VarData::VAR_DATA_SIM;
                    unknowns = BVariable::VariablePointers::removeList(&alias_vars, var_field!((*varData).unknowns, VarData::VarData::VAR_DATA_SIM).clone())?,
                    algebraics = BVariable::VariablePointers::removeList(&alias_vars, var_field!((*varData).algebraics, VarData::VarData::VAR_DATA_SIM).clone())?,
                    states = BVariable::VariablePointers::removeList(&alias_vars, var_field!((*varData).states, VarData::VarData::VAR_DATA_SIM).clone())?,
                    discretes = BVariable::VariablePointers::removeList(&alias_vars, var_field!((*varData).discretes, VarData::VarData::VAR_DATA_SIM).clone())?,
                    clocks = BVariable::VariablePointers::removeList(&alias_vars, var_field!((*varData).clocks, VarData::VarData::VAR_DATA_SIM).clone())?,
                    initials = BVariable::VariablePointers::removeList(&alias_vars, var_field!((*varData).initials, VarData::VarData::VAR_DATA_SIM).clone())?
                );
                (non_trivial_alias, alias_vars) = List::splitOnTrue(&alias_vars, &BVariable::hasNonTrivialAliasBinding)?;
                assign_variant_field!(varData => VarData::VarData::VAR_DATA_SIM; variables = BVariable::VariablePointers::removeList(&alias_vars, var_field!((*varData).variables, VarData::VarData::VAR_DATA_SIM).clone())?);
                (const_vars, alias_vars) = List::splitOnTrue(&alias_vars, &BVariable::hasConstOrParamAliasBinding)?;
                for mut var in &*const_vars {
                    BVariable::setVarKind(var.clone(), metamodelica::Ref::new(VariableKind::VariableKind::PARAMETER { resize_value: None }));
                    BVariable::setBindingAsStartAndFix(var.clone(), true, false)?;
                }
                assign_variant_field!(varData => VarData::VarData::VAR_DATA_SIM;
                    parameters = BVariable::VariablePointers::addList(&const_vars, var_field!((*varData).parameters, VarData::VarData::VAR_DATA_SIM).clone())?,
                    knowns = BVariable::VariablePointers::addList(&const_vars, var_field!((*varData).knowns, VarData::VarData::VAR_DATA_SIM).clone())?,
                    aliasVars = BVariable::VariablePointers::addList(&alias_vars, var_field!((*varData).aliasVars, VarData::VarData::VAR_DATA_SIM).clone())?,
                    nonTrivialAlias = BVariable::VariablePointers::addList(&non_trivial_alias, var_field!((*varData).nonTrivialAlias, VarData::VarData::VAR_DATA_SIM).clone())?
                );
                non_trivial_eqs = ({
            let mut __acc: metamodelica::List<Pointer::Pointer<metamodelica::Ref<Equation::Equation>>> = metamodelica::nil();
            for mut var in (non_trivial_alias).into_iter().cloned() {
                let __x = BEquation::Equation::generateBindingEquation(var.clone(), var_field!((*eqData).uniqueIndex, EqData::EqData::EQ_DATA_SIM).clone(), false, new_iters.clone())?;
                __acc = cons(__x, __acc);
            }
            __acc.reverse()
        });
                assign_variant_field!(eqData => EqData::EqData::EQ_DATA_SIM; removed = BEquation::EquationPointers::addList(&non_trivial_eqs, var_field!((*eqData).removed, EqData::EqData::EQ_DATA_SIM).clone())?);
                (BVariable::VarData::addTypedList(varData, &(UnorderedSet::toList(new_iters)), BVariable::VarData::VarType::ITERATOR.clone())?, BEquation::EqData::addUntypedList(eqData, &auxEquations, false)?)
            },
            _ => {
                Error::addMessage(Error::INTERNAL_ERROR.clone(), list![{ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBAlias.aliasDefault")); __mm_s.push_str(&*literal!(" failed.")); ArcStr::from(__mm_s) }])?;
                return Err("fail")
            },
            _ => unreachable!("match_deref! exhaustiveness placeholder"),
        } })
    });
    Ok((varData, eqData))
}

fn checkReplacements(
    mut replacements: metamodelica::Ref<
        UnorderedMap::UnorderedMap<
            metamodelica::Ref<ComponentRef::NFComponentRef>,
            metamodelica::Ref<Expression::NFExpression>,
        >,
    >,
    mut eqData: metamodelica::Ref<EqData::EqData>,
) -> Result<(
    metamodelica::Ref<
        UnorderedMap::UnorderedMap<
            metamodelica::Ref<ComponentRef::NFComponentRef>,
            metamodelica::Ref<Expression::NFExpression>,
        >,
    >,
    metamodelica::List<Pointer::Pointer<metamodelica::Ref<Equation::Equation>>>,
)> {
    let mut newReplacements: metamodelica::Ref<
        UnorderedMap::UnorderedMap<
            metamodelica::Ref<ComponentRef::NFComponentRef>,
            metamodelica::Ref<Expression::NFExpression>,
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
        1,
    );
    let mut auxEquations: metamodelica::List<Pointer::Pointer<metamodelica::Ref<Equation::Equation>>> =
        metamodelica::nil();
    let mut exceptionMap: metamodelica::Ref<
        UnorderedMap::UnorderedMap<metamodelica::Ref<ComponentRef::NFComponentRef>, ExceptionKind>,
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
        1,
    );
    let mut cref: metamodelica::Ref<ComponentRef::NFComponentRef>;
    let mut exp: metamodelica::Ref<Expression::NFExpression>;
    let mut eqPtr: Pointer::Pointer<metamodelica::Ref<Equation::Equation>>;
    let mut attr: metamodelica::Ref<EquationAttributes::EquationAttributes>;
    BEquation::EqData::map(
        eqData.clone(),
        &({
            let __pe_b1 = exceptionMap.clone();
            move |__pe_a0| filterExceptionsEquation(__pe_a0, __pe_b1.clone())
        }),
    )?;
    for mut keyValueTpl in &*UnorderedMap::toList(replacements) {
        (cref, exp) = keyValueTpl.clone();
        if isValidReplacement(cref.clone(), &exp, exceptionMap.clone())? {
            UnorderedMap::add(cref, exp, newReplacements.clone())?;
        } else {
            attr = BackendDAE::lowerEquationAttributes(ComponentRef::getSubscriptedType(&cref, false)?, false)?;
            eqPtr = BEquation::Equation::makeAssignment(
                Expression::fromCref(cref, false)?,
                exp,
                BEquation::EqData::getUniqueIndex(&eqData)?,
                &(literal!("SIM")),
                crate::NBEquation::Iterator::interned_EMPTY(),
                attr,
            )?;
            auxEquations = metamodelica::cons(eqPtr, auxEquations);
        }
    }
    if Flags::isSet(Flags::DUMP_REPL.clone())? {
        dumpReplacements(newReplacements.clone(), &auxEquations)?;
    }
    Ok((newReplacements, auxEquations))
}

fn isValidReplacement(
    mut cref: metamodelica::Ref<ComponentRef::NFComponentRef>,
    mut exp: &metamodelica::Ref<Expression::NFExpression>,
    mut exceptionMap: metamodelica::Ref<
        UnorderedMap::UnorderedMap<metamodelica::Ref<ComponentRef::NFComponentRef>, ExceptionKind>,
    >,
) -> Result<bool> {
    let mut b: bool = true;
    b = (::match_deref::match_deref! { match &((UnorderedMap::get(cref, exceptionMap)?, &**exp)) {
        (None, _) => true,
        (Some(ExceptionKind::CREF_ALIAS), Deref @ Expression::CREF { .. }) => true,
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(b)
}

// different kinds of exceptions
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, metamodelica::ReferenceEq)]
#[repr(i32)]
pub(crate) enum ExceptionKind {
    NO_ALIAS = 1,
    CREF_ALIAS = 2,
}
impl PartialOrd for ExceptionKind {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for ExceptionKind {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (*self as i32).cmp(&(*other as i32))
    }
}
impl metamodelica::gc::MMTrace for ExceptionKind {
    fn mm_accept(&self, _: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        Ok(())
    }
}

fn filterExceptionsEquation(
    mut eqn: metamodelica::Ref<Equation::Equation>,
    mut acc: metamodelica::Ref<
        UnorderedMap::UnorderedMap<metamodelica::Ref<ComponentRef::NFComponentRef>, ExceptionKind>,
    >,
) -> Result<metamodelica::Ref<Equation::Equation>> {
    let mut eqn: metamodelica::Ref<Equation::Equation> = eqn;
    let () = (match &*eqn {
        BEquation::Equation::ALGORITHM { alg: __eqn_alg, .. } => {
            for mut cref in &*__eqn_alg.outputs.clone() {
                UnorderedMap::add(cref.clone(), ExceptionKind::NO_ALIAS.clone(), acc.clone())?;
            }
            ()
        }
        _ => (),
    });
    BEquation::Equation::map(
        eqn.clone(),
        (std::sync::Arc::new({
            let __pe_b1 = acc;
            move |__pe_a0| filterExceptions(__pe_a0, __pe_b1.clone())
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

fn filterExceptions(
    mut exp: metamodelica::Ref<Expression::NFExpression>,
    mut acc: metamodelica::Ref<
        UnorderedMap::UnorderedMap<metamodelica::Ref<ComponentRef::NFComponentRef>, ExceptionKind>,
    >,
) -> Result<metamodelica::Ref<Expression::NFExpression>> {
    let mut exp: metamodelica::Ref<Expression::NFExpression> = exp;
    let () = (::match_deref::match_deref! { match &(&*exp) {
        Deref @ Expression::CALL { call: call @ Deref @ Call::TYPED_CALL { arguments: Deref @ metamodelica::ListNode::Cons { head: Deref @ Expression::CREF { cref, .. }, tail: Deref @ metamodelica::ListNode::Nil }, .. } } if (metamodelica::stringEq(&(AbsynUtil::pathString(Function::nameConsiderBuiltin(var_field!((**call).r#fn, Call::NFCall::TYPED_CALL)), literal!("."), true, false)?), &(literal!("pre")))) => {
            UnorderedMap::add(cref.clone(), ExceptionKind::NO_ALIAS.clone(), acc)?;
            ()
        },
        Deref @ Expression::CREF { .. } => {
            ()
        },
        Deref @ Expression::TUPLE { elements: __exp_elements, .. } => {
            for mut elem in &*__exp_elements.clone() {
                let () = (match &*elem.clone() {
        Expression::CREF { cref: __elem_cref, .. } => {
            UnorderedMap::add(__elem_cref.clone(), ExceptionKind::CREF_ALIAS.clone(), acc.clone())?;
            ()
        },
        _ => (),
    });
            }
            ()
        },
        _ => {
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(exp)
}

fn dumpReplacements(
    mut replacements: metamodelica::Ref<
        UnorderedMap::UnorderedMap<
            metamodelica::Ref<ComponentRef::NFComponentRef>,
            metamodelica::Ref<Expression::NFExpression>,
        >,
    >,
    mut auxEquations: &metamodelica::List<Pointer::Pointer<metamodelica::Ref<Equation::Equation>>>,
) -> Result<()> {
    metamodelica::print({
        let mut __mm_s = String::new();
        __mm_s.push_str(&*Replacements::simpleToString(replacements)?);
        __mm_s.push_str(&*literal!("\n"));
        ArcStr::from(__mm_s)
    });
    if !((auxEquations).is_empty()) {
        metamodelica::print(StringUtil::headline_4(
            &(literal!("[dumprepl] Found But Illegal Alias Replacements (added as equations):")),
        )?);
        for mut eqPtr in &**auxEquations {
            metamodelica::print({
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("\t"));
                __mm_s.push_str(&*BEquation::Equation::toString(
                    Pointer::access(eqPtr.clone()),
                    literal!(""),
                )?);
                __mm_s.push_str(&*literal!("\n"));
                ArcStr::from(__mm_s)
            });
        }
        metamodelica::print(literal!("\n"));
    }
    Ok(())
}

fn aliasClocks(
    mut varData: metamodelica::Ref<VarData::VarData>,
    mut eqData: metamodelica::Ref<EqData::EqData>,
    mut kind: Partition::Kind,
) -> Result<(metamodelica::Ref<VarData::VarData>, metamodelica::Ref<EqData::EqData>)> {
    let mut varData: metamodelica::Ref<VarData::VarData> = varData;
    let mut eqData: metamodelica::Ref<EqData::EqData> = eqData;
    (varData, eqData) = (::match_deref::match_deref! { match &((varData.clone(), eqData.clone())) {
        (Deref @ BVariable::VarData::VAR_DATA_SIM { .. }, Deref @ BEquation::EqData::EQ_DATA_SIM { .. }) => {
            let mut replacements: metamodelica::Ref<UnorderedMap::UnorderedMap<metamodelica::Ref<ComponentRef::NFComponentRef>, metamodelica::Ref<Expression::NFExpression>>>;
            let mut newEquations: metamodelica::Ref<EquationPointers::EquationPointers>;
            let mut alias_vars: metamodelica::List<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>>;
            let mut auxEquations: metamodelica::List<Pointer::Pointer<metamodelica::Ref<Equation::Equation>>>;
            (replacements, newEquations) = aliasCausalize(var_field!((*varData).clocks, VarData::VarData::VAR_DATA_SIM), var_field!((*eqData).clocked, EqData::EqData::EQ_DATA_SIM).clone(), kind, var_field!((*eqData).uniqueIndex, EqData::EqData::EQ_DATA_SIM).clone(), &(literal!("Clocked")))?;
            (replacements, auxEquations) = checkReplacements(replacements, eqData.clone())?;
            (eqData, varData) = Replacements::applySimple(eqData, varData, replacements.clone())?;
            alias_vars = ({
        let mut __acc: metamodelica::List<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>> = metamodelica::nil();
        for mut cref in (UnorderedMap::keyList(replacements)).into_iter().cloned() {
            let __x = BVariable::getVarPointer(&(cref.clone()), metamodelica::sourceInfo!("NBackEnd/Modules/2_Pre/NBAlias.mo"))?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
            assign_variant_field!(eqData => EqData::EqData::EQ_DATA_SIM; clocked = BEquation::EquationPointers::compress(newEquations)?);
            assign_variant_field!(varData => VarData::VarData::VAR_DATA_SIM;
                clocks = BVariable::VariablePointers::removeList(&alias_vars, var_field!((*varData).clocks, VarData::VarData::VAR_DATA_SIM).clone())?,
                aliasVars = BVariable::VariablePointers::addList(&alias_vars, var_field!((*varData).aliasVars, VarData::VarData::VAR_DATA_SIM).clone())?
            );
            (varData, BEquation::EqData::addUntypedList(eqData, &auxEquations, false)?)
        },
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![{ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBAlias.aliasClocks")); __mm_s.push_str(&*literal!(" failed.")); ArcStr::from(__mm_s) }])?;
            return Err("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((varData, eqData))
}

fn aliasCausalize(
    mut variables: &metamodelica::Ref<VariablePointers::VariablePointers>,
    mut equations: metamodelica::Ref<EquationPointers::EquationPointers>,
    mut kind: Partition::Kind,
    mut index: Pointer::Pointer<i32>,
    mut context: &ArcStr,
) -> Result<(
    metamodelica::Ref<
        UnorderedMap::UnorderedMap<
            metamodelica::Ref<ComponentRef::NFComponentRef>,
            metamodelica::Ref<Expression::NFExpression>,
        >,
    >,
    metamodelica::Ref<EquationPointers::EquationPointers>,
)> {
    let mut replacements: metamodelica::Ref<
        UnorderedMap::UnorderedMap<
            metamodelica::Ref<ComponentRef::NFComponentRef>,
            metamodelica::Ref<Expression::NFExpression>,
        >,
    >;
    let mut newEquations: metamodelica::Ref<EquationPointers::EquationPointers>;
    let mut size: i32;
    let mut setIdx: i32 = 1;
    let mut map: metamodelica::Ref<
        UnorderedMap::UnorderedMap<
            metamodelica::Ref<ComponentRef::NFComponentRef>,
            Pointer::Pointer<metamodelica::Ref<AliasSet::AliasSet>>,
        >,
    >;
    let mut sets: metamodelica::List<metamodelica::Ref<AliasSet::AliasSet>>;
    size = BVariable::VariablePointers::size(variables);
    map = UnorderedMap::new(
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
        size,
    );
    (newEquations, map) = BEquation::EquationPointers::foldRemovePtr(equations, &findSimpleEquation, map)?;
    sets = getSimpleSets(map, size)?;
    if Flags::isSet(Flags::DUMP_REPL.clone())? {
        metamodelica::print({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*StringUtil::headline_2(
                &({
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!("[dumprepl] "));
                    __mm_s.push_str(&*context);
                    __mm_s.push_str(&*literal!(" Alias Sets:"));
                    ArcStr::from(__mm_s)
                }),
            )?);
            __mm_s.push_str(&*literal!("\n"));
            ArcStr::from(__mm_s)
        });
        if (sets).is_empty() {
            metamodelica::print({
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("<No "));
                __mm_s.push_str(&*context);
                __mm_s.push_str(&*literal!(" Alias Sets>\n\n"));
                ArcStr::from(__mm_s)
            });
        } else {
            for mut set in &*sets {
                metamodelica::print({
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*StringUtil::headline_4(
                        &({
                            let mut __mm_s = String::new();
                            __mm_s.push_str(&*literal!("Alias Set "));
                            __mm_s.push_str(&*intString(setIdx));
                            __mm_s.push_str(&*literal!(":"));
                            ArcStr::from(__mm_s)
                        }),
                    )?);
                    __mm_s.push_str(&*AliasSet::toString(metamodelica::AsArg::as_arg(&set))?);
                    __mm_s.push_str(&*literal!("\n"));
                    ArcStr::from(__mm_s)
                });
                setIdx = setIdx + 1;
            }
        }
    }
    replacements = UnorderedMap::new(
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
        size,
    );
    for mut set in &*sets {
        replacements = createReplacementRules(metamodelica::AsArg::as_arg(&set), index.clone(), replacements, kind)?;
    }
    Ok((replacements, newEquations))
}

fn findSimpleEquation(
    mut eq_ptr: Pointer::Pointer<metamodelica::Ref<Equation::Equation>>,
    mut map: metamodelica::Ref<
        UnorderedMap::UnorderedMap<
            metamodelica::Ref<ComponentRef::NFComponentRef>,
            Pointer::Pointer<metamodelica::Ref<AliasSet::AliasSet>>,
        >,
    >,
) -> Result<(
    metamodelica::Ref<
        UnorderedMap::UnorderedMap<
            metamodelica::Ref<ComponentRef::NFComponentRef>,
            Pointer::Pointer<metamodelica::Ref<AliasSet::AliasSet>>,
        >,
    >,
    bool,
)> {
    let mut map: metamodelica::Ref<
        UnorderedMap::UnorderedMap<
            metamodelica::Ref<ComponentRef::NFComponentRef>,
            Pointer::Pointer<metamodelica::Ref<AliasSet::AliasSet>>,
        >,
    > = map;
    let mut delete: bool = false;
    let mut eq: metamodelica::Ref<Equation::Equation>;
    let mut crefTpl: CrefTpl = EMPTY_CREF_TPL().clone();
    eq = forToFullArrayEquation(Pointer::access(eq_ptr))?;
    crefTpl = (match &*eq {
        BEquation::Equation::SCALAR_EQUATION {
            lhs: __eq_lhs,
            rhs: __eq_rhs,
            ..
        } if ((isSimpleExp(__eq_lhs.clone(), true)).0 && (isSimpleExp(__eq_rhs.clone(), true)).0) => {
            crefTpl = Expression::fold(
                __eq_rhs.clone(),
                (std::sync::Arc::new(
                    move |__a0: metamodelica::Ref<Expression::NFExpression>, __a1: CrefTpl| findCrefs(&__a0, __a1),
                )
                    as std::sync::Arc<
                        dyn ::std::ops::Fn(metamodelica::Ref<Expression::NFExpression>, CrefTpl) -> Result<CrefTpl>
                            + 'static,
                    >),
                crefTpl,
            )?;
            crefTpl = Expression::fold(
                __eq_lhs.clone(),
                (std::sync::Arc::new(
                    move |__a0: metamodelica::Ref<Expression::NFExpression>, __a1: CrefTpl| findCrefs(&__a0, __a1),
                )
                    as std::sync::Arc<
                        dyn ::std::ops::Fn(metamodelica::Ref<Expression::NFExpression>, CrefTpl) -> Result<CrefTpl>
                            + 'static,
                    >),
                crefTpl,
            )?;
            crefTpl
        }
        BEquation::Equation::ARRAY_EQUATION {
            lhs: __eq_lhs,
            rhs: __eq_rhs,
            ..
        } if ((isSimpleExp(__eq_lhs.clone(), true)).0
            && (isSimpleExp(__eq_rhs.clone(), true)).0
            && sameArrayness(__eq_lhs.clone(), __eq_rhs.clone())) =>
        {
            crefTpl = Expression::fold(
                __eq_rhs.clone(),
                (std::sync::Arc::new(
                    move |__a0: metamodelica::Ref<Expression::NFExpression>, __a1: CrefTpl| findCrefs(&__a0, __a1),
                )
                    as std::sync::Arc<
                        dyn ::std::ops::Fn(metamodelica::Ref<Expression::NFExpression>, CrefTpl) -> Result<CrefTpl>
                            + 'static,
                    >),
                crefTpl,
            )?;
            crefTpl = Expression::fold(
                __eq_lhs.clone(),
                (std::sync::Arc::new(
                    move |__a0: metamodelica::Ref<Expression::NFExpression>, __a1: CrefTpl| findCrefs(&__a0, __a1),
                )
                    as std::sync::Arc<
                        dyn ::std::ops::Fn(metamodelica::Ref<Expression::NFExpression>, CrefTpl) -> Result<CrefTpl>
                            + 'static,
                    >),
                crefTpl,
            )?;
            crefTpl
        }
        BEquation::Equation::RECORD_EQUATION {
            lhs: __eq_lhs,
            rhs: __eq_rhs,
            ..
        } if ((isSimpleExp(__eq_lhs.clone(), true)).0 && (isSimpleExp(__eq_rhs.clone(), true)).0) => {
            crefTpl = Expression::fold(
                __eq_rhs.clone(),
                (std::sync::Arc::new(
                    move |__a0: metamodelica::Ref<Expression::NFExpression>, __a1: CrefTpl| findCrefs(&__a0, __a1),
                )
                    as std::sync::Arc<
                        dyn ::std::ops::Fn(metamodelica::Ref<Expression::NFExpression>, CrefTpl) -> Result<CrefTpl>
                            + 'static,
                    >),
                crefTpl,
            )?;
            crefTpl = Expression::fold(
                __eq_lhs.clone(),
                (std::sync::Arc::new(
                    move |__a0: metamodelica::Ref<Expression::NFExpression>, __a1: CrefTpl| findCrefs(&__a0, __a1),
                )
                    as std::sync::Arc<
                        dyn ::std::ops::Fn(metamodelica::Ref<Expression::NFExpression>, CrefTpl) -> Result<CrefTpl>
                            + 'static,
                    >),
                crefTpl,
            )?;
            crefTpl
        }
        _ => crefTpl,
    });
    (map, delete) = (::match_deref::match_deref! { match &(crefTpl) {
        CrefTpl { cr_lst: Deref @ metamodelica::ListNode::Cons { head: cr1, tail: Deref @ metamodelica::ListNode::Nil }, .. } => {
            let mut set_ptr: SetPtr;
            let mut set: metamodelica::Ref<AliasSet::AliasSet>;
            if !(UnorderedMap::contains(cr1.clone(), map.clone())?) {
                set = EMPTY_ALIAS_SET().clone();
                assign_field!(
                    set.simple_variables = list![cr1.clone()],
                    set.const_opt = Some(Pointer::create(eq))
                );
                UnorderedMap::add(cr1.clone(), Pointer::create(set), map.clone())?;
            } else {
                set_ptr = UnorderedMap::getOrFail(cr1.clone(), map.clone())?;
                set = Pointer::access(set_ptr.clone());
                if (set.const_opt).is_some() {
                    Error::addMessage(Error::INTERNAL_ERROR.clone(), list![{ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBAlias.findSimpleEquation")); __mm_s.push_str(&*literal!(" failed to add Equation:\n")); __mm_s.push_str(&*BEquation::Equation::toString(eq, literal!(""))?); __mm_s.push_str(&*literal!("\n because the set already contains a constant binding.\n              Overdetermined Set!:")); __mm_s.push_str(&*AliasSet::toString(&set)?); ArcStr::from(__mm_s) }])?;
                    return Err("fail");
                } else {
                    assign_field!(set.const_opt = Some(Pointer::create(eq)));
                    Pointer::update(set_ptr, set);
                }
            }
            (map, true)
        },
        CrefTpl { cr_lst: Deref @ metamodelica::ListNode::Cons { head: cr1, tail: Deref @ metamodelica::ListNode::Cons { head: cr2, tail: Deref @ metamodelica::ListNode::Nil } }, .. } => {
            let mut set_ptr: SetPtr;
            let mut set1_ptr: SetPtr;
            let mut set2_ptr: SetPtr;
            let mut set: metamodelica::Ref<AliasSet::AliasSet>;
            let mut set1: metamodelica::Ref<AliasSet::AliasSet>;
            let mut set2: metamodelica::Ref<AliasSet::AliasSet>;
            let mut new_eq_ptr: Pointer::Pointer<metamodelica::Ref<Equation::Equation>>;
            if UnorderedMap::contains(cr1.clone(), map.clone())? && UnorderedMap::contains(cr2.clone(), map.clone())? {
                set1_ptr = UnorderedMap::getOrFail(cr1.clone(), map.clone())?;
                set2_ptr = UnorderedMap::getOrFail(cr2.clone(), map.clone())?;
                set1 = Pointer::access(set1_ptr.clone());
                if Pointer::referenceEq(&(set1_ptr.clone()), &(set2_ptr.clone())) {
                    if (set1.const_opt).is_some() {
                        set2 = Pointer::access(set2_ptr);
                        Error::addMessage(Error::INTERNAL_ERROR.clone(), list![{ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBAlias.findSimpleEquation")); __mm_s.push_str(&*literal!(" failed to merge following sets ")); __mm_s.push_str(&*literal!("because they would create a loop and both have a constant binding. This would create an underdetermined Set!:\n\n")); __mm_s.push_str(&*literal!("Trying to merge: ")); __mm_s.push_str(&*BEquation::Equation::toString(eq.clone(), literal!(""))?); __mm_s.push_str(&*literal!("\n\n")); __mm_s.push_str(&*AliasSet::toString(&set1)?); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*AliasSet::toString(&set2)?); ArcStr::from(__mm_s) }])?;
                    }
                    new_eq_ptr = Pointer::create(eq);
                    assign_field!(set1.simple_equations = metamodelica::cons(new_eq_ptr, set1.simple_equations.clone()));
                    Pointer::update(set1_ptr, set1);
                } else {
                    set2 = Pointer::access(set2_ptr.clone());
                    set = EMPTY_ALIAS_SET().clone();
                    if (set1.const_opt).is_some() && (set2.const_opt).is_some() {
                        Error::addMessage(Error::INTERNAL_ERROR.clone(), list![{ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBAlias.findSimpleEquation")); __mm_s.push_str(&*literal!(" failed to merge following sets ")); __mm_s.push_str(&*literal!("because both have a constant binding. This would create an overdetermined Set!:\n\n")); __mm_s.push_str(&*AliasSet::toString(&set1)?); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*AliasSet::toString(&set2)?); ArcStr::from(__mm_s) }])?;
                        return Err("fail");
                    } else if (set1.const_opt).is_some() {
                        assign_field!(set.const_opt = set1.const_opt.clone());
                    } else if (set2.const_opt).is_some() {
                        assign_field!(set.const_opt = set2.const_opt.clone());
                    }
                    if List::compareLength(set1.simple_equations.clone(), set2.simple_equations.clone())? > 0 {
                        assign_field!(set.simple_equations = metamodelica::cons(Pointer::create(eq), metamodelica::Dangerous::listAppendDestroy(set2.simple_equations.clone(), set1.simple_equations.clone())));
                    } else {
                        assign_field!(set.simple_equations = metamodelica::cons(Pointer::create(eq), metamodelica::Dangerous::listAppendDestroy(set1.simple_equations.clone(), set2.simple_equations.clone())));
                    }
                    if List::compareLength(set1.simple_variables.clone(), set2.simple_variables.clone())? > 0 {
                        assign_field!(set.simple_variables = metamodelica::Dangerous::listAppendDestroy(set2.simple_variables.clone(), set1.simple_variables.clone()));
                        Pointer::update(set1_ptr.clone(), set);
                        for mut cr in &*set2.simple_variables.clone() {
                            UnorderedMap::add(cr.clone(), set1_ptr.clone(), map.clone())?;
                        }
                    } else {
                        assign_field!(set.simple_variables = metamodelica::Dangerous::listAppendDestroy(set2.simple_variables.clone(), set1.simple_variables.clone()));
                        Pointer::update(set2_ptr.clone(), set);
                        for mut cr in &*set1.simple_variables.clone() {
                            UnorderedMap::add(cr.clone(), set2_ptr.clone(), map.clone())?;
                        }
                    }
                }
            } else if UnorderedMap::contains(cr1.clone(), map.clone())? {
                set_ptr = UnorderedMap::getOrFail(cr1.clone(), map.clone())?;
                set = Pointer::access(set_ptr.clone());
                assign_field!(
                    set.simple_variables = metamodelica::cons(cr2.clone(), set.simple_variables.clone()),
                    set.simple_equations = metamodelica::cons(Pointer::create(eq), set.simple_equations.clone())
                );
                Pointer::update(set_ptr.clone(), set);
                UnorderedMap::add(cr2.clone(), set_ptr, map.clone())?;
            } else if UnorderedMap::contains(cr2.clone(), map.clone())? {
                set_ptr = UnorderedMap::getOrFail(cr2.clone(), map.clone())?;
                set = Pointer::access(set_ptr.clone());
                assign_field!(
                    set.simple_variables = metamodelica::cons(cr1.clone(), set.simple_variables.clone()),
                    set.simple_equations = metamodelica::cons(Pointer::create(eq), set.simple_equations.clone())
                );
                Pointer::update(set_ptr.clone(), set);
                UnorderedMap::add(cr1.clone(), set_ptr, map.clone())?;
            } else {
                set = EMPTY_ALIAS_SET().clone();
                assign_field!(
                    set.simple_variables = list![cr1.clone(), cr2.clone()],
                    set.simple_equations = list![Pointer::create(eq)]
                );
                set_ptr = Pointer::create(set);
                UnorderedMap::add(cr1.clone(), set_ptr.clone(), map.clone())?;
                UnorderedMap::add(cr2.clone(), set_ptr, map.clone())?;
            }
            (map, true)
        },
        _ => {
            (map, false)
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((map, delete))
}

fn forToFullArrayEquation(
    mut eq: metamodelica::Ref<Equation::Equation>,
) -> Result<metamodelica::Ref<Equation::Equation>> {
    let mut eq: metamodelica::Ref<Equation::Equation> = eq;
    let mut names: metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>>;
    let mut ranges: metamodelica::List<metamodelica::Ref<Expression::NFExpression>>;
    let mut maps: metamodelica::List<Option<metamodelica::Ref<Iterator::Iterator>>>;
    let mut iter_sizes: metamodelica::Ref<
        UnorderedMap::UnorderedMap<metamodelica::Ref<ComponentRef::NFComponentRef>, i32>,
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
        1,
    );
    let mut subs_ptr: Pointer::Pointer<Option<metamodelica::List<metamodelica::Ref<Subscript::NFSubscript>>>> =
        Pointer::create(None);
    let mut ok_ptr: Pointer::Pointer<bool> = Pointer::create(true);
    let mut ty_ptr: Pointer::Pointer<Option<metamodelica::Ref<Type::NFType>>> = Pointer::create(None);
    let mut lhs: metamodelica::Ref<Expression::NFExpression>;
    let mut rhs: metamodelica::Ref<Expression::NFExpression>;
    let mut start: i32;
    let mut step: i32;
    let mut stop: i32;
    (lhs, rhs) = (::match_deref::match_deref! { match &(&*eq) {
        Deref @ BEquation::Equation::FOR_EQUATION { body: Deref @ metamodelica::ListNode::Cons { head: Deref @ BEquation::Equation::SCALAR_EQUATION { lhs: __esc_lhs, rhs: __esc_rhs, .. }, tail: Deref @ metamodelica::ListNode::Nil }, .. } => {
            lhs = (*__esc_lhs).clone();
            rhs = (*__esc_rhs).clone();
            (lhs.clone(), rhs.clone())
        },
        Deref @ BEquation::Equation::FOR_EQUATION { body: Deref @ metamodelica::ListNode::Cons { head: Deref @ BEquation::Equation::ARRAY_EQUATION { lhs: __esc_lhs, rhs: __esc_rhs, .. }, tail: Deref @ metamodelica::ListNode::Nil }, .. } => {
            lhs = (*__esc_lhs).clone();
            rhs = (*__esc_rhs).clone();
            (lhs.clone(), rhs.clone())
        },
        _ => {
            return Ok(eq);
            (metamodelica::Ref::new(Expression::NFExpression::EMPTY { ty: openmodelica_nf_frontend::NFType::interned_UNKNOWN() }), metamodelica::Ref::new(Expression::NFExpression::EMPTY { ty: openmodelica_nf_frontend::NFType::interned_UNKNOWN() }))
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    (names, ranges, maps) = BEquation::Iterator::getFrames(&(BEquation::Equation::getForIterator(&eq)));
    for mut tpl in &*List::zip3(names, ranges, maps) {
        let _ = (::match_deref::match_deref! { match &(tpl.clone()) {
            (name, range @ Deref @ Expression::RANGE { .. }, None) if (Expression::isLiteral(metamodelica::AsArg::as_arg(&range))?) => {
                (start, step, stop) = Expression::getIntegerRange(range.clone(), false)?;
                if start != 1 || step != 1 {
                    return Ok(eq);
                }
                UnorderedMap::add(name.clone(), stop, iter_sizes.clone())?;
                ()
            },
            _ => {
                return Ok(eq);
                ()
            },
            _ => unreachable!("match_deref! exhaustiveness placeholder"),
        } });
    }
    lhs = Expression::map(
        lhs,
        (std::sync::Arc::new({
            let __pe_b1 = iter_sizes.clone();
            let __pe_b2 = subs_ptr.clone();
            let __pe_b3 = ok_ptr.clone();
            let __pe_b4 = ty_ptr.clone();
            move |__pe_a0| {
                fullVariableCref(
                    __pe_a0,
                    __pe_b1.clone(),
                    __pe_b2.clone(),
                    __pe_b3.clone(),
                    __pe_b4.clone(),
                )
            }
        })
            as std::sync::Arc<
                dyn ::std::ops::Fn(
                        metamodelica::Ref<Expression::NFExpression>,
                    ) -> Result<metamodelica::Ref<Expression::NFExpression>>
                    + 'static,
            >),
    )?;
    rhs = Expression::map(
        rhs,
        (std::sync::Arc::new({
            let __pe_b1 = iter_sizes.clone();
            let __pe_b2 = subs_ptr.clone();
            let __pe_b3 = ok_ptr.clone();
            let __pe_b4 = ty_ptr.clone();
            move |__pe_a0| {
                fullVariableCref(
                    __pe_a0,
                    __pe_b1.clone(),
                    __pe_b2.clone(),
                    __pe_b3.clone(),
                    __pe_b4.clone(),
                )
            }
        })
            as std::sync::Arc<
                dyn ::std::ops::Fn(
                        metamodelica::Ref<Expression::NFExpression>,
                    ) -> Result<metamodelica::Ref<Expression::NFExpression>>
                    + 'static,
            >),
    )?;
    if !(Pointer::access(ok_ptr)) || (Pointer::access(subs_ptr.clone())).is_none() {
        return Ok(eq);
    }
    if Expression::contains(
        lhs.clone(),
        &({
            let __pe_b1 = iter_sizes.clone();
            move |__pe_a0| isIteratorCref(&__pe_a0, __pe_b1.clone())
        }),
    )? || Expression::contains(
        rhs.clone(),
        &({
            let __pe_b1 = iter_sizes.clone();
            move |__pe_a0| isIteratorCref(&__pe_a0, __pe_b1.clone())
        }),
    )? {
        return Ok(eq);
    }
    lhs = Expression::map(
        lhs,
        (std::sync::Arc::new(Expression::repairOperator)
            as std::sync::Arc<
                dyn ::std::ops::Fn(
                        metamodelica::Ref<Expression::NFExpression>,
                    ) -> Result<metamodelica::Ref<Expression::NFExpression>>
                    + 'static,
            >),
    )?;
    rhs = Expression::map(
        rhs,
        (std::sync::Arc::new(Expression::repairOperator)
            as std::sync::Arc<
                dyn ::std::ops::Fn(
                        metamodelica::Ref<Expression::NFExpression>,
                    ) -> Result<metamodelica::Ref<Expression::NFExpression>>
                    + 'static,
            >),
    )?;
    if ((List::filterOnTrue(
        Util::getOption(Pointer::access(subs_ptr))?,
        (std::sync::Arc::new(
            move |__a0: metamodelica::Ref<Subscript::NFSubscript>| -> metamodelica::Result<_> {
                ::std::result::Result::Ok(Subscript::isIndex(&__a0))
            },
        )
            as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<Subscript::NFSubscript>) -> Result<bool> + 'static>),
    )?)
    .len() as i32)
        != UnorderedMap::size(iter_sizes)
    {
        return Ok(eq);
    }
    eq = metamodelica::Ref::new(Equation::Equation::ARRAY_EQUATION {
        ty: Util::getOption(Pointer::access(ty_ptr))?,
        lhs: lhs,
        rhs: rhs,
        source: BEquation::Equation::getSource(eq.clone()),
        attr: BEquation::Equation::getAttributes(eq),
        recordSize: None,
    });
    Ok(eq)
}

fn isIteratorCref(
    mut exp: &metamodelica::Ref<Expression::NFExpression>,
    mut iter_sizes: metamodelica::Ref<UnorderedMap::UnorderedMap<metamodelica::Ref<ComponentRef::NFComponentRef>, i32>>,
) -> Result<bool> {
    let mut b: bool;
    b = (match &**exp {
        Expression::CREF { cref: __exp_cref, .. } => UnorderedMap::contains(__exp_cref.clone(), iter_sizes)?,
        _ => false,
    });
    Ok(b)
}

fn isFullIteratorSubscript(
    mut sub: &metamodelica::Ref<Subscript::NFSubscript>,
    mut dim: &metamodelica::Ref<Dimension::NFDimension>,
    mut iter_sizes: metamodelica::Ref<UnorderedMap::UnorderedMap<metamodelica::Ref<ComponentRef::NFComponentRef>, i32>>,
) -> Result<bool> {
    let mut b: bool;
    b = (::match_deref::match_deref! { match sub {
        Deref @ Subscript::WHOLE => {
            true
        },
        Deref @ Subscript::SLICE { slice: range @ Deref @ Expression::RANGE { .. } } if (Expression::isLiteral(metamodelica::AsArg::as_arg(&range))? && Dimension::isKnown(dim, false)) => {
            let mut start: i32;
            let mut step: i32;
            let mut stop: i32;
            (start, step, stop) = Expression::getIntegerRange(range.clone(), false)?;
            start == 1 && step == 1 && stop == Dimension::size(dim, false)?
        },
        Deref @ Subscript::INDEX { index: Deref @ Expression::CREF { cref: iter, .. } } if (UnorderedMap::contains(iter.clone(), iter_sizes.clone())? && Dimension::isKnown(dim, false)) => {
            Dimension::size(dim, false)? == UnorderedMap::getSafe(iter.clone(), iter_sizes.clone(), metamodelica::sourceInfo!("NBackEnd/Modules/2_Pre/NBAlias.mo"))?
        },
        _ => {
            false
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(b)
}

fn fullVariableCref(
    mut exp: metamodelica::Ref<Expression::NFExpression>,
    mut iter_sizes: metamodelica::Ref<UnorderedMap::UnorderedMap<metamodelica::Ref<ComponentRef::NFComponentRef>, i32>>,
    mut subs_ptr: Pointer::Pointer<Option<metamodelica::List<metamodelica::Ref<Subscript::NFSubscript>>>>,
    mut ok_ptr: Pointer::Pointer<bool>,
    mut ty_ptr: Pointer::Pointer<Option<metamodelica::Ref<Type::NFType>>>,
) -> Result<metamodelica::Ref<Expression::NFExpression>> {
    let mut exp: metamodelica::Ref<Expression::NFExpression> = exp;
    let mut subs: metamodelica::List<metamodelica::Ref<Subscript::NFSubscript>>;
    let mut dims: metamodelica::List<metamodelica::Ref<Dimension::NFDimension>>;
    let mut var_ptr: Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>;
    let mut name: metamodelica::Ref<ComponentRef::NFComponentRef>;
    let mut ty: metamodelica::Ref<Type::NFType>;
    let mut ok: bool;
    if !(Pointer::access(ok_ptr.clone())) {
        return Ok(exp);
    }
    exp = (match &*exp {
        Expression::CREF { cref: __exp_cref, .. }
            if (UnorderedMap::contains(__exp_cref.clone(), iter_sizes.clone())?) =>
        {
            exp
        }
        Expression::CREF {
            cref: __exp_cref,
            ty: __exp_ty,
        } if (ComponentRef::isTime(metamodelica::AsArg::as_arg(&__exp_cref))?
            || !(ComponentRef::hasSubscripts(metamodelica::AsArg::as_arg(&__exp_cref))?)) =>
        {
            if Type::isArray(metamodelica::AsArg::as_arg(&__exp_ty)) {
                Pointer::update(ok_ptr, false);
            }
            exp
        }
        Expression::CREF { cref: __exp_cref, .. } => {
            var_ptr = BVariable::getVarPointer(
                metamodelica::AsArg::as_arg(&__exp_cref),
                metamodelica::sourceInfo!("NBackEnd/Modules/2_Pre/NBAlias.mo"),
            )?;
            name = BVariable::getVarName(var_ptr);
            ty = ComponentRef::getSubscriptedType(&name, false)?;
            dims = Type::arrayDims(ty.clone());
            subs = ComponentRef::subscriptsAllWithWholeFlat(metamodelica::AsArg::as_arg(&__exp_cref))?;
            ok = ((subs).len() as i32) == ((dims).len() as i32);
            if ok {
                ok = List::all(
                    &({
                        let mut __acc: metamodelica::List<bool> = metamodelica::nil();
                        let __thr_src0 = subs.clone();
                        let mut __thr_it0 = (&__thr_src0).into_iter();
                        let __thr_src1 = dims;
                        let mut __thr_it1 = (&__thr_src1).into_iter();
                        loop {
                            match (__thr_it0.next(), __thr_it1.next()) {
                                (Some(sub), Some(dim)) => {
                                    let __x =
                                        isFullIteratorSubscript(&(sub.clone()), &(dim.clone()), iter_sizes.clone())?;
                                    __acc = cons(__x, __acc);
                                }
                                (None, None) => break,
                                _ => return Err("threaded for: ranges of unequal length"),
                            }
                        }
                        __acc.reverse()
                    }),
                    &fnptr!(Util::id, _),
                )?;
            }
            if ok {
                ok = (::match_deref::match_deref! { match &(Pointer::access(subs_ptr.clone())) {
                    Some(subs2) => {
                        List::isEqualOnTrue(subs, subs2.clone(), &move |__a0: metamodelica::Ref<Subscript::NFSubscript>, __a1: metamodelica::Ref<Subscript::NFSubscript>| Subscript::isEqual(&__a0, &__a1))? && Type::isEqual(&ty, &(Util::getOption(Pointer::access(ty_ptr))?))?
                    },
                    _ => {
                        ok = (((List::uniqueOnTrue(&(List::filterOnTrue(subs.clone(), (std::sync::Arc::new(move |__a0: metamodelica::Ref<Subscript::NFSubscript>| -> metamodelica::Result<_> { ::std::result::Result::Ok(Subscript::isIndex(&__a0)) }) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<Subscript::NFSubscript>) -> Result<bool> + 'static>))?), &move |__a0: metamodelica::Ref<Subscript::NFSubscript>, __a1: metamodelica::Ref<Subscript::NFSubscript>| Subscript::isEqual(&__a0, &__a1))?)).len() as i32) == (((List::filterOnTrue(subs.clone(), (std::sync::Arc::new(move |__a0: metamodelica::Ref<Subscript::NFSubscript>| -> metamodelica::Result<_> { ::std::result::Result::Ok(Subscript::isIndex(&__a0)) }) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<Subscript::NFSubscript>) -> Result<bool> + 'static>))?)).len() as i32);
                        Pointer::update(subs_ptr, Some(subs));
                        Pointer::update(ty_ptr, Some(ty));
                        ok
                    },
                    _ => unreachable!("match_deref! exhaustiveness placeholder"),
                } });
            }
            Pointer::update(ok_ptr, ok);
            Expression::fromCref(name, false)?
        }
        _ => exp,
    });
    Ok(exp)
}

fn findCrefs(mut exp: &metamodelica::Ref<Expression::NFExpression>, mut tpl: CrefTpl) -> Result<CrefTpl> {
    let mut tpl: CrefTpl = tpl;
    tpl = (match &**exp {
        _ if (!(tpl.cont.clone())) => FAILED_CREF_TPL().clone(),
        Expression::CREF { cref: __exp_cref, .. }
            if (BVariable::isParamOrConst(BVariable::getVarPointer(
                metamodelica::AsArg::as_arg(&__exp_cref),
                metamodelica::sourceInfo!("NBackEnd/Modules/2_Pre/NBAlias.mo"),
            )?) || ComponentRef::isTime(metamodelica::AsArg::as_arg(&__exp_cref))?) =>
        {
            tpl.clone()
        }
        Expression::CREF { cref: __exp_cref, .. }
            if ((BVariable::getParent(BVariable::getVarPointer(
                metamodelica::AsArg::as_arg(&__exp_cref),
                metamodelica::sourceInfo!("NBackEnd/Modules/2_Pre/NBAlias.mo"),
            )?))
            .is_some()) =>
        {
            FAILED_CREF_TPL().clone()
        }
        Expression::CREF { cref: __exp_cref, .. }
            if (Variable::isTopLevelInput(
                &(Pointer::access(BVariable::getVarPointer(
                    metamodelica::AsArg::as_arg(&__exp_cref),
                    metamodelica::sourceInfo!("NBackEnd/Modules/2_Pre/NBAlias.mo"),
                )?)),
            )) =>
        {
            FAILED_CREF_TPL().clone()
        }
        Expression::CREF { cref: __exp_cref, .. }
            if (tpl.varCount.clone() < 2
                && !(ComponentRef::hasSubscripts(metamodelica::AsArg::as_arg(&__exp_cref))?)) =>
        {
            tpl.cr_lst = metamodelica::cons(__exp_cref.clone(), tpl.cr_lst.clone());
            tpl.varCount = tpl.varCount.clone() + 1;
            tpl.clone()
        }
        _ if (findCrefsFail(exp)) => FAILED_CREF_TPL().clone(),
        _ => tpl.clone(),
    });
    Ok(tpl)
}

fn findCrefsFail(mut exp: &metamodelica::Ref<Expression::NFExpression>) -> bool {
    let mut cont: bool;
    cont = (match &**exp {
        Expression::CREF { .. } => true,
        Expression::RELATION { .. } => true,
        Expression::IF { .. } => true,
        Expression::CALL { .. } => true,
        Expression::RECORD { .. } => true,
        _ => false,
    });
    cont
}

fn sameArrayness(
    mut exp1: metamodelica::Ref<Expression::NFExpression>,
    mut exp2: metamodelica::Ref<Expression::NFExpression>,
) -> bool {
    let mut same: bool =
        Type::isArray(&(Expression::typeOf(exp1.clone()))) == Type::isArray(&(Expression::typeOf(exp2.clone())));
    same
}

fn isSimpleExp(mut exp: metamodelica::Ref<Expression::NFExpression>, mut simple: bool) -> (bool, i32) {
    let mut simple: bool = simple;
    let mut num_cref: i32 = 0;
    if !(simple) {
        return (simple, num_cref);
    }
    (simple, num_cref) = (::match_deref::match_deref! { match &(&*exp) {
        Deref @ Expression::INTEGER { .. } => {
            (true, 0)
        },
        Deref @ Expression::REAL { .. } => {
            (true, 0)
        },
        Deref @ Expression::BOOLEAN { .. } => {
            (true, 0)
        },
        Deref @ Expression::STRING { .. } => {
            (true, 0)
        },
        Deref @ Expression::CREF { .. } => {
            (true, 1)
        },
        Deref @ Expression::CAST { exp: __exp_exp, .. } => {
            isSimpleExp(__exp_exp.clone(), true)
        },
        Deref @ Expression::UNARY { exp: __exp_exp, operator: __exp_operator } => {
            (simple, num_cref) = isSimpleExp(__exp_exp.clone(), true);
            simple = if (simple) {checkOp(metamodelica::AsArg::as_arg(&__exp_operator), num_cref)} else {false};
            (simple, num_cref)
        },
        Deref @ Expression::LUNARY { exp: __exp_exp, operator: __exp_operator } => {
            (simple, num_cref) = isSimpleExp(__exp_exp.clone(), true);
            simple = if (simple) {checkOp(metamodelica::AsArg::as_arg(&__exp_operator), num_cref)} else {false};
            (simple, num_cref)
        },
        Deref @ Expression::BINARY { operator: Deref @ Operator::OPERATOR { op, .. }, exp1: __exp_exp1, exp2: __exp_exp2 } => {
            let mut num_cref_tmp: i32;
            (simple, num_cref) = isSimpleExp(__exp_exp2.clone(), true);
            if op.clone() == Operator::Op::DIV.clone() && num_cref != 0 {
                simple = false;
                return (simple, num_cref);
            }
            (simple, num_cref_tmp) = isSimpleExp(__exp_exp1.clone(), simple);
            num_cref = num_cref + num_cref_tmp;
            simple = if (simple) {checkOp(var_field!((*exp).operator, Expression::NFExpression::BINARY), num_cref)} else {false};
            (simple, num_cref)
        },
        Deref @ Expression::LBINARY { exp1: __exp_exp1, exp2: __exp_exp2, operator: __exp_operator } => {
            let mut num_cref_tmp: i32;
            (simple, num_cref) = isSimpleExp(__exp_exp1.clone(), true);
            (simple, num_cref_tmp) = isSimpleExp(__exp_exp2.clone(), simple);
            num_cref = num_cref + num_cref_tmp;
            simple = if (simple) {checkOp(metamodelica::AsArg::as_arg(&__exp_operator), num_cref)} else {false};
            (simple, num_cref)
        },
        Deref @ Expression::MULTARY { operator: Deref @ Operator::OPERATOR { op, .. }, arguments: __exp_arguments, inv_arguments: __exp_inv_arguments } => {
            let mut num_cref_tmp: i32;
            for mut arg in &*__exp_inv_arguments.clone() {
                (simple, num_cref_tmp) = isSimpleExp(arg.clone(), simple);
                if !(simple) {
                    return (simple, num_cref);
                }
                num_cref = num_cref + num_cref_tmp;
            }
            if op.clone() == Operator::Op::MUL.clone() && num_cref != 0 {
                simple = false;
                return (simple, num_cref);
            }
            for mut arg in &*__exp_arguments.clone() {
                (simple, num_cref_tmp) = isSimpleExp(arg.clone(), simple);
                if !(simple) {
                    return (simple, num_cref);
                }
                num_cref = num_cref + num_cref_tmp;
            }
            simple = if (simple) {checkOp(var_field!((*exp).operator, Expression::NFExpression::MULTARY), num_cref)} else {false};
            (simple, num_cref)
        },
        _ => {
            (false, num_cref)
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    (simple, num_cref)
}

fn checkOp(mut op: &metamodelica::Ref<Operator::NFOperator>, mut cref_num: i32) -> bool {
    let mut b: bool;
    b = (match &**op {
        Operator::OPERATOR {
            op: Operator::Op::ADD, ..
        } => true,
        Operator::OPERATOR {
            op: Operator::Op::SUB, ..
        } => true,
        Operator::OPERATOR {
            op: Operator::Op::UMINUS,
            ..
        } => true,
        Operator::OPERATOR {
            op: Operator::Op::NOT, ..
        } => true,
        Operator::OPERATOR {
            op: Operator::Op::MUL, ..
        } => cref_num < 2,
        Operator::OPERATOR {
            op: Operator::Op::DIV, ..
        } => cref_num < 2,
        _ => cref_num == 0,
    });
    b
}

fn getSimpleSets(
    mut map: metamodelica::Ref<
        UnorderedMap::UnorderedMap<
            metamodelica::Ref<ComponentRef::NFComponentRef>,
            Pointer::Pointer<metamodelica::Ref<AliasSet::AliasSet>>,
        >,
    >,
    mut size: i32,
) -> Result<metamodelica::List<metamodelica::Ref<AliasSet::AliasSet>>> {
    let mut sets: metamodelica::List<metamodelica::Ref<AliasSet::AliasSet>> = metamodelica::nil();
    let mut cref_marks: metamodelica::Ref<UnorderedSet::UnorderedSet<metamodelica::Ref<ComponentRef::NFComponentRef>>> =
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
            size,
        );
    let mut entry_lst: metamodelica::List<(
        metamodelica::Ref<ComponentRef::NFComponentRef>,
        Pointer::Pointer<metamodelica::Ref<AliasSet::AliasSet>>,
    )>;
    let mut simple_cref: metamodelica::Ref<ComponentRef::NFComponentRef>;
    let mut set_ptr: SetPtr;
    let mut set: metamodelica::Ref<AliasSet::AliasSet>;
    entry_lst = UnorderedMap::toList(map);
    for mut entry in &*entry_lst {
        (simple_cref, set_ptr) = entry.clone();
        if !(UnorderedSet::contains(simple_cref, cref_marks.clone())?) {
            set = Pointer::access(set_ptr);
            sets = metamodelica::cons(set.clone(), sets);
            for mut cr in &*set.simple_variables.clone() {
                if '__try0: {
                    unwrap_break_err!(UnorderedSet::addUnique(cr.clone(), cref_marks.clone()), '__try0);
                    Ok::<(), &'static str>(())
                }
                .is_err()
                {
                    Error::addMessage(
                        Error::INTERNAL_ERROR.clone(),
                        list![{
                            let mut __mm_s = String::new();
                            __mm_s.push_str(&*literal!("NBAlias.getSimpleSets"));
                            __mm_s.push_str(&*literal!(" failed because the set for "));
                            __mm_s.push_str(&*ComponentRef::toString(metamodelica::AsArg::as_arg(&cr))?);
                            __mm_s.push_str(&*literal!(" was already added."));
                            ArcStr::from(__mm_s)
                        }],
                    )?;
                }
            }
        }
    }
    Ok(sets)
}

fn createReplacementRules(
    mut set: &metamodelica::Ref<AliasSet::AliasSet>,
    mut index: Pointer::Pointer<i32>,
    mut replacements: metamodelica::Ref<
        UnorderedMap::UnorderedMap<
            metamodelica::Ref<ComponentRef::NFComponentRef>,
            metamodelica::Ref<Expression::NFExpression>,
        >,
    >,
    mut kind: Partition::Kind,
) -> Result<
    metamodelica::Ref<
        UnorderedMap::UnorderedMap<
            metamodelica::Ref<ComponentRef::NFComponentRef>,
            metamodelica::Ref<Expression::NFExpression>,
        >,
    >,
> {
    let mut replacements: metamodelica::Ref<
        UnorderedMap::UnorderedMap<
            metamodelica::Ref<ComponentRef::NFComponentRef>,
            metamodelica::Ref<Expression::NFExpression>,
        >,
    > = replacements;
    replacements = ({
        let mut var_to_keep: Pointer::Pointer<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>> =
            Pointer::create(Pointer::create(BVariable::DUMMY_VARIABLE().clone()));
        let mut idx_i: i32 = 1;
        (match set.const_opt.clone() {
            Some(mut const_eq) => {
                let mut vars: metamodelica::Ref<VariablePointers::VariablePointers>;
                let mut eqs: metamodelica::Ref<EquationPointers::EquationPointers>;
                let mut comps: metamodelica::List<metamodelica::Ref<StrongComponent::NBStrongComponent>>;
                vars = BVariable::VariablePointers::fromList(
                    &({
                        let mut __acc: metamodelica::List<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>> =
                            metamodelica::nil();
                        for mut cr in (set.simple_variables.clone()).into_iter().cloned() {
                            let __x = BVariable::getVarPointer(
                                &(cr.clone()),
                                metamodelica::sourceInfo!("NBackEnd/Modules/2_Pre/NBAlias.mo"),
                            )?;
                            __acc = cons(__x, __acc);
                        }
                        __acc.reverse()
                    }),
                    true,
                )?;
                eqs = BEquation::EquationPointers::fromList(
                    &(metamodelica::cons(const_eq, set.simple_equations.clone())),
                )?;
                (_, comps) = Causalize::simple(
                    &vars,
                    &eqs,
                    kind,
                    NBAdjacency::MatrixStrictness::MATCHING.clone(),
                    &(crate::NBEquation::Iterator::interned_EMPTY()),
                )?;
                Replacements::simple(&comps, replacements.clone())?;
                replacements
            }
            None if (((set.simple_variables).len() as i32) == ((set.simple_equations).len() as i32)) => {
                let mut vars: metamodelica::Ref<VariablePointers::VariablePointers>;
                let mut eqs: metamodelica::Ref<EquationPointers::EquationPointers>;
                let mut eqns: metamodelica::List<Pointer::Pointer<metamodelica::Ref<Equation::Equation>>>;
                let mut comps: metamodelica::List<metamodelica::Ref<StrongComponent::NBStrongComponent>>;
                vars = BVariable::VariablePointers::fromList(
                    &({
                        let mut __acc: metamodelica::List<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>> =
                            metamodelica::nil();
                        for mut cr in (set.simple_variables.clone()).into_iter().cloned() {
                            let __x = BVariable::getVarPointer(
                                &(cr.clone()),
                                metamodelica::sourceInfo!("NBackEnd/Modules/2_Pre/NBAlias.mo"),
                            )?;
                            __acc = cons(__x, __acc);
                        }
                        __acc.reverse()
                    }),
                    true,
                )?;
                eqns = ASSC::main(&set.simple_equations, set.simple_variables.clone(), index)?;
                eqs = BEquation::EquationPointers::fromList(&eqns)?;
                (_, comps) = Causalize::simple(
                    &vars,
                    &eqs,
                    kind,
                    NBAdjacency::MatrixStrictness::MATCHING.clone(),
                    &(crate::NBEquation::Iterator::interned_EMPTY()),
                )?;
                Replacements::simple(&comps, replacements.clone())?;
                replacements
            }
            _ => {
                let mut rhs: metamodelica::Ref<Expression::NFExpression>;
                let mut solved_eq: metamodelica::Ref<Equation::Equation>;
                let mut eq: Pointer::Pointer<metamodelica::Ref<Equation::Equation>>;
                let mut alias_vars: metamodelica::List<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>>;
                let mut vars: metamodelica::Ref<VariablePointers::VariablePointers>;
                let mut var_lst: metamodelica::List<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>>;
                let mut eqs: metamodelica::Ref<EquationPointers::EquationPointers>;
                let mut comps: metamodelica::List<metamodelica::Ref<StrongComponent::NBStrongComponent>>;
                let mut collector: metamodelica::Ref<AttributeCollector::AttributeCollector>;
                let mut status: Status;
                (alias_vars, collector) = chooseVariableToKeep(
                    &({
                        let mut __acc: metamodelica::List<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>> =
                            metamodelica::nil();
                        for mut cr in (set.simple_variables.clone()).into_iter().cloned() {
                            let __x = BVariable::getVarPointer(
                                &(cr.clone()),
                                metamodelica::sourceInfo!("NBackEnd/Modules/2_Pre/NBAlias.mo"),
                            )?;
                            __acc = cons(__x, __acc);
                        }
                        __acc.reverse()
                    }),
                    var_to_keep.clone(),
                )?;
                vars = BVariable::VariablePointers::fromList(&alias_vars, false)?;
                eqs = BEquation::EquationPointers::fromList(&set.simple_equations)?;
                (_, comps) = Causalize::simple(
                    &vars,
                    &eqs,
                    kind,
                    NBAdjacency::MatrixStrictness::MATCHING.clone(),
                    &(crate::NBEquation::Iterator::interned_EMPTY()),
                )?;
                if Flags::isSet(Flags::DEBUG_ALIAS.clone())? {
                    metamodelica::print({
                        let mut __mm_s = String::new();
                        __mm_s.push_str(&*StringUtil::headline_3(
                            &(literal!("Variable to keep (values of attributes before replacements):")),
                        )?);
                        __mm_s.push_str(&*BVariable::pointerToString(Pointer::access(var_to_keep.clone()))?);
                        __mm_s.push_str(&*literal!("\n\n"));
                        ArcStr::from(__mm_s)
                    });
                }
                Replacements::simple(&comps, replacements.clone())?;
                var_lst = BVariable::VariablePointers::toList(&vars)?;
                if Flags::isSet(Flags::DEBUG_ALIAS.clone())? {
                    metamodelica::print({
                        let mut __mm_s = String::new();
                        __mm_s.push_str(&*StringUtil::headline_4(
                            &(literal!("Attribute collector (before replacements): ")),
                        )?);
                        __mm_s.push_str(&*AttributeCollector::toString(&collector, literal!(""))?);
                        __mm_s.push_str(&*literal!("\n"));
                        ArcStr::from(__mm_s)
                    });
                }
                for mut var in &*var_lst {
                    rhs = UnorderedMap::getSafe(
                        BVariable::getVarName(var.clone()),
                        replacements.clone(),
                        metamodelica::sourceInfo!("NBackEnd/Modules/2_Pre/NBAlias.mo"),
                    )?;
                    eq = BEquation::Equation::makeAssignment(
                        BVariable::toExpression(var.clone())?,
                        rhs,
                        index.clone(),
                        &(arcstr::literal!(BEquation::TMP_STR)),
                        crate::NBEquation::Iterator::interned_EMPTY(),
                        BEquation::default(EquationKind::UNKNOWN.clone(), false, None, None),
                    )?;
                    (solved_eq, status, _) = Solve::solveBody(
                        Pointer::access(eq),
                        BVariable::getVarName(Pointer::access(var_to_keep.clone())),
                        UnorderedMap::new(
                            (std::sync::Arc::new(
                                move |__a0: metamodelica::Ref<Absyn::Path>| -> metamodelica::Result<_> {
                                    ::std::result::Result::Ok(AbsynUtil::pathHash(&__a0))
                                },
                            )
                                as std::sync::Arc<
                                    dyn ::std::ops::Fn(metamodelica::Ref<Absyn::Path>) -> Result<i32> + 'static,
                                >),
                            (std::sync::Arc::new(
                                move |__a0: metamodelica::Ref<Absyn::Path>,
                                      __a1: metamodelica::Ref<Absyn::Path>|
                                      -> metamodelica::Result<_> {
                                    ::std::result::Result::Ok(AbsynUtil::pathEqual(&__a0, &__a1))
                                },
                            )
                                as std::sync::Arc<
                                    dyn ::std::ops::Fn(
                                            metamodelica::Ref<Absyn::Path>,
                                            metamodelica::Ref<Absyn::Path>,
                                        ) -> Result<bool>
                                        + 'static,
                                >),
                            1,
                        ),
                    )?;
                    collector =
                        AttributeCollector::fixValues(collector, BVariable::getVarName(var.clone()), solved_eq)?;
                }
                if Flags::isSet(Flags::DEBUG_ALIAS.clone())? {
                    metamodelica::print({
                        let mut __mm_s = String::new();
                        __mm_s.push_str(&*StringUtil::headline_4(
                            &(literal!("Attribute collector (after replacements): ")),
                        )?);
                        __mm_s.push_str(&*AttributeCollector::toString(&collector, literal!(""))?);
                        __mm_s.push_str(&*literal!("\n"));
                        ArcStr::from(__mm_s)
                    });
                }
                diffTearingSelect(collector.tearingSelect_map.clone(), set)?;
                stateSelectAlways(collector.stateSelect_map.clone(), set)?;
                checkNominalThreshold(collector.nominal_map.clone(), set)?;
                setNewAttributes(var_to_keep.clone(), &collector, set)?;
                if Flags::isSet(Flags::DEBUG_ALIAS.clone())? {
                    metamodelica::print({
                        let mut __mm_s = String::new();
                        __mm_s.push_str(&*StringUtil::headline_3(
                            &(literal!("Variable to keep (values of attributes after replacements):")),
                        )?);
                        __mm_s.push_str(&*BVariable::pointerToString(Pointer::access(var_to_keep))?);
                        __mm_s.push_str(&*literal!("\n"));
                        ArcStr::from(__mm_s)
                    });
                }
                replacements
            }
        })
    });
    Ok(replacements)
}

fn setNewAttributes(
    mut var_to_keep_ptr: Pointer::Pointer<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>>,
    mut attrcollector: &metamodelica::Ref<AttributeCollector::AttributeCollector>,
    mut set: &metamodelica::Ref<AliasSet::AliasSet>,
) -> Result<()> {
    let mut new_cref: Option<metamodelica::Ref<ComponentRef::NFComponentRef>>;
    let mut new_min: Option<metamodelica::Ref<Expression::NFExpression>>;
    let mut new_max: Option<metamodelica::Ref<Expression::NFExpression>>;
    let mut new_start: Option<metamodelica::Ref<Expression::NFExpression>>;
    let mut new_stateSelect: Option<StateSelect>;
    let mut new_tearingSelect: Option<TearingSelect>;
    let mut fixed_var: Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>;
    let mut var_to_keep: Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>> =
        Pointer::access(var_to_keep_ptr.clone());
    let mut fixed_start_map: metamodelica::Ref<
        UnorderedMap::UnorderedMap<
            metamodelica::Ref<ComponentRef::NFComponentRef>,
            metamodelica::Ref<Expression::NFExpression>,
        >,
    >;
    new_min = getMaximum(attrcollector.min_val_map.clone())?;
    if (new_min).is_some() {
        Pointer::update(
            var_to_keep.clone(),
            BVariable::setMin(Pointer::access(var_to_keep.clone()), new_min.clone(), true)?,
        );
        UnorderedMap::add(
            BVariable::getVarName(var_to_keep.clone()),
            Util::getOption(new_min)?,
            attrcollector.min_val_map.clone(),
        )?;
    }
    new_max = getMinimum(attrcollector.max_val_map.clone())?;
    if (new_max).is_some() {
        Pointer::update(
            var_to_keep.clone(),
            BVariable::setMax(Pointer::access(var_to_keep.clone()), new_max.clone(), true)?,
        );
        UnorderedMap::add(
            BVariable::getVarName(var_to_keep.clone()),
            Util::getOption(new_max)?,
            attrcollector.max_val_map.clone(),
        )?;
    }
    fixed_start_map = setStartFixed(attrcollector.start_map.clone(), attrcollector.fixed_map.clone(), set)?;
    if UnorderedMap::isEmpty(fixed_start_map.clone()) && !(UnorderedMap::isEmpty(attrcollector.start_map.clone())) {
        new_cref = selectStartByConfidence(
            attrcollector.start_map.clone(),
            attrcollector.start_binding_map.clone(),
            set,
        )?;
        if (new_cref).is_some() {
            new_start = Some(UnorderedMap::getSafe(
                Util::getOption(new_cref)?,
                attrcollector.start_map.clone(),
                metamodelica::sourceInfo!("NBackEnd/Modules/2_Pre/NBAlias.mo"),
            )?);
            Pointer::update(
                var_to_keep.clone(),
                BVariable::setStartAttribute(
                    Pointer::access(var_to_keep.clone()),
                    Util::getOption(new_start.clone())?,
                    true,
                )?,
            );
            UnorderedMap::add(
                BVariable::getVarName(var_to_keep.clone()),
                Util::getOption(new_start)?,
                attrcollector.start_map.clone(),
            )?;
        }
    } else if UnorderedMap::size(fixed_start_map.clone()) == 1 {
        new_start = Some((UnorderedMap::valueList(fixed_start_map.clone())).head().cloned()?);
        fixed_var = BVariable::getVarPointer(
            &(UnorderedMap::firstKey(fixed_start_map)?),
            metamodelica::sourceInfo!("NBackEnd/Modules/2_Pre/NBAlias.mo"),
        )?;
        BVariable::setFixed(fixed_var.clone(), false, true)?;
        UnorderedMap::add(
            BVariable::getVarName(fixed_var),
            metamodelica::Ref::new(Expression::NFExpression::BOOLEAN { value: false }),
            attrcollector.fixed_map.clone(),
        )?;
        BVariable::setFixed(var_to_keep.clone(), true, true)?;
        UnorderedMap::add(
            BVariable::getVarName(var_to_keep.clone()),
            metamodelica::Ref::new(Expression::NFExpression::BOOLEAN { value: true }),
            attrcollector.fixed_map.clone(),
        )?;
        Pointer::update(
            var_to_keep.clone(),
            BVariable::setStartAttribute(
                Pointer::access(var_to_keep.clone()),
                Util::getOption(new_start.clone())?,
                true,
            )?,
        );
        UnorderedMap::add(
            BVariable::getVarName(var_to_keep.clone()),
            Util::getOption(new_start)?,
            attrcollector.start_map.clone(),
        )?;
    }
    (new_cref, new_stateSelect) = chooseStateSelect(attrcollector.stateSelect_map.clone())?;
    if (new_stateSelect).is_some()
        && (UnorderedMap::get(
            BVariable::getVarName(var_to_keep.clone()),
            attrcollector.stateSelect_map.clone(),
        )?)
        .is_some()
    {
        Pointer::update(
            var_to_keep.clone(),
            BVariable::setStateSelect(
                Pointer::access(var_to_keep.clone()),
                Util::getOption(new_stateSelect.clone())?,
                true,
            )?,
        );
        UnorderedMap::add(
            BVariable::getVarName(var_to_keep.clone()),
            Util::getOption(new_stateSelect.clone())?,
            attrcollector.stateSelect_map.clone(),
        )?;
        if Util::getOption(new_stateSelect)? == StateSelect::ALWAYS.clone() {
            new_start = Some(UnorderedMap::getSafe(
                Util::getOption(new_cref)?,
                attrcollector.start_map.clone(),
                metamodelica::sourceInfo!("NBackEnd/Modules/2_Pre/NBAlias.mo"),
            )?);
            Pointer::update(
                var_to_keep.clone(),
                BVariable::setStartAttribute(
                    Pointer::access(var_to_keep.clone()),
                    Util::getOption(new_start.clone())?,
                    true,
                )?,
            );
            UnorderedMap::add(
                BVariable::getVarName(var_to_keep.clone()),
                Util::getOption(new_start)?,
                attrcollector.start_map.clone(),
            )?;
        }
    }
    new_tearingSelect = chooseTearingSelect(attrcollector.tearingSelect_map.clone())?;
    if (new_tearingSelect).is_some()
        && (UnorderedMap::get(
            BVariable::getVarName(var_to_keep.clone()),
            attrcollector.tearingSelect_map.clone(),
        )?)
        .is_some()
    {
        Pointer::update(
            var_to_keep.clone(),
            BVariable::setTearingSelect(
                Pointer::access(var_to_keep.clone()),
                Util::getOption(new_tearingSelect.clone())?,
                true,
            ),
        );
        UnorderedMap::add(
            BVariable::getVarName(var_to_keep.clone()),
            Util::getOption(new_tearingSelect)?,
            attrcollector.tearingSelect_map.clone(),
        )?;
    }
    Pointer::update(var_to_keep_ptr, var_to_keep);
    Ok(())
}

fn chooseVariableToKeep(
    mut var_lst: &metamodelica::List<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>>,
    mut var_to_keep: Pointer::Pointer<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>>,
) -> Result<(
    metamodelica::List<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>>,
    metamodelica::Ref<AttributeCollector::AttributeCollector>,
)> {
    let mut acc: metamodelica::List<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>> = metamodelica::nil();
    let mut attrcollector: metamodelica::Ref<AttributeCollector::AttributeCollector> =
        metamodelica::Ref::new(AttributeCollector::AttributeCollector {
            min_val_map: UnorderedMap::new(
                (std::sync::Arc::new(move |__a0: metamodelica::Ref<ComponentRef::NFComponentRef>| {
                    ComponentRef::hash(&__a0)
                })
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
            max_val_map: UnorderedMap::new(
                (std::sync::Arc::new(move |__a0: metamodelica::Ref<ComponentRef::NFComponentRef>| {
                    ComponentRef::hash(&__a0)
                })
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
            start_map: UnorderedMap::new(
                (std::sync::Arc::new(move |__a0: metamodelica::Ref<ComponentRef::NFComponentRef>| {
                    ComponentRef::hash(&__a0)
                })
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
            start_binding_map: UnorderedMap::new(
                (std::sync::Arc::new(move |__a0: metamodelica::Ref<ComponentRef::NFComponentRef>| {
                    ComponentRef::hash(&__a0)
                })
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
            fixed_map: UnorderedMap::new(
                (std::sync::Arc::new(move |__a0: metamodelica::Ref<ComponentRef::NFComponentRef>| {
                    ComponentRef::hash(&__a0)
                })
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
            nominal_map: UnorderedMap::new(
                (std::sync::Arc::new(move |__a0: metamodelica::Ref<ComponentRef::NFComponentRef>| {
                    ComponentRef::hash(&__a0)
                })
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
            stateSelect_map: UnorderedMap::new(
                (std::sync::Arc::new(move |__a0: metamodelica::Ref<ComponentRef::NFComponentRef>| {
                    ComponentRef::hash(&__a0)
                })
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
            tearingSelect_map: UnorderedMap::new(
                (std::sync::Arc::new(move |__a0: metamodelica::Ref<ComponentRef::NFComponentRef>| {
                    ComponentRef::hash(&__a0)
                })
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
    let mut var: Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>;
    let mut rest: metamodelica::List<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>>;
    let mut cur_rating: i32;
    let mut max_rating: i32;
    let (__pa0, __pa1) = ::match_deref::match_deref! { match &((*var_lst)) {
        Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
        _ => return Err("pattern mismatch"),
    } };
    var = metamodelica::Own::own(__pa0);
    rest = metamodelica::Own::own(__pa1);
    Pointer::update(var_to_keep.clone(), var.clone());
    (max_rating, attrcollector) = rateVar(var, attrcollector)?;
    for mut var in &*rest {
        let mut var = var.clone();
        (cur_rating, attrcollector) = rateVar(var.clone(), attrcollector)?;
        if cur_rating > max_rating {
            max_rating = cur_rating;
            acc = metamodelica::cons(Pointer::access(var_to_keep.clone()), acc);
            Pointer::update(var_to_keep.clone(), var);
        } else {
            acc = metamodelica::cons(var, acc);
        }
    }
    Ok((acc, attrcollector))
}

fn getMaximum(
    mut map: metamodelica::Ref<
        UnorderedMap::UnorderedMap<
            metamodelica::Ref<ComponentRef::NFComponentRef>,
            metamodelica::Ref<Expression::NFExpression>,
        >,
    >,
) -> Result<Option<metamodelica::Ref<Expression::NFExpression>>> {
    let mut max_exp: Option<metamodelica::Ref<Expression::NFExpression>>;
    let mut constants: metamodelica::List<metamodelica::Ref<Expression::NFExpression>>;
    let mut rest: metamodelica::List<metamodelica::Ref<Expression::NFExpression>>;
    let mut lst_values: metamodelica::List<metamodelica::Ref<Expression::NFExpression>> =
        UnorderedMap::valueList(map.clone());
    let mut max_val: metamodelica::Real;
    (constants, rest) = List::splitOnTrue(&lst_values, &move |__a0: metamodelica::Ref<
        Expression::NFExpression,
    >|
          -> metamodelica::Result<_> {
        ::std::result::Result::Ok(Expression::isConstNumber(&__a0))
    })?;
    if !((constants).is_empty()) {
        max_val = List::maxElement(
            &({
                let mut __acc: metamodelica::List<metamodelica::Real> = metamodelica::nil();
                for mut val in (constants).into_iter().cloned() {
                    let __x = Expression::realValue(&(val.clone()))?;
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            }),
            &fnptr!(realLt, metamodelica::Real, metamodelica::Real),
        )?;
        rest = metamodelica::cons(
            metamodelica::Ref::new(Expression::NFExpression::REAL { value: max_val }),
            rest,
        );
    }
    rest = List::uniqueOnTrue(&rest, &Expression::isEqual)?;
    if (rest).is_empty() {
        max_exp = None;
    } else if List::hasOneElement(&rest) {
        max_exp = Some((rest).head().cloned()?);
    } else {
        max_exp = Some(makeBoundCall(NFBuiltinFuncs::MAX_REAL().clone(), rest)?);
    }
    Ok(max_exp)
}

fn getMinimum(
    mut map: metamodelica::Ref<
        UnorderedMap::UnorderedMap<
            metamodelica::Ref<ComponentRef::NFComponentRef>,
            metamodelica::Ref<Expression::NFExpression>,
        >,
    >,
) -> Result<Option<metamodelica::Ref<Expression::NFExpression>>> {
    let mut min_exp: Option<metamodelica::Ref<Expression::NFExpression>>;
    let mut constants: metamodelica::List<metamodelica::Ref<Expression::NFExpression>>;
    let mut rest: metamodelica::List<metamodelica::Ref<Expression::NFExpression>>;
    let mut lst_values: metamodelica::List<metamodelica::Ref<Expression::NFExpression>> =
        UnorderedMap::valueList(map.clone());
    let mut min_val: metamodelica::Real;
    (constants, rest) = List::splitOnTrue(&lst_values, &move |__a0: metamodelica::Ref<
        Expression::NFExpression,
    >|
          -> metamodelica::Result<_> {
        ::std::result::Result::Ok(Expression::isConstNumber(&__a0))
    })?;
    if !((constants).is_empty()) {
        min_val = List::minElement(
            &({
                let mut __acc: metamodelica::List<metamodelica::Real> = metamodelica::nil();
                for mut val in (constants).into_iter().cloned() {
                    let __x = Expression::realValue(&(val.clone()))?;
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            }),
            &fnptr!(realLt, metamodelica::Real, metamodelica::Real),
        )?;
        rest = metamodelica::cons(
            metamodelica::Ref::new(Expression::NFExpression::REAL { value: min_val }),
            rest,
        );
    }
    rest = List::uniqueOnTrue(&rest, &Expression::isEqual)?;
    if (rest).is_empty() {
        min_exp = None;
    } else if List::hasOneElement(&rest) {
        min_exp = Some((rest).head().cloned()?);
    } else {
        min_exp = Some(makeBoundCall(NFBuiltinFuncs::MIN_REAL().clone(), rest)?);
    }
    Ok(min_exp)
}

fn makeBoundCall(
    mut r#fn: metamodelica::Ref<Function::Function>,
    mut args: metamodelica::List<metamodelica::Ref<Expression::NFExpression>>,
) -> Result<metamodelica::Ref<Expression::NFExpression>> {
    let mut exp: metamodelica::Ref<Expression::NFExpression>;
    let mut call_args: metamodelica::List<metamodelica::Ref<Expression::NFExpression>> = args.clone();
    let mut dims: metamodelica::List<metamodelica::Ref<Dimension::NFDimension>> = metamodelica::nil();
    let mut iters: metamodelica::List<(
        metamodelica::Ref<InstNode::InstNode>,
        metamodelica::Ref<Expression::NFExpression>,
    )> = metamodelica::nil();
    let mut iter: metamodelica::Ref<InstNode::InstNode>;
    let mut sub: metamodelica::Ref<Subscript::NFSubscript>;
    for mut arg in &*args {
        if Expression::hasArrayType(arg.clone()) {
            dims = Type::arrayDims(Expression::typeOf(arg.clone()));
            break;
        }
    }
    for mut dim in &*dims {
        iter = InstNode::newUniqueIterator(
            metamodelica::sourceInfo!("NBackEnd/Modules/2_Pre/NBAlias.mo"),
            openmodelica_nf_frontend::NFType::interned_INTEGER(),
        );
        iters = metamodelica::cons(
            (
                iter.clone(),
                metamodelica::Ref::new(Expression::NFExpression::RANGE {
                    ty: metamodelica::Ref::new(Type::NFType::ARRAY {
                        elementType: openmodelica_nf_frontend::NFType::interned_INTEGER(),
                        dimensions: list![dim.clone()],
                    }),
                    start: metamodelica::Ref::new(Expression::NFExpression::INTEGER { value: 1 }),
                    step: None,
                    stop: Dimension::sizeExp(metamodelica::AsArg::as_arg(&dim))?,
                }),
            ),
            iters,
        );
        sub = metamodelica::Ref::new(Subscript::NFSubscript::INDEX {
            index: metamodelica::Ref::new(Expression::NFExpression::CREF {
                ty: openmodelica_nf_frontend::NFType::interned_INTEGER(),
                cref: ComponentRef::makeIterator(iter, openmodelica_nf_frontend::NFType::interned_INTEGER())?,
            }),
        });
        call_args = ({
            let mut __acc: metamodelica::List<metamodelica::Ref<Expression::NFExpression>> = metamodelica::nil();
            for mut a in (call_args).into_iter().cloned() {
                let __x = if (Expression::hasArrayType(a.clone())) {
                    Expression::applySubscript(&sub, &(a.clone()), &(metamodelica::nil()), false)?
                } else {
                    a.clone()
                };
                __acc = cons(__x, __acc);
            }
            __acc.reverse()
        });
    }
    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(call_args.reverse()) {
        Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
        _ => return Err("pattern mismatch"),
    } };
    exp = metamodelica::Own::own(__pa0);
    call_args = metamodelica::Own::own(__pa1);
    for mut arg in &*call_args {
        exp = metamodelica::Ref::new(Expression::NFExpression::CALL {
            call: Call::makeTypedCall(
                r#fn.clone(),
                list![arg.clone(), exp],
                Variability::PARAMETER.clone(),
                NFPrefixes::Purity::PURE.clone(),
                r#fn.returnType.clone(),
            ),
        });
    }
    if !((dims).is_empty()) {
        exp = metamodelica::Ref::new(Expression::NFExpression::CALL {
            call: metamodelica::Ref::new(Call::NFCall::TYPED_ARRAY_CONSTRUCTOR {
                ty: Type::liftArrayLeftList(Expression::typeOf(exp.clone()), &dims),
                var: Variability::PARAMETER.clone(),
                purity: NFPrefixes::Purity::PURE.clone(),
                exp: exp,
                iters: iters,
            }),
        });
    }
    Ok(exp)
}

fn setStartFixed(
    mut start_map: metamodelica::Ref<
        UnorderedMap::UnorderedMap<
            metamodelica::Ref<ComponentRef::NFComponentRef>,
            metamodelica::Ref<Expression::NFExpression>,
        >,
    >,
    mut fixed_map: metamodelica::Ref<
        UnorderedMap::UnorderedMap<
            metamodelica::Ref<ComponentRef::NFComponentRef>,
            metamodelica::Ref<Expression::NFExpression>,
        >,
    >,
    mut set: &metamodelica::Ref<AliasSet::AliasSet>,
) -> Result<
    metamodelica::Ref<
        UnorderedMap::UnorderedMap<
            metamodelica::Ref<ComponentRef::NFComponentRef>,
            metamodelica::Ref<Expression::NFExpression>,
        >,
    >,
> {
    let mut fixed_start_map: metamodelica::Ref<
        UnorderedMap::UnorderedMap<
            metamodelica::Ref<ComponentRef::NFComponentRef>,
            metamodelica::Ref<Expression::NFExpression>,
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
        1,
    );
    let mut fixed_lst: metamodelica::List<(
        metamodelica::Ref<ComponentRef::NFComponentRef>,
        metamodelica::Ref<Expression::NFExpression>,
    )> = UnorderedMap::toList(fixed_map.clone());
    let mut fixed_start_lst: metamodelica::List<metamodelica::Ref<Expression::NFExpression>>;
    let mut count_fixed: i32 = 0;
    let mut cref: metamodelica::Ref<ComponentRef::NFComponentRef>;
    let mut sval: metamodelica::Ref<Expression::NFExpression>;
    let mut fval: metamodelica::Ref<Expression::NFExpression>;
    for mut tpl in &*fixed_lst {
        (cref, fval) = tpl.clone();
        if Expression::isTrue(&fval) {
            count_fixed = count_fixed + 1;
            sval = UnorderedMap::getSafe(
                cref.clone(),
                start_map.clone(),
                metamodelica::sourceInfo!("NBackEnd/Modules/2_Pre/NBAlias.mo"),
            )?;
            UnorderedMap::add(cref, sval, fixed_start_map.clone())?;
        }
    }
    if count_fixed > 1 {
        fixed_start_lst = UnorderedMap::valueList(fixed_start_map.clone());
        if !(List::allEqual(&fixed_start_lst, &Expression::isEqual)?) {
            if List::all(&fixed_start_lst, &move |__a0: metamodelica::Ref<
                Expression::NFExpression,
            >| Expression::isLiteral(&__a0))?
            {
                if Flags::isSet(Flags::DUMP_REPL.clone())? {
                    Error::addMessage(
                        Error::INTERNAL_ERROR.clone(),
                        list![{
                            let mut __mm_s = String::new();
                            __mm_s.push_str(&*literal!("NBAlias.setStartFixed"));
                            __mm_s.push_str(&*literal!(
                                " failed because multiple variables are fixed with different start values!\n"
                            ));
                            __mm_s.push_str(&*AliasSet::toString(set)?);
                            __mm_s.push_str(&*literal!("\n\tFixed start map after replacements:\n\t"));
                            __mm_s.push_str(&*UnorderedMap::toString(
                                fixed_start_map.clone(),
                                &move |__a0: metamodelica::Ref<ComponentRef::NFComponentRef>| {
                                    ComponentRef::toString(&__a0)
                                },
                                &Expression::toString,
                                literal!("\n\t"),
                                &(literal!(", ")),
                            )?);
                            ArcStr::from(__mm_s)
                        }],
                    )?;
                    return Err("fail");
                } else {
                    Error::addMessage(
                        Error::INTERNAL_ERROR.clone(),
                        list![{
                            let mut __mm_s = String::new();
                            __mm_s.push_str(&*literal!("NBAlias.setStartFixed"));
                            __mm_s.push_str(&*literal!(" failed because multiple variables are fixed with different start values! Use -d=dumprepl for more information.\n"));
                            ArcStr::from(__mm_s)
                        }],
                    )?;
                    return Err("fail");
                }
            } else if Flags::isSet(Flags::DUMP_REPL.clone())? {
                Error::addCompilerWarning({
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!("NBAlias.setStartFixed"));
                    __mm_s.push_str(&*literal!(": Multiple variables are fixed with start values that could not be proven equal; picking one arbitrarily.\n"));
                    __mm_s.push_str(&*AliasSet::toString(set)?);
                    __mm_s.push_str(&*literal!("\n\tFixed start map after replacements:\n\t"));
                    __mm_s.push_str(&*UnorderedMap::toString(
                        fixed_start_map.clone(),
                        &move |__a0: metamodelica::Ref<ComponentRef::NFComponentRef>| ComponentRef::toString(&__a0),
                        &Expression::toString,
                        literal!("\n\t"),
                        &(literal!(", ")),
                    )?);
                    ArcStr::from(__mm_s)
                })?;
            } else {
                Error::addCompilerWarning({
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!("NBAlias.setStartFixed"));
                    __mm_s.push_str(&*literal!(": Multiple variables are fixed with start values that could not be proven equal; picking one arbitrarily. Use -d=dumprepl for more information.\n"));
                    ArcStr::from(__mm_s)
                })?;
            }
        } else if List::allEqual(&fixed_start_lst, &Expression::isEqual)? {
            if Flags::isSet(Flags::DUMP_REPL.clone())? {
                Error::addCompilerWarning({
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!("NBAlias.setStartFixed"));
                    __mm_s.push_str(&*literal!(
                        ": Multiple variables are fixed and have identical start values.\n"
                    ));
                    __mm_s.push_str(&*AliasSet::toString(set)?);
                    __mm_s.push_str(&*literal!("\n\tFixed start map after replacements:\n\t"));
                    __mm_s.push_str(&*UnorderedMap::toString(
                        fixed_start_map.clone(),
                        &move |__a0: metamodelica::Ref<ComponentRef::NFComponentRef>| ComponentRef::toString(&__a0),
                        &Expression::toString,
                        literal!("\n\t"),
                        &(literal!(", ")),
                    )?);
                    ArcStr::from(__mm_s)
                })?;
            } else {
                Error::addCompilerWarning({
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!("NBAlias.setStartFixed"));
                    __mm_s.push_str(&*literal!(": Multiple variables are fixed and have identical start values. Use -d=dumprepl for more information.\n"));
                    ArcStr::from(__mm_s)
                })?;
            }
        }
    }
    Ok(fixed_start_map)
}

fn selectStartByConfidence(
    mut start_map: metamodelica::Ref<
        UnorderedMap::UnorderedMap<
            metamodelica::Ref<ComponentRef::NFComponentRef>,
            metamodelica::Ref<Expression::NFExpression>,
        >,
    >,
    mut binding_map: metamodelica::Ref<
        UnorderedMap::UnorderedMap<
            metamodelica::Ref<ComponentRef::NFComponentRef>,
            metamodelica::Ref<Binding::NFBinding>,
        >,
    >,
    mut set: &metamodelica::Ref<AliasSet::AliasSet>,
) -> Result<Option<metamodelica::Ref<ComponentRef::NFComponentRef>>> {
    let mut best: Option<metamodelica::Ref<ComponentRef::NFComponentRef>> = None;
    let mut cref: metamodelica::Ref<ComponentRef::NFComponentRef>;
    let mut val: metamodelica::Ref<Expression::NFExpression>;
    let mut best_val: metamodelica::Ref<Expression::NFExpression> =
        metamodelica::Ref::new(Expression::NFExpression::INTEGER { value: 0 });
    let mut b: metamodelica::Ref<Binding::NFBinding>;
    let mut best_b: metamodelica::Ref<Binding::NFBinding> = Binding::EMPTY_BINDING().clone();
    let mut cmp: i32;
    let mut tie: bool = false;
    for mut tpl in &*UnorderedMap::toList(start_map.clone()) {
        (cref, val) = tpl.clone();
        b = UnorderedMap::getSafe(
            cref.clone(),
            binding_map.clone(),
            metamodelica::sourceInfo!("NBackEnd/Modules/2_Pre/NBAlias.mo"),
        )?;
        cmp = if ((best).is_none()) {
            -1
        } else {
            Binding::compareStartConfidence(b.clone(), best_b.clone())?
        };
        if cmp < 0 {
            (best, best_val, best_b) = (Some(cref), val, b);
            tie = false;
        } else if cmp == 0 && !(Expression::isEqual(val.clone(), best_val.clone())?) {
            tie = true;
        }
    }
    if tie {
        if Flags::isSet(Flags::DUMP_REPL.clone())? {
            Error::addCompilerWarning({
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("NBAlias.selectStartByConfidence"));
                __mm_s.push_str(&*literal!(
                    ": Alias set with conflicting unfixed start values of equal confidence detected.\n"
                ));
                __mm_s.push_str(&*AliasSet::toString(set)?);
                __mm_s.push_str(&*literal!("\n\tStart map after replacements:\n\t"));
                __mm_s.push_str(&*UnorderedMap::toString(
                    start_map,
                    &move |__a0: metamodelica::Ref<ComponentRef::NFComponentRef>| ComponentRef::toString(&__a0),
                    &Expression::toString,
                    literal!("\n\t"),
                    &(literal!(", ")),
                )?);
                ArcStr::from(__mm_s)
            })?;
        } else {
            Error::addCompilerWarning({
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("NBAlias.selectStartByConfidence"));
                __mm_s.push_str(&*literal!(": Alias set with conflicting unfixed start values of equal confidence detected. Use -d=dumprepl for more information.\n"));
                ArcStr::from(__mm_s)
            })?;
        }
    }
    Ok(best)
}

fn checkNominalThreshold(
    mut map: metamodelica::Ref<
        UnorderedMap::UnorderedMap<
            metamodelica::Ref<ComponentRef::NFComponentRef>,
            metamodelica::Ref<Expression::NFExpression>,
        >,
    >,
    mut set: &metamodelica::Ref<AliasSet::AliasSet>,
) -> Result<()> {
    let mut current: metamodelica::List<metamodelica::Ref<Expression::NFExpression>>;
    let mut lst_values: metamodelica::List<metamodelica::Ref<Expression::NFExpression>> =
        UnorderedMap::valueList(map.clone());
    let mut arr_iter: metamodelica::Array<metamodelica::Ref<ExpressionIterator::NFExpressionIterator>>;
    let mut iter: metamodelica::Ref<ExpressionIterator::NFExpressionIterator>;
    let mut exp: metamodelica::Ref<Expression::NFExpression>;
    let mut index: i32 = 1;
    if (lst_values).is_empty() {
        return Ok(());
    }
    if !(List::allEqual(
        &({
            let mut __acc: metamodelica::List<i32> = metamodelica::nil();
            for mut e in (lst_values.clone()).into_iter().cloned() {
                let __x = Type::sizeOf(&(Expression::typeOf(e.clone())), false)?;
                __acc = cons(__x, __acc);
            }
            __acc.reverse()
        }),
        &fnptr!(intEq, i32, i32),
    )?) {
        Error::addCompilerWarning({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("NBAlias.checkNominalThreshold"));
            __mm_s.push_str(&*literal!(
                " failed because array nominal values have different size. Use -d=dumprepl for more information.\n"
            ));
            ArcStr::from(__mm_s)
        })?;
        return Err("fail");
    }
    arr_iter = metamodelica::arrayFromVec(
        ({
            let mut __acc: metamodelica::List<metamodelica::Ref<ExpressionIterator::NFExpressionIterator>> =
                metamodelica::nil();
            for mut e in (lst_values).into_iter().cloned() {
                let __x = ExpressionIterator::fromExp(e.clone(), false, false)?;
                __acc = cons(__x, __acc);
            }
            __acc.reverse()
        })
        .into_iter()
        .cloned()
        .collect(),
    );
    while ExpressionIterator::hasNext(
        &({
            let __elt = (*metamodelica::index_checked(&arr_iter.borrow(), 1)?).clone();
            __elt
        }),
    ) {
        current = metamodelica::nil();
        for mut i in 1..=metamodelica::arrayLength(arr_iter.clone()) {
            (iter, exp) = ExpressionIterator::next(
                ({
                    let __elt = (*metamodelica::index_checked(&arr_iter.borrow(), i)?).clone();
                    __elt
                }),
            )?;
            metamodelica::arrayUpdate(arr_iter.clone(), i, iter)?;
            current = metamodelica::cons(exp, current);
        }
        checkNominalThresholdSingle(&current, map.clone(), set, index)?;
        index = index + 1;
    }
    Ok(())
}

fn checkNominalThresholdSingle(
    mut lst_values: &metamodelica::List<metamodelica::Ref<Expression::NFExpression>>,
    mut map: metamodelica::Ref<
        UnorderedMap::UnorderedMap<
            metamodelica::Ref<ComponentRef::NFComponentRef>,
            metamodelica::Ref<Expression::NFExpression>,
        >,
    >,
    mut set: &metamodelica::Ref<AliasSet::AliasSet>,
    mut index: i32,
) -> Result<()> {
    let mut constants: metamodelica::List<metamodelica::Ref<Expression::NFExpression>>;
    let mut rest: metamodelica::List<metamodelica::Ref<Expression::NFExpression>>;
    let mut zeroes: metamodelica::List<metamodelica::Ref<Expression::NFExpression>>;
    let mut real_constants: metamodelica::List<metamodelica::Real>;
    let mut nom_min: metamodelica::Real;
    let mut nom_max: metamodelica::Real;
    let mut nom_quotient: metamodelica::Real;
    let mut r#str: ArcStr;
    (constants, rest) = List::splitOnTrue(
        lst_values,
        &move |__a0: metamodelica::Ref<Expression::NFExpression>| -> metamodelica::Result<_> {
            ::std::result::Result::Ok(Expression::isConstNumber(&__a0))
        },
    )?;
    (zeroes, constants) = List::splitOnTrue(&constants, &move |__a0: metamodelica::Ref<Expression::NFExpression>| {
        Expression::isZero(&__a0)
    })?;
    if Flags::isSet(Flags::FAILTRACE.clone())? && !((rest).is_empty()) {
        r#str = {
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("NBAlias.checkNominalThresholdSingle"));
            __mm_s.push_str(&*literal!(
                ": There are non literal nominal values in following alias set:\n"
            ));
            __mm_s.push_str(&*AliasSet::toString(set)?);
            __mm_s.push_str(&*literal!(
                "\n\tNominal map after replacements (conflicting array index = "
            ));
            __mm_s.push_str(&*intString(index));
            __mm_s.push_str(&*literal!("):\n\t"));
            __mm_s.push_str(&*UnorderedMap::toString(
                map.clone(),
                &move |__a0: metamodelica::Ref<ComponentRef::NFComponentRef>| ComponentRef::toString(&__a0),
                &Expression::toString,
                literal!("\n\t"),
                &(literal!(", ")),
            )?);
            ArcStr::from(__mm_s)
        };
        Error::addCompilerWarning(r#str)?;
    }
    if !((constants).is_empty()) {
        real_constants = ({
            let mut __acc: metamodelica::List<_> = metamodelica::nil();
            for mut val in (constants).into_iter().cloned() {
                let __x = (Expression::realValue(&(val.clone()))?).abs();
                __acc = cons(__x, __acc);
            }
            __acc.reverse()
        });
        nom_min = List::minElement(&real_constants, &fnptr!(realLt, metamodelica::Real, metamodelica::Real))?;
        nom_max = List::maxElement(&real_constants, &fnptr!(realLt, metamodelica::Real, metamodelica::Real))?;
        nom_quotient = metamodelica::real_div_checked(nom_max, nom_min)?;
        if nom_quotient > NOMINAL_THRESHOLD.clone() {
            r#str = {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("NBAlias.checkNominalThresholdSingle"));
                __mm_s.push_str(&*literal!(
                    ": The quotient of the greatest and lowest nominal value is greater than the nominal threshold = "
                ));
                __mm_s.push_str(&*realString(NOMINAL_THRESHOLD.clone()));
                __mm_s.push_str(&*literal!("."));
                ArcStr::from(__mm_s)
            };
            if Flags::isSet(Flags::DUMP_REPL.clone())? {
                r#str = {
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*r#str);
                    __mm_s.push_str(&*literal!("\n"));
                    __mm_s.push_str(&*AliasSet::toString(set)?);
                    __mm_s.push_str(&*literal!(
                        "\n\tNominal map after replacements (conflicting array index = "
                    ));
                    __mm_s.push_str(&*intString(index));
                    __mm_s.push_str(&*literal!("):\n\t"));
                    __mm_s.push_str(&*UnorderedMap::toString(
                        map.clone(),
                        &move |__a0: metamodelica::Ref<ComponentRef::NFComponentRef>| ComponentRef::toString(&__a0),
                        &Expression::toString,
                        literal!("\n\t"),
                        &(literal!(", ")),
                    )?);
                    ArcStr::from(__mm_s)
                };
            } else {
                r#str = {
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*r#str);
                    __mm_s.push_str(&*literal!(" Use -d=dumprepl for more information.\n"));
                    ArcStr::from(__mm_s)
                };
            }
            Error::addCompilerWarning(r#str)?;
        }
    }
    if !((zeroes).is_empty()) {
        r#str = {
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("NBAlias.checkNominalThresholdSingle"));
            __mm_s.push_str(&*literal!(": Zero valued nominal values are not allowed."));
            ArcStr::from(__mm_s)
        };
        if Flags::isSet(Flags::DUMP_REPL.clone())? {
            r#str = {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*r#str);
                __mm_s.push_str(&*literal!(
                    "\n\tNominal map after replacements (violating array index = "
                ));
                __mm_s.push_str(&*intString(index));
                __mm_s.push_str(&*literal!("):\n\t"));
                __mm_s.push_str(&*UnorderedMap::toString(
                    map,
                    &move |__a0: metamodelica::Ref<ComponentRef::NFComponentRef>| ComponentRef::toString(&__a0),
                    &Expression::toString,
                    literal!("\n\t"),
                    &(literal!(", ")),
                )?);
                ArcStr::from(__mm_s)
            };
        } else {
            r#str = {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*r#str);
                __mm_s.push_str(&*literal!(" Use -d=dumprepl for more information.\n"));
                ArcStr::from(__mm_s)
            };
        }
        Error::addCompilerWarning(r#str)?;
    }
    Ok(())
}

fn stateSelectAlways(
    mut map: metamodelica::Ref<
        UnorderedMap::UnorderedMap<metamodelica::Ref<ComponentRef::NFComponentRef>, StateSelect>,
    >,
    mut set: &metamodelica::Ref<AliasSet::AliasSet>,
) -> Result<()> {
    let mut lst_values: metamodelica::List<StateSelect> = UnorderedMap::valueList(map.clone());
    let mut count: i32 = 0;
    for mut val in &*lst_values {
        if val.clone() == StateSelect::ALWAYS.clone() {
            count = count + 1;
        }
    }
    if count > 1 {
        if Flags::isSet(Flags::DUMP_REPL.clone())? {
            Error::addMessage(
                Error::INTERNAL_ERROR.clone(),
                list![{
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!("NBAlias.stateSelectAlways"));
                    __mm_s.push_str(&*literal!(
                        " failed because multiple variables have StateSelect = always!\n"
                    ));
                    __mm_s.push_str(&*AliasSet::toString(set)?);
                    __mm_s.push_str(&*literal!("\n\tStateSelect map after replacements:\n\t"));
                    __mm_s.push_str(&*UnorderedMap::toString(
                        map,
                        &move |__a0: metamodelica::Ref<ComponentRef::NFComponentRef>| ComponentRef::toString(&__a0),
                        &BackendExtension::VariableAttributes::stateSelectString,
                        literal!("\n\t"),
                        &(literal!(", ")),
                    )?);
                    ArcStr::from(__mm_s)
                }],
            )?;
            return Err("fail");
        } else {
            Error::addMessage(
                Error::INTERNAL_ERROR.clone(),
                list![{
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!("NBAlias.stateSelectAlways"));
                    __mm_s.push_str(&*literal!(" failed because multiple variables have StateSelect = always! Use -d=dumprepl for more information.\n"));
                    ArcStr::from(__mm_s)
                }],
            )?;
            return Err("fail");
        }
    }
    Ok(())
}

fn diffTearingSelect(
    mut map: metamodelica::Ref<
        UnorderedMap::UnorderedMap<metamodelica::Ref<ComponentRef::NFComponentRef>, TearingSelect>,
    >,
    mut set: &metamodelica::Ref<AliasSet::AliasSet>,
) -> Result<()> {
    let mut lst_values: metamodelica::List<TearingSelect> = UnorderedMap::valueList(map.clone());
    let mut first: TearingSelect;
    let mut rest: metamodelica::List<TearingSelect>;
    let mut equal: bool = true;
    if !((lst_values).is_empty()) {
        let (__pa0, __pa1) = ::match_deref::match_deref! { match &(lst_values) {
            Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
            _ => return Err("pattern mismatch"),
        } };
        first = metamodelica::Own::own(__pa0);
        rest = metamodelica::Own::own(__pa1);
        for mut val in &*rest {
            if first != val.clone() {
                equal = false;
                break;
            }
        }
        if !(equal) {
            if Flags::isSet(Flags::DUMP_REPL.clone())? {
                Error::addCompilerNotification({
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!("There are different TearingSelect values.\n"));
                    __mm_s.push_str(&*AliasSet::toString(set)?);
                    __mm_s.push_str(&*literal!("\n\tTearingSelect map after replacements:\n\t"));
                    __mm_s.push_str(&*UnorderedMap::toString(
                        map,
                        &move |__a0: metamodelica::Ref<ComponentRef::NFComponentRef>| ComponentRef::toString(&__a0),
                        &BackendExtension::VariableAttributes::tearingSelectString,
                        literal!("\n\t"),
                        &(literal!(", ")),
                    )?);
                    ArcStr::from(__mm_s)
                })?;
            } else {
                Error::addCompilerNotification(literal!(
                    "There are different TearingSelect values. Use -d=dumprepl for more information.\n"
                ))?;
            }
        }
    }
    Ok(())
}

fn chooseStateSelect(
    mut map: metamodelica::Ref<
        UnorderedMap::UnorderedMap<metamodelica::Ref<ComponentRef::NFComponentRef>, StateSelect>,
    >,
) -> Result<(
    Option<metamodelica::Ref<ComponentRef::NFComponentRef>>,
    Option<StateSelect>,
)> {
    let mut chosen_cref: Option<metamodelica::Ref<ComponentRef::NFComponentRef>>;
    let mut chosen_val: Option<StateSelect>;
    let mut lst_values: metamodelica::List<(metamodelica::Ref<ComponentRef::NFComponentRef>, StateSelect)> =
        UnorderedMap::toList(map.clone());
    let mut sval: StateSelect;
    let mut state_select: StateSelect = StateSelect::NEVER.clone();
    let mut compref: metamodelica::Ref<ComponentRef::NFComponentRef>;
    let mut cref: metamodelica::Ref<ComponentRef::NFComponentRef>;
    if (lst_values).is_empty() {
        chosen_val = None;
        chosen_cref = None;
    } else if List::hasOneElement(&lst_values) {
        (compref, sval) = (lst_values).head().cloned()?;
        chosen_val = Some(sval);
        chosen_cref = Some(compref);
    } else {
        (compref, state_select) = (lst_values).head().cloned()?;
        for mut tpl in &*lst_values {
            (cref, sval) = tpl.clone();
            if sval > state_select {
                state_select = sval;
                compref = cref;
            }
        }
        chosen_val = Some(state_select);
        chosen_cref = Some(compref);
    }
    Ok((chosen_cref, chosen_val))
}

fn chooseTearingSelect(
    mut map: metamodelica::Ref<
        UnorderedMap::UnorderedMap<metamodelica::Ref<ComponentRef::NFComponentRef>, TearingSelect>,
    >,
) -> Result<Option<TearingSelect>> {
    let mut chosen_val: Option<TearingSelect>;
    let mut lst_values: metamodelica::List<TearingSelect> = UnorderedMap::valueList(map.clone());
    let mut tearing_select: TearingSelect;
    if (lst_values).is_empty() {
        chosen_val = None;
    } else if List::hasOneElement(&lst_values) {
        chosen_val = Some((lst_values).head().cloned()?);
    } else {
        tearing_select = TearingSelect::NEVER.clone();
        for mut val in &*lst_values {
            if val.clone() > tearing_select {
                tearing_select = val.clone();
            }
        }
        chosen_val = Some(tearing_select);
    }
    Ok(chosen_val)
}

fn mean(mut lst: metamodelica::List<metamodelica::Ref<Expression::NFExpression>>) -> Result<metamodelica::Real> {
    let mut mean_val: metamodelica::Real;
    let mut cur_sum: metamodelica::Real;
    cur_sum = ({
        let mut __acc: metamodelica::Real = metamodelica::OrderedFloat(0.0_f64);
        for mut val in (lst.clone()).into_iter().cloned() {
            let __x = Expression::realValue(&(val.clone()))?;
            __acc += __x;
        }
        __acc
    });
    mean_val = metamodelica::real_div_checked(cur_sum, metamodelica::OrderedFloat(((lst).len() as i32) as f64))?;
    metamodelica::print({
        let mut __mm_s = String::new();
        __mm_s.push_str(&*literal!("Mean = "));
        __mm_s.push_str(&*ArcStr::from(::std::format!("{}", mean_val)));
        ArcStr::from(__mm_s)
    });
    Ok(mean_val)
}

fn optionMinMax(
    mut var_ptr: Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>,
    mut attr_min: Option<metamodelica::Ref<Binding::NFBinding>>,
    mut attr_max: Option<metamodelica::Ref<Binding::NFBinding>>,
    mut attrcollector: metamodelica::Ref<AttributeCollector::AttributeCollector>,
) -> Result<metamodelica::Ref<AttributeCollector::AttributeCollector>> {
    let mut attrcollector: metamodelica::Ref<AttributeCollector::AttributeCollector> = attrcollector;
    let mut min_b: metamodelica::Ref<Binding::NFBinding>;
    let mut max_b: metamodelica::Ref<Binding::NFBinding>;
    if (attr_min).is_some() {
        let __pa0 = ::match_deref::match_deref! { match &(attr_min) {
            Some(__pa0) => __pa0.clone(),
            _ => return Err("pattern mismatch"),
        } };
        min_b = metamodelica::Own::own(__pa0);
        UnorderedMap::add(
            BVariable::getVarName(var_ptr.clone()),
            Binding::getTypedExp(&min_b)?,
            attrcollector.min_val_map.clone(),
        )?;
    }
    if (attr_max).is_some() {
        let __pa1 = ::match_deref::match_deref! { match &(attr_max) {
            Some(__pa1) => __pa1.clone(),
            _ => return Err("pattern mismatch"),
        } };
        max_b = metamodelica::Own::own(__pa1);
        UnorderedMap::add(
            BVariable::getVarName(var_ptr),
            Binding::getTypedExp(&max_b)?,
            attrcollector.max_val_map.clone(),
        )?;
    }
    Ok(attrcollector)
}

fn isAuxStart(mut exp: &metamodelica::Ref<Expression::NFExpression>) -> Result<bool> {
    let mut b: bool;
    b = (match &**exp {
        Expression::CALL { .. } => true,
        Expression::RECORD_ELEMENT {
            recordExp: __exp_recordExp,
            ..
        } => Expression::isCall(metamodelica::AsArg::as_arg(&__exp_recordExp)),
        Expression::TUPLE_ELEMENT {
            tupleExp: __exp_tupleExp,
            ..
        } => Expression::isCall(metamodelica::AsArg::as_arg(&__exp_tupleExp)),
        Expression::ARRAY { .. } => Array::all(
            var_field!((**exp).elements, Expression::NFExpression::ARRAY).clone(),
            &move |__a0: metamodelica::Ref<Expression::NFExpression>| isAuxStart(&__a0),
        )?,
        _ => false,
    });
    Ok(b)
}

fn optionStartFixed(
    mut var_ptr: Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>,
    mut attr_start: Option<metamodelica::Ref<Binding::NFBinding>>,
    mut attr_fixed: Option<metamodelica::Ref<Binding::NFBinding>>,
    mut attrcollector: metamodelica::Ref<AttributeCollector::AttributeCollector>,
) -> Result<metamodelica::Ref<AttributeCollector::AttributeCollector>> {
    let mut attrcollector: metamodelica::Ref<AttributeCollector::AttributeCollector> = attrcollector;
    let mut start_b: metamodelica::Ref<Binding::NFBinding>;
    let mut fixed_b: metamodelica::Ref<Binding::NFBinding>;
    if (attr_start).is_some() {
        let __pa0 = ::match_deref::match_deref! { match &(attr_start) {
            Some(__pa0) => __pa0.clone(),
            _ => return Err("pattern mismatch"),
        } };
        start_b = metamodelica::Own::own(__pa0);
        if !(BVariable::isFunctionAlias(var_ptr.clone())?
            && Binding::source(&start_b) == Binding::Source::GENERATED.clone()
            && isAuxStart(&(Binding::getTypedExp(&start_b)?))?)
        {
            UnorderedMap::add(
                BVariable::getVarName(var_ptr.clone()),
                Binding::getTypedExp(&start_b)?,
                attrcollector.start_map.clone(),
            )?;
            UnorderedMap::add(
                BVariable::getVarName(var_ptr.clone()),
                start_b,
                attrcollector.start_binding_map.clone(),
            )?;
        }
    }
    if (attr_fixed).is_some() {
        let __pa1 = ::match_deref::match_deref! { match &(attr_fixed) {
            Some(__pa1) => __pa1.clone(),
            _ => return Err("pattern mismatch"),
        } };
        fixed_b = metamodelica::Own::own(__pa1);
        UnorderedMap::add(
            BVariable::getVarName(var_ptr),
            Binding::getTypedExp(&fixed_b)?,
            attrcollector.fixed_map.clone(),
        )?;
    }
    Ok(attrcollector)
}

fn rateVar(
    mut var_ptr: Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>,
    mut attrcollector: metamodelica::Ref<AttributeCollector::AttributeCollector>,
) -> Result<(i32, metamodelica::Ref<AttributeCollector::AttributeCollector>)> {
    let mut rating: i32;
    let mut attrcollector: metamodelica::Ref<AttributeCollector::AttributeCollector> = attrcollector;
    let mut name: metamodelica::Ref<ComponentRef::NFComponentRef>;
    let mut nominal_val: metamodelica::Ref<Expression::NFExpression>;
    let mut stateSelect_val: StateSelect;
    let mut tearingSelect_val: TearingSelect;
    if BVariable::isFunctionAlias(var_ptr.clone())? || BVariable::isClockAlias(var_ptr.clone())? {
        rating = -10000;
    } else {
        name = BVariable::getVarName(var_ptr.clone());
        rating = -(ComponentRef::depth(&name));
    }
    let () = (::match_deref::match_deref! { match &(Pointer::access(var_ptr.clone())) {
        Deref @ Variable::VARIABLE { backendinfo: Deref @ BackendExtension::BackendInfo::BACKEND_INFO { attributes: attr @ Deref @ BackendExtension::VariableAttributes::VAR_ATTR_REAL { .. }, .. }, .. } => {
            attrcollector = optionMinMax(var_ptr.clone(), var_field!((**attr).min, BackendExtension::VariableAttributes::VariableAttributes::VAR_ATTR_REAL).clone(), var_field!((**attr).max, BackendExtension::VariableAttributes::VariableAttributes::VAR_ATTR_REAL).clone(), attrcollector)?;
            attrcollector = optionStartFixed(var_ptr.clone(), var_field!((**attr).start, BackendExtension::VariableAttributes::VariableAttributes::VAR_ATTR_REAL).clone(), var_field!((**attr).fixed, BackendExtension::VariableAttributes::VariableAttributes::VAR_ATTR_REAL).clone(), attrcollector)?;
            if (var_field!((**attr).nominal, BackendExtension::VariableAttributes::VariableAttributes::VAR_ATTR_REAL)).is_some() {
                nominal_val = Binding::getTypedExp(&(Util::getOption(var_field!((**attr).nominal, BackendExtension::VariableAttributes::VariableAttributes::VAR_ATTR_REAL).clone())?))?;
                UnorderedMap::add(BVariable::getVarName(var_ptr.clone()), nominal_val, attrcollector.nominal_map.clone())?;
            }
            if (var_field!((**attr).stateSelect, BackendExtension::VariableAttributes::VariableAttributes::VAR_ATTR_REAL)).is_some() {
                stateSelect_val = Util::getOption(var_field!((**attr).stateSelect, BackendExtension::VariableAttributes::VariableAttributes::VAR_ATTR_REAL).clone())?;
                if stateSelect_val == StateSelect::ALWAYS.clone() {
                    rating = rating + 100;
                } else if stateSelect_val == StateSelect::PREFER.clone() {
                    rating = rating + 50;
                }
                UnorderedMap::add(BVariable::getVarName(var_ptr.clone()), stateSelect_val, attrcollector.stateSelect_map.clone())?;
            }
            if (var_field!((**attr).tearingSelect, BackendExtension::VariableAttributes::VariableAttributes::VAR_ATTR_REAL)).is_some() {
                tearingSelect_val = Util::getOption(var_field!((**attr).tearingSelect, BackendExtension::VariableAttributes::VariableAttributes::VAR_ATTR_REAL).clone())?;
                UnorderedMap::add(BVariable::getVarName(var_ptr), tearingSelect_val, attrcollector.tearingSelect_map.clone())?;
            }
            ()
        },
        Deref @ Variable::VARIABLE { backendinfo: Deref @ BackendExtension::BackendInfo::BACKEND_INFO { attributes: attr @ Deref @ BackendExtension::VariableAttributes::VAR_ATTR_INT { .. }, .. }, .. } => {
            attrcollector = optionMinMax(var_ptr.clone(), var_field!((**attr).min, BackendExtension::VariableAttributes::VariableAttributes::VAR_ATTR_INT).clone(), var_field!((**attr).max, BackendExtension::VariableAttributes::VariableAttributes::VAR_ATTR_INT).clone(), attrcollector)?;
            attrcollector = optionStartFixed(var_ptr, var_field!((**attr).start, BackendExtension::VariableAttributes::VariableAttributes::VAR_ATTR_INT).clone(), var_field!((**attr).fixed, BackendExtension::VariableAttributes::VariableAttributes::VAR_ATTR_INT).clone(), attrcollector)?;
            ()
        },
        Deref @ Variable::VARIABLE { backendinfo: Deref @ BackendExtension::BackendInfo::BACKEND_INFO { attributes: attr @ Deref @ BackendExtension::VariableAttributes::VAR_ATTR_BOOL { .. }, .. }, .. } => {
            attrcollector = optionStartFixed(var_ptr, var_field!((**attr).start, BackendExtension::VariableAttributes::VariableAttributes::VAR_ATTR_BOOL).clone(), var_field!((**attr).fixed, BackendExtension::VariableAttributes::VariableAttributes::VAR_ATTR_BOOL).clone(), attrcollector)?;
            ()
        },
        _ => {
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((rating, attrcollector))
}

pub mod AttributeCollector {
    use super::*;
    #[derive(Clone, Debug, Eq, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
    pub struct AttributeCollector {
        /// set containing all minimum values
        pub min_val_map: metamodelica::Ref<
            UnorderedMap::UnorderedMap<
                metamodelica::Ref<ComponentRef::NFComponentRef>,
                metamodelica::Ref<Expression::NFExpression>,
            >,
        >,
        /// set containing all maximum values
        pub max_val_map: metamodelica::Ref<
            UnorderedMap::UnorderedMap<
                metamodelica::Ref<ComponentRef::NFComponentRef>,
                metamodelica::Ref<Expression::NFExpression>,
            >,
        >,
        /// set containing all start values
        pub start_map: metamodelica::Ref<
            UnorderedMap::UnorderedMap<
                metamodelica::Ref<ComponentRef::NFComponentRef>,
                metamodelica::Ref<Expression::NFExpression>,
            >,
        >,
        /// start bindings, for their confidence
        pub start_binding_map: metamodelica::Ref<
            UnorderedMap::UnorderedMap<
                metamodelica::Ref<ComponentRef::NFComponentRef>,
                metamodelica::Ref<Binding::NFBinding>,
            >,
        >,
        /// set containing all fixed values
        pub fixed_map: metamodelica::Ref<
            UnorderedMap::UnorderedMap<
                metamodelica::Ref<ComponentRef::NFComponentRef>,
                metamodelica::Ref<Expression::NFExpression>,
            >,
        >,
        /// set containing all nominal values
        pub nominal_map: metamodelica::Ref<
            UnorderedMap::UnorderedMap<
                metamodelica::Ref<ComponentRef::NFComponentRef>,
                metamodelica::Ref<Expression::NFExpression>,
            >,
        >,
        /// set containing all stateSelect values
        pub stateSelect_map:
            metamodelica::Ref<UnorderedMap::UnorderedMap<metamodelica::Ref<ComponentRef::NFComponentRef>, StateSelect>>,
        /// set containing all tearingSelect values
        pub tearingSelect_map: metamodelica::Ref<
            UnorderedMap::UnorderedMap<metamodelica::Ref<ComponentRef::NFComponentRef>, TearingSelect>,
        >,
    }

    impl metamodelica::gc::MMTrace for AttributeCollector {
        fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
            metamodelica::gc::MMTrace::mm_accept(&self.min_val_map, __mmv)?;
            metamodelica::gc::MMTrace::mm_accept(&self.max_val_map, __mmv)?;
            metamodelica::gc::MMTrace::mm_accept(&self.start_map, __mmv)?;
            metamodelica::gc::MMTrace::mm_accept(&self.start_binding_map, __mmv)?;
            metamodelica::gc::MMTrace::mm_accept(&self.fixed_map, __mmv)?;
            metamodelica::gc::MMTrace::mm_accept(&self.nominal_map, __mmv)?;
            metamodelica::gc::MMTrace::mm_accept(&self.stateSelect_map, __mmv)?;
            metamodelica::gc::MMTrace::mm_accept(&self.tearingSelect_map, __mmv)?;
            Ok(())
        }
    }
    impl Default for AttributeCollector {
        fn default() -> Self {
            Self {
                min_val_map: Default::default(),
                max_val_map: Default::default(),
                start_map: Default::default(),
                start_binding_map: Default::default(),
                fixed_map: Default::default(),
                nominal_map: Default::default(),
                stateSelect_map: Default::default(),
                tearingSelect_map: Default::default(),
            }
        }
    }

    pub type ATTRIBUTE_COLLECTOR = AttributeCollector;

    pub(crate) fn toString(
        mut attrcollector: &metamodelica::Ref<AttributeCollector>,
        mut r#str: ArcStr,
    ) -> Result<ArcStr> {
        let mut r#str: ArcStr = r#str;
        let mut array_maps: metamodelica::Array<
            metamodelica::Ref<
                UnorderedMap::UnorderedMap<
                    metamodelica::Ref<ComponentRef::NFComponentRef>,
                    metamodelica::Ref<Expression::NFExpression>,
                >,
            >,
        >;
        let mut array_names: metamodelica::Array<ArcStr>;
        array_maps = metamodelica::arrayFromVec(
            list![
                attrcollector.min_val_map.clone(),
                attrcollector.max_val_map.clone(),
                attrcollector.start_map.clone(),
                attrcollector.fixed_map.clone(),
                attrcollector.nominal_map.clone()
            ]
            .into_iter()
            .cloned()
            .collect(),
        );
        array_names = metamodelica::arrayFromVec(
            list![
                literal!("Min map"),
                literal!("Max map"),
                literal!("Start map"),
                literal!("Fixed map"),
                literal!("Nominal map")
            ]
            .into_iter()
            .cloned()
            .collect(),
        );
        for mut i in 1..=metamodelica::arrayLength(array_maps.clone()) {
            if UnorderedMap::isEmpty(
                ({
                    let __elt = (*metamodelica::index_checked(&array_maps.borrow(), i)?).clone();
                    __elt
                }),
            ) == false
            {
                r#str = {
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*r#str);
                    __mm_s.push_str(&*metamodelica::arrayGet(array_names.clone(), i)?);
                    __mm_s.push_str(&*literal!(":\n\t"));
                    __mm_s.push_str(&*UnorderedMap::toString(
                        ({
                            let __elt = (*metamodelica::index_checked(&array_maps.borrow(), i)?).clone();
                            __elt
                        }),
                        &move |__a0: metamodelica::Ref<ComponentRef::NFComponentRef>| ComponentRef::toString(&__a0),
                        &Expression::toString,
                        literal!("\n\t"),
                        &(literal!(", ")),
                    )?);
                    __mm_s.push_str(&*literal!("\n"));
                    ArcStr::from(__mm_s)
                };
            }
        }
        if UnorderedMap::isEmpty(attrcollector.stateSelect_map.clone()) == false {
            r#str = {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*r#str);
                __mm_s.push_str(&*literal!("StateSelect map"));
                __mm_s.push_str(&*literal!(":\n\t"));
                __mm_s.push_str(&*UnorderedMap::toString(
                    attrcollector.stateSelect_map.clone(),
                    &move |__a0: metamodelica::Ref<ComponentRef::NFComponentRef>| ComponentRef::toString(&__a0),
                    &BackendExtension::VariableAttributes::stateSelectString,
                    literal!("\n\t"),
                    &(literal!(", ")),
                )?);
                __mm_s.push_str(&*literal!("\n"));
                ArcStr::from(__mm_s)
            };
        }
        if UnorderedMap::isEmpty(attrcollector.tearingSelect_map.clone()) == false {
            r#str = {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*r#str);
                __mm_s.push_str(&*literal!("TearingSelect map"));
                __mm_s.push_str(&*literal!(":\n\t"));
                __mm_s.push_str(&*UnorderedMap::toString(
                    attrcollector.tearingSelect_map.clone(),
                    &move |__a0: metamodelica::Ref<ComponentRef::NFComponentRef>| ComponentRef::toString(&__a0),
                    &BackendExtension::VariableAttributes::tearingSelectString,
                    literal!("\n\t"),
                    &(literal!(", ")),
                )?);
                __mm_s.push_str(&*literal!("\n"));
                ArcStr::from(__mm_s)
            };
        }
        Ok(r#str)
    }

    pub(crate) fn fixValues(
        mut attrcollector: metamodelica::Ref<AttributeCollector>,
        mut var_cref: metamodelica::Ref<ComponentRef::NFComponentRef>,
        mut solved_eq: metamodelica::Ref<Equation::Equation>,
    ) -> Result<metamodelica::Ref<AttributeCollector>> {
        let mut attrcollector: metamodelica::Ref<AttributeCollector> = attrcollector;
        let mut repl: metamodelica::Ref<
            UnorderedMap::UnorderedMap<
                metamodelica::Ref<ComponentRef::NFComponentRef>,
                metamodelica::Ref<Expression::NFExpression>,
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
            1,
        );
        let mut rhs: metamodelica::Ref<Expression::NFExpression>;
        let mut new_rhs: metamodelica::Ref<Expression::NFExpression>;
        let mut diff_rhs: metamodelica::Ref<Expression::NFExpression>;
        let mut args: metamodelica::Ref<DifferentiationArguments::DifferentiationArguments>;
        let mut swap_min_max: bool;
        let mut min_val_opt: Option<metamodelica::Ref<Expression::NFExpression>> =
            UnorderedMap::get(var_cref.clone(), attrcollector.min_val_map.clone())?;
        let mut max_val_opt: Option<metamodelica::Ref<Expression::NFExpression>> =
            UnorderedMap::get(var_cref.clone(), attrcollector.max_val_map.clone())?;
        let mut start_opt: Option<metamodelica::Ref<Expression::NFExpression>> =
            UnorderedMap::get(var_cref.clone(), attrcollector.start_map.clone())?;
        let mut nominal_opt: Option<metamodelica::Ref<Expression::NFExpression>> =
            UnorderedMap::get(var_cref.clone(), attrcollector.nominal_map.clone())?;
        let mut ty: metamodelica::Ref<Type::NFType>;
        let __pa0 = ::match_deref::match_deref! { match &(BEquation::Equation::getRHS(solved_eq)?) {
            Some(__pa0) => __pa0.clone(),
            _ => return Err("pattern mismatch"),
        } };
        rhs = metamodelica::Own::own(__pa0);
        if (min_val_opt).is_some() {
            UnorderedMap::add(var_cref.clone(), Util::getOption(min_val_opt)?, repl.clone())?;
            new_rhs = Expression::map(
                rhs.clone(),
                (std::sync::Arc::new({
                    let __pe_b1 = repl.clone();
                    move |__pe_a0| Replacements::applySimpleExp(__pe_a0, __pe_b1.clone())
                })
                    as std::sync::Arc<
                        dyn ::std::ops::Fn(
                                metamodelica::Ref<Expression::NFExpression>,
                            )
                                -> Result<metamodelica::Ref<Expression::NFExpression>>
                            + 'static,
                    >),
            )?;
            new_rhs = SimplifyExp::simplify(new_rhs, false)?;
            UnorderedMap::add(var_cref.clone(), new_rhs, attrcollector.min_val_map.clone())?;
            min_val_opt = UnorderedMap::get(var_cref.clone(), attrcollector.min_val_map.clone())?;
        }
        if (max_val_opt).is_some() {
            UnorderedMap::add(var_cref.clone(), Util::getOption(max_val_opt)?, repl.clone())?;
            new_rhs = Expression::map(
                rhs.clone(),
                (std::sync::Arc::new({
                    let __pe_b1 = repl.clone();
                    move |__pe_a0| Replacements::applySimpleExp(__pe_a0, __pe_b1.clone())
                })
                    as std::sync::Arc<
                        dyn ::std::ops::Fn(
                                metamodelica::Ref<Expression::NFExpression>,
                            )
                                -> Result<metamodelica::Ref<Expression::NFExpression>>
                            + 'static,
                    >),
            )?;
            new_rhs = SimplifyExp::simplify(new_rhs, false)?;
            UnorderedMap::add(var_cref.clone(), new_rhs, attrcollector.max_val_map.clone())?;
            max_val_opt = UnorderedMap::get(var_cref.clone(), attrcollector.max_val_map.clone())?;
        }
        ty = Expression::typeOf(rhs.clone());
        if !(Type::isContinuous(ty.clone())?)
            || !(Type::isInteger(&(Type::elementType(ty.clone())))?)
            || Type::isRecord(&ty)
        {
            swap_min_max = false;
        } else {
            args = Differentiate::DifferentiationArguments::default(
                Differentiate::DifferentiationType::SIMPLE.clone(),
                UnorderedMap::new(
                    (std::sync::Arc::new(move |__a0: metamodelica::Ref<Absyn::Path>| -> metamodelica::Result<_> {
                        ::std::result::Result::Ok(AbsynUtil::pathHash(&__a0))
                    })
                        as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<Absyn::Path>) -> Result<i32> + 'static>),
                    (std::sync::Arc::new(
                        move |__a0: metamodelica::Ref<Absyn::Path>,
                              __a1: metamodelica::Ref<Absyn::Path>|
                              -> metamodelica::Result<_> {
                            ::std::result::Result::Ok(AbsynUtil::pathEqual(&__a0, &__a1))
                        },
                    )
                        as std::sync::Arc<
                            dyn ::std::ops::Fn(
                                    metamodelica::Ref<Absyn::Path>,
                                    metamodelica::Ref<Absyn::Path>,
                                ) -> Result<bool>
                                + 'static,
                        >),
                    1,
                ),
            );
            assign_field!(args.diffCref = var_cref.clone());
            (diff_rhs, _) = Differentiate::differentiateExpression(rhs.clone(), args)?;
            diff_rhs = SimplifyExp::simplify(diff_rhs, false)?;
            swap_min_max = Expression::isNegative(&diff_rhs)?;
        }
        if swap_min_max && (min_val_opt).is_some() && (max_val_opt).is_some() {
            UnorderedMap::add(
                var_cref.clone(),
                Util::getOption(max_val_opt)?,
                attrcollector.min_val_map.clone(),
            )?;
            UnorderedMap::add(
                var_cref.clone(),
                Util::getOption(min_val_opt)?,
                attrcollector.max_val_map.clone(),
            )?;
        }
        if (start_opt).is_some() {
            UnorderedMap::add(var_cref.clone(), Util::getOption(start_opt)?, repl.clone())?;
            new_rhs = Expression::map(
                rhs.clone(),
                (std::sync::Arc::new({
                    let __pe_b1 = repl.clone();
                    move |__pe_a0| Replacements::applySimpleExp(__pe_a0, __pe_b1.clone())
                })
                    as std::sync::Arc<
                        dyn ::std::ops::Fn(
                                metamodelica::Ref<Expression::NFExpression>,
                            )
                                -> Result<metamodelica::Ref<Expression::NFExpression>>
                            + 'static,
                    >),
            )?;
            new_rhs = SimplifyExp::simplify(new_rhs, false)?;
            UnorderedMap::add(var_cref.clone(), new_rhs, attrcollector.start_map.clone())?;
        }
        if (nominal_opt).is_some() {
            UnorderedMap::add(var_cref.clone(), Util::getOption(nominal_opt)?, repl.clone())?;
            new_rhs = Expression::map(
                rhs,
                (std::sync::Arc::new({
                    let __pe_b1 = repl;
                    move |__pe_a0| Replacements::applySimpleExp(__pe_a0, __pe_b1.clone())
                })
                    as std::sync::Arc<
                        dyn ::std::ops::Fn(
                                metamodelica::Ref<Expression::NFExpression>,
                            )
                                -> Result<metamodelica::Ref<Expression::NFExpression>>
                            + 'static,
                    >),
            )?;
            new_rhs = Expression::getNominal(new_rhs)?;
            UnorderedMap::add(var_cref, new_rhs, attrcollector.nominal_map.clone())?;
        }
        Ok(attrcollector)
    }
}
