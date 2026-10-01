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
use crate::DAEDump;
use crate::DAEUtil;
use crate::Expression;
use crate::ExpressionSimplify;
use crate::HashTable2;
use crate::Types;
use crate::VarTransform;
use openmodelica_ast::Absyn;
use openmodelica_ast_collections::AvlSetPath;
use openmodelica_frontend_dump::AbsynUtil;
use openmodelica_frontend_dump::AvlTreePathFunction;
use openmodelica_frontend_dump::ClassInfUtil;
use openmodelica_frontend_dump::ComponentReferenceBasics;
use openmodelica_frontend_dump::ElementSource;
use openmodelica_frontend_dump::ExpressionBasics;
use openmodelica_frontend_dump::HashTable3;
use openmodelica_frontend_dump::HashTableCG;
use openmodelica_frontend_dump::SCodeUtil;
use openmodelica_frontend_dump::TypesDump;
use openmodelica_frontend_types::ClassInf;
use openmodelica_frontend_types::DAE;
use openmodelica_frontend_types::SCode;
use openmodelica_util::BaseHashTable;
use openmodelica_util::Config;
use openmodelica_util::Debug;
use openmodelica_util::Error;
use openmodelica_util::Flags;
use openmodelica_util::Global;
use openmodelica_util::Util;
use openmodelica_util_datatypes_basic::List;

pub type Functiontuple = (
    Option<metamodelica::Ref<AvlTreePathFunction::Tree>>,
    metamodelica::List<DAE::InlineType>,
);

pub fn inlineStartAttribute(
    mut inVariableAttributesOption: Option<metamodelica::Ref<DAE::VariableAttributes>>,
    mut isource: metamodelica::Ref<DAE::ElementSource>,
    mut fns: Functiontuple,
) -> (
    Option<metamodelica::Ref<DAE::VariableAttributes>>,
    metamodelica::Ref<DAE::ElementSource>,
    bool,
) {
    let mut outVariableAttributesOption: Option<metamodelica::Ref<DAE::VariableAttributes>>;
    let mut osource: metamodelica::Ref<DAE::ElementSource>;
    let mut b: bool;
    (outVariableAttributesOption, osource, b) = 'mc: {
        let __mc_input = &inVariableAttributesOption;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                None => {
                    Ok((None, isource.clone(), false))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Some(Deref @ DAE::VariableAttributes::VAR_ATTR_REAL { quantity, unit, displayUnit, min, max, start: Some(r), fixed, nominal, stateSelectOption, uncertainOption, distributionOption, equationBound, isProtected, finalPrefix, startOrigin: so }) => {
                    let mut source: metamodelica::Ref<DAE::ElementSource>;
                    let mut r = (*r).clone();
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(inlineExp(r.clone(), fns.clone(), isource.clone())) {
                        (__pa0, __pa1, true, _) => (__pa0.clone(), __pa1.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    r = metamodelica::Own::own(__pa0);
                    source = metamodelica::Own::own(__pa1);
                    Ok((Some(metamodelica::Ref::new(DAE::VariableAttributes::VAR_ATTR_REAL { quantity: quantity.clone(), unit: unit.clone(), displayUnit: displayUnit.clone(), min: min.clone(), max: max.clone(), start: Some(r.clone()), fixed: fixed.clone(), nominal: nominal.clone(), stateSelectOption: stateSelectOption.clone(), uncertainOption: uncertainOption.clone(), distributionOption: distributionOption.clone(), equationBound: equationBound.clone(), isProtected: isProtected.clone(), finalPrefix: finalPrefix.clone(), startOrigin: so.clone() })), source.clone(), true))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Some(Deref @ DAE::VariableAttributes::VAR_ATTR_INT { quantity, min, max, start: Some(r), fixed, uncertainOption, distributionOption, equationBound, isProtected, finalPrefix, startOrigin: so }) => {
                    let mut source: metamodelica::Ref<DAE::ElementSource>;
                    let mut r = (*r).clone();
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(inlineExp(r.clone(), fns.clone(), isource.clone())) {
                        (__pa0, __pa1, true, _) => (__pa0.clone(), __pa1.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    r = metamodelica::Own::own(__pa0);
                    source = metamodelica::Own::own(__pa1);
                    Ok((Some(metamodelica::Ref::new(DAE::VariableAttributes::VAR_ATTR_INT { quantity: quantity.clone(), min: min.clone(), max: max.clone(), start: Some(r.clone()), fixed: fixed.clone(), uncertainOption: uncertainOption.clone(), distributionOption: distributionOption.clone(), equationBound: equationBound.clone(), isProtected: isProtected.clone(), finalPrefix: finalPrefix.clone(), startOrigin: so.clone() })), source.clone(), true))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Some(Deref @ DAE::VariableAttributes::VAR_ATTR_BOOL { quantity, start: Some(r), fixed, equationBound, isProtected, finalPrefix, startOrigin: so }) => {
                    let mut source: metamodelica::Ref<DAE::ElementSource>;
                    let mut r = (*r).clone();
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(inlineExp(r.clone(), fns.clone(), isource.clone())) {
                        (__pa0, __pa1, true, _) => (__pa0.clone(), __pa1.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    r = metamodelica::Own::own(__pa0);
                    source = metamodelica::Own::own(__pa1);
                    Ok((Some(metamodelica::Ref::new(DAE::VariableAttributes::VAR_ATTR_BOOL { quantity: quantity.clone(), start: Some(r.clone()), fixed: fixed.clone(), equationBound: equationBound.clone(), isProtected: isProtected.clone(), finalPrefix: finalPrefix.clone(), startOrigin: so.clone() })), source.clone(), true))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Some(Deref @ DAE::VariableAttributes::VAR_ATTR_STRING { quantity, start: Some(r), fixed, equationBound, isProtected, finalPrefix, startOrigin: so }) => {
                    let mut source: metamodelica::Ref<DAE::ElementSource>;
                    let mut r = (*r).clone();
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(inlineExp(r.clone(), fns.clone(), isource.clone())) {
                        (__pa0, __pa1, true, _) => (__pa0.clone(), __pa1.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    r = metamodelica::Own::own(__pa0);
                    source = metamodelica::Own::own(__pa1);
                    Ok((Some(metamodelica::Ref::new(DAE::VariableAttributes::VAR_ATTR_STRING { quantity: quantity.clone(), start: Some(r.clone()), fixed: fixed.clone(), equationBound: equationBound.clone(), isProtected: isProtected.clone(), finalPrefix: finalPrefix.clone(), startOrigin: so.clone() })), source.clone(), true))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Some(Deref @ DAE::VariableAttributes::VAR_ATTR_ENUMERATION { quantity, min, max, start: Some(r), fixed, equationBound, isProtected, finalPrefix, startOrigin: so }) => {
                    let mut source: metamodelica::Ref<DAE::ElementSource>;
                    let mut r = (*r).clone();
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(inlineExp(r.clone(), fns.clone(), isource.clone())) {
                        (__pa0, __pa1, true, _) => (__pa0.clone(), __pa1.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    r = metamodelica::Own::own(__pa0);
                    source = metamodelica::Own::own(__pa1);
                    Ok((Some(metamodelica::Ref::new(DAE::VariableAttributes::VAR_ATTR_ENUMERATION { quantity: quantity.clone(), min: min.clone(), max: max.clone(), start: Some(r.clone()), fixed: fixed.clone(), equationBound: equationBound.clone(), isProtected: isProtected.clone(), finalPrefix: finalPrefix.clone(), startOrigin: so.clone() })), source.clone(), true))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((inVariableAttributesOption.clone(), isource.clone(), false))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    (outVariableAttributesOption, osource, b)
}

pub fn inlineCallsInFunctions(
    mut inElementList: metamodelica::List<DAE::Function>,
    mut inFunctions: &Functiontuple,
) -> metamodelica::List<DAE::Function> {
    let mut outElementList: metamodelica::List<DAE::Function>;
    let mut body: metamodelica::List<metamodelica::Ref<DAE::Element>> = metamodelica::nil();
    let mut fn_def: DAE::FunctionDefinition;
    let mut fn_defs: metamodelica::List<DAE::FunctionDefinition>;
    outElementList = ({
        let mut __acc: metamodelica::List<DAE::Function> = metamodelica::nil();
        for mut r#fn in (inElementList).into_iter().cloned() {
            let __x = 'mc: {
                let __mc_input = r#fn.clone();
                if let Ok((__v, __wb0, __wb1)) = (|| -> Result<_> {
                    ::match_deref::match_deref! { match &__mc_input {
                        DAE::Function::FUNCTION { functions: Deref @ metamodelica::ListNode::Cons { head: fn_def @ DAE::FunctionDefinition::FUNCTION_DEF { .. }, tail: fn_defs }, .. } => {
                            let mut fn_def = (*fn_def).clone();
                            let mut body: metamodelica::List<metamodelica::Ref<DAE::Element>> = body.clone();
                            let mut r#fn: DAE::Function = r#fn.clone();
                            let __pa0 = ::match_deref::match_deref! { match &(inlineDAEElements(var_field!(fn_def.body, DAE::FunctionDefinition::FUNCTION_DEF), inFunctions, metamodelica::nil(), false)) {
                                (__pa0, true) => __pa0.clone(),
                                _ => return Err("pattern mismatch"),
                            } };
                            body = metamodelica::Own::own(__pa0);
                            let __owned_variant_body_0 = body.clone();
                            if let DAE::FunctionDefinition::FUNCTION_DEF { body, .. } = &mut fn_def {
                                *body = __owned_variant_body_0;
                            } else { panic!("owned-variant field-assign: value held a different variant than DAE::FunctionDefinition::FUNCTION_DEF"); }
                            let __owned_variant_functions_0 = metamodelica::cons(fn_def.clone(), fn_defs.clone());
                            if let DAE::Function::FUNCTION { functions, .. } = &mut r#fn {
                                *functions = __owned_variant_functions_0;
                            } else { panic!("owned-variant field-assign: value held a different variant than DAE::Function::FUNCTION"); }
                            Ok((r#fn.clone(), body.clone(), r#fn.clone()))
                        }
                        _ => return Err("nomatch"),
                    }}
                })() {
                    body = __wb0;
                    r#fn = __wb1;
                    break 'mc __v;
                }
                if let Ok((__v, __wb0, __wb1)) = (|| -> Result<_> {
                    ::match_deref::match_deref! { match &__mc_input {
                        DAE::Function::FUNCTION { functions: Deref @ metamodelica::ListNode::Cons { head: fn_def @ DAE::FunctionDefinition::FUNCTION_EXT { .. }, tail: fn_defs }, .. } => {
                            let mut fn_def = (*fn_def).clone();
                            let mut body: metamodelica::List<metamodelica::Ref<DAE::Element>> = body.clone();
                            let mut r#fn: DAE::Function = r#fn.clone();
                            let __pa0 = ::match_deref::match_deref! { match &(inlineDAEElements(var_field!(fn_def.body, DAE::FunctionDefinition::FUNCTION_EXT), inFunctions, metamodelica::nil(), false)) {
                                (__pa0, true) => __pa0.clone(),
                                _ => return Err("pattern mismatch"),
                            } };
                            body = metamodelica::Own::own(__pa0);
                            let __owned_variant_body_0 = body.clone();
                            if let DAE::FunctionDefinition::FUNCTION_EXT { body, .. } = &mut fn_def {
                                *body = __owned_variant_body_0;
                            } else { panic!("owned-variant field-assign: value held a different variant than DAE::FunctionDefinition::FUNCTION_EXT"); }
                            let __owned_variant_functions_0 = metamodelica::cons(fn_def.clone(), fn_defs.clone());
                            if let DAE::Function::FUNCTION { functions, .. } = &mut r#fn {
                                *functions = __owned_variant_functions_0;
                            } else { panic!("owned-variant field-assign: value held a different variant than DAE::Function::FUNCTION"); }
                            Ok((r#fn.clone(), body.clone(), r#fn.clone()))
                        }
                        _ => return Err("nomatch"),
                    }}
                })() {
                    body = __wb0;
                    r#fn = __wb1;
                    break 'mc __v;
                }
                if let Ok(__v) = (|| -> Result<_> {
                    ::match_deref::match_deref! { match &__mc_input {
                        _ => {
                            Ok(r#fn.clone())
                        }
                        _ => return Err("nomatch"),
                    }}
                })() {
                    break 'mc __v;
                }
                panic!("matchcontinue: no arm matched")
            };
            __acc = cons(__x, __acc);
        }
        __acc.reverse()
    });
    outElementList
}

fn inlineDAEElementsLst<'__b>(
    mut inElementList: &'__b metamodelica::List<metamodelica::List<metamodelica::Ref<DAE::Element>>>,
    mut inFunctions: &'__b Functiontuple,
    mut iAcc: metamodelica::List<metamodelica::List<metamodelica::Ref<DAE::Element>>>,
    mut iInlined: bool,
) -> (
    metamodelica::List<metamodelica::List<metamodelica::Ref<DAE::Element>>>,
    bool,
) {
    '__tco: loop {
        ::match_deref::match_deref! { match inElementList {
            Deref @ metamodelica::ListNode::Nil => {
                return (iAcc.reverse(), iInlined)
            },
            Deref @ metamodelica::ListNode::Cons { head: elem, tail: rest } => {
                let mut acc: metamodelica::List<metamodelica::List<metamodelica::Ref<DAE::Element>>>;
                let mut inlined: bool;
                let mut elem = (*elem).clone();
                (elem, inlined) = inlineDAEElements(metamodelica::AsArg::as_arg(&elem), inFunctions, metamodelica::nil(), false);
                { (inElementList, inFunctions, iAcc, iInlined) = (rest, inFunctions, metamodelica::cons(elem.clone(), iAcc), inlined || iInlined); continue '__tco; }
            },
            _ => unreachable!("tail-call lowered match: no arm matched"),
        } }
    }
}

fn inlineDAEElements<'__b>(
    mut inElementList: &'__b metamodelica::List<metamodelica::Ref<DAE::Element>>,
    mut inFunctions: &'__b Functiontuple,
    mut iAcc: metamodelica::List<metamodelica::Ref<DAE::Element>>,
    mut iInlined: bool,
) -> (metamodelica::List<metamodelica::Ref<DAE::Element>>, bool) {
    '__tco: loop {
        ::match_deref::match_deref! { match inElementList {
            Deref @ metamodelica::ListNode::Nil => {
                return (iAcc.reverse(), iInlined)
            },
            Deref @ metamodelica::ListNode::Cons { head: elem, tail: rest } => {
                let mut acc: metamodelica::List<metamodelica::Ref<DAE::Element>>;
                let mut inlined: bool;
                let mut elem = (*elem).clone();
                (elem, inlined) = inlineDAEElement(elem.clone(), inFunctions.clone());
                { (inElementList, inFunctions, iAcc, iInlined) = (rest, inFunctions, metamodelica::cons(elem.clone(), iAcc), inlined || iInlined); continue '__tco; }
            },
            _ => unreachable!("tail-call lowered match: no arm matched"),
        } }
    }
}

fn inlineDAEElement(
    mut inElement: metamodelica::Ref<DAE::Element>,
    mut inFunctions: Functiontuple,
) -> (metamodelica::Ref<DAE::Element>, bool) {
    let mut outElement: metamodelica::Ref<DAE::Element>;
    let mut inlined: bool;
    (outElement, inlined) = 'mc: {
        let __mc_input = (inElement, inFunctions);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Element::VAR { componentRef, kind, direction, parallelism, protection, ty, binding: Some(binding), dims, connectorType: ct, source, variableAttributesOption, comment: absynCommentOption, innerOuter, encrypted: e }, fns) => {
                    let mut binding_1: metamodelica::Ref<DAE::Exp>;
                    let mut source = (*source).clone();
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(inlineExp(binding.clone(), fns.clone(), source.clone())) {
                        (__pa0, __pa1, true, _) => (__pa0.clone(), __pa1.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    binding_1 = metamodelica::Own::own(__pa0);
                    source = metamodelica::Own::own(__pa1);
                    Ok((metamodelica::Ref::new(DAE::Element::VAR { componentRef: componentRef.clone(), kind: kind.clone(), direction: direction.clone(), parallelism: parallelism.clone(), protection: protection.clone(), ty: ty.clone(), binding: Some(binding_1.clone()), dims: dims.clone(), connectorType: ct.clone(), source: source.clone(), variableAttributesOption: variableAttributesOption.clone(), comment: absynCommentOption.clone(), innerOuter: innerOuter.clone(), encrypted: e.clone() }), true))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Element::DEFINE { componentRef, exp, source }, fns) => {
                    let mut exp_1: metamodelica::Ref<DAE::Exp>;
                    let mut source = (*source).clone();
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(inlineExp(exp.clone(), fns.clone(), source.clone())) {
                        (__pa0, __pa1, true, _) => (__pa0.clone(), __pa1.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    exp_1 = metamodelica::Own::own(__pa0);
                    source = metamodelica::Own::own(__pa1);
                    Ok((metamodelica::Ref::new(DAE::Element::DEFINE { componentRef: componentRef.clone(), exp: exp_1.clone(), source: source.clone() }), true))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Element::INITIALDEFINE { componentRef, exp, source }, fns) => {
                    let mut exp_1: metamodelica::Ref<DAE::Exp>;
                    let mut source = (*source).clone();
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(inlineExp(exp.clone(), fns.clone(), source.clone())) {
                        (__pa0, __pa1, true, _) => (__pa0.clone(), __pa1.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    exp_1 = metamodelica::Own::own(__pa0);
                    source = metamodelica::Own::own(__pa1);
                    Ok((metamodelica::Ref::new(DAE::Element::INITIALDEFINE { componentRef: componentRef.clone(), exp: exp_1.clone(), source: source.clone() }), true))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Element::EQUATION { exp: exp1, scalar: exp2, source }, fns) => {
                    let mut exp1_1: metamodelica::Ref<DAE::Exp>;
                    let mut exp2_1: metamodelica::Ref<DAE::Exp>;
                    let mut b1: bool;
                    let mut b2: bool;
                    let mut source = (*source).clone();
                    (exp1_1, source, b1, _) = inlineExp(exp1.clone(), fns.clone(), source.clone());
                    (exp2_1, source, b2, _) = inlineExp(exp2.clone(), fns.clone(), source.clone());
                    let true = (b1 || b2) else { return Err("pattern mismatch") };
                    Ok((metamodelica::Ref::new(DAE::Element::EQUATION { exp: exp1_1.clone(), scalar: exp2_1.clone(), source: source.clone() }), true))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Element::ARRAY_EQUATION { dimension, exp: exp1, array: exp2, source }, fns) => {
                    let mut exp1_1: metamodelica::Ref<DAE::Exp>;
                    let mut exp2_1: metamodelica::Ref<DAE::Exp>;
                    let mut b1: bool;
                    let mut b2: bool;
                    let mut source = (*source).clone();
                    (exp1_1, source, b1, _) = inlineExp(exp1.clone(), fns.clone(), source.clone());
                    (exp2_1, source, b2, _) = inlineExp(exp2.clone(), fns.clone(), source.clone());
                    let true = (b1 || b2) else { return Err("pattern mismatch") };
                    Ok((metamodelica::Ref::new(DAE::Element::ARRAY_EQUATION { dimension: dimension.clone(), exp: exp1_1.clone(), array: exp2_1.clone(), source: source.clone() }), true))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Element::INITIAL_ARRAY_EQUATION { dimension, exp: exp1, array: exp2, source }, fns) => {
                    let mut exp1_1: metamodelica::Ref<DAE::Exp>;
                    let mut exp2_1: metamodelica::Ref<DAE::Exp>;
                    let mut b1: bool;
                    let mut b2: bool;
                    let mut source = (*source).clone();
                    (exp1_1, source, b1, _) = inlineExp(exp1.clone(), fns.clone(), source.clone());
                    (exp2_1, source, b2, _) = inlineExp(exp2.clone(), fns.clone(), source.clone());
                    let true = (b1 || b2) else { return Err("pattern mismatch") };
                    Ok((metamodelica::Ref::new(DAE::Element::INITIAL_ARRAY_EQUATION { dimension: dimension.clone(), exp: exp1_1.clone(), array: exp2_1.clone(), source: source.clone() }), true))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Element::COMPLEX_EQUATION { lhs: exp1, rhs: exp2, source }, fns) => {
                    let mut exp1_1: metamodelica::Ref<DAE::Exp>;
                    let mut exp2_1: metamodelica::Ref<DAE::Exp>;
                    let mut b1: bool;
                    let mut b2: bool;
                    let mut source = (*source).clone();
                    (exp1_1, source, b1, _) = inlineExp(exp1.clone(), fns.clone(), source.clone());
                    (exp2_1, source, b2, _) = inlineExp(exp2.clone(), fns.clone(), source.clone());
                    let true = (b1 || b2) else { return Err("pattern mismatch") };
                    Ok((metamodelica::Ref::new(DAE::Element::COMPLEX_EQUATION { lhs: exp1_1.clone(), rhs: exp2_1.clone(), source: source.clone() }), true))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Element::INITIAL_COMPLEX_EQUATION { lhs: exp1, rhs: exp2, source }, fns) => {
                    let mut exp1_1: metamodelica::Ref<DAE::Exp>;
                    let mut exp2_1: metamodelica::Ref<DAE::Exp>;
                    let mut b1: bool;
                    let mut b2: bool;
                    let mut source = (*source).clone();
                    (exp1_1, source, b1, _) = inlineExp(exp1.clone(), fns.clone(), source.clone());
                    (exp2_1, source, b2, _) = inlineExp(exp2.clone(), fns.clone(), source.clone());
                    let true = (b1 || b2) else { return Err("pattern mismatch") };
                    Ok((metamodelica::Ref::new(DAE::Element::INITIAL_COMPLEX_EQUATION { lhs: exp1_1.clone(), rhs: exp2_1.clone(), source: source.clone() }), true))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Element::WHEN_EQUATION { condition: exp, equations: elist, elsewhen_: Some(el), source }, fns) => {
                    let mut elist_1: metamodelica::List<metamodelica::Ref<DAE::Element>>;
                    let mut el_1: metamodelica::Ref<DAE::Element>;
                    let mut exp_1: metamodelica::Ref<DAE::Exp>;
                    let mut b1: bool;
                    let mut b2: bool;
                    let mut b3: bool;
                    let mut source = (*source).clone();
                    (exp_1, source, b1, _) = inlineExp(exp.clone(), fns.clone(), source.clone());
                    (elist_1, b2) = inlineDAEElements(metamodelica::AsArg::as_arg(&elist), &(fns.clone()), metamodelica::nil(), false);
                    (el_1, b3) = inlineDAEElement(el.clone(), fns.clone());
                    let true = (b1 || b2 || b3) else { return Err("pattern mismatch") };
                    Ok((metamodelica::Ref::new(DAE::Element::WHEN_EQUATION { condition: exp_1.clone(), equations: elist_1.clone(), elsewhen_: Some(el_1.clone()), source: source.clone() }), true))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Element::WHEN_EQUATION { condition: exp, equations: elist, elsewhen_: None, source }, fns) => {
                    let mut elist_1: metamodelica::List<metamodelica::Ref<DAE::Element>>;
                    let mut exp_1: metamodelica::Ref<DAE::Exp>;
                    let mut b1: bool;
                    let mut b2: bool;
                    let mut source = (*source).clone();
                    (exp_1, source, b1, _) = inlineExp(exp.clone(), fns.clone(), source.clone());
                    (elist_1, b2) = inlineDAEElements(metamodelica::AsArg::as_arg(&elist), &(fns.clone()), metamodelica::nil(), false);
                    let true = (b1 || b2) else { return Err("pattern mismatch") };
                    Ok((metamodelica::Ref::new(DAE::Element::WHEN_EQUATION { condition: exp_1.clone(), equations: elist_1.clone(), elsewhen_: None, source: source.clone() }), true))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Element::IF_EQUATION { condition1: explst, equations2: dlist, equations3: elist, source }, fns) => {
                    let mut elist_1: metamodelica::List<metamodelica::Ref<DAE::Element>>;
                    let mut dlist_1: metamodelica::List<metamodelica::List<metamodelica::Ref<DAE::Element>>>;
                    let mut explst_1: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                    let mut b1: bool;
                    let mut b2: bool;
                    let mut b3: bool;
                    let mut source = (*source).clone();
                    (explst_1, source, b1) = inlineExps(explst.clone(), &(fns.clone()), source.clone());
                    (dlist_1, b2) = inlineDAEElementsLst(metamodelica::AsArg::as_arg(&dlist), &(fns.clone()), metamodelica::nil(), false);
                    (elist_1, b3) = inlineDAEElements(metamodelica::AsArg::as_arg(&elist), &(fns.clone()), metamodelica::nil(), false);
                    let true = (b1 || b2 || b3) else { return Err("pattern mismatch") };
                    Ok((metamodelica::Ref::new(DAE::Element::IF_EQUATION { condition1: explst_1.clone(), equations2: dlist_1.clone(), equations3: elist_1.clone(), source: source.clone() }), true))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Element::INITIAL_IF_EQUATION { condition1: explst, equations2: dlist, equations3: elist, source }, fns) => {
                    let mut elist_1: metamodelica::List<metamodelica::Ref<DAE::Element>>;
                    let mut dlist_1: metamodelica::List<metamodelica::List<metamodelica::Ref<DAE::Element>>>;
                    let mut explst_1: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                    let mut b1: bool;
                    let mut b2: bool;
                    let mut b3: bool;
                    let mut source = (*source).clone();
                    (explst_1, source, b1) = inlineExps(explst.clone(), &(fns.clone()), source.clone());
                    (dlist_1, b2) = inlineDAEElementsLst(metamodelica::AsArg::as_arg(&dlist), &(fns.clone()), metamodelica::nil(), false);
                    (elist_1, b3) = inlineDAEElements(metamodelica::AsArg::as_arg(&elist), &(fns.clone()), metamodelica::nil(), false);
                    let true = (b1 || b2 || b3) else { return Err("pattern mismatch") };
                    Ok((metamodelica::Ref::new(DAE::Element::INITIAL_IF_EQUATION { condition1: explst_1.clone(), equations2: dlist_1.clone(), equations3: elist_1.clone(), source: source.clone() }), true))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Element::INITIALEQUATION { exp1, exp2, source }, fns) => {
                    let mut exp1_1: metamodelica::Ref<DAE::Exp>;
                    let mut exp2_1: metamodelica::Ref<DAE::Exp>;
                    let mut source = (*source).clone();
                    (exp1_1, source, _, _) = inlineExp(exp1.clone(), fns.clone(), source.clone());
                    (exp2_1, source, _, _) = inlineExp(exp2.clone(), fns.clone(), source.clone());
                    Ok((metamodelica::Ref::new(DAE::Element::INITIALEQUATION { exp1: exp1_1.clone(), exp2: exp2_1.clone(), source: source.clone() }), true))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Element::ALGORITHM { algorithm_: alg, source }, fns) => {
                    let mut alg_1: metamodelica::Ref<DAE::Algorithm>;
                    let __pa0 = ::match_deref::match_deref! { match &(inlineAlgorithm(metamodelica::AsArg::as_arg(&alg), fns.clone())?) {
                        (__pa0, true) => __pa0.clone(),
                        _ => return Err("pattern mismatch"),
                    } };
                    alg_1 = metamodelica::Own::own(__pa0);
                    Ok((metamodelica::Ref::new(DAE::Element::ALGORITHM { algorithm_: alg_1.clone(), source: source.clone() }), true))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Element::INITIALALGORITHM { algorithm_: alg, source }, fns) => {
                    let mut alg_1: metamodelica::Ref<DAE::Algorithm>;
                    let __pa0 = ::match_deref::match_deref! { match &(inlineAlgorithm(metamodelica::AsArg::as_arg(&alg), fns.clone())?) {
                        (__pa0, true) => __pa0.clone(),
                        _ => return Err("pattern mismatch"),
                    } };
                    alg_1 = metamodelica::Own::own(__pa0);
                    Ok((metamodelica::Ref::new(DAE::Element::INITIALALGORITHM { algorithm_: alg_1.clone(), source: source.clone() }), true))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Element::COMP { ident: i, dAElist: elist, source, comment: absynCommentOption }, fns) => {
                    let mut elist_1: metamodelica::List<metamodelica::Ref<DAE::Element>>;
                    let __pa0 = ::match_deref::match_deref! { match &(inlineDAEElements(metamodelica::AsArg::as_arg(&elist), &(fns.clone()), metamodelica::nil(), false)) {
                        (__pa0, true) => __pa0.clone(),
                        _ => return Err("pattern mismatch"),
                    } };
                    elist_1 = metamodelica::Own::own(__pa0);
                    Ok((metamodelica::Ref::new(DAE::Element::COMP { ident: i.clone(), dAElist: elist_1.clone(), source: source.clone(), comment: absynCommentOption.clone() }), true))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Element::ASSERT { condition: exp1, message: exp2, level: exp3, source }, fns) => {
                    let mut exp1_1: metamodelica::Ref<DAE::Exp>;
                    let mut exp2_1: metamodelica::Ref<DAE::Exp>;
                    let mut exp3_1: metamodelica::Ref<DAE::Exp>;
                    let mut b1: bool;
                    let mut b2: bool;
                    let mut b3: bool;
                    let mut source = (*source).clone();
                    (exp1_1, source, b1, _) = inlineExp(exp1.clone(), fns.clone(), source.clone());
                    (exp2_1, source, b2, _) = inlineExp(exp2.clone(), fns.clone(), source.clone());
                    (exp3_1, source, b3, _) = inlineExp(exp3.clone(), fns.clone(), source.clone());
                    let true = (b1 || b2 || b3) else { return Err("pattern mismatch") };
                    Ok((metamodelica::Ref::new(DAE::Element::ASSERT { condition: exp1_1.clone(), message: exp2_1.clone(), level: exp3_1.clone(), source: source.clone() }), true))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Element::INITIAL_ASSERT { condition: exp1, message: exp2, level: exp3, source }, fns) => {
                    let mut exp1_1: metamodelica::Ref<DAE::Exp>;
                    let mut exp2_1: metamodelica::Ref<DAE::Exp>;
                    let mut exp3_1: metamodelica::Ref<DAE::Exp>;
                    let mut b1: bool;
                    let mut b2: bool;
                    let mut b3: bool;
                    let mut source = (*source).clone();
                    (exp1_1, source, b1, _) = inlineExp(exp1.clone(), fns.clone(), source.clone());
                    (exp2_1, source, b2, _) = inlineExp(exp2.clone(), fns.clone(), source.clone());
                    (exp3_1, source, b3, _) = inlineExp(exp3.clone(), fns.clone(), source.clone());
                    let true = (b1 || b2 || b3) else { return Err("pattern mismatch") };
                    Ok((metamodelica::Ref::new(DAE::Element::INITIAL_ASSERT { condition: exp1_1.clone(), message: exp2_1.clone(), level: exp3_1.clone(), source: source.clone() }), true))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Element::TERMINATE { message: exp, source }, fns) => {
                    let mut exp_1: metamodelica::Ref<DAE::Exp>;
                    let mut source = (*source).clone();
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(inlineExp(exp.clone(), fns.clone(), source.clone())) {
                        (__pa0, __pa1, true, _) => (__pa0.clone(), __pa1.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    exp_1 = metamodelica::Own::own(__pa0);
                    source = metamodelica::Own::own(__pa1);
                    Ok((metamodelica::Ref::new(DAE::Element::TERMINATE { message: exp_1.clone(), source: source.clone() }), true))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Element::INITIAL_TERMINATE { message: exp, source }, fns) => {
                    let mut exp_1: metamodelica::Ref<DAE::Exp>;
                    let mut source = (*source).clone();
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(inlineExp(exp.clone(), fns.clone(), source.clone())) {
                        (__pa0, __pa1, true, _) => (__pa0.clone(), __pa1.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    exp_1 = metamodelica::Own::own(__pa0);
                    source = metamodelica::Own::own(__pa1);
                    Ok((metamodelica::Ref::new(DAE::Element::INITIAL_TERMINATE { message: exp_1.clone(), source: source.clone() }), true))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Element::REINIT { componentRef, exp, source }, fns) => {
                    let mut exp_1: metamodelica::Ref<DAE::Exp>;
                    let mut source = (*source).clone();
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(inlineExp(exp.clone(), fns.clone(), source.clone())) {
                        (__pa0, __pa1, true, _) => (__pa0.clone(), __pa1.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    exp_1 = metamodelica::Own::own(__pa0);
                    source = metamodelica::Own::own(__pa1);
                    Ok((metamodelica::Ref::new(DAE::Element::REINIT { componentRef: componentRef.clone(), exp: exp_1.clone(), source: source.clone() }), true))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Element::NORETCALL { exp, source }, fns) => {
                    let mut exp = (*exp).clone();
                    let mut source = (*source).clone();
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(inlineExp(exp.clone(), fns.clone(), source.clone())) {
                        (__pa0, __pa1, true, _) => (__pa0.clone(), __pa1.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    exp = metamodelica::Own::own(__pa0);
                    source = metamodelica::Own::own(__pa1);
                    Ok((metamodelica::Ref::new(DAE::Element::NORETCALL { exp: exp.clone(), source: source.clone() }), true))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Element::INITIAL_NORETCALL { exp, source }, fns) => {
                    let mut exp = (*exp).clone();
                    let mut source = (*source).clone();
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(inlineExp(exp.clone(), fns.clone(), source.clone())) {
                        (__pa0, __pa1, true, _) => (__pa0.clone(), __pa1.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    exp = metamodelica::Own::own(__pa0);
                    source = metamodelica::Own::own(__pa1);
                    Ok((metamodelica::Ref::new(DAE::Element::INITIAL_NORETCALL { exp: exp.clone(), source: source.clone() }), true))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (el, _) => {
                    Ok((el.clone(), false))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    (outElement, inlined)
}

pub(crate) fn inlineAlgorithm(
    mut inAlgorithm: &metamodelica::Ref<DAE::Algorithm>,
    mut inElementList: Functiontuple,
) -> Result<(metamodelica::Ref<DAE::Algorithm>, bool)> {
    let mut outAlgorithm: metamodelica::Ref<DAE::Algorithm>;
    let mut inlined: bool;
    (outAlgorithm, inlined) = (match &**inAlgorithm {
        DAE::Algorithm { statementLst: stmts } => {
            let mut fns = inElementList;
            let mut stmts_1: metamodelica::List<metamodelica::Ref<DAE::Statement>>;
            (stmts_1, inlined) = inlineStatements(stmts, &fns, metamodelica::nil(), false);
            (
                metamodelica::Ref::new(DAE::Algorithm { statementLst: stmts_1 }),
                inlined,
            )
        }
        _ => {
            let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else {
                return Err("pattern mismatch");
            };
            Debug::trace(literal!("Inline.inlineAlgorithm failed\n"))?;
            return Err("fail");
        }
    });
    Ok((outAlgorithm, inlined))
}

pub fn inlineStatements<'__b>(
    mut inStatements: &'__b metamodelica::List<metamodelica::Ref<DAE::Statement>>,
    mut inElementList: &'__b Functiontuple,
    mut iAcc: metamodelica::List<metamodelica::Ref<DAE::Statement>>,
    mut iInlined: bool,
) -> (metamodelica::List<metamodelica::Ref<DAE::Statement>>, bool) {
    '__tco: loop {
        ::match_deref::match_deref! { match inStatements {
            Deref @ metamodelica::ListNode::Nil => {
                return (iAcc.reverse(), iInlined)
            },
            Deref @ metamodelica::ListNode::Cons { head: stmt, tail: rest } => {
                let mut acc: metamodelica::List<metamodelica::Ref<DAE::Statement>>;
                let mut inlined: bool;
                let mut stmt = (*stmt).clone();
                (stmt, inlined) = inlineStatement(stmt.clone(), inElementList.clone());
                { (inStatements, inElementList, iAcc, iInlined) = (rest, inElementList, metamodelica::cons(stmt.clone(), iAcc), inlined || iInlined); continue '__tco; }
            },
            _ => unreachable!("tail-call lowered match: no arm matched"),
        } }
    }
}

fn inlineStatement(
    mut inStatement: metamodelica::Ref<DAE::Statement>,
    mut inElementList: Functiontuple,
) -> (metamodelica::Ref<DAE::Statement>, bool) {
    let mut outStatement: metamodelica::Ref<DAE::Statement>;
    let mut inlined: bool;
    (outStatement, inlined) = 'mc: {
        let __mc_input = (inStatement, inElementList);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Statement::STMT_ASSIGN { type_: t, exp1: e1, exp: e2, source }, fns) => {
                    let mut e1_1: metamodelica::Ref<DAE::Exp>;
                    let mut e2_1: metamodelica::Ref<DAE::Exp>;
                    let mut b1: bool;
                    let mut b2: bool;
                    let mut source = (*source).clone();
                    (e1_1, source, b1, _) = inlineExp(e1.clone(), fns.clone(), source.clone());
                    (e2_1, source, b2, _) = inlineExp(e2.clone(), fns.clone(), source.clone());
                    let true = (b1 || b2) else { return Err("pattern mismatch") };
                    Ok((metamodelica::Ref::new(DAE::Statement::STMT_ASSIGN { type_: t.clone(), exp1: e1_1.clone(), exp: e2_1.clone(), source: source.clone() }), true))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Statement::STMT_TUPLE_ASSIGN { type_: t, expExpLst: explst, exp: e, source }, fns) => {
                    let mut e_1: metamodelica::Ref<DAE::Exp>;
                    let mut explst_1: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                    let mut b1: bool;
                    let mut b2: bool;
                    let mut source = (*source).clone();
                    (explst_1, source, b1) = inlineExps(explst.clone(), &(fns.clone()), source.clone());
                    (e_1, source, b2, _) = inlineExp(e.clone(), fns.clone(), source.clone());
                    let true = (b1 || b2) else { return Err("pattern mismatch") };
                    Ok((metamodelica::Ref::new(DAE::Statement::STMT_TUPLE_ASSIGN { type_: t.clone(), expExpLst: explst_1.clone(), exp: e_1.clone(), source: source.clone() }), true))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Statement::STMT_ASSIGN_ARR { type_: t, lhs: e1, exp: e2, source }, fns) => {
                    let mut e1_1: metamodelica::Ref<DAE::Exp>;
                    let mut e2_1: metamodelica::Ref<DAE::Exp>;
                    let mut b1: bool;
                    let mut b2: bool;
                    let mut source = (*source).clone();
                    (e1_1, source, b1, _) = inlineExp(e1.clone(), fns.clone(), source.clone());
                    (e2_1, source, b2, _) = inlineExp(e2.clone(), fns.clone(), source.clone());
                    let true = (b1 || b2) else { return Err("pattern mismatch") };
                    Ok((metamodelica::Ref::new(DAE::Statement::STMT_ASSIGN_ARR { type_: t.clone(), lhs: e1_1.clone(), exp: e2_1.clone(), source: source.clone() }), true))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Statement::STMT_IF { exp: e, statementLst: stmts, else_: a_else, source }, fns) => {
                    let mut e_1: metamodelica::Ref<DAE::Exp>;
                    let mut a_else_1: metamodelica::Ref<DAE::Else>;
                    let mut stmts_1: metamodelica::List<metamodelica::Ref<DAE::Statement>>;
                    let mut b1: bool;
                    let mut b2: bool;
                    let mut b3: bool;
                    let mut source = (*source).clone();
                    (e_1, source, b1, _) = inlineExp(e.clone(), fns.clone(), source.clone());
                    (stmts_1, b2) = inlineStatements(metamodelica::AsArg::as_arg(&stmts), &(fns.clone()), metamodelica::nil(), false);
                    (a_else_1, source, b3) = inlineElse(a_else.clone(), fns.clone(), source.clone());
                    let true = (b1 || b2 || b3) else { return Err("pattern mismatch") };
                    Ok((metamodelica::Ref::new(DAE::Statement::STMT_IF { exp: e_1.clone(), statementLst: stmts_1.clone(), else_: a_else_1.clone(), source: source.clone() }), true))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Statement::STMT_FOR { type_: t, iterIsArray: b, iter: i, range: e, statementLst: stmts, source, sub_iters }, fns) => {
                    let mut e_1: metamodelica::Ref<DAE::Exp>;
                    let mut stmts_1: metamodelica::List<metamodelica::Ref<DAE::Statement>>;
                    let mut b1: bool;
                    let mut b2: bool;
                    let mut source = (*source).clone();
                    (e_1, source, b1, _) = inlineExp(e.clone(), fns.clone(), source.clone());
                    (stmts_1, b2) = inlineStatements(metamodelica::AsArg::as_arg(&stmts), &(fns.clone()), metamodelica::nil(), false);
                    let true = (b1 || b2) else { return Err("pattern mismatch") };
                    Ok((metamodelica::Ref::new(DAE::Statement::STMT_FOR { type_: t.clone(), iterIsArray: b.clone(), iter: i.clone(), range: e_1.clone(), statementLst: stmts_1.clone(), source: source.clone(), sub_iters: sub_iters.clone() }), true))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Statement::STMT_WHILE { exp: e, statementLst: stmts, source }, fns) => {
                    let mut e_1: metamodelica::Ref<DAE::Exp>;
                    let mut stmts_1: metamodelica::List<metamodelica::Ref<DAE::Statement>>;
                    let mut b1: bool;
                    let mut b2: bool;
                    let mut source = (*source).clone();
                    (e_1, source, b1, _) = inlineExp(e.clone(), fns.clone(), source.clone());
                    (stmts_1, b2) = inlineStatements(metamodelica::AsArg::as_arg(&stmts), &(fns.clone()), metamodelica::nil(), false);
                    let true = (b1 || b2) else { return Err("pattern mismatch") };
                    Ok((metamodelica::Ref::new(DAE::Statement::STMT_WHILE { exp: e_1.clone(), statementLst: stmts_1.clone(), source: source.clone() }), true))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Statement::STMT_WHEN { exp: e, conditions, initialCall, statementLst: stmts, elseWhen: Some(stmt), source }, fns) => {
                    let mut stmt_1: metamodelica::Ref<DAE::Statement>;
                    let mut e_1: metamodelica::Ref<DAE::Exp>;
                    let mut stmts_1: metamodelica::List<metamodelica::Ref<DAE::Statement>>;
                    let mut b1: bool;
                    let mut b2: bool;
                    let mut b3: bool;
                    let mut source = (*source).clone();
                    (e_1, source, b1, _) = inlineExp(e.clone(), fns.clone(), source.clone());
                    (stmts_1, b2) = inlineStatements(metamodelica::AsArg::as_arg(&stmts), &(fns.clone()), metamodelica::nil(), false);
                    (stmt_1, b3) = inlineStatement(stmt.clone(), fns.clone());
                    let true = (b1 || b2 || b3) else { return Err("pattern mismatch") };
                    Ok((metamodelica::Ref::new(DAE::Statement::STMT_WHEN { exp: e_1.clone(), conditions: conditions.clone(), initialCall: initialCall.clone(), statementLst: stmts_1.clone(), elseWhen: Some(stmt_1.clone()), source: source.clone() }), true))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Statement::STMT_WHEN { exp: e, conditions, initialCall, statementLst: stmts, elseWhen: None, source }, fns) => {
                    let mut e_1: metamodelica::Ref<DAE::Exp>;
                    let mut stmts_1: metamodelica::List<metamodelica::Ref<DAE::Statement>>;
                    let mut b1: bool;
                    let mut b2: bool;
                    let mut source = (*source).clone();
                    (e_1, source, b1, _) = inlineExp(e.clone(), fns.clone(), source.clone());
                    (stmts_1, b2) = inlineStatements(metamodelica::AsArg::as_arg(&stmts), &(fns.clone()), metamodelica::nil(), false);
                    let true = (b1 || b2) else { return Err("pattern mismatch") };
                    Ok((metamodelica::Ref::new(DAE::Statement::STMT_WHEN { exp: e_1.clone(), conditions: conditions.clone(), initialCall: initialCall.clone(), statementLst: stmts_1.clone(), elseWhen: None, source: source.clone() }), true))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Statement::STMT_ASSERT { cond: e1, msg: e2, level: e3, source }, fns) => {
                    let mut e1_1: metamodelica::Ref<DAE::Exp>;
                    let mut e2_1: metamodelica::Ref<DAE::Exp>;
                    let mut e3_1: metamodelica::Ref<DAE::Exp>;
                    let mut b1: bool;
                    let mut b2: bool;
                    let mut b3: bool;
                    let mut source = (*source).clone();
                    (e1_1, source, b1, _) = inlineExp(e1.clone(), fns.clone(), source.clone());
                    (e2_1, source, b2, _) = inlineExp(e2.clone(), fns.clone(), source.clone());
                    (e3_1, source, b3, _) = inlineExp(e3.clone(), fns.clone(), source.clone());
                    let true = (b1 || b2 || b3) else { return Err("pattern mismatch") };
                    Ok((metamodelica::Ref::new(DAE::Statement::STMT_ASSERT { cond: e1_1.clone(), msg: e2_1.clone(), level: e3_1.clone(), source: source.clone() }), true))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Statement::STMT_TERMINATE { msg: e, source }, fns) => {
                    let mut e_1: metamodelica::Ref<DAE::Exp>;
                    let mut source = (*source).clone();
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(inlineExp(e.clone(), fns.clone(), source.clone())) {
                        (__pa0, __pa1, true, _) => (__pa0.clone(), __pa1.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    e_1 = metamodelica::Own::own(__pa0);
                    source = metamodelica::Own::own(__pa1);
                    Ok((metamodelica::Ref::new(DAE::Statement::STMT_TERMINATE { msg: e_1.clone(), source: source.clone() }), true))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Statement::STMT_REINIT { var: e1, value: e2, source }, fns) => {
                    let mut e1_1: metamodelica::Ref<DAE::Exp>;
                    let mut e2_1: metamodelica::Ref<DAE::Exp>;
                    let mut b1: bool;
                    let mut b2: bool;
                    let mut source = (*source).clone();
                    (e1_1, source, b1, _) = inlineExp(e1.clone(), fns.clone(), source.clone());
                    (e2_1, source, b2, _) = inlineExp(e2.clone(), fns.clone(), source.clone());
                    let true = (b1 || b2) else { return Err("pattern mismatch") };
                    Ok((metamodelica::Ref::new(DAE::Statement::STMT_REINIT { var: e1_1.clone(), value: e2_1.clone(), source: source.clone() }), true))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Statement::STMT_NORETCALL { exp: e, source }, fns) => {
                    let mut e_1: metamodelica::Ref<DAE::Exp>;
                    let mut source = (*source).clone();
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(inlineExp(e.clone(), fns.clone(), source.clone())) {
                        (__pa0, __pa1, true, _) => (__pa0.clone(), __pa1.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    e_1 = metamodelica::Own::own(__pa0);
                    source = metamodelica::Own::own(__pa1);
                    Ok((metamodelica::Ref::new(DAE::Statement::STMT_NORETCALL { exp: e_1.clone(), source: source.clone() }), true))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Statement::STMT_FAILURE { body: stmts, source }, fns) => {
                    let mut stmts_1: metamodelica::List<metamodelica::Ref<DAE::Statement>>;
                    let __pa0 = ::match_deref::match_deref! { match &(inlineStatements(metamodelica::AsArg::as_arg(&stmts), &(fns.clone()), metamodelica::nil(), false)) {
                        (__pa0, true) => __pa0.clone(),
                        _ => return Err("pattern mismatch"),
                    } };
                    stmts_1 = metamodelica::Own::own(__pa0);
                    Ok((metamodelica::Ref::new(DAE::Statement::STMT_FAILURE { body: stmts_1.clone(), source: source.clone() }), true))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (stmt, _) => {
                    Ok((stmt.clone(), false))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    (outStatement, inlined)
}

fn inlineElse(
    mut inElse: metamodelica::Ref<DAE::Else>,
    mut inElementList: Functiontuple,
    mut inSource: metamodelica::Ref<DAE::ElementSource>,
) -> (
    metamodelica::Ref<DAE::Else>,
    metamodelica::Ref<DAE::ElementSource>,
    bool,
) {
    let mut outElse: metamodelica::Ref<DAE::Else>;
    let mut outSource: metamodelica::Ref<DAE::ElementSource>;
    let mut inlined: bool;
    (outElse, outSource, inlined) = 'mc: {
        let __mc_input = (inElse, inElementList, inSource);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Else::ELSEIF { exp: e, statementLst: stmts, else_: a_else }, fns, source) => {
                    let mut a_else_1: metamodelica::Ref<DAE::Else>;
                    let mut e_1: metamodelica::Ref<DAE::Exp>;
                    let mut stmts_1: metamodelica::List<metamodelica::Ref<DAE::Statement>>;
                    let mut b1: bool;
                    let mut b2: bool;
                    let mut b3: bool;
                    let mut source = (*source).clone();
                    (e_1, source, b1, _) = inlineExp(e.clone(), fns.clone(), source.clone());
                    (stmts_1, b2) = inlineStatements(metamodelica::AsArg::as_arg(&stmts), &(fns.clone()), metamodelica::nil(), false);
                    (a_else_1, source, b3) = inlineElse(a_else.clone(), fns.clone(), source.clone());
                    let true = (b1 || b2 || b3) else { return Err("pattern mismatch") };
                    Ok((metamodelica::Ref::new(DAE::Else::ELSEIF { exp: e_1.clone(), statementLst: stmts_1.clone(), else_: a_else_1.clone() }), source.clone(), true))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Else::ELSE { statementLst: stmts }, fns, source) => {
                    let mut stmts_1: metamodelica::List<metamodelica::Ref<DAE::Statement>>;
                    let __pa0 = ::match_deref::match_deref! { match &(inlineStatements(metamodelica::AsArg::as_arg(&stmts), &(fns.clone()), metamodelica::nil(), false)) {
                        (__pa0, true) => __pa0.clone(),
                        _ => return Err("pattern mismatch"),
                    } };
                    stmts_1 = metamodelica::Own::own(__pa0);
                    Ok((metamodelica::Ref::new(DAE::Else::ELSE { statementLst: stmts_1.clone() }), source.clone(), true))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (a_else, _, source) => {
                    Ok((a_else.clone(), source.clone(), false))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    (outElse, outSource, inlined)
}

pub fn inlineExpOpt(
    mut inExpOption: Option<metamodelica::Ref<DAE::Exp>>,
    mut inElementList: Functiontuple,
    mut inSource: metamodelica::Ref<DAE::ElementSource>,
) -> (
    Option<metamodelica::Ref<DAE::Exp>>,
    metamodelica::Ref<DAE::ElementSource>,
    bool,
) {
    let mut outExpOption: Option<metamodelica::Ref<DAE::Exp>>;
    let mut outSource: metamodelica::Ref<DAE::ElementSource>;
    let mut inlined: bool;
    (outExpOption, outSource, inlined) = (::match_deref::match_deref! { match &(inExpOption) {
        None => {
            (None, inSource, false)
        },
        Some(exp) => {
            let mut source: metamodelica::Ref<DAE::ElementSource>;
            let mut b: bool;
            let mut exp = (*exp).clone();
            (exp, source, b, _) = inlineExp(exp.clone(), inElementList, inSource);
            (Some(exp.clone()), source, b)
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    (outExpOption, outSource, inlined)
}

pub fn inlineExp(
    mut inExp: metamodelica::Ref<DAE::Exp>,
    mut inElementList: Functiontuple,
    mut inSource: metamodelica::Ref<DAE::ElementSource>,
) -> (
    metamodelica::Ref<DAE::Exp>,
    metamodelica::Ref<DAE::ElementSource>,
    bool,
    metamodelica::List<metamodelica::Ref<DAE::Statement>>,
) {
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    let mut outSource: metamodelica::Ref<DAE::ElementSource>;
    let mut inlined: bool;
    let mut assrtLstOut: metamodelica::List<metamodelica::Ref<DAE::Statement>>;
    (outExp, outSource, inlined, assrtLstOut) = 'mc: {
        let __mc_input = (inExp.clone(), inElementList, inSource.clone());
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CREF { componentRef: Deref @ DAE::ComponentRef::WILD { .. }, .. }, _, _) => {
                    Ok((inExp.clone(), inSource.clone(), false, metamodelica::nil()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e, fns, source) => {
                    let mut e_1: metamodelica::Ref<DAE::Exp>;
                    let mut e_2: metamodelica::Ref<DAE::Exp>;
                    let mut assrtLst: metamodelica::List<metamodelica::Ref<DAE::Statement>>;
                    let mut source = (*source).clone();
                    (e_1, assrtLst) = Expression::traverseExpBottomUp(e.clone(), &({ let __pe_b2 = fns.clone(); move |__pe_a0, __pe_a1| Ok(inlineCall(__pe_a0, __pe_a1, &__pe_b2)) }), metamodelica::nil())?;
                    let false = (referenceEq(&*(e.clone()),&*(&*e_1))) else { return Err("pattern mismatch") };
                    if Flags::isSet(Flags::INFO_XML_OPERATIONS.clone())? {
                        source = ElementSource::addSymbolicTransformation(source.clone(), metamodelica::Ref::new(DAE::SymbolicOperation::OP_INLINE { before: metamodelica::Ref::new(DAE::EquationExp::PARTIAL_EQUATION { exp: e.clone() }), after: metamodelica::Ref::new(DAE::EquationExp::PARTIAL_EQUATION { exp: e_1.clone() }) }))?;
                        let (__pa0, __pa1) = ::match_deref::match_deref! { match &(ExpressionSimplify::simplifyAddSymbolicOperation(metamodelica::Ref::new(DAE::EquationExp::PARTIAL_EQUATION { exp: e_1.clone() }), source.clone())?) {
                            (Deref @ DAE::EquationExp::PARTIAL_EQUATION { exp: __pa0 }, __pa1) => (__pa0.clone(), __pa1.clone()),
                            _ => return Err("pattern mismatch"),
                        } };
                        e_2 = metamodelica::Own::own(__pa0);
                        source = metamodelica::Own::own(__pa1);
                    } else {
                        (e_2, _) = ExpressionSimplify::simplify(e_1.clone())?;
                    }
                    Ok((e_2.clone(), source.clone(), true, assrtLst.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((inExp.clone(), inSource.clone(), false, metamodelica::nil()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    (outExp, outSource, inlined, assrtLstOut)
}

pub fn forceInlineExp(
    mut inExp: metamodelica::Ref<DAE::Exp>,
    mut inElementList: Functiontuple,
    mut inSource: metamodelica::Ref<DAE::ElementSource>,
    mut cevalConst: &dyn ::std::ops::Fn(
        metamodelica::Ref<DAE::Exp>,
        metamodelica::Ref<AvlTreePathFunction::Tree>,
    ) -> Result<metamodelica::Ref<DAE::Exp>>,
) -> Result<(metamodelica::Ref<DAE::Exp>, metamodelica::Ref<DAE::ElementSource>, bool)> {
    pub type CevalConstFunc = std::sync::Arc<
        dyn ::std::ops::Fn(
                metamodelica::Ref<DAE::Exp>,
                metamodelica::Ref<AvlTreePathFunction::Tree>,
            ) -> Result<metamodelica::Ref<DAE::Exp>>
            + 'static,
    >;

    let mut outExp: metamodelica::Ref<DAE::Exp>;
    let mut outSource: metamodelica::Ref<DAE::ElementSource>;
    let mut inlineperformed: bool;
    (outExp, outSource, inlineperformed) = (::match_deref::match_deref! { match &(inElementList) {
        (Some(functionTree), _) if (Expression::isConst(inExp.clone())?) => {
            let mut e = inExp.clone();
            let mut source = inSource.clone();
            let mut e_1: metamodelica::Ref<DAE::Exp>;
            let mut b: bool;
            match '__try0: {
                e_1 = unwrap_break_err!(cevalConst(inExp.clone(), functionTree.clone()), '__try0);
                source = unwrap_break_err!(ElementSource::addSymbolicTransformation(source.clone(), metamodelica::Ref::new(DAE::SymbolicOperation::OP_INLINE { before: metamodelica::Ref::new(DAE::EquationExp::PARTIAL_EQUATION { exp: e.clone() }), after: metamodelica::Ref::new(DAE::EquationExp::PARTIAL_EQUATION { exp: e_1.clone() }) })), '__try0);
                b = true;
                Ok::<_, &'static str>((b.clone(), e_1.clone(), source.clone()))
            } {
                Ok((__try0_o0, __try0_o1, __try0_o2)) => {
                    b = __try0_o0;
                    e_1 = __try0_o1;
                    source = __try0_o2;
                }
                Err(_) => {
                    e_1 = inExp.clone();
                    source = inSource.clone();
                    b = false;
                }
            }
            (e_1, source, b)
        },
        fns => {
            let mut e = inExp.clone();
            let mut source = inSource.clone();
            let mut e_1: metamodelica::Ref<DAE::Exp>;
            let mut b: bool;
            (e_1, _) = Expression::traverseExpBottomUp(e.clone(), &({ let __pe_b2 = fns.clone(); let __pe_b3 = openmodelica_ast_collections::AvlSetPath::Tree::interned_EMPTY(); move |__pe_a0, __pe_a1| Ok(forceInlineCall(__pe_a0, __pe_a1, &__pe_b2, __pe_b3.clone())) }), metamodelica::nil())?;
            b = !(referenceEq(&*(&*e),&*(&*e_1)));
            if b {
                source = ElementSource::addSymbolicTransformation(source, metamodelica::Ref::new(DAE::SymbolicOperation::OP_INLINE { before: metamodelica::Ref::new(DAE::EquationExp::PARTIAL_EQUATION { exp: e }), after: metamodelica::Ref::new(DAE::EquationExp::PARTIAL_EQUATION { exp: e_1.clone() }) }))?;
                let (__pa0, __pa1) = ::match_deref::match_deref! { match &(ExpressionSimplify::simplifyAddSymbolicOperation(metamodelica::Ref::new(DAE::EquationExp::PARTIAL_EQUATION { exp: e_1 }), source)?) {
                    (Deref @ DAE::EquationExp::PARTIAL_EQUATION { exp: __pa0 }, __pa1) => (__pa0.clone(), __pa1.clone()),
                    _ => return Err("pattern mismatch"),
                } };
                e_1 = metamodelica::Own::own(__pa0);
                source = metamodelica::Own::own(__pa1);
            }
            (e_1, source, b)
        },
        _ => {
            (inExp.clone(), inSource, false)
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outExp, outSource, inlineperformed))
}

pub fn inlineExps(
    mut inExps: metamodelica::List<metamodelica::Ref<DAE::Exp>>,
    mut inElementList: &Functiontuple,
    mut inSource: metamodelica::Ref<DAE::ElementSource>,
) -> (
    metamodelica::List<metamodelica::Ref<DAE::Exp>>,
    metamodelica::Ref<DAE::ElementSource>,
    bool,
) {
    let mut outExps: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
    let mut outSource: metamodelica::Ref<DAE::ElementSource>;
    let mut inlined: bool;
    (outExps, outSource, inlined) = inlineExpsWork(inExps, inElementList, inSource, metamodelica::nil(), false);
    (outExps, outSource, inlined)
}

fn inlineExpsWork<'__b>(
    mut inExps: metamodelica::List<metamodelica::Ref<DAE::Exp>>,
    mut fns: &'__b Functiontuple,
    mut inSource: metamodelica::Ref<DAE::ElementSource>,
    mut iAcc: metamodelica::List<metamodelica::Ref<DAE::Exp>>,
    mut iInlined: bool,
) -> (
    metamodelica::List<metamodelica::Ref<DAE::Exp>>,
    metamodelica::Ref<DAE::ElementSource>,
    bool,
) {
    '__tco: loop {
        ::match_deref::match_deref! { match &(inExps) {
            Deref @ metamodelica::ListNode::Nil => {
                return (iAcc.reverse(), inSource, iInlined)
            },
            Deref @ metamodelica::ListNode::Cons { head: e, tail: exps } => {
                let mut source: metamodelica::Ref<DAE::ElementSource>;
                let mut b: bool;
                let mut e = (*e).clone();
                let mut exps = (*exps).clone();
                (e, source, b, _) = inlineExp(e.clone(), fns.clone(), inSource);
                { (inExps, fns, inSource, iAcc, iInlined) = (exps.clone(), fns, source, metamodelica::cons(e.clone(), iAcc), b || iInlined); continue '__tco; }
            },
            _ => unreachable!("tail-call lowered match: no arm matched"),
        } }
    }
}

pub(crate) fn checkExpsTypeEquiv(
    mut inExp1: metamodelica::Ref<DAE::Exp>,
    mut inExp2: metamodelica::Ref<DAE::Exp>,
) -> Result<bool> {
    let mut bEquiv: bool;
    bEquiv = (match &*inExp2 {
        _ => {
            let mut ty1: metamodelica::Ref<DAE::Type>;
            let mut ty2: metamodelica::Ref<DAE::Type>;
            let mut b: bool;
            if Config::acceptMetaModelicaGrammar()? {
                b = true;
            } else {
                ty1 = Expression::r#typeof(inExp1)?;
                ty2 = Expression::r#typeof(inExp2)?;
                (ty2, _) = Types::traverseType(
                    ty2,
                    -1,
                    &fnptr!(Types::makeExpDimensionsUnknown, metamodelica::Ref<DAE::Type>, i32),
                )?;
                b = Types::equivtypesOrRecordSubtypeOf(ty1, ty2);
            }
            b
        }
    });
    Ok(bEquiv)
}

pub fn inlineCall(
    mut exp: metamodelica::Ref<DAE::Exp>,
    mut assrtLst: metamodelica::List<metamodelica::Ref<DAE::Statement>>,
    mut fns: &Functiontuple,
) -> (
    metamodelica::Ref<DAE::Exp>,
    metamodelica::List<metamodelica::Ref<DAE::Statement>>,
) {
    let mut exp: metamodelica::Ref<DAE::Exp> = exp;
    let mut assrtLst: metamodelica::List<metamodelica::Ref<DAE::Statement>> = assrtLst;
    (exp, assrtLst) = 'mc: {
        let __mc_input = exp.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::CALL { attr: Deref @ DAE::CallAttributes { inlineType, .. }, .. } => {
                    let false = (Flags::isSet(Flags::INLINE_FUNCTIONS.clone())?) else { return Err("pattern mismatch") };
                    let false = (openmodelica_frontend_types::DAE::InlineType::BUILTIN_EARLY_INLINE == inlineType.clone()) else { return Err("pattern mismatch") };
                    Ok((exp.clone(), assrtLst.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Exp::CALL { path: p, expLst: _, attr: Deref @ DAE::CallAttributes { ty, .. } } => {
                    let mut newExp: metamodelica::Ref<DAE::Exp>;
                    let mut func: DAE::Function;
                    func = getFunction(p.clone(), fns)?;
                    let false = (DAEUtil::getFunctionImpureAttribute(&func)?) else { return Err("pattern mismatch") };
                    let 0 = (Types::getDimensionProduct(metamodelica::AsArg::as_arg(&ty))?) else { return Err("pattern mismatch") };
                    newExp = Expression::makeArray(metamodelica::nil(), ty.clone(), true);
                    Ok((newExp.clone(), assrtLst.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                        e1 @ Deref @ DAE::Exp::CALL { path: p, expLst: args, attr: Deref @ DAE::CallAttributes { ty, inlineType, .. } } => {
                            let mut r#fn: metamodelica::List<metamodelica::Ref<DAE::Element>>;
                            let mut crefs: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
                            let mut argmap: metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, metamodelica::Ref<DAE::Exp>)>;
                            let mut lst_cr: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
                            let mut newExp: metamodelica::Ref<DAE::Exp>;
                            let mut newExp1: metamodelica::Ref<DAE::Exp>;
                            let mut assrt: metamodelica::Ref<DAE::Statement>;
                            let mut checkcr: (metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>, (i32, i32, metamodelica::Array<Option<(metamodelica::Ref<DAE::ComponentRef>, metamodelica::Ref<DAE::ComponentRef>)>>), i32, (HashTableCG::FuncHashCref, HashTableCG::FuncCrefEqual, HashTableCG::FuncCrefStr, HashTableCG::FuncExpStr));
                            let mut stmts: metamodelica::List<metamodelica::Ref<DAE::Statement>>;
                            let mut assrtStmts: metamodelica::List<metamodelica::Ref<DAE::Statement>>;
                            let mut repl: VarTransform::VariableReplacements;
                            let mut generateEvents: bool;
                            let mut comment: Option<metamodelica::Ref<SCode::Comment>>;
                            let mut assrtLst: metamodelica::List<metamodelica::Ref<DAE::Statement>> = assrtLst.clone();
                            let true = (checkInlineType(inlineType.clone(), fns)) else { return Err("pattern mismatch") };
                            (r#fn, comment) = getFunctionBody(p.clone(), fns)?;
                            (checkcr, repl) = getInlineHashTableVarTransform();
                            if Config::acceptMetaModelicaGrammar()? {
                                crefs = List::map(r#fn.clone(), &move |__a0: metamodelica::Ref<DAE::Element>| -> metamodelica::Result<_> { ::std::result::Result::Ok(getInputCrefs(&__a0)) })?;
                                crefs = List::select(crefs.clone(), (std::sync::Arc::new(move |__a0: metamodelica::Ref<DAE::ComponentRef>| -> metamodelica::Result<_> { ::std::result::Result::Ok(removeWilds(&__a0)) }) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>) -> Result<bool> + 'static>))?;
                                argmap = List::zip(crefs.clone(), args.clone());
                                let false = (List::any(&r#fn, &move |__a0: metamodelica::Ref<DAE::Element>| -> metamodelica::Result<_> { ::std::result::Result::Ok(DAEUtil::isProtectedVar(&__a0)) })?) else { return Err("pattern mismatch") };
                                newExp = getRhsExp(&r#fn)?;
                                let true = (checkExpsTypeEquiv(e1.clone(), newExp.clone())?) else { return Err("pattern mismatch") };
                                (argmap, checkcr) = extendCrefRecords(&argmap, checkcr.clone())?;
                                newExp = Expression::addNoEventToRelationsAndConds(newExp.clone())?;
                                let __pa0 = ::match_deref::match_deref! { match &(Expression::traverseExpBottomUp(newExp.clone(), &fnptr!(replaceArgs, metamodelica::Ref<DAE::Exp>, (metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, metamodelica::Ref<DAE::Exp>)>, (metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>, (i32, i32, metamodelica::Array<Option<(metamodelica::Ref<DAE::ComponentRef>, metamodelica::Ref<DAE::ComponentRef>)>>), i32, (Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>, metamodelica::Ref<DAE::ComponentRef>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>) -> Result<ArcStr> + 'static>, Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>) -> Result<ArcStr> + 'static>)), bool)), (argmap.clone(), checkcr.clone(), true))?) {
                                    (__pa0, (_, _, true)) => __pa0.clone(),
                                    _ => return Err("pattern mismatch"),
                                } };
                                newExp = metamodelica::Own::own(__pa0);
                                (newExp1, assrtLst) = Expression::traverseExpBottomUp(newExp.clone(), &({ let __pe_b2 = fns.clone(); move |__pe_a0, __pe_a1| Ok(inlineCall(__pe_a0, __pe_a1, &__pe_b2)) }), assrtLst.clone())?;
                            } else {
                                (crefs, lst_cr, stmts, repl) = getFunctionInputsOutputBody(&r#fn, repl.clone())?;
                                (repl, assrtStmts) = mergeFunctionBody(&stmts, repl.clone(), metamodelica::nil())?;
                                if (assrtStmts).is_empty() {
                                    newExp = Expression::makeTuple(({
                let mut __acc: metamodelica::List<metamodelica::Ref<DAE::Exp>> = metamodelica::nil();
                for mut cr in (lst_cr.clone()).into_iter().cloned() {
                            let __x = getReplacementCheckComplex(repl.clone(), cr.clone(), ty.clone())?;
                            __acc = cons(__x, __acc);
                }
                __acc.reverse()
            }))?;
                                    let true = (checkExpsTypeEquiv(e1.clone(), newExp.clone())?) else { return Err("pattern mismatch") };
                                    argmap = List::zip(crefs.clone(), args.clone());
                                    (checkcr, _) = getInlineHashTableVarTransform();
                                    (argmap, checkcr) = extendCrefRecords(&argmap, checkcr.clone())?;
                                    generateEvents = hasGenerateEventsAnnotation(comment.clone());
                                    newExp = if (!(generateEvents)) {Expression::addNoEventToRelationsAndConds(newExp.clone())?} else {newExp.clone()};
                                    let __pa1 = ::match_deref::match_deref! { match &(Expression::traverseExpBottomUp(newExp.clone(), &fnptr!(replaceArgs, metamodelica::Ref<DAE::Exp>, (metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, metamodelica::Ref<DAE::Exp>)>, (metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>, (i32, i32, metamodelica::Array<Option<(metamodelica::Ref<DAE::ComponentRef>, metamodelica::Ref<DAE::ComponentRef>)>>), i32, (Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>, metamodelica::Ref<DAE::ComponentRef>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>) -> Result<ArcStr> + 'static>, Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>) -> Result<ArcStr> + 'static>)), bool)), (argmap.clone(), checkcr.clone(), true))?) {
                                                (__pa1, (_, _, true)) => __pa1.clone(),
                                                _ => return Err("pattern mismatch"),
                                    } };
                                    newExp = metamodelica::Own::own(__pa1);
                                    (newExp1, assrtLst) = Expression::traverseExpBottomUp(newExp.clone(), &({ let __pe_b2 = fns.clone(); move |__pe_a0, __pe_a1| Ok(inlineCall(__pe_a0, __pe_a1, &__pe_b2)) }), assrtLst.clone())?;
                                } else {
                                    let true = (((assrtStmts).len() as i32) == 1) else { return Err("pattern mismatch") };
                                    assrt = (assrtStmts).head().cloned()?;
                                    ::match_deref::match_deref! { match &(assrt.clone()) {
                                                Deref @ DAE::Statement::STMT_ASSERT { .. } => (),
                                                _ => return Err("pattern mismatch"),
                                    } };
                                    newExp = Expression::makeTuple(({
                let mut __acc: metamodelica::List<metamodelica::Ref<DAE::Exp>> = metamodelica::nil();
                for mut cr in (lst_cr.clone()).into_iter().cloned() {
                            let __x = getReplacementCheckComplex(repl.clone(), cr.clone(), ty.clone())?;
                            __acc = cons(__x, __acc);
                }
                __acc.reverse()
            }))?;
                                    let true = (checkExpsTypeEquiv(e1.clone(), newExp.clone())?) else { return Err("pattern mismatch") };
                                    argmap = List::zip(crefs.clone(), args.clone());
                                    (argmap, checkcr) = extendCrefRecords(&argmap, checkcr.clone())?;
                                    generateEvents = hasGenerateEventsAnnotation(comment.clone());
                                    newExp = if (!(generateEvents)) {Expression::addNoEventToRelationsAndConds(newExp.clone())?} else {newExp.clone()};
                                    let __pa2 = ::match_deref::match_deref! { match &(Expression::traverseExpBottomUp(newExp.clone(), &fnptr!(replaceArgs, metamodelica::Ref<DAE::Exp>, (metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, metamodelica::Ref<DAE::Exp>)>, (metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>, (i32, i32, metamodelica::Array<Option<(metamodelica::Ref<DAE::ComponentRef>, metamodelica::Ref<DAE::ComponentRef>)>>), i32, (Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>, metamodelica::Ref<DAE::ComponentRef>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>) -> Result<ArcStr> + 'static>, Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>) -> Result<ArcStr> + 'static>)), bool)), (argmap.clone(), checkcr.clone(), true))?) {
                                                (__pa2, (_, _, true)) => __pa2.clone(),
                                                _ => return Err("pattern mismatch"),
                                    } };
                                    newExp = metamodelica::Own::own(__pa2);
                                    assrt = inlineAssert(&assrt, fns, argmap.clone(), checkcr.clone())?;
                                    (newExp1, assrtLst) = Expression::traverseExpBottomUp(newExp.clone(), &({ let __pe_b2 = fns.clone(); move |__pe_a0, __pe_a1| Ok(inlineCall(__pe_a0, __pe_a1, &__pe_b2)) }), metamodelica::cons(assrt.clone(), assrtLst.clone()))?;
                                }
                            }
                            Ok(((newExp1.clone(), assrtLst.clone()), assrtLst.clone()))
                        }
                        _ => return Err("nomatch"),
                    }}
        })() {
            assrtLst = __wb0;
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((exp.clone(), assrtLst.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    (exp, assrtLst)
}

fn inlineAssert(
    mut assrtIn: &metamodelica::Ref<DAE::Statement>,
    mut fns: &Functiontuple,
    mut argmap: metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, metamodelica::Ref<DAE::Exp>)>,
    mut checkcr: (
        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<
                Option<(
                    metamodelica::Ref<DAE::ComponentRef>,
                    metamodelica::Ref<DAE::ComponentRef>,
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
            Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>) -> Result<ArcStr> + 'static>,
        ),
    ),
) -> Result<metamodelica::Ref<DAE::Statement>> {
    let mut assrtOut: metamodelica::Ref<DAE::Statement>;
    let mut source: metamodelica::Ref<DAE::ElementSource>;
    let mut cond: metamodelica::Ref<DAE::Exp>;
    let mut msg: metamodelica::Ref<DAE::Exp>;
    let mut level: metamodelica::Ref<DAE::Exp>;
    let (__pa0, __pa1, __pa2, __pa3) = ::match_deref::match_deref! { match &((*assrtIn)) {
        Deref @ DAE::Statement::STMT_ASSERT { cond: __pa0, msg: __pa1, level: __pa2, source: __pa3 } => (__pa0.clone(), __pa1.clone(), __pa2.clone(), __pa3.clone()),
        _ => return Err("pattern mismatch"),
    } };
    cond = metamodelica::Own::own(__pa0);
    msg = metamodelica::Own::own(__pa1);
    level = metamodelica::Own::own(__pa2);
    source = metamodelica::Own::own(__pa3);
    let __pa4 = ::match_deref::match_deref! { match &(Expression::traverseExpBottomUp(cond, &fnptr!(replaceArgs, metamodelica::Ref<DAE::Exp>, (metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, metamodelica::Ref<DAE::Exp>)>, (metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>, (i32, i32, metamodelica::Array<Option<(metamodelica::Ref<DAE::ComponentRef>, metamodelica::Ref<DAE::ComponentRef>)>>), i32, (Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>, metamodelica::Ref<DAE::ComponentRef>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>) -> Result<ArcStr> + 'static>, Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>) -> Result<ArcStr> + 'static>)), bool)), (argmap.clone(), checkcr.clone(), true))?) {
        (__pa4, (_, _, true)) => __pa4.clone(),
        _ => return Err("pattern mismatch"),
    } };
    cond = metamodelica::Own::own(__pa4);
    let __pa5 = ::match_deref::match_deref! { match &(Expression::traverseExpBottomUp(msg, &fnptr!(replaceArgs, metamodelica::Ref<DAE::Exp>, (metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, metamodelica::Ref<DAE::Exp>)>, (metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>, (i32, i32, metamodelica::Array<Option<(metamodelica::Ref<DAE::ComponentRef>, metamodelica::Ref<DAE::ComponentRef>)>>), i32, (Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>, metamodelica::Ref<DAE::ComponentRef>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>) -> Result<ArcStr> + 'static>, Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>) -> Result<ArcStr> + 'static>)), bool)), (argmap, checkcr, true))?) {
        (__pa5, (_, _, true)) => __pa5.clone(),
        _ => return Err("pattern mismatch"),
    } };
    msg = metamodelica::Own::own(__pa5);
    assrtOut = metamodelica::Ref::new(DAE::Statement::STMT_ASSERT {
        cond: cond,
        msg: msg,
        level: level,
        source: source,
    });
    Ok(assrtOut)
}

pub fn hasGenerateEventsAnnotation(mut comment: Option<metamodelica::Ref<SCode::Comment>>) -> bool {
    let mut b: bool;
    b = (::match_deref::match_deref! { match &(comment) {
        Some(Deref @ SCode::Comment { annotation_: Some(anno), .. }) => {
            SCodeUtil::hasBooleanNamedAnnotation(metamodelica::AsArg::as_arg(&anno), &(literal!("GenerateEvents")))
        },
        _ => {
            false
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    b
}

fn dumpArgmap(mut inTpl: &(metamodelica::Ref<DAE::ComponentRef>, metamodelica::Ref<DAE::Exp>)) -> Result<()> {
    let mut cr: metamodelica::Ref<DAE::ComponentRef>;
    let mut exp: metamodelica::Ref<DAE::Exp>;
    (cr, exp) = inTpl.clone();
    metamodelica::print({
        let mut __mm_s = String::new();
        __mm_s.push_str(&*ComponentReferenceBasics::printComponentRefStr(&cr)?);
        __mm_s.push_str(&*literal!(" -> "));
        __mm_s.push_str(&*ExpressionBasics::printExpStr(exp)?);
        __mm_s.push_str(&*literal!("\n"));
        ArcStr::from(__mm_s)
    });
    Ok(())
}

pub fn forceInlineCall(
    mut exp: metamodelica::Ref<DAE::Exp>,
    mut assrtLst: metamodelica::List<metamodelica::Ref<DAE::Statement>>,
    mut fns: &Functiontuple,
    mut visitedPaths: metamodelica::Ref<AvlSetPath::Tree>,
) -> (
    metamodelica::Ref<DAE::Exp>,
    metamodelica::List<metamodelica::Ref<DAE::Statement>>,
) {
    let mut exp: metamodelica::Ref<DAE::Exp> = exp;
    let mut assrtLst: metamodelica::List<metamodelica::Ref<DAE::Statement>> = assrtLst;
    (exp, assrtLst) = 'mc: {
        let __mc_input = exp.clone();
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                        e1 @ Deref @ DAE::Exp::CALL { path: p, expLst: args, attr: Deref @ DAE::CallAttributes { inlineType, .. } } => {
                            if !((!(AvlSetPath::hasKey(visitedPaths.clone(), p.clone())?))) { return Err("guard") }
                            let mut r#fn: metamodelica::List<metamodelica::Ref<DAE::Element>>;
                            let mut lst_cr: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
                            let mut crefs: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
                            let mut argmap: metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, metamodelica::Ref<DAE::Exp>)>;
                            let mut newExp: metamodelica::Ref<DAE::Exp>;
                            let mut newExp1: metamodelica::Ref<DAE::Exp>;
                            let mut checkcr: (metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>, (i32, i32, metamodelica::Array<Option<(metamodelica::Ref<DAE::ComponentRef>, metamodelica::Ref<DAE::ComponentRef>)>>), i32, (HashTableCG::FuncHashCref, HashTableCG::FuncCrefEqual, HashTableCG::FuncCrefStr, HashTableCG::FuncExpStr));
                            let mut stmts: metamodelica::List<metamodelica::Ref<DAE::Statement>>;
                            let mut repl: VarTransform::VariableReplacements;
                            let mut generateEvents: bool;
                            let mut comment: Option<metamodelica::Ref<SCode::Comment>>;
                            let mut assrtLst: metamodelica::List<metamodelica::Ref<DAE::Statement>> = assrtLst.clone();
                            let false = (Config::acceptMetaModelicaGrammar()?) else { return Err("pattern mismatch") };
                            let true = (checkInlineType(inlineType.clone(), fns)) else { return Err("pattern mismatch") };
                            (r#fn, comment) = getFunctionBody(p.clone(), fns)?;
                            (checkcr, repl) = getInlineHashTableVarTransform();
                            (crefs, lst_cr, stmts, repl) = getFunctionInputsOutputBody(&r#fn, repl.clone())?;
                            (repl, _) = mergeFunctionBody(&stmts, repl.clone(), metamodelica::nil())?;
                            newExp = Expression::makeTuple(({
                let mut __acc: metamodelica::List<metamodelica::Ref<DAE::Exp>> = metamodelica::nil();
                for mut cr in (lst_cr.clone()).into_iter().cloned() {
                            let __x = VarTransform::getReplacement(&repl, cr.clone())?;
                            __acc = cons(__x, __acc);
                }
                __acc.reverse()
            }))?;
                            let true = (checkExpsTypeEquiv(e1.clone(), newExp.clone())?) else { return Err("pattern mismatch") };
                            argmap = List::zip(crefs.clone(), args.clone());
                            (argmap, checkcr) = extendCrefRecords(&argmap, checkcr.clone())?;
                            generateEvents = hasGenerateEventsAnnotation(comment.clone());
                            newExp = if (!(generateEvents)) {Expression::addNoEventToRelationsAndConds(newExp.clone())?} else {newExp.clone()};
                            let __pa0 = ::match_deref::match_deref! { match &(Expression::traverseExpBottomUp(newExp.clone(), &fnptr!(replaceArgs, metamodelica::Ref<DAE::Exp>, (metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, metamodelica::Ref<DAE::Exp>)>, (metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>, (i32, i32, metamodelica::Array<Option<(metamodelica::Ref<DAE::ComponentRef>, metamodelica::Ref<DAE::ComponentRef>)>>), i32, (Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>) -> Result<i32> + 'static>, Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>, metamodelica::Ref<DAE::ComponentRef>) -> Result<bool> + 'static>, Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>) -> Result<ArcStr> + 'static>, Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>) -> Result<ArcStr> + 'static>)), bool)), (argmap.clone(), checkcr.clone(), true))?) {
                                (__pa0, (_, _, true)) => __pa0.clone(),
                                _ => return Err("pattern mismatch"),
                            } };
                            newExp = metamodelica::Own::own(__pa0);
                            (newExp1, assrtLst) = Expression::traverseExpBottomUp(newExp.clone(), &({ let __pe_b2 = fns.clone(); let __pe_b3 = AvlSetPath::add(visitedPaths.clone(), metamodelica::AsArg::as_arg(&p))?; move |__pe_a0, __pe_a1| Ok(forceInlineCall(__pe_a0, __pe_a1, &__pe_b2, __pe_b3.clone())) }), assrtLst.clone())?;
                            Ok(((newExp1.clone(), assrtLst.clone()), assrtLst.clone()))
                        }
                        _ => return Err("nomatch"),
                    }}
        })() {
            assrtLst = __wb0;
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((exp.clone(), assrtLst.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    (exp, assrtLst)
}

fn mergeFunctionBody<'__b>(
    mut iStmts: &'__b metamodelica::List<metamodelica::Ref<DAE::Statement>>,
    mut iRepl: VarTransform::VariableReplacements,
    mut assertStmtsIn: metamodelica::List<metamodelica::Ref<DAE::Statement>>,
) -> Result<(
    VarTransform::VariableReplacements,
    metamodelica::List<metamodelica::Ref<DAE::Statement>>,
)> {
    '__tco: loop {
        ::match_deref::match_deref! { match iStmts {
            Deref @ metamodelica::ListNode::Nil => {
                return Ok((iRepl, assertStmtsIn))
            },
            Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Statement::STMT_ASSIGN { exp1: Deref @ DAE::Exp::CREF { componentRef: cr, .. }, exp, .. }, tail: stmts } => {
                let mut repl: VarTransform::VariableReplacements;
                let mut assertStmts: metamodelica::List<metamodelica::Ref<DAE::Statement>>;
                let mut exp = (*exp).clone();
                (exp, _) = VarTransform::replaceExp(exp.clone(), &iRepl, None)?;
                repl = VarTransform::addReplacementNoTransitive(iRepl, cr.clone(), exp.clone())?;
                { (iStmts, iRepl, assertStmtsIn) = (stmts, repl, assertStmtsIn); continue '__tco; }
            },
            Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Statement::STMT_ASSIGN_ARR { lhs: Deref @ DAE::Exp::CREF { componentRef: cr, .. }, exp, .. }, tail: stmts } => {
                let mut repl: VarTransform::VariableReplacements;
                let mut assertStmts: metamodelica::List<metamodelica::Ref<DAE::Statement>>;
                let mut exp = (*exp).clone();
                (exp, _) = VarTransform::replaceExp(exp.clone(), &iRepl, None)?;
                repl = VarTransform::addReplacementNoTransitive(iRepl, cr.clone(), exp.clone())?;
                { (iStmts, iRepl, assertStmtsIn) = (stmts, repl, assertStmtsIn); continue '__tco; }
            },
            Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Statement::STMT_TUPLE_ASSIGN { expExpLst: explst, exp, .. }, tail: stmts } => {
                let mut repl: VarTransform::VariableReplacements;
                let mut assertStmts: metamodelica::List<metamodelica::Ref<DAE::Statement>>;
                let mut exp = (*exp).clone();
                (exp, _) = VarTransform::replaceExp(exp.clone(), &iRepl, None)?;
                repl = addTplAssignToRepl(metamodelica::AsArg::as_arg(&explst), 1, metamodelica::AsArg::as_arg(&exp), iRepl)?;
                { (iStmts, iRepl, assertStmtsIn) = (stmts, repl, assertStmtsIn); continue '__tco; }
            },
            Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Statement::STMT_ASSERT { cond: exp, msg: exp1, level: exp2, source }, tail: stmts } => {
                let mut repl: VarTransform::VariableReplacements;
                let mut stmt: metamodelica::Ref<DAE::Statement>;
                let mut assertStmts: metamodelica::List<metamodelica::Ref<DAE::Statement>>;
                let mut exp = (*exp).clone();
                let mut exp1 = (*exp1).clone();
                let mut exp2 = (*exp2).clone();
                (exp, _) = VarTransform::replaceExp(exp.clone(), &iRepl, None)?;
                (exp1, _) = VarTransform::replaceExp(exp1.clone(), &iRepl, None)?;
                (exp2, _) = VarTransform::replaceExp(exp2.clone(), &iRepl, None)?;
                stmt = metamodelica::Ref::new(DAE::Statement::STMT_ASSERT { cond: exp.clone(), msg: exp1.clone(), level: exp2.clone(), source: source.clone() });
                { (iStmts, iRepl, assertStmtsIn) = (stmts, iRepl, metamodelica::cons(stmt, assertStmtsIn)); continue '__tco; }
            },
            Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Statement::STMT_IF { exp, statementLst: Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Statement::STMT_ASSIGN { exp1: Deref @ DAE::Exp::CREF { componentRef: cr1, .. }, exp: exp1, .. }, tail: Deref @ metamodelica::ListNode::Nil }, else_: Deref @ DAE::Else::ELSE { statementLst: Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Statement::STMT_ASSIGN { exp1: Deref @ DAE::Exp::CREF { componentRef: cr2, .. }, exp: exp2, .. }, tail: Deref @ metamodelica::ListNode::Nil } }, .. }, tail: stmts } if (ComponentReferenceBasics::crefEqual(metamodelica::AsArg::as_arg(&cr1), metamodelica::AsArg::as_arg(&cr2))?) => {
                let mut repl: VarTransform::VariableReplacements;
                let mut assertStmts: metamodelica::List<metamodelica::Ref<DAE::Statement>>;
                let mut exp = (*exp).clone();
                let mut exp1 = (*exp1).clone();
                let mut exp2 = (*exp2).clone();
                (exp, _) = VarTransform::replaceExp(exp.clone(), &iRepl, None)?;
                (exp1, _) = VarTransform::replaceExp(exp1.clone(), &iRepl, None)?;
                (exp2, _) = VarTransform::replaceExp(exp2.clone(), &iRepl, None)?;
                repl = VarTransform::addReplacementNoTransitive(iRepl, cr1.clone(), metamodelica::Ref::new(DAE::Exp::IFEXP { expCond: exp.clone(), expThen: exp1.clone(), expElse: exp2.clone() }))?;
                { (iStmts, iRepl, assertStmtsIn) = (stmts, repl, assertStmtsIn); continue '__tco; }
            },
            Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Statement::STMT_IF { exp, statementLst: Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Statement::STMT_ASSIGN_ARR { lhs: Deref @ DAE::Exp::CREF { componentRef: cr1, .. }, exp: exp1, .. }, tail: Deref @ metamodelica::ListNode::Nil }, else_: Deref @ DAE::Else::ELSE { statementLst: Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Statement::STMT_ASSIGN_ARR { lhs: Deref @ DAE::Exp::CREF { componentRef: cr2, .. }, exp: exp2, .. }, tail: Deref @ metamodelica::ListNode::Nil } }, .. }, tail: stmts } if (ComponentReferenceBasics::crefEqual(metamodelica::AsArg::as_arg(&cr1), metamodelica::AsArg::as_arg(&cr2))?) => {
                let mut repl: VarTransform::VariableReplacements;
                let mut assertStmts: metamodelica::List<metamodelica::Ref<DAE::Statement>>;
                let mut exp = (*exp).clone();
                let mut exp1 = (*exp1).clone();
                let mut exp2 = (*exp2).clone();
                (exp, _) = VarTransform::replaceExp(exp.clone(), &iRepl, None)?;
                (exp1, _) = VarTransform::replaceExp(exp1.clone(), &iRepl, None)?;
                (exp2, _) = VarTransform::replaceExp(exp2.clone(), &iRepl, None)?;
                repl = VarTransform::addReplacementNoTransitive(iRepl, cr1.clone(), metamodelica::Ref::new(DAE::Exp::IFEXP { expCond: exp.clone(), expThen: exp1.clone(), expElse: exp2.clone() }))?;
                { (iStmts, iRepl, assertStmtsIn) = (stmts, repl, assertStmtsIn); continue '__tco; }
            },
            _ => return Err("match: no arm matched"),
        } }
    }
}

fn addTplAssignToRepl<'__b>(
    mut explst: &'__b metamodelica::List<metamodelica::Ref<DAE::Exp>>,
    mut indx: i32,
    mut iExp: &'__b metamodelica::Ref<DAE::Exp>,
    mut iRepl: VarTransform::VariableReplacements,
) -> Result<VarTransform::VariableReplacements> {
    '__tco: loop {
        ::match_deref::match_deref! { match explst {
            Deref @ metamodelica::ListNode::Nil => {
                return Ok(iRepl)
            },
            Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Exp::CREF { componentRef: cr, ty: tp }, tail: rest } => {
                let mut repl: VarTransform::VariableReplacements;
                let mut exp: metamodelica::Ref<DAE::Exp>;
                exp = metamodelica::Ref::new(DAE::Exp::TSUB { exp: iExp.clone(), ix: indx, ty: tp.clone() });
                repl = VarTransform::addReplacementNoTransitive(iRepl, cr.clone(), exp)?;
                { (explst, indx, iExp, iRepl) = (rest, indx + 1, iExp, repl); continue '__tco; }
            },
            _ => return Err("match: no arm matched"),
        } }
    }
}

fn getFunctionInputsOutputBody(
    mut r#fn: &metamodelica::List<metamodelica::Ref<DAE::Element>>,
    mut iRepl: VarTransform::VariableReplacements,
) -> Result<(
    metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
    metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
    metamodelica::List<metamodelica::Ref<DAE::Statement>>,
    VarTransform::VariableReplacements,
)> {
    let mut oInputs: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>> = metamodelica::nil();
    let mut oOutputs: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>> = metamodelica::nil();
    let mut oBody: metamodelica::List<metamodelica::Ref<DAE::Statement>> = metamodelica::nil();
    let mut oRepl: VarTransform::VariableReplacements = iRepl;
    let mut elt: metamodelica::Ref<DAE::Element> =
        <metamodelica::Ref<DAE::Element> as ::std::default::Default>::default();
    let mut cr: metamodelica::Ref<DAE::ComponentRef>;
    let mut binding: Option<metamodelica::Ref<DAE::Exp>>;
    let mut tp: metamodelica::Ref<DAE::Type>;
    let mut st: metamodelica::List<metamodelica::Ref<DAE::Statement>>;
    for mut elt in &**r#fn {
        let mut elt = elt.clone();
        let () = (::match_deref::match_deref! { match &(elt.clone()) {
            Deref @ DAE::Element::VAR { componentRef: __esc_cr, direction: DAE::VarDirection::INPUT { .. }, .. } => {
                cr = (*__esc_cr).clone();
                oInputs = metamodelica::cons(cr.clone(), oInputs);
                ()
            },
            Deref @ DAE::Element::VAR { componentRef: __esc_cr, direction: DAE::VarDirection::OUTPUT { .. }, binding: __esc_binding, ty: __elt_ty, .. } => {
                cr = (*__esc_cr).clone();
                binding = (*__esc_binding).clone();
                binding = makeComplexBinding(binding.clone(), __elt_ty.clone());
                oRepl = addOptBindingReplacements(cr.clone(), binding.clone(), oRepl)?;
                oOutputs = metamodelica::cons(cr.clone(), oOutputs);
                ()
            },
            Deref @ DAE::Element::VAR { componentRef: __esc_cr, protection: DAE::VarVisibility::PROTECTED { .. }, binding: __esc_binding, .. } => {
                cr = (*__esc_cr).clone();
                binding = (*__esc_binding).clone();
                tp = ComponentReference::crefTypeFull(metamodelica::AsArg::as_arg(&cr))?;
                let false = (Expression::isArrayType(&tp)) else { return Err("pattern mismatch") };
                let false = (Expression::isRecordType(&tp)) else { return Err("pattern mismatch") };
                oRepl = addOptBindingReplacements(cr.clone(), binding.clone(), oRepl)?;
                ()
            },
            Deref @ DAE::Element::ALGORITHM { algorithm_: Deref @ DAE::Algorithm { statementLst: __esc_st }, .. } => {
                st = (*__esc_st).clone();
                oBody = List::append_reverse(metamodelica::AsArg::as_arg(&st), oBody);
                ()
            },
            _ => {
                Error::addInternalError({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Unknown element: ")); __mm_s.push_str(&*DAEDump::dumpElementsStr(&(list![elt]))?); ArcStr::from(__mm_s) }, metamodelica::sourceInfo!("FrontEnd/Inline.mo"))?;
                return Err("fail")
            },
            _ => unreachable!("match_deref! exhaustiveness placeholder"),
        } });
    }
    oInputs = oInputs.reverse();
    oOutputs = oOutputs.reverse();
    oBody = oBody.reverse();
    Ok((oInputs, oOutputs, oBody, oRepl))
}

fn makeComplexBinding(
    mut binding: Option<metamodelica::Ref<DAE::Exp>>,
    mut ty: metamodelica::Ref<DAE::Type>,
) -> Option<metamodelica::Ref<DAE::Exp>> {
    let mut binding: Option<metamodelica::Ref<DAE::Exp>> = binding;
    binding = (::match_deref::match_deref! { match &((binding.clone(), ty.clone())) {
        (None, Deref @ DAE::Type::T_COMPLEX { .. }) => {
            let mut expl: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
            let mut strl: metamodelica::List<ArcStr>;
            let mut exp: metamodelica::Ref<DAE::Exp>;
            expl = metamodelica::nil();
            strl = metamodelica::nil();
            for mut var in &*var_field!((*ty).varLst, DAE::Type::T_COMPLEX).clone().reverse() {
                let () = (::match_deref::match_deref! { match &(var.clone()) {
        Deref @ DAE::Var { binding: Deref @ DAE::Binding::EQBOUND { exp: __esc_exp, .. }, .. } => {
            exp = (*__esc_exp).clone();
            expl = metamodelica::cons(exp.clone(), expl);
            strl = metamodelica::cons(var.name.clone(), strl);
            ()
        },
        _ => {
            return binding;
            ()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
            }
            Some(metamodelica::Ref::new(DAE::Exp::RECORD { path: ClassInfUtil::getStateName(var_field!((*ty).complexClassType, DAE::Type::T_COMPLEX)), exps: expl, comp: strl, ty: ty }))
        },
        _ => {
            binding
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    binding
}

fn addOptBindingReplacements(
    mut cr: metamodelica::Ref<DAE::ComponentRef>,
    mut binding: Option<metamodelica::Ref<DAE::Exp>>,
    mut iRepl: VarTransform::VariableReplacements,
) -> Result<VarTransform::VariableReplacements> {
    let mut oRepl: VarTransform::VariableReplacements;
    oRepl = (::match_deref::match_deref! { match &(binding) {
        Some(e) => {
            addReplacement(cr, e.clone(), iRepl)?
        },
        None => {
            iRepl
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(oRepl)
}

fn addReplacement(
    mut iCr: metamodelica::Ref<DAE::ComponentRef>,
    mut iExp: metamodelica::Ref<DAE::Exp>,
    mut iRepl: VarTransform::VariableReplacements,
) -> Result<VarTransform::VariableReplacements> {
    let mut oRepl: VarTransform::VariableReplacements;
    oRepl = (match &*iCr {
        DAE::ComponentRef::CREF_IDENT { .. } => VarTransform::addReplacement(iRepl, iCr, iExp)?,
        _ => return Err("fail"),
    });
    Ok(oRepl)
}

pub fn checkInlineType(mut inIT: DAE::InlineType, mut fns: &Functiontuple) -> bool {
    let mut outb: bool;
    outb = (::match_deref::match_deref! { match &(fns) {
        (_, itlst) => {
            let mut it = inIT;
            let mut b: bool;
            b = listMember(it, itlst.clone());
            b
        },
        _ => {
            false
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outb
}

// TODO: mahge: This needs to be rewritten completely.
pub fn extendCrefRecords(
    mut inArgmap: &metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, metamodelica::Ref<DAE::Exp>)>,
    mut inCheckCr: (
        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<
                Option<(
                    metamodelica::Ref<DAE::ComponentRef>,
                    metamodelica::Ref<DAE::ComponentRef>,
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
            Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>) -> Result<ArcStr> + 'static>,
        ),
    ),
) -> Result<(
    metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, metamodelica::Ref<DAE::Exp>)>,
    (
        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<
                Option<(
                    metamodelica::Ref<DAE::ComponentRef>,
                    metamodelica::Ref<DAE::ComponentRef>,
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
            Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>) -> Result<ArcStr> + 'static>,
        ),
    ),
)> {
    let mut outArgmap: metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, metamodelica::Ref<DAE::Exp>)>;
    let mut outCheckCr: (
        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<
                Option<(
                    metamodelica::Ref<DAE::ComponentRef>,
                    metamodelica::Ref<DAE::ComponentRef>,
                )>,
            >,
        ),
        i32,
        (
            HashTableCG::FuncHashCref,
            HashTableCG::FuncCrefEqual,
            HashTableCG::FuncCrefStr,
            HashTableCG::FuncExpStr,
        ),
    );
    (outArgmap, outCheckCr) = 'mc: {
        let __mc_input = (&**inArgmap, inCheckCr);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Nil, ht) => {
                    Ok((metamodelica::nil(), ht.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: (c, Deref @ DAE::Exp::CAST { exp: e, ty: Deref @ DAE::Type::T_COMPLEX { .. } }), tail: res }, ht) => {
                    let mut ht1: (metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>, (i32, i32, metamodelica::Array<Option<(metamodelica::Ref<DAE::ComponentRef>, metamodelica::Ref<DAE::ComponentRef>)>>), i32, (HashTableCG::FuncHashCref, HashTableCG::FuncCrefEqual, HashTableCG::FuncCrefStr, HashTableCG::FuncExpStr));
                    let mut new1: metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, metamodelica::Ref<DAE::Exp>)>;
                    (new1, ht1) = extendCrefRecords(&(metamodelica::cons((c.clone(), e.clone()), res.clone())), ht.clone())?;
                    Ok((new1.clone(), ht1.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: (c, e @ Deref @ DAE::Exp::CREF { componentRef: cref, ty: Deref @ DAE::Type::T_COMPLEX { varLst, .. } }), tail: res }, ht) => {
                    let mut ht1: (metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>, (i32, i32, metamodelica::Array<Option<(metamodelica::Ref<DAE::ComponentRef>, metamodelica::Ref<DAE::ComponentRef>)>>), i32, (HashTableCG::FuncHashCref, HashTableCG::FuncCrefEqual, HashTableCG::FuncCrefStr, HashTableCG::FuncExpStr));
                    let mut ht2: (metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>, (i32, i32, metamodelica::Array<Option<(metamodelica::Ref<DAE::ComponentRef>, metamodelica::Ref<DAE::ComponentRef>)>>), i32, (HashTableCG::FuncHashCref, HashTableCG::FuncCrefEqual, HashTableCG::FuncCrefStr, HashTableCG::FuncExpStr));
                    let mut res1: metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, metamodelica::Ref<DAE::Exp>)>;
                    let mut res2: metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, metamodelica::Ref<DAE::Exp>)>;
                    let mut new: metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, metamodelica::Ref<DAE::Exp>)>;
                    let mut new1: metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, metamodelica::Ref<DAE::Exp>)>;
                    (res1, ht1) = extendCrefRecords(metamodelica::AsArg::as_arg(&res), ht.clone())?;
                    new = List::map2(varLst.clone(), &move |__a0: metamodelica::Ref<DAE::Var>, __a1: metamodelica::Ref<DAE::ComponentRef>, __a2: metamodelica::Ref<DAE::ComponentRef>| extendCrefRecords1(&__a0, &__a1, &__a2), c.clone(), cref.clone())?;
                    (new1, ht2) = extendCrefRecords(&new, ht1.clone())?;
                    res2 = listAppend(new1.clone(), res1.clone());
                    Ok((metamodelica::cons((c.clone(), e.clone()), res2.clone()), ht2.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: (c, e @ Deref @ DAE::Exp::CREF { componentRef: cref, .. }), tail: res }, ht) => {
                    let mut ht1: (metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>, (i32, i32, metamodelica::Array<Option<(metamodelica::Ref<DAE::ComponentRef>, metamodelica::Ref<DAE::ComponentRef>)>>), i32, (HashTableCG::FuncHashCref, HashTableCG::FuncCrefEqual, HashTableCG::FuncCrefStr, HashTableCG::FuncExpStr));
                    let mut ht2: (metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>, (i32, i32, metamodelica::Array<Option<(metamodelica::Ref<DAE::ComponentRef>, metamodelica::Ref<DAE::ComponentRef>)>>), i32, (HashTableCG::FuncHashCref, HashTableCG::FuncCrefEqual, HashTableCG::FuncCrefStr, HashTableCG::FuncExpStr));
                    let mut res1: metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, metamodelica::Ref<DAE::Exp>)>;
                    let mut res2: metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, metamodelica::Ref<DAE::Exp>)>;
                    let mut new: metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, metamodelica::Ref<DAE::Exp>)>;
                    let mut new1: metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, metamodelica::Ref<DAE::Exp>)>;
                    let mut varLst: metamodelica::List<metamodelica::Ref<DAE::Var>>;
                    let __pa0 = ::match_deref::match_deref! { match &(ComponentReference::crefLastType(metamodelica::AsArg::as_arg(&cref))?) {
                        Deref @ DAE::Type::T_COMPLEX { varLst: __pa0, .. } => __pa0.clone(),
                        _ => return Err("pattern mismatch"),
                    } };
                    varLst = metamodelica::Own::own(__pa0);
                    (res1, ht1) = extendCrefRecords(metamodelica::AsArg::as_arg(&res), ht.clone())?;
                    new = List::map2(varLst.clone(), &move |__a0: metamodelica::Ref<DAE::Var>, __a1: metamodelica::Ref<DAE::ComponentRef>, __a2: metamodelica::Ref<DAE::ComponentRef>| extendCrefRecords1(&__a0, &__a1, &__a2), c.clone(), cref.clone())?;
                    (new1, ht2) = extendCrefRecords(&new, ht1.clone())?;
                    res2 = listAppend(new1.clone(), res1.clone());
                    Ok((metamodelica::cons((c.clone(), e.clone()), res2.clone()), ht2.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: (c, e @ Deref @ DAE::Exp::CALL { expLst: expl, attr: Deref @ DAE::CallAttributes { ty: Deref @ DAE::Type::T_COMPLEX { complexClassType: ClassInf::State::RECORD { path: rpath }, varLst, .. }, .. }, .. }), tail: res }, ht) => {
                    if !((AbsynUtil::pathEqual(var_field!((**e).path, DAE::Exp::CALL), metamodelica::AsArg::as_arg(&rpath)))) { return Err("guard") }
                    let mut ht1: (metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>, (i32, i32, metamodelica::Array<Option<(metamodelica::Ref<DAE::ComponentRef>, metamodelica::Ref<DAE::ComponentRef>)>>), i32, (HashTableCG::FuncHashCref, HashTableCG::FuncCrefEqual, HashTableCG::FuncCrefStr, HashTableCG::FuncExpStr));
                    let mut ht2: (metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>, (i32, i32, metamodelica::Array<Option<(metamodelica::Ref<DAE::ComponentRef>, metamodelica::Ref<DAE::ComponentRef>)>>), i32, (HashTableCG::FuncHashCref, HashTableCG::FuncCrefEqual, HashTableCG::FuncCrefStr, HashTableCG::FuncExpStr));
                    let mut res1: metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, metamodelica::Ref<DAE::Exp>)>;
                    let mut res2: metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, metamodelica::Ref<DAE::Exp>)>;
                    let mut new: metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, metamodelica::Ref<DAE::Exp>)>;
                    let mut new1: metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, metamodelica::Ref<DAE::Exp>)>;
                    let mut crlst: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
                    (res1, ht1) = extendCrefRecords(metamodelica::AsArg::as_arg(&res), ht.clone())?;
                    crlst = List::map1(varLst.clone(), &move |__a0: metamodelica::Ref<DAE::Var>, __a1: metamodelica::Ref<DAE::ComponentRef>| extendCrefRecords2(&__a0, &__a1), c.clone())?;
                    new = List::zip(crlst.clone(), expl.clone());
                    (new1, ht2) = extendCrefRecords(&new, ht1.clone())?;
                    res2 = listAppend(new1.clone(), res1.clone());
                    Ok((metamodelica::cons((c.clone(), e.clone()), res2.clone()), ht2.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: (c, e @ Deref @ DAE::Exp::RECORD { exps: expl, ty: Deref @ DAE::Type::T_COMPLEX { varLst, .. }, .. }), tail: res }, ht) => {
                    let mut ht1: (metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>, (i32, i32, metamodelica::Array<Option<(metamodelica::Ref<DAE::ComponentRef>, metamodelica::Ref<DAE::ComponentRef>)>>), i32, (HashTableCG::FuncHashCref, HashTableCG::FuncCrefEqual, HashTableCG::FuncCrefStr, HashTableCG::FuncExpStr));
                    let mut ht2: (metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>, (i32, i32, metamodelica::Array<Option<(metamodelica::Ref<DAE::ComponentRef>, metamodelica::Ref<DAE::ComponentRef>)>>), i32, (HashTableCG::FuncHashCref, HashTableCG::FuncCrefEqual, HashTableCG::FuncCrefStr, HashTableCG::FuncExpStr));
                    let mut res1: metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, metamodelica::Ref<DAE::Exp>)>;
                    let mut res2: metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, metamodelica::Ref<DAE::Exp>)>;
                    let mut new: metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, metamodelica::Ref<DAE::Exp>)>;
                    let mut new1: metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, metamodelica::Ref<DAE::Exp>)>;
                    let mut crlst: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
                    (res1, ht1) = extendCrefRecords(metamodelica::AsArg::as_arg(&res), ht.clone())?;
                    crlst = List::map1(varLst.clone(), &move |__a0: metamodelica::Ref<DAE::Var>, __a1: metamodelica::Ref<DAE::ComponentRef>| extendCrefRecords2(&__a0, &__a1), c.clone())?;
                    new = List::zip(crlst.clone(), expl.clone());
                    (new1, ht2) = extendCrefRecords(&new, ht1.clone())?;
                    res2 = listAppend(new1.clone(), res1.clone());
                    Ok((metamodelica::cons((c.clone(), e.clone()), res2.clone()), ht2.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: (c, e), tail: res }, ht) => {
                    let mut ht1: (metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>, (i32, i32, metamodelica::Array<Option<(metamodelica::Ref<DAE::ComponentRef>, metamodelica::Ref<DAE::ComponentRef>)>>), i32, (HashTableCG::FuncHashCref, HashTableCG::FuncCrefEqual, HashTableCG::FuncCrefStr, HashTableCG::FuncExpStr));
                    let mut ht2: (metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>, (i32, i32, metamodelica::Array<Option<(metamodelica::Ref<DAE::ComponentRef>, metamodelica::Ref<DAE::ComponentRef>)>>), i32, (HashTableCG::FuncHashCref, HashTableCG::FuncCrefEqual, HashTableCG::FuncCrefStr, HashTableCG::FuncExpStr));
                    let mut ht3: (metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>, (i32, i32, metamodelica::Array<Option<(metamodelica::Ref<DAE::ComponentRef>, metamodelica::Ref<DAE::ComponentRef>)>>), i32, (HashTableCG::FuncHashCref, HashTableCG::FuncCrefEqual, HashTableCG::FuncCrefStr, HashTableCG::FuncExpStr));
                    let mut res1: metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, metamodelica::Ref<DAE::Exp>)>;
                    let mut varLst: metamodelica::List<metamodelica::Ref<DAE::Var>>;
                    let mut crlst: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
                    let mut creftpllst: metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, metamodelica::Ref<DAE::ComponentRef>)>;
                    let __pa0 = ::match_deref::match_deref! { match &(Expression::r#typeof(e.clone())?) {
                        Deref @ DAE::Type::T_COMPLEX { varLst: __pa0, .. } => __pa0.clone(),
                        _ => return Err("pattern mismatch"),
                    } };
                    varLst = metamodelica::Own::own(__pa0);
                    crlst = List::map1(varLst.clone(), &move |__a0: metamodelica::Ref<DAE::Var>, __a1: metamodelica::Ref<DAE::ComponentRef>| extendCrefRecords2(&__a0, &__a1), c.clone())?;
                    creftpllst = List::map1(crlst.clone(), &fnptr!(Util::makeTuple, _, _), c.clone())?;
                    ht1 = List::fold(&creftpllst, &BaseHashTable::add, ht.clone())?;
                    ht2 = getCheckCref(&crlst, ht1.clone())?;
                    (res1, ht3) = extendCrefRecords(metamodelica::AsArg::as_arg(&res), ht2.clone())?;
                    Ok((metamodelica::cons((c.clone(), e.clone()), res1.clone()), ht3.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: (c, e), tail: res }, ht) => {
                    let mut ht1: (metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>, (i32, i32, metamodelica::Array<Option<(metamodelica::Ref<DAE::ComponentRef>, metamodelica::Ref<DAE::ComponentRef>)>>), i32, (HashTableCG::FuncHashCref, HashTableCG::FuncCrefEqual, HashTableCG::FuncCrefStr, HashTableCG::FuncExpStr));
                    let mut res1: metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, metamodelica::Ref<DAE::Exp>)>;
                    (res1, ht1) = extendCrefRecords(metamodelica::AsArg::as_arg(&res), ht.clone())?;
                    Ok((metamodelica::cons((c.clone(), e.clone()), res1.clone()), ht1.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((outArgmap, outCheckCr))
}

fn getCheckCref(
    mut inCrefs: &metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
    mut inCheckCr: (
        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<
                Option<(
                    metamodelica::Ref<DAE::ComponentRef>,
                    metamodelica::Ref<DAE::ComponentRef>,
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
            Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>) -> Result<ArcStr> + 'static>,
        ),
    ),
) -> Result<(
    metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
    (
        i32,
        i32,
        metamodelica::Array<
            Option<(
                metamodelica::Ref<DAE::ComponentRef>,
                metamodelica::Ref<DAE::ComponentRef>,
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
        Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>) -> Result<ArcStr> + 'static>,
    ),
)> {
    let mut outCheckCr: (
        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<
                Option<(
                    metamodelica::Ref<DAE::ComponentRef>,
                    metamodelica::Ref<DAE::ComponentRef>,
                )>,
            >,
        ),
        i32,
        (
            HashTableCG::FuncHashCref,
            HashTableCG::FuncCrefEqual,
            HashTableCG::FuncCrefStr,
            HashTableCG::FuncExpStr,
        ),
    );
    outCheckCr = 'mc: {
        let __mc_input = (&**inCrefs, inCheckCr);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Nil, ht) => {
                    Ok(ht.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: cr, tail: rest }, ht) => {
                    let mut ht1: (metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>, (i32, i32, metamodelica::Array<Option<(metamodelica::Ref<DAE::ComponentRef>, metamodelica::Ref<DAE::ComponentRef>)>>), i32, (HashTableCG::FuncHashCref, HashTableCG::FuncCrefEqual, HashTableCG::FuncCrefStr, HashTableCG::FuncExpStr));
                    let mut ht2: (metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>, (i32, i32, metamodelica::Array<Option<(metamodelica::Ref<DAE::ComponentRef>, metamodelica::Ref<DAE::ComponentRef>)>>), i32, (HashTableCG::FuncHashCref, HashTableCG::FuncCrefEqual, HashTableCG::FuncCrefStr, HashTableCG::FuncExpStr));
                    let mut ht3: (metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>, (i32, i32, metamodelica::Array<Option<(metamodelica::Ref<DAE::ComponentRef>, metamodelica::Ref<DAE::ComponentRef>)>>), i32, (HashTableCG::FuncHashCref, HashTableCG::FuncCrefEqual, HashTableCG::FuncCrefStr, HashTableCG::FuncExpStr));
                    let mut crlst: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
                    let mut varLst: metamodelica::List<metamodelica::Ref<DAE::Var>>;
                    let mut creftpllst: metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, metamodelica::Ref<DAE::ComponentRef>)>;
                    let __pa0 = ::match_deref::match_deref! { match &(ComponentReference::crefLastType(metamodelica::AsArg::as_arg(&cr))?) {
                        Deref @ DAE::Type::T_COMPLEX { varLst: __pa0, .. } => __pa0.clone(),
                        _ => return Err("pattern mismatch"),
                    } };
                    varLst = metamodelica::Own::own(__pa0);
                    crlst = List::map1(varLst.clone(), &move |__a0: metamodelica::Ref<DAE::Var>, __a1: metamodelica::Ref<DAE::ComponentRef>| extendCrefRecords2(&__a0, &__a1), cr.clone())?;
                    ht1 = getCheckCref(&crlst, ht.clone())?;
                    creftpllst = List::map1(crlst.clone(), &fnptr!(Util::makeTuple, _, _), cr.clone())?;
                    ht2 = List::fold(&creftpllst, &BaseHashTable::add, ht1.clone())?;
                    ht3 = getCheckCref(metamodelica::AsArg::as_arg(&rest), ht2.clone())?;
                    Ok(ht3.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: _, tail: rest }, ht) => {
                    let mut ht1: (metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>, (i32, i32, metamodelica::Array<Option<(metamodelica::Ref<DAE::ComponentRef>, metamodelica::Ref<DAE::ComponentRef>)>>), i32, (HashTableCG::FuncHashCref, HashTableCG::FuncCrefEqual, HashTableCG::FuncCrefStr, HashTableCG::FuncExpStr));
                    ht1 = getCheckCref(metamodelica::AsArg::as_arg(&rest), ht.clone())?;
                    Ok(ht1.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(outCheckCr)
}

fn extendCrefRecords1(
    mut ev: &metamodelica::Ref<DAE::Var>,
    mut c: &metamodelica::Ref<DAE::ComponentRef>,
    mut e: &metamodelica::Ref<DAE::ComponentRef>,
) -> Result<(metamodelica::Ref<DAE::ComponentRef>, metamodelica::Ref<DAE::Exp>)> {
    let mut outArg: (metamodelica::Ref<DAE::ComponentRef>, metamodelica::Ref<DAE::Exp>);
    outArg = 'mc: {
        let __mc_input = &**ev;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Var { name, ty: tp, .. } => {
                    let mut c1: metamodelica::Ref<DAE::ComponentRef>;
                    let mut e1: metamodelica::Ref<DAE::ComponentRef>;
                    let mut exp: metamodelica::Ref<DAE::Exp>;
                    c1 = ComponentReference::crefPrependIdent(c, metamodelica::AsArg::as_arg(&name), &(metamodelica::nil()), metamodelica::AsArg::as_arg(&tp))?;
                    e1 = ComponentReference::crefPrependIdent(e, metamodelica::AsArg::as_arg(&name), &(metamodelica::nil()), metamodelica::AsArg::as_arg(&tp))?;
                    exp = Expression::makeCrefExp(e1.clone(), tp.clone())?;
                    Ok((c1.clone(), exp.clone()))
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
                    Debug::trace(literal!("Inline.extendCrefRecords1 failed\n"))?;
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(outArg)
}

fn extendCrefRecords2(
    mut ev: &metamodelica::Ref<DAE::Var>,
    mut c: &metamodelica::Ref<DAE::ComponentRef>,
) -> Result<metamodelica::Ref<DAE::ComponentRef>> {
    let mut outArg: metamodelica::Ref<DAE::ComponentRef>;
    outArg = 'mc: {
        let __mc_input = &**ev;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Var { name, ty: tp, .. } => {
                    let mut c1: metamodelica::Ref<DAE::ComponentRef>;
                    c1 = ComponentReference::crefPrependIdent(c, metamodelica::AsArg::as_arg(&name), &(metamodelica::nil()), metamodelica::AsArg::as_arg(&tp))?;
                    Ok(c1.clone())
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
                    Debug::trace(literal!("Inline.extendCrefRecords2 failed\n"))?;
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(outArg)
}

pub fn getFunctionBody(
    mut p: metamodelica::Ref<Absyn::Path>,
    mut fns: &Functiontuple,
) -> Result<(
    metamodelica::List<metamodelica::Ref<DAE::Element>>,
    Option<metamodelica::Ref<SCode::Comment>>,
)> {
    let mut outfn: metamodelica::List<metamodelica::Ref<DAE::Element>>;
    let mut oComment: Option<metamodelica::Ref<SCode::Comment>>;
    (outfn, oComment) = 'mc: {
        let __mc_input = fns;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Some(ftree), _) => {
                    let mut body: metamodelica::List<metamodelica::Ref<DAE::Element>>;
                    let mut comment: Option<metamodelica::Ref<SCode::Comment>>;
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(AvlTreePathFunction::get(metamodelica::AsArg::as_arg(&ftree), p.clone())?) {
                        Some(DAE::Function::FUNCTION { functions: Deref @ metamodelica::ListNode::Cons { head: DAE::FunctionDefinition::FUNCTION_DEF { body: __pa0 }, tail: _ }, comment: __pa1, .. }) => (__pa0.clone(), __pa1.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    body = metamodelica::Own::own(__pa0);
                    comment = metamodelica::Own::own(__pa1);
                    Ok((body.clone(), comment.clone()))
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
                    Debug::traceln({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Inline.getFunctionBody failed for function: ")); __mm_s.push_str(&*AbsynUtil::pathString(p.clone(), literal!("."), true, false)?); ArcStr::from(__mm_s) })?;
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((outfn, oComment))
}

pub fn getFunction(mut p: metamodelica::Ref<Absyn::Path>, mut fns: &Functiontuple) -> Result<DAE::Function> {
    let mut func: DAE::Function = <DAE::Function as ::std::default::Default>::default();
    func = 'mc: {
        let __mc_input = fns;
        if let Ok((__v, __wb0)) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Some(ftree), _) => {
                    let mut func: DAE::Function = func.clone();
                    let __pa0 = ::match_deref::match_deref! { match &(AvlTreePathFunction::get(metamodelica::AsArg::as_arg(&ftree), p.clone())?) {
                        Some(__pa0) => __pa0.clone(),
                        _ => return Err("pattern mismatch"),
                    } };
                    func = metamodelica::Own::own(__pa0);
                    Ok((func.clone(), func.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            func = __wb0;
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { return Err("pattern mismatch") };
                    Debug::traceln({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("Inline.getFunction failed for function: ")); __mm_s.push_str(&*AbsynUtil::pathString(p.clone(), literal!("."), true, false)?); ArcStr::from(__mm_s) })?;
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(func)
}

fn getRhsExp<'__b>(
    mut inElementList: &'__b metamodelica::List<metamodelica::Ref<DAE::Element>>,
) -> Result<metamodelica::Ref<DAE::Exp>> {
    '__tco: loop {
        ::match_deref::match_deref! { match inElementList {
            Deref @ metamodelica::ListNode::Nil => {
                let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { return Err("pattern mismatch") };
                Debug::trace(literal!("Inline.getRhsExp failed - cannot inline such a function\n"))?;
                return Ok(return Err("fail"))
            },
            Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Element::ALGORITHM { algorithm_: Deref @ DAE::Algorithm { statementLst: Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Statement::STMT_ASSIGN { exp: res, .. }, tail: Deref @ metamodelica::ListNode::Nil } }, .. }, tail: _ } => {
                return Ok(res.clone())
            },
            Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Element::ALGORITHM { algorithm_: Deref @ DAE::Algorithm { statementLst: Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Statement::STMT_TUPLE_ASSIGN { exp: res, .. }, tail: Deref @ metamodelica::ListNode::Nil } }, .. }, tail: _ } => {
                return Ok(res.clone())
            },
            Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Element::ALGORITHM { algorithm_: Deref @ DAE::Algorithm { statementLst: Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Statement::STMT_ASSIGN_ARR { exp: res, .. }, tail: Deref @ metamodelica::ListNode::Nil } }, .. }, tail: _ } => {
                return Ok(res.clone())
            },
            Deref @ metamodelica::ListNode::Cons { head: _, tail: cdr } => {
                let mut res: metamodelica::Ref<DAE::Exp>;
                { inElementList = cdr; continue '__tco; }
            },
            _ => return Err("match: no arm matched"),
        } }
    }
}

pub fn replaceArgs(
    mut inExp: metamodelica::Ref<DAE::Exp>,
    mut inTuple: (
        metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, metamodelica::Ref<DAE::Exp>)>,
        (
            metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
            (
                i32,
                i32,
                metamodelica::Array<
                    Option<(
                        metamodelica::Ref<DAE::ComponentRef>,
                        metamodelica::Ref<DAE::ComponentRef>,
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
                Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>) -> Result<ArcStr> + 'static>,
            ),
        ),
        bool,
    ),
) -> (
    metamodelica::Ref<DAE::Exp>,
    (
        metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, metamodelica::Ref<DAE::Exp>)>,
        (
            metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
            (
                i32,
                i32,
                metamodelica::Array<
                    Option<(
                        metamodelica::Ref<DAE::ComponentRef>,
                        metamodelica::Ref<DAE::ComponentRef>,
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
                Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>) -> Result<ArcStr> + 'static>,
            ),
        ),
        bool,
    ),
) {
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    let mut outTuple: (
        metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, metamodelica::Ref<DAE::Exp>)>,
        (
            metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
            (
                i32,
                i32,
                metamodelica::Array<
                    Option<(
                        metamodelica::Ref<DAE::ComponentRef>,
                        metamodelica::Ref<DAE::ComponentRef>,
                    )>,
                >,
            ),
            i32,
            (
                HashTableCG::FuncHashCref,
                HashTableCG::FuncCrefEqual,
                HashTableCG::FuncCrefStr,
                HashTableCG::FuncExpStr,
            ),
        ),
        bool,
    );
    (outExp, outTuple) = 'mc: {
        let __mc_input = (inExp.clone(), &inTuple);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CREF { componentRef: cref, .. }, (argmap, _, true)) => {
                    let mut e: metamodelica::Ref<DAE::Exp>;
                    e = getExpFromArgMap(metamodelica::AsArg::as_arg(&argmap), metamodelica::AsArg::as_arg(&cref))?;
                    (e, _) = ExpressionSimplify::simplify(e.clone())?;
                    Ok((e.clone(), inTuple.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CREF { componentRef: cref, .. }, (argmap, checkcr, true)) => {
                    if !((BaseHashTable::hasKey(ComponentReferenceBasics::crefFirstCref(cref.clone())?, &(checkcr.clone()))?)) { return Err("guard") }
                    Ok((inExp.clone(), (argmap.clone(), checkcr.clone(), false)))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CREF { componentRef: cref, .. }, (argmap, _, true)) => {
                    let mut firstCref: metamodelica::Ref<DAE::ComponentRef>;
                    let mut e: metamodelica::Ref<DAE::Exp>;
                    let mut cref = (*cref).clone();
                    firstCref = ComponentReferenceBasics::crefFirstCref(cref.clone())?;
                    ::match_deref::match_deref! { match &(ComponentReferenceBasics::crefSubs(&firstCref)?) {
                        Deref @ metamodelica::ListNode::Nil => (),
                        _ => return Err("pattern mismatch"),
                    } };
                    e = getExpFromArgMap(metamodelica::AsArg::as_arg(&argmap), &firstCref)?;
                    while !(ComponentReference::crefIsIdent(metamodelica::AsArg::as_arg(&cref))) {
                        cref = ComponentReference::crefRest(metamodelica::AsArg::as_arg(&cref))?;
                        ::match_deref::match_deref! { match &(ComponentReferenceBasics::crefSubs(metamodelica::AsArg::as_arg(&cref))?) {
                            Deref @ metamodelica::ListNode::Nil => (),
                            _ => return Err("pattern mismatch"),
                        } };
                        e = metamodelica::Ref::new(DAE::Exp::RSUB { exp: e.clone(), ix: -1, fieldName: ComponentReferenceBasics::crefFirstIdent(metamodelica::AsArg::as_arg(&cref))?, ty: ComponentReference::crefType(metamodelica::AsArg::as_arg(&cref))? });
                    }
                    Ok((e.clone(), inTuple.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CREF { componentRef: cref, .. }, (argmap, checkcr, true)) => {
                    getExpFromArgMap(metamodelica::AsArg::as_arg(&argmap), &(ComponentReference::crefStripSubs(&(ComponentReferenceBasics::crefFirstCref(cref.clone())?))?))?;
                    Ok((inExp.clone(), (argmap.clone(), checkcr.clone(), false)))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::UNBOX { exp: Deref @ DAE::Exp::CALL { path, expLst, attr: Deref @ DAE::CallAttributes { ty: _, tuple_, builtin: false, isImpure, isFunctionPointerCall: _, inlineType, tailCall: tc, noReturn: _ } }, ty }, (argmap, _, true)) => {
                    let mut cref: metamodelica::Ref<DAE::ComponentRef>;
                    let mut e: metamodelica::Ref<DAE::Exp>;
                    let mut b: bool;
                    let mut isFunctionPointerCall: bool;
                    let mut ty2: metamodelica::Ref<DAE::Type>;
                    let mut path = (*path).clone();
                    let mut expLst = (*expLst).clone();
                    cref = ComponentReference::pathToCref(metamodelica::AsArg::as_arg(&path));
                    let (__pa2, __pa0, __pa1) = ::match_deref::match_deref! { match &(getExpFromArgMap(metamodelica::AsArg::as_arg(&argmap), &cref)?) {
                        __pa2 @ Deref @ DAE::Exp::CREF { componentRef: __pa0, ty: __pa1 } => (__pa2.clone(), __pa0.clone(), __pa1.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    cref = metamodelica::Own::own(__pa0);
                    ty2 = metamodelica::Own::own(__pa1);
                    e = metamodelica::Own::own(__pa2);
                    path = ComponentReference::crefToPath(&cref)?;
                    expLst = List::map(expLst.clone(), &move |__a0: metamodelica::Ref<DAE::Exp>| -> metamodelica::Result<_> { ::std::result::Result::Ok(Expression::unboxExp(&__a0)) })?;
                    b = Expression::isBuiltinFunctionReference(&e);
                    isFunctionPointerCall = Types::isFunctionReferenceVar(&ty2);
                    e = metamodelica::Ref::new(DAE::Exp::CALL { path: path.clone(), expLst: expLst.clone(), attr: metamodelica::Ref::new(DAE::CallAttributes { ty: ty.clone(), tuple_: tuple_.clone(), builtin: b, isImpure: isImpure.clone(), isFunctionPointerCall: isFunctionPointerCall, inlineType: inlineType.clone(), tailCall: tc.clone(), noReturn: DAE::NoReturn::RETURNS.clone() }) });
                    (e, _) = ExpressionSimplify::simplify(e.clone())?;
                    Ok((e.clone(), inTuple.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e @ Deref @ DAE::Exp::UNBOX { exp: Deref @ DAE::Exp::CALL { path, expLst: _, attr: Deref @ DAE::CallAttributes { builtin: false, .. } }, ty: _ }, (argmap, checkcr, true)) => {
                    let mut cref: metamodelica::Ref<DAE::ComponentRef>;
                    cref = ComponentReference::pathToCref(metamodelica::AsArg::as_arg(&path));
                    let true = (BaseHashTable::hasKey(cref.clone(), &(checkcr.clone()))?) else { return Err("pattern mismatch") };
                    Ok((e.clone(), (argmap.clone(), checkcr.clone(), false)))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Exp::CALL { path, expLst, attr: Deref @ DAE::CallAttributes { ty: Deref @ DAE::Type::T_METATYPE { .. }, tuple_, builtin: false, isImpure, isFunctionPointerCall: _, inlineType: _, tailCall: tc, noReturn: _ } }, (argmap, _, true)) => {
                    let mut cref: metamodelica::Ref<DAE::ComponentRef>;
                    let mut e: metamodelica::Ref<DAE::Exp>;
                    let mut b: bool;
                    let mut isFunctionPointerCall: bool;
                    let mut ty: metamodelica::Ref<DAE::Type>;
                    let mut ty2: metamodelica::Ref<DAE::Type>;
                    let mut inlineType: DAE::InlineType;
                    let mut path = (*path).clone();
                    let mut expLst = (*expLst).clone();
                    cref = ComponentReference::pathToCref(metamodelica::AsArg::as_arg(&path));
                    let (__pa2, __pa0, __pa1) = ::match_deref::match_deref! { match &(getExpFromArgMap(metamodelica::AsArg::as_arg(&argmap), &cref)?) {
                        __pa2 @ Deref @ DAE::Exp::CREF { componentRef: __pa0, ty: __pa1 } => (__pa2.clone(), __pa0.clone(), __pa1.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    cref = metamodelica::Own::own(__pa0);
                    ty = metamodelica::Own::own(__pa1);
                    e = metamodelica::Own::own(__pa2);
                    path = ComponentReference::crefToPath(&cref)?;
                    expLst = List::map(expLst.clone(), &move |__a0: metamodelica::Ref<DAE::Exp>| -> metamodelica::Result<_> { ::std::result::Result::Ok(Expression::unboxExp(&__a0)) })?;
                    b = Expression::isBuiltinFunctionReference(&e);
                    (ty2, inlineType) = functionReferenceType(ty.clone())?;
                    isFunctionPointerCall = Types::isFunctionReferenceVar(&ty2);
                    e = metamodelica::Ref::new(DAE::Exp::CALL { path: path.clone(), expLst: expLst.clone(), attr: metamodelica::Ref::new(DAE::CallAttributes { ty: ty2.clone(), tuple_: tuple_.clone(), builtin: b, isImpure: isImpure.clone(), isFunctionPointerCall: isFunctionPointerCall, inlineType: inlineType, tailCall: tc.clone(), noReturn: DAE::NoReturn::RETURNS.clone() }) });
                    e = boxIfUnboxedFunRef(e.clone(), &ty);
                    (e, _) = ExpressionSimplify::simplify(e.clone())?;
                    Ok((e.clone(), inTuple.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (e @ Deref @ DAE::Exp::CALL { path, expLst: _, attr: Deref @ DAE::CallAttributes { ty: Deref @ DAE::Type::T_METATYPE { .. }, builtin: false, .. } }, (argmap, checkcr, true)) => {
                    let mut cref: metamodelica::Ref<DAE::ComponentRef>;
                    cref = ComponentReference::pathToCref(metamodelica::AsArg::as_arg(&path));
                    let true = (BaseHashTable::hasKey(cref.clone(), &(checkcr.clone()))?) else { return Err("pattern mismatch") };
                    Ok((e.clone(), (argmap.clone(), checkcr.clone(), false)))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((inExp.clone(), inTuple.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    (outExp, outTuple)
}

fn boxIfUnboxedFunRef(
    mut iexp: metamodelica::Ref<DAE::Exp>,
    mut ty: &metamodelica::Ref<DAE::Type>,
) -> metamodelica::Ref<DAE::Exp> {
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    outExp = (::match_deref::match_deref! { match ty {
        Deref @ DAE::Type::T_FUNCTION_REFERENCE_FUNC { functionType: Deref @ DAE::Type::T_FUNCTION { funcResultType: t, .. }, .. } => {
            let mut exp = iexp.clone();
            exp = if (Types::isBoxedType(metamodelica::AsArg::as_arg(&t))) {exp} else {metamodelica::Ref::new(DAE::Exp::BOX { exp: exp })};
            exp
        },
        _ => {
            iexp
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outExp
}

fn functionReferenceType(
    mut ty1: metamodelica::Ref<DAE::Type>,
) -> Result<(metamodelica::Ref<DAE::Type>, DAE::InlineType)> {
    let mut ty2: metamodelica::Ref<DAE::Type>;
    let mut inlineType: DAE::InlineType;
    (ty2, inlineType) = (::match_deref::match_deref! { match &(ty1.clone()) {
        Deref @ DAE::Type::T_FUNCTION_REFERENCE_FUNC { functionType: Deref @ DAE::Type::T_FUNCTION { functionAttributes: DAE::FunctionAttributes { inline: __esc_inlineType, .. }, funcResultType: ty, .. }, .. } => {
            inlineType = (*__esc_inlineType).clone();
            (Types::simplifyType(ty.clone())?, inlineType.clone())
        },
        _ => {
            (ty1, openmodelica_frontend_types::DAE::InlineType::NO_INLINE)
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((ty2, inlineType))
}

fn getExpFromArgMap(
    mut inArgMap: &metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, metamodelica::Ref<DAE::Exp>)>,
    mut inComponentRef: &metamodelica::Ref<DAE::ComponentRef>,
) -> Result<metamodelica::Ref<DAE::Exp>> {
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    let mut arg: (metamodelica::Ref<DAE::ComponentRef>, metamodelica::Ref<DAE::Exp>) = (
        metamodelica::Ref::new(DAE::ComponentRef::WILD),
        <metamodelica::Ref<DAE::Exp> as ::std::default::Default>::default(),
    );
    let mut subs: metamodelica::List<metamodelica::Ref<DAE::Subscript>>;
    let mut key: metamodelica::Ref<DAE::ComponentRef>;
    let mut cref: metamodelica::Ref<DAE::ComponentRef>;
    let mut exp: metamodelica::Ref<DAE::Exp>;
    subs = ComponentReferenceBasics::crefSubs(inComponentRef)?;
    key = ComponentReference::crefStripSubs(inComponentRef)?;
    '__loop0: for mut arg in &**inArgMap {
        let mut arg = arg.clone();
        (cref, exp) = arg;
        if ComponentReferenceBasics::crefEqual(&cref, &key)? {
            if let Ok(__iflet1) = Expression::applyExpSubscripts(exp.clone(), subs.clone()) {
                outExp = __iflet1;
            } else {
                continue '__loop0;
            }
            return Ok(outExp);
        }
    }
    if Flags::isSet(Flags::FAILTRACE.clone())? {
        Debug::traceln({
            let mut __mm_s = String::new();
            __mm_s.push_str(&*literal!(
                "Inline.getExpFromArgMap failed with empty argmap and cref: "
            ));
            __mm_s.push_str(&*ComponentReferenceBasics::printComponentRefStr(inComponentRef)?);
            ArcStr::from(__mm_s)
        })?;
    }
    return Err("fail");
    Ok(outExp)
}

fn getInputCrefs(mut inElement: &metamodelica::Ref<DAE::Element>) -> metamodelica::Ref<DAE::ComponentRef> {
    let mut outComponentRef: metamodelica::Ref<DAE::ComponentRef>;
    outComponentRef = (match &**inElement {
        DAE::Element::VAR {
            componentRef: cref,
            direction: DAE::VarDirection::INPUT { .. },
            ..
        } => cref.clone(),
        _ => openmodelica_frontend_types::DAE::ComponentRef::interned_WILD(),
    });
    outComponentRef
}

fn removeWilds(mut inComponentRef: &metamodelica::Ref<DAE::ComponentRef>) -> bool {
    let mut outBoolean: bool;
    outBoolean = (match &**inComponentRef {
        DAE::ComponentRef::WILD { .. } => false,
        _ => true,
    });
    outBoolean
}

pub(crate) fn printInlineTypeStr(mut it: DAE::InlineType) -> ArcStr {
    let mut r#str: ArcStr;
    r#str = (match it {
        DAE::InlineType::NO_INLINE { .. } => literal!("No inline"),
        DAE::InlineType::AFTER_INDEX_RED_INLINE { .. } => literal!("Inline after index reduction"),
        DAE::InlineType::EARLY_INLINE { .. } => literal!("Inline as soon as possible"),
        DAE::InlineType::BUILTIN_EARLY_INLINE { .. } => {
            literal!("Inline as soon as possible, even if inlining is globally disabled")
        }
        DAE::InlineType::NORM_INLINE { .. } => literal!("Inline before index reduction"),
        DAE::InlineType::DEFAULT_INLINE { .. } => literal!("Inline if necessary"),
    });
    r#str
}

pub(crate) fn simplifyAndInlineEquationExp(
    mut inExp: metamodelica::Ref<DAE::EquationExp>,
    mut fns: &Functiontuple,
    mut inSource: metamodelica::Ref<DAE::ElementSource>,
) -> Result<(
    metamodelica::Ref<DAE::EquationExp>,
    metamodelica::Ref<DAE::ElementSource>,
)> {
    let mut exp: metamodelica::Ref<DAE::EquationExp>;
    let mut source: metamodelica::Ref<DAE::ElementSource>;
    (exp, source) = ExpressionSimplify::simplifyAddSymbolicOperation(inExp, inSource)?;
    (exp, source) = inlineEquationExp(
        exp,
        &({
            let __pe_b2 = fns.clone();
            move |__pe_a0, __pe_a1| Ok(inlineCall(__pe_a0, __pe_a1, &__pe_b2))
        }),
        source,
    )?;
    Ok((exp, source))
}

pub fn simplifyAndForceInlineEquationExp(
    mut inExp: metamodelica::Ref<DAE::EquationExp>,
    mut fns: &Functiontuple,
    mut inSource: metamodelica::Ref<DAE::ElementSource>,
) -> Result<(
    metamodelica::Ref<DAE::EquationExp>,
    metamodelica::Ref<DAE::ElementSource>,
)> {
    let mut exp: metamodelica::Ref<DAE::EquationExp>;
    let mut source: metamodelica::Ref<DAE::ElementSource>;
    (exp, source) = ExpressionSimplify::simplifyAddSymbolicOperation(inExp, inSource)?;
    (exp, source) = inlineEquationExp(
        exp,
        &({
            let __pe_b2 = fns.clone();
            let __pe_b3 = openmodelica_ast_collections::AvlSetPath::Tree::interned_EMPTY();
            move |__pe_a0, __pe_a1| Ok(forceInlineCall(__pe_a0, __pe_a1, &__pe_b2, __pe_b3.clone()))
        }),
        source,
    )?;
    Ok((exp, source))
}

pub(crate) fn inlineEquationExp(
    mut inExp: metamodelica::Ref<DAE::EquationExp>,
    mut r#fn: &dyn ::std::ops::Fn(
        metamodelica::Ref<DAE::Exp>,
        metamodelica::List<metamodelica::Ref<DAE::Statement>>,
    ) -> Result<(
        metamodelica::Ref<DAE::Exp>,
        metamodelica::List<metamodelica::Ref<DAE::Statement>>,
    )>,
    mut inSource: metamodelica::Ref<DAE::ElementSource>,
) -> Result<(
    metamodelica::Ref<DAE::EquationExp>,
    metamodelica::Ref<DAE::ElementSource>,
)> {
    pub type Func = std::sync::Arc<
        dyn ::std::ops::Fn(
                metamodelica::Ref<DAE::Exp>,
                metamodelica::List<metamodelica::Ref<DAE::Statement>>,
            ) -> Result<(
                metamodelica::Ref<DAE::Exp>,
                metamodelica::List<metamodelica::Ref<DAE::Statement>>,
            )> + 'static,
    >;

    pub type Functiontuple = (
        Option<metamodelica::Ref<AvlTreePathFunction::Tree>>,
        metamodelica::List<DAE::InlineType>,
    );

    let mut outExp: metamodelica::Ref<DAE::EquationExp>;
    let mut source: metamodelica::Ref<DAE::ElementSource>;
    (outExp, source) = (match &*inExp {
        DAE::EquationExp::PARTIAL_EQUATION { exp: e } => {
            let mut changed: bool;
            let mut e_1: metamodelica::Ref<DAE::Exp>;
            let mut eq2: metamodelica::Ref<DAE::EquationExp>;
            (e_1, _) = Expression::traverseExpBottomUp(e.clone(), r#fn, metamodelica::nil())?;
            changed = !(referenceEq(&*(e.clone()), &*(&*e_1)));
            eq2 = metamodelica::Ref::new(DAE::EquationExp::PARTIAL_EQUATION { exp: e_1 });
            source = ElementSource::condAddSymbolicTransformation(
                changed,
                inSource,
                metamodelica::Ref::new(DAE::SymbolicOperation::OP_INLINE {
                    before: inExp,
                    after: eq2.clone(),
                }),
            )?;
            (eq2, source) = ExpressionSimplify::condSimplifyAddSymbolicOperation(changed, eq2, source)?;
            (eq2, source)
        }
        DAE::EquationExp::RESIDUAL_EXP { exp: e } => {
            let mut changed: bool;
            let mut e_1: metamodelica::Ref<DAE::Exp>;
            let mut eq2: metamodelica::Ref<DAE::EquationExp>;
            (e_1, _) = Expression::traverseExpBottomUp(e.clone(), r#fn, metamodelica::nil())?;
            changed = !(referenceEq(&*(e.clone()), &*(&*e_1)));
            eq2 = metamodelica::Ref::new(DAE::EquationExp::RESIDUAL_EXP { exp: e_1 });
            source = ElementSource::condAddSymbolicTransformation(
                changed,
                inSource,
                metamodelica::Ref::new(DAE::SymbolicOperation::OP_INLINE {
                    before: inExp,
                    after: eq2.clone(),
                }),
            )?;
            (eq2, source) = ExpressionSimplify::condSimplifyAddSymbolicOperation(changed, eq2, source)?;
            (eq2, source)
        }
        DAE::EquationExp::EQUALITY_EXPS { lhs: e1, rhs: e2 } => {
            let mut changed: bool;
            let mut e1_1: metamodelica::Ref<DAE::Exp>;
            let mut e2_1: metamodelica::Ref<DAE::Exp>;
            let mut eq2: metamodelica::Ref<DAE::EquationExp>;
            (e1_1, _) = Expression::traverseExpBottomUp(e1.clone(), r#fn, metamodelica::nil())?;
            (e2_1, _) = Expression::traverseExpBottomUp(e2.clone(), r#fn, metamodelica::nil())?;
            changed = !(referenceEq(&*(e1.clone()), &*(&*e1_1)) && referenceEq(&*(e2.clone()), &*(&*e2_1)));
            eq2 = metamodelica::Ref::new(DAE::EquationExp::EQUALITY_EXPS { lhs: e1_1, rhs: e2_1 });
            source = ElementSource::condAddSymbolicTransformation(
                changed,
                inSource,
                metamodelica::Ref::new(DAE::SymbolicOperation::OP_INLINE {
                    before: inExp,
                    after: eq2.clone(),
                }),
            )?;
            (eq2, source) = ExpressionSimplify::condSimplifyAddSymbolicOperation(changed, eq2, source)?;
            (eq2, source)
        }
        _ => {
            Error::addMessage(
                Error::INTERNAL_ERROR.clone(),
                list![literal!("Inline.inlineEquationExp failed")],
            )?;
            return Err("fail");
        }
    });
    Ok((outExp, source))
}

fn getReplacementCheckComplex(
    mut repl: VarTransform::VariableReplacements,
    mut cr: metamodelica::Ref<DAE::ComponentRef>,
    mut ty: metamodelica::Ref<DAE::Type>,
) -> Result<metamodelica::Ref<DAE::Exp>> {
    let mut exp: metamodelica::Ref<DAE::Exp>;
    exp = 'mc: {
        let __mc_input = &*ty;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(VarTransform::getReplacement(&repl, cr.clone())?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Type::T_COMPLEX { complexClassType: ClassInf::State::RECORD { path }, varLst: vars, .. } => {
                    let mut crs: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
                    let mut exps: metamodelica::List<metamodelica::Ref<DAE::Exp>>;
                    crs = List::map1(List::map(vars.clone(), &move |__a0: metamodelica::Ref<DAE::Var>| -> metamodelica::Result<_> { ::std::result::Result::Ok(TypesDump::getVarName(&__a0)) })?, &move |__a0: ArcStr, __a1: metamodelica::Ref<DAE::ComponentRef>| ComponentReference::appendStringCref(__a0, &__a1), cr.clone())?;
                    exps = List::map1r(crs.clone(), &move |__a0: VarTransform::VariableReplacements, __a1: metamodelica::Ref<DAE::ComponentRef>| VarTransform::getReplacement(&__a0, __a1), repl.clone())?;
                    Ok(metamodelica::Ref::new(DAE::Exp::CALL { path: path.clone(), expLst: exps.clone(), attr: metamodelica::Ref::new(DAE::CallAttributes { ty: ty.clone(), tuple_: false, builtin: false, isImpure: false, isFunctionPointerCall: false, inlineType: openmodelica_frontend_types::DAE::InlineType::NO_INLINE, tailCall: openmodelica_frontend_types::DAE::TailCall::NO_TAIL, noReturn: DAE::NoReturn::RETURNS.clone() }) }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(exp)
}

fn getInlineHashTableVarTransform() -> (
    (
        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<
                Option<(
                    metamodelica::Ref<DAE::ComponentRef>,
                    metamodelica::Ref<DAE::ComponentRef>,
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
            Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::ComponentRef>) -> Result<ArcStr> + 'static>,
        ),
    ),
    VarTransform::VariableReplacements,
) {
    let mut ht: (
        metamodelica::Array<metamodelica::List<(metamodelica::Ref<DAE::ComponentRef>, i32)>>,
        (
            i32,
            i32,
            metamodelica::Array<
                Option<(
                    metamodelica::Ref<DAE::ComponentRef>,
                    metamodelica::Ref<DAE::ComponentRef>,
                )>,
            >,
        ),
        i32,
        (
            HashTableCG::FuncHashCref,
            HashTableCG::FuncCrefEqual,
            HashTableCG::FuncCrefStr,
            HashTableCG::FuncExpStr,
        ),
    ) = HashTableCG::emptyHashTableSized(BaseHashTable::lowBucketSize.clone());
    let mut repl: VarTransform::VariableReplacements =
        VarTransform::emptyReplacementsSized(BaseHashTable::lowBucketSize.clone());
    (ht, repl)
}

// Fresh tables: in this port that is cheaper than clearing the cached ones.
// They hold one function's variables, so the smallest size will do.
