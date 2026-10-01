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
use crate::NBAdjacency::Mapping;
use crate::NBDifferentiate as Differentiate;
use crate::NBDifferentiate::DifferentiationArguments;
use crate::NBDifferentiate::DifferentiationType;
use crate::NBEquation as BEquation;
use crate::NBEquation;
use crate::NBEquation::EqData;
use crate::NBEquation::Equation;
use crate::NBEquation::EquationPointers;
use crate::NBEquation::Iterator;
use crate::NBMatching as Matching;
use crate::NBModule as Module;
use crate::NBPartition as Partition;
use crate::NBReplacements as Replacements;
use crate::NBSlice as Slice;
use crate::NBSolve;
use crate::NBSorting as Sorting;
use crate::NBStrongComponent as StrongComponent;
use crate::NBTearing as Tearing;
use crate::NBVariable as BVariable;
use crate::NBVariable;
use crate::NBVariable::VarData;
use crate::NBVariable::VariablePointer;
use crate::NBVariable::VariablePointers;
use crate::NBackendDAE as BackendDAE;
use crate::NBackendDAE as Jacobian;
use openmodelica_ast::Absyn::Path;
use openmodelica_backend_util::Coloring;
use openmodelica_frontend_types::DAE;
use openmodelica_nf_frontend::NFAlgorithm as Algorithm;
use openmodelica_nf_frontend::NFBackendExtension::BackendInfo;
use openmodelica_nf_frontend::NFComponentRef as ComponentRef;
use openmodelica_nf_frontend::NFExpression as Expression;
use openmodelica_nf_frontend::NFFunction::Function;
use openmodelica_nf_frontend::NFInstNode::InstNode;
use openmodelica_nf_frontend::NFOperator as Operator;
use openmodelica_nf_frontend::NFOperator::MathClassification;
use openmodelica_nf_frontend::NFOperator::SizeClassification;
use openmodelica_nf_frontend::NFScalarize as Scalarize;
use openmodelica_nf_frontend::NFSimplifyExp as SimplifyExp;
use openmodelica_nf_frontend::NFStatement as Statement;
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
use openmodelica_util_datatypes_basic::PointerWeak;

// OF imports
// NF imports
// Backend imports
// Sparsity-pattern graph coloring, shared with the old backend.
// Util imports
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, metamodelica::ReferenceEq)]
#[repr(i32)]
pub enum JacobianType {
    ODE = 1,
    DAE = 2,
    LS = 3,
    NLS = 4,
    OPT_LFG = 5,
    OPT_MRF = 6,
    OPT_R0 = 7,
}
impl PartialOrd for JacobianType {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for JacobianType {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (*self as i32).cmp(&(*other as i32))
    }
}
impl metamodelica::gc::MMTrace for JacobianType {
    fn mm_accept(&self, _: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        Ok(())
    }
}

pub(crate) fn isDynamic(mut jacType: JacobianType) -> bool {
    let mut b: bool;
    b = (match jacType {
        JacobianType::ODE => true,
        JacobianType::DAE { .. } => true,
        JacobianType::OPT_LFG => true,
        JacobianType::OPT_MRF => true,
        JacobianType::OPT_R0 => true,
        _ => false,
    });
    b
}

pub(crate) fn main(
    mut bdae: metamodelica::Ref<Jacobian::NBackendDAE>,
    mut kind: Partition::Kind,
) -> Result<metamodelica::Ref<Jacobian::NBackendDAE>> {
    let mut bdae: metamodelica::Ref<Jacobian::NBackendDAE> = bdae;
    let func: Module::jacobianInterface = getModule()?;
    bdae = (::match_deref::match_deref! { match &(bdae.clone()) {
        Deref @ Jacobian::MAIN { varData: Deref @ NBVariable::VarData::VAR_DATA_SIM { knowns, .. }, .. } => {
            let mut name: ArcStr;
            if Flags::isSet(Flags::JAC_DUMP.clone())? {
                metamodelica::print({ let mut __mm_s = String::new(); __mm_s.push_str(&*StringUtil::headline_1(&(literal!("[symjacdump] Creating symbolic Jacobians:")))?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) });
            }
            name = (match kind {
        Partition::Kind::ODE => {
            name = literal!("ODE_JAC");
            assign_variant_field!(bdae => Jacobian::NBackendDAE::MAIN; ode = applyToPartitions(var_field!((*bdae).ode, Jacobian::NBackendDAE::MAIN).clone(), var_field!((*bdae).funcMap, Jacobian::NBackendDAE::MAIN).clone(), metamodelica::AsArg::as_arg(&knowns), name.clone(), &*(func.clone()), true)?.0);
            name
        },
        Partition::Kind::DAE => {
            name = literal!("DAE_JAC");
            assign_variant_field!(bdae => Jacobian::NBackendDAE::MAIN; dae = Some((applyToPartitions(var_field!((*bdae).dae, Jacobian::NBackendDAE::MAIN).clone().ok_or("pattern mismatch")?, var_field!((*bdae).funcMap, Jacobian::NBackendDAE::MAIN).clone(), metamodelica::AsArg::as_arg(&knowns), name.clone(), &*(func.clone()), true)?).0));
            name
        },
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![{ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBJacobian.main")); __mm_s.push_str(&*literal!(" failed for: ")); __mm_s.push_str(&*Partition::Partition::kindToString(kind)?); ArcStr::from(__mm_s) }])?;
            return Err("fail")
        },
    });
            assign_variant_field!(bdae => Jacobian::NBackendDAE::MAIN;
                ode_event = applyToPartitions(var_field!((*bdae).ode_event, Jacobian::NBackendDAE::MAIN).clone(), var_field!((*bdae).funcMap, Jacobian::NBackendDAE::MAIN).clone(), metamodelica::AsArg::as_arg(&knowns), name.clone(), &*(func.clone()), kind != Partition::Kind::DAE.clone())?.0,
                algebraic = applyToPartitions(var_field!((*bdae).algebraic, Jacobian::NBackendDAE::MAIN).clone(), var_field!((*bdae).funcMap, Jacobian::NBackendDAE::MAIN).clone(), metamodelica::AsArg::as_arg(&knowns), name.clone(), &*(func.clone()), true)?.0,
                alg_event = applyToPartitions(var_field!((*bdae).alg_event, Jacobian::NBackendDAE::MAIN).clone(), var_field!((*bdae).funcMap, Jacobian::NBackendDAE::MAIN).clone(), metamodelica::AsArg::as_arg(&knowns), name.clone(), &*(func.clone()), true)?.0,
                init = applyToPartitions(var_field!((*bdae).init, Jacobian::NBackendDAE::MAIN).clone(), var_field!((*bdae).funcMap, Jacobian::NBackendDAE::MAIN).clone(), metamodelica::AsArg::as_arg(&knowns), name.clone(), &*(func.clone()), true)?.0
            );
            if (var_field!((*bdae).init_0, Jacobian::NBackendDAE::MAIN)).is_some() {
                assign_variant_field!(bdae => Jacobian::NBackendDAE::MAIN; init_0 = Some((applyToPartitions(var_field!((*bdae).init_0, Jacobian::NBackendDAE::MAIN).clone().ok_or("pattern mismatch")?, var_field!((*bdae).funcMap, Jacobian::NBackendDAE::MAIN).clone(), metamodelica::AsArg::as_arg(&knowns), name, &*(func.clone()), true)?).0));
            }
            bdae
        },
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![{ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBJacobian.main")); __mm_s.push_str(&*literal!(" failed for: ")); __mm_s.push_str(&*Jacobian::toString(&bdae, literal!(""))?); ArcStr::from(__mm_s) }])?;
            return Err("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(bdae)
}

pub(crate) fn applyToPartitions(
    mut partitions: metamodelica::List<metamodelica::Ref<Partition::Partition::Partition>>,
    mut funcMap: metamodelica::Ref<
        UnorderedMap::UnorderedMap<metamodelica::Ref<Path>, metamodelica::Ref<Function::Function>>,
    >,
    mut knowns: &metamodelica::Ref<VariablePointers::VariablePointers>,
    mut name: ArcStr,
    mut func: &dyn ::std::ops::Fn(
        ArcStr,
        JacobianType,
        metamodelica::Ref<VariablePointers::VariablePointers>,
        metamodelica::Ref<VariablePointers::VariablePointers>,
        metamodelica::Ref<EquationPointers::EquationPointers>,
        Option<metamodelica::Array<metamodelica::Ref<StrongComponent::NBStrongComponent>>>,
        Option<metamodelica::Ref<Adjacency::Matrix::Matrix>>,
        metamodelica::Ref<UnorderedMap::UnorderedMap<metamodelica::Ref<Path>, metamodelica::Ref<Function::Function>>>,
        bool,
    ) -> Result<Option<metamodelica::Ref<Jacobian::NBackendDAE>>>,
    mut simJacobian: bool,
) -> Result<(
    metamodelica::List<metamodelica::Ref<Partition::Partition::Partition>>,
    metamodelica::Ref<UnorderedMap::UnorderedMap<metamodelica::Ref<Path>, metamodelica::Ref<Function::Function>>>,
)> {
    let mut partitions: metamodelica::List<metamodelica::Ref<Partition::Partition::Partition>> = partitions;
    let mut funcMap: metamodelica::Ref<
        UnorderedMap::UnorderedMap<metamodelica::Ref<Path>, metamodelica::Ref<Function::Function>>,
    > = funcMap;
    partitions = ({
        let mut __acc: metamodelica::List<metamodelica::Ref<Partition::Partition::Partition>> = metamodelica::nil();
        for mut part in (partitions).into_iter().cloned() {
            let __x = partJacobian(part.clone(), funcMap.clone(), knowns, name.clone(), func, simJacobian)?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    Ok((partitions, funcMap))
}

pub(crate) fn nonlinear(
    mut seedCandidates: metamodelica::Ref<VariablePointers::VariablePointers>,
    mut partialCandidates: metamodelica::Ref<VariablePointers::VariablePointers>,
    mut equations: metamodelica::Ref<EquationPointers::EquationPointers>,
    mut comps: metamodelica::Array<metamodelica::Ref<StrongComponent::NBStrongComponent>>,
    mut full: Option<metamodelica::Ref<Adjacency::Matrix::Matrix>>,
    mut funcMap: metamodelica::Ref<
        UnorderedMap::UnorderedMap<metamodelica::Ref<Path>, metamodelica::Ref<Function::Function>>,
    >,
    mut name: ArcStr,
    mut staticAsContinuous: bool,
) -> Result<Option<metamodelica::Ref<Jacobian::NBackendDAE>>> {
    let mut jacobian: Option<metamodelica::Ref<Jacobian::NBackendDAE>>;
    let func: Module::jacobianInterface = if (Flags::isSet(Flags::NLS_ANALYTIC_JACOBIAN.clone())?) {
        (std::sync::Arc::new(jacobianSymbolic)
            as std::sync::Arc<
                dyn ::std::ops::Fn(
                        ArcStr,
                        JacobianType,
                        metamodelica::Ref<VariablePointers::VariablePointers>,
                        metamodelica::Ref<VariablePointers::VariablePointers>,
                        metamodelica::Ref<EquationPointers::EquationPointers>,
                        Option<metamodelica::Array<metamodelica::Ref<StrongComponent::NBStrongComponent>>>,
                        Option<metamodelica::Ref<Adjacency::Matrix::Matrix>>,
                        metamodelica::Ref<
                            UnorderedMap::UnorderedMap<metamodelica::Ref<Path>, metamodelica::Ref<Function::Function>>,
                        >,
                        bool,
                    ) -> Result<Option<metamodelica::Ref<Jacobian::NBackendDAE>>>
                    + 'static,
            >)
    } else {
        (std::sync::Arc::new(jacobianNumeric)
            as std::sync::Arc<
                dyn ::std::ops::Fn(
                        ArcStr,
                        JacobianType,
                        metamodelica::Ref<VariablePointers::VariablePointers>,
                        metamodelica::Ref<VariablePointers::VariablePointers>,
                        metamodelica::Ref<EquationPointers::EquationPointers>,
                        Option<metamodelica::Array<metamodelica::Ref<StrongComponent::NBStrongComponent>>>,
                        Option<metamodelica::Ref<Adjacency::Matrix::Matrix>>,
                        metamodelica::Ref<
                            UnorderedMap::UnorderedMap<metamodelica::Ref<Path>, metamodelica::Ref<Function::Function>>,
                        >,
                        bool,
                    ) -> Result<Option<metamodelica::Ref<Jacobian::NBackendDAE>>>
                    + 'static,
            >)
    };
    match '__try0: {
        jacobian = unwrap_break_err!(func(name.clone(), JacobianType::NLS.clone(), seedCandidates.clone(), partialCandidates.clone(), equations.clone(), Some(comps.clone()), full.clone(), funcMap.clone(), staticAsContinuous), '__try0);
        Ok::<_, &'static str>((jacobian.clone(),))
    } {
        Ok((__try0_o0,)) => {
            jacobian = __try0_o0;
        }
        Err(_) => {
            jacobian = jacobianNumeric(
                name.clone(),
                JacobianType::NLS.clone(),
                seedCandidates.clone(),
                partialCandidates.clone(),
                equations.clone(),
                Some(comps.clone()),
                full.clone(),
                funcMap.clone(),
                staticAsContinuous,
            )?;
        }
    }
    Ok(jacobian)
}

pub(crate) fn combine(
    mut jacobians: &metamodelica::List<metamodelica::Ref<Jacobian::NBackendDAE>>,
    mut name: ArcStr,
) -> Result<metamodelica::Ref<Jacobian::NBackendDAE>> {
    let mut jacobian: metamodelica::Ref<Jacobian::NBackendDAE>;
    let mut jacType: JacobianType = JacobianType::NLS.clone();
    let mut variables: metamodelica::List<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>> =
        metamodelica::nil();
    let mut unknowns: metamodelica::List<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>> =
        metamodelica::nil();
    let mut auxiliaryVars: metamodelica::List<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>> =
        metamodelica::nil();
    let mut aliasVars: metamodelica::List<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>> =
        metamodelica::nil();
    let mut diffVars: metamodelica::List<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>> =
        metamodelica::nil();
    let mut dependencies: metamodelica::List<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>> =
        metamodelica::nil();
    let mut resultVars: metamodelica::List<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>> =
        metamodelica::nil();
    let mut tmpVars: metamodelica::List<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>> =
        metamodelica::nil();
    let mut seedVars: metamodelica::List<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>> =
        metamodelica::nil();
    let mut comps: metamodelica::List<metamodelica::Ref<StrongComponent::NBStrongComponent>> = metamodelica::nil();
    let mut sparsity_patterns: metamodelica::List<metamodelica::Ref<Adjacency::Matrix::Matrix>> = metamodelica::nil();
    let mut varData: metamodelica::Ref<VarData::VarData>;
    if List::hasOneElement(jacobians) {
        jacobian = (jacobians).head().cloned()?;
        jacobian = (match &*jacobian {
            Jacobian::JACOBIAN { .. } => {
                assign_variant_field!(jacobian => Jacobian::NBackendDAE::JACOBIAN; name = name);
                jacobian
            }
            _ => {
                Error::addMessage(
                    Error::INTERNAL_ERROR.clone(),
                    list![{
                        let mut __mm_s = String::new();
                        __mm_s.push_str(&*literal!("NBJacobian.combine"));
                        __mm_s.push_str(&*literal!(" failed for\n"));
                        __mm_s.push_str(&*Jacobian::toString(&jacobian, literal!(""))?);
                        ArcStr::from(__mm_s)
                    }],
                )?;
                return Err("fail");
            }
        });
    } else {
        for mut jac in &**jacobians {
            let () = (::match_deref::match_deref! { match &(jac.clone()) {
                Deref @ Jacobian::JACOBIAN { varData: tmpVarData @ Deref @ NBVariable::VarData::VAR_DATA_JAC { .. }, jacType: __jac_jacType, sparsity: __jac_sparsity, .. } => {
                    jacType = __jac_jacType.clone();
                    variables = listAppend(NBVariable::VariablePointers::toList(var_field!((**tmpVarData).variables, VarData::VarData::VAR_DATA_JAC))?, variables);
                    unknowns = listAppend(NBVariable::VariablePointers::toList(var_field!((**tmpVarData).unknowns, VarData::VarData::VAR_DATA_JAC))?, unknowns);
                    auxiliaryVars = listAppend(NBVariable::VariablePointers::toList(var_field!((**tmpVarData).auxiliaries, VarData::VarData::VAR_DATA_JAC))?, auxiliaryVars);
                    aliasVars = listAppend(NBVariable::VariablePointers::toList(var_field!((**tmpVarData).aliasVars, VarData::VarData::VAR_DATA_JAC))?, aliasVars);
                    diffVars = listAppend(NBVariable::VariablePointers::toList(var_field!((**tmpVarData).diffVars, VarData::VarData::VAR_DATA_JAC))?, diffVars);
                    dependencies = listAppend(NBVariable::VariablePointers::toList(var_field!((**tmpVarData).dependencies, VarData::VarData::VAR_DATA_JAC))?, dependencies);
                    resultVars = listAppend(NBVariable::VariablePointers::toList(var_field!((**tmpVarData).resultVars, VarData::VarData::VAR_DATA_JAC))?, resultVars);
                    tmpVars = listAppend(NBVariable::VariablePointers::toList(var_field!((**tmpVarData).tmpVars, VarData::VarData::VAR_DATA_JAC))?, tmpVars);
                    seedVars = listAppend(NBVariable::VariablePointers::toList(var_field!((**tmpVarData).seedVars, VarData::VarData::VAR_DATA_JAC))?, seedVars);
                    comps = listAppend(var_field!((**jac).comps, Jacobian::NBackendDAE::JACOBIAN).clone().borrow().iter().cloned().collect::<metamodelica::List<_>>(), comps);
                    sparsity_patterns = metamodelica::cons(__jac_sparsity.clone(), sparsity_patterns);
                    ()
                },
                _ => {
                    Error::addMessage(Error::INTERNAL_ERROR.clone(), list![{ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBJacobian.combine")); __mm_s.push_str(&*literal!(" failed for\n")); __mm_s.push_str(&*Jacobian::toString(metamodelica::AsArg::as_arg(&jac), literal!(""))?); ArcStr::from(__mm_s) }])?;
                    return Err("fail")
                },
                _ => unreachable!("match_deref! exhaustiveness placeholder"),
            } });
        }
        varData = metamodelica::Ref::new(VarData::VarData::VAR_DATA_JAC {
            variables: NBVariable::VariablePointers::fromList(&variables, false)?,
            unknowns: NBVariable::VariablePointers::fromList(&unknowns, false)?,
            auxiliaries: NBVariable::VariablePointers::fromList(&auxiliaryVars, false)?,
            aliasVars: NBVariable::VariablePointers::fromList(&aliasVars, false)?,
            diffVars: NBVariable::VariablePointers::fromList(&diffVars, false)?,
            dependencies: NBVariable::VariablePointers::fromList(&dependencies, false)?,
            resultVars: NBVariable::VariablePointers::fromList(&resultVars, false)?,
            tmpVars: NBVariable::VariablePointers::fromList(&tmpVars, false)?,
            seedVars: NBVariable::VariablePointers::fromList(&seedVars, false)?,
        });
        jacobian = metamodelica::Ref::new(Jacobian::NBackendDAE::JACOBIAN {
            name: name.clone(),
            jacType: jacType,
            varData: varData,
            comps: metamodelica::arrayFromVec(comps.into_iter().cloned().collect()),
            sparsity: Adjacency::Matrix::combine(sparsity_patterns)?,
            isAdjoint: metamodelica::stringEq(&name, &(literal!("ADJ"))),
        });
    }
    Ok(jacobian)
}

pub(crate) fn getModule() -> Result<
    Arc<
        dyn ::std::ops::Fn(
                ArcStr,
                JacobianType,
                metamodelica::Ref<VariablePointers::VariablePointers>,
                metamodelica::Ref<VariablePointers::VariablePointers>,
                metamodelica::Ref<EquationPointers::EquationPointers>,
                Option<metamodelica::Array<metamodelica::Ref<StrongComponent::NBStrongComponent>>>,
                Option<metamodelica::Ref<Adjacency::Matrix::Matrix>>,
                metamodelica::Ref<
                    UnorderedMap::UnorderedMap<metamodelica::Ref<Path>, metamodelica::Ref<Function::Function>>,
                >,
                bool,
            ) -> Result<Option<metamodelica::Ref<Jacobian::NBackendDAE>>>
            + 'static,
    >,
> {
    let mut func: Module::jacobianInterface;
    func = (::match_deref::match_deref! { match &(Flags::getConfigString(Flags::GENERATE_DYNAMIC_JACOBIAN.clone())?) {
        Deref @ "symbolic" => (std::sync::Arc::new(jacobianSymbolic) as std::sync::Arc<dyn ::std::ops::Fn(ArcStr, JacobianType, metamodelica::Ref<VariablePointers::VariablePointers>, metamodelica::Ref<VariablePointers::VariablePointers>, metamodelica::Ref<EquationPointers::EquationPointers>, Option<metamodelica::Array<metamodelica::Ref<StrongComponent::NBStrongComponent>>>, Option<metamodelica::Ref<Adjacency::Matrix::Matrix>>, metamodelica::Ref<UnorderedMap::UnorderedMap<metamodelica::Ref<Path>, metamodelica::Ref<Function::Function>>>, bool) -> Result<Option<metamodelica::Ref<Jacobian::NBackendDAE>>> + 'static>),
        Deref @ "symbolicadjoint" => (std::sync::Arc::new(jacobianSymbolicAdjoint) as std::sync::Arc<dyn ::std::ops::Fn(ArcStr, JacobianType, metamodelica::Ref<VariablePointers::VariablePointers>, metamodelica::Ref<VariablePointers::VariablePointers>, metamodelica::Ref<EquationPointers::EquationPointers>, Option<metamodelica::Array<metamodelica::Ref<StrongComponent::NBStrongComponent>>>, Option<metamodelica::Ref<Adjacency::Matrix::Matrix>>, metamodelica::Ref<UnorderedMap::UnorderedMap<metamodelica::Ref<Path>, metamodelica::Ref<Function::Function>>>, bool) -> Result<Option<metamodelica::Ref<Jacobian::NBackendDAE>>> + 'static>),
        Deref @ "bidirectional" => (std::sync::Arc::new(jacobianSymbolic) as std::sync::Arc<dyn ::std::ops::Fn(ArcStr, JacobianType, metamodelica::Ref<VariablePointers::VariablePointers>, metamodelica::Ref<VariablePointers::VariablePointers>, metamodelica::Ref<EquationPointers::EquationPointers>, Option<metamodelica::Array<metamodelica::Ref<StrongComponent::NBStrongComponent>>>, Option<metamodelica::Ref<Adjacency::Matrix::Matrix>>, metamodelica::Ref<UnorderedMap::UnorderedMap<metamodelica::Ref<Path>, metamodelica::Ref<Function::Function>>>, bool) -> Result<Option<metamodelica::Ref<Jacobian::NBackendDAE>>> + 'static>),
        Deref @ "numeric" => (std::sync::Arc::new(jacobianNumeric) as std::sync::Arc<dyn ::std::ops::Fn(ArcStr, JacobianType, metamodelica::Ref<VariablePointers::VariablePointers>, metamodelica::Ref<VariablePointers::VariablePointers>, metamodelica::Ref<EquationPointers::EquationPointers>, Option<metamodelica::Array<metamodelica::Ref<StrongComponent::NBStrongComponent>>>, Option<metamodelica::Ref<Adjacency::Matrix::Matrix>>, metamodelica::Ref<UnorderedMap::UnorderedMap<metamodelica::Ref<Path>, metamodelica::Ref<Function::Function>>>, bool) -> Result<Option<metamodelica::Ref<Jacobian::NBackendDAE>>> + 'static>),
        Deref @ "none" => (std::sync::Arc::new(fnptr!(jacobianNone, ArcStr, JacobianType, metamodelica::Ref<VariablePointers::VariablePointers>, metamodelica::Ref<VariablePointers::VariablePointers>, metamodelica::Ref<EquationPointers::EquationPointers>, Option<metamodelica::Array<metamodelica::Ref<StrongComponent::NBStrongComponent>>>, Option<metamodelica::Ref<Adjacency::Matrix::Matrix>>, metamodelica::Ref<UnorderedMap::UnorderedMap<metamodelica::Ref<Path>, metamodelica::Ref<Function::Function>>>, bool)) as std::sync::Arc<dyn ::std::ops::Fn(ArcStr, JacobianType, metamodelica::Ref<VariablePointers::VariablePointers>, metamodelica::Ref<VariablePointers::VariablePointers>, metamodelica::Ref<EquationPointers::EquationPointers>, Option<metamodelica::Array<metamodelica::Ref<StrongComponent::NBStrongComponent>>>, Option<metamodelica::Ref<Adjacency::Matrix::Matrix>>, metamodelica::Ref<UnorderedMap::UnorderedMap<metamodelica::Ref<Path>, metamodelica::Ref<Function::Function>>>, bool) -> Result<Option<metamodelica::Ref<Jacobian::NBackendDAE>>> + 'static>),
        _ => {
            Error::addMessage(Error::INTERNAL_ERROR.clone(), list![{ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBJacobian.getModule")); __mm_s.push_str(&*literal!(" failed because of unknown jacobian type: ")); __mm_s.push_str(&*Flags::getConfigString(Flags::GENERATE_DYNAMIC_JACOBIAN.clone())?); ArcStr::from(__mm_s) }])?;
            return Err("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(func)
}

pub(crate) fn toString(mut jacobian: &metamodelica::Ref<Jacobian::NBackendDAE>, mut r#str: ArcStr) -> Result<ArcStr> {
    let mut r#str: ArcStr = r#str;
    r#str = Jacobian::toString(jacobian, r#str)?;
    Ok(r#str)
}

pub(crate) fn jacobianTypeString(mut jacType: JacobianType) -> ArcStr {
    let mut r#str: ArcStr;
    r#str = (match jacType {
        JacobianType::ODE => literal!("[ODE]"),
        JacobianType::DAE { .. } => literal!("[DAE]"),
        JacobianType::LS => literal!("[LS-]"),
        JacobianType::NLS => literal!("[NLS]"),
        JacobianType::OPT_LFG => literal!("[OPT-LFG]"),
        JacobianType::OPT_MRF => literal!("[OPT-MRF]"),
        JacobianType::OPT_R0 => literal!("[OPT-R0]"),
        _ => literal!("[ERR]"),
    });
    r#str
}

// ToDo: all the DAEMode stuff is probably incorrect!
// TODO: refactor with map
fn getOptimizableVars(
    mut variables: &metamodelica::Ref<VariablePointers::VariablePointers>,
) -> Result<metamodelica::List<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>>> {
    let mut optimizable_vars: metamodelica::List<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>> =
        metamodelica::nil();
    for mut var_ptr in &*NBVariable::VariablePointers::toList(variables)? {
        if NBVariable::isOptimizable(var_ptr.clone()) {
            optimizable_vars = metamodelica::cons(var_ptr.clone(), optimizable_vars);
        }
    }
    Ok(optimizable_vars)
}

fn getSeedCandidatesDynamicOptimization(
    mut part: &metamodelica::Ref<Partition::Partition::Partition>,
    mut all_knowns: &metamodelica::Ref<VariablePointers::VariablePointers>,
    mut filter: Arc<
        dyn ::std::ops::Fn(Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>) -> Result<bool> + 'static,
    >,
) -> Result<metamodelica::List<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>>> {
    let mut unknowns: metamodelica::List<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>>;
    let mut derivative_vars: metamodelica::List<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>>;
    let mut unknown_states: metamodelica::List<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>>;
    unknowns = getOptimizableVars(all_knowns)?;
    derivative_vars = ({
        let mut __acc: metamodelica::List<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>> =
            metamodelica::nil();
        for mut var in (NBVariable::VariablePointers::toList(&part.unknowns)?)
            .into_iter()
            .cloned()
        {
            if !(NBVariable::isStateDerivative(var.clone())) {
                continue;
            }
            let __x = var.clone();
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    unknown_states = ({
        let mut __acc: metamodelica::List<_> = metamodelica::nil();
        for mut var in (derivative_vars).into_iter().cloned() {
            let __x = ((NBVariable::getVarState(var.clone())?).0).ok_or("pattern mismatch")?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    unknowns = listAppend(unknown_states, unknowns);
    unknowns = List::filterOnTrue(unknowns, filter.clone())?;
    Ok(unknowns)
}

fn getLfgPartialCandidates(
    mut part: &metamodelica::Ref<Partition::Partition::Partition>,
) -> Result<metamodelica::List<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>>> {
    let mut partialCandidates: metamodelica::List<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>>;
    let mut lagrange_vars: metamodelica::List<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>> =
        metamodelica::nil();
    let mut derivative_vars: metamodelica::List<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>> =
        metamodelica::nil();
    let mut path_vars: metamodelica::List<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>> =
        metamodelica::nil();
    for mut var_ptr in &*NBVariable::VariablePointers::toList(&part.unknowns)? {
        if NBVariable::isLagrange(var_ptr.clone()) {
            lagrange_vars = metamodelica::cons(var_ptr.clone(), lagrange_vars);
        } else if NBVariable::isStateDerivative(var_ptr.clone()) {
            derivative_vars = metamodelica::cons(var_ptr.clone(), derivative_vars);
        } else if NBVariable::isPathConstraint(var_ptr.clone()) {
            path_vars = metamodelica::cons(var_ptr.clone(), path_vars);
        }
    }
    partialCandidates = listAppend(lagrange_vars, listAppend(derivative_vars, path_vars)).reverse();
    Ok(partialCandidates)
}

fn getMrfPartialCandidates(
    mut part: &metamodelica::Ref<Partition::Partition::Partition>,
) -> Result<metamodelica::List<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>>> {
    let mut partialCandidates: metamodelica::List<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>>;
    let mut mayer_vars: metamodelica::List<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>> =
        metamodelica::nil();
    let mut final_vars: metamodelica::List<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>> =
        metamodelica::nil();
    for mut var_ptr in &*NBVariable::VariablePointers::toList(&part.unknowns)? {
        if NBVariable::isMayer(var_ptr.clone()) {
            mayer_vars = metamodelica::cons(var_ptr.clone(), mayer_vars);
        } else if NBVariable::isFinalConstraint(var_ptr.clone()) {
            final_vars = metamodelica::cons(var_ptr.clone(), final_vars);
        }
    }
    partialCandidates = listAppend(mayer_vars, final_vars).reverse();
    Ok(partialCandidates)
}

fn getR0PartialCandidates(
    mut part: &metamodelica::Ref<Partition::Partition::Partition>,
) -> Result<metamodelica::List<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>>> {
    let mut partialCandidates: metamodelica::List<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>> =
        metamodelica::nil();
    for mut var_ptr in &*NBVariable::VariablePointers::toList(&part.unknowns)? {
        if NBVariable::isInitialConstraint(var_ptr.clone()) {
            partialCandidates = metamodelica::cons(var_ptr.clone(), partialCandidates);
        }
    }
    partialCandidates = partialCandidates.reverse();
    Ok(partialCandidates)
}

// TODO: before this is ever called, we should check if the variable / annotation pairs are even valid: e.g. path constraint with final time or so!
// add a module for optimization? where we check the model, may do some transformations etc?
fn partJacobianDynamicOptimization(
    mut part: &metamodelica::Ref<Partition::Partition::Partition>,
    mut all_knowns: &metamodelica::Ref<VariablePointers::VariablePointers>,
    mut name: ArcStr,
    mut func: &dyn ::std::ops::Fn(
        ArcStr,
        JacobianType,
        metamodelica::Ref<VariablePointers::VariablePointers>,
        metamodelica::Ref<VariablePointers::VariablePointers>,
        metamodelica::Ref<EquationPointers::EquationPointers>,
        Option<metamodelica::Array<metamodelica::Ref<StrongComponent::NBStrongComponent>>>,
        Option<metamodelica::Ref<Adjacency::Matrix::Matrix>>,
        metamodelica::Ref<UnorderedMap::UnorderedMap<metamodelica::Ref<Path>, metamodelica::Ref<Function::Function>>>,
        bool,
    ) -> Result<Option<metamodelica::Ref<Jacobian::NBackendDAE>>>,
    mut funcMap: metamodelica::Ref<
        UnorderedMap::UnorderedMap<metamodelica::Ref<Path>, metamodelica::Ref<Function::Function>>,
    >,
) -> Result<(
    Option<metamodelica::Ref<Jacobian::NBackendDAE>>,
    Option<metamodelica::Ref<Jacobian::NBackendDAE>>,
    Option<metamodelica::Ref<Jacobian::NBackendDAE>>,
)> {
    let mut LFG_jacobian: Option<metamodelica::Ref<Jacobian::NBackendDAE>>;
    let mut MRF_jacobian: Option<metamodelica::Ref<Jacobian::NBackendDAE>>;
    let mut R0_jacobian: Option<metamodelica::Ref<Jacobian::NBackendDAE>>;
    let mut staticAsContinuous: bool = true;
    let mut seedCandidates: metamodelica::Ref<VariablePointers::VariablePointers>;
    let mut partialCandidates: metamodelica::Ref<VariablePointers::VariablePointers>;
    partialCandidates = NBVariable::VariablePointers::fromList(
        &(listAppend(
            getLfgPartialCandidates(part)?,
            NBVariable::VariablePointers::toList(&part.unknowns)?,
        )),
        part.unknowns.scalarized.clone(),
    )?;
    seedCandidates = NBVariable::VariablePointers::fromList(
        &(getSeedCandidatesDynamicOptimization(
            part,
            all_knowns,
            (std::sync::Arc::new(fnptr!(
                NBVariable::isLfgVariable,
                Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>
            ))
                as std::sync::Arc<
                    dyn ::std::ops::Fn(Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>) -> Result<bool>
                        + 'static,
                >),
        )?),
        partialCandidates.scalarized.clone(),
    )?;
    LFG_jacobian = func(
        name.clone(),
        JacobianType::OPT_LFG.clone(),
        seedCandidates,
        partialCandidates,
        part.equations.clone(),
        part.strongComponents.clone(),
        part.adjacencyMatrix.clone(),
        funcMap.clone(),
        staticAsContinuous,
    )?;
    partialCandidates = NBVariable::VariablePointers::fromList(
        &(listAppend(
            getMrfPartialCandidates(part)?,
            NBVariable::VariablePointers::toList(&part.unknowns)?,
        )),
        part.unknowns.scalarized.clone(),
    )?;
    seedCandidates = NBVariable::VariablePointers::fromList(
        &(getSeedCandidatesDynamicOptimization(
            part,
            all_knowns,
            (std::sync::Arc::new(fnptr!(
                NBVariable::isMrfVariable,
                Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>
            ))
                as std::sync::Arc<
                    dyn ::std::ops::Fn(Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>) -> Result<bool>
                        + 'static,
                >),
        )?),
        partialCandidates.scalarized.clone(),
    )?;
    MRF_jacobian = func(
        name.clone(),
        JacobianType::OPT_MRF.clone(),
        seedCandidates,
        partialCandidates,
        part.equations.clone(),
        part.strongComponents.clone(),
        part.adjacencyMatrix.clone(),
        funcMap.clone(),
        staticAsContinuous,
    )?;
    partialCandidates = NBVariable::VariablePointers::fromList(
        &(listAppend(
            getR0PartialCandidates(part)?,
            NBVariable::VariablePointers::toList(&part.unknowns)?,
        )),
        part.unknowns.scalarized.clone(),
    )?;
    seedCandidates = NBVariable::VariablePointers::fromList(
        &(getSeedCandidatesDynamicOptimization(
            part,
            all_knowns,
            (std::sync::Arc::new(fnptr!(
                NBVariable::isR0Variable,
                Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>
            ))
                as std::sync::Arc<
                    dyn ::std::ops::Fn(Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>) -> Result<bool>
                        + 'static,
                >),
        )?),
        partialCandidates.scalarized.clone(),
    )?;
    R0_jacobian = func(
        name,
        JacobianType::OPT_R0.clone(),
        seedCandidates,
        partialCandidates,
        part.equations.clone(),
        part.strongComponents.clone(),
        part.adjacencyMatrix.clone(),
        funcMap,
        staticAsContinuous,
    )?;
    Ok((LFG_jacobian, MRF_jacobian, R0_jacobian))
}

fn partJacobian(
    mut part: metamodelica::Ref<Partition::Partition::Partition>,
    mut funcMap: metamodelica::Ref<
        UnorderedMap::UnorderedMap<metamodelica::Ref<Path>, metamodelica::Ref<Function::Function>>,
    >,
    mut knowns: &metamodelica::Ref<VariablePointers::VariablePointers>,
    mut name: ArcStr,
    mut func: &dyn ::std::ops::Fn(
        ArcStr,
        JacobianType,
        metamodelica::Ref<VariablePointers::VariablePointers>,
        metamodelica::Ref<VariablePointers::VariablePointers>,
        metamodelica::Ref<EquationPointers::EquationPointers>,
        Option<metamodelica::Array<metamodelica::Ref<StrongComponent::NBStrongComponent>>>,
        Option<metamodelica::Ref<Adjacency::Matrix::Matrix>>,
        metamodelica::Ref<UnorderedMap::UnorderedMap<metamodelica::Ref<Path>, metamodelica::Ref<Function::Function>>>,
        bool,
    ) -> Result<Option<metamodelica::Ref<Jacobian::NBackendDAE>>>,
    mut simJacobian: bool,
) -> Result<metamodelica::Ref<Partition::Partition::Partition>> {
    let mut part: metamodelica::Ref<Partition::Partition::Partition> = part;
    let mut jacType: JacobianType;
    let mut unknowns: metamodelica::Ref<VariablePointers::VariablePointers>;
    let mut derivative_vars: metamodelica::List<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>>;
    let mut state_vars: metamodelica::List<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>>;
    let mut seedCandidates: metamodelica::Ref<VariablePointers::VariablePointers>;
    let mut partialCandidates: metamodelica::Ref<VariablePointers::VariablePointers>;
    let mut jacobian: Option<metamodelica::Ref<Jacobian::NBackendDAE>>;
    let mut LFG_jacobian: Option<metamodelica::Ref<Jacobian::NBackendDAE>> = None;
    let mut MRF_jacobian: Option<metamodelica::Ref<Jacobian::NBackendDAE>> = None;
    let mut R0_jacobian: Option<metamodelica::Ref<Jacobian::NBackendDAE>> = None;
    let mut adjointJac: Option<metamodelica::Ref<Jacobian::NBackendDAE>>;
    let mut kind: Partition::Kind = Partition::Partition::getKind(&part);
    let mut updated: bool;
    assign_field!(
        part.strongComponents = (match part.strongComponents.clone() {
            Some(mut comps) => {
                let mut tmp: metamodelica::Ref<StrongComponent::NBStrongComponent>;
                for mut i in 1..=metamodelica::arrayLength(comps.clone()) {
                    (tmp, updated) = compJacobian(
                        ({
                            let __elt = (*metamodelica::index_checked(&comps.borrow(), i)?).clone();
                            __elt
                        }),
                        part.adjacencyMatrix.clone(),
                        funcMap.clone(),
                        kind,
                    )?;
                    if updated {
                        metamodelica::arrayUpdate(comps.clone(), i, tmp)?;
                    }
                }
                Some(comps.clone())
            }
            _ => {
                part.strongComponents.clone()
            }
        })
    );
    if simJacobian && Partition::Partition::isODEorDAE(&part) {
        partialCandidates = part.unknowns.clone();
        unknowns = if (Partition::Partition::getKind(&part) == Partition::Kind::DAE.clone()) {
            part.daeUnknowns.clone().ok_or("pattern mismatch")?
        } else {
            part.unknowns.clone()
        };
        jacType = if (Partition::Partition::getKind(&part) == Partition::Kind::DAE.clone()) {
            JacobianType::DAE.clone()
        } else {
            JacobianType::ODE.clone()
        };
        derivative_vars = ({
            let mut __acc: metamodelica::List<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>> =
                metamodelica::nil();
            for mut var in (NBVariable::VariablePointers::toList(&unknowns)?).into_iter().cloned() {
                if !(NBVariable::isStateDerivative(var.clone())) {
                    continue;
                }
                let __x = var.clone();
                __acc = cons(__x, __acc);
            }
            __acc.reverse()
        });
        state_vars = ({
            let mut __acc: metamodelica::List<_> = metamodelica::nil();
            for mut var in (derivative_vars).into_iter().cloned() {
                let __x = ((NBVariable::getVarState(var.clone())?).0).ok_or("pattern mismatch")?;
                __acc = cons(__x, __acc);
            }
            __acc.reverse()
        });
        seedCandidates = NBVariable::VariablePointers::fromList(&state_vars, partialCandidates.scalarized.clone())?;
        jacobian = func(
            name.clone(),
            jacType,
            seedCandidates.clone(),
            partialCandidates.clone(),
            part.equations.clone(),
            part.strongComponents.clone(),
            part.adjacencyMatrix.clone(),
            funcMap.clone(),
            Partition::kindIsInitial(kind),
        )?;
        if Flags::getConfigBool(Flags::MOO_DYNAMIC_OPTIMIZATION.clone())? {
            (LFG_jacobian, MRF_jacobian, R0_jacobian) =
                partJacobianDynamicOptimization(&part, knowns, name.clone(), func, funcMap.clone())?;
        }
        if metamodelica::stringEq(
            &(Flags::getConfigString(Flags::GENERATE_DYNAMIC_JACOBIAN.clone())?),
            &(literal!("bidirectional")),
        ) && (jacobian).is_some()
            && !(Jacobian::getIsAdjoint(&(jacobian.clone().ok_or("pattern mismatch")?))?)
        {
            adjointJac = jacobianSymbolicAdjoint(
                name,
                jacType,
                seedCandidates,
                partialCandidates,
                part.equations.clone(),
                part.strongComponents.clone(),
                part.adjacencyMatrix.clone(),
                funcMap,
                kind == Partition::Kind::INI.clone(),
            )?;
            assign_field!(
                part.association = metamodelica::Ref::new(Partition::Association::Association::CONTINUOUS {
                    kind: kind,
                    jacobian: jacobian,
                    jacobianAdjoint: adjointJac,
                    LFG_jacobian: LFG_jacobian,
                    MRF_jacobian: MRF_jacobian,
                    R0_jacobian: R0_jacobian
                })
            );
        } else if (jacobian).is_some() {
            if Jacobian::getIsAdjoint(&(jacobian.clone().ok_or("pattern mismatch")?))? {
                assign_field!(
                    part.association = metamodelica::Ref::new(Partition::Association::Association::CONTINUOUS {
                        kind: kind,
                        jacobian: None,
                        jacobianAdjoint: jacobian,
                        LFG_jacobian: LFG_jacobian,
                        MRF_jacobian: MRF_jacobian,
                        R0_jacobian: R0_jacobian
                    })
                );
            } else {
                assign_field!(
                    part.association = metamodelica::Ref::new(Partition::Association::Association::CONTINUOUS {
                        kind: kind,
                        jacobian: jacobian,
                        jacobianAdjoint: None,
                        LFG_jacobian: LFG_jacobian,
                        MRF_jacobian: MRF_jacobian,
                        R0_jacobian: R0_jacobian
                    })
                );
            }
        } else {
            assign_field!(
                part.association = metamodelica::Ref::new(Partition::Association::Association::CONTINUOUS {
                    kind: kind,
                    jacobian: None,
                    jacobianAdjoint: None,
                    LFG_jacobian: LFG_jacobian,
                    MRF_jacobian: MRF_jacobian,
                    R0_jacobian: R0_jacobian
                })
            );
        }
        if Flags::isSet(Flags::JAC_DUMP.clone())? {
            metamodelica::print(Partition::Partition::toString(&part, 2)?);
        }
    }
    Ok(part)
}

fn forEquationStart(mut eqn: &metamodelica::Ref<Equation::Equation>) -> i32 {
    let mut s: i32 = 0;
    let mut start_exp: metamodelica::Ref<Expression::NFExpression>;
    let () = (::match_deref::match_deref! { match eqn {
        Deref @ NBEquation::Equation::FOR_EQUATION { iter: Deref @ NBEquation::Iterator::SINGLE { range: Deref @ Expression::RANGE { start: __esc_start_exp, .. }, .. }, .. } => {
            start_exp = (*__esc_start_exp).clone();
            s = Expression::integerValueOrDefault(metamodelica::AsArg::as_arg(&start_exp), 0);
            ()
        },
        _ => (),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    s
}

fn partialSliceSeedCandidates(
    mut iteration_vars: &metamodelica::List<
        metamodelica::Ref<Slice::NBSlice<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>>>,
    >,
    mut residual_eqns: &metamodelica::List<
        metamodelica::Ref<Slice::NBSlice<Pointer::Pointer<metamodelica::Ref<Equation::Equation>>>>,
    >,
) -> Result<metamodelica::List<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>>> {
    let mut seed_candidates: metamodelica::List<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>> =
        metamodelica::nil();
    let mut for_start: i32 = 0;
    let mut s: i32;
    let mut slice_first_1based: i32;
    let mut elem_vars: metamodelica::List<metamodelica::Ref<Variable::NFVariable>>;
    let mut var_elem: metamodelica::Ref<Variable::NFVariable>;
    for mut eqn_slice in &**residual_eqns {
        s = forEquationStart(&(Pointer::access(Slice::getT(eqn_slice.clone()))));
        if s > 0 {
            if for_start == 0 {
                for_start = s;
            } else {
                for_start = intMin(for_start, s);
            }
        }
    }
    for mut var_slice in &**iteration_vars {
        if (var_slice.indices).is_empty() {
            seed_candidates = metamodelica::cons(Slice::getT(var_slice.clone()), seed_candidates);
        } else {
            var_elem = Pointer::access(Slice::getT(var_slice.clone()));
            slice_first_1based = (var_slice.indices).head().cloned()? + 1;
            if (for_start == 0 || for_start >= slice_first_1based)
                && (Type::isReal(&(Type::arrayElementType(&var_elem.ty)))?
                    || Type::isComplex(&(Type::arrayElementType(&var_elem.ty))))
            {
                elem_vars = Scalarize::scalarizeBackendVariable(&var_elem, var_slice.indices.clone())?;
                for mut v in &*elem_vars {
                    seed_candidates = metamodelica::cons(Pointer::create(v.clone()), seed_candidates);
                }
            } else {
                seed_candidates = metamodelica::cons(Slice::getT(var_slice.clone()), seed_candidates);
            }
        }
    }
    seed_candidates = seed_candidates.reverse();
    Ok(seed_candidates)
}

fn compJacobian(
    mut comp: metamodelica::Ref<StrongComponent::NBStrongComponent>,
    mut full: Option<metamodelica::Ref<Adjacency::Matrix::Matrix>>,
    mut funcMap: metamodelica::Ref<
        UnorderedMap::UnorderedMap<metamodelica::Ref<Path>, metamodelica::Ref<Function::Function>>,
    >,
    mut kind: Partition::Kind,
) -> Result<(metamodelica::Ref<StrongComponent::NBStrongComponent>, bool)> {
    let mut comp: metamodelica::Ref<StrongComponent::NBStrongComponent> = comp;
    let mut updated: bool;
    let mut strict: metamodelica::Ref<Tearing::NBTearing>;
    let mut residual_comps: metamodelica::List<metamodelica::Ref<StrongComponent::NBStrongComponent>>;
    let mut seed_candidates: metamodelica::List<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>>;
    let mut residual_vars: metamodelica::List<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>>;
    let mut inner_vars: metamodelica::List<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>>;
    let staticAsContinuous: bool = Partition::kindIsInitial(kind);
    (comp, updated) = (match &*comp {
        StrongComponent::ALGEBRAIC_LOOP { strict, .. }
            if (!(List::any(
                &({
                    let mut __acc: metamodelica::List<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>> =
                        metamodelica::nil();
                    for mut v in (strict.iteration_vars.clone()).into_iter().cloned() {
                        let __x = Slice::getT(v.clone());
                        __acc = cons(__x, __acc);
                    }
                    __acc.reverse()
                }),
                &({
                    let __pe_b1 = staticAsContinuous;
                    move |__pe_a0| NBVariable::isContinuous(__pe_a0, __pe_b1.clone())
                }),
            )?)) =>
        {
            (comp, false)
        }
        StrongComponent::ALGEBRAIC_LOOP {
            strict: __esc_strict, ..
        } => {
            strict = (*__esc_strict).clone();
            residual_comps = ({
                let mut __acc: metamodelica::List<metamodelica::Ref<StrongComponent::NBStrongComponent>> =
                    metamodelica::nil();
                for mut eqn in (strict.residual_eqns.clone()).into_iter().cloned() {
                    let __x = StrongComponent::fromSolvedEquationSlice(eqn.clone())?;
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            });
            seed_candidates = partialSliceSeedCandidates(&strict.iteration_vars, &strict.residual_eqns)?;
            residual_vars = ({
                let mut __acc: metamodelica::List<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>> =
                    metamodelica::nil();
                for mut eqn in (strict.residual_eqns.clone()).into_iter().cloned() {
                    let __x = NBEquation::Equation::getResidualVar(Slice::getT(eqn.clone()))?;
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            });
            inner_vars = ({
                let mut __acc: metamodelica::List<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>> =
                    metamodelica::nil();
                for mut comp in (strict.innerEquations.clone()).borrow().iter() {
                    let __x = ({
                        let mut __acc: metamodelica::List<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>> =
                            metamodelica::nil();
                        for mut var in (StrongComponent::getVariables(comp.clone())?).into_iter().cloned() {
                            if !(NBVariable::isContinuous(var.clone(), staticAsContinuous)?) {
                                continue;
                            }
                            let __x = var.clone();
                            __acc = cons(__x, __acc);
                        }
                        __acc.reverse()
                    });
                    __acc = __x.append(&__acc);
                }
                __acc
            });
            assign_field!(
                strict.jac = nonlinear(
                    NBVariable::VariablePointers::fromList(&seed_candidates, false)?,
                    NBVariable::VariablePointers::fromList(&(listAppend(residual_vars, inner_vars)), false)?,
                    NBEquation::EquationPointers::fromList(
                        &({
                            let mut __acc: metamodelica::List<Pointer::Pointer<metamodelica::Ref<Equation::Equation>>> =
                                metamodelica::nil();
                            for mut eqn in (strict.residual_eqns.clone()).into_iter().cloned() {
                                let __x = Slice::getT(eqn.clone());
                                __acc = cons(__x, __acc);
                            }
                            __acc.reverse()
                        })
                    )?,
                    Array::appendList(strict.innerEquations.clone(), residual_comps)?,
                    full,
                    funcMap,
                    {
                        let mut __mm_s = String::new();
                        __mm_s.push_str(&*Partition::Partition::kindToString(kind)?);
                        __mm_s.push_str(&*if (var_field!(
                            (*comp).implicitlyCreated,
                            StrongComponent::NBStrongComponent::ALGEBRAIC_LOOP
                        )
                        .clone())
                        {
                            literal!("_NLS_IMPL_")
                        } else {
                            if (var_field!((*comp).linear, StrongComponent::NBStrongComponent::ALGEBRAIC_LOOP).clone())
                            {
                                literal!("_LS_JAC_")
                            } else {
                                literal!("_NLS_JAC_")
                            }
                        });
                        __mm_s.push_str(&*intString(
                            var_field!((*comp).idx, StrongComponent::NBStrongComponent::ALGEBRAIC_LOOP).clone(),
                        ));
                        ArcStr::from(__mm_s)
                    },
                    staticAsContinuous
                )?
            );
            assign_variant_field!(comp => StrongComponent::NBStrongComponent::ALGEBRAIC_LOOP; strict = strict.clone());
            if Flags::isSet(Flags::JAC_DUMP.clone())? {
                metamodelica::print({
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*StrongComponent::toString(&comp, -1)?);
                    __mm_s.push_str(&*literal!("\n"));
                    ArcStr::from(__mm_s)
                });
            }
            (comp, true)
        }
        _ => (comp, false),
    });
    Ok((comp, updated))
}

fn jacobianSymbolic(
    mut name: ArcStr,
    mut jacType: JacobianType,
    mut seedCandidates: metamodelica::Ref<VariablePointers::VariablePointers>,
    mut partialCandidates: metamodelica::Ref<VariablePointers::VariablePointers>,
    mut equations: metamodelica::Ref<EquationPointers::EquationPointers>,
    mut strongComponents: Option<metamodelica::Array<metamodelica::Ref<StrongComponent::NBStrongComponent>>>,
    mut full: Option<metamodelica::Ref<Adjacency::Matrix::Matrix>>,
    mut funcMap: metamodelica::Ref<
        UnorderedMap::UnorderedMap<metamodelica::Ref<Path>, metamodelica::Ref<Function::Function>>,
    >,
    mut staticAsContinuous: bool,
) -> Result<Option<metamodelica::Ref<Jacobian::NBackendDAE>>> {
    let mut jacobian: Option<metamodelica::Ref<Jacobian::NBackendDAE>>;
    let mut comps: metamodelica::List<metamodelica::Ref<StrongComponent::NBStrongComponent>>;
    let mut diffed_comps: metamodelica::List<metamodelica::Ref<StrongComponent::NBStrongComponent>>;
    let mut seed_vars_ptr: Pointer::Pointer<
        metamodelica::List<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>>,
    > = Pointer::create(metamodelica::nil());
    let mut pDer_vars_ptr: Pointer::Pointer<
        metamodelica::List<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>>,
    > = Pointer::create(metamodelica::nil());
    let mut diff_map: metamodelica::Ref<
        UnorderedMap::UnorderedMap<
            metamodelica::Ref<ComponentRef::NFComponentRef>,
            metamodelica::Ref<ComponentRef::NFComponentRef>,
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
    let mut seed_diff_map: metamodelica::Ref<
        UnorderedMap::UnorderedMap<
            metamodelica::Ref<ComponentRef::NFComponentRef>,
            metamodelica::Ref<ComponentRef::NFComponentRef>,
        >,
    >;
    let mut diffArguments: metamodelica::Ref<DifferentiationArguments::DifferentiationArguments>;
    let mut idx: Pointer::Pointer<i32> = Pointer::create(0);
    let mut adjacencyVars: metamodelica::Ref<VariablePointers::VariablePointers>;
    let mut all_vars: metamodelica::List<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>>;
    let mut unknown_vars: metamodelica::List<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>>;
    let mut aux_vars: metamodelica::List<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>>;
    let mut alias_vars: metamodelica::List<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>>;
    let mut depend_vars: metamodelica::List<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>>;
    let mut res_vars: metamodelica::List<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>>;
    let mut res_vars_d: metamodelica::List<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>>;
    let mut tmp_vars: metamodelica::List<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>>;
    let mut tmp_vars_d: metamodelica::List<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>>;
    let mut seed_vars: metamodelica::List<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>>;
    let mut seed_vars_d: metamodelica::List<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>>;
    let mut varDataJac: metamodelica::Ref<VarData::VarData>;
    let mut fullLocal: metamodelica::Ref<Adjacency::Matrix::Matrix>;
    let mut sparsity: metamodelica::Ref<Adjacency::Matrix::Matrix>;
    let mut seed_set: metamodelica::Ref<UnorderedSet::UnorderedSet<metamodelica::Ref<ComponentRef::NFComponentRef>>> =
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
    let mut pder_set: metamodelica::Ref<UnorderedSet::UnorderedSet<metamodelica::Ref<ComponentRef::NFComponentRef>>> =
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
    let mut adj_base_seen: metamodelica::Ref<
        UnorderedSet::UnorderedSet<metamodelica::Ref<ComponentRef::NFComponentRef>>,
    >;
    let mut adj_seed_list: metamodelica::List<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>>;
    let mut adj_base_cref: metamodelica::Ref<ComponentRef::NFComponentRef>;
    let mut func: NBVariable::checkVar = getTmpFilterFunction(jacType)?;
    if (strongComponents).is_some() {
        comps = ({
            let mut __acc: metamodelica::List<metamodelica::Ref<StrongComponent::NBStrongComponent>> =
                metamodelica::nil();
            for mut comp in (strongComponents.ok_or("pattern mismatch")?).borrow().iter() {
                if !(!(StrongComponent::isDiscrete(&(comp.clone()))?)) {
                    continue;
                }
                let __x = comp.clone();
                __acc = cons(__x, __acc);
            }
            __acc.reverse()
        });
    } else {
        Error::addMessage(
            Error::INTERNAL_ERROR.clone(),
            list![{
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("NBJacobian.jacobianSymbolic"));
                __mm_s.push_str(&*literal!(" failed because no strong components were given!"));
                ArcStr::from(__mm_s)
            }],
        )?;
        return Err("fail");
    }
    NBVariable::VariablePointers::mapPtr(
        seedCandidates.clone(),
        &({
            let __pe_b1 = name.clone();
            let __pe_b2 = seed_vars_ptr.clone();
            let __pe_b3 = diff_map.clone();
            let __pe_b4: Arc<
                dyn ::std::ops::Fn(
                        metamodelica::Ref<ComponentRef::NFComponentRef>,
                        ArcStr,
                    ) -> Result<(
                        metamodelica::Ref<ComponentRef::NFComponentRef>,
                        Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>,
                    )> + 'static,
            > = (std::sync::Arc::new(
                move |__a0: metamodelica::Ref<ComponentRef::NFComponentRef>, __a1: ArcStr| {
                    NBVariable::makeSeedVar(__a0, &__a1)
                },
            )
                as std::sync::Arc<
                    dyn ::std::ops::Fn(
                            metamodelica::Ref<ComponentRef::NFComponentRef>,
                            ArcStr,
                        ) -> Result<(
                            metamodelica::Ref<ComponentRef::NFComponentRef>,
                            Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>,
                        )> + 'static,
                >);
            let __pe_b5 = staticAsContinuous;
            move |__pe_a0| {
                makeVarTraverse(
                    __pe_a0,
                    __pe_b1.clone(),
                    __pe_b2.clone(),
                    __pe_b3.clone(),
                    &*__pe_b4,
                    __pe_b5.clone(),
                )
            }
        }),
    )?;
    for mut v in &*NBVariable::VariablePointers::toList(&seedCandidates)? {
        if NBVariable::isContinuous(v.clone(), staticAsContinuous)? {
            UnorderedSet::add(NBVariable::getVarName(v.clone()), seed_set.clone())?;
            UnorderedSet::add(
                ComponentRef::stripSubscriptsAll(&(NBVariable::getVarName(v.clone()))),
                seed_set.clone(),
            )?;
        }
    }
    (res_vars, tmp_vars) = List::splitOnTrue(
        &(NBVariable::VariablePointers::toList(&partialCandidates)?),
        &*(func.clone()),
    )?;
    (tmp_vars, _) = List::splitOnTrue(
        &tmp_vars,
        &({
            let __pe_b1 = staticAsContinuous;
            move |__pe_a0| NBVariable::isContinuous(__pe_a0, __pe_b1.clone())
        }),
    )?;
    for mut v in &*res_vars {
        makeVarTraverse(
            v.clone(),
            name.clone(),
            pDer_vars_ptr.clone(),
            diff_map.clone(),
            &({
                let __pe_b2 = false;
                move |__pe_a0, __pe_a1| NBVariable::makePDerVar(__pe_a0, &__pe_a1, __pe_b2.clone())
            }),
            staticAsContinuous,
        )?;
    }
    for mut v in &*res_vars {
        UnorderedSet::add(NBVariable::getVarName(v.clone()), pder_set.clone())?;
    }
    res_vars_d = Pointer::access(pDer_vars_ptr).reverse();
    pDer_vars_ptr = Pointer::create(metamodelica::nil());
    seed_diff_map = UnorderedMap::copy(diff_map.clone());
    for mut v in &*tmp_vars {
        makeVarTraverse(
            v.clone(),
            name.clone(),
            pDer_vars_ptr.clone(),
            diff_map.clone(),
            &({
                let __pe_b2 = true;
                move |__pe_a0, __pe_a1| NBVariable::makePDerVar(__pe_a0, &__pe_a1, __pe_b2.clone())
            }),
            staticAsContinuous,
        )?;
    }
    tmp_vars_d = Pointer::access(pDer_vars_ptr);
    diffArguments = metamodelica::Ref::new(DifferentiationArguments::DifferentiationArguments {
        diffCref: openmodelica_nf_frontend::NFComponentRef::interned_EMPTY(),
        new_vars: metamodelica::nil(),
        diff_map: Some(diff_map),
        diffType: Differentiate::DifferentiationType::JACOBIAN.clone(),
        funcMap: funcMap,
        scalarized: seedCandidates.scalarized.clone(),
        adjoint_map: None,
        current_grad: metamodelica::Ref::new(Expression::NFExpression::EMPTY {
            ty: openmodelica_nf_frontend::NFType::interned_REAL(),
        }),
        collectAdjoints: false,
    });
    (diffed_comps, diffArguments) = Differentiate::differentiateStrongComponentList(
        comps.clone(),
        diffArguments,
        idx,
        &name,
        &(literal!("NBJacobian.jacobianSymbolic")),
    )?;
    unknown_vars = listAppend(res_vars_d.clone(), tmp_vars_d.clone());
    all_vars = unknown_vars.clone();
    seed_vars_d = Pointer::access(seed_vars_ptr).reverse();
    aux_vars = seed_vars_d.clone();
    alias_vars = metamodelica::nil();
    depend_vars = metamodelica::nil();
    varDataJac = metamodelica::Ref::new(VarData::VarData::VAR_DATA_JAC {
        variables: NBVariable::VariablePointers::fromList(&all_vars, false)?,
        unknowns: NBVariable::VariablePointers::fromList(&unknown_vars, false)?,
        auxiliaries: NBVariable::VariablePointers::fromList(&aux_vars, false)?,
        aliasVars: NBVariable::VariablePointers::fromList(&alias_vars, false)?,
        diffVars: partialCandidates,
        dependencies: NBVariable::VariablePointers::fromList(&depend_vars, false)?,
        resultVars: NBVariable::VariablePointers::fromList(&res_vars_d, false)?,
        tmpVars: NBVariable::VariablePointers::fromList(&tmp_vars_d, false)?,
        seedVars: NBVariable::VariablePointers::fromList(&seed_vars_d, false)?,
    });
    adj_base_seen = UnorderedSet::new(
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
    adj_seed_list = metamodelica::nil();
    for mut v in &*NBVariable::VariablePointers::toList(&seedCandidates)? {
        adj_base_cref = ComponentRef::stripSubscriptsAll(&(NBVariable::getVarName(v.clone())));
        if !(UnorderedSet::contains(adj_base_cref.clone(), adj_base_seen.clone())?) {
            UnorderedSet::add(adj_base_cref, adj_base_seen.clone())?;
            adj_seed_list = metamodelica::cons(
                NBVariable::getVarPointer(
                    &(NBVariable::getVarName(v.clone())),
                    metamodelica::sourceInfo!("NBackEnd/Modules/3_Post/NBJacobian.mo"),
                )?,
                adj_seed_list,
            );
        }
    }
    adjacencyVars = NBVariable::VariablePointers::fromList(&(adj_seed_list.reverse()), false)?;
    adjacencyVars = NBVariable::VariablePointers::addList(&tmp_vars, adjacencyVars)?;
    if jacType == JacobianType::ODE.clone() {
        adjacencyVars = NBVariable::VariablePointers::addList(&res_vars, adjacencyVars)?;
    }
    fullLocal = Adjacency::Matrix::createFull(
        &adjacencyVars,
        &(NBEquation::EquationPointers::fromList(
            &(List::flatten(
                ({
                    let mut __acc: metamodelica::List<
                        metamodelica::List<Pointer::Pointer<metamodelica::Ref<Equation::Equation>>>,
                    > = metamodelica::nil();
                    for mut comp in (comps.clone()).into_iter().cloned() {
                        let __x = StrongComponent::getEquations(comp.clone())?;
                        __acc = cons(__x, __acc);
                    }
                    __acc.reverse()
                }),
            )?),
        )?),
        Partition::Kind::ODE.clone(),
    )?;
    sparsity = Adjacency::Matrix::fullToSparsity(&fullLocal, &comps, seed_set, pder_set, seed_diff_map, false)?;
    jacobian = Some(metamodelica::Ref::new(Jacobian::NBackendDAE::JACOBIAN {
        name: name,
        jacType: jacType,
        varData: varDataJac,
        comps: metamodelica::arrayFromVec(diffed_comps.into_iter().cloned().collect()),
        sparsity: sparsity,
        isAdjoint: false,
    }));
    Ok(jacobian)
}

fn sizeClassificationFromType(mut ty: metamodelica::Ref<Type::NFType>) -> SizeClassification {
    let mut sc: SizeClassification;
    sc = (match Type::dimensionCount(ty) {
        0 => SizeClassification::SCALAR.clone(),
        1 => SizeClassification::ELEMENT_WISE.clone(),
        2 => SizeClassification::MATRIX.clone(),
        _ => SizeClassification::ELEMENT_WISE.clone(),
    });
    sc
}

// Helper: build addition (or single term) expression from a list of terms for a given LHS cref.
fn buildAdjointRhs(
    mut lhsCref: &metamodelica::Ref<ComponentRef::NFComponentRef>,
    mut terms: metamodelica::List<metamodelica::Ref<Expression::NFExpression>>,
) -> Result<metamodelica::Ref<Expression::NFExpression>> {
    let mut rhs: metamodelica::Ref<Expression::NFExpression>;
    let mut vty: metamodelica::Ref<Type::NFType>;
    let mut sc: SizeClassification;
    let mut addOp: metamodelica::Ref<Operator::NFOperator>;
    vty = ComponentRef::getComponentType(lhsCref);
    if (terms).is_empty() {
        rhs = Expression::makeZero(&vty)?;
        return Ok(rhs);
    }
    if List::hasOneElement(&terms) {
        rhs = (terms).head().cloned()?;
        return Ok(rhs);
    }
    sc = sizeClassificationFromType(vty.clone());
    addOp = Operator::fromClassification((MathClassification::ADDITION.clone(), sc), vty)?;
    rhs = SimplifyExp::simplify(
        metamodelica::Ref::new(Expression::NFExpression::MULTARY {
            arguments: terms,
            inv_arguments: metamodelica::nil(),
            operator: addOp,
        }),
        false,
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
    Ok(rhs)
}

// Helper: run reverse-mode on a residual expression with a given seed (current_grad),
// accumulating into the provided adjoint_map. Returns updated DifferentiationArguments.
fn accumulateAdjointForResidual(
    mut residual: metamodelica::Ref<Expression::NFExpression>,
    mut seed: metamodelica::Ref<Expression::NFExpression>,
    mut diff_map: metamodelica::Ref<
        UnorderedMap::UnorderedMap<
            metamodelica::Ref<ComponentRef::NFComponentRef>,
            metamodelica::Ref<ComponentRef::NFComponentRef>,
        >,
    >,
    mut funcMapIn: metamodelica::Ref<
        UnorderedMap::UnorderedMap<metamodelica::Ref<Path>, metamodelica::Ref<Function::Function>>,
    >,
    mut scalarized: bool,
    mut adjoint_map_in: metamodelica::Ref<
        UnorderedMap::UnorderedMap<
            metamodelica::Ref<ComponentRef::NFComponentRef>,
            metamodelica::List<metamodelica::Ref<Expression::NFExpression>>,
        >,
    >,
) -> Result<metamodelica::Ref<DifferentiationArguments::DifferentiationArguments>> {
    let mut diffArguments: metamodelica::Ref<DifferentiationArguments::DifferentiationArguments>;
    diffArguments = metamodelica::Ref::new(DifferentiationArguments::DifferentiationArguments {
        diffCref: openmodelica_nf_frontend::NFComponentRef::interned_EMPTY(),
        new_vars: metamodelica::nil(),
        diff_map: Some(diff_map),
        diffType: Differentiate::DifferentiationType::JACOBIAN.clone(),
        funcMap: funcMapIn,
        scalarized: scalarized,
        adjoint_map: Some(adjoint_map_in),
        current_grad: seed,
        collectAdjoints: true,
    });
    (_, diffArguments) = Differentiate::differentiateExpression(residual, diffArguments)?;
    Ok(diffArguments)
}

// Reusable builder for a SINGLE_COMPONENT adjoint assignment (tmp or result var).
fn makeAdjointComponentFromRhs(
    mut lhsKey: metamodelica::Ref<ComponentRef::NFComponentRef>,
    mut rhsExpr: metamodelica::Ref<Expression::NFExpression>,
    mut contextName: &ArcStr,
    mut eqIndex: i32,
) -> Result<metamodelica::Ref<StrongComponent::NBStrongComponent>> {
    let mut diffed_comp: metamodelica::Ref<StrongComponent::NBStrongComponent>;
    let mut eqPtr: Pointer::Pointer<metamodelica::Ref<Equation::Equation>>;
    let mut eq: metamodelica::Ref<Equation::Equation>;
    let mut lhsVarPtr: Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>;
    eqPtr = NBEquation::Equation::makeAssignment(
        Expression::fromCref(lhsKey.clone(), false)?,
        rhsExpr,
        Pointer::create(eqIndex),
        contextName,
        crate::NBEquation::Iterator::interned_EMPTY(),
        NBEquation::default(NBEquation::EquationKind::CONTINUOUS.clone(), false, None, None),
    )?;
    lhsVarPtr = NBVariable::getVarPointer(
        &lhsKey,
        metamodelica::sourceInfo!("NBackEnd/Modules/3_Post/NBJacobian.mo"),
    )?;
    eq = Pointer::access(eqPtr.clone());
    diffed_comp = (match &*eq {
        NBEquation::Equation::SCALAR_EQUATION { .. } => {
            if !((ComponentRef::subscriptsAllFlat(&lhsKey)?).is_empty()) {
                diffed_comp = metamodelica::Ref::new(StrongComponent::NBStrongComponent::SLICED_COMPONENT {
                    var_cref: lhsKey,
                    var: metamodelica::Ref::new(Slice::NBSlice {
                        t: lhsVarPtr,
                        indices: metamodelica::nil(),
                    }),
                    eqn: metamodelica::Ref::new(Slice::NBSlice {
                        t: eqPtr,
                        indices: metamodelica::nil(),
                    }),
                    status: NBSolve::Status::EXPLICIT.clone(),
                });
            } else {
                diffed_comp = metamodelica::Ref::new(StrongComponent::NBStrongComponent::SINGLE_COMPONENT {
                    var: lhsVarPtr,
                    eqn: eqPtr,
                    status: NBSolve::Status::EXPLICIT.clone(),
                });
            }
            diffed_comp
        }
        NBEquation::Equation::ARRAY_EQUATION { .. } => {
            metamodelica::Ref::new(StrongComponent::NBStrongComponent::SINGLE_COMPONENT {
                var: lhsVarPtr,
                eqn: eqPtr,
                status: NBSolve::Status::EXPLICIT.clone(),
            })
        }
        NBEquation::Equation::RECORD_EQUATION { .. } => {
            metamodelica::Ref::new(StrongComponent::NBStrongComponent::SINGLE_COMPONENT {
                var: lhsVarPtr,
                eqn: eqPtr,
                status: NBSolve::Status::EXPLICIT.clone(),
            })
        }
        _ => {
            Error::addMessage(
                Error::INTERNAL_ERROR.clone(),
                list![{
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!("NBJacobian.makeAdjointComponentFromRhs"));
                    __mm_s.push_str(&*literal!(" cannot create adjoint strong component for equation "));
                    __mm_s.push_str(&*NBEquation::Equation::toString(eq, literal!(""))?);
                    ArcStr::from(__mm_s)
                }],
            )?;
            return Err("fail");
        }
    });
    Ok(diffed_comp)
}

fn addEntryToLPAMap(
    mut vptr: Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>,
    mut diff_map: metamodelica::Ref<
        UnorderedMap::UnorderedMap<
            metamodelica::Ref<ComponentRef::NFComponentRef>,
            metamodelica::Ref<ComponentRef::NFComponentRef>,
        >,
    >,
    mut loop_product_adjoint_map: metamodelica::Ref<
        UnorderedMap::UnorderedMap<
            metamodelica::Ref<ComponentRef::NFComponentRef>,
            metamodelica::List<metamodelica::Ref<Expression::NFExpression>>,
        >,
    >,
) -> Result<()> {
    let mut mappedSeed: Option<metamodelica::Ref<ComponentRef::NFComponentRef>>;
    mappedSeed = UnorderedMap::get(NBVariable::getVarName(vptr), diff_map)?;
    if (mappedSeed).is_some() {
        UnorderedMap::tryAdd(
            mappedSeed.ok_or("pattern mismatch")?,
            metamodelica::nil(),
            loop_product_adjoint_map,
        )?;
    }
    Ok(())
}

// Resolve base variables that were actually mapped to tmp pDER vars.
// This avoids relying on splitOnTrue output ordering semantics.
fn getBaseTmpVarCandidates(
    mut partialVars: &metamodelica::List<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>>,
    mut tmpPDerVars: &metamodelica::List<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>>,
    mut diff_map: metamodelica::Ref<
        UnorderedMap::UnorderedMap<
            metamodelica::Ref<ComponentRef::NFComponentRef>,
            metamodelica::Ref<ComponentRef::NFComponentRef>,
        >,
    >,
) -> Result<metamodelica::List<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>>> {
    let mut baseTmpVars: metamodelica::List<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>> =
        metamodelica::nil();
    let mut tmpPDerSet: metamodelica::Ref<UnorderedSet::UnorderedSet<metamodelica::Ref<ComponentRef::NFComponentRef>>>;
    let mut baseCref: metamodelica::Ref<ComponentRef::NFComponentRef>;
    let mut o_mapped: Option<metamodelica::Ref<ComponentRef::NFComponentRef>>;
    tmpPDerSet = UnorderedSet::new(
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
        Util::nextPrime(((tmpPDerVars).len() as i32)),
    );
    for mut v in &**tmpPDerVars {
        UnorderedSet::add(NBVariable::getVarName(v.clone()), tmpPDerSet.clone())?;
    }
    for mut v in &**partialVars {
        baseCref = NBVariable::getVarName(v.clone());
        o_mapped = UnorderedMap::get(baseCref, diff_map.clone())?;
        if (o_mapped).is_some() && UnorderedSet::contains(o_mapped.ok_or("pattern mismatch")?, tmpPDerSet.clone())? {
            baseTmpVars = metamodelica::cons(v.clone(), baseTmpVars);
        }
    }
    baseTmpVars = baseTmpVars.reverse();
    Ok(baseTmpVars)
}

// Build a filtered diff map for a given variable list.
// For each variable pointer v in 'vars', if there exists a mapping
//   base = BVariable.getVarName(v) -> mapped in 'globalDiffMap'
// then add (base -> mapped) to the returned map.
fn populateDiffMap(
    mut vars: &metamodelica::List<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>>,
    mut globalDiffMap: metamodelica::Ref<
        UnorderedMap::UnorderedMap<
            metamodelica::Ref<ComponentRef::NFComponentRef>,
            metamodelica::Ref<ComponentRef::NFComponentRef>,
        >,
    >,
) -> Result<
    metamodelica::Ref<
        UnorderedMap::UnorderedMap<
            metamodelica::Ref<ComponentRef::NFComponentRef>,
            metamodelica::Ref<ComponentRef::NFComponentRef>,
        >,
    >,
> {
    let mut outMap: metamodelica::Ref<
        UnorderedMap::UnorderedMap<
            metamodelica::Ref<ComponentRef::NFComponentRef>,
            metamodelica::Ref<ComponentRef::NFComponentRef>,
        >,
    >;
    let mut baseCref: metamodelica::Ref<ComponentRef::NFComponentRef>;
    let mut o_mappedCref: Option<metamodelica::Ref<ComponentRef::NFComponentRef>>;
    outMap = UnorderedMap::new(
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
        Util::nextPrime(((vars).len() as i32)),
    );
    for mut vp in &**vars {
        baseCref = NBVariable::getVarName(vp.clone());
        o_mappedCref = UnorderedMap::get(baseCref.clone(), globalDiffMap.clone())?;
        if (o_mappedCref).is_some() {
            UnorderedMap::add(baseCref, o_mappedCref.ok_or("pattern mismatch")?, outMap.clone())?;
        }
    }
    Ok(outMap)
}

fn isSupportedAdjointStrongComponent<'__b>(
    mut comp: &'__b metamodelica::Ref<StrongComponent::NBStrongComponent>,
) -> bool {
    '__tco: loop {
        match &**comp {
            StrongComponent::SINGLE_COMPONENT { .. } => return true,
            StrongComponent::MULTI_COMPONENT { .. } => return true,
            StrongComponent::SLICED_COMPONENT { .. } => return true,
            StrongComponent::RESIZABLE_COMPONENT { .. } => return true,
            StrongComponent::ALGEBRAIC_LOOP { .. } => return true,
            StrongComponent::ALIAS { .. } => {
                comp = var_field!((**comp).original, StrongComponent::NBStrongComponent::ALIAS);
                continue '__tco;
            }
            _ => return false,
        }
    }
}

pub type AdjointTermList = metamodelica::List<metamodelica::Ref<Expression::NFExpression>>;

fn generateAdjointComponent(
    mut comp: metamodelica::Ref<StrongComponent::NBStrongComponent>,
    mut diff_map: metamodelica::Ref<
        UnorderedMap::UnorderedMap<
            metamodelica::Ref<ComponentRef::NFComponentRef>,
            metamodelica::Ref<ComponentRef::NFComponentRef>,
        >,
    >,
    mut funcMap: metamodelica::Ref<
        UnorderedMap::UnorderedMap<metamodelica::Ref<Path>, metamodelica::Ref<Function::Function>>,
    >,
    mut scalarized: bool,
    mut staticAsContinuous: bool,
    mut idx: Pointer::Pointer<i32>,
    mut contextName: ArcStr,
    mut seedCandidates: &metamodelica::Ref<VariablePointers::VariablePointers>,
    mut tmpVarCandidates: metamodelica::List<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>>,
) -> Result<(
    metamodelica::List<metamodelica::Ref<StrongComponent::NBStrongComponent>>,
    metamodelica::List<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>>,
)> {
    let mut adjointComps: metamodelica::List<metamodelica::Ref<StrongComponent::NBStrongComponent>> =
        metamodelica::nil();
    let mut newTmpVars: metamodelica::List<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>> =
        metamodelica::nil();
    let mut c_noalias: metamodelica::Ref<StrongComponent::NBStrongComponent>;
    let mut fresh_adjoint_map: metamodelica::Ref<
        UnorderedMap::UnorderedMap<
            metamodelica::Ref<ComponentRef::NFComponentRef>,
            metamodelica::List<metamodelica::Ref<Expression::NFExpression>>,
        >,
    >;
    let mut diffArgs: metamodelica::Ref<DifferentiationArguments::DifferentiationArguments>;
    let mut eq: metamodelica::Ref<Equation::Equation>;
    let mut adjStmts: metamodelica::List<metamodelica::Ref<Statement::NFStatement>>;
    let mut eqPtr: Pointer::Pointer<metamodelica::Ref<Equation::Equation>>;
    let mut adjVarSlices: metamodelica::List<
        metamodelica::Ref<Slice::NBSlice<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>>>,
    >;
    let mut ssaPDerVarsPtr: Pointer::Pointer<
        metamodelica::List<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>>,
    > = Pointer::create(metamodelica::nil());
    c_noalias = StrongComponent::removeAlias(comp);
    let () = ({
        let mut replacements: metamodelica::List<(
            metamodelica::Ref<ComponentRef::NFComponentRef>,
            (metamodelica::Ref<ComponentRef::NFComponentRef>, i32),
        )> = metamodelica::nil();
        let mut newVars: metamodelica::List<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>> =
            metamodelica::nil();
        let mut init: bool = false;
        (match &*c_noalias.clone() {
            StrongComponent::ALGEBRAIC_LOOP { strict: tearing, .. } => {
                let mut itVarPtrs: metamodelica::List<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>>;
                let mut residuals: metamodelica::List<metamodelica::Ref<Expression::NFExpression>>;
                let mut lambdaPtrs: metamodelica::List<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>>;
                let mut lambdaCrefs: metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>>;
                let mut iRes: i32;
                let mut lhsVarPtr: Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>;
                let mut newC: metamodelica::Ref<ComponentRef::NFComponentRef>;
                let mut diff_map_y: metamodelica::Ref<
                    UnorderedMap::UnorderedMap<
                        metamodelica::Ref<ComponentRef::NFComponentRef>,
                        metamodelica::Ref<ComponentRef::NFComponentRef>,
                    >,
                >;
                let mut diff_map_x: metamodelica::Ref<
                    UnorderedMap::UnorderedMap<
                        metamodelica::Ref<ComponentRef::NFComponentRef>,
                        metamodelica::Ref<ComponentRef::NFComponentRef>,
                    >,
                >;
                let mut diff_map_union: metamodelica::Ref<
                    UnorderedMap::UnorderedMap<
                        metamodelica::Ref<ComponentRef::NFComponentRef>,
                        metamodelica::Ref<ComponentRef::NFComponentRef>,
                    >,
                >;
                let mut loop_product_adjoint_map: metamodelica::Ref<
                    UnorderedMap::UnorderedMap<
                        metamodelica::Ref<ComponentRef::NFComponentRef>,
                        metamodelica::List<metamodelica::Ref<Expression::NFExpression>>,
                    >,
                >;
                let mut seedPtrListX: metamodelica::List<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>>;
                let mut linResEqnPtrs: metamodelica::List<Pointer::Pointer<metamodelica::Ref<Equation::Equation>>>;
                let mut terms_j: AdjointTermList;
                let mut terms_x: AdjointTermList;
                let mut lhs_j: metamodelica::Ref<Expression::NFExpression>;
                let mut rhs_j: metamodelica::Ref<Expression::NFExpression>;
                let mut rhs_x: metamodelica::Ref<Expression::NFExpression>;
                let mut resid_j: Pointer::Pointer<metamodelica::Ref<Equation::Equation>>;
                let mut o_ySeedCref: Option<metamodelica::Ref<ComponentRef::NFComponentRef>>;
                let mut o_pDerX: Option<metamodelica::Ref<ComponentRef::NFComponentRef>>;
                let mut ySeedCref: metamodelica::Ref<ComponentRef::NFComponentRef>;
                let mut baseX: metamodelica::Ref<ComponentRef::NFComponentRef>;
                let mut pDerX: metamodelica::Ref<ComponentRef::NFComponentRef>;
                let mut loopComp: metamodelica::Ref<StrongComponent::NBStrongComponent>;
                let mut vty: metamodelica::Ref<Type::NFType>;
                let mut xbarStmts: metamodelica::List<metamodelica::Ref<Statement::NFStatement>>;
                let mut sc_x: SizeClassification;
                let mut addOp_x: metamodelica::Ref<Operator::NFOperator>;
                let mut accRhs: metamodelica::Ref<Expression::NFExpression>;
                itVarPtrs = Tearing::getIterationVars(metamodelica::AsArg::as_arg(&tearing));
                residuals = ({
                    let mut __acc: metamodelica::List<metamodelica::Ref<Expression::NFExpression>> =
                        metamodelica::nil();
                    for mut e in (Tearing::getResidualEqns(metamodelica::AsArg::as_arg(&tearing)))
                        .into_iter()
                        .cloned()
                    {
                        let __x = NBEquation::Equation::getResidualExp(&(Pointer::access(e.clone())), true)?;
                        __acc = cons(__x, __acc);
                    }
                    __acc.reverse()
                });
                lambdaPtrs = metamodelica::nil();
                lambdaCrefs = metamodelica::nil();
                for mut iIdx in 1..=((residuals).len() as i32) {
                    (lhsVarPtr, newC) = NBVariable::makeAuxVar(
                        &(arcstr::literal!(NBVariable::TEMPORARY_STR)),
                        Pointer::access(idx.clone()) + 1,
                        openmodelica_nf_frontend::NFType::interned_REAL(),
                        false,
                    )?;
                    Pointer::update(idx.clone(), Pointer::access(idx.clone()) + 1);
                    (newC, lhsVarPtr) = NBVariable::makePDerVar(newC, &contextName, true)?;
                    lambdaPtrs = metamodelica::cons(lhsVarPtr, lambdaPtrs);
                    lambdaCrefs = metamodelica::cons(newC, lambdaCrefs);
                }
                lambdaPtrs = lambdaPtrs.reverse();
                lambdaCrefs = lambdaCrefs.reverse();
                newTmpVars = lambdaPtrs.clone();
                diff_map_y = populateDiffMap(&itVarPtrs, diff_map.clone())?;
                seedPtrListX = listAppend(NBVariable::VariablePointers::toList(seedCandidates)?, tmpVarCandidates);
                seedPtrListX = ({
                    let mut __acc: metamodelica::List<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>> =
                        metamodelica::nil();
                    for mut vp in (seedPtrListX).into_iter().cloned() {
                        if !(!(UnorderedMap::contains(NBVariable::getVarName(vp.clone()), diff_map_y.clone())?)) {
                            continue;
                        }
                        let __x = vp.clone();
                        __acc = cons(__x, __acc);
                    }
                    __acc.reverse()
                });
                diff_map_x = populateDiffMap(&seedPtrListX, diff_map)?;
                diff_map_union = UnorderedMap::merge(
                    diff_map_y.clone(),
                    diff_map_x.clone(),
                    metamodelica::sourceInfo!("NBackEnd/Modules/3_Post/NBJacobian.mo"),
                )?;
                loop_product_adjoint_map = UnorderedMap::new(
                    (std::sync::Arc::new(move |__a0: metamodelica::Ref<ComponentRef::NFComponentRef>| {
                        ComponentRef::hash(&__a0)
                    })
                        as std::sync::Arc<
                            dyn ::std::ops::Fn(metamodelica::Ref<ComponentRef::NFComponentRef>) -> Result<i32>
                                + 'static,
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
                    ((itVarPtrs).len() as i32) + ((seedPtrListX).len() as i32),
                );
                for mut vp in &*itVarPtrs {
                    addEntryToLPAMap(vp.clone(), diff_map_y.clone(), loop_product_adjoint_map.clone())?;
                }
                for mut vp in &*seedPtrListX {
                    addEntryToLPAMap(vp.clone(), diff_map_x.clone(), loop_product_adjoint_map.clone())?;
                }
                iRes = 1;
                for mut residual_i in &*residuals {
                    if iRes > ((lambdaCrefs).len() as i32) {
                        break;
                    }
                    diffArgs = accumulateAdjointForResidual(
                        residual_i.clone(),
                        Expression::fromCref((lambdaCrefs).get(iRes)?, false)?,
                        diff_map_union.clone(),
                        funcMap.clone(),
                        scalarized,
                        loop_product_adjoint_map,
                    )?;
                    loop_product_adjoint_map = diffArgs.adjoint_map.clone().ok_or("pattern mismatch")?;
                    iRes = iRes + 1;
                }
                linResEqnPtrs = metamodelica::nil();
                for mut vp in &*itVarPtrs {
                    o_ySeedCref = UnorderedMap::get(NBVariable::getVarName(vp.clone()), diff_map_y.clone())?;
                    if (o_ySeedCref).is_some() {
                        ySeedCref = o_ySeedCref.ok_or("pattern mismatch")?;
                        terms_j = UnorderedMap::getOrDefault(
                            ySeedCref.clone(),
                            loop_product_adjoint_map.clone(),
                            metamodelica::nil(),
                        )?;
                        lhs_j = buildAdjointRhs(&ySeedCref, terms_j)?;
                        rhs_j = Expression::fromCref(ySeedCref, false)?;
                        resid_j = NBEquation::Equation::makeAssignment(
                            lhs_j,
                            rhs_j,
                            idx.clone(),
                            &contextName,
                            crate::NBEquation::Iterator::interned_EMPTY(),
                            NBEquation::default(NBEquation::EquationKind::CONTINUOUS.clone(), false, None, None),
                        )?;
                        linResEqnPtrs = metamodelica::cons(
                            NBEquation::Equation::createResidual(resid_j, None, false, false)?,
                            linResEqnPtrs,
                        );
                    }
                }
                linResEqnPtrs = linResEqnPtrs.reverse();
                if !((linResEqnPtrs).is_empty()) {
                    loopComp = makeLinearAlgebraicLoop(lambdaPtrs, linResEqnPtrs, None, false, false)?;
                    adjointComps = metamodelica::cons(loopComp, adjointComps);
                }
                xbarStmts = metamodelica::nil();
                for mut seedVarPtrX in &*seedPtrListX {
                    baseX = NBVariable::getVarName(seedVarPtrX.clone());
                    o_pDerX = UnorderedMap::get(baseX, diff_map_x.clone())?;
                    if (o_pDerX).is_some() {
                        pDerX = o_pDerX.ok_or("pattern mismatch")?;
                        terms_x = UnorderedMap::getOrDefault(
                            pDerX.clone(),
                            loop_product_adjoint_map.clone(),
                            metamodelica::nil(),
                        )?;
                        if !((terms_x).is_empty()) {
                            rhs_x = Expression::negate(buildAdjointRhs(&pDerX, terms_x)?);
                            vty = ComponentRef::getComponentType(&pDerX);
                            if Expression::containsCref(rhs_x.clone(), &pDerX)? {
                                accRhs = rhs_x;
                            } else {
                                sc_x = sizeClassificationFromType(vty.clone());
                                addOp_x = Operator::fromClassification(
                                    (MathClassification::ADDITION.clone(), sc_x),
                                    vty.clone(),
                                )?;
                                accRhs = SimplifyExp::simplify(
                                    metamodelica::Ref::new(Expression::NFExpression::MULTARY {
                                        arguments: list![Expression::fromCref(pDerX.clone(), false)?, rhs_x],
                                        inv_arguments: metamodelica::nil(),
                                        operator: addOp_x,
                                    }),
                                    false,
                                )?;
                            }
                            accRhs = Expression::map(
                                accRhs,
                                (std::sync::Arc::new(Expression::repairOperator)
                                    as std::sync::Arc<
                                        dyn ::std::ops::Fn(
                                                metamodelica::Ref<Expression::NFExpression>,
                                            )
                                                -> Result<metamodelica::Ref<Expression::NFExpression>>
                                            + 'static,
                                    >),
                            )?;
                            xbarStmts = metamodelica::cons(
                                metamodelica::Ref::new(Statement::NFStatement::ASSIGNMENT {
                                    lhs: Expression::fromCref(pDerX, false)?,
                                    rhs: accRhs,
                                    ty: vty,
                                    source: DAE::emptyElementSource().clone(),
                                }),
                                xbarStmts,
                            );
                        }
                    }
                }
                xbarStmts = xbarStmts.reverse();
                if !((xbarStmts).is_empty()) {
                    eqPtr = NBEquation::Equation::makeAlgorithm(xbarStmts.clone(), init)?;
                    NBEquation::Equation::createName(eqPtr.clone(), idx, &contextName)?;
                    adjVarSlices = collectAdjointVarSlices(&xbarStmts, metamodelica::nil())?.reverse();
                    adjointComps = metamodelica::cons(
                        metamodelica::Ref::new(StrongComponent::NBStrongComponent::MULTI_COMPONENT {
                            vars: adjVarSlices,
                            eqn: metamodelica::Ref::new(Slice::NBSlice {
                                t: eqPtr,
                                indices: metamodelica::nil(),
                            }),
                            status: NBSolve::Status::EXPLICIT.clone(),
                        }),
                        adjointComps,
                    );
                }
                ()
            }
            StrongComponent::SINGLE_COMPONENT {
                eqn: __c_noalias_eqn, ..
            } => {
                eq = Pointer::access(__c_noalias_eqn.clone());
                fresh_adjoint_map = UnorderedMap::new(
                    (std::sync::Arc::new(move |__a0: metamodelica::Ref<ComponentRef::NFComponentRef>| {
                        ComponentRef::hash(&__a0)
                    })
                        as std::sync::Arc<
                            dyn ::std::ops::Fn(metamodelica::Ref<ComponentRef::NFComponentRef>) -> Result<i32>
                                + 'static,
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
                    16,
                );
                diffArgs = metamodelica::Ref::new(DifferentiationArguments::DifferentiationArguments {
                    diffCref: openmodelica_nf_frontend::NFComponentRef::interned_EMPTY(),
                    new_vars: metamodelica::nil(),
                    diff_map: Some(diff_map),
                    diffType: DifferentiationType::JACOBIAN.clone(),
                    funcMap: funcMap,
                    scalarized: scalarized,
                    adjoint_map: Some(fresh_adjoint_map),
                    current_grad: metamodelica::Ref::new(Expression::NFExpression::EMPTY {
                        ty: openmodelica_nf_frontend::NFType::interned_REAL(),
                    }),
                    collectAdjoints: true,
                });
                (diffArgs, adjStmts) = Differentiate::differentiateEquationAdjoint(&eq, diffArgs)?;
                if !((adjStmts).is_empty()) {
                    eqPtr = NBEquation::Equation::makeAlgorithm(adjStmts.clone(), init)?;
                    NBEquation::Equation::createName(eqPtr.clone(), idx, &contextName)?;
                    adjVarSlices = collectAdjointVarSlices(&adjStmts, metamodelica::nil())?.reverse();
                    adjointComps = list![metamodelica::Ref::new(
                        StrongComponent::NBStrongComponent::MULTI_COMPONENT {
                            vars: adjVarSlices,
                            eqn: metamodelica::Ref::new(Slice::NBSlice {
                                t: eqPtr,
                                indices: metamodelica::nil()
                            }),
                            status: NBSolve::Status::EXPLICIT.clone()
                        }
                    )];
                }
                ()
            }
            StrongComponent::MULTI_COMPONENT {
                eqn: __c_noalias_eqn, ..
            } => {
                let mut ssaAlg: metamodelica::Ref<StrongComponent::NBStrongComponent>;
                let mut seenCrefs: metamodelica::Ref<
                    UnorderedSet::UnorderedSet<metamodelica::Ref<ComponentRef::NFComponentRef>>,
                >;
                let mut origCref: metamodelica::Ref<ComponentRef::NFComponentRef>;
                let mut finalSsaCref: metamodelica::Ref<ComponentRef::NFComponentRef>;
                let mut pDerOrigCref: metamodelica::Ref<ComponentRef::NFComponentRef>;
                let mut pDerSsaCref: metamodelica::Ref<ComponentRef::NFComponentRef>;
                let mut vty: metamodelica::Ref<Type::NFType>;
                eq = (match &*(Pointer::access(Slice::getT(__c_noalias_eqn.clone()))) {
                    NBEquation::Equation::ALGORITHM { .. } => {
                        (ssaAlg, replacements, newVars) = algorithmToSSA(&c_noalias)?;
                        if Flags::isSet(Flags::DEBUG_ADJOINT.clone())? {
                            metamodelica::print({
                                let mut __mm_s = String::new();
                                __mm_s.push_str(&*literal!("SSA algorithm for adjoint of component "));
                                __mm_s.push_str(&*StrongComponent::toString(&c_noalias, -1)?);
                                __mm_s.push_str(&*literal!(":\n"));
                                __mm_s.push_str(&*StrongComponent::toString(&ssaAlg, -1)?);
                                __mm_s.push_str(&*literal!("\n"));
                                ArcStr::from(__mm_s)
                            });
                        }
                        for mut ssaVarPtr in &*newVars {
                            makeVarTraverse(
                                ssaVarPtr.clone(),
                                contextName.clone(),
                                ssaPDerVarsPtr.clone(),
                                diff_map.clone(),
                                &({
                                    let __pe_b2 = true;
                                    move |__pe_a0, __pe_a1| NBVariable::makePDerVar(__pe_a0, &__pe_a1, __pe_b2.clone())
                                }),
                                staticAsContinuous,
                            )?;
                        }
                        for mut pDerVarPtr in &*Pointer::access(ssaPDerVarsPtr) {
                            newTmpVars = metamodelica::cons(pDerVarPtr.clone(), newTmpVars);
                        }
                        (match &*ssaAlg {
                            StrongComponent::MULTI_COMPONENT { eqn: __ssaAlg_eqn, .. } => {
                                Pointer::access(Slice::getT(__ssaAlg_eqn.clone()))
                            }
                            _ => Pointer::access(Slice::getT(__c_noalias_eqn.clone())),
                        })
                    }
                    _ => Pointer::access(Slice::getT(__c_noalias_eqn.clone())),
                });
                fresh_adjoint_map = UnorderedMap::new(
                    (std::sync::Arc::new(move |__a0: metamodelica::Ref<ComponentRef::NFComponentRef>| {
                        ComponentRef::hash(&__a0)
                    })
                        as std::sync::Arc<
                            dyn ::std::ops::Fn(metamodelica::Ref<ComponentRef::NFComponentRef>) -> Result<i32>
                                + 'static,
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
                    16,
                );
                diffArgs = metamodelica::Ref::new(DifferentiationArguments::DifferentiationArguments {
                    diffCref: openmodelica_nf_frontend::NFComponentRef::interned_EMPTY(),
                    new_vars: metamodelica::nil(),
                    diff_map: Some(diff_map.clone()),
                    diffType: DifferentiationType::JACOBIAN.clone(),
                    funcMap: funcMap,
                    scalarized: scalarized,
                    adjoint_map: Some(fresh_adjoint_map),
                    current_grad: metamodelica::Ref::new(Expression::NFExpression::EMPTY {
                        ty: openmodelica_nf_frontend::NFType::interned_REAL(),
                    }),
                    collectAdjoints: true,
                });
                (diffArgs, adjStmts) = Differentiate::differentiateEquationAdjoint(&eq, diffArgs)?;
                if !((newVars).is_empty()) {
                    seenCrefs = UnorderedSet::new(
                        (std::sync::Arc::new(move |__a0: metamodelica::Ref<ComponentRef::NFComponentRef>| {
                            ComponentRef::hash(&__a0)
                        })
                            as std::sync::Arc<
                                dyn ::std::ops::Fn(metamodelica::Ref<ComponentRef::NFComponentRef>) -> Result<i32>
                                    + 'static,
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
                        4,
                    );
                    for mut replacement in &*replacements.reverse() {
                        let (__pa0, (__pa1, _)) = replacement.clone();
                        origCref = metamodelica::Own::own(__pa0);
                        finalSsaCref = metamodelica::Own::own(__pa1);
                        if !(UnorderedSet::contains(origCref.clone(), seenCrefs.clone())?) {
                            UnorderedSet::add(origCref.clone(), seenCrefs.clone())?;
                            if UnorderedMap::contains(origCref.clone(), diff_map.clone())?
                                && UnorderedMap::contains(finalSsaCref.clone(), diff_map.clone())?
                            {
                                pDerOrigCref = UnorderedMap::getOrFail(origCref, diff_map.clone())?;
                                pDerSsaCref = UnorderedMap::getOrFail(finalSsaCref, diff_map.clone())?;
                                vty = ComponentRef::getSubscriptedType(&pDerSsaCref, true)?;
                                adjStmts = metamodelica::cons(
                                    metamodelica::Ref::new(Statement::NFStatement::ASSIGNMENT {
                                        lhs: Expression::fromCref(pDerSsaCref, false)?,
                                        rhs: Expression::fromCref(pDerOrigCref, false)?,
                                        ty: vty,
                                        source: DAE::emptyElementSource().clone(),
                                    }),
                                    adjStmts,
                                );
                            }
                        }
                    }
                }
                if !((adjStmts).is_empty()) {
                    eqPtr = NBEquation::Equation::makeAlgorithm(adjStmts.clone(), init)?;
                    NBEquation::Equation::createName(eqPtr.clone(), idx, &contextName)?;
                    adjVarSlices = collectAdjointVarSlices(&adjStmts, metamodelica::nil())?.reverse();
                    adjointComps = list![metamodelica::Ref::new(
                        StrongComponent::NBStrongComponent::MULTI_COMPONENT {
                            vars: adjVarSlices,
                            eqn: metamodelica::Ref::new(Slice::NBSlice {
                                t: eqPtr,
                                indices: metamodelica::nil()
                            }),
                            status: NBSolve::Status::EXPLICIT.clone()
                        }
                    )];
                }
                ()
            }
            StrongComponent::SLICED_COMPONENT {
                eqn: __c_noalias_eqn, ..
            } => {
                eq = Pointer::access(Slice::getT(__c_noalias_eqn.clone()));
                adjointComps = generateAdjointForComponent(
                    &eq,
                    &c_noalias,
                    diff_map,
                    funcMap,
                    scalarized,
                    init,
                    idx,
                    &contextName,
                )?;
                ()
            }
            StrongComponent::RESIZABLE_COMPONENT {
                eqn: __c_noalias_eqn, ..
            } => {
                eq = Pointer::access(Slice::getT(__c_noalias_eqn.clone()));
                adjointComps = generateAdjointForComponent(
                    &eq,
                    &c_noalias,
                    diff_map,
                    funcMap,
                    scalarized,
                    init,
                    idx,
                    &contextName,
                )?;
                ()
            }
            StrongComponent::GENERIC_COMPONENT {
                eqn: __c_noalias_eqn, ..
            } => {
                eq = Pointer::access(Slice::getT(__c_noalias_eqn.clone()));
                adjointComps = generateAdjointForComponent(
                    &eq,
                    &c_noalias,
                    diff_map,
                    funcMap,
                    scalarized,
                    init,
                    idx,
                    &contextName,
                )?;
                ()
            }
            _ => {
                Error::addMessage(
                    Error::INTERNAL_ERROR.clone(),
                    list![{
                        let mut __mm_s = String::new();
                        __mm_s.push_str(&*literal!("NBJacobian.generateAdjointComponent"));
                        __mm_s.push_str(&*literal!(" unsupported component type: "));
                        __mm_s.push_str(&*StrongComponent::toString(&c_noalias, -1)?);
                        ArcStr::from(__mm_s)
                    }],
                )?;
                ()
            }
        })
    });
    Ok((adjointComps, newTmpVars))
}

fn generateAdjointForComponent(
    mut eq: &metamodelica::Ref<Equation::Equation>,
    mut originalComp: &metamodelica::Ref<StrongComponent::NBStrongComponent>,
    mut diff_map: metamodelica::Ref<
        UnorderedMap::UnorderedMap<
            metamodelica::Ref<ComponentRef::NFComponentRef>,
            metamodelica::Ref<ComponentRef::NFComponentRef>,
        >,
    >,
    mut funcMap: metamodelica::Ref<
        UnorderedMap::UnorderedMap<metamodelica::Ref<Path>, metamodelica::Ref<Function::Function>>,
    >,
    mut scalarized: bool,
    mut init: bool,
    mut idx: Pointer::Pointer<i32>,
    mut contextName: &ArcStr,
) -> Result<metamodelica::List<metamodelica::Ref<StrongComponent::NBStrongComponent>>> {
    let mut adjointComps: metamodelica::List<metamodelica::Ref<StrongComponent::NBStrongComponent>> =
        metamodelica::nil();
    let mut fresh_adjoint_map: metamodelica::Ref<
        UnorderedMap::UnorderedMap<
            metamodelica::Ref<ComponentRef::NFComponentRef>,
            metamodelica::List<metamodelica::Ref<Expression::NFExpression>>,
        >,
    >;
    let mut diffArgs: metamodelica::Ref<DifferentiationArguments::DifferentiationArguments>;
    let mut adjStmts: metamodelica::List<metamodelica::Ref<Statement::NFStatement>>;
    let mut eqPtr: Pointer::Pointer<metamodelica::Ref<Equation::Equation>>;
    let mut adjVarSlices: metamodelica::List<
        metamodelica::Ref<Slice::NBSlice<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>>>,
    >;
    let mut adjVarCref: metamodelica::Ref<ComponentRef::NFComponentRef>;
    fresh_adjoint_map = UnorderedMap::new(
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
        16,
    );
    diffArgs = metamodelica::Ref::new(DifferentiationArguments::DifferentiationArguments {
        diffCref: openmodelica_nf_frontend::NFComponentRef::interned_EMPTY(),
        new_vars: metamodelica::nil(),
        diff_map: Some(diff_map),
        diffType: DifferentiationType::JACOBIAN.clone(),
        funcMap: funcMap,
        scalarized: scalarized,
        adjoint_map: Some(fresh_adjoint_map),
        current_grad: metamodelica::Ref::new(Expression::NFExpression::EMPTY {
            ty: openmodelica_nf_frontend::NFType::interned_REAL(),
        }),
        collectAdjoints: true,
    });
    (diffArgs, adjStmts) = Differentiate::differentiateEquationAdjoint(eq, diffArgs)?;
    if !((adjStmts).is_empty()) {
        eqPtr = NBEquation::Equation::makeAlgorithm(adjStmts.clone(), init)?;
        NBEquation::Equation::createName(eqPtr.clone(), idx, contextName)?;
        adjVarSlices = collectAdjointVarSlices(&adjStmts, metamodelica::nil())?.reverse();
        adjVarCref = (match &**originalComp {
            StrongComponent::SLICED_COMPONENT {
                var_cref: __originalComp_var_cref,
                ..
            } => __originalComp_var_cref.clone(),
            StrongComponent::RESIZABLE_COMPONENT {
                var_cref: __originalComp_var_cref,
                ..
            } => __originalComp_var_cref.clone(),
            StrongComponent::GENERIC_COMPONENT {
                var_cref: __originalComp_var_cref,
                ..
            } => __originalComp_var_cref.clone(),
            _ => openmodelica_nf_frontend::NFComponentRef::interned_EMPTY(),
        });
        adjointComps = list![metamodelica::Ref::new(
            StrongComponent::NBStrongComponent::MULTI_COMPONENT {
                vars: adjVarSlices,
                eqn: metamodelica::Ref::new(Slice::NBSlice {
                    t: eqPtr,
                    indices: metamodelica::nil()
                }),
                status: NBSolve::Status::EXPLICIT.clone()
            }
        )];
    }
    Ok(adjointComps)
}

fn collectAdjointVarSlices(
    mut stmts: &metamodelica::List<metamodelica::Ref<Statement::NFStatement>>,
    mut varSlices: metamodelica::List<
        metamodelica::Ref<Slice::NBSlice<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>>>,
    >,
) -> Result<
    metamodelica::List<metamodelica::Ref<Slice::NBSlice<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>>>>,
> {
    let mut varSlices: metamodelica::List<
        metamodelica::Ref<Slice::NBSlice<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>>>,
    > = varSlices;
    let mut vPtr: Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>;
    let mut baseCref: metamodelica::Ref<ComponentRef::NFComponentRef>;
    for mut s in &**stmts {
        let () = (::match_deref::match_deref! { match &(s.clone()) {
            Deref @ Statement::ASSIGNMENT { lhs: Deref @ Expression::CREF { .. }, .. } => {
                baseCref = ComponentRef::stripSubscriptsAll(&(Expression::toCref(var_field!((**s).lhs, Statement::NFStatement::ASSIGNMENT))?));
                if '__try0: {
                    vPtr = unwrap_break_err!(NBVariable::getVarPointer(&baseCref, metamodelica::sourceInfo!("NBackEnd/Modules/3_Post/NBJacobian.mo")), '__try0);
                    varSlices = metamodelica::cons(metamodelica::Ref::new(Slice::NBSlice { t: vPtr.clone(), indices: metamodelica::nil() }), varSlices.clone());
                    Ok::<(), &'static str>(())
                }.is_err() {
                }
                ()
            },
            Deref @ Statement::FOR { body: __s_body, .. } => {
                varSlices = collectAdjointVarSlices(metamodelica::AsArg::as_arg(&__s_body), varSlices)?;
                ()
            },
            Deref @ Statement::IF { branches: __s_branches, .. } => {
                for mut branch in &*__s_branches.clone() {
                    varSlices = collectAdjointVarSlices(&(Util::tuple22(branch.clone())), varSlices)?;
                }
                ()
            },
            _ => (),
            _ => unreachable!("match_deref! exhaustiveness placeholder"),
        } });
    }
    Ok(varSlices)
}

fn jacobianSymbolicAdjoint(
    mut name: ArcStr,
    mut jacType: JacobianType,
    mut seedCandidates: metamodelica::Ref<VariablePointers::VariablePointers>,
    mut partialCandidates: metamodelica::Ref<VariablePointers::VariablePointers>,
    mut equations: metamodelica::Ref<EquationPointers::EquationPointers>,
    mut strongComponents: Option<metamodelica::Array<metamodelica::Ref<StrongComponent::NBStrongComponent>>>,
    mut full: Option<metamodelica::Ref<Adjacency::Matrix::Matrix>>,
    mut funcMap: metamodelica::Ref<
        UnorderedMap::UnorderedMap<metamodelica::Ref<Path>, metamodelica::Ref<Function::Function>>,
    >,
    mut staticAsContinuous: bool,
) -> Result<Option<metamodelica::Ref<Jacobian::NBackendDAE>>> {
    let mut jacobian: Option<metamodelica::Ref<Jacobian::NBackendDAE>>;
    let mut comps: metamodelica::List<metamodelica::Ref<StrongComponent::NBStrongComponent>>;
    let mut primalComps: metamodelica::List<metamodelica::Ref<StrongComponent::NBStrongComponent>>;
    let mut diffed_comps: metamodelica::List<metamodelica::Ref<StrongComponent::NBStrongComponent>> =
        metamodelica::nil();
    let mut seed_vars_ptr: Pointer::Pointer<
        metamodelica::List<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>>,
    > = Pointer::create(metamodelica::nil());
    let mut pDer_vars_ptr: Pointer::Pointer<
        metamodelica::List<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>>,
    > = Pointer::create(metamodelica::nil());
    let mut diff_map: metamodelica::Ref<
        UnorderedMap::UnorderedMap<
            metamodelica::Ref<ComponentRef::NFComponentRef>,
            metamodelica::Ref<ComponentRef::NFComponentRef>,
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
    let mut idx: Pointer::Pointer<i32> = Pointer::create(0);
    let mut all_vars: metamodelica::List<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>>;
    let mut unknown_vars: metamodelica::List<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>>;
    let mut aux_vars: metamodelica::List<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>>;
    let mut alias_vars: metamodelica::List<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>>;
    let mut depend_vars: metamodelica::List<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>>;
    let mut res_vars: metamodelica::List<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>>;
    let mut tmp_vars: metamodelica::List<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>>;
    let mut seed_vars: metamodelica::List<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>>;
    let mut old_res_vars: metamodelica::List<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>>;
    let mut baseTmpVarCandidates: metamodelica::List<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>>;
    let mut varDataJac: metamodelica::Ref<VarData::VarData>;
    let mut adjacencyVars: metamodelica::Ref<VariablePointers::VariablePointers>;
    let mut fullLocal: metamodelica::Ref<Adjacency::Matrix::Matrix>;
    let mut sparsity: metamodelica::Ref<Adjacency::Matrix::Matrix>;
    let mut seed_set: metamodelica::Ref<UnorderedSet::UnorderedSet<metamodelica::Ref<ComponentRef::NFComponentRef>>> =
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
    let mut pder_set: metamodelica::Ref<UnorderedSet::UnorderedSet<metamodelica::Ref<ComponentRef::NFComponentRef>>> =
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
    let mut newName: ArcStr;
    let mut func: NBVariable::checkVar = getTmpFilterFunction(jacType)?;
    let mut compAdjComps: metamodelica::List<metamodelica::Ref<StrongComponent::NBStrongComponent>>;
    let mut compNewVars: metamodelica::List<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>>;
    newName = {
        let mut __mm_s = String::new();
        __mm_s.push_str(&*name);
        __mm_s.push_str(&*literal!("_ADJ"));
        ArcStr::from(__mm_s)
    };
    if (strongComponents).is_some() {
        comps = ({
            let mut __acc: metamodelica::List<metamodelica::Ref<StrongComponent::NBStrongComponent>> =
                metamodelica::nil();
            for mut comp in (strongComponents.ok_or("pattern mismatch")?).borrow().iter() {
                if !(!(StrongComponent::isDiscrete(&(comp.clone()))?)) {
                    continue;
                }
                let __x = comp.clone();
                __acc = cons(__x, __acc);
            }
            __acc.reverse()
        });
        primalComps = comps.clone();
        for mut c in &*comps {
            if !(isSupportedAdjointStrongComponent(metamodelica::AsArg::as_arg(&c))) {
                Error::addMessage(
                    Error::INTERNAL_ERROR.clone(),
                    list![{
                        let mut __mm_s = String::new();
                        __mm_s.push_str(&*literal!("NBJacobian.jacobianSymbolicAdjoint"));
                        __mm_s.push_str(&*literal!(" only supports SINGLE_COMPONENT, MULTI_COMPONENT, SLICED_COMPONENT, RESIZABLE_COMPONENT and ALGEBRAIC_LOOP in symbolic adjoint jacobian generation!"));
                        ArcStr::from(__mm_s)
                    }],
                )?;
                return Err("fail");
            }
            if Flags::isSet(Flags::DEBUG_ADJOINT.clone())? {
                metamodelica::print({
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!("Primal component: "));
                    __mm_s.push_str(&*StrongComponent::toString(metamodelica::AsArg::as_arg(&c), -1)?);
                    __mm_s.push_str(&*literal!("\n"));
                    ArcStr::from(__mm_s)
                });
            }
        }
    } else {
        Error::addMessage(
            Error::INTERNAL_ERROR.clone(),
            list![{
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("NBJacobian.jacobianSymbolicAdjoint"));
                __mm_s.push_str(&*literal!(" failed because no strong components were given!"));
                ArcStr::from(__mm_s)
            }],
        )?;
        return Err("fail");
    }
    if Flags::isSet(Flags::DEBUG_ADJOINT.clone())? {
        metamodelica::print({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("Seed candidates before pDer creation:\n"));
            __mm_s.push_str(&*NBVariable::VariablePointers::toString(
                &seedCandidates,
                literal!("Seed Candidates"),
                None,
                true,
            )?);
            __mm_s.push_str(&*literal!("\n"));
            ArcStr::from(__mm_s)
        });
        metamodelica::print({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("Partial candidates before pDer creation:\n"));
            __mm_s.push_str(&*NBVariable::VariablePointers::toString(
                &partialCandidates,
                literal!("Partial Candidates"),
                None,
                true,
            )?);
            __mm_s.push_str(&*literal!("\n"));
            ArcStr::from(__mm_s)
        });
    }
    for mut v in &*NBVariable::VariablePointers::toList(&seedCandidates)? {
        makeVarTraverse(
            v.clone(),
            newName.clone(),
            pDer_vars_ptr.clone(),
            diff_map.clone(),
            &({
                let __pe_b2 = false;
                move |__pe_a0, __pe_a1| NBVariable::makePDerVar(__pe_a0, &__pe_a1, __pe_b2.clone())
            }),
            staticAsContinuous,
        )?;
        if NBVariable::isContinuous(v.clone(), staticAsContinuous)? {
            UnorderedSet::add(NBVariable::getVarName(v.clone()), seed_set.clone())?;
        }
    }
    res_vars = Pointer::access(pDer_vars_ptr).reverse();
    (old_res_vars, tmp_vars) = List::splitOnTrue(
        &(NBVariable::VariablePointers::toList(&partialCandidates)?),
        &*(func.clone()),
    )?;
    (tmp_vars, _) = List::splitOnTrue(
        &tmp_vars,
        &({
            let __pe_b1 = staticAsContinuous;
            move |__pe_a0| NBVariable::isContinuous(__pe_a0, __pe_b1.clone())
        }),
    )?;
    for mut v in &*old_res_vars {
        UnorderedSet::add(NBVariable::getVarName(v.clone()), pder_set.clone())?;
    }
    for mut v in &*old_res_vars {
        makeVarTraverse(
            v.clone(),
            newName.clone(),
            seed_vars_ptr.clone(),
            diff_map.clone(),
            &move |__a0: metamodelica::Ref<ComponentRef::NFComponentRef>, __a1: ArcStr| {
                NBVariable::makeSeedVar(__a0, &__a1)
            },
            staticAsContinuous,
        )?;
    }
    seed_vars = Pointer::access(seed_vars_ptr.clone()).reverse();
    if Flags::isSet(Flags::DEBUG_ADJOINT.clone())? {
        metamodelica::print({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("seed vars after seed creation:\n"));
            __mm_s.push_str(&*NBVariable::VariablePointers::toString(
                &(NBVariable::VariablePointers::fromList(&seed_vars, false)?),
                literal!("Seed Vars"),
                None,
                true,
            )?);
            __mm_s.push_str(&*literal!("\n"));
            ArcStr::from(__mm_s)
        });
        metamodelica::print({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("res vars after pDer creation:\n"));
            __mm_s.push_str(&*NBVariable::VariablePointers::toString(
                &(NBVariable::VariablePointers::fromList(&res_vars, false)?),
                literal!("Res Vars"),
                None,
                true,
            )?);
            __mm_s.push_str(&*literal!("\n"));
            ArcStr::from(__mm_s)
        });
        metamodelica::print({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("tmp vars after pDer creation:\n"));
            __mm_s.push_str(&*NBVariable::VariablePointers::toString(
                &(NBVariable::VariablePointers::fromList(&tmp_vars, false)?),
                literal!("Tmp Vars"),
                None,
                true,
            )?);
            __mm_s.push_str(&*literal!("\n"));
            ArcStr::from(__mm_s)
        });
    }
    pDer_vars_ptr = Pointer::create(metamodelica::nil());
    for mut v in &*tmp_vars {
        makeVarTraverse(
            v.clone(),
            newName.clone(),
            pDer_vars_ptr.clone(),
            diff_map.clone(),
            &({
                let __pe_b2 = true;
                move |__pe_a0, __pe_a1| NBVariable::makePDerVar(__pe_a0, &__pe_a1, __pe_b2.clone())
            }),
            staticAsContinuous,
        )?;
    }
    tmp_vars = Pointer::access(pDer_vars_ptr);
    baseTmpVarCandidates = getBaseTmpVarCandidates(
        &(NBVariable::VariablePointers::toList(&partialCandidates)?),
        &tmp_vars,
        diff_map.clone(),
    )?;
    if Flags::isSet(Flags::DEBUG_ADJOINT.clone())? {
        metamodelica::print({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("Diff map before component generation:\n"));
            __mm_s.push_str(&*diffMapToString(diff_map.clone())?);
            __mm_s.push_str(&*literal!("\n"));
            ArcStr::from(__mm_s)
        });
    }
    for mut comp in &*primalComps {
        (compAdjComps, compNewVars) = generateAdjointComponent(
            comp.clone(),
            diff_map.clone(),
            funcMap.clone(),
            seedCandidates.scalarized.clone(),
            staticAsContinuous,
            idx.clone(),
            newName.clone(),
            &(seedCandidates.clone()),
            baseTmpVarCandidates.clone(),
        )?;
        for mut ac in &*compAdjComps {
            diffed_comps = metamodelica::cons(ac.clone(), diffed_comps);
        }
        for mut v in &*compNewVars {
            tmp_vars = metamodelica::cons(v.clone(), tmp_vars);
        }
        if Flags::isSet(Flags::DEBUG_ADJOINT.clone())? {
            for mut ac in &*compAdjComps {
                metamodelica::print({
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!("[adjoint] generated component: "));
                    __mm_s.push_str(&*StrongComponent::toString(metamodelica::AsArg::as_arg(&ac), -1)?);
                    __mm_s.push_str(&*literal!("\n"));
                    ArcStr::from(__mm_s)
                });
            }
        }
    }
    if Flags::isSet(Flags::DEBUG_ADJOINT.clone())? {
        metamodelica::print(literal!("Final list of differentiated components:\n"));
        for mut comp in &*diffed_comps {
            metamodelica::print({
                let mut __mm_s = String::new();
                __mm_s.push_str(&*StrongComponent::toString(metamodelica::AsArg::as_arg(&comp), -1)?);
                __mm_s.push_str(&*literal!("\n"));
                ArcStr::from(__mm_s)
            });
        }
    }
    unknown_vars = listAppend(res_vars.clone(), tmp_vars.clone());
    all_vars = unknown_vars.clone();
    seed_vars = Pointer::access(seed_vars_ptr);
    aux_vars = seed_vars.clone();
    alias_vars = metamodelica::nil();
    depend_vars = metamodelica::nil();
    varDataJac = metamodelica::Ref::new(VarData::VarData::VAR_DATA_JAC {
        variables: NBVariable::VariablePointers::fromList(&all_vars, false)?,
        unknowns: NBVariable::VariablePointers::fromList(&unknown_vars, false)?,
        auxiliaries: NBVariable::VariablePointers::fromList(&aux_vars, false)?,
        aliasVars: NBVariable::VariablePointers::fromList(&alias_vars, false)?,
        diffVars: partialCandidates.clone(),
        dependencies: NBVariable::VariablePointers::fromList(&depend_vars, false)?,
        resultVars: NBVariable::VariablePointers::fromList(&res_vars, false)?,
        tmpVars: NBVariable::VariablePointers::fromList(&tmp_vars, false)?,
        seedVars: NBVariable::VariablePointers::fromList(&seed_vars, false)?,
    });
    adjacencyVars = NBVariable::VariablePointers::clone(&seedCandidates, true)?;
    adjacencyVars = NBVariable::VariablePointers::addList(&baseTmpVarCandidates, adjacencyVars)?;
    if jacType == JacobianType::ODE.clone() {
        adjacencyVars = NBVariable::VariablePointers::addList(
            &(NBVariable::VariablePointers::toList(&partialCandidates)?),
            adjacencyVars,
        )?;
    }
    fullLocal = Adjacency::Matrix::createFull(
        &adjacencyVars,
        &(NBEquation::EquationPointers::fromList(
            &(List::flatten(
                ({
                    let mut __acc: metamodelica::List<
                        metamodelica::List<Pointer::Pointer<metamodelica::Ref<Equation::Equation>>>,
                    > = metamodelica::nil();
                    for mut comp in (comps.clone()).into_iter().cloned() {
                        let __x = StrongComponent::getEquations(comp.clone())?;
                        __acc = cons(__x, __acc);
                    }
                    __acc.reverse()
                }),
            )?),
        )?),
        Partition::Kind::ODE.clone(),
    )?;
    sparsity = Adjacency::Matrix::fullToSparsity(&fullLocal, &comps, seed_set, pder_set, diff_map, true)?;
    jacobian = Some(metamodelica::Ref::new(Jacobian::NBackendDAE::JACOBIAN {
        name: newName,
        jacType: jacType,
        varData: varDataJac,
        comps: metamodelica::arrayFromVec(diffed_comps.into_iter().cloned().collect()),
        sparsity: sparsity,
        isAdjoint: true,
    }));
    Ok(jacobian)
}

fn jacobianNumeric(
    mut name: ArcStr,
    mut jacType: JacobianType,
    mut seedCandidates: metamodelica::Ref<VariablePointers::VariablePointers>,
    mut partialCandidates: metamodelica::Ref<VariablePointers::VariablePointers>,
    mut equations: metamodelica::Ref<EquationPointers::EquationPointers>,
    mut strongComponents: Option<metamodelica::Array<metamodelica::Ref<StrongComponent::NBStrongComponent>>>,
    mut full: Option<metamodelica::Ref<Adjacency::Matrix::Matrix>>,
    mut funcMap: metamodelica::Ref<
        UnorderedMap::UnorderedMap<metamodelica::Ref<Path>, metamodelica::Ref<Function::Function>>,
    >,
    mut staticAsContinuous: bool,
) -> Result<Option<metamodelica::Ref<Jacobian::NBackendDAE>>> {
    let mut jacobian: Option<metamodelica::Ref<Jacobian::NBackendDAE>>;
    let mut varDataJac: metamodelica::Ref<VarData::VarData>;
    let mut adjacencyVars: metamodelica::Ref<VariablePointers::VariablePointers>;
    let mut sparsity: metamodelica::Ref<Adjacency::Matrix::Matrix>;
    let mut fullLocal: metamodelica::Ref<Adjacency::Matrix::Matrix>;
    let mut res_vars: metamodelica::List<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>>;
    let mut tmp_vars: metamodelica::List<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>>;
    let mut seed_vars_d: metamodelica::List<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>>;
    let mut pDer_vars_d: metamodelica::List<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>>;
    let mut func: NBVariable::checkVar = getTmpFilterFunction(jacType)?;
    let mut seed_vars_ptr: Pointer::Pointer<
        metamodelica::List<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>>,
    > = Pointer::create(metamodelica::nil());
    let mut pDer_vars_ptr: Pointer::Pointer<
        metamodelica::List<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>>,
    > = Pointer::create(metamodelica::nil());
    let mut diff_map: metamodelica::Ref<
        UnorderedMap::UnorderedMap<
            metamodelica::Ref<ComponentRef::NFComponentRef>,
            metamodelica::Ref<ComponentRef::NFComponentRef>,
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
    let mut seed_set: metamodelica::Ref<UnorderedSet::UnorderedSet<metamodelica::Ref<ComponentRef::NFComponentRef>>> =
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
    let mut pder_set: metamodelica::Ref<UnorderedSet::UnorderedSet<metamodelica::Ref<ComponentRef::NFComponentRef>>> =
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
    (res_vars, tmp_vars) = List::splitOnTrue(
        &(NBVariable::VariablePointers::toList(&partialCandidates)?),
        &*(func.clone()),
    )?;
    (tmp_vars, _) = List::splitOnTrue(
        &tmp_vars,
        &({
            let __pe_b1 = staticAsContinuous;
            move |__pe_a0| NBVariable::isContinuous(__pe_a0, __pe_b1.clone())
        }),
    )?;
    NBVariable::VariablePointers::mapPtr(
        seedCandidates.clone(),
        &({
            let __pe_b1 = name.clone();
            let __pe_b2 = seed_vars_ptr.clone();
            let __pe_b3 = diff_map.clone();
            let __pe_b4: Arc<
                dyn ::std::ops::Fn(
                        metamodelica::Ref<ComponentRef::NFComponentRef>,
                        ArcStr,
                    ) -> Result<(
                        metamodelica::Ref<ComponentRef::NFComponentRef>,
                        Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>,
                    )> + 'static,
            > = (std::sync::Arc::new(
                move |__a0: metamodelica::Ref<ComponentRef::NFComponentRef>, __a1: ArcStr| {
                    NBVariable::makeSeedVar(__a0, &__a1)
                },
            )
                as std::sync::Arc<
                    dyn ::std::ops::Fn(
                            metamodelica::Ref<ComponentRef::NFComponentRef>,
                            ArcStr,
                        ) -> Result<(
                            metamodelica::Ref<ComponentRef::NFComponentRef>,
                            Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>,
                        )> + 'static,
                >);
            let __pe_b5 = staticAsContinuous;
            move |__pe_a0| {
                makeVarTraverse(
                    __pe_a0,
                    __pe_b1.clone(),
                    __pe_b2.clone(),
                    __pe_b3.clone(),
                    &*__pe_b4,
                    __pe_b5.clone(),
                )
            }
        }),
    )?;
    seed_vars_d = Pointer::access(seed_vars_ptr);
    for mut v in &*NBVariable::VariablePointers::toList(&seedCandidates)? {
        if NBVariable::isContinuous(v.clone(), staticAsContinuous)? {
            UnorderedSet::add(NBVariable::getVarName(v.clone()), seed_set.clone())?;
            UnorderedSet::add(
                ComponentRef::stripSubscriptsAll(&(NBVariable::getVarName(v.clone()))),
                seed_set.clone(),
            )?;
        }
    }
    for mut v in &*res_vars {
        UnorderedSet::add(NBVariable::getVarName(v.clone()), pder_set.clone())?;
        makeVarTraverse(
            v.clone(),
            name.clone(),
            pDer_vars_ptr.clone(),
            diff_map.clone(),
            &({
                let __pe_b2 = false;
                move |__pe_a0, __pe_a1| NBVariable::makePDerVar(__pe_a0, &__pe_a1, __pe_b2.clone())
            }),
            staticAsContinuous,
        )?;
    }
    pDer_vars_d = Pointer::access(pDer_vars_ptr);
    varDataJac = metamodelica::Ref::new(VarData::VarData::VAR_DATA_JAC {
        variables: NBVariable::VariablePointers::fromList(&(metamodelica::nil()), false)?,
        unknowns: partialCandidates.clone(),
        auxiliaries: NBVariable::VariablePointers::fromList(&seed_vars_d, false)?,
        aliasVars: NBVariable::VariablePointers::fromList(&(metamodelica::nil()), false)?,
        diffVars: partialCandidates,
        dependencies: NBVariable::VariablePointers::fromList(&(metamodelica::nil()), false)?,
        resultVars: NBVariable::VariablePointers::fromList(&pDer_vars_d, false)?,
        tmpVars: NBVariable::VariablePointers::fromList(&tmp_vars, false)?,
        seedVars: NBVariable::VariablePointers::fromList(&seed_vars_d, false)?,
    });
    if (strongComponents).is_some() {
        adjacencyVars = NBVariable::VariablePointers::clone(&seedCandidates, true)?;
        adjacencyVars = NBVariable::VariablePointers::addList(&tmp_vars, adjacencyVars)?;
        if jacType == JacobianType::ODE.clone() {
            adjacencyVars = NBVariable::VariablePointers::addList(&res_vars, adjacencyVars)?;
        }
        fullLocal = Adjacency::Matrix::createFull(
            &adjacencyVars,
            &(NBEquation::EquationPointers::fromList(
                &(List::flatten(
                    ({
                        let mut __acc: metamodelica::List<
                            metamodelica::List<Pointer::Pointer<metamodelica::Ref<Equation::Equation>>>,
                        > = metamodelica::nil();
                        for mut comp in (strongComponents
                            .clone()
                            .ok_or("pattern mismatch")?
                            .borrow()
                            .iter()
                            .cloned()
                            .collect::<metamodelica::List<_>>())
                        .into_iter()
                        .cloned()
                        {
                            let __x = StrongComponent::getEquations(comp.clone())?;
                            __acc = cons(__x, __acc);
                        }
                        __acc.reverse()
                    }),
                )?),
            )?),
            Partition::Kind::ODE.clone(),
        )?;
        sparsity = Adjacency::Matrix::fullToSparsity(
            &fullLocal,
            &(strongComponents
                .ok_or("pattern mismatch")?
                .borrow()
                .iter()
                .cloned()
                .collect::<metamodelica::List<_>>()),
            seed_set,
            pder_set,
            diff_map,
            false,
        )?;
    } else {
        Error::addMessage(
            Error::INTERNAL_ERROR.clone(),
            list![{
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("NBJacobian.jacobianNumeric"));
                __mm_s.push_str(&*literal!(" failed because strong components are missing."));
                ArcStr::from(__mm_s)
            }],
        )?;
        return Err("fail");
    }
    jacobian = Some(metamodelica::Ref::new(Jacobian::NBackendDAE::JACOBIAN {
        name: name,
        jacType: jacType,
        varData: varDataJac,
        comps: metamodelica::arrayFromVec(metamodelica::nil().into_iter().cloned().collect()),
        sparsity: sparsity,
        isAdjoint: false,
    }));
    Ok(jacobian)
}

fn jacobianNone(
    mut name: ArcStr,
    mut jacType: JacobianType,
    mut seedCandidates: metamodelica::Ref<VariablePointers::VariablePointers>,
    mut partialCandidates: metamodelica::Ref<VariablePointers::VariablePointers>,
    mut equations: metamodelica::Ref<EquationPointers::EquationPointers>,
    mut strongComponents: Option<metamodelica::Array<metamodelica::Ref<StrongComponent::NBStrongComponent>>>,
    mut full: Option<metamodelica::Ref<Adjacency::Matrix::Matrix>>,
    mut funcMap: metamodelica::Ref<
        UnorderedMap::UnorderedMap<metamodelica::Ref<Path>, metamodelica::Ref<Function::Function>>,
    >,
    mut staticAsContinuous: bool,
) -> Option<metamodelica::Ref<Jacobian::NBackendDAE>> {
    let mut jacobian: Option<metamodelica::Ref<Jacobian::NBackendDAE>>;
    jacobian = None;
    jacobian
}

fn getTmpFilterFunction(
    mut jacType: JacobianType,
) -> Result<Arc<dyn ::std::ops::Fn(Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>) -> Result<bool> + 'static>>
{
    let mut func: NBVariable::checkVar;
    func = (match jacType {
        JacobianType::ODE => {
            (std::sync::Arc::new(fnptr!(
                NBVariable::isStateDerivative,
                Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>
            ))
                as std::sync::Arc<
                    dyn ::std::ops::Fn(Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>) -> Result<bool>
                        + 'static,
                >)
        }
        JacobianType::DAE { .. } => {
            (std::sync::Arc::new(fnptr!(
                NBVariable::isResidual,
                Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>
            ))
                as std::sync::Arc<
                    dyn ::std::ops::Fn(Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>) -> Result<bool>
                        + 'static,
                >)
        }
        JacobianType::LS => {
            (std::sync::Arc::new(fnptr!(
                NBVariable::isResidual,
                Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>
            ))
                as std::sync::Arc<
                    dyn ::std::ops::Fn(Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>) -> Result<bool>
                        + 'static,
                >)
        }
        JacobianType::NLS => {
            (std::sync::Arc::new(fnptr!(
                NBVariable::isResidual,
                Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>
            ))
                as std::sync::Arc<
                    dyn ::std::ops::Fn(Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>) -> Result<bool>
                        + 'static,
                >)
        }
        JacobianType::OPT_LFG => {
            (std::sync::Arc::new(fnptr!(
                NBVariable::isLfgFunction,
                Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>
            ))
                as std::sync::Arc<
                    dyn ::std::ops::Fn(Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>) -> Result<bool>
                        + 'static,
                >)
        }
        JacobianType::OPT_MRF => {
            (std::sync::Arc::new(fnptr!(
                NBVariable::isMrfFunction,
                Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>
            ))
                as std::sync::Arc<
                    dyn ::std::ops::Fn(Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>) -> Result<bool>
                        + 'static,
                >)
        }
        JacobianType::OPT_R0 => {
            (std::sync::Arc::new(fnptr!(
                NBVariable::isInitialConstraint,
                Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>
            ))
                as std::sync::Arc<
                    dyn ::std::ops::Fn(Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>) -> Result<bool>
                        + 'static,
                >)
        }
        _ => {
            Error::addMessage(
                Error::INTERNAL_ERROR.clone(),
                list![{
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!("NBJacobian.getTmpFilterFunction"));
                    __mm_s.push_str(&*literal!(" failed because jacobian type is not known: "));
                    __mm_s.push_str(&*jacobianTypeString(jacType));
                    ArcStr::from(__mm_s)
                }],
            )?;
            return Err("fail");
        }
    });
    Ok(func)
}

fn makeVarTraverse(
    mut var_ptr: Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>,
    mut name: ArcStr,
    mut vars_ptr: Pointer::Pointer<metamodelica::List<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>>>,
    mut map: metamodelica::Ref<
        UnorderedMap::UnorderedMap<
            metamodelica::Ref<ComponentRef::NFComponentRef>,
            metamodelica::Ref<ComponentRef::NFComponentRef>,
        >,
    >,
    mut makeVar: &dyn ::std::ops::Fn(
        metamodelica::Ref<ComponentRef::NFComponentRef>,
        ArcStr,
    ) -> Result<(
        metamodelica::Ref<ComponentRef::NFComponentRef>,
        Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>,
    )>,
    mut staticAsContinuous: bool,
) -> Result<()> {
    pub type Func = std::sync::Arc<
        dyn ::std::ops::Fn(
                metamodelica::Ref<ComponentRef::NFComponentRef>,
                ArcStr,
            ) -> Result<(
                metamodelica::Ref<ComponentRef::NFComponentRef>,
                Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>,
            )> + 'static,
    >;

    let mut var: metamodelica::Ref<Variable::NFVariable> = Pointer::access(var_ptr.clone());
    let mut diff: metamodelica::Ref<ComponentRef::NFComponentRef>;
    let mut parent_name: metamodelica::Ref<ComponentRef::NFComponentRef>;
    let mut diff_parent_name: metamodelica::Ref<ComponentRef::NFComponentRef>;
    let mut diff_ptr: Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>;
    let mut parent: Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>;
    let mut diff_parent: Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>;
    if NBVariable::isContinuous(var_ptr.clone(), staticAsContinuous)? {
        (diff, diff_ptr) = makeVar(var.name.clone(), name.clone())?;
        Pointer::update(
            vars_ptr.clone(),
            metamodelica::cons(diff_ptr.clone(), Pointer::access(vars_ptr)),
        );
        UnorderedMap::add(var.name.clone(), diff.clone(), map.clone())?;
        if ComponentRef::hasSubscripts(&var.name)?
            && !(List::all(
                &(ComponentRef::subscriptsAllFlat(&var.name)?),
                &move |__a0: metamodelica::Ref<Subscript::NFSubscript>| Subscript::isLiteral(&__a0),
            )?)
            && !(UnorderedMap::contains(ComponentRef::stripSubscriptsAll(&var.name), map.clone())?)
        {
            UnorderedMap::add(ComponentRef::stripSubscriptsAll(&var.name), diff, map.clone())?;
        }
        let () = (match NBVariable::getParent(var_ptr) {
            Some(mut __esc_parent) => {
                parent = __esc_parent.clone();
                parent_name = NBVariable::getVarName(parent);
                diff_parent = (::match_deref::match_deref! { match &(UnorderedMap::get(parent_name.clone(), map.clone())?) {
                    Some(__esc_diff_parent_name) => {
                        diff_parent_name = (*__esc_diff_parent_name).clone();
                        NBVariable::getVarPointer(metamodelica::AsArg::as_arg(&diff_parent_name), metamodelica::sourceInfo!("NBackEnd/Modules/3_Post/NBJacobian.mo"))?
                    },
                    _ => {
                        (diff_parent_name, _) = makeVar(parent_name.clone(), name)?;
                        UnorderedMap::add(parent_name, diff_parent_name.clone(), map)?;
                        NBVariable::getVarPointer(&diff_parent_name, metamodelica::sourceInfo!("NBackEnd/Modules/3_Post/NBJacobian.mo"))?
                    },
                    _ => unreachable!("match_deref! exhaustiveness placeholder"),
                } });
                NBVariable::addRecordChild(diff_parent.clone(), diff_ptr.clone())?;
                diff_ptr = NBVariable::setParent(diff_ptr, diff_parent);
                ()
            }
            _ => (),
        });
    }
    Ok(())
}

fn diffMapToString(
    mut map: metamodelica::Ref<
        UnorderedMap::UnorderedMap<
            metamodelica::Ref<ComponentRef::NFComponentRef>,
            metamodelica::Ref<ComponentRef::NFComponentRef>,
        >,
    >,
) -> Result<ArcStr> {
    let mut s: ArcStr;
    s = UnorderedMap::toString(
        map,
        &move |__a0: metamodelica::Ref<ComponentRef::NFComponentRef>| ComponentRef::toString(&__a0),
        &move |__a0: metamodelica::Ref<ComponentRef::NFComponentRef>| ComponentRef::toString(&__a0),
        literal!("\n  "),
        &(literal!(" -> ")),
    )?;
    s = {
        let mut __mm_s = String::new();
        __mm_s.push_str(&*literal!("{\n  "));
        __mm_s.push_str(&*s);
        __mm_s.push_str(&*literal!("\n}"));
        ArcStr::from(__mm_s)
    };
    Ok(s)
}

fn makeLinearAlgebraicLoop(
    mut itVarPtrs: metamodelica::List<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>>,
    mut resEqnPtrs: metamodelica::List<Pointer::Pointer<metamodelica::Ref<Equation::Equation>>>,
    mut jac: Option<metamodelica::Ref<Jacobian::NBackendDAE>>,
    mut mixed: bool,
    mut homotopy: bool,
) -> Result<metamodelica::Ref<StrongComponent::NBStrongComponent>> {
    let mut comp: metamodelica::Ref<StrongComponent::NBStrongComponent>;
    let mut m1: i32 = ((itVarPtrs).len() as i32);
    let mut m2: i32 = ((resEqnPtrs).len() as i32);
    let mut itVars_s: metamodelica::List<
        metamodelica::Ref<Slice::NBSlice<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>>>,
    >;
    let mut res_s: metamodelica::List<
        metamodelica::Ref<Slice::NBSlice<Pointer::Pointer<metamodelica::Ref<Equation::Equation>>>>,
    >;
    let mut tearingSet: metamodelica::Ref<Tearing::NBTearing>;
    if m1 != m2 {
        Error::addMessage(
            Error::INTERNAL_ERROR.clone(),
            list![literal!("makeLinearAlgebraicLoop: |vars| != |eqns|")],
        )?;
        return Err("fail");
    }
    itVars_s = ({
        let mut __acc: metamodelica::List<
            metamodelica::Ref<Slice::NBSlice<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>>>,
        > = metamodelica::nil();
        for mut vp in (itVarPtrs).into_iter().cloned() {
            let __x = metamodelica::Ref::new(Slice::NBSlice {
                t: vp.clone(),
                indices: metamodelica::nil(),
            });
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    res_s = ({
        let mut __acc: metamodelica::List<
            metamodelica::Ref<Slice::NBSlice<Pointer::Pointer<metamodelica::Ref<Equation::Equation>>>>,
        > = metamodelica::nil();
        for mut ep in (resEqnPtrs).into_iter().cloned() {
            let __x = metamodelica::Ref::new(Slice::NBSlice {
                t: ep.clone(),
                indices: metamodelica::nil(),
            });
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    tearingSet = metamodelica::Ref::new(Tearing::NBTearing {
        iteration_vars: itVars_s,
        residual_eqns: res_s,
        innerEquations: metamodelica::arrayFromVec(metamodelica::nil().into_iter().cloned().collect()),
        jac: jac,
    });
    comp = metamodelica::Ref::new(StrongComponent::NBStrongComponent::ALGEBRAIC_LOOP {
        idx: -1,
        strict: tearingSet,
        casual: None,
        linear: true,
        mixed: mixed,
        homotopy: homotopy,
        status: NBSolve::Status::IMPLICIT.clone(),
        implicitlyCreated: false,
    });
    Ok(comp)
}

fn makeSSAVar(
    mut baseCref: &metamodelica::Ref<ComponentRef::NFComponentRef>,
    mut idx: i32,
) -> Result<(
    Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>,
    metamodelica::Ref<ComponentRef::NFComponentRef>,
)> {
    let mut ssaVarPtr: Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>;
    let mut ssaCref: metamodelica::Ref<ComponentRef::NFComponentRef>;
    let mut origVarPtr: Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>;
    let mut origVar: metamodelica::Ref<Variable::NFVariable>;
    let mut newNode: metamodelica::Ref<InstNode::InstNode>;
    let mut ty: metamodelica::Ref<Type::NFType>;
    origVarPtr = NBVariable::getVarPointer(
        baseCref,
        metamodelica::sourceInfo!("NBackEnd/Modules/3_Post/NBJacobian.mo"),
    )?;
    origVar = Pointer::access(origVarPtr);
    ty = ComponentRef::getSubscriptedType(baseCref, false)?;
    newNode = metamodelica::Ref::new(InstNode::InstNode::VAR_NODE {
        name: {
            let mut __mm_s = String::new();
            __mm_s.push_str(&*ComponentRef::firstName(baseCref, false)?);
            __mm_s.push_str(&*literal!("_"));
            __mm_s.push_str(&*intString(idx));
            ArcStr::from(__mm_s)
        },
        varPointer: PointerWeak::downgrade(Pointer::createImmutable(NBVariable::DUMMY_VARIABLE().clone())),
    });
    ssaCref = ComponentRef::fromNode(newNode, ty, metamodelica::nil(), ComponentRef::Origin::CREF.clone())?;
    assign_field!(
        origVar.backendinfo = metamodelica::Ref::new(BackendInfo::BackendInfo {
            varKind: origVar.backendinfo.varKind.clone(),
            attributes: origVar.backendinfo.attributes.clone(),
            annotations: origVar.backendinfo.annotations.clone(),
            var_pre: origVar.backendinfo.var_pre.clone(),
            var_seed: None,
            var_pder_res: None,
            var_pder_tmp: None,
            var_start: origVar.backendinfo.var_start.clone(),
            parent: origVar.backendinfo.parent.clone()
        })
    );
    (ssaVarPtr, ssaCref) = NBVariable::makeVarPtr(origVar, ssaCref)?;
    Ok((ssaVarPtr, ssaCref))
}

fn algorithmToSSA(
    mut comp: &metamodelica::Ref<StrongComponent::NBStrongComponent>,
) -> Result<(
    metamodelica::Ref<StrongComponent::NBStrongComponent>,
    metamodelica::List<(
        metamodelica::Ref<ComponentRef::NFComponentRef>,
        (metamodelica::Ref<ComponentRef::NFComponentRef>, i32),
    )>,
    metamodelica::List<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>>,
)> {
    let mut ssaComp: metamodelica::Ref<StrongComponent::NBStrongComponent>;
    let mut replacements: metamodelica::List<(
        metamodelica::Ref<ComponentRef::NFComponentRef>,
        (metamodelica::Ref<ComponentRef::NFComponentRef>, i32),
    )>;
    let mut newVars: metamodelica::List<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>>;
    let mut eqn: metamodelica::Ref<Equation::Equation>;
    let mut alg: metamodelica::Ref<Algorithm::NFAlgorithm>;
    let mut stmt: metamodelica::Ref<Statement::NFStatement>;
    let mut lhsCref: metamodelica::Ref<ComponentRef::NFComponentRef>;
    let mut baseCref: metamodelica::Ref<ComponentRef::NFComponentRef>;
    let mut ssaCref: metamodelica::Ref<ComponentRef::NFComponentRef>;
    let mut cnt: i32;
    let mut idx: i32;
    let mut lineIdx: i32;
    let mut ssaVarPtr: Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>;
    let mut lhsExp: metamodelica::Ref<Expression::NFExpression>;
    let mut rhsExp: metamodelica::Ref<Expression::NFExpression>;
    let mut assignCount: metamodelica::Ref<
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
    let mut ssaIdx: metamodelica::Ref<
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
    let mut activeRepl: metamodelica::Ref<
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
    let mut ssaStmts: metamodelica::List<metamodelica::Ref<Statement::NFStatement>> = metamodelica::nil();
    let mut replAcc: metamodelica::List<(
        metamodelica::Ref<ComponentRef::NFComponentRef>,
        (metamodelica::Ref<ComponentRef::NFComponentRef>, i32),
    )> = metamodelica::nil();
    let mut newVarsAcc: metamodelica::List<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>> =
        metamodelica::nil();
    let mut ssaEqnPtr: Pointer::Pointer<metamodelica::Ref<Equation::Equation>>;
    (ssaComp, replacements, newVars) = (match &**comp {
        StrongComponent::MULTI_COMPONENT {
            eqn: __comp_eqn,
            status: __comp_status,
            vars: __comp_vars,
        } => {
            eqn = Pointer::access(Slice::getT(__comp_eqn.clone()));
            let __pa0 = ::match_deref::match_deref! { match &(eqn.clone()) {
                Deref @ NBEquation::Equation::ALGORITHM { alg: __pa0, .. } => __pa0.clone(),
                _ => return Err("pattern mismatch"),
            } };
            alg = metamodelica::Own::own(__pa0);
            for mut origStmt in &*alg.statements.clone() {
                let () = (match &*origStmt.clone() {
                    Statement::ASSIGNMENT {
                        lhs: __origStmt_lhs, ..
                    } => {
                        lhsCref = (match &*__origStmt_lhs.clone() {
                            Expression::CREF {
                                cref: __esc_lhsCref, ..
                            } => {
                                lhsCref = (*__esc_lhsCref).clone();
                                lhsCref.clone()
                            }
                            _ => openmodelica_nf_frontend::NFComponentRef::interned_EMPTY(),
                        });
                        if !(ComponentRef::isEmpty(&lhsCref)) {
                            baseCref = ComponentRef::stripSubscriptsAll(&lhsCref);
                            cnt = UnorderedMap::getOrDefault(baseCref.clone(), assignCount.clone(), 0)?;
                            UnorderedMap::add(baseCref, cnt + 1, assignCount.clone())?;
                        }
                        ()
                    }
                    _ => (),
                });
            }
            lineIdx = 1;
            for mut origStmt in &*alg.statements.clone() {
                stmt = (match &*origStmt.clone() {
                    Statement::ASSIGNMENT {
                        lhs: __origStmt_lhs,
                        rhs: __origStmt_rhs,
                        source: __origStmt_source,
                        ty: __origStmt_ty,
                    } => {
                        rhsExp = Expression::map(
                            __origStmt_rhs.clone(),
                            (std::sync::Arc::new({
                                let __pe_b1 = activeRepl.clone();
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
                        lhsExp = __origStmt_lhs.clone();
                        lhsCref = (match &*__origStmt_lhs.clone() {
                            Expression::CREF {
                                cref: __esc_lhsCref, ..
                            } => {
                                lhsCref = (*__esc_lhsCref).clone();
                                lhsCref.clone()
                            }
                            _ => openmodelica_nf_frontend::NFComponentRef::interned_EMPTY(),
                        });
                        if !(ComponentRef::isEmpty(&lhsCref)) {
                            baseCref = ComponentRef::stripSubscriptsAll(&lhsCref);
                            if UnorderedMap::getOrDefault(baseCref.clone(), assignCount.clone(), 1)? > 1 {
                                idx = UnorderedMap::getOrDefault(baseCref.clone(), ssaIdx.clone(), 0)? + 1;
                                UnorderedMap::add(baseCref.clone(), idx, ssaIdx.clone())?;
                                (ssaVarPtr, ssaCref) = makeSSAVar(&baseCref, idx)?;
                                newVarsAcc = metamodelica::cons(ssaVarPtr, newVarsAcc);
                                ssaCref = ComponentRef::copySubscripts(&lhsCref, ssaCref)?;
                                UnorderedMap::add(
                                    baseCref.clone(),
                                    Expression::fromCref(ComponentRef::stripSubscriptsAll(&ssaCref), false)?,
                                    activeRepl.clone(),
                                )?;
                                replAcc = metamodelica::cons(
                                    (baseCref, (ComponentRef::stripSubscriptsAll(&ssaCref), lineIdx)),
                                    replAcc,
                                );
                                lhsExp = Expression::fromCref(ssaCref, false)?;
                            }
                        }
                        metamodelica::Ref::new(Statement::NFStatement::ASSIGNMENT {
                            lhs: lhsExp,
                            rhs: rhsExp,
                            ty: __origStmt_ty.clone(),
                            source: __origStmt_source.clone(),
                        })
                    }
                    _ => origStmt.clone(),
                });
                ssaStmts = metamodelica::cons(stmt, ssaStmts);
                lineIdx = lineIdx + 1;
            }
            assign_field!(alg.statements = ssaStmts.reverse());
            eqn = (match &*eqn {
                NBEquation::Equation::ALGORITHM { .. } => {
                    assign_variant_field!(eqn => Equation::Equation::ALGORITHM; alg = alg);
                    eqn
                }
                _ => eqn,
            });
            ssaEqnPtr = Pointer::create(eqn);
            (
                metamodelica::Ref::new(StrongComponent::NBStrongComponent::MULTI_COMPONENT {
                    vars: listAppend(
                        __comp_vars.clone(),
                        ({
                            let mut __acc: metamodelica::List<
                                metamodelica::Ref<
                                    Slice::NBSlice<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>>,
                                >,
                            > = metamodelica::nil();
                            for mut v in (newVarsAcc.clone().reverse()).into_iter().cloned() {
                                let __x = metamodelica::Ref::new(Slice::NBSlice {
                                    t: v.clone(),
                                    indices: metamodelica::nil(),
                                });
                                __acc = cons(__x, __acc);
                            }
                            __acc.reverse()
                        }),
                    ),
                    eqn: metamodelica::Ref::new(Slice::NBSlice {
                        t: ssaEqnPtr,
                        indices: metamodelica::nil(),
                    }),
                    status: __comp_status.clone(),
                }),
                replAcc.reverse(),
                newVarsAcc.reverse(),
            )
        }
        _ => {
            Error::addMessage(
                Error::INTERNAL_ERROR.clone(),
                list![{
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!("NBJacobian.algorithmToSSA"));
                    __mm_s.push_str(&*literal!(" expects a MULTI_COMPONENT with an ALGORITHM equation."));
                    ArcStr::from(__mm_s)
                }],
            )?;
            return Err("fail");
        }
    });
    Ok((ssaComp, replacements, newVars))
}
