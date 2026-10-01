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

use crate::ComponentReference;
use crate::Expression;
use crate::ExpressionDump;
use crate::ExpressionSimplify;
use crate::HashTable2;
use openmodelica_ast::Absyn;
use openmodelica_frontend_dump::ComponentReferenceBasics;
use openmodelica_frontend_dump::ExpressionBasics;
use openmodelica_frontend_dump::HashTable3;
use openmodelica_frontend_types::DAE;
use openmodelica_frontend_types::SCode;
use openmodelica_util::BaseHashTable;
use openmodelica_util::Error;
use openmodelica_util::Util;
use openmodelica_util_datatypes_basic::List;

/// VariableReplacements consists of a mapping between variables and expressions, the first binary tree of this type.
/// To eliminate a variable from an equation system a replacement rule varname->expression is added to this
/// datatype.
/// To be able to update these replacement rules incrementally a backward lookup mechanism is also required.
/// For instance, having a rule a->b and adding a rule b->c requires to find the first rule a->b and update it to
/// a->c. This is what the second binary tree is used for.
#[derive(Clone, metamodelica::MMCtor, metamodelica::ReferenceEq)]
pub struct VariableReplacements {
    /// src -> dst, used for replacing. src is variable, dst is expression.
    pub hashTable: (
        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<Option<(metamodelica::Ref<DAE::ComponentRef>, metamodelica::Ref<DAE::Exp>)>>,
        ),
        i32,
        (
            HashTable2::FuncHashCref,
            HashTable2::FuncCrefEqual,
            HashTable2::FuncCrefStr,
            HashTable2::FuncExpStr,
        ),
    ),
    /// dst -> list of sources. dst is a variable, sources are variables.
    pub invHashTable: (
        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<
                Option<(
                    metamodelica::Ref<DAE::ComponentRef>,
                    metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
                )>,
            >,
        ),
        i32,
        (
            HashTable3::FuncHashCref,
            HashTable3::FuncCrefEqual,
            HashTable3::FuncCrefStr,
            HashTable3::FuncExpStr,
        ),
    ),
}

impl metamodelica::gc::MMTrace for VariableReplacements {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        metamodelica::gc::MMTrace::mm_accept(&self.hashTable, __mmv)?;
        metamodelica::gc::MMTrace::mm_accept(&self.invHashTable, __mmv)?;
        Ok(())
    }
}
impl PartialEq for VariableReplacements {
    fn eq(&self, other: &Self) -> bool {
        (match ((&self.hashTable), (&other.hashTable)) {
            ((__lt0, __lt1, __lt2, __lt3), (__rt0, __rt1, __rt2, __rt3)) => {
                (__lt0 == __rt0)
                    && (__lt1 == __rt1)
                    && (__lt2 == __rt2)
                    && (match (__lt3, __rt3) {
                        ((__lt0, __lt1, __lt2, __lt3), (__rt0, __rt1, __rt2, __rt3)) => {
                            std::sync::Arc::ptr_eq(__lt0, __rt0)
                                && std::sync::Arc::ptr_eq(__lt1, __rt1)
                                && std::sync::Arc::ptr_eq(__lt2, __rt2)
                                && std::sync::Arc::ptr_eq(__lt3, __rt3)
                        }
                    })
            }
        }) && (match ((&self.invHashTable), (&other.invHashTable)) {
            ((__lt0, __lt1, __lt2, __lt3), (__rt0, __rt1, __rt2, __rt3)) => {
                (__lt0 == __rt0)
                    && (__lt1 == __rt1)
                    && (__lt2 == __rt2)
                    && (match (__lt3, __rt3) {
                        ((__lt0, __lt1, __lt2, __lt3), (__rt0, __rt1, __rt2, __rt3)) => {
                            std::sync::Arc::ptr_eq(__lt0, __rt0)
                                && std::sync::Arc::ptr_eq(__lt1, __rt1)
                                && std::sync::Arc::ptr_eq(__lt2, __rt2)
                                && std::sync::Arc::ptr_eq(__lt3, __rt3)
                        }
                    })
            }
        })
    }
}
impl Eq for VariableReplacements {}
impl PartialOrd for VariableReplacements {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for VariableReplacements {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (match ((&self.hashTable), (&other.hashTable)) {
            ((__lt0, __lt1, __lt2, __lt3), (__rt0, __rt1, __rt2, __rt3)) => __lt0
                .cmp(__rt0)
                .then_with(|| __lt1.cmp(__rt1))
                .then_with(|| __lt2.cmp(__rt2))
                .then_with(|| {
                    (match (__lt3, __rt3) {
                        ((__lt0, __lt1, __lt2, __lt3), (__rt0, __rt1, __rt2, __rt3)) => (std::sync::Arc::as_ptr(__lt0)
                            as *const ())
                            .cmp(&(std::sync::Arc::as_ptr(__rt0) as *const ()))
                            .then_with(|| {
                                (std::sync::Arc::as_ptr(__lt1) as *const ())
                                    .cmp(&(std::sync::Arc::as_ptr(__rt1) as *const ()))
                            })
                            .then_with(|| {
                                (std::sync::Arc::as_ptr(__lt2) as *const ())
                                    .cmp(&(std::sync::Arc::as_ptr(__rt2) as *const ()))
                            })
                            .then_with(|| {
                                (std::sync::Arc::as_ptr(__lt3) as *const ())
                                    .cmp(&(std::sync::Arc::as_ptr(__rt3) as *const ()))
                            }),
                    })
                }),
        })
        .then_with(|| {
            (match ((&self.invHashTable), (&other.invHashTable)) {
                ((__lt0, __lt1, __lt2, __lt3), (__rt0, __rt1, __rt2, __rt3)) => __lt0
                    .cmp(__rt0)
                    .then_with(|| __lt1.cmp(__rt1))
                    .then_with(|| __lt2.cmp(__rt2))
                    .then_with(|| {
                        (match (__lt3, __rt3) {
                            ((__lt0, __lt1, __lt2, __lt3), (__rt0, __rt1, __rt2, __rt3)) => {
                                (std::sync::Arc::as_ptr(__lt0) as *const ())
                                    .cmp(&(std::sync::Arc::as_ptr(__rt0) as *const ()))
                                    .then_with(|| {
                                        (std::sync::Arc::as_ptr(__lt1) as *const ())
                                            .cmp(&(std::sync::Arc::as_ptr(__rt1) as *const ()))
                                    })
                                    .then_with(|| {
                                        (std::sync::Arc::as_ptr(__lt2) as *const ())
                                            .cmp(&(std::sync::Arc::as_ptr(__rt2) as *const ()))
                                    })
                                    .then_with(|| {
                                        (std::sync::Arc::as_ptr(__lt3) as *const ())
                                            .cmp(&(std::sync::Arc::as_ptr(__rt3) as *const ()))
                                    })
                            }
                        })
                    }),
            })
        })
    }
}
impl std::fmt::Debug for VariableReplacements {
    fn fmt(&self, __f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut __ds = __f.debug_struct("VariableReplacements");
        __ds.field(
            "hashTable",
            &format_args!("<dyn-fn-container@{:p}>", (&self.hashTable) as *const _),
        );
        __ds.field(
            "invHashTable",
            &format_args!("<dyn-fn-container@{:p}>", (&self.invHashTable) as *const _),
        );
        __ds.finish()
    }
}

impl Default for VariableReplacements {
    fn default() -> Self {
        Self {
            hashTable: (
                Default::default(),
                Default::default(),
                Default::default(),
                (
                    {
                        let __placeholder: HashTable2::FuncHashCref =
                            std::sync::Arc::new(|_| panic!("default-constructed placeholder fn must not be called"));
                        __placeholder
                    },
                    {
                        let __placeholder: HashTable2::FuncCrefEqual =
                            std::sync::Arc::new(|_, _| panic!("default-constructed placeholder fn must not be called"));
                        __placeholder
                    },
                    {
                        let __placeholder: HashTable2::FuncCrefStr =
                            std::sync::Arc::new(|_| panic!("default-constructed placeholder fn must not be called"));
                        __placeholder
                    },
                    {
                        let __placeholder: HashTable2::FuncExpStr =
                            std::sync::Arc::new(|_| panic!("default-constructed placeholder fn must not be called"));
                        __placeholder
                    },
                ),
            ),
            invHashTable: (
                Default::default(),
                Default::default(),
                Default::default(),
                (
                    {
                        let __placeholder: HashTable3::FuncHashCref =
                            std::sync::Arc::new(|_| panic!("default-constructed placeholder fn must not be called"));
                        __placeholder
                    },
                    {
                        let __placeholder: HashTable3::FuncCrefEqual =
                            std::sync::Arc::new(|_, _| panic!("default-constructed placeholder fn must not be called"));
                        __placeholder
                    },
                    {
                        let __placeholder: HashTable3::FuncCrefStr =
                            std::sync::Arc::new(|_| panic!("default-constructed placeholder fn must not be called"));
                        __placeholder
                    },
                    {
                        let __placeholder: HashTable3::FuncExpStr =
                            std::sync::Arc::new(|_| panic!("default-constructed placeholder fn must not be called"));
                        __placeholder
                    },
                ),
            ),
        }
    }
}

pub type REPLACEMENTS = VariableReplacements;

//protected import Debug;
pub(crate) fn applyReplacementsDAE(
    mut dae: &DAE::DAElist,
    mut repl: &VariableReplacements,
    mut condExpFunc: Option<Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<bool> + 'static>>,
) -> Result<DAE::DAElist> {
    pub type FuncTypeExp_ExpToBoolean =
        std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<bool> + 'static>;

    let mut outDae: DAE::DAElist;
    outDae = (match dae.clone() {
        DAE::DAElist { elementLst: ref elts } => {
            let mut elts = elts.clone();
            elts = applyReplacementsDAEElts(elts.clone(), repl, condExpFunc)?;
            DAE::DAElist {
                elementLst: elts.clone(),
            }
        }
    });
    Ok(outDae)
}

pub(crate) fn applyReplacementsDAEElts(
    mut inDae: metamodelica::List<metamodelica::Ref<DAE::Element>>,
    mut repl: &VariableReplacements,
    mut condExpFunc: Option<Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<bool> + 'static>>,
) -> Result<metamodelica::List<metamodelica::Ref<DAE::Element>>> {
    pub type FuncTypeExp_ExpToBoolean =
        std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<bool> + 'static>;

    let mut outDae: metamodelica::List<metamodelica::Ref<DAE::Element>>;
    if BaseHashTable::hashTableCurrentSize(&(repl.hashTable.clone())) == 0 {
        outDae = inDae;
        return Ok(outDae);
    }
    outDae = ({
        let mut __acc: metamodelica::List<metamodelica::Ref<DAE::Element>> = metamodelica::nil();
        for mut elt in (inDae).into_iter().cloned() {
            let __x = (::match_deref::match_deref! { match &(elt.clone()) {
                Deref @ DAE::Element::VAR { componentRef: cr, kind, direction: dir, parallelism: prl, protection: prot, ty: tp, binding: Some(bindExp), dims, connectorType: ct, source, variableAttributesOption: attr, comment: cmt, innerOuter: io, encrypted: ie } => {
                    let mut bindExp2: metamodelica::Ref<DAE::Exp>;
                    let mut attr = (*attr).clone();
                    (bindExp2, _) = replaceExp(bindExp.clone(), repl, condExpFunc.clone())?;
                    attr = applyReplacementsVarAttr(attr.clone(), repl, condExpFunc.clone())?;
                    metamodelica::Ref::new(DAE::Element::VAR { componentRef: cr.clone(), kind: kind.clone(), direction: dir.clone(), parallelism: prl.clone(), protection: prot.clone(), ty: tp.clone(), binding: Some(bindExp2.clone()), dims: dims.clone(), connectorType: ct.clone(), source: source.clone(), variableAttributesOption: attr.clone(), comment: cmt.clone(), innerOuter: io.clone(), encrypted: ie.clone() })
                },
                Deref @ DAE::Element::VAR { componentRef: cr, kind, direction: dir, parallelism: prl, protection: prot, ty: tp, binding: None, dims, connectorType: ct, source, variableAttributesOption: attr, comment: cmt, innerOuter: io, encrypted: ie } => {
                    let mut attr = (*attr).clone();
                    attr = applyReplacementsVarAttr(attr.clone(), repl, condExpFunc.clone())?;
                    metamodelica::Ref::new(DAE::Element::VAR { componentRef: cr.clone(), kind: kind.clone(), direction: dir.clone(), parallelism: prl.clone(), protection: prot.clone(), ty: tp.clone(), binding: None, dims: dims.clone(), connectorType: ct.clone(), source: source.clone(), variableAttributesOption: attr.clone(), comment: cmt.clone(), innerOuter: io.clone(), encrypted: ie.clone() })
                },
                Deref @ DAE::Element::DEFINE { componentRef: cr, exp: e, source } => {
                    let mut cr2: metamodelica::Ref<DAE::ComponentRef>;
                    let mut e2: metamodelica::Ref<DAE::Exp>;
                    (e2, _) = replaceExp(e.clone(), repl, condExpFunc.clone())?;
                    let __pa0 = ::match_deref::match_deref! { match &(replaceExp(Expression::crefExp(cr.clone())?, repl, condExpFunc.clone())?) {
                        (Deref @ DAE::Exp::CREF { componentRef: __pa0, ty: _ }, _) => __pa0.clone(),
                        _ => return Err("pattern mismatch"),
                    } };
                    cr2 = metamodelica::Own::own(__pa0);
                    metamodelica::Ref::new(DAE::Element::DEFINE { componentRef: cr2.clone(), exp: e2.clone(), source: source.clone() })
                },
                Deref @ DAE::Element::INITIALDEFINE { componentRef: cr, exp: e, source } => {
                    let mut cr2: metamodelica::Ref<DAE::ComponentRef>;
                    let mut e2: metamodelica::Ref<DAE::Exp>;
                    (e2, _) = replaceExp(e.clone(), repl, condExpFunc.clone())?;
                    let __pa0 = ::match_deref::match_deref! { match &(replaceExp(Expression::crefExp(cr.clone())?, repl, condExpFunc.clone())?) {
                        (Deref @ DAE::Exp::CREF { componentRef: __pa0, ty: _ }, _) => __pa0.clone(),
                        _ => return Err("pattern mismatch"),
                    } };
                    cr2 = metamodelica::Own::own(__pa0);
                    metamodelica::Ref::new(DAE::Element::INITIALDEFINE { componentRef: cr2.clone(), exp: e2.clone(), source: source.clone() })
                },
                Deref @ DAE::Element::EQUEQUATION { cr1: cr, cr2: cr1, source } => {
                    let mut cr2: metamodelica::Ref<DAE::ComponentRef>;
                    let mut cr1_2: metamodelica::Ref<DAE::ComponentRef>;
                    let __pa0 = ::match_deref::match_deref! { match &(replaceExp(Expression::crefExp(cr.clone())?, repl, condExpFunc.clone())?) {
                        (Deref @ DAE::Exp::CREF { componentRef: __pa0, ty: _ }, _) => __pa0.clone(),
                        _ => return Err("pattern mismatch"),
                    } };
                    cr2 = metamodelica::Own::own(__pa0);
                    let __pa2 = ::match_deref::match_deref! { match &(replaceExp(Expression::crefExp(cr1.clone())?, repl, condExpFunc.clone())?) {
                        (Deref @ DAE::Exp::CREF { componentRef: __pa2, ty: _ }, _) => __pa2.clone(),
                        _ => return Err("pattern mismatch"),
                    } };
                    cr1_2 = metamodelica::Own::own(__pa2);
                    metamodelica::Ref::new(DAE::Element::EQUEQUATION { cr1: cr2.clone(), cr2: cr1_2.clone(), source: source.clone() })
                },
                Deref @ DAE::Element::EQUATION { exp: e1, scalar: e2, source } => {
                    let mut e22: metamodelica::Ref<DAE::Exp>;
                    let mut e11: metamodelica::Ref<DAE::Exp>;
                    (e11, _) = replaceExp(e1.clone(), repl, condExpFunc.clone())?;
                    (e22, _) = replaceExp(e2.clone(), repl, condExpFunc.clone())?;
                    metamodelica::Ref::new(DAE::Element::EQUATION { exp: e11.clone(), scalar: e22.clone(), source: source.clone() })
                },
                Deref @ DAE::Element::ARRAY_EQUATION { dimension: idims, exp: e1, array: e2, source } => {
                    let mut e22: metamodelica::Ref<DAE::Exp>;
                    let mut e11: metamodelica::Ref<DAE::Exp>;
                    (e11, _) = replaceExp(e1.clone(), repl, condExpFunc.clone())?;
                    (e22, _) = replaceExp(e2.clone(), repl, condExpFunc.clone())?;
                    metamodelica::Ref::new(DAE::Element::ARRAY_EQUATION { dimension: idims.clone(), exp: e11.clone(), array: e22.clone(), source: source.clone() })
                },
                Deref @ DAE::Element::INITIAL_ARRAY_EQUATION { dimension: idims, exp: e1, array: e2, source } => {
                    let mut e22: metamodelica::Ref<DAE::Exp>;
                    let mut e11: metamodelica::Ref<DAE::Exp>;
                    (e11, _) = replaceExp(e1.clone(), repl, condExpFunc.clone())?;
                    (e22, _) = replaceExp(e2.clone(), repl, condExpFunc.clone())?;
                    metamodelica::Ref::new(DAE::Element::INITIAL_ARRAY_EQUATION { dimension: idims.clone(), exp: e11.clone(), array: e22.clone(), source: source.clone() })
                },
                Deref @ DAE::Element::WHEN_EQUATION { condition: e1, equations: elist, elsewhen_: Some(elt2), source } => {
                    let mut elist2: metamodelica::List<metamodelica::Ref<DAE::Element>>;
                    let mut e11: metamodelica::Ref<DAE::Exp>;
                    let mut elt2 = (*elt2).clone();
                    (e11, _) = replaceExp(e1.clone(), repl, condExpFunc.clone())?;
                    let __pa0 = ::match_deref::match_deref! { match &(applyReplacementsDAEElts(list![elt2.clone()], repl, condExpFunc.clone())?) {
                        Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: Deref @ metamodelica::ListNode::Nil } => __pa0.clone(),
                        _ => return Err("pattern mismatch"),
                    } };
                    elt2 = metamodelica::Own::own(__pa0);
                    elist2 = applyReplacementsDAEElts(elist.clone(), repl, condExpFunc.clone())?;
                    metamodelica::Ref::new(DAE::Element::WHEN_EQUATION { condition: e11.clone(), equations: elist2.clone(), elsewhen_: Some(elt2.clone()), source: source.clone() })
                },
                Deref @ DAE::Element::WHEN_EQUATION { condition: e1, equations: elist, elsewhen_: None, source } => {
                    let mut elist2: metamodelica::List<metamodelica::Ref<DAE::Element>>;
                    let mut e11: metamodelica::Ref<DAE::Exp>;
                    (e11, _) = replaceExp(e1.clone(), repl, condExpFunc.clone())?;
                    elist2 = applyReplacementsDAEElts(elist.clone(), repl, condExpFunc.clone())?;
                    metamodelica::Ref::new(DAE::Element::WHEN_EQUATION { condition: e11.clone(), equations: elist2.clone(), elsewhen_: None, source: source.clone() })
                },
                Deref @ DAE::Element::IF_EQUATION { condition1: conds, equations2: tbs, equations3: elist2, source } => {
                    let mut elist22: metamodelica::List<metamodelica::Ref<DAE::Element>>;
                    let mut tbs_1: metamodelica::List<metamodelica::List<metamodelica::Ref<DAE::Element>>>;
                    let mut conds_1: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                    (conds_1, _) = replaceExpList(metamodelica::AsArg::as_arg(&conds), repl, condExpFunc.clone())?;
                    tbs_1 = List::map2(tbs.clone(), &move |__a0: metamodelica::List<metamodelica::Ref<DAE::Element>>, __a1: VariableReplacements, __a2: Option<Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<bool> + 'static>>| applyReplacementsDAEElts(__a0, &__a1, __a2), repl.clone(), condExpFunc.clone())?;
                    elist22 = applyReplacementsDAEElts(elist2.clone(), repl, condExpFunc.clone())?;
                    metamodelica::Ref::new(DAE::Element::IF_EQUATION { condition1: conds_1.clone(), equations2: tbs_1.clone(), equations3: elist22.clone(), source: source.clone() })
                },
                Deref @ DAE::Element::INITIAL_IF_EQUATION { condition1: conds, equations2: tbs, equations3: elist2, source } => {
                    let mut elist22: metamodelica::List<metamodelica::Ref<DAE::Element>>;
                    let mut tbs_1: metamodelica::List<metamodelica::List<metamodelica::Ref<DAE::Element>>>;
                    let mut conds_1: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                    (conds_1, _) = replaceExpList(metamodelica::AsArg::as_arg(&conds), repl, condExpFunc.clone())?;
                    tbs_1 = List::map2(tbs.clone(), &move |__a0: metamodelica::List<metamodelica::Ref<DAE::Element>>, __a1: VariableReplacements, __a2: Option<Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<bool> + 'static>>| applyReplacementsDAEElts(__a0, &__a1, __a2), repl.clone(), condExpFunc.clone())?;
                    elist22 = applyReplacementsDAEElts(elist2.clone(), repl, condExpFunc.clone())?;
                    metamodelica::Ref::new(DAE::Element::INITIAL_IF_EQUATION { condition1: conds_1.clone(), equations2: tbs_1.clone(), equations3: elist22.clone(), source: source.clone() })
                },
                Deref @ DAE::Element::INITIALEQUATION { exp1: e1, exp2: e2, source } => {
                    let mut e22: metamodelica::Ref<DAE::Exp>;
                    let mut e11: metamodelica::Ref<DAE::Exp>;
                    (e11, _) = replaceExp(e1.clone(), repl, condExpFunc.clone())?;
                    (e22, _) = replaceExp(e2.clone(), repl, condExpFunc.clone())?;
                    metamodelica::Ref::new(DAE::Element::INITIALEQUATION { exp1: e11.clone(), exp2: e22.clone(), source: source.clone() })
                },
                Deref @ DAE::Element::ALGORITHM { algorithm_: Deref @ DAE::Algorithm { statementLst: stmts }, source } => {
                    let mut stmts2: metamodelica::List<metamodelica::Ref<DAE::Statement>>;
                    (stmts2, _) = replaceEquationsStmts(metamodelica::AsArg::as_arg(&stmts), repl, condExpFunc.clone());
                    metamodelica::Ref::new(DAE::Element::ALGORITHM { algorithm_: metamodelica::Ref::new(DAE::Algorithm { statementLst: stmts2.clone() }), source: source.clone() })
                },
                Deref @ DAE::Element::INITIALALGORITHM { algorithm_: Deref @ DAE::Algorithm { statementLst: stmts }, source } => {
                    let mut stmts2: metamodelica::List<metamodelica::Ref<DAE::Statement>>;
                    (stmts2, _) = replaceEquationsStmts(metamodelica::AsArg::as_arg(&stmts), repl, condExpFunc.clone());
                    metamodelica::Ref::new(DAE::Element::INITIALALGORITHM { algorithm_: metamodelica::Ref::new(DAE::Algorithm { statementLst: stmts2.clone() }), source: source.clone() })
                },
                Deref @ DAE::Element::COMP { ident: id, dAElist: elist, source, comment: cmt } => {
                    let mut elist = (*elist).clone();
                    elist = applyReplacementsDAEElts(elist.clone(), repl, condExpFunc.clone())?;
                    metamodelica::Ref::new(DAE::Element::COMP { ident: id.clone(), dAElist: elist.clone(), source: source.clone(), comment: cmt.clone() })
                },
                Deref @ DAE::Element::EXTOBJECTCLASS { .. } => {
                    elt.clone()
                },
                Deref @ DAE::Element::ASSERT { condition: e1, message: e2, level: e3, source } => {
                    let mut e22: metamodelica::Ref<DAE::Exp>;
                    let mut e11: metamodelica::Ref<DAE::Exp>;
                    let mut e32: metamodelica::Ref<DAE::Exp>;
                    (e11, _) = replaceExp(e1.clone(), repl, condExpFunc.clone())?;
                    (e22, _) = replaceExp(e2.clone(), repl, condExpFunc.clone())?;
                    (e32, _) = replaceExp(e3.clone(), repl, condExpFunc.clone())?;
                    metamodelica::Ref::new(DAE::Element::ASSERT { condition: e11.clone(), message: e22.clone(), level: e32.clone(), source: source.clone() })
                },
                Deref @ DAE::Element::INITIAL_ASSERT { condition: e1, message: e2, level: e3, source } => {
                    let mut e22: metamodelica::Ref<DAE::Exp>;
                    let mut e11: metamodelica::Ref<DAE::Exp>;
                    let mut e32: metamodelica::Ref<DAE::Exp>;
                    (e11, _) = replaceExp(e1.clone(), repl, condExpFunc.clone())?;
                    (e22, _) = replaceExp(e2.clone(), repl, condExpFunc.clone())?;
                    (e32, _) = replaceExp(e3.clone(), repl, condExpFunc.clone())?;
                    metamodelica::Ref::new(DAE::Element::INITIAL_ASSERT { condition: e11.clone(), message: e22.clone(), level: e32.clone(), source: source.clone() })
                },
                Deref @ DAE::Element::TERMINATE { message: e1, source } => {
                    let mut e11: metamodelica::Ref<DAE::Exp>;
                    (e11, _) = replaceExp(e1.clone(), repl, condExpFunc.clone())?;
                    metamodelica::Ref::new(DAE::Element::TERMINATE { message: e11.clone(), source: source.clone() })
                },
                Deref @ DAE::Element::INITIAL_TERMINATE { message: e1, source } => {
                    let mut e11: metamodelica::Ref<DAE::Exp>;
                    (e11, _) = replaceExp(e1.clone(), repl, condExpFunc.clone())?;
                    metamodelica::Ref::new(DAE::Element::INITIAL_TERMINATE { message: e11.clone(), source: source.clone() })
                },
                Deref @ DAE::Element::REINIT { componentRef: cr, exp: e1, source } => {
                    let mut cr2: metamodelica::Ref<DAE::ComponentRef>;
                    let mut e11: metamodelica::Ref<DAE::Exp>;
                    (e11, _) = replaceExp(e1.clone(), repl, condExpFunc.clone())?;
                    let __pa0 = ::match_deref::match_deref! { match &(replaceExp(Expression::crefExp(cr.clone())?, repl, condExpFunc.clone())?) {
                        (Deref @ DAE::Exp::CREF { componentRef: __pa0, ty: _ }, _) => __pa0.clone(),
                        _ => return Err("pattern mismatch"),
                    } };
                    cr2 = metamodelica::Own::own(__pa0);
                    metamodelica::Ref::new(DAE::Element::REINIT { componentRef: cr2.clone(), exp: e11.clone(), source: source.clone() })
                },
                Deref @ DAE::Element::COMPLEX_EQUATION { lhs: e1, rhs: e2, source } => {
                    let mut e22: metamodelica::Ref<DAE::Exp>;
                    let mut e11: metamodelica::Ref<DAE::Exp>;
                    (e11, _) = replaceExp(e1.clone(), repl, condExpFunc.clone())?;
                    (e22, _) = replaceExp(e2.clone(), repl, condExpFunc.clone())?;
                    metamodelica::Ref::new(DAE::Element::COMPLEX_EQUATION { lhs: e11.clone(), rhs: e22.clone(), source: source.clone() })
                },
                Deref @ DAE::Element::INITIAL_COMPLEX_EQUATION { lhs: e1, rhs: e2, source } => {
                    let mut e22: metamodelica::Ref<DAE::Exp>;
                    let mut e11: metamodelica::Ref<DAE::Exp>;
                    (e11, _) = replaceExp(e1.clone(), repl, condExpFunc.clone())?;
                    (e22, _) = replaceExp(e2.clone(), repl, condExpFunc.clone())?;
                    metamodelica::Ref::new(DAE::Element::INITIAL_COMPLEX_EQUATION { lhs: e11.clone(), rhs: e22.clone(), source: source.clone() })
                },
                _ => {
                    Error::addInternalError(literal!("applyReplacementsDAEElts should not fail"), metamodelica::sourceInfo!("Util/VarTransform.mo"))?;
                    return Err("fail")
                },
                _ => unreachable!("match_deref! exhaustiveness placeholder"),
            } });
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    Ok(outDae)
}

fn applyReplacementsVarAttr(
    mut attr: Option<metamodelica::Ref<DAE::VariableAttributes>>,
    mut repl: &VariableReplacements,
    mut condExpFunc: Option<Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<bool> + 'static>>,
) -> Result<Option<metamodelica::Ref<DAE::VariableAttributes>>> {
    pub type FuncTypeExp_ExpToBoolean =
        std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<bool> + 'static>;

    let mut outAttr: Option<metamodelica::Ref<DAE::VariableAttributes>>;
    outAttr = (::match_deref::match_deref! { match &(attr) {
        Some(Deref @ DAE::VariableAttributes::VAR_ATTR_REAL { quantity, unit, displayUnit, min, max, start: initial_, fixed, nominal, stateSelectOption: stateSelect, uncertainOption: unc, distributionOption: dist, equationBound: eb, isProtected: ip, finalPrefix: r#fn, startOrigin }) => {
            let mut quantity = (*quantity).clone();
            let mut unit = (*unit).clone();
            let mut displayUnit = (*displayUnit).clone();
            let mut min = (*min).clone();
            let mut max = (*max).clone();
            let mut initial_ = (*initial_).clone();
            let mut fixed = (*fixed).clone();
            let mut nominal = (*nominal).clone();
            quantity = replaceExpOpt(quantity.clone(), repl, condExpFunc.clone())?;
            unit = replaceExpOpt(unit.clone(), repl, condExpFunc.clone())?;
            displayUnit = replaceExpOpt(displayUnit.clone(), repl, condExpFunc.clone())?;
            min = replaceExpOpt(min.clone(), repl, condExpFunc.clone())?;
            max = replaceExpOpt(max.clone(), repl, condExpFunc.clone())?;
            initial_ = replaceExpOpt(initial_.clone(), repl, condExpFunc.clone())?;
            fixed = replaceExpOpt(fixed.clone(), repl, condExpFunc.clone())?;
            nominal = replaceExpOpt(nominal.clone(), repl, condExpFunc)?;
            Some(metamodelica::Ref::new(DAE::VariableAttributes::VAR_ATTR_REAL { quantity: quantity.clone(), unit: unit.clone(), displayUnit: displayUnit.clone(), min: min.clone(), max: max.clone(), start: initial_.clone(), fixed: fixed.clone(), nominal: nominal.clone(), stateSelectOption: stateSelect.clone(), uncertainOption: unc.clone(), distributionOption: dist.clone(), equationBound: eb.clone(), isProtected: ip.clone(), finalPrefix: r#fn.clone(), startOrigin: startOrigin.clone() }))
        },
        Some(Deref @ DAE::VariableAttributes::VAR_ATTR_INT { quantity, min, max, start: initial_, fixed, uncertainOption: unc, distributionOption: dist, equationBound: eb, isProtected: ip, finalPrefix: r#fn, startOrigin }) => {
            let mut quantity = (*quantity).clone();
            let mut min = (*min).clone();
            let mut max = (*max).clone();
            let mut initial_ = (*initial_).clone();
            let mut fixed = (*fixed).clone();
            quantity = replaceExpOpt(quantity.clone(), repl, condExpFunc.clone())?;
            min = replaceExpOpt(min.clone(), repl, condExpFunc.clone())?;
            max = replaceExpOpt(max.clone(), repl, condExpFunc.clone())?;
            initial_ = replaceExpOpt(initial_.clone(), repl, condExpFunc.clone())?;
            fixed = replaceExpOpt(fixed.clone(), repl, condExpFunc)?;
            Some(metamodelica::Ref::new(DAE::VariableAttributes::VAR_ATTR_INT { quantity: quantity.clone(), min: min.clone(), max: max.clone(), start: initial_.clone(), fixed: fixed.clone(), uncertainOption: unc.clone(), distributionOption: dist.clone(), equationBound: eb.clone(), isProtected: ip.clone(), finalPrefix: r#fn.clone(), startOrigin: startOrigin.clone() }))
        },
        Some(Deref @ DAE::VariableAttributes::VAR_ATTR_BOOL { quantity, start: initial_, fixed, equationBound: eb, isProtected: ip, finalPrefix: r#fn, startOrigin }) => {
            let mut quantity = (*quantity).clone();
            let mut initial_ = (*initial_).clone();
            let mut fixed = (*fixed).clone();
            quantity = replaceExpOpt(quantity.clone(), repl, condExpFunc.clone())?;
            initial_ = replaceExpOpt(initial_.clone(), repl, condExpFunc.clone())?;
            fixed = replaceExpOpt(fixed.clone(), repl, condExpFunc)?;
            Some(metamodelica::Ref::new(DAE::VariableAttributes::VAR_ATTR_BOOL { quantity: quantity.clone(), start: initial_.clone(), fixed: fixed.clone(), equationBound: eb.clone(), isProtected: ip.clone(), finalPrefix: r#fn.clone(), startOrigin: startOrigin.clone() }))
        },
        Some(Deref @ DAE::VariableAttributes::VAR_ATTR_STRING { quantity, start: initial_, fixed, equationBound: eb, isProtected: ip, finalPrefix: r#fn, startOrigin }) => {
            let mut quantity = (*quantity).clone();
            let mut initial_ = (*initial_).clone();
            let mut fixed = (*fixed).clone();
            quantity = replaceExpOpt(quantity.clone(), repl, condExpFunc.clone())?;
            initial_ = replaceExpOpt(initial_.clone(), repl, condExpFunc.clone())?;
            fixed = replaceExpOpt(fixed.clone(), repl, condExpFunc)?;
            Some(metamodelica::Ref::new(DAE::VariableAttributes::VAR_ATTR_STRING { quantity: quantity.clone(), start: initial_.clone(), fixed: fixed.clone(), equationBound: eb.clone(), isProtected: ip.clone(), finalPrefix: r#fn.clone(), startOrigin: startOrigin.clone() }))
        },
        None => {
            None
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok(outAttr)
}

pub(crate) fn applyReplacements(
    mut inVariableReplacements1: VariableReplacements,
    mut inComponentRef2: metamodelica::Ref<DAE::ComponentRef>,
    mut inComponentRef3: metamodelica::Ref<DAE::ComponentRef>,
) -> Result<(
    metamodelica::Ref<DAE::ComponentRef>,
    metamodelica::Ref<DAE::ComponentRef>,
)> {
    let mut outComponentRef1: metamodelica::Ref<DAE::ComponentRef>;
    let mut outComponentRef2: metamodelica::Ref<DAE::ComponentRef>;
    (outComponentRef1, outComponentRef2) = (::match_deref::match_deref! { match &((inVariableReplacements1, inComponentRef2, inComponentRef3)) {
        (repl, cr1, cr2) => {
            let mut cr1_1: metamodelica::Ref<DAE::ComponentRef>;
            let mut cr2_1: metamodelica::Ref<DAE::ComponentRef>;
            let __pa0 = ::match_deref::match_deref! { match &(replaceExp(Expression::crefExp(cr1.clone())?, metamodelica::AsArg::as_arg(&repl), None)?) {
                (Deref @ DAE::Exp::CREF { componentRef: __pa0, ty: _ }, _) => __pa0.clone(),
                _ => return Err("pattern mismatch"),
            } };
            cr1_1 = metamodelica::Own::own(__pa0);
            let __pa2 = ::match_deref::match_deref! { match &(replaceExp(Expression::crefExp(cr2.clone())?, metamodelica::AsArg::as_arg(&repl), None)?) {
                (Deref @ DAE::Exp::CREF { componentRef: __pa2, ty: _ }, _) => __pa2.clone(),
                _ => return Err("pattern mismatch"),
            } };
            cr2_1 = metamodelica::Own::own(__pa2);
            (cr1_1, cr2_1)
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outComponentRef1, outComponentRef2))
}

pub(crate) fn applyReplacementList(
    mut repl: &VariableReplacements,
    mut increfs: &metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
) -> Result<metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>> {
    let mut ocrefs: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
    ocrefs = (::match_deref::match_deref! { match increfs {
        Deref @ metamodelica::ListNode::Nil => {
            metamodelica::nil()
        },
        Deref @ metamodelica::ListNode::Cons { head: cr1, tail: __esc_ocrefs } => {
            ocrefs = (*__esc_ocrefs).clone();
            let mut cr1_1: metamodelica::Ref<DAE::ComponentRef>;
            let __pa0 = ::match_deref::match_deref! { match &(replaceExp(Expression::crefExp(cr1.clone())?, repl, None)?) {
                (Deref @ DAE::Exp::CREF { componentRef: __pa0, ty: _ }, _) => __pa0.clone(),
                _ => return Err("pattern mismatch"),
            } };
            cr1_1 = metamodelica::Own::own(__pa0);
            ocrefs = applyReplacementList(repl, metamodelica::AsArg::as_arg(&ocrefs))?;
            metamodelica::cons(cr1_1, ocrefs.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(ocrefs)
}

pub(crate) fn applyReplacementsExp(
    mut repl: &VariableReplacements,
    mut inExp1: metamodelica::Ref<DAE::Exp>,
    mut inExp2: metamodelica::Ref<DAE::Exp>,
) -> Result<(metamodelica::Ref<DAE::Exp>, metamodelica::Ref<DAE::Exp>)> {
    let mut outExp1: metamodelica::Ref<DAE::Exp>;
    let mut outExp2: metamodelica::Ref<DAE::Exp>;
    (outExp1, outExp2) = (::match_deref::match_deref! { match &((inExp1, inExp2)) {
        (e1, e2) => {
            let mut e1 = (*e1).clone();
            let mut e2 = (*e2).clone();
            (e1, _) = replaceExp(e1.clone(), repl, None)?;
            (e2, _) = replaceExp(e2.clone(), repl, None)?;
            (e1, _) = ExpressionSimplify::simplify1(e1.clone())?;
            (e2, _) = ExpressionSimplify::simplify1(e2.clone())?;
            (e1.clone(), e2.clone())
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outExp1, outExp2))
}

pub(crate) fn emptyReplacementsArray(mut n: i32) -> Result<metamodelica::Array<VariableReplacements>> {
    let mut repl: metamodelica::Array<VariableReplacements>;
    repl = metamodelica::arrayFromVec(emptyReplacementsArray2(n)?.into_iter().cloned().collect());
    Ok(repl)
}

fn emptyReplacementsArray2(mut n: i32) -> Result<metamodelica::List<VariableReplacements>> {
    let mut replLst: metamodelica::List<VariableReplacements> = metamodelica::nil();
    replLst = 'mc: {
        let __mc_input = n;
        if let Ok(__v) = (|| -> Result<_> {
            let 0 = __mc_input.clone() else { return Err("nomatch") };
            Ok(metamodelica::nil())
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            let true = (n < 0) else { return Err("pattern mismatch") };
            metamodelica::print(literal!(
                "Internal error, emptyReplacementsArray2 called with negative n!"
            ));
            Ok(return Err("fail"))
        })() {
            break 'mc __v;
        }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            let mut r: VariableReplacements;
            let mut replLst: metamodelica::List<VariableReplacements> = replLst.clone();
            let true = (n > 0) else { return Err("pattern mismatch") };
            r = emptyReplacements();
            replLst = emptyReplacementsArray2(n - 1)?;
            Ok((metamodelica::cons(r.clone(), replLst.clone()), replLst.clone()))
        })() {
            replLst = __wb0;
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(replLst)
}

pub fn emptyReplacements() -> VariableReplacements {
    let mut outVariableReplacements: VariableReplacements;
    outVariableReplacements = (match () {
        () => {
            let mut ht: (
                metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
                (
                    i32,
                    i32,
                    metamodelica::Array<Option<(metamodelica::Ref<DAE::ComponentRef>, metamodelica::Ref<DAE::Exp>)>>,
                ),
                i32,
                (
                    HashTable2::FuncHashCref,
                    HashTable2::FuncCrefEqual,
                    HashTable2::FuncCrefStr,
                    HashTable2::FuncExpStr,
                ),
            );
            let mut invHt: (
                metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
                (
                    i32,
                    i32,
                    metamodelica::Array<
                        Option<(
                            metamodelica::Ref<DAE::ComponentRef>,
                            metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
                        )>,
                    >,
                ),
                i32,
                (
                    HashTable3::FuncHashCref,
                    HashTable3::FuncCrefEqual,
                    HashTable3::FuncCrefStr,
                    HashTable3::FuncExpStr,
                ),
            );
            ht = HashTable2::emptyHashTable();
            invHt = HashTable3::emptyHashTable();
            VariableReplacements {
                hashTable: ht,
                invHashTable: invHt,
            }
        }
    });
    outVariableReplacements
}

pub(crate) fn emptyReplacementsSized(mut size: i32) -> VariableReplacements {
    let mut outVariableReplacements: VariableReplacements;
    outVariableReplacements = (match size {
        _ => {
            let mut ht: (
                metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
                (
                    i32,
                    i32,
                    metamodelica::Array<Option<(metamodelica::Ref<DAE::ComponentRef>, metamodelica::Ref<DAE::Exp>)>>,
                ),
                i32,
                (
                    HashTable2::FuncHashCref,
                    HashTable2::FuncCrefEqual,
                    HashTable2::FuncCrefStr,
                    HashTable2::FuncExpStr,
                ),
            );
            let mut invHt: (
                metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
                (
                    i32,
                    i32,
                    metamodelica::Array<
                        Option<(
                            metamodelica::Ref<DAE::ComponentRef>,
                            metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
                        )>,
                    >,
                ),
                i32,
                (
                    HashTable3::FuncHashCref,
                    HashTable3::FuncCrefEqual,
                    HashTable3::FuncCrefStr,
                    HashTable3::FuncExpStr,
                ),
            );
            ht = HashTable2::emptyHashTableSized(size);
            invHt = HashTable3::emptyHashTableSized(size);
            VariableReplacements {
                hashTable: ht,
                invHashTable: invHt,
            }
        }
    });
    outVariableReplacements
}

pub(crate) fn replaceEquationsStmts(
    mut inAlgorithmStatementLst: &metamodelica::List<metamodelica::Ref<DAE::Statement>>,
    mut repl: &VariableReplacements,
    mut condExpFunc: Option<Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<bool> + 'static>>,
) -> (metamodelica::List<metamodelica::Ref<DAE::Statement>>, bool) {
    pub type FuncTypeExp_ExpToBoolean =
        std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<bool> + 'static>;

    let mut outAlgorithmStatementLst: metamodelica::List<metamodelica::Ref<DAE::Statement>>;
    let mut replacementPerformed: bool;
    (outAlgorithmStatementLst, replacementPerformed) = 'mc: {
        let __mc_input = &**inAlgorithmStatementLst;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Nil => {
                    Ok((metamodelica::nil(), false))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Statement::STMT_ASSIGN { type_: tp, exp1: e2, exp: e, source }, tail: xs } => {
                    let mut e_1: metamodelica::Ref<DAE::Exp>;
                    let mut e_2: metamodelica::Ref<DAE::Exp>;
                    let mut xs_1: metamodelica::List<metamodelica::Ref<DAE::Statement>>;
                    let mut b1: bool;
                    let mut b2: bool;
                    (e_1, b1) = replaceExp(e.clone(), repl, condExpFunc.clone())?;
                    (e_2, b2) = replaceExp(e2.clone(), repl, condExpFunc.clone())?;
                    let true = (b1 || b2) else { return Err("pattern mismatch") };
                    (xs_1, _) = replaceEquationsStmts(metamodelica::AsArg::as_arg(&xs), repl, condExpFunc.clone());
                    Ok((metamodelica::cons(metamodelica::Ref::new(DAE::Statement::STMT_ASSIGN { type_: tp.clone(), exp1: e_2.clone(), exp: e_1.clone(), source: source.clone() }), xs_1.clone()), true))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Statement::STMT_TUPLE_ASSIGN { type_: tp, expExpLst: expl1, exp: e, source }, tail: xs } => {
                    let mut e_1: metamodelica::Ref<DAE::Exp>;
                    let mut expl2: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                    let mut xs_1: metamodelica::List<metamodelica::Ref<DAE::Statement>>;
                    let mut b1: bool;
                    let mut b2: bool;
                    (e_1, b1) = replaceExp(e.clone(), repl, condExpFunc.clone())?;
                    (expl2, b2) = replaceExpList(metamodelica::AsArg::as_arg(&expl1), repl, condExpFunc.clone())?;
                    let true = (b1 || b2) else { return Err("pattern mismatch") };
                    (xs_1, _) = replaceEquationsStmts(metamodelica::AsArg::as_arg(&xs), repl, condExpFunc.clone());
                    Ok((metamodelica::cons(metamodelica::Ref::new(DAE::Statement::STMT_TUPLE_ASSIGN { type_: tp.clone(), expExpLst: expl2.clone(), exp: e_1.clone(), source: source.clone() }), xs_1.clone()), true))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Statement::STMT_ASSIGN_ARR { type_: tp, lhs: e1, exp: e2, source }, tail: xs } => {
                    let mut e_1: metamodelica::Ref<DAE::Exp>;
                    let mut e_2: metamodelica::Ref<DAE::Exp>;
                    let mut xs_1: metamodelica::List<metamodelica::Ref<DAE::Statement>>;
                    let mut b1: bool;
                    let mut b2: bool;
                    (e_1, b1) = replaceExp(e1.clone(), repl, condExpFunc.clone())?;
                    (e_2, b2) = replaceExp(e2.clone(), repl, condExpFunc.clone())?;
                    let true = (b1 || b2) else { return Err("pattern mismatch") };
                    (xs_1, _) = replaceEquationsStmts(metamodelica::AsArg::as_arg(&xs), repl, condExpFunc.clone());
                    Ok((metamodelica::cons(metamodelica::Ref::new(DAE::Statement::STMT_ASSIGN_ARR { type_: tp.clone(), lhs: e_1.clone(), exp: e_2.clone(), source: source.clone() }), xs_1.clone()), true))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Statement::STMT_IF { exp: e, statementLst: stmts, else_: el, source }, tail: xs } => {
                    let mut e_1: metamodelica::Ref<DAE::Exp>;
                    let mut xs_1: metamodelica::List<metamodelica::Ref<DAE::Statement>>;
                    let mut stmts2: metamodelica::List<metamodelica::Ref<DAE::Statement>>;
                    let mut b1: bool;
                    let mut b2: bool;
                    let mut b3: bool;
                    let mut el_1: metamodelica::Ref<DAE::Else>;
                    (el_1, b1) = replaceEquationsElse(metamodelica::AsArg::as_arg(&el), repl, condExpFunc.clone());
                    (stmts2, b2) = replaceEquationsStmts(metamodelica::AsArg::as_arg(&stmts), repl, condExpFunc.clone());
                    (e_1, b3) = replaceExp(e.clone(), repl, condExpFunc.clone())?;
                    let true = (b1 || b2 || b3) else { return Err("pattern mismatch") };
                    (xs_1, _) = replaceEquationsStmts(metamodelica::AsArg::as_arg(&xs), repl, condExpFunc.clone());
                    Ok((metamodelica::cons(metamodelica::Ref::new(DAE::Statement::STMT_IF { exp: e_1.clone(), statementLst: stmts2.clone(), else_: el_1.clone(), source: source.clone() }), xs_1.clone()), true))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Statement::STMT_FOR { type_: tp, iterIsArray, iter: id1, range: e, statementLst: stmts, source, sub_iters }, tail: xs } => {
                    let mut e_1: metamodelica::Ref<DAE::Exp>;
                    let mut xs_1: metamodelica::List<metamodelica::Ref<DAE::Statement>>;
                    let mut stmts2: metamodelica::List<metamodelica::Ref<DAE::Statement>>;
                    let mut b1: bool;
                    let mut b2: bool;
                    (stmts2, b1) = replaceEquationsStmts(metamodelica::AsArg::as_arg(&stmts), repl, condExpFunc.clone());
                    (e_1, b2) = replaceExp(e.clone(), repl, condExpFunc.clone())?;
                    let true = (b1 || b2) else { return Err("pattern mismatch") };
                    (xs_1, _) = replaceEquationsStmts(metamodelica::AsArg::as_arg(&xs), repl, condExpFunc.clone());
                    Ok((metamodelica::cons(metamodelica::Ref::new(DAE::Statement::STMT_FOR { type_: tp.clone(), iterIsArray: iterIsArray.clone(), iter: id1.clone(), range: e_1.clone(), statementLst: stmts2.clone(), source: source.clone(), sub_iters: sub_iters.clone() }), xs_1.clone()), true))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Statement::STMT_WHILE { exp: e, statementLst: stmts, source }, tail: xs } => {
                    let mut e_1: metamodelica::Ref<DAE::Exp>;
                    let mut xs_1: metamodelica::List<metamodelica::Ref<DAE::Statement>>;
                    let mut stmts2: metamodelica::List<metamodelica::Ref<DAE::Statement>>;
                    let mut b1: bool;
                    let mut b2: bool;
                    (stmts2, b1) = replaceEquationsStmts(metamodelica::AsArg::as_arg(&stmts), repl, condExpFunc.clone());
                    (e_1, b2) = replaceExp(e.clone(), repl, condExpFunc.clone())?;
                    let true = (b1 || b2) else { return Err("pattern mismatch") };
                    (xs_1, _) = replaceEquationsStmts(metamodelica::AsArg::as_arg(&xs), repl, condExpFunc.clone());
                    Ok((metamodelica::cons(metamodelica::Ref::new(DAE::Statement::STMT_WHILE { exp: e_1.clone(), statementLst: stmts2.clone(), source: source.clone() }), xs_1.clone()), true))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Statement::STMT_WHEN { exp: e, conditions, initialCall, statementLst: stmts, elseWhen: ew, source }, tail: xs } => {
                    let mut e_1: metamodelica::Ref<DAE::Exp>;
                    let mut xs_1: metamodelica::List<metamodelica::Ref<DAE::Statement>>;
                    let mut stmts2: metamodelica::List<metamodelica::Ref<DAE::Statement>>;
                    let mut b1: bool;
                    let mut b2: bool;
                    let mut b3: bool;
                    let mut ew_1: Option<metamodelica::Ref<DAE::Statement>>;
                    (ew_1, b1) = replaceOptEquationsStmts(ew.clone(), repl, condExpFunc.clone());
                    (stmts2, b2) = replaceEquationsStmts(metamodelica::AsArg::as_arg(&stmts), repl, condExpFunc.clone());
                    (e_1, b3) = replaceExp(e.clone(), repl, condExpFunc.clone())?;
                    let true = (b1 || b2 || b3) else { return Err("pattern mismatch") };
                    (xs_1, _) = replaceEquationsStmts(metamodelica::AsArg::as_arg(&xs), repl, condExpFunc.clone());
                    Ok((metamodelica::cons(metamodelica::Ref::new(DAE::Statement::STMT_WHEN { exp: e_1.clone(), conditions: conditions.clone(), initialCall: initialCall.clone(), statementLst: stmts2.clone(), elseWhen: ew_1.clone(), source: source.clone() }), xs_1.clone()), true))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Statement::STMT_ASSERT { cond: e, msg: e2, level: e3, source }, tail: xs } => {
                    let mut e_1: metamodelica::Ref<DAE::Exp>;
                    let mut e_2: metamodelica::Ref<DAE::Exp>;
                    let mut e_3: metamodelica::Ref<DAE::Exp>;
                    let mut xs_1: metamodelica::List<metamodelica::Ref<DAE::Statement>>;
                    let mut b1: bool;
                    let mut b2: bool;
                    let mut b3: bool;
                    (e_1, b1) = replaceExp(e.clone(), repl, condExpFunc.clone())?;
                    (e_2, b2) = replaceExp(e2.clone(), repl, condExpFunc.clone())?;
                    (e_3, b3) = replaceExp(e3.clone(), repl, condExpFunc.clone())?;
                    let true = (b1 || b2 || b3) else { return Err("pattern mismatch") };
                    (xs_1, _) = replaceEquationsStmts(metamodelica::AsArg::as_arg(&xs), repl, condExpFunc.clone());
                    Ok((metamodelica::cons(metamodelica::Ref::new(DAE::Statement::STMT_ASSERT { cond: e_1.clone(), msg: e_2.clone(), level: e_3.clone(), source: source.clone() }), xs_1.clone()), true))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Statement::STMT_TERMINATE { msg: e, source }, tail: xs } => {
                    let mut e_1: metamodelica::Ref<DAE::Exp>;
                    let mut xs_1: metamodelica::List<metamodelica::Ref<DAE::Statement>>;
                    let __pa0 = ::match_deref::match_deref! { match &(replaceExp(e.clone(), repl, condExpFunc.clone())?) {
                        (__pa0, true) => __pa0.clone(),
                        _ => return Err("pattern mismatch"),
                    } };
                    e_1 = metamodelica::Own::own(__pa0);
                    (xs_1, _) = replaceEquationsStmts(metamodelica::AsArg::as_arg(&xs), repl, condExpFunc.clone());
                    Ok((metamodelica::cons(metamodelica::Ref::new(DAE::Statement::STMT_TERMINATE { msg: e_1.clone(), source: source.clone() }), xs_1.clone()), true))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Statement::STMT_REINIT { var: e, value: e2, source }, tail: xs } => {
                    let mut e_1: metamodelica::Ref<DAE::Exp>;
                    let mut e_2: metamodelica::Ref<DAE::Exp>;
                    let mut xs_1: metamodelica::List<metamodelica::Ref<DAE::Statement>>;
                    let mut b1: bool;
                    let mut b2: bool;
                    (e_1, b1) = replaceExp(e.clone(), repl, condExpFunc.clone())?;
                    (e_2, b2) = replaceExp(e2.clone(), repl, condExpFunc.clone())?;
                    let true = (b1 || b2) else { return Err("pattern mismatch") };
                    (xs_1, _) = replaceEquationsStmts(metamodelica::AsArg::as_arg(&xs), repl, condExpFunc.clone());
                    Ok((metamodelica::cons(metamodelica::Ref::new(DAE::Statement::STMT_REINIT { var: e_1.clone(), value: e_2.clone(), source: source.clone() }), xs_1.clone()), true))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Statement::STMT_NORETCALL { exp: e, source }, tail: xs } => {
                    let mut e_1: metamodelica::Ref<DAE::Exp>;
                    let mut xs_1: metamodelica::List<metamodelica::Ref<DAE::Statement>>;
                    let __pa0 = ::match_deref::match_deref! { match &(replaceExp(e.clone(), repl, condExpFunc.clone())?) {
                        (__pa0, true) => __pa0.clone(),
                        _ => return Err("pattern mismatch"),
                    } };
                    e_1 = metamodelica::Own::own(__pa0);
                    (xs_1, _) = replaceEquationsStmts(metamodelica::AsArg::as_arg(&xs), repl, condExpFunc.clone());
                    Ok((metamodelica::cons(metamodelica::Ref::new(DAE::Statement::STMT_NORETCALL { exp: e_1.clone(), source: source.clone() }), xs_1.clone()), true))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ metamodelica::ListNode::Cons { head: x, tail: xs } => {
                    let mut xs_1: metamodelica::List<metamodelica::Ref<DAE::Statement>>;
                    let mut b1: bool;
                    (xs_1, b1) = replaceEquationsStmts(metamodelica::AsArg::as_arg(&xs), repl, condExpFunc.clone());
                    Ok((metamodelica::cons(x.clone(), xs_1.clone()), b1))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    (outAlgorithmStatementLst, replacementPerformed)
}

fn replaceEquationsElse(
    mut inElse: &metamodelica::Ref<DAE::Else>,
    mut repl: &VariableReplacements,
    mut condExpFunc: Option<Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<bool> + 'static>>,
) -> (metamodelica::Ref<DAE::Else>, bool) {
    pub type FuncTypeExp_ExpToBoolean =
        std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<bool> + 'static>;

    let mut outElse: metamodelica::Ref<DAE::Else>;
    let mut replacementPerformed: bool;
    (outElse, replacementPerformed) = 'mc: {
        let __mc_input = &**inElse;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Else::ELSEIF { exp: e, statementLst: st, else_: el } => {
                    let mut e_1: metamodelica::Ref<DAE::Exp>;
                    let mut st_1: metamodelica::List<metamodelica::Ref<DAE::Statement>>;
                    let mut el_1: metamodelica::Ref<DAE::Else>;
                    let mut b1: bool;
                    let mut b2: bool;
                    let mut b3: bool;
                    (el_1, b1) = replaceEquationsElse(metamodelica::AsArg::as_arg(&el), repl, condExpFunc.clone());
                    (st_1, b2) = replaceEquationsStmts(metamodelica::AsArg::as_arg(&st), repl, condExpFunc.clone());
                    (e_1, b3) = replaceExp(e.clone(), repl, condExpFunc.clone())?;
                    let true = (b1 || b2 || b3) else { return Err("pattern mismatch") };
                    Ok((metamodelica::Ref::new(DAE::Else::ELSEIF { exp: e_1.clone(), statementLst: st_1.clone(), else_: el_1.clone() }), true))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Else::ELSE { statementLst: st } => {
                    let mut st_1: metamodelica::List<metamodelica::Ref<DAE::Statement>>;
                    let __pa0 = ::match_deref::match_deref! { match &(replaceEquationsStmts(metamodelica::AsArg::as_arg(&st), repl, condExpFunc.clone())) {
                        (__pa0, true) => __pa0.clone(),
                        _ => return Err("pattern mismatch"),
                    } };
                    st_1 = metamodelica::Own::own(__pa0);
                    Ok((metamodelica::Ref::new(DAE::Else::ELSE { statementLst: st_1.clone() }), true))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((inElse.clone(), false))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    (outElse, replacementPerformed)
}

fn replaceOptEquationsStmts(
    mut optStmt: Option<metamodelica::Ref<DAE::Statement>>,
    mut inVariableReplacements: &VariableReplacements,
    mut condExpFunc: Option<Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<bool> + 'static>>,
) -> (Option<metamodelica::Ref<DAE::Statement>>, bool) {
    pub type FuncTypeExp_ExpToBoolean =
        std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<bool> + 'static>;

    let mut outAlgorithmStatementLst: Option<metamodelica::Ref<DAE::Statement>>;
    let mut replacementPerformed: bool;
    (outAlgorithmStatementLst, replacementPerformed) = 'mc: {
        let __mc_input = &optStmt;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Some(stmt) => {
                    let mut stmt2: metamodelica::Ref<DAE::Statement>;
                    let __pa0 = ::match_deref::match_deref! { match &(replaceEquationsStmts(&(list![stmt.clone()]), inVariableReplacements, condExpFunc.clone())) {
                        (Deref @ metamodelica::ListNode::Cons { head: __pa0, tail: Deref @ metamodelica::ListNode::Nil }, true) => __pa0.clone(),
                        _ => return Err("pattern mismatch"),
                    } };
                    stmt2 = metamodelica::Own::own(__pa0);
                    Ok((Some(stmt2.clone()), true))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((optStmt.clone(), false))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    (outAlgorithmStatementLst, replacementPerformed)
}

pub(crate) fn dumpReplacements(mut inVariableReplacements: &VariableReplacements) -> Result<()> {
    let () = (match inVariableReplacements.clone() {
        VariableReplacements { hashTable: mut ht, .. } => {
            let mut r#str: ArcStr;
            let mut len_str: ArcStr;
            let mut len: i32;
            let mut tplLst: metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, metamodelica::Ref<DAE::Exp>)>;
            tplLst = BaseHashTable::hashTableList(&(ht.clone()))?;
            r#str = stringDelimitList(List::map(tplLst.clone(), &printReplacementTupleStr)?, literal!("\n"));
            metamodelica::print(literal!("Replacements: ("));
            len = ((tplLst).len() as i32);
            len_str = intString(len);
            metamodelica::print(len_str);
            metamodelica::print(literal!(")\n"));
            metamodelica::print(literal!("=============\n"));
            metamodelica::print(r#str);
            metamodelica::print(literal!("\n"));
            ()
        }
    });
    Ok(())
}

pub(crate) fn dumpReplacementsStr(mut inVariableReplacements: &VariableReplacements) -> Result<ArcStr> {
    let mut ostr: ArcStr;
    ostr = (match inVariableReplacements.clone() {
        VariableReplacements { hashTable: mut ht, .. } => {
            let mut r#str: ArcStr;
            let mut s1: ArcStr;
            let mut tplLst: metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, metamodelica::Ref<DAE::Exp>)>;
            tplLst = BaseHashTable::hashTableList(&(ht.clone()))?;
            r#str = stringDelimitList(List::map(tplLst.clone(), &printReplacementTupleStr)?, literal!("\n"));
            s1 = {
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("Replacements: ("));
                __mm_s.push_str(&*intString(((tplLst).len() as i32)));
                __mm_s.push_str(&*literal!(")\n=============\n"));
                __mm_s.push_str(&*r#str);
                __mm_s.push_str(&*literal!("\n"));
                ArcStr::from(__mm_s)
            };
            s1
        }
    });
    Ok(ostr)
}

pub(crate) fn getAllReplacements(
    mut inVariableReplacements: &VariableReplacements,
) -> Result<(
    metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
    metamodelica::List<metamodelica::Ref<DAE::Exp>>,
)> {
    let mut crefs: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
    let mut dsts: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
    (crefs, dsts) = (match inVariableReplacements.clone() {
        VariableReplacements { hashTable: mut ht, .. } => {
            let mut tplLst: metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, metamodelica::Ref<DAE::Exp>)>;
            tplLst = BaseHashTable::hashTableList(&(ht.clone()))?;
            crefs = List::map(tplLst.clone(), &fnptr!(Util::tuple21, _))?;
            dsts = List::map(tplLst, &fnptr!(Util::tuple22, _))?;
            (crefs, dsts)
        }
    });
    Ok((crefs, dsts))
}

fn printReplacementTupleStr(
    mut tpl: (metamodelica::Ref<DAE::ComponentRef>, metamodelica::Ref<DAE::Exp>),
) -> Result<ArcStr> {
    let mut r#str: ArcStr;
    r#str = {
        let mut __mm_s = String::new();
        __mm_s.push_str(&*ComponentReferenceBasics::printComponentRefStr(
            &(Util::tuple21(tpl.clone())),
        )?);
        __mm_s.push_str(&*literal!(" -> "));
        __mm_s.push_str(&*ExpressionBasics::printExpStr(Util::tuple22(tpl))?);
        ArcStr::from(__mm_s)
    };
    Ok(r#str)
}

pub(crate) fn replacementSources(
    mut repl: &VariableReplacements,
) -> Result<metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>> {
    let mut sources: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
    sources = (match repl.clone() {
        VariableReplacements {
            hashTable: mut ht,
            invHashTable: _,
        } => {
            sources = BaseHashTable::hashTableKeyList(&(ht.clone()))?;
            sources
        }
    });
    Ok(sources)
}

pub(crate) fn replacementTargets(
    mut repl: &VariableReplacements,
) -> Result<metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>> {
    let mut sources: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
    sources = (match repl.clone() {
        VariableReplacements {
            hashTable: mut ht,
            invHashTable: _,
        } => {
            let mut targets: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
            let mut targets2: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
            targets = BaseHashTable::hashTableValueList(&(ht.clone()))?;
            targets2 = List::flatten(List::map(targets, &Expression::extractCrefsFromExp)?)?;
            targets2
        }
    });
    Ok(sources)
}

pub fn addReplacementLst(
    mut inRepl: VariableReplacements,
    mut crs: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
    mut dsts: metamodelica::List<metamodelica::Ref<DAE::Exp>>,
) -> Result<VariableReplacements> {
    let mut repl: VariableReplacements;
    repl = (::match_deref::match_deref! { match &((inRepl, crs, dsts)) {
        (__esc_repl, Deref @ metamodelica::ListNode::Nil, Deref @ metamodelica::ListNode::Nil) => {
            repl = (*__esc_repl).clone();
            repl.clone()
        },
        (__esc_repl, Deref @ metamodelica::ListNode::Cons { head: cr, tail: crrest }, Deref @ metamodelica::ListNode::Cons { head: dst, tail: dstrest }) => {
            repl = (*__esc_repl).clone();
            repl = addReplacement(repl.clone(), cr.clone(), dst.clone())?;
            repl = addReplacementLst(repl.clone(), crrest.clone(), dstrest.clone())?;
            repl.clone()
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok(repl)
}

pub fn addReplacement(
    mut repl: VariableReplacements,
    mut inSrc: metamodelica::Ref<DAE::ComponentRef>,
    mut inDst: metamodelica::Ref<DAE::Exp>,
) -> Result<VariableReplacements> {
    let mut outRepl: VariableReplacements;
    outRepl = 'mc: {
        let __mc_input = (&repl, inSrc, inDst);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (VariableReplacements { .. }, src, dst) => {
                    let mut src_1: metamodelica::Ref<DAE::ComponentRef>;
                    let mut dst_1: metamodelica::Ref<DAE::Exp>;
                    let mut ht: (metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>, (i32, i32, metamodelica::Array<Option<(metamodelica::Ref<DAE::ComponentRef>, metamodelica::Ref<DAE::Exp>)>>), i32, (HashTable2::FuncHashCref, HashTable2::FuncCrefEqual, HashTable2::FuncCrefStr, HashTable2::FuncExpStr));
                    let mut ht_1: (metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>, (i32, i32, metamodelica::Array<Option<(metamodelica::Ref<DAE::ComponentRef>, metamodelica::Ref<DAE::Exp>)>>), i32, (HashTable2::FuncHashCref, HashTable2::FuncCrefEqual, HashTable2::FuncCrefStr, HashTable2::FuncExpStr));
                    let mut invHt: (metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>, (i32, i32, metamodelica::Array<Option<(metamodelica::Ref<DAE::ComponentRef>, metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>)>>), i32, (HashTable3::FuncHashCref, HashTable3::FuncCrefEqual, HashTable3::FuncCrefStr, HashTable3::FuncExpStr));
                    let mut invHt_1: (metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>, (i32, i32, metamodelica::Array<Option<(metamodelica::Ref<DAE::ComponentRef>, metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>)>>), i32, (HashTable3::FuncHashCref, HashTable3::FuncCrefEqual, HashTable3::FuncCrefStr, HashTable3::FuncExpStr));
                    let (VariableReplacements { hashTable: __pa0, invHashTable: __pa1 }, __pa2, __pa3) = makeTransitive(repl.clone(), src.clone(), dst.clone())?;
                    ht = metamodelica::Own::own(__pa0);
                    invHt = metamodelica::Own::own(__pa1);
                    src_1 = metamodelica::Own::own(__pa2);
                    dst_1 = metamodelica::Own::own(__pa3);
                    ht_1 = BaseHashTable::add((src_1.clone(), dst_1.clone()), ht.clone())?;
                    invHt_1 = addReplacementInv(invHt.clone(), src_1.clone(), dst_1.clone())?;
                    Ok(VariableReplacements { hashTable: ht_1.clone(), invHashTable: invHt_1.clone() })
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    metamodelica::print(literal!("-add_replacement failed\n"));
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(outRepl)
}

pub(crate) fn addReplacementNoTransitive(
    mut repl: VariableReplacements,
    mut src: metamodelica::Ref<DAE::ComponentRef>,
    mut dst: metamodelica::Ref<DAE::Exp>,
) -> Result<VariableReplacements> {
    let mut outRepl: VariableReplacements = repl;
    let mut ht: (
        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<Option<(metamodelica::Ref<DAE::ComponentRef>, metamodelica::Ref<DAE::Exp>)>>,
        ),
        i32,
        (
            HashTable2::FuncHashCref,
            HashTable2::FuncCrefEqual,
            HashTable2::FuncCrefStr,
            HashTable2::FuncExpStr,
        ),
    );
    let mut invHt: (
        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<
                Option<(
                    metamodelica::Ref<DAE::ComponentRef>,
                    metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
                )>,
            >,
        ),
        i32,
        (
            HashTable3::FuncHashCref,
            HashTable3::FuncCrefEqual,
            HashTable3::FuncCrefStr,
            HashTable3::FuncExpStr,
        ),
    );
    let VariableReplacements {
        hashTable: __pa0,
        invHashTable: __pa1,
    } = outRepl;
    ht = metamodelica::Own::own(__pa0);
    invHt = metamodelica::Own::own(__pa1);
    ht = BaseHashTable::add((src.clone(), dst.clone()), ht)?;
    invHt = addReplacementInv(invHt, src, dst)?;
    outRepl = VariableReplacements {
        hashTable: ht,
        invHashTable: invHt,
    };
    Ok(outRepl)
}

fn addReplacementInv(
    mut invHt: (
        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<
                Option<(
                    metamodelica::Ref<DAE::ComponentRef>,
                    metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
                )>,
            >,
        ),
        i32,
        (
            Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>) -> Result<i32> + 'static>,
            Arc<
                dyn ::std::ops::Fn(
                        metamodelica::Ref<DAE::ComponentRef>,
                        metamodelica::Ref<DAE::ComponentRef>,
                    ) -> Result<bool>
                    + 'static,
            >,
            Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>) -> Result<ArcStr> + 'static>,
            Arc<
                dyn ::std::ops::Fn(metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>) -> Result<ArcStr>
                    + 'static,
            >,
        ),
    ),
    mut src: metamodelica::Ref<DAE::ComponentRef>,
    mut dst: metamodelica::Ref<DAE::Exp>,
) -> Result<(
    metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
    (
        i32,
        i32,
        metamodelica::Array<
            Option<(
                metamodelica::Ref<DAE::ComponentRef>,
                metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
            )>,
        >,
    ),
    i32,
    (
        Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>) -> Result<i32> + 'static>,
        Arc<
            dyn ::std::ops::Fn(
                    metamodelica::Ref<DAE::ComponentRef>,
                    metamodelica::Ref<DAE::ComponentRef>,
                ) -> Result<bool>
                + 'static,
        >,
        Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>) -> Result<ArcStr> + 'static>,
        Arc<dyn ::std::ops::Fn(metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>) -> Result<ArcStr> + 'static>,
    ),
)> {
    let mut outInvHt: (
        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<
                Option<(
                    metamodelica::Ref<DAE::ComponentRef>,
                    metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
                )>,
            >,
        ),
        i32,
        (
            HashTable3::FuncHashCref,
            HashTable3::FuncCrefEqual,
            HashTable3::FuncCrefStr,
            HashTable3::FuncExpStr,
        ),
    );
    outInvHt = (match &*dst {
        _ => {
            let mut invHt_1: (
                metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
                (
                    i32,
                    i32,
                    metamodelica::Array<
                        Option<(
                            metamodelica::Ref<DAE::ComponentRef>,
                            metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
                        )>,
                    >,
                ),
                i32,
                (
                    HashTable3::FuncHashCref,
                    HashTable3::FuncCrefEqual,
                    HashTable3::FuncCrefStr,
                    HashTable3::FuncExpStr,
                ),
            );
            let mut dests: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
            dests = Expression::extractCrefsFromExp(dst)?;
            invHt_1 = List::fold1r(&dests, &addReplacementInv2, src, invHt)?;
            invHt_1
        }
    });
    Ok(outInvHt)
}

fn addReplacementInv2(
    mut invHt: (
        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<
                Option<(
                    metamodelica::Ref<DAE::ComponentRef>,
                    metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
                )>,
            >,
        ),
        i32,
        (
            Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>) -> Result<i32> + 'static>,
            Arc<
                dyn ::std::ops::Fn(
                        metamodelica::Ref<DAE::ComponentRef>,
                        metamodelica::Ref<DAE::ComponentRef>,
                    ) -> Result<bool>
                    + 'static,
            >,
            Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>) -> Result<ArcStr> + 'static>,
            Arc<
                dyn ::std::ops::Fn(metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>) -> Result<ArcStr>
                    + 'static,
            >,
        ),
    ),
    mut dst: metamodelica::Ref<DAE::ComponentRef>,
    mut src: metamodelica::Ref<DAE::ComponentRef>,
) -> Result<(
    metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
    (
        i32,
        i32,
        metamodelica::Array<
            Option<(
                metamodelica::Ref<DAE::ComponentRef>,
                metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
            )>,
        >,
    ),
    i32,
    (
        Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>) -> Result<i32> + 'static>,
        Arc<
            dyn ::std::ops::Fn(
                    metamodelica::Ref<DAE::ComponentRef>,
                    metamodelica::Ref<DAE::ComponentRef>,
                ) -> Result<bool>
                + 'static,
        >,
        Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>) -> Result<ArcStr> + 'static>,
        Arc<dyn ::std::ops::Fn(metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>) -> Result<ArcStr> + 'static>,
    ),
)> {
    let mut outInvHt: (
        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<
                Option<(
                    metamodelica::Ref<DAE::ComponentRef>,
                    metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
                )>,
            >,
        ),
        i32,
        (
            HashTable3::FuncHashCref,
            HashTable3::FuncCrefEqual,
            HashTable3::FuncCrefStr,
            HashTable3::FuncExpStr,
        ),
    );
    let mut srcs: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
    if BaseHashTable::hasKey(dst.clone(), &invHt)? {
        srcs = BaseHashTable::get(dst.clone(), &invHt)?;
        srcs = amortizeUnion(metamodelica::cons(src, srcs));
        outInvHt = BaseHashTable::add((dst, srcs), invHt)?;
    } else {
        outInvHt = BaseHashTable::add((dst, list![src]), invHt)?;
    }
    Ok(outInvHt)
}

fn amortizeUnion(
    mut inCrefs: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
) -> metamodelica::List<metamodelica::Ref<DAE::ComponentRef>> {
    let mut crefs: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
    crefs = (::match_deref::match_deref! { match &(&*inCrefs) {
        _ if (intMod(((inCrefs).len() as i32), 7) == 0) => List::union(&(metamodelica::nil()), &inCrefs),
        _ => inCrefs.clone(),
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    crefs
}

pub(crate) fn addReplacementIfNot(
    mut condition: bool,
    mut repl: VariableReplacements,
    mut inSrc: metamodelica::Ref<DAE::ComponentRef>,
    mut inDst: metamodelica::Ref<DAE::Exp>,
) -> Result<VariableReplacements> {
    let mut outRepl: VariableReplacements;
    outRepl = (match condition {
        false => {
            let mut src = inSrc;
            let mut dst = inDst;
            let mut repl_1: VariableReplacements;
            repl_1 = addReplacement(repl, src, dst)?;
            repl_1
        }
        true => repl,
    });
    Ok(outRepl)
}

fn makeTransitive(
    mut repl: VariableReplacements,
    mut src: metamodelica::Ref<DAE::ComponentRef>,
    mut dst: metamodelica::Ref<DAE::Exp>,
) -> Result<(
    VariableReplacements,
    metamodelica::Ref<DAE::ComponentRef>,
    metamodelica::Ref<DAE::Exp>,
)> {
    let mut outRepl: VariableReplacements;
    let mut outSrc: metamodelica::Ref<DAE::ComponentRef>;
    let mut outDst: metamodelica::Ref<DAE::Exp>;
    (outRepl, outSrc, outDst) = (match &*dst {
        _ => {
            let mut repl_1: VariableReplacements;
            let mut repl_2: VariableReplacements;
            let mut src_1: metamodelica::Ref<DAE::ComponentRef>;
            let mut src_2: metamodelica::Ref<DAE::ComponentRef>;
            let mut dst_1: metamodelica::Ref<DAE::Exp>;
            let mut dst_2: metamodelica::Ref<DAE::Exp>;
            let mut dst_3: metamodelica::Ref<DAE::Exp>;
            (repl_1, src_1, dst_1) = makeTransitive1(repl, src, dst);
            (repl_2, src_2, dst_2) = makeTransitive2(repl_1, src_1, dst_1);
            (dst_3, _) = ExpressionSimplify::simplify1(dst_2)?;
            (repl_2, src_2, dst_3)
        }
    });
    Ok((outRepl, outSrc, outDst))
}

fn makeTransitive1(
    mut repl: VariableReplacements,
    mut src: metamodelica::Ref<DAE::ComponentRef>,
    mut dst: metamodelica::Ref<DAE::Exp>,
) -> (
    VariableReplacements,
    metamodelica::Ref<DAE::ComponentRef>,
    metamodelica::Ref<DAE::Exp>,
) {
    let mut outRepl: VariableReplacements;
    let mut outSrc: metamodelica::Ref<DAE::ComponentRef>;
    let mut outDst: metamodelica::Ref<DAE::Exp>;
    (outRepl, outSrc, outDst) = 'mc: {
        let __mc_input = repl.clone();
        if let Ok(__v) = (|| -> Result<_> {
            let VariableReplacements {
                hashTable: _,
                invHashTable: mut invHt,
            } = __mc_input.clone()
            else {
                return Err("nomatch");
            };
            let mut lst: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
            let mut repl_1: VariableReplacements;
            let mut singleRepl: VariableReplacements;
            lst = BaseHashTable::get(src.clone(), &(invHt.clone()))?;
            singleRepl = addReplacementNoTransitive(emptyReplacementsSized(53), src.clone(), dst.clone())?;
            repl_1 = makeTransitive12(lst.clone(), repl.clone(), &singleRepl)?;
            Ok((repl_1.clone(), src.clone(), dst.clone()))
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            Ok((repl.clone(), src.clone(), dst.clone()))
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    (outRepl, outSrc, outDst)
}

fn makeTransitive12<'__b>(
    mut lst: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
    mut repl: VariableReplacements,
    mut singleRepl: &'__b VariableReplacements,
) -> Result<VariableReplacements> {
    '__tco: loop {
        ::match_deref::match_deref! { match &((lst, repl.clone())) {
            (Deref @ metamodelica::ListNode::Nil, _) => {
                return Ok(repl)
            },
            (Deref @ metamodelica::ListNode::Cons { head: cr, tail: crs }, VariableReplacements { hashTable: ht, .. }) => {
                let mut crDst: metamodelica::Ref<DAE::Exp>;
                let mut repl1: VariableReplacements;
                let mut repl2: VariableReplacements;
                crDst = BaseHashTable::get(cr.clone(), &(ht.clone()))?;
                (crDst, _) = replaceExp(crDst, singleRepl, None)?;
                repl1 = addReplacementNoTransitive(repl, cr.clone(), crDst)?;
                { (lst, repl, singleRepl) = (crs.clone(), repl1, singleRepl); continue '__tco; }
            },
            _ => return Err("match: no arm matched"),
        } }
    }
}

fn makeTransitive2(
    mut repl: VariableReplacements,
    mut src: metamodelica::Ref<DAE::ComponentRef>,
    mut dst: metamodelica::Ref<DAE::Exp>,
) -> (
    VariableReplacements,
    metamodelica::Ref<DAE::ComponentRef>,
    metamodelica::Ref<DAE::Exp>,
) {
    let mut outRepl: VariableReplacements;
    let mut outSrc: metamodelica::Ref<DAE::ComponentRef>;
    let mut outDst: metamodelica::Ref<DAE::Exp>;
    (outRepl, outSrc, outDst) = 'mc: {
        let __mc_input = &*dst;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut dst_1: metamodelica::Ref<DAE::Exp>;
                    (dst_1, _) = replaceExp(dst.clone(), &repl, None)?;
                    Ok((repl.clone(), src.clone(), dst_1.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((repl.clone(), src.clone(), dst.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    (outRepl, outSrc, outDst)
}

pub(crate) fn getReplacement(
    mut inVariableReplacements: &VariableReplacements,
    mut inComponentRef: metamodelica::Ref<DAE::ComponentRef>,
) -> Result<metamodelica::Ref<DAE::Exp>> {
    let mut outComponentRef: metamodelica::Ref<DAE::Exp>;
    outComponentRef = (match inVariableReplacements.clone() {
        VariableReplacements { hashTable: mut ht, .. } => {
            let mut src = inComponentRef;
            let mut dst: metamodelica::Ref<DAE::Exp>;
            dst = BaseHashTable::get(src, &(ht.clone()))?;
            dst
        }
    });
    Ok(outComponentRef)
}

pub(crate) fn replaceExpOpt(
    mut inExp: Option<metamodelica::Ref<DAE::Exp>>,
    mut repl: &VariableReplacements,
    mut funcOpt: Option<Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<bool> + 'static>>,
) -> Result<Option<metamodelica::Ref<DAE::Exp>>> {
    pub type FuncTypeExp_ExpToBoolean =
        std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<bool> + 'static>;

    let mut outExp: Option<metamodelica::Ref<DAE::Exp>>;
    outExp = (::match_deref::match_deref! { match &(inExp) {
        Some(e) => {
            let mut e = (*e).clone();
            (e, _) = replaceExp(e.clone(), repl, funcOpt)?;
            Some(e.clone())
        },
        _ => {
            None
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outExp)
}

pub(crate) fn avoidDoubleHashLookup(
    mut inExp: metamodelica::Ref<DAE::Exp>,
    mut inType: metamodelica::Ref<DAE::Type>,
) -> metamodelica::Ref<DAE::Exp> {
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    outExp = 'mc: {
        let __mc_input = &*inExp;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::CREF { componentRef: cr, ty: Deref @ DAE::Type::T_UNKNOWN { .. } } => {
                    Ok(Expression::makeCrefExp(cr.clone(), inType.clone())?)
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

pub(crate) fn replaceExpRepeated(
    mut e: metamodelica::Ref<DAE::Exp>,
    mut repl: &VariableReplacements,
    mut func: Option<Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<bool> + 'static>>,
    mut maxIter: i32,
) -> Result<metamodelica::Ref<DAE::Exp>> {
    pub type VisitFunc = std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<bool> + 'static>;

    let mut outExp: metamodelica::Ref<DAE::Exp>;
    outExp = replaceExpRepeated2(e, repl, func, maxIter, 1, false)?;
    Ok(outExp)
}

pub(crate) fn replaceExpRepeated2<'__b>(
    mut e: metamodelica::Ref<DAE::Exp>,
    mut repl: &'__b VariableReplacements,
    mut func: Option<Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<bool> + 'static>>,
    mut maxIter: i32,
    mut i: i32,
    mut equal: bool,
) -> Result<metamodelica::Ref<DAE::Exp>> {
    pub type VisitFunc = std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<bool> + 'static>;

    '__tco: loop {
        let mut e1: metamodelica::Ref<DAE::Exp>;
        let mut b: bool;
        if i > maxIter || equal {
            return Ok(e);
        } else {
            (e1, b) = replaceExp(e, repl, func.clone())?;
            {
                (e, repl, func, maxIter, i, equal) = (e1, repl, func, maxIter, i + 1, !(b));
                continue '__tco;
            }
        }
    }
}

pub fn replaceExp(
    mut inExp: metamodelica::Ref<DAE::Exp>,
    mut inVarReplacements: &VariableReplacements,
    mut inCondition: Option<Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<bool> + 'static>>,
) -> Result<(metamodelica::Ref<DAE::Exp>, bool)> {
    pub type FuncTypeExp_ExpToBoolean =
        std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<bool> + 'static>;

    let mut outExp: metamodelica::Ref<DAE::Exp>;
    let mut replacementPerformed: bool;
    outExp = inExp.clone();
    if replaceExpCond(inCondition.clone(), inExp.clone())? {
        (outExp, _) = Expression::traverseExpBottomUp(
            inExp.clone(),
            &({
                let __pe_b1 = inVarReplacements.clone();
                let __pe_b2 = inCondition;
                move |__pe_a0, __pe_a3| replaceExpCref(__pe_a0, &__pe_b1, __pe_b2.clone(), __pe_a3)
            }),
            true,
        )?;
    }
    replacementPerformed = !(referenceEq(&*(&*outExp), &*(inExp)));
    Ok((outExp, replacementPerformed))
}

fn replaceExpCref(
    mut inExp: metamodelica::Ref<DAE::Exp>,
    mut inVarReplacements: &VariableReplacements,
    mut inCondition: Option<Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<bool> + 'static>>,
    mut inReplacementPerformed: bool,
) -> Result<(metamodelica::Ref<DAE::Exp>, bool)> {
    pub type FuncTypeExp_ExpToBoolean =
        std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<bool> + 'static>;

    let mut outExp: metamodelica::Ref<DAE::Exp>;
    let mut replacementPerformed: bool;
    if !(replaceExpCond(inCondition, inExp.clone())?) {
        Error::addInternalError(
            literal!("Got exp to replace when condition is not allowing replacements. Check traversal."),
            metamodelica::sourceInfo!("Util/VarTransform.mo"),
        )?;
    }
    replacementPerformed = false;
    outExp = inExp.clone();
    let () = (match &*inExp {
        DAE::Exp::CREF {
            componentRef: cr,
            ty: __inExp_ty,
        } => {
            if '__try0: {
                outExp = unwrap_break_err!(getReplacement(inVarReplacements, cr.clone()), '__try0);
                outExp = avoidDoubleHashLookup(outExp.clone(), __inExp_ty.clone());
                replacementPerformed = true;
                Ok::<(), &'static str>(())
            }
            .is_err()
            {}
            ()
        }
        _ => (),
    });
    Ok((outExp, replacementPerformed))
}

pub(crate) fn replaceExpList(
    mut iexpl: &metamodelica::List<metamodelica::Ref<DAE::Exp>>,
    mut repl: &VariableReplacements,
    mut cond: Option<Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<bool> + 'static>>,
) -> Result<(metamodelica::List<metamodelica::Ref<DAE::Exp>>, bool)> {
    pub type FuncTypeExp_ExpToBoolean =
        std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<bool> + 'static>;

    let mut outExpl: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
    let mut replacementPerformed: bool;
    let mut acc1: metamodelica::List<metamodelica::Ref<DAE::Exp>> = metamodelica::nil();
    let mut acc2: bool = false;
    let mut c: bool;
    for mut exp in &**iexpl {
        let mut exp = exp.clone();
        (exp, c) = replaceExp(exp, repl, cond.clone())?;
        acc2 = acc2 || c;
        acc1 = metamodelica::cons(exp, acc1);
    }
    outExpl = metamodelica::Dangerous::listReverseInPlace(acc1);
    replacementPerformed = acc2;
    Ok((outExpl, replacementPerformed))
}

fn replaceExpCond(
    mut inFuncTypeExpExpToBooleanOption: Option<
        Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<bool> + 'static>,
    >,
    mut inExp: metamodelica::Ref<DAE::Exp>,
) -> Result<bool> {
    pub type FuncTypeExp_ExpToBoolean =
        std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<bool> + 'static>;

    let mut outBoolean: bool;
    outBoolean = (match inFuncTypeExpExpToBooleanOption {
        Some(mut cond) => {
            let mut e = inExp;
            let mut res: bool;
            res = cond(e)?;
            res
        }
        _ => true,
    });
    Ok(outBoolean)
}

fn replaceExpMatrix(
    mut inTplExpExpBooleanLstLst: &metamodelica::List<metamodelica::List<metamodelica::Ref<DAE::Exp>>>,
    mut inVariableReplacements: &VariableReplacements,
    mut inFuncTypeExpExpToBooleanOption: Option<
        Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<bool> + 'static>,
    >,
) -> Result<(
    metamodelica::List<metamodelica::List<metamodelica::Ref<DAE::Exp>>>,
    bool,
)> {
    pub type FuncTypeExp_ExpToBoolean =
        std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>) -> Result<bool> + 'static>;

    let mut outTplExpExpBooleanLstLst: metamodelica::List<metamodelica::List<metamodelica::Ref<DAE::Exp>>>;
    let mut replacementPerformed: bool;
    let mut acc1: metamodelica::List<metamodelica::List<metamodelica::Ref<DAE::Exp>>> = metamodelica::nil();
    let mut acc2: bool = false;
    let mut c: bool;
    for mut exp in &**inTplExpExpBooleanLstLst {
        let mut exp = exp.clone();
        (exp, c) = replaceExpList(&exp, inVariableReplacements, inFuncTypeExpExpToBooleanOption.clone())?;
        acc2 = acc2 || c;
        acc1 = metamodelica::cons(exp, acc1);
    }
    outTplExpExpBooleanLstLst = metamodelica::Dangerous::listReverseInPlace(acc1);
    replacementPerformed = acc2;
    Ok((outTplExpExpBooleanLstLst, replacementPerformed))
}
