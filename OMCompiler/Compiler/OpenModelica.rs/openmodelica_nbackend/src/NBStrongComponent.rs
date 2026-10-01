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
use crate::NBCausalize as Causalize;
use crate::NBEquation::Equation;
use crate::NBEquation::EquationAttributes;
use crate::NBEquation::EquationPointer;
use crate::NBEquation::EquationPointers;
use crate::NBEquation::IfEquationBody;
use crate::NBEquation::Iterator;
use crate::NBInitialization as Initialization;
use crate::NBInline as Inline;
use crate::NBJacobian::JacobianType;
use crate::NBMatching as Matching;
use crate::NBPartition as BPartition;
use crate::NBPartition::Partition;
use crate::NBResizable as Resizable;
use crate::NBResizable::EvalOrder;
use crate::NBSlice as Slice;
use crate::NBSolve as Solve;
use crate::NBSorting as Sorting;
use crate::NBSorting::SuperNode;
use crate::NBTearing as Tearing;
use crate::NBVariable as BVariable;
use crate::NBVariable::VariablePointer;
use crate::NBVariable::VariablePointers;
use crate::NBackendDAE as BackendDAE;
use openmodelica_nf_frontend::NFComponentRef as ComponentRef;
use openmodelica_nf_frontend::NFDimension as Dimension;
use openmodelica_nf_frontend::NFExpression as Expression;
use openmodelica_nf_frontend::NFSubscript as Subscript;
use openmodelica_nf_frontend::NFType as Type;
use openmodelica_nf_frontend::NFVariable as Variable;
use openmodelica_util::Error;
use openmodelica_util::StringUtil;
use openmodelica_util::UnorderedMap;
use openmodelica_util::UnorderedSet;
use openmodelica_util::Util;
use openmodelica_util_datatypes_basic::Array;
use openmodelica_util_datatypes_basic::List;
use openmodelica_util_datatypes_basic::Pointer;

/// file:        NBStrongComponent.mo
/// package:     NBStrongComponent
/// description: This file contains the data-types used save the strong Component
///              data after causalization.
#[derive(Clone, Debug, Eq, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub enum NBStrongComponent {
    /// component for all equations that solve for a single (possibly multidimensional) variable
    ///    SCALAR_EQUATION, ARRAY_EQUATION, RECORD_EQUATION.
    SINGLE_COMPONENT {
        var: Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>,
        eqn: Pointer::Pointer<metamodelica::Ref<Equation::Equation>>,
        status: Solve::Status,
    },
    /// component for all equations that can solve for more than one variable instance
    ///    ALGORITHM, WHEN_EQUATION, IF_EQUATION
    MULTI_COMPONENT {
        vars: metamodelica::List<
            metamodelica::Ref<Slice::NBSlice<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>>>,
        >,
        eqn: metamodelica::Ref<Slice::NBSlice<Pointer::Pointer<metamodelica::Ref<Equation::Equation>>>>,
        status: Solve::Status,
    },
    /// component for all equations AND/OR variables that need to be sliced (zero based indices)
    SLICED_COMPONENT {
        /// cref to solve for
        var_cref: metamodelica::Ref<ComponentRef::NFComponentRef>,
        /// sliced variable
        var: metamodelica::Ref<Slice::NBSlice<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>>>,
        /// sliced equation
        eqn: metamodelica::Ref<Slice::NBSlice<Pointer::Pointer<metamodelica::Ref<Equation::Equation>>>>,
        status: Solve::Status,
    },
    /// component for for-equations with trivial evaluation order
    RESIZABLE_COMPONENT {
        /// cref to solve for
        var_cref: metamodelica::Ref<ComponentRef::NFComponentRef>,
        /// sliced variable
        var: metamodelica::Ref<Slice::NBSlice<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>>>,
        /// sliced equation
        eqn: metamodelica::Ref<Slice::NBSlice<Pointer::Pointer<metamodelica::Ref<Equation::Equation>>>>,
        /// independent, forward, backward
        order:
            metamodelica::Ref<UnorderedMap::UnorderedMap<metamodelica::Ref<ComponentRef::NFComponentRef>, EvalOrder>>,
        status: Solve::Status,
    },
    /// component for all equations that need to be sliced but where no for-loop could be recovered
    ///    has no status since this is generated by the Solve module and is always status=EXPLICIT.
    GENERIC_COMPONENT {
        /// cref to solve for
        var_cref: metamodelica::Ref<ComponentRef::NFComponentRef>,
        /// sliced variable
        var: metamodelica::Ref<Slice::NBSlice<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>>>,
        /// sliced equation
        eqn: metamodelica::Ref<Slice::NBSlice<Pointer::Pointer<metamodelica::Ref<Equation::Equation>>>>,
    },
    /// component for entwined equations that have to be called in a specific interleaved order
    ///    but do not form an algebraic loop. Slices can be SLICED_COMPONENT, GENERIC_COMPONENT,
    ///    RESIZABLE_COMPONENT, SINGLE_COMPONENT or MULTI_COMPONENT.
    ENTWINED_COMPONENT {
        /// one entry per distinct equation (for-loop or scalar)
        entwined_slices: metamodelica::List<metamodelica::Ref<NBStrongComponent>>,
        /// equation with scalar idx (0 based) - fallback scalarization
        entwined_tpl_lst: metamodelica::List<(Pointer::Pointer<metamodelica::Ref<Equation::Equation>>, i32)>,
    },
    /// component for equations that have to be solved as a system.
    ALGEBRAIC_LOOP {
        idx: i32,
        strict: metamodelica::Ref<Tearing::NBTearing>,
        casual: Option<metamodelica::Ref<Tearing::NBTearing>>,
        /// true if the loop is linear
        linear: bool,
        /// true for systems that have discrete variables
        mixed: bool,
        /// true if contains homotopy()
        homotopy: bool,
        status: Solve::Status,
        /// true if this component was promoted straight from a
        ///                                 single/multi/resizable component by NBSolve.mo's
        ///                                 Tearing.implicit() rather than found and torn by
        ///                                 NBTearing.mo's own tearing pass. The two are numbered
        ///                                 (idx) via separate counters that can coincide, so this
        ///                                 flag lets the generated Jacobian's name stay unique.
        implicitlyCreated: bool,
    },
    /// Component representing equal strong components in ODE<->INIT<->DAE
    ///    has no status since this is generated by the Solve module and is always status=EXPLICIT.
    ALIAS {
        /// The strong component array and index it refers to
        aliasInfo: metamodelica::Ref<AliasInfo::AliasInfo>,
        /// The original strong component for analysis
        original: metamodelica::Ref<NBStrongComponent>,
    },
}
impl metamodelica::gc::MMTrace for NBStrongComponent {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        match self {
            NBStrongComponent::SINGLE_COMPONENT { var, eqn, status } => {
                metamodelica::gc::MMTrace::mm_accept(var, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(eqn, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(status, __mmv)?;
                Ok(())
            }
            NBStrongComponent::MULTI_COMPONENT { vars, eqn, status } => {
                metamodelica::gc::MMTrace::mm_accept(vars, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(eqn, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(status, __mmv)?;
                Ok(())
            }
            NBStrongComponent::SLICED_COMPONENT {
                var_cref,
                var,
                eqn,
                status,
            } => {
                metamodelica::gc::MMTrace::mm_accept(var_cref, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(var, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(eqn, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(status, __mmv)?;
                Ok(())
            }
            NBStrongComponent::RESIZABLE_COMPONENT {
                var_cref,
                var,
                eqn,
                order,
                status,
            } => {
                metamodelica::gc::MMTrace::mm_accept(var_cref, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(var, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(eqn, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(order, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(status, __mmv)?;
                Ok(())
            }
            NBStrongComponent::GENERIC_COMPONENT { var_cref, var, eqn } => {
                metamodelica::gc::MMTrace::mm_accept(var_cref, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(var, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(eqn, __mmv)?;
                Ok(())
            }
            NBStrongComponent::ENTWINED_COMPONENT {
                entwined_slices,
                entwined_tpl_lst,
            } => {
                metamodelica::gc::MMTrace::mm_accept(entwined_slices, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(entwined_tpl_lst, __mmv)?;
                Ok(())
            }
            NBStrongComponent::ALGEBRAIC_LOOP {
                idx,
                strict,
                casual,
                linear,
                mixed,
                homotopy,
                status,
                implicitlyCreated,
            } => {
                metamodelica::gc::MMTrace::mm_accept(idx, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(strict, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(casual, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(linear, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(mixed, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(homotopy, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(status, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(implicitlyCreated, __mmv)?;
                Ok(())
            }
            NBStrongComponent::ALIAS { aliasInfo, original } => {
                metamodelica::gc::MMTrace::mm_accept(aliasInfo, __mmv)?;
                metamodelica::gc::MMTrace::mm_accept(original, __mmv)?;
                Ok(())
            }
        }
    }
}
impl Default for NBStrongComponent {
    fn default() -> Self {
        Self::ENTWINED_COMPONENT {
            entwined_slices: Default::default(),
            entwined_tpl_lst: Default::default(),
        }
    }
}
pub use self::NBStrongComponent::{
    ALGEBRAIC_LOOP, ALIAS, ENTWINED_COMPONENT, GENERIC_COMPONENT, MULTI_COMPONENT, RESIZABLE_COMPONENT,
    SINGLE_COMPONENT, SLICED_COMPONENT,
};
pub mod AliasInfo {
    use super::*;
    #[derive(Clone, Debug, Eq, Hash, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
    pub struct AliasInfo {
        /// The partition kind
        pub kind: BPartition::Kind,
        /// the partition index
        pub partitionIndex: i32,
        /// The index in that strong component array
        pub componentIndex: i32,
    }

    impl metamodelica::gc::MMTrace for AliasInfo {
        fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
            metamodelica::gc::MMTrace::mm_accept(&self.kind, __mmv)?;
            metamodelica::gc::MMTrace::mm_accept(&self.partitionIndex, __mmv)?;
            metamodelica::gc::MMTrace::mm_accept(&self.componentIndex, __mmv)?;
            Ok(())
        }
    }
    impl Default for AliasInfo {
        fn default() -> Self {
            Self {
                kind: Default::default(),
                partitionIndex: Default::default(),
                componentIndex: Default::default(),
            }
        }
    }

    pub type ALIAS_INFO = AliasInfo;

    pub(crate) fn toString(mut info: &metamodelica::Ref<AliasInfo>) -> Result<ArcStr> {
        let mut r#str: ArcStr = {
            let mut __mm_s = String::new();
            __mm_s.push_str(&*BPartition::Partition::kindToString(info.kind.clone())?);
            __mm_s.push_str(&*literal!("["));
            __mm_s.push_str(&*intString(info.partitionIndex.clone()));
            __mm_s.push_str(&*literal!(" | "));
            __mm_s.push_str(&*intString(info.componentIndex.clone()));
            __mm_s.push_str(&*literal!("]"));
            ArcStr::from(__mm_s)
        };
        Ok(r#str)
    }

    pub(crate) fn hash(mut info: &metamodelica::Ref<AliasInfo>) -> Result<i32> {
        let mut i: i32 = stringHashDjb2(&(toString(info)?));
        Ok(i)
    }

    pub(crate) fn isEqual(mut info1: &metamodelica::Ref<AliasInfo>, mut info2: &metamodelica::Ref<AliasInfo>) -> bool {
        let mut b: bool = info1.componentIndex.clone() == info2.componentIndex.clone()
            && info1.partitionIndex.clone() == info2.partitionIndex.clone()
            && info1.kind.clone() == info2.kind.clone();
        b
    }
}

pub(crate) fn toString(mut comp: &metamodelica::Ref<NBStrongComponent>, mut index: i32) -> Result<ArcStr> {
    let mut r#str: ArcStr;
    let mut s: i32 = size(comp, true)?;
    let mut indexStr: ArcStr = if (index > 0) {
        {
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!(" "));
            __mm_s.push_str(&*intString(index));
            ArcStr::from(__mm_s)
        }
    } else {
        literal!("")
    };
    r#str = (match &**comp {
        SINGLE_COMPONENT {
            eqn: __comp_eqn,
            status: __comp_status,
            var: __comp_var,
        } => {
            r#str = StringUtil::headline_3(
                &({
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!("BLOCK"));
                    __mm_s.push_str(&*indexStr);
                    __mm_s.push_str(&*literal!(": Single Strong Component (status = "));
                    __mm_s.push_str(&*Solve::statusString(__comp_status.clone()));
                    __mm_s.push_str(&*literal!(", size = "));
                    __mm_s.push_str(&*intString(s));
                    __mm_s.push_str(&*literal!(")"));
                    ArcStr::from(__mm_s)
                }),
            )?;
            r#str = {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*r#str);
                __mm_s.push_str(&*literal!("### Variable:\n"));
                __mm_s.push_str(&*Variable::toString(
                    &(Pointer::access(__comp_var.clone())),
                    literal!("\t"),
                    false,
                )?);
                __mm_s.push_str(&*literal!("\n"));
                ArcStr::from(__mm_s)
            };
            r#str = {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*r#str);
                __mm_s.push_str(&*literal!("### Equation:\n"));
                __mm_s.push_str(&*Equation::toString(
                    Pointer::access(__comp_eqn.clone()),
                    literal!("\t"),
                )?);
                __mm_s.push_str(&*literal!("\n"));
                ArcStr::from(__mm_s)
            };
            r#str
        }
        MULTI_COMPONENT {
            eqn: __comp_eqn,
            status: __comp_status,
            vars: __comp_vars,
        } => {
            r#str = StringUtil::headline_3(
                &({
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!("BLOCK"));
                    __mm_s.push_str(&*indexStr);
                    __mm_s.push_str(&*literal!(": Multi Strong Component (status = "));
                    __mm_s.push_str(&*Solve::statusString(__comp_status.clone()));
                    __mm_s.push_str(&*literal!(", size = "));
                    __mm_s.push_str(&*intString(s));
                    __mm_s.push_str(&*literal!(")"));
                    ArcStr::from(__mm_s)
                }),
            )?;
            r#str = {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*r#str);
                __mm_s.push_str(&*literal!("### Variables:\n"));
                ArcStr::from(__mm_s)
            };
            r#str = {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*r#str);
                __mm_s.push_str(&*List::toString(
                    __comp_vars.clone(),
                    &({
                        let __pe_b1 = (std::sync::Arc::new(BVariable::pointerToString)
                            as std::sync::Arc<
                                dyn ::std::ops::Fn(
                                        Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>,
                                    ) -> Result<ArcStr>
                                    + 'static,
                            >);
                        let __pe_b2 = 10;
                        move |__pe_a0| Slice::toString(__pe_a0, &*__pe_b1, __pe_b2.clone())
                    }),
                    List::Style::NEWLINE_TAB.clone(),
                )?);
                ArcStr::from(__mm_s)
            };
            r#str = {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*r#str);
                __mm_s.push_str(&*literal!("\n### Equation:\n"));
                __mm_s.push_str(&*Slice::toString(
                    __comp_eqn.clone(),
                    &({
                        let __pe_b1 = literal!("\t");
                        move |__pe_a0| Equation::pointerToString(__pe_a0, __pe_b1.clone())
                    }),
                    10,
                )?);
                __mm_s.push_str(&*literal!("\n"));
                ArcStr::from(__mm_s)
            };
            r#str
        }
        SLICED_COMPONENT {
            eqn: __comp_eqn,
            status: __comp_status,
            var_cref: __comp_var_cref,
            ..
        } => {
            r#str = if (index == -2) {
                literal!("")
            } else {
                StringUtil::headline_3(
                    &({
                        let mut __mm_s = String::new();
                        __mm_s.push_str(&*literal!("BLOCK"));
                        __mm_s.push_str(&*indexStr);
                        __mm_s.push_str(&*literal!(": Sliced Component (status = "));
                        __mm_s.push_str(&*Solve::statusString(__comp_status.clone()));
                        __mm_s.push_str(&*literal!(", size = "));
                        __mm_s.push_str(&*intString(s));
                        __mm_s.push_str(&*literal!(")"));
                        ArcStr::from(__mm_s)
                    }),
                )?
            };
            r#str = {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*r#str);
                __mm_s.push_str(&*literal!("### Variable:\n\t"));
                __mm_s.push_str(&*ComponentRef::toString(metamodelica::AsArg::as_arg(&__comp_var_cref))?);
                __mm_s.push_str(&*literal!("\n"));
                ArcStr::from(__mm_s)
            };
            r#str = {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*r#str);
                __mm_s.push_str(&*literal!("### Equation:\n"));
                __mm_s.push_str(&*Slice::toString(
                    __comp_eqn.clone(),
                    &({
                        let __pe_b1 = literal!("\t");
                        move |__pe_a0| Equation::pointerToString(__pe_a0, __pe_b1.clone())
                    }),
                    10,
                )?);
                __mm_s.push_str(&*literal!("\n"));
                ArcStr::from(__mm_s)
            };
            r#str
        }
        RESIZABLE_COMPONENT {
            eqn: __comp_eqn,
            status: __comp_status,
            var_cref: __comp_var_cref,
            ..
        } => {
            r#str = StringUtil::headline_3(
                &({
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!("BLOCK"));
                    __mm_s.push_str(&*indexStr);
                    __mm_s.push_str(&*literal!(": Resizable Component (status = "));
                    __mm_s.push_str(&*Solve::statusString(__comp_status.clone()));
                    __mm_s.push_str(&*literal!(", size = "));
                    __mm_s.push_str(&*intString(s));
                    __mm_s.push_str(&*literal!(")"));
                    ArcStr::from(__mm_s)
                }),
            )?;
            r#str = {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*r#str);
                __mm_s.push_str(&*literal!("### Variable:\n\t"));
                __mm_s.push_str(&*ComponentRef::toString(metamodelica::AsArg::as_arg(&__comp_var_cref))?);
                __mm_s.push_str(&*literal!("\n"));
                ArcStr::from(__mm_s)
            };
            r#str = {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*r#str);
                __mm_s.push_str(&*literal!("### Equation:\n"));
                __mm_s.push_str(&*Equation::pointerToString(
                    Slice::getT(__comp_eqn.clone()),
                    literal!("\t"),
                )?);
                __mm_s.push_str(&*literal!("\n"));
                ArcStr::from(__mm_s)
            };
            r#str
        }
        ENTWINED_COMPONENT {
            entwined_slices: __comp_entwined_slices,
            entwined_tpl_lst: __comp_entwined_tpl_lst,
        } => {
            r#str = StringUtil::headline_3(
                &({
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!("BLOCK"));
                    __mm_s.push_str(&*indexStr);
                    __mm_s.push_str(&*literal!(": Entwined Component (status = Solve.EXPLICIT, size = "));
                    __mm_s.push_str(&*intString(s));
                    __mm_s.push_str(&*literal!(")"));
                    ArcStr::from(__mm_s)
                }),
            )?;
            r#str = {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*r#str);
                __mm_s.push_str(&*literal!("call order: "));
                __mm_s.push_str(&*List::toString(
                    ({
                        let mut __acc: metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>> =
                            metamodelica::nil();
                        for mut e in (__comp_entwined_tpl_lst.clone()).into_iter().cloned() {
                            let __x = Equation::getEqnName(Util::tuple21(e.clone()))?;
                            __acc = cons(__x, __acc);
                        }
                        __acc.reverse()
                    }),
                    &move |__a0: metamodelica::Ref<ComponentRef::NFComponentRef>| ComponentRef::toString(&__a0),
                    List::Style::FLAT_CURLY_SHORT.clone(),
                )?);
                __mm_s.push_str(&*literal!("\n"));
                ArcStr::from(__mm_s)
            };
            r#str = {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*r#str);
                __mm_s.push_str(&*List::toString(
                    __comp_entwined_slices.clone(),
                    &({
                        let __pe_b1 = -2;
                        move |__pe_a0| toString(&__pe_a0, __pe_b1.clone())
                    }),
                    List::Style::NONE.clone(),
                )?);
                ArcStr::from(__mm_s)
            };
            r#str
        }
        GENERIC_COMPONENT {
            eqn: __comp_eqn,
            var_cref: __comp_var_cref,
            ..
        } => {
            r#str = StringUtil::headline_3(
                &({
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!("BLOCK"));
                    __mm_s.push_str(&*indexStr);
                    __mm_s.push_str(&*literal!(": Generic Component (status = Solve.EXPLICIT, size = "));
                    __mm_s.push_str(&*intString(s));
                    __mm_s.push_str(&*literal!(")"));
                    ArcStr::from(__mm_s)
                }),
            )?;
            r#str = {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*r#str);
                __mm_s.push_str(&*literal!("### Variable:\n\t"));
                __mm_s.push_str(&*ComponentRef::toString(metamodelica::AsArg::as_arg(&__comp_var_cref))?);
                __mm_s.push_str(&*literal!("\n"));
                ArcStr::from(__mm_s)
            };
            r#str = {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*r#str);
                __mm_s.push_str(&*literal!("### Equation:\n"));
                __mm_s.push_str(&*Slice::toString(
                    __comp_eqn.clone(),
                    &({
                        let __pe_b1 = literal!("\t");
                        move |__pe_a0| Equation::pointerToString(__pe_a0, __pe_b1.clone())
                    }),
                    10,
                )?);
                __mm_s.push_str(&*literal!("\n"));
                ArcStr::from(__mm_s)
            };
            r#str
        }
        ALGEBRAIC_LOOP {
            casual: __comp_casual,
            homotopy: __comp_homotopy,
            linear: __comp_linear,
            mixed: __comp_mixed,
            strict: __comp_strict,
            ..
        } => {
            r#str = StringUtil::headline_3(
                &({
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!("BLOCK"));
                    __mm_s.push_str(&*indexStr);
                    __mm_s.push_str(&*literal!(": Algebraic Loop (Linear = "));
                    __mm_s.push_str(&*boolString(__comp_linear.clone()));
                    __mm_s.push_str(&*literal!(", Mixed = "));
                    __mm_s.push_str(&*boolString(__comp_mixed.clone()));
                    __mm_s.push_str(&*literal!(", Homotopy = "));
                    __mm_s.push_str(&*boolString(__comp_homotopy.clone()));
                    __mm_s.push_str(&*literal!(", size = "));
                    __mm_s.push_str(&*intString(s));
                    __mm_s.push_str(&*literal!(")"));
                    ArcStr::from(__mm_s)
                }),
            )?;
            r#str = {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*r#str);
                __mm_s.push_str(&*Tearing::toString(
                    metamodelica::AsArg::as_arg(&__comp_strict),
                    literal!("Strict Tearing Set"),
                )?);
                ArcStr::from(__mm_s)
            };
            if (__comp_casual).is_some() {
                r#str = {
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*r#str);
                    __mm_s.push_str(&*Tearing::toString(
                        &(Util::getOption(__comp_casual.clone())?),
                        literal!("Casual Tearing Set"),
                    )?);
                    ArcStr::from(__mm_s)
                };
            }
            r#str
        }
        ALIAS {
            aliasInfo: __comp_aliasInfo,
            original: __comp_original,
        } => {
            r#str = {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("--- Alias of "));
                __mm_s.push_str(&*AliasInfo::toString(metamodelica::AsArg::as_arg(&__comp_aliasInfo))?);
                __mm_s.push_str(&*literal!(" ---\n"));
                __mm_s.push_str(&*toString(metamodelica::AsArg::as_arg(&__comp_original), index)?);
                ArcStr::from(__mm_s)
            };
            r#str
        }
        _ => {
            Error::addMessage(
                Error::INTERNAL_ERROR.clone(),
                list![{
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!("NBStrongComponent.toString"));
                    __mm_s.push_str(&*literal!(" failed."));
                    ArcStr::from(__mm_s)
                }],
            )?;
            return Err("fail");
        }
    });
    Ok(r#str)
}

#[derive(Clone, Copy, Debug, Eq, Hash, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub struct CountCollector {
    pub single_scalar: i32,
    pub single_array: i32,
    pub single_record: i32,
    pub multi_algorithm: i32,
    pub multi_when: i32,
    pub multi_if: i32,
    pub multi_tpl: i32,
    pub resizable_for: i32,
    pub generic_for: i32,
    pub entwined_for: i32,
    pub loop_lin: i32,
    pub loop_nlin: i32,
}

impl metamodelica::gc::MMTrace for CountCollector {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        metamodelica::gc::MMTrace::mm_accept(&self.single_scalar, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.single_array, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.single_record, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.multi_algorithm, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.multi_when, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.multi_if, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.multi_tpl, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.resizable_for, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.generic_for, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.entwined_for, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.loop_lin, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.loop_nlin, __mmv)?;
        Ok(())
    }
}
pub type COUNT_COLLECTOR = CountCollector;

pub(crate) fn strongComponentInfo(
    mut comp: metamodelica::Ref<NBStrongComponent>,
    mut collector_ptr: Pointer::Pointer<CountCollector>,
) -> Result<metamodelica::Ref<NBStrongComponent>> {
    let mut comp: metamodelica::Ref<NBStrongComponent> = comp;
    let mut collector: CountCollector = Pointer::access(collector_ptr.clone());
    let () = (match &*comp {
        SINGLE_COMPONENT { eqn: __comp_eqn, .. } => {
            let () = (match &*(Pointer::access(__comp_eqn.clone())) {
                Equation::SCALAR_EQUATION { .. } => {
                    collector.single_scalar = collector.single_scalar.clone() + 1;
                    Pointer::update(collector_ptr, collector);
                    ()
                }
                Equation::ARRAY_EQUATION { .. } => {
                    collector.single_array = collector.single_array.clone() + 1;
                    Pointer::update(collector_ptr, collector);
                    ()
                }
                Equation::RECORD_EQUATION { .. } => {
                    collector.single_record = collector.single_record.clone() + 1;
                    Pointer::update(collector_ptr, collector);
                    ()
                }
                _ => {
                    Error::addCompilerWarning({
                        let mut __mm_s = String::new();
                        __mm_s.push_str(&*literal!("Cannot classify strong component:\n"));
                        __mm_s.push_str(&*toString(&comp, -1)?);
                        __mm_s.push_str(&*literal!("\n"));
                        ArcStr::from(__mm_s)
                    })?;
                    ()
                }
            });
            ()
        }
        MULTI_COMPONENT { eqn: __comp_eqn, .. } => {
            let () = (match &*(Pointer::access(Slice::getT(__comp_eqn.clone()))) {
                Equation::ALGORITHM { .. } => {
                    collector.multi_algorithm = collector.multi_algorithm.clone() + 1;
                    Pointer::update(collector_ptr, collector);
                    ()
                }
                Equation::WHEN_EQUATION { .. } => {
                    collector.multi_when = collector.multi_when.clone() + 1;
                    Pointer::update(collector_ptr, collector);
                    ()
                }
                Equation::IF_EQUATION { .. } => {
                    collector.multi_if = collector.multi_if.clone() + 1;
                    Pointer::update(collector_ptr, collector);
                    ()
                }
                Equation::RECORD_EQUATION { .. } => {
                    collector.multi_tpl = collector.multi_tpl.clone() + 1;
                    Pointer::update(collector_ptr, collector);
                    ()
                }
                _ => {
                    Error::addCompilerWarning({
                        let mut __mm_s = String::new();
                        __mm_s.push_str(&*literal!("Cannot classify strong component:\n"));
                        __mm_s.push_str(&*toString(&comp, -1)?);
                        __mm_s.push_str(&*literal!("\n"));
                        ArcStr::from(__mm_s)
                    })?;
                    ()
                }
            });
            ()
        }
        SLICED_COMPONENT { eqn: __comp_eqn, .. } => {
            let () = (match &*(Pointer::access(Slice::getT(__comp_eqn.clone()))) {
                Equation::SCALAR_EQUATION { .. } => {
                    collector.single_scalar = collector.single_scalar.clone() + 1;
                    Pointer::update(collector_ptr, collector);
                    ()
                }
                Equation::ARRAY_EQUATION { .. } => {
                    collector.single_array = collector.single_array.clone() + 1;
                    Pointer::update(collector_ptr, collector);
                    ()
                }
                Equation::RECORD_EQUATION { .. } => {
                    collector.single_record = collector.single_record.clone() + 1;
                    Pointer::update(collector_ptr, collector);
                    ()
                }
                _ => {
                    Error::addCompilerWarning({
                        let mut __mm_s = String::new();
                        __mm_s.push_str(&*literal!("Cannot classify strong component:\n"));
                        __mm_s.push_str(&*toString(&comp, -1)?);
                        __mm_s.push_str(&*literal!("\n"));
                        ArcStr::from(__mm_s)
                    })?;
                    ()
                }
            });
            ()
        }
        RESIZABLE_COMPONENT { .. } => {
            collector.resizable_for = collector.resizable_for.clone() + 1;
            Pointer::update(collector_ptr, collector);
            ()
        }
        GENERIC_COMPONENT { .. } => {
            collector.generic_for = collector.generic_for.clone() + 1;
            Pointer::update(collector_ptr, collector);
            ()
        }
        ENTWINED_COMPONENT { .. } => {
            collector.entwined_for = collector.entwined_for.clone() + 1;
            Pointer::update(collector_ptr, collector);
            ()
        }
        ALGEBRAIC_LOOP {
            linear: __comp_linear, ..
        } if (__comp_linear.clone()) => {
            collector.loop_lin = collector.loop_lin.clone() + 1;
            Pointer::update(collector_ptr, collector);
            ()
        }
        ALGEBRAIC_LOOP { .. } => {
            collector.loop_nlin = collector.loop_nlin.clone() + 1;
            Pointer::update(collector_ptr, collector);
            ()
        }
        ALIAS {
            original: __comp_original,
            ..
        } => {
            strongComponentInfo(__comp_original.clone(), collector_ptr)?;
            ()
        }
        _ => {
            Error::addCompilerWarning({
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("Cannot classify strong component:\n"));
                __mm_s.push_str(&*toString(&comp, -1)?);
                __mm_s.push_str(&*literal!("\n"));
                ArcStr::from(__mm_s)
            })?;
            ()
        }
    });
    Ok(comp)
}

pub(crate) fn hash(mut comp: &metamodelica::Ref<NBStrongComponent>) -> Result<i32> {
    let mut i: i32;
    i = (match &**comp {
        SINGLE_COMPONENT {
            eqn: __comp_eqn,
            var: __comp_var,
            ..
        } => BVariable::hash(__comp_var.clone())? + Equation::hash(__comp_eqn.clone())?,
        MULTI_COMPONENT { eqn: __comp_eqn, .. } => Equation::hash(Slice::getT(__comp_eqn.clone()))?,
        SLICED_COMPONENT {
            eqn: __comp_eqn,
            var_cref: __comp_var_cref,
            ..
        } => {
            ComponentRef::hash(metamodelica::AsArg::as_arg(&__comp_var_cref))?
                + Equation::hash(Slice::getT(__comp_eqn.clone()))?
        }
        RESIZABLE_COMPONENT {
            eqn: __comp_eqn,
            var_cref: __comp_var_cref,
            ..
        } => {
            ComponentRef::hash(metamodelica::AsArg::as_arg(&__comp_var_cref))?
                + Equation::hash(Slice::getT(__comp_eqn.clone()))?
        }
        GENERIC_COMPONENT { eqn: __comp_eqn, .. } => Equation::hash(Slice::getT(__comp_eqn.clone()))?,
        ENTWINED_COMPONENT {
            entwined_slices: __comp_entwined_slices,
            ..
        } => {
            ({
                let mut __acc: i32 = 0;
                for mut sub_comp in (__comp_entwined_slices.clone()).into_iter().cloned() {
                    let __x = hash(&(sub_comp.clone()))?;
                    __acc += __x;
                }
                __acc
            })
        }
        ALGEBRAIC_LOOP {
            strict: __comp_strict, ..
        } => Tearing::hash(metamodelica::AsArg::as_arg(&__comp_strict))?,
        ALIAS {
            aliasInfo: __comp_aliasInfo,
            ..
        } => AliasInfo::hash(metamodelica::AsArg::as_arg(&__comp_aliasInfo))?,
        _ => {
            Error::addMessage(
                Error::INTERNAL_ERROR.clone(),
                list![{
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!("NBStrongComponent.hash"));
                    __mm_s.push_str(&*literal!(" failed."));
                    ArcStr::from(__mm_s)
                }],
            )?;
            return Err("fail");
        }
    });
    Ok(i)
}

pub(crate) fn isEqual(
    mut comp1: &metamodelica::Ref<NBStrongComponent>,
    mut comp2: &metamodelica::Ref<NBStrongComponent>,
) -> Result<bool> {
    let mut b: bool;
    b = (::match_deref::match_deref! { match (comp1, comp2) {
        (Deref @ SINGLE_COMPONENT { .. }, Deref @ SINGLE_COMPONENT { .. }) => BVariable::equalName(var_field!((**comp1).var, NBStrongComponent::SINGLE_COMPONENT).clone(), var_field!((**comp2).var, NBStrongComponent::SINGLE_COMPONENT).clone())? && Equation::isEqualPtr(var_field!((**comp1).eqn, NBStrongComponent::SINGLE_COMPONENT).clone(), var_field!((**comp2).eqn, NBStrongComponent::SINGLE_COMPONENT).clone())?,
        (Deref @ MULTI_COMPONENT { .. }, Deref @ MULTI_COMPONENT { .. }) => Equation::isEqualPtr(Slice::getT(var_field!((**comp1).eqn, NBStrongComponent::MULTI_COMPONENT).clone()), Slice::getT(var_field!((**comp2).eqn, NBStrongComponent::MULTI_COMPONENT).clone()))?,
        (Deref @ SLICED_COMPONENT { .. }, Deref @ SLICED_COMPONENT { .. }) => ComponentRef::isEqual(var_field!((**comp1).var_cref, NBStrongComponent::SLICED_COMPONENT), var_field!((**comp2).var_cref, NBStrongComponent::SLICED_COMPONENT))? && Slice::isEqual(var_field!((**comp1).eqn, NBStrongComponent::SLICED_COMPONENT).clone(), var_field!((**comp2).eqn, NBStrongComponent::SLICED_COMPONENT).clone(), &Equation::isEqualPtr)?,
        (Deref @ RESIZABLE_COMPONENT { .. }, Deref @ RESIZABLE_COMPONENT { .. }) => ComponentRef::isEqual(var_field!((**comp1).var_cref, NBStrongComponent::RESIZABLE_COMPONENT), var_field!((**comp2).var_cref, NBStrongComponent::RESIZABLE_COMPONENT))? && Slice::isEqual(var_field!((**comp1).eqn, NBStrongComponent::RESIZABLE_COMPONENT).clone(), var_field!((**comp2).eqn, NBStrongComponent::RESIZABLE_COMPONENT).clone(), &Equation::isEqualPtr)?,
        (Deref @ GENERIC_COMPONENT { .. }, Deref @ GENERIC_COMPONENT { .. }) => Slice::isEqual(var_field!((**comp1).eqn, NBStrongComponent::GENERIC_COMPONENT).clone(), var_field!((**comp2).eqn, NBStrongComponent::GENERIC_COMPONENT).clone(), &Equation::isEqualPtr)?,
        (Deref @ ENTWINED_COMPONENT { .. }, Deref @ ENTWINED_COMPONENT { .. }) => List::isEqualOnTrue(var_field!((**comp1).entwined_slices, NBStrongComponent::ENTWINED_COMPONENT).clone(), var_field!((**comp2).entwined_slices, NBStrongComponent::ENTWINED_COMPONENT).clone(), &move |__a0: metamodelica::Ref<NBStrongComponent>, __a1: metamodelica::Ref<NBStrongComponent>| isEqual(&__a0, &__a1))?,
        (Deref @ ALGEBRAIC_LOOP { .. }, Deref @ ALGEBRAIC_LOOP { .. }) => Tearing::isEqual(var_field!((**comp1).strict, NBStrongComponent::ALGEBRAIC_LOOP), var_field!((**comp2).strict, NBStrongComponent::ALGEBRAIC_LOOP))?,
        (Deref @ ALIAS { .. }, Deref @ ALIAS { .. }) => AliasInfo::isEqual(var_field!((**comp1).aliasInfo, NBStrongComponent::ALIAS), var_field!((**comp2).aliasInfo, NBStrongComponent::ALIAS)),
        _ => false,
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(b)
}

pub(crate) fn size(mut comp: &metamodelica::Ref<NBStrongComponent>, mut resize: bool) -> Result<i32> {
    let mut s: i32;
    s = (match &**comp {
        SINGLE_COMPONENT { eqn: __comp_eqn, .. } => Equation::size(__comp_eqn.clone(), resize)?,
        MULTI_COMPONENT { eqn: __comp_eqn, .. } => Slice::size(
            __comp_eqn.clone(),
            &({
                let __pe_b1 = resize;
                move |__pe_a0| Equation::size(__pe_a0, __pe_b1.clone())
            }),
        )?,
        SLICED_COMPONENT { eqn: __comp_eqn, .. } => Slice::size(
            __comp_eqn.clone(),
            &({
                let __pe_b1 = resize;
                move |__pe_a0| Equation::size(__pe_a0, __pe_b1.clone())
            }),
        )?,
        RESIZABLE_COMPONENT { eqn: __comp_eqn, .. } => Slice::size(
            __comp_eqn.clone(),
            &({
                let __pe_b1 = resize;
                move |__pe_a0| Equation::size(__pe_a0, __pe_b1.clone())
            }),
        )?,
        GENERIC_COMPONENT { eqn: __comp_eqn, .. } => Slice::size(
            __comp_eqn.clone(),
            &({
                let __pe_b1 = resize;
                move |__pe_a0| Equation::size(__pe_a0, __pe_b1.clone())
            }),
        )?,
        ENTWINED_COMPONENT {
            entwined_slices: __comp_entwined_slices,
            ..
        } => {
            ({
                let mut __acc: i32 = 0;
                for mut c in (__comp_entwined_slices.clone()).into_iter().cloned() {
                    let __x = size(&(c.clone()), resize)?;
                    __acc += __x;
                }
                __acc
            })
        }
        ALGEBRAIC_LOOP {
            strict: __comp_strict, ..
        } => Tearing::size(metamodelica::AsArg::as_arg(&__comp_strict), resize)?,
        ALIAS {
            original: __comp_original,
            ..
        } => size(metamodelica::AsArg::as_arg(&__comp_original), resize)?,
        _ => {
            Error::addMessage(
                Error::INTERNAL_ERROR.clone(),
                list![{
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!("NBStrongComponent.size"));
                    __mm_s.push_str(&*literal!(" failed. Cannot determine size of strong component:\n"));
                    __mm_s.push_str(&*toString(comp, -1)?);
                    __mm_s.push_str(&*literal!("\n"));
                    ArcStr::from(__mm_s)
                }],
            )?;
            return Err("fail");
        }
    });
    Ok(s)
}

pub(crate) fn removeAlias(mut comp: metamodelica::Ref<NBStrongComponent>) -> metamodelica::Ref<NBStrongComponent> {
    let mut comp: metamodelica::Ref<NBStrongComponent> = comp;
    comp = (match &*comp {
        ALIAS {
            original: __comp_original,
            ..
        } => __comp_original.clone(),
        _ => comp,
    });
    comp
}

pub(crate) fn createPseudoSlice(
    mut var_arr_idx: i32,
    mut eqn_arr_idx: i32,
    mut cref_to_solve: metamodelica::Ref<ComponentRef::NFComponentRef>,
    mut eqn_scal_indices: metamodelica::List<i32>,
    mut eqn_to_var: metamodelica::Array<i32>,
    mut eqns: &metamodelica::Ref<EquationPointers::EquationPointers>,
    mut mapping: &metamodelica::Ref<Mapping::Mapping>,
    mut independent: bool,
) -> Result<metamodelica::Ref<NBStrongComponent>> {
    let __ab_eqn_to_var = eqn_to_var.borrow();
    let mut comp: metamodelica::Ref<NBStrongComponent>;
    let mut var_ptr: Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>;
    let mut eqn_ptr: Pointer::Pointer<metamodelica::Ref<Equation::Equation>>;
    let mut first_var: i32;
    let mut var_size: i32;
    let mut first_eqn: i32;
    let mut eqn_size: i32;
    let mut var_slice: metamodelica::Ref<Slice::NBSlice<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>>>;
    let mut eqn_slice: metamodelica::Ref<Slice::NBSlice<Pointer::Pointer<metamodelica::Ref<Equation::Equation>>>>;
    let mut var_scal_indices: metamodelica::List<i32>;
    let mut order: metamodelica::Ref<
        UnorderedMap::UnorderedMap<metamodelica::Ref<ComponentRef::NFComponentRef>, EvalOrder>,
    >;
    var_ptr = BVariable::getVarPointer(
        &cref_to_solve,
        metamodelica::sourceInfo!("NBackEnd/Classes/NBStrongComponent.mo"),
    )?;
    eqn_ptr = EquationPointers::getEqnAt(eqns, eqn_arr_idx)?;
    (first_var, var_size) = ({
        let __elt = (*metamodelica::index_checked(&mapping.var_AtS.borrow(), var_arr_idx)?).clone();
        __elt
    });
    (first_eqn, eqn_size) = ({
        let __elt = (*metamodelica::index_checked(&mapping.eqn_AtS.borrow(), eqn_arr_idx)?).clone();
        __elt
    });
    var_scal_indices = ({
        let mut __acc: metamodelica::List<i32> = metamodelica::nil();
        for mut e in (eqn_scal_indices.clone()).into_iter().cloned() {
            let __x = (*metamodelica::index_checked(&__ab_eqn_to_var, e.clone())?).clone();
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    if independent
        && Equation::isArrayEquation(eqn_ptr.clone())
        && ((eqn_scal_indices).len() as i32) == eqn_size
        && ((var_scal_indices).len() as i32) == var_size
    {
        var_slice = metamodelica::Ref::new(Slice::NBSlice {
            t: var_ptr,
            indices: metamodelica::nil(),
        });
        eqn_slice = metamodelica::Ref::new(Slice::NBSlice {
            t: eqn_ptr.clone(),
            indices: metamodelica::nil(),
        });
    } else {
        var_slice = metamodelica::Ref::new(Slice::NBSlice {
            t: var_ptr,
            indices: ({
                let mut __acc: metamodelica::List<i32> = metamodelica::nil();
                for mut idx in (var_scal_indices).into_iter().cloned() {
                    let __x = idx.clone() - first_var;
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            }),
        });
        eqn_slice = metamodelica::Ref::new(Slice::NBSlice {
            t: eqn_ptr.clone(),
            indices: ({
                let mut __acc: metamodelica::List<i32> = metamodelica::nil();
                for mut idx in (eqn_scal_indices.clone()).into_iter().cloned() {
                    let __x = idx.clone() - first_eqn;
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            }),
        });
    }
    order = Resizable::detect(&(Pointer::access(eqn_ptr)), &cref_to_solve)?;
    if !(List::any(
        &(UnorderedMap::valueList(order.clone())),
        &fnptr!(Resizable::orderFailed, EvalOrder),
    )?) && ((eqn_scal_indices).len() as i32) == eqn_size
    {
        comp = metamodelica::Ref::new(NBStrongComponent::RESIZABLE_COMPONENT {
            var_cref: cref_to_solve,
            var: var_slice,
            eqn: eqn_slice,
            order: order,
            status: Solve::Status::UNPROCESSED.clone(),
        });
    } else {
        comp = createSliceOrSingle(cref_to_solve, var_slice, eqn_slice)?;
    }
    Ok(comp)
}

pub(crate) fn createPseudoEntwined(
    mut eqn_indices: metamodelica::List<i32>,
    mut eqn_to_var: metamodelica::Array<i32>,
    mut mapping: &metamodelica::Ref<Mapping::Mapping>,
    mut vars: &metamodelica::Ref<VariablePointers::VariablePointers>,
    mut eqns: &metamodelica::Ref<EquationPointers::EquationPointers>,
    mut nodes: &metamodelica::List<metamodelica::Ref<SuperNode::SuperNode>>,
) -> Result<metamodelica::Ref<NBStrongComponent>> {
    let mut entwined: metamodelica::Ref<NBStrongComponent>;
    let mut elem_map: metamodelica::Ref<UnorderedMap::UnorderedMap<i32, metamodelica::List<i32>>> = UnorderedMap::new(
        std::sync::Arc::new(fnptr!(Util::id, _)),
        (std::sync::Arc::new(fnptr!(intEq, i32, i32))
            as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>),
        1,
    );
    let mut cref_map: metamodelica::Ref<
        UnorderedMap::UnorderedMap<i32, metamodelica::Ref<ComponentRef::NFComponentRef>>,
    > = UnorderedMap::new(
        std::sync::Arc::new(fnptr!(Util::id, _)),
        (std::sync::Arc::new(fnptr!(intEq, i32, i32))
            as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>),
        1,
    );
    let mut eqn_arr_idx: i32;
    let mut var_arr_idx: i32;
    let mut scal_indices: metamodelica::List<i32>;
    let mut entwined_slices: metamodelica::List<metamodelica::Ref<NBStrongComponent>> = metamodelica::nil();
    let mut entwined_tpl_lst: metamodelica::List<(Pointer::Pointer<metamodelica::Ref<Equation::Equation>>, i32)>;
    for mut idx in &*eqn_indices {
        UnorderedMap::add(
            ({
                let __elt = (*metamodelica::index_checked(&mapping.eqn_StA.borrow(), idx.clone())?).clone();
                __elt
            }),
            metamodelica::cons(
                idx.clone(),
                UnorderedMap::getOrDefault(
                    ({
                        let __elt = (*metamodelica::index_checked(&mapping.eqn_StA.borrow(), idx.clone())?).clone();
                        __elt
                    }),
                    elem_map.clone(),
                    metamodelica::nil(),
                )?,
            ),
            elem_map.clone(),
        )?;
    }
    for mut node in &**nodes {
        let () = (match &*node.clone() {
            Sorting::SuperNode::ARRAY_BUCKET {
                arr_idx: __node_arr_idx,
                cref_to_solve: __node_cref_to_solve,
                ..
            } => {
                UnorderedMap::add(__node_arr_idx.clone(), __node_cref_to_solve.clone(), cref_map.clone())?;
                ()
            }
            _ => (),
        });
    }
    for mut tpl in &*UnorderedMap::toList(elem_map) {
        (eqn_arr_idx, scal_indices) = tpl.clone();
        if UnorderedMap::contains(eqn_arr_idx, cref_map.clone())? {
            var_arr_idx = ({
                let __elt = (*metamodelica::index_checked(
                    &mapping.var_StA.borrow(),
                    ({
                        let __elt = (*metamodelica::index_checked(
                            &eqn_to_var.borrow(),
                            Util::tuple21(
                                ({
                                    let __elt =
                                        (*metamodelica::index_checked(&mapping.eqn_AtS.borrow(), eqn_arr_idx)?).clone();
                                    __elt
                                }),
                            ),
                        )?)
                        .clone();
                        __elt
                    }),
                )?)
                .clone();
                __elt
            });
            entwined_slices = metamodelica::cons(
                createPseudoSlice(
                    var_arr_idx,
                    eqn_arr_idx,
                    UnorderedMap::getSafe(
                        eqn_arr_idx,
                        cref_map.clone(),
                        metamodelica::sourceInfo!("NBackEnd/Classes/NBStrongComponent.mo"),
                    )?,
                    scal_indices,
                    eqn_to_var.clone(),
                    eqns,
                    mapping,
                    false,
                )?,
                entwined_slices,
            );
        } else {
            entwined_slices = metamodelica::cons(
                createPseudoScalar(&scal_indices, eqn_to_var.clone(), mapping, vars, eqns)?,
                entwined_slices,
            );
        }
    }
    entwined_tpl_lst = ({
        let mut __acc: metamodelica::List<(Pointer::Pointer<metamodelica::Ref<Equation::Equation>>, i32)> =
            metamodelica::nil();
        for mut idx in (eqn_indices).into_iter().cloned() {
            let __x = (
                EquationPointers::getEqnAt(
                    eqns,
                    ({
                        let __elt = (*metamodelica::index_checked(&mapping.eqn_StA.borrow(), idx.clone())?).clone();
                        __elt
                    }),
                )?,
                idx.clone(),
            );
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    entwined = metamodelica::Ref::new(NBStrongComponent::ENTWINED_COMPONENT {
        entwined_slices: entwined_slices,
        entwined_tpl_lst: entwined_tpl_lst,
    });
    Ok(entwined)
}

pub(crate) fn createAlias(
    mut kind: BPartition::Kind,
    mut partitionIndex: i32,
    mut index_ptr: Pointer::Pointer<i32>,
    mut orig_comp: metamodelica::Ref<NBStrongComponent>,
) -> metamodelica::Ref<NBStrongComponent> {
    let mut alias_comp: metamodelica::Ref<NBStrongComponent>;
    alias_comp = metamodelica::Ref::new(NBStrongComponent::ALIAS {
        aliasInfo: metamodelica::Ref::new(AliasInfo::AliasInfo {
            kind: kind,
            partitionIndex: partitionIndex,
            componentIndex: Pointer::access(index_ptr.clone()),
        }),
        original: orig_comp,
    });
    Pointer::update(index_ptr.clone(), Pointer::access(index_ptr) + 1);
    alias_comp
}

pub(crate) fn createPseudoEntwinedIndices(
    mut entwined_indices: metamodelica::Array<metamodelica::List<i32>>,
    mut eqns: &metamodelica::Ref<EquationPointers::EquationPointers>,
    mut mapping: &metamodelica::Ref<Mapping::Mapping>,
) -> Result<metamodelica::List<(Pointer::Pointer<metamodelica::Ref<Equation::Equation>>, i32)>> {
    let mut flat_tpl_indices: metamodelica::List<(Pointer::Pointer<metamodelica::Ref<Equation::Equation>>, i32)> =
        metamodelica::nil();
    let mut arr_idx: i32;
    let mut first_idx: i32;
    let mut eqn_StA: metamodelica::Array<i32>;
    let __range0 = entwined_indices.clone().borrow().iter().cloned().collect::<Vec<_>>();
    for mut tmp in __range0 {
        for mut scal_idx in &*tmp {
            eqn_StA = mapping.eqn_StA.clone();
            arr_idx = ({
                let __elt = (*metamodelica::index_checked(&eqn_StA.borrow(), scal_idx.clone())?).clone();
                __elt
            });
            (first_idx, _) = ({
                let __elt = (*metamodelica::index_checked(&mapping.eqn_AtS.borrow(), arr_idx)?).clone();
                __elt
            });
            flat_tpl_indices = metamodelica::cons(
                (EquationPointers::getEqnAt(eqns, arr_idx)?, scal_idx.clone() - first_idx),
                flat_tpl_indices,
            );
        }
    }
    flat_tpl_indices = flat_tpl_indices.reverse();
    Ok(flat_tpl_indices)
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, metamodelica::ReferenceEq)]
#[repr(i32)]
pub enum DAEType {
    UNPROCESSED = 1,
    REMOVED = 2,
    INNER = 3,
    RESIDUAL = 4,
}
impl PartialOrd for DAEType {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for DAEType {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (*self as i32).cmp(&(*other as i32))
    }
}
impl metamodelica::gc::MMTrace for DAEType {
    fn mm_accept(&self, _: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        Ok(())
    }
}

pub(crate) fn sortDAEModeComponents(
    mut comps: Option<metamodelica::Array<metamodelica::Ref<NBStrongComponent>>>,
    mut variables: &metamodelica::Ref<VariablePointers::VariablePointers>,
    mut uniqueIndex: Pointer::Pointer<i32>,
) -> Result<Option<metamodelica::Array<metamodelica::Ref<NBStrongComponent>>>> {
    let mut comps: Option<metamodelica::Array<metamodelica::Ref<NBStrongComponent>>> = comps;
    let mut residuals: metamodelica::List<metamodelica::Ref<NBStrongComponent>> = metamodelica::nil();
    let mut inners: metamodelica::List<metamodelica::Ref<NBStrongComponent>> = metamodelica::nil();
    let mut slice_set: metamodelica::Ref<UnorderedSet::UnorderedSet<metamodelica::Ref<ComponentRef::NFComponentRef>>> =
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
    comps = (match comps.clone() {
        Some(mut original) => {
            let __range0 = original.clone().borrow().iter().cloned().collect::<Vec<_>>();
            for mut comp in __range0 {
                (residuals, inners) = sortDAEModeComponent(
                    comp,
                    residuals,
                    inners,
                    variables,
                    uniqueIndex.clone(),
                    slice_set.clone(),
                )?;
            }
            comps = Some(metamodelica::arrayFromVec(
                listAppend(inners.reverse(), residuals).into_iter().cloned().collect(),
            ));
            comps
        }
        _ => comps,
    });
    Ok(comps)
}

pub(crate) fn sortDAEModeComponent(
    mut comp: metamodelica::Ref<NBStrongComponent>,
    mut residuals: metamodelica::List<metamodelica::Ref<NBStrongComponent>>,
    mut inners: metamodelica::List<metamodelica::Ref<NBStrongComponent>>,
    mut variables: &metamodelica::Ref<VariablePointers::VariablePointers>,
    mut uniqueIndex: Pointer::Pointer<i32>,
    mut slice_set: metamodelica::Ref<UnorderedSet::UnorderedSet<metamodelica::Ref<ComponentRef::NFComponentRef>>>,
) -> Result<(
    metamodelica::List<metamodelica::Ref<NBStrongComponent>>,
    metamodelica::List<metamodelica::Ref<NBStrongComponent>>,
)> {
    let mut residuals: metamodelica::List<metamodelica::Ref<NBStrongComponent>> = residuals;
    let mut inners: metamodelica::List<metamodelica::Ref<NBStrongComponent>> = inners;
    let mut new_residuals: metamodelica::List<metamodelica::Ref<NBStrongComponent>>;
    let mut dae_type: DAEType;
    (new_residuals, dae_type) = (match &*comp {
        SINGLE_COMPONENT { eqn: __comp_eqn, .. } => {
            (new_residuals, dae_type) = singleDAEModeComponent(__comp_eqn.clone(), variables, uniqueIndex)?;
            (new_residuals, dae_type)
        }
        MULTI_COMPONENT {
            eqn: __comp_eqn,
            vars: __comp_vars,
            ..
        } => {
            (new_residuals, dae_type) = slicedDAEModeComponent(
                __comp_vars.clone(),
                &(list![__comp_eqn.clone()]),
                variables,
                uniqueIndex,
                slice_set,
            )?;
            (new_residuals, dae_type)
        }
        SLICED_COMPONENT {
            eqn: __comp_eqn,
            var: __comp_var,
            ..
        } => {
            (new_residuals, dae_type) = slicedDAEModeComponent(
                list![__comp_var.clone()],
                &(list![__comp_eqn.clone()]),
                variables,
                uniqueIndex,
                slice_set,
            )?;
            (new_residuals, dae_type)
        }
        RESIZABLE_COMPONENT {
            eqn: __comp_eqn,
            var: __comp_var,
            ..
        } => {
            (new_residuals, dae_type) = slicedDAEModeComponent(
                list![__comp_var.clone()],
                &(list![__comp_eqn.clone()]),
                variables,
                uniqueIndex,
                slice_set,
            )?;
            (new_residuals, dae_type)
        }
        GENERIC_COMPONENT {
            eqn: __comp_eqn,
            var: __comp_var,
            ..
        } => {
            (new_residuals, dae_type) = slicedDAEModeComponent(
                list![__comp_var.clone()],
                &(list![__comp_eqn.clone()]),
                variables,
                uniqueIndex,
                slice_set,
            )?;
            (new_residuals, dae_type)
        }
        ALGEBRAIC_LOOP {
            strict: __comp_strict, ..
        } => {
            (new_residuals, dae_type) = slicedDAEModeComponent(
                __comp_strict.iteration_vars.clone(),
                &__comp_strict.residual_eqns,
                variables,
                uniqueIndex,
                slice_set,
            )?;
            (new_residuals, dae_type)
        }
        _ => (
            metamodelica::nil(),
            if (isDiscrete(&comp)?) {
                DAEType::REMOVED.clone()
            } else {
                DAEType::INNER.clone()
            },
        ),
    });
    if dae_type == DAEType::RESIDUAL.clone() {
        residuals = listAppend(new_residuals, residuals);
    } else if dae_type == DAEType::INNER.clone() {
        inners = metamodelica::cons(comp, inners);
    }
    Ok((residuals, inners))
}

pub(crate) fn slicedDAEModeComponent(
    mut var_slices: metamodelica::List<
        metamodelica::Ref<Slice::NBSlice<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>>>,
    >,
    mut eqn_slices: &metamodelica::List<
        metamodelica::Ref<Slice::NBSlice<Pointer::Pointer<metamodelica::Ref<Equation::Equation>>>>,
    >,
    mut variables: &metamodelica::Ref<VariablePointers::VariablePointers>,
    mut uniqueIndex: Pointer::Pointer<i32>,
    mut slice_set: metamodelica::Ref<UnorderedSet::UnorderedSet<metamodelica::Ref<ComponentRef::NFComponentRef>>>,
) -> Result<(metamodelica::List<metamodelica::Ref<NBStrongComponent>>, DAEType)> {
    let mut new_residuals: metamodelica::List<metamodelica::Ref<NBStrongComponent>>;
    let mut dae_type: DAEType = DAEType::RESIDUAL.clone();
    let mut eqn: Pointer::Pointer<metamodelica::Ref<Equation::Equation>>;
    let mut eqn_name: metamodelica::Ref<ComponentRef::NFComponentRef>;
    let mut acc_new_residuals: metamodelica::List<metamodelica::List<metamodelica::Ref<NBStrongComponent>>> =
        metamodelica::nil();
    if List::all(
        &({
            let mut __acc: metamodelica::List<metamodelica::List<i32>> = metamodelica::nil();
            for mut v in (var_slices).into_iter().cloned() {
                let __x = v.indices.clone();
                __acc = cons(__x, __acc);
            }
            __acc.reverse()
        }),
        &fnptr!(listEmpty, _),
    )? {
        for mut eqn_slice in &**eqn_slices {
            eqn = Slice::getT(eqn_slice.clone());
            eqn_name = Equation::getEqnName(eqn.clone())?;
            if (eqn_slice.indices).is_empty() && !(UnorderedSet::contains(eqn_name, slice_set.clone())?) {
                (new_residuals, dae_type) = singleDAEModeComponent(eqn, variables, uniqueIndex.clone())?;
                if dae_type == DAEType::RESIDUAL.clone() {
                    acc_new_residuals = metamodelica::cons(new_residuals, acc_new_residuals);
                } else if dae_type == DAEType::INNER.clone() {
                    break;
                }
            } else {
                dae_type = DAEType::INNER.clone();
                break;
            }
        }
    } else {
        dae_type = DAEType::INNER.clone();
    }
    if dae_type == DAEType::INNER.clone() {
        for mut eqn_slice in &**eqn_slices {
            eqn = Slice::getT(eqn_slice.clone());
            eqn_name = Equation::getEqnName(eqn)?;
            UnorderedSet::add(eqn_name, slice_set.clone())?;
        }
        new_residuals = metamodelica::nil();
    } else {
        new_residuals = List::flatten(acc_new_residuals)?;
    }
    Ok((new_residuals, dae_type))
}

pub(crate) fn singleDAEModeComponent(
    mut eqn_ptr: Pointer::Pointer<metamodelica::Ref<Equation::Equation>>,
    mut variables: &metamodelica::Ref<VariablePointers::VariablePointers>,
    mut uniqueIndex: Pointer::Pointer<i32>,
) -> Result<(metamodelica::List<metamodelica::Ref<NBStrongComponent>>, DAEType)> {
    let mut new_residuals: metamodelica::List<metamodelica::Ref<NBStrongComponent>>;
    let mut dae_type: DAEType = DAEType::RESIDUAL.clone();
    let mut new_eqns: Pointer::Pointer<metamodelica::List<Pointer::Pointer<metamodelica::Ref<Equation::Equation>>>>;
    let mut dummy_set: metamodelica::Ref<
        UnorderedSet::UnorderedSet<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>>,
    >;
    let mut eqn: metamodelica::Ref<Equation::Equation>;
    let mut eqns: metamodelica::List<Pointer::Pointer<metamodelica::Ref<Equation::Equation>>>;
    new_eqns = Pointer::create(metamodelica::nil());
    dummy_set = UnorderedSet::new(
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
    eqn = Inline::inlineRecordTupleArrayEquation(
        Pointer::access(eqn_ptr),
        &(crate::NBEquation::Iterator::interned_EMPTY()),
        variables,
        new_eqns.clone(),
        dummy_set,
        uniqueIndex,
        true,
    )?;
    eqns = Pointer::access(new_eqns);
    eqns = if ((eqns).is_empty()) {
        list![Pointer::create(eqn)]
    } else {
        eqns
    };
    (new_residuals, dae_type) = inlinedDAEModeComponent(&eqns)?;
    Ok((new_residuals, dae_type))
}

pub(crate) fn inlinedDAEModeComponent(
    mut eqns: &metamodelica::List<Pointer::Pointer<metamodelica::Ref<Equation::Equation>>>,
) -> Result<(metamodelica::List<metamodelica::Ref<NBStrongComponent>>, DAEType)> {
    let mut comps: metamodelica::List<metamodelica::Ref<NBStrongComponent>> = metamodelica::nil();
    let mut dae_type: DAEType = DAEType::UNPROCESSED.clone();
    let mut new_eqn: Pointer::Pointer<metamodelica::Ref<Equation::Equation>>;
    let mut new_comp: metamodelica::Ref<NBStrongComponent>;
    for mut eqn in &**eqns {
        if Equation::isDiscrete(eqn.clone()) {
            if dae_type < DAEType::INNER.clone() {
                dae_type = DAEType::REMOVED.clone();
            }
        } else {
            new_eqn = Equation::createResidual(eqn.clone(), None, false, true)?;
            if Equation::isResidual(new_eqn.clone()) {
                new_comp = metamodelica::Ref::new(NBStrongComponent::SINGLE_COMPONENT {
                    var: Equation::getResidualVar(new_eqn.clone())?,
                    eqn: new_eqn,
                    status: Solve::Status::UNPROCESSED.clone(),
                });
                comps = metamodelica::cons(new_comp, comps);
                dae_type = DAEType::RESIDUAL.clone();
            } else {
                dae_type = DAEType::INNER.clone();
                break;
            }
        }
    }
    Ok((comps, dae_type))
}

pub(crate) fn fromSolvedEquationSlice(
    mut eqn_slice: metamodelica::Ref<Slice::NBSlice<Pointer::Pointer<metamodelica::Ref<Equation::Equation>>>>,
) -> Result<metamodelica::Ref<NBStrongComponent>> {
    fn simpleSolvedEquation(
        mut eqn: metamodelica::Ref<Equation::Equation>,
        mut eqn_ptr: Pointer::Pointer<metamodelica::Ref<Equation::Equation>>,
    ) -> Result<metamodelica::Ref<NBStrongComponent>> {
        let mut comp: metamodelica::Ref<NBStrongComponent>;
        comp = (::match_deref::match_deref! { match &(Equation::getLHS(eqn.clone())?) {
            Some(lhs @ Deref @ Expression::CREF { .. }) => {
                metamodelica::Ref::new(NBStrongComponent::SINGLE_COMPONENT { var: BVariable::getVarPointer(&(Expression::toCref(metamodelica::AsArg::as_arg(&lhs))?), metamodelica::sourceInfo!("NBackEnd/Classes/NBStrongComponent.mo"))?, eqn: eqn_ptr, status: Solve::Status::EXPLICIT.clone() })
            },
            _ => {
                metamodelica::Ref::new(NBStrongComponent::MULTI_COMPONENT { vars: Equation::getLHSVars(&eqn)?, eqn: metamodelica::Ref::new(Slice::NBSlice { t: eqn_ptr, indices: metamodelica::nil() }), status: Solve::Status::EXPLICIT.clone() })
            },
            _ => unreachable!("match_deref! exhaustiveness placeholder"),
        } });
        Ok(comp)
    }

    let mut comp: metamodelica::Ref<NBStrongComponent>;
    let mut eqn_ptr: Pointer::Pointer<metamodelica::Ref<Equation::Equation>> = Slice::getT(eqn_slice.clone());
    let mut eqn: metamodelica::Ref<Equation::Equation> = Pointer::access(eqn_ptr.clone());
    let mut body: metamodelica::Ref<IfEquationBody::IfEquationBody>;
    let mut subs: metamodelica::List<metamodelica::Ref<Subscript::NFSubscript>>;
    let mut lhs_cref: metamodelica::Ref<ComponentRef::NFComponentRef>;
    if !((eqn_slice.indices).is_empty())
        && Equation::isArrayEquation(eqn_ptr.clone())
        && List::hasOneElement(&eqn_slice.indices)
    {
        subs = ({
            let mut __acc: metamodelica::List<metamodelica::Ref<Subscript::NFSubscript>> = metamodelica::nil();
            for mut l in
                (Slice::indexToLocation((eqn_slice.indices).head().cloned()?, Equation::sizes(eqn_ptr, false)?))
                    .into_iter()
                    .cloned()
            {
                let __x = metamodelica::Ref::new(Subscript::NFSubscript::INDEX {
                    index: metamodelica::Ref::new(Expression::NFExpression::INTEGER { value: l.clone() + 1 }),
                });
                __acc = cons(__x, __acc);
            }
            __acc.reverse()
        });
        lhs_cref = ComponentRef::setSubscripts(subs, Expression::toCref(&(Util::getOption(Equation::getLHS(eqn)?)?))?)?;
        comp = metamodelica::Ref::new(NBStrongComponent::SLICED_COMPONENT {
            var_cref: lhs_cref.clone(),
            var: metamodelica::Ref::new(Slice::NBSlice {
                t: BVariable::getVarPointer(
                    &lhs_cref,
                    metamodelica::sourceInfo!("NBackEnd/Classes/NBStrongComponent.mo"),
                )?,
                indices: metamodelica::nil(),
            }),
            eqn: eqn_slice,
            status: Solve::Status::EXPLICIT.clone(),
        });
    } else {
        comp = (match &*eqn.clone() {
            Equation::SCALAR_EQUATION { .. } => simpleSolvedEquation(eqn, eqn_ptr)?,
            Equation::ARRAY_EQUATION { .. } => simpleSolvedEquation(eqn, eqn_ptr)?,
            Equation::RECORD_EQUATION { .. } => simpleSolvedEquation(eqn, eqn_ptr)?,
            Equation::IF_EQUATION { body: __esc_body, .. } => {
                body = (*__esc_body).clone();
                if IfEquationBody::isSplit(metamodelica::AsArg::as_arg(&body))? {
                    comp = metamodelica::Ref::new(NBStrongComponent::SINGLE_COMPONENT {
                        var: BVariable::getVarPointer(
                            &(Expression::toCref(&(Util::getOption(Equation::getLHS(eqn)?)?))?),
                            metamodelica::sourceInfo!("NBackEnd/Classes/NBStrongComponent.mo"),
                        )?,
                        eqn: eqn_ptr,
                        status: Solve::Status::EXPLICIT.clone(),
                    });
                } else {
                    comp = metamodelica::Ref::new(NBStrongComponent::MULTI_COMPONENT {
                        vars: Equation::getLHSVars(&eqn)?,
                        eqn: metamodelica::Ref::new(Slice::NBSlice {
                            t: eqn_ptr,
                            indices: metamodelica::nil(),
                        }),
                        status: Solve::Status::EXPLICIT.clone(),
                    });
                }
                comp
            }
            Equation::FOR_EQUATION { .. } => metamodelica::Ref::new(NBStrongComponent::SLICED_COMPONENT {
                var_cref: openmodelica_nf_frontend::NFComponentRef::interned_EMPTY(),
                var: metamodelica::Ref::new(Slice::NBSlice {
                    t: Pointer::create(BVariable::DUMMY_VARIABLE().clone()),
                    indices: metamodelica::nil(),
                }),
                eqn: eqn_slice,
                status: Solve::Status::EXPLICIT.clone(),
            }),
            _ => {
                Error::addMessage(
                    Error::INTERNAL_ERROR.clone(),
                    list![{
                        let mut __mm_s = String::new();
                        __mm_s.push_str(&*literal!("NBStrongComponent.fromSolvedEquationSlice"));
                        __mm_s.push_str(&*literal!(" failed for:\n"));
                        __mm_s.push_str(&*Slice::toString(
                            eqn_slice,
                            &({
                                let __pe_b1 = literal!("");
                                move |__pe_a0| Equation::pointerToString(__pe_a0, __pe_b1.clone())
                            }),
                            10,
                        )?);
                        ArcStr::from(__mm_s)
                    }],
                )?;
                return Err("fail");
            }
        });
    }
    Ok(comp)
}

pub(crate) fn toSolvedEquation(
    mut comp: &metamodelica::Ref<NBStrongComponent>,
) -> Result<Pointer::Pointer<metamodelica::Ref<Equation::Equation>>> {
    let mut eqn: Pointer::Pointer<metamodelica::Ref<Equation::Equation>>;
    eqn = (match &**comp {
        SINGLE_COMPONENT {
            status: Solve::Status::EXPLICIT,
            eqn: __comp_eqn,
            ..
        } => __comp_eqn.clone(),
        MULTI_COMPONENT {
            status: Solve::Status::EXPLICIT,
            eqn: __comp_eqn,
            ..
        } => Slice::getT(__comp_eqn.clone()),
        SLICED_COMPONENT {
            status: Solve::Status::EXPLICIT,
            eqn: __comp_eqn,
            ..
        } => Slice::getT(__comp_eqn.clone()),
        GENERIC_COMPONENT { eqn: __comp_eqn, .. } => Slice::getT(__comp_eqn.clone()),
        _ => {
            Error::addMessage(
                Error::INTERNAL_ERROR.clone(),
                list![{
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!("NBStrongComponent.toSolvedEquation"));
                    __mm_s.push_str(&*literal!(
                        " failed because strong component could not be\n        solved explicitly:\n"
                    ));
                    __mm_s.push_str(&*toString(comp, -1)?);
                    ArcStr::from(__mm_s)
                }],
            )?;
            return Err("fail");
        }
    });
    Ok(eqn)
}

pub(crate) fn collectCrefs(
    mut comp: &metamodelica::Ref<NBStrongComponent>,
    mut var_rep: &metamodelica::Ref<VariablePointers::VariablePointers>,
    mut eqn_rep: &metamodelica::Ref<VariablePointers::VariablePointers>,
    mut var_rep_mapping: &metamodelica::Ref<Mapping::Mapping>,
    mut eqn_rep_mapping: &metamodelica::Ref<Mapping::Mapping>,
    mut map: metamodelica::Ref<
        UnorderedMap::UnorderedMap<
            metamodelica::Ref<ComponentRef::NFComponentRef>,
            metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>>,
        >,
    >,
    mut set: metamodelica::Ref<UnorderedSet::UnorderedSet<metamodelica::Ref<ComponentRef::NFComponentRef>>>,
    mut jacType: JacobianType,
) -> Result<()> {
    let () = (match &**comp {
        SINGLE_COMPONENT {
            eqn: __comp_eqn,
            var: __comp_var,
            ..
        } if (Equation::isArrayEquation(__comp_eqn.clone())) => {
            let mut dependencies: metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>>;
            let mut scalarized_dependencies: metamodelica::List<(
                metamodelica::Ref<ComponentRef::NFComponentRef>,
                metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>>,
            )>;
            dependencies = Equation::collectCrefs(
                Pointer::access(__comp_eqn.clone()),
                (std::sync::Arc::new({
                    let __pe_b2 = set;
                    move |__pe_a0, __pe_a1| Slice::getDependentCrefCausalized(__pe_a0, __pe_a1, __pe_b2.clone())
                })
                    as std::sync::Arc<
                        dyn ::std::ops::Fn(
                                metamodelica::Ref<ComponentRef::NFComponentRef>,
                                metamodelica::Ref<
                                    UnorderedSet::UnorderedSet<metamodelica::Ref<ComponentRef::NFComponentRef>>,
                                >,
                            )
                                -> Result<metamodelica::Ref<ComponentRef::NFComponentRef>>
                            + 'static,
                    >),
                (std::sync::Arc::new(
                    move |__a0: metamodelica::Ref<Expression::NFExpression>,
                          __a1: Arc<
                        dyn ::std::ops::Fn(
                                metamodelica::Ref<Expression::NFExpression>,
                            )
                                -> Result<metamodelica::Ref<Expression::NFExpression>>
                            + 'static,
                    >| Expression::fakeMap(__a0, metamodelica::arc_ref(&__a1)),
                )
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
                            )
                                -> Result<metamodelica::Ref<Expression::NFExpression>>
                            + 'static,
                    >),
            )?;
            scalarized_dependencies = Slice::getDependentCrefsPseudoArrayCausalized(
                BVariable::getVarName(__comp_var.clone()),
                dependencies,
                metamodelica::nil(),
            )?;
            addScalarizedDependencies(scalarized_dependencies, map, jacType)?;
            ()
        }
        SINGLE_COMPONENT {
            eqn: __comp_eqn,
            var: __comp_var,
            ..
        } => {
            let mut dependencies: metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>>;
            let mut deps_set: metamodelica::Ref<
                UnorderedSet::UnorderedSet<metamodelica::Ref<ComponentRef::NFComponentRef>>,
            >;
            dependencies = Equation::collectCrefs(
                Pointer::access(__comp_eqn.clone()),
                (std::sync::Arc::new({
                    let __pe_b2 = set;
                    move |__pe_a0, __pe_a1| Slice::getDependentCrefCausalized(__pe_a0, __pe_a1, __pe_b2.clone())
                })
                    as std::sync::Arc<
                        dyn ::std::ops::Fn(
                                metamodelica::Ref<ComponentRef::NFComponentRef>,
                                metamodelica::Ref<
                                    UnorderedSet::UnorderedSet<metamodelica::Ref<ComponentRef::NFComponentRef>>,
                                >,
                            )
                                -> Result<metamodelica::Ref<ComponentRef::NFComponentRef>>
                            + 'static,
                    >),
                (std::sync::Arc::new(
                    move |__a0: metamodelica::Ref<Expression::NFExpression>,
                          __a1: Arc<
                        dyn ::std::ops::Fn(
                                metamodelica::Ref<Expression::NFExpression>,
                            )
                                -> Result<metamodelica::Ref<Expression::NFExpression>>
                            + 'static,
                    >| Expression::fakeMap(__a0, metamodelica::arc_ref(&__a1)),
                )
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
                            )
                                -> Result<metamodelica::Ref<Expression::NFExpression>>
                            + 'static,
                    >),
            )?;
            dependencies = List::flatten(
                ({
                    let mut __acc: metamodelica::List<
                        metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>>,
                    > = metamodelica::nil();
                    for mut dep in (dependencies).into_iter().cloned() {
                        let __x = ComponentRef::scalarizeAll(dep.clone(), true)?;
                        __acc = cons(__x, __acc);
                    }
                    __acc.reverse()
                }),
            )?;
            deps_set = prepareDependencies(
                UnorderedSet::fromList(
                    &dependencies,
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
                )?,
                map.clone(),
                jacType,
            )?;
            updateDependencyMap(BVariable::getVarName(__comp_var.clone()), deps_set, map)?;
            ()
        }
        MULTI_COMPONENT {
            eqn: __comp_eqn,
            vars: __comp_vars,
            ..
        } => {
            let mut cref: metamodelica::Ref<ComponentRef::NFComponentRef> = metamodelica::Ref::new(ComponentRef::EMPTY);
            let mut dependencies: metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>>;
            let mut deps_set: metamodelica::Ref<
                UnorderedSet::UnorderedSet<metamodelica::Ref<ComponentRef::NFComponentRef>>,
            >;
            dependencies = Equation::collectCrefs(
                Pointer::access(Slice::getT(__comp_eqn.clone())),
                (std::sync::Arc::new({
                    let __pe_b2 = set;
                    move |__pe_a0, __pe_a1| Slice::getDependentCrefCausalized(__pe_a0, __pe_a1, __pe_b2.clone())
                })
                    as std::sync::Arc<
                        dyn ::std::ops::Fn(
                                metamodelica::Ref<ComponentRef::NFComponentRef>,
                                metamodelica::Ref<
                                    UnorderedSet::UnorderedSet<metamodelica::Ref<ComponentRef::NFComponentRef>>,
                                >,
                            )
                                -> Result<metamodelica::Ref<ComponentRef::NFComponentRef>>
                            + 'static,
                    >),
                (std::sync::Arc::new(
                    move |__a0: metamodelica::Ref<Expression::NFExpression>,
                          __a1: Arc<
                        dyn ::std::ops::Fn(
                                metamodelica::Ref<Expression::NFExpression>,
                            )
                                -> Result<metamodelica::Ref<Expression::NFExpression>>
                            + 'static,
                    >| Expression::fakeMap(__a0, metamodelica::arc_ref(&__a1)),
                )
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
                            )
                                -> Result<metamodelica::Ref<Expression::NFExpression>>
                            + 'static,
                    >),
            )?;
            dependencies = ({
                let mut __acc: metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>> =
                    metamodelica::nil();
                for mut dep in (dependencies).into_iter().cloned() {
                    let __x = ComponentRef::stripIteratorSubscripts(dep.clone())?;
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            });
            dependencies = List::flatten(
                ({
                    let mut __acc: metamodelica::List<
                        metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>>,
                    > = metamodelica::nil();
                    for mut dep in (dependencies).into_iter().cloned() {
                        let __x = ComponentRef::scalarizeAll(dep.clone(), true)?;
                        __acc = cons(__x, __acc);
                    }
                    __acc.reverse()
                }),
            )?;
            deps_set = prepareDependencies(
                UnorderedSet::fromList(
                    &dependencies,
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
                )?,
                map.clone(),
                jacType,
            )?;
            for mut var in &*__comp_vars.clone() {
                for mut cref in &*ComponentRef::scalarizeAll(BVariable::getVarName(Slice::getT(var.clone())), true)? {
                    let mut cref = cref.clone();
                    updateDependencyMap(cref, deps_set.clone(), map.clone())?;
                }
            }
            ()
        }
        RESIZABLE_COMPONENT {
            eqn: __comp_eqn,
            var_cref: __comp_var_cref,
            ..
        } if (Equation::isForEquation(Slice::getT(__comp_eqn.clone()))) => {
            addForLoopDependencies(
                Pointer::access(Slice::getT(__comp_eqn.clone())),
                __comp_eqn.indices.clone(),
                __comp_var_cref.clone(),
                var_rep,
                eqn_rep,
                var_rep_mapping,
                eqn_rep_mapping,
                map,
                set,
                jacType,
            )?;
            ()
        }
        SLICED_COMPONENT {
            eqn: __comp_eqn,
            var_cref: __comp_var_cref,
            ..
        } if (Equation::isForEquation(Slice::getT(__comp_eqn.clone()))) => {
            addForLoopDependencies(
                Pointer::access(Slice::getT(__comp_eqn.clone())),
                __comp_eqn.indices.clone(),
                __comp_var_cref.clone(),
                var_rep,
                eqn_rep,
                var_rep_mapping,
                eqn_rep_mapping,
                map,
                set,
                jacType,
            )?;
            ()
        }
        SLICED_COMPONENT {
            eqn: __comp_eqn,
            var_cref: __comp_var_cref,
            ..
        } if (Equation::isArrayEquation(Slice::getT(__comp_eqn.clone()))) => {
            let mut dependencies: metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>>;
            let mut scalarized_dependencies: metamodelica::List<(
                metamodelica::Ref<ComponentRef::NFComponentRef>,
                metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>>,
            )>;
            let mut eqn: metamodelica::Ref<Equation::Equation>;
            eqn = Pointer::access(Slice::getT(__comp_eqn.clone()));
            dependencies = Equation::collectCrefs(
                eqn,
                (std::sync::Arc::new({
                    let __pe_b2 = set;
                    move |__pe_a0, __pe_a1| Slice::getDependentCrefCausalized(__pe_a0, __pe_a1, __pe_b2.clone())
                })
                    as std::sync::Arc<
                        dyn ::std::ops::Fn(
                                metamodelica::Ref<ComponentRef::NFComponentRef>,
                                metamodelica::Ref<
                                    UnorderedSet::UnorderedSet<metamodelica::Ref<ComponentRef::NFComponentRef>>,
                                >,
                            )
                                -> Result<metamodelica::Ref<ComponentRef::NFComponentRef>>
                            + 'static,
                    >),
                (std::sync::Arc::new(
                    move |__a0: metamodelica::Ref<Expression::NFExpression>,
                          __a1: Arc<
                        dyn ::std::ops::Fn(
                                metamodelica::Ref<Expression::NFExpression>,
                            )
                                -> Result<metamodelica::Ref<Expression::NFExpression>>
                            + 'static,
                    >| Expression::fakeMap(__a0, metamodelica::arc_ref(&__a1)),
                )
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
                            )
                                -> Result<metamodelica::Ref<Expression::NFExpression>>
                            + 'static,
                    >),
            )?;
            scalarized_dependencies = Slice::getDependentCrefsPseudoArrayCausalized(
                __comp_var_cref.clone(),
                dependencies,
                __comp_eqn.indices.clone(),
            )?;
            addScalarizedDependencies(scalarized_dependencies, map, jacType)?;
            ()
        }
        SLICED_COMPONENT {
            eqn: __comp_eqn,
            var_cref: __comp_var_cref,
            ..
        } => {
            let mut dependencies: metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>>;
            let mut eqn: metamodelica::Ref<Equation::Equation>;
            let mut deps_set: metamodelica::Ref<
                UnorderedSet::UnorderedSet<metamodelica::Ref<ComponentRef::NFComponentRef>>,
            >;
            eqn = Pointer::access(Slice::getT(__comp_eqn.clone()));
            dependencies = Equation::collectCrefs(
                eqn,
                (std::sync::Arc::new({
                    let __pe_b2 = set;
                    move |__pe_a0, __pe_a1| Slice::getDependentCrefCausalized(__pe_a0, __pe_a1, __pe_b2.clone())
                })
                    as std::sync::Arc<
                        dyn ::std::ops::Fn(
                                metamodelica::Ref<ComponentRef::NFComponentRef>,
                                metamodelica::Ref<
                                    UnorderedSet::UnorderedSet<metamodelica::Ref<ComponentRef::NFComponentRef>>,
                                >,
                            )
                                -> Result<metamodelica::Ref<ComponentRef::NFComponentRef>>
                            + 'static,
                    >),
                (std::sync::Arc::new(
                    move |__a0: metamodelica::Ref<Expression::NFExpression>,
                          __a1: Arc<
                        dyn ::std::ops::Fn(
                                metamodelica::Ref<Expression::NFExpression>,
                            )
                                -> Result<metamodelica::Ref<Expression::NFExpression>>
                            + 'static,
                    >| Expression::fakeMap(__a0, metamodelica::arc_ref(&__a1)),
                )
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
                            )
                                -> Result<metamodelica::Ref<Expression::NFExpression>>
                            + 'static,
                    >),
            )?;
            dependencies = List::flatten(
                ({
                    let mut __acc: metamodelica::List<
                        metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>>,
                    > = metamodelica::nil();
                    for mut dep in (dependencies).into_iter().cloned() {
                        let __x = ComponentRef::scalarizeAll(dep.clone(), true)?;
                        __acc = cons(__x, __acc);
                    }
                    __acc.reverse()
                }),
            )?;
            deps_set = prepareDependencies(
                UnorderedSet::fromList(
                    &dependencies,
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
                )?,
                map.clone(),
                jacType,
            )?;
            updateDependencyMap(__comp_var_cref.clone(), deps_set, map)?;
            ()
        }
        GENERIC_COMPONENT {
            eqn: __comp_eqn,
            var_cref: __comp_var_cref,
            ..
        } if (Equation::isForEquation(Slice::getT(__comp_eqn.clone()))) => {
            addForLoopDependencies(
                Pointer::access(Slice::getT(__comp_eqn.clone())),
                __comp_eqn.indices.clone(),
                __comp_var_cref.clone(),
                var_rep,
                eqn_rep,
                var_rep_mapping,
                eqn_rep_mapping,
                map,
                set,
                jacType,
            )?;
            ()
        }
        ALGEBRAIC_LOOP { strict, .. } => {
            let mut eqn_ptr: Pointer::Pointer<metamodelica::Ref<Equation::Equation>>;
            let mut cref: metamodelica::Ref<ComponentRef::NFComponentRef> = metamodelica::Ref::new(ComponentRef::EMPTY);
            let mut loop_vars: metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>>;
            let mut tmp: metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>>;
            let mut scalarized_dependencies: metamodelica::List<(
                metamodelica::Ref<ComponentRef::NFComponentRef>,
                metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>>,
            )>;
            let mut body: metamodelica::Ref<Equation::Equation>;
            let mut iter: metamodelica::Ref<Iterator::Iterator>;
            let mut deps_set: metamodelica::Ref<
                UnorderedSet::UnorderedSet<metamodelica::Ref<ComponentRef::NFComponentRef>>,
            >;
            deps_set = UnorderedSet::new(
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
                13,
            );
            for mut slice in &*strict.residual_eqns.clone() {
                tmp = Equation::collectCrefs(
                    Pointer::access(Slice::getT(slice.clone())),
                    (std::sync::Arc::new({
                        let __pe_b2 = set.clone();
                        move |__pe_a0, __pe_a1| Slice::getDependentCrefCausalized(__pe_a0, __pe_a1, __pe_b2.clone())
                    })
                        as std::sync::Arc<
                            dyn ::std::ops::Fn(
                                    metamodelica::Ref<ComponentRef::NFComponentRef>,
                                    metamodelica::Ref<
                                        UnorderedSet::UnorderedSet<metamodelica::Ref<ComponentRef::NFComponentRef>>,
                                    >,
                                )
                                    -> Result<metamodelica::Ref<ComponentRef::NFComponentRef>>
                                + 'static,
                        >),
                    (std::sync::Arc::new(
                        move |__a0: metamodelica::Ref<Expression::NFExpression>,
                              __a1: Arc<
                            dyn ::std::ops::Fn(
                                    metamodelica::Ref<Expression::NFExpression>,
                                )
                                    -> Result<metamodelica::Ref<Expression::NFExpression>>
                                + 'static,
                        >| Expression::fakeMap(__a0, metamodelica::arc_ref(&__a1)),
                    )
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
                                )
                                    -> Result<metamodelica::Ref<Expression::NFExpression>>
                                + 'static,
                        >),
                )?;
                eqn_ptr = Slice::getT(slice.clone());
                if Equation::isForEquation(eqn_ptr.clone()) {
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(Pointer::access(eqn_ptr.clone())) {
                        Deref @ Equation::FOR_EQUATION { iter: __pa0, body: Deref @ metamodelica::ListNode::Cons { head: __pa1, tail: Deref @ metamodelica::ListNode::Nil }, .. } => (__pa0.clone(), __pa1.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    iter = metamodelica::Own::own(__pa0);
                    body = metamodelica::Own::own(__pa1);
                    cref = Equation::getEqnName(eqn_ptr.clone())?;
                    scalarized_dependencies = Slice::getDependentCrefsPseudoForCausalized(
                        cref,
                        &tmp,
                        var_rep,
                        eqn_rep,
                        var_rep_mapping,
                        eqn_rep_mapping,
                        &iter,
                        Equation::size(eqn_ptr, false)?,
                        slice.indices.clone(),
                        true,
                    )?;
                    tmp = List::flatten(
                        ({
                            let mut __acc: metamodelica::List<
                                metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>>,
                            > = metamodelica::nil();
                            for mut tpl in (scalarized_dependencies).into_iter().cloned() {
                                let __x = Util::tuple22(tpl.clone());
                                __acc = cons(__x, __acc);
                            }
                            __acc.reverse()
                        }),
                    )?;
                }
                for mut dep in &*tmp {
                    for mut scal in &*ComponentRef::scalarizeAll(dep.clone(), true)? {
                        UnorderedSet::add(scal.clone(), deps_set.clone())?;
                    }
                }
            }
            deps_set = prepareDependencies(deps_set, map.clone(), jacType)?;
            loop_vars = ({
                let mut __acc: metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>> =
                    metamodelica::nil();
                for mut var in (strict.iteration_vars.clone()).into_iter().cloned() {
                    let __x = BVariable::getVarName(Slice::getT(var.clone()));
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            });
            for mut i in 1..=metamodelica::arrayLength(strict.innerEquations.clone()) {
                collectCrefs(
                    &({
                        let __elt = (*metamodelica::index_checked(&strict.innerEquations.borrow(), i)?).clone();
                        __elt
                    }),
                    var_rep,
                    eqn_rep,
                    var_rep_mapping,
                    eqn_rep_mapping,
                    map.clone(),
                    set.clone(),
                    jacType,
                )?;
                loop_vars = listAppend(
                    ({
                        let mut __acc: metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>> =
                            metamodelica::nil();
                        for mut var in (getVariables(
                            ({
                                let __elt = (*metamodelica::index_checked(&strict.innerEquations.borrow(), i)?).clone();
                                __elt
                            }),
                        )?)
                        .into_iter()
                        .cloned()
                        {
                            let __x = BVariable::getVarName(var.clone());
                            __acc = cons(__x, __acc);
                        }
                        __acc.reverse()
                    }),
                    loop_vars,
                );
            }
            for mut cref in &*loop_vars {
                let mut cref = cref.clone();
                updateDependencyMap(cref, deps_set.clone(), map.clone())?;
            }
            ()
        }
        ALIAS {
            original: __comp_original,
            ..
        } => {
            collectCrefs(
                metamodelica::AsArg::as_arg(&__comp_original),
                var_rep,
                eqn_rep,
                var_rep_mapping,
                eqn_rep_mapping,
                map,
                set,
                jacType,
            )?;
            ()
        }
        _ => (),
    });
    Ok(())
}

pub(crate) fn addScalarizedDependencies(
    mut scalarized_dependencies: metamodelica::List<(
        metamodelica::Ref<ComponentRef::NFComponentRef>,
        metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>>,
    )>,
    mut map: metamodelica::Ref<
        UnorderedMap::UnorderedMap<
            metamodelica::Ref<ComponentRef::NFComponentRef>,
            metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>>,
        >,
    >,
    mut jacType: JacobianType,
) -> Result<()> {
    let mut cref: metamodelica::Ref<ComponentRef::NFComponentRef>;
    let mut dependencies: metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>>;
    let mut deps_set: metamodelica::Ref<UnorderedSet::UnorderedSet<metamodelica::Ref<ComponentRef::NFComponentRef>>>;
    for mut tpl in &*scalarized_dependencies.reverse() {
        (cref, dependencies) = tpl.clone();
        deps_set = prepareDependencies(
            UnorderedSet::fromList(
                &dependencies,
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
            )?,
            map.clone(),
            jacType,
        )?;
        updateDependencyMap(cref, deps_set, map.clone())?;
    }
    Ok(())
}

pub(crate) fn addForLoopDependencies(
    mut eqn: metamodelica::Ref<Equation::Equation>,
    mut indices: metamodelica::List<i32>,
    mut var_cref: metamodelica::Ref<ComponentRef::NFComponentRef>,
    mut var_rep: &metamodelica::Ref<VariablePointers::VariablePointers>,
    mut eqn_rep: &metamodelica::Ref<VariablePointers::VariablePointers>,
    mut var_rep_mapping: &metamodelica::Ref<Mapping::Mapping>,
    mut eqn_rep_mapping: &metamodelica::Ref<Mapping::Mapping>,
    mut map: metamodelica::Ref<
        UnorderedMap::UnorderedMap<
            metamodelica::Ref<ComponentRef::NFComponentRef>,
            metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>>,
        >,
    >,
    mut set: metamodelica::Ref<UnorderedSet::UnorderedSet<metamodelica::Ref<ComponentRef::NFComponentRef>>>,
    mut jacType: JacobianType,
) -> Result<()> {
    let mut iter: metamodelica::Ref<Iterator::Iterator>;
    let mut body: metamodelica::Ref<Equation::Equation>;
    let mut dependencies: metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>>;
    let mut cref: metamodelica::Ref<ComponentRef::NFComponentRef>;
    let mut scalarized_dependencies: metamodelica::List<(
        metamodelica::Ref<ComponentRef::NFComponentRef>,
        metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>>,
    )>;
    match '__try0: {
        let (__pa1, __pa2) = ::match_deref::match_deref! { match &(eqn.clone()) {
            Deref @ Equation::FOR_EQUATION { iter: __pa1, body: Deref @ metamodelica::ListNode::Cons { head: __pa2, tail: Deref @ metamodelica::ListNode::Nil }, .. } => (__pa1.clone(), __pa2.clone()),
            _ => break '__try0 Err::<_, _>("pattern mismatch"),
        } };
        iter = metamodelica::Own::own(__pa1);
        body = metamodelica::Own::own(__pa2);
        Ok::<_, &'static str>((body.clone(), iter.clone()))
    } {
        Ok((__try0_o0, __try0_o1)) => {
            body = __try0_o0;
            iter = __try0_o1;
        }
        Err(__try0_err) => {
            Error::addMessage(
                Error::INTERNAL_ERROR.clone(),
                list![{
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!("NBStrongComponent.addForLoopDependencies"));
                    __mm_s.push_str(&*literal!(
                        " failed because the for-loop had more than one body equation:\n"
                    ));
                    __mm_s.push_str(&*Equation::toString(eqn.clone(), literal!(""))?);
                    ArcStr::from(__mm_s)
                }],
            )?;
            return Err(__try0_err);
        }
    }
    dependencies = Equation::collectCrefs(
        eqn.clone(),
        (std::sync::Arc::new({
            let __pe_b2 = set;
            move |__pe_a0, __pe_a1| Slice::getDependentCrefCausalized(__pe_a0, __pe_a1, __pe_b2.clone())
        })
            as std::sync::Arc<
                dyn ::std::ops::Fn(
                        metamodelica::Ref<ComponentRef::NFComponentRef>,
                        metamodelica::Ref<UnorderedSet::UnorderedSet<metamodelica::Ref<ComponentRef::NFComponentRef>>>,
                    ) -> Result<metamodelica::Ref<ComponentRef::NFComponentRef>>
                    + 'static,
            >),
        (std::sync::Arc::new(
            move |__a0: metamodelica::Ref<Expression::NFExpression>,
                  __a1: Arc<
                dyn ::std::ops::Fn(
                        metamodelica::Ref<Expression::NFExpression>,
                    ) -> Result<metamodelica::Ref<Expression::NFExpression>>
                    + 'static,
            >| Expression::fakeMap(__a0, metamodelica::arc_ref(&__a1)),
        )
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
    if ComponentRef::isEmpty(&var_cref) {
        let __pa4 = ::match_deref::match_deref! { match &(Equation::getLHS(body)?) {
            Some(Deref @ Expression::CREF { cref: __pa4, .. }) => __pa4.clone(),
            _ => return Err("pattern mismatch"),
        } };
        cref = metamodelica::Own::own(__pa4);
    } else {
        cref = var_cref;
    }
    scalarized_dependencies = Slice::getDependentCrefsPseudoForCausalized(
        cref,
        &dependencies,
        var_rep,
        eqn_rep,
        var_rep_mapping,
        eqn_rep_mapping,
        &iter,
        Equation::size(Pointer::create(eqn), false)?,
        indices,
        false,
    )?;
    addScalarizedDependencies(scalarized_dependencies, map, jacType)?;
    Ok(())
}

pub(crate) fn addLoopJacobian(
    mut comp: metamodelica::Ref<NBStrongComponent>,
    mut jac: Option<metamodelica::Ref<BackendDAE::NBackendDAE>>,
) -> Result<metamodelica::Ref<NBStrongComponent>> {
    let mut comp: metamodelica::Ref<NBStrongComponent> = comp;
    comp = (match &*comp {
        ALGEBRAIC_LOOP { strict, .. } => {
            let mut strict = (*strict).clone();
            assign_field!(strict.jac = jac);
            assign_variant_field!(comp => NBStrongComponent::ALGEBRAIC_LOOP; strict = strict.clone());
            comp
        }
        _ => {
            Error::addMessage(
                Error::INTERNAL_ERROR.clone(),
                list![{
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!("NBStrongComponent.addLoopJacobian"));
                    __mm_s.push_str(&*literal!(" failed because of wrong component: "));
                    __mm_s.push_str(&*toString(&comp, -1)?);
                    ArcStr::from(__mm_s)
                }],
            )?;
            return Err("fail");
        }
    });
    Ok(comp)
}

pub(crate) fn getLoopResiduals(
    mut comp: &metamodelica::Ref<NBStrongComponent>,
) -> Result<metamodelica::List<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>>> {
    let mut residuals: metamodelica::List<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>>;
    residuals = (match &**comp {
        ALGEBRAIC_LOOP {
            strict: __comp_strict, ..
        } => Tearing::getResidualVars(metamodelica::AsArg::as_arg(&__comp_strict))?,
        _ => metamodelica::nil(),
    });
    Ok(residuals)
}

pub(crate) fn getVariables(
    mut comp: metamodelica::Ref<NBStrongComponent>,
) -> Result<metamodelica::List<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>>> {
    '__tco: loop {
        match &*comp {
            SINGLE_COMPONENT { var: __comp_var, .. } => return Ok(list![__comp_var.clone()]),
            MULTI_COMPONENT { vars: __comp_vars, .. } => {
                return Ok(({
                    let mut __acc: metamodelica::List<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>> =
                        metamodelica::nil();
                    for mut v in (__comp_vars.clone()).into_iter().cloned() {
                        let __x = Slice::getT(v.clone());
                        __acc = cons(__x, __acc);
                    }
                    __acc.reverse()
                }));
            }
            SLICED_COMPONENT { var: __comp_var, .. } => return Ok(list![Slice::getT(__comp_var.clone())]),
            RESIZABLE_COMPONENT { var: __comp_var, .. } => return Ok(list![Slice::getT(__comp_var.clone())]),
            GENERIC_COMPONENT { var: __comp_var, .. } => return Ok(list![Slice::getT(__comp_var.clone())]),
            ENTWINED_COMPONENT {
                entwined_slices: __comp_entwined_slices,
                ..
            } => {
                return Ok(List::flatten(
                    ({
                        let mut __acc: metamodelica::List<
                            metamodelica::List<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>>,
                        > = metamodelica::nil();
                        for mut slice in (__comp_entwined_slices.clone()).into_iter().cloned() {
                            let __x = getVariables(slice.clone())?;
                            __acc = cons(__x, __acc);
                        }
                        __acc.reverse()
                    }),
                )?);
            }
            ALGEBRAIC_LOOP {
                strict: __comp_strict, ..
            } => return Ok(Tearing::getVariables(metamodelica::AsArg::as_arg(&__comp_strict))?),
            ALIAS {
                original: __comp_original,
                ..
            } => {
                comp = __comp_original.clone();
                continue '__tco;
            }
            _ => {
                Error::addMessage(
                    Error::INTERNAL_ERROR.clone(),
                    list![{
                        let mut __mm_s = String::new();
                        __mm_s.push_str(&*literal!("NBStrongComponent.getVariables"));
                        __mm_s.push_str(&*literal!(" failed because of wrong component: "));
                        __mm_s.push_str(&*toString(&comp, -1)?);
                        ArcStr::from(__mm_s)
                    }],
                )?;
                return Ok(return Err("fail"));
            }
        }
    }
}

pub(crate) fn getVariableCrefs<'__b>(
    mut comp: &'__b metamodelica::Ref<NBStrongComponent>,
) -> Result<metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>>> {
    let mut var_crefs: metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>>;
    var_crefs = (match &**comp {
        SINGLE_COMPONENT { .. } => list![BVariable::getVarName(
            var_field!((**comp).var, NBStrongComponent::SINGLE_COMPONENT).clone()
        )],
        MULTI_COMPONENT { .. } => {
            ({
                let mut __acc: metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>> =
                    metamodelica::nil();
                for mut v in (var_field!((**comp).vars, NBStrongComponent::MULTI_COMPONENT).clone())
                    .into_iter()
                    .cloned()
                {
                    let __x = BVariable::getVarName(Slice::getT(v.clone()));
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            })
        }
        SLICED_COMPONENT { .. } => {
            if ComponentRef::isEmpty(var_field!((**comp).var_cref, NBStrongComponent::SLICED_COMPONENT)) {
                match '__try0: {
                    var_crefs = list![BVariable::getVarName(
                        unwrap_break_err!(Equation::getResidualVar(Slice::getT(var_field!((**comp).eqn, NBStrongComponent::SLICED_COMPONENT).clone())), '__try0)
                    )];
                    Ok::<_, &'static str>((var_crefs.clone(),))
                } {
                    Ok((__try0_o0,)) => {
                        var_crefs = __try0_o0;
                    }
                    Err(_) => {
                        var_crefs = list![var_field!((**comp).var_cref, NBStrongComponent::SLICED_COMPONENT).clone()];
                    }
                }
            } else {
                var_crefs = list![var_field!((**comp).var_cref, NBStrongComponent::SLICED_COMPONENT).clone()];
            }
            var_crefs
        }
        RESIZABLE_COMPONENT { .. } => {
            list![var_field!((**comp).var_cref, NBStrongComponent::RESIZABLE_COMPONENT).clone()]
        }
        GENERIC_COMPONENT { .. } => list![var_field!((**comp).var_cref, NBStrongComponent::GENERIC_COMPONENT).clone()],
        ENTWINED_COMPONENT { .. } => List::flatten(
            ({
                let mut __acc: metamodelica::List<metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>>> =
                    metamodelica::nil();
                for mut slice in (var_field!((**comp).entwined_slices, NBStrongComponent::ENTWINED_COMPONENT).clone())
                    .into_iter()
                    .cloned()
                {
                    let __x = ({
                        let mut __acc: metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>> =
                            metamodelica::nil();
                        for mut var in (getVariables(slice.clone())?).into_iter().cloned() {
                            let __x = BVariable::getVarName(var.clone());
                            __acc = cons(__x, __acc);
                        }
                        __acc.reverse()
                    });
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            }),
        )?,
        ALGEBRAIC_LOOP { .. } => {
            ({
                let mut __acc: metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>> =
                    metamodelica::nil();
                for mut var in (Tearing::getVariables(var_field!((**comp).strict, NBStrongComponent::ALGEBRAIC_LOOP))?)
                    .into_iter()
                    .cloned()
                {
                    let __x = BVariable::getVarName(var.clone());
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            })
        }
        ALIAS { .. } => getVariableCrefs(var_field!((**comp).original, NBStrongComponent::ALIAS))?,
        _ => {
            Error::addMessage(
                Error::INTERNAL_ERROR.clone(),
                list![{
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!("NBStrongComponent.getVariableCrefs"));
                    __mm_s.push_str(&*literal!(" failed because of wrong component: "));
                    __mm_s.push_str(&*toString(comp, -1)?);
                    ArcStr::from(__mm_s)
                }],
            )?;
            return Err("fail");
        }
    });
    Ok(var_crefs)
}

pub(crate) fn getVarCref<'__b>(
    mut comp: &'__b metamodelica::Ref<NBStrongComponent>,
) -> Result<metamodelica::Ref<ComponentRef::NFComponentRef>> {
    '__tco: loop {
        match &**comp {
            SLICED_COMPONENT { .. } => {
                return Ok(var_field!((**comp).var_cref, NBStrongComponent::SLICED_COMPONENT).clone());
            }
            RESIZABLE_COMPONENT { .. } => {
                return Ok(var_field!((**comp).var_cref, NBStrongComponent::RESIZABLE_COMPONENT).clone());
            }
            GENERIC_COMPONENT { .. } => {
                return Ok(var_field!((**comp).var_cref, NBStrongComponent::GENERIC_COMPONENT).clone());
            }
            ALIAS { .. } => {
                comp = var_field!((**comp).original, NBStrongComponent::ALIAS);
                continue '__tco;
            }
            _ => {
                Error::addMessage(
                    Error::INTERNAL_ERROR.clone(),
                    list![{
                        let mut __mm_s = String::new();
                        __mm_s.push_str(&*literal!("NBStrongComponent.getVarCref"));
                        __mm_s.push_str(&*literal!(" failed because of wrong component: "));
                        __mm_s.push_str(&*toString(comp, -1)?);
                        ArcStr::from(__mm_s)
                    }],
                )?;
                return Ok(return Err("fail"));
            }
        }
    }
}

pub(crate) fn getEquations(
    mut comp: metamodelica::Ref<NBStrongComponent>,
) -> Result<metamodelica::List<Pointer::Pointer<metamodelica::Ref<Equation::Equation>>>> {
    '__tco: loop {
        match &*comp {
            SINGLE_COMPONENT { eqn: __comp_eqn, .. } => return Ok(list![__comp_eqn.clone()]),
            MULTI_COMPONENT { eqn: __comp_eqn, .. } => return Ok(list![Slice::getT(__comp_eqn.clone())]),
            SLICED_COMPONENT { eqn: __comp_eqn, .. } => return Ok(list![Slice::getT(__comp_eqn.clone())]),
            RESIZABLE_COMPONENT { eqn: __comp_eqn, .. } => return Ok(list![Slice::getT(__comp_eqn.clone())]),
            GENERIC_COMPONENT { eqn: __comp_eqn, .. } => return Ok(list![Slice::getT(__comp_eqn.clone())]),
            ENTWINED_COMPONENT {
                entwined_slices: __comp_entwined_slices,
                ..
            } => {
                return Ok(List::flatten(
                    ({
                        let mut __acc: metamodelica::List<
                            metamodelica::List<Pointer::Pointer<metamodelica::Ref<Equation::Equation>>>,
                        > = metamodelica::nil();
                        for mut slice in (__comp_entwined_slices.clone()).into_iter().cloned() {
                            let __x = getEquations(slice.clone())?;
                            __acc = cons(__x, __acc);
                        }
                        __acc.reverse()
                    }),
                )?);
            }
            ALGEBRAIC_LOOP {
                strict: __comp_strict, ..
            } => return Ok(Tearing::getResidualEqns(metamodelica::AsArg::as_arg(&__comp_strict))),
            ALIAS {
                original: __comp_original,
                ..
            } => {
                comp = __comp_original.clone();
                continue '__tco;
            }
            _ => {
                Error::addMessage(
                    Error::INTERNAL_ERROR.clone(),
                    list![{
                        let mut __mm_s = String::new();
                        __mm_s.push_str(&*literal!("NBStrongComponent.getEquations"));
                        __mm_s.push_str(&*literal!(" failed because of wrong component: "));
                        __mm_s.push_str(&*toString(&comp, -1)?);
                        ArcStr::from(__mm_s)
                    }],
                )?;
                return Ok(return Err("fail"));
            }
        }
    }
}

pub(crate) fn getSolveStatus<'__b>(mut comp: &'__b metamodelica::Ref<NBStrongComponent>) -> Result<Solve::Status> {
    '__tco: loop {
        match &**comp {
            SINGLE_COMPONENT { .. } => {
                return Ok(var_field!((**comp).status, NBStrongComponent::SINGLE_COMPONENT).clone());
            }
            MULTI_COMPONENT { .. } => {
                return Ok(var_field!((**comp).status, NBStrongComponent::MULTI_COMPONENT).clone());
            }
            SLICED_COMPONENT { .. } => {
                return Ok(var_field!((**comp).status, NBStrongComponent::SLICED_COMPONENT).clone());
            }
            RESIZABLE_COMPONENT { .. } => {
                return Ok(var_field!((**comp).status, NBStrongComponent::RESIZABLE_COMPONENT).clone());
            }
            GENERIC_COMPONENT { .. } => return Ok(Solve::Status::EXPLICIT.clone()),
            ENTWINED_COMPONENT { .. } => return Ok(Solve::Status::EXPLICIT.clone()),
            ALGEBRAIC_LOOP { .. } => return Ok(var_field!((**comp).status, NBStrongComponent::ALGEBRAIC_LOOP).clone()),
            ALIAS { .. } => {
                comp = var_field!((**comp).original, NBStrongComponent::ALIAS);
                continue '__tco;
            }
            _ => {
                Error::addMessage(
                    Error::INTERNAL_ERROR.clone(),
                    list![{
                        let mut __mm_s = String::new();
                        __mm_s.push_str(&*literal!("NBStrongComponent.getSolveStatus"));
                        __mm_s.push_str(&*literal!(" failed because of wrong component: "));
                        __mm_s.push_str(&*toString(comp, -1)?);
                        ArcStr::from(__mm_s)
                    }],
                )?;
                return Ok(return Err("fail"));
            }
        }
    }
}

pub(crate) fn isDiscrete<'__b>(mut comp: &'__b metamodelica::Ref<NBStrongComponent>) -> Result<bool> {
    '__tco: loop {
        match &**comp {
            SINGLE_COMPONENT { .. } => {
                return Ok(Equation::isDiscrete(
                    var_field!((**comp).eqn, NBStrongComponent::SINGLE_COMPONENT).clone(),
                ));
            }
            MULTI_COMPONENT { .. } => {
                return Ok(Equation::isDiscrete(Slice::getT(
                    var_field!((**comp).eqn, NBStrongComponent::MULTI_COMPONENT).clone(),
                )) || List::any(
                    &({
                        let mut __acc: metamodelica::List<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>> =
                            metamodelica::nil();
                        for mut v in (var_field!((**comp).vars, NBStrongComponent::MULTI_COMPONENT).clone())
                            .into_iter()
                            .cloned()
                        {
                            let __x = Slice::getT(v.clone());
                            __acc = cons(__x, __acc);
                        }
                        __acc.reverse()
                    }),
                    &fnptr!(
                        BVariable::isDiscrete,
                        Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>
                    ),
                )?);
            }
            SLICED_COMPONENT { .. } => {
                return Ok(Equation::isDiscrete(Slice::getT(
                    var_field!((**comp).eqn, NBStrongComponent::SLICED_COMPONENT).clone(),
                )));
            }
            RESIZABLE_COMPONENT { .. } => {
                return Ok(Equation::isDiscrete(Slice::getT(
                    var_field!((**comp).eqn, NBStrongComponent::RESIZABLE_COMPONENT).clone(),
                )));
            }
            ENTWINED_COMPONENT { .. } => {
                return Ok(List::all(
                    var_field!((**comp).entwined_slices, NBStrongComponent::ENTWINED_COMPONENT),
                    &move |__a0: metamodelica::Ref<NBStrongComponent>| isDiscrete(&__a0),
                )?);
            }
            GENERIC_COMPONENT { .. } => {
                return Ok(Equation::isDiscrete(Slice::getT(
                    var_field!((**comp).eqn, NBStrongComponent::GENERIC_COMPONENT).clone(),
                )));
            }
            ALGEBRAIC_LOOP { .. } => return Ok(var_field!((**comp).mixed, NBStrongComponent::ALGEBRAIC_LOOP).clone()),
            ALIAS { .. } => {
                comp = var_field!((**comp).original, NBStrongComponent::ALIAS);
                continue '__tco;
            }
            _ => {
                Error::addMessage(
                    Error::INTERNAL_ERROR.clone(),
                    list![{
                        let mut __mm_s = String::new();
                        __mm_s.push_str(&*literal!("NBStrongComponent.isDiscrete"));
                        __mm_s.push_str(&*literal!(" failed because of wrong component: "));
                        __mm_s.push_str(&*toString(comp, -1)?);
                        ArcStr::from(__mm_s)
                    }],
                )?;
                return Ok(return Err("fail"));
            }
        }
    }
}

pub(crate) fn isDummy(mut comp: &metamodelica::Ref<NBStrongComponent>) -> bool {
    let mut b: bool;
    b = (match &**comp {
        SINGLE_COMPONENT { eqn: __comp_eqn, .. } => Equation::isDummy(&(Pointer::access(__comp_eqn.clone()))),
        MULTI_COMPONENT { eqn: __comp_eqn, .. } => {
            Equation::isDummy(&(Pointer::access(Slice::getT(__comp_eqn.clone()))))
        }
        _ => false,
    });
    b
}

pub(crate) fn isAlias(mut comp: &metamodelica::Ref<NBStrongComponent>) -> bool {
    let mut b: bool;
    b = (match &**comp {
        ALIAS { .. } => true,
        _ => false,
    });
    b
}

pub(crate) fn isSingleComponent(mut comp: metamodelica::Ref<NBStrongComponent>) -> bool {
    let mut b: bool;
    b = (match &*(removeAlias(comp)) {
        SINGLE_COMPONENT { .. } => true,
        _ => false,
    });
    b
}

pub(crate) fn isAlgebraicLoop(mut comp: metamodelica::Ref<NBStrongComponent>) -> bool {
    let mut b: bool;
    b = (match &*(removeAlias(comp)) {
        ALGEBRAIC_LOOP { .. } => true,
        _ => false,
    });
    b
}

pub(crate) fn setHomotopy(
    mut comp: metamodelica::Ref<NBStrongComponent>,
    mut homotopy: bool,
) -> metamodelica::Ref<NBStrongComponent> {
    let mut comp: metamodelica::Ref<NBStrongComponent> = comp;
    comp = (match &*comp {
        ALGEBRAIC_LOOP { .. } => {
            assign_variant_field!(comp => NBStrongComponent::ALGEBRAIC_LOOP; homotopy = homotopy);
            comp
        }
        _ => comp,
    });
    comp
}

pub(crate) fn createPseudoScalar(
    mut comp_indices: &metamodelica::List<i32>,
    mut eqn_to_var: metamodelica::Array<i32>,
    mut mapping: &metamodelica::Ref<Mapping::Mapping>,
    mut vars: &metamodelica::Ref<VariablePointers::VariablePointers>,
    mut eqns: &metamodelica::Ref<EquationPointers::EquationPointers>,
) -> Result<metamodelica::Ref<NBStrongComponent>> {
    let mut comp: metamodelica::Ref<NBStrongComponent>;
    comp = ({
        let mut homotopy: Pointer::Pointer<bool> = Pointer::create(false);
        (::match_deref::match_deref! { match comp_indices {
            Deref @ metamodelica::ListNode::Cons { head: i, tail: Deref @ metamodelica::ListNode::Nil } => {
                let mut var_scal_idx: i32;
                let mut var_arr_idx: i32;
                let mut var: Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>;
                let mut eqn: Pointer::Pointer<metamodelica::Ref<Equation::Equation>>;
                let mut var_slice: metamodelica::Ref<Slice::NBSlice<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>>>;
                let mut eqn_slice: metamodelica::Ref<Slice::NBSlice<Pointer::Pointer<metamodelica::Ref<Equation::Equation>>>>;
                var_scal_idx = ({let __elt = (*metamodelica::index_checked(&eqn_to_var.borrow(), i.clone())?).clone(); __elt});
                var_arr_idx = ({let __elt = (*metamodelica::index_checked(&mapping.var_StA.borrow(), var_scal_idx)?).clone(); __elt});
                var = BVariable::VariablePointers::getVarAt(vars, var_arr_idx)?;
                eqn = EquationPointers::getEqnAt(eqns, ({let __elt = (*metamodelica::index_checked(&mapping.eqn_StA.borrow(), i.clone())?).clone(); __elt}))?;
                if Equation::isForEquation(eqn.clone()) {
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(getLoopVarsAndEqns(comp_indices, eqn_to_var.clone(), mapping, vars, eqns)) {
                        Ok((Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: Deref @ metamodelica::ListNode::Nil }, Deref @ metamodelica::ListNode::Cons { head: __pa1, tail: Deref @ metamodelica::ListNode::Nil })) => (__pa0.clone(), __pa1.clone()),
                        _ => {
                        Error::addMessage(Error::INTERNAL_ERROR.clone(), list![{ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBStrongComponent.createPseudoScalar")); __mm_s.push_str(&*literal!(" failed because single indices did not turn out to be single components.")); ArcStr::from(__mm_s) }])?;
                        return Err("fail");
                        },
                    } };
                    var_slice = metamodelica::Own::own(__pa0);
                    eqn_slice = metamodelica::Own::own(__pa1);
                    comp = metamodelica::Ref::new(NBStrongComponent::SLICED_COMPONENT { var_cref: BVariable::VariablePointers::varSlice(vars, var_scal_idx, ({let __elt = (*metamodelica::index_checked(&mapping.var_StA.borrow(), var_scal_idx)?).clone(); __elt}), mapping, true)?, var: var_slice, eqn: eqn_slice, status: Solve::Status::UNPROCESSED.clone() });
                } else if Equation::isCompound(eqn.clone()) {
                    comp = metamodelica::Ref::new(NBStrongComponent::MULTI_COMPONENT { vars: list![metamodelica::Ref::new(Slice::NBSlice { t: var, indices: metamodelica::nil() })], eqn: metamodelica::Ref::new(Slice::NBSlice { t: eqn, indices: metamodelica::nil() }), status: Solve::Status::UNPROCESSED.clone() });
                } else {
                    let (__pa4, __pa5) = ::match_deref::match_deref! { match &(getLoopVarsAndEqns(comp_indices, eqn_to_var.clone(), mapping, vars, eqns)) {
                        Ok((Deref @ metamodelica::ListNode::Cons { head: __pa4, tail: Deref @ metamodelica::ListNode::Nil }, Deref @ metamodelica::ListNode::Cons { head: __pa5, tail: Deref @ metamodelica::ListNode::Nil })) => (__pa4.clone(), __pa5.clone()),
                        _ => {
                        Error::addMessage(Error::INTERNAL_ERROR.clone(), list![{ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBStrongComponent.createPseudoScalar")); __mm_s.push_str(&*literal!(" failed because single indices did not turn out to be single components.")); ArcStr::from(__mm_s) }])?;
                        return Err("fail");
                        },
                    } };
                    var_slice = metamodelica::Own::own(__pa4);
                    eqn_slice = metamodelica::Own::own(__pa5);
                    comp = createSliceOrSingle(BVariable::VariablePointers::varSlice(vars, var_scal_idx, ({let __elt = (*metamodelica::index_checked(&mapping.var_StA.borrow(), var_scal_idx)?).clone(); __elt}), mapping, true)?, var_slice, eqn_slice)?;
                }
                comp
            },
            _ => {
                let mut eqn: Pointer::Pointer<metamodelica::Ref<Equation::Equation>>;
                let mut comp_vars: metamodelica::List<metamodelica::Ref<Slice::NBSlice<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>>>>;
                let mut comp_eqns: metamodelica::List<metamodelica::Ref<Slice::NBSlice<Pointer::Pointer<metamodelica::Ref<Equation::Equation>>>>>;
                let mut tearingSet: metamodelica::Ref<Tearing::NBTearing>;
                let mut var_slice: metamodelica::Ref<Slice::NBSlice<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>>>;
                let mut eqn_slice: metamodelica::Ref<Slice::NBSlice<Pointer::Pointer<metamodelica::Ref<Equation::Equation>>>>;
                let mut resolved_cref: metamodelica::Ref<ComponentRef::NFComponentRef>;
                (comp_vars, comp_eqns) = getLoopVarsAndEqns(comp_indices, eqn_to_var.clone(), mapping, vars, eqns)?;
                comp = (::match_deref::match_deref! { match &((comp_vars.clone(), comp_eqns.clone())) {
            (Deref @ metamodelica::ListNode::Cons { head: __esc_var_slice, tail: Deref @ metamodelica::ListNode::Nil }, Deref @ metamodelica::ListNode::Cons { head: eqn_slice, tail: Deref @ metamodelica::ListNode::Nil }) if (!(Equation::isForEquation(Slice::getT(eqn_slice.clone())) || Equation::isAlgorithm(Slice::getT(eqn_slice.clone())))) => {
                var_slice = (*__esc_var_slice).clone();
                resolved_cref = if (Slice::isFull(var_slice.clone())) {BVariable::getVarName(Slice::getT(var_slice.clone()))} else {Slice::resolveSlicedCref(BVariable::getVarName(Slice::getT(var_slice.clone())), Pointer::access(Slice::getT(eqn_slice.clone())), Slice::size(var_slice.clone(), &({ let __pe_b1 = false; move |__pe_a0| BVariable::size(__pe_a0, __pe_b1.clone()) }))?)?};
                createSliceOrSingle(resolved_cref, var_slice.clone(), eqn_slice.clone())?
            },
            (_, Deref @ metamodelica::ListNode::Cons { head: eqn_slice, tail: Deref @ metamodelica::ListNode::Nil }) if (!(Equation::isForEquation(Slice::getT(eqn_slice.clone()))) || Equation::isRecordOrTupleEquation(Slice::getT(eqn_slice.clone()))?) => metamodelica::Ref::new(NBStrongComponent::MULTI_COMPONENT { vars: comp_vars, eqn: eqn_slice.clone(), status: Solve::Status::UNPROCESSED.clone() }),
            _ => {
                tearingSet = metamodelica::Ref::new(Tearing::NBTearing { iteration_vars: comp_vars, residual_eqns: comp_eqns.clone(), innerEquations: metamodelica::arrayFromVec(metamodelica::nil().into_iter().cloned().collect()), jac: None });
                for mut eqn in &*comp_eqns {
                    let mut eqn = eqn.clone();
                    Equation::map(Pointer::access(Slice::getT(eqn)), (std::sync::Arc::new({ let __pe_b1 = homotopy.clone(); move |__pe_a0| Initialization::containsHomotopyCall(__pe_a0, __pe_b1.clone()) }) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<Expression::NFExpression>) -> Result<metamodelica::Ref<Expression::NFExpression>> + 'static>), None, (std::sync::Arc::new(Expression::map) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<Expression::NFExpression>, Arc<dyn ::std::ops::Fn(metamodelica::Ref<Expression::NFExpression>) -> Result<metamodelica::Ref<Expression::NFExpression>> + 'static>) -> Result<metamodelica::Ref<Expression::NFExpression>> + 'static>))?;
                }
                metamodelica::Ref::new(NBStrongComponent::ALGEBRAIC_LOOP { idx: -1, strict: tearingSet, casual: None, linear: false, mixed: false, homotopy: Pointer::access(homotopy), status: Solve::Status::IMPLICIT.clone(), implicitlyCreated: false })
            },
            _ => unreachable!("match_deref! exhaustiveness placeholder"),
        } });
                comp
            },
            _ => {
                Error::addMessage(Error::INTERNAL_ERROR.clone(), list![{ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NBStrongComponent.createPseudoScalar")); __mm_s.push_str(&*literal!(" failed.")); ArcStr::from(__mm_s) }])?;
                return Err("fail")
            },
            _ => unreachable!("match_deref! exhaustiveness placeholder"),
        } })
    });
    Ok(comp)
}

pub(crate) fn createSliceOrSingle(
    mut cref: metamodelica::Ref<ComponentRef::NFComponentRef>,
    mut var_slice: metamodelica::Ref<Slice::NBSlice<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>>>,
    mut eqn_slice: metamodelica::Ref<Slice::NBSlice<Pointer::Pointer<metamodelica::Ref<Equation::Equation>>>>,
) -> Result<metamodelica::Ref<NBStrongComponent>> {
    let mut comp: metamodelica::Ref<NBStrongComponent>;
    if Slice::isFull(var_slice.clone()) && Slice::isFull(eqn_slice.clone()) && !(ComponentRef::hasSubscripts(&cref)?) {
        comp = metamodelica::Ref::new(NBStrongComponent::SINGLE_COMPONENT {
            var: Slice::getT(var_slice),
            eqn: Slice::getT(eqn_slice),
            status: Solve::Status::UNPROCESSED.clone(),
        });
    } else {
        comp = metamodelica::Ref::new(NBStrongComponent::SLICED_COMPONENT {
            var_cref: cref,
            var: var_slice,
            eqn: eqn_slice,
            status: Solve::Status::UNPROCESSED.clone(),
        });
    }
    Ok(comp)
}

fn getLoopVarsAndEqns(
    mut comp_indices: &metamodelica::List<i32>,
    mut eqn_to_var: metamodelica::Array<i32>,
    mut mapping: &metamodelica::Ref<Mapping::Mapping>,
    mut vars: &metamodelica::Ref<VariablePointers::VariablePointers>,
    mut eqns: &metamodelica::Ref<EquationPointers::EquationPointers>,
) -> Result<(
    metamodelica::List<metamodelica::Ref<Slice::NBSlice<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>>>>,
    metamodelica::List<metamodelica::Ref<Slice::NBSlice<Pointer::Pointer<metamodelica::Ref<Equation::Equation>>>>>,
)> {
    let __ab_eqn_to_var = eqn_to_var.borrow();
    let mut acc_vars: metamodelica::List<
        metamodelica::Ref<Slice::NBSlice<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>>>,
    > = metamodelica::nil();
    let mut acc_eqns: metamodelica::List<
        metamodelica::Ref<Slice::NBSlice<Pointer::Pointer<metamodelica::Ref<Equation::Equation>>>>,
    > = metamodelica::nil();
    let mut var_idx: i32;
    let mut var_arr_idx: i32;
    let mut var_scal_idx: i32;
    let mut eqn_arr_idx: i32;
    let mut eqn_scal_idx: i32;
    let mut idx_lst: metamodelica::List<i32>;
    let mut var: Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>;
    let mut eqn: Pointer::Pointer<metamodelica::Ref<Equation::Equation>>;
    let mut len_comps: i32 = ((comp_indices).len() as i32);
    let mut var_map: metamodelica::Ref<UnorderedMap::UnorderedMap<i32, metamodelica::List<i32>>> = UnorderedMap::new(
        std::sync::Arc::new(fnptr!(Util::id, _)),
        (std::sync::Arc::new(fnptr!(intEq, i32, i32))
            as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>),
        len_comps,
    );
    let mut eqn_map: metamodelica::Ref<UnorderedMap::UnorderedMap<i32, metamodelica::List<i32>>> = UnorderedMap::new(
        std::sync::Arc::new(fnptr!(Util::id, _)),
        (std::sync::Arc::new(fnptr!(intEq, i32, i32))
            as std::sync::Arc<dyn ::std::ops::Fn(i32, i32) -> Result<bool> + 'static>),
        len_comps,
    );
    for mut eqn_idx in &**comp_indices {
        var_idx = (*metamodelica::index_checked(&__ab_eqn_to_var, eqn_idx.clone())?).clone();
        var_arr_idx = ({
            let __elt = (*metamodelica::index_checked(&mapping.var_StA.borrow(), var_idx)?).clone();
            __elt
        });
        eqn_arr_idx = ({
            let __elt = (*metamodelica::index_checked(&mapping.eqn_StA.borrow(), eqn_idx.clone())?).clone();
            __elt
        });
        idx_lst = UnorderedMap::getOrDefault(var_arr_idx, var_map.clone(), metamodelica::nil())?;
        UnorderedMap::add(var_arr_idx, metamodelica::cons(var_idx, idx_lst), var_map.clone())?;
        idx_lst = UnorderedMap::getOrDefault(eqn_arr_idx, eqn_map.clone(), metamodelica::nil())?;
        UnorderedMap::add(
            eqn_arr_idx,
            metamodelica::cons(eqn_idx.clone(), idx_lst),
            eqn_map.clone(),
        )?;
    }
    for mut tpl in &*UnorderedMap::toList(var_map) {
        (var_arr_idx, idx_lst) = tpl.clone();
        (var_scal_idx, _) = ({
            let __elt = (*metamodelica::index_checked(&mapping.var_AtS.borrow(), var_arr_idx)?).clone();
            __elt
        });
        var = BVariable::VariablePointers::getVarAt(vars, var_arr_idx)?;
        idx_lst = if (((idx_lst).len() as i32) == BVariable::size(var.clone(), false)?) {
            metamodelica::nil()
        } else {
            ({
                let mut __acc: metamodelica::List<i32> = metamodelica::nil();
                for mut i in (idx_lst).into_iter().cloned() {
                    let __x = i.clone() - var_scal_idx;
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            })
        };
        acc_vars = metamodelica::cons(
            metamodelica::Ref::new(Slice::NBSlice {
                t: var,
                indices: sortAscending(idx_lst)?,
            }),
            acc_vars,
        );
    }
    for mut tpl in &*UnorderedMap::toList(eqn_map) {
        (eqn_arr_idx, idx_lst) = tpl.clone();
        (eqn_scal_idx, _) = ({
            let __elt = (*metamodelica::index_checked(&mapping.eqn_AtS.borrow(), eqn_arr_idx)?).clone();
            __elt
        });
        eqn = EquationPointers::getEqnAt(eqns, eqn_arr_idx)?;
        idx_lst = if (((idx_lst).len() as i32) == Equation::size(eqn.clone(), false)?) {
            metamodelica::nil()
        } else {
            ({
                let mut __acc: metamodelica::List<i32> = metamodelica::nil();
                for mut i in (idx_lst).into_iter().cloned() {
                    let __x = i.clone() - eqn_scal_idx;
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            })
        };
        acc_eqns = metamodelica::cons(
            metamodelica::Ref::new(Slice::NBSlice {
                t: eqn,
                indices: sortAscending(idx_lst)?,
            }),
            acc_eqns,
        );
    }
    Ok((acc_vars, acc_eqns))
}

fn sortAscending(mut lst: metamodelica::List<i32>) -> Result<metamodelica::List<i32>> {
    let mut sorted: metamodelica::List<i32>;
    if (lst).is_empty() || ((lst).rest()?).is_empty() {
        sorted = lst;
    } else {
        sorted = Array::heapSort(metamodelica::arrayFromVec(lst.into_iter().cloned().collect()))?
            .borrow()
            .iter()
            .cloned()
            .collect::<metamodelica::List<_>>();
    }
    Ok(sorted)
}

fn updateDependencyMap(
    mut cref: metamodelica::Ref<ComponentRef::NFComponentRef>,
    mut dependencies: metamodelica::Ref<UnorderedSet::UnorderedSet<metamodelica::Ref<ComponentRef::NFComponentRef>>>,
    mut map: metamodelica::Ref<
        UnorderedMap::UnorderedMap<
            metamodelica::Ref<ComponentRef::NFComponentRef>,
            metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>>,
        >,
    >,
) -> Result<()> {
    let mut removed: bool;
    removed = UnorderedSet::remove(cref.clone(), dependencies.clone())?;
    UnorderedMap::add(cref.clone(), UnorderedSet::toList(dependencies.clone()), map)?;
    if removed {
        UnorderedSet::addNew(cref, dependencies)?;
    }
    Ok(())
}

fn prepareDependencies(
    mut dependencies: metamodelica::Ref<UnorderedSet::UnorderedSet<metamodelica::Ref<ComponentRef::NFComponentRef>>>,
    mut map: metamodelica::Ref<
        UnorderedMap::UnorderedMap<
            metamodelica::Ref<ComponentRef::NFComponentRef>,
            metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>>,
        >,
    >,
    mut jacType: JacobianType,
) -> Result<metamodelica::Ref<UnorderedSet::UnorderedSet<metamodelica::Ref<ComponentRef::NFComponentRef>>>> {
    fn addSubDependencies(
        mut dep: metamodelica::Ref<ComponentRef::NFComponentRef>,
        mut map: metamodelica::Ref<
            UnorderedMap::UnorderedMap<
                metamodelica::Ref<ComponentRef::NFComponentRef>,
                metamodelica::List<metamodelica::Ref<ComponentRef::NFComponentRef>>,
            >,
        >,
        mut checkFn: &dyn ::std::ops::Fn(Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>) -> Result<bool>,
        mut set: metamodelica::Ref<UnorderedSet::UnorderedSet<metamodelica::Ref<ComponentRef::NFComponentRef>>>,
    ) -> Result<metamodelica::Ref<UnorderedSet::UnorderedSet<metamodelica::Ref<ComponentRef::NFComponentRef>>>> {
        let mut set: metamodelica::Ref<UnorderedSet::UnorderedSet<metamodelica::Ref<ComponentRef::NFComponentRef>>> =
            set;
        if BVariable::checkCref(
            &dep,
            checkFn,
            metamodelica::sourceInfo!("NBackEnd/Classes/NBStrongComponent.mo"),
        )? {
            UnorderedSet::add(dep, set.clone())?;
        } else {
            for mut tmp in &*UnorderedMap::getSafe(
                dep,
                map,
                metamodelica::sourceInfo!("NBackEnd/Classes/NBStrongComponent.mo"),
            )? {
                UnorderedSet::add(tmp.clone(), set.clone())?;
            }
        }
        Ok(set)
    }

    let mut dependencies: metamodelica::Ref<
        UnorderedSet::UnorderedSet<metamodelica::Ref<ComponentRef::NFComponentRef>>,
    > = dependencies;
    dependencies = UnorderedSet::selfMap(
        dependencies,
        &({
            let __pe_b1: Arc<
                dyn ::std::ops::Fn(
                        metamodelica::Ref<Expression::NFExpression>,
                    ) -> Result<metamodelica::Ref<Expression::NFExpression>>
                    + 'static,
            > = (std::sync::Arc::new(Expression::replaceResizableParameter)
                as std::sync::Arc<
                    dyn ::std::ops::Fn(
                            metamodelica::Ref<Expression::NFExpression>,
                        ) -> Result<metamodelica::Ref<Expression::NFExpression>>
                        + 'static,
                >);
            move |__pe_a0| ComponentRef::mapExp(&__pe_a0, __pe_b1.clone())
        }),
    )?;
    dependencies = UnorderedSet::selfMap(
        dependencies,
        &({
            let __pe_b1 = false;
            move |__pe_a0| ComponentRef::simplifySubscripts(__pe_a0, __pe_b1.clone())
        }),
    )?;
    dependencies = (match jacType {
        JacobianType::ODE => UnorderedSet::fold(
            dependencies,
            &({
                let __pe_b1 = map;
                let __pe_b2 = (std::sync::Arc::new(fnptr!(
                    BVariable::isState,
                    Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>
                ))
                    as std::sync::Arc<
                        dyn ::std::ops::Fn(Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>) -> Result<bool>
                            + 'static,
                    >);
                move |__pe_a0, __pe_a3| addSubDependencies(__pe_a0, __pe_b1.clone(), &*__pe_b2, __pe_a3)
            }),
            UnorderedSet::new(
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
                13,
            ),
        )?,
        JacobianType::OPT_LFG => UnorderedSet::fold(
            dependencies,
            &({
                let __pe_b1 = map;
                let __pe_b2 = (std::sync::Arc::new(fnptr!(
                    BVariable::isStateOrOptimizable,
                    Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>
                ))
                    as std::sync::Arc<
                        dyn ::std::ops::Fn(Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>) -> Result<bool>
                            + 'static,
                    >);
                move |__pe_a0, __pe_a3| addSubDependencies(__pe_a0, __pe_b1.clone(), &*__pe_b2, __pe_a3)
            }),
            UnorderedSet::new(
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
                13,
            ),
        )?,
        JacobianType::OPT_MRF => UnorderedSet::fold(
            dependencies,
            &({
                let __pe_b1 = map;
                let __pe_b2 = (std::sync::Arc::new(fnptr!(
                    BVariable::isStateOrOptimizable,
                    Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>
                ))
                    as std::sync::Arc<
                        dyn ::std::ops::Fn(Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>) -> Result<bool>
                            + 'static,
                    >);
                move |__pe_a0, __pe_a3| addSubDependencies(__pe_a0, __pe_b1.clone(), &*__pe_b2, __pe_a3)
            }),
            UnorderedSet::new(
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
                13,
            ),
        )?,
        JacobianType::OPT_R0 => UnorderedSet::fold(
            dependencies,
            &({
                let __pe_b1 = map;
                let __pe_b2 = (std::sync::Arc::new(fnptr!(
                    BVariable::isStateOrOptimizable,
                    Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>
                ))
                    as std::sync::Arc<
                        dyn ::std::ops::Fn(Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>) -> Result<bool>
                            + 'static,
                    >);
                move |__pe_a0, __pe_a3| addSubDependencies(__pe_a0, __pe_b1.clone(), &*__pe_b2, __pe_a3)
            }),
            UnorderedSet::new(
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
                13,
            ),
        )?,
        _ => dependencies,
    });
    Ok(dependencies)
}
