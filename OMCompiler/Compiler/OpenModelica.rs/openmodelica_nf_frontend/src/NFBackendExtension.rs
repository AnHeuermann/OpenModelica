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

use crate::NFAttributes as Attributes;
use crate::NFBinding as Binding;
use crate::NFBinding::Source;
use crate::NFCall as Call;
use crate::NFCeval as Ceval;
use crate::NFComplexType as ComplexType;
use crate::NFComponent as Component;
use crate::NFComponentRef as ComponentRef;
use crate::NFDimension as Dimension;
use crate::NFExpression as Expression;
use crate::NFExpressionIterator as ExpressionIterator;
use crate::NFFunction::Function;
use crate::NFInstNode::InstNode;
use crate::NFPrefixes;
use crate::NFPrefixes::Direction;
use crate::NFPrefixes::Variability;
use crate::NFType as Type;
use crate::NFVariable as Variable;
use openmodelica_ast::Absyn;
use openmodelica_frontend_dump::AbsynUtil;
use openmodelica_frontend_dump::Dump;
use openmodelica_frontend_dump::SCodeUtil;
use openmodelica_frontend_types::DAE;
use openmodelica_frontend_types::SCode;
use openmodelica_util::Error;
use openmodelica_util::UnorderedMap;
use openmodelica_util::Util;
use openmodelica_util_datatypes_basic::List;
use openmodelica_util_datatypes_basic::Pointer;
use openmodelica_util_datatypes_basic::PointerWeak;

// OF imports
//NF imports
// Util imports
pub mod BackendInfo {
    use super::*;
    #[derive(Clone, Debug, Eq, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
    pub struct BackendInfo {
        /// Structural kind: state, algebraic...
        pub varKind: metamodelica::Ref<VariableKind::VariableKind>,
        /// values on built-in attributes
        pub attributes: metamodelica::Ref<VariableAttributes::VariableAttributes>,
        /// values on annotations (vendor specific)
        pub annotations: metamodelica::Ref<Annotations::Annotations>,
        /// Pointer (var -> pre) or (pre -> var) if existent.
        pub var_pre: Option<PointerWeak::PointerWeak<metamodelica::Ref<Variable::NFVariable>>>,
        /// Pointer (var -> seed) or (seed -> var) if existent.
        pub var_seed: Option<PointerWeak::PointerWeak<metamodelica::Ref<Variable::NFVariable>>>,
        /// Pointer (var -> pder, result var in Jacobian) or (pder -> var) if existent.
        pub var_pder_res: Option<PointerWeak::PointerWeak<metamodelica::Ref<Variable::NFVariable>>>,
        /// Pointer (var -> pder, tmp var in Jacobian) or (pder -> var) if existent.
        pub var_pder_tmp: Option<PointerWeak::PointerWeak<metamodelica::Ref<Variable::NFVariable>>>,
        /// Pointer (var -> start) or (start -> var) if existent.
        pub var_start: Option<PointerWeak::PointerWeak<metamodelica::Ref<Variable::NFVariable>>>,
        /// record parent if it is part of a record.
        pub parent: Option<PointerWeak::PointerWeak<metamodelica::Ref<Variable::NFVariable>>>,
    }

    impl metamodelica::gc::MMTrace for BackendInfo {
        fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
            metamodelica::gc::MMTrace::mm_accept(&self.varKind, __mmv)?;
            metamodelica::gc::MMTrace::mm_accept(&self.attributes, __mmv)?;
            metamodelica::gc::MMTrace::mm_accept(&self.annotations, __mmv)?;
            metamodelica::gc::MMTrace::mm_accept(&self.var_pre, __mmv)?;
            metamodelica::gc::MMTrace::mm_accept(&self.var_seed, __mmv)?;
            metamodelica::gc::MMTrace::mm_accept(&self.var_pder_res, __mmv)?;
            metamodelica::gc::MMTrace::mm_accept(&self.var_pder_tmp, __mmv)?;
            metamodelica::gc::MMTrace::mm_accept(&self.var_start, __mmv)?;
            metamodelica::gc::MMTrace::mm_accept(&self.parent, __mmv)?;
            Ok(())
        }
    }
    impl Default for BackendInfo {
        fn default() -> Self {
            Self {
                varKind: Default::default(),
                attributes: Default::default(),
                annotations: Default::default(),
                var_pre: Default::default(),
                var_seed: Default::default(),
                var_pder_res: Default::default(),
                var_pder_tmp: Default::default(),
                var_start: Default::default(),
                parent: Default::default(),
            }
        }
    }

    pub type BACKEND_INFO = BackendInfo;

    pub fn toString(mut backendInfo: &metamodelica::Ref<BackendInfo>) -> Result<ArcStr> {
        let mut r#str: ArcStr;
        r#str = VariableAttributes::toString(&backendInfo.attributes)?;
        r#str = {
            let mut __mm_s = String::new();
            __mm_s.push_str(&*VariableKind::toString(&backendInfo.varKind));
            __mm_s.push_str(&*if (metamodelica::stringEq(&r#str, &(literal!("")))) {
                literal!("")
            } else {
                {
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!(" "));
                    __mm_s.push_str(&*r#str);
                    ArcStr::from(__mm_s)
                }
            });
            ArcStr::from(__mm_s)
        };
        Ok(r#str)
    }

    pub fn map(
        mut binfo: metamodelica::Ref<BackendInfo>,
        mut func: Arc<
            dyn ::std::ops::Fn(
                    metamodelica::Ref<Expression::NFExpression>,
                ) -> Result<metamodelica::Ref<Expression::NFExpression>>
                + 'static,
        >,
    ) -> Result<metamodelica::Ref<BackendInfo>> {
        pub type expFunc = std::sync::Arc<
            dyn ::std::ops::Fn(
                    metamodelica::Ref<Expression::NFExpression>,
                ) -> Result<metamodelica::Ref<Expression::NFExpression>>
                + 'static,
        >;

        let mut binfo: metamodelica::Ref<BackendInfo> = binfo;
        assign_field!(binfo.attributes = VariableAttributes::map(binfo.attributes.clone(), func.clone())?);
        Ok(binfo)
    }

    pub fn getVarKind(mut binfo: &metamodelica::Ref<BackendInfo>) -> metamodelica::Ref<VariableKind::VariableKind> {
        let mut varKind: metamodelica::Ref<VariableKind::VariableKind> = binfo.varKind.clone();
        varKind
    }

    pub fn setVarKind(
        mut binfo: metamodelica::Ref<BackendInfo>,
        mut varKind: metamodelica::Ref<VariableKind::VariableKind>,
    ) -> metamodelica::Ref<BackendInfo> {
        let mut binfo: metamodelica::Ref<BackendInfo> = binfo;
        assign_field!(binfo.varKind = varKind);
        binfo
    }

    pub fn setStateSelect(
        mut info: metamodelica::Ref<BackendInfo>,
        mut stateSelect_val: StateSelect,
        mut overwrite: bool,
    ) -> metamodelica::Ref<BackendInfo> {
        let mut info: metamodelica::Ref<BackendInfo> = info;
        assign_field!(
            info.attributes = VariableAttributes::setStateSelect(info.attributes.clone(), stateSelect_val, overwrite)
        );
        info
    }

    pub fn setParent(
        mut binfo: metamodelica::Ref<BackendInfo>,
        mut parent: Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>,
    ) -> metamodelica::Ref<BackendInfo> {
        let mut binfo: metamodelica::Ref<BackendInfo> = binfo;
        assign_field!(binfo.parent = Some(PointerWeak::downgrade(parent)));
        binfo
    }

    pub fn weaken(
        mut strong: Option<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>>,
    ) -> Option<PointerWeak::PointerWeak<metamodelica::Ref<Variable::NFVariable>>> {
        let mut weak: Option<PointerWeak::PointerWeak<metamodelica::Ref<Variable::NFVariable>>>;
        weak = (match strong {
            Some(mut p) => Some(PointerWeak::downgrade(p)),
            _ => None,
        });
        weak
    }

    pub fn strengthen(
        mut weak: Option<PointerWeak::PointerWeak<metamodelica::Ref<Variable::NFVariable>>>,
    ) -> Option<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>> {
        let mut strong: Option<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>>;
        strong = 'mc: {
            let __mc_input = weak;
            if let Ok(__v) = (|| -> Result<_> {
                let Some(mut w) = __mc_input.clone() else {
                    return Err("nomatch");
                };
                Ok(Some(PointerWeak::upgrade(w.clone())?))
            })() {
                break 'mc __v;
            }
            if let Ok(__v) = (|| -> Result<_> {
                let _ = __mc_input.clone() else { return Err("nomatch") };
                Ok(None)
            })() {
                break 'mc __v;
            }
            panic!("matchcontinue: no arm matched")
        };
        strong
    }

    pub type setPartner = std::sync::Arc<
        dyn ::std::ops::Fn(
                metamodelica::Ref<BackendInfo>,
                Option<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>>,
            ) -> Result<metamodelica::Ref<BackendInfo>>
            + 'static,
    >;

    pub fn setVarPre(
        mut binfo: metamodelica::Ref<BackendInfo>,
        mut var_ptr: Option<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>>,
    ) -> metamodelica::Ref<BackendInfo> {
        let mut binfo: metamodelica::Ref<BackendInfo> = binfo;
        assign_field!(binfo.var_pre = weaken(var_ptr));
        binfo
    }

    pub fn setVarSeed(
        mut binfo: metamodelica::Ref<BackendInfo>,
        mut var_ptr: Option<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>>,
    ) -> metamodelica::Ref<BackendInfo> {
        let mut binfo: metamodelica::Ref<BackendInfo> = binfo;
        assign_field!(binfo.var_seed = weaken(var_ptr));
        binfo
    }

    pub fn setVarPDer(
        mut binfo: metamodelica::Ref<BackendInfo>,
        mut var_ptr: Option<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>>,
        mut isTmp: bool,
    ) -> metamodelica::Ref<BackendInfo> {
        let mut binfo: metamodelica::Ref<BackendInfo> = binfo;
        if isTmp {
            assign_field!(binfo.var_pder_tmp = weaken(var_ptr));
        } else {
            assign_field!(binfo.var_pder_res = weaken(var_ptr));
        }
        binfo
    }

    pub fn setVarStart(
        mut binfo: metamodelica::Ref<BackendInfo>,
        mut var_ptr: Option<Pointer::Pointer<metamodelica::Ref<Variable::NFVariable>>>,
    ) -> metamodelica::Ref<BackendInfo> {
        let mut binfo: metamodelica::Ref<BackendInfo> = binfo;
        assign_field!(binfo.var_start = weaken(var_ptr));
        binfo
    }

    pub fn setAttributes(
        mut binfo: metamodelica::Ref<BackendInfo>,
        mut attributes: metamodelica::Ref<VariableAttributes::VariableAttributes>,
        mut annotations: metamodelica::Ref<Annotations::Annotations>,
    ) -> metamodelica::Ref<BackendInfo> {
        let mut binfo: metamodelica::Ref<BackendInfo> = binfo;
        assign_field!(binfo.attributes = attributes, binfo.annotations = annotations);
        binfo
    }

    pub fn setHideResult(
        mut binfo: metamodelica::Ref<BackendInfo>,
        mut hideResult: bool,
    ) -> metamodelica::Ref<BackendInfo> {
        let mut binfo: metamodelica::Ref<BackendInfo> = binfo;
        binfo = (::match_deref::match_deref! { match &(binfo.clone()) {
            Deref @ BackendInfo { annotations: anno @ Deref @ Annotations::ANNOTATIONS { .. }, .. } => {
                let mut anno = (*anno).clone();
                assign_field!(anno.hideResult = hideResult);
                assign_field!(binfo.annotations = anno.clone());
                binfo
            },
            _ => {
                binfo
            },
            _ => unreachable!("match_deref! exhaustiveness placeholder"),
        } });
        binfo
    }

    pub(crate) fn scalarize(
        mut binfo: metamodelica::Ref<BackendInfo>,
        mut length: i32,
    ) -> Result<metamodelica::List<metamodelica::Ref<BackendInfo>>> {
        let mut binfo_list: metamodelica::List<metamodelica::Ref<BackendInfo>>;
        binfo_list = (match &*binfo.varKind.clone() {
            VariableKind::FRONTEND_DUMMY => List::fill(binfo, length),
            _ => {
                let mut scalar_attributes: metamodelica::List<
                    metamodelica::Ref<VariableAttributes::VariableAttributes>,
                >;
                let mut uniform: bool;
                (scalar_attributes, uniform) = VariableAttributes::scalarize(binfo.attributes.clone(), length)?;
                if uniform {
                    assign_field!(binfo.attributes = (scalar_attributes).head().cloned()?);
                }
                if (uniform) {
                    List::fill(binfo, length)
                } else {
                    ({
                        let mut __acc: metamodelica::List<metamodelica::Ref<BackendInfo>> = metamodelica::nil();
                        for mut attr in (scalar_attributes).into_iter().cloned() {
                            let __x = metamodelica::Ref::new(BackendInfo {
                                varKind: binfo.varKind.clone(),
                                attributes: attr.clone(),
                                annotations: binfo.annotations.clone(),
                                var_pre: binfo.var_pre.clone(),
                                var_seed: binfo.var_seed.clone(),
                                var_pder_res: binfo.var_pder_res.clone(),
                                var_pder_tmp: binfo.var_pder_tmp.clone(),
                                var_start: binfo.var_start.clone(),
                                parent: binfo.parent.clone(),
                            });
                            __acc = cons(__x, __acc);
                        }
                        __acc.reverse()
                    })
                }
            }
        });
        Ok(binfo_list)
    }
}

thread_local! { static __DUMMY_BACKEND_INFO_TLS: metamodelica::Ref<BackendInfo::BackendInfo> = metamodelica::Ref::new(BackendInfo::BackendInfo { varKind: crate::NFBackendExtension::VariableKind::interned_FRONTEND_DUMMY(), attributes: EMPTY_VAR_ATTR_REAL().clone(), annotations: EMPTY_ANNOTATIONS.clone(), var_pre: None, var_seed: None, var_pder_res: None, var_pder_tmp: None, var_start: None, parent: None }); }
pub fn DUMMY_BACKEND_INFO() -> metamodelica::Ref<BackendInfo::BackendInfo> {
    __DUMMY_BACKEND_INFO_TLS.with(|__t| __t.clone())
}

pub mod VariableKind {
    use super::*;
    #[derive(Clone, Debug, Eq, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
    pub enum VariableKind {
        TIME,
        ALGEBRAIC,
        STATE {
            /// how often this states was differentiated
            index: i32,
            /// pointer to the derivative
            derivative: Option<PointerWeak::PointerWeak<metamodelica::Ref<Variable::NFVariable>>>,
            /// false if it was forced by StateSelect.always or StateSelect.prefer or generated by index reduction
            natural: bool,
        },
        STATE_DER {
            /// Original state
            state: PointerWeak::PointerWeak<metamodelica::Ref<Variable::NFVariable>>,
            /// Optional alias state expression. Result of differentiating the state if existant!
            alias: Option<Pointer::Pointer<metamodelica::Ref<Expression::NFExpression>>>,
        },
        DUMMY_DER {
            /// corresponding dummy state
            dummy_state: PointerWeak::PointerWeak<metamodelica::Ref<Variable::NFVariable>>,
        },
        DUMMY_STATE {
            /// corresponding dummy derivative
            dummy_der: PointerWeak::PointerWeak<metamodelica::Ref<Variable::NFVariable>>,
        },
        DISCRETE,
        DISCRETE_STATE,
        PREVIOUS,
        CLOCK,
        CLOCKED,
        PARAMETER {
            /// if the parameter is resizable, this is the computed optimal size
            resize_value: Option<i32>,
        },
        CONSTANT,
        ITERATOR,
        RECORD {
            children: metamodelica::List<PointerWeak::PointerWeak<metamodelica::Ref<Variable::NFVariable>>>,
            min_var: Variability,
            max_var: Variability,
        },
        START {
            /// Pointer to the corresponding original variable.
            original: PointerWeak::PointerWeak<metamodelica::Ref<Variable::NFVariable>>,
        },
        EXTOBJ {
            fullClassName: metamodelica::Ref<Absyn::Path>,
        },
        JAC_VAR,
        JAC_TMP_VAR,
        SEED_VAR,
        OPT_CONSTR,
        OPT_FCONSTR,
        OPT_INPUT_WITH_DER,
        OPT_INPUT_DER,
        OPT_TGRID,
        OPT_LOOP_INPUT {
            replaceCref: metamodelica::Ref<ComponentRef::NFComponentRef>,
        },
        /// algebraic state used by inline solver
        ALG_STATE,
        /// algebraic state old value used by inline solver
        ALG_STATE_OLD,
        RESIDUAL_VAR,
        /// auxiliary variable used for DAEmode
        DAE_AUX_VAR,
        /// used in SIMCODE, iteration variables in algebraic loops
        LOOP_ITERATION,
        /// used in SIMCODE, inner variables of a torn algebraic loop
        LOOP_SOLVED,
        /// Undefined variable type. Only to be used during frontend phase.
        FRONTEND_DUMMY,
    }
    impl metamodelica::gc::MMTrace for VariableKind {
        fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
            match self {
                VariableKind::TIME => Ok(()),
                VariableKind::ALGEBRAIC => Ok(()),
                VariableKind::STATE {
                    index,
                    derivative,
                    natural,
                } => {
                    metamodelica::gc::MMTrace::mm_accept(index, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(derivative, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(natural, __mmv)?;
                    Ok(())
                }
                VariableKind::STATE_DER { state, alias } => {
                    metamodelica::gc::MMTrace::mm_accept(state, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(alias, __mmv)?;
                    Ok(())
                }
                VariableKind::DUMMY_DER { dummy_state } => {
                    metamodelica::gc::MMTrace::mm_accept(dummy_state, __mmv)?;
                    Ok(())
                }
                VariableKind::DUMMY_STATE { dummy_der } => {
                    metamodelica::gc::MMTrace::mm_accept(dummy_der, __mmv)?;
                    Ok(())
                }
                VariableKind::DISCRETE => Ok(()),
                VariableKind::DISCRETE_STATE => Ok(()),
                VariableKind::PREVIOUS => Ok(()),
                VariableKind::CLOCK => Ok(()),
                VariableKind::CLOCKED => Ok(()),
                VariableKind::PARAMETER { resize_value } => {
                    metamodelica::gc::MMTrace::mm_accept(resize_value, __mmv)?;
                    Ok(())
                }
                VariableKind::CONSTANT => Ok(()),
                VariableKind::ITERATOR => Ok(()),
                VariableKind::RECORD {
                    children,
                    min_var,
                    max_var,
                } => {
                    metamodelica::gc::MMTrace::mm_accept(children, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(min_var, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(max_var, __mmv)?;
                    Ok(())
                }
                VariableKind::START { original } => {
                    metamodelica::gc::MMTrace::mm_accept(original, __mmv)?;
                    Ok(())
                }
                VariableKind::EXTOBJ { fullClassName } => {
                    metamodelica::gc::MMTrace::mm_accept(fullClassName, __mmv)?;
                    Ok(())
                }
                VariableKind::JAC_VAR => Ok(()),
                VariableKind::JAC_TMP_VAR => Ok(()),
                VariableKind::SEED_VAR => Ok(()),
                VariableKind::OPT_CONSTR => Ok(()),
                VariableKind::OPT_FCONSTR => Ok(()),
                VariableKind::OPT_INPUT_WITH_DER => Ok(()),
                VariableKind::OPT_INPUT_DER => Ok(()),
                VariableKind::OPT_TGRID => Ok(()),
                VariableKind::OPT_LOOP_INPUT { replaceCref } => {
                    metamodelica::gc::MMTrace::mm_accept(replaceCref, __mmv)?;
                    Ok(())
                }
                VariableKind::ALG_STATE => Ok(()),
                VariableKind::ALG_STATE_OLD => Ok(()),
                VariableKind::RESIDUAL_VAR => Ok(()),
                VariableKind::DAE_AUX_VAR => Ok(()),
                VariableKind::LOOP_ITERATION => Ok(()),
                VariableKind::LOOP_SOLVED => Ok(()),
                VariableKind::FRONTEND_DUMMY => Ok(()),
            }
        }
    }
    impl VariableKind {
        pub fn interned_TIME() -> metamodelica::Ref<VariableKind> {
            thread_local! {
                static INTERNED: metamodelica::Ref<VariableKind> = metamodelica::Ref::new(VariableKind::TIME);
            }
            INTERNED.with(|i| i.clone())
        }
        pub fn interned_ALGEBRAIC() -> metamodelica::Ref<VariableKind> {
            thread_local! {
                static INTERNED: metamodelica::Ref<VariableKind> = metamodelica::Ref::new(VariableKind::ALGEBRAIC);
            }
            INTERNED.with(|i| i.clone())
        }
        pub fn interned_DISCRETE() -> metamodelica::Ref<VariableKind> {
            thread_local! {
                static INTERNED: metamodelica::Ref<VariableKind> = metamodelica::Ref::new(VariableKind::DISCRETE);
            }
            INTERNED.with(|i| i.clone())
        }
        pub fn interned_DISCRETE_STATE() -> metamodelica::Ref<VariableKind> {
            thread_local! {
                static INTERNED: metamodelica::Ref<VariableKind> = metamodelica::Ref::new(VariableKind::DISCRETE_STATE);
            }
            INTERNED.with(|i| i.clone())
        }
        pub fn interned_PREVIOUS() -> metamodelica::Ref<VariableKind> {
            thread_local! {
                static INTERNED: metamodelica::Ref<VariableKind> = metamodelica::Ref::new(VariableKind::PREVIOUS);
            }
            INTERNED.with(|i| i.clone())
        }
        pub fn interned_CLOCK() -> metamodelica::Ref<VariableKind> {
            thread_local! {
                static INTERNED: metamodelica::Ref<VariableKind> = metamodelica::Ref::new(VariableKind::CLOCK);
            }
            INTERNED.with(|i| i.clone())
        }
        pub fn interned_CLOCKED() -> metamodelica::Ref<VariableKind> {
            thread_local! {
                static INTERNED: metamodelica::Ref<VariableKind> = metamodelica::Ref::new(VariableKind::CLOCKED);
            }
            INTERNED.with(|i| i.clone())
        }
        pub fn interned_CONSTANT() -> metamodelica::Ref<VariableKind> {
            thread_local! {
                static INTERNED: metamodelica::Ref<VariableKind> = metamodelica::Ref::new(VariableKind::CONSTANT);
            }
            INTERNED.with(|i| i.clone())
        }
        pub fn interned_ITERATOR() -> metamodelica::Ref<VariableKind> {
            thread_local! {
                static INTERNED: metamodelica::Ref<VariableKind> = metamodelica::Ref::new(VariableKind::ITERATOR);
            }
            INTERNED.with(|i| i.clone())
        }
        pub fn interned_JAC_VAR() -> metamodelica::Ref<VariableKind> {
            thread_local! {
                static INTERNED: metamodelica::Ref<VariableKind> = metamodelica::Ref::new(VariableKind::JAC_VAR);
            }
            INTERNED.with(|i| i.clone())
        }
        pub fn interned_JAC_TMP_VAR() -> metamodelica::Ref<VariableKind> {
            thread_local! {
                static INTERNED: metamodelica::Ref<VariableKind> = metamodelica::Ref::new(VariableKind::JAC_TMP_VAR);
            }
            INTERNED.with(|i| i.clone())
        }
        pub fn interned_SEED_VAR() -> metamodelica::Ref<VariableKind> {
            thread_local! {
                static INTERNED: metamodelica::Ref<VariableKind> = metamodelica::Ref::new(VariableKind::SEED_VAR);
            }
            INTERNED.with(|i| i.clone())
        }
        pub fn interned_OPT_CONSTR() -> metamodelica::Ref<VariableKind> {
            thread_local! {
                static INTERNED: metamodelica::Ref<VariableKind> = metamodelica::Ref::new(VariableKind::OPT_CONSTR);
            }
            INTERNED.with(|i| i.clone())
        }
        pub fn interned_OPT_FCONSTR() -> metamodelica::Ref<VariableKind> {
            thread_local! {
                static INTERNED: metamodelica::Ref<VariableKind> = metamodelica::Ref::new(VariableKind::OPT_FCONSTR);
            }
            INTERNED.with(|i| i.clone())
        }
        pub fn interned_OPT_INPUT_WITH_DER() -> metamodelica::Ref<VariableKind> {
            thread_local! {
                static INTERNED: metamodelica::Ref<VariableKind> = metamodelica::Ref::new(VariableKind::OPT_INPUT_WITH_DER);
            }
            INTERNED.with(|i| i.clone())
        }
        pub fn interned_OPT_INPUT_DER() -> metamodelica::Ref<VariableKind> {
            thread_local! {
                static INTERNED: metamodelica::Ref<VariableKind> = metamodelica::Ref::new(VariableKind::OPT_INPUT_DER);
            }
            INTERNED.with(|i| i.clone())
        }
        pub fn interned_OPT_TGRID() -> metamodelica::Ref<VariableKind> {
            thread_local! {
                static INTERNED: metamodelica::Ref<VariableKind> = metamodelica::Ref::new(VariableKind::OPT_TGRID);
            }
            INTERNED.with(|i| i.clone())
        }
        pub fn interned_ALG_STATE() -> metamodelica::Ref<VariableKind> {
            thread_local! {
                static INTERNED: metamodelica::Ref<VariableKind> = metamodelica::Ref::new(VariableKind::ALG_STATE);
            }
            INTERNED.with(|i| i.clone())
        }
        pub fn interned_ALG_STATE_OLD() -> metamodelica::Ref<VariableKind> {
            thread_local! {
                static INTERNED: metamodelica::Ref<VariableKind> = metamodelica::Ref::new(VariableKind::ALG_STATE_OLD);
            }
            INTERNED.with(|i| i.clone())
        }
        pub fn interned_RESIDUAL_VAR() -> metamodelica::Ref<VariableKind> {
            thread_local! {
                static INTERNED: metamodelica::Ref<VariableKind> = metamodelica::Ref::new(VariableKind::RESIDUAL_VAR);
            }
            INTERNED.with(|i| i.clone())
        }
        pub fn interned_DAE_AUX_VAR() -> metamodelica::Ref<VariableKind> {
            thread_local! {
                static INTERNED: metamodelica::Ref<VariableKind> = metamodelica::Ref::new(VariableKind::DAE_AUX_VAR);
            }
            INTERNED.with(|i| i.clone())
        }
        pub fn interned_LOOP_ITERATION() -> metamodelica::Ref<VariableKind> {
            thread_local! {
                static INTERNED: metamodelica::Ref<VariableKind> = metamodelica::Ref::new(VariableKind::LOOP_ITERATION);
            }
            INTERNED.with(|i| i.clone())
        }
        pub fn interned_LOOP_SOLVED() -> metamodelica::Ref<VariableKind> {
            thread_local! {
                static INTERNED: metamodelica::Ref<VariableKind> = metamodelica::Ref::new(VariableKind::LOOP_SOLVED);
            }
            INTERNED.with(|i| i.clone())
        }
        pub fn interned_FRONTEND_DUMMY() -> metamodelica::Ref<VariableKind> {
            thread_local! {
                static INTERNED: metamodelica::Ref<VariableKind> = metamodelica::Ref::new(VariableKind::FRONTEND_DUMMY);
            }
            INTERNED.with(|i| i.clone())
        }
    }
    pub fn interned_TIME() -> metamodelica::Ref<VariableKind> {
        VariableKind::interned_TIME()
    }
    pub fn interned_ALGEBRAIC() -> metamodelica::Ref<VariableKind> {
        VariableKind::interned_ALGEBRAIC()
    }
    pub fn interned_DISCRETE() -> metamodelica::Ref<VariableKind> {
        VariableKind::interned_DISCRETE()
    }
    pub fn interned_DISCRETE_STATE() -> metamodelica::Ref<VariableKind> {
        VariableKind::interned_DISCRETE_STATE()
    }
    pub fn interned_PREVIOUS() -> metamodelica::Ref<VariableKind> {
        VariableKind::interned_PREVIOUS()
    }
    pub fn interned_CLOCK() -> metamodelica::Ref<VariableKind> {
        VariableKind::interned_CLOCK()
    }
    pub fn interned_CLOCKED() -> metamodelica::Ref<VariableKind> {
        VariableKind::interned_CLOCKED()
    }
    pub fn interned_CONSTANT() -> metamodelica::Ref<VariableKind> {
        VariableKind::interned_CONSTANT()
    }
    pub fn interned_ITERATOR() -> metamodelica::Ref<VariableKind> {
        VariableKind::interned_ITERATOR()
    }
    pub fn interned_JAC_VAR() -> metamodelica::Ref<VariableKind> {
        VariableKind::interned_JAC_VAR()
    }
    pub fn interned_JAC_TMP_VAR() -> metamodelica::Ref<VariableKind> {
        VariableKind::interned_JAC_TMP_VAR()
    }
    pub fn interned_SEED_VAR() -> metamodelica::Ref<VariableKind> {
        VariableKind::interned_SEED_VAR()
    }
    pub fn interned_OPT_CONSTR() -> metamodelica::Ref<VariableKind> {
        VariableKind::interned_OPT_CONSTR()
    }
    pub fn interned_OPT_FCONSTR() -> metamodelica::Ref<VariableKind> {
        VariableKind::interned_OPT_FCONSTR()
    }
    pub fn interned_OPT_INPUT_WITH_DER() -> metamodelica::Ref<VariableKind> {
        VariableKind::interned_OPT_INPUT_WITH_DER()
    }
    pub fn interned_OPT_INPUT_DER() -> metamodelica::Ref<VariableKind> {
        VariableKind::interned_OPT_INPUT_DER()
    }
    pub fn interned_OPT_TGRID() -> metamodelica::Ref<VariableKind> {
        VariableKind::interned_OPT_TGRID()
    }
    pub fn interned_ALG_STATE() -> metamodelica::Ref<VariableKind> {
        VariableKind::interned_ALG_STATE()
    }
    pub fn interned_ALG_STATE_OLD() -> metamodelica::Ref<VariableKind> {
        VariableKind::interned_ALG_STATE_OLD()
    }
    pub fn interned_RESIDUAL_VAR() -> metamodelica::Ref<VariableKind> {
        VariableKind::interned_RESIDUAL_VAR()
    }
    pub fn interned_DAE_AUX_VAR() -> metamodelica::Ref<VariableKind> {
        VariableKind::interned_DAE_AUX_VAR()
    }
    pub fn interned_LOOP_ITERATION() -> metamodelica::Ref<VariableKind> {
        VariableKind::interned_LOOP_ITERATION()
    }
    pub fn interned_LOOP_SOLVED() -> metamodelica::Ref<VariableKind> {
        VariableKind::interned_LOOP_SOLVED()
    }
    pub fn interned_FRONTEND_DUMMY() -> metamodelica::Ref<VariableKind> {
        VariableKind::interned_FRONTEND_DUMMY()
    }
    impl Default for VariableKind {
        fn default() -> Self {
            Self::TIME
        }
    }
    pub use self::VariableKind::{
        ALG_STATE, ALG_STATE_OLD, ALGEBRAIC, CLOCK, CLOCKED, CONSTANT, DAE_AUX_VAR, DISCRETE, DISCRETE_STATE,
        DUMMY_DER, DUMMY_STATE, EXTOBJ, FRONTEND_DUMMY, ITERATOR, JAC_TMP_VAR, JAC_VAR, LOOP_ITERATION, LOOP_SOLVED,
        OPT_CONSTR, OPT_FCONSTR, OPT_INPUT_DER, OPT_INPUT_WITH_DER, OPT_LOOP_INPUT, OPT_TGRID, PARAMETER, PREVIOUS,
        RECORD, RESIDUAL_VAR, SEED_VAR, START, STATE, STATE_DER, TIME,
    };
    pub fn toString(mut varKind: &metamodelica::Ref<VariableKind>) -> ArcStr {
        let mut r#str: ArcStr;
        r#str = (match &**varKind {
            TIME { .. } => literal!("[TIME]"),
            ALGEBRAIC { .. } => literal!("[ALGB]"),
            STATE { .. } => literal!("[STAT]"),
            STATE_DER { .. } => literal!("[DER-]"),
            DUMMY_DER { .. } => literal!("[DDER]"),
            DUMMY_STATE { .. } => literal!("[DSTA]"),
            DISCRETE { .. } => literal!("[DISC]"),
            DISCRETE_STATE { .. } => literal!("[DISS]"),
            PREVIOUS { .. } => literal!("[PRE-]"),
            CLOCK { .. } => literal!("[CLCK]"),
            CLOCKED { .. } => literal!("[CLKD]"),
            PARAMETER { .. } => literal!("[PRMT]"),
            CONSTANT { .. } => literal!("[CNST]"),
            ITERATOR { .. } => literal!("[ITER]"),
            RECORD { .. } => literal!("[RECD]"),
            START { .. } => literal!("[STRT]"),
            EXTOBJ { .. } => literal!("[EXTO]"),
            JAC_VAR { .. } => literal!("[JVAR]"),
            JAC_TMP_VAR { .. } => literal!("[JTMP]"),
            SEED_VAR { .. } => literal!("[SEED]"),
            OPT_CONSTR { .. } => literal!("[OPT][CONS]"),
            OPT_FCONSTR { .. } => literal!("[OPT][FCON]"),
            OPT_INPUT_WITH_DER { .. } => literal!("[OPT][INWD]"),
            OPT_INPUT_DER { .. } => literal!("[OPT][INPD]"),
            OPT_TGRID { .. } => literal!("[OPT][TGRD]"),
            OPT_LOOP_INPUT { .. } => literal!("[OPT][LOOP]"),
            ALG_STATE { .. } => literal!("[ASTA]"),
            RESIDUAL_VAR { .. } => literal!("[RES-]"),
            DAE_AUX_VAR { .. } => literal!("[AUX-]"),
            LOOP_ITERATION { .. } => literal!("[LOOP]"),
            LOOP_SOLVED { .. } => literal!("[INNR]"),
            FRONTEND_DUMMY { .. } => literal!("[DMMY]"),
            _ => {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("[FAIL] "));
                __mm_s.push_str(&*literal!("NFBackendExtension.VariableKind.toString"));
                __mm_s.push_str(&*literal!(" failed."));
                ArcStr::from(__mm_s)
            }
        });
        r#str
    }

    pub fn isTimeDependent(mut varKind: &metamodelica::Ref<VariableKind>) -> bool {
        let mut b: bool;
        b = (match &**varKind {
            PARAMETER { .. } => false,
            CONSTANT { .. } => false,
            ITERATOR { .. } => false,
            START { .. } => false,
            _ => true,
        });
        b
    }

    pub fn fromType(
        mut ty: metamodelica::Ref<Type::NFType>,
        mut makeParam: bool,
    ) -> Result<metamodelica::Ref<VariableKind>> {
        let mut varKind: metamodelica::Ref<VariableKind>;
        let mut variability: Variability;
        if Type::isRecord(&(Type::arrayElementType(&ty))) {
            variability = if (makeParam) {
                Variability::PARAMETER.clone()
            } else {
                Variability::CONTINUOUS.clone()
            };
            varKind = metamodelica::Ref::new(VariableKind::RECORD {
                children: metamodelica::nil(),
                min_var: variability,
                max_var: variability,
            });
        } else if makeParam {
            varKind = metamodelica::Ref::new(VariableKind::PARAMETER { resize_value: None });
        } else if Type::isDiscrete(ty)? {
            varKind = crate::NFBackendExtension::VariableKind::interned_DISCRETE();
        } else {
            varKind = crate::NFBackendExtension::VariableKind::interned_ALGEBRAIC();
        }
        Ok(varKind)
    }
}

pub mod VariableAttributes {
    use super::*;
    #[derive(Clone, Debug, Eq, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
    pub enum VariableAttributes {
        VAR_ATTR_REAL {
            /// quantity
            quantity: Option<metamodelica::Ref<Binding::NFBinding>>,
            /// SI Unit for actual computation value
            unit: Option<metamodelica::Ref<Binding::NFBinding>>,
            /// SI Unit only for displaying
            displayUnit: Option<metamodelica::Ref<Binding::NFBinding>>,
            /// Lower boundry
            min: Option<metamodelica::Ref<Binding::NFBinding>>,
            /// Upper boundry
            max: Option<metamodelica::Ref<Binding::NFBinding>>,
            /// start value
            start: Option<metamodelica::Ref<Binding::NFBinding>>,
            /// fixed - true: default for parameter/constant, false - default for other variables
            fixed: Option<metamodelica::Ref<Binding::NFBinding>>,
            /// nominal
            nominal: Option<metamodelica::Ref<Binding::NFBinding>>,
            /// Priority to be selected as a state during index reduction
            stateSelect: Option<StateSelect>,
            /// Priority to be selected as an iteration variable during tearing
            tearingSelect: Option<TearingSelect>,
            /// Attributes from data reconcilliation
            uncertainty: Option<Uncertainty>,
            /// ToDo: ???
            distribution: Option<Distribution>,
            /// A binding expression for certain types. E.G. parameters
            binding: Option<metamodelica::Ref<Binding::NFBinding>>,
            /// Defined in protected scope
            isProtected: Option<bool>,
            /// Defined as final
            finalPrefix: Option<bool>,
        },
        VAR_ATTR_INT {
            /// quantity
            quantity: Option<metamodelica::Ref<Binding::NFBinding>>,
            /// Lower boundry
            min: Option<metamodelica::Ref<Binding::NFBinding>>,
            /// Upper boundry
            max: Option<metamodelica::Ref<Binding::NFBinding>>,
            /// start value
            start: Option<metamodelica::Ref<Binding::NFBinding>>,
            /// fixed - true: default for parameter/constant, false - default for other variables
            fixed: Option<metamodelica::Ref<Binding::NFBinding>>,
            /// Attributes from data reconcilliation
            uncertainty: Option<Uncertainty>,
            /// ToDo: ???
            distribution: Option<Distribution>,
            /// A binding expression for certain types. E.G. parameters
            binding: Option<metamodelica::Ref<Binding::NFBinding>>,
            /// Defined in protected scope
            isProtected: Option<bool>,
            /// Defined as final
            finalPrefix: Option<bool>,
        },
        VAR_ATTR_BOOL {
            /// quantity
            quantity: Option<metamodelica::Ref<Binding::NFBinding>>,
            /// start value
            start: Option<metamodelica::Ref<Binding::NFBinding>>,
            /// fixed - true: default for parameter/constant, false - default for other variables
            fixed: Option<metamodelica::Ref<Binding::NFBinding>>,
            /// A binding expression for certain types. E.G. parameters
            binding: Option<metamodelica::Ref<Binding::NFBinding>>,
            /// Defined in protected scope
            isProtected: Option<bool>,
            /// Defined as final
            finalPrefix: Option<bool>,
        },
        VAR_ATTR_CLOCK {
            /// Defined in protected scope
            isProtected: Option<bool>,
            /// Defined as final
            finalPrefix: Option<bool>,
        },
        /// kabdelhak: why does string have quantity/start/fixed?
        VAR_ATTR_STRING {
            /// quantity
            quantity: Option<metamodelica::Ref<Binding::NFBinding>>,
            /// start value
            start: Option<metamodelica::Ref<Binding::NFBinding>>,
            /// fixed - true: default for parameter/constant, false - default for other variables
            fixed: Option<metamodelica::Ref<Binding::NFBinding>>,
            /// A binding expression for certain types. E.G. parameters
            binding: Option<metamodelica::Ref<Binding::NFBinding>>,
            /// Defined in protected scope
            isProtected: Option<bool>,
            /// Defined as final
            finalPrefix: Option<bool>,
        },
        VAR_ATTR_ENUMERATION {
            /// quantity
            quantity: Option<metamodelica::Ref<Binding::NFBinding>>,
            /// Lower boundry
            min: Option<metamodelica::Ref<Binding::NFBinding>>,
            /// Upper boundry
            max: Option<metamodelica::Ref<Binding::NFBinding>>,
            /// start value
            start: Option<metamodelica::Ref<Binding::NFBinding>>,
            /// fixed - true: default for parameter/constant, false - default for other variables
            fixed: Option<metamodelica::Ref<Binding::NFBinding>>,
            /// A binding expression for certain types. E.G. parameters
            binding: Option<metamodelica::Ref<Binding::NFBinding>>,
            /// Defined in protected scope
            isProtected: Option<bool>,
            /// Defined as final
            finalPrefix: Option<bool>,
        },
        VAR_ATTR_RECORD {
            indexMap: metamodelica::Ref<UnorderedMap::UnorderedMap<ArcStr, i32>>,
            childrenAttr: metamodelica::Array<metamodelica::Ref<VariableAttributes>>,
        },
    }
    impl metamodelica::gc::MMTrace for VariableAttributes {
        fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
            match self {
                VariableAttributes::VAR_ATTR_REAL {
                    quantity,
                    unit,
                    displayUnit,
                    min,
                    max,
                    start,
                    fixed,
                    nominal,
                    stateSelect,
                    tearingSelect,
                    uncertainty,
                    distribution,
                    binding,
                    isProtected,
                    finalPrefix,
                } => {
                    metamodelica::gc::MMTrace::mm_accept(quantity, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(unit, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(displayUnit, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(min, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(max, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(start, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(fixed, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(nominal, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(stateSelect, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(tearingSelect, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(uncertainty, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(distribution, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(binding, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(isProtected, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(finalPrefix, __mmv)?;
                    Ok(())
                }
                VariableAttributes::VAR_ATTR_INT {
                    quantity,
                    min,
                    max,
                    start,
                    fixed,
                    uncertainty,
                    distribution,
                    binding,
                    isProtected,
                    finalPrefix,
                } => {
                    metamodelica::gc::MMTrace::mm_accept(quantity, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(min, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(max, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(start, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(fixed, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(uncertainty, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(distribution, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(binding, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(isProtected, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(finalPrefix, __mmv)?;
                    Ok(())
                }
                VariableAttributes::VAR_ATTR_BOOL {
                    quantity,
                    start,
                    fixed,
                    binding,
                    isProtected,
                    finalPrefix,
                } => {
                    metamodelica::gc::MMTrace::mm_accept(quantity, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(start, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(fixed, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(binding, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(isProtected, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(finalPrefix, __mmv)?;
                    Ok(())
                }
                VariableAttributes::VAR_ATTR_CLOCK {
                    isProtected,
                    finalPrefix,
                } => {
                    metamodelica::gc::MMTrace::mm_accept(isProtected, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(finalPrefix, __mmv)?;
                    Ok(())
                }
                VariableAttributes::VAR_ATTR_STRING {
                    quantity,
                    start,
                    fixed,
                    binding,
                    isProtected,
                    finalPrefix,
                } => {
                    metamodelica::gc::MMTrace::mm_accept(quantity, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(start, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(fixed, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(binding, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(isProtected, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(finalPrefix, __mmv)?;
                    Ok(())
                }
                VariableAttributes::VAR_ATTR_ENUMERATION {
                    quantity,
                    min,
                    max,
                    start,
                    fixed,
                    binding,
                    isProtected,
                    finalPrefix,
                } => {
                    metamodelica::gc::MMTrace::mm_accept(quantity, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(min, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(max, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(start, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(fixed, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(binding, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(isProtected, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(finalPrefix, __mmv)?;
                    Ok(())
                }
                VariableAttributes::VAR_ATTR_RECORD { indexMap, childrenAttr } => {
                    metamodelica::gc::MMTrace::mm_accept(indexMap, __mmv)?;
                    metamodelica::gc::MMTrace::mm_accept(childrenAttr, __mmv)?;
                    Ok(())
                }
            }
        }
    }
    impl Default for VariableAttributes {
        fn default() -> Self {
            Self::VAR_ATTR_CLOCK {
                isProtected: Default::default(),
                finalPrefix: Default::default(),
            }
        }
    }
    pub use self::VariableAttributes::{
        VAR_ATTR_BOOL, VAR_ATTR_CLOCK, VAR_ATTR_ENUMERATION, VAR_ATTR_INT, VAR_ATTR_REAL, VAR_ATTR_RECORD,
        VAR_ATTR_STRING,
    };
    #[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, metamodelica::ReferenceEq)]
    #[repr(i32)]
    pub(crate) enum VarType {
        ENUMERATION = 1,
        CLOCK = 2,
        STRING = 3,
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

    pub fn toString(mut attr: &metamodelica::Ref<VariableAttributes>) -> Result<ArcStr> {
        let mut r#str: ArcStr = literal!("");
        r#str = (match &**attr {
            VAR_ATTR_REAL {
                fixed: __attr_fixed,
                max: __attr_max,
                min: __attr_min,
                nominal: __attr_nominal,
                start: __attr_start,
                stateSelect: __attr_stateSelect,
                tearingSelect: __attr_tearingSelect,
                ..
            } => attributesToString(
                &(list![
                    (
                        literal!("fixed"),
                        Util::applyOption(__attr_fixed.clone(), &move |__a0: metamodelica::Ref<
                            Binding::NFBinding,
                        >| Binding::getTypedExp(
                            &__a0
                        ))?
                    ),
                    (
                        literal!("start"),
                        Util::applyOption(__attr_start.clone(), &move |__a0: metamodelica::Ref<
                            Binding::NFBinding,
                        >| Binding::getTypedExp(
                            &__a0
                        ))?
                    ),
                    (
                        literal!("min"),
                        Util::applyOption(__attr_min.clone(), &move |__a0: metamodelica::Ref<
                            Binding::NFBinding,
                        >| Binding::getTypedExp(
                            &__a0
                        ))?
                    ),
                    (
                        literal!("max"),
                        Util::applyOption(__attr_max.clone(), &move |__a0: metamodelica::Ref<
                            Binding::NFBinding,
                        >| Binding::getTypedExp(
                            &__a0
                        ))?
                    ),
                    (
                        literal!("nominal"),
                        Util::applyOption(__attr_nominal.clone(), &move |__a0: metamodelica::Ref<
                            Binding::NFBinding,
                        >| Binding::getTypedExp(
                            &__a0
                        ))?
                    )
                ]),
                __attr_stateSelect.clone(),
                __attr_tearingSelect.clone(),
            )?,
            VAR_ATTR_INT {
                fixed: __attr_fixed,
                max: __attr_max,
                min: __attr_min,
                start: __attr_start,
                ..
            } => attributesToString(
                &(list![
                    (
                        literal!("fixed"),
                        Util::applyOption(__attr_fixed.clone(), &move |__a0: metamodelica::Ref<
                            Binding::NFBinding,
                        >| Binding::getTypedExp(
                            &__a0
                        ))?
                    ),
                    (
                        literal!("start"),
                        Util::applyOption(__attr_start.clone(), &move |__a0: metamodelica::Ref<
                            Binding::NFBinding,
                        >| Binding::getTypedExp(
                            &__a0
                        ))?
                    ),
                    (
                        literal!("min"),
                        Util::applyOption(__attr_min.clone(), &move |__a0: metamodelica::Ref<
                            Binding::NFBinding,
                        >| Binding::getTypedExp(
                            &__a0
                        ))?
                    ),
                    (
                        literal!("max"),
                        Util::applyOption(__attr_max.clone(), &move |__a0: metamodelica::Ref<
                            Binding::NFBinding,
                        >| Binding::getTypedExp(
                            &__a0
                        ))?
                    )
                ]),
                None,
                None,
            )?,
            VAR_ATTR_BOOL {
                fixed: __attr_fixed,
                start: __attr_start,
                ..
            } => attributesToString(
                &(list![
                    (
                        literal!("fixed"),
                        Util::applyOption(__attr_fixed.clone(), &move |__a0: metamodelica::Ref<
                            Binding::NFBinding,
                        >| Binding::getTypedExp(
                            &__a0
                        ))?
                    ),
                    (
                        literal!("start"),
                        Util::applyOption(__attr_start.clone(), &move |__a0: metamodelica::Ref<
                            Binding::NFBinding,
                        >| Binding::getTypedExp(
                            &__a0
                        ))?
                    )
                ]),
                None,
                None,
            )?,
            VAR_ATTR_CLOCK { .. } => literal!(""),
            VAR_ATTR_STRING {
                fixed: __attr_fixed,
                start: __attr_start,
                ..
            } => attributesToString(
                &(list![
                    (
                        literal!("fixed"),
                        Util::applyOption(__attr_fixed.clone(), &move |__a0: metamodelica::Ref<
                            Binding::NFBinding,
                        >| Binding::getTypedExp(
                            &__a0
                        ))?
                    ),
                    (
                        literal!("start"),
                        Util::applyOption(__attr_start.clone(), &move |__a0: metamodelica::Ref<
                            Binding::NFBinding,
                        >| Binding::getTypedExp(
                            &__a0
                        ))?
                    )
                ]),
                None,
                None,
            )?,
            VAR_ATTR_ENUMERATION {
                fixed: __attr_fixed,
                max: __attr_max,
                min: __attr_min,
                start: __attr_start,
                ..
            } => attributesToString(
                &(list![
                    (
                        literal!("fixed"),
                        Util::applyOption(__attr_fixed.clone(), &move |__a0: metamodelica::Ref<
                            Binding::NFBinding,
                        >| Binding::getTypedExp(
                            &__a0
                        ))?
                    ),
                    (
                        literal!("start"),
                        Util::applyOption(__attr_start.clone(), &move |__a0: metamodelica::Ref<
                            Binding::NFBinding,
                        >| Binding::getTypedExp(
                            &__a0
                        ))?
                    ),
                    (
                        literal!("min"),
                        Util::applyOption(__attr_min.clone(), &move |__a0: metamodelica::Ref<
                            Binding::NFBinding,
                        >| Binding::getTypedExp(
                            &__a0
                        ))?
                    ),
                    (
                        literal!("max"),
                        Util::applyOption(__attr_max.clone(), &move |__a0: metamodelica::Ref<
                            Binding::NFBinding,
                        >| Binding::getTypedExp(
                            &__a0
                        ))?
                    )
                ]),
                None,
                None,
            )?,
            VAR_ATTR_RECORD {
                indexMap: __attr_indexMap,
                ..
            } => List::toString(
                UnorderedMap::toList(__attr_indexMap.clone()),
                &({
                    let __pe_b1 = var_field!((**attr).childrenAttr, VariableAttributes::VAR_ATTR_RECORD).clone();
                    move |__pe_a0| recordString(&__pe_a0, __pe_b1.clone())
                }),
                List::Style::FLAT.clone(),
            )?,
            _ => {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("NFBackendExtension.VariableAttributes.toString"));
                __mm_s.push_str(&*literal!(" failed. Attribute string could not be created."));
                ArcStr::from(__mm_s)
            }
        });
        r#str = if (metamodelica::stringEq(&(literal!("")), &r#str)) {
            literal!("")
        } else {
            {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("("));
                __mm_s.push_str(&*r#str);
                __mm_s.push_str(&*literal!(")"));
                ArcStr::from(__mm_s)
            }
        };
        Ok(r#str)
    }

    pub(crate) fn recordString(
        mut attr_tpl: &(ArcStr, i32),
        mut childrenAttr: metamodelica::Array<metamodelica::Ref<VariableAttributes>>,
    ) -> Result<ArcStr> {
        let __ab_childrenAttr = childrenAttr.borrow();
        let mut r#str: ArcStr;
        let mut name: ArcStr;
        let mut index: i32;
        (name, index) = attr_tpl.clone();
        r#str = {
            let mut __mm_s = String::new();
            __mm_s.push_str(&*name);
            __mm_s.push_str(&*toString(&(*metamodelica::index_checked(&__ab_childrenAttr, index)?))?);
            ArcStr::from(__mm_s)
        };
        Ok(r#str)
    }

    pub fn create(
        mut attrs: &metamodelica::List<(ArcStr, metamodelica::Ref<Binding::NFBinding>)>,
        mut ty: &metamodelica::Ref<Type::NFType>,
        mut compAttrs: &metamodelica::Ref<Attributes::NFAttributes>,
        mut children: &metamodelica::List<metamodelica::Ref<Variable::NFVariable>>,
        mut comment: &metamodelica::Ref<SCode::Comment>,
    ) -> Result<metamodelica::Ref<VariableAttributes>> {
        let mut attributes: metamodelica::Ref<VariableAttributes>;
        let mut is_final: bool;
        let mut complexTy: metamodelica::Ref<ComplexType::NFComplexType>;
        is_final =
            compAttrs.isFinal.clone() || compAttrs.variability.clone() == Variability::STRUCTURAL_PARAMETER.clone();
        attributes = (::match_deref::match_deref! { match &(Type::arrayElementType(ty)) {
            Deref @ Type::REAL => createReal(attrs, is_final, comment)?,
            Deref @ Type::INTEGER => createInt(attrs, is_final)?,
            Deref @ Type::BOOLEAN => createBool(attrs, is_final)?,
            Deref @ Type::STRING => createString(attrs, is_final)?,
            Deref @ Type::ENUMERATION { .. } => createEnum(attrs, is_final)?,
            Deref @ Type::CLOCK => createClock(is_final),
            Deref @ Type::COMPLEX { complexTy: __esc_complexTy @ Deref @ ComplexType::RECORD { .. }, .. } => {
                complexTy = (*__esc_complexTy).clone();
                createRecord(attrs, var_field!((*complexTy).indexMap, ComplexType::NFComplexType::RECORD).clone(), children, is_final)?
            },
            _ => createReal(attrs, is_final, comment)?,
            _ => unreachable!("match_deref! exhaustiveness placeholder"),
        } });
        Ok(attributes)
    }

    pub(crate) fn map(
        mut attributes: metamodelica::Ref<VariableAttributes>,
        mut func: Arc<
            dyn ::std::ops::Fn(
                    metamodelica::Ref<Expression::NFExpression>,
                ) -> Result<metamodelica::Ref<Expression::NFExpression>>
                + 'static,
        >,
    ) -> Result<metamodelica::Ref<VariableAttributes>> {
        pub type expFunc = std::sync::Arc<
            dyn ::std::ops::Fn(
                    metamodelica::Ref<Expression::NFExpression>,
                ) -> Result<metamodelica::Ref<Expression::NFExpression>>
                + 'static,
        >;

        let mut attributes: metamodelica::Ref<VariableAttributes> = attributes;
        attributes = (match &*attributes {
            VAR_ATTR_REAL {
                quantity: __attributes_quantity,
                ..
            } => {
                assign_variant_field!(attributes => VariableAttributes::VAR_ATTR_REAL;
                    quantity = Util::applyOption(__attributes_quantity.clone(), &({ let __pe_b1: Arc<dyn ::std::ops::Fn(metamodelica::Ref<Expression::NFExpression>) -> Result<metamodelica::Ref<Expression::NFExpression>> + 'static> = func.clone(); move |__pe_a0| Binding::mapExp(__pe_a0, __pe_b1.clone()) }))?,
                    unit = Util::applyOption(var_field!((*attributes).unit, VariableAttributes::VAR_ATTR_REAL).clone(), &({ let __pe_b1: Arc<dyn ::std::ops::Fn(metamodelica::Ref<Expression::NFExpression>) -> Result<metamodelica::Ref<Expression::NFExpression>> + 'static> = func.clone(); move |__pe_a0| Binding::mapExp(__pe_a0, __pe_b1.clone()) }))?,
                    displayUnit = Util::applyOption(var_field!((*attributes).displayUnit, VariableAttributes::VAR_ATTR_REAL).clone(), &({ let __pe_b1: Arc<dyn ::std::ops::Fn(metamodelica::Ref<Expression::NFExpression>) -> Result<metamodelica::Ref<Expression::NFExpression>> + 'static> = func.clone(); move |__pe_a0| Binding::mapExp(__pe_a0, __pe_b1.clone()) }))?,
                    min = Util::applyOption(var_field!((*attributes).min, VariableAttributes::VAR_ATTR_REAL).clone(), &({ let __pe_b1: Arc<dyn ::std::ops::Fn(metamodelica::Ref<Expression::NFExpression>) -> Result<metamodelica::Ref<Expression::NFExpression>> + 'static> = func.clone(); move |__pe_a0| Binding::mapExp(__pe_a0, __pe_b1.clone()) }))?,
                    max = Util::applyOption(var_field!((*attributes).max, VariableAttributes::VAR_ATTR_REAL).clone(), &({ let __pe_b1: Arc<dyn ::std::ops::Fn(metamodelica::Ref<Expression::NFExpression>) -> Result<metamodelica::Ref<Expression::NFExpression>> + 'static> = func.clone(); move |__pe_a0| Binding::mapExp(__pe_a0, __pe_b1.clone()) }))?,
                    start = Util::applyOption(var_field!((*attributes).start, VariableAttributes::VAR_ATTR_REAL).clone(), &({ let __pe_b1: Arc<dyn ::std::ops::Fn(metamodelica::Ref<Expression::NFExpression>) -> Result<metamodelica::Ref<Expression::NFExpression>> + 'static> = func.clone(); move |__pe_a0| Binding::mapExp(__pe_a0, __pe_b1.clone()) }))?,
                    fixed = Util::applyOption(var_field!((*attributes).fixed, VariableAttributes::VAR_ATTR_REAL).clone(), &({ let __pe_b1: Arc<dyn ::std::ops::Fn(metamodelica::Ref<Expression::NFExpression>) -> Result<metamodelica::Ref<Expression::NFExpression>> + 'static> = func.clone(); move |__pe_a0| Binding::mapExp(__pe_a0, __pe_b1.clone()) }))?,
                    nominal = Util::applyOption(var_field!((*attributes).nominal, VariableAttributes::VAR_ATTR_REAL).clone(), &({ let __pe_b1: Arc<dyn ::std::ops::Fn(metamodelica::Ref<Expression::NFExpression>) -> Result<metamodelica::Ref<Expression::NFExpression>> + 'static> = func.clone(); move |__pe_a0| Binding::mapExp(__pe_a0, __pe_b1.clone()) }))?,
                    binding = Util::applyOption(var_field!((*attributes).binding, VariableAttributes::VAR_ATTR_REAL).clone(), &({ let __pe_b1: Arc<dyn ::std::ops::Fn(metamodelica::Ref<Expression::NFExpression>) -> Result<metamodelica::Ref<Expression::NFExpression>> + 'static> = func.clone(); move |__pe_a0| Binding::mapExp(__pe_a0, __pe_b1.clone()) }))?
                );
                attributes
            }
            VAR_ATTR_INT {
                quantity: __attributes_quantity,
                ..
            } => {
                assign_variant_field!(attributes => VariableAttributes::VAR_ATTR_INT;
                    quantity = Util::applyOption(__attributes_quantity.clone(), &({ let __pe_b1: Arc<dyn ::std::ops::Fn(metamodelica::Ref<Expression::NFExpression>) -> Result<metamodelica::Ref<Expression::NFExpression>> + 'static> = func.clone(); move |__pe_a0| Binding::mapExp(__pe_a0, __pe_b1.clone()) }))?,
                    min = Util::applyOption(var_field!((*attributes).min, VariableAttributes::VAR_ATTR_INT).clone(), &({ let __pe_b1: Arc<dyn ::std::ops::Fn(metamodelica::Ref<Expression::NFExpression>) -> Result<metamodelica::Ref<Expression::NFExpression>> + 'static> = func.clone(); move |__pe_a0| Binding::mapExp(__pe_a0, __pe_b1.clone()) }))?,
                    max = Util::applyOption(var_field!((*attributes).max, VariableAttributes::VAR_ATTR_INT).clone(), &({ let __pe_b1: Arc<dyn ::std::ops::Fn(metamodelica::Ref<Expression::NFExpression>) -> Result<metamodelica::Ref<Expression::NFExpression>> + 'static> = func.clone(); move |__pe_a0| Binding::mapExp(__pe_a0, __pe_b1.clone()) }))?,
                    start = Util::applyOption(var_field!((*attributes).start, VariableAttributes::VAR_ATTR_INT).clone(), &({ let __pe_b1: Arc<dyn ::std::ops::Fn(metamodelica::Ref<Expression::NFExpression>) -> Result<metamodelica::Ref<Expression::NFExpression>> + 'static> = func.clone(); move |__pe_a0| Binding::mapExp(__pe_a0, __pe_b1.clone()) }))?,
                    fixed = Util::applyOption(var_field!((*attributes).fixed, VariableAttributes::VAR_ATTR_INT).clone(), &({ let __pe_b1: Arc<dyn ::std::ops::Fn(metamodelica::Ref<Expression::NFExpression>) -> Result<metamodelica::Ref<Expression::NFExpression>> + 'static> = func.clone(); move |__pe_a0| Binding::mapExp(__pe_a0, __pe_b1.clone()) }))?,
                    binding = Util::applyOption(var_field!((*attributes).binding, VariableAttributes::VAR_ATTR_INT).clone(), &({ let __pe_b1: Arc<dyn ::std::ops::Fn(metamodelica::Ref<Expression::NFExpression>) -> Result<metamodelica::Ref<Expression::NFExpression>> + 'static> = func.clone(); move |__pe_a0| Binding::mapExp(__pe_a0, __pe_b1.clone()) }))?
                );
                attributes
            }
            VAR_ATTR_BOOL {
                quantity: __attributes_quantity,
                ..
            } => {
                assign_variant_field!(attributes => VariableAttributes::VAR_ATTR_BOOL;
                    quantity = Util::applyOption(__attributes_quantity.clone(), &({ let __pe_b1: Arc<dyn ::std::ops::Fn(metamodelica::Ref<Expression::NFExpression>) -> Result<metamodelica::Ref<Expression::NFExpression>> + 'static> = func.clone(); move |__pe_a0| Binding::mapExp(__pe_a0, __pe_b1.clone()) }))?,
                    start = Util::applyOption(var_field!((*attributes).start, VariableAttributes::VAR_ATTR_BOOL).clone(), &({ let __pe_b1: Arc<dyn ::std::ops::Fn(metamodelica::Ref<Expression::NFExpression>) -> Result<metamodelica::Ref<Expression::NFExpression>> + 'static> = func.clone(); move |__pe_a0| Binding::mapExp(__pe_a0, __pe_b1.clone()) }))?,
                    fixed = Util::applyOption(var_field!((*attributes).fixed, VariableAttributes::VAR_ATTR_BOOL).clone(), &({ let __pe_b1: Arc<dyn ::std::ops::Fn(metamodelica::Ref<Expression::NFExpression>) -> Result<metamodelica::Ref<Expression::NFExpression>> + 'static> = func.clone(); move |__pe_a0| Binding::mapExp(__pe_a0, __pe_b1.clone()) }))?,
                    binding = Util::applyOption(var_field!((*attributes).binding, VariableAttributes::VAR_ATTR_BOOL).clone(), &({ let __pe_b1: Arc<dyn ::std::ops::Fn(metamodelica::Ref<Expression::NFExpression>) -> Result<metamodelica::Ref<Expression::NFExpression>> + 'static> = func.clone(); move |__pe_a0| Binding::mapExp(__pe_a0, __pe_b1.clone()) }))?
                );
                attributes
            }
            VAR_ATTR_STRING {
                quantity: __attributes_quantity,
                ..
            } => {
                assign_variant_field!(attributes => VariableAttributes::VAR_ATTR_STRING;
                    quantity = Util::applyOption(__attributes_quantity.clone(), &({ let __pe_b1: Arc<dyn ::std::ops::Fn(metamodelica::Ref<Expression::NFExpression>) -> Result<metamodelica::Ref<Expression::NFExpression>> + 'static> = func.clone(); move |__pe_a0| Binding::mapExp(__pe_a0, __pe_b1.clone()) }))?,
                    start = Util::applyOption(var_field!((*attributes).start, VariableAttributes::VAR_ATTR_STRING).clone(), &({ let __pe_b1: Arc<dyn ::std::ops::Fn(metamodelica::Ref<Expression::NFExpression>) -> Result<metamodelica::Ref<Expression::NFExpression>> + 'static> = func.clone(); move |__pe_a0| Binding::mapExp(__pe_a0, __pe_b1.clone()) }))?,
                    fixed = Util::applyOption(var_field!((*attributes).fixed, VariableAttributes::VAR_ATTR_STRING).clone(), &({ let __pe_b1: Arc<dyn ::std::ops::Fn(metamodelica::Ref<Expression::NFExpression>) -> Result<metamodelica::Ref<Expression::NFExpression>> + 'static> = func.clone(); move |__pe_a0| Binding::mapExp(__pe_a0, __pe_b1.clone()) }))?,
                    binding = Util::applyOption(var_field!((*attributes).binding, VariableAttributes::VAR_ATTR_STRING).clone(), &({ let __pe_b1: Arc<dyn ::std::ops::Fn(metamodelica::Ref<Expression::NFExpression>) -> Result<metamodelica::Ref<Expression::NFExpression>> + 'static> = func.clone(); move |__pe_a0| Binding::mapExp(__pe_a0, __pe_b1.clone()) }))?
                );
                attributes
            }
            VAR_ATTR_ENUMERATION {
                quantity: __attributes_quantity,
                ..
            } => {
                assign_variant_field!(attributes => VariableAttributes::VAR_ATTR_ENUMERATION;
                    quantity = Util::applyOption(__attributes_quantity.clone(), &({ let __pe_b1: Arc<dyn ::std::ops::Fn(metamodelica::Ref<Expression::NFExpression>) -> Result<metamodelica::Ref<Expression::NFExpression>> + 'static> = func.clone(); move |__pe_a0| Binding::mapExp(__pe_a0, __pe_b1.clone()) }))?,
                    min = Util::applyOption(var_field!((*attributes).min, VariableAttributes::VAR_ATTR_ENUMERATION).clone(), &({ let __pe_b1: Arc<dyn ::std::ops::Fn(metamodelica::Ref<Expression::NFExpression>) -> Result<metamodelica::Ref<Expression::NFExpression>> + 'static> = func.clone(); move |__pe_a0| Binding::mapExp(__pe_a0, __pe_b1.clone()) }))?,
                    max = Util::applyOption(var_field!((*attributes).max, VariableAttributes::VAR_ATTR_ENUMERATION).clone(), &({ let __pe_b1: Arc<dyn ::std::ops::Fn(metamodelica::Ref<Expression::NFExpression>) -> Result<metamodelica::Ref<Expression::NFExpression>> + 'static> = func.clone(); move |__pe_a0| Binding::mapExp(__pe_a0, __pe_b1.clone()) }))?,
                    start = Util::applyOption(var_field!((*attributes).start, VariableAttributes::VAR_ATTR_ENUMERATION).clone(), &({ let __pe_b1: Arc<dyn ::std::ops::Fn(metamodelica::Ref<Expression::NFExpression>) -> Result<metamodelica::Ref<Expression::NFExpression>> + 'static> = func.clone(); move |__pe_a0| Binding::mapExp(__pe_a0, __pe_b1.clone()) }))?,
                    fixed = Util::applyOption(var_field!((*attributes).fixed, VariableAttributes::VAR_ATTR_ENUMERATION).clone(), &({ let __pe_b1: Arc<dyn ::std::ops::Fn(metamodelica::Ref<Expression::NFExpression>) -> Result<metamodelica::Ref<Expression::NFExpression>> + 'static> = func.clone(); move |__pe_a0| Binding::mapExp(__pe_a0, __pe_b1.clone()) }))?,
                    binding = Util::applyOption(var_field!((*attributes).binding, VariableAttributes::VAR_ATTR_ENUMERATION).clone(), &({ let __pe_b1: Arc<dyn ::std::ops::Fn(metamodelica::Ref<Expression::NFExpression>) -> Result<metamodelica::Ref<Expression::NFExpression>> + 'static> = func.clone(); move |__pe_a0| Binding::mapExp(__pe_a0, __pe_b1.clone()) }))?
                );
                attributes
            }
            VAR_ATTR_RECORD { .. } => {
                assign_variant_field!(attributes => VariableAttributes::VAR_ATTR_RECORD; childrenAttr = metamodelica::arrayFromVec(({
        let mut __acc: metamodelica::List<metamodelica::Ref<VariableAttributes>> = metamodelica::nil();
        for mut attr in (var_field!((*attributes).childrenAttr, VariableAttributes::VAR_ATTR_RECORD).clone()).borrow().iter() {
            let __x = map(attr.clone(), func.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }).into_iter().cloned().collect()));
                attributes
            }
            _ => attributes,
        });
        Ok(attributes)
    }

    pub fn setFixed(
        mut attributes: metamodelica::Ref<VariableAttributes>,
        mut ty: metamodelica::Ref<Type::NFType>,
        mut b: bool,
        mut overwrite: bool,
    ) -> Result<metamodelica::Ref<VariableAttributes>> {
        let mut attributes: metamodelica::Ref<VariableAttributes> = attributes;
        let mut sizes: metamodelica::List<i32>;
        let mut start: metamodelica::Ref<Expression::NFExpression>;
        let mut iter_range: metamodelica::Ref<Expression::NFExpression>;
        let mut fixedExp: metamodelica::Ref<Expression::NFExpression> =
            metamodelica::Ref::new(Expression::NFExpression::BOOLEAN { value: b });
        let mut step: Option<metamodelica::Ref<Expression::NFExpression>>;
        let mut iter_name: metamodelica::Ref<InstNode::InstNode>;
        let mut iterators: metamodelica::List<(
            metamodelica::Ref<InstNode::InstNode>,
            metamodelica::Ref<Expression::NFExpression>,
        )> = metamodelica::nil();
        let mut fixedBinding: metamodelica::Ref<Binding::NFBinding>;
        if Type::isArray(&ty) {
            sizes = ({
                let mut __acc: metamodelica::List<i32> = metamodelica::nil();
                for mut dim in (Type::arrayDims(ty.clone())).into_iter().cloned() {
                    let __x = Dimension::size(&(dim.clone()), false)?;
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            });
            start = metamodelica::Ref::new(Expression::NFExpression::INTEGER { value: 1 });
            step = None;
            for mut stop in &*sizes {
                iter_name = InstNode::newUniqueIterator(Absyn::dummyInfo.clone(), crate::NFType::interned_INTEGER());
                iter_range = metamodelica::Ref::new(Expression::NFExpression::RANGE {
                    ty: crate::NFType::interned_INTEGER(),
                    start: start.clone(),
                    step: step.clone(),
                    stop: metamodelica::Ref::new(Expression::NFExpression::INTEGER { value: stop.clone() }),
                });
                iterators = metamodelica::cons((iter_name, iter_range), iterators);
            }
            fixedExp = metamodelica::Ref::new(Expression::NFExpression::CALL {
                call: metamodelica::Ref::new(Call::NFCall::TYPED_ARRAY_CONSTRUCTOR {
                    ty: ty,
                    var: Expression::variability(fixedExp.clone())?,
                    purity: NFPrefixes::Purity::PURE.clone(),
                    exp: fixedExp,
                    iters: iterators.reverse(),
                }),
            });
        }
        fixedBinding = Binding::makeFlat(
            fixedExp,
            Variability::CONSTANT.clone(),
            Source::GENERATED.clone(),
            Binding::NO_CONFIDENCE.clone(),
        );
        attributes = (match &*attributes {
            VAR_ATTR_REAL {
                fixed: __attributes_fixed,
                ..
            } if (overwrite || (__attributes_fixed).is_none()) => {
                assign_variant_field!(attributes => VariableAttributes::VAR_ATTR_REAL; fixed = Some(fixedBinding));
                attributes
            }
            VAR_ATTR_INT {
                fixed: __attributes_fixed,
                ..
            } if (overwrite || (__attributes_fixed).is_none()) => {
                assign_variant_field!(attributes => VariableAttributes::VAR_ATTR_INT; fixed = Some(fixedBinding));
                attributes
            }
            VAR_ATTR_BOOL {
                fixed: __attributes_fixed,
                ..
            } if (overwrite || (__attributes_fixed).is_none()) => {
                assign_variant_field!(attributes => VariableAttributes::VAR_ATTR_BOOL; fixed = Some(fixedBinding));
                attributes
            }
            VAR_ATTR_STRING {
                fixed: __attributes_fixed,
                ..
            } if (overwrite || (__attributes_fixed).is_none()) => {
                assign_variant_field!(attributes => VariableAttributes::VAR_ATTR_STRING; fixed = Some(fixedBinding));
                attributes
            }
            VAR_ATTR_ENUMERATION {
                fixed: __attributes_fixed,
                ..
            } if (overwrite || (__attributes_fixed).is_none()) => {
                assign_variant_field!(attributes => VariableAttributes::VAR_ATTR_ENUMERATION; fixed = Some(fixedBinding));
                attributes
            }
            _ => attributes,
        });
        Ok(attributes)
    }

    pub(crate) fn isFixed(mut attributes: &metamodelica::Ref<VariableAttributes>) -> Result<bool> {
        let mut fixed: bool;
        fixed = (::match_deref::match_deref! { match attributes {
            Deref @ VAR_ATTR_REAL { fixed: Some(b), .. } => {
                Expression::isAllTrue(Binding::getTypedExp(metamodelica::AsArg::as_arg(&b))?)?
            },
            Deref @ VAR_ATTR_INT { fixed: Some(b), .. } => {
                Expression::isAllTrue(Binding::getTypedExp(metamodelica::AsArg::as_arg(&b))?)?
            },
            Deref @ VAR_ATTR_BOOL { fixed: Some(b), .. } => {
                Expression::isAllTrue(Binding::getTypedExp(metamodelica::AsArg::as_arg(&b))?)?
            },
            Deref @ VAR_ATTR_STRING { fixed: Some(b), .. } => {
                Expression::isAllTrue(Binding::getTypedExp(metamodelica::AsArg::as_arg(&b))?)?
            },
            Deref @ VAR_ATTR_ENUMERATION { fixed: Some(b), .. } => {
                Expression::isAllTrue(Binding::getTypedExp(metamodelica::AsArg::as_arg(&b))?)?
            },
            _ => {
                false
            },
            _ => unreachable!("match_deref! exhaustiveness placeholder"),
        } });
        Ok(fixed)
    }

    pub fn setStartAttribute(
        mut attributes: metamodelica::Ref<VariableAttributes>,
        mut start: metamodelica::Ref<Expression::NFExpression>,
        mut overwrite: bool,
    ) -> Result<metamodelica::Ref<VariableAttributes>> {
        let mut attributes: metamodelica::Ref<VariableAttributes> = attributes;
        let mut startBinding: metamodelica::Ref<Binding::NFBinding>;
        startBinding = Binding::makeFlat(
            start.clone(),
            Expression::variability(start)?,
            Source::GENERATED.clone(),
            Binding::NO_CONFIDENCE.clone(),
        );
        attributes = (match &*attributes {
            VAR_ATTR_REAL {
                start: __attributes_start,
                ..
            } if (overwrite || (__attributes_start).is_none()) => {
                assign_variant_field!(attributes => VariableAttributes::VAR_ATTR_REAL; start = Some(startBinding));
                attributes
            }
            VAR_ATTR_INT {
                start: __attributes_start,
                ..
            } if (overwrite || (__attributes_start).is_none()) => {
                assign_variant_field!(attributes => VariableAttributes::VAR_ATTR_INT; start = Some(startBinding));
                attributes
            }
            VAR_ATTR_BOOL {
                start: __attributes_start,
                ..
            } if (overwrite || (__attributes_start).is_none()) => {
                assign_variant_field!(attributes => VariableAttributes::VAR_ATTR_BOOL; start = Some(startBinding));
                attributes
            }
            VAR_ATTR_STRING {
                start: __attributes_start,
                ..
            } if (overwrite || (__attributes_start).is_none()) => {
                assign_variant_field!(attributes => VariableAttributes::VAR_ATTR_STRING; start = Some(startBinding));
                attributes
            }
            VAR_ATTR_ENUMERATION {
                start: __attributes_start,
                ..
            } if (overwrite || (__attributes_start).is_none()) => {
                assign_variant_field!(attributes => VariableAttributes::VAR_ATTR_ENUMERATION; start = Some(startBinding));
                attributes
            }
            _ => attributes,
        });
        Ok(attributes)
    }

    pub fn getStartAttribute(
        mut attributes: &metamodelica::Ref<VariableAttributes>,
    ) -> Result<Option<metamodelica::Ref<Expression::NFExpression>>> {
        let mut start: Option<metamodelica::Ref<Expression::NFExpression>>;
        start = (match &**attributes {
            VAR_ATTR_REAL {
                start: __attributes_start,
                ..
            } => Util::applyOption(__attributes_start.clone(), &move |__a0: metamodelica::Ref<
                Binding::NFBinding,
            >| Binding::getTypedExp(&__a0))?,
            VAR_ATTR_INT {
                start: __attributes_start,
                ..
            } => Util::applyOption(__attributes_start.clone(), &move |__a0: metamodelica::Ref<
                Binding::NFBinding,
            >| Binding::getTypedExp(&__a0))?,
            VAR_ATTR_BOOL {
                start: __attributes_start,
                ..
            } => Util::applyOption(__attributes_start.clone(), &move |__a0: metamodelica::Ref<
                Binding::NFBinding,
            >| Binding::getTypedExp(&__a0))?,
            VAR_ATTR_STRING {
                start: __attributes_start,
                ..
            } => Util::applyOption(__attributes_start.clone(), &move |__a0: metamodelica::Ref<
                Binding::NFBinding,
            >| Binding::getTypedExp(&__a0))?,
            VAR_ATTR_ENUMERATION {
                start: __attributes_start,
                ..
            } => Util::applyOption(__attributes_start.clone(), &move |__a0: metamodelica::Ref<
                Binding::NFBinding,
            >| Binding::getTypedExp(&__a0))?,
            _ => None,
        });
        Ok(start)
    }

    pub fn setMin(
        mut attributes: metamodelica::Ref<VariableAttributes>,
        mut min_val: Option<metamodelica::Ref<Expression::NFExpression>>,
        mut overwrite: bool,
    ) -> Result<metamodelica::Ref<VariableAttributes>> {
        let mut attributes: metamodelica::Ref<VariableAttributes> = attributes;
        let mut min_binding: Option<metamodelica::Ref<Binding::NFBinding>> = Util::applyOption(
            min_val.clone(),
            &fnptr!(expToGeneratedBinding, metamodelica::Ref<Expression::NFExpression>),
        )?;
        attributes = (match &*attributes {
            VAR_ATTR_REAL {
                min: __attributes_min, ..
            } if (overwrite || (__attributes_min).is_none()) => {
                assign_variant_field!(attributes => VariableAttributes::VAR_ATTR_REAL; min = min_binding);
                attributes
            }
            VAR_ATTR_INT {
                min: __attributes_min, ..
            } if (overwrite || (__attributes_min).is_none()) => {
                assign_variant_field!(attributes => VariableAttributes::VAR_ATTR_INT; min = min_binding);
                attributes
            }
            VAR_ATTR_ENUMERATION {
                min: __attributes_min, ..
            } if (overwrite || (__attributes_min).is_none()) => {
                assign_variant_field!(attributes => VariableAttributes::VAR_ATTR_ENUMERATION; min = min_binding);
                attributes
            }
            _ => attributes,
        });
        Ok(attributes)
    }

    pub fn setMax(
        mut attributes: metamodelica::Ref<VariableAttributes>,
        mut max_val: Option<metamodelica::Ref<Expression::NFExpression>>,
        mut overwrite: bool,
    ) -> Result<metamodelica::Ref<VariableAttributes>> {
        let mut attributes: metamodelica::Ref<VariableAttributes> = attributes;
        let mut max_binding: Option<metamodelica::Ref<Binding::NFBinding>> = Util::applyOption(
            max_val.clone(),
            &fnptr!(expToGeneratedBinding, metamodelica::Ref<Expression::NFExpression>),
        )?;
        attributes = (match &*attributes {
            VAR_ATTR_REAL {
                max: __attributes_max, ..
            } if (overwrite || (__attributes_max).is_none()) => {
                assign_variant_field!(attributes => VariableAttributes::VAR_ATTR_REAL; max = max_binding);
                attributes
            }
            VAR_ATTR_INT {
                max: __attributes_max, ..
            } if (overwrite || (__attributes_max).is_none()) => {
                assign_variant_field!(attributes => VariableAttributes::VAR_ATTR_INT; max = max_binding);
                attributes
            }
            VAR_ATTR_ENUMERATION {
                max: __attributes_max, ..
            } if (overwrite || (__attributes_max).is_none()) => {
                assign_variant_field!(attributes => VariableAttributes::VAR_ATTR_ENUMERATION; max = max_binding);
                attributes
            }
            _ => attributes,
        });
        Ok(attributes)
    }

    pub fn merge(
        mut dst: metamodelica::Ref<VariableAttributes>,
        mut src: &metamodelica::Ref<VariableAttributes>,
    ) -> Result<metamodelica::Ref<VariableAttributes>> {
        let mut dst: metamodelica::Ref<VariableAttributes> = dst;
        dst = (::match_deref::match_deref! { match &((dst.clone(), src.clone())) {
            (Deref @ VAR_ATTR_REAL { .. }, Deref @ VAR_ATTR_REAL { .. }) => {
                assign_variant_field!(dst => VariableAttributes::VAR_ATTR_REAL;
                    quantity = mergeOpt(var_field!((*dst).quantity, VariableAttributes::VAR_ATTR_REAL).clone(), var_field!((**src).quantity, VariableAttributes::VAR_ATTR_REAL).clone()),
                    unit = mergeOpt(var_field!((*dst).unit, VariableAttributes::VAR_ATTR_REAL).clone(), var_field!((**src).unit, VariableAttributes::VAR_ATTR_REAL).clone()),
                    displayUnit = mergeOpt(var_field!((*dst).displayUnit, VariableAttributes::VAR_ATTR_REAL).clone(), var_field!((**src).displayUnit, VariableAttributes::VAR_ATTR_REAL).clone()),
                    min = tightestBound(var_field!((*dst).min, VariableAttributes::VAR_ATTR_REAL).clone(), var_field!((**src).min, VariableAttributes::VAR_ATTR_REAL).clone(), true)?,
                    max = tightestBound(var_field!((*dst).max, VariableAttributes::VAR_ATTR_REAL).clone(), var_field!((**src).max, VariableAttributes::VAR_ATTR_REAL).clone(), false)?,
                    start = mergeOpt(var_field!((*dst).start, VariableAttributes::VAR_ATTR_REAL).clone(), var_field!((**src).start, VariableAttributes::VAR_ATTR_REAL).clone()),
                    fixed = mergeOpt(var_field!((*dst).fixed, VariableAttributes::VAR_ATTR_REAL).clone(), var_field!((**src).fixed, VariableAttributes::VAR_ATTR_REAL).clone()),
                    nominal = mergeOpt(var_field!((*dst).nominal, VariableAttributes::VAR_ATTR_REAL).clone(), var_field!((**src).nominal, VariableAttributes::VAR_ATTR_REAL).clone())
                );
                dst
            },
            (Deref @ VAR_ATTR_INT { .. }, Deref @ VAR_ATTR_INT { .. }) => {
                assign_variant_field!(dst => VariableAttributes::VAR_ATTR_INT;
                    quantity = mergeOpt(var_field!((*dst).quantity, VariableAttributes::VAR_ATTR_INT).clone(), var_field!((**src).quantity, VariableAttributes::VAR_ATTR_INT).clone()),
                    min = tightestBound(var_field!((*dst).min, VariableAttributes::VAR_ATTR_INT).clone(), var_field!((**src).min, VariableAttributes::VAR_ATTR_INT).clone(), true)?,
                    max = tightestBound(var_field!((*dst).max, VariableAttributes::VAR_ATTR_INT).clone(), var_field!((**src).max, VariableAttributes::VAR_ATTR_INT).clone(), false)?,
                    start = mergeOpt(var_field!((*dst).start, VariableAttributes::VAR_ATTR_INT).clone(), var_field!((**src).start, VariableAttributes::VAR_ATTR_INT).clone()),
                    fixed = mergeOpt(var_field!((*dst).fixed, VariableAttributes::VAR_ATTR_INT).clone(), var_field!((**src).fixed, VariableAttributes::VAR_ATTR_INT).clone())
                );
                dst
            },
            (Deref @ VAR_ATTR_BOOL { .. }, Deref @ VAR_ATTR_BOOL { .. }) => {
                assign_variant_field!(dst => VariableAttributes::VAR_ATTR_BOOL;
                    quantity = mergeOpt(var_field!((*dst).quantity, VariableAttributes::VAR_ATTR_BOOL).clone(), var_field!((**src).quantity, VariableAttributes::VAR_ATTR_BOOL).clone()),
                    start = mergeOpt(var_field!((*dst).start, VariableAttributes::VAR_ATTR_BOOL).clone(), var_field!((**src).start, VariableAttributes::VAR_ATTR_BOOL).clone()),
                    fixed = mergeOpt(var_field!((*dst).fixed, VariableAttributes::VAR_ATTR_BOOL).clone(), var_field!((**src).fixed, VariableAttributes::VAR_ATTR_BOOL).clone())
                );
                dst
            },
            (Deref @ VAR_ATTR_STRING { .. }, Deref @ VAR_ATTR_STRING { .. }) => {
                assign_variant_field!(dst => VariableAttributes::VAR_ATTR_STRING;
                    quantity = mergeOpt(var_field!((*dst).quantity, VariableAttributes::VAR_ATTR_STRING).clone(), var_field!((**src).quantity, VariableAttributes::VAR_ATTR_STRING).clone()),
                    start = mergeOpt(var_field!((*dst).start, VariableAttributes::VAR_ATTR_STRING).clone(), var_field!((**src).start, VariableAttributes::VAR_ATTR_STRING).clone()),
                    fixed = mergeOpt(var_field!((*dst).fixed, VariableAttributes::VAR_ATTR_STRING).clone(), var_field!((**src).fixed, VariableAttributes::VAR_ATTR_STRING).clone())
                );
                dst
            },
            (Deref @ VAR_ATTR_ENUMERATION { .. }, Deref @ VAR_ATTR_ENUMERATION { .. }) => {
                assign_variant_field!(dst => VariableAttributes::VAR_ATTR_ENUMERATION;
                    quantity = mergeOpt(var_field!((*dst).quantity, VariableAttributes::VAR_ATTR_ENUMERATION).clone(), var_field!((**src).quantity, VariableAttributes::VAR_ATTR_ENUMERATION).clone()),
                    min = mergeOpt(var_field!((*dst).min, VariableAttributes::VAR_ATTR_ENUMERATION).clone(), var_field!((**src).min, VariableAttributes::VAR_ATTR_ENUMERATION).clone()),
                    max = mergeOpt(var_field!((*dst).max, VariableAttributes::VAR_ATTR_ENUMERATION).clone(), var_field!((**src).max, VariableAttributes::VAR_ATTR_ENUMERATION).clone()),
                    start = mergeOpt(var_field!((*dst).start, VariableAttributes::VAR_ATTR_ENUMERATION).clone(), var_field!((**src).start, VariableAttributes::VAR_ATTR_ENUMERATION).clone()),
                    fixed = mergeOpt(var_field!((*dst).fixed, VariableAttributes::VAR_ATTR_ENUMERATION).clone(), var_field!((**src).fixed, VariableAttributes::VAR_ATTR_ENUMERATION).clone())
                );
                dst
            },
            _ => dst,
            _ => unreachable!("match_deref! exhaustiveness placeholder"),
        } });
        Ok(dst)
    }

    pub(crate) fn mergeOpt(
        mut dst: Option<metamodelica::Ref<Binding::NFBinding>>,
        mut src: Option<metamodelica::Ref<Binding::NFBinding>>,
    ) -> Option<metamodelica::Ref<Binding::NFBinding>> {
        let mut dst: Option<metamodelica::Ref<Binding::NFBinding>> = dst;
        dst = if ((dst).is_some()) { dst } else { src };
        dst
    }

    pub(crate) fn tightestBound(
        mut dst: Option<metamodelica::Ref<Binding::NFBinding>>,
        mut src: Option<metamodelica::Ref<Binding::NFBinding>>,
        mut isMin: bool,
    ) -> Result<Option<metamodelica::Ref<Binding::NFBinding>>> {
        let mut res: Option<metamodelica::Ref<Binding::NFBinding>>;
        let mut db: metamodelica::Ref<Binding::NFBinding>;
        let mut sb: metamodelica::Ref<Binding::NFBinding>;
        let mut de: metamodelica::Ref<Expression::NFExpression>;
        let mut se: metamodelica::Ref<Expression::NFExpression>;
        let mut dv: metamodelica::Real;
        let mut sv: metamodelica::Real;
        if (dst).is_none() {
            res = src;
        } else if (src).is_none() {
            res = dst;
        } else {
            let __pa0 = ::match_deref::match_deref! { match &(dst.clone()) {
                Some(__pa0) => __pa0.clone(),
                _ => return Err("pattern mismatch"),
            } };
            db = metamodelica::Own::own(__pa0);
            let __pa1 = ::match_deref::match_deref! { match &(src.clone()) {
                Some(__pa1) => __pa1.clone(),
                _ => return Err("pattern mismatch"),
            } };
            sb = metamodelica::Own::own(__pa1);
            de = Binding::getTypedExp(&db)?;
            se = Binding::getTypedExp(&sb)?;
            if Expression::isConstNumber(&de) && Expression::isConstNumber(&se) {
                dv = Expression::realValue(&de)?;
                sv = Expression::realValue(&se)?;
                if isMin {
                    res = if (sv > dv) { src } else { dst };
                } else {
                    res = if (sv < dv) { src } else { dst };
                }
            } else {
                res = dst;
            }
        }
        Ok(res)
    }

    pub fn setStateSelect(
        mut attributes: metamodelica::Ref<VariableAttributes>,
        mut stateSelect_val: StateSelect,
        mut overwrite: bool,
    ) -> metamodelica::Ref<VariableAttributes> {
        let mut attributes: metamodelica::Ref<VariableAttributes> = attributes;
        attributes = (match &*attributes {
            VAR_ATTR_REAL {
                stateSelect: __attributes_stateSelect,
                ..
            } if (overwrite || (__attributes_stateSelect).is_none()) => {
                assign_variant_field!(attributes => VariableAttributes::VAR_ATTR_REAL; stateSelect = Some(stateSelect_val));
                attributes
            }
            _ => attributes,
        });
        attributes
    }

    pub fn getStateSelect(mut attributes: &metamodelica::Ref<VariableAttributes>) -> StateSelect {
        let mut stateSelect: StateSelect;
        stateSelect = (match &**attributes {
            VAR_ATTR_REAL {
                stateSelect: Some(__esc_stateSelect),
                ..
            } => {
                stateSelect = (*__esc_stateSelect).clone();
                stateSelect.clone()
            }
            _ => StateSelect::DEFAULT.clone(),
        });
        stateSelect
    }

    pub fn setTearingSelect(
        mut attributes: metamodelica::Ref<VariableAttributes>,
        mut tearingSelect_val: TearingSelect,
        mut overwrite: bool,
    ) -> metamodelica::Ref<VariableAttributes> {
        let mut attributes: metamodelica::Ref<VariableAttributes> = attributes;
        attributes = (match &*attributes {
            VAR_ATTR_REAL {
                tearingSelect: __attributes_tearingSelect,
                ..
            } if (overwrite || (__attributes_tearingSelect).is_none()) => {
                assign_variant_field!(attributes => VariableAttributes::VAR_ATTR_REAL; tearingSelect = Some(tearingSelect_val));
                attributes
            }
            _ => attributes,
        });
        attributes
    }

    pub fn getTearingSelect(mut attributes: &metamodelica::Ref<VariableAttributes>) -> TearingSelect {
        let mut tearingSelect: TearingSelect;
        tearingSelect = (match &**attributes {
            VAR_ATTR_REAL {
                tearingSelect: Some(__esc_tearingSelect),
                ..
            } => {
                tearingSelect = (*__esc_tearingSelect).clone();
                tearingSelect.clone()
            }
            _ => TearingSelect::DEFAULT.clone(),
        });
        tearingSelect
    }

    pub fn getMin(
        mut attr: &metamodelica::Ref<VariableAttributes>,
    ) -> Result<Option<metamodelica::Ref<Expression::NFExpression>>> {
        let mut min: Option<metamodelica::Ref<Expression::NFExpression>>;
        min = (match &**attr {
            VAR_ATTR_REAL { min: __attr_min, .. } => {
                Util::applyOption(__attr_min.clone(), &move |__a0: metamodelica::Ref<
                    Binding::NFBinding,
                >| Binding::getTypedExp(&__a0))?
            }
            VAR_ATTR_INT { min: __attr_min, .. } => {
                Util::applyOption(__attr_min.clone(), &move |__a0: metamodelica::Ref<
                    Binding::NFBinding,
                >| Binding::getTypedExp(&__a0))?
            }
            VAR_ATTR_ENUMERATION { min: __attr_min, .. } => {
                Util::applyOption(__attr_min.clone(), &move |__a0: metamodelica::Ref<
                    Binding::NFBinding,
                >| Binding::getTypedExp(&__a0))?
            }
            _ => None,
        });
        Ok(min)
    }

    pub fn getMax(
        mut attr: &metamodelica::Ref<VariableAttributes>,
    ) -> Result<Option<metamodelica::Ref<Expression::NFExpression>>> {
        let mut max: Option<metamodelica::Ref<Expression::NFExpression>>;
        max = (match &**attr {
            VAR_ATTR_REAL { max: __attr_max, .. } => {
                Util::applyOption(__attr_max.clone(), &move |__a0: metamodelica::Ref<
                    Binding::NFBinding,
                >| Binding::getTypedExp(&__a0))?
            }
            VAR_ATTR_INT { max: __attr_max, .. } => {
                Util::applyOption(__attr_max.clone(), &move |__a0: metamodelica::Ref<
                    Binding::NFBinding,
                >| Binding::getTypedExp(&__a0))?
            }
            VAR_ATTR_ENUMERATION { max: __attr_max, .. } => {
                Util::applyOption(__attr_max.clone(), &move |__a0: metamodelica::Ref<
                    Binding::NFBinding,
                >| Binding::getTypedExp(&__a0))?
            }
            _ => None,
        });
        Ok(max)
    }

    pub fn getNominal(
        mut attr: &metamodelica::Ref<VariableAttributes>,
    ) -> Result<Option<metamodelica::Ref<Expression::NFExpression>>> {
        let mut nominal: Option<metamodelica::Ref<Expression::NFExpression>>;
        nominal = (match &**attr {
            VAR_ATTR_REAL {
                nominal: __attr_nominal,
                ..
            } => Util::applyOption(__attr_nominal.clone(), &move |__a0: metamodelica::Ref<
                Binding::NFBinding,
            >| Binding::getTypedExp(&__a0))?,
            _ => None,
        });
        Ok(nominal)
    }

    pub(crate) fn scalarizeReal(
        mut quantity_iter: metamodelica::Ref<ExpressionIterator::NFExpressionIterator>,
        mut unit_iter: metamodelica::Ref<ExpressionIterator::NFExpressionIterator>,
        mut displayUnit_iter: metamodelica::Ref<ExpressionIterator::NFExpressionIterator>,
        mut min_iter: metamodelica::Ref<ExpressionIterator::NFExpressionIterator>,
        mut max_iter: metamodelica::Ref<ExpressionIterator::NFExpressionIterator>,
        mut start_iter: metamodelica::Ref<ExpressionIterator::NFExpressionIterator>,
        mut fixed_iter: metamodelica::Ref<ExpressionIterator::NFExpressionIterator>,
        mut nominal_iter: metamodelica::Ref<ExpressionIterator::NFExpressionIterator>,
        mut stateSelect: Option<StateSelect>,
        mut tearingSelect: Option<TearingSelect>,
        mut uncertainty: Option<Uncertainty>,
        mut distribution: Option<Distribution>,
        mut binding_iter: metamodelica::Ref<ExpressionIterator::NFExpressionIterator>,
        mut isProtected: Option<bool>,
        mut finalPrefix: Option<bool>,
        mut length: i32,
    ) -> Result<(metamodelica::List<metamodelica::Ref<VariableAttributes>>, bool)> {
        let mut scalar_attributes: metamodelica::List<metamodelica::Ref<VariableAttributes>> = metamodelica::nil();
        let mut uniform: bool;
        let mut quantity_e: Option<metamodelica::Ref<Expression::NFExpression>>;
        let mut unit_e: Option<metamodelica::Ref<Expression::NFExpression>>;
        let mut displayUnit_e: Option<metamodelica::Ref<Expression::NFExpression>>;
        let mut min_e: Option<metamodelica::Ref<Expression::NFExpression>>;
        let mut max_e: Option<metamodelica::Ref<Expression::NFExpression>>;
        let mut start_e: Option<metamodelica::Ref<Expression::NFExpression>>;
        let mut fixed_e: Option<metamodelica::Ref<Expression::NFExpression>>;
        let mut nominal_e: Option<metamodelica::Ref<Expression::NFExpression>>;
        let mut binding_e: Option<metamodelica::Ref<Expression::NFExpression>>;
        let mut quantity_loc: metamodelica::Ref<ExpressionIterator::NFExpressionIterator> = quantity_iter.clone();
        let mut unit_loc: metamodelica::Ref<ExpressionIterator::NFExpressionIterator> = unit_iter.clone();
        let mut displayUnit_loc: metamodelica::Ref<ExpressionIterator::NFExpressionIterator> = displayUnit_iter.clone();
        let mut min_loc: metamodelica::Ref<ExpressionIterator::NFExpressionIterator> = min_iter.clone();
        let mut max_loc: metamodelica::Ref<ExpressionIterator::NFExpressionIterator> = max_iter.clone();
        let mut start_loc: metamodelica::Ref<ExpressionIterator::NFExpressionIterator> = start_iter.clone();
        let mut fixed_loc: metamodelica::Ref<ExpressionIterator::NFExpressionIterator> = fixed_iter.clone();
        let mut nominal_loc: metamodelica::Ref<ExpressionIterator::NFExpressionIterator> = nominal_iter.clone();
        let mut binding_loc: metamodelica::Ref<ExpressionIterator::NFExpressionIterator> = binding_iter.clone();
        uniform = length > 1
            && List::all(
                &(list![
                    quantity_iter,
                    unit_iter,
                    displayUnit_iter,
                    min_iter,
                    max_iter,
                    start_iter,
                    fixed_iter,
                    nominal_iter,
                    binding_iter
                ]),
                &move |__a0: metamodelica::Ref<ExpressionIterator::NFExpressionIterator>| {
                    ExpressionIterator::isUniform(&__a0)
                },
            )?;
        for mut i in 1..=if (uniform) { 1 } else { length } {
            (quantity_loc, quantity_e) = ExpressionIterator::nextOpt(quantity_loc)?;
            (unit_loc, unit_e) = ExpressionIterator::nextOpt(unit_loc)?;
            (displayUnit_loc, displayUnit_e) = ExpressionIterator::nextOpt(displayUnit_loc)?;
            (min_loc, min_e) = ExpressionIterator::nextOpt(min_loc)?;
            (max_loc, max_e) = ExpressionIterator::nextOpt(max_loc)?;
            (start_loc, start_e) = ExpressionIterator::nextOpt(start_loc)?;
            (fixed_loc, fixed_e) = ExpressionIterator::nextOpt(fixed_loc)?;
            (nominal_loc, nominal_e) = ExpressionIterator::nextOpt(nominal_loc)?;
            (binding_loc, binding_e) = ExpressionIterator::nextOpt(binding_loc)?;
            scalar_attributes = metamodelica::cons(
                metamodelica::Ref::new(VariableAttributes::VAR_ATTR_REAL {
                    quantity: Util::applyOption(
                        quantity_e,
                        &fnptr!(expToGeneratedBinding, metamodelica::Ref<Expression::NFExpression>),
                    )?,
                    unit: Util::applyOption(
                        unit_e,
                        &fnptr!(expToGeneratedBinding, metamodelica::Ref<Expression::NFExpression>),
                    )?,
                    displayUnit: Util::applyOption(
                        displayUnit_e,
                        &fnptr!(expToGeneratedBinding, metamodelica::Ref<Expression::NFExpression>),
                    )?,
                    min: Util::applyOption(
                        min_e,
                        &fnptr!(expToGeneratedBinding, metamodelica::Ref<Expression::NFExpression>),
                    )?,
                    max: Util::applyOption(
                        max_e,
                        &fnptr!(expToGeneratedBinding, metamodelica::Ref<Expression::NFExpression>),
                    )?,
                    start: Util::applyOption(
                        start_e,
                        &fnptr!(expToGeneratedBinding, metamodelica::Ref<Expression::NFExpression>),
                    )?,
                    fixed: Util::applyOption(
                        fixed_e,
                        &fnptr!(expToGeneratedBinding, metamodelica::Ref<Expression::NFExpression>),
                    )?,
                    nominal: Util::applyOption(
                        nominal_e,
                        &fnptr!(expToGeneratedBinding, metamodelica::Ref<Expression::NFExpression>),
                    )?,
                    stateSelect: stateSelect.clone(),
                    tearingSelect: tearingSelect.clone(),
                    uncertainty: uncertainty.clone(),
                    distribution: distribution.clone(),
                    binding: Util::applyOption(
                        binding_e,
                        &fnptr!(expToGeneratedBinding, metamodelica::Ref<Expression::NFExpression>),
                    )?,
                    isProtected: isProtected.clone(),
                    finalPrefix: finalPrefix.clone(),
                }),
                scalar_attributes,
            );
        }
        scalar_attributes = if (uniform) {
            List::fill((scalar_attributes).head().cloned()?, length)
        } else {
            metamodelica::Dangerous::listReverseInPlace(scalar_attributes)
        };
        Ok((scalar_attributes, uniform))
    }

    pub(crate) fn scalarizeInt(
        mut quantity_iter: metamodelica::Ref<ExpressionIterator::NFExpressionIterator>,
        mut min_iter: metamodelica::Ref<ExpressionIterator::NFExpressionIterator>,
        mut max_iter: metamodelica::Ref<ExpressionIterator::NFExpressionIterator>,
        mut start_iter: metamodelica::Ref<ExpressionIterator::NFExpressionIterator>,
        mut fixed_iter: metamodelica::Ref<ExpressionIterator::NFExpressionIterator>,
        mut uncertainty: Option<Uncertainty>,
        mut distribution: Option<Distribution>,
        mut binding_iter: metamodelica::Ref<ExpressionIterator::NFExpressionIterator>,
        mut isProtected: Option<bool>,
        mut finalPrefix: Option<bool>,
        mut length: i32,
    ) -> Result<(metamodelica::List<metamodelica::Ref<VariableAttributes>>, bool)> {
        let mut scalar_attributes: metamodelica::List<metamodelica::Ref<VariableAttributes>> = metamodelica::nil();
        let mut uniform: bool;
        let mut quantity_e: Option<metamodelica::Ref<Expression::NFExpression>>;
        let mut min_e: Option<metamodelica::Ref<Expression::NFExpression>>;
        let mut max_e: Option<metamodelica::Ref<Expression::NFExpression>>;
        let mut start_e: Option<metamodelica::Ref<Expression::NFExpression>>;
        let mut fixed_e: Option<metamodelica::Ref<Expression::NFExpression>>;
        let mut binding_e: Option<metamodelica::Ref<Expression::NFExpression>>;
        let mut quantity_loc: metamodelica::Ref<ExpressionIterator::NFExpressionIterator> = quantity_iter.clone();
        let mut min_loc: metamodelica::Ref<ExpressionIterator::NFExpressionIterator> = min_iter.clone();
        let mut max_loc: metamodelica::Ref<ExpressionIterator::NFExpressionIterator> = max_iter.clone();
        let mut start_loc: metamodelica::Ref<ExpressionIterator::NFExpressionIterator> = start_iter.clone();
        let mut fixed_loc: metamodelica::Ref<ExpressionIterator::NFExpressionIterator> = fixed_iter.clone();
        let mut binding_loc: metamodelica::Ref<ExpressionIterator::NFExpressionIterator> = binding_iter.clone();
        uniform = length > 1
            && List::all(
                &(list![quantity_iter, min_iter, max_iter, start_iter, fixed_iter, binding_iter]),
                &move |__a0: metamodelica::Ref<ExpressionIterator::NFExpressionIterator>| {
                    ExpressionIterator::isUniform(&__a0)
                },
            )?;
        for mut i in 1..=if (uniform) { 1 } else { length } {
            (quantity_loc, quantity_e) = ExpressionIterator::nextOpt(quantity_loc)?;
            (min_loc, min_e) = ExpressionIterator::nextOpt(min_loc)?;
            (max_loc, max_e) = ExpressionIterator::nextOpt(max_loc)?;
            (start_loc, start_e) = ExpressionIterator::nextOpt(start_loc)?;
            (fixed_loc, fixed_e) = ExpressionIterator::nextOpt(fixed_loc)?;
            (binding_loc, binding_e) = ExpressionIterator::nextOpt(binding_loc)?;
            scalar_attributes = metamodelica::cons(
                metamodelica::Ref::new(VariableAttributes::VAR_ATTR_INT {
                    quantity: Util::applyOption(
                        quantity_e,
                        &fnptr!(expToGeneratedBinding, metamodelica::Ref<Expression::NFExpression>),
                    )?,
                    min: Util::applyOption(
                        min_e,
                        &fnptr!(expToGeneratedBinding, metamodelica::Ref<Expression::NFExpression>),
                    )?,
                    max: Util::applyOption(
                        max_e,
                        &fnptr!(expToGeneratedBinding, metamodelica::Ref<Expression::NFExpression>),
                    )?,
                    start: Util::applyOption(
                        start_e,
                        &fnptr!(expToGeneratedBinding, metamodelica::Ref<Expression::NFExpression>),
                    )?,
                    fixed: Util::applyOption(
                        fixed_e,
                        &fnptr!(expToGeneratedBinding, metamodelica::Ref<Expression::NFExpression>),
                    )?,
                    uncertainty: uncertainty.clone(),
                    distribution: distribution.clone(),
                    binding: Util::applyOption(
                        binding_e,
                        &fnptr!(expToGeneratedBinding, metamodelica::Ref<Expression::NFExpression>),
                    )?,
                    isProtected: isProtected.clone(),
                    finalPrefix: finalPrefix.clone(),
                }),
                scalar_attributes,
            );
        }
        scalar_attributes = if (uniform) {
            List::fill((scalar_attributes).head().cloned()?, length)
        } else {
            metamodelica::Dangerous::listReverseInPlace(scalar_attributes)
        };
        Ok((scalar_attributes, uniform))
    }

    pub(crate) fn scalarizeBool(
        mut quantity_iter: metamodelica::Ref<ExpressionIterator::NFExpressionIterator>,
        mut start_iter: metamodelica::Ref<ExpressionIterator::NFExpressionIterator>,
        mut fixed_iter: metamodelica::Ref<ExpressionIterator::NFExpressionIterator>,
        mut binding_iter: metamodelica::Ref<ExpressionIterator::NFExpressionIterator>,
        mut isProtected: Option<bool>,
        mut finalPrefix: Option<bool>,
        mut length: i32,
    ) -> Result<(metamodelica::List<metamodelica::Ref<VariableAttributes>>, bool)> {
        let mut scalar_attributes: metamodelica::List<metamodelica::Ref<VariableAttributes>> = metamodelica::nil();
        let mut uniform: bool;
        let mut quantity_e: Option<metamodelica::Ref<Expression::NFExpression>>;
        let mut start_e: Option<metamodelica::Ref<Expression::NFExpression>>;
        let mut fixed_e: Option<metamodelica::Ref<Expression::NFExpression>>;
        let mut binding_e: Option<metamodelica::Ref<Expression::NFExpression>>;
        let mut quantity_loc: metamodelica::Ref<ExpressionIterator::NFExpressionIterator> = quantity_iter.clone();
        let mut start_loc: metamodelica::Ref<ExpressionIterator::NFExpressionIterator> = start_iter.clone();
        let mut fixed_loc: metamodelica::Ref<ExpressionIterator::NFExpressionIterator> = fixed_iter.clone();
        let mut binding_loc: metamodelica::Ref<ExpressionIterator::NFExpressionIterator> = binding_iter.clone();
        uniform = length > 1
            && List::all(
                &(list![quantity_iter, start_iter, fixed_iter, binding_iter]),
                &move |__a0: metamodelica::Ref<ExpressionIterator::NFExpressionIterator>| {
                    ExpressionIterator::isUniform(&__a0)
                },
            )?;
        for mut i in 1..=if (uniform) { 1 } else { length } {
            (quantity_loc, quantity_e) = ExpressionIterator::nextOpt(quantity_loc)?;
            (start_loc, start_e) = ExpressionIterator::nextOpt(start_loc)?;
            (fixed_loc, fixed_e) = ExpressionIterator::nextOpt(fixed_loc)?;
            (binding_loc, binding_e) = ExpressionIterator::nextOpt(binding_loc)?;
            scalar_attributes = metamodelica::cons(
                metamodelica::Ref::new(VariableAttributes::VAR_ATTR_BOOL {
                    quantity: Util::applyOption(
                        quantity_e,
                        &fnptr!(expToGeneratedBinding, metamodelica::Ref<Expression::NFExpression>),
                    )?,
                    start: Util::applyOption(
                        start_e,
                        &fnptr!(expToGeneratedBinding, metamodelica::Ref<Expression::NFExpression>),
                    )?,
                    fixed: Util::applyOption(
                        fixed_e,
                        &fnptr!(expToGeneratedBinding, metamodelica::Ref<Expression::NFExpression>),
                    )?,
                    binding: Util::applyOption(
                        binding_e,
                        &fnptr!(expToGeneratedBinding, metamodelica::Ref<Expression::NFExpression>),
                    )?,
                    isProtected: isProtected.clone(),
                    finalPrefix: finalPrefix.clone(),
                }),
                scalar_attributes,
            );
        }
        scalar_attributes = if (uniform) {
            List::fill((scalar_attributes).head().cloned()?, length)
        } else {
            metamodelica::Dangerous::listReverseInPlace(scalar_attributes)
        };
        Ok((scalar_attributes, uniform))
    }

    pub(crate) fn scalarizeClock(
        mut isProtected: Option<bool>,
        mut finalPrefix: Option<bool>,
        mut length: i32,
    ) -> (metamodelica::List<metamodelica::Ref<VariableAttributes>>, bool) {
        let mut scalar_attributes: metamodelica::List<metamodelica::Ref<VariableAttributes>> = List::fill(
            metamodelica::Ref::new(VariableAttributes::VAR_ATTR_CLOCK {
                isProtected: isProtected.clone(),
                finalPrefix: finalPrefix.clone(),
            }),
            length,
        );
        let mut uniform: bool = true;
        (scalar_attributes, uniform)
    }

    pub(crate) fn scalarizeString(
        mut quantity_iter: metamodelica::Ref<ExpressionIterator::NFExpressionIterator>,
        mut start_iter: metamodelica::Ref<ExpressionIterator::NFExpressionIterator>,
        mut fixed_iter: metamodelica::Ref<ExpressionIterator::NFExpressionIterator>,
        mut binding_iter: metamodelica::Ref<ExpressionIterator::NFExpressionIterator>,
        mut isProtected: Option<bool>,
        mut finalPrefix: Option<bool>,
        mut length: i32,
    ) -> Result<(metamodelica::List<metamodelica::Ref<VariableAttributes>>, bool)> {
        let mut scalar_attributes: metamodelica::List<metamodelica::Ref<VariableAttributes>> = metamodelica::nil();
        let mut uniform: bool;
        let mut quantity_e: Option<metamodelica::Ref<Expression::NFExpression>>;
        let mut start_e: Option<metamodelica::Ref<Expression::NFExpression>>;
        let mut fixed_e: Option<metamodelica::Ref<Expression::NFExpression>>;
        let mut binding_e: Option<metamodelica::Ref<Expression::NFExpression>>;
        let mut quantity_loc: metamodelica::Ref<ExpressionIterator::NFExpressionIterator> = quantity_iter.clone();
        let mut start_loc: metamodelica::Ref<ExpressionIterator::NFExpressionIterator> = start_iter.clone();
        let mut fixed_loc: metamodelica::Ref<ExpressionIterator::NFExpressionIterator> = fixed_iter.clone();
        let mut binding_loc: metamodelica::Ref<ExpressionIterator::NFExpressionIterator> = binding_iter.clone();
        uniform = length > 1
            && List::all(
                &(list![quantity_iter, start_iter, fixed_iter, binding_iter]),
                &move |__a0: metamodelica::Ref<ExpressionIterator::NFExpressionIterator>| {
                    ExpressionIterator::isUniform(&__a0)
                },
            )?;
        for mut i in 1..=if (uniform) { 1 } else { length } {
            (quantity_loc, quantity_e) = ExpressionIterator::nextOpt(quantity_loc)?;
            (start_loc, start_e) = ExpressionIterator::nextOpt(start_loc)?;
            (fixed_loc, fixed_e) = ExpressionIterator::nextOpt(fixed_loc)?;
            (binding_loc, binding_e) = ExpressionIterator::nextOpt(binding_loc)?;
            scalar_attributes = metamodelica::cons(
                metamodelica::Ref::new(VariableAttributes::VAR_ATTR_STRING {
                    quantity: Util::applyOption(
                        quantity_e,
                        &fnptr!(expToGeneratedBinding, metamodelica::Ref<Expression::NFExpression>),
                    )?,
                    start: Util::applyOption(
                        start_e,
                        &fnptr!(expToGeneratedBinding, metamodelica::Ref<Expression::NFExpression>),
                    )?,
                    fixed: Util::applyOption(
                        fixed_e,
                        &fnptr!(expToGeneratedBinding, metamodelica::Ref<Expression::NFExpression>),
                    )?,
                    binding: Util::applyOption(
                        binding_e,
                        &fnptr!(expToGeneratedBinding, metamodelica::Ref<Expression::NFExpression>),
                    )?,
                    isProtected: isProtected.clone(),
                    finalPrefix: finalPrefix.clone(),
                }),
                scalar_attributes,
            );
        }
        scalar_attributes = if (uniform) {
            List::fill((scalar_attributes).head().cloned()?, length)
        } else {
            metamodelica::Dangerous::listReverseInPlace(scalar_attributes)
        };
        Ok((scalar_attributes, uniform))
    }

    pub(crate) fn scalarizeEnumeration(
        mut quantity_iter: metamodelica::Ref<ExpressionIterator::NFExpressionIterator>,
        mut min_iter: metamodelica::Ref<ExpressionIterator::NFExpressionIterator>,
        mut max_iter: metamodelica::Ref<ExpressionIterator::NFExpressionIterator>,
        mut start_iter: metamodelica::Ref<ExpressionIterator::NFExpressionIterator>,
        mut fixed_iter: metamodelica::Ref<ExpressionIterator::NFExpressionIterator>,
        mut binding_iter: metamodelica::Ref<ExpressionIterator::NFExpressionIterator>,
        mut isProtected: Option<bool>,
        mut finalPrefix: Option<bool>,
        mut length: i32,
    ) -> Result<(metamodelica::List<metamodelica::Ref<VariableAttributes>>, bool)> {
        let mut scalar_attributes: metamodelica::List<metamodelica::Ref<VariableAttributes>> = metamodelica::nil();
        let mut uniform: bool;
        let mut quantity_e: Option<metamodelica::Ref<Expression::NFExpression>>;
        let mut min_e: Option<metamodelica::Ref<Expression::NFExpression>>;
        let mut max_e: Option<metamodelica::Ref<Expression::NFExpression>>;
        let mut start_e: Option<metamodelica::Ref<Expression::NFExpression>>;
        let mut fixed_e: Option<metamodelica::Ref<Expression::NFExpression>>;
        let mut binding_e: Option<metamodelica::Ref<Expression::NFExpression>>;
        let mut quantity_loc: metamodelica::Ref<ExpressionIterator::NFExpressionIterator> = quantity_iter.clone();
        let mut min_loc: metamodelica::Ref<ExpressionIterator::NFExpressionIterator> = min_iter.clone();
        let mut max_loc: metamodelica::Ref<ExpressionIterator::NFExpressionIterator> = max_iter.clone();
        let mut start_loc: metamodelica::Ref<ExpressionIterator::NFExpressionIterator> = start_iter.clone();
        let mut fixed_loc: metamodelica::Ref<ExpressionIterator::NFExpressionIterator> = fixed_iter.clone();
        let mut binding_loc: metamodelica::Ref<ExpressionIterator::NFExpressionIterator> = binding_iter.clone();
        uniform = length > 1
            && List::all(
                &(list![quantity_iter, min_iter, max_iter, start_iter, fixed_iter, binding_iter]),
                &move |__a0: metamodelica::Ref<ExpressionIterator::NFExpressionIterator>| {
                    ExpressionIterator::isUniform(&__a0)
                },
            )?;
        for mut i in 1..=if (uniform) { 1 } else { length } {
            (quantity_loc, quantity_e) = ExpressionIterator::nextOpt(quantity_loc)?;
            (min_loc, min_e) = ExpressionIterator::nextOpt(min_loc)?;
            (max_loc, max_e) = ExpressionIterator::nextOpt(max_loc)?;
            (start_loc, start_e) = ExpressionIterator::nextOpt(start_loc)?;
            (fixed_loc, fixed_e) = ExpressionIterator::nextOpt(fixed_loc)?;
            (binding_loc, binding_e) = ExpressionIterator::nextOpt(binding_loc)?;
            scalar_attributes = metamodelica::cons(
                metamodelica::Ref::new(VariableAttributes::VAR_ATTR_ENUMERATION {
                    quantity: Util::applyOption(
                        quantity_e,
                        &fnptr!(expToGeneratedBinding, metamodelica::Ref<Expression::NFExpression>),
                    )?,
                    min: Util::applyOption(
                        min_e,
                        &fnptr!(expToGeneratedBinding, metamodelica::Ref<Expression::NFExpression>),
                    )?,
                    max: Util::applyOption(
                        max_e,
                        &fnptr!(expToGeneratedBinding, metamodelica::Ref<Expression::NFExpression>),
                    )?,
                    start: Util::applyOption(
                        start_e,
                        &fnptr!(expToGeneratedBinding, metamodelica::Ref<Expression::NFExpression>),
                    )?,
                    fixed: Util::applyOption(
                        fixed_e,
                        &fnptr!(expToGeneratedBinding, metamodelica::Ref<Expression::NFExpression>),
                    )?,
                    binding: Util::applyOption(
                        binding_e,
                        &fnptr!(expToGeneratedBinding, metamodelica::Ref<Expression::NFExpression>),
                    )?,
                    isProtected: isProtected.clone(),
                    finalPrefix: finalPrefix.clone(),
                }),
                scalar_attributes,
            );
        }
        scalar_attributes = if (uniform) {
            List::fill((scalar_attributes).head().cloned()?, length)
        } else {
            metamodelica::Dangerous::listReverseInPlace(scalar_attributes)
        };
        Ok((scalar_attributes, uniform))
    }

    pub(crate) fn scalarize(
        mut attributes: metamodelica::Ref<VariableAttributes>,
        mut length: i32,
    ) -> Result<(metamodelica::List<metamodelica::Ref<VariableAttributes>>, bool)> {
        let mut scalar_attributes: metamodelica::List<metamodelica::Ref<VariableAttributes>> = metamodelica::nil();
        let mut uniform: bool;
        (scalar_attributes, uniform) = (match &*attributes {
            VAR_ATTR_REAL {
                binding: __attributes_binding,
                displayUnit: __attributes_displayUnit,
                distribution: __attributes_distribution,
                finalPrefix: __attributes_finalPrefix,
                fixed: __attributes_fixed,
                isProtected: __attributes_isProtected,
                max: __attributes_max,
                min: __attributes_min,
                nominal: __attributes_nominal,
                quantity: __attributes_quantity,
                start: __attributes_start,
                stateSelect: __attributes_stateSelect,
                tearingSelect: __attributes_tearingSelect,
                uncertainty: __attributes_uncertainty,
                unit: __attributes_unit,
            } => scalarizeReal(
                ExpressionIterator::fromExpOpt(Util::applyOption(
                    __attributes_quantity.clone(),
                    &move |__a0: metamodelica::Ref<Binding::NFBinding>| Binding::getTypedExp(&__a0),
                )?)?,
                ExpressionIterator::fromExpOpt(Util::applyOption(
                    __attributes_unit.clone(),
                    &move |__a0: metamodelica::Ref<Binding::NFBinding>| Binding::getTypedExp(&__a0),
                )?)?,
                ExpressionIterator::fromExpOpt(Util::applyOption(
                    __attributes_displayUnit.clone(),
                    &move |__a0: metamodelica::Ref<Binding::NFBinding>| Binding::getTypedExp(&__a0),
                )?)?,
                ExpressionIterator::fromExpOpt(Util::applyOption(
                    __attributes_min.clone(),
                    &move |__a0: metamodelica::Ref<Binding::NFBinding>| Binding::getTypedExp(&__a0),
                )?)?,
                ExpressionIterator::fromExpOpt(Util::applyOption(
                    __attributes_max.clone(),
                    &move |__a0: metamodelica::Ref<Binding::NFBinding>| Binding::getTypedExp(&__a0),
                )?)?,
                ExpressionIterator::fromExpOpt(Util::applyOption(
                    __attributes_start.clone(),
                    &move |__a0: metamodelica::Ref<Binding::NFBinding>| Binding::getTypedExp(&__a0),
                )?)?,
                ExpressionIterator::fromExpOpt(Util::applyOption(
                    __attributes_fixed.clone(),
                    &move |__a0: metamodelica::Ref<Binding::NFBinding>| Binding::getTypedExp(&__a0),
                )?)?,
                ExpressionIterator::fromExpOpt(Util::applyOption(
                    __attributes_nominal.clone(),
                    &move |__a0: metamodelica::Ref<Binding::NFBinding>| Binding::getTypedExp(&__a0),
                )?)?,
                __attributes_stateSelect.clone(),
                __attributes_tearingSelect.clone(),
                __attributes_uncertainty.clone(),
                __attributes_distribution.clone(),
                ExpressionIterator::fromExpOpt(Util::applyOption(
                    __attributes_binding.clone(),
                    &move |__a0: metamodelica::Ref<Binding::NFBinding>| Binding::getTypedExp(&__a0),
                )?)?,
                __attributes_isProtected.clone(),
                __attributes_finalPrefix.clone(),
                length,
            )?,
            VAR_ATTR_INT {
                binding: __attributes_binding,
                distribution: __attributes_distribution,
                finalPrefix: __attributes_finalPrefix,
                fixed: __attributes_fixed,
                isProtected: __attributes_isProtected,
                max: __attributes_max,
                min: __attributes_min,
                quantity: __attributes_quantity,
                start: __attributes_start,
                uncertainty: __attributes_uncertainty,
            } => scalarizeInt(
                ExpressionIterator::fromExpOpt(Util::applyOption(
                    __attributes_quantity.clone(),
                    &move |__a0: metamodelica::Ref<Binding::NFBinding>| Binding::getTypedExp(&__a0),
                )?)?,
                ExpressionIterator::fromExpOpt(Util::applyOption(
                    __attributes_min.clone(),
                    &move |__a0: metamodelica::Ref<Binding::NFBinding>| Binding::getTypedExp(&__a0),
                )?)?,
                ExpressionIterator::fromExpOpt(Util::applyOption(
                    __attributes_max.clone(),
                    &move |__a0: metamodelica::Ref<Binding::NFBinding>| Binding::getTypedExp(&__a0),
                )?)?,
                ExpressionIterator::fromExpOpt(Util::applyOption(
                    __attributes_start.clone(),
                    &move |__a0: metamodelica::Ref<Binding::NFBinding>| Binding::getTypedExp(&__a0),
                )?)?,
                ExpressionIterator::fromExpOpt(Util::applyOption(
                    __attributes_fixed.clone(),
                    &move |__a0: metamodelica::Ref<Binding::NFBinding>| Binding::getTypedExp(&__a0),
                )?)?,
                __attributes_uncertainty.clone(),
                __attributes_distribution.clone(),
                ExpressionIterator::fromExpOpt(Util::applyOption(
                    __attributes_binding.clone(),
                    &move |__a0: metamodelica::Ref<Binding::NFBinding>| Binding::getTypedExp(&__a0),
                )?)?,
                __attributes_isProtected.clone(),
                __attributes_finalPrefix.clone(),
                length,
            )?,
            VAR_ATTR_BOOL {
                binding: __attributes_binding,
                finalPrefix: __attributes_finalPrefix,
                fixed: __attributes_fixed,
                isProtected: __attributes_isProtected,
                quantity: __attributes_quantity,
                start: __attributes_start,
            } => scalarizeBool(
                ExpressionIterator::fromExpOpt(Util::applyOption(
                    __attributes_quantity.clone(),
                    &move |__a0: metamodelica::Ref<Binding::NFBinding>| Binding::getTypedExp(&__a0),
                )?)?,
                ExpressionIterator::fromExpOpt(Util::applyOption(
                    __attributes_start.clone(),
                    &move |__a0: metamodelica::Ref<Binding::NFBinding>| Binding::getTypedExp(&__a0),
                )?)?,
                ExpressionIterator::fromExpOpt(Util::applyOption(
                    __attributes_fixed.clone(),
                    &move |__a0: metamodelica::Ref<Binding::NFBinding>| Binding::getTypedExp(&__a0),
                )?)?,
                ExpressionIterator::fromExpOpt(Util::applyOption(
                    __attributes_binding.clone(),
                    &move |__a0: metamodelica::Ref<Binding::NFBinding>| Binding::getTypedExp(&__a0),
                )?)?,
                __attributes_isProtected.clone(),
                __attributes_finalPrefix.clone(),
                length,
            )?,
            VAR_ATTR_CLOCK {
                finalPrefix: __attributes_finalPrefix,
                isProtected: __attributes_isProtected,
            } => scalarizeClock(
                __attributes_isProtected.clone(),
                __attributes_finalPrefix.clone(),
                length,
            ),
            VAR_ATTR_STRING {
                binding: __attributes_binding,
                finalPrefix: __attributes_finalPrefix,
                fixed: __attributes_fixed,
                isProtected: __attributes_isProtected,
                quantity: __attributes_quantity,
                start: __attributes_start,
            } => scalarizeString(
                ExpressionIterator::fromExpOpt(Util::applyOption(
                    __attributes_quantity.clone(),
                    &move |__a0: metamodelica::Ref<Binding::NFBinding>| Binding::getTypedExp(&__a0),
                )?)?,
                ExpressionIterator::fromExpOpt(Util::applyOption(
                    __attributes_start.clone(),
                    &move |__a0: metamodelica::Ref<Binding::NFBinding>| Binding::getTypedExp(&__a0),
                )?)?,
                ExpressionIterator::fromExpOpt(Util::applyOption(
                    __attributes_fixed.clone(),
                    &move |__a0: metamodelica::Ref<Binding::NFBinding>| Binding::getTypedExp(&__a0),
                )?)?,
                ExpressionIterator::fromExpOpt(Util::applyOption(
                    __attributes_binding.clone(),
                    &move |__a0: metamodelica::Ref<Binding::NFBinding>| Binding::getTypedExp(&__a0),
                )?)?,
                __attributes_isProtected.clone(),
                __attributes_finalPrefix.clone(),
                length,
            )?,
            VAR_ATTR_ENUMERATION {
                binding: __attributes_binding,
                finalPrefix: __attributes_finalPrefix,
                fixed: __attributes_fixed,
                isProtected: __attributes_isProtected,
                max: __attributes_max,
                min: __attributes_min,
                quantity: __attributes_quantity,
                start: __attributes_start,
            } => scalarizeEnumeration(
                ExpressionIterator::fromExpOpt(Util::applyOption(
                    __attributes_quantity.clone(),
                    &move |__a0: metamodelica::Ref<Binding::NFBinding>| Binding::getTypedExp(&__a0),
                )?)?,
                ExpressionIterator::fromExpOpt(Util::applyOption(
                    __attributes_min.clone(),
                    &move |__a0: metamodelica::Ref<Binding::NFBinding>| Binding::getTypedExp(&__a0),
                )?)?,
                ExpressionIterator::fromExpOpt(Util::applyOption(
                    __attributes_max.clone(),
                    &move |__a0: metamodelica::Ref<Binding::NFBinding>| Binding::getTypedExp(&__a0),
                )?)?,
                ExpressionIterator::fromExpOpt(Util::applyOption(
                    __attributes_start.clone(),
                    &move |__a0: metamodelica::Ref<Binding::NFBinding>| Binding::getTypedExp(&__a0),
                )?)?,
                ExpressionIterator::fromExpOpt(Util::applyOption(
                    __attributes_fixed.clone(),
                    &move |__a0: metamodelica::Ref<Binding::NFBinding>| Binding::getTypedExp(&__a0),
                )?)?,
                ExpressionIterator::fromExpOpt(Util::applyOption(
                    __attributes_binding.clone(),
                    &move |__a0: metamodelica::Ref<Binding::NFBinding>| Binding::getTypedExp(&__a0),
                )?)?,
                __attributes_isProtected.clone(),
                __attributes_finalPrefix.clone(),
                length,
            )?,
            VAR_ATTR_RECORD { .. } => (list![attributes], false),
            _ => {
                Error::terminate(
                    {
                        let mut __mm_s = String::new();
                        __mm_s.push_str(&*literal!("NFBackendExtension.VariableAttributes.scalarize"));
                        __mm_s.push_str(&*literal!("failed. Not yet handled: "));
                        __mm_s.push_str(&*toString(&attributes)?);
                        ArcStr::from(__mm_s)
                    },
                    &(metamodelica::sourceInfo!("NFFrontEnd/NFBackendExtension.mo")),
                )?;
                return Err("fail");
            }
        });
        Ok((scalar_attributes, uniform))
    }

    pub(crate) fn elemType(
        mut attr: &metamodelica::Ref<VariableAttributes>,
    ) -> Result<metamodelica::Ref<Type::NFType>> {
        let mut ty: metamodelica::Ref<Type::NFType>;
        ty = (match &**attr {
            VAR_ATTR_REAL { .. } => crate::NFType::interned_REAL(),
            VAR_ATTR_INT { .. } => crate::NFType::interned_INTEGER(),
            VAR_ATTR_BOOL { .. } => crate::NFType::interned_BOOLEAN(),
            VAR_ATTR_CLOCK { .. } => crate::NFType::interned_CLOCK(),
            VAR_ATTR_STRING { .. } => crate::NFType::interned_STRING(),
            _ => {
                Error::terminate(
                    {
                        let mut __mm_s = String::new();
                        __mm_s.push_str(&*literal!("NFBackendExtension.VariableAttributes.elemType"));
                        __mm_s.push_str(&*literal!(" cannot create type from attributes: "));
                        __mm_s.push_str(&*toString(attr)?);
                        ArcStr::from(__mm_s)
                    },
                    &(metamodelica::sourceInfo!("NFFrontEnd/NFBackendExtension.mo")),
                )?;
                return Err("fail");
            }
        });
        Ok(ty)
    }

    pub(crate) fn attributesToString(
        mut tpl_list: &metamodelica::List<(ArcStr, Option<metamodelica::Ref<Expression::NFExpression>>)>,
        mut stateSelect: Option<StateSelect>,
        mut tearingSelect: Option<TearingSelect>,
    ) -> Result<ArcStr> {
        let mut r#str: ArcStr = literal!("");
        let mut buffer: metamodelica::List<ArcStr> = metamodelica::nil();
        let mut name: ArcStr;
        for mut tpl in &**tpl_list {
            buffer = attributeToString(&(tpl.clone()), buffer)?;
        }
        buffer = stateSelectStringBuffer(stateSelect, buffer)?;
        buffer = tearingSelectStringBuffer(tearingSelect, buffer)?;
        buffer = buffer.reverse();
        if !((buffer).is_empty()) {
            let (__pa0, __pa1) = ::match_deref::match_deref! { match &(buffer) {
                Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
                _ => return Err("pattern mismatch"),
            } };
            name = metamodelica::Own::own(__pa0);
            buffer = metamodelica::Own::own(__pa1);
            r#str = {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*r#str);
                __mm_s.push_str(&*name);
                ArcStr::from(__mm_s)
            };
            for mut name in &*buffer {
                let mut name = name.clone();
                r#str = {
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*r#str);
                    __mm_s.push_str(&*literal!(", "));
                    __mm_s.push_str(&*name);
                    ArcStr::from(__mm_s)
                };
            }
        }
        Ok(r#str)
    }

    pub(crate) fn attributeToString(
        mut tpl: &(ArcStr, Option<metamodelica::Ref<Expression::NFExpression>>),
        mut buffer: metamodelica::List<ArcStr>,
    ) -> Result<metamodelica::List<ArcStr>> {
        let mut buffer: metamodelica::List<ArcStr> = buffer;
        let mut name: ArcStr;
        let mut optAttr: Option<metamodelica::Ref<Expression::NFExpression>>;
        let mut attr: metamodelica::Ref<Expression::NFExpression>;
        (name, optAttr) = tpl.clone();
        if (optAttr).is_some() {
            let __pa0 = ::match_deref::match_deref! { match &(optAttr) {
                Some(__pa0) => __pa0.clone(),
                _ => return Err("pattern mismatch"),
            } };
            attr = metamodelica::Own::own(__pa0);
            buffer = metamodelica::cons(
                {
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*name);
                    __mm_s.push_str(&*literal!(" = "));
                    __mm_s.push_str(&*Expression::toString(attr)?);
                    ArcStr::from(__mm_s)
                },
                buffer,
            );
        }
        Ok(buffer)
    }

    pub fn stateSelectString(mut stateSelect: StateSelect) -> Result<ArcStr> {
        let mut r#str: ArcStr;
        r#str = (match stateSelect {
            StateSelect::NEVER => literal!("StateSelect = never"),
            StateSelect::AVOID => literal!("StateSelect = avoid"),
            StateSelect::DEFAULT => literal!("StateSelect = default"),
            StateSelect::PREFER => literal!("StateSelect = prefer"),
            StateSelect::ALWAYS => literal!("StateSelect = always"),
        });
        Ok(r#str)
    }

    pub fn tearingSelectString(mut tearingSelect: TearingSelect) -> Result<ArcStr> {
        let mut r#str: ArcStr;
        r#str = (match tearingSelect {
            TearingSelect::NEVER => literal!("TearingSelect = never"),
            TearingSelect::AVOID => literal!("TearingSelect = avoid"),
            TearingSelect::DEFAULT => literal!("TearingSelect = default"),
            TearingSelect::PREFER => literal!("TearingSelect = prefer"),
            TearingSelect::ALWAYS => literal!("TearingSelect = always"),
        });
        Ok(r#str)
    }

    pub(crate) fn stateSelectStringBuffer(
        mut optStateSelect: Option<StateSelect>,
        mut buffer: metamodelica::List<ArcStr>,
    ) -> Result<metamodelica::List<ArcStr>> {
        let mut buffer: metamodelica::List<ArcStr> = buffer;
        let mut stateSelect: StateSelect;
        if (optStateSelect).is_some() {
            let __pa0 = ::match_deref::match_deref! { match &(optStateSelect) {
                Some(__pa0) => __pa0.clone(),
                _ => return Err("pattern mismatch"),
            } };
            stateSelect = metamodelica::Own::own(__pa0);
            buffer = metamodelica::cons(stateSelectString(stateSelect)?, buffer);
        }
        Ok(buffer)
    }

    pub(crate) fn tearingSelectStringBuffer(
        mut optTearingSelect: Option<TearingSelect>,
        mut buffer: metamodelica::List<ArcStr>,
    ) -> Result<metamodelica::List<ArcStr>> {
        let mut buffer: metamodelica::List<ArcStr> = buffer;
        let mut tearingSelect: TearingSelect;
        if (optTearingSelect).is_some() {
            let __pa0 = ::match_deref::match_deref! { match &(optTearingSelect) {
                Some(__pa0) => __pa0.clone(),
                _ => return Err("pattern mismatch"),
            } };
            tearingSelect = metamodelica::Own::own(__pa0);
            buffer = metamodelica::cons(tearingSelectString(tearingSelect)?, buffer);
        }
        Ok(buffer)
    }

    fn createReal(
        mut attrs: &metamodelica::List<(ArcStr, metamodelica::Ref<Binding::NFBinding>)>,
        mut isFinal: bool,
        mut comment: &metamodelica::Ref<SCode::Comment>,
    ) -> Result<metamodelica::Ref<VariableAttributes>> {
        let mut attributes: metamodelica::Ref<VariableAttributes>;
        let mut name: ArcStr;
        let mut b: metamodelica::Ref<Binding::NFBinding>;
        let mut quantity: Option<metamodelica::Ref<Binding::NFBinding>> = None;
        let mut unit: Option<metamodelica::Ref<Binding::NFBinding>> = None;
        let mut displayUnit: Option<metamodelica::Ref<Binding::NFBinding>> = None;
        let mut min: Option<metamodelica::Ref<Binding::NFBinding>> = None;
        let mut max: Option<metamodelica::Ref<Binding::NFBinding>> = None;
        let mut start: Option<metamodelica::Ref<Binding::NFBinding>> = None;
        let mut fixed: Option<metamodelica::Ref<Binding::NFBinding>> = None;
        let mut nominal: Option<metamodelica::Ref<Binding::NFBinding>> = None;
        let mut state_select: Option<StateSelect> = None;
        let mut tearing_select: Option<TearingSelect> = None;
        for mut attr in &**attrs {
            (name, b) = attr.clone();
            let () = (::match_deref::match_deref! { match &(name.clone()) {
                Deref @ "displayUnit" => {
                    displayUnit = createAttribute(b);
                    ()
                },
                Deref @ "fixed" => {
                    fixed = createAttribute(b);
                    ()
                },
                Deref @ "max" => {
                    max = createAttribute(b);
                    ()
                },
                Deref @ "min" => {
                    min = createAttribute(b);
                    ()
                },
                Deref @ "nominal" => {
                    nominal = createAttribute(b);
                    ()
                },
                Deref @ "quantity" => {
                    quantity = createAttribute(b);
                    ()
                },
                Deref @ "start" => {
                    start = createAttribute(b);
                    ()
                },
                Deref @ "stateSelect" => {
                    state_select = createStateSelect(&b)?;
                    ()
                },
                Deref @ "unbounded" => (),
                Deref @ "unit" => {
                    unit = createAttribute(b);
                    ()
                },
                _ => {
                    Error::terminate({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NFBackendExtension.VariableAttributes.createReal")); __mm_s.push_str(&*literal!(" got unknown type attribute ")); __mm_s.push_str(&*name); ArcStr::from(__mm_s) }, &(metamodelica::sourceInfo!("NFFrontEnd/NFBackendExtension.mo")))?;
                    return Err("fail")
                },
                _ => unreachable!("match_deref! exhaustiveness placeholder"),
            } });
        }
        tearing_select = createTearingSelect(comment)?;
        attributes = metamodelica::Ref::new(VariableAttributes::VAR_ATTR_REAL {
            quantity: quantity,
            unit: unit,
            displayUnit: displayUnit,
            min: min,
            max: max,
            start: start,
            fixed: fixed,
            nominal: nominal,
            stateSelect: state_select,
            tearingSelect: tearing_select,
            uncertainty: None,
            distribution: None,
            binding: None,
            isProtected: None,
            finalPrefix: Some(isFinal),
        });
        Ok(attributes)
    }

    fn createInt(
        mut attrs: &metamodelica::List<(ArcStr, metamodelica::Ref<Binding::NFBinding>)>,
        mut isFinal: bool,
    ) -> Result<metamodelica::Ref<VariableAttributes>> {
        let mut attributes: metamodelica::Ref<VariableAttributes>;
        let mut name: ArcStr;
        let mut b: metamodelica::Ref<Binding::NFBinding>;
        let mut quantity: Option<metamodelica::Ref<Binding::NFBinding>> = None;
        let mut min: Option<metamodelica::Ref<Binding::NFBinding>> = None;
        let mut max: Option<metamodelica::Ref<Binding::NFBinding>> = None;
        let mut start: Option<metamodelica::Ref<Binding::NFBinding>> = None;
        let mut fixed: Option<metamodelica::Ref<Binding::NFBinding>> = None;
        if (attrs).is_empty() && !(isFinal) {
            attributes = EMPTY_VAR_ATTR_INT().clone();
        } else {
            for mut attr in &**attrs {
                (name, b) = attr.clone();
                let () = (::match_deref::match_deref! { match &(name.clone()) {
                    Deref @ "quantity" => {
                        quantity = createAttribute(b);
                        ()
                    },
                    Deref @ "min" => {
                        min = createAttribute(b);
                        ()
                    },
                    Deref @ "max" => {
                        max = createAttribute(b);
                        ()
                    },
                    Deref @ "start" => {
                        start = createAttribute(b);
                        ()
                    },
                    Deref @ "fixed" => {
                        fixed = createAttribute(b);
                        ()
                    },
                    _ => {
                        Error::terminate({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NFBackendExtension.VariableAttributes.createInt")); __mm_s.push_str(&*literal!(" got unknown type attribute ")); __mm_s.push_str(&*name); ArcStr::from(__mm_s) }, &(metamodelica::sourceInfo!("NFFrontEnd/NFBackendExtension.mo")))?;
                        return Err("fail")
                    },
                    _ => unreachable!("match_deref! exhaustiveness placeholder"),
                } });
            }
            attributes = metamodelica::Ref::new(VariableAttributes::VAR_ATTR_INT {
                quantity: quantity,
                min: min,
                max: max,
                start: start,
                fixed: fixed,
                uncertainty: None,
                distribution: None,
                binding: None,
                isProtected: None,
                finalPrefix: Some(isFinal),
            });
        }
        Ok(attributes)
    }

    fn createBool(
        mut attrs: &metamodelica::List<(ArcStr, metamodelica::Ref<Binding::NFBinding>)>,
        mut isFinal: bool,
    ) -> Result<metamodelica::Ref<VariableAttributes>> {
        let mut attributes: metamodelica::Ref<VariableAttributes>;
        let mut name: ArcStr;
        let mut b: metamodelica::Ref<Binding::NFBinding>;
        let mut quantity: Option<metamodelica::Ref<Binding::NFBinding>> = None;
        let mut start: Option<metamodelica::Ref<Binding::NFBinding>> = None;
        let mut fixed: Option<metamodelica::Ref<Binding::NFBinding>> = None;
        if (attrs).is_empty() && !(isFinal) {
            attributes = EMPTY_VAR_ATTR_BOOL().clone();
        } else {
            for mut attr in &**attrs {
                (name, b) = attr.clone();
                let () = (::match_deref::match_deref! { match &(name.clone()) {
                    Deref @ "quantity" => {
                        quantity = createAttribute(b);
                        ()
                    },
                    Deref @ "start" => {
                        start = createAttribute(b);
                        ()
                    },
                    Deref @ "fixed" => {
                        fixed = createAttribute(b);
                        ()
                    },
                    _ => {
                        Error::terminate({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NFBackendExtension.VariableAttributes.createBool")); __mm_s.push_str(&*literal!(" got unknown type attribute ")); __mm_s.push_str(&*name); ArcStr::from(__mm_s) }, &(metamodelica::sourceInfo!("NFFrontEnd/NFBackendExtension.mo")))?;
                        return Err("fail")
                    },
                    _ => unreachable!("match_deref! exhaustiveness placeholder"),
                } });
            }
            attributes = metamodelica::Ref::new(VariableAttributes::VAR_ATTR_BOOL {
                quantity: quantity,
                start: start,
                fixed: fixed,
                binding: None,
                isProtected: None,
                finalPrefix: Some(isFinal),
            });
        }
        Ok(attributes)
    }

    fn createString(
        mut attrs: &metamodelica::List<(ArcStr, metamodelica::Ref<Binding::NFBinding>)>,
        mut isFinal: bool,
    ) -> Result<metamodelica::Ref<VariableAttributes>> {
        let mut attributes: metamodelica::Ref<VariableAttributes>;
        let mut name: ArcStr;
        let mut b: metamodelica::Ref<Binding::NFBinding>;
        let mut quantity: Option<metamodelica::Ref<Binding::NFBinding>> = None;
        let mut start: Option<metamodelica::Ref<Binding::NFBinding>> = None;
        let mut fixed: Option<metamodelica::Ref<Binding::NFBinding>> = None;
        if (attrs).is_empty() && !(isFinal) {
            attributes = EMPTY_VAR_ATTR_STRING().clone();
        } else {
            for mut attr in &**attrs {
                (name, b) = attr.clone();
                let () = (::match_deref::match_deref! { match &(name.clone()) {
                    Deref @ "quantity" => {
                        quantity = createAttribute(b);
                        ()
                    },
                    Deref @ "start" => {
                        start = createAttribute(b);
                        ()
                    },
                    Deref @ "fixed" => {
                        fixed = createAttribute(b);
                        ()
                    },
                    _ => {
                        Error::terminate({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NFBackendExtension.VariableAttributes.createString")); __mm_s.push_str(&*literal!(" got unknown type attribute ")); __mm_s.push_str(&*name); ArcStr::from(__mm_s) }, &(metamodelica::sourceInfo!("NFFrontEnd/NFBackendExtension.mo")))?;
                        return Err("fail")
                    },
                    _ => unreachable!("match_deref! exhaustiveness placeholder"),
                } });
            }
            attributes = metamodelica::Ref::new(VariableAttributes::VAR_ATTR_STRING {
                quantity: quantity,
                start: start,
                fixed: fixed,
                binding: None,
                isProtected: None,
                finalPrefix: Some(isFinal),
            });
        }
        Ok(attributes)
    }

    fn createEnum(
        mut attrs: &metamodelica::List<(ArcStr, metamodelica::Ref<Binding::NFBinding>)>,
        mut isFinal: bool,
    ) -> Result<metamodelica::Ref<VariableAttributes>> {
        let mut attributes: metamodelica::Ref<VariableAttributes>;
        let mut name: ArcStr;
        let mut b: metamodelica::Ref<Binding::NFBinding>;
        let mut quantity: Option<metamodelica::Ref<Binding::NFBinding>> = None;
        let mut min: Option<metamodelica::Ref<Binding::NFBinding>> = None;
        let mut max: Option<metamodelica::Ref<Binding::NFBinding>> = None;
        let mut start: Option<metamodelica::Ref<Binding::NFBinding>> = None;
        let mut fixed: Option<metamodelica::Ref<Binding::NFBinding>> = None;
        if (attrs).is_empty() && !(isFinal) {
            attributes = EMPTY_VAR_ATTR_REAL().clone();
        } else {
            for mut attr in &**attrs {
                (name, b) = attr.clone();
                let () = (::match_deref::match_deref! { match &(name.clone()) {
                    Deref @ "fixed" => {
                        fixed = createAttribute(b);
                        ()
                    },
                    Deref @ "max" => {
                        max = createAttribute(b);
                        ()
                    },
                    Deref @ "min" => {
                        min = createAttribute(b);
                        ()
                    },
                    Deref @ "quantity" => {
                        quantity = createAttribute(b);
                        ()
                    },
                    Deref @ "start" => {
                        start = createAttribute(b);
                        ()
                    },
                    _ => {
                        Error::terminate({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NFBackendExtension.VariableAttributes.createEnum")); __mm_s.push_str(&*literal!(" got unknown type attribute ")); __mm_s.push_str(&*name); ArcStr::from(__mm_s) }, &(metamodelica::sourceInfo!("NFFrontEnd/NFBackendExtension.mo")))?;
                        return Err("fail")
                    },
                    _ => unreachable!("match_deref! exhaustiveness placeholder"),
                } });
            }
            attributes = metamodelica::Ref::new(VariableAttributes::VAR_ATTR_ENUMERATION {
                quantity: quantity,
                min: min,
                max: max,
                start: start,
                fixed: fixed,
                binding: None,
                isProtected: None,
                finalPrefix: Some(isFinal),
            });
        }
        Ok(attributes)
    }

    fn createClock(mut isFinal: bool) -> metamodelica::Ref<VariableAttributes> {
        let mut attributes: metamodelica::Ref<VariableAttributes> =
            metamodelica::Ref::new(VariableAttributes::VAR_ATTR_CLOCK {
                isProtected: None,
                finalPrefix: Some(isFinal),
            });
        attributes
    }

    fn createRecord(
        mut attrs: &metamodelica::List<(ArcStr, metamodelica::Ref<Binding::NFBinding>)>,
        mut indexMap: metamodelica::Ref<UnorderedMap::UnorderedMap<ArcStr, i32>>,
        mut children: &metamodelica::List<metamodelica::Ref<Variable::NFVariable>>,
        mut isFinal: bool,
    ) -> Result<metamodelica::Ref<VariableAttributes>> {
        let mut attributes: metamodelica::Ref<VariableAttributes>;
        let mut childrenAttr: metamodelica::Array<metamodelica::Ref<VariableAttributes>> =
            arrayCreate(((children).len() as i32), EMPTY_VAR_ATTR_REAL().clone());
        let mut index: i32;
        for mut var in &**children {
            let () = (match UnorderedMap::get(ComponentRef::firstName(&var.name, false)?, indexMap.clone())? {
                Some(mut __esc_index) => {
                    index = __esc_index.clone();
                    {
                        let __cell0 = create(
                            &var.typeAttributes,
                            &var.ty,
                            &var.attributes,
                            &var.children,
                            &var.comment,
                        )?;
                        let __idx0 = index;
                        *metamodelica::index_mut_checked(&mut childrenAttr.clone().borrow_mut(), __idx0)? = __cell0;
                    }
                    ()
                }
                _ => (),
            });
        }
        attributes = metamodelica::Ref::new(VariableAttributes::VAR_ATTR_RECORD {
            indexMap: indexMap,
            childrenAttr: childrenAttr.clone(),
        });
        Ok(attributes)
    }

    fn createAttribute(
        mut binding: metamodelica::Ref<Binding::NFBinding>,
    ) -> Option<metamodelica::Ref<Binding::NFBinding>> {
        let mut attribute: Option<metamodelica::Ref<Binding::NFBinding>> = Some(binding.clone());
        attribute
    }

    fn createStateSelect(mut binding: &metamodelica::Ref<Binding::NFBinding>) -> Result<Option<StateSelect>> {
        let mut stateSelect: Option<StateSelect>;
        let mut exp: metamodelica::Ref<Expression::NFExpression> = Binding::getTypedExp(binding)?;
        let mut name: ArcStr;
        name = getStateSelectName(exp)?;
        stateSelect = Some(lookupStateSelectMember(&name)?);
        Ok(stateSelect)
    }

    fn getStateSelectName(mut exp: metamodelica::Ref<Expression::NFExpression>) -> Result<ArcStr> {
        '__tco: loop {
            let mut arg: metamodelica::Ref<Expression::NFExpression>;
            let mut node: metamodelica::Ref<InstNode::InstNode>;
            let mut call: metamodelica::Ref<Call::NFCall>;
            let mut rest: metamodelica::List<metamodelica::Ref<Expression::NFExpression>>;
            ::match_deref::match_deref! { match &(exp.clone()) {
                Deref @ Expression::ENUM_LITERAL { name: __exp_name, .. } => return Ok(__exp_name.clone()),
                Deref @ Expression::CREF { cref: Deref @ ComponentRef::CREF { .. }, .. } => return Ok(ComponentRef::nodeName(var_field!((*exp).cref, Expression::NFExpression::CREF))?),
                Deref @ Expression::CALL { call: __esc_call @ Deref @ Call::TYPED_ARRAY_CONSTRUCTOR { .. } } => {
                    call = (*__esc_call).clone();
                    { exp = var_field!((*call).exp, Call::NFCall::TYPED_ARRAY_CONSTRUCTOR).clone(); continue '__tco; }
                },
                Deref @ Expression::CALL { call: call @ Deref @ Call::TYPED_CALL { arguments: Deref @ metamodelica::ListNode::Cons { head: __esc_arg, tail: _ }, .. } } if (metamodelica::stringEq(&(AbsynUtil::pathString(Function::nameConsiderBuiltin(var_field!((**call).r#fn, Call::NFCall::TYPED_CALL)), literal!("."), true, false)?), &(literal!("fill")))) => {
                    arg = (*__esc_arg).clone();
                    { exp = arg.clone(); continue '__tco; }
                },
                Deref @ Expression::ARRAY { .. } => {
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(var_field!((*exp).elements, Expression::NFExpression::ARRAY).clone().borrow().iter().cloned().collect::<metamodelica::List<_>>()) {
                        Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    arg = metamodelica::Own::own(__pa0);
                    rest = metamodelica::Own::own(__pa1);
                    if !((rest).is_empty() || List::all(&rest, &({ let __pe_b1 = arg.clone(); move |__pe_a0| Expression::isEqual(__pe_a0, __pe_b1.clone()) }))?) {
                        Error::terminate({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NFBackendExtension.VariableAttributes.getStateSelectName")); __mm_s.push_str(&*literal!(" cannot handle array StateSelect with different values yet:")); __mm_s.push_str(&*Expression::toString(exp)?); ArcStr::from(__mm_s) }, &(metamodelica::sourceInfo!("NFFrontEnd/NFBackendExtension.mo")))?;
                        return Err("fail");
                    }
                    { exp = arg; continue '__tco; }
                },
                _ => {
                    Error::terminate({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NFBackendExtension.VariableAttributes.getStateSelectName")); __mm_s.push_str(&*literal!(" got invalid StateSelect expression ")); __mm_s.push_str(&*Expression::toString(exp)?); ArcStr::from(__mm_s) }, &(metamodelica::sourceInfo!("NFFrontEnd/NFBackendExtension.mo")))?;
                    return Ok(return Err("fail"))
                },
                _ => return Err("match: no arm matched"),
            } }
        }
    }

    fn lookupStateSelectMember(mut name: &ArcStr) -> Result<StateSelect> {
        let mut stateSelect: StateSelect;
        stateSelect = (::match_deref::match_deref! { match &(name.clone()) {
            Deref @ "never" => StateSelect::NEVER.clone(),
            Deref @ "avoid" => StateSelect::AVOID.clone(),
            Deref @ "default" => StateSelect::DEFAULT.clone(),
            Deref @ "prefer" => StateSelect::PREFER.clone(),
            Deref @ "always" => StateSelect::ALWAYS.clone(),
            _ => {
                Error::terminate({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("NFBackendExtension.VariableAttributes.lookupStateSelectMember")); __mm_s.push_str(&*literal!(" got unknown StateSelect literal ")); __mm_s.push_str(&*name); ArcStr::from(__mm_s) }, &(metamodelica::sourceInfo!("NFFrontEnd/NFBackendExtension.mo")))?;
                return Err("fail")
            },
            _ => unreachable!("match_deref! exhaustiveness placeholder"),
        } });
        Ok(stateSelect)
    }

    fn createTearingSelect(mut cmt: &metamodelica::Ref<SCode::Comment>) -> Result<Option<TearingSelect>> {
        let mut tearingSelect: Option<TearingSelect> = None;
        let mut opt_anno: Option<metamodelica::Ref<SCode::Annotation>>;
        let mut anno: metamodelica::Ref<SCode::Annotation>;
        let mut r#mod: metamodelica::Ref<SCode::Mod>;
        let mut opt_val: Option<metamodelica::Ref<Absyn::Exp>>;
        let mut val: metamodelica::Ref<Absyn::Exp>;
        let mut name: ArcStr;
        let mut info: SourceInfo;
        opt_anno = SCodeUtil::commentAnnotation(cmt);
        if (opt_anno).is_none() {
            return Ok(tearingSelect);
        }
        let __pa0 = ::match_deref::match_deref! { match &(opt_anno) {
            Some(__pa0) => __pa0.clone(),
            _ => return Err("pattern mismatch"),
        } };
        anno = metamodelica::Own::own(__pa0);
        r#mod = SCodeUtil::lookupAnnotation(&anno, &(literal!("__OpenModelica_tearingSelect")));
        if SCodeUtil::isEmptyMod(&r#mod) {
            r#mod = SCodeUtil::lookupAnnotation(&anno, &(literal!("tearingSelect")));
            if !(SCodeUtil::isEmptyMod(&r#mod)) {
                Error::addSourceMessage(
                    &(Error::DEPRECATED_EXPRESSION.clone()),
                    list![literal!("tearingSelect"), literal!("__OpenModelica_tearingSelect")],
                    &(SCodeUtil::getModifierInfo(&r#mod)),
                )?;
            }
        }
        opt_val = SCodeUtil::getModifierBinding(&r#mod);
        if (opt_val).is_none() {
            return Ok(tearingSelect);
        }
        let __pa1 = ::match_deref::match_deref! { match &(opt_val) {
            Some(__pa1) => __pa1.clone(),
            _ => return Err("pattern mismatch"),
        } };
        val = metamodelica::Own::own(__pa1);
        info = SCodeUtil::getModifierInfo(&r#mod);
        name = getTearingSelectName(&val, &info)?;
        tearingSelect = lookupTearingSelectMember(&name);
        if (tearingSelect).is_none() {
            Error::addSourceMessage(
                &(Error::UNKNOWN_ANNOTATION_VALUE.clone()),
                list![Dump::printExpStr(val)?, literal!("__OpenModelica_tearingSelect")],
                &info,
            )?;
        }
        Ok(tearingSelect)
    }

    fn getTearingSelectName(mut exp: &metamodelica::Ref<Absyn::Exp>, mut info: &SourceInfo) -> Result<ArcStr> {
        let mut name: ArcStr;
        name = (::match_deref::match_deref! { match exp {
            Deref @ Absyn::Exp::CREF { componentRef: Deref @ Absyn::ComponentRef::CREF_QUAL { name: Deref @ "TearingSelect", subscripts: Deref @ metamodelica::ListNode::Nil, componentRef: Deref @ Absyn::ComponentRef::CREF_IDENT { name: __esc_name, subscripts: Deref @ metamodelica::ListNode::Nil } } } => {
                name = (*__esc_name).clone();
                name.clone()
            },
            Deref @ Absyn::Exp::CREF { componentRef: Deref @ Absyn::ComponentRef::CREF_IDENT { name: __esc_name, subscripts: Deref @ metamodelica::ListNode::Nil } } => {
                name = (*__esc_name).clone();
                Error::addSourceMessage(&(Error::DEPRECATED_EXPRESSION.clone()), list![name.clone(), { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("TearingSelect.")); __mm_s.push_str(&*name); ArcStr::from(__mm_s) }], info)?;
                literal!("")
            },
            _ => literal!(""),
            _ => unreachable!("match_deref! exhaustiveness placeholder"),
        } });
        Ok(name)
    }

    fn lookupTearingSelectMember(mut name: &ArcStr) -> Option<TearingSelect> {
        let mut tearingSelect: Option<TearingSelect>;
        tearingSelect = (::match_deref::match_deref! { match &(name.clone()) {
            Deref @ "never" => Some(TearingSelect::NEVER.clone()),
            Deref @ "avoid" => Some(TearingSelect::AVOID.clone()),
            Deref @ "default" => Some(TearingSelect::DEFAULT.clone()),
            Deref @ "prefer" => Some(TearingSelect::PREFER.clone()),
            Deref @ "always" => Some(TearingSelect::ALWAYS.clone()),
            _ => None,
            _ => unreachable!("match_deref! exhaustiveness placeholder"),
        } });
        tearingSelect
    }

    fn expToGeneratedBinding(
        mut exp: metamodelica::Ref<Expression::NFExpression>,
    ) -> metamodelica::Ref<Binding::NFBinding> {
        let mut binding: metamodelica::Ref<Binding::NFBinding> = Binding::makeFlat(
            exp.clone(),
            Variability::CONSTANT.clone(),
            Source::GENERATED.clone(),
            Binding::NO_CONFIDENCE.clone(),
        );
        binding
    }
}

thread_local! { static __EMPTY_VAR_ATTR_REAL_TLS: metamodelica::Ref<VariableAttributes::VariableAttributes> = metamodelica::Ref::new(VariableAttributes::VariableAttributes::VAR_ATTR_REAL { quantity: None, unit: None, displayUnit: None, min: None, max: None, start: None, fixed: None, nominal: None, stateSelect: None, tearingSelect: None, uncertainty: None, distribution: None, binding: None, isProtected: None, finalPrefix: None }); }
pub fn EMPTY_VAR_ATTR_REAL() -> metamodelica::Ref<VariableAttributes::VariableAttributes> {
    __EMPTY_VAR_ATTR_REAL_TLS.with(|__t| __t.clone())
}

thread_local! { static __EMPTY_VAR_ATTR_INT_TLS: metamodelica::Ref<VariableAttributes::VariableAttributes> = metamodelica::Ref::new(VariableAttributes::VariableAttributes::VAR_ATTR_INT { quantity: None, min: None, max: None, start: None, fixed: None, uncertainty: None, distribution: None, binding: None, isProtected: None, finalPrefix: None }); }
pub(crate) fn EMPTY_VAR_ATTR_INT() -> metamodelica::Ref<VariableAttributes::VariableAttributes> {
    __EMPTY_VAR_ATTR_INT_TLS.with(|__t| __t.clone())
}

thread_local! { static __EMPTY_VAR_ATTR_BOOL_TLS: metamodelica::Ref<VariableAttributes::VariableAttributes> = metamodelica::Ref::new(VariableAttributes::VariableAttributes::VAR_ATTR_BOOL { quantity: None, start: None, fixed: None, binding: None, isProtected: None, finalPrefix: None }); }
pub(crate) fn EMPTY_VAR_ATTR_BOOL() -> metamodelica::Ref<VariableAttributes::VariableAttributes> {
    __EMPTY_VAR_ATTR_BOOL_TLS.with(|__t| __t.clone())
}

thread_local! { static __EMPTY_VAR_ATTR_CLOCK_TLS: metamodelica::Ref<VariableAttributes::VariableAttributes> = metamodelica::Ref::new(VariableAttributes::VariableAttributes::VAR_ATTR_CLOCK { isProtected: None, finalPrefix: None }); }
pub(crate) fn EMPTY_VAR_ATTR_CLOCK() -> metamodelica::Ref<VariableAttributes::VariableAttributes> {
    __EMPTY_VAR_ATTR_CLOCK_TLS.with(|__t| __t.clone())
}

thread_local! { static __EMPTY_VAR_ATTR_STRING_TLS: metamodelica::Ref<VariableAttributes::VariableAttributes> = metamodelica::Ref::new(VariableAttributes::VariableAttributes::VAR_ATTR_STRING { quantity: None, start: None, fixed: None, binding: None, isProtected: None, finalPrefix: None }); }
pub(crate) fn EMPTY_VAR_ATTR_STRING() -> metamodelica::Ref<VariableAttributes::VariableAttributes> {
    __EMPTY_VAR_ATTR_STRING_TLS.with(|__t| __t.clone())
}

thread_local! { static __EMPTY_VAR_ATTR_ENUMERATION_TLS: metamodelica::Ref<VariableAttributes::VariableAttributes> = metamodelica::Ref::new(VariableAttributes::VariableAttributes::VAR_ATTR_ENUMERATION { quantity: None, min: None, max: None, start: None, fixed: None, binding: None, isProtected: None, finalPrefix: None }); }
pub(crate) fn EMPTY_VAR_ATTR_ENUMERATION() -> metamodelica::Ref<VariableAttributes::VariableAttributes> {
    __EMPTY_VAR_ATTR_ENUMERATION_TLS.with(|__t| __t.clone())
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, metamodelica::ReferenceEq)]
#[repr(i32)]
pub enum StateSelect {
    NEVER = 1,
    AVOID = 2,
    DEFAULT = 3,
    PREFER = 4,
    ALWAYS = 5,
}
impl PartialOrd for StateSelect {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for StateSelect {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (*self as i32).cmp(&(*other as i32))
    }
}
impl metamodelica::gc::MMTrace for StateSelect {
    fn mm_accept(&self, _: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        Ok(())
    }
}
impl Default for StateSelect {
    fn default() -> Self {
        Self::NEVER
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, metamodelica::ReferenceEq)]
#[repr(i32)]
pub enum TearingSelect {
    NEVER = 1,
    AVOID = 2,
    DEFAULT = 3,
    PREFER = 4,
    ALWAYS = 5,
}
impl PartialOrd for TearingSelect {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for TearingSelect {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (*self as i32).cmp(&(*other as i32))
    }
}
impl metamodelica::gc::MMTrace for TearingSelect {
    fn mm_accept(&self, _: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        Ok(())
    }
}
impl Default for TearingSelect {
    fn default() -> Self {
        Self::NEVER
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, metamodelica::ReferenceEq)]
#[repr(i32)]
pub enum Uncertainty {
    GIVEN = 1,
    SOUGHT = 2,
    REFINE = 3,
    PROPAGATE = 4,
}
impl PartialOrd for Uncertainty {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for Uncertainty {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (*self as i32).cmp(&(*other as i32))
    }
}
impl metamodelica::gc::MMTrace for Uncertainty {
    fn mm_accept(&self, _: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        Ok(())
    }
}

#[derive(Clone, Debug, Eq, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub struct Distribution {
    pub name: metamodelica::Ref<Expression::NFExpression>,
    pub params: metamodelica::Ref<Expression::NFExpression>,
    pub paramNames: metamodelica::Ref<Expression::NFExpression>,
}

impl metamodelica::gc::MMTrace for Distribution {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        metamodelica::gc::MMTrace::mm_accept(&self.name, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.params, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.paramNames, __mmv)?;
        Ok(())
    }
}
pub type DISTRIBUTION = Distribution;

pub mod Annotations {
    use super::*;
    /// all annotations that are vendor specific
    ///      note: doesn't include __OpenModelica_tearingSelect, this is considered a first class attribute
    #[derive(Clone, Debug, Eq, Hash, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
    pub struct Annotations {
        pub hideResult: bool,
        pub resizable: bool,
        pub optimizable: bool,
        pub optimizerExpression: Option<OptimizerExpression>,
    }

    impl metamodelica::gc::MMTrace for Annotations {
        fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
            metamodelica::gc::MMTrace::mm_accept(&self.hideResult, __mmv)?;
            metamodelica::gc::MMTrace::mm_accept(&self.resizable, __mmv)?;
            metamodelica::gc::MMTrace::mm_accept(&self.optimizable, __mmv)?;
            metamodelica::gc::MMTrace::mm_accept(&self.optimizerExpression, __mmv)?;
            Ok(())
        }
    }
    impl Default for Annotations {
        fn default() -> Self {
            Self {
                hideResult: Default::default(),
                resizable: Default::default(),
                optimizable: Default::default(),
                optimizerExpression: Default::default(),
            }
        }
    }

    pub type ANNOTATIONS = Annotations;

    pub fn create(
        mut comment: &metamodelica::Ref<SCode::Comment>,
        mut attributes: &metamodelica::Ref<Attributes::NFAttributes>,
    ) -> metamodelica::Ref<Annotations> {
        let mut annotations: metamodelica::Ref<Annotations> = EMPTY_ANNOTATIONS.clone();
        let mut r#mod: metamodelica::Ref<SCode::Mod>;
        let mut b: bool;
        if attributes.isResizable.clone() {
            assign_field!(annotations.resizable = true);
        }
        let () = (::match_deref::match_deref! { match comment {
            Deref @ SCode::Comment { annotation_: Some(Deref @ SCode::Annotation { modification: __esc_mod @ Deref @ SCode::Mod::MOD { .. } }), .. } => {
                r#mod = (*__esc_mod).clone();
                for mut submod in &*var_field!((*r#mod).subModLst, SCode::Mod::MOD).clone() {
                    let () = (::match_deref::match_deref! { match &(submod.clone()) {
            Deref @ SCode::SubMod { ident: Deref @ "HideResult", r#mod: Deref @ SCode::Mod::MOD { binding: Some(Deref @ Absyn::Exp::BOOL { value: true }), .. } } => {
                assign_field!(annotations.hideResult = true);
                ()
            },
            Deref @ SCode::SubMod { ident: Deref @ "__OpenModelica_resizable", r#mod: Deref @ SCode::Mod::MOD { binding: Some(Deref @ Absyn::Exp::BOOL { value: __esc_b }), .. } } => {
                b = (*__esc_b).clone();
                assign_field!(annotations.resizable = b.clone());
                ()
            },
            Deref @ SCode::SubMod { ident: Deref @ "optimizable", r#mod: Deref @ SCode::Mod::MOD { binding: Some(Deref @ Absyn::Exp::BOOL { value: __esc_b }), .. } } => {
                b = (*__esc_b).clone();
                assign_field!(annotations.optimizable = b.clone());
                ()
            },
            Deref @ SCode::SubMod { ident: Deref @ "isMayer", r#mod: Deref @ SCode::Mod::MOD { binding: Some(Deref @ Absyn::Exp::BOOL { value: __esc_b }), .. } } => {
                b = (*__esc_b).clone();
                assign_field!(annotations.optimizerExpression = Some(OptimizerExpression::MAYER.clone()));
                ()
            },
            Deref @ SCode::SubMod { ident: Deref @ "isLagrange", r#mod: Deref @ SCode::Mod::MOD { binding: Some(Deref @ Absyn::Exp::BOOL { value: __esc_b }), .. } } => {
                b = (*__esc_b).clone();
                assign_field!(annotations.optimizerExpression = Some(OptimizerExpression::LAGRANGE.clone()));
                ()
            },
            Deref @ SCode::SubMod { ident: Deref @ "isConstraint", r#mod: Deref @ SCode::Mod::MOD { binding: Some(Deref @ Absyn::Exp::BOOL { value: __esc_b }), .. } } => {
                b = (*__esc_b).clone();
                assign_field!(annotations.optimizerExpression = Some(OptimizerExpression::PATH_CONSTRAINT.clone()));
                ()
            },
            Deref @ SCode::SubMod { ident: Deref @ "isInitialConstraint", r#mod: Deref @ SCode::Mod::MOD { binding: Some(Deref @ Absyn::Exp::BOOL { value: __esc_b }), .. } } => {
                b = (*__esc_b).clone();
                assign_field!(annotations.optimizerExpression = Some(OptimizerExpression::INITIAL_CONSTRAINT.clone()));
                ()
            },
            Deref @ SCode::SubMod { ident: Deref @ "isFinalConstraint", r#mod: Deref @ SCode::Mod::MOD { binding: Some(Deref @ Absyn::Exp::BOOL { value: __esc_b }), .. } } => {
                b = (*__esc_b).clone();
                assign_field!(annotations.optimizerExpression = Some(OptimizerExpression::FINAL_CONSTRAINT.clone()));
                ()
            },
            Deref @ SCode::SubMod { ident: Deref @ "isInitialTime", r#mod: Deref @ SCode::Mod::MOD { binding: Some(Deref @ Absyn::Exp::BOOL { value: __esc_b }), .. } } => {
                b = (*__esc_b).clone();
                assign_field!(annotations.optimizerExpression = Some(OptimizerExpression::INITIAL_TIME.clone()));
                ()
            },
            Deref @ SCode::SubMod { ident: Deref @ "isFinalTime", r#mod: Deref @ SCode::Mod::MOD { binding: Some(Deref @ Absyn::Exp::BOOL { value: __esc_b }), .. } } => {
                b = (*__esc_b).clone();
                assign_field!(annotations.optimizerExpression = Some(OptimizerExpression::FINAL_TIME.clone()));
                ()
            },
            _ => (),
            _ => unreachable!("match_deref! exhaustiveness placeholder"),
        } });
                }
                ()
            },
            _ => (),
            _ => unreachable!("match_deref! exhaustiveness placeholder"),
        } });
        annotations
    }
}

// TODO: how to use Initial or Final state? - better state-pair Real x_0 = x (initialState = true);  -> binding only for initial time / optimizer?
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, metamodelica::ReferenceEq)]
#[repr(i32)]
pub enum OptimizerExpression {
    MAYER = 1,
    LAGRANGE = 2,
    PATH_CONSTRAINT = 3,
    INITIAL_CONSTRAINT = 4,
    FINAL_CONSTRAINT = 5,
    INITIAL_TIME = 6,
    FINAL_TIME = 7,
}
impl PartialOrd for OptimizerExpression {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for OptimizerExpression {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (*self as i32).cmp(&(*other as i32))
    }
}
impl metamodelica::gc::MMTrace for OptimizerExpression {
    fn mm_accept(&self, _: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        Ok(())
    }
}
impl Default for OptimizerExpression {
    fn default() -> Self {
        Self::MAYER
    }
}

pub static EMPTY_ANNOTATIONS: std::sync::LazyLock<metamodelica::Ref<Annotations::Annotations>> =
    std::sync::LazyLock::new(|| {
        metamodelica::Ref::new(Annotations::Annotations {
            hideResult: false,
            resizable: false,
            optimizable: false,
            optimizerExpression: None,
        })
    });
