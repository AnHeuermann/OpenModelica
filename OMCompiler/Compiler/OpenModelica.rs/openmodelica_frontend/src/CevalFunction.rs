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

use crate::Ceval;
use crate::FGraph;
use crate::FNode;
use crate::Lookup;
use openmodelica_ast::Absyn;
use openmodelica_frontend_base::ComponentReference;
use openmodelica_frontend_base::DAEDump;
use openmodelica_frontend_base::DAEUtil;
use openmodelica_frontend_base::Expression;
use openmodelica_frontend_base::Types;
use openmodelica_frontend_base::ValuesUtil;
use openmodelica_frontend_dump::AbsynUtil;
use openmodelica_frontend_dump::ComponentReferenceBasics;
use openmodelica_frontend_dump::ElementSource;
use openmodelica_frontend_dump::ExpressionBasics;
use openmodelica_frontend_dump::FCore;
use openmodelica_frontend_dump::TypesDump;
use openmodelica_frontend_dump::ValuesMake;
use openmodelica_frontend_types::ClassInf;
use openmodelica_frontend_types::DAE;
use openmodelica_frontend_types::SCode;
use openmodelica_frontend_types::Values;
use openmodelica_util::Debug;
use openmodelica_util::Error;
use openmodelica_util::Flags;
use openmodelica_util::Graph;
use openmodelica_util::Lapack;
use openmodelica_util::Util;
use openmodelica_util_datatypes_basic::List;
use openmodelica_util_datatypes_basic::Mutable;

// Jump table for CevalFunction:
// [TYPE]  Types.
// [EVAL]  Constant evaluation functions.
// [EENV]  Environment extension functions (add variables).
// [MENV]  Environment manipulation functions (set and get variables).
// [DEPS]  Function variable dependency handling.
// [EOPT]  Expression optimization functions.
// public imports
// protected imports
// [TYPE]  Types
pub type FunctionVar = (
    metamodelica::Ref<DAE::Element>,
    Option<metamodelica::Ref<Values::Value>>,
);

// LoopControl is used to control the functions behaviour in different
// situations. All evaluation functions returns a LoopControl variable that
// tells the caller whether it should continue evaluating or not.
#[derive(Clone, Copy, Debug, Eq, Hash, metamodelica::MMCtor, metamodelica::MetaCmp, metamodelica::ReferenceEq)]
pub(crate) enum LoopControl {
    /// Continue to the next statement.
    NEXT,
    /// Exit the current loop.
    BREAK,
    /// Exit the function.
    RETURN,
}
impl metamodelica::gc::MMTrace for LoopControl {
    fn mm_accept(&self, __mmv: &mut dyn metamodelica::gc::MMVisitor) -> Result<(), ()> {
        match self {
            LoopControl::NEXT => Ok(()),
            LoopControl::BREAK => Ok(()),
            LoopControl::RETURN => Ok(()),
        }
    }
}
impl Default for LoopControl {
    fn default() -> Self {
        Self::NEXT
    }
}
pub(crate) use self::LoopControl::{BREAK, NEXT, RETURN};

// [EVAL]  Constant evaluation functions.
pub fn evaluate(
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
    mut inFunction: &DAE::Function,
    mut inFunctionArguments: &metamodelica::List<metamodelica::Ref<Values::Value>>,
) -> Result<(FCore::Cache, metamodelica::Ref<Values::Value>)> {
    let mut outCache: FCore::Cache;
    let mut outResult: metamodelica::Ref<Values::Value>;
    (outCache, outResult) = 'mc: {
        let __mc_input = inFunction;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                DAE::Function::FUNCTION { path: p, functions: Deref @ metamodelica::ListNode::Cons { head: func, tail: _ }, type_: ty, partialPrefix: false, source: src, .. } => {
                    let mut result: metamodelica::Ref<Values::Value>;
                    let mut func_name: ArcStr;
                    let mut cache: FCore::Cache;
                    func_name = AbsynUtil::pathString(p.clone(), literal!("."), true, false)?;
                    (cache, result) = evaluateFunctionDefinition(inCache.clone(), inEnv.clone(), func_name.clone(), metamodelica::AsArg::as_arg(&func), metamodelica::AsArg::as_arg(&ty), inFunctionArguments, src.clone())?;
                    Ok((cache.clone(), result.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                DAE::Function::FUNCTION { path: p, functions: Deref @ metamodelica::ListNode::Cons { head: _, tail: _ }, partialPrefix, .. } => {
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { return Err("pattern mismatch") };
                    Debug::traceln({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("- CevalFunction.evaluate failed for function: ")); __mm_s.push_str(&*if (partialPrefix.clone()) {literal!("partial ")} else {literal!("")}); __mm_s.push_str(&*AbsynUtil::pathString(p.clone(), literal!("."), true, false)?); ArcStr::from(__mm_s) })?;
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((outCache, outResult))
}

fn evaluateFunctionDefinition(
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
    mut inFuncName: ArcStr,
    mut inFunc: &DAE::FunctionDefinition,
    mut inFuncType: &metamodelica::Ref<DAE::Type>,
    mut inFuncArgs: &metamodelica::List<metamodelica::Ref<Values::Value>>,
    mut inSource: metamodelica::Ref<DAE::ElementSource>,
) -> Result<(FCore::Cache, metamodelica::Ref<Values::Value>)> {
    let mut outCache: FCore::Cache;
    let mut outResult: metamodelica::Ref<Values::Value>;
    (outCache, outResult) = 'mc: {
        let __mc_input = inFunc.clone();
        if let Ok(__v) = (|| -> Result<_> {
            let DAE::FunctionDefinition::FUNCTION_DEF { body: mut body } = __mc_input.clone() else {
                return Err("nomatch");
            };
            let mut vars: metamodelica::List<metamodelica::Ref<DAE::Element>>;
            let mut output_vars: metamodelica::List<metamodelica::Ref<DAE::Element>>;
            let mut func_params: metamodelica::List<(
                metamodelica::Ref<DAE::Element>,
                Option<metamodelica::Ref<Values::Value>>,
            )>;
            let mut cache: FCore::Cache;
            let mut env: FCore::Graph;
            let mut return_values: metamodelica::List<metamodelica::Ref<Values::Value>>;
            let mut return_value: metamodelica::Ref<Values::Value>;
            let mut body = body.clone();
            (vars, body) = List::splitOnFirstMatch(
                body.clone(),
                &move |__a0: metamodelica::Ref<DAE::Element>| -> metamodelica::Result<_> {
                    ::std::result::Result::Ok(DAEUtil::isNotVar(&__a0))
                },
            )?;
            vars = List::map(vars.clone(), &move |__a0: metamodelica::Ref<DAE::Element>| {
                removeSelfReferentialDims(&__a0)
            })?;
            output_vars = List::filterOnTrue(
                vars.clone(),
                (std::sync::Arc::new(
                    move |__a0: metamodelica::Ref<DAE::Element>| -> metamodelica::Result<_> {
                        ::std::result::Result::Ok(DAEUtil::isOutputVar(&__a0))
                    },
                )
                    as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Element>) -> Result<bool> + 'static>),
            )?;
            func_params = pairFuncParamsWithArgs(&vars, inFuncArgs)?;
            func_params = sortFunctionVarsByDependency(func_params.clone(), inSource.clone())?;
            (cache, env) =
                setupFunctionEnvironment(inCache.clone(), inEnv.clone(), inFuncName.clone(), func_params.clone())?;
            (cache, env, _) = evaluateElements(
                body.clone(),
                cache.clone(),
                env.clone(),
                crate::CevalFunction::LoopControl::NEXT,
            )?;
            return_values = List::map1(
                output_vars.clone(),
                &move |__a0: metamodelica::Ref<DAE::Element>, __a1: FCore::Graph| getFunctionReturnValue(&__a0, __a1),
                env.clone(),
            )?;
            return_value = boxReturnValue(return_values.clone());
            Ok((cache.clone(), return_value.clone()))
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let DAE::FunctionDefinition::FUNCTION_EXT {
                body: mut body,
                externalDecl:
                    DAE::ExternalDecl {
                        name: mut ext_fun_name,
                        args: ref ext_fun_args,
                        ..
                    },
            } = __mc_input.clone()
            else {
                return Err("nomatch");
            };
            let mut vars: metamodelica::List<metamodelica::Ref<DAE::Element>>;
            let mut output_vars: metamodelica::List<metamodelica::Ref<DAE::Element>>;
            let mut func_params: metamodelica::List<(
                metamodelica::Ref<DAE::Element>,
                Option<metamodelica::Ref<Values::Value>>,
            )>;
            let mut cache: FCore::Cache;
            let mut env: FCore::Graph;
            let mut return_values: metamodelica::List<metamodelica::Ref<Values::Value>>;
            let mut return_value: metamodelica::Ref<Values::Value>;
            (vars, _) = List::splitOnFirstMatch(
                body.clone(),
                &move |__a0: metamodelica::Ref<DAE::Element>| -> metamodelica::Result<_> {
                    ::std::result::Result::Ok(DAEUtil::isNotVar(&__a0))
                },
            )?;
            vars = List::map(vars.clone(), &move |__a0: metamodelica::Ref<DAE::Element>| {
                removeSelfReferentialDims(&__a0)
            })?;
            output_vars = List::filterOnTrue(
                vars.clone(),
                (std::sync::Arc::new(
                    move |__a0: metamodelica::Ref<DAE::Element>| -> metamodelica::Result<_> {
                        ::std::result::Result::Ok(DAEUtil::isOutputVar(&__a0))
                    },
                )
                    as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Element>) -> Result<bool> + 'static>),
            )?;
            func_params = pairFuncParamsWithArgs(&vars, inFuncArgs)?;
            func_params = sortFunctionVarsByDependency(func_params.clone(), inSource.clone())?;
            (cache, env) =
                setupFunctionEnvironment(inCache.clone(), inEnv.clone(), inFuncName.clone(), func_params.clone())?;
            (cache, env) = evaluateExternalFunc(
                &(ext_fun_name.clone()),
                &(ext_fun_args.clone()),
                cache.clone(),
                env.clone(),
            )?;
            return_values = List::map1(
                output_vars.clone(),
                &move |__a0: metamodelica::Ref<DAE::Element>, __a1: FCore::Graph| getFunctionReturnValue(&__a0, __a1),
                env.clone(),
            )?;
            return_value = boxReturnValue(return_values.clone());
            Ok((cache.clone(), return_value.clone()))
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else {
                return Err("pattern mismatch");
            };
            Debug::trace(literal!("- CevalFunction.evaluateFunction failed.\n"))?;
            Ok(return Err("fail"))
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((outCache, outResult))
}

fn pairFuncParamsWithArgs(
    mut inElements: &metamodelica::List<metamodelica::Ref<DAE::Element>>,
    mut inValues: &metamodelica::List<metamodelica::Ref<Values::Value>>,
) -> Result<
    metamodelica::List<(
        metamodelica::Ref<DAE::Element>,
        Option<metamodelica::Ref<Values::Value>>,
    )>,
> {
    let mut outFunctionVars: metamodelica::List<(
        metamodelica::Ref<DAE::Element>,
        Option<metamodelica::Ref<Values::Value>>,
    )>;
    outFunctionVars = (::match_deref::match_deref! { match (inElements, inValues) {
        (Deref @ metamodelica::ListNode::Nil, Deref @ metamodelica::ListNode::Nil) => {
            metamodelica::nil()
        },
        (Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Element::VAR { direction: DAE::VarDirection::INPUT { .. }, .. }, tail: _ }, Deref @ metamodelica::ListNode::Nil) => {
            let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { return Err("pattern mismatch") };
            Debug::trace(literal!("- CevalFunction.pairFuncParamsWithArgs failed because of too few input arguments.\n"))?;
            return Err("fail")
        },
        (Deref @ metamodelica::ListNode::Cons { head: var @ Deref @ DAE::Element::VAR { direction: DAE::VarDirection::INPUT { .. }, .. }, tail: rest_vars }, Deref @ metamodelica::ListNode::Cons { head: val, tail: rest_vals }) => {
            let mut params: metamodelica::List<(metamodelica::Ref<DAE::Element>, Option<metamodelica::Ref<Values::Value>>)>;
            params = pairFuncParamsWithArgs(rest_vars, rest_vals)?;
            metamodelica::cons((var.clone(), Some(val.clone())), params)
        },
        (Deref @ metamodelica::ListNode::Cons { head: var, tail: rest_vars }, _) => {
            let mut params: metamodelica::List<(metamodelica::Ref<DAE::Element>, Option<metamodelica::Ref<Values::Value>>)>;
            params = pairFuncParamsWithArgs(rest_vars, inValues)?;
            metamodelica::cons((var.clone(), None), params)
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok(outFunctionVars)
}

fn removeSelfReferentialDims(
    mut inElement: &metamodelica::Ref<DAE::Element>,
) -> Result<metamodelica::Ref<DAE::Element>> {
    let mut outElement: metamodelica::Ref<DAE::Element>;
    outElement = (::match_deref::match_deref! { match inElement {
        Deref @ DAE::Element::VAR { componentRef: cref @ Deref @ DAE::ComponentRef::CREF_IDENT { ident: name, .. }, kind: vk, direction: vd, parallelism: vp, protection: vv, ty, binding: bind, dims, connectorType: ct, source: es, variableAttributesOption: va, comment: cmt, innerOuter: io, encrypted: e } => {
            let mut dims = (*dims).clone();
            dims = List::map1(dims.clone(), &fnptr!(removeSelfReferentialDim, metamodelica::Ref<DAE::Dimension>, ArcStr), name.clone())?;
            metamodelica::Ref::new(DAE::Element::VAR { componentRef: cref.clone(), kind: vk.clone(), direction: vd.clone(), parallelism: vp.clone(), protection: vv.clone(), ty: ty.clone(), binding: bind.clone(), dims: dims.clone(), connectorType: ct.clone(), source: es.clone(), variableAttributesOption: va.clone(), comment: cmt.clone(), innerOuter: io.clone(), encrypted: e.clone() })
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok(outElement)
}

fn removeSelfReferentialDim(
    mut inDim: metamodelica::Ref<DAE::Dimension>,
    mut inName: ArcStr,
) -> metamodelica::Ref<DAE::Dimension> {
    let mut outDim: metamodelica::Ref<DAE::Dimension>;
    outDim = 'mc: {
        let __mc_input = &*inDim;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Dimension::DIM_EXP { exp } => {
                    let mut crefs: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
                    crefs = Expression::extractCrefsFromExp(exp.clone())?;
                    let true = (List::isMemberOnTrue(inName.clone(), &crefs, &move |__a0: ArcStr, __a1: metamodelica::Ref<DAE::ComponentRef>| -> metamodelica::Result<_> { ::std::result::Result::Ok(isCrefNamed(&__a0, &__a1)) })?) else { return Err("pattern mismatch") };
                    Ok(openmodelica_frontend_types::DAE::Dimension::interned_DIM_UNKNOWN())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(inDim.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    outDim
}

fn isCrefNamed(mut inName: &ArcStr, mut inCref: &metamodelica::Ref<DAE::ComponentRef>) -> bool {
    let mut outIsNamed: bool;
    outIsNamed = (match &**inCref {
        DAE::ComponentRef::CREF_IDENT { ident: name, .. } => stringEq(&inName, &name),
        _ => false,
    });
    outIsNamed
}

fn evaluateExtInputArg(
    mut inArgument: &DAE::ExtArg,
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
) -> Result<(metamodelica::Ref<Values::Value>, FCore::Cache)> {
    let mut outValue: metamodelica::Ref<Values::Value>;
    let mut outCache: FCore::Cache;
    (outValue, outCache) = 'mc: {
        let __mc_input = (inArgument.clone(), inCache.clone());
        if let Ok(__v) = (|| -> Result<_> {
            let (
                DAE::ExtArg::EXTARG {
                    componentRef: ref cref,
                    type_: ref ty,
                    ..
                },
                _,
            ) = __mc_input.clone()
            else {
                return Err("nomatch");
            };
            let mut val: metamodelica::Ref<Values::Value>;
            val = getVariableValue(cref.clone(), &(ty.clone()), inEnv.clone())?;
            Ok((val.clone(), inCache.clone()))
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let (DAE::ExtArg::EXTARGEXP { exp: mut exp, .. }, mut cache) = __mc_input.clone() else {
                return Err("nomatch");
            };
            let mut val: metamodelica::Ref<Values::Value>;
            (cache, val) = cevalExp(exp.clone(), cache.clone(), inEnv.clone())?;
            Ok((val.clone(), cache.clone()))
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let (
                DAE::ExtArg::EXTARGSIZE {
                    componentRef: ref cref,
                    exp: mut exp,
                    ..
                },
                mut cache,
            ) = __mc_input.clone()
            else {
                return Err("nomatch");
            };
            let mut val: metamodelica::Ref<Values::Value>;
            let mut exp = exp.clone();
            exp = metamodelica::Ref::new(DAE::Exp::SIZE {
                exp: metamodelica::Ref::new(DAE::Exp::CREF {
                    componentRef: cref.clone(),
                    ty: DAE::T_UNKNOWN_DEFAULT().clone(),
                }),
                sz: Some(exp.clone()),
            });
            (cache, val) = cevalExp(exp.clone(), cache.clone(), inEnv.clone())?;
            Ok((val.clone(), cache.clone()))
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            let mut err_str: ArcStr;
            let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else {
                return Err("pattern mismatch");
            };
            err_str = DAEDump::dumpExtArgStr(inArgument)?;
            Debug::traceln({
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("- CevalFunction.evaluateExtInputArg failed on "));
                __mm_s.push_str(&*err_str);
                ArcStr::from(__mm_s)
            })?;
            Ok(return Err("fail"))
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((outValue, outCache))
}

fn evaluateExtIntArg(
    mut inArg: &DAE::ExtArg,
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
) -> Result<(i32, FCore::Cache)> {
    let mut outValue: i32;
    let mut outCache: FCore::Cache;
    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(evaluateExtInputArg(inArg, inCache, inEnv)?) {
        (Deref @ Values::Value::INTEGER { integer: __pa0 }, __pa1) => (__pa0.clone(), __pa1.clone()),
        _ => return Err("pattern mismatch"),
    } };
    outValue = metamodelica::Own::own(__pa0);
    outCache = metamodelica::Own::own(__pa1);
    Ok((outValue, outCache))
}

fn evaluateExtRealArg(
    mut inArg: &DAE::ExtArg,
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
) -> Result<(metamodelica::Real, FCore::Cache)> {
    let mut outValue: metamodelica::Real;
    let mut outCache: FCore::Cache;
    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(evaluateExtInputArg(inArg, inCache, inEnv)?) {
        (Deref @ Values::Value::REAL { real: __pa0 }, __pa1) => (__pa0.clone(), __pa1.clone()),
        _ => return Err("pattern mismatch"),
    } };
    outValue = metamodelica::Own::own(__pa0);
    outCache = metamodelica::Own::own(__pa1);
    Ok((outValue, outCache))
}

fn evaluateExtStringArg(
    mut inArg: &DAE::ExtArg,
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
) -> Result<(ArcStr, FCore::Cache)> {
    let mut outValue: ArcStr;
    let mut outCache: FCore::Cache;
    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(evaluateExtInputArg(inArg, inCache, inEnv)?) {
        (Deref @ Values::Value::STRING { string: __pa0 }, __pa1) => (__pa0.clone(), __pa1.clone()),
        _ => return Err("pattern mismatch"),
    } };
    outValue = metamodelica::Own::own(__pa0);
    outCache = metamodelica::Own::own(__pa1);
    Ok((outValue, outCache))
}

fn evaluateExtIntArrayArg(
    mut inArg: &DAE::ExtArg,
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
) -> Result<(metamodelica::List<i32>, FCore::Cache)> {
    let mut outValue: metamodelica::List<i32>;
    let mut outCache: FCore::Cache;
    let mut val: metamodelica::Ref<Values::Value>;
    (val, outCache) = evaluateExtInputArg(inArg, inCache, inEnv)?;
    outValue = ValuesUtil::arrayValueInts(&val)?;
    Ok((outValue, outCache))
}

fn evaluateExtRealArrayArg(
    mut inArg: &DAE::ExtArg,
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
) -> Result<(metamodelica::List<metamodelica::Real>, FCore::Cache)> {
    let mut outValue: metamodelica::List<metamodelica::Real>;
    let mut outCache: FCore::Cache;
    let mut val: metamodelica::Ref<Values::Value>;
    (val, outCache) = evaluateExtInputArg(inArg, inCache, inEnv)?;
    outValue = ValuesUtil::arrayValueReals(&val)?;
    Ok((outValue, outCache))
}

fn evaluateExtRealMatrixArg(
    mut inArg: &DAE::ExtArg,
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
) -> Result<(metamodelica::List<metamodelica::List<metamodelica::Real>>, FCore::Cache)> {
    let mut outValue: metamodelica::List<metamodelica::List<metamodelica::Real>>;
    let mut outCache: FCore::Cache;
    let mut val: metamodelica::Ref<Values::Value>;
    (val, outCache) = evaluateExtInputArg(inArg, inCache, inEnv)?;
    outValue = ValuesUtil::matrixValueReals(&val)?;
    Ok((outValue, outCache))
}

fn evaluateExtOutputArg(mut inArg: DAE::ExtArg) -> Result<metamodelica::Ref<DAE::ComponentRef>> {
    let mut outCref: metamodelica::Ref<DAE::ComponentRef>;
    let DAE::EXTARG {
        componentRef: __pa0, ..
    } = (inArg)
    else {
        return Err("pattern mismatch");
    };
    outCref = metamodelica::Own::own(__pa0);
    Ok(outCref)
}

fn assignExtOutputs(
    mut inArgs: metamodelica::List<DAE::ExtArg>,
    mut inValues: metamodelica::List<metamodelica::Ref<Values::Value>>,
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
) -> Result<(FCore::Cache, FCore::Graph)> {
    '__tco: loop {
        ::match_deref::match_deref! { match &((inArgs, inValues, inCache.clone(), inEnv.clone())) {
            (Deref @ metamodelica::ListNode::Nil, Deref @ metamodelica::ListNode::Nil, _, _) => {
                return Ok((inCache, inEnv))
            },
            (Deref @ metamodelica::ListNode::Cons { head: arg, tail: rest_args }, Deref @ metamodelica::ListNode::Cons { head: val, tail: rest_vals }, cache, env) => {
                let mut cr: metamodelica::Ref<DAE::ComponentRef>;
                let mut val = (*val).clone();
                let mut cache = (*cache).clone();
                let mut env = (*env).clone();
                cr = evaluateExtOutputArg(arg.clone())?;
                val = unliftExtOutputValue(cr.clone(), val.clone(), env.clone());
                (cache, env) = assignVariable(cr, metamodelica::AsArg::as_arg(&val), metamodelica::AsArg::as_arg(&cache), metamodelica::AsArg::as_arg(&env))?;
                { (inArgs, inValues, inCache, inEnv) = (rest_args.clone(), rest_vals.clone(), cache.clone(), env.clone()); continue '__tco; }
            },
            _ => return Err("match: no arm matched"),
        } }
    }
}

fn unliftExtOutputValue(
    mut inCref: metamodelica::Ref<DAE::ComponentRef>,
    mut inValue: metamodelica::Ref<Values::Value>,
    mut inEnv: FCore::Graph,
) -> metamodelica::Ref<Values::Value> {
    let mut outValue: metamodelica::Ref<Values::Value>;
    outValue = 'mc: {
        let __mc_input = &*inValue;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ Values::Value::ARRAY { valueLst: vals @ Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::ARRAY { .. }, tail: _ }, dimLst: Deref @ metamodelica::ListNode::Cons { head: dim, tail: _ } } => {
                    let mut ty: metamodelica::Ref<DAE::Type>;
                    let mut dims: metamodelica::List<metamodelica::Ref<DAE::Dimension>>;
                    let mut vals = (*vals).clone();
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(getVariableTypeAndBinding(inCref.clone(), inEnv.clone())?) {
                        (Deref @ DAE::Type::T_ARRAY { ty: __pa0, dims: __pa1 }, _) => (__pa0.clone(), __pa1.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    ty = metamodelica::Own::own(__pa0);
                    dims = metamodelica::Own::own(__pa1);
                    let false = (Types::isNonscalarArray(&ty, &dims)) else { return Err("pattern mismatch") };
                    vals = List::map(vals.clone(), &move |__a0: metamodelica::Ref<Values::Value>| ValuesUtil::arrayScalar(&__a0))?;
                    Ok(metamodelica::Ref::new(Values::Value::ARRAY { valueLst: vals.clone(), dimLst: list![dim.clone()] }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(inValue.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    outValue
}

fn evaluateExternalFunc(
    mut inFuncName: &ArcStr,
    mut inFuncArgs: &metamodelica::List<DAE::ExtArg>,
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
) -> Result<(FCore::Cache, FCore::Graph)> {
    let mut outCache: FCore::Cache;
    let mut outEnv: FCore::Graph;
    (outCache, outEnv) = (::match_deref::match_deref! { match &((inFuncName.clone(), &**inFuncArgs)) {
        (Deref @ "dgeev", Deref @ metamodelica::ListNode::Cons { head: arg_JOBVL, tail: Deref @ metamodelica::ListNode::Cons { head: arg_JOBVR, tail: Deref @ metamodelica::ListNode::Cons { head: arg_N, tail: Deref @ metamodelica::ListNode::Cons { head: arg_A, tail: Deref @ metamodelica::ListNode::Cons { head: arg_LDA, tail: Deref @ metamodelica::ListNode::Cons { head: arg_WR, tail: Deref @ metamodelica::ListNode::Cons { head: arg_WI, tail: Deref @ metamodelica::ListNode::Cons { head: arg_VL, tail: Deref @ metamodelica::ListNode::Cons { head: arg_LDVL, tail: Deref @ metamodelica::ListNode::Cons { head: arg_VR, tail: Deref @ metamodelica::ListNode::Cons { head: arg_LDVR, tail: Deref @ metamodelica::ListNode::Cons { head: arg_WORK, tail: Deref @ metamodelica::ListNode::Cons { head: arg_LWORK, tail: Deref @ metamodelica::ListNode::Cons { head: arg_INFO, tail: Deref @ metamodelica::ListNode::Nil } } } } } } } } } } } } } }) => {
            let mut cache = inCache;
            let mut env = inEnv;
            let mut val_INFO: metamodelica::Ref<Values::Value>;
            let mut val_WI: metamodelica::Ref<Values::Value>;
            let mut val_WORK: metamodelica::Ref<Values::Value>;
            let mut val_WR: metamodelica::Ref<Values::Value>;
            let mut val_A: metamodelica::Ref<Values::Value>;
            let mut val_VL: metamodelica::Ref<Values::Value>;
            let mut val_VR: metamodelica::Ref<Values::Value>;
            let mut INFO: i32;
            let mut LDA: i32;
            let mut LDVL: i32;
            let mut LDVR: i32;
            let mut LWORK: i32;
            let mut N: i32;
            let mut JOBVL: ArcStr;
            let mut JOBVR: ArcStr;
            let mut WI: metamodelica::List<metamodelica::Real>;
            let mut WORK: metamodelica::List<metamodelica::Real>;
            let mut WR: metamodelica::List<metamodelica::Real>;
            let mut A: metamodelica::List<metamodelica::List<metamodelica::Real>>;
            let mut VL: metamodelica::List<metamodelica::List<metamodelica::Real>>;
            let mut VR: metamodelica::List<metamodelica::List<metamodelica::Real>>;
            let mut arg_out: metamodelica::List<DAE::ExtArg>;
            let mut val_out: metamodelica::List<metamodelica::Ref<Values::Value>>;
            (JOBVL, cache) = evaluateExtStringArg(metamodelica::AsArg::as_arg(&arg_JOBVL), cache, env.clone())?;
            (JOBVR, cache) = evaluateExtStringArg(metamodelica::AsArg::as_arg(&arg_JOBVR), cache, env.clone())?;
            (N, cache) = evaluateExtIntArg(metamodelica::AsArg::as_arg(&arg_N), cache, env.clone())?;
            (A, cache) = evaluateExtRealMatrixArg(metamodelica::AsArg::as_arg(&arg_A), cache, env.clone())?;
            (LDA, cache) = evaluateExtIntArg(metamodelica::AsArg::as_arg(&arg_LDA), cache, env.clone())?;
            (LDVL, cache) = evaluateExtIntArg(metamodelica::AsArg::as_arg(&arg_LDVL), cache, env.clone())?;
            (LDVR, cache) = evaluateExtIntArg(metamodelica::AsArg::as_arg(&arg_LDVR), cache, env.clone())?;
            (WORK, cache) = evaluateExtRealArrayArg(metamodelica::AsArg::as_arg(&arg_WORK), cache, env.clone())?;
            (LWORK, cache) = evaluateExtIntArg(metamodelica::AsArg::as_arg(&arg_LWORK), cache, env.clone())?;
            (A, WR, WI, VL, VR, WORK, INFO) = Lapack::dgeev(JOBVL, JOBVR, N, A, LDA, LDVL, LDVR, WORK, LWORK);
            val_A = ValuesMake::makeRealMatrix(A)?;
            val_WR = ValuesMake::makeRealArray(WR)?;
            val_WI = ValuesMake::makeRealArray(WI)?;
            val_VL = ValuesMake::makeRealMatrix(VL)?;
            val_VR = ValuesMake::makeRealMatrix(VR)?;
            val_WORK = ValuesMake::makeRealArray(WORK)?;
            val_INFO = ValuesMake::makeInteger(INFO);
            arg_out = list![arg_A.clone(), arg_WR.clone(), arg_WI.clone(), arg_VL.clone(), arg_VR.clone(), arg_WORK.clone(), arg_INFO.clone()];
            val_out = list![val_A, val_WR, val_WI, val_VL, val_VR, val_WORK, val_INFO];
            (cache, env) = assignExtOutputs(arg_out, val_out, cache, env)?;
            (cache, env)
        },
        (Deref @ "dgegv", Deref @ metamodelica::ListNode::Cons { head: arg_JOBVL, tail: Deref @ metamodelica::ListNode::Cons { head: arg_JOBVR, tail: Deref @ metamodelica::ListNode::Cons { head: arg_N, tail: Deref @ metamodelica::ListNode::Cons { head: arg_A, tail: Deref @ metamodelica::ListNode::Cons { head: arg_LDA, tail: Deref @ metamodelica::ListNode::Cons { head: arg_B, tail: Deref @ metamodelica::ListNode::Cons { head: arg_LDB, tail: Deref @ metamodelica::ListNode::Cons { head: arg_ALPHAR, tail: Deref @ metamodelica::ListNode::Cons { head: arg_ALPHAI, tail: Deref @ metamodelica::ListNode::Cons { head: arg_BETA, tail: Deref @ metamodelica::ListNode::Cons { head: arg_VL, tail: Deref @ metamodelica::ListNode::Cons { head: arg_LDVL, tail: Deref @ metamodelica::ListNode::Cons { head: arg_VR, tail: Deref @ metamodelica::ListNode::Cons { head: arg_LDVR, tail: Deref @ metamodelica::ListNode::Cons { head: arg_WORK, tail: Deref @ metamodelica::ListNode::Cons { head: arg_LWORK, tail: Deref @ metamodelica::ListNode::Cons { head: arg_INFO, tail: Deref @ metamodelica::ListNode::Nil } } } } } } } } } } } } } } } } }) => {
            let mut cache = inCache;
            let mut env = inEnv;
            let mut val_INFO: metamodelica::Ref<Values::Value>;
            let mut val_ALPHAI: metamodelica::Ref<Values::Value>;
            let mut val_ALPHAR: metamodelica::Ref<Values::Value>;
            let mut val_BETA: metamodelica::Ref<Values::Value>;
            let mut val_WORK: metamodelica::Ref<Values::Value>;
            let mut val_VL: metamodelica::Ref<Values::Value>;
            let mut val_VR: metamodelica::Ref<Values::Value>;
            let mut INFO: i32;
            let mut LDA: i32;
            let mut LDB: i32;
            let mut LDVL: i32;
            let mut LDVR: i32;
            let mut LWORK: i32;
            let mut N: i32;
            let mut JOBVL: ArcStr;
            let mut JOBVR: ArcStr;
            let mut ALPHAI: metamodelica::List<metamodelica::Real>;
            let mut ALPHAR: metamodelica::List<metamodelica::Real>;
            let mut BETA: metamodelica::List<metamodelica::Real>;
            let mut WORK: metamodelica::List<metamodelica::Real>;
            let mut A: metamodelica::List<metamodelica::List<metamodelica::Real>>;
            let mut B: metamodelica::List<metamodelica::List<metamodelica::Real>>;
            let mut VL: metamodelica::List<metamodelica::List<metamodelica::Real>>;
            let mut VR: metamodelica::List<metamodelica::List<metamodelica::Real>>;
            let mut arg_out: metamodelica::List<DAE::ExtArg>;
            let mut val_out: metamodelica::List<metamodelica::Ref<Values::Value>>;
            (JOBVL, cache) = evaluateExtStringArg(metamodelica::AsArg::as_arg(&arg_JOBVL), cache, env.clone())?;
            (JOBVR, cache) = evaluateExtStringArg(metamodelica::AsArg::as_arg(&arg_JOBVR), cache, env.clone())?;
            (N, cache) = evaluateExtIntArg(metamodelica::AsArg::as_arg(&arg_N), cache, env.clone())?;
            (A, cache) = evaluateExtRealMatrixArg(metamodelica::AsArg::as_arg(&arg_A), cache, env.clone())?;
            (LDA, cache) = evaluateExtIntArg(metamodelica::AsArg::as_arg(&arg_LDA), cache, env.clone())?;
            (B, cache) = evaluateExtRealMatrixArg(metamodelica::AsArg::as_arg(&arg_B), cache, env.clone())?;
            (LDB, cache) = evaluateExtIntArg(metamodelica::AsArg::as_arg(&arg_LDB), cache, env.clone())?;
            (LDVL, cache) = evaluateExtIntArg(metamodelica::AsArg::as_arg(&arg_LDVL), cache, env.clone())?;
            (LDVR, cache) = evaluateExtIntArg(metamodelica::AsArg::as_arg(&arg_LDVR), cache, env.clone())?;
            (WORK, cache) = evaluateExtRealArrayArg(metamodelica::AsArg::as_arg(&arg_WORK), cache, env.clone())?;
            (LWORK, cache) = evaluateExtIntArg(metamodelica::AsArg::as_arg(&arg_LWORK), cache, env.clone())?;
            (ALPHAR, ALPHAI, BETA, VL, VR, WORK, INFO) = Lapack::dgegv(JOBVL, JOBVR, N, A, LDA, B, LDB, LDVL, LDVR, WORK, LWORK);
            val_ALPHAR = ValuesMake::makeRealArray(ALPHAR)?;
            val_ALPHAI = ValuesMake::makeRealArray(ALPHAI)?;
            val_BETA = ValuesMake::makeRealArray(BETA)?;
            val_VL = ValuesMake::makeRealMatrix(VL)?;
            val_VR = ValuesMake::makeRealMatrix(VR)?;
            val_WORK = ValuesMake::makeRealArray(WORK)?;
            val_INFO = ValuesMake::makeInteger(INFO);
            arg_out = list![arg_ALPHAR.clone(), arg_ALPHAI.clone(), arg_BETA.clone(), arg_VL.clone(), arg_VR.clone(), arg_WORK.clone(), arg_INFO.clone()];
            val_out = list![val_ALPHAR, val_ALPHAI, val_BETA, val_VL, val_VR, val_WORK, val_INFO];
            (cache, env) = assignExtOutputs(arg_out, val_out, cache, env)?;
            (cache, env)
        },
        (Deref @ "dgels", Deref @ metamodelica::ListNode::Cons { head: arg_TRANS, tail: Deref @ metamodelica::ListNode::Cons { head: arg_M, tail: Deref @ metamodelica::ListNode::Cons { head: arg_N, tail: Deref @ metamodelica::ListNode::Cons { head: arg_NRHS, tail: Deref @ metamodelica::ListNode::Cons { head: arg_A, tail: Deref @ metamodelica::ListNode::Cons { head: arg_LDA, tail: Deref @ metamodelica::ListNode::Cons { head: arg_B, tail: Deref @ metamodelica::ListNode::Cons { head: arg_LDB, tail: Deref @ metamodelica::ListNode::Cons { head: arg_WORK, tail: Deref @ metamodelica::ListNode::Cons { head: arg_LWORK, tail: Deref @ metamodelica::ListNode::Cons { head: arg_INFO, tail: Deref @ metamodelica::ListNode::Nil } } } } } } } } } } }) => {
            let mut cache = inCache;
            let mut env = inEnv;
            let mut val_INFO: metamodelica::Ref<Values::Value>;
            let mut val_WORK: metamodelica::Ref<Values::Value>;
            let mut val_A: metamodelica::Ref<Values::Value>;
            let mut val_B: metamodelica::Ref<Values::Value>;
            let mut INFO: i32;
            let mut LDA: i32;
            let mut LDB: i32;
            let mut LWORK: i32;
            let mut M: i32;
            let mut N: i32;
            let mut NRHS: i32;
            let mut TRANS: ArcStr;
            let mut WORK: metamodelica::List<metamodelica::Real>;
            let mut A: metamodelica::List<metamodelica::List<metamodelica::Real>>;
            let mut B: metamodelica::List<metamodelica::List<metamodelica::Real>>;
            let mut arg_out: metamodelica::List<DAE::ExtArg>;
            let mut val_out: metamodelica::List<metamodelica::Ref<Values::Value>>;
            (TRANS, cache) = evaluateExtStringArg(metamodelica::AsArg::as_arg(&arg_TRANS), cache, env.clone())?;
            (M, cache) = evaluateExtIntArg(metamodelica::AsArg::as_arg(&arg_M), cache, env.clone())?;
            (N, cache) = evaluateExtIntArg(metamodelica::AsArg::as_arg(&arg_N), cache, env.clone())?;
            (NRHS, cache) = evaluateExtIntArg(metamodelica::AsArg::as_arg(&arg_NRHS), cache, env.clone())?;
            (A, cache) = evaluateExtRealMatrixArg(metamodelica::AsArg::as_arg(&arg_A), cache, env.clone())?;
            (LDA, cache) = evaluateExtIntArg(metamodelica::AsArg::as_arg(&arg_LDA), cache, env.clone())?;
            (B, cache) = evaluateExtRealMatrixArg(metamodelica::AsArg::as_arg(&arg_B), cache, env.clone())?;
            (LDB, cache) = evaluateExtIntArg(metamodelica::AsArg::as_arg(&arg_LDB), cache, env.clone())?;
            (WORK, cache) = evaluateExtRealArrayArg(metamodelica::AsArg::as_arg(&arg_WORK), cache, env.clone())?;
            (LWORK, cache) = evaluateExtIntArg(metamodelica::AsArg::as_arg(&arg_LWORK), cache, env.clone())?;
            (A, B, WORK, INFO) = Lapack::dgels(TRANS, M, N, NRHS, A, LDA, B, LDB, WORK, LWORK);
            val_A = ValuesMake::makeRealMatrix(A)?;
            val_B = ValuesMake::makeRealMatrix(B)?;
            val_WORK = ValuesMake::makeRealArray(WORK)?;
            val_INFO = ValuesMake::makeInteger(INFO);
            arg_out = list![arg_A.clone(), arg_B.clone(), arg_WORK.clone(), arg_INFO.clone()];
            val_out = list![val_A, val_B, val_WORK, val_INFO];
            (cache, env) = assignExtOutputs(arg_out, val_out, cache, env)?;
            (cache, env)
        },
        (Deref @ "dgelsx", Deref @ metamodelica::ListNode::Cons { head: arg_M, tail: Deref @ metamodelica::ListNode::Cons { head: arg_N, tail: Deref @ metamodelica::ListNode::Cons { head: arg_NRHS, tail: Deref @ metamodelica::ListNode::Cons { head: arg_A, tail: Deref @ metamodelica::ListNode::Cons { head: arg_LDA, tail: Deref @ metamodelica::ListNode::Cons { head: arg_B, tail: Deref @ metamodelica::ListNode::Cons { head: arg_LDB, tail: Deref @ metamodelica::ListNode::Cons { head: arg_JPVT, tail: Deref @ metamodelica::ListNode::Cons { head: arg_RCOND, tail: Deref @ metamodelica::ListNode::Cons { head: arg_RANK, tail: Deref @ metamodelica::ListNode::Cons { head: arg_WORK, tail: Deref @ metamodelica::ListNode::Cons { head: arg_INFO, tail: Deref @ metamodelica::ListNode::Nil } } } } } } } } } } } }) => {
            let mut cache = inCache;
            let mut env = inEnv;
            let mut val_INFO: metamodelica::Ref<Values::Value>;
            let mut val_RANK: metamodelica::Ref<Values::Value>;
            let mut val_JPVT: metamodelica::Ref<Values::Value>;
            let mut val_A: metamodelica::Ref<Values::Value>;
            let mut val_B: metamodelica::Ref<Values::Value>;
            let mut INFO: i32;
            let mut LDA: i32;
            let mut LDB: i32;
            let mut M: i32;
            let mut N: i32;
            let mut NRHS: i32;
            let mut RANK: i32;
            let mut RCOND: metamodelica::Real;
            let mut JPVT: metamodelica::List<i32>;
            let mut WORK: metamodelica::List<metamodelica::Real>;
            let mut A: metamodelica::List<metamodelica::List<metamodelica::Real>>;
            let mut B: metamodelica::List<metamodelica::List<metamodelica::Real>>;
            let mut arg_out: metamodelica::List<DAE::ExtArg>;
            let mut val_out: metamodelica::List<metamodelica::Ref<Values::Value>>;
            (M, cache) = evaluateExtIntArg(metamodelica::AsArg::as_arg(&arg_M), cache, env.clone())?;
            (N, cache) = evaluateExtIntArg(metamodelica::AsArg::as_arg(&arg_N), cache, env.clone())?;
            (NRHS, cache) = evaluateExtIntArg(metamodelica::AsArg::as_arg(&arg_NRHS), cache, env.clone())?;
            (A, cache) = evaluateExtRealMatrixArg(metamodelica::AsArg::as_arg(&arg_A), cache, env.clone())?;
            (LDA, cache) = evaluateExtIntArg(metamodelica::AsArg::as_arg(&arg_LDA), cache, env.clone())?;
            (B, cache) = evaluateExtRealMatrixArg(metamodelica::AsArg::as_arg(&arg_B), cache, env.clone())?;
            (LDB, cache) = evaluateExtIntArg(metamodelica::AsArg::as_arg(&arg_LDB), cache, env.clone())?;
            (JPVT, cache) = evaluateExtIntArrayArg(metamodelica::AsArg::as_arg(&arg_JPVT), cache, env.clone())?;
            (RCOND, cache) = evaluateExtRealArg(metamodelica::AsArg::as_arg(&arg_RCOND), cache, env.clone())?;
            (WORK, cache) = evaluateExtRealArrayArg(metamodelica::AsArg::as_arg(&arg_WORK), cache, env.clone())?;
            (A, B, JPVT, RANK, INFO) = Lapack::dgelsx(M, N, NRHS, A, LDA, B, LDB, JPVT, RCOND, WORK);
            val_A = ValuesMake::makeRealMatrix(A)?;
            val_B = ValuesMake::makeRealMatrix(B)?;
            val_JPVT = ValuesMake::makeIntArray(JPVT)?;
            val_RANK = ValuesMake::makeInteger(RANK);
            val_INFO = ValuesMake::makeInteger(INFO);
            arg_out = list![arg_A.clone(), arg_B.clone(), arg_JPVT.clone(), arg_RANK.clone(), arg_INFO.clone()];
            val_out = list![val_A, val_B, val_JPVT, val_RANK, val_INFO];
            (cache, env) = assignExtOutputs(arg_out, val_out, cache, env)?;
            (cache, env)
        },
        (Deref @ "dgelsx", Deref @ metamodelica::ListNode::Cons { head: arg_M, tail: Deref @ metamodelica::ListNode::Cons { head: arg_N, tail: Deref @ metamodelica::ListNode::Cons { head: arg_NRHS, tail: Deref @ metamodelica::ListNode::Cons { head: arg_A, tail: Deref @ metamodelica::ListNode::Cons { head: arg_LDA, tail: Deref @ metamodelica::ListNode::Cons { head: arg_B, tail: Deref @ metamodelica::ListNode::Cons { head: arg_LDB, tail: Deref @ metamodelica::ListNode::Cons { head: arg_JPVT, tail: Deref @ metamodelica::ListNode::Cons { head: arg_RCOND, tail: Deref @ metamodelica::ListNode::Cons { head: arg_RANK, tail: Deref @ metamodelica::ListNode::Cons { head: arg_WORK, tail: Deref @ metamodelica::ListNode::Cons { head: _, tail: Deref @ metamodelica::ListNode::Cons { head: arg_INFO, tail: Deref @ metamodelica::ListNode::Nil } } } } } } } } } } } } }) => {
            let mut cache = inCache;
            let mut env = inEnv;
            let mut val_INFO: metamodelica::Ref<Values::Value>;
            let mut val_RANK: metamodelica::Ref<Values::Value>;
            let mut val_JPVT: metamodelica::Ref<Values::Value>;
            let mut val_A: metamodelica::Ref<Values::Value>;
            let mut val_B: metamodelica::Ref<Values::Value>;
            let mut INFO: i32;
            let mut LDA: i32;
            let mut LDB: i32;
            let mut M: i32;
            let mut N: i32;
            let mut NRHS: i32;
            let mut RANK: i32;
            let mut RCOND: metamodelica::Real;
            let mut JPVT: metamodelica::List<i32>;
            let mut WORK: metamodelica::List<metamodelica::Real>;
            let mut A: metamodelica::List<metamodelica::List<metamodelica::Real>>;
            let mut B: metamodelica::List<metamodelica::List<metamodelica::Real>>;
            let mut arg_out: metamodelica::List<DAE::ExtArg>;
            let mut val_out: metamodelica::List<metamodelica::Ref<Values::Value>>;
            (M, cache) = evaluateExtIntArg(metamodelica::AsArg::as_arg(&arg_M), cache, env.clone())?;
            (N, cache) = evaluateExtIntArg(metamodelica::AsArg::as_arg(&arg_N), cache, env.clone())?;
            (NRHS, cache) = evaluateExtIntArg(metamodelica::AsArg::as_arg(&arg_NRHS), cache, env.clone())?;
            (A, cache) = evaluateExtRealMatrixArg(metamodelica::AsArg::as_arg(&arg_A), cache, env.clone())?;
            (LDA, cache) = evaluateExtIntArg(metamodelica::AsArg::as_arg(&arg_LDA), cache, env.clone())?;
            (B, cache) = evaluateExtRealMatrixArg(metamodelica::AsArg::as_arg(&arg_B), cache, env.clone())?;
            (LDB, cache) = evaluateExtIntArg(metamodelica::AsArg::as_arg(&arg_LDB), cache, env.clone())?;
            (JPVT, cache) = evaluateExtIntArrayArg(metamodelica::AsArg::as_arg(&arg_JPVT), cache, env.clone())?;
            (RCOND, cache) = evaluateExtRealArg(metamodelica::AsArg::as_arg(&arg_RCOND), cache, env.clone())?;
            (WORK, cache) = evaluateExtRealArrayArg(metamodelica::AsArg::as_arg(&arg_WORK), cache, env.clone())?;
            (A, B, JPVT, RANK, INFO) = Lapack::dgelsx(M, N, NRHS, A, LDA, B, LDB, JPVT, RCOND, WORK);
            val_A = ValuesMake::makeRealMatrix(A)?;
            val_B = ValuesMake::makeRealMatrix(B)?;
            val_JPVT = ValuesMake::makeIntArray(JPVT)?;
            val_RANK = ValuesMake::makeInteger(RANK);
            val_INFO = ValuesMake::makeInteger(INFO);
            arg_out = list![arg_A.clone(), arg_B.clone(), arg_JPVT.clone(), arg_RANK.clone(), arg_INFO.clone()];
            val_out = list![val_A, val_B, val_JPVT, val_RANK, val_INFO];
            (cache, env) = assignExtOutputs(arg_out, val_out, cache, env)?;
            (cache, env)
        },
        (Deref @ "dgelsy", Deref @ metamodelica::ListNode::Cons { head: arg_M, tail: Deref @ metamodelica::ListNode::Cons { head: arg_N, tail: Deref @ metamodelica::ListNode::Cons { head: arg_NRHS, tail: Deref @ metamodelica::ListNode::Cons { head: arg_A, tail: Deref @ metamodelica::ListNode::Cons { head: arg_LDA, tail: Deref @ metamodelica::ListNode::Cons { head: arg_B, tail: Deref @ metamodelica::ListNode::Cons { head: arg_LDB, tail: Deref @ metamodelica::ListNode::Cons { head: arg_JPVT, tail: Deref @ metamodelica::ListNode::Cons { head: arg_RCOND, tail: Deref @ metamodelica::ListNode::Cons { head: arg_RANK, tail: Deref @ metamodelica::ListNode::Cons { head: arg_WORK, tail: Deref @ metamodelica::ListNode::Cons { head: arg_LWORK, tail: Deref @ metamodelica::ListNode::Cons { head: arg_INFO, tail: Deref @ metamodelica::ListNode::Nil } } } } } } } } } } } } }) => {
            let mut cache = inCache;
            let mut env = inEnv;
            let mut val_INFO: metamodelica::Ref<Values::Value>;
            let mut val_RANK: metamodelica::Ref<Values::Value>;
            let mut val_JPVT: metamodelica::Ref<Values::Value>;
            let mut val_WORK: metamodelica::Ref<Values::Value>;
            let mut val_A: metamodelica::Ref<Values::Value>;
            let mut val_B: metamodelica::Ref<Values::Value>;
            let mut INFO: i32;
            let mut LDA: i32;
            let mut LDB: i32;
            let mut LWORK: i32;
            let mut M: i32;
            let mut N: i32;
            let mut NRHS: i32;
            let mut RANK: i32;
            let mut RCOND: metamodelica::Real;
            let mut JPVT: metamodelica::List<i32>;
            let mut WORK: metamodelica::List<metamodelica::Real>;
            let mut A: metamodelica::List<metamodelica::List<metamodelica::Real>>;
            let mut B: metamodelica::List<metamodelica::List<metamodelica::Real>>;
            let mut arg_out: metamodelica::List<DAE::ExtArg>;
            let mut val_out: metamodelica::List<metamodelica::Ref<Values::Value>>;
            (M, cache) = evaluateExtIntArg(metamodelica::AsArg::as_arg(&arg_M), cache, env.clone())?;
            (N, cache) = evaluateExtIntArg(metamodelica::AsArg::as_arg(&arg_N), cache, env.clone())?;
            (NRHS, cache) = evaluateExtIntArg(metamodelica::AsArg::as_arg(&arg_NRHS), cache, env.clone())?;
            (A, cache) = evaluateExtRealMatrixArg(metamodelica::AsArg::as_arg(&arg_A), cache, env.clone())?;
            (LDA, cache) = evaluateExtIntArg(metamodelica::AsArg::as_arg(&arg_LDA), cache, env.clone())?;
            (B, cache) = evaluateExtRealMatrixArg(metamodelica::AsArg::as_arg(&arg_B), cache, env.clone())?;
            (LDB, cache) = evaluateExtIntArg(metamodelica::AsArg::as_arg(&arg_LDB), cache, env.clone())?;
            (JPVT, cache) = evaluateExtIntArrayArg(metamodelica::AsArg::as_arg(&arg_JPVT), cache, env.clone())?;
            (RCOND, cache) = evaluateExtRealArg(metamodelica::AsArg::as_arg(&arg_RCOND), cache, env.clone())?;
            (WORK, cache) = evaluateExtRealArrayArg(metamodelica::AsArg::as_arg(&arg_WORK), cache, env.clone())?;
            (LWORK, cache) = evaluateExtIntArg(metamodelica::AsArg::as_arg(&arg_LWORK), cache, env.clone())?;
            (A, B, JPVT, RANK, WORK, INFO) = Lapack::dgelsy(M, N, NRHS, A, LDA, B, LDB, JPVT, RCOND, WORK, LWORK);
            val_A = ValuesMake::makeRealMatrix(A)?;
            val_B = ValuesMake::makeRealMatrix(B)?;
            val_JPVT = ValuesMake::makeIntArray(JPVT)?;
            val_RANK = ValuesMake::makeInteger(RANK);
            val_WORK = ValuesMake::makeRealArray(WORK)?;
            val_INFO = ValuesMake::makeInteger(INFO);
            arg_out = list![arg_A.clone(), arg_B.clone(), arg_JPVT.clone(), arg_RANK.clone(), arg_WORK.clone(), arg_INFO.clone()];
            val_out = list![val_A, val_B, val_JPVT, val_RANK, val_WORK, val_INFO];
            (cache, env) = assignExtOutputs(arg_out, val_out, cache, env)?;
            (cache, env)
        },
        (Deref @ "dgesv", Deref @ metamodelica::ListNode::Cons { head: arg_N, tail: Deref @ metamodelica::ListNode::Cons { head: arg_NRHS, tail: Deref @ metamodelica::ListNode::Cons { head: arg_A, tail: Deref @ metamodelica::ListNode::Cons { head: arg_LDA, tail: Deref @ metamodelica::ListNode::Cons { head: arg_IPIV, tail: Deref @ metamodelica::ListNode::Cons { head: arg_B, tail: Deref @ metamodelica::ListNode::Cons { head: arg_LDB, tail: Deref @ metamodelica::ListNode::Cons { head: arg_INFO, tail: Deref @ metamodelica::ListNode::Nil } } } } } } } }) => {
            let mut cache = inCache;
            let mut env = inEnv;
            let mut val_INFO: metamodelica::Ref<Values::Value>;
            let mut val_IPIV: metamodelica::Ref<Values::Value>;
            let mut val_A: metamodelica::Ref<Values::Value>;
            let mut val_B: metamodelica::Ref<Values::Value>;
            let mut INFO: i32;
            let mut LDA: i32;
            let mut LDB: i32;
            let mut N: i32;
            let mut NRHS: i32;
            let mut IPIV: metamodelica::List<i32>;
            let mut A: metamodelica::List<metamodelica::List<metamodelica::Real>>;
            let mut B: metamodelica::List<metamodelica::List<metamodelica::Real>>;
            let mut arg_out: metamodelica::List<DAE::ExtArg>;
            let mut val_out: metamodelica::List<metamodelica::Ref<Values::Value>>;
            (N, cache) = evaluateExtIntArg(metamodelica::AsArg::as_arg(&arg_N), cache, env.clone())?;
            (NRHS, cache) = evaluateExtIntArg(metamodelica::AsArg::as_arg(&arg_NRHS), cache, env.clone())?;
            (A, cache) = evaluateExtRealMatrixArg(metamodelica::AsArg::as_arg(&arg_A), cache, env.clone())?;
            (LDA, cache) = evaluateExtIntArg(metamodelica::AsArg::as_arg(&arg_LDA), cache, env.clone())?;
            (B, cache) = evaluateExtRealMatrixArg(metamodelica::AsArg::as_arg(&arg_B), cache, env.clone())?;
            (LDB, cache) = evaluateExtIntArg(metamodelica::AsArg::as_arg(&arg_LDB), cache, env.clone())?;
            (A, IPIV, B, INFO) = Lapack::dgesv(N, NRHS, A, LDA, B, LDB);
            val_A = ValuesMake::makeRealMatrix(A)?;
            val_IPIV = ValuesMake::makeIntArray(IPIV)?;
            val_B = ValuesMake::makeRealMatrix(B)?;
            val_INFO = ValuesMake::makeInteger(INFO);
            arg_out = list![arg_A.clone(), arg_IPIV.clone(), arg_B.clone(), arg_INFO.clone()];
            val_out = list![val_A, val_IPIV, val_B, val_INFO];
            (cache, env) = assignExtOutputs(arg_out, val_out, cache, env)?;
            (cache, env)
        },
        (Deref @ "dgglse", Deref @ metamodelica::ListNode::Cons { head: arg_M, tail: Deref @ metamodelica::ListNode::Cons { head: arg_N, tail: Deref @ metamodelica::ListNode::Cons { head: arg_P, tail: Deref @ metamodelica::ListNode::Cons { head: arg_A, tail: Deref @ metamodelica::ListNode::Cons { head: arg_LDA, tail: Deref @ metamodelica::ListNode::Cons { head: arg_B, tail: Deref @ metamodelica::ListNode::Cons { head: arg_LDB, tail: Deref @ metamodelica::ListNode::Cons { head: arg_C, tail: Deref @ metamodelica::ListNode::Cons { head: arg_D, tail: Deref @ metamodelica::ListNode::Cons { head: arg_X, tail: Deref @ metamodelica::ListNode::Cons { head: arg_WORK, tail: Deref @ metamodelica::ListNode::Cons { head: arg_LWORK, tail: Deref @ metamodelica::ListNode::Cons { head: arg_INFO, tail: Deref @ metamodelica::ListNode::Nil } } } } } } } } } } } } }) => {
            let mut cache = inCache;
            let mut env = inEnv;
            let mut val_INFO: metamodelica::Ref<Values::Value>;
            let mut val_C: metamodelica::Ref<Values::Value>;
            let mut val_D: metamodelica::Ref<Values::Value>;
            let mut val_WORK: metamodelica::Ref<Values::Value>;
            let mut val_X: metamodelica::Ref<Values::Value>;
            let mut val_A: metamodelica::Ref<Values::Value>;
            let mut val_B: metamodelica::Ref<Values::Value>;
            let mut INFO: i32;
            let mut LDA: i32;
            let mut LDB: i32;
            let mut LWORK: i32;
            let mut M: i32;
            let mut N: i32;
            let mut P: i32;
            let mut C: metamodelica::List<metamodelica::Real>;
            let mut D: metamodelica::List<metamodelica::Real>;
            let mut WORK: metamodelica::List<metamodelica::Real>;
            let mut X: metamodelica::List<metamodelica::Real>;
            let mut A: metamodelica::List<metamodelica::List<metamodelica::Real>>;
            let mut B: metamodelica::List<metamodelica::List<metamodelica::Real>>;
            let mut arg_out: metamodelica::List<DAE::ExtArg>;
            let mut val_out: metamodelica::List<metamodelica::Ref<Values::Value>>;
            (M, cache) = evaluateExtIntArg(metamodelica::AsArg::as_arg(&arg_M), cache, env.clone())?;
            (N, cache) = evaluateExtIntArg(metamodelica::AsArg::as_arg(&arg_N), cache, env.clone())?;
            (P, cache) = evaluateExtIntArg(metamodelica::AsArg::as_arg(&arg_P), cache, env.clone())?;
            (A, cache) = evaluateExtRealMatrixArg(metamodelica::AsArg::as_arg(&arg_A), cache, env.clone())?;
            (LDA, cache) = evaluateExtIntArg(metamodelica::AsArg::as_arg(&arg_LDA), cache, env.clone())?;
            (B, cache) = evaluateExtRealMatrixArg(metamodelica::AsArg::as_arg(&arg_B), cache, env.clone())?;
            (LDB, cache) = evaluateExtIntArg(metamodelica::AsArg::as_arg(&arg_LDB), cache, env.clone())?;
            (C, cache) = evaluateExtRealArrayArg(metamodelica::AsArg::as_arg(&arg_C), cache, env.clone())?;
            (D, cache) = evaluateExtRealArrayArg(metamodelica::AsArg::as_arg(&arg_D), cache, env.clone())?;
            (WORK, cache) = evaluateExtRealArrayArg(metamodelica::AsArg::as_arg(&arg_WORK), cache, env.clone())?;
            (LWORK, cache) = evaluateExtIntArg(metamodelica::AsArg::as_arg(&arg_LWORK), cache, env.clone())?;
            (A, B, C, D, X, WORK, INFO) = Lapack::dgglse(M, N, P, A, LDA, B, LDB, C, D, WORK, LWORK);
            val_A = ValuesMake::makeRealMatrix(A)?;
            val_B = ValuesMake::makeRealMatrix(B)?;
            val_C = ValuesMake::makeRealArray(C)?;
            val_D = ValuesMake::makeRealArray(D)?;
            val_X = ValuesMake::makeRealArray(X)?;
            val_WORK = ValuesMake::makeRealArray(WORK)?;
            val_INFO = ValuesMake::makeInteger(INFO);
            arg_out = list![arg_A.clone(), arg_B.clone(), arg_C.clone(), arg_D.clone(), arg_X.clone(), arg_WORK.clone(), arg_INFO.clone()];
            val_out = list![val_A, val_B, val_C, val_D, val_X, val_WORK, val_INFO];
            (cache, env) = assignExtOutputs(arg_out, val_out, cache, env)?;
            (cache, env)
        },
        (Deref @ "dgtsv", Deref @ metamodelica::ListNode::Cons { head: arg_N, tail: Deref @ metamodelica::ListNode::Cons { head: arg_NRHS, tail: Deref @ metamodelica::ListNode::Cons { head: arg_DL, tail: Deref @ metamodelica::ListNode::Cons { head: arg_D, tail: Deref @ metamodelica::ListNode::Cons { head: arg_DU, tail: Deref @ metamodelica::ListNode::Cons { head: arg_B, tail: Deref @ metamodelica::ListNode::Cons { head: arg_LDB, tail: Deref @ metamodelica::ListNode::Cons { head: arg_INFO, tail: Deref @ metamodelica::ListNode::Nil } } } } } } } }) => {
            let mut cache = inCache;
            let mut env = inEnv;
            let mut val_INFO: metamodelica::Ref<Values::Value>;
            let mut val_D: metamodelica::Ref<Values::Value>;
            let mut val_DL: metamodelica::Ref<Values::Value>;
            let mut val_DU: metamodelica::Ref<Values::Value>;
            let mut val_B: metamodelica::Ref<Values::Value>;
            let mut INFO: i32;
            let mut LDB: i32;
            let mut N: i32;
            let mut NRHS: i32;
            let mut D: metamodelica::List<metamodelica::Real>;
            let mut DL: metamodelica::List<metamodelica::Real>;
            let mut DU: metamodelica::List<metamodelica::Real>;
            let mut B: metamodelica::List<metamodelica::List<metamodelica::Real>>;
            let mut arg_out: metamodelica::List<DAE::ExtArg>;
            let mut val_out: metamodelica::List<metamodelica::Ref<Values::Value>>;
            (N, cache) = evaluateExtIntArg(metamodelica::AsArg::as_arg(&arg_N), cache, env.clone())?;
            (NRHS, cache) = evaluateExtIntArg(metamodelica::AsArg::as_arg(&arg_NRHS), cache, env.clone())?;
            (DL, cache) = evaluateExtRealArrayArg(metamodelica::AsArg::as_arg(&arg_DL), cache, env.clone())?;
            (D, cache) = evaluateExtRealArrayArg(metamodelica::AsArg::as_arg(&arg_D), cache, env.clone())?;
            (DU, cache) = evaluateExtRealArrayArg(metamodelica::AsArg::as_arg(&arg_DU), cache, env.clone())?;
            (B, cache) = evaluateExtRealMatrixArg(metamodelica::AsArg::as_arg(&arg_B), cache, env.clone())?;
            (LDB, cache) = evaluateExtIntArg(metamodelica::AsArg::as_arg(&arg_LDB), cache, env.clone())?;
            (DL, D, DU, B, INFO) = Lapack::dgtsv(N, NRHS, DL, D, DU, B, LDB);
            val_DL = ValuesMake::makeRealArray(DL)?;
            val_D = ValuesMake::makeRealArray(D)?;
            val_DU = ValuesMake::makeRealArray(DU)?;
            val_B = ValuesMake::makeRealMatrix(B)?;
            val_INFO = ValuesMake::makeInteger(INFO);
            arg_out = list![arg_DL.clone(), arg_D.clone(), arg_DU.clone(), arg_B.clone(), arg_INFO.clone()];
            val_out = list![val_DL, val_D, val_DU, val_B, val_INFO];
            (cache, env) = assignExtOutputs(arg_out, val_out, cache, env)?;
            (cache, env)
        },
        (Deref @ "dgbsv", Deref @ metamodelica::ListNode::Cons { head: arg_N, tail: Deref @ metamodelica::ListNode::Cons { head: arg_KL, tail: Deref @ metamodelica::ListNode::Cons { head: arg_KU, tail: Deref @ metamodelica::ListNode::Cons { head: arg_NRHS, tail: Deref @ metamodelica::ListNode::Cons { head: arg_AB, tail: Deref @ metamodelica::ListNode::Cons { head: arg_LDAB, tail: Deref @ metamodelica::ListNode::Cons { head: arg_IPIV, tail: Deref @ metamodelica::ListNode::Cons { head: arg_B, tail: Deref @ metamodelica::ListNode::Cons { head: arg_LDB, tail: Deref @ metamodelica::ListNode::Cons { head: arg_INFO, tail: Deref @ metamodelica::ListNode::Nil } } } } } } } } } }) => {
            let mut cache = inCache;
            let mut env = inEnv;
            let mut val_INFO: metamodelica::Ref<Values::Value>;
            let mut val_IPIV: metamodelica::Ref<Values::Value>;
            let mut val_AB: metamodelica::Ref<Values::Value>;
            let mut val_B: metamodelica::Ref<Values::Value>;
            let mut INFO: i32;
            let mut KL: i32;
            let mut KU: i32;
            let mut LDAB: i32;
            let mut LDB: i32;
            let mut N: i32;
            let mut NRHS: i32;
            let mut IPIV: metamodelica::List<i32>;
            let mut AB: metamodelica::List<metamodelica::List<metamodelica::Real>>;
            let mut B: metamodelica::List<metamodelica::List<metamodelica::Real>>;
            let mut arg_out: metamodelica::List<DAE::ExtArg>;
            let mut val_out: metamodelica::List<metamodelica::Ref<Values::Value>>;
            (N, cache) = evaluateExtIntArg(metamodelica::AsArg::as_arg(&arg_N), cache, env.clone())?;
            (KL, cache) = evaluateExtIntArg(metamodelica::AsArg::as_arg(&arg_KL), cache, env.clone())?;
            (KU, cache) = evaluateExtIntArg(metamodelica::AsArg::as_arg(&arg_KU), cache, env.clone())?;
            (NRHS, cache) = evaluateExtIntArg(metamodelica::AsArg::as_arg(&arg_NRHS), cache, env.clone())?;
            (AB, cache) = evaluateExtRealMatrixArg(metamodelica::AsArg::as_arg(&arg_AB), cache, env.clone())?;
            (LDAB, cache) = evaluateExtIntArg(metamodelica::AsArg::as_arg(&arg_LDAB), cache, env.clone())?;
            (B, cache) = evaluateExtRealMatrixArg(metamodelica::AsArg::as_arg(&arg_B), cache, env.clone())?;
            (LDB, cache) = evaluateExtIntArg(metamodelica::AsArg::as_arg(&arg_LDB), cache, env.clone())?;
            (AB, IPIV, B, INFO) = Lapack::dgbsv(N, KL, KU, NRHS, AB, LDAB, B, LDB);
            val_AB = ValuesMake::makeRealMatrix(AB)?;
            val_IPIV = ValuesMake::makeIntArray(IPIV)?;
            val_B = ValuesMake::makeRealMatrix(B)?;
            val_INFO = ValuesMake::makeInteger(INFO);
            arg_out = list![arg_AB.clone(), arg_IPIV.clone(), arg_B.clone(), arg_INFO.clone()];
            val_out = list![val_AB, val_IPIV, val_B, val_INFO];
            (cache, env) = assignExtOutputs(arg_out, val_out, cache, env)?;
            (cache, env)
        },
        (Deref @ "dgesvd", Deref @ metamodelica::ListNode::Cons { head: arg_JOBU, tail: Deref @ metamodelica::ListNode::Cons { head: arg_JOBVT, tail: Deref @ metamodelica::ListNode::Cons { head: arg_M, tail: Deref @ metamodelica::ListNode::Cons { head: arg_N, tail: Deref @ metamodelica::ListNode::Cons { head: arg_A, tail: Deref @ metamodelica::ListNode::Cons { head: arg_LDA, tail: Deref @ metamodelica::ListNode::Cons { head: arg_S, tail: Deref @ metamodelica::ListNode::Cons { head: arg_U, tail: Deref @ metamodelica::ListNode::Cons { head: arg_LDU, tail: Deref @ metamodelica::ListNode::Cons { head: arg_VT, tail: Deref @ metamodelica::ListNode::Cons { head: arg_LDVT, tail: Deref @ metamodelica::ListNode::Cons { head: arg_WORK, tail: Deref @ metamodelica::ListNode::Cons { head: arg_LWORK, tail: Deref @ metamodelica::ListNode::Cons { head: arg_INFO, tail: Deref @ metamodelica::ListNode::Nil } } } } } } } } } } } } } }) => {
            let mut cache = inCache;
            let mut env = inEnv;
            let mut val_INFO: metamodelica::Ref<Values::Value>;
            let mut val_WORK: metamodelica::Ref<Values::Value>;
            let mut val_A: metamodelica::Ref<Values::Value>;
            let mut val_S: metamodelica::Ref<Values::Value>;
            let mut val_U: metamodelica::Ref<Values::Value>;
            let mut val_VT: metamodelica::Ref<Values::Value>;
            let mut INFO: i32;
            let mut LDA: i32;
            let mut LDU: i32;
            let mut LDVT: i32;
            let mut LWORK: i32;
            let mut M: i32;
            let mut N: i32;
            let mut JOBU: ArcStr;
            let mut JOBVT: ArcStr;
            let mut WORK: metamodelica::List<metamodelica::Real>;
            let mut S: metamodelica::List<metamodelica::Real>;
            let mut A: metamodelica::List<metamodelica::List<metamodelica::Real>>;
            let mut U: metamodelica::List<metamodelica::List<metamodelica::Real>>;
            let mut VT: metamodelica::List<metamodelica::List<metamodelica::Real>>;
            let mut arg_out: metamodelica::List<DAE::ExtArg>;
            let mut val_out: metamodelica::List<metamodelica::Ref<Values::Value>>;
            (JOBU, cache) = evaluateExtStringArg(metamodelica::AsArg::as_arg(&arg_JOBU), cache, env.clone())?;
            (JOBVT, cache) = evaluateExtStringArg(metamodelica::AsArg::as_arg(&arg_JOBVT), cache, env.clone())?;
            (M, cache) = evaluateExtIntArg(metamodelica::AsArg::as_arg(&arg_M), cache, env.clone())?;
            (N, cache) = evaluateExtIntArg(metamodelica::AsArg::as_arg(&arg_N), cache, env.clone())?;
            (A, cache) = evaluateExtRealMatrixArg(metamodelica::AsArg::as_arg(&arg_A), cache, env.clone())?;
            (LDA, cache) = evaluateExtIntArg(metamodelica::AsArg::as_arg(&arg_LDA), cache, env.clone())?;
            (LDU, cache) = evaluateExtIntArg(metamodelica::AsArg::as_arg(&arg_LDU), cache, env.clone())?;
            (LDVT, cache) = evaluateExtIntArg(metamodelica::AsArg::as_arg(&arg_LDVT), cache, env.clone())?;
            (WORK, cache) = evaluateExtRealArrayArg(metamodelica::AsArg::as_arg(&arg_WORK), cache, env.clone())?;
            (LWORK, cache) = evaluateExtIntArg(metamodelica::AsArg::as_arg(&arg_LWORK), cache, env.clone())?;
            (A, S, U, VT, WORK, INFO) = Lapack::dgesvd(JOBU, JOBVT, M, N, A, LDA, LDU, LDVT, WORK, LWORK);
            val_A = ValuesMake::makeRealMatrix(A)?;
            val_S = ValuesMake::makeRealArray(S)?;
            val_U = ValuesMake::makeRealMatrix(U)?;
            val_VT = ValuesMake::makeRealMatrix(VT)?;
            val_WORK = ValuesMake::makeRealArray(WORK)?;
            val_INFO = ValuesMake::makeInteger(INFO);
            arg_out = list![arg_A.clone(), arg_S.clone(), arg_U.clone(), arg_VT.clone(), arg_WORK.clone(), arg_INFO.clone()];
            val_out = list![val_A, val_S, val_U, val_VT, val_WORK, val_INFO];
            (cache, env) = assignExtOutputs(arg_out, val_out, cache, env)?;
            (cache, env)
        },
        (Deref @ "dgetrf", Deref @ metamodelica::ListNode::Cons { head: arg_M, tail: Deref @ metamodelica::ListNode::Cons { head: arg_N, tail: Deref @ metamodelica::ListNode::Cons { head: arg_A, tail: Deref @ metamodelica::ListNode::Cons { head: arg_LDA, tail: Deref @ metamodelica::ListNode::Cons { head: arg_IPIV, tail: Deref @ metamodelica::ListNode::Cons { head: arg_INFO, tail: Deref @ metamodelica::ListNode::Nil } } } } } }) => {
            let mut cache = inCache;
            let mut env = inEnv;
            let mut val_INFO: metamodelica::Ref<Values::Value>;
            let mut val_IPIV: metamodelica::Ref<Values::Value>;
            let mut val_A: metamodelica::Ref<Values::Value>;
            let mut INFO: i32;
            let mut LDA: i32;
            let mut M: i32;
            let mut N: i32;
            let mut IPIV: metamodelica::List<i32>;
            let mut A: metamodelica::List<metamodelica::List<metamodelica::Real>>;
            let mut arg_out: metamodelica::List<DAE::ExtArg>;
            let mut val_out: metamodelica::List<metamodelica::Ref<Values::Value>>;
            (M, cache) = evaluateExtIntArg(metamodelica::AsArg::as_arg(&arg_M), cache, env.clone())?;
            (N, cache) = evaluateExtIntArg(metamodelica::AsArg::as_arg(&arg_N), cache, env.clone())?;
            (A, cache) = evaluateExtRealMatrixArg(metamodelica::AsArg::as_arg(&arg_A), cache, env.clone())?;
            (LDA, cache) = evaluateExtIntArg(metamodelica::AsArg::as_arg(&arg_LDA), cache, env.clone())?;
            (A, IPIV, INFO) = Lapack::dgetrf(M, N, A, LDA);
            val_A = ValuesMake::makeRealMatrix(A)?;
            val_IPIV = ValuesMake::makeIntArray(IPIV)?;
            val_INFO = ValuesMake::makeInteger(INFO);
            arg_out = list![arg_A.clone(), arg_IPIV.clone(), arg_INFO.clone()];
            val_out = list![val_A, val_IPIV, val_INFO];
            (cache, env) = assignExtOutputs(arg_out, val_out, cache, env)?;
            (cache, env)
        },
        (Deref @ "dgetrs", Deref @ metamodelica::ListNode::Cons { head: arg_TRANS, tail: Deref @ metamodelica::ListNode::Cons { head: arg_N, tail: Deref @ metamodelica::ListNode::Cons { head: arg_NRHS, tail: Deref @ metamodelica::ListNode::Cons { head: arg_A, tail: Deref @ metamodelica::ListNode::Cons { head: arg_LDA, tail: Deref @ metamodelica::ListNode::Cons { head: arg_IPIV, tail: Deref @ metamodelica::ListNode::Cons { head: arg_B, tail: Deref @ metamodelica::ListNode::Cons { head: arg_LDB, tail: Deref @ metamodelica::ListNode::Cons { head: arg_INFO, tail: Deref @ metamodelica::ListNode::Nil } } } } } } } } }) => {
            let mut cache = inCache;
            let mut env = inEnv;
            let mut val_INFO: metamodelica::Ref<Values::Value>;
            let mut val_B: metamodelica::Ref<Values::Value>;
            let mut INFO: i32;
            let mut LDA: i32;
            let mut LDB: i32;
            let mut N: i32;
            let mut NRHS: i32;
            let mut TRANS: ArcStr;
            let mut IPIV: metamodelica::List<i32>;
            let mut A: metamodelica::List<metamodelica::List<metamodelica::Real>>;
            let mut B: metamodelica::List<metamodelica::List<metamodelica::Real>>;
            let mut arg_out: metamodelica::List<DAE::ExtArg>;
            let mut val_out: metamodelica::List<metamodelica::Ref<Values::Value>>;
            (TRANS, cache) = evaluateExtStringArg(metamodelica::AsArg::as_arg(&arg_TRANS), cache, env.clone())?;
            (N, cache) = evaluateExtIntArg(metamodelica::AsArg::as_arg(&arg_N), cache, env.clone())?;
            (NRHS, cache) = evaluateExtIntArg(metamodelica::AsArg::as_arg(&arg_NRHS), cache, env.clone())?;
            (A, cache) = evaluateExtRealMatrixArg(metamodelica::AsArg::as_arg(&arg_A), cache, env.clone())?;
            (LDA, cache) = evaluateExtIntArg(metamodelica::AsArg::as_arg(&arg_LDA), cache, env.clone())?;
            (IPIV, cache) = evaluateExtIntArrayArg(metamodelica::AsArg::as_arg(&arg_IPIV), cache, env.clone())?;
            (B, cache) = evaluateExtRealMatrixArg(metamodelica::AsArg::as_arg(&arg_B), cache, env.clone())?;
            (LDB, cache) = evaluateExtIntArg(metamodelica::AsArg::as_arg(&arg_LDB), cache, env.clone())?;
            (B, INFO) = Lapack::dgetrs(TRANS, N, NRHS, A, LDA, IPIV, B, LDB);
            val_B = ValuesMake::makeRealMatrix(B)?;
            val_INFO = ValuesMake::makeInteger(INFO);
            arg_out = list![arg_B.clone(), arg_INFO.clone()];
            val_out = list![val_B, val_INFO];
            (cache, env) = assignExtOutputs(arg_out, val_out, cache, env)?;
            (cache, env)
        },
        (Deref @ "dgetri", Deref @ metamodelica::ListNode::Cons { head: arg_N, tail: Deref @ metamodelica::ListNode::Cons { head: arg_A, tail: Deref @ metamodelica::ListNode::Cons { head: arg_LDA, tail: Deref @ metamodelica::ListNode::Cons { head: arg_IPIV, tail: Deref @ metamodelica::ListNode::Cons { head: arg_WORK, tail: Deref @ metamodelica::ListNode::Cons { head: arg_LWORK, tail: Deref @ metamodelica::ListNode::Cons { head: arg_INFO, tail: Deref @ metamodelica::ListNode::Nil } } } } } } }) => {
            let mut cache = inCache;
            let mut env = inEnv;
            let mut val_INFO: metamodelica::Ref<Values::Value>;
            let mut val_WORK: metamodelica::Ref<Values::Value>;
            let mut val_A: metamodelica::Ref<Values::Value>;
            let mut INFO: i32;
            let mut LDA: i32;
            let mut LWORK: i32;
            let mut N: i32;
            let mut IPIV: metamodelica::List<i32>;
            let mut WORK: metamodelica::List<metamodelica::Real>;
            let mut A: metamodelica::List<metamodelica::List<metamodelica::Real>>;
            let mut arg_out: metamodelica::List<DAE::ExtArg>;
            let mut val_out: metamodelica::List<metamodelica::Ref<Values::Value>>;
            (N, cache) = evaluateExtIntArg(metamodelica::AsArg::as_arg(&arg_N), cache, env.clone())?;
            (A, cache) = evaluateExtRealMatrixArg(metamodelica::AsArg::as_arg(&arg_A), cache, env.clone())?;
            (LDA, cache) = evaluateExtIntArg(metamodelica::AsArg::as_arg(&arg_LDA), cache, env.clone())?;
            (IPIV, cache) = evaluateExtIntArrayArg(metamodelica::AsArg::as_arg(&arg_IPIV), cache, env.clone())?;
            (WORK, cache) = evaluateExtRealArrayArg(metamodelica::AsArg::as_arg(&arg_WORK), cache, env.clone())?;
            (LWORK, cache) = evaluateExtIntArg(metamodelica::AsArg::as_arg(&arg_LWORK), cache, env.clone())?;
            (A, WORK, INFO) = Lapack::dgetri(N, A, LDA, IPIV, WORK, LWORK);
            val_A = ValuesMake::makeRealMatrix(A)?;
            val_WORK = ValuesMake::makeRealArray(WORK)?;
            val_INFO = ValuesMake::makeInteger(INFO);
            arg_out = list![arg_A.clone(), arg_WORK.clone(), arg_INFO.clone()];
            val_out = list![val_A, val_WORK, val_INFO];
            (cache, env) = assignExtOutputs(arg_out, val_out, cache, env)?;
            (cache, env)
        },
        (Deref @ "dgeqpf", Deref @ metamodelica::ListNode::Cons { head: arg_M, tail: Deref @ metamodelica::ListNode::Cons { head: arg_N, tail: Deref @ metamodelica::ListNode::Cons { head: arg_A, tail: Deref @ metamodelica::ListNode::Cons { head: arg_LDA, tail: Deref @ metamodelica::ListNode::Cons { head: arg_JPVT, tail: Deref @ metamodelica::ListNode::Cons { head: arg_TAU, tail: Deref @ metamodelica::ListNode::Cons { head: arg_WORK, tail: Deref @ metamodelica::ListNode::Cons { head: arg_INFO, tail: Deref @ metamodelica::ListNode::Nil } } } } } } } }) => {
            let mut cache = inCache;
            let mut env = inEnv;
            let mut val_INFO: metamodelica::Ref<Values::Value>;
            let mut val_JPVT: metamodelica::Ref<Values::Value>;
            let mut val_TAU: metamodelica::Ref<Values::Value>;
            let mut val_A: metamodelica::Ref<Values::Value>;
            let mut INFO: i32;
            let mut LDA: i32;
            let mut M: i32;
            let mut N: i32;
            let mut JPVT: metamodelica::List<i32>;
            let mut TAU: metamodelica::List<metamodelica::Real>;
            let mut WORK: metamodelica::List<metamodelica::Real>;
            let mut A: metamodelica::List<metamodelica::List<metamodelica::Real>>;
            let mut arg_out: metamodelica::List<DAE::ExtArg>;
            let mut val_out: metamodelica::List<metamodelica::Ref<Values::Value>>;
            (M, cache) = evaluateExtIntArg(metamodelica::AsArg::as_arg(&arg_M), cache, env.clone())?;
            (N, cache) = evaluateExtIntArg(metamodelica::AsArg::as_arg(&arg_N), cache, env.clone())?;
            (A, cache) = evaluateExtRealMatrixArg(metamodelica::AsArg::as_arg(&arg_A), cache, env.clone())?;
            (LDA, cache) = evaluateExtIntArg(metamodelica::AsArg::as_arg(&arg_LDA), cache, env.clone())?;
            (JPVT, cache) = evaluateExtIntArrayArg(metamodelica::AsArg::as_arg(&arg_JPVT), cache, env.clone())?;
            (WORK, cache) = evaluateExtRealArrayArg(metamodelica::AsArg::as_arg(&arg_WORK), cache, env.clone())?;
            (A, JPVT, TAU, INFO) = Lapack::dgeqpf(M, N, A, LDA, JPVT, WORK);
            val_A = ValuesMake::makeRealMatrix(A)?;
            val_JPVT = ValuesMake::makeIntArray(JPVT)?;
            val_TAU = ValuesMake::makeRealArray(TAU)?;
            val_INFO = ValuesMake::makeInteger(INFO);
            arg_out = list![arg_A.clone(), arg_JPVT.clone(), arg_TAU.clone(), arg_INFO.clone()];
            val_out = list![val_A, val_JPVT, val_TAU, val_INFO];
            (cache, env) = assignExtOutputs(arg_out, val_out, cache, env)?;
            (cache, env)
        },
        (Deref @ "dorgqr", Deref @ metamodelica::ListNode::Cons { head: arg_M, tail: Deref @ metamodelica::ListNode::Cons { head: arg_N, tail: Deref @ metamodelica::ListNode::Cons { head: arg_K, tail: Deref @ metamodelica::ListNode::Cons { head: arg_A, tail: Deref @ metamodelica::ListNode::Cons { head: arg_LDA, tail: Deref @ metamodelica::ListNode::Cons { head: arg_TAU, tail: Deref @ metamodelica::ListNode::Cons { head: arg_WORK, tail: Deref @ metamodelica::ListNode::Cons { head: arg_LWORK, tail: Deref @ metamodelica::ListNode::Cons { head: arg_INFO, tail: Deref @ metamodelica::ListNode::Nil } } } } } } } } }) => {
            let mut cache = inCache;
            let mut env = inEnv;
            let mut val_INFO: metamodelica::Ref<Values::Value>;
            let mut val_WORK: metamodelica::Ref<Values::Value>;
            let mut val_A: metamodelica::Ref<Values::Value>;
            let mut INFO: i32;
            let mut K: i32;
            let mut LDA: i32;
            let mut LWORK: i32;
            let mut M: i32;
            let mut N: i32;
            let mut TAU: metamodelica::List<metamodelica::Real>;
            let mut WORK: metamodelica::List<metamodelica::Real>;
            let mut A: metamodelica::List<metamodelica::List<metamodelica::Real>>;
            let mut arg_out: metamodelica::List<DAE::ExtArg>;
            let mut val_out: metamodelica::List<metamodelica::Ref<Values::Value>>;
            (M, cache) = evaluateExtIntArg(metamodelica::AsArg::as_arg(&arg_M), cache, env.clone())?;
            (N, cache) = evaluateExtIntArg(metamodelica::AsArg::as_arg(&arg_N), cache, env.clone())?;
            (K, cache) = evaluateExtIntArg(metamodelica::AsArg::as_arg(&arg_K), cache, env.clone())?;
            (A, cache) = evaluateExtRealMatrixArg(metamodelica::AsArg::as_arg(&arg_A), cache, env.clone())?;
            (LDA, cache) = evaluateExtIntArg(metamodelica::AsArg::as_arg(&arg_LDA), cache, env.clone())?;
            (TAU, cache) = evaluateExtRealArrayArg(metamodelica::AsArg::as_arg(&arg_TAU), cache, env.clone())?;
            (WORK, cache) = evaluateExtRealArrayArg(metamodelica::AsArg::as_arg(&arg_WORK), cache, env.clone())?;
            (LWORK, cache) = evaluateExtIntArg(metamodelica::AsArg::as_arg(&arg_LWORK), cache, env.clone())?;
            (A, WORK, INFO) = Lapack::dorgqr(M, N, K, A, LDA, TAU, WORK, LWORK);
            val_A = ValuesMake::makeRealMatrix(A)?;
            val_WORK = ValuesMake::makeRealArray(WORK)?;
            val_INFO = ValuesMake::makeInteger(INFO);
            arg_out = list![arg_A.clone(), arg_WORK.clone(), arg_INFO.clone()];
            val_out = list![val_A, val_WORK, val_INFO];
            (cache, env) = assignExtOutputs(arg_out, val_out, cache, env)?;
            (cache, env)
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok((outCache, outEnv))
}

fn evaluateElements(
    mut inElements: metamodelica::List<metamodelica::Ref<DAE::Element>>,
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
    mut inLoopControl: LoopControl,
) -> Result<(FCore::Cache, FCore::Graph, LoopControl)> {
    '__tco: loop {
        ::match_deref::match_deref! { match &((inElements, inLoopControl)) {
            (_, LoopControl::RETURN { .. }) => {
                return Ok((inCache, inEnv, inLoopControl))
            },
            (Deref @ metamodelica::ListNode::Nil, _) => {
                return Ok((inCache, inEnv, crate::CevalFunction::LoopControl::NEXT))
            },
            (Deref @ metamodelica::ListNode::Cons { head: elem, tail: rest_elems }, _) => {
                let mut cache: FCore::Cache;
                let mut env: FCore::Graph;
                let mut loop_ctrl: LoopControl;
                (cache, env, loop_ctrl) = evaluateElement(metamodelica::AsArg::as_arg(&elem), inCache, inEnv)?;
                { (inElements, inCache, inEnv, inLoopControl) = (rest_elems.clone(), cache, env, loop_ctrl); continue '__tco; }
            },
            _ => return Err("match: no arm matched"),
        } }
    }
}

fn evaluateElement(
    mut inElement: &metamodelica::Ref<DAE::Element>,
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
) -> Result<(FCore::Cache, FCore::Graph, LoopControl)> {
    let mut outCache: FCore::Cache;
    let mut outEnv: FCore::Graph;
    let mut outLoopControl: LoopControl;
    (outCache, outEnv, outLoopControl) = (::match_deref::match_deref! { match inElement {
        Deref @ DAE::Element::ALGORITHM { algorithm_: Deref @ DAE::Algorithm { statementLst: sl }, .. } => {
            let mut cache: FCore::Cache;
            let mut env: FCore::Graph;
            let mut loop_ctrl: LoopControl;
            let mut sl = (*sl).clone();
            let (__pa0, (_, __pa1)) = DAEUtil::traverseDAEEquationsStmts(sl.clone(), (std::sync::Arc::new(Expression::traverseSubexpressionsHelper) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>, _) -> Result<_> + 'static>), ((std::sync::Arc::new(optimizeExpTraverser) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>, FCore::Graph) -> Result<(metamodelica::Ref<DAE::Exp>, FCore::Graph)> + 'static>), inEnv))?;
            sl = metamodelica::Own::own(__pa0);
            env = metamodelica::Own::own(__pa1);
            (cache, env, loop_ctrl) = evaluateStatements(sl.clone(), inCache, env)?;
            (cache, env, loop_ctrl)
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok((outCache, outEnv, outLoopControl))
}

fn evaluateStatement(
    mut inStatement: metamodelica::Ref<DAE::Statement>,
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
) -> Result<(FCore::Cache, FCore::Graph, LoopControl)> {
    let mut outCache: FCore::Cache;
    let mut outEnv: FCore::Graph;
    let mut outLoopControl: LoopControl;
    (outCache, outEnv, outLoopControl) = (::match_deref::match_deref! { match &(inStatement.clone()) {
        Deref @ DAE::Statement::STMT_ASSIGN { exp1: lhs, exp: rhs, .. } => {
            let mut cache = inCache.clone();
            let mut env = inEnv.clone();
            let mut lhs_cref: metamodelica::Ref<DAE::ComponentRef>;
            let mut rhs_val: metamodelica::Ref<Values::Value>;
            (cache, rhs_val) = cevalExp(rhs.clone(), cache, env.clone())?;
            lhs_cref = extractLhsComponentRef(lhs.clone())?;
            (cache, env) = assignVariable(lhs_cref, &rhs_val, &cache, &env)?;
            (cache, env, crate::CevalFunction::LoopControl::NEXT)
        },
        Deref @ DAE::Statement::STMT_TUPLE_ASSIGN { .. } => {
            let mut cache: FCore::Cache;
            let mut env: FCore::Graph;
            (cache, env) = evaluateTupleAssignStatement(&inStatement, inCache, inEnv)?;
            (cache, env, crate::CevalFunction::LoopControl::NEXT)
        },
        Deref @ DAE::Statement::STMT_ASSIGN_ARR { lhs, exp: rhs, .. } => {
            let mut env = inEnv.clone();
            let mut cache: FCore::Cache;
            let mut lhs_cref: metamodelica::Ref<DAE::ComponentRef>;
            let mut rhs_val: metamodelica::Ref<Values::Value>;
            (cache, rhs_val) = cevalExp(rhs.clone(), inCache, env.clone())?;
            lhs_cref = extractLhsComponentRef(lhs.clone())?;
            (cache, env) = assignVariable(lhs_cref, &rhs_val, &cache, &env)?;
            (cache, env, crate::CevalFunction::LoopControl::NEXT)
        },
        Deref @ DAE::Statement::STMT_IF { .. } => {
            let mut cache: FCore::Cache;
            let mut env: FCore::Graph;
            let mut loop_ctrl: LoopControl;
            (cache, env, loop_ctrl) = evaluateIfStatement(&inStatement, inCache, inEnv)?;
            (cache, env, loop_ctrl)
        },
        Deref @ DAE::Statement::STMT_FOR { .. } => {
            let mut cache: FCore::Cache;
            let mut env: FCore::Graph;
            let mut loop_ctrl: LoopControl;
            (cache, env, loop_ctrl) = evaluateForStatement(&inStatement, inCache, inEnv)?;
            (cache, env, loop_ctrl)
        },
        Deref @ DAE::Statement::STMT_WHILE { exp: condition, statementLst: statements, .. } => {
            let mut cache: FCore::Cache;
            let mut env: FCore::Graph;
            let mut loop_ctrl: LoopControl;
            (cache, env, loop_ctrl) = evaluateWhileStatement(metamodelica::AsArg::as_arg(&condition), metamodelica::AsArg::as_arg(&statements), &inCache, &inEnv, crate::CevalFunction::LoopControl::NEXT)?;
            (cache, env, loop_ctrl)
        },
        Deref @ DAE::Statement::STMT_ASSERT { cond: condition, .. } => {
            let mut cache: FCore::Cache;
            let __pa0 = ::match_deref::match_deref! { match &(cevalExp(condition.clone(), inCache, inEnv.clone())?) {
                (__pa0, Deref @ Values::Value::BOOL { boolean: true }) => __pa0.clone(),
                _ => return Err("pattern mismatch"),
            } };
            cache = metamodelica::Own::own(__pa0);
            (cache, inEnv, crate::CevalFunction::LoopControl::NEXT)
        },
        Deref @ DAE::Statement::STMT_ASSERT { cond: condition, .. } => {
            let mut cache: FCore::Cache;
            let __pa0 = ::match_deref::match_deref! { match &(cevalExp(condition.clone(), inCache, inEnv.clone())?) {
                (__pa0, Deref @ Values::Value::BOOL { boolean: true }) => __pa0.clone(),
                _ => return Err("pattern mismatch"),
            } };
            cache = metamodelica::Own::own(__pa0);
            (cache, inEnv, crate::CevalFunction::LoopControl::NEXT)
        },
        Deref @ DAE::Statement::STMT_NORETCALL { exp: rhs @ Deref @ DAE::Exp::CALL { expLst: exps, attr: Deref @ DAE::CallAttributes { tailCall, .. }, .. }, .. } => {
            let mut cache: FCore::Cache;
            let mut env: FCore::Graph;
            let mut v: metamodelica::Ref<Values::Value>;
            let mut vals: metamodelica::List<metamodelica::Ref<Values::Value>>;
            let mut var: ArcStr;
            let mut vars: metamodelica::List<ArcStr>;
            (cache, vals) = cevalExpList(exps.clone(), inCache, inEnv.clone())?;
            (cache, v) = cevalExp(rhs.clone(), cache, inEnv.clone())?;
            (cache, env, outLoopControl) = (::match_deref::match_deref! { match &(tailCall.clone()) {
        DAE::TailCall::NO_TAIL { .. } => (cache, inEnv, crate::CevalFunction::LoopControl::NEXT),
        DAE::TailCall::TAIL { outVars: Deref @ metamodelica::ListNode::Nil, .. } => (cache, inEnv, crate::CevalFunction::LoopControl::RETURN),
        DAE::TailCall::TAIL { outVars: Deref @ metamodelica::ListNode::Cons { head: __esc_var, tail: Deref @ metamodelica::ListNode::Nil }, .. } => {
            var = (*__esc_var).clone();
            (cache, env) = assignVariable(ComponentReference::makeUntypedCrefIdent(var.clone()), &v, &cache, &inEnv)?;
            (cache, env, crate::CevalFunction::LoopControl::RETURN)
        },
        DAE::TailCall::TAIL { outVars: __esc_vars, .. } => {
            vars = (*__esc_vars).clone();
            env = inEnv.clone();
            let __pa0 = ::match_deref::match_deref! { match &(v) {
                Deref @ Values::Value::TUPLE { valueLst: __pa0 } => __pa0.clone(),
                _ => return Err("pattern mismatch"),
            } };
            vals = metamodelica::Own::own(__pa0);
            for mut val in &*vals {
                let (__pa1, __pa2) = ::match_deref::match_deref! { match &(vars.clone()) {
                    Deref @ metamodelica::ListNode::Cons { head: __pa1, tail: __pa2 } => (__pa1.clone(), __pa2.clone()),
                    _ => return Err("pattern mismatch"),
                } };
                var = metamodelica::Own::own(__pa1);
                vars = metamodelica::Own::own(__pa2);
                (cache, env) = assignVariable(ComponentReference::makeUntypedCrefIdent(var), metamodelica::AsArg::as_arg(&val), &cache, &inEnv)?;
            }
            (cache, env, crate::CevalFunction::LoopControl::RETURN)
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
            (cache, env, crate::CevalFunction::LoopControl::NEXT)
        },
        Deref @ DAE::Statement::STMT_RETURN { .. } => {
            (inCache, inEnv, crate::CevalFunction::LoopControl::RETURN)
        },
        Deref @ DAE::Statement::STMT_BREAK { .. } => {
            (inCache, inEnv, crate::CevalFunction::LoopControl::BREAK)
        },
        _ => {
            let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { return Err("pattern mismatch") };
            Debug::traceln(literal!("- CevalFunction.evaluateStatement failed for:"))?;
            Debug::traceln(DAEDump::ppStatementStr(inStatement))?;
            return Err("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outCache, outEnv, outLoopControl))
}

fn evaluateStatements(
    mut inStatement: metamodelica::List<metamodelica::Ref<DAE::Statement>>,
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
) -> Result<(FCore::Cache, FCore::Graph, LoopControl)> {
    let mut outCache: FCore::Cache;
    let mut outEnv: FCore::Graph;
    let mut outLoopControl: LoopControl;
    (outCache, outEnv, outLoopControl) =
        evaluateStatements2(inStatement, inCache, inEnv, crate::CevalFunction::LoopControl::NEXT)?;
    Ok((outCache, outEnv, outLoopControl))
}

fn evaluateStatements2(
    mut inStatement: metamodelica::List<metamodelica::Ref<DAE::Statement>>,
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
    mut inLoopControl: LoopControl,
) -> Result<(FCore::Cache, FCore::Graph, LoopControl)> {
    '__tco: loop {
        ::match_deref::match_deref! { match &((inStatement, inLoopControl)) {
            (_, LoopControl::BREAK { .. }) => {
                return Ok((inCache, inEnv, inLoopControl))
            },
            (_, LoopControl::RETURN { .. }) => {
                return Ok((inCache, inEnv, inLoopControl))
            },
            (Deref @ metamodelica::ListNode::Nil, _) => {
                return Ok((inCache, inEnv, inLoopControl))
            },
            (Deref @ metamodelica::ListNode::Cons { head: stmt, tail: rest_stmts }, LoopControl::NEXT { .. }) => {
                let mut cache: FCore::Cache;
                let mut env: FCore::Graph;
                let mut loop_ctrl: LoopControl;
                (cache, env, loop_ctrl) = evaluateStatement(stmt.clone(), inCache, inEnv)?;
                { (inStatement, inCache, inEnv, inLoopControl) = (rest_stmts.clone(), cache, env, loop_ctrl); continue '__tco; }
            },
            _ => return Err("match: no arm matched"),
        } }
    }
}

fn evaluateTupleAssignStatement(
    mut inStatement: &metamodelica::Ref<DAE::Statement>,
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
) -> Result<(FCore::Cache, FCore::Graph)> {
    let mut outCache: FCore::Cache;
    let mut outEnv: FCore::Graph;
    (outCache, outEnv) = (match &**inStatement {
        DAE::Statement::STMT_TUPLE_ASSIGN {
            expExpLst: lhs_expl,
            exp: rhs,
            ..
        } => {
            let mut env = inEnv;
            let mut rhs_vals: metamodelica::List<metamodelica::Ref<Values::Value>>;
            let mut lhs_crefs: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>;
            let mut cache: FCore::Cache;
            let (__pa0, __pa1) = ::match_deref::match_deref! { match &(cevalExp(rhs.clone(), inCache, env.clone())?) {
                (__pa0, Deref @ Values::Value::TUPLE { valueLst: __pa1 }) => (__pa0.clone(), __pa1.clone()),
                _ => return Err("pattern mismatch"),
            } };
            cache = metamodelica::Own::own(__pa0);
            rhs_vals = metamodelica::Own::own(__pa1);
            lhs_crefs = List::map(lhs_expl.clone(), &extractLhsComponentRef)?;
            (cache, env) = assignTuple(lhs_crefs, rhs_vals, cache, env)?;
            (cache, env)
        }
        _ => return Err("match: no arm matched"),
    });
    Ok((outCache, outEnv))
}

fn evaluateIfStatement(
    mut inStatement: &metamodelica::Ref<DAE::Statement>,
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
) -> Result<(FCore::Cache, FCore::Graph, LoopControl)> {
    let mut outCache: FCore::Cache;
    let mut outEnv: FCore::Graph;
    let mut outLoopControl: LoopControl;
    (outCache, outEnv, outLoopControl) = (match &**inStatement {
        DAE::Statement::STMT_IF {
            exp: cond,
            statementLst: stmts,
            else_: else_branch,
            ..
        } => {
            let mut cache: FCore::Cache;
            let mut env: FCore::Graph;
            let mut bool_cond: bool;
            let mut loop_ctrl: LoopControl;
            let (__pa0, __pa1) = ::match_deref::match_deref! { match &(cevalExp(cond.clone(), inCache, inEnv.clone())?) {
                (__pa0, Deref @ Values::Value::BOOL { boolean: __pa1 }) => (__pa0.clone(), __pa1.clone()),
                _ => return Err("pattern mismatch"),
            } };
            cache = metamodelica::Own::own(__pa0);
            bool_cond = metamodelica::Own::own(__pa1);
            (cache, env, loop_ctrl) =
                evaluateIfStatement2(bool_cond, stmts.clone(), else_branch.clone(), cache, inEnv)?;
            (cache, env, loop_ctrl)
        }
        _ => return Err("match: no arm matched"),
    });
    Ok((outCache, outEnv, outLoopControl))
}

fn evaluateIfStatement2(
    mut inCondition: bool,
    mut inStatements: metamodelica::List<metamodelica::Ref<DAE::Statement>>,
    mut inElse: metamodelica::Ref<DAE::Else>,
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
) -> Result<(FCore::Cache, FCore::Graph, LoopControl)> {
    '__tco: loop {
        ::match_deref::match_deref! { match &((inCondition, inStatements, inElse, inEnv.clone())) {
            (true, statements, _, env) => {
                let mut cache: FCore::Cache;
                let mut loop_ctrl: LoopControl;
                let mut env = (*env).clone();
                return Ok(evaluateStatements(statements.clone(), inCache, env.clone())?)
            },
            (false, _, Deref @ DAE::Else::ELSE { statementLst: statements }, env) => {
                let mut cache: FCore::Cache;
                let mut loop_ctrl: LoopControl;
                let mut env = (*env).clone();
                return Ok(evaluateStatements(statements.clone(), inCache, env.clone())?)
            },
            (false, _, Deref @ DAE::Else::ELSEIF { exp: condition, statementLst: statements, else_: else_branch }, env) => {
                let mut cache: FCore::Cache;
                let mut bool_condition: bool;
                let mut loop_ctrl: LoopControl;
                let mut env = (*env).clone();
                let (__pa0, __pa1) = ::match_deref::match_deref! { match &(cevalExp(condition.clone(), inCache, env.clone())?) {
                    (__pa0, Deref @ Values::Value::BOOL { boolean: __pa1 }) => (__pa0.clone(), __pa1.clone()),
                    _ => return Err("pattern mismatch"),
                } };
                cache = metamodelica::Own::own(__pa0);
                bool_condition = metamodelica::Own::own(__pa1);
                { (inCondition, inStatements, inElse, inCache, inEnv) = (bool_condition, statements.clone(), else_branch.clone(), cache, env.clone()); continue '__tco; }
            },
            (false, _, Deref @ DAE::Else::NOELSE { .. }, _) => {
                return Ok((inCache, inEnv, crate::CevalFunction::LoopControl::NEXT))
            },
            _ => return Err("match: no arm matched"),
        } }
    }
}

fn evaluateForStatement(
    mut inStatement: &metamodelica::Ref<DAE::Statement>,
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
) -> Result<(FCore::Cache, FCore::Graph, LoopControl)> {
    let mut outCache: FCore::Cache;
    let mut outEnv: FCore::Graph;
    let mut outLoopControl: LoopControl;
    (outCache, outEnv, outLoopControl) = 'mc: {
        let __mc_input = (&**inStatement, inEnv);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Statement::STMT_FOR { type_: ety, iter: iter_name, range, statementLst: statements, .. }, env) => {
                    let mut ty: metamodelica::Ref<DAE::Type>;
                    let mut range_vals: metamodelica::List<metamodelica::Ref<Values::Value>>;
                    let mut cache: FCore::Cache;
                    let mut iter_cr: metamodelica::Ref<DAE::ComponentRef>;
                    let mut loop_ctrl: LoopControl;
                    let mut env = (*env).clone();
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(cevalExp(range.clone(), inCache.clone(), env.clone())?) {
                        (__pa0, Deref @ Values::Value::ARRAY { valueLst: __pa1, .. }) => (__pa0.clone(), __pa1.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    cache = metamodelica::Own::own(__pa0);
                    range_vals = metamodelica::Own::own(__pa1);
                    (env, ty, iter_cr) = extendEnvWithForScope(iter_name.clone(), ety.clone(), env.clone())?;
                    (cache, env, loop_ctrl) = evaluateForLoopArray(cache.clone(), env.clone(), &iter_cr, &ty, range_vals.clone(), metamodelica::AsArg::as_arg(&statements), crate::CevalFunction::LoopControl::NEXT)?;
                    Ok((cache.clone(), env.clone(), loop_ctrl))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Statement::STMT_FOR { range, .. }, _) => {
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { return Err("pattern mismatch") };
                    Debug::traceln(literal!("- evaluateForStatement not implemented for:"))?;
                    Debug::traceln(ExpressionBasics::printExpStr(range.clone())?)?;
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((outCache, outEnv, outLoopControl))
}

fn evaluateForLoopArray<'__b>(
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
    mut inIter: &'__b metamodelica::Ref<DAE::ComponentRef>,
    mut inIterType: &'__b metamodelica::Ref<DAE::Type>,
    mut inValues: metamodelica::List<metamodelica::Ref<Values::Value>>,
    mut inStatements: &'__b metamodelica::List<metamodelica::Ref<DAE::Statement>>,
    mut inLoopControl: LoopControl,
) -> Result<(FCore::Cache, FCore::Graph, LoopControl)> {
    '__tco: loop {
        ::match_deref::match_deref! { match &((inEnv.clone(), inValues, inLoopControl)) {
            (_, _, LoopControl::BREAK { .. }) => {
                return Ok((inCache, inEnv, crate::CevalFunction::LoopControl::NEXT))
            },
            (_, _, LoopControl::RETURN { .. }) => {
                return Ok((inCache, inEnv, inLoopControl))
            },
            (_, Deref @ metamodelica::ListNode::Nil, _) => {
                return Ok((inCache, inEnv, inLoopControl))
            },
            (env, Deref @ metamodelica::ListNode::Cons { head: value, tail: rest_vals }, LoopControl::NEXT { .. }) => {
                let mut cache: FCore::Cache;
                let mut loop_ctrl: LoopControl;
                let mut env = (*env).clone();
                env = updateVariableBinding(inIter, env.clone(), inIterType.clone(), value.clone())?;
                (cache, env, loop_ctrl) = evaluateStatements(inStatements.clone(), inCache, env.clone())?;
                { (inCache, inEnv, inIter, inIterType, inValues, inStatements, inLoopControl) = (cache, env.clone(), inIter, inIterType, rest_vals.clone(), inStatements, loop_ctrl); continue '__tco; }
            },
            _ => return Err("match: no arm matched"),
        } }
    }
}

fn evaluateWhileStatement(
    mut inCondition: &metamodelica::Ref<DAE::Exp>,
    mut inStatements: &metamodelica::List<metamodelica::Ref<DAE::Statement>>,
    mut inCache: &FCore::Cache,
    mut inEnv: &FCore::Graph,
    mut inLoopControl: LoopControl,
) -> Result<(FCore::Cache, FCore::Graph, LoopControl)> {
    let mut outCache: FCore::Cache;
    let mut outEnv: FCore::Graph;
    let mut outLoopControl: LoopControl;
    (outCache, outEnv, outLoopControl) = (match inLoopControl {
        LoopControl::BREAK { .. } => (inCache.clone(), inEnv.clone(), crate::CevalFunction::LoopControl::NEXT),
        LoopControl::RETURN { .. } => (inCache.clone(), inEnv.clone(), inLoopControl),
        _ => {
            let mut cache: FCore::Cache;
            let mut env: FCore::Graph;
            let mut loop_ctrl: LoopControl;
            let mut b: bool;
            let (__pa0, __pa1) = ::match_deref::match_deref! { match &(cevalExp(inCondition.clone(), inCache.clone(), inEnv.clone())?) {
                (__pa0, Deref @ Values::Value::BOOL { boolean: __pa1 }) => (__pa0.clone(), __pa1.clone()),
                _ => return Err("pattern mismatch"),
            } };
            cache = metamodelica::Own::own(__pa0);
            b = metamodelica::Own::own(__pa1);
            if b {
                (cache, env, loop_ctrl) = evaluateStatements(inStatements.clone(), cache, inEnv.clone())?;
                (cache, env, loop_ctrl) = evaluateWhileStatement(inCondition, inStatements, &cache, &env, loop_ctrl)?;
            } else {
                loop_ctrl = crate::CevalFunction::LoopControl::NEXT;
                env = inEnv.clone();
            }
            (cache, env, loop_ctrl)
        }
    });
    Ok((outCache, outEnv, outLoopControl))
}

fn extractLhsComponentRef(mut inExp: metamodelica::Ref<DAE::Exp>) -> Result<metamodelica::Ref<DAE::ComponentRef>> {
    let mut outCref: metamodelica::Ref<DAE::ComponentRef>;
    outCref = (match &*inExp {
        DAE::Exp::CREF { componentRef: cref, .. } => cref.clone(),
        _ => {
            let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else {
                return Err("pattern mismatch");
            };
            Debug::traceln({
                let mut __mm_s = String::new();
                __mm_s.push_str(&*literal!("- CevalFunction.extractLhsComponentRef failed on "));
                __mm_s.push_str(&*ExpressionBasics::printExpStr(inExp)?);
                ArcStr::from(__mm_s)
            })?;
            return Err("fail");
        }
    });
    Ok(outCref)
}

fn cevalExp(
    mut inExp: metamodelica::Ref<DAE::Exp>,
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
) -> Result<(FCore::Cache, metamodelica::Ref<Values::Value>)> {
    let mut outCache: FCore::Cache;
    let mut outValue: metamodelica::Ref<Values::Value>;
    (outCache, outValue) = Ceval::ceval(
        inCache,
        inEnv,
        inExp,
        true,
        Absyn::Msg::MSG {
            info: Absyn::dummyInfo.clone(),
        },
        0,
    )?;
    let false = (openmodelica_frontend_types::Values::Value::interned_META_FAIL() == outValue.clone()) else {
        return Err("pattern mismatch");
    };
    Ok((outCache, outValue))
}

fn cevalExpList(
    mut inExpLst: metamodelica::List<metamodelica::Ref<DAE::Exp>>,
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
) -> Result<(FCore::Cache, metamodelica::List<metamodelica::Ref<Values::Value>>)> {
    let mut outCache: FCore::Cache;
    let mut outValue: metamodelica::List<metamodelica::Ref<Values::Value>>;
    (outCache, outValue) = Ceval::cevalList(
        inCache,
        inEnv,
        inExpLst,
        true,
        Absyn::Msg::MSG {
            info: Absyn::dummyInfo.clone(),
        },
        0,
    )?;
    Ok((outCache, outValue))
}

// [EENV]  Environment extension functions (add variables).
fn setupFunctionEnvironment(
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
    mut inFuncName: ArcStr,
    mut inFuncParams: metamodelica::List<(
        metamodelica::Ref<DAE::Element>,
        Option<metamodelica::Ref<Values::Value>>,
    )>,
) -> Result<(FCore::Cache, FCore::Graph)> {
    let mut outCache: FCore::Cache;
    let mut outEnv: FCore::Graph;
    outEnv = FGraph::openScope(
        inEnv,
        openmodelica_frontend_types::SCode::Encapsulated::NOT_ENCAPSULATED,
        inFuncName,
        Some(openmodelica_frontend_dump::FCore::ScopeType::FUNCTION_SCOPE),
    )?;
    (outCache, outEnv) = extendEnvWithFunctionVars(inCache, outEnv, inFuncParams)?;
    Ok((outCache, outEnv))
}

fn extendEnvWithFunctionVars(
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
    mut inFuncParams: metamodelica::List<(
        metamodelica::Ref<DAE::Element>,
        Option<metamodelica::Ref<Values::Value>>,
    )>,
) -> Result<(FCore::Cache, FCore::Graph)> {
    '__tco: loop {
        ::match_deref::match_deref! { match &((inCache.clone(), inEnv.clone(), inFuncParams)) {
            (_, _, Deref @ metamodelica::ListNode::Nil) => {
                return Ok((inCache, inEnv))
            },
            (cache, env, Deref @ metamodelica::ListNode::Cons { head: param, tail: rest_params }) => {
                let mut cache = (*cache).clone();
                let mut env = (*env).clone();
                (cache, env) = extendEnvWithFunctionVar(cache.clone(), env.clone(), &(param.clone()))?;
                { (inCache, inEnv, inFuncParams) = (cache.clone(), env.clone(), rest_params.clone()); continue '__tco; }
            },
            _ => return Err("match: no arm matched"),
        } }
    }
}

fn extendEnvWithFunctionVar(
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
    mut inFuncParam: &FunctionVar,
) -> Result<(FCore::Cache, FCore::Graph)> {
    let mut outCache: FCore::Cache;
    let mut outEnv: FCore::Graph;
    (outCache, outEnv) = 'mc: {
        let __mc_input = (inEnv.clone(), inFuncParam);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (env, (e, val @ Some(_))) => {
                    let mut cache: FCore::Cache;
                    let mut env = (*env).clone();
                    (cache, env) = extendEnvWithElement(metamodelica::AsArg::as_arg(&e), val.clone(), &inCache, env.clone())?;
                    Ok((cache.clone(), env.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (env, (e @ Deref @ DAE::Element::VAR { binding: binding_exp, .. }, None)) => {
                    let mut val: Option<metamodelica::Ref<Values::Value>>;
                    let mut cache: FCore::Cache;
                    let mut env = (*env).clone();
                    (val, cache) = evaluateBinding(binding_exp.clone(), inCache.clone(), inEnv.clone())?;
                    (cache, env) = extendEnvWithElement(metamodelica::AsArg::as_arg(&e), val.clone(), &cache, env.clone())?;
                    Ok((cache.clone(), env.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, (e, _)) => {
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { return Err("pattern mismatch") };
                    Debug::traceln(literal!("- CevalFunction.extendEnvWithFunctionVars failed for:"))?;
                    Debug::traceln(DAEDump::dumpElementsStr(&(list![e.clone()]))?)?;
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((outCache, outEnv))
}

fn evaluateBinding(
    mut inBinding: Option<metamodelica::Ref<DAE::Exp>>,
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
) -> Result<(Option<metamodelica::Ref<Values::Value>>, FCore::Cache)> {
    let mut outValue: Option<metamodelica::Ref<Values::Value>>;
    let mut outCache: FCore::Cache;
    (outValue, outCache) = (::match_deref::match_deref! { match &(inBinding) {
        Some(binding_exp) => {
            let mut cache: FCore::Cache;
            let mut val: metamodelica::Ref<Values::Value>;
            (cache, val) = cevalExp(binding_exp.clone(), inCache, inEnv)?;
            (Some(val), cache)
        },
        None => {
            (None, inCache)
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outValue, outCache))
}

fn extendEnvWithElement(
    mut inElement: &metamodelica::Ref<DAE::Element>,
    mut inBindingValue: Option<metamodelica::Ref<Values::Value>>,
    mut inCache: &FCore::Cache,
    mut inEnv: FCore::Graph,
) -> Result<(FCore::Cache, FCore::Graph)> {
    let mut outCache: FCore::Cache;
    let mut outEnv: FCore::Graph;
    (outCache, outEnv) = (match &**inElement {
        DAE::Element::VAR {
            componentRef: cr,
            ty,
            dims,
            ..
        } => {
            let mut name: ArcStr;
            let mut cache: FCore::Cache;
            let mut env: FCore::Graph;
            name = ComponentReference::crefStr(cr)?;
            (cache, env) = extendEnvWithVar(name, ty.clone(), inBindingValue, dims, inCache, inEnv)?;
            (cache, env)
        }
        _ => return Err("match: no arm matched"),
    });
    Ok((outCache, outEnv))
}

fn extendEnvWithVar(
    mut inName: ArcStr,
    mut inType: metamodelica::Ref<DAE::Type>,
    mut inOptValue: Option<metamodelica::Ref<Values::Value>>,
    mut inDims: &metamodelica::List<metamodelica::Ref<DAE::Dimension>>,
    mut inCache: &FCore::Cache,
    mut inEnv: FCore::Graph,
) -> Result<(FCore::Cache, FCore::Graph)> {
    let mut outCache: FCore::Cache;
    let mut outEnv: FCore::Graph;
    (outCache, outEnv) = 'mc: {
        let __mc_input = inEnv.clone();
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            let mut ty: metamodelica::Ref<DAE::Type>;
            let mut var: metamodelica::Ref<DAE::Var>;
            let mut binding: metamodelica::Ref<DAE::Binding>;
            let mut cache: FCore::Cache;
            let mut env: FCore::Graph;
            let mut record_env: FCore::Graph;
            let true = (Types::isRecord(&inType)) else {
                return Err("pattern mismatch");
            };
            binding = makeBinding(inOptValue.clone());
            (cache, ty) = appendDimensions(inType.clone(), inOptValue.clone(), inDims, inCache, &inEnv)?;
            var = makeFunctionVariable(inName.clone(), ty.clone(), binding.clone());
            (cache, record_env) = makeRecordEnvironment(&inType, inOptValue.clone(), cache.clone(), inEnv.clone())?;
            env = FGraph::mkComponentNode(
                inEnv.clone(),
                var.clone(),
                metamodelica::Ref::new(SCode::Element::COMPONENT {
                    name: inName.clone(),
                    prefixes: SCode::defaultPrefixes.clone(),
                    attributes: SCode::Attributes {
                        arrayDims: metamodelica::nil(),
                        connectorType: openmodelica_frontend_types::SCode::ConnectorType::POTENTIAL,
                        parallelism: openmodelica_frontend_types::SCode::Parallelism::NON_PARALLEL,
                        variability: openmodelica_frontend_types::SCode::Variability::VAR,
                        direction: openmodelica_ast::Absyn::Direction::BIDIR,
                        isField: openmodelica_ast::Absyn::IsField::NONFIELD,
                    },
                    typeSpec: metamodelica::Ref::new(Absyn::TypeSpec::TPATH {
                        path: metamodelica::Ref::new(Absyn::Path::IDENT { name: literal!("") }),
                        arrayDim: None,
                    }),
                    modifications: openmodelica_frontend_types::SCode::Mod::interned_NOMOD(),
                    comment: SCode::noComment.clone(),
                    condition: None,
                    info: Absyn::dummyInfo.clone(),
                }),
                openmodelica_frontend_types::DAE::Mod::interned_NOMOD(),
                openmodelica_frontend_dump::FCore::Status::VAR_TYPED,
                record_env.clone(),
            )?;
            Ok((cache.clone(), env.clone()))
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            let _ = __mc_input.clone() else { return Err("nomatch") };
            let mut ty: metamodelica::Ref<DAE::Type>;
            let mut var: metamodelica::Ref<DAE::Var>;
            let mut binding: metamodelica::Ref<DAE::Binding>;
            let mut cache: FCore::Cache;
            let mut env: FCore::Graph;
            binding = makeBinding(inOptValue.clone());
            (cache, ty) = appendDimensions(inType.clone(), inOptValue.clone(), inDims, inCache, &inEnv)?;
            var = makeFunctionVariable(inName.clone(), ty.clone(), binding.clone());
            env = FGraph::mkComponentNode(
                inEnv.clone(),
                var.clone(),
                metamodelica::Ref::new(SCode::Element::COMPONENT {
                    name: inName.clone(),
                    prefixes: SCode::defaultPrefixes.clone(),
                    attributes: SCode::Attributes {
                        arrayDims: metamodelica::nil(),
                        connectorType: openmodelica_frontend_types::SCode::ConnectorType::POTENTIAL,
                        parallelism: openmodelica_frontend_types::SCode::Parallelism::NON_PARALLEL,
                        variability: openmodelica_frontend_types::SCode::Variability::VAR,
                        direction: openmodelica_ast::Absyn::Direction::BIDIR,
                        isField: openmodelica_ast::Absyn::IsField::NONFIELD,
                    },
                    typeSpec: metamodelica::Ref::new(Absyn::TypeSpec::TPATH {
                        path: metamodelica::Ref::new(Absyn::Path::IDENT { name: literal!("") }),
                        arrayDim: None,
                    }),
                    modifications: openmodelica_frontend_types::SCode::Mod::interned_NOMOD(),
                    comment: SCode::noComment.clone(),
                    condition: None,
                    info: Absyn::dummyInfo.clone(),
                }),
                openmodelica_frontend_types::DAE::Mod::interned_NOMOD(),
                openmodelica_frontend_dump::FCore::Status::VAR_TYPED,
                FGraph::empty(),
            )?;
            Ok((cache.clone(), env.clone()))
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((outCache, outEnv))
}

fn makeFunctionVariable(
    mut inName: ArcStr,
    mut inType: metamodelica::Ref<DAE::Type>,
    mut inBinding: metamodelica::Ref<DAE::Binding>,
) -> metamodelica::Ref<DAE::Var> {
    let mut outVar: metamodelica::Ref<DAE::Var>;
    outVar = metamodelica::Ref::new(DAE::Var {
        name: inName,
        attributes: DAE::dummyAttrVar().clone(),
        ty: inType,
        binding: inBinding,
        bind_from_outside: false,
        constOfForIteratorRange: None,
    });
    outVar
}

fn makeBinding(mut inBindingValue: Option<metamodelica::Ref<Values::Value>>) -> metamodelica::Ref<DAE::Binding> {
    let mut outBinding: metamodelica::Ref<DAE::Binding>;
    outBinding = (::match_deref::match_deref! { match &(inBindingValue) {
        Some(val) => {
            metamodelica::Ref::new(DAE::Binding::VALBOUND { valBound: val.clone(), source: openmodelica_frontend_types::DAE::BindingSource::BINDING_FROM_DEFAULT_VALUE })
        },
        None => {
            openmodelica_frontend_types::DAE::Binding::interned_UNBOUND()
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outBinding
}

fn makeRecordEnvironment(
    mut inRecordType: &metamodelica::Ref<DAE::Type>,
    mut inOptValue: Option<metamodelica::Ref<Values::Value>>,
    mut inCache: FCore::Cache,
    mut inGraph: FCore::Graph,
) -> Result<(FCore::Cache, FCore::Graph)> {
    let mut outCache: FCore::Cache;
    let mut outRecordEnv: FCore::Graph;
    (outCache, outRecordEnv) = (match &**inRecordType {
        DAE::Type::T_COMPLEX {
            complexClassType: ClassInf::State::RECORD { .. },
            varLst: var_lst,
            ..
        } => {
            let mut vals: metamodelica::List<Option<metamodelica::Ref<Values::Value>>>;
            let mut cache: FCore::Cache;
            let mut graph: FCore::Graph;
            let mut parent: Mutable::Mutable<metamodelica::Ref<FCore::Node>>;
            let mut child: Mutable::Mutable<metamodelica::Ref<FCore::Node>>;
            let mut node: metamodelica::Ref<FCore::Node>;
            parent = FGraph::lastScopeRef(&inGraph)?;
            (graph, node) = FGraph::node(
                inGraph,
                arcstr::literal!(FNode::feNodeName),
                list![parent.clone()],
                metamodelica::Ref::new(FCore::Data::ND { scopeType: None }),
            );
            child = FNode::toRef(node);
            FNode::addChildRef(parent, &(arcstr::literal!(FNode::feNodeName)), child.clone(), false)?;
            graph = FGraph::pushScopeRef(graph, child)?;
            vals = getRecordValues(inOptValue, inRecordType)?;
            (cache, graph) = List::threadFold(
                var_lst,
                vals,
                &move |__a0: metamodelica::Ref<DAE::Var>,
                       __a1: Option<metamodelica::Ref<Values::Value>>,
                       __a2: (FCore::Cache, FCore::Graph)| extendEnvWithRecordVar(&__a0, __a1, &__a2),
                (inCache, graph),
            )?;
            (cache, graph)
        }
        _ => return Err("match: no arm matched"),
    });
    Ok((outCache, outRecordEnv))
}

fn getRecordValues(
    mut inOptValue: Option<metamodelica::Ref<Values::Value>>,
    mut inRecordType: &metamodelica::Ref<DAE::Type>,
) -> Result<metamodelica::List<Option<metamodelica::Ref<Values::Value>>>> {
    let mut outValues: metamodelica::List<Option<metamodelica::Ref<Values::Value>>>;
    outValues = (::match_deref::match_deref! { match &((inOptValue, &**inRecordType)) {
        (Some(Deref @ Values::Value::RECORD { orderd: vals, .. }), _) => {
            let mut opt_vals: metamodelica::List<Option<metamodelica::Ref<Values::Value>>>;
            opt_vals = List::map(vals.clone(), &fnptr!(Util::makeOption, _))?;
            opt_vals
        },
        (None, Deref @ DAE::Type::T_COMPLEX { varLst: vars, .. }) => {
            let mut opt_vals: metamodelica::List<Option<metamodelica::Ref<Values::Value>>>;
            let mut n: i32;
            n = ((vars).len() as i32);
            opt_vals = List::fill(None, n);
            opt_vals
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok(outValues)
}

fn extendEnvWithRecordVar(
    mut inVar: &metamodelica::Ref<DAE::Var>,
    mut inOptValue: Option<metamodelica::Ref<Values::Value>>,
    mut inEnv: &(FCore::Cache, FCore::Graph),
) -> Result<(FCore::Cache, FCore::Graph)> {
    let mut outEnv: (FCore::Cache, FCore::Graph);
    outEnv = (::match_deref::match_deref! { match &((&**inVar, inEnv)) {
        (Deref @ DAE::Var { name, ty, .. }, (cache, env)) => {
            let mut cache = (*cache).clone();
            let mut env = (*env).clone();
            (cache, env) = extendEnvWithVar(name.clone(), ty.clone(), inOptValue, &(metamodelica::nil()), metamodelica::AsArg::as_arg(&cache), env.clone())?;
            outEnv = (cache.clone(), env.clone());
            outEnv
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outEnv)
}

fn extendEnvWithForScope(
    mut inIterName: ArcStr,
    mut inIterType: metamodelica::Ref<DAE::Type>,
    mut inEnv: FCore::Graph,
) -> Result<(
    FCore::Graph,
    metamodelica::Ref<DAE::Type>,
    metamodelica::Ref<DAE::ComponentRef>,
)> {
    let mut outEnv: FCore::Graph;
    let mut outIterType: metamodelica::Ref<DAE::Type>;
    let mut outIterCref: metamodelica::Ref<DAE::ComponentRef>;
    outIterType = Types::expTypetoTypesType(&inIterType);
    outEnv = FGraph::addForIterator(
        inEnv,
        inIterName.clone(),
        outIterType.clone(),
        openmodelica_frontend_types::DAE::Binding::interned_UNBOUND(),
        openmodelica_frontend_types::SCode::Variability::CONST,
        Some(openmodelica_frontend_types::DAE::Const::C_CONST),
    )?;
    outIterCref = ComponentReferenceBasics::makeCrefIdent(inIterName, inIterType, metamodelica::nil());
    Ok((outEnv, outIterType, outIterCref))
}

fn appendDimensions(
    mut inType: metamodelica::Ref<DAE::Type>,
    mut inOptBinding: Option<metamodelica::Ref<Values::Value>>,
    mut inDims: &metamodelica::List<metamodelica::Ref<DAE::Dimension>>,
    mut inCache: &FCore::Cache,
    mut inEnv: &FCore::Graph,
) -> Result<(FCore::Cache, metamodelica::Ref<DAE::Type>)> {
    let mut outCache: FCore::Cache;
    let mut outType: metamodelica::Ref<DAE::Type>;
    let mut binding_dims: metamodelica::List<i32>;
    binding_dims = ValuesUtil::valueDimensions(
        &(Util::getOptionOrDefault(
            inOptBinding,
            metamodelica::Ref::new(Values::Value::INTEGER { integer: 0 }),
        )),
    );
    (outCache, outType) = appendDimensions2(inType, inDims, binding_dims, inCache, inEnv)?;
    Ok((outCache, outType))
}

fn appendDimensions2(
    mut inType: metamodelica::Ref<DAE::Type>,
    mut inDims: &metamodelica::List<metamodelica::Ref<DAE::Dimension>>,
    mut inBindingDims: metamodelica::List<i32>,
    mut inCache: &FCore::Cache,
    mut inEnv: &FCore::Graph,
) -> Result<(FCore::Cache, metamodelica::Ref<DAE::Type>)> {
    let mut outCache: FCore::Cache;
    let mut outType: metamodelica::Ref<DAE::Type>;
    (outCache, outType) = 'mc: {
        let __mc_input = (inType, &**inDims, inBindingDims);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (ty, Deref @ metamodelica::ListNode::Nil, _) => {
                    Ok((inCache.clone(), ty.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (ty, Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Dimension::DIM_UNKNOWN { .. }, tail: rest_dims }, Deref @ metamodelica::ListNode::Cons { head: dim_int, tail: bind_dims }) => {
                    let mut dim: metamodelica::Ref<DAE::Dimension>;
                    let mut cache: FCore::Cache;
                    let mut ty = (*ty).clone();
                    dim = Expression::intDimension(dim_int.clone());
                    (cache, ty) = appendDimensions2(ty.clone(), metamodelica::AsArg::as_arg(&rest_dims), bind_dims.clone(), inCache, inEnv)?;
                    Ok((cache.clone(), metamodelica::Ref::new(DAE::Type::T_ARRAY { ty: ty.clone(), dims: list![dim.clone()] })))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (ty, Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Dimension::DIM_UNKNOWN { .. }, tail: rest_dims }, bind_dims) => {
                    let mut cache: FCore::Cache;
                    let mut ty = (*ty).clone();
                    (cache, ty) = appendDimensions2(ty.clone(), metamodelica::AsArg::as_arg(&rest_dims), bind_dims.clone(), inCache, inEnv)?;
                    Ok((cache.clone(), metamodelica::Ref::new(DAE::Type::T_ARRAY { ty: ty.clone(), dims: list![metamodelica::Ref::new(DAE::Dimension::DIM_INTEGER { integer: 0 })] })))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (ty, Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Dimension::DIM_INTEGER { integer: dim_int }, tail: rest_dims }, bind_dims) => {
                    let mut dim: metamodelica::Ref<DAE::Dimension>;
                    let mut cache: FCore::Cache;
                    let mut ty = (*ty).clone();
                    let mut bind_dims = (*bind_dims).clone();
                    dim = metamodelica::Ref::new(DAE::Dimension::DIM_INTEGER { integer: dim_int.clone() });
                    bind_dims = List::restOrEmpty(bind_dims.clone())?;
                    (cache, ty) = appendDimensions2(ty.clone(), metamodelica::AsArg::as_arg(&rest_dims), bind_dims.clone(), inCache, inEnv)?;
                    Ok((cache.clone(), metamodelica::Ref::new(DAE::Type::T_ARRAY { ty: ty.clone(), dims: list![dim.clone()] })))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (ty, Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Dimension::DIM_BOOLEAN { .. }, tail: rest_dims }, bind_dims) => {
                    let mut dim: metamodelica::Ref<DAE::Dimension>;
                    let mut cache: FCore::Cache;
                    let mut ty = (*ty).clone();
                    let mut bind_dims = (*bind_dims).clone();
                    dim = metamodelica::Ref::new(DAE::Dimension::DIM_INTEGER { integer: 2 });
                    bind_dims = List::restOrEmpty(bind_dims.clone())?;
                    (cache, ty) = appendDimensions2(ty.clone(), metamodelica::AsArg::as_arg(&rest_dims), bind_dims.clone(), inCache, inEnv)?;
                    Ok((cache.clone(), metamodelica::Ref::new(DAE::Type::T_ARRAY { ty: ty.clone(), dims: list![dim.clone()] })))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (ty, Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Dimension::DIM_ENUM { size: dim_int, .. }, tail: rest_dims }, bind_dims) => {
                    let mut dim: metamodelica::Ref<DAE::Dimension>;
                    let mut cache: FCore::Cache;
                    let mut ty = (*ty).clone();
                    let mut bind_dims = (*bind_dims).clone();
                    dim = metamodelica::Ref::new(DAE::Dimension::DIM_INTEGER { integer: dim_int.clone() });
                    bind_dims = List::restOrEmpty(bind_dims.clone())?;
                    (cache, ty) = appendDimensions2(ty.clone(), metamodelica::AsArg::as_arg(&rest_dims), bind_dims.clone(), inCache, inEnv)?;
                    Ok((cache.clone(), metamodelica::Ref::new(DAE::Type::T_ARRAY { ty: ty.clone(), dims: list![dim.clone()] })))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (ty, Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Dimension::DIM_EXP { exp: dim_exp }, tail: rest_dims }, bind_dims) => {
                    let mut dim_val: metamodelica::Ref<Values::Value>;
                    let mut dim_int: i32;
                    let mut dim: metamodelica::Ref<DAE::Dimension>;
                    let mut cache: FCore::Cache;
                    let mut ty = (*ty).clone();
                    let mut bind_dims = (*bind_dims).clone();
                    (cache, dim_val) = cevalExp(dim_exp.clone(), inCache.clone(), inEnv.clone())?;
                    dim_int = ValuesUtil::valueInteger(&dim_val)?;
                    dim = metamodelica::Ref::new(DAE::Dimension::DIM_INTEGER { integer: dim_int });
                    bind_dims = List::restOrEmpty(bind_dims.clone())?;
                    (cache, ty) = appendDimensions2(ty.clone(), metamodelica::AsArg::as_arg(&rest_dims), bind_dims.clone(), inCache, inEnv)?;
                    Ok((cache.clone(), metamodelica::Ref::new(DAE::Type::T_ARRAY { ty: ty.clone(), dims: list![dim.clone()] })))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Deref @ metamodelica::ListNode::Cons { head: _, tail: _ }, _) => {
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { return Err("pattern mismatch") };
                    Debug::trace(literal!("- CevalFunction.appendDimensions2 failed\n"))?;
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((outCache, outType))
}

// [MENV]  Environment manipulation functions (set and get variables).
fn assignVariable(
    mut inCref: metamodelica::Ref<DAE::ComponentRef>,
    mut inNewValue: &metamodelica::Ref<Values::Value>,
    mut inCache: &FCore::Cache,
    mut inEnv: &FCore::Graph,
) -> Result<(FCore::Cache, FCore::Graph)> {
    let mut outCache: FCore::Cache;
    let mut outEnv: FCore::Graph;
    (outCache, outEnv) = 'mc: {
        let __mc_input = inCref.clone();
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::ComponentRef::WILD { .. } => {
                    Ok((inCache.clone(), inEnv.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::ComponentRef::CREF_IDENT { ident: id, subscriptLst: Deref @ metamodelica::ListNode::Nil, identType: ety @ Deref @ DAE::Type::T_COMPLEX { complexClassType: ClassInf::State::RECORD { .. }, .. } } => {
                    let mut cache: FCore::Cache;
                    let mut env: FCore::Graph;
                    let mut var: metamodelica::Ref<DAE::Var>;
                    let mut inst_status: FCore::Status;
                    (_, var, _, _, inst_status, env) = Lookup::lookupIdentLocal(inCache.clone(), inEnv, id.clone())?;
                    (cache, env) = assignRecord(metamodelica::AsArg::as_arg(&ety), inNewValue, inCache.clone(), env.clone())?;
                    var = updateRecordBinding(var.clone(), inNewValue.clone());
                    env = FGraph::updateComp(inEnv.clone(), var.clone(), &inst_status, &env);
                    Ok((cache.clone(), env.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                cr @ Deref @ DAE::ComponentRef::CREF_IDENT { subscriptLst: Deref @ metamodelica::ListNode::Nil, .. } => {
                    let mut env: FCore::Graph;
                    let mut ty: metamodelica::Ref<DAE::Type>;
                    ty = Types::unflattenArrayType(Expression::r#typeof(ValuesUtil::valueExp(inNewValue.clone(), None)?)?)?;
                    env = updateVariableBinding(metamodelica::AsArg::as_arg(&cr), inEnv.clone(), ty.clone(), inNewValue.clone())?;
                    Ok((inCache.clone(), env.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::ComponentRef::CREF_IDENT { subscriptLst: subs, .. } => {
                    let mut cr: metamodelica::Ref<DAE::ComponentRef>;
                    let mut cache: FCore::Cache;
                    let mut env: FCore::Graph;
                    let mut ty: metamodelica::Ref<DAE::Type>;
                    let mut val: metamodelica::Ref<Values::Value>;
                    cr = ComponentReference::crefStripSubs(&inCref)?;
                    (ty, val) = getVariableTypeAndValue(cr.clone(), inEnv.clone())?;
                    (cache, val) = assignVector(inNewValue, &val, metamodelica::AsArg::as_arg(&subs), inCache, inEnv)?;
                    env = updateVariableBinding(&cr, inEnv.clone(), ty.clone(), val.clone())?;
                    Ok((cache.clone(), env.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::ComponentRef::CREF_QUAL { ident: id, subscriptLst: Deref @ metamodelica::ListNode::Nil, componentRef: cr_rest, .. } => {
                    let mut cache: FCore::Cache;
                    let mut env: FCore::Graph;
                    let mut var: metamodelica::Ref<DAE::Var>;
                    let mut inst_status: FCore::Status;
                    let mut comp_id: ArcStr;
                    (_, var, _, _, inst_status, env) = Lookup::lookupIdentLocal(inCache.clone(), inEnv, id.clone())?;
                    (cache, env) = assignVariable(cr_rest.clone(), inNewValue, inCache, &env)?;
                    comp_id = ComponentReferenceBasics::crefFirstIdent(metamodelica::AsArg::as_arg(&cr_rest))?;
                    var = updateRecordComponentBinding(var.clone(), comp_id.clone(), inNewValue.clone())?;
                    env = FGraph::updateComp(inEnv.clone(), var.clone(), &inst_status, &env);
                    Ok((cache.clone(), env.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((outCache, outEnv))
}

fn assignTuple(
    mut inLhsCrefs: metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>,
    mut inRhsValues: metamodelica::List<metamodelica::Ref<Values::Value>>,
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
) -> Result<(FCore::Cache, FCore::Graph)> {
    '__tco: loop {
        ::match_deref::match_deref! { match &((inLhsCrefs, inRhsValues, inCache, inEnv)) {
            (Deref @ metamodelica::ListNode::Nil, _, cache, env) => {
                return Ok((cache.clone(), env.clone()))
            },
            (Deref @ metamodelica::ListNode::Cons { head: cr, tail: rest_crefs }, Deref @ metamodelica::ListNode::Cons { head: value, tail: rest_vals }, cache, env) => {
                let mut cache = (*cache).clone();
                let mut env = (*env).clone();
                (cache, env) = assignVariable(cr.clone(), metamodelica::AsArg::as_arg(&value), metamodelica::AsArg::as_arg(&cache), metamodelica::AsArg::as_arg(&env))?;
                { (inLhsCrefs, inRhsValues, inCache, inEnv) = (rest_crefs.clone(), rest_vals.clone(), cache.clone(), env.clone()); continue '__tco; }
            },
            _ => return Err("match: no arm matched"),
        } }
    }
}

fn assignRecord(
    mut inType: &metamodelica::Ref<DAE::Type>,
    mut inValue: &metamodelica::Ref<Values::Value>,
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
) -> Result<(FCore::Cache, FCore::Graph)> {
    let mut outCache: FCore::Cache;
    let mut outEnv: FCore::Graph;
    (outCache, outEnv) = (::match_deref::match_deref! { match (inType, inValue) {
        (Deref @ DAE::Type::T_COMPLEX { varLst: vars, .. }, Deref @ Values::Value::RECORD { orderd: values, .. }) => {
            let mut cache: FCore::Cache;
            let mut env: FCore::Graph;
            (cache, env) = assignRecordComponents(vars, values, inCache, inEnv)?;
            (cache, env)
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok((outCache, outEnv))
}

fn assignRecordComponents<'__b>(
    mut inVars: &'__b metamodelica::List<metamodelica::Ref<DAE::Var>>,
    mut inValues: &'__b metamodelica::List<metamodelica::Ref<Values::Value>>,
    mut inCache: FCore::Cache,
    mut inEnv: FCore::Graph,
) -> Result<(FCore::Cache, FCore::Graph)> {
    '__tco: loop {
        ::match_deref::match_deref! { match (inVars, inValues) {
            (Deref @ metamodelica::ListNode::Nil, Deref @ metamodelica::ListNode::Nil) => {
                return Ok((inCache, inEnv))
            },
            (Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Var { name, ty, .. }, tail: rest_vars }, Deref @ metamodelica::ListNode::Cons { head: val, tail: rest_vals }) => {
                let mut cr: metamodelica::Ref<DAE::ComponentRef>;
                let mut cache: FCore::Cache;
                let mut env: FCore::Graph;
                cr = ComponentReferenceBasics::makeCrefIdent(name.clone(), ty.clone(), metamodelica::nil());
                (cache, env) = assignVariable(cr, metamodelica::AsArg::as_arg(&val), &inCache, &inEnv)?;
                { (inVars, inValues, inCache, inEnv) = (rest_vars, rest_vals, cache, env); continue '__tco; }
            },
            _ => return Err("match: no arm matched"),
        } }
    }
}

pub fn assignVector(
    mut inNewValue: &metamodelica::Ref<Values::Value>,
    mut inOldValue: &metamodelica::Ref<Values::Value>,
    mut inSubscripts: &metamodelica::List<metamodelica::Ref<DAE::Subscript>>,
    mut inCache: &FCore::Cache,
    mut inEnv: &FCore::Graph,
) -> Result<(FCore::Cache, metamodelica::Ref<Values::Value>)> {
    let mut outCache: FCore::Cache;
    let mut outResult: metamodelica::Ref<Values::Value>;
    (outCache, outResult) = 'mc: {
        let __mc_input = (&**inNewValue, &**inOldValue, &**inSubscripts);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, Deref @ metamodelica::ListNode::Nil) => {
                    Ok((inCache.clone(), inNewValue.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, Deref @ Values::Value::ARRAY { valueLst: values, dimLst: dims }, Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Subscript::INDEX { exp: e }, tail: rest_subs }) => {
                    let mut index: metamodelica::Ref<Values::Value>;
                    let mut val: metamodelica::Ref<Values::Value>;
                    let mut i: i32;
                    let mut cache: FCore::Cache;
                    let mut values = (*values).clone();
                    (cache, index) = cevalExp(e.clone(), inCache.clone(), inEnv.clone())?;
                    i = ValuesUtil::valueInteger(&index)?;
                    val = (values).get(i)?;
                    (cache, val) = assignVector(inNewValue, &val, metamodelica::AsArg::as_arg(&rest_subs), &cache, inEnv)?;
                    values = List::replaceAt(val.clone(), i, values.clone())?;
                    Ok((cache.clone(), metamodelica::Ref::new(Values::Value::ARRAY { valueLst: values.clone(), dimLst: dims.clone() })))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ Values::Value::ARRAY { valueLst: values, .. }, Deref @ Values::Value::ARRAY { valueLst: old_values, dimLst: dims }, Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Subscript::SLICE { exp: e }, tail: rest_subs }) => {
                    let mut values2: metamodelica::List<metamodelica::Ref<Values::Value>>;
                    let mut old_values2: metamodelica::List<metamodelica::Ref<Values::Value>>;
                    let mut indices: metamodelica::List<metamodelica::Ref<Values::Value>>;
                    let mut i: i32;
                    let mut cache: FCore::Cache;
                    let mut values = (*values).clone();
                    let mut old_values = (*old_values).clone();
                    let (__pa0, __pa2, __pa1) = ::match_deref::match_deref! { match &(cevalExp(e.clone(), inCache.clone(), inEnv.clone())?) {
                        (__pa0, Deref @ Values::Value::ARRAY { valueLst: __pa2 @ Deref @ metamodelica::ListNode::Cons { head: Deref @ Values::Value::INTEGER { integer: __pa1 }, tail: _ }, .. }) => (__pa0.clone(), __pa2.clone(), __pa1.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    cache = metamodelica::Own::own(__pa0);
                    i = metamodelica::Own::own(__pa1);
                    indices = metamodelica::Own::own(__pa2);
                    (old_values, old_values2) = List::splitr(old_values.clone(), i - 1)?;
                    (cache, values2) = assignSlice(values.clone(), &old_values2, &indices, metamodelica::AsArg::as_arg(&rest_subs), i, &cache, inEnv)?;
                    values = List::append_reverse(metamodelica::AsArg::as_arg(&old_values), values2.clone());
                    Ok((cache.clone(), metamodelica::Ref::new(Values::Value::ARRAY { valueLst: values.clone(), dimLst: dims.clone() })))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ Values::Value::ARRAY { valueLst: values, .. }, Deref @ Values::Value::ARRAY { valueLst: values2, dimLst: dims }, Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::Subscript::WHOLEDIM { .. }, tail: rest_subs }) => {
                    let mut cache: FCore::Cache;
                    let mut values = (*values).clone();
                    (cache, values) = assignWholeDim(metamodelica::AsArg::as_arg(&values), metamodelica::AsArg::as_arg(&values2), metamodelica::AsArg::as_arg(&rest_subs), inCache, inEnv)?;
                    Ok((cache.clone(), metamodelica::Ref::new(Values::Value::ARRAY { valueLst: values.clone(), dimLst: dims.clone() })))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, Deref @ metamodelica::ListNode::Cons { head: sub, tail: _ }) => {
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { return Err("pattern mismatch") };
                    metamodelica::print(literal!("- CevalFunction.assignVector failed on: "));
                    metamodelica::print({ let mut __mm_s = String::new(); __mm_s.push_str(&*ExpressionBasics::printSubscriptStr(metamodelica::AsArg::as_arg(&sub))?); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) });
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((outCache, outResult))
}

fn assignSlice(
    mut inNewValues: metamodelica::List<metamodelica::Ref<Values::Value>>,
    mut inOldValues: &metamodelica::List<metamodelica::Ref<Values::Value>>,
    mut inIndices: &metamodelica::List<metamodelica::Ref<Values::Value>>,
    mut inSubscripts: &metamodelica::List<metamodelica::Ref<DAE::Subscript>>,
    mut inIndex: i32,
    mut inCache: &FCore::Cache,
    mut inEnv: &FCore::Graph,
) -> Result<(FCore::Cache, metamodelica::List<metamodelica::Ref<Values::Value>>)> {
    let mut outCache: FCore::Cache;
    let mut outResult: metamodelica::List<metamodelica::Ref<Values::Value>>;
    (outCache, outResult) = 'mc: {
        let __mc_input = (inNewValues, &**inOldValues, &**inIndices);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (_, _, Deref @ metamodelica::ListNode::Nil) => {
                    Ok((inCache.clone(), inOldValues.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (vl1, Deref @ metamodelica::ListNode::Cons { head: v2, tail: vl2 }, Deref @ metamodelica::ListNode::Cons { head: index, tail: _ }) => {
                    let mut cache: FCore::Cache;
                    let mut vl1 = (*vl1).clone();
                    let true = (inIndex < ValuesUtil::valueInteger(metamodelica::AsArg::as_arg(&index))?) else { return Err("pattern mismatch") };
                    (cache, vl1) = assignSlice(vl1.clone(), metamodelica::AsArg::as_arg(&vl2), inIndices, inSubscripts, inIndex + 1, inCache, inEnv)?;
                    Ok((cache.clone(), metamodelica::cons(v2.clone(), vl1.clone())))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: v1, tail: vl1 }, Deref @ metamodelica::ListNode::Cons { head: v2, tail: vl2 }, Deref @ metamodelica::ListNode::Cons { head: _, tail: rest_indices }) => {
                    let mut cache: FCore::Cache;
                    let mut v1 = (*v1).clone();
                    let mut vl1 = (*vl1).clone();
                    (cache, v1) = assignVector(metamodelica::AsArg::as_arg(&v1), metamodelica::AsArg::as_arg(&v2), inSubscripts, inCache, inEnv)?;
                    (cache, vl1) = assignSlice(vl1.clone(), metamodelica::AsArg::as_arg(&vl2), metamodelica::AsArg::as_arg(&rest_indices), inSubscripts, inIndex + 1, inCache, inEnv)?;
                    Ok((cache.clone(), metamodelica::cons(v1.clone(), vl1.clone())))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((outCache, outResult))
}

fn assignWholeDim(
    mut inNewValues: &metamodelica::List<metamodelica::Ref<Values::Value>>,
    mut inOldValues: &metamodelica::List<metamodelica::Ref<Values::Value>>,
    mut inSubscripts: &metamodelica::List<metamodelica::Ref<DAE::Subscript>>,
    mut inCache: &FCore::Cache,
    mut inEnv: &FCore::Graph,
) -> Result<(FCore::Cache, metamodelica::List<metamodelica::Ref<Values::Value>>)> {
    let mut outCache: FCore::Cache;
    let mut outResult: metamodelica::List<metamodelica::Ref<Values::Value>>;
    (outCache, outResult) = (::match_deref::match_deref! { match (inNewValues, inOldValues) {
        (Deref @ metamodelica::ListNode::Nil, _) => {
            (inCache.clone(), metamodelica::nil())
        },
        (Deref @ metamodelica::ListNode::Cons { head: v1, tail: vl1 }, Deref @ metamodelica::ListNode::Cons { head: v2, tail: vl2 }) => {
            let mut cache: FCore::Cache;
            let mut v1 = (*v1).clone();
            let mut vl1 = (*vl1).clone();
            (cache, v1) = assignVector(metamodelica::AsArg::as_arg(&v1), metamodelica::AsArg::as_arg(&v2), inSubscripts, inCache, inEnv)?;
            (cache, vl1) = assignWholeDim(metamodelica::AsArg::as_arg(&vl1), vl2, inSubscripts, inCache, inEnv)?;
            (cache, metamodelica::cons(v1.clone(), vl1.clone()))
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok((outCache, outResult))
}

fn updateVariableBinding(
    mut inVariableCref: &metamodelica::Ref<DAE::ComponentRef>,
    mut inEnv: FCore::Graph,
    mut inType: metamodelica::Ref<DAE::Type>,
    mut inNewValue: metamodelica::Ref<Values::Value>,
) -> Result<FCore::Graph> {
    let mut outEnv: FCore::Graph;
    let mut var_name: ArcStr;
    let mut var: metamodelica::Ref<DAE::Var>;
    var_name = ComponentReference::crefStr(inVariableCref)?;
    var = makeFunctionVariable(
        var_name,
        inType,
        metamodelica::Ref::new(DAE::Binding::VALBOUND {
            valBound: inNewValue,
            source: openmodelica_frontend_types::DAE::BindingSource::BINDING_FROM_DEFAULT_VALUE,
        }),
    );
    outEnv = FGraph::updateComp(
        inEnv,
        var,
        &(openmodelica_frontend_dump::FCore::Status::VAR_TYPED),
        &(FGraph::empty()),
    );
    Ok(outEnv)
}

fn updateRecordBinding(
    mut inVar: metamodelica::Ref<DAE::Var>,
    mut inValue: metamodelica::Ref<Values::Value>,
) -> metamodelica::Ref<DAE::Var> {
    let mut outVar: metamodelica::Ref<DAE::Var>;
    outVar = inVar;
    assign_field!(
        outVar.binding = metamodelica::Ref::new(DAE::Binding::VALBOUND {
            valBound: inValue,
            source: openmodelica_frontend_types::DAE::BindingSource::BINDING_FROM_DEFAULT_VALUE
        })
    );
    outVar
}

fn updateRecordComponentBinding(
    mut inVar: metamodelica::Ref<DAE::Var>,
    mut inComponentId: ArcStr,
    mut inValue: metamodelica::Ref<Values::Value>,
) -> Result<metamodelica::Ref<DAE::Var>> {
    let mut outVar: metamodelica::Ref<DAE::Var>;
    let mut val: metamodelica::Ref<Values::Value>;
    outVar = inVar;
    val = getBindingOrDefault(&outVar.binding, &outVar.ty)?;
    val = updateRecordComponentValue(inComponentId, inValue, &val)?;
    assign_field!(
        outVar.binding = metamodelica::Ref::new(DAE::Binding::VALBOUND {
            valBound: val,
            source: openmodelica_frontend_types::DAE::BindingSource::BINDING_FROM_DEFAULT_VALUE
        })
    );
    Ok(outVar)
}

fn updateRecordComponentValue(
    mut inComponentId: ArcStr,
    mut inComponentValue: metamodelica::Ref<Values::Value>,
    mut inRecordValue: &metamodelica::Ref<Values::Value>,
) -> Result<metamodelica::Ref<Values::Value>> {
    let mut outRecordValue: metamodelica::Ref<Values::Value>;
    let mut name: metamodelica::Ref<Absyn::Path>;
    let mut vals: metamodelica::List<metamodelica::Ref<Values::Value>>;
    let mut comps: metamodelica::List<ArcStr>;
    let mut pos: i32;
    let (__pa0, __pa1, __pa2) = ::match_deref::match_deref! { match &((*inRecordValue)) {
        Deref @ Values::Value::RECORD { record_: __pa0, orderd: __pa1, comp: __pa2, index: (-1) } => (__pa0.clone(), __pa1.clone(), __pa2.clone()),
        _ => return Err("pattern mismatch"),
    } };
    name = metamodelica::Own::own(__pa0);
    vals = metamodelica::Own::own(__pa1);
    comps = metamodelica::Own::own(__pa2);
    pos = List::position(inComponentId, &comps)?;
    vals = List::replaceAt(inComponentValue, pos, vals)?;
    outRecordValue = metamodelica::Ref::new(Values::Value::RECORD {
        record_: name,
        orderd: vals,
        comp: comps,
        index: -1,
    });
    Ok(outRecordValue)
}

fn getVariableTypeAndBinding(
    mut inCref: metamodelica::Ref<DAE::ComponentRef>,
    mut inEnv: FCore::Graph,
) -> Result<(metamodelica::Ref<DAE::Type>, metamodelica::Ref<DAE::Binding>)> {
    let mut outType: metamodelica::Ref<DAE::Type>;
    let mut outBinding: metamodelica::Ref<DAE::Binding>;
    (_, _, outType, outBinding, _, _, _, _, _) = Lookup::lookupVar(FCore::emptyCache(), inEnv, inCref)?;
    Ok((outType, outBinding))
}

fn getVariableTypeAndValue(
    mut inCref: metamodelica::Ref<DAE::ComponentRef>,
    mut inEnv: FCore::Graph,
) -> Result<(metamodelica::Ref<DAE::Type>, metamodelica::Ref<Values::Value>)> {
    let mut outType: metamodelica::Ref<DAE::Type>;
    let mut outValue: metamodelica::Ref<Values::Value>;
    let mut binding: metamodelica::Ref<DAE::Binding>;
    (outType, binding) = getVariableTypeAndBinding(inCref, inEnv)?;
    outValue = getBindingOrDefault(&binding, &outType)?;
    Ok((outType, outValue))
}

fn getBindingValueOpt(mut inBinding: &metamodelica::Ref<DAE::Binding>) -> Option<metamodelica::Ref<Values::Value>> {
    let mut outValue: Option<metamodelica::Ref<Values::Value>>;
    outValue = (::match_deref::match_deref! { match inBinding {
        Deref @ DAE::Binding::VALBOUND { valBound: val, .. } => {
            Some(val.clone())
        },
        Deref @ DAE::Binding::EQBOUND { evaluatedExp: Some(val), .. } => {
            Some(val.clone())
        },
        _ => {
            None
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outValue
}

fn getBindingOrDefault(
    mut inBinding: &metamodelica::Ref<DAE::Binding>,
    mut inType: &metamodelica::Ref<DAE::Type>,
) -> Result<metamodelica::Ref<Values::Value>> {
    let mut outValue: metamodelica::Ref<Values::Value>;
    outValue = (::match_deref::match_deref! { match inBinding {
        Deref @ DAE::Binding::VALBOUND { valBound: val, .. } => {
            val.clone()
        },
        Deref @ DAE::Binding::EQBOUND { evaluatedExp: Some(val), .. } => {
            val.clone()
        },
        _ => {
            generateDefaultBinding(inType)?
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outValue)
}

fn generateDefaultBinding(mut inType: &metamodelica::Ref<DAE::Type>) -> Result<metamodelica::Ref<Values::Value>> {
    let mut outValue: metamodelica::Ref<Values::Value>;
    outValue = 'mc: {
        let __mc_input = &**inType;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Type::T_INTEGER { .. } => {
                    Ok(metamodelica::Ref::new(Values::Value::INTEGER { integer: 0 }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Type::T_REAL { .. } => {
                    Ok(metamodelica::Ref::new(Values::Value::REAL { real: metamodelica::OrderedFloat(0.0_f64) }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Type::T_STRING { .. } => {
                    Ok(metamodelica::Ref::new(Values::Value::STRING { string: literal!("") }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Type::T_BOOL { .. } => {
                    Ok(metamodelica::Ref::new(Values::Value::BOOL { boolean: false }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Type::T_ENUMERATION { .. } => {
                    Ok(metamodelica::Ref::new(Values::Value::ENUM_LITERAL { name: metamodelica::Ref::new(Absyn::Path::IDENT { name: literal!("") }), index: 0 }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Type::T_ARRAY { dims: Deref @ metamodelica::ListNode::Cons { head: dim, tail: Deref @ metamodelica::ListNode::Nil }, ty } => {
                    let mut int_dim: i32;
                    let mut dims: metamodelica::List<i32>;
                    let mut values: metamodelica::List<metamodelica::Ref<Values::Value>>;
                    let mut value: metamodelica::Ref<Values::Value>;
                    int_dim = Expression::dimensionSize(metamodelica::AsArg::as_arg(&dim))?;
                    value = generateDefaultBinding(metamodelica::AsArg::as_arg(&ty))?;
                    values = List::fill(value.clone(), int_dim);
                    dims = ValuesUtil::valueDimensions(&value);
                    Ok(metamodelica::Ref::new(Values::Value::ARRAY { valueLst: values.clone(), dimLst: metamodelica::cons(int_dim, dims.clone()) }))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Type::T_COMPLEX { complexClassType: ClassInf::State::RECORD { path }, varLst: vars, .. } => {
                    let mut values: metamodelica::List<metamodelica::Ref<Values::Value>>;
                    let mut var_names: metamodelica::List<ArcStr>;
                    (values, var_names) = List::map_2(metamodelica::AsArg::as_arg(&vars), &move |__a0: metamodelica::Ref<DAE::Var>| getRecordVarBindingAndName(&__a0))?;
                    Ok(metamodelica::Ref::new(Values::Value::RECORD { record_: path.clone(), orderd: values.clone(), comp: var_names.clone(), index: -1 }))
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
                    Debug::trace(literal!("- CevalFunction.generateDefaultBinding failed\n"))?;
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(outValue)
}

fn getRecordVarBindingAndName(
    mut inVar: &metamodelica::Ref<DAE::Var>,
) -> Result<(metamodelica::Ref<Values::Value>, ArcStr)> {
    let mut outBinding: metamodelica::Ref<Values::Value>;
    let mut outName: ArcStr;
    (outBinding, outName) = 'mc: {
        let __mc_input = &**inVar;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Var { name, ty, binding, .. } => {
                    let mut val: metamodelica::Ref<Values::Value>;
                    val = getBindingOrDefault(metamodelica::AsArg::as_arg(&binding), metamodelica::AsArg::as_arg(&ty))?;
                    Ok((val.clone(), name.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Var { name, .. } => {
                    let true = (Flags::isSet(Flags::FAILTRACE.clone())?) else { return Err("pattern mismatch") };
                    Debug::traceln({ let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("- CevalFunction.getRecordVarBindingAndName failed on variable ")); __mm_s.push_str(&*name); __mm_s.push_str(&*literal!("\n")); ArcStr::from(__mm_s) })?;
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok((outBinding, outName))
}

fn getFunctionReturnValue(
    mut inOutputVar: &metamodelica::Ref<DAE::Element>,
    mut inEnv: FCore::Graph,
) -> Result<metamodelica::Ref<Values::Value>> {
    let mut outValue: metamodelica::Ref<Values::Value>;
    outValue = (match &**inOutputVar {
        DAE::Element::VAR {
            componentRef: cr, ty, ..
        } => {
            let mut val: metamodelica::Ref<Values::Value>;
            val = getVariableValue(cr.clone(), ty, inEnv)?;
            val
        }
        _ => return Err("match: no arm matched"),
    });
    Ok(outValue)
}

fn getVariableValue(
    mut inCref: metamodelica::Ref<DAE::ComponentRef>,
    mut inType: &metamodelica::Ref<DAE::Type>,
    mut inEnv: FCore::Graph,
) -> Result<metamodelica::Ref<Values::Value>> {
    let mut outValue: metamodelica::Ref<Values::Value>;
    outValue = 'mc: {
        let __mc_input = &**inType;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                Deref @ DAE::Type::T_COMPLEX { complexClassType: ClassInf::State::RECORD { .. }, .. } => {
                    let mut val: metamodelica::Ref<Values::Value>;
                    let mut p: metamodelica::Ref<Absyn::Path>;
                    p = ComponentReference::crefToPath(&inCref)?;
                    val = getRecordValue(&p, inType, &inEnv)?;
                    Ok(val.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut val: metamodelica::Ref<Values::Value>;
                    (_, val) = getVariableTypeAndValue(inCref.clone(), inEnv.clone())?;
                    Ok(val.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(outValue)
}

fn getRecordValue(
    mut inRecordName: &metamodelica::Ref<Absyn::Path>,
    mut inType: &metamodelica::Ref<DAE::Type>,
    mut inEnv: &FCore::Graph,
) -> Result<metamodelica::Ref<Values::Value>> {
    let mut outValue: metamodelica::Ref<Values::Value>;
    outValue = (::match_deref::match_deref! { match (inRecordName, inType) {
        (Deref @ Absyn::Path::IDENT { name: id }, Deref @ DAE::Type::T_COMPLEX { complexClassType: ClassInf::State::RECORD { path: p }, varLst: vars, .. }) => {
            let mut vals: metamodelica::List<metamodelica::Ref<Values::Value>>;
            let mut var_names: metamodelica::List<ArcStr>;
            let mut env: FCore::Graph;
            (_, _, _, _, _, env) = Lookup::lookupIdentLocal(FCore::emptyCache(), inEnv, id.clone())?;
            vals = List::map1(vars.clone(), &move |__a0: metamodelica::Ref<DAE::Var>, __a1: FCore::Graph| getRecordComponentValue(&__a0, &__a1), env)?;
            var_names = List::map(vars.clone(), &move |__a0: metamodelica::Ref<DAE::Var>| -> metamodelica::Result<_> { ::std::result::Result::Ok(TypesDump::getVarName(&__a0)) })?;
            metamodelica::Ref::new(Values::Value::RECORD { record_: p.clone(), orderd: vals, comp: var_names, index: -1 })
        },
        _ => return Err("match: no arm matched"),
    } });
    Ok(outValue)
}

fn getRecordComponentValue(
    mut inVars: &metamodelica::Ref<DAE::Var>,
    mut inEnv: &FCore::Graph,
) -> Result<metamodelica::Ref<Values::Value>> {
    let mut outValues: metamodelica::Ref<Values::Value>;
    outValues = (::match_deref::match_deref! { match inVars {
        Deref @ DAE::Var { name: id, ty: ty @ Deref @ DAE::Type::T_COMPLEX { complexClassType: ClassInf::State::RECORD { .. }, .. }, .. } => {
            let mut val: metamodelica::Ref<Values::Value>;
            val = getRecordValue(&(metamodelica::Ref::new(Absyn::Path::IDENT { name: id.clone() })), metamodelica::AsArg::as_arg(&ty), inEnv)?;
            val
        },
        Deref @ DAE::Var { name: id, ty, binding: tvbinding, .. } => {
            let mut val: metamodelica::Ref<Values::Value>;
            let mut oval: Option<metamodelica::Ref<Values::Value>>;
            let mut binding: metamodelica::Ref<DAE::Binding>;
            let (_, __t1, _, _, _, _) = Lookup::lookupIdentLocal(FCore::emptyCache(), inEnv, id.clone())?;
            let __arc2 = __t1.clone();
            let DAE::TYPES_VAR { binding: __pa0, .. } = &*__arc2;
            binding = metamodelica::Own::own(__pa0);
            oval = getBindingValueOpt(&binding);
            if (oval).is_none() {
                oval = getBindingValueOpt(tvbinding);
            }
            if (oval).is_some() {
                let __pa3 = ::match_deref::match_deref! { match &(oval) {
                    Some(__pa3) => __pa3.clone(),
                    _ => return Err("pattern mismatch"),
                } };
                val = metamodelica::Own::own(__pa3);
            } else {
                val = generateDefaultBinding(ty)?;
            }
            val
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(outValues)
}

fn boxReturnValue(
    mut inReturnValues: metamodelica::List<metamodelica::Ref<Values::Value>>,
) -> metamodelica::Ref<Values::Value> {
    let mut outValue: metamodelica::Ref<Values::Value>;
    outValue = (::match_deref::match_deref! { match &(inReturnValues.clone()) {
        Deref @ metamodelica::ListNode::Nil => {
            openmodelica_frontend_types::Values::Value::interned_NORETCALL()
        },
        Deref @ metamodelica::ListNode::Cons { head: val, tail: Deref @ metamodelica::ListNode::Nil } => {
            val.clone()
        },
        Deref @ metamodelica::ListNode::Cons { head: _, tail: _ } => {
            metamodelica::Ref::new(Values::Value::TUPLE { valueLst: inReturnValues })
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    outValue
}

// [DEPS]  Function variable dependency handling.
fn sortFunctionVarsByDependency(
    mut inFuncVars: metamodelica::List<(
        metamodelica::Ref<DAE::Element>,
        Option<metamodelica::Ref<Values::Value>>,
    )>,
    mut inSource: metamodelica::Ref<DAE::ElementSource>,
) -> Result<
    metamodelica::List<(
        metamodelica::Ref<DAE::Element>,
        Option<metamodelica::Ref<Values::Value>>,
    )>,
> {
    let mut outFuncVars: metamodelica::List<(
        metamodelica::Ref<DAE::Element>,
        Option<metamodelica::Ref<Values::Value>>,
    )>;
    let mut cycles: metamodelica::List<(
        (
            metamodelica::Ref<DAE::Element>,
            Option<metamodelica::Ref<Values::Value>>,
        ),
        metamodelica::List<(
            metamodelica::Ref<DAE::Element>,
            Option<metamodelica::Ref<Values::Value>>,
        )>,
    )>;
    (outFuncVars, cycles) = Graph::topologicalSort(
        &(Graph::buildGraph(
            inFuncVars.clone(),
            &move |__a0: (
                metamodelica::Ref<DAE::Element>,
                Option<metamodelica::Ref<Values::Value>>,
            ),
                   __a1: metamodelica::List<(
                metamodelica::Ref<DAE::Element>,
                Option<metamodelica::Ref<Values::Value>>,
            )>|
                  -> metamodelica::Result<_> {
                ::std::result::Result::Ok(getElementDependencies(&__a0, __a1))
            },
            inFuncVars,
        )?),
        (std::sync::Arc::new(
            move |__a0: (
                metamodelica::Ref<DAE::Element>,
                Option<metamodelica::Ref<Values::Value>>,
            ),
                  __a1: (
                metamodelica::Ref<DAE::Element>,
                Option<metamodelica::Ref<Values::Value>>,
            )| isElementEqual(&__a0, &__a1),
        )
            as std::sync::Arc<
                dyn ::std::ops::Fn(
                        (
                            metamodelica::Ref<DAE::Element>,
                            Option<metamodelica::Ref<Values::Value>>,
                        ),
                        (
                            metamodelica::Ref<DAE::Element>,
                            Option<metamodelica::Ref<Values::Value>>,
                        ),
                    ) -> Result<bool>
                    + 'static,
            >),
    )?;
    checkCyclicalComponents(&cycles, inSource)?;
    Ok(outFuncVars)
}

fn getElementDependencies(
    mut inElement: &FunctionVar,
    mut inAllElements: metamodelica::List<(
        metamodelica::Ref<DAE::Element>,
        Option<metamodelica::Ref<Values::Value>>,
    )>,
) -> metamodelica::List<(
    metamodelica::Ref<DAE::Element>,
    Option<metamodelica::Ref<Values::Value>>,
)> {
    pub(crate) type Arg = (
        metamodelica::List<(
            metamodelica::Ref<DAE::Element>,
            Option<metamodelica::Ref<Values::Value>>,
        )>,
        metamodelica::List<(
            metamodelica::Ref<DAE::Element>,
            Option<metamodelica::Ref<Values::Value>>,
        )>,
        metamodelica::List<ArcStr>,
    );

    let mut outDependencies: metamodelica::List<(
        metamodelica::Ref<DAE::Element>,
        Option<metamodelica::Ref<Values::Value>>,
    )>;
    outDependencies = 'mc: {
        let __mc_input = inElement;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Element::VAR { binding: Some(bind_exp), dims, .. }, _) => {
                    let mut deps: metamodelica::List<(metamodelica::Ref<DAE::Element>, Option<metamodelica::Ref<Values::Value>>)>;
                    let mut arg: (metamodelica::List<(metamodelica::Ref<DAE::Element>, Option<metamodelica::Ref<Values::Value>>)>, metamodelica::List<(metamodelica::Ref<DAE::Element>, Option<metamodelica::Ref<Values::Value>>)>, metamodelica::List<ArcStr>);
                    let (_, ref __pa1 @ (_, ref __pa0, _)) = Expression::traverseExpBidir(bind_exp.clone(), (std::sync::Arc::new(fnptr!(getElementDependenciesTraverserEnter, metamodelica::Ref<DAE::Exp>, (metamodelica::List<(metamodelica::Ref<DAE::Element>, Option<metamodelica::Ref<Values::Value>>)>, metamodelica::List<(metamodelica::Ref<DAE::Element>, Option<metamodelica::Ref<Values::Value>>)>, metamodelica::List<ArcStr>))) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>, (metamodelica::List<(metamodelica::Ref<DAE::Element>, Option<metamodelica::Ref<Values::Value>>)>, metamodelica::List<(metamodelica::Ref<DAE::Element>, Option<metamodelica::Ref<Values::Value>>)>, metamodelica::List<ArcStr>)) -> Result<(metamodelica::Ref<DAE::Exp>, (metamodelica::List<(metamodelica::Ref<DAE::Element>, Option<metamodelica::Ref<Values::Value>>)>, metamodelica::List<(metamodelica::Ref<DAE::Element>, Option<metamodelica::Ref<Values::Value>>)>, metamodelica::List<ArcStr>))> + 'static>), (std::sync::Arc::new(getElementDependenciesTraverserExit) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>, (metamodelica::List<(metamodelica::Ref<DAE::Element>, Option<metamodelica::Ref<Values::Value>>)>, metamodelica::List<(metamodelica::Ref<DAE::Element>, Option<metamodelica::Ref<Values::Value>>)>, metamodelica::List<ArcStr>)) -> Result<(metamodelica::Ref<DAE::Exp>, (metamodelica::List<(metamodelica::Ref<DAE::Element>, Option<metamodelica::Ref<Values::Value>>)>, metamodelica::List<(metamodelica::Ref<DAE::Element>, Option<metamodelica::Ref<Values::Value>>)>, metamodelica::List<ArcStr>))> + 'static>), (inAllElements.clone(), metamodelica::nil(), metamodelica::nil()))?;
                    deps = metamodelica::Own::own(__pa0);
                    arg = metamodelica::Own::own(__pa1);
                    let (_, (_, __pa2, _)) = List::mapFold(metamodelica::AsArg::as_arg(&dims), &fnptr!(getElementDependenciesFromDims, metamodelica::Ref<DAE::Dimension>, (metamodelica::List<(metamodelica::Ref<DAE::Element>, Option<metamodelica::Ref<Values::Value>>)>, metamodelica::List<(metamodelica::Ref<DAE::Element>, Option<metamodelica::Ref<Values::Value>>)>, metamodelica::List<ArcStr>)), arg.clone())?;
                    deps = metamodelica::Own::own(__pa2);
                    Ok(deps.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ DAE::Element::VAR { dims, .. }, _) => {
                    let mut deps: metamodelica::List<(metamodelica::Ref<DAE::Element>, Option<metamodelica::Ref<Values::Value>>)>;
                    let (_, (_, __pa0, _)) = List::mapFold(metamodelica::AsArg::as_arg(&dims), &fnptr!(getElementDependenciesFromDims, metamodelica::Ref<DAE::Dimension>, (metamodelica::List<(metamodelica::Ref<DAE::Element>, Option<metamodelica::Ref<Values::Value>>)>, metamodelica::List<(metamodelica::Ref<DAE::Element>, Option<metamodelica::Ref<Values::Value>>)>, metamodelica::List<ArcStr>)), (inAllElements.clone(), metamodelica::nil(), metamodelica::nil()))?;
                    deps = metamodelica::Own::own(__pa0);
                    Ok(deps.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok(metamodelica::nil())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    outDependencies
}

fn getElementDependenciesFromDims(
    mut inDimension: metamodelica::Ref<DAE::Dimension>,
    mut inArg: (
        metamodelica::List<(
            metamodelica::Ref<DAE::Element>,
            Option<metamodelica::Ref<Values::Value>>,
        )>,
        metamodelica::List<(
            metamodelica::Ref<DAE::Element>,
            Option<metamodelica::Ref<Values::Value>>,
        )>,
        metamodelica::List<ArcStr>,
    ),
) -> (
    metamodelica::Ref<DAE::Dimension>,
    (
        metamodelica::List<(
            metamodelica::Ref<DAE::Element>,
            Option<metamodelica::Ref<Values::Value>>,
        )>,
        metamodelica::List<(
            metamodelica::Ref<DAE::Element>,
            Option<metamodelica::Ref<Values::Value>>,
        )>,
        metamodelica::List<ArcStr>,
    ),
) {
    pub(crate) type Arg = (
        metamodelica::List<(
            metamodelica::Ref<DAE::Element>,
            Option<metamodelica::Ref<Values::Value>>,
        )>,
        metamodelica::List<(
            metamodelica::Ref<DAE::Element>,
            Option<metamodelica::Ref<Values::Value>>,
        )>,
        metamodelica::List<ArcStr>,
    );

    let mut outDimension: metamodelica::Ref<DAE::Dimension>;
    let mut outArg: (
        metamodelica::List<(
            metamodelica::Ref<DAE::Element>,
            Option<metamodelica::Ref<Values::Value>>,
        )>,
        metamodelica::List<(
            metamodelica::Ref<DAE::Element>,
            Option<metamodelica::Ref<Values::Value>>,
        )>,
        metamodelica::List<ArcStr>,
    );
    (outDimension, outArg) = 'mc: {
        let __mc_input = &inArg;
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    let mut arg: (metamodelica::List<(metamodelica::Ref<DAE::Element>, Option<metamodelica::Ref<Values::Value>>)>, metamodelica::List<(metamodelica::Ref<DAE::Element>, Option<metamodelica::Ref<Values::Value>>)>, metamodelica::List<ArcStr>);
                    let mut dim_exp: metamodelica::Ref<DAE::Exp>;
                    dim_exp = Expression::dimensionSizeExp(&inDimension)?;
                    (_, arg) = Expression::traverseExpBidir(dim_exp.clone(), (std::sync::Arc::new(fnptr!(getElementDependenciesTraverserEnter, metamodelica::Ref<DAE::Exp>, (metamodelica::List<(metamodelica::Ref<DAE::Element>, Option<metamodelica::Ref<Values::Value>>)>, metamodelica::List<(metamodelica::Ref<DAE::Element>, Option<metamodelica::Ref<Values::Value>>)>, metamodelica::List<ArcStr>))) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>, (metamodelica::List<(metamodelica::Ref<DAE::Element>, Option<metamodelica::Ref<Values::Value>>)>, metamodelica::List<(metamodelica::Ref<DAE::Element>, Option<metamodelica::Ref<Values::Value>>)>, metamodelica::List<ArcStr>)) -> Result<(metamodelica::Ref<DAE::Exp>, (metamodelica::List<(metamodelica::Ref<DAE::Element>, Option<metamodelica::Ref<Values::Value>>)>, metamodelica::List<(metamodelica::Ref<DAE::Element>, Option<metamodelica::Ref<Values::Value>>)>, metamodelica::List<ArcStr>))> + 'static>), (std::sync::Arc::new(getElementDependenciesTraverserExit) as std::sync::Arc<dyn ::std::ops::Fn(metamodelica::Ref<DAE::Exp>, (metamodelica::List<(metamodelica::Ref<DAE::Element>, Option<metamodelica::Ref<Values::Value>>)>, metamodelica::List<(metamodelica::Ref<DAE::Element>, Option<metamodelica::Ref<Values::Value>>)>, metamodelica::List<ArcStr>)) -> Result<(metamodelica::Ref<DAE::Exp>, (metamodelica::List<(metamodelica::Ref<DAE::Element>, Option<metamodelica::Ref<Values::Value>>)>, metamodelica::List<(metamodelica::Ref<DAE::Element>, Option<metamodelica::Ref<Values::Value>>)>, metamodelica::List<ArcStr>))> + 'static>), inArg.clone())?;
                    Ok((inDimension.clone(), arg.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((inDimension.clone(), inArg.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    (outDimension, outArg)
}

fn getElementDependenciesTraverserEnter(
    mut inExp: metamodelica::Ref<DAE::Exp>,
    mut inArg: (
        metamodelica::List<(
            metamodelica::Ref<DAE::Element>,
            Option<metamodelica::Ref<Values::Value>>,
        )>,
        metamodelica::List<(
            metamodelica::Ref<DAE::Element>,
            Option<metamodelica::Ref<Values::Value>>,
        )>,
        metamodelica::List<ArcStr>,
    ),
) -> (
    metamodelica::Ref<DAE::Exp>,
    (
        metamodelica::List<(
            metamodelica::Ref<DAE::Element>,
            Option<metamodelica::Ref<Values::Value>>,
        )>,
        metamodelica::List<(
            metamodelica::Ref<DAE::Element>,
            Option<metamodelica::Ref<Values::Value>>,
        )>,
        metamodelica::List<ArcStr>,
    ),
) {
    pub(crate) type Arg = (
        metamodelica::List<(
            metamodelica::Ref<DAE::Element>,
            Option<metamodelica::Ref<Values::Value>>,
        )>,
        metamodelica::List<(
            metamodelica::Ref<DAE::Element>,
            Option<metamodelica::Ref<Values::Value>>,
        )>,
        metamodelica::List<ArcStr>,
    );

    let mut outExp: metamodelica::Ref<DAE::Exp>;
    let mut outArg: (
        metamodelica::List<(
            metamodelica::Ref<DAE::Element>,
            Option<metamodelica::Ref<Values::Value>>,
        )>,
        metamodelica::List<(
            metamodelica::Ref<DAE::Element>,
            Option<metamodelica::Ref<Values::Value>>,
        )>,
        metamodelica::List<ArcStr>,
    );
    (outExp, outArg) = 'mc: {
        let __mc_input = (inExp.clone(), &inArg);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (exp @ Deref @ DAE::Exp::CREF { componentRef: Deref @ DAE::ComponentRef::CREF_IDENT { ident: iter, .. }, .. }, (all_el, accum_el, iters @ Deref @ metamodelica::ListNode::Cons { head: _, tail: _ })) => {
                    let true = (List::isMemberOnTrue(iter.clone(), metamodelica::AsArg::as_arg(&iters), &fnptr!(stringEqual, ArcStr, ArcStr))?) else { return Err("pattern mismatch") };
                    Ok((exp.clone(), (all_el.clone(), accum_el.clone(), iters.clone())))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (exp @ Deref @ DAE::Exp::CREF { componentRef: cref, .. }, (all_el, accum_el, iters)) => {
                    let mut e: FunctionVar;
                    let mut all_el = (*all_el).clone();
                    let (__pa0, __pa1) = ::match_deref::match_deref! { match &(List::deleteMemberOnTrue(cref.clone(), all_el.clone(), &move |__a0: metamodelica::Ref<DAE::ComponentRef>, __a1: (metamodelica::Ref<DAE::Element>, Option<metamodelica::Ref<Values::Value>>)| isElementNamed(__a0, &__a1))?) {
                        (__pa0, Some(__pa1)) => (__pa0.clone(), __pa1.clone()),
                        _ => return Err("pattern mismatch"),
                    } };
                    all_el = metamodelica::Own::own(__pa0);
                    e = metamodelica::Own::own(__pa1);
                    Ok((exp.clone(), (all_el.clone(), metamodelica::cons(e.clone(), accum_el.clone()), iters.clone())))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (exp @ Deref @ DAE::Exp::REDUCTION { iterators: riters, .. }, (all_el, accum_el, iters)) => {
                    let mut iters = (*iters).clone();
                    iters = listAppend(List::map(riters.clone(), &move |__a0: metamodelica::Ref<DAE::ReductionIterator>| -> metamodelica::Result<_> { ::std::result::Result::Ok(Expression::reductionIterName(&__a0)) })?, iters.clone());
                    Ok((exp.clone(), (all_el.clone(), accum_el.clone(), iters.clone())))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Ok((inExp.clone(), inArg.clone()))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        panic!("matchcontinue: no arm matched")
    };
    (outExp, outArg)
}

fn getElementDependenciesTraverserExit(
    mut inExp: metamodelica::Ref<DAE::Exp>,
    mut inArg: (
        metamodelica::List<(
            metamodelica::Ref<DAE::Element>,
            Option<metamodelica::Ref<Values::Value>>,
        )>,
        metamodelica::List<(
            metamodelica::Ref<DAE::Element>,
            Option<metamodelica::Ref<Values::Value>>,
        )>,
        metamodelica::List<ArcStr>,
    ),
) -> Result<(
    metamodelica::Ref<DAE::Exp>,
    (
        metamodelica::List<(
            metamodelica::Ref<DAE::Element>,
            Option<metamodelica::Ref<Values::Value>>,
        )>,
        metamodelica::List<(
            metamodelica::Ref<DAE::Element>,
            Option<metamodelica::Ref<Values::Value>>,
        )>,
        metamodelica::List<ArcStr>,
    ),
)> {
    pub(crate) type Arg = (
        metamodelica::List<(
            metamodelica::Ref<DAE::Element>,
            Option<metamodelica::Ref<Values::Value>>,
        )>,
        metamodelica::List<(
            metamodelica::Ref<DAE::Element>,
            Option<metamodelica::Ref<Values::Value>>,
        )>,
        metamodelica::List<ArcStr>,
    );

    let mut outExp: metamodelica::Ref<DAE::Exp>;
    let mut outArg: (
        metamodelica::List<(
            metamodelica::Ref<DAE::Element>,
            Option<metamodelica::Ref<Values::Value>>,
        )>,
        metamodelica::List<(
            metamodelica::Ref<DAE::Element>,
            Option<metamodelica::Ref<Values::Value>>,
        )>,
        metamodelica::List<ArcStr>,
    );
    (outExp, outArg) = (::match_deref::match_deref! { match &((inExp.clone(), inArg.clone())) {
        (exp @ Deref @ DAE::Exp::REDUCTION { iterators: riters, .. }, (all_el, accum_el, iters)) => {
            let mut iters = (*iters).clone();
            iters = compareIterators(&(riters.clone().reverse()), metamodelica::AsArg::as_arg(&iters))?;
            (exp.clone(), (all_el.clone(), accum_el.clone(), iters.clone()))
        },
        _ => {
            (inExp, inArg)
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outExp, outArg))
}

fn compareIterators(
    mut inRiters: &metamodelica::List<metamodelica::Ref<DAE::ReductionIterator>>,
    mut inIters: &metamodelica::List<ArcStr>,
) -> Result<metamodelica::List<ArcStr>> {
    let mut outIters: metamodelica::List<ArcStr>;
    outIters = 'mc: {
        let __mc_input = (&**inRiters, &**inIters);
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Cons { head: Deref @ DAE::ReductionIterator { id: id1, .. }, tail: riters }, Deref @ metamodelica::ListNode::Cons { head: id2, tail: iters }) => {
                    let true = (stringEqual(&id1, &id2)) else { return Err("pattern mismatch") };
                    Ok(compareIterators(metamodelica::AsArg::as_arg(&riters), metamodelica::AsArg::as_arg(&iters))?)
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                (Deref @ metamodelica::ListNode::Nil, _) => {
                    Ok(inIters.clone())
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        if let Ok(__v) = (|| -> Result<_> {
            ::match_deref::match_deref! { match &__mc_input {
                _ => {
                    Error::addMessage(Error::INTERNAL_ERROR.clone(), list![literal!("Different iterators in CevalFunction.compareIterators.")])?;
                    Ok(return Err("fail"))
                }
                _ => return Err("nomatch"),
            }}
        })() {
            break 'mc __v;
        }
        return Err("matchcontinue: no arm matched");
    };
    Ok(outIters)
}

fn isElementNamed(mut inName: metamodelica::Ref<DAE::ComponentRef>, mut inElement: &FunctionVar) -> Result<bool> {
    let mut isNamed: bool;
    let mut name: metamodelica::Ref<DAE::ComponentRef>;
    let __pa0 = ::match_deref::match_deref! { match inElement {
        (Deref @ DAE::Element::VAR { componentRef: __pa0, .. }, _) => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    name = metamodelica::Own::own(__pa0);
    isNamed = ComponentReferenceBasics::crefEqualWithoutSubs(name, inName);
    Ok(isNamed)
}

fn isElementEqual(mut inElement1: &FunctionVar, mut inElement2: &FunctionVar) -> Result<bool> {
    let mut isEqual: bool;
    let mut cr1: metamodelica::Ref<DAE::ComponentRef>;
    let mut cr2: metamodelica::Ref<DAE::ComponentRef>;
    let __pa0 = ::match_deref::match_deref! { match inElement1 {
        (Deref @ DAE::Element::VAR { componentRef: __pa0, .. }, _) => __pa0.clone(),
        _ => return Err("pattern mismatch"),
    } };
    cr1 = metamodelica::Own::own(__pa0);
    let __pa2 = ::match_deref::match_deref! { match inElement2 {
        (Deref @ DAE::Element::VAR { componentRef: __pa2, .. }, _) => __pa2.clone(),
        _ => return Err("pattern mismatch"),
    } };
    cr2 = metamodelica::Own::own(__pa2);
    isEqual = ComponentReferenceBasics::crefEqualWithoutSubs(cr1, cr2);
    Ok(isEqual)
}

fn checkCyclicalComponents(
    mut inCycles: &metamodelica::List<(
        (
            metamodelica::Ref<DAE::Element>,
            Option<metamodelica::Ref<Values::Value>>,
        ),
        metamodelica::List<(
            metamodelica::Ref<DAE::Element>,
            Option<metamodelica::Ref<Values::Value>>,
        )>,
    )>,
    mut inSource: metamodelica::Ref<DAE::ElementSource>,
) -> Result<()> {
    let () = (::match_deref::match_deref! { match inCycles {
        Deref @ metamodelica::ListNode::Nil => {
            ()
        },
        _ => {
            let mut cycles: metamodelica::List<metamodelica::List<(metamodelica::Ref<DAE::Element>, Option<metamodelica::Ref<Values::Value>>)>>;
            let mut elements: metamodelica::List<metamodelica::List<metamodelica::Ref<DAE::Element>>>;
            let mut crefs: metamodelica::List<metamodelica::List<metamodelica::Ref<DAE::ComponentRef>>>;
            let mut names: metamodelica::List<metamodelica::List<ArcStr>>;
            let mut cycles_strs: metamodelica::List<ArcStr>;
            let mut cycles_str: ArcStr;
            let mut scope_str: ArcStr;
            let mut info: SourceInfo;
            cycles = Graph::findCycles(inCycles, &move |__a0: (metamodelica::Ref<DAE::Element>, Option<metamodelica::Ref<Values::Value>>), __a1: (metamodelica::Ref<DAE::Element>, Option<metamodelica::Ref<Values::Value>>)| isElementEqual(&__a0, &__a1))?;
            elements = List::mapList(cycles, &fnptr!(Util::tuple21, _))?;
            crefs = List::mapList(elements, &move |__a0: metamodelica::Ref<DAE::Element>| DAEUtil::varCref(&__a0))?;
            names = List::mapList(crefs, &move |__a0: metamodelica::Ref<DAE::ComponentRef>| ComponentReferenceBasics::printComponentRefStr(&__a0))?;
            cycles_strs = List::map1(names, &fnptr!(stringDelimitList, metamodelica::List<ArcStr>, ArcStr), literal!(","))?;
            cycles_str = stringDelimitList(cycles_strs, literal!("}, {"));
            cycles_str = { let mut __mm_s = String::new(); __mm_s.push_str(&*literal!("{")); __mm_s.push_str(&*cycles_str); __mm_s.push_str(&*literal!("}")); ArcStr::from(__mm_s) };
            scope_str = literal!("");
            info = ElementSource::getElementSourceFileInfo(inSource);
            Error::addSourceMessage(&(Error::CIRCULAR_COMPONENTS.clone()), list![scope_str, cycles_str], &info)?;
            return Err("fail")
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok(())
}

// [EOPT]  Expression optimization functions.
fn optimizeExpTraverser(
    mut inExp: metamodelica::Ref<DAE::Exp>,
    mut inEnv: FCore::Graph,
) -> Result<(metamodelica::Ref<DAE::Exp>, FCore::Graph)> {
    let mut outExp: metamodelica::Ref<DAE::Exp>;
    let mut outEnv: FCore::Graph;
    (outExp, outEnv) = (::match_deref::match_deref! { match &(inExp.clone()) {
        Deref @ DAE::Exp::ASUB { exp: Deref @ DAE::Exp::CREF { componentRef: cref, ty: ety }, sub: subs } => {
            let mut env = inEnv.clone();
            let mut exp: metamodelica::Ref<DAE::Exp>;
            let mut cref = (*cref).clone();
            cref = ComponentReference::subscriptCref(metamodelica::AsArg::as_arg(&cref), subs.clone())?;
            exp = Expression::makeCrefExp(cref.clone(), ety.clone())?;
            (exp, env)
        },
        Deref @ DAE::Exp::TSUB { exp: Deref @ DAE::Exp::TUPLE { PR: Deref @ metamodelica::ListNode::Cons { head: exp, tail: _ } }, ix: 1, .. } => {
            let mut env = inEnv.clone();
            (exp.clone(), env)
        },
        _ => {
            (inExp, inEnv)
        },
        _ => unreachable!("match_deref! exhaustiveness placeholder"),
    } });
    Ok((outExp, outEnv))
}
