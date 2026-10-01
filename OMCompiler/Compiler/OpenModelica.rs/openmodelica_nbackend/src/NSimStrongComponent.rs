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

use crate::NBEquation as BEquation;
use crate::NBEquation::Equation;
use crate::NBEquation::EquationAttributes;
use crate::NBEquation::EquationKind;
use crate::NBEquation::EquationPointer;
use crate::NBEquation::EquationPointers;
use crate::NBEquation::IfEquationBody;
use crate::NBEquation::Iterator;
use crate::NBEquation::SlicingStatus;
use crate::NBEquation::WhenEquationBody;
use crate::NBEquation::WhenStatement;
use crate::NBPartition as Partition;
use crate::NBPartitioning as Partitioning;
use crate::NBPartitioning::BClock;
use crate::NBPartitioning::ClockedInfo;
use crate::NBSlice as Slice;
use crate::NBSolve as Solve;
use crate::NBStrongComponent as StrongComponent;
use crate::NBStrongComponent::AliasInfo;
use crate::NBTearing as Tearing;
use crate::NBVariable as BVariable;
use crate::NBVariable::VariablePointers;
use crate::NBackendDAE as BackendDAE;
use crate::NSimCode as SimCode;
use crate::NSimCode::Identifier;
use crate::NSimCode::SimCodeIndices;
use crate::NSimGenericCall::SimIterator;
use crate::NSimJacobian::SimJacobian;
use crate::NSimPartition as SimPartition;
use crate::NSimVar::SimVar;
use crate::NSimVar::SimVars;
use crate::NSimVar::VarType;
use openmodelica_ast::Absyn;
use openmodelica_backend_types::BackendDAE as OldBackendDAE;
use openmodelica_frontend_dump::AbsynUtil;
use openmodelica_frontend_types::DAE;
use openmodelica_nf_frontend::NFBackendExtension as BackendExtension;
use openmodelica_nf_frontend::NFComponentRef as ComponentRef;
use openmodelica_nf_frontend::NFConvertDAE as ConvertDAE;
use openmodelica_nf_frontend::NFExpression as Expression;
use openmodelica_nf_frontend::NFFunction::Function;
use openmodelica_nf_frontend::NFInstNode::InstNode;
use openmodelica_nf_frontend::NFOperator as Operator;
use openmodelica_nf_frontend::NFScalarize as Scalarize;
use openmodelica_nf_frontend::NFStatement as Statement;
use openmodelica_nf_frontend::NFSubscript as Subscript;
use openmodelica_nf_frontend::NFType as Type;
use openmodelica_nf_frontend::NFVariable as Variable;
use openmodelica_simcode_types::SimCode as OldSimCode;
use openmodelica_util::Error;
use openmodelica_util::ExpandableArray;
use openmodelica_util::StringUtil;
use openmodelica_util::UnorderedMap;
use openmodelica_util::Util;
use openmodelica_util_datatypes_basic::List;
use openmodelica_util_datatypes_basic::Pointer;

// OF imports
// NF imports
// old backend imports
// Backend imports
// Old SimCode imports
// SimCode imports
// Util imports
pub mod Block {
    use super::*;
    /// A single blck from BLT transformation.
    #[derive(Clone, Debug, Eq, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
    pub enum Block {
        /// Single residual equation of the form
        ///      0 = exp
        RESIDUAL {
            index: i32,
            res_index: i32,
            exp: metamodelica::Ref<Expression::NFExpression>,
            source: metamodelica::Ref<DAE::ElementSource>,
            attr: metamodelica::Ref<EquationAttributes::EquationAttributes>,
        },
        /// Single residual array equation of the form
        ///      0 = exp. Structurally equal to RESIDUAL, but the destinction is important
        ///      for code generation.
        ARRAY_RESIDUAL {
            index: i32,
            res_index: i32,
            exp: metamodelica::Ref<Expression::NFExpression>,
            source: metamodelica::Ref<DAE::ElementSource>,
            attr: metamodelica::Ref<EquationAttributes::EquationAttributes>,
        },
        /// for-loop residual equation of the form
        ///      for {i in 1:n, j in 1:m, ...} loop
        ///        0 = exp;
        ///      end for;
        FOR_RESIDUAL {
            index: i32,
            res_index: i32,
            iterators: metamodelica::List<metamodelica::Ref<SimIterator::SimIterator>>,
            exp: metamodelica::Ref<Expression::NFExpression>,
            source: metamodelica::Ref<DAE::ElementSource>,
            attr: metamodelica::Ref<EquationAttributes::EquationAttributes>,
        },
        /// a generic residual calling a for loop body function with an index list.
        GENERIC_RESIDUAL {
            index: i32,
            res_index: i32,
            scal_indices: metamodelica::List<i32>,
            iterators: metamodelica::List<metamodelica::Ref<SimIterator::SimIterator>>,
            exp: metamodelica::Ref<Expression::NFExpression>,
            source: metamodelica::Ref<DAE::ElementSource>,
            attr: metamodelica::Ref<EquationAttributes::EquationAttributes>,
        },
        /// Simple assignment or solved inner equation of (casual) tearing set
        ///      (Dynamic Tearing) with constraints on the solvability
        ///      lhs := rhs
        SIMPLE_ASSIGN {
            index: i32,
            /// left hand side of equation
            lhs: metamodelica::Ref<ComponentRef::NFComponentRef>,
            rhs: metamodelica::Ref<Expression::NFExpression>,
            source: metamodelica::Ref<DAE::ElementSource>,
            attr: metamodelica::Ref<EquationAttributes::EquationAttributes>,
        },
        /// Array assignment where the left hand side can be an array constructor.
        ///      {a, b, ...} := rhs
        ARRAY_ASSIGN {
            index: i32,
            lhs: metamodelica::Ref<Expression::NFExpression>,
            rhs: metamodelica::Ref<Expression::NFExpression>,
            source: metamodelica::Ref<DAE::ElementSource>,
            attr: metamodelica::Ref<EquationAttributes::EquationAttributes>,
        },
        /// a resizable assignment calling a for loop body function.
        RESIZABLE_ASSIGN {
            index: i32,
            call_index: i32,
            iters: metamodelica::List<metamodelica::Ref<SimIterator::SimIterator>>,
            source: metamodelica::Ref<DAE::ElementSource>,
            attr: metamodelica::Ref<EquationAttributes::EquationAttributes>,
        },
        /// a generic assignment calling a for loop body function with an index list.
        GENERIC_ASSIGN {
            index: i32,
            call_index: i32,
            scal_indices: metamodelica::List<i32>,
            source: metamodelica::Ref<DAE::ElementSource>,
            attr: metamodelica::Ref<EquationAttributes::EquationAttributes>,
        },
        /// entwined generic assignments calling for loop body functions with an index list and a call order.
        ENTWINED_ASSIGN {
            index: i32,
            call_order: metamodelica::List<i32>,
            single_calls: metamodelica::List<metamodelica::Ref<Block>>,
            source: metamodelica::Ref<DAE::ElementSource>,
            attr: metamodelica::Ref<EquationAttributes::EquationAttributes>,
        },
        /// Simple alias assignment pointing to the alias equation.
        ///      - alias of will be -1 at the point of creation and computed afterwards
        ALIAS {
            index: i32,
            /// backend alias info
            aliasInfo: metamodelica::Ref<AliasInfo::AliasInfo>,
            /// final alias index
            aliasOf: i32,
            isDiscrete: bool,
        },
        /// An algorithm section.
        ALGORITHM {
            index: i32,
            stmts: metamodelica::List<metamodelica::Ref<Statement::NFStatement>>,
            attr: metamodelica::Ref<EquationAttributes::EquationAttributes>,
        },
        /// An algorithm section that had to be inverted.
        INVERSE_ALGORITHM {
            index: i32,
            stmts: metamodelica::List<metamodelica::Ref<Statement::NFStatement>>,
            /// this is a subset of output crefs of the original algorithm, which are already known
            knownOutputs: metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>>,
            insideNonLinearSystem: bool,
            attr: metamodelica::Ref<EquationAttributes::EquationAttributes>,
        },
        /// An if section.
        IF {
            index: i32,
            branches: metamodelica::List<(
                metamodelica::Ref<Expression::NFExpression>,
                metamodelica::List<metamodelica::Ref<Block>>,
            )>,
            source: metamodelica::Ref<DAE::ElementSource>,
            attr: metamodelica::Ref<EquationAttributes::EquationAttributes>,
        },
        /// A when section.
        WHEN {
            index: i32,
            /// true, if top-level branch with initial()
            initialCall: bool,
            conditions: metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>>,
            when_stmts: metamodelica::List<metamodelica::Ref<WhenStatement::WhenStatement>>,
            else_when: Option<metamodelica::Ref<Block>>,
            source: metamodelica::Ref<DAE::ElementSource>,
            attr: metamodelica::Ref<EquationAttributes::EquationAttributes>,
        },
        /// Linear algebraic loop.
        LINEAR {
            system: metamodelica::Ref<LinearSystem::LinearSystem>,
            alternativeTearing: Option<metamodelica::Ref<LinearSystem::LinearSystem>>,
        },
        /// Nonlinear algebraic loop.
        NONLINEAR {
            system: metamodelica::Ref<NonlinearSystem::NonlinearSystem>,
            alternativeTearing: Option<metamodelica::Ref<NonlinearSystem::NonlinearSystem>>,
        },
        /// Hybrid system containing both continuous and discrete equations.
        HYBRID {
            index: i32,
            continuous: metamodelica::Ref<Block>,
            discreteVars: metamodelica::List<metamodelica::Ref<SimVar::SimVar>>,
            discreteEqs: metamodelica::List<metamodelica::Ref<Block>>,
            indexHybridSystem: i32,
        },
    }
    impl metamodelica::gc::MMTrace for Block {
        fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
            match self {
                Block::RESIDUAL {
                    index,
                    res_index,
                    exp,
                    source,
                    attr,
                } => {
                    metamodelica::gc::MMTrace::mm_accept(index, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(res_index, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(exp, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(source, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(attr, __mmv)?;
                    Ok(())
                }
                Block::ARRAY_RESIDUAL {
                    index,
                    res_index,
                    exp,
                    source,
                    attr,
                } => {
                    metamodelica::gc::MMTrace::mm_accept(index, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(res_index, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(exp, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(source, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(attr, __mmv)?;
                    Ok(())
                }
                Block::FOR_RESIDUAL {
                    index,
                    res_index,
                    iterators,
                    exp,
                    source,
                    attr,
                } => {
                    metamodelica::gc::MMTrace::mm_accept(index, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(res_index, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(iterators, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(exp, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(source, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(attr, __mmv)?;
                    Ok(())
                }
                Block::GENERIC_RESIDUAL {
                    index,
                    res_index,
                    scal_indices,
                    iterators,
                    exp,
                    source,
                    attr,
                } => {
                    metamodelica::gc::MMTrace::mm_accept(index, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(res_index, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(scal_indices, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(iterators, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(exp, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(source, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(attr, __mmv)?;
                    Ok(())
                }
                Block::SIMPLE_ASSIGN {
                    index,
                    lhs,
                    rhs,
                    source,
                    attr,
                } => {
                    metamodelica::gc::MMTrace::mm_accept(index, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(lhs, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(rhs, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(source, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(attr, __mmv)?;
                    Ok(())
                }
                Block::ARRAY_ASSIGN {
                    index,
                    lhs,
                    rhs,
                    source,
                    attr,
                } => {
                    metamodelica::gc::MMTrace::mm_accept(index, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(lhs, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(rhs, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(source, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(attr, __mmv)?;
                    Ok(())
                }
                Block::RESIZABLE_ASSIGN {
                    index,
                    call_index,
                    iters,
                    source,
                    attr,
                } => {
                    metamodelica::gc::MMTrace::mm_accept(index, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(call_index, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(iters, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(source, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(attr, __mmv)?;
                    Ok(())
                }
                Block::GENERIC_ASSIGN {
                    index,
                    call_index,
                    scal_indices,
                    source,
                    attr,
                } => {
                    metamodelica::gc::MMTrace::mm_accept(index, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(call_index, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(scal_indices, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(source, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(attr, __mmv)?;
                    Ok(())
                }
                Block::ENTWINED_ASSIGN {
                    index,
                    call_order,
                    single_calls,
                    source,
                    attr,
                } => {
                    metamodelica::gc::MMTrace::mm_accept(index, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(call_order, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(single_calls, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(source, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(attr, __mmv)?;
                    Ok(())
                }
                Block::ALIAS {
                    index,
                    aliasInfo,
                    aliasOf,
                    isDiscrete,
                } => {
                    metamodelica::gc::MMTrace::mm_accept(index, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(aliasInfo, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(aliasOf, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(isDiscrete, __mmv)?;
                    Ok(())
                }
                Block::ALGORITHM { index, stmts, attr } => {
                    metamodelica::gc::MMTrace::mm_accept(index, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(stmts, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(attr, __mmv)?;
                    Ok(())
                }
                Block::INVERSE_ALGORITHM {
                    index,
                    stmts,
                    knownOutputs,
                    insideNonLinearSystem,
                    attr,
                } => {
                    metamodelica::gc::MMTrace::mm_accept(index, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(stmts, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(knownOutputs, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(insideNonLinearSystem, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(attr, __mmv)?;
                    Ok(())
                }
                Block::IF {
                    index,
                    branches,
                    source,
                    attr,
                } => {
                    metamodelica::gc::MMTrace::mm_accept(index, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(branches, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(source, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(attr, __mmv)?;
                    Ok(())
                }
                Block::WHEN {
                    index,
                    initialCall,
                    conditions,
                    when_stmts,
                    else_when,
                    source,
                    attr,
                } => {
                    metamodelica::gc::MMTrace::mm_accept(index, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(initialCall, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(conditions, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(when_stmts, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(else_when, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(source, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(attr, __mmv)?;
                    Ok(())
                }
                Block::LINEAR {
                    system,
                    alternativeTearing,
                } => {
                    metamodelica::gc::MMTrace::mm_accept(system, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(alternativeTearing, __mmv)?;
                    Ok(())
                }
                Block::NONLINEAR {
                    system,
                    alternativeTearing,
                } => {
                    metamodelica::gc::MMTrace::mm_accept(system, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(alternativeTearing, __mmv)?;
                    Ok(())
                }
                Block::HYBRID {
                    index,
                    continuous,
                    discreteVars,
                    discreteEqs,
                    indexHybridSystem,
                } => {
                    metamodelica::gc::MMTrace::mm_accept(index, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(continuous, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(discreteVars, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(discreteEqs, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(indexHybridSystem, __mmv)?;
                    Ok(())
                }
            }
        }
    }
    impl Default for Block {
        fn default() -> Self {
            Self::LINEAR {
                system: Default::default(),
                alternativeTearing: Default::default(),
            }
        }
    }
    pub use self::Block::{
        ALGORITHM, ALIAS, ARRAY_ASSIGN, ARRAY_RESIDUAL, ENTWINED_ASSIGN, FOR_RESIDUAL, GENERIC_ASSIGN,
        GENERIC_RESIDUAL, HYBRID, IF, INVERSE_ALGORITHM, LINEAR, NONLINEAR, RESIDUAL, RESIZABLE_ASSIGN, SIMPLE_ASSIGN,
        WHEN,
    };
    pub(crate) fn toString(mut blck: &metamodelica::Ref<Block>, mut r#str: ArcStr) -> Result<ArcStr> {
        let mut r#str: ArcStr = r#str;
        r#str = (match &**blck {
            RESIDUAL {
                exp: __blck_exp,
                index: __blck_index,
                ..
            } => {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*r#str);
                __mm_s.push_str(&*literal!("("));
                __mm_s.push_str(&*intString(__blck_index.clone()));
                __mm_s.push_str(&*literal!(") 0 = "));
                __mm_s.push_str(&*Expression::toString(__blck_exp.clone())?);
                __mm_s.push_str(&*literal!("\n"));
                ArcStr::from(__mm_s)
            }
            ARRAY_RESIDUAL {
                exp: __blck_exp,
                index: __blck_index,
                ..
            } => {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*r#str);
                __mm_s.push_str(&*literal!("("));
                __mm_s.push_str(&*intString(__blck_index.clone()));
                __mm_s.push_str(&*literal!(") 0 = "));
                __mm_s.push_str(&*Expression::toString(__blck_exp.clone())?);
                __mm_s.push_str(&*literal!("\n"));
                ArcStr::from(__mm_s)
            }
            FOR_RESIDUAL {
                exp: __blck_exp,
                index: __blck_index,
                iterators: __blck_iterators,
                ..
            } => {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*r#str);
                __mm_s.push_str(&*literal!("("));
                __mm_s.push_str(&*intString(__blck_index.clone()));
                __mm_s.push_str(&*literal!(") For-Loop-Residual:\n"));
                __mm_s.push_str(&*r#str);
                __mm_s.push_str(&*literal!("for "));
                __mm_s.push_str(&*List::toString(
                    __blck_iterators.clone(),
                    &move |__a0: metamodelica::Ref<SimIterator::SimIterator>| forTplStr(&__a0),
                    List::Style::FLAT_CURLY.clone(),
                )?);
                __mm_s.push_str(&*literal!(" loop\n"));
                __mm_s.push_str(&*r#str);
                __mm_s.push_str(&*literal!("  0 = "));
                __mm_s.push_str(&*Expression::toString(__blck_exp.clone())?);
                __mm_s.push_str(&*literal!(";\n"));
                __mm_s.push_str(&*r#str);
                __mm_s.push_str(&*literal!("end for;\n"));
                ArcStr::from(__mm_s)
            }
            GENERIC_RESIDUAL {
                exp: __blck_exp,
                index: __blck_index,
                iterators: __blck_iterators,
                scal_indices: __blck_scal_indices,
                ..
            } => {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*r#str);
                __mm_s.push_str(&*literal!("("));
                __mm_s.push_str(&*intString(__blck_index.clone()));
                __mm_s.push_str(&*literal!(") Generic For-Loop-Residual:\n"));
                __mm_s.push_str(&*r#str);
                __mm_s.push_str(&*List::toStringCustom(
                    __blck_scal_indices.clone(),
                    &fnptr!(intString, i32),
                    literal!("slice"),
                    literal!("{"),
                    literal!(", "),
                    literal!("}"),
                    true,
                    10,
                )?);
                __mm_s.push_str(&*literal!("\n"));
                __mm_s.push_str(&*r#str);
                __mm_s.push_str(&*literal!("for "));
                __mm_s.push_str(&*List::toString(
                    __blck_iterators.clone(),
                    &move |__a0: metamodelica::Ref<SimIterator::SimIterator>| forTplStr(&__a0),
                    List::Style::FLAT_CURLY.clone(),
                )?);
                __mm_s.push_str(&*literal!(" loop\n"));
                __mm_s.push_str(&*r#str);
                __mm_s.push_str(&*literal!("  0 = "));
                __mm_s.push_str(&*Expression::toString(__blck_exp.clone())?);
                __mm_s.push_str(&*literal!(";\n"));
                __mm_s.push_str(&*r#str);
                __mm_s.push_str(&*literal!("end for;\n"));
                ArcStr::from(__mm_s)
            }
            SIMPLE_ASSIGN {
                index: __blck_index,
                lhs: __blck_lhs,
                rhs: __blck_rhs,
                ..
            } => {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*r#str);
                __mm_s.push_str(&*literal!("("));
                __mm_s.push_str(&*intString(__blck_index.clone()));
                __mm_s.push_str(&*literal!(") "));
                __mm_s.push_str(&*ComponentRef::toString(metamodelica::AsArg::as_arg(&__blck_lhs))?);
                __mm_s.push_str(&*literal!(" := "));
                __mm_s.push_str(&*Expression::toString(__blck_rhs.clone())?);
                __mm_s.push_str(&*literal!("\n"));
                ArcStr::from(__mm_s)
            }
            ARRAY_ASSIGN {
                index: __blck_index,
                lhs: __blck_lhs,
                rhs: __blck_rhs,
                ..
            } => {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*r#str);
                __mm_s.push_str(&*literal!("("));
                __mm_s.push_str(&*intString(__blck_index.clone()));
                __mm_s.push_str(&*literal!(") "));
                __mm_s.push_str(&*Expression::toString(__blck_lhs.clone())?);
                __mm_s.push_str(&*literal!(" := "));
                __mm_s.push_str(&*Expression::toString(__blck_rhs.clone())?);
                __mm_s.push_str(&*literal!("\n"));
                ArcStr::from(__mm_s)
            }
            RESIZABLE_ASSIGN {
                call_index: __blck_call_index,
                index: __blck_index,
                ..
            } => {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*r#str);
                __mm_s.push_str(&*literal!("("));
                __mm_s.push_str(&*intString(__blck_index.clone()));
                __mm_s.push_str(&*literal!(") "));
                __mm_s.push_str(&*literal!("resizable call [index  "));
                __mm_s.push_str(&*intString(__blck_call_index.clone()));
                __mm_s.push_str(&*literal!("]\n"));
                ArcStr::from(__mm_s)
            }
            GENERIC_ASSIGN {
                call_index: __blck_call_index,
                index: __blck_index,
                scal_indices: __blck_scal_indices,
                ..
            } => {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*r#str);
                __mm_s.push_str(&*literal!("("));
                __mm_s.push_str(&*intString(__blck_index.clone()));
                __mm_s.push_str(&*literal!(") "));
                __mm_s.push_str(&*literal!("single generic call [index  "));
                __mm_s.push_str(&*intString(__blck_call_index.clone()));
                __mm_s.push_str(&*literal!("] "));
                __mm_s.push_str(&*List::toStringCustom(
                    __blck_scal_indices.clone(),
                    &fnptr!(intString, i32),
                    literal!(""),
                    literal!("{"),
                    literal!(", "),
                    literal!("}"),
                    true,
                    10,
                )?);
                __mm_s.push_str(&*literal!("\n"));
                ArcStr::from(__mm_s)
            }
            ENTWINED_ASSIGN {
                index: __blck_index,
                single_calls: __blck_single_calls,
                ..
            } => {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*r#str);
                __mm_s.push_str(&*List::toStringCustom(
                    __blck_single_calls.clone(),
                    &({
                        let __pe_b1 = literal!("");
                        move |__pe_a0| toString(&__pe_a0, __pe_b1.clone())
                    }),
                    {
                        let mut __mm_s = String::new();
                        __mm_s.push_str(&*literal!("### entwined call ("));
                        __mm_s.push_str(&*intString(__blck_index.clone()));
                        __mm_s.push_str(&*literal!(") ###"));
                        ArcStr::from(__mm_s)
                    },
                    literal!("\n    "),
                    literal!("    "),
                    literal!(""),
                    true,
                    0,
                )?);
                ArcStr::from(__mm_s)
            }
            ALIAS {
                aliasOf: __blck_aliasOf,
                index: __blck_index,
                ..
            } => {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*r#str);
                __mm_s.push_str(&*literal!("("));
                __mm_s.push_str(&*intString(__blck_index.clone()));
                __mm_s.push_str(&*literal!(") Alias of "));
                __mm_s.push_str(&*intString(__blck_aliasOf.clone()));
                __mm_s.push_str(&*literal!("\n"));
                ArcStr::from(__mm_s)
            }
            ALGORITHM {
                index: __blck_index,
                stmts: __blck_stmts,
                ..
            } => {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*r#str);
                __mm_s.push_str(&*literal!("("));
                __mm_s.push_str(&*intString(__blck_index.clone()));
                __mm_s.push_str(&*literal!(") Algorithm\n"));
                __mm_s.push_str(&*Statement::toStringList(
                    metamodelica::AsArg::as_arg(&__blck_stmts),
                    r#str,
                )?);
                ArcStr::from(__mm_s)
            }
            INVERSE_ALGORITHM {
                index: __blck_index,
                stmts: __blck_stmts,
                ..
            } => {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*r#str);
                __mm_s.push_str(&*literal!("("));
                __mm_s.push_str(&*intString(__blck_index.clone()));
                __mm_s.push_str(&*literal!(") Inverse Algorithm\n"));
                __mm_s.push_str(&*Statement::toStringList(
                    metamodelica::AsArg::as_arg(&__blck_stmts),
                    r#str,
                )?);
                __mm_s.push_str(&*literal!("\n"));
                ArcStr::from(__mm_s)
            }
            IF {
                branches: __blck_branches,
                index: __blck_index,
                ..
            } => {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*r#str);
                __mm_s.push_str(&*literal!("("));
                __mm_s.push_str(&*intString(__blck_index.clone()));
                __mm_s.push_str(&*literal!(") "));
                __mm_s.push_str(&*List::toStringCustom(
                    __blck_branches.clone(),
                    &({
                        let __pe_b1 = r#str.clone();
                        move |__pe_a0| ifTplStr(&__pe_a0, __pe_b1.clone())
                    }),
                    literal!(""),
                    r#str.clone(),
                    {
                        let mut __mm_s = String::new();
                        __mm_s.push_str(&*r#str);
                        __mm_s.push_str(&*literal!("else "));
                        ArcStr::from(__mm_s)
                    },
                    {
                        let mut __mm_s = String::new();
                        __mm_s.push_str(&*r#str);
                        __mm_s.push_str(&*literal!("end if;\n"));
                        ArcStr::from(__mm_s)
                    },
                    true,
                    0,
                )?);
                ArcStr::from(__mm_s)
            }
            WHEN {
                conditions: __blck_conditions,
                else_when: __blck_else_when,
                index: __blck_index,
                when_stmts: __blck_when_stmts,
                ..
            } => {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*r#str);
                __mm_s.push_str(&*literal!("("));
                __mm_s.push_str(&*intString(__blck_index.clone()));
                __mm_s.push_str(&*literal!(") "));
                __mm_s.push_str(&*whenString(
                    __blck_conditions.clone(),
                    __blck_when_stmts.clone(),
                    __blck_else_when.clone(),
                    r#str,
                )?);
                ArcStr::from(__mm_s)
            }
            LINEAR {
                system: __blck_system, ..
            } => {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*r#str);
                __mm_s.push_str(&*literal!("("));
                __mm_s.push_str(&*intString(__blck_system.index.clone()));
                __mm_s.push_str(&*literal!(") "));
                __mm_s.push_str(&*LinearSystem::toString(
                    metamodelica::AsArg::as_arg(&__blck_system),
                    r#str,
                )?);
                ArcStr::from(__mm_s)
            }
            NONLINEAR {
                system: __blck_system, ..
            } => {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*r#str);
                __mm_s.push_str(&*literal!("("));
                __mm_s.push_str(&*intString(__blck_system.index.clone()));
                __mm_s.push_str(&*literal!(") "));
                __mm_s.push_str(&*NonlinearSystem::toString(
                    metamodelica::AsArg::as_arg(&__blck_system),
                    r#str,
                )?);
                ArcStr::from(__mm_s)
            }
            HYBRID {
                index: __blck_index, ..
            } => {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*r#str);
                __mm_s.push_str(&*literal!("("));
                __mm_s.push_str(&*intString(__blck_index.clone()));
                __mm_s.push_str(&*literal!(") Hybrid\n"));
                ArcStr::from(__mm_s)
            }
            _ => {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("NSimStrongComponent.Block.toString"));
                __mm_s.push_str(&*literal!(" failed.\n"));
                ArcStr::from(__mm_s)
            }
        });
        Ok(r#str)
    }

    pub(crate) fn forTplStr(mut iter: &metamodelica::Ref<SimIterator::SimIterator>) -> Result<ArcStr> {
        let mut r#str: ArcStr = SimIterator::toString(iter)?;
        Ok(r#str)
    }

    pub(crate) fn ifTplStr(
        mut tpl: &(
            metamodelica::Ref<Expression::NFExpression>,
            metamodelica::List<metamodelica::Ref<Block>>,
        ),
        mut r#str: ArcStr,
    ) -> Result<ArcStr> {
        let mut r#str: ArcStr = r#str;
        let mut condition: metamodelica::Ref<Expression::NFExpression>;
        let mut blcks: metamodelica::List<metamodelica::Ref<Block>>;
        (condition, blcks) = tpl.clone();
        r#str = {
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("if "));
            __mm_s.push_str(&*Expression::toString(condition)?);
            __mm_s.push_str(&*literal!(" then\n  "));
            __mm_s.push_str(&*List::toString(
                blcks,
                &({
                    let __pe_b1 = {
                        let mut __mm_s = String::new();
                        __mm_s.push_str(&*r#str);
                        __mm_s.push_str(&*literal!("  "));
                        ArcStr::from(__mm_s)
                    };
                    move |__pe_a0| toString(&__pe_a0, __pe_b1.clone())
                }),
                List::Style::NEWLINE.clone(),
            )?);
            ArcStr::from(__mm_s)
        };
        Ok(r#str)
    }

    pub(crate) fn getIndex(mut blck: &metamodelica::Ref<Block>) -> Result<i32> {
        let mut index: i32;
        index = (match &**blck {
            RESIDUAL {
                index: __blck_index, ..
            } => __blck_index.clone(),
            ARRAY_RESIDUAL {
                index: __blck_index, ..
            } => __blck_index.clone(),
            FOR_RESIDUAL {
                index: __blck_index, ..
            } => __blck_index.clone(),
            SIMPLE_ASSIGN {
                index: __blck_index, ..
            } => __blck_index.clone(),
            ARRAY_ASSIGN {
                index: __blck_index, ..
            } => __blck_index.clone(),
            RESIZABLE_ASSIGN {
                index: __blck_index, ..
            } => __blck_index.clone(),
            GENERIC_ASSIGN {
                index: __blck_index, ..
            } => __blck_index.clone(),
            ENTWINED_ASSIGN {
                index: __blck_index, ..
            } => __blck_index.clone(),
            ALIAS {
                index: __blck_index, ..
            } => __blck_index.clone(),
            ALGORITHM {
                index: __blck_index, ..
            } => __blck_index.clone(),
            INVERSE_ALGORITHM {
                index: __blck_index, ..
            } => __blck_index.clone(),
            IF {
                index: __blck_index, ..
            } => __blck_index.clone(),
            WHEN {
                index: __blck_index, ..
            } => __blck_index.clone(),
            LINEAR {
                system: __blck_system, ..
            } => __blck_system.index.clone(),
            NONLINEAR {
                system: __blck_system, ..
            } => __blck_system.index.clone(),
            HYBRID {
                index: __blck_index, ..
            } => __blck_index.clone(),
            _ => {
                Error::addMessage(
                    Error::INTERNAL_ERROR.clone(),
                    list![{
                        let mut __mm_s = String::new();
                        __mm_s.push_str(&*literal!("NSimStrongComponent.Block.getIndex"));
                        __mm_s.push_str(&*literal!(" failed for\n"));
                        __mm_s.push_str(&*toString(blck, literal!(""))?);
                        ArcStr::from(__mm_s)
                    }],
                )?;
                return Err("fail");
            }
        });
        Ok(index)
    }

    pub(crate) fn isDiscrete(mut blck: &metamodelica::Ref<Block>) -> bool {
        let mut b: bool;
        b = (match &**blck {
            RESIDUAL { attr, .. } => attr.kind.clone() == EquationKind::DISCRETE.clone(),
            ARRAY_RESIDUAL { attr, .. } => attr.kind.clone() == EquationKind::DISCRETE.clone(),
            FOR_RESIDUAL { attr, .. } => attr.kind.clone() == EquationKind::DISCRETE.clone(),
            SIMPLE_ASSIGN { attr, .. } => attr.kind.clone() == EquationKind::DISCRETE.clone(),
            ARRAY_ASSIGN { attr, .. } => attr.kind.clone() == EquationKind::DISCRETE.clone(),
            RESIZABLE_ASSIGN { attr, .. } => attr.kind.clone() == EquationKind::DISCRETE.clone(),
            GENERIC_ASSIGN { attr, .. } => attr.kind.clone() == EquationKind::DISCRETE.clone(),
            ENTWINED_ASSIGN { attr, .. } => attr.kind.clone() == EquationKind::DISCRETE.clone(),
            ALIAS {
                isDiscrete: __blck_isDiscrete,
                ..
            } => __blck_isDiscrete.clone(),
            ALGORITHM { attr, .. } => attr.kind.clone() == EquationKind::DISCRETE.clone(),
            INVERSE_ALGORITHM { attr, .. } => attr.kind.clone() == EquationKind::DISCRETE.clone(),
            IF { attr, .. } => attr.kind.clone() == EquationKind::DISCRETE.clone(),
            WHEN { attr, .. } => attr.kind.clone() == EquationKind::DISCRETE.clone(),
            _ => false,
        });
        b
    }

    pub(crate) fn filterWhen(
        mut blcks: metamodelica::List<metamodelica::Ref<Block>>,
        mut out_blcks: metamodelica::List<metamodelica::Ref<Block>>,
        mut new_blcks: metamodelica::List<metamodelica::Ref<Block>>,
        mut indices: SimCodeIndices,
    ) -> Result<(
        metamodelica::List<metamodelica::Ref<Block>>,
        metamodelica::List<metamodelica::Ref<Block>>,
        SimCodeIndices,
    )> {
        '__tco: loop {
            let mut blck: metamodelica::Ref<Block>;
            let mut new_blck: metamodelica::Ref<Block>;
            let mut rest: metamodelica::List<metamodelica::Ref<Block>>;
            let mut stmts: metamodelica::List<metamodelica::Ref<Statement::NFStatement>>;
            ::match_deref::match_deref! { match &(blcks) {
                Deref @ metamodelica::ListNode::Cons { head: Deref @ WHEN { .. }, tail: __esc_rest } => {
                    rest = (*__esc_rest).clone();
                    { (blcks, out_blcks, new_blcks, indices) = (rest.clone(), out_blcks, new_blcks, indices); continue '__tco; }
                },
                Deref @ metamodelica::ListNode::Cons { head: __esc_blck @ Deref @ ALGORITHM { .. }, tail: __esc_rest } => {
                    blck = (*__esc_blck).clone();
                    rest = (*__esc_rest).clone();
                    stmts = Statement::filterDiscrete(var_field!((*blck).stmts, Block::ALGORITHM).clone(), metamodelica::nil())?;
                    if (stmts).is_empty() {
                        (out_blcks, new_blcks, indices) = filterWhen(rest.clone(), out_blcks, new_blcks, indices)?;
                    } else if List::compareLength(stmts.clone(), var_field!((*blck).stmts, Block::ALGORITHM).clone())? != 0 {
                        new_blck = metamodelica::Ref::new(Block::ALGORITHM { index: indices.equationIndex.clone(), stmts: stmts, attr: var_field!((*blck).attr, Block::ALGORITHM).clone() });
                        indices.equationIndex = indices.equationIndex.clone() + 1;
                        (out_blcks, new_blcks, indices) = filterWhen(rest.clone(), metamodelica::cons(new_blck.clone(), out_blcks), metamodelica::cons(new_blck, new_blcks), indices)?;
                    } else {
                        (out_blcks, new_blcks, indices) = filterWhen(rest.clone(), metamodelica::cons(blck.clone(), out_blcks), new_blcks, indices)?;
                    }
                    return Ok((out_blcks, new_blcks, indices))
                },
                Deref @ metamodelica::ListNode::Cons { head: __esc_blck, tail: __esc_rest } => {
                    blck = (*__esc_blck).clone();
                    rest = (*__esc_rest).clone();
                    { (blcks, out_blcks, new_blcks, indices) = (rest.clone(), metamodelica::cons(blck.clone(), out_blcks), new_blcks, indices); continue '__tco; }
                },
                _ => return Ok((out_blcks, new_blcks, indices)),
                _ => return Err("match: no arm matched"),
            } }
        }
    }

    pub(crate) fn map(
        mut blck: metamodelica::Ref<Block>,
        mut func: Arc<
            dyn ::std::ops::Fn(
                    metamodelica::Ref<Expression::NFExpression>,
                ) -> Result<metamodelica::Ref<Expression::NFExpression>>
                + 'static,
        >,
    ) -> Result<metamodelica::Ref<Block>> {
        pub type expFunc = std::sync::Arc<
            dyn ::std::ops::Fn(
                    metamodelica::Ref<Expression::NFExpression>,
                ) -> Result<metamodelica::Ref<Expression::NFExpression>>
                + 'static,
        >;

        let mut blck: metamodelica::Ref<Block> = blck;
        blck = (match &*blck {
            RESIDUAL { exp: __blck_exp, .. } => {
                assign_variant_field!(blck => Block::RESIDUAL; exp = Expression::map(__blck_exp.clone(), func.clone())?);
                blck
            }
            SIMPLE_ASSIGN { rhs: __blck_rhs, .. } => {
                assign_variant_field!(blck => Block::SIMPLE_ASSIGN; rhs = Expression::map(__blck_rhs.clone(), func.clone())?);
                blck
            }
            _ => blck,
        });
        Ok(blck)
    }

    pub(crate) fn listToString(
        mut blcks: &metamodelica::List<metamodelica::Ref<Block>>,
        mut r#str: ArcStr,
        mut header: &ArcStr,
    ) -> Result<ArcStr> {
        let mut r#str: ArcStr = r#str;
        let mut indent: ArcStr = r#str.clone();
        r#str = if (!metamodelica::stringEq(&header, &(literal!("")))) {
            StringUtil::headline_3(header)?
        } else {
            literal!("")
        };
        for mut blck in &**blcks {
            r#str = {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*r#str);
                __mm_s.push_str(&*toString(metamodelica::AsArg::as_arg(&blck), indent.clone())?);
                ArcStr::from(__mm_s)
            };
        }
        Ok(r#str)
    }

    pub(crate) fn createBlocks(
        mut partitions: &metamodelica::List<metamodelica::Ref<Partition::Partition::Partition>>,
        mut all_blcks: metamodelica::List<metamodelica::Ref<Block>>,
        mut simCodeIndices: SimCodeIndices,
        mut simcode_map: metamodelica::Ref<
            UnorderedMap::UnorderedMap<
                metamodelica::Ref<ComponentRef::NFComponentRef>,
                metamodelica::Ref<SimVar::SimVar>,
            >,
        >,
        mut equation_map: metamodelica::Ref<
            UnorderedMap::UnorderedMap<metamodelica::Ref<ComponentRef::NFComponentRef>, metamodelica::Ref<Block>>,
        >,
    ) -> Result<(
        metamodelica::List<metamodelica::List<metamodelica::Ref<Block>>>,
        metamodelica::List<metamodelica::Ref<Block>>,
        SimCodeIndices,
    )> {
        let mut blcks: metamodelica::List<metamodelica::List<metamodelica::Ref<Block>>> = metamodelica::nil();
        let mut all_blcks: metamodelica::List<metamodelica::Ref<Block>> = all_blcks;
        let mut simCodeIndices: SimCodeIndices = simCodeIndices;
        let mut tmp: metamodelica::List<metamodelica::Ref<Block>>;
        for mut partition in &**partitions {
            (tmp, simCodeIndices) = fromPartition(
                metamodelica::AsArg::as_arg(&partition),
                simCodeIndices,
                simcode_map.clone(),
                equation_map.clone(),
            )?;
            blcks = metamodelica::cons(tmp.clone(), blcks);
            all_blcks = listAppend(tmp, all_blcks);
        }
        blcks = blcks.reverse();
        Ok((blcks, all_blcks, simCodeIndices))
    }

    pub(crate) fn createDiscreteBlocks(
        mut partitions: &metamodelica::List<metamodelica::Ref<Partition::Partition::Partition>>,
        mut blcks: metamodelica::List<metamodelica::List<metamodelica::Ref<Block>>>,
        mut all_blcks: metamodelica::List<metamodelica::Ref<Block>>,
        mut event_dependencies: metamodelica::List<metamodelica::Ref<Block>>,
        mut simCodeIndices: SimCodeIndices,
        mut simcode_map: metamodelica::Ref<
            UnorderedMap::UnorderedMap<
                metamodelica::Ref<ComponentRef::NFComponentRef>,
                metamodelica::Ref<SimVar::SimVar>,
            >,
        >,
        mut equation_map: metamodelica::Ref<
            UnorderedMap::UnorderedMap<metamodelica::Ref<ComponentRef::NFComponentRef>, metamodelica::Ref<Block>>,
        >,
    ) -> Result<(
        metamodelica::List<metamodelica::List<metamodelica::Ref<Block>>>,
        metamodelica::List<metamodelica::Ref<Block>>,
        metamodelica::List<metamodelica::Ref<Block>>,
        SimCodeIndices,
    )> {
        let mut blcks: metamodelica::List<metamodelica::List<metamodelica::Ref<Block>>> = blcks;
        let mut all_blcks: metamodelica::List<metamodelica::Ref<Block>> = all_blcks;
        let mut event_dependencies: metamodelica::List<metamodelica::Ref<Block>> = event_dependencies;
        let mut simCodeIndices: SimCodeIndices = simCodeIndices;
        let mut tmp: metamodelica::List<metamodelica::Ref<Block>>;
        let mut new_blcks: metamodelica::List<metamodelica::Ref<Block>>;
        for mut partition in &**partitions {
            (tmp, simCodeIndices) = fromPartition(
                metamodelica::AsArg::as_arg(&partition),
                simCodeIndices,
                simcode_map.clone(),
                equation_map.clone(),
            )?;
            all_blcks = listAppend(tmp.clone(), all_blcks);
            (tmp, new_blcks, simCodeIndices) =
                filterWhen(tmp.reverse(), metamodelica::nil(), metamodelica::nil(), simCodeIndices)?;
            all_blcks = listAppend(new_blcks, all_blcks);
            blcks = metamodelica::cons(tmp.clone(), blcks);
            tmp = ({
                let mut __acc: metamodelica::List<metamodelica::Ref<Block>> = metamodelica::nil();
                for mut blck in (tmp).into_iter().cloned() {
                    if !(!(isDiscrete(&(blck.clone())))) {
                        continue;
                    }
                    let __x = blck.clone();
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            });
            event_dependencies = listAppend(tmp, event_dependencies);
        }
        blcks = blcks.reverse();
        Ok((blcks, all_blcks, event_dependencies, simCodeIndices))
    }

    pub(crate) fn createInitialBlocks(
        mut partitions: &metamodelica::List<metamodelica::Ref<Partition::Partition::Partition>>,
        mut simCodeIndices: SimCodeIndices,
        mut simcode_map: metamodelica::Ref<
            UnorderedMap::UnorderedMap<
                metamodelica::Ref<ComponentRef::NFComponentRef>,
                metamodelica::Ref<SimVar::SimVar>,
            >,
        >,
        mut equation_map: metamodelica::Ref<
            UnorderedMap::UnorderedMap<metamodelica::Ref<ComponentRef::NFComponentRef>, metamodelica::Ref<Block>>,
        >,
    ) -> Result<(metamodelica::List<metamodelica::Ref<Block>>, SimCodeIndices)> {
        let mut blcks: metamodelica::List<metamodelica::Ref<Block>>;
        let mut simCodeIndices: SimCodeIndices = simCodeIndices;
        let mut tmp: metamodelica::List<metamodelica::Ref<Block>>;
        let mut tmp_lst: metamodelica::List<metamodelica::List<metamodelica::Ref<Block>>> = metamodelica::nil();
        for mut partition in &**partitions {
            (tmp, simCodeIndices) = fromPartition(
                metamodelica::AsArg::as_arg(&partition),
                simCodeIndices,
                simcode_map.clone(),
                equation_map.clone(),
            )?;
            tmp_lst = metamodelica::cons(tmp, tmp_lst);
        }
        blcks = List::flatten(tmp_lst)?;
        Ok((blcks, simCodeIndices))
    }

    pub(crate) fn createParameterBlocks(
        mut comps: &metamodelica::List<metamodelica::Ref<StrongComponent::NBStrongComponent>>,
        mut simCodeIndices: SimCodeIndices,
        mut simcode_map: metamodelica::Ref<
            UnorderedMap::UnorderedMap<
                metamodelica::Ref<ComponentRef::NFComponentRef>,
                metamodelica::Ref<SimVar::SimVar>,
            >,
        >,
        mut equation_map: metamodelica::Ref<
            UnorderedMap::UnorderedMap<metamodelica::Ref<ComponentRef::NFComponentRef>, metamodelica::Ref<Block>>,
        >,
    ) -> Result<(metamodelica::List<metamodelica::Ref<Block>>, SimCodeIndices)> {
        let mut blcks: metamodelica::List<metamodelica::Ref<Block>> = metamodelica::nil();
        let mut simCodeIndices: SimCodeIndices = simCodeIndices;
        let mut tmp: metamodelica::Ref<Block>;
        let mut index: i32;
        for mut comp in &**comps {
            (tmp, simCodeIndices, index) = fromStrongComponent(
                metamodelica::AsArg::as_arg(&comp),
                simCodeIndices,
                Partition::Kind::INI.clone(),
                simcode_map.clone(),
                equation_map.clone(),
            )?;
            blcks = metamodelica::cons(tmp, blcks);
        }
        blcks = blcks.reverse();
        Ok((blcks, simCodeIndices))
    }

    pub(crate) fn createAttributeBlocks(
        mut vars: &metamodelica::List<metamodelica::Ref<VariablePointers::VariablePointers>>,
        mut simCodeIndices: SimCodeIndices,
        mut simcode_map: metamodelica::Ref<
            UnorderedMap::UnorderedMap<
                metamodelica::Ref<ComponentRef::NFComponentRef>,
                metamodelica::Ref<SimVar::SimVar>,
            >,
        >,
    ) -> Result<(
        metamodelica::List<metamodelica::Ref<Block>>,
        metamodelica::List<metamodelica::Ref<Block>>,
        metamodelica::List<metamodelica::Ref<Block>>,
        SimCodeIndices,
    )> {
        let mut min_blcks: metamodelica::List<metamodelica::Ref<Block>> = metamodelica::nil();
        let mut max_blcks: metamodelica::List<metamodelica::Ref<Block>> = metamodelica::nil();
        let mut nominal_blcks: metamodelica::List<metamodelica::Ref<Block>> = metamodelica::nil();
        let mut simCodeIndices: SimCodeIndices = simCodeIndices;
        let mut sim_vars: metamodelica::List<metamodelica::Ref<Variable::NFVariable>> = metamodelica::nil();
        let mut var: metamodelica::Ref<Variable::NFVariable> =
            <metamodelica::Ref<Variable::NFVariable> as ::std::default::Default>::default();
        for mut var_ptrs in &**vars {
            for mut var_ptr in &*BVariable::VariablePointers::toList(metamodelica::AsArg::as_arg(&var_ptrs))? {
                var = Pointer::access(var_ptr.clone());
                if UnorderedMap::contains(var.name.clone(), simcode_map.clone())? {
                    sim_vars = metamodelica::cons(var, sim_vars);
                }
            }
        }
        sim_vars = sim_vars.reverse();
        for mut var in &*sim_vars {
            let mut var = var.clone();
            (nominal_blcks, simCodeIndices) = createAttributeBlock(
                &(var.clone()),
                BackendExtension::VariableAttributes::getNominal(&var.backendinfo.attributes)?,
                nominal_blcks,
                simCodeIndices,
            )?;
        }
        for mut var in &*sim_vars {
            let mut var = var.clone();
            (min_blcks, simCodeIndices) = createAttributeBlock(
                &(var.clone()),
                BackendExtension::VariableAttributes::getMin(&var.backendinfo.attributes)?,
                min_blcks,
                simCodeIndices,
            )?;
        }
        for mut var in &*sim_vars {
            let mut var = var.clone();
            (max_blcks, simCodeIndices) = createAttributeBlock(
                &(var.clone()),
                BackendExtension::VariableAttributes::getMax(&var.backendinfo.attributes)?,
                max_blcks,
                simCodeIndices,
            )?;
        }
        min_blcks = min_blcks.reverse();
        max_blcks = max_blcks.reverse();
        nominal_blcks = nominal_blcks.reverse();
        Ok((min_blcks, max_blcks, nominal_blcks, simCodeIndices))
    }

    pub(crate) fn createAttributeBlock(
        mut var: &metamodelica::Ref<Variable::NFVariable>,
        mut attribute: Option<metamodelica::Ref<Expression::NFExpression>>,
        mut blcks: metamodelica::List<metamodelica::Ref<Block>>,
        mut simCodeIndices: SimCodeIndices,
    ) -> Result<(metamodelica::List<metamodelica::Ref<Block>>, SimCodeIndices)> {
        let mut blcks: metamodelica::List<metamodelica::Ref<Block>> = blcks;
        let mut simCodeIndices: SimCodeIndices = simCodeIndices;
        let _ = (::match_deref::match_deref! { match &(attribute) {
            Some(exp) if (!(Expression::isLiteralXML(exp.clone())?)) => {
                blcks = metamodelica::cons(metamodelica::Ref::new(Block::SIMPLE_ASSIGN { index: simCodeIndices.equationIndex.clone(), lhs: var.name.clone(), rhs: exp.clone(), source: DAE::emptyElementSource().clone(), attr: BEquation::default(EquationKind::CONTINUOUS.clone(), false, None, None) }), blcks);
                simCodeIndices.equationIndex = simCodeIndices.equationIndex.clone() + 1;
                ()
            },
            _ => {
                ()
            },
            _ => unreachable!("match_deref! exhaustiveness placeholder"),
        } });
        Ok((blcks, simCodeIndices))
    }

    pub(crate) fn createDAEModeBlocks(
        mut partitions: metamodelica::List<metamodelica::Ref<Partition::Partition::Partition>>,
        mut simCodeIndices: SimCodeIndices,
        mut simcode_map: metamodelica::Ref<
            UnorderedMap::UnorderedMap<
                metamodelica::Ref<ComponentRef::NFComponentRef>,
                metamodelica::Ref<SimVar::SimVar>,
            >,
        >,
        mut equation_map: metamodelica::Ref<
            UnorderedMap::UnorderedMap<metamodelica::Ref<ComponentRef::NFComponentRef>, metamodelica::Ref<Block>>,
        >,
    ) -> Result<(
        metamodelica::List<metamodelica::List<metamodelica::Ref<Block>>>,
        metamodelica::List<metamodelica::Ref<SimVar::SimVar>>,
        SimCodeIndices,
    )> {
        let mut blcks: metamodelica::List<metamodelica::List<metamodelica::Ref<Block>>> = metamodelica::nil();
        let mut vars: metamodelica::List<metamodelica::Ref<SimVar::SimVar>> = metamodelica::nil();
        let mut simCodeIndices: SimCodeIndices = simCodeIndices;
        let mut indices_ptr: Pointer::Pointer<SimCodeIndices>;
        let mut vars_ptr: Pointer::Pointer<metamodelica::List<metamodelica::Ref<SimVar::SimVar>>> =
            Pointer::create(metamodelica::nil());
        let mut tmp: metamodelica::List<metamodelica::Ref<Block>>;
        for mut partition in &*partitions.reverse() {
            indices_ptr = Pointer::create(simCodeIndices);
            Partition::Partition::mapStrongComponents(
                partition.clone(),
                &({
                    let __pe_b1 = vars_ptr.clone();
                    let __pe_b2 = indices_ptr.clone();
                    let __pe_b3 = VarType::RESIDUAL.clone();
                    move |__pe_a0| {
                        SimVar::createFromResidualComponent(__pe_a0, __pe_b1.clone(), __pe_b2.clone(), __pe_b3.clone())
                    }
                }),
            )?;
            (tmp, simCodeIndices) = fromPartition(
                metamodelica::AsArg::as_arg(&partition),
                Pointer::access(indices_ptr),
                simcode_map.clone(),
                equation_map.clone(),
            )?;
            blcks = metamodelica::cons(tmp, blcks);
        }
        vars = Pointer::access(vars_ptr).reverse();
        Ok((blcks, vars, simCodeIndices))
    }

    pub(crate) fn createClockedBlocks(
        mut partitions: metamodelica::List<metamodelica::Ref<Partition::Partition::Partition>>,
        mut simCodeIndices: SimCodeIndices,
        mut simcode_map: metamodelica::Ref<
            UnorderedMap::UnorderedMap<
                metamodelica::Ref<ComponentRef::NFComponentRef>,
                metamodelica::Ref<SimVar::SimVar>,
            >,
        >,
        mut equation_map: metamodelica::Ref<
            UnorderedMap::UnorderedMap<metamodelica::Ref<ComponentRef::NFComponentRef>, metamodelica::Ref<Block>>,
        >,
        mut info: &metamodelica::Ref<ClockedInfo::ClockedInfo>,
    ) -> Result<(
        metamodelica::List<metamodelica::Ref<SimPartition::NSimPartition>>,
        metamodelica::List<metamodelica::Ref<Block>>,
        SimCodeIndices,
    )> {
        pub(crate) type SimPartitions = metamodelica::List<metamodelica::Ref<SimPartition::NSimPartition>>;

        let mut baseParts: metamodelica::List<metamodelica::Ref<SimPartition::NSimPartition>>;
        let mut eventClocks: metamodelica::List<metamodelica::Ref<Block>>;
        let mut simCodeIndices: SimCodeIndices = simCodeIndices;
        let mut clock_collector: metamodelica::Ref<
            UnorderedMap::UnorderedMap<
                metamodelica::Ref<BClock::BClock>,
                metamodelica::List<metamodelica::Ref<SimPartition::NSimPartition>>,
            >,
        > = UnorderedMap::new(
            (std::sync::Arc::new(move |__a0: metamodelica::Ref<BClock::BClock>| Partitioning::BClock::hash(&__a0))
                as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<BClock::BClock>) -> Result<i32> + 'static>),
            (std::sync::Arc::new(
                move |__a0: metamodelica::Ref<BClock::BClock>, __a1: metamodelica::Ref<BClock::BClock>| {
                    Partitioning::BClock::isEqual(&__a0, &__a1)
                },
            )
                as std::sync::Arc<
                    dyn ::std::ops::Fn(
                            metamodelica::Ref<BClock::BClock>,
                            metamodelica::Ref<BClock::BClock>,
                        ) -> Result<bool>
                        + 'static,
                >),
            1,
        );
        let mut blcks: metamodelica::List<metamodelica::Ref<Block>>;
        let mut vars: metamodelica::List<metamodelica::Ref<SimVar::SimVar>>;
        let mut clock: metamodelica::Ref<BClock::BClock>;
        let mut subClock: metamodelica::Ref<BClock::BClock>;
        let mut baseClock: metamodelica::Ref<BClock::BClock>;
        let mut holdEvents: bool;
        let mut baseClock_opt: Option<metamodelica::Ref<BClock::BClock>>;
        let mut subPart: metamodelica::Ref<SimPartition::NSimPartition>;
        for mut c in &*UnorderedMap::valueList(info.baseClocks.clone()) {
            UnorderedMap::add(c.clone(), metamodelica::nil(), clock_collector.clone())?;
        }
        for mut partition in &*partitions.reverse() {
            (blcks, simCodeIndices) = fromPartition(
                metamodelica::AsArg::as_arg(&partition),
                simCodeIndices,
                simcode_map.clone(),
                equation_map.clone(),
            )?;
            vars = SimVars::getPartitionVars(metamodelica::AsArg::as_arg(&partition), simcode_map.clone())?;
            (clock, baseClock_opt, holdEvents) =
                Partition::Partition::getClocks(metamodelica::AsArg::as_arg(&partition))?;
            if (baseClock_opt).is_some() {
                let __pa0 = ::match_deref::match_deref! { match &(baseClock_opt) {
                    Some(__pa0) => __pa0.clone(),
                    _ => return Err("pattern mismatch"),
                } };
                baseClock = metamodelica::Own::own(__pa0);
                subClock = clock;
            } else {
                baseClock = clock;
                subClock = Partitioning::DEFAULT_SUB_CLOCK().clone();
            }
            subPart = SimPartition::createSubPartition(subClock, blcks, vars, holdEvents);
            UnorderedMap::add(
                baseClock.clone(),
                metamodelica::cons(
                    subPart,
                    UnorderedMap::getSafe(
                        baseClock,
                        clock_collector.clone(),
                        metamodelica::sourceInfo!("NSimCode/NSimStrongComponent.mo"),
                    )?,
                ),
                clock_collector.clone(),
            )?;
        }
        (baseParts, eventClocks, simCodeIndices) = SimPartition::createBasePartitions(clock_collector, simCodeIndices);
        Ok((baseParts, eventClocks, simCodeIndices))
    }

    pub(crate) fn createNoReturnBlocks(
        mut equations: &metamodelica::Ref<EquationPointers::EquationPointers>,
        mut simCodeIndices: SimCodeIndices,
        mut kind: Partition::Kind,
        mut simcode_map: metamodelica::Ref<
            UnorderedMap::UnorderedMap<
                metamodelica::Ref<ComponentRef::NFComponentRef>,
                metamodelica::Ref<SimVar::SimVar>,
            >,
        >,
        mut equation_map: metamodelica::Ref<
            UnorderedMap::UnorderedMap<metamodelica::Ref<ComponentRef::NFComponentRef>, metamodelica::Ref<Block>>,
        >,
    ) -> Result<(metamodelica::List<metamodelica::Ref<Block>>, SimCodeIndices)> {
        let mut blcks: metamodelica::List<metamodelica::Ref<Block>> = metamodelica::nil();
        let mut simCodeIndices: SimCodeIndices = simCodeIndices;
        let mut eqn: metamodelica::Ref<Equation::Equation>;
        let mut tmp: metamodelica::Ref<Block>;
        for mut i in 1..=ExpandableArray::getLastUsedIndex(equations.eqArr.clone()) {
            if ExpandableArray::occupied(i, equations.eqArr.clone()) {
                eqn = Pointer::access(ExpandableArray::get(i, equations.eqArr.clone())?);
                (tmp, simCodeIndices) = (::match_deref::match_deref! { match &(eqn.clone()) {
                    Deref @ BEquation::Equation::SCALAR_EQUATION { lhs: Deref @ Expression::CREF { cref, .. }, .. } => {
                        createEquation(BVariable::getVar(metamodelica::AsArg::as_arg(&cref), metamodelica::sourceInfo!("NSimCode/NSimStrongComponent.mo"))?, eqn, Solve::Status::EXPLICIT.clone(), simCodeIndices, kind, simcode_map.clone(), equation_map.clone())?
                    },
                    Deref @ BEquation::Equation::ARRAY_EQUATION { lhs: Deref @ Expression::CREF { cref, .. }, .. } => {
                        createEquation(BVariable::getVar(metamodelica::AsArg::as_arg(&cref), metamodelica::sourceInfo!("NSimCode/NSimStrongComponent.mo"))?, eqn, Solve::Status::EXPLICIT.clone(), simCodeIndices, kind, simcode_map.clone(), equation_map.clone())?
                    },
                    Deref @ BEquation::Equation::RECORD_EQUATION { lhs: Deref @ Expression::CREF { cref, .. }, .. } => {
                        createEquation(BVariable::getVar(metamodelica::AsArg::as_arg(&cref), metamodelica::sourceInfo!("NSimCode/NSimStrongComponent.mo"))?, eqn, Solve::Status::EXPLICIT.clone(), simCodeIndices, kind, simcode_map.clone(), equation_map.clone())?
                    },
                    Deref @ BEquation::Equation::WHEN_EQUATION { .. } => {
                        createEquation(BVariable::DUMMY_VARIABLE().clone(), eqn, Solve::Status::EXPLICIT.clone(), simCodeIndices, kind, simcode_map.clone(), equation_map.clone())?
                    },
                    Deref @ BEquation::Equation::ALGORITHM { .. } => {
                        createAlgorithm(eqn, simCodeIndices, equation_map.clone())?
                    },
                    Deref @ BEquation::Equation::FOR_EQUATION { .. } => {
                        createAlgorithm(eqn, simCodeIndices, equation_map.clone())?
                    },
                    _ => {
                        Error::addMessage(Error::INTERNAL_ERROR.clone(), list![{ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NSimStrongComponent.Block.createNoReturnBlocks")); __mm_s.push_str(&*literal!(" failed for\n")); __mm_s.push_str(&*BEquation::Equation::toString(eqn, literal!(""))?); ArcStr::from(__mm_s) }])?;
                        return Err("fail")
                    },
                    _ => unreachable!("match_deref! exhaustiveness placeholder"),
                } });
                blcks = metamodelica::cons(tmp, blcks);
            }
        }
        Ok((blcks, simCodeIndices))
    }

    pub(crate) fn fromPartition(
        mut partition: &metamodelica::Ref<Partition::Partition::Partition>,
        mut simCodeIndices: SimCodeIndices,
        mut simcode_map: metamodelica::Ref<
            UnorderedMap::UnorderedMap<
                metamodelica::Ref<ComponentRef::NFComponentRef>,
                metamodelica::Ref<SimVar::SimVar>,
            >,
        >,
        mut equation_map: metamodelica::Ref<
            UnorderedMap::UnorderedMap<metamodelica::Ref<ComponentRef::NFComponentRef>, metamodelica::Ref<Block>>,
        >,
    ) -> Result<(metamodelica::List<metamodelica::Ref<Block>>, SimCodeIndices)> {
        let mut blcks: metamodelica::List<metamodelica::Ref<Block>>;
        let mut simCodeIndices: SimCodeIndices = simCodeIndices;
        blcks = ({
            let mut result: metamodelica::List<metamodelica::Ref<Block>> = metamodelica::nil();
            (match partition.strongComponents.clone() {
                Some(mut comps) => {
                    let mut kind: Partition::Kind;
                    let mut tmp: metamodelica::Ref<Block>;
                    let mut index: i32;
                    let mut alias_index: i32;
                    kind = Partition::Partition::getKind(partition);
                    for mut i in ({
                        let __s = metamodelica::arrayLength(comps.clone());
                        let __e = 1;
                        (0i32..)
                            .map(move |__k| __s + __k * (-1))
                            .take_while(move |&__v| __v >= __e)
                    }) {
                        (tmp, simCodeIndices, index) = fromStrongComponent(
                            &({
                                let __elt = (*metamodelica::index_checked(&comps.borrow(), i)?).clone();
                                __elt
                            }),
                            simCodeIndices,
                            kind,
                            simcode_map.clone(),
                            equation_map.clone(),
                        )?;
                        alias_index = (match &*({
                            let __elt = (*metamodelica::index_checked(&comps.borrow(), i)?).clone();
                            __elt
                        }) {
                            StrongComponent::ALIAS { aliasInfo, .. } => {
                                UnorderedMap::getOrDefault(aliasInfo.clone(), simCodeIndices.alias_map.clone(), -1)?
                            }
                            _ => index,
                        });
                        UnorderedMap::add(
                            metamodelica::Ref::new(AliasInfo::AliasInfo {
                                kind: kind,
                                partitionIndex: partition.index.clone(),
                                componentIndex: i,
                            }),
                            alias_index,
                            simCodeIndices.alias_map.clone(),
                        )?;
                        result = metamodelica::cons(tmp, result);
                    }
                    result
                }
                _ => {
                    Error::addMessage(
                        Error::INTERNAL_ERROR.clone(),
                        list![{
                            let mut __mm_s = String::new();
                            __mm_s.push_str(&*literal!("NSimStrongComponent.Block.fromPartition"));
                            __mm_s.push_str(&*literal!(" failed for\n"));
                            __mm_s.push_str(&*Partition::Partition::toString(partition, 0)?);
                            ArcStr::from(__mm_s)
                        }],
                    )?;
                    return Err("fail");
                }
            })
        });
        Ok((blcks, simCodeIndices))
    }

    pub(crate) fn blockSource(mut blck: &metamodelica::Ref<Block>) -> metamodelica::Ref<DAE::ElementSource> {
        let mut source: metamodelica::Ref<DAE::ElementSource>;
        source = (match &**blck {
            RESIDUAL {
                source: __blck_source, ..
            } => __blck_source.clone(),
            ARRAY_RESIDUAL {
                source: __blck_source, ..
            } => __blck_source.clone(),
            FOR_RESIDUAL {
                source: __blck_source, ..
            } => __blck_source.clone(),
            GENERIC_RESIDUAL {
                source: __blck_source, ..
            } => __blck_source.clone(),
            SIMPLE_ASSIGN {
                source: __blck_source, ..
            } => __blck_source.clone(),
            ARRAY_ASSIGN {
                source: __blck_source, ..
            } => __blck_source.clone(),
            RESIZABLE_ASSIGN {
                source: __blck_source, ..
            } => __blck_source.clone(),
            GENERIC_ASSIGN {
                source: __blck_source, ..
            } => __blck_source.clone(),
            ENTWINED_ASSIGN {
                source: __blck_source, ..
            } => __blck_source.clone(),
            IF {
                source: __blck_source, ..
            } => __blck_source.clone(),
            WHEN {
                source: __blck_source, ..
            } => __blck_source.clone(),
            _ => DAE::emptyElementSource().clone(),
        });
        source
    }

    pub(crate) fn jacobianHasGenericLoopCalls(mut jac: &metamodelica::Ref<SimJacobian::SimJacobian>) -> bool {
        let mut b: bool;
        b = (match &**jac {
            SimJacobian::SIM_JAC { .. } => !((jac.generic_loop_calls).is_empty()),
        });
        b
    }

    pub(crate) fn isForOrGenericResidual(mut blck: &metamodelica::Ref<Block>) -> bool {
        let mut b: bool;
        b = (match &**blck {
            FOR_RESIDUAL { .. } => true,
            GENERIC_RESIDUAL { .. } => true,
            _ => false,
        });
        b
    }

    pub(crate) fn fromStrongComponent(
        mut comp: &metamodelica::Ref<StrongComponent::NBStrongComponent>,
        mut simCodeIndices: SimCodeIndices,
        mut kind: Partition::Kind,
        mut simcode_map: metamodelica::Ref<
            UnorderedMap::UnorderedMap<
                metamodelica::Ref<ComponentRef::NFComponentRef>,
                metamodelica::Ref<SimVar::SimVar>,
            >,
        >,
        mut equation_map: metamodelica::Ref<
            UnorderedMap::UnorderedMap<metamodelica::Ref<ComponentRef::NFComponentRef>, metamodelica::Ref<Block>>,
        >,
    ) -> Result<(metamodelica::Ref<Block>, SimCodeIndices, i32)> {
        let mut blck: metamodelica::Ref<Block>;
        let mut simCodeIndices: SimCodeIndices = simCodeIndices;
        let mut index: i32;
        (blck, index) = ({
            let mut eqns: metamodelica::List<metamodelica::Ref<Block>> = metamodelica::nil();
            let mut crefs: metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>> = metamodelica::nil();
            let mut linVars: metamodelica::List<metamodelica::Ref<SimVar::SimVar>> = metamodelica::nil();
            let mut residual_index: i32 = 0;
            let mut single_calls: metamodelica::List<metamodelica::Ref<Block>> = metamodelica::nil();
            let mut call_order: metamodelica::List<i32> = metamodelica::nil();
            (match &**comp {
                StrongComponent::SINGLE_COMPONENT {
                    eqn: __comp_eqn,
                    status: __comp_status,
                    var: __comp_var,
                } => {
                    let mut tmp: metamodelica::Ref<Block>;
                    (tmp, simCodeIndices) = createEquation(
                        Pointer::access(__comp_var.clone()),
                        Pointer::access(__comp_eqn.clone()),
                        __comp_status.clone(),
                        simCodeIndices,
                        kind,
                        simcode_map,
                        equation_map,
                    )?;
                    (tmp.clone(), getIndex(&tmp)?)
                }
                StrongComponent::MULTI_COMPONENT {
                    eqn: __comp_eqn,
                    status: __comp_status,
                    ..
                } => {
                    let mut tmp: metamodelica::Ref<Block>;
                    (tmp, simCodeIndices) = createEquation(
                        BVariable::DUMMY_VARIABLE().clone(),
                        Pointer::access(Slice::getT(__comp_eqn.clone())),
                        __comp_status.clone(),
                        simCodeIndices,
                        kind,
                        simcode_map,
                        equation_map,
                    )?;
                    (tmp.clone(), getIndex(&tmp)?)
                }
                StrongComponent::SLICED_COMPONENT { eqn: __comp_eqn, .. }
                    if (BEquation::Equation::isForEquation(Slice::getT(__comp_eqn.clone()))) =>
                {
                    let mut tmp: metamodelica::Ref<Block>;
                    (tmp, simCodeIndices) = createAlgorithm(
                        Pointer::access(Slice::getT(__comp_eqn.clone())),
                        simCodeIndices,
                        equation_map,
                    )?;
                    (tmp.clone(), getIndex(&tmp)?)
                }
                StrongComponent::SLICED_COMPONENT {
                    eqn: __comp_eqn,
                    status: __comp_status,
                    var_cref: __comp_var_cref,
                    ..
                } => {
                    let mut tmp: metamodelica::Ref<Block>;
                    let mut eqn: metamodelica::Ref<Equation::Equation>;
                    eqn = Pointer::access(Slice::getT(__comp_eqn.clone()));
                    (tmp, simCodeIndices) = createEquation(
                        Variable::fromCref(__comp_var_cref.clone())?,
                        eqn,
                        __comp_status.clone(),
                        simCodeIndices,
                        kind,
                        simcode_map,
                        equation_map,
                    )?;
                    (tmp.clone(), getIndex(&tmp)?)
                }
                StrongComponent::RESIZABLE_COMPONENT {
                    eqn: __comp_eqn,
                    var_cref: __comp_var_cref,
                    ..
                } if (BEquation::Equation::isForEquation(Slice::getT(__comp_eqn.clone()))) => {
                    let mut tmp: metamodelica::Ref<Block>;
                    let mut eqn_ptr: Pointer::Pointer<metamodelica::Ref<Equation::Equation>>;
                    let mut eqn: metamodelica::Ref<Equation::Equation>;
                    let mut generic_call_index: i32;
                    let mut ident: metamodelica::Ref<Identifier::Identifier>;
                    let mut iters: metamodelica::List<metamodelica::Ref<SimIterator::SimIterator>>;
                    eqn_ptr = Slice::getT(__comp_eqn.clone());
                    eqn = Pointer::access(eqn_ptr.clone());
                    ident = metamodelica::Ref::new(Identifier::Identifier {
                        eqn: eqn_ptr.clone(),
                        var_cref: __comp_var_cref.clone(),
                        resizable: true,
                    });
                    iters = SimIterator::fromIterator(&(BEquation::Equation::getForIterator(&eqn)))?;
                    generic_call_index = UnorderedMap::tryAdd(
                        ident,
                        UnorderedMap::size(simCodeIndices.generic_call_map.clone()),
                        simCodeIndices.generic_call_map.clone(),
                    )?;
                    tmp = metamodelica::Ref::new(Block::RESIZABLE_ASSIGN {
                        index: simCodeIndices.equationIndex.clone(),
                        call_index: generic_call_index,
                        iters: iters,
                        source: BEquation::Equation::getSource(eqn.clone()),
                        attr: BEquation::Equation::getAttributes(eqn),
                    });
                    UnorderedMap::add(BEquation::Equation::getEqnName(eqn_ptr)?, tmp.clone(), equation_map)?;
                    simCodeIndices.equationIndex = simCodeIndices.equationIndex.clone() + 1;
                    (tmp.clone(), getIndex(&tmp)?)
                }
                StrongComponent::GENERIC_COMPONENT {
                    eqn: __comp_eqn,
                    var_cref: __comp_var_cref,
                    ..
                } => {
                    let mut tmp: metamodelica::Ref<Block>;
                    let mut eqn_ptr: Pointer::Pointer<metamodelica::Ref<Equation::Equation>>;
                    let mut eqn: metamodelica::Ref<Equation::Equation>;
                    let mut generic_call_index: i32;
                    let mut ident: metamodelica::Ref<Identifier::Identifier>;
                    eqn_ptr = Slice::getT(__comp_eqn.clone());
                    eqn = Pointer::access(eqn_ptr.clone());
                    ident = metamodelica::Ref::new(Identifier::Identifier {
                        eqn: eqn_ptr.clone(),
                        var_cref: __comp_var_cref.clone(),
                        resizable: false,
                    });
                    generic_call_index = UnorderedMap::tryAdd(
                        ident,
                        UnorderedMap::size(simCodeIndices.generic_call_map.clone()),
                        simCodeIndices.generic_call_map.clone(),
                    )?;
                    tmp = metamodelica::Ref::new(Block::GENERIC_ASSIGN {
                        index: simCodeIndices.equationIndex.clone(),
                        call_index: generic_call_index,
                        scal_indices: __comp_eqn.indices.clone(),
                        source: BEquation::Equation::getSource(eqn.clone()),
                        attr: BEquation::Equation::getAttributes(eqn),
                    });
                    UnorderedMap::add(BEquation::Equation::getEqnName(eqn_ptr)?, tmp.clone(), equation_map)?;
                    simCodeIndices.equationIndex = simCodeIndices.equationIndex.clone() + 1;
                    (tmp.clone(), getIndex(&tmp)?)
                }
                StrongComponent::ENTWINED_COMPONENT {
                    entwined_slices: __comp_entwined_slices,
                    entwined_tpl_lst: __comp_entwined_tpl_lst,
                } => {
                    let mut tmp: metamodelica::Ref<Block>;
                    let mut eqn_ptr: Pointer::Pointer<metamodelica::Ref<Equation::Equation>>;
                    let mut single_call: metamodelica::Ref<Block>;
                    let mut entwined_index_map: metamodelica::Ref<
                        UnorderedMap::UnorderedMap<metamodelica::Ref<ComponentRef::NFComponentRef>, i32>,
                    >;
                    entwined_index_map = UnorderedMap::new(
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
                        1,
                    );
                    for mut slice in &*__comp_entwined_slices.clone() {
                        (single_call, simCodeIndices, _) = fromStrongComponent(
                            metamodelica::AsArg::as_arg(&slice),
                            simCodeIndices,
                            kind,
                            simcode_map.clone(),
                            equation_map.clone(),
                        )?;
                        UnorderedMap::add(
                            getEntwinedEquationName(metamodelica::AsArg::as_arg(&slice))?,
                            ((single_calls).len() as i32),
                            entwined_index_map.clone(),
                        )?;
                        single_calls = metamodelica::cons(single_call, single_calls);
                    }
                    single_calls = single_calls.reverse();
                    for mut tpl in &*__comp_entwined_tpl_lst.clone().reverse() {
                        (eqn_ptr, _) = tpl.clone();
                        call_order = metamodelica::cons(
                            UnorderedMap::getSafe(
                                BEquation::Equation::getEqnName(eqn_ptr)?,
                                entwined_index_map.clone(),
                                metamodelica::sourceInfo!("NSimCode/NSimStrongComponent.mo"),
                            )?,
                            call_order,
                        );
                    }
                    tmp = metamodelica::Ref::new(Block::ENTWINED_ASSIGN {
                        index: simCodeIndices.equationIndex.clone(),
                        call_order: call_order,
                        single_calls: single_calls,
                        source: DAE::emptyElementSource().clone(),
                        attr: BEquation::default(EquationKind::CONTINUOUS.clone(), false, None, None),
                    });
                    simCodeIndices.equationIndex = simCodeIndices.equationIndex.clone() + 1;
                    (tmp.clone(), getIndex(&tmp)?)
                }
                StrongComponent::ALGEBRAIC_LOOP {
                    strict,
                    homotopy: __comp_homotopy,
                    linear: __comp_linear,
                    mixed: __comp_mixed,
                    ..
                } => {
                    let mut system: metamodelica::Ref<NonlinearSystem::NonlinearSystem>;
                    let mut linSystem: metamodelica::Ref<LinearSystem::LinearSystem>;
                    let mut sysIndex: i32;
                    let mut allLinVarsFound: bool;
                    let mut osimvar: Option<metamodelica::Ref<SimVar::SimVar>>;
                    let mut tmp: metamodelica::Ref<Block>;
                    let mut var: metamodelica::Ref<Variable::NFVariable>;
                    let mut jacobian: Option<metamodelica::Ref<SimJacobian::SimJacobian>>;
                    for mut i in 1..=metamodelica::arrayLength(strict.innerEquations.clone()) {
                        (tmp, simCodeIndices, _) = fromStrongComponent(
                            &({
                                let __elt = (*metamodelica::index_checked(&strict.innerEquations.borrow(), i)?).clone();
                                __elt
                            }),
                            simCodeIndices,
                            kind,
                            simcode_map.clone(),
                            equation_map.clone(),
                        )?;
                        eqns = metamodelica::cons(tmp, eqns);
                    }
                    for mut slice in &*strict.residual_eqns.clone() {
                        (tmp, simCodeIndices, residual_index) =
                            createResidual(slice.clone(), simCodeIndices, residual_index, equation_map.clone())?;
                        eqns = metamodelica::cons(tmp, eqns);
                    }
                    allLinVarsFound = true;
                    for mut slice in &*strict.iteration_vars.clone() {
                        var = Pointer::access(Slice::getT(slice.clone()));
                        if Variable::size(&var, false)? > 1 {
                            for mut scal_var in &*Scalarize::scalarizeBackendVariable(&var, slice.indices.clone())? {
                                crefs = metamodelica::cons(scal_var.name.clone(), crefs);
                                osimvar = UnorderedMap::get(scal_var.name.clone(), simcode_map.clone())?;
                                if (osimvar).is_some() {
                                    linVars = metamodelica::cons(osimvar.ok_or("pattern mismatch")?, linVars);
                                } else {
                                    allLinVarsFound = false;
                                }
                            }
                        } else {
                            crefs = metamodelica::cons(var.name.clone(), crefs);
                            osimvar = UnorderedMap::get(var.name.clone(), simcode_map.clone())?;
                            if (osimvar).is_some() {
                                linVars = metamodelica::cons(osimvar.ok_or("pattern mismatch")?, linVars);
                            } else {
                                allLinVarsFound = false;
                            }
                        }
                    }
                    if (strict.jac).is_some() {
                        (jacobian, simCodeIndices) = SimJacobian::create(
                            &(strict.jac.clone().ok_or("pattern mismatch")?),
                            simCodeIndices,
                            simcode_map,
                        )?;
                    } else {
                        jacobian = None;
                    }
                    if __comp_linear.clone()
                        && (jacobian).is_some()
                        && !(__comp_homotopy.clone())
                        && !(__comp_mixed.clone())
                        && !(List::any(
                            &eqns,
                            &move |__a0: metamodelica::Ref<Block>| -> metamodelica::Result<_> {
                                ::std::result::Result::Ok(isForOrGenericResidual(&__a0))
                            },
                        )?)
                        && !(jacobianHasGenericLoopCalls(&(jacobian.clone().ok_or("pattern mismatch")?)))
                        && allLinVarsFound
                    {
                        linSystem = metamodelica::Ref::new(LinearSystem::LinearSystem {
                            index: simCodeIndices.equationIndex.clone(),
                            mixed: __comp_mixed.clone(),
                            torn: true,
                            vars: linVars.reverse(),
                            beqs: metamodelica::nil(),
                            simJac: metamodelica::nil(),
                            residual: eqns.clone().reverse(),
                            jacobian: jacobian,
                            sources: ({
                                let mut __acc: metamodelica::List<metamodelica::Ref<DAE::ElementSource>> =
                                    metamodelica::nil();
                                for mut b in (eqns.reverse()).into_iter().cloned() {
                                    let __x = blockSource(&(b.clone()));
                                    __acc = cons(__x, __acc);
                                }
                                __acc.reverse()
                            }),
                            indexSystem: simCodeIndices.linearSystemIndex.clone(),
                            size: ((crefs).len() as i32),
                            partOfJac: false,
                        });
                        simCodeIndices.linearSystemIndex = simCodeIndices.linearSystemIndex.clone() + 1;
                        simCodeIndices.equationIndex = simCodeIndices.equationIndex.clone() + 1;
                        tmp = metamodelica::Ref::new(Block::LINEAR {
                            system: linSystem.clone(),
                            alternativeTearing: None,
                        });
                        sysIndex = linSystem.index.clone();
                    } else {
                        system = metamodelica::Ref::new(NonlinearSystem::NonlinearSystem {
                            index: simCodeIndices.equationIndex.clone(),
                            blcks: eqns.reverse(),
                            crefs: crefs.clone().reverse(),
                            indexSystem: simCodeIndices.nonlinearSystemIndex.clone(),
                            size: ((crefs).len() as i32),
                            jacobian: Pointer::create(jacobian),
                            homotopy: __comp_homotopy.clone(),
                            mixed: __comp_mixed.clone(),
                            torn: true,
                        });
                        simCodeIndices.nonlinearSystemIndex = simCodeIndices.nonlinearSystemIndex.clone() + 1;
                        simCodeIndices.equationIndex = simCodeIndices.equationIndex.clone() + 1;
                        tmp = metamodelica::Ref::new(Block::NONLINEAR {
                            system: system.clone(),
                            alternativeTearing: None,
                        });
                        sysIndex = system.index.clone();
                    }
                    (tmp, sysIndex)
                }
                StrongComponent::ALIAS {
                    aliasInfo: __comp_aliasInfo,
                    original: __comp_original,
                } => {
                    let mut tmp: metamodelica::Ref<Block>;
                    let mut aliasOf: i32;
                    aliasOf =
                        UnorderedMap::getOrDefault(__comp_aliasInfo.clone(), simCodeIndices.alias_map.clone(), -1)?;
                    tmp = metamodelica::Ref::new(Block::ALIAS {
                        index: simCodeIndices.equationIndex.clone(),
                        aliasInfo: __comp_aliasInfo.clone(),
                        aliasOf: aliasOf,
                        isDiscrete: StrongComponent::isDiscrete(comp)?
                            && !(StrongComponent::isAlgebraicLoop(__comp_original.clone())),
                    });
                    simCodeIndices.equationIndex = simCodeIndices.equationIndex.clone() + 1;
                    (tmp.clone(), getIndex(&tmp)?)
                }
                StrongComponent::ENTWINED_COMPONENT { .. } => {
                    Error::addMessage(
                        Error::INTERNAL_ERROR.clone(),
                        list![{
                            let mut __mm_s = String::new();
                            __mm_s.push_str(&*literal!("NSimStrongComponent.Block.fromStrongComponent"));
                            __mm_s.push_str(&*literal!(" failed because entwined equations have to be resolved beforehand in Solve.solve(). Failed for:\n"));
                            __mm_s.push_str(&*StrongComponent::toString(comp, -1)?);
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
                            __mm_s.push_str(&*literal!("NSimStrongComponent.Block.fromStrongComponent"));
                            __mm_s.push_str(&*literal!(" failed with unknown reason for\n"));
                            __mm_s.push_str(&*StrongComponent::toString(comp, -1)?);
                            ArcStr::from(__mm_s)
                        }],
                    )?;
                    return Err("fail");
                }
            })
        });
        Ok((blck, simCodeIndices, index))
    }

    pub(crate) fn createResidual(
        mut slice: metamodelica::Ref<Slice::NBSlice<Pointer::Pointer<metamodelica::Ref<Equation::Equation>>>>,
        mut simCodeIndices: SimCodeIndices,
        mut res_idx: i32,
        mut equation_map: metamodelica::Ref<
            UnorderedMap::UnorderedMap<metamodelica::Ref<ComponentRef::NFComponentRef>, metamodelica::Ref<Block>>,
        >,
    ) -> Result<(metamodelica::Ref<Block>, SimCodeIndices, i32)> {
        let mut blck: metamodelica::Ref<Block>;
        let mut simCodeIndices: SimCodeIndices = simCodeIndices;
        let mut res_idx: i32 = res_idx;
        let mut eqn: metamodelica::Ref<Equation::Equation> = Pointer::access(Slice::getT(slice.clone()));
        blck = (::match_deref::match_deref! { match &((eqn.clone(), slice.indices.clone())) {
            (Deref @ BEquation::Equation::SCALAR_EQUATION { .. }, _) => {
                let mut tmp: metamodelica::Ref<Block>;
                tmp = metamodelica::Ref::new(Block::RESIDUAL { index: simCodeIndices.equationIndex.clone(), res_index: res_idx, exp: var_field!((*eqn).rhs, Equation::Equation::SCALAR_EQUATION).clone(), source: var_field!((*eqn).source, Equation::Equation::SCALAR_EQUATION).clone(), attr: var_field!((*eqn).attr, Equation::Equation::SCALAR_EQUATION).clone() });
                simCodeIndices.equationIndex = simCodeIndices.equationIndex.clone() + 1;
                res_idx = res_idx + 1;
                tmp
            },
            (Deref @ BEquation::Equation::IF_EQUATION { .. }, _) => {
                let mut tmp: metamodelica::Ref<Block>;
                (tmp, simCodeIndices, res_idx) = createResidual(metamodelica::Ref::new(Slice::NBSlice { t: Pointer::create(BEquation::IfEquationBody::inline(&(var_field!((*eqn).body, Equation::Equation::IF_EQUATION).clone()), eqn.clone())?), indices: slice.indices.clone() }), simCodeIndices, res_idx, equation_map.clone())?;
                tmp
            },
            (Deref @ BEquation::Equation::ARRAY_EQUATION { .. }, Deref @ metamodelica::ListNode::Nil) => {
                let mut tmp: metamodelica::Ref<Block>;
                tmp = metamodelica::Ref::new(Block::ARRAY_RESIDUAL { index: simCodeIndices.equationIndex.clone(), res_index: res_idx, exp: var_field!((*eqn).rhs, Equation::Equation::ARRAY_EQUATION).clone(), source: var_field!((*eqn).source, Equation::Equation::ARRAY_EQUATION).clone(), attr: var_field!((*eqn).attr, Equation::Equation::ARRAY_EQUATION).clone() });
                simCodeIndices.equationIndex = simCodeIndices.equationIndex.clone() + 1;
                res_idx = res_idx + BEquation::Equation::size(Slice::getT(slice), false)?;
                tmp
            },
            (Deref @ BEquation::Equation::ARRAY_EQUATION { .. }, Deref @ metamodelica::ListNode::Cons { head: i, tail: Deref @ metamodelica::ListNode::Nil }) => {
                let mut tmp: metamodelica::Ref<Block>;
                let mut subs: metamodelica::List<metamodelica::Ref<Subscript::NFSubscript>>;
                subs = ({
            let mut __acc: metamodelica::List<metamodelica::Ref<Subscript::NFSubscript>> = metamodelica::nil();
            for mut s in (Slice::indexToLocation(i.clone(), BEquation::Equation::sizes(Slice::getT(slice.clone()), false)?)).into_iter().cloned() {
                let __x = Subscript::fromExp(metamodelica::Ref::new(Expression::NFExpression::INTEGER { value: s.clone() }));
                __acc = cons(__x, __acc);
            }
            __acc.reverse()
        });
                tmp = metamodelica::Ref::new(Block::ARRAY_RESIDUAL { index: simCodeIndices.equationIndex.clone(), res_index: res_idx, exp: Expression::applySubscripts(&subs, var_field!((*eqn).rhs, Equation::Equation::ARRAY_EQUATION).clone(), false)?, source: var_field!((*eqn).source, Equation::Equation::ARRAY_EQUATION).clone(), attr: var_field!((*eqn).attr, Equation::Equation::ARRAY_EQUATION).clone() });
                simCodeIndices.equationIndex = simCodeIndices.equationIndex.clone() + 1;
                res_idx = res_idx + ((slice.indices).len() as i32);
                tmp
            },
            (Deref @ BEquation::Equation::ARRAY_EQUATION { .. }, _) => {
                let mut tmp: metamodelica::Ref<Block>;
                tmp = metamodelica::Ref::new(Block::ARRAY_RESIDUAL { index: simCodeIndices.equationIndex.clone(), res_index: res_idx, exp: var_field!((*eqn).rhs, Equation::Equation::ARRAY_EQUATION).clone(), source: var_field!((*eqn).source, Equation::Equation::ARRAY_EQUATION).clone(), attr: var_field!((*eqn).attr, Equation::Equation::ARRAY_EQUATION).clone() });
                simCodeIndices.equationIndex = simCodeIndices.equationIndex.clone() + 1;
                res_idx = res_idx + BEquation::Equation::size(Slice::getT(slice), false)?;
                tmp
            },
            (Deref @ BEquation::Equation::FOR_EQUATION { body: Deref @ metamodelica::ListNode::Cons { head: _, tail: Deref @ metamodelica::ListNode::Nil }, .. }, Deref @ metamodelica::ListNode::Nil) => {
                let mut tmp: metamodelica::Ref<Block>;
                tmp = metamodelica::Ref::new(Block::FOR_RESIDUAL { index: simCodeIndices.equationIndex.clone(), res_index: res_idx, iterators: SimIterator::fromIterator(var_field!((*eqn).iter, Equation::Equation::FOR_EQUATION))?, exp: (BEquation::Equation::getRHS(eqn.clone())?).ok_or("pattern mismatch")?, source: var_field!((*eqn).source, Equation::Equation::FOR_EQUATION).clone(), attr: var_field!((*eqn).attr, Equation::Equation::FOR_EQUATION).clone() });
                simCodeIndices.equationIndex = simCodeIndices.equationIndex.clone() + 1;
                res_idx = res_idx + BEquation::Equation::size(Slice::getT(slice), false)?;
                tmp
            },
            (Deref @ BEquation::Equation::FOR_EQUATION { body: Deref @ metamodelica::ListNode::Cons { head: body_eqn, tail: Deref @ metamodelica::ListNode::Nil }, .. }, _) if (BEquation::Equation::size(Pointer::create(body_eqn.clone()), false)? > 1) => {
                Error::addMessage(Error::INTERNAL_ERROR.clone(), list![{ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NSimStrongComponent.Block.createResidual")); __mm_s.push_str(&*literal!(" does not support a part of a for equation with an array valued body:\n")); __mm_s.push_str(&*Slice::toString(slice, &({ let __pe_b1 = literal!(""); move |__pe_a0| BEquation::Equation::pointerToString(__pe_a0, __pe_b1.clone()) }), 10)?); ArcStr::from(__mm_s) }])?;
                return Err("fail")
            },
            (Deref @ BEquation::Equation::FOR_EQUATION { body: Deref @ metamodelica::ListNode::Cons { head: _, tail: Deref @ metamodelica::ListNode::Nil }, .. }, _) => {
                let mut tmp: metamodelica::Ref<Block>;
                tmp = metamodelica::Ref::new(Block::GENERIC_RESIDUAL { index: simCodeIndices.equationIndex.clone(), res_index: res_idx, scal_indices: slice.indices.clone(), iterators: SimIterator::fromIterator(var_field!((*eqn).iter, Equation::Equation::FOR_EQUATION))?, exp: (BEquation::Equation::getRHS(eqn.clone())?).ok_or("pattern mismatch")?, source: var_field!((*eqn).source, Equation::Equation::FOR_EQUATION).clone(), attr: var_field!((*eqn).attr, Equation::Equation::FOR_EQUATION).clone() });
                simCodeIndices.equationIndex = simCodeIndices.equationIndex.clone() + 1;
                res_idx = res_idx + ((slice.indices).len() as i32);
                tmp
            },
            (Deref @ BEquation::Equation::WHEN_EQUATION { .. }, _) => {
                let mut tmp: metamodelica::Ref<Block>;
                (tmp, simCodeIndices) = createWhenBody(&(var_field!((*eqn).body, Equation::Equation::WHEN_EQUATION).clone()), var_field!((*eqn).source, Equation::Equation::WHEN_EQUATION).clone(), var_field!((*eqn).attr, Equation::Equation::WHEN_EQUATION).clone(), simCodeIndices)?;
                tmp
            },
            _ => {
                Error::addMessage(Error::INTERNAL_ERROR.clone(), list![{ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NSimStrongComponent.Block.createResidual")); __mm_s.push_str(&*literal!(" failed for\n")); __mm_s.push_str(&*Slice::toString(slice, &({ let __pe_b1 = literal!(""); move |__pe_a0| BEquation::Equation::pointerToString(__pe_a0, __pe_b1.clone()) }), 10)?); ArcStr::from(__mm_s) }])?;
                return Err("fail")
            },
            _ => unreachable!("match_deref! exhaustiveness placeholder"),
        } });
        UnorderedMap::add(
            BEquation::Equation::getEqnName(Pointer::create(eqn))?,
            blck.clone(),
            equation_map,
        )?;
        Ok((blck, simCodeIndices, res_idx))
    }

    pub(crate) fn createEquation(
        mut var: metamodelica::Ref<Variable::NFVariable>,
        mut eqn: metamodelica::Ref<Equation::Equation>,
        mut status: Solve::Status,
        mut simCodeIndices: SimCodeIndices,
        mut kind: Partition::Kind,
        mut simcode_map: metamodelica::Ref<
            UnorderedMap::UnorderedMap<
                metamodelica::Ref<ComponentRef::NFComponentRef>,
                metamodelica::Ref<SimVar::SimVar>,
            >,
        >,
        mut equation_map: metamodelica::Ref<
            UnorderedMap::UnorderedMap<metamodelica::Ref<ComponentRef::NFComponentRef>, metamodelica::Ref<Block>>,
        >,
    ) -> Result<(metamodelica::Ref<Block>, SimCodeIndices)> {
        let mut blck: metamodelica::Ref<Block>;
        let mut simCodeIndices: SimCodeIndices = simCodeIndices;
        blck = (::match_deref::match_deref! { match &((eqn.clone(), status)) {
            (Deref @ BEquation::Equation::SCALAR_EQUATION { .. }, Solve::Status::EXPLICIT) => {
                let mut tmp: metamodelica::Ref<Block>;
                tmp = metamodelica::Ref::new(Block::SIMPLE_ASSIGN { index: simCodeIndices.equationIndex.clone(), lhs: var.name.clone(), rhs: var_field!((*eqn).rhs, Equation::Equation::SCALAR_EQUATION).clone(), source: var_field!((*eqn).source, Equation::Equation::SCALAR_EQUATION).clone(), attr: var_field!((*eqn).attr, Equation::Equation::SCALAR_EQUATION).clone() });
                simCodeIndices.equationIndex = simCodeIndices.equationIndex.clone() + 1;
                tmp
            },
            (Deref @ BEquation::Equation::ARRAY_EQUATION { .. }, Solve::Status::EXPLICIT) => {
                let mut rhs: metamodelica::Ref<Expression::NFExpression>;
                let mut tmp: metamodelica::Ref<Block>;
                rhs = if (Type::isArray(&(Expression::typeOf(var_field!((*eqn).rhs, Equation::Equation::ARRAY_EQUATION).clone())))) {var_field!((*eqn).rhs, Equation::Equation::ARRAY_EQUATION).clone()} else {Expression::fillType(var_field!((*eqn).ty, Equation::Equation::ARRAY_EQUATION).clone(), var_field!((*eqn).rhs, Equation::Equation::ARRAY_EQUATION).clone())?};
                tmp = metamodelica::Ref::new(Block::ARRAY_ASSIGN { index: simCodeIndices.equationIndex.clone(), lhs: var_field!((*eqn).lhs, Equation::Equation::ARRAY_EQUATION).clone(), rhs: rhs, source: var_field!((*eqn).source, Equation::Equation::ARRAY_EQUATION).clone(), attr: var_field!((*eqn).attr, Equation::Equation::ARRAY_EQUATION).clone() });
                simCodeIndices.equationIndex = simCodeIndices.equationIndex.clone() + 1;
                tmp
            },
            (Deref @ BEquation::Equation::RECORD_EQUATION { .. }, Solve::Status::EXPLICIT) => {
                let mut tmp: metamodelica::Ref<Block>;
                (tmp, simCodeIndices) = createAlgorithm(eqn.clone(), simCodeIndices, equation_map.clone())?;
                tmp
            },
            (Deref @ BEquation::Equation::FOR_EQUATION { .. }, Solve::Status::EXPLICIT) => {
                let mut tmp: metamodelica::Ref<Block>;
                (tmp, simCodeIndices) = createAlgorithm(eqn.clone(), simCodeIndices, equation_map.clone())?;
                tmp
            },
            (Deref @ BEquation::Equation::WHEN_EQUATION { .. }, Solve::Status::EXPLICIT) => {
                let mut tmp: metamodelica::Ref<Block>;
                (tmp, simCodeIndices) = createWhenBody(&(var_field!((*eqn).body, Equation::Equation::WHEN_EQUATION).clone()), var_field!((*eqn).source, Equation::Equation::WHEN_EQUATION).clone(), var_field!((*eqn).attr, Equation::Equation::WHEN_EQUATION).clone(), simCodeIndices)?;
                tmp
            },
            (Deref @ BEquation::Equation::IF_EQUATION { .. }, Solve::Status::EXPLICIT) => {
                let mut tmp: metamodelica::Ref<Block>;
                let mut branches: metamodelica::List<(metamodelica::Ref<Expression::NFExpression>, metamodelica::List<metamodelica::Ref<Block>>)>;
                (branches, simCodeIndices) = createIfBody(var_field!((*eqn).body, Equation::Equation::IF_EQUATION).clone(), metamodelica::nil(), simCodeIndices, kind, simcode_map, equation_map.clone())?;
                tmp = metamodelica::Ref::new(Block::IF { index: simCodeIndices.equationIndex.clone(), branches: branches.reverse(), source: var_field!((*eqn).source, Equation::Equation::IF_EQUATION).clone(), attr: var_field!((*eqn).attr, Equation::Equation::IF_EQUATION).clone() });
                simCodeIndices.equationIndex = simCodeIndices.equationIndex.clone() + 1;
                tmp
            },
            (Deref @ BEquation::Equation::ALGORITHM { .. }, Solve::Status::EXPLICIT) => {
                let mut tmp: metamodelica::Ref<Block>;
                tmp = metamodelica::Ref::new(Block::ALGORITHM { index: simCodeIndices.equationIndex.clone(), stmts: var_field!((*eqn).alg, Equation::Equation::ALGORITHM).statements.clone(), attr: var_field!((*eqn).attr, Equation::Equation::ALGORITHM).clone() });
                simCodeIndices.equationIndex = simCodeIndices.equationIndex.clone() + 1;
                tmp
            },
            (_, Solve::Status::IMPLICIT) => {
                let mut tmp: metamodelica::Ref<Block>;
                (tmp, simCodeIndices) = createImplicitEquation(var, eqn.clone(), simCodeIndices, kind, simcode_map, equation_map.clone())?;
                tmp
            },
            _ => {
                Error::addMessage(Error::INTERNAL_ERROR.clone(), list![{ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NSimStrongComponent.Block.createEquation")); __mm_s.push_str(&*literal!(" failed with status ")); __mm_s.push_str(&*Solve::statusString(status)); __mm_s.push_str(&*literal!(" for\n")); __mm_s.push_str(&*BEquation::Equation::toString(eqn.clone(), literal!(""))?); ArcStr::from(__mm_s) }])?;
                return Err("fail")
            },
            _ => unreachable!("match_deref! exhaustiveness placeholder"),
        } });
        UnorderedMap::add(
            BEquation::Equation::getEqnName(Pointer::create(eqn))?,
            blck.clone(),
            equation_map,
        )?;
        Ok((blck, simCodeIndices))
    }

    pub(crate) fn createImplicitEquation(
        mut var: metamodelica::Ref<Variable::NFVariable>,
        mut eqn: metamodelica::Ref<Equation::Equation>,
        mut simCodeIndices: SimCodeIndices,
        mut kind: Partition::Kind,
        mut simcode_map: metamodelica::Ref<
            UnorderedMap::UnorderedMap<
                metamodelica::Ref<ComponentRef::NFComponentRef>,
                metamodelica::Ref<SimVar::SimVar>,
            >,
        >,
        mut equation_map: metamodelica::Ref<
            UnorderedMap::UnorderedMap<metamodelica::Ref<ComponentRef::NFComponentRef>, metamodelica::Ref<Block>>,
        >,
    ) -> Result<(metamodelica::Ref<Block>, SimCodeIndices)> {
        let mut blck: metamodelica::Ref<Block>;
        let mut simCodeIndices: SimCodeIndices = simCodeIndices;
        let mut comp: metamodelica::Ref<StrongComponent::NBStrongComponent>;
        let mut index: i32;
        (comp, index) = Tearing::implicit(
            metamodelica::Ref::new(StrongComponent::NBStrongComponent::SINGLE_COMPONENT {
                var: Pointer::create(var),
                eqn: Pointer::create(eqn),
                status: Solve::Status::IMPLICIT.clone(),
            }),
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
            simCodeIndices.implicitIndex.clone(),
            kind,
        )?;
        simCodeIndices.implicitIndex = index;
        (blck, simCodeIndices, _) = fromStrongComponent(&comp, simCodeIndices, kind, simcode_map, equation_map)?;
        Ok((blck, simCodeIndices))
    }

    pub(crate) fn createWhenBody(
        mut body: &metamodelica::Ref<WhenEquationBody::WhenEquationBody>,
        mut source: metamodelica::Ref<DAE::ElementSource>,
        mut attr: metamodelica::Ref<EquationAttributes::EquationAttributes>,
        mut simCodeIndices: SimCodeIndices,
    ) -> Result<(metamodelica::Ref<Block>, SimCodeIndices)> {
        let mut blck: metamodelica::Ref<Block>;
        let mut simCodeIndices: SimCodeIndices = simCodeIndices;
        let mut conditions: metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>>;
        let mut when_stmts: metamodelica::List<metamodelica::Ref<WhenStatement::WhenStatement>>;
        let mut else_when: Option<metamodelica::Ref<WhenEquationBody::WhenEquationBody>>;
        let mut tmp: metamodelica::Ref<Block>;
        let mut else_when_block: Option<metamodelica::Ref<Block>>;
        let mut index: i32 = simCodeIndices.equationIndex.clone();
        simCodeIndices.equationIndex = simCodeIndices.equationIndex.clone() + 1;
        (conditions, when_stmts, else_when) = BEquation::WhenEquationBody::getBodyAttributes(body)?;
        if (else_when).is_some() {
            (tmp, simCodeIndices) = createWhenBody(
                &(else_when.ok_or("pattern mismatch")?),
                source.clone(),
                attr.clone(),
                simCodeIndices,
            )?;
            else_when_block = Some(tmp);
        } else {
            else_when_block = None;
        }
        blck = metamodelica::Ref::new(Block::WHEN {
            index: index,
            initialCall: false,
            conditions: conditions,
            when_stmts: when_stmts,
            else_when: else_when_block,
            source: source,
            attr: attr,
        });
        Ok((blck, simCodeIndices))
    }

    pub(crate) fn createIfBody(
        mut body: metamodelica::Ref<IfEquationBody::IfEquationBody>,
        mut branches: metamodelica::List<(
            metamodelica::Ref<Expression::NFExpression>,
            metamodelica::List<metamodelica::Ref<Block>>,
        )>,
        mut simCodeIndices: SimCodeIndices,
        mut kind: Partition::Kind,
        mut simcode_map: metamodelica::Ref<
            UnorderedMap::UnorderedMap<
                metamodelica::Ref<ComponentRef::NFComponentRef>,
                metamodelica::Ref<SimVar::SimVar>,
            >,
        >,
        mut equation_map: metamodelica::Ref<
            UnorderedMap::UnorderedMap<metamodelica::Ref<ComponentRef::NFComponentRef>, metamodelica::Ref<Block>>,
        >,
    ) -> Result<(
        metamodelica::List<(
            metamodelica::Ref<Expression::NFExpression>,
            metamodelica::List<metamodelica::Ref<Block>>,
        )>,
        SimCodeIndices,
    )> {
        let mut branches: metamodelica::List<(
            metamodelica::Ref<Expression::NFExpression>,
            metamodelica::List<metamodelica::Ref<Block>>,
        )> = branches;
        let mut simCodeIndices: SimCodeIndices = simCodeIndices;
        let mut comps: metamodelica::List<metamodelica::Ref<StrongComponent::NBStrongComponent>>;
        let mut blck: metamodelica::Ref<Block>;
        let mut blcks: metamodelica::List<metamodelica::Ref<Block>> = metamodelica::nil();
        comps = ({
            let mut __acc: metamodelica::List<metamodelica::Ref<StrongComponent::NBStrongComponent>> =
                metamodelica::nil();
            for mut eqn in (body.then_eqns.clone()).into_iter().cloned() {
                let __x = StrongComponent::fromSolvedEquationSlice(metamodelica::Ref::new(Slice::NBSlice {
                    t: eqn.clone(),
                    indices: metamodelica::nil(),
                }))?;
                __acc = cons(__x, __acc);
            }
            __acc.reverse()
        });
        for mut comp in &*comps.reverse() {
            (blck, simCodeIndices, _) = fromStrongComponent(
                metamodelica::AsArg::as_arg(&comp),
                simCodeIndices,
                kind,
                simcode_map.clone(),
                equation_map.clone(),
            )?;
            blcks = metamodelica::cons(blck, blcks);
        }
        branches = metamodelica::cons((body.condition.clone(), blcks), branches);
        if (body.else_if).is_some() {
            (branches, simCodeIndices) = createIfBody(
                body.else_if.clone().ok_or("pattern mismatch")?,
                branches,
                simCodeIndices,
                kind,
                simcode_map,
                equation_map,
            )?;
        }
        Ok((branches, simCodeIndices))
    }

    pub(crate) fn createAlgorithm(
        mut eqn: metamodelica::Ref<Equation::Equation>,
        mut indices: SimCodeIndices,
        mut equation_map: metamodelica::Ref<
            UnorderedMap::UnorderedMap<metamodelica::Ref<ComponentRef::NFComponentRef>, metamodelica::Ref<Block>>,
        >,
    ) -> Result<(metamodelica::Ref<Block>, SimCodeIndices)> {
        let mut blck: metamodelica::Ref<Block>;
        let mut indices: SimCodeIndices = indices;
        let mut stmts: metamodelica::List<metamodelica::Ref<Statement::NFStatement>>;
        stmts = (match &*eqn {
            BEquation::Equation::ALGORITHM { alg: __eqn_alg, .. } => __eqn_alg.statements.clone(),
            _ => BEquation::Equation::toStatement(&eqn)?,
        });
        blck = metamodelica::Ref::new(Block::ALGORITHM {
            index: indices.equationIndex.clone(),
            stmts: stmts,
            attr: BEquation::Equation::getAttributes(eqn.clone()),
        });
        indices.equationIndex = indices.equationIndex.clone() + 1;
        UnorderedMap::add(
            BEquation::Equation::getEqnName(Pointer::create(eqn))?,
            blck.clone(),
            equation_map,
        )?;
        Ok((blck, indices))
    }

    pub(crate) fn createAssignment(
        mut eqn: metamodelica::Ref<Equation::Equation>,
        mut simCodeIndices: SimCodeIndices,
    ) -> Result<(metamodelica::Ref<Block>, SimCodeIndices)> {
        let mut blck: metamodelica::Ref<Block>;
        let mut simCodeIndices: SimCodeIndices = simCodeIndices;
        blck = (::match_deref::match_deref! { match &(eqn.clone()) {
            qual @ Deref @ BEquation::Equation::SCALAR_EQUATION { lhs: Deref @ Expression::CREF { cref, .. }, .. } => {
                let mut tmp: metamodelica::Ref<Block>;
                tmp = metamodelica::Ref::new(Block::SIMPLE_ASSIGN { index: simCodeIndices.equationIndex.clone(), lhs: cref.clone(), rhs: var_field!((**qual).rhs, Equation::Equation::SCALAR_EQUATION).clone(), source: var_field!((**qual).source, Equation::Equation::SCALAR_EQUATION).clone(), attr: var_field!((**qual).attr, Equation::Equation::SCALAR_EQUATION).clone() });
                simCodeIndices.equationIndex = simCodeIndices.equationIndex.clone() + 1;
                tmp
            },
            qual @ Deref @ BEquation::Equation::ARRAY_EQUATION { lhs: Deref @ Expression::CREF { cref, .. }, .. } => {
                let mut tmp: metamodelica::Ref<Block>;
                tmp = metamodelica::Ref::new(Block::SIMPLE_ASSIGN { index: simCodeIndices.equationIndex.clone(), lhs: cref.clone(), rhs: var_field!((**qual).rhs, Equation::Equation::ARRAY_EQUATION).clone(), source: var_field!((**qual).source, Equation::Equation::ARRAY_EQUATION).clone(), attr: var_field!((**qual).attr, Equation::Equation::ARRAY_EQUATION).clone() });
                simCodeIndices.equationIndex = simCodeIndices.equationIndex.clone() + 1;
                tmp
            },
            _ => {
                Error::addMessage(Error::INTERNAL_ERROR.clone(), list![{ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NSimStrongComponent.Block.createAssignment")); __mm_s.push_str(&*literal!(" failed for\n")); __mm_s.push_str(&*BEquation::Equation::toString(eqn, literal!(""))?); ArcStr::from(__mm_s) }])?;
                return Err("fail")
            },
            _ => unreachable!("match_deref! exhaustiveness placeholder"),
        } });
        Ok((blck, simCodeIndices))
    }

    pub(crate) fn collectAlgebraicLoops(
        mut blcks: &metamodelica::List<metamodelica::List<metamodelica::Ref<Block>>>,
        mut linearLoops: metamodelica::List<metamodelica::Ref<Block>>,
        mut nonlinearLoops: metamodelica::List<metamodelica::Ref<Block>>,
        mut jacobians: metamodelica::List<metamodelica::Ref<SimJacobian::SimJacobian>>,
        mut simCodeIndices: SimCodeIndices,
        mut simcode_map: metamodelica::Ref<
            UnorderedMap::UnorderedMap<
                metamodelica::Ref<ComponentRef::NFComponentRef>,
                metamodelica::Ref<SimVar::SimVar>,
            >,
        >,
    ) -> Result<(
        metamodelica::List<metamodelica::Ref<Block>>,
        metamodelica::List<metamodelica::Ref<Block>>,
        metamodelica::List<metamodelica::Ref<SimJacobian::SimJacobian>>,
        SimCodeIndices,
    )> {
        let mut linearLoops: metamodelica::List<metamodelica::Ref<Block>> = linearLoops;
        let mut nonlinearLoops: metamodelica::List<metamodelica::Ref<Block>> = nonlinearLoops;
        let mut jacobians: metamodelica::List<metamodelica::Ref<SimJacobian::SimJacobian>> = jacobians;
        let mut simCodeIndices: SimCodeIndices = simCodeIndices;
        for mut blck_lst in &**blcks {
            (linearLoops, nonlinearLoops, jacobians, simCodeIndices) = collectAlgebraicLoopsSingle(
                metamodelica::AsArg::as_arg(&blck_lst),
                linearLoops,
                nonlinearLoops,
                jacobians,
                simCodeIndices,
                simcode_map.clone(),
            )?;
        }
        Ok((linearLoops, nonlinearLoops, jacobians, simCodeIndices))
    }

    pub(crate) fn collectAlgebraicLoopsSingle(
        mut blck_lst: &metamodelica::List<metamodelica::Ref<Block>>,
        mut linearLoops: metamodelica::List<metamodelica::Ref<Block>>,
        mut nonlinearLoops: metamodelica::List<metamodelica::Ref<Block>>,
        mut jacobians: metamodelica::List<metamodelica::Ref<SimJacobian::SimJacobian>>,
        mut simCodeIndices: SimCodeIndices,
        mut simcode_map: metamodelica::Ref<
            UnorderedMap::UnorderedMap<
                metamodelica::Ref<ComponentRef::NFComponentRef>,
                metamodelica::Ref<SimVar::SimVar>,
            >,
        >,
    ) -> Result<(
        metamodelica::List<metamodelica::Ref<Block>>,
        metamodelica::List<metamodelica::Ref<Block>>,
        metamodelica::List<metamodelica::Ref<SimJacobian::SimJacobian>>,
        SimCodeIndices,
    )> {
        let mut linearLoops: metamodelica::List<metamodelica::Ref<Block>> = linearLoops;
        let mut nonlinearLoops: metamodelica::List<metamodelica::Ref<Block>> = nonlinearLoops;
        let mut jacobians: metamodelica::List<metamodelica::Ref<SimJacobian::SimJacobian>> = jacobians;
        let mut simCodeIndices: SimCodeIndices = simCodeIndices;
        for mut blck in &**blck_lst {
            let mut blck = blck.clone();
            (linearLoops, nonlinearLoops) = (match &*blck {
                LINEAR {
                    system: __blck_system, ..
                } => {
                    if (__blck_system.jacobian).is_some() {
                        jacobians =
                            metamodelica::cons(__blck_system.jacobian.clone().ok_or("pattern mismatch")?, jacobians);
                    }
                    (metamodelica::cons(blck, linearLoops), nonlinearLoops)
                }
                NONLINEAR {
                    system: __blck_system, ..
                } => {
                    let mut opt_jacobian: Option<metamodelica::Ref<SimJacobian::SimJacobian>>;
                    let mut jacobian: metamodelica::Ref<SimJacobian::SimJacobian>;
                    opt_jacobian = NonlinearSystem::getJacobian(metamodelica::AsArg::as_arg(&__blck_system));
                    if (opt_jacobian).is_some() {
                        jacobian = opt_jacobian.clone().ok_or("pattern mismatch")?;
                        jacobians = metamodelica::cons(jacobian, jacobians);
                    }
                    assign_variant_field!(blck => Block::NONLINEAR; system = NonlinearSystem::setJacobian(__blck_system.clone(), opt_jacobian));
                    (linearLoops, metamodelica::cons(blck, nonlinearLoops))
                }
                _ => (linearLoops, nonlinearLoops),
            });
        }
        Ok((linearLoops, nonlinearLoops, jacobians, simCodeIndices))
    }

    pub(crate) fn convert(mut blck: &metamodelica::Ref<Block>) -> Result<metamodelica::Ref<OldSimCode::SimEqSystem>> {
        let mut oldBlck: metamodelica::Ref<OldSimCode::SimEqSystem>;
        oldBlck = ({
            let mut oldBranches: metamodelica::List<(
                metamodelica::Ref<DAE::Exp>,
                metamodelica::List<metamodelica::Ref<OldSimCode::SimEqSystem>>,
            )> = metamodelica::nil();
            let mut else_branch: metamodelica::List<metamodelica::Ref<OldSimCode::SimEqSystem>> = metamodelica::nil();
            (match &**blck {
                RESIDUAL {
                    attr: __blck_attr,
                    exp: __blck_exp,
                    index: __blck_index,
                    res_index: __blck_res_index,
                    source: __blck_source,
                } => metamodelica::Ref::new(OldSimCode::SimEqSystem::SES_RESIDUAL {
                    index: __blck_index.clone(),
                    res_index: __blck_res_index.clone(),
                    exp: Expression::toDAE(__blck_exp.clone(), false)?,
                    source: __blck_source.clone(),
                    eqAttr: BEquation::EquationAttributes::convert(metamodelica::AsArg::as_arg(&__blck_attr))?,
                }),
                ARRAY_RESIDUAL {
                    attr: __blck_attr,
                    exp: __blck_exp,
                    index: __blck_index,
                    res_index: __blck_res_index,
                    source: __blck_source,
                } => metamodelica::Ref::new(OldSimCode::SimEqSystem::SES_RESIDUAL {
                    index: __blck_index.clone(),
                    res_index: __blck_res_index.clone(),
                    exp: Expression::toDAE(__blck_exp.clone(), false)?,
                    source: __blck_source.clone(),
                    eqAttr: BEquation::EquationAttributes::convert(metamodelica::AsArg::as_arg(&__blck_attr))?,
                }),
                FOR_RESIDUAL {
                    attr: __blck_attr,
                    exp: __blck_exp,
                    index: __blck_index,
                    iterators: __blck_iterators,
                    res_index: __blck_res_index,
                    source: __blck_source,
                } => metamodelica::Ref::new(OldSimCode::SimEqSystem::SES_FOR_RESIDUAL {
                    index: __blck_index.clone(),
                    res_index: __blck_res_index.clone(),
                    iterators: ({
                        let mut __acc: metamodelica::List<OldBackendDAE::SimIterator> = metamodelica::nil();
                        for mut it in (__blck_iterators.clone()).into_iter().cloned() {
                            let __x = SimIterator::convert(&(it.clone()))?;
                            __acc = cons(__x, __acc);
                        }
                        __acc.reverse()
                    }),
                    exp: Expression::toDAE(__blck_exp.clone(), false)?,
                    source: __blck_source.clone(),
                    eqAttr: BEquation::EquationAttributes::convert(metamodelica::AsArg::as_arg(&__blck_attr))?,
                }),
                GENERIC_RESIDUAL {
                    attr: __blck_attr,
                    exp: __blck_exp,
                    index: __blck_index,
                    iterators: __blck_iterators,
                    res_index: __blck_res_index,
                    scal_indices: __blck_scal_indices,
                    source: __blck_source,
                } => metamodelica::Ref::new(OldSimCode::SimEqSystem::SES_GENERIC_RESIDUAL {
                    index: __blck_index.clone(),
                    res_index: __blck_res_index.clone(),
                    scal_indices: __blck_scal_indices.clone(),
                    iterators: ({
                        let mut __acc: metamodelica::List<OldBackendDAE::SimIterator> = metamodelica::nil();
                        for mut it in (__blck_iterators.clone()).into_iter().cloned() {
                            let __x = SimIterator::convert(&(it.clone()))?;
                            __acc = cons(__x, __acc);
                        }
                        __acc.reverse()
                    }),
                    exp: Expression::toDAE(__blck_exp.clone(), false)?,
                    source: __blck_source.clone(),
                    eqAttr: BEquation::EquationAttributes::convert(metamodelica::AsArg::as_arg(&__blck_attr))?,
                }),
                SIMPLE_ASSIGN {
                    attr: __blck_attr,
                    index: __blck_index,
                    lhs: __blck_lhs,
                    rhs: __blck_rhs,
                    source: __blck_source,
                } => metamodelica::Ref::new(OldSimCode::SimEqSystem::SES_SIMPLE_ASSIGN {
                    index: __blck_index.clone(),
                    cref: ComponentRef::toDAE(metamodelica::AsArg::as_arg(&__blck_lhs))?,
                    exp: Expression::toDAE(__blck_rhs.clone(), false)?,
                    source: __blck_source.clone(),
                    eqAttr: BEquation::EquationAttributes::convert(metamodelica::AsArg::as_arg(&__blck_attr))?,
                }),
                ARRAY_ASSIGN {
                    attr: __blck_attr,
                    index: __blck_index,
                    lhs: __blck_lhs,
                    rhs: __blck_rhs,
                    source: __blck_source,
                } => metamodelica::Ref::new(OldSimCode::SimEqSystem::SES_ARRAY_CALL_ASSIGN {
                    index: __blck_index.clone(),
                    lhs: Expression::toDAE(__blck_lhs.clone(), false)?,
                    exp: Expression::toDAE(__blck_rhs.clone(), false)?,
                    source: __blck_source.clone(),
                    eqAttr: BEquation::EquationAttributes::convert(metamodelica::AsArg::as_arg(&__blck_attr))?,
                }),
                RESIZABLE_ASSIGN {
                    attr: __blck_attr,
                    call_index: __blck_call_index,
                    index: __blck_index,
                    iters: __blck_iters,
                    source: __blck_source,
                } => metamodelica::Ref::new(OldSimCode::SimEqSystem::SES_RESIZABLE_ASSIGN {
                    index: __blck_index.clone(),
                    call_index: __blck_call_index.clone(),
                    iters: ({
                        let mut __acc: metamodelica::List<OldBackendDAE::SimIterator> = metamodelica::nil();
                        for mut it in (__blck_iters.clone()).into_iter().cloned() {
                            let __x = SimIterator::convert(&(it.clone()))?;
                            __acc = cons(__x, __acc);
                        }
                        __acc.reverse()
                    }),
                    source: __blck_source.clone(),
                    eqAttr: BEquation::EquationAttributes::convert(metamodelica::AsArg::as_arg(&__blck_attr))?,
                }),
                GENERIC_ASSIGN {
                    attr: __blck_attr,
                    call_index: __blck_call_index,
                    index: __blck_index,
                    scal_indices: __blck_scal_indices,
                    source: __blck_source,
                } => metamodelica::Ref::new(OldSimCode::SimEqSystem::SES_GENERIC_ASSIGN {
                    index: __blck_index.clone(),
                    call_index: __blck_call_index.clone(),
                    scal_indices: __blck_scal_indices.clone(),
                    source: __blck_source.clone(),
                    eqAttr: BEquation::EquationAttributes::convert(metamodelica::AsArg::as_arg(&__blck_attr))?,
                }),
                ENTWINED_ASSIGN {
                    attr: __blck_attr,
                    call_order: __blck_call_order,
                    index: __blck_index,
                    single_calls: __blck_single_calls,
                    source: __blck_source,
                } => metamodelica::Ref::new(OldSimCode::SimEqSystem::SES_ENTWINED_ASSIGN {
                    index: __blck_index.clone(),
                    call_order: __blck_call_order.clone(),
                    single_calls: ({
                        let mut __acc: metamodelica::List<metamodelica::Ref<OldSimCode::SimEqSystem>> =
                            metamodelica::nil();
                        for mut single_call in (__blck_single_calls.clone()).into_iter().cloned() {
                            let __x = convert(&(single_call.clone()))?;
                            __acc = cons(__x, __acc);
                        }
                        __acc.reverse()
                    }),
                    source: __blck_source.clone(),
                    eqAttr: BEquation::EquationAttributes::convert(metamodelica::AsArg::as_arg(&__blck_attr))?,
                }),
                IF {
                    attr: __blck_attr,
                    branches: __blck_branches,
                    index: __blck_index,
                    source: __blck_source,
                } => {
                    let mut exp: metamodelica::Ref<Expression::NFExpression>;
                    let mut blcks: metamodelica::List<metamodelica::Ref<Block>>;
                    for mut branch in &*__blck_branches.clone() {
                        (exp, blcks) = branch.clone();
                        if Expression::isEnd(&exp) {
                            if (else_branch).is_empty() {
                                else_branch = ({
                                    let mut __acc: metamodelica::List<metamodelica::Ref<OldSimCode::SimEqSystem>> =
                                        metamodelica::nil();
                                    for mut blck_ in (blcks).into_iter().cloned() {
                                        let __x = convert(&(blck_.clone()))?;
                                        __acc = cons(__x, __acc);
                                    }
                                    __acc.reverse()
                                });
                            } else {
                                Error::addMessage(
                                    Error::INTERNAL_ERROR.clone(),
                                    list![{
                                        let mut __mm_s = String::new();
                                        __mm_s.push_str(&*literal!("NSimStrongComponent.Block.convert"));
                                        __mm_s.push_str(&*literal!(" failed because there is\n                  at least two non-conditional branches in:\n"));
                                        __mm_s.push_str(&*toString(blck, literal!(""))?);
                                        ArcStr::from(__mm_s)
                                    }],
                                )?;
                                return Err("fail");
                            }
                        } else if (else_branch).is_empty() {
                            oldBranches = metamodelica::cons(
                                (
                                    Expression::toDAE(exp, false)?,
                                    ({
                                        let mut __acc: metamodelica::List<metamodelica::Ref<OldSimCode::SimEqSystem>> =
                                            metamodelica::nil();
                                        for mut blck_ in (blcks).into_iter().cloned() {
                                            let __x = convert(&(blck_.clone()))?;
                                            __acc = cons(__x, __acc);
                                        }
                                        __acc.reverse()
                                    }),
                                ),
                                oldBranches,
                            );
                        } else {
                            Error::addMessage(
                                Error::INTERNAL_ERROR.clone(),
                                list![{
                                    let mut __mm_s = String::new();
                                    __mm_s.push_str(&*literal!("NSimStrongComponent.Block.convert"));
                                    __mm_s.push_str(&*literal!(" failed because there is a\n                conditional branch after a non-conditional branch in:\n"));
                                    __mm_s.push_str(&*toString(blck, literal!(""))?);
                                    ArcStr::from(__mm_s)
                                }],
                            )?;
                            return Err("fail");
                        }
                    }
                    if (else_branch).is_empty() {
                        Error::addMessage(
                            Error::INTERNAL_ERROR.clone(),
                            list![{
                                let mut __mm_s = String::new();
                                __mm_s.push_str(&*literal!("NSimStrongComponent.Block.convert"));
                                __mm_s.push_str(&*literal!(" failed because there "));
                                __mm_s.push_str(&*literal!("is no non-conditional branch in:\n"));
                                __mm_s.push_str(&*toString(blck, literal!(""))?);
                                ArcStr::from(__mm_s)
                            }],
                        )?;
                        return Err("fail");
                    }
                    metamodelica::Ref::new(OldSimCode::SimEqSystem::SES_IFEQUATION {
                        index: __blck_index.clone(),
                        ifbranches: oldBranches.reverse(),
                        elsebranch: else_branch,
                        source: __blck_source.clone(),
                        eqAttr: BEquation::EquationAttributes::convert(metamodelica::AsArg::as_arg(&__blck_attr))?,
                    })
                }
                WHEN {
                    attr: __blck_attr,
                    conditions: __blck_conditions,
                    else_when: __blck_else_when,
                    index: __blck_index,
                    initialCall: __blck_initialCall,
                    source: __blck_source,
                    when_stmts: __blck_when_stmts,
                } => metamodelica::Ref::new(OldSimCode::SimEqSystem::SES_WHEN {
                    index: __blck_index.clone(),
                    conditions: ({
                        let mut __acc: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>> = metamodelica::nil();
                        for mut cr in (__blck_conditions.clone()).into_iter().cloned() {
                            let __x = ComponentRef::toDAE(&(cr.clone()))?;
                            __acc = cons(__x, __acc);
                        }
                        __acc.reverse()
                    }),
                    initialCall: __blck_initialCall.clone(),
                    whenStmtLst: ({
                        let mut __acc: metamodelica::List<OldBackendDAE::WhenOperator> = metamodelica::nil();
                        for mut stmt in (__blck_when_stmts.clone()).into_iter().cloned() {
                            let __x = BEquation::WhenStatement::convert(&(stmt.clone()))?;
                            __acc = cons(__x, __acc);
                        }
                        __acc.reverse()
                    }),
                    elseWhen: Util::applyOption(__blck_else_when.clone(), &move |__a0: metamodelica::Ref<Block>| {
                        convert(&__a0)
                    })?,
                    source: __blck_source.clone(),
                    eqAttr: BEquation::EquationAttributes::convert(metamodelica::AsArg::as_arg(&__blck_attr))?,
                }),
                LINEAR {
                    system: __blck_system, ..
                } => metamodelica::Ref::new(OldSimCode::SimEqSystem::SES_LINEAR {
                    lSystem: LinearSystem::convert(metamodelica::AsArg::as_arg(&__blck_system))?,
                    alternativeTearing: None,
                    eqAttr: BEquation::EquationAttributes::convert(
                        &(BEquation::default(EquationKind::CONTINUOUS.clone(), false, None, None)),
                    )?,
                }),
                NONLINEAR {
                    system: __blck_system, ..
                } => metamodelica::Ref::new(OldSimCode::SimEqSystem::SES_NONLINEAR {
                    nlSystem: NonlinearSystem::convert(metamodelica::AsArg::as_arg(&__blck_system))?,
                    alternativeTearing: None,
                    eqAttr: BEquation::EquationAttributes::convert(
                        &(BEquation::default(EquationKind::CONTINUOUS.clone(), false, None, None)),
                    )?,
                }),
                ALGORITHM {
                    attr: __blck_attr,
                    index: __blck_index,
                    stmts: __blck_stmts,
                } => metamodelica::Ref::new(OldSimCode::SimEqSystem::SES_ALGORITHM {
                    index: __blck_index.clone(),
                    statements: ConvertDAE::convertStatements(__blck_stmts.clone())?,
                    eqAttr: BEquation::EquationAttributes::convert(metamodelica::AsArg::as_arg(&__blck_attr))?,
                }),
                ALIAS {
                    aliasOf: __blck_aliasOf,
                    index: __blck_index,
                    ..
                } if (__blck_aliasOf.clone() > 0) => metamodelica::Ref::new(OldSimCode::SimEqSystem::SES_ALIAS {
                    index: __blck_index.clone(),
                    aliasOf: __blck_aliasOf.clone(),
                }),
                ALIAS {
                    aliasOf: __blck_aliasOf,
                    ..
                } if (__blck_aliasOf.clone() == -1) => {
                    Error::addMessage(
                        Error::INTERNAL_ERROR.clone(),
                        list![{
                            let mut __mm_s = String::new();
                            __mm_s.push_str(&*literal!("NSimStrongComponent.Block.convert"));
                            __mm_s.push_str(&*literal!(
                                " failed for following alias block because the index has not been updated:\n"
                            ));
                            __mm_s.push_str(&*toString(blck, literal!(""))?);
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
                            __mm_s.push_str(&*literal!("NSimStrongComponent.Block.convert"));
                            __mm_s.push_str(&*literal!(" failed for\n"));
                            __mm_s.push_str(&*toString(blck, literal!(""))?);
                            ArcStr::from(__mm_s)
                        }],
                    )?;
                    return Err("fail");
                }
            })
        });
        Ok(oldBlck)
    }

    pub(crate) fn convertList(
        mut blck_lst: metamodelica::List<metamodelica::Ref<Block>>,
    ) -> Result<metamodelica::List<metamodelica::Ref<OldSimCode::SimEqSystem>>> {
        let mut oldBlck_lst: metamodelica::List<metamodelica::Ref<OldSimCode::SimEqSystem>> = ({
            let mut __acc: metamodelica::List<metamodelica::Ref<OldSimCode::SimEqSystem>> = metamodelica::nil();
            for mut blck in (blck_lst.clone()).into_iter().cloned() {
                let __x = convert(&(blck.clone()))?;
                __acc = cons(__x, __acc);
            }
            __acc.reverse()
        });
        Ok(oldBlck_lst)
    }

    pub(crate) fn convertListList(
        mut blck_lst_lst: metamodelica::List<metamodelica::List<metamodelica::Ref<Block>>>,
    ) -> Result<metamodelica::List<metamodelica::List<metamodelica::Ref<OldSimCode::SimEqSystem>>>> {
        let mut oldBlck_lst_lst: metamodelica::List<metamodelica::List<metamodelica::Ref<OldSimCode::SimEqSystem>>> =
            ({
                let mut __acc: metamodelica::List<metamodelica::List<metamodelica::Ref<OldSimCode::SimEqSystem>>> =
                    metamodelica::nil();
                for mut blck_lst in (blck_lst_lst.clone()).into_iter().cloned() {
                    let __x = convertList(blck_lst.clone())?;
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            });
        Ok(oldBlck_lst_lst)
    }

    pub(crate) fn fixIndices<'__b>(
        mut blcks: &'__b metamodelica::List<metamodelica::Ref<Block>>,
        mut acc: metamodelica::List<metamodelica::Ref<Block>>,
        mut indices: SimCodeIndices,
    ) -> Result<(metamodelica::List<metamodelica::Ref<Block>>, SimCodeIndices)> {
        '__tco: loop {
            ::match_deref::match_deref! { match blcks {
                Deref @ metamodelica::ListNode::Cons { head: blck, tail: rest } => {
                    let mut blck = (*blck).clone();
                    (blck, indices) = fixIndex(blck.clone(), indices)?;
                    { (blcks, acc, indices) = (rest, metamodelica::cons(blck.clone(), acc), indices); continue '__tco; }
                },
                _ => {
                    return Ok((acc, indices))
                },
                _ => return Err("match: no arm matched"),
            } }
        }
    }

    pub(crate) fn fixIndex(
        mut blck: metamodelica::Ref<Block>,
        mut indices: SimCodeIndices,
    ) -> Result<(metamodelica::Ref<Block>, SimCodeIndices)> {
        let mut blck: metamodelica::Ref<Block> = blck;
        let mut indices: SimCodeIndices = indices;
        blck = (match &*blck {
            RESIDUAL { .. } => {
                assign_variant_field!(blck => Block::RESIDUAL; index = indices.equationIndex.clone());
                indices.equationIndex = indices.equationIndex.clone() + 1;
                blck
            }
            ARRAY_RESIDUAL { .. } => {
                assign_variant_field!(blck => Block::ARRAY_RESIDUAL; index = indices.equationIndex.clone());
                indices.equationIndex = indices.equationIndex.clone() + 1;
                blck
            }
            SIMPLE_ASSIGN { .. } => {
                assign_variant_field!(blck => Block::SIMPLE_ASSIGN; index = indices.equationIndex.clone());
                indices.equationIndex = indices.equationIndex.clone() + 1;
                blck
            }
            ARRAY_ASSIGN { .. } => {
                assign_variant_field!(blck => Block::ARRAY_ASSIGN; index = indices.equationIndex.clone());
                indices.equationIndex = indices.equationIndex.clone() + 1;
                blck
            }
            RESIZABLE_ASSIGN { .. } => {
                assign_variant_field!(blck => Block::RESIZABLE_ASSIGN; index = indices.equationIndex.clone());
                indices.equationIndex = indices.equationIndex.clone() + 1;
                blck
            }
            GENERIC_ASSIGN { .. } => {
                assign_variant_field!(blck => Block::GENERIC_ASSIGN; index = indices.equationIndex.clone());
                indices.equationIndex = indices.equationIndex.clone() + 1;
                blck
            }
            ALIAS { .. } => {
                assign_variant_field!(blck => Block::ALIAS; index = indices.equationIndex.clone());
                indices.equationIndex = indices.equationIndex.clone() + 1;
                blck
            }
            ALGORITHM { .. } => {
                assign_variant_field!(blck => Block::ALGORITHM; index = indices.equationIndex.clone());
                indices.equationIndex = indices.equationIndex.clone() + 1;
                blck
            }
            INVERSE_ALGORITHM { .. } => {
                assign_variant_field!(blck => Block::INVERSE_ALGORITHM; index = indices.equationIndex.clone());
                indices.equationIndex = indices.equationIndex.clone() + 1;
                blck
            }
            IF { .. } => {
                assign_variant_field!(blck => Block::IF; index = indices.equationIndex.clone());
                indices.equationIndex = indices.equationIndex.clone() + 1;
                blck
            }
            WHEN { .. } => {
                let mut tmp: metamodelica::Ref<Block>;
                assign_variant_field!(blck => Block::WHEN; index = indices.equationIndex.clone());
                indices.equationIndex = indices.equationIndex.clone() + 1;
                if (var_field!((*blck).else_when, Block::WHEN)).is_some() {
                    (tmp, indices) = fixIndex(
                        var_field!((*blck).else_when, Block::WHEN)
                            .clone()
                            .ok_or("pattern mismatch")?,
                        indices,
                    )?;
                    assign_variant_field!(blck => Block::WHEN; else_when = Some(tmp));
                }
                blck
            }
            LINEAR { .. } => blck,
            NONLINEAR { .. } => blck,
            HYBRID {
                continuous: __blck_continuous,
                discreteEqs: __blck_discreteEqs,
                ..
            } => {
                let mut tmp: metamodelica::Ref<Block>;
                let mut tmp_lst: metamodelica::List<metamodelica::Ref<Block>>;
                (tmp, indices) = fixIndex(__blck_continuous.clone(), indices)?;
                (tmp_lst, indices) = fixIndices(
                    metamodelica::AsArg::as_arg(&__blck_discreteEqs),
                    metamodelica::nil(),
                    indices,
                )?;
                assign_variant_field!(blck => Block::HYBRID;
                    continuous = tmp,
                    discreteEqs = tmp_lst
                );
                blck
            }
            _ => {
                Error::addMessage(
                    Error::INTERNAL_ERROR.clone(),
                    list![{
                        let mut __mm_s = String::new();
                        __mm_s.push_str(&*literal!("NSimStrongComponent.Block.fixIndex"));
                        __mm_s.push_str(&*literal!(" failed for\n"));
                        __mm_s.push_str(&*toString(&blck, literal!(""))?);
                        ArcStr::from(__mm_s)
                    }],
                )?;
                return Err("fail");
            }
        });
        Ok((blck, indices))
    }

    pub(crate) fn collectEntwinedEquations(
        mut blck: &metamodelica::Ref<Block>,
    ) -> metamodelica::List<metamodelica::Ref<Block>> {
        let mut lst: metamodelica::List<metamodelica::Ref<Block>>;
        lst = (match &**blck {
            ENTWINED_ASSIGN {
                single_calls: __blck_single_calls,
                ..
            } => __blck_single_calls.clone(),
            _ => metamodelica::nil(),
        });
        lst
    }

    fn whenString(
        mut conditions: metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>>,
        mut when_stmts: metamodelica::List<metamodelica::Ref<WhenStatement::WhenStatement>>,
        mut else_when: Option<metamodelica::Ref<Block>>,
        mut r#str: ArcStr,
    ) -> Result<ArcStr> {
        let mut r#str: ArcStr = r#str;
        let mut indent: ArcStr = r#str.clone();
        r#str = {
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("when "));
            __mm_s.push_str(&*List::toString(
                conditions,
                &move |__a0: metamodelica::Ref<ComponentRef::NFComponentRef>| ComponentRef::toString(&__a0),
                List::Style::FLAT_CURLY.clone(),
            )?);
            __mm_s.push_str(&*literal!("\n"));
            __mm_s.push_str(&*List::toString(
                when_stmts,
                &({
                    let __pe_b1 = {
                        let mut __mm_s = String::new();
                        __mm_s.push_str(&*indent);
                        __mm_s.push_str(&*literal!("\t"));
                        ArcStr::from(__mm_s)
                    };
                    move |__pe_a0| BEquation::WhenStatement::toString(&__pe_a0, __pe_b1.clone())
                }),
                List::Style::NEWLINE.clone(),
            )?);
            __mm_s.push_str(&*literal!("\n"));
            ArcStr::from(__mm_s)
        };
        if (else_when).is_some() {
            r#str = {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*r#str);
                __mm_s.push_str(&*indent);
                __mm_s.push_str(&*literal!("else"));
                __mm_s.push_str(&*toString(&(else_when.ok_or("pattern mismatch")?), literal!(""))?);
                ArcStr::from(__mm_s)
            };
        } else {
            r#str = {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*r#str);
                __mm_s.push_str(&*indent);
                __mm_s.push_str(&*literal!("end when;\n"));
                ArcStr::from(__mm_s)
            };
        }
        Ok(r#str)
    }

    fn getGenericAssignIndex(mut blck: &metamodelica::Ref<Block>) -> Result<i32> {
        let mut index: i32;
        index = (match &**blck {
            RESIZABLE_ASSIGN {
                call_index: __blck_call_index,
                ..
            } => __blck_call_index.clone(),
            GENERIC_ASSIGN {
                call_index: __blck_call_index,
                ..
            } => __blck_call_index.clone(),
            _ => {
                Error::addMessage(
                    Error::INTERNAL_ERROR.clone(),
                    list![{
                        let mut __mm_s = String::new();
                        __mm_s.push_str(&*literal!("NSimStrongComponent.Block.getGenericAssignIndex"));
                        __mm_s.push_str(&*literal!(" failed for\n"));
                        __mm_s.push_str(&*toString(blck, literal!(""))?);
                        ArcStr::from(__mm_s)
                    }],
                )?;
                return Err("fail");
            }
        });
        Ok(index)
    }

    fn getGenericEquationName(
        mut comp: &metamodelica::Ref<StrongComponent::NBStrongComponent>,
    ) -> Result<metamodelica::Ref<ComponentRef::NFComponentRef>> {
        let mut name: metamodelica::Ref<ComponentRef::NFComponentRef>;
        name = (match &**comp {
            StrongComponent::GENERIC_COMPONENT { eqn: __comp_eqn, .. } => {
                BEquation::Equation::getEqnName(Slice::getT(__comp_eqn.clone()))?
            }
            _ => {
                Error::addMessage(
                    Error::INTERNAL_ERROR.clone(),
                    list![{
                        let mut __mm_s = String::new();
                        __mm_s.push_str(&*literal!("NSimStrongComponent.Block.getGenericEquationName"));
                        __mm_s.push_str(&*literal!(" failed for\n"));
                        __mm_s.push_str(&*StrongComponent::toString(comp, -1)?);
                        ArcStr::from(__mm_s)
                    }],
                )?;
                return Err("fail");
            }
        });
        Ok(name)
    }

    fn getEntwinedEquationName(
        mut comp: &metamodelica::Ref<StrongComponent::NBStrongComponent>,
    ) -> Result<metamodelica::Ref<ComponentRef::NFComponentRef>> {
        let mut name: metamodelica::Ref<ComponentRef::NFComponentRef>;
        name = (match &**comp {
            StrongComponent::SINGLE_COMPONENT { eqn: __comp_eqn, .. } => {
                BEquation::Equation::getEqnName(__comp_eqn.clone())?
            }
            StrongComponent::MULTI_COMPONENT { eqn: __comp_eqn, .. } => {
                BEquation::Equation::getEqnName(Slice::getT(__comp_eqn.clone()))?
            }
            StrongComponent::SLICED_COMPONENT { eqn: __comp_eqn, .. } => {
                BEquation::Equation::getEqnName(Slice::getT(__comp_eqn.clone()))?
            }
            StrongComponent::GENERIC_COMPONENT { eqn: __comp_eqn, .. } => {
                BEquation::Equation::getEqnName(Slice::getT(__comp_eqn.clone()))?
            }
            StrongComponent::RESIZABLE_COMPONENT { eqn: __comp_eqn, .. } => {
                BEquation::Equation::getEqnName(Slice::getT(__comp_eqn.clone()))?
            }
            _ => {
                Error::addMessage(
                    Error::INTERNAL_ERROR.clone(),
                    list![{
                        let mut __mm_s = String::new();
                        __mm_s.push_str(&*literal!("NSimStrongComponent.Block.getEntwinedEquationName"));
                        __mm_s.push_str(&*literal!(" failed for\n"));
                        __mm_s.push_str(&*StrongComponent::toString(comp, -1)?);
                        ArcStr::from(__mm_s)
                    }],
                )?;
                return Err("fail");
            }
        });
        Ok(name)
    }
}

pub mod LinearSystem {
    use super::*;
    #[derive(Clone, Debug, Eq, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
    pub struct LinearSystem {
        pub index: i32,
        pub mixed: bool,
        pub torn: bool,
        pub vars: metamodelica::List<metamodelica::Ref<SimVar::SimVar>>,
        pub beqs: metamodelica::List<metamodelica::Ref<Expression::NFExpression>>,
        pub simJac: metamodelica::List<(i32, i32, metamodelica::Ref<Block::Block>)>,
        pub residual: metamodelica::List<metamodelica::Ref<Block::Block>>,
        pub jacobian: Option<metamodelica::Ref<SimJacobian::SimJacobian>>,
        pub sources: metamodelica::List<metamodelica::Ref<DAE::ElementSource>>,
        pub indexSystem: i32,
        /// Number of variables that are solved in this system. Needed because 'crefs' only contains the iteration variables.
        pub size: i32,
        /// if TRUE then this system is part of a jacobian matrix
        pub partOfJac: bool,
    }

    impl metamodelica::gc::MMTrace for LinearSystem {
        fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
            metamodelica::gc::MMTrace::mm_accept(&self.index, __mmv)?;
            metamodelica::gc::MMTrace::mm_accept(&self.mixed, __mmv)?;
            metamodelica::gc::MMTrace::mm_accept(&self.torn, __mmv)?;
            metamodelica::gc::MMTrace::mm_accept(&self.vars, __mmv)?;
            metamodelica::gc::MMTrace::mm_accept(&self.beqs, __mmv)?;
            metamodelica::gc::MMTrace::mm_accept(&self.simJac, __mmv)?;
            metamodelica::gc::MMTrace::mm_accept(&self.residual, __mmv)?;
            metamodelica::gc::MMTrace::mm_accept(&self.jacobian, __mmv)?;
            metamodelica::gc::MMTrace::mm_accept(&self.sources, __mmv)?;
            metamodelica::gc::MMTrace::mm_accept(&self.indexSystem, __mmv)?;
            metamodelica::gc::MMTrace::mm_accept(&self.size, __mmv)?;
            metamodelica::gc::MMTrace::mm_accept(&self.partOfJac, __mmv)?;
            Ok(())
        }
    }
    impl Default for LinearSystem {
        fn default() -> Self {
            Self {
                index: Default::default(),
                mixed: Default::default(),
                torn: Default::default(),
                vars: Default::default(),
                beqs: Default::default(),
                simJac: Default::default(),
                residual: Default::default(),
                jacobian: Default::default(),
                sources: Default::default(),
                indexSystem: Default::default(),
                size: Default::default(),
                partOfJac: Default::default(),
            }
        }
    }

    pub type LINEAR_SYSTEM = LinearSystem;

    pub(crate) fn toString(mut system: &metamodelica::Ref<LinearSystem>, mut r#str: ArcStr) -> Result<ArcStr> {
        let mut r#str: ArcStr = r#str;
        r#str = {
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("Linear System (size = "));
            __mm_s.push_str(&*intString(system.size.clone()));
            __mm_s.push_str(&*literal!(", jacobian = "));
            __mm_s.push_str(&*boolString(system.partOfJac.clone()));
            __mm_s.push_str(&*literal!(", mixed = "));
            __mm_s.push_str(&*boolString(system.mixed.clone()));
            __mm_s.push_str(&*literal!(", torn = "));
            __mm_s.push_str(&*boolString(system.torn.clone()));
            __mm_s.push_str(&*literal!(")\n"));
            __mm_s.push_str(&*Block::listToString(
                &system.residual,
                {
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*r#str);
                    __mm_s.push_str(&*literal!("--"));
                    ArcStr::from(__mm_s)
                },
                &(literal!("")),
            )?);
            ArcStr::from(__mm_s)
        };
        Ok(r#str)
    }

    pub(crate) fn convert(
        mut system: &metamodelica::Ref<LinearSystem>,
    ) -> Result<metamodelica::Ref<OldSimCode::LinearSystem>> {
        let mut oldSystem: metamodelica::Ref<OldSimCode::LinearSystem>;
        oldSystem = metamodelica::Ref::new(OldSimCode::LinearSystem {
            index: system.index.clone(),
            partOfMixed: system.mixed.clone(),
            tornSystem: system.torn.clone(),
            vars: SimVar::convertList(system.vars.clone())?,
            beqs: metamodelica::nil(),
            simJac: metamodelica::nil(),
            residual: Block::convertList(system.residual.clone())?,
            jacobianMatrix: Util::applyOption(system.jacobian.clone(), &move |__a0: metamodelica::Ref<
                SimJacobian::SimJacobian,
            >| SimJacobian::convert(&__a0))?,
            sources: system.sources.clone(),
            indexLinearSystem: system.indexSystem.clone(),
            nUnknowns: system.size.clone(),
            partOfJac: system.partOfJac.clone(),
        });
        Ok(oldSystem)
    }
}

pub mod NonlinearSystem {
    use super::*;
    #[derive(Clone, Debug, Eq, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
    pub struct NonlinearSystem {
        pub index: i32,
        /// equations
        pub blcks: metamodelica::List<metamodelica::Ref<Block::Block>>,
        /// iteration variables
        pub crefs: metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>>,
        pub indexSystem: i32,
        /// Number of variables that are solved in this system. Needed because 'crefs' only contains the iteration variables.
        pub size: i32,
        pub jacobian: Pointer::Pointer<Option<metamodelica::Ref<SimJacobian::SimJacobian>>>,
        pub homotopy: bool,
        pub mixed: bool,
        pub torn: bool,
    }

    impl metamodelica::gc::MMTrace for NonlinearSystem {
        fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
            metamodelica::gc::MMTrace::mm_accept(&self.index, __mmv)?;
            metamodelica::gc::MMTrace::mm_accept(&self.blcks, __mmv)?;
            metamodelica::gc::MMTrace::mm_accept(&self.crefs, __mmv)?;
            metamodelica::gc::MMTrace::mm_accept(&self.indexSystem, __mmv)?;
            metamodelica::gc::MMTrace::mm_accept(&self.size, __mmv)?;
            metamodelica::gc::MMTrace::mm_accept(&self.jacobian, __mmv)?;
            metamodelica::gc::MMTrace::mm_accept(&self.homotopy, __mmv)?;
            metamodelica::gc::MMTrace::mm_accept(&self.mixed, __mmv)?;
            metamodelica::gc::MMTrace::mm_accept(&self.torn, __mmv)?;
            Ok(())
        }
    }
    impl Default for NonlinearSystem {
        fn default() -> Self {
            Self {
                index: Default::default(),
                blcks: Default::default(),
                crefs: Default::default(),
                indexSystem: Default::default(),
                size: Default::default(),
                jacobian: Default::default(),
                homotopy: Default::default(),
                mixed: Default::default(),
                torn: Default::default(),
            }
        }
    }

    pub type NONLINEAR_SYSTEM = NonlinearSystem;

    pub(crate) fn getJacobian(
        mut syst: &metamodelica::Ref<NonlinearSystem>,
    ) -> Option<metamodelica::Ref<SimJacobian::SimJacobian>> {
        let mut jacobian: Option<metamodelica::Ref<SimJacobian::SimJacobian>> = Pointer::access(syst.jacobian.clone());
        jacobian
    }

    pub(crate) fn setJacobian(
        mut syst: metamodelica::Ref<NonlinearSystem>,
        mut jacobian: Option<metamodelica::Ref<SimJacobian::SimJacobian>>,
    ) -> metamodelica::Ref<NonlinearSystem> {
        let mut syst: metamodelica::Ref<NonlinearSystem> = syst;
        Pointer::update(syst.jacobian.clone(), jacobian);
        syst
    }

    pub(crate) fn toString(mut system: &metamodelica::Ref<NonlinearSystem>, mut r#str: ArcStr) -> Result<ArcStr> {
        let mut r#str: ArcStr = r#str;
        r#str = {
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("Nonlinear System (size = "));
            __mm_s.push_str(&*intString(system.size.clone()));
            __mm_s.push_str(&*literal!(", homotopy = "));
            __mm_s.push_str(&*boolString(system.homotopy.clone()));
            __mm_s.push_str(&*literal!(", mixed = "));
            __mm_s.push_str(&*boolString(system.mixed.clone()));
            __mm_s.push_str(&*literal!(", torn = "));
            __mm_s.push_str(&*boolString(system.torn.clone()));
            __mm_s.push_str(&*literal!(")\n"));
            __mm_s.push_str(&*r#str);
            __mm_s.push_str(&*literal!("--"));
            __mm_s.push_str(&*List::toStringCustom(
                system.crefs.clone(),
                &move |__a0: metamodelica::Ref<ComponentRef::NFComponentRef>| ComponentRef::toString(&__a0),
                literal!("Iteration Vars:"),
                literal!("{"),
                literal!(", "),
                literal!("}"),
                true,
                10,
            )?);
            __mm_s.push_str(&*literal!("\n"));
            __mm_s.push_str(&*Block::listToString(
                &system.blcks,
                {
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*r#str);
                    __mm_s.push_str(&*literal!("--"));
                    ArcStr::from(__mm_s)
                },
                &(literal!("")),
            )?);
            ArcStr::from(__mm_s)
        };
        Ok(r#str)
    }

    pub(crate) fn convert(
        mut system: &metamodelica::Ref<NonlinearSystem>,
    ) -> Result<metamodelica::Ref<OldSimCode::NonlinearSystem>> {
        let mut oldSystem: metamodelica::Ref<OldSimCode::NonlinearSystem>;
        let mut crefs: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>> = metamodelica::nil();
        for mut cref in &*system.crefs.clone() {
            crefs = metamodelica::cons(ComponentRef::toDAE(metamodelica::AsArg::as_arg(&cref))?, crefs);
        }
        oldSystem = metamodelica::Ref::new(OldSimCode::NonlinearSystem {
            index: system.index.clone(),
            eqs: Block::convertList(system.blcks.clone())?,
            crefs: crefs.reverse(),
            indexNonLinearSystem: system.indexSystem.clone(),
            nUnknowns: system.size.clone(),
            jacobianMatrix: Util::applyOption(
                Pointer::access(system.jacobian.clone()),
                &move |__a0: metamodelica::Ref<SimJacobian::SimJacobian>| SimJacobian::convert(&__a0),
            )?,
            homotopySupport: system.homotopy.clone(),
            mixedSystem: system.mixed.clone(),
            tornSystem: system.torn.clone(),
            clockIndex: None,
        });
        Ok(oldSystem)
    }
}
