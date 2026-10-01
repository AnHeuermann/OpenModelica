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

use crate::BackendCevalInterface;
use crate::Ceval;
use crate::FGraph;
use crate::FNode;
use crate::InnerOuter;
use crate::Inst;
use crate::InstFunction;
use crate::InstMeta;
use crate::Lookup;
use crate::OperatorOverloading;
use crate::Patternm;
use crate::PrefixUtil;
use openmodelica_ast::Absyn;
use openmodelica_error::ErrorExt;
use openmodelica_frontend_base::ComponentReference;
use openmodelica_frontend_base::DAEUtil;
use openmodelica_frontend_base::Expression;
use openmodelica_frontend_base::ExpressionDump;
use openmodelica_frontend_base::ExpressionSimplify;
use openmodelica_frontend_base::Inline;
use openmodelica_frontend_base::Types;
use openmodelica_frontend_base::ValuesUtil;
use openmodelica_frontend_base::VarTransform;
use openmodelica_frontend_dump::AbsynToSCode;
use openmodelica_frontend_dump::AbsynUtil;
use openmodelica_frontend_dump::AvlTreePathFunction;
use openmodelica_frontend_dump::BackendInterface;
use openmodelica_frontend_dump::ComponentReferenceBasics;
use openmodelica_frontend_dump::Dump;
use openmodelica_frontend_dump::ExpressionBasics;
use openmodelica_frontend_dump::FCore;
use openmodelica_frontend_dump::MetaUtil;
use openmodelica_frontend_dump::SCodeDump;
use openmodelica_frontend_dump::SCodeUtil;
use openmodelica_frontend_dump::TypesDump;
use openmodelica_frontend_dump::ValuesDump;
use openmodelica_frontend_inst::InstTypes;
use openmodelica_frontend_types::ClassInf;
use openmodelica_frontend_types::DAE;
use openmodelica_frontend_types::SCode;
use openmodelica_frontend_types::Values;
use openmodelica_util::Config;
use openmodelica_util::Debug;
use openmodelica_util::Error;
use openmodelica_util::Flags;
use openmodelica_util::Global;
use openmodelica_util::Print;
use openmodelica_util::System;
use openmodelica_util::Util;
use openmodelica_util_datatypes_basic::Array;
use openmodelica_util_datatypes_basic::List;
use openmodelica_util_datatypes_basic::Mutable;

pub(crate) const SLOT_NOT_EVALUATED: i32 = 0;

pub(crate) const SLOT_EVALUATING: i32 = 1;

pub(crate) const SLOT_EVALUATED: i32 = 2;

#[derive(Clone, Debug, Eq, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub struct Slot {
    /// The slots default argument.
    pub defaultArg: metamodelica::Ref<DAE::FuncArg>,
    /// True if the slot has been filled, otherwise false.
    pub slotFilled: bool,
    /// The argument for the slot given by the function call.
    pub arg: Option<metamodelica::Ref<DAE::Exp>>,
    /// The dimensions of the slot.
    pub dims: metamodelica::List<metamodelica::Ref<DAE::Dimension>>,
    /// The index of the slot, 1 = first slot etc.
    pub idx: i32,
    pub evalStatus: i32,
}

impl metamodelica::gc::MMTrace for Slot {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        metamodelica::gc::MMTrace::mm_accept(&self.defaultArg, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.slotFilled, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.arg, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.dims, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.idx, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.evalStatus, __mmv)?;
        Ok(())
    }
}
impl Default for Slot {
    fn default() -> Self {
        Self {
            defaultArg: Default::default(),
            slotFilled: Default::default(),
            arg: Default::default(),
            dims: Default::default(),
            idx: Default::default(),
            evalStatus: Default::default(),
        }
    }
}

pub type SLOT = Slot;

thread_local! { static __BUILTIN_TIME_TLS: Option<(metamodelica::Ref<DAE::Exp>, DAE::Properties, metamodelica::Ref<DAE::Attributes>)> = Some((metamodelica::Ref::new(DAE::Exp::CREF { componentRef: metamodelica::Ref::new(DAE::ComponentRef::CREF_IDENT { ident: literal!("time"), identType: DAE::T_REAL_DEFAULT().clone(), subscriptLst: metamodelica::nil() }), ty: DAE::T_REAL_DEFAULT().clone() }), DAE::Properties::PROP { type_: DAE::T_REAL_DEFAULT().clone(), constFlag: openmodelica_frontend_types::DAE::Const::C_VAR }, DAE::dummyAttrInput().clone())); }
pub(crate) fn BUILTIN_TIME() -> Option<(
    metamodelica::Ref<DAE::Exp>,
    DAE::Properties,
    metamodelica::Ref<DAE::Attributes>,
)> {
    __BUILTIN_TIME_TLS.with(|__t| __t.clone())
}

pub(crate) fn elabExpList(
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
    mut inExpl: &metamodelica::List<metamodelica::Ref<Absyn::Exp>>,
    mut inImplicit: bool,
    mut inDoVect: bool,
    mut inPrefix: DAE::Prefix,
    mut inInfo: SourceInfo,
    mut inLastType: metamodelica::Ref<DAE::Type>,
) -> Result<(
    FCore::Cache,
    metamodelica::List<metamodelica::Ref<DAE::Exp>>,
    metamodelica::List<DAE::Properties>,
)> {
    let mut outCache: FCore::Cache = inCache;
    let mut outExpl: metamodelica::List<metamodelica::Ref<DAE::Exp>> = metamodelica::nil();
    let mut outProperties: metamodelica::List<DAE::Properties> = metamodelica::nil();
    let mut exp: metamodelica::Ref<DAE::Exp>;
    let mut prop: DAE::Properties;
    let mut last_ty: metamodelica::Ref<DAE::Type> = inLastType;
    let mut cr: metamodelica::Ref<Absyn::ComponentRef>;
    let mut path: metamodelica::Ref<Absyn::Path>;
    let mut path1: metamodelica::Ref<Absyn::Path>;
    let mut path2: metamodelica::Ref<Absyn::Path>;
    let mut name: ArcStr;
    let mut names: metamodelica::List<ArcStr>;
    let mut idx: i32;
    for mut e in &**inExpl {
        match '__try0: {
            let __pa1 = ::match_deref::match_deref! { match &(e.clone()) {
                Deref @ Absyn::Exp::CREF { componentRef: __pa1 @ Deref @ Absyn::ComponentRef::CREF_FULLYQUALIFIED { .. } } => __pa1.clone(),
                _ => break '__try0 Err::<_, _>("pattern mismatch"),
            } };
            cr = metamodelica::Own::own(__pa1);
            let (__pa3, __pa4) = ::match_deref::match_deref! { match &(last_ty.clone()) {
                Deref @ DAE::Type::T_ENUMERATION { path: __pa3, names: __pa4, .. } => (__pa3.clone(), __pa4.clone()),
                _ => break '__try0 Err::<_, _>("pattern mismatch"),
            } };
            path2 = metamodelica::Own::own(__pa3);
            names = metamodelica::Own::own(__pa4);
            path = unwrap_break_err!(AbsynUtil::crefToPath(&cr), '__try0);
            let (__pa5, __pa6) = ::match_deref::match_deref! { match &(unwrap_break_err!(AbsynUtil::splitQualAndIdentPath(&path), '__try0)) {
                (__pa5, Deref @ Absyn::Path::IDENT { name: __pa6 }) => (__pa5.clone(), __pa6.clone()),
                _ => break '__try0 Err::<_, _>("pattern mismatch"),
            } };
            path1 = metamodelica::Own::own(__pa5);
            name = metamodelica::Own::own(__pa6);
            let true = (AbsynUtil::pathEqual(&path1, &path2)) else {
                break '__try0 Err::<_, _>("pattern mismatch");
            };
            idx = unwrap_break_err!(List::position(name.clone(), &names), '__try0);
            exp = metamodelica::Ref::new(DAE::Exp::ENUM_LITERAL {
                name: path.clone(),
                index: idx,
            });
            prop = DAE::Properties::PROP {
                type_: last_ty.clone(),
                constFlag: openmodelica_frontend_types::DAE::Const::C_CONST,
            };
            Ok::<_, &'static str>((exp.clone(), prop.clone()))
        } {
            Ok((__try0_o0, __try0_o1)) => {
                exp = __try0_o0;
                prop = __try0_o1;
            }
            Err(_) => {
                (outCache, exp, prop) = elabExpInExpression(
                    outCache.clone(),
                    inEnv.clone(),
                    e.clone(),
                    inImplicit,
                    inDoVect,
                    inPrefix.clone(),
                    inInfo.clone(),
                )?;
                last_ty = Types::getPropType(&prop);
            }
        }
        outExpl = metamodelica::cons(exp.clone(), outExpl);
        outProperties = metamodelica::cons(prop.clone(), outProperties);
    }
    outExpl = outExpl.reverse();
    outProperties = outProperties.reverse();
    Ok((outCache, outExpl, outProperties))
}

fn elabExpList_enum(mut inExp: &metamodelica::Ref<Absyn::Exp>, mut inLastType: &metamodelica::Ref<DAE::Type>) -> i32 {
    let mut outIndex: i32;
    outIndex = 'mc: {
        let __mc_input = (&**inExp, &**inLastType);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ Absyn::Exp::CREF { componentRef: cr @ Deref @ Absyn::ComponentRef::CREF_FULLYQUALIFIED { .. } }, Deref @ DAE::Type::T_ENUMERATION { path: path2, names, .. }) => {
                    let mut path: metamodelica::Ref<Absyn::Path>;
                    let mut path1: metamodelica::Ref<Absyn::Path>;
                    let mut name: ArcStr;
                    path = AbsynUtil::crefToPath(metamodelica::AsArg::as_arg(&cr))?;
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(AbsynUtil::splitQualAndIdentPath(&path)?) {
                        (__pa0, Deref @ Absyn::Path::IDENT { name: __pa1 }) => (__pa0.clone(), __pa1.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    path1 = metamodelica::Own::own(__pa0);
                    name = metamodelica::Own::own(__pa1);
                    let true = (AbsynUtil::pathEqual(&path1, metamodelica::AsArg::as_arg(&path2))) else { return Err("pattern mismatch") };
                    Ok(List::position(name.clone(), metamodelica::AsArg::as_arg(&names))?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(-1)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    outIndex
}

pub(crate) fn elabExpListList(
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
    mut inExpl: &metamodelica::List<metamodelica::List<metamodelica::Ref<Absyn::Exp>>>,
    mut inImplicit: bool,
    mut inDoVect: bool,
    mut inPrefix: DAE::Prefix,
    mut inInfo: SourceInfo,
    mut inLastType: metamodelica::Ref<DAE::Type>,
) -> Result<(
    FCore::Cache,
    metamodelica::List<metamodelica::List<metamodelica::Ref<DAE::Exp>>>,
    metamodelica::List<metamodelica::List<DAE::Properties>>,
)> {
    let mut outCache: FCore::Cache = inCache;
    let mut outExpl: metamodelica::List<metamodelica::List<metamodelica::Ref<DAE::Exp>>> = metamodelica::nil();
    let mut outProperties: metamodelica::List<metamodelica::List<DAE::Properties>> = metamodelica::nil();
    let mut expl: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
    let mut props: metamodelica::List<DAE::Properties>;
    let mut last_ty: metamodelica::Ref<DAE::Type> = inLastType;
    for mut lst in &**inExpl {
        (outCache, expl, props) = elabExpList(
            outCache,
            inEnv.clone(),
            metamodelica::AsArg::as_arg(&lst),
            inImplicit,
            inDoVect,
            inPrefix.clone(),
            inInfo.clone(),
            last_ty,
        )?;
        outExpl = metamodelica::cons(expl, outExpl);
        outProperties = metamodelica::cons(props.clone(), outProperties);
        last_ty = Types::getPropType(&((props).head().cloned()?));
    }
    outExpl = outExpl.reverse();
    outProperties = outProperties.reverse();
    Ok((outCache, outExpl, outProperties))
}

fn elabExpOptAndMatchType(
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
    mut inExp: Option<metamodelica::Ref<Absyn::Exp>>,
    mut inDefaultType: metamodelica::Ref<DAE::Type>,
    mut inImplicit: bool,
    mut inDoVect: bool,
    mut inPrefix: DAE::Prefix,
    mut inInfo: SourceInfo,
) -> Result<(FCore::Cache, Option<metamodelica::Ref<DAE::Exp>>, DAE::Properties)> {
    let mut outCache: FCore::Cache = inCache;
    let mut outExp: Option<metamodelica::Ref<DAE::Exp>>;
    let mut outProperties: DAE::Properties;
    let mut exp: metamodelica::Ref<Absyn::Exp>;
    let mut dexp: metamodelica::Ref<DAE::Exp>;
    let mut prop: DAE::Properties;
    outProperties = DAE::Properties::PROP {
        type_: inDefaultType,
        constFlag: openmodelica_frontend_types::DAE::Const::C_CONST,
    };
    if (inExp).is_some() {
        let __pa0 = ::match_deref::match_deref! { match &(inExp) {
            Some(__pa0) => __pa0.clone(),
            _ => return Err("pattern mismatch"),
        } };
        exp = metamodelica::Own::own(__pa0);
        (outCache, dexp, prop) = elabExpInExpression(outCache, inEnv, exp, inImplicit, inDoVect, inPrefix, inInfo)?;
        (dexp, outProperties) = Types::matchProp(dexp, &prop, &outProperties, true)?;
        outExp = Some(dexp);
    } else {
        outExp = None;
    }
    Ok((outCache, outExp, outProperties))
}

pub fn elabExp(
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
    mut inExp: metamodelica::Ref<Absyn::Exp>,
    mut inImplicit: bool,
    mut inDoVect: bool,
    mut inPrefix: DAE::Prefix,
    mut inInfo: SourceInfo,
) -> Result<(FCore::Cache, metamodelica::Ref<DAE::Exp>, DAE::Properties)> {
    let mut outCache: FCore::Cache = inCache.clone();
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    let mut outProperties: DAE::Properties;
    let mut e: metamodelica::Ref<Absyn::Exp>;
    let mut num_errmsgs: i32;
    let mut elabfunc: PartialElabExpFunc;
    e = if (BackendInterface::noRewriteRulesFrontEnd()?) {
        inExp
    } else {
        (BackendInterface::rewriteFrontEnd(inExp)?).0
    };
    num_errmsgs = Error::getNumErrorMessages();
    match '__try0: {
        elabfunc = (match &*e {
            Absyn::Exp::END { .. } => {
                unwrap_break_err!(Error::addSourceMessage(&(Error::END_ILLEGAL_USE_ERROR.clone()), metamodelica::nil(), &inInfo), '__try0);
                break '__try0 Err::<_, _>("fail");
            }
            Absyn::Exp::CREF { .. } => {
                (std::sync::Arc::new(elabExp_Cref)
                    as std::sync::Arc<
                        dyn ::std::ops::Fn(
                                FCore::Cache,
                                FCore::Graph,
                                metamodelica::Ref<Absyn::Exp>,
                                bool,
                                bool,
                                DAE::Prefix,
                                SourceInfo,
                            )
                                -> Result<(FCore::Cache, metamodelica::Ref<DAE::Exp>, DAE::Properties)>
                            + 'static,
                    >)
            }
            Absyn::Exp::BINARY { .. } => {
                (std::sync::Arc::new(elabExp_Binary)
                    as std::sync::Arc<
                        dyn ::std::ops::Fn(
                                FCore::Cache,
                                FCore::Graph,
                                metamodelica::Ref<Absyn::Exp>,
                                bool,
                                bool,
                                DAE::Prefix,
                                SourceInfo,
                            )
                                -> Result<(FCore::Cache, metamodelica::Ref<DAE::Exp>, DAE::Properties)>
                            + 'static,
                    >)
            }
            Absyn::Exp::UNARY { .. } => {
                (std::sync::Arc::new(elabExp_Unary)
                    as std::sync::Arc<
                        dyn ::std::ops::Fn(
                                FCore::Cache,
                                FCore::Graph,
                                metamodelica::Ref<Absyn::Exp>,
                                bool,
                                bool,
                                DAE::Prefix,
                                SourceInfo,
                            )
                                -> Result<(FCore::Cache, metamodelica::Ref<DAE::Exp>, DAE::Properties)>
                            + 'static,
                    >)
            }
            Absyn::Exp::LBINARY { .. } => {
                (std::sync::Arc::new(elabExp_Binary)
                    as std::sync::Arc<
                        dyn ::std::ops::Fn(
                                FCore::Cache,
                                FCore::Graph,
                                metamodelica::Ref<Absyn::Exp>,
                                bool,
                                bool,
                                DAE::Prefix,
                                SourceInfo,
                            )
                                -> Result<(FCore::Cache, metamodelica::Ref<DAE::Exp>, DAE::Properties)>
                            + 'static,
                    >)
            }
            Absyn::Exp::LUNARY { .. } => {
                (std::sync::Arc::new(elabExp_LUnary)
                    as std::sync::Arc<
                        dyn ::std::ops::Fn(
                                FCore::Cache,
                                FCore::Graph,
                                metamodelica::Ref<Absyn::Exp>,
                                bool,
                                bool,
                                DAE::Prefix,
                                SourceInfo,
                            )
                                -> Result<(FCore::Cache, metamodelica::Ref<DAE::Exp>, DAE::Properties)>
                            + 'static,
                    >)
            }
            Absyn::Exp::RELATION { .. } => {
                (std::sync::Arc::new(elabExp_Binary)
                    as std::sync::Arc<
                        dyn ::std::ops::Fn(
                                FCore::Cache,
                                FCore::Graph,
                                metamodelica::Ref<Absyn::Exp>,
                                bool,
                                bool,
                                DAE::Prefix,
                                SourceInfo,
                            )
                                -> Result<(FCore::Cache, metamodelica::Ref<DAE::Exp>, DAE::Properties)>
                            + 'static,
                    >)
            }
            Absyn::Exp::IFEXP { .. } => {
                (std::sync::Arc::new(elabExp_If)
                    as std::sync::Arc<
                        dyn ::std::ops::Fn(
                                FCore::Cache,
                                FCore::Graph,
                                metamodelica::Ref<Absyn::Exp>,
                                bool,
                                bool,
                                DAE::Prefix,
                                SourceInfo,
                            )
                                -> Result<(FCore::Cache, metamodelica::Ref<DAE::Exp>, DAE::Properties)>
                            + 'static,
                    >)
            }
            Absyn::Exp::CALL { .. } => {
                (std::sync::Arc::new(elabExp_Call)
                    as std::sync::Arc<
                        dyn ::std::ops::Fn(
                                FCore::Cache,
                                FCore::Graph,
                                metamodelica::Ref<Absyn::Exp>,
                                bool,
                                bool,
                                DAE::Prefix,
                                SourceInfo,
                            )
                                -> Result<(FCore::Cache, metamodelica::Ref<DAE::Exp>, DAE::Properties)>
                            + 'static,
                    >)
            }
            Absyn::Exp::PARTEVALFUNCTION { .. } => {
                (std::sync::Arc::new(elabExp_PartEvalFunction)
                    as std::sync::Arc<
                        dyn ::std::ops::Fn(
                                FCore::Cache,
                                FCore::Graph,
                                metamodelica::Ref<Absyn::Exp>,
                                bool,
                                bool,
                                DAE::Prefix,
                                SourceInfo,
                            )
                                -> Result<(FCore::Cache, metamodelica::Ref<DAE::Exp>, DAE::Properties)>
                            + 'static,
                    >)
            }
            Absyn::Exp::TUPLE { .. } => {
                (std::sync::Arc::new(elabExp_Tuple)
                    as std::sync::Arc<
                        dyn ::std::ops::Fn(
                                FCore::Cache,
                                FCore::Graph,
                                metamodelica::Ref<Absyn::Exp>,
                                bool,
                                bool,
                                DAE::Prefix,
                                SourceInfo,
                            )
                                -> Result<(FCore::Cache, metamodelica::Ref<DAE::Exp>, DAE::Properties)>
                            + 'static,
                    >)
            }
            Absyn::Exp::RANGE { .. } => {
                (std::sync::Arc::new(elabExp_Range)
                    as std::sync::Arc<
                        dyn ::std::ops::Fn(
                                FCore::Cache,
                                FCore::Graph,
                                metamodelica::Ref<Absyn::Exp>,
                                bool,
                                bool,
                                DAE::Prefix,
                                SourceInfo,
                            )
                                -> Result<(FCore::Cache, metamodelica::Ref<DAE::Exp>, DAE::Properties)>
                            + 'static,
                    >)
            }
            Absyn::Exp::ARRAY { .. } => {
                (std::sync::Arc::new(elabExp_Array)
                    as std::sync::Arc<
                        dyn ::std::ops::Fn(
                                FCore::Cache,
                                FCore::Graph,
                                metamodelica::Ref<Absyn::Exp>,
                                bool,
                                bool,
                                DAE::Prefix,
                                SourceInfo,
                            )
                                -> Result<(FCore::Cache, metamodelica::Ref<DAE::Exp>, DAE::Properties)>
                            + 'static,
                    >)
            }
            Absyn::Exp::MATRIX { .. } => {
                (std::sync::Arc::new(elabExp_Matrix)
                    as std::sync::Arc<
                        dyn ::std::ops::Fn(
                                FCore::Cache,
                                FCore::Graph,
                                metamodelica::Ref<Absyn::Exp>,
                                bool,
                                bool,
                                DAE::Prefix,
                                SourceInfo,
                            )
                                -> Result<(FCore::Cache, metamodelica::Ref<DAE::Exp>, DAE::Properties)>
                            + 'static,
                    >)
            }
            Absyn::Exp::CODE { .. } => {
                (std::sync::Arc::new(elabExp_Code)
                    as std::sync::Arc<
                        dyn ::std::ops::Fn(
                                FCore::Cache,
                                FCore::Graph,
                                metamodelica::Ref<Absyn::Exp>,
                                bool,
                                bool,
                                DAE::Prefix,
                                SourceInfo,
                            )
                                -> Result<(FCore::Cache, metamodelica::Ref<DAE::Exp>, DAE::Properties)>
                            + 'static,
                    >)
            }
            Absyn::Exp::CONS { .. } => {
                (std::sync::Arc::new(elabExp_Cons)
                    as std::sync::Arc<
                        dyn ::std::ops::Fn(
                                FCore::Cache,
                                FCore::Graph,
                                metamodelica::Ref<Absyn::Exp>,
                                bool,
                                bool,
                                DAE::Prefix,
                                SourceInfo,
                            )
                                -> Result<(FCore::Cache, metamodelica::Ref<DAE::Exp>, DAE::Properties)>
                            + 'static,
                    >)
            }
            Absyn::Exp::LIST { .. } => {
                (std::sync::Arc::new(elabExp_List)
                    as std::sync::Arc<
                        dyn ::std::ops::Fn(
                                FCore::Cache,
                                FCore::Graph,
                                metamodelica::Ref<Absyn::Exp>,
                                bool,
                                bool,
                                DAE::Prefix,
                                SourceInfo,
                            )
                                -> Result<(FCore::Cache, metamodelica::Ref<DAE::Exp>, DAE::Properties)>
                            + 'static,
                    >)
            }
            Absyn::Exp::MATCHEXP { .. } => {
                (std::sync::Arc::new(Patternm::elabMatchExpression)
                    as std::sync::Arc<
                        dyn ::std::ops::Fn(
                                FCore::Cache,
                                FCore::Graph,
                                metamodelica::Ref<Absyn::Exp>,
                                bool,
                                bool,
                                DAE::Prefix,
                                SourceInfo,
                            )
                                -> Result<(FCore::Cache, metamodelica::Ref<DAE::Exp>, DAE::Properties)>
                            + 'static,
                    >)
            }
            Absyn::Exp::DOT { .. } => {
                (std::sync::Arc::new(elabExp_Dot)
                    as std::sync::Arc<
                        dyn ::std::ops::Fn(
                                FCore::Cache,
                                FCore::Graph,
                                metamodelica::Ref<Absyn::Exp>,
                                bool,
                                bool,
                                DAE::Prefix,
                                SourceInfo,
                            )
                                -> Result<(FCore::Cache, metamodelica::Ref<DAE::Exp>, DAE::Properties)>
                            + 'static,
                    >)
            }
            Absyn::Exp::EXPRESSIONCOMMENT { .. } => {
                (std::sync::Arc::new(elabExp_Comment)
                    as std::sync::Arc<
                        dyn ::std::ops::Fn(
                                FCore::Cache,
                                FCore::Graph,
                                metamodelica::Ref<Absyn::Exp>,
                                bool,
                                bool,
                                DAE::Prefix,
                                SourceInfo,
                            )
                                -> Result<(FCore::Cache, metamodelica::Ref<DAE::Exp>, DAE::Properties)>
                            + 'static,
                    >)
            }
            _ => {
                (std::sync::Arc::new(elabExp_BuiltinType)
                    as std::sync::Arc<
                        dyn ::std::ops::Fn(
                                FCore::Cache,
                                FCore::Graph,
                                metamodelica::Ref<Absyn::Exp>,
                                bool,
                                bool,
                                DAE::Prefix,
                                SourceInfo,
                            )
                                -> Result<(FCore::Cache, metamodelica::Ref<DAE::Exp>, DAE::Properties)>
                            + 'static,
                    >)
            }
        });
        (outCache, outExp, outProperties) = unwrap_break_err!(elabfunc(inCache.clone(), inEnv.clone(), e.clone(), inImplicit, inDoVect, inPrefix.clone(), inInfo.clone()), '__try0);
        Ok::<_, &'static str>((
            elabfunc.clone(),
            outCache.clone(),
            outExp.clone(),
            outProperties.clone(),
        ))
    } {
        Ok((__try0_o0, __try0_o1, __try0_o2, __try0_o3)) => {
            elabfunc = __try0_o0;
            outCache = __try0_o1;
            outExp = __try0_o2;
            outProperties = __try0_o3;
        }
        Err(__try0_err) => {
            let true = (num_errmsgs == Error::getNumErrorMessages()) else {
                return Err("pattern mismatch");
            };
            Error::addSourceMessage(
                &(Error::GENERIC_ELAB_EXPRESSION.clone()),
                list![Dump::printExpStr(e.clone())?],
                &inInfo,
            )?;
            return Err(__try0_err);
        }
    }
    Ok((outCache, outExp, outProperties))
}

pub type PartialElabExpFunc = std::sync::Arc<
    dyn ::std::ops::Fn(
            FCore::Cache,
            FCore::Graph,
            metamodelica::Ref<Absyn::Exp>,
            bool,
            bool,
            DAE::Prefix,
            SourceInfo,
        ) -> Result<(FCore::Cache, metamodelica::Ref<DAE::Exp>, DAE::Properties)>
        + 'static,
>;

fn elabExp_BuiltinType(
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
    mut inExp: metamodelica::Ref<Absyn::Exp>,
    mut inImplicit: bool,
    mut inDoVect: bool,
    mut inPrefix: DAE::Prefix,
    mut inInfo: SourceInfo,
) -> Result<(FCore::Cache, metamodelica::Ref<DAE::Exp>, DAE::Properties)> {
    let mut outCache: FCore::Cache = inCache;
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    let mut outProperties: DAE::Properties;
    (outExp, outProperties) = (match &*inExp {
        Absyn::Exp::INTEGER { value: __inExp_value } => (
            metamodelica::Ref::new(DAE::Exp::ICONST {
                integer: __inExp_value.clone(),
            }),
            DAE::Properties::PROP {
                type_: DAE::T_INTEGER_DEFAULT().clone(),
                constFlag: openmodelica_frontend_types::DAE::Const::C_CONST,
            },
        ),
        Absyn::Exp::REAL { value: __inExp_value } => (
            metamodelica::Ref::new(DAE::Exp::RCONST {
                real: stringReal(__inExp_value.clone())?,
            }),
            DAE::Properties::PROP {
                type_: DAE::T_REAL_DEFAULT().clone(),
                constFlag: openmodelica_frontend_types::DAE::Const::C_CONST,
            },
        ),
        Absyn::Exp::STRING { value: __inExp_value } => (
            metamodelica::Ref::new(DAE::Exp::SCONST {
                string: System::unescapedString(__inExp_value.clone()),
            }),
            DAE::Properties::PROP {
                type_: DAE::T_STRING_DEFAULT().clone(),
                constFlag: openmodelica_frontend_types::DAE::Const::C_CONST,
            },
        ),
        Absyn::Exp::BOOL { value: __inExp_value } => (
            metamodelica::Ref::new(DAE::Exp::BCONST {
                bool: __inExp_value.clone(),
            }),
            DAE::Properties::PROP {
                type_: DAE::T_BOOL_DEFAULT().clone(),
                constFlag: openmodelica_frontend_types::DAE::Const::C_CONST,
            },
        ),
        _ => return Err("match: no arm matched"),
    });
    Ok((outCache, outExp, outProperties))
}

fn elabExp_Cref(
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
    mut inExp: metamodelica::Ref<Absyn::Exp>,
    mut inImplicit: bool,
    mut inDoVect: bool,
    mut inPrefix: DAE::Prefix,
    mut inInfo: SourceInfo,
) -> Result<(FCore::Cache, metamodelica::Ref<DAE::Exp>, DAE::Properties)> {
    let mut outCache: FCore::Cache = inCache.clone();
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    let mut outProperties: DAE::Properties;
    let mut cr: metamodelica::Ref<Absyn::ComponentRef>;
    let mut ty: metamodelica::Ref<DAE::Type>;
    let mut c: DAE::Const;
    let __pa0 = ::match_deref::match_deref! { match &(inExp) {
        Deref @ Absyn::Exp::CREF { componentRef: __pa0 } => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    cr = metamodelica::Own::own(__pa0);
    let (__pa1, __pa2, __pa3) = ::match_deref::match_deref! { match &(elabCref(inCache, inEnv, cr, inImplicit, inDoVect, inPrefix, inInfo)?) {
        (__pa1, Some((__pa2, __pa3, _))) => (__pa1.clone(), __pa2.clone(), __pa3.clone()),
        _ => return Err("pattern mismatch"),
    } };
    outCache = metamodelica::Own::own(__pa1);
    outExp = metamodelica::Own::own(__pa2);
    outProperties = metamodelica::Own::own(__pa3);
    if !(Flags::getConfigBool(Flags::CEVAL_EQUATION.clone())?) {
        let DAE::PROP {
            type_: __pa4,
            constFlag: __pa5,
        } = (outProperties.clone())
        else {
            return Err("pattern mismatch");
        };
        ty = metamodelica::Own::own(__pa4);
        c = metamodelica::Own::own(__pa5);
        outProperties = if (Types::isParameter(c)) {
            DAE::Properties::PROP {
                type_: ty,
                constFlag: openmodelica_frontend_types::DAE::Const::C_VAR,
            }
        } else {
            outProperties
        };
    }
    Ok((outCache, outExp, outProperties))
}

fn elabExp_Binary(
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
    mut inExp: metamodelica::Ref<Absyn::Exp>,
    mut inImplicit: bool,
    mut inDoVect: bool,
    mut inPrefix: DAE::Prefix,
    mut inInfo: SourceInfo,
) -> Result<(FCore::Cache, metamodelica::Ref<DAE::Exp>, DAE::Properties)> {
    let mut outCache: FCore::Cache = inCache.clone();
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    let mut outProperties: DAE::Properties;
    let mut e1: metamodelica::Ref<Absyn::Exp> = metamodelica::Ref::new(Absyn::Exp::BREAK);
    let mut e2: metamodelica::Ref<Absyn::Exp> = metamodelica::Ref::new(Absyn::Exp::BREAK);
    let mut op: Absyn::Operator = Absyn::Operator::ADD;
    let mut prop1: DAE::Properties;
    let mut prop2: DAE::Properties;
    let mut exp1: metamodelica::Ref<DAE::Exp>;
    let mut exp2: metamodelica::Ref<DAE::Exp>;
    let () = (match &*inExp {
        Absyn::Exp::BINARY {
            exp1: __esc_e1,
            op: __esc_op,
            exp2: __esc_e2,
        } => {
            e1 = (*__esc_e1).clone();
            op = (*__esc_op).clone();
            e2 = (*__esc_e2).clone();
            ()
        }
        Absyn::Exp::LBINARY {
            exp1: __esc_e1,
            op: __esc_op,
            exp2: __esc_e2,
        } => {
            e1 = (*__esc_e1).clone();
            op = (*__esc_op).clone();
            e2 = (*__esc_e2).clone();
            ()
        }
        Absyn::Exp::RELATION {
            exp1: __esc_e1,
            op: __esc_op,
            exp2: __esc_e2,
        } => {
            e1 = (*__esc_e1).clone();
            op = (*__esc_op).clone();
            e2 = (*__esc_e2).clone();
            ()
        }
        _ => return Err("match: no arm matched"),
    });
    (outCache, exp1, prop1) = elabExpInExpression(
        inCache,
        inEnv.clone(),
        e1.clone(),
        inImplicit,
        inDoVect,
        inPrefix.clone(),
        inInfo.clone(),
    )?;
    (outCache, exp2, prop2) = elabExpInExpression(
        outCache,
        inEnv.clone(),
        e2.clone(),
        inImplicit,
        inDoVect,
        inPrefix.clone(),
        inInfo.clone(),
    )?;
    (outCache, outExp, outProperties) = OperatorOverloading::binary(
        outCache, inEnv, op, prop1, exp1, prop2, exp2, &inExp, &e1, &e2, inImplicit, &inPrefix, &inInfo,
    )?;
    Ok((outCache, outExp, outProperties))
}

fn elabExp_Unary(
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
    mut inExp: metamodelica::Ref<Absyn::Exp>,
    mut inImplicit: bool,
    mut inDoVect: bool,
    mut inPrefix: DAE::Prefix,
    mut inInfo: SourceInfo,
) -> Result<(FCore::Cache, metamodelica::Ref<DAE::Exp>, DAE::Properties)> {
    let mut outCache: FCore::Cache = inCache.clone();
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    let mut outProperties: DAE::Properties;
    let mut e: metamodelica::Ref<Absyn::Exp>;
    let mut op: Absyn::Operator;
    let mut ty: metamodelica::Ref<DAE::Type>;
    let mut c: DAE::Const;
    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(inExp.clone()) {
        Deref @ Absyn::Exp::UNARY { op: __pa0, exp: __pa1 } => (__pa0.clone(), __pa1.clone()),
        _ => return Err("pattern mismatch"),
    } };
    op = metamodelica::Own::own(__pa0);
    e = metamodelica::Own::own(__pa1);
    let (__pa2, __pa3, __pa6, __pa4, __pa5) = ::match_deref::match_deref! { match &(elabExpInExpression(inCache, inEnv.clone(), e.clone(), inImplicit, inDoVect, inPrefix.clone(), inInfo.clone())?) {
        (__pa2, __pa3, __pa6 @ DAE::Properties::PROP { type_: __pa4, constFlag: __pa5 }) => (__pa2.clone(), __pa3.clone(), __pa6.clone(), __pa4.clone(), __pa5.clone()),
        _ => return Err("pattern mismatch"),
    } };
    outCache = metamodelica::Own::own(__pa2);
    outExp = metamodelica::Own::own(__pa3);
    ty = metamodelica::Own::own(__pa4);
    c = metamodelica::Own::own(__pa5);
    outProperties = metamodelica::Own::own(__pa6);
    if !(op == openmodelica_ast::Absyn::Operator::UPLUS
        && Types::isIntegerOrRealOrSubTypeOfEither(Types::arrayElementType(&ty)))
    {
        (outCache, outExp, outProperties) = OperatorOverloading::unary(
            outCache,
            inEnv,
            op,
            &outProperties,
            outExp,
            &inExp,
            e,
            inImplicit,
            &inPrefix,
            &inInfo,
        )?;
    }
    Ok((outCache, outExp, outProperties))
}

fn elabExp_LUnary(
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
    mut inExp: metamodelica::Ref<Absyn::Exp>,
    mut inImplicit: bool,
    mut inDoVect: bool,
    mut inPrefix: DAE::Prefix,
    mut inInfo: SourceInfo,
) -> Result<(FCore::Cache, metamodelica::Ref<DAE::Exp>, DAE::Properties)> {
    let mut outCache: FCore::Cache = inCache;
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    let mut outProperties: DAE::Properties;
    let mut e: metamodelica::Ref<Absyn::Exp>;
    let mut op: Absyn::Operator;
    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(inExp.clone()) {
        Deref @ Absyn::Exp::LUNARY { op: __pa0, exp: __pa1 } => (__pa0.clone(), __pa1.clone()),
        _ => return Err("pattern mismatch"),
    } };
    op = metamodelica::Own::own(__pa0);
    e = metamodelica::Own::own(__pa1);
    (outCache, outExp, outProperties) = elabExpInExpression(
        outCache,
        inEnv.clone(),
        e.clone(),
        inImplicit,
        inDoVect,
        inPrefix.clone(),
        inInfo.clone(),
    )?;
    (outCache, outExp, outProperties) = OperatorOverloading::unary(
        outCache,
        inEnv,
        op,
        &outProperties,
        outExp,
        &inExp,
        e,
        inImplicit,
        &inPrefix,
        &inInfo,
    )?;
    Ok((outCache, outExp, outProperties))
}

fn elabExp_If(
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
    mut inExp: metamodelica::Ref<Absyn::Exp>,
    mut inImplicit: bool,
    mut inDoVect: bool,
    mut inPrefix: DAE::Prefix,
    mut inInfo: SourceInfo,
) -> Result<(FCore::Cache, metamodelica::Ref<DAE::Exp>, DAE::Properties)> {
    let mut outCache: FCore::Cache = inCache.clone();
    let mut outExp: metamodelica::Ref<DAE::Exp> = <metamodelica::Ref<DAE::Exp> as ::std::default::Default>::default();
    let mut outProperties: DAE::Properties = <DAE::Properties as ::std::default::Default>::default();
    let mut cond_e: metamodelica::Ref<Absyn::Exp>;
    let mut true_e: metamodelica::Ref<Absyn::Exp>;
    let mut false_e: metamodelica::Ref<Absyn::Exp>;
    let mut cond_exp: metamodelica::Ref<DAE::Exp>;
    let mut true_exp: metamodelica::Ref<DAE::Exp> = <metamodelica::Ref<DAE::Exp> as ::std::default::Default>::default();
    let mut false_exp: metamodelica::Ref<DAE::Exp> =
        <metamodelica::Ref<DAE::Exp> as ::std::default::Default>::default();
    let mut cond_prop: DAE::Properties;
    let mut true_prop: DAE::Properties = <DAE::Properties as ::std::default::Default>::default();
    let mut false_prop: DAE::Properties = <DAE::Properties as ::std::default::Default>::default();
    let mut cache: FCore::Cache;
    let mut b: bool = false;
    let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(AbsynUtil::canonIfExp(&inExp)?) {
        Deref @ Absyn::Exp::IFEXP { ifExp: __pa0, trueBranch: __pa1, elseBranch: __pa2, .. } => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
        _ => return Err("pattern mismatch"),
    } };
    cond_e = metamodelica::Own::own(__pa0);
    true_e = metamodelica::Own::own(__pa1);
    false_e = metamodelica::Own::own(__pa2);
    (cache, cond_exp, cond_prop) = elabExpInExpression(
        inCache,
        inEnv.clone(),
        cond_e,
        inImplicit,
        inDoVect,
        inPrefix.clone(),
        inInfo.clone(),
    )?;
    (outCache, outExp, outProperties) = 'mc: {
        let __mc_input = ();
        if let Ok((__v, __wb0, __wb1, __wb2, __wb3, __wb4, __wb5, __wb6)) = (|| -> Result<_> {
            let () = __mc_input.clone() else { return Err("nomatch") };
            let mut false_exp: metamodelica::Ref<DAE::Exp> = false_exp.clone();
            let mut false_prop: DAE::Properties = false_prop.clone();
            let mut outCache: FCore::Cache = outCache.clone();
            let mut outExp: metamodelica::Ref<DAE::Exp> = outExp.clone();
            let mut outProperties: DAE::Properties = outProperties.clone();
            let mut true_exp: metamodelica::Ref<DAE::Exp> = true_exp.clone();
            let mut true_prop: DAE::Properties = true_prop.clone();
            ErrorExt::setCheckpoint(literal!("Static.elabExp:IFEXP"));
            (outCache, true_exp, true_prop) = elabExpInExpression(
                cache.clone(),
                inEnv.clone(),
                true_e.clone(),
                inImplicit,
                inDoVect,
                inPrefix.clone(),
                inInfo.clone(),
            )?;
            (outCache, false_exp, false_prop) = elabExpInExpression(
                outCache.clone(),
                inEnv.clone(),
                false_e.clone(),
                inImplicit,
                inDoVect,
                inPrefix.clone(),
                inInfo.clone(),
            )?;
            (outCache, outExp, outProperties) = makeIfExp(
                outCache.clone(),
                inEnv.clone(),
                cond_exp.clone(),
                cond_prop.clone(),
                true_exp.clone(),
                true_prop.clone(),
                false_exp.clone(),
                false_prop.clone(),
                inImplicit,
                inPrefix.clone(),
                &inInfo,
            )?;
            ErrorExt::delCheckpoint(literal!("Static.elabExp:IFEXP"));
            Ok((
                (outCache.clone(), outExp.clone(), outProperties.clone()),
                false_exp.clone(),
                false_prop.clone(),
                outCache.clone(),
                outExp.clone(),
                outProperties.clone(),
                true_exp.clone(),
                true_prop.clone(),
            ))
        })() {
            false_exp = __wb0;
            false_prop = __wb1;
            outCache = __wb2;
            outExp = __wb3;
            outProperties = __wb4;
            true_exp = __wb5;
            true_prop = __wb6;
            break 'mc __v;
        }
        if let Ok((__v, __wb0, __wb1, __wb2, __wb3)) = (|| -> Result<_> {
            let () = __mc_input.clone() else { return Err("nomatch") };
            let mut b: bool = b.clone();
            let mut outCache: FCore::Cache = outCache.clone();
            let mut outExp: metamodelica::Ref<DAE::Exp> = outExp.clone();
            let mut outProperties: DAE::Properties = outProperties.clone();
            ErrorExt::setCheckpoint(literal!("Static.elabExp:IFEXP:HACK"));
            let true = (Types::isParameterOrConstant(Types::propAllConst(cond_prop.clone())?)) else {
                return Err("pattern mismatch");
            };
            let (__pa0, __pa1) = ::match_deref::match_deref! { match &(Ceval::ceval(cache.clone(), inEnv.clone(), cond_exp.clone(), inImplicit, Absyn::Msg::MSG { info: inInfo.clone() }, 0)?) {
                (__pa0, Deref @ Values::Value::BOOL { boolean: __pa1 }) => (__pa0.clone(), __pa1.clone()),
                _ => return Err("pattern mismatch"),
            } };
            outCache = metamodelica::Own::own(__pa0);
            b = metamodelica::Own::own(__pa1);
            (outCache, outExp, outProperties) = elabExpInExpression(
                outCache.clone(),
                inEnv.clone(),
                if (b) { true_e.clone() } else { false_e.clone() },
                inImplicit,
                inDoVect,
                inPrefix.clone(),
                inInfo.clone(),
            )?;
            ErrorExt::delCheckpoint(literal!("Static.elabExp:IFEXP:HACK"));
            ErrorExt::rollBack(literal!("Static.elabExp:IFEXP"));
            Ok((
                (outCache.clone(), outExp.clone(), outProperties.clone()),
                b.clone(),
                outCache.clone(),
                outExp.clone(),
                outProperties.clone(),
            ))
        })() {
            b = __wb0;
            outCache = __wb1;
            outExp = __wb2;
            outProperties = __wb3;
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            ErrorExt::rollBack(literal!("Static.elabExp:IFEXP:HACK"));
            ErrorExt::delCheckpoint(literal!("Static.elabExp:IFEXP"));
            Ok(return Err("fail"))
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((outCache, outExp, outProperties))
}

fn elabExp_Call(
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
    mut inExp: metamodelica::Ref<Absyn::Exp>,
    mut inImplicit: bool,
    mut inDoVect: bool,
    mut inPrefix: DAE::Prefix,
    mut inInfo: SourceInfo,
) -> Result<(FCore::Cache, metamodelica::Ref<DAE::Exp>, DAE::Properties)> {
    let mut outCache: FCore::Cache = inCache.clone();
    let mut outExp: metamodelica::Ref<DAE::Exp> = <metamodelica::Ref<DAE::Exp> as ::std::default::Default>::default();
    let mut outProperties: DAE::Properties = <DAE::Properties as ::std::default::Default>::default();
    let mut func_name: metamodelica::Ref<Absyn::ComponentRef>;
    let mut args: metamodelica::Ref<Absyn::FunctionArgs>;
    let mut type_vars: metamodelica::List<metamodelica::Ref<Absyn::Path>>;
    let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(inExp) {
        Deref @ Absyn::Exp::CALL { function_: __pa0, functionArgs: __pa1, typeVars: __pa2 } => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
        _ => return Err("pattern mismatch"),
    } };
    func_name = metamodelica::Own::own(__pa0);
    args = metamodelica::Own::own(__pa1);
    type_vars = metamodelica::Own::own(__pa2);
    let () = (match &*args {
        Absyn::FunctionArgs::FUNCTIONARGS {
            argNames: __args_argNames,
            args: __args_args,
        } => {
            (outCache, outExp, outProperties) = elabCall(
                inCache,
                inEnv,
                func_name,
                __args_args.clone(),
                __args_argNames.clone(),
                &type_vars,
                inImplicit,
                inPrefix,
                inInfo,
            )?;
            (outExp, _) = ExpressionSimplify::simplify1(outExp)?;
            ()
        }
        Absyn::FunctionArgs::FOR_ITER_FARG {
            exp: __args_exp,
            iterType: __args_iterType,
            iterators: __args_iterators,
        } => {
            (outCache, outExp, outProperties) = elabCallReduction(
                inCache,
                inEnv,
                &func_name,
                __args_exp.clone(),
                __args_iterType.clone(),
                metamodelica::AsArg::as_arg(&__args_iterators),
                inImplicit,
                inDoVect,
                inPrefix,
                inInfo,
            )?;
            ()
        }
    });
    Ok((outCache, outExp, outProperties))
}

fn elabExp_Dot(
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
    mut inExp: metamodelica::Ref<Absyn::Exp>,
    mut inImplicit: bool,
    mut inDoVect: bool,
    mut inPrefix: DAE::Prefix,
    mut inInfo: SourceInfo,
) -> Result<(FCore::Cache, metamodelica::Ref<DAE::Exp>, DAE::Properties)> {
    let mut outCache: FCore::Cache = inCache.clone();
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    let mut outProperties: DAE::Properties;
    (outExp, outProperties) = (match &*inExp {
        Absyn::Exp::DOT {
            exp: __inExp_exp,
            index: __inExp_index,
        } => {
            let mut s: ArcStr;
            let mut ty: metamodelica::Ref<DAE::Type>;
            s = (::match_deref::match_deref! { match &(__inExp_index.clone()) {
                Deref @ Absyn::Exp::CREF { componentRef: Deref @ Absyn::ComponentRef::CREF_IDENT { name: __esc_s, .. } } => {
                    s = (*__esc_s).clone();
                    s.clone()
                },
                _ => {
                    Error::addSourceMessage(&(Error::COMPILER_ERROR.clone()), list![{ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Dot operator is only allowed when indexing using a single simple name, got: ")); __mm_s.push_str(&*Dump::printExpStr(__inExp_index.clone())?); ArcStr::from(__mm_s) }], &inInfo)?;
                    return Err("fail")
                },
                _ => unreachable!("match_deref! exhaustiveness placeholder"),
            } });
            (outCache, outExp, outProperties) = elabExp(
                inCache,
                inEnv,
                __inExp_exp.clone(),
                inImplicit,
                inDoVect,
                inPrefix,
                inInfo.clone(),
            )?;
            ty = Types::getPropType(&outProperties);
            let () = (::match_deref::match_deref! { match &(ty.clone()) {
                Deref @ DAE::Type::T_TUPLE { names: Some(names), types: __ty_types } => {
                    let mut i: i32;
                    if !(listMember(s.clone(), names.clone())) {
                        Error::addSourceMessage(&(Error::COMPILER_ERROR.clone()), list![{ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Dot operator could not find ")); __mm_s.push_str(&*s); __mm_s.push_str(&*literal!(" in ")); __mm_s.push_str(&*TypesDump::unparseType(ty)?); ArcStr::from(__mm_s) }], &inInfo)?;
                        return Err("fail");
                    }
                    i = List::position(s, metamodelica::AsArg::as_arg(&names))?;
                    outExp = metamodelica::Ref::new(DAE::Exp::TSUB { exp: outExp, ix: i, ty: (__ty_types).get(i)? });
                    outProperties = DAE::Properties::PROP { type_: (__ty_types).get(i)?, constFlag: Types::propAllConst(outProperties)? };
                    ()
                },
                _ => {
                    Error::addSourceMessage(&(Error::COMPILER_ERROR.clone()), list![{ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Dot operator is only allowed when the expression returns a named tuple. Got expression: ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(outExp.clone())?); __mm_s.push_str(&*literal!(" with type ")); __mm_s.push_str(&*TypesDump::unparseType(ty)?); ArcStr::from(__mm_s) }], &inInfo)?;
                    return Err("fail")
                },
                _ => unreachable!("match_deref! exhaustiveness placeholder"),
            } });
            (outExp, outProperties)
        }
        _ => return Err("match: no arm matched"),
    });
    Ok((outCache, outExp, outProperties))
}

fn elabExp_Comment(
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
    mut inExp: metamodelica::Ref<Absyn::Exp>,
    mut inImplicit: bool,
    mut inDoVect: bool,
    mut inPrefix: DAE::Prefix,
    mut inInfo: SourceInfo,
) -> Result<(FCore::Cache, metamodelica::Ref<DAE::Exp>, DAE::Properties)> {
    let mut outCache: FCore::Cache = inCache.clone();
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    let mut outProperties: DAE::Properties;
    let mut exp: metamodelica::Ref<Absyn::Exp>;
    let __pa0 = ::match_deref::match_deref! { match &(inExp) {
        Deref @ Absyn::Exp::EXPRESSIONCOMMENT { exp: __pa0, .. } => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    exp = metamodelica::Own::own(__pa0);
    (outCache, outExp, outProperties) = elabExp(inCache, inEnv, exp, inImplicit, inDoVect, inPrefix, inInfo)?;
    Ok((outCache, outExp, outProperties))
}

fn elabExp_PartEvalFunction(
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
    mut inExp: metamodelica::Ref<Absyn::Exp>,
    mut inImplicit: bool,
    mut inDoVect: bool,
    mut inPrefix: DAE::Prefix,
    mut inInfo: SourceInfo,
) -> Result<(FCore::Cache, metamodelica::Ref<DAE::Exp>, DAE::Properties)> {
    let mut outCache: FCore::Cache = inCache.clone();
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    let mut outProperties: DAE::Properties;
    let mut cref: metamodelica::Ref<Absyn::ComponentRef>;
    let mut pos_args: metamodelica::List<metamodelica::Ref<Absyn::Exp>>;
    let mut named_args: metamodelica::List<metamodelica::Ref<Absyn::NamedArg>>;
    let mut path: metamodelica::Ref<Absyn::Path>;
    let mut ty: metamodelica::Ref<DAE::Type>;
    let mut tty: metamodelica::Ref<DAE::Type>;
    let mut tty2: metamodelica::Ref<DAE::Type>;
    let mut args: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
    let mut consts: metamodelica::List<DAE::Const>;
    let mut slots: metamodelica::List<Slot>;
    let mut c: DAE::Const;
    let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(inExp) {
        Deref @ Absyn::Exp::PARTEVALFUNCTION { function_: __pa0, functionArgs: Deref @ Absyn::FunctionArgs::FUNCTIONARGS { args: __pa1, argNames: __pa2 } } => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
        _ => return Err("pattern mismatch"),
    } };
    cref = metamodelica::Own::own(__pa0);
    pos_args = metamodelica::Own::own(__pa1);
    named_args = metamodelica::Own::own(__pa2);
    if (pos_args).is_empty() && (named_args).is_empty() {
        (outCache, outExp, outProperties) = elabExpInExpression(
            inCache,
            inEnv,
            metamodelica::Ref::new(Absyn::Exp::CREF { componentRef: cref }),
            inImplicit,
            inDoVect,
            inPrefix,
            inInfo,
        )?;
    } else {
        path = AbsynUtil::crefToPath(&cref)?;
        let (__pa4, __pa5) = ::match_deref::match_deref! { match &(Lookup::lookupFunctionsInEnv(inCache, inEnv.clone(), path.clone(), inInfo.clone())) {
            (__pa4, Deref @ metamodelica::ListNode::Cons { head: __pa5, tail: Deref @ metamodelica::ListNode::Nil }) => (__pa4.clone(), __pa5.clone()),
            _ => return Err("pattern mismatch"),
        } };
        outCache = metamodelica::Own::own(__pa4);
        tty = metamodelica::Own::own(__pa5);
        tty = Types::makeFunctionPolymorphicReference(&tty)?;
        (outCache, args, consts, _, tty, _, slots) = elabTypes(
            outCache,
            inEnv.clone(),
            &pos_args,
            &named_args,
            &(metamodelica::nil()),
            list![tty],
            true,
            true,
            inImplicit,
            inPrefix,
            inInfo,
        )?;
        if !(Types::isFunctionPointer(&tty)) {
            (outCache, path) = Inst::makeFullyQualified(outCache, inEnv.clone(), path)?;
            let (__pa7, Util::SUCCESS { .. }) =
                (instantiateDaeFunction(outCache, inEnv, path.clone(), false, None, true))
            else {
                return Err("pattern mismatch");
            };
            outCache = metamodelica::Own::own(__pa7);
        }
        tty2 = stripExtraArgsFromType(&slots, tty.clone())?;
        tty2 = Types::makeFunctionPolymorphicReference(&tty2)?;
        ty = Types::simplifyType(tty2.clone())?;
        tty = Types::simplifyType(tty)?;
        c = List::fold(
            &consts,
            &fnptr!(Types::constAnd, DAE::Const, DAE::Const),
            openmodelica_frontend_types::DAE::Const::C_CONST,
        )?;
        outExp = metamodelica::Ref::new(DAE::Exp::PARTEVALFUNCTION {
            path: path,
            expList: args,
            ty: ty,
            origType: tty,
        });
        outProperties = DAE::Properties::PROP {
            type_: tty2,
            constFlag: c,
        };
    }
    Ok((outCache, outExp, outProperties))
}

fn elabExp_Tuple(
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
    mut inExp: metamodelica::Ref<Absyn::Exp>,
    mut inImplicit: bool,
    mut inDoVect: bool,
    mut inPrefix: DAE::Prefix,
    mut inInfo: SourceInfo,
) -> Result<(FCore::Cache, metamodelica::Ref<DAE::Exp>, DAE::Properties)> {
    let mut outCache: FCore::Cache = inCache.clone();
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    let mut outProperties: DAE::Properties;
    (outCache, outExp, outProperties) =
        elabExp_Tuple_LHS_RHS(inCache, inEnv, inExp, inImplicit, inDoVect, inPrefix, inInfo, false)?;
    Ok((outCache, outExp, outProperties))
}

fn elabExp_Tuple_LHS_RHS(
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
    mut inExp: metamodelica::Ref<Absyn::Exp>,
    mut inImplicit: bool,
    mut inDoVect: bool,
    mut inPrefix: DAE::Prefix,
    mut inInfo: SourceInfo,
    mut isLhs: bool,
) -> Result<(FCore::Cache, metamodelica::Ref<DAE::Exp>, DAE::Properties)> {
    let mut outCache: FCore::Cache = inCache;
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    let mut outProperties: DAE::Properties;
    let mut el: metamodelica::List<metamodelica::Ref<Absyn::Exp>>;
    let mut expl: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
    let mut props: metamodelica::List<DAE::Properties>;
    let mut types: metamodelica::List<metamodelica::Ref<DAE::Type>>;
    let mut consts: metamodelica::List<metamodelica::Ref<DAE::TupleConst>>;
    let __pa0 = ::match_deref::match_deref! { match &(inExp) {
        Deref @ Absyn::Exp::TUPLE { expressions: __pa0 } => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    el = metamodelica::Own::own(__pa0);
    if ((el).len() as i32) == 1 {
        (outCache, outExp, outProperties) =
            elabExp(outCache, inEnv, (el).get(1)?, inImplicit, inDoVect, inPrefix, inInfo)?;
        return Ok((outCache, outExp, outProperties));
    }
    (outCache, expl, props) = elabTuple(outCache, inEnv, el, inImplicit, inDoVect, inPrefix, inInfo, isLhs)?;
    (types, consts) = splitProps(props);
    (outExp, outProperties) = fixTupleMetaModelica(expl, types, consts)?;
    Ok((outCache, outExp, outProperties))
}

pub(crate) fn elabExpLHS(
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
    mut inExp: metamodelica::Ref<Absyn::Exp>,
    mut inImplicit: bool,
    mut inDoVect: bool,
    mut inPrefix: DAE::Prefix,
    mut inInfo: SourceInfo,
) -> Result<(FCore::Cache, metamodelica::Ref<DAE::Exp>, DAE::Properties)> {
    let mut outCache: FCore::Cache = inCache.clone();
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    let mut outProperties: DAE::Properties;
    (outCache, outExp, outProperties) = (match &*inExp {
        Absyn::Exp::TUPLE { .. } => {
            (outCache, outExp, outProperties) =
                elabExp_Tuple_LHS_RHS(inCache, inEnv, inExp, inImplicit, inDoVect, inPrefix, inInfo, true)?;
            (outCache, outExp, outProperties)
        }
        _ => {
            (outCache, outExp, outProperties) = elabExp(inCache, inEnv, inExp, inImplicit, inDoVect, inPrefix, inInfo)?;
            (outCache, outExp, outProperties)
        }
    });
    Ok((outCache, outExp, outProperties))
}

fn elabExp_Range(
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
    mut inExp: metamodelica::Ref<Absyn::Exp>,
    mut inImplicit: bool,
    mut inDoVect: bool,
    mut inPrefix: DAE::Prefix,
    mut inInfo: SourceInfo,
) -> Result<(FCore::Cache, metamodelica::Ref<DAE::Exp>, DAE::Properties)> {
    let mut outCache: FCore::Cache = inCache.clone();
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    let mut outProperties: DAE::Properties;
    let mut start: metamodelica::Ref<Absyn::Exp>;
    let mut step: metamodelica::Ref<Absyn::Exp>;
    let mut stop: metamodelica::Ref<Absyn::Exp>;
    let mut ostep: Option<metamodelica::Ref<Absyn::Exp>>;
    let mut start_exp: metamodelica::Ref<DAE::Exp>;
    let mut step_exp: metamodelica::Ref<DAE::Exp>;
    let mut stop_exp: metamodelica::Ref<DAE::Exp>;
    let mut ostep_exp: Option<metamodelica::Ref<DAE::Exp>> = None;
    let mut start_ty: metamodelica::Ref<DAE::Type>;
    let mut step_ty: metamodelica::Ref<DAE::Type>;
    let mut stop_ty: metamodelica::Ref<DAE::Type>;
    let mut ety: metamodelica::Ref<DAE::Type>;
    let mut ty: metamodelica::Ref<DAE::Type>;
    let mut ostep_ty: Option<metamodelica::Ref<DAE::Type>> = None;
    let mut start_c: DAE::Const;
    let mut step_c: DAE::Const;
    let mut stop_c: DAE::Const;
    let mut c: DAE::Const;
    let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(inExp) {
        Deref @ Absyn::Exp::RANGE { start: __pa0, step: __pa1, stop: __pa2 } => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
        _ => return Err("pattern mismatch"),
    } };
    start = metamodelica::Own::own(__pa0);
    ostep = metamodelica::Own::own(__pa1);
    stop = metamodelica::Own::own(__pa2);
    let (__pa3, __pa4, __pa5, __pa6) = ::match_deref::match_deref! { match &(elabExpInExpression(inCache, inEnv.clone(), start, inImplicit, inDoVect, inPrefix.clone(), inInfo.clone())?) {
        (__pa3, __pa4, DAE::Properties::PROP { type_: __pa5, constFlag: __pa6 }) => (__pa3.clone(), __pa4.clone(), __pa5.clone(), __pa6.clone()),
        _ => return Err("pattern mismatch"),
    } };
    outCache = metamodelica::Own::own(__pa3);
    start_exp = metamodelica::Own::own(__pa4);
    start_ty = metamodelica::Own::own(__pa5);
    start_c = metamodelica::Own::own(__pa6);
    let (__pa7, __pa8, __pa9, __pa10) = ::match_deref::match_deref! { match &(elabExpInExpression(outCache, inEnv.clone(), stop, inImplicit, inDoVect, inPrefix.clone(), inInfo.clone())?) {
        (__pa7, __pa8, DAE::Properties::PROP { type_: __pa9, constFlag: __pa10 }) => (__pa7.clone(), __pa8.clone(), __pa9.clone(), __pa10.clone()),
        _ => return Err("pattern mismatch"),
    } };
    outCache = metamodelica::Own::own(__pa7);
    stop_exp = metamodelica::Own::own(__pa8);
    stop_ty = metamodelica::Own::own(__pa9);
    stop_c = metamodelica::Own::own(__pa10);
    c = Types::constAnd(start_c, stop_c);
    if (ostep).is_some() {
        let __pa11 = ::match_deref::match_deref! { match &(ostep) {
            Some(__pa11) => __pa11.clone(),
            _ => return Err("pattern mismatch"),
        } };
        step = metamodelica::Own::own(__pa11);
        let (__pa12, __pa13, __pa14, __pa15) = ::match_deref::match_deref! { match &(elabExpInExpression(outCache, inEnv.clone(), step, inImplicit, inDoVect, inPrefix, inInfo.clone())?) {
            (__pa12, __pa13, DAE::Properties::PROP { type_: __pa14, constFlag: __pa15 }) => (__pa12.clone(), __pa13.clone(), __pa14.clone(), __pa15.clone()),
            _ => return Err("pattern mismatch"),
        } };
        outCache = metamodelica::Own::own(__pa12);
        step_exp = metamodelica::Own::own(__pa13);
        step_ty = metamodelica::Own::own(__pa14);
        step_c = metamodelica::Own::own(__pa15);
        ostep_exp = Some(step_exp);
        ostep_ty = Some(step_ty);
        c = Types::constAnd(c, step_c);
    }
    if Types::isBoxedType(&start_ty) {
        (start_exp, start_ty) = Types::matchType(start_exp, start_ty.clone(), Types::unboxedType(start_ty)?, true)?;
    }
    if Types::isBoxedType(&stop_ty) {
        (stop_exp, stop_ty) = Types::matchType(stop_exp, stop_ty.clone(), Types::unboxedType(stop_ty)?, true)?;
    }
    (start_exp, ostep_exp, stop_exp, ety) = deoverloadRange(
        start_exp,
        start_ty.clone(),
        ostep_exp,
        ostep_ty,
        stop_exp,
        stop_ty,
        &inInfo,
    )?;
    (outCache, ty) = elabRangeType(
        outCache,
        inEnv,
        start_exp.clone(),
        ostep_exp.clone(),
        stop_exp.clone(),
        start_ty,
        &ety,
        c,
        inImplicit,
    );
    outExp = metamodelica::Ref::new(DAE::Exp::RANGE {
        ty: ty.clone(),
        start: start_exp,
        step: ostep_exp,
        stop: stop_exp,
    });
    outProperties = DAE::Properties::PROP {
        type_: ty,
        constFlag: c,
    };
    Ok((outCache, outExp, outProperties))
}

fn elabExp_Array(
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
    mut inExp: metamodelica::Ref<Absyn::Exp>,
    mut inImplicit: bool,
    mut inDoVect: bool,
    mut inPrefix: DAE::Prefix,
    mut inInfo: SourceInfo,
) -> Result<(FCore::Cache, metamodelica::Ref<DAE::Exp>, DAE::Properties)> {
    let mut outCache: FCore::Cache = inCache.clone();
    let mut outExp: metamodelica::Ref<DAE::Exp> = <metamodelica::Ref<DAE::Exp> as ::std::default::Default>::default();
    let mut outProperties: DAE::Properties = <DAE::Properties as ::std::default::Default>::default();
    let mut es: metamodelica::List<metamodelica::Ref<Absyn::Exp>>;
    let mut expl: metamodelica::List<metamodelica::Ref<DAE::Exp>> = metamodelica::nil();
    let mut props: metamodelica::List<DAE::Properties> = metamodelica::nil();
    let mut ty: metamodelica::Ref<DAE::Type> = metamodelica::Ref::new(DAE::Type::T_NORETCALL);
    let mut arr_ty: metamodelica::Ref<DAE::Type> = metamodelica::Ref::new(DAE::Type::T_NORETCALL);
    let mut c: DAE::Const = DAE::Const::C_CONST;
    let mut exp: metamodelica::Ref<DAE::Exp> = <metamodelica::Ref<DAE::Exp> as ::std::default::Default>::default();
    (outExp, outProperties) = 'mc: {
        let __mc_input = inExp;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Absyn::Exp::ARRAY { arrayExp: Deref @ metamodelica::ListNode::Nil } => {
                    if !((Config::acceptMetaModelicaGrammar()?)) { return Err("guard") }
                    Ok((metamodelica::Ref::new(DAE::Exp::LIST { valList: metamodelica::nil() }), DAE::Properties::PROP { type_: DAE::T_METALIST_DEFAULT().clone(), constFlag: openmodelica_frontend_types::DAE::Const::C_CONST }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok((__v, __wb0, __wb1, __wb2, __wb3, __wb4, __wb5, __wb6)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Absyn::Exp::ARRAY { arrayExp: es } => {
                    let mut arr_ty: metamodelica::Ref<DAE::Type> = arr_ty.clone();
                    let mut c: DAE::Const = c.clone();
                    let mut exp: metamodelica::Ref<DAE::Exp> = exp.clone();
                    let mut expl: metamodelica::List<metamodelica::Ref<DAE::Exp>> = expl.clone();
                    let mut outCache: FCore::Cache = outCache.clone();
                    let mut props: metamodelica::List<DAE::Properties> = props.clone();
                    let mut ty: metamodelica::Ref<DAE::Type> = ty.clone();
                    (outCache, expl, props) = elabExpList(inCache.clone(), inEnv.clone(), metamodelica::AsArg::as_arg(&es), inImplicit, inDoVect, inPrefix.clone(), inInfo.clone(), DAE::T_UNKNOWN_DEFAULT().clone())?;
                    let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(elabArray(expl.clone(), &props, &inPrefix, &inInfo)?) {
                        (__pa0, DAE::Properties::PROP { type_: __pa1, constFlag: __pa2 }) => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    expl = metamodelica::Own::own(__pa0);
                    ty = metamodelica::Own::own(__pa1);
                    c = metamodelica::Own::own(__pa2);
                    arr_ty = metamodelica::Ref::new(DAE::Type::T_ARRAY { ty: ty.clone(), dims: list![metamodelica::Ref::new(DAE::Dimension::DIM_INTEGER { integer: ((expl).len() as i32) })] });
                    exp = metamodelica::Ref::new(DAE::Exp::ARRAY { ty: Types::simplifyType(arr_ty.clone())?, scalar: !(Types::isArray(&ty)), array: expl.clone() });
                    InstMeta::checkArrayType(&ty)?;
                    exp = elabMatrixToMatrixExp(exp.clone());
                    Ok(((exp.clone(), DAE::Properties::PROP { type_: arr_ty.clone(), constFlag: c }), arr_ty.clone(), c.clone(), exp.clone(), expl.clone(), outCache.clone(), props.clone(), ty.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            arr_ty = __wb0;
            c = __wb1;
            exp = __wb2;
            expl = __wb3;
            outCache = __wb4;
            props = __wb5;
            ty = __wb6;
            break 'mc __v;
        }
        if let Ok((__v, __wb0, __wb1, __wb2)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Absyn::Exp::ARRAY { arrayExp: es } => {
                    if !((Config::acceptMetaModelicaGrammar()?)) { return Err("guard") }
                    let mut outCache: FCore::Cache = outCache.clone();
                    let mut outExp: metamodelica::Ref<DAE::Exp> = outExp.clone();
                    let mut outProperties: DAE::Properties = outProperties.clone();
                    (outCache, outExp, outProperties) = elabExpInExpression(inCache.clone(), inEnv.clone(), metamodelica::Ref::new(Absyn::Exp::LIST { exps: es.clone() }), inImplicit, inDoVect, inPrefix.clone(), inInfo.clone())?;
                    Ok(((outExp.clone(), outProperties.clone()), outCache.clone(), outExp.clone(), outProperties.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            outCache = __wb0;
            outExp = __wb1;
            outProperties = __wb2;
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((outCache, outExp, outProperties))
}

fn elabExp_Matrix(
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
    mut inExp: metamodelica::Ref<Absyn::Exp>,
    mut inImplicit: bool,
    mut inDoVect: bool,
    mut inPrefix: DAE::Prefix,
    mut inInfo: SourceInfo,
) -> Result<(FCore::Cache, metamodelica::Ref<DAE::Exp>, DAE::Properties)> {
    let mut outCache: FCore::Cache = inCache.clone();
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    let mut outProperties: DAE::Properties;
    let mut ess: metamodelica::List<metamodelica::List<metamodelica::Ref<Absyn::Exp>>>;
    let mut dess: metamodelica::List<metamodelica::List<metamodelica::Ref<DAE::Exp>>>;
    let mut props: metamodelica::List<metamodelica::List<DAE::Properties>>;
    let mut tys: metamodelica::List<metamodelica::Ref<DAE::Type>>;
    let mut nmax: i32;
    let mut have_real: bool;
    let mut ty: metamodelica::Ref<DAE::Type>;
    let mut c: DAE::Const;
    let mut dim1: metamodelica::Ref<DAE::Dimension>;
    let mut dim2: metamodelica::Ref<DAE::Dimension>;
    let __pa0 = ::match_deref::match_deref! { match &(inExp) {
        Deref @ Absyn::Exp::MATRIX { matrix: __pa0 } => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    ess = metamodelica::Own::own(__pa0);
    (outCache, dess, props) = elabExpListList(
        inCache,
        inEnv.clone(),
        &ess,
        inImplicit,
        inDoVect,
        inPrefix.clone(),
        inInfo.clone(),
        DAE::T_UNKNOWN_DEFAULT().clone(),
    )?;
    tys = ({
        let mut __acc: metamodelica::List<metamodelica::Ref<DAE::Type>> = metamodelica::nil();
        for mut pl in (props.clone()).into_iter().cloned() {
            let __x = ({
                let mut __acc: metamodelica::List<metamodelica::Ref<DAE::Type>> = metamodelica::nil();
                for mut p in (pl.clone()).into_iter().cloned() {
                    let __x = Types::getPropType(&(p.clone()));
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            });
            __acc = __x.append(&__acc);
        }
        __acc
    });
    nmax = matrixConstrMaxDim(&tys);
    have_real = Types::containReal(&tys);
    if have_real {
        (dess, props) = List::threadMapList_2(&dess, props, &elabExp_Matrix_realCast)?;
    }
    let (__pa1, __pa2, __pa3, __pa4, __pa5, __pa6) = ::match_deref::match_deref! { match &(elabMatrixSemi(outCache, &inEnv, &dess, &props, inImplicit, have_real, nmax, inDoVect, inPrefix, &inInfo)?) {
        (__pa1, __pa2, DAE::Properties::PROP { type_: __pa3, constFlag: __pa4 }, __pa5, __pa6) => (__pa1.clone(), __pa2.clone(), __pa3.clone(), __pa4.clone(), __pa5.clone(), __pa6.clone()),
        _ => return Err("pattern mismatch"),
    } };
    outCache = metamodelica::Own::own(__pa1);
    outExp = metamodelica::Own::own(__pa2);
    ty = metamodelica::Own::own(__pa3);
    c = metamodelica::Own::own(__pa4);
    dim1 = metamodelica::Own::own(__pa5);
    dim2 = metamodelica::Own::own(__pa6);
    outExp = elabMatrixToMatrixExp(outExp);
    ty = Types::unliftArray(&(Types::unliftArray(&ty)?))?;
    ty = metamodelica::Ref::new(DAE::Type::T_ARRAY {
        ty: ty,
        dims: list![dim2],
    });
    ty = metamodelica::Ref::new(DAE::Type::T_ARRAY {
        ty: ty,
        dims: list![dim1],
    });
    outProperties = DAE::Properties::PROP {
        type_: ty,
        constFlag: c,
    };
    Ok((outCache, outExp, outProperties))
}

fn elabExp_Matrix_realCast(
    mut inExp: metamodelica::Ref<DAE::Exp>,
    mut inProperties: DAE::Properties,
) -> Result<(metamodelica::Ref<DAE::Exp>, DAE::Properties)> {
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    let mut outProperties: DAE::Properties;
    let mut ty: metamodelica::Ref<DAE::Type>;
    ty = Types::getPropType(&inProperties);
    if Types::isInteger(&ty) {
        ty = Types::setArrayElementType(&ty, &(DAE::T_REAL_DEFAULT().clone()));
        outProperties = Types::setPropType(&inProperties, ty.clone());
        ty = Types::simplifyType(ty)?;
        (outExp, _) = ExpressionSimplify::simplify1(metamodelica::Ref::new(DAE::Exp::CAST { ty: ty, exp: inExp }))?;
    } else {
        outExp = inExp;
        outProperties = inProperties;
    }
    Ok((outExp, outProperties))
}

fn elabExp_Code(
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
    mut inExp: metamodelica::Ref<Absyn::Exp>,
    mut inImplicit: bool,
    mut inDoVect: bool,
    mut inPrefix: DAE::Prefix,
    mut inInfo: SourceInfo,
) -> Result<(FCore::Cache, metamodelica::Ref<DAE::Exp>, DAE::Properties)> {
    let mut outCache: FCore::Cache = inCache;
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    let mut outProperties: DAE::Properties;
    let mut ty: metamodelica::Ref<DAE::Type>;
    let mut ty2: metamodelica::Ref<DAE::Type>;
    let mut cn: metamodelica::Ref<Absyn::CodeNode>;
    let __pa0 = ::match_deref::match_deref! { match &(inExp) {
        Deref @ Absyn::Exp::CODE { code: __pa0 } => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    cn = metamodelica::Own::own(__pa0);
    ty = elabCodeType(&cn)?;
    ty2 = Types::simplifyType(ty.clone())?;
    outExp = metamodelica::Ref::new(DAE::Exp::CODE { code: cn, ty: ty2 });
    outProperties = DAE::Properties::PROP {
        type_: ty,
        constFlag: openmodelica_frontend_types::DAE::Const::C_CONST,
    };
    Ok((outCache, outExp, outProperties))
}

fn elabExp_Cons(
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
    mut inExp: metamodelica::Ref<Absyn::Exp>,
    mut inImplicit: bool,
    mut inDoVect: bool,
    mut inPrefix: DAE::Prefix,
    mut inInfo: SourceInfo,
) -> Result<(FCore::Cache, metamodelica::Ref<DAE::Exp>, DAE::Properties)> {
    let mut outCache: FCore::Cache = inCache;
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    let mut outProperties: DAE::Properties;
    let mut e1: metamodelica::Ref<Absyn::Exp>;
    let mut e2: metamodelica::Ref<Absyn::Exp>;
    let mut exp1: metamodelica::Ref<DAE::Exp>;
    let mut exp2: metamodelica::Ref<DAE::Exp>;
    let mut prop1: DAE::Properties;
    let mut ty: metamodelica::Ref<DAE::Type>;
    let mut ty1: metamodelica::Ref<DAE::Type>;
    let mut ty2: metamodelica::Ref<DAE::Type>;
    let mut c1: DAE::Const;
    let mut c2: DAE::Const;
    let mut exp_str: ArcStr;
    let mut ty1_str: ArcStr;
    let mut ty2_str: ArcStr;
    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(inExp.clone()) {
        Deref @ Absyn::Exp::CONS { head: __pa0, rest: __pa1 } => (__pa0.clone(), __pa1.clone()),
        _ => return Err("pattern mismatch"),
    } };
    e1 = metamodelica::Own::own(__pa0);
    e2 = metamodelica::Own::own(__pa1);
    let (__pa2, __pa3) = ::match_deref::match_deref! { match &(MetaUtil::transformArrayNodesToListNodes(list![e1, e2])) {
        Deref @ metamodelica::ListNode::Cons { head: __pa2, tail: Deref @ metamodelica::ListNode::Cons { head: __pa3, tail: Deref @ metamodelica::ListNode::Nil } } => (__pa2.clone(), __pa3.clone()),
        _ => return Err("pattern mismatch"),
    } };
    e1 = metamodelica::Own::own(__pa2);
    e2 = metamodelica::Own::own(__pa3);
    (outCache, exp1, prop1) = elabExpInExpression(
        outCache,
        inEnv.clone(),
        e1,
        inImplicit,
        inDoVect,
        inPrefix.clone(),
        inInfo.clone(),
    )?;
    let (__pa5, __pa6, __pa7, __pa8) = ::match_deref::match_deref! { match &(elabExpInExpression(outCache, inEnv, e2, inImplicit, inDoVect, inPrefix, inInfo.clone())?) {
        (__pa5, __pa6, DAE::Properties::PROP { type_: Deref @ DAE::Type::T_METALIST { ty: __pa7 }, constFlag: __pa8 }) => (__pa5.clone(), __pa6.clone(), __pa7.clone(), __pa8.clone()),
        _ => return Err("pattern mismatch"),
    } };
    outCache = metamodelica::Own::own(__pa5);
    exp2 = metamodelica::Own::own(__pa6);
    ty2 = metamodelica::Own::own(__pa7);
    c2 = metamodelica::Own::own(__pa8);
    match '__try10: {
        ty1 =
            unwrap_break_err!(Types::getUniontypeIfMetarecordReplaceAllSubtypes(Types::getPropType(&prop1)), '__try10);
        ty2 = unwrap_break_err!(Types::getUniontypeIfMetarecordReplaceAllSubtypes(ty2.clone()), '__try10);
        c1 = unwrap_break_err!(Types::propAllConst(prop1.clone()), '__try10);
        ty = unwrap_break_err!(Types::getUniontypeIfMetarecordReplaceAllSubtypes(unwrap_break_err!(Types::superType(Types::boxIfUnboxedType(ty1.clone()), Types::boxIfUnboxedType(ty2.clone())), '__try10)), '__try10);
        (exp1, _) = unwrap_break_err!(Types::matchType(exp1.clone(), ty1.clone(), ty.clone(), true), '__try10);
        ty = metamodelica::Ref::new(DAE::Type::T_METALIST { ty: ty.clone() });
        (exp2, _) = unwrap_break_err!(Types::matchType(exp2.clone(), ty.clone(), metamodelica::Ref::new(DAE::Type::T_METALIST { ty: ty2.clone() }), true), '__try10);
        outExp = metamodelica::Ref::new(DAE::Exp::CONS {
            car: exp1.clone(),
            cdr: exp2.clone(),
        });
        outProperties = DAE::Properties::PROP {
            type_: ty.clone(),
            constFlag: Types::constAnd(c1, c2),
        };
        Ok::<_, &'static str>((
            c1.clone(),
            exp1.clone(),
            exp2.clone(),
            outExp.clone(),
            outProperties.clone(),
            ty.clone(),
            ty1.clone(),
            ty2.clone(),
        ))
    } {
        Ok((__try10_o0, __try10_o1, __try10_o2, __try10_o3, __try10_o4, __try10_o5, __try10_o6, __try10_o7)) => {
            c1 = __try10_o0;
            exp1 = __try10_o1;
            exp2 = __try10_o2;
            outExp = __try10_o3;
            outProperties = __try10_o4;
            ty = __try10_o5;
            ty1 = __try10_o6;
            ty2 = __try10_o7;
        }
        Err(__try10_err) => {
            exp_str = Dump::printExpStr(inExp.clone())?;
            ty1_str = TypesDump::unparseType(Types::getPropType(&prop1))?;
            ty2_str = TypesDump::unparseType(ty2.clone())?;
            Error::addSourceMessage(
                &(Error::META_CONS_TYPE_MATCH.clone()),
                list![exp_str.clone(), ty1_str.clone(), ty2_str.clone()],
                &inInfo,
            )?;
            return Err(__try10_err);
        }
    }
    Ok((outCache, outExp, outProperties))
}

fn elabExp_List(
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
    mut inExp: metamodelica::Ref<Absyn::Exp>,
    mut inImplicit: bool,
    mut inDoVect: bool,
    mut inPrefix: DAE::Prefix,
    mut inInfo: SourceInfo,
) -> Result<(FCore::Cache, metamodelica::Ref<DAE::Exp>, DAE::Properties)> {
    let mut outCache: FCore::Cache = inCache.clone();
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    let mut outProperties: DAE::Properties;
    let mut es: metamodelica::List<metamodelica::Ref<Absyn::Exp>>;
    let mut expl: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
    let mut props: metamodelica::List<DAE::Properties>;
    let mut types: metamodelica::List<metamodelica::Ref<DAE::Type>>;
    let mut consts: metamodelica::List<DAE::Const>;
    let mut c: DAE::Const;
    let mut ty: metamodelica::Ref<DAE::Type>;
    let __pa0 = ::match_deref::match_deref! { match &(inExp) {
        Deref @ Absyn::Exp::LIST { exps: __pa0 } => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    es = metamodelica::Own::own(__pa0);
    if (es).is_empty() {
        outExp = metamodelica::Ref::new(DAE::Exp::LIST {
            valList: metamodelica::nil(),
        });
        outProperties = DAE::Properties::PROP {
            type_: DAE::T_METALIST_DEFAULT().clone(),
            constFlag: openmodelica_frontend_types::DAE::Const::C_CONST,
        };
    } else {
        (outCache, expl, props) = elabExpList(
            inCache,
            inEnv,
            &es,
            inImplicit,
            inDoVect,
            inPrefix,
            inInfo,
            DAE::T_UNKNOWN_DEFAULT().clone(),
        )?;
        types = ({
            let mut __acc: metamodelica::List<metamodelica::Ref<DAE::Type>> = metamodelica::nil();
            for mut p in (props.clone()).into_iter().cloned() {
                let __x = Types::getPropType(&(p.clone()));
                __acc = cons(__x, __acc);
            }
            __acc.reverse()
        });
        consts = Types::getConstList(&props)?;
        c = List::fold(
            &consts,
            &fnptr!(Types::constAnd, DAE::Const, DAE::Const),
            openmodelica_frontend_types::DAE::Const::C_CONST,
        )?;
        ty = Types::boxIfUnboxedType(List::reduce(&types, &Types::superType)?);
        (expl, _) = Types::matchTypes(expl, types, &ty, true)?;
        outExp = metamodelica::Ref::new(DAE::Exp::LIST { valList: expl });
        outProperties = DAE::Properties::PROP {
            type_: metamodelica::Ref::new(DAE::Type::T_METALIST { ty: ty }),
            constFlag: c,
        };
    }
    Ok((outCache, outExp, outProperties))
}

pub(crate) fn elabExpInExpression(
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
    mut inExp: metamodelica::Ref<Absyn::Exp>,
    mut inImplicit: bool,
    mut performVectorization: bool,
    mut inPrefix: DAE::Prefix,
    mut info: SourceInfo,
) -> Result<(FCore::Cache, metamodelica::Ref<DAE::Exp>, DAE::Properties)> {
    let mut outCache: FCore::Cache;
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    let mut outProperties: DAE::Properties;
    (outCache, outExp, outProperties) =
        elabExp(inCache, inEnv, inExp, inImplicit, performVectorization, inPrefix, info)?;
    (outExp, outProperties) = elabExpInExpression2(outExp, outProperties);
    Ok((outCache, outExp, outProperties))
}

fn elabExpInExpression2(
    mut inExp: metamodelica::Ref<DAE::Exp>,
    mut inProperties: DAE::Properties,
) -> (metamodelica::Ref<DAE::Exp>, DAE::Properties) {
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    let mut outProperties: DAE::Properties;
    (outExp, outProperties) = (::match_deref::match_deref! { match &(inProperties.clone()) {
        DAE::Properties::PROP_TUPLE { type_: Deref @ DAE::Type::T_TUPLE { types: Deref @ metamodelica::ListNode::Cons { head: ty, tail: _ }, .. }, tupleConst: Deref @ DAE::TupleConst::TUPLE_CONST { tupleConstLst: Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::TupleConst::SINGLE_CONST { r#const: c }, tail: _ } } } => {
            (metamodelica::Ref::new(DAE::Exp::TSUB { exp: inExp, ix: 1, ty: ty.clone() }), DAE::Properties::PROP { type_: ty.clone(), constFlag: c.clone() })
        },
        _ => {
            (inExp, inProperties)
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    (outExp, outProperties)
}

pub(crate) fn checkAssignmentToInput(
    mut inExp: &metamodelica::Ref<Absyn::Exp>,
    mut inAttributes: &metamodelica::Ref<DAE::Attributes>,
    mut inEnv: &FCore::Graph,
    mut inAllowTopLevelInputs: bool,
    mut inInfo: &SourceInfo,
) -> Result<()> {
    if !(inAllowTopLevelInputs) && FGraph::inFunctionScope(inEnv) && !(Config::acceptParModelicaGrammar()?) {
        checkAssignmentToInput2(inExp, inAttributes, inInfo)?;
    }
    Ok(())
}

fn checkAssignmentToInput2(
    mut inExp: &metamodelica::Ref<Absyn::Exp>,
    mut inAttributes: &metamodelica::Ref<DAE::Attributes>,
    mut inInfo: &SourceInfo,
) -> Result<()> {
    let () = (::match_deref::match_deref! { match (inExp, inAttributes) {
        (Deref @ Absyn::Exp::CREF { componentRef: cr }, Deref @ DAE::Attributes { direction: Absyn::Direction::INPUT { .. }, .. }) => {
            let mut cr_str: ArcStr;
            cr_str = Dump::printComponentRefStr(cr)?;
            Error::addSourceMessage(&(Error::ASSIGN_READONLY_ERROR.clone()), list![literal!("input"), cr_str], inInfo)?;
            return Err("fail")
        },
        _ => {
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(())
}

pub(crate) fn checkAssignmentToInputs(
    mut inExpCrefs: &metamodelica::List<metamodelica::Ref<Absyn::Exp>>,
    mut inAttributes: metamodelica::List<metamodelica::Ref<DAE::Attributes>>,
    mut inEnv: &FCore::Graph,
    mut inInfo: SourceInfo,
) -> Result<()> {
    if FGraph::inFunctionScope(inEnv) {
        List::threadMap1_0(
            inExpCrefs,
            inAttributes,
            &move |__a0: metamodelica::Ref<Absyn::Exp>, __a1: metamodelica::Ref<DAE::Attributes>, __a2: SourceInfo| {
                checkAssignmentToInput2(&__a0, &__a1, &__a2)
            },
            inInfo,
        )?;
    }
    Ok(())
}

pub(crate) fn elabExpCrefNoEvalList(
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
    mut inExpl: &metamodelica::List<metamodelica::Ref<Absyn::Exp>>,
    mut inImplicit: bool,
    mut inDoVect: bool,
    mut inPrefix: DAE::Prefix,
    mut inInfo: SourceInfo,
) -> Result<(
    FCore::Cache,
    metamodelica::List<metamodelica::Ref<DAE::Exp>>,
    metamodelica::List<DAE::Properties>,
    metamodelica::List<metamodelica::Ref<DAE::Attributes>>,
)> {
    let mut outCache: FCore::Cache = inCache;
    let mut outExpl: metamodelica::List<metamodelica::Ref<DAE::Exp>> = metamodelica::nil();
    let mut outProperties: metamodelica::List<DAE::Properties> = metamodelica::nil();
    let mut outAttributes: metamodelica::List<metamodelica::Ref<DAE::Attributes>> = metamodelica::nil();
    let mut num_err: i32 = Error::getNumErrorMessages();
    let mut exp: metamodelica::Ref<DAE::Exp>;
    let mut prop: DAE::Properties;
    let mut props: metamodelica::List<DAE::Properties> = metamodelica::nil();
    let mut attr: metamodelica::Ref<DAE::Attributes>;
    let mut cr: metamodelica::Ref<Absyn::ComponentRef>;
    let mut ty: metamodelica::Ref<DAE::Type>;
    let mut c: DAE::Const;
    for mut e in &**inExpl {
        if '__try0: {
            let __pa1 = ::match_deref::match_deref! { match &(e.clone()) {
                Deref @ Absyn::Exp::CREF { componentRef: __pa1 } => __pa1.clone(),
                _ => break '__try0 Err::<_, _>("pattern mismatch"),
            } };
            cr = metamodelica::Own::own(__pa1);
            (outCache, exp, prop, attr) = unwrap_break_err!(elabCrefNoEval(outCache.clone(), inEnv.clone(), cr.clone(), inImplicit, inDoVect, inPrefix.clone(), inInfo.clone()), '__try0);
            outExpl = metamodelica::cons(exp.clone(), outExpl.clone());
            outAttributes = metamodelica::cons(attr.clone(), outAttributes.clone());
            props = metamodelica::cons(prop.clone(), props.clone());
            Ok::<(), &'static str>(())
        }.is_err() {
            let true = (num_err == Error::getNumErrorMessages()) else { return Err("pattern mismatch") };
            Error::addSourceMessage(&(Error::GENERIC_ELAB_EXPRESSION.clone()), list![Dump::printExpStr(e.clone())?], &inInfo)?;
        }
    }
    if !(Flags::getConfigBool(Flags::CEVAL_EQUATION.clone())?) {
        for mut p in &*props {
            let mut p = p.clone();
            let DAE::PROP {
                type_: __pa2,
                constFlag: __pa3,
            } = (p.clone())
            else {
                return Err("pattern mismatch");
            };
            ty = metamodelica::Own::own(__pa2);
            c = metamodelica::Own::own(__pa3);
            p = if (Types::isParameter(c)) {
                DAE::Properties::PROP {
                    type_: ty,
                    constFlag: openmodelica_frontend_types::DAE::Const::C_VAR,
                }
            } else {
                p
            };
            outProperties = metamodelica::cons(p, outProperties);
        }
    } else {
        outProperties = props.reverse();
    }
    outExpl = outExpl.reverse();
    outAttributes = outAttributes.reverse();
    Ok((outCache, outExpl, outProperties, outAttributes))
}

// Part of MetaModelica extension
pub(crate) fn elabListExp(
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
    mut inExpList: &metamodelica::List<metamodelica::Ref<Absyn::Exp>>,
    mut inProp: DAE::Properties,
    mut inImplicit: bool,
    mut inDoVect: bool,
    mut inPrefix: DAE::Prefix,
    mut inInfo: SourceInfo,
) -> Result<(FCore::Cache, metamodelica::Ref<DAE::Exp>, DAE::Properties)> {
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    let mut outProperties: DAE::Properties = <DAE::Properties as ::std::default::Default>::default();
    (outCache, outExp, outProperties) = 'mc: {
        let __mc_input = &**inExpList;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Nil => {
                    Ok((inCache.clone(), metamodelica::Ref::new(DAE::Exp::LIST { valList: metamodelica::nil() }), inProp.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok((__v, __wb0, __wb1)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                        _ => {
                            let mut expl: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                            let mut props: metamodelica::List<DAE::Properties>;
                            let mut types: metamodelica::List<metamodelica::Ref<DAE::Type>>;
                            let mut c: DAE::Const;
                            let mut ty: metamodelica::Ref<DAE::Type>;
                            let mut outCache: FCore::Cache = outCache.clone();
                            let mut outProperties: DAE::Properties = outProperties.clone();
                            let __pa0 = ::match_deref::match_deref! { match &(inProp.clone()) {
                                DAE::Properties::PROP { type_: Deref @ DAE::Type::T_METALIST { .. }, constFlag: __pa0 } => __pa0.clone(),
                                _ => return Err("pattern mismatch"),
                            } };
                            c = metamodelica::Own::own(__pa0);
                            (outCache, expl, props) = elabExpList(inCache.clone(), inEnv.clone(), inExpList, inImplicit, inDoVect, inPrefix.clone(), inInfo.clone(), DAE::T_UNKNOWN_DEFAULT().clone())?;
                            types = ({
                let mut __acc: metamodelica::List<metamodelica::Ref<DAE::Type>> = metamodelica::nil();
                for mut p in (props.clone()).into_iter().cloned() {
                            let __x = Types::getPropType(&(p.clone()));
                            __acc = cons(__x, __acc);
                }
                __acc.reverse()
            });
                            (expl, ty) = Types::listMatchSuperType(&expl, &types, true)?;
                            outProperties = DAE::Properties::PROP { type_: metamodelica::Ref::new(DAE::Type::T_METALIST { ty: ty.clone() }), constFlag: c };
                            Ok(((outCache.clone(), metamodelica::Ref::new(DAE::Exp::LIST { valList: expl.clone() }), outProperties.clone()), outCache.clone(), outProperties.clone()))
                        }
                        _ => return Err("nomatch"),
                    }}
        })() {
            outCache = __wb0;
            outProperties = __wb1;
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { return Err("pattern mismatch") };
                    Debug::traceln(literal!("- Static.elabListExp failed, non-matching args in list constructor?"))?;
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((outCache, outExp, outProperties))
}

/* ------------------------------- */
pub fn fromEquationsToAlgAssignments(
    mut cp: metamodelica::Ref<Absyn::ClassPart>,
) -> Result<metamodelica::List<metamodelica::Ref<Absyn::AlgorithmItem>>> {
    let mut algsOut: metamodelica::List<metamodelica::Ref<Absyn::AlgorithmItem>>;
    algsOut = (match &*cp {
        Absyn::ClassPart::ALGORITHMS { contents: alg } => alg.clone(),
        Absyn::ClassPart::EQUATIONS { contents: rest } => {
            fromEquationsToAlgAssignmentsWork(metamodelica::AsArg::as_arg(&rest))?
        }
        _ => {
            let mut r#str: ArcStr;
            r#str = Dump::unparseClassPart(cp)?;
            Error::addInternalError(
                {
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!(
                        "Static.fromEquationsToAlgAssignments: Unknown classPart in match expression:\n"
                    ));
                    __mm_s.push_str(&*r#str);
                    ArcStr::from(__mm_s)
                },
                metamodelica::sourceInfo!("FrontEnd/Static.mo"),
            )?;
            return Err("fail");
        }
    });
    Ok(algsOut)
}

fn fromEquationsToAlgAssignmentsWork(
    mut eqsIn: &metamodelica::List<metamodelica::Ref<Absyn::EquationItem>>,
) -> Result<metamodelica::List<metamodelica::Ref<Absyn::AlgorithmItem>>> {
    let mut algsOut: metamodelica::List<metamodelica::Ref<Absyn::AlgorithmItem>> = metamodelica::nil();
    for mut ei in &**eqsIn {
        let () = (match &*ei.clone() {
            Absyn::EquationItem::EQUATIONITEM {
                equation_: eq,
                comment,
                info,
            } => {
                let mut algs: metamodelica::List<metamodelica::Ref<Absyn::AlgorithmItem>>;
                algs = fromEquationToAlgAssignment(
                    metamodelica::AsArg::as_arg(&eq),
                    comment.clone(),
                    metamodelica::AsArg::as_arg(&info),
                )?;
                algsOut = listAppend(algs, algsOut);
                ()
            }
            Absyn::EquationItem::EQUATIONITEMCOMMENT { .. } => (),
        });
    }
    algsOut = algsOut.reverse();
    Ok(algsOut)
}

fn fromEquationBranchesToAlgBranches(
    mut eqsIn: &metamodelica::List<(
        metamodelica::Ref<Absyn::Exp>,
        metamodelica::List<metamodelica::Ref<Absyn::EquationItem>>,
    )>,
) -> Result<
    metamodelica::List<(
        metamodelica::Ref<Absyn::Exp>,
        metamodelica::List<metamodelica::Ref<Absyn::AlgorithmItem>>,
    )>,
> {
    let mut algsOut: metamodelica::List<(
        metamodelica::Ref<Absyn::Exp>,
        metamodelica::List<metamodelica::Ref<Absyn::AlgorithmItem>>,
    )> = metamodelica::nil();
    let mut e: metamodelica::Ref<Absyn::Exp>;
    let mut eqs: metamodelica::List<metamodelica::Ref<Absyn::EquationItem>>;
    let mut algs: metamodelica::List<metamodelica::Ref<Absyn::AlgorithmItem>>;
    for mut branch in &**eqsIn {
        (e, eqs) = branch.clone();
        algs = fromEquationsToAlgAssignmentsWork(&eqs)?;
        algsOut = metamodelica::cons((e, algs), algsOut);
    }
    algsOut = algsOut.reverse();
    Ok(algsOut)
}

fn fromEquationToAlgAssignment(
    mut eq: &metamodelica::Ref<Absyn::Equation>,
    mut comment: Option<metamodelica::Ref<Absyn::Comment>>,
    mut info: &SourceInfo,
) -> Result<metamodelica::List<metamodelica::Ref<Absyn::AlgorithmItem>>> {
    let mut algStatement: metamodelica::List<metamodelica::Ref<Absyn::AlgorithmItem>>;
    algStatement = 'mc: {
        let __mc_input = &**eq;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Absyn::Equation::EQ_EQUALS { leftSide: Deref @ Absyn::Exp::CREF { componentRef: Deref @ Absyn::ComponentRef::CREF_IDENT { name: strLeft, subscripts: Deref @ metamodelica::ListNode::Nil } }, rightSide: Deref @ Absyn::Exp::CREF { componentRef: Deref @ Absyn::ComponentRef::CREF_IDENT { name: strRight, subscripts: Deref @ metamodelica::ListNode::Nil } } } => {
                    let true = (metamodelica::stringEq(&strLeft, &strRight)) else { return Err("pattern mismatch") };
                    Ok(metamodelica::nil())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Absyn::Equation::EQ_EQUALS { leftSide: left, rightSide: right } => {
                    let mut algItem1: metamodelica::Ref<Absyn::AlgorithmItem>;
                    let mut algItem2: metamodelica::Ref<Absyn::AlgorithmItem>;
                    ::match_deref::match_deref! { match &(AbsynUtil::stripCommentExpressions(right.clone(), false)?) {
                        Deref @ Absyn::Exp::BOOL { value: true } => (),
                        _ => return Err("pattern mismatch"),
                    } };
                    if '__try0: {
                        ::match_deref::match_deref! { match &(left.clone()) {
                            Deref @ Absyn::Exp::CREF { componentRef: _ } => (),
                            _ => break '__try0 Err::<_, _>("pattern mismatch"),
                        } };
                        Ok::<(), &'static str>(())
                    }.is_ok() { return Err("failure(): body succeeded") }
                    algItem1 = metamodelica::Ref::new(Absyn::AlgorithmItem::ALGORITHMITEM { algorithm_: metamodelica::Ref::new(Absyn::Algorithm::ALG_NORETCALL { functionCall: metamodelica::Ref::new(Absyn::ComponentRef::CREF_IDENT { name: literal!("fail"), subscripts: metamodelica::nil() }), functionArgs: metamodelica::Ref::new(Absyn::FunctionArgs::FUNCTIONARGS { args: metamodelica::nil(), argNames: metamodelica::nil() }) }), comment: comment.clone(), info: info.clone() });
                    algItem2 = metamodelica::Ref::new(Absyn::AlgorithmItem::ALGORITHMITEM { algorithm_: metamodelica::Ref::new(Absyn::Algorithm::ALG_IF { ifExp: metamodelica::Ref::new(Absyn::Exp::LUNARY { op: openmodelica_ast::Absyn::Operator::NOT, exp: left.clone() }), trueBranch: list![algItem1.clone()], elseIfAlgorithmBranch: metamodelica::nil(), elseBranch: metamodelica::nil() }), comment: comment.clone(), info: info.clone() });
                    Ok(list![algItem2.clone()])
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Absyn::Equation::EQ_EQUALS { leftSide: left, rightSide: Deref @ Absyn::Exp::BOOL { value: false } } => {
                    let mut algItem1: metamodelica::Ref<Absyn::AlgorithmItem>;
                    let mut algItem2: metamodelica::Ref<Absyn::AlgorithmItem>;
                    if '__try0: {
                        ::match_deref::match_deref! { match &(left.clone()) {
                            Deref @ Absyn::Exp::CREF { componentRef: _ } => (),
                            _ => break '__try0 Err::<_, _>("pattern mismatch"),
                        } };
                        Ok::<(), &'static str>(())
                    }.is_ok() { return Err("failure(): body succeeded") }
                    algItem1 = metamodelica::Ref::new(Absyn::AlgorithmItem::ALGORITHMITEM { algorithm_: metamodelica::Ref::new(Absyn::Algorithm::ALG_NORETCALL { functionCall: metamodelica::Ref::new(Absyn::ComponentRef::CREF_IDENT { name: literal!("fail"), subscripts: metamodelica::nil() }), functionArgs: metamodelica::Ref::new(Absyn::FunctionArgs::FUNCTIONARGS { args: metamodelica::nil(), argNames: metamodelica::nil() }) }), comment: comment.clone(), info: info.clone() });
                    algItem2 = metamodelica::Ref::new(Absyn::AlgorithmItem::ALGORITHMITEM { algorithm_: metamodelica::Ref::new(Absyn::Algorithm::ALG_IF { ifExp: left.clone(), trueBranch: list![algItem1.clone()], elseIfAlgorithmBranch: metamodelica::nil(), elseBranch: metamodelica::nil() }), comment: comment.clone(), info: info.clone() });
                    Ok(list![algItem2.clone()])
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Absyn::Equation::EQ_PDE { .. } => {
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Absyn::Equation::EQ_NORETCALL { functionName: Deref @ Absyn::ComponentRef::CREF_IDENT { name: Deref @ "fail", subscripts: _ }, functionArgs: _ } => {
                    let mut algItem: metamodelica::Ref<Absyn::AlgorithmItem>;
                    algItem = metamodelica::Ref::new(Absyn::AlgorithmItem::ALGORITHMITEM { algorithm_: metamodelica::Ref::new(Absyn::Algorithm::ALG_NORETCALL { functionCall: metamodelica::Ref::new(Absyn::ComponentRef::CREF_IDENT { name: literal!("fail"), subscripts: metamodelica::nil() }), functionArgs: metamodelica::Ref::new(Absyn::FunctionArgs::FUNCTIONARGS { args: metamodelica::nil(), argNames: metamodelica::nil() }) }), comment: comment.clone(), info: info.clone() });
                    Ok(list![algItem.clone()])
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Absyn::Equation::EQ_NORETCALL { functionName: cref, functionArgs: fargs } => {
                    let mut algItem: metamodelica::Ref<Absyn::AlgorithmItem>;
                    algItem = metamodelica::Ref::new(Absyn::AlgorithmItem::ALGORITHMITEM { algorithm_: metamodelica::Ref::new(Absyn::Algorithm::ALG_NORETCALL { functionCall: cref.clone(), functionArgs: fargs.clone() }), comment: comment.clone(), info: info.clone() });
                    Ok(list![algItem.clone()])
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Absyn::Equation::EQ_EQUALS { leftSide: left, rightSide: right } => {
                    let mut algItem: metamodelica::Ref<Absyn::AlgorithmItem>;
                    algItem = metamodelica::Ref::new(Absyn::AlgorithmItem::ALGORITHMITEM { algorithm_: metamodelica::Ref::new(Absyn::Algorithm::ALG_ASSIGN { assignComponent: left.clone(), value: right.clone() }), comment: comment.clone(), info: info.clone() });
                    Ok(list![algItem.clone()])
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Absyn::Equation::EQ_FAILURE { equ: Deref @ Absyn::EquationItem::EQUATIONITEM { equation_: eq2, comment: comment2, info: info2 } } => {
                    let mut res: metamodelica::Ref<Absyn::AlgorithmItem>;
                    let mut algs: metamodelica::List<metamodelica::Ref<Absyn::AlgorithmItem>>;
                    algs = fromEquationToAlgAssignment(metamodelica::AsArg::as_arg(&eq2), comment2.clone(), metamodelica::AsArg::as_arg(&info2))?;
                    res = metamodelica::Ref::new(Absyn::AlgorithmItem::ALGORITHMITEM { algorithm_: metamodelica::Ref::new(Absyn::Algorithm::ALG_FAILURE { equ: algs.clone() }), comment: comment.clone(), info: info.clone() });
                    Ok(list![res.clone()])
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Absyn::Equation::EQ_IF { ifExp: e, equationTrueItems: eqTrueItems, elseIfBranches: eqBranches, equationElseItems: eqElseItems } => {
                    let mut res: metamodelica::Ref<Absyn::AlgorithmItem>;
                    let mut algTrueItems: metamodelica::List<metamodelica::Ref<Absyn::AlgorithmItem>>;
                    let mut algElseItems: metamodelica::List<metamodelica::Ref<Absyn::AlgorithmItem>>;
                    let mut algBranches: metamodelica::List<(metamodelica::Ref<Absyn::Exp>, metamodelica::List<metamodelica::Ref<Absyn::AlgorithmItem>>)>;
                    algTrueItems = fromEquationsToAlgAssignmentsWork(metamodelica::AsArg::as_arg(&eqTrueItems))?;
                    algElseItems = fromEquationsToAlgAssignmentsWork(metamodelica::AsArg::as_arg(&eqElseItems))?;
                    algBranches = fromEquationBranchesToAlgBranches(metamodelica::AsArg::as_arg(&eqBranches))?;
                    res = metamodelica::Ref::new(Absyn::AlgorithmItem::ALGORITHMITEM { algorithm_: metamodelica::Ref::new(Absyn::Algorithm::ALG_IF { ifExp: e.clone(), trueBranch: algTrueItems.clone(), elseIfAlgorithmBranch: algBranches.clone(), elseBranch: algElseItems.clone() }), comment: comment.clone(), info: info.clone() });
                    Ok(list![res.clone()])
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut r#str: ArcStr;
                    r#str = Dump::equationName(eq)?;
                    Error::addSourceMessage(&(Error::META_MATCH_EQUATION_FORBIDDEN.clone()), list![r#str.clone()], info)?;
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(algStatement)
}

fn elabMatrixToMatrixExp(mut inExp: metamodelica::Ref<DAE::Exp>) -> metamodelica::Ref<DAE::Exp> {
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    outExp = 'mc: {
        let __mc_input = &*inExp;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::ARRAY { ty: a @ Deref @ DAE::Type::T_ARRAY { dims: Deref @ metamodelica::ListNode::Cons { head: _, tail: Deref @ metamodelica::ListNode::Cons { head: _, tail: Deref @ metamodelica::ListNode::Nil } }, .. }, array: expl, .. } => {
                    let mut mexpl: metamodelica::List<metamodelica::List<metamodelica::Ref<DAE::Exp>>>;
                    let mut d1: i32;
                    mexpl = List::map(expl.clone(), &move |__a0: metamodelica::Ref<DAE::Exp>| Expression::arrayContent(&__a0))?;
                    d1 = ((mexpl).len() as i32);
                    let true = (Expression::typeBuiltin(&(Expression::unliftArray(&(Expression::unliftArray(metamodelica::AsArg::as_arg(&a))?))?))) else { return Err("pattern mismatch") };
                    Ok(metamodelica::Ref::new(DAE::Exp::MATRIX { ty: a.clone(), integer: d1, matrix: mexpl.clone() }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(inExp.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    outExp
}

fn matrixConstrMaxDim(mut inTypes: &metamodelica::List<metamodelica::Ref<DAE::Type>>) -> i32 {
    let mut outMaxDim: i32 = 2;
    for mut ty in &**inTypes {
        outMaxDim = std::cmp::max(Types::numberOfDimensions(metamodelica::AsArg::as_arg(&ty)), outMaxDim);
    }
    outMaxDim
}

fn elabCallReduction(
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
    mut inReductionFn: &metamodelica::Ref<Absyn::ComponentRef>,
    mut inReductionExp: metamodelica::Ref<Absyn::Exp>,
    mut inIterType: Absyn::ReductionIterType,
    mut inIterators: &metamodelica::List<metamodelica::Ref<Absyn::ForIterator>>,
    mut inImplicit: bool,
    mut inDoVect: bool,
    mut inPrefix: DAE::Prefix,
    mut inInfo: SourceInfo,
) -> Result<(FCore::Cache, metamodelica::Ref<DAE::Exp>, DAE::Properties)> {
    let mut outCache: FCore::Cache;
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    let mut outProperties: DAE::Properties;
    let mut env: FCore::Graph;
    let mut fold_env: FCore::Graph;
    let mut reduction_iters: metamodelica::List<metamodelica::Ref<DAE::ReductionIterator>>;
    let mut dims: metamodelica::List<metamodelica::Ref<DAE::Dimension>>;
    let mut iter_const: DAE::Const;
    let mut exp_const: DAE::Const;
    let mut c: DAE::Const;
    let mut has_guard_exp: bool;
    let mut exp: metamodelica::Ref<DAE::Exp>;
    let mut afold_exp: Option<metamodelica::Ref<Absyn::Exp>>;
    let mut fold_exp: Option<metamodelica::Ref<DAE::Exp>>;
    let mut exp_ty: metamodelica::Ref<DAE::Type>;
    let mut res_ty: metamodelica::Ref<DAE::Type>;
    let mut r#fn: metamodelica::Ref<Absyn::Path>;
    let mut v: Option<metamodelica::Ref<Values::Value>>;
    let mut fold_id: ArcStr;
    let mut res_id: ArcStr;
    match '__try0: {
        env = unwrap_break_err!(FGraph::openScope(inEnv.clone(), openmodelica_frontend_types::SCode::Encapsulated::NOT_ENCAPSULATED, arcstr::literal!(FCore::forIterScopeName), None), '__try0);
        (outCache, env, reduction_iters, dims, iter_const, has_guard_exp) = unwrap_break_err!(elabCallReductionIterators(inCache.clone(), env.clone(), inIterators, inReductionExp.clone(), inImplicit, inDoVect, inPrefix.clone(), inInfo.clone()), '__try0);
        dims = unwrap_break_err!(fixDimsIterType(inIterType, dims.clone()), '__try0);
        let (__pa1, __pa2, __pa3, __pa4) = ::match_deref::match_deref! { match &(unwrap_break_err!(elabExpInExpression(outCache.clone(), env.clone(), inReductionExp.clone(), inImplicit, inDoVect, inPrefix.clone(), inInfo.clone()), '__try0)) {
            (__pa1, __pa2, DAE::Properties::PROP { type_: __pa3, constFlag: __pa4 }) => (__pa1.clone(), __pa2.clone(), __pa3.clone(), __pa4.clone()),
            _ => break '__try0 Err::<_, _>("pattern mismatch"),
        } };
        outCache = metamodelica::Own::own(__pa1);
        exp = metamodelica::Own::own(__pa2);
        exp_ty = metamodelica::Own::own(__pa3);
        exp_const = metamodelica::Own::own(__pa4);
        c = Types::constAnd(exp_const, iter_const);
        r#fn = (::match_deref::match_deref! { match inReductionFn {
            Deref @ Absyn::ComponentRef::CREF_IDENT { name: Deref @ "$array", subscripts: Deref @ metamodelica::ListNode::Nil } => metamodelica::Ref::new(Absyn::Path::IDENT { name: literal!("array") }),
            _ => unwrap_break_err!(AbsynUtil::crefToPath(inReductionFn), '__try0),
            _ => unreachable!("match_deref! exhaustiveness placeholder"),
        } });
        (outCache, exp, exp_ty, res_ty, v, r#fn) = unwrap_break_err!(reductionType(outCache.clone(), inEnv.clone(), r#fn.clone(), exp.clone(), exp_ty.clone(), &(unwrap_break_err!(Types::unboxedType(exp_ty.clone()), '__try0)), &dims, has_guard_exp, inInfo.clone()), '__try0);
        outProperties = DAE::Properties::PROP {
            type_: exp_ty.clone(),
            constFlag: c,
        };
        fold_id = Util::getTempVariableIndex();
        res_id = Util::getTempVariableIndex();
        (fold_env, afold_exp) = unwrap_break_err!(makeReductionFoldExp(env.clone(), r#fn.clone(), exp_ty.clone(), res_ty.clone(), fold_id.clone(), res_id.clone()), '__try0);
        (outCache, fold_exp, _) = unwrap_break_err!(elabExpOptAndMatchType(outCache.clone(), fold_env.clone(), afold_exp.clone(), res_ty.clone(), inImplicit, inDoVect, inPrefix.clone(), inInfo.clone()), '__try0);
        outExp = metamodelica::Ref::new(DAE::Exp::REDUCTION {
            reductionInfo: metamodelica::Ref::new(DAE::ReductionInfo {
                path: r#fn.clone(),
                iterType: inIterType,
                exprType: exp_ty.clone(),
                defaultValue: v.clone(),
                foldName: fold_id.clone(),
                resultName: res_id.clone(),
                foldExp: fold_exp.clone(),
            }),
            expr: exp.clone(),
            iterators: reduction_iters.clone(),
        });
        Ok::<_, &'static str>((
            afold_exp.clone(),
            c.clone(),
            dims.clone(),
            env.clone(),
            exp.clone(),
            exp_const.clone(),
            exp_ty.clone(),
            r#fn.clone(),
            fold_env.clone(),
            fold_exp.clone(),
            fold_id.clone(),
            has_guard_exp.clone(),
            iter_const.clone(),
            outCache.clone(),
            outExp.clone(),
            outProperties.clone(),
            reduction_iters.clone(),
            res_id.clone(),
            res_ty.clone(),
            v.clone(),
        ))
    } {
        Ok((
            __try0_o0,
            __try0_o1,
            __try0_o2,
            __try0_o3,
            __try0_o4,
            __try0_o5,
            __try0_o6,
            __try0_o7,
            __try0_o8,
            __try0_o9,
            __try0_o10,
            __try0_o11,
            __try0_o12,
            __try0_o13,
            __try0_o14,
            __try0_o15,
            __try0_o16,
            __try0_o17,
            __try0_o18,
            __try0_o19,
        )) => {
            afold_exp = __try0_o0;
            c = __try0_o1;
            dims = __try0_o2;
            env = __try0_o3;
            exp = __try0_o4;
            exp_const = __try0_o5;
            exp_ty = __try0_o6;
            r#fn = __try0_o7;
            fold_env = __try0_o8;
            fold_exp = __try0_o9;
            fold_id = __try0_o10;
            has_guard_exp = __try0_o11;
            iter_const = __try0_o12;
            outCache = __try0_o13;
            outExp = __try0_o14;
            outProperties = __try0_o15;
            reduction_iters = __try0_o16;
            res_id = __try0_o17;
            res_ty = __try0_o18;
            v = __try0_o19;
        }
        Err(__try0_err) => {
            if ((inIterators).len() as i32) > 1 {
                Error::addSourceMessage(
                    &(Error::INTERNAL_ERROR.clone()),
                    list![literal!(
                        "Reductions using multiple iterators is not yet implemented. Try rewriting the expression using nested reductions (e.g. array(i+j for i, j) => array(array(i+j for i) for j)."
                    )],
                    &inInfo,
                )?;
            } else {
                let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else {
                    return Err("pattern mismatch");
                };
                Debug::traceln(literal!("Static.elabCallReduction - failed!"))?;
            }
            return Err(__try0_err);
        }
    }
    Ok((outCache, outExp, outProperties))
}

fn fixDimsIterType(
    mut iterType: Absyn::ReductionIterType,
    mut dims: metamodelica::List<metamodelica::Ref<DAE::Dimension>>,
) -> Result<metamodelica::List<metamodelica::Ref<DAE::Dimension>>> {
    let mut outDims: metamodelica::List<metamodelica::Ref<DAE::Dimension>>;
    outDims = (match iterType {
        Absyn::ReductionIterType::COMBINE { .. } => dims,
        _ => list![(dims).head().cloned()?],
    });
    Ok(outDims)
}

fn elabCallReductionIterators(
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
    mut inIterators: &metamodelica::List<metamodelica::Ref<Absyn::ForIterator>>,
    mut inReductionExp: metamodelica::Ref<Absyn::Exp>,
    mut inImpl: bool,
    mut inDoVect: bool,
    mut inPrefix: DAE::Prefix,
    mut inInfo: SourceInfo,
) -> Result<(
    FCore::Cache,
    FCore::Graph,
    metamodelica::List<metamodelica::Ref<DAE::ReductionIterator>>,
    metamodelica::List<metamodelica::Ref<DAE::Dimension>>,
    DAE::Const,
    bool,
)> {
    let mut outCache: FCore::Cache = inCache;
    let mut outIteratorsEnv: FCore::Graph = inEnv.clone();
    let mut outIterators: metamodelica::List<metamodelica::Ref<DAE::ReductionIterator>> = metamodelica::nil();
    let mut outDims: metamodelica::List<metamodelica::Ref<DAE::Dimension>> = metamodelica::nil();
    let mut outConst: DAE::Const = openmodelica_frontend_types::DAE::Const::C_CONST;
    let mut outHasGuard: bool = false;
    let mut iter_name: ArcStr;
    let mut aiter_exp: metamodelica::Ref<Absyn::Exp>;
    let mut oaguard_exp: Option<metamodelica::Ref<Absyn::Exp>>;
    let mut oaiter_exp: Option<metamodelica::Ref<Absyn::Exp>>;
    let mut iter_exp: metamodelica::Ref<DAE::Exp>;
    let mut guard_exp: Option<metamodelica::Ref<DAE::Exp>>;
    let mut full_iter_ty: metamodelica::Ref<DAE::Type>;
    let mut iter_ty: metamodelica::Ref<DAE::Type>;
    let mut iter_const: DAE::Const;
    let mut guard_const: DAE::Const;
    let mut c: DAE::Const;
    let mut dim: metamodelica::Ref<DAE::Dimension>;
    let mut env: FCore::Graph;
    for mut iter in &**inIterators {
        let __arc3 = iter.clone();
        let Absyn::ITERATOR {
            name: __pa0,
            guardExp: __pa1,
            range: __pa2,
        } = &*__arc3;
        iter_name = metamodelica::Own::own(__pa0);
        oaguard_exp = metamodelica::Own::own(__pa1);
        oaiter_exp = metamodelica::Own::own(__pa2);
        if (oaiter_exp).is_some() {
            let __pa4 = ::match_deref::match_deref! { match &(oaiter_exp) {
                Some(__pa4) => __pa4.clone(),
                _ => return Err("pattern mismatch"),
            } };
            aiter_exp = metamodelica::Own::own(__pa4);
            let (__pa5, __pa6, __pa7, __pa8) = ::match_deref::match_deref! { match &(elabExpInExpression(outCache, inEnv.clone(), aiter_exp, inImpl, inDoVect, inPrefix.clone(), inInfo.clone())?) {
                (__pa5, __pa6, DAE::Properties::PROP { type_: __pa7, constFlag: __pa8 }) => (__pa5.clone(), __pa6.clone(), __pa7.clone(), __pa8.clone()),
                _ => return Err("pattern mismatch"),
            } };
            outCache = metamodelica::Own::own(__pa5);
            iter_exp = metamodelica::Own::own(__pa6);
            full_iter_ty = metamodelica::Own::own(__pa7);
            iter_const = metamodelica::Own::own(__pa8);
        } else {
            let (__pa9, __pa10, __pa11, __pa12) = ::match_deref::match_deref! { match &(deduceIterationRange(iter_name.clone(), &(AbsynUtil::findIteratorIndexedCrefs(inReductionExp.clone(), &iter_name, metamodelica::nil())?), inEnv.clone(), outCache, &inInfo)?) {
                (__pa9, DAE::Properties::PROP { type_: __pa10, constFlag: __pa11 }, __pa12) => (__pa9.clone(), __pa10.clone(), __pa11.clone(), __pa12.clone()),
                _ => return Err("pattern mismatch"),
            } };
            iter_exp = metamodelica::Own::own(__pa9);
            full_iter_ty = metamodelica::Own::own(__pa10);
            iter_const = metamodelica::Own::own(__pa11);
            outCache = metamodelica::Own::own(__pa12);
        }
        c = if (FGraph::inFunctionScope(&inEnv)) {
            iter_const
        } else {
            openmodelica_frontend_types::DAE::Const::C_CONST
        };
        (outCache, iter_exp, _) = Ceval::cevalIfConstant(
            outCache,
            inEnv.clone(),
            iter_exp,
            DAE::Properties::PROP {
                type_: full_iter_ty.clone(),
                constFlag: c,
            },
            inImpl,
            inInfo.clone(),
        )?;
        (iter_ty, dim) = Types::unliftArrayOrList(full_iter_ty)?;
        env = FGraph::addForIterator(
            inEnv.clone(),
            iter_name.clone(),
            iter_ty.clone(),
            openmodelica_frontend_types::DAE::Binding::interned_UNBOUND(),
            openmodelica_frontend_types::SCode::Variability::CONST,
            Some(iter_const),
        )?;
        outIteratorsEnv = FGraph::addForIterator(
            outIteratorsEnv,
            iter_name.clone(),
            iter_ty.clone(),
            openmodelica_frontend_types::DAE::Binding::interned_UNBOUND(),
            openmodelica_frontend_types::SCode::Variability::CONST,
            Some(iter_const),
        )?;
        let (__pa13, __pa14, __pa15) = ::match_deref::match_deref! { match &(elabExpOptAndMatchType(outCache, env, oaguard_exp, DAE::T_BOOL_DEFAULT().clone(), inImpl, inDoVect, inPrefix.clone(), inInfo.clone())?) {
            (__pa13, __pa14, DAE::Properties::PROP { type_: _, constFlag: __pa15 }) => (__pa13.clone(), __pa14.clone(), __pa15.clone()),
            _ => return Err("pattern mismatch"),
        } };
        outCache = metamodelica::Own::own(__pa13);
        guard_exp = metamodelica::Own::own(__pa14);
        guard_const = metamodelica::Own::own(__pa15);
        if (guard_exp).is_some() {
            outHasGuard = true;
            dim = openmodelica_frontend_types::DAE::Dimension::interned_DIM_UNKNOWN();
        }
        outConst = Types::constAnd(outConst, Types::constAnd(guard_const, iter_const));
        outIterators = metamodelica::cons(
            metamodelica::Ref::new(DAE::ReductionIterator {
                id: iter_name,
                exp: iter_exp,
                guardExp: guard_exp,
                ty: iter_ty,
            }),
            outIterators,
        );
        outDims = metamodelica::cons(dim, outDims);
    }
    outIterators = outIterators.reverse();
    outDims = outDims.reverse();
    Ok((outCache, outIteratorsEnv, outIterators, outDims, outConst, outHasGuard))
}

pub(crate) fn deduceIterationRange(
    mut inIterator: ArcStr,
    mut inCrefs: &metamodelica::List<(metamodelica::Ref<Absyn::ComponentRef>, i32)>,
    mut inEnv: FCore::Graph,
    mut inCache: FCore::Cache,
    mut inInfo: &SourceInfo,
) -> Result<(metamodelica::Ref<DAE::Exp>, DAE::Properties, FCore::Cache)> {
    let mut outRange: metamodelica::Ref<DAE::Exp> = metamodelica::Ref::new(DAE::Exp::ICONST { integer: 0 });
    let mut outProperties: DAE::Properties = DAE::Properties::PROP {
        type_: DAE::T_UNKNOWN_DEFAULT().clone(),
        constFlag: openmodelica_frontend_types::DAE::Const::C_UNKNOWN,
    };
    let mut outCache: FCore::Cache = inCache;
    let mut acref: metamodelica::Ref<Absyn::ComponentRef>;
    let mut cref: metamodelica::Ref<DAE::ComponentRef>;
    let mut idx: i32;
    let mut i1: i32;
    let mut i2: i32;
    let mut ty: metamodelica::Ref<DAE::Type>;
    let mut dims: metamodelica::List<metamodelica::Ref<DAE::Dimension>>;
    let mut dim: metamodelica::Ref<DAE::Dimension>;
    let mut range: metamodelica::Ref<DAE::Exp>;
    let mut ranges: metamodelica::List<metamodelica::Ref<DAE::Exp>> = metamodelica::nil();
    let mut cr_str1: ArcStr;
    let mut cr_str2: ArcStr;
    if (inCrefs).is_empty() {
        Error::addSourceMessageAndFail(
            &(Error::IMPLICIT_ITERATOR_NOT_FOUND_IN_LOOP_BODY.clone()),
            list![inIterator],
            inInfo,
        )?;
        unreachable!("Error.addSourceMessageAndFail always fails — caller-side flow-analysis hint");
    }
    for mut cr in &**inCrefs {
        (acref, idx) = cr.clone();
        cref = ComponentReference::toExpCref(&acref)?;
        if let Ok((__pa0, _, __pa1, _, _, _, _, _, _)) =
            Lookup::lookupVar(outCache.clone(), inEnv.clone(), cref.clone())
        {
            outCache = metamodelica::Own::own(__pa0);
            ty = metamodelica::Own::own(__pa1);
        } else {
            Error::addSourceMessageAndFail(
                &(Error::LOOKUP_VARIABLE_ERROR.clone()),
                list![Dump::printComponentRefStr(&acref)?, literal!("")],
                inInfo,
            )?;
            unreachable!("Error.addSourceMessageAndFail always fails — caller-side flow-analysis hint");
        }
        dims = TypesDump::getDimensions(&ty);
        if idx <= ((dims).len() as i32) {
            dim = (dims).get(idx)?;
            (range, outProperties) = deduceReductionIterationRange2(dim, cref, ty.clone(), idx)?;
        } else {
            range = metamodelica::Ref::new(DAE::Exp::ICONST { integer: 0 });
            outProperties = DAE::Properties::PROP {
                type_: metamodelica::Ref::new(DAE::Type::T_ARRAY {
                    ty: DAE::T_UNKNOWN_DEFAULT().clone(),
                    dims: list![metamodelica::Ref::new(DAE::Dimension::DIM_INTEGER { integer: 0 })],
                }),
                constFlag: openmodelica_frontend_types::DAE::Const::C_UNKNOWN,
            };
        }
        ranges = metamodelica::cons(range, ranges);
    }
    let (__pa2, __pa3) = ::match_deref::match_deref! { match &(ranges) {
        Deref @ metamodelica::ListNode::Cons { head: __pa2, tail: __pa3 } => (__pa2.clone(), __pa3.clone()),
        _ => return Err("pattern mismatch"),
    } };
    outRange = metamodelica::Own::own(__pa2);
    ranges = metamodelica::Own::own(__pa3);
    idx = 2;
    for mut r in &*ranges {
        if !(ExpressionBasics::expEqual(metamodelica::AsArg::as_arg(&r), outRange.clone())?) {
            (acref, i1) = (inCrefs).head().cloned()?;
            cr_str1 = Dump::printComponentRefStr(&acref)?;
            (acref, i2) = (inCrefs).get(idx)?;
            cr_str2 = Dump::printComponentRefStr(&acref)?;
            Error::addSourceMessageAndFail(
                &(Error::INCOMPATIBLE_IMPLICIT_RANGES.clone()),
                list![intString(i2), cr_str2, intString(i1), cr_str1],
                inInfo,
            )?;
            unreachable!("Error.addSourceMessageAndFail always fails — caller-side flow-analysis hint");
        }
        idx = idx + 1;
    }
    Ok((outRange, outProperties, outCache))
}

fn iteratorIndexedCrefsEqual(
    mut inCref1: &(metamodelica::Ref<Absyn::ComponentRef>, i32),
    mut inCref2: &(metamodelica::Ref<Absyn::ComponentRef>, i32),
) -> Result<bool> {
    let mut outEqual: bool;
    let mut cr1: metamodelica::Ref<Absyn::ComponentRef>;
    let mut cr2: metamodelica::Ref<Absyn::ComponentRef>;
    let mut idx1: i32;
    let mut idx2: i32;
    (cr1, idx1) = inCref1.clone();
    (cr2, idx2) = inCref2.clone();
    outEqual = idx1 == idx2 && AbsynUtil::crefEqual(&cr1, &cr2)?;
    Ok(outEqual)
}

fn deduceReductionIterationRange_traverser(
    mut inExp: metamodelica::Ref<Absyn::Exp>,
    mut inCrefs: metamodelica::List<(metamodelica::Ref<Absyn::ComponentRef>, i32)>,
    mut inIterator: &ArcStr,
) -> (
    metamodelica::Ref<Absyn::Exp>,
    metamodelica::List<(metamodelica::Ref<Absyn::ComponentRef>, i32)>,
) {
    let mut outExp: metamodelica::Ref<Absyn::Exp> = inExp.clone();
    let mut outCrefs: metamodelica::List<(metamodelica::Ref<Absyn::ComponentRef>, i32)>;
    outCrefs = (match &*inExp {
        Absyn::Exp::CREF { componentRef: cref } => getIteratorIndexedCrefs(cref.clone(), inIterator, inCrefs),
        _ => inCrefs,
    });
    (outExp, outCrefs)
}

fn getIteratorIndexedCrefs<'__b>(
    mut inCref: metamodelica::Ref<Absyn::ComponentRef>,
    mut inIterator: &'__b ArcStr,
    mut inCrefs: metamodelica::List<(metamodelica::Ref<Absyn::ComponentRef>, i32)>,
) -> metamodelica::List<(metamodelica::Ref<Absyn::ComponentRef>, i32)> {
    let mut outCrefs: metamodelica::List<(metamodelica::Ref<Absyn::ComponentRef>, i32)> = inCrefs.clone();
    let mut crefs: metamodelica::List<(metamodelica::Ref<Absyn::ComponentRef>, i32)>;
    outCrefs = (match &*inCref {
        Absyn::ComponentRef::CREF_IDENT {
            name: id,
            subscripts: subs,
        } => {
            let mut idx: i32;
            let mut name: ArcStr;
            idx = 1;
            for mut sub in &*subs.clone() {
                let () = (::match_deref::match_deref! { match &(sub.clone()) {
                    Deref @ Absyn::Subscript::SUBSCRIPT { subscript: Deref @ Absyn::Exp::CREF { componentRef: Deref @ Absyn::ComponentRef::CREF_IDENT { name: __esc_name, subscripts: Deref @ metamodelica::ListNode::Nil } } } => {
                        name = (*__esc_name).clone();
                        if metamodelica::stringEq(&name, &inIterator) {
                            outCrefs = metamodelica::cons((metamodelica::Ref::new(Absyn::ComponentRef::CREF_IDENT { name: id.clone(), subscripts: metamodelica::nil() }), idx), outCrefs);
                        }
                        ()
                    },
                    _ => (),
                    _ => unreachable!("match_deref! exhaustiveness placeholder"),
                } });
                idx = idx + 1;
            }
            outCrefs
        }
        Absyn::ComponentRef::CREF_QUAL {
            name: id,
            subscripts: subs,
            componentRef: cref,
        } => {
            let mut idx: i32;
            let mut cref = (*cref).clone();
            crefs = getIteratorIndexedCrefs(cref.clone(), inIterator, metamodelica::nil());
            for mut cr in &*crefs {
                (cref, idx) = cr.clone();
                outCrefs = metamodelica::cons(
                    (
                        metamodelica::Ref::new(Absyn::ComponentRef::CREF_QUAL {
                            name: id.clone(),
                            subscripts: subs.clone(),
                            componentRef: cref.clone(),
                        }),
                        idx,
                    ),
                    outCrefs,
                );
            }
            getIteratorIndexedCrefs(
                metamodelica::Ref::new(Absyn::ComponentRef::CREF_IDENT {
                    name: id.clone(),
                    subscripts: subs.clone(),
                }),
                inIterator,
                outCrefs,
            )
        }
        Absyn::ComponentRef::CREF_FULLYQUALIFIED { componentRef: cref } => {
            let mut idx: i32;
            let mut cref = (*cref).clone();
            crefs = getIteratorIndexedCrefs(cref.clone(), inIterator, metamodelica::nil());
            for mut cr in &*crefs {
                (cref, idx) = cr.clone();
                outCrefs = metamodelica::cons(
                    (
                        metamodelica::Ref::new(Absyn::ComponentRef::CREF_FULLYQUALIFIED {
                            componentRef: cref.clone(),
                        }),
                        idx,
                    ),
                    outCrefs,
                );
            }
            outCrefs
        }
        _ => inCrefs,
    });
    outCrefs
}

fn deduceReductionIterationRange2(
    mut inDimension: metamodelica::Ref<DAE::Dimension>,
    mut inCref: metamodelica::Ref<DAE::ComponentRef>,
    mut inType: metamodelica::Ref<DAE::Type>,
    mut inIndex: i32,
) -> Result<(metamodelica::Ref<DAE::Exp>, DAE::Properties)> {
    let mut outRange: metamodelica::Ref<DAE::Exp>;
    let mut outProperties: DAE::Properties;
    let mut range_ty: metamodelica::Ref<DAE::Type> = metamodelica::Ref::new(DAE::Type::T_NORETCALL);
    let mut range_const: DAE::Const = DAE::Const::C_CONST;
    let mut enum_path: metamodelica::Ref<Absyn::Path>;
    let mut enum_start: metamodelica::Ref<Absyn::Path>;
    let mut enum_end: metamodelica::Ref<Absyn::Path>;
    let mut enum_lits: metamodelica::List<ArcStr>;
    let mut sz: i32;
    let mut size_exp: metamodelica::Ref<DAE::Exp>;
    outRange = (match &*inDimension.clone() {
        DAE::Dimension::DIM_BOOLEAN { .. } => {
            range_ty = metamodelica::Ref::new(DAE::Type::T_ARRAY {
                ty: DAE::T_BOOL_DEFAULT().clone(),
                dims: list![inDimension],
            });
            range_const = openmodelica_frontend_types::DAE::Const::C_CONST;
            metamodelica::Ref::new(DAE::Exp::RANGE {
                ty: range_ty.clone(),
                start: metamodelica::Ref::new(DAE::Exp::BCONST { bool: false }),
                step: None,
                stop: metamodelica::Ref::new(DAE::Exp::BCONST { bool: true }),
            })
        }
        DAE::Dimension::DIM_ENUM {
            enumTypeName: __esc_enum_path,
            literals: __esc_enum_lits,
            ..
        } => {
            enum_path = (*__esc_enum_path).clone();
            enum_lits = (*__esc_enum_lits).clone();
            enum_start =
                AbsynUtil::suffixPath(metamodelica::AsArg::as_arg(&enum_path), &((enum_lits).head().cloned()?));
            enum_end = AbsynUtil::suffixPath(
                metamodelica::AsArg::as_arg(&enum_path),
                &(List::last(metamodelica::AsArg::as_arg(&enum_lits))?),
            );
            range_ty = metamodelica::Ref::new(DAE::Type::T_ENUMERATION {
                index: None,
                path: enum_path.clone(),
                names: enum_lits.clone(),
                literalVarLst: metamodelica::nil(),
                attributeLst: metamodelica::nil(),
            });
            range_ty = metamodelica::Ref::new(DAE::Type::T_ARRAY {
                ty: range_ty,
                dims: list![inDimension],
            });
            range_const = openmodelica_frontend_types::DAE::Const::C_CONST;
            metamodelica::Ref::new(DAE::Exp::RANGE {
                ty: range_ty.clone(),
                start: metamodelica::Ref::new(DAE::Exp::ENUM_LITERAL {
                    name: enum_start,
                    index: 1,
                }),
                step: None,
                stop: metamodelica::Ref::new(DAE::Exp::ENUM_LITERAL {
                    name: enum_end,
                    index: ((enum_lits).len() as i32),
                }),
            })
        }
        DAE::Dimension::DIM_INTEGER { integer: __esc_sz } => {
            sz = (*__esc_sz).clone();
            range_ty = metamodelica::Ref::new(DAE::Type::T_ARRAY {
                ty: DAE::T_INTEGER_DEFAULT().clone(),
                dims: list![inDimension],
            });
            range_const = openmodelica_frontend_types::DAE::Const::C_CONST;
            metamodelica::Ref::new(DAE::Exp::RANGE {
                ty: range_ty.clone(),
                start: metamodelica::Ref::new(DAE::Exp::ICONST { integer: 1 }),
                step: None,
                stop: metamodelica::Ref::new(DAE::Exp::ICONST { integer: sz.clone() }),
            })
        }
        _ => {
            size_exp = metamodelica::Ref::new(DAE::Exp::SIZE {
                exp: metamodelica::Ref::new(DAE::Exp::CREF {
                    componentRef: inCref,
                    ty: inType,
                }),
                sz: Some(metamodelica::Ref::new(DAE::Exp::ICONST { integer: inIndex })),
            });
            range_ty = metamodelica::Ref::new(DAE::Type::T_ARRAY {
                ty: DAE::T_INTEGER_DEFAULT().clone(),
                dims: list![inDimension],
            });
            range_const = openmodelica_frontend_types::DAE::Const::C_PARAM;
            metamodelica::Ref::new(DAE::Exp::RANGE {
                ty: range_ty.clone(),
                start: metamodelica::Ref::new(DAE::Exp::ICONST { integer: 1 }),
                step: None,
                stop: size_exp,
            })
        }
    });
    outProperties = DAE::Properties::PROP {
        type_: range_ty,
        constFlag: range_const,
    };
    Ok((outRange, outProperties))
}

fn makeReductionFoldExp(
    mut inEnv: FCore::Graph,
    mut path: metamodelica::Ref<Absyn::Path>,
    mut expty: metamodelica::Ref<DAE::Type>,
    mut resultTy: metamodelica::Ref<DAE::Type>,
    mut foldId: ArcStr,
    mut resultId: ArcStr,
) -> Result<(FCore::Graph, Option<metamodelica::Ref<Absyn::Exp>>)> {
    let mut outEnv: FCore::Graph;
    let mut afoldExp: Option<metamodelica::Ref<Absyn::Exp>>;
    (outEnv, afoldExp) = (::match_deref::match_deref! { match &(AbsynUtil::makeNotFullyQualified(path.clone())) {
        Deref @ Absyn::Path::IDENT { name: Deref @ "$array" } => {
            (inEnv, None)
        },
        Deref @ Absyn::Path::IDENT { name: Deref @ "array" } => {
            (inEnv, None)
        },
        Deref @ Absyn::Path::IDENT { name: Deref @ "list" } => {
            (inEnv, None)
        },
        Deref @ Absyn::Path::IDENT { name: Deref @ "listReverse" } => {
            (inEnv, None)
        },
        Deref @ Absyn::Path::IDENT { name: Deref @ "sum" } => {
            let mut exp: metamodelica::Ref<Absyn::Exp>;
            let mut cr1: metamodelica::Ref<Absyn::ComponentRef>;
            let mut cr2: metamodelica::Ref<Absyn::ComponentRef>;
            let mut env: FCore::Graph;
            env = FGraph::addForIterator(inEnv, foldId.clone(), expty.clone(), openmodelica_frontend_types::DAE::Binding::interned_UNBOUND(), openmodelica_frontend_types::SCode::Variability::VAR, Some(openmodelica_frontend_types::DAE::Const::C_VAR))?;
            env = FGraph::addForIterator(env, resultId.clone(), expty, openmodelica_frontend_types::DAE::Binding::interned_UNBOUND(), openmodelica_frontend_types::SCode::Variability::VAR, Some(openmodelica_frontend_types::DAE::Const::C_VAR))?;
            cr1 = metamodelica::Ref::new(Absyn::ComponentRef::CREF_IDENT { name: foldId, subscripts: metamodelica::nil() });
            cr2 = metamodelica::Ref::new(Absyn::ComponentRef::CREF_IDENT { name: resultId, subscripts: metamodelica::nil() });
            exp = metamodelica::Ref::new(Absyn::Exp::BINARY { exp1: metamodelica::Ref::new(Absyn::Exp::CREF { componentRef: cr2 }), op: openmodelica_ast::Absyn::Operator::ADD, exp2: metamodelica::Ref::new(Absyn::Exp::CREF { componentRef: cr1 }) });
            (env, Some(exp))
        },
        Deref @ Absyn::Path::IDENT { name: Deref @ "product" } => {
            let mut exp: metamodelica::Ref<Absyn::Exp>;
            let mut cr1: metamodelica::Ref<Absyn::ComponentRef>;
            let mut cr2: metamodelica::Ref<Absyn::ComponentRef>;
            let mut env: FCore::Graph;
            env = FGraph::addForIterator(inEnv, foldId.clone(), expty.clone(), openmodelica_frontend_types::DAE::Binding::interned_UNBOUND(), openmodelica_frontend_types::SCode::Variability::VAR, Some(openmodelica_frontend_types::DAE::Const::C_VAR))?;
            env = FGraph::addForIterator(env, resultId.clone(), expty, openmodelica_frontend_types::DAE::Binding::interned_UNBOUND(), openmodelica_frontend_types::SCode::Variability::VAR, Some(openmodelica_frontend_types::DAE::Const::C_VAR))?;
            cr1 = metamodelica::Ref::new(Absyn::ComponentRef::CREF_IDENT { name: foldId, subscripts: metamodelica::nil() });
            cr2 = metamodelica::Ref::new(Absyn::ComponentRef::CREF_IDENT { name: resultId, subscripts: metamodelica::nil() });
            exp = metamodelica::Ref::new(Absyn::Exp::BINARY { exp1: metamodelica::Ref::new(Absyn::Exp::CREF { componentRef: cr2 }), op: openmodelica_ast::Absyn::Operator::MUL, exp2: metamodelica::Ref::new(Absyn::Exp::CREF { componentRef: cr1 }) });
            (env, Some(exp))
        },
        _ => {
            let mut exp: metamodelica::Ref<Absyn::Exp>;
            let mut cr: metamodelica::Ref<Absyn::ComponentRef>;
            let mut cr1: metamodelica::Ref<Absyn::ComponentRef>;
            let mut cr2: metamodelica::Ref<Absyn::ComponentRef>;
            let mut env: FCore::Graph;
            cr = AbsynUtil::pathToCref(&path);
            env = FGraph::addForIterator(inEnv, foldId.clone(), expty, openmodelica_frontend_types::DAE::Binding::interned_UNBOUND(), openmodelica_frontend_types::SCode::Variability::VAR, Some(openmodelica_frontend_types::DAE::Const::C_VAR))?;
            env = FGraph::addForIterator(env, resultId.clone(), resultTy, openmodelica_frontend_types::DAE::Binding::interned_UNBOUND(), openmodelica_frontend_types::SCode::Variability::VAR, Some(openmodelica_frontend_types::DAE::Const::C_VAR))?;
            cr1 = metamodelica::Ref::new(Absyn::ComponentRef::CREF_IDENT { name: foldId, subscripts: metamodelica::nil() });
            cr2 = metamodelica::Ref::new(Absyn::ComponentRef::CREF_IDENT { name: resultId, subscripts: metamodelica::nil() });
            exp = metamodelica::Ref::new(Absyn::Exp::CALL { function_: cr, functionArgs: metamodelica::Ref::new(Absyn::FunctionArgs::FUNCTIONARGS { args: list![metamodelica::Ref::new(Absyn::Exp::CREF { componentRef: cr1 }), metamodelica::Ref::new(Absyn::Exp::CREF { componentRef: cr2 })], argNames: metamodelica::nil() }), typeVars: metamodelica::nil() });
            (env, Some(exp))
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outEnv, afoldExp))
}

fn reductionType(
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
    mut inFn: metamodelica::Ref<Absyn::Path>,
    mut inExp: metamodelica::Ref<DAE::Exp>,
    mut inType: metamodelica::Ref<DAE::Type>,
    mut unboxedType: &metamodelica::Ref<DAE::Type>,
    mut dims: &metamodelica::List<metamodelica::Ref<DAE::Dimension>>,
    mut hasGuardExp: bool,
    mut info: SourceInfo,
) -> Result<(
    FCore::Cache,
    metamodelica::Ref<DAE::Exp>,
    metamodelica::Ref<DAE::Type>,
    metamodelica::Ref<DAE::Type>,
    Option<metamodelica::Ref<Values::Value>>,
    metamodelica::Ref<Absyn::Path>,
)> {
    let mut outCache: FCore::Cache = inCache.clone();
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    let mut outType: metamodelica::Ref<DAE::Type>;
    let mut resultType: metamodelica::Ref<DAE::Type>;
    let mut defaultValue: Option<metamodelica::Ref<Values::Value>>;
    let mut outPath: metamodelica::Ref<Absyn::Path>;
    let mut r#fn: metamodelica::Ref<Absyn::Path> = AbsynUtil::makeNotFullyQualified(inFn.clone());
    (outExp, outType, resultType, defaultValue, outPath) = (::match_deref::match_deref! { match &((r#fn.clone(), unboxedType.clone())) {
        (Deref @ Absyn::Path::IDENT { name: Deref @ "array" }, _) => {
            let mut ty: metamodelica::Ref<DAE::Type>;
            ty = List::foldr(dims, &fnptr!(Types::liftArray, metamodelica::Ref<DAE::Type>, metamodelica::Ref<DAE::Dimension>), inType)?;
            (inExp, ty.clone(), ty, Some(metamodelica::Ref::new(Values::Value::ARRAY { valueLst: metamodelica::nil(), dimLst: list![0] })), r#fn)
        },
        (Deref @ Absyn::Path::IDENT { name: Deref @ "$array" }, _) => {
            let mut ty: metamodelica::Ref<DAE::Type>;
            ty = List::foldr(dims, &fnptr!(Types::liftArray, metamodelica::Ref<DAE::Type>, metamodelica::Ref<DAE::Dimension>), inType)?;
            (inExp, ty.clone(), ty, Some(metamodelica::Ref::new(Values::Value::ARRAY { valueLst: metamodelica::nil(), dimLst: list![0] })), r#fn)
        },
        (Deref @ Absyn::Path::IDENT { name: Deref @ "list" }, _) => {
            let mut ty: metamodelica::Ref<DAE::Type>;
            let mut exp: metamodelica::Ref<DAE::Exp>;
            (exp, ty) = Types::matchType(inExp, inType, DAE::T_METABOXED_DEFAULT().clone(), true)?;
            ty = List::foldr(dims, &move |__a0: metamodelica::Ref<DAE::Type>, __a1: metamodelica::Ref<DAE::Dimension>| -> metamodelica::Result<_> { ::std::result::Result::Ok(Types::liftList(__a0, &__a1)) }, ty)?;
            (exp, ty.clone(), ty, Some(metamodelica::Ref::new(Values::Value::LIST { valueLst: metamodelica::nil() })), r#fn)
        },
        (Deref @ Absyn::Path::IDENT { name: Deref @ "listReverse" }, _) => {
            let mut ty: metamodelica::Ref<DAE::Type>;
            let mut exp: metamodelica::Ref<DAE::Exp>;
            (exp, ty) = Types::matchType(inExp, inType, DAE::T_METABOXED_DEFAULT().clone(), true)?;
            ty = List::foldr(dims, &move |__a0: metamodelica::Ref<DAE::Type>, __a1: metamodelica::Ref<DAE::Dimension>| -> metamodelica::Result<_> { ::std::result::Result::Ok(Types::liftList(__a0, &__a1)) }, ty)?;
            (exp, ty.clone(), ty, Some(metamodelica::Ref::new(Values::Value::LIST { valueLst: metamodelica::nil() })), r#fn)
        },
        (Deref @ Absyn::Path::IDENT { name: Deref @ "min" }, Deref @ DAE::Type::T_REAL { .. }) => {
            let mut r: metamodelica::Real;
            let mut ty: metamodelica::Ref<DAE::Type>;
            let mut v: metamodelica::Ref<Values::Value>;
            let mut exp: metamodelica::Ref<DAE::Exp>;
            r = System::realMaxLit();
            v = metamodelica::Ref::new(Values::Value::REAL { real: r });
            (exp, ty) = Types::matchType(inExp, inType, DAE::T_REAL_DEFAULT().clone(), true)?;
            (exp, ty.clone(), ty, Some(v), r#fn)
        },
        (Deref @ Absyn::Path::IDENT { name: Deref @ "min" }, Deref @ DAE::Type::T_INTEGER { .. }) => {
            let mut i: i32;
            let mut ty: metamodelica::Ref<DAE::Type>;
            let mut v: metamodelica::Ref<Values::Value>;
            let mut exp: metamodelica::Ref<DAE::Exp>;
            i = System::intMaxLit();
            v = metamodelica::Ref::new(Values::Value::INTEGER { integer: i });
            (exp, ty) = Types::matchType(inExp, inType, DAE::T_INTEGER_DEFAULT().clone(), true)?;
            (exp, ty.clone(), ty, Some(v), r#fn)
        },
        (Deref @ Absyn::Path::IDENT { name: Deref @ "min" }, Deref @ DAE::Type::T_BOOL { .. }) => {
            let mut ty: metamodelica::Ref<DAE::Type>;
            let mut v: metamodelica::Ref<Values::Value>;
            let mut exp: metamodelica::Ref<DAE::Exp>;
            v = metamodelica::Ref::new(Values::Value::BOOL { boolean: true });
            (exp, ty) = Types::matchType(inExp, inType, DAE::T_BOOL_DEFAULT().clone(), true)?;
            (exp, ty.clone(), ty, Some(v), r#fn)
        },
        (Deref @ Absyn::Path::IDENT { name: Deref @ "min" }, Deref @ DAE::Type::T_STRING { .. }) => {
            let mut ty: metamodelica::Ref<DAE::Type>;
            let mut exp: metamodelica::Ref<DAE::Exp>;
            (exp, ty) = Types::matchType(inExp, inType, DAE::T_STRING_DEFAULT().clone(), true)?;
            (exp, ty.clone(), ty, None, r#fn)
        },
        (Deref @ Absyn::Path::IDENT { name: Deref @ "min" }, Deref @ DAE::Type::T_ENUMERATION { .. }) => {
            let mut ty: metamodelica::Ref<DAE::Type>;
            let mut v: metamodelica::Ref<Values::Value>;
            let mut exp: metamodelica::Ref<DAE::Exp>;
            v = metamodelica::Ref::new(Values::Value::ENUM_LITERAL { name: AbsynUtil::suffixPath(var_field!((**unboxedType).path, DAE::Type::T_ENUMERATION), &(List::last(var_field!((**unboxedType).names, DAE::Type::T_ENUMERATION))?)), index: ((var_field!((**unboxedType).names, DAE::Type::T_ENUMERATION)).len() as i32) });
            (exp, ty) = Types::matchType(inExp, inType, DAE::T_ENUMERATION_DEFAULT().clone(), true)?;
            (exp, ty.clone(), ty, Some(v), r#fn)
        },
        (Deref @ Absyn::Path::IDENT { name: Deref @ "max" }, Deref @ DAE::Type::T_REAL { .. }) => {
            let mut r: metamodelica::Real;
            let mut ty: metamodelica::Ref<DAE::Type>;
            let mut v: metamodelica::Ref<Values::Value>;
            let mut exp: metamodelica::Ref<DAE::Exp>;
            r = -(System::realMaxLit());
            v = metamodelica::Ref::new(Values::Value::REAL { real: r });
            (exp, ty) = Types::matchType(inExp, inType, DAE::T_REAL_DEFAULT().clone(), true)?;
            (exp, ty.clone(), ty, Some(v), r#fn)
        },
        (Deref @ Absyn::Path::IDENT { name: Deref @ "max" }, Deref @ DAE::Type::T_INTEGER { .. }) => {
            let mut i: i32;
            let mut ty: metamodelica::Ref<DAE::Type>;
            let mut v: metamodelica::Ref<Values::Value>;
            let mut exp: metamodelica::Ref<DAE::Exp>;
            i = intNeg(System::intMaxLit());
            v = metamodelica::Ref::new(Values::Value::INTEGER { integer: i });
            (exp, ty) = Types::matchType(inExp, inType, DAE::T_INTEGER_DEFAULT().clone(), true)?;
            (exp, ty.clone(), ty, Some(v), r#fn)
        },
        (Deref @ Absyn::Path::IDENT { name: Deref @ "max" }, Deref @ DAE::Type::T_BOOL { .. }) => {
            let mut ty: metamodelica::Ref<DAE::Type>;
            let mut v: metamodelica::Ref<Values::Value>;
            let mut exp: metamodelica::Ref<DAE::Exp>;
            v = metamodelica::Ref::new(Values::Value::BOOL { boolean: false });
            (exp, ty) = Types::matchType(inExp, inType, DAE::T_BOOL_DEFAULT().clone(), true)?;
            (exp, ty.clone(), ty, Some(v), r#fn)
        },
        (Deref @ Absyn::Path::IDENT { name: Deref @ "max" }, Deref @ DAE::Type::T_STRING { .. }) => {
            let mut ty: metamodelica::Ref<DAE::Type>;
            let mut v: metamodelica::Ref<Values::Value>;
            let mut exp: metamodelica::Ref<DAE::Exp>;
            v = metamodelica::Ref::new(Values::Value::STRING { string: literal!("") });
            (exp, ty) = Types::matchType(inExp, inType, DAE::T_STRING_DEFAULT().clone(), true)?;
            (exp, ty.clone(), ty, Some(v), r#fn)
        },
        (Deref @ Absyn::Path::IDENT { name: Deref @ "max" }, Deref @ DAE::Type::T_ENUMERATION { .. }) => {
            let mut ty: metamodelica::Ref<DAE::Type>;
            let mut v: metamodelica::Ref<Values::Value>;
            let mut exp: metamodelica::Ref<DAE::Exp>;
            v = metamodelica::Ref::new(Values::Value::ENUM_LITERAL { name: AbsynUtil::suffixPath(var_field!((**unboxedType).path, DAE::Type::T_ENUMERATION), &((var_field!((**unboxedType).names, DAE::Type::T_ENUMERATION)).head().cloned()?)), index: 1 });
            (exp, ty) = Types::matchType(inExp, inType, DAE::T_ENUMERATION_DEFAULT().clone(), true)?;
            (exp, ty.clone(), ty, Some(v), r#fn)
        },
        (Deref @ Absyn::Path::IDENT { name: Deref @ "sum" }, Deref @ DAE::Type::T_REAL { .. }) => {
            let mut ty: metamodelica::Ref<DAE::Type>;
            let mut v: metamodelica::Ref<Values::Value>;
            let mut exp: metamodelica::Ref<DAE::Exp>;
            v = metamodelica::Ref::new(Values::Value::REAL { real: metamodelica::OrderedFloat(0.0_f64) });
            (exp, ty) = Types::matchType(inExp, inType, DAE::T_REAL_DEFAULT().clone(), true)?;
            (exp, ty.clone(), ty, Some(v), r#fn)
        },
        (Deref @ Absyn::Path::IDENT { name: Deref @ "sum" }, Deref @ DAE::Type::T_INTEGER { .. }) => {
            let mut ty: metamodelica::Ref<DAE::Type>;
            let mut v: metamodelica::Ref<Values::Value>;
            let mut exp: metamodelica::Ref<DAE::Exp>;
            v = metamodelica::Ref::new(Values::Value::INTEGER { integer: 0 });
            (exp, ty) = Types::matchType(inExp, inType, DAE::T_INTEGER_DEFAULT().clone(), true)?;
            (exp, ty.clone(), ty, Some(v), r#fn)
        },
        (Deref @ Absyn::Path::IDENT { name: Deref @ "sum" }, Deref @ DAE::Type::T_BOOL { .. }) => {
            let mut ty: metamodelica::Ref<DAE::Type>;
            let mut v: metamodelica::Ref<Values::Value>;
            let mut exp: metamodelica::Ref<DAE::Exp>;
            v = metamodelica::Ref::new(Values::Value::BOOL { boolean: false });
            (exp, ty) = Types::matchType(inExp, inType, DAE::T_BOOL_DEFAULT().clone(), true)?;
            (exp, ty.clone(), ty, Some(v), r#fn)
        },
        (Deref @ Absyn::Path::IDENT { name: Deref @ "sum" }, Deref @ DAE::Type::T_STRING { .. }) => {
            let mut ty: metamodelica::Ref<DAE::Type>;
            let mut v: metamodelica::Ref<Values::Value>;
            let mut exp: metamodelica::Ref<DAE::Exp>;
            v = metamodelica::Ref::new(Values::Value::STRING { string: literal!("") });
            (exp, ty) = Types::matchType(inExp, inType, DAE::T_STRING_DEFAULT().clone(), true)?;
            (exp, ty.clone(), ty, Some(v), r#fn)
        },
        (Deref @ Absyn::Path::IDENT { name: Deref @ "sum" }, Deref @ DAE::Type::T_ARRAY { .. }) => {
            (inExp, inType.clone(), inType, None, r#fn)
        },
        (Deref @ Absyn::Path::IDENT { name: Deref @ "product" }, Deref @ DAE::Type::T_REAL { .. }) => {
            let mut ty: metamodelica::Ref<DAE::Type>;
            let mut v: metamodelica::Ref<Values::Value>;
            let mut exp: metamodelica::Ref<DAE::Exp>;
            v = metamodelica::Ref::new(Values::Value::REAL { real: metamodelica::OrderedFloat(1.0_f64) });
            (exp, ty) = Types::matchType(inExp, inType, DAE::T_REAL_DEFAULT().clone(), true)?;
            (exp, ty.clone(), ty, Some(v), r#fn)
        },
        (Deref @ Absyn::Path::IDENT { name: Deref @ "product" }, Deref @ DAE::Type::T_INTEGER { .. }) => {
            let mut ty: metamodelica::Ref<DAE::Type>;
            let mut v: metamodelica::Ref<Values::Value>;
            let mut exp: metamodelica::Ref<DAE::Exp>;
            v = metamodelica::Ref::new(Values::Value::INTEGER { integer: 1 });
            (exp, ty) = Types::matchType(inExp, inType, DAE::T_INTEGER_DEFAULT().clone(), true)?;
            (exp, ty.clone(), ty, Some(v), r#fn)
        },
        (Deref @ Absyn::Path::IDENT { name: Deref @ "product" }, Deref @ DAE::Type::T_BOOL { .. }) => {
            let mut ty: metamodelica::Ref<DAE::Type>;
            let mut v: metamodelica::Ref<Values::Value>;
            let mut exp: metamodelica::Ref<DAE::Exp>;
            v = metamodelica::Ref::new(Values::Value::BOOL { boolean: true });
            (exp, ty) = Types::matchType(inExp, inType, DAE::T_BOOL_DEFAULT().clone(), true)?;
            (exp, ty.clone(), ty, Some(v), r#fn)
        },
        (Deref @ Absyn::Path::IDENT { name: Deref @ "product" }, Deref @ DAE::Type::T_STRING { .. }) => {
            Error::addSourceMessage(&(Error::INTERNAL_ERROR.clone()), list![literal!("product reduction not defined for String")], &info)?;
            return Err("fail")
        },
        (Deref @ Absyn::Path::IDENT { name: Deref @ "product" }, Deref @ DAE::Type::T_ARRAY { .. }) => {
            (inExp, inType.clone(), inType, None, r#fn)
        },
        _ => {
            let mut fnTypes: metamodelica::List<metamodelica::Ref<DAE::Type>>;
            let mut ty: metamodelica::Ref<DAE::Type>;
            let mut ty2: metamodelica::Ref<DAE::Type>;
            let mut typeA: metamodelica::Ref<DAE::Type>;
            let mut typeB: metamodelica::Ref<DAE::Type>;
            let mut resType: metamodelica::Ref<DAE::Type>;
            let mut path: metamodelica::Ref<Absyn::Path>;
            let mut exp: metamodelica::Ref<DAE::Exp>;
            let mut bindings: metamodelica::List<(ArcStr, metamodelica::List<metamodelica::Ref<DAE::Type>>)>;
            let mut defaultBinding: Option<metamodelica::Ref<Values::Value>>;
            (outCache, fnTypes) = Lookup::lookupFunctionsInEnv(inCache, inEnv.clone(), inFn.clone(), info.clone());
            (typeA, typeB, resType, defaultBinding, path) = checkReductionType1(&inEnv, inFn, fnTypes, &info)?;
            ty2 = if ((defaultBinding).is_some()) {typeB.clone()} else {inType.clone()};
            (exp, typeA, bindings) = Types::matchTypePolymorphicWithError(inExp, inType.clone(), typeA, Some(path.clone()), metamodelica::nil(), &info)?;
            (_, typeB, bindings) = Types::matchTypePolymorphicWithError(metamodelica::Ref::new(DAE::Exp::CREF { componentRef: metamodelica::Ref::new(DAE::ComponentRef::CREF_IDENT { ident: literal!("$result"), identType: DAE::T_ANYTYPE_DEFAULT().clone(), subscriptLst: metamodelica::nil() }), ty: DAE::T_ANYTYPE_DEFAULT().clone() }), ty2, typeB, Some(path.clone()), bindings, &info)?;
            bindings = Types::solvePolymorphicBindings(bindings, &info, path.clone())?;
            typeA = Types::fixPolymorphicRestype(typeA, &bindings, &info)?;
            typeB = Types::fixPolymorphicRestype(typeB, &bindings, &info)?;
            resType = Types::fixPolymorphicRestype(resType, &bindings, &info)?;
            (exp, ty) = checkReductionType2(exp, inType, typeA.clone(), typeB.clone(), resType.clone(), Types::equivtypes(typeA, typeB.clone()) || (defaultBinding).is_some(), Types::equivtypes(typeB.clone(), resType), &info)?;
            let (__pa0, Util::SUCCESS { .. }) = (instantiateDaeFunction(outCache, inEnv, path.clone(), false, None, true)) else { return Err("pattern mismatch") };
            outCache = metamodelica::Own::own(__pa0);
            Error::assertionOrAddSourceMessage(Config::acceptMetaModelicaGrammar()? || Flags::isSet(Flags::EXPERIMENTAL_REDUCTIONS.clone())?, &(Error::COMPILER_NOTIFICATION.clone()), list![literal!("Custom reduction functions are an OpenModelica extension to the Modelica Specification. Do not use them if you need your model to compile using other tools or if you are concerned about using experimental features. Use -d=experimentalReductions to disable this message.")], &info)?;
            (exp, ty, typeB, defaultBinding, path)
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outCache, outExp, outType, resultType, defaultValue, outPath))
}

fn checkReductionType1(
    mut inEnv: &FCore::Graph,
    mut inPath: metamodelica::Ref<Absyn::Path>,
    mut fnTypes: metamodelica::List<metamodelica::Ref<DAE::Type>>,
    mut info: &SourceInfo,
) -> Result<(
    metamodelica::Ref<DAE::Type>,
    metamodelica::Ref<DAE::Type>,
    metamodelica::Ref<DAE::Type>,
    Option<metamodelica::Ref<Values::Value>>,
    metamodelica::Ref<Absyn::Path>,
)> {
    let mut typeA: metamodelica::Ref<DAE::Type>;
    let mut typeB: metamodelica::Ref<DAE::Type>;
    let mut resType: metamodelica::Ref<DAE::Type>;
    let mut startValue: Option<metamodelica::Ref<Values::Value>>;
    let mut outPath: metamodelica::Ref<Absyn::Path>;
    (typeA, typeB, resType, startValue, outPath) = (::match_deref::match_deref! { match &(fnTypes.clone()) {
        Deref @ metamodelica::ListNode::Nil => {
            let mut str1: ArcStr;
            let mut str2: ArcStr;
            str1 = AbsynUtil::pathString(inPath, literal!("."), true, false)?;
            str2 = FGraph::printGraphPathStr(inEnv);
            Error::addSourceMessage(&(Error::LOOKUP_FUNCTION_ERROR.clone()), list![str1, str2], info)?;
            return Err("fail")
        },
        Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Type::T_FUNCTION { funcArg: Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::FuncArg { ty: __esc_typeA, r#const: DAE::Const::C_VAR { .. }, .. }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::FuncArg { ty: __esc_typeB, r#const: DAE::Const::C_VAR { .. }, defaultBinding: Some(e), .. }, tail: Deref @ metamodelica::ListNode::Nil } }, funcResultType: __esc_resType, path, .. }, tail: Deref @ metamodelica::ListNode::Nil } => {
            typeA = (*__esc_typeA).clone();
            typeB = (*__esc_typeB).clone();
            resType = (*__esc_resType).clone();
            let mut v: metamodelica::Ref<Values::Value>;
            v = Ceval::cevalSimple(e.clone())?;
            (typeA.clone(), typeB.clone(), resType.clone(), Some(v), path.clone())
        },
        Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Type::T_FUNCTION { funcArg: Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::FuncArg { ty: __esc_typeA, r#const: DAE::Const::C_VAR { .. }, .. }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::FuncArg { ty: __esc_typeB, r#const: DAE::Const::C_VAR { .. }, defaultBinding: None, .. }, tail: Deref @ metamodelica::ListNode::Nil } }, funcResultType: __esc_resType, path, .. }, tail: Deref @ metamodelica::ListNode::Nil } => {
            typeA = (*__esc_typeA).clone();
            typeB = (*__esc_typeB).clone();
            resType = (*__esc_resType).clone();
            (typeA.clone(), typeB.clone(), resType.clone(), None, path.clone())
        },
        _ => {
            let mut str1: ArcStr;
            str1 = stringDelimitList(List::map(fnTypes, &TypesDump::unparseType)?, literal!(","));
            Error::addSourceMessage(&(Error::UNSUPPORTED_REDUCTION_TYPE.clone()), list![str1], info)?;
            return Err("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((typeA, typeB, resType, startValue, outPath))
}

fn checkReductionType2(
    mut inExp: metamodelica::Ref<DAE::Exp>,
    mut expType: metamodelica::Ref<DAE::Type>,
    mut typeA: metamodelica::Ref<DAE::Type>,
    mut typeB: metamodelica::Ref<DAE::Type>,
    mut typeC: metamodelica::Ref<DAE::Type>,
    mut equivAB: bool,
    mut equivBC: bool,
    mut info: &SourceInfo,
) -> Result<(metamodelica::Ref<DAE::Exp>, metamodelica::Ref<DAE::Type>)> {
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    let mut outTy: metamodelica::Ref<DAE::Type>;
    (outExp, outTy) = (match (equivAB, equivBC) {
        (true, true) => (inExp, typeA),
        (_, false) => {
            let mut str1: ArcStr;
            let mut str2: ArcStr;
            str1 = TypesDump::unparseType(typeB)?;
            str2 = TypesDump::unparseType(typeC)?;
            Error::addSourceMessage(
                &(Error::REDUCTION_TYPE_ERROR.clone()),
                list![
                    literal!("second argument"),
                    literal!("result-type"),
                    literal!("identical"),
                    str1,
                    str2
                ],
                info,
            )?;
            return Err("fail");
        }
        (false, true) => {
            let mut str1: ArcStr;
            let mut str2: ArcStr;
            str1 = TypesDump::unparseType(typeA)?;
            str2 = TypesDump::unparseType(typeB)?;
            Error::addSourceMessage(
                &(Error::REDUCTION_TYPE_ERROR.clone()),
                list![
                    literal!("first"),
                    literal!("second arguments"),
                    literal!("identical"),
                    str1,
                    str2
                ],
                info,
            )?;
            return Err("fail");
        }
        (true, true) => {
            let mut str1: ArcStr;
            let mut str2: ArcStr;
            str1 = TypesDump::unparseType(expType)?;
            str2 = TypesDump::unparseType(typeA)?;
            Error::addSourceMessage(
                &(Error::REDUCTION_TYPE_ERROR.clone()),
                list![
                    literal!("reduction expression"),
                    literal!("first argument"),
                    literal!("compatible"),
                    str1,
                    str2
                ],
                info,
            )?;
            return Err("fail");
        }
        _ => return Err("match: no arm matched"),
    });
    Ok((outExp, outTy))
}

fn constToVariability(mut r#const: DAE::Const) -> Result<SCode::Variability> {
    let mut variability: SCode::Variability;
    variability = (match r#const {
        DAE::Const::C_VAR { .. } => openmodelica_frontend_types::SCode::Variability::VAR,
        DAE::Const::C_PARAM { .. } => openmodelica_frontend_types::SCode::Variability::PARAM,
        DAE::Const::C_CONST { .. } => openmodelica_frontend_types::SCode::Variability::CONST,
        DAE::Const::C_UNKNOWN { .. } => {
            let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else {
                return Err("pattern mismatch");
            };
            Debug::trace(literal!("- Static.constToVariability failed on DAE.C_UNKNOWN()\n"))?;
            return Err("fail");
        }
    });
    Ok(variability)
}

fn constructArrayType(
    mut arrayType: &metamodelica::Ref<DAE::Type>,
    mut expType: &metamodelica::Ref<DAE::Type>,
) -> Result<metamodelica::Ref<DAE::Type>> {
    let mut resType: metamodelica::Ref<DAE::Type>;
    resType = (::match_deref::match_deref! { match arrayType {
        Deref @ DAE::Type::T_UNKNOWN { .. } => {
            expType.clone()
        },
        Deref @ DAE::Type::T_ARRAY { dims: Deref @ metamodelica::ListNode::Cons { head: dim, tail: Deref @ metamodelica::ListNode::Nil }, ty } => {
            let mut ty = (*ty).clone();
            ty = constructArrayType(metamodelica::AsArg::as_arg(&ty), expType)?;
            metamodelica::Ref::new(DAE::Type::T_ARRAY { ty: ty.clone(), dims: list![dim.clone()] })
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok(resType)
}

fn elabCodeType(mut inCode: &metamodelica::Ref<Absyn::CodeNode>) -> Result<metamodelica::Ref<DAE::Type>> {
    let mut outType: metamodelica::Ref<DAE::Type>;
    outType = (match &**inCode {
        Absyn::CodeNode::C_TYPENAME { .. } => metamodelica::Ref::new(DAE::Type::T_CODE {
            ty: openmodelica_frontend_types::DAE::CodeType::C_TYPENAME,
        }),
        Absyn::CodeNode::C_VARIABLENAME { .. } => metamodelica::Ref::new(DAE::Type::T_CODE {
            ty: openmodelica_frontend_types::DAE::CodeType::C_VARIABLENAME,
        }),
        Absyn::CodeNode::C_EQUATIONSECTION { .. } => metamodelica::Ref::new(DAE::Type::T_COMPLEX {
            complexClassType: ClassInf::State::UNKNOWN {
                path: metamodelica::Ref::new(Absyn::Path::IDENT {
                    name: literal!("EquationSection"),
                }),
            },
            varLst: metamodelica::nil(),
            equalityConstraint: None,
            usedExternally: false,
        }),
        Absyn::CodeNode::C_ALGORITHMSECTION { .. } => metamodelica::Ref::new(DAE::Type::T_COMPLEX {
            complexClassType: ClassInf::State::UNKNOWN {
                path: metamodelica::Ref::new(Absyn::Path::IDENT {
                    name: literal!("AlgorithmSection"),
                }),
            },
            varLst: metamodelica::nil(),
            equalityConstraint: None,
            usedExternally: false,
        }),
        Absyn::CodeNode::C_ELEMENT { .. } => metamodelica::Ref::new(DAE::Type::T_COMPLEX {
            complexClassType: ClassInf::State::UNKNOWN {
                path: metamodelica::Ref::new(Absyn::Path::IDENT {
                    name: literal!("Element"),
                }),
            },
            varLst: metamodelica::nil(),
            equalityConstraint: None,
            usedExternally: false,
        }),
        Absyn::CodeNode::C_EXPRESSION { .. } => metamodelica::Ref::new(DAE::Type::T_CODE {
            ty: openmodelica_frontend_types::DAE::CodeType::C_EXPRESSION,
        }),
        Absyn::CodeNode::C_MODIFICATION { .. } => metamodelica::Ref::new(DAE::Type::T_CODE {
            ty: openmodelica_frontend_types::DAE::CodeType::C_EXPRESSION_OR_MODIFICATION,
        }),
        _ => return Err("match: no arm matched"),
    });
    Ok(outType)
}

pub fn elabGraphicsExp(
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
    mut inExp: metamodelica::Ref<Absyn::Exp>,
    mut inBoolean: bool,
    mut inPrefix: DAE::Prefix,
    mut info: &SourceInfo,
) -> Result<(FCore::Cache, metamodelica::Ref<DAE::Exp>, DAE::Properties)> {
    let mut outCache: FCore::Cache;
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    let mut outProperties: DAE::Properties;
    (outCache, outExp, outProperties) = 'mc: {
        let __mc_input = (inCache, inEnv, inExp.clone(), inBoolean, inPrefix);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, _, Deref @ Absyn::Exp::INTEGER { value: i }, _, _) => {
                    Ok((cache.clone(), metamodelica::Ref::new(DAE::Exp::ICONST { integer: i.clone() }), DAE::Properties::PROP { type_: DAE::T_INTEGER_DEFAULT().clone(), constFlag: openmodelica_frontend_types::DAE::Const::C_CONST }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, _, Deref @ Absyn::Exp::REAL { value: s }, _, _) => {
                    let mut r: metamodelica::Real;
                    r = stringReal(s.clone())?;
                    Ok((cache.clone(), metamodelica::Ref::new(DAE::Exp::RCONST { real: r }), DAE::Properties::PROP { type_: DAE::T_REAL_DEFAULT().clone(), constFlag: openmodelica_frontend_types::DAE::Const::C_CONST }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, _, Deref @ Absyn::Exp::STRING { value: s }, _, _) => {
                    let mut s = (*s).clone();
                    s = System::unescapedString(s.clone());
                    Ok((cache.clone(), metamodelica::Ref::new(DAE::Exp::SCONST { string: s.clone() }), DAE::Properties::PROP { type_: DAE::T_STRING_DEFAULT().clone(), constFlag: openmodelica_frontend_types::DAE::Const::C_CONST }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, _, Deref @ Absyn::Exp::BOOL { value: b }, _, _) => {
                    Ok((cache.clone(), metamodelica::Ref::new(DAE::Exp::BCONST { bool: b.clone() }), DAE::Properties::PROP { type_: DAE::T_BOOL_DEFAULT().clone(), constFlag: openmodelica_frontend_types::DAE::Const::C_CONST }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, Deref @ Absyn::Exp::CREF { componentRef: cr }, r#impl, pre) => {
                    let mut dexp: metamodelica::Ref<DAE::Exp>;
                    let mut prop: DAE::Properties;
                    let mut cache = (*cache).clone();
                    let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(elabCref(cache.clone(), env.clone(), cr.clone(), r#impl.clone(), true, pre.clone(), info.clone())?) {
                        (__pa0, Some((__pa1, __pa2, _))) => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    cache = metamodelica::Own::own(__pa0);
                    dexp = metamodelica::Own::own(__pa1);
                    prop = metamodelica::Own::own(__pa2);
                    Ok((cache.clone(), dexp.clone(), prop.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, exp @ Deref @ Absyn::Exp::BINARY { exp1: e1, op, exp2: e2 }, r#impl, pre) => {
                    let mut dexp: metamodelica::Ref<DAE::Exp>;
                    let mut e1_1: metamodelica::Ref<DAE::Exp>;
                    let mut e2_1: metamodelica::Ref<DAE::Exp>;
                    let mut prop: DAE::Properties;
                    let mut prop1: DAE::Properties;
                    let mut prop2: DAE::Properties;
                    let mut cache = (*cache).clone();
                    (cache, e1_1, prop1) = elabGraphicsExp(cache.clone(), env.clone(), e1.clone(), r#impl.clone(), pre.clone(), info)?;
                    (cache, e2_1, prop2) = elabGraphicsExp(cache.clone(), env.clone(), e2.clone(), r#impl.clone(), pre.clone(), info)?;
                    (cache, dexp, prop) = OperatorOverloading::binary(cache.clone(), env.clone(), op.clone(), prop1.clone(), e1_1.clone(), prop2.clone(), e2_1.clone(), metamodelica::AsArg::as_arg(&exp), metamodelica::AsArg::as_arg(&e1), metamodelica::AsArg::as_arg(&e2), r#impl.clone(), metamodelica::AsArg::as_arg(&pre), info)?;
                    Ok((cache.clone(), dexp.clone(), prop.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, e @ Deref @ Absyn::Exp::UNARY { op: Absyn::Operator::UPLUS { .. }, .. }, r#impl, pre) => {
                    let mut e_1: metamodelica::Ref<DAE::Exp>;
                    let mut prop: DAE::Properties;
                    let mut t: metamodelica::Ref<DAE::Type>;
                    let mut c: DAE::Const;
                    let mut cache = (*cache).clone();
                    let (__pa0, __pa1, __pa2, __pa3) = ::match_deref::match_deref! { match &(elabGraphicsExp(cache.clone(), env.clone(), e.clone(), r#impl.clone(), pre.clone(), info)?) {
                        (__pa0, __pa1, DAE::Properties::PROP { type_: __pa2, constFlag: __pa3 }) => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    cache = metamodelica::Own::own(__pa0);
                    e_1 = metamodelica::Own::own(__pa1);
                    t = metamodelica::Own::own(__pa2);
                    c = metamodelica::Own::own(__pa3);
                    let true = (Types::isRealOrSubTypeReal(Types::arrayElementType(&t))) else { return Err("pattern mismatch") };
                    prop = DAE::Properties::PROP { type_: t.clone(), constFlag: c };
                    Ok((cache.clone(), e_1.clone(), prop.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, exp @ Deref @ Absyn::Exp::UNARY { op, exp: e }, r#impl, pre) => {
                    let mut dexp: metamodelica::Ref<DAE::Exp>;
                    let mut e_1: metamodelica::Ref<DAE::Exp>;
                    let mut prop: DAE::Properties;
                    let mut prop1: DAE::Properties;
                    let mut cache = (*cache).clone();
                    (cache, e_1, prop1) = elabGraphicsExp(cache.clone(), env.clone(), e.clone(), r#impl.clone(), pre.clone(), info)?;
                    (cache, dexp, prop) = OperatorOverloading::unary(cache.clone(), env.clone(), op.clone(), &prop1, e_1.clone(), metamodelica::AsArg::as_arg(&exp), e.clone(), r#impl.clone(), metamodelica::AsArg::as_arg(&pre), info)?;
                    Ok((cache.clone(), dexp.clone(), prop.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, exp @ Deref @ Absyn::Exp::LBINARY { exp1: e1, op, exp2: e2 }, r#impl, pre) => {
                    let mut dexp: metamodelica::Ref<DAE::Exp>;
                    let mut e1_1: metamodelica::Ref<DAE::Exp>;
                    let mut e2_1: metamodelica::Ref<DAE::Exp>;
                    let mut prop: DAE::Properties;
                    let mut prop1: DAE::Properties;
                    let mut prop2: DAE::Properties;
                    let mut cache = (*cache).clone();
                    (cache, e1_1, prop1) = elabGraphicsExp(cache.clone(), env.clone(), e1.clone(), r#impl.clone(), pre.clone(), info)?;
                    (cache, e2_1, prop2) = elabGraphicsExp(cache.clone(), env.clone(), e2.clone(), r#impl.clone(), pre.clone(), info)?;
                    (cache, dexp, prop) = OperatorOverloading::binary(cache.clone(), env.clone(), op.clone(), prop1.clone(), e1_1.clone(), prop2.clone(), e2_1.clone(), metamodelica::AsArg::as_arg(&exp), metamodelica::AsArg::as_arg(&e1), metamodelica::AsArg::as_arg(&e2), r#impl.clone(), metamodelica::AsArg::as_arg(&pre), info)?;
                    Ok((cache.clone(), dexp.clone(), prop.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, exp @ Deref @ Absyn::Exp::LUNARY { op, exp: e }, r#impl, pre) => {
                    let mut dexp: metamodelica::Ref<DAE::Exp>;
                    let mut e_1: metamodelica::Ref<DAE::Exp>;
                    let mut prop: DAE::Properties;
                    let mut prop1: DAE::Properties;
                    let mut cache = (*cache).clone();
                    (cache, e_1, prop1) = elabGraphicsExp(cache.clone(), env.clone(), e.clone(), r#impl.clone(), pre.clone(), info)?;
                    (cache, dexp, prop) = OperatorOverloading::unary(cache.clone(), env.clone(), op.clone(), &prop1, e_1.clone(), metamodelica::AsArg::as_arg(&exp), e.clone(), r#impl.clone(), metamodelica::AsArg::as_arg(&pre), info)?;
                    Ok((cache.clone(), dexp.clone(), prop.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, exp @ Deref @ Absyn::Exp::RELATION { exp1: e1, op, exp2: e2 }, r#impl, pre) => {
                    let mut dexp: metamodelica::Ref<DAE::Exp>;
                    let mut e1_1: metamodelica::Ref<DAE::Exp>;
                    let mut e2_1: metamodelica::Ref<DAE::Exp>;
                    let mut prop: DAE::Properties;
                    let mut prop1: DAE::Properties;
                    let mut prop2: DAE::Properties;
                    let mut cache = (*cache).clone();
                    (cache, e1_1, prop1) = elabGraphicsExp(cache.clone(), env.clone(), e1.clone(), r#impl.clone(), pre.clone(), info)?;
                    (cache, e2_1, prop2) = elabGraphicsExp(cache.clone(), env.clone(), e2.clone(), r#impl.clone(), pre.clone(), info)?;
                    (cache, dexp, prop) = OperatorOverloading::binary(cache.clone(), env.clone(), op.clone(), prop1.clone(), e1_1.clone(), prop2.clone(), e2_1.clone(), metamodelica::AsArg::as_arg(&exp), metamodelica::AsArg::as_arg(&e1), metamodelica::AsArg::as_arg(&e2), r#impl.clone(), metamodelica::AsArg::as_arg(&pre), info)?;
                    Ok((cache.clone(), dexp.clone(), prop.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, e @ Deref @ Absyn::Exp::IFEXP { .. }, r#impl, pre) => {
                    let mut e1_1: metamodelica::Ref<DAE::Exp>;
                    let mut e2_1: metamodelica::Ref<DAE::Exp>;
                    let mut e_1: metamodelica::Ref<DAE::Exp>;
                    let mut e3_1: metamodelica::Ref<DAE::Exp>;
                    let mut prop: DAE::Properties;
                    let mut prop1: DAE::Properties;
                    let mut prop2: DAE::Properties;
                    let mut prop3: DAE::Properties;
                    let mut e1: metamodelica::Ref<Absyn::Exp>;
                    let mut e2: metamodelica::Ref<Absyn::Exp>;
                    let mut e3: metamodelica::Ref<Absyn::Exp>;
                    let mut cache = (*cache).clone();
                    let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(AbsynUtil::canonIfExp(metamodelica::AsArg::as_arg(&e))?) {
                        Deref @ Absyn::Exp::IFEXP { ifExp: __pa0, trueBranch: __pa1, elseBranch: __pa2, .. } => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    e1 = metamodelica::Own::own(__pa0);
                    e2 = metamodelica::Own::own(__pa1);
                    e3 = metamodelica::Own::own(__pa2);
                    (cache, e1_1, prop1) = elabGraphicsExp(cache.clone(), env.clone(), e1.clone(), r#impl.clone(), pre.clone(), info)?;
                    (cache, e2_1, prop2) = elabGraphicsExp(cache.clone(), env.clone(), e2.clone(), r#impl.clone(), pre.clone(), info)?;
                    (cache, e3_1, prop3) = elabGraphicsExp(cache.clone(), env.clone(), e3.clone(), r#impl.clone(), pre.clone(), info)?;
                    (cache, e_1, prop) = makeIfExp(cache.clone(), env.clone(), e1_1.clone(), prop1.clone(), e2_1.clone(), prop2.clone(), e3_1.clone(), prop3.clone(), r#impl.clone(), pre.clone(), info)?;
                    Ok((cache.clone(), e_1.clone(), prop.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, Deref @ Absyn::Exp::CALL { function_: r#fn, functionArgs: Deref @ Absyn::FunctionArgs::FUNCTIONARGS { args, argNames: nargs }, .. }, _, pre) => {
                    let mut e_1: metamodelica::Ref<DAE::Exp>;
                    let mut prop: DAE::Properties;
                    let mut cache = (*cache).clone();
                    (cache, e_1, prop) = elabCall(cache.clone(), env.clone(), r#fn.clone(), args.clone(), nargs.clone(), var_field!((*inExp).typeVars, Absyn::Exp::CALL), true, pre.clone(), info.clone())?;
                    Ok((cache.clone(), e_1.clone(), prop.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, Deref @ Absyn::Exp::TUPLE { expressions: es @ Deref @ metamodelica::ListNode::Cons { head: _, tail: _ } }, r#impl, pre) => {
                    let mut es_1: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                    let mut props: metamodelica::List<DAE::Properties>;
                    let mut types: metamodelica::List<metamodelica::Ref<DAE::Type>>;
                    let mut consts: metamodelica::List<metamodelica::Ref<DAE::TupleConst>>;
                    let mut cache = (*cache).clone();
                    (cache, es_1, props) = elabTuple(cache.clone(), env.clone(), es.clone(), r#impl.clone(), false, pre.clone(), info.clone(), false)?;
                    (types, consts) = splitProps(props.clone());
                    Ok((cache.clone(), metamodelica::Ref::new(DAE::Exp::TUPLE { PR: es_1.clone() }), DAE::Properties::PROP_TUPLE { type_: metamodelica::Ref::new(DAE::Type::T_TUPLE { types: types.clone(), names: None }), tupleConst: metamodelica::Ref::new(DAE::TupleConst::TUPLE_CONST { tupleConstLst: consts.clone() }) }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, Deref @ Absyn::Exp::RANGE { start, step: None, stop }, r#impl, pre) => {
                    let mut start_1: metamodelica::Ref<DAE::Exp>;
                    let mut stop_1: metamodelica::Ref<DAE::Exp>;
                    let mut t: metamodelica::Ref<DAE::Type>;
                    let mut start_t: metamodelica::Ref<DAE::Type>;
                    let mut stop_t: metamodelica::Ref<DAE::Type>;
                    let mut c_start: DAE::Const;
                    let mut c_stop: DAE::Const;
                    let mut r#const: DAE::Const;
                    let mut rt: metamodelica::Ref<DAE::Type>;
                    let mut cache = (*cache).clone();
                    let (__pa0, __pa1, __pa2, __pa3) = ::match_deref::match_deref! { match &(elabGraphicsExp(cache.clone(), env.clone(), start.clone(), r#impl.clone(), pre.clone(), info)?) {
                        (__pa0, __pa1, DAE::Properties::PROP { type_: __pa2, constFlag: __pa3 }) => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    cache = metamodelica::Own::own(__pa0);
                    start_1 = metamodelica::Own::own(__pa1);
                    start_t = metamodelica::Own::own(__pa2);
                    c_start = metamodelica::Own::own(__pa3);
                    let (__pa4, __pa5, __pa6, __pa7) = ::match_deref::match_deref! { match &(elabGraphicsExp(cache.clone(), env.clone(), stop.clone(), r#impl.clone(), pre.clone(), info)?) {
                        (__pa4, __pa5, DAE::Properties::PROP { type_: __pa6, constFlag: __pa7 }) => (__pa4.clone(), __pa5.clone(), __pa6.clone(), __pa7.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    cache = metamodelica::Own::own(__pa4);
                    stop_1 = metamodelica::Own::own(__pa5);
                    stop_t = metamodelica::Own::own(__pa6);
                    c_stop = metamodelica::Own::own(__pa7);
                    let __pa8 = ::match_deref::match_deref! { match &(deoverloadRange(start_1.clone(), start_t.clone(), None, None, stop_1.clone(), stop_t.clone(), info)?) {
                        (_, None, _, __pa8) => __pa8.clone(),
                        _ => return Err("pattern mismatch"),
                    } };
                    rt = metamodelica::Own::own(__pa8);
                    r#const = Types::constAnd(c_start, c_stop);
                    (cache, t) = elabRangeType(cache.clone(), env.clone(), start_1.clone(), None, stop_1.clone(), start_t.clone(), &rt, r#const, r#impl.clone());
                    Ok((cache.clone(), metamodelica::Ref::new(DAE::Exp::RANGE { ty: t.clone(), start: start_1.clone(), step: None, stop: stop_1.clone() }), DAE::Properties::PROP { type_: t.clone(), constFlag: r#const }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, Deref @ Absyn::Exp::RANGE { start, step: Some(step), stop }, r#impl, pre) => {
                    let mut start_1: metamodelica::Ref<DAE::Exp>;
                    let mut stop_1: metamodelica::Ref<DAE::Exp>;
                    let mut start_2: metamodelica::Ref<DAE::Exp>;
                    let mut stop_2: metamodelica::Ref<DAE::Exp>;
                    let mut step_1: metamodelica::Ref<DAE::Exp>;
                    let mut step_2: metamodelica::Ref<DAE::Exp>;
                    let mut t: metamodelica::Ref<DAE::Type>;
                    let mut start_t: metamodelica::Ref<DAE::Type>;
                    let mut stop_t: metamodelica::Ref<DAE::Type>;
                    let mut step_t: metamodelica::Ref<DAE::Type>;
                    let mut c1: DAE::Const;
                    let mut c_start: DAE::Const;
                    let mut c_stop: DAE::Const;
                    let mut r#const: DAE::Const;
                    let mut c_step: DAE::Const;
                    let mut rt: metamodelica::Ref<DAE::Type>;
                    let mut cache = (*cache).clone();
                    let (__pa0, __pa1, __pa2, __pa3) = ::match_deref::match_deref! { match &(elabGraphicsExp(cache.clone(), env.clone(), start.clone(), r#impl.clone(), pre.clone(), info)?) {
                        (__pa0, __pa1, DAE::Properties::PROP { type_: __pa2, constFlag: __pa3 }) => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    cache = metamodelica::Own::own(__pa0);
                    start_1 = metamodelica::Own::own(__pa1);
                    start_t = metamodelica::Own::own(__pa2);
                    c_start = metamodelica::Own::own(__pa3);
                    let (__pa4, __pa5, __pa6, __pa7) = ::match_deref::match_deref! { match &(elabGraphicsExp(cache.clone(), env.clone(), step.clone(), r#impl.clone(), pre.clone(), info)?) {
                        (__pa4, __pa5, DAE::Properties::PROP { type_: __pa6, constFlag: __pa7 }) => (__pa4.clone(), __pa5.clone(), __pa6.clone(), __pa7.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    cache = metamodelica::Own::own(__pa4);
                    step_1 = metamodelica::Own::own(__pa5);
                    step_t = metamodelica::Own::own(__pa6);
                    c_step = metamodelica::Own::own(__pa7);
                    let (__pa8, __pa9, __pa10, __pa11) = ::match_deref::match_deref! { match &(elabGraphicsExp(cache.clone(), env.clone(), stop.clone(), r#impl.clone(), pre.clone(), info)?) {
                        (__pa8, __pa9, DAE::Properties::PROP { type_: __pa10, constFlag: __pa11 }) => (__pa8.clone(), __pa9.clone(), __pa10.clone(), __pa11.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    cache = metamodelica::Own::own(__pa8);
                    stop_1 = metamodelica::Own::own(__pa9);
                    stop_t = metamodelica::Own::own(__pa10);
                    c_stop = metamodelica::Own::own(__pa11);
                    let (__pa12, __pa13, __pa14, __pa15) = ::match_deref::match_deref! { match &(deoverloadRange(start_1.clone(), start_t.clone(), Some(step_1.clone()), Some(step_t.clone()), stop_1.clone(), stop_t.clone(), info)?) {
                        (__pa12, Some(__pa13), __pa14, __pa15) => (__pa12.clone(), __pa13.clone(), __pa14.clone(), __pa15.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    start_2 = metamodelica::Own::own(__pa12);
                    step_2 = metamodelica::Own::own(__pa13);
                    stop_2 = metamodelica::Own::own(__pa14);
                    rt = metamodelica::Own::own(__pa15);
                    c1 = Types::constAnd(c_start, c_step);
                    r#const = Types::constAnd(c1, c_stop);
                    (cache, t) = elabRangeType(cache.clone(), env.clone(), start_1.clone(), Some(step_1.clone()), stop_1.clone(), start_t.clone(), &rt, r#const, r#impl.clone());
                    Ok((cache.clone(), metamodelica::Ref::new(DAE::Exp::RANGE { ty: t.clone(), start: start_2.clone(), step: Some(step_2.clone()), stop: stop_2.clone() }), DAE::Properties::PROP { type_: t.clone(), constFlag: r#const }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, Deref @ Absyn::Exp::ARRAY { arrayExp: es }, r#impl, pre) => {
                    let mut l: i32;
                    let mut a: bool;
                    let mut t: metamodelica::Ref<DAE::Type>;
                    let mut r#const: DAE::Const;
                    let mut es_1: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                    let mut at: metamodelica::Ref<DAE::Type>;
                    let mut cache = (*cache).clone();
                    let (__pa0, __pa1, __pa2, __pa3) = ::match_deref::match_deref! { match &(elabGraphicsArray(cache.clone(), env.clone(), metamodelica::AsArg::as_arg(&es), r#impl.clone(), pre.clone(), info)?) {
                        (__pa0, __pa1, DAE::Properties::PROP { type_: __pa2, constFlag: __pa3 }) => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    cache = metamodelica::Own::own(__pa0);
                    es_1 = metamodelica::Own::own(__pa1);
                    t = metamodelica::Own::own(__pa2);
                    r#const = metamodelica::Own::own(__pa3);
                    l = ((es_1).len() as i32);
                    at = Types::simplifyType(t.clone())?;
                    a = Types::isArray(&t);
                    Ok((cache.clone(), metamodelica::Ref::new(DAE::Exp::ARRAY { ty: at.clone(), scalar: a, array: es_1.clone() }), DAE::Properties::PROP { type_: metamodelica::Ref::new(DAE::Type::T_ARRAY { ty: t.clone(), dims: list![metamodelica::Ref::new(DAE::Dimension::DIM_INTEGER { integer: l })] }), constFlag: r#const }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, Deref @ Absyn::Exp::MATRIX { matrix: ess }, r#impl, pre) => {
                    let mut nmax: i32;
                    let mut dim1: metamodelica::Ref<DAE::Dimension>;
                    let mut dim2: metamodelica::Ref<DAE::Dimension>;
                    let mut havereal: bool;
                    let mut mexp: metamodelica::Ref<DAE::Exp>;
                    let mut t: metamodelica::Ref<DAE::Type>;
                    let mut t_1: metamodelica::Ref<DAE::Type>;
                    let mut t_2: metamodelica::Ref<DAE::Type>;
                    let mut c: DAE::Const;
                    let mut tps_2: metamodelica::List<metamodelica::Ref<DAE::Type>>;
                    let mut tps: metamodelica::List<metamodelica::List<DAE::Properties>>;
                    let mut tps_1: metamodelica::List<metamodelica::List<metamodelica::Ref<DAE::Type>>>;
                    let mut dess: metamodelica::List<metamodelica::List<metamodelica::Ref<DAE::Exp>>>;
                    let mut cache = (*cache).clone();
                    (cache, dess, tps) = elabExpListList(cache.clone(), env.clone(), metamodelica::AsArg::as_arg(&ess), r#impl.clone(), true, pre.clone(), info.clone(), DAE::T_UNKNOWN_DEFAULT().clone())?;
                    tps_1 = List::mapList(tps.clone(), &move |__a0: DAE::Properties| -> metamodelica::Result<_> { ::std::result::Result::Ok(Types::getPropType(&__a0)) })?;
                    tps_2 = List::flatten(tps_1.clone())?;
                    nmax = matrixConstrMaxDim(&tps_2);
                    havereal = Types::containReal(&tps_2);
                    let (__pa0, __pa1, __pa2, __pa3, __pa4, __pa5) = ::match_deref::match_deref! { match &(elabMatrixSemi(cache.clone(), metamodelica::AsArg::as_arg(&env), &dess, &tps, r#impl.clone(), havereal, nmax, true, pre.clone(), info)?) {
                        (__pa0, __pa1, DAE::Properties::PROP { type_: __pa2, constFlag: __pa3 }, __pa4, __pa5) => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone(), __pa4.clone(), __pa5.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    cache = metamodelica::Own::own(__pa0);
                    mexp = metamodelica::Own::own(__pa1);
                    t = metamodelica::Own::own(__pa2);
                    c = metamodelica::Own::own(__pa3);
                    dim1 = metamodelica::Own::own(__pa4);
                    dim2 = metamodelica::Own::own(__pa5);
                    elabMatrixToMatrixExp(mexp.clone());
                    t_1 = Types::unliftArray(&t)?;
                    t_2 = Types::unliftArray(&t_1)?;
                    Ok((cache.clone(), mexp.clone(), DAE::Properties::PROP { type_: metamodelica::Ref::new(DAE::Type::T_ARRAY { ty: metamodelica::Ref::new(DAE::Type::T_ARRAY { ty: t_2.clone(), dims: list![dim2.clone()] }), dims: list![dim1.clone()] }), constFlag: c }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, e, _, pre) => {
                    let mut s: ArcStr;
                    let mut ps: ArcStr;
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { return Err("pattern mismatch") };
                    Print::printErrorBuf(literal!("- Inst.elabGraphicsExp failed: "))?;
                    ps = PrefixUtil::printPrefixStr2(pre.clone())?;
                    s = Dump::printExpStr(e.clone())?;
                    Print::printErrorBuf({ let mut __mm_s = String::new(); __mm_s.push_str(&*ps); __mm_s.push_str(&*s); ArcStr::from(__mm_s) })?;
                    Print::printErrorBuf(literal!("\n"))?;
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((outCache, outExp, outProperties))
}

fn deoverloadRange(
    mut inStartExp: metamodelica::Ref<DAE::Exp>,
    mut inStartType: metamodelica::Ref<DAE::Type>,
    mut inStepExp: Option<metamodelica::Ref<DAE::Exp>>,
    mut inStepType: Option<metamodelica::Ref<DAE::Type>>,
    mut inStopExp: metamodelica::Ref<DAE::Exp>,
    mut inStopType: metamodelica::Ref<DAE::Type>,
    mut inInfo: &SourceInfo,
) -> Result<(
    metamodelica::Ref<DAE::Exp>,
    Option<metamodelica::Ref<DAE::Exp>>,
    metamodelica::Ref<DAE::Exp>,
    metamodelica::Ref<DAE::Type>,
)> {
    let mut outStart: metamodelica::Ref<DAE::Exp>;
    let mut outStep: Option<metamodelica::Ref<DAE::Exp>>;
    let mut outStop: metamodelica::Ref<DAE::Exp>;
    let mut outRangeType: metamodelica::Ref<DAE::Type>;
    (outStart, outStep, outStop, outRangeType) = (::match_deref::match_deref! { match &((inStartType.clone(), inStepType, inStopType.clone())) {
        (Deref @ DAE::Type::T_BOOL { .. }, None, Deref @ DAE::Type::T_BOOL { .. }) => {
            (inStartExp, None, inStopExp, DAE::T_BOOL_DEFAULT().clone())
        },
        (Deref @ DAE::Type::T_INTEGER { .. }, None, Deref @ DAE::Type::T_INTEGER { .. }) => {
            (inStartExp, inStepExp, inStopExp, DAE::T_INTEGER_DEFAULT().clone())
        },
        (Deref @ DAE::Type::T_INTEGER { .. }, Some(Deref @ DAE::Type::T_INTEGER { .. }), Deref @ DAE::Type::T_INTEGER { .. }) => {
            (inStartExp, inStepExp, inStopExp, DAE::T_INTEGER_DEFAULT().clone())
        },
        (Deref @ DAE::Type::T_ENUMERATION { names: ns, .. }, None, Deref @ DAE::Type::T_ENUMERATION { names: ne, .. }) => {
            let mut et: metamodelica::Ref<DAE::Type>;
            let mut e1_str: ArcStr;
            let mut e2_str: ArcStr;
            let mut t1_str: ArcStr;
            if List::isEqual(ns.clone(), ne.clone(), true)? {
                et = Types::simplifyType(inStartType)?;
            } else {
                e1_str = ExpressionBasics::printExpStr(inStartExp.clone())?;
                e2_str = ExpressionBasics::printExpStr(inStopExp.clone())?;
                t1_str = TypesDump::unparseTypeNoAttr(&inStartType)?;
                Error::addSourceMessageAndFail(&(Error::UNRESOLVABLE_TYPE.clone()), list![{ let mut __mm_s = String::new(); __mm_s.push_str(&*e1_str); __mm_s.push_str(&*literal!(":")); __mm_s.push_str(&*e2_str); ArcStr::from(__mm_s) }, { let mut __mm_s = String::new(); __mm_s.push_str(&*t1_str); __mm_s.push_str(&*literal!(", ")); __mm_s.push_str(&*t1_str); ArcStr::from(__mm_s) }, literal!("")], inInfo)?;
                unreachable!("Error.addSourceMessageAndFail always fails — caller-side flow-analysis hint");
            }
            (inStartExp, None, inStopExp, et)
        },
        (_, None, _) => {
            let (__pa0, __pa1) = ::match_deref::match_deref! { match &(OperatorOverloading::elabArglist(&(list![DAE::T_REAL_DEFAULT().clone(), DAE::T_REAL_DEFAULT().clone()]), &(list![(inStartExp, inStartType), (inStopExp, inStopType)]))?) {
                (Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: Deref @ metamodelica::ListNode::Cons { head: __pa1, tail: Deref @ metamodelica::ListNode::Nil } }, _) => (__pa0.clone(), __pa1.clone()),
                _ => return Err("pattern mismatch"),
            } };
            outStart = metamodelica::Own::own(__pa0);
            outStop = metamodelica::Own::own(__pa1);
            (outStart, None, outStop, DAE::T_REAL_DEFAULT().clone())
        },
        (_, Some(step_ty), _) => {
            let mut step_exp: metamodelica::Ref<DAE::Exp>;
            let __pa0 = ::match_deref::match_deref! { match &(inStepExp) {
                Some(__pa0) => __pa0.clone(),
                _ => return Err("pattern mismatch"),
            } };
            step_exp = metamodelica::Own::own(__pa0);
            let (__pa1, __pa2, __pa3) = ::match_deref::match_deref! { match &(OperatorOverloading::elabArglist(&(list![DAE::T_REAL_DEFAULT().clone(), DAE::T_REAL_DEFAULT().clone(), DAE::T_REAL_DEFAULT().clone()]), &(list![(inStartExp, inStartType), (step_exp, step_ty.clone()), (inStopExp, inStopType)]))?) {
                (Deref @ metamodelica::ListNode::Cons { head: __pa1, tail: Deref @ metamodelica::ListNode::Cons { head: __pa2, tail: Deref @ metamodelica::ListNode::Cons { head: __pa3, tail: Deref @ metamodelica::ListNode::Nil } } }, _) => (__pa1.clone(), __pa2.clone(), __pa3.clone()),
                _ => return Err("pattern mismatch"),
            } };
            outStart = metamodelica::Own::own(__pa1);
            step_exp = metamodelica::Own::own(__pa2);
            outStop = metamodelica::Own::own(__pa3);
            (outStart, Some(step_exp), outStop, DAE::T_REAL_DEFAULT().clone())
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok((outStart, outStep, outStop, outRangeType))
}

fn elabRangeType(
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
    mut inStart: metamodelica::Ref<DAE::Exp>,
    mut inStep: Option<metamodelica::Ref<DAE::Exp>>,
    mut inStop: metamodelica::Ref<DAE::Exp>,
    mut inType: metamodelica::Ref<DAE::Type>,
    mut inExpType: &metamodelica::Ref<DAE::Type>,
    mut co: DAE::Const,
    mut inImpl: bool,
) -> (FCore::Cache, metamodelica::Ref<DAE::Type>) {
    let mut outCache: FCore::Cache;
    let mut outType: metamodelica::Ref<DAE::Type>;
    (outCache, outType) = 'mc: {
        let __mc_input = (inStep, co);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, DAE::Const::C_VAR { .. }) => {
                    Ok((inCache.clone(), metamodelica::Ref::new(DAE::Type::T_ARRAY { ty: inType.clone(), dims: list![openmodelica_frontend_types::DAE::Dimension::interned_DIM_UNKNOWN()] })))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (None, _) => {
                    let mut start_val: metamodelica::Ref<Values::Value>;
                    let mut stop_val: metamodelica::Ref<Values::Value>;
                    let mut dim: i32;
                    let mut cache: FCore::Cache;
                    (cache, start_val) = Ceval::ceval(inCache.clone(), inEnv.clone(), inStart.clone(), inImpl, Absyn::Msg::MSG { info: Absyn::dummyInfo.clone() }, 0)?;
                    (cache, stop_val) = Ceval::ceval(cache.clone(), inEnv.clone(), inStop.clone(), inImpl, Absyn::Msg::MSG { info: Absyn::dummyInfo.clone() }, 0)?;
                    dim = elabRangeSize(&start_val, None, &stop_val)?;
                    Ok((cache.clone(), metamodelica::Ref::new(DAE::Type::T_ARRAY { ty: inType.clone(), dims: list![metamodelica::Ref::new(DAE::Dimension::DIM_INTEGER { integer: dim })] })))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Some(step_exp), _) => {
                    let mut start_val: metamodelica::Ref<Values::Value>;
                    let mut step_val: metamodelica::Ref<Values::Value>;
                    let mut stop_val: metamodelica::Ref<Values::Value>;
                    let mut dim: i32;
                    let mut cache: FCore::Cache;
                    (cache, start_val) = Ceval::ceval(inCache.clone(), inEnv.clone(), inStart.clone(), inImpl, Absyn::Msg::MSG { info: Absyn::dummyInfo.clone() }, 0)?;
                    (cache, step_val) = Ceval::ceval(cache.clone(), inEnv.clone(), step_exp.clone(), inImpl, Absyn::Msg::MSG { info: Absyn::dummyInfo.clone() }, 0)?;
                    (cache, stop_val) = Ceval::ceval(cache.clone(), inEnv.clone(), inStop.clone(), inImpl, Absyn::Msg::MSG { info: Absyn::dummyInfo.clone() }, 0)?;
                    dim = elabRangeSize(&start_val, Some(step_val.clone()), &stop_val)?;
                    Ok((cache.clone(), metamodelica::Ref::new(DAE::Type::T_ARRAY { ty: inType.clone(), dims: list![metamodelica::Ref::new(DAE::Dimension::DIM_INTEGER { integer: dim })] })))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((inCache.clone(), metamodelica::Ref::new(DAE::Type::T_ARRAY { ty: inType.clone(), dims: list![openmodelica_frontend_types::DAE::Dimension::interned_DIM_UNKNOWN()] })))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    (outCache, outType)
}

fn elabRangeSize(
    mut inStartValue: &metamodelica::Ref<Values::Value>,
    mut inStepValue: Option<metamodelica::Ref<Values::Value>>,
    mut inStopValue: &metamodelica::Ref<Values::Value>,
) -> Result<i32> {
    let mut outSize: i32;
    outSize = 'mc: {
        let __mc_input = (&**inStartValue, inStepValue, &**inStopValue);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, None, _) => {
                    let false = (ValuesUtil::safeLessEq(inStartValue, inStopValue)?) else { return Err("pattern mismatch") };
                    Ok(0)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ Values::Value::INTEGER { integer: int_start }, None, Deref @ Values::Value::INTEGER { integer: int_stop }) => {
                    let mut dim: i32;
                    dim = int_stop.clone() - int_start.clone() + 1;
                    Ok(dim)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ Values::Value::INTEGER { integer: int_start }, Some(Deref @ Values::Value::INTEGER { integer: int_step }), Deref @ Values::Value::INTEGER { integer: int_stop }) => {
                    let mut dim: i32;
                    dim = int_stop.clone() - int_start.clone();
                    dim = intDiv(dim, int_step.clone()) + 1;
                    Ok(dim)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ Values::Value::REAL { real: real_start }, None, Deref @ Values::Value::REAL { real: real_stop }) => {
                    Ok(Util::realRangeSize(real_start.clone(), metamodelica::OrderedFloat(1.0_f64), real_stop.clone())?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ Values::Value::REAL { real: real_start }, Some(Deref @ Values::Value::REAL { real: real_step }), Deref @ Values::Value::REAL { real: real_stop }) => {
                    Ok(Util::realRangeSize(real_start.clone(), real_step.clone(), real_stop.clone())?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ Values::Value::ENUM_LITERAL { index: int_start, .. }, None, Deref @ Values::Value::ENUM_LITERAL { index: int_stop, .. }) => {
                    let mut dim: i32;
                    dim = int_stop.clone() - int_start.clone() + 1;
                    Ok(dim)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ Values::Value::BOOL { boolean: true }, None, Deref @ Values::Value::BOOL { boolean: false }) => {
                    Ok(0)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ Values::Value::BOOL { boolean: false }, None, Deref @ Values::Value::BOOL { boolean: true }) => {
                    Ok(2)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ Values::Value::BOOL { boolean: _ }, None, Deref @ Values::Value::BOOL { boolean: _ }) => {
                    Ok(1)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(outSize)
}

fn elabTuple(
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
    mut inExpl: metamodelica::List<metamodelica::Ref<Absyn::Exp>>,
    mut inImplicit: bool,
    mut inDoVect: bool,
    mut inPrefix: DAE::Prefix,
    mut inInfo: SourceInfo,
    mut isLhs: bool,
) -> Result<(
    FCore::Cache,
    metamodelica::List<metamodelica::Ref<DAE::Exp>>,
    metamodelica::List<DAE::Properties>,
)> {
    let mut outCache: FCore::Cache = inCache;
    let mut outExpl: metamodelica::List<metamodelica::Ref<DAE::Exp>> = metamodelica::nil();
    let mut outProperties: metamodelica::List<DAE::Properties> = metamodelica::nil();
    let mut exp: metamodelica::Ref<DAE::Exp>;
    let mut prop: DAE::Properties;
    if if (!(isLhs)) {
        !(Config::acceptMetaModelicaGrammar()?)
    } else {
        false
    } {
        Error::addSourceMessage(
            &(Error::RHS_TUPLE_EXPRESSION.clone()),
            list![Dump::printExpStr(metamodelica::Ref::new(Absyn::Exp::TUPLE {
                expressions: inExpl.clone()
            }))?],
            &inInfo,
        )?;
        return Err("fail");
    }
    for mut e in &*inExpl {
        (outCache, exp, prop) = elabExp(
            outCache,
            inEnv.clone(),
            e.clone(),
            inImplicit,
            inDoVect,
            inPrefix.clone(),
            inInfo.clone(),
        )?;
        if AbsynUtil::isTuple(metamodelica::AsArg::as_arg(&e)) {
            (exp, prop) = Types::matchProp(
                exp,
                &prop,
                &(DAE::Properties::PROP {
                    type_: DAE::T_METABOXED_DEFAULT().clone(),
                    constFlag: openmodelica_frontend_types::DAE::Const::C_CONST,
                }),
                true,
            )?;
        }
        outExpl = metamodelica::cons(exp, outExpl);
        outProperties = metamodelica::cons(prop, outProperties);
    }
    outExpl = outExpl.reverse();
    outProperties = outProperties.reverse();
    Ok((outCache, outExpl, outProperties))
}

fn stripExtraArgsFromType(
    mut slots: &metamodelica::List<Slot>,
    mut inType: metamodelica::Ref<DAE::Type>,
) -> Result<metamodelica::Ref<DAE::Type>> {
    let mut outType: metamodelica::Ref<DAE::Type> = inType;
    outType = 'mc: {
        let __mc_input = outType.clone();
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Type::T_FUNCTION { .. } => {
                    let mut outType: metamodelica::Ref<DAE::Type> = outType.clone();
                    assign_variant_field!(outType => DAE::Type::T_FUNCTION; funcArg = stripExtraArgsFromType2(slots, var_field!((*outType).funcArg, DAE::Type::T_FUNCTION), metamodelica::nil())?);
                    Ok((outType.clone(), outType.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            outType = __wb0;
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { return Err("pattern mismatch") };
                    Debug::trace(literal!("- Static.stripExtraArgsFromType failed\n"))?;
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(outType)
}

fn stripExtraArgsFromType2<'__b>(
    mut inSlots: &'__b metamodelica::List<Slot>,
    mut inType: &'__b metamodelica::List<metamodelica::Ref<DAE::FuncArg>>,
    mut inAccumType: metamodelica::List<metamodelica::Ref<DAE::FuncArg>>,
) -> Result<metamodelica::List<metamodelica::Ref<DAE::FuncArg>>> {
    '__tco: loop {
        ::match_deref::match_deref! { match (inSlots, inType) {
            (Deref @ metamodelica::ListNode::Cons { head: Slot { slotFilled: true, .. }, tail: slotsRest }, Deref @ metamodelica::ListNode::Cons { head: _, tail: rest }) => {
                { (inSlots, inType, inAccumType) = (slotsRest, rest, inAccumType); continue '__tco; }
            },
            (Deref @ metamodelica::ListNode::Cons { head: Slot { slotFilled: false, .. }, tail: slotsRest }, Deref @ metamodelica::ListNode::Cons { head: arg, tail: rest }) => {
                { (inSlots, inType, inAccumType) = (slotsRest, rest, metamodelica::cons(arg.clone(), inAccumType)); continue '__tco; }
            },
            (Deref @ metamodelica::ListNode::Nil, Deref @ metamodelica::ListNode::Nil) => {
                return Ok(inAccumType.reverse())
            },
            _ => return Err("match: no arm matched"),
        } }
    }
}

fn elabArray(
    mut inExpl: metamodelica::List<metamodelica::Ref<DAE::Exp>>,
    mut inProps: &metamodelica::List<DAE::Properties>,
    mut inPrefix: &DAE::Prefix,
    mut inInfo: &SourceInfo,
) -> Result<(metamodelica::List<metamodelica::Ref<DAE::Exp>>, DAE::Properties)> {
    let mut outExpLst: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
    let mut outProperties: DAE::Properties;
    let mut types: metamodelica::List<metamodelica::Ref<DAE::Type>> = metamodelica::nil();
    let mut ty: metamodelica::Ref<DAE::Type>;
    let mut c: DAE::Const = openmodelica_frontend_types::DAE::Const::C_CONST;
    let mut c2: DAE::Const;
    let mut mixed: bool;
    if (inExpl).is_empty() {
        Error::addSourceMessage(&(Error::EMPTY_ARRAY.clone()), metamodelica::nil(), inInfo)?;
        return Err("fail");
    }
    for mut p in &**inProps {
        let DAE::PROP {
            type_: __pa0,
            constFlag: __pa1,
        } = (p.clone())
        else {
            return Err("pattern mismatch");
        };
        ty = metamodelica::Own::own(__pa0);
        c2 = metamodelica::Own::own(__pa1);
        types = metamodelica::cons(ty, types);
        c = Types::constAnd(c, c2);
    }
    types = types.reverse();
    (ty, mixed) = elabArrayHasMixedIntReals(&types)?;
    if mixed {
        outExpLst = elabArrayReal2(inExpl, &types, ty.clone())?;
    } else {
        (outExpLst, ty) = elabArray2(inExpl, &types, inPrefix, inInfo)?;
    }
    outProperties = DAE::Properties::PROP {
        type_: ty,
        constFlag: c,
    };
    Ok((outExpLst, outProperties))
}

fn elabArrayHasMixedIntReals(
    mut inTypes: &metamodelica::List<metamodelica::Ref<DAE::Type>>,
) -> Result<(metamodelica::Ref<DAE::Type>, bool)> {
    let mut outType: metamodelica::Ref<DAE::Type>;
    let mut outIsMixed: bool = true;
    let mut ty: metamodelica::Ref<DAE::Type>;
    let mut rest_tys: metamodelica::List<metamodelica::Ref<DAE::Type>>;
    let (__pa0, __pa1) = ::match_deref::match_deref! { match &((*inTypes)) {
        Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
        _ => return Err("pattern mismatch"),
    } };
    outType = metamodelica::Own::own(__pa0);
    rest_tys = metamodelica::Own::own(__pa1);
    if Types::isReal(&outType) {
        while !((rest_tys).is_empty()) {
            let (__pa2, __pa3) = ::match_deref::match_deref! { match &(rest_tys) {
                Deref @ metamodelica::ListNode::Cons { head: __pa2, tail: __pa3 } => (__pa2.clone(), __pa3.clone()),
                _ => return Err("pattern mismatch"),
            } };
            ty = metamodelica::Own::own(__pa2);
            rest_tys = metamodelica::Own::own(__pa3);
            if Types::isInteger(&ty) {
                return Ok((outType, outIsMixed));
            }
        }
    } else if Types::isInteger(&outType) {
        while !((rest_tys).is_empty()) {
            let (__pa4, __pa5) = ::match_deref::match_deref! { match &(rest_tys) {
                Deref @ metamodelica::ListNode::Cons { head: __pa4, tail: __pa5 } => (__pa4.clone(), __pa5.clone()),
                _ => return Err("pattern mismatch"),
            } };
            outType = metamodelica::Own::own(__pa4);
            rest_tys = metamodelica::Own::own(__pa5);
            if Types::isReal(&outType) {
                return Ok((outType, outIsMixed));
            }
        }
    }
    outIsMixed = false;
    Ok((outType, outIsMixed))
}

fn elabArrayConst(mut inProperties: &metamodelica::List<DAE::Properties>) -> Result<DAE::Const> {
    let mut outConst: DAE::Const = openmodelica_frontend_types::DAE::Const::C_CONST;
    for mut prop in &**inProperties {
        outConst = Types::constAnd(outConst, Types::getPropConst(prop.clone())?);
    }
    Ok(outConst)
}

fn elabArrayReal2(
    mut inExpl: metamodelica::List<metamodelica::Ref<DAE::Exp>>,
    mut inTypes: &metamodelica::List<metamodelica::Ref<DAE::Type>>,
    mut inExpectedType: metamodelica::Ref<DAE::Type>,
) -> Result<metamodelica::List<metamodelica::Ref<DAE::Exp>>> {
    let mut outExpl: metamodelica::List<metamodelica::Ref<DAE::Exp>> = metamodelica::nil();
    let mut exp: metamodelica::Ref<DAE::Exp>;
    let mut rest_expl: metamodelica::List<metamodelica::Ref<DAE::Exp>> = inExpl;
    for mut ty in &**inTypes {
        let (__pa0, __pa1) = ::match_deref::match_deref! { match &(rest_expl) {
            Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
            _ => return Err("pattern mismatch"),
        } };
        exp = metamodelica::Own::own(__pa0);
        rest_expl = metamodelica::Own::own(__pa1);
        if !(Types::equivtypes(ty.clone(), inExpectedType.clone())) {
            (exp, _) = Types::matchType(exp, ty.clone(), inExpectedType.clone(), true)?;
        }
        outExpl = metamodelica::cons(exp, outExpl);
    }
    outExpl = outExpl.reverse();
    Ok(outExpl)
}

fn elabArray2(
    mut inExpl: metamodelica::List<metamodelica::Ref<DAE::Exp>>,
    mut inTypes: &metamodelica::List<metamodelica::Ref<DAE::Type>>,
    mut inPrefix: &DAE::Prefix,
    mut inInfo: &SourceInfo,
) -> Result<(
    metamodelica::List<metamodelica::Ref<DAE::Exp>>,
    metamodelica::Ref<DAE::Type>,
)> {
    let mut outExpl: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
    let mut outType: metamodelica::Ref<DAE::Type>;
    let mut ty2: metamodelica::Ref<DAE::Type>;
    let mut rest_tys: metamodelica::List<metamodelica::Ref<DAE::Type>>;
    let mut exp1: metamodelica::Ref<DAE::Exp>;
    let mut rest_expl: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
    let mut pre_str: ArcStr;
    let mut exp_str: ArcStr;
    let mut expl_str: ArcStr;
    let mut ty1_str: ArcStr;
    let mut ty2_str: ArcStr;
    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(inExpl.clone()) {
        Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
        _ => return Err("pattern mismatch"),
    } };
    exp1 = metamodelica::Own::own(__pa0);
    rest_expl = metamodelica::Own::own(__pa1);
    let (__pa2, __pa3) = ::match_deref::match_deref! { match &((*inTypes)) {
        Deref @ metamodelica::ListNode::Cons { head: __pa2, tail: __pa3 } => (__pa2.clone(), __pa3.clone()),
        _ => return Err("pattern mismatch"),
    } };
    outType = metamodelica::Own::own(__pa2);
    rest_tys = metamodelica::Own::own(__pa3);
    outExpl = list![exp1];
    outType = Types::getUniontypeIfMetarecordReplaceAllSubtypes(outType)?;
    for mut exp2 in &*rest_expl {
        let mut exp2 = exp2.clone();
        let (__pa4, __pa5) = ::match_deref::match_deref! { match &(rest_tys) {
            Deref @ metamodelica::ListNode::Cons { head: __pa4, tail: __pa5 } => (__pa4.clone(), __pa5.clone()),
            _ => return Err("pattern mismatch"),
        } };
        ty2 = metamodelica::Own::own(__pa4);
        rest_tys = metamodelica::Own::own(__pa5);
        ty2 = Types::getUniontypeIfMetarecordReplaceAllSubtypes(ty2)?;
        if !(Types::equivtypes(outType.clone(), ty2.clone())) {
            if let Ok((__pa6, __pa7)) = Types::matchType(exp2.clone(), outType.clone(), ty2.clone(), false) {
                exp2 = metamodelica::Own::own(__pa6);
                outType = metamodelica::Own::own(__pa7);
            } else {
                ty1_str = TypesDump::unparseTypeNoAttr(&outType)?;
                ty2_str = TypesDump::unparseTypeNoAttr(&ty2)?;
                Types::typeErrorSanityCheck(ty1_str.clone(), &ty2_str, inInfo)?;
                pre_str = PrefixUtil::printPrefixStr(inPrefix)?;
                exp_str = ExpressionBasics::printExpStr(exp2.clone())?;
                expl_str = List::toStringCustom(
                    inExpl.clone(),
                    &ExpressionBasics::printExpStr,
                    literal!(""),
                    literal!("["),
                    literal!(","),
                    literal!("]"),
                    true,
                    0,
                )?;
                Error::addSourceMessageAndFail(
                    &(Error::TYPE_MISMATCH_ARRAY_EXP.clone()),
                    list![
                        pre_str.clone(),
                        exp_str.clone(),
                        ty1_str.clone(),
                        expl_str.clone(),
                        ty2_str.clone()
                    ],
                    inInfo,
                )?;
                unreachable!("Error.addSourceMessageAndFail always fails — caller-side flow-analysis hint");
            }
        }
        outExpl = metamodelica::cons(exp2, outExpl);
    }
    outExpl = outExpl.reverse();
    Ok((outExpl, outType))
}

fn elabGraphicsArray(
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
    mut inExpl: &metamodelica::List<metamodelica::Ref<Absyn::Exp>>,
    mut inImplicit: bool,
    mut inPrefix: DAE::Prefix,
    mut inInfo: &SourceInfo,
) -> Result<(
    FCore::Cache,
    metamodelica::List<metamodelica::Ref<DAE::Exp>>,
    DAE::Properties,
)> {
    let mut outCache: FCore::Cache = inCache;
    let mut outExpl: metamodelica::List<metamodelica::Ref<DAE::Exp>> = metamodelica::nil();
    let mut outProperties: DAE::Properties;
    let mut c: DAE::Const = openmodelica_frontend_types::DAE::Const::C_CONST;
    let mut c2: DAE::Const;
    let mut exp: metamodelica::Ref<DAE::Exp>;
    let mut ty: metamodelica::Ref<DAE::Type> = metamodelica::Ref::new(DAE::Type::T_NORETCALL);
    if (inExpl).is_empty() {
        Error::addSourceMessage(&(Error::EMPTY_ARRAY.clone()), metamodelica::nil(), inInfo)?;
        return Err("fail");
    }
    for mut e in &**inExpl {
        let (__pa0, __pa1, __pa2, __pa3) = ::match_deref::match_deref! { match &(elabGraphicsExp(outCache, inEnv.clone(), e.clone(), inImplicit, inPrefix.clone(), inInfo)?) {
            (__pa0, __pa1, DAE::Properties::PROP { type_: __pa2, constFlag: __pa3 }) => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone()),
            _ => return Err("pattern mismatch"),
        } };
        outCache = metamodelica::Own::own(__pa0);
        exp = metamodelica::Own::own(__pa1);
        ty = metamodelica::Own::own(__pa2);
        c2 = metamodelica::Own::own(__pa3);
        outExpl = metamodelica::cons(exp, outExpl);
        c = Types::constAnd(c, c2);
    }
    outExpl = outExpl.reverse();
    outProperties = DAE::Properties::PROP {
        type_: ty,
        constFlag: c,
    };
    Ok((outCache, outExpl, outProperties))
}

fn elabMatrixComma(
    mut inExpl: &metamodelica::List<metamodelica::Ref<DAE::Exp>>,
    mut inProps: &metamodelica::List<DAE::Properties>,
    mut inHaveReal: bool,
    mut inDims: i32,
    mut inInfo: &SourceInfo,
) -> Result<(
    metamodelica::Ref<DAE::Exp>,
    DAE::Properties,
    metamodelica::Ref<DAE::Dimension>,
    metamodelica::Ref<DAE::Dimension>,
)> {
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    let mut outProperties: DAE::Properties;
    let mut outDim1: metamodelica::Ref<DAE::Dimension>;
    let mut outDim2: metamodelica::Ref<DAE::Dimension>;
    let mut exp: metamodelica::Ref<DAE::Exp>;
    let mut rest_expl: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
    let mut accum_expl: metamodelica::List<metamodelica::Ref<DAE::Exp>> = metamodelica::nil();
    let mut prop: DAE::Properties;
    let mut rest_props: metamodelica::List<DAE::Properties>;
    let mut ty: metamodelica::Ref<DAE::Type>;
    let mut sty: metamodelica::Ref<DAE::Type>;
    let mut dim1: metamodelica::Ref<DAE::Dimension>;
    let mut dim2: metamodelica::Ref<DAE::Dimension>;
    match '__try0: {
        let (__pa1, __pa2) = ::match_deref::match_deref! { match &((*inExpl)) {
            Deref @ metamodelica::ListNode::Cons { head: __pa1, tail: __pa2 } => (__pa1.clone(), __pa2.clone()),
            _ => break '__try0 Err::<_, _>("pattern mismatch"),
        } };
        exp = metamodelica::Own::own(__pa1);
        rest_expl = metamodelica::Own::own(__pa2);
        let (__pa3, __pa4) = ::match_deref::match_deref! { match &((*inProps)) {
            Deref @ metamodelica::ListNode::Cons { head: __pa3, tail: __pa4 } => (__pa3.clone(), __pa4.clone()),
            _ => break '__try0 Err::<_, _>("pattern mismatch"),
        } };
        prop = metamodelica::Own::own(__pa3);
        rest_props = metamodelica::Own::own(__pa4);
        let (__pa5, __pa7, __pa6) = ::match_deref::match_deref! { match &(unwrap_break_err!(promoteExp(exp.clone(), prop.clone(), inDims), '__try0)) {
            (__pa5, __pa7 @ DAE::Properties::PROP { type_: __pa6, .. }) => (__pa5.clone(), __pa7.clone(), __pa6.clone()),
            _ => break '__try0 Err::<_, _>("pattern mismatch"),
        } };
        exp = metamodelica::Own::own(__pa5);
        ty = metamodelica::Own::own(__pa6);
        outProperties = metamodelica::Own::own(__pa7);
        accum_expl = metamodelica::cons(exp.clone(), accum_expl.clone());
        let (__pa8, __pa9) = ::match_deref::match_deref! { match &(TypesDump::getDimensions(&ty)) {
            Deref @ metamodelica::ListNode::Cons { head: __pa8, tail: Deref @ metamodelica::ListNode::Cons { head: __pa9, tail: _ } } => (__pa8.clone(), __pa9.clone()),
            _ => break '__try0 Err::<_, _>("pattern mismatch"),
        } };
        outDim1 = metamodelica::Own::own(__pa8);
        outDim2 = metamodelica::Own::own(__pa9);
        while !((rest_expl).is_empty()) {
            let (__pa11, __pa12) = ::match_deref::match_deref! { match &(rest_expl.clone()) {
                Deref @ metamodelica::ListNode::Cons { head: __pa11, tail: __pa12 } => (__pa11.clone(), __pa12.clone()),
                _ => break '__try0 Err::<_, _>("pattern mismatch"),
            } };
            exp = metamodelica::Own::own(__pa11);
            rest_expl = metamodelica::Own::own(__pa12);
            let (__pa13, __pa14) = ::match_deref::match_deref! { match &(rest_props.clone()) {
                Deref @ metamodelica::ListNode::Cons { head: __pa13, tail: __pa14 } => (__pa13.clone(), __pa14.clone()),
                _ => break '__try0 Err::<_, _>("pattern mismatch"),
            } };
            prop = metamodelica::Own::own(__pa13);
            rest_props = metamodelica::Own::own(__pa14);
            let (__pa15, __pa17, __pa16) = ::match_deref::match_deref! { match &(unwrap_break_err!(promoteExp(exp.clone(), prop.clone(), inDims), '__try0)) {
                (__pa15, __pa17 @ DAE::Properties::PROP { type_: __pa16, .. }) => (__pa15.clone(), __pa17.clone(), __pa16.clone()),
                _ => break '__try0 Err::<_, _>("pattern mismatch"),
            } };
            exp = metamodelica::Own::own(__pa15);
            ty = metamodelica::Own::own(__pa16);
            prop = metamodelica::Own::own(__pa17);
            accum_expl = metamodelica::cons(exp.clone(), accum_expl.clone());
            let (__pa18, __pa19) = ::match_deref::match_deref! { match &(TypesDump::getDimensions(&ty)) {
                Deref @ metamodelica::ListNode::Cons { head: __pa18, tail: Deref @ metamodelica::ListNode::Cons { head: __pa19, tail: _ } } => (__pa18.clone(), __pa19.clone()),
                _ => break '__try0 Err::<_, _>("pattern mismatch"),
            } };
            dim1 = metamodelica::Own::own(__pa18);
            dim2 = metamodelica::Own::own(__pa19);
            if !(unwrap_break_err!(Expression::dimensionsEqual(&dim1, &outDim1), '__try0)) {
                unwrap_break_err!(Error::addSourceMessageAndFail(&(Error::COMMA_OPERATOR_DIFFERENT_SIZES.clone()), list![unwrap_break_err!(ExpressionBasics::printExpStr(unwrap_break_err!((inExpl).head().cloned(), '__try0)), '__try0), unwrap_break_err!(ExpressionBasics::dimensionString(&outDim1), '__try0), unwrap_break_err!(ExpressionBasics::printExpStr(exp.clone()), '__try0), unwrap_break_err!(ExpressionBasics::dimensionString(&dim1), '__try0)], inInfo), '__try0);
                unreachable!("Error.addSourceMessageAndFail always fails — caller-side flow-analysis hint");
            }
            outDim2 = Expression::dimensionsAdd(&dim2, &outDim2);
            outProperties = unwrap_break_err!(Types::matchWithPromote(&prop, &outProperties, inHaveReal), '__try0);
        }
        sty = Expression::liftArrayLeftList(
            unwrap_break_err!(Expression::unliftArrayX(ty.clone(), 2), '__try0),
            list![outDim1.clone(), outDim2.clone()],
        );
        outExp = metamodelica::Ref::new(DAE::Exp::ARRAY {
            ty: sty.clone(),
            scalar: false,
            array: accum_expl.clone().reverse(),
        });
        Ok::<_, &'static str>((
            accum_expl.clone(),
            exp.clone(),
            outDim1.clone(),
            outDim2.clone(),
            outExp.clone(),
            outProperties.clone(),
            prop.clone(),
            rest_expl.clone(),
            rest_props.clone(),
            sty.clone(),
            ty.clone(),
        ))
    } {
        Ok((
            __try0_o0,
            __try0_o1,
            __try0_o2,
            __try0_o3,
            __try0_o4,
            __try0_o5,
            __try0_o6,
            __try0_o7,
            __try0_o8,
            __try0_o9,
            __try0_o10,
        )) => {
            accum_expl = __try0_o0;
            exp = __try0_o1;
            outDim1 = __try0_o2;
            outDim2 = __try0_o3;
            outExp = __try0_o4;
            outProperties = __try0_o5;
            prop = __try0_o6;
            rest_expl = __try0_o7;
            rest_props = __try0_o8;
            sty = __try0_o9;
            ty = __try0_o10;
        }
        Err(__try0_err) => {
            let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else {
                return Err("pattern mismatch");
            };
            Debug::traceln(literal!("- Static.elabMatrixComma failed"))?;
            return Err(__try0_err);
        }
    }
    Ok((outExp, outProperties, outDim1, outDim2))
}

fn elabMatrixCatTwoExp(mut inExp: &metamodelica::Ref<DAE::Exp>) -> Result<metamodelica::Ref<DAE::Exp>> {
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    let mut expl: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
    match '__try0: {
        let __pa1 = ::match_deref::match_deref! { match &((*inExp)) {
            Deref @ DAE::Exp::ARRAY { array: __pa1, .. } => __pa1.clone(),
            _ => break '__try0 Err::<_, _>("pattern mismatch"),
        } };
        expl = metamodelica::Own::own(__pa1);
        expl = unwrap_break_err!(ExpressionSimplify::simplifyList(expl.clone()), '__try0);
        expl = ({
            let mut __acc: metamodelica::List<metamodelica::Ref<DAE::Exp>> = metamodelica::nil();
            for mut e in (expl.clone()).into_iter().cloned() {
                let __x = unwrap_break_err!(Expression::matrixToArray(e.clone()), '__try0);
                __acc = cons(__x, __acc);
            }
            __acc.reverse()
        });
        outExp = unwrap_break_err!(elabMatrixCatTwo(expl.clone()), '__try0);
        Ok::<_, &'static str>((expl.clone(), outExp.clone()))
    } {
        Ok((__try0_o0, __try0_o1)) => {
            expl = __try0_o0;
            outExp = __try0_o1;
        }
        Err(__try0_err) => {
            let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else {
                return Err("pattern mismatch");
            };
            Debug::traceln(literal!("- Static.elabMatrixCatTwoExp failed"))?;
            return Err(__try0_err);
        }
    }
    Ok(outExp)
}

fn elabMatrixCatTwo(
    mut inExpl: metamodelica::List<metamodelica::Ref<DAE::Exp>>,
) -> Result<metamodelica::Ref<DAE::Exp>> {
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    let mut ty: metamodelica::Ref<DAE::Type>;
    match '__try0: {
        outExp = ({
            let mut __acc: Option<_> = None;
            for mut e in (inExpl.clone().reverse()).into_iter().cloned() {
                let __x = e.clone();
                __acc = Some(match __acc {
                    None => __x,
                    Some(__cur) => unwrap_break_err!(elabMatrixCatTwo2(&__x, &__cur), '__try0),
                });
            }
            unwrap_break_err!(__acc.ok_or_else(|| "empty elabMatrixCatTwo2 reduction"), '__try0)
        });
        Ok::<_, &'static str>((outExp.clone(),))
    } {
        Ok((__try0_o0,)) => {
            outExp = __try0_o0;
        }
        Err(_) => {
            ty = Expression::r#typeof((inExpl).head().cloned()?)?;
            outExp = Expression::makePureBuiltinCall(
                literal!("cat"),
                metamodelica::cons(metamodelica::Ref::new(DAE::Exp::ICONST { integer: 2 }), inExpl.clone()),
                ty.clone(),
            );
        }
    }
    Ok(outExp)
}

fn elabMatrixCatTwo2(
    mut inExp1: &metamodelica::Ref<DAE::Exp>,
    mut inExp2: &metamodelica::Ref<DAE::Exp>,
) -> Result<metamodelica::Ref<DAE::Exp>> {
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    let mut expl1: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
    let mut expl2: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
    let mut sc: bool;
    let mut ty: metamodelica::Ref<DAE::Type>;
    let (__pa0, __pa1) = ::match_deref::match_deref! { match &((*inExp1)) {
        Deref @ DAE::Exp::ARRAY { scalar: __pa0, array: __pa1, .. } => (__pa0.clone(), __pa1.clone()),
        _ => return Err("pattern mismatch"),
    } };
    sc = metamodelica::Own::own(__pa0);
    expl1 = metamodelica::Own::own(__pa1);
    let __pa2 = ::match_deref::match_deref! { match &((*inExp2)) {
        Deref @ DAE::Exp::ARRAY { array: __pa2, .. } => __pa2.clone(),
        _ => return Err("pattern mismatch"),
    } };
    expl2 = metamodelica::Own::own(__pa2);
    expl1 = ({
        let mut __acc: metamodelica::List<metamodelica::Ref<DAE::Exp>> = metamodelica::nil();
        let __thr_src0 = expl1;
        let mut __thr_it0 = (&__thr_src0).into_iter();
        let __thr_src1 = expl2;
        let mut __thr_it1 = (&__thr_src1).into_iter();
        loop {
            match (__thr_it0.next(), __thr_it1.next()) {
                (Some(e1), Some(e2)) => {
                    let __x = elabMatrixCatTwo3(&(e1.clone()), &(e2.clone()))?;
                    __acc = cons(__x, __acc);
                }
                (None, None) => break,
                _ => return Err("threaded for: ranges of unequal length"),
            }
        }
        __acc.reverse()
    });
    ty = Expression::r#typeof((expl1).head().cloned()?)?;
    ty = Expression::liftArrayLeft(
        ty,
        metamodelica::Ref::new(DAE::Dimension::DIM_INTEGER {
            integer: ((expl1).len() as i32),
        }),
    );
    outExp = metamodelica::Ref::new(DAE::Exp::ARRAY {
        ty: ty,
        scalar: sc,
        array: expl1,
    });
    Ok(outExp)
}

fn elabMatrixCatTwo3(
    mut inExp1: &metamodelica::Ref<DAE::Exp>,
    mut inExp2: &metamodelica::Ref<DAE::Exp>,
) -> Result<metamodelica::Ref<DAE::Exp>> {
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    let mut ty1: metamodelica::Ref<DAE::Type>;
    let mut ty2: metamodelica::Ref<DAE::Type>;
    let mut sc: bool;
    let mut expl1: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
    let mut expl2: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
    let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &((*inExp1)) {
        Deref @ DAE::Exp::ARRAY { ty: __pa0, scalar: __pa1, array: __pa2 } => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
        _ => return Err("pattern mismatch"),
    } };
    ty1 = metamodelica::Own::own(__pa0);
    sc = metamodelica::Own::own(__pa1);
    expl1 = metamodelica::Own::own(__pa2);
    let (__pa3, __pa4) = ::match_deref::match_deref! { match &((*inExp2)) {
        Deref @ DAE::Exp::ARRAY { ty: __pa3, array: __pa4, .. } => (__pa3.clone(), __pa4.clone()),
        _ => return Err("pattern mismatch"),
    } };
    ty2 = metamodelica::Own::own(__pa3);
    expl2 = metamodelica::Own::own(__pa4);
    expl2 = listAppend(expl1, expl2);
    ty1 = Expression::concatArrayType(&ty1, &ty2)?;
    outExp = metamodelica::Ref::new(DAE::Exp::ARRAY {
        ty: ty1,
        scalar: sc,
        array: expl2,
    });
    Ok(outExp)
}

fn elabMatrixCatOne(
    mut inExpl: metamodelica::List<metamodelica::Ref<DAE::Exp>>,
) -> Result<metamodelica::Ref<DAE::Exp>> {
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    let mut ty: metamodelica::Ref<DAE::Type>;
    match '__try0: {
        outExp = unwrap_break_err!(List::reduce(&inExpl, &move |__a0: metamodelica::Ref<DAE::Exp>, __a1: metamodelica::Ref<DAE::Exp>| elabMatrixCatOne2(&__a0, &__a1)), '__try0);
        Ok::<_, &'static str>((outExp.clone(),))
    } {
        Ok((__try0_o0,)) => {
            outExp = __try0_o0;
        }
        Err(_) => {
            ty = Expression::r#typeof((inExpl).head().cloned()?)?;
            outExp = Expression::makePureBuiltinCall(
                literal!("cat"),
                metamodelica::cons(metamodelica::Ref::new(DAE::Exp::ICONST { integer: 1 }), inExpl.clone()),
                ty.clone(),
            );
        }
    }
    Ok(outExp)
}

fn elabMatrixCatOne2(
    mut inArray1: &metamodelica::Ref<DAE::Exp>,
    mut inArray2: &metamodelica::Ref<DAE::Exp>,
) -> Result<metamodelica::Ref<DAE::Exp>> {
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    let mut ety: metamodelica::Ref<DAE::Type>;
    let mut at: bool;
    let mut dim: metamodelica::Ref<DAE::Dimension>;
    let mut dim1: metamodelica::Ref<DAE::Dimension>;
    let mut dim2: metamodelica::Ref<DAE::Dimension>;
    let mut dim_rest: metamodelica::List<metamodelica::Ref<DAE::Dimension>>;
    let mut expl: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
    let mut expl1: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
    let mut expl2: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
    let (__pa0, __pa1, __pa2, __pa3, __pa4) = ::match_deref::match_deref! { match &((*inArray1)) {
        Deref @ DAE::Exp::ARRAY { ty: Deref @ DAE::Type::T_ARRAY { ty: __pa0, dims: Deref @ metamodelica::ListNode::Cons { head: __pa1, tail: __pa2 } }, scalar: __pa3, array: __pa4 } => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone(), __pa4.clone()),
        _ => return Err("pattern mismatch"),
    } };
    ety = metamodelica::Own::own(__pa0);
    dim1 = metamodelica::Own::own(__pa1);
    dim_rest = metamodelica::Own::own(__pa2);
    at = metamodelica::Own::own(__pa3);
    expl1 = metamodelica::Own::own(__pa4);
    let (__pa6, __pa7) = ::match_deref::match_deref! { match &((*inArray2)) {
        Deref @ DAE::Exp::ARRAY { ty: Deref @ DAE::Type::T_ARRAY { dims: Deref @ metamodelica::ListNode::Cons { head: __pa6, tail: _ }, .. }, array: __pa7, .. } => (__pa6.clone(), __pa7.clone()),
        _ => return Err("pattern mismatch"),
    } };
    dim2 = metamodelica::Own::own(__pa6);
    expl2 = metamodelica::Own::own(__pa7);
    expl = listAppend(expl1, expl2);
    dim = Expression::dimensionsAdd(&dim1, &dim2);
    outExp = metamodelica::Ref::new(DAE::Exp::ARRAY {
        ty: metamodelica::Ref::new(DAE::Type::T_ARRAY {
            ty: ety,
            dims: metamodelica::cons(dim, dim_rest),
        }),
        scalar: at,
        array: expl,
    });
    Ok(outExp)
}

fn promoteExp(
    mut inExp: metamodelica::Ref<DAE::Exp>,
    mut inProperties: DAE::Properties,
    mut inDims: i32,
) -> Result<(metamodelica::Ref<DAE::Exp>, DAE::Properties)> {
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    let mut outProperties: DAE::Properties;
    let mut ty: metamodelica::Ref<DAE::Type>;
    let mut c: DAE::Const;
    match '__try0: {
        let DAE::PROP {
            type_: __pa1,
            constFlag: __pa2,
        } = (inProperties.clone())
        else {
            break '__try0 Err::<_, _>("pattern mismatch");
        };
        ty = metamodelica::Own::own(__pa1);
        c = metamodelica::Own::own(__pa2);
        (outExp, ty) = Expression::promoteExp(inExp.clone(), ty.clone(), inDims);
        outProperties = DAE::Properties::PROP {
            type_: ty.clone(),
            constFlag: c,
        };
        Ok::<_, &'static str>((c.clone(), outExp.clone(), outProperties.clone(), ty.clone()))
    } {
        Ok((__try0_o0, __try0_o1, __try0_o2, __try0_o3)) => {
            c = __try0_o0;
            outExp = __try0_o1;
            outProperties = __try0_o2;
            ty = __try0_o3;
        }
        Err(__try0_err) => {
            let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else {
                return Err("pattern mismatch");
            };
            Debug::traceln(literal!("- Static.promoteExp failed"))?;
            return Err(__try0_err);
        }
    }
    Ok((outExp, outProperties))
}

fn elabMatrixSemi(
    mut inCache: FCore::Cache,
    mut inEnv: &FCore::Graph,
    mut inMatrix: &metamodelica::List<metamodelica::List<metamodelica::Ref<DAE::Exp>>>,
    mut inProperties: &metamodelica::List<metamodelica::List<DAE::Properties>>,
    mut inImpl: bool,
    mut inHaveReal: bool,
    mut inDims: i32,
    mut inDoVectorization: bool,
    mut inPrefix: DAE::Prefix,
    mut inInfo: &SourceInfo,
) -> Result<(
    FCore::Cache,
    metamodelica::Ref<DAE::Exp>,
    DAE::Properties,
    metamodelica::Ref<DAE::Dimension>,
    metamodelica::Ref<DAE::Dimension>,
)> {
    let mut outCache: FCore::Cache = inCache;
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    let mut outProperties: DAE::Properties;
    let mut outDim1: metamodelica::Ref<DAE::Dimension>;
    let mut outDim2: metamodelica::Ref<DAE::Dimension>;
    let mut expl: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
    let mut rest_expl: metamodelica::List<metamodelica::List<metamodelica::Ref<DAE::Exp>>>;
    let mut props: metamodelica::List<DAE::Properties>;
    let mut rest_props: metamodelica::List<metamodelica::List<DAE::Properties>>;
    let mut exp: metamodelica::Ref<DAE::Exp>;
    let mut prop: DAE::Properties;
    let mut dim1: metamodelica::Ref<DAE::Dimension>;
    let mut dim2: metamodelica::Ref<DAE::Dimension>;
    let mut dim1_str: ArcStr;
    let mut dim2_str: ArcStr;
    let mut pre_str: ArcStr;
    let mut el_str: ArcStr;
    let mut ty1_str: ArcStr;
    let mut ty2_str: ArcStr;
    let (__pa0, __pa1) = ::match_deref::match_deref! { match &((*inMatrix)) {
        Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
        _ => return Err("pattern mismatch"),
    } };
    expl = metamodelica::Own::own(__pa0);
    rest_expl = metamodelica::Own::own(__pa1);
    let (__pa2, __pa3) = ::match_deref::match_deref! { match &((*inProperties)) {
        Deref @ metamodelica::ListNode::Cons { head: __pa2, tail: __pa3 } => (__pa2.clone(), __pa3.clone()),
        _ => return Err("pattern mismatch"),
    } };
    props = metamodelica::Own::own(__pa2);
    rest_props = metamodelica::Own::own(__pa3);
    (outExp, outProperties, outDim1, outDim2) = elabMatrixComma(&expl, &props, inHaveReal, inDims, inInfo)?;
    outExp = elabMatrixCatTwoExp(&outExp)?;
    while !((rest_expl).is_empty()) {
        let (__pa4, __pa5) = ::match_deref::match_deref! { match &(rest_expl) {
            Deref @ metamodelica::ListNode::Cons { head: __pa4, tail: __pa5 } => (__pa4.clone(), __pa5.clone()),
            _ => return Err("pattern mismatch"),
        } };
        expl = metamodelica::Own::own(__pa4);
        rest_expl = metamodelica::Own::own(__pa5);
        let (__pa6, __pa7) = ::match_deref::match_deref! { match &(rest_props) {
            Deref @ metamodelica::ListNode::Cons { head: __pa6, tail: __pa7 } => (__pa6.clone(), __pa7.clone()),
            _ => return Err("pattern mismatch"),
        } };
        props = metamodelica::Own::own(__pa6);
        rest_props = metamodelica::Own::own(__pa7);
        (exp, prop, dim1, dim2) = elabMatrixComma(&expl, &props, inHaveReal, inDims, inInfo)?;
        if !(Expression::dimensionsEqual(&dim2, &outDim2)?) {
            dim1_str = ExpressionBasics::dimensionString(&dim1)?;
            dim2_str = ExpressionBasics::dimensionString(&dim2)?;
            pre_str = PrefixUtil::printPrefixStr3(inPrefix.clone())?;
            el_str = List::toStringCustom(
                expl.clone(),
                &ExpressionBasics::printExpStr,
                literal!(""),
                literal!("{"),
                literal!(", "),
                literal!("}"),
                true,
                0,
            )?;
            Error::addSourceMessageAndFail(
                &(Error::MATRIX_EXP_ROW_SIZE.clone()),
                list![pre_str.clone(), el_str.clone(), dim1_str, dim2_str],
                inInfo,
            )?;
            unreachable!("Error.addSourceMessageAndFail always fails — caller-side flow-analysis hint");
        }
        if let Ok(__iflet8) = Types::matchWithPromote(&outProperties, &prop, inHaveReal) {
            outProperties = __iflet8;
        } else {
            ty1_str = TypesDump::unparsePropTypeNoAttr(&outProperties)?;
            ty2_str = TypesDump::unparsePropTypeNoAttr(&prop)?;
            Types::typeErrorSanityCheck(ty1_str.clone(), &ty2_str, inInfo)?;
            pre_str = PrefixUtil::printPrefixStr3(inPrefix.clone())?;
            el_str = List::toStringCustom(
                expl.clone(),
                &ExpressionBasics::printExpStr,
                literal!(""),
                literal!("{"),
                literal!(", "),
                literal!("}"),
                true,
                0,
            )?;
            Error::addSourceMessageAndFail(
                &(Error::TYPE_MISMATCH_MATRIX_EXP.clone()),
                list![pre_str.clone(), el_str.clone(), ty1_str.clone(), ty2_str.clone()],
                inInfo,
            )?;
            unreachable!("Error.addSourceMessageAndFail always fails — caller-side flow-analysis hint");
        }
        exp = elabMatrixCatTwoExp(&exp)?;
        outExp = elabMatrixCatOne(list![outExp, exp])?;
        outDim1 = Expression::dimensionsAdd(&dim1, &outDim1);
    }
    Ok((outCache, outExp, outProperties, outDim1, outDim2))
}

fn verifyBuiltInHandlerType(
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
    mut inExpl: &metamodelica::List<metamodelica::Ref<Absyn::Exp>>,
    mut inImplicit: bool,
    mut inTypeChecker: &dyn ::std::ops::Fn(metamodelica::Ref<DAE::Type>) -> Result<bool>,
    mut inFnName: ArcStr,
    mut inPrefix: DAE::Prefix,
    mut inInfo: SourceInfo,
) -> Result<(FCore::Cache, metamodelica::Ref<DAE::Exp>, DAE::Properties)> {
    pub type extraFunc = std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Type>) -> Result<bool> + 'static>;

    let mut outCache: FCore::Cache;
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    let mut outProperties: DAE::Properties;
    let mut e: metamodelica::Ref<Absyn::Exp>;
    let mut ty: metamodelica::Ref<DAE::Type>;
    let __pa0 = ::match_deref::match_deref! { match &((*inExpl)) {
        Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: Deref @ metamodelica::ListNode::Nil } => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    e = metamodelica::Own::own(__pa0);
    (outCache, _, outProperties) = elabExpInExpression(
        inCache,
        inEnv.clone(),
        e.clone(),
        inImplicit,
        true,
        inPrefix.clone(),
        inInfo.clone(),
    )?;
    ty = Types::getPropType(&outProperties);
    ty = Types::arrayElementType(&ty);
    let true = (inTypeChecker(ty)?) else {
        return Err("pattern mismatch");
    };
    let (__pa2, __pa3, __pa4) = ::match_deref::match_deref! { match &(elabCallArgs(outCache, inEnv, metamodelica::Ref::new(Absyn::Path::FULLYQUALIFIED { path: metamodelica::Ref::new(Absyn::Path::IDENT { name: inFnName }) }), list![e], metamodelica::nil(), &(metamodelica::nil()), inImplicit, inPrefix, inInfo)?) {
        (__pa2, __pa3, __pa4 @ DAE::Properties::PROP { .. }) => (__pa2.clone(), __pa3.clone(), __pa4.clone()),
        _ => return Err("pattern mismatch"),
    } };
    outCache = metamodelica::Own::own(__pa2);
    outExp = metamodelica::Own::own(__pa3);
    outProperties = metamodelica::Own::own(__pa4);
    Ok((outCache, outExp, outProperties))
}

fn elabBuiltinCardinality(
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
    mut inPosArgs: &metamodelica::List<metamodelica::Ref<Absyn::Exp>>,
    mut inNamedArgs: &metamodelica::List<metamodelica::Ref<Absyn::NamedArg>>,
    mut inImplicit: bool,
    mut inPrefix: DAE::Prefix,
    mut inInfo: SourceInfo,
) -> Result<(FCore::Cache, metamodelica::Ref<DAE::Exp>, DAE::Properties)> {
    let mut outCache: FCore::Cache;
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    let mut outProperties: DAE::Properties;
    let mut ty: metamodelica::Ref<DAE::Type>;
    let mut e: metamodelica::Ref<Absyn::Exp>;
    checkBuiltinCallArgs(inPosArgs, inNamedArgs, 1, literal!("cardinality"), &inInfo)?;
    let __pa0 = ::match_deref::match_deref! { match &((*inPosArgs)) {
        Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: Deref @ metamodelica::ListNode::Nil } => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    e = metamodelica::Own::own(__pa0);
    (outCache, outExp, outProperties) = elabExpInExpression(inCache, inEnv, e, inImplicit, true, inPrefix, inInfo)?;
    let DAE::PROP { type_: __pa2, .. } = (outProperties) else {
        return Err("pattern mismatch");
    };
    ty = metamodelica::Own::own(__pa2);
    ty = Types::liftArrayListDims(DAE::T_INTEGER_DEFAULT().clone(), TypesDump::getDimensions(&ty));
    outExp = Expression::makePureBuiltinCall(literal!("cardinality"), list![outExp], ty.clone());
    outProperties = DAE::Properties::PROP {
        type_: ty,
        constFlag: openmodelica_frontend_types::DAE::Const::C_CONST,
    };
    Ok((outCache, outExp, outProperties))
}

fn elabBuiltinSmooth(
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
    mut inPosArgs: metamodelica::List<metamodelica::Ref<Absyn::Exp>>,
    mut inNamedArgs: metamodelica::List<metamodelica::Ref<Absyn::NamedArg>>,
    mut inImplicit: bool,
    mut inPrefix: DAE::Prefix,
    mut inInfo: SourceInfo,
) -> Result<(FCore::Cache, metamodelica::Ref<DAE::Exp>, DAE::Properties)> {
    let mut outCache: FCore::Cache;
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    let mut outProperties: DAE::Properties;
    let mut msg_str: ArcStr;
    let mut p: metamodelica::Ref<Absyn::Exp>;
    let mut expr: metamodelica::Ref<Absyn::Exp>;
    let mut dp: metamodelica::Ref<DAE::Exp>;
    let mut dexpr: metamodelica::Ref<DAE::Exp>;
    let mut ty: metamodelica::Ref<DAE::Type>;
    let mut c: DAE::Const;
    if ((inPosArgs).len() as i32) != 2 || !((inNamedArgs).is_empty()) {
        msg_str = literal!(", expected smooth(p, expr)");
        printBuiltinFnArgError(
            &(literal!("smooth")),
            &msg_str,
            inPosArgs.clone(),
            inNamedArgs.clone(),
            inPrefix.clone(),
            &inInfo,
        )?;
    }
    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(inPosArgs.clone()) {
        Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: Deref @ metamodelica::ListNode::Cons { head: __pa1, tail: Deref @ metamodelica::ListNode::Nil } } => (__pa0.clone(), __pa1.clone()),
        _ => return Err("pattern mismatch"),
    } };
    p = metamodelica::Own::own(__pa0);
    expr = metamodelica::Own::own(__pa1);
    let (__pa3, __pa4, __pa5, __pa6) = ::match_deref::match_deref! { match &(elabExpInExpression(inCache, inEnv.clone(), p, inImplicit, true, inPrefix.clone(), inInfo.clone())?) {
        (__pa3, __pa4, DAE::Properties::PROP { type_: __pa5, constFlag: __pa6 }) => (__pa3.clone(), __pa4.clone(), __pa5.clone(), __pa6.clone()),
        _ => return Err("pattern mismatch"),
    } };
    outCache = metamodelica::Own::own(__pa3);
    dp = metamodelica::Own::own(__pa4);
    ty = metamodelica::Own::own(__pa5);
    c = metamodelica::Own::own(__pa6);
    if !(Types::isParameterOrConstant(c)) || !(Types::isInteger(&ty)) {
        msg_str = literal!(", first argument must be a constant or parameter expression of type Integer");
        printBuiltinFnArgError(
            &(literal!("smooth")),
            &msg_str,
            inPosArgs.clone(),
            inNamedArgs.clone(),
            inPrefix.clone(),
            &inInfo,
        )?;
    }
    let (__pa7, __pa8, __pa11, __pa9, __pa10) = ::match_deref::match_deref! { match &(elabExpInExpression(outCache, inEnv, expr, inImplicit, true, inPrefix.clone(), inInfo.clone())?) {
        (__pa7, __pa8, __pa11 @ DAE::Properties::PROP { type_: __pa9, constFlag: __pa10 }) => (__pa7.clone(), __pa8.clone(), __pa11.clone(), __pa9.clone(), __pa10.clone()),
        _ => return Err("pattern mismatch"),
    } };
    outCache = metamodelica::Own::own(__pa7);
    dexpr = metamodelica::Own::own(__pa8);
    ty = metamodelica::Own::own(__pa9);
    c = metamodelica::Own::own(__pa10);
    outProperties = metamodelica::Own::own(__pa11);
    if !(Types::isReal(&ty) || Types::isRecordWithOnlyReals(&ty)?) {
        msg_str = literal!(", second argument must be a Real, array of Reals or record only containing Reals");
        printBuiltinFnArgError(
            &(literal!("smooth")),
            &msg_str,
            inPosArgs,
            inNamedArgs,
            inPrefix,
            &inInfo,
        )?;
    }
    ty = Types::simplifyType(ty)?;
    outExp = Expression::makePureBuiltinCall(literal!("smooth"), list![dp, dexpr], ty);
    Ok((outCache, outExp, outProperties))
}

fn printBuiltinFnArgError(
    mut inFnName: &ArcStr,
    mut inMsg: &ArcStr,
    mut inPosArgs: metamodelica::List<metamodelica::Ref<Absyn::Exp>>,
    mut inNamedArgs: metamodelica::List<metamodelica::Ref<Absyn::NamedArg>>,
    mut inPrefix: DAE::Prefix,
    mut inInfo: &SourceInfo,
) -> Result<()> {
    let mut args_str: ArcStr;
    let mut pre_str: ArcStr;
    let mut msg_str: ArcStr;
    let mut pos_args: metamodelica::List<ArcStr>;
    let mut named_args: metamodelica::List<ArcStr>;
    pos_args = ({
        let mut __acc: metamodelica::List<ArcStr> = metamodelica::nil();
        for mut arg in (inPosArgs).into_iter().cloned() {
            let __x = Dump::printExpStr(arg.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    named_args = ({
        let mut __acc: metamodelica::List<ArcStr> = metamodelica::nil();
        for mut arg in (inNamedArgs).into_iter().cloned() {
            let __x = Dump::printNamedArgStr(&(arg.clone()))?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    args_str = stringDelimitList(listAppend(pos_args, named_args), literal!(", "));
    pre_str = PrefixUtil::printPrefixStr3(inPrefix)?;
    msg_str = {
        let mut __mm_s = String::new();
        __mm_s.push_str(&*inFnName);
        __mm_s.push_str(&*literal!("("));
        __mm_s.push_str(&*args_str);
        __mm_s.push_str(&*literal!(")"));
        __mm_s.push_str(&*inMsg);
        ArcStr::from(__mm_s)
    };
    Error::addSourceMessageAndFail(
        &(Error::WRONG_TYPE_OR_NO_OF_ARGS.clone()),
        list![msg_str, pre_str],
        inInfo,
    )?;
    unreachable!("Error.addSourceMessageAndFail always fails — caller-side flow-analysis hint");
    Ok(())
}

fn elabBuiltinSize(
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
    mut inAbsynExpLst: &metamodelica::List<metamodelica::Ref<Absyn::Exp>>,
    mut inNamedArg: &metamodelica::List<metamodelica::Ref<Absyn::NamedArg>>,
    mut inBoolean: bool,
    mut inPrefix: DAE::Prefix,
    mut info: SourceInfo,
) -> Result<(FCore::Cache, metamodelica::Ref<DAE::Exp>, DAE::Properties)> {
    let mut outCache: FCore::Cache;
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    let mut outProperties: DAE::Properties;
    (outCache, outExp, outProperties) = (::match_deref::match_deref! { match inAbsynExpLst {
        Deref @ metamodelica::ListNode::Cons { head: arraycr, tail: Deref @ metamodelica::ListNode::Cons { head: dim, tail: Deref @ metamodelica::ListNode::Nil } } => {
            let mut cache = inCache;
            let mut env = inEnv;
            let mut r#impl = inBoolean;
            let mut pre = inPrefix;
            let mut dimp: metamodelica::Ref<DAE::Exp>;
            let mut arraycrefe: metamodelica::Ref<DAE::Exp>;
            let mut exp: metamodelica::Ref<DAE::Exp>;
            let mut prop: DAE::Properties;
            let mut ety: metamodelica::Ref<DAE::Type>;
            let mut dims: metamodelica::List<metamodelica::Ref<DAE::Dimension>>;
            let mut dims1: metamodelica::List<metamodelica::Ref<DAE::Dimension>>;
            let mut dims2: metamodelica::List<metamodelica::Ref<DAE::Dimension>>;
            (cache, dimp, _) = elabExpInExpression(cache, env.clone(), dim.clone(), r#impl, true, pre.clone(), info.clone())?;
            (cache, arraycrefe, prop) = elabExpInExpression(cache, env.clone(), arraycr.clone(), r#impl, false, pre, info.clone())?;
            ety = Expression::r#typeof(arraycrefe.clone())?;
            dims1 = Expression::arrayDimension(&ety);
            (_, dims2) = TypesDump::flattenArrayType(&(Types::getPropType(&prop)));
            dims = if (((dims1).len() as i32) >= ((dims2).len() as i32)) {dims1} else {dims2};
            let (__pa0, __pa1) = ::match_deref::match_deref! { match &(elabBuiltinSizeIndex(arraycrefe, &prop, &ety, dimp, &dims, &env, &info)) {
                (Some(__pa0), Some(__pa1)) => (__pa0.clone(), __pa1.clone()),
                _ => return Err("pattern mismatch"),
            } };
            exp = metamodelica::Own::own(__pa0);
            prop = metamodelica::Own::own(__pa1);
            (cache, exp, prop)
        },
        Deref @ metamodelica::ListNode::Cons { head: arraycr, tail: Deref @ metamodelica::ListNode::Nil } => {
            let mut cache = inCache;
            let mut env = inEnv;
            let mut r#impl = inBoolean;
            let mut pre = inPrefix;
            let mut arraycrefe: metamodelica::Ref<DAE::Exp>;
            let mut exp: metamodelica::Ref<DAE::Exp>;
            let mut arrtp: metamodelica::Ref<DAE::Type>;
            let mut prop: DAE::Properties;
            let mut ety: metamodelica::Ref<DAE::Type>;
            let mut dims: metamodelica::List<metamodelica::Ref<DAE::Dimension>>;
            let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(elabExpInExpression(cache, env, arraycr.clone(), r#impl, false, pre, info.clone())?) {
                (__pa0, __pa1, DAE::Properties::PROP { type_: __pa2, constFlag: _ }) => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
                _ => return Err("pattern mismatch"),
            } };
            cache = metamodelica::Own::own(__pa0);
            arraycrefe = metamodelica::Own::own(__pa1);
            arrtp = metamodelica::Own::own(__pa2);
            ety = Expression::r#typeof(arraycrefe.clone())?;
            dims = Expression::arrayDimension(&ety);
            (exp, prop) = elabBuiltinSizeNoIndex(arraycrefe, &ety, dims, arrtp, &info)?;
            (cache, exp, prop)
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok((outCache, outExp, outProperties))
}

fn elabBuiltinSizeNoIndex(
    mut inArrayExp: metamodelica::Ref<DAE::Exp>,
    mut inArrayExpType: &metamodelica::Ref<DAE::Type>,
    mut inDimensions: metamodelica::List<metamodelica::Ref<DAE::Dimension>>,
    mut inArrayType: metamodelica::Ref<DAE::Type>,
    mut inInfo: &SourceInfo,
) -> Result<(metamodelica::Ref<DAE::Exp>, DAE::Properties)> {
    let mut outSizeExp: metamodelica::Ref<DAE::Exp>;
    let mut outProperties: DAE::Properties;
    (outSizeExp, outProperties) = 'mc: {
        let __mc_input = &*inDimensions;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Nil => {
                    let mut exp_str: ArcStr;
                    let mut size_str: ArcStr;
                    let false = (Types::isUnknownType(inArrayExpType)) else { return Err("pattern mismatch") };
                    exp_str = ExpressionBasics::printExpStr(inArrayExp.clone())?;
                    size_str = { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("size(")); __mm_s.push_str(&*exp_str); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) };
                    Error::addSourceMessage(&(Error::INVALID_ARGUMENT_TYPE_FIRST_ARRAY.clone()), list![size_str.clone()], inInfo)?;
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: _, tail: _ } => {
                    let mut dim_expl: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                    let mut dim_int: i32;
                    let mut exp: metamodelica::Ref<DAE::Exp>;
                    let mut prop: DAE::Properties;
                    let mut ty: metamodelica::Ref<DAE::Type>;
                    dim_expl = List::map(inDimensions.clone(), &move |__a0: metamodelica::Ref<DAE::Dimension>| Expression::dimensionSizeExp(&__a0))?;
                    dim_int = ((dim_expl).len() as i32);
                    ty = metamodelica::Ref::new(DAE::Type::T_ARRAY { ty: DAE::T_INTEGER_DEFAULT().clone(), dims: list![metamodelica::Ref::new(DAE::Dimension::DIM_INTEGER { integer: dim_int })] });
                    exp = metamodelica::Ref::new(DAE::Exp::ARRAY { ty: ty.clone(), scalar: true, array: dim_expl.clone() });
                    prop = DAE::Properties::PROP { type_: ty.clone(), constFlag: openmodelica_frontend_types::DAE::Const::C_CONST };
                    Ok((exp.clone(), prop.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: _, tail: _ } => {
                    let mut exp: metamodelica::Ref<DAE::Exp>;
                    let mut prop: DAE::Properties;
                    let mut b: bool;
                    let mut cnst: DAE::Const;
                    let mut ty: metamodelica::Ref<DAE::Type>;
                    b = Types::dimensionsKnown(inArrayType.clone());
                    cnst = Types::boolConstSize(b);
                    exp = metamodelica::Ref::new(DAE::Exp::SIZE { exp: inArrayExp.clone(), sz: None });
                    ty = metamodelica::Ref::new(DAE::Type::T_ARRAY { ty: DAE::T_INTEGER_DEFAULT().clone(), dims: list![openmodelica_frontend_types::DAE::Dimension::interned_DIM_UNKNOWN()] });
                    prop = DAE::Properties::PROP { type_: ty.clone(), constFlag: cnst };
                    Ok((exp.clone(), prop.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((outSizeExp, outProperties))
}

fn elabBuiltinSizeIndex(
    mut inArrayExp: metamodelica::Ref<DAE::Exp>,
    mut inArrayProp: &DAE::Properties,
    mut inArrayType: &metamodelica::Ref<DAE::Type>,
    mut inIndexExp: metamodelica::Ref<DAE::Exp>,
    mut inDimensions: &metamodelica::List<metamodelica::Ref<DAE::Dimension>>,
    mut inEnv: &FCore::Graph,
    mut inInfo: &SourceInfo,
) -> (Option<metamodelica::Ref<DAE::Exp>>, Option<DAE::Properties>) {
    let mut outSizeExp: Option<metamodelica::Ref<DAE::Exp>>;
    let mut outProperties: Option<DAE::Properties>;
    (outSizeExp, outProperties) = 'mc: {
        let __mc_input = &**inDimensions;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Nil => {
                    let mut exp_str: ArcStr;
                    let mut index_str: ArcStr;
                    let mut size_str: ArcStr;
                    let false = (Types::isUnknownType(inArrayType)) else { return Err("pattern mismatch") };
                    exp_str = ExpressionBasics::printExpStr(inArrayExp.clone())?;
                    index_str = ExpressionBasics::printExpStr(inIndexExp.clone())?;
                    size_str = { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("size(")); __mm_s.push_str(&*exp_str); __mm_s.push_str(&*literal!(", ")); __mm_s.push_str(&*index_str); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) };
                    Error::addSourceMessage(&(Error::INVALID_ARGUMENT_TYPE_FIRST_ARRAY.clone()), list![size_str.clone()], inInfo)?;
                    Ok((None, None))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut dim_int: i32;
                    let mut dim_count: i32;
                    let mut exp: metamodelica::Ref<DAE::Exp>;
                    let mut dim: metamodelica::Ref<DAE::Dimension>;
                    let mut prop: DAE::Properties;
                    dim_int = Expression::expInt(&inIndexExp)?;
                    dim_count = ((inDimensions).len() as i32);
                    let true = (dim_int > 0 && dim_int <= dim_count) else { return Err("pattern mismatch") };
                    dim = (inDimensions).get(dim_int)?;
                    exp = Expression::dimensionSizeConstantExp(&dim)?;
                    prop = DAE::Properties::PROP { type_: DAE::T_INTEGER_DEFAULT().clone(), constFlag: openmodelica_frontend_types::DAE::Const::C_CONST };
                    Ok((Some(exp.clone()), Some(prop.clone())))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut dim_int: i32;
                    let mut dim_count: i32;
                    let mut exp_str: ArcStr;
                    let mut index_str: ArcStr;
                    let mut dim_str: ArcStr;
                    let false = (Types::isUnknownType(inArrayType)) else { return Err("pattern mismatch") };
                    dim_int = Expression::expInt(&inIndexExp)?;
                    dim_count = ((inDimensions).len() as i32);
                    let true = (dim_int <= 0 || dim_int > dim_count) else { return Err("pattern mismatch") };
                    index_str = intString(dim_int);
                    exp_str = ExpressionBasics::printExpStr(inArrayExp.clone())?;
                    dim_str = intString(dim_count);
                    Error::addSourceMessage(&(Error::INVALID_SIZE_INDEX.clone()), list![index_str.clone(), exp_str.clone(), dim_str.clone()], inInfo)?;
                    Ok((None, None))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut exp: metamodelica::Ref<DAE::Exp>;
                    let mut prop: DAE::Properties;
                    let mut cnst: DAE::Const;
                    exp = metamodelica::Ref::new(DAE::Exp::SIZE { exp: inArrayExp.clone(), sz: Some(inIndexExp.clone()) });
                    cnst = openmodelica_frontend_types::DAE::Const::C_PARAM;
                    cnst = if (FGraph::inFunctionScope(inEnv)) {openmodelica_frontend_types::DAE::Const::C_VAR} else {cnst};
                    prop = DAE::Properties::PROP { type_: DAE::T_INTEGER_DEFAULT().clone(), constFlag: cnst };
                    Ok((Some(exp.clone()), Some(prop.clone())))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    (outSizeExp, outProperties)
}

fn elabBuiltinNDims(
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
    mut inAbsynExpLst: metamodelica::List<metamodelica::Ref<Absyn::Exp>>,
    mut inNamedArg: &metamodelica::List<metamodelica::Ref<Absyn::NamedArg>>,
    mut inBoolean: bool,
    mut inPrefix: DAE::Prefix,
    mut info: SourceInfo,
) -> Result<(FCore::Cache, metamodelica::Ref<DAE::Exp>, DAE::Properties)> {
    let mut outCache: FCore::Cache;
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    let mut outProperties: DAE::Properties;
    (outCache, outExp, outProperties) = 'mc: {
        let __mc_input = (inCache, inEnv, inAbsynExpLst, inBoolean, inPrefix);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, Deref @ metamodelica::ListNode::Cons { head: arraycr, tail: Deref @ metamodelica::ListNode::Nil }, r#impl, pre) => {
                    let mut exp: metamodelica::Ref<DAE::Exp>;
                    let mut arrtp: metamodelica::Ref<DAE::Type>;
                    let mut nd: i32;
                    let mut cache = (*cache).clone();
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(elabExpInExpression(cache.clone(), env.clone(), arraycr.clone(), r#impl.clone(), true, pre.clone(), info.clone())?) {
                        (__pa0, _, DAE::Properties::PROP { type_: __pa1, constFlag: _ }) => (__pa0.clone(), __pa1.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    cache = metamodelica::Own::own(__pa0);
                    arrtp = metamodelica::Own::own(__pa1);
                    nd = Types::numberOfDimensions(&arrtp);
                    exp = metamodelica::Ref::new(DAE::Exp::ICONST { integer: nd });
                    Ok((cache.clone(), exp.clone(), DAE::Properties::PROP { type_: DAE::T_INTEGER_DEFAULT().clone(), constFlag: openmodelica_frontend_types::DAE::Const::C_CONST }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, expl, _, pre) => {
                    let mut sp: ArcStr;
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { return Err("pattern mismatch") };
                    sp = PrefixUtil::printPrefixStr3(pre.clone())?;
                    Debug::traceln({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("- Static.elabBuiltinNdims failed for: ndims(")); __mm_s.push_str(&*Dump::printExpLstStr(expl.clone())?); __mm_s.push_str(&*literal!(" in component: ")); __mm_s.push_str(&*sp); ArcStr::from(__mm_s) })?;
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((outCache, outExp, outProperties))
}

fn elabBuiltinFill(
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
    mut inAbsynExpLst: metamodelica::List<metamodelica::Ref<Absyn::Exp>>,
    mut inNamedArg: &metamodelica::List<metamodelica::Ref<Absyn::NamedArg>>,
    mut inBoolean: bool,
    mut inPrefix: DAE::Prefix,
    mut info: SourceInfo,
) -> Result<(FCore::Cache, metamodelica::Ref<DAE::Exp>, DAE::Properties)> {
    let mut outCache: FCore::Cache;
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    let mut outProperties: DAE::Properties;
    (outCache, outExp, outProperties) = 'mc: {
        let __mc_input = (inCache, inEnv, inAbsynExpLst, inBoolean, inPrefix.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, Deref @ metamodelica::ListNode::Cons { head: s, tail: dims }, r#impl, pre) => {
                    let mut s_1: metamodelica::Ref<DAE::Exp>;
                    let mut exp: metamodelica::Ref<DAE::Exp>;
                    let mut prop: DAE::Properties;
                    let mut dims_1: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                    let mut dimprops: metamodelica::List<DAE::Properties>;
                    let mut sty: metamodelica::Ref<DAE::Type>;
                    let mut dimvals: metamodelica::List<metamodelica::Ref<Values::Value>>;
                    let mut c1: DAE::Const;
                    let mut cache = (*cache).clone();
                    (cache, s_1, prop) = elabExpInExpression(cache.clone(), env.clone(), s.clone(), r#impl.clone(), true, pre.clone(), info.clone())?;
                    (cache, dims_1, dimprops) = elabExpList(cache.clone(), env.clone(), metamodelica::AsArg::as_arg(&dims), r#impl.clone(), true, pre.clone(), info.clone(), DAE::T_UNKNOWN_DEFAULT().clone())?;
                    (dims_1, _) = Types::matchTypes(dims_1.clone(), List::map(dimprops.clone(), &move |__a0: DAE::Properties| -> metamodelica::Result<_> { ::std::result::Result::Ok(Types::getPropType(&__a0)) })?, &(DAE::T_INTEGER_DEFAULT().clone()), false)?;
                    c1 = Types::propertiesListToConst(&dimprops)?;
                    if '__try0: {
                        let DAE::C_VAR { .. } = (c1) else { break '__try0 Err::<_, _>("pattern mismatch") };
                        Ok::<(), &'static str>(())
                    }.is_ok() { return Err("failure(): body succeeded") }
                    c1 = Types::constAnd(c1, Types::propAllConst(prop.clone())?);
                    sty = Types::getPropType(&prop);
                    (cache, dimvals) = Ceval::cevalList(cache.clone(), env.clone(), dims_1.clone(), r#impl.clone(), openmodelica_ast::Absyn::Msg::NO_MSG, 0)?;
                    (cache, exp, prop) = ExpressionSimplify::elabBuiltinFill2(cache.clone(), s_1.clone(), sty.clone(), &dimvals, c1, metamodelica::AsArg::as_arg(&dims), &info)?;
                    Ok((cache.clone(), exp.clone(), prop.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, Deref @ metamodelica::ListNode::Cons { head: s, tail: dims }, r#impl, pre) => {
                    let mut s_1: metamodelica::Ref<DAE::Exp>;
                    let mut exp: metamodelica::Ref<DAE::Exp>;
                    let mut prop: DAE::Properties;
                    let mut dims_1: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                    let mut dimprops: metamodelica::List<DAE::Properties>;
                    let mut sty: metamodelica::Ref<DAE::Type>;
                    let mut c1: DAE::Const;
                    let mut exp_type: metamodelica::Ref<DAE::Type>;
                    let mut cache = (*cache).clone();
                    c1 = unevaluatedFunctionVariability(metamodelica::AsArg::as_arg(&env))?;
                    (cache, s_1, prop) = elabExpInExpression(cache.clone(), env.clone(), s.clone(), r#impl.clone(), true, pre.clone(), info.clone())?;
                    (cache, dims_1, dimprops) = elabExpList(cache.clone(), env.clone(), metamodelica::AsArg::as_arg(&dims), r#impl.clone(), true, pre.clone(), info.clone(), DAE::T_UNKNOWN_DEFAULT().clone())?;
                    (dims_1, _) = Types::matchTypes(dims_1.clone(), List::map(dimprops.clone(), &move |__a0: DAE::Properties| -> metamodelica::Result<_> { ::std::result::Result::Ok(Types::getPropType(&__a0)) })?, &(DAE::T_INTEGER_DEFAULT().clone()), false)?;
                    sty = Types::getPropType(&prop);
                    sty = Types::liftTypeWithDimExps(sty.clone(), &dims_1)?;
                    exp_type = Types::simplifyType(sty.clone())?;
                    prop = DAE::Properties::PROP { type_: sty.clone(), constFlag: c1 };
                    exp = Expression::makePureBuiltinCall(literal!("fill"), metamodelica::cons(s_1.clone(), dims_1.clone()), exp_type.clone());
                    Ok((cache.clone(), exp.clone(), prop.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, Deref @ metamodelica::ListNode::Cons { head: s, tail: dims }, r#impl, pre) => {
                    let mut s_1: metamodelica::Ref<DAE::Exp>;
                    let mut exp: metamodelica::Ref<DAE::Exp>;
                    let mut prop: DAE::Properties;
                    let mut dims_1: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                    let mut sty: metamodelica::Ref<DAE::Type>;
                    let mut c1: DAE::Const;
                    let mut exp_type: metamodelica::Ref<DAE::Type>;
                    let mut cache = (*cache).clone();
                    let false = (Config::splitArrays()?) else { return Err("pattern mismatch") };
                    let (__pa0, __pa1, __pa2, __pa3) = ::match_deref::match_deref! { match &(elabExpInExpression(cache.clone(), env.clone(), s.clone(), r#impl.clone(), true, pre.clone(), info.clone())?) {
                        (__pa0, __pa1, DAE::Properties::PROP { type_: __pa2, constFlag: __pa3 }) => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    cache = metamodelica::Own::own(__pa0);
                    s_1 = metamodelica::Own::own(__pa1);
                    sty = metamodelica::Own::own(__pa2);
                    c1 = metamodelica::Own::own(__pa3);
                    (cache, dims_1, _) = elabExpList(cache.clone(), env.clone(), metamodelica::AsArg::as_arg(&dims), r#impl.clone(), true, pre.clone(), info.clone(), DAE::T_UNKNOWN_DEFAULT().clone())?;
                    sty = Types::liftTypeWithDimExps(sty.clone(), &dims_1)?;
                    exp_type = Types::simplifyType(sty.clone())?;
                    c1 = Types::constAnd(c1, openmodelica_frontend_types::DAE::Const::C_PARAM);
                    prop = DAE::Properties::PROP { type_: sty.clone(), constFlag: c1 };
                    exp = Expression::makePureBuiltinCall(literal!("fill"), metamodelica::cons(s_1.clone(), dims_1.clone()), exp_type.clone());
                    Ok((cache.clone(), exp.clone(), prop.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, env, dims, _, _) => {
                    let mut r#str: ArcStr;
                    r#str = { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Static.elabBuiltinFill failed in component")); __mm_s.push_str(&*PrefixUtil::printPrefixStr3(inPrefix.clone())?); __mm_s.push_str(&*literal!(" and scope: ")); __mm_s.push_str(&*FGraph::printGraphPathStr(metamodelica::AsArg::as_arg(&env))); __mm_s.push_str(&*literal!(" for expression: fill(")); __mm_s.push_str(&*Dump::printExpLstStr(dims.clone())?); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) };
                    Error::addSourceMessage(&(Error::INTERNAL_ERROR.clone()), list![r#str.clone()], &info)?;
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, dims, r#impl, pre) => {
                    let mut implstr: ArcStr;
                    let mut expstr: ArcStr;
                    let mut r#str: ArcStr;
                    let mut sp: ArcStr;
                    let mut expstrs: metamodelica::List<ArcStr>;
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { return Err("pattern mismatch") };
                    Debug::trace(literal!("- Static.elabBuiltinFill: Couldn't elaborate fill(): "))?;
                    implstr = boolString(r#impl.clone());
                    expstrs = List::map(dims.clone(), &Dump::printExpStr)?;
                    expstr = stringDelimitList(expstrs.clone(), literal!(", "));
                    sp = PrefixUtil::printPrefixStr3(pre.clone())?;
                    r#str = stringAppendList(list![expstr.clone(), literal!(" impl="), implstr.clone(), literal!(", in component: "), sp.clone()]);
                    Debug::traceln(r#str.clone())?;
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((outCache, outExp, outProperties))
}

fn elabBuiltinSymmetric(
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
    mut inAbsynExpLst: &metamodelica::List<metamodelica::Ref<Absyn::Exp>>,
    mut inNamedArg: &metamodelica::List<metamodelica::Ref<Absyn::NamedArg>>,
    mut inBoolean: bool,
    mut inPrefix: DAE::Prefix,
    mut info: SourceInfo,
) -> Result<(FCore::Cache, metamodelica::Ref<DAE::Exp>, DAE::Properties)> {
    let mut outCache: FCore::Cache;
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    let mut outProperties: DAE::Properties;
    (outCache, outExp, outProperties) = (::match_deref::match_deref! { match inAbsynExpLst {
        Deref @ metamodelica::ListNode::Cons { head: matexp, tail: Deref @ metamodelica::ListNode::Nil } => {
            let mut cache = inCache;
            let mut env = inEnv;
            let mut r#impl = inBoolean;
            let mut pre = inPrefix;
            let mut tp: metamodelica::Ref<DAE::Type>;
            let mut d1: metamodelica::Ref<DAE::Dimension>;
            let mut d2: metamodelica::Ref<DAE::Dimension>;
            let mut eltp: metamodelica::Ref<DAE::Type>;
            let mut newtp: metamodelica::Ref<DAE::Type>;
            let mut prop: DAE::Properties;
            let mut c: DAE::Const;
            let mut exp_1: metamodelica::Ref<DAE::Exp>;
            let mut exp: metamodelica::Ref<DAE::Exp>;
            let (__pa0, __pa1, __pa2, __pa3, __pa4, __pa5) = ::match_deref::match_deref! { match &(elabExpInExpression(cache, env, matexp.clone(), r#impl, true, pre, info)?) {
                (__pa0, __pa1, DAE::Properties::PROP { type_: Deref @ DAE::Type::T_ARRAY { dims: Deref @ metamodelica::ListNode::Cons { head: __pa2, tail: Deref @ metamodelica::ListNode::Nil }, ty: Deref @ DAE::Type::T_ARRAY { dims: Deref @ metamodelica::ListNode::Cons { head: __pa3, tail: Deref @ metamodelica::ListNode::Nil }, ty: __pa4 } }, constFlag: __pa5 }) => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone(), __pa4.clone(), __pa5.clone()),
                _ => return Err("pattern mismatch"),
            } };
            cache = metamodelica::Own::own(__pa0);
            exp_1 = metamodelica::Own::own(__pa1);
            d1 = metamodelica::Own::own(__pa2);
            d2 = metamodelica::Own::own(__pa3);
            eltp = metamodelica::Own::own(__pa4);
            c = metamodelica::Own::own(__pa5);
            newtp = metamodelica::Ref::new(DAE::Type::T_ARRAY { ty: metamodelica::Ref::new(DAE::Type::T_ARRAY { ty: eltp, dims: list![d1] }), dims: list![d2] });
            tp = Types::simplifyType(newtp.clone())?;
            exp = Expression::makePureBuiltinCall(literal!("symmetric"), list![exp_1], tp);
            prop = DAE::Properties::PROP { type_: newtp, constFlag: c };
            (cache, exp, prop)
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok((outCache, outExp, outProperties))
}

fn elabBuiltinClassDirectory(
    mut inCache: FCore::Cache,
    mut inEnv: &FCore::Graph,
    mut inAbsynExpLst: &metamodelica::List<metamodelica::Ref<Absyn::Exp>>,
    mut inNamedArg: &metamodelica::List<metamodelica::Ref<Absyn::NamedArg>>,
    mut inBoolean: bool,
    mut inPrefix: &DAE::Prefix,
    mut info: &SourceInfo,
) -> Result<(FCore::Cache, metamodelica::Ref<DAE::Exp>, DAE::Properties)> {
    let mut outCache: FCore::Cache;
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    let mut outProperties: DAE::Properties;
    (outCache, outExp, outProperties) = (match info.clone() {
        SourceInfo {
            fileName: mut fileName, ..
        } => {
            let mut r#str: ArcStr;
            r#str = stringAppend(System::dirname(fileName.clone()), literal!("/"));
            Error::addSourceMessage(
                &(Error::NON_STANDARD_OPERATOR_CLASS_DIRECTORY.clone()),
                metamodelica::nil(),
                info,
            )?;
            (
                inCache,
                metamodelica::Ref::new(DAE::Exp::SCONST { string: r#str }),
                DAE::Properties::PROP {
                    type_: DAE::T_STRING_DEFAULT().clone(),
                    constFlag: openmodelica_frontend_types::DAE::Const::C_CONST,
                },
            )
        }
        _ => return Err("match: no arm matched"),
    });
    Ok((outCache, outExp, outProperties))
}

fn elabBuiltinSourceInfo(
    mut inCache: FCore::Cache,
    mut inEnv: &FCore::Graph,
    mut inAbsynExpLst: &metamodelica::List<metamodelica::Ref<Absyn::Exp>>,
    mut inNamedArg: &metamodelica::List<metamodelica::Ref<Absyn::NamedArg>>,
    mut inBoolean: bool,
    mut inPrefix: &DAE::Prefix,
    mut info: &SourceInfo,
) -> Result<(FCore::Cache, metamodelica::Ref<DAE::Exp>, DAE::Properties)> {
    let mut outCache: FCore::Cache;
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    let mut outProperties: DAE::Properties;
    ::match_deref::match_deref! { match &((*inAbsynExpLst)) {
        Deref @ metamodelica::ListNode::Nil => (),
        _ => return Err("pattern mismatch"),
    } };
    (outCache, outExp, outProperties) = (match info.clone() {
        SourceInfo { .. } => {
            let mut args: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
            args = list![
                metamodelica::Ref::new(DAE::Exp::SCONST {
                    string: info.fileName.clone()
                }),
                metamodelica::Ref::new(DAE::Exp::BCONST {
                    bool: info.isReadOnly.clone()
                }),
                metamodelica::Ref::new(DAE::Exp::ICONST {
                    integer: info.lineNumberStart.clone()
                }),
                metamodelica::Ref::new(DAE::Exp::ICONST {
                    integer: info.columnNumberStart.clone()
                }),
                metamodelica::Ref::new(DAE::Exp::ICONST {
                    integer: info.lineNumberEnd.clone()
                }),
                metamodelica::Ref::new(DAE::Exp::ICONST {
                    integer: info.columnNumberEnd.clone()
                }),
                metamodelica::Ref::new(DAE::Exp::RCONST {
                    real: metamodelica::OrderedFloat(0.0_f64)
                })
            ];
            outExp = metamodelica::Ref::new(DAE::Exp::METARECORDCALL {
                path: metamodelica::Ref::new(Absyn::Path::QUALIFIED {
                    name: literal!("SourceInfo"),
                    path: metamodelica::Ref::new(Absyn::Path::IDENT {
                        name: literal!("SOURCEINFO"),
                    }),
                }),
                args: args,
                fieldNames: list![
                    literal!("fileName"),
                    literal!("isReadOnly"),
                    literal!("lineNumberStart"),
                    literal!("columnNumberStart"),
                    literal!("lineNumberEnd"),
                    literal!("columnNumberEnd"),
                    literal!("lastEditTime")
                ],
                index: 0,
                typeVars: metamodelica::nil(),
            });
            (
                inCache,
                outExp,
                DAE::Properties::PROP {
                    type_: DAE::T_SOURCEINFO_DEFAULT().clone(),
                    constFlag: openmodelica_frontend_types::DAE::Const::C_CONST,
                },
            )
        }
        _ => return Err("match: no arm matched"),
    });
    Ok((outCache, outExp, outProperties))
}

fn elabBuiltinSome(
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
    mut inPosArgs: &metamodelica::List<metamodelica::Ref<Absyn::Exp>>,
    mut inNamedArgs: &metamodelica::List<metamodelica::Ref<Absyn::NamedArg>>,
    mut inImplicit: bool,
    mut inPrefix: DAE::Prefix,
    mut inInfo: SourceInfo,
) -> Result<(FCore::Cache, metamodelica::Ref<DAE::Exp>, DAE::Properties)> {
    let mut outCache: FCore::Cache;
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    let mut outProperties: DAE::Properties;
    let mut arg: metamodelica::Ref<DAE::Exp>;
    let mut prop: DAE::Properties;
    let mut ty: metamodelica::Ref<DAE::Type>;
    let mut c: DAE::Const;
    if ((inPosArgs).len() as i32) != 1 || !((inNamedArgs).is_empty()) {
        Error::addSourceMessageAndFail(
            &(Error::WRONG_TYPE_OR_NO_OF_ARGS.clone()),
            list![literal!("SOME"), literal!("")],
            &inInfo,
        )?;
        unreachable!("Error.addSourceMessageAndFail always fails — caller-side flow-analysis hint");
    } else {
        (outCache, arg, prop) = elabExpInExpression(
            inCache,
            inEnv,
            (inPosArgs).head().cloned()?,
            inImplicit,
            true,
            inPrefix,
            inInfo,
        )?;
        ty = Types::getPropType(&prop);
        (arg, ty) = Types::matchType(arg, ty, DAE::T_METABOXED_DEFAULT().clone(), true)?;
        c = Types::propAllConst(prop)?;
        outExp = metamodelica::Ref::new(DAE::Exp::META_OPTION { exp: Some(arg) });
        outProperties = DAE::Properties::PROP {
            type_: metamodelica::Ref::new(DAE::Type::T_METAOPTION { ty: ty }),
            constFlag: c,
        };
    }
    Ok((outCache, outExp, outProperties))
}

fn elabBuiltinNone(
    mut inCache: FCore::Cache,
    mut inEnv: &FCore::Graph,
    mut inPosArgs: &metamodelica::List<metamodelica::Ref<Absyn::Exp>>,
    mut inNamedArgs: &metamodelica::List<metamodelica::Ref<Absyn::NamedArg>>,
    mut inImplicit: bool,
    mut inPrefix: &DAE::Prefix,
    mut inInfo: &SourceInfo,
) -> Result<(FCore::Cache, metamodelica::Ref<DAE::Exp>, DAE::Properties)> {
    let mut outCache: FCore::Cache = inCache;
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    let mut outProperties: DAE::Properties;
    if !((inPosArgs).is_empty()) || !((inNamedArgs).is_empty()) {
        Error::addSourceMessageAndFail(
            &(Error::WRONG_TYPE_OR_NO_OF_ARGS.clone()),
            list![literal!("NONE"), literal!("")],
            inInfo,
        )?;
        unreachable!("Error.addSourceMessageAndFail always fails — caller-side flow-analysis hint");
    } else {
        outExp = metamodelica::Ref::new(DAE::Exp::META_OPTION { exp: None });
        outProperties = DAE::Properties::PROP {
            type_: metamodelica::Ref::new(DAE::Type::T_METAOPTION {
                ty: DAE::T_UNKNOWN_DEFAULT().clone(),
            }),
            constFlag: openmodelica_frontend_types::DAE::Const::C_CONST,
        };
    }
    Ok((outCache, outExp, outProperties))
}

fn elabBuiltinHomotopy(
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
    mut inPosArgs: metamodelica::List<metamodelica::Ref<Absyn::Exp>>,
    mut inNamedArgs: metamodelica::List<metamodelica::Ref<Absyn::NamedArg>>,
    mut inImplicit: bool,
    mut inPrefix: DAE::Prefix,
    mut inInfo: SourceInfo,
) -> Result<(FCore::Cache, metamodelica::Ref<DAE::Exp>, DAE::Properties)> {
    let mut outCache: FCore::Cache = inCache.clone();
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    let mut outProperties: DAE::Properties;
    let mut replaceWith: ArcStr;
    let mut e: metamodelica::Ref<Absyn::Exp>;
    let mut e1: metamodelica::Ref<Absyn::Exp>;
    let mut e2: metamodelica::Ref<Absyn::Exp>;
    replaceWith = Flags::getConfigString(Flags::REPLACE_HOMOTOPY.clone())?;
    if metamodelica::stringEq(&replaceWith, &(literal!("actual")))
        || metamodelica::stringEq(&replaceWith, &(literal!("simplified")))
    {
        let (__pa0, __pa1) = ::match_deref::match_deref! { match &(getHomotopyArguments(inPosArgs, inNamedArgs)?) {
            Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: Deref @ metamodelica::ListNode::Cons { head: __pa1, tail: Deref @ metamodelica::ListNode::Nil } } => (__pa0.clone(), __pa1.clone()),
            _ => return Err("pattern mismatch"),
        } };
        e1 = metamodelica::Own::own(__pa0);
        e2 = metamodelica::Own::own(__pa1);
        e = if (metamodelica::stringEq(&replaceWith, &(literal!("actual")))) {
            e1
        } else {
            e2
        };
        (outCache, outExp, outProperties) = elabExpInExpression(inCache, inEnv, e, inImplicit, true, inPrefix, inInfo)?;
    } else {
        (outCache, outExp, outProperties) = elabCallArgs(
            inCache,
            inEnv,
            metamodelica::Ref::new(Absyn::Path::IDENT {
                name: literal!("homotopy"),
            }),
            inPosArgs,
            inNamedArgs,
            &(metamodelica::nil()),
            inImplicit,
            inPrefix,
            inInfo,
        )?;
    }
    Ok((outCache, outExp, outProperties))
}

fn getHomotopyArguments(
    mut args: metamodelica::List<metamodelica::Ref<Absyn::Exp>>,
    mut nargs: metamodelica::List<metamodelica::Ref<Absyn::NamedArg>>,
) -> Result<metamodelica::List<metamodelica::Ref<Absyn::Exp>>> {
    let mut outPositionalArgs: metamodelica::List<metamodelica::Ref<Absyn::Exp>>;
    outPositionalArgs = (::match_deref::match_deref! { match &((args.clone(), nargs.clone())) {
        (Deref @ metamodelica::ListNode::Cons { head: e1, tail: Deref @ metamodelica::ListNode::Cons { head: e2, tail: Deref @ metamodelica::ListNode::Nil } }, _) => {
            list![e1.clone(), e2.clone()]
        },
        (Deref @ metamodelica::ListNode::Nil, Deref @ metamodelica::ListNode::Cons { head: Deref @ Absyn::NamedArg { argName: Deref @ "actual", argValue: e1 }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Absyn::NamedArg { argName: Deref @ "simplified", argValue: e2 }, tail: Deref @ metamodelica::ListNode::Nil } }) => {
            list![e1.clone(), e2.clone()]
        },
        (Deref @ metamodelica::ListNode::Nil, Deref @ metamodelica::ListNode::Cons { head: Deref @ Absyn::NamedArg { argName: Deref @ "simplified", argValue: e2 }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ Absyn::NamedArg { argName: Deref @ "actual", argValue: e1 }, tail: Deref @ metamodelica::ListNode::Nil } }) => {
            list![e1.clone(), e2.clone()]
        },
        (Deref @ metamodelica::ListNode::Cons { head: e1, tail: Deref @ metamodelica::ListNode::Nil }, Deref @ metamodelica::ListNode::Cons { head: Deref @ Absyn::NamedArg { argName: Deref @ "simplified", argValue: e2 }, tail: Deref @ metamodelica::ListNode::Nil }) => {
            list![e1.clone(), e2.clone()]
        },
        _ => {
            Error::addCompilerError({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("+replaceHomotopy: homotopy called with wrong arguments: ")); __mm_s.push_str(&*Dump::printFunctionArgsStr(&(metamodelica::Ref::new(Absyn::FunctionArgs::FUNCTIONARGS { args: args, argNames: nargs })))?); ArcStr::from(__mm_s) })?;
            return Err("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outPositionalArgs)
}

fn elabBuiltinDynamicSelect(
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
    mut inPosArgs: metamodelica::List<metamodelica::Ref<Absyn::Exp>>,
    mut inNamedArgs: metamodelica::List<metamodelica::Ref<Absyn::NamedArg>>,
    mut inImplicit: bool,
    mut inPrefix: DAE::Prefix,
    mut inInfo: SourceInfo,
) -> Result<(FCore::Cache, metamodelica::Ref<DAE::Exp>, DAE::Properties)> {
    let mut outCache: FCore::Cache = inCache.clone();
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    let mut outProperties: DAE::Properties;
    let mut msg_str: ArcStr;
    let mut astatic: metamodelica::Ref<Absyn::Exp>;
    let mut adynamic: metamodelica::Ref<Absyn::Exp>;
    let mut dstatic: metamodelica::Ref<DAE::Exp>;
    let mut ddynamic: metamodelica::Ref<DAE::Exp>;
    let mut ty: metamodelica::Ref<DAE::Type>;
    if ((inPosArgs).len() as i32) != 2 || !((inNamedArgs).is_empty()) {
        msg_str = literal!(", expected DynamicSelect(staticExp, dynamicExp)");
        printBuiltinFnArgError(
            &(literal!("DynamicSelect")),
            &msg_str,
            inPosArgs.clone(),
            inNamedArgs,
            inPrefix.clone(),
            &inInfo,
        )?;
    }
    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(inPosArgs) {
        Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: Deref @ metamodelica::ListNode::Cons { head: __pa1, tail: Deref @ metamodelica::ListNode::Nil } } => (__pa0.clone(), __pa1.clone()),
        _ => return Err("pattern mismatch"),
    } };
    astatic = metamodelica::Own::own(__pa0);
    adynamic = metamodelica::Own::own(__pa1);
    let (__pa3, __pa4, __pa6, __pa5) = ::match_deref::match_deref! { match &(elabExpInExpression(inCache, inEnv.clone(), astatic, inImplicit, true, inPrefix.clone(), inInfo.clone())?) {
        (__pa3, __pa4, __pa6 @ DAE::Properties::PROP { type_: __pa5, constFlag: _ }) => (__pa3.clone(), __pa4.clone(), __pa6.clone(), __pa5.clone()),
        _ => return Err("pattern mismatch"),
    } };
    outCache = metamodelica::Own::own(__pa3);
    dstatic = metamodelica::Own::own(__pa4);
    ty = metamodelica::Own::own(__pa5);
    outProperties = metamodelica::Own::own(__pa6);
    match '__try7: {
        (outCache, ddynamic, _) = unwrap_break_err!(elabExpInExpression(outCache.clone(), inEnv.clone(), adynamic.clone(), inImplicit, true, inPrefix.clone(), inInfo.clone()), '__try7);
        outExp = Expression::makePureBuiltinCall(
            literal!("DynamicSelect"),
            list![dstatic.clone(), ddynamic.clone()],
            ty.clone(),
        );
        Ok::<_, &'static str>((outExp.clone(),))
    } {
        Ok((__try7_o0,)) => {
            outExp = __try7_o0;
        }
        Err(_) => {
            outExp = dstatic.clone();
        }
    }
    Ok((outCache, outExp, outProperties))
}

fn elabBuiltinTranspose(
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
    mut inPosArgs: &metamodelica::List<metamodelica::Ref<Absyn::Exp>>,
    mut inNamedArg: &metamodelica::List<metamodelica::Ref<Absyn::NamedArg>>,
    mut inImpl: bool,
    mut inPrefix: DAE::Prefix,
    mut inInfo: SourceInfo,
) -> Result<(FCore::Cache, metamodelica::Ref<DAE::Exp>, DAE::Properties)> {
    let mut outCache: FCore::Cache;
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    let mut outProperties: DAE::Properties;
    let mut aexp: metamodelica::Ref<Absyn::Exp>;
    let mut exp: metamodelica::Ref<DAE::Exp>;
    let mut ty: metamodelica::Ref<DAE::Type>;
    let mut el_ty: metamodelica::Ref<DAE::Type>;
    let mut c: DAE::Const;
    let mut d1: metamodelica::Ref<DAE::Dimension>;
    let mut d2: metamodelica::Ref<DAE::Dimension>;
    let __pa0 = ::match_deref::match_deref! { match &((*inPosArgs)) {
        Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: Deref @ metamodelica::ListNode::Nil } => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    aexp = metamodelica::Own::own(__pa0);
    let (__pa2, __pa3, __pa4, __pa5) = ::match_deref::match_deref! { match &(elabExpInExpression(inCache, inEnv, aexp, inImpl, true, inPrefix, inInfo)?) {
        (__pa2, __pa3, DAE::Properties::PROP { type_: __pa4, constFlag: __pa5 }) => (__pa2.clone(), __pa3.clone(), __pa4.clone(), __pa5.clone()),
        _ => return Err("pattern mismatch"),
    } };
    outCache = metamodelica::Own::own(__pa2);
    exp = metamodelica::Own::own(__pa3);
    ty = metamodelica::Own::own(__pa4);
    c = metamodelica::Own::own(__pa5);
    let (__pa6, __pa7, __pa8) = ::match_deref::match_deref! { match &(ty) {
        Deref @ DAE::Type::T_ARRAY { ty: Deref @ DAE::Type::T_ARRAY { ty: __pa6, dims: Deref @ metamodelica::ListNode::Cons { head: __pa7, tail: Deref @ metamodelica::ListNode::Nil } }, dims: Deref @ metamodelica::ListNode::Cons { head: __pa8, tail: Deref @ metamodelica::ListNode::Nil } } => (__pa6.clone(), __pa7.clone(), __pa8.clone()),
        _ => return Err("pattern mismatch"),
    } };
    el_ty = metamodelica::Own::own(__pa6);
    d1 = metamodelica::Own::own(__pa7);
    d2 = metamodelica::Own::own(__pa8);
    ty = metamodelica::Ref::new(DAE::Type::T_ARRAY {
        ty: metamodelica::Ref::new(DAE::Type::T_ARRAY {
            ty: el_ty,
            dims: list![d2],
        }),
        dims: list![d1],
    });
    outProperties = DAE::Properties::PROP {
        type_: ty.clone(),
        constFlag: c,
    };
    ty = Types::simplifyType(ty)?;
    outExp = Expression::makePureBuiltinCall(literal!("transpose"), list![exp], ty);
    Ok((outCache, outExp, outProperties))
}

fn elabBuiltinSum(
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
    mut inAbsynExpLst: &metamodelica::List<metamodelica::Ref<Absyn::Exp>>,
    mut inNamedArg: &metamodelica::List<metamodelica::Ref<Absyn::NamedArg>>,
    mut inBoolean: bool,
    mut inPrefix: DAE::Prefix,
    mut info: SourceInfo,
) -> Result<(FCore::Cache, metamodelica::Ref<DAE::Exp>, DAE::Properties)> {
    let mut outCache: FCore::Cache;
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    let mut outProperties: DAE::Properties;
    (outCache, outExp, outProperties) = (::match_deref::match_deref! { match inAbsynExpLst {
        Deref @ metamodelica::ListNode::Cons { head: arrexp, tail: Deref @ metamodelica::ListNode::Nil } => {
            let mut cache = inCache;
            let mut env = inEnv;
            let mut r#impl = inBoolean;
            let mut pre = inPrefix;
            let mut exp_1: metamodelica::Ref<DAE::Exp>;
            let mut exp_2: metamodelica::Ref<DAE::Exp>;
            let mut t: metamodelica::Ref<DAE::Type>;
            let mut tp: metamodelica::Ref<DAE::Type>;
            let mut c: DAE::Const;
            let mut b: bool;
            let mut estr: ArcStr;
            let mut tstr: ArcStr;
            let mut etp: metamodelica::Ref<DAE::Type>;
            let (__pa0, __pa1, __pa2, __pa3) = ::match_deref::match_deref! { match &(elabExpInExpression(cache, env, arrexp.clone(), r#impl, true, pre, info.clone())?) {
                (__pa0, __pa1, DAE::Properties::PROP { type_: __pa2, constFlag: __pa3 }) => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone()),
                _ => return Err("pattern mismatch"),
            } };
            cache = metamodelica::Own::own(__pa0);
            exp_1 = metamodelica::Own::own(__pa1);
            t = metamodelica::Own::own(__pa2);
            c = metamodelica::Own::own(__pa3);
            tp = Types::arrayElementType(&t);
            etp = Types::simplifyType(tp.clone())?;
            b = Types::isArray(&t);
            b = b && Types::isSimpleType(&tp);
            estr = Dump::printExpStr(arrexp.clone())?;
            tstr = TypesDump::unparseType(t)?;
            Error::assertionOrAddSourceMessage(b, &(Error::SUM_EXPECTED_ARRAY.clone()), list![estr, tstr], &info)?;
            exp_2 = Expression::makePureBuiltinCall(literal!("sum"), list![exp_1], etp);
            (cache, exp_2, DAE::Properties::PROP { type_: tp, constFlag: c })
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok((outCache, outExp, outProperties))
}

fn elabBuiltinProduct(
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
    mut inAbsynExpLst: &metamodelica::List<metamodelica::Ref<Absyn::Exp>>,
    mut inNamedArg: &metamodelica::List<metamodelica::Ref<Absyn::NamedArg>>,
    mut inBoolean: bool,
    mut inPrefix: DAE::Prefix,
    mut info: SourceInfo,
) -> Result<(FCore::Cache, metamodelica::Ref<DAE::Exp>, DAE::Properties)> {
    let mut outCache: FCore::Cache;
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    let mut outProperties: DAE::Properties;
    (outCache, outExp, outProperties) = 'mc: {
        let __mc_input = (inCache, inEnv, &**inAbsynExpLst, inBoolean, inPrefix);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, Deref @ metamodelica::ListNode::Cons { head: arrexp, tail: Deref @ metamodelica::ListNode::Nil }, r#impl, pre) => {
                    let mut exp_1: metamodelica::Ref<DAE::Exp>;
                    let mut c: DAE::Const;
                    let mut ty: metamodelica::Ref<DAE::Type>;
                    let mut str_exp: ArcStr;
                    let mut str_pre: ArcStr;
                    let mut cache = (*cache).clone();
                    let (__pa0, __pa1, __pa2, __pa3) = ::match_deref::match_deref! { match &(elabExpInExpression(cache.clone(), env.clone(), arrexp.clone(), r#impl.clone(), true, pre.clone(), info.clone())?) {
                        (__pa0, __pa1, DAE::Properties::PROP { type_: __pa2, constFlag: __pa3 }) => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    cache = metamodelica::Own::own(__pa0);
                    exp_1 = metamodelica::Own::own(__pa1);
                    ty = metamodelica::Own::own(__pa2);
                    c = metamodelica::Own::own(__pa3);
                    (exp_1, _) = Types::matchType(exp_1.clone(), ty.clone(), DAE::T_INTEGER_DEFAULT().clone(), true)?;
                    str_exp = { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("product(")); __mm_s.push_str(&*Dump::printExpStr(arrexp.clone())?); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) };
                    str_pre = PrefixUtil::printPrefixStr3(pre.clone())?;
                    Error::addSourceMessage(&(Error::BUILTIN_FUNCTION_PRODUCT_HAS_SCALAR_PARAMETER.clone()), list![str_exp.clone(), str_pre.clone()], &info)?;
                    Ok((cache.clone(), exp_1.clone(), DAE::Properties::PROP { type_: DAE::T_INTEGER_DEFAULT().clone(), constFlag: c }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, Deref @ metamodelica::ListNode::Cons { head: arrexp, tail: Deref @ metamodelica::ListNode::Nil }, r#impl, pre) => {
                    let mut exp_1: metamodelica::Ref<DAE::Exp>;
                    let mut c: DAE::Const;
                    let mut ty: metamodelica::Ref<DAE::Type>;
                    let mut str_exp: ArcStr;
                    let mut str_pre: ArcStr;
                    let mut cache = (*cache).clone();
                    let (__pa0, __pa1, __pa2, __pa3) = ::match_deref::match_deref! { match &(elabExpInExpression(cache.clone(), env.clone(), arrexp.clone(), r#impl.clone(), true, pre.clone(), info.clone())?) {
                        (__pa0, __pa1, DAE::Properties::PROP { type_: __pa2, constFlag: __pa3 }) => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    cache = metamodelica::Own::own(__pa0);
                    exp_1 = metamodelica::Own::own(__pa1);
                    ty = metamodelica::Own::own(__pa2);
                    c = metamodelica::Own::own(__pa3);
                    (exp_1, _) = Types::matchType(exp_1.clone(), ty.clone(), DAE::T_REAL_DEFAULT().clone(), true)?;
                    str_exp = { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("product(")); __mm_s.push_str(&*Dump::printExpStr(arrexp.clone())?); __mm_s.push_str(&*literal!(")")); ArcStr::from(__mm_s) };
                    str_pre = PrefixUtil::printPrefixStr3(pre.clone())?;
                    Error::addSourceMessage(&(Error::BUILTIN_FUNCTION_PRODUCT_HAS_SCALAR_PARAMETER.clone()), list![str_exp.clone(), str_pre.clone()], &info)?;
                    Ok((cache.clone(), exp_1.clone(), DAE::Properties::PROP { type_: DAE::T_REAL_DEFAULT().clone(), constFlag: c }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, Deref @ metamodelica::ListNode::Cons { head: arrexp, tail: Deref @ metamodelica::ListNode::Nil }, r#impl, pre) => {
                    let mut exp_1: metamodelica::Ref<DAE::Exp>;
                    let mut exp_2: metamodelica::Ref<DAE::Exp>;
                    let mut t: metamodelica::Ref<DAE::Type>;
                    let mut tp: metamodelica::Ref<DAE::Type>;
                    let mut c: DAE::Const;
                    let mut etp: metamodelica::Ref<DAE::Type>;
                    let mut cache = (*cache).clone();
                    let (__pa0, __pa1, __pa3, __pa2, __pa4) = ::match_deref::match_deref! { match &(elabExpInExpression(cache.clone(), env.clone(), arrexp.clone(), r#impl.clone(), true, pre.clone(), info.clone())?) {
                        (__pa0, __pa1, DAE::Properties::PROP { type_: __pa3 @ Deref @ DAE::Type::T_ARRAY { dims: Deref @ metamodelica::ListNode::Cons { head: _, tail: Deref @ metamodelica::ListNode::Nil }, ty: __pa2 }, constFlag: __pa4 }) => (__pa0.clone(), __pa1.clone(), __pa3.clone(), __pa2.clone(), __pa4.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    cache = metamodelica::Own::own(__pa0);
                    exp_1 = metamodelica::Own::own(__pa1);
                    tp = metamodelica::Own::own(__pa2);
                    t = metamodelica::Own::own(__pa3);
                    c = metamodelica::Own::own(__pa4);
                    tp = Types::arrayElementType(&t);
                    etp = Types::simplifyType(tp.clone())?;
                    exp_2 = Expression::makePureBuiltinCall(literal!("product"), list![exp_1.clone()], etp.clone());
                    if !(Types::arrayHasUnknownDims(&t)?) {
                        exp_2 = elabBuiltinProduct2(exp_2.clone());
                    }
                    Ok((cache.clone(), exp_2.clone(), DAE::Properties::PROP { type_: tp.clone(), constFlag: c }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((outCache, outExp, outProperties))
}

fn elabBuiltinProduct2(mut inExp: metamodelica::Ref<DAE::Exp>) -> metamodelica::Ref<DAE::Exp> {
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    outExp = 'mc: {
        let __mc_input = &*inExp;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::ListNode::Cons { head: array_exp, tail: Deref @ metamodelica::ListNode::Nil }, .. } => {
                    Ok(Expression::makeProductLst(Expression::arrayElements(array_exp.clone())?)?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(inExp.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    outExp
}

fn elabBuiltinPre(
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
    mut inPosArgs: &metamodelica::List<metamodelica::Ref<Absyn::Exp>>,
    mut inNamedArgs: &metamodelica::List<metamodelica::Ref<Absyn::NamedArg>>,
    mut inImplicit: bool,
    mut inPrefix: DAE::Prefix,
    mut inInfo: SourceInfo,
) -> Result<(FCore::Cache, metamodelica::Ref<DAE::Exp>, DAE::Properties)> {
    let mut outCache: FCore::Cache;
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    let mut outProperties: DAE::Properties;
    let mut exp: metamodelica::Ref<DAE::Exp>;
    let mut ty: metamodelica::Ref<DAE::Type>;
    let mut ty2: metamodelica::Ref<DAE::Type>;
    let mut c: DAE::Const;
    let mut expl: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
    let mut sc: bool;
    let mut exp_str: ArcStr;
    let mut pre_str: ArcStr;
    checkBuiltinCallArgs(inPosArgs, inNamedArgs, 1, literal!("pre"), &inInfo)?;
    let (__pa0, __pa1, __pa2, __pa3) = ::match_deref::match_deref! { match &(elabExpInExpression(inCache, inEnv, (inPosArgs).head().cloned()?, inImplicit, true, inPrefix.clone(), inInfo.clone())?) {
        (__pa0, __pa1, DAE::Properties::PROP { type_: __pa2, constFlag: __pa3 }) => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone()),
        _ => return Err("pattern mismatch"),
    } };
    outCache = metamodelica::Own::own(__pa0);
    exp = metamodelica::Own::own(__pa1);
    ty = metamodelica::Own::own(__pa2);
    c = metamodelica::Own::own(__pa3);
    if Expression::isMatrix(&exp) {
        let __pa4 = ::match_deref::match_deref! { match &(ty.clone()) {
            Deref @ DAE::Type::T_ARRAY { ty: __pa4, .. } => __pa4.clone(),
            _ => return Err("pattern mismatch"),
        } };
        ty2 = metamodelica::Own::own(__pa4);
        ty2 = Types::unliftArray(&ty2)?;
        outExp = Expression::makePureBuiltinCall(literal!("pre"), list![exp], Types::simplifyType(ty2.clone())?);
        outExp = elabBuiltinPreMatrix(outExp, ty2)?;
    } else if Types::isArray(&ty) {
        ty2 = Types::unliftArray(&ty)?;
        outExp = Expression::makePureBuiltinCall(literal!("pre"), list![exp], Types::simplifyType(ty2.clone())?);
        (expl, sc) = elabBuiltinPre2(outExp, ty2);
        outExp = metamodelica::Ref::new(DAE::Exp::ARRAY {
            ty: Types::simplifyType(ty.clone())?,
            scalar: sc,
            array: expl,
        });
    } else {
        ty = Types::arrayElementType(&ty);
        if Types::basicType(&ty) {
            outExp = Expression::makePureBuiltinCall(literal!("pre"), list![exp], Types::simplifyType(ty.clone())?);
        } else {
            exp_str = ExpressionBasics::printExpStr(exp)?;
            pre_str = PrefixUtil::printPrefixStr3(inPrefix)?;
            Error::addSourceMessageAndFail(
                &(Error::OPERAND_BUILTIN_TYPE.clone()),
                list![literal!("pre"), pre_str, exp_str],
                &inInfo,
            )?;
            unreachable!("Error.addSourceMessageAndFail always fails — caller-side flow-analysis hint");
        }
    }
    outProperties = DAE::Properties::PROP {
        type_: ty,
        constFlag: c,
    };
    Ok((outCache, outExp, outProperties))
}

fn elabBuiltinPre2(
    mut inExp: metamodelica::Ref<DAE::Exp>,
    mut inType: metamodelica::Ref<DAE::Type>,
) -> (metamodelica::List<metamodelica::Ref<DAE::Exp>>, bool) {
    let mut outExp: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
    let mut outScalar: bool;
    (outExp, outScalar) = 'mc: {
        let __mc_input = &*inExp;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Exp::ARRAY { scalar: sc, array: expl, .. }, tail: Deref @ metamodelica::ListNode::Nil }, .. } => {
                    Ok((makePreLst(expl.clone(), inType.clone())?, sc.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                        Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Exp::MATRIX { ty, integer: i, matrix: mexpl }, tail: Deref @ metamodelica::ListNode::Nil }, .. } => {
                            let mut mexpl = (*mexpl).clone();
                            mexpl = ({
                let mut __acc: metamodelica::List<metamodelica::List<metamodelica::Ref<DAE::Exp>>> = metamodelica::nil();
                for mut e in (mexpl.clone()).into_iter().cloned() {
                            let __x = makePreLst(e.clone(), inType.clone())?;
                            __acc = cons(__x, __acc);
                }
                __acc.reverse()
            });
                            Ok((list![metamodelica::Ref::new(DAE::Exp::MATRIX { ty: ty.clone(), integer: i.clone(), matrix: mexpl.clone() })], false))
                        }
                        _ => return Err("nomatch"),
                    }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((list![inExp.clone()], false))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    (outExp, outScalar)
}

fn elabBuiltinInStream(
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
    mut inArgs: &metamodelica::List<metamodelica::Ref<Absyn::Exp>>,
    mut inNamedArgs: &metamodelica::List<metamodelica::Ref<Absyn::NamedArg>>,
    mut inImpl: bool,
    mut inPrefix: DAE::Prefix,
    mut inInfo: SourceInfo,
) -> Result<(FCore::Cache, metamodelica::Ref<DAE::Exp>, DAE::Properties)> {
    let mut outCache: FCore::Cache;
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    let mut outProperties: DAE::Properties;
    let mut e: metamodelica::Ref<Absyn::Exp>;
    let mut exp: metamodelica::Ref<DAE::Exp>;
    let mut ty: metamodelica::Ref<DAE::Type>;
    let __pa0 = ::match_deref::match_deref! { match &((*inArgs)) {
        Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: Deref @ metamodelica::ListNode::Nil } => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    e = metamodelica::Own::own(__pa0);
    (outCache, exp, outProperties) = elabExpInExpression(
        inCache,
        inEnv.clone(),
        e.clone(),
        inImpl,
        true,
        inPrefix.clone(),
        inInfo.clone(),
    )?;
    ty = Types::getPropType(&outProperties);
    outExp = elabBuiltinStreamOperator(
        outCache.clone(),
        inEnv.clone(),
        literal!("inStream"),
        exp,
        ty.clone(),
        &inInfo,
    )?;
    if Types::dimensionsKnown(ty) {
        (outCache, outExp, outProperties) = elabCallArgs(
            outCache,
            inEnv,
            metamodelica::Ref::new(Absyn::Path::IDENT {
                name: literal!("inStream"),
            }),
            list![e],
            metamodelica::nil(),
            &(metamodelica::nil()),
            inImpl,
            inPrefix,
            inInfo,
        )?;
    }
    Ok((outCache, outExp, outProperties))
}

fn elabBuiltinActualStream(
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
    mut inArgs: &metamodelica::List<metamodelica::Ref<Absyn::Exp>>,
    mut inNamedArgs: &metamodelica::List<metamodelica::Ref<Absyn::NamedArg>>,
    mut inImpl: bool,
    mut inPrefix: DAE::Prefix,
    mut inInfo: SourceInfo,
) -> Result<(FCore::Cache, metamodelica::Ref<DAE::Exp>, DAE::Properties)> {
    let mut outCache: FCore::Cache;
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    let mut outProperties: DAE::Properties;
    let mut e: metamodelica::Ref<Absyn::Exp>;
    let mut exp: metamodelica::Ref<DAE::Exp>;
    let mut ty: metamodelica::Ref<DAE::Type>;
    let __pa0 = ::match_deref::match_deref! { match &((*inArgs)) {
        Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: Deref @ metamodelica::ListNode::Nil } => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    e = metamodelica::Own::own(__pa0);
    (outCache, exp, outProperties) = elabExpInExpression(
        inCache,
        inEnv.clone(),
        e.clone(),
        inImpl,
        true,
        inPrefix.clone(),
        inInfo.clone(),
    )?;
    ty = Types::getPropType(&outProperties);
    outExp = elabBuiltinStreamOperator(
        outCache.clone(),
        inEnv.clone(),
        literal!("actualStream"),
        exp,
        ty.clone(),
        &inInfo,
    )?;
    if Types::dimensionsKnown(ty) {
        (outCache, outExp, outProperties) = elabCallArgs(
            outCache,
            inEnv,
            metamodelica::Ref::new(Absyn::Path::IDENT {
                name: literal!("actualStream"),
            }),
            list![e],
            metamodelica::nil(),
            &(metamodelica::nil()),
            inImpl,
            inPrefix,
            inInfo,
        )?;
    }
    Ok((outCache, outExp, outProperties))
}

fn elabBuiltinStreamOperator(
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
    mut inOperator: ArcStr,
    mut inExp: metamodelica::Ref<DAE::Exp>,
    mut inType: metamodelica::Ref<DAE::Type>,
    mut inInfo: &SourceInfo,
) -> Result<metamodelica::Ref<DAE::Exp>> {
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    outExp = (::match_deref::match_deref! { match &(inExp.clone()) {
        Deref @ DAE::Exp::ARRAY { array: Deref @ metamodelica::ListNode::Nil, .. } => {
            inExp
        },
        _ => {
            let mut et: metamodelica::Ref<DAE::Type>;
            let mut exp: metamodelica::Ref<DAE::Exp>;
            let __pa0 = ::match_deref::match_deref! { match &(Expression::flattenArrayExpToList(inExp)) {
                Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: _ } => __pa0.clone(),
                _ => return Err("pattern mismatch"),
            } };
            exp = metamodelica::Own::own(__pa0);
            validateBuiltinStreamOperator(inCache, inEnv, exp.clone(), &inType, inOperator.clone(), inInfo)?;
            et = Types::simplifyType(inType)?;
            exp = Expression::makePureBuiltinCall(inOperator, list![exp], et);
            exp
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outExp)
}

fn validateBuiltinStreamOperator(
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
    mut inOperand: metamodelica::Ref<DAE::Exp>,
    mut inType: &metamodelica::Ref<DAE::Type>,
    mut inOperator: ArcStr,
    mut inInfo: &SourceInfo,
) -> Result<()> {
    let () = 'mc: {
        let __mc_input = &*inOperand;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::CREF { componentRef: cr, .. } => {
                    let mut attr: metamodelica::Ref<DAE::Attributes>;
                    (_, attr, _, _, _, _, _, _, _) = Lookup::lookupVar(inCache.clone(), inEnv.clone(), cr.clone())?;
                    ::match_deref::match_deref! { match &(attr.clone()) {
                        Deref @ DAE::Attributes { connectorType: Deref @ DAE::ConnectorType::STREAM { .. }, .. } => (),
                        _ => return Err("pattern mismatch"),
                    } };
                    Ok(())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut op_str: ArcStr;
                    op_str = ExpressionBasics::printExpStr(inOperand.clone())?;
                    Error::addSourceMessage(&(Error::NON_STREAM_OPERAND_IN_STREAM_OPERATOR.clone()), list![op_str.clone(), inOperator.clone()], inInfo)?;
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(())
}

fn makePreLst(
    mut inExpl: metamodelica::List<metamodelica::Ref<DAE::Exp>>,
    mut inType: metamodelica::Ref<DAE::Type>,
) -> Result<metamodelica::List<metamodelica::Ref<DAE::Exp>>> {
    let mut outExpl: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
    let mut ty: metamodelica::Ref<DAE::Type>;
    ty = Types::simplifyType(inType)?;
    outExpl = ({
        let mut __acc: metamodelica::List<metamodelica::Ref<DAE::Exp>> = metamodelica::nil();
        for mut e in (inExpl).into_iter().cloned() {
            let __x = Expression::makePureBuiltinCall(literal!("pre"), list![e.clone()], ty.clone());
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    Ok(outExpl)
}

fn elabBuiltinPreMatrix(
    mut inExp: metamodelica::Ref<DAE::Exp>,
    mut inType: metamodelica::Ref<DAE::Type>,
) -> Result<metamodelica::Ref<DAE::Exp>> {
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    outExp = (::match_deref::match_deref! { match &(inExp.clone()) {
        Deref @ DAE::Exp::CALL { expLst: Deref @ metamodelica::ListNode::Cons { head: exp @ Deref @ DAE::Exp::MATRIX { .. }, tail: Deref @ metamodelica::ListNode::Nil }, .. } => {
            let mut exp = (*exp).clone();
            assign_variant_field!(exp => DAE::Exp::MATRIX; matrix = ({
        let mut __acc: metamodelica::List<metamodelica::List<metamodelica::Ref<DAE::Exp>>> = metamodelica::nil();
        for mut row in (var_field!((*exp).matrix, DAE::Exp::MATRIX).clone()).into_iter().cloned() {
            let __x = makePreLst(row.clone(), inType.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }));
            exp.clone()
        },
        _ => {
            inExp
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outExp)
}

fn elabBuiltinArray(
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
    mut inPosArgs: &metamodelica::List<metamodelica::Ref<Absyn::Exp>>,
    mut inNamedArgs: &metamodelica::List<metamodelica::Ref<Absyn::NamedArg>>,
    mut inImplicit: bool,
    mut inPrefix: DAE::Prefix,
    mut inInfo: SourceInfo,
) -> Result<(FCore::Cache, metamodelica::Ref<DAE::Exp>, DAE::Properties)> {
    let mut outCache: FCore::Cache;
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    let mut outProperties: DAE::Properties;
    let mut expl: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
    let mut props: metamodelica::List<DAE::Properties>;
    let mut ty: metamodelica::Ref<DAE::Type>;
    let mut arr_ty: metamodelica::Ref<DAE::Type>;
    let mut c: DAE::Const;
    let mut len: i32;
    (outCache, expl, props) = elabExpList(
        inCache,
        inEnv,
        inPosArgs,
        inImplicit,
        true,
        inPrefix.clone(),
        inInfo.clone(),
        DAE::T_UNKNOWN_DEFAULT().clone(),
    )?;
    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(elabBuiltinArray2(&expl, props, inPrefix, &inInfo)?) {
        (_, DAE::Properties::PROP { type_: __pa0, constFlag: __pa1 }) => (__pa0.clone(), __pa1.clone()),
        _ => return Err("pattern mismatch"),
    } };
    ty = metamodelica::Own::own(__pa0);
    c = metamodelica::Own::own(__pa1);
    len = ((expl).len() as i32);
    arr_ty = metamodelica::Ref::new(DAE::Type::T_ARRAY {
        ty: ty.clone(),
        dims: list![metamodelica::Ref::new(DAE::Dimension::DIM_INTEGER { integer: len })],
    });
    outProperties = DAE::Properties::PROP {
        type_: arr_ty.clone(),
        constFlag: c,
    };
    arr_ty = Types::simplifyType(arr_ty)?;
    outExp = metamodelica::Ref::new(DAE::Exp::ARRAY {
        ty: arr_ty,
        scalar: Types::isArray(&ty),
        array: expl,
    });
    Ok((outCache, outExp, outProperties))
}

fn elabBuiltinArray2(
    mut inExpl: &metamodelica::List<metamodelica::Ref<DAE::Exp>>,
    mut inProperties: metamodelica::List<DAE::Properties>,
    mut inPrefix: DAE::Prefix,
    mut inInfo: &SourceInfo,
) -> Result<(metamodelica::List<metamodelica::Ref<DAE::Exp>>, DAE::Properties)> {
    let mut outExpl: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
    let mut outProperties: DAE::Properties;
    let mut pre_str: ArcStr;
    let mut prop: DAE::Properties;
    if !(sameDimensions(inProperties.clone())?) {
        pre_str = PrefixUtil::printPrefixStr3(inPrefix)?;
        Error::addSourceMessageAndFail(
            &(Error::DIFFERENT_DIM_SIZE_IN_ARGUMENTS.clone()),
            list![literal!("array"), pre_str],
            inInfo,
        )?;
        unreachable!("Error.addSourceMessageAndFail always fails — caller-side flow-analysis hint");
    }
    prop = if (Types::propsContainReal(&inProperties)) {
        DAE::Properties::PROP {
            type_: DAE::T_REAL_DEFAULT().clone(),
            constFlag: openmodelica_frontend_types::DAE::Const::C_VAR,
        }
    } else {
        (inProperties).head().cloned()?
    };
    (outExpl, outProperties) = elabBuiltinArray3(inExpl, inProperties, &prop)?;
    Ok((outExpl, outProperties))
}

fn elabBuiltinArray3(
    mut inExpl: &metamodelica::List<metamodelica::Ref<DAE::Exp>>,
    mut inPropertiesLst: metamodelica::List<DAE::Properties>,
    mut inProperties: &DAE::Properties,
) -> Result<(metamodelica::List<metamodelica::Ref<DAE::Exp>>, DAE::Properties)> {
    let mut outExpl: metamodelica::List<metamodelica::Ref<DAE::Exp>> = metamodelica::nil();
    let mut outProperties: DAE::Properties = (inPropertiesLst).head().cloned()?;
    let mut prop: DAE::Properties;
    let mut rest_props: metamodelica::List<DAE::Properties> = inPropertiesLst.clone();
    for mut e in &**inExpl {
        let mut e = e.clone();
        let (__pa0, __pa1) = ::match_deref::match_deref! { match &(rest_props) {
            Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
            _ => return Err("pattern mismatch"),
        } };
        prop = metamodelica::Own::own(__pa0);
        rest_props = metamodelica::Own::own(__pa1);
        (e, _) = Types::matchProp(e, &prop, inProperties, true)?;
        outExpl = metamodelica::cons(e, outExpl);
    }
    outExpl = outExpl.reverse();
    Ok((outExpl, outProperties))
}

fn elabBuiltinZeros(
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
    mut inPosArgs: metamodelica::List<metamodelica::Ref<Absyn::Exp>>,
    mut inNamedArgs: &metamodelica::List<metamodelica::Ref<Absyn::NamedArg>>,
    mut inImplicit: bool,
    mut inPrefix: DAE::Prefix,
    mut inInfo: SourceInfo,
) -> Result<(FCore::Cache, metamodelica::Ref<DAE::Exp>, DAE::Properties)> {
    let mut outCache: FCore::Cache;
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    let mut outProperties: DAE::Properties;
    (outCache, outExp, outProperties) = elabBuiltinFill(
        inCache,
        inEnv,
        metamodelica::cons(metamodelica::Ref::new(Absyn::Exp::INTEGER { value: 0 }), inPosArgs),
        &(metamodelica::nil()),
        inImplicit,
        inPrefix,
        inInfo,
    )?;
    Ok((outCache, outExp, outProperties))
}

fn sameDimensions(mut inProps: metamodelica::List<DAE::Properties>) -> Result<bool> {
    let mut res: bool;
    let mut types: metamodelica::List<metamodelica::Ref<DAE::Type>>;
    let mut dims: metamodelica::List<metamodelica::List<metamodelica::Ref<DAE::Dimension>>>;
    types = List::map(inProps, &move |__a0: DAE::Properties| -> metamodelica::Result<_> {
        ::std::result::Result::Ok(Types::getPropType(&__a0))
    })?;
    dims = List::map(
        types,
        &move |__a0: metamodelica::Ref<DAE::Type>| -> metamodelica::Result<_> {
            ::std::result::Result::Ok(TypesDump::getDimensions(&__a0))
        },
    )?;
    res = sameDimensions2(dims)?;
    Ok(res)
}

fn sameDimensionsExceptionDimX(
    mut inProps: metamodelica::List<DAE::Properties>,
    mut dimException: i32,
) -> Result<bool> {
    let mut res: bool;
    let mut types: metamodelica::List<metamodelica::Ref<DAE::Type>>;
    let mut dims: metamodelica::List<metamodelica::List<metamodelica::Ref<DAE::Dimension>>>;
    types = List::map(inProps, &move |__a0: DAE::Properties| -> metamodelica::Result<_> {
        ::std::result::Result::Ok(Types::getPropType(&__a0))
    })?;
    dims = List::map(
        types,
        &move |__a0: metamodelica::Ref<DAE::Type>| -> metamodelica::Result<_> {
            ::std::result::Result::Ok(TypesDump::getDimensions(&__a0))
        },
    )?;
    dims = List::map1(dims, &*(Arc::new(listDelete.clone())), dimException)?;
    res = sameDimensions2(dims)?;
    Ok(res)
}

fn sameDimensions2(
    mut inDimensions: metamodelica::List<metamodelica::List<metamodelica::Ref<DAE::Dimension>>>,
) -> Result<bool> {
    let mut outSame: bool = true;
    let mut dims: metamodelica::List<metamodelica::Ref<DAE::Dimension>>;
    let mut rest_dims: metamodelica::List<metamodelica::List<metamodelica::Ref<DAE::Dimension>>> = inDimensions.clone();
    if (inDimensions).is_empty() {
        return Ok(outSame);
    }
    while !(((rest_dims).head().cloned()?).is_empty()) {
        dims = ({
            let mut __acc: metamodelica::List<metamodelica::Ref<DAE::Dimension>> = metamodelica::nil();
            for mut d in (rest_dims.clone()).into_iter().cloned() {
                let __x = (d).head().cloned()?;
                __acc = cons(__x, __acc);
            }
            __acc.reverse()
        });
        if !(sameDimensions3(&dims)?) {
            outSame = false;
            return Ok(outSame);
        }
        rest_dims = ({
            let mut __acc: metamodelica::List<metamodelica::List<metamodelica::Ref<DAE::Dimension>>> =
                metamodelica::nil();
            for mut d in (rest_dims).into_iter().cloned() {
                let __x = (d).rest()?;
                __acc = cons(__x, __acc);
            }
            __acc.reverse()
        });
    }
    for mut d in &*rest_dims {
        let true = ((d).is_empty()) else {
            return Err("pattern mismatch");
        };
    }
    Ok(outSame)
}

fn sameDimensions3(mut inDims: &metamodelica::List<metamodelica::Ref<DAE::Dimension>>) -> Result<bool> {
    let mut outSame: bool = true;
    let mut dim1: metamodelica::Ref<DAE::Dimension>;
    if (inDims).is_empty() {
        return Ok(outSame);
    }
    dim1 = (inDims).head().cloned()?;
    for mut dim2 in &*(inDims).rest()? {
        if !(Expression::dimensionsEqual(&dim1, metamodelica::AsArg::as_arg(&dim2))?) {
            outSame = false;
            return Ok(outSame);
        }
    }
    Ok(outSame)
}

fn elabBuiltinOnes(
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
    mut inPosArgs: metamodelica::List<metamodelica::Ref<Absyn::Exp>>,
    mut inNamedArg: &metamodelica::List<metamodelica::Ref<Absyn::NamedArg>>,
    mut inImplicit: bool,
    mut inPrefix: DAE::Prefix,
    mut inInfo: SourceInfo,
) -> Result<(FCore::Cache, metamodelica::Ref<DAE::Exp>, DAE::Properties)> {
    let mut outCache: FCore::Cache;
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    let mut outProperties: DAE::Properties;
    (outCache, outExp, outProperties) = elabBuiltinFill(
        inCache,
        inEnv,
        metamodelica::cons(metamodelica::Ref::new(Absyn::Exp::INTEGER { value: 1 }), inPosArgs),
        &(metamodelica::nil()),
        inImplicit,
        inPrefix,
        inInfo,
    )?;
    Ok((outCache, outExp, outProperties))
}

fn elabBuiltinMax(
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
    mut inFnArgs: &metamodelica::List<metamodelica::Ref<Absyn::Exp>>,
    mut inNamedArg: &metamodelica::List<metamodelica::Ref<Absyn::NamedArg>>,
    mut inImpl: bool,
    mut inPrefix: DAE::Prefix,
    mut info: SourceInfo,
) -> Result<(FCore::Cache, metamodelica::Ref<DAE::Exp>, DAE::Properties)> {
    let mut outCache: FCore::Cache;
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    let mut outProperties: DAE::Properties;
    (outCache, outExp, outProperties) =
        elabBuiltinMinMaxCommon(inCache, inEnv, literal!("max"), inFnArgs, inImpl, inPrefix, info)?;
    Ok((outCache, outExp, outProperties))
}

fn elabBuiltinMin(
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
    mut inFnArgs: &metamodelica::List<metamodelica::Ref<Absyn::Exp>>,
    mut inNamedArg: &metamodelica::List<metamodelica::Ref<Absyn::NamedArg>>,
    mut inImpl: bool,
    mut inPrefix: DAE::Prefix,
    mut info: SourceInfo,
) -> Result<(FCore::Cache, metamodelica::Ref<DAE::Exp>, DAE::Properties)> {
    let mut outCache: FCore::Cache;
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    let mut outProperties: DAE::Properties;
    (outCache, outExp, outProperties) =
        elabBuiltinMinMaxCommon(inCache, inEnv, literal!("min"), inFnArgs, inImpl, inPrefix, info)?;
    Ok((outCache, outExp, outProperties))
}

fn elabBuiltinMinMaxCommon(
    mut cache: FCore::Cache,
    mut env: FCore::Graph,
    mut inFnName: ArcStr,
    mut inFnArgs: &metamodelica::List<metamodelica::Ref<Absyn::Exp>>,
    mut r#impl: bool,
    mut prefix: DAE::Prefix,
    mut info: SourceInfo,
) -> Result<(FCore::Cache, metamodelica::Ref<DAE::Exp>, DAE::Properties)> {
    let mut cache: FCore::Cache = cache;
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    let mut outProperties: DAE::Properties;
    (outExp, outProperties) = (::match_deref::match_deref! { match inFnArgs {
        Deref @ metamodelica::ListNode::Cons { head: arrexp, tail: Deref @ metamodelica::ListNode::Nil } => {
            let mut arrexp_1: metamodelica::Ref<DAE::Exp>;
            let mut call: metamodelica::Ref<DAE::Exp>;
            let mut tp: metamodelica::Ref<DAE::Type>;
            let mut ty: metamodelica::Ref<DAE::Type>;
            let mut elt_ty: metamodelica::Ref<DAE::Type>;
            let mut c: DAE::Const;
            let (__pa0, __pa1, __pa2, __pa3) = ::match_deref::match_deref! { match &(elabExpInExpression(cache, env, arrexp.clone(), r#impl, true, prefix, info)?) {
                (__pa0, __pa1, DAE::Properties::PROP { type_: __pa2, constFlag: __pa3 }) => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone()),
                _ => return Err("pattern mismatch"),
            } };
            cache = metamodelica::Own::own(__pa0);
            arrexp_1 = metamodelica::Own::own(__pa1);
            ty = metamodelica::Own::own(__pa2);
            c = metamodelica::Own::own(__pa3);
            let true = (Types::isArray(&ty)) else { return Err("pattern mismatch") };
            arrexp_1 = Expression::matrixToArray(arrexp_1)?;
            elt_ty = Types::arrayElementType(&ty);
            tp = Types::simplifyType(elt_ty.clone())?;
            let false = (Types::isString(&tp)) else { return Err("pattern mismatch") };
            call = Expression::makePureBuiltinCall(inFnName, list![arrexp_1], tp);
            (call, DAE::Properties::PROP { type_: elt_ty, constFlag: c })
        },
        Deref @ metamodelica::ListNode::Cons { head: s1, tail: Deref @ metamodelica::ListNode::Cons { head: s2, tail: Deref @ metamodelica::ListNode::Nil } } => {
            let mut s1_1: metamodelica::Ref<DAE::Exp>;
            let mut s2_1: metamodelica::Ref<DAE::Exp>;
            let mut call: metamodelica::Ref<DAE::Exp>;
            let mut tp: metamodelica::Ref<DAE::Type>;
            let mut ty: metamodelica::Ref<DAE::Type>;
            let mut ty1: metamodelica::Ref<DAE::Type>;
            let mut ty2: metamodelica::Ref<DAE::Type>;
            let mut c: DAE::Const;
            let mut c1: DAE::Const;
            let mut c2: DAE::Const;
            let (__pa0, __pa1, __pa2, __pa3) = ::match_deref::match_deref! { match &(elabExpInExpression(cache, env.clone(), s1.clone(), r#impl, true, prefix.clone(), info.clone())?) {
                (__pa0, __pa1, DAE::Properties::PROP { type_: __pa2, constFlag: __pa3 }) => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone()),
                _ => return Err("pattern mismatch"),
            } };
            cache = metamodelica::Own::own(__pa0);
            s1_1 = metamodelica::Own::own(__pa1);
            ty1 = metamodelica::Own::own(__pa2);
            c1 = metamodelica::Own::own(__pa3);
            let (__pa4, __pa5, __pa6, __pa7) = ::match_deref::match_deref! { match &(elabExpInExpression(cache, env, s2.clone(), r#impl, true, prefix, info)?) {
                (__pa4, __pa5, DAE::Properties::PROP { type_: __pa6, constFlag: __pa7 }) => (__pa4.clone(), __pa5.clone(), __pa6.clone(), __pa7.clone()),
                _ => return Err("pattern mismatch"),
            } };
            cache = metamodelica::Own::own(__pa4);
            s2_1 = metamodelica::Own::own(__pa5);
            ty2 = metamodelica::Own::own(__pa6);
            c2 = metamodelica::Own::own(__pa7);
            let (__pa8, __pa9, __pa10) = ::match_deref::match_deref! { match &(Types::checkTypeCompat(s1_1, ty1, s2_1, &ty2, false)?) {
                (__pa8, __pa9, __pa10, true) => (__pa8.clone(), __pa9.clone(), __pa10.clone()),
                _ => return Err("pattern mismatch"),
            } };
            s1_1 = metamodelica::Own::own(__pa8);
            s2_1 = metamodelica::Own::own(__pa9);
            ty = metamodelica::Own::own(__pa10);
            c = Types::constAnd(c1, c2);
            tp = Types::simplifyType(ty.clone())?;
            let false = (Types::isString(&tp)) else { return Err("pattern mismatch") };
            call = Expression::makePureBuiltinCall(inFnName, list![s1_1, s2_1], tp);
            (call, DAE::Properties::PROP { type_: ty, constFlag: c })
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok((cache, outExp, outProperties))
}

fn elabBuiltinClock(
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
    mut args: &metamodelica::List<metamodelica::Ref<Absyn::Exp>>,
    mut nargs: &metamodelica::List<metamodelica::Ref<Absyn::NamedArg>>,
    mut inBoolean: bool,
    mut inPrefix: DAE::Prefix,
    mut info: SourceInfo,
) -> Result<(FCore::Cache, metamodelica::Ref<DAE::Exp>, DAE::Properties)> {
    let mut outCache: FCore::Cache;
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    let mut outProperties: DAE::Properties;
    (outCache, outExp, outProperties) = ({
        let mut prop: DAE::Properties = DAE::Properties::PROP {
            type_: DAE::T_CLOCK_DEFAULT().clone(),
            constFlag: openmodelica_frontend_types::DAE::Const::C_VAR,
        };
        'mc: {
            let __mc_input = (inCache, inEnv, &**args, &**nargs, inBoolean, inPrefix);
            if let Ok(__v) = (|| -> Result<_> {
                ::match_deref::match_deref! { match &__mc_input {
                    (cache, _, Deref @ metamodelica::ListNode::Nil, Deref @ metamodelica::ListNode::Nil, _, _) => {
                        let mut call: metamodelica::Ref<DAE::Exp>;
                        call = metamodelica::Ref::new(DAE::Exp::CLKCONST { clk: openmodelica_frontend_types::DAE::ClockKind::interned_INFERRED_CLOCK() });
                        Ok((cache.clone(), call.clone(), DAE::Properties::PROP { type_: DAE::T_CLOCK_DEFAULT().clone(), constFlag: openmodelica_frontend_types::DAE::Const::C_VAR }))
                    }
                    _ => return Err("nomatch"),
                }}
            })() {
                break 'mc __v;
            }
            if let Ok(__v) = (|| -> Result<_> {
                ::match_deref::match_deref! { match &__mc_input {
                    (cache, env, Deref @ metamodelica::ListNode::Cons { head: aintervalCounter, tail: Deref @ metamodelica::ListNode::Nil }, Deref @ metamodelica::ListNode::Nil, r#impl, pre) => {
                        let mut call: metamodelica::Ref<DAE::Exp>;
                        let mut intervalCounter: metamodelica::Ref<DAE::Exp>;
                        let mut ty1: metamodelica::Ref<DAE::Type>;
                        let mut prop1: DAE::Properties;
                        let mut cache = (*cache).clone();
                        (cache, intervalCounter, prop1) = elabExpInExpression(cache.clone(), env.clone(), aintervalCounter.clone(), r#impl.clone(), true, pre.clone(), info.clone())?;
                        ty1 = Types::arrayElementType(&(Types::getPropType(&prop1)));
                        (intervalCounter, _) = Types::matchType(intervalCounter.clone(), ty1.clone(), DAE::T_INTEGER_DEFAULT().clone(), true)?;
                        call = metamodelica::Ref::new(DAE::Exp::CLKCONST { clk: metamodelica::Ref::new(DAE::ClockKind::RATIONAL_CLOCK { intervalCounter: intervalCounter.clone(), resolution: metamodelica::Ref::new(DAE::Exp::ICONST { integer: 1 }) }) });
                        Ok((cache.clone(), call.clone(), prop.clone()))
                    }
                    _ => return Err("nomatch"),
                }}
            })() {
                break 'mc __v;
            }
            if let Ok(__v) = (|| -> Result<_> {
                ::match_deref::match_deref! { match &__mc_input {
                    (cache, env, Deref @ metamodelica::ListNode::Cons { head: aintervalCounter, tail: Deref @ metamodelica::ListNode::Cons { head: aresolution, tail: Deref @ metamodelica::ListNode::Nil } }, Deref @ metamodelica::ListNode::Nil, r#impl, pre) => {
                        let mut call: metamodelica::Ref<DAE::Exp>;
                        let mut intervalCounter: metamodelica::Ref<DAE::Exp>;
                        let mut resolution: metamodelica::Ref<DAE::Exp>;
                        let mut ty1: metamodelica::Ref<DAE::Type>;
                        let mut ty2: metamodelica::Ref<DAE::Type>;
                        let mut prop1: DAE::Properties;
                        let mut prop2: DAE::Properties;
                        let mut val: metamodelica::Ref<Values::Value>;
                        let mut cache = (*cache).clone();
                        (cache, intervalCounter, prop1) = elabExpInExpression(cache.clone(), env.clone(), aintervalCounter.clone(), r#impl.clone(), true, pre.clone(), info.clone())?;
                        (cache, resolution, prop2) = elabExpInExpression(cache.clone(), env.clone(), aresolution.clone(), r#impl.clone(), true, pre.clone(), info.clone())?;
                        ty1 = Types::arrayElementType(&(Types::getPropType(&prop1)));
                        ty2 = Types::arrayElementType(&(Types::getPropType(&prop2)));
                        (intervalCounter, _) = Types::matchType(intervalCounter.clone(), ty1.clone(), DAE::T_INTEGER_DEFAULT().clone(), true)?;
                        (resolution, _) = Types::matchType(resolution.clone(), ty2.clone(), DAE::T_INTEGER_DEFAULT().clone(), true)?;
                        (cache, val) = Ceval::ceval(cache.clone(), env.clone(), resolution.clone(), false, Absyn::Msg::MSG { info: info.clone() }, 0)?;
                        Error::assertionOrAddSourceMessage(ValuesUtil::valueInteger(&val)? >= 1, &(Error::WRONG_VALUE_OF_ARG.clone()), list![literal!("Clock"), literal!("resolution"), ValuesDump::valString(&val)?, literal!(">= 1")], &info)?;
                        resolution = ValuesUtil::valueExp(val.clone(), Some(resolution.clone()))?;
                        call = metamodelica::Ref::new(DAE::Exp::CLKCONST { clk: metamodelica::Ref::new(DAE::ClockKind::RATIONAL_CLOCK { intervalCounter: intervalCounter.clone(), resolution: resolution.clone() }) });
                        Ok((cache.clone(), call.clone(), prop.clone()))
                    }
                    _ => return Err("nomatch"),
                }}
            })() {
                break 'mc __v;
            }
            if let Ok(__v) = (|| -> Result<_> {
                ::match_deref::match_deref! { match &__mc_input {
                    (cache, env, Deref @ metamodelica::ListNode::Cons { head: ainterval, tail: Deref @ metamodelica::ListNode::Nil }, Deref @ metamodelica::ListNode::Nil, r#impl, pre) => {
                        let mut call: metamodelica::Ref<DAE::Exp>;
                        let mut interval: metamodelica::Ref<DAE::Exp>;
                        let mut ty1: metamodelica::Ref<DAE::Type>;
                        let mut prop1: DAE::Properties;
                        let mut cache = (*cache).clone();
                        (cache, interval, prop1) = elabExpInExpression(cache.clone(), env.clone(), ainterval.clone(), r#impl.clone(), true, pre.clone(), info.clone())?;
                        ty1 = Types::arrayElementType(&(Types::getPropType(&prop1)));
                        (interval, _) = Types::matchType(interval.clone(), ty1.clone(), DAE::T_REAL_DEFAULT().clone(), true)?;
                        call = metamodelica::Ref::new(DAE::Exp::CLKCONST { clk: metamodelica::Ref::new(DAE::ClockKind::REAL_CLOCK { interval: interval.clone() }) });
                        Ok((cache.clone(), call.clone(), prop.clone()))
                    }
                    _ => return Err("nomatch"),
                }}
            })() {
                break 'mc __v;
            }
            if let Ok(__v) = (|| -> Result<_> {
                ::match_deref::match_deref! { match &__mc_input {
                    (cache, env, Deref @ metamodelica::ListNode::Cons { head: acondition, tail: Deref @ metamodelica::ListNode::Nil }, Deref @ metamodelica::ListNode::Nil, r#impl, pre) => {
                        let mut call: metamodelica::Ref<DAE::Exp>;
                        let mut condition: metamodelica::Ref<DAE::Exp>;
                        let mut ty1: metamodelica::Ref<DAE::Type>;
                        let mut prop1: DAE::Properties;
                        let mut cache = (*cache).clone();
                        (cache, condition, prop1) = elabExpInExpression(cache.clone(), env.clone(), acondition.clone(), r#impl.clone(), true, pre.clone(), info.clone())?;
                        ty1 = Types::arrayElementType(&(Types::getPropType(&prop1)));
                        (condition, _) = Types::matchType(condition.clone(), ty1.clone(), DAE::T_BOOL_DEFAULT().clone(), true)?;
                        call = metamodelica::Ref::new(DAE::Exp::CLKCONST { clk: metamodelica::Ref::new(DAE::ClockKind::EVENT_CLOCK { condition: condition.clone(), startInterval: metamodelica::Ref::new(DAE::Exp::RCONST { real: metamodelica::OrderedFloat(0.0_f64) }) }) });
                        Ok((cache.clone(), call.clone(), prop.clone()))
                    }
                    _ => return Err("nomatch"),
                }}
            })() {
                break 'mc __v;
            }
            if let Ok(__v) = (|| -> Result<_> {
                ::match_deref::match_deref! { match &__mc_input {
                    (cache, env, Deref @ metamodelica::ListNode::Cons { head: acondition, tail: Deref @ metamodelica::ListNode::Cons { head: astartInterval, tail: Deref @ metamodelica::ListNode::Nil } }, Deref @ metamodelica::ListNode::Nil, r#impl, pre) => {
                        let mut call: metamodelica::Ref<DAE::Exp>;
                        let mut condition: metamodelica::Ref<DAE::Exp>;
                        let mut startInterval: metamodelica::Ref<DAE::Exp>;
                        let mut ty1: metamodelica::Ref<DAE::Type>;
                        let mut ty2: metamodelica::Ref<DAE::Type>;
                        let mut prop1: DAE::Properties;
                        let mut prop2: DAE::Properties;
                        let mut cache = (*cache).clone();
                        (cache, condition, prop1) = elabExpInExpression(cache.clone(), env.clone(), acondition.clone(), r#impl.clone(), true, pre.clone(), info.clone())?;
                        (cache, startInterval, prop2) = elabExpInExpression(cache.clone(), env.clone(), astartInterval.clone(), r#impl.clone(), true, pre.clone(), info.clone())?;
                        ty1 = Types::arrayElementType(&(Types::getPropType(&prop1)));
                        ty2 = Types::arrayElementType(&(Types::getPropType(&prop2)));
                        (condition, _) = Types::matchType(condition.clone(), ty1.clone(), DAE::T_BOOL_DEFAULT().clone(), true)?;
                        (startInterval, _) = Types::matchType(startInterval.clone(), ty2.clone(), DAE::T_REAL_DEFAULT().clone(), true)?;
                        call = metamodelica::Ref::new(DAE::Exp::CLKCONST { clk: metamodelica::Ref::new(DAE::ClockKind::EVENT_CLOCK { condition: condition.clone(), startInterval: startInterval.clone() }) });
                        Ok((cache.clone(), call.clone(), prop.clone()))
                    }
                    _ => return Err("nomatch"),
                }}
            })() {
                break 'mc __v;
            }
            if let Ok(__v) = (|| -> Result<_> {
                ::match_deref::match_deref! { match &__mc_input {
                    (cache, env, Deref @ metamodelica::ListNode::Cons { head: ac, tail: Deref @ metamodelica::ListNode::Cons { head: asolverMethod, tail: Deref @ metamodelica::ListNode::Nil } }, Deref @ metamodelica::ListNode::Nil, r#impl, pre) => {
                        let mut call: metamodelica::Ref<DAE::Exp>;
                        let mut c: metamodelica::Ref<DAE::Exp>;
                        let mut solverMethod: metamodelica::Ref<DAE::Exp>;
                        let mut ty1: metamodelica::Ref<DAE::Type>;
                        let mut ty2: metamodelica::Ref<DAE::Type>;
                        let mut prop1: DAE::Properties;
                        let mut prop2: DAE::Properties;
                        let mut val: metamodelica::Ref<Values::Value>;
                        let mut cache = (*cache).clone();
                        (cache, c, prop1) = elabExpInExpression(cache.clone(), env.clone(), ac.clone(), r#impl.clone(), true, pre.clone(), info.clone())?;
                        (cache, solverMethod, prop2) = elabExpInExpression(cache.clone(), env.clone(), asolverMethod.clone(), r#impl.clone(), true, pre.clone(), info.clone())?;
                        ty1 = Types::arrayElementType(&(Types::getPropType(&prop1)));
                        ty2 = Types::arrayElementType(&(Types::getPropType(&prop2)));
                        (c, _) = Types::matchType(c.clone(), ty1.clone(), DAE::T_CLOCK_DEFAULT().clone(), true)?;
                        (solverMethod, _) = Types::matchType(solverMethod.clone(), ty2.clone(), DAE::T_STRING_DEFAULT().clone(), true)?;
                        (cache, val) = Ceval::ceval(cache.clone(), env.clone(), solverMethod.clone(), false, Absyn::Msg::MSG { info: info.clone() }, 0)?;
                        solverMethod = ValuesUtil::valueExp(val.clone(), Some(solverMethod.clone()))?;
                        call = metamodelica::Ref::new(DAE::Exp::CLKCONST { clk: metamodelica::Ref::new(DAE::ClockKind::SOLVER_CLOCK { c: c.clone(), solverMethod: solverMethod.clone() }) });
                        Ok((cache.clone(), call.clone(), prop.clone()))
                    }
                    _ => return Err("nomatch"),
                }}
            })() {
                break 'mc __v;
            }
            if let Ok(__v) = (|| -> Result<_> {
                ::match_deref::match_deref! { match &__mc_input {
                    (cache, env, Deref @ metamodelica::ListNode::Cons { head: ac, tail: Deref @ metamodelica::ListNode::Nil }, Deref @ metamodelica::ListNode::Cons { head: Deref @ Absyn::NamedArg { argName: Deref @ "solverMethod", argValue: asolverMethod }, tail: Deref @ metamodelica::ListNode::Nil }, r#impl, pre) => {
                        let mut call: metamodelica::Ref<DAE::Exp>;
                        let mut c: metamodelica::Ref<DAE::Exp>;
                        let mut solverMethod: metamodelica::Ref<DAE::Exp>;
                        let mut ty1: metamodelica::Ref<DAE::Type>;
                        let mut ty2: metamodelica::Ref<DAE::Type>;
                        let mut prop1: DAE::Properties;
                        let mut prop2: DAE::Properties;
                        let mut val: metamodelica::Ref<Values::Value>;
                        let mut cache = (*cache).clone();
                        (cache, c, prop1) = elabExpInExpression(cache.clone(), env.clone(), ac.clone(), r#impl.clone(), true, pre.clone(), info.clone())?;
                        (cache, solverMethod, prop2) = elabExpInExpression(cache.clone(), env.clone(), asolverMethod.clone(), r#impl.clone(), true, pre.clone(), info.clone())?;
                        ty1 = Types::arrayElementType(&(Types::getPropType(&prop1)));
                        ty2 = Types::arrayElementType(&(Types::getPropType(&prop2)));
                        (c, _) = Types::matchType(c.clone(), ty1.clone(), DAE::T_CLOCK_DEFAULT().clone(), true)?;
                        (solverMethod, _) = Types::matchType(solverMethod.clone(), ty2.clone(), DAE::T_STRING_DEFAULT().clone(), true)?;
                        (cache, val) = Ceval::ceval(cache.clone(), env.clone(), solverMethod.clone(), false, Absyn::Msg::MSG { info: info.clone() }, 0)?;
                        solverMethod = ValuesUtil::valueExp(val.clone(), Some(solverMethod.clone()))?;
                        call = metamodelica::Ref::new(DAE::Exp::CLKCONST { clk: metamodelica::Ref::new(DAE::ClockKind::SOLVER_CLOCK { c: c.clone(), solverMethod: solverMethod.clone() }) });
                        Ok((cache.clone(), call.clone(), prop.clone()))
                    }
                    _ => return Err("nomatch"),
                }}
            })() {
                break 'mc __v;
            }
            return Err("matchcontinue: no arm matched");
        }
    });
    Ok((outCache, outExp, outProperties))
}

fn elabBuiltinHold(
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
    mut args: &metamodelica::List<metamodelica::Ref<Absyn::Exp>>,
    mut nargs: &metamodelica::List<metamodelica::Ref<Absyn::NamedArg>>,
    mut inBoolean: bool,
    mut inPrefix: DAE::Prefix,
    mut info: SourceInfo,
) -> Result<(FCore::Cache, metamodelica::Ref<DAE::Exp>, DAE::Properties)> {
    let mut outCache: FCore::Cache;
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    let mut outProperties: DAE::Properties;
    (outCache, outExp, outProperties) = (::match_deref::match_deref! { match (args, nargs) {
        (Deref @ metamodelica::ListNode::Cons { head: au, tail: Deref @ metamodelica::ListNode::Nil }, Deref @ metamodelica::ListNode::Nil) => {
            let mut cache = inCache;
            let mut env = inEnv;
            let mut r#impl = inBoolean;
            let mut pre = inPrefix;
            let mut call: metamodelica::Ref<DAE::Exp>;
            let mut ty1: metamodelica::Ref<DAE::Type>;
            let mut ty: metamodelica::Ref<DAE::Type>;
            let mut prop1: DAE::Properties;
            let mut prop: DAE::Properties;
            (cache, _, prop1) = elabExpInExpression(cache, env.clone(), au.clone(), r#impl, true, pre.clone(), info.clone())?;
            ty1 = Types::arrayElementType(&(Types::getPropType(&prop1)));
            ty = metamodelica::Ref::new(DAE::Type::T_FUNCTION { funcArg: list![metamodelica::Ref::new(DAE::FuncArg { name: literal!("u"), ty: ty1.clone(), r#const: openmodelica_frontend_types::DAE::Const::C_VAR, par: openmodelica_frontend_types::DAE::VarParallelism::NON_PARALLEL, defaultBinding: None })], funcResultType: ty1, functionAttributes: DAE::FUNCTION_ATTRIBUTES_BUILTIN_IMPURE.clone(), path: metamodelica::Ref::new(Absyn::Path::IDENT { name: literal!("hold") }) });
            let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(elabCallArgs3(cache, env, list![ty], metamodelica::Ref::new(Absyn::Path::IDENT { name: literal!("hold") }), args, nargs, &(metamodelica::nil()), r#impl, pre, info)?) {
                (__pa0, Some((__pa1, __pa2))) => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
                _ => return Err("pattern mismatch"),
            } };
            cache = metamodelica::Own::own(__pa0);
            call = metamodelica::Own::own(__pa1);
            prop = metamodelica::Own::own(__pa2);
            (cache, call, prop)
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok((outCache, outExp, outProperties))
}

fn elabBuiltinSample(
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
    mut args: &metamodelica::List<metamodelica::Ref<Absyn::Exp>>,
    mut nargs: &metamodelica::List<metamodelica::Ref<Absyn::NamedArg>>,
    mut inBoolean: bool,
    mut inPrefix: DAE::Prefix,
    mut info: SourceInfo,
) -> Result<(FCore::Cache, metamodelica::Ref<DAE::Exp>, DAE::Properties)> {
    let mut outCache: FCore::Cache;
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    let mut outProperties: DAE::Properties;
    (outCache, outExp, outProperties) = 'mc: {
        let __mc_input = (inCache, inEnv, &**args, &**nargs, inBoolean, inPrefix);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, Deref @ metamodelica::ListNode::Cons { head: astart, tail: Deref @ metamodelica::ListNode::Cons { head: ainterval, tail: Deref @ metamodelica::ListNode::Nil } }, Deref @ metamodelica::ListNode::Nil, r#impl, pre) => {
                    let mut call: metamodelica::Ref<DAE::Exp>;
                    let mut start: metamodelica::Ref<DAE::Exp>;
                    let mut interval: metamodelica::Ref<DAE::Exp>;
                    let mut ty1: metamodelica::Ref<DAE::Type>;
                    let mut ty2: metamodelica::Ref<DAE::Type>;
                    let mut ty: metamodelica::Ref<DAE::Type>;
                    let mut prop1: DAE::Properties;
                    let mut prop2: DAE::Properties;
                    let mut prop: DAE::Properties;
                    let mut cache = (*cache).clone();
                    (cache, start, prop1) = elabExpInExpression(cache.clone(), env.clone(), astart.clone(), r#impl.clone(), true, pre.clone(), info.clone())?;
                    (cache, interval, prop2) = elabExpInExpression(cache.clone(), env.clone(), ainterval.clone(), r#impl.clone(), true, pre.clone(), info.clone())?;
                    ty1 = Types::getPropType(&prop1);
                    ty2 = Types::getPropType(&prop2);
                    (start, _) = Types::matchType(start.clone(), ty1.clone(), DAE::T_REAL_DEFAULT().clone(), true)?;
                    (interval, _) = Types::matchType(interval.clone(), ty2.clone(), DAE::T_REAL_DEFAULT().clone(), true)?;
                    ty = metamodelica::Ref::new(DAE::Type::T_FUNCTION { funcArg: list![metamodelica::Ref::new(DAE::FuncArg { name: literal!("start"), ty: DAE::T_REAL_DEFAULT().clone(), r#const: openmodelica_frontend_types::DAE::Const::C_PARAM, par: openmodelica_frontend_types::DAE::VarParallelism::NON_PARALLEL, defaultBinding: None }), metamodelica::Ref::new(DAE::FuncArg { name: literal!("interval"), ty: DAE::T_REAL_DEFAULT().clone(), r#const: openmodelica_frontend_types::DAE::Const::C_PARAM, par: openmodelica_frontend_types::DAE::VarParallelism::NON_PARALLEL, defaultBinding: None })], funcResultType: DAE::T_BOOL_DEFAULT().clone(), functionAttributes: DAE::FUNCTION_ATTRIBUTES_BUILTIN_IMPURE.clone(), path: metamodelica::Ref::new(Absyn::Path::IDENT { name: literal!("sample") }) });
                    let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(elabCallArgs3(cache.clone(), env.clone(), list![ty.clone()], metamodelica::Ref::new(Absyn::Path::IDENT { name: literal!("sample") }), args, nargs, &(metamodelica::nil()), r#impl.clone(), pre.clone(), info.clone())?) {
                        (__pa0, Some((__pa1, __pa2))) => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    cache = metamodelica::Own::own(__pa0);
                    call = metamodelica::Own::own(__pa1);
                    prop = metamodelica::Own::own(__pa2);
                    Ok((cache.clone(), call.clone(), prop.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, Deref @ metamodelica::ListNode::Cons { head: au, tail: Deref @ metamodelica::ListNode::Cons { head: ac, tail: Deref @ metamodelica::ListNode::Nil } }, Deref @ metamodelica::ListNode::Nil, r#impl, pre) => {
                    let mut call: metamodelica::Ref<DAE::Exp>;
                    let mut c: metamodelica::Ref<DAE::Exp>;
                    let mut ty1: metamodelica::Ref<DAE::Type>;
                    let mut ty2: metamodelica::Ref<DAE::Type>;
                    let mut ty: metamodelica::Ref<DAE::Type>;
                    let mut prop1: DAE::Properties;
                    let mut prop2: DAE::Properties;
                    let mut prop: DAE::Properties;
                    let mut variability: DAE::Const;
                    let mut cache = (*cache).clone();
                    (cache, _, prop1) = elabExpInExpression(cache.clone(), env.clone(), au.clone(), r#impl.clone(), true, pre.clone(), info.clone())?;
                    (cache, c, prop2) = elabExpInExpression(cache.clone(), env.clone(), ac.clone(), r#impl.clone(), true, pre.clone(), info.clone())?;
                    ty1 = Types::arrayElementType(&(Types::getPropType(&prop1)));
                    ty2 = Types::arrayElementType(&(Types::getPropType(&prop2)));
                    variability = Types::getPropConst(prop1.clone())?;
                    (c, _) = Types::matchType(c.clone(), ty2.clone(), DAE::T_CLOCK_DEFAULT().clone(), true)?;
                    ty = metamodelica::Ref::new(DAE::Type::T_FUNCTION { funcArg: list![metamodelica::Ref::new(DAE::FuncArg { name: literal!("u"), ty: ty1.clone(), r#const: variability, par: openmodelica_frontend_types::DAE::VarParallelism::NON_PARALLEL, defaultBinding: None }), metamodelica::Ref::new(DAE::FuncArg { name: literal!("c"), ty: ty2.clone(), r#const: openmodelica_frontend_types::DAE::Const::C_VAR, par: openmodelica_frontend_types::DAE::VarParallelism::NON_PARALLEL, defaultBinding: Some(metamodelica::Ref::new(DAE::Exp::CLKCONST { clk: openmodelica_frontend_types::DAE::ClockKind::interned_INFERRED_CLOCK() })) })], funcResultType: ty1.clone(), functionAttributes: DAE::FUNCTION_ATTRIBUTES_BUILTIN_IMPURE.clone(), path: metamodelica::Ref::new(Absyn::Path::IDENT { name: literal!("sample") }) });
                    let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(elabCallArgs3(cache.clone(), env.clone(), list![ty.clone()], metamodelica::Ref::new(Absyn::Path::IDENT { name: literal!("sample") }), args, nargs, &(metamodelica::nil()), r#impl.clone(), pre.clone(), info.clone())?) {
                        (__pa0, Some((__pa1, __pa2))) => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    cache = metamodelica::Own::own(__pa0);
                    call = metamodelica::Own::own(__pa1);
                    prop = metamodelica::Own::own(__pa2);
                    Ok((cache.clone(), call.clone(), prop.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, Deref @ metamodelica::ListNode::Cons { head: au, tail: Deref @ metamodelica::ListNode::Nil }, Deref @ metamodelica::ListNode::Nil, r#impl, pre) => {
                    let mut call: metamodelica::Ref<DAE::Exp>;
                    let mut ty1: metamodelica::Ref<DAE::Type>;
                    let mut ty: metamodelica::Ref<DAE::Type>;
                    let mut prop1: DAE::Properties;
                    let mut prop: DAE::Properties;
                    let mut variability: DAE::Const;
                    let mut cache = (*cache).clone();
                    (cache, _, prop1) = elabExpInExpression(cache.clone(), env.clone(), au.clone(), r#impl.clone(), true, pre.clone(), info.clone())?;
                    ty1 = Types::arrayElementType(&(Types::getPropType(&prop1)));
                    variability = Types::getPropConst(prop1.clone())?;
                    ty = metamodelica::Ref::new(DAE::Type::T_FUNCTION { funcArg: list![metamodelica::Ref::new(DAE::FuncArg { name: literal!("u"), ty: ty1.clone(), r#const: variability, par: openmodelica_frontend_types::DAE::VarParallelism::NON_PARALLEL, defaultBinding: None }), metamodelica::Ref::new(DAE::FuncArg { name: literal!("c"), ty: DAE::T_CLOCK_DEFAULT().clone(), r#const: openmodelica_frontend_types::DAE::Const::C_VAR, par: openmodelica_frontend_types::DAE::VarParallelism::NON_PARALLEL, defaultBinding: Some(metamodelica::Ref::new(DAE::Exp::CLKCONST { clk: openmodelica_frontend_types::DAE::ClockKind::interned_INFERRED_CLOCK() })) })], funcResultType: ty1.clone(), functionAttributes: DAE::FUNCTION_ATTRIBUTES_BUILTIN_IMPURE.clone(), path: metamodelica::Ref::new(Absyn::Path::IDENT { name: literal!("sample") }) });
                    let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(elabCallArgs3(cache.clone(), env.clone(), list![ty.clone()], metamodelica::Ref::new(Absyn::Path::IDENT { name: literal!("sample") }), args, nargs, &(metamodelica::nil()), r#impl.clone(), pre.clone(), info.clone())?) {
                        (__pa0, Some((__pa1, __pa2))) => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    cache = metamodelica::Own::own(__pa0);
                    call = metamodelica::Own::own(__pa1);
                    prop = metamodelica::Own::own(__pa2);
                    Ok((cache.clone(), call.clone(), prop.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((outCache, outExp, outProperties))
}

fn elabBuiltinShiftSample(
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
    mut args: &metamodelica::List<metamodelica::Ref<Absyn::Exp>>,
    mut nargs: &metamodelica::List<metamodelica::Ref<Absyn::NamedArg>>,
    mut inBoolean: bool,
    mut inPrefix: DAE::Prefix,
    mut info: SourceInfo,
) -> Result<(FCore::Cache, metamodelica::Ref<DAE::Exp>, DAE::Properties)> {
    let mut outCache: FCore::Cache;
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    let mut outProperties: DAE::Properties;
    (outCache, outExp, outProperties) = (::match_deref::match_deref! { match (args, nargs) {
        (Deref @ metamodelica::ListNode::Cons { head: au, tail: Deref @ metamodelica::ListNode::Cons { head: ashiftCounter, tail: Deref @ metamodelica::ListNode::Nil } }, Deref @ metamodelica::ListNode::Nil) => {
            let mut cache = inCache;
            let mut env = inEnv;
            let mut r#impl = inBoolean;
            let mut pre = inPrefix;
            let mut call: metamodelica::Ref<DAE::Exp>;
            let mut shiftCounter: metamodelica::Ref<DAE::Exp>;
            let mut ty1: metamodelica::Ref<DAE::Type>;
            let mut ty: metamodelica::Ref<DAE::Type>;
            let mut prop1: DAE::Properties;
            let mut prop2: DAE::Properties;
            let mut prop: DAE::Properties;
            let mut aresolution: metamodelica::Ref<Absyn::Exp>;
            let mut val: metamodelica::Ref<Values::Value>;
            let mut ashiftCounter = (*ashiftCounter).clone();
            (cache, _, prop1) = elabExpInExpression(cache, env.clone(), au.clone(), r#impl, true, pre.clone(), info.clone())?;
            (cache, shiftCounter, prop2) = elabExpInExpression(cache, env.clone(), ashiftCounter.clone(), r#impl, true, pre.clone(), info.clone())?;
            (shiftCounter, _) = Types::matchType(shiftCounter, Types::getPropType(&prop2), DAE::T_INTEGER_DEFAULT().clone(), true)?;
            (cache, val) = Ceval::ceval(cache, env.clone(), shiftCounter, false, Absyn::Msg::MSG { info: info.clone() }, 0)?;
            Error::assertionOrAddSourceMessage(ValuesUtil::valueInteger(&val)? >= 0, &(Error::WRONG_VALUE_OF_ARG.clone()), list![literal!("shiftSample"), literal!("shiftCounter"), ValuesDump::valString(&val)?, literal!(">= 0")], &info)?;
            ashiftCounter = metamodelica::Ref::new(Absyn::Exp::INTEGER { value: ValuesUtil::valueInteger(&val)? });
            aresolution = metamodelica::Ref::new(Absyn::Exp::INTEGER { value: 1 });
            ty1 = Types::arrayElementType(&(Types::getPropType(&prop1)));
            ty = metamodelica::Ref::new(DAE::Type::T_FUNCTION { funcArg: list![metamodelica::Ref::new(DAE::FuncArg { name: literal!("u"), ty: ty1.clone(), r#const: openmodelica_frontend_types::DAE::Const::C_VAR, par: openmodelica_frontend_types::DAE::VarParallelism::NON_PARALLEL, defaultBinding: None }), metamodelica::Ref::new(DAE::FuncArg { name: literal!("shiftCounter"), ty: DAE::T_INTEGER_DEFAULT().clone(), r#const: openmodelica_frontend_types::DAE::Const::C_PARAM, par: openmodelica_frontend_types::DAE::VarParallelism::NON_PARALLEL, defaultBinding: None }), metamodelica::Ref::new(DAE::FuncArg { name: literal!("resolution"), ty: DAE::T_INTEGER_DEFAULT().clone(), r#const: openmodelica_frontend_types::DAE::Const::C_PARAM, par: openmodelica_frontend_types::DAE::VarParallelism::NON_PARALLEL, defaultBinding: None })], funcResultType: ty1, functionAttributes: DAE::FUNCTION_ATTRIBUTES_BUILTIN_IMPURE.clone(), path: metamodelica::Ref::new(Absyn::Path::IDENT { name: literal!("shiftSample") }) });
            let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(elabCallArgs3(cache, env, list![ty], metamodelica::Ref::new(Absyn::Path::IDENT { name: literal!("shiftSample") }), &(list![au.clone(), ashiftCounter.clone(), aresolution]), nargs, &(metamodelica::nil()), r#impl, pre, info)?) {
                (__pa0, Some((__pa1, __pa2))) => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
                _ => return Err("pattern mismatch"),
            } };
            cache = metamodelica::Own::own(__pa0);
            call = metamodelica::Own::own(__pa1);
            prop = metamodelica::Own::own(__pa2);
            (cache, call, prop)
        },
        (Deref @ metamodelica::ListNode::Cons { head: au, tail: Deref @ metamodelica::ListNode::Cons { head: ashiftCounter, tail: Deref @ metamodelica::ListNode::Cons { head: aresolution, tail: Deref @ metamodelica::ListNode::Nil } } }, Deref @ metamodelica::ListNode::Nil) => {
            let mut cache = inCache;
            let mut env = inEnv;
            let mut r#impl = inBoolean;
            let mut pre = inPrefix;
            let mut call: metamodelica::Ref<DAE::Exp>;
            let mut shiftCounter: metamodelica::Ref<DAE::Exp>;
            let mut resolution: metamodelica::Ref<DAE::Exp>;
            let mut ty1: metamodelica::Ref<DAE::Type>;
            let mut ty: metamodelica::Ref<DAE::Type>;
            let mut prop1: DAE::Properties;
            let mut prop2: DAE::Properties;
            let mut prop3: DAE::Properties;
            let mut prop: DAE::Properties;
            let mut val: metamodelica::Ref<Values::Value>;
            let mut rval: metamodelica::Ref<Values::Value>;
            let mut ashiftCounter = (*ashiftCounter).clone();
            let mut aresolution = (*aresolution).clone();
            (cache, _, prop1) = elabExpInExpression(cache, env.clone(), au.clone(), r#impl, true, pre.clone(), info.clone())?;
            (cache, shiftCounter, prop2) = elabExpInExpression(cache, env.clone(), ashiftCounter.clone(), r#impl, true, pre.clone(), info.clone())?;
            (shiftCounter, _) = Types::matchType(shiftCounter, Types::getPropType(&prop2), DAE::T_INTEGER_DEFAULT().clone(), true)?;
            (cache, val) = Ceval::ceval(cache, env.clone(), shiftCounter, false, Absyn::Msg::MSG { info: info.clone() }, 0)?;
            Error::assertionOrAddSourceMessage(ValuesUtil::valueInteger(&val)? >= 0, &(Error::WRONG_VALUE_OF_ARG.clone()), list![literal!("shiftSample"), literal!("shiftCounter"), ValuesDump::valString(&val)?, literal!(">= 0")], &info)?;
            ashiftCounter = metamodelica::Ref::new(Absyn::Exp::INTEGER { value: ValuesUtil::valueInteger(&val)? });
            (cache, resolution, prop3) = elabExpInExpression(cache, env.clone(), aresolution.clone(), r#impl, true, pre.clone(), info.clone())?;
            (resolution, _) = Types::matchType(resolution, Types::getPropType(&prop3), DAE::T_INTEGER_DEFAULT().clone(), true)?;
            (cache, rval) = Ceval::ceval(cache, env.clone(), resolution, false, Absyn::Msg::MSG { info: info.clone() }, 0)?;
            Error::assertionOrAddSourceMessage(ValuesUtil::valueInteger(&rval)? >= 1, &(Error::WRONG_VALUE_OF_ARG.clone()), list![literal!("shiftSample"), literal!("resolution"), ValuesDump::valString(&rval)?, literal!(">= 1")], &info)?;
            aresolution = metamodelica::Ref::new(Absyn::Exp::INTEGER { value: ValuesUtil::valueInteger(&rval)? });
            ty1 = Types::arrayElementType(&(Types::getPropType(&prop1)));
            ty = metamodelica::Ref::new(DAE::Type::T_FUNCTION { funcArg: list![metamodelica::Ref::new(DAE::FuncArg { name: literal!("u"), ty: ty1.clone(), r#const: openmodelica_frontend_types::DAE::Const::C_VAR, par: openmodelica_frontend_types::DAE::VarParallelism::NON_PARALLEL, defaultBinding: None }), metamodelica::Ref::new(DAE::FuncArg { name: literal!("shiftCounter"), ty: DAE::T_INTEGER_DEFAULT().clone(), r#const: openmodelica_frontend_types::DAE::Const::C_PARAM, par: openmodelica_frontend_types::DAE::VarParallelism::NON_PARALLEL, defaultBinding: None }), metamodelica::Ref::new(DAE::FuncArg { name: literal!("resolution"), ty: DAE::T_INTEGER_DEFAULT().clone(), r#const: openmodelica_frontend_types::DAE::Const::C_PARAM, par: openmodelica_frontend_types::DAE::VarParallelism::NON_PARALLEL, defaultBinding: None })], funcResultType: ty1, functionAttributes: DAE::FUNCTION_ATTRIBUTES_BUILTIN_IMPURE.clone(), path: metamodelica::Ref::new(Absyn::Path::IDENT { name: literal!("shiftSample") }) });
            let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(elabCallArgs3(cache, env, list![ty], metamodelica::Ref::new(Absyn::Path::IDENT { name: literal!("shiftSample") }), &(list![au.clone(), ashiftCounter.clone(), aresolution.clone()]), nargs, &(metamodelica::nil()), r#impl, pre, info)?) {
                (__pa0, Some((__pa1, __pa2))) => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
                _ => return Err("pattern mismatch"),
            } };
            cache = metamodelica::Own::own(__pa0);
            call = metamodelica::Own::own(__pa1);
            prop = metamodelica::Own::own(__pa2);
            (cache, call, prop)
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok((outCache, outExp, outProperties))
}

fn elabBuiltinBackSample(
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
    mut args: &metamodelica::List<metamodelica::Ref<Absyn::Exp>>,
    mut nargs: &metamodelica::List<metamodelica::Ref<Absyn::NamedArg>>,
    mut inBoolean: bool,
    mut inPrefix: DAE::Prefix,
    mut info: SourceInfo,
) -> Result<(FCore::Cache, metamodelica::Ref<DAE::Exp>, DAE::Properties)> {
    let mut outCache: FCore::Cache;
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    let mut outProperties: DAE::Properties;
    (outCache, outExp, outProperties) = (::match_deref::match_deref! { match (args, nargs) {
        (Deref @ metamodelica::ListNode::Cons { head: au, tail: Deref @ metamodelica::ListNode::Cons { head: abackCounter, tail: Deref @ metamodelica::ListNode::Nil } }, Deref @ metamodelica::ListNode::Nil) => {
            let mut cache = inCache;
            let mut env = inEnv;
            let mut r#impl = inBoolean;
            let mut pre = inPrefix;
            let mut call: metamodelica::Ref<DAE::Exp>;
            let mut backCounter: metamodelica::Ref<DAE::Exp>;
            let mut ty1: metamodelica::Ref<DAE::Type>;
            let mut ty: metamodelica::Ref<DAE::Type>;
            let mut prop1: DAE::Properties;
            let mut prop2: DAE::Properties;
            let mut prop: DAE::Properties;
            let mut aresolution: metamodelica::Ref<Absyn::Exp>;
            let mut val: metamodelica::Ref<Values::Value>;
            let mut abackCounter = (*abackCounter).clone();
            (cache, _, prop1) = elabExpInExpression(cache, env.clone(), au.clone(), r#impl, true, pre.clone(), info.clone())?;
            (cache, backCounter, prop2) = elabExpInExpression(cache, env.clone(), abackCounter.clone(), r#impl, true, pre.clone(), info.clone())?;
            (backCounter, _) = Types::matchType(backCounter, Types::getPropType(&prop2), DAE::T_INTEGER_DEFAULT().clone(), true)?;
            (cache, val) = Ceval::ceval(cache, env.clone(), backCounter, false, Absyn::Msg::MSG { info: info.clone() }, 0)?;
            Error::assertionOrAddSourceMessage(ValuesUtil::valueInteger(&val)? >= 0, &(Error::WRONG_VALUE_OF_ARG.clone()), list![literal!("backSample"), literal!("backCounter"), ValuesDump::valString(&val)?, literal!(">= 0")], &info)?;
            abackCounter = metamodelica::Ref::new(Absyn::Exp::INTEGER { value: ValuesUtil::valueInteger(&val)? });
            aresolution = metamodelica::Ref::new(Absyn::Exp::INTEGER { value: 1 });
            ty1 = Types::arrayElementType(&(Types::getPropType(&prop1)));
            ty = metamodelica::Ref::new(DAE::Type::T_FUNCTION { funcArg: list![metamodelica::Ref::new(DAE::FuncArg { name: literal!("u"), ty: ty1.clone(), r#const: openmodelica_frontend_types::DAE::Const::C_VAR, par: openmodelica_frontend_types::DAE::VarParallelism::NON_PARALLEL, defaultBinding: None }), metamodelica::Ref::new(DAE::FuncArg { name: literal!("backCounter"), ty: DAE::T_INTEGER_DEFAULT().clone(), r#const: openmodelica_frontend_types::DAE::Const::C_PARAM, par: openmodelica_frontend_types::DAE::VarParallelism::NON_PARALLEL, defaultBinding: None }), metamodelica::Ref::new(DAE::FuncArg { name: literal!("resolution"), ty: DAE::T_INTEGER_DEFAULT().clone(), r#const: openmodelica_frontend_types::DAE::Const::C_PARAM, par: openmodelica_frontend_types::DAE::VarParallelism::NON_PARALLEL, defaultBinding: None })], funcResultType: ty1, functionAttributes: DAE::FUNCTION_ATTRIBUTES_BUILTIN_IMPURE.clone(), path: metamodelica::Ref::new(Absyn::Path::IDENT { name: literal!("backSample") }) });
            let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(elabCallArgs3(cache, env, list![ty], metamodelica::Ref::new(Absyn::Path::IDENT { name: literal!("backSample") }), &(list![au.clone(), abackCounter.clone(), aresolution]), nargs, &(metamodelica::nil()), r#impl, pre, info)?) {
                (__pa0, Some((__pa1, __pa2))) => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
                _ => return Err("pattern mismatch"),
            } };
            cache = metamodelica::Own::own(__pa0);
            call = metamodelica::Own::own(__pa1);
            prop = metamodelica::Own::own(__pa2);
            (cache, call, prop)
        },
        (Deref @ metamodelica::ListNode::Cons { head: au, tail: Deref @ metamodelica::ListNode::Cons { head: abackCounter, tail: Deref @ metamodelica::ListNode::Cons { head: aresolution, tail: Deref @ metamodelica::ListNode::Nil } } }, Deref @ metamodelica::ListNode::Nil) => {
            let mut cache = inCache;
            let mut env = inEnv;
            let mut r#impl = inBoolean;
            let mut pre = inPrefix;
            let mut call: metamodelica::Ref<DAE::Exp>;
            let mut backCounter: metamodelica::Ref<DAE::Exp>;
            let mut resolution: metamodelica::Ref<DAE::Exp>;
            let mut ty1: metamodelica::Ref<DAE::Type>;
            let mut ty: metamodelica::Ref<DAE::Type>;
            let mut prop1: DAE::Properties;
            let mut prop2: DAE::Properties;
            let mut prop3: DAE::Properties;
            let mut prop: DAE::Properties;
            let mut val: metamodelica::Ref<Values::Value>;
            let mut rval: metamodelica::Ref<Values::Value>;
            let mut abackCounter = (*abackCounter).clone();
            let mut aresolution = (*aresolution).clone();
            (cache, _, prop1) = elabExpInExpression(cache, env.clone(), au.clone(), r#impl, true, pre.clone(), info.clone())?;
            (cache, backCounter, prop2) = elabExpInExpression(cache, env.clone(), abackCounter.clone(), r#impl, true, pre.clone(), info.clone())?;
            (backCounter, _) = Types::matchType(backCounter, Types::getPropType(&prop2), DAE::T_INTEGER_DEFAULT().clone(), true)?;
            (cache, val) = Ceval::ceval(cache, env.clone(), backCounter, false, Absyn::Msg::MSG { info: info.clone() }, 0)?;
            Error::assertionOrAddSourceMessage(ValuesUtil::valueInteger(&val)? >= 0, &(Error::WRONG_VALUE_OF_ARG.clone()), list![literal!("backSample"), literal!("backCounter"), ValuesDump::valString(&val)?, literal!(">= 0")], &info)?;
            abackCounter = metamodelica::Ref::new(Absyn::Exp::INTEGER { value: ValuesUtil::valueInteger(&val)? });
            (cache, resolution, prop3) = elabExpInExpression(cache, env.clone(), aresolution.clone(), r#impl, true, pre.clone(), info.clone())?;
            (resolution, _) = Types::matchType(resolution, Types::getPropType(&prop3), DAE::T_INTEGER_DEFAULT().clone(), true)?;
            (cache, rval) = Ceval::ceval(cache, env.clone(), resolution, false, Absyn::Msg::MSG { info: info.clone() }, 0)?;
            Error::assertionOrAddSourceMessage(ValuesUtil::valueInteger(&rval)? >= 1, &(Error::WRONG_VALUE_OF_ARG.clone()), list![literal!("backSample"), literal!("resolution"), ValuesDump::valString(&rval)?, literal!(">= 1")], &info)?;
            aresolution = metamodelica::Ref::new(Absyn::Exp::INTEGER { value: ValuesUtil::valueInteger(&rval)? });
            ty1 = Types::arrayElementType(&(Types::getPropType(&prop1)));
            ty = metamodelica::Ref::new(DAE::Type::T_FUNCTION { funcArg: list![metamodelica::Ref::new(DAE::FuncArg { name: literal!("u"), ty: ty1.clone(), r#const: openmodelica_frontend_types::DAE::Const::C_VAR, par: openmodelica_frontend_types::DAE::VarParallelism::NON_PARALLEL, defaultBinding: None }), metamodelica::Ref::new(DAE::FuncArg { name: literal!("backCounter"), ty: DAE::T_INTEGER_DEFAULT().clone(), r#const: openmodelica_frontend_types::DAE::Const::C_PARAM, par: openmodelica_frontend_types::DAE::VarParallelism::NON_PARALLEL, defaultBinding: None }), metamodelica::Ref::new(DAE::FuncArg { name: literal!("resolution"), ty: DAE::T_INTEGER_DEFAULT().clone(), r#const: openmodelica_frontend_types::DAE::Const::C_PARAM, par: openmodelica_frontend_types::DAE::VarParallelism::NON_PARALLEL, defaultBinding: None })], funcResultType: ty1, functionAttributes: DAE::FUNCTION_ATTRIBUTES_BUILTIN_IMPURE.clone(), path: metamodelica::Ref::new(Absyn::Path::IDENT { name: literal!("backSample") }) });
            let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(elabCallArgs3(cache, env, list![ty], metamodelica::Ref::new(Absyn::Path::IDENT { name: literal!("backSample") }), &(list![au.clone(), abackCounter.clone(), aresolution.clone()]), nargs, &(metamodelica::nil()), r#impl, pre, info)?) {
                (__pa0, Some((__pa1, __pa2))) => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
                _ => return Err("pattern mismatch"),
            } };
            cache = metamodelica::Own::own(__pa0);
            call = metamodelica::Own::own(__pa1);
            prop = metamodelica::Own::own(__pa2);
            (cache, call, prop)
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok((outCache, outExp, outProperties))
}

fn elabBuiltinNoClock(
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
    mut args: &metamodelica::List<metamodelica::Ref<Absyn::Exp>>,
    mut nargs: &metamodelica::List<metamodelica::Ref<Absyn::NamedArg>>,
    mut inBoolean: bool,
    mut inPrefix: DAE::Prefix,
    mut info: SourceInfo,
) -> Result<(FCore::Cache, metamodelica::Ref<DAE::Exp>, DAE::Properties)> {
    let mut outCache: FCore::Cache;
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    let mut outProperties: DAE::Properties;
    (outCache, outExp, outProperties) = (::match_deref::match_deref! { match (args, nargs) {
        (Deref @ metamodelica::ListNode::Cons { head: au, tail: Deref @ metamodelica::ListNode::Nil }, Deref @ metamodelica::ListNode::Nil) => {
            let mut cache = inCache;
            let mut env = inEnv;
            let mut r#impl = inBoolean;
            let mut pre = inPrefix;
            let mut call: metamodelica::Ref<DAE::Exp>;
            let mut ty1: metamodelica::Ref<DAE::Type>;
            let mut ty: metamodelica::Ref<DAE::Type>;
            let mut prop1: DAE::Properties;
            let mut prop: DAE::Properties;
            (cache, _, prop1) = elabExpInExpression(cache, env.clone(), au.clone(), r#impl, true, pre.clone(), info.clone())?;
            ty1 = Types::arrayElementType(&(Types::getPropType(&prop1)));
            ty = metamodelica::Ref::new(DAE::Type::T_FUNCTION { funcArg: list![metamodelica::Ref::new(DAE::FuncArg { name: literal!("u"), ty: ty1.clone(), r#const: openmodelica_frontend_types::DAE::Const::C_VAR, par: openmodelica_frontend_types::DAE::VarParallelism::NON_PARALLEL, defaultBinding: None })], funcResultType: ty1, functionAttributes: DAE::FUNCTION_ATTRIBUTES_BUILTIN_IMPURE.clone(), path: metamodelica::Ref::new(Absyn::Path::IDENT { name: literal!("noClock") }) });
            let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(elabCallArgs3(cache, env, list![ty], metamodelica::Ref::new(Absyn::Path::IDENT { name: literal!("noClock") }), args, nargs, &(metamodelica::nil()), r#impl, pre, info)?) {
                (__pa0, Some((__pa1, __pa2))) => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
                _ => return Err("pattern mismatch"),
            } };
            cache = metamodelica::Own::own(__pa0);
            call = metamodelica::Own::own(__pa1);
            prop = metamodelica::Own::own(__pa2);
            (cache, call, prop)
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok((outCache, outExp, outProperties))
}

fn elabBuiltinFirstTick(
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
    mut args: &metamodelica::List<metamodelica::Ref<Absyn::Exp>>,
    mut nargs: &metamodelica::List<metamodelica::Ref<Absyn::NamedArg>>,
    mut inBoolean: bool,
    mut inPrefix: DAE::Prefix,
    mut info: SourceInfo,
) -> Result<(FCore::Cache, metamodelica::Ref<DAE::Exp>, DAE::Properties)> {
    let mut outCache: FCore::Cache;
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    let mut outProperties: DAE::Properties;
    (outCache, outExp, outProperties) = (::match_deref::match_deref! { match (args, nargs) {
        (Deref @ metamodelica::ListNode::Nil, Deref @ metamodelica::ListNode::Nil) => {
            let mut cache = inCache;
            let mut env = inEnv;
            let mut r#impl = inBoolean;
            let mut pre = inPrefix;
            let mut call: metamodelica::Ref<DAE::Exp>;
            let mut ty: metamodelica::Ref<DAE::Type>;
            let mut prop: DAE::Properties;
            ty = metamodelica::Ref::new(DAE::Type::T_FUNCTION { funcArg: metamodelica::nil(), funcResultType: DAE::T_BOOL_DEFAULT().clone(), functionAttributes: DAE::FUNCTION_ATTRIBUTES_BUILTIN_IMPURE.clone(), path: metamodelica::Ref::new(Absyn::Path::IDENT { name: literal!("firstTick") }) });
            let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(elabCallArgs3(cache, env, list![ty], metamodelica::Ref::new(Absyn::Path::IDENT { name: literal!("firstTick") }), args, nargs, &(metamodelica::nil()), r#impl, pre, info)?) {
                (__pa0, Some((__pa1, __pa2))) => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
                _ => return Err("pattern mismatch"),
            } };
            cache = metamodelica::Own::own(__pa0);
            call = metamodelica::Own::own(__pa1);
            prop = metamodelica::Own::own(__pa2);
            (cache, call, prop)
        },
        (Deref @ metamodelica::ListNode::Cons { head: au, tail: Deref @ metamodelica::ListNode::Nil }, Deref @ metamodelica::ListNode::Nil) => {
            let mut cache = inCache;
            let mut env = inEnv;
            let mut r#impl = inBoolean;
            let mut pre = inPrefix;
            let mut call: metamodelica::Ref<DAE::Exp>;
            let mut ty1: metamodelica::Ref<DAE::Type>;
            let mut ty: metamodelica::Ref<DAE::Type>;
            let mut prop1: DAE::Properties;
            let mut prop: DAE::Properties;
            (cache, _, prop1) = elabExpInExpression(cache, env.clone(), au.clone(), r#impl, true, pre.clone(), info.clone())?;
            ty1 = Types::arrayElementType(&(Types::getPropType(&prop1)));
            ty = metamodelica::Ref::new(DAE::Type::T_FUNCTION { funcArg: list![metamodelica::Ref::new(DAE::FuncArg { name: literal!("u"), ty: ty1, r#const: openmodelica_frontend_types::DAE::Const::C_VAR, par: openmodelica_frontend_types::DAE::VarParallelism::NON_PARALLEL, defaultBinding: None })], funcResultType: DAE::T_BOOL_DEFAULT().clone(), functionAttributes: DAE::FUNCTION_ATTRIBUTES_BUILTIN_IMPURE.clone(), path: metamodelica::Ref::new(Absyn::Path::IDENT { name: literal!("firstTick") }) });
            let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(elabCallArgs3(cache, env, list![ty], metamodelica::Ref::new(Absyn::Path::IDENT { name: literal!("firstTick") }), args, nargs, &(metamodelica::nil()), r#impl, pre, info)?) {
                (__pa0, Some((__pa1, __pa2))) => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
                _ => return Err("pattern mismatch"),
            } };
            cache = metamodelica::Own::own(__pa0);
            call = metamodelica::Own::own(__pa1);
            prop = metamodelica::Own::own(__pa2);
            (cache, call, prop)
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok((outCache, outExp, outProperties))
}

fn elabBuiltinInterval(
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
    mut args: &metamodelica::List<metamodelica::Ref<Absyn::Exp>>,
    mut nargs: &metamodelica::List<metamodelica::Ref<Absyn::NamedArg>>,
    mut inBoolean: bool,
    mut inPrefix: DAE::Prefix,
    mut info: SourceInfo,
) -> Result<(FCore::Cache, metamodelica::Ref<DAE::Exp>, DAE::Properties)> {
    let mut outCache: FCore::Cache;
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    let mut outProperties: DAE::Properties;
    (outCache, outExp, outProperties) = (::match_deref::match_deref! { match (args, nargs) {
        (Deref @ metamodelica::ListNode::Nil, Deref @ metamodelica::ListNode::Nil) => {
            let mut cache = inCache;
            let mut env = inEnv;
            let mut r#impl = inBoolean;
            let mut pre = inPrefix;
            let mut call: metamodelica::Ref<DAE::Exp>;
            let mut ty: metamodelica::Ref<DAE::Type>;
            let mut prop: DAE::Properties;
            ty = metamodelica::Ref::new(DAE::Type::T_FUNCTION { funcArg: metamodelica::nil(), funcResultType: DAE::T_REAL_DEFAULT().clone(), functionAttributes: DAE::FUNCTION_ATTRIBUTES_BUILTIN_IMPURE.clone(), path: metamodelica::Ref::new(Absyn::Path::IDENT { name: literal!("interval") }) });
            let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(elabCallArgs3(cache, env, list![ty], metamodelica::Ref::new(Absyn::Path::IDENT { name: literal!("interval") }), args, nargs, &(metamodelica::nil()), r#impl, pre, info)?) {
                (__pa0, Some((__pa1, __pa2))) => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
                _ => return Err("pattern mismatch"),
            } };
            cache = metamodelica::Own::own(__pa0);
            call = metamodelica::Own::own(__pa1);
            prop = metamodelica::Own::own(__pa2);
            (cache, call, prop)
        },
        (Deref @ metamodelica::ListNode::Cons { head: au, tail: Deref @ metamodelica::ListNode::Nil }, Deref @ metamodelica::ListNode::Nil) => {
            let mut cache = inCache;
            let mut env = inEnv;
            let mut r#impl = inBoolean;
            let mut pre = inPrefix;
            let mut call: metamodelica::Ref<DAE::Exp>;
            let mut ty1: metamodelica::Ref<DAE::Type>;
            let mut ty: metamodelica::Ref<DAE::Type>;
            let mut prop1: DAE::Properties;
            let mut prop: DAE::Properties;
            (cache, _, prop1) = elabExpInExpression(cache, env.clone(), au.clone(), r#impl, true, pre.clone(), info.clone())?;
            ty1 = Types::arrayElementType(&(Types::getPropType(&prop1)));
            ty = metamodelica::Ref::new(DAE::Type::T_FUNCTION { funcArg: list![metamodelica::Ref::new(DAE::FuncArg { name: literal!("u"), ty: ty1, r#const: openmodelica_frontend_types::DAE::Const::C_VAR, par: openmodelica_frontend_types::DAE::VarParallelism::NON_PARALLEL, defaultBinding: None })], funcResultType: DAE::T_REAL_DEFAULT().clone(), functionAttributes: DAE::FUNCTION_ATTRIBUTES_BUILTIN_IMPURE.clone(), path: metamodelica::Ref::new(Absyn::Path::IDENT { name: literal!("interval") }) });
            let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(elabCallArgs3(cache, env, list![ty], metamodelica::Ref::new(Absyn::Path::IDENT { name: literal!("interval") }), args, nargs, &(metamodelica::nil()), r#impl, pre, info)?) {
                (__pa0, Some((__pa1, __pa2))) => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
                _ => return Err("pattern mismatch"),
            } };
            cache = metamodelica::Own::own(__pa0);
            call = metamodelica::Own::own(__pa1);
            prop = metamodelica::Own::own(__pa2);
            (cache, call, prop)
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok((outCache, outExp, outProperties))
}

fn isBlockTypeWorkaround<'__b>(mut ity: &'__b metamodelica::Ref<DAE::Type>) -> bool {
    '__tco: loop {
        match &**ity {
            DAE::Type::T_SUBTYPE_BASIC { .. } => {
                ity = var_field!((**ity).complexType, DAE::Type::T_SUBTYPE_BASIC);
                continue '__tco;
            }
            DAE::Type::T_COMPLEX { .. } => return true,
            _ => return false,
        }
    }
}

fn elabBuiltinTransition(
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
    mut args: metamodelica::List<metamodelica::Ref<Absyn::Exp>>,
    mut nargs: metamodelica::List<metamodelica::Ref<Absyn::NamedArg>>,
    mut inBoolean: bool,
    mut inPrefix: DAE::Prefix,
    mut info: SourceInfo,
) -> Result<(FCore::Cache, metamodelica::Ref<DAE::Exp>, DAE::Properties)> {
    let mut outCache: FCore::Cache;
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    let mut outProperties: DAE::Properties;
    (outCache, outExp, outProperties) = (match (inCache, inEnv, inBoolean, inPrefix) {
        (mut cache, mut env, mut r#impl, mut pre) => {
            let mut call: metamodelica::Ref<DAE::Exp>;
            let mut ty1: metamodelica::Ref<DAE::Type>;
            let mut ty2: metamodelica::Ref<DAE::Type>;
            let mut ty: metamodelica::Ref<DAE::Type>;
            let mut prop: DAE::Properties;
            let mut n: i32;
            let mut strMsg0: ArcStr;
            let mut strPre: ArcStr;
            let mut s1: ArcStr;
            let mut s2: ArcStr;
            let mut slist: metamodelica::List<ArcStr>;
            slist = List::map(nargs.clone(), &move |__a0: metamodelica::Ref<Absyn::NamedArg>| {
                Dump::printNamedArgStr(&__a0)
            })?;
            s1 = Dump::printExpLstStr(args.clone())?;
            s2 = stringDelimitList(metamodelica::cons(s1, slist), literal!(", "));
            strMsg0 = {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("transition("));
                __mm_s.push_str(&*s2);
                __mm_s.push_str(&*literal!(")"));
                ArcStr::from(__mm_s)
            };
            strPre = PrefixUtil::printPrefixStr3(pre.clone())?;
            n = ((args).len() as i32);
            ty1 = elabBuiltinTransition2(
                cache.clone(),
                env.clone(),
                &args,
                &nargs,
                r#impl,
                pre.clone(),
                info.clone(),
                literal!("from"),
                n,
                &strMsg0,
                strPre.clone(),
            )?;
            ty2 = elabBuiltinTransition2(
                cache.clone(),
                env.clone(),
                &args,
                &nargs,
                r#impl,
                pre.clone(),
                info.clone(),
                literal!("to"),
                n,
                &strMsg0,
                strPre,
            )?;
            ty = metamodelica::Ref::new(DAE::Type::T_FUNCTION {
                funcArg: list![
                    metamodelica::Ref::new(DAE::FuncArg {
                        name: literal!("from"),
                        ty: ty1,
                        r#const: openmodelica_frontend_types::DAE::Const::C_VAR,
                        par: openmodelica_frontend_types::DAE::VarParallelism::NON_PARALLEL,
                        defaultBinding: None
                    }),
                    metamodelica::Ref::new(DAE::FuncArg {
                        name: literal!("to"),
                        ty: ty2,
                        r#const: openmodelica_frontend_types::DAE::Const::C_VAR,
                        par: openmodelica_frontend_types::DAE::VarParallelism::NON_PARALLEL,
                        defaultBinding: None
                    }),
                    metamodelica::Ref::new(DAE::FuncArg {
                        name: literal!("condition"),
                        ty: DAE::T_BOOL_DEFAULT().clone(),
                        r#const: openmodelica_frontend_types::DAE::Const::C_VAR,
                        par: openmodelica_frontend_types::DAE::VarParallelism::NON_PARALLEL,
                        defaultBinding: None
                    }),
                    metamodelica::Ref::new(DAE::FuncArg {
                        name: literal!("immediate"),
                        ty: DAE::T_BOOL_DEFAULT().clone(),
                        r#const: openmodelica_frontend_types::DAE::Const::C_PARAM,
                        par: openmodelica_frontend_types::DAE::VarParallelism::NON_PARALLEL,
                        defaultBinding: Some(metamodelica::Ref::new(DAE::Exp::BCONST { bool: true }))
                    }),
                    metamodelica::Ref::new(DAE::FuncArg {
                        name: literal!("reset"),
                        ty: DAE::T_BOOL_DEFAULT().clone(),
                        r#const: openmodelica_frontend_types::DAE::Const::C_PARAM,
                        par: openmodelica_frontend_types::DAE::VarParallelism::NON_PARALLEL,
                        defaultBinding: Some(metamodelica::Ref::new(DAE::Exp::BCONST { bool: true }))
                    }),
                    metamodelica::Ref::new(DAE::FuncArg {
                        name: literal!("synchronize"),
                        ty: DAE::T_BOOL_DEFAULT().clone(),
                        r#const: openmodelica_frontend_types::DAE::Const::C_PARAM,
                        par: openmodelica_frontend_types::DAE::VarParallelism::NON_PARALLEL,
                        defaultBinding: Some(metamodelica::Ref::new(DAE::Exp::BCONST { bool: false }))
                    }),
                    metamodelica::Ref::new(DAE::FuncArg {
                        name: literal!("priority"),
                        ty: DAE::T_INTEGER_DEFAULT().clone(),
                        r#const: openmodelica_frontend_types::DAE::Const::C_PARAM,
                        par: openmodelica_frontend_types::DAE::VarParallelism::NON_PARALLEL,
                        defaultBinding: Some(metamodelica::Ref::new(DAE::Exp::ICONST { integer: 1 }))
                    })
                ],
                funcResultType: DAE::T_NORETCALL_DEFAULT().clone(),
                functionAttributes: DAE::FUNCTION_ATTRIBUTES_BUILTIN_IMPURE.clone(),
                path: metamodelica::Ref::new(Absyn::Path::IDENT {
                    name: literal!("transition"),
                }),
            });
            let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(elabCallArgs3(cache, env, list![ty], metamodelica::Ref::new(Absyn::Path::IDENT { name: literal!("transition") }), &args, &nargs, &(metamodelica::nil()), r#impl, pre, info)?) {
                (__pa0, Some((__pa1, __pa2))) => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
                _ => return Err("pattern mismatch"),
            } };
            cache = metamodelica::Own::own(__pa0);
            call = metamodelica::Own::own(__pa1);
            prop = metamodelica::Own::own(__pa2);
            (cache, call, prop)
        }
    });
    Ok((outCache, outExp, outProperties))
}

fn elabBuiltinTransition2(
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
    mut args: &metamodelica::List<metamodelica::Ref<Absyn::Exp>>,
    mut nargs: &metamodelica::List<metamodelica::Ref<Absyn::NamedArg>>,
    mut inBoolean: bool,
    mut inPrefix: DAE::Prefix,
    mut info: SourceInfo,
    mut argName: ArcStr,
    mut n: i32,
    mut strMsg0: &ArcStr,
    mut strPre: ArcStr,
) -> Result<metamodelica::Ref<DAE::Type>> {
    let mut ty: metamodelica::Ref<DAE::Type>;
    let mut arg1: metamodelica::Ref<Absyn::Exp>;
    let mut prop1: DAE::Properties;
    let mut nPos: i32;
    let mut s1: ArcStr;
    let mut s2: ArcStr;
    let mut strPos: ArcStr;
    let mut strMsg1: ArcStr;
    let mut b1: bool;
    strPos = if (metamodelica::stringEq(&argName, &(literal!("from")))) {
        literal!("first")
    } else {
        literal!("second")
    };
    nPos = if (metamodelica::stringEq(&argName, &(literal!("from")))) {
        1
    } else {
        2
    };
    b1 = List::isMemberOnTrue(argName.clone(), nargs, &move |__a0: ArcStr,
                                                             __a1: metamodelica::Ref<
        Absyn::NamedArg,
    >|
          -> metamodelica::Result<_> {
        ::std::result::Result::Ok(elabBuiltinTransition3(&__a0, &__a1))
    })?;
    s1 = {
        let mut __mm_s = String::new();
        __mm_s.push_str(&*strMsg0);
        __mm_s.push_str(&*literal!(", named argument \""));
        __mm_s.push_str(&*argName);
        __mm_s.push_str(&*literal!("\" already has a value."));
        ArcStr::from(__mm_s)
    };
    Error::assertionOrAddSourceMessage(
        !(b1 && n >= nPos),
        &(Error::WRONG_TYPE_OR_NO_OF_ARGS.clone()),
        list![s1, strPre.clone()],
        &info,
    )?;
    s2 = {
        let mut __mm_s = String::new();
        __mm_s.push_str(&*strMsg0);
        __mm_s.push_str(&*literal!(", missing value for "));
        __mm_s.push_str(&*strPos);
        __mm_s.push_str(&*literal!(" argument \""));
        __mm_s.push_str(&*argName);
        __mm_s.push_str(&*literal!("\"."));
        ArcStr::from(__mm_s)
    };
    Error::assertionOrAddSourceMessage(
        b1 || n >= nPos,
        &(Error::WRONG_TYPE_OR_NO_OF_ARGS.clone()),
        list![s2, strPre.clone()],
        &info,
    )?;
    arg1 = elabBuiltinTransition5(&argName, b1, args, nargs)?;
    (_, _, prop1) = elabExpInExpression(inCache, inEnv, arg1, inBoolean, true, inPrefix, info.clone())?;
    ty = Types::getPropType(&prop1);
    strMsg1 = {
        let mut __mm_s = String::new();
        __mm_s.push_str(&*strMsg0);
        __mm_s.push_str(&*literal!(", "));
        __mm_s.push_str(&*strPos);
        __mm_s.push_str(&*literal!("argument needs to be a block instance."));
        ArcStr::from(__mm_s)
    };
    Error::assertionOrAddSourceMessage(
        isBlockTypeWorkaround(&ty),
        &(Error::WRONG_TYPE_OR_NO_OF_ARGS.clone()),
        list![strMsg1, strPre],
        &info,
    )?;
    Ok(ty)
}

fn elabBuiltinTransition3(mut name: &ArcStr, mut namedArg: &metamodelica::Ref<Absyn::NamedArg>) -> bool {
    let mut outIsEqual: bool;
    outIsEqual = (match &**namedArg {
        Absyn::NamedArg { .. } => stringEq(&name, &namedArg.argName),
        _ => false,
    });
    outIsEqual
}

fn elabBuiltinTransition4(mut inElement: &metamodelica::Ref<Absyn::NamedArg>) -> metamodelica::Ref<Absyn::Exp> {
    let mut argValue: metamodelica::Ref<Absyn::Exp>;
    let __arc1 = &(*inElement);
    let Absyn::NAMEDARG { argValue: __pa0, .. } = &**__arc1;
    argValue = metamodelica::Own::own(__pa0);
    argValue
}

fn elabBuiltinTransition5(
    mut argName: &ArcStr,
    mut getAsNamedArg: bool,
    mut args: &metamodelica::List<metamodelica::Ref<Absyn::Exp>>,
    mut nargs: &metamodelica::List<metamodelica::Ref<Absyn::NamedArg>>,
) -> Result<metamodelica::Ref<Absyn::Exp>> {
    let mut argValue: metamodelica::Ref<Absyn::Exp>;
    argValue = (::match_deref::match_deref! { match &((argName.clone(), getAsNamedArg)) {
        (Deref @ "from", true) => {
            let mut namedArg: metamodelica::Ref<Absyn::NamedArg>;
            namedArg = List::getMemberOnTrue(literal!("from"), nargs, &move |__a0: ArcStr, __a1: metamodelica::Ref<Absyn::NamedArg>| -> metamodelica::Result<_> { ::std::result::Result::Ok(elabBuiltinTransition3(&__a0, &__a1)) })?;
            elabBuiltinTransition4(&namedArg)
        },
        (Deref @ "from", false) => {
            (args).head().cloned()?
        },
        (Deref @ "to", true) => {
            let mut namedArg: metamodelica::Ref<Absyn::NamedArg>;
            namedArg = List::getMemberOnTrue(literal!("to"), nargs, &move |__a0: ArcStr, __a1: metamodelica::Ref<Absyn::NamedArg>| -> metamodelica::Result<_> { ::std::result::Result::Ok(elabBuiltinTransition3(&__a0, &__a1)) })?;
            elabBuiltinTransition4(&namedArg)
        },
        (Deref @ "to", false) => {
            (args).get(2)?
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok(argValue)
}

fn elabBuiltinInitialState(
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
    mut args: metamodelica::List<metamodelica::Ref<Absyn::Exp>>,
    mut nargs: &metamodelica::List<metamodelica::Ref<Absyn::NamedArg>>,
    mut inBoolean: bool,
    mut inPrefix: DAE::Prefix,
    mut info: SourceInfo,
) -> Result<(FCore::Cache, metamodelica::Ref<DAE::Exp>, DAE::Properties)> {
    let mut outCache: FCore::Cache;
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    let mut outProperties: DAE::Properties;
    (outCache, outExp, outProperties) = (::match_deref::match_deref! { match &((args.clone(), nargs.clone())) {
        (Deref @ metamodelica::ListNode::Cons { head: astate, tail: Deref @ metamodelica::ListNode::Nil }, Deref @ metamodelica::ListNode::Nil) => {
            let mut cache = inCache;
            let mut env = inEnv;
            let mut r#impl = inBoolean;
            let mut pre = inPrefix;
            let mut call: metamodelica::Ref<DAE::Exp>;
            let mut ty1: metamodelica::Ref<DAE::Type>;
            let mut ty: metamodelica::Ref<DAE::Type>;
            let mut prop1: DAE::Properties;
            let mut prop: DAE::Properties;
            let mut strMsg: ArcStr;
            let mut strPre: ArcStr;
            (cache, _, prop1) = elabExpInExpression(cache, env.clone(), astate.clone(), r#impl, true, pre.clone(), info.clone())?;
            ty1 = Types::getPropType(&prop1);
            strMsg = { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("initialState(")); __mm_s.push_str(&*Dump::printExpLstStr(args.clone())?); __mm_s.push_str(&*literal!("), Argument needs to be a block instance.")); ArcStr::from(__mm_s) };
            strPre = PrefixUtil::printPrefixStr3(pre.clone())?;
            Error::assertionOrAddSourceMessage(isBlockTypeWorkaround(&ty1), &(Error::WRONG_TYPE_OR_NO_OF_ARGS.clone()), list![strMsg, strPre], &info)?;
            ty = metamodelica::Ref::new(DAE::Type::T_FUNCTION { funcArg: list![metamodelica::Ref::new(DAE::FuncArg { name: literal!("state"), ty: ty1, r#const: openmodelica_frontend_types::DAE::Const::C_VAR, par: openmodelica_frontend_types::DAE::VarParallelism::NON_PARALLEL, defaultBinding: None })], funcResultType: DAE::T_NORETCALL_DEFAULT().clone(), functionAttributes: DAE::FUNCTION_ATTRIBUTES_BUILTIN_IMPURE.clone(), path: metamodelica::Ref::new(Absyn::Path::IDENT { name: literal!("initialState") }) });
            let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(elabCallArgs3(cache, env, list![ty], metamodelica::Ref::new(Absyn::Path::IDENT { name: literal!("initialState") }), &args, nargs, &(metamodelica::nil()), r#impl, pre, info)?) {
                (__pa0, Some((__pa1, __pa2))) => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
                _ => return Err("pattern mismatch"),
            } };
            cache = metamodelica::Own::own(__pa0);
            call = metamodelica::Own::own(__pa1);
            prop = metamodelica::Own::own(__pa2);
            (cache, call, prop)
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok((outCache, outExp, outProperties))
}

fn elabBuiltinActiveState(
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
    mut args: metamodelica::List<metamodelica::Ref<Absyn::Exp>>,
    mut nargs: &metamodelica::List<metamodelica::Ref<Absyn::NamedArg>>,
    mut inBoolean: bool,
    mut inPrefix: DAE::Prefix,
    mut info: SourceInfo,
) -> Result<(FCore::Cache, metamodelica::Ref<DAE::Exp>, DAE::Properties)> {
    let mut outCache: FCore::Cache;
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    let mut outProperties: DAE::Properties;
    (outCache, outExp, outProperties) = (::match_deref::match_deref! { match &((args.clone(), nargs.clone())) {
        (Deref @ metamodelica::ListNode::Cons { head: astate, tail: Deref @ metamodelica::ListNode::Nil }, Deref @ metamodelica::ListNode::Nil) => {
            let mut cache = inCache;
            let mut env = inEnv;
            let mut r#impl = inBoolean;
            let mut pre = inPrefix;
            let mut call: metamodelica::Ref<DAE::Exp>;
            let mut ty1: metamodelica::Ref<DAE::Type>;
            let mut ty: metamodelica::Ref<DAE::Type>;
            let mut prop1: DAE::Properties;
            let mut prop: DAE::Properties;
            let mut strMsg: ArcStr;
            let mut strPre: ArcStr;
            (cache, _, prop1) = elabExpInExpression(cache, env.clone(), astate.clone(), r#impl, true, pre.clone(), info.clone())?;
            ty1 = Types::getPropType(&prop1);
            strMsg = { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("activeState(")); __mm_s.push_str(&*Dump::printExpLstStr(args.clone())?); __mm_s.push_str(&*literal!("), Argument needs to be a block instance.")); ArcStr::from(__mm_s) };
            strPre = PrefixUtil::printPrefixStr3(pre.clone())?;
            Error::assertionOrAddSourceMessage(isBlockTypeWorkaround(&ty1), &(Error::WRONG_TYPE_OR_NO_OF_ARGS.clone()), list![strMsg, strPre], &info)?;
            ty = metamodelica::Ref::new(DAE::Type::T_FUNCTION { funcArg: list![metamodelica::Ref::new(DAE::FuncArg { name: literal!("state"), ty: ty1, r#const: openmodelica_frontend_types::DAE::Const::C_VAR, par: openmodelica_frontend_types::DAE::VarParallelism::NON_PARALLEL, defaultBinding: None })], funcResultType: DAE::T_BOOL_DEFAULT().clone(), functionAttributes: DAE::FUNCTION_ATTRIBUTES_BUILTIN_IMPURE.clone(), path: metamodelica::Ref::new(Absyn::Path::IDENT { name: literal!("activeState") }) });
            let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(elabCallArgs3(cache, env, list![ty], metamodelica::Ref::new(Absyn::Path::IDENT { name: literal!("activeState") }), &args, nargs, &(metamodelica::nil()), r#impl, pre, info)?) {
                (__pa0, Some((__pa1, __pa2))) => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
                _ => return Err("pattern mismatch"),
            } };
            cache = metamodelica::Own::own(__pa0);
            call = metamodelica::Own::own(__pa1);
            prop = metamodelica::Own::own(__pa2);
            (cache, call, prop)
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok((outCache, outExp, outProperties))
}

fn elabBuiltinTicksInState(
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
    mut args: &metamodelica::List<metamodelica::Ref<Absyn::Exp>>,
    mut nargs: &metamodelica::List<metamodelica::Ref<Absyn::NamedArg>>,
    mut inBoolean: bool,
    mut inPrefix: DAE::Prefix,
    mut info: SourceInfo,
) -> Result<(FCore::Cache, metamodelica::Ref<DAE::Exp>, DAE::Properties)> {
    let mut outCache: FCore::Cache;
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    let mut outProperties: DAE::Properties;
    (outCache, outExp, outProperties) = (::match_deref::match_deref! { match (args, nargs) {
        (Deref @ metamodelica::ListNode::Nil, Deref @ metamodelica::ListNode::Nil) => {
            let mut cache = inCache;
            let mut env = inEnv;
            let mut r#impl = inBoolean;
            let mut pre = inPrefix;
            let mut call: metamodelica::Ref<DAE::Exp>;
            let mut ty: metamodelica::Ref<DAE::Type>;
            let mut prop: DAE::Properties;
            ty = metamodelica::Ref::new(DAE::Type::T_FUNCTION { funcArg: metamodelica::nil(), funcResultType: DAE::T_INTEGER_DEFAULT().clone(), functionAttributes: DAE::FUNCTION_ATTRIBUTES_BUILTIN_IMPURE.clone(), path: metamodelica::Ref::new(Absyn::Path::IDENT { name: literal!("ticksInState") }) });
            let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(elabCallArgs3(cache, env, list![ty], metamodelica::Ref::new(Absyn::Path::IDENT { name: literal!("ticksInState") }), args, nargs, &(metamodelica::nil()), r#impl, pre, info)?) {
                (__pa0, Some((__pa1, __pa2))) => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
                _ => return Err("pattern mismatch"),
            } };
            cache = metamodelica::Own::own(__pa0);
            call = metamodelica::Own::own(__pa1);
            prop = metamodelica::Own::own(__pa2);
            (cache, call, prop)
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok((outCache, outExp, outProperties))
}

fn elabBuiltinTimeInState(
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
    mut args: &metamodelica::List<metamodelica::Ref<Absyn::Exp>>,
    mut nargs: &metamodelica::List<metamodelica::Ref<Absyn::NamedArg>>,
    mut inBoolean: bool,
    mut inPrefix: DAE::Prefix,
    mut info: SourceInfo,
) -> Result<(FCore::Cache, metamodelica::Ref<DAE::Exp>, DAE::Properties)> {
    let mut outCache: FCore::Cache;
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    let mut outProperties: DAE::Properties;
    (outCache, outExp, outProperties) = (::match_deref::match_deref! { match (args, nargs) {
        (Deref @ metamodelica::ListNode::Nil, Deref @ metamodelica::ListNode::Nil) => {
            let mut cache = inCache;
            let mut env = inEnv;
            let mut r#impl = inBoolean;
            let mut pre = inPrefix;
            let mut call: metamodelica::Ref<DAE::Exp>;
            let mut ty: metamodelica::Ref<DAE::Type>;
            let mut prop: DAE::Properties;
            ty = metamodelica::Ref::new(DAE::Type::T_FUNCTION { funcArg: metamodelica::nil(), funcResultType: DAE::T_REAL_DEFAULT().clone(), functionAttributes: DAE::FUNCTION_ATTRIBUTES_BUILTIN_IMPURE.clone(), path: metamodelica::Ref::new(Absyn::Path::IDENT { name: literal!("timeInState") }) });
            let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(elabCallArgs3(cache, env, list![ty], metamodelica::Ref::new(Absyn::Path::IDENT { name: literal!("timeInState") }), args, nargs, &(metamodelica::nil()), r#impl, pre, info)?) {
                (__pa0, Some((__pa1, __pa2))) => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
                _ => return Err("pattern mismatch"),
            } };
            cache = metamodelica::Own::own(__pa0);
            call = metamodelica::Own::own(__pa1);
            prop = metamodelica::Own::own(__pa2);
            (cache, call, prop)
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok((outCache, outExp, outProperties))
}

fn elabBuiltinBoolean(
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
    mut inPosArgs: &metamodelica::List<metamodelica::Ref<Absyn::Exp>>,
    mut inNamedArgs: &metamodelica::List<metamodelica::Ref<Absyn::NamedArg>>,
    mut inImplicit: bool,
    mut inPrefix: DAE::Prefix,
    mut inInfo: SourceInfo,
) -> Result<(FCore::Cache, metamodelica::Ref<DAE::Exp>, DAE::Properties)> {
    let mut outCache: FCore::Cache;
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    let mut outProperties: DAE::Properties;
    (outCache, outExp, outProperties) = verifyBuiltInHandlerType(
        inCache,
        inEnv,
        inPosArgs,
        inImplicit,
        &fnptr!(
            Types::isIntegerOrRealOrBooleanOrSubTypeOfEither,
            metamodelica::Ref<DAE::Type>
        ),
        literal!("boolean"),
        inPrefix,
        inInfo,
    )?;
    Ok((outCache, outExp, outProperties))
}

fn elabBuiltinIntegerEnum(
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
    mut inPosArgs: &metamodelica::List<metamodelica::Ref<Absyn::Exp>>,
    mut inNamedArg: &metamodelica::List<metamodelica::Ref<Absyn::NamedArg>>,
    mut inImplicit: bool,
    mut inPrefix: DAE::Prefix,
    mut inInfo: SourceInfo,
) -> Result<(FCore::Cache, metamodelica::Ref<DAE::Exp>, DAE::Properties)> {
    let mut outCache: FCore::Cache;
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    let mut outProperties: DAE::Properties;
    (outCache, outExp, outProperties) = verifyBuiltInHandlerType(
        inCache,
        inEnv,
        inPosArgs,
        inImplicit,
        &move |__a0: metamodelica::Ref<DAE::Type>| -> metamodelica::Result<_> {
            ::std::result::Result::Ok(Types::isEnumeration(&__a0))
        },
        literal!("Integer"),
        inPrefix,
        inInfo,
    )?;
    Ok((outCache, outExp, outProperties))
}

fn elabBuiltinNoevent(
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
    mut inPosArgs: &metamodelica::List<metamodelica::Ref<Absyn::Exp>>,
    mut inNamedArgs: &metamodelica::List<metamodelica::Ref<Absyn::NamedArg>>,
    mut inImplicit: bool,
    mut inPrefix: DAE::Prefix,
    mut inInfo: SourceInfo,
) -> Result<(FCore::Cache, metamodelica::Ref<DAE::Exp>, DAE::Properties)> {
    let mut outCache: FCore::Cache;
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    let mut outProperties: DAE::Properties;
    let mut e: metamodelica::Ref<Absyn::Exp>;
    checkBuiltinCallArgs(inPosArgs, inNamedArgs, 1, literal!("noEvent"), &inInfo)?;
    e = (inPosArgs).head().cloned()?;
    (outCache, outExp, outProperties) = elabExpInExpression(inCache, inEnv, e, inImplicit, true, inPrefix, inInfo)?;
    outExp = Expression::makePureBuiltinCall(literal!("noEvent"), list![outExp], DAE::T_BOOL_DEFAULT().clone());
    Ok((outCache, outExp, outProperties))
}

fn elabBuiltinEdge(
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
    mut inPosArgs: &metamodelica::List<metamodelica::Ref<Absyn::Exp>>,
    mut inNamedArgs: &metamodelica::List<metamodelica::Ref<Absyn::NamedArg>>,
    mut inImplicit: bool,
    mut inPrefix: DAE::Prefix,
    mut inInfo: SourceInfo,
) -> Result<(FCore::Cache, metamodelica::Ref<DAE::Exp>, DAE::Properties)> {
    let mut outCache: FCore::Cache;
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    let mut outProperties: DAE::Properties;
    let mut ty: metamodelica::Ref<DAE::Type>;
    let mut c: DAE::Const;
    let mut msg: ArcStr;
    checkBuiltinCallArgs(inPosArgs, inNamedArgs, 1, literal!("edge"), &inInfo)?;
    (outCache, outExp, outProperties) = elabExpInExpression(
        inCache,
        inEnv,
        (inPosArgs).head().cloned()?,
        inImplicit,
        true,
        inPrefix,
        inInfo.clone(),
    )?;
    let DAE::PROP {
        type_: __pa0,
        constFlag: __pa1,
    } = (outProperties.clone())
    else {
        return Err("pattern mismatch");
    };
    ty = metamodelica::Own::own(__pa0);
    c = metamodelica::Own::own(__pa1);
    if !(Types::isScalarBoolean(&ty)) {
        msg = {
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("edge("));
            __mm_s.push_str(&*ExpressionBasics::printExpStr(outExp.clone())?);
            __mm_s.push_str(&*literal!(")"));
            ArcStr::from(__mm_s)
        };
        Error::addSourceMessageAndFail(&(Error::TYPE_ERROR.clone()), list![msg], &inInfo)?;
        unreachable!("Error.addSourceMessageAndFail always fails — caller-side flow-analysis hint");
    }
    if Types::isVar(c) {
        outExp = Expression::makePureBuiltinCall(literal!("edge"), list![outExp], DAE::T_BOOL_DEFAULT().clone());
    } else {
        outExp = metamodelica::Ref::new(DAE::Exp::BCONST { bool: false });
    }
    Ok((outCache, outExp, outProperties))
}

fn elabBuiltinDer(
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
    mut inPosArgs: metamodelica::List<metamodelica::Ref<Absyn::Exp>>,
    mut inNamedArgs: &metamodelica::List<metamodelica::Ref<Absyn::NamedArg>>,
    mut inImplicit: bool,
    mut inPrefix: DAE::Prefix,
    mut inInfo: SourceInfo,
) -> Result<(FCore::Cache, metamodelica::Ref<DAE::Exp>, DAE::Properties)> {
    let mut outCache: FCore::Cache;
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    let mut outProperties: DAE::Properties;
    let mut ty: metamodelica::Ref<DAE::Type>;
    let mut c: DAE::Const;
    let mut dims: metamodelica::List<metamodelica::Ref<DAE::Dimension>>;
    let mut exp_str: ArcStr;
    let mut ty_str: ArcStr;
    if FGraph::inFunctionScope(&inEnv) {
        Error::addSourceMessageAndFail(
            &(Error::DERIVATIVE_FUNCTION_CONTEXT.clone()),
            metamodelica::nil(),
            &inInfo,
        )?;
        unreachable!("Error.addSourceMessageAndFail always fails — caller-side flow-analysis hint");
    }
    checkBuiltinCallArgs(&inPosArgs, inNamedArgs, 1, literal!("der"), &inInfo)?;
    (outCache, outExp, outProperties) = elabExpInExpression(
        inCache.clone(),
        inEnv.clone(),
        (inPosArgs).head().cloned()?,
        inImplicit,
        true,
        inPrefix.clone(),
        inInfo.clone(),
    )?;
    let DAE::PROP {
        type_: __pa0,
        constFlag: __pa1,
    } = (outProperties.clone())
    else {
        return Err("pattern mismatch");
    };
    ty = metamodelica::Own::own(__pa0);
    c = metamodelica::Own::own(__pa1);
    if !(Types::isRealOrSubTypeReal(Types::arrayElementType(&ty))) {
        exp_str = Dump::printExpStr((inPosArgs).head().cloned()?)?;
        ty_str = TypesDump::unparseTypeNoAttr(&ty)?;
        Error::addSourceMessageAndFail(&(Error::DERIVATIVE_NON_REAL.clone()), list![exp_str, ty_str], &inInfo)?;
        unreachable!("Error.addSourceMessageAndFail always fails — caller-side flow-analysis hint");
    }
    if Types::isVar(c) {
        if Types::dimensionsKnown(ty.clone()) {
            (outCache, outExp, outProperties) = elabCallArgs(
                inCache,
                inEnv,
                metamodelica::Ref::new(Absyn::Path::IDENT { name: literal!("der") }),
                inPosArgs,
                metamodelica::nil(),
                &(metamodelica::nil()),
                inImplicit,
                inPrefix,
                inInfo,
            )?;
        } else {
            outExp = Expression::makePureBuiltinCall(literal!("der"), list![outExp], Types::simplifyType(ty)?);
        }
    } else {
        dims = TypesDump::getDimensions(&ty);
        (outExp, ty) = Expression::makeZeroExpression(&dims)?;
        outProperties = DAE::Properties::PROP {
            type_: ty,
            constFlag: openmodelica_frontend_types::DAE::Const::C_CONST,
        };
    }
    Ok((outCache, outExp, outProperties))
}

fn elabBuiltinChange(
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
    mut inPosArgs: &metamodelica::List<metamodelica::Ref<Absyn::Exp>>,
    mut inNamedArgs: &metamodelica::List<metamodelica::Ref<Absyn::NamedArg>>,
    mut inImplicit: bool,
    mut inPrefix: DAE::Prefix,
    mut inInfo: SourceInfo,
) -> Result<(FCore::Cache, metamodelica::Ref<DAE::Exp>, DAE::Properties)> {
    let mut outCache: FCore::Cache;
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    let mut outProperties: DAE::Properties;
    let mut e: metamodelica::Ref<Absyn::Exp>;
    let mut pre_str: ArcStr;
    let mut ty: metamodelica::Ref<DAE::Type>;
    let mut c: DAE::Const;
    let mut attr: metamodelica::Ref<DAE::Attributes>;
    let mut cref: metamodelica::Ref<DAE::ComponentRef>;
    let mut var: SCode::Variability;
    checkBuiltinCallArgs(inPosArgs, inNamedArgs, 1, literal!("change"), &inInfo)?;
    e = (inPosArgs).head().cloned()?;
    if !(AbsynUtil::isCref(&e)) {
        pre_str = PrefixUtil::printPrefixStr3(inPrefix.clone())?;
        Error::addSourceMessageAndFail(
            &(Error::ARGUMENT_MUST_BE_VARIABLE.clone()),
            list![literal!("First"), literal!("change"), pre_str],
            &inInfo,
        )?;
        unreachable!("Error.addSourceMessageAndFail always fails — caller-side flow-analysis hint");
    }
    (outCache, outExp, outProperties) = elabExpInExpression(
        inCache,
        inEnv.clone(),
        e,
        inImplicit,
        true,
        inPrefix.clone(),
        inInfo.clone(),
    )?;
    let DAE::PROP {
        type_: __pa0,
        constFlag: __pa1,
    } = (outProperties.clone())
    else {
        return Err("pattern mismatch");
    };
    ty = metamodelica::Own::own(__pa0);
    c = metamodelica::Own::own(__pa1);
    if Types::isSimpleType(&ty) {
        if Types::isParameterOrConstant(c) {
            outExp = metamodelica::Ref::new(DAE::Exp::BCONST { bool: false });
            outProperties = DAE::Properties::PROP {
                type_: DAE::T_BOOL_DEFAULT().clone(),
                constFlag: openmodelica_frontend_types::DAE::Const::C_CONST,
            };
        } else if Types::isDiscreteType(&ty) {
            outExp = Expression::makePureBuiltinCall(literal!("change"), list![outExp], DAE::T_BOOL_DEFAULT().clone());
            outProperties = DAE::Properties::PROP {
                type_: DAE::T_BOOL_DEFAULT().clone(),
                constFlag: openmodelica_frontend_types::DAE::Const::C_VAR,
            };
        } else {
            cref = Expression::getCrefFromCrefOrAsub(&outExp)?;
            (outCache, attr, _, _, _, _, _, _, _) = Lookup::lookupVar(outCache, inEnv, cref)?;
            let __arc3 = attr;
            let DAE::ATTR { variability: __pa2, .. } = &*__arc3;
            var = metamodelica::Own::own(__pa2);
            if var == openmodelica_frontend_types::SCode::Variability::DISCRETE {
                outExp =
                    Expression::makePureBuiltinCall(literal!("change"), list![outExp], DAE::T_BOOL_DEFAULT().clone());
                outProperties = DAE::Properties::PROP {
                    type_: DAE::T_BOOL_DEFAULT().clone(),
                    constFlag: openmodelica_frontend_types::DAE::Const::C_VAR,
                };
            } else {
                pre_str = PrefixUtil::printPrefixStr3(inPrefix)?;
                Error::addSourceMessageAndFail(
                    &(Error::ARGUMENT_MUST_BE_DISCRETE_VAR.clone()),
                    list![literal!("First"), literal!("change"), pre_str],
                    &inInfo,
                )?;
                unreachable!("Error.addSourceMessageAndFail always fails — caller-side flow-analysis hint");
            }
        }
    } else {
        pre_str = PrefixUtil::printPrefixStr3(inPrefix)?;
        Error::addSourceMessageAndFail(
            &(Error::TYPE_MUST_BE_SIMPLE.clone()),
            list![literal!("operand to change"), pre_str],
            &inInfo,
        )?;
        unreachable!("Error.addSourceMessageAndFail always fails — caller-side flow-analysis hint");
    }
    Ok((outCache, outExp, outProperties))
}

fn elabBuiltinCat(
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
    mut inPosArgs: metamodelica::List<metamodelica::Ref<Absyn::Exp>>,
    mut inNamedArgs: &metamodelica::List<metamodelica::Ref<Absyn::NamedArg>>,
    mut inImplicit: bool,
    mut inPrefix: DAE::Prefix,
    mut inInfo: SourceInfo,
) -> Result<(FCore::Cache, metamodelica::Ref<DAE::Exp>, DAE::Properties)> {
    let mut outCache: FCore::Cache;
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    let mut outProperties: DAE::Properties;
    let mut dim_exp: metamodelica::Ref<DAE::Exp>;
    let mut dim_props: DAE::Properties;
    let mut dim_ty: metamodelica::Ref<DAE::Type>;
    let mut ty: metamodelica::Ref<DAE::Type>;
    let mut result_ty: metamodelica::Ref<DAE::Type>;
    let mut dim_c: DAE::Const;
    let mut arr_c: DAE::Const;
    let mut c: DAE::Const;
    let mut pre_str: ArcStr;
    let mut exp_str: ArcStr;
    let mut dim_int: i32;
    let mut arr_expl: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
    let mut arr_props: metamodelica::List<DAE::Properties>;
    let mut arr_tys: metamodelica::List<metamodelica::Ref<DAE::Type>>;
    let mut tys: metamodelica::List<metamodelica::Ref<DAE::Type>>;
    let mut dims: metamodelica::List<metamodelica::Ref<DAE::Dimension>>;
    let mut dim: metamodelica::Ref<DAE::Dimension>;
    if ((inPosArgs).len() as i32) < 2 || !((inNamedArgs).is_empty()) {
        Error::addSourceMessageAndFail(&(Error::WRONG_NO_OF_ARGS.clone()), list![literal!("cat")], &inInfo)?;
        unreachable!("Error.addSourceMessageAndFail always fails — caller-side flow-analysis hint");
    }
    (outCache, dim_exp, dim_props) = elabExpInExpression(
        inCache.clone(),
        inEnv.clone(),
        (inPosArgs).head().cloned()?,
        inImplicit,
        true,
        inPrefix.clone(),
        inInfo.clone(),
    )?;
    let DAE::PROP {
        type_: __pa0,
        constFlag: __pa1,
    } = (dim_props)
    else {
        return Err("pattern mismatch");
    };
    dim_ty = metamodelica::Own::own(__pa0);
    dim_c = metamodelica::Own::own(__pa1);
    if !(Types::isScalarInteger(&dim_ty)) {
        pre_str = PrefixUtil::printPrefixStr3(inPrefix.clone())?;
        Error::addSourceMessageAndFail(
            &(Error::ARGUMENT_MUST_BE_INTEGER.clone()),
            list![literal!("First"), literal!("cat"), pre_str.clone()],
            &inInfo,
        )?;
        unreachable!("Error.addSourceMessageAndFail always fails — caller-side flow-analysis hint");
    }
    let (__pa2, __pa3) = ::match_deref::match_deref! { match &(Ceval::ceval(inCache, inEnv.clone(), dim_exp.clone(), false, Absyn::Msg::MSG { info: inInfo.clone() }, 0)?) {
        (__pa2, Deref @ Values::Value::INTEGER { integer: __pa3 }) => (__pa2.clone(), __pa3.clone()),
        _ => return Err("pattern mismatch"),
    } };
    outCache = metamodelica::Own::own(__pa2);
    dim_int = metamodelica::Own::own(__pa3);
    (outCache, arr_expl, arr_props) = elabExpList(
        outCache,
        inEnv,
        &((inPosArgs).rest()?),
        inImplicit,
        true,
        inPrefix.clone(),
        inInfo.clone(),
        DAE::T_UNKNOWN_DEFAULT().clone(),
    )?;
    arr_tys = ({
        let mut __acc: metamodelica::List<metamodelica::Ref<DAE::Type>> = metamodelica::nil();
        for mut p in (arr_props.clone()).into_iter().cloned() {
            let __x = Types::getPropType(&(p.clone()));
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    let (__pa5, __pa6) = ::match_deref::match_deref! { match &(({
        let mut __acc: metamodelica::List<metamodelica::Ref<DAE::Type>> = metamodelica::nil();
        for mut t in (arr_tys.clone()).into_iter().cloned() {
            let __x = Types::makeNthDimUnknown(&(t.clone()), dim_int)?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    })) {
        Deref @ metamodelica::ListNode::Cons { head: __pa5, tail: __pa6 } => (__pa5.clone(), __pa6.clone()),
        _ => return Err("pattern mismatch"),
    } };
    ty = metamodelica::Own::own(__pa5);
    tys = metamodelica::Own::own(__pa6);
    result_ty = List::fold1(
        &tys,
        &move |__a0: metamodelica::Ref<DAE::Type>, __a1: SourceInfo, __a2: metamodelica::Ref<DAE::Type>| {
            Types::arraySuperType(__a0, &__a1, __a2)
        },
        inInfo.clone(),
        ty,
    )?;
    if let Ok((__pa7, __pa8)) = Types::matchTypes(arr_expl.clone(), arr_tys.clone(), &result_ty, false) {
        arr_expl = metamodelica::Own::own(__pa7);
        arr_tys = metamodelica::Own::own(__pa8);
    } else {
        exp_str = stringDelimitList(
            ({
                let mut __acc: metamodelica::List<ArcStr> = metamodelica::nil();
                for mut e in (inPosArgs.clone()).into_iter().cloned() {
                    let __x = Dump::printExpStr(e.clone())?;
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            }),
            literal!(", "),
        );
        exp_str = {
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("cat("));
            __mm_s.push_str(&*exp_str);
            __mm_s.push_str(&*literal!(")"));
            ArcStr::from(__mm_s)
        };
        pre_str = PrefixUtil::printPrefixStr3(inPrefix.clone())?;
        Error::addSourceMessageAndFail(
            &(Error::DIFFERENT_DIM_SIZE_IN_ARGUMENTS.clone()),
            list![exp_str.clone(), pre_str.clone()],
            &inInfo,
        )?;
        unreachable!("Error.addSourceMessageAndFail always fails — caller-side flow-analysis hint");
    }
    dims = ({
        let mut __acc: metamodelica::List<metamodelica::Ref<DAE::Dimension>> = metamodelica::nil();
        for mut t in (arr_tys).into_iter().cloned() {
            let __x = Types::getDimensionNth(&(t.clone()), dim_int)?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    dim = ({
        let mut __acc: Option<metamodelica::Ref<DAE::Dimension>> = None;
        for mut d in (dims).into_iter().cloned() {
            let __x = d.clone();
            __acc = Some(match __acc {
                None => __x,
                Some(__cur) => Expression::dimensionsAdd(&__x, &__cur),
            });
        }
        __acc.ok_or_else(|| "empty Expression.dimensionsAdd reduction")?
    });
    result_ty = Types::setDimensionNth(&result_ty, &dim, dim_int)?;
    arr_c = elabArrayConst(&arr_props)?;
    c = Types::constAnd(dim_c, arr_c);
    ty = Types::simplifyType(result_ty.clone())?;
    outExp = Expression::makePureBuiltinCall(literal!("cat"), metamodelica::cons(dim_exp, arr_expl), ty);
    outProperties = DAE::Properties::PROP {
        type_: result_ty,
        constFlag: c,
    };
    Ok((outCache, outExp, outProperties))
}

fn elabBuiltinIdentity(
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
    mut inPosArgs: &metamodelica::List<metamodelica::Ref<Absyn::Exp>>,
    mut inNamedArgs: &metamodelica::List<metamodelica::Ref<Absyn::NamedArg>>,
    mut inImplicit: bool,
    mut inPrefix: DAE::Prefix,
    mut inInfo: SourceInfo,
) -> Result<(FCore::Cache, metamodelica::Ref<DAE::Exp>, DAE::Properties)> {
    let mut outCache: FCore::Cache;
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    let mut outProperties: DAE::Properties;
    let mut ty: metamodelica::Ref<DAE::Type>;
    let mut exp_ty: metamodelica::Ref<DAE::Type>;
    let mut c: DAE::Const;
    let mut pre_str: ArcStr;
    let mut msg: Absyn::Msg;
    let mut sz: i32;
    let mut dim_size: metamodelica::Ref<DAE::Dimension>;
    let mut dim_exp: metamodelica::Ref<DAE::Exp>;
    let mut check_model: bool;
    checkBuiltinCallArgs(inPosArgs, inNamedArgs, 1, literal!("identity"), &inInfo)?;
    (outCache, dim_exp, outProperties) = elabExpInExpression(
        inCache,
        inEnv.clone(),
        (inPosArgs).head().cloned()?,
        inImplicit,
        true,
        inPrefix.clone(),
        inInfo.clone(),
    )?;
    let DAE::PROP {
        type_: __pa0,
        constFlag: __pa1,
    } = (outProperties)
    else {
        return Err("pattern mismatch");
    };
    ty = metamodelica::Own::own(__pa0);
    c = metamodelica::Own::own(__pa1);
    if !(Types::isScalarInteger(&ty)) {
        pre_str = PrefixUtil::printPrefixStr3(inPrefix)?;
        Error::addSourceMessageAndFail(
            &(Error::ARGUMENT_MUST_BE_INTEGER.clone()),
            list![literal!("First"), literal!("identity"), pre_str],
            &inInfo,
        )?;
        unreachable!("Error.addSourceMessageAndFail always fails — caller-side flow-analysis hint");
    }
    if Types::isParameterOrConstant(c) {
        check_model = Flags::getConfigBool(Flags::CHECK_MODEL.clone())?;
        msg = if (check_model) {
            openmodelica_ast::Absyn::Msg::NO_MSG
        } else {
            Absyn::Msg::MSG { info: inInfo }
        };
        match '__try2: {
            let (__pa3, __pa4) = ::match_deref::match_deref! { match &(unwrap_break_err!(Ceval::ceval(outCache.clone(), inEnv.clone(), dim_exp.clone(), false, msg.clone(), 0), '__try2)) {
                (__pa3, Deref @ Values::Value::INTEGER { integer: __pa4 }) => (__pa3.clone(), __pa4.clone()),
                _ => break '__try2 Err::<_, _>("pattern mismatch"),
            } };
            outCache = metamodelica::Own::own(__pa3);
            sz = metamodelica::Own::own(__pa4);
            dim_size = metamodelica::Ref::new(DAE::Dimension::DIM_INTEGER { integer: sz });
            dim_exp = metamodelica::Ref::new(DAE::Exp::ICONST { integer: sz });
            Ok::<_, &'static str>((dim_size.clone(),))
        } {
            Ok((__try2_o0,)) => {
                dim_size = __try2_o0;
            }
            Err(_) => {
                if check_model {
                    dim_size = openmodelica_frontend_types::DAE::Dimension::interned_DIM_UNKNOWN();
                } else {
                    return Err("fail");
                }
            }
        }
    } else {
        dim_size = openmodelica_frontend_types::DAE::Dimension::interned_DIM_UNKNOWN();
    }
    ty = Types::liftArrayListDims(DAE::T_INTEGER_DEFAULT().clone(), list![dim_size.clone(), dim_size]);
    exp_ty = Types::simplifyType(ty.clone())?;
    outExp = Expression::makePureBuiltinCall(literal!("identity"), list![dim_exp], exp_ty);
    outProperties = DAE::Properties::PROP {
        type_: ty,
        constFlag: c,
    };
    Ok((outCache, outExp, outProperties))
}

fn zeroSizeOverconstrainedOperator(
    mut inExp: &metamodelica::Ref<DAE::Exp>,
    mut inFExp: metamodelica::Ref<DAE::Exp>,
    mut inInfo: &SourceInfo,
) -> Result<()> {
    let () = (::match_deref::match_deref! { match inExp {
        Deref @ DAE::Exp::ARRAY { array: Deref @ metamodelica::ListNode::Nil, .. } => {
            let mut s: ArcStr;
            s = ExpressionBasics::printExpStr(inFExp)?;
            Error::addSourceMessage(&(Error::OVERCONSTRAINED_OPERATOR_SIZE_ZERO_RETURN_FALSE.clone()), list![s], inInfo)?;
            ()
        },
        _ => {
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(())
}

fn elabBuiltinIsRoot(
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
    mut inPosArgs: &metamodelica::List<metamodelica::Ref<Absyn::Exp>>,
    mut inNamedArgs: &metamodelica::List<metamodelica::Ref<Absyn::NamedArg>>,
    mut inImplicit: bool,
    mut inPrefix: DAE::Prefix,
    mut inInfo: SourceInfo,
) -> Result<(FCore::Cache, metamodelica::Ref<DAE::Exp>, DAE::Properties)> {
    let mut outCache: FCore::Cache;
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    let mut outProperties: DAE::Properties;
    let mut exp: metamodelica::Ref<DAE::Exp>;
    checkBuiltinCallArgs(inPosArgs, inNamedArgs, 1, literal!("Connections.isRoot"), &inInfo)?;
    (outCache, exp, _) = elabExpInExpression(
        inCache,
        inEnv,
        (inPosArgs).head().cloned()?,
        false,
        false,
        inPrefix,
        inInfo.clone(),
    )?;
    outExp = metamodelica::Ref::new(DAE::Exp::CALL {
        path: metamodelica::Ref::new(Absyn::Path::QUALIFIED {
            name: literal!("Connections"),
            path: metamodelica::Ref::new(Absyn::Path::IDENT {
                name: literal!("isRoot"),
            }),
        }),
        expLst: list![exp.clone()],
        attr: DAE::callAttrBuiltinBool().clone(),
    });
    outProperties = DAE::Properties::PROP {
        type_: DAE::T_BOOL_DEFAULT().clone(),
        constFlag: openmodelica_frontend_types::DAE::Const::C_VAR,
    };
    zeroSizeOverconstrainedOperator(&exp, outExp.clone(), &inInfo)?;
    Ok((outCache, outExp, outProperties))
}

fn elabBuiltinRooted(
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
    mut inPosArgs: &metamodelica::List<metamodelica::Ref<Absyn::Exp>>,
    mut inNamedArgs: &metamodelica::List<metamodelica::Ref<Absyn::NamedArg>>,
    mut inImplicit: bool,
    mut inPrefix: DAE::Prefix,
    mut inInfo: SourceInfo,
) -> Result<(FCore::Cache, metamodelica::Ref<DAE::Exp>, DAE::Properties)> {
    let mut outCache: FCore::Cache;
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    let mut outProperties: DAE::Properties;
    let mut exp: metamodelica::Ref<DAE::Exp>;
    checkBuiltinCallArgs(inPosArgs, inNamedArgs, 1, literal!("rooted"), &inInfo)?;
    (outCache, exp, _) = elabExpInExpression(
        inCache,
        inEnv,
        (inPosArgs).head().cloned()?,
        false,
        false,
        inPrefix,
        inInfo.clone(),
    )?;
    outExp = metamodelica::Ref::new(DAE::Exp::CALL {
        path: metamodelica::Ref::new(Absyn::Path::IDENT {
            name: literal!("rooted"),
        }),
        expLst: list![exp.clone()],
        attr: DAE::callAttrBuiltinBool().clone(),
    });
    outProperties = DAE::Properties::PROP {
        type_: DAE::T_BOOL_DEFAULT().clone(),
        constFlag: openmodelica_frontend_types::DAE::Const::C_VAR,
    };
    zeroSizeOverconstrainedOperator(&exp, outExp.clone(), &inInfo)?;
    Ok((outCache, outExp, outProperties))
}

fn elabBuiltinUniqueRootIndices(
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
    mut inAbsynExpLst: &metamodelica::List<metamodelica::Ref<Absyn::Exp>>,
    mut inNamedArg: &metamodelica::List<metamodelica::Ref<Absyn::NamedArg>>,
    mut inBoolean: bool,
    mut inPrefix: DAE::Prefix,
    mut info: SourceInfo,
) -> Result<(FCore::Cache, metamodelica::Ref<DAE::Exp>, DAE::Properties)> {
    let mut outCache: FCore::Cache;
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    let mut outProperties: DAE::Properties;
    (outCache, outExp, outProperties) = (::match_deref::match_deref! { match (inAbsynExpLst, inNamedArg) {
        (Deref @ metamodelica::ListNode::Cons { head: aexp1, tail: Deref @ metamodelica::ListNode::Cons { head: aexp2, tail: Deref @ metamodelica::ListNode::Nil } }, Deref @ metamodelica::ListNode::Nil) => {
            let mut cache = inCache;
            let mut env = inEnv;
            let mut pre = inPrefix;
            let mut exp1: metamodelica::Ref<DAE::Exp>;
            let mut exp2: metamodelica::Ref<DAE::Exp>;
            let mut exp3: metamodelica::Ref<DAE::Exp>;
            let mut lst: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
            let mut dim: i32;
            let mut ty: metamodelica::Ref<DAE::Type>;
            let (__pa0, __pa2, __pa1) = ::match_deref::match_deref! { match &(elabExpInExpression(cache, env.clone(), aexp1.clone(), false, false, pre.clone(), info.clone())?) {
                (__pa0, __pa2 @ Deref @ DAE::Exp::ARRAY { array: __pa1, .. }, _) => (__pa0.clone(), __pa2.clone(), __pa1.clone()),
                _ => return Err("pattern mismatch"),
            } };
            cache = metamodelica::Own::own(__pa0);
            lst = metamodelica::Own::own(__pa1);
            exp1 = metamodelica::Own::own(__pa2);
            dim = ((lst).len() as i32);
            (cache, exp2, _) = elabExpInExpression(cache, env, aexp2.clone(), false, false, pre, info)?;
            exp3 = metamodelica::Ref::new(DAE::Exp::SCONST { string: literal!("") });
            ty = metamodelica::Ref::new(DAE::Type::T_ARRAY { ty: DAE::T_INTEGER_DEFAULT().clone(), dims: list![metamodelica::Ref::new(DAE::Dimension::DIM_INTEGER { integer: dim })] });
            (cache, metamodelica::Ref::new(DAE::Exp::CALL { path: metamodelica::Ref::new(Absyn::Path::QUALIFIED { name: literal!("Connections"), path: metamodelica::Ref::new(Absyn::Path::IDENT { name: literal!("uniqueRootIndices") }) }), expLst: list![exp1, exp2, exp3], attr: metamodelica::Ref::new(DAE::CallAttributes { ty: ty.clone(), tuple_: false, builtin: true, isImpure: false, isFunctionPointerCall: false, inlineType: openmodelica_frontend_types::DAE::InlineType::NO_INLINE, tailCall: openmodelica_frontend_types::DAE::TailCall::NO_TAIL, noReturn: DAE::NoReturn::RETURNS.clone() }) }), DAE::Properties::PROP { type_: ty, constFlag: openmodelica_frontend_types::DAE::Const::C_VAR })
        },
        (Deref @ metamodelica::ListNode::Cons { head: aexp1, tail: Deref @ metamodelica::ListNode::Cons { head: aexp2, tail: Deref @ metamodelica::ListNode::Cons { head: _, tail: Deref @ metamodelica::ListNode::Nil } } }, Deref @ metamodelica::ListNode::Nil) => {
            let mut cache = inCache;
            let mut env = inEnv;
            let mut pre = inPrefix;
            let mut exp1: metamodelica::Ref<DAE::Exp>;
            let mut exp2: metamodelica::Ref<DAE::Exp>;
            let mut exp3: metamodelica::Ref<DAE::Exp>;
            let mut lst: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
            let mut dim: i32;
            let mut ty: metamodelica::Ref<DAE::Type>;
            let (__pa0, __pa2, __pa1) = ::match_deref::match_deref! { match &(elabExpInExpression(cache, env.clone(), aexp1.clone(), false, false, pre.clone(), info.clone())?) {
                (__pa0, __pa2 @ Deref @ DAE::Exp::ARRAY { array: __pa1, .. }, _) => (__pa0.clone(), __pa2.clone(), __pa1.clone()),
                _ => return Err("pattern mismatch"),
            } };
            cache = metamodelica::Own::own(__pa0);
            lst = metamodelica::Own::own(__pa1);
            exp1 = metamodelica::Own::own(__pa2);
            dim = ((lst).len() as i32);
            (cache, exp2, _) = elabExpInExpression(cache, env.clone(), aexp2.clone(), false, false, pre.clone(), info.clone())?;
            (cache, exp3, _) = elabExpInExpression(cache, env, aexp2.clone(), false, false, pre, info)?;
            ty = metamodelica::Ref::new(DAE::Type::T_ARRAY { ty: DAE::T_INTEGER_DEFAULT().clone(), dims: list![metamodelica::Ref::new(DAE::Dimension::DIM_INTEGER { integer: dim })] });
            (cache, metamodelica::Ref::new(DAE::Exp::CALL { path: metamodelica::Ref::new(Absyn::Path::QUALIFIED { name: literal!("Connections"), path: metamodelica::Ref::new(Absyn::Path::IDENT { name: literal!("uniqueRootIndices") }) }), expLst: list![exp1, exp2, exp3], attr: metamodelica::Ref::new(DAE::CallAttributes { ty: ty.clone(), tuple_: false, builtin: true, isImpure: false, isFunctionPointerCall: false, inlineType: openmodelica_frontend_types::DAE::InlineType::NO_INLINE, tailCall: openmodelica_frontend_types::DAE::TailCall::NO_TAIL, noReturn: DAE::NoReturn::RETURNS.clone() }) }), DAE::Properties::PROP { type_: ty, constFlag: openmodelica_frontend_types::DAE::Const::C_VAR })
        },
        (Deref @ metamodelica::ListNode::Cons { head: aexp1, tail: Deref @ metamodelica::ListNode::Cons { head: aexp2, tail: Deref @ metamodelica::ListNode::Nil } }, Deref @ metamodelica::ListNode::Cons { head: Deref @ Absyn::NamedArg { argName: Deref @ "message", argValue: _ }, tail: Deref @ metamodelica::ListNode::Nil }) => {
            let mut cache = inCache;
            let mut env = inEnv;
            let mut pre = inPrefix;
            let mut exp1: metamodelica::Ref<DAE::Exp>;
            let mut exp2: metamodelica::Ref<DAE::Exp>;
            let mut exp3: metamodelica::Ref<DAE::Exp>;
            let mut lst: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
            let mut dim: i32;
            let mut ty: metamodelica::Ref<DAE::Type>;
            let (__pa0, __pa2, __pa1) = ::match_deref::match_deref! { match &(elabExpInExpression(cache, env.clone(), aexp1.clone(), false, false, pre.clone(), info.clone())?) {
                (__pa0, __pa2 @ Deref @ DAE::Exp::ARRAY { array: __pa1, .. }, _) => (__pa0.clone(), __pa2.clone(), __pa1.clone()),
                _ => return Err("pattern mismatch"),
            } };
            cache = metamodelica::Own::own(__pa0);
            lst = metamodelica::Own::own(__pa1);
            exp1 = metamodelica::Own::own(__pa2);
            dim = ((lst).len() as i32);
            (cache, exp2, _) = elabExpInExpression(cache, env.clone(), aexp2.clone(), false, false, pre.clone(), info.clone())?;
            (cache, exp3, _) = elabExpInExpression(cache, env, aexp2.clone(), false, false, pre, info)?;
            ty = metamodelica::Ref::new(DAE::Type::T_ARRAY { ty: DAE::T_INTEGER_DEFAULT().clone(), dims: list![metamodelica::Ref::new(DAE::Dimension::DIM_INTEGER { integer: dim })] });
            (cache, metamodelica::Ref::new(DAE::Exp::CALL { path: metamodelica::Ref::new(Absyn::Path::QUALIFIED { name: literal!("Connections"), path: metamodelica::Ref::new(Absyn::Path::IDENT { name: literal!("uniqueRootIndices") }) }), expLst: list![exp1, exp2, exp3], attr: metamodelica::Ref::new(DAE::CallAttributes { ty: ty.clone(), tuple_: false, builtin: true, isImpure: false, isFunctionPointerCall: false, inlineType: openmodelica_frontend_types::DAE::InlineType::NO_INLINE, tailCall: openmodelica_frontend_types::DAE::TailCall::NO_TAIL, noReturn: DAE::NoReturn::RETURNS.clone() }) }), DAE::Properties::PROP { type_: ty, constFlag: openmodelica_frontend_types::DAE::Const::C_VAR })
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok((outCache, outExp, outProperties))
}

fn elabBuiltinScalar(
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
    mut inPosArgs: &metamodelica::List<metamodelica::Ref<Absyn::Exp>>,
    mut inNamedArgs: &metamodelica::List<metamodelica::Ref<Absyn::NamedArg>>,
    mut inImplicit: bool,
    mut inPrefix: DAE::Prefix,
    mut inInfo: SourceInfo,
) -> Result<(FCore::Cache, metamodelica::Ref<DAE::Exp>, DAE::Properties)> {
    let mut outCache: FCore::Cache;
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    let mut outProperties: DAE::Properties;
    let mut ty: metamodelica::Ref<DAE::Type>;
    let mut scalar_ty: metamodelica::Ref<DAE::Type>;
    let mut c: DAE::Const;
    let mut dims: metamodelica::List<metamodelica::Ref<DAE::Dimension>>;
    let mut ty_str: ArcStr;
    checkBuiltinCallArgs(inPosArgs, inNamedArgs, 1, literal!("scalar"), &inInfo)?;
    let (__pa0, __pa1, __pa2, __pa3) = ::match_deref::match_deref! { match &(elabExpInExpression(inCache, inEnv, (inPosArgs).head().cloned()?, inImplicit, true, inPrefix, inInfo.clone())?) {
        (__pa0, __pa1, DAE::Properties::PROP { type_: __pa2, constFlag: __pa3 }) => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone()),
        _ => return Err("pattern mismatch"),
    } };
    outCache = metamodelica::Own::own(__pa0);
    outExp = metamodelica::Own::own(__pa1);
    ty = metamodelica::Own::own(__pa2);
    c = metamodelica::Own::own(__pa3);
    (scalar_ty, dims) = TypesDump::flattenArrayType(&ty);
    for mut dim in &*dims {
        if Expression::dimensionKnown(metamodelica::AsArg::as_arg(&dim))
            && Expression::dimensionSize(metamodelica::AsArg::as_arg(&dim))? != 1
        {
            ty_str = TypesDump::unparseTypeNoAttr(&ty)?;
            Error::addSourceMessageAndFail(
                &(Error::INVALID_ARRAY_DIM_IN_CONVERSION_OP.clone()),
                list![ty_str],
                &inInfo,
            )?;
            unreachable!("Error.addSourceMessageAndFail always fails — caller-side flow-analysis hint");
        }
    }
    if !((dims).is_empty()) {
        outExp = Expression::makePureBuiltinCall(literal!("scalar"), list![outExp], scalar_ty.clone());
    }
    (outExp, _) = ExpressionSimplify::simplify1(outExp)?;
    outProperties = DAE::Properties::PROP {
        type_: scalar_ty,
        constFlag: c,
    };
    Ok((outCache, outExp, outProperties))
}

thread_local! { static __STRING_ARG_MINLENGTH_TLS: Slot = Slot { defaultArg: metamodelica::Ref::new(DAE::FuncArg { name: literal!("minimumLength"), ty: DAE::T_INTEGER_DEFAULT().clone(), r#const: openmodelica_frontend_types::DAE::Const::C_VAR, par: openmodelica_frontend_types::DAE::VarParallelism::NON_PARALLEL, defaultBinding: None }), slotFilled: false, arg: Some(metamodelica::Ref::new(DAE::Exp::ICONST { integer: 0 })), dims: metamodelica::nil(), idx: 2, evalStatus: SLOT_NOT_EVALUATED.clone() }; }
pub(crate) fn STRING_ARG_MINLENGTH() -> Slot {
    __STRING_ARG_MINLENGTH_TLS.with(|__t| __t.clone())
}

thread_local! { static __STRING_ARG_LEFTJUSTIFIED_TLS: Slot = Slot { defaultArg: metamodelica::Ref::new(DAE::FuncArg { name: literal!("leftJustified"), ty: DAE::T_BOOL_DEFAULT().clone(), r#const: openmodelica_frontend_types::DAE::Const::C_VAR, par: openmodelica_frontend_types::DAE::VarParallelism::NON_PARALLEL, defaultBinding: None }), slotFilled: false, arg: Some(metamodelica::Ref::new(DAE::Exp::BCONST { bool: true })), dims: metamodelica::nil(), idx: 3, evalStatus: SLOT_NOT_EVALUATED.clone() }; }
pub(crate) fn STRING_ARG_LEFTJUSTIFIED() -> Slot {
    __STRING_ARG_LEFTJUSTIFIED_TLS.with(|__t| __t.clone())
}

thread_local! { static __STRING_ARG_SIGNIFICANT_DIGITS_TLS: Slot = Slot { defaultArg: metamodelica::Ref::new(DAE::FuncArg { name: literal!("significantDigits"), ty: DAE::T_INTEGER_DEFAULT().clone(), r#const: openmodelica_frontend_types::DAE::Const::C_VAR, par: openmodelica_frontend_types::DAE::VarParallelism::NON_PARALLEL, defaultBinding: None }), slotFilled: false, arg: Some(metamodelica::Ref::new(DAE::Exp::ICONST { integer: 6 })), dims: metamodelica::nil(), idx: 4, evalStatus: SLOT_NOT_EVALUATED.clone() }; }
pub(crate) fn STRING_ARG_SIGNIFICANT_DIGITS() -> Slot {
    __STRING_ARG_SIGNIFICANT_DIGITS_TLS.with(|__t| __t.clone())
}

fn elabBuiltinString(
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
    mut inPosArgs: metamodelica::List<metamodelica::Ref<Absyn::Exp>>,
    mut inNamedArgs: metamodelica::List<metamodelica::Ref<Absyn::NamedArg>>,
    mut inImplicit: bool,
    mut inPrefix: DAE::Prefix,
    mut inInfo: SourceInfo,
) -> Result<(FCore::Cache, metamodelica::Ref<DAE::Exp>, DAE::Properties)> {
    let mut outCache: FCore::Cache;
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    let mut outProperties: DAE::Properties;
    let mut e: metamodelica::Ref<Absyn::Exp>;
    let mut exp: metamodelica::Ref<DAE::Exp>;
    let mut ty: metamodelica::Ref<DAE::Type>;
    let mut c: DAE::Const;
    let mut args: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
    let mut consts: metamodelica::List<DAE::Const>;
    let mut val_slot: Slot;
    let mut format_arg: Option<metamodelica::Ref<DAE::Exp>> = None;
    let mut slots: metamodelica::List<Slot>;
    match '__try0: {
        e = metamodelica::Ref::new(Absyn::Exp::CALL {
            function_: metamodelica::Ref::new(Absyn::ComponentRef::CREF_IDENT {
                name: literal!("String"),
                subscripts: metamodelica::nil(),
            }),
            functionArgs: metamodelica::Ref::new(Absyn::FunctionArgs::FUNCTIONARGS {
                args: inPosArgs.clone(),
                argNames: inNamedArgs.clone(),
            }),
            typeVars: metamodelica::nil(),
        });
        (outCache, outExp, outProperties) = unwrap_break_err!(OperatorOverloading::string(inCache.clone(), inEnv.clone(), &e, inImplicit, true, inPrefix.clone(), inInfo.clone()), '__try0);
        Ok::<_, &'static str>((e.clone(), outCache.clone(), outExp.clone(), outProperties.clone()))
    } {
        Ok((__try0_o0, __try0_o1, __try0_o2, __try0_o3)) => {
            e = __try0_o0;
            outCache = __try0_o1;
            outExp = __try0_o2;
            outProperties = __try0_o3;
        }
        Err(_) => {
            e = (inPosArgs).head().cloned()?;
            let (__pa1, __pa2, __pa3, __pa4) = ::match_deref::match_deref! { match &(elabExpInExpression(inCache.clone(), inEnv.clone(), e.clone(), inImplicit, true, inPrefix.clone(), inInfo.clone())?) {
                (__pa1, __pa2, DAE::Properties::PROP { type_: __pa3, constFlag: __pa4 }) => (__pa1.clone(), __pa2.clone(), __pa3.clone(), __pa4.clone()),
                _ => return Err("pattern mismatch"),
            } };
            outCache = metamodelica::Own::own(__pa1);
            exp = metamodelica::Own::own(__pa2);
            ty = metamodelica::Own::own(__pa3);
            c = metamodelica::Own::own(__pa4);
            if Types::isMetaBoxedType(&ty) {
                ty = Types::unboxedType(ty.clone())?;
                exp = metamodelica::Ref::new(DAE::Exp::UNBOX {
                    exp: exp.clone(),
                    ty: ty.clone(),
                });
            }
            val_slot = Slot {
                defaultArg: metamodelica::Ref::new(DAE::FuncArg {
                    name: literal!("x"),
                    ty: ty.clone(),
                    r#const: openmodelica_frontend_types::DAE::Const::C_VAR,
                    par: openmodelica_frontend_types::DAE::VarParallelism::NON_PARALLEL,
                    defaultBinding: None,
                }),
                slotFilled: false,
                arg: None,
                dims: metamodelica::nil(),
                idx: 1,
                evalStatus: SLOT_NOT_EVALUATED.clone(),
            };
            match '__try5: {
                slots = list![STRING_ARG_MINLENGTH().clone(), STRING_ARG_LEFTJUSTIFIED().clone()];
                if Types::isRealOrSubTypeReal(ty.clone()) {
                    slots = metamodelica::cons(STRING_ARG_SIGNIFICANT_DIGITS().clone(), slots.clone());
                }
                slots = metamodelica::cons(val_slot.clone(), slots.clone());
                (outCache, args, _, consts, _) = unwrap_break_err!(elabInputArgs(outCache.clone(), inEnv.clone(), &inPosArgs, &inNamedArgs, slots.clone(), false, true, inImplicit, inPrefix.clone(), inInfo.clone(), &(DAE::T_UNKNOWN_DEFAULT().clone()), metamodelica::Ref::new(Absyn::Path::IDENT { name: literal!("String") }), false), '__try5);
                Ok::<_, &'static str>((args.clone(), consts.clone(), outCache.clone(), slots.clone()))
            } {
                Ok((__try5_o0, __try5_o1, __try5_o2, __try5_o3)) => {
                    args = __try5_o0;
                    consts = __try5_o1;
                    outCache = __try5_o2;
                    slots = __try5_o3;
                }
                Err(_) => {
                    if Types::isRealOrSubTypeReal(ty.clone()) {
                        format_arg = Some(metamodelica::Ref::new(DAE::Exp::SCONST { string: literal!("f") }));
                    } else if Types::isIntegerOrSubTypeInteger(ty.clone()) {
                        format_arg = Some(metamodelica::Ref::new(DAE::Exp::SCONST { string: literal!("d") }));
                    } else if Types::isString(&ty) {
                        format_arg = Some(metamodelica::Ref::new(DAE::Exp::SCONST { string: literal!("s") }));
                    } else {
                        format_arg = None;
                    }
                    if (format_arg).is_some() {
                        slots = list![
                            val_slot.clone(),
                            Slot {
                                defaultArg: metamodelica::Ref::new(DAE::FuncArg {
                                    name: literal!("format"),
                                    ty: DAE::T_STRING_DEFAULT().clone(),
                                    r#const: openmodelica_frontend_types::DAE::Const::C_VAR,
                                    par: openmodelica_frontend_types::DAE::VarParallelism::NON_PARALLEL,
                                    defaultBinding: None
                                }),
                                slotFilled: false,
                                arg: format_arg.clone(),
                                dims: metamodelica::nil(),
                                idx: 2,
                                evalStatus: SLOT_NOT_EVALUATED.clone()
                            }
                        ];
                    } else {
                        slots = list![val_slot.clone()];
                    }
                    (outCache, args, _, consts, _) = elabInputArgs(
                        outCache.clone(),
                        inEnv.clone(),
                        &inPosArgs,
                        &inNamedArgs,
                        slots.clone(),
                        false,
                        true,
                        inImplicit,
                        inPrefix.clone(),
                        inInfo.clone(),
                        &(DAE::T_UNKNOWN_DEFAULT().clone()),
                        metamodelica::Ref::new(Absyn::Path::IDENT {
                            name: literal!("String"),
                        }),
                        false,
                    )?;
                }
            }
            c = List::fold(
                &consts,
                &fnptr!(Types::constAnd, DAE::Const, DAE::Const),
                openmodelica_frontend_types::DAE::Const::C_CONST,
            )?;
            outExp = Expression::makePureBuiltinCall(literal!("String"), args.clone(), DAE::T_STRING_DEFAULT().clone());
            outProperties = DAE::Properties::PROP {
                type_: DAE::T_STRING_DEFAULT().clone(),
                constFlag: c,
            };
        }
    }
    Ok((outCache, outExp, outProperties))
}

fn elabBuiltinGetInstanceName(
    mut inCache: FCore::Cache,
    mut inEnv: &FCore::Graph,
    mut inPosArgs: &metamodelica::List<metamodelica::Ref<Absyn::Exp>>,
    mut inNamedArgs: &metamodelica::List<metamodelica::Ref<Absyn::NamedArg>>,
    mut inImplicit: bool,
    mut inPrefix: &DAE::Prefix,
    mut inInfo: &SourceInfo,
) -> Result<(FCore::Cache, metamodelica::Ref<DAE::Exp>, DAE::Properties)> {
    let mut outCache: FCore::Cache = inCache.clone();
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    let mut outProperties: DAE::Properties;
    let mut r#str: ArcStr;
    let mut name: metamodelica::Ref<Absyn::Path>;
    let mut envName: metamodelica::Ref<Absyn::Path>;
    checkBuiltinCallArgs(inPosArgs, inNamedArgs, 0, literal!("getInstanceName"), inInfo)?;
    let FCore::CACHE { modelName: __pa0, .. } = (inCache) else {
        return Err("pattern mismatch");
    };
    name = metamodelica::Own::own(__pa0);
    if PrefixUtil::isNoPrefix(inPrefix) {
        envName = FGraph::getGraphNameNoImplicitScopes(inEnv)?;
        r#str = if (AbsynUtil::pathEqual(&envName, &name)) {
            AbsynUtil::pathLastIdent(&name)
        } else {
            AbsynUtil::pathString(envName, literal!("."), true, false)?
        };
    } else {
        r#str = {
            let mut __mm_s = String::new();
            __mm_s.push_str(&*AbsynUtil::pathLastIdent(&name));
            __mm_s.push_str(&*literal!("."));
            __mm_s.push_str(&*PrefixUtil::printPrefixStr(inPrefix)?);
            ArcStr::from(__mm_s)
        };
    }
    outExp = metamodelica::Ref::new(DAE::Exp::SCONST { string: r#str });
    outProperties = DAE::Properties::PROP {
        type_: DAE::T_STRING_DEFAULT().clone(),
        constFlag: openmodelica_frontend_types::DAE::Const::C_CONST,
    };
    Ok((outCache, outExp, outProperties))
}

fn elabBuiltinIsPresent(
    mut inCache: FCore::Cache,
    mut inEnv: &FCore::Graph,
    mut inPosArgs: &metamodelica::List<metamodelica::Ref<Absyn::Exp>>,
    mut inNamedArgs: &metamodelica::List<metamodelica::Ref<Absyn::NamedArg>>,
    mut inImplicit: bool,
    mut inPrefix: &DAE::Prefix,
    mut info: &SourceInfo,
) -> Result<(FCore::Cache, metamodelica::Ref<DAE::Exp>, DAE::Properties)> {
    let mut outCache: FCore::Cache = inCache;
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    let mut outProperties: DAE::Properties;
    let mut r#str: ArcStr;
    let mut direction: Absyn::Direction;
    let mut exp: metamodelica::Ref<Absyn::Exp>;
    checkBuiltinCallArgs(inPosArgs, inNamedArgs, 1, literal!("isPresent"), info)?;
    if !(FGraph::inFunctionScope(inEnv)) {
        Error::addSourceMessage(
            &(Error::IS_PRESENT_WRONG_SCOPE.clone()),
            list![SCodeDump::restrString(
                &(FGraph::getScopeRestriction(&(FGraph::currentScope(inEnv)))?)
            )?],
            info,
        )?;
    }
    outExp = (::match_deref::match_deref! { match &((inPosArgs).get(1)?) {
        Deref @ Absyn::Exp::CREF { componentRef: Deref @ Absyn::ComponentRef::CREF_IDENT { name: __esc_str, .. } } => {
            r#str = (*__esc_str).clone();
            let (__pa0, __t2, _, _, _, _) = Lookup::lookupIdentLocal(outCache, inEnv, r#str.clone())?;
            let __arc4 = __t2.clone();
            let DAE::TYPES_VAR { attributes: __t3, .. } = &*__arc4;
            let __arc5 = __t3.clone();
            let DAE::ATTR { direction: __pa1, .. } = &*__arc5;
            outCache = metamodelica::Own::own(__pa0);
            direction = metamodelica::Own::own(__pa1);
            let () = (match direction {
        Absyn::Direction::BIDIR { .. } => {
            Error::addSourceMessage(&(Error::IS_PRESENT_WRONG_DIRECTION.clone()), metamodelica::nil(), info)?;
            return Err("fail")
        },
        _ => (),
    });
            Expression::makeImpureBuiltinCall(literal!("isPresent"), metamodelica::cons(metamodelica::Ref::new(DAE::Exp::CREF { componentRef: metamodelica::Ref::new(DAE::ComponentRef::CREF_IDENT { ident: r#str.clone(), identType: DAE::T_BOOL_DEFAULT().clone(), subscriptLst: metamodelica::nil() }), ty: DAE::T_BOOL_DEFAULT().clone() }), metamodelica::nil()), DAE::T_BOOL_DEFAULT().clone())
        },
        __esc_exp => {
            exp = (*__esc_exp).clone();
            Error::addSourceMessage(&(Error::IS_PRESENT_INVALID_EXP.clone()), list![Dump::printExpStr(exp.clone())?], info)?;
            return Err("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outProperties = DAE::Properties::PROP {
        type_: DAE::T_BOOL_DEFAULT().clone(),
        constFlag: openmodelica_frontend_types::DAE::Const::C_VAR,
    };
    Ok((outCache, outExp, outProperties))
}

fn elabBuiltinVector(
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
    mut inPosArgs: &metamodelica::List<metamodelica::Ref<Absyn::Exp>>,
    mut inNamedArgs: &metamodelica::List<metamodelica::Ref<Absyn::NamedArg>>,
    mut inImplicit: bool,
    mut inPrefix: DAE::Prefix,
    mut inInfo: SourceInfo,
) -> Result<(FCore::Cache, metamodelica::Ref<DAE::Exp>, DAE::Properties)> {
    let mut outCache: FCore::Cache;
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    let mut outProperties: DAE::Properties;
    let mut e: metamodelica::Ref<Absyn::Exp>;
    let mut ty: metamodelica::Ref<DAE::Type>;
    let mut arr_ty: metamodelica::Ref<DAE::Type>;
    let mut exp_ty: metamodelica::Ref<DAE::Type>;
    let mut el_ty: metamodelica::Ref<DAE::Type>;
    let mut c: DAE::Const;
    let mut expl: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
    checkBuiltinCallArgs(inPosArgs, inNamedArgs, 1, literal!("vector"), &inInfo)?;
    e = (inPosArgs).head().cloned()?;
    let (__pa0, __pa1, __pa4, __pa2, __pa3) = ::match_deref::match_deref! { match &(elabExpInExpression(inCache, inEnv.clone(), e.clone(), inImplicit, true, inPrefix.clone(), inInfo.clone())?) {
        (__pa0, __pa1, __pa4 @ DAE::Properties::PROP { type_: __pa2, constFlag: __pa3 }) => (__pa0.clone(), __pa1.clone(), __pa4.clone(), __pa2.clone(), __pa3.clone()),
        _ => return Err("pattern mismatch"),
    } };
    outCache = metamodelica::Own::own(__pa0);
    outExp = metamodelica::Own::own(__pa1);
    ty = metamodelica::Own::own(__pa2);
    c = metamodelica::Own::own(__pa3);
    outProperties = metamodelica::Own::own(__pa4);
    if Types::isSimpleType(&ty) {
        arr_ty = Types::liftArray(ty, metamodelica::Ref::new(DAE::Dimension::DIM_INTEGER { integer: 1 }));
        exp_ty = Types::simplifyType(arr_ty.clone())?;
        outExp = metamodelica::Ref::new(DAE::Exp::ARRAY {
            ty: exp_ty,
            scalar: true,
            array: list![outExp],
        });
        outProperties = DAE::Properties::PROP {
            type_: arr_ty,
            constFlag: c,
        };
    } else if Expression::isArray(&outExp) || Expression::isMatrix(&outExp) {
        if Types::numberOfDimensions(&ty) != 1 {
            checkBuiltinVectorDims(e, &inEnv, &ty, inPrefix, &inInfo)?;
            expl = Expression::getArrayOrMatrixContents(&outExp)?;
            expl = flattenArray(&expl);
            el_ty = Types::arrayElementType(&ty);
            arr_ty = Types::liftArray(
                el_ty,
                metamodelica::Ref::new(DAE::Dimension::DIM_INTEGER {
                    integer: ((expl).len() as i32),
                }),
            );
            outExp = metamodelica::Ref::new(DAE::Exp::ARRAY {
                ty: Types::simplifyType(arr_ty.clone())?,
                scalar: false,
                array: expl,
            });
            outProperties = DAE::Properties::PROP {
                type_: arr_ty,
                constFlag: c,
            };
        }
    } else {
        ty = Types::liftArray(
            Types::arrayElementType(&ty),
            openmodelica_frontend_types::DAE::Dimension::interned_DIM_UNKNOWN(),
        );
        exp_ty = Types::simplifyType(ty.clone())?;
        outExp = Expression::makePureBuiltinCall(literal!("vector"), list![outExp], exp_ty);
        outProperties = DAE::Properties::PROP {
            type_: ty,
            constFlag: c,
        };
    }
    Ok((outCache, outExp, outProperties))
}

fn checkBuiltinVectorDims(
    mut inExp: metamodelica::Ref<Absyn::Exp>,
    mut inEnv: &FCore::Graph,
    mut inType: &metamodelica::Ref<DAE::Type>,
    mut inPrefix: DAE::Prefix,
    mut inInfo: &SourceInfo,
) -> Result<()> {
    let mut found_dim_sz_one: bool = false;
    let mut dims: metamodelica::List<i32>;
    let mut arg_str: ArcStr;
    let mut scope_str: ArcStr;
    let mut dim_str: ArcStr;
    let mut pre_str: ArcStr;
    dims = Types::getDimensionSizes(inType)?;
    for mut dim in &*dims {
        if dim.clone() > 1 {
            if found_dim_sz_one {
                scope_str = FGraph::printGraphPathStr(inEnv);
                arg_str = {
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!("vector("));
                    __mm_s.push_str(&*Dump::printExpStr(inExp.clone())?);
                    __mm_s.push_str(&*literal!(")"));
                    ArcStr::from(__mm_s)
                };
                dim_str = {
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!("["));
                    __mm_s.push_str(&*stringDelimitList(
                        ({
                            let mut __acc: metamodelica::List<ArcStr> = metamodelica::nil();
                            for mut d in (dims.clone()).into_iter().cloned() {
                                let __x = intString(d.clone());
                                __acc = cons(__x, __acc);
                            }
                            __acc.reverse()
                        }),
                        literal!(", "),
                    ));
                    __mm_s.push_str(&*literal!("]"));
                    ArcStr::from(__mm_s)
                };
                pre_str = PrefixUtil::printPrefixStr3(inPrefix.clone())?;
                Error::addSourceMessageAndFail(
                    &(Error::BUILTIN_VECTOR_INVALID_DIMENSIONS.clone()),
                    list![scope_str, pre_str, dim_str, arg_str],
                    inInfo,
                )?;
                unreachable!("Error.addSourceMessageAndFail always fails — caller-side flow-analysis hint");
            } else {
                found_dim_sz_one = true;
            }
        }
    }
    Ok(())
}

fn flattenArray(
    mut arr: &metamodelica::List<metamodelica::Ref<DAE::Exp>>,
) -> metamodelica::List<metamodelica::Ref<DAE::Exp>> {
    let mut flattenedExpl: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
    flattenedExpl = (::match_deref::match_deref! { match arr {
        Deref @ metamodelica::ListNode::Nil => {
            metamodelica::nil()
        },
        Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Exp::ARRAY { array: expl, .. }, tail: rest_expl } => {
            let mut expl2: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
            let mut expl = (*expl).clone();
            expl = flattenArray(metamodelica::AsArg::as_arg(&expl));
            expl2 = flattenArray(rest_expl);
            expl2 = listAppend(expl.clone(), expl2);
            expl2
        },
        Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Exp::MATRIX { matrix: Deref @ metamodelica::ListNode::Cons { head: Deref @ metamodelica::ListNode::Cons { head: e, tail: Deref @ metamodelica::ListNode::Nil }, tail: Deref @ metamodelica::ListNode::Nil }, .. }, tail: rest_expl } => {
            let mut expl: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
            expl = flattenArray(rest_expl);
            metamodelica::cons(e.clone(), expl)
        },
        Deref @ metamodelica::ListNode::Cons { head: e, tail: expl } => {
            let mut expl = (*expl).clone();
            expl = flattenArray(metamodelica::AsArg::as_arg(&expl));
            metamodelica::cons(e.clone(), expl.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    flattenedExpl
}

pub(crate) fn elabBuiltinMatrix(
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
    mut inPosArgs: &metamodelica::List<metamodelica::Ref<Absyn::Exp>>,
    mut inNamedArgs: &metamodelica::List<metamodelica::Ref<Absyn::NamedArg>>,
    mut inImpl: bool,
    mut inPrefix: DAE::Prefix,
    mut inInfo: SourceInfo,
) -> Result<(FCore::Cache, metamodelica::Ref<DAE::Exp>, DAE::Properties)> {
    let mut outCache: FCore::Cache;
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    let mut outProperties: DAE::Properties;
    let mut ty: metamodelica::Ref<DAE::Type>;
    checkBuiltinCallArgs(inPosArgs, inNamedArgs, 1, literal!("matrix"), &inInfo)?;
    (outCache, outExp, outProperties) = elabExpInExpression(
        inCache.clone(),
        inEnv.clone(),
        (inPosArgs).head().cloned()?,
        inImpl,
        true,
        inPrefix,
        inInfo.clone(),
    )?;
    ty = Types::getPropType(&outProperties);
    (outExp, outProperties) = elabBuiltinMatrix2(&inCache, &inEnv, outExp, outProperties, &ty, inInfo)?;
    Ok((outCache, outExp, outProperties))
}

fn elabBuiltinMatrix2(
    mut inCache: &FCore::Cache,
    mut inEnv: &FCore::Graph,
    mut inArg: metamodelica::Ref<DAE::Exp>,
    mut inProperties: DAE::Properties,
    mut inType: &metamodelica::Ref<DAE::Type>,
    mut inInfo: SourceInfo,
) -> Result<(metamodelica::Ref<DAE::Exp>, DAE::Properties)> {
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    let mut outProperties: DAE::Properties;
    (outExp, outProperties) = (::match_deref::match_deref! { match &(inArg.clone()) {
        _ if (Types::isSimpleType(inType)) => {
            let mut exp: metamodelica::Ref<DAE::Exp>;
            let mut props: DAE::Properties;
            (exp, props) = promoteExp(inArg, inProperties, 2)?;
            (exp, props)
        },
        _ if (Types::numberOfDimensions(inType) == 1) => {
            let mut exp: metamodelica::Ref<DAE::Exp>;
            let mut props: DAE::Properties;
            (exp, props) = promoteExp(inArg, inProperties, 2)?;
            (exp, props)
        },
        Deref @ DAE::Exp::MATRIX { .. } => {
            (inArg, inProperties)
        },
        Deref @ DAE::Exp::ARRAY { ty: Deref @ DAE::Type::T_ARRAY { ty: ety, dims: Deref @ metamodelica::ListNode::Cons { head: dim1, tail: Deref @ metamodelica::ListNode::Cons { head: dim2, tail: _ } } }, scalar, array: expl } => {
            let mut ty: metamodelica::Ref<DAE::Type>;
            let mut props: DAE::Properties;
            let mut expl = (*expl).clone();
            expl = List::map1(expl.clone(), &move |__a0: metamodelica::Ref<DAE::Exp>, __a1: SourceInfo| elabBuiltinMatrix3(&__a0, &__a1), inInfo)?;
            ty = Types::arrayElementType(inType);
            ty = Types::liftArrayListDims(ty, list![dim1.clone(), dim2.clone()]);
            props = Types::setPropType(&inProperties, ty);
            (metamodelica::Ref::new(DAE::Exp::ARRAY { ty: metamodelica::Ref::new(DAE::Type::T_ARRAY { ty: ety.clone(), dims: list![dim1.clone(), dim2.clone()] }), scalar: scalar.clone(), array: expl.clone() }), props)
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok((outExp, outProperties))
}

fn elabBuiltinMatrix3(
    mut inExp: &metamodelica::Ref<DAE::Exp>,
    mut inInfo: &SourceInfo,
) -> Result<metamodelica::Ref<DAE::Exp>> {
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    outExp = (::match_deref::match_deref! { match inExp {
        Deref @ DAE::Exp::ARRAY { ty: Deref @ DAE::Type::T_ARRAY { ty: ety, dims: Deref @ metamodelica::ListNode::Cons { head: dim, tail: _ } }, scalar, array: expl } => {
            let mut expl = (*expl).clone();
            expl = ({
        let mut __acc: metamodelica::List<metamodelica::Ref<DAE::Exp>> = metamodelica::nil();
        for mut e in (expl.clone()).into_iter().cloned() {
            let __x = arrayScalar(e.clone(), 3, &(literal!("matrix")), inInfo)?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
            metamodelica::Ref::new(DAE::Exp::ARRAY { ty: metamodelica::Ref::new(DAE::Type::T_ARRAY { ty: ety.clone(), dims: list![dim.clone()] }), scalar: scalar.clone(), array: expl.clone() })
        },
        Deref @ DAE::Exp::MATRIX { ty: Deref @ DAE::Type::T_ARRAY { ty: ety, dims: Deref @ metamodelica::ListNode::Cons { head: dim, tail: dims } }, matrix: matrix_expl, .. } => {
            let mut ety2: metamodelica::Ref<DAE::Type>;
            let mut expl: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
            ety2 = metamodelica::Ref::new(DAE::Type::T_ARRAY { ty: ety.clone(), dims: dims.clone() });
            expl = ({
        let mut __acc: metamodelica::List<metamodelica::Ref<DAE::Exp>> = metamodelica::nil();
        for mut e in (matrix_expl.clone()).into_iter().cloned() {
            let __x = Expression::makeArray(e.clone(), ety2.clone(), true);
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
            expl = ({
        let mut __acc: metamodelica::List<metamodelica::Ref<DAE::Exp>> = metamodelica::nil();
        for mut e in (expl).into_iter().cloned() {
            let __x = arrayScalar(e.clone(), 3, &(literal!("matrix")), inInfo)?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
            metamodelica::Ref::new(DAE::Exp::ARRAY { ty: metamodelica::Ref::new(DAE::Type::T_ARRAY { ty: ety.clone(), dims: list![dim.clone()] }), scalar: true, array: expl })
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok(outExp)
}

fn arrayScalar<'__b>(
    mut inExp: metamodelica::Ref<DAE::Exp>,
    mut inDim: i32,
    mut inOperator: &'__b ArcStr,
    mut inInfo: &'__b SourceInfo,
) -> Result<metamodelica::Ref<DAE::Exp>> {
    '__tco: loop {
        ::match_deref::match_deref! { match &(inExp.clone()) {
            Deref @ DAE::Exp::ARRAY { array: Deref @ metamodelica::ListNode::Cons { head: exp, tail: Deref @ metamodelica::ListNode::Nil }, .. } => {
                { (inExp, inDim, inOperator, inInfo) = (exp.clone(), inDim + 1, inOperator, inInfo); continue '__tco; }
            },
            Deref @ DAE::Exp::ARRAY { array: expl, .. } => {
                let mut dim_str: ArcStr;
                let mut size_str: ArcStr;
                dim_str = intString(inDim);
                size_str = intString(((expl).len() as i32));
                Error::addSourceMessage(&(Error::INVALID_ARRAY_DIM_IN_CONVERSION_OP.clone()), list![dim_str, inOperator.clone(), literal!("1"), size_str], inInfo)?;
                return Ok(return Err("fail"))
            },
            Deref @ DAE::Exp::MATRIX { ty, matrix: Deref @ metamodelica::ListNode::Cons { head: expl, tail: Deref @ metamodelica::ListNode::Nil }, .. } => {
                { (inExp, inDim, inOperator, inInfo) = (metamodelica::Ref::new(DAE::Exp::ARRAY { ty: ty.clone(), scalar: true, array: expl.clone() }), inDim + 1, inOperator, inInfo); continue '__tco; }
            },
            Deref @ DAE::Exp::MATRIX { matrix: mexpl, .. } => {
                let mut dim_str: ArcStr;
                let mut size_str: ArcStr;
                dim_str = intString(inDim);
                size_str = intString(((mexpl).len() as i32));
                Error::addSourceMessage(&(Error::INVALID_ARRAY_DIM_IN_CONVERSION_OP.clone()), list![dim_str, inOperator.clone(), literal!("1"), size_str], inInfo)?;
                return Ok(return Err("fail"))
            },
            _ => {
                return Ok(inExp)
            },
            _ => return Err("match: no arm matched"),
        } }
    }
}

pub(crate) fn elabBuiltinHandler(
    mut inIdent: &ArcStr,
) -> Result<
    Arc<
        dyn ::std::ops::Fn(
                FCore::Cache,
                FCore::Graph,
                metamodelica::List<metamodelica::Ref<Absyn::Exp>>,
                metamodelica::List<metamodelica::Ref<Absyn::NamedArg>>,
                bool,
                DAE::Prefix,
                SourceInfo,
            ) -> Result<(FCore::Cache, metamodelica::Ref<DAE::Exp>, DAE::Properties)>
            + 'static,
    >,
> {
    pub type HandlerFunc = std::sync::Arc<
        dyn ::std::ops::Fn(
                FCore::Cache,
                FCore::Graph,
                metamodelica::List<metamodelica::Ref<Absyn::Exp>>,
                metamodelica::List<metamodelica::Ref<Absyn::NamedArg>>,
                bool,
                DAE::Prefix,
                SourceInfo,
            ) -> Result<(FCore::Cache, metamodelica::Ref<DAE::Exp>, DAE::Properties)>
            + 'static,
    >;

    let mut outHandler: Arc<
        dyn ::std::ops::Fn(
                FCore::Cache,
                FCore::Graph,
                metamodelica::List<metamodelica::Ref<Absyn::Exp>>,
                metamodelica::List<metamodelica::Ref<Absyn::NamedArg>>,
                bool,
                DAE::Prefix,
                SourceInfo,
            ) -> Result<(FCore::Cache, metamodelica::Ref<DAE::Exp>, DAE::Properties)>
            + 'static,
    >;
    outHandler = (::match_deref::match_deref! { match &(inIdent.clone()) {
        Deref @ "smooth" => (std::sync::Arc::new(elabBuiltinSmooth) as std::sync::Arc<dyn ::std::ops::Fn(FCore::Cache, FCore::Graph, metamodelica::List<metamodelica::Ref<Absyn::Exp>>, metamodelica::List<metamodelica::Ref<Absyn::NamedArg>>, bool, DAE::Prefix, SourceInfo) -> Result<(FCore::Cache, metamodelica::Ref<DAE::Exp>, DAE::Properties)> + 'static>),
        Deref @ "size" => (std::sync::Arc::new(move |__a0: FCore::Cache, __a1: FCore::Graph, __a2: metamodelica::List<metamodelica::Ref<Absyn::Exp>>, __a3: metamodelica::List<metamodelica::Ref<Absyn::NamedArg>>, __a4: bool, __a5: DAE::Prefix, __a6: SourceInfo| elabBuiltinSize(__a0, __a1, &__a2, &__a3, __a4, __a5, __a6)) as std::sync::Arc<dyn ::std::ops::Fn(FCore::Cache, FCore::Graph, metamodelica::List<metamodelica::Ref<Absyn::Exp>>, metamodelica::List<metamodelica::Ref<Absyn::NamedArg>>, bool, DAE::Prefix, SourceInfo) -> Result<(FCore::Cache, metamodelica::Ref<DAE::Exp>, DAE::Properties)> + 'static>),
        Deref @ "ndims" => (std::sync::Arc::new(move |__a0: FCore::Cache, __a1: FCore::Graph, __a2: metamodelica::List<metamodelica::Ref<Absyn::Exp>>, __a3: metamodelica::List<metamodelica::Ref<Absyn::NamedArg>>, __a4: bool, __a5: DAE::Prefix, __a6: SourceInfo| elabBuiltinNDims(__a0, __a1, __a2, &__a3, __a4, __a5, __a6)) as std::sync::Arc<dyn ::std::ops::Fn(FCore::Cache, FCore::Graph, metamodelica::List<metamodelica::Ref<Absyn::Exp>>, metamodelica::List<metamodelica::Ref<Absyn::NamedArg>>, bool, DAE::Prefix, SourceInfo) -> Result<(FCore::Cache, metamodelica::Ref<DAE::Exp>, DAE::Properties)> + 'static>),
        Deref @ "zeros" => (std::sync::Arc::new(move |__a0: FCore::Cache, __a1: FCore::Graph, __a2: metamodelica::List<metamodelica::Ref<Absyn::Exp>>, __a3: metamodelica::List<metamodelica::Ref<Absyn::NamedArg>>, __a4: bool, __a5: DAE::Prefix, __a6: SourceInfo| elabBuiltinZeros(__a0, __a1, __a2, &__a3, __a4, __a5, __a6)) as std::sync::Arc<dyn ::std::ops::Fn(FCore::Cache, FCore::Graph, metamodelica::List<metamodelica::Ref<Absyn::Exp>>, metamodelica::List<metamodelica::Ref<Absyn::NamedArg>>, bool, DAE::Prefix, SourceInfo) -> Result<(FCore::Cache, metamodelica::Ref<DAE::Exp>, DAE::Properties)> + 'static>),
        Deref @ "ones" => (std::sync::Arc::new(move |__a0: FCore::Cache, __a1: FCore::Graph, __a2: metamodelica::List<metamodelica::Ref<Absyn::Exp>>, __a3: metamodelica::List<metamodelica::Ref<Absyn::NamedArg>>, __a4: bool, __a5: DAE::Prefix, __a6: SourceInfo| elabBuiltinOnes(__a0, __a1, __a2, &__a3, __a4, __a5, __a6)) as std::sync::Arc<dyn ::std::ops::Fn(FCore::Cache, FCore::Graph, metamodelica::List<metamodelica::Ref<Absyn::Exp>>, metamodelica::List<metamodelica::Ref<Absyn::NamedArg>>, bool, DAE::Prefix, SourceInfo) -> Result<(FCore::Cache, metamodelica::Ref<DAE::Exp>, DAE::Properties)> + 'static>),
        Deref @ "fill" => (std::sync::Arc::new(move |__a0: FCore::Cache, __a1: FCore::Graph, __a2: metamodelica::List<metamodelica::Ref<Absyn::Exp>>, __a3: metamodelica::List<metamodelica::Ref<Absyn::NamedArg>>, __a4: bool, __a5: DAE::Prefix, __a6: SourceInfo| elabBuiltinFill(__a0, __a1, __a2, &__a3, __a4, __a5, __a6)) as std::sync::Arc<dyn ::std::ops::Fn(FCore::Cache, FCore::Graph, metamodelica::List<metamodelica::Ref<Absyn::Exp>>, metamodelica::List<metamodelica::Ref<Absyn::NamedArg>>, bool, DAE::Prefix, SourceInfo) -> Result<(FCore::Cache, metamodelica::Ref<DAE::Exp>, DAE::Properties)> + 'static>),
        Deref @ "max" => (std::sync::Arc::new(move |__a0: FCore::Cache, __a1: FCore::Graph, __a2: metamodelica::List<metamodelica::Ref<Absyn::Exp>>, __a3: metamodelica::List<metamodelica::Ref<Absyn::NamedArg>>, __a4: bool, __a5: DAE::Prefix, __a6: SourceInfo| elabBuiltinMax(__a0, __a1, &__a2, &__a3, __a4, __a5, __a6)) as std::sync::Arc<dyn ::std::ops::Fn(FCore::Cache, FCore::Graph, metamodelica::List<metamodelica::Ref<Absyn::Exp>>, metamodelica::List<metamodelica::Ref<Absyn::NamedArg>>, bool, DAE::Prefix, SourceInfo) -> Result<(FCore::Cache, metamodelica::Ref<DAE::Exp>, DAE::Properties)> + 'static>),
        Deref @ "min" => (std::sync::Arc::new(move |__a0: FCore::Cache, __a1: FCore::Graph, __a2: metamodelica::List<metamodelica::Ref<Absyn::Exp>>, __a3: metamodelica::List<metamodelica::Ref<Absyn::NamedArg>>, __a4: bool, __a5: DAE::Prefix, __a6: SourceInfo| elabBuiltinMin(__a0, __a1, &__a2, &__a3, __a4, __a5, __a6)) as std::sync::Arc<dyn ::std::ops::Fn(FCore::Cache, FCore::Graph, metamodelica::List<metamodelica::Ref<Absyn::Exp>>, metamodelica::List<metamodelica::Ref<Absyn::NamedArg>>, bool, DAE::Prefix, SourceInfo) -> Result<(FCore::Cache, metamodelica::Ref<DAE::Exp>, DAE::Properties)> + 'static>),
        Deref @ "transpose" => (std::sync::Arc::new(move |__a0: FCore::Cache, __a1: FCore::Graph, __a2: metamodelica::List<metamodelica::Ref<Absyn::Exp>>, __a3: metamodelica::List<metamodelica::Ref<Absyn::NamedArg>>, __a4: bool, __a5: DAE::Prefix, __a6: SourceInfo| elabBuiltinTranspose(__a0, __a1, &__a2, &__a3, __a4, __a5, __a6)) as std::sync::Arc<dyn ::std::ops::Fn(FCore::Cache, FCore::Graph, metamodelica::List<metamodelica::Ref<Absyn::Exp>>, metamodelica::List<metamodelica::Ref<Absyn::NamedArg>>, bool, DAE::Prefix, SourceInfo) -> Result<(FCore::Cache, metamodelica::Ref<DAE::Exp>, DAE::Properties)> + 'static>),
        Deref @ "symmetric" => (std::sync::Arc::new(move |__a0: FCore::Cache, __a1: FCore::Graph, __a2: metamodelica::List<metamodelica::Ref<Absyn::Exp>>, __a3: metamodelica::List<metamodelica::Ref<Absyn::NamedArg>>, __a4: bool, __a5: DAE::Prefix, __a6: SourceInfo| elabBuiltinSymmetric(__a0, __a1, &__a2, &__a3, __a4, __a5, __a6)) as std::sync::Arc<dyn ::std::ops::Fn(FCore::Cache, FCore::Graph, metamodelica::List<metamodelica::Ref<Absyn::Exp>>, metamodelica::List<metamodelica::Ref<Absyn::NamedArg>>, bool, DAE::Prefix, SourceInfo) -> Result<(FCore::Cache, metamodelica::Ref<DAE::Exp>, DAE::Properties)> + 'static>),
        Deref @ "array" => (std::sync::Arc::new(move |__a0: FCore::Cache, __a1: FCore::Graph, __a2: metamodelica::List<metamodelica::Ref<Absyn::Exp>>, __a3: metamodelica::List<metamodelica::Ref<Absyn::NamedArg>>, __a4: bool, __a5: DAE::Prefix, __a6: SourceInfo| elabBuiltinArray(__a0, __a1, &__a2, &__a3, __a4, __a5, __a6)) as std::sync::Arc<dyn ::std::ops::Fn(FCore::Cache, FCore::Graph, metamodelica::List<metamodelica::Ref<Absyn::Exp>>, metamodelica::List<metamodelica::Ref<Absyn::NamedArg>>, bool, DAE::Prefix, SourceInfo) -> Result<(FCore::Cache, metamodelica::Ref<DAE::Exp>, DAE::Properties)> + 'static>),
        Deref @ "sum" => (std::sync::Arc::new(move |__a0: FCore::Cache, __a1: FCore::Graph, __a2: metamodelica::List<metamodelica::Ref<Absyn::Exp>>, __a3: metamodelica::List<metamodelica::Ref<Absyn::NamedArg>>, __a4: bool, __a5: DAE::Prefix, __a6: SourceInfo| elabBuiltinSum(__a0, __a1, &__a2, &__a3, __a4, __a5, __a6)) as std::sync::Arc<dyn ::std::ops::Fn(FCore::Cache, FCore::Graph, metamodelica::List<metamodelica::Ref<Absyn::Exp>>, metamodelica::List<metamodelica::Ref<Absyn::NamedArg>>, bool, DAE::Prefix, SourceInfo) -> Result<(FCore::Cache, metamodelica::Ref<DAE::Exp>, DAE::Properties)> + 'static>),
        Deref @ "product" => (std::sync::Arc::new(move |__a0: FCore::Cache, __a1: FCore::Graph, __a2: metamodelica::List<metamodelica::Ref<Absyn::Exp>>, __a3: metamodelica::List<metamodelica::Ref<Absyn::NamedArg>>, __a4: bool, __a5: DAE::Prefix, __a6: SourceInfo| elabBuiltinProduct(__a0, __a1, &__a2, &__a3, __a4, __a5, __a6)) as std::sync::Arc<dyn ::std::ops::Fn(FCore::Cache, FCore::Graph, metamodelica::List<metamodelica::Ref<Absyn::Exp>>, metamodelica::List<metamodelica::Ref<Absyn::NamedArg>>, bool, DAE::Prefix, SourceInfo) -> Result<(FCore::Cache, metamodelica::Ref<DAE::Exp>, DAE::Properties)> + 'static>),
        Deref @ "pre" => (std::sync::Arc::new(move |__a0: FCore::Cache, __a1: FCore::Graph, __a2: metamodelica::List<metamodelica::Ref<Absyn::Exp>>, __a3: metamodelica::List<metamodelica::Ref<Absyn::NamedArg>>, __a4: bool, __a5: DAE::Prefix, __a6: SourceInfo| elabBuiltinPre(__a0, __a1, &__a2, &__a3, __a4, __a5, __a6)) as std::sync::Arc<dyn ::std::ops::Fn(FCore::Cache, FCore::Graph, metamodelica::List<metamodelica::Ref<Absyn::Exp>>, metamodelica::List<metamodelica::Ref<Absyn::NamedArg>>, bool, DAE::Prefix, SourceInfo) -> Result<(FCore::Cache, metamodelica::Ref<DAE::Exp>, DAE::Properties)> + 'static>),
        Deref @ "firstTick" => (std::sync::Arc::new(move |__a0: FCore::Cache, __a1: FCore::Graph, __a2: metamodelica::List<metamodelica::Ref<Absyn::Exp>>, __a3: metamodelica::List<metamodelica::Ref<Absyn::NamedArg>>, __a4: bool, __a5: DAE::Prefix, __a6: SourceInfo| elabBuiltinFirstTick(__a0, __a1, &__a2, &__a3, __a4, __a5, __a6)) as std::sync::Arc<dyn ::std::ops::Fn(FCore::Cache, FCore::Graph, metamodelica::List<metamodelica::Ref<Absyn::Exp>>, metamodelica::List<metamodelica::Ref<Absyn::NamedArg>>, bool, DAE::Prefix, SourceInfo) -> Result<(FCore::Cache, metamodelica::Ref<DAE::Exp>, DAE::Properties)> + 'static>),
        Deref @ "interval" => (std::sync::Arc::new(move |__a0: FCore::Cache, __a1: FCore::Graph, __a2: metamodelica::List<metamodelica::Ref<Absyn::Exp>>, __a3: metamodelica::List<metamodelica::Ref<Absyn::NamedArg>>, __a4: bool, __a5: DAE::Prefix, __a6: SourceInfo| elabBuiltinInterval(__a0, __a1, &__a2, &__a3, __a4, __a5, __a6)) as std::sync::Arc<dyn ::std::ops::Fn(FCore::Cache, FCore::Graph, metamodelica::List<metamodelica::Ref<Absyn::Exp>>, metamodelica::List<metamodelica::Ref<Absyn::NamedArg>>, bool, DAE::Prefix, SourceInfo) -> Result<(FCore::Cache, metamodelica::Ref<DAE::Exp>, DAE::Properties)> + 'static>),
        Deref @ "boolean" => (std::sync::Arc::new(move |__a0: FCore::Cache, __a1: FCore::Graph, __a2: metamodelica::List<metamodelica::Ref<Absyn::Exp>>, __a3: metamodelica::List<metamodelica::Ref<Absyn::NamedArg>>, __a4: bool, __a5: DAE::Prefix, __a6: SourceInfo| elabBuiltinBoolean(__a0, __a1, &__a2, &__a3, __a4, __a5, __a6)) as std::sync::Arc<dyn ::std::ops::Fn(FCore::Cache, FCore::Graph, metamodelica::List<metamodelica::Ref<Absyn::Exp>>, metamodelica::List<metamodelica::Ref<Absyn::NamedArg>>, bool, DAE::Prefix, SourceInfo) -> Result<(FCore::Cache, metamodelica::Ref<DAE::Exp>, DAE::Properties)> + 'static>),
        Deref @ "noEvent" => (std::sync::Arc::new(move |__a0: FCore::Cache, __a1: FCore::Graph, __a2: metamodelica::List<metamodelica::Ref<Absyn::Exp>>, __a3: metamodelica::List<metamodelica::Ref<Absyn::NamedArg>>, __a4: bool, __a5: DAE::Prefix, __a6: SourceInfo| elabBuiltinNoevent(__a0, __a1, &__a2, &__a3, __a4, __a5, __a6)) as std::sync::Arc<dyn ::std::ops::Fn(FCore::Cache, FCore::Graph, metamodelica::List<metamodelica::Ref<Absyn::Exp>>, metamodelica::List<metamodelica::Ref<Absyn::NamedArg>>, bool, DAE::Prefix, SourceInfo) -> Result<(FCore::Cache, metamodelica::Ref<DAE::Exp>, DAE::Properties)> + 'static>),
        Deref @ "edge" => (std::sync::Arc::new(move |__a0: FCore::Cache, __a1: FCore::Graph, __a2: metamodelica::List<metamodelica::Ref<Absyn::Exp>>, __a3: metamodelica::List<metamodelica::Ref<Absyn::NamedArg>>, __a4: bool, __a5: DAE::Prefix, __a6: SourceInfo| elabBuiltinEdge(__a0, __a1, &__a2, &__a3, __a4, __a5, __a6)) as std::sync::Arc<dyn ::std::ops::Fn(FCore::Cache, FCore::Graph, metamodelica::List<metamodelica::Ref<Absyn::Exp>>, metamodelica::List<metamodelica::Ref<Absyn::NamedArg>>, bool, DAE::Prefix, SourceInfo) -> Result<(FCore::Cache, metamodelica::Ref<DAE::Exp>, DAE::Properties)> + 'static>),
        Deref @ "der" => (std::sync::Arc::new(move |__a0: FCore::Cache, __a1: FCore::Graph, __a2: metamodelica::List<metamodelica::Ref<Absyn::Exp>>, __a3: metamodelica::List<metamodelica::Ref<Absyn::NamedArg>>, __a4: bool, __a5: DAE::Prefix, __a6: SourceInfo| elabBuiltinDer(__a0, __a1, __a2, &__a3, __a4, __a5, __a6)) as std::sync::Arc<dyn ::std::ops::Fn(FCore::Cache, FCore::Graph, metamodelica::List<metamodelica::Ref<Absyn::Exp>>, metamodelica::List<metamodelica::Ref<Absyn::NamedArg>>, bool, DAE::Prefix, SourceInfo) -> Result<(FCore::Cache, metamodelica::Ref<DAE::Exp>, DAE::Properties)> + 'static>),
        Deref @ "change" => (std::sync::Arc::new(move |__a0: FCore::Cache, __a1: FCore::Graph, __a2: metamodelica::List<metamodelica::Ref<Absyn::Exp>>, __a3: metamodelica::List<metamodelica::Ref<Absyn::NamedArg>>, __a4: bool, __a5: DAE::Prefix, __a6: SourceInfo| elabBuiltinChange(__a0, __a1, &__a2, &__a3, __a4, __a5, __a6)) as std::sync::Arc<dyn ::std::ops::Fn(FCore::Cache, FCore::Graph, metamodelica::List<metamodelica::Ref<Absyn::Exp>>, metamodelica::List<metamodelica::Ref<Absyn::NamedArg>>, bool, DAE::Prefix, SourceInfo) -> Result<(FCore::Cache, metamodelica::Ref<DAE::Exp>, DAE::Properties)> + 'static>),
        Deref @ "cat" => (std::sync::Arc::new(move |__a0: FCore::Cache, __a1: FCore::Graph, __a2: metamodelica::List<metamodelica::Ref<Absyn::Exp>>, __a3: metamodelica::List<metamodelica::Ref<Absyn::NamedArg>>, __a4: bool, __a5: DAE::Prefix, __a6: SourceInfo| elabBuiltinCat(__a0, __a1, __a2, &__a3, __a4, __a5, __a6)) as std::sync::Arc<dyn ::std::ops::Fn(FCore::Cache, FCore::Graph, metamodelica::List<metamodelica::Ref<Absyn::Exp>>, metamodelica::List<metamodelica::Ref<Absyn::NamedArg>>, bool, DAE::Prefix, SourceInfo) -> Result<(FCore::Cache, metamodelica::Ref<DAE::Exp>, DAE::Properties)> + 'static>),
        Deref @ "identity" => (std::sync::Arc::new(move |__a0: FCore::Cache, __a1: FCore::Graph, __a2: metamodelica::List<metamodelica::Ref<Absyn::Exp>>, __a3: metamodelica::List<metamodelica::Ref<Absyn::NamedArg>>, __a4: bool, __a5: DAE::Prefix, __a6: SourceInfo| elabBuiltinIdentity(__a0, __a1, &__a2, &__a3, __a4, __a5, __a6)) as std::sync::Arc<dyn ::std::ops::Fn(FCore::Cache, FCore::Graph, metamodelica::List<metamodelica::Ref<Absyn::Exp>>, metamodelica::List<metamodelica::Ref<Absyn::NamedArg>>, bool, DAE::Prefix, SourceInfo) -> Result<(FCore::Cache, metamodelica::Ref<DAE::Exp>, DAE::Properties)> + 'static>),
        Deref @ "vector" => (std::sync::Arc::new(move |__a0: FCore::Cache, __a1: FCore::Graph, __a2: metamodelica::List<metamodelica::Ref<Absyn::Exp>>, __a3: metamodelica::List<metamodelica::Ref<Absyn::NamedArg>>, __a4: bool, __a5: DAE::Prefix, __a6: SourceInfo| elabBuiltinVector(__a0, __a1, &__a2, &__a3, __a4, __a5, __a6)) as std::sync::Arc<dyn ::std::ops::Fn(FCore::Cache, FCore::Graph, metamodelica::List<metamodelica::Ref<Absyn::Exp>>, metamodelica::List<metamodelica::Ref<Absyn::NamedArg>>, bool, DAE::Prefix, SourceInfo) -> Result<(FCore::Cache, metamodelica::Ref<DAE::Exp>, DAE::Properties)> + 'static>),
        Deref @ "matrix" => (std::sync::Arc::new(move |__a0: FCore::Cache, __a1: FCore::Graph, __a2: metamodelica::List<metamodelica::Ref<Absyn::Exp>>, __a3: metamodelica::List<metamodelica::Ref<Absyn::NamedArg>>, __a4: bool, __a5: DAE::Prefix, __a6: SourceInfo| elabBuiltinMatrix(__a0, __a1, &__a2, &__a3, __a4, __a5, __a6)) as std::sync::Arc<dyn ::std::ops::Fn(FCore::Cache, FCore::Graph, metamodelica::List<metamodelica::Ref<Absyn::Exp>>, metamodelica::List<metamodelica::Ref<Absyn::NamedArg>>, bool, DAE::Prefix, SourceInfo) -> Result<(FCore::Cache, metamodelica::Ref<DAE::Exp>, DAE::Properties)> + 'static>),
        Deref @ "scalar" => (std::sync::Arc::new(move |__a0: FCore::Cache, __a1: FCore::Graph, __a2: metamodelica::List<metamodelica::Ref<Absyn::Exp>>, __a3: metamodelica::List<metamodelica::Ref<Absyn::NamedArg>>, __a4: bool, __a5: DAE::Prefix, __a6: SourceInfo| elabBuiltinScalar(__a0, __a1, &__a2, &__a3, __a4, __a5, __a6)) as std::sync::Arc<dyn ::std::ops::Fn(FCore::Cache, FCore::Graph, metamodelica::List<metamodelica::Ref<Absyn::Exp>>, metamodelica::List<metamodelica::Ref<Absyn::NamedArg>>, bool, DAE::Prefix, SourceInfo) -> Result<(FCore::Cache, metamodelica::Ref<DAE::Exp>, DAE::Properties)> + 'static>),
        Deref @ "String" => (std::sync::Arc::new(elabBuiltinString) as std::sync::Arc<dyn ::std::ops::Fn(FCore::Cache, FCore::Graph, metamodelica::List<metamodelica::Ref<Absyn::Exp>>, metamodelica::List<metamodelica::Ref<Absyn::NamedArg>>, bool, DAE::Prefix, SourceInfo) -> Result<(FCore::Cache, metamodelica::Ref<DAE::Exp>, DAE::Properties)> + 'static>),
        Deref @ "rooted" => (std::sync::Arc::new(move |__a0: FCore::Cache, __a1: FCore::Graph, __a2: metamodelica::List<metamodelica::Ref<Absyn::Exp>>, __a3: metamodelica::List<metamodelica::Ref<Absyn::NamedArg>>, __a4: bool, __a5: DAE::Prefix, __a6: SourceInfo| elabBuiltinRooted(__a0, __a1, &__a2, &__a3, __a4, __a5, __a6)) as std::sync::Arc<dyn ::std::ops::Fn(FCore::Cache, FCore::Graph, metamodelica::List<metamodelica::Ref<Absyn::Exp>>, metamodelica::List<metamodelica::Ref<Absyn::NamedArg>>, bool, DAE::Prefix, SourceInfo) -> Result<(FCore::Cache, metamodelica::Ref<DAE::Exp>, DAE::Properties)> + 'static>),
        Deref @ "Integer" => (std::sync::Arc::new(move |__a0: FCore::Cache, __a1: FCore::Graph, __a2: metamodelica::List<metamodelica::Ref<Absyn::Exp>>, __a3: metamodelica::List<metamodelica::Ref<Absyn::NamedArg>>, __a4: bool, __a5: DAE::Prefix, __a6: SourceInfo| elabBuiltinIntegerEnum(__a0, __a1, &__a2, &__a3, __a4, __a5, __a6)) as std::sync::Arc<dyn ::std::ops::Fn(FCore::Cache, FCore::Graph, metamodelica::List<metamodelica::Ref<Absyn::Exp>>, metamodelica::List<metamodelica::Ref<Absyn::NamedArg>>, bool, DAE::Prefix, SourceInfo) -> Result<(FCore::Cache, metamodelica::Ref<DAE::Exp>, DAE::Properties)> + 'static>),
        Deref @ "EnumToInteger" => (std::sync::Arc::new(move |__a0: FCore::Cache, __a1: FCore::Graph, __a2: metamodelica::List<metamodelica::Ref<Absyn::Exp>>, __a3: metamodelica::List<metamodelica::Ref<Absyn::NamedArg>>, __a4: bool, __a5: DAE::Prefix, __a6: SourceInfo| elabBuiltinIntegerEnum(__a0, __a1, &__a2, &__a3, __a4, __a5, __a6)) as std::sync::Arc<dyn ::std::ops::Fn(FCore::Cache, FCore::Graph, metamodelica::List<metamodelica::Ref<Absyn::Exp>>, metamodelica::List<metamodelica::Ref<Absyn::NamedArg>>, bool, DAE::Prefix, SourceInfo) -> Result<(FCore::Cache, metamodelica::Ref<DAE::Exp>, DAE::Properties)> + 'static>),
        Deref @ "inStream" => (std::sync::Arc::new(move |__a0: FCore::Cache, __a1: FCore::Graph, __a2: metamodelica::List<metamodelica::Ref<Absyn::Exp>>, __a3: metamodelica::List<metamodelica::Ref<Absyn::NamedArg>>, __a4: bool, __a5: DAE::Prefix, __a6: SourceInfo| elabBuiltinInStream(__a0, __a1, &__a2, &__a3, __a4, __a5, __a6)) as std::sync::Arc<dyn ::std::ops::Fn(FCore::Cache, FCore::Graph, metamodelica::List<metamodelica::Ref<Absyn::Exp>>, metamodelica::List<metamodelica::Ref<Absyn::NamedArg>>, bool, DAE::Prefix, SourceInfo) -> Result<(FCore::Cache, metamodelica::Ref<DAE::Exp>, DAE::Properties)> + 'static>),
        Deref @ "actualStream" => (std::sync::Arc::new(move |__a0: FCore::Cache, __a1: FCore::Graph, __a2: metamodelica::List<metamodelica::Ref<Absyn::Exp>>, __a3: metamodelica::List<metamodelica::Ref<Absyn::NamedArg>>, __a4: bool, __a5: DAE::Prefix, __a6: SourceInfo| elabBuiltinActualStream(__a0, __a1, &__a2, &__a3, __a4, __a5, __a6)) as std::sync::Arc<dyn ::std::ops::Fn(FCore::Cache, FCore::Graph, metamodelica::List<metamodelica::Ref<Absyn::Exp>>, metamodelica::List<metamodelica::Ref<Absyn::NamedArg>>, bool, DAE::Prefix, SourceInfo) -> Result<(FCore::Cache, metamodelica::Ref<DAE::Exp>, DAE::Properties)> + 'static>),
        Deref @ "getInstanceName" => (std::sync::Arc::new(move |__a0: FCore::Cache, __a1: FCore::Graph, __a2: metamodelica::List<metamodelica::Ref<Absyn::Exp>>, __a3: metamodelica::List<metamodelica::Ref<Absyn::NamedArg>>, __a4: bool, __a5: DAE::Prefix, __a6: SourceInfo| elabBuiltinGetInstanceName(__a0, &__a1, &__a2, &__a3, __a4, &__a5, &__a6)) as std::sync::Arc<dyn ::std::ops::Fn(FCore::Cache, FCore::Graph, metamodelica::List<metamodelica::Ref<Absyn::Exp>>, metamodelica::List<metamodelica::Ref<Absyn::NamedArg>>, bool, DAE::Prefix, SourceInfo) -> Result<(FCore::Cache, metamodelica::Ref<DAE::Exp>, DAE::Properties)> + 'static>),
        Deref @ "classDirectory" => (std::sync::Arc::new(move |__a0: FCore::Cache, __a1: FCore::Graph, __a2: metamodelica::List<metamodelica::Ref<Absyn::Exp>>, __a3: metamodelica::List<metamodelica::Ref<Absyn::NamedArg>>, __a4: bool, __a5: DAE::Prefix, __a6: SourceInfo| elabBuiltinClassDirectory(__a0, &__a1, &__a2, &__a3, __a4, &__a5, &__a6)) as std::sync::Arc<dyn ::std::ops::Fn(FCore::Cache, FCore::Graph, metamodelica::List<metamodelica::Ref<Absyn::Exp>>, metamodelica::List<metamodelica::Ref<Absyn::NamedArg>>, bool, DAE::Prefix, SourceInfo) -> Result<(FCore::Cache, metamodelica::Ref<DAE::Exp>, DAE::Properties)> + 'static>),
        Deref @ "sample" => (std::sync::Arc::new(move |__a0: FCore::Cache, __a1: FCore::Graph, __a2: metamodelica::List<metamodelica::Ref<Absyn::Exp>>, __a3: metamodelica::List<metamodelica::Ref<Absyn::NamedArg>>, __a4: bool, __a5: DAE::Prefix, __a6: SourceInfo| elabBuiltinSample(__a0, __a1, &__a2, &__a3, __a4, __a5, __a6)) as std::sync::Arc<dyn ::std::ops::Fn(FCore::Cache, FCore::Graph, metamodelica::List<metamodelica::Ref<Absyn::Exp>>, metamodelica::List<metamodelica::Ref<Absyn::NamedArg>>, bool, DAE::Prefix, SourceInfo) -> Result<(FCore::Cache, metamodelica::Ref<DAE::Exp>, DAE::Properties)> + 'static>),
        Deref @ "cardinality" => (std::sync::Arc::new(move |__a0: FCore::Cache, __a1: FCore::Graph, __a2: metamodelica::List<metamodelica::Ref<Absyn::Exp>>, __a3: metamodelica::List<metamodelica::Ref<Absyn::NamedArg>>, __a4: bool, __a5: DAE::Prefix, __a6: SourceInfo| elabBuiltinCardinality(__a0, __a1, &__a2, &__a3, __a4, __a5, __a6)) as std::sync::Arc<dyn ::std::ops::Fn(FCore::Cache, FCore::Graph, metamodelica::List<metamodelica::Ref<Absyn::Exp>>, metamodelica::List<metamodelica::Ref<Absyn::NamedArg>>, bool, DAE::Prefix, SourceInfo) -> Result<(FCore::Cache, metamodelica::Ref<DAE::Exp>, DAE::Properties)> + 'static>),
        Deref @ "homotopy" => (std::sync::Arc::new(elabBuiltinHomotopy) as std::sync::Arc<dyn ::std::ops::Fn(FCore::Cache, FCore::Graph, metamodelica::List<metamodelica::Ref<Absyn::Exp>>, metamodelica::List<metamodelica::Ref<Absyn::NamedArg>>, bool, DAE::Prefix, SourceInfo) -> Result<(FCore::Cache, metamodelica::Ref<DAE::Exp>, DAE::Properties)> + 'static>),
        Deref @ "DynamicSelect" => (std::sync::Arc::new(elabBuiltinDynamicSelect) as std::sync::Arc<dyn ::std::ops::Fn(FCore::Cache, FCore::Graph, metamodelica::List<metamodelica::Ref<Absyn::Exp>>, metamodelica::List<metamodelica::Ref<Absyn::NamedArg>>, bool, DAE::Prefix, SourceInfo) -> Result<(FCore::Cache, metamodelica::Ref<DAE::Exp>, DAE::Properties)> + 'static>),
        Deref @ "Clock" => {
            let true = (Config::synchronousFeaturesAllowed()?) else { return Err("pattern mismatch") };
            (std::sync::Arc::new(move |__a0: FCore::Cache, __a1: FCore::Graph, __a2: metamodelica::List<metamodelica::Ref<Absyn::Exp>>, __a3: metamodelica::List<metamodelica::Ref<Absyn::NamedArg>>, __a4: bool, __a5: DAE::Prefix, __a6: SourceInfo| elabBuiltinClock(__a0, __a1, &__a2, &__a3, __a4, __a5, __a6)) as std::sync::Arc<dyn ::std::ops::Fn(FCore::Cache, FCore::Graph, metamodelica::List<metamodelica::Ref<Absyn::Exp>>, metamodelica::List<metamodelica::Ref<Absyn::NamedArg>>, bool, DAE::Prefix, SourceInfo) -> Result<(FCore::Cache, metamodelica::Ref<DAE::Exp>, DAE::Properties)> + 'static>)
        },
        Deref @ "hold" => {
            let true = (Config::synchronousFeaturesAllowed()?) else { return Err("pattern mismatch") };
            (std::sync::Arc::new(move |__a0: FCore::Cache, __a1: FCore::Graph, __a2: metamodelica::List<metamodelica::Ref<Absyn::Exp>>, __a3: metamodelica::List<metamodelica::Ref<Absyn::NamedArg>>, __a4: bool, __a5: DAE::Prefix, __a6: SourceInfo| elabBuiltinHold(__a0, __a1, &__a2, &__a3, __a4, __a5, __a6)) as std::sync::Arc<dyn ::std::ops::Fn(FCore::Cache, FCore::Graph, metamodelica::List<metamodelica::Ref<Absyn::Exp>>, metamodelica::List<metamodelica::Ref<Absyn::NamedArg>>, bool, DAE::Prefix, SourceInfo) -> Result<(FCore::Cache, metamodelica::Ref<DAE::Exp>, DAE::Properties)> + 'static>)
        },
        Deref @ "shiftSample" => {
            let true = (Config::synchronousFeaturesAllowed()?) else { return Err("pattern mismatch") };
            (std::sync::Arc::new(move |__a0: FCore::Cache, __a1: FCore::Graph, __a2: metamodelica::List<metamodelica::Ref<Absyn::Exp>>, __a3: metamodelica::List<metamodelica::Ref<Absyn::NamedArg>>, __a4: bool, __a5: DAE::Prefix, __a6: SourceInfo| elabBuiltinShiftSample(__a0, __a1, &__a2, &__a3, __a4, __a5, __a6)) as std::sync::Arc<dyn ::std::ops::Fn(FCore::Cache, FCore::Graph, metamodelica::List<metamodelica::Ref<Absyn::Exp>>, metamodelica::List<metamodelica::Ref<Absyn::NamedArg>>, bool, DAE::Prefix, SourceInfo) -> Result<(FCore::Cache, metamodelica::Ref<DAE::Exp>, DAE::Properties)> + 'static>)
        },
        Deref @ "backSample" => {
            let true = (Config::synchronousFeaturesAllowed()?) else { return Err("pattern mismatch") };
            (std::sync::Arc::new(move |__a0: FCore::Cache, __a1: FCore::Graph, __a2: metamodelica::List<metamodelica::Ref<Absyn::Exp>>, __a3: metamodelica::List<metamodelica::Ref<Absyn::NamedArg>>, __a4: bool, __a5: DAE::Prefix, __a6: SourceInfo| elabBuiltinBackSample(__a0, __a1, &__a2, &__a3, __a4, __a5, __a6)) as std::sync::Arc<dyn ::std::ops::Fn(FCore::Cache, FCore::Graph, metamodelica::List<metamodelica::Ref<Absyn::Exp>>, metamodelica::List<metamodelica::Ref<Absyn::NamedArg>>, bool, DAE::Prefix, SourceInfo) -> Result<(FCore::Cache, metamodelica::Ref<DAE::Exp>, DAE::Properties)> + 'static>)
        },
        Deref @ "noClock" => {
            let true = (Config::synchronousFeaturesAllowed()?) else { return Err("pattern mismatch") };
            (std::sync::Arc::new(move |__a0: FCore::Cache, __a1: FCore::Graph, __a2: metamodelica::List<metamodelica::Ref<Absyn::Exp>>, __a3: metamodelica::List<metamodelica::Ref<Absyn::NamedArg>>, __a4: bool, __a5: DAE::Prefix, __a6: SourceInfo| elabBuiltinNoClock(__a0, __a1, &__a2, &__a3, __a4, __a5, __a6)) as std::sync::Arc<dyn ::std::ops::Fn(FCore::Cache, FCore::Graph, metamodelica::List<metamodelica::Ref<Absyn::Exp>>, metamodelica::List<metamodelica::Ref<Absyn::NamedArg>>, bool, DAE::Prefix, SourceInfo) -> Result<(FCore::Cache, metamodelica::Ref<DAE::Exp>, DAE::Properties)> + 'static>)
        },
        Deref @ "transition" => {
            let true = (Config::synchronousFeaturesAllowed()?) else { return Err("pattern mismatch") };
            (std::sync::Arc::new(elabBuiltinTransition) as std::sync::Arc<dyn ::std::ops::Fn(FCore::Cache, FCore::Graph, metamodelica::List<metamodelica::Ref<Absyn::Exp>>, metamodelica::List<metamodelica::Ref<Absyn::NamedArg>>, bool, DAE::Prefix, SourceInfo) -> Result<(FCore::Cache, metamodelica::Ref<DAE::Exp>, DAE::Properties)> + 'static>)
        },
        Deref @ "initialState" => {
            let true = (Config::synchronousFeaturesAllowed()?) else { return Err("pattern mismatch") };
            (std::sync::Arc::new(move |__a0: FCore::Cache, __a1: FCore::Graph, __a2: metamodelica::List<metamodelica::Ref<Absyn::Exp>>, __a3: metamodelica::List<metamodelica::Ref<Absyn::NamedArg>>, __a4: bool, __a5: DAE::Prefix, __a6: SourceInfo| elabBuiltinInitialState(__a0, __a1, __a2, &__a3, __a4, __a5, __a6)) as std::sync::Arc<dyn ::std::ops::Fn(FCore::Cache, FCore::Graph, metamodelica::List<metamodelica::Ref<Absyn::Exp>>, metamodelica::List<metamodelica::Ref<Absyn::NamedArg>>, bool, DAE::Prefix, SourceInfo) -> Result<(FCore::Cache, metamodelica::Ref<DAE::Exp>, DAE::Properties)> + 'static>)
        },
        Deref @ "activeState" => {
            let true = (Config::synchronousFeaturesAllowed()?) else { return Err("pattern mismatch") };
            (std::sync::Arc::new(move |__a0: FCore::Cache, __a1: FCore::Graph, __a2: metamodelica::List<metamodelica::Ref<Absyn::Exp>>, __a3: metamodelica::List<metamodelica::Ref<Absyn::NamedArg>>, __a4: bool, __a5: DAE::Prefix, __a6: SourceInfo| elabBuiltinActiveState(__a0, __a1, __a2, &__a3, __a4, __a5, __a6)) as std::sync::Arc<dyn ::std::ops::Fn(FCore::Cache, FCore::Graph, metamodelica::List<metamodelica::Ref<Absyn::Exp>>, metamodelica::List<metamodelica::Ref<Absyn::NamedArg>>, bool, DAE::Prefix, SourceInfo) -> Result<(FCore::Cache, metamodelica::Ref<DAE::Exp>, DAE::Properties)> + 'static>)
        },
        Deref @ "ticksInState" => {
            let true = (Config::synchronousFeaturesAllowed()?) else { return Err("pattern mismatch") };
            (std::sync::Arc::new(move |__a0: FCore::Cache, __a1: FCore::Graph, __a2: metamodelica::List<metamodelica::Ref<Absyn::Exp>>, __a3: metamodelica::List<metamodelica::Ref<Absyn::NamedArg>>, __a4: bool, __a5: DAE::Prefix, __a6: SourceInfo| elabBuiltinTicksInState(__a0, __a1, &__a2, &__a3, __a4, __a5, __a6)) as std::sync::Arc<dyn ::std::ops::Fn(FCore::Cache, FCore::Graph, metamodelica::List<metamodelica::Ref<Absyn::Exp>>, metamodelica::List<metamodelica::Ref<Absyn::NamedArg>>, bool, DAE::Prefix, SourceInfo) -> Result<(FCore::Cache, metamodelica::Ref<DAE::Exp>, DAE::Properties)> + 'static>)
        },
        Deref @ "timeInState" => {
            let true = (Config::synchronousFeaturesAllowed()?) else { return Err("pattern mismatch") };
            (std::sync::Arc::new(move |__a0: FCore::Cache, __a1: FCore::Graph, __a2: metamodelica::List<metamodelica::Ref<Absyn::Exp>>, __a3: metamodelica::List<metamodelica::Ref<Absyn::NamedArg>>, __a4: bool, __a5: DAE::Prefix, __a6: SourceInfo| elabBuiltinTimeInState(__a0, __a1, &__a2, &__a3, __a4, __a5, __a6)) as std::sync::Arc<dyn ::std::ops::Fn(FCore::Cache, FCore::Graph, metamodelica::List<metamodelica::Ref<Absyn::Exp>>, metamodelica::List<metamodelica::Ref<Absyn::NamedArg>>, bool, DAE::Prefix, SourceInfo) -> Result<(FCore::Cache, metamodelica::Ref<DAE::Exp>, DAE::Properties)> + 'static>)
        },
        Deref @ "sourceInfo" => {
            let true = (Config::acceptMetaModelicaGrammar()?) else { return Err("pattern mismatch") };
            (std::sync::Arc::new(move |__a0: FCore::Cache, __a1: FCore::Graph, __a2: metamodelica::List<metamodelica::Ref<Absyn::Exp>>, __a3: metamodelica::List<metamodelica::Ref<Absyn::NamedArg>>, __a4: bool, __a5: DAE::Prefix, __a6: SourceInfo| elabBuiltinSourceInfo(__a0, &__a1, &__a2, &__a3, __a4, &__a5, &__a6)) as std::sync::Arc<dyn ::std::ops::Fn(FCore::Cache, FCore::Graph, metamodelica::List<metamodelica::Ref<Absyn::Exp>>, metamodelica::List<metamodelica::Ref<Absyn::NamedArg>>, bool, DAE::Prefix, SourceInfo) -> Result<(FCore::Cache, metamodelica::Ref<DAE::Exp>, DAE::Properties)> + 'static>)
        },
        Deref @ "SOME" => {
            let true = (Config::acceptMetaModelicaGrammar()?) else { return Err("pattern mismatch") };
            (std::sync::Arc::new(move |__a0: FCore::Cache, __a1: FCore::Graph, __a2: metamodelica::List<metamodelica::Ref<Absyn::Exp>>, __a3: metamodelica::List<metamodelica::Ref<Absyn::NamedArg>>, __a4: bool, __a5: DAE::Prefix, __a6: SourceInfo| elabBuiltinSome(__a0, __a1, &__a2, &__a3, __a4, __a5, __a6)) as std::sync::Arc<dyn ::std::ops::Fn(FCore::Cache, FCore::Graph, metamodelica::List<metamodelica::Ref<Absyn::Exp>>, metamodelica::List<metamodelica::Ref<Absyn::NamedArg>>, bool, DAE::Prefix, SourceInfo) -> Result<(FCore::Cache, metamodelica::Ref<DAE::Exp>, DAE::Properties)> + 'static>)
        },
        Deref @ "NONE" => {
            let true = (Config::acceptMetaModelicaGrammar()?) else { return Err("pattern mismatch") };
            (std::sync::Arc::new(move |__a0: FCore::Cache, __a1: FCore::Graph, __a2: metamodelica::List<metamodelica::Ref<Absyn::Exp>>, __a3: metamodelica::List<metamodelica::Ref<Absyn::NamedArg>>, __a4: bool, __a5: DAE::Prefix, __a6: SourceInfo| elabBuiltinNone(__a0, &__a1, &__a2, &__a3, __a4, &__a5, &__a6)) as std::sync::Arc<dyn ::std::ops::Fn(FCore::Cache, FCore::Graph, metamodelica::List<metamodelica::Ref<Absyn::Exp>>, metamodelica::List<metamodelica::Ref<Absyn::NamedArg>>, bool, DAE::Prefix, SourceInfo) -> Result<(FCore::Cache, metamodelica::Ref<DAE::Exp>, DAE::Properties)> + 'static>)
        },
        Deref @ "isPresent" => {
            let true = (Config::acceptMetaModelicaGrammar()?) else { return Err("pattern mismatch") };
            (std::sync::Arc::new(move |__a0: FCore::Cache, __a1: FCore::Graph, __a2: metamodelica::List<metamodelica::Ref<Absyn::Exp>>, __a3: metamodelica::List<metamodelica::Ref<Absyn::NamedArg>>, __a4: bool, __a5: DAE::Prefix, __a6: SourceInfo| elabBuiltinIsPresent(__a0, &__a1, &__a2, &__a3, __a4, &__a5, &__a6)) as std::sync::Arc<dyn ::std::ops::Fn(FCore::Cache, FCore::Graph, metamodelica::List<metamodelica::Ref<Absyn::Exp>>, metamodelica::List<metamodelica::Ref<Absyn::NamedArg>>, bool, DAE::Prefix, SourceInfo) -> Result<(FCore::Cache, metamodelica::Ref<DAE::Exp>, DAE::Properties)> + 'static>)
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok(outHandler)
}

pub(crate) fn isBuiltinFunc(
    mut inPath: metamodelica::Ref<Absyn::Path>,
    mut ty: &metamodelica::Ref<DAE::Type>,
) -> (DAE::FunctionBuiltin, bool, metamodelica::Ref<Absyn::Path>) {
    let mut isBuiltin: DAE::FunctionBuiltin = DAE::FunctionBuiltin::FUNCTION_BUILTIN_PTR;
    let mut b: bool;
    let mut outPath: metamodelica::Ref<Absyn::Path>;
    (isBuiltin, b, outPath) = 'mc: {
        let __mc_input = (inPath.clone(), &**ty);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (path, Deref @ DAE::Type::T_FUNCTION { functionAttributes: DAE::FunctionAttributes { isBuiltin: isBuiltin @ DAE::FunctionBuiltin::FUNCTION_BUILTIN { name: _, .. }, .. }, .. }) => {
                    let mut path = (*path).clone();
                    path = AbsynUtil::makeNotFullyQualified(path.clone());
                    Ok((isBuiltin.clone(), true, path.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (path, Deref @ DAE::Type::T_FUNCTION { functionAttributes: DAE::FunctionAttributes { isBuiltin: isBuiltin @ DAE::FunctionBuiltin::FUNCTION_BUILTIN_PTR { .. }, .. }, .. }) => {
                    let mut path = (*path).clone();
                    path = AbsynUtil::makeNotFullyQualified(path.clone());
                    Ok((isBuiltin.clone(), false, path.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ Absyn::Path::IDENT { name: id }, _) => {
                    elabBuiltinHandler(metamodelica::AsArg::as_arg(&id))?;
                    Ok((DAE::FunctionBuiltin::FUNCTION_BUILTIN { name: Some(id.clone()), unboxArgs: false }, true, inPath.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ Absyn::Path::FULLYQUALIFIED { path }, _) => {
                    let mut path = (*path).clone();
                    let mut isBuiltin: DAE::FunctionBuiltin = isBuiltin.clone();
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(isBuiltinFunc(path.clone(), ty)) {
                        (__pa0 @ DAE::FunctionBuiltin::FUNCTION_BUILTIN { .. }, _, __pa1) => (__pa0.clone(), __pa1.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    isBuiltin = metamodelica::Own::own(__pa0);
                    path = metamodelica::Own::own(__pa1);
                    Ok(((isBuiltin.clone(), true, path.clone()), isBuiltin.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            isBuiltin = __wb0;
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ Absyn::Path::QUALIFIED { name: Deref @ "Connections", path: Deref @ Absyn::Path::IDENT { name: Deref @ "isRoot" } }, _) => {
                    Ok((DAE::FunctionBuiltin::FUNCTION_BUILTIN { name: None, unboxArgs: false }, true, inPath.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((openmodelica_frontend_types::DAE::FunctionBuiltin::FUNCTION_NOT_BUILTIN, false, inPath.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    (isBuiltin, b, outPath)
}

fn elabCallBuiltin<'__b>(
    mut inCache: &'__b FCore::Cache,
    mut inEnv: &'__b FCore::Graph,
    mut inFnName: &'__b metamodelica::Ref<Absyn::ComponentRef>,
    mut inPosArgs: &'__b metamodelica::List<metamodelica::Ref<Absyn::Exp>>,
    mut inNamedArgs: &'__b metamodelica::List<metamodelica::Ref<Absyn::NamedArg>>,
    mut inImplicit: bool,
    mut inPrefix: &'__b DAE::Prefix,
    mut inInfo: &'__b SourceInfo,
) -> Result<(FCore::Cache, metamodelica::Ref<DAE::Exp>, DAE::Properties)> {
    pub type HandlerFunc = std::sync::Arc<
        dyn ::std::ops::Fn(
                FCore::Cache,
                FCore::Graph,
                metamodelica::List<metamodelica::Ref<Absyn::Exp>>,
                metamodelica::List<metamodelica::Ref<Absyn::NamedArg>>,
                bool,
                DAE::Prefix,
                SourceInfo,
            ) -> Result<(FCore::Cache, metamodelica::Ref<DAE::Exp>, DAE::Properties)>
            + 'static,
    >;

    '__tco: loop {
        ::match_deref::match_deref! { match inFnName {
            Deref @ Absyn::ComponentRef::CREF_IDENT { subscripts: Deref @ metamodelica::ListNode::Nil, .. } => {
                let mut handler: Arc<dyn ::std::ops::Fn(FCore::Cache, FCore::Graph, metamodelica::List<metamodelica::Ref<Absyn::Exp>>, metamodelica::List<metamodelica::Ref<Absyn::NamedArg>>, bool, DAE::Prefix, SourceInfo) -> Result<(FCore::Cache, metamodelica::Ref<DAE::Exp>, DAE::Properties)> + 'static>;
                handler = elabBuiltinHandler(var_field!((**inFnName).name, Absyn::ComponentRef::CREF_IDENT))?;
                return Ok(handler(inCache.clone(), inEnv.clone(), inPosArgs.clone(), inNamedArgs.clone(), inImplicit, inPrefix.clone(), inInfo.clone())?)
            },
            Deref @ Absyn::ComponentRef::CREF_QUAL { name: Deref @ "Connections", componentRef: Deref @ Absyn::ComponentRef::CREF_IDENT { name: Deref @ "isRoot", .. }, .. } => {
                return Ok(elabBuiltinIsRoot(inCache.clone(), inEnv.clone(), inPosArgs, inNamedArgs, inImplicit, inPrefix.clone(), inInfo.clone())?)
            },
            Deref @ Absyn::ComponentRef::CREF_QUAL { name: Deref @ "Connections", componentRef: Deref @ Absyn::ComponentRef::CREF_IDENT { name: Deref @ "uniqueRootIndices", .. }, .. } => {
                Error::addSourceMessage(&(Error::NON_STANDARD_OPERATOR.clone()), list![literal!("Connections.uniqueRootIndices")], inInfo)?;
                return Ok(elabBuiltinUniqueRootIndices(inCache.clone(), inEnv.clone(), inPosArgs, inNamedArgs, inImplicit, inPrefix.clone(), inInfo.clone())?)
            },
            Deref @ Absyn::ComponentRef::CREF_QUAL { name: Deref @ "Connections", componentRef: Deref @ Absyn::ComponentRef::CREF_IDENT { name: Deref @ "rooted", .. }, .. } => {
                return Ok(elabBuiltinRooted(inCache.clone(), inEnv.clone(), inPosArgs, inNamedArgs, inImplicit, inPrefix.clone(), inInfo.clone())?)
            },
            Deref @ Absyn::ComponentRef::CREF_FULLYQUALIFIED { componentRef: cr } => {
                { (inCache, inEnv, inFnName, inPosArgs, inNamedArgs, inImplicit, inPrefix, inInfo) = (inCache, inEnv, cr, inPosArgs, inNamedArgs, inImplicit, inPrefix, inInfo); continue '__tco; }
            },
            _ => return Err("match: no arm matched"),
        } }
    }
}

fn elabCall(
    mut cache: FCore::Cache,
    mut env: FCore::Graph,
    mut r#fn: metamodelica::Ref<Absyn::ComponentRef>,
    mut args: metamodelica::List<metamodelica::Ref<Absyn::Exp>>,
    mut nargs: metamodelica::List<metamodelica::Ref<Absyn::NamedArg>>,
    mut typeVars: &metamodelica::List<metamodelica::Ref<Absyn::Path>>,
    mut r#impl: bool,
    mut pre: DAE::Prefix,
    mut info: SourceInfo,
) -> Result<(FCore::Cache, metamodelica::Ref<DAE::Exp>, DAE::Properties)> {
    let mut cache: FCore::Cache = cache;
    let mut e: metamodelica::Ref<DAE::Exp>;
    let mut prop: DAE::Properties;
    let mut numErrorMessages: i32 = Error::getNumErrorMessages();
    let mut handles: metamodelica::List<i32>;
    let mut name: ArcStr;
    let mut s: ArcStr;
    let mut s1: ArcStr;
    let mut s2: ArcStr;
    let mut fn_1: metamodelica::Ref<Absyn::Path>;
    let mut fnstr: ArcStr;
    let mut argstr: ArcStr;
    let mut prestr: ArcStr;
    let mut argstrs: metamodelica::List<ArcStr>;
    if hasBuiltInHandler(&r#fn) {
        match '__try0: {
            (cache, e, prop) =
                unwrap_break_err!(elabCallBuiltin(&cache, &env, &r#fn, &args, &nargs, r#impl, &pre, &info), '__try0);
            return Ok((cache, e, prop));
            Ok::<(), &'static str>(())
        } {
            Ok(()) => {}
            Err(__try0_err) => {
                let true = (numErrorMessages == Error::getNumErrorMessages()) else {
                    return Err("pattern mismatch");
                };
                name = Dump::printComponentRefStr(&r#fn)?;
                s1 = stringDelimitList(List::map(args.clone(), &Dump::printExpStr)?, literal!(", "));
                s2 = stringDelimitList(
                    List::map(nargs.clone(), &move |__a0: metamodelica::Ref<Absyn::NamedArg>| {
                        Dump::printNamedArgStr(&__a0)
                    })?,
                    literal!(", "),
                );
                s = if (metamodelica::stringEq(&s2, &(literal!("")))) {
                    s1.clone()
                } else {
                    {
                        let mut __mm_s = String::new();
                        __mm_s.push_str(&*s1);
                        __mm_s.push_str(&*literal!(", "));
                        __mm_s.push_str(&*s2);
                        ArcStr::from(__mm_s)
                    }
                };
                s = stringAppendList(list![name.clone(), literal!("("), s.clone(), literal!(").\n")]);
                Error::addSourceMessage(
                    &(Error::WRONG_TYPE_OR_NO_OF_ARGS.clone()),
                    list![s.clone(), PrefixUtil::printPrefixStr3(pre.clone())?],
                    &info,
                )?;
                return Err(__try0_err);
            }
        }
    }
    handles = metamodelica::nil();
    match '__try1: {
        ErrorExt::setCheckpoint(literal!("elabCall_InteractiveFunction"));
        fn_1 = unwrap_break_err!(AbsynUtil::crefToPath(&r#fn), '__try1);
        (cache, e, prop) = unwrap_break_err!(elabCallArgs(cache.clone(), env.clone(), fn_1.clone(), args.clone(), nargs.clone(), typeVars, r#impl, pre.clone(), info.clone()), '__try1);
        ErrorExt::delCheckpoint(literal!("elabCall_InteractiveFunction"));
        Ok::<_, &'static str>((cache.clone(), e.clone(), prop.clone()))
    } {
        Ok((__try1_o0, __try1_o1, __try1_o2)) => {
            cache = __try1_o0;
            e = __try1_o1;
            prop = __try1_o2;
        }
        Err(_) => {
            if Flags::isSet(Flags::FAILTRACE.clone())? {
                Debug::traceln(literal!("- Static.elabCall failed\n"))?;
                Debug::trace(literal!(" function: "))?;
                fnstr = Dump::printComponentRefStr(&r#fn)?;
                Debug::trace(fnstr.clone())?;
                Debug::trace(literal!("   posargs: "))?;
                argstrs = List::map(args.clone(), &Dump::printExpStr)?;
                argstr = stringDelimitList(argstrs.clone(), literal!(", "));
                Debug::traceln(argstr.clone())?;
                Debug::trace(literal!(" prefix: "))?;
                prestr = PrefixUtil::printPrefixStr(&pre)?;
                Debug::traceln(prestr.clone())?;
            }
            (cache, e, prop) = BackendCevalInterface::elabCallInteractive(
                cache.clone(),
                env.clone(),
                r#fn.clone(),
                args.clone(),
                nargs.clone(),
                r#impl,
                pre.clone(),
                info.clone(),
            )?;
        }
    }
    Ok((cache, e, prop))
}

fn hasBuiltInHandler(mut r#fn: &metamodelica::Ref<Absyn::ComponentRef>) -> bool {
    let mut b: bool;
    b = 'mc: {
        let __mc_input = &**r#fn;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Absyn::ComponentRef::CREF_IDENT { name, subscripts: Deref @ metamodelica::ListNode::Nil } => {
                    elabBuiltinHandler(metamodelica::AsArg::as_arg(&name))?;
                    Ok(true)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Absyn::ComponentRef::CREF_QUAL { name: Deref @ "Connections", componentRef: Deref @ Absyn::ComponentRef::CREF_IDENT { name: Deref @ "isRoot", .. }, .. } => {
                    Ok(true)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Absyn::ComponentRef::CREF_QUAL { name: Deref @ "Connections", componentRef: Deref @ Absyn::ComponentRef::CREF_IDENT { name: Deref @ "uniqueRootIndices", .. }, .. } => {
                    Ok(true)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Absyn::ComponentRef::CREF_QUAL { name: Deref @ "Connections", componentRef: Deref @ Absyn::ComponentRef::CREF_IDENT { name: Deref @ "rooted", .. }, .. } => {
                    Ok(true)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Absyn::ComponentRef::CREF_FULLYQUALIFIED { componentRef: cr } => {
                    Ok(hasBuiltInHandler(metamodelica::AsArg::as_arg(&cr)))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(false)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    b
}

fn isValidDerVariableName(mut exp: metamodelica::Ref<Absyn::Exp>, mut nested: bool) -> bool {
    '__tco: loop {
        let mut arg: metamodelica::Ref<Absyn::Exp>;
        ::match_deref::match_deref! { match &(exp) {
            Deref @ Absyn::Exp::CREF { .. } => return nested,
            Deref @ Absyn::Exp::CALL { function_: Deref @ Absyn::ComponentRef::CREF_IDENT { name: Deref @ "der", .. }, functionArgs: Deref @ Absyn::FunctionArgs::FUNCTIONARGS { args: Deref @ metamodelica::ListNode::Cons { head: __esc_arg, tail: Deref @ metamodelica::ListNode::Nil }, argNames: Deref @ metamodelica::ListNode::Nil }, .. } => {
                arg = (*__esc_arg).clone();
                { (exp, nested) = (arg.clone(), true); continue '__tco; }
            },
            _ => return false,
            _ => unreachable!("tail-call lowered match: no arm matched"),
        } }
    }
}

pub(crate) fn elabVariablenames(
    mut inExpl: metamodelica::List<metamodelica::Ref<Absyn::Exp>>,
) -> Result<metamodelica::List<metamodelica::Ref<DAE::Exp>>> {
    let mut outExpl: metamodelica::List<metamodelica::Ref<DAE::Exp>> = metamodelica::nil();
    outExpl = ({
        let mut __acc: metamodelica::List<metamodelica::Ref<DAE::Exp>> = metamodelica::nil();
        for mut e in (inExpl).into_iter().cloned() {
            let __x = (match &*e.clone() {
                Absyn::Exp::CREF {
                    componentRef: __e_componentRef,
                } => metamodelica::Ref::new(DAE::Exp::CODE {
                    code: metamodelica::Ref::new(Absyn::CodeNode::C_VARIABLENAME {
                        componentRef: __e_componentRef.clone(),
                    }),
                    ty: DAE::T_UNKNOWN_DEFAULT().clone(),
                }),
                Absyn::Exp::CALL { .. } if (isValidDerVariableName(e.clone(), false)) => {
                    metamodelica::Ref::new(DAE::Exp::CODE {
                        code: metamodelica::Ref::new(Absyn::CodeNode::C_EXPRESSION { exp: e.clone() }),
                        ty: DAE::T_UNKNOWN_DEFAULT().clone(),
                    })
                }
                _ => return Err("match: no arm matched"),
            });
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    Ok(outExpl)
}

pub fn getOptionalNamedArgExpList(
    mut name: &ArcStr,
    mut nargs: &metamodelica::List<metamodelica::Ref<Absyn::NamedArg>>,
) -> metamodelica::List<metamodelica::Ref<DAE::Exp>> {
    let mut out: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
    out = 'mc: {
        let __mc_input = &**nargs;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Nil => {
                    Ok(metamodelica::nil())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: Deref @ Absyn::NamedArg { argName, argValue: Deref @ Absyn::Exp::ARRAY { arrayExp: absynExpList } }, tail: _ } => {
                    let true = (stringEq(&name, &argName)) else { return Err("pattern mismatch") };
                    Ok(absynExpListToDaeExpList(metamodelica::AsArg::as_arg(&absynExpList))?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: _, tail: rest } => {
                    Ok(getOptionalNamedArgExpList(name, metamodelica::AsArg::as_arg(&rest)))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    out
}

fn absynExpListToDaeExpList<'__b>(
    mut absynExpList: &'__b metamodelica::List<metamodelica::Ref<Absyn::Exp>>,
) -> Result<metamodelica::List<metamodelica::Ref<DAE::Exp>>> {
    '__tco: loop {
        ::match_deref::match_deref! { match absynExpList {
            Deref @ metamodelica::ListNode::Nil => {
                return Ok(metamodelica::nil())
            },
            Deref @ metamodelica::ListNode::Cons { head: Deref @ Absyn::Exp::CREF { componentRef: absynCr }, tail: absynRest } => {
                let mut daeExpList: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                let mut absynPath: metamodelica::Ref<Absyn::Path>;
                let mut daeCr: metamodelica::Ref<DAE::ComponentRef>;
                let mut crefExp: metamodelica::Ref<DAE::Exp>;
                absynPath = AbsynUtil::crefToPath(metamodelica::AsArg::as_arg(&absynCr))?;
                daeCr = ComponentReference::pathToCref(&absynPath);
                crefExp = Expression::crefExp(daeCr)?;
                daeExpList = absynExpListToDaeExpList(absynRest)?;
                return Ok(metamodelica::cons(crefExp, daeExpList))
            },
            Deref @ metamodelica::ListNode::Cons { head: _, tail: absynRest } => {
                { absynExpList = absynRest; continue '__tco; }
            },
            _ => return Err("match: no arm matched"),
        } }
    }
}

pub fn getOptionalNamedArg(
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
    mut inImplicit: bool,
    mut inArgName: &ArcStr,
    mut inType: metamodelica::Ref<DAE::Type>,
    mut inArgs: &metamodelica::List<metamodelica::Ref<Absyn::NamedArg>>,
    mut inDefaultExp: metamodelica::Ref<DAE::Exp>,
    mut inPrefix: DAE::Prefix,
    mut inInfo: SourceInfo,
) -> (FCore::Cache, metamodelica::Ref<DAE::Exp>) {
    let mut outCache: FCore::Cache = inCache.clone();
    let mut outExp: metamodelica::Ref<DAE::Exp> = inDefaultExp;
    let mut name: ArcStr;
    let mut ty: metamodelica::Ref<DAE::Type>;
    let mut e: metamodelica::Ref<Absyn::Exp>;
    for mut arg in &**inArgs {
        let __arc1 = arg.clone();
        let Absyn::NAMEDARG { argName: __pa0, .. } = &*__arc1;
        name = metamodelica::Own::own(__pa0);
        if metamodelica::stringEq(&name, &inArgName) {
            if '__try2: {
                let __arc4 = arg.clone();
                let Absyn::NAMEDARG { argValue: __pa3, .. } = &*__arc4;
                e = metamodelica::Own::own(__pa3);
                let (__pa5, __pa6, __pa7) = ::match_deref::match_deref! { match &(unwrap_break_err!(elabExpInExpression(inCache.clone(), inEnv.clone(), e.clone(), inImplicit, true, inPrefix.clone(), inInfo.clone()), '__try2)) {
                    (__pa5, __pa6, DAE::Properties::PROP { type_: __pa7, .. }) => (__pa5.clone(), __pa6.clone(), __pa7.clone()),
                    _ => break '__try2 Err::<_, _>("pattern mismatch"),
                } };
                outCache = metamodelica::Own::own(__pa5);
                outExp = metamodelica::Own::own(__pa6);
                ty = metamodelica::Own::own(__pa7);
                (outExp, _) = unwrap_break_err!(Types::matchType(outExp.clone(), ty.clone(), inType.clone(), true), '__try2);
                Ok::<(), &'static str>(())
            }.is_err() {
            }
            break;
        }
    }
    (outCache, outExp)
}

pub fn elabUntypedCref(
    mut inCache: &FCore::Cache,
    mut inEnv: &FCore::Graph,
    mut inCref: &metamodelica::Ref<Absyn::ComponentRef>,
    mut inImplicit: bool,
    mut inPrefix: &DAE::Prefix,
    mut inInfo: &SourceInfo,
) -> Result<(FCore::Cache, metamodelica::Ref<DAE::ComponentRef>)> {
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut outCref: metamodelica::Ref<DAE::ComponentRef>;
    outCref = (match &**inCref {
        Absyn::ComponentRef::CREF_IDENT {
            name: __inCref_name,
            subscripts: __inCref_subscripts,
        } => {
            let mut subs: metamodelica::List<metamodelica::Ref<DAE::Subscript>>;
            (outCache, subs, _) = elabSubscripts(
                inCache.clone(),
                inEnv.clone(),
                metamodelica::AsArg::as_arg(&__inCref_subscripts),
                inImplicit,
                inPrefix.clone(),
                inInfo,
            )?;
            ComponentReferenceBasics::makeCrefIdent(__inCref_name.clone(), DAE::T_UNKNOWN_DEFAULT().clone(), subs)
        }
        Absyn::ComponentRef::CREF_QUAL {
            componentRef: __inCref_componentRef,
            name: __inCref_name,
            subscripts: __inCref_subscripts,
        } => {
            let mut subs: metamodelica::List<metamodelica::Ref<DAE::Subscript>>;
            let mut cr: metamodelica::Ref<DAE::ComponentRef>;
            (outCache, subs, _) = elabSubscripts(
                inCache.clone(),
                inEnv.clone(),
                metamodelica::AsArg::as_arg(&__inCref_subscripts),
                inImplicit,
                inPrefix.clone(),
                inInfo,
            )?;
            (outCache, cr) = elabUntypedCref(
                &outCache,
                inEnv,
                metamodelica::AsArg::as_arg(&__inCref_componentRef),
                inImplicit,
                inPrefix,
                inInfo,
            )?;
            ComponentReferenceBasics::makeCrefQual(__inCref_name.clone(), DAE::T_UNKNOWN_DEFAULT().clone(), subs, cr)
        }
        _ => return Err("match: no arm matched"),
    });
    Ok((outCache, outCref))
}

pub(crate) fn needToRebuild(mut newFile: ArcStr, mut oldFile: ArcStr, mut buildTime: metamodelica::Real) -> bool {
    let mut buildNeeded: bool;
    buildNeeded = 'mc: {
        let __mc_input = (newFile, oldFile);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ "", Deref @ "") => {
                    Ok(true)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (newf, oldf) => {
                    let mut nfmt: metamodelica::Real;
                    let true = (stringEq(&newf, &oldf)) else { return Err("pattern mismatch") };
                    let __pa0 = ::match_deref::match_deref! { match &(System::getFileModificationTime(newf.clone())) {
                        Some(__pa0) => __pa0.clone(),
                        _ => return Err("pattern mismatch"),
                    } };
                    nfmt = metamodelica::Own::own(__pa0);
                    let true = (realGt(buildTime, nfmt)) else { return Err("pattern mismatch") };
                    Ok(false)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(true)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    buildNeeded
}

fn createDummyFarg(mut name: ArcStr) -> metamodelica::Ref<DAE::FuncArg> {
    let mut farg: metamodelica::Ref<DAE::FuncArg>;
    farg = metamodelica::Ref::new(DAE::FuncArg {
        name: name,
        ty: DAE::T_UNKNOWN_DEFAULT().clone(),
        r#const: openmodelica_frontend_types::DAE::Const::C_VAR,
        par: openmodelica_frontend_types::DAE::VarParallelism::NON_PARALLEL,
        defaultBinding: None,
    });
    farg
}

pub(crate) fn elabCallArgs(
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
    mut inPath: metamodelica::Ref<Absyn::Path>,
    mut inAbsynExpLst: metamodelica::List<metamodelica::Ref<Absyn::Exp>>,
    mut inAbsynNamedArgLst: metamodelica::List<metamodelica::Ref<Absyn::NamedArg>>,
    mut typeVars: &metamodelica::List<metamodelica::Ref<Absyn::Path>>,
    mut inBoolean: bool,
    mut inPrefix: DAE::Prefix,
    mut info: SourceInfo,
) -> Result<(FCore::Cache, metamodelica::Ref<DAE::Exp>, DAE::Properties)> {
    let mut outCache: FCore::Cache;
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    let mut outProperties: DAE::Properties;
    let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(elabCallArgs2(inCache, inEnv.clone(), inPath, inAbsynExpLst, inAbsynNamedArgLst, typeVars, inBoolean, Mutable::create(false), inPrefix.clone(), info.clone(), Error::getNumErrorMessages())?) {
        (__pa0, Some((__pa1, __pa2))) => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
        _ => return Err("pattern mismatch"),
    } };
    outCache = metamodelica::Own::own(__pa0);
    outExp = metamodelica::Own::own(__pa1);
    outProperties = metamodelica::Own::own(__pa2);
    (outCache, outProperties) = elabCallArgsEvaluateArrayLength(outCache, inEnv, outProperties, &inPrefix, &info);
    Ok((outCache, outExp, outProperties))
}

fn elabCallArgsEvaluateArrayLength(
    mut inCache: FCore::Cache,
    mut env: FCore::Graph,
    mut inProperties: DAE::Properties,
    mut inPrefix: &DAE::Prefix,
    mut info: &SourceInfo,
) -> (FCore::Cache, DAE::Properties) {
    let mut outCache: FCore::Cache;
    let mut outProperties: DAE::Properties;
    let mut ty: metamodelica::Ref<DAE::Type>;
    match '__try0: {
        let true = (FGraph::checkScopeType(
            &(list![unwrap_break_err!(FGraph::lastScopeRef(&env), '__try0)]),
            Some(openmodelica_frontend_dump::FCore::ScopeType::CLASS_SCOPE),
        )) else {
            break '__try0 Err::<_, _>("pattern mismatch");
        };
        ty = Types::getPropType(&inProperties);
        let (__pa1, (__pa2, _)) = unwrap_break_err!(Types::traverseType(ty.clone(), (inCache.clone(), env.clone()), &fnptr!(elabCallArgsEvaluateArrayLength2, metamodelica::Ref<DAE::Type>, (FCore::Cache, FCore::Graph))), '__try0);
        ty = metamodelica::Own::own(__pa1);
        outCache = metamodelica::Own::own(__pa2);
        outProperties = Types::setPropType(&inProperties, ty.clone());
        Ok::<_, &'static str>((outCache.clone(), outProperties.clone()))
    } {
        Ok((__try0_o0, __try0_o1)) => {
            outCache = __try0_o0;
            outProperties = __try0_o1;
        }
        Err(_) => {
            outCache = inCache.clone();
            outProperties = inProperties.clone();
        }
    }
    (outCache, outProperties)
}

fn elabCallArgsEvaluateArrayLength2(
    mut ty: metamodelica::Ref<DAE::Type>,
    mut inTpl: (FCore::Cache, FCore::Graph),
) -> (metamodelica::Ref<DAE::Type>, (FCore::Cache, FCore::Graph)) {
    let mut oty: metamodelica::Ref<DAE::Type> = ty;
    let mut outTpl: (FCore::Cache, FCore::Graph);
    (oty, outTpl) = 'mc: {
        let __mc_input = (oty.clone(), inTpl.clone());
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Type::T_ARRAY { .. }, tpl) => {
                    let mut dims: metamodelica::List<metamodelica::Ref<DAE::Dimension>>;
                    let mut tpl = (*tpl).clone();
                    let mut oty: metamodelica::Ref<DAE::Type> = oty.clone();
                    (dims, tpl) = List::mapFold(var_field!((*oty).dims, DAE::Type::T_ARRAY), &fnptr!(elabCallArgsEvaluateArrayLength3, metamodelica::Ref<DAE::Dimension>, (FCore::Cache, FCore::Graph)), tpl.clone())?;
                    assign_variant_field!(oty => DAE::Type::T_ARRAY; dims = dims.clone());
                    Ok(((oty.clone(), tpl.clone()), oty.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            oty = __wb0;
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((oty.clone(), inTpl.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    (oty, outTpl)
}

fn elabCallArgsEvaluateArrayLength3(
    mut inDim: metamodelica::Ref<DAE::Dimension>,
    mut inTpl: (FCore::Cache, FCore::Graph),
) -> (metamodelica::Ref<DAE::Dimension>, (FCore::Cache, FCore::Graph)) {
    let mut outDim: metamodelica::Ref<DAE::Dimension>;
    let mut outTpl: (FCore::Cache, FCore::Graph);
    (outDim, outTpl) = 'mc: {
        let __mc_input = (&*inDim, &inTpl);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Dimension::DIM_EXP { exp }, (cache, env)) => {
                    let mut i: i32;
                    let mut cache = (*cache).clone();
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(Ceval::ceval(cache.clone(), env.clone(), exp.clone(), false, openmodelica_ast::Absyn::Msg::NO_MSG, 0)?) {
                        (__pa0, Deref @ Values::Value::INTEGER { integer: __pa1 }) => (__pa0.clone(), __pa1.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    cache = metamodelica::Own::own(__pa0);
                    i = metamodelica::Own::own(__pa1);
                    Ok((metamodelica::Ref::new(DAE::Dimension::DIM_INTEGER { integer: i }), (cache.clone(), env.clone())))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((inDim.clone(), inTpl.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    (outDim, outTpl)
}

fn createInputVariableReplacements(
    mut inSlotLst: &metamodelica::List<Slot>,
    mut inVarsRepl: &VarTransform::VariableReplacements,
) -> Result<VarTransform::VariableReplacements> {
    let mut outVarsRepl: VarTransform::VariableReplacements;
    outVarsRepl = 'mc: {
        let __mc_input = &**inSlotLst;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Nil => {
                    Ok(inVarsRepl.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: Slot { defaultArg: Deref @ DAE::FuncArg { name: id, .. }, slotFilled: true, arg: Some(e), .. }, tail: rest } => {
                    let mut o: VarTransform::VariableReplacements;
                    o = VarTransform::addReplacement(inVarsRepl.clone(), ComponentReferenceBasics::makeCrefIdent(id.clone(), DAE::T_UNKNOWN_DEFAULT().clone(), metamodelica::nil()), e.clone())?;
                    Ok(createInputVariableReplacements(metamodelica::AsArg::as_arg(&rest), &o)?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(createInputVariableReplacements(&((inSlotLst).rest()?), inVarsRepl)?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(outVarsRepl)
}

fn elabCallArgs2(
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
    mut inPath: metamodelica::Ref<Absyn::Path>,
    mut inAbsynExpLst: metamodelica::List<metamodelica::Ref<Absyn::Exp>>,
    mut inAbsynNamedArgLst: metamodelica::List<metamodelica::Ref<Absyn::NamedArg>>,
    mut typeVars: &metamodelica::List<metamodelica::Ref<Absyn::Path>>,
    mut inBoolean: bool,
    mut stopElab: Mutable::Mutable<bool>,
    mut inPrefix: DAE::Prefix,
    mut info: SourceInfo,
    mut numErrors: i32,
) -> Result<(FCore::Cache, Option<(metamodelica::Ref<DAE::Exp>, DAE::Properties)>)> {
    let mut outCache: FCore::Cache;
    let mut expProps: Option<(metamodelica::Ref<DAE::Exp>, DAE::Properties)> = None;
    (outCache, expProps) = 'mc: {
        let __mc_input = (
            inCache,
            inEnv,
            inPath,
            inAbsynExpLst,
            inAbsynNamedArgLst,
            inBoolean,
            inPrefix,
        );
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, r#fn, args, nargs, r#impl, pre) => {
                    let mut fargs: metamodelica::List<metamodelica::Ref<DAE::FuncArg>>;
                    let mut env_1: FCore::Graph;
                    let mut env_2: FCore::Graph;
                    let mut slots: metamodelica::List<Slot>;
                    let mut newslots: metamodelica::List<Slot>;
                    let mut newslots2: metamodelica::List<Slot>;
                    let mut args_2: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                    let mut cl: metamodelica::Ref<SCode::Element>;
                    let mut names: metamodelica::List<ArcStr>;
                    let mut tp: metamodelica::Ref<DAE::Type>;
                    let mut cache = (*cache).clone();
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(Lookup::lookupClassIdent(cache.clone(), env.clone(), &(literal!("GraphicalAnnotationsProgram____")), None)?) {
                        (__pa0, __pa1 @ Deref @ SCode::Element::CLASS { restriction: SCode::Restriction::R_PACKAGE { .. }, .. }, _) => (__pa0.clone(), __pa1.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    cache = metamodelica::Own::own(__pa0);
                    cl = metamodelica::Own::own(__pa1);
                    let (__pa3, __pa4, __pa5) = ::match_deref::match_deref! { match &(Lookup::lookupClass(metamodelica::AsArg::as_arg(&cache), metamodelica::AsArg::as_arg(&env), metamodelica::AsArg::as_arg(&r#fn), None)?) {
                        (__pa3, __pa4 @ Deref @ SCode::Element::CLASS { restriction: SCode::Restriction::R_RECORD { isOperator: _ }, .. }, __pa5) => (__pa3.clone(), __pa4.clone(), __pa5.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    cache = metamodelica::Own::own(__pa3);
                    cl = metamodelica::Own::own(__pa4);
                    env_1 = metamodelica::Own::own(__pa5);
                    (cache, cl, env_2) = Lookup::lookupRecordConstructorClass(cache.clone(), env_1.clone(), r#fn.clone())?;
                    let __pa7 = ::match_deref::match_deref! { match &(SCodeUtil::getClassComponents(&cl)?) {
                        (_, Deref @ metamodelica::ListNode::Cons { head: _, tail: __pa7 }) => __pa7.clone(),
                        _ => return Err("pattern mismatch"),
                    } };
                    names = metamodelica::Own::own(__pa7);
                    fargs = List::map(names.clone(), &fnptr!(createDummyFarg, ArcStr))?;
                    slots = makeEmptySlots(&fargs)?;
                    (cache, _, newslots, _, _) = elabInputArgs(cache.clone(), env.clone(), metamodelica::AsArg::as_arg(&args), metamodelica::AsArg::as_arg(&nargs), slots.clone(), true, false, r#impl.clone(), pre.clone(), info.clone(), &(DAE::T_UNKNOWN_DEFAULT().clone()), r#fn.clone(), true)?;
                    (cache, newslots2, _, _) = fillGraphicsDefaultSlots(cache.clone(), &newslots, &cl, env_2.clone(), r#impl.clone(), pre.clone(), info.clone());
                    args_2 = slotListArgs(&newslots2);
                    tp = complexTypeFromSlots(&newslots2, ClassInf::State::UNKNOWN { path: metamodelica::Ref::new(Absyn::Path::IDENT { name: literal!("") }) })?;
                    Ok((cache.clone(), Some((metamodelica::Ref::new(DAE::Exp::CALL { path: r#fn.clone(), expLst: args_2.clone(), attr: metamodelica::Ref::new(DAE::CallAttributes { ty: tp.clone(), tuple_: false, builtin: false, isImpure: false, isFunctionPointerCall: false, inlineType: openmodelica_frontend_types::DAE::InlineType::NO_INLINE, tailCall: openmodelica_frontend_types::DAE::TailCall::NO_TAIL, noReturn: DAE::NoReturn::RETURNS.clone() }) }), DAE::Properties::PROP { type_: DAE::T_UNKNOWN_DEFAULT().clone(), constFlag: openmodelica_frontend_types::DAE::Const::C_CONST }))))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, r#fn, args, nargs, r#impl, pre) => {
                    let mut outtype: metamodelica::Ref<DAE::Type>;
                    let mut tp1: metamodelica::Ref<DAE::Type>;
                    let mut fargs: metamodelica::List<metamodelica::Ref<DAE::FuncArg>>;
                    let mut slots: metamodelica::List<Slot>;
                    let mut newslots: metamodelica::List<Slot>;
                    let mut newslots2: metamodelica::List<Slot>;
                    let mut args_2: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                    let mut constlist: metamodelica::List<DAE::Const>;
                    let mut constInputArgs: metamodelica::List<DAE::Const>;
                    let mut r#const: DAE::Const;
                    let mut tyconst: metamodelica::Ref<DAE::TupleConst>;
                    let mut prop: DAE::Properties;
                    let mut prop_1: DAE::Properties;
                    let mut path: metamodelica::Ref<Absyn::Path>;
                    let mut vect_dims: metamodelica::List<metamodelica::Ref<DAE::Dimension>>;
                    let mut call_exp: metamodelica::Ref<DAE::Exp>;
                    let mut callExp: metamodelica::Ref<DAE::Exp>;
                    let mut func: DAE::Function;
                    let mut cache = (*cache).clone();
                    let mut expProps: Option<(metamodelica::Ref<DAE::Exp>, DAE::Properties)> = expProps.clone();
                    ErrorExt::setCheckpoint(literal!("RecordConstructor"));
                    (cache, func) = InstFunction::getRecordConstructorFunction(cache.clone(), metamodelica::AsArg::as_arg(&env), r#fn.clone())?;
                    let DAE::RECORD_CONSTRUCTOR { path: __pa0, type_: __pa1, source: _ } = (func.clone()) else { return Err("pattern mismatch") };
                    path = metamodelica::Own::own(__pa0);
                    tp1 = metamodelica::Own::own(__pa1);
                    let (__pa2, __pa3, __pa4) = ::match_deref::match_deref! { match &(tp1.clone()) {
                        Deref @ DAE::Type::T_FUNCTION { funcArg: __pa2, funcResultType: __pa3, functionAttributes: _, path: __pa4 } => (__pa2.clone(), __pa3.clone(), __pa4.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    fargs = metamodelica::Own::own(__pa2);
                    outtype = metamodelica::Own::own(__pa3);
                    path = metamodelica::Own::own(__pa4);
                    slots = makeEmptySlots(&fargs)?;
                    (cache, _, newslots, constInputArgs, _) = elabInputArgs(cache.clone(), env.clone(), metamodelica::AsArg::as_arg(&args), metamodelica::AsArg::as_arg(&nargs), slots.clone(), true, true, r#impl.clone(), pre.clone(), info.clone(), &tp1, path.clone(), false)?;
                    (args_2, newslots2) = addDefaultArgs(newslots.clone(), info.clone())?;
                    vect_dims = slotsVectorizable(&newslots2, &info)?;
                    constlist = constInputArgs.clone();
                    r#const = List::fold(&constlist, &fnptr!(Types::constAnd, DAE::Const, DAE::Const), openmodelica_frontend_types::DAE::Const::C_CONST)?;
                    tyconst = elabConsts(outtype.clone(), r#const)?;
                    prop = getProperties(outtype.clone(), tyconst.clone())?;
                    callExp = metamodelica::Ref::new(DAE::Exp::CALL { path: path.clone(), expLst: args_2.clone(), attr: metamodelica::Ref::new(DAE::CallAttributes { ty: outtype.clone(), tuple_: false, builtin: false, isImpure: false, isFunctionPointerCall: false, inlineType: openmodelica_frontend_types::DAE::InlineType::NO_INLINE, tailCall: openmodelica_frontend_types::DAE::TailCall::NO_TAIL, noReturn: DAE::NoReturn::RETURNS.clone() }) });
                    (call_exp, prop_1) = vectorizeCall(callExp.clone(), &vect_dims, &newslots2, prop.clone(), &info)?;
                    expProps = Some((call_exp.clone(), prop_1.clone()));
                    Mutable::update(stopElab.clone(), true);
                    ErrorExt::rollBack(literal!("RecordConstructor"));
                    Ok(((cache.clone(), expProps.clone()), expProps.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            expProps = __wb0;
            break 'mc __v;
        }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, r#fn, args, nargs, r#impl, pre) => {
                    let mut recordEnv: FCore::Graph;
                    let mut recordCl: metamodelica::Ref<SCode::Element>;
                    let mut fn_1: metamodelica::Ref<Absyn::Path>;
                    let mut typelist: metamodelica::List<metamodelica::Ref<DAE::Type>>;
                    let mut operNames: metamodelica::List<metamodelica::Ref<Absyn::Path>>;
                    let mut cache = (*cache).clone();
                    let mut expProps: Option<(metamodelica::Ref<DAE::Exp>, DAE::Properties)> = expProps.clone();
                    let false = (Mutable::access(stopElab.clone())) else { return Err("pattern mismatch") };
                    (cache, recordCl, recordEnv) = Lookup::lookupClass(metamodelica::AsArg::as_arg(&cache), metamodelica::AsArg::as_arg(&env), metamodelica::AsArg::as_arg(&r#fn), None)?;
                    let true = (SCodeUtil::isOperatorRecord(&recordCl)) else { return Err("pattern mismatch") };
                    fn_1 = AbsynUtil::joinPaths(r#fn.clone(), metamodelica::Ref::new(Absyn::Path::IDENT { name: literal!("'constructor'") }))?;
                    (cache, recordCl, recordEnv) = Lookup::lookupClass(metamodelica::AsArg::as_arg(&cache), &recordEnv, &fn_1, None)?;
                    let true = (SCodeUtil::isOperator(&recordCl)) else { return Err("pattern mismatch") };
                    operNames = AbsynToSCode::getListofQualOperatorFuncsfromOperator(&recordCl)?;
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(Lookup::lookupFunctionsListInEnv(cache.clone(), recordEnv.clone(), &operNames, &info, metamodelica::nil())?) {
                        (__pa0, __pa1 @ Deref @ metamodelica::ListNode::Cons { head: _, tail: _ }) => (__pa0.clone(), __pa1.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    cache = metamodelica::Own::own(__pa0);
                    typelist = metamodelica::Own::own(__pa1);
                    Mutable::update(stopElab.clone(), true);
                    (cache, expProps) = elabCallArgs3(cache.clone(), env.clone(), typelist.clone(), fn_1.clone(), metamodelica::AsArg::as_arg(&args), metamodelica::AsArg::as_arg(&nargs), typeVars, r#impl.clone(), pre.clone(), info.clone())?;
                    ErrorExt::rollBack(literal!("RecordConstructor"));
                    Ok(((cache.clone(), expProps.clone()), expProps.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            expProps = __wb0;
            break 'mc __v;
        }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, r#fn, args, nargs, r#impl, pre) => {
                    let mut t: metamodelica::Ref<DAE::Type>;
                    let mut cache = (*cache).clone();
                    let mut expProps: Option<(metamodelica::Ref<DAE::Exp>, DAE::Properties)> = expProps.clone();
                    ErrorExt::delCheckpoint(literal!("RecordConstructor"));
                    let true = (Config::acceptMetaModelicaGrammar()?) else { return Err("pattern mismatch") };
                    let false = (Mutable::access(stopElab.clone())) else { return Err("pattern mismatch") };
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(Lookup::lookupType(cache.clone(), env.clone(), r#fn.clone(), None)?) {
                        (__pa0, __pa1 @ Deref @ DAE::Type::T_METARECORD { .. }, _) => (__pa0.clone(), __pa1.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    cache = metamodelica::Own::own(__pa0);
                    t = metamodelica::Own::own(__pa1);
                    Mutable::update(stopElab.clone(), true);
                    (cache, expProps) = elabCallArgsMetarecord(cache.clone(), env.clone(), t.clone(), args.clone(), metamodelica::AsArg::as_arg(&nargs), r#impl.clone(), stopElab.clone(), pre.clone(), info.clone())?;
                    Ok(((cache.clone(), expProps.clone()), expProps.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            expProps = __wb0;
            break 'mc __v;
        }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, r#fn, args, nargs, r#impl, pre) => {
                    let mut typelist: metamodelica::List<metamodelica::Ref<DAE::Type>>;
                    let mut cache = (*cache).clone();
                    let mut expProps: Option<(metamodelica::Ref<DAE::Exp>, DAE::Properties)> = expProps.clone();
                    ErrorExt::setCheckpoint(literal!("elabCallArgs2FunctionLookup"));
                    let false = (Mutable::access(stopElab.clone())) else { return Err("pattern mismatch") };
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(Lookup::lookupFunctionsInEnv(cache.clone(), env.clone(), r#fn.clone(), info.clone())) {
                        (__pa0, __pa1 @ Deref @ metamodelica::ListNode::Cons { head: _, tail: _ }) => (__pa0.clone(), __pa1.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    cache = metamodelica::Own::own(__pa0);
                    typelist = metamodelica::Own::own(__pa1);
                    Mutable::update(stopElab.clone(), true);
                    (cache, expProps) = elabCallArgs3(cache.clone(), env.clone(), typelist.clone(), r#fn.clone(), metamodelica::AsArg::as_arg(&args), metamodelica::AsArg::as_arg(&nargs), typeVars, r#impl.clone(), pre.clone(), info.clone())?;
                    ErrorExt::delCheckpoint(literal!("elabCallArgs2FunctionLookup"));
                    Ok(((cache.clone(), expProps.clone()), expProps.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            expProps = __wb0;
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, r#fn, args, nargs, r#impl, pre) => {
                    let mut functype: metamodelica::Ref<DAE::Type>;
                    let mut tp1: metamodelica::Ref<DAE::Type>;
                    let mut args_1: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                    let mut typelist: metamodelica::List<metamodelica::Ref<DAE::Type>>;
                    let mut fn_str: ArcStr;
                    let mut types_str: ArcStr;
                    let mut pre_str: ArcStr;
                    let mut argStr: ArcStr;
                    let mut cache = (*cache).clone();
                    let (__pa0, __pa2, __pa1) = ::match_deref::match_deref! { match &(Lookup::lookupFunctionsInEnv(cache.clone(), env.clone(), r#fn.clone(), info.clone())) {
                        (__pa0, __pa2 @ Deref @ metamodelica::ListNode::Cons { head: __pa1, tail: Deref @ metamodelica::ListNode::Nil }) => (__pa0.clone(), __pa2.clone(), __pa1.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    cache = metamodelica::Own::own(__pa0);
                    tp1 = metamodelica::Own::own(__pa1);
                    typelist = metamodelica::Own::own(__pa2);
                    (cache, args_1, _, _, functype, _, _) = elabTypes(cache.clone(), env.clone(), metamodelica::AsArg::as_arg(&args), metamodelica::AsArg::as_arg(&nargs), &(metamodelica::nil()), typelist.clone(), true, false, r#impl.clone(), pre.clone(), info.clone())?;
                    argStr = ExpressionDump::printExpListStr(args_1.clone())?;
                    pre_str = PrefixUtil::printPrefixStr3(pre.clone())?;
                    fn_str = { let mut __mm_s = String::new(); __mm_s.push_str(&*AbsynUtil::pathString(r#fn.clone(), literal!("."), true, false)?); __mm_s.push_str(&*literal!("(")); __mm_s.push_str(&*argStr); __mm_s.push_str(&*literal!(")\nof type\n  ")); __mm_s.push_str(&*TypesDump::unparseType(functype.clone())?); ArcStr::from(__mm_s) };
                    types_str = { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\n  ")); __mm_s.push_str(&*TypesDump::unparseType(tp1.clone())?); ArcStr::from(__mm_s) };
                    Error::assertionOrAddSourceMessage(Error::getNumErrorMessages() != numErrors, &(Error::NO_MATCHING_FUNCTION_FOUND.clone()), list![fn_str.clone(), pre_str.clone(), types_str.clone()], &info)?;
                    ErrorExt::delCheckpoint(literal!("elabCallArgs2FunctionLookup"));
                    Ok((cache.clone(), None))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, r#fn, _, _, _, _) => {
                    let mut fn_str: ArcStr;
                    let mut s: ArcStr;
                    let mut re: SCode::Restriction;
                    let mut cache = (*cache).clone();
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(Lookup::lookupClass(metamodelica::AsArg::as_arg(&cache), metamodelica::AsArg::as_arg(&env), metamodelica::AsArg::as_arg(&r#fn), None)?) {
                        (__pa0, Deref @ SCode::Element::CLASS { restriction: __pa1, .. }, _) => (__pa0.clone(), __pa1.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    cache = metamodelica::Own::own(__pa0);
                    re = metamodelica::Own::own(__pa1);
                    let false = (SCodeUtil::isFunctionRestriction(&re)) else { return Err("pattern mismatch") };
                    fn_str = AbsynUtil::pathString(r#fn.clone(), literal!("."), true, false)?;
                    s = SCodeDump::restrString(&re)?;
                    Error::addSourceMessage(&(Error::LOOKUP_FUNCTION_GOT_CLASS.clone()), list![fn_str.clone(), s.clone()], &info)?;
                    ErrorExt::delCheckpoint(literal!("elabCallArgs2FunctionLookup"));
                    Ok((cache.clone(), None))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, r#fn, _, _, _, pre) => {
                    let mut typelist: metamodelica::List<metamodelica::Ref<DAE::Type>>;
                    let mut t_lst: metamodelica::List<ArcStr>;
                    let mut fn_str: ArcStr;
                    let mut types_str: ArcStr;
                    let mut pre_str: ArcStr;
                    let mut cache = (*cache).clone();
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(Lookup::lookupFunctionsInEnv(cache.clone(), env.clone(), r#fn.clone(), info.clone())) {
                        (__pa0, __pa1 @ Deref @ metamodelica::ListNode::Cons { head: _, tail: Deref @ metamodelica::ListNode::Cons { head: _, tail: _ } }) => (__pa0.clone(), __pa1.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    cache = metamodelica::Own::own(__pa0);
                    typelist = metamodelica::Own::own(__pa1);
                    t_lst = List::map(typelist.clone(), &TypesDump::unparseType)?;
                    fn_str = AbsynUtil::pathString(r#fn.clone(), literal!("."), true, false)?;
                    pre_str = PrefixUtil::printPrefixStr3(pre.clone())?;
                    types_str = stringDelimitList(t_lst.clone(), literal!("\n -"));
                    Error::addSourceMessage(&(Error::NO_MATCHING_FUNCTION_FOUND.clone()), list![fn_str.clone(), pre_str.clone(), types_str.clone()], &info)?;
                    ErrorExt::delCheckpoint(literal!("elabCallArgs2FunctionLookup"));
                    Ok((cache.clone(), None))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, r#fn, Deref @ metamodelica::ListNode::Cons { head: Deref @ Absyn::Exp::CREF { componentRef: Deref @ Absyn::ComponentRef::CREF_IDENT { name, subscripts: _ } }, tail: Deref @ metamodelica::ListNode::Nil }, _, r#impl, pre) => {
                    if !((Config::acceptOptimicaGrammar()?)) { return Err("guard") }
                    let mut prop: DAE::Properties;
                    let mut daeexp: metamodelica::Ref<DAE::Exp>;
                    let mut tp: metamodelica::Ref<DAE::Type>;
                    let mut cref: metamodelica::Ref<Absyn::ComponentRef>;
                    let mut daecref: metamodelica::Ref<DAE::ComponentRef>;
                    let mut cache = (*cache).clone();
                    let mut expProps: Option<(metamodelica::Ref<DAE::Exp>, DAE::Properties)> = expProps.clone();
                    cref = AbsynUtil::pathToCref(metamodelica::AsArg::as_arg(&r#fn));
                    let (__pa0, __pa3, __pa1, __pa2, __pa4) = ::match_deref::match_deref! { match &(elabCref(cache.clone(), env.clone(), cref.clone(), r#impl.clone(), true, pre.clone(), info.clone())?) {
                        (__pa0, Some((__pa3 @ Deref @ DAE::Exp::CREF { componentRef: __pa1, ty: __pa2 }, __pa4, _))) => (__pa0.clone(), __pa3.clone(), __pa1.clone(), __pa2.clone(), __pa4.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    cache = metamodelica::Own::own(__pa0);
                    daecref = metamodelica::Own::own(__pa1);
                    tp = metamodelica::Own::own(__pa2);
                    daeexp = metamodelica::Own::own(__pa3);
                    prop = metamodelica::Own::own(__pa4);
                    ErrorExt::rollBack(literal!("elabCallArgs2FunctionLookup"));
                    daeexp = metamodelica::Ref::new(DAE::Exp::CREF { componentRef: metamodelica::Ref::new(DAE::ComponentRef::OPTIMICA_ATTR_INST_CREF { componentRef: daecref.clone(), instant: name.clone() }), ty: tp.clone() });
                    expProps = Some((daeexp.clone(), prop.clone()));
                    Ok(((cache.clone(), expProps.clone()), expProps.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            expProps = __wb0;
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, r#fn, _, _, _, _) => {
                    let mut fn_str: ArcStr;
                    let mut scope: ArcStr;
                    if '__try0: {
                        unwrap_break_err!(Lookup::lookupType(cache.clone(), env.clone(), r#fn.clone(), None), '__try0);
                        Ok::<(), &'static str>(())
                    }.is_ok() { return Err("failure(): body succeeded") }
                    scope = { let mut __mm_s = String::new(); __mm_s.push_str(&*FGraph::printGraphPathStr(metamodelica::AsArg::as_arg(&env))); __mm_s.push_str(&*literal!(" (looking for a function or record)")); ArcStr::from(__mm_s) };
                    fn_str = AbsynUtil::pathString(r#fn.clone(), literal!("."), true, false)?;
                    Error::addSourceMessage(&(Error::LOOKUP_ERROR.clone()), list![fn_str.clone(), scope.clone()], &info)?;
                    ErrorExt::delCheckpoint(literal!("elabCallArgs2FunctionLookup"));
                    Ok((cache.clone(), None))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, r#fn, _, _, _, pre) => {
                    let mut fn_str: ArcStr;
                    let mut pre_str: ArcStr;
                    let mut cache = (*cache).clone();
                    let __pa0 = ::match_deref::match_deref! { match &(Lookup::lookupFunctionsInEnv(cache.clone(), env.clone(), r#fn.clone(), info.clone())) {
                        (__pa0, Deref @ metamodelica::ListNode::Nil) => __pa0.clone(),
                        _ => return Err("pattern mismatch"),
                    } };
                    cache = metamodelica::Own::own(__pa0);
                    fn_str = AbsynUtil::pathString(r#fn.clone(), literal!("."), true, false)?;
                    pre_str = PrefixUtil::printPrefixStr3(pre.clone())?;
                    fn_str = { let mut __mm_s = String::new(); __mm_s.push_str(&*fn_str); __mm_s.push_str(&*literal!(" in component ")); __mm_s.push_str(&*pre_str); ArcStr::from(__mm_s) };
                    Error::addSourceMessage(&(Error::NO_MATCHING_FUNCTION_FOUND_NO_CANDIDATE.clone()), list![fn_str.clone()], &info)?;
                    ErrorExt::delCheckpoint(literal!("elabCallArgs2FunctionLookup"));
                    Ok((cache.clone(), None))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, env, r#fn, _, _, _, _) => {
                    ErrorExt::delCheckpoint(literal!("elabCallArgs2FunctionLookup"));
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { return Err("pattern mismatch") };
                    Debug::traceln({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("- Static.elabCallArgs failed on: ")); __mm_s.push_str(&*AbsynUtil::pathString(r#fn.clone(), literal!("."), true, false)?); __mm_s.push_str(&*literal!(" in env: ")); __mm_s.push_str(&*FGraph::printGraphPathStr(metamodelica::AsArg::as_arg(&env))); ArcStr::from(__mm_s) })?;
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((outCache, expProps))
}

pub(crate) fn elabCallArgs3(
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
    mut typelist: metamodelica::List<metamodelica::Ref<DAE::Type>>,
    mut r#fn: metamodelica::Ref<Absyn::Path>,
    mut args: &metamodelica::List<metamodelica::Ref<Absyn::Exp>>,
    mut nargs: &metamodelica::List<metamodelica::Ref<Absyn::NamedArg>>,
    mut typeVars: &metamodelica::List<metamodelica::Ref<Absyn::Path>>,
    mut r#impl: bool,
    mut pre: DAE::Prefix,
    mut info: SourceInfo,
) -> Result<(FCore::Cache, Option<(metamodelica::Ref<DAE::Exp>, DAE::Properties)>)> {
    let mut outCache: FCore::Cache;
    let mut expProps: Option<(metamodelica::Ref<DAE::Exp>, DAE::Properties)>;
    let mut callExp: metamodelica::Ref<DAE::Exp>;
    let mut call_exp: metamodelica::Ref<DAE::Exp>;
    let mut args_1: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
    let mut args_2: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
    let mut constlist: metamodelica::List<DAE::Const>;
    let mut r#const: DAE::Const;
    let mut restype: metamodelica::Ref<DAE::Type>;
    let mut functype: metamodelica::Ref<DAE::Type>;
    let mut isBuiltin: DAE::FunctionBuiltin;
    let mut funcParal: DAE::FunctionParallelism;
    let mut tuple_: bool;
    let mut builtin: bool;
    let mut isImpure: bool;
    let mut noReturn: DAE::NoReturn;
    let mut inlineType: DAE::InlineType;
    let mut fn_1: metamodelica::Ref<Absyn::Path>;
    let mut prop: DAE::Properties;
    let mut prop_1: DAE::Properties;
    let mut tp: metamodelica::Ref<DAE::Type>;
    let mut tyconst: metamodelica::Ref<DAE::TupleConst>;
    let mut vect_dims: metamodelica::List<metamodelica::Ref<DAE::Dimension>>;
    let mut slots: metamodelica::List<Slot>;
    let mut slots2: metamodelica::List<Slot>;
    let mut functionTree: metamodelica::Ref<AvlTreePathFunction::Tree>;
    let mut status: Util::Status;
    let mut cache: FCore::Cache;
    let mut didInline: bool;
    let mut onlyOneFunction: bool;
    let mut isFunctionPointer: bool;
    let mut purity: DAE::Purity;
    onlyOneFunction = ((typelist).len() as i32) == 1;
    let (__pa0, __pa1, __pa2, __pa3, __pa9, __pa4, __pa5, __pa6, __pa7, __pa8, __pa10, __pa11) = ::match_deref::match_deref! { match &(elabTypes(inCache, inEnv.clone(), args, nargs, typeVars, typelist, onlyOneFunction, true, r#impl, pre, info.clone())?) {
        (__pa0, __pa1, __pa2, __pa3, __pa9 @ Deref @ DAE::Type::T_FUNCTION { functionAttributes: DAE::FunctionAttributes { purity: __pa4, inline: __pa5, isFunctionPointer: __pa6, functionParallelism: __pa7, noReturn: __pa8, .. }, .. }, __pa10, __pa11) => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone(), __pa9.clone(), __pa4.clone(), __pa5.clone(), __pa6.clone(), __pa7.clone(), __pa8.clone(), __pa10.clone(), __pa11.clone()),
        _ => return Err("pattern mismatch"),
    } };
    cache = metamodelica::Own::own(__pa0);
    args_1 = metamodelica::Own::own(__pa1);
    constlist = metamodelica::Own::own(__pa2);
    restype = metamodelica::Own::own(__pa3);
    purity = metamodelica::Own::own(__pa4);
    inlineType = metamodelica::Own::own(__pa5);
    isFunctionPointer = metamodelica::Own::own(__pa6);
    funcParal = metamodelica::Own::own(__pa7);
    noReturn = metamodelica::Own::own(__pa8);
    functype = metamodelica::Own::own(__pa9);
    vect_dims = metamodelica::Own::own(__pa10);
    slots = metamodelica::Own::own(__pa11);
    isImpure = purity == DAE::Purity::IMPURE.clone();
    (fn_1, functype) = deoverloadFuncname(r#fn.clone(), functype, &inEnv);
    tuple_ = Types::isTuple(&restype);
    (isBuiltin, builtin, fn_1) = isBuiltinFunc(fn_1, &functype);
    inlineType = inlineBuiltin(&isBuiltin, inlineType);
    let true = (isValidWRTParallelScope(&r#fn, builtin, funcParal, &inEnv, &info)) else {
        return Err("pattern mismatch");
    };
    r#const = List::fold(
        &constlist,
        &fnptr!(Types::constAnd, DAE::Const, DAE::Const),
        openmodelica_frontend_types::DAE::Const::C_CONST,
    )?;
    r#const = if (Flags::isSet(Flags::RML.clone())? && !(builtin) || purity == DAE::Purity::OM_IMPURE.clone()) {
        openmodelica_frontend_types::DAE::Const::C_VAR
    } else {
        r#const
    };
    (cache, r#const) = determineConstSpecialFunc(cache, &inEnv, r#const, &fn_1);
    tyconst = elabConsts(restype.clone(), r#const)?;
    prop = getProperties(restype.clone(), tyconst)?;
    tp = Types::simplifyType(restype.clone())?;
    (args_2, slots2) = addDefaultArgs(slots, info.clone())?;
    let true = (List::fold(&slots2, &fnptr!(slotAnd, Slot, bool), true)?) else {
        return Err("pattern mismatch");
    };
    callExp = metamodelica::Ref::new(DAE::Exp::CALL {
        path: fn_1.clone(),
        expLst: args_2,
        attr: metamodelica::Ref::new(DAE::CallAttributes {
            ty: tp,
            tuple_: tuple_,
            builtin: builtin,
            isImpure: isImpure || purity == DAE::Purity::OM_IMPURE.clone(),
            isFunctionPointerCall: isFunctionPointer,
            inlineType: inlineType,
            tailCall: openmodelica_frontend_types::DAE::TailCall::NO_TAIL,
            noReturn: noReturn,
        }),
    });
    (call_exp, prop_1) = vectorizeCall(callExp, &vect_dims, &slots2, prop, &info)?;
    (cache, status) = instantiateDaeFunction(
        cache.clone(),
        inEnv.clone(),
        if (Lookup::isFunctionCallViaComponent(cache, inEnv.clone(), &r#fn)) {
            r#fn
        } else {
            fn_1
        },
        builtin,
        None,
        true,
    );
    cache = instantiateImplicitRecordConstructors(&cache, &inEnv, &args_1);
    functionTree = FCore::getFunctionTree(&cache);
    (call_exp, _, didInline, _) = Inline::inlineExp(
        call_exp,
        (
            Some(functionTree),
            list![
                openmodelica_frontend_types::DAE::InlineType::BUILTIN_EARLY_INLINE,
                openmodelica_frontend_types::DAE::InlineType::EARLY_INLINE
            ],
        ),
        DAE::emptyElementSource().clone(),
    );
    (call_exp, _) = ExpressionSimplify::condsimplify(didInline, call_exp)?;
    didInline = didInline && !(Config::acceptMetaModelicaGrammar()?);
    prop_1 = if (didInline) {
        Types::setPropType(&prop_1, restype)
    } else {
        prop_1
    };
    if !(isImpure) {
        (cache, call_exp, prop_1) = Ceval::cevalIfConstant(cache, inEnv, call_exp, prop_1, r#impl, info)?;
    }
    expProps = if (Util::isSuccess(status)) {
        Some((call_exp, prop_1))
    } else {
        None
    };
    outCache = cache;
    Ok((outCache, expProps))
}

pub(crate) fn inlineBuiltin(mut isBuiltin: &DAE::FunctionBuiltin, mut inlineType: DAE::InlineType) -> DAE::InlineType {
    let mut outInlineType: DAE::InlineType;
    outInlineType = (match isBuiltin.clone() {
        DAE::FunctionBuiltin::FUNCTION_BUILTIN_PTR { .. } => {
            openmodelica_frontend_types::DAE::InlineType::BUILTIN_EARLY_INLINE
        }
        _ => inlineType,
    });
    outInlineType
}

fn isValidWRTParallelScope(
    mut inFn: &metamodelica::Ref<Absyn::Path>,
    mut isBuiltin: bool,
    mut inFuncParallelism: DAE::FunctionParallelism,
    mut inEnv: &FCore::Graph,
    mut inInfo: &SourceInfo,
) -> bool {
    let mut isValid: bool;
    isValid = isValidWRTParallelScope_dispatch(
        inFn,
        isBuiltin,
        inFuncParallelism,
        &(FGraph::currentScope(inEnv)),
        inInfo,
    );
    isValid
}

fn isValidWRTParallelScope_dispatch(
    mut inFn: &metamodelica::Ref<Absyn::Path>,
    mut isBuiltin: bool,
    mut inFuncParallelism: DAE::FunctionParallelism,
    mut inScope: &metamodelica::List<Mutable::Mutable<metamodelica::Ref<FCore::Node>>>,
    mut inInfo: &SourceInfo,
) -> bool {
    let mut isValid: bool;
    isValid = 'mc: {
        let __mc_input = (isBuiltin, inFuncParallelism, &**inScope);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (true, DAE::FunctionParallelism::FP_NON_PARALLEL { .. }, _) => {
                    Ok(true)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, Deref @ metamodelica::ListNode::Cons { head: r#ref, tail: restScope }) => {
                    let mut scopeName: ArcStr;
                    let false = (FNode::isRefTop(r#ref.clone())) else { return Err("pattern mismatch") };
                    scopeName = FNode::refName(r#ref.clone());
                    let true = (listMember(scopeName.clone(), FCore::implicitScopeNames.clone())) else { return Err("pattern mismatch") };
                    let false = (stringEq(&scopeName, &arcstr::literal!(FCore::parForScopeName))) else { return Err("pattern mismatch") };
                    Ok(isValidWRTParallelScope_dispatch(inFn, isBuiltin, inFuncParallelism, metamodelica::AsArg::as_arg(&restScope), inInfo))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, DAE::FunctionParallelism::FP_NON_PARALLEL { .. }, Deref @ metamodelica::ListNode::Cons { head: r#ref, tail: _ }) => {
                    let true = (FGraph::checkScopeType(&(list![r#ref.clone()]), Some(openmodelica_frontend_dump::FCore::ScopeType::CLASS_SCOPE))) else { return Err("pattern mismatch") };
                    Ok(true)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, DAE::FunctionParallelism::FP_NON_PARALLEL { .. }, Deref @ metamodelica::ListNode::Cons { head: r#ref, tail: _ }) => {
                    let true = (FGraph::checkScopeType(&(list![r#ref.clone()]), Some(openmodelica_frontend_dump::FCore::ScopeType::FUNCTION_SCOPE))) else { return Err("pattern mismatch") };
                    Ok(true)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, DAE::FunctionParallelism::FP_NON_PARALLEL { .. }, Deref @ metamodelica::ListNode::Cons { head: r#ref, tail: _ }) => {
                    let mut scopeName: ArcStr;
                    let mut errorString: ArcStr;
                    let false = (FNode::isRefTop(r#ref.clone())) else { return Err("pattern mismatch") };
                    scopeName = FNode::refName(r#ref.clone());
                    let true = (FGraph::checkScopeType(&(list![r#ref.clone()]), Some(openmodelica_frontend_dump::FCore::ScopeType::PARALLEL_SCOPE))) else { return Err("pattern mismatch") };
                    errorString = { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*literal!("- Non-Parallel function '")); __mm_s.push_str(&*AbsynUtil::pathString(inFn.clone(), literal!("."), true, false)?); __mm_s.push_str(&*literal!("' can not be called from a parallel scope.")); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*literal!("- Here called from :")); __mm_s.push_str(&*scopeName); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*literal!("- Please declare the function as parallel function.")); ArcStr::from(__mm_s) };
                    Error::addSourceMessage(&(Error::PARMODELICA_ERROR.clone()), list![errorString.clone()], inInfo)?;
                    Ok(false)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, DAE::FunctionParallelism::FP_PARALLEL_FUNCTION { .. }, Deref @ metamodelica::ListNode::Cons { head: r#ref, tail: _ }) => {
                    let mut scopeName: ArcStr;
                    let false = (FNode::isRefTop(r#ref.clone())) else { return Err("pattern mismatch") };
                    scopeName = FNode::refName(r#ref.clone());
                    let true = (FGraph::checkScopeType(&(list![r#ref.clone()]), Some(openmodelica_frontend_dump::FCore::ScopeType::PARALLEL_SCOPE))) else { return Err("pattern mismatch") };
                    let false = (stringEqual(&scopeName, &(AbsynUtil::pathString(inFn.clone(), literal!("."), true, false)?))) else { return Err("pattern mismatch") };
                    Ok(true)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, DAE::FunctionParallelism::FP_PARALLEL_FUNCTION { .. }, Deref @ metamodelica::ListNode::Cons { head: r#ref, tail: _ }) => {
                    let mut scopeName: ArcStr;
                    let mut errorString: ArcStr;
                    let false = (FNode::isRefTop(r#ref.clone())) else { return Err("pattern mismatch") };
                    scopeName = FNode::refName(r#ref.clone());
                    let true = (FGraph::checkScopeType(&(list![r#ref.clone()]), Some(openmodelica_frontend_dump::FCore::ScopeType::PARALLEL_SCOPE))) else { return Err("pattern mismatch") };
                    let true = (stringEqual(&scopeName, &(AbsynUtil::pathString(inFn.clone(), literal!("."), true, false)?))) else { return Err("pattern mismatch") };
                    errorString = { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*literal!("- Parallel function '")); __mm_s.push_str(&*AbsynUtil::pathString(inFn.clone(), literal!("."), true, false)?); __mm_s.push_str(&*literal!("' can not call itself. Recurrsion is not allowed for parallel functions currently.")); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*literal!("- Parallel functions can only be called from: 'kernel' functions,")); __mm_s.push_str(&*literal!(" OTHER 'parallel' functions (no recurrsion) or from a body of a")); __mm_s.push_str(&*literal!(" 'parfor' loop")); ArcStr::from(__mm_s) };
                    Error::addSourceMessage(&(Error::PARMODELICA_ERROR.clone()), list![errorString.clone()], inInfo)?;
                    Ok(false)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, DAE::FunctionParallelism::FP_PARALLEL_FUNCTION { .. }, Deref @ metamodelica::ListNode::Cons { head: r#ref, tail: _ }) => {
                    let mut scopeName: ArcStr;
                    let false = (FNode::isRefTop(r#ref.clone())) else { return Err("pattern mismatch") };
                    scopeName = FNode::refName(r#ref.clone());
                    let true = (stringEqual(&scopeName, &arcstr::literal!(FCore::parForScopeName))) else { return Err("pattern mismatch") };
                    Ok(true)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, DAE::FunctionParallelism::FP_PARALLEL_FUNCTION { .. }, Deref @ metamodelica::ListNode::Cons { head: r#ref, tail: _ }) => {
                    let mut scopeName: ArcStr;
                    let mut errorString: ArcStr;
                    let false = (FNode::isRefTop(r#ref.clone())) else { return Err("pattern mismatch") };
                    scopeName = FNode::refName(r#ref.clone());
                    errorString = { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*literal!("- Parallel function '")); __mm_s.push_str(&*AbsynUtil::pathString(inFn.clone(), literal!("."), true, false)?); __mm_s.push_str(&*literal!("' can not be called from a non parallel scope '")); __mm_s.push_str(&*scopeName); __mm_s.push_str(&*literal!("'.\n")); __mm_s.push_str(&*literal!("- Parallel functions can only be called from: 'kernel' functions,")); __mm_s.push_str(&*literal!(" other 'parallel' functions (no recurrsion) or from a body of a")); __mm_s.push_str(&*literal!(" 'parfor' loop")); ArcStr::from(__mm_s) };
                    Error::addSourceMessage(&(Error::PARMODELICA_ERROR.clone()), list![errorString.clone()], inInfo)?;
                    Ok(false)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, DAE::FunctionParallelism::FP_KERNEL_FUNCTION { .. }, Deref @ metamodelica::ListNode::Cons { head: r#ref, tail: _ }) => {
                    let mut scopeName: ArcStr;
                    let mut errorString: ArcStr;
                    let false = (FNode::isRefTop(r#ref.clone())) else { return Err("pattern mismatch") };
                    scopeName = FNode::refName(r#ref.clone());
                    let true = (stringEqual(&scopeName, &(AbsynUtil::pathString(inFn.clone(), literal!("."), true, false)?))) else { return Err("pattern mismatch") };
                    errorString = { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*literal!("- Kernel function '")); __mm_s.push_str(&*AbsynUtil::pathString(inFn.clone(), literal!("."), true, false)?); __mm_s.push_str(&*literal!("' can not call itself. ")); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*literal!("- Recurrsion is not allowed for Kernel functions. ")); ArcStr::from(__mm_s) };
                    Error::addSourceMessage(&(Error::PARMODELICA_ERROR.clone()), list![errorString.clone()], inInfo)?;
                    Ok(false)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, DAE::FunctionParallelism::FP_KERNEL_FUNCTION { .. }, Deref @ metamodelica::ListNode::Cons { head: r#ref, tail: _ }) => {
                    let mut scopeName: ArcStr;
                    let mut errorString: ArcStr;
                    let false = (FNode::isRefTop(r#ref.clone())) else { return Err("pattern mismatch") };
                    scopeName = FNode::refName(r#ref.clone());
                    let true = (FGraph::checkScopeType(&(list![r#ref.clone()]), Some(openmodelica_frontend_dump::FCore::ScopeType::PARALLEL_SCOPE))) else { return Err("pattern mismatch") };
                    errorString = { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*literal!("- Kernel function '")); __mm_s.push_str(&*AbsynUtil::pathString(inFn.clone(), literal!("."), true, false)?); __mm_s.push_str(&*literal!("' can not be called from a parallel scope '")); __mm_s.push_str(&*scopeName); __mm_s.push_str(&*literal!("'.\n")); __mm_s.push_str(&*literal!("- Kernel functions CAN NOT be called from: 'kernel' functions,")); __mm_s.push_str(&*literal!(" 'parallel' functions or from a body of a")); __mm_s.push_str(&*literal!(" 'parfor' loop")); ArcStr::from(__mm_s) };
                    Error::addSourceMessage(&(Error::PARMODELICA_ERROR.clone()), list![errorString.clone()], inInfo)?;
                    Ok(false)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, DAE::FunctionParallelism::FP_KERNEL_FUNCTION { .. }, Deref @ metamodelica::ListNode::Cons { head: r#ref, tail: _ }) => {
                    let mut scopeName: ArcStr;
                    let mut errorString: ArcStr;
                    let false = (FNode::isRefTop(r#ref.clone())) else { return Err("pattern mismatch") };
                    scopeName = FNode::refName(r#ref.clone());
                    let true = (stringEqual(&scopeName, &arcstr::literal!(FCore::parForScopeName))) else { return Err("pattern mismatch") };
                    errorString = { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("\n")); __mm_s.push_str(&*literal!("- Kernel function '")); __mm_s.push_str(&*AbsynUtil::pathString(inFn.clone(), literal!("."), true, false)?); __mm_s.push_str(&*literal!("' can not be called from inside parallel for (parfor) loop body.")); __mm_s.push_str(&*literal!("'.\n")); __mm_s.push_str(&*literal!("- Kernel functions CAN NOT be called from: 'kernel' functions,")); __mm_s.push_str(&*literal!(" 'parallel' functions or from a body of a")); __mm_s.push_str(&*literal!(" 'parfor' loop")); ArcStr::from(__mm_s) };
                    Error::addSourceMessage(&(Error::PARMODELICA_ERROR.clone()), list![errorString.clone()], inInfo)?;
                    Ok(false)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, DAE::FunctionParallelism::FP_KERNEL_FUNCTION { .. }, Deref @ metamodelica::ListNode::Cons { head: r#ref, tail: _ }) => {
                    let mut scopeName: ArcStr;
                    let false = (FNode::isRefTop(r#ref.clone())) else { return Err("pattern mismatch") };
                    scopeName = FNode::refName(r#ref.clone());
                    let false = (stringEqual(&scopeName, &(AbsynUtil::pathString(inFn.clone(), literal!("."), true, false)?))) else { return Err("pattern mismatch") };
                    Ok(true)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(true)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    isValid
}

fn elabCallArgsMetarecord(
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
    mut inType: metamodelica::Ref<DAE::Type>,
    mut inPosArgs: metamodelica::List<metamodelica::Ref<Absyn::Exp>>,
    mut inNamedArgs: &metamodelica::List<metamodelica::Ref<Absyn::NamedArg>>,
    mut inImplicit: bool,
    mut stopElab: Mutable::Mutable<bool>,
    mut inPrefix: DAE::Prefix,
    mut inInfo: SourceInfo,
) -> Result<(FCore::Cache, Option<(metamodelica::Ref<DAE::Exp>, DAE::Properties)>)> {
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut expProps: Option<(metamodelica::Ref<DAE::Exp>, DAE::Properties)>;
    (outCache, expProps) = {
        let mut ty = inType.clone();
        'mc: {
            let __mc_input = ty.clone();
            if let Ok(__v) = (|| -> Result<_> {
                ::match_deref::match_deref! { match &__mc_input {
                    Deref @ DAE::Type::T_METARECORD { path: fq_path, .. } => {
                        let mut r#str: ArcStr;
                        let mut fn_str: ArcStr;
                        let __arc1 = List::find(var_field!((*inType).fields, DAE::Type::T_METARECORD), &move |__a0: metamodelica::Ref<DAE::Var>| -> metamodelica::Result<_> { ::std::result::Result::Ok(Types::varHasMetaRecordType(&__a0)) })?;
                        let DAE::TYPES_VAR { name: __pa0, .. } = &*__arc1;
                        r#str = metamodelica::Own::own(__pa0);
                        fn_str = AbsynUtil::pathString(fq_path.clone(), literal!("."), true, false)?;
                        Error::addSourceMessage(&(Error::METARECORD_CONTAINS_METARECORD_MEMBER.clone()), list![fn_str.clone(), r#str.clone()], &inInfo)?;
                        Ok((inCache.clone(), None))
                    }
                    _ => return Err("nomatch"),
                }}
            })() {
                break 'mc __v;
            }
            if let Ok(__v) = (|| -> Result<_> {
                ::match_deref::match_deref! { match &__mc_input {
                    Deref @ DAE::Type::T_METARECORD { .. } => {
                        let mut fn_str: ArcStr;
                        let false = (((var_field!((*inType).fields, DAE::Type::T_METARECORD)).len() as i32) == ((inPosArgs).len() as i32) + ((inNamedArgs).len() as i32)) else { return Err("pattern mismatch") };
                        fn_str = TypesDump::unparseType(inType.clone())?;
                        Error::addSourceMessage(&(Error::WRONG_NO_OF_ARGS.clone()), list![fn_str.clone()], &inInfo)?;
                        Ok((inCache.clone(), None))
                    }
                    _ => return Err("nomatch"),
                }}
            })() {
                break 'mc __v;
            }
            if let Ok((__v, __wb0, __wb1)) = (|| -> Result<_> {
                ::match_deref::match_deref! { match &__mc_input {
                            Deref @ DAE::Type::T_METARECORD { path: fq_path, .. } => {
                                let mut field_names: metamodelica::List<ArcStr>;
                                let mut tys: metamodelica::List<metamodelica::Ref<DAE::Type>>;
                                let mut typeVars: metamodelica::List<metamodelica::Ref<DAE::Type>>;
                                let mut fargs: metamodelica::List<metamodelica::Ref<DAE::FuncArg>>;
                                let mut slots: metamodelica::List<Slot>;
                                let mut const_lst: metamodelica::List<DAE::Const>;
                                let mut r#const: DAE::Const;
                                let mut ty_const: metamodelica::Ref<DAE::TupleConst>;
                                let mut prop: DAE::Properties;
                                let mut args: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                                let mut bindings: metamodelica::List<(ArcStr, metamodelica::List<metamodelica::Ref<DAE::Type>>)>;
                                let mut outCache: FCore::Cache = outCache.clone();
                                let mut ty: metamodelica::Ref<DAE::Type> = ty.clone();
                                field_names = ({
                    let mut __acc: metamodelica::List<ArcStr> = metamodelica::nil();
                    for mut var in (var_field!((*inType).fields, DAE::Type::T_METARECORD).clone()).into_iter().cloned() {
                                let __x = TypesDump::getVarName(&(var.clone()));
                                __acc = cons(__x, __acc);
                    }
                    __acc.reverse()
                });
                                tys = ({
                    let mut __acc: metamodelica::List<metamodelica::Ref<DAE::Type>> = metamodelica::nil();
                    for mut var in (var_field!((*inType).fields, DAE::Type::T_METARECORD).clone()).into_iter().cloned() {
                                let __x = Types::getVarType(&(var.clone()))?;
                                __acc = cons(__x, __acc);
                    }
                    __acc.reverse()
                });
                                fargs = ({
                    let mut __acc: metamodelica::List<metamodelica::Ref<DAE::FuncArg>> = metamodelica::nil();
                    let __thr_src0 = field_names.clone();
                    let mut __thr_it0 = (&__thr_src0).into_iter();
                    let __thr_src1 = tys.clone();
                    let mut __thr_it1 = (&__thr_src1).into_iter();
                    loop {
                                match (__thr_it0.next(), __thr_it1.next()) {
                                    (Some(n), Some(t)) => {
                                        let __x = Types::makeDefaultFuncArg(n.clone(), t.clone());
                                        __acc = cons(__x, __acc);
                                    }
                                    (None, None) => break,
                                    _ => return Err("threaded for: ranges of unequal length"),
                                }
                    }
                    __acc.reverse()
                });
                                slots = makeEmptySlots(&fargs)?;
                                (outCache, _, slots, const_lst, bindings) = elabInputArgs(inCache.clone(), inEnv.clone(), &inPosArgs, inNamedArgs, slots.clone(), true, true, inImplicit, inPrefix.clone(), inInfo.clone(), &(inType.clone()), var_field!((*inType).utPath, DAE::Type::T_METARECORD).clone(), false)?;
                                r#const = List::fold(&const_lst, &fnptr!(Types::constAnd, DAE::Const, DAE::Const), openmodelica_frontend_types::DAE::Const::C_CONST)?;
                                ty_const = elabConsts(inType.clone(), r#const)?;
                                let true = (List::fold(&slots, &fnptr!(slotAnd, Slot, bool), true)?) else { return Err("pattern mismatch") };
                                args = slotListArgs(&slots);
                                if !((bindings).is_empty()) {
                                    bindings = Types::solvePolymorphicBindings(bindings.clone(), &inInfo, var_field!((*inType).path, DAE::Type::T_METARECORD).clone())?;
                                    typeVars = ({
                    let mut __acc: metamodelica::List<metamodelica::Ref<DAE::Type>> = metamodelica::nil();
                    for mut tv in (var_field!((*inType).typeVars, DAE::Type::T_METARECORD).clone()).into_iter().cloned() {
                                let __x = Types::fixPolymorphicRestype(tv.clone(), &bindings, &inInfo)?;
                                __acc = cons(__x, __acc);
                    }
                    __acc.reverse()
                });
                                    assign_variant_field!(ty => DAE::Type::T_METARECORD; typeVars = typeVars.clone());
                                    prop = getProperties(ty.clone(), ty_const.clone())?;
                                } else {
                                    prop = getProperties(ty.clone(), ty_const.clone())?;
                                }
                                Ok(((outCache.clone(), Some((metamodelica::Ref::new(DAE::Exp::METARECORDCALL { path: fq_path.clone(), args: args.clone(), fieldNames: field_names.clone(), index: var_field!((*inType).index, DAE::Type::T_METARECORD).clone(), typeVars: var_field!((*inType).typeVars, DAE::Type::T_METARECORD).clone() }), prop.clone()))), outCache.clone(), ty.clone()))
                            }
                            _ => return Err("nomatch"),
                        }}
            })() {
                outCache = __wb0;
                ty = __wb1;
                break 'mc __v;
            }
            if let Ok((__v, __wb0)) = (|| -> Result<_> {
                ::match_deref::match_deref! { match &__mc_input {
                            Deref @ DAE::Type::T_METARECORD { path: fq_path, .. } => {
                                let mut r#str: ArcStr;
                                let mut fn_str: ArcStr;
                                let mut tys: metamodelica::List<metamodelica::Ref<DAE::Type>>;
                                let mut prop: DAE::Properties;
                                let mut outCache: FCore::Cache = outCache.clone();
                                (outCache, _, prop) = elabExpInExpression(inCache.clone(), inEnv.clone(), metamodelica::Ref::new(Absyn::Exp::TUPLE { expressions: inPosArgs.clone() }), false, false, inPrefix.clone(), inInfo.clone())?;
                                tys = ({
                    let mut __acc: metamodelica::List<metamodelica::Ref<DAE::Type>> = metamodelica::nil();
                    for mut var in (var_field!((*inType).fields, DAE::Type::T_METARECORD).clone()).into_iter().cloned() {
                                let __x = Types::getVarType(&(var.clone()))?;
                                __acc = cons(__x, __acc);
                    }
                    __acc.reverse()
                });
                                r#str = { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Failed to match types:\n    actual:   ")); __mm_s.push_str(&*TypesDump::unparseType(Types::getPropType(&prop))?); __mm_s.push_str(&*literal!("\n    expected: ")); __mm_s.push_str(&*TypesDump::unparseType(metamodelica::Ref::new(DAE::Type::T_TUPLE { types: tys.clone(), names: None }))?); ArcStr::from(__mm_s) };
                                fn_str = AbsynUtil::pathString(fq_path.clone(), literal!("."), true, false)?;
                                Error::addSourceMessage(&(Error::META_RECORD_FOUND_FAILURE.clone()), list![fn_str.clone(), r#str.clone()], &inInfo)?;
                                Ok(((outCache.clone(), None), outCache.clone()))
                            }
                            _ => return Err("nomatch"),
                        }}
            })() {
                outCache = __wb0;
                break 'mc __v;
            }
            if let Ok(__v) = (|| -> Result<_> {
                ::match_deref::match_deref! { match &__mc_input {
                    Deref @ DAE::Type::T_METARECORD { path: fq_path, .. } => {
                        let mut r#str: ArcStr;
                        let mut fn_str: ArcStr;
                        r#str = { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Failed to elaborate arguments ")); __mm_s.push_str(&*Dump::printExpStr(metamodelica::Ref::new(Absyn::Exp::TUPLE { expressions: inPosArgs.clone() }))?); ArcStr::from(__mm_s) };
                        fn_str = AbsynUtil::pathString(fq_path.clone(), literal!("."), true, false)?;
                        Error::addSourceMessage(&(Error::META_RECORD_FOUND_FAILURE.clone()), list![fn_str.clone(), r#str.clone()], &inInfo)?;
                        Ok((inCache.clone(), None))
                    }
                    _ => return Err("nomatch"),
                }}
            })() {
                break 'mc __v;
            }
            return Err("matchcontinue: no arm matched");
        }
    };
    Ok((outCache, expProps))
}

#[derive(Clone, Copy, Debug, Eq, Hash, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub(crate) enum ForceFunctionInst {
    /// Used when blocking function instantiation to instantiate the function anyway
    FORCE_FUNCTION_INST,
    /// Used when blocking function instantiation to instantiate the function anyway
    NORMAL_FUNCTION_INST,
}
impl metamodelica::gc::MMTrace for ForceFunctionInst {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        match self {
            ForceFunctionInst::FORCE_FUNCTION_INST => Ok(()),
            ForceFunctionInst::NORMAL_FUNCTION_INST => Ok(()),
        }
    }
}
pub(crate) use self::ForceFunctionInst::{FORCE_FUNCTION_INST, NORMAL_FUNCTION_INST};

pub fn instantiateDaeFunction(
    mut inCache: FCore::Cache,
    mut env: FCore::Graph,
    mut name: metamodelica::Ref<Absyn::Path>,
    mut builtin: bool,
    mut clOpt: Option<metamodelica::Ref<SCode::Element>>,
    mut printErrorMsg: bool,
) -> (FCore::Cache, Util::Status) {
    let mut outCache: FCore::Cache;
    let mut status: Util::Status;
    (outCache, status) = instantiateDaeFunction2(
        inCache,
        env,
        name,
        builtin,
        clOpt,
        printErrorMsg,
        crate::Static::ForceFunctionInst::NORMAL_FUNCTION_INST,
    );
    (outCache, status)
}

pub(crate) fn instantiateDaeFunctionFromTypes<'__b>(
    mut inCache: &'__b FCore::Cache,
    mut env: &'__b FCore::Graph,
    mut tys: metamodelica::List<metamodelica::Ref<DAE::Type>>,
    mut builtin: bool,
    mut clOpt: Option<metamodelica::Ref<SCode::Element>>,
    mut printErrorMsg: bool,
    mut acc: Util::Status,
) -> (FCore::Cache, Util::Status) {
    let mut outCache: FCore::Cache;
    let mut status: Util::Status;
    (outCache, status) = (::match_deref::match_deref! { match &((tys, acc)) {
        (Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Type::T_FUNCTION { path: name, .. }, tail: rest }, Util::Status::SUCCESS { .. }) => {
            (outCache, status) = instantiateDaeFunction(inCache.clone(), env.clone(), name.clone(), builtin, clOpt.clone(), printErrorMsg);
            instantiateDaeFunctionFromTypes(inCache, env, rest.clone(), builtin, clOpt, printErrorMsg, status)
        },
        _ => {
            (inCache.clone(), acc)
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    (outCache, status)
}

pub fn instantiateDaeFunctionForceInst(
    mut inCache: FCore::Cache,
    mut env: FCore::Graph,
    mut name: metamodelica::Ref<Absyn::Path>,
    mut builtin: bool,
    mut clOpt: Option<metamodelica::Ref<SCode::Element>>,
    mut printErrorMsg: bool,
) -> (FCore::Cache, Util::Status) {
    let mut outCache: FCore::Cache;
    let mut status: Util::Status;
    (outCache, status) = instantiateDaeFunction2(
        inCache,
        env,
        name,
        builtin,
        clOpt,
        printErrorMsg,
        crate::Static::ForceFunctionInst::FORCE_FUNCTION_INST,
    );
    (outCache, status)
}

fn instantiateDaeFunction2(
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
    mut inName: metamodelica::Ref<Absyn::Path>,
    mut builtin: bool,
    mut clOpt: Option<metamodelica::Ref<SCode::Element>>,
    mut printErrorMsg: bool,
    mut forceFunctionInst: ForceFunctionInst,
) -> (FCore::Cache, Util::Status) {
    let mut outCache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut status: Util::Status;
    let mut numError: i32 = Error::getNumErrorMessages();
    let mut instOnlyForcedFunctions: bool =
        (openmodelica_util::Globals::instOnlyForcedFunctions.with(|__root| __root.borrow().clone())).is_some();
    (outCache, status) = 'mc: {
        let __mc_input = (builtin, clOpt, instOnlyForcedFunctions, forceFunctionInst);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, true, ForceFunctionInst::NORMAL_FUNCTION_INST { .. }) => {
                    let false = (AbsynUtil::pathIsIdent(&inName)) else { return Err("pattern mismatch") };
                    Ok((inCache.clone(), openmodelica_util::Util::Status::SUCCESS))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (true, _, _, _) => {
                    Ok((inCache.clone(), openmodelica_util::Util::Status::SUCCESS))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _, ForceFunctionInst::NORMAL_FUNCTION_INST { .. }) => {
                    let (_, true) = (isExternalObjectFunction(inCache.clone(), &inEnv, &inName)) else { return Err("pattern mismatch") };
                    Ok((inCache.clone(), openmodelica_util::Util::Status::SUCCESS))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, None, _, _) => {
                    let false = (FGraph::isTopScope(&inEnv)) else { return Err("pattern mismatch") };
                    let true = (AbsynUtil::pathSuffixOf(&inName, &(FGraph::getGraphName(&inEnv)?))) else { return Err("pattern mismatch") };
                    Ok((inCache.clone(), openmodelica_util::Util::Status::SUCCESS))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _, _) => {
                    let mut name: metamodelica::Ref<Absyn::Path>;
                    let mut outCache: FCore::Cache = outCache.clone();
                    (outCache, _, _, name) = lookupAndFullyQualify(inCache.clone(), inEnv.clone(), &inName)?;
                    FCore::checkCachedInstFuncGuard(&outCache, name.clone())?;
                    Ok(((outCache.clone(), openmodelica_util::Util::Status::SUCCESS), outCache.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            outCache = __wb0;
            break 'mc __v;
        }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, None, _, _) => {
                    let mut env: FCore::Graph;
                    let mut cl: metamodelica::Ref<SCode::Element>;
                    let mut name: metamodelica::Ref<Absyn::Path>;
                    let mut outCache: FCore::Cache = outCache.clone();
                    (outCache, env, cl, name) = lookupAndFullyQualify(inCache.clone(), inEnv.clone(), &inName)?;
                    outCache = FCore::addCachedInstFuncGuard(outCache.clone(), name.clone())?;
                    (outCache, _, _) = InstFunction::implicitFunctionInstantiation(outCache.clone(), env.clone(), InnerOuter::emptyInstHierarchy().clone(), openmodelica_frontend_types::DAE::Mod::interned_NOMOD(), openmodelica_frontend_types::DAE::Prefix::NOPRE, cl.clone(), metamodelica::nil())?;
                    Ok(((outCache.clone(), openmodelica_util::Util::Status::SUCCESS), outCache.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            outCache = __wb0;
            break 'mc __v;
        }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Some(cl), _, _) => {
                    let mut outCache: FCore::Cache = outCache.clone();
                    (outCache, _) = Inst::makeFullyQualified(inCache.clone(), inEnv.clone(), inName.clone())?;
                    (outCache, _, _) = InstFunction::implicitFunctionInstantiation(outCache.clone(), inEnv.clone(), InnerOuter::emptyInstHierarchy().clone(), openmodelica_frontend_types::DAE::Mod::interned_NOMOD(), openmodelica_frontend_types::DAE::Prefix::NOPRE, cl.clone(), metamodelica::nil())?;
                    Ok(((outCache.clone(), openmodelica_util::Util::Status::SUCCESS), outCache.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            outCache = __wb0;
            break 'mc __v;
        }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, None, _, _) => {
                    let mut cref: metamodelica::Ref<DAE::ComponentRef>;
                    let mut ty: metamodelica::Ref<DAE::Type>;
                    let mut outCache: FCore::Cache = outCache.clone();
                    cref = ComponentReference::pathToCref(&inName);
                    (outCache, _, ty, _, _, _, _, _, _) = Lookup::lookupVar(inCache.clone(), inEnv.clone(), cref.clone())?;
                    ::match_deref::match_deref! { match &(ty.clone()) {
                        Deref @ DAE::Type::T_FUNCTION { .. } => (),
                        _ => return Err("pattern mismatch"),
                    } };
                    Ok(((outCache.clone(), openmodelica_util::Util::Status::SUCCESS), outCache.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            outCache = __wb0;
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, true, _) => {
                    let mut pathStr: ArcStr;
                    let mut envStr: ArcStr;
                    let true = (Error::getNumErrorMessages() == numError) else { return Err("pattern mismatch") };
                    envStr = FGraph::printGraphPathStr(&inEnv);
                    pathStr = AbsynUtil::pathString(inName.clone(), literal!("."), true, false)?;
                    Error::addMessage(Error::GENERIC_INST_FUNCTION.clone(), list![pathStr.clone(), envStr.clone()])?;
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((inCache.clone(), openmodelica_util::Util::Status::FAILURE))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    (outCache, status)
}

fn lookupAndFullyQualify(
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
    mut inFunctionName: &metamodelica::Ref<Absyn::Path>,
) -> Result<(
    FCore::Cache,
    FCore::Graph,
    metamodelica::Ref<SCode::Element>,
    metamodelica::Ref<Absyn::Path>,
)> {
    let mut outCache: FCore::Cache;
    let mut outEnv: FCore::Graph;
    let mut outClass: metamodelica::Ref<SCode::Element>;
    let mut outFunctionName: metamodelica::Ref<Absyn::Path>;
    if Lookup::isFunctionCallViaComponent(inCache.clone(), inEnv.clone(), inFunctionName) {
        (_, outClass, outEnv) = Lookup::lookupClass(&inCache, &inEnv, inFunctionName, None)?;
        outFunctionName = FGraph::joinScopePath(
            &outEnv,
            AbsynUtil::makeIdentPathFromString(SCodeUtil::elementName(&outClass)?),
        )?;
        outCache = inCache;
    } else {
        (outCache, outClass, outEnv) = Lookup::lookupClass(&inCache, &inEnv, inFunctionName, None)?;
        outFunctionName = AbsynUtil::makeFullyQualified(FGraph::joinScopePath(
            &outEnv,
            AbsynUtil::makeIdentPathFromString(SCodeUtil::elementName(&outClass)?),
        )?);
    }
    Ok((outCache, outEnv, outClass, outFunctionName))
}

fn instantiateImplicitRecordConstructors(
    mut inCache: &FCore::Cache,
    mut inEnv: &FCore::Graph,
    mut args: &metamodelica::List<metamodelica::Ref<DAE::Exp>>,
) -> FCore::Cache {
    let mut outCache: FCore::Cache;
    outCache = 'mc: {
        let __mc_input = &**args;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Nil => {
                    Ok(inCache.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Exp::CREF { ty: Deref @ DAE::Type::T_COMPLEX { complexClassType: ClassInf::State::RECORD { path: record_name }, .. }, .. }, tail: rest_args } => {
                    let mut cache: FCore::Cache;
                    let (__pa0, Util::SUCCESS { .. }) = (instantiateDaeFunction(inCache.clone(), inEnv.clone(), record_name.clone(), false, None, false)) else { return Err("pattern mismatch") };
                    cache = metamodelica::Own::own(__pa0);
                    Ok(instantiateImplicitRecordConstructors(&cache, inEnv, metamodelica::AsArg::as_arg(&rest_args)))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: _, tail: rest_args } => {
                    Ok(instantiateImplicitRecordConstructors(inCache, inEnv, metamodelica::AsArg::as_arg(&rest_args)))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    outCache
}

fn addDefaultArgs(
    mut inSlots: metamodelica::List<Slot>,
    mut inInfo: SourceInfo,
) -> Result<(
    metamodelica::List<metamodelica::Ref<DAE::Exp>>,
    metamodelica::List<Slot>,
)> {
    let mut outArgs: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
    let mut outSlots: metamodelica::List<Slot>;
    (outArgs, outSlots) = List::map2_2(
        &(inSlots.clone()),
        &fillDefaultSlot,
        metamodelica::arrayFromVec(inSlots.into_iter().cloned().collect()),
        inInfo,
    )?;
    Ok((outArgs, outSlots))
}

fn fillDefaultSlot(
    mut inSlot: Slot,
    mut inSlotArray: metamodelica::Array<Slot>,
    mut inInfo: SourceInfo,
) -> Result<(metamodelica::Ref<DAE::Exp>, Slot)> {
    let mut outArg: metamodelica::Ref<DAE::Exp>;
    let mut outSlot: Slot;
    (outArg, outSlot) = (::match_deref::match_deref! { match &(inSlot.clone()) {
        Slot { slotFilled: true, arg: Some(arg), .. } => {
            (arg.clone(), inSlot)
        },
        Slot { slotFilled: false, defaultArg: Deref @ DAE::FuncArg { defaultBinding: Some(_), .. }, idx, .. } => {
            fillDefaultSlot2(({let __elt = (*metamodelica::index_checked(&inSlotArray.borrow(), idx.clone())?).clone(); __elt}), inSlotArray.clone(), inInfo)?
        },
        Slot { defaultArg: Deref @ DAE::FuncArg { name: id, .. }, .. } => {
            Error::addSourceMessage(&(Error::UNFILLED_SLOT.clone()), list![id.clone()], &inInfo)?;
            return Err("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outArg, outSlot))
}

fn fillDefaultSlot2(
    mut inSlot: Slot,
    mut inSlotArray: metamodelica::Array<Slot>,
    mut inInfo: SourceInfo,
) -> Result<(metamodelica::Ref<DAE::Exp>, Slot)> {
    let mut outArg: metamodelica::Ref<DAE::Exp>;
    let mut outSlot: Slot = inSlot.clone();
    (outArg, outSlot) = (::match_deref::match_deref! { match &(outSlot.clone()) {
        Slot { arg: Some(exp), evalStatus: 2, .. } => {
            (exp.clone(), inSlot)
        },
        Slot { defaultArg: Deref @ DAE::FuncArg { name: id, .. }, evalStatus: 1, .. } => {
            Error::addSourceMessage(&(Error::CYCLIC_DEFAULT_VALUE.clone()), list![id.clone()], &inInfo)?;
            return Err("fail")
        },
        Slot { defaultArg: Deref @ DAE::FuncArg { defaultBinding: Some(exp), .. }, idx, evalStatus: 0, .. } => {
            let mut exp = (*exp).clone();
            outSlot.evalStatus = SLOT_EVALUATING.clone();
            metamodelica::arrayUpdate(inSlotArray.clone(), idx.clone(), outSlot.clone())?;
            exp = evaluateSlotExp(exp.clone(), inSlotArray.clone(), inInfo)?;
            outSlot.arg = Some(exp.clone());
            outSlot.slotFilled = true;
            outSlot.evalStatus = SLOT_EVALUATED.clone();
            metamodelica::arrayUpdate(inSlotArray.clone(), idx.clone(), outSlot.clone())?;
            (exp.clone(), outSlot)
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok((outArg, outSlot))
}

fn evaluateSlotExp(
    mut inExp: metamodelica::Ref<DAE::Exp>,
    mut inSlotArray: metamodelica::Array<Slot>,
    mut inInfo: SourceInfo,
) -> Result<metamodelica::Ref<DAE::Exp>> {
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    (outExp, _) = Expression::traverseExpBottomUp(inExp, &evaluateSlotExp_traverser, (inSlotArray.clone(), inInfo))?;
    Ok(outExp)
}

fn evaluateSlotExp_traverser(
    mut inExp: metamodelica::Ref<DAE::Exp>,
    mut inTuple: (metamodelica::Array<Slot>, SourceInfo),
) -> Result<(metamodelica::Ref<DAE::Exp>, (metamodelica::Array<Slot>, SourceInfo))> {
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    let mut outTuple: (metamodelica::Array<Slot>, SourceInfo);
    (outExp, outTuple) = (::match_deref::match_deref! { match &((inExp.clone(), inTuple.clone())) {
        (orig_exp @ Deref @ DAE::Exp::CREF { componentRef: Deref @ DAE::ComponentRef::CREF_IDENT { ident: id, .. }, .. }, (slots, info)) => {
            let mut slot: Option<Slot>;
            let mut exp: metamodelica::Ref<DAE::Exp>;
            slot = lookupSlotInArray(id.clone(), slots.clone());
            exp = getOptSlotDefaultExp(slot, slots.clone(), info.clone(), orig_exp.clone())?;
            (exp, (slots.clone(), info.clone()))
        },
        _ => {
            (inExp, inTuple)
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outExp, outTuple))
}

fn lookupSlotInArray(mut inSlotName: ArcStr, mut inSlots: metamodelica::Array<Slot>) -> Option<Slot> {
    let mut outSlot: Option<Slot>;
    let mut slot: Slot;
    match '__try0: {
        (slot, _) = unwrap_break_err!(Array::getMemberOnTrue(inSlotName.clone(), inSlots.clone(), &move |__a0: ArcStr, __a1: Slot| isSlotNamed(&__a0, __a1)), '__try0);
        outSlot = Some(slot.clone());
        Ok::<_, &'static str>((outSlot.clone(),))
    } {
        Ok((__try0_o0,)) => {
            outSlot = __try0_o0;
        }
        Err(_) => {
            outSlot = None;
        }
    }
    outSlot
}

fn isSlotNamed(mut inName: &ArcStr, mut inSlot: Slot) -> Result<bool> {
    let mut outIsNamed: bool;
    let mut id: ArcStr;
    let Slot { defaultArg: __t1, .. } = inSlot;
    let __arc2 = __t1.clone();
    let DAE::FUNCARG { name: __pa0, .. } = &*__arc2;
    id = metamodelica::Own::own(__pa0);
    outIsNamed = stringEq(&id, &inName);
    Ok(outIsNamed)
}

fn getOptSlotDefaultExp(
    mut inSlot: Option<Slot>,
    mut inSlots: metamodelica::Array<Slot>,
    mut inInfo: SourceInfo,
    mut inOrigExp: metamodelica::Ref<DAE::Exp>,
) -> Result<metamodelica::Ref<DAE::Exp>> {
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    outExp = (match inSlot {
        Some(mut slot) => {
            let mut exp: metamodelica::Ref<DAE::Exp>;
            (exp, _) = fillDefaultSlot(slot, inSlots.clone(), inInfo)?;
            exp
        }
        None => inOrigExp,
    });
    Ok(outExp)
}

fn determineConstSpecialFunc(
    mut inCache: FCore::Cache,
    mut inEnv: &FCore::Graph,
    mut inConst: DAE::Const,
    mut inFuncName: &metamodelica::Ref<Absyn::Path>,
) -> (FCore::Cache, DAE::Const) {
    let mut outCache: FCore::Cache;
    let mut outConst: DAE::Const;
    let mut is_ext: bool;
    (outCache, is_ext) = isExternalObjectFunction(inCache, inEnv, inFuncName);
    outConst = if (is_ext) {
        openmodelica_frontend_types::DAE::Const::C_VAR
    } else {
        inConst
    };
    (outCache, outConst)
}

pub fn isExternalObjectFunction(
    mut inCache: FCore::Cache,
    mut inEnv: &FCore::Graph,
    mut inPath: &metamodelica::Ref<Absyn::Path>,
) -> (FCore::Cache, bool) {
    let mut outCache: FCore::Cache;
    let mut outIsExt: bool;
    let mut els: metamodelica::List<metamodelica::Ref<SCode::Element>>;
    let mut last_id: ArcStr;
    match '__try0: {
        let (__pa1, __pa2) = ::match_deref::match_deref! { match &(unwrap_break_err!(Lookup::lookupClass(&inCache, inEnv, inPath, None), '__try0)) {
            (__pa1, Deref @ SCode::Element::CLASS { classDef: Deref @ SCode::ClassDef::PARTS { elementLst: __pa2, .. }, .. }, _) => (__pa1.clone(), __pa2.clone()),
            _ => break '__try0 Err::<_, _>("pattern mismatch"),
        } };
        outCache = metamodelica::Own::own(__pa1);
        els = metamodelica::Own::own(__pa2);
        let true = (SCodeUtil::isExternalObject(&els)) else {
            break '__try0 Err::<_, _>("pattern mismatch");
        };
        outIsExt = true;
        Ok::<_, &'static str>((outCache.clone(), outIsExt.clone()))
    } {
        Ok((__try0_o0, __try0_o1)) => {
            outCache = __try0_o0;
            outIsExt = __try0_o1;
        }
        Err(_) => {
            last_id = AbsynUtil::pathLastIdent(inPath);
            outCache = inCache.clone();
            outIsExt = metamodelica::stringEq(&last_id, &(literal!("constructor")))
                || metamodelica::stringEq(&last_id, &(literal!("destructor")));
        }
    }
    (outCache, outIsExt)
}

pub(crate) const vectorizeArg: &'static str = "$vectorizeArg";

fn vectorizeCall(
    mut inExp: metamodelica::Ref<DAE::Exp>,
    mut inDims: &metamodelica::List<metamodelica::Ref<DAE::Dimension>>,
    mut inSlots: &metamodelica::List<Slot>,
    mut inProperties: DAE::Properties,
    mut info: &SourceInfo,
) -> Result<(metamodelica::Ref<DAE::Exp>, DAE::Properties)> {
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    let mut outProperties: DAE::Properties;
    (outExp, outProperties) = 'mc: {
        let __mc_input = (inExp.clone(), &**inDims, inProperties);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e, Deref @ metamodelica::ListNode::Nil, prop) => {
                    Ok((e.clone(), prop.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e, Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Dimension::DIM_UNKNOWN { .. }, tail: ad }, prop) => {
                    if !((Flags::getConfigBool(Flags::CHECK_MODEL.clone())?)) { return Err("guard") }
                    Ok(vectorizeCall(e.clone(), &(metamodelica::cons(metamodelica::Ref::new(DAE::Dimension::DIM_INTEGER { integer: 1 }), ad.clone())), inSlots, prop.clone(), info)?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e @ Deref @ DAE::Exp::CALL { .. }, Deref @ metamodelica::ListNode::Cons { head: dim, tail: ad }, DAE::Properties::PROP { type_: tp, constFlag: c }) => {
                    let mut vect_exp: metamodelica::Ref<DAE::Exp>;
                    let mut exp_type: metamodelica::Ref<DAE::Type>;
                    let mut int_dim: i32;
                    let mut tp = (*tp).clone();
                    int_dim = Expression::dimensionSize(metamodelica::AsArg::as_arg(&dim))?;
                    exp_type = Types::simplifyType(Types::liftArray(tp.clone(), dim.clone()))?;
                    vect_exp = vectorizeCallScalar(metamodelica::AsArg::as_arg(&e), &exp_type, int_dim, inSlots.clone())?;
                    tp = Types::liftArray(tp.clone(), dim.clone());
                    Ok(vectorizeCall(vect_exp.clone(), metamodelica::AsArg::as_arg(&ad), inSlots, DAE::Properties::PROP { type_: tp.clone(), constFlag: c.clone() }, info)?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::ARRAY { .. }, Deref @ metamodelica::ListNode::Cons { head: dim, tail: ad }, DAE::Properties::PROP { type_: tp, constFlag: c }) => {
                    let mut vect_exp: metamodelica::Ref<DAE::Exp>;
                    let mut int_dim: i32;
                    let mut tp = (*tp).clone();
                    int_dim = Expression::dimensionSize(metamodelica::AsArg::as_arg(&dim))?;
                    vect_exp = vectorizeCallArray(&inExp, int_dim, inSlots.clone())?;
                    tp = Types::liftArrayRight(tp.clone(), dim.clone());
                    Ok(vectorizeCall(vect_exp.clone(), metamodelica::AsArg::as_arg(&ad), inSlots, DAE::Properties::PROP { type_: tp.clone(), constFlag: c.clone() }, info)?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CALL { path: r#fn, expLst: es, attr }, Deref @ metamodelica::ListNode::Cons { head: dim, tail: ad }, prop @ DAE::Properties::PROP { type_: tp, constFlag: c }) => {
                    let mut e: metamodelica::Ref<DAE::Exp>;
                    let mut rinfo: metamodelica::Ref<DAE::ReductionInfo>;
                    let mut foldName: ArcStr;
                    let mut resultName: ArcStr;
                    let mut riters: metamodelica::List<metamodelica::Ref<DAE::ReductionIterator>>;
                    let mut iterType: Absyn::ReductionIterType;
                    let mut es = (*es).clone();
                    let mut prop = (*prop).clone();
                    let mut tp = (*tp).clone();
                    (es, riters) = vectorizeCallUnknownDimension(metamodelica::AsArg::as_arg(&es), inSlots.clone(), info)?;
                    tp = Types::liftArrayRight(tp.clone(), dim.clone());
                    prop = DAE::Properties::PROP { type_: tp.clone(), constFlag: c.clone() };
                    e = metamodelica::Ref::new(DAE::Exp::CALL { path: r#fn.clone(), expLst: es.clone(), attr: attr.clone() });
                    (e, prop) = vectorizeCall(e.clone(), metamodelica::AsArg::as_arg(&ad), inSlots, prop.clone(), info)?;
                    foldName = Util::getTempVariableIndex();
                    resultName = Util::getTempVariableIndex();
                    iterType = if (((riters).len() as i32) > 1) {openmodelica_ast::Absyn::ReductionIterType::THREAD} else {openmodelica_ast::Absyn::ReductionIterType::COMBINE};
                    rinfo = metamodelica::Ref::new(DAE::ReductionInfo { path: metamodelica::Ref::new(Absyn::Path::IDENT { name: literal!("array") }), iterType: iterType, exprType: tp.clone(), defaultValue: Some(metamodelica::Ref::new(Values::Value::ARRAY { valueLst: metamodelica::nil(), dimLst: list![0] })), foldName: foldName.clone(), resultName: resultName.clone(), foldExp: None });
                    Ok((metamodelica::Ref::new(DAE::Exp::REDUCTION { reductionInfo: rinfo.clone(), expr: e.clone(), iterators: riters.clone() }), prop.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CALL { .. }, Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Dimension::DIM_EXP { .. }, tail: _ }, DAE::Properties::PROP { .. }) => {
                    let mut r#str: ArcStr;
                    r#str = { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Cannot vectorize call with dimensions [")); __mm_s.push_str(&*ExpressionBasics::dimensionsString(inDims.clone())?); __mm_s.push_str(&*literal!("]")); ArcStr::from(__mm_s) };
                    Error::addSourceMessage(&(Error::INTERNAL_ERROR.clone()), list![r#str.clone()], info)?;
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut r#str: ArcStr;
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { return Err("pattern mismatch") };
                    r#str = ExpressionBasics::dimensionString(&((inDims).head().cloned()?))?;
                    Debug::traceln({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("- Static.vectorizeCall failed: ")); __mm_s.push_str(&*r#str); ArcStr::from(__mm_s) })?;
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((outExp, outProperties))
}

fn vectorizeCallUnknownDimension(
    mut inEs: &metamodelica::List<metamodelica::Ref<DAE::Exp>>,
    mut inSlots: metamodelica::List<Slot>,
    mut info: &SourceInfo,
) -> Result<(
    metamodelica::List<metamodelica::Ref<DAE::Exp>>,
    metamodelica::List<metamodelica::Ref<DAE::ReductionIterator>>,
)> {
    let mut oes: metamodelica::List<metamodelica::Ref<DAE::Exp>> = metamodelica::nil();
    let mut ofound: metamodelica::List<metamodelica::Ref<DAE::ReductionIterator>> = metamodelica::nil();
    let mut rest_slots: metamodelica::List<Slot> = inSlots;
    let mut dims: metamodelica::List<metamodelica::Ref<DAE::Dimension>>;
    let mut ty: metamodelica::Ref<DAE::Type>;
    let mut tp: metamodelica::Ref<DAE::Type>;
    let mut name: ArcStr;
    for mut e in &**inEs {
        let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(rest_slots) {
            Deref @ metamodelica::ListNode::Cons { head: Slot { dims: __pa0, defaultArg: Deref @ DAE::FuncArg { ty: __pa1, .. }, .. }, tail: __pa2 } => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
            _ => return Err("pattern mismatch"),
        } };
        dims = metamodelica::Own::own(__pa0);
        ty = metamodelica::Own::own(__pa1);
        rest_slots = metamodelica::Own::own(__pa2);
        if (dims).is_empty() {
            oes = metamodelica::cons(e.clone(), oes);
        } else {
            name = Util::getTempVariableIndex();
            tp = Types::expTypetoTypesType(&(Expression::r#typeof(e.clone())?));
            ofound = metamodelica::cons(
                metamodelica::Ref::new(DAE::ReductionIterator {
                    id: name.clone(),
                    exp: e.clone(),
                    guardExp: None,
                    ty: tp,
                }),
                ofound,
            );
            oes = metamodelica::cons(
                metamodelica::Ref::new(DAE::Exp::CREF {
                    componentRef: metamodelica::Ref::new(DAE::ComponentRef::CREF_IDENT {
                        ident: name,
                        identType: ty.clone(),
                        subscriptLst: metamodelica::nil(),
                    }),
                    ty: ty,
                }),
                oes,
            );
        }
    }
    if (ofound).is_empty() {
        Error::addSourceMessageAndFail(
            &(Error::INTERNAL_ERROR.clone()),
            list![literal!(
                "Static.vectorizeCallUnknownDimension could not find any slot to vectorize"
            )],
            info,
        )?;
        unreachable!("Error.addSourceMessageAndFail always fails — caller-side flow-analysis hint");
    }
    oes = oes.reverse();
    ofound = ofound.reverse();
    Ok((oes, ofound))
}

fn vectorizeCallArray(
    mut inExp: &metamodelica::Ref<DAE::Exp>,
    mut inDim: i32,
    mut inSlots: metamodelica::List<Slot>,
) -> Result<metamodelica::Ref<DAE::Exp>> {
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    let mut ty: metamodelica::Ref<DAE::Type>;
    let mut expl: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
    let mut sc: bool;
    let (__pa0, __pa1) = ::match_deref::match_deref! { match &((*inExp)) {
        Deref @ DAE::Exp::ARRAY { ty: __pa0, array: __pa1, .. } => (__pa0.clone(), __pa1.clone()),
        _ => return Err("pattern mismatch"),
    } };
    ty = metamodelica::Own::own(__pa0);
    expl = metamodelica::Own::own(__pa1);
    expl = vectorizeCallArray2(expl, &ty, inDim, inSlots)?;
    sc = Expression::typeBuiltin(&ty);
    ty = Expression::liftArrayRight(
        &ty,
        metamodelica::Ref::new(DAE::Dimension::DIM_INTEGER { integer: inDim }),
    );
    outExp = metamodelica::Ref::new(DAE::Exp::ARRAY {
        ty: ty,
        scalar: sc,
        array: expl,
    });
    Ok(outExp)
}

fn vectorizeCallArray2(
    mut inExpl: metamodelica::List<metamodelica::Ref<DAE::Exp>>,
    mut inType: &metamodelica::Ref<DAE::Type>,
    mut inDim: i32,
    mut inSlots: metamodelica::List<Slot>,
) -> Result<metamodelica::List<metamodelica::Ref<DAE::Exp>>> {
    let mut outExpl: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
    outExpl = ({
        let mut __acc: metamodelica::List<metamodelica::Ref<DAE::Exp>> = metamodelica::nil();
        for mut e in (inExpl).into_iter().cloned() {
            let __x = (match &*e.clone() {
                DAE::Exp::CALL { .. } => vectorizeCallScalar(&(e.clone()), inType, inDim, inSlots.clone())?,
                DAE::Exp::ARRAY { .. } => vectorizeCallArray(&(e.clone()), inDim, inSlots.clone())?,
                _ => return Err("match: no arm matched"),
            });
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    Ok(outExpl)
}

fn vectorizeCallScalar(
    mut exp: &metamodelica::Ref<DAE::Exp>,
    mut ty: &metamodelica::Ref<DAE::Type>,
    mut dim: i32,
    mut slots: metamodelica::List<Slot>,
) -> Result<metamodelica::Ref<DAE::Exp>> {
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    outExp = 'mc: {
        let __mc_input = &**exp;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::CALL { .. } => {
                    let mut expl: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                    let mut scalar: bool;
                    let mut new_exp: metamodelica::Ref<DAE::Exp>;
                    let mut e_type: metamodelica::Ref<DAE::Type>;
                    let mut arr_type: metamodelica::Ref<DAE::Type>;
                    expl = vectorizeCallScalar2(var_field!((**exp).path, DAE::Exp::CALL).clone(), var_field!((**exp).expLst, DAE::Exp::CALL), var_field!((**exp).attr, DAE::Exp::CALL).clone(), slots.clone(), dim)?;
                    e_type = Expression::unliftArray(ty)?;
                    scalar = Expression::typeBuiltin(&e_type);
                    arr_type = metamodelica::Ref::new(DAE::Type::T_ARRAY { ty: e_type.clone(), dims: list![metamodelica::Ref::new(DAE::Dimension::DIM_INTEGER { integer: dim })] });
                    new_exp = metamodelica::Ref::new(DAE::Exp::ARRAY { ty: arr_type.clone(), scalar: scalar, array: expl.clone() });
                    Ok(new_exp.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { return Err("pattern mismatch") };
                    Debug::trace(literal!("-Static.vectorizeCallScalar failed\n"))?;
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(outExp)
}

fn vectorizeCallScalar2(
    mut r#fn: metamodelica::Ref<Absyn::Path>,
    mut exps: &metamodelica::List<metamodelica::Ref<DAE::Exp>>,
    mut attr: metamodelica::Ref<DAE::CallAttributes>,
    mut slots: metamodelica::List<Slot>,
    mut dim: i32,
) -> Result<metamodelica::List<metamodelica::Ref<DAE::Exp>>> {
    let mut res: metamodelica::List<metamodelica::Ref<DAE::Exp>> = metamodelica::nil();
    let mut callargs: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
    for mut cur_dim in ({
        let __s = dim;
        let __e = 1;
        (0i32..)
            .map(move |__k| __s + __k * (-1))
            .take_while(move |&__v| __v >= __e)
    }) {
        callargs = vectorizeCallScalar3(exps, slots.clone(), cur_dim)?;
        res = metamodelica::cons(
            metamodelica::Ref::new(DAE::Exp::CALL {
                path: r#fn.clone(),
                expLst: callargs,
                attr: attr.clone(),
            }),
            res,
        );
    }
    Ok(res)
}

fn vectorizeCallScalar3(
    mut inExpl: &metamodelica::List<metamodelica::Ref<DAE::Exp>>,
    mut inSlots: metamodelica::List<Slot>,
    mut inIndex: i32,
) -> Result<metamodelica::List<metamodelica::Ref<DAE::Exp>>> {
    let mut outExpl: metamodelica::List<metamodelica::Ref<DAE::Exp>> = metamodelica::nil();
    let mut rest_slots: metamodelica::List<Slot> = inSlots;
    let mut dims: metamodelica::List<metamodelica::Ref<DAE::Dimension>>;
    for mut e in &**inExpl {
        let mut e = e.clone();
        let (__pa0, __pa1) = ::match_deref::match_deref! { match &(rest_slots) {
            Deref @ metamodelica::ListNode::Cons { head: Slot { dims: __pa0, .. }, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
            _ => return Err("pattern mismatch"),
        } };
        dims = metamodelica::Own::own(__pa0);
        rest_slots = metamodelica::Own::own(__pa1);
        if !((dims).is_empty()) {
            e = Expression::makeASUB(e, list![metamodelica::Ref::new(DAE::Exp::ICONST { integer: inIndex })])?;
            (e, _) = ExpressionSimplify::simplify1(e)?;
        }
        outExpl = metamodelica::cons(e, outExpl);
    }
    outExpl = outExpl.reverse();
    Ok(outExpl)
}

fn deoverloadFuncname(
    mut inPath: metamodelica::Ref<Absyn::Path>,
    mut inType: metamodelica::Ref<DAE::Type>,
    mut inEnv: &FCore::Graph,
) -> (metamodelica::Ref<Absyn::Path>, metamodelica::Ref<DAE::Type>) {
    let mut outPath: metamodelica::Ref<Absyn::Path>;
    let mut outType: metamodelica::Ref<DAE::Type>;
    (outPath, outType) = (::match_deref::match_deref! { match &(inType.clone()) {
        tty @ Deref @ DAE::Type::T_FUNCTION { functionAttributes: DAE::FunctionAttributes { isBuiltin: DAE::FunctionBuiltin::FUNCTION_BUILTIN { name: Some(name), .. }, .. }, .. } => {
            let mut tty = (*tty).clone();
            assign_variant_field!(tty => DAE::Type::T_FUNCTION; path = metamodelica::Ref::new(Absyn::Path::IDENT { name: name.clone() }));
            (var_field!((*tty).path, DAE::Type::T_FUNCTION).clone(), tty.clone())
        },
        Deref @ DAE::Type::T_FUNCTION { path: r#fn, .. } => {
            (r#fn.clone(), inType)
        },
        _ => {
            (inPath, inType)
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    (outPath, outType)
}

fn elabTypes(
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
    mut inPosArgs: &metamodelica::List<metamodelica::Ref<Absyn::Exp>>,
    mut inNamedArgs: &metamodelica::List<metamodelica::Ref<Absyn::NamedArg>>,
    mut typeVars: &metamodelica::List<metamodelica::Ref<Absyn::Path>>,
    mut inTypes: metamodelica::List<metamodelica::Ref<DAE::Type>>,
    mut inOnlyOneFunction: bool,
    mut inCheckTypes: bool,
    mut inImplicit: bool,
    mut inPrefix: DAE::Prefix,
    mut inInfo: SourceInfo,
) -> Result<(
    FCore::Cache,
    metamodelica::List<metamodelica::Ref<DAE::Exp>>,
    metamodelica::List<DAE::Const>,
    metamodelica::Ref<DAE::Type>,
    metamodelica::Ref<DAE::Type>,
    metamodelica::List<metamodelica::Ref<DAE::Dimension>>,
    metamodelica::List<Slot>,
)> {
    let mut outCache: FCore::Cache = inCache.clone();
    let mut outArgs: metamodelica::List<metamodelica::Ref<DAE::Exp>> = metamodelica::nil();
    let mut outConsts: metamodelica::List<DAE::Const> = metamodelica::nil();
    let mut outResultType: metamodelica::Ref<DAE::Type> = DAE::T_UNKNOWN_DEFAULT().clone();
    let mut outFunctionType: metamodelica::Ref<DAE::Type> = DAE::T_UNKNOWN_DEFAULT().clone();
    let mut outDimensions: metamodelica::List<metamodelica::Ref<DAE::Dimension>> = metamodelica::nil();
    let mut outSlots: metamodelica::List<Slot> = metamodelica::nil();
    let mut params: metamodelica::List<metamodelica::Ref<DAE::FuncArg>>;
    let mut res_ty: metamodelica::Ref<DAE::Type>;
    let mut func_ty: metamodelica::Ref<DAE::Type>;
    let mut func_attr: DAE::FunctionAttributes;
    let mut slots: metamodelica::List<Slot>;
    let mut pb: metamodelica::List<(ArcStr, metamodelica::List<metamodelica::Ref<DAE::Type>>)>;
    let mut path: metamodelica::Ref<Absyn::Path>;
    let mut success: bool = false;
    let mut rest_tys: metamodelica::List<metamodelica::Ref<DAE::Type>> = inTypes;
    let mut tys: metamodelica::List<metamodelica::Ref<DAE::Type>>;
    let mut arg: metamodelica::Ref<DAE::Exp>;
    let mut numArgs: i32;
    let mut funcarg: metamodelica::Ref<DAE::FuncArg>;
    let debug: bool = false;
    if ((rest_tys).len() as i32) > 1 {
        numArgs = ((inPosArgs).len() as i32) + ((inNamedArgs).len() as i32);
        tys = ({
            let mut __acc: metamodelica::List<metamodelica::Ref<DAE::Type>> = metamodelica::nil();
            for mut ty in (rest_tys.clone()).into_iter().cloned() {
                if !(match &*ty.clone() {
                    DAE::Type::T_FUNCTION {
                        funcArg: __ty_funcArg, ..
                    } => {
                        numArgs <= ((__ty_funcArg).len() as i32)
                            && numArgs
                                >= ({
                                    let mut __acc: i32 = 0;
                                    for mut argument in (__ty_funcArg.clone()).into_iter().cloned() {
                                        let __x = if ((argument.defaultBinding).is_none()) { 1 } else { 0 };
                                        __acc += __x;
                                    }
                                    __acc
                                })
                    }
                    _ => return Err("match: no arm matched"),
                }) {
                    continue;
                }
                let __x = ty.clone();
                __acc = cons(__x, __acc);
            }
            __acc.reverse()
        });
        if !((tys).is_empty()) {
            rest_tys = tys;
        }
    }
    while !(success) {
        let (__pa0, __pa1) = ::match_deref::match_deref! { match &(rest_tys) {
            Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
            _ => return Err("pattern mismatch"),
        } };
        func_ty = metamodelica::Own::own(__pa0);
        rest_tys = metamodelica::Own::own(__pa1);
        let (__pa2, __pa3, __pa4, __pa5) = ::match_deref::match_deref! { match &(func_ty.clone()) {
            Deref @ DAE::Type::T_FUNCTION { funcArg: __pa2, funcResultType: __pa3, functionAttributes: __pa4, path: __pa5 } => (__pa2.clone(), __pa3.clone(), __pa4.clone(), __pa5.clone()),
            _ => return Err("pattern mismatch"),
        } };
        params = metamodelica::Own::own(__pa2);
        res_ty = metamodelica::Own::own(__pa3);
        func_attr = metamodelica::Own::own(__pa4);
        path = metamodelica::Own::own(__pa5);
        if debug {
            metamodelica::print({
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("elabTypes, try: "));
                __mm_s.push_str(&*TypesDump::unparseType(func_ty.clone())?);
                __mm_s.push_str(&*literal!("\n"));
                ArcStr::from(__mm_s)
            });
        }
        if '__try6: {
            slots = unwrap_break_err!(makeEmptySlots(&params), '__try6);
            (outCache, outArgs, outSlots, outConsts, pb) = unwrap_break_err!(elabInputArgs(inCache.clone(), inEnv.clone(), inPosArgs, inNamedArgs, slots.clone(), inOnlyOneFunction, inCheckTypes, inImplicit, inPrefix.clone(), inInfo.clone(), &func_ty, path.clone(), false), '__try6);
            (outCache, pb) = unwrap_break_err!(addPolymorphicTypeVars(outCache.clone(), &inEnv, typeVars, &func_ty, pb.clone(), path.clone(), inInfo.clone()), '__try6);
            pb = unwrap_break_err!(Types::solvePolymorphicBindings(pb.clone(), &inInfo, path.clone()), '__try6);
            res_ty = unwrap_break_err!(Types::fixPolymorphicRestype(res_ty.clone(), &pb, &inInfo), '__try6);
            (outArgs, outSlots, params, res_ty) = (match func_attr.isBuiltin.clone() {
        DAE::FunctionBuiltin::FUNCTION_BUILTIN { unboxArgs: true, .. } => (unwrap_break_err!(List::map(outArgs.clone(), &move |__a0: metamodelica::Ref<DAE::Exp>| -> metamodelica::Result<_> { ::std::result::Result::Ok(Expression::unboxExp(&__a0)) }), '__try6), ({
        let mut __acc: metamodelica::List<Slot> = metamodelica::nil();
        for mut slot in (outSlots.clone()).into_iter().cloned() {
            let __x = (::match_deref::match_deref! { match &(slot.clone()) {
        Slot { arg: Some(__esc_arg), .. } => {
            arg = (*__esc_arg).clone();
            slot.arg = Some(Expression::unboxExp(metamodelica::AsArg::as_arg(&arg)));
            slot.clone()
        },
        _ => slot.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }), ({
        let mut __acc: metamodelica::List<metamodelica::Ref<DAE::FuncArg>> = metamodelica::nil();
        for mut p in (params.clone()).into_iter().cloned() {
            let __x = (::match_deref::match_deref! { match &(p.clone()) {
        __esc_funcarg => {
            funcarg = (*__esc_funcarg).clone();
            assign_field!(funcarg.ty = unwrap_break_err!(Types::unboxedType(unwrap_break_err!(Types::fixPolymorphicRestype(p.ty.clone(), &pb, &inInfo), '__try6)), '__try6));
            funcarg.clone()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    }), unwrap_break_err!(Types::unboxedType(res_ty.clone()), '__try6)),
        _ => (outArgs.clone(), outSlots.clone(), params.clone(), res_ty.clone()),
    });
            (params, res_ty) = unwrap_break_err!(applyArgTypesToFuncType(outSlots.clone(), params.clone(), res_ty.clone(), inEnv.clone(), inCheckTypes, &inInfo), '__try6);
            outDimensions = unwrap_break_err!(slotsVectorizable(&outSlots, &inInfo), '__try6);
            outResultType = res_ty.clone();
            outFunctionType = metamodelica::Ref::new(DAE::Type::T_FUNCTION { funcArg: params.clone(), funcResultType: outResultType.clone(), functionAttributes: func_attr.clone(), path: path.clone() });
            outFunctionType = unwrap_break_err!(Types::fixPolymorphicRestype(outFunctionType.clone(), &pb, &inInfo), '__try6);
            outFunctionType = unwrap_break_err!(createActualFunctype(outFunctionType.clone(), outSlots.clone(), inCheckTypes), '__try6);
            success = true;
            if debug {
                metamodelica::print({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("elabTypes success for ")); __mm_s.push_str(&*unwrap_break_err!(TypesDump::unparseType(func_ty.clone()), '__try6)); __mm_s.push_str(&*literal!(": ")); __mm_s.push_str(&*unwrap_break_err!(TypesDump::unparseType(outFunctionType.clone()), '__try6)); __mm_s.push_str(&*literal!("=>")); __mm_s.push_str(&*unwrap_break_err!(TypesDump::unparseType(outResultType.clone()), '__try6)); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) });
            }
            Ok::<(), &'static str>(())
        }.is_err() {
        }
    }
    Ok((
        outCache,
        outArgs,
        outConsts,
        outResultType,
        outFunctionType,
        outDimensions,
        outSlots,
    ))
}

fn addPolymorphicTypeVars(
    mut cache: FCore::Cache,
    mut env: &FCore::Graph,
    mut typeVars: &metamodelica::List<metamodelica::Ref<Absyn::Path>>,
    mut funcTy: &metamodelica::Ref<DAE::Type>,
    mut pb: metamodelica::List<(ArcStr, metamodelica::List<metamodelica::Ref<DAE::Type>>)>,
    mut fnPath: metamodelica::Ref<Absyn::Path>,
    mut info: SourceInfo,
) -> Result<(
    FCore::Cache,
    metamodelica::List<(ArcStr, metamodelica::List<metamodelica::Ref<DAE::Type>>)>,
)> {
    let mut cache: FCore::Cache = cache;
    let mut pb: metamodelica::List<(ArcStr, metamodelica::List<metamodelica::Ref<DAE::Type>>)> = pb;
    let mut ty: metamodelica::Ref<DAE::Type>;
    let mut scope: FCore::Graph;
    let mut e: metamodelica::Ref<SCode::Element>;
    let mut poly_types: metamodelica::List<ArcStr>;
    let mut ty_name: ArcStr;
    if (typeVars).is_empty() {
        return Ok((cache, pb));
    }
    (cache, e, _) = Lookup::lookupClass(&cache, env, &fnPath, None)?;
    poly_types = ({
        let mut __acc: metamodelica::List<ArcStr> = metamodelica::nil();
        for mut c in (SCodeUtil::getClassElements(&e)).into_iter().cloned() {
            if !(SCodeUtil::isPolymorphicTypeVar(&(c.clone()))) {
                continue;
            }
            let __x = {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("$"));
                __mm_s.push_str(&*SCodeUtil::getElementName(&(c.clone()))?);
                ArcStr::from(__mm_s)
            };
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    if ((typeVars).len() as i32) > ((poly_types).len() as i32) {
        Error::addSourceMessage(
            &(Error::TOO_MANY_TYPE_VARS_IN_CALL.clone()),
            list![AbsynUtil::pathString(fnPath, literal!("."), true, false)?],
            &info,
        )?;
        return Err("fail");
    }
    for mut tv in &**typeVars {
        let (__pa0, __pa1) = ::match_deref::match_deref! { match &(poly_types) {
            Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
            _ => return Err("pattern mismatch"),
        } };
        ty_name = metamodelica::Own::own(__pa0);
        poly_types = metamodelica::Own::own(__pa1);
        (cache, e, scope) = Lookup::lookupClass(&cache, env, metamodelica::AsArg::as_arg(&tv), Some(info.clone()))?;
        (cache, _, ty) = Inst::instClassType(cache, scope, e)?;
        pb = Types::addPolymorphicBinding(ty_name, ty, &pb)?;
    }
    Ok((cache, pb))
}

fn applyArgTypesToFuncType(
    mut inSlots: metamodelica::List<Slot>,
    mut inParameters: metamodelica::List<metamodelica::Ref<DAE::FuncArg>>,
    mut inResultType: metamodelica::Ref<DAE::Type>,
    mut inEnv: FCore::Graph,
    mut checkTypes: bool,
    mut inInfo: &SourceInfo,
) -> Result<(
    metamodelica::List<metamodelica::Ref<DAE::FuncArg>>,
    metamodelica::Ref<DAE::Type>,
)> {
    let mut outParameters: metamodelica::List<metamodelica::Ref<DAE::FuncArg>>;
    let mut outResultType: metamodelica::Ref<DAE::Type>;
    let mut tys: metamodelica::List<metamodelica::Ref<DAE::Type>>;
    let mut dims: metamodelica::List<metamodelica::Ref<DAE::Dimension>>;
    let mut used_args: metamodelica::List<ArcStr>;
    let mut used_slots: metamodelica::List<Slot>;
    let mut cache: FCore::Cache;
    let mut env: FCore::Graph;
    let mut vars: metamodelica::List<metamodelica::Ref<DAE::Var>>;
    let mut dummy_var: metamodelica::Ref<SCode::Element>;
    if !(checkTypes) || (inParameters).is_empty() {
        outParameters = inParameters;
        outResultType = inResultType;
        return Ok((outParameters, outResultType));
    }
    tys = ({
        let mut __acc: metamodelica::List<metamodelica::Ref<DAE::Type>> = metamodelica::nil();
        for mut param in (inParameters.clone()).into_iter().cloned() {
            let __x = Types::funcArgType(&(param.clone()));
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    dims = getAllOutputDimensions(&inResultType)?;
    dims = listAppend(
        List::mapFlat(
            &tys,
            &move |__a0: metamodelica::Ref<DAE::Type>| -> metamodelica::Result<_> {
                ::std::result::Result::Ok(TypesDump::getDimensions(&__a0))
            },
        )?,
        dims,
    );
    used_args = extractNamesFromDims(&dims, metamodelica::nil())?;
    used_slots = ({
        let mut __acc: metamodelica::List<Slot> = metamodelica::nil();
        for mut s in (inSlots.clone()).into_iter().cloned() {
            if !(isSlotUsed(s.clone(), &used_args)?) {
                continue;
            }
            let __x = s.clone();
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    cache = FCore::noCache();
    vars = ({
        let mut __acc: metamodelica::List<metamodelica::Ref<DAE::Var>> = metamodelica::nil();
        for mut s in (used_slots).into_iter().cloned() {
            let __x = makeVarFromSlot(&(s.clone()), inEnv.clone(), cache.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    dummy_var = metamodelica::Ref::new(SCode::Element::COMPONENT {
        name: literal!("dummy"),
        prefixes: SCode::defaultPrefixes.clone(),
        attributes: SCode::defaultVarAttr.clone(),
        typeSpec: metamodelica::Ref::new(Absyn::TypeSpec::TPATH {
            path: metamodelica::Ref::new(Absyn::Path::IDENT { name: literal!("") }),
            arrayDim: None,
        }),
        modifications: openmodelica_frontend_types::SCode::Mod::interned_NOMOD(),
        comment: SCode::noComment.clone(),
        condition: None,
        info: Absyn::dummyInfo.clone(),
    });
    env = FGraph::openScope(
        inEnv,
        openmodelica_frontend_types::SCode::Encapsulated::NOT_ENCAPSULATED,
        arcstr::literal!(FCore::forScopeName),
        None,
    )?;
    env = makeDummyFuncEnv(env, &vars, dummy_var)?;
    outParameters = ({
        let mut __acc: metamodelica::List<metamodelica::Ref<DAE::FuncArg>> = metamodelica::nil();
        let __thr_src0 = inSlots;
        let mut __thr_it0 = (&__thr_src0).into_iter();
        let __thr_src1 = inParameters;
        let mut __thr_it1 = (&__thr_src1).into_iter();
        loop {
            match (__thr_it0.next(), __thr_it1.next()) {
                (Some(s), Some(p)) => {
                    let __x = evaluateFuncParamDimAndMatchTypes(&(s.clone()), p.clone(), &env, &cache, inInfo)?;
                    __acc = cons(__x, __acc);
                }
                (None, None) => break,
                _ => return Err("threaded for: ranges of unequal length"),
            }
        }
        __acc.reverse()
    });
    outResultType = evaluateFuncArgTypeDims(inResultType, &env, &cache);
    Ok((outParameters, outResultType))
}

fn getAllOutputDimensions(
    mut inOutputType: &metamodelica::Ref<DAE::Type>,
) -> Result<metamodelica::List<metamodelica::Ref<DAE::Dimension>>> {
    let mut outDimensions: metamodelica::List<metamodelica::Ref<DAE::Dimension>>;
    outDimensions = (match &**inOutputType {
        DAE::Type::T_TUPLE { types: tys, .. } => List::mapFlat(
            tys,
            &move |__a0: metamodelica::Ref<DAE::Type>| -> metamodelica::Result<_> {
                ::std::result::Result::Ok(TypesDump::getDimensions(&__a0))
            },
        )?,
        _ => TypesDump::getDimensions(inOutputType),
    });
    Ok(outDimensions)
}

fn extractNamesFromDims<'__b>(
    mut inDimensions: &'__b metamodelica::List<metamodelica::Ref<DAE::Dimension>>,
    mut inAccumNames: metamodelica::List<ArcStr>,
) -> Result<metamodelica::List<ArcStr>> {
    '__tco: loop {
        ::match_deref::match_deref! { match inDimensions {
            Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Dimension::DIM_EXP { exp }, tail: rest_dims } => {
                let mut crefs: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
                let mut names: metamodelica::List<ArcStr>;
                crefs = Expression::extractCrefsFromExp(exp.clone())?;
                names = List::fold(&crefs, &move |__a0: metamodelica::Ref<DAE::ComponentRef>, __a1: metamodelica::List<ArcStr>| extractNamesFromDims2(&__a0, __a1), inAccumNames)?;
                { (inDimensions, inAccumNames) = (rest_dims, names); continue '__tco; }
            },
            Deref @ metamodelica::ListNode::Cons { head: _, tail: rest_dims } => {
                { (inDimensions, inAccumNames) = (rest_dims, inAccumNames); continue '__tco; }
            },
            Deref @ metamodelica::ListNode::Nil => {
                return Ok(inAccumNames)
            },
            _ => return Err("match: no arm matched"),
        } }
    }
}

fn extractNamesFromDims2(
    mut inCref: &metamodelica::Ref<DAE::ComponentRef>,
    mut inAccumNames: metamodelica::List<ArcStr>,
) -> Result<metamodelica::List<ArcStr>> {
    let mut outNames: metamodelica::List<ArcStr>;
    outNames = (match &**inCref {
        DAE::ComponentRef::CREF_IDENT { ident: name, .. } => {
            outNames = if (List::isMemberOnTrue(name.clone(), &inAccumNames, &fnptr!(stringEq, ArcStr, ArcStr))?) {
                inAccumNames
            } else {
                metamodelica::cons(name.clone(), inAccumNames)
            };
            outNames
        }
        _ => inAccumNames,
    });
    Ok(outNames)
}

fn isSlotUsed(mut inSlot: Slot, mut inUsedNames: &metamodelica::List<ArcStr>) -> Result<bool> {
    let mut outIsUsed: bool;
    let mut slot_name: ArcStr;
    let Slot { defaultArg: __t1, .. } = inSlot;
    let __arc2 = __t1.clone();
    let DAE::FUNCARG { name: __pa0, .. } = &*__arc2;
    slot_name = metamodelica::Own::own(__pa0);
    outIsUsed = List::isMemberOnTrue(slot_name, inUsedNames, &fnptr!(stringEq, ArcStr, ArcStr))?;
    Ok(outIsUsed)
}

fn makeVarFromSlot(
    mut inSlot: &Slot,
    mut inEnv: FCore::Graph,
    mut inCache: FCore::Cache,
) -> Result<metamodelica::Ref<DAE::Var>> {
    let mut outVar: metamodelica::Ref<DAE::Var>;
    outVar = 'mc: {
        let __mc_input = inSlot;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Slot { defaultArg: Deref @ DAE::FuncArg { name, .. }, arg: Some(exp), .. } => {
                    let mut ty: metamodelica::Ref<DAE::Type>;
                    let mut binding: metamodelica::Ref<DAE::Binding>;
                    let false = (Expression::expHasCref(exp.clone(), ComponentReferenceBasics::makeCrefIdent(name.clone(), DAE::T_UNKNOWN_DEFAULT().clone(), metamodelica::nil()))?) else { return Err("pattern mismatch") };
                    ty = Expression::r#typeof(exp.clone())?;
                    let true = (Types::dimensionsKnown(ty.clone())) else { return Err("pattern mismatch") };
                    binding = metamodelica::Ref::new(DAE::Binding::EQBOUND { exp: exp.clone(), evaluatedExp: None, constant_: openmodelica_frontend_types::DAE::Const::C_CONST, source: openmodelica_frontend_types::DAE::BindingSource::BINDING_FROM_DEFAULT_VALUE });
                    Ok(metamodelica::Ref::new(DAE::Var { name: name.clone(), attributes: DAE::dummyAttrParam().clone(), ty: ty.clone(), binding: binding.clone(), bind_from_outside: false, constOfForIteratorRange: None }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Slot { defaultArg: Deref @ DAE::FuncArg { name, .. }, arg: Some(exp), .. } => {
                    let mut ty: metamodelica::Ref<DAE::Type>;
                    let mut binding: metamodelica::Ref<DAE::Binding>;
                    let mut val: metamodelica::Ref<Values::Value>;
                    let mut exp = (*exp).clone();
                    (_, val) = Ceval::ceval(inCache.clone(), inEnv.clone(), exp.clone(), false, openmodelica_ast::Absyn::Msg::NO_MSG, 0)?;
                    exp = ValuesUtil::valueExp(val.clone(), Some(exp.clone()))?;
                    ty = Expression::r#typeof(exp.clone())?;
                    binding = metamodelica::Ref::new(DAE::Binding::EQBOUND { exp: exp.clone(), evaluatedExp: Some(val.clone()), constant_: openmodelica_frontend_types::DAE::Const::C_CONST, source: openmodelica_frontend_types::DAE::BindingSource::BINDING_FROM_DEFAULT_VALUE });
                    Ok(metamodelica::Ref::new(DAE::Var { name: name.clone(), attributes: DAE::dummyAttrParam().clone(), ty: ty.clone(), binding: binding.clone(), bind_from_outside: false, constOfForIteratorRange: None }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Slot { defaultArg: Deref @ DAE::FuncArg { name, ty, .. }, .. } => {
                    Ok(metamodelica::Ref::new(DAE::Var { name: name.clone(), attributes: DAE::dummyAttrParam().clone(), ty: ty.clone(), binding: openmodelica_frontend_types::DAE::Binding::interned_UNBOUND(), bind_from_outside: false, constOfForIteratorRange: None }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(outVar)
}

fn evaluateStructuralSlots2(
    mut inCache: &FCore::Cache,
    mut inEnv: &FCore::Graph,
    mut inSlots: &metamodelica::List<Slot>,
    mut usedSlots: &metamodelica::List<ArcStr>,
    mut acc: &metamodelica::List<Slot>,
) -> (FCore::Cache, metamodelica::List<Slot>) {
    let mut cache: FCore::Cache = FCore::Cache::NO_CACHE;
    let mut slots: metamodelica::List<Slot> = metamodelica::nil();
    (cache, slots) = 'mc: {
        let __mc_input = &**inSlots;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Nil => {
                    Ok((inCache.clone(), acc.clone().reverse()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok((__v, __wb0, __wb1)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: slot, tail: rest } => {
                    let mut cache: FCore::Cache = cache.clone();
                    let mut slots: metamodelica::List<Slot> = slots.clone();
                    let false = (isSlotUsed(slot.clone(), usedSlots)?) else { return Err("pattern mismatch") };
                    (cache, slots) = evaluateStructuralSlots2(inCache, inEnv, metamodelica::AsArg::as_arg(&rest), usedSlots, &(metamodelica::cons(slot.clone(), acc.clone())));
                    Ok(((cache.clone(), slots.clone()), cache.clone(), slots.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            cache = __wb0;
            slots = __wb1;
            break 'mc __v;
        }
        if let Ok((__v, __wb0, __wb1)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: Slot { defaultArg: defaultArg @ Deref @ DAE::FuncArg { .. }, slotFilled: _, arg: Some(exp), dims, idx, evalStatus: ses }, tail: rest } => {
                    let mut slot: Slot;
                    let mut val: metamodelica::Ref<Values::Value>;
                    let mut exp = (*exp).clone();
                    let mut cache: FCore::Cache = cache.clone();
                    let mut slots: metamodelica::List<Slot> = slots.clone();
                    (cache, val) = Ceval::ceval(inCache.clone(), inEnv.clone(), exp.clone(), false, openmodelica_ast::Absyn::Msg::NO_MSG, 0)?;
                    exp = ValuesUtil::valueExp(val.clone(), Some(exp.clone()))?;
                    slot = Slot { defaultArg: defaultArg.clone(), slotFilled: true, arg: Some(exp.clone()), dims: dims.clone(), idx: idx.clone(), evalStatus: ses.clone() };
                    (cache, slots) = evaluateStructuralSlots2(&cache, inEnv, metamodelica::AsArg::as_arg(&rest), usedSlots, &(metamodelica::cons(slot.clone(), acc.clone())));
                    Ok(((cache.clone(), slots.clone()), cache.clone(), slots.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            cache = __wb0;
            slots = __wb1;
            break 'mc __v;
        }
        if let Ok((__v, __wb0, __wb1)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: slot, tail: rest } => {
                    let mut cache: FCore::Cache = cache.clone();
                    let mut slots: metamodelica::List<Slot> = slots.clone();
                    (cache, slots) = evaluateStructuralSlots2(inCache, inEnv, metamodelica::AsArg::as_arg(&rest), usedSlots, &(metamodelica::cons(slot.clone(), acc.clone())));
                    Ok(((cache.clone(), slots.clone()), cache.clone(), slots.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            cache = __wb0;
            slots = __wb1;
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    (cache, slots)
}

fn evaluateStructuralSlots(
    mut inCache: FCore::Cache,
    mut inEnv: &FCore::Graph,
    mut inSlots: metamodelica::List<Slot>,
    mut funcType: &metamodelica::Ref<DAE::Type>,
) -> Result<(FCore::Cache, metamodelica::List<Slot>)> {
    let mut cache: FCore::Cache;
    let mut slots: metamodelica::List<Slot>;
    (cache, slots) = (match &**funcType {
        DAE::Type::T_FUNCTION {
            funcArg,
            funcResultType,
            ..
        } => {
            let mut tys: metamodelica::List<metamodelica::Ref<DAE::Type>>;
            let mut dims: metamodelica::List<metamodelica::Ref<DAE::Dimension>>;
            let mut used_args: metamodelica::List<ArcStr>;
            tys = ({
                let mut __acc: metamodelica::List<metamodelica::Ref<DAE::Type>> = metamodelica::nil();
                for mut arg in (funcArg.clone()).into_iter().cloned() {
                    let __x = Types::funcArgType(&(arg.clone()));
                    __acc = cons(__x, __acc);
                }
                __acc.reverse()
            });
            dims = getAllOutputDimensions(funcResultType)?;
            dims = listAppend(
                List::mapFlat(
                    &tys,
                    &move |__a0: metamodelica::Ref<DAE::Type>| -> metamodelica::Result<_> {
                        ::std::result::Result::Ok(TypesDump::getDimensions(&__a0))
                    },
                )?,
                dims,
            );
            used_args = extractNamesFromDims(&dims, metamodelica::nil())?;
            (cache, slots) = evaluateStructuralSlots2(&inCache, inEnv, &inSlots, &used_args, &(metamodelica::nil()));
            (cache, slots)
        }
        _ => (inCache, inSlots),
    });
    Ok((cache, slots))
}

fn makeDummyFuncEnv(
    mut inEnv: FCore::Graph,
    mut inVars: &metamodelica::List<metamodelica::Ref<DAE::Var>>,
    mut inDummyVar: metamodelica::Ref<SCode::Element>,
) -> Result<FCore::Graph> {
    let mut outEnv: FCore::Graph = inEnv;
    let mut dummy_var: metamodelica::Ref<SCode::Element>;
    for mut var in &**inVars {
        dummy_var = SCodeUtil::setComponentName(
            inDummyVar.clone(),
            DAEUtil::typeVarIdent(metamodelica::AsArg::as_arg(&var)),
        )?;
        outEnv = FGraph::mkComponentNode(
            outEnv,
            var.clone(),
            dummy_var,
            openmodelica_frontend_types::DAE::Mod::interned_NOMOD(),
            openmodelica_frontend_dump::FCore::Status::VAR_TYPED,
            FGraph::empty(),
        )?;
    }
    Ok(outEnv)
}

fn evaluateFuncParamDimAndMatchTypes(
    mut inSlot: &Slot,
    mut inParam: metamodelica::Ref<DAE::FuncArg>,
    mut inEnv: &FCore::Graph,
    mut inCache: &FCore::Cache,
    mut inInfo: &SourceInfo,
) -> Result<metamodelica::Ref<DAE::FuncArg>> {
    let mut outParam: metamodelica::Ref<DAE::FuncArg>;
    outParam = (::match_deref::match_deref! { match &((inSlot.clone(), inParam.clone())) {
        (_, Deref @ DAE::FuncArg { ty: Deref @ DAE::Type::T_CODE { .. }, .. }) => {
            inParam
        },
        (Slot { arg: Some(Deref @ DAE::Exp::ARRAY { ty: sty, .. }), dims: vdims, .. }, _) => {
            let mut pty: metamodelica::Ref<DAE::Type>;
            let mut dims1: metamodelica::List<metamodelica::Ref<DAE::Dimension>>;
            let mut dims2: metamodelica::List<metamodelica::Ref<DAE::Dimension>>;
            let __arc1 = inParam.clone();
            let DAE::FUNCARG { ty: __pa0, .. } = &*__arc1;
            pty = metamodelica::Own::own(__pa0);
            pty = evaluateFuncArgTypeDims(pty, inEnv, inCache);
            dims1 = TypesDump::getDimensions(&pty);
            dims1 = listAppend(vdims.clone(), dims1);
            dims2 = TypesDump::getDimensions(metamodelica::AsArg::as_arg(&sty));
            let true = (Expression::dimsEqual(&dims1, &dims2)?) else { return Err("pattern mismatch") };
            outParam = Types::setFuncArgType(&inParam, pty);
            outParam
        },
        (Slot { arg: Some(Deref @ DAE::Exp::MATRIX { ty: sty, .. }), dims: vdims, .. }, _) => {
            let mut pty: metamodelica::Ref<DAE::Type>;
            let mut dims1: metamodelica::List<metamodelica::Ref<DAE::Dimension>>;
            let mut dims2: metamodelica::List<metamodelica::Ref<DAE::Dimension>>;
            let mut vdims = (*vdims).clone();
            let __arc1 = inParam.clone();
            let DAE::FUNCARG { ty: __pa0, .. } = &*__arc1;
            pty = metamodelica::Own::own(__pa0);
            pty = evaluateFuncArgTypeDims(pty, inEnv, inCache);
            dims1 = TypesDump::getDimensions(&pty);
            vdims = listAppend(dims1, vdims.clone());
            dims2 = TypesDump::getDimensions(metamodelica::AsArg::as_arg(&sty));
            let true = (Expression::dimsEqual(metamodelica::AsArg::as_arg(&vdims), &dims2)?) else { return Err("pattern mismatch") };
            outParam = Types::setFuncArgType(&inParam, pty);
            outParam
        },
        _ => {
            let mut pty: metamodelica::Ref<DAE::Type>;
            let __arc1 = inParam.clone();
            let DAE::FUNCARG { ty: __pa0, .. } = &*__arc1;
            pty = metamodelica::Own::own(__pa0);
            pty = evaluateFuncArgTypeDims(pty, inEnv, inCache);
            outParam = Types::setFuncArgType(&inParam, pty);
            outParam
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outParam)
}

fn evaluateFuncArgTypeDims(
    mut inType: metamodelica::Ref<DAE::Type>,
    mut inEnv: &FCore::Graph,
    mut inCache: &FCore::Cache,
) -> metamodelica::Ref<DAE::Type> {
    let mut outType: metamodelica::Ref<DAE::Type>;
    outType = 'mc: {
        let __mc_input = inType.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Type::T_ARRAY { ty, dims: Deref @ metamodelica::ListNode::Cons { head: dim, tail: Deref @ metamodelica::ListNode::Nil } } => {
                    let mut n: i32;
                    let mut ty = (*ty).clone();
                    let __pa0 = ::match_deref::match_deref! { match &(Ceval::cevalDimension(inCache.clone(), inEnv.clone(), metamodelica::AsArg::as_arg(&dim), false, openmodelica_ast::Absyn::Msg::NO_MSG, 0)?) {
                        (_, Deref @ Values::Value::INTEGER { integer: __pa0 }) => __pa0.clone(),
                        _ => return Err("pattern mismatch"),
                    } };
                    n = metamodelica::Own::own(__pa0);
                    ty = evaluateFuncArgTypeDims(ty.clone(), inEnv, inCache);
                    Ok(metamodelica::Ref::new(DAE::Type::T_ARRAY { ty: ty.clone(), dims: list![metamodelica::Ref::new(DAE::Dimension::DIM_INTEGER { integer: n })] }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Type::T_ARRAY { ty, dims: Deref @ metamodelica::ListNode::Cons { head: dim, tail: Deref @ metamodelica::ListNode::Nil } } => {
                    let mut ty = (*ty).clone();
                    ty = evaluateFuncArgTypeDims(ty.clone(), inEnv, inCache);
                    Ok(metamodelica::Ref::new(DAE::Type::T_ARRAY { ty: ty.clone(), dims: list![dim.clone()] }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                ty @ Deref @ DAE::Type::T_TUPLE { .. } => {
                    let mut ty = (*ty).clone();
                    assign_variant_field!(ty => DAE::Type::T_TUPLE; types = List::map2(var_field!((*ty).types, DAE::Type::T_TUPLE).clone(), &move |__a0: metamodelica::Ref<DAE::Type>, __a1: FCore::Graph, __a2: FCore::Cache| -> metamodelica::Result<_> { ::std::result::Result::Ok(evaluateFuncArgTypeDims(__a0, &__a1, &__a2)) }, inEnv.clone(), inCache.clone())?);
                    Ok(ty.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(inType.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    outType
}

fn createActualFunctype(
    mut tp: metamodelica::Ref<DAE::Type>,
    mut slots: metamodelica::List<Slot>,
    mut checkTypes: bool,
) -> Result<metamodelica::Ref<DAE::Type>> {
    let mut outTp: metamodelica::Ref<DAE::Type> = tp.clone();
    outTp = (::match_deref::match_deref! { match &((outTp.clone(), checkTypes)) {
        (_, true) => tp,
        (Deref @ DAE::Type::T_FUNCTION { .. }, _) => {
            assign_variant_field!(outTp => DAE::Type::T_FUNCTION; funcArg = funcArgsFromSlots(slots));
            outTp
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok(outTp)
}

fn slotsVectorizable(
    mut inSlots: &metamodelica::List<Slot>,
    mut info: &SourceInfo,
) -> Result<metamodelica::List<metamodelica::Ref<DAE::Dimension>>> {
    let mut outDims: metamodelica::List<metamodelica::Ref<DAE::Dimension>>;
    outDims = 'mc: {
        let __mc_input = &**inSlots;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Nil => {
                    Ok(metamodelica::nil())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: Slot { defaultArg: Deref @ DAE::FuncArg { name, .. }, arg: Some(exp), dims: ad @ Deref @ metamodelica::ListNode::Cons { head: _, tail: _ }, .. }, tail: rest } => {
                    sameSlotsVectorizable(metamodelica::AsArg::as_arg(&rest), metamodelica::AsArg::as_arg(&ad), metamodelica::AsArg::as_arg(&name), metamodelica::AsArg::as_arg(&exp), info)?;
                    Ok(ad.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: Slot { dims: Deref @ metamodelica::ListNode::Nil, .. }, tail: rest } => {
                    Ok(slotsVectorizable(metamodelica::AsArg::as_arg(&rest), info)?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { return Err("pattern mismatch") };
                    Debug::trace(literal!("-slots_vectorizable failed\n"))?;
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(outDims)
}

fn sameSlotsVectorizable(
    mut inSlots: &metamodelica::List<Slot>,
    mut inDims: &metamodelica::List<metamodelica::Ref<DAE::Dimension>>,
    mut name: &ArcStr,
    mut exp: &metamodelica::Ref<DAE::Exp>,
    mut info: &SourceInfo,
) -> Result<()> {
    let () = (::match_deref::match_deref! { match inSlots {
        Deref @ metamodelica::ListNode::Cons { head: Slot { defaultArg: Deref @ DAE::FuncArg { name: name2, .. }, arg: Some(exp2), dims: slot_ad @ Deref @ metamodelica::ListNode::Cons { head: _, tail: _ }, .. }, tail: rest } => {
            sameArraydimLst(inDims, name, exp, metamodelica::AsArg::as_arg(&slot_ad), metamodelica::AsArg::as_arg(&name2), metamodelica::AsArg::as_arg(&exp2), info)?;
            sameSlotsVectorizable(rest, inDims, name, exp, info)?;
            ()
        },
        Deref @ metamodelica::ListNode::Cons { head: Slot { dims: Deref @ metamodelica::ListNode::Nil, .. }, tail: rest } => {
            sameSlotsVectorizable(rest, inDims, name, exp, info)?;
            ()
        },
        Deref @ metamodelica::ListNode::Nil => {
            ()
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok(())
}

fn sameArraydimLst(
    mut inDims1: &metamodelica::List<metamodelica::Ref<DAE::Dimension>>,
    mut name1: &ArcStr,
    mut exp1: &metamodelica::Ref<DAE::Exp>,
    mut inDims2: &metamodelica::List<metamodelica::Ref<DAE::Dimension>>,
    mut name2: &ArcStr,
    mut exp2: &metamodelica::Ref<DAE::Exp>,
    mut info: &SourceInfo,
) -> Result<()> {
    let () = 'mc: {
        let __mc_input = (&**inDims2, &**inDims2);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Dimension::DIM_INTEGER { integer: i1 }, tail: ads1 }, Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Dimension::DIM_INTEGER { integer: i2 }, tail: ads2 }) => {
                    let true = (intEq(i1.clone(), i2.clone())) else { return Err("pattern mismatch") };
                    sameArraydimLst(metamodelica::AsArg::as_arg(&ads1), name1, exp1, metamodelica::AsArg::as_arg(&ads2), name2, exp2, info)?;
                    Ok(())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Dimension::DIM_UNKNOWN { .. }, tail: ads1 }, Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Dimension::DIM_UNKNOWN { .. }, tail: ads2 }) => {
                    sameArraydimLst(metamodelica::AsArg::as_arg(&ads1), name1, exp1, metamodelica::AsArg::as_arg(&ads2), name2, exp2, info)?;
                    Ok(())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Dimension::DIM_EXP { exp: e1 }, tail: ads1 }, Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Dimension::DIM_EXP { exp: e2 }, tail: ads2 }) => {
                    let true = (ExpressionBasics::expEqual(metamodelica::AsArg::as_arg(&e1), e2.clone())?) else { return Err("pattern mismatch") };
                    sameArraydimLst(metamodelica::AsArg::as_arg(&ads1), name1, exp1, metamodelica::AsArg::as_arg(&ads2), name2, exp2, info)?;
                    Ok(())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Nil, Deref @ metamodelica::ListNode::Nil) => {
                    Ok(())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: ad1, tail: _ }, Deref @ metamodelica::ListNode::Cons { head: ad2, tail: _ }) => {
                    let mut str1: ArcStr;
                    let mut str2: ArcStr;
                    let mut str3: ArcStr;
                    let mut str4: ArcStr;
                    str1 = ExpressionBasics::printExpStr(exp1.clone())?;
                    str2 = ExpressionBasics::printExpStr(exp2.clone())?;
                    str3 = ExpressionBasics::dimensionString(metamodelica::AsArg::as_arg(&ad1))?;
                    str4 = ExpressionBasics::dimensionString(metamodelica::AsArg::as_arg(&ad2))?;
                    Error::addSourceMessage(&(Error::VECTORIZE_CALL_DIM_MISMATCH.clone()), list![name1.clone(), str1.clone(), name2.clone(), str2.clone(), str3.clone(), str4.clone()], info)?;
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(())
}

fn getProperties(
    mut inType: metamodelica::Ref<DAE::Type>,
    mut inTupleConst: metamodelica::Ref<DAE::TupleConst>,
) -> Result<DAE::Properties> {
    let mut outProperties: DAE::Properties;
    outProperties = (::match_deref::match_deref! { match &((inType, inTupleConst)) {
        (tt @ Deref @ DAE::Type::T_TUPLE { .. }, r#const) => {
            DAE::Properties::PROP_TUPLE { type_: tt.clone(), tupleConst: r#const.clone() }
        },
        (t, Deref @ DAE::TupleConst::TUPLE_CONST { tupleConstLst: Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::TupleConst::SINGLE_CONST { r#const: b }, tail: Deref @ metamodelica::ListNode::Nil } }) => {
            DAE::Properties::PROP { type_: t.clone(), constFlag: b.clone() }
        },
        (t, Deref @ DAE::TupleConst::TUPLE_CONST { tupleConstLst: Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::TupleConst::SINGLE_CONST { r#const: b }, tail: Deref @ metamodelica::ListNode::Nil } }) => {
            DAE::Properties::PROP { type_: t.clone(), constFlag: b.clone() }
        },
        (t, Deref @ DAE::TupleConst::SINGLE_CONST { r#const: b }) => {
            DAE::Properties::PROP { type_: t.clone(), constFlag: b.clone() }
        },
        (ty, r#const) => {
            let mut tystr: ArcStr;
            let mut conststr: ArcStr;
            let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { return Err("pattern mismatch") };
            Debug::trace(literal!("- Static.getProperties failed: "))?;
            tystr = TypesDump::unparseType(ty.clone())?;
            conststr = TypesDump::printTupleConstStr(metamodelica::AsArg::as_arg(&r#const))?;
            Debug::trace(tystr)?;
            Debug::trace(literal!(", "))?;
            Debug::traceln(conststr)?;
            return Err("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outProperties)
}

fn elabConsts(
    mut inType: metamodelica::Ref<DAE::Type>,
    mut inConst: DAE::Const,
) -> Result<metamodelica::Ref<DAE::TupleConst>> {
    let mut outTupleConst: metamodelica::Ref<DAE::TupleConst>;
    outTupleConst = (::match_deref::match_deref! { match &(inType) {
        Deref @ DAE::Type::T_TUPLE { types: tys, .. } => {
            let mut c = inConst;
            let mut consts: metamodelica::List<metamodelica::Ref<DAE::TupleConst>>;
            consts = checkConsts(tys.clone(), c)?;
            metamodelica::Ref::new(DAE::TupleConst::TUPLE_CONST { tupleConstLst: consts })
        },
        ty => {
            let mut c = inConst;
            let mut consts: metamodelica::List<metamodelica::Ref<DAE::TupleConst>>;
            consts = checkConsts(list![ty.clone()], c)?;
            metamodelica::Ref::new(DAE::TupleConst::TUPLE_CONST { tupleConstLst: consts })
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outTupleConst)
}

fn checkConsts(
    mut inTypes: metamodelica::List<metamodelica::Ref<DAE::Type>>,
    mut inConst: DAE::Const,
) -> Result<metamodelica::List<metamodelica::Ref<DAE::TupleConst>>> {
    let mut outTupleConsts: metamodelica::List<metamodelica::Ref<DAE::TupleConst>>;
    outTupleConsts = ({
        let mut __acc: metamodelica::List<metamodelica::Ref<DAE::TupleConst>> = metamodelica::nil();
        for mut ty in (inTypes).into_iter().cloned() {
            let __x = checkConst(&(ty.clone()), inConst)?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    Ok(outTupleConsts)
}

fn checkConst(
    mut inType: &metamodelica::Ref<DAE::Type>,
    mut c: DAE::Const,
) -> Result<metamodelica::Ref<DAE::TupleConst>> {
    let mut outTupleConst: metamodelica::Ref<DAE::TupleConst>;
    outTupleConst = (match &**inType {
        DAE::Type::T_TUPLE { .. } => {
            Error::addInternalError(
                literal!("No support for tuples built by tuples"),
                metamodelica::sourceInfo!("FrontEnd/Static.mo"),
            )?;
            return Err("fail");
        }
        _ => metamodelica::Ref::new(DAE::TupleConst::SINGLE_CONST { r#const: c }),
    });
    Ok(outTupleConst)
}

fn splitProps(
    mut inProperties: metamodelica::List<DAE::Properties>,
) -> (
    metamodelica::List<metamodelica::Ref<DAE::Type>>,
    metamodelica::List<metamodelica::Ref<DAE::TupleConst>>,
) {
    let mut outTypes: metamodelica::List<metamodelica::Ref<DAE::Type>> = metamodelica::nil();
    let mut outConsts: metamodelica::List<metamodelica::Ref<DAE::TupleConst>> = metamodelica::nil();
    let mut ty: metamodelica::Ref<DAE::Type> = metamodelica::Ref::new(DAE::Type::T_NORETCALL);
    let mut c: DAE::Const;
    let mut tc: metamodelica::Ref<DAE::TupleConst>;
    for mut prop in &*inProperties.reverse() {
        tc = (match prop.clone() {
            DAE::Properties::PROP {
                type_: ref __esc_ty,
                constFlag: mut __esc_c,
            } => {
                ty = __esc_ty.clone();
                c = __esc_c.clone();
                metamodelica::Ref::new(DAE::TupleConst::SINGLE_CONST { r#const: c })
            }
            DAE::Properties::PROP_TUPLE {
                type_: ref __esc_ty,
                tupleConst: ref __esc_tc,
            } => {
                ty = __esc_ty.clone();
                tc = __esc_tc.clone();
                tc.clone()
            }
        });
        outTypes = metamodelica::cons(ty.clone(), outTypes);
        outConsts = metamodelica::cons(tc.clone(), outConsts);
    }
    (outTypes, outConsts)
}

fn getTypes(
    mut farg: metamodelica::List<metamodelica::Ref<DAE::FuncArg>>,
) -> metamodelica::List<metamodelica::Ref<DAE::Type>> {
    let mut outTypes: metamodelica::List<metamodelica::Ref<DAE::Type>>;
    outTypes = ({
        let mut __acc: metamodelica::List<metamodelica::Ref<DAE::Type>> = metamodelica::nil();
        for mut arg in (farg).into_iter().cloned() {
            let __x = Types::funcArgType(&(arg.clone()));
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    outTypes
}

fn elabInputArgs(
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
    mut inPosArgs: &metamodelica::List<metamodelica::Ref<Absyn::Exp>>,
    mut inNamedArgs: &metamodelica::List<metamodelica::Ref<Absyn::NamedArg>>,
    mut inSlots: metamodelica::List<Slot>,
    mut inOnlyOneFunction: bool,
    mut inCheckTypes: bool,
    mut inImplicit: bool,
    mut inPrefix: DAE::Prefix,
    mut inInfo: SourceInfo,
    mut inFuncType: &metamodelica::Ref<DAE::Type>,
    mut inPath: metamodelica::Ref<Absyn::Path>,
    mut isGraphicsExp: bool,
) -> Result<(
    FCore::Cache,
    metamodelica::List<metamodelica::Ref<DAE::Exp>>,
    metamodelica::List<Slot>,
    metamodelica::List<DAE::Const>,
    metamodelica::List<(ArcStr, metamodelica::List<metamodelica::Ref<DAE::Type>>)>,
)> {
    let mut outCache: FCore::Cache = inCache;
    let mut outExps: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
    let mut outSlots: metamodelica::List<Slot> = inSlots.clone();
    let mut outConsts: metamodelica::List<DAE::Const>;
    let mut outPolymorphicBindings: metamodelica::List<(ArcStr, metamodelica::List<metamodelica::Ref<DAE::Type>>)> =
        metamodelica::nil();
    let mut fargs: metamodelica::List<metamodelica::Ref<DAE::FuncArg>>;
    let mut consts1: metamodelica::List<DAE::Const>;
    let mut consts2: metamodelica::List<DAE::Const>;
    if (inPosArgs).is_empty() && (inNamedArgs).is_empty() {
        outConsts = list![openmodelica_frontend_types::DAE::Const::C_CONST];
    } else {
        fargs = funcArgsFromSlots(inSlots);
        (outCache, outSlots, consts1, outPolymorphicBindings) = elabPositionalInputArgs(
            outCache,
            inEnv.clone(),
            inPosArgs,
            fargs.clone(),
            outSlots,
            inOnlyOneFunction,
            inCheckTypes,
            inImplicit,
            outPolymorphicBindings,
            inPrefix.clone(),
            inInfo.clone(),
            inPath.clone(),
            isGraphicsExp,
        )?;
        (outCache, outSlots, consts2, outPolymorphicBindings) = elabNamedInputArgs(
            outCache,
            inEnv.clone(),
            inNamedArgs,
            fargs,
            outSlots,
            inOnlyOneFunction,
            inCheckTypes,
            inImplicit,
            outPolymorphicBindings,
            &inPrefix,
            &inInfo,
            &inPath,
            isGraphicsExp,
        )?;
        outConsts = listAppend(consts1, consts2);
    }
    (outCache, outSlots) = evaluateStructuralSlots(outCache, &inEnv, outSlots, inFuncType)?;
    outExps = slotListArgs(&outSlots);
    Ok((outCache, outExps, outSlots, outConsts, outPolymorphicBindings))
}

fn makeEmptySlots(
    mut inArgs: &metamodelica::List<metamodelica::Ref<DAE::FuncArg>>,
) -> Result<metamodelica::List<Slot>> {
    let mut outSlots: metamodelica::List<Slot>;
    (outSlots, _) = List::mapFold(inArgs, &fnptr!(makeEmptySlot, metamodelica::Ref<DAE::FuncArg>, i32), 1)?;
    Ok(outSlots)
}

fn makeEmptySlot(mut inArg: metamodelica::Ref<DAE::FuncArg>, mut inIndex: i32) -> (Slot, i32) {
    let mut outSlot: Slot;
    let mut outIndex: i32;
    outSlot = Slot {
        defaultArg: inArg,
        slotFilled: false,
        arg: None,
        dims: metamodelica::nil(),
        idx: inIndex,
        evalStatus: SLOT_NOT_EVALUATED.clone(),
    };
    outIndex = inIndex + 1;
    (outSlot, outIndex)
}

fn funcArgsFromSlots(mut inSlots: metamodelica::List<Slot>) -> metamodelica::List<metamodelica::Ref<DAE::FuncArg>> {
    let mut outFuncArgs: metamodelica::List<metamodelica::Ref<DAE::FuncArg>>;
    outFuncArgs = ({
        let mut __acc: metamodelica::List<metamodelica::Ref<DAE::FuncArg>> = metamodelica::nil();
        for mut slot in (inSlots).into_iter().cloned() {
            let __x = funcArgFromSlot(slot.clone());
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    outFuncArgs
}

fn funcArgFromSlot(mut inSlot: Slot) -> metamodelica::Ref<DAE::FuncArg> {
    let mut outFuncArg: metamodelica::Ref<DAE::FuncArg>;
    let Slot { defaultArg: __pa0, .. } = inSlot;
    outFuncArg = metamodelica::Own::own(__pa0);
    outFuncArg
}

fn complexTypeFromSlots(
    mut inSlots: &metamodelica::List<Slot>,
    mut complexClassType: ClassInf::State,
) -> Result<metamodelica::Ref<DAE::Type>> {
    let mut outType: metamodelica::Ref<DAE::Type>;
    let mut id: ArcStr;
    let mut ty: metamodelica::Ref<DAE::Type>;
    let mut vars: metamodelica::List<metamodelica::Ref<DAE::Var>> = metamodelica::nil();
    for mut slot in &**inSlots {
        let Slot { defaultArg: __t2, .. } = slot.clone();
        let __arc3 = __t2.clone();
        let DAE::FUNCARG {
            name: __pa0, ty: __pa1, ..
        } = &*__arc3;
        id = metamodelica::Own::own(__pa0);
        ty = metamodelica::Own::own(__pa1);
        vars = metamodelica::cons(Expression::makeVar(id, Types::simplifyType(ty)?), vars);
    }
    vars = vars.reverse();
    outType = metamodelica::Ref::new(DAE::Type::T_COMPLEX {
        complexClassType: complexClassType,
        varLst: vars,
        equalityConstraint: None,
        usedExternally: false,
    });
    Ok(outType)
}

fn slotListArgs(mut inSlots: &metamodelica::List<Slot>) -> metamodelica::List<metamodelica::Ref<DAE::Exp>> {
    let mut outArgs: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
    outArgs = List::filterMap(inSlots, &slotArg);
    outArgs
}

fn slotArg(mut inSlot: Slot) -> Result<metamodelica::Ref<DAE::Exp>> {
    let mut outArg: metamodelica::Ref<DAE::Exp>;
    let __pa0 = ::match_deref::match_deref! { match &(inSlot) {
        Slot { arg: Some(__pa0), .. } => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    outArg = metamodelica::Own::own(__pa0);
    Ok(outArg)
}

fn fillGraphicsDefaultSlots(
    mut inCache: FCore::Cache,
    mut inSlots: &metamodelica::List<Slot>,
    mut inClass: &metamodelica::Ref<SCode::Element>,
    mut inEnv: FCore::Graph,
    mut inImplicit: bool,
    mut inPrefix: DAE::Prefix,
    mut inInfo: SourceInfo,
) -> (
    FCore::Cache,
    metamodelica::List<Slot>,
    metamodelica::List<DAE::Const>,
    metamodelica::List<(ArcStr, metamodelica::List<metamodelica::Ref<DAE::Type>>)>,
) {
    let mut outCache: FCore::Cache = inCache;
    let mut outSlots: metamodelica::List<Slot> = metamodelica::nil();
    let mut outConsts: metamodelica::List<DAE::Const> = metamodelica::nil();
    let mut outPolymorphicBindings: metamodelica::List<(ArcStr, metamodelica::List<metamodelica::Ref<DAE::Type>>)> =
        metamodelica::nil();
    let mut filled: bool;
    let mut e: metamodelica::Ref<Absyn::Exp> = metamodelica::Ref::new(Absyn::Exp::BREAK);
    let mut exp: metamodelica::Ref<DAE::Exp> = <metamodelica::Ref<DAE::Exp> as ::std::default::Default>::default();
    let mut defarg: metamodelica::Ref<DAE::FuncArg>;
    let mut ty: metamodelica::Ref<DAE::Type> = metamodelica::Ref::new(DAE::Type::T_NORETCALL);
    let mut c: DAE::Const = DAE::Const::C_CONST;
    for mut slot in &**inSlots {
        let mut slot = slot.clone();
        let Slot { slotFilled: __pa0, .. } = &slot;
        filled = metamodelica::Own::own(__pa0);
        if !(filled) {
            slot = 'mc: {
                let __mc_input = slot.clone();
                if let Ok((__v, __wb0, __wb1, __wb2, __wb3, __wb4, __wb5, __wb6, __wb7)) = (|| -> Result<_> {
                    ::match_deref::match_deref! { match &__mc_input {
                        Slot { defaultArg: defarg @ Deref @ DAE::FuncArg { .. }, .. } => {
                            let mut c: DAE::Const = c.clone();
                            let mut e: metamodelica::Ref<Absyn::Exp> = e.clone();
                            let mut exp: metamodelica::Ref<DAE::Exp> = exp.clone();
                            let mut outCache: FCore::Cache = outCache.clone();
                            let mut outConsts: metamodelica::List<DAE::Const> = outConsts.clone();
                            let mut outPolymorphicBindings: metamodelica::List<(ArcStr, metamodelica::List<metamodelica::Ref<DAE::Type>>)> = outPolymorphicBindings.clone();
                            let mut slot: Slot = slot.clone();
                            let mut ty: metamodelica::Ref<DAE::Type> = ty.clone();
                            let __pa0 = ::match_deref::match_deref! { match &(SCodeUtil::getElementNamed(defarg.name.clone(), inClass)?) {
                                Deref @ SCode::Element::COMPONENT { modifications: Deref @ SCode::Mod::MOD { binding: Some(__pa0), .. }, .. } => __pa0.clone(),
                                _ => return Err("pattern mismatch"),
                            } };
                            e = metamodelica::Own::own(__pa0);
                            let (__pa2, __pa3, __pa4, __pa5) = ::match_deref::match_deref! { match &(elabExpInExpression(outCache.clone(), inEnv.clone(), e.clone(), inImplicit, true, inPrefix.clone(), inInfo.clone())?) {
                                (__pa2, __pa3, DAE::Properties::PROP { type_: __pa4, constFlag: __pa5 }) => (__pa2.clone(), __pa3.clone(), __pa4.clone(), __pa5.clone()),
                                _ => return Err("pattern mismatch"),
                            } };
                            outCache = metamodelica::Own::own(__pa2);
                            exp = metamodelica::Own::own(__pa3);
                            ty = metamodelica::Own::own(__pa4);
                            c = metamodelica::Own::own(__pa5);
                            (exp, _, outPolymorphicBindings) = Types::matchTypePolymorphic(exp.clone(), ty.clone(), defarg.ty.clone(), FGraph::getGraphPathNoImplicitScope(&inEnv), outPolymorphicBindings.clone(), false)?;
                            let true = (Types::constEqualOrHigher(c, defarg.r#const.clone())) else { return Err("pattern mismatch") };
                            outConsts = metamodelica::cons(c, outConsts.clone());
                            slot.slotFilled = true;
                            slot.arg = Some(exp.clone());
                            Ok((slot.clone(), c.clone(), e.clone(), exp.clone(), outCache.clone(), outConsts.clone(), outPolymorphicBindings.clone(), slot.clone(), ty.clone()))
                        }
                        _ => return Err("nomatch"),
                    }}
                })() {
                    c = __wb0;
                    e = __wb1;
                    exp = __wb2;
                    outCache = __wb3;
                    outConsts = __wb4;
                    outPolymorphicBindings = __wb5;
                    slot = __wb6;
                    ty = __wb7;
                    break 'mc __v;
                }
                if let Ok(__v) = (|| -> Result<_> {
                    ::match_deref::match_deref! { match &__mc_input {
                        _ => {
                            Ok(slot.clone())
                        }
                        _ => return Err("nomatch"),
                    }}
                })() {
                    break 'mc __v;
                }
                panic!("matchcontinue: no arm matched")
            };
        }
        outSlots = metamodelica::cons(slot, outSlots);
    }
    outSlots = outSlots.reverse();
    outConsts = outConsts.reverse();
    (outCache, outSlots, outConsts, outPolymorphicBindings)
}

fn printSlotsStr(mut inSlots: &metamodelica::List<Slot>) -> Result<ArcStr> {
    let mut outString: ArcStr;
    outString = (::match_deref::match_deref! { match inSlots {
        Deref @ metamodelica::ListNode::Cons { head: Slot { defaultArg: farg, slotFilled: filled, arg: exp, dims: ds, .. }, tail: xs } => {
            let mut farg_str: ArcStr;
            let mut filledStr: ArcStr;
            let mut r#str: ArcStr;
            let mut s: ArcStr;
            let mut s1: ArcStr;
            let mut s2: ArcStr;
            let mut res: ArcStr;
            let mut str_lst: metamodelica::List<ArcStr>;
            farg_str = TypesDump::printFargStr(metamodelica::AsArg::as_arg(&farg))?;
            filledStr = if (filled.clone()) {literal!("filled")} else {literal!("not filled")};
            r#str = Util::applyOptionOrDefault(exp.clone(), &ExpressionBasics::printExpStr, literal!(""))?;
            str_lst = List::map(ds.clone(), &move |__a0: metamodelica::Ref<DAE::Dimension>| ExpressionBasics::dimensionString(&__a0))?;
            s = stringDelimitList(str_lst, literal!(", "));
            s1 = stringAppendList(list![literal!("SLOT("), farg_str, literal!(", "), filledStr, literal!(", "), r#str, literal!(", ["), s, literal!("])\n")]);
            s2 = printSlotsStr(xs)?;
            res = stringAppend(s1, s2);
            res
        },
        Deref @ metamodelica::ListNode::Nil => {
            literal!("")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outString)
}

fn isFreeParameterExp(
    mut inExp: &metamodelica::Ref<DAE::Exp>,
    mut inCache: FCore::Cache,
    mut inEnv: &FCore::Graph,
) -> Result<(bool, FCore::Cache)> {
    let mut isFree: bool;
    let mut outCache: FCore::Cache;
    outCache = inCache.clone();
    isFree = (match &**inExp {
        DAE::Exp::ICONST { .. } => true,
        DAE::Exp::RCONST { .. } => true,
        DAE::Exp::SCONST { .. } => true,
        DAE::Exp::BCONST { .. } => true,
        DAE::Exp::CREF { componentRef: cr, .. } => {
            let mut binding: metamodelica::Ref<DAE::Binding>;
            let mut exp1: metamodelica::Ref<DAE::Exp>;
            (outCache, _, _, binding, _, _, _, _, _) = Lookup::lookupVar(inCache, inEnv.clone(), cr.clone())?;
            (match &*binding {
                DAE::Binding::VALBOUND { .. } => true,
                DAE::Binding::EQBOUND { exp: exp1, .. } if (Expression::isConst(exp1.clone())?) => true,
                _ => false,
            })
        }
        DAE::Exp::BINARY { exp1, exp2, .. } => {
            let mut isFree2: bool;
            (isFree, outCache) = isFreeParameterExp(exp1, inCache, inEnv)?;
            (isFree2, outCache) = isFreeParameterExp(exp2, outCache, inEnv)?;
            isFree && isFree2
        }
        DAE::Exp::UNARY { exp: exp1, .. } => {
            (isFree, outCache) = isFreeParameterExp(exp1, inCache, inEnv)?;
            isFree
        }
        DAE::Exp::LBINARY { exp1, exp2, .. } => {
            let mut isFree2: bool;
            (isFree, outCache) = isFreeParameterExp(exp1, inCache, inEnv)?;
            (isFree2, outCache) = isFreeParameterExp(exp2, outCache, inEnv)?;
            isFree && isFree2
        }
        DAE::Exp::LUNARY { exp: exp1, .. } => {
            (isFree, outCache) = isFreeParameterExp(exp1, inCache, inEnv)?;
            isFree
        }
        DAE::Exp::CALL { expLst: exps, .. } => {
            let mut isFree2: bool;
            outCache = inCache;
            isFree = true;
            for mut exp in &*exps.clone() {
                (isFree2, outCache) = isFreeParameterExp(metamodelica::AsArg::as_arg(&exp), outCache, inEnv)?;
                isFree = isFree && isFree2;
            }
            isFree
        }
        DAE::Exp::ARRAY { array: exps, .. } => {
            let mut isFree2: bool;
            outCache = inCache;
            isFree = true;
            for mut exp in &*exps.clone() {
                (isFree2, outCache) = isFreeParameterExp(metamodelica::AsArg::as_arg(&exp), outCache, inEnv)?;
                isFree = isFree && isFree2;
            }
            isFree
        }
        DAE::Exp::MATRIX { matrix: mat, .. } => {
            let mut isFree2: bool;
            outCache = inCache;
            isFree = true;
            for mut row in &*mat.clone() {
                for mut exp in &*row.clone() {
                    (isFree2, outCache) = isFreeParameterExp(metamodelica::AsArg::as_arg(&exp), outCache, inEnv)?;
                    isFree = isFree && isFree2;
                }
            }
            isFree
        }
        DAE::Exp::CAST { exp: exp1, .. } => {
            (isFree, outCache) = isFreeParameterExp(exp1, inCache, inEnv)?;
            isFree
        }
        _ => false,
    });
    Ok((isFree, outCache))
}

fn elabPositionalInputArgs(
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
    mut inPosArgs: &metamodelica::List<metamodelica::Ref<Absyn::Exp>>,
    mut inFuncArgs: metamodelica::List<metamodelica::Ref<DAE::FuncArg>>,
    mut inSlots: metamodelica::List<Slot>,
    mut inOnlyOneFunction: bool,
    mut inCheckTypes: bool,
    mut inImplicit: bool,
    mut inPolymorphicBindings: metamodelica::List<(ArcStr, metamodelica::List<metamodelica::Ref<DAE::Type>>)>,
    mut inPrefix: DAE::Prefix,
    mut inInfo: SourceInfo,
    mut inPath: metamodelica::Ref<Absyn::Path>,
    mut isGraphicsExp: bool,
) -> Result<(
    FCore::Cache,
    metamodelica::List<Slot>,
    metamodelica::List<DAE::Const>,
    metamodelica::List<(ArcStr, metamodelica::List<metamodelica::Ref<DAE::Type>>)>,
)> {
    let mut outCache: FCore::Cache = inCache;
    let mut outSlots: metamodelica::List<Slot> = inSlots;
    let mut outConsts: metamodelica::List<DAE::Const> = metamodelica::nil();
    let mut outPolymorphicBindings: metamodelica::List<(ArcStr, metamodelica::List<metamodelica::Ref<DAE::Type>>)> =
        inPolymorphicBindings;
    let mut farg: metamodelica::Ref<DAE::FuncArg>;
    let mut farg_rest: metamodelica::List<metamodelica::Ref<DAE::FuncArg>> = inFuncArgs;
    let mut c: DAE::Const;
    let mut position: i32 = 1;
    for mut arg in &**inPosArgs {
        let (__pa0, __pa1) = ::match_deref::match_deref! { match &(farg_rest) {
            Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
            _ => return Err("pattern mismatch"),
        } };
        farg = metamodelica::Own::own(__pa0);
        farg_rest = metamodelica::Own::own(__pa1);
        (outCache, outSlots, c, outPolymorphicBindings) = elabPositionalInputArg(
            outCache,
            inEnv.clone(),
            arg.clone(),
            &farg,
            position,
            outSlots,
            inOnlyOneFunction,
            inCheckTypes,
            inImplicit,
            outPolymorphicBindings,
            inPrefix.clone(),
            inInfo.clone(),
            inPath.clone(),
            isGraphicsExp,
        )?;
        position = position + 1;
        outConsts = metamodelica::cons(c, outConsts);
    }
    outConsts = outConsts.reverse();
    Ok((outCache, outSlots, outConsts, outPolymorphicBindings))
}

fn elabPositionalInputArg(
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
    mut inExp: metamodelica::Ref<Absyn::Exp>,
    mut farg: &metamodelica::Ref<DAE::FuncArg>,
    mut position: i32,
    mut inSlotLst: metamodelica::List<Slot>,
    mut onlyOneFunction: bool,
    mut checkTypes: bool,
    mut r#impl: bool,
    mut inPolymorphicBindings: metamodelica::List<(ArcStr, metamodelica::List<metamodelica::Ref<DAE::Type>>)>,
    mut inPrefix: DAE::Prefix,
    mut info: SourceInfo,
    mut path: metamodelica::Ref<Absyn::Path>,
    mut isGraphicsExp: bool,
) -> Result<(
    FCore::Cache,
    metamodelica::List<Slot>,
    DAE::Const,
    metamodelica::List<(ArcStr, metamodelica::List<metamodelica::Ref<DAE::Type>>)>,
)> {
    let mut outCache: FCore::Cache;
    let mut outSlotLst: metamodelica::List<Slot>;
    let mut outConst: DAE::Const;
    let mut outPolymorphicBindings: metamodelica::List<(ArcStr, metamodelica::List<metamodelica::Ref<DAE::Type>>)>;
    let mut numErrors: i32 = Error::getNumErrorMessages();
    (outCache, outSlotLst, outConst, outPolymorphicBindings) = 'mc: {
        let __mc_input = (
            inCache,
            inEnv,
            inExp,
            &**farg,
            inSlotLst,
            onlyOneFunction,
            checkTypes,
            inPolymorphicBindings,
            inPrefix,
        );
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, e, Deref @ DAE::FuncArg { name: id, ty: vt @ Deref @ DAE::Type::T_CODE { ty: ct }, par: pr, .. }, slots, _, true, polymorphicBindings, pre) => {
                    let mut slots_1: metamodelica::List<Slot>;
                    let mut e_1: metamodelica::Ref<DAE::Exp>;
                    e_1 = elabCodeExp(metamodelica::AsArg::as_arg(&e), metamodelica::AsArg::as_arg(&cache), metamodelica::AsArg::as_arg(&env), ct.clone(), &info)?;
                    slots_1 = fillSlot(&(metamodelica::Ref::new(DAE::FuncArg { name: id.clone(), ty: vt.clone(), r#const: openmodelica_frontend_types::DAE::Const::C_VAR, par: pr.clone(), defaultBinding: None })), e_1.clone(), metamodelica::nil(), slots.clone(), pre.clone(), &info, path.clone())?;
                    Ok((cache.clone(), slots_1.clone(), openmodelica_frontend_types::DAE::Const::C_VAR, polymorphicBindings.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, e, Deref @ DAE::FuncArg { name: id, ty: vt, par: pr, .. }, slots, _, true, polymorphicBindings, pre) => {
                    let mut slots_1: metamodelica::List<Slot>;
                    let mut e_1: metamodelica::Ref<DAE::Exp>;
                    let mut e_2: metamodelica::Ref<DAE::Exp>;
                    let mut t: metamodelica::Ref<DAE::Type>;
                    let mut c1: DAE::Const;
                    let mut props: DAE::Properties;
                    let mut cache = (*cache).clone();
                    let mut vt = (*vt).clone();
                    let mut polymorphicBindings = (*polymorphicBindings).clone();
                    (cache, e_1, props) = elabExpInExpression(cache.clone(), env.clone(), e.clone(), r#impl, true, pre.clone(), info.clone())?;
                    t = Types::getPropType(&props);
                    (vt, _) = Types::traverseType(vt.clone(), -1, &fnptr!(Types::makeExpDimensionsUnknown, metamodelica::Ref<DAE::Type>, i32))?;
                    c1 = Types::propAllConst(props.clone())?;
                    (e_2, _, polymorphicBindings) = Types::matchTypePolymorphic(e_1.clone(), t.clone(), vt.clone(), FGraph::getGraphPathNoImplicitScope(metamodelica::AsArg::as_arg(&env)), polymorphicBindings.clone(), false)?;
                    slots_1 = fillSlot(&(metamodelica::Ref::new(DAE::FuncArg { name: id.clone(), ty: vt.clone(), r#const: c1, par: pr.clone(), defaultBinding: None })), e_2.clone(), metamodelica::nil(), slots.clone(), pre.clone(), &info, path.clone())?;
                    Ok((cache.clone(), slots_1.clone(), c1, polymorphicBindings.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, e, Deref @ DAE::FuncArg { name: id, ty: vt, par: pr, .. }, slots, _, true, polymorphicBindings, pre) => {
                    let mut slots_1: metamodelica::List<Slot>;
                    let mut e_1: metamodelica::Ref<DAE::Exp>;
                    let mut e_2: metamodelica::Ref<DAE::Exp>;
                    let mut t: metamodelica::Ref<DAE::Type>;
                    let mut c1: DAE::Const;
                    let mut ds: metamodelica::List<metamodelica::Ref<DAE::Dimension>>;
                    let mut props: DAE::Properties;
                    let mut cache = (*cache).clone();
                    let mut vt = (*vt).clone();
                    let mut polymorphicBindings = (*polymorphicBindings).clone();
                    (cache, e_1, props) = elabExpInExpression(cache.clone(), env.clone(), e.clone(), r#impl, true, pre.clone(), info.clone())?;
                    t = Types::getPropType(&props);
                    (vt, _) = Types::traverseType(vt.clone(), -1, &fnptr!(Types::makeExpDimensionsUnknown, metamodelica::Ref<DAE::Type>, i32))?;
                    c1 = Types::propAllConst(props.clone())?;
                    (e_2, _, ds, polymorphicBindings) = Types::vectorizableType(e_1.clone(), t.clone(), vt.clone(), FGraph::getGraphPathNoImplicitScope(metamodelica::AsArg::as_arg(&env)))?;
                    slots_1 = fillSlot(&(metamodelica::Ref::new(DAE::FuncArg { name: id.clone(), ty: vt.clone(), r#const: c1, par: pr.clone(), defaultBinding: None })), e_2.clone(), ds.clone(), slots.clone(), pre.clone(), &info, path.clone())?;
                    Ok((cache.clone(), slots_1.clone(), c1, polymorphicBindings.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, e, Deref @ DAE::FuncArg { name: id, par: pr, .. }, slots, _, false, polymorphicBindings, pre) => {
                    let mut slots_1: metamodelica::List<Slot>;
                    let mut e_1: metamodelica::Ref<DAE::Exp>;
                    let mut t: metamodelica::Ref<DAE::Type>;
                    let mut c1: DAE::Const;
                    let mut props: DAE::Properties;
                    let mut cache = (*cache).clone();
                    (cache, e_1, props) = elabExpInExpression(cache.clone(), env.clone(), e.clone(), r#impl, true, pre.clone(), info.clone())?;
                    t = Types::getPropType(&props);
                    c1 = Types::propAllConst(props.clone())?;
                    slots_1 = fillSlot(&(metamodelica::Ref::new(DAE::FuncArg { name: id.clone(), ty: t.clone(), r#const: c1, par: pr.clone(), defaultBinding: None })), e_1.clone(), metamodelica::nil(), slots.clone(), pre.clone(), &info, path.clone())?;
                    Ok((cache.clone(), slots_1.clone(), c1, polymorphicBindings.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, e, Deref @ DAE::FuncArg { name: id, ty: vt, .. }, _, true, true, _, pre) => {
                    let mut e_1: metamodelica::Ref<DAE::Exp>;
                    let mut prop: DAE::Properties;
                    let mut s1: ArcStr;
                    let mut s2: ArcStr;
                    let mut s3: ArcStr;
                    let mut s4: ArcStr;
                    let mut s5: ArcStr;
                    let mut cache = (*cache).clone();
                    let true = (Error::getNumErrorMessages() == numErrors) else { return Err("pattern mismatch") };
                    (cache, e_1, prop) = elabExpInExpression(cache.clone(), env.clone(), e.clone(), r#impl, true, pre.clone(), info.clone())?;
                    s1 = intString(position);
                    s2 = AbsynUtil::pathStringNoQual(path.clone(), literal!("."), false, false)?;
                    s3 = ExpressionBasics::printExpStr(e_1.clone())?;
                    s4 = TypesDump::unparseTypeNoAttr(&(Types::getPropType(&prop)))?;
                    s5 = TypesDump::unparseTypeNoAttr(metamodelica::AsArg::as_arg(&vt))?;
                    Error::addSourceMessage(&(Error::ARG_TYPE_MISMATCH.clone()), list![s1.clone(), s2.clone(), id.clone(), s3.clone(), s4.clone(), s5.clone()], &info)?;
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((outCache, outSlotLst, outConst, outPolymorphicBindings))
}

fn elabNamedInputArgs(
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
    mut inAbsynNamedArgLst: &metamodelica::List<metamodelica::Ref<Absyn::NamedArg>>,
    mut inTypesFuncArgLst: metamodelica::List<metamodelica::Ref<DAE::FuncArg>>,
    mut inSlotLst: metamodelica::List<Slot>,
    mut onlyOneFunction: bool,
    mut checkTypes: bool,
    mut r#impl: bool,
    mut inPolymorphicBindings: metamodelica::List<(ArcStr, metamodelica::List<metamodelica::Ref<DAE::Type>>)>,
    mut inPrefix: &DAE::Prefix,
    mut info: &SourceInfo,
    mut path: &metamodelica::Ref<Absyn::Path>,
    mut isGraphicsExp: bool,
) -> Result<(
    FCore::Cache,
    metamodelica::List<Slot>,
    metamodelica::List<DAE::Const>,
    metamodelica::List<(ArcStr, metamodelica::List<metamodelica::Ref<DAE::Type>>)>,
)> {
    let mut outCache: FCore::Cache;
    let mut outSlotLst: metamodelica::List<Slot>;
    let mut outTypesConstLst: metamodelica::List<DAE::Const>;
    let mut outPolymorphicBindings: metamodelica::List<(ArcStr, metamodelica::List<metamodelica::Ref<DAE::Type>>)>;
    (outCache, outSlotLst, outTypesConstLst, outPolymorphicBindings) = (::match_deref::match_deref! { match inAbsynNamedArgLst {
        Deref @ metamodelica::ListNode::Nil => {
            let mut cache = inCache;
            let mut slots = inSlotLst;
            (cache, slots, metamodelica::nil(), inPolymorphicBindings)
        },
        Deref @ metamodelica::ListNode::Cons { head: na, tail: nas } => {
            let mut cache = inCache;
            let mut env = inEnv;
            let mut farg = inTypesFuncArgLst;
            let mut slots = inSlotLst;
            let mut polymorphicBindings = inPolymorphicBindings.clone();
            let mut c1: DAE::Const;
            let mut clist: metamodelica::List<DAE::Const>;
            (cache, slots, c1, polymorphicBindings) = elabNamedInputArg(cache, env.clone(), metamodelica::AsArg::as_arg(&na), farg.clone(), slots, onlyOneFunction, checkTypes, r#impl, polymorphicBindings, inPrefix.clone(), info.clone(), path.clone(), Error::getNumErrorMessages(), isGraphicsExp)?;
            (cache, slots, clist, polymorphicBindings) = elabNamedInputArgs(cache, env, nas, farg, slots, onlyOneFunction, checkTypes, r#impl, polymorphicBindings, inPrefix, info, path, isGraphicsExp)?;
            (cache, slots, metamodelica::cons(c1, clist), polymorphicBindings)
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outCache, outSlotLst, outTypesConstLst, outPolymorphicBindings))
}

fn elabNamedInputArg(
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
    mut inNamedArg: &metamodelica::Ref<Absyn::NamedArg>,
    mut inTypesFuncArgLst: metamodelica::List<metamodelica::Ref<DAE::FuncArg>>,
    mut inSlotLst: metamodelica::List<Slot>,
    mut onlyOneFunction: bool,
    mut checkTypes: bool,
    mut r#impl: bool,
    mut inPolymorphicBindings: metamodelica::List<(ArcStr, metamodelica::List<metamodelica::Ref<DAE::Type>>)>,
    mut inPrefix: DAE::Prefix,
    mut info: SourceInfo,
    mut path: metamodelica::Ref<Absyn::Path>,
    mut numErrors: i32,
    mut isGraphicsExp: bool,
) -> Result<(
    FCore::Cache,
    metamodelica::List<Slot>,
    DAE::Const,
    metamodelica::List<(ArcStr, metamodelica::List<metamodelica::Ref<DAE::Type>>)>,
)> {
    let mut outCache: FCore::Cache;
    let mut outSlotLst: metamodelica::List<Slot>;
    let mut outTypesConstLst: DAE::Const;
    let mut outPolymorphicBindings: metamodelica::List<(ArcStr, metamodelica::List<metamodelica::Ref<DAE::Type>>)>;
    (outCache, outSlotLst, outTypesConstLst, outPolymorphicBindings) = 'mc: {
        let __mc_input = (
            inCache,
            inEnv,
            &**inNamedArg,
            inTypesFuncArgLst,
            inSlotLst,
            onlyOneFunction,
            checkTypes,
            inPolymorphicBindings,
            inPrefix,
        );
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, Deref @ Absyn::NamedArg { argName: id, argValue: e }, farg, slots, _, true, polymorphicBindings, pre) => {
                    let mut e_1: metamodelica::Ref<DAE::Exp>;
                    let mut vt: metamodelica::Ref<DAE::Type>;
                    let mut pr: DAE::VarParallelism;
                    let mut slots_1: metamodelica::List<Slot>;
                    let mut ct: DAE::CodeType;
                    let (__pa1, __pa0) = ::match_deref::match_deref! { match &(findNamedArgType(metamodelica::AsArg::as_arg(&id), metamodelica::AsArg::as_arg(&farg))?) {
                        __pa1 @ Deref @ DAE::Type::T_CODE { ty: __pa0 } => (__pa1.clone(), __pa0.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    ct = metamodelica::Own::own(__pa0);
                    vt = metamodelica::Own::own(__pa1);
                    pr = findNamedArgParallelism(metamodelica::AsArg::as_arg(&id), metamodelica::AsArg::as_arg(&farg))?;
                    e_1 = elabCodeExp(metamodelica::AsArg::as_arg(&e), metamodelica::AsArg::as_arg(&cache), metamodelica::AsArg::as_arg(&env), ct, &info)?;
                    slots_1 = fillSlot(&(metamodelica::Ref::new(DAE::FuncArg { name: id.clone(), ty: vt.clone(), r#const: openmodelica_frontend_types::DAE::Const::C_VAR, par: pr, defaultBinding: None })), e_1.clone(), metamodelica::nil(), slots.clone(), pre.clone(), &info, path.clone())?;
                    Ok((cache.clone(), slots_1.clone(), openmodelica_frontend_types::DAE::Const::C_VAR, polymorphicBindings.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, Deref @ Absyn::NamedArg { argName: id, argValue: e }, farg, slots, _, true, polymorphicBindings, pre) => {
                    let mut e_1: metamodelica::Ref<DAE::Exp>;
                    let mut e_2: metamodelica::Ref<DAE::Exp>;
                    let mut t: metamodelica::Ref<DAE::Type>;
                    let mut vt: metamodelica::Ref<DAE::Type>;
                    let mut c1: DAE::Const;
                    let mut pr: DAE::VarParallelism;
                    let mut slots_1: metamodelica::List<Slot>;
                    let mut cache = (*cache).clone();
                    let mut polymorphicBindings = (*polymorphicBindings).clone();
                    vt = findNamedArgType(metamodelica::AsArg::as_arg(&id), metamodelica::AsArg::as_arg(&farg))?;
                    pr = findNamedArgParallelism(metamodelica::AsArg::as_arg(&id), metamodelica::AsArg::as_arg(&farg))?;
                    let (__pa0, __pa1, __pa2, __pa3) = ::match_deref::match_deref! { match &(elabExpInExpression(cache.clone(), env.clone(), e.clone(), r#impl, true, pre.clone(), info.clone())?) {
                        (__pa0, __pa1, DAE::Properties::PROP { type_: __pa2, constFlag: __pa3 }) => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    cache = metamodelica::Own::own(__pa0);
                    e_1 = metamodelica::Own::own(__pa1);
                    t = metamodelica::Own::own(__pa2);
                    c1 = metamodelica::Own::own(__pa3);
                    (e_2, _, polymorphicBindings) = Types::matchTypePolymorphic(e_1.clone(), t.clone(), vt.clone(), FGraph::getGraphPathNoImplicitScope(metamodelica::AsArg::as_arg(&env)), polymorphicBindings.clone(), false)?;
                    slots_1 = fillSlot(&(metamodelica::Ref::new(DAE::FuncArg { name: id.clone(), ty: vt.clone(), r#const: c1, par: pr, defaultBinding: None })), e_2.clone(), metamodelica::nil(), slots.clone(), pre.clone(), &info, path.clone())?;
                    Ok((cache.clone(), slots_1.clone(), c1, polymorphicBindings.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, Deref @ Absyn::NamedArg { argName: id, argValue: e }, farg, slots, _, true, polymorphicBindings, pre) => {
                    let mut e_1: metamodelica::Ref<DAE::Exp>;
                    let mut e_2: metamodelica::Ref<DAE::Exp>;
                    let mut t: metamodelica::Ref<DAE::Type>;
                    let mut vt: metamodelica::Ref<DAE::Type>;
                    let mut c1: DAE::Const;
                    let mut pr: DAE::VarParallelism;
                    let mut slots_1: metamodelica::List<Slot>;
                    let mut ds: metamodelica::List<metamodelica::Ref<DAE::Dimension>>;
                    let mut cache = (*cache).clone();
                    let mut polymorphicBindings = (*polymorphicBindings).clone();
                    vt = findNamedArgType(metamodelica::AsArg::as_arg(&id), metamodelica::AsArg::as_arg(&farg))?;
                    pr = findNamedArgParallelism(metamodelica::AsArg::as_arg(&id), metamodelica::AsArg::as_arg(&farg))?;
                    let (__pa0, __pa1, __pa2, __pa3) = ::match_deref::match_deref! { match &(elabExpInExpression(cache.clone(), env.clone(), e.clone(), r#impl, true, pre.clone(), info.clone())?) {
                        (__pa0, __pa1, DAE::Properties::PROP { type_: __pa2, constFlag: __pa3 }) => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    cache = metamodelica::Own::own(__pa0);
                    e_1 = metamodelica::Own::own(__pa1);
                    t = metamodelica::Own::own(__pa2);
                    c1 = metamodelica::Own::own(__pa3);
                    (e_2, _, ds, polymorphicBindings) = Types::vectorizableType(e_1.clone(), t.clone(), vt.clone(), FGraph::getGraphPathNoImplicitScope(metamodelica::AsArg::as_arg(&env)))?;
                    slots_1 = fillSlot(&(metamodelica::Ref::new(DAE::FuncArg { name: id.clone(), ty: vt.clone(), r#const: c1, par: pr, defaultBinding: None })), e_2.clone(), ds.clone(), slots.clone(), pre.clone(), &info, path.clone())?;
                    Ok((cache.clone(), slots_1.clone(), c1, polymorphicBindings.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, Deref @ Absyn::NamedArg { argName: id, argValue: e }, farg, slots, _, false, polymorphicBindings, pre) => {
                    let mut e_1: metamodelica::Ref<DAE::Exp>;
                    let mut vt: metamodelica::Ref<DAE::Type>;
                    let mut c1: DAE::Const;
                    let mut pr: DAE::VarParallelism;
                    let mut slots_1: metamodelica::List<Slot>;
                    let mut cache = (*cache).clone();
                    vt = findNamedArgType(metamodelica::AsArg::as_arg(&id), metamodelica::AsArg::as_arg(&farg))?;
                    pr = findNamedArgParallelism(metamodelica::AsArg::as_arg(&id), metamodelica::AsArg::as_arg(&farg))?;
                    let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(elabExpInExpression(cache.clone(), env.clone(), e.clone(), r#impl, true, pre.clone(), info.clone())?) {
                        (__pa0, __pa1, DAE::Properties::PROP { type_: _, constFlag: __pa2 }) => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    cache = metamodelica::Own::own(__pa0);
                    e_1 = metamodelica::Own::own(__pa1);
                    c1 = metamodelica::Own::own(__pa2);
                    slots_1 = fillSlot(&(metamodelica::Ref::new(DAE::FuncArg { name: id.clone(), ty: vt.clone(), r#const: c1, par: pr, defaultBinding: None })), e_1.clone(), metamodelica::nil(), slots.clone(), pre.clone(), &info, path.clone())?;
                    Ok((cache.clone(), slots_1.clone(), c1, polymorphicBindings.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, _, Deref @ Absyn::NamedArg { argName: id, .. }, farg, slots, true, _, polymorphicBindings, _) => {
                    let mut s1: ArcStr;
                    if '__try0: {
                        unwrap_break_err!(findNamedArgType(metamodelica::AsArg::as_arg(&id), metamodelica::AsArg::as_arg(&farg)), '__try0);
                        Ok::<(), &'static str>(())
                    }.is_ok() { return Err("failure(): body succeeded") }
                    s1 = AbsynUtil::pathStringNoQual(path.clone(), literal!("."), false, false)?;
                    Error::addSourceMessage(&(Error::NO_SUCH_PARAMETER.clone()), list![s1.clone(), id.clone()], &info)?;
                    let true = (isGraphicsExp) else { return Err("pattern mismatch") };
                    Ok((cache.clone(), slots.clone(), openmodelica_frontend_types::DAE::Const::C_CONST, polymorphicBindings.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, Deref @ Absyn::NamedArg { argName: id, argValue: e }, farg, _, true, true, _, pre) => {
                    let mut e_1: metamodelica::Ref<DAE::Exp>;
                    let mut vt: metamodelica::Ref<DAE::Type>;
                    let mut prop: DAE::Properties;
                    let mut s1: ArcStr;
                    let mut s2: ArcStr;
                    let mut s3: ArcStr;
                    let mut s4: ArcStr;
                    let mut cache = (*cache).clone();
                    let true = (Error::getNumErrorMessages() == numErrors) else { return Err("pattern mismatch") };
                    vt = findNamedArgType(metamodelica::AsArg::as_arg(&id), metamodelica::AsArg::as_arg(&farg))?;
                    (cache, e_1, prop) = elabExpInExpression(cache.clone(), env.clone(), e.clone(), r#impl, true, pre.clone(), info.clone())?;
                    s1 = AbsynUtil::pathStringNoQual(path.clone(), literal!("."), false, false)?;
                    s2 = ExpressionBasics::printExpStr(e_1.clone())?;
                    s3 = TypesDump::unparseTypeNoAttr(&(Types::getPropType(&prop)))?;
                    s4 = TypesDump::unparseTypeNoAttr(&vt)?;
                    Error::addSourceMessage(&(Error::NAMED_ARG_TYPE_MISMATCH.clone()), list![s1.clone(), id.clone(), s2.clone(), s3.clone(), s4.clone()], &info)?;
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((outCache, outSlotLst, outTypesConstLst, outPolymorphicBindings))
}

fn findNamedArg(
    mut inIdent: &ArcStr,
    mut inArgs: &metamodelica::List<metamodelica::Ref<DAE::FuncArg>>,
) -> Result<metamodelica::Ref<DAE::FuncArg>> {
    let mut outArg: metamodelica::Ref<DAE::FuncArg>;
    let mut id: ArcStr;
    let mut haveMM: bool = Config::acceptMetaModelicaGrammar()?;
    let mut inIdent2: ArcStr = if (haveMM) {
        {
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!("$in_"));
            __mm_s.push_str(&*inIdent);
            ArcStr::from(__mm_s)
        }
    } else {
        literal!("")
    };
    for mut arg in &**inArgs {
        let __arc1 = arg.clone();
        let DAE::FUNCARG { name: __pa0, .. } = &*__arc1;
        id = metamodelica::Own::own(__pa0);
        if metamodelica::stringEq(&id, &inIdent) || haveMM && metamodelica::stringEq(&id, &inIdent2) {
            outArg = arg.clone();
            return Ok(outArg);
        }
    }
    return Err("fail");
    Ok(outArg)
}

fn findNamedArgType(
    mut inIdent: &ArcStr,
    mut inArgs: &metamodelica::List<metamodelica::Ref<DAE::FuncArg>>,
) -> Result<metamodelica::Ref<DAE::Type>> {
    let mut outType: metamodelica::Ref<DAE::Type>;
    let __arc1 = findNamedArg(inIdent, inArgs)?;
    let DAE::FUNCARG { ty: __pa0, .. } = &*__arc1;
    outType = metamodelica::Own::own(__pa0);
    Ok(outType)
}

fn findNamedArgParallelism(
    mut inIdent: &ArcStr,
    mut inArgs: &metamodelica::List<metamodelica::Ref<DAE::FuncArg>>,
) -> Result<DAE::VarParallelism> {
    let mut outParallelism: DAE::VarParallelism;
    let __arc1 = findNamedArg(inIdent, inArgs)?;
    let DAE::FUNCARG { par: __pa0, .. } = &*__arc1;
    outParallelism = metamodelica::Own::own(__pa0);
    Ok(outParallelism)
}

fn fillSlot(
    mut inFuncArg: &metamodelica::Ref<DAE::FuncArg>,
    mut inExp: metamodelica::Ref<DAE::Exp>,
    mut inDims: metamodelica::List<metamodelica::Ref<DAE::Dimension>>,
    mut inSlotLst: metamodelica::List<Slot>,
    mut inPrefix: DAE::Prefix,
    mut inInfo: &SourceInfo,
    mut r#fn: metamodelica::Ref<Absyn::Path>,
) -> Result<metamodelica::List<Slot>> {
    let mut outSlotLst: metamodelica::List<Slot> = metamodelica::nil();
    let mut fa1: ArcStr;
    let mut fa2: ArcStr;
    let mut exp_str: ArcStr;
    let mut c_str: ArcStr;
    let mut pre_str: ArcStr;
    let mut ty1: metamodelica::Ref<DAE::Type>;
    let mut c1: DAE::Const;
    let mut c2: DAE::Const;
    let mut prl: DAE::VarParallelism;
    let mut binding: Option<metamodelica::Ref<DAE::Exp>>;
    let mut filled: bool;
    let mut idx: i32;
    let mut ses: i32;
    let mut slot: Slot;
    let mut rest_slots: metamodelica::List<Slot> = inSlotLst;
    let __arc3 = &(*inFuncArg);
    let DAE::FUNCARG {
        name: __pa0,
        ty: __pa1,
        r#const: __pa2,
        ..
    } = &**__arc3;
    fa1 = metamodelica::Own::own(__pa0);
    ty1 = metamodelica::Own::own(__pa1);
    c1 = metamodelica::Own::own(__pa2);
    while !((rest_slots).is_empty()) {
        let (__pa4, __pa5) = ::match_deref::match_deref! { match &(rest_slots) {
            Deref @ metamodelica::ListNode::Cons { head: __pa4, tail: __pa5 } => (__pa4.clone(), __pa5.clone()),
            _ => return Err("pattern mismatch"),
        } };
        slot = metamodelica::Own::own(__pa4);
        rest_slots = metamodelica::Own::own(__pa5);
        let Slot { defaultArg: __t7, .. } = slot.clone();
        let __arc8 = __t7.clone();
        let DAE::FUNCARG { name: __pa6, .. } = &*__arc8;
        fa2 = metamodelica::Own::own(__pa6);
        if stringEq(&fa1, &fa2)
            || stringEq(
                &({
                    let mut __mm_s = String::new();
                    __mm_s.push_str(&*literal!("$in_"));
                    __mm_s.push_str(&*fa1);
                    ArcStr::from(__mm_s)
                }),
                &fa2,
            )
        {
            let Slot {
                defaultArg: __t15,
                slotFilled: __pa12,
                idx: __pa13,
                evalStatus: __pa14,
                ..
            } = slot;
            let __arc16 = __t15.clone();
            let DAE::FUNCARG {
                r#const: __pa9,
                par: __pa10,
                defaultBinding: __pa11,
                ..
            } = &*__arc16;
            c2 = metamodelica::Own::own(__pa9);
            prl = metamodelica::Own::own(__pa10);
            binding = metamodelica::Own::own(__pa11);
            filled = metamodelica::Own::own(__pa12);
            idx = metamodelica::Own::own(__pa13);
            ses = metamodelica::Own::own(__pa14);
            if filled {
                pre_str = PrefixUtil::printPrefixStr3(inPrefix)?;
                Error::addSourceMessageAndFail(
                    &(Error::FUNCTION_SLOT_ALREADY_FILLED.clone()),
                    list![fa2.clone(), pre_str],
                    inInfo,
                )?;
                unreachable!("Error.addSourceMessageAndFail always fails — caller-side flow-analysis hint");
            }
            if !(Types::constEqualOrHigher(c1, c2)) {
                exp_str = ExpressionBasics::printExpStr(inExp.clone())?;
                c_str = TypesDump::unparseConst(c2);
                Error::addSourceMessageAndFail(
                    &(Error::FUNCTION_SLOT_VARIABILITY.clone()),
                    list![
                        fa1,
                        exp_str,
                        AbsynUtil::pathStringNoQual(r#fn, literal!("."), false, false)?,
                        TypesDump::unparseConst(c1),
                        c_str
                    ],
                    inInfo,
                )?;
                unreachable!("Error.addSourceMessageAndFail always fails — caller-side flow-analysis hint");
            }
            slot = Slot {
                defaultArg: metamodelica::Ref::new(DAE::FuncArg {
                    name: fa2,
                    ty: ty1,
                    r#const: c2,
                    par: prl,
                    defaultBinding: binding,
                }),
                slotFilled: true,
                arg: Some(inExp),
                dims: inDims,
                idx: idx,
                evalStatus: ses,
            };
            outSlotLst = List::append_reverse(&outSlotLst, metamodelica::cons(slot, rest_slots));
            return Ok(outSlotLst);
        }
        outSlotLst = metamodelica::cons(slot, outSlotLst);
    }
    Error::addSourceMessageAndFail(&(Error::NO_SUCH_PARAMETER.clone()), list![literal!(""), fa1], inInfo)?;
    unreachable!("Error.addSourceMessageAndFail always fails — caller-side flow-analysis hint");
    Ok(outSlotLst)
}

pub(crate) fn elabCref(
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
    mut inComponentRef: metamodelica::Ref<Absyn::ComponentRef>,
    mut inImplicit: bool,
    mut performVectorization: bool,
    mut inPrefix: DAE::Prefix,
    mut info: SourceInfo,
) -> Result<(
    FCore::Cache,
    Option<(
        metamodelica::Ref<DAE::Exp>,
        DAE::Properties,
        metamodelica::Ref<DAE::Attributes>,
    )>,
)> {
    let mut outCache: FCore::Cache;
    let mut res: Option<(
        metamodelica::Ref<DAE::Exp>,
        DAE::Properties,
        metamodelica::Ref<DAE::Attributes>,
    )>;
    (outCache, res) = elabCref1(
        inCache,
        inEnv,
        inComponentRef,
        inImplicit,
        performVectorization,
        inPrefix,
        true,
        info,
    )?;
    Ok((outCache, res))
}

pub(crate) fn elabCrefNoEval(
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
    mut inComponentRef: metamodelica::Ref<Absyn::ComponentRef>,
    mut inImplicit: bool,
    mut performVectorization: bool,
    mut inPrefix: DAE::Prefix,
    mut info: SourceInfo,
) -> Result<(
    FCore::Cache,
    metamodelica::Ref<DAE::Exp>,
    DAE::Properties,
    metamodelica::Ref<DAE::Attributes>,
)> {
    let mut outCache: FCore::Cache;
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    let mut outProperties: DAE::Properties;
    let mut outAttributes: metamodelica::Ref<DAE::Attributes>;
    let (__pa0, __pa1, __pa2, __pa3) = ::match_deref::match_deref! { match &(elabCref1(inCache, inEnv, inComponentRef, inImplicit, performVectorization, inPrefix, false, info)?) {
        (__pa0, Some((__pa1, __pa2, __pa3))) => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone()),
        _ => return Err("pattern mismatch"),
    } };
    outCache = metamodelica::Own::own(__pa0);
    outExp = metamodelica::Own::own(__pa1);
    outProperties = metamodelica::Own::own(__pa2);
    outAttributes = metamodelica::Own::own(__pa3);
    Ok((outCache, outExp, outProperties, outAttributes))
}

fn elabCref1(
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
    mut inComponentRef: metamodelica::Ref<Absyn::ComponentRef>,
    mut inImplicit: bool,
    mut performVectorization: bool,
    mut inPrefix: DAE::Prefix,
    mut evalCref: bool,
    mut info: SourceInfo,
) -> Result<(
    FCore::Cache,
    Option<(
        metamodelica::Ref<DAE::Exp>,
        DAE::Properties,
        metamodelica::Ref<DAE::Attributes>,
    )>,
)> {
    let mut outCache: FCore::Cache;
    let mut res: Option<(
        metamodelica::Ref<DAE::Exp>,
        DAE::Properties,
        metamodelica::Ref<DAE::Attributes>,
    )> = None;
    (outCache, res) = 'mc: {
        let __mc_input = (
            inCache.clone(),
            inEnv.clone(),
            inComponentRef.clone(),
            inImplicit,
            inPrefix,
        );
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, _, Deref @ Absyn::ComponentRef::WILD { .. }, _, _) => {
                    let mut t: metamodelica::Ref<DAE::Type>;
                    let mut crefExp: metamodelica::Ref<DAE::Exp>;
                    let mut et: metamodelica::Ref<DAE::Type>;
                    t = DAE::T_ANYTYPE_DEFAULT().clone();
                    et = Types::simplifyType(t.clone())?;
                    crefExp = Expression::makeCrefExp(openmodelica_frontend_types::DAE::ComponentRef::interned_WILD(), et.clone())?;
                    Ok((cache.clone(), Some((crefExp.clone(), DAE::Properties::PROP { type_: t.clone(), constFlag: openmodelica_frontend_types::DAE::Const::C_VAR }, DAE::dummyAttrVar().clone()))))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, _, Deref @ Absyn::ComponentRef::CREF_IDENT { name: Deref @ "Boolean", .. }, _, _) => {
                    let mut t: metamodelica::Ref<DAE::Type>;
                    let mut exp: metamodelica::Ref<DAE::Exp>;
                    exp = Expression::makeScalarArray(list![metamodelica::Ref::new(DAE::Exp::BCONST { bool: false }), metamodelica::Ref::new(DAE::Exp::BCONST { bool: true })], DAE::T_BOOL_DEFAULT().clone());
                    t = metamodelica::Ref::new(DAE::Type::T_ARRAY { ty: DAE::T_BOOL_DEFAULT().clone(), dims: list![metamodelica::Ref::new(DAE::Dimension::DIM_INTEGER { integer: 2 })] });
                    Ok((cache.clone(), Some((exp.clone(), DAE::Properties::PROP { type_: t.clone(), constFlag: openmodelica_frontend_types::DAE::Const::C_CONST }, DAE::dummyAttrConst().clone()))))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, Deref @ Absyn::ComponentRef::CREF_IDENT { name: Deref @ "time", .. }, _, _) => {
                    let mut res: Option<(metamodelica::Ref<DAE::Exp>, DAE::Properties, metamodelica::Ref<DAE::Attributes>)> = res.clone();
                    res = if (isValidTimeScope(inEnv.clone(), &info)?) {BUILTIN_TIME().clone()} else {None};
                    Ok(((inCache.clone(), res.clone()), res.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            res = __wb0;
            break 'mc __v;
        }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, Deref @ Absyn::ComponentRef::CREF_QUAL { .. }, r#impl, pre) => {
                    let mut e: metamodelica::Ref<Absyn::Exp>;
                    let mut subscripts: metamodelica::List<metamodelica::Ref<Absyn::Subscript>>;
                    let mut stripped_cref: metamodelica::Ref<Absyn::ComponentRef>;
                    let mut cache = (*cache).clone();
                    let mut res: Option<(metamodelica::Ref<DAE::Exp>, DAE::Properties, metamodelica::Ref<DAE::Attributes>)> = res.clone();
                    let true = (Config::acceptMetaModelicaGrammar()?) else { return Err("pattern mismatch") };
                    let true = (AbsynUtil::crefHasSubscripts(&inComponentRef)) else { return Err("pattern mismatch") };
                    subscripts = AbsynUtil::crefGetLastSubs(&inComponentRef)?;
                    stripped_cref = AbsynUtil::crefStripLastSubs(inComponentRef.clone())?;
                    let true = (!(AbsynUtil::crefHasSubscripts(&stripped_cref)) && ((subscripts).len() as i32) == 1) else { return Err("pattern mismatch") };
                    let __pa0 = ::match_deref::match_deref! { match &(subscripts.clone()) {
                        Deref @ metamodelica::ListNode::Cons { head: Deref @ Absyn::Subscript::SUBSCRIPT { subscript: __pa0 }, tail: Deref @ metamodelica::ListNode::Nil } => __pa0.clone(),
                        _ => return Err("pattern mismatch"),
                    } };
                    e = metamodelica::Own::own(__pa0);
                    (cache, res) = elabCrefArraySubscripts(stripped_cref.clone(), e.clone(), cache.clone(), env.clone(), pre.clone(), evalCref, r#impl.clone(), info.clone())?;
                    Ok(((cache.clone(), res.clone()), res.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            res = __wb0;
            break 'mc __v;
        }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, Deref @ Absyn::ComponentRef::CREF_IDENT { name: id, subscripts: Deref @ metamodelica::ListNode::Cons { head: Deref @ Absyn::Subscript::SUBSCRIPT { subscript: e }, tail: Deref @ metamodelica::ListNode::Nil } }, r#impl, pre) => {
                    let mut cache = (*cache).clone();
                    let mut res: Option<(metamodelica::Ref<DAE::Exp>, DAE::Properties, metamodelica::Ref<DAE::Attributes>)> = res.clone();
                    let true = (Config::acceptMetaModelicaGrammar()?) else { return Err("pattern mismatch") };
                    (cache, res) = elabCrefArraySubscripts(metamodelica::Ref::new(Absyn::ComponentRef::CREF_IDENT { name: id.clone(), subscripts: metamodelica::nil() }), e.clone(), cache.clone(), env.clone(), pre.clone(), evalCref, r#impl.clone(), info.clone())?;
                    Ok(((cache.clone(), res.clone()), res.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            res = __wb0;
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, c, r#impl, pre) => {
                    let mut c_1: metamodelica::Ref<DAE::ComponentRef>;
                    let mut r#const: DAE::Const;
                    let mut constSubs: DAE::Const;
                    let mut t: metamodelica::Ref<DAE::Type>;
                    let mut exp: metamodelica::Ref<DAE::Exp>;
                    let mut hasZeroSizeDim: bool;
                    let mut splicedExpData: InstTypes::SplicedExpData;
                    let mut forIteratorConstOpt: Option<DAE::Const>;
                    let mut attr: metamodelica::Ref<DAE::Attributes>;
                    let mut binding: metamodelica::Ref<DAE::Binding>;
                    let mut cache = (*cache).clone();
                    let mut env = (*env).clone();
                    let mut c = (*c).clone();
                    c = replaceEnd(c.clone())?;
                    env = if (AbsynUtil::crefIsFullyQualified(&inComponentRef)) {FGraph::topScope(&inEnv)?} else {inEnv.clone()};
                    (cache, c_1, constSubs, hasZeroSizeDim) = elabCrefSubs(cache.clone(), env.clone(), inEnv.clone(), c.clone(), pre.clone(), openmodelica_frontend_types::DAE::Prefix::NOPRE, r#impl.clone(), false, &info)?;
                    (cache, attr, t, binding, forIteratorConstOpt, splicedExpData, _, _, _) = Lookup::lookupVar(cache.clone(), env.clone(), c_1.clone())?;
                    (cache, exp, r#const, attr) = elabCref2(cache.clone(), metamodelica::AsArg::as_arg(&env), &c_1, &attr, constSubs, forIteratorConstOpt.clone(), &t, binding.clone(), performVectorization, &splicedExpData, metamodelica::AsArg::as_arg(&pre), evalCref, &info)?;
                    t = fixEnumerationType(t.clone());
                    (exp, r#const) = evaluateEmptyVariable(hasZeroSizeDim && evalCref, exp.clone(), t.clone(), r#const);
                    Ok((cache.clone(), Some((exp.clone(), DAE::Properties::PROP { type_: t.clone(), constFlag: r#const }, attr.clone()))))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, c, _, _) => {
                    let mut t: metamodelica::Ref<DAE::Type>;
                    let mut exp: metamodelica::Ref<DAE::Exp>;
                    let mut path: metamodelica::Ref<Absyn::Path>;
                    let mut enum_lit_strs: metamodelica::List<ArcStr>;
                    let mut typeStr: ArcStr;
                    let mut cl: metamodelica::Ref<SCode::Element>;
                    let mut cache = (*cache).clone();
                    let mut env = (*env).clone();
                    let mut c = (*c).clone();
                    c = replaceEnd(c.clone())?;
                    path = AbsynUtil::crefToPath(metamodelica::AsArg::as_arg(&c))?;
                    let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &(Lookup::lookupClass(metamodelica::AsArg::as_arg(&cache), metamodelica::AsArg::as_arg(&env), &path, None)?) {
                        (__pa0, __pa1 @ Deref @ SCode::Element::CLASS { restriction: SCode::Restriction::R_ENUMERATION { .. }, .. }, __pa2) => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    cache = metamodelica::Own::own(__pa0);
                    cl = metamodelica::Own::own(__pa1);
                    env = metamodelica::Own::own(__pa2);
                    typeStr = AbsynUtil::pathLastIdent(&path);
                    path = FGraph::joinScopePath(metamodelica::AsArg::as_arg(&env), metamodelica::Ref::new(Absyn::Path::IDENT { name: typeStr.clone() }))?;
                    enum_lit_strs = SCodeUtil::componentNames(&cl);
                    (exp, t) = makeEnumerationArray(path.clone(), enum_lit_strs.clone())?;
                    Ok((cache.clone(), Some((exp.clone(), DAE::Properties::PROP { type_: t.clone(), constFlag: openmodelica_frontend_types::DAE::Const::C_CONST }, DAE::dummyAttrConst().clone()))))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                        (cache, env, c, _, _) => {
                            let mut t: metamodelica::Ref<DAE::Type>;
                            let mut origt: metamodelica::Ref<DAE::Type>;
                            let mut exp: metamodelica::Ref<DAE::Exp>;
                            let mut isBuiltinFn: bool;
                            let mut isBuiltinFnOrInlineBuiltin: bool;
                            let mut path: metamodelica::Ref<Absyn::Path>;
                            let mut fpath: metamodelica::Ref<Absyn::Path>;
                            let mut expCref: metamodelica::Ref<DAE::ComponentRef>;
                            let mut isBuiltin: DAE::FunctionBuiltin;
                            let mut cache = (*cache).clone();
                            let mut c = (*c).clone();
                            path = AbsynUtil::crefToPath(metamodelica::AsArg::as_arg(&c))?;
                            let (__pa0, __pa1) = ::match_deref::match_deref! { match &(lookupFunctionsInEnvNoError(cache.clone(), env.clone(), path.clone(), info.clone())?) {
                                (__pa0, Deref @ metamodelica::ListNode::Cons { head: __pa1, tail: Deref @ metamodelica::ListNode::Nil }) => (__pa0.clone(), __pa1.clone()),
                                _ => return Err("pattern mismatch"),
                            } };
                            cache = metamodelica::Own::own(__pa0);
                            t = metamodelica::Own::own(__pa1);
                            (isBuiltin, isBuiltinFn, path) = isBuiltinFunc(path.clone(), &t);
                            isBuiltinFnOrInlineBuiltin = !(openmodelica_frontend_types::DAE::FunctionBuiltin::FUNCTION_NOT_BUILTIN == isBuiltin.clone());
                            fpath = (match &*t {
                DAE::Type::T_FUNCTION { path: __t_path, .. } => __t_path.clone(),
                _ => return Err("match: no arm matched"),
            });
                            origt = t.clone();
                            t = Types::makeFunctionPolymorphicReference(&t)?;
                            c = AbsynUtil::pathToCref(&fpath);
                            expCref = ComponentReference::toExpCref(metamodelica::AsArg::as_arg(&c))?;
                            exp = Expression::makeCrefExp(expCref.clone(), metamodelica::Ref::new(DAE::Type::T_FUNCTION_REFERENCE_FUNC { builtin: isBuiltinFnOrInlineBuiltin, functionType: origt.clone() }))?;
                            let (__pa3, Util::SUCCESS { .. }) = (instantiateDaeFunction(cache.clone(), env.clone(), path.clone(), isBuiltinFn, None, true)) else { return Err("pattern mismatch") };
                            cache = metamodelica::Own::own(__pa3);
                            Ok((cache.clone(), Some((exp.clone(), DAE::Properties::PROP { type_: t.clone(), constFlag: openmodelica_frontend_types::DAE::Const::C_VAR }, DAE::dummyAttrConst().clone()))))
                        }
                        _ => return Err("nomatch"),
                    }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, _, Deref @ Absyn::ComponentRef::CREF_IDENT { name: Deref @ "NONE", subscripts: Deref @ metamodelica::ListNode::Nil }, _, _) => {
                    let true = (Config::acceptMetaModelicaGrammar()?) else { return Err("pattern mismatch") };
                    Error::addSourceMessage(&(Error::META_NONE_CREF.clone()), metamodelica::nil(), &info)?;
                    Ok((cache.clone(), None))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, env, c, _, _) => {
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { return Err("pattern mismatch") };
                    Debug::traceln({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("- Static.elabCref failed: ")); __mm_s.push_str(&*Dump::printComponentRefStr(metamodelica::AsArg::as_arg(&c))?); __mm_s.push_str(&*literal!(" in env: ")); __mm_s.push_str(&*FGraph::printGraphPathStr(metamodelica::AsArg::as_arg(&env))); ArcStr::from(__mm_s) })?;
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, c, r#impl, pre) => {
                    let mut s: ArcStr;
                    let mut scope: ArcStr;
                    if '__try0: {
                        unwrap_break_err!(elabCrefSubs(cache.clone(), env.clone(), env.clone(), c.clone(), pre.clone(), openmodelica_frontend_types::DAE::Prefix::NOPRE, r#impl.clone(), false, &info), '__try0);
                        Ok::<(), &'static str>(())
                    }.is_ok() { return Err("failure(): body succeeded") }
                    s = Dump::printComponentRefStr(metamodelica::AsArg::as_arg(&c))?;
                    scope = FGraph::printGraphPathStr(metamodelica::AsArg::as_arg(&env));
                    Error::addSourceMessage(&(Error::LOOKUP_VARIABLE_ERROR.clone()), list![s.clone(), scope.clone()], &info)?;
                    Ok((cache.clone(), None))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((outCache, res))
}

fn elabCrefArraySubscripts(
    mut cref: metamodelica::Ref<Absyn::ComponentRef>,
    mut e: metamodelica::Ref<Absyn::Exp>,
    mut cache: FCore::Cache,
    mut env: FCore::Graph,
    mut pre: DAE::Prefix,
    mut evalCref: bool,
    mut r#impl: bool,
    mut info: SourceInfo,
) -> Result<(
    FCore::Cache,
    Option<(
        metamodelica::Ref<DAE::Exp>,
        DAE::Properties,
        metamodelica::Ref<DAE::Attributes>,
    )>,
)> {
    let mut cache: FCore::Cache = cache;
    let mut res: Option<(
        metamodelica::Ref<DAE::Exp>,
        DAE::Properties,
        metamodelica::Ref<DAE::Attributes>,
    )>;
    let mut exp: metamodelica::Ref<DAE::Exp>;
    let mut exp1: metamodelica::Ref<DAE::Exp>;
    let mut exp2: metamodelica::Ref<DAE::Exp>;
    let mut r#const: DAE::Const;
    let mut const1: DAE::Const;
    let mut const2: DAE::Const;
    let mut t: metamodelica::Ref<DAE::Type>;
    let mut sub_ty: metamodelica::Ref<DAE::Type>;
    let mut attr: metamodelica::Ref<DAE::Attributes>;
    let (__pa0, __pa1, __pa2, __pa3, __pa4) = ::match_deref::match_deref! { match &(elabCref1(cache, env.clone(), cref, false, false, pre.clone(), evalCref, info.clone())?) {
        (__pa0, Some((__pa1, DAE::Properties::PROP { type_: __pa2, constFlag: __pa3 }, __pa4))) => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone(), __pa4.clone()),
        _ => return Err("pattern mismatch"),
    } };
    cache = metamodelica::Own::own(__pa0);
    exp1 = metamodelica::Own::own(__pa1);
    t = metamodelica::Own::own(__pa2);
    const1 = metamodelica::Own::own(__pa3);
    attr = metamodelica::Own::own(__pa4);
    t = Types::metaArrayElementType(&t)?;
    let (__pa5, __pa6, __pa7, __pa8) = ::match_deref::match_deref! { match &(elabExpInExpression(cache, env, e, r#impl, false, pre, info)?) {
        (__pa5, __pa6, DAE::Properties::PROP { type_: __pa7, constFlag: __pa8 }) => (__pa5.clone(), __pa6.clone(), __pa7.clone(), __pa8.clone()),
        _ => return Err("pattern mismatch"),
    } };
    cache = metamodelica::Own::own(__pa5);
    exp2 = metamodelica::Own::own(__pa6);
    sub_ty = metamodelica::Own::own(__pa7);
    const2 = metamodelica::Own::own(__pa8);
    if Types::isMetaBoxedType(&sub_ty) {
        sub_ty = Types::unboxedType(sub_ty)?;
        exp2 = metamodelica::Ref::new(DAE::Exp::UNBOX {
            exp: exp2,
            ty: sub_ty.clone(),
        });
    }
    let true = (Types::isScalarInteger(&sub_ty)) else {
        return Err("pattern mismatch");
    };
    r#const = Types::constAnd(const1, const2);
    exp = Expression::makeASUB(exp1, list![exp2])?;
    res = Some((
        exp,
        DAE::Properties::PROP {
            type_: t,
            constFlag: r#const,
        },
        attr,
    ));
    Ok((cache, res))
}

fn isValidTimeScope(mut inEnv: FCore::Graph, mut inInfo: &SourceInfo) -> Result<bool> {
    let mut outIsValid: bool;
    let mut res: SCode::Restriction;
    if let Ok(__iflet0) = FGraph::lastScopeRestriction(inEnv.clone()) {
        res = __iflet0;
    } else {
        outIsValid = true;
        return Ok(outIsValid);
    }
    outIsValid = (match res {
        SCode::Restriction::R_CLASS { .. } => true,
        SCode::Restriction::R_OPTIMIZATION { .. } => true,
        SCode::Restriction::R_MODEL { .. } => true,
        SCode::Restriction::R_BLOCK { .. } => true,
        _ => {
            Error::addSourceMessage(&(Error::INVALID_TIME_SCOPE.clone()), metamodelica::nil(), inInfo)?;
            false
        }
    });
    Ok(outIsValid)
}

fn lookupFunctionsInEnvNoError(
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
    mut inPath: metamodelica::Ref<Absyn::Path>,
    mut inInfo: SourceInfo,
) -> Result<(FCore::Cache, metamodelica::List<metamodelica::Ref<DAE::Type>>)> {
    let mut outCache: FCore::Cache;
    let mut outTypesTypeLst: metamodelica::List<metamodelica::Ref<DAE::Type>>;
    (outCache, outTypesTypeLst) = (match inInfo.clone() {
        _ => {
            ErrorExt::setCheckpoint(literal!("Static.lookupFunctionsInEnvNoError"));
            (outCache, outTypesTypeLst) = Lookup::lookupFunctionsInEnv(inCache, inEnv, inPath, inInfo);
            ErrorExt::rollBack(literal!("Static.lookupFunctionsInEnvNoError"));
            (outCache, outTypesTypeLst)
        }
        _ => {
            ErrorExt::rollBack(literal!("Static.lookupFunctionsInEnvNoError"));
            return Err("fail");
        }
    });
    Ok((outCache, outTypesTypeLst))
}

fn evaluateEmptyVariable(
    mut hasZeroSizeDim: bool,
    mut inExp: metamodelica::Ref<DAE::Exp>,
    mut ty: metamodelica::Ref<DAE::Type>,
    mut c: DAE::Const,
) -> (metamodelica::Ref<DAE::Exp>, DAE::Const) {
    let mut oexp: metamodelica::Ref<DAE::Exp>;
    let mut oc: DAE::Const;
    (oexp, oc) = 'mc: {
        let __mc_input = (hasZeroSizeDim, &*inExp);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                        (true, Deref @ DAE::Exp::ASUB { sub: ss, .. }) => {
                            let mut sc: bool;
                            let mut a: bool;
                            let mut et: metamodelica::Ref<DAE::Type>;
                            let mut expl: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                            let mut exp: metamodelica::Ref<DAE::Exp>;
                            expl = ({
                let mut __acc: metamodelica::List<metamodelica::Ref<DAE::Exp>> = metamodelica::nil();
                for mut sub in (ss.clone()).into_iter().cloned() {
                            let __x = Expression::getSubscriptExp(&(sub.clone()))?;
                            __acc = cons(__x, __acc);
                }
                __acc.reverse()
            });
                            a = Types::isArray(&ty);
                            sc = boolNot(a);
                            et = Types::simplifyType(ty.clone())?;
                            exp = metamodelica::Ref::new(DAE::Exp::ARRAY { ty: et.clone(), scalar: sc, array: metamodelica::nil() });
                            exp = Expression::makeASUB(exp.clone(), expl.clone())?;
                            Ok((exp.clone(), c))
                        }
                        _ => return Err("nomatch"),
                    }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (true, Deref @ DAE::Exp::CREF { componentRef: cr, .. }) => {
                    let mut sc: bool;
                    let mut a: bool;
                    let mut et: metamodelica::Ref<DAE::Type>;
                    let mut exp: metamodelica::Ref<DAE::Exp>;
                    a = Types::isArray(&ty);
                    sc = boolNot(a);
                    et = Types::simplifyType(ty.clone())?;
                    ::match_deref::match_deref! { match &(ComponentReference::crefLastSubs(metamodelica::AsArg::as_arg(&cr))?) {
                        Deref @ metamodelica::ListNode::Nil => (),
                        _ => return Err("pattern mismatch"),
                    } };
                    exp = metamodelica::Ref::new(DAE::Exp::ARRAY { ty: et.clone(), scalar: sc, array: metamodelica::nil() });
                    Ok((exp.clone(), c))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (true, Deref @ DAE::Exp::CREF { componentRef: cr, .. }) => {
                    let mut sc: bool;
                    let mut a: bool;
                    let mut et: metamodelica::Ref<DAE::Type>;
                    let mut ss: metamodelica::List<metamodelica::Ref<DAE::Subscript>>;
                    let mut exp: metamodelica::Ref<DAE::Exp>;
                    a = Types::isArray(&ty);
                    sc = boolNot(a);
                    et = Types::simplifyType(ty.clone())?;
                    let __pa0 = ::match_deref::match_deref! { match &(ComponentReference::crefLastSubs(metamodelica::AsArg::as_arg(&cr))?) {
                        __pa0 @ Deref @ metamodelica::ListNode::Cons { head: _, tail: _ } => __pa0.clone(),
                        _ => return Err("pattern mismatch"),
                    } };
                    ss = metamodelica::Own::own(__pa0);
                    exp = metamodelica::Ref::new(DAE::Exp::ARRAY { ty: et.clone(), scalar: sc, array: metamodelica::nil() });
                    exp = Expression::makeASUB(exp.clone(), List::map(ss.clone(), &move |__a0: metamodelica::Ref<DAE::Subscript>| Expression::getSubscriptExp(&__a0))?)?;
                    Ok((exp.clone(), c))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((inExp.clone(), c))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    (oexp, oc)
}

pub(crate) fn fixEnumerationType(mut inType: metamodelica::Ref<DAE::Type>) -> metamodelica::Ref<DAE::Type> {
    let mut outType: metamodelica::Ref<DAE::Type>;
    outType = (match &*inType {
        DAE::Type::T_ENUMERATION {
            index: Some(_),
            path: p,
            names: n,
            literalVarLst: v,
            attributeLst: al,
        } => metamodelica::Ref::new(DAE::Type::T_ENUMERATION {
            index: None,
            path: p.clone(),
            names: n.clone(),
            literalVarLst: v.clone(),
            attributeLst: al.clone(),
        }),
        _ => inType,
    });
    outType
}

pub(crate) fn applySubscriptsVariability(
    mut inVariability: SCode::Variability,
    mut inSubsConst: DAE::Const,
) -> SCode::Variability {
    let mut outVariability: SCode::Variability;
    outVariability = (match (inVariability, inSubsConst) {
        (SCode::Variability::PARAM { .. }, DAE::Const::C_VAR { .. }) => {
            openmodelica_frontend_types::SCode::Variability::VAR
        }
        (SCode::Variability::CONST { .. }, DAE::Const::C_VAR { .. }) => {
            openmodelica_frontend_types::SCode::Variability::VAR
        }
        (SCode::Variability::CONST { .. }, DAE::Const::C_PARAM { .. }) => {
            openmodelica_frontend_types::SCode::Variability::PARAM
        }
        _ => inVariability,
    });
    outVariability
}

pub(crate) fn makeEnumerationArray(
    mut enumTypeName: metamodelica::Ref<Absyn::Path>,
    mut enumLiterals: metamodelica::List<ArcStr>,
) -> Result<(metamodelica::Ref<DAE::Exp>, metamodelica::Ref<DAE::Type>)> {
    let mut enumArray: metamodelica::Ref<DAE::Exp>;
    let mut enumArrayType: metamodelica::Ref<DAE::Type>;
    let mut enum_lit_expl: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
    let mut sz: i32;
    let mut ety: metamodelica::Ref<DAE::Type>;
    enum_lit_expl = Expression::makeEnumLiterals(enumTypeName.clone(), enumLiterals.clone())?;
    sz = ((enumLiterals).len() as i32);
    ety = metamodelica::Ref::new(DAE::Type::T_ARRAY {
        ty: metamodelica::Ref::new(DAE::Type::T_ENUMERATION {
            index: None,
            path: enumTypeName.clone(),
            names: enumLiterals.clone(),
            literalVarLst: metamodelica::nil(),
            attributeLst: metamodelica::nil(),
        }),
        dims: list![metamodelica::Ref::new(DAE::Dimension::DIM_ENUM {
            enumTypeName: enumTypeName,
            literals: enumLiterals,
            size: sz
        })],
    });
    enumArray = metamodelica::Ref::new(DAE::Exp::ARRAY {
        ty: ety.clone(),
        scalar: true,
        array: enum_lit_expl,
    });
    enumArrayType = ety;
    Ok((enumArray, enumArrayType))
}

fn fillCrefSubscripts(
    mut inComponentRef: metamodelica::Ref<DAE::ComponentRef>,
    mut inType: metamodelica::Ref<DAE::Type>,
) -> Result<metamodelica::Ref<DAE::ComponentRef>> {
    let mut outComponentRef: metamodelica::Ref<DAE::ComponentRef>;
    outComponentRef = (::match_deref::match_deref! { match &(inComponentRef) {
        e @ Deref @ DAE::ComponentRef::CREF_IDENT { subscriptLst: Deref @ metamodelica::ListNode::Nil, .. } => {
            e.clone()
        },
        Deref @ DAE::ComponentRef::CREF_IDENT { ident: id, identType: ty2, subscriptLst: subs } => {
            let mut t = inType;
            let mut subs_1: metamodelica::List<metamodelica::Ref<DAE::Subscript>>;
            subs_1 = fillSubscripts(subs.clone(), &t);
            ComponentReferenceBasics::makeCrefIdent(id.clone(), ty2.clone(), subs_1)
        },
        Deref @ DAE::ComponentRef::CREF_QUAL { ident: id, subscriptLst: subs, componentRef: cref, identType: ty2 } => {
            let mut t = inType;
            let mut cref_1: metamodelica::Ref<DAE::ComponentRef>;
            let mut subs = (*subs).clone();
            subs = fillSubscripts(subs.clone(), metamodelica::AsArg::as_arg(&ty2));
            t = stripPrefixType(&t, metamodelica::AsArg::as_arg(&ty2));
            cref_1 = fillCrefSubscripts(cref.clone(), t)?;
            ComponentReferenceBasics::makeCrefQual(id.clone(), ty2.clone(), subs.clone(), cref_1)
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok(outComponentRef)
}

fn stripPrefixType<'__b>(
    mut inType: &'__b metamodelica::Ref<DAE::Type>,
    mut inPrefixType: &'__b metamodelica::Ref<DAE::Type>,
) -> metamodelica::Ref<DAE::Type> {
    '__tco: loop {
        ::match_deref::match_deref! { match (inType, inPrefixType) {
            (Deref @ DAE::Type::T_ARRAY { ty: t, .. }, Deref @ DAE::Type::T_ARRAY { ty: pt, .. }) => {
                { (inType, inPrefixType) = (t, pt); continue '__tco; }
            },
            _ => {
                return inType.clone()
            },
            _ => unreachable!("tail-call lowered match: no arm matched"),
        } }
    }
}

fn fillSubscripts(
    mut inExpSubscriptLst: metamodelica::List<metamodelica::Ref<DAE::Subscript>>,
    mut inType: &metamodelica::Ref<DAE::Type>,
) -> metamodelica::List<metamodelica::Ref<DAE::Subscript>> {
    let mut outExpSubscriptLst: metamodelica::List<metamodelica::Ref<DAE::Subscript>>;
    outExpSubscriptLst = 'mc: {
        let __mc_input = &**inType;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Type::T_ARRAY { .. } => {
                    let mut subs: metamodelica::List<metamodelica::Ref<DAE::Subscript>>;
                    subs = List::fill(openmodelica_frontend_types::DAE::Subscript::interned_WHOLEDIM(), (((TypesDump::getDimensions(inType))).len() as i32));
                    subs = List::stripN(subs.clone(), ((inExpSubscriptLst).len() as i32))?;
                    subs = listAppend(inExpSubscriptLst.clone(), subs.clone());
                    Ok(subs.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(inExpSubscriptLst.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    outExpSubscriptLst
}

fn elabCref2(
    mut inCache: FCore::Cache,
    mut inEnv: &FCore::Graph,
    mut inCref: &metamodelica::Ref<DAE::ComponentRef>,
    mut inAttributes: &metamodelica::Ref<DAE::Attributes>,
    mut constSubs: DAE::Const,
    mut inIteratorConst: Option<DAE::Const>,
    mut inType: &metamodelica::Ref<DAE::Type>,
    mut inBinding: metamodelica::Ref<DAE::Binding>,
    mut inVectorize: bool,
    mut splicedExpData: &InstTypes::SplicedExpData,
    mut inPrefix: &DAE::Prefix,
    mut evalCref: bool,
    mut info: &SourceInfo,
) -> Result<(
    FCore::Cache,
    metamodelica::Ref<DAE::Exp>,
    DAE::Const,
    metamodelica::Ref<DAE::Attributes>,
)> {
    let mut outCache: FCore::Cache = inCache;
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    let mut outConst: DAE::Const;
    let mut outAttributes: metamodelica::Ref<DAE::Attributes>;
    let mut var: SCode::Variability = DAEUtil::getAttrVariability(inAttributes);
    (outExp, outConst, outAttributes) = 'mc: {
        let __mc_input = (var, &**inType, inBinding.clone(), splicedExpData);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Deref @ DAE::Type::T_UNKNOWN { .. }, _, _) => {
                    let mut expTy: metamodelica::Ref<DAE::Type>;
                    let mut r#const: DAE::Const;
                    expTy = Types::simplifyType(inType.clone())?;
                    r#const = Types::variabilityToConst(var);
                    Ok((metamodelica::Ref::new(DAE::Exp::CREF { componentRef: inCref.clone(), ty: expTy.clone() }), r#const, inAttributes.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (SCode::Variability::PARAM { .. }, _, Deref @ DAE::Binding::EQBOUND { source: DAE::BindingSource::BINDING_FROM_START_VALUE { .. }, .. }, _) => {
                    let mut e: metamodelica::Ref<DAE::Exp>;
                    let mut r#const: DAE::Const;
                    let mut binding: metamodelica::Ref<DAE::Binding>;
                    let mut attr: metamodelica::Ref<DAE::Attributes>;
                    let mut outCache: FCore::Cache = outCache.clone();
                    let true = (Types::getFixedVarAttributeParameterOrConstant(inType)) else { return Err("pattern mismatch") };
                    binding = DAEUtil::setBindingSource(inBinding.clone(), openmodelica_frontend_types::DAE::BindingSource::BINDING_FROM_DEFAULT_VALUE);
                    (outCache, e, r#const, attr) = elabCref2(outCache.clone(), inEnv, inCref, inAttributes, constSubs, inIteratorConst.clone(), inType, binding.clone(), inVectorize, splicedExpData, inPrefix, evalCref, info)?;
                    Ok(((e.clone(), r#const, attr.clone()), outCache.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            outCache = __wb0;
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (SCode::Variability::CONST { .. }, Deref @ DAE::Type::T_ENUMERATION { index: Some(i), path: p, .. }, _, _) => {
                    if !((evalCref)) { return Err("guard") }
                    let mut p = (*p).clone();
                    p = AbsynUtil::joinPaths(p.clone(), ComponentReference::crefLastPath(inCref)?)?;
                    Ok((metamodelica::Ref::new(DAE::Exp::ENUM_LITERAL { name: p.clone(), index: i.clone() }), openmodelica_frontend_types::DAE::Const::C_CONST, inAttributes.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (SCode::Variability::CONST { .. }, _, _, _) => {
                    if !((!(evalCref))) { return Err("guard") }
                    let mut expTy: metamodelica::Ref<DAE::Type>;
                    expTy = Types::simplifyType(inType.clone())?;
                    Ok((Expression::makeCrefExp(inCref.clone(), expTy.clone())?, openmodelica_frontend_types::DAE::Const::C_CONST, inAttributes.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                        (SCode::Variability::CONST { .. }, _, _, InstTypes::SplicedExpData { .. }) => {
                            if !((Types::isVar(constSubs))) { return Err("guard") }
                            let mut cr: metamodelica::Ref<DAE::ComponentRef>;
                            let mut e: metamodelica::Ref<DAE::Exp>;
                            let mut v: metamodelica::Ref<Values::Value>;
                            let mut subsc: metamodelica::List<metamodelica::Ref<DAE::Subscript>>;
                            let mut outCache: FCore::Cache = outCache.clone();
                            cr = ComponentReferenceBasics::crefStripLastSubs(inCref)?;
                            subsc = ComponentReference::crefLastSubs(inCref)?;
                            (outCache, v) = Ceval::cevalCref(outCache.clone(), inEnv.clone(), cr.clone(), false, Absyn::Msg::MSG { info: info.clone() }, 0)?;
                            e = ValuesUtil::valueExp(v.clone(), None)?;
                            e = Expression::makeASUB(e.clone(), ({
                let mut __acc: metamodelica::List<metamodelica::Ref<DAE::Exp>> = metamodelica::nil();
                for mut sub in (subsc.clone()).into_iter().cloned() {
                            let __x = Expression::getSubscriptExp(&(sub.clone()))?;
                            __acc = cons(__x, __acc);
                }
                __acc.reverse()
            }))?;
                            Ok(((e.clone(), openmodelica_frontend_types::DAE::Const::C_VAR, inAttributes.clone()), outCache.clone()))
                        }
                        _ => return Err("nomatch"),
                    }}
        })() {
            outCache = __wb0;
            break 'mc __v;
        }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                        (SCode::Variability::CONST { .. }, _, binding, InstTypes::SplicedExpData { splicedExp: _, identType: idTy }) => {
                            let mut e: metamodelica::Ref<DAE::Exp>;
                            let mut v: metamodelica::Ref<Values::Value>;
                            let mut r#const: DAE::Const;
                            let mut outCache: FCore::Cache = outCache.clone();
                            let true = (Types::equivtypes(inType.clone(), idTy.clone())) else { return Err("pattern mismatch") };
                            match '__try0: {
                                (outCache, v) = unwrap_break_err!(Ceval::cevalCrefBinding(outCache.clone(), inEnv.clone(), inCref.clone(), metamodelica::AsArg::as_arg(&binding), false, Absyn::Msg::MSG { info: info.clone() }, 0), '__try0);
                                e = unwrap_break_err!(ValuesUtil::valueExp(v.clone(), None), '__try0);
                                Ok::<_, &'static str>((e.clone(),))
                            } {
                                Ok((__try0_o0,)) => {
                                    e = __try0_o0;
                                }
                                Err(_) => {
                                    let __pa1 = ::match_deref::match_deref! { match &(DAEUtil::bindingExp(metamodelica::AsArg::as_arg(&binding))?) {
                                                Some(__pa1) => __pa1.clone(),
                                                _ => return Err("pattern mismatch"),
                                    } };
                                    e = metamodelica::Own::own(__pa1);
                                    e = Expression::makeASUB(e.clone(), ({
                let mut __acc: metamodelica::List<metamodelica::Ref<DAE::Exp>> = metamodelica::nil();
                for mut sub in (ComponentReference::crefLastSubs(inCref)?).into_iter().cloned() {
                            let __x = Expression::getSubscriptExp(&(sub.clone()))?;
                            __acc = cons(__x, __acc);
                }
                __acc.reverse()
            }))?;
                                }
                            }
                            r#const = openmodelica_frontend_types::DAE::Const::C_CONST;
                            Ok(((e.clone(), r#const, inAttributes.clone()), outCache.clone()))
                        }
                        _ => return Err("nomatch"),
                    }}
        })() {
            outCache = __wb0;
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (SCode::Variability::CONST { .. }, _, _, _) => {
                    if !(((inIteratorConst).is_some())) { return Err("guard") }
                    let mut expTy: metamodelica::Ref<DAE::Type>;
                    expTy = Types::simplifyType(inType.clone())?;
                    Ok((Expression::makeCrefExp(inCref.clone(), expTy.clone())?, openmodelica_frontend_types::DAE::Const::C_CONST, inAttributes.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (SCode::Variability::CONST { .. }, _, Deref @ DAE::Binding::EQBOUND { constant_: DAE::Const::C_CONST { .. }, .. }, InstTypes::SplicedExpData { splicedExp: sexp, identType: idTy }) => {
                    let mut expTy: metamodelica::Ref<DAE::Type>;
                    let mut expIdTy: metamodelica::Ref<DAE::Type>;
                    let mut cr: metamodelica::Ref<DAE::ComponentRef>;
                    let mut e: metamodelica::Ref<DAE::Exp>;
                    let mut v: metamodelica::Ref<Values::Value>;
                    let mut outCache: FCore::Cache = outCache.clone();
                    expTy = Types::simplifyType(inType.clone())?;
                    expIdTy = Types::simplifyType(idTy.clone())?;
                    cr = fillCrefSubscripts(inCref.clone(), inType.clone())?;
                    e = Expression::makeCrefExp(cr.clone(), expTy.clone())?;
                    e = crefVectorize(inVectorize, e.clone(), inType, sexp.clone(), &expIdTy);
                    (outCache, v) = Ceval::ceval(outCache.clone(), inEnv.clone(), e.clone(), false, Absyn::Msg::MSG { info: info.clone() }, 0)?;
                    e = ValuesUtil::valueExp(v.clone(), Some(e.clone()))?;
                    Ok(((e.clone(), openmodelica_frontend_types::DAE::Const::C_CONST, inAttributes.clone()), outCache.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            outCache = __wb0;
            break 'mc __v;
        }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (SCode::Variability::PARAM { .. }, _, _, InstTypes::SplicedExpData { splicedExp: sexp, identType: idTy }) => {
                    if !((DAEUtil::isBound(&inBinding))) { return Err("guard") }
                    let mut expTy: metamodelica::Ref<DAE::Type>;
                    let mut expIdTy: metamodelica::Ref<DAE::Type>;
                    let mut cr: metamodelica::Ref<DAE::ComponentRef>;
                    let mut e: metamodelica::Ref<DAE::Exp>;
                    let mut v: metamodelica::Ref<Values::Value>;
                    let mut attr: metamodelica::Ref<DAE::Attributes>;
                    let mut outCache: FCore::Cache = outCache.clone();
                    let true = (Flags::isSet(Flags::EVAL_PARAM.clone())? || Config::getEvaluateParametersInAnnotations()?) else { return Err("pattern mismatch") };
                    attr = DAEUtil::setAttrVariability(inAttributes.clone(), openmodelica_frontend_types::SCode::Variability::CONST);
                    expTy = Types::simplifyType(inType.clone())?;
                    expIdTy = Types::simplifyType(idTy.clone())?;
                    cr = fillCrefSubscripts(inCref.clone(), inType.clone())?;
                    e = crefVectorize(inVectorize, Expression::makeCrefExp(cr.clone(), expTy.clone())?, inType, sexp.clone(), &expIdTy);
                    (outCache, v) = Ceval::ceval(outCache.clone(), inEnv.clone(), e.clone(), false, Absyn::Msg::MSG { info: info.clone() }, 0)?;
                    e = ValuesUtil::valueExp(v.clone(), Some(e.clone()))?;
                    Ok(((e.clone(), openmodelica_frontend_types::DAE::Const::C_PARAM, attr.clone()), outCache.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            outCache = __wb0;
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (SCode::Variability::CONST { .. }, _, Deref @ DAE::Binding::EQBOUND { evaluatedExp: Some(v), constant_: DAE::Const::C_CONST { .. }, .. }, InstTypes::SplicedExpData { splicedExp: Some(Deref @ DAE::Exp::CREF { componentRef: cr, .. }), identType: _ }) => {
                    let mut subCr1: metamodelica::Ref<DAE::ComponentRef>;
                    let mut subCr2: metamodelica::Ref<DAE::ComponentRef>;
                    let mut e: metamodelica::Ref<DAE::Exp>;
                    let mut index: metamodelica::Ref<DAE::Exp>;
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(ComponentReference::crefLastSubs(metamodelica::AsArg::as_arg(&cr))?) {
                        Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Subscript::INDEX { exp: Deref @ DAE::Exp::CREF { componentRef: __pa0, .. } }, tail: Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Subscript::SLICE { exp: __pa1 }, tail: Deref @ metamodelica::ListNode::Nil } } => (__pa0.clone(), __pa1.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    subCr2 = metamodelica::Own::own(__pa0);
                    e = metamodelica::Own::own(__pa1);
                    let (__pa5, __pa4) = ::match_deref::match_deref! { match &(ComponentReference::crefLastSubs(inCref)?) {
                        Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Subscript::INDEX { exp: __pa5 @ Deref @ DAE::Exp::CREF { componentRef: __pa4, .. } }, tail: Deref @ metamodelica::ListNode::Nil } => (__pa5.clone(), __pa4.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    subCr1 = metamodelica::Own::own(__pa4);
                    index = metamodelica::Own::own(__pa5);
                    let true = (ComponentReferenceBasics::crefEqual(&subCr1, &subCr2)?) else { return Err("pattern mismatch") };
                    let true = (Expression::isArray(&e) || Expression::isRange(&e)) else { return Err("pattern mismatch") };
                    e = ValuesUtil::valueExp(v.clone(), Some(e.clone()))?;
                    e = metamodelica::Ref::new(DAE::Exp::ASUB { exp: e.clone(), sub: list![metamodelica::Ref::new(DAE::Subscript::INDEX { exp: index.clone() })] });
                    Ok((e.clone(), openmodelica_frontend_types::DAE::Const::C_CONST, inAttributes.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (SCode::Variability::CONST { .. }, _, Deref @ DAE::Binding::UNBOUND { .. }, _) => {
                    if !(((inIteratorConst).is_none())) { return Err("guard") }
                    let mut expTy: metamodelica::Ref<DAE::Type>;
                    let mut cr: metamodelica::Ref<DAE::ComponentRef>;
                    let mut e: metamodelica::Ref<DAE::Exp>;
                    let mut s: ArcStr;
                    let mut scope: ArcStr;
                    let mut pre_str: ArcStr;
                    if Flags::isSet(Flags::STATIC.clone())? {
                        s = ComponentReferenceBasics::printComponentRefStr(inCref)?;
                        scope = FGraph::printGraphPathStr(inEnv);
                        pre_str = PrefixUtil::printPrefixStr2(inPrefix.clone())?;
                        s = { let mut __mm_s = String::new(); __mm_s.push_str(&*pre_str); __mm_s.push_str(&*s); ArcStr::from(__mm_s) };
                        Debug::traceln({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("- Static.elabCref2 failed on: ")); __mm_s.push_str(&*pre_str); __mm_s.push_str(&*s); __mm_s.push_str(&*literal!(" with no constant binding in scope: ")); __mm_s.push_str(&*scope); ArcStr::from(__mm_s) })?;
                    }
                    expTy = Types::simplifyType(inType.clone())?;
                    cr = fillCrefSubscripts(inCref.clone(), inType.clone())?;
                    e = Expression::makeCrefExp(cr.clone(), expTy.clone())?;
                    Ok((e.clone(), openmodelica_frontend_types::DAE::Const::C_CONST, inAttributes.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, _, InstTypes::SplicedExpData { splicedExp: sexp, identType: idTy }) => {
                    let mut expTy: metamodelica::Ref<DAE::Type>;
                    let mut expIdTy: metamodelica::Ref<DAE::Type>;
                    let mut cr: metamodelica::Ref<DAE::ComponentRef>;
                    let mut e: metamodelica::Ref<DAE::Exp>;
                    let mut r#const: DAE::Const;
                    expTy = Types::simplifyType(inType.clone())?;
                    expIdTy = Types::simplifyType(idTy.clone())?;
                    cr = fillCrefSubscripts(inCref.clone(), inType.clone())?;
                    e = crefVectorize(inVectorize, Expression::makeCrefExp(cr.clone(), expTy.clone())?, inType, sexp.clone(), &expIdTy);
                    r#const = Types::variabilityToConst(var);
                    Ok((e.clone(), r#const, inAttributes.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut pre_str: ArcStr;
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { return Err("pattern mismatch") };
                    pre_str = PrefixUtil::printPrefixStr2(inPrefix.clone())?;
                    Debug::traceln({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("- Static.elabCref2 failed for: ")); __mm_s.push_str(&*pre_str); __mm_s.push_str(&*ComponentReferenceBasics::printComponentRefStr(inCref)?); __mm_s.push_str(&*literal!("\n env:")); __mm_s.push_str(&*FGraph::printGraphStr(inEnv)); ArcStr::from(__mm_s) })?;
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((outCache, outExp, outConst, outAttributes))
}

pub fn crefVectorize(
    mut performVectorization: bool,
    mut inExp: metamodelica::Ref<DAE::Exp>,
    mut inType: &metamodelica::Ref<DAE::Type>,
    mut splicedExp: Option<metamodelica::Ref<DAE::Exp>>,
    mut crefIdType: &metamodelica::Ref<DAE::Type>,
) -> metamodelica::Ref<DAE::Exp> {
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    outExp = 'mc: {
        let __mc_input = (performVectorization, inExp.clone(), &**inType, splicedExp);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (false, e, _, _) => {
                    Ok(e.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, e, Deref @ DAE::Type::T_SUBTYPE_BASIC { complexType: t, .. }, _) => {
                    let mut e = (*e).clone();
                    e = crefVectorize(true, e.clone(), metamodelica::AsArg::as_arg(&t), None, crefIdType);
                    Ok(e.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, Deref @ DAE::Type::T_ARRAY { dims: Deref @ metamodelica::ListNode::Cons { head: d1, tail: Deref @ metamodelica::ListNode::Nil }, ty: Deref @ DAE::Type::T_ARRAY { dims: Deref @ metamodelica::ListNode::Cons { head: d2, tail: Deref @ metamodelica::ListNode::Nil }, .. } }, Some(Deref @ DAE::Exp::CREF { componentRef: cr, .. })) => {
                    let mut b1: bool;
                    let mut b2: bool;
                    let mut e: metamodelica::Ref<DAE::Exp>;
                    b1 = Expression::dimensionSize(metamodelica::AsArg::as_arg(&d1))? < Config::vectorizationLimit()?;
                    b2 = Expression::dimensionSize(metamodelica::AsArg::as_arg(&d2))? < Config::vectorizationLimit()?;
                    let true = (boolAnd(b1, b2) || Config::vectorizationLimit()? == 0) else { return Err("pattern mismatch") };
                    e = elabCrefSlice(metamodelica::AsArg::as_arg(&cr), crefIdType.clone())?;
                    e = elabMatrixToMatrixExp(e.clone());
                    Ok(e.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, Deref @ DAE::Type::T_ARRAY { dims: Deref @ metamodelica::ListNode::Cons { head: d1, tail: Deref @ metamodelica::ListNode::Nil }, ty: t }, Some(Deref @ DAE::Exp::CREF { componentRef: cr, .. })) => {
                    let mut e: metamodelica::Ref<DAE::Exp>;
                    let false = (Types::isArray(metamodelica::AsArg::as_arg(&t))) else { return Err("pattern mismatch") };
                    let true = (Expression::dimensionSize(metamodelica::AsArg::as_arg(&d1))? < Config::vectorizationLimit()? || Config::vectorizationLimit()? == 0) else { return Err("pattern mismatch") };
                    e = elabCrefSlice(metamodelica::AsArg::as_arg(&cr), crefIdType.clone())?;
                    Ok(e.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Deref @ DAE::Exp::CREF { componentRef: cr, ty: exptp }, Deref @ DAE::Type::T_ARRAY { dims: Deref @ metamodelica::ListNode::Cons { head: d1, tail: Deref @ metamodelica::ListNode::Nil }, ty: t @ Deref @ DAE::Type::T_ARRAY { dims: Deref @ metamodelica::ListNode::Cons { head: d2, tail: Deref @ metamodelica::ListNode::Nil }, .. } }, _) => {
                    let mut b1: bool;
                    let mut b2: bool;
                    let mut e: metamodelica::Ref<DAE::Exp>;
                    let mut ds: i32;
                    let mut ds2: i32;
                    ds = Expression::dimensionSize(metamodelica::AsArg::as_arg(&d1))?;
                    ds2 = Expression::dimensionSize(metamodelica::AsArg::as_arg(&d2))?;
                    b1 = ds < Config::vectorizationLimit()?;
                    b2 = ds2 < Config::vectorizationLimit()?;
                    let true = (boolAnd(b1, b2) || Config::vectorizationLimit()? == 0) else { return Err("pattern mismatch") };
                    let true = (((ComponentReference::crefLastSubs(metamodelica::AsArg::as_arg(&cr))?)).is_empty()) else { return Err("pattern mismatch") };
                    e = createCrefArray2d(cr.clone(), 1, ds, ds2, exptp.clone(), t.clone(), crefIdType)?;
                    Ok(e.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Deref @ DAE::Exp::CREF { componentRef: cr, ty: exptp }, Deref @ DAE::Type::T_ARRAY { dims: Deref @ metamodelica::ListNode::Cons { head: d1, tail: Deref @ metamodelica::ListNode::Nil }, ty: t }, _) => {
                    let mut e: metamodelica::Ref<DAE::Exp>;
                    let mut ds: i32;
                    let false = (Types::isArray(metamodelica::AsArg::as_arg(&t))) else { return Err("pattern mismatch") };
                    ds = Expression::dimensionSize(metamodelica::AsArg::as_arg(&d1))?;
                    let true = (ds < Config::vectorizationLimit()? || Config::vectorizationLimit()? == 0) else { return Err("pattern mismatch") };
                    e = createCrefArray(cr.clone(), 1, ds, exptp.clone(), t.clone(), crefIdType)?;
                    Ok(e.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(inExp.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    outExp
}

fn extractDimensionOfChild(
    mut inExp: &metamodelica::Ref<DAE::Exp>,
) -> Result<(metamodelica::List<metamodelica::Ref<DAE::Dimension>>, bool)> {
    let mut outExp: metamodelica::List<metamodelica::Ref<DAE::Dimension>>;
    let mut isScalar: bool;
    (outExp, isScalar) = 'mc: {
        let __mc_input = &**inExp;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::ARRAY { ty: Deref @ DAE::Type::T_ARRAY { dims: tl, .. }, scalar: sc, .. } => {
                    Ok((tl.clone(), sc.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::ARRAY { array: expl1 @ Deref @ metamodelica::ListNode::Cons { head: exp2 @ Deref @ DAE::Exp::ARRAY { ty: _, scalar: _, array: _ }, tail: _ }, .. } => {
                    let mut tl: metamodelica::List<metamodelica::Ref<DAE::Dimension>>;
                    let mut x: i32;
                    (tl, _) = extractDimensionOfChild(metamodelica::AsArg::as_arg(&exp2))?;
                    x = ((expl1).len() as i32);
                    Ok((metamodelica::cons(metamodelica::Ref::new(DAE::Dimension::DIM_INTEGER { integer: x }), tl.clone()), false))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::ARRAY { array: expl1, .. } => {
                    let mut x: i32;
                    x = ((expl1).len() as i32);
                    Ok((list![metamodelica::Ref::new(DAE::Dimension::DIM_INTEGER { integer: x })], true))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::CREF { componentRef: _, ty: _ } => {
                    Ok((metamodelica::nil(), true))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((outExp, isScalar))
}

fn elabCrefSlice(
    mut inCref: &metamodelica::Ref<DAE::ComponentRef>,
    mut inType: metamodelica::Ref<DAE::Type>,
) -> Result<metamodelica::Ref<DAE::Exp>> {
    let mut outCref: metamodelica::Ref<DAE::Exp>;
    outCref = (match &**inCref {
        DAE::ComponentRef::CREF_IDENT {
            ident: id,
            subscriptLst: ssl,
            ..
        } => {
            let mut ety = inType;
            let mut exp1: metamodelica::Ref<DAE::Exp>;
            exp1 = flattenSubscript(ssl.clone(), id.clone(), ety)?;
            exp1
        }
        DAE::ComponentRef::CREF_QUAL {
            ident: id,
            identType: prety,
            subscriptLst: ssl,
            componentRef: child,
        } => {
            let mut ety = inType;
            let mut exp1: metamodelica::Ref<DAE::Exp>;
            let mut childExp: metamodelica::Ref<DAE::Exp>;
            childExp = elabCrefSlice(child, ety.clone())?;
            exp1 = flattenSubscript(ssl.clone(), id.clone(), prety.clone())?;
            exp1 = mergeQualWithRest(exp1, childExp, ety)?;
            exp1
        }
        _ => return Err("match: no arm matched"),
    });
    Ok(outCref)
}

fn mergeQualWithRest(
    mut qual: metamodelica::Ref<DAE::Exp>,
    mut rest: metamodelica::Ref<DAE::Exp>,
    mut inType: metamodelica::Ref<DAE::Type>,
) -> Result<metamodelica::Ref<DAE::Exp>> {
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    outExp = (::match_deref::match_deref! { match &(qual) {
        exp1 @ Deref @ DAE::Exp::CREF { componentRef: _, ty: _ } => {
            let mut exp2 = rest;
            mergeQualWithRest2(exp2, exp1.clone())?
        },
        Deref @ DAE::Exp::ARRAY { ty: _, scalar: _, array: expl1 } => {
            let mut exp2 = rest;
            let mut ety = inType;
            let mut iLst: metamodelica::List<metamodelica::Ref<DAE::Dimension>>;
            let mut scalar: bool;
            let mut expl1 = (*expl1).clone();
            expl1 = List::map2(expl1.clone(), &mergeQualWithRest, exp2, ety.clone())?;
            exp2 = metamodelica::Ref::new(DAE::Exp::ARRAY { ty: DAE::T_INTEGER_DEFAULT().clone(), scalar: false, array: expl1.clone() });
            (iLst, scalar) = extractDimensionOfChild(&exp2)?;
            ety = Expression::arrayEltType(&ety);
            exp2 = metamodelica::Ref::new(DAE::Exp::ARRAY { ty: metamodelica::Ref::new(DAE::Type::T_ARRAY { ty: ety, dims: iLst }), scalar: scalar, array: expl1.clone() });
            exp2
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok(outExp)
}

fn mergeQualWithRest2(
    mut rest: metamodelica::Ref<DAE::Exp>,
    mut qual: metamodelica::Ref<DAE::Exp>,
) -> Result<metamodelica::Ref<DAE::Exp>> {
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    outExp = (::match_deref::match_deref! { match &((rest, qual)) {
        (Deref @ DAE::Exp::CREF { componentRef: cref, ty: ety }, Deref @ DAE::Exp::CREF { componentRef: Deref @ DAE::ComponentRef::CREF_IDENT { ident: id, identType: ty2, subscriptLst: ssl }, ty: _ }) => {
            let mut cref_2: metamodelica::Ref<DAE::ComponentRef>;
            cref_2 = ComponentReferenceBasics::makeCrefQual(id.clone(), ty2.clone(), ssl.clone(), cref.clone());
            Expression::makeCrefExp(cref_2, ety.clone())?
        },
        (exp1 @ Deref @ DAE::Exp::ARRAY { ty: ety, scalar: _, array: expl1 }, exp2 @ Deref @ DAE::Exp::CREF { componentRef: Deref @ DAE::ComponentRef::CREF_IDENT { ident: _, identType: _, subscriptLst: _ }, ty: _ }) => {
            let mut scalar: bool;
            let mut exp1 = (*exp1).clone();
            let mut expl1 = (*expl1).clone();
            expl1 = List::map1(expl1.clone(), &mergeQualWithRest2, exp2.clone())?;
            exp1 = metamodelica::Ref::new(DAE::Exp::ARRAY { ty: DAE::T_INTEGER_DEFAULT().clone(), scalar: false, array: expl1.clone() });
            (_, scalar) = extractDimensionOfChild(metamodelica::AsArg::as_arg(&exp1))?;
            metamodelica::Ref::new(DAE::Exp::ARRAY { ty: ety.clone(), scalar: scalar, array: expl1.clone() })
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok(outExp)
}

fn flattenSubscript(
    mut inSubs: metamodelica::List<metamodelica::Ref<DAE::Subscript>>,
    mut name: ArcStr,
    mut inType: metamodelica::Ref<DAE::Type>,
) -> Result<metamodelica::Ref<DAE::Exp>> {
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    outExp = 'mc: {
        let __mc_input = (inSubs, name, inType);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Nil, id, ety) => {
                    let mut exp1: metamodelica::Ref<DAE::Exp>;
                    let mut cref_: metamodelica::Ref<DAE::ComponentRef>;
                    cref_ = ComponentReferenceBasics::makeCrefIdent(id.clone(), ety.clone(), metamodelica::nil());
                    exp1 = Expression::makeCrefExp(cref_.clone(), ety.clone())?;
                    Ok(exp1.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (subs1, id, ety) => {
                    let mut exp2: metamodelica::Ref<DAE::Exp>;
                    exp2 = flattenSubscript2(metamodelica::AsArg::as_arg(&subs1), id.clone(), ety.clone())?;
                    Ok(exp2.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(outExp)
}

// BZ(2010-01-29): Changed to public to be able to vectorize crefs from other places
pub(crate) fn flattenSubscript2(
    mut inSubs: &metamodelica::List<metamodelica::Ref<DAE::Subscript>>,
    mut name: ArcStr,
    mut inType: metamodelica::Ref<DAE::Type>,
) -> Result<metamodelica::Ref<DAE::Exp>> {
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    outExp = 'mc: {
        let __mc_input = (&**inSubs, name, inType);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Nil, _, _) => {
                    Ok(metamodelica::Ref::new(DAE::Exp::ARRAY { ty: DAE::T_UNKNOWN_DEFAULT().clone(), scalar: false, array: metamodelica::nil() }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Subscript::INDEX { exp: exp1 @ Deref @ DAE::Exp::ICONST { integer: _ } }, tail: subs1 }, id, ety) => {
                    let mut exp2: metamodelica::Ref<DAE::Exp>;
                    exp2 = flattenSubscript2(metamodelica::AsArg::as_arg(&subs1), id.clone(), ety.clone())?;
                    exp2 = applySubscript(exp1.clone(), exp2.clone(), id.clone(), Expression::unliftArray(metamodelica::AsArg::as_arg(&ety))?)?;
                    Ok(exp2.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Subscript::SLICE { exp: Deref @ DAE::Exp::ARRAY { ty: _, scalar: _, array: expl1 @ Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Exp::ICONST { integer: 0 }, tail: Deref @ metamodelica::ListNode::Nil } } }, tail: subs1 }, id, ety) => {
                    let mut expl2: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                    let mut exp2: metamodelica::Ref<DAE::Exp>;
                    let mut exp3: metamodelica::Ref<DAE::Exp>;
                    exp2 = flattenSubscript2(metamodelica::AsArg::as_arg(&subs1), id.clone(), ety.clone())?;
                    expl2 = List::map3(expl1.clone(), &applySubscript, exp2.clone(), id.clone(), ety.clone())?;
                    exp3 = (expl2).head().cloned()?;
                    Ok(exp3.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Subscript::SLICE { exp: Deref @ DAE::Exp::ARRAY { ty: _, scalar: _, array: expl1 } }, tail: subs1 }, id, ety) => {
                    let mut exp2: metamodelica::Ref<DAE::Exp>;
                    exp2 = flattenSubscript2(metamodelica::AsArg::as_arg(&subs1), id.clone(), ety.clone())?;
                    Ok(flattenSubscript3(expl1.clone(), id.clone(), ety.clone(), exp2.clone())?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: sub1 @ Deref @ DAE::Subscript::SLICE { exp: Deref @ DAE::Exp::RANGE { .. } }, tail: subs1 }, id, ety) => {
                    let mut expl1: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                    let mut exp2: metamodelica::Ref<DAE::Exp>;
                    expl1 = Expression::expandRange(var_field!((**sub1).exp, DAE::Subscript::SLICE))?;
                    exp2 = flattenSubscript2(metamodelica::AsArg::as_arg(&subs1), id.clone(), ety.clone())?;
                    Ok(flattenSubscript3(expl1.clone(), id.clone(), ety.clone(), exp2.clone())?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(outExp)
}

fn flattenSubscript3(
    mut inSubscripts: metamodelica::List<metamodelica::Ref<DAE::Exp>>,
    mut inName: ArcStr,
    mut inType: metamodelica::Ref<DAE::Type>,
    mut inExp: metamodelica::Ref<DAE::Exp>,
) -> Result<metamodelica::Ref<DAE::Exp>> {
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    let mut expl: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
    let mut dims: metamodelica::List<metamodelica::Ref<DAE::Dimension>>;
    let mut scalar: bool;
    let mut ty: metamodelica::Ref<DAE::Type>;
    expl = ({
        let mut __acc: metamodelica::List<metamodelica::Ref<DAE::Exp>> = metamodelica::nil();
        for mut e in (inSubscripts).into_iter().cloned() {
            let __x = applySubscript(e.clone(), inExp.clone(), inName.clone(), inType.clone())?;
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    outExp = metamodelica::Ref::new(DAE::Exp::ARRAY {
        ty: DAE::T_INTEGER_DEFAULT().clone(),
        scalar: false,
        array: expl.clone(),
    });
    (dims, scalar) = extractDimensionOfChild(&outExp)?;
    ty = Expression::arrayEltType(&inType);
    outExp = metamodelica::Ref::new(DAE::Exp::ARRAY {
        ty: metamodelica::Ref::new(DAE::Type::T_ARRAY { ty: ty, dims: dims }),
        scalar: scalar,
        array: expl,
    });
    Ok(outExp)
}

fn removeDoubleEmptyArrays(mut inArr: metamodelica::Ref<DAE::Exp>) -> metamodelica::Ref<DAE::Exp> {
    let mut outArr: metamodelica::Ref<DAE::Exp>;
    outArr = 'mc: {
        let __mc_input = inArr;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::ARRAY { array: Deref @ metamodelica::ListNode::Cons { head: exp2 @ Deref @ DAE::Exp::ARRAY { array: Deref @ metamodelica::ListNode::Nil, .. }, tail: Deref @ metamodelica::ListNode::Nil }, .. } => {
                    Ok(exp2.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::ARRAY { ty: ty1, scalar: sc, array: expl1 @ Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Exp::ARRAY { .. }, tail: expl3 } } => {
                    let mut exp1: metamodelica::Ref<DAE::Exp>;
                    let mut expl3 = (*expl3).clone();
                    expl3 = List::map(expl1.clone(), &fnptr!(removeDoubleEmptyArrays, metamodelica::Ref<DAE::Exp>))?;
                    exp1 = metamodelica::Ref::new(DAE::Exp::ARRAY { ty: ty1.clone(), scalar: sc.clone(), array: expl3.clone() });
                    Ok(exp1.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                exp1 => {
                    Ok(exp1.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                exp1 => {
                    metamodelica::print({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("- Static.removeDoubleEmptyArrays failure for: ")); __mm_s.push_str(&*ExpressionBasics::printExpStr(exp1.clone())?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) });
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    outArr
}

fn applySubscript(
    mut inSub: metamodelica::Ref<DAE::Exp>,
    mut inSubs: metamodelica::Ref<DAE::Exp>,
    mut name: ArcStr,
    mut inType: metamodelica::Ref<DAE::Type>,
) -> Result<metamodelica::Ref<DAE::Exp>> {
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    outExp = 'mc: {
        let __mc_input = (inSub, inSubs, name, inType);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, exp1 @ Deref @ DAE::Exp::ARRAY { ty: Deref @ DAE::Type::T_ARRAY { dims: arrDim, .. }, scalar: _, array: Deref @ metamodelica::ListNode::Nil }, _, _) => {
                    let true = (Expression::arrayContainZeroDimension(metamodelica::AsArg::as_arg(&arrDim))) else { return Err("pattern mismatch") };
                    Ok(exp1.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::ICONST { integer: 0 }, Deref @ DAE::Exp::ARRAY { ty: Deref @ DAE::Type::T_ARRAY { dims: arrDim, .. }, scalar: _, array: _ }, _, ety) => {
                    let mut ety = (*ety).clone();
                    ety = Expression::arrayEltType(metamodelica::AsArg::as_arg(&ety));
                    Ok(metamodelica::Ref::new(DAE::Exp::ARRAY { ty: metamodelica::Ref::new(DAE::Type::T_ARRAY { ty: ety.clone(), dims: metamodelica::cons(metamodelica::Ref::new(DAE::Dimension::DIM_INTEGER { integer: 0 }), arrDim.clone()) }), scalar: true, array: metamodelica::nil() }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::ICONST { integer: 0 }, _, _, ety) => {
                    let mut ety = (*ety).clone();
                    ety = Expression::arrayEltType(metamodelica::AsArg::as_arg(&ety));
                    Ok(metamodelica::Ref::new(DAE::Exp::ARRAY { ty: metamodelica::Ref::new(DAE::Type::T_ARRAY { ty: ety.clone(), dims: list![metamodelica::Ref::new(DAE::Dimension::DIM_INTEGER { integer: 0 })] }), scalar: true, array: metamodelica::nil() }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (exp1, Deref @ DAE::Exp::ARRAY { ty: _, scalar: _, array: Deref @ metamodelica::ListNode::Nil }, id, ety) => {
                    let mut crty: metamodelica::Ref<DAE::Type>;
                    let mut cref_: metamodelica::Ref<DAE::ComponentRef>;
                    let true = (Expression::isValidSubscript(metamodelica::AsArg::as_arg(&exp1))) else { return Err("pattern mismatch") };
                    crty = Expression::unliftArray(metamodelica::AsArg::as_arg(&ety))?;
                    cref_ = ComponentReferenceBasics::makeCrefIdent(id.clone(), ety.clone(), list![metamodelica::Ref::new(DAE::Subscript::INDEX { exp: exp1.clone() })]);
                    Ok(Expression::makeCrefExp(cref_.clone(), crty.clone())?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (exp1, exp2, _, ety) => {
                    let true = (Expression::isValidSubscript(metamodelica::AsArg::as_arg(&exp1))) else { return Err("pattern mismatch") };
                    Ok(applySubscript2(exp1.clone(), metamodelica::AsArg::as_arg(&exp2), ety.clone())?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(outExp)
}

fn applySubscript2(
    mut inSub: metamodelica::Ref<DAE::Exp>,
    mut inSubs: &metamodelica::Ref<DAE::Exp>,
    mut inType: metamodelica::Ref<DAE::Type>,
) -> Result<metamodelica::Ref<DAE::Exp>> {
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    outExp = (::match_deref::match_deref! { match inSubs {
        Deref @ DAE::Exp::CREF { componentRef: Deref @ DAE::ComponentRef::CREF_IDENT { ident: id, identType: ty2, subscriptLst: subs }, ty: _ } => {
            let mut exp1 = inSub;
            let mut exp2: metamodelica::Ref<DAE::Exp>;
            let mut crty: metamodelica::Ref<DAE::Type>;
            let mut cref_: metamodelica::Ref<DAE::ComponentRef>;
            crty = Expression::unliftArrayTypeWithSubs(metamodelica::cons(metamodelica::Ref::new(DAE::Subscript::INDEX { exp: exp1.clone() }), subs.clone()), ty2.clone())?;
            cref_ = ComponentReferenceBasics::makeCrefIdent(id.clone(), ty2.clone(), metamodelica::cons(metamodelica::Ref::new(DAE::Subscript::INDEX { exp: exp1 }), subs.clone()));
            exp2 = Expression::makeCrefExp(cref_, crty)?;
            exp2
        },
        Deref @ DAE::Exp::ARRAY { ty: _, scalar: _, array: expl1 } => {
            let mut exp1 = inSub;
            let mut ety = inType;
            let mut exp2: metamodelica::Ref<DAE::Exp>;
            let mut iLst: metamodelica::List<metamodelica::Ref<DAE::Dimension>>;
            let mut scalar: bool;
            let mut expl1 = (*expl1).clone();
            expl1 = List::map2(expl1.clone(), &move |__a0: metamodelica::Ref<DAE::Exp>, __a1: metamodelica::Ref<DAE::Exp>, __a2: metamodelica::Ref<DAE::Type>| applySubscript3(&__a0, __a1, __a2), exp1, ety.clone())?;
            exp2 = metamodelica::Ref::new(DAE::Exp::ARRAY { ty: DAE::T_INTEGER_DEFAULT().clone(), scalar: false, array: expl1.clone() });
            (iLst, scalar) = extractDimensionOfChild(&exp2)?;
            ety = Expression::arrayEltType(&ety);
            exp2 = metamodelica::Ref::new(DAE::Exp::ARRAY { ty: metamodelica::Ref::new(DAE::Type::T_ARRAY { ty: ety, dims: iLst }), scalar: scalar, array: expl1.clone() });
            exp2
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok(outExp)
}

fn applySubscript3(
    mut inSubs: &metamodelica::Ref<DAE::Exp>,
    mut inSub: metamodelica::Ref<DAE::Exp>,
    mut inType: metamodelica::Ref<DAE::Type>,
) -> Result<metamodelica::Ref<DAE::Exp>> {
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    outExp = (::match_deref::match_deref! { match inSubs {
        Deref @ DAE::Exp::CREF { componentRef: Deref @ DAE::ComponentRef::CREF_IDENT { ident: id, identType: ty2, subscriptLst: subs }, ty: _ } => {
            let mut exp1 = inSub;
            let mut exp2: metamodelica::Ref<DAE::Exp>;
            let mut crty: metamodelica::Ref<DAE::Type>;
            let mut cref_: metamodelica::Ref<DAE::ComponentRef>;
            crty = Expression::unliftArrayTypeWithSubs(metamodelica::cons(metamodelica::Ref::new(DAE::Subscript::INDEX { exp: exp1.clone() }), subs.clone()), ty2.clone())?;
            cref_ = ComponentReferenceBasics::makeCrefIdent(id.clone(), ty2.clone(), metamodelica::cons(metamodelica::Ref::new(DAE::Subscript::INDEX { exp: exp1 }), subs.clone()));
            exp2 = Expression::makeCrefExp(cref_, crty)?;
            exp2
        },
        Deref @ DAE::Exp::ARRAY { ty: _, scalar: _, array: expl1 } => {
            let mut exp1 = inSub;
            let mut ety = inType;
            let mut exp2: metamodelica::Ref<DAE::Exp>;
            let mut iLst: metamodelica::List<metamodelica::Ref<DAE::Dimension>>;
            let mut scalar: bool;
            let mut expl1 = (*expl1).clone();
            expl1 = List::map2(expl1.clone(), &move |__a0: metamodelica::Ref<DAE::Exp>, __a1: metamodelica::Ref<DAE::Exp>, __a2: metamodelica::Ref<DAE::Type>| applySubscript3(&__a0, __a1, __a2), exp1, ety.clone())?;
            exp2 = metamodelica::Ref::new(DAE::Exp::ARRAY { ty: DAE::T_INTEGER_DEFAULT().clone(), scalar: false, array: expl1.clone() });
            (iLst, scalar) = extractDimensionOfChild(&exp2)?;
            ety = Expression::arrayEltType(&ety);
            exp2 = metamodelica::Ref::new(DAE::Exp::ARRAY { ty: metamodelica::Ref::new(DAE::Type::T_ARRAY { ty: ety, dims: iLst }), scalar: scalar, array: expl1.clone() });
            exp2
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok(outExp)
}

fn callVectorize(
    mut inExp: metamodelica::Ref<DAE::Exp>,
    mut inExpExpLst: &metamodelica::List<metamodelica::Ref<DAE::Exp>>,
) -> Result<metamodelica::List<metamodelica::Ref<DAE::Exp>>> {
    let mut outExpExpLst: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
    outExpExpLst = 'mc: {
        let __mc_input = (inExp, &**inExpExpLst);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Deref @ metamodelica::ListNode::Nil) => {
                    Ok(metamodelica::nil())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (callexp @ Deref @ DAE::Exp::CALL { path: r#fn, expLst: args, attr }, Deref @ metamodelica::ListNode::Cons { head: e, tail: es }) => {
                    let mut es_1: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                    es_1 = callVectorize(callexp.clone(), metamodelica::AsArg::as_arg(&es))?;
                    Ok(metamodelica::cons(metamodelica::Ref::new(DAE::Exp::CALL { path: r#fn.clone(), expLst: metamodelica::cons(e.clone(), args.clone()), attr: attr.clone() }), es_1.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { return Err("pattern mismatch") };
                    Debug::trace(literal!("- Static.callVectorize failed\n"))?;
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(outExpExpLst)
}

fn createCrefArray(
    mut inComponentRef1: metamodelica::Ref<DAE::ComponentRef>,
    mut inInteger2: i32,
    mut inInteger3: i32,
    mut inType4: metamodelica::Ref<DAE::Type>,
    mut inType5: metamodelica::Ref<DAE::Type>,
    mut crefIdType: &metamodelica::Ref<DAE::Type>,
) -> Result<metamodelica::Ref<DAE::Exp>> {
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    outExp = 'mc: {
        let __mc_input = (inComponentRef1, inInteger2, inInteger3, inType4, inType5);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, indx, ds, et, _) => {
                    if !((indx.clone() > ds.clone())) { return Err("guard") }
                    Ok(metamodelica::Ref::new(DAE::Exp::ARRAY { ty: et.clone(), scalar: true, array: metamodelica::nil() }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cr, indx, ds, et, t) => {
                    let mut cr_1: metamodelica::Ref<DAE::ComponentRef>;
                    let mut indx_1: i32;
                    let mut elt_tp: metamodelica::Ref<DAE::Type>;
                    let mut expl: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                    let mut e_1: metamodelica::Ref<DAE::Exp>;
                    indx_1 = indx.clone() + 1;
                    cr_1 = ComponentReference::replaceWholeDimSubscript(metamodelica::AsArg::as_arg(&cr), indx.clone())?;
                    let __pa0 = ::match_deref::match_deref! { match &(createCrefArray(cr.clone(), indx_1, ds.clone(), et.clone(), t.clone(), crefIdType)?) {
                        Deref @ DAE::Exp::ARRAY { ty: _, scalar: _, array: __pa0 } => __pa0.clone(),
                        _ => return Err("pattern mismatch"),
                    } };
                    expl = metamodelica::Own::own(__pa0);
                    elt_tp = Expression::unliftArray(metamodelica::AsArg::as_arg(&et))?;
                    e_1 = crefVectorize(true, Expression::makeCrefExp(cr_1.clone(), elt_tp.clone())?, metamodelica::AsArg::as_arg(&t), None, crefIdType);
                    Ok(metamodelica::Ref::new(DAE::Exp::ARRAY { ty: et.clone(), scalar: true, array: metamodelica::cons(e_1.clone(), expl.clone()) }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cr, indx, ds, et, t) => {
                    let mut indx_1: i32;
                    let mut expl: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                    let mut e_1: metamodelica::Ref<DAE::Exp>;
                    indx_1 = indx.clone() + 1;
                    let __pa0 = ::match_deref::match_deref! { match &(createCrefArray(cr.clone(), indx_1, ds.clone(), et.clone(), t.clone(), crefIdType)?) {
                        Deref @ DAE::Exp::ARRAY { ty: _, scalar: _, array: __pa0 } => __pa0.clone(),
                        _ => return Err("pattern mismatch"),
                    } };
                    expl = metamodelica::Own::own(__pa0);
                    e_1 = Expression::makeASUB(Expression::makeCrefExp(cr.clone(), et.clone())?, list![metamodelica::Ref::new(DAE::Exp::ICONST { integer: indx.clone() })])?;
                    (e_1, _) = ExpressionSimplify::simplify(e_1.clone())?;
                    e_1 = crefVectorize(true, e_1.clone(), metamodelica::AsArg::as_arg(&t), None, crefIdType);
                    Ok(metamodelica::Ref::new(DAE::Exp::ARRAY { ty: et.clone(), scalar: true, array: metamodelica::cons(e_1.clone(), expl.clone()) }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cr, _, _, _, _) => {
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { return Err("pattern mismatch") };
                    Debug::trace({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("createCrefArray failed on:")); __mm_s.push_str(&*ComponentReferenceBasics::printComponentRefStr(metamodelica::AsArg::as_arg(&cr))?); ArcStr::from(__mm_s) })?;
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(outExp)
}

fn createCrefArray2d(
    mut inCref: metamodelica::Ref<DAE::ComponentRef>,
    mut inIndex: i32,
    mut inDim1: i32,
    mut inDim2: i32,
    mut inType5: metamodelica::Ref<DAE::Type>,
    mut inType6: metamodelica::Ref<DAE::Type>,
    mut crefIdType: &metamodelica::Ref<DAE::Type>,
) -> Result<metamodelica::Ref<DAE::Exp>> {
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    outExp = 'mc: {
        let __mc_input = (inCref, inIndex, inDim1, inDim2, inType5, inType6);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, indx, ds, _, et, _) => {
                    if !((indx.clone() > ds.clone())) { return Err("guard") }
                    Ok(metamodelica::Ref::new(DAE::Exp::MATRIX { ty: et.clone(), integer: 0, matrix: metamodelica::nil() }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cr, indx, ds, ds2, et, t) => {
                    let mut cr_1: metamodelica::Ref<DAE::ComponentRef>;
                    let mut indx_1: i32;
                    let mut elt_tp: metamodelica::Ref<DAE::Type>;
                    let mut ms: metamodelica::List<metamodelica::List<metamodelica::Ref<DAE::Exp>>>;
                    let mut expl: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                    indx_1 = indx.clone() + 1;
                    let __pa0 = ::match_deref::match_deref! { match &(createCrefArray2d(cr.clone(), indx_1, ds.clone(), ds2.clone(), et.clone(), t.clone(), crefIdType)?) {
                        Deref @ DAE::Exp::MATRIX { matrix: __pa0, .. } => __pa0.clone(),
                        _ => return Err("pattern mismatch"),
                    } };
                    ms = metamodelica::Own::own(__pa0);
                    cr_1 = ComponentReference::subscriptCref(metamodelica::AsArg::as_arg(&cr), list![metamodelica::Ref::new(DAE::Subscript::INDEX { exp: metamodelica::Ref::new(DAE::Exp::ICONST { integer: indx.clone() }) })])?;
                    elt_tp = Expression::unliftArray(metamodelica::AsArg::as_arg(&et))?;
                    let __pa1 = ::match_deref::match_deref! { match &(crefVectorize(true, Expression::makeCrefExp(cr_1.clone(), elt_tp.clone())?, metamodelica::AsArg::as_arg(&t), None, crefIdType)) {
                        Deref @ DAE::Exp::ARRAY { ty: _, scalar: true, array: __pa1 } => __pa1.clone(),
                        _ => return Err("pattern mismatch"),
                    } };
                    expl = metamodelica::Own::own(__pa1);
                    Ok(metamodelica::Ref::new(DAE::Exp::MATRIX { ty: et.clone(), integer: ds.clone(), matrix: metamodelica::cons(expl.clone(), ms.clone()) }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cr, _, _, _, _, _) => {
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { return Err("pattern mismatch") };
                    Debug::traceln({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("- Static.createCrefArray2d failed on: ")); __mm_s.push_str(&*ComponentReferenceBasics::printComponentRefStr(metamodelica::AsArg::as_arg(&cr))?); ArcStr::from(__mm_s) })?;
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(outExp)
}

pub(crate) fn absynCrefToComponentReference<'__b>(
    mut inComponentRef: &'__b metamodelica::Ref<Absyn::ComponentRef>,
) -> Result<metamodelica::Ref<DAE::ComponentRef>> {
    '__tco: loop {
        ::match_deref::match_deref! { match inComponentRef {
            Deref @ Absyn::ComponentRef::CREF_IDENT { name: i, subscripts: Deref @ metamodelica::ListNode::Nil } => {
                let mut cref: metamodelica::Ref<DAE::ComponentRef>;
                return Ok(ComponentReferenceBasics::makeCrefIdent(i.clone(), DAE::T_UNKNOWN_DEFAULT().clone(), metamodelica::nil()))
            },
            Deref @ Absyn::ComponentRef::CREF_QUAL { name: i, subscripts: Deref @ metamodelica::ListNode::Nil, componentRef: c } => {
                let mut cref: metamodelica::Ref<DAE::ComponentRef>;
                cref = absynCrefToComponentReference(c)?;
                return Ok(ComponentReferenceBasics::makeCrefQual(i.clone(), DAE::T_UNKNOWN_DEFAULT().clone(), metamodelica::nil(), cref))
            },
            Deref @ Absyn::ComponentRef::CREF_FULLYQUALIFIED { componentRef: c } => {
                let mut cref: metamodelica::Ref<DAE::ComponentRef>;
                { inComponentRef = c; continue '__tco; }
            },
            _ => return Err("match: no arm matched"),
        } }
    }
}

fn elabCrefSubs(
    mut inCache: FCore::Cache,
    mut inCrefEnv: FCore::Graph,
    mut inSubsEnv: FCore::Graph,
    mut inComponentRef: metamodelica::Ref<Absyn::ComponentRef>,
    mut inTopPrefix: DAE::Prefix,
    mut inCrefPrefix: DAE::Prefix,
    mut inBoolean: bool,
    mut inHasZeroSizeDim: bool,
    mut info: &SourceInfo,
) -> Result<(FCore::Cache, metamodelica::Ref<DAE::ComponentRef>, DAE::Const, bool)> {
    let mut outCache: FCore::Cache;
    let mut outComponentRef: metamodelica::Ref<DAE::ComponentRef>;
    let mut outConst: DAE::Const;
    let mut outHasZeroSizeDim: bool;
    (outCache, outComponentRef, outConst, outHasZeroSizeDim) = 'mc: {
        let __mc_input = (
            inCache,
            inCrefEnv,
            inSubsEnv,
            inComponentRef.clone(),
            inTopPrefix,
            inCrefPrefix,
            inBoolean,
            inHasZeroSizeDim,
        );
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, crefEnv, crefSubs, Deref @ Absyn::ComponentRef::CREF_IDENT { name: id, subscripts: ss }, topPrefix, crefPrefix, r#impl, hasZeroSizeDim) => {
                    let mut sl: metamodelica::List<metamodelica::Ref<DAE::Dimension>>;
                    let mut r#const: DAE::Const;
                    let mut cr: metamodelica::Ref<DAE::ComponentRef>;
                    let mut id_ty: metamodelica::Ref<DAE::Type>;
                    let mut ss_1: metamodelica::List<metamodelica::Ref<DAE::Subscript>>;
                    let mut cache = (*cache).clone();
                    let mut hasZeroSizeDim = (*hasZeroSizeDim).clone();
                    (cache, cr) = PrefixUtil::prefixCref(cache.clone(), crefEnv.clone(), &(InnerOuter::emptyInstHierarchy().clone()), crefPrefix.clone(), ComponentReferenceBasics::makeCrefIdent(id.clone(), DAE::T_UNKNOWN_DEFAULT().clone(), metamodelica::nil()))?;
                    let (__pa0, _, _, _, _, InstTypes::SPLICEDEXPDATA { identType: __pa1, .. }, _, _, _) = Lookup::lookupVar(cache.clone(), crefEnv.clone(), cr.clone())?;
                    cache = metamodelica::Own::own(__pa0);
                    id_ty = metamodelica::Own::own(__pa1);
                    id_ty = Types::simplifyType(id_ty.clone())?;
                    hasZeroSizeDim = Types::isZeroLengthArray(&id_ty)?;
                    sl = TypesDump::getDimensions(&id_ty);
                    (cache, ss_1, r#const) = elabSubscriptsDims(cache.clone(), crefSubs.clone(), metamodelica::AsArg::as_arg(&ss), sl.clone(), r#impl.clone(), topPrefix.clone(), &inComponentRef, info.clone())?;
                    Ok((cache.clone(), ComponentReferenceBasics::makeCrefIdent(id.clone(), id_ty.clone(), ss_1.clone()), r#const, hasZeroSizeDim.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, crefEnv, crefSubs, Deref @ Absyn::ComponentRef::CREF_QUAL { name: id, subscripts: Deref @ metamodelica::ListNode::Nil, componentRef: restCref }, topPrefix, crefPrefix, r#impl, hasZeroSizeDim) => {
                    let mut t: metamodelica::Ref<DAE::Type>;
                    let mut sl: metamodelica::List<metamodelica::Ref<DAE::Dimension>>;
                    let mut r#const: DAE::Const;
                    let mut cr: metamodelica::Ref<DAE::ComponentRef>;
                    let mut ty: metamodelica::Ref<DAE::Type>;
                    let mut cache = (*cache).clone();
                    let mut crefPrefix = (*crefPrefix).clone();
                    let mut hasZeroSizeDim = (*hasZeroSizeDim).clone();
                    (cache, cr) = PrefixUtil::prefixCref(cache.clone(), crefEnv.clone(), &(InnerOuter::emptyInstHierarchy().clone()), crefPrefix.clone(), ComponentReferenceBasics::makeCrefIdent(id.clone(), DAE::T_UNKNOWN_DEFAULT().clone(), metamodelica::nil()))?;
                    (cache, _, t, _, _, _, _, _, _) = Lookup::lookupVar(cache.clone(), crefEnv.clone(), cr.clone())?;
                    ty = Types::simplifyType(t.clone())?;
                    sl = TypesDump::getDimensions(&ty);
                    crefPrefix = PrefixUtil::prefixAdd(id.clone(), sl.clone(), metamodelica::nil(), metamodelica::AsArg::as_arg(&crefPrefix), openmodelica_frontend_types::SCode::Variability::VAR, ClassInf::State::UNKNOWN { path: metamodelica::Ref::new(Absyn::Path::IDENT { name: literal!("") }) }, info.clone())?;
                    (cache, cr, r#const, hasZeroSizeDim) = elabCrefSubs(cache.clone(), crefEnv.clone(), crefSubs.clone(), restCref.clone(), topPrefix.clone(), crefPrefix.clone(), r#impl.clone(), hasZeroSizeDim.clone(), info)?;
                    Ok((cache.clone(), ComponentReferenceBasics::makeCrefQual(id.clone(), ty.clone(), metamodelica::nil(), cr.clone()), r#const, hasZeroSizeDim.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, crefEnv, crefSubs, Deref @ Absyn::ComponentRef::CREF_QUAL { name: id, subscripts: Deref @ metamodelica::ListNode::Nil, componentRef: restCref }, topPrefix, crefPrefix, r#impl, hasZeroSizeDim) => {
                    let mut r#const: DAE::Const;
                    let mut cr: metamodelica::Ref<DAE::ComponentRef>;
                    let mut cache = (*cache).clone();
                    let mut crefPrefix = (*crefPrefix).clone();
                    let mut hasZeroSizeDim = (*hasZeroSizeDim).clone();
                    crefPrefix = PrefixUtil::prefixAdd(id.clone(), metamodelica::nil(), metamodelica::nil(), metamodelica::AsArg::as_arg(&crefPrefix), openmodelica_frontend_types::SCode::Variability::VAR, ClassInf::State::UNKNOWN { path: metamodelica::Ref::new(Absyn::Path::IDENT { name: literal!("") }) }, info.clone())?;
                    (cache, cr, r#const, hasZeroSizeDim) = elabCrefSubs(cache.clone(), crefEnv.clone(), crefSubs.clone(), restCref.clone(), topPrefix.clone(), crefPrefix.clone(), r#impl.clone(), hasZeroSizeDim.clone(), info)?;
                    Ok((cache.clone(), ComponentReferenceBasics::makeCrefQual(id.clone(), DAE::T_COMPLEX_DEFAULT().clone(), metamodelica::nil(), cr.clone()), r#const, hasZeroSizeDim.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, crefEnv, crefSubs, Deref @ Absyn::ComponentRef::CREF_QUAL { name: id, subscripts: ss @ Deref @ metamodelica::ListNode::Cons { head: _, tail: _ }, componentRef: restCref }, topPrefix, crefPrefix, r#impl, hasZeroSizeDim) => {
                    let mut t: metamodelica::Ref<DAE::Type>;
                    let mut sl: metamodelica::List<metamodelica::Ref<DAE::Dimension>>;
                    let mut r#const: DAE::Const;
                    let mut const1: DAE::Const;
                    let mut const2: DAE::Const;
                    let mut cr: metamodelica::Ref<DAE::ComponentRef>;
                    let mut ty: metamodelica::Ref<DAE::Type>;
                    let mut id_ty: metamodelica::Ref<DAE::Type>;
                    let mut ss_1: metamodelica::List<metamodelica::Ref<DAE::Subscript>>;
                    let mut vt: SCode::Variability;
                    let mut cache = (*cache).clone();
                    let mut crefPrefix = (*crefPrefix).clone();
                    let mut hasZeroSizeDim = (*hasZeroSizeDim).clone();
                    (cache, cr) = PrefixUtil::prefixCref(cache.clone(), crefEnv.clone(), &(InnerOuter::emptyInstHierarchy().clone()), crefPrefix.clone(), ComponentReferenceBasics::makeCrefIdent(id.clone(), DAE::T_UNKNOWN_DEFAULT().clone(), metamodelica::nil()))?;
                    let (__pa0, __t4, __pa2, _, _, InstTypes::SPLICEDEXPDATA { identType: __pa3, .. }, _, _, _) = Lookup::lookupVar(cache.clone(), crefEnv.clone(), cr.clone())?;
                    let __arc5 = __t4.clone();
                    let DAE::ATTR { variability: __pa1, .. } = &*__arc5;
                    cache = metamodelica::Own::own(__pa0);
                    vt = metamodelica::Own::own(__pa1);
                    t = metamodelica::Own::own(__pa2);
                    id_ty = metamodelica::Own::own(__pa3);
                    ty = Types::simplifyType(t.clone())?;
                    id_ty = Types::simplifyType(id_ty.clone())?;
                    sl = TypesDump::getDimensions(&id_ty);
                    (cache, ss_1, const1) = elabSubscriptsDims(cache.clone(), crefSubs.clone(), metamodelica::AsArg::as_arg(&ss), sl.clone(), r#impl.clone(), topPrefix.clone(), &inComponentRef, info.clone())?;
                    crefPrefix = PrefixUtil::prefixAdd(id.clone(), sl.clone(), ss_1.clone(), metamodelica::AsArg::as_arg(&crefPrefix), vt, ClassInf::State::UNKNOWN { path: metamodelica::Ref::new(Absyn::Path::IDENT { name: literal!("") }) }, info.clone())?;
                    (cache, cr, const2, hasZeroSizeDim) = elabCrefSubs(cache.clone(), crefEnv.clone(), crefSubs.clone(), restCref.clone(), topPrefix.clone(), crefPrefix.clone(), r#impl.clone(), hasZeroSizeDim.clone(), info)?;
                    r#const = Types::constAnd(const1, const2);
                    Ok((cache.clone(), ComponentReferenceBasics::makeCrefQual(id.clone(), ty.clone(), ss_1.clone(), cr.clone()), r#const, hasZeroSizeDim.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, crefEnv, crefSubs, Deref @ Absyn::ComponentRef::CREF_FULLYQUALIFIED { componentRef: absynCr }, topPrefix, crefPrefix, r#impl, hasZeroSizeDim) => {
                    let mut const1: DAE::Const;
                    let mut cr: metamodelica::Ref<DAE::ComponentRef>;
                    let mut cache = (*cache).clone();
                    let mut crefEnv = (*crefEnv).clone();
                    let mut hasZeroSizeDim = (*hasZeroSizeDim).clone();
                    crefEnv = FGraph::topScope(metamodelica::AsArg::as_arg(&crefEnv))?;
                    (cache, cr, const1, hasZeroSizeDim) = elabCrefSubs(cache.clone(), crefEnv.clone(), crefSubs.clone(), absynCr.clone(), topPrefix.clone(), crefPrefix.clone(), r#impl.clone(), hasZeroSizeDim.clone(), info)?;
                    Ok((cache.clone(), cr.clone(), const1, hasZeroSizeDim.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, crefEnv, _, absynCref, topPrefix, crefPrefix, _, _) => {
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { return Err("pattern mismatch") };
                    Debug::traceln({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("- Static.elabCrefSubs failed on: ")); __mm_s.push_str(&*literal!("[top:")); __mm_s.push_str(&*PrefixUtil::printPrefixStr(metamodelica::AsArg::as_arg(&topPrefix))?); __mm_s.push_str(&*literal!("].")); __mm_s.push_str(&*PrefixUtil::printPrefixStr(metamodelica::AsArg::as_arg(&crefPrefix))?); __mm_s.push_str(&*literal!(".")); __mm_s.push_str(&*Dump::printComponentRefStr(metamodelica::AsArg::as_arg(&absynCref))?); __mm_s.push_str(&*literal!(" env: ")); __mm_s.push_str(&*FGraph::printGraphPathStr(metamodelica::AsArg::as_arg(&crefEnv))); ArcStr::from(__mm_s) })?;
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((outCache, outComponentRef, outConst, outHasZeroSizeDim))
}

pub fn elabSubscripts(
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
    mut inAbsynSubscriptLst: &metamodelica::List<metamodelica::Ref<Absyn::Subscript>>,
    mut inBoolean: bool,
    mut inPrefix: DAE::Prefix,
    mut info: &SourceInfo,
) -> Result<(
    FCore::Cache,
    metamodelica::List<metamodelica::Ref<DAE::Subscript>>,
    DAE::Const,
)> {
    let mut outCache: FCore::Cache;
    let mut outExpSubscriptLst: metamodelica::List<metamodelica::Ref<DAE::Subscript>>;
    let mut outConst: DAE::Const;
    (outCache, outExpSubscriptLst, outConst) = (::match_deref::match_deref! { match inAbsynSubscriptLst {
        Deref @ metamodelica::ListNode::Nil => {
            let mut cache = inCache;
            (cache, metamodelica::nil(), openmodelica_frontend_types::DAE::Const::C_CONST)
        },
        Deref @ metamodelica::ListNode::Cons { head: sub, tail: subs } => {
            let mut cache = inCache;
            let mut env = inEnv;
            let mut r#impl = inBoolean;
            let mut pre = inPrefix;
            let mut sub_1: metamodelica::Ref<DAE::Subscript>;
            let mut const1: DAE::Const;
            let mut const2: DAE::Const;
            let mut r#const: DAE::Const;
            let mut subs_1: metamodelica::List<metamodelica::Ref<DAE::Subscript>>;
            (cache, sub_1, const1, _) = elabSubscript(cache, env.clone(), metamodelica::AsArg::as_arg(&sub), r#impl, pre.clone(), info.clone())?;
            (cache, subs_1, const2) = elabSubscripts(cache, env, subs, r#impl, pre, info)?;
            r#const = Types::constAnd(const1, const2);
            (cache, metamodelica::cons(sub_1, subs_1), r#const)
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outCache, outExpSubscriptLst, outConst))
}

fn elabSubscriptsDims(
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
    mut inSubscripts: &metamodelica::List<metamodelica::Ref<Absyn::Subscript>>,
    mut inDimensions: metamodelica::List<metamodelica::Ref<DAE::Dimension>>,
    mut inImpl: bool,
    mut inPrefix: DAE::Prefix,
    mut inCref: &metamodelica::Ref<Absyn::ComponentRef>,
    mut inInfo: SourceInfo,
) -> Result<(
    FCore::Cache,
    metamodelica::List<metamodelica::Ref<DAE::Subscript>>,
    DAE::Const,
)> {
    let mut outCache: FCore::Cache = inCache;
    let mut outSubs: metamodelica::List<metamodelica::Ref<DAE::Subscript>> = metamodelica::nil();
    let mut outConst: DAE::Const = openmodelica_frontend_types::DAE::Const::C_CONST;
    let mut rest_dims: metamodelica::List<metamodelica::Ref<DAE::Dimension>> = inDimensions.clone();
    let mut dim: metamodelica::Ref<DAE::Dimension>;
    let mut dsub: metamodelica::Ref<DAE::Subscript>;
    let mut r#const: DAE::Const;
    let mut prop: Option<DAE::Properties>;
    let mut subl_str: ArcStr;
    let mut diml_str: ArcStr;
    let mut cref_str: ArcStr;
    let mut nrdims: i32;
    let mut nrsubs: i32;
    for mut asub in &**inSubscripts {
        if (rest_dims).is_empty() {
            cref_str = Dump::printComponentRefStr(inCref)?;
            subl_str = intString(((inSubscripts).len() as i32));
            diml_str = intString(((inDimensions).len() as i32));
            Error::addSourceMessageAndFail(
                &(Error::WRONG_NUMBER_OF_SUBSCRIPTS.clone()),
                list![cref_str, subl_str, diml_str],
                &inInfo,
            )?;
            unreachable!("Error.addSourceMessageAndFail always fails — caller-side flow-analysis hint");
        } else {
            let (__pa0, __pa1) = ::match_deref::match_deref! { match &(rest_dims) {
                Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
                _ => return Err("pattern mismatch"),
            } };
            dim = metamodelica::Own::own(__pa0);
            rest_dims = metamodelica::Own::own(__pa1);
        }
        (outCache, dsub, r#const, prop) = elabSubscript(
            outCache,
            inEnv.clone(),
            metamodelica::AsArg::as_arg(&asub),
            inImpl,
            inPrefix.clone(),
            inInfo.clone(),
        )?;
        outConst = Types::constAnd(r#const, outConst);
        (outCache, dsub) = elabSubscriptsDims2(
            outCache,
            inEnv.clone(),
            dsub,
            &dim,
            outConst,
            prop,
            inImpl,
            inCref,
            inInfo.clone(),
        )?;
        outSubs = metamodelica::cons(dsub, outSubs);
    }
    nrsubs = ((outSubs).len() as i32);
    if nrsubs > 0 {
        nrdims = ((inDimensions).len() as i32);
        while nrsubs < nrdims {
            outSubs = metamodelica::cons(
                openmodelica_frontend_types::DAE::Subscript::interned_WHOLEDIM(),
                outSubs,
            );
            nrsubs = nrsubs + 1;
        }
    }
    outSubs = outSubs.reverse();
    Ok((outCache, outSubs, outConst))
}

fn elabSubscriptsDims2(
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
    mut inSubscript: metamodelica::Ref<DAE::Subscript>,
    mut inDimension: &metamodelica::Ref<DAE::Dimension>,
    mut inConst: DAE::Const,
    mut inProperties: Option<DAE::Properties>,
    mut inImpl: bool,
    mut inCref: &metamodelica::Ref<Absyn::ComponentRef>,
    mut inInfo: SourceInfo,
) -> Result<(FCore::Cache, metamodelica::Ref<DAE::Subscript>)> {
    let mut outCache: FCore::Cache;
    let mut outSubscript: metamodelica::Ref<DAE::Subscript>;
    (outCache, outSubscript) = 'mc: {
        let __mc_input = (&**inDimension, inProperties);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _) => {
                    let true = (FGraph::inForOrParforIterLoopScope(&inEnv)) else { return Err("pattern mismatch") };
                    let true = (Expression::dimensionKnown(inDimension)) else { return Err("pattern mismatch") };
                    Ok((inCache.clone(), inSubscript.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Some(prop)) => {
                    let mut ty: metamodelica::Ref<DAE::Type>;
                    let true = (Types::isParameter(inConst)) else { return Err("pattern mismatch") };
                    ty = Types::getPropType(metamodelica::AsArg::as_arg(&prop));
                    let false = (Types::getFixedVarAttributeParameterOrConstant(&ty)) else { return Err("pattern mismatch") };
                    Ok((inCache.clone(), inSubscript.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _) => {
                    let mut cache: FCore::Cache;
                    let mut sub: metamodelica::Ref<DAE::Subscript>;
                    let mut int_dim: i32;
                    int_dim = Expression::dimensionSize(inDimension)?;
                    let true = (Types::isParameterOrConstant(inConst)) else { return Err("pattern mismatch") };
                    (cache, sub) = Ceval::cevalSubscript(inCache.clone(), inEnv.clone(), inSubscript.clone(), int_dim, inImpl, Absyn::Msg::MSG { info: inInfo.clone() }, 0)?;
                    Ok((cache.clone(), sub.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Dimension::DIM_EXP { exp: e }, _) => {
                    let mut cache: FCore::Cache;
                    let mut sub: metamodelica::Ref<DAE::Subscript>;
                    let mut int_dim: i32;
                    let true = (Types::isParameterOrConstant(inConst)) else { return Err("pattern mismatch") };
                    let __pa0 = ::match_deref::match_deref! { match &(Ceval::ceval(inCache.clone(), inEnv.clone(), e.clone(), true, Absyn::Msg::MSG { info: inInfo.clone() }, 0)?) {
                        (_, Deref @ Values::Value::INTEGER { integer: __pa0 }) => __pa0.clone(),
                        _ => return Err("pattern mismatch"),
                    } };
                    int_dim = metamodelica::Own::own(__pa0);
                    (cache, sub) = Ceval::cevalSubscript(inCache.clone(), inEnv.clone(), inSubscript.clone(), int_dim, inImpl, Absyn::Msg::MSG { info: inInfo.clone() }, 0)?;
                    Ok((cache.clone(), sub.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _) => {
                    let true = (Flags::getConfigBool(Flags::CHECK_MODEL.clone())?) else { return Err("pattern mismatch") };
                    let true = (Types::isParameterOrConstant(inConst)) else { return Err("pattern mismatch") };
                    Ok((inCache.clone(), inSubscript.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _) => {
                    let true = (Expression::dimensionKnown(inDimension)) else { return Err("pattern mismatch") };
                    let false = (Types::isConstant(inConst) || Types::isParameter(inConst) && !(FGraph::inForLoopScope(&inEnv))) else { return Err("pattern mismatch") };
                    Ok((inCache.clone(), inSubscript.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Dimension::DIM_UNKNOWN { .. }, _) => {
                    Ok((inCache.clone(), inSubscript.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Dimension::DIM_EXP { exp: _ }, _) => {
                    Ok((inCache.clone(), inSubscript.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut sub_str: ArcStr;
                    let mut dim_str: ArcStr;
                    let mut cref_str: ArcStr;
                    sub_str = ExpressionBasics::printSubscriptStr(&inSubscript)?;
                    dim_str = ExpressionBasics::dimensionString(inDimension)?;
                    cref_str = Dump::printComponentRefStr(inCref)?;
                    Error::addSourceMessage(&(Error::ILLEGAL_SUBSCRIPT.clone()), list![sub_str.clone(), dim_str.clone(), cref_str.clone()], &inInfo)?;
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((outCache, outSubscript))
}

fn elabSubscript(
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
    mut inSubscript: &metamodelica::Ref<Absyn::Subscript>,
    mut inBoolean: bool,
    mut inPrefix: DAE::Prefix,
    mut info: SourceInfo,
) -> Result<(
    FCore::Cache,
    metamodelica::Ref<DAE::Subscript>,
    DAE::Const,
    Option<DAE::Properties>,
)> {
    let mut outCache: FCore::Cache;
    let mut outSubscript: metamodelica::Ref<DAE::Subscript>;
    let mut outConst: DAE::Const;
    let mut outProperties: Option<DAE::Properties>;
    (outCache, outSubscript, outConst, outProperties) = 'mc: {
        let __mc_input = (inCache, inEnv.clone(), &**inSubscript, inBoolean, inPrefix);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, _, Deref @ Absyn::Subscript::NOSUB { .. }, _, _) => {
                    Ok((cache.clone(), openmodelica_frontend_types::DAE::Subscript::interned_WHOLEDIM(), openmodelica_frontend_types::DAE::Const::C_CONST, None))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, Deref @ Absyn::Subscript::SUBSCRIPT { subscript: sub }, r#impl, pre) => {
                    let mut sub_1: metamodelica::Ref<DAE::Exp>;
                    let mut ty: metamodelica::Ref<DAE::Type>;
                    let mut r#const: DAE::Const;
                    let mut sub_2: metamodelica::Ref<DAE::Subscript>;
                    let mut prop: DAE::Properties;
                    let mut cache = (*cache).clone();
                    let (__pa0, __pa1, __pa3, __pa2) = ::match_deref::match_deref! { match &(elabExpInExpression(cache.clone(), env.clone(), sub.clone(), r#impl.clone(), true, pre.clone(), info.clone())?) {
                        (__pa0, __pa1, __pa3 @ DAE::Properties::PROP { constFlag: __pa2, .. }) => (__pa0.clone(), __pa1.clone(), __pa3.clone(), __pa2.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    cache = metamodelica::Own::own(__pa0);
                    sub_1 = metamodelica::Own::own(__pa1);
                    r#const = metamodelica::Own::own(__pa2);
                    prop = metamodelica::Own::own(__pa3);
                    let (__pa4, __pa5, __pa7, __pa6) = ::match_deref::match_deref! { match &(Ceval::cevalIfConstant(cache.clone(), env.clone(), sub_1.clone(), prop.clone(), r#impl.clone(), info.clone())?) {
                        (__pa4, __pa5, __pa7 @ DAE::Properties::PROP { type_: __pa6, .. }) => (__pa4.clone(), __pa5.clone(), __pa7.clone(), __pa6.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    cache = metamodelica::Own::own(__pa4);
                    sub_1 = metamodelica::Own::own(__pa5);
                    ty = metamodelica::Own::own(__pa6);
                    prop = metamodelica::Own::own(__pa7);
                    sub_2 = elabSubscriptType(&ty, metamodelica::AsArg::as_arg(&sub), &sub_1, &info)?;
                    Ok((cache.clone(), sub_2.clone(), r#const, Some(prop.clone())))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { return Err("pattern mismatch") };
                    Debug::traceln({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("- Static.elabSubscript failed on ")); __mm_s.push_str(&*Dump::printSubscriptStr(inSubscript)?); __mm_s.push_str(&*literal!(" in env: ")); __mm_s.push_str(&*FGraph::printGraphPathStr(&inEnv)); ArcStr::from(__mm_s) })?;
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((outCache, outSubscript, outConst, outProperties))
}

fn elabSubscriptType<'__b>(
    mut inType: &'__b metamodelica::Ref<DAE::Type>,
    mut inAbsynExp: &'__b metamodelica::Ref<Absyn::Exp>,
    mut inDaeExp: &'__b metamodelica::Ref<DAE::Exp>,
    mut inInfo: &'__b SourceInfo,
) -> Result<metamodelica::Ref<DAE::Subscript>> {
    '__tco: loop {
        ::match_deref::match_deref! { match inType {
            Deref @ DAE::Type::T_INTEGER { .. } => {
                return Ok(metamodelica::Ref::new(DAE::Subscript::INDEX { exp: inDaeExp.clone() }))
            },
            Deref @ DAE::Type::T_ENUMERATION { .. } => {
                return Ok(metamodelica::Ref::new(DAE::Subscript::INDEX { exp: inDaeExp.clone() }))
            },
            Deref @ DAE::Type::T_BOOL { .. } => {
                return Ok(metamodelica::Ref::new(DAE::Subscript::INDEX { exp: inDaeExp.clone() }))
            },
            Deref @ DAE::Type::T_ARRAY { ty: Deref @ DAE::Type::T_INTEGER { .. }, .. } => {
                return Ok(metamodelica::Ref::new(DAE::Subscript::SLICE { exp: inDaeExp.clone() }))
            },
            Deref @ DAE::Type::T_ARRAY { ty: Deref @ DAE::Type::T_ENUMERATION { .. }, .. } => {
                return Ok(metamodelica::Ref::new(DAE::Subscript::SLICE { exp: inDaeExp.clone() }))
            },
            Deref @ DAE::Type::T_ARRAY { ty: Deref @ DAE::Type::T_BOOL { .. }, .. } => {
                return Ok(metamodelica::Ref::new(DAE::Subscript::SLICE { exp: inDaeExp.clone() }))
            },
            Deref @ DAE::Type::T_METABOXED { .. } => {
                { (inType, inAbsynExp, inDaeExp, inInfo) = (var_field!((**inType).ty, DAE::Type::T_METABOXED), inAbsynExp, inDaeExp, inInfo); continue '__tco; }
            },
            _ => {
                let mut e_str: ArcStr;
                let mut t_str: ArcStr;
                e_str = Dump::printExpStr(inAbsynExp.clone())?;
                t_str = TypesDump::unparseType(inType.clone())?;
                Error::addSourceMessage(&(Error::WRONG_DIMENSION_TYPE.clone()), list![e_str, t_str], inInfo)?;
                return Ok(return Err("fail"))
            },
            _ => return Err("match: no arm matched"),
        } }
    }
}

fn subscriptCrefType(
    mut inExp: &metamodelica::Ref<DAE::Exp>,
    mut inType: metamodelica::Ref<DAE::Type>,
) -> metamodelica::Ref<DAE::Type> {
    let mut outType: metamodelica::Ref<DAE::Type>;
    outType = 'mc: {
        let __mc_input = (&**inExp, inType.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CREF { componentRef: c, .. }, t) => {
                    let mut t_1: metamodelica::Ref<DAE::Type>;
                    t_1 = subscriptCrefType2(c.clone(), t.clone())?;
                    Ok(t_1.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(inType.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    outType
}

fn subscriptCrefType2(
    mut inComponentRef: metamodelica::Ref<DAE::ComponentRef>,
    mut inType: metamodelica::Ref<DAE::Type>,
) -> Result<metamodelica::Ref<DAE::Type>> {
    '__tco: loop {
        ::match_deref::match_deref! { match &((inComponentRef, inType)) {
            (Deref @ DAE::ComponentRef::CREF_IDENT { subscriptLst: Deref @ metamodelica::ListNode::Nil, .. }, t) => {
                return Ok(t.clone())
            },
            (Deref @ DAE::ComponentRef::CREF_IDENT { subscriptLst: subs, .. }, t) => {
                let mut t_1: metamodelica::Ref<DAE::Type>;
                return Ok(subscriptType(t.clone(), metamodelica::AsArg::as_arg(&subs))?)
            },
            (Deref @ DAE::ComponentRef::CREF_QUAL { componentRef: c, .. }, t) => {
                let mut t_1: metamodelica::Ref<DAE::Type>;
                { (inComponentRef, inType) = (c.clone(), t.clone()); continue '__tco; }
            },
            _ => return Err("match: no arm matched"),
        } }
    }
}

fn subscriptType(
    mut inType: metamodelica::Ref<DAE::Type>,
    mut inExpSubscriptLst: &metamodelica::List<metamodelica::Ref<DAE::Subscript>>,
) -> Result<metamodelica::Ref<DAE::Type>> {
    let mut outType: metamodelica::Ref<DAE::Type>;
    outType = 'mc: {
        let __mc_input = (inType, &**inExpSubscriptLst);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (t, Deref @ metamodelica::ListNode::Nil) => {
                    Ok(t.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Type::T_ARRAY { dims: Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Dimension::DIM_INTEGER { .. }, tail: Deref @ metamodelica::ListNode::Nil }, ty: t }, Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Subscript::INDEX { .. }, tail: subs }) => {
                    let mut t_1: metamodelica::Ref<DAE::Type>;
                    t_1 = subscriptType(t.clone(), metamodelica::AsArg::as_arg(&subs))?;
                    Ok(t_1.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Type::T_ARRAY { dims: Deref @ metamodelica::ListNode::Cons { head: dim, tail: Deref @ metamodelica::ListNode::Nil }, ty: t }, Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Subscript::SLICE { .. }, tail: subs }) => {
                    let mut t_1: metamodelica::Ref<DAE::Type>;
                    t_1 = subscriptType(t.clone(), metamodelica::AsArg::as_arg(&subs))?;
                    Ok(metamodelica::Ref::new(DAE::Type::T_ARRAY { ty: t_1.clone(), dims: list![dim.clone()] }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Type::T_ARRAY { dims: Deref @ metamodelica::ListNode::Cons { head: dim, tail: Deref @ metamodelica::ListNode::Nil }, ty: t }, Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Subscript::WHOLEDIM { .. }, tail: subs }) => {
                    let mut t_1: metamodelica::Ref<DAE::Type>;
                    t_1 = subscriptType(t.clone(), metamodelica::AsArg::as_arg(&subs))?;
                    Ok(metamodelica::Ref::new(DAE::Type::T_ARRAY { ty: t_1.clone(), dims: list![dim.clone()] }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (t, _) => {
                    Print::printBuf(literal!("- subscript_type failed ("))?;
                    Print::printBuf(TypesDump::printTypeStr(t.clone()))?;
                    Print::printBuf(literal!(" , [...])\n"))?;
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(outType)
}

fn makeIfExp(
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
    mut inCondition: metamodelica::Ref<DAE::Exp>,
    mut inCondProp: DAE::Properties,
    mut inTrueBranch: metamodelica::Ref<DAE::Exp>,
    mut inTrueProp: DAE::Properties,
    mut inFalseBranch: metamodelica::Ref<DAE::Exp>,
    mut inFalseProp: DAE::Properties,
    mut inImplicit: bool,
    mut inPrefix: DAE::Prefix,
    mut inInfo: &SourceInfo,
) -> Result<(FCore::Cache, metamodelica::Ref<DAE::Exp>, DAE::Properties)> {
    let mut outCache: FCore::Cache = inCache.clone();
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    let mut outProperties: DAE::Properties;
    let mut ty_match: bool;
    let mut cond: bool;
    let mut cond_ty: metamodelica::Ref<DAE::Type>;
    let mut true_ty: metamodelica::Ref<DAE::Type>;
    let mut false_ty: metamodelica::Ref<DAE::Type>;
    let mut exp_ty: metamodelica::Ref<DAE::Type>;
    let mut cond_c: DAE::Const;
    let mut true_c: DAE::Const;
    let mut false_c: DAE::Const;
    let mut exp_c: DAE::Const;
    let mut cond_str: ArcStr;
    let mut cond_ty_str: ArcStr;
    let mut e1_str: ArcStr;
    let mut e2_str: ArcStr;
    let mut ty1_str: ArcStr;
    let mut ty2_str: ArcStr;
    let mut pre_str: ArcStr;
    let mut cond_exp: metamodelica::Ref<DAE::Exp>;
    let mut true_exp: metamodelica::Ref<DAE::Exp>;
    let mut false_exp: metamodelica::Ref<DAE::Exp>;
    let DAE::PROP {
        type_: __pa0,
        constFlag: __pa1,
    } = (inCondProp)
    else {
        return Err("pattern mismatch");
    };
    cond_ty = metamodelica::Own::own(__pa0);
    cond_c = metamodelica::Own::own(__pa1);
    (cond_exp, _, ty_match) =
        Types::matchTypeNoFail(inCondition.clone(), cond_ty.clone(), DAE::T_BOOL_DEFAULT().clone());
    if !(ty_match) {
        cond_str = ExpressionBasics::printExpStr(inCondition)?;
        cond_ty_str = TypesDump::unparseTypeNoAttr(&cond_ty)?;
        Error::addSourceMessageAndFail(
            &(Error::IF_CONDITION_TYPE_ERROR.clone()),
            list![cond_str, cond_ty_str],
            inInfo,
        )?;
        unreachable!("Error.addSourceMessageAndFail always fails — caller-side flow-analysis hint");
    }
    let DAE::PROP {
        type_: __pa2,
        constFlag: __pa3,
    } = (inTrueProp.clone())
    else {
        return Err("pattern mismatch");
    };
    true_ty = metamodelica::Own::own(__pa2);
    true_c = metamodelica::Own::own(__pa3);
    let DAE::PROP {
        type_: __pa4,
        constFlag: __pa5,
    } = (inFalseProp.clone())
    else {
        return Err("pattern mismatch");
    };
    false_ty = metamodelica::Own::own(__pa4);
    false_c = metamodelica::Own::own(__pa5);
    (true_exp, false_exp, exp_ty, ty_match) = Types::checkTypeCompat(
        inTrueBranch.clone(),
        true_ty.clone(),
        inFalseBranch.clone(),
        &false_ty,
        false,
    )?;
    if Types::arrayHasUnknownDims(&exp_ty)? && !(FGraph::inFunctionScope(&inEnv)) {
        if Types::isParameterOrConstant(cond_c) {
            cond_c = openmodelica_frontend_types::DAE::Const::C_CONST;
        } else {
            ty_match = false;
        }
    }
    if !(ty_match) && !(Config::getGraphicsExpMode()?) {
        e1_str = ExpressionBasics::printExpStr(inTrueBranch)?;
        e2_str = ExpressionBasics::printExpStr(inFalseBranch)?;
        ty1_str = TypesDump::unparseTypeNoAttr(&true_ty)?;
        ty2_str = TypesDump::unparseTypeNoAttr(&false_ty)?;
        pre_str = PrefixUtil::printPrefixStr3(inPrefix)?;
        Error::addSourceMessageAndFail(
            &(Error::TYPE_MISMATCH_IF_EXP.clone()),
            list![pre_str, e1_str, ty1_str, e2_str, ty2_str],
            inInfo,
        )?;
        unreachable!("Error.addSourceMessageAndFail always fails — caller-side flow-analysis hint");
    }
    if Types::isConstant(cond_c) {
        if '__try6: {
            let (__pa7, __pa8) = ::match_deref::match_deref! { match &(unwrap_break_err!(Ceval::ceval(inCache.clone(), inEnv.clone(), cond_exp.clone(), inImplicit, openmodelica_ast::Absyn::Msg::NO_MSG, 0), '__try6)) {
                (__pa7, Deref @ Values::Value::BOOL { boolean: __pa8 }) => (__pa7.clone(), __pa8.clone()),
                _ => break '__try6 Err::<_, _>("pattern mismatch"),
            } };
            outCache = metamodelica::Own::own(__pa7);
            cond = metamodelica::Own::own(__pa8);
            if cond {
                outExp = true_exp.clone();
                outProperties = inTrueProp.clone();
            } else {
                outExp = false_exp.clone();
                outProperties = inFalseProp.clone();
            }
            return Ok((outCache, outExp, outProperties));
            Ok::<(), &'static str>(())
        }.is_err() {
        }
    }
    exp_c = ({
        let mut __acc: Option<DAE::Const> = None;
        for mut c in (list![cond_c, false_c, true_c]).into_iter().cloned() {
            let __x = c.clone();
            __acc = Some(match __acc {
                None => __x,
                Some(__cur) => Types::constAnd(__x, __cur),
            });
        }
        __acc.ok_or_else(|| "empty Types.constAnd reduction")?
    });
    outExp = metamodelica::Ref::new(DAE::Exp::IFEXP {
        expCond: cond_exp,
        expThen: true_exp,
        expElse: false_exp,
    });
    outProperties = DAE::Properties::PROP {
        type_: exp_ty,
        constFlag: exp_c,
    };
    Ok((outCache, outExp, outProperties))
}

fn canonCref2(
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
    mut inComponentRef: &metamodelica::Ref<DAE::ComponentRef>,
    mut inPrefixCref: metamodelica::Ref<DAE::ComponentRef>,
    mut inBoolean: bool,
) -> Result<(FCore::Cache, metamodelica::Ref<DAE::ComponentRef>)> {
    let mut outCache: FCore::Cache;
    let mut outComponentRef: metamodelica::Ref<DAE::ComponentRef>;
    (outCache, outComponentRef) = (match &**inComponentRef {
        DAE::ComponentRef::CREF_IDENT {
            ident: n,
            identType: ty2,
            subscriptLst: ss,
        } => {
            let mut cache = inCache;
            let mut env = inEnv;
            let mut prefixCr = inPrefixCref;
            let mut r#impl = inBoolean;
            let mut ss_1: metamodelica::List<metamodelica::Ref<DAE::Subscript>>;
            let mut cr: metamodelica::Ref<DAE::ComponentRef>;
            let mut sl: metamodelica::List<i32>;
            let mut t: metamodelica::Ref<DAE::Type>;
            cr = ComponentReference::crefPrependIdent(&prefixCr, n, &(metamodelica::nil()), ty2)?;
            (cache, _, t, _, _, _, _, _, _) = Lookup::lookupVar(cache, env.clone(), cr)?;
            sl = Types::getDimensionSizes(&t)?;
            (cache, ss_1) =
                Ceval::cevalSubscripts(cache, env, ss, &sl, r#impl, openmodelica_ast::Absyn::Msg::NO_MSG, 0)?;
            (
                cache,
                ComponentReferenceBasics::makeCrefIdent(n.clone(), ty2.clone(), ss_1),
            )
        }
        _ => return Err("match: no arm matched"),
    });
    Ok((outCache, outComponentRef))
}

pub(crate) fn canonCref(
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
    mut inComponentRef: metamodelica::Ref<DAE::ComponentRef>,
    mut inBoolean: bool,
) -> Result<(FCore::Cache, metamodelica::Ref<DAE::ComponentRef>)> {
    let mut outCache: FCore::Cache;
    let mut outComponentRef: metamodelica::Ref<DAE::ComponentRef>;
    (outCache, outComponentRef) = 'mc: {
        let __mc_input = (inCache, inEnv, inComponentRef, inBoolean);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, _, Deref @ DAE::ComponentRef::WILD { .. }, _) => {
                    let true = (Config::acceptMetaModelicaGrammar()?) else { return Err("pattern mismatch") };
                    Ok((cache.clone(), openmodelica_frontend_types::DAE::ComponentRef::interned_WILD()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, Deref @ DAE::ComponentRef::CREF_IDENT { ident: n, subscriptLst: ss, .. }, r#impl) => {
                    let mut t: metamodelica::Ref<DAE::Type>;
                    let mut sl: metamodelica::List<i32>;
                    let mut ss_1: metamodelica::List<metamodelica::Ref<DAE::Subscript>>;
                    let mut ty2: metamodelica::Ref<DAE::Type>;
                    let mut cache = (*cache).clone();
                    (cache, _, t, _, _, _, _, _, _) = Lookup::lookupVarIdent(cache.clone(), env.clone(), n.clone(), metamodelica::nil())?;
                    sl = Types::getDimensionSizes(&t)?;
                    (cache, ss_1) = Ceval::cevalSubscripts(cache.clone(), env.clone(), metamodelica::AsArg::as_arg(&ss), &sl, r#impl.clone(), openmodelica_ast::Absyn::Msg::NO_MSG, 0)?;
                    ty2 = Types::simplifyType(t.clone())?;
                    Ok((cache.clone(), ComponentReferenceBasics::makeCrefIdent(n.clone(), ty2.clone(), ss_1.clone())))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (cache, env, Deref @ DAE::ComponentRef::CREF_QUAL { ident: n, subscriptLst: ss, componentRef: c, .. }, r#impl) => {
                    let mut t: metamodelica::Ref<DAE::Type>;
                    let mut sl: metamodelica::List<i32>;
                    let mut ss_1: metamodelica::List<metamodelica::Ref<DAE::Subscript>>;
                    let mut componentEnv: FCore::Graph;
                    let mut c_1: metamodelica::Ref<DAE::ComponentRef>;
                    let mut ty2: metamodelica::Ref<DAE::Type>;
                    let mut cache = (*cache).clone();
                    (cache, _, t, _, _, _, _, componentEnv, _) = Lookup::lookupVarIdent(cache.clone(), env.clone(), n.clone(), metamodelica::nil())?;
                    ty2 = Types::simplifyType(t.clone())?;
                    sl = Types::getDimensionSizes(&t)?;
                    (cache, ss_1) = Ceval::cevalSubscripts(cache.clone(), env.clone(), metamodelica::AsArg::as_arg(&ss), &sl, r#impl.clone(), openmodelica_ast::Absyn::Msg::NO_MSG, 0)?;
                    (cache, c_1) = canonCref(cache.clone(), componentEnv.clone(), c.clone(), r#impl.clone())?;
                    Ok((cache.clone(), ComponentReferenceBasics::makeCrefQual(n.clone(), ty2.clone(), ss_1.clone(), c_1.clone())))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, cr, _) => {
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { return Err("pattern mismatch") };
                    Debug::trace(literal!("- Static.canonCref failed, cr: "))?;
                    Debug::traceln(ComponentReferenceBasics::printComponentRefStr(metamodelica::AsArg::as_arg(&cr))?)?;
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((outCache, outComponentRef))
}

fn unevaluatedFunctionVariability(mut inEnv: &FCore::Graph) -> Result<DAE::Const> {
    let mut outConst: DAE::Const;
    if FGraph::inFunctionScope(inEnv) {
        outConst = openmodelica_frontend_types::DAE::Const::C_VAR;
    } else if Flags::getConfigBool(Flags::CHECK_MODEL.clone())? || Config::splitArrays()? {
        outConst = openmodelica_frontend_types::DAE::Const::C_UNKNOWN;
    } else {
        return Err("fail");
    }
    Ok(outConst)
}

fn slotAnd(mut s: Slot, mut b: bool) -> bool {
    let mut res: bool;
    let Slot { slotFilled: __pa0, .. } = s;
    res = metamodelica::Own::own(__pa0);
    res = b && res;
    res
}

pub fn elabCodeExp(
    mut exp: &metamodelica::Ref<Absyn::Exp>,
    mut cache: &FCore::Cache,
    mut env: &FCore::Graph,
    mut ct: DAE::CodeType,
    mut info: &SourceInfo,
) -> Result<metamodelica::Ref<DAE::Exp>> {
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    outExp = 'mc: {
        let __mc_input = (&**exp, ct);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _) => {
                    let mut dexp: metamodelica::Ref<DAE::Exp>;
                    dexp = elabCodeExp_dispatch(exp.clone(), cache.clone(), env.clone(), ct, info.clone())?;
                    Ok(dexp.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ Absyn::Exp::CODE { code: Deref @ Absyn::CodeNode::C_MODIFICATION { .. } }, DAE::CodeType::C_EXPRESSION_OR_MODIFICATION { .. }) => {
                    Ok(metamodelica::Ref::new(DAE::Exp::CODE { code: var_field!((**exp).code, Absyn::Exp::CODE).clone(), ty: DAE::T_UNKNOWN_DEFAULT().clone() }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ Absyn::Exp::CODE { code: Deref @ Absyn::CodeNode::C_EXPRESSION { .. } }, DAE::CodeType::C_EXPRESSION { .. }) => {
                    Ok(metamodelica::Ref::new(DAE::Exp::CODE { code: var_field!((**exp).code, Absyn::Exp::CODE).clone(), ty: DAE::T_UNKNOWN_DEFAULT().clone() }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, DAE::CodeType::C_EXPRESSION { .. }) => {
                    Ok(metamodelica::Ref::new(DAE::Exp::CODE { code: metamodelica::Ref::new(Absyn::CodeNode::C_EXPRESSION { exp: exp.clone() }), ty: DAE::T_UNKNOWN_DEFAULT().clone() }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, DAE::CodeType::C_EXPRESSION_OR_MODIFICATION { .. }) => {
                    Ok(metamodelica::Ref::new(DAE::Exp::CODE { code: metamodelica::Ref::new(Absyn::CodeNode::C_EXPRESSION { exp: exp.clone() }), ty: DAE::T_UNKNOWN_DEFAULT().clone() }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ Absyn::Exp::CREF { componentRef: cr }, DAE::CodeType::C_TYPENAME { .. }) => {
                    let mut path: metamodelica::Ref<Absyn::Path>;
                    path = AbsynUtil::crefToPath(metamodelica::AsArg::as_arg(&cr))?;
                    Ok(metamodelica::Ref::new(DAE::Exp::CODE { code: metamodelica::Ref::new(Absyn::CodeNode::C_TYPENAME { path: path.clone() }), ty: DAE::T_UNKNOWN_DEFAULT().clone() }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ Absyn::Exp::ARRAY { arrayExp: es }, DAE::CodeType::C_VARIABLENAMES { .. }) => {
                    let mut es_1: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                    let mut et: metamodelica::Ref<DAE::Type>;
                    let mut i: i32;
                    es_1 = List::map4(es.clone(), &move |__a0: metamodelica::Ref<Absyn::Exp>, __a1: FCore::Cache, __a2: FCore::Graph, __a3: DAE::CodeType, __a4: SourceInfo| elabCodeExp(&__a0, &__a1, &__a2, __a3, &__a4), cache.clone(), env.clone(), openmodelica_frontend_types::DAE::CodeType::C_VARIABLENAME, info.clone())?;
                    i = ((es).len() as i32);
                    et = metamodelica::Ref::new(DAE::Type::T_ARRAY { ty: DAE::T_UNKNOWN_DEFAULT().clone(), dims: list![metamodelica::Ref::new(DAE::Dimension::DIM_INTEGER { integer: i })] });
                    Ok(metamodelica::Ref::new(DAE::Exp::ARRAY { ty: et.clone(), scalar: false, array: es_1.clone() }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, DAE::CodeType::C_VARIABLENAMES { .. }) => {
                    let mut et: metamodelica::Ref<DAE::Type>;
                    let mut dexp: metamodelica::Ref<DAE::Exp>;
                    et = metamodelica::Ref::new(DAE::Type::T_ARRAY { ty: DAE::T_UNKNOWN_DEFAULT().clone(), dims: list![metamodelica::Ref::new(DAE::Dimension::DIM_INTEGER { integer: 1 })] });
                    dexp = elabCodeExp(exp, cache, env, openmodelica_frontend_types::DAE::CodeType::C_VARIABLENAME, info)?;
                    Ok(metamodelica::Ref::new(DAE::Exp::ARRAY { ty: et.clone(), scalar: false, array: list![dexp.clone()] }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ Absyn::Exp::CREF { componentRef: cr }, DAE::CodeType::C_VARIABLENAME { .. }) => {
                    Ok(metamodelica::Ref::new(DAE::Exp::CODE { code: metamodelica::Ref::new(Absyn::CodeNode::C_VARIABLENAME { componentRef: cr.clone() }), ty: DAE::T_UNKNOWN_DEFAULT().clone() }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ Absyn::Exp::CALL { .. }, DAE::CodeType::C_VARIABLENAME { .. }) => {
                    if !((isValidDerVariableName(exp.clone(), false))) { return Err("guard") }
                    Ok(metamodelica::Ref::new(DAE::Exp::CODE { code: metamodelica::Ref::new(Absyn::CodeNode::C_EXPRESSION { exp: exp.clone() }), ty: DAE::T_UNKNOWN_DEFAULT().clone() }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut s1: ArcStr;
                    let mut s2: ArcStr;
                    if '__try0: {
                        let DAE::C_VARIABLENAMES { .. } = (ct) else { break '__try0 Err::<_, _>("pattern mismatch") };
                        Ok::<(), &'static str>(())
                    }.is_ok() { return Err("failure(): body succeeded") }
                    s1 = Dump::printExpStr(exp.clone())?;
                    s2 = TypesDump::printCodeTypeStr(ct);
                    Error::addSourceMessage(&(Error::ELAB_CODE_EXP_FAILED.clone()), list![s1.clone(), s2.clone()], info)?;
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(outExp)
}

pub(crate) fn elabCodeExp_dispatch(
    mut exp: metamodelica::Ref<Absyn::Exp>,
    mut cache: FCore::Cache,
    mut env: FCore::Graph,
    mut ct: DAE::CodeType,
    mut info: SourceInfo,
) -> Result<metamodelica::Ref<DAE::Exp>> {
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    outExp = 'mc: {
        let __mc_input = &*exp;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Absyn::Exp::CREF { componentRef: cr } => {
                    let mut dexp: metamodelica::Ref<DAE::Exp>;
                    let mut prop: DAE::Properties;
                    let mut ct2: DAE::CodeType;
                    let mut id: ArcStr;
                    ErrorExt::setCheckpoint(literal!("elabCodeExp_dispatch1"));
                    id = AbsynUtil::crefFirstIdent(metamodelica::AsArg::as_arg(&cr))?;
                    match '__try0: {
                        let true = (metamodelica::stringEq(&id, &(literal!("OpenModelica")))) else { break '__try0 Err::<_, _>("pattern mismatch") };
                        (_, dexp, prop) = unwrap_break_err!(elabExpInExpression(cache.clone(), env.clone(), exp.clone(), false, false, openmodelica_frontend_types::DAE::Prefix::NOPRE, info.clone()), '__try0);
                        Ok::<_, &'static str>((dexp.clone(), prop.clone()))
                    } {
                        Ok((__try0_o0, __try0_o1)) => {
                            dexp = __try0_o0;
                            prop = __try0_o1;
                        }
                        Err(_) => {
                            if '__try1: {
                                        unwrap_break_err!(Lookup::lookupClassIdent(cache.clone(), env.clone(), &id, None), '__try1);
                                        Ok::<(), &'static str>(())
                            }.is_ok() { return Err("failure(): body succeeded") }
                            (_, dexp, prop) = elabExpInExpression(cache.clone(), env.clone(), exp.clone(), false, false, openmodelica_frontend_types::DAE::Prefix::NOPRE, info.clone())?;
                        }
                    }
                    let __pa2 = ::match_deref::match_deref! { match &(Types::getPropType(&prop)) {
                        Deref @ DAE::Type::T_CODE { ty: __pa2 } => __pa2.clone(),
                        _ => return Err("pattern mismatch"),
                    } };
                    ct2 = metamodelica::Own::own(__pa2);
                    let true = (ct == ct2) else { return Err("pattern mismatch") };
                    ErrorExt::delCheckpoint(literal!("elabCodeExp_dispatch1"));
                    Ok(dexp.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Absyn::Exp::CREF { .. } => {
                    ErrorExt::rollBack(literal!("elabCodeExp_dispatch1"));
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut dexp: metamodelica::Ref<DAE::Exp>;
                    let mut prop: DAE::Properties;
                    let mut ct2: DAE::CodeType;
                    let false = (AbsynUtil::isCref(&exp)) else { return Err("pattern mismatch") };
                    ErrorExt::setCheckpoint(literal!("elabCodeExp_dispatch"));
                    (_, dexp, prop) = elabExpInExpression(cache.clone(), env.clone(), exp.clone(), false, false, openmodelica_frontend_types::DAE::Prefix::NOPRE, info.clone())?;
                    let __pa0 = ::match_deref::match_deref! { match &(Types::getPropType(&prop)) {
                        Deref @ DAE::Type::T_CODE { ty: __pa0 } => __pa0.clone(),
                        _ => return Err("pattern mismatch"),
                    } };
                    ct2 = metamodelica::Own::own(__pa0);
                    let true = (ct == ct2) else { return Err("pattern mismatch") };
                    ErrorExt::delCheckpoint(literal!("elabCodeExp_dispatch"));
                    Ok(dexp.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let false = (AbsynUtil::isCref(&exp)) else { return Err("pattern mismatch") };
                    ErrorExt::rollBack(literal!("elabCodeExp_dispatch"));
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(outExp)
}

pub(crate) fn elabArrayDims(
    mut inCache: FCore::Cache,
    mut inEnv: &FCore::Graph,
    mut inComponentRef: &metamodelica::Ref<Absyn::ComponentRef>,
    mut inDimensions: &metamodelica::List<metamodelica::Ref<Absyn::Subscript>>,
    mut inImplicit: bool,
    mut inDoVect: bool,
    mut inPrefix: &DAE::Prefix,
    mut inInfo: &SourceInfo,
) -> Result<(FCore::Cache, metamodelica::List<metamodelica::Ref<DAE::Dimension>>)> {
    let mut outCache: FCore::Cache;
    let mut outDimensions: metamodelica::List<metamodelica::Ref<DAE::Dimension>>;
    (outCache, outDimensions) = elabArrayDims2(
        inCache,
        inEnv,
        inComponentRef,
        inDimensions,
        inImplicit,
        inDoVect,
        inPrefix,
        inInfo,
        metamodelica::nil(),
    )?;
    Ok((outCache, outDimensions))
}

fn elabArrayDims2<'__b>(
    mut inCache: FCore::Cache,
    mut inEnv: &'__b FCore::Graph,
    mut inCref: &'__b metamodelica::Ref<Absyn::ComponentRef>,
    mut inDimensions: &'__b metamodelica::List<metamodelica::Ref<Absyn::Subscript>>,
    mut inImplicit: bool,
    mut inDoVect: bool,
    mut inPrefix: &'__b DAE::Prefix,
    mut inInfo: &'__b SourceInfo,
    mut inElaboratedDims: metamodelica::List<metamodelica::Ref<DAE::Dimension>>,
) -> Result<(FCore::Cache, metamodelica::List<metamodelica::Ref<DAE::Dimension>>)> {
    '__tco: loop {
        ::match_deref::match_deref! { match inDimensions {
            Deref @ metamodelica::ListNode::Nil => {
                return Ok((inCache, inElaboratedDims.reverse()))
            },
            Deref @ metamodelica::ListNode::Cons { head: dim, tail: rest_dims } => {
                let mut cache: FCore::Cache;
                let mut elab_dim: metamodelica::Ref<DAE::Dimension>;
                let mut elab_dims: metamodelica::List<metamodelica::Ref<DAE::Dimension>>;
                (cache, elab_dim) = elabArrayDim(inCache, inEnv, inCref, metamodelica::AsArg::as_arg(&dim), inImplicit, inDoVect, inPrefix, inInfo)?;
                elab_dims = metamodelica::cons(elab_dim, inElaboratedDims);
                { (inCache, inEnv, inCref, inDimensions, inImplicit, inDoVect, inPrefix, inInfo, inElaboratedDims) = (cache, inEnv, inCref, rest_dims, inImplicit, inDoVect, inPrefix, inInfo, elab_dims); continue '__tco; }
            },
            _ => return Err("match: no arm matched"),
        } }
    }
}

fn elabArrayDim(
    mut inCache: FCore::Cache,
    mut inEnv: &FCore::Graph,
    mut inCref: &metamodelica::Ref<Absyn::ComponentRef>,
    mut inDimension: &metamodelica::Ref<Absyn::Subscript>,
    mut inImpl: bool,
    mut inDoVect: bool,
    mut inPrefix: &DAE::Prefix,
    mut inInfo: &SourceInfo,
) -> Result<(FCore::Cache, metamodelica::Ref<DAE::Dimension>)> {
    let mut outCache: FCore::Cache;
    let mut outDimension: metamodelica::Ref<DAE::Dimension>;
    (outCache, outDimension) = 'mc: {
        let __mc_input = (inCache.clone(), &**inDimension);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Deref @ Absyn::Subscript::NOSUB { .. }) => {
                    Ok((inCache.clone(), openmodelica_frontend_types::DAE::Dimension::interned_DIM_UNKNOWN()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Deref @ Absyn::Subscript::SUBSCRIPT { subscript: Deref @ Absyn::Exp::CALL { function_: Deref @ Absyn::ComponentRef::CREF_IDENT { name: Deref @ "size", .. }, functionArgs: Deref @ Absyn::FunctionArgs::FUNCTIONARGS { args: Deref @ metamodelica::ListNode::Cons { head: cr_exp @ Deref @ Absyn::Exp::CREF { componentRef: cr }, tail: Deref @ metamodelica::ListNode::Cons { head: size_arg, tail: Deref @ metamodelica::ListNode::Nil } }, .. }, .. } }) => {
                    let mut dim: metamodelica::Ref<DAE::Dimension>;
                    let mut cache: FCore::Cache;
                    let mut e: metamodelica::Ref<DAE::Exp>;
                    let mut dim_exp: metamodelica::Ref<DAE::Exp>;
                    let true = (AbsynUtil::crefEqual(inCref, metamodelica::AsArg::as_arg(&cr))?) else { return Err("pattern mismatch") };
                    (cache, e, _) = elabExpInExpression(inCache.clone(), inEnv.clone(), cr_exp.clone(), inImpl, inDoVect, inPrefix.clone(), inInfo.clone())?;
                    (cache, dim_exp, _) = elabExpInExpression(cache.clone(), inEnv.clone(), size_arg.clone(), inImpl, inDoVect, inPrefix.clone(), inInfo.clone())?;
                    dim = metamodelica::Ref::new(DAE::Dimension::DIM_EXP { exp: metamodelica::Ref::new(DAE::Exp::SIZE { exp: e.clone(), sz: Some(dim_exp.clone()) }) });
                    Ok((inCache.clone(), dim.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Deref @ Absyn::Subscript::SUBSCRIPT { subscript: Deref @ Absyn::Exp::CREF { componentRef: Deref @ Absyn::ComponentRef::CREF_IDENT { name: Deref @ "Boolean", .. } } }) => {
                    Ok((inCache.clone(), openmodelica_frontend_types::DAE::Dimension::interned_DIM_BOOLEAN()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                        (cache, Deref @ Absyn::Subscript::SUBSCRIPT { subscript: Deref @ Absyn::Exp::CREF { componentRef: cr } }) => {
                            let mut dim: metamodelica::Ref<DAE::Dimension>;
                            let mut type_path: metamodelica::Ref<Absyn::Path>;
                            let mut t: metamodelica::Ref<DAE::Type>;
                            let mut cache = (*cache).clone();
                            type_path = AbsynUtil::crefToPath(metamodelica::AsArg::as_arg(&cr))?;
                            (cache, _, _) = Lookup::lookupClass(metamodelica::AsArg::as_arg(&cache), inEnv, &type_path, None)?;
                            (cache, t, _) = Lookup::lookupType(cache.clone(), inEnv.clone(), type_path.clone(), None)?;
                            dim = (match &*t {
                DAE::Type::T_ENUMERATION { index: None, names: __t_names, path: __t_path, .. } => metamodelica::Ref::new(DAE::Dimension::DIM_ENUM { enumTypeName: __t_path.clone(), literals: __t_names.clone(), size: ((__t_names).len() as i32) }),
                DAE::Type::T_BOOL { .. } => openmodelica_frontend_types::DAE::Dimension::interned_DIM_BOOLEAN(),
                _ => return Err("match: no arm matched"),
            });
                            Ok((cache.clone(), dim.clone()))
                        }
                        _ => return Err("nomatch"),
                    }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Deref @ Absyn::Subscript::SUBSCRIPT { subscript: Deref @ Absyn::Exp::EXPRESSIONCOMMENT { exp: sub, .. } }) => {
                    let mut dim: metamodelica::Ref<DAE::Dimension>;
                    let mut cache: FCore::Cache;
                    (cache, dim) = elabArrayDim(inCache.clone(), inEnv, inCref, &(metamodelica::Ref::new(Absyn::Subscript::SUBSCRIPT { subscript: sub.clone() })), inImpl, inDoVect, inPrefix, inInfo)?;
                    Ok((cache.clone(), dim.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Deref @ Absyn::Subscript::SUBSCRIPT { subscript: sub }) => {
                    let mut dim: metamodelica::Ref<DAE::Dimension>;
                    let mut cache: FCore::Cache;
                    let mut e: metamodelica::Ref<DAE::Exp>;
                    let mut prop: DAE::Properties;
                    (cache, e, prop) = elabExpInExpression(inCache.clone(), inEnv.clone(), sub.clone(), inImpl, inDoVect, inPrefix.clone(), inInfo.clone())?;
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(elabArrayDim2(cache.clone(), inEnv.clone(), inCref, e.clone(), prop.clone(), inImpl, inDoVect, inPrefix, inInfo.clone())?) {
                        (__pa0, Some(__pa1)) => (__pa0.clone(), __pa1.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    cache = metamodelica::Own::own(__pa0);
                    dim = metamodelica::Own::own(__pa1);
                    Ok((cache.clone(), dim.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { return Err("pattern mismatch") };
                    Debug::traceln({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("- Static.elabArrayDim failed on: ")); __mm_s.push_str(&*Dump::printComponentRefStr(inCref)?); __mm_s.push_str(&*Dump::printArraydimStr(list![inDimension.clone()])?); ArcStr::from(__mm_s) })?;
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((outCache, outDimension))
}

fn elabArrayDim2(
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
    mut inCref: &metamodelica::Ref<Absyn::ComponentRef>,
    mut inExp: metamodelica::Ref<DAE::Exp>,
    mut inProperties: DAE::Properties,
    mut inImpl: bool,
    mut inDoVect: bool,
    mut inPrefix: &DAE::Prefix,
    mut inInfo: SourceInfo,
) -> Result<(FCore::Cache, Option<metamodelica::Ref<DAE::Dimension>>)> {
    let mut outCache: FCore::Cache;
    let mut outDimension: Option<metamodelica::Ref<DAE::Dimension>>;
    (outCache, outDimension) = 'mc: {
        let __mc_input = (&inProperties, inImpl);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (DAE::Properties::PROP { type_: Deref @ DAE::Type::T_INTEGER { .. }, constFlag: cnst }, _) => {
                    let mut cache: FCore::Cache;
                    let mut i: i32;
                    let true = (Types::isParameterOrConstant(cnst.clone())) else { return Err("pattern mismatch") };
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(Ceval::ceval(inCache.clone(), inEnv.clone(), inExp.clone(), inImpl, Absyn::Msg::MSG { info: Absyn::dummyInfo.clone() }, 0)?) {
                        (__pa0, Deref @ Values::Value::INTEGER { integer: __pa1 }) => (__pa0.clone(), __pa1.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    cache = metamodelica::Own::own(__pa0);
                    i = metamodelica::Own::own(__pa1);
                    Ok((cache.clone(), Some(metamodelica::Ref::new(DAE::Dimension::DIM_INTEGER { integer: i }))))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (DAE::Properties::PROP { type_: Deref @ DAE::Type::T_INTEGER { .. }, constFlag: DAE::Const::C_PARAM { .. } }, _) => {
                    let false = (Config::splitArrays()?) else { return Err("pattern mismatch") };
                    Ok((inCache.clone(), Some(metamodelica::Ref::new(DAE::Dimension::DIM_EXP { exp: inExp.clone() }))))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (DAE::Properties::PROP { type_: Deref @ DAE::Type::T_INTEGER { .. }, constFlag: DAE::Const::C_VAR { .. } }, false) => {
                    let mut e_str: ArcStr;
                    e_str = ExpressionBasics::printExpStr(inExp.clone())?;
                    Error::addSourceMessage(&(Error::DIMENSION_NOT_KNOWN.clone()), list![e_str.clone()], &inInfo)?;
                    Ok((inCache.clone(), None))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (DAE::Properties::PROP { type_: Deref @ DAE::Type::T_INTEGER { .. }, constFlag: _ }, true) => {
                    let mut cache: FCore::Cache;
                    let mut e: metamodelica::Ref<DAE::Exp>;
                    (cache, e, _) = Ceval::cevalIfConstant(inCache.clone(), inEnv.clone(), inExp.clone(), inProperties.clone(), inImpl, inInfo.clone())?;
                    Ok((cache.clone(), Some(metamodelica::Ref::new(DAE::Dimension::DIM_EXP { exp: e.clone() }))))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _) => {
                    let mut cache: FCore::Cache;
                    let mut e: metamodelica::Ref<DAE::Exp>;
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(Ceval::cevalIfConstant(inCache.clone(), inEnv.clone(), inExp.clone(), inProperties.clone(), inImpl, inInfo.clone())?) {
                        (__pa0, __pa1 @ Deref @ DAE::Exp::SIZE { exp: _, sz: _ }, _) => (__pa0.clone(), __pa1.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    cache = metamodelica::Own::own(__pa0);
                    e = metamodelica::Own::own(__pa1);
                    Ok((cache.clone(), Some(metamodelica::Ref::new(DAE::Dimension::DIM_EXP { exp: e.clone() }))))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _) => {
                    let true = (Flags::getConfigBool(Flags::CHECK_MODEL.clone())?) else { return Err("pattern mismatch") };
                    Ok((inCache.clone(), Some(openmodelica_frontend_types::DAE::Dimension::interned_DIM_UNKNOWN())))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (DAE::Properties::PROP { type_: Deref @ DAE::Type::T_INTEGER { .. }, constFlag: cnst }, _) => {
                    let mut e_str: ArcStr;
                    let mut a_str: ArcStr;
                    let true = (Types::isParameterOrConstant(cnst.clone())) else { return Err("pattern mismatch") };
                    e_str = ExpressionBasics::printExpStr(inExp.clone())?;
                    a_str = { let mut __mm_s = String::new(); __mm_s.push_str(&*Dump::printComponentRefStr(inCref)?); __mm_s.push_str(&*literal!("[")); __mm_s.push_str(&*e_str); __mm_s.push_str(&*literal!("]")); ArcStr::from(__mm_s) };
                    Error::addSourceMessage(&(Error::STRUCTURAL_PARAMETER_OR_CONSTANT_WITH_NO_BINDING.clone()), list![e_str.clone(), a_str.clone()], &inInfo)?;
                    Ok((inCache.clone(), None))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (DAE::Properties::PROP { type_: ty, constFlag: _ }, _) => {
                    let mut e_str: ArcStr;
                    let mut t_str: ArcStr;
                    e_str = ExpressionBasics::printExpStr(inExp.clone())?;
                    t_str = TypesDump::unparseType(ty.clone())?;
                    Types::typeErrorSanityCheck(t_str.clone(), &(literal!("Integer")), &inInfo)?;
                    Error::addSourceMessage(&(Error::ARRAY_DIMENSION_INTEGER.clone()), list![e_str.clone(), t_str.clone()], &inInfo)?;
                    Ok((inCache.clone(), None))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((outCache, outDimension))
}

fn consStrippedCref(
    mut e: &metamodelica::Ref<Absyn::Exp>,
    mut es: metamodelica::List<metamodelica::Ref<Absyn::Exp>>,
) -> Result<metamodelica::List<metamodelica::Ref<Absyn::Exp>>> {
    let mut oes: metamodelica::List<metamodelica::Ref<Absyn::Exp>>;
    oes = (match &**e {
        Absyn::Exp::CREF { componentRef: cr } => {
            let mut cr = (*cr).clone();
            cr = AbsynUtil::crefStripLastSubs(cr.clone())?;
            metamodelica::cons(
                metamodelica::Ref::new(Absyn::Exp::CREF {
                    componentRef: cr.clone(),
                }),
                es,
            )
        }
        _ => es,
    });
    Ok(oes)
}

fn replaceEnd(mut inCref: metamodelica::Ref<Absyn::ComponentRef>) -> Result<metamodelica::Ref<Absyn::ComponentRef>> {
    let mut outCref: metamodelica::Ref<Absyn::ComponentRef>;
    let mut cr_parts: metamodelica::List<metamodelica::Ref<Absyn::ComponentRef>>;
    let mut cr: metamodelica::Ref<Absyn::ComponentRef> = metamodelica::Ref::new(Absyn::ComponentRef::ALLWILD);
    let mut cr_no_subs: metamodelica::Ref<Absyn::ComponentRef>;
    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(AbsynUtil::crefExplode(&inCref, metamodelica::nil())) {
        Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: __pa1 } => (__pa0.clone(), __pa1.clone()),
        _ => return Err("pattern mismatch"),
    } };
    outCref = metamodelica::Own::own(__pa0);
    cr_parts = metamodelica::Own::own(__pa1);
    if !(AbsynUtil::crefIsIdent(&outCref)) {
        outCref = inCref;
        return Ok(outCref);
    }
    if AbsynUtil::crefIsFullyQualified(&inCref) {
        outCref = AbsynUtil::crefMakeFullyQualified(outCref);
    }
    outCref = replaceEndInSubs(
        AbsynUtil::crefStripLastSubs(outCref.clone())?,
        &(AbsynUtil::crefLastSubs(&outCref)?),
    )?;
    for mut cr in &*cr_parts {
        let mut cr = cr.clone();
        cr_no_subs = AbsynUtil::crefStripLastSubs(cr.clone())?;
        outCref = AbsynUtil::joinCrefs(&outCref, cr_no_subs)?;
        outCref = replaceEndInSubs(outCref, &(AbsynUtil::crefLastSubs(&cr)?))?;
    }
    Ok(outCref)
}

fn replaceEndInSubs(
    mut inCref: metamodelica::Ref<Absyn::ComponentRef>,
    mut inSubscripts: &metamodelica::List<metamodelica::Ref<Absyn::Subscript>>,
) -> Result<metamodelica::Ref<Absyn::ComponentRef>> {
    let mut outCref: metamodelica::Ref<Absyn::ComponentRef> = inCref.clone();
    let mut subs: metamodelica::List<metamodelica::Ref<Absyn::Subscript>> = metamodelica::nil();
    let mut new_sub: metamodelica::Ref<Absyn::Subscript>;
    let mut i: i32 = 1;
    if (inSubscripts).is_empty() {
        return Ok(outCref);
    }
    for mut sub in &**inSubscripts {
        new_sub = replaceEndInSub(sub.clone(), i, inCref.clone())?;
        subs = metamodelica::cons(new_sub, subs);
        i = i + 1;
    }
    outCref = AbsynUtil::crefSetLastSubs(outCref, &(subs.reverse()))?;
    Ok(outCref)
}

fn replaceEndInSub(
    mut inSubscript: metamodelica::Ref<Absyn::Subscript>,
    mut inDimIndex: i32,
    mut inCref: metamodelica::Ref<Absyn::ComponentRef>,
) -> Result<metamodelica::Ref<Absyn::Subscript>> {
    let mut outSubscript: metamodelica::Ref<Absyn::Subscript>;
    outSubscript = (match &*inSubscript {
        Absyn::Subscript::SUBSCRIPT {
            subscript: __inSubscript_subscript,
        } => metamodelica::Ref::new(Absyn::Subscript::SUBSCRIPT {
            subscript: replaceEndTraverser(__inSubscript_subscript.clone(), (inCref, inDimIndex))?,
        }),
        _ => inSubscript,
    });
    Ok(outSubscript)
}

fn replaceEndTraverser(
    mut inExp: metamodelica::Ref<Absyn::Exp>,
    mut inTuple: (metamodelica::Ref<Absyn::ComponentRef>, i32),
) -> Result<metamodelica::Ref<Absyn::Exp>> {
    let mut outExp: metamodelica::Ref<Absyn::Exp>;
    outExp = (match &*inExp {
        Absyn::Exp::END { .. } => {
            let mut cr: metamodelica::Ref<Absyn::ComponentRef>;
            let mut i: i32;
            (cr, i) = inTuple;
            metamodelica::Ref::new(Absyn::Exp::CALL {
                function_: metamodelica::Ref::new(Absyn::ComponentRef::CREF_IDENT {
                    name: literal!("size"),
                    subscripts: metamodelica::nil(),
                }),
                functionArgs: metamodelica::Ref::new(Absyn::FunctionArgs::FUNCTIONARGS {
                    args: list![
                        metamodelica::Ref::new(Absyn::Exp::CREF { componentRef: cr }),
                        metamodelica::Ref::new(Absyn::Exp::INTEGER { value: i })
                    ],
                    argNames: metamodelica::nil(),
                }),
                typeVars: metamodelica::nil(),
            })
        }
        Absyn::Exp::CREF {
            componentRef: __inExp_componentRef,
        } => metamodelica::Ref::new(Absyn::Exp::CREF {
            componentRef: replaceEnd(__inExp_componentRef.clone())?,
        }),
        _ => AbsynUtil::traverseExpShallow(inExp, inTuple, &replaceEndTraverser)?,
    });
    Ok(outExp)
}

fn fixTupleMetaModelica(
    mut exps: metamodelica::List<metamodelica::Ref<DAE::Exp>>,
    mut types: metamodelica::List<metamodelica::Ref<DAE::Type>>,
    mut consts: metamodelica::List<metamodelica::Ref<DAE::TupleConst>>,
) -> Result<(metamodelica::Ref<DAE::Exp>, DAE::Properties)> {
    let mut exp: metamodelica::Ref<DAE::Exp>;
    let mut prop: DAE::Properties;
    let mut c: DAE::Const;
    let mut tys2: metamodelica::List<metamodelica::Ref<DAE::Type>>;
    let mut exps2: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
    if Config::acceptMetaModelicaGrammar()? {
        c = Types::tupleConstListToConst(&consts)?;
        tys2 = ({
            let mut __acc: metamodelica::List<metamodelica::Ref<DAE::Type>> = metamodelica::nil();
            for mut ty in (types.clone()).into_iter().cloned() {
                let __x = Types::boxIfUnboxedType(ty.clone());
                __acc = cons(__x, __acc);
            }
            __acc.reverse()
        });
        (exps2, tys2) = Types::matchTypeTuple(&exps, &types, &tys2, false)?;
        exp = metamodelica::Ref::new(DAE::Exp::META_TUPLE { listExp: exps2 });
        prop = DAE::Properties::PROP {
            type_: metamodelica::Ref::new(DAE::Type::T_METATUPLE { types: tys2 }),
            constFlag: c,
        };
    } else {
        exp = metamodelica::Ref::new(DAE::Exp::TUPLE { PR: exps });
        prop = DAE::Properties::PROP_TUPLE {
            type_: metamodelica::Ref::new(DAE::Type::T_TUPLE {
                types: types,
                names: None,
            }),
            tupleConst: metamodelica::Ref::new(DAE::TupleConst::TUPLE_CONST { tupleConstLst: consts }),
        };
    }
    Ok((exp, prop))
}

fn checkBuiltinCallArgs(
    mut inPosArgs: &metamodelica::List<metamodelica::Ref<Absyn::Exp>>,
    mut inNamedArgs: &metamodelica::List<metamodelica::Ref<Absyn::NamedArg>>,
    mut inExpectedArgs: i32,
    mut inFnName: ArcStr,
    mut inInfo: &SourceInfo,
) -> Result<()> {
    if ((inPosArgs).len() as i32) != inExpectedArgs || !((inNamedArgs).is_empty()) {
        Error::addSourceMessageAndFail(&(Error::WRONG_NO_OF_ARGS.clone()), list![inFnName], inInfo)?;
        unreachable!("Error.addSourceMessageAndFail always fails — caller-side flow-analysis hint");
    }
    Ok(())
}
